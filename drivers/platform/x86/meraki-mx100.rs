//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/meraki-mx100.c
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
// Cisco Meraki MX100 (Tinkerbell) board platform driver
//
// Based off of arch/x86/platform/meraki/tink.c from the
// Meraki GPL release meraki-firmware-sources-r23-20150601
//
// Format inspired by platform/x86/pcengines-apuv2.c
//
// Copyright (C) 2021 Chris Blake <chrisrblake93@gmail.com>
//

// LEDs
    static const struct software_node tink_gpio_leds_node = {
    .name = "meraki-mx100-leds",
    };
    static const struct property_entry tink_internet_led_props[] = {
    PROPERTY_ENTRY_STRING("label", "mx100:green:internet"),
    PROPERTY_ENTRY_STRING("linux,default-trigger", "default-on"),
    PROPERTY_ENTRY_GPIO("gpios", &lpc_ich_gpio_swnode, 11, GPIO_ACTIVE_LOW),
    { }
    };
    static const struct software_node tink_internet_led_node = {
    .name = "internet-led",
    .parent = &tink_gpio_leds_node,
    .properties = tink_internet_led_props,
    };
    static const struct property_entry tink_lan2_led_props[] = {
    PROPERTY_ENTRY_STRING("label", "mx100:green:lan2"),
    PROPERTY_ENTRY_GPIO("gpios", &lpc_ich_gpio_swnode, 18, GPIO_ACTIVE_HIGH),
    { }
    };
    static const struct software_node tink_lan2_led_node = {
    .name = "lan2-led",
    .parent = &tink_gpio_leds_node,
    .properties = tink_lan2_led_props,
    };
    static const struct property_entry tink_lan3_led_props[] = {
    PROPERTY_ENTRY_STRING("label", "mx100:green:lan3"),
    PROPERTY_ENTRY_GPIO("gpios", &lpc_ich_gpio_swnode, 20, GPIO_ACTIVE_HIGH),
    { }
    };
    static const struct software_node tink_lan3_led_node = {
    .name = "lan3-led",
    .parent = &tink_gpio_leds_node,
    .properties = tink_lan3_led_props,
    };
    static const struct property_entry tink_lan4_led_props[] = {
    PROPERTY_ENTRY_STRING("label", "mx100:green:lan4"),
    PROPERTY_ENTRY_GPIO("gpios", &lpc_ich_gpio_swnode, 22, GPIO_ACTIVE_HIGH),
    { }
    };
    static const struct software_node tink_lan4_led_node = {
    .name = "lan4-led",
    .parent = &tink_gpio_leds_node,
    .properties = tink_lan4_led_props,
    };
    static const struct property_entry tink_lan5_led_props[] = {
    PROPERTY_ENTRY_STRING("label", "mx100:green:lan5"),
    PROPERTY_ENTRY_GPIO("gpios", &lpc_ich_gpio_swnode, 23, GPIO_ACTIVE_HIGH),
    { }
    };
    static const struct software_node tink_lan5_led_node = {
    .name = "lan5-led",
    .parent = &tink_gpio_leds_node,
    .properties = tink_lan5_led_props,
    };
    static const struct property_entry tink_lan6_led_props[] = {
    PROPERTY_ENTRY_STRING("label", "mx100:green:lan6"),
    PROPERTY_ENTRY_GPIO("gpios", &lpc_ich_gpio_swnode, 32, GPIO_ACTIVE_HIGH),
    { }
    };
    static const struct software_node tink_lan6_led_node = {
    .name = "lan6-led",
    .parent = &tink_gpio_leds_node,
    .properties = tink_lan6_led_props,
    };
    static const struct property_entry tink_lan7_led_props[] = {
    PROPERTY_ENTRY_STRING("label", "mx100:green:lan7"),
    PROPERTY_ENTRY_GPIO("gpios", &lpc_ich_gpio_swnode, 34, GPIO_ACTIVE_HIGH),
    { }
    };
    static const struct software_node tink_lan7_led_node = {
    .name = "lan7-led",
    .parent = &tink_gpio_leds_node,
    .properties = tink_lan7_led_props,
    };
    static const struct property_entry tink_lan8_led_props[] = {
    PROPERTY_ENTRY_STRING("label", "mx100:green:lan8"),
    PROPERTY_ENTRY_GPIO("gpios", &lpc_ich_gpio_swnode, 35, GPIO_ACTIVE_HIGH),
    { }
    };
    static const struct software_node tink_lan8_led_node = {
    .name = "lan8-led",
    .parent = &tink_gpio_leds_node,
    .properties = tink_lan8_led_props,
    };
    static const struct property_entry tink_lan9_led_props[] = {
    PROPERTY_ENTRY_STRING("label", "mx100:green:lan9"),
    PROPERTY_ENTRY_GPIO("gpios", &lpc_ich_gpio_swnode, 36, GPIO_ACTIVE_HIGH),
    { }
    };
    static const struct software_node tink_lan9_led_node = {
    .name = "lan9-led",
    .parent = &tink_gpio_leds_node,
    .properties = tink_lan9_led_props,
    };
    static const struct property_entry tink_lan10_led_props[] = {
    PROPERTY_ENTRY_STRING("label", "mx100:green:lan10"),
    PROPERTY_ENTRY_GPIO("gpios", &lpc_ich_gpio_swnode, 37, GPIO_ACTIVE_HIGH),
    { }
    };
    static const struct software_node tink_lan10_led_node = {
    .name = "lan10-led",
    .parent = &tink_gpio_leds_node,
    .properties = tink_lan10_led_props,
    };
    static const struct property_entry tink_lan11_led_props[] = {
    PROPERTY_ENTRY_STRING("label", "mx100:green:lan11"),
    PROPERTY_ENTRY_GPIO("gpios", &lpc_ich_gpio_swnode, 48, GPIO_ACTIVE_HIGH),
    { }
    };
    static const struct software_node tink_lan11_led_node = {
    .name = "lan11-led",
    .parent = &tink_gpio_leds_node,
    .properties = tink_lan11_led_props,
    };
    static const struct property_entry tink_ha_green_led_props[] = {
    PROPERTY_ENTRY_STRING("label", "mx100:green:ha"),
    PROPERTY_ENTRY_GPIO("gpios", &lpc_ich_gpio_swnode, 16, GPIO_ACTIVE_LOW),
    { }
    };
    static const struct software_node tink_ha_green_led_node = {
    .name = "ha-green-led",
    .parent = &tink_gpio_leds_node,
    .properties = tink_ha_green_led_props,
    };
    static const struct property_entry tink_ha_orange_led_props[] = {
    PROPERTY_ENTRY_STRING("label", "mx100:orange:ha"),
    PROPERTY_ENTRY_GPIO("gpios", &lpc_ich_gpio_swnode, 7, GPIO_ACTIVE_LOW),
    { }
    };
    static const struct software_node tink_ha_orange_led_node = {
    .name = "ha-orange-led",
    .parent = &tink_gpio_leds_node,
    .properties = tink_ha_orange_led_props,
    };
    static const struct property_entry tink_usb_green_led_props[] = {
    PROPERTY_ENTRY_STRING("label", "mx100:green:usb"),
    PROPERTY_ENTRY_GPIO("gpios", &lpc_ich_gpio_swnode, 21, GPIO_ACTIVE_LOW),
    { }
    };
    static const struct software_node tink_usb_green_led_node = {
    .name = "usb-green-led",
    .parent = &tink_gpio_leds_node,
    .properties = tink_usb_green_led_props,
    };
    static const struct property_entry tink_usb_orange_led_props[] = {
    PROPERTY_ENTRY_STRING("label", "mx100:orange:usb"),
    PROPERTY_ENTRY_GPIO("gpios", &lpc_ich_gpio_swnode, 19, GPIO_ACTIVE_LOW),
    { }
    };
    static const struct software_node tink_usb_orange_led_node = {
    .name = "usb-orange-led",
    .parent = &tink_gpio_leds_node,
    .properties = tink_usb_orange_led_props,
    };
// Reset Button
    static const struct property_entry tink_gpio_keys_props[] = {
    PROPERTY_ENTRY_U32("poll-interval", 20),
    { }
    };
    static const struct software_node tink_gpio_keys_node = {
    .name = "mx100-keys",
    .properties = tink_gpio_keys_props,
    };
    static const struct property_entry tink_reset_key_props[] = {
    PROPERTY_ENTRY_U32("linux,code", KEY_RESTART),
    PROPERTY_ENTRY_STRING("label", "Reset"),
    PROPERTY_ENTRY_GPIO("gpios", &lpc_ich_gpio_swnode, 60, GPIO_ACTIVE_LOW),
    PROPERTY_ENTRY_U32("linux,input-type", EV_KEY),
    PROPERTY_ENTRY_U32("debounce-interval", 100),
    { }
    };
    static const struct software_node tink_reset_key_node = {
    .name = "reset",
    .parent = &tink_gpio_keys_node,
    .properties = tink_reset_key_props,
    };
    static const struct software_node *tink_swnodes[] = {
// LEDs nodes
    &tink_gpio_leds_node,
    &tink_internet_led_node,
    &tink_lan2_led_node,
    &tink_lan3_led_node,
    &tink_lan4_led_node,
    &tink_lan5_led_node,
    &tink_lan6_led_node,
    &tink_lan7_led_node,
    &tink_lan8_led_node,
    &tink_lan9_led_node,
    &tink_lan10_led_node,
    &tink_lan11_led_node,
    &tink_ha_green_led_node,
    &tink_ha_orange_led_node,
    &tink_usb_green_led_node,
    &tink_usb_orange_led_node,
// Keys nodes
    &tink_gpio_keys_node,
    &tink_reset_key_node,
    core::ptr::null_mut()
    };
// Board setup
    static const struct dmi_system_id tink_systems[] __initconst = {
    {
    .matches = {
    DMI_EXACT_MATCH(DMI_SYS_VENDOR, "Cisco"),
    DMI_EXACT_MATCH(DMI_PRODUCT_NAME, "MX100-HW"),
    },
    },
    {} /* Terminating entry */
    };
    MODULE_DEVICE_TABLE(dmi, tink_systems);
    static struct platform_device *tink_leds_pdev;
    static struct platform_device *tink_keys_pdev;
#[no_mangle]
unsafe extern "C" fn tink_board_init() -> int __init {
    static int __init tink_board_init(void)
    {
    struct platform_device_info keys_info = {
    .name = "gpio-keys-polled",
    .id = PLATFORM_DEVID_NONE,
    };
    struct platform_device_info leds_info = {
    .name = "leds-gpio",
    .id = PLATFORM_DEVID_NONE,
    };
    int err;
    if (!dmi_first_match(tink_systems))
    return -ENODEV;
//
// We need to make sure that GPIO60 isn't set to native mode as is default since it's our
// Reset Button. To do this, write to GPIO_USE_SEL2 to have GPIO60 set to GPIO mode.
// This is documented on page 1609 of the PCH datasheet, order number 327879-005US
//
    outl(inl(0x530) | BIT(28), 0x530);
    err = software_node_register_node_group(tink_swnodes);
    if (err) {
    pr_err("failed to register software nodes: %d\n", err);
    return err;
    }
    leds_info.fwnode = software_node_fwnode(&tink_gpio_leds_node);
    tink_leds_pdev = platform_device_register_full(&leds_info);
    if (IS_ERR(tink_leds_pdev)) {
    err = PTR_ERR(tink_leds_pdev);
    pr_err("failed to create LED device: %d\n", err);
    goto err_unregister_swnodes;
    }
    keys_info.fwnode = software_node_fwnode(&tink_gpio_keys_node);
    tink_keys_pdev = platform_device_register_full(&keys_info);
    if (IS_ERR(tink_keys_pdev)) {
    err = PTR_ERR(tink_keys_pdev);
    pr_err("failed to create key device: %d\n", err);
    goto err_unregister_leds;
    }
    return 0;
    err_unregister_leds:
    platform_device_unregister(tink_leds_pdev);
    err_unregister_swnodes:
    software_node_unregister_node_group(tink_swnodes);
    return err;
    }
    module_init(tink_board_init);
#[no_mangle]
unsafe extern "C" fn tink_board_exit() -> void __exit {
    static void __exit tink_board_exit(void)
    {
    platform_device_unregister(tink_keys_pdev);
    platform_device_unregister(tink_leds_pdev);
    software_node_unregister_node_group(tink_swnodes);
    }
    module_exit(tink_board_exit);
    MODULE_AUTHOR("Chris Blake <chrisrblake93@gmail.com>");
    MODULE_DESCRIPTION("Cisco Meraki MX100 Platform Driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:meraki-mx100");
    MODULE_IMPORT_NS("LPC_ICH");
