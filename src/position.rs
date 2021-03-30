use bevy::math::Vec3;

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

// Pour définir la place que prend une forme (Shape) on définit
// l'ensemble de ses carrés qui font chacun 50 x 50 px
impl Position {
    pub fn to_vec(&self) -> Vec3 {
        Vec3::new(self.x as f32, self.y as f32, 0.)
    }
}
