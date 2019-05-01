#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum SelectInfo {
    None,
    Player,
}

impl Default for SelectInfo {
    fn default() -> Self {
        SelectInfo::None
    }
}

#[derive(Debug, Default)]
pub struct Selection {
    pub select_info: Option<SelectInfo>,
    pub just_selected: bool,
}

impl Selection {
    pub fn select(&mut self, info: SelectInfo) {
        self.select_info = Some(info);
        self.just_selected = true;
    }

    pub fn unselect(&mut self) {
        self.select_info = None;
    }
}
