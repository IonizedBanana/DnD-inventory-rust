use crate::util::*;
use serde::{Deserialize, Serialize};

// structure for notes
#[derive(Serialize, Deserialize)]
pub struct Note {
    pub title: String,
    pub body: String,
}

// functions for notes
impl Note {
    pub fn print(&self) {
        clear();
        println!("Note Title:\n{}", self.title);
        if !self.body.is_empty() {
            println!("Note Body:\n{}", self.body);
        }
    }
    pub fn edit_title(&mut self) {
        clear();
        self.title = edit_string("editing title", &self.title)
    }
    pub fn edit_body(&mut self) {
        clear();
        self.body = edit_string("editing body", &self.body)
    }
}

// function to print a list of notes from a notebook
pub fn print_notes(notebook: &Vec<Note>) {
    // .iter() returns an iterator of all the items, .enumerate() returns a value pair
    // in the form (i, x) where i is the index, and x is the value at that index
    for (i, note) in notebook.iter().enumerate() {
        println!("{}. {}", (i + 1), note.title);
    }
}

// function to look at notes. also handles input for manipulating notes
// and viewing the main notes body
pub fn view_notes(notebook: &mut Vec<Note>) {
    loop {
        clear();
        print_notes(&notebook);
        let input = get_action(
            "which note would you like to view? press enter to go back, enter -1 to edit, or enter -2 to remove a note",
        );
        if input == 0 {
            break;
        } else if input == -1 {
            let which = get_which("which note would you like to edit?", notebook.len());
            let title_or_body =
                get_action("which field would you like to edit?\n1. Title\n2. Body");
            let which = which as usize;
            if title_or_body == 1 {
                notebook[which].edit_title();
            } else if title_or_body == 2 {
                notebook[which].edit_body();
            }
        } else if input == -2 {
            clear();
            print_notes(&notebook);
            let which = get_which("which note do you want to remove?", notebook.len());
            notebook.remove(which);
        } else {
            notebook[(input - 1) as usize].print();
            wait();
        }
    }
}

// function to create a note and add it to the notebook
pub fn add_note(notebook: &mut Vec<Note>) {
    clear();
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
