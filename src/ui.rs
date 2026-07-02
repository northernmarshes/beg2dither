use ratatui::Frame;
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::widgets::FrameExt as _;
use ratatui_explorer::FileExplorer;

pub fn ui(f: &mut Frame, fe: &mut FileExplorer) {
    let layout = Layout::horizontal([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)]);
    let chunks = layout.split(f.area());

    f.render_widget_ref(fe.widget(), chunks[0]);
}
