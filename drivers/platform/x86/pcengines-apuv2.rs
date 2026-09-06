//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/pcengines-apuv2.c
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
// PC-Engines APUv2/APUv3 board platform driver
// for GPIO buttons and LEDs
//
// Copyright (C) 2018 metux IT consult
// Author: Enrico Weigelt <info@metux.net>
//

//
// NOTE: this driver only supports APUv2/3 - not APUv1, as this one
// has completely different register layouts.
//
// Register mappings

// Order in which the GPIO lines are defined in the register list
pub const APU2_GPIO_LINE_LED1: c_int = 0;
pub const APU2_GPIO_LINE_LED2: c_int = 1;
pub const APU2_GPIO_LINE_LED3: c_int = 2;
pub const APU2_GPIO_LINE_MODESW: c_int = 3;
pub const APU2_GPIO_LINE_SIMSWAP: c_int = 4;
pub const APU2_GPIO_LINE_MPCIE2: c_int = 5;
pub const APU2_GPIO_LINE_MPCIE3: c_int = 6;
// GPIO device
    static int apu2_gpio_regs[] = {
    [APU2_GPIO_LINE_LED1]		= APU2_GPIO_REG_LED1,
    [APU2_GPIO_LINE_LED2]		= APU2_GPIO_REG_LED2,
    [APU2_GPIO_LINE_LED3]		= APU2_GPIO_REG_LED3,
    [APU2_GPIO_LINE_MODESW]		= APU2_GPIO_REG_MODESW,
    [APU2_GPIO_LINE_SIMSWAP]	= APU2_GPIO_REG_SIMSWAP,
    [APU2_GPIO_LINE_MPCIE2]		= APU2_GPIO_REG_MPCIE2,
    [APU2_GPIO_LINE_MPCIE3]		= APU2_GPIO_REG_MPCIE3,
    };
    static const char * const apu2_gpio_names[] = {
    [APU2_GPIO_LINE_LED1]		= "front-led1",
    [APU2_GPIO_LINE_LED2]		= "front-led2",
    [APU2_GPIO_LINE_LED3]		= "front-led3",
    [APU2_GPIO_LINE_MODESW]		= "front-button",
    [APU2_GPIO_LINE_SIMSWAP]	= "simswap",
    [APU2_GPIO_LINE_MPCIE2]		= "mpcie2_reset",
    [APU2_GPIO_LINE_MPCIE3]		= "mpcie3_reset",
    };
    static const struct amd_fch_gpio_pdata board_apu2 = {
    .gpio_num	= ARRAY_SIZE(apu2_gpio_regs),
    .gpio_reg	= apu2_gpio_regs,
    .gpio_names	= apu2_gpio_names,
    };
    static const struct software_node apu2_gpiochip_node = {
    .name = AMD_FCH_GPIO_DRIVER_NAME,
    };
// GPIO LEDs device
    static const struct software_node apu2_leds_node = {
    .name = "apu2-leds",
    };
    static const struct property_entry apu2_led1_props[] = {
    PROPERTY_ENTRY_STRING("label", "apu:green:1"),
    PROPERTY_ENTRY_GPIO("gpios", &apu2_gpiochip_node,
    APU2_GPIO_LINE_LED1, GPIO_ACTIVE_LOW),
    { }
    };
    static const struct software_node apu2_led1_swnode = {
    .name = "led-1",
    .parent = &apu2_leds_node,
    .properties = apu2_led1_props,
    };
    static const struct property_entry apu2_led2_props[] = {
    PROPERTY_ENTRY_STRING("label", "apu:green:2"),
    PROPERTY_ENTRY_GPIO("gpios", &apu2_gpiochip_node,
    APU2_GPIO_LINE_LED2, GPIO_ACTIVE_LOW),
    { }
    };
    static const struct software_node apu2_led2_swnode = {
    .name = "led-2",
    .parent = &apu2_leds_node,
    .properties = apu2_led2_props,
    };
    static const struct property_entry apu2_led3_props[] = {
    PROPERTY_ENTRY_STRING("label", "apu:green:3"),
    PROPERTY_ENTRY_GPIO("gpios", &apu2_gpiochip_node,
    APU2_GPIO_LINE_LED3, GPIO_ACTIVE_LOW),
    { }
    };
    static const struct software_node apu2_led3_swnode = {
    .name = "led-3",
    .parent = &apu2_leds_node,
    .properties = apu2_led3_props,
    };
// GPIO keyboard device
    static const struct property_entry apu2_keys_props[] = {
    PROPERTY_ENTRY_U32("poll-interval", 100),
    { }
    };
    static const struct software_node apu2_keys_node = {
    .name = "apu2-keys",
    .properties = apu2_keys_props,
    };
    static const struct property_entry apu2_front_button_props[] = {
    PROPERTY_ENTRY_STRING("label", "front button"),
    PROPERTY_ENTRY_U32("linux,code", KEY_RESTART),
    PROPERTY_ENTRY_GPIO("gpios", &apu2_gpiochip_node,
    APU2_GPIO_LINE_MODESW, GPIO_ACTIVE_LOW),
    PROPERTY_ENTRY_U32("debounce-interval", 10),
    { }
    };
    static const struct software_node apu2_front_button_swnode = {
    .name = "front-button",
    .parent = &apu2_keys_node,
    .properties = apu2_front_button_props,
    };
    static const struct software_node *apu2_swnodes[] = {
    &apu2_gpiochip_node,
// LEDs nodes
    &apu2_leds_node,
    &apu2_led1_swnode,
    &apu2_led2_swnode,
    &apu2_led3_swnode,
// Keys nodes
    &apu2_keys_node,
    &apu2_front_button_swnode,
    core::ptr::null_mut()
    };
// Board setup
// Note: matching works on string prefix, so "apu2" must come before "apu"
    static const struct dmi_system_id apu_gpio_dmi_table[] __initconst = {
// APU2 w/ legacy BIOS < 4.0.8
    {
    .ident		= "apu2",
    .matches	= {
    DMI_MATCH(DMI_SYS_VENDOR, "PC Engines"),
    DMI_MATCH(DMI_BOARD_NAME, "APU2")
    },
    .driver_data	= (void *)&board_apu2,
    },
// APU2 w/ legacy BIOS >= 4.0.8
    {
    .ident		= "apu2",
    .matches	= {
    DMI_MATCH(DMI_SYS_VENDOR, "PC Engines"),
    DMI_MATCH(DMI_BOARD_NAME, "apu2")
    },
    .driver_data	= (void *)&board_apu2,
    },
// APU2 w/ mainline BIOS
    {
    .ident		= "apu2",
    .matches	= {
    DMI_MATCH(DMI_SYS_VENDOR, "PC Engines"),
    DMI_MATCH(DMI_BOARD_NAME, "PC Engines apu2")
    },
    .driver_data	= (void *)&board_apu2,
    },
// APU3 w/ legacy BIOS < 4.0.8
    {
    .ident		= "apu3",
    .matches	= {
    DMI_MATCH(DMI_SYS_VENDOR, "PC Engines"),
    DMI_MATCH(DMI_BOARD_NAME, "APU3")
    },
    .driver_data = (void *)&board_apu2,
    },
// APU3 w/ legacy BIOS >= 4.0.8
    {
    .ident       = "apu3",
    .matches     = {
    DMI_MATCH(DMI_SYS_VENDOR, "PC Engines"),
    DMI_MATCH(DMI_BOARD_NAME, "apu3")
    },
    .driver_data = (void *)&board_apu2,
    },
// APU3 w/ mainline BIOS
    {
    .ident       = "apu3",
    .matches     = {
    DMI_MATCH(DMI_SYS_VENDOR, "PC Engines"),
    DMI_MATCH(DMI_BOARD_NAME, "PC Engines apu3")
    },
    .driver_data = (void *)&board_apu2,
    },
// APU4 w/ legacy BIOS < 4.0.8
    {
    .ident        = "apu4",
    .matches    = {
    DMI_MATCH(DMI_SYS_VENDOR, "PC Engines"),
    DMI_MATCH(DMI_BOARD_NAME, "APU4")
    },
    .driver_data = (void *)&board_apu2,
    },
// APU4 w/ legacy BIOS >= 4.0.8
    {
    .ident       = "apu4",
    .matches     = {
    DMI_MATCH(DMI_SYS_VENDOR, "PC Engines"),
    DMI_MATCH(DMI_BOARD_NAME, "apu4")
    },
    .driver_data = (void *)&board_apu2,
    },
// APU4 w/ mainline BIOS
    {
    .ident       = "apu4",
    .matches     = {
    DMI_MATCH(DMI_SYS_VENDOR, "PC Engines"),
    DMI_MATCH(DMI_BOARD_NAME, "PC Engines apu4")
    },
    .driver_data = (void *)&board_apu2,
    },
    {}
    };
    static struct platform_device *apu_gpio_pdev;
    static struct platform_device *apu_leds_pdev;
    static struct platform_device *apu_keys_pdev;
    static struct platform_device * __init apu_create_pdev(const char *name,
    const void *data, size_t size,
    const struct software_node *swnode)
    {
    struct platform_device_info pdev_info = {
    .name = name,
    .id = PLATFORM_DEVID_NONE,
    .data = data,
    .size_data = size,
    .swnode = swnode,
    };
    struct platform_device *pdev;
    int err;
    pdev = platform_device_register_full(&pdev_info);
    err = PTR_ERR_OR_ZERO(pdev);
    if (err)
    pr_err("failed registering %s: %d\n", name, err);
    return pdev;
    }
#[no_mangle]
unsafe extern "C" fn apu_board_init() -> int __init {
    static int __init apu_board_init(void)
    {
    const struct dmi_system_id *id;
    int err;
    id = dmi_first_match(apu_gpio_dmi_table);
    if (!id) {
    pr_err("failed to detect APU board via DMI\n");
    return -ENODEV;
    }
    err = software_node_register_node_group(apu2_swnodes);
    if (err) {
    pr_err("failed to register software nodes: %d\n", err);
    return err;
    }
    apu_gpio_pdev = apu_create_pdev(AMD_FCH_GPIO_DRIVER_NAME,
    id.driver_data, sizeof(struct amd_fch_gpio_pdata),
    &apu2_gpiochip_node);
    err = PTR_ERR_OR_ZERO(apu_gpio_pdev);
    if (err)
    goto err_unregister_swnodes;
    apu_leds_pdev = apu_create_pdev("leds-gpio", core::ptr::null_mut(), 0, &apu2_leds_node);
    err = PTR_ERR_OR_ZERO(apu_leds_pdev);
    if (err)
    goto err_unregister_gpio;
    apu_keys_pdev = apu_create_pdev("gpio-keys-polled", core::ptr::null_mut(), 0, &apu2_keys_node);
    err = PTR_ERR_OR_ZERO(apu_keys_pdev);
    if (err)
    goto err_unregister_leds;
    return 0;
    err_unregister_leds:
    platform_device_unregister(apu_leds_pdev);
    err_unregister_gpio:
    platform_device_unregister(apu_gpio_pdev);
    err_unregister_swnodes:
    software_node_unregister_node_group(apu2_swnodes);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn apu_board_exit() -> void __exit {
    static void __exit apu_board_exit(void)
    {
    platform_device_unregister(apu_keys_pdev);
    platform_device_unregister(apu_leds_pdev);
    platform_device_unregister(apu_gpio_pdev);
    software_node_unregister_node_group(apu2_swnodes);
    }
    module_init(apu_board_init);
    module_exit(apu_board_exit);
    MODULE_AUTHOR("Enrico Weigelt, metux IT consult <info@metux.net>");
    MODULE_DESCRIPTION("PC Engines APUv2/APUv3 board GPIO/LEDs/keys driver");
    MODULE_LICENSE("GPL");
    MODULE_DEVICE_TABLE(dmi, apu_gpio_dmi_table);
    MODULE_SOFTDEP("pre: platform:" AMD_FCH_GPIO_DRIVER_NAME " platform:leds-gpio platform:gpio_keys_polled");
