use serde::{Deserialize, Serialize};
use crate::{states::PrintState, util::*};

// structure for an item
#[derive(Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub damage: String,
    pub healing: String,
    pub value: String,
    pub amount: u32,
}

// functions for items
impl Item {
    pub fn print(&self) {
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
        if self.amount > 1 {
            println!("amount: {}x", self.amount)
        }
    }

    pub fn edit_name(&mut self) {
        self.name = edit_string("editing name", &self.name)
    }

    pub fn edit_description(&mut self) {
        self.description = edit_string("editing description", &self.description)
    }

    pub fn edit_damage(&mut self) {
        self.damage = edit_string("editing damage", &self.damage)
    }

    pub fn edit_healing(&mut self) {
        self.healing = edit_string("editing healing", &self.healing)
    }

    pub fn edit_value(&mut self) {
        self.value = edit_string("editing value", &self.value)
    }

    pub fn edit_amount(&mut self) {
        self.amount = get_amount();
    }

    pub fn remove(&mut self) {
        self.amount = self.amount - 1;
    }
}

// lists all the items in the inventory
pub fn print_items(inventory: &Vec<Item>) {
    for (i, item) in inventory.iter().enumerate() {
        if item.amount > 1 {
            println!("{}. {} (x{})", (i + 1), item.name, item.amount);
        } else {
            println!("{}. {}", (i + 1), item.name);
        }

        let mut print_state = PrintState::Arrow;

        if !item.damage.is_empty() {
            print_state.format_print();
            print!("{} damage", item.damage);
            print_state = print_state.next();
        }
        if !item.healing.is_empty() {
            print_state.format_print();
            print!("{} healing", item.healing);
            print_state = print_state.next();
        }
        if !item.value.is_empty() {
            print_state.format_print();
            print!("value: {}", item.value);
            print_state = print_state.next();
        }
        match print_state {
            PrintState::Arrow => {}
            PrintState::Comma => {
                print!("\n");
            }
        }
    }
}

// uses the above function to list the items, this function handles the inputs
pub fn view_inv(inventory: &mut Vec<Item>) {
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
            let which = get_which("which item would you like to edit?", inventory.len());
            if which == 0 {
                continue;
            }
            let which_field = get_action(
                "which field would you like to edit?\n1. Name\n2. Description\n3. Damage\n4. Healing\n5. value\n9. amount",
            );
            if which_field == 1 {
                inventory[which].edit_name();
            } else if which_field == 2 {
                inventory[which].edit_description();
            } else if which_field == 3 {
                inventory[which].edit_damage();
            } else if which_field == 4 {
                inventory[which].edit_healing();
            } else if which_field == 5 {
                inventory[which].edit_value();
            } else if which_field == 9 {
                inventory[which].edit_amount();
                if inventory[which].amount == 0 {
                    inventory.remove(which);
                }
            } else {
                continue;
            }
        } else if input == -2 {
            clear();
            print_items(&inventory);
            let which = get_which("which item do you want to remove?", inventory.len());
            inventory[which].remove();
            if inventory[which].amount == 0 {
                inventory.remove(which);
            }
        } else if input as usize > inventory.len() {
            println!("invalid index");
            wait();
            continue;
        } else {
            inventory[(input - 1) as usize].print();
            wait();
        }
    }
}

// adds an item to the inventory
pub fn add_item(inventory: &mut Vec<Item>) {
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
    let amount = init_amount();
    let item = Item {
        name: String::from(name),
        description: String::from(description),
        damage: String::from(damage),
        healing: String::from(healing),
        value: String::from(value),
        amount: amount,
    };
    inventory.push(item);
}
