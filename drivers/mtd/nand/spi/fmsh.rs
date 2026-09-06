//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/spi/fmsh.c
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
// Copyright (c) 2020-2021 Rockchip Electronics Co., Ltd.
//
// Author: Dingqiang Lin <jon.lin@rock-chips.com>
//

pub const SPINAND_MFR_FMSH: c_uint = 0xA1;
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
    static SPINAND_OP_VARIANTS(fm25g_read_cache_variants,
    SPINAND_PAGE_READ_FROM_CACHE_1S_4S_4S_OP(0, 1, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_1S_1S_4S_OP(0, 1, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_1S_2S_2S_OP(0, 1, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_1S_1S_2S_OP(0, 1, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_FAST_1S_1S_1S_OP(0, 1, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_1S_1S_1S_OP(0, 1, core::ptr::null_mut(), 0, 0));
    static int fm25g01b_ooblayout_ecc(struct mtd_info *mtd, int section,
    struct mtd_oob_region *region)
    {
    if (section)
    return -ERANGE;
    region.offset = 64;
    region.length = 64;
    return 0;
    }
    static int fm25g01b_ooblayout_free(struct mtd_info *mtd, int section,
    struct mtd_oob_region *region)
    {
    if (section)
    return -ERANGE;
// reserve 2 bytes for the BBM
    region.offset = 2;
    region.length = 62;
    return 0;
    }
    static int fm25g01b_ecc_get_status(struct spinand_device *spinand,
    u8 status)
    {
    switch (status & FM25G01B_STATUS_ECC_MASK) {
    case FM25G01B_STATUS_ECC_NO_BITFLIPS:
    return 0;
    case FM25G01B_STATUS_ECC_1_3_BITFLIPS:
    return 3;
    case FM25G01B_STATUS_ECC_4_BITFLIPS:
    return 4;
    case FM25G01B_STATUS_ECC_5_BITFLIPS:
    return 5;
    case FM25G01B_STATUS_ECC_6_BITFLIPS:
    return 6;
    case FM25G01B_STATUS_ECC_7_BITFLIPS:
    return 7;
    case FM25G01B_STATUS_ECC_8_BITFLIPS:
    return 8;
    case FM25G01B_STATUS_ECC_UNCOR_ERROR:
    return -EBADMSG;
    default:
    break;
    }
    return -EINVAL;
    }
    static int fm25s01a_ooblayout_ecc(struct mtd_info *mtd, int section,
    struct mtd_oob_region *region)
    {
    return -ERANGE;
    }
    static int fm25s01a_ooblayout_free(struct mtd_info *mtd, int section,
    struct mtd_oob_region *region)
    {
    if (section)
    return -ERANGE;
    region.offset = 2;
    region.length = 62;
    return 0;
    }
    static int fm25s01bi3_ecc_get_status(struct spinand_device *spinand,
    u8 status)
    {
    switch (status & FM25S01BI3_STATUS_ECC_MASK) {
    case FM25S01BI3_STATUS_ECC_NO_BITFLIPS:
    return 0;
    case FM25S01BI3_STATUS_ECC_UNCOR_ERROR:
    return -EBADMSG;
    case FM25S01BI3_STATUS_ECC_1_3_BITFLIPS:
    return 3;
    case FM25S01BI3_STATUS_ECC_4_6_BITFLIPS:
    return 6;
    case FM25S01BI3_STATUS_ECC_7_8_BITFLIPS:
    return 8;
    default:
    break;
    }
    return -EINVAL;
    }
    static int fm25s01bi3_ooblayout_ecc(struct mtd_info *mtd, int section,
    struct mtd_oob_region *region)
    {
    if (section)
    return -ERANGE;
    region.offset = 64;
    region.length = 64;
    return 0;
    }
    static int fm25s01bi3_ooblayout_free(struct mtd_info *mtd, int section,
    struct mtd_oob_region *region)
    {
    if (section > 3)
    return -ERANGE;
    region.offset = (16 * section) + 4;
    region.length = 12;
    return 0;
    }
    static const struct mtd_ooblayout_ops fm25g01b_ooblayout = {
    .ecc = fm25g01b_ooblayout_ecc,
    .free = fm25g01b_ooblayout_free,
    };
    static const struct mtd_ooblayout_ops fm25s01a_ooblayout = {
    .ecc = fm25s01a_ooblayout_ecc,
    .free = fm25s01a_ooblayout_free,
    };
    static const struct mtd_ooblayout_ops fm25s01bi3_ooblayout = {
    .ecc = fm25s01bi3_ooblayout_ecc,
    .free = fm25s01bi3_ooblayout_free,
    };
    static const struct spinand_info fmsh_spinand_table[] = {
    SPINAND_INFO("FM25G01B",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0xd1),
    NAND_MEMORG(1, 2048, 128, 64, 1024, 21, 1, 1, 1),
    NAND_ECCREQ(8, 528),
    SPINAND_INFO_OP_VARIANTS(&fm25g_read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&fm25g01b_ooblayout,
    fm25g01b_ecc_get_status)),
    SPINAND_INFO("FM25G02B",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0xd2),
    NAND_MEMORG(1, 2048, 128, 64, 2048, 41, 1, 1, 1),
    NAND_ECCREQ(8, 528),
    SPINAND_INFO_OP_VARIANTS(&fm25g_read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&fm25g01b_ooblayout,
    fm25g01b_ecc_get_status)),
    SPINAND_INFO("FM25S01A",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0xE4),
    NAND_MEMORG(1, 2048, 64, 64, 1024, 20, 1, 1, 1),
    NAND_ECCREQ(1, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    0,
    SPINAND_ECCINFO(&fm25s01a_ooblayout, core::ptr::null_mut())),
    SPINAND_INFO("FM25S01BI3",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0xd4),
    NAND_MEMORG(1, 2048, 128, 64, 1024, 20, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&fm25s01bi3_ooblayout,
    fm25s01bi3_ecc_get_status)),
    };
    static const struct spinand_manufacturer_ops fmsh_spinand_manuf_ops = {
    };
    const struct spinand_manufacturer fmsh_spinand_manufacturer = {
    .id = SPINAND_MFR_FMSH,
    .name = "Fudan Micro",
    .chips = fmsh_spinand_table,
    .nchips = ARRAY_SIZE(fmsh_spinand_table),
    .ops = &fmsh_spinand_manuf_ops,
    };
