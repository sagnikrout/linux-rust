//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpio/gpio-i8255.h
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
// Copyright 2022 William Breathitt Gray

//
// struct i8255_regmap_config - Configuration for the register map of an i8255
// @parent:	parent device
// @map:	regmap for the i8255
// @num_ppi:	number of i8255 Programmable Peripheral Interface
// @names:	(optional) array of names for gpios
// @domain:	(optional) IRQ domain if the controller is interrupt-capable
//
// Note: The regmap is expected to have cache enabled and i8255 control
// registers not marked as volatile.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i8255_regmap_config {
    pub parent: *mut device,
    pub map: *mut regmap,
    pub num_ppi: c_int,
    pub names: *const *const c_char,
    pub domain: *mut irq_domain,
}
