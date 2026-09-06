//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/raw/sharpsl.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2004 Richard Purdie
// Copyright (C) 2008 Dmitry Baryshkov
//
// Based on Sharp's NAND driver sharp_sl.c
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sharpsl_nand {
    pub controller: nand_controller,
    pub chip: nand_chip,
    pub io: *mut void __iomem,
}

    static inline struct sharpsl_nand *mtd_to_sharpsl(struct mtd_info *mtd)
    {
    return container_of(mtd_to_nand(mtd), struct sharpsl_nand, chip);
    }
// register offset
pub const ECCLPLB: c_uint = 0x00	/* line parity 7 - 0 bit */;
pub const ECCLPUB: c_uint = 0x04	/* line parity 15 - 8 bit */;
pub const ECCCP: c_uint = 0x08	/* column parity 5 - 0 bit */;
pub const ECCCNTR: c_uint = 0x0C	/* ECC byte counter */;
pub const ECCCLRR: c_uint = 0x10	/* cleare ECC */;
pub const FLASHIO: c_uint = 0x14	/* Flash I/O */;
pub const FLASHCTL: c_uint = 0x18	/* Flash Control */;
// Flash control bit

//
// hardware specific access to control-lines
// ctrl:
// NAND_CNE: bit 0 -> ! bit 0 & 4
// NAND_CLE: bit 1 -> bit 1
// NAND_ALE: bit 2 -> bit 2
//
    static void sharpsl_nand_hwcontrol(struct nand_chip *chip, int cmd,
    unsigned int ctrl)
    {
    struct sharpsl_nand *sharpsl = mtd_to_sharpsl(nand_to_mtd(chip));
    if (ctrl & NAND_CTRL_CHANGE) {
    let mut bits: c_uchar = ctrl & 0x07;
    bits |= (ctrl & 0x01) << 4;
    bits ^= 0x11;
    writeb((readb(sharpsl.io + FLASHCTL) & ~0x17) | bits, sharpsl.io + FLASHCTL);
    }
    if (cmd != NAND_CMD_NONE)
    writeb(cmd, chip.legacy.IO_ADDR_W);
    }
#[no_mangle]
unsafe extern "C" fn sharpsl_nand_dev_ready(chip: *mut nand_chip) -> c_int {
    static int sharpsl_nand_dev_ready(struct nand_chip *chip)
    {
    struct sharpsl_nand *sharpsl = mtd_to_sharpsl(nand_to_mtd(chip));
    return !((readb(sharpsl.io + FLASHCTL) & FLRYBY) == 0);
    }
#[no_mangle]
unsafe extern "C" fn sharpsl_nand_enable_hwecc(chip: *mut nand_chip, mode: c_int) {
    static void sharpsl_nand_enable_hwecc(struct nand_chip *chip, int mode)
    {
    struct sharpsl_nand *sharpsl = mtd_to_sharpsl(nand_to_mtd(chip));
    writeb(0, sharpsl.io + ECCCLRR);
    }
    static int sharpsl_nand_calculate_ecc(struct nand_chip *chip,
    const u_char * dat, u_char * ecc_code)
    {
    struct sharpsl_nand *sharpsl = mtd_to_sharpsl(nand_to_mtd(chip));
    ecc_code[0] = ~readb(sharpsl.io + ECCLPUB);
    ecc_code[1] = ~readb(sharpsl.io + ECCLPLB);
    ecc_code[2] = (~readb(sharpsl.io + ECCCP) << 2) | 0x03;
    return readb(sharpsl.io + ECCCNTR) != 0;
    }
#[no_mangle]
unsafe extern "C" fn sharpsl_attach_chip(chip: *mut nand_chip) -> c_int {
    static int sharpsl_attach_chip(struct nand_chip *chip)
    {
    if (chip.ecc.engine_type != NAND_ECC_ENGINE_TYPE_ON_HOST)
    return 0;
    chip.ecc.size = 256;
    chip.ecc.bytes = 3;
    chip.ecc.strength = 1;
    chip.ecc.hwctl = sharpsl_nand_enable_hwecc;
    chip.ecc.calculate = sharpsl_nand_calculate_ecc;
    chip.ecc.correct = rawnand_sw_hamming_correct;
    return 0;
    }
    static const struct nand_controller_ops sharpsl_ops = {
    .attach_chip = sharpsl_attach_chip,
    };
//
// Main initialization routine
//
#[no_mangle]
unsafe extern "C" fn sharpsl_nand_probe(pdev: *mut platform_device) -> c_int {
    static int sharpsl_nand_probe(struct platform_device *pdev)
    {
    struct nand_chip *this;
    struct mtd_info *mtd;
    struct resource *r;
    let mut err: c_int = 0;
    struct sharpsl_nand *sharpsl;
    struct sharpsl_nand_platform_data *data = dev_get_platdata(&pdev.dev);
    if (!data) {
    dev_err(&pdev.dev, "no platform data!\n");
    return -EINVAL;
    }
// Allocate memory for MTD device structure and private data
    sharpsl = kzalloc_obj(struct sharpsl_nand);
    if (!sharpsl)
    return -ENOMEM;
    r = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!r) {
    dev_err(&pdev.dev, "no io memory resource defined!\n");
    err = -ENODEV;
    goto err_get_res;
    }
// map physical address
    sharpsl.io = ioremap(r.start, resource_size(r));
    if (!sharpsl.io) {
    dev_err(&pdev.dev, "ioremap to access Sharp SL NAND chip failed\n");
    err = -EIO;
    goto err_ioremap;
    }
// Get pointer to private data
    this = (struct nand_chip *)(&sharpsl.chip);
    nand_controller_init(&sharpsl.controller);
    sharpsl.controller.ops = &sharpsl_ops;
    this.controller = &sharpsl.controller;
// Link the private data with the MTD structure
    mtd = nand_to_mtd(this);
    mtd.dev.parent = &pdev.dev;
    mtd_set_ooblayout(mtd, data.ecc_layout);
    platform_set_drvdata(pdev, sharpsl);
//
// PXA initialize
//
    writeb(readb(sharpsl.io + FLASHCTL) | FLWP, sharpsl.io + FLASHCTL);
// Set address of NAND IO lines
    this.legacy.IO_ADDR_R = sharpsl.io + FLASHIO;
    this.legacy.IO_ADDR_W = sharpsl.io + FLASHIO;
// Set address of hardware control function
    this.legacy.cmd_ctrl = sharpsl_nand_hwcontrol;
    this.legacy.dev_ready = sharpsl_nand_dev_ready;
// 15 us command delay time
    this.legacy.chip_delay = 15;
    this.badblock_pattern = data.badblock_pattern;
// Scan to find existence of the device
    err = nand_scan(this, 1);
    if (err)
    goto err_scan;
// Register the partitions
    mtd.name = "sharpsl-nand";
    err = mtd_device_parse_register(mtd, data.part_parsers, core::ptr::null_mut(),
    data.partitions, data.nr_partitions);
    if (err)
    goto err_add;
// Return happy
    return 0;
    err_add:
    nand_cleanup(this);
    err_scan:
    iounmap(sharpsl.io);
    err_ioremap:
    err_get_res:
    kfree(sharpsl);
    return err;
    }
//
// Clean up routine
//
#[no_mangle]
unsafe extern "C" fn sharpsl_nand_remove(pdev: *mut platform_device) {
    static void sharpsl_nand_remove(struct platform_device *pdev)
    {
    struct sharpsl_nand *sharpsl = platform_get_drvdata(pdev);
    struct nand_chip *chip = &sharpsl.chip;
    int ret;
// Unregister device
    ret = mtd_device_unregister(nand_to_mtd(chip));
    WARN_ON(ret);
// Release resources
    nand_cleanup(chip);
    iounmap(sharpsl.io);
// Free the driver's structure
    kfree(sharpsl);
    }
    static struct platform_driver sharpsl_nand_driver = {
    .driver = {
    .name	= "sharpsl-nand",
    },
    .probe		= sharpsl_nand_probe,
    .remove		= sharpsl_nand_remove,
    };
    module_platform_driver(sharpsl_nand_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Richard Purdie <rpurdie@rpsys.net>");
    MODULE_DESCRIPTION("Device specific logic for NAND flash on Sharp SL-C7xx Series");
