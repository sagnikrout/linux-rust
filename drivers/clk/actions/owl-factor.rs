//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/actions/owl-factor.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// OWL factor clock driver
//
// Copyright (c) 2014 Actions Semi Inc.
// Author: David Liu <liuwei@actions-semi.com>
//
// Copyright (c) 2018 Linaro Ltd.
// Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_factor_table {
    pub val: c_uint,
    pub mul: c_uint,
    pub div: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct owl_factor_hw {
    pub reg: u32,
    pub shift: u8,
    pub width: u8,
    pub fct_flags: u8,
    pub table: *mut clk_factor_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct owl_factor {
    pub factor_hw: owl_factor_hw,
    pub common: owl_clk_common,
}

extern "C" {
    pub fn container_of(_arg: common, owl_factor: struct, _arg: common) -> return;
}
