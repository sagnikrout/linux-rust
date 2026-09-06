//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/ad73311.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// File:         sound/soc/codec/ad73311.h
// Based on:
// Author:       Cliff Cai <cliff.cai@analog.com>
//
// Created:      Thur Sep 25, 2008
// Description:  definitions for AD73311 registers
//
// Modified:
// Copyright 2006 Analog Devices Inc.
//
// Bugs:         Enter bugs at http://blackfin.uclinux.org
//
pub const AD_CONTROL: c_uint = 0x8000;
pub const AD_DATA: c_uint = 0x0000;
pub const AD_READ: c_uint = 0x4000;
pub const AD_WRITE: c_uint = 0x0000;
// Control register A

pub const REGA_MODE_PRO: c_uint = 0x00;
pub const REGA_MODE_DATA: c_uint = 0x01;
pub const REGA_MODE_MIXED: c_uint = 0x03;
pub const REGA_DLB: c_uint = 0x04;
pub const REGA_SLB: c_uint = 0x08;

pub const REGA_RESET: c_uint = 0x80;
// Control register B

// Control register C

// Control register D

// Control register E

// Control register F

