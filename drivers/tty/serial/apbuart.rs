//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/tty/serial/apbuart.h
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

pub const UART_NR: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct grlib_apbuart_regs_map {
    pub data: u32,
    pub status: u32,
    pub ctrl: u32,
    pub scaler: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amba_prom_registers {
    pub phys_addr: c_uint,
    pub reg_size: c_uint,
}

//
// The following defines the bits in the APBUART Status Registers.
//
pub const UART_STATUS_DR: c_uint = 0x00000001	/* Data Ready */;
pub const UART_STATUS_TSE: c_uint = 0x00000002	/* TX Send Register Empty */;
pub const UART_STATUS_THE: c_uint = 0x00000004	/* TX Hold Register Empty */;
pub const UART_STATUS_BR: c_uint = 0x00000008	/* Break Error */;
pub const UART_STATUS_OE: c_uint = 0x00000010	/* RX Overrun Error */;
pub const UART_STATUS_PE: c_uint = 0x00000020	/* RX Parity Error */;
pub const UART_STATUS_FE: c_uint = 0x00000040	/* RX Framing Error */;
pub const UART_STATUS_ERR: c_uint = 0x00000078	/* Error Mask */;
//
// The following defines the bits in the APBUART Ctrl Registers.
//
pub const UART_CTRL_RE: c_uint = 0x00000001	/* Receiver enable */;
pub const UART_CTRL_TE: c_uint = 0x00000002	/* Transmitter enable */;
pub const UART_CTRL_RI: c_uint = 0x00000004	/* Receiver interrupt enable */;
pub const UART_CTRL_TI: c_uint = 0x00000008	/* Transmitter irq */;
pub const UART_CTRL_PS: c_uint = 0x00000010	/* Parity select */;
pub const UART_CTRL_PE: c_uint = 0x00000020	/* Parity enable */;
pub const UART_CTRL_FL: c_uint = 0x00000040	/* Flow control enable */;
pub const UART_CTRL_LB: c_uint = 0x00000080	/* Loopback enable */;

