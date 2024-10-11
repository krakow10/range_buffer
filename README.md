Based on https://github.com/AxisAngles/RangeBuffer

```rust
let mut w=Writer::new();

w.write(3*2u128.pow(60),123);
w.write(3*2u128.pow(60),123);

let bytes=w.take();
println!("{:?}",bytes);

let mut r=Reader::new(bytes.as_slice());

assert_eq!(r.read(3*2u128.pow(60)),123);
assert_eq!(r.read(3*2u128.pow(60)),123);
```
