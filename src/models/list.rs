use ratatui::widgets::ListState;

#[derive(Clone, Debug, Default)]
pub(crate) struct StatefulList<T> {
    pub(crate) state: ListState,
    pub(crate) items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub(crate) fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub(crate) fn down(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub(crate) fn up(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
