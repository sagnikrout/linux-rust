//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/serial.h
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


//
// include/linux/serial.h
//
// Copyright (C) 1992 by Theodore Ts'o.
//
// Redistribution of this file is permitted under the terms of the GNU
// Public License (GPL)
//

// Helper for dealing with UART_LCR_WLEN* defines

// FIFO and shifting register empty

//
// Counters of the input lines (CTS, DSR, RI, CD) interrupts
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct async_icount {
    pub rx: __u32 cts, dsr, rng, dcd, tx,,
    pub brk: __u32 frame, parity, overrun,,
    pub buf_overrun: __u32,
}

