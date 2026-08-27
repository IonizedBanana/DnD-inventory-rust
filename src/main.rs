use crate::character::*;
use crate::item::*;
use crate::money::*;
use crate::note::*;
use crate::potion::*;
use crate::spells::*;
use crate::states::*;
use crate::util::*;
use std::fs::{self, File};

pub mod item;
pub mod money;
pub mod note;
pub mod potion;
pub mod states;
pub mod util;

fn main() {
    // constants so I can change these without refactoring half my code
    const SAVE_FILE_PATH: &str = "DnD_save.json";
    const TEMP_SAVE_PATH: &str = "temp_save.json";
    const OPTIONS: &str = "please select an option:\n1. view inventory\n2. view notebook\n3. view purse\n4. view potion bag\n5. add item\n6. add note\n7. add potion\n9. save and quit";

    // creating the vecs that i need, and initializing a couple vars
    let mut inventory: Vec<Item> = Vec::new();
    let mut notebook: Vec<Note> = Vec::new();
    let mut purse: Vec<Money> = Vec::new();
    let mut potion_bag: Vec<Potion> = Vec::new();
    let mut save_file: File;
    let mut menu_state: ProgramState = ProgramState::None;

    // checking if the save exists, and handling logic
    // accordingly
    match fs::exists(SAVE_FILE_PATH) {
        Ok(true) => {
            create_data(
                &mut inventory,
                &mut notebook,
                &mut purse,
                &mut potion_bag,
                SAVE_FILE_PATH,
            );
            save_file = make_save(TEMP_SAVE_PATH);
        }
        Ok(false) => {
            save_file = make_default(&mut purse, SAVE_FILE_PATH);
        }
        Err(_) => {
            panic!("could not check save file!")
        }
    };
    loop {
        // main loop for the program
        clear();
        let action = get_action(OPTIONS);
        // match case for handling user action
        // then offloading menu logic to a different match case
        match action {
            i32::MIN..=0_i32 => continue,
            1 => {
                menu_state = ProgramState::InvMenu;
            }
            2 => {
                menu_state = ProgramState::NoteMenu;
            }
            3 => {
                menu_state = ProgramState::PurseMenu;
            }
            4 => {
                menu_state = ProgramState::PotionMenu;
            }
            5 => {
                add_item(&mut inventory);
            }
            6 => {
                add_note(&mut notebook);
            }
            7 => {
                add_potion(&mut potion_bag);
            }
            8 => continue,
            9 => {
                save_all(
                    &inventory,
                    &notebook,
                    &purse,
                    &potion_bag,
                    &mut save_file,
                    SAVE_FILE_PATH,
                    TEMP_SAVE_PATH,
                );
                // this is just here so its obvious that it worked
                println!("writing data..");
                wait();
                break;
            }
            10..=i32::MAX => continue,
        }
        // handling menu logic
        match menu_state {
            ProgramState::None => {}
            ProgramState::InvMenu => {
                view_inv(&mut inventory);
                menu_state = ProgramState::None;
            }
            ProgramState::NoteMenu => {
                view_notes(&mut notebook);
                menu_state = ProgramState::None;
            }
            ProgramState::PurseMenu => {
                view_purse(&mut purse);
                menu_state = ProgramState::None;
            }
            ProgramState::PotionMenu => {
                view_potion_bag(&mut potion_bag);
                menu_state = ProgramState::None;
            }
        }
    }
}
