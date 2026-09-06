//! Automatically rewritten from C to Rust
//! Source: drivers/platform/chrome/chromeos_of_hw_prober.c
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
// ChromeOS Device Tree Hardware Prober
//
// Copyright (c) 2024 Google LLC
//

//
// struct hw_prober_entry - Holds an entry for the hardware prober
//
// @compatible:	compatible string to match against the machine
// @prober:	prober function to call when machine matches
// @data:	extra data for the prober function
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_prober_entry {
    pub compatible: *const c_char,
    pub data): *const *const *const int (prober)(struct device dev, void,
    pub data: *const c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chromeos_i2c_probe_data {
    pub cfg: *const i2c_of_probe_cfg,
    pub opts: *const i2c_of_probe_simple_opts,
}

#[no_mangle]
unsafe extern "C" fn chromeos_i2c_component_prober(dev: *mut device, _data: *const c_void) -> c_int {
    static int chromeos_i2c_component_prober(struct device *dev, const void *_data)
    {
    const struct chromeos_i2c_probe_data *data = _data;
    struct i2c_of_probe_simple_ctx ctx = {
    .opts = data.opts,
    };
    return i2c_of_probe_component(dev, data.cfg, &ctx);
    }

    static const struct i2c_of_probe_cfg chromeos_i2c_probe_simple_ ## _type ## _cfg = {	\
    .type = #_type,									\
    .ops = &i2c_of_probe_simple_ops,						\
    }

    static const struct chromeos_i2c_probe_data chromeos_i2c_probe_dumb_ ## _type = {	\
    .cfg = &(const struct i2c_of_probe_cfg) {					\
    .type = #_type,								\
    },										\
    }
    DEFINE_CHROMEOS_I2C_PROBE_DATA_DUMB_BY_TYPE(touchscreen);
    DEFINE_CHROMEOS_I2C_PROBE_DATA_DUMB_BY_TYPE(trackpad);
    DEFINE_CHROMEOS_I2C_PROBE_CFG_SIMPLE_BY_TYPE(touchscreen);
    DEFINE_CHROMEOS_I2C_PROBE_CFG_SIMPLE_BY_TYPE(trackpad);
    static const struct chromeos_i2c_probe_data chromeos_i2c_probe_hana_trackpad = {
    .cfg = &chromeos_i2c_probe_simple_trackpad_cfg,
    .opts = &(const struct i2c_of_probe_simple_opts) {
    .res_node_compatible = "elan,ekth3000",
    .supply_name = "vcc",
//
// ELAN trackpad needs 2 ms for H/W init and 100 ms for F/W init.
// Synaptics trackpad needs 100 ms.
//
    .post_power_on_delay_ms = 110,
    },
    };
    static const struct chromeos_i2c_probe_data chromeos_i2c_probe_squirtle_touchscreen = {
    .cfg = &chromeos_i2c_probe_simple_touchscreen_cfg,
    .opts = &(const struct i2c_of_probe_simple_opts) {
    .res_node_compatible = "elan,ekth6a12nay",
    .supply_name = "vcc33",
    .gpio_name = "reset",
    .post_power_on_delay_ms = 10,
    .post_gpio_config_delay_ms = 300,
    },
    };
    static const struct hw_prober_entry hw_prober_platforms[] = {
    {
    .compatible = "google,hana",
    .prober = chromeos_i2c_component_prober,
    .data = &chromeos_i2c_probe_dumb_touchscreen,
    }, {
    .compatible = "google,hana",
    .prober = chromeos_i2c_component_prober,
    .data = &chromeos_i2c_probe_hana_trackpad,
    }, {
    .compatible = "google,spherion",
    .prober = chromeos_i2c_component_prober,
    .data = &chromeos_i2c_probe_dumb_trackpad,
    }, {
    .compatible = "google,squirtle",
    .prober = chromeos_i2c_component_prober,
    .data = &chromeos_i2c_probe_dumb_trackpad,
    }, {
    .compatible = "google,squirtle",
    .prober = chromeos_i2c_component_prober,
    .data = &chromeos_i2c_probe_squirtle_touchscreen,
    }, {
    .compatible = "google,steelix",
    .prober = chromeos_i2c_component_prober,
    .data = &chromeos_i2c_probe_dumb_trackpad,
    }, {
    .compatible = "google,voltorb",
    .prober = chromeos_i2c_component_prober,
    .data = &chromeos_i2c_probe_dumb_trackpad,
    },
    };
#[no_mangle]
unsafe extern "C" fn chromeos_of_hw_prober_probe(pdev: *mut platform_device) -> c_int {
    static int chromeos_of_hw_prober_probe(struct platform_device *pdev)
    {
    for (size_t i = 0; i < ARRAY_SIZE(hw_prober_platforms); i++) {
    int ret;
    if (!of_machine_is_compatible(hw_prober_platforms[i].compatible))
    continue;
    ret = hw_prober_platforms[i].prober(&pdev.dev, hw_prober_platforms[i].data);
// Ignore unrecoverable errors and keep going through other probers
    if (ret == -EPROBE_DEFER)
    return ret;
    }
    return 0;
    }
    static struct platform_driver chromeos_of_hw_prober_driver = {
    .probe	= chromeos_of_hw_prober_probe,
    .driver	= {
    .name = DRV_NAME,
    },
    };
    static struct platform_device *chromeos_of_hw_prober_pdev;
#[no_mangle]
unsafe extern "C" fn chromeos_of_hw_prober_driver_init() -> c_int {
    static int chromeos_of_hw_prober_driver_init(void)
    {
    size_t i;
    int ret;
    for (i = 0; i < ARRAY_SIZE(hw_prober_platforms); i++)
    if (of_machine_is_compatible(hw_prober_platforms[i].compatible))
    break;
    if (i == ARRAY_SIZE(hw_prober_platforms))
    return -ENODEV;
    ret = platform_driver_register(&chromeos_of_hw_prober_driver);
    if (ret)
    return ret;
    chromeos_of_hw_prober_pdev =
    platform_device_register_simple(DRV_NAME, PLATFORM_DEVID_NONE, core::ptr::null_mut(), 0);
    if (IS_ERR(chromeos_of_hw_prober_pdev))
    goto err;
    return 0;
    err:
    platform_driver_unregister(&chromeos_of_hw_prober_driver);
    return PTR_ERR(chromeos_of_hw_prober_pdev);
    }
    module_init(chromeos_of_hw_prober_driver_init);
#[no_mangle]
unsafe extern "C" fn chromeos_of_hw_prober_driver_exit() {
    static void chromeos_of_hw_prober_driver_exit(void)
    {
    platform_device_unregister(chromeos_of_hw_prober_pdev);
    platform_driver_unregister(&chromeos_of_hw_prober_driver);
    }
    module_exit(chromeos_of_hw_prober_driver_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("ChromeOS device tree hardware prober");
    MODULE_IMPORT_NS("I2C_OF_PROBER");
