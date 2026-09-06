//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/consolemap.h
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
// consolemap.h
//
// Interface between console.c, selection.c  and consolemap.c
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum translation_map {
    LAT1_MAP,
    GRAF_MAP,
    IBMPC_MAP,
    USER_MAP,

    FIRST_MAP = LAT1_MAP,
    LAST_MAP = USER_MAP,
}

extern "C" {
    pub fn inverse_translate(conp: *const vc_data, glyph: u16, use_unicode: bool) -> u16;
}
extern "C" {
    pub fn conv_uni_to_pc(conp: *mut vc_data, ucs: c_long) -> c_int;
}
extern "C" {
    pub fn conv_8bit_to_uni(c: c_uchar) -> u32;
}
extern "C" {
    pub fn conv_uni_to_8bit(uni: u32) -> c_int;
}
extern "C" {
    pub fn console_map_init();
}
extern "C" {
    pub fn ucs_get_width(cp: u32) -> c_uint;
}
extern "C" {
    pub fn ucs_recompose(base: u32, mark: u32) -> u32;
}
extern "C" {
    pub fn ucs_get_fallback(cp: u32) -> u32;
}

