use std::io;
use std::io::prelude::*;
use std::fs::{self, File, read_to_string};
use dialoguer::Input;
use crate::states::*;
use crate::money::*;
use crate::item::*;
use crate::note::*;
use crate::potion::*;

// function to pause output until user presses enter
// mostly so that i can clear output before printing stuff
pub fn wait() {
    // _ var name just means im not using whatever value
    // get_input returns, doing this makes the compiler not
    // yell at me
    let _ = get_input("press enter to continue...");
}

// function to clear the output without just printing 1000 lines
pub fn clear() {
    clearscreen::clear().expect("failed to clear screen!");
}

// function to get user input, returns a string
pub fn get_input(message: &str) -> String {
    println!("{}", message);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error reading input!");
    input
}
// uses the get_input function, but returns an i32, useful
// to make a selection from a list in one function call
pub fn get_action(message: &str) -> i32 {
    loop {
        let input = get_input(message);
        let _input: i32 = match input.trim().parse() {
            Ok(input) => return input,
            Err(_) => break 0,
        };
    }
}

pub fn get_amount() -> u32 {
    let input = get_action("enter amount");
    if input < 0 {
        return 1 as u32;
    } else {
        return input as u32;
    }
}

pub fn init_amount() -> u32 {
    let amt = get_amount();
    if amt == 0 { 1 } else { amt }
}

pub fn get_which(message: &str, len: usize) -> usize {
    loop {
        let which = get_action(message);
        if which > len as i32 {
            println!("invalid index!");
            continue;
        }
        if which == 0 {
            break 0;
        }
        return (which - 1) as usize;
    }
}

// allows inline editing of a string
// .with_prompt(<some string>) prints <some string>:
// .with_initial_text is the text that can be edited
pub fn edit_string(message: &str, curr_val: &String) -> String {
    let new_val: String = Input::new().with_prompt(message).with_initial_text(curr_val).interact_text().expect("couldnt edit text!");
    return new_val
}

pub fn make_default(purse: &mut Vec<Money>, path: &str) -> File {
    let platinum = Money {
        coin: MoneyType::Platinum,
        amount: 0,
    };
    let gold = Money {
        coin: MoneyType::Gold,
        amount: 0,
    };
    let silver = Money {
        coin: MoneyType::Silver,
        amount: 0,
    };
    let copper = Money {
        coin: MoneyType::Copper,
        amount: 0,
    };
    purse.push(platinum);
    purse.push(gold);
    purse.push(silver);
    purse.push(copper);
    let save_file = make_save(path);
    save_file
}

// reads the save file and populates the vecs
pub fn create_data(
    inventory: &mut Vec<Item>,
    notebook: &mut Vec<Note>,
    purse: &mut Vec<Money>,
    potion_bag: &mut Vec<Potion>,
    file_path: &str,
) {
    let mut state = SaveState::Inventory;
    for line in read_to_string(file_path)
        .expect("could not read file when creating inventory")
        .lines()
    {
        if line == "*exists*" {
            continue;
        }
        if line == "*end*" {
            state = state.next();
            continue;
        }

        match &state {
            SaveState::Inventory => {
                let deserialized =
                    serde_json::from_str(line).expect("inventory could not deserialize");
                inventory.push(deserialized);
            }
            SaveState::Notebook => {
                let deserialized =
                    serde_json::from_str(line).expect("notebook could not deserialize");
                notebook.push(deserialized);
            }
            SaveState::Purse => {
                let deserialized = serde_json::from_str(line).expect("purse could not deserialize");
                purse.push(deserialized);
            }
            SaveState::Potion => {
                let deserialized =
                    serde_json::from_str(line).expect("potion_bag could not deserialize");
                potion_bag.push(deserialized);
            }
            SaveState::None => {}
        }
    }
}

// function that takes all vectors, the current save, and both the temp and final save file paths
// writes each vector to the temp save, the copies the temp save to the final save
// and since fs::copy overwrites, no duplicate data is left
pub fn save_all(
    inventory: &Vec<Item>,
    notebook: &Vec<Note>,
    purse: &Vec<Money>,
    potion_bag: &Vec<Potion>,
    save_file: &mut File,
    path: &str,
    temp: &str,
) {
    let mut state = SaveState::Inventory;
    loop {
        match state {
            SaveState::Inventory => {
                save_file
                    .write(b"*exists*\n")
                    .expect("could not write to inventory save");
                for item in inventory {
                    let serialized =
                        serde_json::to_string(item).expect("could not serialize inventory");
                    save_file
                        .write(serialized.as_bytes())
                        .expect("could not write serialized inventory");
                    save_file
                        .write(b"\n")
                        .expect("could not write newline in inventory");
                }
                save_file
                    .write(b"*end*\n")
                    .expect("could not write end to inventory");
                state = state.next();
            }
            SaveState::Notebook => {
                for note in notebook {
                    let serialized =
                        serde_json::to_string(note).expect("could not serialize notebook");
                    save_file
                        .write(serialized.as_bytes())
                        .expect("could not write notebook");
                    save_file
                        .write(b"\n")
                        .expect("could not write newline in notebook");
                }
                save_file
                    .write(b"*end*\n")
                    .expect("could not write end in notebook");
                state = state.next();
            }
            SaveState::Purse => {
                for coin in purse {
                    let serialized = serde_json::to_string(coin).expect("purse couldnt serialize");
                    save_file
                        .write(serialized.as_bytes())
                        .expect("purse couldnt write");
                    save_file.write(b"\n").expect("purse couldnt write newline");
                }
                save_file
                    .write(b"*end*\n")
                    .expect("purse couldnt write *end*");
                state = state.next();
            }
            SaveState::Potion => {
                for p in potion_bag {
                    let serialized =
                        serde_json::to_string(p).expect("potion bag could not serialize");
                    save_file
                        .write(serialized.as_bytes())
                        .expect("potion bag couldnt write");
                    save_file
                        .write(b"\n")
                        .expect("potion bag couldnt write newline");
                }
                save_file
                    .write(b"*end*\n")
                    .expect("potion_bag couldnt write *end*");
                state = state.next();
            }
            SaveState::None => {
                let _result = fs::copy(temp, path);
                let _rm_result = fs::remove_file(temp);
                break;
            }
        }
    }
}

// creates a file at the specified path
pub fn make_save(path: &str) -> File {
    let save_file = File::create(path).expect("Could not make file!");
    save_file
}
