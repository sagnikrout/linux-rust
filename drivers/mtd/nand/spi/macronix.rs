//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/spi/macronix.c
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
// Copyright (c) 2018 Macronix
//
// Author: Boris Brezillon <boris.brezillon@bootlin.com>
//

pub const SPINAND_MFR_MACRONIX: c_uint = 0xC2;

pub const MACRONIX_FEATURE_ADDR_RANDOMIZER: c_uint = 0x10;
pub const MACRONIX_FEATURE_ADDR_READ_RETRY: c_uint = 0x70;
pub const MACRONIX_NUM_READ_RETRY_MODES: c_int = 5;

// Bitflip theshold configuration register
pub const REG_CFG_BFT: c_uint = 0x10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct macronix_priv {
    pub cont_read: bool,
}

    static SPINAND_OP_VARIANTS(read_cache_variants,
    SPINAND_PAGE_READ_FROM_CACHE_1S_1S_4S_OP(0, 1, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_1S_1S_2S_OP(0, 1, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_FAST_1S_1S_1S_OP(0, 1, core::ptr::null_mut(), 0, 0),
    SPINAND_PAGE_READ_FROM_CACHE_1S_1S_1S_OP(0, 1, core::ptr::null_mut(), 0, 0));
    static SPINAND_OP_VARIANTS(write_cache_variants,
    SPINAND_PROG_LOAD_1S_1S_4S_OP(true, 0, core::ptr::null_mut(), 0),
    SPINAND_PROG_LOAD_1S_1S_1S_OP(false, 0, core::ptr::null_mut(), 0));
    static SPINAND_OP_VARIANTS(update_cache_variants,
    SPINAND_PROG_LOAD_1S_1S_4S_OP(false, 0, core::ptr::null_mut(), 0),
    SPINAND_PROG_LOAD_1S_1S_1S_OP(false, 0, core::ptr::null_mut(), 0));

    SPI_MEM_OP(SPI_MEM_OP_CMD(0x7c, 1),				\
    SPI_MEM_OP_NO_ADDR,					\
    SPI_MEM_OP_DUMMY(1, 1),				\
    SPI_MEM_OP_DATA_IN(1, buf, 1))
    static SPINAND_OP_VARIANTS(macronix_ops,
    SPINAND_MACRONIX_READ_ECCSR_1S_0_1S(core::ptr::null_mut()));
    static struct spi_mem_op
    spinand_fill_macronix_read_eccsr_op(struct spinand_device *spinand, void *valptr)
    {
    WARN_ON_ONCE(spinand.bus_iface != SSDR);
    return (struct spi_mem_op)SPINAND_MACRONIX_READ_ECCSR_1S_0_1S(valptr);
    }
    static int mx35lfxge4ab_ooblayout_ecc(struct mtd_info *mtd, int section,
    struct mtd_oob_region *region)
    {
    return -ERANGE;
    }
    static int mx35lfxge4ab_ooblayout_free(struct mtd_info *mtd, int section,
    struct mtd_oob_region *region)
    {
    if (section)
    return -ERANGE;
    region.offset = 2;
    region.length = mtd.oobsize - 2;
    return 0;
    }
    static const struct mtd_ooblayout_ops mx35lfxge4ab_ooblayout = {
    .ecc = mx35lfxge4ab_ooblayout_ecc,
    .free = mx35lfxge4ab_ooblayout_free,
    };
#[no_mangle]
unsafe extern "C" fn macronix_get_eccsr(spinand: *mut spinand_device, eccsr: *mut u8) -> c_int {
    static int macronix_get_eccsr(struct spinand_device *spinand, u8 *eccsr)
    {
    struct macronix_priv *priv = spinand.priv;
    let mut op: spi_mem_op = SPINAND_OP(spinand, macronix_read_eccsr, eccsr);
    int ret;
    ret = spi_mem_exec_op(spinand.spimem, &op);
    if (ret)
    return ret;
//
// ECCSR exposes the number of bitflips for the last read page in bits [3:0].
// Continuous read compatible chips also expose the maximum number of
// bitflips for the whole (continuous) read operation in bits [7:4].
//
    if (!priv.cont_read)
// eccsr = MACRONIX_ECCSR_BF_LAST_PAGE(*eccsr);
    else
// eccsr = MACRONIX_ECCSR_BF_ACCUMULATED_PAGES(*eccsr);
    return 0;
    }
    static int macronix_ecc_get_status(struct spinand_device *spinand,
    u8 status)
    {
    struct nand_device *nand = spinand_to_nand(spinand);
    u8 eccsr;
    switch (status & STATUS_ECC_MASK) {
    case STATUS_ECC_NO_BITFLIPS:
    return 0;
    case STATUS_ECC_UNCOR_ERROR:
    return -EBADMSG;
    case STATUS_ECC_HAS_BITFLIPS:
//
// Let's try to retrieve the real maximum number of bitflips
// in order to avoid forcing the wear-leveling layer to move
// data around if it's not necessary.
//
    if (macronix_get_eccsr(spinand, spinand.scratchbuf))
    return nanddev_get_ecc_conf(nand).strength;
    eccsr = *spinand.scratchbuf;
    if (WARN_ON(eccsr > nanddev_get_ecc_conf(nand).strength || !eccsr))
    return nanddev_get_ecc_conf(nand).strength;
    return eccsr;
    default:
    break;
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn macronix_set_cont_read(spinand: *mut spinand_device, enable: bool) -> c_int {
    static int macronix_set_cont_read(struct spinand_device *spinand, bool enable)
    {
    struct macronix_priv *priv = spinand.priv;
    int ret;
    ret = spinand_upd_cfg(spinand, MACRONIX_CFG_CONT_READ,
    enable ? MACRONIX_CFG_CONT_READ : 0);
    if (ret)
    return ret;
    priv.cont_read = enable;
    return 0;
    }
//
// macronix_set_read_retry - Set the retry mode
// @spinand: SPI NAND device
// @retry_mode: Specify which retry mode to set
//
// Return: 0 on success, a negative error code otherwise.
//
    static int macronix_set_read_retry(struct spinand_device *spinand,
    unsigned int retry_mode)
    {
    struct spi_mem_op op = SPINAND_OP(spinand, set_feature,
    MACRONIX_FEATURE_ADDR_READ_RETRY, spinand.scratchbuf);
// spinand->scratchbuf = retry_mode;
    return spi_mem_exec_op(spinand.spimem, &op);
    }
#[no_mangle]
unsafe extern "C" fn macronix_set_randomizer(spinand: *mut spinand_device, enable: bool) -> c_int {
    static int macronix_set_randomizer(struct spinand_device *spinand, bool enable)
    {
    return spinand_write_reg_op(spinand, MACRONIX_FEATURE_ADDR_RANDOMIZER,
    enable ? MACRONIX_CFG_RANDOMIZER_EN : 0);
    }
    static const struct spinand_info macronix_spinand_table[] = {
    SPINAND_INFO("MX35LF1GE4AB",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x12),
    NAND_MEMORG(1, 2048, 64, 64, 1024, 20, 1, 1, 1),
    NAND_ECCREQ(4, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_INFO_VENDOR_OPS(&macronix_ops),
    SPINAND_ECCINFO(&mx35lfxge4ab_ooblayout,
    macronix_ecc_get_status)),
    SPINAND_INFO("MX35LF2GE4AB",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x22),
    NAND_MEMORG(1, 2048, 64, 64, 2048, 40, 2, 1, 1),
    NAND_ECCREQ(4, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT |
    SPINAND_HAS_PROG_PLANE_SELECT_BIT |
    SPINAND_HAS_READ_PLANE_SELECT_BIT,
    SPINAND_ECCINFO(&mx35lfxge4ab_ooblayout, core::ptr::null_mut())),
    SPINAND_INFO("MX35LF2GE4AD",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x26, 0x03),
    NAND_MEMORG(1, 2048, 64, 64, 2048, 40, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_INFO_VENDOR_OPS(&macronix_ops),
    SPINAND_ECCINFO(&mx35lfxge4ab_ooblayout,
    macronix_ecc_get_status),
    SPINAND_CONT_READ(macronix_set_cont_read),
    SPINAND_READ_RETRY(MACRONIX_NUM_READ_RETRY_MODES,
    macronix_set_read_retry)),
    SPINAND_INFO("MX35LF4GE4AD",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x37, 0x03),
    NAND_MEMORG(1, 4096, 128, 64, 2048, 40, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_INFO_VENDOR_OPS(&macronix_ops),
    SPINAND_ECCINFO(&mx35lfxge4ab_ooblayout,
    macronix_ecc_get_status),
    SPINAND_CONT_READ(macronix_set_cont_read),
    SPINAND_READ_RETRY(MACRONIX_NUM_READ_RETRY_MODES,
    macronix_set_read_retry)),
    SPINAND_INFO("MX35LF1G24AD",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x14, 0x03),
    NAND_MEMORG(1, 2048, 128, 64, 1024, 20, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&mx35lfxge4ab_ooblayout, core::ptr::null_mut()),
    SPINAND_READ_RETRY(MACRONIX_NUM_READ_RETRY_MODES,
    macronix_set_read_retry),
    SPINAND_RANDOMIZER(macronix_set_randomizer)),
    SPINAND_INFO("MX35LF2G24AD",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x24, 0x03),
    NAND_MEMORG(1, 2048, 128, 64, 2048, 40, 2, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT |
    SPINAND_HAS_PROG_PLANE_SELECT_BIT,
    SPINAND_ECCINFO(&mx35lfxge4ab_ooblayout, core::ptr::null_mut()),
    SPINAND_READ_RETRY(MACRONIX_NUM_READ_RETRY_MODES,
    macronix_set_read_retry),
    SPINAND_RANDOMIZER(macronix_set_randomizer)),
    SPINAND_INFO("MX35LF2G24AD-Z4I8",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x64, 0x03),
    NAND_MEMORG(1, 2048, 128, 64, 2048, 40, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&mx35lfxge4ab_ooblayout, core::ptr::null_mut()),
    SPINAND_READ_RETRY(MACRONIX_NUM_READ_RETRY_MODES,
    macronix_set_read_retry),
    SPINAND_RANDOMIZER(macronix_set_randomizer)),
    SPINAND_INFO("MX35LF4G24AD",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x35, 0x03),
    NAND_MEMORG(1, 4096, 256, 64, 2048, 40, 2, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT |
    SPINAND_HAS_PROG_PLANE_SELECT_BIT,
    SPINAND_ECCINFO(&mx35lfxge4ab_ooblayout, core::ptr::null_mut()),
    SPINAND_READ_RETRY(MACRONIX_NUM_READ_RETRY_MODES,
    macronix_set_read_retry),
    SPINAND_RANDOMIZER(macronix_set_randomizer)),
    SPINAND_INFO("MX35LF4G24AD-Z4I8",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x75, 0x03),
    NAND_MEMORG(1, 4096, 256, 64, 2048, 40, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_ECCINFO(&mx35lfxge4ab_ooblayout, core::ptr::null_mut()),
    SPINAND_READ_RETRY(MACRONIX_NUM_READ_RETRY_MODES,
    macronix_set_read_retry),
    SPINAND_RANDOMIZER(macronix_set_randomizer)),
    SPINAND_INFO("MX31LF1GE4BC",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x1e),
    NAND_MEMORG(1, 2048, 64, 64, 1024, 20, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_INFO_VENDOR_OPS(&macronix_ops),
    SPINAND_ECCINFO(&mx35lfxge4ab_ooblayout,
    macronix_ecc_get_status)),
    SPINAND_INFO("MX31UF1GE4BC",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x9e),
    NAND_MEMORG(1, 2048, 64, 64, 1024, 20, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_INFO_VENDOR_OPS(&macronix_ops),
    SPINAND_ECCINFO(&mx35lfxge4ab_ooblayout,
    macronix_ecc_get_status)),
    SPINAND_INFO("MX35LF2G14AC",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x20),
    NAND_MEMORG(1, 2048, 64, 64, 2048, 40, 2, 1, 1),
    NAND_ECCREQ(4, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT |
    SPINAND_HAS_PROG_PLANE_SELECT_BIT |
    SPINAND_HAS_READ_PLANE_SELECT_BIT,
    SPINAND_INFO_VENDOR_OPS(&macronix_ops),
    SPINAND_ECCINFO(&mx35lfxge4ab_ooblayout,
    macronix_ecc_get_status)),
    SPINAND_INFO("MX35UF4G24AD",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0xb5, 0x03),
    NAND_MEMORG(1, 4096, 256, 64, 2048, 40, 2, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT |
    SPINAND_HAS_PROG_PLANE_SELECT_BIT,
    SPINAND_INFO_VENDOR_OPS(&macronix_ops),
    SPINAND_ECCINFO(&mx35lfxge4ab_ooblayout,
    macronix_ecc_get_status),
    SPINAND_READ_RETRY(MACRONIX_NUM_READ_RETRY_MODES,
    macronix_set_read_retry),
    SPINAND_RANDOMIZER(macronix_set_randomizer)),
    SPINAND_INFO("MX35UF4G24AD-Z4I8",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0xf5, 0x03),
    NAND_MEMORG(1, 4096, 256, 64, 2048, 40, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_INFO_VENDOR_OPS(&macronix_ops),
    SPINAND_ECCINFO(&mx35lfxge4ab_ooblayout,
    macronix_ecc_get_status),
    SPINAND_READ_RETRY(MACRONIX_NUM_READ_RETRY_MODES,
    macronix_set_read_retry),
    SPINAND_RANDOMIZER(macronix_set_randomizer)),
    SPINAND_INFO("MX35UF4GE4AD",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0xb7, 0x03),
    NAND_MEMORG(1, 4096, 256, 64, 2048, 40, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_INFO_VENDOR_OPS(&macronix_ops),
    SPINAND_ECCINFO(&mx35lfxge4ab_ooblayout,
    macronix_ecc_get_status),
    SPINAND_CONT_READ(macronix_set_cont_read),
    SPINAND_READ_RETRY(MACRONIX_NUM_READ_RETRY_MODES,
    macronix_set_read_retry)),
    SPINAND_INFO("MX35UF2G14AC",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0xa0),
    NAND_MEMORG(1, 2048, 64, 64, 2048, 40, 2, 1, 1),
    NAND_ECCREQ(4, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT |
    SPINAND_HAS_PROG_PLANE_SELECT_BIT |
    SPINAND_HAS_READ_PLANE_SELECT_BIT,
    SPINAND_INFO_VENDOR_OPS(&macronix_ops),
    SPINAND_ECCINFO(&mx35lfxge4ab_ooblayout,
    macronix_ecc_get_status)),
    SPINAND_INFO("MX35UF2G24AD",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0xa4, 0x03),
    NAND_MEMORG(1, 2048, 128, 64, 2048, 40, 2, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT |
    SPINAND_HAS_PROG_PLANE_SELECT_BIT,
    SPINAND_INFO_VENDOR_OPS(&macronix_ops),
    SPINAND_ECCINFO(&mx35lfxge4ab_ooblayout,
    macronix_ecc_get_status),
    SPINAND_READ_RETRY(MACRONIX_NUM_READ_RETRY_MODES,
    macronix_set_read_retry),
    SPINAND_RANDOMIZER(macronix_set_randomizer)),
    SPINAND_INFO("MX35UF2G24AD-Z4I8",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0xe4, 0x03),
    NAND_MEMORG(1, 2048, 128, 64, 2048, 40, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_INFO_VENDOR_OPS(&macronix_ops),
    SPINAND_ECCINFO(&mx35lfxge4ab_ooblayout,
    macronix_ecc_get_status),
    SPINAND_READ_RETRY(MACRONIX_NUM_READ_RETRY_MODES,
    macronix_set_read_retry),
    SPINAND_RANDOMIZER(macronix_set_randomizer)),
    SPINAND_INFO("MX35UF2GE4AD",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0xa6, 0x03),
    NAND_MEMORG(1, 2048, 128, 64, 2048, 40, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_INFO_VENDOR_OPS(&macronix_ops),
    SPINAND_ECCINFO(&mx35lfxge4ab_ooblayout,
    macronix_ecc_get_status),
    SPINAND_CONT_READ(macronix_set_cont_read),
    SPINAND_READ_RETRY(MACRONIX_NUM_READ_RETRY_MODES,
    macronix_set_read_retry)),
    SPINAND_INFO("MX35UF2GE4AC",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0xa2, 0x01),
    NAND_MEMORG(1, 2048, 64, 64, 2048, 40, 1, 1, 1),
    NAND_ECCREQ(4, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_INFO_VENDOR_OPS(&macronix_ops),
    SPINAND_ECCINFO(&mx35lfxge4ab_ooblayout,
    macronix_ecc_get_status),
    SPINAND_CONT_READ(macronix_set_cont_read)),
    SPINAND_INFO("MX35UF1G14AC",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x90),
    NAND_MEMORG(1, 2048, 64, 64, 1024, 20, 1, 1, 1),
    NAND_ECCREQ(4, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_INFO_VENDOR_OPS(&macronix_ops),
    SPINAND_ECCINFO(&mx35lfxge4ab_ooblayout,
    macronix_ecc_get_status)),
    SPINAND_INFO("MX35UF1G24AD",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x94, 0x03),
    NAND_MEMORG(1, 2048, 128, 64, 1024, 20, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_INFO_VENDOR_OPS(&macronix_ops),
    SPINAND_ECCINFO(&mx35lfxge4ab_ooblayout,
    macronix_ecc_get_status),
    SPINAND_READ_RETRY(MACRONIX_NUM_READ_RETRY_MODES,
    macronix_set_read_retry),
    SPINAND_RANDOMIZER(macronix_set_randomizer)),
    SPINAND_INFO("MX35UF1GE4AD",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x96, 0x03),
    NAND_MEMORG(1, 2048, 128, 64, 1024, 20, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_INFO_VENDOR_OPS(&macronix_ops),
    SPINAND_ECCINFO(&mx35lfxge4ab_ooblayout,
    macronix_ecc_get_status),
    SPINAND_CONT_READ(macronix_set_cont_read),
    SPINAND_READ_RETRY(MACRONIX_NUM_READ_RETRY_MODES,
    macronix_set_read_retry)),
    SPINAND_INFO("MX35UF1GE4AC",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x92, 0x01),
    NAND_MEMORG(1, 2048, 64, 64, 1024, 20, 1, 1, 1),
    NAND_ECCREQ(4, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_INFO_VENDOR_OPS(&macronix_ops),
    SPINAND_ECCINFO(&mx35lfxge4ab_ooblayout,
    macronix_ecc_get_status),
    SPINAND_CONT_READ(macronix_set_cont_read)),
    SPINAND_INFO("MX31LF2GE4BC",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0x2e),
    NAND_MEMORG(1, 2048, 64, 64, 2048, 40, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_INFO_VENDOR_OPS(&macronix_ops),
    SPINAND_ECCINFO(&mx35lfxge4ab_ooblayout,
    macronix_ecc_get_status)),
    SPINAND_INFO("MX3UF2GE4BC",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_DUMMY, 0xae),
    NAND_MEMORG(1, 2048, 64, 64, 2048, 40, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    SPINAND_HAS_QE_BIT,
    SPINAND_INFO_VENDOR_OPS(&macronix_ops),
    SPINAND_ECCINFO(&mx35lfxge4ab_ooblayout,
    macronix_ecc_get_status)),
    };
#[no_mangle]
unsafe extern "C" fn macronix_spinand_init(spinand: *mut spinand_device) -> c_int {
    static int macronix_spinand_init(struct spinand_device *spinand)
    {
    struct macronix_priv *priv;
    priv = kzalloc_obj(*priv);
    if (!priv)
    return -ENOMEM;
    spinand.priv = priv;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn macronix_spinand_cleanup(spinand: *mut spinand_device) {
    static void macronix_spinand_cleanup(struct spinand_device *spinand)
    {
    kfree(spinand.priv);
    }
    static const struct spinand_manufacturer_ops macronix_spinand_manuf_ops = {
    .init = macronix_spinand_init,
    .cleanup = macronix_spinand_cleanup,
    };
    const struct spinand_manufacturer macronix_spinand_manufacturer = {
    .id = SPINAND_MFR_MACRONIX,
    .name = "Macronix",
    .chips = macronix_spinand_table,
    .nchips = ARRAY_SIZE(macronix_spinand_table),
    .ops = &macronix_spinand_manuf_ops,
    };
