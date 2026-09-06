//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/88pm860x_charger.c
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
// Battery driver for Marvell 88PM860x PMIC
//
// Copyright (c) 2012 Marvell International Ltd.
// Author:	Jett Zhou <jtzhou@marvell.com>
// Haojian Zhuang <haojian.zhuang@marvell.com>
//

// bit definitions of Status Query Interface 2

// bit definitions of Reset Out Register

// bit definitions of PreReg 1

// bit definitions of Charger Control 1 Register

// bit definitions of Charger Control 2 Register

// bit definitions of Charger Control 3 Register

// bit definitions of Charger Control 4 Register

// bit definitions of Charger Control 6 Register

// bit definitions of Charger Control 7 Register

// bit definitions of Measurement Enable 1 Register

// bit definitions of Measurement Enable 3 Register

pub const FSM_INIT: c_int = 0;
pub const FSM_DISCHARGE: c_int = 1;
pub const FSM_PRECHARGE: c_int = 2;
pub const FSM_FASTCHARGE: c_int = 3;
pub const PRECHARGE_THRESHOLD: c_int = 3100;
pub const POWEROFF_THRESHOLD: c_int = 3400;
pub const CHARGE_THRESHOLD: c_int = 4000;
pub const DISCHARGE_THRESHOLD: c_int = 4180;
// over-temperature on PM8606 setting

// over-voltage protect on vchg setting mv
pub const VCHG_NORMAL_LOW: c_int = 4200;
pub const VCHG_NORMAL_CHECK: c_int = 5800;
pub const VCHG_NORMAL_HIGH: c_int = 6000;
pub const VCHG_OVP_LOW: c_int = 5500;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm860x_charger_info {
    pub chip: *mut pm860x_chip,
    pub i2c: *mut i2c_client,
    pub i2c_8606: *mut i2c_client,
    pub dev: *mut device,
    pub usb: *mut power_supply,
    pub lock: mutex,
    pub irq_nums: c_int,
    pub irq: [c_int; 7],
    pub /: *mut *mut unsigned state:3; / fsm state,
    pub /: *mut *mut unsigned online:1; / usb charger,
    pub /: *mut *mut unsigned present:1; / battery present,
    pub allowed:1: unsigned,
}

    static char *pm860x_supplied_to[] = {
    "battery-monitor",
    };
#[no_mangle]
unsafe extern "C" fn measure_vchg(info: *mut pm860x_charger_info, data: *mut c_int) -> c_int {
    static int measure_vchg(struct pm860x_charger_info *info, int *data)
    {
    unsigned char buf[2];
    let mut ret: c_int = 0;
    ret = pm860x_bulk_read(info.i2c, PM8607_VCHG_MEAS1, 2, buf);
    if (ret < 0)
    return ret;
// data = ((buf[0] & 0xff) << 4) | (buf[1] & 0x0f);
// V_BATT_MEAS(mV) = value * 5 * 1.8 * 1000 / (2^12)
// data = ((*data & 0xfff) * 9 * 125) >> 9;
    dev_dbg(info.dev, "%s, vchg: %d mv\n", __func__, *data);
    return ret;
    }
    static void set_vchg_threshold(struct pm860x_charger_info *info,
    int min, int max)
    {
    int data;
// (tmp << 8) * / 5 / 1800
    if (min <= 0)
    data = 0;
    else
    data = (min << 5) / 1125;
    pm860x_reg_write(info.i2c, PM8607_VCHG_LOWTH, data);
    dev_dbg(info.dev, "VCHG_LOWTH:%dmv, 0x%x\n", min, data);
    if (max <= 0)
    data = 0xff;
    else
    data = (max << 5) / 1125;
    pm860x_reg_write(info.i2c, PM8607_VCHG_HIGHTH, data);
    dev_dbg(info.dev, "VCHG_HIGHTH:%dmv, 0x%x\n", max, data);
    }
    static void set_vbatt_threshold(struct pm860x_charger_info *info,
    int min, int max)
    {
    int data;
// (tmp << 8) * 3 / 1800
    if (min <= 0)
    data = 0;
    else
    data = (min << 5) / 675;
    pm860x_reg_write(info.i2c, PM8607_VBAT_LOWTH, data);
    dev_dbg(info.dev, "VBAT Min:%dmv, LOWTH:0x%x\n", min, data);
    if (max <= 0)
    data = 0xff;
    else
    data = (max << 5) / 675;
    pm860x_reg_write(info.i2c, PM8607_VBAT_HIGHTH, data);
    dev_dbg(info.dev, "VBAT Max:%dmv, HIGHTH:0x%x\n", max, data);
    return;
    }
#[no_mangle]
unsafe extern "C" fn start_precharge(info: *mut pm860x_charger_info) -> c_int {
    static int start_precharge(struct pm860x_charger_info *info)
    {
    int ret;
    dev_dbg(info.dev, "Start Pre-charging!\n");
    set_vbatt_threshold(info, 0, 0);
    ret = pm860x_reg_write(info.i2c_8606, PM8606_PREREGULATORA,
    PREREG1_1350MA | PREREG1_VSYS_4_5V);
    if (ret < 0)
    goto out;
// stop charging
    ret = pm860x_set_bits(info.i2c, PM8607_CHG_CTRL1, 3,
    CC1_MODE_OFF);
    if (ret < 0)
    goto out;
// set 270 minutes timeout
    ret = pm860x_set_bits(info.i2c, PM8607_CHG_CTRL3, (0xf << 4),
    CC3_270MIN_TIMEOUT);
    if (ret < 0)
    goto out;
// set precharge current, termination voltage, IBAT & TBAT monitor
    ret = pm860x_reg_write(info.i2c, PM8607_CHG_CTRL4,
    CC4_IPRE_40MA | CC4_VPCHG_3_2V |
    CC4_IFCHG_MON_EN | CC4_BTEMP_MON_EN);
    if (ret < 0)
    goto out;
    ret = pm860x_set_bits(info.i2c, PM8607_CHG_CTRL7,
    CC7_BAT_REM_EN | CC7_IFSM_EN,
    CC7_BAT_REM_EN | CC7_IFSM_EN);
    if (ret < 0)
    goto out;
// trigger precharge
    ret = pm860x_set_bits(info.i2c, PM8607_CHG_CTRL1, 3,
    CC1_MODE_PRECHARGE);
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn start_fastcharge(info: *mut pm860x_charger_info) -> c_int {
    static int start_fastcharge(struct pm860x_charger_info *info)
    {
    int ret;
    dev_dbg(info.dev, "Start Fast-charging!\n");
// set fastcharge termination current & voltage, disable charging
    ret = pm860x_reg_write(info.i2c, PM8607_CHG_CTRL1,
    CC1_MODE_OFF | CC1_ITERM_60MA |
    CC1_VFCHG_4_2V);
    if (ret < 0)
    goto out;
    ret = pm860x_reg_write(info.i2c_8606, PM8606_PREREGULATORA,
    PREREG1_540MA | PREREG1_VSYS_4_5V);
    if (ret < 0)
    goto out;
    ret = pm860x_set_bits(info.i2c, PM8607_CHG_CTRL2, 0x1f,
    CC2_ICHG_500MA);
    if (ret < 0)
    goto out;
// set 270 minutes timeout
    ret = pm860x_set_bits(info.i2c, PM8607_CHG_CTRL3, (0xf << 4),
    CC3_270MIN_TIMEOUT);
    if (ret < 0)
    goto out;
// set IBAT & TBAT monitor
    ret = pm860x_set_bits(info.i2c, PM8607_CHG_CTRL4,
    CC4_IFCHG_MON_EN | CC4_BTEMP_MON_EN,
    CC4_IFCHG_MON_EN | CC4_BTEMP_MON_EN);
    if (ret < 0)
    goto out;
    ret = pm860x_set_bits(info.i2c, PM8607_CHG_CTRL6,
    CC6_BAT_OV_EN | CC6_BAT_UV_EN |
    CC6_UV_VBAT_SET,
    CC6_BAT_OV_EN | CC6_BAT_UV_EN |
    CC6_UV_VBAT_SET);
    if (ret < 0)
    goto out;
    ret = pm860x_set_bits(info.i2c, PM8607_CHG_CTRL7,
    CC7_BAT_REM_EN | CC7_IFSM_EN,
    CC7_BAT_REM_EN | CC7_IFSM_EN);
    if (ret < 0)
    goto out;
// launch fast-charge
    ret = pm860x_set_bits(info.i2c, PM8607_CHG_CTRL1, 3,
    CC1_MODE_FASTCHARGE);
// vchg threshold setting
    set_vchg_threshold(info, VCHG_NORMAL_LOW, VCHG_NORMAL_HIGH);
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stop_charge(info: *mut pm860x_charger_info, vbatt: c_int) {
    static void stop_charge(struct pm860x_charger_info *info, int vbatt)
    {
    dev_dbg(info.dev, "Stop charging!\n");
    pm860x_set_bits(info.i2c, PM8607_CHG_CTRL1, 3, CC1_MODE_OFF);
    if (vbatt > CHARGE_THRESHOLD && info.online)
    set_vbatt_threshold(info, CHARGE_THRESHOLD, 0);
    }
#[no_mangle]
unsafe extern "C" fn power_off_notification(info: *mut pm860x_charger_info) {
    static void power_off_notification(struct pm860x_charger_info *info)
    {
    dev_dbg(info.dev, "Power-off notification!\n");
    }
#[no_mangle]
unsafe extern "C" fn set_charging_fsm(info: *mut pm860x_charger_info) -> c_int {
    static int set_charging_fsm(struct pm860x_charger_info *info)
    {
    struct power_supply *psy;
    union power_supply_propval data;
    static const unsigned char fsm_state[][16] = {
    "init", "discharge", "precharge", "fastcharge",
    };
    int ret;
    int vbatt;
    psy = power_supply_get_by_name(pm860x_supplied_to[0]);
    if (!psy)
    return -EINVAL;
    ret = power_supply_get_property(psy, POWER_SUPPLY_PROP_VOLTAGE_NOW,
    &data);
    if (ret) {
    power_supply_put(psy);
    return ret;
    }
    vbatt = data.intval / 1000;
    ret = power_supply_get_property(psy, POWER_SUPPLY_PROP_PRESENT, &data);
    if (ret) {
    power_supply_put(psy);
    return ret;
    }
    power_supply_put(psy);
    mutex_lock(&info.lock);
    info.present = data.intval;
    dev_dbg(info.dev, "Entering FSM:%s, Charger:%s, Battery:%s, "
    "Allowed:%d\n",
    fsm_state[info.state],
    (info.online) ? "online" : "N/A",
    (info.present) ? "present" : "N/A", info.allowed);
    dev_dbg(info.dev, "set_charging_fsm:vbatt:%d(mV)\n", vbatt);
    switch (info.state) {
    case FSM_INIT:
    if (info.online && info.present && info.allowed) {
    if (vbatt < PRECHARGE_THRESHOLD) {
    info.state = FSM_PRECHARGE;
    start_precharge(info);
    } else if (vbatt > DISCHARGE_THRESHOLD) {
    info.state = FSM_DISCHARGE;
    stop_charge(info, vbatt);
    } else if (vbatt < DISCHARGE_THRESHOLD) {
    info.state = FSM_FASTCHARGE;
    start_fastcharge(info);
    }
    } else {
    if (vbatt < POWEROFF_THRESHOLD) {
    power_off_notification(info);
    } else {
    info.state = FSM_DISCHARGE;
    stop_charge(info, vbatt);
    }
    }
    break;
    case FSM_PRECHARGE:
    if (info.online && info.present && info.allowed) {
    if (vbatt > PRECHARGE_THRESHOLD) {
    info.state = FSM_FASTCHARGE;
    start_fastcharge(info);
    }
    } else {
    info.state = FSM_DISCHARGE;
    stop_charge(info, vbatt);
    }
    break;
    case FSM_FASTCHARGE:
    if (info.online && info.present && info.allowed) {
    if (vbatt < PRECHARGE_THRESHOLD) {
    info.state = FSM_PRECHARGE;
    start_precharge(info);
    }
    } else {
    info.state = FSM_DISCHARGE;
    stop_charge(info, vbatt);
    }
    break;
    case FSM_DISCHARGE:
    if (info.online && info.present && info.allowed) {
    if (vbatt < PRECHARGE_THRESHOLD) {
    info.state = FSM_PRECHARGE;
    start_precharge(info);
    } else if (vbatt < DISCHARGE_THRESHOLD) {
    info.state = FSM_FASTCHARGE;
    start_fastcharge(info);
    }
    } else {
    if (vbatt < POWEROFF_THRESHOLD)
    power_off_notification(info);
#[no_mangle]
pub unsafe extern "C" fn if(info->online: vbatt > CHARGE_THRESHOLD &&) -> else {
    else if (vbatt > CHARGE_THRESHOLD && info.online)
    set_vbatt_threshold(info, CHARGE_THRESHOLD, 0);
    }
    break;
    default:
    dev_warn(info.dev, "FSM meets wrong state:%d\n",
    info.state);
    break;
    }
    dev_dbg(info.dev,
    "Out FSM:%s, Charger:%s, Battery:%s, Allowed:%d\n",
    fsm_state[info.state],
    (info.online) ? "online" : "N/A",
    (info.present) ? "present" : "N/A", info.allowed);
    mutex_unlock(&info.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pm860x_charger_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t pm860x_charger_handler(int irq, void *data)
    {
    struct pm860x_charger_info *info = data;
    int ret;
    mutex_lock(&info.lock);
    ret = pm860x_reg_read(info.i2c, PM8607_STATUS_2);
    if (ret < 0) {
    mutex_unlock(&info.lock);
    goto out;
    }
    if (ret & STATUS2_CHG) {
    info.online = 1;
    info.allowed = 1;
    } else {
    info.online = 0;
    info.allowed = 0;
    }
    mutex_unlock(&info.lock);
    dev_dbg(info.dev, "%s, Charger:%s, Allowed:%d\n", __func__,
    (info.online) ? "online" : "N/A", info.allowed);
    set_charging_fsm(info);
    power_supply_changed(info.usb);
    out:
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn pm860x_temp_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t pm860x_temp_handler(int irq, void *data)
    {
    struct power_supply *psy;
    struct pm860x_charger_info *info = data;
    union power_supply_propval temp;
    int value;
    int ret;
    psy = power_supply_get_by_name(pm860x_supplied_to[0]);
    if (!psy)
    return IRQ_HANDLED;
    ret = power_supply_get_property(psy, POWER_SUPPLY_PROP_TEMP, &temp);
    if (ret)
    goto out;
    value = temp.intval / 10;
    mutex_lock(&info.lock);
// Temperature < -10 C or >40 C, Will not allow charge
    if (value < -10 || value > 40)
    info.allowed = 0;
    else
    info.allowed = 1;
    dev_dbg(info.dev, "%s, Allowed: %d\n", __func__, info.allowed);
    mutex_unlock(&info.lock);
    set_charging_fsm(info);
    out:
    power_supply_put(psy);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn pm860x_exception_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t pm860x_exception_handler(int irq, void *data)
    {
    struct pm860x_charger_info *info = data;
    mutex_lock(&info.lock);
    info.allowed = 0;
    mutex_unlock(&info.lock);
    dev_dbg(info.dev, "%s, irq: %d\n", __func__, irq);
    set_charging_fsm(info);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn pm860x_done_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t pm860x_done_handler(int irq, void *data)
    {
    struct pm860x_charger_info *info = data;
    struct power_supply *psy;
    union power_supply_propval val;
    int ret;
    int vbatt;
    mutex_lock(&info.lock);
// pre-charge done, will transimit to fast-charge stage
    if (info.state == FSM_PRECHARGE) {
    info.allowed = 1;
    goto out;
    }
//
// Fast charge done, delay to read
// the correct status of CHG_DET.
//
    mdelay(5);
    info.allowed = 0;
    psy = power_supply_get_by_name(pm860x_supplied_to[0]);
    if (!psy)
    goto out;
    ret = power_supply_get_property(psy, POWER_SUPPLY_PROP_VOLTAGE_NOW,
    &val);
    if (ret)
    goto out_psy_put;
    vbatt = val.intval / 1000;
//
// CHG_DONE interrupt is faster than CHG_DET interrupt when
// plug in/out usb, So we can not rely on info->online, we
// need check pm8607 status register to check usb is online
// or not, then we can decide it is real charge done
// automatically or it is triggered by usb plug out;
//
    ret = pm860x_reg_read(info.i2c, PM8607_STATUS_2);
    if (ret < 0)
    goto out_psy_put;
    if (vbatt > CHARGE_THRESHOLD && ret & STATUS2_CHG)
    power_supply_set_property(psy, POWER_SUPPLY_PROP_CHARGE_FULL,
    &val);
    out_psy_put:
    power_supply_put(psy);
    out:
    mutex_unlock(&info.lock);
    dev_dbg(info.dev, "%s, Allowed: %d\n", __func__, info.allowed);
    set_charging_fsm(info);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn pm860x_vbattery_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t pm860x_vbattery_handler(int irq, void *data)
    {
    struct pm860x_charger_info *info = data;
    mutex_lock(&info.lock);
    set_vbatt_threshold(info, 0, 0);
    if (info.present && info.online)
    info.allowed = 1;
    else
    info.allowed = 0;
    mutex_unlock(&info.lock);
    dev_dbg(info.dev, "%s, Allowed: %d\n", __func__, info.allowed);
    set_charging_fsm(info);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn pm860x_vchg_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t pm860x_vchg_handler(int irq, void *data)
    {
    struct pm860x_charger_info *info = data;
    let mut vchg: c_int = 0;
    if (info.present)
    goto out;
    measure_vchg(info, &vchg);
    mutex_lock(&info.lock);
    if (!info.online) {
    int status;
// check if over-temp on pm8606 or not
    status = pm860x_reg_read(info.i2c_8606, PM8606_FLAGS);
    if (status & OVER_TEMP_FLAG) {
// clear over temp flag and set auto recover
    pm860x_set_bits(info.i2c_8606, PM8606_FLAGS,
    OVER_TEMP_FLAG, OVER_TEMP_FLAG);
    pm860x_set_bits(info.i2c_8606,
    PM8606_VSYS,
    OVTEMP_AUTORECOVER,
    OVTEMP_AUTORECOVER);
    dev_dbg(info.dev,
    "%s, pm8606 over-temp occurred\n", __func__);
    }
    }
    if (vchg > VCHG_NORMAL_CHECK) {
    set_vchg_threshold(info, VCHG_OVP_LOW, 0);
    info.allowed = 0;
    dev_dbg(info.dev,
    "%s,pm8607 over-vchg occurred,vchg = %dmv\n",
    __func__, vchg);
    } else if (vchg < VCHG_OVP_LOW) {
    set_vchg_threshold(info, VCHG_NORMAL_LOW,
    VCHG_NORMAL_HIGH);
    info.allowed = 1;
    dev_dbg(info.dev,
    "%s,pm8607 over-vchg recover,vchg = %dmv\n",
    __func__, vchg);
    }
    mutex_unlock(&info.lock);
    dev_dbg(info.dev, "%s, Allowed: %d\n", __func__, info.allowed);
    set_charging_fsm(info);
    out:
    return IRQ_HANDLED;
    }
    static int pm860x_usb_get_prop(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct pm860x_charger_info *info = power_supply_get_drvdata(psy);
    switch (psp) {
    case POWER_SUPPLY_PROP_STATUS:
    if (info.state == FSM_FASTCHARGE ||
    info.state == FSM_PRECHARGE)
    val.intval = POWER_SUPPLY_STATUS_CHARGING;
    else
    val.intval = POWER_SUPPLY_STATUS_DISCHARGING;
    break;
    case POWER_SUPPLY_PROP_ONLINE:
    val.intval = info.online;
    break;
    default:
    return -ENODEV;
    }
    return 0;
    }
    static enum power_supply_property pm860x_usb_props[] = {
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_ONLINE,
    };
#[no_mangle]
unsafe extern "C" fn pm860x_init_charger(info: *mut pm860x_charger_info) -> c_int {
    static int pm860x_init_charger(struct pm860x_charger_info *info)
    {
    int ret;
    ret = pm860x_reg_read(info.i2c, PM8607_STATUS_2);
    if (ret < 0)
    return ret;
    mutex_lock(&info.lock);
    info.state = FSM_INIT;
    if (ret & STATUS2_CHG) {
    info.online = 1;
    info.allowed = 1;
    } else {
    info.online = 0;
    info.allowed = 0;
    }
    mutex_unlock(&info.lock);
    set_charging_fsm(info);
    return 0;
    }
    static struct pm860x_irq_desc {
    const char *name;
    irqreturn_t (*handler)(int irq, void *data);
    } pm860x_irq_descs[] = {
    { "usb supply detect", pm860x_charger_handler },
    { "charge done", pm860x_done_handler },
    { "charge timeout", pm860x_exception_handler },
    { "charge fault", pm860x_exception_handler },
    { "temperature", pm860x_temp_handler },
    { "vbatt", pm860x_vbattery_handler },
    { "vchg", pm860x_vchg_handler },
    };
    static const struct power_supply_desc pm860x_charger_desc = {
    .name		= "usb",
    .type		= POWER_SUPPLY_TYPE_USB,
    .properties	= pm860x_usb_props,
    .num_properties	= ARRAY_SIZE(pm860x_usb_props),
    .get_property	= pm860x_usb_get_prop,
    };
#[no_mangle]
unsafe extern "C" fn pm860x_charger_probe(pdev: *mut platform_device) -> c_int {
    static int pm860x_charger_probe(struct platform_device *pdev)
    {
    struct pm860x_chip *chip = dev_get_drvdata(pdev.dev.parent);
    let mut psy_cfg: power_supply_config = {};
    struct pm860x_charger_info *info;
    int ret;
    int count;
    int i;
    int j;
    info = devm_kzalloc(&pdev.dev, sizeof(*info), GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    count = pdev.num_resources;
    for (i = 0, j = 0; i < count; i++) {
    info.irq[j] = platform_get_irq(pdev, i);
    if (info.irq[j] < 0)
    continue;
    j++;
    }
    info.irq_nums = j;
    info.chip = chip;
    info.i2c =
    (chip.id == CHIP_PM8607) ? chip.client : chip.companion;
    info.i2c_8606 =
    (chip.id == CHIP_PM8607) ? chip.companion : chip.client;
    if (!info.i2c_8606) {
    dev_err(&pdev.dev, "Missed I2C address of 88PM8606!\n");
    return -EINVAL;
    }
    info.dev = &pdev.dev;
// set init value for the case we are not using battery
    set_vchg_threshold(info, VCHG_NORMAL_LOW, VCHG_OVP_LOW);
    mutex_init(&info.lock);
    platform_set_drvdata(pdev, info);
    psy_cfg.drv_data = info;
    psy_cfg.supplied_to = pm860x_supplied_to;
    psy_cfg.num_supplicants = ARRAY_SIZE(pm860x_supplied_to);
    info.usb = devm_power_supply_register(&pdev.dev, &pm860x_charger_desc,
    &psy_cfg);
    if (IS_ERR(info.usb)) {
    return PTR_ERR(info.usb);
    }
    pm860x_init_charger(info);
    for (i = 0; i < ARRAY_SIZE(info.irq); i++) {
    ret = devm_request_threaded_irq(&pdev.dev, info.irq[i], core::ptr::null_mut(),
    pm860x_irq_descs[i].handler,
    IRQF_ONESHOT,
    pm860x_irq_descs[i].name, info);
    if (ret < 0)
    return ret;
    }
    return 0;
    }
    static struct platform_driver pm860x_charger_driver = {
    .driver = {
    .name = "88pm860x-charger",
    },
    .probe = pm860x_charger_probe,
    };
    module_platform_driver(pm860x_charger_driver);
    MODULE_DESCRIPTION("Marvell 88PM860x Charger driver");
    MODULE_LICENSE("GPL");
