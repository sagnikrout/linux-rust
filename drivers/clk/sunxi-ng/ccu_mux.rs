//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/sunxi-ng/ccu_mux.h
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
pub struct ccu_mux_fixed_prediv {
    pub index: u8,
    pub div: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccu_mux_var_prediv {
    pub index: u8,
    pub shift: u8,
    pub width: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccu_mux_internal {
    pub shift: u8,
    pub width: u8,
    pub table: *const u8,
    pub fixed_predivs: *const ccu_mux_fixed_prediv,
    pub n_predivs: u8,
    pub var_predivs: *const ccu_mux_var_prediv,
    pub n_var_predivs: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccu_mux {
    pub enable: u32,
    pub mux: ccu_mux_internal,
    pub common: ccu_common,
}

extern "C" {
    pub fn container_of(_arg: common, ccu_mux: struct, _arg: common) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccu_mux_nb {
    pub clk_nb: notifier_block,
    pub common: *mut ccu_common,
    pub cm: *mut ccu_mux_internal,
    pub /: *mut *mut u32 delay_us; / How many us to wait after reparenting,
    pub /: *mut *mut u8 bypass_index; / Which parent to temporarily use,
    pub /: *mut *mut u8 original_index; / This is set by the notifier callback,
}

extern "C" {
    pub fn ccu_mux_notifier_register(clk: *mut clk, mux_nb: *mut ccu_mux_nb) -> c_int;
}
