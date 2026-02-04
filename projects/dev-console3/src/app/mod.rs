use crate::terminal::TerminalHandler;
use crate::widgets::manager::ComponentManager;
use crate::widgets::{ComponentRegistry, WidgetOutcome, Component};
use crate::views::dashboard::Dashboard;
use crate::views::profiles::Profiles;
use crate::widgets::components::toast::ToastManager;
use crate::widgets::components::tabbed_bar::TabbedBar;
use crate::config::BindingsConfig;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders};
use std::time::Duration;

pub struct App {
    pub terminal: TerminalHandler,
    pub manager: ComponentManager,
    pub main_tab_bar: TabbedBar,
    pub toasts: ToastManager,
    pub should_quit: bool,
    
    // Layout State
    pub last_main_area: Rect,
    pub last_tab_area: Rect,
}

impl App {
    pub fn new() -> Result<Self> {
        let mut manager = ComponentManager::new();
        
        // Initial registration
        manager.register("dashboard", ComponentRegistry::Dashboard(Dashboard::new()));
        manager.register("profiles", ComponentRegistry::Profiles(Profiles::new()));
        manager.focus("dashboard");

        Ok(Self {
            terminal: TerminalHandler::new()?,
            manager,
            main_tab_bar: TabbedBar::from_config("MainContentTabBar")?,
            toasts: ToastManager::new(),
            should_quit: false,
            last_main_area: Rect::default(),
            last_tab_area: Rect::default(),
        })
    }

    pub fn run(&mut self) -> Result<()> {
        while !self.should_quit {
            self.draw()?;
            self.handle_events()?;
            self.update()?;
        }
        self.terminal.exit_raw_mode()?;
        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        self.manager.on_tick()?;
        self.toasts.on_tick()?;
        Ok(())
    }

    fn draw(&mut self) -> Result<()> {
        self.terminal.terminal.draw(|f| {
            let area = f.area();
            self.last_main_area = area;
            
            // 0. Calculate Global Layout
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1), // Title Bar
                    Constraint::Min(0),    // Main View (Tabbed)
                    Constraint::Length(2), // Status Bar
                ])
                .split(area);

            // 1. Render Global Elements
            f.render_widget(crate::widgets::elements::title_bar::TitleBar::new("DEV-CONSOLE V3".to_string()), chunks[0]);
            
            // Determine active tab for bindings
            let active_tab = self.main_tab_bar.get_active_id().unwrap_or_else(|| "dashboard".to_string());
            let mut status_text = "Ready".to_string();
            
            // Build help string from active tab bindings
            let mut help_hints = String::new();
            if let Some(bindings) = self.main_tab_bar.config.tab_bindings.get(&active_tab) {
                for (i, item) in bindings.items.iter().enumerate() {
                    if i > 0 { help_hints.push_str(" | "); }
                    help_hints.push_str(&format!("{} {}", item.key, item.description));
                }
            }
            if help_hints.is_empty() {
                help_hints = "F1 HELP | Q QUIT".to_string();
            } else {
                help_hints = format!("{} | Q QUIT", help_hints);
            }

            f.render_widget(crate::widgets::elements::status_bar::StatusBar::new(status_text, help_hints), chunks[2]);

            // 2. Render Tabbed Main Content
            let main_block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray));
            
            self.last_tab_area = chunks[1];
            let inner_main = self.main_tab_bar.render_integrated(chunks[1], f.buffer_mut(), main_block);

            // 3. Draw focused component in the tab body area
            let active_tab = self.main_tab_bar.get_active_id().unwrap_or_else(|| "dashboard".to_string());
            if let Some(component) = self.manager.get_mut(&active_tab) {
                component.view(f, inner_main);
            }

            // 4. Draw global overlays
            self.toasts.view(f, area);
        })?;
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(16))? {
            let event = event::read()?;
            match event {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        // Try to let the manager handle it first (it checks the focus stack)
                        let outcome = self.manager.handle_key(key)?;
                        
                        if outcome == WidgetOutcome::None {
                            match key.code {
                                KeyCode::Char('q') => {
                                    self.should_quit = true;
                                }
                                _ => {}
                            }
                        } else if let WidgetOutcome::Confirmed(_) = outcome {
                            self.toasts.add("Confirmed".to_string(), crate::widgets::components::toast::ToastLevel::Success);
                        }
                    }
                }
                Event::Mouse(mouse) => {
                    // 1. Pass to tab bar first (Global)
                    if let WidgetOutcome::Confirmed(id) = self.main_tab_bar.handle_mouse(mouse, self.last_tab_area)? {
                         self.main_tab_bar.set_active(&id);
                         self.manager.focus(&id); 
                         self.toasts.add(format!("Switched to {}", id), crate::widgets::components::toast::ToastLevel::Info);
                    } else {
                        // 2. Pass to active component via manager
                        // We need the area where the component is rendered (inner_main)
                        // For simplicity in this demo, we'll re-calculate or use last_tab_area
                        self.manager.handle_mouse(mouse, self.last_tab_area)?;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}