//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/tty/serial/timbuart.h
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
// timbuart.c timberdale FPGA GPIO driver
// Copyright (c) 2009 Intel Corporation
//
// Supports:
// Timberdale FPGA UART
//
pub const TIMBUART_FIFO_SIZE: c_int = 2048;
pub const TIMBUART_RXFIFO: c_uint = 0x08;
pub const TIMBUART_TXFIFO: c_uint = 0x0c;
pub const TIMBUART_IER: c_uint = 0x10;
pub const TIMBUART_IPR: c_uint = 0x14;
pub const TIMBUART_ISR: c_uint = 0x18;
pub const TIMBUART_CTRL: c_uint = 0x1c;
pub const TIMBUART_BAUDRATE: c_uint = 0x20;
pub const TIMBUART_CTRL_RTS: c_uint = 0x01;
pub const TIMBUART_CTRL_CTS: c_uint = 0x02;
pub const TIMBUART_CTRL_FLSHTX: c_uint = 0x40;
pub const TIMBUART_CTRL_FLSHRX: c_uint = 0x80;
pub const TXBF: c_uint = 0x01;
pub const TXBAE: c_uint = 0x02;
pub const CTS_DELTA: c_uint = 0x04;
pub const RXDP: c_uint = 0x08;
pub const RXBAF: c_uint = 0x10;
pub const RXBF: c_uint = 0x20;
pub const RXTT: c_uint = 0x40;
pub const RXBNAE: c_uint = 0x80;
pub const TXBE: c_uint = 0x100;

pub const TIMBUART_MAJOR: c_int = 204;
pub const TIMBUART_MINOR: c_int = 192;
