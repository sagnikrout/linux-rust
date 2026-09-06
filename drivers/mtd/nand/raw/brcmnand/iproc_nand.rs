//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/raw/brcmnand/iproc_nand.c
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
pub struct iproc_nand_soc {
    pub soc: brcmnand_soc,
    pub idm_base: *mut void __iomem,
    pub ext_base: *mut void __iomem,
    pub idm_lock: spinlock_t,
}

pub const IPROC_NAND_CTLR_READY_OFFSET: c_uint = 0x10;

pub const IPROC_NAND_IO_CTRL_OFFSET: c_uint = 0x00;

#[no_mangle]
unsafe extern "C" fn iproc_nand_intc_ack(soc: *mut brcmnand_soc) -> bool {
    static bool iproc_nand_intc_ack(struct brcmnand_soc *soc)
    {
    struct iproc_nand_soc *priv =
    container_of(soc, struct iproc_nand_soc, soc);
    void __iomem *mmio = priv.ext_base + IPROC_NAND_CTLR_READY_OFFSET;
    let mut val: u32 = brcmnand_readl(mmio);
    if (val & IPROC_NAND_CTLR_READY) {
    brcmnand_writel(IPROC_NAND_CTLR_READY, mmio);
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn iproc_nand_intc_set(soc: *mut brcmnand_soc, en: bool) {
    static void iproc_nand_intc_set(struct brcmnand_soc *soc, bool en)
    {
    struct iproc_nand_soc *priv =
    container_of(soc, struct iproc_nand_soc, soc);
    void __iomem *mmio = priv.idm_base + IPROC_NAND_IO_CTRL_OFFSET;
    u32 val;
    unsigned long flags;
    spin_lock_irqsave(&priv.idm_lock, flags);
    val = brcmnand_readl(mmio);
    if (en)
    val |= IPROC_NAND_INT_CTRL_READ_ENABLE;
    else
    val &= ~IPROC_NAND_INT_CTRL_READ_ENABLE;
    brcmnand_writel(val, mmio);
    spin_unlock_irqrestore(&priv.idm_lock, flags);
    }
    static void iproc_nand_apb_access(struct brcmnand_soc *soc, bool prepare,
    bool is_param)
    {
    struct iproc_nand_soc *priv =
    container_of(soc, struct iproc_nand_soc, soc);
    void __iomem *mmio = priv.idm_base + IPROC_NAND_IO_CTRL_OFFSET;
    u32 val;
    unsigned long flags;
    spin_lock_irqsave(&priv.idm_lock, flags);
    val = brcmnand_readl(mmio);
//
// In the case of BE or when dealing with NAND data, alway configure
// the APB bus to LE mode before accessing the FIFO and back to BE mode
// after the access is done
//
    if (IS_ENABLED(CONFIG_CPU_BIG_ENDIAN) || !is_param) {
    if (prepare)
    val |= IPROC_NAND_APB_LE_MODE;
    else
    val &= ~IPROC_NAND_APB_LE_MODE;
    } else { /* when in LE accessing the parameter page, keep APB in BE */
    val &= ~IPROC_NAND_APB_LE_MODE;
    }
    brcmnand_writel(val, mmio);
    spin_unlock_irqrestore(&priv.idm_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn iproc_nand_probe(pdev: *mut platform_device) -> c_int {
    static int iproc_nand_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct iproc_nand_soc *priv;
    struct brcmnand_soc *soc;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    soc = &priv.soc;
    spin_lock_init(&priv.idm_lock);
    priv.idm_base = devm_platform_ioremap_resource_byname(pdev, "iproc-idm");
    if (IS_ERR(priv.idm_base))
    return PTR_ERR(priv.idm_base);
    priv.ext_base = devm_platform_ioremap_resource_byname(pdev, "iproc-ext");
    if (IS_ERR(priv.ext_base))
    return PTR_ERR(priv.ext_base);
    soc.ctlrdy_ack = iproc_nand_intc_ack;
    soc.ctlrdy_set_enabled = iproc_nand_intc_set;
    soc.prepare_data_bus = iproc_nand_apb_access;
    return brcmnand_probe(pdev, soc);
    }
    static const struct of_device_id iproc_nand_of_match[] = {
    { .compatible = "brcm,nand-iproc" },
    {},
    };
    MODULE_DEVICE_TABLE(of, iproc_nand_of_match);
    static struct platform_driver iproc_nand_driver = {
    .probe			= iproc_nand_probe,
    .remove			= brcmnand_remove,
    .driver = {
    .name		= "iproc_nand",
    .pm		= &brcmnand_pm_ops,
    .of_match_table	= iproc_nand_of_match,
    }
    };
    module_platform_driver(iproc_nand_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Brian Norris");
    MODULE_AUTHOR("Ray Jui");
    MODULE_DESCRIPTION("NAND driver for Broadcom IPROC-based SoCs");
