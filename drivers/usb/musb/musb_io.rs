//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/musb/musb_io.h
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
// MUSB OTG driver register I/O
//
// Copyright 2005 Mentor Graphics Corporation
// Copyright (C) 2005-2006 by Texas Instruments
// Copyright (C) 2006-2007 Nokia Corporation
//

//
// struct musb_io - IO functions for MUSB
// @ep_offset:	platform specific function to get end point offset
// @ep_select:	platform specific function to select end point
// @fifo_offset: platform specific function to get fifo offset
// @read_fifo:	platform specific function to read fifo
// @write_fifo:	platform specific function to write fifo
// @busctl_offset: platform specific function to get busctl offset
// @get_toggle: platform specific function to get toggle
// @set_toggle: platform specific function to set toggle
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct musb_io {
    pub offset): *mut *mut u32 (ep_offset)(u8 epnum, u16,
    pub epnum): *mut *mut *mut void (ep_select)(void __iomem mbase, u8,
    pub epnum): *mut *mut u32 (fifo_offset)(u8,
    pub buf): *mut *mut *mut void (read_fifo)(struct musb_hw_ep hw_ep, u16 len, u8,
    pub buf): *const *const *const void (write_fifo)(struct musb_hw_ep hw_ep, u16 len, u8,
    pub offset): *mut *mut u32 (busctl_offset)(u8 epnum, u16,
    pub is_out): *mut *mut *mut u16 (get_toggle)(struct musb_qh qh, int,
    pub urb): *mut *mut *mut u16 (set_toggle)(struct musb_qh qh, int is_out, struct urb,
}

// Do not add new entries here, add them the struct musb_io instead
extern "C" {
    pub fn u8(addr: *mut *mut musb_readb)(void __iomem, offset: u32) -> extern;
}
extern "C" {
    pub fn void(addr: *mut *mut musb_writeb)(void __iomem, offset: u32, data: u8) -> extern;
}
extern "C" {
    pub fn u8(addr: *mut *mut musb_clearb)(void __iomem, offset: u32) -> extern;
}
extern "C" {
    pub fn u16(addr: *mut *mut musb_readw)(void __iomem, offset: u32) -> extern;
}
extern "C" {
    pub fn void(addr: *mut *mut musb_writew)(void __iomem, offset: u32, data: u16) -> extern;
}
extern "C" {
    pub fn u16(addr: *mut *mut musb_clearw)(void __iomem, offset: u32) -> extern;
}
extern "C" {
    pub fn musb_readl(addr: *mut void __iomem, offset: u32) -> u32;
}
extern "C" {
    pub fn musb_writel(addr: *mut void __iomem, offset: u32, data: u32);
}
