//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/dw/internal.h
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
// Copyright (C) 2013 Intel Corporation
//

extern "C" {
    pub fn do_dma_probe(chip: *mut dw_dma_chip) -> c_int;
}
extern "C" {
    pub fn do_dma_remove(chip: *mut dw_dma_chip) -> c_int;
}
extern "C" {
    pub fn do_dw_dma_on(dw: *mut dw_dma);
}
extern "C" {
    pub fn do_dw_dma_off(dw: *mut dw_dma);
}
extern "C" {
    pub fn do_dw_dma_disable(chip: *mut dw_dma_chip) -> c_int;
}
extern "C" {
    pub fn do_dw_dma_enable(chip: *mut dw_dma_chip) -> c_int;
}
extern "C" {
    pub fn dw_dma_filter(chan: *mut dma_chan, param: *mut c_void) -> bool;
}

extern "C" {
    pub fn dw_dma_acpi_controller_register(dw: *mut dw_dma);
}
extern "C" {
    pub fn dw_dma_acpi_controller_free(dw: *mut dw_dma);
}

extern "C" {
    pub fn dw_dma_of_controller_register(dw: *mut dw_dma);
}
extern "C" {
    pub fn dw_dma_of_controller_free(dw: *mut dw_dma);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_dma_chip_pdata {
    pub pdata: *const dw_dma_platform_data,
    pub chip): *mut *mut int (probe)(struct dw_dma_chip,
    pub chip): *mut *mut int (remove)(struct dw_dma_chip,
    pub chip: *mut dw_dma_chip,
    pub m_master: u8,
    pub p_master: u8,
}
