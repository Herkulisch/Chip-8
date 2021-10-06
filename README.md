# An attempt to implement a Chip 8 Emulator in Rust
I used this [Documentation](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.5) to help me understand the architecture of the chip.

---
After just 2 days I had a ~~working~~ emulator, I just had some minor bugs in it.
My goals were to write it in Rust 🦀 and let it run as a terminal user interface application.
I found a test rom, which showed me which instructions were still flawed, so I could fix them more precisely. I used a enum to seperate all Instructions and also implemented the From trait for the enum, so that 16bit value could easily be parsed into a chip8 instruction.

I removed all the crossterm stuff and decided to make this into a library and compile it into Web Assembly and then use a Web framework as the GUI.
Its sad that the TUI Version did not work as intended, but it is still in the master branch and if the crossterm crate gets upgraded I'll try to get a working TUI Version.
