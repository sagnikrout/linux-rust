//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/bq256xx_charger.c
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
// BQ256XX Battery Charger Driver
// Copyright (C) 2020 Texas Instruments Incorporated - http://www.ti.com

pub const BQ256XX_INPUT_CURRENT_LIMIT: c_uint = 0x00;
pub const BQ256XX_CHARGER_CONTROL_0: c_uint = 0x01;
pub const BQ256XX_CHARGE_CURRENT_LIMIT: c_uint = 0x02;
pub const BQ256XX_PRECHG_AND_TERM_CURR_LIM: c_uint = 0x03;
pub const BQ256XX_BATTERY_VOLTAGE_LIMIT: c_uint = 0x04;
pub const BQ256XX_CHARGER_CONTROL_1: c_uint = 0x05;
pub const BQ256XX_CHARGER_CONTROL_2: c_uint = 0x06;
pub const BQ256XX_CHARGER_CONTROL_3: c_uint = 0x07;
pub const BQ256XX_CHARGER_STATUS_0: c_uint = 0x08;
pub const BQ256XX_CHARGER_STATUS_1: c_uint = 0x09;
pub const BQ256XX_CHARGER_STATUS_2: c_uint = 0x0a;
pub const BQ256XX_PART_INFORMATION: c_uint = 0x0b;
pub const BQ256XX_CHARGER_CONTROL_4: c_uint = 0x0c;

pub const BQ256XX_IINDPM_STEP_uA: c_int = 100000;
pub const BQ256XX_IINDPM_OFFSET_uA: c_int = 100000;
pub const BQ256XX_IINDPM_MIN_uA: c_int = 100000;
pub const BQ256XX_IINDPM_MAX_uA: c_int = 3200000;
pub const BQ256XX_IINDPM_DEF_uA: c_int = 2400000;

pub const BQ256XX_TS_IGNORE_SHIFT: c_int = 6;

pub const BQ256XX_VINDPM_STEP_uV: c_int = 100000;
pub const BQ256XX_VINDPM_OFFSET_uV: c_int = 3900000;
pub const BQ256XX_VINDPM_MIN_uV: c_int = 3900000;
pub const BQ256XX_VINDPM_MAX_uV: c_int = 5400000;
pub const BQ256XX_VINDPM_DEF_uV: c_int = 4500000;

pub const BQ2560X_VBATREG_STEP_uV: c_int = 32000;
pub const BQ2560X_VBATREG_OFFSET_uV: c_int = 3856000;
pub const BQ2560X_VBATREG_MIN_uV: c_int = 3856000;
pub const BQ2560X_VBATREG_MAX_uV: c_int = 4624000;
pub const BQ2560X_VBATREG_DEF_uV: c_int = 4208000;
pub const BQ25601D_VBATREG_OFFSET_uV: c_int = 3847000;
pub const BQ25601D_VBATREG_MIN_uV: c_int = 3847000;
pub const BQ25601D_VBATREG_MAX_uV: c_int = 4615000;
pub const BQ25601D_VBATREG_DEF_uV: c_int = 4199000;
pub const BQ2561X_VBATREG_STEP_uV: c_int = 10000;
pub const BQ25611D_VBATREG_MIN_uV: c_int = 3494000;
pub const BQ25611D_VBATREG_MAX_uV: c_int = 4510000;
pub const BQ25611D_VBATREG_DEF_uV: c_int = 4190000;
pub const BQ25618_VBATREG_MIN_uV: c_int = 3504000;
pub const BQ25618_VBATREG_MAX_uV: c_int = 4500000;
pub const BQ25618_VBATREG_DEF_uV: c_int = 4200000;
pub const BQ256XX_VBATREG_BIT_SHIFT: c_int = 3;
pub const BQ2561X_VBATREG_THRESH: c_uint = 0x8;
pub const BQ25611D_VBATREG_THRESH_uV: c_int = 4290000;
pub const BQ25618_VBATREG_THRESH_uV: c_int = 4300000;

pub const BQ256XX_CHG_CONFIG_BIT_SHIFT: c_int = 4;

pub const BQ256XX_ITERM_STEP_uA: c_int = 60000;
pub const BQ256XX_ITERM_OFFSET_uA: c_int = 60000;
pub const BQ256XX_ITERM_MIN_uA: c_int = 60000;
pub const BQ256XX_ITERM_MAX_uA: c_int = 780000;
pub const BQ256XX_ITERM_DEF_uA: c_int = 180000;
pub const BQ25618_ITERM_STEP_uA: c_int = 20000;
pub const BQ25618_ITERM_OFFSET_uA: c_int = 20000;
pub const BQ25618_ITERM_MIN_uA: c_int = 20000;
pub const BQ25618_ITERM_MAX_uA: c_int = 260000;
pub const BQ25618_ITERM_DEF_uA: c_int = 60000;

pub const BQ256XX_IPRECHG_STEP_uA: c_int = 60000;
pub const BQ256XX_IPRECHG_OFFSET_uA: c_int = 60000;
pub const BQ256XX_IPRECHG_MIN_uA: c_int = 60000;
pub const BQ256XX_IPRECHG_MAX_uA: c_int = 780000;
pub const BQ256XX_IPRECHG_DEF_uA: c_int = 180000;
pub const BQ25618_IPRECHG_STEP_uA: c_int = 20000;
pub const BQ25618_IPRECHG_OFFSET_uA: c_int = 20000;
pub const BQ25618_IPRECHG_MIN_uA: c_int = 20000;
pub const BQ25618_IPRECHG_MAX_uA: c_int = 260000;
pub const BQ25618_IPRECHG_DEF_uA: c_int = 40000;
pub const BQ256XX_IPRECHG_BIT_SHIFT: c_int = 4;

pub const BQ256XX_ICHG_STEP_uA: c_int = 60000;
pub const BQ256XX_ICHG_MIN_uA: c_int = 0;
pub const BQ256XX_ICHG_MAX_uA: c_int = 3000000;
pub const BQ2560X_ICHG_DEF_uA: c_int = 2040000;
pub const BQ25611D_ICHG_DEF_uA: c_int = 1020000;
pub const BQ25618_ICHG_STEP_uA: c_int = 20000;
pub const BQ25618_ICHG_MIN_uA: c_int = 0;
pub const BQ25618_ICHG_MAX_uA: c_int = 1500000;
pub const BQ25618_ICHG_DEF_uA: c_int = 340000;
pub const BQ25618_ICHG_THRESH: c_uint = 0x3c;
pub const BQ25618_ICHG_THRESH_uA: c_int = 1180000;

pub const BQ256XX_VBUS_STAT_NO_INPUT: c_int = 0;

pub const BQ256XX_CHRG_STAT_NOT_CHRGING: c_int = 0;

pub const BQ256XX_CHRG_FAULT_NORMAL: c_int = 0;

pub const BQ256XX_NUM_WD_VAL: c_int = 4;

pub const BQ256XX_WATCHDOG_MAX: c_int = 1600000;
pub const BQ256XX_WATCHDOG_DIS: c_int = 0;
pub const BQ256XX_WDT_BIT_SHIFT: c_int = 4;

//
// struct bq256xx_init_data -
// @ichg: fast charge current
// @iindpm: input current limit
// @vbatreg: charge voltage
// @iterm: termination current
// @iprechg: precharge current
// @vindpm: input voltage limit
// @ichg_max: maximum fast charge current
// @vbatreg_max: maximum charge voltage
// @ts_ignore: TS_IGNORE flag
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bq256xx_init_data {
    pub ichg: u32,
    pub iindpm: u32,
    pub vbatreg: u32,
    pub iterm: u32,
    pub iprechg: u32,
    pub vindpm: u32,
    pub ichg_max: u32,
    pub vbatreg_max: u32,
    pub ts_ignore: bool,
}

//
// struct bq256xx_state -
// @vbus_stat: VBUS status according to BQ256XX_CHARGER_STATUS_0
// @chrg_stat: charging status according to BQ256XX_CHARGER_STATUS_0
// @online: PG status according to BQ256XX_CHARGER_STATUS_0
//
// @wdt_fault: watchdog fault according to BQ256XX_CHARGER_STATUS_1
// @bat_fault: battery fault according to BQ256XX_CHARGER_STATUS_1
// @chrg_fault: charging fault according to BQ256XX_CHARGER_STATUS_1
// @ntc_fault: TS fault according to BQ256XX_CHARGER_STATUS_1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bq256xx_state {
    pub vbus_stat: u8,
    pub chrg_stat: u8,
    pub online: bool,
    pub wdt_fault: u8,
    pub bat_fault: u8,
    pub chrg_fault: u8,
    pub ntc_fault: u8,
}

    enum bq256xx_id {
    BQ25600,
    BQ25600D,
    BQ25601,
    BQ25601D,
    BQ25618,
    BQ25619,
    BQ25611D,
    };
//
// struct bq256xx_device -
// @client: i2c client structure
// @regmap: register map structure
// @dev: device structure
// @charger: power supply registered for the charger
// @battery: power supply registered for the battery
// @lock: mutex lock structure
//
// @usb2_phy: usb_phy identifier
// @usb3_phy: usb_phy identifier
// @usb_nb: notifier block
// @usb_work: usb work queue
// @usb_event: usb_event code
//
// @model_name: i2c name string
//
// @init_data: initialization data
// @chip_info: device variant information
// @state: device status and faults
// @watchdog_timer: watchdog timer value in milliseconds
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bq256xx_device {
    pub client: *mut i2c_client,
    pub dev: *mut device,
    pub charger: *mut power_supply,
    pub battery: *mut power_supply,
    pub lock: mutex,
    pub regmap: *mut regmap,
    pub usb2_phy: *mut usb_phy,
    pub usb3_phy: *mut usb_phy,
    pub usb_nb: notifier_block,
    pub usb_work: work_struct,
    pub usb_event: c_ulong,
    pub model_name: [c_char; I2C_NAME_SIZE],
    pub init_data: bq256xx_init_data,
    pub chip_info: *const bq256xx_chip_info,
    pub state: bq256xx_state,
    pub watchdog_timer: c_int,
}

//
// struct bq256xx_chip_info -
// @model_id: device instance
//
// @bq256xx_regmap_config: regmap configuration struct
// @bq256xx_get_ichg: pointer to instance specific get_ichg function
// @bq256xx_get_iindpm: pointer to instance specific get_iindpm function
// @bq256xx_get_vbatreg: pointer to instance specific get_vbatreg function
// @bq256xx_get_iterm: pointer to instance specific get_iterm function
// @bq256xx_get_iprechg: pointer to instance specific get_iprechg function
// @bq256xx_get_vindpm: pointer to instance specific get_vindpm function
//
// @bq256xx_set_ichg: pointer to instance specific set_ichg function
// @bq256xx_set_iindpm: pointer to instance specific set_iindpm function
// @bq256xx_set_vbatreg: pointer to instance specific set_vbatreg function
// @bq256xx_set_iterm: pointer to instance specific set_iterm function
// @bq256xx_set_iprechg: pointer to instance specific set_iprechg function
// @bq256xx_set_vindpm: pointer to instance specific set_vindpm function
// @bq256xx_set_charge_type: pointer to instance specific set_charge_type function
// @bq256xx_set_ts_ignore: pointer to instance specific set_ts_ignore function
//
// @bq256xx_def_ichg: default ichg value in microamps
// @bq256xx_def_iindpm: default iindpm value in microamps
// @bq256xx_def_vbatreg: default vbatreg value in microvolts
// @bq256xx_def_iterm: default iterm value in microamps
// @bq256xx_def_iprechg: default iprechg value in microamps
// @bq256xx_def_vindpm: default vindpm value in microvolts
//
// @bq256xx_max_ichg: maximum charge current in microamps
// @bq256xx_max_vbatreg: maximum battery regulation voltage in microvolts
//
// @has_usb_detect: indicates whether device has BC1.2 detection
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bq256xx_chip_info {
    pub model_id: c_int,
    pub bq256xx_regmap_config: *const regmap_config,
    pub bq): *mut *mut int (bq256xx_get_ichg)(struct bq256xx_device,
    pub bq): *mut *mut int (bq256xx_get_iindpm)(struct bq256xx_device,
    pub bq): *mut *mut int (bq256xx_get_vbatreg)(struct bq256xx_device,
    pub bq): *mut *mut int (bq256xx_get_iterm)(struct bq256xx_device,
    pub bq): *mut *mut int (bq256xx_get_iprechg)(struct bq256xx_device,
    pub bq): *mut *mut int (bq256xx_get_vindpm)(struct bq256xx_device,
    pub ichg): *mut *mut *mut int (bq256xx_set_ichg)(struct bq256xx_device bq, int,
    pub iindpm): *mut *mut *mut int (bq256xx_set_iindpm)(struct bq256xx_device bq, int,
    pub vbatreg): *mut *mut *mut int (bq256xx_set_vbatreg)(struct bq256xx_device bq, int,
    pub iterm): *mut *mut *mut int (bq256xx_set_iterm)(struct bq256xx_device bq, int,
    pub iprechg): *mut *mut *mut int (bq256xx_set_iprechg)(struct bq256xx_device bq, int,
    pub vindpm): *mut *mut *mut int (bq256xx_set_vindpm)(struct bq256xx_device bq, int,
    pub type): *mut *mut *mut int (bq256xx_set_charge_type)(struct bq256xx_device bq, int,
    pub ts_ignore): *mut *mut *mut int (bq256xx_set_ts_ignore)(struct bq256xx_device bq, bool,
    pub bq256xx_def_ichg: c_int,
    pub bq256xx_def_iindpm: c_int,
    pub bq256xx_def_vbatreg: c_int,
    pub bq256xx_def_iterm: c_int,
    pub bq256xx_def_iprechg: c_int,
    pub bq256xx_def_vindpm: c_int,
    pub bq256xx_max_ichg: c_int,
    pub bq256xx_max_vbatreg: c_int,
    pub has_usb_detect: bool,
}

    static int bq256xx_watchdog_time[BQ256XX_NUM_WD_VAL] = {
    0, 40000, 80000, 1600000
    };
    static const int bq25611d_vbatreg_values[] = {
    3494000, 3590000, 3686000, 3790000, 3894000, 3990000, 4090000, 4140000,
    4190000
    };
    static const int bq25618_619_vbatreg_values[] = {
    3504000, 3600000, 3696000, 3800000, 3904000, 4000000, 4100000, 4150000,
    4200000
    };
    static const int bq25618_619_ichg_values[] = {
    1290000, 1360000, 1430000, 1500000
    };
#[no_mangle]
unsafe extern "C" fn bq256xx_array_parse(array_size: c_int, val: c_int, array[]: c_int) -> c_int {
    static int bq256xx_array_parse(int array_size, int val, const int array[])
    {
    let mut i: c_int = 0;
    if (val < array[i])
    return i - 1;
    if (val >= array[array_size - 1])
    return array_size - 1;
    for (i = 1; i < array_size; i++) {
    if (val == array[i])
    return i;
    if (val > array[i - 1] && val < array[i])
    return i - 1;
    }
    return -EINVAL;
    }
    static int bq256xx_usb_notifier(struct notifier_block *nb, unsigned long val,
    void *priv)
    {
    struct bq256xx_device *bq =
    container_of(nb, struct bq256xx_device, usb_nb);
    bq.usb_event = val;
    queue_work(system_power_efficient_wq, &bq.usb_work);
    return NOTIFY_OK;
    }
#[no_mangle]
unsafe extern "C" fn bq256xx_usb_work(data: *mut work_struct) {
    static void bq256xx_usb_work(struct work_struct *data)
    {
    struct bq256xx_device *bq =
    container_of(data, struct bq256xx_device, usb_work);
    switch (bq.usb_event) {
    case USB_EVENT_ID:
    break;
    case USB_EVENT_NONE:
    power_supply_changed(bq.charger);
    break;
    default:
    dev_err(bq.dev, "Error switching to charger mode.\n");
    break;
    }
    }
    static const struct reg_default bq2560x_reg_defs[] = {
    {BQ256XX_INPUT_CURRENT_LIMIT, 0x17},
    {BQ256XX_CHARGER_CONTROL_0, 0x1a},
    {BQ256XX_CHARGE_CURRENT_LIMIT, 0xa2},
    {BQ256XX_PRECHG_AND_TERM_CURR_LIM, 0x22},
    {BQ256XX_BATTERY_VOLTAGE_LIMIT, 0x58},
    {BQ256XX_CHARGER_CONTROL_1, 0x9f},
    {BQ256XX_CHARGER_CONTROL_2, 0x66},
    {BQ256XX_CHARGER_CONTROL_3, 0x4c},
    };
    static const struct reg_default bq25611d_reg_defs[] = {
    {BQ256XX_INPUT_CURRENT_LIMIT, 0x17},
    {BQ256XX_CHARGER_CONTROL_0, 0x1a},
    {BQ256XX_CHARGE_CURRENT_LIMIT, 0x91},
    {BQ256XX_PRECHG_AND_TERM_CURR_LIM, 0x12},
    {BQ256XX_BATTERY_VOLTAGE_LIMIT, 0x40},
    {BQ256XX_CHARGER_CONTROL_1, 0x9e},
    {BQ256XX_CHARGER_CONTROL_2, 0xe6},
    {BQ256XX_CHARGER_CONTROL_3, 0x4c},
    {BQ256XX_PART_INFORMATION, 0x54},
    {BQ256XX_CHARGER_CONTROL_4, 0x75},
    };
    static const struct reg_default bq25618_619_reg_defs[] = {
    {BQ256XX_INPUT_CURRENT_LIMIT, 0x17},
    {BQ256XX_CHARGER_CONTROL_0, 0x1a},
    {BQ256XX_CHARGE_CURRENT_LIMIT, 0x91},
    {BQ256XX_PRECHG_AND_TERM_CURR_LIM, 0x12},
    {BQ256XX_BATTERY_VOLTAGE_LIMIT, 0x40},
    {BQ256XX_CHARGER_CONTROL_1, 0x9e},
    {BQ256XX_CHARGER_CONTROL_2, 0xe6},
    {BQ256XX_CHARGER_CONTROL_3, 0x4c},
    {BQ256XX_PART_INFORMATION, 0x2c},
    {BQ256XX_CHARGER_CONTROL_4, 0x75},
    };
    static int bq256xx_get_state(struct bq256xx_device *bq,
    struct bq256xx_state *state)
    {
    unsigned int charger_status_0;
    unsigned int charger_status_1;
    int ret;
    ret = regmap_read(bq.regmap, BQ256XX_CHARGER_STATUS_0,
    &charger_status_0);
    if (ret)
    return ret;
    ret = regmap_read(bq.regmap, BQ256XX_CHARGER_STATUS_1,
    &charger_status_1);
    if (ret)
    return ret;
    state.vbus_stat = charger_status_0 & BQ256XX_VBUS_STAT_MASK;
    state.chrg_stat = charger_status_0 & BQ256XX_CHRG_STAT_MASK;
    state.online = charger_status_0 & BQ256XX_PG_STAT_MASK;
    state.wdt_fault = charger_status_1 & BQ256XX_WDT_FAULT_MASK;
    state.bat_fault = charger_status_1 & BQ256XX_BAT_FAULT_MASK;
    state.chrg_fault = charger_status_1 & BQ256XX_CHRG_FAULT_MASK;
    state.ntc_fault = charger_status_1 & BQ256XX_NTC_FAULT_MASK;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bq256xx_set_charge_type(bq: *mut bq256xx_device, type: c_int) -> c_int {
    static int bq256xx_set_charge_type(struct bq256xx_device *bq, int type)
    {
    let mut chg_config: c_int = 0;
    switch (type) {
    case POWER_SUPPLY_CHARGE_TYPE_NONE:
    chg_config = 0x0;
    break;
    case POWER_SUPPLY_CHARGE_TYPE_TRICKLE:
    case POWER_SUPPLY_CHARGE_TYPE_FAST:
    chg_config = 0x1;
    break;
    default:
    return -EINVAL;
    }
    return regmap_update_bits(bq.regmap, BQ256XX_CHARGER_CONTROL_0,
    BQ256XX_CHG_CONFIG_MASK,
    (chg_config ? 1 : 0) << BQ256XX_CHG_CONFIG_BIT_SHIFT);
    }
#[no_mangle]
unsafe extern "C" fn bq256xx_get_ichg_curr(bq: *mut bq256xx_device) -> c_int {
    static int bq256xx_get_ichg_curr(struct bq256xx_device *bq)
    {
    unsigned int charge_current_limit;
    unsigned int ichg_reg_code;
    int ret;
    ret = regmap_read(bq.regmap, BQ256XX_CHARGE_CURRENT_LIMIT,
    &charge_current_limit);
    if (ret)
    return ret;
    ichg_reg_code = charge_current_limit & BQ256XX_ICHG_MASK;
    return ichg_reg_code * BQ256XX_ICHG_STEP_uA;
    }
#[no_mangle]
unsafe extern "C" fn bq25618_619_get_ichg_curr(bq: *mut bq256xx_device) -> c_int {
    static int bq25618_619_get_ichg_curr(struct bq256xx_device *bq)
    {
    unsigned int charge_current_limit;
    unsigned int ichg_reg_code;
    int ret;
    ret = regmap_read(bq.regmap, BQ256XX_CHARGE_CURRENT_LIMIT,
    &charge_current_limit);
    if (ret)
    return ret;
    ichg_reg_code = charge_current_limit & BQ256XX_ICHG_MASK;
    if (ichg_reg_code < BQ25618_ICHG_THRESH)
    return ichg_reg_code * BQ25618_ICHG_STEP_uA;
    return bq25618_619_ichg_values[ichg_reg_code - BQ25618_ICHG_THRESH];
    }
#[no_mangle]
unsafe extern "C" fn bq256xx_set_ichg_curr(bq: *mut bq256xx_device, ichg: c_int) -> c_int {
    static int bq256xx_set_ichg_curr(struct bq256xx_device *bq, int ichg)
    {
    unsigned int ichg_reg_code;
    let mut ichg_max: c_int = bq.init_data.ichg_max;
    ichg = clamp(ichg, BQ256XX_ICHG_MIN_uA, ichg_max);
    ichg_reg_code = ichg / BQ256XX_ICHG_STEP_uA;
    return regmap_update_bits(bq.regmap, BQ256XX_CHARGE_CURRENT_LIMIT,
    BQ256XX_ICHG_MASK, ichg_reg_code);
    }
#[no_mangle]
unsafe extern "C" fn bq25618_619_set_ichg_curr(bq: *mut bq256xx_device, ichg: c_int) -> c_int {
    static int bq25618_619_set_ichg_curr(struct bq256xx_device *bq, int ichg)
    {
    let mut array_size: c_int = ARRAY_SIZE(bq25618_619_ichg_values);
    unsigned int ichg_reg_code;
    let mut ichg_max: c_int = bq.init_data.ichg_max;
    ichg = clamp(ichg, BQ25618_ICHG_MIN_uA, ichg_max);
    if (ichg <= BQ25618_ICHG_THRESH_uA) {
    ichg_reg_code = ichg / BQ25618_ICHG_STEP_uA;
    } else {
    ichg_reg_code = bq256xx_array_parse(array_size, ichg,
    bq25618_619_ichg_values) + BQ25618_ICHG_THRESH;
    }
    return regmap_update_bits(bq.regmap, BQ256XX_CHARGE_CURRENT_LIMIT,
    BQ256XX_ICHG_MASK, ichg_reg_code);
    }
#[no_mangle]
unsafe extern "C" fn bq25618_619_get_chrg_volt(bq: *mut bq256xx_device) -> c_int {
    static int bq25618_619_get_chrg_volt(struct bq256xx_device *bq)
    {
    unsigned int battery_volt_lim;
    unsigned int vbatreg_reg_code;
    int ret;
    ret = regmap_read(bq.regmap, BQ256XX_BATTERY_VOLTAGE_LIMIT,
    &battery_volt_lim);
    if (ret)
    return ret;
    vbatreg_reg_code = (battery_volt_lim & BQ256XX_VBATREG_MASK) >>
    BQ256XX_VBATREG_BIT_SHIFT;
    if (vbatreg_reg_code > BQ2561X_VBATREG_THRESH)
    return ((vbatreg_reg_code - BQ2561X_VBATREG_THRESH) *
    BQ2561X_VBATREG_STEP_uV) +
    BQ25618_VBATREG_THRESH_uV;
    return bq25618_619_vbatreg_values[vbatreg_reg_code];
    }
#[no_mangle]
unsafe extern "C" fn bq25611d_get_chrg_volt(bq: *mut bq256xx_device) -> c_int {
    static int bq25611d_get_chrg_volt(struct bq256xx_device *bq)
    {
    unsigned int battery_volt_lim;
    unsigned int vbatreg_reg_code;
    int ret;
    ret = regmap_read(bq.regmap, BQ256XX_BATTERY_VOLTAGE_LIMIT,
    &battery_volt_lim);
    if (ret)
    return ret;
    vbatreg_reg_code = (battery_volt_lim & BQ256XX_VBATREG_MASK) >>
    BQ256XX_VBATREG_BIT_SHIFT;
    if (vbatreg_reg_code > BQ2561X_VBATREG_THRESH)
    return ((vbatreg_reg_code - BQ2561X_VBATREG_THRESH) *
    BQ2561X_VBATREG_STEP_uV) +
    BQ25611D_VBATREG_THRESH_uV;
    return bq25611d_vbatreg_values[vbatreg_reg_code];
    }
#[no_mangle]
unsafe extern "C" fn bq2560x_get_chrg_volt(bq: *mut bq256xx_device) -> c_int {
    static int bq2560x_get_chrg_volt(struct bq256xx_device *bq)
    {
    unsigned int battery_volt_lim;
    unsigned int vbatreg_reg_code;
    int ret;
    ret = regmap_read(bq.regmap, BQ256XX_BATTERY_VOLTAGE_LIMIT,
    &battery_volt_lim);
    if (ret)
    return ret;
    vbatreg_reg_code = (battery_volt_lim & BQ256XX_VBATREG_MASK) >>
    BQ256XX_VBATREG_BIT_SHIFT;
    return (vbatreg_reg_code * BQ2560X_VBATREG_STEP_uV)
    + BQ2560X_VBATREG_OFFSET_uV;
    }
#[no_mangle]
unsafe extern "C" fn bq25601d_get_chrg_volt(bq: *mut bq256xx_device) -> c_int {
    static int bq25601d_get_chrg_volt(struct bq256xx_device *bq)
    {
    unsigned int battery_volt_lim;
    unsigned int vbatreg_reg_code;
    int ret;
    ret = regmap_read(bq.regmap, BQ256XX_BATTERY_VOLTAGE_LIMIT,
    &battery_volt_lim);
    if (ret)
    return ret;
    vbatreg_reg_code = (battery_volt_lim & BQ256XX_VBATREG_MASK) >>
    BQ256XX_VBATREG_BIT_SHIFT;
    return (vbatreg_reg_code * BQ2560X_VBATREG_STEP_uV)
    + BQ25601D_VBATREG_OFFSET_uV;
    }
#[no_mangle]
unsafe extern "C" fn bq25618_619_set_chrg_volt(bq: *mut bq256xx_device, vbatreg: c_int) -> c_int {
    static int bq25618_619_set_chrg_volt(struct bq256xx_device *bq, int vbatreg)
    {
    let mut array_size: c_int = ARRAY_SIZE(bq25618_619_vbatreg_values);
    unsigned int vbatreg_reg_code;
    let mut vbatreg_max: c_int = bq.init_data.vbatreg_max;
    vbatreg = clamp(vbatreg, BQ25618_VBATREG_MIN_uV, vbatreg_max);
    if (vbatreg > BQ25618_VBATREG_THRESH_uV)
    vbatreg_reg_code = ((vbatreg -
    BQ25618_VBATREG_THRESH_uV) /
    (BQ2561X_VBATREG_STEP_uV)) + BQ2561X_VBATREG_THRESH;
    else {
    vbatreg_reg_code = bq256xx_array_parse(array_size, vbatreg,
    bq25618_619_vbatreg_values);
    }
    return regmap_update_bits(bq.regmap, BQ256XX_BATTERY_VOLTAGE_LIMIT,
    BQ256XX_VBATREG_MASK, vbatreg_reg_code <<
    BQ256XX_VBATREG_BIT_SHIFT);
    }
#[no_mangle]
unsafe extern "C" fn bq25611d_set_chrg_volt(bq: *mut bq256xx_device, vbatreg: c_int) -> c_int {
    static int bq25611d_set_chrg_volt(struct bq256xx_device *bq, int vbatreg)
    {
    let mut array_size: c_int = ARRAY_SIZE(bq25611d_vbatreg_values);
    unsigned int vbatreg_reg_code;
    let mut vbatreg_max: c_int = bq.init_data.vbatreg_max;
    vbatreg = clamp(vbatreg, BQ25611D_VBATREG_MIN_uV, vbatreg_max);
    if (vbatreg > BQ25611D_VBATREG_THRESH_uV)
    vbatreg_reg_code = ((vbatreg -
    BQ25611D_VBATREG_THRESH_uV) /
    (BQ2561X_VBATREG_STEP_uV)) + BQ2561X_VBATREG_THRESH;
    else {
    vbatreg_reg_code = bq256xx_array_parse(array_size, vbatreg,
    bq25611d_vbatreg_values);
    }
    return regmap_update_bits(bq.regmap, BQ256XX_BATTERY_VOLTAGE_LIMIT,
    BQ256XX_VBATREG_MASK, vbatreg_reg_code <<
    BQ256XX_VBATREG_BIT_SHIFT);
    }
#[no_mangle]
unsafe extern "C" fn bq2560x_set_chrg_volt(bq: *mut bq256xx_device, vbatreg: c_int) -> c_int {
    static int bq2560x_set_chrg_volt(struct bq256xx_device *bq, int vbatreg)
    {
    unsigned int vbatreg_reg_code;
    let mut vbatreg_max: c_int = bq.init_data.vbatreg_max;
    vbatreg = clamp(vbatreg, BQ2560X_VBATREG_MIN_uV, vbatreg_max);
    vbatreg_reg_code = (vbatreg - BQ2560X_VBATREG_OFFSET_uV) /
    BQ2560X_VBATREG_STEP_uV;
    return regmap_update_bits(bq.regmap, BQ256XX_BATTERY_VOLTAGE_LIMIT,
    BQ256XX_VBATREG_MASK, vbatreg_reg_code <<
    BQ256XX_VBATREG_BIT_SHIFT);
    }
#[no_mangle]
unsafe extern "C" fn bq25601d_set_chrg_volt(bq: *mut bq256xx_device, vbatreg: c_int) -> c_int {
    static int bq25601d_set_chrg_volt(struct bq256xx_device *bq, int vbatreg)
    {
    unsigned int vbatreg_reg_code;
    let mut vbatreg_max: c_int = bq.init_data.vbatreg_max;
    vbatreg = clamp(vbatreg, BQ25601D_VBATREG_MIN_uV, vbatreg_max);
    vbatreg_reg_code = (vbatreg - BQ25601D_VBATREG_OFFSET_uV) /
    BQ2560X_VBATREG_STEP_uV;
    return regmap_update_bits(bq.regmap, BQ256XX_BATTERY_VOLTAGE_LIMIT,
    BQ256XX_VBATREG_MASK, vbatreg_reg_code <<
    BQ256XX_VBATREG_BIT_SHIFT);
    }
#[no_mangle]
unsafe extern "C" fn bq256xx_set_ts_ignore(bq: *mut bq256xx_device, ts_ignore: bool) -> c_int {
    static int bq256xx_set_ts_ignore(struct bq256xx_device *bq, bool ts_ignore)
    {
    return regmap_update_bits(bq.regmap, BQ256XX_INPUT_CURRENT_LIMIT,
    BQ256XX_TS_IGNORE, (ts_ignore ? 1 : 0) << BQ256XX_TS_IGNORE_SHIFT);
    }
#[no_mangle]
unsafe extern "C" fn bq256xx_get_prechrg_curr(bq: *mut bq256xx_device) -> c_int {
    static int bq256xx_get_prechrg_curr(struct bq256xx_device *bq)
    {
    unsigned int prechg_and_term_curr_lim;
    unsigned int iprechg_reg_code;
    int ret;
    ret = regmap_read(bq.regmap, BQ256XX_PRECHG_AND_TERM_CURR_LIM,
    &prechg_and_term_curr_lim);
    if (ret)
    return ret;
    iprechg_reg_code = (prechg_and_term_curr_lim & BQ256XX_IPRECHG_MASK)
    >> BQ256XX_IPRECHG_BIT_SHIFT;
    return (iprechg_reg_code * BQ256XX_IPRECHG_STEP_uA) +
    BQ256XX_IPRECHG_OFFSET_uA;
    }
#[no_mangle]
unsafe extern "C" fn bq256xx_set_prechrg_curr(bq: *mut bq256xx_device, iprechg: c_int) -> c_int {
    static int bq256xx_set_prechrg_curr(struct bq256xx_device *bq, int iprechg)
    {
    unsigned int iprechg_reg_code;
    iprechg = clamp(iprechg, BQ256XX_IPRECHG_MIN_uA,
    BQ256XX_IPRECHG_MAX_uA);
    iprechg_reg_code = ((iprechg - BQ256XX_IPRECHG_OFFSET_uA) /
    BQ256XX_IPRECHG_STEP_uA) << BQ256XX_IPRECHG_BIT_SHIFT;
    return regmap_update_bits(bq.regmap, BQ256XX_PRECHG_AND_TERM_CURR_LIM,
    BQ256XX_IPRECHG_MASK, iprechg_reg_code);
    }
#[no_mangle]
unsafe extern "C" fn bq25618_619_get_prechrg_curr(bq: *mut bq256xx_device) -> c_int {
    static int bq25618_619_get_prechrg_curr(struct bq256xx_device *bq)
    {
    unsigned int prechg_and_term_curr_lim;
    unsigned int iprechg_reg_code;
    int ret;
    ret = regmap_read(bq.regmap, BQ256XX_PRECHG_AND_TERM_CURR_LIM,
    &prechg_and_term_curr_lim);
    if (ret)
    return ret;
    iprechg_reg_code = (prechg_and_term_curr_lim & BQ256XX_IPRECHG_MASK)
    >> BQ256XX_IPRECHG_BIT_SHIFT;
    return (iprechg_reg_code * BQ25618_IPRECHG_STEP_uA) +
    BQ25618_IPRECHG_OFFSET_uA;
    }
#[no_mangle]
unsafe extern "C" fn bq25618_619_set_prechrg_curr(bq: *mut bq256xx_device, iprechg: c_int) -> c_int {
    static int bq25618_619_set_prechrg_curr(struct bq256xx_device *bq, int iprechg)
    {
    unsigned int iprechg_reg_code;
    iprechg = clamp(iprechg, BQ25618_IPRECHG_MIN_uA,
    BQ25618_IPRECHG_MAX_uA);
    iprechg_reg_code = ((iprechg - BQ25618_IPRECHG_OFFSET_uA) /
    BQ25618_IPRECHG_STEP_uA) << BQ256XX_IPRECHG_BIT_SHIFT;
    return regmap_update_bits(bq.regmap, BQ256XX_PRECHG_AND_TERM_CURR_LIM,
    BQ256XX_IPRECHG_MASK, iprechg_reg_code);
    }
#[no_mangle]
unsafe extern "C" fn bq256xx_get_term_curr(bq: *mut bq256xx_device) -> c_int {
    static int bq256xx_get_term_curr(struct bq256xx_device *bq)
    {
    unsigned int prechg_and_term_curr_lim;
    unsigned int iterm_reg_code;
    int ret;
    ret = regmap_read(bq.regmap, BQ256XX_PRECHG_AND_TERM_CURR_LIM,
    &prechg_and_term_curr_lim);
    if (ret)
    return ret;
    iterm_reg_code = prechg_and_term_curr_lim & BQ256XX_ITERM_MASK;
    return (iterm_reg_code * BQ256XX_ITERM_STEP_uA) +
    BQ256XX_ITERM_OFFSET_uA;
    }
#[no_mangle]
unsafe extern "C" fn bq256xx_set_term_curr(bq: *mut bq256xx_device, iterm: c_int) -> c_int {
    static int bq256xx_set_term_curr(struct bq256xx_device *bq, int iterm)
    {
    unsigned int iterm_reg_code;
    iterm = clamp(iterm, BQ256XX_ITERM_MIN_uA, BQ256XX_ITERM_MAX_uA);
    iterm_reg_code = (iterm - BQ256XX_ITERM_OFFSET_uA) /
    BQ256XX_ITERM_STEP_uA;
    return regmap_update_bits(bq.regmap, BQ256XX_PRECHG_AND_TERM_CURR_LIM,
    BQ256XX_ITERM_MASK, iterm_reg_code);
    }
#[no_mangle]
unsafe extern "C" fn bq25618_619_get_term_curr(bq: *mut bq256xx_device) -> c_int {
    static int bq25618_619_get_term_curr(struct bq256xx_device *bq)
    {
    unsigned int prechg_and_term_curr_lim;
    unsigned int iterm_reg_code;
    int ret;
    ret = regmap_read(bq.regmap, BQ256XX_PRECHG_AND_TERM_CURR_LIM,
    &prechg_and_term_curr_lim);
    if (ret)
    return ret;
    iterm_reg_code = prechg_and_term_curr_lim & BQ256XX_ITERM_MASK;
    return (iterm_reg_code * BQ25618_ITERM_STEP_uA) +
    BQ25618_ITERM_OFFSET_uA;
    }
#[no_mangle]
unsafe extern "C" fn bq25618_619_set_term_curr(bq: *mut bq256xx_device, iterm: c_int) -> c_int {
    static int bq25618_619_set_term_curr(struct bq256xx_device *bq, int iterm)
    {
    unsigned int iterm_reg_code;
    iterm = clamp(iterm, BQ25618_ITERM_MIN_uA, BQ25618_ITERM_MAX_uA);
    iterm_reg_code = (iterm - BQ25618_ITERM_OFFSET_uA) /
    BQ25618_ITERM_STEP_uA;
    return regmap_update_bits(bq.regmap, BQ256XX_PRECHG_AND_TERM_CURR_LIM,
    BQ256XX_ITERM_MASK, iterm_reg_code);
    }
#[no_mangle]
unsafe extern "C" fn bq256xx_get_input_volt_lim(bq: *mut bq256xx_device) -> c_int {
    static int bq256xx_get_input_volt_lim(struct bq256xx_device *bq)
    {
    unsigned int charger_control_2;
    unsigned int vindpm_reg_code;
    int ret;
    ret = regmap_read(bq.regmap, BQ256XX_CHARGER_CONTROL_2,
    &charger_control_2);
    if (ret)
    return ret;
    vindpm_reg_code = charger_control_2 & BQ256XX_VINDPM_MASK;
    return (vindpm_reg_code * BQ256XX_VINDPM_STEP_uV) +
    BQ256XX_VINDPM_OFFSET_uV;
    }
#[no_mangle]
unsafe extern "C" fn bq256xx_set_input_volt_lim(bq: *mut bq256xx_device, vindpm: c_int) -> c_int {
    static int bq256xx_set_input_volt_lim(struct bq256xx_device *bq, int vindpm)
    {
    unsigned int vindpm_reg_code;
    vindpm = clamp(vindpm, BQ256XX_VINDPM_MIN_uV, BQ256XX_VINDPM_MAX_uV);
    vindpm_reg_code = (vindpm - BQ256XX_VINDPM_OFFSET_uV) /
    BQ256XX_VINDPM_STEP_uV;
    return regmap_update_bits(bq.regmap, BQ256XX_CHARGER_CONTROL_2,
    BQ256XX_VINDPM_MASK, vindpm_reg_code);
    }
#[no_mangle]
unsafe extern "C" fn bq256xx_get_input_curr_lim(bq: *mut bq256xx_device) -> c_int {
    static int bq256xx_get_input_curr_lim(struct bq256xx_device *bq)
    {
    unsigned int input_current_limit;
    unsigned int iindpm_reg_code;
    int ret;
    ret = regmap_read(bq.regmap, BQ256XX_INPUT_CURRENT_LIMIT,
    &input_current_limit);
    if (ret)
    return ret;
    iindpm_reg_code = input_current_limit & BQ256XX_IINDPM_MASK;
    return (iindpm_reg_code * BQ256XX_IINDPM_STEP_uA) +
    BQ256XX_IINDPM_OFFSET_uA;
    }
#[no_mangle]
unsafe extern "C" fn bq256xx_set_input_curr_lim(bq: *mut bq256xx_device, iindpm: c_int) -> c_int {
    static int bq256xx_set_input_curr_lim(struct bq256xx_device *bq, int iindpm)
    {
    unsigned int iindpm_reg_code;
    iindpm = clamp(iindpm, BQ256XX_IINDPM_MIN_uA, BQ256XX_IINDPM_MAX_uA);
    iindpm_reg_code = (iindpm - BQ256XX_IINDPM_OFFSET_uA) /
    BQ256XX_IINDPM_STEP_uA;
    return regmap_update_bits(bq.regmap, BQ256XX_INPUT_CURRENT_LIMIT,
    BQ256XX_IINDPM_MASK, iindpm_reg_code);
    }
#[no_mangle]
unsafe extern "C" fn bq256xx_charger_reset(data: *mut c_void) {
    static void bq256xx_charger_reset(void *data)
    {
    struct bq256xx_device *bq = data;
    regmap_update_bits(bq.regmap, BQ256XX_PART_INFORMATION,
    BQ256XX_REG_RST, BQ256XX_REG_RST);
    if (!IS_ERR_OR_NULL(bq.usb2_phy))
    usb_unregister_notifier(bq.usb2_phy, &bq.usb_nb);
    if (!IS_ERR_OR_NULL(bq.usb3_phy))
    usb_unregister_notifier(bq.usb3_phy, &bq.usb_nb);
    cancel_work_sync(&bq.usb_work);
    }
    static int bq256xx_set_charger_property(struct power_supply *psy,
    enum power_supply_property prop,
    const union power_supply_propval *val)
    {
    struct bq256xx_device *bq = power_supply_get_drvdata(psy);
    let mut ret: c_int = -EINVAL;
    switch (prop) {
    case POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT:
    ret = bq.chip_info.bq256xx_set_iindpm(bq, val.intval);
    if (ret)
    return ret;
    break;
    case POWER_SUPPLY_PROP_STATUS:
    break;
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE:
    ret = bq.chip_info.bq256xx_set_vbatreg(bq, val.intval);
    if (ret)
    return ret;
    break;
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT:
    ret = bq.chip_info.bq256xx_set_ichg(bq, val.intval);
    if (ret)
    return ret;
    break;
    case POWER_SUPPLY_PROP_PRECHARGE_CURRENT:
    ret = bq.chip_info.bq256xx_set_iprechg(bq, val.intval);
    if (ret)
    return ret;
    break;
    case POWER_SUPPLY_PROP_CHARGE_TERM_CURRENT:
    ret = bq.chip_info.bq256xx_set_iterm(bq, val.intval);
    if (ret)
    return ret;
    break;
    case POWER_SUPPLY_PROP_INPUT_VOLTAGE_LIMIT:
    ret = bq.chip_info.bq256xx_set_vindpm(bq, val.intval);
    if (ret)
    return ret;
    break;
    case POWER_SUPPLY_PROP_CHARGE_TYPE:
    ret = bq.chip_info.bq256xx_set_charge_type(bq, val.intval);
    if (ret)
    return ret;
    break;
    default:
    break;
    }
    return ret;
    }
    static int bq256xx_get_battery_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct bq256xx_device *bq = power_supply_get_drvdata(psy);
    switch (psp) {
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT_MAX:
    val.intval = bq.init_data.ichg_max;
    break;
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE_MAX:
    val.intval = bq.init_data.vbatreg_max;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int bq256xx_get_charger_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct bq256xx_device *bq = power_supply_get_drvdata(psy);
    struct bq256xx_state state;
    let mut ret: c_int = 0;
    mutex_lock(&bq.lock);
    ret = bq256xx_get_state(bq, &state);
    mutex_unlock(&bq.lock);
    if (ret)
    return ret;
    switch (psp) {
    case POWER_SUPPLY_PROP_STATUS:
    if (state.vbus_stat == BQ256XX_VBUS_STAT_NO_INPUT ||
    state.vbus_stat == BQ256XX_VBUS_STAT_USB_OTG)
    val.intval = POWER_SUPPLY_STATUS_DISCHARGING;
#[no_mangle]
pub unsafe extern "C" fn if(BQ256XX_CHRG_STAT_NOT_CHRGING: state.chrg_stat ==) -> else {
    else if (state.chrg_stat == BQ256XX_CHRG_STAT_NOT_CHRGING)
    val.intval = POWER_SUPPLY_STATUS_NOT_CHARGING;
#[no_mangle]
pub unsafe extern "C" fn if(BQ256XX_CHRG_STAT_CHRG_TERM: state.chrg_stat ==) -> else {
    else if (state.chrg_stat == BQ256XX_CHRG_STAT_CHRG_TERM)
    val.intval = POWER_SUPPLY_STATUS_FULL;
    else
    val.intval = POWER_SUPPLY_STATUS_CHARGING;
    break;
    case POWER_SUPPLY_PROP_HEALTH:
    val.intval = POWER_SUPPLY_HEALTH_UNKNOWN;
    if (state.wdt_fault) {
    val.intval =
    POWER_SUPPLY_HEALTH_WATCHDOG_TIMER_EXPIRE;
    } else if (state.bat_fault) {
    val.intval = POWER_SUPPLY_HEALTH_OVERVOLTAGE;
    } else {
    switch (state.chrg_stat) {
    case BQ256XX_CHRG_FAULT_INPUT:
    val.intval =
    POWER_SUPPLY_HEALTH_UNSPEC_FAILURE;
    break;
    case BQ256XX_CHRG_FAULT_THERM:
    val.intval = POWER_SUPPLY_HEALTH_OVERHEAT;
    break;
    case BQ256XX_CHRG_FAULT_CST_EXPIRE:
    val.intval =
    POWER_SUPPLY_HEALTH_SAFETY_TIMER_EXPIRE;
    break;
    default:
    break;
    }
    switch (state.ntc_fault) {
    case BQ256XX_NTC_FAULT_WARM:
    val.intval = POWER_SUPPLY_HEALTH_WARM;
    break;
    case BQ256XX_NTC_FAULT_COOL:
    val.intval = POWER_SUPPLY_HEALTH_COOL;
    break;
    case BQ256XX_NTC_FAULT_COLD:
    val.intval = POWER_SUPPLY_HEALTH_COLD;
    break;
    case BQ256XX_NTC_FAULT_HOT:
    val.intval = POWER_SUPPLY_HEALTH_HOT;
    break;
    default:
    val.intval = POWER_SUPPLY_HEALTH_GOOD;
    break;
    }
    }
    break;
    case POWER_SUPPLY_PROP_USB_TYPE:
    if (bq.chip_info.has_usb_detect) {
    switch (state.vbus_stat) {
    case BQ256XX_VBUS_STAT_USB_SDP:
    val.intval = POWER_SUPPLY_USB_TYPE_SDP;
    break;
    case BQ256XX_VBUS_STAT_USB_CDP:
    val.intval = POWER_SUPPLY_USB_TYPE_CDP;
    break;
    case BQ256XX_VBUS_STAT_USB_DCP:
    val.intval = POWER_SUPPLY_USB_TYPE_DCP;
    break;
    case BQ256XX_VBUS_STAT_USB_OTG:
    val.intval = POWER_SUPPLY_USB_TYPE_ACA;
    break;
    default:
    val.intval = POWER_SUPPLY_USB_TYPE_UNKNOWN;
    break;
    }
    } else {
    switch (state.vbus_stat) {
    case BQ256XX_VBUS_STAT_USB_SDP:
    val.intval = POWER_SUPPLY_USB_TYPE_SDP;
    break;
    case BQ256XX_VBUS_STAT_USB_OTG:
    val.intval = POWER_SUPPLY_USB_TYPE_ACA;
    break;
    default:
    val.intval = POWER_SUPPLY_USB_TYPE_UNKNOWN;
    break;
    }
    }
    break;
    case POWER_SUPPLY_PROP_CHARGE_TYPE:
    switch (state.chrg_stat) {
    case BQ256XX_CHRG_STAT_NOT_CHRGING:
    val.intval = POWER_SUPPLY_CHARGE_TYPE_NONE;
    break;
    case BQ256XX_CHRG_STAT_PRECHRGING:
    val.intval = POWER_SUPPLY_CHARGE_TYPE_TRICKLE;
    break;
    case BQ256XX_CHRG_STAT_FAST_CHRGING:
    val.intval = POWER_SUPPLY_CHARGE_TYPE_FAST;
    break;
    case BQ256XX_CHRG_STAT_CHRG_TERM:
    val.intval = POWER_SUPPLY_CHARGE_TYPE_TRICKLE;
    break;
    default:
    val.intval = POWER_SUPPLY_CHARGE_TYPE_UNKNOWN;
    }
    break;
    case POWER_SUPPLY_PROP_MANUFACTURER:
    val.strval = BQ256XX_MANUFACTURER;
    break;
    case POWER_SUPPLY_PROP_MODEL_NAME:
    val.strval = bq.model_name;
    break;
    case POWER_SUPPLY_PROP_ONLINE:
    val.intval = state.online;
    break;
    case POWER_SUPPLY_PROP_INPUT_VOLTAGE_LIMIT:
    ret = bq.chip_info.bq256xx_get_vindpm(bq);
    if (ret < 0)
    return ret;
    val.intval = ret;
    break;
    case POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT:
    ret = bq.chip_info.bq256xx_get_iindpm(bq);
    if (ret < 0)
    return ret;
    val.intval = ret;
    break;
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE:
    ret = bq.chip_info.bq256xx_get_vbatreg(bq);
    if (ret < 0)
    return ret;
    val.intval = ret;
    break;
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT:
    ret = bq.chip_info.bq256xx_get_ichg(bq);
    if (ret < 0)
    return ret;
    val.intval = ret;
    break;
    case POWER_SUPPLY_PROP_PRECHARGE_CURRENT:
    ret = bq.chip_info.bq256xx_get_iprechg(bq);
    if (ret < 0)
    return ret;
    val.intval = ret;
    break;
    case POWER_SUPPLY_PROP_CHARGE_TERM_CURRENT:
    ret = bq.chip_info.bq256xx_get_iterm(bq);
    if (ret < 0)
    return ret;
    val.intval = ret;
    break;
    default:
    return -EINVAL;
    }
    return ret;
    }
    static bool bq256xx_state_changed(struct bq256xx_device *bq,
    struct bq256xx_state *new_state)
    {
    struct bq256xx_state old_state;
    mutex_lock(&bq.lock);
    old_state = bq.state;
    mutex_unlock(&bq.lock);
    return memcmp(&old_state, new_state, sizeof(struct bq256xx_state)) != 0;
    }
#[no_mangle]
unsafe extern "C" fn bq256xx_irq_handler_thread(irq: c_int, private: *mut c_void) -> irqreturn_t {
    static irqreturn_t bq256xx_irq_handler_thread(int irq, void *private)
    {
    struct bq256xx_device *bq = private;
    struct bq256xx_state state;
    int ret;
    ret = bq256xx_get_state(bq, &state);
    if (ret < 0)
    goto irq_out;
    if (!bq256xx_state_changed(bq, &state))
    goto irq_out;
    mutex_lock(&bq.lock);
    bq.state = state;
    mutex_unlock(&bq.lock);
    power_supply_changed(bq.charger);
    irq_out:
    return IRQ_HANDLED;
    }
    static enum power_supply_property bq256xx_power_supply_props[] = {
    POWER_SUPPLY_PROP_MANUFACTURER,
    POWER_SUPPLY_PROP_MODEL_NAME,
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_ONLINE,
    POWER_SUPPLY_PROP_HEALTH,
    POWER_SUPPLY_PROP_INPUT_VOLTAGE_LIMIT,
    POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT,
    POWER_SUPPLY_PROP_CHARGE_TYPE,
    POWER_SUPPLY_PROP_USB_TYPE,
    POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT,
    POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE,
    POWER_SUPPLY_PROP_PRECHARGE_CURRENT,
    POWER_SUPPLY_PROP_CHARGE_TERM_CURRENT,
    };
    static enum power_supply_property bq256xx_battery_props[] = {
    POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT_MAX,
    POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE_MAX,
    };
    static int bq256xx_property_is_writeable(struct power_supply *psy,
    enum power_supply_property prop)
    {
    switch (prop) {
    case POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT:
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE:
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT:
    case POWER_SUPPLY_PROP_PRECHARGE_CURRENT:
    case POWER_SUPPLY_PROP_CHARGE_TERM_CURRENT:
    case POWER_SUPPLY_PROP_STATUS:
    case POWER_SUPPLY_PROP_INPUT_VOLTAGE_LIMIT:
    case POWER_SUPPLY_PROP_CHARGE_TYPE:
    return true;
    default:
    return false;
    }
    }
    static const struct power_supply_desc bq256xx_power_supply_desc = {
    .name = "bq256xx-charger",
    .type = POWER_SUPPLY_TYPE_USB,
    .usb_types = BIT(POWER_SUPPLY_USB_TYPE_SDP) |
    BIT(POWER_SUPPLY_USB_TYPE_CDP) |
    BIT(POWER_SUPPLY_USB_TYPE_DCP) |
    BIT(POWER_SUPPLY_USB_TYPE_ACA) |
    BIT(POWER_SUPPLY_USB_TYPE_UNKNOWN),
    .properties = bq256xx_power_supply_props,
    .num_properties = ARRAY_SIZE(bq256xx_power_supply_props),
    .get_property = bq256xx_get_charger_property,
    .set_property = bq256xx_set_charger_property,
    .property_is_writeable = bq256xx_property_is_writeable,
    };
    static struct power_supply_desc bq256xx_battery_desc = {
    .name			= "bq256xx-battery",
    .type			= POWER_SUPPLY_TYPE_BATTERY,
    .get_property		= bq256xx_get_battery_property,
    .properties		= bq256xx_battery_props,
    .num_properties		= ARRAY_SIZE(bq256xx_battery_props),
    .property_is_writeable	= bq256xx_property_is_writeable,
    };
#[no_mangle]
unsafe extern "C" fn bq256xx_is_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool bq256xx_is_volatile_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case BQ256XX_INPUT_CURRENT_LIMIT:
    case BQ256XX_CHARGER_STATUS_0...BQ256XX_CHARGER_STATUS_2:
    return true;
    default:
    return false;
    }
    }
    static const struct regmap_config bq25600_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = BQ256XX_PART_INFORMATION,
    .reg_defaults	= bq2560x_reg_defs,
    .num_reg_defaults = ARRAY_SIZE(bq2560x_reg_defs),
    .cache_type = REGCACHE_FLAT,
    .volatile_reg = bq256xx_is_volatile_reg,
    };
    static const struct regmap_config bq25611d_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = BQ256XX_CHARGER_CONTROL_4,
    .reg_defaults	= bq25611d_reg_defs,
    .num_reg_defaults = ARRAY_SIZE(bq25611d_reg_defs),
    .cache_type = REGCACHE_FLAT,
    .volatile_reg = bq256xx_is_volatile_reg,
    };
    static const struct regmap_config bq25618_619_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = BQ256XX_CHARGER_CONTROL_4,
    .reg_defaults	= bq25618_619_reg_defs,
    .num_reg_defaults = ARRAY_SIZE(bq25618_619_reg_defs),
    .cache_type = REGCACHE_FLAT,
    .volatile_reg = bq256xx_is_volatile_reg,
    };
    static const struct bq256xx_chip_info bq256xx_chip_info_tbl[] = {
    [BQ25600] = {
    .model_id = BQ25600,
    .bq256xx_regmap_config = &bq25600_regmap_config,
    .bq256xx_get_ichg = bq256xx_get_ichg_curr,
    .bq256xx_get_iindpm = bq256xx_get_input_curr_lim,
    .bq256xx_get_vbatreg = bq2560x_get_chrg_volt,
    .bq256xx_get_iterm = bq256xx_get_term_curr,
    .bq256xx_get_iprechg = bq256xx_get_prechrg_curr,
    .bq256xx_get_vindpm = bq256xx_get_input_volt_lim,
    .bq256xx_set_ts_ignore = core::ptr::null_mut(),
    .bq256xx_set_ichg = bq256xx_set_ichg_curr,
    .bq256xx_set_iindpm = bq256xx_set_input_curr_lim,
    .bq256xx_set_vbatreg = bq2560x_set_chrg_volt,
    .bq256xx_set_iterm = bq256xx_set_term_curr,
    .bq256xx_set_iprechg = bq256xx_set_prechrg_curr,
    .bq256xx_set_vindpm = bq256xx_set_input_volt_lim,
    .bq256xx_set_charge_type = bq256xx_set_charge_type,
    .bq256xx_def_ichg = BQ2560X_ICHG_DEF_uA,
    .bq256xx_def_iindpm = BQ256XX_IINDPM_DEF_uA,
    .bq256xx_def_vbatreg = BQ2560X_VBATREG_DEF_uV,
    .bq256xx_def_iterm = BQ256XX_ITERM_DEF_uA,
    .bq256xx_def_iprechg = BQ256XX_IPRECHG_DEF_uA,
    .bq256xx_def_vindpm = BQ256XX_VINDPM_DEF_uV,
    .bq256xx_max_ichg = BQ256XX_ICHG_MAX_uA,
    .bq256xx_max_vbatreg = BQ2560X_VBATREG_MAX_uV,
    .has_usb_detect = false,
    },
    [BQ25600D] = {
    .model_id = BQ25600D,
    .bq256xx_regmap_config = &bq25600_regmap_config,
    .bq256xx_get_ichg = bq256xx_get_ichg_curr,
    .bq256xx_get_iindpm = bq256xx_get_input_curr_lim,
    .bq256xx_get_vbatreg = bq2560x_get_chrg_volt,
    .bq256xx_get_iterm = bq256xx_get_term_curr,
    .bq256xx_get_iprechg = bq256xx_get_prechrg_curr,
    .bq256xx_get_vindpm = bq256xx_get_input_volt_lim,
    .bq256xx_set_ichg = bq256xx_set_ichg_curr,
    .bq256xx_set_iindpm = bq256xx_set_input_curr_lim,
    .bq256xx_set_vbatreg = bq2560x_set_chrg_volt,
    .bq256xx_set_iterm = bq256xx_set_term_curr,
    .bq256xx_set_iprechg = bq256xx_set_prechrg_curr,
    .bq256xx_set_vindpm = bq256xx_set_input_volt_lim,
    .bq256xx_set_charge_type = bq256xx_set_charge_type,
    .bq256xx_set_ts_ignore = core::ptr::null_mut(),
    .bq256xx_def_ichg = BQ2560X_ICHG_DEF_uA,
    .bq256xx_def_iindpm = BQ256XX_IINDPM_DEF_uA,
    .bq256xx_def_vbatreg = BQ2560X_VBATREG_DEF_uV,
    .bq256xx_def_iterm = BQ256XX_ITERM_DEF_uA,
    .bq256xx_def_iprechg = BQ256XX_IPRECHG_DEF_uA,
    .bq256xx_def_vindpm = BQ256XX_VINDPM_DEF_uV,
    .bq256xx_max_ichg = BQ256XX_ICHG_MAX_uA,
    .bq256xx_max_vbatreg = BQ2560X_VBATREG_MAX_uV,
    .has_usb_detect = true,
    },
    [BQ25601] = {
    .model_id = BQ25601,
    .bq256xx_regmap_config = &bq25600_regmap_config,
    .bq256xx_get_ichg = bq256xx_get_ichg_curr,
    .bq256xx_get_iindpm = bq256xx_get_input_curr_lim,
    .bq256xx_get_vbatreg = bq2560x_get_chrg_volt,
    .bq256xx_get_iterm = bq256xx_get_term_curr,
    .bq256xx_get_iprechg = bq256xx_get_prechrg_curr,
    .bq256xx_get_vindpm = bq256xx_get_input_volt_lim,
    .bq256xx_set_ichg = bq256xx_set_ichg_curr,
    .bq256xx_set_iindpm = bq256xx_set_input_curr_lim,
    .bq256xx_set_vbatreg = bq2560x_set_chrg_volt,
    .bq256xx_set_iterm = bq256xx_set_term_curr,
    .bq256xx_set_iprechg = bq256xx_set_prechrg_curr,
    .bq256xx_set_vindpm = bq256xx_set_input_volt_lim,
    .bq256xx_set_charge_type = bq256xx_set_charge_type,
    .bq256xx_set_ts_ignore = core::ptr::null_mut(),
    .bq256xx_def_ichg = BQ2560X_ICHG_DEF_uA,
    .bq256xx_def_iindpm = BQ256XX_IINDPM_DEF_uA,
    .bq256xx_def_vbatreg = BQ2560X_VBATREG_DEF_uV,
    .bq256xx_def_iterm = BQ256XX_ITERM_DEF_uA,
    .bq256xx_def_iprechg = BQ256XX_IPRECHG_DEF_uA,
    .bq256xx_def_vindpm = BQ256XX_VINDPM_DEF_uV,
    .bq256xx_max_ichg = BQ256XX_ICHG_MAX_uA,
    .bq256xx_max_vbatreg = BQ2560X_VBATREG_MAX_uV,
    .has_usb_detect = false,
    },
    [BQ25601D] = {
    .model_id = BQ25601D,
    .bq256xx_regmap_config = &bq25600_regmap_config,
    .bq256xx_get_ichg = bq256xx_get_ichg_curr,
    .bq256xx_get_iindpm = bq256xx_get_input_curr_lim,
    .bq256xx_get_vbatreg = bq25601d_get_chrg_volt,
    .bq256xx_get_iterm = bq256xx_get_term_curr,
    .bq256xx_get_iprechg = bq256xx_get_prechrg_curr,
    .bq256xx_get_vindpm = bq256xx_get_input_volt_lim,
    .bq256xx_set_ichg = bq256xx_set_ichg_curr,
    .bq256xx_set_iindpm = bq256xx_set_input_curr_lim,
    .bq256xx_set_vbatreg = bq25601d_set_chrg_volt,
    .bq256xx_set_iterm = bq256xx_set_term_curr,
    .bq256xx_set_iprechg = bq256xx_set_prechrg_curr,
    .bq256xx_set_vindpm = bq256xx_set_input_volt_lim,
    .bq256xx_set_charge_type = bq256xx_set_charge_type,
    .bq256xx_set_ts_ignore = core::ptr::null_mut(),
    .bq256xx_def_ichg = BQ2560X_ICHG_DEF_uA,
    .bq256xx_def_iindpm = BQ256XX_IINDPM_DEF_uA,
    .bq256xx_def_vbatreg = BQ2560X_VBATREG_DEF_uV,
    .bq256xx_def_iterm = BQ256XX_ITERM_DEF_uA,
    .bq256xx_def_iprechg = BQ256XX_IPRECHG_DEF_uA,
    .bq256xx_def_vindpm = BQ256XX_VINDPM_DEF_uV,
    .bq256xx_max_ichg = BQ256XX_ICHG_MAX_uA,
    .bq256xx_max_vbatreg = BQ2560X_VBATREG_MAX_uV,
    .has_usb_detect = true,
    },
    [BQ25611D] = {
    .model_id = BQ25611D,
    .bq256xx_regmap_config = &bq25611d_regmap_config,
    .bq256xx_get_ichg = bq256xx_get_ichg_curr,
    .bq256xx_get_iindpm = bq256xx_get_input_curr_lim,
    .bq256xx_get_vbatreg = bq25611d_get_chrg_volt,
    .bq256xx_get_iterm = bq256xx_get_term_curr,
    .bq256xx_get_iprechg = bq256xx_get_prechrg_curr,
    .bq256xx_get_vindpm = bq256xx_get_input_volt_lim,
    .bq256xx_set_ichg = bq256xx_set_ichg_curr,
    .bq256xx_set_iindpm = bq256xx_set_input_curr_lim,
    .bq256xx_set_vbatreg = bq25611d_set_chrg_volt,
    .bq256xx_set_iterm = bq256xx_set_term_curr,
    .bq256xx_set_iprechg = bq256xx_set_prechrg_curr,
    .bq256xx_set_vindpm = bq256xx_set_input_volt_lim,
    .bq256xx_set_charge_type = bq256xx_set_charge_type,
    .bq256xx_set_ts_ignore = bq256xx_set_ts_ignore,
    .bq256xx_def_ichg = BQ25611D_ICHG_DEF_uA,
    .bq256xx_def_iindpm = BQ256XX_IINDPM_DEF_uA,
    .bq256xx_def_vbatreg = BQ25611D_VBATREG_DEF_uV,
    .bq256xx_def_iterm = BQ256XX_ITERM_DEF_uA,
    .bq256xx_def_iprechg = BQ256XX_IPRECHG_DEF_uA,
    .bq256xx_def_vindpm = BQ256XX_VINDPM_DEF_uV,
    .bq256xx_max_ichg = BQ256XX_ICHG_MAX_uA,
    .bq256xx_max_vbatreg = BQ25611D_VBATREG_MAX_uV,
    .has_usb_detect = true,
    },
    [BQ25618] = {
    .model_id = BQ25618,
    .bq256xx_regmap_config = &bq25618_619_regmap_config,
    .bq256xx_get_ichg = bq25618_619_get_ichg_curr,
    .bq256xx_get_iindpm = bq256xx_get_input_curr_lim,
    .bq256xx_get_vbatreg = bq25618_619_get_chrg_volt,
    .bq256xx_get_iterm = bq25618_619_get_term_curr,
    .bq256xx_get_iprechg = bq25618_619_get_prechrg_curr,
    .bq256xx_get_vindpm = bq256xx_get_input_volt_lim,
    .bq256xx_set_ichg = bq25618_619_set_ichg_curr,
    .bq256xx_set_iindpm = bq256xx_set_input_curr_lim,
    .bq256xx_set_vbatreg = bq25618_619_set_chrg_volt,
    .bq256xx_set_iterm = bq25618_619_set_term_curr,
    .bq256xx_set_iprechg = bq25618_619_set_prechrg_curr,
    .bq256xx_set_vindpm = bq256xx_set_input_volt_lim,
    .bq256xx_set_charge_type = bq256xx_set_charge_type,
    .bq256xx_set_ts_ignore = bq256xx_set_ts_ignore,
    .bq256xx_def_ichg = BQ25618_ICHG_DEF_uA,
    .bq256xx_def_iindpm = BQ256XX_IINDPM_DEF_uA,
    .bq256xx_def_vbatreg = BQ25618_VBATREG_DEF_uV,
    .bq256xx_def_iterm = BQ25618_ITERM_DEF_uA,
    .bq256xx_def_iprechg = BQ25618_IPRECHG_DEF_uA,
    .bq256xx_def_vindpm = BQ256XX_VINDPM_DEF_uV,
    .bq256xx_max_ichg = BQ25618_ICHG_MAX_uA,
    .bq256xx_max_vbatreg = BQ25618_VBATREG_MAX_uV,
    .has_usb_detect = false,
    },
    [BQ25619] = {
    .model_id = BQ25619,
    .bq256xx_regmap_config = &bq25618_619_regmap_config,
    .bq256xx_get_ichg = bq25618_619_get_ichg_curr,
    .bq256xx_get_iindpm = bq256xx_get_input_curr_lim,
    .bq256xx_get_vbatreg = bq25618_619_get_chrg_volt,
    .bq256xx_get_iterm = bq25618_619_get_term_curr,
    .bq256xx_get_iprechg = bq25618_619_get_prechrg_curr,
    .bq256xx_get_vindpm = bq256xx_get_input_volt_lim,
    .bq256xx_set_ichg = bq25618_619_set_ichg_curr,
    .bq256xx_set_iindpm = bq256xx_set_input_curr_lim,
    .bq256xx_set_vbatreg = bq25618_619_set_chrg_volt,
    .bq256xx_set_iterm = bq25618_619_set_term_curr,
    .bq256xx_set_iprechg = bq25618_619_set_prechrg_curr,
    .bq256xx_set_vindpm = bq256xx_set_input_volt_lim,
    .bq256xx_set_charge_type = bq256xx_set_charge_type,
    .bq256xx_set_ts_ignore = bq256xx_set_ts_ignore,
    .bq256xx_def_ichg = BQ25618_ICHG_DEF_uA,
    .bq256xx_def_iindpm = BQ256XX_IINDPM_DEF_uA,
    .bq256xx_def_vbatreg = BQ25618_VBATREG_DEF_uV,
    .bq256xx_def_iterm = BQ25618_ITERM_DEF_uA,
    .bq256xx_def_iprechg = BQ25618_IPRECHG_DEF_uA,
    .bq256xx_def_vindpm = BQ256XX_VINDPM_DEF_uV,
    .bq256xx_max_ichg = BQ25618_ICHG_MAX_uA,
    .bq256xx_max_vbatreg = BQ25618_VBATREG_MAX_uV,
    .has_usb_detect = false,
    },
    };
    static int bq256xx_power_supply_init(struct bq256xx_device *bq,
    struct power_supply_config *psy_cfg, struct device *dev)
    {
    bq.charger = devm_power_supply_register(bq.dev,
    &bq256xx_power_supply_desc,
    psy_cfg);
    if (IS_ERR(bq.charger)) {
    dev_err(dev, "power supply register charger failed\n");
    return PTR_ERR(bq.charger);
    }
    bq.battery = devm_power_supply_register(bq.dev,
    &bq256xx_battery_desc,
    psy_cfg);
    if (IS_ERR(bq.battery)) {
    dev_err(dev, "power supply register battery failed\n");
    return PTR_ERR(bq.battery);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bq256xx_hw_init(bq: *mut bq256xx_device) -> c_int {
    static int bq256xx_hw_init(struct bq256xx_device *bq)
    {
    struct power_supply_battery_info *bat_info;
    let mut wd_reg_val: c_int = BQ256XX_WATCHDOG_DIS;
    let mut ret: c_int = 0;
    int i;
    for (i = 0; i < BQ256XX_NUM_WD_VAL; i++) {
    if (bq.watchdog_timer == bq256xx_watchdog_time[i]) {
    wd_reg_val = i;
    break;
    }
    if (i + 1 < BQ256XX_NUM_WD_VAL &&
    bq.watchdog_timer > bq256xx_watchdog_time[i] &&
    bq.watchdog_timer < bq256xx_watchdog_time[i + 1])
    wd_reg_val = i;
    }
    ret = regmap_update_bits(bq.regmap, BQ256XX_CHARGER_CONTROL_1,
    BQ256XX_WATCHDOG_MASK, wd_reg_val <<
    BQ256XX_WDT_BIT_SHIFT);
    if (ret)
    return ret;
    ret = power_supply_get_battery_info(bq.charger, &bat_info);
    if (ret == -ENOMEM)
    return ret;
    if (ret) {
    dev_warn(bq.dev, "battery info missing, default values will be applied\n");
    bat_info.constant_charge_current_max_ua =
    bq.chip_info.bq256xx_def_ichg;
    bat_info.constant_charge_voltage_max_uv =
    bq.chip_info.bq256xx_def_vbatreg;
    bat_info.precharge_current_ua =
    bq.chip_info.bq256xx_def_iprechg;
    bat_info.charge_term_current_ua =
    bq.chip_info.bq256xx_def_iterm;
    bq.init_data.ichg_max =
    bq.chip_info.bq256xx_max_ichg;
    bq.init_data.vbatreg_max =
    bq.chip_info.bq256xx_max_vbatreg;
    } else {
    bq.init_data.ichg_max =
    bat_info.constant_charge_current_max_ua;
    bq.init_data.vbatreg_max =
    bat_info.constant_charge_voltage_max_uv;
    }
    ret = bq.chip_info.bq256xx_set_vindpm(bq, bq.init_data.vindpm);
    if (ret)
    return ret;
    ret = bq.chip_info.bq256xx_set_iindpm(bq, bq.init_data.iindpm);
    if (ret)
    return ret;
    ret = bq.chip_info.bq256xx_set_ichg(bq,
    bq.chip_info.bq256xx_def_ichg);
    if (ret)
    return ret;
    ret = bq.chip_info.bq256xx_set_iprechg(bq,
    bat_info.precharge_current_ua);
    if (ret)
    return ret;
    ret = bq.chip_info.bq256xx_set_vbatreg(bq,
    bq.chip_info.bq256xx_def_vbatreg);
    if (ret)
    return ret;
    ret = bq.chip_info.bq256xx_set_iterm(bq,
    bat_info.charge_term_current_ua);
    if (ret)
    return ret;
    if (bq.chip_info.bq256xx_set_ts_ignore) {
    ret = bq.chip_info.bq256xx_set_ts_ignore(bq, bq.init_data.ts_ignore);
    if (ret)
    return ret;
    }
    power_supply_put_battery_info(bq.charger, bat_info);
    return 0;
    }
    static int bq256xx_parse_dt(struct bq256xx_device *bq,
    struct power_supply_config *psy_cfg, struct device *dev)
    {
    let mut ret: c_int = 0;
    psy_cfg.drv_data = bq;
    psy_cfg.fwnode = dev_fwnode(dev);
    ret = device_property_read_u32(bq.dev, "ti,watchdog-timeout-ms",
    &bq.watchdog_timer);
    if (ret)
    bq.watchdog_timer = BQ256XX_WATCHDOG_DIS;
    if (bq.watchdog_timer > BQ256XX_WATCHDOG_MAX ||
    bq.watchdog_timer < BQ256XX_WATCHDOG_DIS)
    return -EINVAL;
    ret = device_property_read_u32(bq.dev,
    "input-voltage-limit-microvolt",
    &bq.init_data.vindpm);
    if (ret)
    bq.init_data.vindpm = bq.chip_info.bq256xx_def_vindpm;
    ret = device_property_read_u32(bq.dev,
    "input-current-limit-microamp",
    &bq.init_data.iindpm);
    if (ret)
    bq.init_data.iindpm = bq.chip_info.bq256xx_def_iindpm;
    bq.init_data.ts_ignore = device_property_read_bool(bq.dev, "ti,no-thermistor");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bq256xx_probe(client: *mut i2c_client) -> c_int {
    static int bq256xx_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    struct device *dev = &client.dev;
    struct bq256xx_device *bq;
    let mut psy_cfg: power_supply_config = { };
    int ret;
    bq = devm_kzalloc(dev, sizeof(*bq), GFP_KERNEL);
    if (!bq)
    return -ENOMEM;
    bq.client = client;
    bq.dev = dev;
    bq.chip_info = i2c_get_match_data(client);
    mutex_init(&bq.lock);
    strscpy(bq.model_name, id.name, sizeof(bq.model_name));
    bq.regmap = devm_regmap_init_i2c(client,
    bq.chip_info.bq256xx_regmap_config);
    if (IS_ERR(bq.regmap)) {
    dev_err(dev, "Failed to allocate register map\n");
    return PTR_ERR(bq.regmap);
    }
    i2c_set_clientdata(client, bq);
    ret = bq256xx_parse_dt(bq, &psy_cfg, dev);
    if (ret) {
    dev_err(dev, "Failed to read device tree properties%d\n", ret);
    return ret;
    }
    INIT_WORK(&bq.usb_work, bq256xx_usb_work);
    bq.usb_nb.notifier_call = bq256xx_usb_notifier;
// OTG reporting
    bq.usb2_phy = devm_usb_get_phy(dev, USB_PHY_TYPE_USB2);
    bq.usb3_phy = devm_usb_get_phy(dev, USB_PHY_TYPE_USB3);
    ret = bq256xx_power_supply_init(bq, &psy_cfg, dev);
    if (ret) {
    dev_err(dev, "Failed to register power supply\n");
    return ret;
    }
// Register after the power supplies so devm runs it first.
    ret = devm_add_action_or_reset(dev, bq256xx_charger_reset, bq);
    if (ret)
    return ret;
    if (!IS_ERR_OR_NULL(bq.usb2_phy))
    usb_register_notifier(bq.usb2_phy, &bq.usb_nb);
    if (!IS_ERR_OR_NULL(bq.usb3_phy))
    usb_register_notifier(bq.usb3_phy, &bq.usb_nb);
    if (client.irq) {
    ret = devm_request_threaded_irq(dev, client.irq, core::ptr::null_mut(),
    bq256xx_irq_handler_thread,
    IRQF_TRIGGER_FALLING |
    IRQF_ONESHOT,
    dev_name(&client.dev), bq);
    if (ret < 0)
    return ret;
    }
    ret = bq256xx_hw_init(bq);
    if (ret) {
    dev_err(dev, "Cannot initialize the chip.\n");
    return ret;
    }
    return ret;
    }
    static const struct i2c_device_id bq256xx_i2c_ids[] = {
    { .name = "bq25600", .driver_data = (kernel_ulong_t)&bq256xx_chip_info_tbl[BQ25600] },
    { .name = "bq25600d", .driver_data = (kernel_ulong_t)&bq256xx_chip_info_tbl[BQ25600D] },
    { .name = "bq25601", .driver_data = (kernel_ulong_t)&bq256xx_chip_info_tbl[BQ25601] },
    { .name = "bq25601d", .driver_data = (kernel_ulong_t)&bq256xx_chip_info_tbl[BQ25601D] },
    { .name = "bq25611d", .driver_data = (kernel_ulong_t)&bq256xx_chip_info_tbl[BQ25611D] },
    { .name = "bq25618", .driver_data = (kernel_ulong_t)&bq256xx_chip_info_tbl[BQ25618] },
    { .name = "bq25619", .driver_data = (kernel_ulong_t)&bq256xx_chip_info_tbl[BQ25619] },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, bq256xx_i2c_ids);
    static const struct of_device_id bq256xx_of_match[] = {
    { .compatible = "ti,bq25600", .data = &bq256xx_chip_info_tbl[BQ25600] },
    { .compatible = "ti,bq25600d", .data = &bq256xx_chip_info_tbl[BQ25600D] },
    { .compatible = "ti,bq25601", .data = &bq256xx_chip_info_tbl[BQ25601] },
    { .compatible = "ti,bq25601d", .data = &bq256xx_chip_info_tbl[BQ25601D] },
    { .compatible = "ti,bq25611d", .data = &bq256xx_chip_info_tbl[BQ25611D] },
    { .compatible = "ti,bq25618", .data = &bq256xx_chip_info_tbl[BQ25618] },
    { .compatible = "ti,bq25619", .data = &bq256xx_chip_info_tbl[BQ25619] },
    {}
    };
    MODULE_DEVICE_TABLE(of, bq256xx_of_match);
    static const struct acpi_device_id bq256xx_acpi_match[] = {
    { "bq25600", (kernel_ulong_t)&bq256xx_chip_info_tbl[BQ25600] },
    { "bq25600d", (kernel_ulong_t)&bq256xx_chip_info_tbl[BQ25600D] },
    { "bq25601", (kernel_ulong_t)&bq256xx_chip_info_tbl[BQ25601] },
    { "bq25601d", (kernel_ulong_t)&bq256xx_chip_info_tbl[BQ25601D] },
    { "bq25611d", (kernel_ulong_t)&bq256xx_chip_info_tbl[BQ25611D] },
    { "bq25618", (kernel_ulong_t)&bq256xx_chip_info_tbl[BQ25618] },
    { "bq25619", (kernel_ulong_t)&bq256xx_chip_info_tbl[BQ25619] },
    {}
    };
    MODULE_DEVICE_TABLE(acpi, bq256xx_acpi_match);
    static struct i2c_driver bq256xx_driver = {
    .driver = {
    .name = "bq256xx-charger",
    .of_match_table = bq256xx_of_match,
    .acpi_match_table = bq256xx_acpi_match,
    },
    .probe = bq256xx_probe,
    .id_table = bq256xx_i2c_ids,
    };
    module_i2c_driver(bq256xx_driver);
    MODULE_AUTHOR("Ricardo Rivera-Matos <r-rivera-matos@ti.com>");
    MODULE_DESCRIPTION("bq256xx charger driver");
    MODULE_LICENSE("GPL v2");
