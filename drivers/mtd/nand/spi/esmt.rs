//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/spi/esmt.c
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
// Chuanhong Guo <gch981213@gmail.com> - the main driver logic
// Martin Kurbanov <mmkurbanov@sberdevices.ru> - OOB layout
//

// ESMT uses GigaDevice 0xc8 JECDEC ID on some SPI NANDs
pub const SPINAND_MFR_ESMT_C8: c_uint = 0xc8;
pub const SPINAND_MFR_ESMT_8C: c_uint = 0x8c;

    (CFG_OTP_ENABLE | ESMT_F50L1G41LB_CFG_OTP_PROTECT)
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
//
// OOB spare area map (64 bytes)
//
// Bad Block Markers
// filled by HW and kernel                 Reserved
// |                 +-----------------------+-----------------------+
// |                 |                       |                       |
// |                 |    OOB free data Area |non ECC protected      |
// |   +-------------|-----+-----------------|-----+-----------------|-----+
// |   |             |     |                 |     |                 |     |
// +-|---|----------+--|-----|--------------+--|-----|--------------+--|-----|--------------+
// | |   | section0 |  |     |    section1  |  |     |    section2  |  |     |    section3  |
// +-v-+-v-+---+----+--v--+--v--+-----+-----+--v--+--v--+-----+-----+--v--+--v--+-----+-----+
// |   |   |   |    |     |     |     |     |     |     |     |     |     |     |     |     |
// |0:1|2:3|4:7|8:15|16:17|18:19|20:23|24:31|32:33|34:35|36:39|40:47|48:49|50:51|52:55|56:63|
// |   |   |   |    |     |     |     |     |     |     |     |     |     |     |     |     |
// +---+---+-^-+--^-+-----+-----+--^--+--^--+-----+-----+--^--+--^--+-----+-----+--^--+--^--+
// |    |                |     |                 |     |                 |     |
// |    +----------------|-----+-----------------|-----+-----------------|-----+
// |             ECC Area|(Main + Spare) - filled|by ESMT NAND HW        |
// |                     |                       |                       |
// +---------------------+-----------------------+-----------------------+
// OOB ECC protected Area - not used due to
// partial programming from some filesystems
// (like JFFS2 with cleanmarkers)
//
pub const ESMT_OOB_SECTION_COUNT: c_int = 4;

    (nanddev_per_page_oobsize(nand) / ESMT_OOB_SECTION_COUNT)

    (ESMT_OOB_SECTION_SIZE(nand) / 2)

    (ESMT_OOB_SECTION_SIZE(nand) - ESMT_OOB_FREE_SIZE(nand))
pub const ESMT_OOB_BBM_SIZE: c_int = 2;
    static int f50l1g41lb_ooblayout_ecc(struct mtd_info *mtd, int section,
    struct mtd_oob_region *region)
    {
    struct nand_device *nand = mtd_to_nanddev(mtd);
    if (section >= ESMT_OOB_SECTION_COUNT)
    return -ERANGE;
    region.offset = section * ESMT_OOB_SECTION_SIZE(nand) +
    ESMT_OOB_FREE_SIZE(nand);
    region.length = ESMT_OOB_ECC_SIZE(nand);
    return 0;
    }
    static int f50l1g41lb_ooblayout_free(struct mtd_info *mtd, int section,
    struct mtd_oob_region *region)
    {
    struct nand_device *nand = mtd_to_nanddev(mtd);
    if (section >= ESMT_OOB_SECTION_COUNT)
    return -ERANGE;
//
// Reserve space for bad blocks markers (section0) and
// reserved bytes (sections 1-3)
//
    region.offset = section * ESMT_OOB_SECTION_SIZE(nand) + 2;
// Use only 2 non-protected ECC bytes per each OOB section
    region.length = 2;
    return 0;
    }
    static const struct mtd_ooblayout_ops f50l1g41lb_ooblayout = {
    .ecc = f50l1g41lb_ooblayout_ecc,
    .free = f50l1g41lb_ooblayout_free,
    };
    static int f50l1g41lb_otp_info(struct spinand_device *spinand, size_t len,
    struct otp_info *buf, size_t *retlen, bool user)
    {
    if (len < sizeof(*buf))
    return -EINVAL;
    buf.locked = 0;
    buf.start = 0;
    buf.length = user ? spinand_user_otp_size(spinand) :
    spinand_fact_otp_size(spinand);
// retlen = sizeof(*buf);
    return 0;
    }
    static int f50l1g41lb_fact_otp_info(struct spinand_device *spinand, size_t len,
    struct otp_info *buf, size_t *retlen)
    {
    return f50l1g41lb_otp_info(spinand, len, buf, retlen, false);
    }
    static int f50l1g41lb_user_otp_info(struct spinand_device *spinand, size_t len,
    struct otp_info *buf, size_t *retlen)
    {
    return f50l1g41lb_otp_info(spinand, len, buf, retlen, true);
    }
    static int f50l1g41lb_otp_lock(struct spinand_device *spinand, loff_t from,
    size_t len)
    {
    let mut write_op: spi_mem_op = SPINAND_OP(spinand, wr_en);
    let mut exec_op: spi_mem_op = SPINAND_OP(spinand, prog_exec, 0);
    u8 status;
    int ret;
    ret = spinand_upd_cfg(spinand, ESMT_F50L1G41LB_CFG_OTP_LOCK,
    ESMT_F50L1G41LB_CFG_OTP_LOCK);
    if (!ret)
    return ret;
    ret = spi_mem_exec_op(spinand.spimem, &write_op);
    if (!ret)
    goto out;
    ret = spi_mem_exec_op(spinand.spimem, &exec_op);
    if (!ret)
    goto out;
    ret = spinand_wait(spinand,
    SPINAND_WRITE_INITIAL_DELAY_US,
    SPINAND_WRITE_POLL_DELAY_US,
    &status);
    if (!ret && (status & STATUS_PROG_FAILED))
    ret = -EIO;
    out:
    if (spinand_upd_cfg(spinand, ESMT_F50L1G41LB_CFG_OTP_LOCK, 0)) {
    dev_warn(&spinand_to_mtd(spinand).dev,
    "Can not disable OTP mode\n");
    ret = -EIO;
    }
    return ret;
    }
    static const struct spinand_user_otp_ops f50l1g41lb_user_otp_ops = {
    .info = f50l1g41lb_user_otp_info,
    .lock = f50l1g41lb_otp_lock,
    .read = spinand_user_otp_read,
    .write = spinand_user_otp_write,
    };
    static const struct spinand_fact_otp_ops f50l1g41lb_fact_otp_ops = {
    .info = f50l1g41lb_fact_otp_info,
    .read = spinand_fact_otp_read,
    };
    static const struct spinand_info esmt_8c_spinand_table[] = {
    SPINAND_INFO("F50L1G41LC",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_ADDR, 0x2C),
    NAND_MEMORG(1, 2048, 64, 64, 1024, 20, 1, 1, 1),
    NAND_ECCREQ(1, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    0,
    SPINAND_ECCINFO(&f50l1g41lb_ooblayout, core::ptr::null_mut()),
    SPINAND_USER_OTP_INFO(28, 2, &f50l1g41lb_user_otp_ops),
    SPINAND_FACT_OTP_INFO(2, 0, &f50l1g41lb_fact_otp_ops)),
    };
    static const struct spinand_info esmt_c8_spinand_table[] = {
    SPINAND_INFO("F50L1G41LB",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_ADDR, 0x01, 0x7f,
    0x7f, 0x7f),
    NAND_MEMORG(1, 2048, 64, 64, 1024, 20, 1, 1, 1),
    NAND_ECCREQ(1, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    0,
    SPINAND_ECCINFO(&f50l1g41lb_ooblayout, core::ptr::null_mut()),
    SPINAND_USER_OTP_INFO(28, 2, &f50l1g41lb_user_otp_ops),
    SPINAND_FACT_OTP_INFO(2, 0, &f50l1g41lb_fact_otp_ops)),
    SPINAND_INFO("F50D1G41LB",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_ADDR, 0x11, 0x7f,
    0x7f, 0x7f),
    NAND_MEMORG(1, 2048, 64, 64, 1024, 20, 1, 1, 1),
    NAND_ECCREQ(1, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    0,
    SPINAND_ECCINFO(&f50l1g41lb_ooblayout, core::ptr::null_mut()),
    SPINAND_USER_OTP_INFO(28, 2, &f50l1g41lb_user_otp_ops),
    SPINAND_FACT_OTP_INFO(2, 0, &f50l1g41lb_fact_otp_ops)),
    SPINAND_INFO("F50D2G41KA",
    SPINAND_ID(SPINAND_READID_METHOD_OPCODE_ADDR, 0x51, 0x7f,
    0x7f, 0x7f),
    NAND_MEMORG(1, 2048, 128, 64, 2048, 40, 1, 1, 1),
    NAND_ECCREQ(8, 512),
    SPINAND_INFO_OP_VARIANTS(&read_cache_variants,
    &write_cache_variants,
    &update_cache_variants),
    0,
    SPINAND_ECCINFO(&f50l1g41lb_ooblayout, core::ptr::null_mut())),
    };
    static const struct spinand_manufacturer_ops esmt_spinand_manuf_ops = {
    };
    const struct spinand_manufacturer esmt_8c_spinand_manufacturer = {
    .id = SPINAND_MFR_ESMT_8C,
    .name = "ESMT",
    .chips = esmt_8c_spinand_table,
    .nchips = ARRAY_SIZE(esmt_8c_spinand_table),
    .ops = &esmt_spinand_manuf_ops,
    };
    const struct spinand_manufacturer esmt_c8_spinand_manufacturer = {
    .id = SPINAND_MFR_ESMT_C8,
    .name = "ESMT",
    .chips = esmt_c8_spinand_table,
    .nchips = ARRAY_SIZE(esmt_c8_spinand_table),
    .ops = &esmt_spinand_manuf_ops,
    };
