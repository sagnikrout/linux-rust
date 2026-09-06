//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-designware-platdrv.c
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
// Synopsys DesignWare I2C adapter driver.
//
// Based on the TI DAVINCI I2C adapter driver.
//
// Copyright (C) 2006 Texas Instruments.
// Copyright (C) 2007 MontaVista Software Inc.
// Copyright (C) 2009 Provigent Ltd.
//

#[no_mangle]
unsafe extern "C" fn i2c_dw_get_clk_rate_khz(dev: *mut dw_i2c_dev) -> u32 {
    static u32 i2c_dw_get_clk_rate_khz(struct dw_i2c_dev *dev)
    {
    return clk_get_rate(dev.clk) / HZ_PER_KHZ;
    }
#[no_mangle]
unsafe extern "C" fn dw_i2c_get_parent_regmap(dev: *mut dw_i2c_dev) -> c_int {
    static int dw_i2c_get_parent_regmap(struct dw_i2c_dev *dev)
    {
    dev.map = dev_get_regmap(dev.dev.parent, core::ptr::null_mut());
    if (!dev.map)
    return -ENODEV;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dw_i2c_plat_pm_cleanup(dev: *mut dw_i2c_dev) {
    static void dw_i2c_plat_pm_cleanup(struct dw_i2c_dev *dev)
    {
    pm_runtime_disable(dev.dev);
    if (dev.shared_with_punit)
    pm_runtime_put_noidle(dev.dev);
    }
#[no_mangle]
unsafe extern "C" fn dw_i2c_plat_request_regs(dev: *mut dw_i2c_dev) -> c_int {
    static int dw_i2c_plat_request_regs(struct dw_i2c_dev *dev)
    {
    struct platform_device *pdev = to_platform_device(dev.dev);
    int ret;
    if (device_is_compatible(dev.dev, "intel,xe-i2c"))
    return dw_i2c_get_parent_regmap(dev);
    switch (dev.flags & MODEL_MASK) {
    case MODEL_WANGXUN_SP:
    ret = dw_i2c_get_parent_regmap(dev);
    break;
    default:
    dev.base = devm_platform_ioremap_resource(pdev, 0);
    ret = PTR_ERR_OR_ZERO(dev.base);
    break;
    }
    return ret;
    }
    static const struct dmi_system_id dw_i2c_hwmon_class_dmi[] = {
    {
    .ident = "Qtechnology QT5222",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Qtechnology"),
    DMI_MATCH(DMI_PRODUCT_NAME, "QT5222"),
    },
    },
    { } /* terminate list */
    };
    static const struct i2c_dw_semaphore_callbacks i2c_dw_semaphore_cb_table[] = {

    {
    .probe = i2c_dw_baytrail_probe_lock_support,
    },

    {
    .probe = i2c_dw_amdpsp_probe_lock_support,
    },

    {}
    };
#[no_mangle]
unsafe extern "C" fn i2c_dw_probe_lock_support(dev: *mut dw_i2c_dev) -> c_int {
    static int i2c_dw_probe_lock_support(struct dw_i2c_dev *dev)
    {
    const struct i2c_dw_semaphore_callbacks *ptr;
    let mut i: c_int = 0;
    int ret;
    dev.semaphore_idx = -1;
    for (ptr = i2c_dw_semaphore_cb_table; ptr.probe; ptr++) {
    ret = ptr.probe(dev);
    if (ret) {
//
// If there is no semaphore device attached to this
// controller, we shouldn't abort general i2c_controller
// probe.
//
    if (ret != -ENODEV)
    return ret;
    i++;
    continue;
    }
    dev.semaphore_idx = i;
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dw_i2c_plat_probe(pdev: *mut platform_device) -> c_int {
    static int dw_i2c_plat_probe(struct platform_device *pdev)
    {
    let mut flags: u32 = (uintptr_t)device_get_match_data(&pdev.dev);
    struct device *device = &pdev.dev;
    struct i2c_adapter *adap;
    struct dw_i2c_dev *dev;
    int irq, ret;
    irq = platform_get_irq_optional(pdev, 0);
    if (irq == -ENXIO)
    flags |= ACCESS_POLLING;
#[no_mangle]
pub unsafe extern "C" fn if(0: irq <) -> else {
    else if (irq < 0)
    return irq;
    dev = devm_kzalloc(device, sizeof(*dev), GFP_KERNEL);
    if (!dev)
    return -ENOMEM;
    if (device_property_present(device, "wx,i2c-snps-model"))
    flags = MODEL_WANGXUN_SP | ACCESS_POLLING;
    dev.dev = device;
    dev.irq = irq;
    dev.flags = flags;
    platform_set_drvdata(pdev, dev);
    ret = dw_i2c_plat_request_regs(dev);
    if (ret)
    return ret;
    dev.rst = devm_reset_control_get_optional_exclusive_deasserted(device, core::ptr::null_mut());
    if (IS_ERR(dev.rst))
    return dev_err_probe(device, PTR_ERR(dev.rst), "failed to acquire reset\n");
    ret = i2c_dw_fw_parse_and_configure(dev);
    if (ret)
    return ret;
    ret = i2c_dw_probe_lock_support(dev);
    if (ret)
    return dev_err_probe(device, ret, "failed to probe lock support\n");
    i2c_dw_configure(dev);
// Optional interface clock
    dev.pclk = devm_clk_get_optional(device, "pclk");
    if (IS_ERR(dev.pclk))
    return dev_err_probe(device, PTR_ERR(dev.pclk), "failed to acquire pclk\n");
    dev.clk = devm_clk_get_optional(device, core::ptr::null_mut());
    if (IS_ERR(dev.clk))
    return dev_err_probe(device, PTR_ERR(dev.clk), "failed to acquire clock\n");
    ret = i2c_dw_prepare_clk(dev, true);
    if (ret)
    return ret;
    if (dev.clk) {
    struct i2c_timings *t = &dev.timings;
    u64 clk_khz;
    dev.get_clk_rate_khz = i2c_dw_get_clk_rate_khz;
    clk_khz = dev.get_clk_rate_khz(dev);
    if (!dev.sda_hold_time && t.sda_hold_ns)
    dev.sda_hold_time =
    DIV_S64_ROUND_CLOSEST(clk_khz * t.sda_hold_ns, MICRO);
    }
    adap = &dev.adapter;
    adap.owner = THIS_MODULE;
    adap.class = dmi_check_system(dw_i2c_hwmon_class_dmi) ?
    I2C_CLASS_HWMON : I2C_CLASS_DEPRECATED;
    adap.nr = -1;
    if (dev.flags & ACCESS_NO_IRQ_SUSPEND)
    dev_pm_set_driver_flags(device, DPM_FLAG_SMART_PREPARE);
    else
    dev_pm_set_driver_flags(device, DPM_FLAG_SMART_PREPARE | DPM_FLAG_SMART_SUSPEND);
    device_enable_async_suspend(device);
// The code below assumes runtime PM to be disabled.
    WARN_ON(pm_runtime_enabled(device));
    pm_runtime_set_autosuspend_delay(device, 1000);
    pm_runtime_use_autosuspend(device);
    pm_runtime_set_active(device);
    if (dev.shared_with_punit)
    pm_runtime_get_noresume(device);
    pm_runtime_enable(device);
    ret = i2c_dw_probe(dev);
    if (ret) {
    dw_i2c_plat_pm_cleanup(dev);
    i2c_dw_prepare_clk(dev, false);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dw_i2c_plat_remove(pdev: *mut platform_device) {
    static void dw_i2c_plat_remove(struct platform_device *pdev)
    {
    struct dw_i2c_dev *dev = platform_get_drvdata(pdev);
    struct device *device = &pdev.dev;
    pm_runtime_get_sync(device);
    i2c_del_adapter(&dev.adapter);
    i2c_dw_disable(dev);
    pm_runtime_dont_use_autosuspend(device);
    pm_runtime_put_noidle(device);
    dw_i2c_plat_pm_cleanup(dev);
    i2c_dw_prepare_clk(dev, false);
    }
    static const struct of_device_id dw_i2c_of_match[] = {
    { .compatible = "mobileye,eyeq6lplus-i2c" },
    { .compatible = "mscc,ocelot-i2c" },
    { .compatible = "snps,designware-i2c" },
    {}
    };
    MODULE_DEVICE_TABLE(of, dw_i2c_of_match);
    static const struct acpi_device_id dw_i2c_acpi_match[] = {
    { "80860F41", ACCESS_NO_IRQ_SUSPEND },
    { "808622C1", ACCESS_NO_IRQ_SUSPEND },
    { "AMD0010", ACCESS_INTR_MASK },
    { "AMDI0010", ACCESS_INTR_MASK },
    { "AMDI0019", ACCESS_INTR_MASK | ARBITRATION_SEMAPHORE },
    { "AMDI0510", 0 },
    { "APMC0D0F", 0 },
    { "FUJI200B", 0 },
    { "GOOG5000", 0 },
    { "HISI02A1", 0 },
    { "HISI02A2", 0 },
    { "HISI02A3", 0 },
    { "HJMC3001", ACCESS_INTR_MASK },
    { "HYGO0010", ACCESS_INTR_MASK },
    { "INT33C2", 0 },
    { "INT33C3", 0 },
    { "INT3432", 0 },
    { "INT3433", 0 },
    { "INTC10EF", 0 },
    { "LECA0003", 0 },
    {}
    };
    MODULE_DEVICE_TABLE(acpi, dw_i2c_acpi_match);
    static const struct platform_device_id dw_i2c_platform_ids[] = {
    { "i2c_designware" },
    {}
    };
    MODULE_DEVICE_TABLE(platform, dw_i2c_platform_ids);
#[no_mangle]
unsafe extern "C" fn dw_i2c_plat_shutdown(pdev: *mut platform_device) {
    static void dw_i2c_plat_shutdown(struct platform_device *pdev)
    {
    struct dw_i2c_dev *i_dev;
    i_dev = platform_get_drvdata(pdev);
    if (!i_dev)
    return;
    pm_runtime_disable(&pdev.dev);
    if (!pm_runtime_status_suspended(&pdev.dev))
    i2c_dw_shutdown(i_dev);
    }
    static struct platform_driver dw_i2c_driver = {
    .probe = dw_i2c_plat_probe,
    .remove = dw_i2c_plat_remove,
    .shutdown = dw_i2c_plat_shutdown,
    .driver		= {
    .name	= "i2c_designware",
    .of_match_table = dw_i2c_of_match,
    .acpi_match_table = dw_i2c_acpi_match,
    .pm	= pm_ptr(&i2c_dw_dev_pm_ops),
    },
    .id_table = dw_i2c_platform_ids,
    };
#[no_mangle]
unsafe extern "C" fn dw_i2c_init_driver() -> int __init {
    static int __init dw_i2c_init_driver(void)
    {
    return platform_driver_register(&dw_i2c_driver);
    }
    subsys_initcall(dw_i2c_init_driver);
#[no_mangle]
unsafe extern "C" fn dw_i2c_exit_driver() -> void __exit {
    static void __exit dw_i2c_exit_driver(void)
    {
    platform_driver_unregister(&dw_i2c_driver);
    }
    module_exit(dw_i2c_exit_driver);
    MODULE_AUTHOR("Baruch Siach <baruch@tkos.co.il>");
    MODULE_DESCRIPTION("Synopsys DesignWare I2C bus adapter");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("I2C_DW");
    MODULE_IMPORT_NS("I2C_DW_COMMON");
