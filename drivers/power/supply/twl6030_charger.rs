//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/twl6030_charger.c
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
// TWL6030 charger
//
// Copyright (C) 2024 Andreas Kemnade <andreas@kemnade.info>
//
// based on older 6030 driver found in a v3.0 vendor kernel
//
// based on twl4030_bci_battery.c by TI
// Copyright (C) 2008 Texas Instruments, Inc.
//

pub const CONTROLLER_INT_MASK: c_uint = 0x00;
pub const CONTROLLER_CTRL1: c_uint = 0x01;
pub const CONTROLLER_WDG: c_uint = 0x02;
pub const CONTROLLER_STAT1: c_uint = 0x03;
pub const CHARGERUSB_INT_STATUS: c_uint = 0x04;
pub const CHARGERUSB_INT_MASK: c_uint = 0x05;
pub const CHARGERUSB_STATUS_INT1: c_uint = 0x06;
pub const CHARGERUSB_STATUS_INT2: c_uint = 0x07;
pub const CHARGERUSB_CTRL1: c_uint = 0x08;
pub const CHARGERUSB_CTRL2: c_uint = 0x09;
pub const CHARGERUSB_CTRL3: c_uint = 0x0A;
pub const CHARGERUSB_STAT1: c_uint = 0x0B;
pub const CHARGERUSB_VOREG: c_uint = 0x0C;
pub const CHARGERUSB_VICHRG: c_uint = 0x0D;
pub const CHARGERUSB_CINLIMIT: c_uint = 0x0E;
pub const CHARGERUSB_CTRLLIMIT1: c_uint = 0x0F;
pub const CHARGERUSB_CTRLLIMIT2: c_uint = 0x10;
pub const ANTICOLLAPSE_CTRL1: c_uint = 0x11;
pub const ANTICOLLAPSE_CTRL2: c_uint = 0x12;
// TWL6032 registers 0xDA to 0xDE - TWL6032_MODULE_CHARGER
pub const CONTROLLER_CTRL2: c_uint = 0x00;
pub const CONTROLLER_VSEL_COMP: c_uint = 0x01;
pub const CHARGERUSB_VSYSREG: c_uint = 0x02;
pub const CHARGERUSB_VICHRG_PC: c_uint = 0x03;
pub const LINEAR_CHRG_STS: c_uint = 0x04;
pub const LINEAR_CHRG_STS_CRYSTL_OSC_OK: c_uint = 0x40;
pub const LINEAR_CHRG_STS_END_OF_CHARGE: c_uint = 0x20;
pub const LINEAR_CHRG_STS_VBATOV: c_uint = 0x10;
pub const LINEAR_CHRG_STS_VSYSOV: c_uint = 0x08;
pub const LINEAR_CHRG_STS_DPPM_STS: c_uint = 0x04;
pub const LINEAR_CHRG_STS_CV_STS: c_uint = 0x02;
pub const LINEAR_CHRG_STS_CC_STS: c_uint = 0x01;
pub const FG_REG_00: c_uint = 0x00;
pub const FG_REG_01: c_uint = 0x01;
pub const FG_REG_02: c_uint = 0x02;
pub const FG_REG_03: c_uint = 0x03;
pub const FG_REG_04: c_uint = 0x04;
pub const FG_REG_05: c_uint = 0x05;
pub const FG_REG_06: c_uint = 0x06;
pub const FG_REG_07: c_uint = 0x07;
pub const FG_REG_08: c_uint = 0x08;
pub const FG_REG_09: c_uint = 0x09;
pub const FG_REG_10: c_uint = 0x0A;
pub const FG_REG_11: c_uint = 0x0B;
// CONTROLLER_INT_MASK

// CONTROLLER_CTRL1

// CONTROLLER_STAT1

// CHARGERUSB_INT_STATUS

// CHARGERUSB_INT_MASK

// CHARGERUSB_STATUS_INT1

// CHARGERUSB_STATUS_INT2

// CHARGERUSB_CTRL1

// CHARGERUSB_CTRL2

// CHARGERUSB_CTRL3

// CHARGERUSB_VOREG

pub const CHARGERUSB_VOREG_3P52: c_uint = 0x01;
pub const CHARGERUSB_VOREG_4P0: c_uint = 0x19;
pub const CHARGERUSB_VOREG_4P2: c_uint = 0x23;
pub const CHARGERUSB_VOREG_4P76: c_uint = 0x3F;
// CHARGERUSB_VICHRG
//
// might be inaccurate for < 500 mA, diffent scale might apply,
// either starting from 100 mA or 300 mA
//

// CHARGERUSB_CINLIMIT
pub const CHARGERUSB_CIN_LIMIT_100: c_uint = 0x1;
pub const CHARGERUSB_CIN_LIMIT_300: c_uint = 0x5;
pub const CHARGERUSB_CIN_LIMIT_500: c_uint = 0x9;
pub const CHARGERUSB_CIN_LIMIT_NONE: c_uint = 0xF;
// CHARGERUSB_CTRLLIMIT2
pub const CHARGERUSB_CTRLLIMIT2_1500: c_uint = 0x0E;

// ANTICOLLAPSE_CTRL2
pub const BUCK_VTH_SHIFT: c_int = 5;
// FG_REG_00
pub const CC_ACTIVE_MODE_SHIFT: c_int = 6;

pub const REG_TOGGLE1: c_uint = 0x90;
pub const REG_PWDNSTATUS1: c_uint = 0x93;

pub const BBSPOR_CFG: c_uint = 0xE6;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct twl6030_charger_info {
    pub dev: *mut device,
    pub usb: *mut power_supply,
    pub binfo: *mut power_supply_battery_info,
    pub work: work_struct,
    pub irq_chg: c_int,
    pub input_current_limit: c_int,
    pub channel_vusb: *mut iio_channel,
    pub charger_monitor: delayed_work,
    pub extended_current_range: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct twl6030_charger_chip_data {
    pub extended_current_range: bool,
}

#[no_mangle]
unsafe extern "C" fn twl6030_charger_read(reg: u8, val: *mut u8) -> c_int {
    static int twl6030_charger_read(u8 reg, u8 *val)
    {
    return twl_i2c_read_u8(TWL_MODULE_MAIN_CHARGE, val, reg);
    }
#[no_mangle]
unsafe extern "C" fn twl6030_charger_write(reg: u8, val: u8) -> c_int {
    static int twl6030_charger_write(u8 reg, u8 val)
    {
    return twl_i2c_write_u8(TWL_MODULE_MAIN_CHARGE, val, reg);
    }
    static int twl6030_config_cinlimit_reg(struct twl6030_charger_info *charger,
    unsigned int ua)
    {
    if (ua >= 50000 && ua <= 750000) {
    ua = (ua - 50000) / 50000;
    } else if ((ua > 750000) && (ua <= 1500000) && charger.extended_current_range) {
    ua = ((ua % 100000) ? 0x30 : 0x20) + ((ua - 100000) / 100000);
    } else {
    if (ua < 50000) {
    dev_err(charger.dev, "invalid input current limit\n");
    return -EINVAL;
    }
// This is no current limit
    ua = 0x0F;
    }
    return twl6030_charger_write(CHARGERUSB_CINLIMIT, ua);
    }
//
// rewriting all stuff here, resets to extremely conservative defaults were
// seen under some circumstances, like charge voltage to 3.5V
//
#[no_mangle]
unsafe extern "C" fn twl6030_enable_usb(charger: *mut twl6030_charger_info) -> c_int {
    static int twl6030_enable_usb(struct twl6030_charger_info *charger)
    {
    int ret;
    ret = twl6030_charger_write(CHARGERUSB_VICHRG,
    UA_TO_VICHRG(charger.binfo.constant_charge_current_max_ua));
    if (ret < 0)
    return ret;
    ret = twl6030_charger_write(CONTROLLER_WDG, 0xff);
    if (ret < 0)
    return ret;
    charger.input_current_limit = 500000;
    ret = twl6030_config_cinlimit_reg(charger, charger.input_current_limit);
    if (ret < 0)
    return ret;
    ret = twl6030_charger_write(CHARGERUSB_CINLIMIT, CHARGERUSB_CIN_LIMIT_500);
    if (ret < 0)
    return ret;
    ret = twl6030_charger_write(CHARGERUSB_VOREG,
    UV_TO_VOREG(charger.binfo.constant_charge_voltage_max_uv));
    if (ret < 0)
    return ret;
    ret = twl6030_charger_write(CHARGERUSB_CTRL1, TERM);
    if (ret < 0)
    return ret;
    if (charger.binfo.charge_term_current_ua != -EINVAL) {
    ret = twl6030_charger_write(CHARGERUSB_CTRL2,
    UA_TO_VITERM(charger.binfo.charge_term_current_ua));
    if (ret < 0)
    return ret;
    }
    return twl6030_charger_write(CONTROLLER_CTRL1, CONTROLLER_CTRL1_EN_CHARGER);
    }
#[no_mangle]
unsafe extern "C" fn twl6030_charger_wdg(data: *mut work_struct) {
    static void twl6030_charger_wdg(struct work_struct *data)
    {
    struct twl6030_charger_info *charger =
    container_of(data, struct twl6030_charger_info,
    charger_monitor.work);
    u8 val;
    u8 int_stat;
    u8 stat_int1;
    u8 stat_int2;
    twl6030_charger_read(CONTROLLER_STAT1, &val);
    twl6030_charger_read(CHARGERUSB_INT_STATUS, &int_stat);
    twl6030_charger_read(CHARGERUSB_STATUS_INT1, &stat_int1);
    twl6030_charger_read(CHARGERUSB_STATUS_INT2, &stat_int2);
    dev_dbg(charger.dev,
    "wdg: stat1: %02x %s INT_STATUS %02x STATUS_INT1 %02x STATUS_INT2 %02x\n",
    val, (val & VBUS_DET) ? "usb online" :  "usb offline",
    int_stat, stat_int1, stat_int2);
    twl6030_charger_write(CONTROLLER_WDG, 0xff);
    schedule_delayed_work(&charger.charger_monitor,
    msecs_to_jiffies(10000));
    }
#[no_mangle]
unsafe extern "C" fn twl6030_charger_interrupt(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t twl6030_charger_interrupt(int irq, void *arg)
    {
    struct twl6030_charger_info *charger = arg;
    u8 val;
    u8 int_stat;
    u8 stat_int1;
    u8 stat_int2;
    if (twl6030_charger_read(CONTROLLER_STAT1, &val) < 0)
    return IRQ_HANDLED;
    if (twl6030_charger_read(CHARGERUSB_INT_STATUS, &int_stat) < 0)
    return IRQ_HANDLED;
    if (twl6030_charger_read(CHARGERUSB_STATUS_INT1, &stat_int1) < 0)
    return IRQ_HANDLED;
    if (twl6030_charger_read(CHARGERUSB_STATUS_INT2, &stat_int2) < 0)
    return IRQ_HANDLED;
    dev_dbg(charger.dev,
    "charger irq: stat1: %02x %s INT_STATUS %02x STATUS_INT1 %02x STATUS_INT2 %02x\n",
    val, (val & VBUS_DET) ? "usb online" :  "usb offline",
    int_stat, stat_int1, stat_int2);
    power_supply_changed(charger.usb);
    if (val & VBUS_DET) {
    if (twl6030_charger_read(CONTROLLER_CTRL1, &val) < 0)
    return IRQ_HANDLED;
    if (!(val & CONTROLLER_CTRL1_EN_CHARGER)) {
    if (twl6030_enable_usb(charger) < 0)
    return IRQ_HANDLED;
    schedule_delayed_work(&charger.charger_monitor,
    msecs_to_jiffies(10000));
    }
    } else {
    cancel_delayed_work(&charger.charger_monitor);
    }
    return IRQ_HANDLED;
    }
    static int twl6030_charger_usb_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct twl6030_charger_info *charger = power_supply_get_drvdata(psy);
    int ret;
    u8 stat1;
    u8 intstat;
    ret = twl6030_charger_read(CONTROLLER_STAT1, &stat1);
    if (ret)
    return ret;
    switch (psp) {
    case POWER_SUPPLY_PROP_STATUS:
    if (!(stat1 & VBUS_DET)) {
    val.intval = POWER_SUPPLY_STATUS_NOT_CHARGING;
    break;
    }
    ret = twl6030_charger_read(CHARGERUSB_STATUS_INT2, &intstat);
    if (ret)
    return ret;
    if (intstat & CHARGE_DONE)
    val.intval = POWER_SUPPLY_STATUS_FULL;
#[no_mangle]
pub unsafe extern "C" fn if(CURRENT_TERM: intstat &) -> else {
    else if (intstat & CURRENT_TERM)
    val.intval = POWER_SUPPLY_STATUS_NOT_CHARGING;
    else
    val.intval = POWER_SUPPLY_STATUS_CHARGING;
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_NOW:
    if (!charger.channel_vusb)
    return -ENODATA;
    ret = iio_read_channel_processed_scale(charger.channel_vusb, &val.intval, 1000);
    if (ret < 0)
    return ret;
    break;
    case POWER_SUPPLY_PROP_ONLINE:
    val.intval = !!(stat1 & VBUS_DET);
    break;
    case POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT:
    val.intval = charger.input_current_limit;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int twl6030_charger_usb_set_property(struct power_supply *psy,
    enum power_supply_property psp,
    const union power_supply_propval *val)
    {
    struct twl6030_charger_info *charger = power_supply_get_drvdata(psy);
    switch (psp) {
    case POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT:
    charger.input_current_limit = val.intval;
    return twl6030_config_cinlimit_reg(charger, charger.input_current_limit);
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int twl6030_charger_usb_property_is_writeable(struct power_supply *psy,
    enum power_supply_property psp)
    {
    dev_info(&psy.dev, "is %d writeable?\n", (int)psp);
    switch (psp) {
    case POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT:
    return true;
    default:
    return false;
    }
    }
    static enum power_supply_property twl6030_charger_props[] = {
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_ONLINE,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT,
    };
    static const struct power_supply_desc twl6030_charger_usb_desc = {
    .name		= "twl6030_usb",
    .type		= POWER_SUPPLY_TYPE_USB,
    .properties	= twl6030_charger_props,
    .num_properties	= ARRAY_SIZE(twl6030_charger_props),
    .get_property	= twl6030_charger_usb_get_property,
    .set_property	= twl6030_charger_usb_set_property,
    .property_is_writeable	= twl6030_charger_usb_property_is_writeable,
    };
#[no_mangle]
unsafe extern "C" fn twl6030_charger_probe(pdev: *mut platform_device) -> c_int {
    static int twl6030_charger_probe(struct platform_device *pdev)
    {
    struct twl6030_charger_info *charger;
    const struct twl6030_charger_chip_data *chip_data;
    let mut psy_cfg: power_supply_config = {};
    int ret;
    u8 val;
    charger = devm_kzalloc(&pdev.dev, sizeof(*charger), GFP_KERNEL);
    if (!charger)
    return -ENOMEM;
    charger.dev = &pdev.dev;
    charger.irq_chg = platform_get_irq(pdev, 0);
    chip_data = device_get_match_data(&pdev.dev);
    if (!chip_data)
    return dev_err_probe(&pdev.dev, -EINVAL, "missing chip data\n");
    charger.extended_current_range = chip_data.extended_current_range;
    platform_set_drvdata(pdev, charger);
    psy_cfg.drv_data = charger;
    psy_cfg.fwnode = dev_fwnode(&pdev.dev);
    charger.channel_vusb = devm_iio_channel_get(&pdev.dev, "vusb");
    if (IS_ERR(charger.channel_vusb)) {
    ret = PTR_ERR(charger.channel_vusb);
    if (ret == -EPROBE_DEFER)
    return ret;	/* iio not ready */
    dev_warn(&pdev.dev, "could not request vusb iio channel (%d)",
    ret);
    charger.channel_vusb = core::ptr::null_mut();
    }
    charger.usb = devm_power_supply_register(&pdev.dev,
    &twl6030_charger_usb_desc,
    &psy_cfg);
    if (IS_ERR(charger.usb))
    return dev_err_probe(&pdev.dev, PTR_ERR(charger.usb),
    "Failed to register usb\n");
    ret = power_supply_get_battery_info(charger.usb, &charger.binfo);
    if (ret < 0)
    return dev_err_probe(&pdev.dev, ret,
    "Failed to get battery info\n");
    dev_info(&pdev.dev, "battery with vmax %d imax: %d\n",
    charger.binfo.constant_charge_voltage_max_uv,
    charger.binfo.constant_charge_current_max_ua);
    if (charger.binfo.constant_charge_voltage_max_uv == -EINVAL) {
    ret = twl6030_charger_read(CHARGERUSB_CTRLLIMIT1, &val);
    if (ret < 0)
    return ret;
    charger.binfo.constant_charge_voltage_max_uv =
    VOREG_TO_UV(val);
    }
    if (charger.binfo.constant_charge_voltage_max_uv > 4760000 ||
    charger.binfo.constant_charge_voltage_max_uv < 350000)
    return dev_err_probe(&pdev.dev, -EINVAL,
    "Invalid charge voltage\n");
    if (charger.binfo.constant_charge_current_max_ua == -EINVAL) {
    ret = twl6030_charger_read(CHARGERUSB_CTRLLIMIT2, &val);
    if (ret < 0)
    return ret;
    charger.binfo.constant_charge_current_max_ua = VICHRG_TO_UA(val);
    }
    if (charger.binfo.constant_charge_current_max_ua < 100000 ||
    charger.binfo.constant_charge_current_max_ua > 1500000) {
    return dev_err_probe(&pdev.dev, -EINVAL,
    "Invalid charge current\n");
    }
    if ((charger.binfo.charge_term_current_ua != -EINVAL) &&
    (charger.binfo.charge_term_current_ua > 400000 ||
    charger.binfo.charge_term_current_ua < 50000)) {
    return dev_err_probe(&pdev.dev, -EINVAL,
    "Invalid charge termination current\n");
    }
    ret = devm_delayed_work_autocancel(&pdev.dev,
    &charger.charger_monitor,
    twl6030_charger_wdg);
    if (ret < 0)
    return dev_err_probe(&pdev.dev, ret,
    "Failed to register delayed work\n");
    ret = devm_request_threaded_irq(&pdev.dev, charger.irq_chg, core::ptr::null_mut(),
    twl6030_charger_interrupt,
    IRQF_ONESHOT, pdev.name,
    charger);
    if (ret < 0)
    return ret;
// turing to charging to configure things
    twl6030_charger_write(CONTROLLER_CTRL1, 0);
    twl6030_charger_interrupt(0, charger);
    return 0;
    }
    static const struct twl6030_charger_chip_data twl6030_data = {
    .extended_current_range = false,
    };
    static const struct twl6030_charger_chip_data twl6032_data = {
    .extended_current_range = true,
    };
    static const struct of_device_id twl_charger_of_match[] = {
    {.compatible = "ti,twl6030-charger", .data = &twl6030_data},
    {.compatible = "ti,twl6032-charger", .data = &twl6032_data},
    { }
    };
    MODULE_DEVICE_TABLE(of, twl_charger_of_match);
    static struct platform_driver twl6030_charger_driver = {
    .probe = twl6030_charger_probe,
    .driver	= {
    .name	= "twl6030_charger",
    .of_match_table = twl_charger_of_match,
    },
    };
    module_platform_driver(twl6030_charger_driver);
    MODULE_DESCRIPTION("TWL6030 Battery Charger Interface driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_CONSUMER");
