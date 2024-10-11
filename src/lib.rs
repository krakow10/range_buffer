use std::io::Result;

// #[derive(Default)]
// struct State{
// 	c:u64,
// 	v:u64,
// }

// pub struct Reader<R:Read>{
// 	source:R,
// 	state:State,
// }
// impl<R:Read> Reader<R>{
// 	pub fn new(source:R)->Self{
// 		Self{
// 			source,
// 			state:State::default(),
// 		}
// 	}
// }

// impl<R:Read> Read for Reader<R>{
// 	fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
// 		//Initialize an zeroed array of 8 bytes
// 		let mut buf=[0;8];
// 		self.source.read_exact(&mut buf)?;
// 		Ok(8)
// 	}
// }


struct RangeBuffer{
	data:Vec<u8>,
	c:u128,
	v:u64,
}

impl RangeBuffer{
	const CAP:u128=1<<64;
	pub fn new()->Self{
		Self{
			data:Vec::new(),
			c:1,
			v:0,
		}
	}
	pub fn read(&mut self,mut n:u128)->u64{
		debug_assert!(n<=(u64::MAX as u128+1));
		let mut v=0_u64;
		let mut c=1_u128;
		while n*self.c>Self::CAP{
			let f0 = (u64::MAX as u128+1)/(self.c as u128);
			let f1 = n.div_ceil(f0);
			v+=c as u64*self.v;
			c*=f0;
			n=f1;

			let bytes=[
				self.data.pop().unwrap(),
				self.data.pop().unwrap(),
				self.data.pop().unwrap(),
				self.data.pop().unwrap(),
				self.data.pop().unwrap(),
				self.data.pop().unwrap(),
				self.data.pop().unwrap(),
				self.data.pop().unwrap(),
			];
			// The list is reversed!
			self.v=u64::from_be_bytes(bytes);
			self.c=1;
		}

		v+=c as u64*(self.v%n as u64);
		self.v/=n as u64;
		self.c*=n;

		v
	}
	pub fn write(&mut self,mut n:u128,mut v:u64){
		debug_assert!(n<=(u64::MAX as u128+1));
		while self.c*n>Self::CAP{
			// Split n into two factors across the chunk boundary
			let f0 = (u64::MAX as u128+1)/(self.c as u128);
			let f1 = n.div_ceil(f0);
			let chunk=self.v+self.c as u64*(v%f0 as u64);
			self.data.extend_from_slice(&mut chunk.to_le_bytes());
			self.v=0;
			self.c=1;
			n=f1;
			v/=f0 as u64;
		}
		self.v+=self.c as u64*v;
		self.c*=n;
	}
	pub fn flush(&mut self){
		let mut c=1_u128;
		while c<self.c{
			c<<=8;
			self.data.push(self.v as u8);
			self.v>>=8;
		}
	}
	pub fn take(mut self)->Vec<u8>{
		self.flush();
		self.data
	}
	/// Finalize data with .flush() before getting a slice
	pub fn as_slice(&self)->&[u8]{
		self.data.as_slice()
	}
}

#[test]
fn the(){
	let mut r=RangeBuffer::new();

	r.write(3*2u128.pow(60),123);
	r.write(3*2u128.pow(60),123);

	r.flush();
	let bytes=r.as_slice();
	println!("{:?}",bytes);

	assert_eq!(r.read(3*2u128.pow(60)),123);
	assert_eq!(r.read(3*2u128.pow(60)),123);
}
