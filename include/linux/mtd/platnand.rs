//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mtd/platnand.h
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
// Copyright © 2000-2010 David Woodhouse <dwmw2@infradead.org>
// Steven J. Hill <sjhill@realitydiluted.com>
// Thomas Gleixner <tglx@kernel.org>
//
// Contains all platform NAND related definitions.
//

//
// struct platform_nand_chip - chip level device structure
// @nr_chips: max. number of chips to scan for
// @chip_offset: chip number offset
// @nr_partitions: number of partitions pointed to by partitions (or zero)
// @partitions: mtd partition list
// @chip_delay: R/B delay value in us
// @options: Option flags, e.g. 16bit buswidth
// @bbt_options: BBT option flags, e.g. NAND_BBT_USE_FLASH
// @part_probe_types: NULL-terminated array of probe types
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct platform_nand_chip {
    pub nr_chips: c_int,
    pub chip_offset: c_int,
    pub nr_partitions: c_int,
    pub partitions: *mut mtd_partition,
    pub chip_delay: c_int,
    pub options: c_uint,
    pub bbt_options: c_uint,
    pub part_probe_types: *const c_char,
}

//
// struct platform_nand_ctrl - controller level device structure
// @probe: platform specific function to probe/setup hardware
// @remove: platform specific function to remove/teardown hardware
// @dev_ready: platform specific function to read ready/busy pin
// @select_chip: platform specific chip select function
// @cmd_ctrl: platform specific function for controlling
// ALE/CLE/nCE. Also used to write command and address
// @write_buf: platform specific function for write buffer
// @read_buf: platform specific function for read buffer
// @priv: private data to transport driver specific settings
//
// All fields are optional and depend on the hardware driver requirements
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct platform_nand_ctrl {
    pub pdev): *mut *mut int (probe)(struct platform_device,
    pub pdev): *mut *mut void (remove)(struct platform_device,
    pub chip): *mut *mut int (dev_ready)(struct nand_chip,
    pub cs): *mut *mut *mut void (select_chip)(struct nand_chip chip, int,
    pub ctrl): *mut *mut *mut void (cmd_ctrl)(struct nand_chip chip, int dat, unsigned int,
    pub len): *const *const *const *const void (write_buf)(struct nand_chip chip, uint8_t buf, int,
    pub len): *mut *mut *mut *mut void (read_buf)(struct nand_chip chip, uint8_t buf, int,
    pub priv: *mut c_void,
}

//
// struct platform_nand_data - container structure for platform-specific data
// @chip: chip level chip structure
// @ctrl: controller level device structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct platform_nand_data {
    pub chip: platform_nand_chip,
    pub ctrl: platform_nand_ctrl,
}
