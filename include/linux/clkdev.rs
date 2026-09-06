//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/clkdev.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// include/linux/clkdev.h
//
// Copyright (C) 2008 Russell King.
//
// Helper for the clk API to assist looking up a struct clk.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_lookup {
    pub node: list_head,
    pub dev_id: *const c_char,
    pub con_id: *const c_char,
    pub clk: *mut clk,
    pub clk_hw: *mut clk_hw,
}

extern "C" {
    pub fn clkdev_add(cl: *mut clk_lookup);
}
extern "C" {
    pub fn clkdev_drop(cl: *mut clk_lookup);
}
extern "C" {
    pub fn clkdev_add_table(: *mut clk_lookup, _arg: usize);
}
extern "C" {
    pub fn clk_add_alias(: *const c_char, : *const c_char, : *const c_char, : *mut device) -> c_int;
}
extern "C" {
    pub fn clk_register_clkdev(: *mut clk, : *const c_char, : *const c_char) -> c_int;
}
extern "C" {
    pub fn clk_hw_register_clkdev(: *mut clk_hw, : *const c_char, : *const c_char) -> c_int;
}
