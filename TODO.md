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

make two seperate "accounts", DM and Player. DM makes items, potions, or any kind of loot. Players can manage what they have been given and edit as need be. The DM can also just have a saved stock of anything they make and when it is transferred or given to players it doesn't remove it from the DM, only copies it. make a way to set the amount.
figure out how to make LAN connection not only work but also create a little share hub where a "DM account" can share items.
