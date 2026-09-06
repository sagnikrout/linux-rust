//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/smb347-charger.c
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
// Summit Microelectronics SMB347 Battery Charger Driver
//
// Copyright (C) 2011, Intel Corporation
//
// Authors: Bruce E. Robertson <bruce.e.robertson@intel.com>
// Mika Westerberg <mika.westerberg@linux.intel.com>
//

// Use the default compensation method

// Use default factory programmed value for hard/soft temperature limit

//
// Configuration registers. These are mirrored to volatile RAM and can be
// written once %CMD_A_ALLOW_WRITE is set in %CMD_A register. They will be
// reloaded from non-volatile registers after POR.
//
pub const CFG_CHARGE_CURRENT: c_uint = 0x00;
pub const CFG_CHARGE_CURRENT_FCC_MASK: c_uint = 0xe0;
pub const CFG_CHARGE_CURRENT_FCC_SHIFT: c_int = 5;
pub const CFG_CHARGE_CURRENT_PCC_MASK: c_uint = 0x18;
pub const CFG_CHARGE_CURRENT_PCC_SHIFT: c_int = 3;
pub const CFG_CHARGE_CURRENT_TC_MASK: c_uint = 0x07;
pub const CFG_CURRENT_LIMIT: c_uint = 0x01;
pub const CFG_CURRENT_LIMIT_DC_MASK: c_uint = 0xf0;
pub const CFG_CURRENT_LIMIT_DC_SHIFT: c_int = 4;
pub const CFG_CURRENT_LIMIT_USB_MASK: c_uint = 0x0f;
pub const CFG_FLOAT_VOLTAGE: c_uint = 0x03;
pub const CFG_FLOAT_VOLTAGE_FLOAT_MASK: c_uint = 0x3f;
pub const CFG_FLOAT_VOLTAGE_THRESHOLD_MASK: c_uint = 0xc0;
pub const CFG_FLOAT_VOLTAGE_THRESHOLD_SHIFT: c_int = 6;
pub const CFG_STAT: c_uint = 0x05;

pub const CFG_PIN: c_uint = 0x06;
pub const CFG_PIN_EN_CTRL_MASK: c_uint = 0x60;
pub const CFG_PIN_EN_CTRL_ACTIVE_HIGH: c_uint = 0x40;
pub const CFG_PIN_EN_CTRL_ACTIVE_LOW: c_uint = 0x60;

pub const CFG_THERM: c_uint = 0x07;
pub const CFG_THERM_SOFT_HOT_COMPENSATION_MASK: c_uint = 0x03;
pub const CFG_THERM_SOFT_HOT_COMPENSATION_SHIFT: c_int = 0;
pub const CFG_THERM_SOFT_COLD_COMPENSATION_MASK: c_uint = 0x0c;
pub const CFG_THERM_SOFT_COLD_COMPENSATION_SHIFT: c_int = 2;

pub const CFG_SYSOK: c_uint = 0x08;

pub const CFG_OTHER: c_uint = 0x09;
pub const CFG_OTHER_RID_MASK: c_uint = 0xc0;
pub const CFG_OTHER_RID_ENABLED_AUTO_OTG: c_uint = 0xc0;
pub const CFG_OTG: c_uint = 0x0a;
pub const CFG_OTG_TEMP_THRESHOLD_MASK: c_uint = 0x30;

pub const CFG_OTG_TEMP_THRESHOLD_SHIFT: c_int = 4;
pub const CFG_OTG_CC_COMPENSATION_MASK: c_uint = 0xc0;
pub const CFG_OTG_CC_COMPENSATION_SHIFT: c_int = 6;
pub const CFG_TEMP_LIMIT: c_uint = 0x0b;
pub const CFG_TEMP_LIMIT_SOFT_HOT_MASK: c_uint = 0x03;
pub const CFG_TEMP_LIMIT_SOFT_HOT_SHIFT: c_int = 0;
pub const CFG_TEMP_LIMIT_SOFT_COLD_MASK: c_uint = 0x0c;
pub const CFG_TEMP_LIMIT_SOFT_COLD_SHIFT: c_int = 2;
pub const CFG_TEMP_LIMIT_HARD_HOT_MASK: c_uint = 0x30;
pub const CFG_TEMP_LIMIT_HARD_HOT_SHIFT: c_int = 4;
pub const CFG_TEMP_LIMIT_HARD_COLD_MASK: c_uint = 0xc0;
pub const CFG_TEMP_LIMIT_HARD_COLD_SHIFT: c_int = 6;
pub const CFG_FAULT_IRQ: c_uint = 0x0c;

pub const CFG_STATUS_IRQ: c_uint = 0x0d;

pub const CFG_ADDRESS: c_uint = 0x0e;
// Command registers
pub const CMD_A: c_uint = 0x30;

pub const CMD_B: c_uint = 0x31;
pub const CMD_C: c_uint = 0x33;
// Interrupt Status registers
pub const IRQSTAT_A: c_uint = 0x35;
pub const IRQSTAT_C: c_uint = 0x37;

pub const IRQSTAT_D: c_uint = 0x38;

pub const IRQSTAT_E: c_uint = 0x39;

pub const IRQSTAT_F: c_uint = 0x3a;
// Status registers
pub const STAT_A: c_uint = 0x3b;
pub const STAT_A_FLOAT_VOLTAGE_MASK: c_uint = 0x3f;
pub const STAT_B: c_uint = 0x3c;
pub const STAT_C: c_uint = 0x3d;

pub const STAT_C_CHG_MASK: c_uint = 0x06;
pub const STAT_C_CHG_SHIFT: c_int = 1;

pub const STAT_E: c_uint = 0x3f;
pub const SMB347_MAX_REGISTER: c_uint = 0x3f;
//
// struct smb347_charger - smb347 charger instance
// @dev: pointer to device
// @regmap: pointer to driver regmap
// @mains: power_supply instance for AC/DC power
// @usb: power_supply instance for USB power
// @usb_rdev: USB VBUS regulator device
// @id: SMB charger ID
// @mains_online: is AC/DC input connected
// @usb_online: is USB input connected
// @irq_unsupported: is interrupt unsupported by SMB hardware
// @usb_vbus_enabled: is USB VBUS powered by SMB charger
// @max_charge_current: maximum current (in uA) the battery can be charged
// @max_charge_voltage: maximum voltage (in uV) the battery can be charged
// @pre_charge_current: current (in uA) to use in pre-charging phase
// @termination_current: current (in uA) used to determine when the
// charging cycle terminates
// @pre_to_fast_voltage: voltage (in uV) treshold used for transitioning to
// pre-charge to fast charge mode
// @mains_current_limit: maximum input current drawn from AC/DC input (in uA)
// @usb_hc_current_limit: maximum input high current (in uA) drawn from USB
// input
// @chip_temp_threshold: die temperature where device starts limiting charge
// current [%100 - %130] (in degree C)
// @soft_cold_temp_limit: soft cold temperature limit [%0 - %15] (in degree C),
// granularity is 5 deg C.
// @soft_hot_temp_limit: soft hot temperature limit [%40 - %55] (in degree  C),
// granularity is 5 deg C.
// @hard_cold_temp_limit: hard cold temperature limit [%-5 - %10] (in degree C),
// granularity is 5 deg C.
// @hard_hot_temp_limit: hard hot temperature limit [%50 - %65] (in degree C),
// granularity is 5 deg C.
// @suspend_on_hard_temp_limit: suspend charging when hard limit is hit
// @soft_temp_limit_compensation: compensation method when soft temperature
// limit is hit
// @charge_current_compensation: current (in uA) for charging compensation
// current when temperature hits soft limits
// @use_mains: AC/DC input can be used
// @use_usb: USB input can be used
// @use_usb_otg: USB OTG output can be used (not implemented yet)
// @enable_control: how charging enable/disable is controlled
// (driver/pin controls)
// @inok_polarity: polarity of INOK signal which denotes presence of external
// power supply
//
// @use_main, @use_usb, and @use_usb_otg are means to enable/disable
// hardware support for these. This is useful when we want to have for
// example OTG charging controlled via OTG transceiver driver and not by
// the SMB347 hardware.
//
// Hard and soft temperature limit values are given as described in the
// device data sheet and assuming NTC beta value is %3750. Even if this is
// not the case, these values should be used. They can be mapped to the
// corresponding NTC beta values with the help of table %2 in the data
// sheet. So for example if NTC beta is %3375 and we want to program hard
// hot limit to be %53 deg C, @hard_hot_temp_limit should be set to %50.
//
// If zero value is given in any of the current and voltage values, the
// factory programmed default will be used. For soft/hard temperature
// values, pass in %SMB3XX_TEMP_USE_DEFAULT instead.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb347_charger {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub mains: *mut power_supply,
    pub usb: *mut power_supply,
    pub usb_rdev: *mut regulator_dev,
    pub id: c_uint,
    pub mains_online: bool,
    pub usb_online: bool,
    pub irq_unsupported: bool,
    pub usb_vbus_enabled: bool,
    pub max_charge_current: c_uint,
    pub max_charge_voltage: c_uint,
    pub pre_charge_current: c_uint,
    pub termination_current: c_uint,
    pub pre_to_fast_voltage: c_uint,
    pub mains_current_limit: c_uint,
    pub usb_hc_current_limit: c_uint,
    pub chip_temp_threshold: c_uint,
    pub soft_cold_temp_limit: c_int,
    pub soft_hot_temp_limit: c_int,
    pub hard_cold_temp_limit: c_int,
    pub hard_hot_temp_limit: c_int,
    pub suspend_on_hard_temp_limit: bool,
    pub soft_temp_limit_compensation: c_uint,
    pub charge_current_compensation: c_uint,
    pub use_mains: bool,
    pub use_usb: bool,
    pub use_usb_otg: bool,
    pub enable_control: c_uint,
    pub inok_polarity: c_uint,
}

    enum smb_charger_chipid {
    SMB345,
    SMB347,
    SMB358,
    NUM_CHIP_TYPES,
    };
// Fast charge current in uA
    static const unsigned int fcc_tbl[NUM_CHIP_TYPES][8] = {
    [SMB345] = {  200000,  450000,  600000,  900000,
    1300000, 1500000, 1800000, 2000000 },
    [SMB347] = {  700000,  900000, 1200000, 1500000,
    1800000, 2000000, 2200000, 2500000 },
    [SMB358] = {  200000,  450000,  600000,  900000,
    1300000, 1500000, 1800000, 2000000 },
    };
// Pre-charge current in uA
    static const unsigned int pcc_tbl[NUM_CHIP_TYPES][4] = {
    [SMB345] = { 150000, 250000, 350000, 450000 },
    [SMB347] = { 100000, 150000, 200000, 250000 },
    [SMB358] = { 150000, 250000, 350000, 450000 },
    };
// Termination current in uA
    static const unsigned int tc_tbl[NUM_CHIP_TYPES][8] = {
    [SMB345] = {  30000,  40000,  60000,  80000,
    100000, 125000, 150000, 200000 },
    [SMB347] = {  37500,  50000, 100000, 150000,
    200000, 250000, 500000, 600000 },
    [SMB358] = {  30000,  40000,  60000,  80000,
    100000, 125000, 150000, 200000 },
    };
// Input current limit in uA
    static const unsigned int icl_tbl[NUM_CHIP_TYPES][10] = {
    [SMB345] = {  300000,  500000,  700000, 1000000, 1500000,
    1800000, 2000000, 2000000, 2000000, 2000000 },
    [SMB347] = {  300000,  500000,  700000,  900000, 1200000,
    1500000, 1800000, 2000000, 2200000, 2500000 },
    [SMB358] = {  300000,  500000,  700000, 1000000, 1500000,
    1800000, 2000000, 2000000, 2000000, 2000000 },
    };
// Charge current compensation in uA
    static const unsigned int ccc_tbl[NUM_CHIP_TYPES][4] = {
    [SMB345] = {  200000,  450000,  600000,  900000 },
    [SMB347] = {  250000,  700000,  900000, 1200000 },
    [SMB358] = {  200000,  450000,  600000,  900000 },
    };
// Convert register value to current using lookup table
#[no_mangle]
unsafe extern "C" fn hw_to_current(tbl: *const c_uint, size: usize, val: c_uint) -> c_int {
    static int hw_to_current(const unsigned int *tbl, size_t size, unsigned int val)
    {
    if (val >= size)
    return -EINVAL;
    return tbl[val];
    }
// Convert current to register value using lookup table
#[no_mangle]
unsafe extern "C" fn current_to_hw(tbl: *const c_uint, size: usize, val: c_uint) -> c_int {
    static int current_to_hw(const unsigned int *tbl, size_t size, unsigned int val)
    {
    size_t i;
    for (i = 0; i < size; i++)
    if (val < tbl[i])
    break;
    return i > 0 ? i - 1 : -EINVAL;
    }
//
// smb347_update_ps_status - refreshes the power source status
// @smb: pointer to smb347 charger instance
//
// Function checks whether any power source is connected to the charger and
// updates internal state accordingly. If there is a change to previous state
// function returns %1, otherwise %0 and negative errno in case of errror.
//
#[no_mangle]
unsafe extern "C" fn smb347_update_ps_status(smb: *mut smb347_charger) -> c_int {
    static int smb347_update_ps_status(struct smb347_charger *smb)
    {
    let mut usb: bool = false;
    let mut dc: bool = false;
    unsigned int val;
    int ret;
    ret = regmap_read(smb.regmap, IRQSTAT_E, &val);
    if (ret < 0)
    return ret;
//
// Dc and usb are set depending on whether they are enabled in
// platform data _and_ whether corresponding undervoltage is set.
//
    if (smb.use_mains)
    dc = !(val & IRQSTAT_E_DCIN_UV_STAT);
    if (smb.use_usb)
    usb = !(val & IRQSTAT_E_USBIN_UV_STAT);
    ret = smb.mains_online != dc || smb.usb_online != usb;
    smb.mains_online = dc;
    smb.usb_online = usb;
    return ret;
    }
//
// smb347_is_ps_online - returns whether input power source is connected
// @smb: pointer to smb347 charger instance
//
// Returns %true if input power source is connected. Note that this is
// dependent on what platform has configured for usable power sources. For
// example if USB is disabled, this will return %false even if the USB cable
// is connected.
//
#[no_mangle]
unsafe extern "C" fn smb347_is_ps_online(smb: *mut smb347_charger) -> bool {
    static bool smb347_is_ps_online(struct smb347_charger *smb)
    {
    return smb.usb_online || smb.mains_online;
    }
//
// smb347_charging_status - returns status of charging
// @smb: pointer to smb347 charger instance
//
// Function returns charging status. %0 means no charging is in progress,
// %1 means pre-charging, %2 fast-charging and %3 taper-charging.
//
#[no_mangle]
unsafe extern "C" fn smb347_charging_status(smb: *mut smb347_charger) -> c_int {
    static int smb347_charging_status(struct smb347_charger *smb)
    {
    unsigned int val;
    int ret;
    if (!smb347_is_ps_online(smb))
    return 0;
    ret = regmap_read(smb.regmap, STAT_C, &val);
    if (ret < 0)
    return 0;
    return (val & STAT_C_CHG_MASK) >> STAT_C_CHG_SHIFT;
    }
#[no_mangle]
unsafe extern "C" fn smb347_charging_set(smb: *mut smb347_charger, enable: bool) -> c_int {
    static int smb347_charging_set(struct smb347_charger *smb, bool enable)
    {
    if (smb.enable_control != SMB3XX_CHG_ENABLE_SW) {
    dev_dbg(smb.dev, "charging enable/disable in SW disabled\n");
    return 0;
    }
    if (enable && smb.usb_vbus_enabled) {
    dev_dbg(smb.dev, "charging not enabled because USB is in host mode\n");
    return 0;
    }
    return regmap_update_bits(smb.regmap, CMD_A, CMD_A_CHG_ENABLED,
    enable ? CMD_A_CHG_ENABLED : 0);
    }
#[no_mangle]
pub unsafe extern "C" fn smb347_charging_enable(smb: *mut smb347_charger) -> c_int {
    static inline int smb347_charging_enable(struct smb347_charger *smb)
    {
    return smb347_charging_set(smb, true);
    }
#[no_mangle]
pub unsafe extern "C" fn smb347_charging_disable(smb: *mut smb347_charger) -> c_int {
    static inline int smb347_charging_disable(struct smb347_charger *smb)
    {
    return smb347_charging_set(smb, false);
    }
#[no_mangle]
unsafe extern "C" fn smb347_start_stop_charging(smb: *mut smb347_charger) -> c_int {
    static int smb347_start_stop_charging(struct smb347_charger *smb)
    {
    int ret;
//
// Depending on whether valid power source is connected or not, we
// disable or enable the charging. We do it manually because it
// depends on how the platform has configured the valid inputs.
//
    if (smb347_is_ps_online(smb)) {
    ret = smb347_charging_enable(smb);
    if (ret < 0)
    dev_err(smb.dev, "failed to enable charging\n");
    } else {
    ret = smb347_charging_disable(smb);
    if (ret < 0)
    dev_err(smb.dev, "failed to disable charging\n");
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn smb347_set_charge_current(smb: *mut smb347_charger) -> c_int {
    static int smb347_set_charge_current(struct smb347_charger *smb)
    {
    let mut id: c_uint = smb.id;
    int ret;
    if (smb.max_charge_current) {
    ret = current_to_hw(fcc_tbl[id], ARRAY_SIZE(fcc_tbl[id]),
    smb.max_charge_current);
    if (ret < 0)
    return ret;
    ret = regmap_update_bits(smb.regmap, CFG_CHARGE_CURRENT,
    CFG_CHARGE_CURRENT_FCC_MASK,
    ret << CFG_CHARGE_CURRENT_FCC_SHIFT);
    if (ret < 0)
    return ret;
    }
    if (smb.pre_charge_current) {
    ret = current_to_hw(pcc_tbl[id], ARRAY_SIZE(pcc_tbl[id]),
    smb.pre_charge_current);
    if (ret < 0)
    return ret;
    ret = regmap_update_bits(smb.regmap, CFG_CHARGE_CURRENT,
    CFG_CHARGE_CURRENT_PCC_MASK,
    ret << CFG_CHARGE_CURRENT_PCC_SHIFT);
    if (ret < 0)
    return ret;
    }
    if (smb.termination_current) {
    ret = current_to_hw(tc_tbl[id], ARRAY_SIZE(tc_tbl[id]),
    smb.termination_current);
    if (ret < 0)
    return ret;
    ret = regmap_update_bits(smb.regmap, CFG_CHARGE_CURRENT,
    CFG_CHARGE_CURRENT_TC_MASK, ret);
    if (ret < 0)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn smb347_set_current_limits(smb: *mut smb347_charger) -> c_int {
    static int smb347_set_current_limits(struct smb347_charger *smb)
    {
    let mut id: c_uint = smb.id;
    int ret;
    if (smb.mains_current_limit) {
    ret = current_to_hw(icl_tbl[id], ARRAY_SIZE(icl_tbl[id]),
    smb.mains_current_limit);
    if (ret < 0)
    return ret;
    ret = regmap_update_bits(smb.regmap, CFG_CURRENT_LIMIT,
    CFG_CURRENT_LIMIT_DC_MASK,
    ret << CFG_CURRENT_LIMIT_DC_SHIFT);
    if (ret < 0)
    return ret;
    }
    if (smb.usb_hc_current_limit) {
    ret = current_to_hw(icl_tbl[id], ARRAY_SIZE(icl_tbl[id]),
    smb.usb_hc_current_limit);
    if (ret < 0)
    return ret;
    ret = regmap_update_bits(smb.regmap, CFG_CURRENT_LIMIT,
    CFG_CURRENT_LIMIT_USB_MASK, ret);
    if (ret < 0)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn smb347_set_voltage_limits(smb: *mut smb347_charger) -> c_int {
    static int smb347_set_voltage_limits(struct smb347_charger *smb)
    {
    int ret;
    if (smb.pre_to_fast_voltage) {
    ret = smb.pre_to_fast_voltage;
// uV
    ret = clamp_val(ret, 2400000, 3000000) - 2400000;
    ret /= 200000;
    ret = regmap_update_bits(smb.regmap, CFG_FLOAT_VOLTAGE,
    CFG_FLOAT_VOLTAGE_THRESHOLD_MASK,
    ret << CFG_FLOAT_VOLTAGE_THRESHOLD_SHIFT);
    if (ret < 0)
    return ret;
    }
    if (smb.max_charge_voltage) {
    ret = smb.max_charge_voltage;
// uV
    ret = clamp_val(ret, 3500000, 4500000) - 3500000;
    ret /= 20000;
    ret = regmap_update_bits(smb.regmap, CFG_FLOAT_VOLTAGE,
    CFG_FLOAT_VOLTAGE_FLOAT_MASK, ret);
    if (ret < 0)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn smb347_set_temp_limits(smb: *mut smb347_charger) -> c_int {
    static int smb347_set_temp_limits(struct smb347_charger *smb)
    {
    let mut id: c_uint = smb.id;
    let mut enable_therm_monitor: bool = false;
    let mut ret: c_int = 0;
    int val;
    if (smb.chip_temp_threshold) {
    val = smb.chip_temp_threshold;
// degree C
    val = clamp_val(val, 100, 130) - 100;
    val /= 10;
    ret = regmap_update_bits(smb.regmap, CFG_OTG,
    CFG_OTG_TEMP_THRESHOLD_MASK,
    val << CFG_OTG_TEMP_THRESHOLD_SHIFT);
    if (ret < 0)
    return ret;
    }
    if (smb.soft_cold_temp_limit != SMB3XX_TEMP_USE_DEFAULT) {
    val = smb.soft_cold_temp_limit;
    val = clamp_val(val, 0, 15);
    val /= 5;
// this goes from higher to lower so invert the value
    val = ~val & 0x3;
    ret = regmap_update_bits(smb.regmap, CFG_TEMP_LIMIT,
    CFG_TEMP_LIMIT_SOFT_COLD_MASK,
    val << CFG_TEMP_LIMIT_SOFT_COLD_SHIFT);
    if (ret < 0)
    return ret;
    enable_therm_monitor = true;
    }
    if (smb.soft_hot_temp_limit != SMB3XX_TEMP_USE_DEFAULT) {
    val = smb.soft_hot_temp_limit;
    val = clamp_val(val, 40, 55) - 40;
    val /= 5;
    ret = regmap_update_bits(smb.regmap, CFG_TEMP_LIMIT,
    CFG_TEMP_LIMIT_SOFT_HOT_MASK,
    val << CFG_TEMP_LIMIT_SOFT_HOT_SHIFT);
    if (ret < 0)
    return ret;
    enable_therm_monitor = true;
    }
    if (smb.hard_cold_temp_limit != SMB3XX_TEMP_USE_DEFAULT) {
    val = smb.hard_cold_temp_limit;
    val = clamp_val(val, -5, 10) + 5;
    val /= 5;
// this goes from higher to lower so invert the value
    val = ~val & 0x3;
    ret = regmap_update_bits(smb.regmap, CFG_TEMP_LIMIT,
    CFG_TEMP_LIMIT_HARD_COLD_MASK,
    val << CFG_TEMP_LIMIT_HARD_COLD_SHIFT);
    if (ret < 0)
    return ret;
    enable_therm_monitor = true;
    }
    if (smb.hard_hot_temp_limit != SMB3XX_TEMP_USE_DEFAULT) {
    val = smb.hard_hot_temp_limit;
    val = clamp_val(val, 50, 65) - 50;
    val /= 5;
    ret = regmap_update_bits(smb.regmap, CFG_TEMP_LIMIT,
    CFG_TEMP_LIMIT_HARD_HOT_MASK,
    val << CFG_TEMP_LIMIT_HARD_HOT_SHIFT);
    if (ret < 0)
    return ret;
    enable_therm_monitor = true;
    }
//
// If any of the temperature limits are set, we also enable the
// thermistor monitoring.
//
// When soft limits are hit, the device will start to compensate
// current and/or voltage depending on the configuration.
//
// When hard limit is hit, the device will suspend charging
// depending on the configuration.
//
    if (enable_therm_monitor) {
    ret = regmap_update_bits(smb.regmap, CFG_THERM,
    CFG_THERM_MONITOR_DISABLED, 0);
    if (ret < 0)
    return ret;
    }
    if (smb.suspend_on_hard_temp_limit) {
    ret = regmap_update_bits(smb.regmap, CFG_SYSOK,
    CFG_SYSOK_SUSPEND_HARD_LIMIT_DISABLED, 0);
    if (ret < 0)
    return ret;
    }
    if (smb.soft_temp_limit_compensation !=
    SMB3XX_SOFT_TEMP_COMPENSATE_DEFAULT) {
    val = smb.soft_temp_limit_compensation & 0x3;
    ret = regmap_update_bits(smb.regmap, CFG_THERM,
    CFG_THERM_SOFT_HOT_COMPENSATION_MASK,
    val << CFG_THERM_SOFT_HOT_COMPENSATION_SHIFT);
    if (ret < 0)
    return ret;
    ret = regmap_update_bits(smb.regmap, CFG_THERM,
    CFG_THERM_SOFT_COLD_COMPENSATION_MASK,
    val << CFG_THERM_SOFT_COLD_COMPENSATION_SHIFT);
    if (ret < 0)
    return ret;
    }
    if (smb.charge_current_compensation) {
    val = current_to_hw(ccc_tbl[id], ARRAY_SIZE(ccc_tbl[id]),
    smb.charge_current_compensation);
    if (val < 0)
    return val;
    ret = regmap_update_bits(smb.regmap, CFG_OTG,
    CFG_OTG_CC_COMPENSATION_MASK,
    (val & 0x3) << CFG_OTG_CC_COMPENSATION_SHIFT);
    if (ret < 0)
    return ret;
    }
    return ret;
    }
//
// smb347_set_writable - enables/disables writing to non-volatile registers
// @smb: pointer to smb347 charger instance
//
// You can enable/disable writing to the non-volatile configuration
// registers by calling this function.
//
// Returns %0 on success and negative errno in case of failure.
//
    static int smb347_set_writable(struct smb347_charger *smb, bool writable,
    bool irq_toggle)
    {
    struct i2c_client *client = to_i2c_client(smb.dev);
    int ret;
    if (writable && irq_toggle && !smb.irq_unsupported)
    disable_irq(client.irq);
    ret = regmap_update_bits(smb.regmap, CMD_A, CMD_A_ALLOW_WRITE,
    writable ? CMD_A_ALLOW_WRITE : 0);
    if ((!writable || ret) && irq_toggle && !smb.irq_unsupported)
    enable_irq(client.irq);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn smb347_hw_init(smb: *mut smb347_charger) -> c_int {
    static int smb347_hw_init(struct smb347_charger *smb)
    {
    unsigned int val;
    int ret;
    ret = smb347_set_writable(smb, true, false);
    if (ret < 0)
    return ret;
//
// Program the platform specific configuration values to the device
// first.
//
    ret = smb347_set_charge_current(smb);
    if (ret < 0)
    goto fail;
    ret = smb347_set_current_limits(smb);
    if (ret < 0)
    goto fail;
    ret = smb347_set_voltage_limits(smb);
    if (ret < 0)
    goto fail;
    ret = smb347_set_temp_limits(smb);
    if (ret < 0)
    goto fail;
// If USB charging is disabled we put the USB in suspend mode
    if (!smb.use_usb) {
    ret = regmap_update_bits(smb.regmap, CMD_A,
    CMD_A_SUSPEND_ENABLED,
    CMD_A_SUSPEND_ENABLED);
    if (ret < 0)
    goto fail;
    }
//
// If configured by platform data, we enable hardware Auto-OTG
// support for driving VBUS. Otherwise we disable it.
//
    ret = regmap_update_bits(smb.regmap, CFG_OTHER, CFG_OTHER_RID_MASK,
    smb.use_usb_otg ? CFG_OTHER_RID_ENABLED_AUTO_OTG : 0);
    if (ret < 0)
    goto fail;
// Activate pin control, making it writable.
    switch (smb.enable_control) {
    case SMB3XX_CHG_ENABLE_PIN_ACTIVE_LOW:
    case SMB3XX_CHG_ENABLE_PIN_ACTIVE_HIGH:
    ret = regmap_set_bits(smb.regmap, CFG_PIN, CFG_PIN_EN_CTRL);
    if (ret < 0)
    goto fail;
    }
//
// Make the charging functionality controllable by a write to the
// command register unless pin control is specified in the platform
// data.
//
    switch (smb.enable_control) {
    case SMB3XX_CHG_ENABLE_PIN_ACTIVE_LOW:
    val = CFG_PIN_EN_CTRL_ACTIVE_LOW;
    break;
    case SMB3XX_CHG_ENABLE_PIN_ACTIVE_HIGH:
    val = CFG_PIN_EN_CTRL_ACTIVE_HIGH;
    break;
    default:
    val = 0;
    break;
    }
    ret = regmap_update_bits(smb.regmap, CFG_PIN, CFG_PIN_EN_CTRL_MASK,
    val);
    if (ret < 0)
    goto fail;
// Disable Automatic Power Source Detection (APSD) interrupt.
    ret = regmap_update_bits(smb.regmap, CFG_PIN, CFG_PIN_EN_APSD_IRQ, 0);
    if (ret < 0)
    goto fail;
    ret = smb347_update_ps_status(smb);
    if (ret < 0)
    goto fail;
    ret = smb347_start_stop_charging(smb);
    fail:
    smb347_set_writable(smb, false, false);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn smb347_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t smb347_interrupt(int irq, void *data)
    {
    struct smb347_charger *smb = data;
    unsigned int stat_c, irqstat_c, irqstat_d, irqstat_e;
    let mut handled: bool = false;
    int ret;
// SMB347 it needs at least 20ms for setting IRQSTAT_E_*IN_UV_IRQ
    usleep_range(25000, 35000);
    ret = regmap_read(smb.regmap, STAT_C, &stat_c);
    if (ret < 0) {
    dev_warn(smb.dev, "reading STAT_C failed\n");
    return IRQ_NONE;
    }
    ret = regmap_read(smb.regmap, IRQSTAT_C, &irqstat_c);
    if (ret < 0) {
    dev_warn(smb.dev, "reading IRQSTAT_C failed\n");
    return IRQ_NONE;
    }
    ret = regmap_read(smb.regmap, IRQSTAT_D, &irqstat_d);
    if (ret < 0) {
    dev_warn(smb.dev, "reading IRQSTAT_D failed\n");
    return IRQ_NONE;
    }
    ret = regmap_read(smb.regmap, IRQSTAT_E, &irqstat_e);
    if (ret < 0) {
    dev_warn(smb.dev, "reading IRQSTAT_E failed\n");
    return IRQ_NONE;
    }
//
// If we get charger error we report the error back to user.
// If the error is recovered charging will resume again.
//
    if (stat_c & STAT_C_CHARGER_ERROR) {
    dev_err(smb.dev, "charging stopped due to charger error\n");
    if (smb.use_mains)
    power_supply_changed(smb.mains);
    if (smb.use_usb)
    power_supply_changed(smb.usb);
    handled = true;
    }
//
// If we reached the termination current the battery is charged and
// we can update the status now. Charging is automatically
// disabled by the hardware.
//
    if (irqstat_c & (IRQSTAT_C_TERMINATION_IRQ | IRQSTAT_C_TAPER_IRQ)) {
    if (irqstat_c & IRQSTAT_C_TERMINATION_STAT) {
    if (smb.use_mains)
    power_supply_changed(smb.mains);
    if (smb.use_usb)
    power_supply_changed(smb.usb);
    }
    dev_dbg(smb.dev, "going to HW maintenance mode\n");
    handled = true;
    }
//
// If we got a charger timeout INT that means the charge
// full is not detected with in charge timeout value.
//
    if (irqstat_d & IRQSTAT_D_CHARGE_TIMEOUT_IRQ) {
    dev_dbg(smb.dev, "total Charge Timeout INT received\n");
    if (irqstat_d & IRQSTAT_D_CHARGE_TIMEOUT_STAT)
    dev_warn(smb.dev, "charging stopped due to timeout\n");
    if (smb.use_mains)
    power_supply_changed(smb.mains);
    if (smb.use_usb)
    power_supply_changed(smb.usb);
    handled = true;
    }
//
// If we got an under voltage interrupt it means that AC/USB input
// was connected or disconnected.
//
    if (irqstat_e & (IRQSTAT_E_USBIN_UV_IRQ | IRQSTAT_E_DCIN_UV_IRQ)) {
    if (smb347_update_ps_status(smb) > 0) {
    smb347_start_stop_charging(smb);
    if (smb.use_mains)
    power_supply_changed(smb.mains);
    if (smb.use_usb)
    power_supply_changed(smb.usb);
    }
    handled = true;
    }
    return handled ? IRQ_HANDLED : IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn smb347_irq_set(smb: *mut smb347_charger, enable: bool) -> c_int {
    static int smb347_irq_set(struct smb347_charger *smb, bool enable)
    {
    int ret;
    if (smb.irq_unsupported)
    return 0;
    ret = smb347_set_writable(smb, true, true);
    if (ret < 0)
    return ret;
//
// Enable/disable interrupts for:
// - under voltage
// - termination current reached
// - charger timeout
// - charger error
//
    ret = regmap_update_bits(smb.regmap, CFG_FAULT_IRQ, 0xff,
    enable ? CFG_FAULT_IRQ_DCIN_UV : 0);
    if (ret < 0)
    goto fail;
    ret = regmap_update_bits(smb.regmap, CFG_STATUS_IRQ, 0xff,
    enable ? (CFG_STATUS_IRQ_TERMINATION_OR_TAPER |
    CFG_STATUS_IRQ_CHARGE_TIMEOUT) : 0);
    if (ret < 0)
    goto fail;
    ret = regmap_update_bits(smb.regmap, CFG_PIN, CFG_PIN_EN_CHARGER_ERROR,
    enable ? CFG_PIN_EN_CHARGER_ERROR : 0);
    fail:
    smb347_set_writable(smb, false, true);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn smb347_irq_enable(smb: *mut smb347_charger) -> c_int {
    static inline int smb347_irq_enable(struct smb347_charger *smb)
    {
    return smb347_irq_set(smb, true);
    }
#[no_mangle]
pub unsafe extern "C" fn smb347_irq_disable(smb: *mut smb347_charger) -> c_int {
    static inline int smb347_irq_disable(struct smb347_charger *smb)
    {
    return smb347_irq_set(smb, false);
    }
    static int smb347_irq_init(struct smb347_charger *smb,
    struct i2c_client *client)
    {
    int ret;
    smb.irq_unsupported = true;
//
// Interrupt pin is optional. If it is connected, we setup the
// interrupt support here.
//
    if (!client.irq)
    return 0;
    ret = smb347_set_writable(smb, true, false);
    if (ret < 0)
    return ret;
//
// Configure the STAT output to be suitable for interrupts: disable
// all other output (except interrupts) and make it active low.
//
    ret = regmap_update_bits(smb.regmap, CFG_STAT,
    CFG_STAT_ACTIVE_HIGH | CFG_STAT_DISABLED,
    CFG_STAT_DISABLED);
    smb347_set_writable(smb, false, false);
    if (ret < 0) {
    dev_warn(smb.dev, "failed to initialize IRQ: %d\n", ret);
    dev_warn(smb.dev, "disabling IRQ support\n");
    return 0;
    }
    ret = devm_request_threaded_irq(smb.dev, client.irq, core::ptr::null_mut(),
    smb347_interrupt, IRQF_ONESHOT,
    client.name, smb);
    if (ret)
    return ret;
    smb.irq_unsupported = false;
    ret = smb347_irq_enable(smb);
    if (ret < 0)
    return ret;
    return 0;
    }
//
// Returns the constant charge current programmed
// into the charger in uA.
//
#[no_mangle]
unsafe extern "C" fn get_const_charge_current(smb: *mut smb347_charger) -> c_int {
    static int get_const_charge_current(struct smb347_charger *smb)
    {
    let mut id: c_uint = smb.id;
    int ret, intval;
    unsigned int v;
    if (!smb347_is_ps_online(smb))
    return -ENODATA;
    ret = regmap_read(smb.regmap, STAT_B, &v);
    if (ret < 0)
    return ret;
//
// The current value is composition of FCC and PCC values
// and we can detect which table to use from bit 5.
//
    if (v & 0x20) {
    intval = hw_to_current(fcc_tbl[id],
    ARRAY_SIZE(fcc_tbl[id]), v & 7);
    } else {
    v >>= 3;
    intval = hw_to_current(pcc_tbl[id],
    ARRAY_SIZE(pcc_tbl[id]), v & 7);
    }
    return intval;
    }
//
// Returns the constant charge voltage programmed
// into the charger in uV.
//
#[no_mangle]
unsafe extern "C" fn get_const_charge_voltage(smb: *mut smb347_charger) -> c_int {
    static int get_const_charge_voltage(struct smb347_charger *smb)
    {
    int ret, intval;
    unsigned int v;
    if (!smb347_is_ps_online(smb))
    return -ENODATA;
    ret = regmap_read(smb.regmap, STAT_A, &v);
    if (ret < 0)
    return ret;
    v &= STAT_A_FLOAT_VOLTAGE_MASK;
    if (v > 0x3d)
    v = 0x3d;
    intval = 3500000 + v * 20000;
    return intval;
    }
    static int smb347_get_charging_status(struct smb347_charger *smb,
    struct power_supply *psy)
    {
    int ret, status;
    unsigned int val;
    if (psy.desc.type == POWER_SUPPLY_TYPE_USB) {
    if (!smb.usb_online)
    return POWER_SUPPLY_STATUS_DISCHARGING;
    } else {
    if (!smb.mains_online)
    return POWER_SUPPLY_STATUS_DISCHARGING;
    }
    ret = regmap_read(smb.regmap, STAT_C, &val);
    if (ret < 0)
    return ret;
    if ((val & STAT_C_CHARGER_ERROR) ||
    (val & STAT_C_HOLDOFF_STAT)) {
//
// set to NOT CHARGING upon charger error
// or charging has stopped.
//
    status = POWER_SUPPLY_STATUS_NOT_CHARGING;
    } else {
    if ((val & STAT_C_CHG_MASK) >> STAT_C_CHG_SHIFT) {
//
// set to charging if battery is in pre-charge,
// fast charge or taper charging mode.
//
    status = POWER_SUPPLY_STATUS_CHARGING;
    } else if (val & STAT_C_CHG_TERM) {
//
// set the status to FULL if battery is not in pre
// charge, fast charge or taper charging mode AND
// charging is terminated at least once.
//
    status = POWER_SUPPLY_STATUS_FULL;
    } else {
//
// in this case no charger error or termination
// occurred but charging is not in progress!!!
//
    status = POWER_SUPPLY_STATUS_NOT_CHARGING;
    }
    }
    return status;
    }
    static int smb347_get_property_locked(struct power_supply *psy,
    enum power_supply_property prop,
    union power_supply_propval *val)
    {
    struct smb347_charger *smb = power_supply_get_drvdata(psy);
    int ret;
    switch (prop) {
    case POWER_SUPPLY_PROP_STATUS:
    ret = smb347_get_charging_status(smb, psy);
    if (ret < 0)
    return ret;
    val.intval = ret;
    break;
    case POWER_SUPPLY_PROP_CHARGE_TYPE:
    if (psy.desc.type == POWER_SUPPLY_TYPE_USB) {
    if (!smb.usb_online)
    return -ENODATA;
    } else {
    if (!smb.mains_online)
    return -ENODATA;
    }
//
// We handle trickle and pre-charging the same, and taper
// and none the same.
//
    switch (smb347_charging_status(smb)) {
    case 1:
    val.intval = POWER_SUPPLY_CHARGE_TYPE_TRICKLE;
    break;
    case 2:
    val.intval = POWER_SUPPLY_CHARGE_TYPE_FAST;
    break;
    default:
    val.intval = POWER_SUPPLY_CHARGE_TYPE_NONE;
    break;
    }
    break;
    case POWER_SUPPLY_PROP_ONLINE:
    if (psy.desc.type == POWER_SUPPLY_TYPE_USB)
    val.intval = smb.usb_online;
    else
    val.intval = smb.mains_online;
    break;
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE:
    ret = get_const_charge_voltage(smb);
    if (ret < 0)
    return ret;
    val.intval = ret;
    break;
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT:
    ret = get_const_charge_current(smb);
    if (ret < 0)
    return ret;
    val.intval = ret;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int smb347_get_property(struct power_supply *psy,
    enum power_supply_property prop,
    union power_supply_propval *val)
    {
    struct smb347_charger *smb = power_supply_get_drvdata(psy);
    struct i2c_client *client = to_i2c_client(smb.dev);
    int ret;
    if (!smb.irq_unsupported)
    disable_irq(client.irq);
    ret = smb347_get_property_locked(psy, prop, val);
    if (!smb.irq_unsupported)
    enable_irq(client.irq);
    return ret;
    }
    static enum power_supply_property smb347_properties[] = {
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_CHARGE_TYPE,
    POWER_SUPPLY_PROP_ONLINE,
    POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT,
    POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE,
    };
#[no_mangle]
unsafe extern "C" fn smb347_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool smb347_volatile_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case IRQSTAT_A:
    case IRQSTAT_C:
    case IRQSTAT_D:
    case IRQSTAT_E:
    case IRQSTAT_F:
    case STAT_A:
    case STAT_B:
    case STAT_C:
    case STAT_E:
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn smb347_readable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool smb347_readable_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case CFG_CHARGE_CURRENT:
    case CFG_CURRENT_LIMIT:
    case CFG_FLOAT_VOLTAGE:
    case CFG_STAT:
    case CFG_PIN:
    case CFG_THERM:
    case CFG_SYSOK:
    case CFG_OTHER:
    case CFG_OTG:
    case CFG_TEMP_LIMIT:
    case CFG_FAULT_IRQ:
    case CFG_STATUS_IRQ:
    case CFG_ADDRESS:
    case CMD_A:
    case CMD_B:
    case CMD_C:
    return true;
    }
    return smb347_volatile_reg(dev, reg);
    }
#[no_mangle]
unsafe extern "C" fn smb347_dt_parse_dev_info(smb: *mut smb347_charger) {
    static void smb347_dt_parse_dev_info(struct smb347_charger *smb)
    {
    struct device *dev = smb.dev;
    smb.soft_temp_limit_compensation =
    SMB3XX_SOFT_TEMP_COMPENSATE_DEFAULT;
//
// These properties come from the battery info, still we need to
// pre-initialize the values. See smb347_get_battery_info() below.
//
    smb.soft_cold_temp_limit = SMB3XX_TEMP_USE_DEFAULT;
    smb.hard_cold_temp_limit = SMB3XX_TEMP_USE_DEFAULT;
    smb.soft_hot_temp_limit  = SMB3XX_TEMP_USE_DEFAULT;
    smb.hard_hot_temp_limit  = SMB3XX_TEMP_USE_DEFAULT;
// Charging constraints
    device_property_read_u32(dev, "summit,fast-voltage-threshold-microvolt",
    &smb.pre_to_fast_voltage);
    device_property_read_u32(dev, "summit,mains-current-limit-microamp",
    &smb.mains_current_limit);
    device_property_read_u32(dev, "summit,usb-current-limit-microamp",
    &smb.usb_hc_current_limit);
// For thermometer monitoring
    device_property_read_u32(dev, "summit,chip-temperature-threshold-celsius",
    &smb.chip_temp_threshold);
    device_property_read_u32(dev, "summit,soft-compensation-method",
    &smb.soft_temp_limit_compensation);
    device_property_read_u32(dev, "summit,charge-current-compensation-microamp",
    &smb.charge_current_compensation);
// Supported charging mode
    smb.use_mains = device_property_read_bool(dev, "summit,enable-mains-charging");
    smb.use_usb = device_property_read_bool(dev, "summit,enable-usb-charging");
    smb.use_usb_otg = device_property_read_bool(dev, "summit,enable-otg-charging");
// Select charging control
    device_property_read_u32(dev, "summit,enable-charge-control",
    &smb.enable_control);
//
// Polarity of INOK signal indicating presence of external power
// supply connected to the charger.
//
    device_property_read_u32(dev, "summit,inok-polarity",
    &smb.inok_polarity);
    }
#[no_mangle]
unsafe extern "C" fn smb347_get_battery_info(smb: *mut smb347_charger) -> c_int {
    static int smb347_get_battery_info(struct smb347_charger *smb)
    {
    struct power_supply_battery_info *info;
    struct power_supply *supply;
    int err;
    if (smb.mains)
    supply = smb.mains;
    else
    supply = smb.usb;
    err = power_supply_get_battery_info(supply, &info);
    if (err == -ENXIO || err == -ENODEV)
    return 0;
    if (err)
    return err;
    if (info.constant_charge_current_max_ua != -EINVAL)
    smb.max_charge_current = info.constant_charge_current_max_ua;
    if (info.constant_charge_voltage_max_uv != -EINVAL)
    smb.max_charge_voltage = info.constant_charge_voltage_max_uv;
    if (info.precharge_current_ua != -EINVAL)
    smb.pre_charge_current = info.precharge_current_ua;
    if (info.charge_term_current_ua != -EINVAL)
    smb.termination_current = info.charge_term_current_ua;
    if (info.temp_alert_min != INT_MIN)
    smb.soft_cold_temp_limit = info.temp_alert_min;
    if (info.temp_alert_max != INT_MAX)
    smb.soft_hot_temp_limit = info.temp_alert_max;
    if (info.temp_min != INT_MIN)
    smb.hard_cold_temp_limit = info.temp_min;
    if (info.temp_max != INT_MAX)
    smb.hard_hot_temp_limit = info.temp_max;
// Suspend when battery temperature is outside hard limits
    if (smb.hard_cold_temp_limit != SMB3XX_TEMP_USE_DEFAULT ||
    smb.hard_hot_temp_limit != SMB3XX_TEMP_USE_DEFAULT)
    smb.suspend_on_hard_temp_limit = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn smb347_usb_vbus_get_current_limit(rdev: *mut regulator_dev) -> c_int {
    static int smb347_usb_vbus_get_current_limit(struct regulator_dev *rdev)
    {
    struct smb347_charger *smb = rdev_get_drvdata(rdev);
    unsigned int val;
    int ret;
    ret = regmap_read(smb.regmap, CFG_OTG, &val);
    if (ret < 0)
    return ret;
//
// It's unknown what happens if this bit is unset due to lack of
// access to the datasheet, assume it's limit-enable.
//
    if (!(val & CFG_OTG_CURRENT_LIMIT_250mA))
    return 0;
    return val & CFG_OTG_CURRENT_LIMIT_750mA ? 750000 : 250000;
    }
    static int smb347_usb_vbus_set_new_current_limit(struct smb347_charger *smb,
    int max_uA)
    {
    const unsigned int mask = CFG_OTG_CURRENT_LIMIT_750mA |
    CFG_OTG_CURRENT_LIMIT_250mA;
    let mut val: c_uint = CFG_OTG_CURRENT_LIMIT_250mA;
    int ret;
    if (max_uA >= 750000)
    val |= CFG_OTG_CURRENT_LIMIT_750mA;
    ret = regmap_update_bits(smb.regmap, CFG_OTG, mask, val);
    if (ret < 0)
    dev_err(smb.dev, "failed to change USB current limit\n");
    return ret;
    }
    static int smb347_usb_vbus_set_current_limit(struct regulator_dev *rdev,
    int min_uA, int max_uA)
    {
    struct smb347_charger *smb = rdev_get_drvdata(rdev);
    int ret;
    ret = smb347_set_writable(smb, true, true);
    if (ret < 0)
    return ret;
    ret = smb347_usb_vbus_set_new_current_limit(smb, max_uA);
    smb347_set_writable(smb, false, true);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn smb347_usb_vbus_regulator_enable(rdev: *mut regulator_dev) -> c_int {
    static int smb347_usb_vbus_regulator_enable(struct regulator_dev *rdev)
    {
    struct smb347_charger *smb = rdev_get_drvdata(rdev);
    int ret, max_uA;
    ret = smb347_set_writable(smb, true, true);
    if (ret < 0)
    return ret;
    smb347_charging_disable(smb);
    if (device_property_read_bool(&rdev.dev, "summit,needs-inok-toggle")) {
    let mut sysok: c_uint = 0;
    if (smb.inok_polarity == SMB3XX_SYSOK_INOK_ACTIVE_LOW)
    sysok = CFG_SYSOK_INOK_ACTIVE_HIGH;
//
// VBUS won't be powered if INOK is active, so we need to
// manually disable INOK on some platforms.
//
    ret = regmap_update_bits(smb.regmap, CFG_SYSOK,
    CFG_SYSOK_INOK_ACTIVE_HIGH, sysok);
    if (ret < 0) {
    dev_err(smb.dev, "failed to disable INOK\n");
    goto done;
    }
    }
    ret = smb347_usb_vbus_get_current_limit(rdev);
    if (ret < 0) {
    dev_err(smb.dev, "failed to get USB VBUS current limit\n");
    goto done;
    }
    max_uA = ret;
    ret = smb347_usb_vbus_set_new_current_limit(smb, 250000);
    if (ret < 0) {
    dev_err(smb.dev, "failed to preset USB VBUS current limit\n");
    goto done;
    }
    ret = regmap_set_bits(smb.regmap, CMD_A, CMD_A_OTG_ENABLED);
    if (ret < 0) {
    dev_err(smb.dev, "failed to enable USB VBUS\n");
    goto done;
    }
    smb.usb_vbus_enabled = true;
    ret = smb347_usb_vbus_set_new_current_limit(smb, max_uA);
    if (ret < 0) {
    dev_err(smb.dev, "failed to restore USB VBUS current limit\n");
    goto done;
    }
    done:
    smb347_set_writable(smb, false, true);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn smb347_usb_vbus_regulator_disable(rdev: *mut regulator_dev) -> c_int {
    static int smb347_usb_vbus_regulator_disable(struct regulator_dev *rdev)
    {
    struct smb347_charger *smb = rdev_get_drvdata(rdev);
    int ret;
    ret = smb347_set_writable(smb, true, true);
    if (ret < 0)
    return ret;
    ret = regmap_clear_bits(smb.regmap, CMD_A, CMD_A_OTG_ENABLED);
    if (ret < 0) {
    dev_err(smb.dev, "failed to disable USB VBUS\n");
    goto done;
    }
    smb.usb_vbus_enabled = false;
    if (device_property_read_bool(&rdev.dev, "summit,needs-inok-toggle")) {
    let mut sysok: c_uint = 0;
    if (smb.inok_polarity == SMB3XX_SYSOK_INOK_ACTIVE_HIGH)
    sysok = CFG_SYSOK_INOK_ACTIVE_HIGH;
    ret = regmap_update_bits(smb.regmap, CFG_SYSOK,
    CFG_SYSOK_INOK_ACTIVE_HIGH, sysok);
    if (ret < 0) {
    dev_err(smb.dev, "failed to enable INOK\n");
    goto done;
    }
    }
    smb347_start_stop_charging(smb);
    done:
    smb347_set_writable(smb, false, true);
    return ret;
    }
    static const struct regmap_config smb347_regmap = {
    .reg_bits	= 8,
    .val_bits	= 8,
    .max_register	= SMB347_MAX_REGISTER,
    .volatile_reg	= smb347_volatile_reg,
    .readable_reg	= smb347_readable_reg,
    .cache_type	= REGCACHE_MAPLE,
    };
    static const struct regulator_ops smb347_usb_vbus_regulator_ops = {
    .is_enabled	= regulator_is_enabled_regmap,
    .enable		= smb347_usb_vbus_regulator_enable,
    .disable	= smb347_usb_vbus_regulator_disable,
    .get_current_limit = smb347_usb_vbus_get_current_limit,
    .set_current_limit = smb347_usb_vbus_set_current_limit,
    };
    static const struct power_supply_desc smb347_mains_desc = {
    .name		= "smb347-mains",
    .type		= POWER_SUPPLY_TYPE_MAINS,
    .get_property	= smb347_get_property,
    .properties	= smb347_properties,
    .num_properties	= ARRAY_SIZE(smb347_properties),
    };
    static const struct power_supply_desc smb347_usb_desc = {
    .name		= "smb347-usb",
    .type		= POWER_SUPPLY_TYPE_USB,
    .get_property	= smb347_get_property,
    .properties	= smb347_properties,
    .num_properties	= ARRAY_SIZE(smb347_properties),
    };
    static const struct regulator_desc smb347_usb_vbus_regulator_desc = {
    .name		= "smb347-usb-vbus",
    .of_match	= of_match_ptr("usb-vbus"),
    .ops		= &smb347_usb_vbus_regulator_ops,
    .type		= REGULATOR_VOLTAGE,
    .owner		= THIS_MODULE,
    .enable_reg	= CMD_A,
    .enable_mask	= CMD_A_OTG_ENABLED,
    .enable_val	= CMD_A_OTG_ENABLED,
    .fixed_uV	= 5000000,
    .n_voltages	= 1,
    };
#[no_mangle]
unsafe extern "C" fn smb347_probe(client: *mut i2c_client) -> c_int {
    static int smb347_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    let mut mains_usb_cfg: power_supply_config = {};
    let mut usb_rdev_cfg: regulator_config = {};
    struct device *dev = &client.dev;
    struct smb347_charger *smb;
    int ret;
    smb = devm_kzalloc(dev, sizeof(*smb), GFP_KERNEL);
    if (!smb)
    return -ENOMEM;
    smb.dev = &client.dev;
    smb.id = id.driver_data;
    i2c_set_clientdata(client, smb);
    smb347_dt_parse_dev_info(smb);
    if (!smb.use_mains && !smb.use_usb)
    return -EINVAL;
    smb.regmap = devm_regmap_init_i2c(client, &smb347_regmap);
    if (IS_ERR(smb.regmap))
    return PTR_ERR(smb.regmap);
    mains_usb_cfg.drv_data = smb;
    mains_usb_cfg.fwnode = dev_fwnode(dev);
    if (smb.use_mains) {
    smb.mains = devm_power_supply_register(dev, &smb347_mains_desc,
    &mains_usb_cfg);
    if (IS_ERR(smb.mains))
    return PTR_ERR(smb.mains);
    }
    if (smb.use_usb) {
    smb.usb = devm_power_supply_register(dev, &smb347_usb_desc,
    &mains_usb_cfg);
    if (IS_ERR(smb.usb))
    return PTR_ERR(smb.usb);
    }
    ret = smb347_get_battery_info(smb);
    if (ret)
    return ret;
    ret = smb347_hw_init(smb);
    if (ret < 0)
    return ret;
    ret = smb347_irq_init(smb, client);
    if (ret)
    return ret;
    usb_rdev_cfg.dev = dev;
    usb_rdev_cfg.driver_data = smb;
    usb_rdev_cfg.regmap = smb.regmap;
    smb.usb_rdev = devm_regulator_register(dev,
    &smb347_usb_vbus_regulator_desc,
    &usb_rdev_cfg);
    if (IS_ERR(smb.usb_rdev)) {
    smb347_irq_disable(smb);
    return PTR_ERR(smb.usb_rdev);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn smb347_remove(client: *mut i2c_client) {
    static void smb347_remove(struct i2c_client *client)
    {
    struct smb347_charger *smb = i2c_get_clientdata(client);
    smb347_usb_vbus_regulator_disable(smb.usb_rdev);
    smb347_irq_disable(smb);
    }
#[no_mangle]
unsafe extern "C" fn smb347_shutdown(client: *mut i2c_client) {
    static void smb347_shutdown(struct i2c_client *client)
    {
    smb347_remove(client);
    }
    static const struct i2c_device_id smb347_id[] = {
    { .name = "smb345", .driver_data = SMB345 },
    { .name = "smb347", .driver_data = SMB347 },
    { .name = "smb358", .driver_data = SMB358 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, smb347_id);
    static const struct of_device_id smb3xx_of_match[] = {
    { .compatible = "summit,smb345" },
    { .compatible = "summit,smb347" },
    { .compatible = "summit,smb358" },
    { },
    };
    MODULE_DEVICE_TABLE(of, smb3xx_of_match);
    static struct i2c_driver smb347_driver = {
    .driver = {
    .name = "smb347",
    .of_match_table = smb3xx_of_match,
    },
    .probe = smb347_probe,
    .remove = smb347_remove,
    .shutdown = smb347_shutdown,
    .id_table = smb347_id,
    };
    module_i2c_driver(smb347_driver);
    MODULE_AUTHOR("Bruce E. Robertson <bruce.e.robertson@intel.com>");
    MODULE_AUTHOR("Mika Westerberg <mika.westerberg@linux.intel.com>");
    MODULE_DESCRIPTION("SMB347 battery charger driver");
    MODULE_LICENSE("GPL");
