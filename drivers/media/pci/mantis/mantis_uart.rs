//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/mantis/mantis_uart.h
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
pub const MANTIS_UART_CTL: c_uint = 0xe0;

pub const MANTIS_UART_RXD: c_uint = 0xe8;
pub const MANTIS_UART_BAUD: c_uint = 0xec;
pub const MANTIS_UART_STAT: c_uint = 0xf0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mantis_baud {
    MANTIS_BAUD_9600	= 0,
    MANTIS_BAUD_19200,
    MANTIS_BAUD_38400,
    MANTIS_BAUD_57600,
    MANTIS_BAUD_115200
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mantis_parity {
    MANTIS_PARITY_NONE	= 0,
    MANTIS_PARITY_EVEN,
    MANTIS_PARITY_ODD,
}

extern "C" {
    pub fn mantis_uart_init(mantis: *mut mantis_pci) -> c_int;
}
extern "C" {
    pub fn mantis_uart_exit(mantis: *mut mantis_pci);
}
