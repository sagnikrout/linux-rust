//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/spi/gigadevice.c
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
// Chuanhong Guo <gch981213@gmail.com>
//

pub const SPINAND_MFR_GIGADEVICE: c_uint = 0xC8;

pub const GD5FXGQXXEXXG_REG_STATUS2: c_uint = 0xf0;

// Feature bit definitions

// ECC status extraction helpers

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gigadevice_priv {
    pub continuous_read: bool,
}

    static SPINAND_OP_VARIANTS(read_cache_variants,
    SPINAND_PAGE_READ_FROM_CACHE_1S_4S_4S_OP(0, 1, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_1S_1S_4S_OP(0, 1, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_1S_2S_2S_OP(0, 1, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_1S_1S_2S_OP(0, 1, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_FAST_1S_1S_1S_OP(0, 1, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_1S_1S_1S_OP(0, 1, core::ptr::null_mut(), 0, 0));
    static SPINAND_OP_VARIANTS(read_cache_variants_f,
    SPINAND_PAGE_READ_FROM_CACHE_1S_4S_4S_OP(0, 1, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_3A_1S_1S_4S_OP(0, 1, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_1S_2S_2S_OP(0, 1, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_3A_1S_1S_2S_OP(0, 1, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_FAST_3A_1S_1S_1S_OP(0, 1, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_3A_1S_1S_1S_OP(0, 0, core::ptr::null_mut(), 0, 0));
    static SPINAND_OP_VARIANTS(read_cache_variants_1gq5,
    SPINAND_PAGE_READ_FROM_CACHE_1S_4S_4S_OP(0, 2, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_1S_1S_4S_OP(0, 1, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_1S_2S_2S_OP(0, 1, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_1S_1S_2S_OP(0, 1, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_FAST_1S_1S_1S_OP(0, 1, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_1S_1S_1S_OP(0, 1, core::ptr::null_mut(), 0, 0));
    static SPINAND_OP_VARIANTS(read_cache_variants_2gq5,
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
#[no_mangle]
unsafe extern "C" fn gd5fxgm9_get_eccsr(spinand: *mut spinand_device, eccsr: *mut u8) -> c_int {
    static int gd5fxgm9_get_eccsr(struct spinand_device *spinand, u8 *eccsr)
    {
    struct gigadevice_priv *priv = spinand.priv;
    struct spi_mem_op op = SPI_MEM_OP(SPI_MEM_OP_CMD(0x7c, 1),
    SPI_MEM_OP_NO_ADDR,
    SPI_MEM_OP_DUMMY(1, 1),
    SPI_MEM_OP_DATA_IN(1, eccsr, 1));
    int ret;
    ret = spi_mem_exec_op(spinand.spimem, &op);
    if (ret)
    return ret;
    if (priv.continuous_read)
// eccsr = GD_ECCSR_ACCUMULATED(*eccsr);
    else
// eccsr = GD_ECCSR_LAST_PAGE(*eccsr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gd5fxgm9_ecc_get_status(spinand: *mut spinand_device, status: u8) -> c_int {
    static int gd5fxgm9_ecc_get_status(struct spinand_device *spinand, u8 status)
    {
    struct nand_device *nand = spinand_to_nand(spinand);
    u8 eccsr;
    int ret;
    switch (status & STATUS_ECC_MASK) {
    case STATUS_ECC_NO_BITFLIPS:
    return 0;
    case GD5FXGQ4XA_STATUS_ECC_1_7_BITFLIPS:
    ret = gd5fxgm9_get_eccsr(spinand, spinand.scratchbuf);
    if (ret)
    return nanddev_get_ecc_conf(nand).strength;
    eccsr = *spinand.scratchbuf;
    if (WARN_ON(!eccsr || eccsr > nanddev_get_ecc_conf(nand).strength))
    return nanddev_get_ecc_conf(nand).strength;
    return eccsr;
    case GD5FXGQ4XA_STATUS_ECC_8_BITFLIPS:
    return 8;
    case STATUS_ECC_UNCOR_ERROR:
    return -EBADMSG;
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn gd5fxgm9_set_continuous_read(spinand: *mut spinand_device, enable: bool) -> c_int {
    static int gd5fxgm9_set_continuous_read(struct spinand_device *spinand, bool enable)
    {
    struct gigadevice_priv *priv = spinand.priv;
    int ret;
    ret = spinand_upd_cfg(spinand, GD_FEATURE_NR,
    enable ? 0 : GD_FEATURE_NR);
    if (ret)
    return ret;
    priv.continuous_read = enable;
    return 0;
    }
    static int gd5fxgq4xa_ooblayout_ecc(struct mtd_info *mtd, int section,
    struct mtd_oob_region *region)
    {
    if (section > 3)
    return -ERANGE;
    region.offset = (16 * section) + 8;
    region.length = 8;
    return 0;
    }
    static int gd5fxgq4xa_ooblayout_free(struct mtd_info *mtd, int section,
    struct mtd_oob_region *region)
    {
    if (section > 3)
    return -ERANGE;
    if (section) {
    region.offset = 16 * section;
    region.length = 8;
    } else {
// section 0 has one byte reserved for bad block mark
    region.offset = 1;
    region.length = 7;
    }
    return 0;
    }
    static const struct mtd_ooblayout_ops gd5fxgq4xa_ooblayout = {
    .ecc = gd5fxgq4xa_ooblayout_ecc,
    .free = gd5fxgq4xa_ooblayout_free,
    };
    static int gd5fxgq4xa_ecc_get_status(struct spinand_device *spinand,
    u8 status)
    {
    switch (status & STATUS_ECC_MASK) {
    case STATUS_ECC_NO_BITFLIPS:
    return 0;
    case GD5FXGQ4XA_STATUS_ECC_1_7_BITFLIPS:
// 1-7 bits are flipped. return the maximum.
    return 7;
    case GD5FXGQ4XA_STATUS_ECC_8_BITFLIPS:
    return 8;
    case STATUS_ECC_UNCOR_ERROR:
    return -EBADMSG;
    default:
    break;
    }
    return -EINVAL;
    }
    static int gd5fxgqx_variant2_ooblayout_ecc(struct mtd_info *mtd, int section,
    struct mtd_oob_region *region)
    {
    if (section)
    return -ERANGE;
    region.offset = 64;
    region.length = 64;
    return 0;
    }
    static int gd5fxgqx_variant2_ooblayout_free(struct mtd_info *mtd, int section,
    struct mtd_oob_region *region)
    {
    if (section)
    return -ERANGE;
// Reserve 1 bytes for the BBM.
    region.offset = 1;
    region.length = 63;
    return 0;
    }
// Valid for Q4/Q5 and Q6 (untested) devices
    static const struct mtd_ooblayout_ops gd5fxgqx_variant2_ooblayout = {
    .ecc = gd5fxgqx_variant2_ooblayout_ecc,
    .free = gd5fxgqx_variant2_ooblayout_free,
    };
    static int gd5fxgq4xc_ooblayout_256_ecc(struct mtd_info *mtd, int section,
    struct mtd_oob_region *oobregion)
    {
    if (section)
    return -ERANGE;
    oobregion.offset = 128;
    oobregion.length = 128;
    return 0;
    }
    static int gd5fxgq4xc_ooblayout_256_free(struct mtd_info *mtd, int section,
    struct mtd_oob_region *oobregion)
    {
    if (section)
    return -ERANGE;
    oobregion.offset = 1;
    oobregion.length = 127;
    return 0;
    }
    static const struct mtd_ooblayout_ops gd5fxgq4xc_oob_256_ops = {
    .ecc = gd5fxgq4xc_ooblayout_256_ecc,
    .free = gd5fxgq4xc_ooblayout_256_free,
    };
    static int gd5fxgq4uexxg_ecc_get_status(struct spinand_device *spinand,
    u8 status)
    {
    u8 status2;
    struct spi_mem_op op = SPINAND_OP(spinand, get_feature,
    GD5FXGQXXEXXG_REG_STATUS2, spinand.scratchbuf);
    int ret;
    switch (status & STATUS_ECC_MASK) {
    case STATUS_ECC_NO_BITFLIPS:
    return 0;
    case GD5FXGQ4XA_STATUS_ECC_1_7_BITFLIPS:
//
// Read status2 register to determine a more fine grained
// bit error status
//
    ret = spi_mem_exec_op(spinand.spimem, &op);
    if (ret)
    return ret;
//
// 4 ... 7 bits are flipped (1..4 can't be detected, so
// report the maximum of 4 in this case
//
// bits sorted this way (3...0): ECCS1,ECCS0,ECCSE1,ECCSE0
    status2 = *(spinand.scratchbuf);
    return ((status & STATUS_ECC_MASK) >> 2) |
    ((status2 & STATUS_ECC_MASK) >> 4);
    case GD5FXGQ4XA_STATUS_ECC_8_BITFLIPS:
    return 8;
    case STATUS_ECC_UNCOR_ERROR:
    return -EBADMSG;
    default:
    break;
    }
    return -EINVAL;
    }
    static int gd5fxgq5xexxg_ecc_get_status(struct spinand_device *spinand,
    u8 status)
    {
    u8 status2;
    struct spi_mem_op op = SPINAND_OP(spinand, get_feature,
    GD5FXGQXXEXXG_REG_STATUS2, spinand.scratchbuf);
    int ret;
    switch (status & STATUS_ECC_MASK) {
    case STATUS_ECC_NO_BITFLIPS:
    return 0;
    case GD5FXGQ5XE_STATUS_ECC_1_4_BITFLIPS:
//
// Read status2 register to determine a more fine grained
// bit error status
//
    ret = spi_mem_exec_op(spinand.spimem, &op);
    if (ret)
    return ret;
//
// 1 ... 4 bits are flipped (and corrected)
//
// bits sorted this way (1...0): ECCSE1, ECCSE0
    status2 = *(spinand.scratchbuf);
    return ((status2 & STATUS_ECC_MASK) >> 4) + 1;
    case STATUS_ECC_UNCOR_ERROR:
    return -EBADMSG;
    default:
    break;
    }
    return -EINVAL;
    }
    static int gd5fxgq4ufxxg_ecc_get_status(struct spinand_device *spinand,
    u8 status)
    {
    switch (status & GD5FXGQ4UXFXXG_STATUS_ECC_MASK) {
    case GD5FXGQ4UXFXXG_STATUS_ECC_NO_BITFLIPS:
    return 0;
    case GD5FXGQ4UXFXXG_STATUS_ECC_1_3_BITFLIPS:
    return 3;
    case GD5FXGQ4UXFXXG_STATUS_ECC_UNCOR_ERROR:
    return -EBADMSG;
    default: /* (2 << 4) through (6 << 4) are 4-8 corrected errors */
    return ((status & GD5FXGQ4UXFXXG_STATUS_ECC_MASK) >> 4) + 2;
    }
    return -EINVAL;
    }
    static const struct spinand_info gigadevice_spinand_table[] = {
    SPINAND_INFO("GD5F1GQ4xA",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_ADDR, 0xf1),
    NAND_MEMORG(1, 2048, 64, 64, 1024, 20, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&gd5fxgq4xa_ooblayout,
    gd5fxgq4xa_ecc_get_status)),
    SPINAND_INFO("GD5F2GQ4xA",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_ADDR, 0xf2),
    NAND_MEMORG(1, 2048, 64, 64, 2048, 40, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&gd5fxgq4xa_ooblayout,
    gd5fxgq4xa_ecc_get_status)),
    SPINAND_INFO("GD5F4GQ4xA",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_ADDR, 0xf4),
    NAND_MEMORG(1, 2048, 64, 64, 4096, 80, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&gd5fxgq4xa_ooblayout,
    gd5fxgq4xa_ecc_get_status)),
    SPINAND_INFO("GD5F4GQ4RC",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE, 0xa4, 0x68),
    NAND_MEMORG(1, 4096, 256, 64, 2048, 40, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants_f,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&gd5fxgq4xc_oob_256_ops,
    gd5fxgq4ufxxg_ecc_get_status)),
    SPINAND_INFO("GD5F4GQ4UC",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE, 0xb4, 0x68),
    NAND_MEMORG(1, 4096, 256, 64, 2048, 40, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants_f,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&gd5fxgq4xc_oob_256_ops,
    gd5fxgq4ufxxg_ecc_get_status)),
    SPINAND_INFO("GD5F1GQ4UExxG",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_ADDR, 0xd1),
    NAND_MEMORG(1, 2048, 128, 64, 1024, 20, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&gd5fxgqx_variant2_ooblayout,
    gd5fxgq4uexxg_ecc_get_status)),
    SPINAND_INFO("GD5F1GQ4RExxG",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_ADDR, 0xc1),
    NAND_MEMORG(1, 2048, 128, 64, 1024, 20, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&gd5fxgqx_variant2_ooblayout,
    gd5fxgq4uexxg_ecc_get_status)),
    SPINAND_INFO("GD5F2GQ4UExxG",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_ADDR, 0xd2),
    NAND_MEMORG(1, 2048, 128, 64, 2048, 40, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&gd5fxgqx_variant2_ooblayout,
    gd5fxgq4uexxg_ecc_get_status)),
    SPINAND_INFO("GD5F2GQ4RExxG",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_ADDR, 0xc2),
    NAND_MEMORG(1, 2048, 128, 64, 2048, 40, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&gd5fxgqx_variant2_ooblayout,
    gd5fxgq4uexxg_ecc_get_status)),
    SPINAND_INFO("GD5F1GQ4UFxxG",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE, 0xb1, 0x48),
    NAND_MEMORG(1, 2048, 128, 64, 1024, 20, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants_f,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&gd5fxgqx_variant2_ooblayout,
    gd5fxgq4ufxxg_ecc_get_status)),
    SPINAND_INFO("GD5F1GQ5UExxG",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x51),
    NAND_MEMORG(1, 2048, 128, 64, 1024, 20, 1, 1, 1),
    NAND_ECCREQ(4, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants_1gq5,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&gd5fxgqx_variant2_ooblayout,
    gd5fxgq5xexxg_ecc_get_status)),
    SPINAND_INFO("GD5F1GQ5RExxG",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x41),
    NAND_MEMORG(1, 2048, 128, 64, 1024, 20, 1, 1, 1),
    NAND_ECCREQ(4, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants_1gq5,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&gd5fxgqx_variant2_ooblayout,
    gd5fxgq5xexxg_ecc_get_status)),
    SPINAND_INFO("GD5F2GQ5UExxG",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x52),
    NAND_MEMORG(1, 2048, 128, 64, 2048, 40, 1, 1, 1),
    NAND_ECCREQ(4, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants_2gq5,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&gd5fxgqx_variant2_ooblayout,
    gd5fxgq5xexxg_ecc_get_status)),
    SPINAND_INFO("GD5F2GQ5RExxG",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x42),
    NAND_MEMORG(1, 2048, 128, 64, 2048, 40, 1, 1, 1),
    NAND_ECCREQ(4, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants_2gq5,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&gd5fxgqx_variant2_ooblayout,
    gd5fxgq5xexxg_ecc_get_status)),
    SPINAND_INFO("GD5F4GQ6UExxG",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x55),
    NAND_MEMORG(1, 2048, 128, 64, 2048, 40, 1, 2, 1),
    NAND_ECCREQ(4, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants_2gq5,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&gd5fxgqx_variant2_ooblayout,
    gd5fxgq5xexxg_ecc_get_status)),
    SPINAND_INFO("GD5F4GQ6RExxG",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x45),
    NAND_MEMORG(1, 2048, 128, 64, 2048, 40, 1, 2, 1),
    NAND_ECCREQ(4, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants_2gq5,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&gd5fxgqx_variant2_ooblayout,
    gd5fxgq5xexxg_ecc_get_status)),
    SPINAND_INFO("GD5F1GM7UExxG",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x91),
    NAND_MEMORG(1, 2048, 128, 64, 1024, 20, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants_1gq5,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&gd5fxgqx_variant2_ooblayout,
    gd5fxgq4uexxg_ecc_get_status)),
    SPINAND_INFO("GD5F1GM7RExxG",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x81),
    NAND_MEMORG(1, 2048, 128, 64, 1024, 20, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants_1gq5,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&gd5fxgqx_variant2_ooblayout,
    gd5fxgq4uexxg_ecc_get_status)),
    SPINAND_INFO("GD5F2GM7UExxG",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x92),
    NAND_MEMORG(1, 2048, 128, 64, 2048, 40, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants_1gq5,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&gd5fxgqx_variant2_ooblayout,
    gd5fxgq4uexxg_ecc_get_status)),
    SPINAND_INFO("GD5F2GM7RExxG",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x82),
    NAND_MEMORG(1, 2048, 128, 64, 2048, 40, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants_1gq5,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&gd5fxgqx_variant2_ooblayout,
    gd5fxgq4uexxg_ecc_get_status)),
    SPINAND_INFO("GD5F4GM8UExxG",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x95),
    NAND_MEMORG(1, 2048, 128, 64, 4096, 80, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants_1gq5,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&gd5fxgqx_variant2_ooblayout,
    gd5fxgq4uexxg_ecc_get_status)),
    SPINAND_INFO("GD5F4GM8RExxG",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x85),
    NAND_MEMORG(1, 2048, 128, 64, 4096, 80, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants_1gq5,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&gd5fxgqx_variant2_ooblayout,
    gd5fxgq4uexxg_ecc_get_status)),
    SPINAND_INFO("GD5F2GQ5xExxH",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x22),
    NAND_MEMORG(1, 2048, 64, 64, 2048, 40, 1, 1, 1),
    NAND_ECCREQ(4, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants_2gq5,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&gd5fxgqx_variant2_ooblayout,
    gd5fxgq4uexxg_ecc_get_status)),
    SPINAND_INFO("GD5F1GQ5RExxH",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x21),
    NAND_MEMORG(1, 2048, 64, 64, 1024, 20, 1, 1, 1),
    NAND_ECCREQ(4, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants_1gq5,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&gd5fxgqx_variant2_ooblayout,
    gd5fxgq4uexxg_ecc_get_status)),
    SPINAND_INFO("GD5F1GQ4RExxH",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0xc9),
    NAND_MEMORG(1, 2048, 64, 64, 1024, 20, 1, 1, 1),
    NAND_ECCREQ(4, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants_1gq5,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&gd5fxgqx_variant2_ooblayout,
    gd5fxgq4uexxg_ecc_get_status)),
    SPINAND_INFO("GD5F1GM9UExxG",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x91, 0x01),
    NAND_MEMORG(1, 2048, 128, 64, 1024, 20, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants_1gq5,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&gd5fxgqx_variant2_ooblayout,
    gd5fxgm9_ecc_get_status),
    SPINAND_CONT_READ(gd5fxgm9_set_continuous_read)),
    SPINAND_INFO("GD5F1GM9RExxG",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x81, 0x01),
    NAND_MEMORG(1, 2048, 128, 64, 1024, 20, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants_1gq5,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&gd5fxgqx_variant2_ooblayout,
    gd5fxgm9_ecc_get_status),
    SPINAND_CONT_READ(gd5fxgm9_set_continuous_read)),
    };
#[no_mangle]
unsafe extern "C" fn gd5fxgm9_spinand_init(spinand: *mut spinand_device) -> c_int {
    static int gd5fxgm9_spinand_init(struct spinand_device *spinand)
    {
    struct gigadevice_priv *priv;
    priv = kzalloc_obj(*priv);
    if (!priv)
    return -ENOMEM;
    spinand.priv =  priv;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gd5fxgm9_spinand_cleanup(spinand: *mut spinand_device) {
    static void gd5fxgm9_spinand_cleanup(struct spinand_device *spinand)
    {
    kfree(spinand.priv);
    }
    static const struct spinand_manufacturer_ops gigadevice_spinand_manuf_ops = {
    .init = gd5fxgm9_spinand_init,
    .cleanup = gd5fxgm9_spinand_cleanup,
    };
    const struct spinand_manufacturer gigadevice_spinand_manufacturer = {
    .id = SPINAND_MFR_GIGADEVICE,
    .name = "GigaDevice",
    .chips = gigadevice_spinand_table,
    .nchips = ARRAY_SIZE(gigadevice_spinand_table),
    .ops = &gigadevice_spinand_manuf_ops,
    };
