use crate::config::Config;
use crate::store;
use crate::store::{Path, Shortcut};
use crate::tableview::{GuiResult, RowifyFn, TableView};
use std::cell::RefCell;

use log::debug;
use ratatui::text::{Line, Span};
use ratatui::{
    style::{Color, Stylize},
    widgets::Row,
    DefaultTerminal,
};

use std::env;
use std::rc::Rc;

// Entry point
enum View {
    History,
    Shortcuts,
}

struct Gui<'a> {
    terminal: DefaultTerminal,
    current_view: View,
    history_view: TableView<'a, store::Path, bool>,
    shortcut_view: TableView<'a, store::Shortcut, bool>,
}

impl<'a> Gui<'a> {
    fn format_history_row_builder(
        store: &'a store::Store,
        config: &'a Config,
        view_state: Rc<RefCell<bool>>,
    ) -> RowifyFn<'a, store::Path> {
        let view_state = view_state.clone();
        Box::new(move |paths| {
            let shortcuts: Vec<Shortcut> = store.list_all_shortcuts().unwrap();
            let date_color = config.colors.date.parse::<Color>().unwrap();
            let path_color = config.colors.path.parse::<Color>().unwrap();
            paths
                .iter()
                .map(|path| {
                    let shortened_line = match *view_state.borrow() {
                        true => Self::shorten_path(config, &shortcuts, path),
                        false => None,
                    };
                    let line = shortened_line.unwrap_or_else(|| Self::reduce_path(path));
                    vec![
                        Line::from(Span::from((config.date_formater)(path.date)).fg(date_color)),
                        Line::from(line).fg(path_color),
                    ]
                })
                .map(Row::new)
                .collect()
        })
    }

    /// Return a Line when the path can accept a substitution by a shortcut
    fn shorten_path(
        config: &Config,
        shortcuts: &Vec<Shortcut>,
        path: &Path,
    ) -> Option<Line<'static>> {
        let mut shortened_line: Option<Line> = None;
        let mut cpath = "";
        let scc = config.colors.shortcut_name.parse::<Color>().unwrap();
        for shortcut in shortcuts {
            let spm = format!("{}/", shortcut.path);
            if (path.path.starts_with(&spm) || path.path == shortcut.path)
                && shortcut.path.len() > cpath.len()
            {
                cpath = shortcut.path.as_str();
                shortened_line = Some(
                    Span::from("[").fg(scc)
                        + Span::from(shortcut.name.clone()).fg(scc)
                        + Span::from("]").fg(scc)
                        + Span::from(String::from(&path.path[(spm.len() - 1)..])),
                );
            }
        }
        shortened_line
    }

    /// Return a Line with possibly a substitution with the HOME shortcut
    fn reduce_path(path: &Path) -> Line {
        let home = env::var("HOME");
        match home {
            Ok(home) => {
                let spm = home.clone() + "/";
                if path.path.starts_with(&(spm)) || path.path == home {
                    Span::from("~").fg(Color::DarkGray) + Span::from(&path.path[(spm.len() - 1)..])
                } else {
                    Line::from(Span::from(path.path.clone()))
                }
            }
            Err(_) => Line::from(Span::from(path.path.clone())),
        }
    }

    fn new(store: &'a store::Store, config: &'a Config) -> Gui<'a> {
        let view_state = Rc::<RefCell<bool>>::new(RefCell::new(true));
        Gui {
            terminal: ratatui::init(),
            current_view: View::History,
            history_view: TableView::new(
                vec!["date".to_string(), "path".to_string()],
                Box::new(|pos, len, text| store.list_paths(pos, len, text)),
                Box::new(Gui::format_history_row_builder(
                    store,
                    config,
                    view_state.clone(),
                )),
                |path| path.path.clone(),
                config,
                view_state.clone(),
            ),
            shortcut_view: TableView::new(
                vec!["shortcut".to_string(), "path".to_string()],
                Box::new(|pos: usize, len: usize, text: &str| store.list_shortcuts(pos, len, text)),
                Box::new(|shortcuts: &Vec<store::Shortcut>| {
                    let scc = config.colors.shortcut_name.parse::<Color>().unwrap();
                    let path_color = config.colors.path.parse::<Color>().unwrap();
                    shortcuts
                        .iter()
                        .map(|shortcut| {
                            Row::new(vec![
                                Line::from(Span::from(shortcut.name.clone()).fg(scc)),
                                Line::from(Span::from(shortcut.path.clone())).fg(path_color),
                            ])
                        })
                        .collect()
                }),
                |shortcut: &store::Shortcut| shortcut.path.clone(),
                config,
                view_state.clone(),
            ),
        }
    }

    fn run(&mut self) -> Option<String> {
        loop {
            let res = match self.current_view {
                View::History => self.history_view.run(&mut self.terminal),
                View::Shortcuts => self.shortcut_view.run(&mut self.terminal),
            };
            match res {
                GuiResult::Quit => {
                    ratatui::restore();
                    return None;
                }
                GuiResult::Print(str) => {
                    ratatui::restore();
                    return Some(str);
                }
                GuiResult::Next => match self.current_view {
                    View::History => self.current_view = View::Shortcuts,
                    View::Shortcuts => self.current_view = View::History,
                },
            }
        }
    }
}

pub(crate) fn gui(store: store::Store, config: Config) -> Option<String> {
    color_eyre::install().unwrap();
    debug!("HistoryView::new()");
    let mut gui = Gui::new(&store, &config);
    gui.run()
}
