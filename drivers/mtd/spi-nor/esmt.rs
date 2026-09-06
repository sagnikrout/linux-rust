//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/spi-nor/esmt.c
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

    static const struct flash_info esmt_nor_parts[] = {
    {
    .id = SNOR_ID(0x8c, 0x20, 0x16),
    .name = "f25l32pa",
    .size = SZ_4M,
    .flags = SPI_NOR_HAS_LOCK | SPI_NOR_SWP_IS_VOLATILE,
    .no_sfdp_flags = SECT_4K,
    }, {
    .id = SNOR_ID(0x8c, 0x41, 0x16),
    .name = "f25l32qa-2s",
    .size = SZ_4M,
    .flags = SPI_NOR_HAS_LOCK,
    .no_sfdp_flags = SECT_4K,
    }, {
    .id = SNOR_ID(0x8c, 0x41, 0x17),
    .name = "f25l64qa",
    .size = SZ_8M,
    .flags = SPI_NOR_HAS_LOCK,
    .no_sfdp_flags = SECT_4K,
    }
    };
    const struct spi_nor_manufacturer spi_nor_esmt = {
    .name = "esmt",
    .parts = esmt_nor_parts,
    .nparts = ARRAY_SIZE(esmt_nor_parts),
    };
