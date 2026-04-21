#[derive(Debug, Clone)]
pub struct Sizing {
    pub top_bar_height: u16,
    pub tool_options_height: u16,
    pub left_toolbar_width: u16,
    pub right_sidebar_width: u16,
    pub right_sidebar_top_inset: u16,
    pub right_sidebar_gap: u16,
    pub layers_pad_height: u16,
    pub history_pad_height: u16,
    pub footer_height: u16,
    pub footer_inset_top: u16,
    pub dock_header_height: u16,
    pub dock_toolbar_height: u16,
    pub toolbox_button_size: u16,
    pub layer_row_height: u16,
    pub history_row_height: u16,
    pub palette_lead_width: u16,
    pub palette_cell_size: u16,
    pub zoom_control_width: u16,
}

impl Default for Sizing {
    fn default() -> Self {
        Self {
            top_bar_height: 46,
            tool_options_height: 50,
            left_toolbar_width: 126,
            right_sidebar_width: 262,
            right_sidebar_top_inset: 34,
            right_sidebar_gap: 81,
            layers_pad_height: 220,
            history_pad_height: 219,
            footer_height: 54,
            footer_inset_top: 6,
            dock_header_height: 32,
            dock_toolbar_height: 34,
            toolbox_button_size: 46,
            layer_row_height: 34,
            history_row_height: 24,
            palette_lead_width: 420,
            palette_cell_size: 20,
            zoom_control_width: 96,
        }
    }
}
