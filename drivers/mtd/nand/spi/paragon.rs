//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/spi/paragon.c
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
// Copyright (C) 2019 Jeff Kletsky
//
// Author: Jeff Kletsky <git-commits@allycomm.com>
//

pub const SPINAND_MFR_PARAGON: c_uint = 0xa1;

    static SPINAND_OP_VARIANTS(read_cache_variants,
    SPINAND_PAGE_READ_FROM_CACHE_1S_4S_4S_OP(0, 2, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_1S_1S_4S_OP(0, 1, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_1S_2S_2S_OP(0, 1, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_1S_1S_2S_OP(0, 1, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_FAST_1S_1S_1S_OP(0, 1, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_1S_1S_1S_OP(0, 1, core::ptr::null_mut(), 0, 0));
    static SPINAND_OP_VARIANTS(write_cache_variants,
    SPINAND_PROG_LOAD_1S_1S_4S_OP(true, 0, core::ptr::null_mut(), 0),
    SPINAND_PROG_LOAD_1S_1S_1S_OP(true, 0, core::ptr::null_mut(), 0));
    static SPINAND_OP_VARIANTS(update_cache_variants,
    SPINAND_PROG_LOAD_1S_1S_4S_OP(false, 0, core::ptr::null_mut(), 0),
    SPINAND_PROG_LOAD_1S_1S_1S_OP(false, 0, core::ptr::null_mut(), 0));
    static int pn26g0xa_ooblayout_ecc(struct mtd_info *mtd, int section,
    struct mtd_oob_region *region)
    {
    if (section > 3)
    return -ERANGE;
    region.offset = 6 + (15 * section); /* 4 BBM + 2 user bytes */
    region.length = 13;
    return 0;
    }
    static int pn26g0xa_ooblayout_free(struct mtd_info *mtd, int section,
    struct mtd_oob_region *region)
    {
    if (section > 4)
    return -ERANGE;
    if (section == 4) {
    region.offset = 64;
    region.length = 64;
    } else {
    region.offset = 4 + (15 * section);
    region.length = 2;
    }
    return 0;
    }
    static int pn26g0xa_ecc_get_status(struct spinand_device *spinand,
    u8 status)
    {
    switch (status & PN26G0XA_STATUS_ECC_BITMASK) {
    case PN26G0XA_STATUS_ECC_NONE_DETECTED:
    return 0;
    case PN26G0XA_STATUS_ECC_1_7_CORRECTED:
    return 7;	/* Return upper limit by convention */
    case PN26G0XA_STATUS_ECC_8_CORRECTED:
    return 8;
    case PN26G0XA_STATUS_ECC_ERRORED:
    return -EBADMSG;
    default:
    break;
    }
    return -EINVAL;
    }
    static const struct mtd_ooblayout_ops pn26g0xa_ooblayout = {
    .ecc = pn26g0xa_ooblayout_ecc,
    .free = pn26g0xa_ooblayout_free,
    };
    static const struct spinand_info paragon_spinand_table[] = {
    SPINAND_INFO("PN26G01A",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0xe1),
    NAND_MEMORG(1, 2048, 128, 64, 1024, 21, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    0,
    SPINAND_ECCINFO(&pn26g0xa_ooblayout,
    pn26g0xa_ecc_get_status)),
    SPINAND_INFO("PN26G02A",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0xe2),
    NAND_MEMORG(1, 2048, 128, 64, 2048, 41, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    0,
    SPINAND_ECCINFO(&pn26g0xa_ooblayout,
    pn26g0xa_ecc_get_status)),
    };
    static const struct spinand_manufacturer_ops paragon_spinand_manuf_ops = {
    };
    const struct spinand_manufacturer paragon_spinand_manufacturer = {
    .id = SPINAND_MFR_PARAGON,
    .name = "Paragon",
    .chips = paragon_spinand_table,
    .nchips = ARRAY_SIZE(paragon_spinand_table),
    .ops = &paragon_spinand_manuf_ops,
    };
