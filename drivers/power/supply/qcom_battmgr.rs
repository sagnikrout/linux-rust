//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/qcom_battmgr.c
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
// Copyright (c) 2019-2020, The Linux Foundation. All rights reserved.
// Copyright (c) 2022, Linaro Ltd
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

pub const BATTMGR_CHEMISTRY_LEN: c_int = 4;
pub const BATTMGR_STRING_LEN: c_int = 128;
    enum qcom_battmgr_variant {
    QCOM_BATTMGR_SC8280XP,
    QCOM_BATTMGR_SM8350,
    QCOM_BATTMGR_SM8550,
    QCOM_BATTMGR_X1E80100,
    };
pub const BATTMGR_BAT_STATUS: c_uint = 0x1;
pub const BATTMGR_REQUEST_NOTIFICATION: c_uint = 0x4;
pub const BATTMGR_NOTIFICATION: c_uint = 0x7;
pub const NOTIF_BAT_PROPERTY: c_uint = 0x30;
pub const NOTIF_USB_PROPERTY: c_uint = 0x32;
pub const NOTIF_WLS_PROPERTY: c_uint = 0x34;
pub const NOTIF_BAT_STATUS: c_uint = 0x80;
pub const NOTIF_BAT_INFO: c_uint = 0x81;
pub const NOTIF_BAT_CHARGING_STATE: c_uint = 0x83;
pub const BATTMGR_BAT_INFO: c_uint = 0x9;
pub const BATTMGR_BAT_DISCHARGE_TIME: c_uint = 0xc;
pub const BATTMGR_BAT_CHARGE_TIME: c_uint = 0xd;
pub const BATTMGR_BAT_PROPERTY_GET: c_uint = 0x30;
pub const BATTMGR_BAT_PROPERTY_SET: c_uint = 0x31;
pub const BATT_STATUS: c_int = 0;
pub const BATT_HEALTH: c_int = 1;
pub const BATT_PRESENT: c_int = 2;
pub const BATT_CHG_TYPE: c_int = 3;
pub const BATT_CAPACITY: c_int = 4;
pub const BATT_SOH: c_int = 5;
pub const BATT_VOLT_OCV: c_int = 6;
pub const BATT_VOLT_NOW: c_int = 7;
pub const BATT_VOLT_MAX: c_int = 8;
pub const BATT_CURR_NOW: c_int = 9;
pub const BATT_CHG_CTRL_LIM: c_int = 10;
pub const BATT_CHG_CTRL_LIM_MAX: c_int = 11;
pub const BATT_TEMP: c_int = 12;
pub const BATT_TECHNOLOGY: c_int = 13;
pub const BATT_CHG_COUNTER: c_int = 14;
pub const BATT_CYCLE_COUNT: c_int = 15;
pub const BATT_CHG_FULL_DESIGN: c_int = 16;
pub const BATT_CHG_FULL: c_int = 17;
pub const BATT_MODEL_NAME: c_int = 18;
pub const BATT_TTF_AVG: c_int = 19;
pub const BATT_TTE_AVG: c_int = 20;
pub const BATT_RESISTANCE: c_int = 21;
pub const BATT_POWER_NOW: c_int = 22;
pub const BATT_POWER_AVG: c_int = 23;
pub const BATT_CHG_CTRL_EN: c_int = 24;
pub const BATT_CHG_CTRL_START_THR: c_int = 25;
pub const BATT_CHG_CTRL_END_THR: c_int = 26;
pub const BATTMGR_USB_PROPERTY_GET: c_uint = 0x32;
pub const BATTMGR_USB_PROPERTY_SET: c_uint = 0x33;
pub const USB_ONLINE: c_int = 0;
pub const USB_VOLT_NOW: c_int = 1;
pub const USB_VOLT_MAX: c_int = 2;
pub const USB_CURR_NOW: c_int = 3;
pub const USB_CURR_MAX: c_int = 4;
pub const USB_INPUT_CURR_LIMIT: c_int = 5;
pub const USB_TYPE: c_int = 6;
pub const USB_ADAP_TYPE: c_int = 7;
pub const USB_MOISTURE_DET_EN: c_int = 8;
pub const USB_MOISTURE_DET_STS: c_int = 9;
pub const BATTMGR_WLS_PROPERTY_GET: c_uint = 0x34;
pub const BATTMGR_WLS_PROPERTY_SET: c_uint = 0x35;
pub const WLS_ONLINE: c_int = 0;
pub const WLS_VOLT_NOW: c_int = 1;
pub const WLS_VOLT_MAX: c_int = 2;
pub const WLS_CURR_NOW: c_int = 3;
pub const WLS_CURR_MAX: c_int = 4;
pub const WLS_TYPE: c_int = 5;
pub const WLS_BOOST_EN: c_int = 6;
pub const BATTMGR_CHG_CTRL_LIMIT_EN: c_uint = 0x48;
pub const CHARGE_CTRL_START_THR_MIN: c_int = 50;
pub const CHARGE_CTRL_START_THR_MAX: c_int = 95;
pub const CHARGE_CTRL_END_THR_MIN: c_int = 55;
pub const CHARGE_CTRL_END_THR_MAX: c_int = 100;
pub const CHARGE_CTRL_DELTA_SOC: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_battmgr_enable_request {
    pub hdr: pmic_glink_hdr,
    pub battery_id: __le32,
    pub power_state: __le32,
    pub low_capacity: __le32,
    pub high_capacity: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_battmgr_property_request {
    pub hdr: pmic_glink_hdr,
    pub battery: __le32,
    pub property: __le32,
    pub value: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_battmgr_update_request {
    pub hdr: pmic_glink_hdr,
    pub battery_id: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_battmgr_charge_time_request {
    pub hdr: pmic_glink_hdr,
    pub battery_id: __le32,
    pub percent: __le32,
    pub reserved: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_battmgr_discharge_time_request {
    pub hdr: pmic_glink_hdr,
    pub battery_id: __le32,
    pub /: *mut *mut __le32 rate; / 0 for current rate,
    pub reserved: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_battmgr_charge_ctrl_request {
    pub hdr: pmic_glink_hdr,
    pub enable: __le32,
    pub target_soc: __le32,
    pub delta_soc: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_battmgr_message {
    pub hdr: pmic_glink_hdr,
    union {
    struct {
    pub property: __le32,
    pub value: __le32,
    pub result: __le32,
    pub intval: },
    struct {
    pub property: __le32,
    pub model: [c_char; BATTMGR_STRING_LEN],
    pub strval: },
    struct {
//
// 0: mWh
// 1: mAh
//
    pub power_unit: __le32,
    pub design_capacity: __le32,
    pub last_full_capacity: __le32,
//
// 0 nonrechargable
// 1 rechargable
//
    pub battery_tech: __le32,
    pub /: *mut *mut __le32 design_voltage; / mV,
    pub capacity_low: __le32,
    pub capacity_warning: __le32,
    pub cycle_count: __le32,
// thousandth of percent
    pub accuracy: __le32,
    pub max_sample_time_ms: __le32,
    pub min_sample_time_ms: __le32,
    pub max_average_interval_ms: __le32,
    pub min_average_interval_ms: __le32,
// granularity between low and warning
    pub capacity_granularity1: __le32,
// granularity between warning and full
    pub capacity_granularity2: __le32,
//
// 0: no
// 1: cold
// 2: hot
//
    pub swappable: __le32,
    pub capabilities: __le32,
    pub model_number: [c_char; BATTMGR_STRING_LEN],
    pub serial_number: [c_char; BATTMGR_STRING_LEN],
    pub battery_type: [c_char; BATTMGR_STRING_LEN],
    pub oem_info: [c_char; BATTMGR_STRING_LEN],
    pub battery_chemistry: [c_char; BATTMGR_CHEMISTRY_LEN],
    pub uid: [c_char; BATTMGR_STRING_LEN],
    pub critical_bias: __le32,
    pub day: u8,
    pub month: u8,
    pub year: __le16,
    pub battery_id: __le32,
    pub info: },
    struct {
//
// BIT(0) discharging
// BIT(1) charging
// BIT(2) critical low
//
    pub battery_state: __le32,
// mWh or mAh, based on info->power_unit
    pub capacity: __le32,
    pub rate: __le32,
// mv
    pub battery_voltage: __le32,
//
// BIT(0) power online
// BIT(1) discharging
// BIT(2) charging
// BIT(3) battery critical
//
    pub power_state: __le32,
//
// 1: AC
// 2: USB
// 3: Wireless
//
    pub charging_source: __le32,
    pub temperature: __le32,
    pub status: },
    pub time: __le32,
    pub notification: __le32,
}

    };
pub const BATTMGR_CHARGING_SOURCE_AC: c_int = 1;
pub const BATTMGR_CHARGING_SOURCE_USB: c_int = 2;
pub const BATTMGR_CHARGING_SOURCE_WIRELESS: c_int = 3;
    enum qcom_battmgr_unit {
    QCOM_BATTMGR_UNIT_mWh = 0,
    QCOM_BATTMGR_UNIT_mAh = 1
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_battmgr_info {
    pub valid: bool,
    pub present: bool,
    pub charge_type: c_uint,
    pub design_capacity: c_uint,
    pub last_full_capacity: c_uint,
    pub voltage_max_design: c_uint,
    pub voltage_max: c_uint,
    pub capacity_low: c_uint,
    pub capacity_warning: c_uint,
    pub cycle_count: c_uint,
    pub charge_count: c_uint,
    pub charge_ctrl_start: c_uint,
    pub charge_ctrl_end: c_uint,
    pub model_number: [c_char; BATTMGR_STRING_LEN],
    pub serial_number: [c_char; BATTMGR_STRING_LEN],
    pub oem_info: [c_char; BATTMGR_STRING_LEN],
    pub technology: c_uchar,
    pub day: c_uchar,
    pub month: c_uchar,
    pub year: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_battmgr_status {
    pub status: c_uint,
    pub health: c_uint,
    pub capacity: c_uint,
    pub percent: c_uint,
    pub current_now: c_int,
    pub power_now: c_int,
    pub voltage_now: c_uint,
    pub voltage_ocv: c_uint,
    pub temperature: c_uint,
    pub resistance: c_uint,
    pub soh_percent: c_uint,
    pub discharge_time: c_uint,
    pub charge_time: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_battmgr_ac {
    pub online: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_battmgr_usb {
    pub online: bool,
    pub voltage_now: c_uint,
    pub voltage_max: c_uint,
    pub current_now: c_uint,
    pub current_max: c_uint,
    pub current_limit: c_uint,
    pub usb_type: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_battmgr_wireless {
    pub online: bool,
    pub voltage_now: c_uint,
    pub voltage_max: c_uint,
    pub current_now: c_uint,
    pub current_max: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_battmgr {
    pub dev: *mut device,
    pub client: *mut pmic_glink_client,
    pub variant: enum qcom_battmgr_variant,
    pub ac_psy: *mut power_supply,
    pub bat_psy: *mut power_supply,
    pub usb_psy: *mut power_supply,
    pub wls_psy: *mut power_supply,
    pub unit: enum qcom_battmgr_unit,
    pub error: c_int,
    pub ack: completion,
    pub service_up: bool,
    pub info: qcom_battmgr_info,
    pub status: qcom_battmgr_status,
    pub ac: qcom_battmgr_ac,
    pub usb: qcom_battmgr_usb,
    pub wireless: qcom_battmgr_wireless,
    pub enable_work: work_struct,
//
// @lock is used to prevent concurrent power supply requests to the
// firmware, as it then stops responding.
//
    pub lock: mutex,
}

#[no_mangle]
unsafe extern "C" fn qcom_battmgr_request(battmgr: *mut qcom_battmgr, data: *mut c_void, len: usize) -> c_int {
    static int qcom_battmgr_request(struct qcom_battmgr *battmgr, void *data, size_t len)
    {
    unsigned long left;
    int ret;
    reinit_completion(&battmgr.ack);
    battmgr.error = 0;
    ret = pmic_glink_send(battmgr.client, data, len);
    if (ret < 0)
    return ret;
    left = wait_for_completion_timeout(&battmgr.ack, HZ);
    if (!left)
    return -ETIMEDOUT;
    return battmgr.error;
    }
    static int qcom_battmgr_request_property(struct qcom_battmgr *battmgr, int opcode,
    int property, u32 value)
    {
    struct qcom_battmgr_property_request request = {
    .hdr.owner = cpu_to_le32(PMIC_GLINK_OWNER_BATTMGR),
    .hdr.type = cpu_to_le32(PMIC_GLINK_REQ_RESP),
    .hdr.opcode = cpu_to_le32(opcode),
    .battery = cpu_to_le32(0),
    .property = cpu_to_le32(property),
    .value = cpu_to_le32(value),
    };
    return qcom_battmgr_request(battmgr, &request, sizeof(request));
    }
#[no_mangle]
unsafe extern "C" fn qcom_battmgr_update_status(battmgr: *mut qcom_battmgr) -> c_int {
    static int qcom_battmgr_update_status(struct qcom_battmgr *battmgr)
    {
    struct qcom_battmgr_update_request request = {
    .hdr.owner = cpu_to_le32(PMIC_GLINK_OWNER_BATTMGR),
    .hdr.type = cpu_to_le32(PMIC_GLINK_REQ_RESP),
    .hdr.opcode = cpu_to_le32(BATTMGR_BAT_STATUS),
    .battery_id = cpu_to_le32(0),
    };
    return qcom_battmgr_request(battmgr, &request, sizeof(request));
    }
#[no_mangle]
unsafe extern "C" fn qcom_battmgr_update_info(battmgr: *mut qcom_battmgr) -> c_int {
    static int qcom_battmgr_update_info(struct qcom_battmgr *battmgr)
    {
    struct qcom_battmgr_update_request request = {
    .hdr.owner = cpu_to_le32(PMIC_GLINK_OWNER_BATTMGR),
    .hdr.type = cpu_to_le32(PMIC_GLINK_REQ_RESP),
    .hdr.opcode = cpu_to_le32(BATTMGR_BAT_INFO),
    .battery_id = cpu_to_le32(0),
    };
    return qcom_battmgr_request(battmgr, &request, sizeof(request));
    }
#[no_mangle]
unsafe extern "C" fn qcom_battmgr_update_charge_time(battmgr: *mut qcom_battmgr) -> c_int {
    static int qcom_battmgr_update_charge_time(struct qcom_battmgr *battmgr)
    {
    struct qcom_battmgr_charge_time_request request = {
    .hdr.owner = cpu_to_le32(PMIC_GLINK_OWNER_BATTMGR),
    .hdr.type = cpu_to_le32(PMIC_GLINK_REQ_RESP),
    .hdr.opcode = cpu_to_le32(BATTMGR_BAT_CHARGE_TIME),
    .battery_id = cpu_to_le32(0),
    .percent = cpu_to_le32(100),
    };
    return qcom_battmgr_request(battmgr, &request, sizeof(request));
    }
#[no_mangle]
unsafe extern "C" fn qcom_battmgr_update_discharge_time(battmgr: *mut qcom_battmgr) -> c_int {
    static int qcom_battmgr_update_discharge_time(struct qcom_battmgr *battmgr)
    {
    struct qcom_battmgr_discharge_time_request request = {
    .hdr.owner = cpu_to_le32(PMIC_GLINK_OWNER_BATTMGR),
    .hdr.type = cpu_to_le32(PMIC_GLINK_REQ_RESP),
    .hdr.opcode = cpu_to_le32(BATTMGR_BAT_DISCHARGE_TIME),
    .battery_id = cpu_to_le32(0),
    .rate = cpu_to_le32(0),
    };
    return qcom_battmgr_request(battmgr, &request, sizeof(request));
    }
    static const u8 sm8350_bat_prop_map[] = {
    [POWER_SUPPLY_PROP_STATUS] = BATT_STATUS,
    [POWER_SUPPLY_PROP_HEALTH] = BATT_HEALTH,
    [POWER_SUPPLY_PROP_PRESENT] = BATT_PRESENT,
    [POWER_SUPPLY_PROP_CHARGE_TYPE] = BATT_CHG_TYPE,
    [POWER_SUPPLY_PROP_CAPACITY] = BATT_CAPACITY,
    [POWER_SUPPLY_PROP_VOLTAGE_OCV] = BATT_VOLT_OCV,
    [POWER_SUPPLY_PROP_VOLTAGE_NOW] = BATT_VOLT_NOW,
    [POWER_SUPPLY_PROP_VOLTAGE_MAX] = BATT_VOLT_MAX,
    [POWER_SUPPLY_PROP_CURRENT_NOW] = BATT_CURR_NOW,
    [POWER_SUPPLY_PROP_TEMP] = BATT_TEMP,
    [POWER_SUPPLY_PROP_TECHNOLOGY] = BATT_TECHNOLOGY,
    [POWER_SUPPLY_PROP_CHARGE_COUNTER] =  BATT_CHG_COUNTER,
    [POWER_SUPPLY_PROP_CYCLE_COUNT] = BATT_CYCLE_COUNT,
    [POWER_SUPPLY_PROP_CHARGE_FULL_DESIGN] =  BATT_CHG_FULL_DESIGN,
    [POWER_SUPPLY_PROP_CHARGE_FULL] = BATT_CHG_FULL,
    [POWER_SUPPLY_PROP_MODEL_NAME] = BATT_MODEL_NAME,
    [POWER_SUPPLY_PROP_TIME_TO_FULL_AVG] = BATT_TTF_AVG,
    [POWER_SUPPLY_PROP_TIME_TO_EMPTY_AVG] = BATT_TTE_AVG,
    [POWER_SUPPLY_PROP_INTERNAL_RESISTANCE] = BATT_RESISTANCE,
    [POWER_SUPPLY_PROP_STATE_OF_HEALTH] = BATT_SOH,
    [POWER_SUPPLY_PROP_POWER_NOW] = BATT_POWER_NOW,
    [POWER_SUPPLY_PROP_CHARGE_CONTROL_START_THRESHOLD] = BATT_CHG_CTRL_START_THR,
    [POWER_SUPPLY_PROP_CHARGE_CONTROL_END_THRESHOLD] = BATT_CHG_CTRL_END_THR,
    };
    static int qcom_battmgr_bat_sm8350_update(struct qcom_battmgr *battmgr,
    enum power_supply_property psp)
    {
    unsigned int prop;
    int ret;
    if (psp >= ARRAY_SIZE(sm8350_bat_prop_map))
    return -EINVAL;
    prop = sm8350_bat_prop_map[psp];
    mutex_lock(&battmgr.lock);
    ret = qcom_battmgr_request_property(battmgr, BATTMGR_BAT_PROPERTY_GET, prop, 0);
    mutex_unlock(&battmgr.lock);
    return ret;
    }
    static int qcom_battmgr_bat_sc8280xp_update(struct qcom_battmgr *battmgr,
    enum power_supply_property psp)
    {
    int ret;
    mutex_lock(&battmgr.lock);
    if (!battmgr.info.valid) {
    ret = qcom_battmgr_update_info(battmgr);
    if (ret < 0)
    goto out_unlock;
    battmgr.info.valid = true;
    }
    ret = qcom_battmgr_update_status(battmgr);
    if (ret < 0)
    goto out_unlock;
    if (psp == POWER_SUPPLY_PROP_TIME_TO_FULL_AVG) {
    ret = qcom_battmgr_update_charge_time(battmgr);
    if (ret < 0) {
    ret = -ENODATA;
    goto out_unlock;
    }
    }
    if (psp == POWER_SUPPLY_PROP_TIME_TO_EMPTY_AVG) {
    ret = qcom_battmgr_update_discharge_time(battmgr);
    if (ret < 0) {
    ret = -ENODATA;
    goto out_unlock;
    }
    }
    out_unlock:
    mutex_unlock(&battmgr.lock);
    return ret;
    }
    static int qcom_battmgr_bat_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct qcom_battmgr *battmgr = power_supply_get_drvdata(psy);
    let mut unit: enum qcom_battmgr_unit = battmgr.unit;
    int ret;
    if (!battmgr.service_up)
    return -EAGAIN;
    if (battmgr.variant == QCOM_BATTMGR_SC8280XP ||
    battmgr.variant == QCOM_BATTMGR_X1E80100)
    ret = qcom_battmgr_bat_sc8280xp_update(battmgr, psp);
    else
    ret = qcom_battmgr_bat_sm8350_update(battmgr, psp);
    if (ret < 0)
    return ret;
    switch (psp) {
    case POWER_SUPPLY_PROP_STATUS:
    val.intval = battmgr.status.status;
    break;
    case POWER_SUPPLY_PROP_CHARGE_TYPE:
    val.intval = battmgr.info.charge_type;
    break;
    case POWER_SUPPLY_PROP_HEALTH:
    val.intval = battmgr.status.health;
    break;
    case POWER_SUPPLY_PROP_PRESENT:
    val.intval = battmgr.info.present;
    break;
    case POWER_SUPPLY_PROP_TECHNOLOGY:
    val.intval = battmgr.info.technology;
    break;
    case POWER_SUPPLY_PROP_CYCLE_COUNT:
    val.intval = battmgr.info.cycle_count;
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_MAX_DESIGN:
    val.intval = battmgr.info.voltage_max_design;
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_MAX:
    val.intval = battmgr.info.voltage_max;
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_NOW:
    val.intval = battmgr.status.voltage_now;
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_OCV:
    val.intval = battmgr.status.voltage_ocv;
    break;
    case POWER_SUPPLY_PROP_CURRENT_NOW:
    val.intval = battmgr.status.current_now;
    break;
    case POWER_SUPPLY_PROP_POWER_NOW:
    val.intval = battmgr.status.power_now;
    break;
    case POWER_SUPPLY_PROP_CHARGE_FULL_DESIGN:
    if (unit != QCOM_BATTMGR_UNIT_mAh)
    return -ENODATA;
    val.intval = battmgr.info.design_capacity;
    break;
    case POWER_SUPPLY_PROP_CHARGE_FULL:
    if (unit != QCOM_BATTMGR_UNIT_mAh)
    return -ENODATA;
    val.intval = battmgr.info.last_full_capacity;
    break;
    case POWER_SUPPLY_PROP_CHARGE_EMPTY:
    if (unit != QCOM_BATTMGR_UNIT_mAh)
    return -ENODATA;
    val.intval = battmgr.info.capacity_low;
    break;
    case POWER_SUPPLY_PROP_CHARGE_NOW:
    if (unit != QCOM_BATTMGR_UNIT_mAh)
    return -ENODATA;
    val.intval = battmgr.status.capacity;
    break;
    case POWER_SUPPLY_PROP_CHARGE_COUNTER:
    val.intval = battmgr.info.charge_count;
    break;
    case POWER_SUPPLY_PROP_ENERGY_FULL_DESIGN:
    if (unit != QCOM_BATTMGR_UNIT_mWh)
    return -ENODATA;
    val.intval = battmgr.info.design_capacity;
    break;
    case POWER_SUPPLY_PROP_ENERGY_FULL:
    if (unit != QCOM_BATTMGR_UNIT_mWh)
    return -ENODATA;
    val.intval = battmgr.info.last_full_capacity;
    break;
    case POWER_SUPPLY_PROP_ENERGY_EMPTY:
    if (unit != QCOM_BATTMGR_UNIT_mWh)
    return -ENODATA;
    val.intval = battmgr.info.capacity_low;
    break;
    case POWER_SUPPLY_PROP_ENERGY_NOW:
    if (unit != QCOM_BATTMGR_UNIT_mWh)
    return -ENODATA;
    val.intval = battmgr.status.capacity;
    break;
    case POWER_SUPPLY_PROP_CAPACITY:
    if (battmgr.status.percent == (unsigned int)-1)
    return -ENODATA;
    val.intval = battmgr.status.percent;
    break;
    case POWER_SUPPLY_PROP_TEMP:
    val.intval = battmgr.status.temperature;
    break;
    case POWER_SUPPLY_PROP_INTERNAL_RESISTANCE:
    val.intval = battmgr.status.resistance;
    break;
    case POWER_SUPPLY_PROP_STATE_OF_HEALTH:
    val.intval = battmgr.status.soh_percent;
    break;
    case POWER_SUPPLY_PROP_TIME_TO_EMPTY_AVG:
    val.intval = battmgr.status.discharge_time;
    break;
    case POWER_SUPPLY_PROP_TIME_TO_FULL_AVG:
    val.intval = battmgr.status.charge_time;
    break;
    case POWER_SUPPLY_PROP_CHARGE_CONTROL_START_THRESHOLD:
    val.intval = battmgr.info.charge_ctrl_start;
    break;
    case POWER_SUPPLY_PROP_CHARGE_CONTROL_END_THRESHOLD:
    val.intval = battmgr.info.charge_ctrl_end;
    break;
    case POWER_SUPPLY_PROP_MANUFACTURE_YEAR:
    val.intval = battmgr.info.year;
    break;
    case POWER_SUPPLY_PROP_MANUFACTURE_MONTH:
    val.intval = battmgr.info.month;
    break;
    case POWER_SUPPLY_PROP_MANUFACTURE_DAY:
    val.intval = battmgr.info.day;
    break;
    case POWER_SUPPLY_PROP_MODEL_NAME:
    val.strval = battmgr.info.model_number;
    break;
    case POWER_SUPPLY_PROP_MANUFACTURER:
    val.strval = battmgr.info.oem_info;
    break;
    case POWER_SUPPLY_PROP_SERIAL_NUMBER:
    val.strval = battmgr.info.serial_number;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int qcom_battmgr_set_charge_control(struct qcom_battmgr *battmgr,
    u32 target_soc, u32 delta_soc)
    {
    struct qcom_battmgr_charge_ctrl_request request = {
    .hdr.owner = cpu_to_le32(PMIC_GLINK_OWNER_BATTMGR),
    .hdr.type = cpu_to_le32(PMIC_GLINK_REQ_RESP),
    .hdr.opcode = cpu_to_le32(BATTMGR_CHG_CTRL_LIMIT_EN),
    .enable = cpu_to_le32(1),
    .target_soc = cpu_to_le32(target_soc),
    .delta_soc = cpu_to_le32(delta_soc),
    };
    return qcom_battmgr_request(battmgr, &request, sizeof(request));
    }
#[no_mangle]
unsafe extern "C" fn qcom_battmgr_set_charge_start_threshold(battmgr: *mut qcom_battmgr, start_soc: c_int) -> c_int {
    static int qcom_battmgr_set_charge_start_threshold(struct qcom_battmgr *battmgr, int start_soc)
    {
    u32 target_soc, delta_soc;
    int ret;
    start_soc = clamp(start_soc, CHARGE_CTRL_START_THR_MIN, CHARGE_CTRL_START_THR_MAX);
//
// If the new start threshold is larger than the old end threshold,
// move the end threshold one step (DELTA_SOC) after the new start
// threshold.
//
    if (start_soc > battmgr.info.charge_ctrl_end) {
    target_soc = start_soc + CHARGE_CTRL_DELTA_SOC;
    target_soc = min_t(u32, target_soc, CHARGE_CTRL_END_THR_MAX);
    delta_soc = target_soc - start_soc;
    delta_soc = min_t(u32, delta_soc, CHARGE_CTRL_DELTA_SOC);
    } else {
    target_soc =  battmgr.info.charge_ctrl_end;
    delta_soc = battmgr.info.charge_ctrl_end - start_soc;
    }
    mutex_lock(&battmgr.lock);
    ret = qcom_battmgr_set_charge_control(battmgr, target_soc, delta_soc);
    mutex_unlock(&battmgr.lock);
    if (!ret) {
    battmgr.info.charge_ctrl_start = start_soc;
    battmgr.info.charge_ctrl_end = target_soc;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_battmgr_set_charge_end_threshold(battmgr: *mut qcom_battmgr, end_soc: c_int) -> c_int {
    static int qcom_battmgr_set_charge_end_threshold(struct qcom_battmgr *battmgr, int end_soc)
    {
    let mut delta_soc: u32 = CHARGE_CTRL_DELTA_SOC;
    int ret;
    end_soc = clamp(end_soc, CHARGE_CTRL_END_THR_MIN, CHARGE_CTRL_END_THR_MAX);
    if (battmgr.info.charge_ctrl_start && end_soc > battmgr.info.charge_ctrl_start)
    delta_soc = end_soc - battmgr.info.charge_ctrl_start;
    mutex_lock(&battmgr.lock);
    ret = qcom_battmgr_set_charge_control(battmgr, end_soc, delta_soc);
    mutex_unlock(&battmgr.lock);
    if (!ret) {
    battmgr.info.charge_ctrl_start = end_soc - delta_soc;
    battmgr.info.charge_ctrl_end = end_soc;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_battmgr_charge_control_thresholds_init(battmgr: *mut qcom_battmgr) -> c_int {
    static int qcom_battmgr_charge_control_thresholds_init(struct qcom_battmgr *battmgr)
    {
    int ret;
    u8 en, end_soc, start_soc, delta_soc;
    ret = nvmem_cell_read_u8(battmgr.dev.parent, "charge_limit_en", &en);
    if (!ret && en != 0) {
    ret = nvmem_cell_read_u8(battmgr.dev.parent, "charge_limit_end", &end_soc);
    if (ret < 0)
    return ret;
    ret = nvmem_cell_read_u8(battmgr.dev.parent, "charge_limit_delta", &delta_soc);
    if (ret < 0)
    return ret;
    if (delta_soc >= end_soc)
    return -EINVAL;
    start_soc = end_soc - delta_soc;
    end_soc = clamp(end_soc, CHARGE_CTRL_END_THR_MIN, CHARGE_CTRL_END_THR_MAX);
    start_soc = clamp(start_soc, CHARGE_CTRL_START_THR_MIN, CHARGE_CTRL_START_THR_MAX);
    battmgr.info.charge_ctrl_start = start_soc;
    battmgr.info.charge_ctrl_end = end_soc;
    }
    return 0;
    }
    static int qcom_battmgr_bat_is_writeable(struct power_supply *psy,
    enum power_supply_property psp)
    {
    switch (psp) {
    case POWER_SUPPLY_PROP_CHARGE_CONTROL_START_THRESHOLD:
    case POWER_SUPPLY_PROP_CHARGE_CONTROL_END_THRESHOLD:
    return 1;
    default:
    return 0;
    }
    return 0;
    }
    static int qcom_battmgr_bat_set_property(struct power_supply *psy,
    enum power_supply_property psp,
    const union power_supply_propval *pval)
    {
    struct qcom_battmgr *battmgr = power_supply_get_drvdata(psy);
    if (!battmgr.service_up)
    return -EAGAIN;
    switch (psp) {
    case POWER_SUPPLY_PROP_CHARGE_CONTROL_START_THRESHOLD:
    return qcom_battmgr_set_charge_start_threshold(battmgr, pval.intval);
    case POWER_SUPPLY_PROP_CHARGE_CONTROL_END_THRESHOLD:
    return qcom_battmgr_set_charge_end_threshold(battmgr, pval.intval);
    default:
    return -EINVAL;
    }
    return 0;
    }
    static const enum power_supply_property sc8280xp_bat_props[] = {
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_PRESENT,
    POWER_SUPPLY_PROP_TECHNOLOGY,
    POWER_SUPPLY_PROP_CAPACITY,
    POWER_SUPPLY_PROP_CYCLE_COUNT,
    POWER_SUPPLY_PROP_VOLTAGE_MAX_DESIGN,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    POWER_SUPPLY_PROP_POWER_NOW,
    POWER_SUPPLY_PROP_CHARGE_FULL_DESIGN,
    POWER_SUPPLY_PROP_CHARGE_FULL,
    POWER_SUPPLY_PROP_CHARGE_EMPTY,
    POWER_SUPPLY_PROP_CHARGE_NOW,
    POWER_SUPPLY_PROP_ENERGY_FULL_DESIGN,
    POWER_SUPPLY_PROP_ENERGY_FULL,
    POWER_SUPPLY_PROP_ENERGY_EMPTY,
    POWER_SUPPLY_PROP_ENERGY_NOW,
    POWER_SUPPLY_PROP_TEMP,
    POWER_SUPPLY_PROP_MANUFACTURE_YEAR,
    POWER_SUPPLY_PROP_MANUFACTURE_MONTH,
    POWER_SUPPLY_PROP_MANUFACTURE_DAY,
    POWER_SUPPLY_PROP_MODEL_NAME,
    POWER_SUPPLY_PROP_MANUFACTURER,
    POWER_SUPPLY_PROP_SERIAL_NUMBER,
    };
    static const struct power_supply_desc sc8280xp_bat_psy_desc = {
    .name = "qcom-battmgr-bat",
    .type = POWER_SUPPLY_TYPE_BATTERY,
    .properties = sc8280xp_bat_props,
    .num_properties = ARRAY_SIZE(sc8280xp_bat_props),
    .get_property = qcom_battmgr_bat_get_property,
    };
    static const enum power_supply_property x1e80100_bat_props[] = {
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_PRESENT,
    POWER_SUPPLY_PROP_TECHNOLOGY,
    POWER_SUPPLY_PROP_CYCLE_COUNT,
    POWER_SUPPLY_PROP_VOLTAGE_MAX_DESIGN,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    POWER_SUPPLY_PROP_POWER_NOW,
    POWER_SUPPLY_PROP_CHARGE_FULL_DESIGN,
    POWER_SUPPLY_PROP_CHARGE_FULL,
    POWER_SUPPLY_PROP_CHARGE_EMPTY,
    POWER_SUPPLY_PROP_CHARGE_NOW,
    POWER_SUPPLY_PROP_ENERGY_FULL_DESIGN,
    POWER_SUPPLY_PROP_ENERGY_FULL,
    POWER_SUPPLY_PROP_ENERGY_EMPTY,
    POWER_SUPPLY_PROP_ENERGY_NOW,
    POWER_SUPPLY_PROP_TEMP,
    POWER_SUPPLY_PROP_MANUFACTURE_YEAR,
    POWER_SUPPLY_PROP_MANUFACTURE_MONTH,
    POWER_SUPPLY_PROP_MANUFACTURE_DAY,
    POWER_SUPPLY_PROP_MODEL_NAME,
    POWER_SUPPLY_PROP_MANUFACTURER,
    POWER_SUPPLY_PROP_SERIAL_NUMBER,
    POWER_SUPPLY_PROP_CHARGE_CONTROL_START_THRESHOLD,
    POWER_SUPPLY_PROP_CHARGE_CONTROL_END_THRESHOLD,
    };
    static const struct power_supply_desc x1e80100_bat_psy_desc = {
    .name = "qcom-battmgr-bat",
    .type = POWER_SUPPLY_TYPE_BATTERY,
    .properties = x1e80100_bat_props,
    .num_properties = ARRAY_SIZE(x1e80100_bat_props),
    .get_property = qcom_battmgr_bat_get_property,
    .set_property = qcom_battmgr_bat_set_property,
    .property_is_writeable = qcom_battmgr_bat_is_writeable,
    };
    static const enum power_supply_property sm8350_bat_props[] = {
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_HEALTH,
    POWER_SUPPLY_PROP_PRESENT,
    POWER_SUPPLY_PROP_CHARGE_TYPE,
    POWER_SUPPLY_PROP_CAPACITY,
    POWER_SUPPLY_PROP_VOLTAGE_OCV,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    POWER_SUPPLY_PROP_VOLTAGE_MAX,
    POWER_SUPPLY_PROP_CURRENT_NOW,
    POWER_SUPPLY_PROP_TEMP,
    POWER_SUPPLY_PROP_TECHNOLOGY,
    POWER_SUPPLY_PROP_CHARGE_COUNTER,
    POWER_SUPPLY_PROP_CYCLE_COUNT,
    POWER_SUPPLY_PROP_CHARGE_FULL_DESIGN,
    POWER_SUPPLY_PROP_CHARGE_FULL,
    POWER_SUPPLY_PROP_MODEL_NAME,
    POWER_SUPPLY_PROP_TIME_TO_FULL_AVG,
    POWER_SUPPLY_PROP_TIME_TO_EMPTY_AVG,
    POWER_SUPPLY_PROP_INTERNAL_RESISTANCE,
    POWER_SUPPLY_PROP_STATE_OF_HEALTH,
    POWER_SUPPLY_PROP_POWER_NOW,
    };
    static const struct power_supply_desc sm8350_bat_psy_desc = {
    .name = "qcom-battmgr-bat",
    .type = POWER_SUPPLY_TYPE_BATTERY,
    .properties = sm8350_bat_props,
    .num_properties = ARRAY_SIZE(sm8350_bat_props),
    .get_property = qcom_battmgr_bat_get_property,
    };
    static const enum power_supply_property sm8550_bat_props[] = {
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_HEALTH,
    POWER_SUPPLY_PROP_PRESENT,
    POWER_SUPPLY_PROP_CHARGE_TYPE,
    POWER_SUPPLY_PROP_CAPACITY,
    POWER_SUPPLY_PROP_VOLTAGE_OCV,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    POWER_SUPPLY_PROP_VOLTAGE_MAX,
    POWER_SUPPLY_PROP_CURRENT_NOW,
    POWER_SUPPLY_PROP_TEMP,
    POWER_SUPPLY_PROP_TECHNOLOGY,
    POWER_SUPPLY_PROP_CHARGE_COUNTER,
    POWER_SUPPLY_PROP_CYCLE_COUNT,
    POWER_SUPPLY_PROP_CHARGE_FULL_DESIGN,
    POWER_SUPPLY_PROP_CHARGE_FULL,
    POWER_SUPPLY_PROP_MODEL_NAME,
    POWER_SUPPLY_PROP_TIME_TO_FULL_AVG,
    POWER_SUPPLY_PROP_TIME_TO_EMPTY_AVG,
    POWER_SUPPLY_PROP_INTERNAL_RESISTANCE,
    POWER_SUPPLY_PROP_STATE_OF_HEALTH,
    POWER_SUPPLY_PROP_POWER_NOW,
    POWER_SUPPLY_PROP_CHARGE_CONTROL_START_THRESHOLD,
    POWER_SUPPLY_PROP_CHARGE_CONTROL_END_THRESHOLD,
    };
    static const struct power_supply_desc sm8550_bat_psy_desc = {
    .name = "qcom-battmgr-bat",
    .type = POWER_SUPPLY_TYPE_BATTERY,
    .properties = sm8550_bat_props,
    .num_properties = ARRAY_SIZE(sm8550_bat_props),
    .get_property = qcom_battmgr_bat_get_property,
    .set_property = qcom_battmgr_bat_set_property,
    .property_is_writeable = qcom_battmgr_bat_is_writeable,
    };
    static int qcom_battmgr_ac_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct qcom_battmgr *battmgr = power_supply_get_drvdata(psy);
    int ret;
    if (!battmgr.service_up)
    return -EAGAIN;
    ret = qcom_battmgr_bat_sc8280xp_update(battmgr, psp);
    if (ret)
    return ret;
    switch (psp) {
    case POWER_SUPPLY_PROP_ONLINE:
    val.intval = battmgr.ac.online;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static const enum power_supply_property sc8280xp_ac_props[] = {
    POWER_SUPPLY_PROP_ONLINE,
    };
    static const struct power_supply_desc sc8280xp_ac_psy_desc = {
    .name = "qcom-battmgr-ac",
    .type = POWER_SUPPLY_TYPE_MAINS,
    .properties = sc8280xp_ac_props,
    .num_properties = ARRAY_SIZE(sc8280xp_ac_props),
    .get_property = qcom_battmgr_ac_get_property,
    };
    static const u8 sm8350_usb_prop_map[] = {
    [POWER_SUPPLY_PROP_ONLINE] = USB_ONLINE,
    [POWER_SUPPLY_PROP_VOLTAGE_NOW] = USB_VOLT_NOW,
    [POWER_SUPPLY_PROP_VOLTAGE_MAX] = USB_VOLT_MAX,
    [POWER_SUPPLY_PROP_CURRENT_NOW] = USB_CURR_NOW,
    [POWER_SUPPLY_PROP_CURRENT_MAX] = USB_CURR_MAX,
    [POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT] = USB_INPUT_CURR_LIMIT,
    [POWER_SUPPLY_PROP_USB_TYPE] = USB_TYPE,
    };
    static int qcom_battmgr_usb_sm8350_update(struct qcom_battmgr *battmgr,
    enum power_supply_property psp)
    {
    unsigned int prop;
    int ret;
    if (psp >= ARRAY_SIZE(sm8350_usb_prop_map))
    return -EINVAL;
    prop = sm8350_usb_prop_map[psp];
    mutex_lock(&battmgr.lock);
    ret = qcom_battmgr_request_property(battmgr, BATTMGR_USB_PROPERTY_GET, prop, 0);
    mutex_unlock(&battmgr.lock);
    return ret;
    }
    static int qcom_battmgr_usb_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct qcom_battmgr *battmgr = power_supply_get_drvdata(psy);
    int ret;
    if (!battmgr.service_up)
    return -EAGAIN;
    if (battmgr.variant == QCOM_BATTMGR_SC8280XP ||
    battmgr.variant == QCOM_BATTMGR_X1E80100)
    ret = qcom_battmgr_bat_sc8280xp_update(battmgr, psp);
    else
    ret = qcom_battmgr_usb_sm8350_update(battmgr, psp);
    if (ret)
    return ret;
    switch (psp) {
    case POWER_SUPPLY_PROP_ONLINE:
    val.intval = battmgr.usb.online;
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_NOW:
    val.intval = battmgr.usb.voltage_now;
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_MAX:
    val.intval = battmgr.usb.voltage_max;
    break;
    case POWER_SUPPLY_PROP_CURRENT_NOW:
    val.intval = battmgr.usb.current_now;
    break;
    case POWER_SUPPLY_PROP_CURRENT_MAX:
    val.intval = battmgr.usb.current_max;
    break;
    case POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT:
    val.intval = battmgr.usb.current_limit;
    break;
    case POWER_SUPPLY_PROP_USB_TYPE:
    val.intval = battmgr.usb.usb_type;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static const enum power_supply_property sc8280xp_usb_props[] = {
    POWER_SUPPLY_PROP_ONLINE,
    };
    static const struct power_supply_desc sc8280xp_usb_psy_desc = {
    .name = "qcom-battmgr-usb",
    .type = POWER_SUPPLY_TYPE_USB,
    .properties = sc8280xp_usb_props,
    .num_properties = ARRAY_SIZE(sc8280xp_usb_props),
    .get_property = qcom_battmgr_usb_get_property,
    .usb_types = BIT(POWER_SUPPLY_USB_TYPE_UNKNOWN) |
    BIT(POWER_SUPPLY_USB_TYPE_SDP)     |
    BIT(POWER_SUPPLY_USB_TYPE_DCP)     |
    BIT(POWER_SUPPLY_USB_TYPE_CDP)     |
    BIT(POWER_SUPPLY_USB_TYPE_ACA)     |
    BIT(POWER_SUPPLY_USB_TYPE_C)       |
    BIT(POWER_SUPPLY_USB_TYPE_PD)      |
    BIT(POWER_SUPPLY_USB_TYPE_PD_DRP)  |
    BIT(POWER_SUPPLY_USB_TYPE_PD_PPS)  |
    BIT(POWER_SUPPLY_USB_TYPE_APPLE_BRICK_ID),
    };
    static const enum power_supply_property sm8350_usb_props[] = {
    POWER_SUPPLY_PROP_ONLINE,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    POWER_SUPPLY_PROP_VOLTAGE_MAX,
    POWER_SUPPLY_PROP_CURRENT_NOW,
    POWER_SUPPLY_PROP_CURRENT_MAX,
    POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT,
    POWER_SUPPLY_PROP_USB_TYPE,
    };
    static const struct power_supply_desc sm8350_usb_psy_desc = {
    .name = "qcom-battmgr-usb",
    .type = POWER_SUPPLY_TYPE_USB,
    .properties = sm8350_usb_props,
    .num_properties = ARRAY_SIZE(sm8350_usb_props),
    .get_property = qcom_battmgr_usb_get_property,
    .usb_types = BIT(POWER_SUPPLY_USB_TYPE_UNKNOWN) |
    BIT(POWER_SUPPLY_USB_TYPE_SDP)     |
    BIT(POWER_SUPPLY_USB_TYPE_DCP)     |
    BIT(POWER_SUPPLY_USB_TYPE_CDP)     |
    BIT(POWER_SUPPLY_USB_TYPE_ACA)     |
    BIT(POWER_SUPPLY_USB_TYPE_C)       |
    BIT(POWER_SUPPLY_USB_TYPE_PD)      |
    BIT(POWER_SUPPLY_USB_TYPE_PD_DRP)  |
    BIT(POWER_SUPPLY_USB_TYPE_PD_PPS)  |
    BIT(POWER_SUPPLY_USB_TYPE_APPLE_BRICK_ID),
    };
    static const u8 sm8350_wls_prop_map[] = {
    [POWER_SUPPLY_PROP_ONLINE] = WLS_ONLINE,
    [POWER_SUPPLY_PROP_VOLTAGE_NOW] = WLS_VOLT_NOW,
    [POWER_SUPPLY_PROP_VOLTAGE_MAX] = WLS_VOLT_MAX,
    [POWER_SUPPLY_PROP_CURRENT_NOW] = WLS_CURR_NOW,
    [POWER_SUPPLY_PROP_CURRENT_MAX] = WLS_CURR_MAX,
    };
    static int qcom_battmgr_wls_sm8350_update(struct qcom_battmgr *battmgr,
    enum power_supply_property psp)
    {
    unsigned int prop;
    int ret;
    if (psp >= ARRAY_SIZE(sm8350_wls_prop_map))
    return -EINVAL;
    prop = sm8350_wls_prop_map[psp];
    mutex_lock(&battmgr.lock);
    ret = qcom_battmgr_request_property(battmgr, BATTMGR_WLS_PROPERTY_GET, prop, 0);
    mutex_unlock(&battmgr.lock);
    return ret;
    }
    static int qcom_battmgr_wls_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct qcom_battmgr *battmgr = power_supply_get_drvdata(psy);
    int ret;
    if (!battmgr.service_up)
    return -EAGAIN;
    if (battmgr.variant == QCOM_BATTMGR_SC8280XP ||
    battmgr.variant == QCOM_BATTMGR_X1E80100)
    ret = qcom_battmgr_bat_sc8280xp_update(battmgr, psp);
    else
    ret = qcom_battmgr_wls_sm8350_update(battmgr, psp);
    if (ret < 0)
    return ret;
    switch (psp) {
    case POWER_SUPPLY_PROP_ONLINE:
    val.intval = battmgr.wireless.online;
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_NOW:
    val.intval = battmgr.wireless.voltage_now;
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_MAX:
    val.intval = battmgr.wireless.voltage_max;
    break;
    case POWER_SUPPLY_PROP_CURRENT_NOW:
    val.intval = battmgr.wireless.current_now;
    break;
    case POWER_SUPPLY_PROP_CURRENT_MAX:
    val.intval = battmgr.wireless.current_max;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static const enum power_supply_property sc8280xp_wls_props[] = {
    POWER_SUPPLY_PROP_ONLINE,
    };
    static const struct power_supply_desc sc8280xp_wls_psy_desc = {
    .name = "qcom-battmgr-wls",
    .type = POWER_SUPPLY_TYPE_WIRELESS,
    .properties = sc8280xp_wls_props,
    .num_properties = ARRAY_SIZE(sc8280xp_wls_props),
    .get_property = qcom_battmgr_wls_get_property,
    };
    static const enum power_supply_property sm8350_wls_props[] = {
    POWER_SUPPLY_PROP_ONLINE,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    POWER_SUPPLY_PROP_VOLTAGE_MAX,
    POWER_SUPPLY_PROP_CURRENT_NOW,
    POWER_SUPPLY_PROP_CURRENT_MAX,
    };
    static const struct power_supply_desc sm8350_wls_psy_desc = {
    .name = "qcom-battmgr-wls",
    .type = POWER_SUPPLY_TYPE_WIRELESS,
    .properties = sm8350_wls_props,
    .num_properties = ARRAY_SIZE(sm8350_wls_props),
    .get_property = qcom_battmgr_wls_get_property,
    };
    static void qcom_battmgr_notification(struct qcom_battmgr *battmgr,
    const struct qcom_battmgr_message *msg,
    int len)
    {
    let mut payload_len: usize = len - sizeof(struct pmic_glink_hdr);
    unsigned int notification;
    if (payload_len != sizeof(msg.notification)) {
    dev_warn(battmgr.dev, "ignoring notification with invalid length\n");
    return;
    }
    notification = le32_to_cpu(msg.notification);
    notification &= 0xff;
    switch (notification) {
    case NOTIF_BAT_INFO:
    battmgr.info.valid = false;
    fallthrough;
    case NOTIF_BAT_STATUS:
    case NOTIF_BAT_PROPERTY:
    case NOTIF_BAT_CHARGING_STATE:
    power_supply_changed(battmgr.bat_psy);
    break;
    case NOTIF_USB_PROPERTY:
    power_supply_changed(battmgr.usb_psy);
    break;
    case NOTIF_WLS_PROPERTY:
    power_supply_changed(battmgr.wls_psy);
    break;
    default:
    dev_err(battmgr.dev, "unknown notification: %#x\n", notification);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn qcom_battmgr_sc8280xp_strcpy(dest: *mut c_char, src: *const c_char) {
    static void qcom_battmgr_sc8280xp_strcpy(char *dest, const char *src)
    {
    let mut len: usize = src[0];
// Some firmware versions return Pascal-style strings
    if (len < BATTMGR_STRING_LEN && len == strnlen(src + 1, BATTMGR_STRING_LEN - 1)) {
    memcpy(dest, src + 1, len);
    dest[len] = '\0';
    } else {
    strscpy(dest, src, BATTMGR_STRING_LEN);
    }
    }
#[no_mangle]
unsafe extern "C" fn qcom_battmgr_sc8280xp_parse_technology(chemistry: *const c_char) -> c_uint {
    static unsigned int qcom_battmgr_sc8280xp_parse_technology(const char *chemistry)
    {
    if ((!strncmp(chemistry, "LIO", 3)) ||
    (!strncmp(chemistry, "OOI", 3)))
    return POWER_SUPPLY_TECHNOLOGY_LION;
    if (!strncmp(chemistry, "LIP", 3) ||
    !strncmp(chemistry, "LiP", 3))
    return POWER_SUPPLY_TECHNOLOGY_LIPO;
    pr_err("Unknown battery technology '%s'\n", chemistry);
    return POWER_SUPPLY_TECHNOLOGY_UNKNOWN;
    }
#[no_mangle]
unsafe extern "C" fn qcom_battmgr_sc8280xp_convert_temp(temperature: c_uint) -> c_uint {
    static unsigned int qcom_battmgr_sc8280xp_convert_temp(unsigned int temperature)
    {
    return DIV_ROUND_CLOSEST(temperature, 10);
    }
    static void qcom_battmgr_sc8280xp_callback(struct qcom_battmgr *battmgr,
    const struct qcom_battmgr_message *resp,
    size_t len)
    {
    let mut opcode: c_uint = le32_to_cpu(resp.hdr.opcode);
    unsigned int source;
    unsigned int state;
    let mut payload_len: usize = len - sizeof(struct pmic_glink_hdr);
    if (payload_len < sizeof(__le32)) {
    dev_warn(battmgr.dev, "invalid payload length for %#x: %zd\n",
    opcode, len);
    return;
    }
    switch (opcode) {
    case BATTMGR_REQUEST_NOTIFICATION:
    battmgr.error = 0;
    break;
    case BATTMGR_BAT_INFO:
// some firmware versions report an extra __le32 at the end of the payload
    if (payload_len != sizeof(resp.info) &&
    payload_len != (sizeof(resp.info) + sizeof(__le32))) {
    dev_warn(battmgr.dev,
    "invalid payload length for battery information request: %zd\n",
    payload_len);
    battmgr.error = -ENODATA;
    return;
    }
    battmgr.unit = le32_to_cpu(resp.info.power_unit);
    battmgr.info.present = true;
    battmgr.info.design_capacity = le32_to_cpu(resp.info.design_capacity) * 1000;
    battmgr.info.last_full_capacity = le32_to_cpu(resp.info.last_full_capacity) * 1000;
    battmgr.info.voltage_max_design = le32_to_cpu(resp.info.design_voltage) * 1000;
    battmgr.info.capacity_low = le32_to_cpu(resp.info.capacity_low) * 1000;
    battmgr.info.cycle_count = le32_to_cpu(resp.info.cycle_count);
    qcom_battmgr_sc8280xp_strcpy(battmgr.info.model_number, resp.info.model_number);
    qcom_battmgr_sc8280xp_strcpy(battmgr.info.serial_number, resp.info.serial_number);
    battmgr.info.technology = qcom_battmgr_sc8280xp_parse_technology(resp.info.battery_chemistry);
    qcom_battmgr_sc8280xp_strcpy(battmgr.info.oem_info, resp.info.oem_info);
    battmgr.info.day = resp.info.day;
    battmgr.info.month = resp.info.month;
    battmgr.info.year = le16_to_cpu(resp.info.year);
    break;
    case BATTMGR_BAT_STATUS:
    if (payload_len != sizeof(resp.status)) {
    dev_warn(battmgr.dev,
    "invalid payload length for battery status request: %zd\n",
    payload_len);
    battmgr.error = -ENODATA;
    return;
    }
    state = le32_to_cpu(resp.status.battery_state);
    if (state & BIT(0))
    battmgr.status.status = POWER_SUPPLY_STATUS_DISCHARGING;
#[no_mangle]
pub unsafe extern "C" fn if(BIT(1): state &) -> else {
    else if (state & BIT(1))
    battmgr.status.status = POWER_SUPPLY_STATUS_CHARGING;
    else
    battmgr.status.status = POWER_SUPPLY_STATUS_NOT_CHARGING;
    battmgr.status.capacity = le32_to_cpu(resp.status.capacity) * 1000;
    battmgr.status.power_now = le32_to_cpu(resp.status.rate) * 1000;
    battmgr.status.voltage_now = le32_to_cpu(resp.status.battery_voltage) * 1000;
    battmgr.status.temperature = qcom_battmgr_sc8280xp_convert_temp(le32_to_cpu(resp.status.temperature));
    source = le32_to_cpu(resp.status.charging_source);
    battmgr.ac.online = source == BATTMGR_CHARGING_SOURCE_AC;
    battmgr.usb.online = source == BATTMGR_CHARGING_SOURCE_USB;
    battmgr.wireless.online = source == BATTMGR_CHARGING_SOURCE_WIRELESS;
    if (battmgr.info.last_full_capacity != 0) {
//
// 100 * battmgr->status.capacity can overflow a 32bit
// unsigned integer. FW readings are in m{W/A}h, which
// are multiplied by 1000 converting them to u{W/A}h,
// the format the power_supply API expects.
// To avoid overflow use the original value for dividend
// and convert the divider back to m{W/A}h, which can be
// done without any loss of precision.
//
    battmgr.status.percent =
    (100 * le32_to_cpu(resp.status.capacity)) /
    (battmgr.info.last_full_capacity / 1000);
    } else {
//
// Let the sysfs handler know no data is available at
// this time.
//
    battmgr.status.percent = (unsigned int)-1;
    }
    break;
    case BATTMGR_BAT_DISCHARGE_TIME:
    battmgr.status.discharge_time = le32_to_cpu(resp.time);
    break;
    case BATTMGR_BAT_CHARGE_TIME:
    battmgr.status.charge_time = le32_to_cpu(resp.time);
    break;
    case BATTMGR_CHG_CTRL_LIMIT_EN:
    battmgr.error = 0;
    break;
    default:
    dev_warn(battmgr.dev, "unknown message %#x\n", opcode);
    break;
    }
    complete(&battmgr.ack);
    }
    static void qcom_battmgr_sm8350_callback(struct qcom_battmgr *battmgr,
    const struct qcom_battmgr_message *resp,
    size_t len)
    {
    unsigned int property;
    let mut opcode: c_uint = le32_to_cpu(resp.hdr.opcode);
    let mut payload_len: usize = len - sizeof(struct pmic_glink_hdr);
    unsigned int val;
    if (payload_len < sizeof(__le32)) {
    dev_warn(battmgr.dev, "invalid payload length for %#x: %zd\n",
    opcode, len);
    return;
    }
    switch (opcode) {
    case BATTMGR_BAT_PROPERTY_GET:
    property = le32_to_cpu(resp.intval.property);
    if (property == BATT_MODEL_NAME) {
    if (payload_len != sizeof(resp.strval)) {
    dev_warn(battmgr.dev,
    "invalid payload length for BATT_MODEL_NAME request: %zd\n",
    payload_len);
    battmgr.error = -ENODATA;
    return;
    }
    } else {
    if (payload_len != sizeof(resp.intval)) {
    dev_warn(battmgr.dev,
    "invalid payload length for %#x request: %zd\n",
    property, payload_len);
    battmgr.error = -ENODATA;
    return;
    }
    battmgr.error = le32_to_cpu(resp.intval.result);
    if (battmgr.error)
    goto out_complete;
    }
    switch (property) {
    case BATT_STATUS:
    battmgr.status.status = le32_to_cpu(resp.intval.value);
    break;
    case BATT_HEALTH:
    battmgr.status.health = le32_to_cpu(resp.intval.value);
    break;
    case BATT_PRESENT:
    battmgr.info.present = le32_to_cpu(resp.intval.value);
    break;
    case BATT_CHG_TYPE:
    battmgr.info.charge_type = le32_to_cpu(resp.intval.value);
    break;
    case BATT_CAPACITY:
    battmgr.status.percent = le32_to_cpu(resp.intval.value) / 100;
    break;
    case BATT_SOH:
    battmgr.status.soh_percent = le32_to_cpu(resp.intval.value);
    break;
    case BATT_VOLT_OCV:
    battmgr.status.voltage_ocv = le32_to_cpu(resp.intval.value);
    break;
    case BATT_VOLT_NOW:
    battmgr.status.voltage_now = le32_to_cpu(resp.intval.value);
    break;
    case BATT_VOLT_MAX:
    battmgr.info.voltage_max = le32_to_cpu(resp.intval.value);
    break;
    case BATT_CURR_NOW:
    battmgr.status.current_now = le32_to_cpu(resp.intval.value);
    break;
    case BATT_TEMP:
    val = le32_to_cpu(resp.intval.value);
    battmgr.status.temperature = DIV_ROUND_CLOSEST(val, 10);
    break;
    case BATT_TECHNOLOGY:
    battmgr.info.technology = le32_to_cpu(resp.intval.value);
    break;
    case BATT_CHG_COUNTER:
    battmgr.info.charge_count = le32_to_cpu(resp.intval.value);
    break;
    case BATT_CYCLE_COUNT:
    battmgr.info.cycle_count = le32_to_cpu(resp.intval.value);
    break;
    case BATT_CHG_FULL_DESIGN:
    battmgr.info.design_capacity = le32_to_cpu(resp.intval.value);
    break;
    case BATT_CHG_FULL:
    battmgr.info.last_full_capacity = le32_to_cpu(resp.intval.value);
    break;
    case BATT_MODEL_NAME:
    strscpy(battmgr.info.model_number, resp.strval.model, BATTMGR_STRING_LEN);
    break;
    case BATT_TTF_AVG:
    battmgr.status.charge_time = le32_to_cpu(resp.intval.value);
    break;
    case BATT_TTE_AVG:
    battmgr.status.discharge_time = le32_to_cpu(resp.intval.value);
    break;
    case BATT_RESISTANCE:
    battmgr.status.resistance = le32_to_cpu(resp.intval.value);
    break;
    case BATT_POWER_NOW:
    battmgr.status.power_now = le32_to_cpu(resp.intval.value);
    break;
    case BATT_CHG_CTRL_START_THR:
    battmgr.info.charge_ctrl_start = le32_to_cpu(resp.intval.value);
    break;
    case BATT_CHG_CTRL_END_THR:
    battmgr.info.charge_ctrl_end = le32_to_cpu(resp.intval.value);
    break;
    default:
    dev_warn(battmgr.dev, "unknown property %#x\n", property);
    break;
    }
    break;
    case BATTMGR_USB_PROPERTY_GET:
    property = le32_to_cpu(resp.intval.property);
    if (payload_len != sizeof(resp.intval)) {
    dev_warn(battmgr.dev,
    "invalid payload length for %#x request: %zd\n",
    property, payload_len);
    battmgr.error = -ENODATA;
    return;
    }
    battmgr.error = le32_to_cpu(resp.intval.result);
    if (battmgr.error)
    goto out_complete;
    switch (property) {
    case USB_ONLINE:
    battmgr.usb.online = le32_to_cpu(resp.intval.value);
    break;
    case USB_VOLT_NOW:
    battmgr.usb.voltage_now = le32_to_cpu(resp.intval.value);
    break;
    case USB_VOLT_MAX:
    battmgr.usb.voltage_max = le32_to_cpu(resp.intval.value);
    break;
    case USB_CURR_NOW:
    battmgr.usb.current_now = le32_to_cpu(resp.intval.value);
    break;
    case USB_CURR_MAX:
    battmgr.usb.current_max = le32_to_cpu(resp.intval.value);
    break;
    case USB_INPUT_CURR_LIMIT:
    battmgr.usb.current_limit = le32_to_cpu(resp.intval.value);
    break;
    case USB_TYPE:
    battmgr.usb.usb_type = le32_to_cpu(resp.intval.value);
    break;
    default:
    dev_warn(battmgr.dev, "unknown property %#x\n", property);
    break;
    }
    break;
    case BATTMGR_WLS_PROPERTY_GET:
    property = le32_to_cpu(resp.intval.property);
    if (payload_len != sizeof(resp.intval)) {
    dev_warn(battmgr.dev,
    "invalid payload length for %#x request: %zd\n",
    property, payload_len);
    battmgr.error = -ENODATA;
    return;
    }
    battmgr.error = le32_to_cpu(resp.intval.result);
    if (battmgr.error)
    goto out_complete;
    switch (property) {
    case WLS_ONLINE:
    battmgr.wireless.online = le32_to_cpu(resp.intval.value);
    break;
    case WLS_VOLT_NOW:
    battmgr.wireless.voltage_now = le32_to_cpu(resp.intval.value);
    break;
    case WLS_VOLT_MAX:
    battmgr.wireless.voltage_max = le32_to_cpu(resp.intval.value);
    break;
    case WLS_CURR_NOW:
    battmgr.wireless.current_now = le32_to_cpu(resp.intval.value);
    break;
    case WLS_CURR_MAX:
    battmgr.wireless.current_max = le32_to_cpu(resp.intval.value);
    break;
    default:
    dev_warn(battmgr.dev, "unknown property %#x\n", property);
    break;
    }
    break;
    case BATTMGR_REQUEST_NOTIFICATION:
    case BATTMGR_CHG_CTRL_LIMIT_EN:
    battmgr.error = 0;
    break;
    default:
    dev_warn(battmgr.dev, "unknown message %#x\n", opcode);
    break;
    }
    out_complete:
    complete(&battmgr.ack);
    }
#[no_mangle]
unsafe extern "C" fn qcom_battmgr_callback(data: *const c_void, len: usize, priv: *mut c_void) {
    static void qcom_battmgr_callback(const void *data, size_t len, void *priv)
    {
    const struct pmic_glink_hdr *hdr = data;
    struct qcom_battmgr *battmgr = priv;
    let mut opcode: c_uint = le32_to_cpu(hdr.opcode);
    if (opcode == BATTMGR_NOTIFICATION)
    qcom_battmgr_notification(battmgr, data, len);
    else if (battmgr.variant == QCOM_BATTMGR_SC8280XP ||
    battmgr.variant == QCOM_BATTMGR_X1E80100)
    qcom_battmgr_sc8280xp_callback(battmgr, data, len);
    else
    qcom_battmgr_sm8350_callback(battmgr, data, len);
    }
#[no_mangle]
unsafe extern "C" fn qcom_battmgr_enable_worker(work: *mut work_struct) {
    static void qcom_battmgr_enable_worker(struct work_struct *work)
    {
    struct qcom_battmgr *battmgr = container_of(work, struct qcom_battmgr, enable_work);
    struct qcom_battmgr_enable_request req = {
    .hdr.owner = cpu_to_le32(PMIC_GLINK_OWNER_BATTMGR),
    .hdr.type = cpu_to_le32(PMIC_GLINK_NOTIFY),
    .hdr.opcode = cpu_to_le32(BATTMGR_REQUEST_NOTIFICATION),
    };
    int ret;
    ret = qcom_battmgr_request(battmgr, &req, sizeof(req));
    if (ret)
    dev_err(battmgr.dev, "failed to request power notifications\n");
    }
#[no_mangle]
unsafe extern "C" fn qcom_battmgr_pdr_notify(priv: *mut c_void, state: c_int) {
    static void qcom_battmgr_pdr_notify(void *priv, int state)
    {
    struct qcom_battmgr *battmgr = priv;
    if (state == SERVREG_SERVICE_STATE_UP) {
    battmgr.service_up = true;
    schedule_work(&battmgr.enable_work);
    } else {
    battmgr.service_up = false;
    }
    }
    static const struct of_device_id qcom_battmgr_of_variants[] = {
    { .compatible = "qcom,glymur-pmic-glink", .data = (void *)QCOM_BATTMGR_X1E80100 },
    { .compatible = "qcom,kaanapali-pmic-glink", .data = (void *)QCOM_BATTMGR_SM8550 },
    { .compatible = "qcom,sc8180x-pmic-glink", .data = (void *)QCOM_BATTMGR_SC8280XP },
    { .compatible = "qcom,sc8280xp-pmic-glink", .data = (void *)QCOM_BATTMGR_SC8280XP },
    { .compatible = "qcom,sm8550-pmic-glink", .data = (void *)QCOM_BATTMGR_SM8550 },
    { .compatible = "qcom,x1e80100-pmic-glink", .data = (void *)QCOM_BATTMGR_X1E80100 },
// Unmatched devices falls back to QCOM_BATTMGR_SM8350
    {}
    };
    static char *qcom_battmgr_battery[] = { "battery" };
    static int qcom_battmgr_probe(struct auxiliary_device *adev,
    const struct auxiliary_device_id *id)
    {
    const struct power_supply_desc *psy_desc;
    let mut psy_cfg_supply: power_supply_config = {};
    let mut psy_cfg: power_supply_config = {};
    const struct of_device_id *match;
    struct qcom_battmgr *battmgr;
    struct device *dev = &adev.dev;
    int ret;
    battmgr = devm_kzalloc(dev, sizeof(*battmgr), GFP_KERNEL);
    if (!battmgr)
    return -ENOMEM;
    battmgr.dev = dev;
    psy_cfg.drv_data = battmgr;
    psy_cfg.fwnode = dev_fwnode(&adev.dev);
    psy_cfg_supply.drv_data = battmgr;
    psy_cfg_supply.fwnode = dev_fwnode(&adev.dev);
    psy_cfg_supply.supplied_to = qcom_battmgr_battery;
    psy_cfg_supply.num_supplicants = 1;
    mutex_init(&battmgr.lock);
    init_completion(&battmgr.ack);
    match = of_match_device(qcom_battmgr_of_variants, dev.parent);
    if (match)
    battmgr.variant = (unsigned long)match.data;
    else
    battmgr.variant = QCOM_BATTMGR_SM8350;
    ret = qcom_battmgr_charge_control_thresholds_init(battmgr);
    if (ret < 0)
    return dev_err_probe(dev, ret,
    "failed to init battery charge control thresholds\n");
    if (battmgr.variant == QCOM_BATTMGR_SC8280XP ||
    battmgr.variant == QCOM_BATTMGR_X1E80100) {
    if (battmgr.variant == QCOM_BATTMGR_X1E80100)
    psy_desc = &x1e80100_bat_psy_desc;
    else
    psy_desc = &sc8280xp_bat_psy_desc;
    battmgr.bat_psy = devm_power_supply_register(dev, psy_desc, &psy_cfg);
    if (IS_ERR(battmgr.bat_psy))
    return dev_err_probe(dev, PTR_ERR(battmgr.bat_psy),
    "failed to register battery power supply\n");
    battmgr.ac_psy = devm_power_supply_register(dev, &sc8280xp_ac_psy_desc, &psy_cfg_supply);
    if (IS_ERR(battmgr.ac_psy))
    return dev_err_probe(dev, PTR_ERR(battmgr.ac_psy),
    "failed to register AC power supply\n");
    battmgr.usb_psy = devm_power_supply_register(dev, &sc8280xp_usb_psy_desc, &psy_cfg_supply);
    if (IS_ERR(battmgr.usb_psy))
    return dev_err_probe(dev, PTR_ERR(battmgr.usb_psy),
    "failed to register USB power supply\n");
    battmgr.wls_psy = devm_power_supply_register(dev, &sc8280xp_wls_psy_desc, &psy_cfg_supply);
    if (IS_ERR(battmgr.wls_psy))
    return dev_err_probe(dev, PTR_ERR(battmgr.wls_psy),
    "failed to register wireless charing power supply\n");
    } else {
    if (battmgr.variant == QCOM_BATTMGR_SM8550)
    psy_desc = &sm8550_bat_psy_desc;
    else
    psy_desc = &sm8350_bat_psy_desc;
    battmgr.bat_psy = devm_power_supply_register(dev, psy_desc, &psy_cfg);
    if (IS_ERR(battmgr.bat_psy))
    return dev_err_probe(dev, PTR_ERR(battmgr.bat_psy),
    "failed to register battery power supply\n");
    battmgr.usb_psy = devm_power_supply_register(dev, &sm8350_usb_psy_desc, &psy_cfg_supply);
    if (IS_ERR(battmgr.usb_psy))
    return dev_err_probe(dev, PTR_ERR(battmgr.usb_psy),
    "failed to register USB power supply\n");
    battmgr.wls_psy = devm_power_supply_register(dev, &sm8350_wls_psy_desc, &psy_cfg_supply);
    if (IS_ERR(battmgr.wls_psy))
    return dev_err_probe(dev, PTR_ERR(battmgr.wls_psy),
    "failed to register wireless charing power supply\n");
    }
    ret = devm_work_autocancel(dev, &battmgr.enable_work,
    qcom_battmgr_enable_worker);
    if (ret)
    return ret;
    battmgr.client = devm_pmic_glink_client_alloc(dev, PMIC_GLINK_OWNER_BATTMGR,
    qcom_battmgr_callback,
    qcom_battmgr_pdr_notify,
    battmgr);
    if (IS_ERR(battmgr.client))
    return PTR_ERR(battmgr.client);
    pmic_glink_client_register(battmgr.client);
    return 0;
    }
    static const struct auxiliary_device_id qcom_battmgr_id_table[] = {
    { .name = "pmic_glink.power-supply", },
    {},
    };
    MODULE_DEVICE_TABLE(auxiliary, qcom_battmgr_id_table);
    static struct auxiliary_driver qcom_battmgr_driver = {
    .name = "pmic_glink_power_supply",
    .probe = qcom_battmgr_probe,
    .id_table = qcom_battmgr_id_table,
    };
    module_auxiliary_driver(qcom_battmgr_driver);
    MODULE_DESCRIPTION("Qualcomm PMIC GLINK battery manager driver");
    MODULE_LICENSE("GPL");
