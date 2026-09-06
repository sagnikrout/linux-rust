//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/intel/atomisp2/led.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Driver for controlling LEDs for cameras connected to the Intel atomisp2
// The main purpose of this driver is to turn off LEDs which are on at boot.
//
// Copyright (C) 2020 Hans de Goede <hdegoede@redhat.com>
//

// This must be leds-gpio as the leds-gpio driver binds to the name

    static const struct gpio_led atomisp2_leds[] = {
    {
    .name = "atomisp2::camera",
    .default_state  = LEDS_GPIO_DEFSTATE_OFF,
    },
    };
    static const struct gpio_led_platform_data atomisp2_leds_pdata = {
    .num_leds	= ARRAY_SIZE(atomisp2_leds),
    .leds		= atomisp2_leds,
    };
    static struct gpiod_lookup_table asus_t100ta_lookup = {
    .dev_id = DEV_NAME,
    .table = {
    GPIO_LOOKUP_IDX("INT33FC:02", 8, core::ptr::null_mut(), 0, GPIO_ACTIVE_HIGH),
    { }
    }
    };
    static struct gpiod_lookup_table asus_t100chi_lookup = {
    .dev_id = DEV_NAME,
    .table = {
    GPIO_LOOKUP_IDX("INT33FC:01", 24, core::ptr::null_mut(), 0, GPIO_ACTIVE_HIGH),
    { }
    }
    };
    static const struct dmi_system_id atomisp2_led_systems[] __initconst = {
    {
    .matches = {
    DMI_EXACT_MATCH(DMI_SYS_VENDOR, "ASUSTeK COMPUTER INC."),
// Non exact match to also match T100TAF
    DMI_MATCH(DMI_PRODUCT_NAME, "T100TA"),
    },
    .driver_data = &asus_t100ta_lookup,
    },
    {
    .matches = {
    DMI_EXACT_MATCH(DMI_SYS_VENDOR, "ASUSTeK COMPUTER INC."),
    DMI_EXACT_MATCH(DMI_PRODUCT_NAME, "T200TA"),
    },
    .driver_data = &asus_t100ta_lookup,
    },
    {
    .matches = {
    DMI_EXACT_MATCH(DMI_SYS_VENDOR, "ASUSTeK COMPUTER INC."),
    DMI_EXACT_MATCH(DMI_PRODUCT_NAME, "T100CHI"),
    },
    .driver_data = &asus_t100chi_lookup,
    },
    {} /* Terminating entry */
    };
    MODULE_DEVICE_TABLE(dmi, atomisp2_led_systems);
    static struct gpiod_lookup_table *gpio_lookup;
    static struct platform_device *pdev;
#[no_mangle]
unsafe extern "C" fn atomisp2_led_init() -> int __init {
    static int __init atomisp2_led_init(void)
    {
    const struct dmi_system_id *system;
    system = dmi_first_match(atomisp2_led_systems);
    if (!system)
    return -ENODEV;
    gpio_lookup = system.driver_data;
    gpiod_add_lookup_table(gpio_lookup);
    pdev = platform_device_register_resndata(core::ptr::null_mut(),
    DEV_NAME, PLATFORM_DEVID_NONE,
    core::ptr::null_mut(), 0, &atomisp2_leds_pdata,
    sizeof(atomisp2_leds_pdata));
    if (IS_ERR(pdev))
    gpiod_remove_lookup_table(gpio_lookup);
    return PTR_ERR_OR_ZERO(pdev);
    }
#[no_mangle]
unsafe extern "C" fn atomisp2_led_cleanup() -> void __exit {
    static void __exit atomisp2_led_cleanup(void)
    {
    platform_device_unregister(pdev);
    gpiod_remove_lookup_table(gpio_lookup);
    }
    module_init(atomisp2_led_init);
    module_exit(atomisp2_led_cleanup);
//
// The ACPI INIT method from Asus WMI's code on the T100TA and T200TA turns the
// LED on (without the WMI interface allowing further control over the LED).
// Ensure we are loaded after asus-nb-wmi so that we turn the LED off again.
//
    MODULE_SOFTDEP("pre: asus_nb_wmi");
    MODULE_AUTHOR("Hans de Goede <hdegoede@redhat.com");
    MODULE_DESCRIPTION("Intel atomisp2 camera LED driver");
    MODULE_LICENSE("GPL");
