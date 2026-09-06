//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dma/dw.h
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
// Driver for the Synopsys DesignWare DMA Controller
//
// Copyright (C) 2007 Atmel Corporation
// Copyright (C) 2010-2011 ST Microelectronics
// Copyright (C) 2014 Intel Corporation
//

//
// struct dw_dma_chip - representation of DesignWare DMA controller hardware
// @dev:		struct device of the DMA controller
// @id:			instance ID
// @irq:		irq line
// @regs:		memory mapped I/O space
// @clk:		hclk clock
// @dw:			struct dw_dma that is filed by dw_dma_probe()
// @pdata:		pointer to platform data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_dma_chip {
    pub dev: *mut device,
    pub id: c_int,
    pub irq: c_int,
    pub regs: *mut void __iomem,
    pub clk: *mut clk,
    pub dw: *mut dw_dma,
    pub pdata: *const dw_dma_platform_data,
}

// Export to the platform drivers

extern "C" {
    pub fn dw_dma_probe(chip: *mut dw_dma_chip) -> c_int;
}
extern "C" {
    pub fn dw_dma_remove(chip: *mut dw_dma_chip) -> c_int;
}
extern "C" {
    pub fn idma32_dma_probe(chip: *mut dw_dma_chip) -> c_int;
}
extern "C" {
    pub fn idma32_dma_remove(chip: *mut dw_dma_chip) -> c_int;
}

