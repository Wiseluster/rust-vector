use std::convert::*;
use std::fmt::*;
use std::ops::*;

#[derive(Clone, Copy, Debug)]
pub struct Vector
{
	x: f64,
	y: f64,
	z: f64
}

impl Vector
{
	pub fn at<T, U, V>(x: T, y: U, z: V) -> Vector
		where T: Into<f64>,
		      U: Into<f64>,
		      V: Into<f64>
	{
		Vector {x: x.into(), y: y.into(), z: z.into()}
	}
}

impl Add for Vector
{
	type Output = Vector;

	fn add(self, other: Vector) -> Vector
	{
		Vector::at(self.x + other.x, self.y + other.y, self.z + other.z)
	}
}

impl AddAssign for Vector
{
	fn add_assign(&mut self, other: Vector)
	{
		*self = *self + other;
	}
}

impl Sub for Vector
{
	type Output = Vector;

	fn sub(self, other: Vector) -> Vector
	{
		Vector::at(self.x - other.x, self.y - other.y, self.z - other.z)
	}
}

impl SubAssign for Vector
{
	fn sub_assign(&mut self, other: Vector)
	{
		*self = *self - other;
	}
}

impl Mul for Vector
{
	type Output = f64;

	fn mul(self, other: Vector) -> f64
	{
		self.x * other.x + self.y * other.y + self.z * other.z
	}
}

impl <T> Mul<T> for Vector
	where T: Into<f64>
{
	type Output = Vector;

	fn mul(self, val: T) -> Vector
	{
		let scale = val.into();

		Vector::at(self.x * scale, self.y * scale, self.z * scale)
	}
}

impl <T> MulAssign<T> for Vector
	where T: Into<f64>
{
	fn mul_assign(&mut self, val: T)
	{
		*self = *self * val;
	}
}

impl Rem for Vector
{
	type Output = Vector;

	fn rem(self, other: Vector) -> Vector
	{
		Vector::at(self.y * other.z - self.z * other.y, self.z * other.x - self.x * other.z, self.x * other.y - self.y * other.x)
	}
}

impl RemAssign for Vector
{
	fn rem_assign(&mut self, other: Vector)
	{
		*self = *self % other;
	}
}

impl Neg for Vector
{
	type Output = Vector;

	fn neg(self) -> Vector
	{
		Vector::at(-self.x, -self.y, -self.z)
	}
}

impl Not for Vector
{
	type Output = f64;

	fn not(self) -> f64
	{
		(self * self).sqrt()
	}
}

impl Display for Vector
{
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "({}, {}, {})", self.x, self.y, self.z)
	}
}
