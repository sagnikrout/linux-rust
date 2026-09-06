//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/sbs-charger.c
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
// Copyright (c) 2016, Prodys S.L.
//
// This adds support for sbs-charger compilant chips as defined here:
// http://sbs-forum.org/specs/sbc110.pdf
//
// Implemetation based on sbs-battery.c
//

pub const SBS_CHARGER_REG_SPEC_INFO: c_uint = 0x11;
pub const SBS_CHARGER_REG_STATUS: c_uint = 0x13;
pub const SBS_CHARGER_REG_ALARM_WARNING: c_uint = 0x16;

pub const SBS_CHARGER_POLL_TIME: c_int = 500;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbs_info {
    pub client: *mut i2c_client,
    pub power_supply: *mut power_supply,
    pub regmap: *mut regmap,
    pub work: delayed_work,
    pub last_state: c_uint,
}

    static int sbs_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct sbs_info *chip = power_supply_get_drvdata(psy);
    unsigned int reg;
    reg = chip.last_state;
    switch (psp) {
    case POWER_SUPPLY_PROP_PRESENT:
    val.intval = !!(reg & SBS_CHARGER_STATUS_BATTERY_PRESENT);
    break;
    case POWER_SUPPLY_PROP_ONLINE:
    val.intval = !!(reg & SBS_CHARGER_STATUS_AC_PRESENT);
    break;
    case POWER_SUPPLY_PROP_STATUS:
    val.intval = POWER_SUPPLY_STATUS_UNKNOWN;
    if (!(reg & SBS_CHARGER_STATUS_BATTERY_PRESENT))
    val.intval = POWER_SUPPLY_STATUS_NOT_CHARGING;
    else if (reg & SBS_CHARGER_STATUS_AC_PRESENT &&
    !(reg & SBS_CHARGER_STATUS_CHARGE_INHIBITED))
    val.intval = POWER_SUPPLY_STATUS_CHARGING;
    else
    val.intval = POWER_SUPPLY_STATUS_DISCHARGING;
    break;
    case POWER_SUPPLY_PROP_HEALTH:
    if (reg & SBS_CHARGER_STATUS_RES_COLD)
    val.intval = POWER_SUPPLY_HEALTH_COLD;
    if (reg & SBS_CHARGER_STATUS_RES_HOT)
    val.intval = POWER_SUPPLY_HEALTH_OVERHEAT;
    else
    val.intval = POWER_SUPPLY_HEALTH_GOOD;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sbs_check_state(chip: *mut sbs_info) -> c_int {
    static int sbs_check_state(struct sbs_info *chip)
    {
    unsigned int reg;
    int ret;
    ret = regmap_read(chip.regmap, SBS_CHARGER_REG_STATUS, &reg);
    if (!ret && reg != chip.last_state) {
    chip.last_state = reg;
    power_supply_changed(chip.power_supply);
    return 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sbs_delayed_work(work: *mut work_struct) {
    static void sbs_delayed_work(struct work_struct *work)
    {
    struct sbs_info *chip = container_of(work, struct sbs_info, work.work);
    sbs_check_state(chip);
    schedule_delayed_work(&chip.work,
    msecs_to_jiffies(SBS_CHARGER_POLL_TIME));
    }
#[no_mangle]
unsafe extern "C" fn sbs_irq_thread(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t sbs_irq_thread(int irq, void *data)
    {
    struct sbs_info *chip = data;
    int ret;
    ret = sbs_check_state(chip);
    return ret ? IRQ_HANDLED : IRQ_NONE;
    }
    static enum power_supply_property sbs_properties[] = {
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_PRESENT,
    POWER_SUPPLY_PROP_ONLINE,
    POWER_SUPPLY_PROP_HEALTH,
    };
#[no_mangle]
unsafe extern "C" fn sbs_readable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool sbs_readable_reg(struct device *dev, unsigned int reg)
    {
    return reg >= SBS_CHARGER_REG_SPEC_INFO;
    }
#[no_mangle]
unsafe extern "C" fn sbs_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool sbs_volatile_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case SBS_CHARGER_REG_STATUS:
    return true;
    }
    return false;
    }
    static const struct regmap_config sbs_regmap = {
    .reg_bits	= 8,
    .val_bits	= 16,
    .max_register	= SBS_CHARGER_REG_ALARM_WARNING,
    .readable_reg	= sbs_readable_reg,
    .volatile_reg	= sbs_volatile_reg,
    .val_format_endian = REGMAP_ENDIAN_LITTLE, /* since based on SMBus */
    };
    static const struct power_supply_desc sbs_default_desc = {
    .type = POWER_SUPPLY_TYPE_MAINS,
    .properties = sbs_properties,
    .num_properties = ARRAY_SIZE(sbs_properties),
    .get_property = sbs_get_property,
    };
#[no_mangle]
unsafe extern "C" fn sbs_probe(client: *mut i2c_client) -> c_int {
    static int sbs_probe(struct i2c_client *client)
    {
    let mut psy_cfg: power_supply_config = {};
    struct power_supply_desc *sbs_desc;
    struct sbs_info *chip;
    int ret, val;
    sbs_desc = devm_kmemdup(&client.dev, &sbs_default_desc,
    sizeof(*sbs_desc), GFP_KERNEL);
    if (!sbs_desc)
    return -ENOMEM;
    sbs_desc.name = devm_kasprintf(&client.dev, GFP_KERNEL, "sbs-%s",
    dev_name(&client.dev));
    if (!sbs_desc.name)
    return -ENOMEM;
    chip = devm_kzalloc(&client.dev, sizeof(struct sbs_info), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    chip.client = client;
    psy_cfg.fwnode = dev_fwnode(&client.dev);
    psy_cfg.drv_data = chip;
    i2c_set_clientdata(client, chip);
    chip.regmap = devm_regmap_init_i2c(client, &sbs_regmap);
    if (IS_ERR(chip.regmap))
    return PTR_ERR(chip.regmap);
//
// Before we register, we need to make sure we can actually talk
// to the battery.
//
    ret = regmap_read(chip.regmap, SBS_CHARGER_REG_STATUS, &val);
    if (ret)
    return dev_err_probe(&client.dev, ret, "Failed to get device status\n");
    chip.last_state = val;
    chip.power_supply = devm_power_supply_register(&client.dev, sbs_desc, &psy_cfg);
    if (IS_ERR(chip.power_supply))
    return dev_err_probe(&client.dev, PTR_ERR(chip.power_supply),
    "Failed to register power supply\n");
//
// The sbs-charger spec doesn't impose the use of an interrupt. So in
// the case it wasn't provided we use polling in order get the charger's
// status.
//
    if (client.irq) {
    ret = devm_request_threaded_irq(&client.dev, client.irq,
    core::ptr::null_mut(), sbs_irq_thread,
    IRQF_TRIGGER_FALLING | IRQF_ONESHOT,
    dev_name(&client.dev), chip);
    if (ret)
    return ret;
    } else {
    ret = devm_delayed_work_autocancel(&client.dev, &chip.work,
    sbs_delayed_work);
    if (ret)
    return dev_err_probe(&client.dev, ret,
    "Failed to init work for polling\n");
    schedule_delayed_work(&chip.work,
    msecs_to_jiffies(SBS_CHARGER_POLL_TIME));
    }
    dev_info(&client.dev,
    "%s: smart charger device registered\n", client.name);
    return 0;
    }

    static const struct of_device_id sbs_dt_ids[] = {
    { .compatible = "sbs,sbs-charger" },
    { },
    };
    MODULE_DEVICE_TABLE(of, sbs_dt_ids);

    static const struct i2c_device_id sbs_id[] = {
    { .name = "sbs-charger" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, sbs_id);
    static struct i2c_driver sbs_driver = {
    .probe		= sbs_probe,
    .id_table	= sbs_id,
    .driver = {
    .name	= "sbs-charger",
    .of_match_table = of_match_ptr(sbs_dt_ids),
    },
    };
    module_i2c_driver(sbs_driver);
    MODULE_AUTHOR("Nicolas Saenz Julienne <nicolassaenzj@gmail.com>");
    MODULE_DESCRIPTION("SBS smart charger driver");
    MODULE_LICENSE("GPL v2");
