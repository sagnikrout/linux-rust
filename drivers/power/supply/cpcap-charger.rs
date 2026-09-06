//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/cpcap-charger.c
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
// Motorola CPCAP PMIC battery charger driver
//
// Copyright (C) 2017 Tony Lindgren <tony@atomide.com>
//
// Rewritten for Linux power framework with some parts based on
// earlier driver found in the Motorola Linux kernel:
//
// Copyright (C) 2009-2010 Motorola, Inc.
//

//
// CPCAP_REG_CRM register bits. For documentation of somewhat similar hardware,
// see NXP "MC13783 Power Management and Audio Circuit Users's Guide"
// MC13783UG.pdf chapter "8.5 Battery Interface Register Summary". The registers
// and values for CPCAP are different, but some of the internal components seem
// similar. Also see the Motorola Linux kernel cpcap-regbits.h. CPCAP_REG_CHRGR_1
// bits that seem to describe the CRM register.
//

// CPCAP_REG_CRM trickle charge voltages

//
// CPCAP_REG_CRM charge voltages based on the ADC channel 1 values.
// Note that these register bits don't match MC13783UG.pdf VCHRG
// register bits.
//

//
// CPCAP_REG_CRM charge currents. These seem to match MC13783UG.pdf
// values in "Table 8-3. Charge Path Regulator Current Limit
// Characteristics" for the nominal values.
//
// Except 70mA and 1.596A and unlimited, these are simply 88.7mA / step.
//

// CPCAP_REG_VUSBC register bits needed for VBUS

    enum {
    CPCAP_CHARGER_IIO_BATTDET,
    CPCAP_CHARGER_IIO_VOLTAGE,
    CPCAP_CHARGER_IIO_VBUS,
    CPCAP_CHARGER_IIO_CHRG_CURRENT,
    CPCAP_CHARGER_IIO_BATT_CURRENT,
    CPCAP_CHARGER_IIO_NR,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpcap_charger_ddata {
    pub dev: *mut device,
    pub reg: *mut regmap,
    pub irq_list: list_head,
    pub detect_work: delayed_work,
    pub vbus_work: delayed_work,
    pub /: *mut *mut *mut gpio_desc gpio[2]; / gpio_reven0 & 1,
    pub channels: [*mut iio_channel; CPCAP_CHARGER_IIO_NR],
    pub usb: *mut power_supply,
    pub /: *mut *mut phy_companion comparator; / For USB VBUS,
    pub vbus_enabled:1: c_uint,
    pub feeding_vbus:1: c_uint,
    pub active: core::sync::atomic::AtomicI32,
    pub status: c_int,
    pub voltage: c_int,
    pub limit_current: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpcap_interrupt_desc {
    pub irq: c_int,
    pub node: list_head,
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpcap_charger_ints_state {
    pub chrg_det: bool,
    pub rvrs_chrg: bool,
    pub vbusov: bool,
    pub chrg_se1b: bool,
    pub rvrs_mode: bool,
    pub chrgcurr2: bool,
    pub chrgcurr1: bool,
    pub vbusvld: bool,
    pub battdetb: bool,
}

    static enum power_supply_property cpcap_charger_props[] = {
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_ONLINE,
    POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE,
    POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    POWER_SUPPLY_PROP_CURRENT_NOW,
    };
#[no_mangle]
unsafe extern "C" fn cpcap_charger_get_charge_voltage(ddata: *mut cpcap_charger_ddata) -> c_int {
    static int cpcap_charger_get_charge_voltage(struct cpcap_charger_ddata *ddata)
    {
    struct iio_channel *channel;
    int error, value = 0;
    channel = ddata.channels[CPCAP_CHARGER_IIO_VOLTAGE];
    error = iio_read_channel_processed(channel, &value);
    if (error < 0) {
    dev_warn(ddata.dev, "%s failed: %i\n", __func__, error);
    return 0;
    }
    return value;
    }
#[no_mangle]
unsafe extern "C" fn cpcap_charger_get_charge_current(ddata: *mut cpcap_charger_ddata) -> c_int {
    static int cpcap_charger_get_charge_current(struct cpcap_charger_ddata *ddata)
    {
    struct iio_channel *channel;
    int error, value = 0;
    channel = ddata.channels[CPCAP_CHARGER_IIO_CHRG_CURRENT];
    error = iio_read_channel_processed(channel, &value);
    if (error < 0) {
    dev_warn(ddata.dev, "%s failed: %i\n", __func__, error);
    return 0;
    }
    return value;
    }
    static int cpcap_charger_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct cpcap_charger_ddata *ddata = dev_get_drvdata(psy.dev.parent);
    switch (psp) {
    case POWER_SUPPLY_PROP_STATUS:
    val.intval = ddata.status;
    break;
    case POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT:
    val.intval = ddata.limit_current;
    break;
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE:
    val.intval = ddata.voltage;
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_NOW:
    if (ddata.status == POWER_SUPPLY_STATUS_CHARGING)
    val.intval = cpcap_charger_get_charge_voltage(ddata) *
    1000;
    else
    val.intval = 0;
    break;
    case POWER_SUPPLY_PROP_CURRENT_NOW:
    if (ddata.status == POWER_SUPPLY_STATUS_CHARGING)
    val.intval = cpcap_charger_get_charge_current(ddata) *
    1000;
    else
    val.intval = 0;
    break;
    case POWER_SUPPLY_PROP_ONLINE:
    val.intval = ddata.status == POWER_SUPPLY_STATUS_CHARGING;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cpcap_charger_match_voltage(voltage: c_int) -> c_int {
    static int cpcap_charger_match_voltage(int voltage)
    {
    switch (voltage) {
    case 0 ... 4100000 - 1: return 3800000;
    case 4100000 ... 4120000 - 1: return 4100000;
    case 4120000 ... 4150000 - 1: return 4120000;
    case 4150000 ... 4170000 - 1: return 4150000;
    case 4170000 ... 4200000 - 1: return 4170000;
    case 4200000 ... 4230000 - 1: return 4200000;
    case 4230000 ... 4250000 - 1: return 4230000;
    case 4250000 ... 4270000 - 1: return 4250000;
    case 4270000 ... 4300000 - 1: return 4270000;
    case 4300000 ... 4330000 - 1: return 4300000;
    case 4330000 ... 4350000 - 1: return 4330000;
    case 4350000 ... 4380000 - 1: return 4350000;
    case 4380000 ... 4400000 - 1: return 4380000;
    case 4400000 ... 4420000 - 1: return 4400000;
    case 4420000 ... 4440000 - 1: return 4420000;
    case 4440000: return 4440000;
    default: return 0;
    }
    }
    static int
    cpcap_charger_get_bat_const_charge_voltage(struct cpcap_charger_ddata *ddata)
    {
    union power_supply_propval prop;
    struct power_supply *battery;
    let mut voltage: c_int = ddata.voltage;
    int error;
    battery = power_supply_get_by_name("battery");
    if (battery) {
    error = power_supply_get_property(battery,
    POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE,
    &prop);
    if (!error)
    voltage = prop.intval;
    power_supply_put(battery);
    }
    return voltage;
    }
#[no_mangle]
unsafe extern "C" fn cpcap_charger_current_to_regval(microamp: c_int) -> c_int {
    static int cpcap_charger_current_to_regval(int microamp)
    {
    let mut miliamp: c_int = microamp / 1000;
    int res;
    if (miliamp < 0)
    return -EINVAL;
    if (miliamp < 70)
    return CPCAP_REG_CRM_ICHRG(0x0);
    if (miliamp < 177)
    return CPCAP_REG_CRM_ICHRG(0x1);
    if (miliamp >= 1596)
    return CPCAP_REG_CRM_ICHRG(0xe);
    res = microamp / 88666;
    if (res > 0xd)
    res = 0xd;
    return CPCAP_REG_CRM_ICHRG(res);
    }
    static int cpcap_charger_set_property(struct power_supply *psy,
    enum power_supply_property psp,
    const union power_supply_propval *val)
    {
    struct cpcap_charger_ddata *ddata = dev_get_drvdata(psy.dev.parent);
    int voltage, batvolt;
    switch (psp) {
    case POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT:
    if (cpcap_charger_current_to_regval(val.intval) < 0)
    return -EINVAL;
    ddata.limit_current = val.intval;
    schedule_delayed_work(&ddata.detect_work, 0);
    break;
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE:
    voltage = cpcap_charger_match_voltage(val.intval);
    batvolt = cpcap_charger_get_bat_const_charge_voltage(ddata);
    if (voltage > batvolt)
    voltage = batvolt;
    ddata.voltage = voltage;
    schedule_delayed_work(&ddata.detect_work, 0);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int cpcap_charger_property_is_writeable(struct power_supply *psy,
    enum power_supply_property psp)
    {
    switch (psp) {
    case POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT:
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE:
    return 1;
    default:
    return 0;
    }
    }
    static void cpcap_charger_set_cable_path(struct cpcap_charger_ddata *ddata,
    bool enabled)
    {
    if (!ddata.gpio[0])
    return;
    gpiod_set_value(ddata.gpio[0], enabled);
    }
    static void cpcap_charger_set_inductive_path(struct cpcap_charger_ddata *ddata,
    bool enabled)
    {
    if (!ddata.gpio[1])
    return;
    gpiod_set_value(ddata.gpio[1], enabled);
    }
    static void cpcap_charger_update_state(struct cpcap_charger_ddata *ddata,
    int state)
    {
    const char *status;
    if (state > POWER_SUPPLY_STATUS_FULL) {
    dev_warn(ddata.dev, "unknown state: %i\n", state);
    return;
    }
    ddata.status = state;
    switch (state) {
    case POWER_SUPPLY_STATUS_DISCHARGING:
    status = "DISCONNECTED";
    break;
    case POWER_SUPPLY_STATUS_NOT_CHARGING:
    status = "DETECTING";
    break;
    case POWER_SUPPLY_STATUS_CHARGING:
    status = "CHARGING";
    break;
    case POWER_SUPPLY_STATUS_FULL:
    status = "DONE";
    break;
    default:
    return;
    }
    dev_dbg(ddata.dev, "state: %s\n", status);
    }
#[no_mangle]
unsafe extern "C" fn cpcap_charger_disable(ddata: *mut cpcap_charger_ddata) -> c_int {
    static int cpcap_charger_disable(struct cpcap_charger_ddata *ddata)
    {
    int error;
    error = regmap_update_bits(ddata.reg, CPCAP_REG_CRM, 0x3fff,
    CPCAP_REG_CRM_FET_OVRD |
    CPCAP_REG_CRM_FET_CTRL);
    if (error)
    dev_err(ddata.dev, "%s failed with %i\n", __func__, error);
    return error;
    }
    static int cpcap_charger_enable(struct cpcap_charger_ddata *ddata,
    int max_voltage, int charge_current,
    int trickle_current)
    {
    int error;
    if (!max_voltage || !charge_current)
    return -EINVAL;
    dev_dbg(ddata.dev, "enable: %i %i %i\n",
    max_voltage, charge_current, trickle_current);
    error = regmap_update_bits(ddata.reg, CPCAP_REG_CRM, 0x3fff,
    CPCAP_REG_CRM_CHRG_LED_EN |
    trickle_current |
    CPCAP_REG_CRM_FET_OVRD |
    CPCAP_REG_CRM_FET_CTRL |
    max_voltage |
    charge_current);
    if (error)
    dev_err(ddata.dev, "%s failed with %i\n", __func__, error);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn cpcap_charger_vbus_valid(ddata: *mut cpcap_charger_ddata) -> bool {
    static bool cpcap_charger_vbus_valid(struct cpcap_charger_ddata *ddata)
    {
    int error, value = 0;
    struct iio_channel *channel =
    ddata.channels[CPCAP_CHARGER_IIO_VBUS];
    error = iio_read_channel_processed(channel, &value);
    if (error >= 0)
    return value > 3900;
    dev_err(ddata.dev, "error reading VBUS: %i\n", error);
    return false;
    }
// VBUS control functions for the USB PHY companion
#[no_mangle]
unsafe extern "C" fn cpcap_charger_vbus_work(work: *mut work_struct) {
    static void cpcap_charger_vbus_work(struct work_struct *work)
    {
    struct cpcap_charger_ddata *ddata;
    let mut vbus: bool = false;
    int error;
    ddata = container_of(work, struct cpcap_charger_ddata,
    vbus_work.work);
    if (ddata.vbus_enabled) {
    vbus = cpcap_charger_vbus_valid(ddata);
    if (vbus) {
    dev_dbg(ddata.dev, "VBUS already provided\n");
    return;
    }
    ddata.feeding_vbus = true;
    cpcap_charger_set_cable_path(ddata, false);
    cpcap_charger_set_inductive_path(ddata, false);
    error = cpcap_charger_disable(ddata);
    if (error)
    goto out_err;
    cpcap_charger_update_state(ddata,
    POWER_SUPPLY_STATUS_DISCHARGING);
    error = regmap_update_bits(ddata.reg, CPCAP_REG_VUSBC,
    CPCAP_BIT_VBUS_SWITCH,
    CPCAP_BIT_VBUS_SWITCH);
    if (error)
    goto out_err;
    error = regmap_update_bits(ddata.reg, CPCAP_REG_CRM,
    CPCAP_REG_CRM_RVRSMODE,
    CPCAP_REG_CRM_RVRSMODE);
    if (error)
    goto out_err;
    } else {
    error = regmap_update_bits(ddata.reg, CPCAP_REG_VUSBC,
    CPCAP_BIT_VBUS_SWITCH, 0);
    if (error)
    goto out_err;
    error = regmap_update_bits(ddata.reg, CPCAP_REG_CRM,
    CPCAP_REG_CRM_RVRSMODE, 0);
    if (error)
    goto out_err;
    cpcap_charger_set_cable_path(ddata, true);
    cpcap_charger_set_inductive_path(ddata, true);
    ddata.feeding_vbus = false;
    }
    return;
    out_err:
    cpcap_charger_update_state(ddata, POWER_SUPPLY_STATUS_UNKNOWN);
    dev_err(ddata.dev, "%s could not %s vbus: %i\n", __func__,
    str_enable_disable(ddata.vbus_enabled), error);
    }
    static int cpcap_charger_set_vbus(struct phy_companion *comparator,
    bool enabled)
    {
    struct cpcap_charger_ddata *ddata =
    container_of(comparator, struct cpcap_charger_ddata,
    comparator);
    ddata.vbus_enabled = enabled;
    schedule_delayed_work(&ddata.vbus_work, 0);
    return 0;
    }
// Charger interrupt handling functions
    static int cpcap_charger_get_ints_state(struct cpcap_charger_ddata *ddata,
    struct cpcap_charger_ints_state *s)
    {
    int val, error;
    error = regmap_read(ddata.reg, CPCAP_REG_INTS1, &val);
    if (error)
    return error;
    s.chrg_det = val & BIT(13);
    s.rvrs_chrg = val & BIT(12);
    s.vbusov = val & BIT(11);
    error = regmap_read(ddata.reg, CPCAP_REG_INTS2, &val);
    if (error)
    return error;
    s.chrg_se1b = val & BIT(13);
    s.rvrs_mode = val & BIT(6);
    s.chrgcurr2 = val & BIT(5);
    s.chrgcurr1 = val & BIT(4);
    s.vbusvld = val & BIT(3);
    error = regmap_read(ddata.reg, CPCAP_REG_INTS4, &val);
    if (error)
    return error;
    s.battdetb = val & BIT(6);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cpcap_charger_voltage_to_regval(voltage: c_int) -> c_int {
    static int cpcap_charger_voltage_to_regval(int voltage)
    {
    int offset;
    switch (voltage) {
    case 0 ... 4100000 - 1:
    return 0;
    case 4100000 ... 4200000 - 1:
    offset = 1;
    break;
    case 4200000 ... 4300000 - 1:
    offset = 0;
    break;
    case 4300000 ... 4380000 - 1:
    offset = -1;
    break;
    case 4380000 ... 4440000:
    offset = -2;
    break;
    default:
    return 0;
    }
    return ((voltage - 4100000) / 20000) + offset;
    }
    static void cpcap_charger_disconnect(struct cpcap_charger_ddata *ddata,
    int state, unsigned long delay)
    {
    int error;
// Update battery state before disconnecting the charger
    switch (state) {
    case POWER_SUPPLY_STATUS_DISCHARGING:
    case POWER_SUPPLY_STATUS_FULL:
    power_supply_changed(ddata.usb);
    break;
    default:
    break;
    }
    error = cpcap_charger_disable(ddata);
    if (error) {
    cpcap_charger_update_state(ddata, POWER_SUPPLY_STATUS_UNKNOWN);
    return;
    }
    cpcap_charger_update_state(ddata, state);
    power_supply_changed(ddata.usb);
    schedule_delayed_work(&ddata.detect_work, delay);
    }
#[no_mangle]
unsafe extern "C" fn cpcap_usb_detect(work: *mut work_struct) {
    static void cpcap_usb_detect(struct work_struct *work)
    {
    struct cpcap_charger_ddata *ddata;
    struct cpcap_charger_ints_state s;
    int error, new_state;
    ddata = container_of(work, struct cpcap_charger_ddata,
    detect_work.work);
    error = cpcap_charger_get_ints_state(ddata, &s);
    if (error)
    return;
// Just init the state if a charger is connected with no chrg_det set
    if (!s.chrg_det && s.chrgcurr1 && s.vbusvld) {
    cpcap_charger_update_state(ddata,
    POWER_SUPPLY_STATUS_NOT_CHARGING);
    return;
    }
//
// If battery voltage is higher than charge voltage, it may have been
// charged to 4.35V by Android. Try again in 10 minutes.
//
    if (cpcap_charger_get_charge_voltage(ddata) > ddata.voltage) {
    cpcap_charger_disconnect(ddata,
    POWER_SUPPLY_STATUS_NOT_CHARGING,
    HZ * 60 * 10);
    return;
    }
// Delay for 80ms to avoid vbus bouncing when usb cable is plugged in
    usleep_range(80000, 120000);
// Throttle chrgcurr2 interrupt for charger done and retry
    switch (ddata.status) {
    case POWER_SUPPLY_STATUS_CHARGING:
    if (s.chrgcurr2)
    break;
    new_state = POWER_SUPPLY_STATUS_FULL;
    if (s.chrgcurr1 && s.vbusvld) {
    cpcap_charger_disconnect(ddata, new_state, HZ * 5);
    return;
    }
    break;
    case POWER_SUPPLY_STATUS_FULL:
    if (!s.chrgcurr2)
    break;
    if (s.vbusvld)
    new_state = POWER_SUPPLY_STATUS_NOT_CHARGING;
    else
    new_state = POWER_SUPPLY_STATUS_DISCHARGING;
    cpcap_charger_disconnect(ddata, new_state, HZ * 5);
    return;
    default:
    break;
    }
    if (!ddata.feeding_vbus && cpcap_charger_vbus_valid(ddata) &&
    s.chrgcurr1) {
    int max_current;
    int vchrg, ichrg;
    union power_supply_propval val;
    struct power_supply *battery;
    battery = power_supply_get_by_name("battery");
    if (!battery) {
    dev_err(ddata.dev, "battery power_supply not available\n");
    return;
    }
    error = power_supply_get_property(battery, POWER_SUPPLY_PROP_PRESENT, &val);
    power_supply_put(battery);
    if (error)
    goto out_err;
    if (val.intval) {
    max_current = 1596000;
    } else {
    dev_info(ddata.dev, "battery not inserted, charging disabled\n");
    max_current = 0;
    }
    if (max_current > ddata.limit_current)
    max_current = ddata.limit_current;
    ichrg = cpcap_charger_current_to_regval(max_current);
    vchrg = cpcap_charger_voltage_to_regval(ddata.voltage);
    error = cpcap_charger_enable(ddata,
    CPCAP_REG_CRM_VCHRG(vchrg),
    ichrg, 0);
    if (error)
    goto out_err;
    cpcap_charger_update_state(ddata,
    POWER_SUPPLY_STATUS_CHARGING);
    } else {
    error = cpcap_charger_disable(ddata);
    if (error)
    goto out_err;
    cpcap_charger_update_state(ddata,
    POWER_SUPPLY_STATUS_DISCHARGING);
    }
    power_supply_changed(ddata.usb);
    return;
    out_err:
    cpcap_charger_update_state(ddata, POWER_SUPPLY_STATUS_UNKNOWN);
    dev_err(ddata.dev, "%s failed with %i\n", __func__, error);
    }
#[no_mangle]
unsafe extern "C" fn cpcap_charger_irq_thread(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t cpcap_charger_irq_thread(int irq, void *data)
    {
    struct cpcap_charger_ddata *ddata = data;
    if (!atomic_read(&ddata.active))
    return IRQ_NONE;
    schedule_delayed_work(&ddata.detect_work, 0);
    return IRQ_HANDLED;
    }
    static int cpcap_usb_init_irq(struct platform_device *pdev,
    struct cpcap_charger_ddata *ddata,
    const char *name)
    {
    struct cpcap_interrupt_desc *d;
    int irq, error;
    irq = platform_get_irq_byname(pdev, name);
    if (irq < 0)
    return -ENODEV;
    error = devm_request_threaded_irq(ddata.dev, irq, core::ptr::null_mut(),
    cpcap_charger_irq_thread,
    IRQF_SHARED | IRQF_ONESHOT,
    name, ddata);
    if (error)
    return error;
    d = devm_kzalloc(ddata.dev, sizeof(*d), GFP_KERNEL);
    if (!d)
    return -ENOMEM;
    d.name = name;
    d.irq = irq;
    list_add(&d.node, &ddata.irq_list);
    return 0;
    }
    static const char * const cpcap_charger_irqs[] = {
// REG_INT_0
    "chrg_det", "rvrs_chrg",
// REG_INT1
    "chrg_se1b", "se0conn", "rvrs_mode", "chrgcurr2", "chrgcurr1", "vbusvld",
// REG_INT_3
    "battdetb",
    };
    static int cpcap_usb_init_interrupts(struct platform_device *pdev,
    struct cpcap_charger_ddata *ddata)
    {
    int i, error;
    for (i = 0; i < ARRAY_SIZE(cpcap_charger_irqs); i++) {
    error = cpcap_usb_init_irq(pdev, ddata, cpcap_charger_irqs[i]);
    if (error)
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cpcap_charger_init_optional_gpios(ddata: *mut cpcap_charger_ddata) {
    static void cpcap_charger_init_optional_gpios(struct cpcap_charger_ddata *ddata)
    {
    int i;
    for (i = 0; i < 2; i++) {
    ddata.gpio[i] = devm_gpiod_get_index(ddata.dev, "mode",
    i, GPIOD_OUT_HIGH);
    if (IS_ERR(ddata.gpio[i])) {
    dev_info(ddata.dev, "no mode change GPIO%i: %li\n",
    i, PTR_ERR(ddata.gpio[i]));
    ddata.gpio[i] = core::ptr::null_mut();
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn cpcap_charger_init_iio(ddata: *mut cpcap_charger_ddata) -> c_int {
    static int cpcap_charger_init_iio(struct cpcap_charger_ddata *ddata)
    {
    const char * const names[CPCAP_CHARGER_IIO_NR] = {
    "battdetb", "battp", "vbus", "chg_isense", "batti",
    };
    int error, i;
    for (i = 0; i < CPCAP_CHARGER_IIO_NR; i++) {
    ddata.channels[i] = devm_iio_channel_get(ddata.dev,
    names[i]);
    if (IS_ERR(ddata.channels[i])) {
    error = PTR_ERR(ddata.channels[i]);
    goto out_err;
    }
    if (!ddata.channels[i].indio_dev) {
    error = -ENXIO;
    goto out_err;
    }
    }
    return 0;
    out_err:
    if (error != -EPROBE_DEFER)
    dev_err(ddata.dev, "could not initialize VBUS or ID IIO: %i\n",
    error);
    return error;
    }
    static char *cpcap_charger_supplied_to[] = {
    "battery",
    };
    static const struct power_supply_desc cpcap_charger_usb_desc = {
    .name		= "usb",
    .type		= POWER_SUPPLY_TYPE_USB,
    .properties	= cpcap_charger_props,
    .num_properties	= ARRAY_SIZE(cpcap_charger_props),
    .get_property	= cpcap_charger_get_property,
    .set_property	= cpcap_charger_set_property,
    .property_is_writeable = cpcap_charger_property_is_writeable,
    };
    static const struct of_device_id cpcap_charger_id_table[] = {
    {
    .compatible = "motorola,mapphone-cpcap-charger",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, cpcap_charger_id_table);
#[no_mangle]
unsafe extern "C" fn cpcap_charger_probe(pdev: *mut platform_device) -> c_int {
    static int cpcap_charger_probe(struct platform_device *pdev)
    {
    struct cpcap_charger_ddata *ddata;
    let mut psy_cfg: power_supply_config = {};
    int error;
    ddata = devm_kzalloc(&pdev.dev, sizeof(*ddata), GFP_KERNEL);
    if (!ddata)
    return -ENOMEM;
    ddata.dev = &pdev.dev;
    ddata.voltage = 4200000;
    ddata.limit_current = 532000;
    ddata.reg = dev_get_regmap(ddata.dev.parent, core::ptr::null_mut());
    if (!ddata.reg)
    return -ENODEV;
    INIT_LIST_HEAD(&ddata.irq_list);
    INIT_DELAYED_WORK(&ddata.detect_work, cpcap_usb_detect);
    INIT_DELAYED_WORK(&ddata.vbus_work, cpcap_charger_vbus_work);
    platform_set_drvdata(pdev, ddata);
    error = cpcap_charger_init_iio(ddata);
    if (error)
    return error;
    atomic_set(&ddata.active, 1);
    psy_cfg.fwnode = dev_fwnode(&pdev.dev);
    psy_cfg.drv_data = ddata;
    psy_cfg.supplied_to = cpcap_charger_supplied_to;
    psy_cfg.num_supplicants = ARRAY_SIZE(cpcap_charger_supplied_to);
    ddata.usb = devm_power_supply_register(ddata.dev,
    &cpcap_charger_usb_desc,
    &psy_cfg);
    if (IS_ERR(ddata.usb)) {
    error = PTR_ERR(ddata.usb);
    dev_err(ddata.dev, "failed to register USB charger: %i\n",
    error);
    return error;
    }
    error = cpcap_usb_init_interrupts(pdev, ddata);
    if (error)
    return error;
    ddata.comparator.set_vbus = cpcap_charger_set_vbus;
    error = omap_usb2_set_comparator(&ddata.comparator);
    if (error == -ENODEV) {
    dev_info(ddata.dev, "charger needs phy, deferring probe\n");
    return -EPROBE_DEFER;
    }
    cpcap_charger_init_optional_gpios(ddata);
    schedule_delayed_work(&ddata.detect_work, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cpcap_charger_shutdown(pdev: *mut platform_device) {
    static void cpcap_charger_shutdown(struct platform_device *pdev)
    {
    struct cpcap_charger_ddata *ddata = platform_get_drvdata(pdev);
    int error;
    atomic_set(&ddata.active, 0);
    error = omap_usb2_set_comparator(core::ptr::null_mut());
    if (error)
    dev_warn(ddata.dev, "could not clear USB comparator: %i\n",
    error);
    error = cpcap_charger_disable(ddata);
    if (error) {
    cpcap_charger_update_state(ddata, POWER_SUPPLY_STATUS_UNKNOWN);
    dev_warn(ddata.dev, "could not clear charger: %i\n",
    error);
    }
    cpcap_charger_update_state(ddata, POWER_SUPPLY_STATUS_DISCHARGING);
    cancel_delayed_work_sync(&ddata.vbus_work);
    cancel_delayed_work_sync(&ddata.detect_work);
    }
#[no_mangle]
unsafe extern "C" fn cpcap_charger_remove(pdev: *mut platform_device) {
    static void cpcap_charger_remove(struct platform_device *pdev)
    {
    cpcap_charger_shutdown(pdev);
    }
    static struct platform_driver cpcap_charger_driver = {
    .probe = cpcap_charger_probe,
    .driver	= {
    .name	= "cpcap-charger",
    .of_match_table = cpcap_charger_id_table,
    },
    .shutdown = cpcap_charger_shutdown,
    .remove = cpcap_charger_remove,
    };
    module_platform_driver(cpcap_charger_driver);
    MODULE_AUTHOR("Tony Lindgren <tony@atomide.com>");
    MODULE_DESCRIPTION("CPCAP Battery Charger Interface driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:cpcap-charger");
    MODULE_IMPORT_NS("IIO_CONSUMER");
