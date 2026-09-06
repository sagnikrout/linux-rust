//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/spi/dosilicon.c
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
// Author: Ahmed Naseef <naseefkm@gmail.com>
//

pub const SPINAND_MFR_DOSILICON: c_uint = 0xE5;
    static SPINAND_OP_VARIANTS(read_cache_variants,
    SPINAND_PAGE_READ_FROM_CACHE_1S_1S_4S_OP(0, 1, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_1S_1S_2S_OP(0, 1, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_FAST_1S_1S_1S_OP(0, 1, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_1S_1S_1S_OP(0, 1, core::ptr::null_mut(), 0, 0));
    static SPINAND_OP_VARIANTS(write_cache_variants,
    SPINAND_PROG_LOAD_1S_1S_4S_OP(true, 0, core::ptr::null_mut(), 0),
    SPINAND_PROG_LOAD_1S_1S_1S_OP(true, 0, core::ptr::null_mut(), 0));
    static SPINAND_OP_VARIANTS(update_cache_variants,
    SPINAND_PROG_LOAD_1S_1S_4S_OP(false, 0, core::ptr::null_mut(), 0),
    SPINAND_PROG_LOAD_1S_1S_1S_OP(false, 0, core::ptr::null_mut(), 0));
    static int ds35xx_ooblayout_ecc(struct mtd_info *mtd, int section,
    struct mtd_oob_region *region)
    {
    if (section > 3)
    return -ERANGE;
    region.offset = 8 + (section * 16);
    region.length = 8;
    return 0;
    }
    static int ds35xx_ooblayout_free(struct mtd_info *mtd, int section,
    struct mtd_oob_region *region)
    {
    if (section > 3)
    return -ERANGE;
    if (section == 0) {
// reserve 2 bytes for the BBM
    region.offset = 2;
    region.length = 6;
    } else {
    region.offset = section * 16;
    region.length = 8;
    }
    return 0;
    }
    static const struct mtd_ooblayout_ops ds35xx_ooblayout = {
    .ecc = ds35xx_ooblayout_ecc,
    .free = ds35xx_ooblayout_free,
    };
    static const struct spinand_info dosilicon_spinand_table[] = {
    SPINAND_INFO("DS35Q1GA",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x71),
    NAND_MEMORG(1, 2048, 64, 64, 1024, 20, 1, 1, 1),
    NAND_ECCREQ(4, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&ds35xx_ooblayout, core::ptr::null_mut())),
    SPINAND_INFO("DS35M1GA",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x21),
    NAND_MEMORG(1, 2048, 64, 64, 1024, 20, 1, 1, 1),
    NAND_ECCREQ(4, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&ds35xx_ooblayout, core::ptr::null_mut())),
    };
    static const struct spinand_manufacturer_ops dosilicon_spinand_manuf_ops = {
    };
    const struct spinand_manufacturer dosilicon_spinand_manufacturer = {
    .id = SPINAND_MFR_DOSILICON,
    .name = "Dosilicon",
    .chips = dosilicon_spinand_table,
    .nchips = ARRAY_SIZE(dosilicon_spinand_table),
    .ops = &dosilicon_spinand_manuf_ops,
    };
