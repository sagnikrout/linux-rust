//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/spi/xtx.c
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
// Author:
// Felix Matouschek <felix@matouschek.org>
//

pub const SPINAND_MFR_XTX: c_uint = 0x0B;

    static SPINAND_OP_VARIANTS(read_cache_variants,
    SPINAND_PAGE_READ_FROM_CACHE_1S_4S_4S_OP(0, 1, core::ptr::null_mut(), 0, 0),
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
    static int xt26g0xa_ooblayout_ecc(struct mtd_info *mtd, int section,
    struct mtd_oob_region *region)
    {
    if (section)
    return -ERANGE;
    region.offset = 48;
    region.length = 16;
    return 0;
    }
    static int xt26g0xa_ooblayout_free(struct mtd_info *mtd, int section,
    struct mtd_oob_region *region)
    {
    if (section)
    return -ERANGE;
    region.offset = 1;
    region.length = 47;
    return 0;
    }
    static const struct mtd_ooblayout_ops xt26g0xa_ooblayout = {
    .ecc = xt26g0xa_ooblayout_ecc,
    .free = xt26g0xa_ooblayout_free,
    };
    static int xt26g0xa_ecc_get_status(struct spinand_device *spinand,
    u8 status)
    {
    status = status & XT26G0XA_STATUS_ECC_MASK;
    switch (status) {
    case XT26G0XA_STATUS_ECC_NO_DETECTED:
    return 0;
    case XT26G0XA_STATUS_ECC_8_CORRECTED:
    return 8;
    case XT26G0XA_STATUS_ECC_UNCOR_ERROR:
    return -EBADMSG;
    default:
    break;
    }
// At this point values greater than (2 << 4) are invalid
    if (status > XT26G0XA_STATUS_ECC_UNCOR_ERROR)
    return -EINVAL;
// (1 << 2) through (7 << 2) are 1-7 corrected errors
    return status >> 2;
    }
    static int xt26xxxd_ooblayout_ecc(struct mtd_info *mtd, int section,
    struct mtd_oob_region *region)
    {
    if (section)
    return -ERANGE;
    region.offset = mtd.oobsize / 2;
    region.length = mtd.oobsize / 2;
    return 0;
    }
    static int xt26xxxd_ooblayout_free(struct mtd_info *mtd, int section,
    struct mtd_oob_region *region)
    {
    if (section)
    return -ERANGE;
    region.offset = 2;
    region.length = mtd.oobsize / 2 - 2;
    return 0;
    }
    static const struct mtd_ooblayout_ops xt26xxxd_ooblayout = {
    .ecc = xt26xxxd_ooblayout_ecc,
    .free = xt26xxxd_ooblayout_free,
    };
    static int xt26xxxd_ecc_get_status(struct spinand_device *spinand,
    u8 status)
    {
    switch (FIELD_GET(STATUS_ECC_MASK, status)) {
    case XT26XXXD_STATUS_ECC_NO_DETECTED:
    return 0;
    case XT26XXXD_STATUS_ECC_UNCOR_ERROR:
    return -EBADMSG;
    case XT26XXXD_STATUS_ECC_1_7_CORRECTED:
    return 4 + FIELD_GET(XT26XXXD_STATUS_ECC3_ECC2_MASK, status);
    case XT26XXXD_STATUS_ECC_8_CORRECTED:
    return 8;
    default:
    break;
    }
    return -EINVAL;
    }
    static const struct spinand_info xtx_spinand_table[] = {
    SPINAND_INFO("XT26G01A",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_ADDR, 0xE1),
    NAND_MEMORG(1, 2048, 64, 64, 1024, 20, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&xt26g0xa_ooblayout,
    xt26g0xa_ecc_get_status)),
    SPINAND_INFO("XT26G02A",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_ADDR, 0xE2),
    NAND_MEMORG(1, 2048, 64, 64, 2048, 40, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&xt26g0xa_ooblayout,
    xt26g0xa_ecc_get_status)),
    SPINAND_INFO("XT26G04A",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_ADDR, 0xE3),
    NAND_MEMORG(1, 2048, 64, 128, 2048, 40, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&xt26g0xa_ooblayout,
    xt26g0xa_ecc_get_status)),
    SPINAND_INFO("XT26G01D",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_ADDR, 0x31),
    NAND_MEMORG(1, 2048, 128, 64, 1024, 20, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    0,
    SPINAND_ECCINFO(&xt26xxxd_ooblayout,
    xt26xxxd_ecc_get_status)),
    SPINAND_INFO("XT26G11D",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_ADDR, 0x34),
    NAND_MEMORG(1, 2048, 128, 64, 1024, 20, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    0,
    SPINAND_ECCINFO(&xt26xxxd_ooblayout,
    xt26xxxd_ecc_get_status)),
    SPINAND_INFO("XT26Q01D",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_ADDR, 0x51),
    NAND_MEMORG(1, 2048, 128, 64, 1024, 20, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    0,
    SPINAND_ECCINFO(&xt26xxxd_ooblayout,
    xt26xxxd_ecc_get_status)),
    SPINAND_INFO("XT26G02D",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_ADDR, 0x32),
    NAND_MEMORG(1, 2048, 128, 64, 2048, 40, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    0,
    SPINAND_ECCINFO(&xt26xxxd_ooblayout,
    xt26xxxd_ecc_get_status)),
    SPINAND_INFO("XT26G12D",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_ADDR, 0x35),
    NAND_MEMORG(1, 2048, 128, 64, 2048, 40, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    0,
    SPINAND_ECCINFO(&xt26xxxd_ooblayout,
    xt26xxxd_ecc_get_status)),
    SPINAND_INFO("XT26Q02D",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_ADDR, 0x52),
    NAND_MEMORG(1, 2048, 128, 64, 2048, 40, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    0,
    SPINAND_ECCINFO(&xt26xxxd_ooblayout,
    xt26xxxd_ecc_get_status)),
    SPINAND_INFO("XT26G04D",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_ADDR, 0x33),
    NAND_MEMORG(1, 4096, 256, 64, 2048, 40, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    0,
    SPINAND_ECCINFO(&xt26xxxd_ooblayout,
    xt26xxxd_ecc_get_status)),
    SPINAND_INFO("XT26Q04D",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_ADDR, 0x53),
    NAND_MEMORG(1, 4096, 256, 64, 2048, 40, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    0,
    SPINAND_ECCINFO(&xt26xxxd_ooblayout,
    xt26xxxd_ecc_get_status)),
    };
    static const struct spinand_manufacturer_ops xtx_spinand_manuf_ops = {
    };
    const struct spinand_manufacturer xtx_spinand_manufacturer = {
    .id = SPINAND_MFR_XTX,
    .name = "XTX",
    .chips = xtx_spinand_table,
    .nchips = ARRAY_SIZE(xtx_spinand_table),
    .ops = &xtx_spinand_manuf_ops,
    };
