//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/tmio.h
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

// TMIO MMC platform flags
//
// Some controllers can support a 2-byte block size when the bus width is
// configured in 4-bit mode.
//

// Some controllers can support SDIO IRQ signalling

// Some features are only available or tested on R-Car Gen2 or later

//
// Some controllers require waiting for the SD bus to become idle before
// writing to some registers.
//

//
// Use the busy timeout feature. Probably all TMIO versions support it. Yet,
// we don't have documentation for old variants, so we enable only known good
// variants with this flag. Can be removed once all variants are known good.
//

// Some controllers have CMD12 automatically issue/non-issue register

// Controller has some SDIO status bits which must be 1

// Some controllers have a 32-bit wide data port register

// Some controllers allows to set SDx actual clock

// Some controllers have a CBSY bit

// Some controllers have a 64-bit wide data port register

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmio_mmc_data {
    pub chan_priv_tx: *mut c_void,
    pub chan_priv_rx: *mut c_void,
    pub hclk: c_uint,
    pub capabilities: c_ulong,
    pub capabilities2: c_ulong,
    pub flags: c_ulong,
    pub /: *mut *mut u32 ocr_mask; / available voltages,
    pub dma_rx_offset: dma_addr_t,
    pub max_blk_count: c_uint,
    pub max_segs: c_ushort,
}
