//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/spi/ato.c
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
// Copyright (C) 2022 Aidan MacDonald
//
// Author: Aidan MacDonald <aidanmacdonald.0x0@gmail.com>
//

pub const SPINAND_MFR_ATO: c_uint = 0x9b;
    static SPINAND_OP_VARIANTS(read_cache_variants,
    SPINAND_PAGE_READ_FROM_CACHE_1S_1S_4S_OP(0, 1, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_FAST_1S_1S_1S_OP(0, 1, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_1S_1S_1S_OP(0, 1, core::ptr::null_mut(), 0, 0));
    static SPINAND_OP_VARIANTS(write_cache_variants,
    SPINAND_PROG_LOAD_1S_1S_4S_OP(true, 0, core::ptr::null_mut(), 0),
    SPINAND_PROG_LOAD_1S_1S_1S_OP(true, 0, core::ptr::null_mut(), 0));
    static SPINAND_OP_VARIANTS(update_cache_variants,
    SPINAND_PROG_LOAD_1S_1S_4S_OP(false, 0, core::ptr::null_mut(), 0),
    SPINAND_PROG_LOAD_1S_1S_1S_OP(false, 0, core::ptr::null_mut(), 0));
    static int ato25d1ga_ooblayout_ecc(struct mtd_info *mtd, int section,
    struct mtd_oob_region *region)
    {
    if (section > 3)
    return -ERANGE;
    region.offset = (16 * section) + 8;
    region.length = 8;
    return 0;
    }
    static int ato25d1ga_ooblayout_free(struct mtd_info *mtd, int section,
    struct mtd_oob_region *region)
    {
    if (section > 3)
    return -ERANGE;
    if (section) {
    region.offset = (16 * section);
    region.length = 8;
    } else {
// first byte of section 0 is reserved for the BBM
    region.offset = 1;
    region.length = 7;
    }
    return 0;
    }
    static const struct mtd_ooblayout_ops ato25d1ga_ooblayout = {
    .ecc = ato25d1ga_ooblayout_ecc,
    .free = ato25d1ga_ooblayout_free,
    };
    static const struct spinand_info ato_spinand_table[] = {
    SPINAND_INFO("ATO25D1GA",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_ADDR, 0x12),
    NAND_MEMORG(1, 2048, 64, 64, 1024, 20, 1, 1, 1),
    NAND_ECCREQ(1, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&ato25d1ga_ooblayout, core::ptr::null_mut())),
    };
    static const struct spinand_manufacturer_ops ato_spinand_manuf_ops = {
    };
    const struct spinand_manufacturer ato_spinand_manufacturer = {
    .id = SPINAND_MFR_ATO,
    .name = "ATO",
    .chips = ato_spinand_table,
    .nchips = ARRAY_SIZE(ato_spinand_table),
    .ops = &ato_spinand_manuf_ops,
    };
