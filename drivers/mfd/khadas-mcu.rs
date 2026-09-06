//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/khadas-mcu.c
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
// Driver for Khadas System control Microcontroller
//
// Copyright (C) 2020 BayLibre SAS
//
// Author(s): Neil Armstrong <narmstrong@baylibre.com>
//

#[no_mangle]
unsafe extern "C" fn khadas_mcu_reg_volatile(dev: *mut device, reg: c_uint) -> bool {
    static bool khadas_mcu_reg_volatile(struct device *dev, unsigned int reg)
    {
    if (reg >= KHADAS_MCU_USER_DATA_0_REG &&
    reg < KHADAS_MCU_PWR_OFF_CMD_REG)
    return true;
    switch (reg) {
    case KHADAS_MCU_PWR_OFF_CMD_REG:
    case KHADAS_MCU_PASSWD_START_REG:
    case KHADAS_MCU_CHECK_VEN_PASSWD_REG:
    case KHADAS_MCU_CHECK_USER_PASSWD_REG:
    case KHADAS_MCU_WOL_INIT_START_REG:
    case KHADAS_MCU_CMD_FAN_STATUS_CTRL_REG:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn khadas_mcu_reg_writeable(dev: *mut device, reg: c_uint) -> bool {
    static bool khadas_mcu_reg_writeable(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case KHADAS_MCU_PASSWD_VEN_0_REG:
    case KHADAS_MCU_PASSWD_VEN_1_REG:
    case KHADAS_MCU_PASSWD_VEN_2_REG:
    case KHADAS_MCU_PASSWD_VEN_3_REG:
    case KHADAS_MCU_PASSWD_VEN_4_REG:
    case KHADAS_MCU_PASSWD_VEN_5_REG:
    case KHADAS_MCU_MAC_0_REG:
    case KHADAS_MCU_MAC_1_REG:
    case KHADAS_MCU_MAC_2_REG:
    case KHADAS_MCU_MAC_3_REG:
    case KHADAS_MCU_MAC_4_REG:
    case KHADAS_MCU_MAC_5_REG:
    case KHADAS_MCU_USID_0_REG:
    case KHADAS_MCU_USID_1_REG:
    case KHADAS_MCU_USID_2_REG:
    case KHADAS_MCU_USID_3_REG:
    case KHADAS_MCU_USID_4_REG:
    case KHADAS_MCU_USID_5_REG:
    case KHADAS_MCU_VERSION_0_REG:
    case KHADAS_MCU_VERSION_1_REG:
    case KHADAS_MCU_DEVICE_NO_0_REG:
    case KHADAS_MCU_DEVICE_NO_1_REG:
    case KHADAS_MCU_FACTORY_TEST_REG:
    case KHADAS_MCU_SHUTDOWN_NORMAL_STATUS_REG:
    return false;
    default:
    return true;
    }
    }
    static const struct regmap_config khadas_mcu_regmap_config = {
    .reg_bits	= 8,
    .reg_stride	= 1,
    .val_bits	= 8,
    .max_register	= KHADAS_MCU_CMD_FAN_STATUS_CTRL_REG,
    .volatile_reg	= khadas_mcu_reg_volatile,
    .writeable_reg	= khadas_mcu_reg_writeable,
    .cache_type	= REGCACHE_MAPLE,
    };
    static struct mfd_cell khadas_mcu_fan_cells[] = {
// VIM1/2 Rev13+ and VIM3 only
    { .name = "khadas-mcu-fan-ctrl", },
    };
    static struct mfd_cell khadas_mcu_cells[] = {
    { .name = "khadas-mcu-user-mem", },
    };
#[no_mangle]
unsafe extern "C" fn khadas_mcu_probe(client: *mut i2c_client) -> c_int {
    static int khadas_mcu_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct khadas_mcu *ddata;
    int ret;
    ddata = devm_kzalloc(dev, sizeof(*ddata), GFP_KERNEL);
    if (!ddata)
    return -ENOMEM;
    i2c_set_clientdata(client, ddata);
    ddata.dev = dev;
    ddata.regmap = devm_regmap_init_i2c(client, &khadas_mcu_regmap_config);
    if (IS_ERR(ddata.regmap)) {
    ret = PTR_ERR(ddata.regmap);
    dev_err(dev, "Failed to allocate register map: %d\n", ret);
    return ret;
    }
    ret = devm_mfd_add_devices(dev, PLATFORM_DEVID_NONE,
    khadas_mcu_cells,
    ARRAY_SIZE(khadas_mcu_cells),
    core::ptr::null_mut(), 0, core::ptr::null_mut());
    if (ret)
    return ret;
    if (of_property_present(dev.of_node, "#cooling-cells"))
    return devm_mfd_add_devices(dev, PLATFORM_DEVID_NONE,
    khadas_mcu_fan_cells,
    ARRAY_SIZE(khadas_mcu_fan_cells),
    core::ptr::null_mut(), 0, core::ptr::null_mut());
    return 0;
    }

    static const struct of_device_id khadas_mcu_of_match[] = {
    { .compatible = "khadas,mcu", },
    {},
    };
    MODULE_DEVICE_TABLE(of, khadas_mcu_of_match);

    static struct i2c_driver khadas_mcu_driver = {
    .driver = {
    .name = "khadas-mcu-core",
    .of_match_table = of_match_ptr(khadas_mcu_of_match),
    },
    .probe = khadas_mcu_probe,
    };
    module_i2c_driver(khadas_mcu_driver);
    MODULE_DESCRIPTION("Khadas MCU core driver");
    MODULE_AUTHOR("Neil Armstrong <narmstrong@baylibre.com>");
    MODULE_LICENSE("GPL v2");
