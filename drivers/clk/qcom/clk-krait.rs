//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/qcom/clk-krait.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct krait_mux_clk {
    pub parent_map: *mut c_uint,
    pub offset: u32,
    pub mask: u32,
    pub shift: u32,
    pub en_mask: u32,
    pub lpl: bool,
    pub safe_sel: u8,
    pub old_index: u8,
    pub reparent: bool,
    pub disable_sec_src_gating: bool,
    pub hw: clk_hw,
    pub clk_nb: notifier_block,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct krait_div2_clk {
    pub offset: u32,
    pub width: u8,
    pub shift: u32,
    pub lpl: bool,
    pub hw: clk_hw,
}

