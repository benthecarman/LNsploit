mod dad;
mod main;
mod node;
mod nodes_list;
mod screen;
mod simulation;
mod welcome;

pub use dad::*;
pub use main::*;
pub use node::*;
pub use nodes_list::*;
pub use screen::*;
pub use simulation::*;
pub use welcome::*;

use crossterm::event::KeyEvent;

#[derive(Debug)]
pub enum AppEvent {
    Tick,
    Input(KeyEvent),
    Back,
    Quit,
}
