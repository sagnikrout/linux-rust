//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dma/hsu.h
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
// Driver for the High Speed UART DMA
//
// Copyright (C) 2015 Intel Corporation
//

//
// struct hsu_dma_chip - representation of HSU DMA hardware
// @dev:		 struct device of the DMA controller
// @irq:		 irq line
// @regs:		 memory mapped I/O space
// @length:		 I/O space length
// @offset:		 offset of the I/O space where registers are located
// @hsu:		 struct hsu_dma that is filed by ->probe()
// @pdata:		 platform data for the DMA controller if provided
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsu_dma_chip {
    pub dev: *mut device,
    pub irq: c_int,
    pub regs: *mut void __iomem,
    pub length: c_uint,
    pub offset: c_uint,
    pub hsu: *mut hsu_dma,
}

// Export to the internal users
extern "C" {
    pub fn hsu_dma_do_irq(chip: *mut hsu_dma_chip, nr: c_ushort, status: u32) -> c_int;
}
// Export to the platform drivers
extern "C" {
    pub fn hsu_dma_probe(chip: *mut hsu_dma_chip) -> c_int;
}
extern "C" {
    pub fn hsu_dma_remove(chip: *mut hsu_dma_chip) -> c_int;
}

