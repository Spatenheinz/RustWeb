use std::rc::Rc;

use yew::Reducible;

use crate::models::FoodClubEntry;

pub enum FoodEntryAction {
    Add(FoodClubEntry),
    Delete(String),
}

pub struct FoodEntryState {
    pub tasks: Vec<FoodClubEntry>,
}

impl Default for FoodEntryState {
    fn default() -> Self {
        Self { tasks: vec![] }
    }
}

impl Reducible for FoodEntryState {
    type Action = FoodEntryAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_tasks = match action {
            FoodEntryAction::Add(task) => {
                let mut tasks = self.tasks.clone();
                tasks.push(task);
                tasks
            }
            FoodEntryAction::Delete(id) => {
                let mut tasks = self.tasks.clone();
                // tasks.retain(|task| task.id != id);
                tasks
            }
        };

        Self { tasks: next_tasks }.into()
    }
}
