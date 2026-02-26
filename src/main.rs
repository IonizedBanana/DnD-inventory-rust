use serde::{Deserialize, Serialize};
use std::fs::{self, File, read_to_string};
use std::io;
use std::io::prelude::*;

// some enums to represent states
enum SaveState {
    Inventory,
    Notebook,
    Purse,
    Potion,
}

impl SaveState {
    fn next(&self) -> SaveState {
        match *self {
            SaveState::Inventory => SaveState::Notebook,
            SaveState::Notebook => SaveState::Purse,
            SaveState::Purse => SaveState::Potion,
            SaveState::Potion => SaveState::Inventory,
        }
    }
}

enum ProgramState {
    MainMenu,
    InvMenu,
    NoteMenu,
    PurseMenu,
    PotionMenu,
}

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
    coin: MoneyType,
    amount: i32,
}
#[derive(Serialize, Deserialize, Debug)]
enum MoneyType {
    Copper,
    Silver,
    Gold,
    Platium,
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

#[derive(Serialize, Deserialize)]
struct Potion {
    name: String,
    effect: String,
    amount: u32,
}
// TODO: finish this
impl Potion {
    fn edit_amount(&mut self) {
        println!("old amount: {}", self.amount);
        loop {
            let change_amount = get_action("enter amount changed (+n to add, -n to remove)");
            // this is disgusting. I could make amount an i32 but it doesnt need to be
            // so I type cast like 4 times to check for a valid number and assign
            let signed_amt = self.amount as i32;
            if (signed_amt + change_amount) >= 0 {
                self.amount = (signed_amt + change_amount) as u32;
                break;
            } else {
                println!("invalid amount!")
            }
        }
    }
    fn print(&self) {
        clear();
        println!(
            "name:\n{}\neffect:\n{}\namount: {}x",
            self.name, self.effect, self.amount
        );
        wait();
    }
    fn edit_name(&mut self) {
        println!("old name: {}", self.name);
        self.name = get_input("enter a new name:")
    }
    fn edit_effect(&mut self) {
        println!("old effect: {}", self.effect);
        self.effect = get_input("enter a new effect:");
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
    for note in notebook {
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
            println!("{}. {:?}\n-> {}", index, coin.coin, coin.amount);
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

fn add_potion(potion_bag: &mut Vec<Potion>) {
    clear();
    let name = get_input("enter the name of the potion:");
    let name = name.trim();
    let effect = get_input("enter the effect of the potion:");
    let effect = effect.trim();
    // mut so I can type cast, declaring outside the loop
    let mut amount: i32;
    loop {
        amount = get_action("enter the amount of them you have:");
        // making sure amount is a valid number
        if amount > 0 {
            break;
        } else {
            println!("invalid amount");
        }
    }
    let potion = Potion {
        name: String::from(name),
        effect: String::from(effect),
        amount: amount as u32,
    };
    potion_bag.push(potion);
}
fn print_potions(potion_bag: &Vec<Potion>) {
    clear();
    let mut index = 1;
    for p in &*potion_bag {
        println!("{}. {} ({}x)", index, p.name, p.amount);
        index += 1;
    }
}

fn view_potion_bag(potion_bag: &mut Vec<Potion>) {
    loop {
        print_potions(&potion_bag);
        let action = get_action(
            "which potion would you like to view? -1 to edit -2 to remove, -3 to edit amount",
        );
        if action == 0 {
            break;
        } else if action == -1 {
            print_potions(&potion_bag);
            let which = get_action("which potion would you like to edit?");
            let which_field = get_action("which field would you like to edit?\n1. name\n2. effect");
            if which_field == 1 {
                potion_bag[(which as usize) - 1].edit_name();
            } else if which_field == 2 {
                potion_bag[(which as usize) - 1].edit_effect();
            }
        } else if action == -2 {
            let which = get_action("which potion would you like to remove?");
            potion_bag.remove((which as usize) - 1);
        } else if action == -3 {
            let which = get_action("which potion would you like to edit?");
            potion_bag[(which as usize) - 1].edit_amount();
        } else {
            potion_bag[(action as usize) - 1].print();
        }
    }
}

// creates a file at the specified path
fn make_save(path: &str) -> File {
    let save_file = File::create(path).expect("Could not make file!");
    save_file
}

// TODO I want to try to combine these somehow, but I dont want to have to refactor the function
// with every new inventory menu. if I combine them as is, I'd have to fix the required args, every
// reference to the function, and then add the logic. doable, but seems like more work than making
// one new function and adding an extra function call to my saving option. 
// figure this out sometime

// following 3 functions deserialize each vec and write to the save file
fn save_inventory(inventory: &Vec<Item>, save_file: &mut File) {
    save_file
        .write(b"*exists*\n")
        .expect("could not write to inventory save");
    for item in inventory {
        let serialized = serde_json::to_string(item).expect("could not serialize inventory");
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
}

fn save_notebook(notebook: &Vec<Note>, save_file: &mut File) {
    for note in notebook {
        let serialized = serde_json::to_string(note).expect("could not serialize notebook");
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
}

fn save_purse(purse: &Vec<Money>, save_file: &mut File) {
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
}

fn save_potions(potion_bag: &Vec<Potion>, save_file: &mut File) {
    for p in potion_bag {
        let serialized = serde_json::to_string(p).expect("potion bag could not serialize");
        save_file
            .write(serialized.as_bytes())
            .expect("potion bag couldnt write");
        save_file
            .write(b"\n")
            .expect("potion bag couldnt write newline");
    }
}

// reads the save file and populates the vecs
fn create_data(
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
        }
    }
}

// writes the contents of the temp save to the real save path 
// fs::copy overwrites, so no duplicate data can be written
fn write_save(temp: &str, path: &str){
    let _result = fs::copy(temp, path);
    let _rm_result = fs::remove_file(temp);
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

fn make_default(purse: &mut Vec<Money>, path: &str) -> File {
    let platinum = Money {
        coin: MoneyType::Platium,
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

fn main() {
    // two constants so I can change these without refactoring half my code
    const SAVE_FILE_PATH: &str = "DnD_save.json";
    // const OLD_SAVE_PATH: &str = "DnD_save_old.json";
    const TEMP_SAVE_PATH: &str = "temp_save.json";
    const OPTIONS: &str = "please select an option:\n1. view inventory\n2. view notebook\n3. view purse\n4. view potion bag\n5. add item\n6. add note\n7. add potion\n9. save and quit";
    let menu_state = ProgramState::MainMenu;

    // creating the vecs that i need, and initializing the save_file var
    let mut inventory: Vec<Item> = Vec::new();
    let mut notebook: Vec<Note> = Vec::new();
    let mut purse: Vec<Money> = Vec::new();
    let mut potion_bag: Vec<Potion> = Vec::new();
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
        create_data(
            &mut inventory,
            &mut notebook,
            &mut purse,
            &mut potion_bag,
            SAVE_FILE_PATH,
        );
        save_file = make_save(TEMP_SAVE_PATH);
    } else {
        save_file = make_default(&mut purse, SAVE_FILE_PATH);
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
            view_potion_bag(&mut potion_bag);
        } else if action == 5 {
            add_item(&mut inventory);
        } else if action == 6 {
            add_note(&mut notebook);
        } else if action == 7 {
            add_potion(&mut potion_bag);
        } else if action == 9 {
            save_inventory(&inventory, &mut save_file);
            save_notebook(&notebook, &mut save_file);
            save_purse(&purse, &mut save_file);
            save_potions(&potion_bag, &mut save_file);
            write_save(TEMP_SAVE_PATH, SAVE_FILE_PATH);
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
