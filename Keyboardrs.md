# How everything works
## Electrical 
### Matrix
#### Basic Matrix
Most cheap everyday office keyboards use idea called a matrix. A matrix is just essentially just a grid where at every point there lies a key. To find what keys are being pressed, voltage is sent down a column and checks what rows are reviving power. Here is an example of how the scanning would work.  X means its pressed, _ means its not.

| X | _ | 1|
| --- | --- | --- |
| _ | X | 0 |
| 1 | 0 |

- Step 1: power column 0 
- Step 2: check row 0 for power, power
- Step 3: check row 1 for power, no power
- Step 4: power column 1
- Step 5: check row 0 for power, no power
- Step 6: check row 1 for power, power

After that scanning we know that key (0, 0) and (1, 1) is being pressed.
Lets try that again but with 3 keys

| X | X | 1|
| --- | --- | --- |
| _ | X | 0 |
| 1 | 0 |

- Step 1: power column 0 
- Step 2: check row 0 for power, power
- Step 3: check row 1 for power,  power
- Step 4: power column 1
- Step 5: check row 0 for power, power
- Step 6: check row 1 for power, power

Oh no what happened there why is there power on row 0 column 1 when the switch isn't being pressed. this happens because when all 3 keys are pressed power goes down column 1 across row 1 and since (0, 1) is being pressed it connects column 0 to column 1 and since (0, 0) is connected to column 0 and is pressed it connects column 0 to row 0 causing a key press they never happened. this is called ghosting and is solved in the next part.
 
#### Diode Matrix
A diode matrix is essentially the same as a basic matrix but between the key and column or row there is a diode to block electricity one way to stop ghosting. Now lets try that 3 key press example again

| X | X | 1|
| --- | --- | --- |
| _ | X | 0 |
| 1 | 0 |

- Step 1: power column 0 
- Step 2: check row 0 for power, power
- Step 3: check row 1 for power,  power
- Step 4: power column 1
- Step 5: check row 0 for power, no power
- Step 6: check row 1 for power, power

Using this method lets use electrically have unlimited key presses instead of being limited to 2 keys with just a basic matrix.

## Programming
### USB
#### NKRO and Boot protocol
As I said in [Diode Matrix](#Diode-Matrix) with a diode matrix you can have unlimited amount of key presses but you may be thinking why do some keyboards only support 6 keys at once know as 6 key roll over. This is because in USB HID spec there is a protocol that all keyboards should implement, this is know as the boot protocol. The boot protocol exist to that a bios/UEFI doesn't have to implement the full USB HID parser it only needs to support this simple protocol for both mouses and keyboards. A USB HID report is a some bytes that are defined to mean something by a descriptor 

The protocol is defined as follows. 

Byte 0 a bitmap of all the modifier keys(ctrl, shift, gui, alt and the right side equivalents).
Byte 1 is reserved for OEM use.
byte 2 is keycode 1
byte 3 is keycode 2
byte 4 is keycode 3 
byte 5 is keycode 4 
byte 6 is keycode 5
byte 7 is keycode 6 

As you can see the boot protocol only defines 6 keycodes meaning the there may only be 6 keys pressed at once.

A large misconception is that a report for a keyboard has to look like these but that is incorrect a keyboard report can be anything that is legal under the USB HID spec. A keyboard that can support a unlimited number of keys at once is called no key roll over or NKRO. NKRO is implemented by using a bitmap of the same size as the range of scan codes the keyboard wishes to implement, instead of the individual byes per keycode.
### Matrix scanning
Matrix scanning is generally done by utilizing 2 for loops one iterating through a list of output pins and one iterating through a list of input pins. The following is some pseudocode showing how to implement that.
```
int (col, row) = (0, 0)

for loop iterating though output pins {
	pin.high()
	for loop iterating though input pins {
		if pin.is_high() {
			// code to a key to the usb report
		}
		row = row + 1
	}
	pin.low()
	col = col + 1
}
```

Here is my library's implementation 

```rust
for (col, pin) in cols.iter_mut().enumerate() {
	pin.set_high().unwrap();
    for (row, pin) in rows.iter_mut().enumerate() {
        if pin.is_high().unwrap() {
            // on press
            keyboard.key_press(keys[keyboard.layer][row][col], col, row);
        } else {
            // on release
            // logic to check if key was pressed last scan is inside the function.
            keyboard.key_release(keys, col, row);
        }
	}
	pin.set_low().unwrap();
}

```
### Rotary Encoder
#### Lookup table[^1]
##### Explanation 
A fast, accurate, and efficient way to figure out what direction a quadrature is turning is to use a lookup table. By comparing the last values of channel a and b to the current values of channel a and b you can find which direction the encoder turned. For example lets say out previous values were 0, 0 and out new values are 1, 0. As you can see in the image if both channels were low and then only a went to high that must mean the encoder in turning clockwise. 

![Image of Channel a and b](http://3.bp.blogspot.com/-z-JNB_pMrSg/U9w52Ly30bI/AAAAAAAAOvo/33mT_0UdOmA/s1600/quadrature_cw.png)

Doing this with every possible combination gives us a nice table that shows us the direction, what do you know mapping out all possible combinations shows us that old a and b and new a and b represent a 4 bit number. This fact is really nice because we can just use that 4 bit number to index our lookup table to find the direction. 

![](https://3.bp.blogspot.com/-ndRzMPOSBZo/U9w53h6jUTI/AAAAAAAAOv8/Gg892hjSyc8/s1600/quadrature_table.png)
##### Implementation
We are going to use a unsigned 8 bit int to store our values. First I am going to explain some bitwise operations.
- Bitwise AND &, and compares each bit in 2 numbers and if both bits are 1 the results is 1 for that bit, if not 0. Ex 0110 & 1101 = 0100
-  Bitwise OR |, or compares each bit and if either or or both are 1 outputs 1 for that bit otherwise zero. Ex 1000 | 0001 = 10001
-  Shift left <<, this operators shift all bits to the left by the number specified, the new bits are filled in with zeros, Ex 0010 << 1 = 0100, 0010 << 2 = 1000

First lets read our new values, to have channel a value be in the seconded position we need shift over its position by 1 and then do a bitwise or operation on channel a and channel b. If out new values were 1 and 0 this would give us 0000_0010. 

Before we shift our 8 bit number by 2 bits to make room for the new values, we want check if the new new values read were the same as the old ones. This is pretty simple to do all we need to do is use something called a bitwise mask. A bitwise mask is simply using a binary number and a operator to retrieve information, in our case we to just get the first 2 bits of our 8 bit int. 

Now to compare all we need to do is apply with a the following bit mask with and & operator 0000_00011 and compare that the new values to the values. If they are not equal we can show shift our values over by 2 to the left. Now that we did that we can let our values = values | new values. Now that we have our old values and new values in 1 number next each other we can index the lookup table with out values and a bitwise mask. The bitwise mask is needed to cancel out any bits after the 4th, so it would be 0000_1111. The value we get from out lookup table is the direction the encoder is turning.

A note on my library's implementation, there is a value called pulses which is used when your rotary encoder is indented. Each indent sends 4 pulses so to make sure it's only sending a signal once 4 pulses have been sent, it adds the result of the lookup table indexing to pulses and once that reaches either positive 4 meaning or negative 4, changes the direction. Also there are self because the method is contained inside a class/struct. 

Here is a pseudo code and my library's implementation 
```
const LOOKUP_TABLE = [0, -1, 1, 0, 1, 0, 0, -1, -1, 0, 0, 1, 0, 1, -1, 0]
int new_state = channel_a.is_high() << 1 | channel_b.is_high()
if state & 0011(as a binary literal) != new_state {
	state = state << 2
	state = state | new_state
	// the direction the encoder is turning
	LOOKUP_TABLE[state & 1111(as a binary literal)] 
}
```
```rust
let new_state: u8 = (self.channel_a.is_high().unwrap() as u8) << 1 | (self.channel_b.is_high().unwrap() as u8) << 0;
if self.state & 0b0011 != new_state {
    self.state <<= 2;
    self.state |= new_state;

    self.pulses += Self::LOOKUP_TABLE[self.state as usize & 0b1111];
    if self.pulses == 4 {
        self.dir = Dir::Cw;
    } else if self.pulses == -4 {
        self.dir = Dir::Cww;
    } else {
        self.dir = Dir::Same;
    }
		self.pulses %= 4;
} else {
    self.dir = Dir::Same;
}
```

# Resources 
[^1]: https://makeatronics.blogspot.com/2013/02/efficiently-reading-quadrature-with.html.
