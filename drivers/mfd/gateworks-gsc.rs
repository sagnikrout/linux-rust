//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/gateworks-gsc.c
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
// The Gateworks System Controller (GSC) is a multi-function
// device designed for use in Gateworks Single Board Computers.
// The control interface is I2C, with an interrupt. The device supports
// system functions such as push-button monitoring, multiple ADC's for
// voltage and temperature monitoring, fan controller and watchdog monitor.
//
// Copyright (C) 2020 Gateworks Corporation
//

//
// The GSC suffers from an errata where occasionally during
// ADC cycles the chip can NAK I2C transactions. To ensure we have reliable
// register access we place retries around register access.
//
pub const I2C_RETRIES: c_int = 3;
#[no_mangle]
pub unsafe extern "C" fn gsc_write(context: *mut c_void, reg: c_uint, val: c_uint) -> c_int {
    int gsc_write(void *context, unsigned int reg, unsigned int val)
    {
    struct i2c_client *client = context;
    int retry, ret;
    for (retry = 0; retry < I2C_RETRIES; retry++) {
    ret = i2c_smbus_write_byte_data(client, reg, val);
//
// -EAGAIN returned when the i2c host controller is busy
// -EIO returned when i2c device is busy
//
    if (ret != -EAGAIN && ret != -EIO)
    break;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(gsc_write);
#[no_mangle]
pub unsafe extern "C" fn gsc_read(context: *mut c_void, reg: c_uint, val: *mut c_uint) -> c_int {
    int gsc_read(void *context, unsigned int reg, unsigned int *val)
    {
    struct i2c_client *client = context;
    int retry, ret;
    for (retry = 0; retry < I2C_RETRIES; retry++) {
    ret = i2c_smbus_read_byte_data(client, reg);
//
// -EAGAIN returned when the i2c host controller is busy
// -EIO returned when i2c device is busy
//
    if (ret != -EAGAIN && ret != -EIO)
    break;
    }
// val = ret & 0xff;
    return 0;
    }
    EXPORT_SYMBOL_GPL(gsc_read);
//
// gsc_powerdown - API to use GSC to power down board for a specific time
//
// secs - number of seconds to remain powered off
//
#[no_mangle]
unsafe extern "C" fn gsc_powerdown(gsc: *mut gsc_dev, secs: c_ulong) -> c_int {
    static int gsc_powerdown(struct gsc_dev *gsc, unsigned long secs)
    {
    int ret;
    unsigned char regs[4];
    dev_info(&gsc.i2c.dev, "GSC powerdown for %ld seconds\n",
    secs);
    put_unaligned_le32(secs, regs);
    ret = regmap_bulk_write(gsc.regmap, GSC_TIME_ADD, regs, 4);
    if (ret)
    return ret;
    ret = regmap_update_bits(gsc.regmap, GSC_CTRL_1,
    BIT(GSC_CTRL_1_SLEEP_ADD),
    BIT(GSC_CTRL_1_SLEEP_ADD));
    if (ret)
    return ret;
    ret = regmap_update_bits(gsc.regmap, GSC_CTRL_1,
    BIT(GSC_CTRL_1_SLEEP_ACTIVATE) |
    BIT(GSC_CTRL_1_SLEEP_ENABLE),
    BIT(GSC_CTRL_1_SLEEP_ACTIVATE) |
    BIT(GSC_CTRL_1_SLEEP_ENABLE));
    return ret;
    }
    static ssize_t gsc_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct gsc_dev *gsc = dev_get_drvdata(dev);
    const char *name = attr.attr.name;
    let mut rz: c_int = 0;
    if (strcasecmp(name, "fw_version") == 0)
    rz = sprintf(buf, "%d\n", gsc.fwver);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcasecmp(name, 0: "fw_crc") ==) -> else {
    else if (strcasecmp(name, "fw_crc") == 0)
    rz = sprintf(buf, "0x%04x\n", gsc.fwcrc);
    else
    dev_err(dev, "invalid command: '%s'\n", name);
    return rz;
    }
    static ssize_t gsc_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct gsc_dev *gsc = dev_get_drvdata(dev);
    const char *name = attr.attr.name;
    long value;
    if (strcasecmp(name, "powerdown") == 0) {
    if (kstrtol(buf, 0, &value) == 0)
    gsc_powerdown(gsc, value);
    } else {
    dev_err(dev, "invalid command: '%s\n", name);
    }
    return count;
    }
    static struct device_attribute attr_fwver =
    __ATTR(fw_version, 0440, gsc_show, core::ptr::null_mut());
    static struct device_attribute attr_fwcrc =
    __ATTR(fw_crc, 0440, gsc_show, core::ptr::null_mut());
    static struct device_attribute attr_pwrdown =
    __ATTR(powerdown, 0220, core::ptr::null_mut(), gsc_store);
    static struct attribute *gsc_attrs[] = {
    &attr_fwver.attr,
    &attr_fwcrc.attr,
    &attr_pwrdown.attr,
    core::ptr::null_mut(),
    };
    static struct attribute_group attr_group = {
    .attrs = gsc_attrs,
    };
    static const struct of_device_id gsc_of_match[] = {
    { .compatible = "gw,gsc", },
    { }
    };
    MODULE_DEVICE_TABLE(of, gsc_of_match);
    static const struct regmap_bus gsc_regmap_bus = {
    .reg_read = gsc_read,
    .reg_write = gsc_write,
    };
    static const struct regmap_config gsc_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .cache_type = REGCACHE_NONE,
    .max_register = GSC_WP,
    };
    static const struct regmap_irq gsc_irqs[] = {
    REGMAP_IRQ_REG(GSC_IRQ_PB, 0, BIT(GSC_IRQ_PB)),
    REGMAP_IRQ_REG(GSC_IRQ_KEY_ERASED, 0, BIT(GSC_IRQ_KEY_ERASED)),
    REGMAP_IRQ_REG(GSC_IRQ_EEPROM_WP, 0, BIT(GSC_IRQ_EEPROM_WP)),
    REGMAP_IRQ_REG(GSC_IRQ_RESV, 0, BIT(GSC_IRQ_RESV)),
    REGMAP_IRQ_REG(GSC_IRQ_GPIO, 0, BIT(GSC_IRQ_GPIO)),
    REGMAP_IRQ_REG(GSC_IRQ_TAMPER, 0, BIT(GSC_IRQ_TAMPER)),
    REGMAP_IRQ_REG(GSC_IRQ_WDT_TIMEOUT, 0, BIT(GSC_IRQ_WDT_TIMEOUT)),
    REGMAP_IRQ_REG(GSC_IRQ_SWITCH_HOLD, 0, BIT(GSC_IRQ_SWITCH_HOLD)),
    };
    static const struct regmap_irq_chip gsc_irq_chip = {
    .name = "gateworks-gsc",
    .irqs = gsc_irqs,
    .num_irqs = ARRAY_SIZE(gsc_irqs),
    .num_regs = 1,
    .status_base = GSC_IRQ_STATUS,
    .unmask_base = GSC_IRQ_ENABLE,
    .ack_base = GSC_IRQ_STATUS,
    .ack_invert = true,
    };
#[no_mangle]
unsafe extern "C" fn gsc_probe(client: *mut i2c_client) -> c_int {
    static int gsc_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct gsc_dev *gsc;
    struct regmap_irq_chip_data *irq_data;
    int ret;
    unsigned int reg;
    gsc = devm_kzalloc(dev, sizeof(*gsc), GFP_KERNEL);
    if (!gsc)
    return -ENOMEM;
    gsc.dev = &client.dev;
    gsc.i2c = client;
    i2c_set_clientdata(client, gsc);
    gsc.regmap = devm_regmap_init(dev, &gsc_regmap_bus, client,
    &gsc_regmap_config);
    if (IS_ERR(gsc.regmap))
    return PTR_ERR(gsc.regmap);
    if (regmap_read(gsc.regmap, GSC_FW_VER, &reg))
    return -EIO;
    gsc.fwver = reg;
    regmap_read(gsc.regmap, GSC_FW_CRC, &reg);
    gsc.fwcrc = reg;
    regmap_read(gsc.regmap, GSC_FW_CRC + 1, &reg);
    gsc.fwcrc |= reg << 8;
    gsc.i2c_hwmon = devm_i2c_new_dummy_device(dev, client.adapter,
    GSC_HWMON);
    if (IS_ERR(gsc.i2c_hwmon)) {
    dev_err(dev, "Failed to allocate I2C device for HWMON\n");
    return PTR_ERR(gsc.i2c_hwmon);
    }
    ret = devm_regmap_add_irq_chip(dev, gsc.regmap, client.irq,
    IRQF_ONESHOT | IRQF_SHARED |
    IRQF_TRIGGER_LOW, 0,
    &gsc_irq_chip, &irq_data);
    if (ret)
    return ret;
    dev_info(dev, "Gateworks System Controller v%d: fw 0x%04x\n",
    gsc.fwver, gsc.fwcrc);
    ret = sysfs_create_group(&dev.kobj, &attr_group);
    if (ret)
    dev_err(dev, "failed to create sysfs attrs\n");
    ret = devm_of_platform_populate(dev);
    if (ret) {
    sysfs_remove_group(&dev.kobj, &attr_group);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gsc_remove(client: *mut i2c_client) {
    static void gsc_remove(struct i2c_client *client)
    {
    sysfs_remove_group(&client.dev.kobj, &attr_group);
    }
    static struct i2c_driver gsc_driver = {
    .driver = {
    .name	= "gateworks-gsc",
    .of_match_table = gsc_of_match,
    },
    .probe		= gsc_probe,
    .remove		= gsc_remove,
    };
    module_i2c_driver(gsc_driver);
    MODULE_AUTHOR("Tim Harvey <tharvey@gateworks.com>");
    MODULE_DESCRIPTION("I2C Core interface for GSC");
    MODULE_LICENSE("GPL v2");
