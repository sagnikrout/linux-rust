//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/acer-ec-a500.c
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
// Acer Iconia Tab A500 Embedded Controller Driver
//
// Copyright 2020 GRATE-driver project
//

pub const A500_EC_I2C_ERR_TIMEOUT: c_int = 500;
pub const A500_EC_POWER_CMD_TIMEOUT: c_int = 1000;
//
// Controller's firmware expects specific command opcodes to be used for the
// corresponding registers. Unsupported commands are skipped by the firmware.
//
pub const CMD_SHUTDOWN: c_uint = 0x0;
pub const CMD_WARM_REBOOT: c_uint = 0x0;
pub const CMD_COLD_REBOOT: c_uint = 0x1;
    enum {
    REG_CURRENT_NOW = 0x03,
    REG_SHUTDOWN = 0x52,
    REG_WARM_REBOOT = 0x54,
    REG_COLD_REBOOT = 0x55,
    };
    static struct i2c_client *a500_ec_client_pm_off;
    static int a500_ec_read(void *context, const void *reg_buf, size_t reg_size,
    void *val_buf, size_t val_sizel)
    {
    struct i2c_client *client = context;
    unsigned int reg, retries = 5;
    u16 *ret_val = val_buf;
    let mut ret: i32 = 0;
    reg = *(u8 *)reg_buf;
    while (retries-- > 0) {
    ret = i2c_smbus_read_word_data(client, reg);
    if (ret >= 0)
    break;
    msleep(A500_EC_I2C_ERR_TIMEOUT);
    }
    if (ret < 0) {
    dev_err(&client.dev, "read 0x%x failed: %d\n", reg, ret);
    return ret;
    }
// ret_val = ret;
    if (reg == REG_CURRENT_NOW)
    fsleep(10000);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn a500_ec_write(context: *mut c_void, data: *const c_void, count: usize) -> c_int {
    static int a500_ec_write(void *context, const void *data, size_t count)
    {
    struct i2c_client *client = context;
    unsigned int reg, val, retries = 5;
    let mut ret: i32 = 0;
    reg = *(u8  *)(data + 0);
    val = *(u16 *)(data + 1);
    while (retries-- > 0) {
    ret = i2c_smbus_write_word_data(client, reg, val);
    if (ret >= 0)
    break;
    msleep(A500_EC_I2C_ERR_TIMEOUT);
    }
    if (ret < 0) {
    dev_err(&client.dev, "write 0x%x failed: %d\n", reg, ret);
    return ret;
    }
    return 0;
    }
    static const struct regmap_config a500_ec_regmap_config = {
    .name = "KB930",
    .reg_bits = 8,
    .val_bits = 16,
    .max_register = 0xff,
    };
    static const struct regmap_bus a500_ec_regmap_bus = {
    .reg_format_endian_default = REGMAP_ENDIAN_NATIVE,
    .val_format_endian_default = REGMAP_ENDIAN_LITTLE,
    .write = a500_ec_write,
    .read = a500_ec_read,
    .max_raw_read = 2,
    };
#[no_mangle]
unsafe extern "C" fn a500_ec_poweroff() {
    static void a500_ec_poweroff(void)
    {
    i2c_smbus_write_word_data(a500_ec_client_pm_off,
    REG_SHUTDOWN, CMD_SHUTDOWN);
    mdelay(A500_EC_POWER_CMD_TIMEOUT);
    }
    static int a500_ec_restart_notify(struct notifier_block *this,
    unsigned long reboot_mode, void *data)
    {
    if (reboot_mode == REBOOT_WARM)
    i2c_smbus_write_word_data(a500_ec_client_pm_off,
    REG_WARM_REBOOT, CMD_WARM_REBOOT);
    else
    i2c_smbus_write_word_data(a500_ec_client_pm_off,
    REG_COLD_REBOOT, CMD_COLD_REBOOT);
    mdelay(A500_EC_POWER_CMD_TIMEOUT);
    return NOTIFY_DONE;
    }
    static struct notifier_block a500_ec_restart_handler = {
    .notifier_call = a500_ec_restart_notify,
    .priority = 200,
    };
    static const struct mfd_cell a500_ec_cells[] = {
    { .name = "acer-a500-iconia-battery", },
    { .name = "acer-a500-iconia-leds", },
    };
#[no_mangle]
unsafe extern "C" fn a500_ec_probe(client: *mut i2c_client) -> c_int {
    static int a500_ec_probe(struct i2c_client *client)
    {
    struct regmap *regmap;
    int err;
    regmap = devm_regmap_init(&client.dev, &a500_ec_regmap_bus,
    client, &a500_ec_regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    err = devm_mfd_add_devices(&client.dev, PLATFORM_DEVID_AUTO,
    a500_ec_cells, ARRAY_SIZE(a500_ec_cells),
    core::ptr::null_mut(), 0, core::ptr::null_mut());
    if (err) {
    dev_err(&client.dev, "failed to add sub-devices: %d\n", err);
    return err;
    }
    if (of_device_is_system_power_controller(client.dev.of_node)) {
    a500_ec_client_pm_off = client;
    err = register_restart_handler(&a500_ec_restart_handler);
    if (err)
    return err;
    if (!pm_power_off)
    pm_power_off = a500_ec_poweroff;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn a500_ec_remove(client: *mut i2c_client) {
    static void a500_ec_remove(struct i2c_client *client)
    {
    if (of_device_is_system_power_controller(client.dev.of_node)) {
    if (pm_power_off == a500_ec_poweroff)
    pm_power_off = core::ptr::null_mut();
    unregister_restart_handler(&a500_ec_restart_handler);
    }
    }
    static const struct of_device_id a500_ec_match[] = {
    { .compatible = "acer,a500-iconia-ec" },
    { }
    };
    MODULE_DEVICE_TABLE(of, a500_ec_match);
    static struct i2c_driver a500_ec_driver = {
    .driver = {
    .name = "acer-a500-embedded-controller",
    .of_match_table = a500_ec_match,
    },
    .probe = a500_ec_probe,
    .remove = a500_ec_remove,
    };
    module_i2c_driver(a500_ec_driver);
    MODULE_DESCRIPTION("Acer Iconia Tab A500 Embedded Controller driver");
    MODULE_AUTHOR("Dmitry Osipenko <digetx@gmail.com>");
    MODULE_LICENSE("GPL");
