//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/tty/serial/8250/8250.h
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
//
// Driver for 8250/16550-type serial ports
//
// Based on drivers/char/serial.c, by Linus Torvalds, Theodore Ts'o.
//
// Copyright (C) 2001 Russell King.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uart_8250_dma {
    pub p): *mut *mut int (tx_dma)(struct uart_8250_port,
    pub p): *mut *mut int (rx_dma)(struct uart_8250_port,
    pub p): *mut *mut void (prepare_tx_dma)(struct uart_8250_port,
    pub p): *mut *mut void (prepare_rx_dma)(struct uart_8250_port,
// Filter function
    pub fn: dma_filter_fn,
// Parameter to the filter function
    pub rx_param: *mut c_void,
    pub tx_param: *mut c_void,
    pub rxconf: dma_slave_config,
    pub txconf: dma_slave_config,
    pub rxchan: *mut dma_chan,
    pub txchan: *mut dma_chan,
// Device address base for DMA operations
    pub rx_dma_addr: phys_addr_t,
    pub tx_dma_addr: phys_addr_t,
// DMA address of the buffer in memory
    pub rx_addr: dma_addr_t,
    pub tx_addr: dma_addr_t,
    pub rx_cookie: dma_cookie_t,
    pub tx_cookie: dma_cookie_t,
    pub rx_buf: *mut c_void,
    pub rx_size: usize,
    pub tx_size: usize,
    pub tx_running: c_uchar,
    pub tx_err: c_uchar,
    pub rx_running: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct old_serial_port {
    pub uart: c_uint,
    pub baud_base: c_uint,
    pub port: c_uint,
    pub irq: c_uint,
    pub flags: upf_t,
    pub io_type: c_uchar,
    pub iomem_base: *mut unsigned char __iomem,
    pub iomem_reg_shift: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct serial8250_config {
    pub name: *const c_char,
    pub fifo_size: c_ushort,
    pub tx_loadsz: c_ushort,
    pub fcr: c_uchar,
    pub rxtrig_bytes: [c_uchar; UART_FCR_R_TRIG_MAX_STATE],
    pub flags: c_uint,
}

// STOP PARITY EPAR SPAR WLEN5 WLEN6
//

// Module parameters

extern "C" {
    pub fn serial8250_register_ports(drv: *mut uart_driver, dev: *mut device);
}
// Legacy ISA bus related APIs
extern "C" {
    pub fn void(_arg: *mut serial8250_isa_config_fn)(int, : *mut uart_port, : *mut u32) -> typedef;
}
extern "C" {
    pub fn serial8250_isa_init_ports();
}
//
// serial_lsr_in - Read LSR register and preserve flags across reads
// @up:	uart 8250 port
//
// Read LSR register and handle saving non-preserved flags across reads.
// The flags that are not preserved across reads are stored into
// up->lsr_saved_flags.
//
// Returns LSR value or'ed with the preserved flags (if any).
//
// For the 16C950
//
extern "C" {
    pub fn serial8250_clear_fifos(p: *mut uart_8250_port);
}
extern "C" {
    pub fn serial8250_clear_and_reinit_fifos(p: *mut uart_8250_port);
}
extern "C" {
    pub fn serial8250_rpm_get(p: *mut uart_8250_port);
}
extern "C" {
    pub fn serial8250_rpm_put(p: *mut uart_8250_port);
}
// Port locked to synchronize UART_IER access against the console.
extern "C" {
    pub fn serial8250_em485_start_tx(p: *mut uart_8250_port, toggle_ier: bool);
}
extern "C" {
    pub fn serial8250_em485_stop_tx(p: *mut uart_8250_port, toggle_ier: bool);
}
extern "C" {
    pub fn serial8250_em485_destroy(p: *mut uart_8250_port);
}
// MCR <-> TIOCM conversion
// MSR <-> TIOCM conversion

extern "C" {
    pub fn serial8250_pnp_init() -> c_int;
}
extern "C" {
    pub fn serial8250_pnp_exit();
}

extern "C" {
    pub fn univ8250_rsa_support(ops: *mut uart_ops, core_ops: *const uart_ops);
}
extern "C" {
    pub fn rsa_enable(up: *mut uart_8250_port);
}
extern "C" {
    pub fn rsa_disable(up: *mut uart_8250_port);
}
extern "C" {
    pub fn rsa_autoconfig(up: *mut uart_8250_port);
}
extern "C" {
    pub fn rsa_reset(up: *mut uart_8250_port);
}

extern "C" {
    pub fn fintek_8250_probe(uart: *mut uart_8250_port) -> c_int;
}

extern "C" {
    pub fn hub6_match_port(port1: *const uart_port, port2: *const uart_port) -> bool;
}

extern "C" {
    pub fn is_omap1_8250(_arg: pt) -> return;
}

extern "C" {
    pub fn serial8250_tx_dma(: *mut uart_8250_port) -> c_int;
}
extern "C" {
    pub fn serial8250_tx_dma_flush(: *mut uart_8250_port);
}
extern "C" {
    pub fn serial8250_rx_dma(: *mut uart_8250_port) -> c_int;
}
extern "C" {
    pub fn serial8250_rx_dma_flush(: *mut uart_8250_port);
}
extern "C" {
    pub fn serial8250_request_dma(: *mut uart_8250_port) -> c_int;
}
extern "C" {
    pub fn serial8250_release_dma(: *mut uart_8250_port);
}

// already in high speed mode
