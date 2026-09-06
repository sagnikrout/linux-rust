//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/nfc/st95hf/spi.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// ---------------------------------------------------------------------------
// drivers/nfc/st95hf/spi.h functions declarations for SPI communication
// ---------------------------------------------------------------------------
// Copyright (C) 2015 STMicroelectronics – All Rights Reserved
//

// Basic ST95HF SPI CMDs
pub const ST95HF_COMMAND_SEND: c_uint = 0x0;
pub const ST95HF_COMMAND_RESET: c_uint = 0x1;
pub const ST95HF_COMMAND_RECEIVE: c_uint = 0x2;
pub const ST95HF_RESET_CMD_LEN: c_uint = 0x1;
//
// structure to contain st95hf spi communication specific information.
// @req_issync: true for synchronous calls.
// @spidev: st95hf spi device object.
// @done: completion structure to wait for st95hf response
// for synchronous calls.
// @spi_lock: mutex to allow only one spi transfer at a time.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st95hf_spi_context {
    pub req_issync: bool,
    pub spidev: *mut spi_device,
    pub done: completion,
    pub spi_lock: mutex,
}

// flag to differentiate synchronous & asynchronous spi request
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum req_type {
    SYNC,
    ASYNC,
}
