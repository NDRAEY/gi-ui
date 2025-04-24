use crate::Drawable;

pub trait HasParent<'a> {
    // fn parent_mut(&mut self) -> Option<&mut dyn Drawable> {
    //     None
    // }

    fn parent(&self) -> Option<&dyn Drawable> {
        None
    }

    fn set_parent(&'a mut self, parent: &'a dyn Drawable) {
        todo!();
    }
}
