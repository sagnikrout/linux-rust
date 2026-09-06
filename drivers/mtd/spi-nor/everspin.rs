//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/spi-nor/everspin.c
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
// Copyright (C) 2005, Intec Automation Inc.
// Copyright (C) 2014, Freescale Semiconductor, Inc.
//

    static const struct flash_info everspin_nor_parts[] = {
    {
    .name = "mr25h128",
    .size = SZ_16K,
    .sector_size = SZ_16K,
    .addr_nbytes = 2,
    .flags = SPI_NOR_NO_ERASE,
    }, {
    .name = "mr25h256",
    .size = SZ_32K,
    .sector_size = SZ_32K,
    .addr_nbytes = 2,
    .flags = SPI_NOR_NO_ERASE,
    }, {
    .name = "mr25h10",
    .size = SZ_128K,
    .sector_size = SZ_128K,
    .flags = SPI_NOR_NO_ERASE,
    }, {
    .name = "mr25h40",
    .size = SZ_512K,
    .sector_size = SZ_512K,
    .flags = SPI_NOR_NO_ERASE,
    }
    };
#[no_mangle]
unsafe extern "C" fn everspin_nor_default_init(nor: *mut spi_nor) {
    static void everspin_nor_default_init(struct spi_nor *nor)
    {
// Everspin FRAMs don't support the fast read opcode.
    nor.params.hwcaps.mask &= ~SNOR_HWCAPS_READ_FAST;
    }
    static const struct spi_nor_fixups everspin_nor_fixups = {
    .default_init = everspin_nor_default_init,
    };
    const struct spi_nor_manufacturer spi_nor_everspin = {
    .name = "everspin",
    .parts = everspin_nor_parts,
    .nparts = ARRAY_SIZE(everspin_nor_parts),
    .fixups = &everspin_nor_fixups,
    };
