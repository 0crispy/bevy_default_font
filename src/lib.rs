use std::cell::RefCell;
use std::sync::Mutex;

use bevy::app::Plugin;
use bevy::prelude::*;

pub enum Mode {
    System,
    Setup,
}
pub struct DefaultFontPlugin<
    A: FnMut(&Res<Assets<Font>>, &Res<AssetServer>) -> Handle<Font> + 'static + Send + Sync,
> {
    font: Mutex<RefCell<Option<A>>>,
    mode: Mode,
}
impl<A: FnMut(&Res<Assets<Font>>, &Res<AssetServer>) -> Handle<Font> + 'static + Send + Sync>
    DefaultFontPlugin<A>
{
    pub fn new(font: A, mode: Mode) -> Self {
        Self {
            font: Mutex::new(RefCell::new(Some(font))),
            mode,
        }
    }

    fn default_fonts(
        mut styles: Query<(&mut Text, Entity)>,
        assets: Res<Assets<Font>>,
        font: Res<DefaultFont<A>>,
        server: Res<AssetServer>,
    ) {
        let mut font = font.font.lock().unwrap();

        for (mut text, _) in styles.iter_mut() {
            for section in &mut text.sections {
                if assets.get(&section.style.font).is_none() {
                    section.style.font = font(&assets, &server);
                };
            }
        }
    }
}

impl<A: FnMut(&Res<Assets<Font>>, &Res<AssetServer>) -> Handle<Font> + 'static + Send + Sync> Plugin
    for DefaultFontPlugin<A>
{
    fn build(&self, app: &mut App) {
        replace_with::replace_with_or_abort(&mut self.font.lock().unwrap(), |font| {
            app.insert_resource(DefaultFont {
                font: Mutex::new(font.replace(None).unwrap()),
            });
            font
        });
        match &self.mode {
            Mode::System => app.add_systems(Update,Self::default_fonts),
            Mode::Setup => app.add_systems(Update,Self::default_fonts),
        };
    }
}

#[derive(Resource)]
pub struct DefaultFont<
    A: FnMut(&Res<Assets<Font>>, &Res<AssetServer>) -> Handle<Font> + 'static + Send + Sync,
> {
    pub font: Mutex<A>,
}
