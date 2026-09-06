//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/amba/serial.h
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
// linux/include/asm-arm/hardware/serial_amba.h
//
// Internal header file for AMBA serial ports
//
// Copyright (C) ARM Limited
// Copyright (C) 2000 Deep Blue Solutions Ltd.
//

// -------------------------------------------------------------------------------
// From AMBA UART (PL010) Block Specification
// -------------------------------------------------------------------------------
// UART Register Offsets.
//
pub const UART01x_DR: c_uint = 0x00	/* Data read or written from the interface. */;
pub const UART01x_RSR: c_uint = 0x04	/* Receive status register (Read). */;
pub const UART01x_ECR: c_uint = 0x04	/* Error clear register (Write). */;
pub const UART010_LCRH: c_uint = 0x08	/* Line control register, high byte. */;
pub const ST_UART011_DMAWM: c_uint = 0x08    /* DMA watermark configure register. */;
pub const UART010_LCRM: c_uint = 0x0C	/* Line control register, middle byte. */;
pub const ST_UART011_TIMEOUT: c_uint = 0x0C    /* Timeout period register. */;
pub const UART010_LCRL: c_uint = 0x10	/* Line control register, low byte. */;
pub const UART010_CR: c_uint = 0x14	/* Control register. */;
pub const UART01x_FR: c_uint = 0x18	/* Flag register (Read only). */;
pub const UART010_IIR: c_uint = 0x1C	/* Interrupt identification register (Read). */;
pub const UART010_ICR: c_uint = 0x1C	/* Interrupt clear register (Write). */;
pub const ST_UART011_LCRH_RX: c_uint = 0x1C    /* Rx line control register. */;
pub const UART01x_ILPR: c_uint = 0x20	/* IrDA low power counter register. */;
pub const UART011_IBRD: c_uint = 0x24	/* Integer baud rate divisor register. */;
pub const UART011_FBRD: c_uint = 0x28	/* Fractional baud rate divisor register. */;
pub const UART011_LCRH: c_uint = 0x2c	/* Line control register. */;
pub const ST_UART011_LCRH_TX: c_uint = 0x2c    /* Tx Line control register. */;
pub const UART011_CR: c_uint = 0x30	/* Control register. */;
pub const UART011_IFLS: c_uint = 0x34	/* Interrupt fifo level select. */;
pub const UART011_IMSC: c_uint = 0x38	/* Interrupt mask. */;
pub const UART011_RIS: c_uint = 0x3c	/* Raw interrupt status. */;
pub const UART011_MIS: c_uint = 0x40	/* Masked interrupt status. */;
pub const UART011_ICR: c_uint = 0x44	/* Interrupt clear register. */;
pub const UART011_DMACR: c_uint = 0x48	/* DMA control register. */;
pub const ST_UART011_XFCR: c_uint = 0x50	/* XON/XOFF control register. */;
pub const ST_UART011_XON1: c_uint = 0x54	/* XON1 register. */;
pub const ST_UART011_XON2: c_uint = 0x58	/* XON2 register. */;
pub const ST_UART011_XOFF1: c_uint = 0x5C	/* XON1 register. */;
pub const ST_UART011_XOFF2: c_uint = 0x60	/* XON2 register. */;
pub const ST_UART011_ITCR: c_uint = 0x80	/* Integration test control register. */;
pub const ST_UART011_ITIP: c_uint = 0x84	/* Integration test input register. */;
pub const ST_UART011_ABCR: c_uint = 0x100	/* Autobaud control register. */;
pub const ST_UART011_ABIMSC: c_uint = 0x15C	/* Autobaud interrupt mask/clear register. */;
//
// ZTE UART register offsets.  This UART has a radically different address
// allocation from the ARM and ST variants, so we list all registers here.
// We assume unlisted registers do not exist.
//
pub const ZX_UART011_DR: c_uint = 0x04;
pub const ZX_UART011_FR: c_uint = 0x14;
pub const ZX_UART011_IBRD: c_uint = 0x24;
pub const ZX_UART011_FBRD: c_uint = 0x28;
pub const ZX_UART011_LCRH: c_uint = 0x30;
pub const ZX_UART011_CR: c_uint = 0x34;
pub const ZX_UART011_IFLS: c_uint = 0x38;
pub const ZX_UART011_IMSC: c_uint = 0x40;
pub const ZX_UART011_RIS: c_uint = 0x44;
pub const ZX_UART011_MIS: c_uint = 0x48;
pub const ZX_UART011_ICR: c_uint = 0x4c;
pub const ZX_UART011_DMACR: c_uint = 0x50;

//
// Some bits of Flag Register on ZTE device have different position from
// standard ones.
//

pub const UART01x_LCRH_WLEN_8: c_uint = 0x60;
pub const UART01x_LCRH_WLEN_7: c_uint = 0x40;
pub const UART01x_LCRH_WLEN_6: c_uint = 0x20;
pub const UART01x_LCRH_WLEN_5: c_uint = 0x00;

// special values for ST vendor with deeper fifo

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amba_pl010_data {
    pub mctrl): *mut *mut *mut *mut void (set_mctrl)(struct amba_device dev, void __iomem base, unsigned int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amba_pl011_data {
    pub filter_param): *mut *mut *mut bool (dma_filter)(struct dma_chan chan, void,
    pub dma_rx_param: *mut c_void,
    pub dma_tx_param: *mut c_void,
    pub dma_rx_poll_enable: bool,
    pub dma_rx_poll_rate: c_uint,
    pub dma_rx_poll_timeout: c_uint,
    pub (*init)(void): *mut c_void,
    pub (*exit)(void): *mut c_void,
}

