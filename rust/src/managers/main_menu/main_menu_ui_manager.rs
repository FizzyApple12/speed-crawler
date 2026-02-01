use godot::classes::{Button, CanvasItem, INode, Node, SpinBox};
use godot::prelude::*;

pub enum MenuUIView {
    Home,
    GameSetup,
}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct MainMenuUIManager {
    #[export]
    start_menu_root: OnEditor<Gd<CanvasItem>>,
    #[export]
    game_setup_root: OnEditor<Gd<CanvasItem>>,

    #[export]
    resume_button: OnEditor<Gd<Button>>,

    #[export]
    seed_box: OnEditor<Gd<SpinBox>>,

    base: Base<Node>,
}

impl MainMenuUIManager {
    pub fn open_ui_view(&mut self, view: MenuUIView) {
        match view {
            MenuUIView::Home => {
                self.start_menu_root.set_visible(true);

                self.game_setup_root.set_visible(false);
            }
            MenuUIView::GameSetup => {
                self.start_menu_root.set_visible(false);

                self.game_setup_root.set_visible(true);
            }
        }
    }

    pub fn allow_resume(&mut self, can_resume: bool) {
        self.resume_button.set_disabled(!can_resume);
    }

    pub fn get_seed(&mut self) -> i64 {
        self.seed_box.get_value() as i64
    }

    pub fn set_seed(&mut self, new_seed: i64) {
        self.seed_box.set_value(new_seed as f64);
    }
}

#[godot_api]
impl INode for MainMenuUIManager {
    fn init(base: Base<Node>) -> Self {
        Self {
            start_menu_root: OnEditor::default(),
            game_setup_root: OnEditor::default(),

            resume_button: OnEditor::default(),

            seed_box: OnEditor::default(),

            base,
        }
    }

    fn ready(&mut self) {
        self.open_ui_view(MenuUIView::Home);
    }
}
