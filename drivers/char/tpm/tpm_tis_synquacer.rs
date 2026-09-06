//! Automatically rewritten from C to Rust
//! Source: drivers/char/tpm/tpm_tis_synquacer.c
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
// Copyright (C) 2020 Linaro Ltd.
//
// This device driver implements MMIO TPM on SynQuacer Platform.
//

//
// irq > 0 means: use irq $irq;
// irq = 0 means: autoprobe for an irq;
// irq = -1 means: no irq support
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpm_tis_synquacer_info {
    pub res: resource,
    pub irq: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpm_tis_synquacer_phy {
    pub priv: tpm_tis_data,
    pub iobase: *mut void __iomem,
}

    static inline struct tpm_tis_synquacer_phy *to_tpm_tis_tcg_phy(struct tpm_tis_data *data)
    {
    return container_of(data, struct tpm_tis_synquacer_phy, priv);
    }
    static int tpm_tis_synquacer_read_bytes(struct tpm_tis_data *data, u32 addr,
    u16 len, u8 *result,
    enum tpm_tis_io_mode io_mode)
    {
    struct tpm_tis_synquacer_phy *phy = to_tpm_tis_tcg_phy(data);
    switch (io_mode) {
    case TPM_TIS_PHYS_8:
    while (len--)
// result++ = ioread8(phy->iobase + addr);
    break;
    case TPM_TIS_PHYS_16:
    result[1] = ioread8(phy.iobase + addr + 1);
    result[0] = ioread8(phy.iobase + addr);
    break;
    case TPM_TIS_PHYS_32:
    result[3] = ioread8(phy.iobase + addr + 3);
    result[2] = ioread8(phy.iobase + addr + 2);
    result[1] = ioread8(phy.iobase + addr + 1);
    result[0] = ioread8(phy.iobase + addr);
    break;
    }
    return 0;
    }
    static int tpm_tis_synquacer_write_bytes(struct tpm_tis_data *data, u32 addr,
    u16 len, const u8 *value,
    enum tpm_tis_io_mode io_mode)
    {
    struct tpm_tis_synquacer_phy *phy = to_tpm_tis_tcg_phy(data);
    switch (io_mode) {
    case TPM_TIS_PHYS_8:
    while (len--)
    iowrite8(*value++, phy.iobase + addr);
    break;
    case TPM_TIS_PHYS_16:
    return -EINVAL;
    case TPM_TIS_PHYS_32:
//
// Due to the limitation of SPI controller on SynQuacer,
// 16/32 bits access must be done in byte-wise and descending order.
//
    iowrite8(value[3], phy.iobase + addr + 3);
    iowrite8(value[2], phy.iobase + addr + 2);
    iowrite8(value[1], phy.iobase + addr + 1);
    iowrite8(value[0], phy.iobase + addr);
    break;
    }
    return 0;
    }
    static const struct tpm_tis_phy_ops tpm_tcg_bw = {
    .read_bytes	= tpm_tis_synquacer_read_bytes,
    .write_bytes	= tpm_tis_synquacer_write_bytes,
    };
    static int tpm_tis_synquacer_init(struct device *dev,
    struct tpm_tis_synquacer_info *tpm_info)
    {
    struct tpm_tis_synquacer_phy *phy;
    phy = devm_kzalloc(dev, sizeof(struct tpm_tis_synquacer_phy), GFP_KERNEL);
    if (phy == core::ptr::null_mut())
    return -ENOMEM;
    phy.iobase = devm_ioremap_resource(dev, &tpm_info.res);
    if (IS_ERR(phy.iobase))
    return PTR_ERR(phy.iobase);
    return tpm_tis_core_init(dev, &phy.priv, tpm_info.irq, &tpm_tcg_bw,
    ACPI_HANDLE(dev));
    }
    static SIMPLE_DEV_PM_OPS(tpm_tis_synquacer_pm, tpm_pm_suspend, tpm_tis_resume);
#[no_mangle]
unsafe extern "C" fn tpm_tis_synquacer_probe(pdev: *mut platform_device) -> c_int {
    static int tpm_tis_synquacer_probe(struct platform_device *pdev)
    {
    let mut tpm_info: tpm_tis_synquacer_info = {};
    struct resource *res;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (res == core::ptr::null_mut()) {
    dev_err(&pdev.dev, "no memory resource defined\n");
    return -ENODEV;
    }
    tpm_info.res = *res;
    tpm_info.irq = -1;
    return tpm_tis_synquacer_init(&pdev.dev, &tpm_info);
    }
#[no_mangle]
unsafe extern "C" fn tpm_tis_synquacer_remove(pdev: *mut platform_device) {
    static void tpm_tis_synquacer_remove(struct platform_device *pdev)
    {
    struct tpm_chip *chip = dev_get_drvdata(&pdev.dev);
    tpm_chip_unregister(chip);
    tpm_tis_remove(chip);
    }

    static const struct of_device_id tis_synquacer_of_platform_match[] = {
    {.compatible = "socionext,synquacer-tpm-mmio"},
    {},
    };
    MODULE_DEVICE_TABLE(of, tis_synquacer_of_platform_match);

    static const struct acpi_device_id tpm_synquacer_acpi_tbl[] = {
    { "SCX0009" },
    {},
    };
    MODULE_DEVICE_TABLE(acpi, tpm_synquacer_acpi_tbl);

    static struct platform_driver tis_synquacer_drv = {
    .probe = tpm_tis_synquacer_probe,
    .remove = tpm_tis_synquacer_remove,
    .driver = {
    .name		= "tpm_tis_synquacer",
    .pm		= &tpm_tis_synquacer_pm,
    .of_match_table = of_match_ptr(tis_synquacer_of_platform_match),
    .acpi_match_table = ACPI_PTR(tpm_synquacer_acpi_tbl),
    },
    };
    module_platform_driver(tis_synquacer_drv);
    MODULE_DESCRIPTION("TPM MMIO Driver for Socionext SynQuacer platform");
    MODULE_LICENSE("GPL");
