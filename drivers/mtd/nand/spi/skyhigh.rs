//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/spi/skyhigh.c
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
// Copyright (c) 2024 SkyHigh Memory Limited
//
// Author: Takahiro Kuwano <takahiro.kuwano@infineon.com>
// Co-Author: KR Kim <kr.kim@skyhighmemory.com>
//

pub const SPINAND_MFR_SKYHIGH: c_uint = 0x01;

    static SPINAND_OP_VARIANTS(read_cache_variants,
    SPINAND_PAGE_READ_FROM_CACHE_1S_4S_4S_OP(0, 4, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_1S_1S_4S_OP(0, 1, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_1S_2S_2S_OP(0, 2, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_1S_1S_2S_OP(0, 1, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_FAST_1S_1S_1S_OP(0, 1, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_1S_1S_1S_OP(0, 1, core::ptr::null_mut(), 0, 0));
    static SPINAND_OP_VARIANTS(write_cache_variants,
    SPINAND_PROG_LOAD_1S_1S_4S_OP(true, 0, core::ptr::null_mut(), 0),
    SPINAND_PROG_LOAD_1S_1S_1S_OP(true, 0, core::ptr::null_mut(), 0));
    static SPINAND_OP_VARIANTS(update_cache_variants,
    SPINAND_PROG_LOAD_1S_1S_4S_OP(false, 0, core::ptr::null_mut(), 0),
    SPINAND_PROG_LOAD_1S_1S_1S_OP(false, 0, core::ptr::null_mut(), 0));
    static int skyhigh_spinand_ooblayout_ecc(struct mtd_info *mtd, int section,
    struct mtd_oob_region *region)
    {
// ECC bytes are stored in hidden area.
    return -ERANGE;
    }
    static int skyhigh_spinand_ooblayout_free(struct mtd_info *mtd, int section,
    struct mtd_oob_region *region)
    {
    if (section)
    return -ERANGE;
// ECC bytes are stored in hidden area. Reserve 2 bytes for the BBM.
    region.offset = 2;
    region.length = mtd.oobsize - 2;
    return 0;
    }
    static const struct mtd_ooblayout_ops skyhigh_spinand_ooblayout = {
    .ecc = skyhigh_spinand_ooblayout_ecc,
    .free = skyhigh_spinand_ooblayout_free,
    };
    static int skyhigh_spinand_ecc_get_status(struct spinand_device *spinand,
    u8 status)
    {
    switch (status & STATUS_ECC_MASK) {
    case STATUS_ECC_NO_BITFLIPS:
    return 0;
    case SKYHIGH_STATUS_ECC_UNCOR_ERROR:
    return -EBADMSG;
    case SKYHIGH_STATUS_ECC_1TO2_BITFLIPS:
    return 2;
    case SKYHIGH_STATUS_ECC_3TO6_BITFLIPS:
    return 6;
    default:
    break;
    }
    return -EINVAL;
    }
    static const struct spinand_info skyhigh_spinand_table[] = {
    SPINAND_INFO("S35ML01G301",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x15),
    NAND_MEMORG(1, 2048, 64, 64, 1024, 20, 1, 1, 1),
    NAND_ECCREQ(6, 32),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_NO_RAW_ACCESS,
    SPINAND_ECCINFO(&skyhigh_spinand_ooblayout,
    skyhigh_spinand_ecc_get_status)),
    SPINAND_INFO("S35ML01G300",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x14),
    NAND_MEMORG(1, 2048, 128, 64, 1024, 20, 1, 1, 1),
    NAND_ECCREQ(6, 32),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_NO_RAW_ACCESS,
    SPINAND_ECCINFO(&skyhigh_spinand_ooblayout,
    skyhigh_spinand_ecc_get_status)),
    SPINAND_INFO("S35ML02G300",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x25),
    NAND_MEMORG(1, 2048, 128, 64, 2048, 40, 2, 1, 1),
    NAND_ECCREQ(6, 32),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_NO_RAW_ACCESS,
    SPINAND_ECCINFO(&skyhigh_spinand_ooblayout,
    skyhigh_spinand_ecc_get_status)),
    SPINAND_INFO("S35ML04G300",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x35),
    NAND_MEMORG(1, 2048, 128, 64, 4096, 80, 2, 1, 1),
    NAND_ECCREQ(6, 32),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_NO_RAW_ACCESS,
    SPINAND_ECCINFO(&skyhigh_spinand_ooblayout,
    skyhigh_spinand_ecc_get_status)),
    };
#[no_mangle]
unsafe extern "C" fn skyhigh_spinand_init(spinand: *mut spinand_device) -> c_int {
    static int skyhigh_spinand_init(struct spinand_device *spinand)
    {
//
// Config_Protect_En (bit 1 in Block Lock register) must be set to 1
// before writing other bits. Do it here before core unlocks all blocks
// by writing block protection bits.
//
    return spinand_write_reg_op(spinand, REG_BLOCK_LOCK,
    SKYHIGH_CONFIG_PROTECT_EN);
    }
    static const struct spinand_manufacturer_ops skyhigh_spinand_manuf_ops = {
    .init = skyhigh_spinand_init,
    };
    const struct spinand_manufacturer skyhigh_spinand_manufacturer = {
    .id = SPINAND_MFR_SKYHIGH,
    .name = "SkyHigh",
    .chips = skyhigh_spinand_table,
    .nchips = ARRAY_SIZE(skyhigh_spinand_table),
    .ops = &skyhigh_spinand_manuf_ops,
    };
