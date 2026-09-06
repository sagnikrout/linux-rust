//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/ast/ast_post.h
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


// SPDX-License-Identifier: MIT

// DRAM timing tables
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ast_dramstruct {
    pub index: u32,
    pub data: u32,
}

// control commands
pub const __AST_DRAMSTRUCT_UDELAY: c_uint = 0xff00;
pub const __AST_DRAMSTRUCT_INVALID: c_uint = 0xffff;

extern "C" {
    pub fn mmc_test(ast: *mut ast_device, datagen: u32, test_ctl: u8) -> bool;
}
extern "C" {
    pub fn mmc_test_burst(ast: *mut ast_device, datagen: u32) -> bool;
}
// ast_2000.c
extern "C" {
    pub fn ast_2000_set_def_ext_reg(ast: *mut ast_device);
}
// ast_2300.c
extern "C" {
    pub fn ast_2300_set_def_ext_reg(ast: *mut ast_device);
}
