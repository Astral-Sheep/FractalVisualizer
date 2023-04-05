pub mod init;
pub mod rendering;
pub mod shader;

use bytemuck::{ Pod, Zeroable };

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Zeroable, Pod)]
pub struct Vector2_32
{
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Zeroable, Pod)]
pub struct Vector2_64
{
    pub x: f64,
    pub y: f64,
}

impl Vector2_32
{
    pub fn new(x: f32, y: f32) -> Self
    {
        Self { x, y }
    }
}

impl std::ops::Add for Vector2_32
{
    type Output = Self;

    fn add (self, rhs: Self) -> Self::Output
    {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl std::ops::AddAssign for Vector2_32
{
    fn add_assign(&mut self, rhs: Self)
    {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Vector2_64
{
    pub fn new(x: f64, y: f64) -> Self
    {
        Self { x, y }
    }
}

impl std::ops::Add for Vector2_64
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output
    {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl std::ops::AddAssign for Vector2_64
{
    fn add_assign(&mut self, rhs: Self)
    {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
