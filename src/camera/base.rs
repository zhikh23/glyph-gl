use crate::math::matrices::Matrix4;

pub trait Camera {
    fn view(&self) -> Matrix4;
    fn proj(&self) -> Matrix4;
}
