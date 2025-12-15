use serde::{Deserialize, Serialize};
use std::fs::{self, File, read_to_string};
use std::io;
use std::io::prelude::*;

// #[derive(Serialize, Deserialize)]
#[derive(Serialize, Deserialize)]
struct Item {
    name: String,
    description: String,
    damage: String,
    healing: String,
    value: String,
}

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
        self.name = String::from(get_input("enter a new name:").trim());
    }

    fn edit_description(&mut self) {
        self.description = String::from(get_input("enter a new description:").trim());
    }

    fn edit_damage(&mut self) {
        self.damage = String::from(get_input("enter a new damage number:").trim());
    }

    fn edit_healing(&mut self) {
        self.healing = String::from(get_input("enter a new healing amount:").trim());
    }

    fn edit_value(&mut self) {
        self.value = String::from(get_input("enter a new value:").trim());
    }
}

#[derive(Serialize, Deserialize)]
struct Note {
    title: String,
    body: String,
}

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
        self.title = String::from(get_input("enter a new title:").trim());
    }
    fn edit_body(&mut self) {
        clear();
        self.body = String::from(get_input("enter a new body:").trim());
    }
}

fn wait() {
    let _ = get_input("press enter to continue...");
}

fn clear() {
    clearscreen::clear().expect("failed to clear screen!");
}

fn get_input(message: &str) -> String {
    println!("{}", message);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error reading input!");
    input
}

fn get_action(message: &str) -> i32 {
    loop {
        let input = get_input(message);
        let _input: i32 = match input.trim().parse() {
            Ok(input) => return input,
            Err(_) => break 0,
        };
    }
}

fn print_notes(notebook: &Vec<Note>) {
    let mut index = 1;
    for note in &*notebook {
        println!("{}. {}", index, note.title);
        index += 1;
    }
}

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

fn print_items(inventory: &Vec<Item>) {
    let mut index = 1;
    for item in inventory {
        println!("{}. {}", index, item.name);
        if !item.damage.is_empty() && !item.healing.is_empty() && !item.value.is_empty() {
            println!("-> {} damage, {} healing, value: {}", item.damage, item.healing, item.value);
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

fn make_save() -> File {
    let save_file = File::create("DnD_save.json").expect("Could not make file!");
    save_file
}

fn open_save(path: &str) -> File {
    let save_exists: bool = match fs::exists("DnD_save_old.json") {
        Ok(true) => true,
        Ok(false) => false,
        Err(_) => false,
    };
    if save_exists {
        fs::remove_file("DnD_save_old.json").expect("cant remove ts ah file");
    }
    let old_contents = fs::read_to_string(path).expect("fuck you bitch");
    let mut old_file = File::create("DnD_save_old.json").expect("221 cant create file");
    old_file
        .write(old_contents.as_bytes())
        .expect("222 cabt write ts ah");
    fs::remove_file(path).expect("could not remove file");
    let file = make_save();
    file
}

fn save_inventory(inventory: &mut Vec<Item>, save_file: &mut File) {
    save_file.write(b"*exists*\n").expect("240 couldnt write");
    for item in inventory {
        let serialized = serde_json::to_string(item).expect("221 couldnt write");
        save_file
            .write(serialized.as_bytes())
            .expect("222 couldnt write");
        save_file.write(b"\n").expect("223 couldnt write");
    }
    save_file.write(b"*end*\n").expect("225 couldnt write");
}

fn save_notebook(notebook: &mut Vec<Note>, save_file: &mut File) {
    for note in notebook {
        let serialized = serde_json::to_string(note).expect("230 couldnt write");
        save_file
            .write(serialized.as_bytes())
            .expect("231 couldnt write");
        save_file.write(b"\n").expect("232 couldnt write");
    }
}

fn create_data(inventory: &mut Vec<Item>, notebook: &mut Vec<Note>, file_path: &str) {
    let mut exists = false;
    let mut end = false;
    for line in read_to_string(file_path)
        .expect("could not read file when creating inventory")
        .lines()
    {
        if line == "*exists*" {
            exists = true;
            continue;
        }
        if !exists {
            panic!("save corrupted! move \"DnD_save_old.json\" to \"DnD_save.json\"")
        }

        if line == "*end*" {
            end = true;
            continue;
        }
        if !end {
            let deserialized = serde_json::from_str(line).expect("could not deserialize");
            inventory.push(deserialized);
        } else {
            let deserialized = serde_json::from_str(line).expect("could not deserialize");
            notebook.push(deserialized);
        }
    }
}

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
    const SAVE_FILE_PATH: &str = "DnD_save.json";
    const OPTIONS: &str = "please select an option:\n1. view inventory\n2. view notebook\n3. add item\n4. add note\n5. save and quit";
    let mut inventory: Vec<Item> = Vec::new();
    let mut notebook: Vec<Note> = Vec::new();
    let mut save_file: File;
    let save_exists: bool = match fs::exists(SAVE_FILE_PATH) {
        Ok(true) => true,
        Ok(false) => false,
        Err(_) => false,
    };
    if save_exists {
        let data_safe = verify_data(SAVE_FILE_PATH);
        if !data_safe {
            panic!("save corrupted! move \"DnD_save_old.json\" to \"DnD_save.json\"");
        }
        create_data(&mut inventory, &mut notebook, SAVE_FILE_PATH);
        save_file = open_save(SAVE_FILE_PATH);
    } else {
        save_file = make_save();
    }
    loop {
        clear();
        let action = get_action(OPTIONS);
        if action == 1 {
            list_items(&mut inventory);
        } else if action == 2 {
            view_notes(&mut notebook);
        } else if action == 3 {
            add_item(&mut inventory);
        } else if action == 4 {
            add_note(&mut notebook);
        } else if action == 5 {
            save_inventory(&mut inventory, &mut save_file);
            save_notebook(&mut notebook, &mut save_file);
            println!("writing data..");
            wait();
            break;
        } else {
            continue;
        }
    }
}
