//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/mt6360_charger.c
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
// Copyright (c) 2021 MediaTek Inc.
//

pub const MT6360_PMU_CHG_CTRL1: c_uint = 0x311;
pub const MT6360_PMU_CHG_CTRL2: c_uint = 0x312;
pub const MT6360_PMU_CHG_CTRL3: c_uint = 0x313;
pub const MT6360_PMU_CHG_CTRL4: c_uint = 0x314;
pub const MT6360_PMU_CHG_CTRL5: c_uint = 0x315;
pub const MT6360_PMU_CHG_CTRL6: c_uint = 0x316;
pub const MT6360_PMU_CHG_CTRL7: c_uint = 0x317;
pub const MT6360_PMU_CHG_CTRL8: c_uint = 0x318;
pub const MT6360_PMU_CHG_CTRL9: c_uint = 0x319;
pub const MT6360_PMU_CHG_CTRL10: c_uint = 0x31A;
pub const MT6360_PMU_DEVICE_TYPE: c_uint = 0x322;
pub const MT6360_PMU_USB_STATUS1: c_uint = 0x327;
pub const MT6360_PMU_CHG_STAT: c_uint = 0x34A;
pub const MT6360_PMU_CHG_CTRL19: c_uint = 0x361;
pub const MT6360_PMU_FOD_STAT: c_uint = 0x3E7;
// MT6360_PMU_CHG_CTRL1

// MT6360_PMU_CHG_CTRL2

// MT6360_PMU_CHG_CTRL3

// MT6360_PMU_CHG_CTRL4

// MT6360_PMU_CHG_CTRL5

// MT6360_PMU_CHG_CTRL6

// MT6360_PMU_CHG_CTRL7

// MT6360_PMU_CHG_CTRL8

// MT6360_PMU_CHG_CTRL9

// MT6360_PMU_CHG_CTRL10

// MT6360_PMU_DEVICE_TYPE

// MT6360_PMU_USB_STATUS1

// MT6360_PMU_CHG_STAT

// MT6360_PMU_CHG_CTRL19

// MT6360_PMU_FOD_STAT

// uV
pub const MT6360_VMIVR_MIN: c_int = 3900000;
pub const MT6360_VMIVR_MAX: c_int = 13400000;
pub const MT6360_VMIVR_STEP: c_int = 100000;
// uA
pub const MT6360_ICHG_MIN: c_int = 100000;
pub const MT6360_ICHG_MAX: c_int = 5000000;
pub const MT6360_ICHG_STEP: c_int = 100000;
// uV
pub const MT6360_VOREG_MIN: c_int = 3900000;
pub const MT6360_VOREG_MAX: c_int = 4710000;
pub const MT6360_VOREG_STEP: c_int = 10000;
// uA
pub const MT6360_AICR_MIN: c_int = 100000;
pub const MT6360_AICR_MAX: c_int = 3250000;
pub const MT6360_AICR_STEP: c_int = 50000;
// uA
pub const MT6360_IPREC_MIN: c_int = 100000;
pub const MT6360_IPREC_MAX: c_int = 850000;
pub const MT6360_IPREC_STEP: c_int = 50000;
// uA
pub const MT6360_IEOC_MIN: c_int = 100000;
pub const MT6360_IEOC_MAX: c_int = 850000;
pub const MT6360_IEOC_STEP: c_int = 50000;
    enum {
    MT6360_RANGE_VMIVR,
    MT6360_RANGE_ICHG,
    MT6360_RANGE_VOREG,
    MT6360_RANGE_AICR,
    MT6360_RANGE_IPREC,
    MT6360_RANGE_IEOC,
    MT6360_RANGE_MAX,
    };
    static const struct linear_range mt6360_chg_range[MT6360_RANGE_MAX] = {
    LINEAR_RANGE_IDX(MT6360_RANGE_VMIVR, 3900000, 0, 0x5F, 100000),
    LINEAR_RANGE_IDX(MT6360_RANGE_ICHG, 100000, 0, 0x31, 100000),
    LINEAR_RANGE_IDX(MT6360_RANGE_VOREG, 3900000, 0, 0x51, 10000),
    LINEAR_RANGE_IDX(MT6360_RANGE_AICR, 100000, 0, 0x3F, 50000),
    LINEAR_RANGE_IDX(MT6360_RANGE_IPREC, 100000, 0, 0x0F, 50000),
    LINEAR_RANGE_IDX(MT6360_RANGE_IEOC, 100000, 0, 0x0F, 50000),
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt6360_chg_info {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub psy_desc: power_supply_desc,
    pub psy: *mut power_supply,
    pub otg_rdev: *mut regulator_dev,
    pub chgdet_lock: mutex,
    pub vinovp: u32,
    pub pwr_rdy: bool,
    pub bc12_en: bool,
    pub psy_usb_type: c_int,
    pub chrdet_work: work_struct,
}

    enum mt6360_iinlmtsel {
    MT6360_IINLMTSEL_AICR_3250 = 0,
    MT6360_IINLMTSEL_CHG_TYPE,
    MT6360_IINLMTSEL_AICR,
    MT6360_IINLMTSEL_LOWER_LEVEL,
    };
    enum mt6360_pmu_chg_type {
    MT6360_CHG_TYPE_NOVBUS = 0,
    MT6360_CHG_TYPE_UNDER_GOING,
    MT6360_CHG_TYPE_SDP,
    MT6360_CHG_TYPE_SDPNSTD,
    MT6360_CHG_TYPE_DCP,
    MT6360_CHG_TYPE_CDP,
    MT6360_CHG_TYPE_DISABLE_BC12,
    MT6360_CHG_TYPE_MAX,
    };
    static int mt6360_get_chrdet_ext_stat(struct mt6360_chg_info *mci,
    bool *pwr_rdy)
    {
    int ret;
    unsigned int regval;
    ret = regmap_read(mci.regmap, MT6360_PMU_FOD_STAT, &regval);
    if (ret < 0)
    return ret;
// pwr_rdy = (regval & MT6360_CHRDET_EXT_MASK) ? true : false;
    return 0;
    }
    static int mt6360_charger_get_online(struct mt6360_chg_info *mci,
    union power_supply_propval *val)
    {
    int ret;
    bool pwr_rdy;
    ret = mt6360_get_chrdet_ext_stat(mci, &pwr_rdy);
    if (ret < 0)
    return ret;
    val.intval = pwr_rdy ? true : false;
    return 0;
    }
    static int mt6360_charger_get_status(struct mt6360_chg_info *mci,
    union power_supply_propval *val)
    {
    int status, ret;
    unsigned int regval;
    bool pwr_rdy;
    ret = mt6360_get_chrdet_ext_stat(mci, &pwr_rdy);
    if (ret < 0)
    return ret;
    if (!pwr_rdy) {
    status = POWER_SUPPLY_STATUS_DISCHARGING;
    goto out;
    }
    ret = regmap_read(mci.regmap, MT6360_PMU_CHG_STAT, &regval);
    if (ret < 0)
    return ret;
    regval &= MT6360_CHG_STAT_MASK;
    regval >>= MT6360_CHG_STAT_SHFT;
    switch (regval) {
    case 0x0:
    status = POWER_SUPPLY_STATUS_NOT_CHARGING;
    break;
    case 0x1:
    status = POWER_SUPPLY_STATUS_CHARGING;
    break;
    case 0x2:
    status = POWER_SUPPLY_STATUS_FULL;
    break;
    default:
    ret = -EIO;
    }
    out:
    if (!ret)
    val.intval = status;
    return ret;
    }
    static int mt6360_charger_get_charge_type(struct mt6360_chg_info *mci,
    union power_supply_propval *val)
    {
    int type, ret;
    unsigned int regval;
    u8 chg_stat;
    ret = regmap_read(mci.regmap, MT6360_PMU_CHG_STAT, &regval);
    if (ret < 0)
    return ret;
    chg_stat = (regval & MT6360_CHG_STAT_MASK) >> MT6360_CHG_STAT_SHFT;
    switch (chg_stat) {
    case 0x01: /* Charge in Progress */
    if (regval & MT6360_VBAT_LVL_MASK)
    type = POWER_SUPPLY_CHARGE_TYPE_FAST;
    else
    type = POWER_SUPPLY_CHARGE_TYPE_TRICKLE;
    break;
    case 0x00: /* Not Charging */
    case 0x02: /* Charge Done */
    case 0x03: /* Charge Fault */
    default:
    type = POWER_SUPPLY_CHARGE_TYPE_NONE;
    break;
    }
    val.intval = type;
    return 0;
    }
    static int mt6360_charger_get_ichg(struct mt6360_chg_info *mci,
    union power_supply_propval *val)
    {
    int ret;
    u32 sel, value;
    ret = regmap_read(mci.regmap, MT6360_PMU_CHG_CTRL7, &sel);
    if (ret < 0)
    return ret;
    sel = (sel & MT6360_ICHG_MASK) >> MT6360_ICHG_SHFT;
    ret = linear_range_get_value(&mt6360_chg_range[MT6360_RANGE_ICHG], sel, &value);
    if (!ret)
    val.intval = value;
    return ret;
    }
    static int mt6360_charger_get_max_ichg(struct mt6360_chg_info *mci,
    union power_supply_propval *val)
    {
    val.intval = MT6360_ICHG_MAX;
    return 0;
    }
    static int mt6360_charger_get_cv(struct mt6360_chg_info *mci,
    union power_supply_propval *val)
    {
    int ret;
    u32 sel, value;
    ret = regmap_read(mci.regmap, MT6360_PMU_CHG_CTRL4, &sel);
    if (ret < 0)
    return ret;
    sel = (sel & MT6360_VOREG_MASK) >> MT6360_VOREG_SHFT;
    ret = linear_range_get_value(&mt6360_chg_range[MT6360_RANGE_VOREG], sel, &value);
    if (!ret)
    val.intval = value;
    return ret;
    }
    static int mt6360_charger_get_max_cv(struct mt6360_chg_info *mci,
    union power_supply_propval *val)
    {
    val.intval = MT6360_VOREG_MAX;
    return 0;
    }
    static int mt6360_charger_get_aicr(struct mt6360_chg_info *mci,
    union power_supply_propval *val)
    {
    int ret;
    u32 sel, value;
    ret = regmap_read(mci.regmap, MT6360_PMU_CHG_CTRL3, &sel);
    if (ret < 0)
    return ret;
    sel = (sel & MT6360_IAICR_MASK) >> MT6360_IAICR_SHFT;
    ret = linear_range_get_value(&mt6360_chg_range[MT6360_RANGE_AICR], sel, &value);
    if (!ret)
    val.intval = value;
    return ret;
    }
    static int mt6360_charger_get_mivr(struct mt6360_chg_info *mci,
    union power_supply_propval *val)
    {
    int ret;
    u32 sel, value;
    ret = regmap_read(mci.regmap, MT6360_PMU_CHG_CTRL6, &sel);
    if (ret < 0)
    return ret;
    sel = (sel & MT6360_VMIVR_MASK) >> MT6360_VMIVR_SHFT;
    ret = linear_range_get_value(&mt6360_chg_range[MT6360_RANGE_VMIVR], sel, &value);
    if (!ret)
    val.intval = value;
    return ret;
    }
    static int mt6360_charger_get_iprechg(struct mt6360_chg_info *mci,
    union power_supply_propval *val)
    {
    int ret;
    u32 sel, value;
    ret = regmap_read(mci.regmap, MT6360_PMU_CHG_CTRL8, &sel);
    if (ret < 0)
    return ret;
    sel = (sel & MT6360_IPREC_MASK) >> MT6360_IPREC_SHFT;
    ret = linear_range_get_value(&mt6360_chg_range[MT6360_RANGE_IPREC], sel, &value);
    if (!ret)
    val.intval = value;
    return ret;
    }
    static int mt6360_charger_get_ieoc(struct mt6360_chg_info *mci,
    union power_supply_propval *val)
    {
    int ret;
    u32 sel, value;
    ret = regmap_read(mci.regmap, MT6360_PMU_CHG_CTRL9, &sel);
    if (ret < 0)
    return ret;
    sel = (sel & MT6360_IEOC_MASK) >> MT6360_IEOC_SHFT;
    ret = linear_range_get_value(&mt6360_chg_range[MT6360_RANGE_IEOC], sel, &value);
    if (!ret)
    val.intval = value;
    return ret;
    }
    static int mt6360_charger_set_online(struct mt6360_chg_info *mci,
    const union power_supply_propval *val)
    {
    let mut force_sleep: u8 = val.intval ? 0 : 1;
    return regmap_update_bits(mci.regmap,
    MT6360_PMU_CHG_CTRL1,
    MT6360_FSLP_MASK,
    force_sleep << MT6360_FSLP_SHFT);
    }
    static int mt6360_charger_set_ichg(struct mt6360_chg_info *mci,
    const union power_supply_propval *val)
    {
    u32 sel;
    linear_range_get_selector_within(&mt6360_chg_range[MT6360_RANGE_ICHG], val.intval, &sel);
    return regmap_update_bits(mci.regmap,
    MT6360_PMU_CHG_CTRL7,
    MT6360_ICHG_MASK,
    sel << MT6360_ICHG_SHFT);
    }
    static int mt6360_charger_set_cv(struct mt6360_chg_info *mci,
    const union power_supply_propval *val)
    {
    u32 sel;
    linear_range_get_selector_within(&mt6360_chg_range[MT6360_RANGE_VOREG], val.intval, &sel);
    return regmap_update_bits(mci.regmap,
    MT6360_PMU_CHG_CTRL4,
    MT6360_VOREG_MASK,
    sel << MT6360_VOREG_SHFT);
    }
    static int mt6360_charger_set_aicr(struct mt6360_chg_info *mci,
    const union power_supply_propval *val)
    {
    u32 sel;
    linear_range_get_selector_within(&mt6360_chg_range[MT6360_RANGE_AICR], val.intval, &sel);
    return regmap_update_bits(mci.regmap,
    MT6360_PMU_CHG_CTRL3,
    MT6360_IAICR_MASK,
    sel << MT6360_IAICR_SHFT);
    }
    static int mt6360_charger_set_mivr(struct mt6360_chg_info *mci,
    const union power_supply_propval *val)
    {
    u32 sel;
    linear_range_get_selector_within(&mt6360_chg_range[MT6360_RANGE_VMIVR], val.intval, &sel);
    return regmap_update_bits(mci.regmap,
    MT6360_PMU_CHG_CTRL3,
    MT6360_VMIVR_MASK,
    sel << MT6360_VMIVR_SHFT);
    }
    static int mt6360_charger_set_iprechg(struct mt6360_chg_info *mci,
    const union power_supply_propval *val)
    {
    u32 sel;
    linear_range_get_selector_within(&mt6360_chg_range[MT6360_RANGE_IPREC], val.intval, &sel);
    return regmap_update_bits(mci.regmap,
    MT6360_PMU_CHG_CTRL8,
    MT6360_IPREC_MASK,
    sel << MT6360_IPREC_SHFT);
    }
    static int mt6360_charger_set_ieoc(struct mt6360_chg_info *mci,
    const union power_supply_propval *val)
    {
    u32 sel;
    linear_range_get_selector_within(&mt6360_chg_range[MT6360_RANGE_IEOC], val.intval, &sel);
    return regmap_update_bits(mci.regmap,
    MT6360_PMU_CHG_CTRL9,
    MT6360_IEOC_MASK,
    sel << MT6360_IEOC_SHFT);
    }
    static int mt6360_charger_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct mt6360_chg_info *mci = power_supply_get_drvdata(psy);
    let mut ret: c_int = 0;
    switch (psp) {
    case POWER_SUPPLY_PROP_ONLINE:
    ret = mt6360_charger_get_online(mci, val);
    break;
    case POWER_SUPPLY_PROP_STATUS:
    ret = mt6360_charger_get_status(mci, val);
    break;
    case POWER_SUPPLY_PROP_CHARGE_TYPE:
    ret = mt6360_charger_get_charge_type(mci, val);
    break;
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT:
    ret = mt6360_charger_get_ichg(mci, val);
    break;
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT_MAX:
    ret = mt6360_charger_get_max_ichg(mci, val);
    break;
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE:
    ret = mt6360_charger_get_cv(mci, val);
    break;
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE_MAX:
    ret = mt6360_charger_get_max_cv(mci, val);
    break;
    case POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT:
    ret = mt6360_charger_get_aicr(mci, val);
    break;
    case POWER_SUPPLY_PROP_INPUT_VOLTAGE_LIMIT:
    ret = mt6360_charger_get_mivr(mci, val);
    break;
    case POWER_SUPPLY_PROP_PRECHARGE_CURRENT:
    ret = mt6360_charger_get_iprechg(mci, val);
    break;
    case POWER_SUPPLY_PROP_CHARGE_TERM_CURRENT:
    ret = mt6360_charger_get_ieoc(mci, val);
    break;
    case POWER_SUPPLY_PROP_USB_TYPE:
    val.intval = mci.psy_usb_type;
    break;
    default:
    ret = -ENODATA;
    }
    return ret;
    }
    static int mt6360_charger_set_property(struct power_supply *psy,
    enum power_supply_property psp,
    const union power_supply_propval *val)
    {
    struct mt6360_chg_info *mci = power_supply_get_drvdata(psy);
    int ret;
    switch (psp) {
    case POWER_SUPPLY_PROP_ONLINE:
    ret = mt6360_charger_set_online(mci, val);
    break;
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT:
    ret = mt6360_charger_set_ichg(mci, val);
    break;
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE:
    ret = mt6360_charger_set_cv(mci, val);
    break;
    case POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT:
    ret = mt6360_charger_set_aicr(mci, val);
    break;
    case POWER_SUPPLY_PROP_INPUT_VOLTAGE_LIMIT:
    ret = mt6360_charger_set_mivr(mci, val);
    break;
    case POWER_SUPPLY_PROP_PRECHARGE_CURRENT:
    ret = mt6360_charger_set_iprechg(mci, val);
    break;
    case POWER_SUPPLY_PROP_CHARGE_TERM_CURRENT:
    ret = mt6360_charger_set_ieoc(mci, val);
    break;
    default:
    ret = -EINVAL;
    }
    return ret;
    }
    static int mt6360_charger_property_is_writeable(struct power_supply *psy,
    enum power_supply_property psp)
    {
    switch (psp) {
    case POWER_SUPPLY_PROP_ONLINE:
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT:
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE:
    case POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT:
    case POWER_SUPPLY_PROP_INPUT_VOLTAGE_LIMIT:
    case POWER_SUPPLY_PROP_PRECHARGE_CURRENT:
    case POWER_SUPPLY_PROP_CHARGE_TERM_CURRENT:
    return 1;
    default:
    return 0;
    }
    }
    static enum power_supply_property mt6360_charger_properties[] = {
    POWER_SUPPLY_PROP_ONLINE,
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_CHARGE_TYPE,
    POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT,
    POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT_MAX,
    POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE,
    POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE_MAX,
    POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT,
    POWER_SUPPLY_PROP_INPUT_VOLTAGE_LIMIT,
    POWER_SUPPLY_PROP_PRECHARGE_CURRENT,
    POWER_SUPPLY_PROP_CHARGE_TERM_CURRENT,
    POWER_SUPPLY_PROP_USB_TYPE,
    };
    static const struct power_supply_desc mt6360_charger_desc = {
    .type			= POWER_SUPPLY_TYPE_USB,
    .properties		= mt6360_charger_properties,
    .num_properties		= ARRAY_SIZE(mt6360_charger_properties),
    .get_property		= mt6360_charger_get_property,
    .set_property		= mt6360_charger_set_property,
    .property_is_writeable	= mt6360_charger_property_is_writeable,
    .usb_types		= BIT(POWER_SUPPLY_USB_TYPE_SDP) |
    BIT(POWER_SUPPLY_USB_TYPE_CDP) |
    BIT(POWER_SUPPLY_USB_TYPE_DCP) |
    BIT(POWER_SUPPLY_USB_TYPE_UNKNOWN),
    };
    static const struct regulator_ops mt6360_chg_otg_ops = {
    .list_voltage = regulator_list_voltage_linear,
    .enable = regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .is_enabled = regulator_is_enabled_regmap,
    .set_voltage_sel = regulator_set_voltage_sel_regmap,
    .get_voltage_sel = regulator_get_voltage_sel_regmap,
    };
    static const struct regulator_desc mt6360_otg_rdesc = {
    .of_match = "usb-otg-vbus-regulator",
    .name = "usb-otg-vbus",
    .ops = &mt6360_chg_otg_ops,
    .owner = THIS_MODULE,
    .type = REGULATOR_VOLTAGE,
    .min_uV = 4425000,
    .uV_step = 25000,
    .n_voltages = 57,
    .vsel_reg = MT6360_PMU_CHG_CTRL5,
    .vsel_mask = MT6360_VOBST_MASK,
    .enable_reg = MT6360_PMU_CHG_CTRL1,
    .enable_mask = MT6360_OPA_MODE_MASK,
    };
#[no_mangle]
unsafe extern "C" fn mt6360_pmu_attach_i_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t mt6360_pmu_attach_i_handler(int irq, void *data)
    {
    struct mt6360_chg_info *mci = data;
    int ret;
    unsigned int usb_status;
    int last_usb_type;
    mutex_lock(&mci.chgdet_lock);
    if (!mci.bc12_en) {
    dev_warn(mci.dev, "Received attach interrupt, bc12 disabled, ignore irq\n");
    goto out;
    }
    last_usb_type = mci.psy_usb_type;
// Plug in
    ret = regmap_read(mci.regmap, MT6360_PMU_USB_STATUS1, &usb_status);
    if (ret < 0)
    goto out;
    usb_status &= MT6360_USB_STATUS_MASK;
    usb_status >>= MT6360_USB_STATUS_SHFT;
    switch (usb_status) {
    case MT6360_CHG_TYPE_NOVBUS:
    dev_dbg(mci.dev, "Received attach interrupt, no vbus\n");
    goto out;
    case MT6360_CHG_TYPE_UNDER_GOING:
    dev_dbg(mci.dev, "Received attach interrupt, under going...\n");
    goto out;
    case MT6360_CHG_TYPE_SDP:
    mci.psy_usb_type = POWER_SUPPLY_USB_TYPE_SDP;
    break;
    case MT6360_CHG_TYPE_SDPNSTD:
    mci.psy_usb_type = POWER_SUPPLY_USB_TYPE_SDP;
    break;
    case MT6360_CHG_TYPE_CDP:
    mci.psy_usb_type = POWER_SUPPLY_USB_TYPE_CDP;
    break;
    case MT6360_CHG_TYPE_DCP:
    mci.psy_usb_type = POWER_SUPPLY_USB_TYPE_DCP;
    break;
    case MT6360_CHG_TYPE_DISABLE_BC12:
    dev_dbg(mci.dev, "Received attach interrupt, bc12 detect not enable\n");
    goto out;
    default:
    mci.psy_usb_type = POWER_SUPPLY_USB_TYPE_UNKNOWN;
    dev_dbg(mci.dev, "Received attach interrupt, reserved address\n");
    goto out;
    }
    dev_dbg(mci.dev, "Received attach interrupt, chg_type = %d\n", mci.psy_usb_type);
    if (last_usb_type != mci.psy_usb_type)
    power_supply_changed(mci.psy);
    out:
    mutex_unlock(&mci.chgdet_lock);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn mt6360_handle_chrdet_ext_evt(mci: *mut mt6360_chg_info) {
    static void mt6360_handle_chrdet_ext_evt(struct mt6360_chg_info *mci)
    {
    int ret;
    bool pwr_rdy;
    mutex_lock(&mci.chgdet_lock);
    ret = mt6360_get_chrdet_ext_stat(mci, &pwr_rdy);
    if (ret < 0)
    goto out;
    if (mci.pwr_rdy == pwr_rdy) {
    dev_dbg(mci.dev, "Received vbus interrupt, pwr_rdy is same(%d)\n", pwr_rdy);
    goto out;
    }
    mci.pwr_rdy = pwr_rdy;
    dev_dbg(mci.dev, "Received vbus interrupt, pwr_rdy = %d\n", pwr_rdy);
    if (!pwr_rdy) {
    mci.psy_usb_type = POWER_SUPPLY_USB_TYPE_UNKNOWN;
    power_supply_changed(mci.psy);
    }
    ret = regmap_update_bits(mci.regmap,
    MT6360_PMU_DEVICE_TYPE,
    MT6360_USBCHGEN_MASK,
    pwr_rdy ? MT6360_USBCHGEN_MASK : 0);
    if (ret < 0)
    goto out;
    mci.bc12_en = pwr_rdy;
    out:
    mutex_unlock(&mci.chgdet_lock);
    }
#[no_mangle]
unsafe extern "C" fn mt6360_chrdet_work(work: *mut work_struct) {
    static void mt6360_chrdet_work(struct work_struct *work)
    {
    struct mt6360_chg_info *mci = (struct mt6360_chg_info *)container_of(
    work, struct mt6360_chg_info, chrdet_work);
    mt6360_handle_chrdet_ext_evt(mci);
    }
#[no_mangle]
unsafe extern "C" fn mt6360_pmu_chrdet_ext_evt_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t mt6360_pmu_chrdet_ext_evt_handler(int irq, void *data)
    {
    struct mt6360_chg_info *mci = data;
    mt6360_handle_chrdet_ext_evt(mci);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn mt6360_chg_irq_register(pdev: *mut platform_device) -> c_int {
    static int mt6360_chg_irq_register(struct platform_device *pdev)
    {
    const struct {
    const char *name;
    irq_handler_t handler;
    } irq_descs[] = {
    { "attach_i", mt6360_pmu_attach_i_handler },
    { "chrdet_ext_evt", mt6360_pmu_chrdet_ext_evt_handler }
    };
    int i, ret;
    for (i = 0; i < ARRAY_SIZE(irq_descs); i++) {
    ret = platform_get_irq_byname(pdev, irq_descs[i].name);
    if (ret < 0)
    return ret;
    ret = devm_request_threaded_irq(&pdev.dev, ret, core::ptr::null_mut(),
    irq_descs[i].handler,
    IRQF_TRIGGER_FALLING | IRQF_ONESHOT,
    irq_descs[i].name,
    platform_get_drvdata(pdev));
    if (ret < 0)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt6360_vinovp_trans_to_sel(val: u32) -> u32 {
    static u32 mt6360_vinovp_trans_to_sel(u32 val)
    {
    u32 vinovp_tbl[] = { 5500000, 6500000, 11000000, 14500000 };
    int i;
// Select the smaller and equal supported value
    for (i = 0; i < ARRAY_SIZE(vinovp_tbl)-1; i++) {
    if (val < vinovp_tbl[i+1])
    break;
    }
    return i;
    }
#[no_mangle]
unsafe extern "C" fn mt6360_chg_init_setting(mci: *mut mt6360_chg_info) -> c_int {
    static int mt6360_chg_init_setting(struct mt6360_chg_info *mci)
    {
    int ret;
    u32 sel;
    sel = mt6360_vinovp_trans_to_sel(mci.vinovp);
    ret = regmap_update_bits(mci.regmap, MT6360_PMU_CHG_CTRL19,
    MT6360_VINOVP_MASK, sel << MT6360_VINOVP_SHFT);
    if (ret)
    return dev_err_probe(mci.dev, ret, "%s: Failed to apply vinovp\n", __func__);
    ret = regmap_update_bits(mci.regmap, MT6360_PMU_DEVICE_TYPE,
    MT6360_USBCHGEN_MASK, 0);
    if (ret)
    return dev_err_probe(mci.dev, ret, "%s: Failed to disable bc12\n", __func__);
    ret = regmap_update_bits(mci.regmap, MT6360_PMU_CHG_CTRL2,
    MT6360_IINLMTSEL_MASK,
    MT6360_IINLMTSEL_AICR <<
    MT6360_IINLMTSEL_SHFT);
    if (ret)
    return dev_err_probe(mci.dev, ret,
    "%s: Failed to switch iinlmtsel to aicr\n", __func__);
    usleep_range(5000, 6000);
    ret = regmap_update_bits(mci.regmap, MT6360_PMU_CHG_CTRL3,
    MT6360_ILIM_EN_MASK, 0);
    if (ret)
    return dev_err_probe(mci.dev, ret,
    "%s: Failed to disable ilim\n", __func__);
    ret = regmap_update_bits(mci.regmap, MT6360_PMU_CHG_CTRL10,
    MT6360_OTG_OC_MASK, MT6360_OTG_OC_MASK);
    if (ret)
    return dev_err_probe(mci.dev, ret,
    "%s: Failed to config otg oc to 3A\n", __func__);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt6360_charger_probe(pdev: *mut platform_device) -> c_int {
    static int mt6360_charger_probe(struct platform_device *pdev)
    {
    struct mt6360_chg_info *mci;
    let mut charger_cfg: power_supply_config = {};
    let mut config: regulator_config = { };
    int ret;
    mci = devm_kzalloc(&pdev.dev, sizeof(*mci), GFP_KERNEL);
    if (!mci)
    return -ENOMEM;
    mci.dev = &pdev.dev;
    mci.vinovp = 6500000;
    mutex_init(&mci.chgdet_lock);
    platform_set_drvdata(pdev, mci);
    ret = devm_work_autocancel(&pdev.dev, &mci.chrdet_work, mt6360_chrdet_work);
    if (ret)
    return dev_err_probe(&pdev.dev, ret, "Failed to set delayed work\n");
    ret = device_property_read_u32(&pdev.dev, "richtek,vinovp-microvolt", &mci.vinovp);
    if (ret)
    dev_warn(&pdev.dev, "Failed to parse vinovp in DT, keep default 6.5v\n");
    mci.regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!mci.regmap)
    return dev_err_probe(&pdev.dev, -ENODEV, "Failed to get parent regmap\n");
    ret = mt6360_chg_init_setting(mci);
    if (ret)
    return dev_err_probe(&pdev.dev, ret, "Failed to initial setting\n");
    memcpy(&mci.psy_desc, &mt6360_charger_desc, sizeof(mci.psy_desc));
    mci.psy_desc.name = dev_name(&pdev.dev);
    charger_cfg.drv_data = mci;
    charger_cfg.fwnode = dev_fwnode(&pdev.dev);
    mci.psy = devm_power_supply_register(&pdev.dev,
    &mci.psy_desc, &charger_cfg);
    if (IS_ERR(mci.psy))
    return dev_err_probe(&pdev.dev, PTR_ERR(mci.psy),
    "Failed to register power supply dev\n");
    ret = mt6360_chg_irq_register(pdev);
    if (ret)
    return dev_err_probe(&pdev.dev, ret, "Failed to register irqs\n");
    config.dev = &pdev.dev;
    config.regmap = mci.regmap;
    mci.otg_rdev = devm_regulator_register(&pdev.dev, &mt6360_otg_rdesc,
    &config);
    if (IS_ERR(mci.otg_rdev))
    return PTR_ERR(mci.otg_rdev);
    schedule_work(&mci.chrdet_work);
    return 0;
    }
    static const struct of_device_id __maybe_unused mt6360_charger_of_id[] = {
    { .compatible = "mediatek,mt6360-chg", },
    {},
    };
    MODULE_DEVICE_TABLE(of, mt6360_charger_of_id);
    static const struct platform_device_id mt6360_charger_id[] = {
    { .name = "mt6360-chg" },
    { }
    };
    MODULE_DEVICE_TABLE(platform, mt6360_charger_id);
    static struct platform_driver mt6360_charger_driver = {
    .driver = {
    .name = "mt6360-chg",
    .of_match_table = mt6360_charger_of_id,
    },
    .probe = mt6360_charger_probe,
    .id_table = mt6360_charger_id,
    };
    module_platform_driver(mt6360_charger_driver);
    MODULE_AUTHOR("Gene Chen <gene_chen@richtek.com>");
    MODULE_DESCRIPTION("MT6360 Charger Driver");
    MODULE_LICENSE("GPL");
