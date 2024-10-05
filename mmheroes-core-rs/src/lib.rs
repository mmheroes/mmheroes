#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unusual_byte_groupings)]
#![feature(context_ext)]
#![feature(local_waker)]
#![feature(layout_for_ptr)]

pub mod util;

mod random;

pub mod ffi;
pub mod logic;
pub mod ui;
