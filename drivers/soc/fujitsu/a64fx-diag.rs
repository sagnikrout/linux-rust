//! Automatically rewritten from C to Rust
//! Source: drivers/soc/fujitsu/a64fx-diag.c
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
// A64FX diag driver.
// Copyright (c) 2022 Fujitsu Ltd.
//

pub const A64FX_DIAG_IRQ: c_int = 1;
pub const BMC_DIAG_INTERRUPT_ENABLE: c_uint = 0x40;
pub const BMC_DIAG_INTERRUPT_STATUS: c_uint = 0x44;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct a64fx_diag_priv {
    pub mmsc_reg_base: *mut void __iomem,
    pub irq: c_int,
    pub has_nmi: bool,
}

#[no_mangle]
unsafe extern "C" fn a64fx_diag_handler_nmi(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t a64fx_diag_handler_nmi(int irq, void *dev_id)
    {
    nmi_panic(core::ptr::null_mut(), "a64fx_diag: interrupt received\n");
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn a64fx_diag_handler_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t a64fx_diag_handler_irq(int irq, void *dev_id)
    {
    panic("a64fx_diag: interrupt received\n");
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn a64fx_diag_interrupt_clear(priv: *mut a64fx_diag_priv) {
    static void a64fx_diag_interrupt_clear(struct a64fx_diag_priv *priv)
    {
    void __iomem *diag_status_reg_addr;
    u32 mmsc;
    diag_status_reg_addr = priv.mmsc_reg_base + BMC_DIAG_INTERRUPT_STATUS;
    mmsc = readl(diag_status_reg_addr);
    if (mmsc & BMC_DIAG_INTERRUPT_MASK)
    writel(BMC_DIAG_INTERRUPT_MASK, diag_status_reg_addr);
    }
#[no_mangle]
unsafe extern "C" fn a64fx_diag_interrupt_enable(priv: *mut a64fx_diag_priv) {
    static void a64fx_diag_interrupt_enable(struct a64fx_diag_priv *priv)
    {
    void __iomem *diag_enable_reg_addr;
    u32 mmsc;
    diag_enable_reg_addr = priv.mmsc_reg_base + BMC_DIAG_INTERRUPT_ENABLE;
    mmsc = readl(diag_enable_reg_addr);
    if (!(mmsc & BMC_DIAG_INTERRUPT_MASK)) {
    mmsc |= BMC_DIAG_INTERRUPT_MASK;
    writel(mmsc, diag_enable_reg_addr);
    }
    }
#[no_mangle]
unsafe extern "C" fn a64fx_diag_interrupt_disable(priv: *mut a64fx_diag_priv) {
    static void a64fx_diag_interrupt_disable(struct a64fx_diag_priv *priv)
    {
    void __iomem *diag_enable_reg_addr;
    u32 mmsc;
    diag_enable_reg_addr = priv.mmsc_reg_base + BMC_DIAG_INTERRUPT_ENABLE;
    mmsc = readl(diag_enable_reg_addr);
    if (mmsc & BMC_DIAG_INTERRUPT_MASK) {
    mmsc &= ~BMC_DIAG_INTERRUPT_MASK;
    writel(mmsc, diag_enable_reg_addr);
    }
    }
#[no_mangle]
unsafe extern "C" fn a64fx_diag_probe(pdev: *mut platform_device) -> c_int {
    static int a64fx_diag_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct a64fx_diag_priv *priv;
    unsigned long irq_flags;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (priv == core::ptr::null_mut())
    return -ENOMEM;
    priv.mmsc_reg_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.mmsc_reg_base))
    return PTR_ERR(priv.mmsc_reg_base);
    priv.irq = platform_get_irq(pdev, A64FX_DIAG_IRQ);
    if (priv.irq < 0)
    return priv.irq;
    platform_set_drvdata(pdev, priv);
    irq_flags = IRQF_PERCPU | IRQF_NOBALANCING | IRQF_NO_AUTOEN |
    IRQF_NO_THREAD;
    ret = request_nmi(priv.irq, &a64fx_diag_handler_nmi, irq_flags,
    "a64fx_diag_nmi", core::ptr::null_mut());
    if (ret) {
    ret = request_irq(priv.irq, &a64fx_diag_handler_irq,
    irq_flags, "a64fx_diag_irq", core::ptr::null_mut());
    if (ret) {
    dev_err(dev, "cannot register IRQ %d\n", ret);
    return ret;
    }
    enable_irq(priv.irq);
    } else {
    enable_nmi(priv.irq);
    priv.has_nmi = true;
    }
    a64fx_diag_interrupt_clear(priv);
    a64fx_diag_interrupt_enable(priv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn a64fx_diag_remove(pdev: *mut platform_device) {
    static void a64fx_diag_remove(struct platform_device *pdev)
    {
    struct a64fx_diag_priv *priv = platform_get_drvdata(pdev);
    a64fx_diag_interrupt_disable(priv);
    a64fx_diag_interrupt_clear(priv);
    if (priv.has_nmi)
    free_nmi(priv.irq, core::ptr::null_mut());
    else
    free_irq(priv.irq, core::ptr::null_mut());
    }
    static const struct acpi_device_id a64fx_diag_acpi_match[] = {
    { "FUJI2007", 0 },
    { },
    };
    MODULE_DEVICE_TABLE(acpi, a64fx_diag_acpi_match);
    static struct platform_driver a64fx_diag_driver = {
    .driver = {
    .name = "a64fx_diag_driver",
    .acpi_match_table = ACPI_PTR(a64fx_diag_acpi_match),
    },
    .probe = a64fx_diag_probe,
    .remove = a64fx_diag_remove,
    };
    module_platform_driver(a64fx_diag_driver);
    MODULE_AUTHOR("Hitomi Hasegawa <hasegawa-hitomi@fujitsu.com>");
    MODULE_DESCRIPTION("A64FX diag driver");
