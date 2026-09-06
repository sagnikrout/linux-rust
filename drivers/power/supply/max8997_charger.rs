//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/max8997_charger.c
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
// max8997_charger.c - Power supply consumer driver for the Maxim 8997/8966
//
// Copyright (C) 2011 Samsung Electronics
// MyungJoo Ham <myungjoo.ham@samsung.com>

// MAX8997_REG_STATUS4
pub const DCINOK_SHIFT: c_int = 1;

pub const DETBAT_SHIFT: c_int = 2;

// MAX8997_REG_MBCCTRL1
pub const TFCH_SHIFT: c_int = 4;

// MAX8997_REG_MBCCTRL5
pub const ITOPOFF_SHIFT: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct charger_data {
    pub dev: *mut device,
    pub iodev: *mut max8997_dev,
    pub battery: *mut power_supply,
    pub reg: *mut regulator,
    pub edev: *mut extcon_dev,
    pub extcon_nb: notifier_block,
    pub extcon_work: work_struct,
}

    static enum power_supply_property max8997_battery_props[] = {
    POWER_SUPPLY_PROP_STATUS, /* "FULL", "CHARGING" or "DISCHARGING". */
    POWER_SUPPLY_PROP_PRESENT, /* the presence of battery */
    POWER_SUPPLY_PROP_ONLINE, /* charger is active or not */
    };
// Note that the charger control is done by a current regulator "CHARGER"
    static int max8997_battery_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct charger_data *charger = power_supply_get_drvdata(psy);
    struct i2c_client *i2c = charger.iodev.i2c;
    int ret;
    u8 reg;
    switch (psp) {
    case POWER_SUPPLY_PROP_STATUS:
    val.intval = 0;
    ret = max8997_read_reg(i2c, MAX8997_REG_STATUS4, &reg);
    if (ret)
    return ret;
    if ((reg & (1 << 0)) == 0x1)
    val.intval = POWER_SUPPLY_STATUS_FULL;
#[no_mangle]
pub unsafe extern "C" fn if(DCINOK_MASK): (reg &) -> else {
    else if ((reg & DCINOK_MASK))
    val.intval = POWER_SUPPLY_STATUS_CHARGING;
    else
    val.intval = POWER_SUPPLY_STATUS_DISCHARGING;
    break;
    case POWER_SUPPLY_PROP_PRESENT:
    val.intval = 0;
    ret = max8997_read_reg(i2c, MAX8997_REG_STATUS4, &reg);
    if (ret)
    return ret;
    if ((reg & DETBAT_MASK) == 0x0)
    val.intval = 1;
    break;
    case POWER_SUPPLY_PROP_ONLINE:
    val.intval = 0;
    ret = max8997_read_reg(i2c, MAX8997_REG_STATUS4, &reg);
    if (ret)
    return ret;
    if (reg & DCINOK_MASK)
    val.intval = 1;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max8997_battery_extcon_evt_worker(work: *mut work_struct) {
    static void max8997_battery_extcon_evt_worker(struct work_struct *work)
    {
    struct charger_data *charger =
    container_of(work, struct charger_data, extcon_work);
    struct extcon_dev *edev = charger.edev;
    int current_limit;
    if (extcon_get_state(edev, EXTCON_CHG_USB_SDP) > 0) {
    dev_dbg(charger.dev, "USB SDP charger is connected\n");
    current_limit = 450000;
    } else if (extcon_get_state(edev, EXTCON_CHG_USB_DCP) > 0) {
    dev_dbg(charger.dev, "USB DCP charger is connected\n");
    current_limit = 650000;
    } else if (extcon_get_state(edev, EXTCON_CHG_USB_FAST) > 0) {
    dev_dbg(charger.dev, "USB FAST charger is connected\n");
    current_limit = 650000;
    } else if (extcon_get_state(edev, EXTCON_CHG_USB_SLOW) > 0) {
    dev_dbg(charger.dev, "USB SLOW charger is connected\n");
    current_limit = 650000;
    } else if (extcon_get_state(edev, EXTCON_CHG_USB_CDP) > 0) {
    dev_dbg(charger.dev, "USB CDP charger is connected\n");
    current_limit = 650000;
    } else {
    dev_dbg(charger.dev, "USB charger is disconnected\n");
    current_limit = -1;
    }
    if (current_limit > 0) {
    let mut ret: c_int = regulator_set_current_limit(charger.reg, current_limit, current_limit);
    if (ret) {
    dev_err(charger.dev, "failed to set current limit: %d\n", ret);
    return;
    }
    ret = regulator_enable(charger.reg);
    if (ret)
    dev_err(charger.dev, "failed to enable regulator: %d\n", ret);
    } else {
    let mut ret: c_int = regulator_disable(charger.reg);
    if (ret)
    dev_err(charger.dev, "failed to disable regulator: %d\n", ret);
    }
    }
    static int max8997_battery_extcon_evt(struct notifier_block *nb,
    unsigned long event, void *param)
    {
    struct charger_data *charger =
    container_of(nb, struct charger_data, extcon_nb);
    schedule_work(&charger.extcon_work);
    return NOTIFY_OK;
    }
    static const struct power_supply_desc max8997_battery_desc = {
    .name		= "max8997_pmic",
    .type		= POWER_SUPPLY_TYPE_BATTERY,
    .get_property	= max8997_battery_get_property,
    .properties	= max8997_battery_props,
    .num_properties	= ARRAY_SIZE(max8997_battery_props),
    };
#[no_mangle]
unsafe extern "C" fn max8997_battery_probe(pdev: *mut platform_device) -> c_int {
    static int max8997_battery_probe(struct platform_device *pdev)
    {
    let mut ret: c_int = 0;
    struct charger_data *charger;
    struct max8997_dev *iodev = dev_get_drvdata(pdev.dev.parent);
    struct device_node *np = pdev.dev.of_node;
    struct i2c_client *i2c = iodev.i2c;
    struct max8997_platform_data *pdata = iodev.pdata;
    let mut psy_cfg: power_supply_config = {};
    if (!pdata) {
    dev_err(&pdev.dev, "No platform data supplied.\n");
    return -EINVAL;
    }
    if (pdata.eoc_mA) {
    let mut val: c_int = (pdata.eoc_mA - 50) / 10;
    if (val < 0)
    val = 0;
    if (val > 0xf)
    val = 0xf;
    ret = max8997_update_reg(i2c, MAX8997_REG_MBCCTRL5,
    val << ITOPOFF_SHIFT, ITOPOFF_MASK);
    if (ret < 0) {
    dev_err(&pdev.dev, "Cannot use i2c bus.\n");
    return ret;
    }
    }
    switch (pdata.timeout) {
    case 5:
    ret = max8997_update_reg(i2c, MAX8997_REG_MBCCTRL1,
    0x2 << TFCH_SHIFT, TFCH_MASK);
    break;
    case 6:
    ret = max8997_update_reg(i2c, MAX8997_REG_MBCCTRL1,
    0x3 << TFCH_SHIFT, TFCH_MASK);
    break;
    case 7:
    ret = max8997_update_reg(i2c, MAX8997_REG_MBCCTRL1,
    0x4 << TFCH_SHIFT, TFCH_MASK);
    break;
    case 0:
    ret = max8997_update_reg(i2c, MAX8997_REG_MBCCTRL1,
    0x7 << TFCH_SHIFT, TFCH_MASK);
    break;
    default:
    dev_err(&pdev.dev, "incorrect timeout value (%d)\n",
    pdata.timeout);
    return -EINVAL;
    }
    if (ret < 0) {
    dev_err(&pdev.dev, "Cannot use i2c bus.\n");
    return ret;
    }
    charger = devm_kzalloc(&pdev.dev, sizeof(*charger), GFP_KERNEL);
    if (!charger)
    return -ENOMEM;
    platform_set_drvdata(pdev, charger);
    charger.dev = &pdev.dev;
    charger.iodev = iodev;
    psy_cfg.drv_data = charger;
    charger.battery = devm_power_supply_register(&pdev.dev,
    &max8997_battery_desc,
    &psy_cfg);
    if (IS_ERR(charger.battery)) {
    dev_err(&pdev.dev, "failed: power supply register\n");
    return PTR_ERR(charger.battery);
    }
// grab regulator from parent device's node
    pdev.dev.of_node = iodev.dev.of_node;
    charger.reg = devm_regulator_get_optional(&pdev.dev, "charger");
    pdev.dev.of_node = np;
    if (IS_ERR(charger.reg)) {
    if (PTR_ERR(charger.reg) == -EPROBE_DEFER)
    return -EPROBE_DEFER;
    dev_info(&pdev.dev, "couldn't get charger regulator\n");
    }
    charger.edev = extcon_get_extcon_dev("max8997-muic");
    if (IS_ERR(charger.edev)) {
    dev_err_probe(charger.dev, PTR_ERR(charger.edev),
    "couldn't get extcon device: max8997-muic\n");
    return PTR_ERR(charger.edev);
    }
    if (!IS_ERR(charger.reg) && !IS_ERR_OR_NULL(charger.edev)) {
    ret = devm_work_autocancel(&pdev.dev, &charger.extcon_work,
    max8997_battery_extcon_evt_worker);
    if (ret) {
    dev_err(&pdev.dev, "failed to add extcon evt stop action: %d\n", ret);
    return ret;
    }
    charger.extcon_nb.notifier_call = max8997_battery_extcon_evt;
    ret = devm_extcon_register_notifier_all(&pdev.dev, charger.edev,
    &charger.extcon_nb);
    if (ret) {
    dev_err(&pdev.dev, "failed to register extcon notifier\n");
    return ret;
    }
    }
    return 0;
    }
    static const struct platform_device_id max8997_battery_id[] = {
    { .name = "max8997-battery" },
    { }
    };
    MODULE_DEVICE_TABLE(platform, max8997_battery_id);
    static struct platform_driver max8997_battery_driver = {
    .driver = {
    .name = "max8997-battery",
    },
    .probe = max8997_battery_probe,
    .id_table = max8997_battery_id,
    };
    module_platform_driver(max8997_battery_driver);
    MODULE_DESCRIPTION("MAXIM 8997/8966 battery control driver");
    MODULE_AUTHOR("MyungJoo Ham <myungjoo.ham@samsung.com>");
    MODULE_LICENSE("GPL");
