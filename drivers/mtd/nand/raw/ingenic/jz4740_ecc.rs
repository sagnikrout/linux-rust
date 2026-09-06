//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/raw/ingenic/jz4740_ecc.c
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
// JZ4740 ECC controller driver
//
// Copyright (c) 2019 Paul Cercueil <paul@crapouillou.net>
//
// based on jz4740-nand.c
//

pub const JZ_REG_NAND_ECC_CTRL: c_uint = 0x00;
pub const JZ_REG_NAND_DATA: c_uint = 0x04;
pub const JZ_REG_NAND_PAR0: c_uint = 0x08;
pub const JZ_REG_NAND_PAR1: c_uint = 0x0C;
pub const JZ_REG_NAND_PAR2: c_uint = 0x10;
pub const JZ_REG_NAND_IRQ_STAT: c_uint = 0x14;
pub const JZ_REG_NAND_IRQ_CTRL: c_uint = 0x18;

    static const uint8_t empty_block_ecc[] = {
    0xcd, 0x9d, 0x90, 0x58, 0xf4, 0x8b, 0xff, 0xb7, 0x6f
    };
#[no_mangle]
unsafe extern "C" fn jz4740_ecc_reset(ecc: *mut ingenic_ecc, calc_ecc: bool) {
    static void jz4740_ecc_reset(struct ingenic_ecc *ecc, bool calc_ecc)
    {
    uint32_t reg;
// Clear interrupt status
    writel(0, ecc.base + JZ_REG_NAND_IRQ_STAT);
// Initialize and enable ECC hardware
    reg = readl(ecc.base + JZ_REG_NAND_ECC_CTRL);
    reg |= JZ_NAND_ECC_CTRL_RESET;
    reg |= JZ_NAND_ECC_CTRL_ENABLE;
    reg |= JZ_NAND_ECC_CTRL_RS;
    if (calc_ecc) /* calculate ECC from data */
    reg |= JZ_NAND_ECC_CTRL_ENCODING;
    else /* correct data from ECC */
    reg &= ~JZ_NAND_ECC_CTRL_ENCODING;
    writel(reg, ecc.base + JZ_REG_NAND_ECC_CTRL);
    }
    static int jz4740_ecc_calculate(struct ingenic_ecc *ecc,
    struct ingenic_ecc_params *params,
    const u8 *buf, u8 *ecc_code)
    {
    uint32_t reg, status;
    let mut timeout: c_uint = 1000;
    int i;
    jz4740_ecc_reset(ecc, true);
    do {
    status = readl(ecc.base + JZ_REG_NAND_IRQ_STAT);
    } while (!(status & JZ_NAND_STATUS_ENC_FINISH) && --timeout);
    if (timeout == 0)
    return -ETIMEDOUT;
    reg = readl(ecc.base + JZ_REG_NAND_ECC_CTRL);
    reg &= ~JZ_NAND_ECC_CTRL_ENABLE;
    writel(reg, ecc.base + JZ_REG_NAND_ECC_CTRL);
    for (i = 0; i < params.bytes; ++i)
    ecc_code[i] = readb(ecc.base + JZ_REG_NAND_PAR0 + i);
//
// If the written data is completely 0xff, we also want to write 0xff as
// ECC, otherwise we will get in trouble when doing subpage writes.
//
    if (memcmp(ecc_code, empty_block_ecc, sizeof(empty_block_ecc)) == 0)
    memset(ecc_code, 0xff, sizeof(empty_block_ecc));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jz_nand_correct_data(buf: *mut u8, index: c_int, mask: c_int) {
    static void jz_nand_correct_data(uint8_t *buf, int index, int mask)
    {
    let mut offset: c_int = index & 0x7;
    uint16_t data;
    index += (index >> 3);
    data = buf[index];
    data |= buf[index + 1] << 8;
    mask ^= (data >> offset) & 0x1ff;
    data &= ~(0x1ff << offset);
    data |= (mask << offset);
    buf[index] = data & 0xff;
    buf[index + 1] = (data >> 8) & 0xff;
    }
    static int jz4740_ecc_correct(struct ingenic_ecc *ecc,
    struct ingenic_ecc_params *params,
    u8 *buf, u8 *ecc_code)
    {
    int i, error_count, index;
    uint32_t reg, status, error;
    let mut timeout: c_uint = 1000;
    jz4740_ecc_reset(ecc, false);
    for (i = 0; i < params.bytes; ++i)
    writeb(ecc_code[i], ecc.base + JZ_REG_NAND_PAR0 + i);
    reg = readl(ecc.base + JZ_REG_NAND_ECC_CTRL);
    reg |= JZ_NAND_ECC_CTRL_PAR_READY;
    writel(reg, ecc.base + JZ_REG_NAND_ECC_CTRL);
    do {
    status = readl(ecc.base + JZ_REG_NAND_IRQ_STAT);
    } while (!(status & JZ_NAND_STATUS_DEC_FINISH) && --timeout);
    if (timeout == 0)
    return -ETIMEDOUT;
    reg = readl(ecc.base + JZ_REG_NAND_ECC_CTRL);
    reg &= ~JZ_NAND_ECC_CTRL_ENABLE;
    writel(reg, ecc.base + JZ_REG_NAND_ECC_CTRL);
    if (status & JZ_NAND_STATUS_ERROR) {
    if (status & JZ_NAND_STATUS_UNCOR_ERROR)
    return -EBADMSG;
    error_count = (status & JZ_NAND_STATUS_ERR_COUNT) >> 29;
    for (i = 0; i < error_count; ++i) {
    error = readl(ecc.base + JZ_REG_NAND_ERR(i));
    index = ((error >> 16) & 0x1ff) - 1;
    if (index >= 0 && index < params.size)
    jz_nand_correct_data(buf, index, error & 0x1ff);
    }
    return error_count;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jz4740_ecc_disable(ecc: *mut ingenic_ecc) {
    static void jz4740_ecc_disable(struct ingenic_ecc *ecc)
    {
    u32 reg;
    writel(0, ecc.base + JZ_REG_NAND_IRQ_STAT);
    reg = readl(ecc.base + JZ_REG_NAND_ECC_CTRL);
    reg &= ~JZ_NAND_ECC_CTRL_ENABLE;
    writel(reg, ecc.base + JZ_REG_NAND_ECC_CTRL);
    }
    static const struct ingenic_ecc_ops jz4740_ecc_ops = {
    .disable = jz4740_ecc_disable,
    .calculate = jz4740_ecc_calculate,
    .correct = jz4740_ecc_correct,
    };
    static const struct of_device_id jz4740_ecc_dt_match[] = {
    { .compatible = "ingenic,jz4740-ecc", .data = &jz4740_ecc_ops },
    {},
    };
    MODULE_DEVICE_TABLE(of, jz4740_ecc_dt_match);
    static struct platform_driver jz4740_ecc_driver = {
    .probe		= ingenic_ecc_probe,
    .driver	= {
    .name	= "jz4740-ecc",
    .of_match_table = jz4740_ecc_dt_match,
    },
    };
    module_platform_driver(jz4740_ecc_driver);
    MODULE_AUTHOR("Paul Cercueil <paul@crapouillou.net>");
    MODULE_DESCRIPTION("Ingenic JZ4740 ECC controller driver");
    MODULE_LICENSE("GPL v2");
