//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/tty/serial/8250/8250_dwlib.h
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


// SPDX-License-Identifier: GPL-2.0+
// Synopsys DesignWare 8250 library header file.

// Offsets for the DesignWare specific registers
pub const DW_UART_USR: c_uint = 0x1f /* UART Status Register */;
pub const DW_UART_DMASA: c_uint = 0xa8 /* DMA Software Ack */;
pub const DW_UART_TCR: c_uint = 0xac /* Transceiver Control Register (RS485) */;
pub const DW_UART_DE_EN: c_uint = 0xb0 /* Driver Output Enable Register */;
pub const DW_UART_RE_EN: c_uint = 0xb4 /* Receiver Output Enable Register */;
pub const DW_UART_DLF: c_uint = 0xc0 /* Divisor Latch Fraction Register */;
pub const DW_UART_RAR: c_uint = 0xc4 /* Receive Address Register */;
pub const DW_UART_TAR: c_uint = 0xc8 /* Transmit Address Register */;
pub const DW_UART_LCR_EXT: c_uint = 0xcc /* Line Extended Control Register */;
pub const DW_UART_CPR: c_uint = 0xf4 /* Component Parameter Register */;
pub const DW_UART_UCV: c_uint = 0xf8 /* UART Component Version */;
// Interrupt ID Register bits

// Modem Control Register bits

// Line Status Register bits

// UART Status Register bits

// Transceiver Control Register bits

// Receive / Transmit Address Register bits

// Line Extended Control Register bits

// Component Parameter Register bits

// Helpers for FIFO size calculation

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw8250_port_data {
// Port properties
    pub line: c_int,
// DMA operations
    pub dma: uart_8250_dma,
// Hardware configuration
    pub cpr_value: u32,
    pub dlf_size: u8,
// RS485 variables
    pub hw_rs485_support: bool,
}

extern "C" {
    pub fn dw8250_do_set_termios(p: *mut uart_port, termios: *mut ktermios, old: *const ktermios);
}
extern "C" {
    pub fn dw8250_setup_port(p: *mut uart_port);
}
extern "C" {
    pub fn ioread32be(offset: p->membase +) -> return;
}
extern "C" {
    pub fn readl(offset: p->membase +) -> return;
}
