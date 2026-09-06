//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/sunxi/clk-factors.h
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
pub struct clk_factors_config {
    pub nshift: u8,
    pub nwidth: u8,
    pub kshift: u8,
    pub kwidth: u8,
    pub mshift: u8,
    pub mwidth: u8,
    pub pshift: u8,
    pub pwidth: u8,
    pub n_start: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct factors_request {
    pub rate: c_ulong,
    pub parent_rate: c_ulong,
    pub parent_index: u8,
    pub n: u8,
    pub k: u8,
    pub m: u8,
    pub p: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct factors_data {
    pub enable: c_int,
    pub mux: c_int,
    pub muxmask: c_int,
    pub table: *const clk_factors_config,
    pub req): *mut *mut void (getter)(struct factors_request,
    pub req): *mut *mut void (recalc)(struct factors_request,
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_factors {
    pub hw: clk_hw,
    pub reg: *mut void __iomem,
    pub config: *const clk_factors_config,
    pub req): *mut *mut void (get_factors)(struct factors_request,
    pub req): *mut *mut void (recalc)(struct factors_request,
    pub lock: *mut spinlock_t,
// for cleanup
    pub mux: *mut clk_mux,
    pub gate: *mut clk_gate,
}

extern "C" {
    pub fn sunxi_factors_unregister(node: *mut device_node, clk: *mut clk);
}
