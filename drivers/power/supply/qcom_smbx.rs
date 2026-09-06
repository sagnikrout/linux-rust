//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/qcom_smbx.c
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
// Copyright (c) 2016-2019 The Linux Foundation. All rights reserved.
// Copyright (c) 2023, Linaro Ltd.
// Author: Casey Connolly <casey.connolly@linaro.org>
//
// This driver is for the switch-mode battery charger and boost
// hardware found in pmi8998 and related PMICs.
//

// clang-format off
pub const BATTERY_CHARGER_STATUS_1: c_uint = 0x06;

pub const STEP_CHARGING_STATUS_SHIFT: c_int = 3;

pub const BATTERY_CHARGER_STATUS_2: c_uint = 0x07;

pub const BATTERY_CHARGER_STATUS_4: c_uint = 0x0A;

pub const BATTERY_CHARGER_STATUS_7: c_uint = 0x0D;

pub const CHARGING_ENABLE_CMD: c_uint = 0x42;

pub const CHGR_CFG2: c_uint = 0x51;

pub const PRE_CHARGE_CURRENT_CFG: c_uint = 0x60;

pub const FAST_CHARGE_CURRENT_CFG: c_uint = 0x61;

pub const FLOAT_VOLTAGE_CFG: c_uint = 0x70;

pub const FG_UPDATE_CFG_2_SEL: c_uint = 0x7D;

pub const JEITA_EN_CFG: c_uint = 0x90;

pub const INT_RT_STS: c_uint = 0x310;

pub const OTG_CFG: c_uint = 0x153;

pub const OTG_ENG_OTG_CFG: c_uint = 0x1C0;

pub const APSD_STATUS: c_uint = 0x307;

pub const APSD_RESULT_STATUS: c_uint = 0x308;

pub const USBIN_CMD_IL: c_uint = 0x340;

pub const TYPE_C_STATUS_1: c_uint = 0x30B;

pub const TYPE_C_STATUS_2: c_uint = 0x30C;

pub const TYPE_C_STATUS_3: c_uint = 0x30D;

pub const TYPE_C_STATUS_4: c_uint = 0x30E;

pub const TYPE_C_STATUS_5: c_uint = 0x30F;

pub const CMD_APSD: c_uint = 0x341;

pub const TYPE_C_CFG: c_uint = 0x358;

pub const TYPE_C_CFG_2: c_uint = 0x359;

pub const TYPE_C_CFG_3: c_uint = 0x35A;

pub const USBIN_OPTIONS_1_CFG: c_uint = 0x362;

pub const USBIN_OPTIONS_2_CFG: c_uint = 0x363;

pub const TAPER_TIMER_SEL_CFG: c_uint = 0x364;

pub const USBIN_LOAD_CFG: c_uint = 0x365;

pub const USBIN_ICL_OPTIONS: c_uint = 0x366;

pub const TYPE_C_INTRPT_ENB_SOFTWARE_CTRL: c_uint = 0x368;

pub const USBIN_CURRENT_LIMIT_CFG: c_uint = 0x370;

pub const USBIN_AICL_OPTIONS_CFG: c_uint = 0x380;

pub const USBIN_5V_AICL_THRESHOLD_CFG: c_uint = 0x381;

pub const USBIN_CONT_AICL_THRESHOLD_CFG: c_uint = 0x384;

pub const DC_ENG_SSUPPLY_CFG2: c_uint = 0x4C1;

pub const OTG_SS_SLOW: c_uint = 0x3;
pub const DCIN_AICL_REF_SEL_CFG: c_uint = 0x481;

pub const WI_PWR_OPTIONS: c_uint = 0x495;

pub const ICL_STATUS: c_uint = 0x607;

pub const POWER_PATH_STATUS: c_uint = 0x60B;

pub const BARK_BITE_WDOG_PET: c_uint = 0x643;

pub const WD_CFG: c_uint = 0x651;

pub const SNARL_BARK_BITE_WD_CFG: c_uint = 0x653;

pub const AICL_RERUN_TIME_CFG: c_uint = 0x661;

pub const STAT_CFG: c_uint = 0x690;

pub const SDP_CURRENT_UA: c_int = 500000;
pub const CDP_CURRENT_UA: c_int = 1500000;
pub const DCP_CURRENT_UA: c_int = 1500000;

// pmi8998 registers represent current in increments of 1/40th of an amp
pub const CURRENT_SCALE_FACTOR: c_int = 25000;
// clang-format on
    enum charger_status {
    TRICKLE_CHARGE = 0,
    PRE_CHARGE,
    FAST_CHARGE,
    FULLON_CHARGE,
    TAPER_CHARGE,
    TERMINATE_CHARGE,
    INHIBIT_CHARGE,
    DISABLE_CHARGE,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb_init_register {
    pub addr: u16,
    pub mask: u8,
    pub val: u8,
}

//
// struct smb_chip - smb chip structure
// @dev:		Device reference for power_supply
// @name:		The platform device name
// @base:		Base address for smb registers
// @regmap:		Register map
// @batt_info:		Battery data from DT
// @status_change_work: Worker to handle plug/unplug events
// @cable_irq:		USB plugin IRQ
// @wakeup_enabled:	If the cable IRQ will cause a wakeup
// @usb_in_i_chan:	USB_IN current measurement channel
// @usb_in_v_chan:	USB_IN voltage measurement channel
// @chg_psy:		Charger power supply instance
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb_chip {
    pub dev: *mut device,
    pub name: *const c_char,
    pub base: c_uint,
    pub regmap: *mut regmap,
    pub batt_info: *mut power_supply_battery_info,
    pub status_change_work: delayed_work,
    pub cable_irq: c_int,
    pub wakeup_enabled: bool,
    pub usb_in_i_chan: *mut iio_channel,
    pub usb_in_v_chan: *mut iio_channel,
    pub chg_psy: *mut power_supply,
}

    static enum power_supply_property smb_properties[] = {
    POWER_SUPPLY_PROP_MANUFACTURER,
    POWER_SUPPLY_PROP_MODEL_NAME,
    POWER_SUPPLY_PROP_CURRENT_MAX,
    POWER_SUPPLY_PROP_CURRENT_NOW,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_HEALTH,
    POWER_SUPPLY_PROP_ONLINE,
    POWER_SUPPLY_PROP_USB_TYPE,
    };
#[no_mangle]
unsafe extern "C" fn smb_get_prop_usb_online(chip: *mut smb_chip, val: *mut c_int) -> c_int {
    static int smb_get_prop_usb_online(struct smb_chip *chip, int *val)
    {
    unsigned int stat;
    int rc;
    rc = regmap_read(chip.regmap, chip.base + POWER_PATH_STATUS, &stat);
    if (rc < 0) {
    dev_err(chip.dev, "Couldn't read power path status: %d\n", rc);
    return rc;
    }
// val = (stat & P_PATH_USE_USBIN_BIT) &&
    (stat & P_PATH_VALID_INPUT_POWER_SOURCE_STS_BIT);
    return 0;
    }
//
// Qualcomm "automatic power source detection" aka APSD
// tells us what type of charger we're connected to.
//
#[no_mangle]
unsafe extern "C" fn smb_apsd_get_charger_type(chip: *mut smb_chip, val: *mut c_int) -> c_int {
    static int smb_apsd_get_charger_type(struct smb_chip *chip, int *val)
    {
    unsigned int apsd_stat, stat;
    let mut usb_online: c_int = 0;
    int rc;
    rc = smb_get_prop_usb_online(chip, &usb_online);
    if (!usb_online) {
// val = POWER_SUPPLY_USB_TYPE_UNKNOWN;
    return rc;
    }
    rc = regmap_read(chip.regmap, chip.base + APSD_STATUS, &apsd_stat);
    if (rc < 0) {
    dev_err(chip.dev, "Failed to read apsd status, rc = %d", rc);
    return rc;
    }
    if (!(apsd_stat & APSD_DTC_STATUS_DONE_BIT)) {
    dev_dbg(chip.dev, "Apsd not ready");
    return -EAGAIN;
    }
    rc = regmap_read(chip.regmap, chip.base + APSD_RESULT_STATUS, &stat);
    if (rc < 0) {
    dev_err(chip.dev, "Failed to read apsd result, rc = %d", rc);
    return rc;
    }
    stat &= APSD_RESULT_STATUS_MASK;
    if (stat & CDP_CHARGER_BIT)
// val = POWER_SUPPLY_USB_TYPE_CDP;
#[no_mangle]
pub unsafe extern "C" fn if(FLOAT_CHARGER_BIT): stat & (DCP_CHARGER_BIT | OCP_CHARGER_BIT |) -> else {
    else if (stat & (DCP_CHARGER_BIT | OCP_CHARGER_BIT | FLOAT_CHARGER_BIT))
// val = POWER_SUPPLY_USB_TYPE_DCP;
    else /* SDP_CHARGER_BIT (or others) */
// val = POWER_SUPPLY_USB_TYPE_SDP;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn smb_get_prop_status(chip: *mut smb_chip, val: *mut c_int) -> c_int {
    static int smb_get_prop_status(struct smb_chip *chip, int *val)
    {
    unsigned char stat[2];
    let mut usb_online: c_int = 0;
    int rc;
    rc = smb_get_prop_usb_online(chip, &usb_online);
    if (!usb_online) {
// val = POWER_SUPPLY_STATUS_DISCHARGING;
    return rc;
    }
    rc = regmap_bulk_read(chip.regmap,
    chip.base + BATTERY_CHARGER_STATUS_1, &stat, 2);
    if (rc < 0) {
    dev_err(chip.dev, "Failed to read charging status ret=%d\n",
    rc);
    return rc;
    }
    if (stat[1] & CHARGER_ERROR_STATUS_BAT_OV_BIT) {
// val = POWER_SUPPLY_STATUS_NOT_CHARGING;
    return 0;
    }
    stat[0] = stat[0] & BATTERY_CHARGER_STATUS_MASK;
    switch (stat[0]) {
    case TRICKLE_CHARGE:
    case PRE_CHARGE:
    case FAST_CHARGE:
    case FULLON_CHARGE:
    case TAPER_CHARGE:
// val = POWER_SUPPLY_STATUS_CHARGING;
    return rc;
    case DISABLE_CHARGE:
// val = POWER_SUPPLY_STATUS_NOT_CHARGING;
    return rc;
    case TERMINATE_CHARGE:
    case INHIBIT_CHARGE:
// val = POWER_SUPPLY_STATUS_FULL;
    return rc;
    default:
// val = POWER_SUPPLY_STATUS_UNKNOWN;
    return rc;
    }
    }
    static inline int smb_get_current_limit(struct smb_chip *chip,
    unsigned int *val)
    {
    let mut rc: c_int = regmap_read(chip.regmap, chip.base + ICL_STATUS, val);
    if (rc >= 0)
// val *= CURRENT_SCALE_FACTOR;
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn smb_set_current_limit(chip: *mut smb_chip, val: c_uint) -> c_int {
    static int smb_set_current_limit(struct smb_chip *chip, unsigned int val)
    {
    unsigned char val_raw;
    if (val > 4800000) {
    dev_err(chip.dev,
    "Can't set current limit higher than 4800000uA");
    return -EINVAL;
    }
    val_raw = val / CURRENT_SCALE_FACTOR;
    return regmap_write(chip.regmap, chip.base + USBIN_CURRENT_LIMIT_CFG,
    val_raw);
    }
#[no_mangle]
unsafe extern "C" fn smb_status_change_work(work: *mut work_struct) {
    static void smb_status_change_work(struct work_struct *work)
    {
    unsigned int charger_type, current_ua;
    let mut usb_online: c_int = 0;
    int count, rc;
    struct smb_chip *chip;
    chip = container_of(work, struct smb_chip, status_change_work.work);
    smb_get_prop_usb_online(chip, &usb_online);
    if (!usb_online)
    return;
    for (count = 0; count < 3; count++) {
    dev_dbg(chip.dev, "get charger type retry %d\n", count);
    rc = smb_apsd_get_charger_type(chip, &charger_type);
    if (rc != -EAGAIN)
    break;
    msleep(100);
    }
    if (rc < 0 && rc != -EAGAIN) {
    dev_err(chip.dev, "get charger type failed: %d\n", rc);
    return;
    }
    if (rc < 0) {
    rc = regmap_update_bits(chip.regmap, chip.base + CMD_APSD,
    APSD_RERUN_BIT, APSD_RERUN_BIT);
    schedule_delayed_work(&chip.status_change_work,
    msecs_to_jiffies(1000));
    dev_dbg(chip.dev, "get charger type failed, rerun apsd\n");
    return;
    }
    switch (charger_type) {
    case POWER_SUPPLY_USB_TYPE_CDP:
    current_ua = CDP_CURRENT_UA;
    break;
    case POWER_SUPPLY_USB_TYPE_DCP:
    current_ua = DCP_CURRENT_UA;
    break;
    case POWER_SUPPLY_USB_TYPE_SDP:
    default:
    current_ua = SDP_CURRENT_UA;
    break;
    }
    smb_set_current_limit(chip, current_ua);
    power_supply_changed(chip.chg_psy);
    }
    static int smb_get_iio_chan(struct smb_chip *chip, struct iio_channel *chan,
    int *val)
    {
    int rc;
    union power_supply_propval status;
    rc = power_supply_get_property(chip.chg_psy, POWER_SUPPLY_PROP_STATUS,
    &status);
    if (rc < 0 || status.intval != POWER_SUPPLY_STATUS_CHARGING) {
// val = 0;
    return 0;
    }
    if (IS_ERR(chan)) {
    dev_err(chip.dev, "Failed to chan, err = %li", PTR_ERR(chan));
    return PTR_ERR(chan);
    }
    return iio_read_channel_processed(chan, val);
    }
#[no_mangle]
unsafe extern "C" fn smb_get_prop_health(chip: *mut smb_chip, val: *mut c_int) -> c_int {
    static int smb_get_prop_health(struct smb_chip *chip, int *val)
    {
    int rc;
    unsigned int stat;
    rc = regmap_read(chip.regmap, chip.base + BATTERY_CHARGER_STATUS_2,
    &stat);
    if (rc < 0) {
    dev_err(chip.dev, "Couldn't read charger status rc=%d\n", rc);
    return rc;
    }
    switch (stat) {
    case CHARGER_ERROR_STATUS_BAT_OV_BIT:
// val = POWER_SUPPLY_HEALTH_OVERVOLTAGE;
    return 0;
    case BAT_TEMP_STATUS_TOO_COLD_BIT:
// val = POWER_SUPPLY_HEALTH_COLD;
    return 0;
    case BAT_TEMP_STATUS_TOO_HOT_BIT:
// val = POWER_SUPPLY_HEALTH_OVERHEAT;
    return 0;
    case BAT_TEMP_STATUS_COLD_SOFT_LIMIT_BIT:
// val = POWER_SUPPLY_HEALTH_COOL;
    return 0;
    case BAT_TEMP_STATUS_HOT_SOFT_LIMIT_BIT:
// val = POWER_SUPPLY_HEALTH_WARM;
    return 0;
    default:
// val = POWER_SUPPLY_HEALTH_GOOD;
    return 0;
    }
    }
    static int smb_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct smb_chip *chip = power_supply_get_drvdata(psy);
    switch (psp) {
    case POWER_SUPPLY_PROP_MANUFACTURER:
    val.strval = "Qualcomm";
    return 0;
    case POWER_SUPPLY_PROP_MODEL_NAME:
    val.strval = chip.name;
    return 0;
    case POWER_SUPPLY_PROP_CURRENT_MAX:
    return smb_get_current_limit(chip, &val.intval);
    case POWER_SUPPLY_PROP_CURRENT_NOW:
    return smb_get_iio_chan(chip, chip.usb_in_i_chan,
    &val.intval);
    case POWER_SUPPLY_PROP_VOLTAGE_NOW:
    return smb_get_iio_chan(chip, chip.usb_in_v_chan,
    &val.intval);
    case POWER_SUPPLY_PROP_ONLINE:
    return smb_get_prop_usb_online(chip, &val.intval);
    case POWER_SUPPLY_PROP_STATUS:
    return smb_get_prop_status(chip, &val.intval);
    case POWER_SUPPLY_PROP_HEALTH:
    return smb_get_prop_health(chip, &val.intval);
    case POWER_SUPPLY_PROP_USB_TYPE:
    return smb_apsd_get_charger_type(chip, &val.intval);
    default:
    dev_err(chip.dev, "invalid property: %d\n", psp);
    return -EINVAL;
    }
    }
    static int smb_set_property(struct power_supply *psy,
    enum power_supply_property psp,
    const union power_supply_propval *val)
    {
    struct smb_chip *chip = power_supply_get_drvdata(psy);
    switch (psp) {
    case POWER_SUPPLY_PROP_STATUS:
    return regmap_update_bits(chip.regmap, chip.base + USBIN_CMD_IL,
    USBIN_SUSPEND_BIT, !val.intval);
    case POWER_SUPPLY_PROP_CURRENT_MAX:
    return smb_set_current_limit(chip, val.intval);
    default:
    dev_err(chip.dev, "No setter for property: %d\n", psp);
    return -EINVAL;
    }
    }
    static int smb_property_is_writable(struct power_supply *psy,
    enum power_supply_property psp)
    {
    switch (psp) {
    case POWER_SUPPLY_PROP_STATUS:
    case POWER_SUPPLY_PROP_CURRENT_MAX:
    return 1;
    default:
    return 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn smb_handle_batt_overvoltage(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t smb_handle_batt_overvoltage(int irq, void *data)
    {
    struct smb_chip *chip = data;
    unsigned int status;
    regmap_read(chip.regmap, chip.base + BATTERY_CHARGER_STATUS_2,
    &status);
    if (status & CHARGER_ERROR_STATUS_BAT_OV_BIT) {
// The hardware stops charging automatically
    dev_err(chip.dev, "battery overvoltage detected\n");
    power_supply_changed(chip.chg_psy);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn smb_handle_usb_plugin(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t smb_handle_usb_plugin(int irq, void *data)
    {
    struct smb_chip *chip = data;
    power_supply_changed(chip.chg_psy);
    schedule_delayed_work(&chip.status_change_work,
    msecs_to_jiffies(1500));
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn smb_handle_usb_icl_change(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t smb_handle_usb_icl_change(int irq, void *data)
    {
    struct smb_chip *chip = data;
    power_supply_changed(chip.chg_psy);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn smb_handle_wdog_bark(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t smb_handle_wdog_bark(int irq, void *data)
    {
    struct smb_chip *chip = data;
    int rc;
    power_supply_changed(chip.chg_psy);
    rc = regmap_write(chip.regmap, BARK_BITE_WDOG_PET,
    BARK_BITE_WDOG_PET_BIT);
    if (rc < 0)
    dev_err(chip.dev, "Couldn't pet the dog rc=%d\n", rc);
    return IRQ_HANDLED;
    }
    static const struct power_supply_desc smb_psy_desc = {
    .name = "pmi8998_charger",
    .type = POWER_SUPPLY_TYPE_USB,
    .usb_types = BIT(POWER_SUPPLY_USB_TYPE_SDP) |
    BIT(POWER_SUPPLY_USB_TYPE_CDP) |
    BIT(POWER_SUPPLY_USB_TYPE_DCP) |
    BIT(POWER_SUPPLY_USB_TYPE_UNKNOWN),
    .properties = smb_properties,
    .num_properties = ARRAY_SIZE(smb_properties),
    .get_property = smb_get_property,
    .set_property = smb_set_property,
    .property_is_writeable = smb_property_is_writable,
    };
// Init sequence derived from vendor downstream driver
    static const struct smb_init_register smb_init_seq[] = {
    { .addr = AICL_RERUN_TIME_CFG, .mask = AICL_RERUN_TIME_MASK, .val = 0 },
//
// By default configure us as an upstream facing port
// FIXME: This will be handled by the type-c driver
//
    { .addr = TYPE_C_INTRPT_ENB_SOFTWARE_CTRL,
    .mask = TYPEC_POWER_ROLE_CMD_MASK | VCONN_EN_SRC_BIT |
    VCONN_EN_VALUE_BIT,
    .val = VCONN_EN_SRC_BIT },
//
// Disable Type-C factory mode and stay in Attached.SRC state when VCONN
// over-current happens
//
    { .addr = TYPE_C_CFG,
    .mask = FACTORY_MODE_DETECTION_EN_BIT | VCONN_OC_CFG_BIT,
    .val = 0 },
// Configure VBUS for software control
    { .addr = OTG_CFG, .mask = OTG_EN_SRC_CFG_BIT, .val = 0 },
//
// Use VBAT to determine the recharge threshold when battery is full
// rather than the state of charge.
//
    { .addr = FG_UPDATE_CFG_2_SEL,
    .mask = SOC_LT_CHG_RECHARGE_THRESH_SEL_BIT |
    VBT_LT_CHG_RECHARGE_THRESH_SEL_BIT,
    .val = VBT_LT_CHG_RECHARGE_THRESH_SEL_BIT },
// Enable charging
    { .addr = USBIN_OPTIONS_1_CFG, .mask = HVDCP_EN_BIT, .val = 0 },
    { .addr = CHARGING_ENABLE_CMD,
    .mask = CHARGING_ENABLE_CMD_BIT,
    .val = CHARGING_ENABLE_CMD_BIT },
//
// Match downstream defaults
// CHG_EN_SRC_BIT - charger enable is controlled by software
// CHG_EN_POLARITY_BIT - polarity of charge enable pin when in HW control
// pulled low on OnePlus 6 and SHIFT6mq
// PRETOFAST_TRANSITION_CFG_BIT -
// BAT_OV_ECC_BIT -
// I_TERM_BIT - Current termination ?? 0 = enabled
// AUTO_RECHG_BIT - Enable automatic recharge when battery is full
// 0 = enabled
// EN_ANALOG_DROP_IN_VBATT_BIT
// CHARGER_INHIBIT_BIT - Inhibit charging based on battery voltage
// instead of ??
//
    { .addr = CHGR_CFG2,
    .mask = CHG_EN_SRC_BIT | CHG_EN_POLARITY_BIT |
    PRETOFAST_TRANSITION_CFG_BIT | BAT_OV_ECC_BIT | I_TERM_BIT |
    AUTO_RECHG_BIT | EN_ANALOG_DROP_IN_VBATT_BIT |
    CHARGER_INHIBIT_BIT,
    .val = CHARGER_INHIBIT_BIT },
// STAT pin software override, match downstream. Parallel charging?
    { .addr = STAT_CFG,
    .mask = STAT_SW_OVERRIDE_CFG_BIT,
    .val = STAT_SW_OVERRIDE_CFG_BIT },
// Set the default SDP charger type to a 500ma USB 2.0 port
    { .addr = USBIN_ICL_OPTIONS,
    .mask = USB51_MODE_BIT | USBIN_MODE_CHG_BIT,
    .val = USB51_MODE_BIT },
// Disable watchdog
    { .addr = SNARL_BARK_BITE_WD_CFG, .mask = 0xff, .val = 0 },
    { .addr = WD_CFG,
    .mask = WATCHDOG_TRIGGER_AFP_EN_BIT | WDOG_TIMER_EN_ON_PLUGIN_BIT |
    BARK_WDOG_INT_EN_BIT,
    .val = 0 },
// These bits aren't documented anywhere
    { .addr = USBIN_5V_AICL_THRESHOLD_CFG,
    .mask = USBIN_5V_AICL_THRESHOLD_CFG_MASK,
    .val = 0x3 },
    { .addr = USBIN_CONT_AICL_THRESHOLD_CFG,
    .mask = USBIN_CONT_AICL_THRESHOLD_CFG_MASK,
    .val = 0x3 },
//
// Enable Automatic Input Current Limit, this will slowly ramp up the current
// When connected to a wall charger, and automatically stop when it detects
// the charger current limit (voltage drop?) or it reaches the programmed limit.
//
    { .addr = USBIN_AICL_OPTIONS_CFG,
    .mask = USBIN_AICL_START_AT_MAX_BIT | USBIN_AICL_ADC_EN_BIT |
    USBIN_AICL_EN_BIT | SUSPEND_ON_COLLAPSE_USBIN_BIT |
    USBIN_HV_COLLAPSE_RESPONSE_BIT |
    USBIN_LV_COLLAPSE_RESPONSE_BIT,
    .val = USBIN_HV_COLLAPSE_RESPONSE_BIT |
    USBIN_LV_COLLAPSE_RESPONSE_BIT | USBIN_AICL_EN_BIT },
//
// Set pre charge current to default, the OnePlus 6 bootloader
// sets this very conservatively.
//
    { .addr = PRE_CHARGE_CURRENT_CFG,
    .mask = PRE_CHARGE_CURRENT_SETTING_MASK,
    .val = 500000 / CURRENT_SCALE_FACTOR },
//
// This overrides all of the current limit options exposed to userspace
// and prevents the device from pulling more than ~1A. This is done
// to minimise potential fire hazard risks.
//
    { .addr = FAST_CHARGE_CURRENT_CFG,
    .mask = FAST_CHARGE_CURRENT_SETTING_MASK,
    .val = 1000000 / CURRENT_SCALE_FACTOR },
    };
#[no_mangle]
unsafe extern "C" fn smb_init_hw(chip: *mut smb_chip) -> c_int {
    static int smb_init_hw(struct smb_chip *chip)
    {
    int rc, i;
    for (i = 0; i < ARRAY_SIZE(smb_init_seq); i++) {
    dev_dbg(chip.dev, "%d: Writing 0x%02x to 0x%02x\n", i,
    smb_init_seq[i].val, smb_init_seq[i].addr);
    rc = regmap_update_bits(chip.regmap,
    chip.base + smb_init_seq[i].addr,
    smb_init_seq[i].mask,
    smb_init_seq[i].val);
    if (rc < 0)
    return dev_err_probe(chip.dev, rc,
    "%s: init command %d failed\n",
    __func__, i);
    }
    return 0;
    }
    static int smb_init_irq(struct smb_chip *chip, int *irq, const char *name,
    irqreturn_t (*handler)(int irq, void *data))
    {
    int irqnum;
    int rc;
    irqnum = platform_get_irq_byname(to_platform_device(chip.dev), name);
    if (irqnum < 0)
    return irqnum;
    rc = devm_request_threaded_irq(chip.dev, irqnum, core::ptr::null_mut(), handler,
    IRQF_ONESHOT, name, chip);
    if (rc < 0)
    return rc;
    if (irq)
// irq = irqnum;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn smb_probe(pdev: *mut platform_device) -> c_int {
    static int smb_probe(struct platform_device *pdev)
    {
    let mut supply_config: power_supply_config = {};
    struct power_supply_desc *desc;
    struct smb_chip *chip;
    int rc, irq;
    chip = devm_kzalloc(&pdev.dev, sizeof(*chip), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    chip.dev = &pdev.dev;
    chip.name = pdev.name;
    chip.regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!chip.regmap)
    return dev_err_probe(chip.dev, -ENODEV,
    "failed to locate the regmap\n");
    rc = device_property_read_u32(chip.dev, "reg", &chip.base);
    if (rc < 0)
    return dev_err_probe(chip.dev, rc,
    "Couldn't read base address\n");
    chip.usb_in_v_chan = devm_iio_channel_get(chip.dev, "usbin_v");
    if (IS_ERR(chip.usb_in_v_chan))
    return dev_err_probe(chip.dev, PTR_ERR(chip.usb_in_v_chan),
    "Couldn't get usbin_v IIO channel\n");
    chip.usb_in_i_chan = devm_iio_channel_get(chip.dev, "usbin_i");
    if (IS_ERR(chip.usb_in_i_chan)) {
    return dev_err_probe(chip.dev, PTR_ERR(chip.usb_in_i_chan),
    "Couldn't get usbin_i IIO channel\n");
    }
    rc = smb_init_hw(chip);
    if (rc < 0)
    return rc;
    supply_config.drv_data = chip;
    supply_config.fwnode = dev_fwnode(&pdev.dev);
    desc = devm_kzalloc(chip.dev, sizeof(smb_psy_desc), GFP_KERNEL);
    if (!desc)
    return -ENOMEM;
    memcpy(desc, &smb_psy_desc, sizeof(smb_psy_desc));
    desc.name =
    devm_kasprintf(chip.dev, GFP_KERNEL, "%s-charger",
    (const char *)device_get_match_data(chip.dev));
    if (!desc.name)
    return -ENOMEM;
    chip.chg_psy =
    devm_power_supply_register(chip.dev, desc, &supply_config);
    if (IS_ERR(chip.chg_psy))
    return dev_err_probe(chip.dev, PTR_ERR(chip.chg_psy),
    "failed to register power supply\n");
    rc = power_supply_get_battery_info(chip.chg_psy, &chip.batt_info);
    if (rc)
    return dev_err_probe(chip.dev, rc,
    "Failed to get battery info\n");
    rc = devm_delayed_work_autocancel(chip.dev, &chip.status_change_work,
    smb_status_change_work);
    if (rc)
    return dev_err_probe(chip.dev, rc,
    "Failed to init status change work\n");
    rc = (chip.batt_info.voltage_max_design_uv - 3487500) / 7500 + 1;
    rc = regmap_update_bits(chip.regmap, chip.base + FLOAT_VOLTAGE_CFG,
    FLOAT_VOLTAGE_SETTING_MASK, rc);
    if (rc < 0)
    return dev_err_probe(chip.dev, rc, "Couldn't set vbat max\n");
    rc = smb_init_irq(chip, &irq, "bat-ov", smb_handle_batt_overvoltage);
    if (rc < 0)
    return rc;
    rc = smb_init_irq(chip, &chip.cable_irq, "usb-plugin",
    smb_handle_usb_plugin);
    if (rc < 0)
    return rc;
    rc = smb_init_irq(chip, &irq, "usbin-icl-change",
    smb_handle_usb_icl_change);
    if (rc < 0)
    return rc;
    rc = smb_init_irq(chip, &irq, "wdog-bark", smb_handle_wdog_bark);
    if (rc < 0)
    return rc;
    devm_device_init_wakeup(chip.dev);
    rc = devm_pm_set_wake_irq(chip.dev, chip.cable_irq);
    if (rc < 0)
    return dev_err_probe(chip.dev, rc, "Couldn't set wake irq\n");
    platform_set_drvdata(pdev, chip);
// Initialise charger state
    schedule_delayed_work(&chip.status_change_work, 0);
    return 0;
    }
    static const struct of_device_id smb_match_id_table[] = {
    { .compatible = "qcom,pmi8998-charger", .data = "pmi8998" },
    { .compatible = "qcom,pm660-charger", .data = "pm660" },
    { /* sentinal */ }
    };
    MODULE_DEVICE_TABLE(of, smb_match_id_table);
    static struct platform_driver qcom_spmi_smb = {
    .probe = smb_probe,
    .driver = {
    .name = "qcom-smbx-charger",
    .of_match_table = smb_match_id_table,
    },
    };
    module_platform_driver(qcom_spmi_smb);
    MODULE_AUTHOR("Casey Connolly <casey.connolly@linaro.org>");
    MODULE_DESCRIPTION("Qualcomm SMB2 Charger Driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_CONSUMER");
