//! `lumen log`: browse commits grouped by day and open one in the diff viewer.

use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::{Frame, Terminal};

use super::diff::{self, DiffOptions};
use crate::vcs::{LogEntry, VcsBackend};

type Tui = Terminal<CrosstermBackend<io::Stdout>>;
/// Left and right-aligned halves of one screen row.
type Row = (Line<'static>, Line<'static>);

const MONTHS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

pub fn run(
    entries: Vec<LogEntry>,
    backend: &dyn VcsBackend,
    theme_name: Option<String>,
    wrap: bool,
) -> io::Result<()> {
    if entries.is_empty() {
        return Ok(());
    }
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    enter_tui(&mut terminal)?;
    let result = event_loop(&mut terminal, &entries, backend, theme_name, wrap);
    execute!(io::stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    result
}

fn enter_tui(terminal: &mut Tui) -> io::Result<()> {
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    terminal.clear()
}

fn event_loop(
    terminal: &mut Tui,
    entries: &[LogEntry],
    backend: &dyn VcsBackend,
    theme_name: Option<String>,
    wrap: bool,
) -> io::Result<()> {
    let last = entries.len() - 1;
    let (mut selected, mut scroll, mut status) = (0, 0, String::new());
    loop {
        terminal.draw(|frame| draw(frame, entries, selected, &mut scroll, &status))?;
        let Event::Key(key) = event::read()? else {
            continue;
        };
        if key.kind != KeyEventKind::Press {
            continue;
        }
        status.clear();
        let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
        match (key.code, ctrl) {
            (KeyCode::Char('q') | KeyCode::Esc, _) | (KeyCode::Char('c'), true) => return Ok(()),
            (KeyCode::Char('j') | KeyCode::Down, _) => selected = (selected + 1).min(last),
            (KeyCode::Char('k') | KeyCode::Up, _) => selected = selected.saturating_sub(1),
            (KeyCode::Char('d'), true) | (KeyCode::PageDown, _) => {
                selected = (selected + 10).min(last)
            }
            (KeyCode::Char('u'), true) | (KeyCode::PageUp, _) => {
                selected = selected.saturating_sub(10)
            }
            (KeyCode::Char('g') | KeyCode::Home, _) => selected = 0,
            (KeyCode::Char('G') | KeyCode::End, _) => selected = last,
            (KeyCode::Char('y'), _) => {
                let entry = &entries[selected];
                status = match arboard::Clipboard::new()
                    .and_then(|mut c| c.set_text(&entry.commit_id))
                {
                    Ok(()) => format!("copied {}", entry.short_id),
                    Err(e) => format!("copy failed: {}", e),
                };
            }
            (KeyCode::Enter, _) => {
                let commit = backend
                    .get_commit(&entries[selected].commit_id)
                    .map_err(io::Error::other)?;
                let options = DiffOptions {
                    reference: None,
                    pr: None,
                    detect_pr: false,
                    file: None,
                    watch: false,
                    theme: theme_name.clone(),
                    stacked: false,
                    focus: None,
                    origin: None,
                    wrap,
                };
                // The viewer assumes a blank screen, so wipe the list first.
                terminal.clear()?;
                diff::run_show_ui(options, commit, backend)?;
                // The viewer leaves raw mode and the alternate screen when it quits.
                enter_tui(terminal)?;
            }
            _ => {}
        }
    }
}

fn draw(
    frame: &mut Frame,
    entries: &[LogEntry],
    selected: usize,
    scroll: &mut usize,
    status: &str,
) {
    let area = frame.area();
    let [list, footer] = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(area);

    // ponytail: rebuilds every row per frame; fine for the default 1000 commits,
    // build only the visible window if huge -n values get slow.
    let (rows, top, sel) = build_rows(entries, selected);
    // Keep the selected commit and its day heading on screen.
    *scroll = (*scroll)
        .min(top)
        .max((sel + 2).saturating_sub(list.height as usize));

    let visible = rows.iter().enumerate().skip(*scroll);
    for (y, (i, (left, right))) in (list.y..list.bottom()).zip(visible) {
        let row = Rect {
            y,
            height: 1,
            ..list
        };
        let left_width = row.width.saturating_sub(right.width() as u16);
        frame.render_widget(
            left,
            Rect {
                width: left_width,
                ..row
            },
        );
        frame.render_widget(right, row);
        if i == sel || i == sel + 1 {
            // Reverse video reads well on any terminal palette.
            let bar = Style::new()
                .fg(Color::Reset)
                .add_modifier(Modifier::REVERSED)
                .remove_modifier(Modifier::DIM);
            frame.buffer_mut().set_style(row, bar);
        }
    }

    let hints = Line::from(vec![
        Span::styled(" enter open · y copy sha · q quit  ", dim()),
        Span::styled(status.to_string(), Style::new().fg(Color::Green)),
    ]);
    let position = Line::from(Span::styled(
        format!("{}/{} ", selected + 1, entries.len()),
        dim(),
    ));
    frame.render_widget(hints, footer);
    frame.render_widget(position.right_aligned(), footer);
}

// Plain ANSI styles so the list follows the terminal's own color scheme.
fn dim() -> Style {
    Style::new().add_modifier(Modifier::DIM)
}

/// Rows for the whole list, plus the index of the selected commit's block top
/// (its day heading, when it starts a day) and of its first row.
fn build_rows(entries: &[LogEntry], selected: usize) -> (Vec<Row>, usize, usize) {
    let (mut rows, mut top, mut sel) = (Vec::new(), 0, 0);
    for (i, entry) in entries.iter().enumerate() {
        let day = &entry.date[..10];
        let new_day = i == 0 || day != &entries[i - 1].date[..10];
        if new_day && i > 0 {
            rows.push(Row::default());
        }
        let block_top = rows.len();
        if new_day {
            let heading = Line::styled(day_heading(day), Style::new().add_modifier(Modifier::BOLD));
            rows.push((heading, Line::default()));
        }
        if i == selected {
            (top, sel) = (block_top, rows.len());
        }

        let mut subject = vec![Span::raw("  ")];
        if !entry.refs.is_empty() {
            let refs = format!("({}) ", entry.refs.join(", "));
            subject.push(Span::styled(refs, Style::new().fg(Color::Cyan)));
        }
        subject.push(Span::styled(
            entry.summary.clone(),
            Style::new().add_modifier(Modifier::BOLD),
        ));
        // A span, not Line::styled: a line's style would paint the whole row.
        let short_id = Line::from(Span::styled(
            format!("  {} ", entry.short_id),
            Style::new().fg(Color::Yellow),
        ));
        rows.push((Line::from(subject), short_id.right_aligned()));

        let byline = Line::from(vec![
            Span::raw("  "),
            Span::raw(entry.author.clone()),
            Span::styled(format!(" · {}", entry.relative_time), dim()),
        ]);
        rows.push((byline, Line::default()));
    }
    (rows, top, sel)
}

/// "2026-09-01" -> " Commits on Sep 1, 2026"
fn day_heading(day: &str) -> String {
    let month: usize = day[5..7].parse().unwrap_or(1);
    let dom = day[8..10].trim_start_matches('0');
    format!(" Commits on {} {}, {}", MONTHS[month - 1], dom, &day[..4])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_rows_groups_by_day() {
        let entry = |id: &str, date: &str| LogEntry {
            commit_id: id.into(),
            short_id: id.into(),
            summary: "subject".into(),
            author: "someone".into(),
            date: date.into(),
            relative_time: "now".into(),
            refs: vec![],
        };
        let entries = [
            entry("a", "2026-09-30 10:00:00"),
            entry("b", "2026-09-30 09:00:00"),
            entry("c", "2026-09-01 08:00:00"),
        ];
        // heading, a, a, b, b, blank, heading, c, c
        let (rows, top, sel) = build_rows(&entries, 2);
        assert_eq!(rows.len(), 9);
        assert_eq!((top, sel), (6, 7));
        assert_eq!(rows[6].0.to_string(), " Commits on Sep 1, 2026");
        let (_, top, sel) = build_rows(&entries, 1);
        assert_eq!((top, sel), (3, 3));
    }
}
