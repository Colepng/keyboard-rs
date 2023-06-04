# Keyboard
Keyboard firmware library for rp2040 chips written in rust 

# Todo
- [ ] abstraction for keycodes - in progress
- [ ] layers
    - [x] switch layers
    - [ ] toggle layers
    - [x] momentary layers
- [ ] rotary encoders
    - [x] single rotary encoder support
    - [x] multi layer action support
    - [x] multi rotary encoder support
    - [ ] action on holding down and rotating
- [ ] NKRO
- [ ] macros
- [ ] row2col scanning
- [ ] duplex matrix scanning
- [ ] square / round-robin matrix scanning
- [ ] mouse keys
- [ ] oled display support
- [ ] led support

# Example warnings
both onekey and late-night-engineering need the encoders feature to be enabled when building.
Normally if you would enable it when you are declaring this library as a dependency
