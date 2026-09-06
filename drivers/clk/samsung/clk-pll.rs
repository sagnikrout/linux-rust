//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/samsung/clk-pll.h
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
// Copyright (c) 2013 Samsung Electronics Co., Ltd.
// Copyright (c) 2013 Linaro Ltd.
//
// Common Clock Framework support for all PLL's in Samsung platforms
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum samsung_pll_type {
    pll_2126,
    pll_3000,
    pll_35xx,
    pll_36xx,
    pll_2550,
    pll_2650,
    pll_4500,
    pll_4502,
    pll_4508,
    pll_4600,
    pll_4650,
    pll_4650c,
    pll_6552,
    pll_6552_s3c2416,
    pll_6553,
    pll_2550x,
    pll_2550xx,
    pll_2650x,
    pll_2650xx,
    pll_1417x,
    pll_1418x,
    pll_1450x,
    pll_1451x,
    pll_1452x,
    pll_1460x,
    pll_0818x,
    pll_0822x,
    pll_0831x,
    pll_142xx,
    pll_0516x,
    pll_0517x,
    pll_0518x,
    pll_531x,
    pll_1051x,
    pll_1052x,
    pll_0717x,
    pll_0718x,
    pll_0732x,
    pll_4311,
    pll_1017x,
    pll_1031x,
    pll_a9fracm,
    pll_a9fraco,
}

// NOTE: Rate table should be kept sorted in descending order.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct samsung_pll_rate_table {
    pub rate: c_uint,
    pub pdiv: c_uint,
    pub mdiv: c_uint,
    pub sdiv: c_uint,
    pub kdiv: c_uint,
    pub afc: c_uint,
    pub mfr: c_uint,
    pub mrr: c_uint,
    pub vsel: c_uint,
}
