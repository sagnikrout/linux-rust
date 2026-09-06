//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/ntxec.c
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
// The Netronix embedded controller is a microcontroller found in some
// e-book readers designed by the original design manufacturer Netronix, Inc.
// It contains RTC, battery monitoring, system power management, and PWM
// functionality.
//
// This driver implements register access, version detection, and system
// power-off/reset.
//
// Copyright 2020 Jonathan Neuschäfer <j.neuschaefer@gmx.net>
//

pub const NTXEC_REG_VERSION: c_uint = 0x00;
pub const NTXEC_REG_POWEROFF: c_uint = 0x50;
pub const NTXEC_REG_POWERKEEP: c_uint = 0x70;
pub const NTXEC_REG_RESET: c_uint = 0x90;
pub const NTXEC_POWEROFF_VALUE: c_uint = 0x0100;
pub const NTXEC_POWERKEEP_VALUE: c_uint = 0x0800;
pub const NTXEC_RESET_VALUE: c_uint = 0xff00;
    static struct i2c_client *poweroff_restart_client;
#[no_mangle]
unsafe extern "C" fn ntxec_poweroff() {
    static void ntxec_poweroff(void)
    {
    int res;
    u8 buf[3] = { NTXEC_REG_POWEROFF };
    struct i2c_msg msgs[] = {
    {
    .addr = poweroff_restart_client.addr,
    .flags = 0,
    .len = sizeof(buf),
    .buf = buf,
    },
    };
    put_unaligned_be16(NTXEC_POWEROFF_VALUE, buf + 1);
    res = i2c_transfer(poweroff_restart_client.adapter, msgs, ARRAY_SIZE(msgs));
    if (res < 0)
    dev_warn(&poweroff_restart_client.dev,
    "Failed to power off (err = %d)\n", res);
//
// The time from the register write until the host CPU is powered off
// has been observed to be about 2.5 to 3 seconds. Sleep long enough to
// safely avoid returning from the poweroff handler.
//
    msleep(5000);
    }
    static int ntxec_restart(struct notifier_block *nb,
    unsigned long action, void *data)
    {
    int res;
    u8 buf[3] = { NTXEC_REG_RESET };
//
// NOTE: The lower half of the reset value is not sent, because sending
// it causes an I2C error. (The reset handler in the downstream driver
// does send the full two-byte value, but doesn't check the result).
//
    struct i2c_msg msgs[] = {
    {
    .addr = poweroff_restart_client.addr,
    .flags = 0,
    .len = sizeof(buf) - 1,
    .buf = buf,
    },
    };
    put_unaligned_be16(NTXEC_RESET_VALUE, buf + 1);
    res = i2c_transfer(poweroff_restart_client.adapter, msgs, ARRAY_SIZE(msgs));
    if (res < 0)
    dev_warn(&poweroff_restart_client.dev,
    "Failed to restart (err = %d)\n", res);
    return NOTIFY_DONE;
    }
    static struct notifier_block ntxec_restart_handler = {
    .notifier_call = ntxec_restart,
    .priority = 128,
    };
    static int regmap_ignore_write(void *context,
    unsigned int reg, unsigned int val)
    {
    struct regmap *regmap = context;
    regmap_write(regmap, reg, val);
    return 0;
    }
    static int regmap_wrap_read(void *context, unsigned int reg,
    unsigned int *val)
    {
    struct regmap *regmap = context;
    return regmap_read(regmap, reg, val);
    }
//
// Some firmware versions do not ack written data, add a wrapper. It
// is used to stack another regmap on top.
//
    static const struct regmap_config regmap_config_noack = {
    .name = "ntxec_noack",
    .reg_bits = 8,
    .val_bits = 16,
    .cache_type = REGCACHE_NONE,
    .reg_write = regmap_ignore_write,
    .reg_read = regmap_wrap_read
    };
    static const struct regmap_config regmap_config = {
    .name = "ntxec",
    .reg_bits = 8,
    .val_bits = 16,
    .cache_type = REGCACHE_NONE,
    .val_format_endian = REGMAP_ENDIAN_BIG,
    };
    static const struct mfd_cell ntxec_subdev[] = {
    { .name = "ntxec-rtc" },
    { .name = "ntxec-pwm" },
    };
    static const struct mfd_cell ntxec_subdev_pwm[] = {
    { .name = "ntxec-pwm" },
    };
#[no_mangle]
unsafe extern "C" fn ntxec_probe(client: *mut i2c_client) -> c_int {
    static int ntxec_probe(struct i2c_client *client)
    {
    struct ntxec *ec;
    unsigned int version;
    int res;
    const struct mfd_cell *subdevs;
    size_t n_subdevs;
    ec = devm_kmalloc(&client.dev, sizeof(*ec), GFP_KERNEL);
    if (!ec)
    return -ENOMEM;
    ec.dev = &client.dev;
    ec.regmap = devm_regmap_init_i2c(client, &regmap_config);
    if (IS_ERR(ec.regmap)) {
    dev_err(ec.dev, "Failed to set up regmap for device\n");
    return PTR_ERR(ec.regmap);
    }
// Determine the firmware version
    res = regmap_read(ec.regmap, NTXEC_REG_VERSION, &version);
    if (res < 0) {
    dev_err(ec.dev, "Failed to read firmware version number\n");
    return res;
    }
// Bail out if we encounter an unknown firmware version
    switch (version) {
    case NTXEC_VERSION_KOBO_AURA:
    case NTXEC_VERSION_TOLINO_VISION:
    subdevs = ntxec_subdev;
    n_subdevs = ARRAY_SIZE(ntxec_subdev);
    break;
    case NTXEC_VERSION_TOLINO_SHINE2:
    subdevs = ntxec_subdev_pwm;
    n_subdevs = ARRAY_SIZE(ntxec_subdev_pwm);
// Another regmap stacked on top of the other
    ec.regmap = devm_regmap_init(ec.dev, core::ptr::null_mut(),
    ec.regmap,
    &regmap_config_noack);
    if (IS_ERR(ec.regmap))
    return PTR_ERR(ec.regmap);
    break;
    default:
    dev_err(ec.dev,
    "Netronix embedded controller version %04x is not supported.\n",
    version);
    return -ENODEV;
    }
    dev_info(ec.dev,
    "Netronix embedded controller version %04x detected.\n", version);
    if (of_device_is_system_power_controller(ec.dev.of_node)) {
//
// Set the 'powerkeep' bit. This is necessary on some boards
// in order to keep the system running.
//
    res = regmap_write(ec.regmap, NTXEC_REG_POWERKEEP,
    NTXEC_POWERKEEP_VALUE);
    if (res < 0)
    return res;
    if (poweroff_restart_client)
//
// Another instance of the driver already took
// poweroff/restart duties.
//
    dev_err(ec.dev, "poweroff_restart_client already assigned\n");
    else
    poweroff_restart_client = client;
    if (pm_power_off)
// Another driver already registered a poweroff handler.
    dev_err(ec.dev, "pm_power_off already assigned\n");
    else
    pm_power_off = ntxec_poweroff;
    res = register_restart_handler(&ntxec_restart_handler);
    if (res)
    dev_err(ec.dev,
    "Failed to register restart handler: %d\n", res);
    }
    i2c_set_clientdata(client, ec);
    res = devm_mfd_add_devices(ec.dev, PLATFORM_DEVID_NONE,
    subdevs, n_subdevs, core::ptr::null_mut(), 0, core::ptr::null_mut());
    if (res)
    dev_err(ec.dev, "Failed to add subdevices: %d\n", res);
    return res;
    }
#[no_mangle]
unsafe extern "C" fn ntxec_remove(client: *mut i2c_client) {
    static void ntxec_remove(struct i2c_client *client)
    {
    if (client == poweroff_restart_client) {
    poweroff_restart_client = core::ptr::null_mut();
    pm_power_off = core::ptr::null_mut();
    unregister_restart_handler(&ntxec_restart_handler);
    }
    }
    static const struct of_device_id of_ntxec_match_table[] = {
    { .compatible = "netronix,ntxec", },
    {}
    };
    MODULE_DEVICE_TABLE(of, of_ntxec_match_table);
    static struct i2c_driver ntxec_driver = {
    .driver = {
    .name = "ntxec",
    .of_match_table = of_ntxec_match_table,
    },
    .probe = ntxec_probe,
    .remove = ntxec_remove,
    };
    module_i2c_driver(ntxec_driver);
    MODULE_AUTHOR("Jonathan Neuschäfer <j.neuschaefer@gmx.net>");
    MODULE_DESCRIPTION("Core driver for Netronix EC");
    MODULE_LICENSE("GPL");
