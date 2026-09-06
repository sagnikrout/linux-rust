//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wlcore/io.h
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
// This file is part of wl1271
//
// Copyright (C) 1998-2009 Texas Instruments. All rights reserved.
// Copyright (C) 2008-2010 Nokia Corporation
//
// Contact: Luciano Coelho <luciano.coelho@nokia.com>
//

pub const HW_ACCESS_MEMORY_MAX_RANGE: c_uint = 0x1FFC0;
pub const HW_PARTITION_REGISTERS_ADDR: c_uint = 0x1FFC0;

pub const HW_ACCESS_REGISTER_SIZE: c_int = 4;
pub const HW_ACCESS_PRAM_MAX_RANGE: c_uint = 0x3c000;
extern "C" {
    pub fn wlcore_disable_interrupts(wl: *mut wl1271);
}
extern "C" {
    pub fn wlcore_disable_interrupts_nosync(wl: *mut wl1271);
}
extern "C" {
    pub fn wlcore_enable_interrupts(wl: *mut wl1271);
}
extern "C" {
    pub fn wlcore_synchronize_interrupts(wl: *mut wl1271);
}
extern "C" {
    pub fn wl1271_io_reset(wl: *mut wl1271);
}
extern "C" {
    pub fn wl1271_io_init(wl: *mut wl1271);
}
extern "C" {
    pub fn wlcore_translate_addr(wl: *mut wl1271, addr: c_int) -> c_int;
}
// Raw target IO, address is not translated
extern "C" {
    pub fn wlcore_raw_read(_arg: wl, _arg: wl->rtable[reg], _arg: buf, _arg: len, _arg: fixed) -> return;
}
extern "C" {
    pub fn wlcore_raw_write(_arg: wl, _arg: wl->rtable[reg], _arg: buf, _arg: len, _arg: fixed) -> return;
}
// val = le32_to_cpu(*wl->buffer_32);
// wl->buffer_32 = cpu_to_le32(val);
extern "C" {
    pub fn wlcore_raw_read(_arg: wl, _arg: physical, _arg: buf, _arg: len, _arg: fixed) -> return;
}
extern "C" {
    pub fn wlcore_raw_write(_arg: wl, _arg: physical, _arg: buf, _arg: len, _arg: fixed) -> return;
}
extern "C" {
    pub fn wlcore_write(_arg: wl, _arg: wl->rtable[reg], _arg: buf, _arg: len, _arg: fixed) -> return;
}
extern "C" {
    pub fn wlcore_read(_arg: wl, _arg: wl->rtable[reg], _arg: buf, _arg: len, _arg: fixed) -> return;
}
// Convert from FW internal address which is chip arch dependent
extern "C" {
    pub fn wlcore_raw_read(_arg: wl, _arg: physical, _arg: buf, _arg: len, _arg: fixed) -> return;
}
extern "C" {
    pub fn wlcore_raw_read32(_arg: wl, _arg: wlcore_translate_addr(wl, _arg: addr), _arg: val) -> return;
}
extern "C" {
    pub fn wlcore_raw_write32(_arg: wl, _arg: wlcore_translate_addr(wl, _arg: addr), _arg: val) -> return;
}
extern "C" {
    pub fn wl1271_set_block_size(wl: *mut wl1271) -> bool;
}
// Functions from wl1271_main.c
extern "C" {
    pub fn wl1271_tx_dummy_packet(wl: *mut wl1271) -> c_int;
}
