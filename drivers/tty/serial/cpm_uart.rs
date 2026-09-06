//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/tty/serial/cpm_uart.h
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
// Driver for CPM (SCC/SMC) serial ports
//
// Copyright (C) 2004 Freescale Semiconductor, Inc.
//
// 2006 (c) MontaVista Software, Inc.
// Vitaly Bordug <vbordug@ru.mvista.com>
//

pub const SERIAL_CPM_MAJOR: c_int = 204;
pub const SERIAL_CPM_MINOR: c_int = 46;

pub const FLAG_SMC: c_uint = 0x00000002;
pub const FLAG_CONSOLE: c_uint = 0x00000001;
pub const UART_NR: c_int = 6;
pub const RX_NUM_FIFO: c_int = 4;
pub const RX_BUF_SIZE: c_int = 32;
pub const TX_NUM_FIFO: c_int = 4;
pub const TX_BUF_SIZE: c_int = 32;
pub const GPIO_CTS: c_int = 0;
pub const GPIO_RTS: c_int = 1;
pub const GPIO_DCD: c_int = 2;
pub const GPIO_DSR: c_int = 3;
pub const GPIO_DTR: c_int = 4;
pub const GPIO_RI: c_int = 5;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uart_cpm_port {
    pub port: uart_port,
    pub rx_nrfifos: u16,
    pub rx_fifosize: u16,
    pub tx_nrfifos: u16,
    pub tx_fifosize: u16,
    pub smcp: *mut smc_t __iomem,
    pub smcup: *mut smc_uart_t __iomem,
    pub sccp: *mut scc_t __iomem,
    pub sccup: *mut scc_uart_t __iomem,
    pub rx_bd_base: *mut cbd_t __iomem,
    pub rx_cur: *mut cbd_t __iomem,
    pub tx_bd_base: *mut cbd_t __iomem,
    pub tx_cur: *mut cbd_t __iomem,
    pub tx_buf: *mut c_uchar,
    pub rx_buf: *mut c_uchar,
    pub flags: u32,
    pub clk: *mut clk,
    pub brg: u8,
    pub dp_addr: c_uint,
    pub mem_addr: *mut c_void,
    pub dma_addr: dma_addr_t,
    pub mem_size: u32,
// wait on close if needed
    pub wait_closing: c_int,
// value to combine with opcode to form cpm command
    pub command: u32,
    pub gpios: [*mut gpio_desc; NUM_GPIOS],
}

//
// sane check
// something nasty happened
// sane check
// something nasty happened
