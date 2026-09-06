//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-iproc-qspi.c
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
// Copyright 2016 Broadcom Limited
//

pub const INTR_BASE_BIT_SHIFT: c_uint = 0x02;
pub const INTR_COUNT: c_uint = 0x07;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_iproc_intc {
    pub soc_intc: bcm_qspi_soc_intc,
    pub pdev: *mut platform_device,
    pub int_reg: *mut void __iomem,
    pub int_status_reg: *mut void __iomem,
    pub soclock: spinlock_t,
    pub big_endian: bool,
}

#[no_mangle]
unsafe extern "C" fn bcm_iproc_qspi_get_l2_int_status(soc_intc: *mut bcm_qspi_soc_intc) -> u32 {
    static u32 bcm_iproc_qspi_get_l2_int_status(struct bcm_qspi_soc_intc *soc_intc)
    {
    struct bcm_iproc_intc *priv =
    container_of(soc_intc, struct bcm_iproc_intc, soc_intc);
    void __iomem *mmio = priv.int_status_reg;
    int i;
    let mut val: u32 = 0, sts = 0;
    for (i = 0; i < INTR_COUNT; i++) {
    if (bcm_qspi_readl(priv.big_endian, mmio + (i * 4)))
    val |= 1UL << i;
    }
    if (val & INTR_MSPI_DONE_MASK)
    sts |= MSPI_DONE;
    if (val & BSPI_LR_INTERRUPTS_ALL)
    sts |= BSPI_DONE;
    if (val & BSPI_LR_INTERRUPTS_ERROR)
    sts |= BSPI_ERR;
    return sts;
    }
#[no_mangle]
unsafe extern "C" fn bcm_iproc_qspi_int_ack(soc_intc: *mut bcm_qspi_soc_intc, type: c_int) {
    static void bcm_iproc_qspi_int_ack(struct bcm_qspi_soc_intc *soc_intc, int type)
    {
    struct bcm_iproc_intc *priv =
    container_of(soc_intc, struct bcm_iproc_intc, soc_intc);
    void __iomem *mmio = priv.int_status_reg;
    let mut mask: u32 = get_qspi_mask(type);
    int i;
    for (i = 0; i < INTR_COUNT; i++) {
    if (mask & (1UL << i))
    bcm_qspi_writel(priv.big_endian, 1, mmio + (i * 4));
    }
    }
    static void bcm_iproc_qspi_int_set(struct bcm_qspi_soc_intc *soc_intc, int type,
    bool en)
    {
    struct bcm_iproc_intc *priv =
    container_of(soc_intc, struct bcm_iproc_intc, soc_intc);
    void __iomem *mmio = priv.int_reg;
    let mut mask: u32 = get_qspi_mask(type);
    u32 val;
    unsigned long flags;
    spin_lock_irqsave(&priv.soclock, flags);
    val = bcm_qspi_readl(priv.big_endian, mmio);
    if (en)
    val = val | (mask << INTR_BASE_BIT_SHIFT);
    else
    val = val & ~(mask << INTR_BASE_BIT_SHIFT);
    bcm_qspi_writel(priv.big_endian, val, mmio);
    spin_unlock_irqrestore(&priv.soclock, flags);
    }
#[no_mangle]
unsafe extern "C" fn bcm_iproc_probe(pdev: *mut platform_device) -> c_int {
    static int bcm_iproc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct bcm_iproc_intc *priv;
    struct bcm_qspi_soc_intc *soc_intc;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    soc_intc = &priv.soc_intc;
    priv.pdev = pdev;
    spin_lock_init(&priv.soclock);
    priv.int_reg = devm_platform_ioremap_resource_byname(pdev, "intr_regs");
    if (IS_ERR(priv.int_reg))
    return PTR_ERR(priv.int_reg);
    priv.int_status_reg = devm_platform_ioremap_resource_byname(pdev,
    "intr_status_reg");
    if (IS_ERR(priv.int_status_reg))
    return PTR_ERR(priv.int_status_reg);
    priv.big_endian = of_device_is_big_endian(dev.of_node);
    bcm_iproc_qspi_int_ack(soc_intc, MSPI_BSPI_DONE);
    bcm_iproc_qspi_int_set(soc_intc, MSPI_BSPI_DONE, false);
    soc_intc.bcm_qspi_int_ack = bcm_iproc_qspi_int_ack;
    soc_intc.bcm_qspi_int_set = bcm_iproc_qspi_int_set;
    soc_intc.bcm_qspi_get_int_status = bcm_iproc_qspi_get_l2_int_status;
    return bcm_qspi_probe(pdev, soc_intc);
    }
#[no_mangle]
unsafe extern "C" fn bcm_iproc_remove(pdev: *mut platform_device) {
    static void bcm_iproc_remove(struct platform_device *pdev)
    {
    bcm_qspi_remove(pdev);
    }
    static const struct of_device_id bcm_iproc_of_match[] = {
    { .compatible = "brcm,spi-nsp-qspi" },
    { .compatible = "brcm,spi-ns2-qspi" },
    {},
    };
    MODULE_DEVICE_TABLE(of, bcm_iproc_of_match);
    static struct platform_driver bcm_iproc_driver = {
    .probe			= bcm_iproc_probe,
    .remove			= bcm_iproc_remove,
    .driver = {
    .name		= "bcm_iproc",
    .pm		= pm_sleep_ptr(&bcm_qspi_pm_ops),
    .of_match_table = bcm_iproc_of_match,
    }
    };
    module_platform_driver(bcm_iproc_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Kamal Dasu");
    MODULE_DESCRIPTION("SPI flash driver for Broadcom iProc SoCs");
