# moon-gas


https://github.com/justdie386/moon-gas/assets/79466202/a753d644-5b41-4aaa-a180-f1ddca5f0215

The code is pretty weird, i'll have to rework on the logic, but its still pretty great, and it works


How to use

Run cargo build --release --features lua51


to build on macos, if you get an error related to cc, add this to ~/.cargo/config, this fixed it for me


[target.x86_64-apple-darwin]

rustflags = [

  "-C", "link-arg=-undefined",
  
  "-C", "link-arg=dynamic_lookup",
  
]

[target.aarch64-apple-darwin]

rustflags = [

  "-C", "link-arg=-undefined",
  
  "-C", "link-arg=dynamic_lookup",
  
]
