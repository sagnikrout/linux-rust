//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/spacemit/ccu_mix.h
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
// Copyright (c) 2024 SpacemiT Technology Co. Ltd
// Copyright (c) 2024-2025 Haylen Chu <heylenay@4d2.org>
//

//
// struct ccu_gate_config - Gate configuration
//
// @mask:	Mask to enable the gate. Some clocks may have more than one bit
// set in this field.
// @inverted:	Enable bit is inverted, 1 - disable clock, 0 - enable clock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccu_gate_config {
    pub mask: u32,
    pub inverted: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccu_factor_config {
    pub div: u32,
    pub mul: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccu_mux_config {
    pub shift: u8,
    pub width: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccu_div_config {
    pub shift: u8,
    pub width: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccu_mix {
    pub factor: ccu_factor_config,
    pub gate: ccu_gate_config,
    pub div: ccu_div_config,
    pub mux: ccu_mux_config,
    pub common: ccu_common,
}

extern "C" {
    pub fn container_of(_arg: common, ccu_mix: struct, _arg: common) -> return;
}
