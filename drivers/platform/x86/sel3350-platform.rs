//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/sel3350-platform.c
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


// SPDX-License-Identifier: GPL-2.0-only OR BSD-3-Clause
//
// Copyright 2023 Schweitzer Engineering Laboratories, Inc.
// 2350 NE Hopkins Court, Pullman, WA 99163 USA
//
// Platform support for the b2093 mainboard used in SEL-3350 computers.
// Consumes GPIO from the SoC to provide standard LED and power supply
// devices.
//

// Broxton communities

    static const char *const sel3350_leds_gpio_names[] = {
    AUX_LED_GRN1,
    AUX_LED_GRN2,
    AUX_LED_GRN3,
    AUX_LED_GRN4,
    ALARM_STATE_USER,
    ENABLE_STATE_USER,
    AUX_LED_RED1,
    AUX_LED_RED2,
    AUX_LED_RED3,
    AUX_LED_RED4,
    };
// LEDs
    static struct gpio_led sel3350_leds[] = {
    { .name = "sel:green:aux1",
    .default_state = LEDS_GPIO_DEFSTATE_KEEP,
    .retain_state_suspended = 1,
    .retain_state_shutdown = 1,
    },
    { .name = "sel:green:aux2",
    .default_state = LEDS_GPIO_DEFSTATE_KEEP,
    .retain_state_suspended = 1,
    .retain_state_shutdown = 1,
    },
    { .name = "sel:green:aux3",
    .default_state = LEDS_GPIO_DEFSTATE_KEEP,
    .retain_state_suspended = 1,
    .retain_state_shutdown = 1,
    },
    { .name = "sel:green:aux4",
    .default_state = LEDS_GPIO_DEFSTATE_KEEP,
    .retain_state_suspended = 1,
    .retain_state_shutdown = 1,
    },
    { .name = "sel:red:alarm",
    .default_state = LEDS_GPIO_DEFSTATE_KEEP,
    .retain_state_suspended = 1,
    .retain_state_shutdown = 1,
    },
    { .name = "sel:green:enabled",
    .default_state = LEDS_GPIO_DEFSTATE_KEEP,
    .retain_state_suspended = 1,
    .retain_state_shutdown = 1,
    },
    { .name = "sel:red:aux1",
    .default_state = LEDS_GPIO_DEFSTATE_KEEP,
    .retain_state_suspended = 1,
    .retain_state_shutdown = 1,
    },
    { .name = "sel:red:aux2",
    .default_state = LEDS_GPIO_DEFSTATE_KEEP,
    .retain_state_suspended = 1,
    .retain_state_shutdown = 1,
    },
    { .name = "sel:red:aux3",
    .default_state = LEDS_GPIO_DEFSTATE_KEEP,
    .retain_state_suspended = 1,
    .retain_state_shutdown = 1,
    },
    { .name = "sel:red:aux4",
    .default_state = LEDS_GPIO_DEFSTATE_KEEP,
    .retain_state_suspended = 1,
    .retain_state_shutdown = 1,
    },
    };
    static const struct gpio_led_platform_data sel3350_leds_pdata = {
    .num_leds = ARRAY_SIZE(sel3350_leds),
    .leds = sel3350_leds,
    };
    static struct gpiod_lookup_table sel3350_gpios_table = {
    .dev_id = B2093_GPIO_ACPI_ID ":00",
    .table = {
    GPIO_LOOKUP(BXT_NW, 44, SEL_PS_A_DETECT, GPIO_ACTIVE_LOW),
    GPIO_LOOKUP(BXT_NW, 45, SEL_PS_A_GOOD,   GPIO_ACTIVE_LOW),
    GPIO_LOOKUP(BXT_NW, 46, SEL_PS_B_DETECT, GPIO_ACTIVE_LOW),
    GPIO_LOOKUP(BXT_NW, 47, SEL_PS_B_GOOD,   GPIO_ACTIVE_LOW),
    GPIO_LOOKUP(BXT_NW, 49, AUX_LED_GRN1, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP(BXT_NW, 50, AUX_LED_GRN2, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP(BXT_NW, 51, AUX_LED_GRN3, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP(BXT_NW, 52, AUX_LED_GRN4, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP(BXT_W,  20, ALARM_STATE_USER, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP(BXT_W,  21, ENABLE_STATE_USER, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP(BXT_SW, 37, AUX_LED_RED1, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP(BXT_SW, 38, AUX_LED_RED2, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP(BXT_SW, 39, AUX_LED_RED3, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP(BXT_SW, 40, AUX_LED_RED4, GPIO_ACTIVE_HIGH),
    {},
    }
    };
// Power Supplies
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sel3350_power_cfg_data {
    pub ps_detect: *mut gpio_desc,
    pub ps_good: *mut gpio_desc,
}

    static int sel3350_power_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct sel3350_power_cfg_data *data = power_supply_get_drvdata(psy);
    switch (psp) {
    case POWER_SUPPLY_PROP_HEALTH:
    if (gpiod_get_value(data.ps_detect)) {
    if (gpiod_get_value(data.ps_good))
    val.intval = POWER_SUPPLY_HEALTH_GOOD;
    else
    val.intval = POWER_SUPPLY_HEALTH_UNSPEC_FAILURE;
    } else {
    val.intval = POWER_SUPPLY_HEALTH_UNKNOWN;
    }
    break;
    case POWER_SUPPLY_PROP_PRESENT:
    val.intval = gpiod_get_value(data.ps_detect);
    break;
    case POWER_SUPPLY_PROP_ONLINE:
    val.intval = gpiod_get_value(data.ps_good);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static const enum power_supply_property sel3350_power_properties[] = {
    POWER_SUPPLY_PROP_HEALTH,
    POWER_SUPPLY_PROP_PRESENT,
    POWER_SUPPLY_PROP_ONLINE,
    };
    static const struct power_supply_desc sel3350_ps_a_desc = {
    .name = SEL_PS_A,
    .type = POWER_SUPPLY_TYPE_MAINS,
    .properties = sel3350_power_properties,
    .num_properties = ARRAY_SIZE(sel3350_power_properties),
    .get_property = sel3350_power_get_property,
    };
    static const struct power_supply_desc sel3350_ps_b_desc = {
    .name = SEL_PS_B,
    .type = POWER_SUPPLY_TYPE_MAINS,
    .properties = sel3350_power_properties,
    .num_properties = ARRAY_SIZE(sel3350_power_properties),
    .get_property = sel3350_power_get_property,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sel3350_data {
    pub leds_pdev: *mut platform_device,
    pub ps_a: *mut power_supply,
    pub ps_b: *mut power_supply,
    pub ps_a_cfg_data: sel3350_power_cfg_data,
    pub ps_b_cfg_data: sel3350_power_cfg_data,
}

#[no_mangle]
unsafe extern "C" fn sel3350_probe(pdev: *mut platform_device) -> c_int {
    static int sel3350_probe(struct platform_device *pdev)
    {
    int rs;
    int i;
    struct sel3350_data *sel3350;
    let mut ps_cfg: power_supply_config = {};
    sel3350 = devm_kzalloc(&pdev.dev, sizeof(struct sel3350_data), GFP_KERNEL);
    if (!sel3350)
    return -ENOMEM;
    platform_set_drvdata(pdev, sel3350);
    gpiod_add_lookup_table(&sel3350_gpios_table);
    for (i = 0; i < ARRAY_SIZE(sel3350_leds); ++i) {
    sel3350_leds[i].gpiod = devm_gpiod_get(&pdev.dev,
    sel3350_leds_gpio_names[i],
    GPIOD_ASIS);
    if (IS_ERR_OR_NULL(sel3350_leds[i].gpiod)) {
    rs = -EPROBE_DEFER;
    goto err_gpio_loop;
    }
    gpiod_set_consumer_name(sel3350_leds[i].gpiod, sel3350_leds[i].name);
    }
    sel3350.leds_pdev = platform_device_register_data(
    core::ptr::null_mut(),
    "leds-gpio",
    PLATFORM_DEVID_NONE,
    &sel3350_leds_pdata,
    sizeof(sel3350_leds_pdata));
    if (IS_ERR(sel3350.leds_pdev)) {
    rs = PTR_ERR(sel3350.leds_pdev);
    dev_err(&pdev.dev, "Failed registering platform device: %d\n", rs);
    goto err_platform;
    }
// Power Supply A
    sel3350.ps_a_cfg_data.ps_detect = devm_gpiod_get(&pdev.dev,
    SEL_PS_A_DETECT,
    GPIOD_IN);
    sel3350.ps_a_cfg_data.ps_good = devm_gpiod_get(&pdev.dev,
    SEL_PS_A_GOOD,
    GPIOD_IN);
    ps_cfg.drv_data = &sel3350.ps_a_cfg_data;
    sel3350.ps_a = devm_power_supply_register(&pdev.dev,
    &sel3350_ps_a_desc,
    &ps_cfg);
    if (IS_ERR(sel3350.ps_a)) {
    rs = PTR_ERR(sel3350.ps_a);
    dev_err(&pdev.dev, "Failed registering power supply A: %d\n", rs);
    goto err_ps;
    }
// Power Supply B
    sel3350.ps_b_cfg_data.ps_detect = devm_gpiod_get(&pdev.dev,
    SEL_PS_B_DETECT,
    GPIOD_IN);
    sel3350.ps_b_cfg_data.ps_good = devm_gpiod_get(&pdev.dev,
    SEL_PS_B_GOOD,
    GPIOD_IN);
    ps_cfg.drv_data = &sel3350.ps_b_cfg_data;
    sel3350.ps_b = devm_power_supply_register(&pdev.dev,
    &sel3350_ps_b_desc,
    &ps_cfg);
    if (IS_ERR(sel3350.ps_b)) {
    rs = PTR_ERR(sel3350.ps_b);
    dev_err(&pdev.dev, "Failed registering power supply B: %d\n", rs);
    goto err_ps;
    }
    return 0;
    err_gpio_loop:
    while (i--)
    devm_gpiod_put(&pdev.dev, sel3350_leds[i].gpiod);
    goto err_platform;
    err_ps:
    platform_device_unregister(sel3350.leds_pdev);
    err_platform:
    gpiod_remove_lookup_table(&sel3350_gpios_table);
    return rs;
    }
#[no_mangle]
unsafe extern "C" fn sel3350_remove(pdev: *mut platform_device) {
    static void sel3350_remove(struct platform_device *pdev)
    {
    struct sel3350_data *sel3350 = platform_get_drvdata(pdev);
    platform_device_unregister(sel3350.leds_pdev);
    gpiod_remove_lookup_table(&sel3350_gpios_table);
    }
    static const struct acpi_device_id sel3350_device_ids[] = {
    { B2093_GPIO_ACPI_ID, 0 },
    { "", 0 },
    };
    MODULE_DEVICE_TABLE(acpi, sel3350_device_ids);
    static struct platform_driver sel3350_platform_driver = {
    .probe = sel3350_probe,
    .remove = sel3350_remove,
    .driver = {
    .name = "sel3350-platform",
    .acpi_match_table = sel3350_device_ids,
    },
    };
    module_platform_driver(sel3350_platform_driver);
    MODULE_AUTHOR("Schweitzer Engineering Laboratories");
    MODULE_DESCRIPTION("SEL-3350 platform driver");
    MODULE_LICENSE("Dual BSD/GPL");
    MODULE_SOFTDEP("pre: pinctrl_broxton leds-gpio");
