pub mod init;
pub mod rendering;
pub mod shader;

use bytemuck::{ Pod, Zeroable };

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Zeroable, Pod)]
pub struct Vector2
{
    pub x: f32,
    pub y: f32,
}

impl Vector2
{
    pub fn new(x: f32, y: f32) -> Vector2
    {
        Vector2 { x, y }
    }
}

impl std::ops::Add for Vector2
{
    type Output = Self;

    fn add (self, rhs: Self) -> Self::Output
    {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl std::ops::Sub for Vector2
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output
    {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl std::ops::AddAssign for Vector2
{
    fn add_assign(&mut self, rhs: Self)
    {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::SubAssign for Vector2
{
    fn sub_assign(&mut self, rhs: Self)
    {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
