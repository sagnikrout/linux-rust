//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/axp288_fuel_gauge.c
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
// axp288_fuel_gauge.c - Xpower AXP288 PMIC Fuel Gauge Driver
//
// Copyright (C) 2020-2021 Andrejus Basovas <xxx@yyy.tld>
// Copyright (C) 2016-2021 Hans de Goede <hdegoede@redhat.com>
// Copyright (C) 2014 Intel Corporation
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//

pub const CHRG_CCCV_CC_MASK: c_uint = 0xf     /* 4 bits */;
pub const CHRG_CCCV_CC_BIT_POS: c_int = 0;

pub const CHRG_CCCV_CV_MASK: c_uint = 0x60        /* 2 bits */;
pub const CHRG_CCCV_CV_BIT_POS: c_int = 5;
pub const CHRG_CCCV_CV_4100MV: c_uint = 0x0     /* 4.10V */;
pub const CHRG_CCCV_CV_4150MV: c_uint = 0x1     /* 4.15V */;
pub const CHRG_CCCV_CV_4200MV: c_uint = 0x2     /* 4.20V */;
pub const CHRG_CCCV_CV_4350MV: c_uint = 0x3     /* 4.35V */;

pub const FG_15BIT_VAL_MASK: c_uint = 0x7fff;

pub const FG_REP_CAP_VAL_MASK: c_uint = 0x7F;

pub const FG_OCV_CAP_VAL_MASK: c_uint = 0x7F;

pub const FG_CC_CAP_VAL_MASK: c_uint = 0x7F;
pub const FG_LOW_CAP_THR1_MASK: c_uint = 0xf0    /* 5% tp 20% */;
pub const FG_LOW_CAP_THR1_VAL: c_uint = 0xa0    /* 15 perc */;
pub const FG_LOW_CAP_THR2_MASK: c_uint = 0x0f    /* 0% to 15% */;

// 1.1mV per LSB expressed in uV

// properties converted to uV, uA

pub const AXP288_FG_INTR_NUM: c_int = 6;

    static bool no_current_sense_res;
    module_param(no_current_sense_res, bool, 0444);
    MODULE_PARM_DESC(no_current_sense_res, "No (or broken) current sense resistor");
    enum {
    QWBTU_IRQ = 0,
    WBTU_IRQ,
    QWBTO_IRQ,
    WBTO_IRQ,
    WL2_IRQ,
    WL1_IRQ,
    };
    enum {
    BAT_CHRG_CURR,
    BAT_D_CURR,
    BAT_VOLT,
    IIO_CHANNEL_NUM
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct axp288_fg_info {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub irq: [c_int; AXP288_FG_INTR_NUM],
    pub iio_channel: [*mut iio_channel; IIO_CHANNEL_NUM],
    pub bat: *mut power_supply,
    pub lock: mutex,
    pub status: c_int,
    pub max_volt: c_int,
    pub pwr_op: c_int,
    pub low_cap: c_int,
    pub debug_file: *mut dentry,
    pub /: *mut *mut char valid; / zero until following fields are valid,
    pub /: *mut *mut unsigned long last_updated; / in jiffies,
    pub pwr_stat: c_int,
    pub fg_res: c_int,
    pub bat_volt: c_int,
    pub d_curr: c_int,
    pub c_curr: c_int,
    pub ocv: c_int,
    pub fg_cc_mtr1: c_int,
    pub fg_des_cap1: c_int,
}

    static enum power_supply_property fuel_gauge_props[] = {
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_PRESENT,
    POWER_SUPPLY_PROP_HEALTH,
    POWER_SUPPLY_PROP_VOLTAGE_MAX_DESIGN,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    POWER_SUPPLY_PROP_VOLTAGE_OCV,
    POWER_SUPPLY_PROP_CAPACITY,
    POWER_SUPPLY_PROP_CAPACITY_ALERT_MIN,
    POWER_SUPPLY_PROP_TECHNOLOGY,
// The 3 props below are not used when no_current_sense_res is set
    POWER_SUPPLY_PROP_CHARGE_FULL,
    POWER_SUPPLY_PROP_CHARGE_NOW,
    POWER_SUPPLY_PROP_CURRENT_NOW,
    };
#[no_mangle]
unsafe extern "C" fn fuel_gauge_reg_readb(info: *mut axp288_fg_info, reg: c_int) -> c_int {
    static int fuel_gauge_reg_readb(struct axp288_fg_info *info, int reg)
    {
    unsigned int val;
    int ret;
    ret = regmap_read(info.regmap, reg, &val);
    if (ret < 0) {
    dev_err(info.dev, "Error reading reg 0x%02x err: %d\n", reg, ret);
    return ret;
    }
    return val;
    }
#[no_mangle]
unsafe extern "C" fn fuel_gauge_reg_writeb(info: *mut axp288_fg_info, reg: c_int, val: u8) -> c_int {
    static int fuel_gauge_reg_writeb(struct axp288_fg_info *info, int reg, u8 val)
    {
    int ret;
    ret = regmap_write(info.regmap, reg, (unsigned int)val);
    if (ret < 0)
    dev_err(info.dev, "Error writing reg 0x%02x err: %d\n", reg, ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn fuel_gauge_read_15bit_word(info: *mut axp288_fg_info, reg: c_int) -> c_int {
    static int fuel_gauge_read_15bit_word(struct axp288_fg_info *info, int reg)
    {
    unsigned char buf[2];
    int ret;
    ret = regmap_bulk_read(info.regmap, reg, buf, 2);
    if (ret < 0) {
    dev_err(info.dev, "Error reading reg 0x%02x err: %d\n", reg, ret);
    return ret;
    }
    ret = get_unaligned_be16(buf);
    if (!(ret & FG_15BIT_WORD_VALID)) {
    dev_err(info.dev, "Error reg 0x%02x contents not valid\n", reg);
    return -ENXIO;
    }
    return ret & FG_15BIT_VAL_MASK;
    }
#[no_mangle]
unsafe extern "C" fn fuel_gauge_read_12bit_word(info: *mut axp288_fg_info, reg: c_int) -> c_int {
    static int fuel_gauge_read_12bit_word(struct axp288_fg_info *info, int reg)
    {
    unsigned char buf[2];
    int ret;
    ret = regmap_bulk_read(info.regmap, reg, buf, 2);
    if (ret < 0) {
    dev_err(info.dev, "Error reading reg 0x%02x err: %d\n", reg, ret);
    return ret;
    }
// 12-bit data values have upper 8 bits in buf[0], lower 4 in buf[1]
    return (buf[0] << 4) | ((buf[1] >> 4) & 0x0f);
    }
#[no_mangle]
unsafe extern "C" fn fuel_gauge_update_registers(info: *mut axp288_fg_info) -> c_int {
    static int fuel_gauge_update_registers(struct axp288_fg_info *info)
    {
    int ret;
    if (info.valid && time_before(jiffies, info.last_updated + AXP288_REG_UPDATE_INTERVAL))
    return 0;
    dev_dbg(info.dev, "Fuel Gauge updating register values...\n");
    ret = iosf_mbi_block_punit_i2c_access();
    if (ret < 0)
    return ret;
    ret = fuel_gauge_reg_readb(info, AXP20X_PWR_INPUT_STATUS);
    if (ret < 0)
    goto out;
    info.pwr_stat = ret;
    if (no_current_sense_res)
    ret = fuel_gauge_reg_readb(info, AXP288_FG_OCV_CAP_REG);
    else
    ret = fuel_gauge_reg_readb(info, AXP20X_FG_RES);
    if (ret < 0)
    goto out;
    info.fg_res = ret;
    ret = iio_read_channel_raw(info.iio_channel[BAT_VOLT], &info.bat_volt);
    if (ret < 0)
    goto out;
    ret = fuel_gauge_read_12bit_word(info, AXP288_FG_OCVH_REG);
    if (ret < 0)
    goto out;
    info.ocv = ret;
    if (no_current_sense_res)
    goto out_no_current_sense_res;
    if (info.pwr_stat & PS_STAT_BAT_CHRG_DIR) {
    info.d_curr = 0;
    ret = iio_read_channel_raw(info.iio_channel[BAT_CHRG_CURR], &info.c_curr);
    if (ret < 0)
    goto out;
    } else {
    info.c_curr = 0;
    ret = iio_read_channel_raw(info.iio_channel[BAT_D_CURR], &info.d_curr);
    if (ret < 0)
    goto out;
    }
    ret = fuel_gauge_read_15bit_word(info, AXP288_FG_CC_MTR1_REG);
    if (ret < 0)
    goto out;
    info.fg_cc_mtr1 = ret;
    ret = fuel_gauge_read_15bit_word(info, AXP288_FG_DES_CAP1_REG);
    if (ret < 0)
    goto out;
    info.fg_des_cap1 = ret;
    out_no_current_sense_res:
    info.last_updated = jiffies;
    info.valid = 1;
    ret = 0;
    out:
    iosf_mbi_unblock_punit_i2c_access();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn fuel_gauge_get_status(info: *mut axp288_fg_info) {
    static void fuel_gauge_get_status(struct axp288_fg_info *info)
    {
    let mut pwr_stat: c_int = info.pwr_stat;
    let mut fg_res: c_int = info.fg_res;
    let mut curr: c_int = info.d_curr;
// Report full if Vbus is valid and the reported capacity is 100%
    if (!(pwr_stat & PS_STAT_VBUS_VALID))
    goto not_full;
    if (!(fg_res & FG_REP_CAP_VALID))
    goto not_full;
    fg_res &= ~FG_REP_CAP_VALID;
    if (fg_res == 100) {
    info.status = POWER_SUPPLY_STATUS_FULL;
    return;
    }
//
// Sometimes the charger turns itself off before fg-res reaches 100%.
// When this happens the AXP288 reports a not-charging status and
// 0 mA discharge current.
//
    if (fg_res < 90 || (pwr_stat & PS_STAT_BAT_CHRG_DIR) || no_current_sense_res)
    goto not_full;
    if (curr == 0) {
    info.status = POWER_SUPPLY_STATUS_FULL;
    return;
    }
    not_full:
    if (pwr_stat & PS_STAT_BAT_CHRG_DIR)
    info.status = POWER_SUPPLY_STATUS_CHARGING;
    else
    info.status = POWER_SUPPLY_STATUS_DISCHARGING;
    }
#[no_mangle]
unsafe extern "C" fn fuel_gauge_battery_health(info: *mut axp288_fg_info) -> c_int {
    static int fuel_gauge_battery_health(struct axp288_fg_info *info)
    {
    let mut vocv: c_int = VOLTAGE_FROM_ADC(info.ocv);
    let mut health: c_int = POWER_SUPPLY_HEALTH_UNKNOWN;
    if (vocv > info.max_volt)
    health = POWER_SUPPLY_HEALTH_OVERVOLTAGE;
    else
    health = POWER_SUPPLY_HEALTH_GOOD;
    return health;
    }
    static int fuel_gauge_get_property(struct power_supply *ps,
    enum power_supply_property prop,
    union power_supply_propval *val)
    {
    struct axp288_fg_info *info = power_supply_get_drvdata(ps);
    int ret, value;
    mutex_lock(&info.lock);
    ret = fuel_gauge_update_registers(info);
    if (ret < 0)
    goto out;
    switch (prop) {
    case POWER_SUPPLY_PROP_STATUS:
    fuel_gauge_get_status(info);
    val.intval = info.status;
    break;
    case POWER_SUPPLY_PROP_HEALTH:
    val.intval = fuel_gauge_battery_health(info);
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_NOW:
    value = VOLTAGE_FROM_ADC(info.bat_volt);
    val.intval = PROP_VOLT(value);
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_OCV:
    value = VOLTAGE_FROM_ADC(info.ocv);
    val.intval = PROP_VOLT(value);
    break;
    case POWER_SUPPLY_PROP_CURRENT_NOW:
    if (info.d_curr > 0)
    value = -1 * info.d_curr;
    else
    value = info.c_curr;
    val.intval = PROP_CURR(value);
    break;
    case POWER_SUPPLY_PROP_PRESENT:
    if (info.pwr_op & CHRG_STAT_BAT_PRESENT)
    val.intval = 1;
    else
    val.intval = 0;
    break;
    case POWER_SUPPLY_PROP_CAPACITY:
    if (!(info.fg_res & FG_REP_CAP_VALID))
    dev_err(info.dev, "capacity measurement not valid\n");
    val.intval = (info.fg_res & FG_REP_CAP_VAL_MASK);
    break;
    case POWER_SUPPLY_PROP_CAPACITY_ALERT_MIN:
    val.intval = (info.low_cap & 0x0f);
    break;
    case POWER_SUPPLY_PROP_TECHNOLOGY:
    val.intval = POWER_SUPPLY_TECHNOLOGY_LION;
    break;
    case POWER_SUPPLY_PROP_CHARGE_NOW:
    val.intval = info.fg_cc_mtr1 * FG_DES_CAP_RES_LSB;
    break;
    case POWER_SUPPLY_PROP_CHARGE_FULL:
    val.intval = info.fg_des_cap1 * FG_DES_CAP_RES_LSB;
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_MAX_DESIGN:
    val.intval = PROP_VOLT(info.max_volt);
    break;
    default:
    ret = -EINVAL;
    }
    out:
    mutex_unlock(&info.lock);
    return ret;
    }
    static int fuel_gauge_set_property(struct power_supply *ps,
    enum power_supply_property prop,
    const union power_supply_propval *val)
    {
    struct axp288_fg_info *info = power_supply_get_drvdata(ps);
    int new_low_cap, ret = 0;
    mutex_lock(&info.lock);
    switch (prop) {
    case POWER_SUPPLY_PROP_CAPACITY_ALERT_MIN:
    if ((val.intval < 0) || (val.intval > 15)) {
    ret = -EINVAL;
    break;
    }
    new_low_cap = info.low_cap;
    new_low_cap &= 0xf0;
    new_low_cap |= (val.intval & 0xf);
    ret = fuel_gauge_reg_writeb(info, AXP288_FG_LOW_CAP_REG, new_low_cap);
    if (ret == 0)
    info.low_cap = new_low_cap;
    break;
    default:
    ret = -EINVAL;
    break;
    }
    mutex_unlock(&info.lock);
    return ret;
    }
    static int fuel_gauge_property_is_writeable(struct power_supply *psy,
    enum power_supply_property psp)
    {
    int ret;
    switch (psp) {
    case POWER_SUPPLY_PROP_CAPACITY_ALERT_MIN:
    ret = 1;
    break;
    default:
    ret = 0;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn fuel_gauge_thread_handler(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t fuel_gauge_thread_handler(int irq, void *dev)
    {
    struct axp288_fg_info *info = dev;
    int i;
    for (i = 0; i < AXP288_FG_INTR_NUM; i++) {
    if (info.irq[i] == irq)
    break;
    }
    if (i >= AXP288_FG_INTR_NUM) {
    dev_warn(info.dev, "spurious interrupt!!\n");
    return IRQ_NONE;
    }
    switch (i) {
    case QWBTU_IRQ:
    dev_info(info.dev, "Quit Battery under temperature in work mode IRQ (QWBTU)\n");
    break;
    case WBTU_IRQ:
    dev_info(info.dev, "Battery under temperature in work mode IRQ (WBTU)\n");
    break;
    case QWBTO_IRQ:
    dev_info(info.dev, "Quit Battery over temperature in work mode IRQ (QWBTO)\n");
    break;
    case WBTO_IRQ:
    dev_info(info.dev, "Battery over temperature in work mode IRQ (WBTO)\n");
    break;
    case WL2_IRQ:
    dev_info(info.dev, "Low Batt Warning(2) INTR\n");
    break;
    case WL1_IRQ:
    dev_info(info.dev, "Low Batt Warning(1) INTR\n");
    break;
    default:
    dev_warn(info.dev, "Spurious Interrupt!!!\n");
    }
    mutex_lock(&info.lock);
    info.valid = 0; /* Force updating of the cached registers */
    mutex_unlock(&info.lock);
    power_supply_changed(info.bat);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn fuel_gauge_external_power_changed(psy: *mut power_supply) {
    static void fuel_gauge_external_power_changed(struct power_supply *psy)
    {
    struct axp288_fg_info *info = power_supply_get_drvdata(psy);
    mutex_lock(&info.lock);
    info.valid = 0; /* Force updating of the cached registers */
    mutex_unlock(&info.lock);
    power_supply_changed(psy);
    }
    static struct power_supply_desc fuel_gauge_desc = {
    .name			= DEV_NAME,
    .type			= POWER_SUPPLY_TYPE_BATTERY,
    .properties		= fuel_gauge_props,
    .num_properties		= ARRAY_SIZE(fuel_gauge_props),
    .get_property		= fuel_gauge_get_property,
    .set_property		= fuel_gauge_set_property,
    .property_is_writeable	= fuel_gauge_property_is_writeable,
    .external_power_changed	= fuel_gauge_external_power_changed,
    };
//
// Some devices have no battery (HDMI sticks) and the axp288 battery's
// detection reports one despite it not being there.
// Please keep this listed sorted alphabetically.
//
    static const struct dmi_system_id axp288_quirks[] = {
    {
// ACEPC T8 Cherry Trail Z8350 mini PC
    .matches = {
    DMI_EXACT_MATCH(DMI_BOARD_VENDOR, "To be filled by O.E.M."),
    DMI_EXACT_MATCH(DMI_BOARD_NAME, "Cherry Trail CR"),
    DMI_EXACT_MATCH(DMI_PRODUCT_SKU, "T8"),
// also match on somewhat unique bios-version
    DMI_EXACT_MATCH(DMI_BIOS_VERSION, "1.000"),
    },
    .driver_data = (void *)AXP288_QUIRK_NO_BATTERY,
    },
    {
// ACEPC T11 Cherry Trail Z8350 mini PC
    .matches = {
    DMI_EXACT_MATCH(DMI_BOARD_VENDOR, "To be filled by O.E.M."),
    DMI_EXACT_MATCH(DMI_BOARD_NAME, "Cherry Trail CR"),
    DMI_EXACT_MATCH(DMI_PRODUCT_SKU, "T11"),
// also match on somewhat unique bios-version
    DMI_EXACT_MATCH(DMI_BIOS_VERSION, "1.000"),
    },
    .driver_data = (void *)AXP288_QUIRK_NO_BATTERY,
    },
    {
// Intel Bay Trail Compute Stick
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Intel"),
// Partial match for STCK1A32WFC STCK1A32FC, STCK1A8LFC variants
    DMI_MATCH(DMI_PRODUCT_NAME, "STCK1A"),
    },
    .driver_data = (void *)AXP288_QUIRK_NO_BATTERY,
    },
    {
// Intel Cherry Trail Compute Stick
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Intel"),
// Partial match for STK1AW32SC and STK1A32SC variants
    DMI_MATCH(DMI_PRODUCT_NAME, "STK1A"),
    },
    .driver_data = (void *)AXP288_QUIRK_NO_BATTERY,
    },
    {
// Meegopad T02
    .matches = {
    DMI_MATCH(DMI_PRODUCT_NAME, "MEEGOPAD T02"),
    },
    .driver_data = (void *)AXP288_QUIRK_NO_BATTERY,
    },
    {	/* Mele PCG03 Mini PC */
    .matches = {
    DMI_EXACT_MATCH(DMI_BOARD_VENDOR, "Mini PC"),
    DMI_EXACT_MATCH(DMI_BOARD_NAME, "Mini PC"),
    },
    .driver_data = (void *)AXP288_QUIRK_NO_BATTERY,
    },
    {
// Minix Neo Z83-4 mini PC
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "MINIX"),
    DMI_MATCH(DMI_PRODUCT_NAME, "Z83-4"),
    },
    .driver_data = (void *)AXP288_QUIRK_NO_BATTERY,
    },
    {
//
// One Mix 1, this uses the "T3 MRD" boardname used by
// generic mini PCs, but it is a mini laptop so it does
// actually have a battery!
//
    .matches = {
    DMI_MATCH(DMI_BOARD_NAME, "T3 MRD"),
    DMI_MATCH(DMI_BIOS_DATE, "06/14/2018"),
    },
    .driver_data = core::ptr::null_mut(),
    },
    {
// Radxa ROCK Pi X Single Board Computer
    .matches = {
    DMI_MATCH(DMI_BOARD_NAME, "ROCK Pi X"),
    DMI_MATCH(DMI_BOARD_VENDOR, "Radxa"),
    },
    .driver_data = (void *)AXP288_QUIRK_NO_BATTERY,
    },
    {
//
// Various Ace PC/Meegopad/MinisForum/Wintel Mini-PCs/HDMI-sticks
// This entry must be last because it is generic, this allows
// adding more specifuc quirks overriding this generic entry.
//
    .matches = {
    DMI_MATCH(DMI_BOARD_NAME, "T3 MRD"),
    DMI_MATCH(DMI_CHASSIS_TYPE, "3"),
    DMI_MATCH(DMI_BIOS_VENDOR, "American Megatrends Inc."),
    },
    .driver_data = (void *)AXP288_QUIRK_NO_BATTERY,
    },
    {}
    };
#[no_mangle]
unsafe extern "C" fn axp288_fuel_gauge_read_initial_regs(info: *mut axp288_fg_info) -> c_int {
    static int axp288_fuel_gauge_read_initial_regs(struct axp288_fg_info *info)
    {
    unsigned int val;
    int ret;
//
// On some devices the fuelgauge and charger parts of the axp288 are
// not used, check that the fuelgauge is enabled (CC_CTRL != 0).
//
    ret = regmap_read(info.regmap, AXP20X_CC_CTRL, &val);
    if (ret < 0)
    return ret;
    if (val == 0)
    return -ENODEV;
    ret = fuel_gauge_reg_readb(info, AXP288_FG_DES_CAP1_REG);
    if (ret < 0)
    return ret;
    if (!(ret & FG_DES_CAP1_VALID)) {
    dev_err(info.dev, "axp288 not configured by firmware\n");
    return -ENODEV;
    }
    ret = fuel_gauge_reg_readb(info, AXP20X_CHRG_CTRL1);
    if (ret < 0)
    return ret;
    switch ((ret & CHRG_CCCV_CV_MASK) >> CHRG_CCCV_CV_BIT_POS) {
    case CHRG_CCCV_CV_4100MV:
    info.max_volt = 4100;
    break;
    case CHRG_CCCV_CV_4150MV:
    info.max_volt = 4150;
    break;
    case CHRG_CCCV_CV_4200MV:
    info.max_volt = 4200;
    break;
    case CHRG_CCCV_CV_4350MV:
    info.max_volt = 4350;
    break;
    }
    ret = fuel_gauge_reg_readb(info, AXP20X_PWR_OP_MODE);
    if (ret < 0)
    return ret;
    info.pwr_op = ret;
    ret = fuel_gauge_reg_readb(info, AXP288_FG_LOW_CAP_REG);
    if (ret < 0)
    return ret;
    info.low_cap = ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn axp288_fuel_gauge_release_iio_chans(data: *mut c_void) {
    static void axp288_fuel_gauge_release_iio_chans(void *data)
    {
    struct axp288_fg_info *info = data;
    int i;
    for (i = 0; i < IIO_CHANNEL_NUM; i++)
    if (!IS_ERR_OR_NULL(info.iio_channel[i]))
    iio_channel_release(info.iio_channel[i]);
    }
#[no_mangle]
unsafe extern "C" fn axp288_fuel_gauge_probe(pdev: *mut platform_device) -> c_int {
    static int axp288_fuel_gauge_probe(struct platform_device *pdev)
    {
    struct axp288_fg_info *info;
    struct axp20x_dev *axp20x = dev_get_drvdata(pdev.dev.parent);
    let mut psy_cfg: power_supply_config = {};
    static const char * const iio_chan_name[] = {
    [BAT_CHRG_CURR] = "axp288-chrg-curr",
    [BAT_D_CURR] = "axp288-chrg-d-curr",
    [BAT_VOLT] = "axp288-batt-volt",
    };
    const struct dmi_system_id *dmi_id;
    struct device *dev = &pdev.dev;
    let mut quirks: c_ulong = 0;
    int i, pirq, ret;
//
// Normally the native AXP288 fg/charger drivers are preferred but
// on some devices the ACPI drivers should be used instead.
//
    if (!acpi_quirk_skip_acpi_ac_and_battery())
    return -ENODEV;
    dmi_id = dmi_first_match(axp288_quirks);
    if (dmi_id)
    quirks = (unsigned long)dmi_id.driver_data;
    if (quirks & AXP288_QUIRK_NO_BATTERY)
    return -ENODEV;
    info = devm_kzalloc(dev, sizeof(*info), GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    info.dev = dev;
    info.regmap = axp20x.regmap;
    info.status = POWER_SUPPLY_STATUS_UNKNOWN;
    info.valid = 0;
    platform_set_drvdata(pdev, info);
    mutex_init(&info.lock);
    for (i = 0; i < AXP288_FG_INTR_NUM; i++) {
    pirq = platform_get_irq(pdev, i);
    if (pirq < 0)
    continue;
    ret = regmap_irq_get_virq(axp20x.regmap_irqc, pirq);
    if (ret < 0)
    return dev_err_probe(dev, ret, "getting vIRQ %d\n", pirq);
    info.irq[i] = ret;
    }
    for (i = 0; i < IIO_CHANNEL_NUM; i++) {
//
// Note cannot use devm_iio_channel_get because x86 systems
// lack the device<->channel maps which iio_channel_get will
// try to use when passed a non NULL device pointer.
//
    info.iio_channel[i] =
    iio_channel_get(core::ptr::null_mut(), iio_chan_name[i]);
    if (IS_ERR(info.iio_channel[i])) {
    ret = PTR_ERR(info.iio_channel[i]);
    dev_dbg(dev, "error getting iiochan %s: %d\n", iio_chan_name[i], ret);
// Wait for axp288_adc to load
    if (ret == -ENODEV)
    ret = -EPROBE_DEFER;
    axp288_fuel_gauge_release_iio_chans(info);
    return ret;
    }
    }
    ret = devm_add_action_or_reset(dev, axp288_fuel_gauge_release_iio_chans, info);
    if (ret)
    return ret;
    ret = iosf_mbi_block_punit_i2c_access();
    if (ret < 0)
    return ret;
    ret = axp288_fuel_gauge_read_initial_regs(info);
    iosf_mbi_unblock_punit_i2c_access();
    if (ret < 0)
    return ret;
    psy_cfg.drv_data = info;
    if (no_current_sense_res)
    fuel_gauge_desc.num_properties = ARRAY_SIZE(fuel_gauge_props) - 3;
    info.bat = devm_power_supply_register(dev, &fuel_gauge_desc, &psy_cfg);
    if (IS_ERR(info.bat)) {
    ret = PTR_ERR(info.bat);
    dev_err(dev, "failed to register battery: %d\n", ret);
    return ret;
    }
    for (i = 0; i < AXP288_FG_INTR_NUM; i++) {
    ret = devm_request_threaded_irq(dev, info.irq[i], core::ptr::null_mut(),
    fuel_gauge_thread_handler,
    IRQF_ONESHOT, DEV_NAME, info);
    if (ret)
    return ret;
    }
    return 0;
    }
    static const struct platform_device_id axp288_fg_id_table[] = {
    { .name = DEV_NAME },
    { }
    };
    MODULE_DEVICE_TABLE(platform, axp288_fg_id_table);
    static struct platform_driver axp288_fuel_gauge_driver = {
    .probe = axp288_fuel_gauge_probe,
    .id_table = axp288_fg_id_table,
    .driver = {
    .name = DEV_NAME,
    },
    };
    module_platform_driver(axp288_fuel_gauge_driver);
    MODULE_AUTHOR("Ramakrishna Pallala <ramakrishna.pallala@intel.com>");
    MODULE_AUTHOR("Todd Brandt <todd.e.brandt@linux.intel.com>");
    MODULE_DESCRIPTION("Xpower AXP288 Fuel Gauge Driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_CONSUMER");
