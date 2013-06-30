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
		let mut ret = fraction{x:self.x/y,y:self.y/y};
		if ret.y<0 {
			ret.y=-ret.y;
			ret.x=-ret.x;
		}
		ret
	}	
	pub fn new(x:int,y:int)->fraction{
		if y<0 {
			fraction{x:-x,y:-y}.gcd()
		}else{
			fraction{x:x,y:y}.gcd()
		}
		
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

#[test]	
fn create() {
	assert!(fraction::new(234,556).to_str()==~"117/278");
}

#[test]	
fn add_num() {
	assert!((fraction::new(4,9)+3).to_str()==~"31/9");
}

#[test]	
fn add_fra() {
	assert!((fraction::new(2,6)+fraction::new(7,3)).to_str()==~"8/3");
}

#[test]	
fn sub_num() {
	assert!((fraction::new(1,9)-3).to_str()==~"-26/9");
}

#[test]	
fn sub_fra() {
	assert!((fraction::new(6,7)-fraction::new(26,14)).to_str()==~"-1");
}

#[test]	
fn mul_num() {
	assert!(((fraction::new(2,11)*(-8)).to_str()==~"-16/11"));
}

#[test]	
fn mul_fra() {
	assert!((fraction::new(77,6)*fraction::new(1,-8)).to_str()==~"-77/48");
}

#[test]	
fn div_num() {
	assert!((fraction::new(-65,79)/5).to_str()==~"-13/79");
}

#[test]	
fn div_fra() {
	assert!((fraction::new(-33,51)/fraction::new(4,-7)).to_str()==~"77/68");
}

#[test]	
fn mix() {
	let a = fraction::new(1,7);
	let b = fraction::new(29,-48);
	let c = fraction::new(32,5);
	let d = fraction::new(-7,3);
	let e = fraction::new(-23,-11);
	assert!((((a*2+b/3)/c-d+2)*(e+9)).to_str()==~"8552261/177408");
}