pub mod components {
    pub use crate::{
        exercise_editor::UiExerciseEditor,
        tabs::{UiTab, UiTabs},
    };
}

pub mod ui;
pub mod ui_events;
pub(crate) mod exercise_editor;
pub(crate) mod tabs;
pub(crate) mod menu;
pub(crate) mod exercises_table;
pub(crate) mod main_ui_layer;
pub(crate) mod footer;
pub(crate) mod page_exercises;
pub(crate) mod page_account;
