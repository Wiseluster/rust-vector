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
		Vector {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
	}
}

impl AddAssign for Vector
{
	fn add_assign(&mut self, other: Vector)
	{
		self.x += other.x;
		self.y += other.y;
		self.z += other.z;
	}
}

impl Sub for Vector
{
	type Output = Vector;

	fn sub(self, other: Vector) -> Vector
	{
		Vector {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
	}
}

impl SubAssign for Vector
{
	fn sub_assign(&mut self, other: Vector)
	{
		self.x -= other.x;
		self.y -= other.y;
		self.z -= other.z;
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

	fn mul(self, value: T) -> Vector
	{
		let scale = value.into();

		Vector {x: self.x * scale, y: self.y * scale, z: self.z * scale}
	}
}

impl <T> MulAssign<T> for Vector
	where T: Into<f64>
{
	fn mul_assign(&mut self, value: T)
	{
		let scale = value.into();

		self.x *= scale;
		self.y *= scale;
		self.z *= scale;
	}
}

impl Rem for Vector
{
	type Output = Vector;

	fn rem(self, other: Vector) -> Vector
	{
		Vector {x: self.y * other.z - self.z * other.y, y: self.z * other.x - self.x * other.z, z: self.x * other.y - self.y * other.x}
	}
}

impl RemAssign for Vector
{
	fn rem_assign(&mut self, other: Vector)
	{
		let x = self.y * other.z - self.z * other.y;
		let y = self.z * other.x - self.x * other.z;
		let z = self.x * other.y - self.y * other.x;

		self.x = x;
		self.y = y;
		self.z = z;
	}
}

impl Neg for Vector
{
	type Output = Vector;

	fn neg(self) -> Vector
	{
		Vector {x: -self.x, y: -self.y, z: -self.z}
	}
}

impl Not for Vector
{
	type Output = f64;

	fn not(self) -> f64
	{
		(self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
	}
}

impl Display for Vector
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		write!(formatter, "({}, {}, {})", self.x, self.y, self.z)
	}
}
