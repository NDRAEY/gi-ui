#![cfg_attr(all(not(test), feature = "no_std"), no_std)]

use core::any::Any;

extern crate alloc;

pub mod canvas;
pub mod components;
pub mod draw;

#[macro_use]
pub mod position;

#[macro_use]
pub mod size;
pub mod alignment;
pub mod side;

// #[cfg(test)]
// pub mod tests;

pub mod helpers;
pub mod rect;

pub trait Drawable: draw::Draw + size::Size {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn parent(&self) -> Option<&dyn Drawable> {
        None
    }

    fn set_parent(&mut self, _parent: Box<dyn Drawable>) {
        todo!();
    }
}
