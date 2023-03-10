# RustyForth
Forth written in python inspired from tsoding's [porth](https://gitlab.com/tsoding/porth).

### Keywords Implemented:

- [x] not an operator but a number (Push)
- [x] + (Plus)
- [x] - (Minus)
- [x] = (Equal)
- [x] . (Dump)
- [ ] > (GreaterThan)
- [ ] if
- [ ] end
- [ ] else
- [ ] dup
- [ ] while
- [ ] do

- Others might also be added but for now the scope is limited to this

### Running RustyForth

Same as porth, Forth has two mode. A simulation mode and a compilation mode. Simulation mode
simulates program and shows its output. It is used to easily implement new feature and once 
it becomes a solid feature, then make the same for compilation mode.

Note: extension of input file must be .rf

-   Method I
    -   Simulation Mode
            
            cargo run sim {{ input_file.rf }}

    -   Compilation Mode
            
            cargo run com {{ input_file.rf }}

            ./input_file

-   Method II
    -   Build a release binary of compiler:

            cargo build --release

        the binary will be generated in "RustyForth/target/release/rustyforth" where RustyForth is repo's root directory.

    -   Simulation Mode
            
            rustyforth sim {{ input_file.rf }}

    -   Compilation Mode
            
            rustyforth com {{ input_file.rf }}

            ./input_file

The main aim for project was to learn rust and its mysterious ways. The match statements, powerful enums, etc.
