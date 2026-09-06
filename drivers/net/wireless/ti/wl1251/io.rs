//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wl1251/io.h
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
// This file is part of wl12xx
//
// Copyright (C) 2008 Nokia Corporation
//

pub const HW_ACCESS_MEMORY_MAX_RANGE: c_uint = 0x1FFC0;
pub const HW_ACCESS_PART0_SIZE_ADDR: c_uint = 0x1FFC0;
pub const HW_ACCESS_PART0_START_ADDR: c_uint = 0x1FFC4;
pub const HW_ACCESS_PART1_SIZE_ADDR: c_uint = 0x1FFC8;
pub const HW_ACCESS_PART1_START_ADDR: c_uint = 0x1FFCC;
pub const HW_ACCESS_REGISTER_SIZE: c_int = 4;
pub const HW_ACCESS_PRAM_MAX_RANGE: c_uint = 0x3c000;
extern "C" {
    pub fn le32_to_cpu(_arg: wl->buffer_32) -> return;
}
// Memory target IO, address is translated to partition 0
extern "C" {
    pub fn wl1251_mem_read(wl: *mut wl1251, addr: c_int, buf: *mut c_void, len: usize);
}
extern "C" {
    pub fn wl1251_mem_write(wl: *mut wl1251, addr: c_int, buf: *mut c_void, len: usize);
}
extern "C" {
    pub fn wl1251_mem_read32(wl: *mut wl1251, addr: c_int) -> u32;
}
extern "C" {
    pub fn wl1251_mem_write32(wl: *mut wl1251, addr: c_int, val: u32);
}
// Registers IO
extern "C" {
    pub fn wl1251_reg_read32(wl: *mut wl1251, addr: c_int) -> u32;
}
extern "C" {
    pub fn wl1251_reg_write32(wl: *mut wl1251, addr: c_int, val: u32);
}
