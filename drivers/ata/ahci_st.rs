//! Automatically rewritten from C to Rust
//! Source: drivers/ata/ahci_st.c
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
// Copyright (C) 2012 STMicroelectronics Limited
//
// Authors: Francesco Virlinzi <francesco.virlinzi@st.com>
// Alexandre Torgue <alexandre.torgue@st.com>
//

pub const ST_AHCI_OOBR: c_uint = 0xbc;

pub const ST_AHCI_OOBR_CWMIN_SHIFT: c_int = 24;
pub const ST_AHCI_OOBR_CWMAX_SHIFT: c_int = 16;
pub const ST_AHCI_OOBR_CIMIN_SHIFT: c_int = 8;
pub const ST_AHCI_OOBR_CIMAX_SHIFT: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_ahci_drv_data {
    pub pwr: *mut reset_control,
    pub sw_rst: *mut reset_control,
    pub pwr_rst: *mut reset_control,
}

#[no_mangle]
unsafe extern "C" fn st_ahci_configure_oob(mmio: *mut void __iomem) {
    static void st_ahci_configure_oob(void __iomem *mmio)
    {
    unsigned long old_val, new_val;
    new_val = (0x02 << ST_AHCI_OOBR_CWMIN_SHIFT) |
    (0x04 << ST_AHCI_OOBR_CWMAX_SHIFT) |
    (0x08 << ST_AHCI_OOBR_CIMIN_SHIFT) |
    (0x0C << ST_AHCI_OOBR_CIMAX_SHIFT);
    old_val = readl(mmio + ST_AHCI_OOBR);
    writel(old_val | ST_AHCI_OOBR_WE, mmio + ST_AHCI_OOBR);
    writel(new_val | ST_AHCI_OOBR_WE, mmio + ST_AHCI_OOBR);
    writel(new_val, mmio + ST_AHCI_OOBR);
    }
    static int st_ahci_deassert_resets(struct ahci_host_priv *hpriv,
    struct device *dev)
    {
    struct st_ahci_drv_data *drv_data = hpriv.plat_data;
    int err;
    if (drv_data.pwr) {
    err = reset_control_deassert(drv_data.pwr);
    if (err) {
    dev_err(dev, "unable to bring out of pwrdwn\n");
    return err;
    }
    }
    if (drv_data.sw_rst) {
    err = reset_control_deassert(drv_data.sw_rst);
    if (err) {
    dev_err(dev, "unable to bring out of sw-rst\n");
    return err;
    }
    }
    if (drv_data.pwr_rst) {
    err = reset_control_deassert(drv_data.pwr_rst);
    if (err) {
    dev_err(dev, "unable to bring out of pwr-rst\n");
    return err;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn st_ahci_host_stop(host: *mut ata_host) {
    static void st_ahci_host_stop(struct ata_host *host)
    {
    struct ahci_host_priv *hpriv = host.private_data;
    struct st_ahci_drv_data *drv_data = hpriv.plat_data;
    struct device *dev = host.dev;
    int err;
    if (drv_data.pwr) {
    err = reset_control_assert(drv_data.pwr);
    if (err)
    dev_err(dev, "unable to pwrdwn\n");
    }
    ahci_platform_disable_resources(hpriv);
    }
    static int st_ahci_probe_resets(struct ahci_host_priv *hpriv,
    struct device *dev)
    {
    struct st_ahci_drv_data *drv_data = hpriv.plat_data;
    drv_data.pwr = devm_reset_control_get(dev, "pwr-dwn");
    if (IS_ERR(drv_data.pwr)) {
    dev_info(dev, "power reset control not defined\n");
    drv_data.pwr = core::ptr::null_mut();
    }
    drv_data.sw_rst = devm_reset_control_get(dev, "sw-rst");
    if (IS_ERR(drv_data.sw_rst)) {
    dev_info(dev, "soft reset control not defined\n");
    drv_data.sw_rst = core::ptr::null_mut();
    }
    drv_data.pwr_rst = devm_reset_control_get(dev, "pwr-rst");
    if (IS_ERR(drv_data.pwr_rst)) {
    dev_dbg(dev, "power soft reset control not defined\n");
    drv_data.pwr_rst = core::ptr::null_mut();
    }
    return st_ahci_deassert_resets(hpriv, dev);
    }
    static struct ata_port_operations st_ahci_port_ops = {
    .inherits	= &ahci_platform_ops,
    .host_stop	= st_ahci_host_stop,
    };
    static const struct ata_port_info st_ahci_port_info = {
    .flags          = AHCI_FLAG_COMMON,
    .pio_mask       = ATA_PIO4,
    .udma_mask      = ATA_UDMA6,
    .port_ops       = &st_ahci_port_ops,
    };
    static const struct scsi_host_template ahci_platform_sht = {
    AHCI_SHT(DRV_NAME),
    };
#[no_mangle]
unsafe extern "C" fn st_ahci_probe(pdev: *mut platform_device) -> c_int {
    static int st_ahci_probe(struct platform_device *pdev)
    {
    struct st_ahci_drv_data *drv_data;
    struct ahci_host_priv *hpriv;
    int err;
    drv_data = devm_kzalloc(&pdev.dev, sizeof(*drv_data), GFP_KERNEL);
    if (!drv_data)
    return -ENOMEM;
    hpriv = ahci_platform_get_resources(pdev, 0);
    if (IS_ERR(hpriv))
    return PTR_ERR(hpriv);
    hpriv.plat_data = drv_data;
    err = st_ahci_probe_resets(hpriv, &pdev.dev);
    if (err)
    return err;
    err = ahci_platform_enable_resources(hpriv);
    if (err)
    return err;
    st_ahci_configure_oob(hpriv.mmio);
    err = ahci_platform_init_host(pdev, hpriv, &st_ahci_port_info,
    &ahci_platform_sht);
    if (err) {
    ahci_platform_disable_resources(hpriv);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn st_ahci_suspend(dev: *mut device) -> c_int {
    static int st_ahci_suspend(struct device *dev)
    {
    struct ata_host *host = dev_get_drvdata(dev);
    struct ahci_host_priv *hpriv = host.private_data;
    struct st_ahci_drv_data *drv_data = hpriv.plat_data;
    int err;
    err = ahci_platform_suspend_host(dev);
    if (err)
    return err;
    if (drv_data.pwr) {
    err = reset_control_assert(drv_data.pwr);
    if (err) {
    dev_err(dev, "unable to pwrdwn");
    return err;
    }
    }
    ahci_platform_disable_resources(hpriv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn st_ahci_resume(dev: *mut device) -> c_int {
    static int st_ahci_resume(struct device *dev)
    {
    struct ata_host *host = dev_get_drvdata(dev);
    struct ahci_host_priv *hpriv = host.private_data;
    int err;
    err = ahci_platform_enable_resources(hpriv);
    if (err)
    return err;
    err = st_ahci_deassert_resets(hpriv, dev);
    if (err) {
    ahci_platform_disable_resources(hpriv);
    return err;
    }
    st_ahci_configure_oob(hpriv.mmio);
    return ahci_platform_resume_host(dev);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(st_ahci_pm_ops, st_ahci_suspend, st_ahci_resume);
    static const struct of_device_id st_ahci_match[] = {
    { .compatible = "st,ahci", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, st_ahci_match);
    static struct platform_driver st_ahci_driver = {
    .driver = {
    .name = DRV_NAME,
    .pm = pm_sleep_ptr(&st_ahci_pm_ops),
    .of_match_table = st_ahci_match,
    },
    .probe = st_ahci_probe,
    .remove = ata_platform_remove_one,
    };
    module_platform_driver(st_ahci_driver);
    MODULE_AUTHOR("Alexandre Torgue <alexandre.torgue@st.com>");
    MODULE_AUTHOR("Francesco Virlinzi <francesco.virlinzi@st.com>");
    MODULE_DESCRIPTION("STMicroelectronics SATA AHCI Driver");
    MODULE_LICENSE("GPL v2");
