//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/raw/ndfc.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Overview:
// Platform independent driver for NDFC (NanD Flash Controller)
// integrated into EP440 cores
//
// Ported to an OF platform driver by Sean MacLennan
//
// The NDFC supports multiple chips, but this driver only supports a
// single chip since I do not have access to any boards with
// multiple chips.
//
// Author: Thomas Gleixner
//
// Copyright 2006 IBM
// Copyright 2008 PIKA Technologies
// Sean MacLennan <smaclennan@pikatech.com>
//

pub const NDFC_MAX_CS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ndfc_controller {
    pub ofdev: *mut platform_device,
    pub ndfcbase: *mut void __iomem,
    pub chip: nand_chip,
    pub chip_select: c_int,
    pub ndfc_control: nand_controller,
}

    static struct ndfc_controller ndfc_ctrl[NDFC_MAX_CS];
#[no_mangle]
unsafe extern "C" fn ndfc_select_chip(nchip: *mut nand_chip, chip: c_int) {
    static void ndfc_select_chip(struct nand_chip *nchip, int chip)
    {
    uint32_t ccr;
    struct ndfc_controller *ndfc = nand_get_controller_data(nchip);
    ccr = ioread32be(ndfc.ndfcbase + NDFC_CCR);
    if (chip >= 0) {
    ccr &= ~NDFC_CCR_BS_MASK;
    ccr |= NDFC_CCR_BS(chip + ndfc.chip_select);
    } else
    ccr |= NDFC_CCR_RESET_CE;
    iowrite32be(ccr, ndfc.ndfcbase + NDFC_CCR);
    }
#[no_mangle]
unsafe extern "C" fn ndfc_hwcontrol(chip: *mut nand_chip, cmd: c_int, ctrl: c_uint) {
    static void ndfc_hwcontrol(struct nand_chip *chip, int cmd, unsigned int ctrl)
    {
    struct ndfc_controller *ndfc = nand_get_controller_data(chip);
    if (cmd == NAND_CMD_NONE)
    return;
    if (ctrl & NAND_CLE)
    writel(cmd & 0xFF, ndfc.ndfcbase + NDFC_CMD);
    else
    writel(cmd & 0xFF, ndfc.ndfcbase + NDFC_ALE);
    }
#[no_mangle]
unsafe extern "C" fn ndfc_ready(chip: *mut nand_chip) -> c_int {
    static int ndfc_ready(struct nand_chip *chip)
    {
    struct ndfc_controller *ndfc = nand_get_controller_data(chip);
    return ioread32be(ndfc.ndfcbase + NDFC_STAT) & NDFC_STAT_IS_READY;
    }
#[no_mangle]
unsafe extern "C" fn ndfc_enable_hwecc(chip: *mut nand_chip, mode: c_int) {
    static void ndfc_enable_hwecc(struct nand_chip *chip, int mode)
    {
    uint32_t ccr;
    struct ndfc_controller *ndfc = nand_get_controller_data(chip);
    ccr = ioread32be(ndfc.ndfcbase + NDFC_CCR);
    ccr |= NDFC_CCR_RESET_ECC;
    iowrite32be(ccr, ndfc.ndfcbase + NDFC_CCR);
    wmb();
    }
    static int ndfc_calculate_ecc(struct nand_chip *chip,
    const u_char *dat, u_char *ecc_code)
    {
    struct ndfc_controller *ndfc = nand_get_controller_data(chip);
    uint32_t ecc;
    uint8_t *p = (uint8_t *)&ecc;
    wmb();
    ecc = ioread32be(ndfc.ndfcbase + NDFC_ECC);
// The NDFC uses Smart Media (SMC) bytes order
    ecc_code[0] = p[1];
    ecc_code[1] = p[2];
    ecc_code[2] = p[3];
    return 0;
    }
//
// Speedups for buffer read/write/verify
//
// NDFC allows 32bit read/write of data. So we can speed up the buffer
// functions. No further checking, as nand_base will always read/write
// page aligned.
//
#[no_mangle]
unsafe extern "C" fn ndfc_read_buf(chip: *mut nand_chip, buf: *mut u8, len: c_int) {
    static void ndfc_read_buf(struct nand_chip *chip, uint8_t *buf, int len)
    {
    struct ndfc_controller *ndfc = nand_get_controller_data(chip);
    uint32_t *p = (uint32_t *) buf;
    for(;len > 0; len -= 4)
// p++ = ioread32be(ndfc->ndfcbase + NDFC_DATA);
    }
#[no_mangle]
unsafe extern "C" fn ndfc_write_buf(chip: *mut nand_chip, buf: *const u8, len: c_int) {
    static void ndfc_write_buf(struct nand_chip *chip, const uint8_t *buf, int len)
    {
    struct ndfc_controller *ndfc = nand_get_controller_data(chip);
    uint32_t *p = (uint32_t *) buf;
    for(;len > 0; len -= 4)
    iowrite32be(*p++, ndfc.ndfcbase + NDFC_DATA);
    }
//
// Initialize chip structure
//
    static int ndfc_chip_init(struct ndfc_controller *ndfc,
    struct device_node *node)
    {
    struct device_node *flash_np;
    struct nand_chip *chip = &ndfc.chip;
    struct mtd_info *mtd = nand_to_mtd(chip);
    int ret;
    chip.legacy.IO_ADDR_R = ndfc.ndfcbase + NDFC_DATA;
    chip.legacy.IO_ADDR_W = ndfc.ndfcbase + NDFC_DATA;
    chip.legacy.cmd_ctrl = ndfc_hwcontrol;
    chip.legacy.dev_ready = ndfc_ready;
    chip.legacy.select_chip = ndfc_select_chip;
    chip.legacy.chip_delay = 50;
    chip.controller = &ndfc.ndfc_control;
    chip.legacy.read_buf = ndfc_read_buf;
    chip.legacy.write_buf = ndfc_write_buf;
    chip.ecc.correct = rawnand_sw_hamming_correct;
    chip.ecc.hwctl = ndfc_enable_hwecc;
    chip.ecc.calculate = ndfc_calculate_ecc;
    chip.ecc.engine_type = NAND_ECC_ENGINE_TYPE_ON_HOST;
    chip.ecc.size = 256;
    chip.ecc.bytes = 3;
    chip.ecc.strength = 1;
    nand_set_controller_data(chip, ndfc);
    mtd.dev.parent = &ndfc.ofdev.dev;
    flash_np = of_get_next_child(node, core::ptr::null_mut());
    if (!flash_np)
    return -ENODEV;
    nand_set_flash_node(chip, flash_np);
    mtd.name = kasprintf(GFP_KERNEL, "%s.%pOFn", dev_name(&ndfc.ofdev.dev),
    flash_np);
    if (!mtd.name) {
    ret = -ENOMEM;
    goto err;
    }
    ret = nand_scan(chip, 1);
    if (ret)
    goto err;
    ret = mtd_device_register(mtd, core::ptr::null_mut(), 0);
    err:
    of_node_put(flash_np);
    if (ret)
    kfree(mtd.name);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ndfc_probe(ofdev: *mut platform_device) -> c_int {
    static int ndfc_probe(struct platform_device *ofdev)
    {
    struct ndfc_controller *ndfc;
    const __be32 *reg;
    u32 ccr;
    u32 cs;
    int err, len = 0;
// Read the reg property to get the chip select
    reg = of_get_property(ofdev.dev.of_node, "reg", &len);
    if (reg == core::ptr::null_mut() || len != 12) {
    dev_err(&ofdev.dev, "unable read reg property (%d)\n", len);
    return -ENOENT;
    }
    cs = be32_to_cpu(reg[0]);
    if (cs >= NDFC_MAX_CS) {
    dev_err(&ofdev.dev, "invalid CS number (%d)\n", cs);
    return -EINVAL;
    }
    ndfc = &ndfc_ctrl[cs];
    ndfc.chip_select = cs;
    nand_controller_init(&ndfc.ndfc_control);
    ndfc.ofdev = ofdev;
    dev_set_drvdata(&ofdev.dev, ndfc);
    ndfc.ndfcbase = of_iomap(ofdev.dev.of_node, 0);
    if (!ndfc.ndfcbase) {
    dev_err(&ofdev.dev, "failed to get memory\n");
    return -EIO;
    }
    ccr = NDFC_CCR_BS(ndfc.chip_select);
// It is ok if ccr does not exist - just default to 0
    reg = of_get_property(ofdev.dev.of_node, "ccr", core::ptr::null_mut());
    if (reg)
    ccr |= be32_to_cpup(reg);
    iowrite32be(ccr, ndfc.ndfcbase + NDFC_CCR);
// Set the bank settings if given
    reg = of_get_property(ofdev.dev.of_node, "bank-settings", core::ptr::null_mut());
    if (reg) {
    let mut offset: c_int = NDFC_BCFG0 + (ndfc.chip_select << 2);
    iowrite32be(be32_to_cpup(reg), ndfc.ndfcbase + offset);
    }
    err = ndfc_chip_init(ndfc, ofdev.dev.of_node);
    if (err) {
    iounmap(ndfc.ndfcbase);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ndfc_remove(ofdev: *mut platform_device) {
    static void ndfc_remove(struct platform_device *ofdev)
    {
    struct ndfc_controller *ndfc = dev_get_drvdata(&ofdev.dev);
    struct nand_chip *chip = &ndfc.chip;
    struct mtd_info *mtd = nand_to_mtd(chip);
    int ret;
    ret = mtd_device_unregister(mtd);
    WARN_ON(ret);
    nand_cleanup(chip);
    kfree(mtd.name);
    }
    static const struct of_device_id ndfc_match[] = {
    { .compatible = "ibm,ndfc", },
    {}
    };
    MODULE_DEVICE_TABLE(of, ndfc_match);
    static struct platform_driver ndfc_driver = {
    .driver = {
    .name = "ndfc",
    .of_match_table = ndfc_match,
    },
    .probe = ndfc_probe,
    .remove = ndfc_remove,
    };
    module_platform_driver(ndfc_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Thomas Gleixner <tglx@kernel.org>");
    MODULE_DESCRIPTION("OF Platform driver for NDFC");
