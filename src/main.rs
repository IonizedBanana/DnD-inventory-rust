use std::io;

struct Item {
    name: String,
    description: String,
    damage: String,
    healing: String,
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
    }
}

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

fn get_action(message: &str) -> u32 {
    loop {
        let input = get_input(message);
        let _input: u32 = match input.trim().parse() {
            Ok(input) => return input,
            Err(_) => break 0
        };
    }
}

fn view_notes(notebook: &Vec<Note>) {
    loop {
        clear();
        let mut index = 1;
        for note in notebook {
            println!("{}. {}", index, note.title);
            index += 1;
        }
        let input = get_action("which note would you like to view? press enter to go back");
        if input == 0 {
            break;
        }
        let input = input as usize;
        notebook[input - 1].print();
        wait();
    }
}

fn add_note(notebook: &mut Vec<Note>) {
    let title = get_input("please enter a title for the note:");
    let title = title.trim();
    let body = get_input("please enter the body of the note:");
    let body = body.trim();
    let mut note = Note {
        title: String::from(title),
        body: String::from(body),
    };
    notebook.push(note);
}

fn list_items(inventory: &Vec<Item>) {
    loop {
        clear();
        let mut index = 1;
        for item in inventory {
            println!("{}. {}", index, item.name);
            if !item.damage.is_empty() && !item.healing.is_empty() {
                println!("-> {} damage, {} healing", item.damage, item.healing);
            } else if !item.healing.is_empty() {
                println!("-> {} healing", item.healing);
            } else if !item.damage.is_empty() {
                println!("-> {} damage", item.damage);
            }
            index += 1;
        }
        let input = get_action("which item would you like to view? press enter to go back");
        if input == 0 {
            break;
        }
        let input = input as usize;
        inventory[input - 1].print();
        wait();
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
    let mut item = Item {
        name: String::from(name),
        description: String::from(description),
        damage: String::from(damage),
        healing: String::from(healing),
    };
    inventory.push(item);
}

fn main() {
    const OPTIONS: &str =
        "please select an option:\n1. view inventory\n2. view notebook\n3. add item\n4. add note";
    let mut inventory: Vec<Item> = Vec::new();
    let mut notebook: Vec<Note> = Vec::new();

    loop {
        clear();
        let action = get_action(OPTIONS);
        if action == 1 {
            // item1.print();
            list_items(&inventory);
        } else if action == 2 {
            // note1.print();
            view_notes(&notebook);
        } else if action == 3 {
            add_item(&mut inventory);
        } else if action == 4 {
            add_note(&mut notebook);
        } else {
            continue;
        }
    }
}
