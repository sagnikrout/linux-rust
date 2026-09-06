//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/spi/mmc_spi.h
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

// Put this in platform_data of a device being used to manage an MMC/SD
// card slot.  (Modeled after PXA mmc glue; see that for usage examples.)
//
// REVISIT This is not a spi-specific notion.  Any card slot should be
// able to handle it.  If the MMC core doesn't adopt this kind of notion,
// switch the "struct device *" parameters over to "struct spi_device *".
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmc_spi_platform_data {
// driver activation and (optional) card detect irq hookup
    pub ): *mut c_void,
    pub ): *mut *mut *mut void (exit)(struct device , void,
// Capabilities to pass into mmc core (e.g. MMC_CAP_NEEDS_POLL).
    pub caps: c_ulong,
    pub caps2: c_ulong,
// how long to debounce card detect, in msecs
    pub detect_delay: u16,
// power management
    pub /: *mut *mut u16 powerup_msecs; / delay of up to 250 msec,
    pub /: *mut *mut u32 ocr_mask; / available voltages,
    pub maskval): *mut *mut *mut void (setpower)(struct device , unsigned int,
}

extern "C" {
    pub fn mmc_spi_put_pdata(spi: *mut spi_device);
}
