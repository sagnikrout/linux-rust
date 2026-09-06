//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/sprd/pll.h
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
// Spreadtrum pll clock driver
//
// Copyright (C) 2015~2017 Spreadtrum, Inc.
// Author: Chunyan Zhang <chunyan.zhang@spreadtrum.com>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_cfg {
    pub val: u32,
    pub msk: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_bit_field {
    pub shift: u8,
    pub width: u8,
}

//
// struct sprd_pll - definition of adjustable pll clock
//
// @reg:	registers used to set the configuration of pll clock,
// reg[0] shows how many registers this pll clock uses.
// @itable:	pll ibias table, itable[0] means how many items this
// table includes
// @udelay	delay time after setting rate
// @factors	used to calculate the pll clock rate
// @fvco:	fvco threshold rate
// @fflag:	fvco flag
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sprd_pll {
    pub regs_num: u32,
    pub itable: *const u64,
    pub factors: *const clk_bit_field,
    pub udelay: u16,
    pub k1: u16,
    pub k2: u16,
    pub fflag: u16,
    pub fvco: u64,
    pub common: sprd_clk_common,
}

extern "C" {
    pub fn container_of(_arg: common, sprd_pll: struct, _arg: common) -> return;
}
