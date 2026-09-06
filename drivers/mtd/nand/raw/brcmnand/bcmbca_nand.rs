//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/raw/brcmnand/bcmbca_nand.c
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
// Copyright © 2015 Broadcom Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcmbca_nand_soc {
    pub soc: brcmnand_soc,
    pub base: *mut void __iomem,
}

pub const BCMBCA_NAND_INT_STATUS: c_uint = 0x00;
pub const BCMBCA_NAND_INT_EN: c_uint = 0x04;
    enum {
    BCMBCA_CTLRDY		= BIT(4),
    };

pub const ALIGN_REQ: c_int = 8;

pub const ALIGN_REQ: c_int = 4;

#[no_mangle]
pub unsafe extern "C" fn bcmbca_nand_is_buf_aligned(flash_cache: *mut c_void, buffer: *mut c_void) -> bool {
    static inline bool bcmbca_nand_is_buf_aligned(void *flash_cache,  void *buffer)
    {
    return IS_ALIGNED((uintptr_t)buffer, ALIGN_REQ) &&
    IS_ALIGNED((uintptr_t)flash_cache, ALIGN_REQ);
    }
#[no_mangle]
unsafe extern "C" fn bcmbca_nand_intc_ack(soc: *mut brcmnand_soc) -> bool {
    static bool bcmbca_nand_intc_ack(struct brcmnand_soc *soc)
    {
    struct bcmbca_nand_soc *priv =
    container_of(soc, struct bcmbca_nand_soc, soc);
    void __iomem *mmio = priv.base + BCMBCA_NAND_INT_STATUS;
    let mut val: u32 = brcmnand_readl(mmio);
    if (val & BCMBCA_CTLRDY) {
    brcmnand_writel(val & ~BCMBCA_CTLRDY, mmio);
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn bcmbca_nand_intc_set(soc: *mut brcmnand_soc, en: bool) {
    static void bcmbca_nand_intc_set(struct brcmnand_soc *soc, bool en)
    {
    struct bcmbca_nand_soc *priv =
    container_of(soc, struct bcmbca_nand_soc, soc);
    void __iomem *mmio = priv.base + BCMBCA_NAND_INT_EN;
    let mut val: u32 = brcmnand_readl(mmio);
    if (en)
    val |= BCMBCA_CTLRDY;
    else
    val &= ~BCMBCA_CTLRDY;
    brcmnand_writel(val, mmio);
    }
    static void bcmbca_read_data_bus(struct brcmnand_soc *soc,
    void __iomem *flash_cache,  u32 *buffer, int fc_words)
    {
//
// memcpy can do unaligned aligned access depending on source
// and dest address, which is incompatible with nand cache. Fallback
// to the memcpy_fromio in such case
//
    if (bcmbca_nand_is_buf_aligned((void  *)flash_cache, buffer))
    memcpy((void *)buffer, (void  *)flash_cache, fc_words * 4);
    else
    memcpy_fromio((void *)buffer, flash_cache, fc_words * 4);
    }
#[no_mangle]
unsafe extern "C" fn bcmbca_nand_probe(pdev: *mut platform_device) -> c_int {
    static int bcmbca_nand_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct bcmbca_nand_soc *priv;
    struct brcmnand_soc *soc;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    soc = &priv.soc;
    priv.base = devm_platform_ioremap_resource_byname(pdev, "nand-int-base");
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    soc.ctlrdy_ack = bcmbca_nand_intc_ack;
    soc.ctlrdy_set_enabled = bcmbca_nand_intc_set;
    soc.read_data_bus = bcmbca_read_data_bus;
    return brcmnand_probe(pdev, soc);
    }
    static const struct of_device_id bcmbca_nand_of_match[] = {
    { .compatible = "brcm,nand-bcm63138" },
    {},
    };
    MODULE_DEVICE_TABLE(of, bcmbca_nand_of_match);
    static struct platform_driver bcmbca_nand_driver = {
    .probe			= bcmbca_nand_probe,
    .remove			= brcmnand_remove,
    .driver = {
    .name		= "bcmbca_nand",
    .pm		= &brcmnand_pm_ops,
    .of_match_table	= bcmbca_nand_of_match,
    }
    };
    module_platform_driver(bcmbca_nand_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Brian Norris");
    MODULE_DESCRIPTION("NAND driver for BCMBCA");
