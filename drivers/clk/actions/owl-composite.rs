//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/actions/owl-composite.h
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
// OWL composite clock driver
//
// Copyright (c) 2014 Actions Semi Inc.
// Author: David Liu <liuwei@actions-semi.com>
//
// Copyright (c) 2018 Linaro Ltd.
// Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>

#[repr(C)]
#[derive(Copy, Clone)]
pub union owl_rate {
    pub div_hw: owl_divider_hw,
    pub factor_hw: owl_factor_hw,
    pub fix_fact_hw: clk_fixed_factor,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct owl_composite {
    pub mux_hw: owl_mux_hw,
    pub gate_hw: owl_gate_hw,
    pub rate: owl_rate,
    pub fix_fact_ops: *const clk_ops,
    pub common: owl_clk_common,
}

extern "C" {
    pub fn container_of(_arg: common, owl_composite: struct, _arg: common) -> return;
}
