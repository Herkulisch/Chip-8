# An attempt to implement a Chip 8 Emulator in Rust
I used this [Documentation](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.5) to help me understand the architecture of the chip.

---

***This version is kinda deprecated right now, I will refactor most of it to make it similar to the lib and wasm version. And I will probably rename this into Terminal-Version or something like that.***





After just 2 days I had a ~~working~~ emulator, I just had some minor bugs in it.
My goals were to write it in Rust 🦀 and let it run as a terminal user interface application.
I found a test rom, which showed me which instructions were still flawed, so I could fix them more precisely. I used a enum to seperate all Instructions and also implemented the From trait for the enum, so that 16bit value could easily be parsed into a chip8 instruction.

The only things I could not get right were not related with the emulator or the instructions itself, but with some of the crates I used
- I couldnt get audio working, although I found the rodio crate, which should help me. But it just could not find any of my audio devices on my machines.
- The controls are not responsive at all, I have some troubles finding out the reason for that, but it has to do with my usage of the crossterm crate.
- ~~A pong game rom just does not collide with the pong player, its just phases through it.~~ This seems fixed now thanks to [@tobiasvl](https://github.com/tobiasvl)
- Some visual glitches on games like ghostEscape
