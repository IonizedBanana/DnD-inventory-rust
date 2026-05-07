use crate::util::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Potion {
    pub name: String,
    pub effect: String,
    pub amount: u32,
}

impl Potion {
    pub fn edit_amount(&mut self) {
        println!("old amount: {}", self.amount);
        loop {
            let change_amount = get_action("enter amount changed (+n to add, -n to remove)");
            // casting here so i dont have to do it later and can reuse this
            let signed_amt = self.amount as i32;
            if (signed_amt + change_amount) >= 0 {
                self.amount = (signed_amt + change_amount) as u32;
                break;
            } else {
                println!("invalid amount!")
            }
        }
    }

    pub fn use_potion(&mut self) {
        self.amount = self.amount - 1;
    }

    pub fn print(&self) {
        clear();
        println!(
            "name:\n{}\neffect:\n{}\namount: {}x",
            self.name, self.effect, self.amount
        );
        wait();
    }
    pub fn edit_name(&mut self) {
        self.name = edit_string("editing name", &self.name)
    }
    pub fn edit_effect(&mut self) {
        self.effect = edit_string("editing effect", &self.effect)
    }
}

pub fn add_potion(potion_bag: &mut Vec<Potion>) {
    clear();
    let name = get_input("enter the name of the potion:");
    let name = name.trim();
    let effect = get_input("enter the effect of the potion:");
    let effect = effect.trim();
    let amount = init_amount();
    let potion = Potion {
        name: String::from(name),
        effect: String::from(effect),
        amount: amount,
    };
    potion_bag.push(potion);
}

// prints out all the potions in the potion bag
pub fn print_potions(potion_bag: &Vec<Potion>) {
    clear();
    for (i, p) in potion_bag.iter().enumerate() {
        if p.amount > 1 {
            println!("{}. {} (x{})", (i + 1), p.name, p.amount);
        } else {
            println!("{}. {}", (i + 1), p.name);
        }
    }
}

// uses the above function to print, this one handles input and manipulation
pub fn view_potion_bag(potion_bag: &mut Vec<Potion>) {
    loop {
        print_potions(&potion_bag);
        let input = get_action(
            "which potion would you like to view? -1 to edit -2 to remove, -3 to edit amount, -4 to use a potion",
        );
        if input == 0 {
            break;
        } else if input == -1 {
            print_potions(&potion_bag);
            let which = get_which("which potion would you like to edit?", potion_bag.len());
            let which_field = get_action("which field would you like to edit?\n1. name\n2. effect");
            if which_field == 1 {
                potion_bag[which].edit_name();
            } else if which_field == 2 {
                potion_bag[which].edit_effect();
            }
        } else if input == -2 {
            let which = get_which("which potion would you like to remove?", potion_bag.len());
            potion_bag.remove(which);
        } else if input == -3 {
            let which = get_which("which potion would you like to edit?", potion_bag.len());
            potion_bag[which].edit_amount();
        } else if input == -4 {
            let which = get_which("which potion do you want to use?", potion_bag.len());
            potion_bag[which].use_potion();
            if potion_bag[which].amount <= 0 {
                potion_bag.remove(which);
            }
        } else {
            potion_bag[(input as usize) - 1].print();
        }
    }
}
