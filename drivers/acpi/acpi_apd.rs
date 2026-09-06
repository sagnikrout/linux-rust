//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/acpi_apd.c
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
// AMD ACPI support for ACPI2platform device.
//
// Copyright (c) 2014,2015 AMD Corporation.
// Authors: Ken Xue <Ken.Xue@amd.com>
// Wu, Jeff <Jeff.Wu@amd.com>
//

    struct apd_private_data;
//
// struct apd_device_desc - a descriptor for apd device
// @fixed_clk_rate: fixed rate input clock source for acpi device;
// 0 means no fixed rate input clock source
// @properties: build-in properties of the device such as UART
// @setup: a hook routine to set device resource during create platform device
//
// Device description defined as acpi_device_id.driver_data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apd_device_desc {
    pub fixed_clk_rate: c_uint,
    pub properties: *mut property_entry,
    pub pdata): *mut *mut int (setup)(struct apd_private_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apd_private_data {
    pub clk: *mut clk,
    pub adev: *mut acpi_device,
    pub dev_desc: *const apd_device_desc,
}

#[no_mangle]
unsafe extern "C" fn acpi_apd_setup(pdata: *mut apd_private_data) -> c_int {
    static int acpi_apd_setup(struct apd_private_data *pdata)
    {
    const struct apd_device_desc *dev_desc = pdata.dev_desc;
    struct clk *clk;
    if (dev_desc.fixed_clk_rate) {
    clk = clk_register_fixed_rate(&pdata.adev.dev,
    dev_name(&pdata.adev.dev),
    core::ptr::null_mut(), 0, dev_desc.fixed_clk_rate);
    clk_register_clkdev(clk, core::ptr::null_mut(), dev_name(&pdata.adev.dev));
    pdata.clk = clk;
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn fch_misc_setup(pdata: *mut apd_private_data) -> c_int {
    static int fch_misc_setup(struct apd_private_data *pdata)
    {
    struct acpi_device *adev = pdata.adev;
    const union acpi_object *obj;
    struct platform_device *clkdev;
    struct fch_clk_data *clk_data;
    struct resource_entry *rentry;
    struct list_head resource_list;
    int ret;
    clk_data = devm_kzalloc(&adev.dev, sizeof(*clk_data), GFP_KERNEL);
    if (!clk_data)
    return -ENOMEM;
    INIT_LIST_HEAD(&resource_list);
    ret = acpi_dev_get_memory_resources(adev, &resource_list);
    if (ret < 0)
    return -ENOENT;
    if (!acpi_dev_get_property(adev, "clk-name", ACPI_TYPE_STRING, &obj)) {
    clk_data.name = devm_kzalloc(&adev.dev, obj.string.length,
    GFP_KERNEL);
    if (!clk_data.name)
    return -ENOMEM;
    strscpy(clk_data.name, obj.string.pointer, obj.string.length);
    } else {
// Set default name to mclk if entry missing in firmware
    clk_data.name = "mclk";
    }
    list_for_each_entry(rentry, &resource_list, node) {
    clk_data.base = devm_ioremap(&adev.dev, rentry.res.start,
    resource_size(rentry.res));
    break;
    }
    if (!clk_data.base)
    return -ENOMEM;
    acpi_dev_free_resource_list(&resource_list);
    clkdev = platform_device_register_data(&adev.dev, "clk-fch",
    PLATFORM_DEVID_NONE, clk_data,
    sizeof(*clk_data));
    return PTR_ERR_OR_ZERO(clkdev);
    }
    static const struct apd_device_desc cz_i2c_desc = {
    .setup = acpi_apd_setup,
    .fixed_clk_rate = 133 * HZ_PER_MHZ,
    };
    static const struct apd_device_desc wt_i2c_desc = {
    .setup = acpi_apd_setup,
    .fixed_clk_rate = 150 * HZ_PER_MHZ,
    };
    static const struct apd_device_desc wt_i3c_desc = {
    .setup = acpi_apd_setup,
    .fixed_clk_rate = 125 * HZ_PER_MHZ,
    };
    static struct property_entry uart_properties[] = {
    PROPERTY_ENTRY_U32("reg-io-width", 4),
    PROPERTY_ENTRY_U32("reg-shift", 2),
    PROPERTY_ENTRY_BOOL("snps,uart-16550-compatible"),
    { },
    };
    static const struct apd_device_desc cz_uart_desc = {
    .setup = acpi_apd_setup,
    .fixed_clk_rate = 48 * HZ_PER_MHZ,
    .properties = uart_properties,
    };
    static const struct apd_device_desc fch_misc_desc = {
    .setup = fch_misc_setup,
    };

    static const struct apd_device_desc xgene_i2c_desc = {
    .setup = acpi_apd_setup,
    .fixed_clk_rate = 100 * HZ_PER_MHZ,
    };
    static const struct apd_device_desc vulcan_spi_desc = {
    .setup = acpi_apd_setup,
    .fixed_clk_rate = 133 * HZ_PER_MHZ,
    };
    static const struct apd_device_desc hip07_i2c_desc = {
    .setup = acpi_apd_setup,
    .fixed_clk_rate = 200 * HZ_PER_MHZ,
    };
    static const struct apd_device_desc hip08_i2c_desc = {
    .setup = acpi_apd_setup,
    .fixed_clk_rate = 250 * HZ_PER_MHZ,
    };
    static const struct apd_device_desc hip08_lite_i2c_desc = {
    .setup = acpi_apd_setup,
    .fixed_clk_rate = 125 * HZ_PER_MHZ,
    };
    static const struct apd_device_desc thunderx2_i2c_desc = {
    .setup = acpi_apd_setup,
    .fixed_clk_rate = 125 * HZ_PER_MHZ,
    };
    static const struct apd_device_desc nxp_i2c_desc = {
    .setup = acpi_apd_setup,
    .fixed_clk_rate = 350 * HZ_PER_MHZ,
    };
    static const struct apd_device_desc hip08_spi_desc = {
    .setup = acpi_apd_setup,
    .fixed_clk_rate = 250 * HZ_PER_MHZ,
    };
    static const struct apd_device_desc leca_spi_desc = {
    .setup = acpi_apd_setup,
    .fixed_clk_rate = 400 * HZ_PER_MHZ,
    };
    static const struct apd_device_desc leca_i2c_desc = {
    .setup = acpi_apd_setup,
    .fixed_clk_rate = 250 * HZ_PER_MHZ,
    };
    static const struct apd_device_desc hjmc_i2c_desc = {
    .setup = acpi_apd_setup,
    .fixed_clk_rate = 200 * HZ_PER_MHZ,
    };

//
// Create platform device during acpi scan attach handle.
// Return value > 0 on success of creating device.
//
    static int acpi_apd_create_device(struct acpi_device *adev,
    const struct acpi_device_id *id)
    {
    const struct apd_device_desc *dev_desc = (void *)id.driver_data;
    struct apd_private_data *pdata;
    struct platform_device *pdev;
    int ret;
    if (!dev_desc) {
    pdev = acpi_create_platform_device(adev, core::ptr::null_mut());
    return IS_ERR_OR_NULL(pdev) ? PTR_ERR(pdev) : 1;
    }
    pdata = kzalloc_obj(*pdata);
    if (!pdata)
    return -ENOMEM;
    pdata.adev = adev;
    pdata.dev_desc = dev_desc;
    if (dev_desc.setup) {
    ret = dev_desc.setup(pdata);
    if (ret)
    goto err_out;
    }
    adev.driver_data = pdata;
    pdev = acpi_create_platform_device(adev, dev_desc.properties);
    if (!IS_ERR_OR_NULL(pdev))
    return 1;
    ret = PTR_ERR(pdev);
    adev.driver_data = core::ptr::null_mut();
    err_out:
    kfree(pdata);
    return ret;
    }
    static const struct acpi_device_id acpi_apd_device_ids[] = {
// Generic apd devices

    { "AMD0010", APD_ADDR(cz_i2c_desc) },
    { "AMD0020", APD_ADDR(cz_uart_desc) },
    { "AMD0030", },
    { "AMD0040", APD_ADDR(fch_misc_desc)},
    { "AMDI0010", APD_ADDR(wt_i2c_desc) },
    { "AMDI0015", APD_ADDR(wt_i3c_desc) },
    { "AMDI0019", APD_ADDR(wt_i2c_desc) },
    { "AMDI0020", APD_ADDR(cz_uart_desc) },
    { "AMDI0022", APD_ADDR(cz_uart_desc) },
    { "HYGO0010", APD_ADDR(wt_i2c_desc) },

    { "APMC0D0F", APD_ADDR(xgene_i2c_desc) },
    { "BRCM900D", APD_ADDR(vulcan_spi_desc) },
    { "CAV900D",  APD_ADDR(vulcan_spi_desc) },
    { "CAV9007",  APD_ADDR(thunderx2_i2c_desc) },
    { "HISI02A1", APD_ADDR(hip07_i2c_desc) },
    { "HISI02A2", APD_ADDR(hip08_i2c_desc) },
    { "HISI02A3", APD_ADDR(hip08_lite_i2c_desc) },
    { "HISI0173", APD_ADDR(hip08_spi_desc) },
    { "HJMC3001", APD_ADDR(hjmc_i2c_desc) },
    { "LECA0002", APD_ADDR(leca_spi_desc) },
    { "LECA0003", APD_ADDR(leca_i2c_desc) },
    { "NXP0001", APD_ADDR(nxp_i2c_desc) },

    { }
    };
    static struct acpi_scan_handler apd_handler = {
    .ids = acpi_apd_device_ids,
    .attach = acpi_apd_create_device,
    };
#[no_mangle]
pub unsafe extern "C" fn acpi_apd_init() -> void __init {
    void __init acpi_apd_init(void)
    {
    acpi_scan_add_handler(&apd_handler);
    }
