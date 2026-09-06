//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/char/tpm/tpm_ftpm_tee.h
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
// Copyright (C) Microsoft Corporation
//

// The TAFs ID implemented in this TA

// max. buffer size supported by fTPM
pub const MAX_COMMAND_SIZE: c_int = 4096;
pub const MAX_RESPONSE_SIZE: c_int = 4096;
//
// struct ftpm_tee_private - fTPM's private data
// @chip:     struct tpm_chip instance registered with tpm framework.
// @session:  fTPM TA session identifier.
// @ctx:      TEE context handler.
// @shm:      Memory pool shared with fTPM TA in TEE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftpm_tee_private {
    pub chip: *mut tpm_chip,
    pub session: u32,
    pub ctx: *mut tee_context,
    pub shm: *mut tee_shm,
}
