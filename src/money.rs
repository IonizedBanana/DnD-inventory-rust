use serde::{Deserialize, Serialize};
use crate::util::*;

#[derive(Serialize, Deserialize)]
pub struct Money {
   pub coin: MoneyType,
   pub amount: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MoneyType {
    Copper,
    Silver,
    Gold,
    Platinum,
}

// functions for money
impl Money {
    pub fn edit_money(&mut self, amount: i32) {
        println!("current amount: {}", self.amount);
        self.amount += amount;
    }
}

// function to view the purse, also handles input
pub fn view_purse(purse: &mut Vec<Money>) {
    loop {
        clear();
        for (i, coin) in purse.iter().enumerate() {
            println!("{}. {:?}\n-> {}", (i + 1), coin.coin, coin.amount);
        }

        let action = get_action("enter 1 to edit money");
        if action == 0 {
            break;
        } else if action == 1 {
            let which = get_which("which coin type to edit?", purse.len());
            let amount = get_action("enter money amount to add (negative if spending money):");
            purse[which].edit_money(amount);
        } else {
            continue;
        }
    }
}
