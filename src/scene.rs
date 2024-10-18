/// enum of various scenes that exist
// not sure if there's a better way to do this...
#[derive(Clone, Debug)]
pub enum EScene {
    Gameplay,
    MainMenu,
}

use crate::context::Context;

pub mod credits;
pub mod gameplay;
pub mod main_menu;
pub mod pause;
pub mod settings;

pub trait Scene {
    fn update(&mut self, ctx: &mut Context);
    fn draw(&mut self, ctx: &mut Context);
}
