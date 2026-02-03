use godot::classes::{CanvasItem, INode, Label, Node, TextureProgressBar};
use godot::prelude::*;

pub enum GameUIView {
    Loading,
    Countdown,
    Playing,
    Scoring(bool),
    Paused,
}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct GameUIManager {
    view: GameUIView,

    #[export]
    loading_root: OnEditor<Gd<CanvasItem>>,

    #[export]
    countdown_root: OnEditor<Gd<CanvasItem>>,
    #[export]
    countdown_big_3: OnEditor<Gd<CanvasItem>>,
    #[export]
    countdown_big_2: OnEditor<Gd<CanvasItem>>,
    #[export]
    countdown_big_1: OnEditor<Gd<CanvasItem>>,
    #[export]
    countdown_big_go: OnEditor<Gd<CanvasItem>>,

    #[export]
    progress_ring: OnEditor<Gd<TextureProgressBar>>,

    #[export]
    score_root: OnEditor<Gd<CanvasItem>>,

    #[export]
    pause_root: OnEditor<Gd<CanvasItem>>,

    #[export]
    floor_label: OnEditor<Gd<Label>>,

    #[export]
    game_time_label: OnEditor<Gd<Label>>,
    #[export]
    target_time_label: OnEditor<Gd<Label>>,

    #[export]
    score_floor_number_label: OnEditor<Gd<Label>>,
    #[export]
    score_game_time_label: OnEditor<Gd<Label>>,
    #[export]
    score_target_time_label: OnEditor<Gd<Label>>,
    #[export]
    score_time_delta_label: OnEditor<Gd<Label>>,
    #[export]
    score_start_money_label: OnEditor<Gd<Label>>,
    #[export]
    score_money_delta_label: OnEditor<Gd<Label>>,
    #[export]
    score_end_money_label: OnEditor<Gd<Label>>,

    #[export]
    score_home_button: OnEditor<Gd<CanvasItem>>,
    #[export]
    score_exit_button: OnEditor<Gd<CanvasItem>>,
    #[export]
    score_continue_button: OnEditor<Gd<CanvasItem>>,

    base: Base<Node>,
}

impl GameUIManager {
    pub fn open_ui_view(&mut self, view: GameUIView) {
        match view {
            GameUIView::Loading => {
                self.loading_root.set_visible(true);

                self.countdown_root.set_visible(false);

                self.score_root.set_visible(false);

                self.pause_root.set_visible(false);
            }
            GameUIView::Countdown => {
                self.loading_root.set_visible(false);

                self.countdown_root.set_visible(true);

                self.score_root.set_visible(false);

                self.pause_root.set_visible(false);
            }
            GameUIView::Playing => {
                self.loading_root.set_visible(false);

                self.countdown_root.set_visible(true);
                self.countdown_big_3.set_visible(false);
                self.countdown_big_2.set_visible(false);
                self.countdown_big_1.set_visible(false);
                self.countdown_big_go.set_visible(true);

                self.score_root.set_visible(false);

                self.pause_root.set_visible(false);
            }
            GameUIView::Scoring(game_over) => {
                self.loading_root.set_visible(false);

                self.countdown_root.set_visible(false);

                self.score_root.set_visible(true);
                self.score_home_button.set_visible(game_over);
                self.score_exit_button.set_visible(!game_over);
                self.score_continue_button.set_visible(!game_over);

                self.pause_root.set_visible(false);
            }
            GameUIView::Paused => {
                self.loading_root.set_visible(false);

                self.countdown_root.set_visible(false);

                self.score_root.set_visible(false);

                self.pause_root.set_visible(true);
            }
        }

        self.view = view;
    }

    pub fn set_countdown_progress(&mut self, time: f64, duration: f64) {
        self.progress_ring
            .set_value((time / duration).clamp(0.0, 1.0) * 100.0);

        match time {
            x if x <= 1.0 => {
                self.countdown_big_3.set_visible(false);
                self.countdown_big_2.set_visible(false);
                self.countdown_big_1.set_visible(true);
                self.countdown_big_go.set_visible(false);
            }
            x if x <= 2.0 => {
                self.countdown_big_3.set_visible(false);
                self.countdown_big_2.set_visible(true);
                self.countdown_big_1.set_visible(false);
                self.countdown_big_go.set_visible(false);
            }
            x if x <= 3.0 => {
                self.countdown_big_3.set_visible(true);
                self.countdown_big_2.set_visible(false);
                self.countdown_big_1.set_visible(false);
                self.countdown_big_go.set_visible(false);
            }
            _ => {
                self.countdown_big_3.set_visible(false);
                self.countdown_big_2.set_visible(false);
                self.countdown_big_1.set_visible(false);
                self.countdown_big_go.set_visible(false);
            }
        }
    }

    pub fn set_floor(&mut self, floor: i64) {
        self.floor_label.set_text(&format!("floor {floor}"));
        self.score_floor_number_label
            .set_text(&format!("floor {floor}"));
    }

    pub fn set_game_time(&mut self, time: f64, target: f64) {
        self.countdown_root.set_visible(time <= 1.0);

        self.game_time_label
            .set_text(&Self::get_formatted_time(time));
        self.target_time_label
            .set_text(&Self::get_formatted_time(target));
    }

    pub fn set_exploration_progress(&mut self, progress: f64) {
        self.progress_ring
            .set_value(progress.clamp(0.0, 1.0) * 100.0);
    }

    pub fn set_scores(&mut self, time: f64, target: f64, start_money: i64, end_money: i64) {
        let delta_time = target - time;

        self.score_game_time_label
            .set_text(&Self::get_formatted_time(time));
        self.score_target_time_label
            .set_text(&Self::get_formatted_time(target));
        self.score_time_delta_label.set_text(&format!(
            "{}{}",
            if delta_time >= 0.0 { "+" } else { "-" },
            Self::get_formatted_time(delta_time.abs())
        ));

        let delta_money = end_money - start_money;

        self.score_start_money_label
            .set_text(&format!("${start_money}"));
        self.score_money_delta_label.set_text(&format!(
            "{}${}",
            if delta_money >= 0 { "+" } else { "-" },
            delta_money.abs(),
        ));
        self.score_end_money_label.set_text(
            &(if end_money > 0 {
                format!("${end_money}")
            } else {
                "DEFUNDED".to_string()
            }),
        );
    }

    fn get_formatted_time(time: f64) -> String {
        let milliseconds = ((time * 100.0) % 100.0).floor() as i64;
        let seconds = time.floor() as i64;
        let minutes = seconds / 60;
        let hours = minutes / 60;

        format!(
            "{:02}:{:02}:{:02}.{:02}",
            hours,
            minutes % 60,
            seconds % 60,
            milliseconds
        )
    }
}

#[godot_api]
impl INode for GameUIManager {
    fn init(base: Base<Node>) -> Self {
        Self {
            view: GameUIView::Loading,

            loading_root: OnEditor::default(),

            countdown_root: OnEditor::default(),
            countdown_big_3: OnEditor::default(),
            countdown_big_2: OnEditor::default(),
            countdown_big_1: OnEditor::default(),
            countdown_big_go: OnEditor::default(),

            progress_ring: OnEditor::default(),

            score_root: OnEditor::default(),

            pause_root: OnEditor::default(),

            floor_label: OnEditor::default(),

            game_time_label: OnEditor::default(),
            target_time_label: OnEditor::default(),

            score_floor_number_label: OnEditor::default(),
            score_game_time_label: OnEditor::default(),
            score_target_time_label: OnEditor::default(),
            score_time_delta_label: OnEditor::default(),
            score_start_money_label: OnEditor::default(),
            score_money_delta_label: OnEditor::default(),
            score_end_money_label: OnEditor::default(),

            score_home_button: OnEditor::default(),
            score_exit_button: OnEditor::default(),
            score_continue_button: OnEditor::default(),

            base,
        }
    }

    fn ready(&mut self) {
        self.open_ui_view(GameUIView::Loading);
    }
}
