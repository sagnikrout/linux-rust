//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/pxa3xx-gcu.h
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

// Number of 32bit words in display list (ring buffer).

// To be increased when breaking the ABI
pub const PXA3XX_GCU_SHARED_MAGIC: c_uint = 0x30000001;
pub const PXA3XX_GCU_BATCH_WORDS: c_int = 8192;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxa3xx_gcu_shared {
    pub buffer: [u32; PXA3XX_GCU_BUFFER_WORDS],
    pub hw_running: bool,
    pub buffer_phys: c_ulong,
    pub num_words: c_uint,
    pub num_writes: c_uint,
    pub num_done: c_uint,
    pub num_interrupts: c_uint,
    pub num_wait_idle: c_uint,
    pub num_wait_free: c_uint,
    pub num_idle: c_uint,
    pub magic: u32,
}

// Initialization and synchronization.
// Hardware is started upon write().

