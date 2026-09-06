//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/regulator/userspace-consumer.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0
//
// struct regulator_userspace_consumer_data - line consumer
// initialisation data.
//
// @name: Name for the consumer line
// @num_supplies: Number of supplies feeding the line
// @supplies: Supplies configuration.
// @init_on: Set if the regulators supplying the line should be
// enabled during initialisation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regulator_userspace_consumer_data {
    pub name: *const c_char,
    pub num_supplies: c_int,
    pub supplies: *mut regulator_bulk_data,
    pub init_on: bool,
    pub no_autoswitch: bool,
}
