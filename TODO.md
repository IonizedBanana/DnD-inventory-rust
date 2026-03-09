- make a better money management system [DONE]
- add a system for potion effects (see: spell recovery potion) 
- look into possible dnd beyond api (SUPER FAR AWAY)
- add an item counter
- add bounds checking for inventories and stuff [DONE]
- implement a less error prone save loading [DONE]
    - load values, make a temp file to save, copy temp file to save file
    - fs::copy overwrites, so no duplicate data is created


BRAINSTORM:

menu logic
? add an exec function to each struct, where printing the menu for that type happens


saving combination:
have an enum with different states for saving. 
let the enum hold references to each vector. 
when we save, go through each enum state, which will hold a reference to each vector
run the saving logic on it using a match case

more dynamic saving logic:
serialize the next save state at the end of each Vec
update the save state in the loading function with the serialized state
