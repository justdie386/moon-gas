# moon-gas

The C-example folder is there to show an example of moon-gas, and how it works, the code from the main.lua will be able to compile it, just by running lua main.lua (assuming you've got the right lua version which is lua5.1)

How to use
Run cargo build --release --features lua51
Copy the file libmoon-gas.so from target/release into the USAGE folder
That should be it, running lua main.lua should do it