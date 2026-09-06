//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/raw/ingenic/jz4780_bch.c
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
// JZ4780 BCH controller driver
//
// Copyright (c) 2015 Imagination Technologies
// Author: Alex Smith <alex.smith@imgtec.com>
//

pub const BCH_BHCR: c_uint = 0x0;
pub const BCH_BHCCR: c_uint = 0x8;
pub const BCH_BHCNT: c_uint = 0xc;
pub const BCH_BHDR: c_uint = 0x10;
pub const BCH_BHPAR0: c_uint = 0x14;
pub const BCH_BHERR0: c_uint = 0x84;
pub const BCH_BHINT: c_uint = 0x184;
pub const BCH_BHINTES: c_uint = 0x188;
pub const BCH_BHINTEC: c_uint = 0x18c;
pub const BCH_BHINTE: c_uint = 0x190;
pub const BCH_BHCR_BSEL_SHIFT: c_int = 4;

pub const BCH_BHCNT_PARITYSIZE_SHIFT: c_int = 16;

pub const BCH_BHCNT_BLOCKSIZE_SHIFT: c_int = 0;

pub const BCH_BHERR_MASK_SHIFT: c_int = 16;

pub const BCH_BHERR_INDEX_SHIFT: c_int = 0;

pub const BCH_BHINT_ERRC_SHIFT: c_int = 24;

pub const BCH_BHINT_TERRC_SHIFT: c_int = 16;

// Timeout for BCH calculation/correction.
pub const BCH_TIMEOUT_US: c_int = 100000;
    static void jz4780_bch_reset(struct ingenic_ecc *bch,
    struct ingenic_ecc_params *params, bool encode)
    {
    u32 reg;
// Clear interrupt status.
    writel(readl(bch.base + BCH_BHINT), bch.base + BCH_BHINT);
// Set up BCH count register.
    reg = params.size << BCH_BHCNT_BLOCKSIZE_SHIFT;
    reg |= params.bytes << BCH_BHCNT_PARITYSIZE_SHIFT;
    writel(reg, bch.base + BCH_BHCNT);
// Initialise and enable BCH.
    reg = BCH_BHCR_BCHE | BCH_BHCR_INIT;
    reg |= params.strength << BCH_BHCR_BSEL_SHIFT;
    if (encode)
    reg |= BCH_BHCR_ENCE;
    writel(reg, bch.base + BCH_BHCR);
    }
#[no_mangle]
unsafe extern "C" fn jz4780_bch_disable(bch: *mut ingenic_ecc) {
    static void jz4780_bch_disable(struct ingenic_ecc *bch)
    {
    writel(readl(bch.base + BCH_BHINT), bch.base + BCH_BHINT);
    writel(BCH_BHCR_BCHE, bch.base + BCH_BHCCR);
    }
    static void jz4780_bch_write_data(struct ingenic_ecc *bch, const void *buf,
    size_t size)
    {
    let mut size32: usize = size / sizeof(u32);
    let mut size8: usize = size % sizeof(u32);
    const u32 *src32;
    const u8 *src8;
    src32 = (const u32 *)buf;
    while (size32--)
    writel(*src32++, bch.base + BCH_BHDR);
    src8 = (const u8 *)src32;
    while (size8--)
    writeb(*src8++, bch.base + BCH_BHDR);
    }
    static void jz4780_bch_read_parity(struct ingenic_ecc *bch, void *buf,
    size_t size)
    {
    let mut size32: usize = size / sizeof(u32);
    let mut size8: usize = size % sizeof(u32);
    u32 *dest32;
    u8 *dest8;
    u32 val, offset = 0;
    dest32 = (u32 *)buf;
    while (size32--) {
// dest32++ = readl(bch->base + BCH_BHPAR0 + offset);
    offset += sizeof(u32);
    }
    dest8 = (u8 *)dest32;
    val = readl(bch.base + BCH_BHPAR0 + offset);
    switch (size8) {
    case 3:
    dest8[2] = (val >> 16) & 0xff;
    fallthrough;
    case 2:
    dest8[1] = (val >> 8) & 0xff;
    fallthrough;
    case 1:
    dest8[0] = val & 0xff;
    break;
    }
    }
    static bool jz4780_bch_wait_complete(struct ingenic_ecc *bch, unsigned int irq,
    u32 *status)
    {
    u32 reg;
    int ret;
//
// While we could use interrupts here and sleep until the operation
// completes, the controller works fairly quickly (usually a few
// microseconds) and so the overhead of sleeping until we get an
// interrupt quite noticeably decreases performance.
//
    ret = readl_poll_timeout(bch.base + BCH_BHINT, reg,
    (reg & irq) == irq, 0, BCH_TIMEOUT_US);
    if (ret)
    return false;
    if (status)
// status = reg;
    writel(reg, bch.base + BCH_BHINT);
    return true;
    }
    static int jz4780_calculate(struct ingenic_ecc *bch,
    struct ingenic_ecc_params *params,
    const u8 *buf, u8 *ecc_code)
    {
    let mut ret: c_int = 0;
    mutex_lock(&bch.lock);
    jz4780_bch_reset(bch, params, true);
    jz4780_bch_write_data(bch, buf, params.size);
    if (jz4780_bch_wait_complete(bch, BCH_BHINT_ENCF, core::ptr::null_mut())) {
    jz4780_bch_read_parity(bch, ecc_code, params.bytes);
    } else {
    dev_err(bch.dev, "timed out while calculating ECC\n");
    ret = -ETIMEDOUT;
    }
    jz4780_bch_disable(bch);
    mutex_unlock(&bch.lock);
    return ret;
    }
    static int jz4780_correct(struct ingenic_ecc *bch,
    struct ingenic_ecc_params *params,
    u8 *buf, u8 *ecc_code)
    {
    u32 reg, mask, index;
    int i, ret, count;
    mutex_lock(&bch.lock);
    jz4780_bch_reset(bch, params, false);
    jz4780_bch_write_data(bch, buf, params.size);
    jz4780_bch_write_data(bch, ecc_code, params.bytes);
    if (!jz4780_bch_wait_complete(bch, BCH_BHINT_DECF, &reg)) {
    dev_err(bch.dev, "timed out while correcting data\n");
    ret = -ETIMEDOUT;
    goto out;
    }
    if (reg & BCH_BHINT_UNCOR) {
    dev_warn(bch.dev, "uncorrectable ECC error\n");
    ret = -EBADMSG;
    goto out;
    }
// Correct any detected errors.
    if (reg & BCH_BHINT_ERR) {
    count = (reg & BCH_BHINT_ERRC_MASK) >> BCH_BHINT_ERRC_SHIFT;
    ret = (reg & BCH_BHINT_TERRC_MASK) >> BCH_BHINT_TERRC_SHIFT;
    for (i = 0; i < count; i++) {
    reg = readl(bch.base + BCH_BHERR0 + (i * 4));
    mask = (reg & BCH_BHERR_MASK_MASK) >>
    BCH_BHERR_MASK_SHIFT;
    index = (reg & BCH_BHERR_INDEX_MASK) >>
    BCH_BHERR_INDEX_SHIFT;
    buf[(index * 2) + 0] ^= mask;
    buf[(index * 2) + 1] ^= mask >> 8;
    }
    } else {
    ret = 0;
    }
    out:
    jz4780_bch_disable(bch);
    mutex_unlock(&bch.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn jz4780_bch_probe(pdev: *mut platform_device) -> c_int {
    static int jz4780_bch_probe(struct platform_device *pdev)
    {
    struct ingenic_ecc *bch;
    int ret;
    ret = ingenic_ecc_probe(pdev);
    if (ret)
    return ret;
    bch = platform_get_drvdata(pdev);
    clk_set_rate(bch.clk, BCH_CLK_RATE);
    return 0;
    }
    static const struct ingenic_ecc_ops jz4780_bch_ops = {
    .disable = jz4780_bch_disable,
    .calculate = jz4780_calculate,
    .correct = jz4780_correct,
    };
    static const struct of_device_id jz4780_bch_dt_match[] = {
    { .compatible = "ingenic,jz4780-bch", .data = &jz4780_bch_ops },
    {},
    };
    MODULE_DEVICE_TABLE(of, jz4780_bch_dt_match);
    static struct platform_driver jz4780_bch_driver = {
    .probe		= jz4780_bch_probe,
    .driver	= {
    .name	= "jz4780-bch",
    .of_match_table = jz4780_bch_dt_match,
    },
    };
    module_platform_driver(jz4780_bch_driver);
    MODULE_AUTHOR("Alex Smith <alex@alex-smith.me.uk>");
    MODULE_AUTHOR("Harvey Hunt <harveyhuntnexus@gmail.com>");
    MODULE_DESCRIPTION("Ingenic JZ4780 BCH error correction driver");
    MODULE_LICENSE("GPL v2");
