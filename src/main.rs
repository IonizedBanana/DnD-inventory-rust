use serde::{Deserialize, Serialize};
use std::fs::{self, File, read_to_string};
use std::io;
use std::io::prelude::*;

// structure for an item
#[derive(Serialize, Deserialize)]
struct Item {
    name: String,
    description: String,
    damage: String,
    healing: String,
    value: String,
}

// structure for money (gold, silver, copper)
#[derive(Serialize, Deserialize)]
struct Money {
    coin: String,
    amount: i32,
}

// functions for money
impl Money {
    fn edit_money(&mut self, amount: i32) {
        println!("current amount: {}", self.amount);
        self.amount += amount;
    }
}

// functions for items
impl Item {
    fn print(&self) {
        clear();
        println!("Item Name: {}", self.name);
        if !self.description.is_empty() {
            println!("Item Description: {}", self.description);
        }
        if !self.damage.is_empty() {
            println!("Item Damage: {}", self.damage);
        }
        if !self.healing.is_empty() {
            println!("item Healing: {}", self.healing);
        }
        if !self.value.is_empty() {
            println!("item value: {}", self.value);
        }
    }

    fn edit_name(&mut self) {
        println!("old name: {}", self.name);
        self.name = String::from(get_input("enter a new name:").trim());
    }

    fn edit_description(&mut self) {
        println!("old description: {}", self.description);
        self.description = String::from(get_input("enter a new description:").trim());
    }

    fn edit_damage(&mut self) {
        println!("old damage: {}", self.damage);
        self.damage = String::from(get_input("enter a new damage number:").trim());
    }

    fn edit_healing(&mut self) {
        println!("old healing: {}", self.healing);
        self.healing = String::from(get_input("enter a new healing amount:").trim());
    }

    fn edit_value(&mut self) {
        println!("old value: {}", self.value);
        self.value = String::from(get_input("enter a new value:").trim());
    }
}

// structure for notes
#[derive(Serialize, Deserialize)]
struct Note {
    title: String,
    body: String,
}

// functions for notes
impl Note {
    fn print(&self) {
        clear();
        println!("Note Title:\n{}", self.title);
        if !self.body.is_empty() {
            println!("Note Body:\n{}", self.body);
        }
    }
    fn edit_title(&mut self) {
        clear();
        println!("old title: {}", self.title);
        self.title = String::from(get_input("enter a new title:").trim());
    }
    fn edit_body(&mut self) {
        clear();
        println!("old body: {}", self.body);
        self.body = String::from(get_input("enter a new body:").trim());
    }
}

// function to pause output until user presses enter 
// mostly so that i can clear output before printing stuff
fn wait() {
    // _ var name just means im not using whatever value 
    // get_input returns, doing this makes the compiler not 
    // yell at me
    let _ = get_input("press enter to continue...");
}

// function to clear the output without just printing 1000 lines
fn clear() {
    clearscreen::clear().expect("failed to clear screen!");
}

// function to get user input, returns a string
fn get_input(message: &str) -> String {
    println!("{}", message);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error reading input!");
    input
}

// uses the get_input function, but returns an i32, useful 
// to make a selection from a list in one function call
fn get_action(message: &str) -> i32 {
    loop {
        let input = get_input(message);
        let _input: i32 = match input.trim().parse() {
            Ok(input) => return input,
            Err(_) => break 0,
        };
    }
}

// function to print a list of notes from a notebook
fn print_notes(notebook: &Vec<Note>) {
    // probably a better way to do the loop 
    // so i dont have to manually track the index, 
    // but the pointer is useful, so
    let mut index = 1;
    for note in &*notebook {
        println!("{}. {}", index, note.title);
        index += 1;
    }
}

// function to look at notes. also handles input for manipulating notes 
// and viewing the main notes body
fn view_notes(notebook: &mut Vec<Note>) {
    loop {
        clear();
        print_notes(&notebook);
        let input = get_action(
            "which note would you like to view? press enter to go back, enter -1 to edit, or enter -2 to remove a note",
        );
        if input == 0 {
            break;
        } else if input == -1 {
            let which = get_action("which note would you like to edit?");
            let title_or_body =
                get_action("which field would you like to edit?\n1. Title\n2. Body");
            let which = which as usize;
            if title_or_body == 1 {
                notebook[which - 1].edit_title();
            } else if title_or_body == 2 {
                notebook[which - 1].edit_body();
            }
        } else if input == -2 {
            clear();
            print_notes(&notebook);
            let which = get_action("which note do you want to remove?");
            let which = which as usize;
            notebook.remove(which - 1);
        } else {
            let input = input as usize;
            notebook[input - 1].print();
            wait();
        }
    }
}

// function to create a note and add it to the notebook
fn add_note(notebook: &mut Vec<Note>) {
    let title = get_input("please enter a title for the note:");
    let title = title.trim();
    let body = get_input("please enter the body of the note:");
    let body = body.trim();
    let note = Note {
        title: String::from(title),
        body: String::from(body),
    };
    notebook.push(note);
}

// lists all the items in the inventory
fn print_items(inventory: &Vec<Item>) {
    let mut index = 1;
    for item in inventory {
        println!("{}. {}", index, item.name);
        if !item.damage.is_empty() && !item.healing.is_empty() && !item.value.is_empty() {
            println!(
                "-> {} damage, {} healing, value: {}",
                item.damage, item.healing, item.value
            );
        } else if !item.healing.is_empty() && !item.value.is_empty() {
            println!("-> {} healing, value: {}", item.healing, item.value);
        } else if !item.damage.is_empty() && !item.value.is_empty() {
            println!("-> {} damage, value: {}", item.damage, item.value);
        } else if !item.value.is_empty() {
            println!("-> value: {}", item.value);
        } else if !item.damage.is_empty() {
            println!("-> {} damage", item.damage);
        } else if !item.healing.is_empty() {
            println!("-> {} healing", item.healing);
        }
        index += 1;
    }
}

// uses the above function to list the items, this function handles the inputs 
fn list_items(inventory: &mut Vec<Item>) {
    loop {
        clear();
        print_items(&inventory);
        let input = get_action(
            "which item would you like to view? press enter to go back, enter -1 to edit, or enter -2 to remove an item",
        );
        if input == 0 {
            break;
        } else if input == -1 {
            clear();
            print_items(&inventory);
            let which = get_action("which item would you like to edit?");
            let which_field = get_action(
                "which field would you like to edit?\n1. Name\n2. Description\n3. Damage\n4. Healing\n5. value",
            );
            let which = which as usize;
            if which_field == 1 {
                inventory[which - 1].edit_name();
            } else if which_field == 2 {
                inventory[which - 1].edit_description();
            } else if which_field == 3 {
                inventory[which - 1].edit_damage();
            } else if which_field == 4 {
                inventory[which - 1].edit_healing();
            } else if which_field == 5 {
                inventory[which - 1].edit_value();
            }
        } else if input == -2 {
            clear();
            print_items(&inventory);
            let which = get_action("which item do you want to remove?");
            let which = which as usize;
            inventory.remove(which - 1);
        } else {
            let input = input as usize;
            inventory[input - 1].print();
            wait();
        }
    }
}

// function to view the purse, also handles input
fn view_purse(purse: &mut Vec<Money>) {
    loop {
        let mut index = 1;
        clear();
        for coin in &*purse {
            println!("{}. {}\n-> {}", index, coin.coin, coin.amount);
            index += 1;
        }

        let action = get_action("enter 1 to edit money");
        if action == 0 {
            break;
        } else if action == 1 {
            let which = get_action("which coin type to edit?");
            let which = which as usize;
            let amount = get_action("enter money amount to add (negative if spending money):");
            purse[which - 1].edit_money(amount);
        }
    }
}

fn add_item(inventory: &mut Vec<Item>) {
    clear();
    let name = get_input("enter the name of the item:");
    let name = name.trim();
    let description = get_input("add a description, if necessary");
    let description = description.trim();
    let damage = get_input("enter the damage, if applicable");
    let damage = damage.trim();
    let healing = get_input("enter the healing, if applicable");
    let healing = healing.trim();
    let value = get_input("enter the value, if known");
    let value = value.trim();
    let item = Item {
        name: String::from(name),
        description: String::from(description),
        damage: String::from(damage),
        healing: String::from(healing),
        value: String::from(value),
    };
    inventory.push(item);
}

// creates a file at the specified path 
// I need to change this to take the const I set in the main loop
fn make_save() -> File {
    let save_file = File::create("DnD_save.json").expect("Could not make file!");
    save_file
}

// swaps the new save for the old save, and blanks the new save 
// in order to be ready to save later
fn open_save(path: &str) -> File {
    let save_exists: bool = match fs::exists("DnD_save_old.json") {
        Ok(true) => true,
        Ok(false) => false,
        Err(_) => false,
    };
    if save_exists {
        fs::remove_file("DnD_save_old.json").expect("cant remove file");
    }
    let old_contents = fs::read_to_string(path).expect("could not read file");
    let mut old_file = File::create("DnD_save_old.json").expect("cant create file");
    old_file
        .write(old_contents.as_bytes())
        .expect("could not write file");
    fs::remove_file(path).expect("could not remove file");
    let file = make_save();
    file
}

// following 3 functions deserialize each vec and write to the save file
fn save_inventory(inventory: &Vec<Item>, save_file: &mut File) {
    save_file.write(b"*exists*\n").expect("could not write to inventory save");
    for item in inventory {
        let serialized = serde_json::to_string(item).expect("could not serialize inventory");
        save_file
            .write(serialized.as_bytes())
            .expect("could not write serialized inventory");
        save_file.write(b"\n").expect("could not write newline in inventory");
    }
    save_file.write(b"*end*\n").expect("could not write end to inventory");
}

fn save_notebook(notebook: &Vec<Note>, save_file: &mut File) {
    for note in notebook {
        let serialized = serde_json::to_string(note).expect("could not serialize notebook");
        save_file
            .write(serialized.as_bytes())
            .expect("could not write notebook");
        save_file.write(b"\n").expect("could not write newline in notebook");
    }
    save_file
        .write(b"*end*\n")
        .expect("could not write end in notebook");
}

fn save_purse(purse: &Vec<Money>, save_file: &mut File) {
    for coin in purse {
        let serialized = serde_json::to_string(coin).expect("purse couldnt serialize");
        save_file
            .write(serialized.as_bytes())
            .expect("purse couldnt write");
        save_file.write(b"\n").expect("purse couldnt write newline");
    }
}

// reads the save file and populates the vecs
fn create_data(
    inventory: &mut Vec<Item>,
    notebook: &mut Vec<Note>,
    purse: &mut Vec<Money>,
    file_path: &str,
) {
    let mut end = false;
    let mut end2 = false;
    for line in read_to_string(file_path)
        .expect("could not read file when creating inventory")
        .lines()
    {
        if line == "*exists*" {
            continue;
        }
        if line == "*end*" && end == true {
            end2 = true;
            continue;
        } else if line == "*end*" {
            end = true;
            continue;
        }
        if !end {
            let deserialized = serde_json::from_str(line).expect("inventory could not deserialize");
            inventory.push(deserialized);
        } else if !end2 {
            let deserialized = serde_json::from_str(line).expect("notebook could not deserialize");
            notebook.push(deserialized);
        } else {
            let deserialized = serde_json::from_str(line).expect("purse could not deserialize");
            purse.push(deserialized);
        }
    }
}

// checking if the save file is valid 
// i tried to do this in the main function but it didnt work 
// works here tho :shrug:
fn verify_data(file_path: &str) -> bool {
    for line in read_to_string(file_path)
        .expect("could not read file when creating inventory")
        .lines()
    {
        if line == "*exists*" {
            return true;
        }
    }
    false
}

fn main() {
    // two constants so I can change these without refactoring half my code
    const SAVE_FILE_PATH: &str = "DnD_save.json";
    const OPTIONS: &str = "please select an option:\n1. view inventory\n2. view notebook\n3. view purse\n4. add item\n5. add note\n9. save and quit";
    // creating the vecs that i need, and initializing the save_file var
    let mut inventory: Vec<Item> = Vec::new();
    let mut notebook: Vec<Note> = Vec::new();
    let mut purse: Vec<Money> = Vec::new();
    let mut save_file: File;
    // checking if the save exists
    let save_exists: bool = match fs::exists(SAVE_FILE_PATH) {
        Ok(true) => true,
        Ok(false) => false,
        Err(_) => false,
    };
    if save_exists {
        // making sure the save is valid, panicing if not and loading the save data if so
        let data_safe = verify_data(SAVE_FILE_PATH);
        if !data_safe {
            panic!("save corrupted! move \"DnD_save_old.json\" to \"DnD_save.json\"");
        }
        create_data(&mut inventory, &mut notebook, &mut purse, SAVE_FILE_PATH);
        save_file = open_save(SAVE_FILE_PATH);
    } else {
        // setting default data and creating a blank save if one doesnt exist
        // probably could be in its own function tbh
        let platinum = Money {
            coin: String::from("Platinum"),
            amount: 0,
        };
        let gold = Money {
            coin: String::from("Gold"),
            amount: 0,
        };
        let silver = Money {
            coin: String::from("Silver"),
            amount: 0,
        };
        let copper = Money {
            coin: String::from("Copper"),
            amount: 0,
        };
        purse.push(platinum);
        purse.push(gold);
        purse.push(silver);
        purse.push(copper);
        save_file = make_save();
    }
    loop {
        // main loop for the program
        clear();
        let action = get_action(OPTIONS);
        if action == 1 {
            list_items(&mut inventory);
        } else if action == 2 {
            view_notes(&mut notebook);
        } else if action == 3 {
            view_purse(&mut purse);
        } else if action == 4 {
            add_item(&mut inventory);
        } else if action == 5 {
            add_note(&mut notebook);
        } else if action == 9 {
            save_inventory(&inventory, &mut save_file);
            save_notebook(&notebook, &mut save_file);
            save_purse(&purse, &mut save_file);
            // this is just here so its obvious that it worked
            println!("writing data..");
            wait();
            break;
        } else {
            // if the user inputs literally anything besies the actions listed the loop just
            // continues
            continue;
        }
    }
}
