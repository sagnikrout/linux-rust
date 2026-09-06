//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtl818x/rtl8180/max2820.h
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
// Radio tuning for Maxim max2820 on RTL8180
//
// Copyright 2007 Andrea Merello <andrea.merello@gmail.com>
//
// Code from the BSD driver and the rtl8181 project have been
// very useful to understand certain things
//
// I want to thanks the Authors of such projects and the Ndiswrapper
// project Authors.
//
// A special Big Thanks also is for all people who donated me cards,
// making possible the creation of the original rtl8180 driver
// from which this code is derived!
//
pub const MAXIM_ANTENNA: c_uint = 0xb3;
