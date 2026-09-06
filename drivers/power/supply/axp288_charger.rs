//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/axp288_charger.c
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
// axp288_charger.c - X-power AXP288 PMIC Charger driver
//
// Copyright (C) 2016-2017 Hans de Goede <hdegoede@redhat.com>
// Copyright (C) 2014 Intel Corporation
// Author: Ramakrishna Pallala <ramakrishna.pallala@intel.com>
//

pub const VBUS_ISPOUT_CUR_LIM_MASK: c_uint = 0x03;
pub const VBUS_ISPOUT_CUR_LIM_BIT_POS: c_int = 0;
pub const VBUS_ISPOUT_CUR_LIM_900MA: c_uint = 0x0	/* 900mA */;
pub const VBUS_ISPOUT_CUR_LIM_1500MA: c_uint = 0x1	/* 1500mA */;
pub const VBUS_ISPOUT_CUR_LIM_2000MA: c_uint = 0x2	/* 2000mA */;
pub const VBUS_ISPOUT_CUR_NO_LIM: c_uint = 0x3	/* 2500mA */;
pub const VBUS_ISPOUT_VHOLD_SET_MASK: c_uint = 0x38;
pub const VBUS_ISPOUT_VHOLD_SET_BIT_POS: c_uint = 0x3;

pub const VBUS_ISPOUT_VHOLD_SET_4400MV: c_uint = 0x4	/* 4400mV */;

pub const CHRG_CCCV_CC_MASK: c_uint = 0xf		/* 4 bits */;
pub const CHRG_CCCV_CC_BIT_POS: c_int = 0;

pub const CHRG_CCCV_CV_MASK: c_uint = 0x60		/* 2 bits */;
pub const CHRG_CCCV_CV_BIT_POS: c_int = 5;
pub const CHRG_CCCV_CV_4100MV: c_uint = 0x0		/* 4.10V */;
pub const CHRG_CCCV_CV_4150MV: c_uint = 0x1		/* 4.15V */;
pub const CHRG_CCCV_CV_4200MV: c_uint = 0x2		/* 4.20V */;
pub const CHRG_CCCV_CV_4350MV: c_uint = 0x3		/* 4.35V */;

pub const CNTL2_CC_TIMEOUT_MASK: c_uint = 0x3	/* 2 bits */;

pub const CNTL2_CC_TIMEOUT_12HRS: c_uint = 0x3	/* 12 Hrs */;

pub const CNTL2_PC_TIMEOUT_MASK: c_uint = 0xC0;

pub const CNTL2_PC_TIMEOUT_70MINS: c_uint = 0x3;

pub const CHRG_VBUS_ILIM_MASK: c_uint = 0xf0;
pub const CHRG_VBUS_ILIM_BIT_POS: c_int = 4;
pub const CHRG_VBUS_ILIM_100MA: c_uint = 0x0	/* 100mA */;
pub const CHRG_VBUS_ILIM_500MA: c_uint = 0x1	/* 500mA */;
pub const CHRG_VBUS_ILIM_900MA: c_uint = 0x2	/* 900mA */;
pub const CHRG_VBUS_ILIM_1500MA: c_uint = 0x3	/* 1500mA */;
pub const CHRG_VBUS_ILIM_2000MA: c_uint = 0x4	/* 2000mA */;
pub const CHRG_VBUS_ILIM_2500MA: c_uint = 0x5	/* 2500mA */;
pub const CHRG_VBUS_ILIM_3000MA: c_uint = 0x6	/* 3000mA */;
pub const CHRG_VBUS_ILIM_3500MA: c_uint = 0x7	/* 3500mA */;
pub const CHRG_VBUS_ILIM_4000MA: c_uint = 0x8	/* 4000mA */;
pub const CHRG_VLTFC_0C: c_uint = 0xA5	/* 0 DegC */;
pub const CHRG_VHTFC_45C: c_uint = 0x1F	/* 45 DegC */;

    enum {
    VBUS_OV_IRQ = 0,
    CHARGE_DONE_IRQ,
    CHARGE_CHARGING_IRQ,
    BAT_SAFE_QUIT_IRQ,
    BAT_SAFE_ENTER_IRQ,
    QCBTU_IRQ,
    CBTU_IRQ,
    QCBTO_IRQ,
    CBTO_IRQ,
    CHRG_INTR_END,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct axp288_chrg_info {
    pub pdev: *mut platform_device,
    pub regmap: *mut regmap,
    pub regmap_irqc: *mut regmap_irq_chip_data,
    pub irq: [c_int; CHRG_INTR_END],
    pub psy_usb: *mut power_supply,
    pub lock: mutex,
// OTG/Host mode
    struct {
    pub work: work_struct,
    pub cable: *mut extcon_dev,
    pub id_nb: notifier_block,
    pub id_short: bool,
    pub otg: },
// SDP/CDP/DCP USB charging cable notifications
    struct {
    pub edev: *mut extcon_dev,
    pub nb: notifier_block,
    pub work: work_struct,
    pub cable: },
    pub cc: c_int,
    pub cv: c_int,
    pub max_cc: c_int,
    pub max_cv: c_int,
    pub last_updated: c_ulong,
    pub input_status: c_uint,
    pub op_mode: c_uint,
    pub backend_control: c_uint,
    pub valid: bool,
}

#[no_mangle]
pub unsafe extern "C" fn axp288_charger_set_cc(info: *mut axp288_chrg_info, cc: c_int) -> c_int {
    static inline int axp288_charger_set_cc(struct axp288_chrg_info *info, int cc)
    {
    u8 reg_val;
    int ret;
    if (cc < CHRG_CCCV_CC_OFFSET)
    cc = CHRG_CCCV_CC_OFFSET;
#[no_mangle]
pub unsafe extern "C" fn if(info->max_cc: cc >) -> else {
    else if (cc > info.max_cc)
    cc = info.max_cc;
    reg_val = (cc - CHRG_CCCV_CC_OFFSET) / CHRG_CCCV_CC_LSB_RES;
    cc = (reg_val * CHRG_CCCV_CC_LSB_RES) + CHRG_CCCV_CC_OFFSET;
    reg_val = reg_val << CHRG_CCCV_CC_BIT_POS;
    ret = regmap_update_bits(info.regmap,
    AXP20X_CHRG_CTRL1,
    CHRG_CCCV_CC_MASK, reg_val);
    if (ret >= 0)
    info.cc = cc;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn axp288_charger_set_cv(info: *mut axp288_chrg_info, cv: c_int) -> c_int {
    static inline int axp288_charger_set_cv(struct axp288_chrg_info *info, int cv)
    {
    u8 reg_val;
    int ret;
    if (cv >= CV_4350MV) {
    reg_val = CHRG_CCCV_CV_4350MV;
    cv = CV_4350MV;
    } else if (cv >= CV_4200MV) {
    reg_val = CHRG_CCCV_CV_4200MV;
    cv = CV_4200MV;
    } else if (cv >= CV_4150MV) {
    reg_val = CHRG_CCCV_CV_4150MV;
    cv = CV_4150MV;
    } else {
    reg_val = CHRG_CCCV_CV_4100MV;
    cv = CV_4100MV;
    }
    reg_val = reg_val << CHRG_CCCV_CV_BIT_POS;
    ret = regmap_update_bits(info.regmap,
    AXP20X_CHRG_CTRL1,
    CHRG_CCCV_CV_MASK, reg_val);
    if (ret >= 0)
    info.cv = cv;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn axp288_charger_get_vbus_inlmt(info: *mut axp288_chrg_info) -> c_int {
    static int axp288_charger_get_vbus_inlmt(struct axp288_chrg_info *info)
    {
    unsigned int val;
    val = info.backend_control;
    val >>= CHRG_VBUS_ILIM_BIT_POS;
    switch (val) {
    case CHRG_VBUS_ILIM_100MA:
    return 100000;
    case CHRG_VBUS_ILIM_500MA:
    return 500000;
    case CHRG_VBUS_ILIM_900MA:
    return 900000;
    case CHRG_VBUS_ILIM_1500MA:
    return 1500000;
    case CHRG_VBUS_ILIM_2000MA:
    return 2000000;
    case CHRG_VBUS_ILIM_2500MA:
    return 2500000;
    case CHRG_VBUS_ILIM_3000MA:
    return 3000000;
    case CHRG_VBUS_ILIM_3500MA:
    return 3500000;
    default:
// All b1xxx values map to 4000 mA
    return 4000000;
    }
    }
    static inline int axp288_charger_set_vbus_inlmt(struct axp288_chrg_info *info,
    int inlmt)
    {
    int ret;
    u8 reg_val;
    if (inlmt >= 4000000)
    reg_val = CHRG_VBUS_ILIM_4000MA << CHRG_VBUS_ILIM_BIT_POS;
#[no_mangle]
pub unsafe extern "C" fn if(3500000: inlmt >=) -> else {
    else if (inlmt >= 3500000)
    reg_val = CHRG_VBUS_ILIM_3500MA << CHRG_VBUS_ILIM_BIT_POS;
#[no_mangle]
pub unsafe extern "C" fn if(3000000: inlmt >=) -> else {
    else if (inlmt >= 3000000)
    reg_val = CHRG_VBUS_ILIM_3000MA << CHRG_VBUS_ILIM_BIT_POS;
#[no_mangle]
pub unsafe extern "C" fn if(2500000: inlmt >=) -> else {
    else if (inlmt >= 2500000)
    reg_val = CHRG_VBUS_ILIM_2500MA << CHRG_VBUS_ILIM_BIT_POS;
#[no_mangle]
pub unsafe extern "C" fn if(2000000: inlmt >=) -> else {
    else if (inlmt >= 2000000)
    reg_val = CHRG_VBUS_ILIM_2000MA << CHRG_VBUS_ILIM_BIT_POS;
#[no_mangle]
pub unsafe extern "C" fn if(1500000: inlmt >=) -> else {
    else if (inlmt >= 1500000)
    reg_val = CHRG_VBUS_ILIM_1500MA << CHRG_VBUS_ILIM_BIT_POS;
#[no_mangle]
pub unsafe extern "C" fn if(900000: inlmt >=) -> else {
    else if (inlmt >= 900000)
    reg_val = CHRG_VBUS_ILIM_900MA << CHRG_VBUS_ILIM_BIT_POS;
#[no_mangle]
pub unsafe extern "C" fn if(500000: inlmt >=) -> else {
    else if (inlmt >= 500000)
    reg_val = CHRG_VBUS_ILIM_500MA << CHRG_VBUS_ILIM_BIT_POS;
    else
    reg_val = CHRG_VBUS_ILIM_100MA << CHRG_VBUS_ILIM_BIT_POS;
    ret = regmap_update_bits(info.regmap, AXP20X_CHRG_BAK_CTRL,
    CHRG_VBUS_ILIM_MASK, reg_val);
    if (ret < 0)
    dev_err(&info.pdev.dev, "charger BAK control %d\n", ret);
    return ret;
    }
    static int axp288_charger_vbus_path_select(struct axp288_chrg_info *info,
    bool enable)
    {
    int ret;
    if (enable)
    ret = regmap_update_bits(info.regmap, AXP20X_VBUS_IPSOUT_MGMT,
    VBUS_ISPOUT_VBUS_PATH_DIS, 0);
    else
    ret = regmap_update_bits(info.regmap, AXP20X_VBUS_IPSOUT_MGMT,
    VBUS_ISPOUT_VBUS_PATH_DIS, VBUS_ISPOUT_VBUS_PATH_DIS);
    if (ret < 0)
    dev_err(&info.pdev.dev, "axp288 vbus path select %d\n", ret);
    return ret;
    }
    static int axp288_charger_enable_charger(struct axp288_chrg_info *info,
    bool enable)
    {
    int ret;
    if (enable)
    ret = regmap_update_bits(info.regmap, AXP20X_CHRG_CTRL1,
    CHRG_CCCV_CHG_EN, CHRG_CCCV_CHG_EN);
    else
    ret = regmap_update_bits(info.regmap, AXP20X_CHRG_CTRL1,
    CHRG_CCCV_CHG_EN, 0);
    if (ret < 0)
    dev_err(&info.pdev.dev, "axp288 enable charger %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn axp288_get_charger_health(info: *mut axp288_chrg_info) -> c_int {
    static int axp288_get_charger_health(struct axp288_chrg_info *info)
    {
    if (!(info.input_status & PS_STAT_VBUS_PRESENT))
    return POWER_SUPPLY_HEALTH_UNKNOWN;
    if (!(info.input_status & PS_STAT_VBUS_VALID))
    return POWER_SUPPLY_HEALTH_DEAD;
#[no_mangle]
pub unsafe extern "C" fn if(CHRG_STAT_PMIC_OTP: info->op_mode &) -> else {
    else if (info.op_mode & CHRG_STAT_PMIC_OTP)
    return POWER_SUPPLY_HEALTH_OVERHEAT;
#[no_mangle]
pub unsafe extern "C" fn if(CHRG_STAT_BAT_SAFE_MODE: info->op_mode &) -> else {
    else if (info.op_mode & CHRG_STAT_BAT_SAFE_MODE)
    return POWER_SUPPLY_HEALTH_SAFETY_TIMER_EXPIRE;
    else
    return POWER_SUPPLY_HEALTH_GOOD;
    }
    static int axp288_charger_usb_set_property(struct power_supply *psy,
    enum power_supply_property psp,
    const union power_supply_propval *val)
    {
    struct axp288_chrg_info *info = power_supply_get_drvdata(psy);
    let mut ret: c_int = 0;
    int scaled_val;
    mutex_lock(&info.lock);
    switch (psp) {
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT:
    scaled_val = min(val.intval, info.max_cc);
    scaled_val = DIV_ROUND_CLOSEST(scaled_val, 1000);
    ret = axp288_charger_set_cc(info, scaled_val);
    if (ret < 0) {
    dev_warn(&info.pdev.dev, "set charge current failed\n");
    goto out;
    }
    break;
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE:
    scaled_val = DIV_ROUND_CLOSEST(val.intval, 1000);
    scaled_val = min(scaled_val, info.max_cv);
    ret = axp288_charger_set_cv(info, scaled_val);
    if (ret < 0) {
    dev_warn(&info.pdev.dev, "set charge voltage failed\n");
    goto out;
    }
    break;
    case POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT:
    ret = axp288_charger_set_vbus_inlmt(info, val.intval);
    if (ret < 0) {
    dev_warn(&info.pdev.dev, "set input current limit failed\n");
    goto out;
    }
    info.valid = false;
    break;
    default:
    ret = -EINVAL;
    }
    out:
    mutex_unlock(&info.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn axp288_charger_reg_readb(info: *mut axp288_chrg_info, reg: c_int, ret_val: *mut c_uint) -> c_int {
    static int axp288_charger_reg_readb(struct axp288_chrg_info *info, int reg, unsigned int *ret_val)
    {
    int ret;
    ret = regmap_read(info.regmap, reg, ret_val);
    if (ret < 0) {
    dev_err(&info.pdev.dev, "Error %d on reading value from register 0x%04x\n",
    ret,
    reg);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn axp288_charger_usb_update_property(info: *mut axp288_chrg_info) -> c_int {
    static int axp288_charger_usb_update_property(struct axp288_chrg_info *info)
    {
    let mut ret: c_int = 0;
    if (info.valid && time_before(jiffies, info.last_updated + AXP288_REG_UPDATE_INTERVAL))
    return 0;
    dev_dbg(&info.pdev.dev, "Charger updating register values...\n");
    ret = iosf_mbi_block_punit_i2c_access();
    if (ret < 0)
    return ret;
    ret = axp288_charger_reg_readb(info, AXP20X_PWR_INPUT_STATUS, &info.input_status);
    if (ret < 0)
    goto out;
    ret = axp288_charger_reg_readb(info, AXP20X_PWR_OP_MODE, &info.op_mode);
    if (ret < 0)
    goto out;
    ret = axp288_charger_reg_readb(info, AXP20X_CHRG_BAK_CTRL, &info.backend_control);
    if (ret < 0)
    goto out;
    info.last_updated = jiffies;
    info.valid = true;
    out:
    iosf_mbi_unblock_punit_i2c_access();
    return ret;
    }
    static int axp288_charger_usb_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct axp288_chrg_info *info = power_supply_get_drvdata(psy);
    int ret;
    mutex_lock(&info.lock);
    ret = axp288_charger_usb_update_property(info);
    if (ret < 0)
    goto out;
    switch (psp) {
    case POWER_SUPPLY_PROP_PRESENT:
// Check for OTG case first
    if (info.otg.id_short) {
    val.intval = 0;
    break;
    }
    val.intval = (info.input_status & PS_STAT_VBUS_PRESENT) ? 1 : 0;
    break;
    case POWER_SUPPLY_PROP_ONLINE:
// Check for OTG case first
    if (info.otg.id_short) {
    val.intval = 0;
    break;
    }
    val.intval = (info.input_status & PS_STAT_VBUS_VALID) ? 1 : 0;
    break;
    case POWER_SUPPLY_PROP_HEALTH:
    val.intval = axp288_get_charger_health(info);
    break;
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT:
    val.intval = info.cc * 1000;
    break;
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT_MAX:
    val.intval = info.max_cc * 1000;
    break;
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE:
    val.intval = info.cv * 1000;
    break;
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE_MAX:
    val.intval = info.max_cv * 1000;
    break;
    case POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT:
    val.intval = axp288_charger_get_vbus_inlmt(info);
    break;
    default:
    ret = -EINVAL;
    }
    out:
    mutex_unlock(&info.lock);
    return ret;
    }
    static int axp288_charger_property_is_writeable(struct power_supply *psy,
    enum power_supply_property psp)
    {
    int ret;
    switch (psp) {
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT:
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE:
    case POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT:
    ret = 1;
    break;
    default:
    ret = 0;
    }
    return ret;
    }
    static enum power_supply_property axp288_usb_props[] = {
    POWER_SUPPLY_PROP_PRESENT,
    POWER_SUPPLY_PROP_ONLINE,
    POWER_SUPPLY_PROP_TYPE,
    POWER_SUPPLY_PROP_HEALTH,
    POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT,
    POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT_MAX,
    POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE,
    POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE_MAX,
    POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT,
    };
    static const struct power_supply_desc axp288_charger_desc = {
    .name			= "axp288_charger",
    .type			= POWER_SUPPLY_TYPE_USB,
    .properties		= axp288_usb_props,
    .num_properties		= ARRAY_SIZE(axp288_usb_props),
    .get_property		= axp288_charger_usb_get_property,
    .set_property		= axp288_charger_usb_set_property,
    .property_is_writeable	= axp288_charger_property_is_writeable,
    };
#[no_mangle]
unsafe extern "C" fn axp288_charger_irq_thread_handler(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t axp288_charger_irq_thread_handler(int irq, void *dev)
    {
    struct axp288_chrg_info *info = dev;
    int i;
    for (i = 0; i < CHRG_INTR_END; i++) {
    if (info.irq[i] == irq)
    break;
    }
    if (i >= CHRG_INTR_END) {
    dev_warn(&info.pdev.dev, "spurious interrupt!!\n");
    return IRQ_NONE;
    }
    switch (i) {
    case VBUS_OV_IRQ:
    dev_dbg(&info.pdev.dev, "VBUS Over Voltage INTR\n");
    break;
    case CHARGE_DONE_IRQ:
    dev_dbg(&info.pdev.dev, "Charging Done INTR\n");
    break;
    case CHARGE_CHARGING_IRQ:
    dev_dbg(&info.pdev.dev, "Start Charging IRQ\n");
    break;
    case BAT_SAFE_QUIT_IRQ:
    dev_dbg(&info.pdev.dev,
    "Quit Safe Mode(restart timer) Charging IRQ\n");
    break;
    case BAT_SAFE_ENTER_IRQ:
    dev_dbg(&info.pdev.dev,
    "Enter Safe Mode(timer expire) Charging IRQ\n");
    break;
    case QCBTU_IRQ:
    dev_dbg(&info.pdev.dev,
    "Quit Battery Under Temperature(CHRG) INTR\n");
    break;
    case CBTU_IRQ:
    dev_dbg(&info.pdev.dev,
    "Hit Battery Under Temperature(CHRG) INTR\n");
    break;
    case QCBTO_IRQ:
    dev_dbg(&info.pdev.dev,
    "Quit Battery Over Temperature(CHRG) INTR\n");
    break;
    case CBTO_IRQ:
    dev_dbg(&info.pdev.dev,
    "Hit Battery Over Temperature(CHRG) INTR\n");
    break;
    default:
    dev_warn(&info.pdev.dev, "Spurious Interrupt!!!\n");
    goto out;
    }
    mutex_lock(&info.lock);
    info.valid = false;
    mutex_unlock(&info.lock);
    power_supply_changed(info.psy_usb);
    out:
    return IRQ_HANDLED;
    }
//
// The HP Pavilion x2 10 series comes in a number of variants:
// Bay Trail SoC    + AXP288 PMIC, Micro-USB, DMI_BOARD_NAME: "8021"
// Bay Trail SoC    + AXP288 PMIC, Type-C,    DMI_BOARD_NAME: "815D"
// Cherry Trail SoC + AXP288 PMIC, Type-C,    DMI_BOARD_NAME: "813E"
// Cherry Trail SoC + TI PMIC,     Type-C,    DMI_BOARD_NAME: "827C" or "82F4"
//
// The variants with the AXP288 + Type-C connector are all kinds of special:
//
// 1. They use a Type-C connector which the AXP288 does not support, so when
// using a Type-C charger it is not recognized. Unlike most AXP288 devices,
// this model actually has mostly working ACPI AC / Battery code, the ACPI code
// "solves" this by simply setting the input_current_limit to 3A.
// There are still some issues with the ACPI code, so we use this native driver,
// and to solve the charging not working (500mA is not enough) issue we hardcode
// the 3A input_current_limit like the ACPI code does.
//
// 2. If no charger is connected the machine boots with the vbus-path disabled.
// Normally this is done when a 5V boost converter is active to avoid the PMIC
// trying to charge from the 5V boost converter's output. This is done when
// an OTG host cable is inserted and the ID pin on the micro-B receptacle is
// pulled low and the ID pin has an ACPI event handler associated with it
// which re-enables the vbus-path when the ID pin is pulled high when the
// OTG host cable is removed. The Type-C connector has no ID pin, there is
// no ID pin handler and there appears to be no 5V boost converter, so we
// end up not charging because the vbus-path is disabled, until we unplug
// the charger which automatically clears the vbus-path disable bit and then
// on the second plug-in of the adapter we start charging. To solve the not
// charging on first charger plugin we unconditionally enable the vbus-path at
// probe on this model, which is safe since there is no 5V boost converter.
//
    static const struct dmi_system_id axp288_hp_x2_dmi_ids[] = {
    {
    .matches = {
    DMI_EXACT_MATCH(DMI_SYS_VENDOR, "Hewlett-Packard"),
    DMI_EXACT_MATCH(DMI_PRODUCT_NAME, "HP Pavilion x2 Detachable"),
    DMI_EXACT_MATCH(DMI_BOARD_NAME, "815D"),
    },
    },
    {
    .matches = {
    DMI_EXACT_MATCH(DMI_SYS_VENDOR, "HP"),
    DMI_EXACT_MATCH(DMI_PRODUCT_NAME, "HP Pavilion x2 Detachable"),
    DMI_EXACT_MATCH(DMI_BOARD_NAME, "813E"),
    },
    },
    {} /* Terminating entry */
    };
#[no_mangle]
unsafe extern "C" fn axp288_charger_extcon_evt_worker(work: *mut work_struct) {
    static void axp288_charger_extcon_evt_worker(struct work_struct *work)
    {
    struct axp288_chrg_info *info =
    container_of(work, struct axp288_chrg_info, cable.work);
    int ret, current_limit;
    struct extcon_dev *edev = info.cable.edev;
    unsigned int val;
    ret = regmap_read(info.regmap, AXP20X_PWR_INPUT_STATUS, &val);
    if (ret < 0) {
    dev_err(&info.pdev.dev, "Error reading status (%d)\n", ret);
    return;
    }
// Offline? Disable charging and bail
    if (!(val & PS_STAT_VBUS_VALID)) {
    dev_dbg(&info.pdev.dev, "USB charger disconnected\n");
    axp288_charger_enable_charger(info, false);
    mutex_lock(&info.lock);
    info.valid = false;
    mutex_unlock(&info.lock);
    power_supply_changed(info.psy_usb);
    return;
    }
// Determine cable/charger type
    if (dmi_check_system(axp288_hp_x2_dmi_ids)) {
// See comment above axp288_hp_x2_dmi_ids declaration
    dev_dbg(&info.pdev.dev, "HP X2 with Type-C, setting inlmt to 3A\n");
    current_limit = 3000000;
    } else if (extcon_get_state(edev, EXTCON_CHG_USB_SDP) > 0) {
    dev_dbg(&info.pdev.dev, "USB SDP charger is connected\n");
    current_limit = 500000;
    } else if (extcon_get_state(edev, EXTCON_CHG_USB_CDP) > 0) {
    dev_dbg(&info.pdev.dev, "USB CDP charger is connected\n");
    current_limit = 1500000;
    } else if (extcon_get_state(edev, EXTCON_CHG_USB_DCP) > 0) {
    dev_dbg(&info.pdev.dev, "USB DCP charger is connected\n");
    current_limit = 2000000;
    } else {
// Charger type detection still in progress, bail.
    return;
    }
// Set vbus current limit first, then enable charger
    ret = axp288_charger_set_vbus_inlmt(info, current_limit);
    if (ret == 0)
    axp288_charger_enable_charger(info, true);
    else
    dev_err(&info.pdev.dev,
    "error setting current limit (%d)\n", ret);
    mutex_lock(&info.lock);
    info.valid = false;
    mutex_unlock(&info.lock);
    power_supply_changed(info.psy_usb);
    }
    static int axp288_charger_handle_cable_evt(struct notifier_block *nb,
    unsigned long event, void *param)
    {
    struct axp288_chrg_info *info =
    container_of(nb, struct axp288_chrg_info, cable.nb);
    schedule_work(&info.cable.work);
    return NOTIFY_OK;
    }
#[no_mangle]
unsafe extern "C" fn axp288_charger_otg_evt_worker(work: *mut work_struct) {
    static void axp288_charger_otg_evt_worker(struct work_struct *work)
    {
    struct axp288_chrg_info *info =
    container_of(work, struct axp288_chrg_info, otg.work);
    struct extcon_dev *edev = info.otg.cable;
    int ret, usb_host = extcon_get_state(edev, EXTCON_USB_HOST);
    dev_dbg(&info.pdev.dev, "external connector USB-Host is %s\n",
    usb_host ? "attached" : "detached");
//
// Set usb_id_short flag to avoid running charger detection logic
// in case usb host.
//
    info.otg.id_short = usb_host;
// Disable VBUS path before enabling the 5V boost
    ret = axp288_charger_vbus_path_select(info, !info.otg.id_short);
    if (ret < 0)
    dev_warn(&info.pdev.dev, "vbus path disable failed\n");
    }
    static int axp288_charger_handle_otg_evt(struct notifier_block *nb,
    unsigned long event, void *param)
    {
    struct axp288_chrg_info *info =
    container_of(nb, struct axp288_chrg_info, otg.id_nb);
    schedule_work(&info.otg.work);
    return NOTIFY_OK;
    }
#[no_mangle]
unsafe extern "C" fn charger_init_hw_regs(info: *mut axp288_chrg_info) -> c_int {
    static int charger_init_hw_regs(struct axp288_chrg_info *info)
    {
    int ret, cc, cv;
    unsigned int val;
// Program temperature thresholds
    ret = regmap_write(info.regmap, AXP20X_V_LTF_CHRG, CHRG_VLTFC_0C);
    if (ret < 0) {
    dev_err(&info.pdev.dev, "register(%x) write error(%d)\n",
    AXP20X_V_LTF_CHRG, ret);
    return ret;
    }
    ret = regmap_write(info.regmap, AXP20X_V_HTF_CHRG, CHRG_VHTFC_45C);
    if (ret < 0) {
    dev_err(&info.pdev.dev, "register(%x) write error(%d)\n",
    AXP20X_V_HTF_CHRG, ret);
    return ret;
    }
// Do not turn-off charger o/p after charge cycle ends
    ret = regmap_update_bits(info.regmap,
    AXP20X_CHRG_CTRL2,
    CNTL2_CHG_OUT_TURNON, CNTL2_CHG_OUT_TURNON);
    if (ret < 0) {
    dev_err(&info.pdev.dev, "register(%x) write error(%d)\n",
    AXP20X_CHRG_CTRL2, ret);
    return ret;
    }
// Setup ending condition for charging to be 10% of I(chrg)
    ret = regmap_update_bits(info.regmap,
    AXP20X_CHRG_CTRL1,
    CHRG_CCCV_ITERM_20P, 0);
    if (ret < 0) {
    dev_err(&info.pdev.dev, "register(%x) write error(%d)\n",
    AXP20X_CHRG_CTRL1, ret);
    return ret;
    }
// Disable OCV-SOC curve calibration
    ret = regmap_update_bits(info.regmap,
    AXP20X_CC_CTRL,
    FG_CNTL_OCV_ADJ_EN, 0);
    if (ret < 0) {
    dev_err(&info.pdev.dev, "register(%x) write error(%d)\n",
    AXP20X_CC_CTRL, ret);
    return ret;
    }
    if (dmi_check_system(axp288_hp_x2_dmi_ids)) {
// See comment above axp288_hp_x2_dmi_ids declaration
    ret = axp288_charger_vbus_path_select(info, true);
    if (ret < 0)
    return ret;
    } else {
// Set Vhold to the factory default / recommended 4.4V
    val = VBUS_ISPOUT_VHOLD_SET_4400MV << VBUS_ISPOUT_VHOLD_SET_BIT_POS;
    ret = regmap_update_bits(info.regmap, AXP20X_VBUS_IPSOUT_MGMT,
    VBUS_ISPOUT_VHOLD_SET_MASK, val);
    if (ret < 0) {
    dev_err(&info.pdev.dev, "register(%x) write error(%d)\n",
    AXP20X_VBUS_IPSOUT_MGMT, ret);
    return ret;
    }
    }
// Read current charge voltage and current limit
    ret = regmap_read(info.regmap, AXP20X_CHRG_CTRL1, &val);
    if (ret < 0) {
    dev_err(&info.pdev.dev, "register(%x) read error(%d)\n",
    AXP20X_CHRG_CTRL1, ret);
    return ret;
    }
// Determine charge voltage
    cv = (val & CHRG_CCCV_CV_MASK) >> CHRG_CCCV_CV_BIT_POS;
    switch (cv) {
    case CHRG_CCCV_CV_4100MV:
    info.cv = CV_4100MV;
    break;
    case CHRG_CCCV_CV_4150MV:
    info.cv = CV_4150MV;
    break;
    case CHRG_CCCV_CV_4200MV:
    info.cv = CV_4200MV;
    break;
    case CHRG_CCCV_CV_4350MV:
    info.cv = CV_4350MV;
    break;
    }
// Determine charge current limit
    cc = (val & CHRG_CCCV_CC_MASK) >> CHRG_CCCV_CC_BIT_POS;
    cc = (cc * CHRG_CCCV_CC_LSB_RES) + CHRG_CCCV_CC_OFFSET;
    info.cc = cc;
//
// Do not allow the user to configure higher settings then those
// set by the firmware
//
    info.max_cv = info.cv;
    info.max_cc = info.cc;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn axp288_charger_probe(pdev: *mut platform_device) -> c_int {
    static int axp288_charger_probe(struct platform_device *pdev)
    {
    int ret, i, pirq;
    struct axp288_chrg_info *info;
    struct device *dev = &pdev.dev;
    struct axp20x_dev *axp20x = dev_get_drvdata(pdev.dev.parent);
    let mut charger_cfg: power_supply_config = {};
    const char *extcon_name = core::ptr::null_mut();
    unsigned int val;
//
// Normally the native AXP288 fg/charger drivers are preferred but
// on some devices the ACPI drivers should be used instead.
//
    if (!acpi_quirk_skip_acpi_ac_and_battery())
    return -ENODEV;
//
// On some devices the fuelgauge and charger parts of the axp288 are
// not used, check that the fuelgauge is enabled (CC_CTRL != 0).
//
    ret = regmap_read(axp20x.regmap, AXP20X_CC_CTRL, &val);
    if (ret < 0)
    return ret;
    if (val == 0)
    return -ENODEV;
    info = devm_kzalloc(dev, sizeof(*info), GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    mutex_init(&info.lock);
    info.pdev = pdev;
    info.regmap = axp20x.regmap;
    info.regmap_irqc = axp20x.regmap_irqc;
    info.cable.edev = extcon_get_extcon_dev(AXP288_EXTCON_DEV_NAME);
    if (IS_ERR(info.cable.edev))
    return dev_err_probe(dev, PTR_ERR(info.cable.edev),
    "extcon_get_extcon_dev(%s) failed\n",
    AXP288_EXTCON_DEV_NAME);
//
// On devices with broken ACPI GPIO event handlers there also is no ACPI
// "INT3496" (USB_HOST_EXTCON_HID) device. x86-android-tablets.ko
// instantiates an "intel-int3496" extcon on these devs as a workaround.
//
    if (acpi_quirk_skip_gpio_event_handlers())
    extcon_name = "intel-int3496";
#[no_mangle]
pub unsafe extern "C" fn if(_arg: acpi_dev_present(USB_HOST_EXTCON_HID, _arg: NULL, _arg: -1)) -> else {
    else if (acpi_dev_present(USB_HOST_EXTCON_HID, core::ptr::null_mut(), -1))
    extcon_name = USB_HOST_EXTCON_NAME;
    if (extcon_name) {
    info.otg.cable = extcon_get_extcon_dev(extcon_name);
    if (IS_ERR(info.otg.cable))
    return dev_err_probe(dev, PTR_ERR(info.otg.cable),
    "extcon_get_extcon_dev(%s) failed\n",
    USB_HOST_EXTCON_NAME);
    dev_info(dev, "Using " USB_HOST_EXTCON_HID " extcon for usb-id\n");
    }
    platform_set_drvdata(pdev, info);
    ret = charger_init_hw_regs(info);
    if (ret)
    return ret;
// Register with power supply class
    charger_cfg.drv_data = info;
    info.psy_usb = devm_power_supply_register(dev, &axp288_charger_desc,
    &charger_cfg);
    if (IS_ERR(info.psy_usb))
    return dev_err_probe(dev, PTR_ERR(info.psy_usb),
    "failed to register power supply: %d\n", ret);
// Cancel our work on cleanup, register this before the notifiers
    ret = devm_work_autocancel(dev, &info.cable.work,
    axp288_charger_extcon_evt_worker);
    if (ret)
    return ret;
// Register for extcon notification
    info.cable.nb.notifier_call = axp288_charger_handle_cable_evt;
    ret = devm_extcon_register_notifier_all(dev, info.cable.edev,
    &info.cable.nb);
    if (ret)
    return dev_err_probe(dev, ret, "failed to register cable extcon notifier\n");
    schedule_work(&info.cable.work);
    ret = devm_work_autocancel(dev, &info.otg.work,
    axp288_charger_otg_evt_worker);
    if (ret)
    return ret;
// Register for OTG notification
    info.otg.id_nb.notifier_call = axp288_charger_handle_otg_evt;
    if (info.otg.cable) {
    ret = devm_extcon_register_notifier(dev, info.otg.cable,
    EXTCON_USB_HOST, &info.otg.id_nb);
    if (ret)
    return dev_err_probe(dev, ret,
    "failed to register EXTCON_USB_HOST notifier\n");
    schedule_work(&info.otg.work);
    }
// Register charger interrupts
    for (i = 0; i < CHRG_INTR_END; i++) {
    pirq = platform_get_irq(info.pdev, i);
    if (pirq < 0)
    return pirq;
    info.irq[i] = regmap_irq_get_virq(info.regmap_irqc, pirq);
    if (info.irq[i] < 0) {
    dev_warn(&info.pdev.dev,
    "failed to get virtual interrupt=%d\n", pirq);
    return info.irq[i];
    }
    ret = devm_request_threaded_irq(&info.pdev.dev, info.irq[i],
    core::ptr::null_mut(), axp288_charger_irq_thread_handler,
    IRQF_ONESHOT, info.pdev.name, info);
    if (ret)
    return ret;
    }
    return 0;
    }
    static const struct platform_device_id axp288_charger_id_table[] = {
    { .name = "axp288_charger" },
    { }
    };
    MODULE_DEVICE_TABLE(platform, axp288_charger_id_table);
    static struct platform_driver axp288_charger_driver = {
    .probe = axp288_charger_probe,
    .id_table = axp288_charger_id_table,
    .driver = {
    .name = "axp288_charger",
    },
    };
    module_platform_driver(axp288_charger_driver);
    MODULE_AUTHOR("Ramakrishna Pallala <ramakrishna.pallala@intel.com>");
    MODULE_DESCRIPTION("X-power AXP288 Charger Driver");
    MODULE_LICENSE("GPL v2");
