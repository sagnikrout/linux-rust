//! Automatically rewritten from C to Rust
//! Source: net/rfkill/rfkill-gpio.c
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
// Copyright (c) 2011, NVIDIA Corporation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rfkill_gpio_data {
    pub name: *const c_char,
    pub type: enum rfkill_type,
    pub reset_gpio: *mut gpio_desc,
    pub shutdown_gpio: *mut gpio_desc,
    pub rfkill_dev: *mut rfkill,
    pub clk: *mut clk,
    pub clk_enabled: bool,
}

#[no_mangle]
unsafe extern "C" fn rfkill_gpio_set_power(data: *mut c_void, blocked: bool) -> c_int {
    static int rfkill_gpio_set_power(void *data, bool blocked)
    {
    struct rfkill_gpio_data *rfkill = data;
    if (!blocked && !IS_ERR(rfkill.clk) && !rfkill.clk_enabled) {
    let mut ret: c_int = clk_enable(rfkill.clk);
    if (ret)
    return ret;
    }
    gpiod_set_value_cansleep(rfkill.shutdown_gpio, !blocked);
    gpiod_set_value_cansleep(rfkill.reset_gpio, !blocked);
    if (blocked && !IS_ERR(rfkill.clk) && rfkill.clk_enabled)
    clk_disable(rfkill.clk);
    rfkill.clk_enabled = !blocked;
    return 0;
    }
    static const struct rfkill_ops rfkill_gpio_ops = {
    .set_block = rfkill_gpio_set_power,
    };
    let mut reset_gpios: static struct acpi_gpio_params = { 0, 0, false };
    let mut shutdown_gpios: static struct acpi_gpio_params = { 1, 0, false };
    static const struct acpi_gpio_mapping acpi_rfkill_default_gpios[] = {
    { "reset-gpios", &reset_gpios, 1 },
    { "shutdown-gpios", &shutdown_gpios, 1 },
    { },
    };
    static int rfkill_gpio_acpi_probe(struct device *dev,
    struct rfkill_gpio_data *rfkill)
    {
    const struct acpi_device_id *id;
    id = acpi_match_device(dev.driver.acpi_match_table, dev);
    if (!id)
    return -ENODEV;
    rfkill.type = (unsigned)id.driver_data;
    return devm_acpi_dev_add_driver_gpios(dev, acpi_rfkill_default_gpios);
    }
// List of DMI matches for devices on which rfkill-gpio should not load,
// to avoid firmware bugs.
//
    static const struct dmi_system_id rfkill_gpio_deny_table[] = {
    {
// Lenovo Yoga Tab 3 Pro YT3-X90, bogus "BCM4752" device in DSDT
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Intel Corporation"),
    DMI_MATCH(DMI_PRODUCT_VERSION, "Blade3-10A-001"),
    },
    },
    { }
    };
#[no_mangle]
unsafe extern "C" fn rfkill_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int rfkill_gpio_probe(struct platform_device *pdev)
    {
    struct rfkill_gpio_data *rfkill;
    const char *type_name = core::ptr::null_mut();
    const char *name_property;
    const char *type_property;
    struct gpio_desc *gpio;
    int ret;
    if (dmi_check_system(rfkill_gpio_deny_table))
    return -ENODEV;
    rfkill = devm_kzalloc(&pdev.dev, sizeof(*rfkill), GFP_KERNEL);
    if (!rfkill)
    return -ENOMEM;
    if (dev_of_node(&pdev.dev)) {
    name_property = "label";
    type_property = "radio-type";
    } else {
    name_property = "name";
    type_property = "type";
    }
    device_property_read_string(&pdev.dev, name_property, &rfkill.name);
    device_property_read_string(&pdev.dev, type_property, &type_name);
    if (!rfkill.name)
    rfkill.name = dev_name(&pdev.dev);
    rfkill.type = rfkill_find_type(type_name);
    if (ACPI_HANDLE(&pdev.dev)) {
    ret = rfkill_gpio_acpi_probe(&pdev.dev, rfkill);
    if (ret)
    return ret;
    }
    rfkill.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    gpio = devm_gpiod_get_optional(&pdev.dev, "reset", GPIOD_ASIS);
    if (IS_ERR(gpio))
    return PTR_ERR(gpio);
    rfkill.reset_gpio = gpio;
    gpio = devm_gpiod_get_optional(&pdev.dev, "shutdown", GPIOD_ASIS);
    if (IS_ERR(gpio))
    return PTR_ERR(gpio);
    rfkill.shutdown_gpio = gpio;
// Make sure at-least one GPIO is defined for this instance
    if (!rfkill.reset_gpio && !rfkill.shutdown_gpio) {
    dev_err(&pdev.dev, "invalid platform data\n");
    return -EINVAL;
    }
    ret = gpiod_direction_output(rfkill.reset_gpio, true);
    if (ret)
    return ret;
    ret = gpiod_direction_output(rfkill.shutdown_gpio, true);
    if (ret)
    return ret;
    rfkill.rfkill_dev = rfkill_alloc(rfkill.name, &pdev.dev,
    rfkill.type, &rfkill_gpio_ops,
    rfkill);
    if (!rfkill.rfkill_dev)
    return -ENOMEM;
    if (device_property_present(&pdev.dev, "default-blocked"))
    rfkill_init_sw_state(rfkill.rfkill_dev, true);
    ret = rfkill_register(rfkill.rfkill_dev);
    if (ret < 0)
    goto err_destroy;
    platform_set_drvdata(pdev, rfkill);
    dev_info(&pdev.dev, "%s device registered.\n", rfkill.name);
    return 0;
    err_destroy:
    rfkill_destroy(rfkill.rfkill_dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rfkill_gpio_remove(pdev: *mut platform_device) {
    static void rfkill_gpio_remove(struct platform_device *pdev)
    {
    struct rfkill_gpio_data *rfkill = platform_get_drvdata(pdev);
    rfkill_unregister(rfkill.rfkill_dev);
    rfkill_destroy(rfkill.rfkill_dev);
    }

    static const struct acpi_device_id rfkill_acpi_match[] = {
    { "BCM4752", RFKILL_TYPE_GPS },
    { "LNV4752", RFKILL_TYPE_GPS },
    { },
    };
    MODULE_DEVICE_TABLE(acpi, rfkill_acpi_match);

    static const struct of_device_id rfkill_of_match[] __maybe_unused = {
    { .compatible = "rfkill-gpio", },
    { },
    };
    MODULE_DEVICE_TABLE(of, rfkill_of_match);
    static struct platform_driver rfkill_gpio_driver = {
    .probe = rfkill_gpio_probe,
    .remove = rfkill_gpio_remove,
    .driver = {
    .name = "rfkill_gpio",
    .acpi_match_table = ACPI_PTR(rfkill_acpi_match),
    .of_match_table = of_match_ptr(rfkill_of_match),
    },
    };
    module_platform_driver(rfkill_gpio_driver);
    MODULE_DESCRIPTION("gpio rfkill");
    MODULE_AUTHOR("NVIDIA");
    MODULE_LICENSE("GPL");
