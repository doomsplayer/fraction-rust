extern mod std;

#[deriving(Eq)]
priv struct fraction{
	x:int,
	y:int
}

impl fraction {
	priv fn gcd(&mut self)->fraction{
		let mut (x,y) = (self.x,self.y);
		while x % y != 0{
			let z = x % y;
			x=y;
			y=z;
		}
		fraction{x:self.x/y,y:self.y/y}
	}	
	pub fn new(x:int,y:int)->fraction{
		fraction{x:x,y:y}.gcd()
	}
}

impl std::to_str::ToStr for fraction {
	fn to_str(&self) -> ~str {
		if self.y!=1{
			fmt!("%d/%d",self.x,self.y)
		}else{
			fmt!("%d",self.x)
		}
	}
}

trait To_Decimal{
	fn to_decimal(&self)->fraction;
}

impl To_Decimal for uint{
	fn to_decimal(&self)->fraction{
		fraction{x:*self as int,y:1}
	}
}

impl To_Decimal for int{
	fn to_decimal(&self)->fraction{
		fraction{x:*self,y:1}
	}
}

impl To_Decimal for fraction {
	fn to_decimal(&self)->fraction{
		*self
	}
}

impl<T:To_Decimal> std::ops::Add<T,fraction> for fraction{
	fn add(&self,rhs:&T)->fraction{
		let r = rhs.to_decimal();
		fraction{x:self.x*r.y+r.x*self.y,y:self.y*r.y}.gcd()
	}
}

impl<T:To_Decimal> std::ops::Sub<T,fraction> for fraction{
	 fn sub(&self,rhs:&T)->fraction{
		let r = rhs.to_decimal();
		fraction{x:self.x*r.y-r.x*self.y,y:self.y*r.y}.gcd()
	}
}

impl<T:To_Decimal> std::ops::Mul<T,fraction> for fraction{
	fn mul(&self,rhs:&T)->fraction{
		fraction{x:self.x*rhs.to_decimal().x,y:self.y*rhs.to_decimal().y}.gcd()
	}
}

impl<T:To_Decimal> std::ops::Div<T,fraction> for fraction{
	fn div(&self,rhs:&T)->fraction{
		fraction{x:self.x*rhs.to_decimal().y,y:self.y*rhs.to_decimal().x}.gcd()
	}
}