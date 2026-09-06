//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mmc/sdio_func.h
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
// include/linux/mmc/sdio_func.h
//
// Copyright 2007-2008 Pierre Ossman
//

extern "C" {
    pub fn void(: *mut sdio_irq_handler_t)(struct sdio_func) -> typedef;
}
//
// SDIO function CIS tuple (unknown to the core)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdio_func_tuple {
    pub next: *mut sdio_func_tuple,
    pub code: c_uchar,
    pub size: c_uchar,
    pub data: [c_uchar; ],
}

//
// SDIO function devices
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdio_func {
    pub /: *mut *mut *mut mmc_card card; / the card this device belongs to,
    pub /: *mut *mut device dev; / the device,
    pub /: *mut *mut *mut sdio_irq_handler_t irq_handler; / IRQ callback,
    pub /: *mut *mut unsigned int num; / function number,
    pub /: *mut *mut unsigned char class; / standard interface class,
    pub /: *mut *mut unsigned short vendor; / vendor id,
    pub /: *mut *mut unsigned short device; / device id,
    pub /: *mut *mut unsigned max_blksize; / maximum block size,
    pub /: *mut *mut unsigned cur_blksize; / current block size,
    pub /: *mut *mut unsigned enable_timeout; / max enable timeout in msec,
    pub /: *mut *mut unsigned int state; / function state,

    pub /: *mut *mut *mut u8 tmpbuf; / DMA:able scratch buffer,
    pub /: *mut *mut u8 major_rev; / major revision number,
    pub /: *mut *mut u8 minor_rev; / minor revision number,
    pub /: *mut *mut unsigned num_info; / number of info strings,
    pub /: *const *const *const *const char info; / info strings,
    pub tuples: *mut sdio_func_tuple,
}

//
// SDIO function device driver
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdio_driver {
    pub name: *mut c_char,
    pub id_table: *const sdio_device_id,
    pub ): *const *const *const int (probe)(struct sdio_func , struct sdio_device_id,
    pub ): *mut *mut void (remove)(struct sdio_func,
    pub ): *mut *mut void (shutdown)(struct sdio_func,
    pub drv: device_driver,
}

//
// SDIO_DEVICE - macro used to describe a specific SDIO device
// @vend: the 16 bit manufacturer code
// @dev: the 16 bit function id
//
// This macro is used to create a struct sdio_device_id that matches a
// specific device. The class field will be set to SDIO_ANY_ID.
//

//
// SDIO_DEVICE_CLASS - macro used to describe a specific SDIO device class
// @dev_class: the 8 bit standard interface code
//
// This macro is used to create a struct sdio_device_id that matches a
// specific standard SDIO function type.  The vendor and device fields will
// be set to SDIO_ANY_ID.
//

// use a macro to avoid include chaining to get THIS_MODULE

extern "C" {
    pub fn __sdio_register_driver(: *mut sdio_driver, : *mut module) -> c_int;
}
extern "C" {
    pub fn sdio_unregister_driver(: *mut sdio_driver);
}
//
// module_sdio_driver() - Helper macro for registering a SDIO driver
// @__sdio_driver: sdio_driver struct
//
// Helper macro for SDIO drivers which do not do anything special in module
// init/exit. This eliminates a lot of boilerplate. Each module may only
// use this macro once, and calling it replaces module_init() and module_exit()
//

//
// SDIO I/O operations
//
extern "C" {
    pub fn sdio_claim_host(func: *mut sdio_func);
}
extern "C" {
    pub fn sdio_release_host(func: *mut sdio_func);
}
extern "C" {
    pub fn sdio_enable_func(func: *mut sdio_func) -> c_int;
}
extern "C" {
    pub fn sdio_disable_func(func: *mut sdio_func) -> c_int;
}
extern "C" {
    pub fn sdio_set_block_size(func: *mut sdio_func, blksz: unsigned) -> c_int;
}
extern "C" {
    pub fn sdio_claim_irq(func: *mut sdio_func, handler: *mut sdio_irq_handler_t) -> c_int;
}
extern "C" {
    pub fn sdio_release_irq(func: *mut sdio_func) -> c_int;
}
extern "C" {
    pub fn sdio_align_size(func: *mut sdio_func, sz: c_uint) -> c_uint;
}
extern "C" {
    pub fn sdio_readb(func: *mut sdio_func, addr: c_uint, err_ret: *mut c_int) -> u8;
}
extern "C" {
    pub fn sdio_readw(func: *mut sdio_func, addr: c_uint, err_ret: *mut c_int) -> u16;
}
extern "C" {
    pub fn sdio_readl(func: *mut sdio_func, addr: c_uint, err_ret: *mut c_int) -> u32;
}
extern "C" {
    pub fn sdio_get_host_pm_caps(func: *mut sdio_func) -> mmc_pm_flag_t;
}
extern "C" {
    pub fn sdio_set_host_pm_flags(func: *mut sdio_func, flags: mmc_pm_flag_t) -> c_int;
}
extern "C" {
    pub fn sdio_retune_crc_disable(func: *mut sdio_func);
}
extern "C" {
    pub fn sdio_retune_crc_enable(func: *mut sdio_func);
}
extern "C" {
    pub fn sdio_retune_hold_now(func: *mut sdio_func);
}
extern "C" {
    pub fn sdio_retune_release(func: *mut sdio_func);
}
