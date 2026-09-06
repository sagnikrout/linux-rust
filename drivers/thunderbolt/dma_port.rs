//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/thunderbolt/dma_port.h
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
// Thunderbolt DMA configuration based mailbox support
//
// Copyright (C) 2017, Intel Corporation
// Authors: Michael Jamet <michael.jamet@intel.com>
// Mika Westerberg <mika.westerberg@linux.intel.com>
//

pub const DMA_PORT_CSS_ADDRESS: c_uint = 0x3fffff;

extern "C" {
    pub fn dma_port_free(dma: *mut tb_dma_port);
}
extern "C" {
    pub fn dma_port_flash_update_auth(dma: *mut tb_dma_port) -> c_int;
}
extern "C" {
    pub fn dma_port_flash_update_auth_status(dma: *mut tb_dma_port, status: *mut u32) -> c_int;
}
extern "C" {
    pub fn dma_port_power_cycle(dma: *mut tb_dma_port) -> c_int;
}
