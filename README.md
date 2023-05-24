# moon-gas


https://github.com/justdie386/moon-gas/assets/79466202/a753d644-5b41-4aaa-a180-f1ddca5f0215

The code is pretty weird, i'll have to rework on the logic, but its still pretty great, and it works


How to use

Run cargo build --release --features *the lua version you want to use*

Has support for lua version from 5.1 to 5.4 and also for luajit
but i only tested 5.1 and 5.4

to build on macos, if you get an error related to cc, add this to ~/.cargo/config, this fixed it for me

For intel
```
[target.x86_64-apple-darwin]

rustflags = [

  "-C", "link-arg=-undefined",
  
  "-C", "link-arg=dynamic_lookup",
  
]
```
For m1-m2
```
[target.aarch64-apple-darwin]

rustflags = [

  "-C", "link-arg=-undefined",
  
  "-C", "link-arg=dynamic_lookup",
  
]
```