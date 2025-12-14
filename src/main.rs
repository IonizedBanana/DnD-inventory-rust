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

fn view_notes(notebook: &mut Vec<Note>) {
    loop {
        clear();
        let mut index = 1;
        for note in & *notebook {
            println!("{}. {}", index, note.title);
            index += 1;
        }
        let input = get_action(
            "which note would you like to view? press enter to go back, or enter -1 to edit",
        );
        if input == 0 {
            break;
        } else if input == -1 {
            let which = get_action("which note would you like to edit?");
            let title_or_body = get_action(
                "would you like to edit the title or body? enter 1 for title, enter 2 for body",
            );
            let which = which as usize;
            if title_or_body == 1 {
                notebook[which - 1].edit_title();
            } else if title_or_body == 2 {
                notebook[which - 1].edit_body();
            } else {
            }
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
    let mut note = Note {
        title: String::from(title),
        body: String::from(body),
    };
    notebook.push(note);
}

fn list_items(inventory: &mut Vec<Item>) {
    loop {
        clear();
        let mut index = 1;
        for item in & *inventory {
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
        let input = get_action(
            "which item would you like to view? press enter to go back, or enter -1 to edit",
        );
        if input == 0 {
            break;
        } else if input == -1 {
            let which = get_action("which note would you like to edit?");
            let which_field = get_action(
                "which field would you like to edit?\n1. Name\n2. Description\n3. Damage\n4. Healing",
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
            }
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
            list_items(&mut inventory);
        } else if action == 2 {
            // note1.print();
            view_notes(&mut notebook);
        } else if action == 3 {
            add_item(&mut inventory);
        } else if action == 4 {
            add_note(&mut notebook);
        } else {
            continue;
        }
    }
}
