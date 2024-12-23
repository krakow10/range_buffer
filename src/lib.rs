use std::io::{Read,Write,Result};

struct State{
	c:u128,
	v:u64,
}
impl State{
	const CAP:u128=1<<64;
}

pub struct Reader<R:Read>{
	source:R,
	state:State,
}
pub struct Writer<W:Write>{
	sink:W,
	state:State,
}

impl<R:Read> Reader<R>{
	pub fn new(source:R)->Self{
		Self{
			source,
			state:
			State{
				c:State::CAP,
				v:0,
			},
		}
	}
	pub fn read(&mut self,mut n:u128)->Result<u64>{
		debug_assert!(n<=State::CAP);
		let mut v=0_u64;
		let mut c=1_u128;
		while n*self.state.c>=State::CAP{
			let f0=State::CAP/(self.state.c as u128);
			let f1=n.div_ceil(f0);
			v+=c as u64*self.state.v;
			c*=f0;
			n=f1;

			let mut bytes=[0;8];
			self.source.read_exact(&mut bytes)?;
			self.state.v=u64::from_le_bytes(bytes);
			self.state.c=1;
		}
		v+=c as u64*(self.state.v%n as u64);
		self.state.v/=n as u64;
		self.state.c*=n;

		Ok(v)
	}
}

impl<W:Write> Writer<W>{
	pub fn new(sink:W)->Self{
		Self{
			sink,
			state:
			State{
				c:1,
				v:0,
			},
		}
	}
	/// n is the number of possible values of v, like an enum. v should be less than n
	pub fn write(&mut self,mut n:u128,mut v:u64)->Result<()>{
		debug_assert!((v as u128)<n);
		debug_assert!(n<=State::CAP);
		while self.state.c*n>State::CAP{
			// Split n into two factors across the chunk boundary
			let f0=State::CAP/(self.state.c as u128);
			let f1=n.div_ceil(f0);
			let chunk=self.state.v+self.state.c as u64*(v%f0 as u64);
			//This needs to be 8 otherwise it's an error
			self.sink.write_all(&chunk.to_le_bytes())?;
			self.state.v=0;
			self.state.c=1;
			n=f1;
			v/=f0 as u64;
		}
		self.state.v+=self.state.c as u64*v;
		self.state.c*=n;
		Ok(())
	}
	pub fn flush(&mut self)->Result<()>{
		let mut c=1_u128;
		while c<self.state.c{
			c<<=8;
			// If this actually flushes data, it won't be readable in chunks...
			// TODO: fix read from non-chunk aligned data
			self.sink.write_all(&[self.state.v as u8])?;
			self.state.v>>=8;
		}
		Ok(())
	}
}

#[test]
fn round_trip()->Result<()>{
	let mut data:Vec<u8>=Vec::new();
	let mut w=Writer::new(&mut data);

	w.write(3*2u128.pow(60),123)?;
	w.write(3*2u128.pow(60),123)?;

	w.flush()?;

	let mut r=Reader::new(std::io::Cursor::new(data));

	assert_eq!(r.read(3*2u128.pow(60))?,123);
	assert_eq!(r.read(3*2u128.pow(60))?,123);

	Ok(())
}
#[test]
fn edge_case()->Result<()>{
	let mut data:Vec<u8>=Vec::new();
	let mut w=Writer::new(&mut data);

	w.write(1<<64,123)?;
	w.write(1<<64,123)?;

	w.flush()?;

	let mut r=Reader::new(std::io::Cursor::new(data));

	assert_eq!(r.read(1<<64)?,123);
	assert_eq!(r.read(1<<64)?,123);

	Ok(())
}
