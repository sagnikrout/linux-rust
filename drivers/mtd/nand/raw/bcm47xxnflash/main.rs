//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/raw/bcm47xxnflash/main.c
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
// BCM47XX NAND flash driver
//
// Copyright (C) 2012 Rafał Miłecki <zajec5@gmail.com>
//

    MODULE_DESCRIPTION("NAND flash driver for BCMA bus");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Rafał Miłecki");
    static const char *probes[] = { "bcm47xxpart", core::ptr::null_mut() };
#[no_mangle]
unsafe extern "C" fn bcm47xxnflash_probe(pdev: *mut platform_device) -> c_int {
    static int bcm47xxnflash_probe(struct platform_device *pdev)
    {
    struct bcma_nflash *nflash = dev_get_platdata(&pdev.dev);
    struct bcm47xxnflash *b47n;
    struct mtd_info *mtd;
    let mut err: c_int = 0;
    b47n = devm_kzalloc(&pdev.dev, sizeof(*b47n), GFP_KERNEL);
    if (!b47n)
    return -ENOMEM;
    nand_set_controller_data(&b47n.nand_chip, b47n);
    mtd = nand_to_mtd(&b47n.nand_chip);
    mtd.dev.parent = &pdev.dev;
    b47n.cc = container_of(nflash, struct bcma_drv_cc, nflash);
    if (b47n.cc.core.bus.chipinfo.id == BCMA_CHIP_ID_BCM4706) {
    err = bcm47xxnflash_ops_bcm4706_init(b47n);
    } else {
    pr_err("Device not supported\n");
    err = -ENOTSUPP;
    }
    if (err) {
    pr_err("Initialization failed: %d\n", err);
    return err;
    }
    platform_set_drvdata(pdev, b47n);
    err = mtd_device_parse_register(mtd, probes, core::ptr::null_mut(), core::ptr::null_mut(), 0);
    if (err) {
    pr_err("Failed to register MTD device: %d\n", err);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm47xxnflash_remove(pdev: *mut platform_device) {
    static void bcm47xxnflash_remove(struct platform_device *pdev)
    {
    struct bcm47xxnflash *nflash = platform_get_drvdata(pdev);
    struct nand_chip *chip = &nflash.nand_chip;
    int ret;
    ret = mtd_device_unregister(nand_to_mtd(chip));
    WARN_ON(ret);
    nand_cleanup(chip);
    }
    static struct platform_driver bcm47xxnflash_driver = {
    .probe	= bcm47xxnflash_probe,
    .remove = bcm47xxnflash_remove,
    .driver = {
    .name = "bcma_nflash",
    },
    };
    module_platform_driver(bcm47xxnflash_driver);
