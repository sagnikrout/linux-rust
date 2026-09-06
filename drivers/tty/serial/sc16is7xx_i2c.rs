//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/sc16is7xx_i2c.c
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
// SC16IS7xx I2C interface driver

#[no_mangle]
unsafe extern "C" fn sc16is7xx_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int sc16is7xx_i2c_probe(struct i2c_client *i2c)
    {
    const struct sc16is7xx_devtype *devtype;
    struct regmap *regmaps[SC16IS7XX_MAX_PORTS];
    struct regmap_config regcfg;
    unsigned int i;
    devtype = i2c_get_match_data(i2c);
    if (!devtype)
    return dev_err_probe(&i2c.dev, -ENODEV, "Failed to match device\n");
    memcpy(&regcfg, &sc16is7xx_regcfg, sizeof(struct regmap_config));
    for (i = 0; i < devtype.nr_uart; i++) {
    regcfg.name = sc16is7xx_regmap_name(i);
    regcfg.read_flag_mask = sc16is7xx_regmap_port_mask(i);
    regcfg.write_flag_mask = sc16is7xx_regmap_port_mask(i);
    regmaps[i] = devm_regmap_init_i2c(i2c, &regcfg);
    }
    return sc16is7xx_probe(&i2c.dev, devtype, regmaps, i2c.irq);
    }
#[no_mangle]
unsafe extern "C" fn sc16is7xx_i2c_remove(client: *mut i2c_client) {
    static void sc16is7xx_i2c_remove(struct i2c_client *client)
    {
    sc16is7xx_remove(&client.dev);
    }
    static const struct i2c_device_id sc16is7xx_i2c_id_table[] = {
    { .name = "sc16is74x", .driver_data = (kernel_ulong_t)&sc16is74x_devtype },
    { .name = "sc16is740", .driver_data = (kernel_ulong_t)&sc16is74x_devtype },
    { .name = "sc16is741", .driver_data = (kernel_ulong_t)&sc16is74x_devtype },
    { .name = "sc16is750", .driver_data = (kernel_ulong_t)&sc16is750_devtype },
    { .name = "sc16is752", .driver_data = (kernel_ulong_t)&sc16is752_devtype },
    { .name = "sc16is760", .driver_data = (kernel_ulong_t)&sc16is760_devtype },
    { .name = "sc16is762", .driver_data = (kernel_ulong_t)&sc16is762_devtype },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, sc16is7xx_i2c_id_table);
    static struct i2c_driver sc16is7xx_i2c_driver = {
    .driver = {
    .name		= KBUILD_MODNAME,
    .of_match_table	= sc16is7xx_dt_ids,
    },
    .probe		= sc16is7xx_i2c_probe,
    .remove		= sc16is7xx_i2c_remove,
    .id_table	= sc16is7xx_i2c_id_table,
    };
    module_i2c_driver(sc16is7xx_i2c_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION(KBUILD_MODNAME " interface driver");
    MODULE_IMPORT_NS("SERIAL_NXP_SC16IS7XX");
