//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/raw/brcmnand/bcm6368_nand.c
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
// Copyright 2015 Simon Arlott
//
// Derived from bcm63138_nand.c:
// Copyright © 2015 Broadcom Corporation
//
// Derived from bcm963xx_4.12L.06B_consumer/shared/opensource/include/bcm963xx/63268_map_part.h:
// Copyright 2000-2010 Broadcom Corporation
//
// Derived from bcm963xx_4.12L.06B_consumer/shared/opensource/flash/nandflash.c:
// Copyright 2000-2010 Broadcom Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm6368_nand_soc {
    pub soc: brcmnand_soc,
    pub base: *mut void __iomem,
}

pub const BCM6368_NAND_INT: c_uint = 0x00;
pub const BCM6368_NAND_STATUS_SHIFT: c_int = 0;

pub const BCM6368_NAND_ENABLE_SHIFT: c_int = 16;

pub const BCM6368_NAND_BASE_ADDR0: c_uint = 0x04;
pub const BCM6368_NAND_BASE_ADDR1: c_uint = 0x0c;
    enum {
    BCM6368_NP_READ		= BIT(0),
    BCM6368_BLOCK_ERASE	= BIT(1),
    BCM6368_COPY_BACK	= BIT(2),
    BCM6368_PAGE_PGM	= BIT(3),
    BCM6368_CTRL_READY	= BIT(4),
    BCM6368_DEV_RBPIN	= BIT(5),
    BCM6368_ECC_ERR_UNC	= BIT(6),
    BCM6368_ECC_ERR_CORR	= BIT(7),
    };
#[no_mangle]
unsafe extern "C" fn bcm6368_nand_intc_ack(soc: *mut brcmnand_soc) -> bool {
    static bool bcm6368_nand_intc_ack(struct brcmnand_soc *soc)
    {
    struct bcm6368_nand_soc *priv =
    container_of(soc, struct bcm6368_nand_soc, soc);
    void __iomem *mmio = priv.base + BCM6368_NAND_INT;
    let mut val: u32 = brcmnand_readl(mmio);
    if (val & (BCM6368_CTRL_READY << BCM6368_NAND_STATUS_SHIFT)) {
// Ack interrupt
    val &= ~BCM6368_NAND_STATUS_MASK;
    val |= BCM6368_CTRL_READY << BCM6368_NAND_STATUS_SHIFT;
    brcmnand_writel(val, mmio);
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn bcm6368_nand_intc_set(soc: *mut brcmnand_soc, en: bool) {
    static void bcm6368_nand_intc_set(struct brcmnand_soc *soc, bool en)
    {
    struct bcm6368_nand_soc *priv =
    container_of(soc, struct bcm6368_nand_soc, soc);
    void __iomem *mmio = priv.base + BCM6368_NAND_INT;
    let mut val: u32 = brcmnand_readl(mmio);
// Don't ack any interrupts
    val &= ~BCM6368_NAND_STATUS_MASK;
    if (en)
    val |= BCM6368_CTRL_READY << BCM6368_NAND_ENABLE_SHIFT;
    else
    val &= ~(BCM6368_CTRL_READY << BCM6368_NAND_ENABLE_SHIFT);
    brcmnand_writel(val, mmio);
    }
#[no_mangle]
unsafe extern "C" fn bcm6368_nand_probe(pdev: *mut platform_device) -> c_int {
    static int bcm6368_nand_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct bcm6368_nand_soc *priv;
    struct brcmnand_soc *soc;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    soc = &priv.soc;
    priv.base = devm_platform_ioremap_resource_byname(pdev, "nand-int-base");
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    soc.ctlrdy_ack = bcm6368_nand_intc_ack;
    soc.ctlrdy_set_enabled = bcm6368_nand_intc_set;
// Disable and ack all interrupts
    brcmnand_writel(0, priv.base + BCM6368_NAND_INT);
    brcmnand_writel(BCM6368_NAND_STATUS_MASK,
    priv.base + BCM6368_NAND_INT);
    return brcmnand_probe(pdev, soc);
    }
    static const struct of_device_id bcm6368_nand_of_match[] = {
    { .compatible = "brcm,nand-bcm6368" },
    {},
    };
    MODULE_DEVICE_TABLE(of, bcm6368_nand_of_match);
    static struct platform_driver bcm6368_nand_driver = {
    .probe			= bcm6368_nand_probe,
    .remove			= brcmnand_remove,
    .driver = {
    .name		= "bcm6368_nand",
    .pm		= &brcmnand_pm_ops,
    .of_match_table	= bcm6368_nand_of_match,
    }
    };
    module_platform_driver(bcm6368_nand_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Simon Arlott");
    MODULE_DESCRIPTION("NAND driver for BCM6368");
