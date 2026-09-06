//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/rt5668.h
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
// linux/sound/rt5668.h -- Platform data for RT5668
//
// Copyright 2018 Realtek Microelectronics
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt5668_dmic1_data_pin {
    RT5668_DMIC1_NULL,
    RT5668_DMIC1_DATA_GPIO2,
    RT5668_DMIC1_DATA_GPIO5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt5668_dmic1_clk_pin {
    RT5668_DMIC1_CLK_GPIO1,
    RT5668_DMIC1_CLK_GPIO3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt5668_jd_src {
    RT5668_JD_NULL,
    RT5668_JD1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt5668_platform_data {
    pub dmic1_data_pin: rt5668_dmic1_data_pin,
    pub dmic1_clk_pin: rt5668_dmic1_clk_pin,
    pub jd_src: rt5668_jd_src,
}
