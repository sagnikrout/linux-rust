//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/spi/alliancememory.c
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
// Author: Mario Kicherer <dev@kicherer.org>
//

pub const SPINAND_MFR_ALLIANCEMEMORY: c_uint = 0x52;

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
#[no_mangle]
unsafe extern "C" fn am_get_eccsize(mtd: *mut mtd_info) -> c_int {
    static int am_get_eccsize(struct mtd_info *mtd)
    {
    if (mtd.oobsize == 64)
    return 0x20;
#[no_mangle]
pub unsafe extern "C" fn if(128: mtd->oobsize ==) -> else {
    else if (mtd.oobsize == 128)
    return 0x38;
#[no_mangle]
pub unsafe extern "C" fn if(256: mtd->oobsize ==) -> else {
    else if (mtd.oobsize == 256)
    return 0x70;
    else
    return -EINVAL;
    }
    static int am_ooblayout_ecc(struct mtd_info *mtd, int section,
    struct mtd_oob_region *region)
    {
    int ecc_bytes;
    ecc_bytes = am_get_eccsize(mtd);
    if (ecc_bytes < 0)
    return ecc_bytes;
    region.offset = mtd.oobsize - ecc_bytes;
    region.length = ecc_bytes;
    return 0;
    }
    static int am_ooblayout_free(struct mtd_info *mtd, int section,
    struct mtd_oob_region *region)
    {
    int ecc_bytes;
    if (section)
    return -ERANGE;
    ecc_bytes = am_get_eccsize(mtd);
    if (ecc_bytes < 0)
    return ecc_bytes;
//
// It is unclear how many bytes are used for the bad block marker. We
// reserve the common two bytes here.
//
// The free area in this kind of flash is divided into chunks where the
// first 4 bytes of each chunk are unprotected. The number of chunks
// depends on the specific model. The models with 4096+256 bytes pages
// have 8 chunks, the others 4 chunks.
//
    region.offset = 2;
    region.length = mtd.oobsize - 2 - ecc_bytes;
    return 0;
    }
    static const struct mtd_ooblayout_ops am_ooblayout = {
    .ecc = am_ooblayout_ecc,
    .free = am_ooblayout_free,
    };
#[no_mangle]
unsafe extern "C" fn am_ecc_get_status(spinand: *mut spinand_device, status: u8) -> c_int {
    static int am_ecc_get_status(struct spinand_device *spinand, u8 status)
    {
    switch (status & AM_STATUS_ECC_BITMASK) {
    case AM_STATUS_ECC_NONE_DETECTED:
    return 0;
    case AM_STATUS_ECC_CORRECTED:
//
// use oobsize to determine the flash model and the maximum of
// correctable errors and return maximum - 1 by convention
//
    if (spinand.base.mtd.oobsize == 64)
    return 3;
    else
    return 7;
    case AM_STATUS_ECC_ERRORED:
    return -EBADMSG;
    case AM_STATUS_ECC_MAX_CORRECTED:
//
// use oobsize to determine the flash model and the maximum of
// correctable errors
//
    if (spinand.base.mtd.oobsize == 64)
    return 4;
    else
    return 8;
    default:
    break;
    }
    return -EINVAL;
    }
    static const struct spinand_info alliancememory_spinand_table[] = {
    SPINAND_INFO("AS5F34G04SND",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x2f),
    NAND_MEMORG(1, 2048, 128, 64, 4096, 80, 1, 1, 1),
    NAND_ECCREQ(4, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&am_ooblayout,
    am_ecc_get_status)),
    };
    static const struct spinand_manufacturer_ops alliancememory_spinand_manuf_ops = {
    };
    const struct spinand_manufacturer alliancememory_spinand_manufacturer = {
    .id = SPINAND_MFR_ALLIANCEMEMORY,
    .name = "AllianceMemory",
    .chips = alliancememory_spinand_table,
    .nchips = ARRAY_SIZE(alliancememory_spinand_table),
    .ops = &alliancememory_spinand_manuf_ops,
    };
