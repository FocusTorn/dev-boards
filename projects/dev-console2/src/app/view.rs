use ratatui::{
    layout::{Alignment, Rect, Margin, Layout, Constraint, Position},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph, BorderType, Clear},
    Frame,
};
use crate::app::{App, AppLayout, TaskState}; 
use crate::widgets::tab_bar::{TabBarItem, TabBarWidget};
use crate::widgets::command_list::CommandListWidget;
use crate::widgets::progress_bar::ProgressBarWidget;
use crate::widgets::status_box::StatusBoxWidget;
use crate::widgets::output_box::OutputBoxWidget;
use crate::widgets::smooth_scrollbar::{ScrollBar, ScrollLengths};
use crate::widgets::toast::ToastWidget;

/// UI Rendering implementation (The 'View' of application logic).
///>
/// The `view` module handles the pure projection of the `App` state onto the 
/// terminal screen. It is responsible for calculating layout, rendering 
/// widgets, and managing visual transitions like animations and toasts.
///< 
impl App {
    /// Main entry point for the frame-based rendering pass.
    ///>
    /// Recalculates layout if the terminal size has changed, clears the 
    /// screen if the dimensions are too small, and coordinates the 
    /// rendering of all major UI regions.
    ///< 
    pub fn view(&mut self, frame: &mut Frame) {
        if self.view_area != frame.area() { //>
            self.view_area = frame.area();
            self.layout = self.calculate_layout(self.view_area);
            self.check_terminal_size(self.view_area);
            self.sync_autoscroll();
        } //< 

        if self.terminal_too_small { //> 
            self.render_terminal_too_small(frame);
            return;
        } //< 
        
        let layout = self.layout; 
        self.render_title_bar(frame, layout.title);
        self.render_main_content(frame, layout);
        self.render_bindings(frame, layout.bindings);
        self.render_status_bar(frame, layout.status_bar);
        frame.render_widget(ToastWidget::new(&mut self.toast_manager), frame.area());
    }

    /// Renders the centered application title with decorative borders.
    fn render_title_bar(&self, frame: &mut Frame, area: Rect) {
        let title_text = &self.config.application.title;
        let line = if (area.width as usize) <= title_text.len() + 2 { //> 
             Line::from(Span::styled(title_text, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)))
        } else {
            let dash_count = (area.width as usize).saturating_sub(title_text.len() + 2);
            let left = dash_count / 2;
            let right = dash_count - left;
            Line::from(vec![
                Span::styled("═".repeat(left), Style::default().fg(Color::White)),
                Span::styled(format!(" {} ", title_text), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                Span::styled("═".repeat(right), Style::default().fg(Color::White)),
            ])
        }; //< 
        frame.render_widget(Paragraph::new(line).alignment(Alignment::Center), area);
    }

    /// Displays an error message when the terminal window is too small.
    fn render_terminal_too_small(&self, frame: &mut Frame) {
        frame.render_widget(Clear, frame.area());
        let message = format!("Terminal Too Small\nRequired: {}x{}\nCurrent: {}x{}\n\nPress 'q' to quit", self.config.application.min_width, self.config.application.min_height, frame.area().width, frame.area().height);
        frame.render_widget(Paragraph::new(message).alignment(Alignment::Center).style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)), frame.area());
    }

    /// Renders the complex multi-panel center area of the application.
    ///>
    /// Orchestrates the rendering of tab bars, profile information, command 
    /// lists, progress bars, and the output scroll region.
    ///< 
    fn render_main_content(&mut self, frame: &mut Frame, layout: AppLayout) {
        TabBarWidget::render_composite(&self.config, &self.tabs, &["MainContentTabBar"], layout.main, frame.buffer_mut());
        
        // Render Profile Panel
        let profile_block = Block::bordered().title(" Profile ");
        let inner_profile_area = profile_block.inner(layout.profile);
        frame.render_widget(profile_block, layout.profile);
        
        if !self.profile_ids.is_empty() { //> 
            let current_profile = &self.profile_ids[self.selected_profile_index];
            let profile_text = if self.profile_ids.len() > 1 { 
                format!("{} ({} of {})", current_profile, self.selected_profile_index + 1, self.profile_ids.len()) 
            } else { 
                current_profile.to_string() 
            };
            frame.render_widget(Paragraph::new(profile_text).style(Style::default().fg(Color::Cyan)).block(Block::bordered().border_type(BorderType::Rounded).title(" Sketch Profile ").title_style(Style::default().fg(Color::Yellow))), inner_profile_area);
        } else {
            frame.render_widget(Paragraph::new("No profiles found").style(Style::default().fg(Color::DarkGray)).block(Block::bordered().border_type(BorderType::Rounded).title(" Sketch Profile ").title_style(Style::default().fg(Color::Yellow))), inner_profile_area);
        } //< 

        // Command List
        frame.render_widget(
            CommandListWidget::new(&self.commands, self.selected_command_index, self.hovered_command_index)
                .border_style(self.theme.style("commands_border"))
                .title_style(self.theme.style("commands_title"))
                .highlight_style(self.theme.style("commands_highlight")),
            layout.commands
        );

        // Status / Progress Panel
        match &self.task_state { //> 
            TaskState::Running { percentage, visual_percentage, stage, start_time, smoothed_eta, last_updated, .. } => {
                let elapsed_duration = start_time.elapsed();
                let elapsed_str = format!("{:02}:{:02}", elapsed_duration.as_secs() / 60, elapsed_duration.as_secs() % 60);
                
                let eta_str = if *percentage > 5.0 && *percentage < 100.0 { //> 
                    if let Some(smoothed) = smoothed_eta { //> 
                        let time_since_estimate = last_updated.elapsed().as_secs_f64();
                        let live_eta = smoothed - time_since_estimate;
                        let eta_secs = (live_eta.max(0.0)) as u64;
                        format!("{:02}:{:02}", eta_secs / 60, eta_secs % 60) 
                    } else { 
                        "--:--".to_string() 
                    } //< 
                } else if *percentage >= 100.0 { 
                    "00:00".to_string() 
                } else { 
                    "--:--".to_string() 
                }; //< 
                frame.render_widget(
                    ProgressBarWidget::new("Status".to_string(), *visual_percentage, stage.clone())
                        .elapsed(elapsed_str)
                        .eta(eta_str)
                        .border_style(self.theme.style("progress_border"))
                        .title_style(self.theme.style("progress_title")),
                    layout.status
                );
            }
            _ => {
                frame.render_widget(StatusBoxWidget::new(&self.status_text), layout.status);
            }
        } //< 
        
        // Output Panel with Scrolling and Scrollbar
        let display_lines = if self.output_cached_lines.is_empty() {
            vec![Line::from(Span::styled("No output yet.", Style::default().fg(Color::DarkGray)))]
        } else {
            self.output_cached_lines.clone()
        };

        frame.render_widget(
            OutputBoxWidget::new(&display_lines, self.output_scroll, &self.theme)
                .autoscroll(self.output_autoscroll)
                .input(self.input_active, self.input.value(), self.input.visual_cursor()),
            layout.output
        );

        if self.input_active {
            let inner_output_area = Block::bordered().inner(layout.output);
            let [_, input_part] = Layout::vertical([
                Constraint::Min(0),
                Constraint::Length(3),
            ]).areas(inner_output_area);
            let input_inner = Block::bordered().inner(input_part);
            frame.set_cursor_position(Position::new(
                input_inner.x + self.input.visual_cursor() as u16,
                input_inner.y
            ));
        }
        
        // Render Output Auto-Toggle (Static Tab)
        let output_static_tabs = vec![TabBarItem { id: "autoscroll".to_string(), name: "Auto".to_string(), active: self.output_autoscroll }];
        if let Some((widget, horizontal, vertical, off_x, off_y)) = TabBarWidget::from_config(&self.config, &output_static_tabs, "OutputPanelStaticOptions") { //> 
            widget.render_aligned(layout.output, horizontal, vertical, off_x, off_y, frame.buffer_mut()); 
        } //< 
    }

    /// Renders context-sensitive keybinding help text.
    fn render_bindings(&self, frame: &mut Frame, area: Rect) {
        let mut spans = Vec::new();
        
        // 1. Tab-specific Bindings (Cyan)
        if let Some(active_tab) = self.tabs.iter().find(|tab| tab.active) { //> 
            if let Some(tab_bar) = self.config.tab_bars.iter().find(|tb| tb.id == "MainContentTabBar") { //> 
                if let Some(bindings_config) = tab_bar.tab_bindings.get(&active_tab.id) { //> 
                    let separator = &bindings_config.separator;
                    for (i, binding) in bindings_config.items.iter().enumerate() { //> 
                        if i > 0 { spans.push(Span::raw(separator.clone())); }
                        spans.push(Span::styled(binding.key.clone(), Style::default().fg(Color::Cyan)));
                        spans.push(Span::raw(" "));
                        spans.push(Span::styled(binding.description.clone(), Style::default().fg(Color::Indexed(242))));
                    } //< 
                } //< 
            } //< 
        } //< 

        if spans.is_empty() { spans.push(Span::raw("")); } 
        frame.render_widget(Paragraph::new(Line::from(spans)), area);
    }
    
    /// Renders the global status bar with terminal size and global hotkeys.
    fn render_status_bar(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Block::new().borders(ratatui::widgets::Borders::TOP).border_style(Style::default().fg(Color::White)), area);
        
        let text_area = Rect { x: area.x, y: area.y + 1, width: area.width, height: 1 };
        if text_area.height > 0 && text_area.width > 0 { //> 
            let val = if self.config.application.status_bar.default_text.is_empty() { "Status: Ready".to_string() } else { self.config.application.status_bar.default_text.clone() };
            frame.render_widget(Paragraph::new(Line::from(vec![Span::styled(format!("{} ", val), Style::default().fg(Color::White))])), text_area);
        } //< 

        if self.config.application.show_terminal_size { //> 
            let total = frame.area();
            let mut size_text = format!("[{}, {}]", total.width, total.height);
            if self.config.application.show_press_and_modifier && !self.last_raw_input.is_empty() { //> 
                size_text = format!("{}  {}", size_text, self.last_raw_input);
            } //< 
            let size_area = Rect { 
                x: area.x + (area.width.saturating_sub(size_text.len() as u16)) / 2, 
                y: text_area.y, 
                width: size_text.len() as u16, 
                height: 1 
            };
            frame.render_widget(Paragraph::new(size_text).style(Style::default().fg(Color::DarkGray)), size_area);
        } //< 

        // Render Global Bindings in Status Bar
        if !self.config.application.bindings.items.is_empty() && text_area.height > 0 { //> 
            let mut spans = Vec::new();
            let separator = &self.config.application.bindings.separator;
            for (i, binding) in self.config.application.bindings.items.iter().enumerate() { //> 
                if i > 0 { spans.push(Span::raw(separator.clone())); }
                spans.push(Span::styled(binding.key.clone(), Style::default().fg(Color::Cyan)));
                spans.push(Span::raw(" "));
                spans.push(Span::styled(binding.description.clone(), Style::default().fg(Color::Indexed(242))));
            } //< 
            frame.render_widget(Paragraph::new(Line::from(spans)).alignment(Alignment::Right), text_area);
        } //< 
    }
}