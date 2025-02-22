#![cfg_attr(all(not(test), feature = "no_std"), no_std)]

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

#[cfg(test)]
pub mod tests;

pub trait Drawable: draw::Draw + size::Size {}
