//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/serial-omap.h
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
// Driver for OMAP-UART controller.
// Based on drivers/serial/8250.c
//
// Copyright (C) 2010 Texas Instruments.
//
// Authors:
// Govindraj R	<govindraj.raja@ti.com>
// Thara Gopinath	<thara@ti.com>
//

//
// Use tty device name as ttyO, [O -> OMAP]
// in bootargs we specify as console=ttyO0 if uart1
// is used as console uart.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_uart_port_info {
    pub /: *mut *mut bool dma_enabled; / To specify DMA Mode,
    pub /: *mut *mut unsigned int uartclk; / UART clock rate,
    pub /: *mut *mut *mut upf_t flags; / UPF_ flags,
    pub dma_rx_buf_size: c_uint,
    pub dma_rx_timeout: c_uint,
    pub autosuspend_timeout: c_uint,
    pub dma_rx_poll_rate: c_uint,
    pub ): *mut *mut int (get_context_loss_count)(struct device,
    pub bool): *mut *mut *mut void (enable_wakeup)(struct device ,,
}
