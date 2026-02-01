use crate::types::save_game::SaveGame;
use godot::classes::file_access::ModeFlags;
use godot::classes::{FileAccess, INode, Node};
use godot::prelude::*;
use std::sync::{Arc, Mutex};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SaveFileOpenError {
    #[error("failed to open save file: {0:?}")]
    FileOpenFailed(godot::global::Error),

    #[error("failed to lock save file")]
    FileLockFailed,
}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct SaveManager {
    save_file: Arc<Mutex<Option<Gd<FileAccess>>>>,
    pub save_game: Option<SaveGame>,

    base: Base<Node>,
}

impl SaveManager {
    pub fn update_save_game(&mut self, new_save_game: Option<SaveGame>) {
        self.save_game = new_save_game.clone();

        self.write_save_game(new_save_game);
    }

    fn read_save_game(&mut self) -> Option<SaveGame> {
        godot_print!("reading save game");

        match self.save_file.lock() {
            Ok(mut file_lock) => {
                if file_lock.is_none() {
                    match Self::init_save_file() {
                        Ok(save_file) => *file_lock = Some(save_file),
                        Err(err) => {
                            godot_print!("failed to create save file for read: {err}");

                            return None;
                        }
                    }
                }

                match *file_lock {
                    Some(ref mut file) => {
                        match serde_json::from_str::<SaveGame>(&file.get_as_text().to_string()) {
                            Ok(save_game) => Some(save_game),
                            Err(err) => {
                                godot_print!("failed to parse save file: {err}");

                                None
                            }
                        }
                    }
                    None => {
                        godot_print!("failed to open save file for read");

                        None
                    }
                }
            }
            Err(err) => {
                godot_print!("failed to lock save file for read: {err}");

                None
            }
        }
    }

    fn write_save_game(&mut self, save_game: Option<SaveGame>) {
        godot_print!("writing save game");

        match self.save_file.lock() {
            Ok(mut file_lock) => {
                if file_lock.is_none() {
                    match Self::init_save_file() {
                        Ok(save_file) => *file_lock = Some(save_file),
                        Err(err) => {
                            godot_print!("failed to create save file for write: {err}");

                            return;
                        }
                    }
                }

                match *file_lock {
                    Some(ref mut file) => {
                        file.seek(0);
                        let _ = file.resize(0);
                        file.store_string(&serde_json::to_string(&save_game).unwrap());
                        file.flush();
                    }
                    None => {
                        godot_print!("failed to open save file for write");
                    }
                }
            }
            Err(err) => {
                godot_print!("failed to lock save file for write: {err}");
            }
        };
    }

    fn init_save_file() -> Result<Gd<FileAccess>, SaveFileOpenError> {
        let save_exists = FileAccess::file_exists("user://game.save");
        match FileAccess::open(
            "user://game.save",
            if save_exists {
                ModeFlags::READ_WRITE
            } else {
                ModeFlags::WRITE_READ
            },
        ) {
            Some(save_file) => {
                godot_print!("save file handle opened");

                Ok(save_file)
            }
            None => {
                godot_error!("failed to open save file handle");

                Err(SaveFileOpenError::FileOpenFailed(
                    FileAccess::get_open_error(),
                ))
            }
        }
    }
}

#[godot_api]
impl INode for SaveManager {
    fn init(base: Base<Node>) -> Self {
        let mut save_manager = Self {
            #[allow(clippy::arc_with_non_send_sync)]
            save_file: Arc::new(Mutex::new(None)),
            save_game: None,

            base,
        };

        save_manager.save_game = save_manager.read_save_game();

        save_manager
    }
}

impl Drop for SaveManager {
    fn drop(&mut self) {
        self.write_save_game(self.save_game.clone());

        match self.save_file.lock() {
            Ok(mut file_lock) => match *file_lock {
                Some(ref mut file) => {
                    file.close();

                    *file_lock = None;

                    godot_print!("closed save file");
                }
                None => {
                    godot_print!("no save file to close");
                }
            },
            Err(err) => {
                godot_print!("failed to lock save file for close: {err}");
            }
        };
    }
}
