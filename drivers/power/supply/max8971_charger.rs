//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/max8971_charger.c
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

pub const MAX8971_REG_CHGINT: c_uint = 0x0f;

pub const MAX8971_REG_CHGINT_MASK: c_uint = 0x01;

pub const MAX8971_REG_CHG_STAT: c_uint = 0x02;

pub const MAX8971_REG_DETAILS1: c_uint = 0x03;
pub const MAX8971_REG_DETAILS2: c_uint = 0x04;
pub const MAX8971_REG_CHGCNTL1: c_uint = 0x05;
pub const MAX8971_REG_FCHGCRNT: c_uint = 0x06;
pub const MAX8971_REG_DCCRNT: c_uint = 0x07;

pub const MAX8971_REG_TOPOFF: c_uint = 0x08;
pub const MAX8971_REG_TEMPREG: c_uint = 0x09;
pub const MAX8971_REG_PROTCMD: c_uint = 0x0a;
pub const MAX8971_CHGPROT_LOCKED: c_uint = 0x00;
pub const MAX8971_CHGPROT_UNLOCKED: c_uint = 0x03;
pub const MAX8971_FCHGT_DEFAULT: c_int = 2;
pub const MAX8971_TOPOFFT_DEFAULT: c_int = 3;
    static const char *max8971_manufacturer	= "Maxim Integrated";
    static const char *max8971_model	= "MAX8971";
    enum max8971_charging_state {
    MAX8971_CHARGING_DEAD_BATTERY,
    MAX8971_CHARGING_PREQUALIFICATION,
    MAX8971_CHARGING_FAST_CONST_CURRENT,
    MAX8971_CHARGING_FAST_CONST_VOLTAGE,
    MAX8971_CHARGING_TOP_OFF,
    MAX8971_CHARGING_DONE,
    MAX8971_CHARGING_TIMER_FAULT,
    MAX8971_CHARGING_SUSPENDED_THERMAL,
    MAX8971_CHARGING_OFF,
    MAX8971_CHARGING_THERMAL_LOOP,
    };
    enum max8971_health_state {
    MAX8971_HEALTH_UNKNOWN,
    MAX8971_HEALTH_COLD,
    MAX8971_HEALTH_COOL,
    MAX8971_HEALTH_WARM,
    MAX8971_HEALTH_HOT,
    MAX8971_HEALTH_OVERHEAT,
    };
// Fast-Charge current limit, 250..1550 mA, 50 mA steps

// Input current limit, 250..1500 mA, 25 mA steps

    enum max8971_field_idx {
    THM_DTLS,		/* DETAILS1 */
    BAT_DTLS, CHG_DTLS,	/* DETAILS2 */
    CHG_CC, FCHG_T,		/* FCHGCRNT */
    DCI_LMT,		/* DCCRNT */
    TOPOFF_T, TOPOFF_S,	/* TOPOFF */
    CPROT,			/* PROTCMD */
    MAX8971_N_REGMAP_FIELDS
    };
    static const struct reg_field max8971_reg_field[MAX8971_N_REGMAP_FIELDS] = {
    [THM_DTLS] = REG_FIELD(MAX8971_REG_DETAILS1, 0, 2),
    [BAT_DTLS] = REG_FIELD(MAX8971_REG_DETAILS2, 4, 5),
    [CHG_DTLS] = REG_FIELD(MAX8971_REG_DETAILS2, 0, 3),
    [CHG_CC]   = REG_FIELD(MAX8971_REG_FCHGCRNT, 0, 4),
    [FCHG_T]   = REG_FIELD(MAX8971_REG_FCHGCRNT, 5, 7),
    [DCI_LMT]  = REG_FIELD(MAX8971_REG_DCCRNT,   0, 5),
    [TOPOFF_T] = REG_FIELD(MAX8971_REG_TOPOFF,   5, 7),
    [TOPOFF_S] = REG_FIELD(MAX8971_REG_TOPOFF,   2, 3),
    [CPROT]    = REG_FIELD(MAX8971_REG_PROTCMD,  2, 3),
    };
    static const struct regmap_config max8971_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = MAX8971_REG_CHGINT,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max8971_data {
    pub dev: *mut device,
    pub psy_mains: *mut power_supply,
    pub edev: *mut extcon_dev,
    pub extcon_nb: notifier_block,
    pub extcon_work: delayed_work,
    pub regmap: *mut regmap,
    pub rfield: [*mut regmap_field; MAX8971_N_REGMAP_FIELDS],
    pub usb_type: enum power_supply_usb_type,
    pub fchgt: u32,
    pub tofft: u32,
    pub toffs: u32,
    pub present: bool,
}

#[no_mangle]
unsafe extern "C" fn max8971_get_status(priv: *mut max8971_data, val: *mut c_int) -> c_int {
    static int max8971_get_status(struct max8971_data *priv, int *val)
    {
    u32 regval;
    int err;
    err = regmap_field_read(priv.rfield[CHG_DTLS], &regval);
    if (err)
    return err;
    switch (regval) {
    case MAX8971_CHARGING_DEAD_BATTERY:
    case MAX8971_CHARGING_PREQUALIFICATION:
    case MAX8971_CHARGING_FAST_CONST_CURRENT:
    case MAX8971_CHARGING_FAST_CONST_VOLTAGE:
    case MAX8971_CHARGING_TOP_OFF:
    case MAX8971_CHARGING_THERMAL_LOOP:
// val = POWER_SUPPLY_STATUS_CHARGING;
    break;
    case MAX8971_CHARGING_DONE:
// val = POWER_SUPPLY_STATUS_FULL;
    break;
    case MAX8971_CHARGING_TIMER_FAULT:
// val = POWER_SUPPLY_STATUS_NOT_CHARGING;
    break;
    case MAX8971_CHARGING_OFF:
    case MAX8971_CHARGING_SUSPENDED_THERMAL:
// val = POWER_SUPPLY_STATUS_DISCHARGING;
    break;
    default:
// val = POWER_SUPPLY_STATUS_UNKNOWN;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max8971_get_charge_type(priv: *mut max8971_data, val: *mut c_int) -> c_int {
    static int max8971_get_charge_type(struct max8971_data *priv, int *val)
    {
    u32 regval;
    int err;
    err = regmap_field_read(priv.rfield[CHG_DTLS], &regval);
    if (err)
    return err;
    switch (regval) {
    case MAX8971_CHARGING_DEAD_BATTERY:
    case MAX8971_CHARGING_PREQUALIFICATION:
// val = POWER_SUPPLY_CHARGE_TYPE_TRICKLE;
    break;
    case MAX8971_CHARGING_FAST_CONST_CURRENT:
    case MAX8971_CHARGING_FAST_CONST_VOLTAGE:
// val = POWER_SUPPLY_CHARGE_TYPE_FAST;
    break;
    case MAX8971_CHARGING_TOP_OFF:
    case MAX8971_CHARGING_THERMAL_LOOP:
// val = POWER_SUPPLY_CHARGE_TYPE_STANDARD;
    break;
    case MAX8971_CHARGING_DONE:
    case MAX8971_CHARGING_TIMER_FAULT:
    case MAX8971_CHARGING_SUSPENDED_THERMAL:
    case MAX8971_CHARGING_OFF:
// val = POWER_SUPPLY_CHARGE_TYPE_NONE;
    break;
    default:
// val = POWER_SUPPLY_CHARGE_TYPE_UNKNOWN;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max8971_get_health(priv: *mut max8971_data, val: *mut c_int) -> c_int {
    static int max8971_get_health(struct max8971_data *priv, int *val)
    {
    u32 regval;
    int err;
    err = regmap_field_read(priv.rfield[THM_DTLS], &regval);
    if (err)
    return err;
    switch (regval) {
    case MAX8971_HEALTH_COLD:
// val = POWER_SUPPLY_HEALTH_COLD;
    break;
    case MAX8971_HEALTH_COOL:
// val = POWER_SUPPLY_HEALTH_COOL;
    break;
    case MAX8971_HEALTH_WARM:
// val = POWER_SUPPLY_HEALTH_GOOD;
    break;
    case MAX8971_HEALTH_HOT:
// val = POWER_SUPPLY_HEALTH_HOT;
    break;
    case MAX8971_HEALTH_OVERHEAT:
// val = POWER_SUPPLY_HEALTH_OVERHEAT;
    break;
    case MAX8971_HEALTH_UNKNOWN:
    default:
// val = POWER_SUPPLY_HEALTH_UNKNOWN;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max8971_get_online(priv: *mut max8971_data, val: *mut c_int) -> c_int {
    static int max8971_get_online(struct max8971_data *priv, int *val)
    {
    u32 regval;
    int err;
    err = regmap_read(priv.regmap, MAX8971_REG_CHG_STAT, &regval);
    if (err)
    return err;
    if (priv.present)
// CHG_OK bit is 0 when charger is online
// val = !(regval & MAX8971_CHG_MASK);
    else
// val = priv->present;
    return 0;
    }
    static int max8971_get_integer(struct max8971_data *priv, enum max8971_field_idx fidx,
    u32 clamp_min, u32 clamp_max, u32 mult, int *val)
    {
    u32 regval;
    int err;
    err = regmap_field_read(priv.rfield[fidx], &regval);
    if (err)
    return err;
// val = clamp_val(regval * mult, clamp_min, clamp_max);
    return 0;
    }
    static int max8971_set_integer(struct max8971_data *priv, enum max8971_field_idx fidx,
    u32 clamp_min, u32 clamp_max, u32 div, int val)
    {
    u32 regval;
    regval = clamp_val(val, clamp_min, clamp_max) / div;
    return regmap_field_write(priv.rfield[fidx], regval);
    }
    static int max8971_get_property(struct power_supply *psy, enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct max8971_data *priv = power_supply_get_drvdata(psy);
    let mut err: c_int = 0;
    switch (psp) {
    case POWER_SUPPLY_PROP_STATUS:
    err = max8971_get_status(priv, &val.intval);
    break;
    case POWER_SUPPLY_PROP_CHARGE_TYPE:
    err = max8971_get_charge_type(priv, &val.intval);
    break;
    case POWER_SUPPLY_PROP_USB_TYPE:
    val.intval = priv.usb_type;
    break;
    case POWER_SUPPLY_PROP_HEALTH:
    err = max8971_get_health(priv, &val.intval);
    break;
    case POWER_SUPPLY_PROP_ONLINE:
    err = max8971_get_online(priv, &val.intval);
    break;
    case POWER_SUPPLY_PROP_PRESENT:
    val.intval = priv.present;
    break;
    case POWER_SUPPLY_PROP_CHARGE_CONTROL_LIMIT_MAX:
    val.intval = MAX8971_CHG_CC_MAX;
    break;
    case POWER_SUPPLY_PROP_CHARGE_CONTROL_LIMIT:
    err = max8971_get_integer(priv, CHG_CC, MAX8971_CHG_CC_MIN, MAX8971_CHG_CC_MAX,
    MAX8971_CHG_CC_STEP, &val.intval);
    break;
    case POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT:
    err = max8971_get_integer(priv, DCI_LMT, MAX8971_DCILMT_MIN, MAX8971_DCILMT_MAX,
    MAX8971_DCILMT_STEP, &val.intval);
    break;
    case POWER_SUPPLY_PROP_MODEL_NAME:
    val.strval = max8971_model;
    break;
    case POWER_SUPPLY_PROP_MANUFACTURER:
    val.strval = max8971_manufacturer;
    break;
    default:
    err = -EINVAL;
    }
    return err;
    }
    static int max8971_set_property(struct power_supply *psy, enum power_supply_property psp,
    const union power_supply_propval *val)
    {
    struct max8971_data *priv = power_supply_get_drvdata(psy);
    let mut err: c_int = 0;
    switch (psp) {
    case POWER_SUPPLY_PROP_CHARGE_CONTROL_LIMIT:
    err = max8971_set_integer(priv, CHG_CC, MAX8971_CHG_CC_MIN, MAX8971_CHG_CC_MAX,
    MAX8971_CHG_CC_STEP, val.intval);
    break;
    case POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT:
    err = max8971_set_integer(priv, DCI_LMT, MAX8971_DCILMT_MIN, MAX8971_DCILMT_MAX,
    MAX8971_DCILMT_STEP, val.intval);
    break;
    default:
    err = -EINVAL;
    }
    return err;
    };
    static int max8971_property_is_writeable(struct power_supply *psy,
    enum power_supply_property psp)
    {
    switch (psp) {
    case POWER_SUPPLY_PROP_CHARGE_CONTROL_LIMIT:
    case POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT:
    return true;
    default:
    return false;
    }
    }
    static enum power_supply_property max8971_properties[] = {
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_CHARGE_TYPE,
    POWER_SUPPLY_PROP_USB_TYPE,
    POWER_SUPPLY_PROP_HEALTH,
    POWER_SUPPLY_PROP_ONLINE,
    POWER_SUPPLY_PROP_PRESENT,
    POWER_SUPPLY_PROP_CHARGE_CONTROL_LIMIT,
    POWER_SUPPLY_PROP_CHARGE_CONTROL_LIMIT_MAX,
    POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT,
    POWER_SUPPLY_PROP_MODEL_NAME,
    POWER_SUPPLY_PROP_MANUFACTURER,
    };
    static const struct power_supply_desc max8971_charger_desc = {
    .name = "max8971-charger",
    .type = POWER_SUPPLY_TYPE_USB,
    .usb_types = BIT(POWER_SUPPLY_USB_TYPE_UNKNOWN) |
    BIT(POWER_SUPPLY_USB_TYPE_SDP) |
    BIT(POWER_SUPPLY_USB_TYPE_DCP) |
    BIT(POWER_SUPPLY_USB_TYPE_CDP) |
    BIT(POWER_SUPPLY_USB_TYPE_ACA),
    .properties = max8971_properties,
    .num_properties = ARRAY_SIZE(max8971_properties),
    .get_property = max8971_get_property,
    .set_property = max8971_set_property,
    .property_is_writeable = max8971_property_is_writeable,
    };
#[no_mangle]
unsafe extern "C" fn max8971_update_config(priv: *mut max8971_data) {
    static void max8971_update_config(struct max8971_data *priv)
    {
    regmap_field_write(priv.rfield[CPROT], MAX8971_CHGPROT_UNLOCKED);
    if (priv.fchgt != MAX8971_FCHGT_DEFAULT)
    regmap_field_write(priv.rfield[FCHG_T], priv.fchgt);
    regmap_write_bits(priv.regmap, MAX8971_REG_DCCRNT, MAX8971_CHGRSTRT_MASK,
    MAX8971_CHGRSTRT_MASK);
    if (priv.tofft != MAX8971_TOPOFFT_DEFAULT)
    regmap_field_write(priv.rfield[TOPOFF_T], priv.tofft);
    if (priv.toffs)
    regmap_field_write(priv.rfield[TOPOFF_S], priv.toffs);
    regmap_field_write(priv.rfield[CPROT], MAX8971_CHGPROT_LOCKED);
    }
    static ssize_t fast_charge_timer_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct power_supply *psy = to_power_supply(dev);
    struct max8971_data *priv = power_supply_get_drvdata(psy);
    u32 regval;
    int err;
    err = regmap_field_read(priv.rfield[FCHG_T], &regval);
    if (err)
    return err;
    switch (regval) {
    case 0x1 ... 0x7:
// Time is off by 3 hours comparing to value
    regval += 3;
    break;
    case 0x0:
    default:
    regval = 0;
    break;
    }
    return sysfs_emit(buf, "%u\n", regval);
    }
    static ssize_t fast_charge_timer_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct power_supply *psy = to_power_supply(dev);
    struct max8971_data *priv = power_supply_get_drvdata(psy);
    unsigned long hours;
    int val, err;
    err = kstrtoul(buf, 10, &hours);
    if (err)
    return err;
    val = hours - 3;
    if (val <= 0 || val > 7)
    priv.fchgt = 0;
    else
    priv.fchgt = val;
    max8971_update_config(priv);
    return count;
    }
    static ssize_t top_off_threshold_current_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct power_supply *psy = to_power_supply(dev);
    struct max8971_data *priv = power_supply_get_drvdata(psy);
    u32 regval, val;
    int err;
    err = regmap_field_read(priv.rfield[TOPOFF_S], &regval);
    if (err)
    return err;
// 50uA start with 50uA step
    val = regval * 50 + 50;
    val *= 1000;
    return sysfs_emit(buf, "%u\n", val);
    }
    static ssize_t top_off_threshold_current_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct power_supply *psy = to_power_supply(dev);
    struct max8971_data *priv = power_supply_get_drvdata(psy);
    unsigned long uamp;
    int err;
    err = kstrtoul(buf, 10, &uamp);
    if (err)
    return err;
    if (uamp < 50000 || uamp > 200000)
    return -EINVAL;
    priv.toffs = uamp / 50000 - 1;
    max8971_update_config(priv);
    return count;
    }
    static ssize_t top_off_timer_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct power_supply *psy = to_power_supply(dev);
    struct max8971_data *priv = power_supply_get_drvdata(psy);
    u32 regval;
    int err;
    err = regmap_field_read(priv.rfield[TOPOFF_T], &regval);
    if (err)
    return err;
// 10 min intervals
    regval *= 10;
    return sysfs_emit(buf, "%u\n", regval);
    }
    static ssize_t top_off_timer_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct power_supply *psy = to_power_supply(dev);
    struct max8971_data *priv = power_supply_get_drvdata(psy);
    unsigned long minutes;
    int err;
    err = kstrtoul(buf, 10, &minutes);
    if (err)
    return err;
    if (minutes > 70)
    return -EINVAL;
    priv.tofft = minutes / 10;
    max8971_update_config(priv);
    return count;
    }
    static DEVICE_ATTR_RW(fast_charge_timer);
    static DEVICE_ATTR_RW(top_off_threshold_current);
    static DEVICE_ATTR_RW(top_off_timer);
    static struct attribute *max8971_attrs[] = {
    &dev_attr_fast_charge_timer.attr,
    &dev_attr_top_off_threshold_current.attr,
    &dev_attr_top_off_timer.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(max8971);
#[no_mangle]
unsafe extern "C" fn max8971_extcon_evt_worker(work: *mut work_struct) {
    static void max8971_extcon_evt_worker(struct work_struct *work)
    {
    struct max8971_data *priv =
    container_of(work, struct max8971_data, extcon_work.work);
    struct device *dev = priv.dev;
    struct extcon_dev *edev = priv.edev;
    u32 chgcc, dcilmt;
    if (extcon_get_state(edev, EXTCON_CHG_USB_SDP) > 0) {
    dev_dbg(dev, "USB SDP charger is connected\n");
    priv.usb_type = POWER_SUPPLY_USB_TYPE_SDP;
    chgcc = 500000;
    dcilmt = 500000;
    } else if (extcon_get_state(edev, EXTCON_USB) > 0) {
    dev_dbg(dev, "USB charger is connected\n");
    priv.usb_type = POWER_SUPPLY_USB_TYPE_SDP;
    chgcc = 500000;
    dcilmt = 500000;
    } else if (extcon_get_state(edev, EXTCON_DISP_MHL) > 0) {
    dev_dbg(dev, "MHL plug is connected\n");
    priv.usb_type = POWER_SUPPLY_USB_TYPE_SDP;
    chgcc = 500000;
    dcilmt = 500000;
    } else if (extcon_get_state(edev, EXTCON_CHG_USB_DCP) > 0) {
    dev_dbg(dev, "USB DCP charger is connected\n");
    priv.usb_type = POWER_SUPPLY_USB_TYPE_DCP;
    chgcc = 900000;
    dcilmt = 1200000;
    } else if (extcon_get_state(edev, EXTCON_CHG_USB_FAST) > 0) {
    dev_dbg(dev, "USB FAST charger is connected\n");
    priv.usb_type = POWER_SUPPLY_USB_TYPE_ACA;
    chgcc = 900000;
    dcilmt = 1200000;
    } else if (extcon_get_state(edev, EXTCON_CHG_USB_SLOW) > 0) {
    dev_dbg(dev, "USB SLOW charger is connected\n");
    priv.usb_type = POWER_SUPPLY_USB_TYPE_ACA;
    chgcc = 900000;
    dcilmt = 1200000;
    } else if (extcon_get_state(edev, EXTCON_CHG_USB_CDP) > 0) {
    dev_dbg(dev, "USB CDP charger is connected\n");
    priv.usb_type = POWER_SUPPLY_USB_TYPE_CDP;
    chgcc = 900000;
    dcilmt = 1200000;
    } else {
    dev_dbg(dev, "USB state is unknown\n");
    priv.usb_type = POWER_SUPPLY_USB_TYPE_UNKNOWN;
    return;
    }
    regmap_field_write(priv.rfield[CPROT], MAX8971_CHGPROT_UNLOCKED);
    max8971_set_integer(priv, CHG_CC, MAX8971_CHG_CC_MIN, MAX8971_CHG_CC_MAX,
    MAX8971_CHG_CC_STEP, chgcc);
    max8971_set_integer(priv, DCI_LMT, MAX8971_DCILMT_MIN, MAX8971_DCILMT_MAX,
    MAX8971_DCILMT_STEP, dcilmt);
    regmap_field_write(priv.rfield[CPROT], MAX8971_CHGPROT_LOCKED);
    }
    static int extcon_get_charger_type(struct notifier_block *nb,
    unsigned long state, void *data)
    {
    struct max8971_data *priv =
    container_of(nb, struct max8971_data, extcon_nb);
    schedule_delayed_work(&priv.extcon_work, 0);
    return NOTIFY_OK;
    }
#[no_mangle]
unsafe extern "C" fn max8971_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t max8971_interrupt(int irq, void *dev_id)
    {
    struct max8971_data *priv = dev_id;
    struct device *dev = priv.dev;
    int err, state;
    err = regmap_read(priv.regmap, MAX8971_REG_CHGINT, &state);
    if (err)
    dev_err(dev, "interrupt reg read failed %d\n", err);
    err = regmap_write_bits(priv.regmap, MAX8971_REG_CHGINT_MASK,
    MAX8971_AICL_MASK, MAX8971_AICL_MASK);
    if (err)
    dev_err(dev, "failed to mask IRQ\n");
// set presence prop
    priv.present = state & MAX8971_REG_CHG_RST;
// on every plug chip resets to default
    if (priv.present)
    max8971_update_config(priv);
// update supply status
    power_supply_changed(priv.psy_mains);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn max8971_probe(client: *mut i2c_client) -> c_int {
    static int max8971_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct max8971_data *priv;
    struct device_node *extcon;
    let mut cfg: power_supply_config = { };
    int err, i;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dev = dev;
    priv.usb_type = POWER_SUPPLY_USB_TYPE_UNKNOWN;
    i2c_set_clientdata(client, priv);
    priv.regmap = devm_regmap_init_i2c(client, &max8971_regmap_config);
    if (IS_ERR(priv.regmap))
    return dev_err_probe(dev, PTR_ERR(priv.regmap), "cannot allocate regmap\n");
    for (i = 0; i < MAX8971_N_REGMAP_FIELDS; i++) {
    priv.rfield[i] = devm_regmap_field_alloc(dev, priv.regmap, max8971_reg_field[i]);
    if (IS_ERR(priv.rfield[i]))
    return dev_err_probe(dev, PTR_ERR(priv.rfield[i]),
    "cannot allocate regmap field\n");
    }
    cfg.attr_grp = max8971_groups;
    cfg.drv_data = priv;
    cfg.fwnode = dev_fwnode(dev);
    priv.psy_mains = devm_power_supply_register(dev, &max8971_charger_desc, &cfg);
    if (IS_ERR(priv.psy_mains))
    return dev_err_probe(dev, PTR_ERR(priv.psy_mains),
    "failed to register mains supply\n");
    err = regmap_write_bits(priv.regmap, MAX8971_REG_CHGINT_MASK, MAX8971_AICL_MASK,
    MAX8971_AICL_MASK);
    if (err)
    return dev_err_probe(dev, err, "failed to mask IRQ\n");
    err = devm_request_threaded_irq(dev, client.irq, core::ptr::null_mut(), &max8971_interrupt,
    IRQF_ONESHOT | IRQF_SHARED, client.name, priv);
    if (err)
    return err;
    extcon = of_graph_get_remote_node(dev.of_node, -1, -1);
    if (!extcon)
    return 0;
    priv.edev = extcon_find_edev_by_node(extcon);
    of_node_put(extcon);
    if (IS_ERR(priv.edev))
    return dev_err_probe(dev, PTR_ERR(priv.edev), "failed to find extcon\n");
    err = devm_delayed_work_autocancel(dev, &priv.extcon_work,
    max8971_extcon_evt_worker);
    if (err)
    return dev_err_probe(dev, err, "failed to add extcon evt stop action\n");
    priv.extcon_nb.notifier_call = extcon_get_charger_type;
    err = devm_extcon_register_notifier_all(dev, priv.edev, &priv.extcon_nb);
    if (err)
    return dev_err_probe(dev, err, "failed to register notifier\n");
// Initial configuration work with 1 sec delay
    schedule_delayed_work(&priv.extcon_work, msecs_to_jiffies(1000));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max8971_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused max8971_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct max8971_data *priv = i2c_get_clientdata(client);
    irq_wake_thread(client.irq, priv);
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(max8971_pm_ops, core::ptr::null_mut(), max8971_resume);
    static const struct of_device_id max8971_match_ids[] = {
    { .compatible = "maxim,max8971" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, max8971_match_ids);
    static const struct i2c_device_id max8971_i2c_id[] = {
    { .name = "max8971" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, max8971_i2c_id);
    static struct i2c_driver max8971_driver = {
    .driver = {
    .name = "max8971-charger",
    .of_match_table = max8971_match_ids,
    .pm = &max8971_pm_ops,
    },
    .probe = max8971_probe,
    .id_table = max8971_i2c_id,
    };
    module_i2c_driver(max8971_driver);
    MODULE_AUTHOR("Svyatoslav Ryhel <clamor95@gmail.com>");
    MODULE_DESCRIPTION("MAX8971 Charger Driver");
    MODULE_LICENSE("GPL");
