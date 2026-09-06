//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/qcom/clk-branch.h
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
// Copyright (c) 2013, The Linux Foundation. All rights reserved.

//
// struct clk_branch - gating clock with status bit and dynamic hardware gating
//
// @hwcg_reg: dynamic hardware clock gating register
// @hwcg_bit: ORed with @hwcg_reg to enable dynamic hardware clock gating
// @halt_reg: halt register
// @halt_bit: ANDed with @halt_reg to test for clock halted
// @halt_check: type of halt checking to perform
// @clkr: handle between common and hardware-specific interfaces
//
// Clock which can gate its output.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_branch {
    pub hwcg_reg: u32,
    pub halt_reg: u32,
    pub hwcg_bit: u8,
    pub halt_bit: u8,
    pub halt_check: u8,

    pub clkr: clk_regmap,
}

//
// struct clk_mem_branch - gating clock which are associated with memories
//
// @mem_enable_reg: branch clock memory gating register
// @mem_ack_reg: branch clock memory ack register
// @mem_enable_ack_mask: branch clock memory enable and ack field in @mem_ack_reg
// @mem_enable_mask: branch clock memory enable mask
// @mem_enable_invert: branch clock memory enable and disable has invert logic
// @branch: branch clock gating handle
//
// Clock which can gate its memories.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_mem_branch {
    pub mem_enable_reg: u32,
    pub mem_ack_reg: u32,
    pub mem_enable_ack_mask: u32,
    pub mem_enable_mask: u32,
    pub mem_enable_invert: bool,
    pub branch: clk_branch,
}

// Branch clock common bits for HLOS-owned clocks

