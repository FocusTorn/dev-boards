use crate::widgets::{InteractiveWidget, WidgetOutcome};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{List, ListItem, ListState, Widget},
};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
}

#[derive(Debug)]
pub struct FileBrowser {
    pub current_dir: PathBuf,
    pub entries: Vec<FileEntry>,
    pub selected_index: usize,
    pub history: Vec<PathBuf>,
    pub last_visible_height: usize,
}

impl FileBrowser {
    pub fn new(path: PathBuf) -> Self {
        let mut browser = Self {
            current_dir: path,
            entries: Vec::new(),
            selected_index: 0,
            history: Vec::new(),
            last_visible_height: 10, // Default fallback
        };
        browser.load_directory();
        browser
    }

    /// Loads entries from the current directory and sorts them:
    /// 1. '..' parent directory (if exists)
    /// 2. Directories first
    /// 3. Then alphabetically by name
    pub fn load_directory(&mut self) {
        // Only read from disk if the directory actually exists
        // In tests, we might have manually injected entries
        if self.current_dir.exists() && self.entries.is_empty() {
            if let Ok(read_entries) = std::fs::read_dir(&self.current_dir) {
                // Add parent directory entry
                if let Some(parent) = self.current_dir.parent() {
                    self.entries.push(FileEntry {
                        name: "..".to_string(),
                        path: parent.to_path_buf(),
                        is_dir: true,
                    });
                }

                for entry in read_entries.flatten() {
                    let path = entry.path();
                    let name = entry.file_name().to_string_lossy().to_string();
                    let is_dir = path.is_dir();
                    self.entries.push(FileEntry { name, path, is_dir });
                }
            }
        }

        self.entries.sort_by(|a, b| {
            // '..' always comes first
            if a.name == ".." {
                return std::cmp::Ordering::Less;
            }
            if b.name == ".." {
                return std::cmp::Ordering::Greater;
            }

            if a.is_dir != b.is_dir {
                // Directories come before files
                b.is_dir.cmp(&a.is_dir)
            } else {
                // Same type, sort by name case-insensitively
                a.name.to_lowercase().cmp(&b.name.to_lowercase())
            }
        });
    }

    pub fn navigate_parent(&mut self) {
        if let Some(parent) = self.current_dir.parent() {
            let target_path = parent.to_path_buf();
            let old_dir_name = self
                .current_dir
                .file_name()
                .map(|n| n.to_string_lossy().to_string());

            self.current_dir = target_path;
            self.entries.clear();
            self.selected_index = 0;
            self.load_directory();

            // Try to select the directory we just came from
            if let Some(old_name) = old_dir_name {
                if let Some(pos) = self.entries.iter().position(|e| e.name == old_name) {
                    self.selected_index = pos;
                }
            }
        }
    }

    pub fn navigate_back(&mut self) {
        if let Some(prior_dir) = self.history.pop() {
            let old_dir_name = self
                .current_dir
                .file_name()
                .map(|n| n.to_string_lossy().to_string());

            self.current_dir = prior_dir;
            self.entries.clear();
            self.selected_index = 0;
            self.load_directory();

            // Try to select the directory we just came from
            if let Some(old_name) = old_dir_name {
                if let Some(pos) = self.entries.iter().position(|e| e.name == old_name) {
                    self.selected_index = pos;
                }
            }
        }
    }

    pub fn navigate_into(&mut self) {
        if let Some(entry) = self.entries.get(self.selected_index).cloned() {
            if entry.is_dir {
                let target_path = entry.path.clone();
                let old_dir_name = self
                    .current_dir
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string());

                // Push current to history before changing
                self.history.push(self.current_dir.clone());

                self.current_dir = target_path;
                self.entries.clear();
                self.selected_index = 0;
                self.load_directory();

                // If we went UP (via '..'), try to select the directory we just came from
                if entry.name == ".." {
                    if let Some(old_name) = old_dir_name {
                        if let Some(pos) = self.entries.iter().position(|e| e.name == old_name) {
                            self.selected_index = pos;
                        }
                    }
                }
            }
        }
    }

    /// Renders the browser using a mutable reference to track dimensions
    pub fn render_stateful(&mut self, area: Rect, buf: &mut Buffer) {
        self.last_visible_height = area.height as usize;
        (&*self).render(area, buf);
    }
}

impl Widget for FileBrowser {
    fn render(self, area: Rect, buf: &mut Buffer) {
        (&self).render(area, buf);
    }
}

impl Widget for &FileBrowser {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Add single column padding on the left only
        let list_area = Rect {
            x: area.x.saturating_add(1),
            y: area.y,
            width: area.width.saturating_sub(1),
            height: area.height,
        };

        let items: Vec<ListItem> = self
            .entries
            .iter()
            .map(|entry| {
                let icon = if entry.is_dir { "📁 " } else { "📄 " };
                let style = if entry.is_dir {
                    Style::default().fg(Color::Cyan)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(format!("{}{}", icon, entry.name)).style(style)
            })
            .collect();

        let list = List::new(items)
            .highlight_style(
                Style::default()
                    .bg(Color::Indexed(238))
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(""); // Removed ">> "

        let mut state = ListState::default();
        state.select(Some(self.selected_index));

        ratatui::widgets::StatefulWidget::render(list, list_area, buf, &mut state);
    }
}

impl InteractiveWidget for FileBrowser {
    type Outcome = PathBuf;
    fn handle_key(&mut self, key: KeyEvent) -> WidgetOutcome<PathBuf> {
        match key.code {
            KeyCode::Up => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                    WidgetOutcome::Consumed
                } else {
                    WidgetOutcome::None
                }
            }
            KeyCode::Down => {
                if !self.entries.is_empty() && self.selected_index < self.entries.len() - 1 {
                    self.selected_index += 1;
                    WidgetOutcome::Consumed
                } else {
                    WidgetOutcome::None
                }
            }
            KeyCode::PageUp => {
                if self.selected_index > 0 {
                    self.selected_index =
                        self.selected_index.saturating_sub(self.last_visible_height);
                    WidgetOutcome::Consumed
                } else {
                    WidgetOutcome::None
                }
            }
            KeyCode::PageDown => {
                if !self.entries.is_empty() && self.selected_index < self.entries.len() - 1 {
                    self.selected_index = (self.selected_index + self.last_visible_height)
                        .min(self.entries.len() - 1);
                    WidgetOutcome::Consumed
                } else {
                    WidgetOutcome::None
                }
            }
            KeyCode::Right => {
                if let Some(entry) = self.entries.get(self.selected_index).cloned() {
                    if entry.is_dir {
                        self.navigate_into();
                        WidgetOutcome::Changed(self.current_dir.clone())
                    } else {
                        WidgetOutcome::None
                    }
                } else {
                    WidgetOutcome::None
                }
            }
            KeyCode::Left => {
                self.navigate_parent();
                WidgetOutcome::Changed(self.current_dir.clone())
            }
            KeyCode::Enter => {
                if let Some(entry) = self.entries.get(self.selected_index).cloned() {
                    if entry.is_dir {
                        self.navigate_into();
                        WidgetOutcome::Changed(self.current_dir.clone())
                    } else {
                        WidgetOutcome::Confirmed(entry.path)
                    }
                } else {
                    WidgetOutcome::None
                }
            }
            KeyCode::Backspace => {
                self.navigate_back();
                WidgetOutcome::Changed(self.current_dir.clone())
            }
            KeyCode::Esc => WidgetOutcome::Canceled,
            _ => WidgetOutcome::None,
        }
    }

    fn handle_mouse(
        &mut self,
        mouse: crossterm::event::MouseEvent,
        area: Rect,
    ) -> WidgetOutcome<PathBuf> {
        // Account for the same padding as in render (Left only)
        let list_area = Rect {
            x: area.x.saturating_add(1),
            y: area.y,
            width: area.width.saturating_sub(1),
            height: area.height,
        };

        if !list_area.contains(ratatui::layout::Position::new(mouse.column, mouse.row)) {
            return WidgetOutcome::None;
        }

        match mouse.kind {
            crossterm::event::MouseEventKind::Down(crossterm::event::MouseButton::Left) => {
                let relative_y = mouse.row.saturating_sub(list_area.y) as usize;
                if relative_y < self.entries.len() {
                    if self.selected_index == relative_y {
                        // Double click (approx) or confirmed click on selected
                        if let Some(entry) = self.entries.get(self.selected_index).cloned() {
                            if entry.is_dir {
                                self.navigate_into();
                                return WidgetOutcome::Changed(self.current_dir.clone());
                            } else {
                                return WidgetOutcome::Confirmed(entry.path);
                            }
                        }
                    } else {
                        self.selected_index = relative_y;
                        return WidgetOutcome::Consumed;
                    }
                }
            }
            crossterm::event::MouseEventKind::ScrollUp => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                    return WidgetOutcome::Consumed;
                }
            }
            crossterm::event::MouseEventKind::ScrollDown => {
                if !self.entries.is_empty() && self.selected_index < self.entries.len() - 1 {
                    self.selected_index += 1;
                    return WidgetOutcome::Consumed;
                }
            }
            _ => {}
        }
        WidgetOutcome::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    fn make_key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::empty())
    }

    #[test]
    fn test_file_browser_sorting() {
        let mut browser = FileBrowser::new(PathBuf::from("."));

        // Manually inject unsorted entries to test the sorting logic
        browser.entries = vec![
            FileEntry {
                name: "z_file.txt".into(),
                path: "z_file.txt".into(),
                is_dir: false,
            },
            FileEntry {
                name: "..".into(),
                path: "..".into(),
                is_dir: true,
            },
            FileEntry {
                name: "a_dir".into(),
                path: "a_dir".into(),
                is_dir: true,
            },
            FileEntry {
                name: "b_file.txt".into(),
                path: "b_file.txt".into(),
                is_dir: false,
            },
            FileEntry {
                name: "m_dir".into(),
                path: "m_dir".into(),
                is_dir: true,
            },
        ];

        browser.load_directory(); // This should trigger sorting

        assert_eq!(browser.entries[0].name, "..");
        assert_eq!(browser.entries[1].name, "a_dir");
        assert_eq!(browser.entries[2].name, "m_dir");
        assert_eq!(browser.entries[3].name, "b_file.txt");
        assert_eq!(browser.entries[4].name, "z_file.txt");
    }

    #[test]
    fn test_file_browser_navigation() {
        let mut browser = FileBrowser::new(PathBuf::from("."));
        browser.entries = vec![
            FileEntry {
                name: "1".into(),
                path: "1".into(),
                is_dir: false,
            },
            FileEntry {
                name: "2".into(),
                path: "2".into(),
                is_dir: false,
            },
        ];

        // Down
        browser.handle_key(make_key(KeyCode::Down));
        assert_eq!(browser.selected_index, 1);

        // Down (boundary)
        browser.handle_key(make_key(KeyCode::Down));
        assert_eq!(browser.selected_index, 1);

        // Up
        browser.handle_key(make_key(KeyCode::Up));
        assert_eq!(browser.selected_index, 0);

        // Up (boundary)
        browser.handle_key(make_key(KeyCode::Up));
        assert_eq!(browser.selected_index, 0);
    }

    #[test]
    fn test_file_browser_selection_and_cancel() {
        let mut browser = FileBrowser::new(PathBuf::from("."));
        let file_path = PathBuf::from("test.txt");
        browser.entries = vec![FileEntry {
            name: "test.txt".into(),
            path: file_path.clone(),
            is_dir: false,
        }];

        // Confirm
        let outcome = browser.handle_key(make_key(KeyCode::Enter));
        assert_eq!(outcome, WidgetOutcome::Confirmed(file_path));

        // Cancel
        let outcome = browser.handle_key(make_key(KeyCode::Esc));
        assert_eq!(outcome, WidgetOutcome::Canceled);
    }

    #[test]
    fn test_file_browser_arrow_navigation() {
        let mut browser = FileBrowser::new(PathBuf::from("."));
        browser.entries = vec![FileEntry {
            name: "dir1".into(),
            path: "dir1".into(),
            is_dir: true,
        }];
        browser.selected_index = 0;

        // ArrowRight should navigate into
        let outcome = browser.handle_key(make_key(KeyCode::Right));
        assert_eq!(browser.current_dir, PathBuf::from("dir1"));
        assert!(matches!(outcome, WidgetOutcome::Changed(_)));

        // ArrowLeft should navigate parent
        let initial_dir = browser.current_dir.clone();
        browser.handle_key(make_key(KeyCode::Left));
        assert_eq!(browser.current_dir, initial_dir.parent().unwrap());
    }

    #[test]
    fn test_file_browser_history_navigation() {
        let mut browser = FileBrowser::new(PathBuf::from("dir_a"));
        browser.entries = vec![FileEntry {
            name: "dir_b".into(),
            path: PathBuf::from("dir_a/dir_b"),
            is_dir: true,
        }];
        browser.selected_index = 0;

        // 1. Navigate Into
        browser.handle_key(make_key(KeyCode::Right));
        assert_eq!(browser.current_dir, PathBuf::from("dir_a/dir_b"));
        assert_eq!(browser.history.len(), 1);
        assert_eq!(browser.history[0], PathBuf::from("dir_a"));

        // 2. Backspace should go back to prior dir (history)
        browser.handle_key(make_key(KeyCode::Backspace));
        assert_eq!(browser.current_dir, PathBuf::from("dir_a"));
        assert_eq!(browser.history.len(), 0);
    }

    #[test]
    fn test_file_browser_page_navigation() {
        let mut browser = FileBrowser::new(PathBuf::from("."));
        browser.entries = (0..20)
            .map(|i| FileEntry {
                name: i.to_string(),
                path: i.to_string().into(),
                is_dir: false,
            })
            .collect();
        browser.last_visible_height = 5;
        browser.selected_index = 0;

        // 1. Page Down
        browser.handle_key(make_key(KeyCode::PageDown));
        assert_eq!(browser.selected_index, 5);

        // 2. Page Down (Boundary)
        browser.selected_index = 18;
        browser.handle_key(make_key(KeyCode::PageDown));
        assert_eq!(browser.selected_index, 19);

        // 3. Page Up
        browser.selected_index = 10;
        browser.handle_key(make_key(KeyCode::PageUp));
        assert_eq!(browser.selected_index, 5);

        // 4. Page Up (Boundary)
        browser.selected_index = 2;
        browser.handle_key(make_key(KeyCode::PageUp));
        assert_eq!(browser.selected_index, 0);
    }

    #[test]
    fn test_file_browser_comprehensive_navigation() {
        let mut browser = FileBrowser::new(PathBuf::from("root"));
        browser.entries = vec![
            FileEntry {
                name: "dir1".into(),
                path: "root/dir1".into(),
                is_dir: true,
            },
            FileEntry {
                name: "file1".into(),
                path: "root/file1".into(),
                is_dir: false,
            },
        ];
        browser.selected_index = 0;

        // 1. Enter dir1 via ArrowRight
        browser.handle_key(make_key(KeyCode::Right));
        assert_eq!(browser.current_dir, PathBuf::from("root/dir1"));
        assert_eq!(browser.history.last(), Some(&PathBuf::from("root")));

        // 2. Go back via Backspace
        browser.handle_key(make_key(KeyCode::Backspace));
        assert_eq!(browser.current_dir, PathBuf::from("root"));
        assert!(browser.history.is_empty());

        // Re-inject mock entries as navigate_back cleared them (disk mock)
        browser.entries = vec![
            FileEntry {
                name: "dir1".into(),
                path: "root/dir1".into(),
                is_dir: true,
            },
            FileEntry {
                name: "file1".into(),
                path: "root/file1".into(),
                is_dir: false,
            },
        ];

        // 3. Move down via Down arrow
        browser.handle_key(make_key(KeyCode::Down));
        assert_eq!(browser.selected_index, 1);

        // 4. ArrowLeft from root should try to go to parent of root
        // (Assuming root has a parent or is just a relative name)
        browser.handle_key(make_key(KeyCode::Left));
        let expected_parent = PathBuf::from("root")
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or(PathBuf::from(""));
        assert_eq!(browser.current_dir, expected_parent);
    }

    #[test]
    fn test_file_browser_rendering() {
        let mut browser = FileBrowser::new(PathBuf::from("."));
        browser.entries = vec![
            FileEntry {
                name: "a_dir".into(),
                path: "a_dir".into(),
                is_dir: true,
            },
            FileEntry {
                name: "b_file.txt".into(),
                path: "b_file.txt".into(),
                is_dir: false,
            },
        ];

        let area = Rect::new(0, 0, 30, 5);
        let mut buf = Buffer::empty(area);
        (&browser).render(area, &mut buf);

        let content = buffer_content(&buf);
        // The list widget might add symbols or padding for highlighting
        // Based on debug output, it seems to add an extra space after the emoji
        assert!(content.contains("📁  a_dir"));
        assert!(content.contains("📄  b_file.txt"));
    }

    fn buffer_content(buf: &Buffer) -> String {
        let mut content = String::new();
        for y in 0..buf.area.height {
            for x in 0..buf.area.width {
                content.push_str(buf[(x, y)].symbol());
            }
            content.push('\n');
        }
        content
    }
}
