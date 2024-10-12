Based on https://github.com/AxisAngles/RangeBuffer

```rust
let mut data:Vec<u8>=Vec::new();
let mut w=Writer::new(&mut data);

w.write(3*2u128.pow(60),123)?;
w.write(3*2u128.pow(60),123)?;

w.flush()?;

let mut r=Reader::new(std::io::Cursor::new(data));

assert_eq!(r.read(3*2u128.pow(60))?,123);
assert_eq!(r.read(3*2u128.pow(60))?,123);
```
