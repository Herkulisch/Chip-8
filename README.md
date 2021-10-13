# An attempt to implement a Chip 8 Emulator in Rust
I used this [Documentation](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.5) to help me understand the architecture of the chip.

---
After some hurdles I refactored the binary version into a library and made this the new default branch. This library gives you acces to a Chip Controller, which if created lets you pass it a ROM. This ROM can be executed by letting the chip tick. The Chip reads his input by reading the pressed_key variable. So if you want to tell the chip that a key was pressed you have to assign a value with the Set Pressed Key Function.
