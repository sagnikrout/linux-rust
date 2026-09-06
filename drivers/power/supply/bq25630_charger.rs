//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/bq25630_charger.c
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
// Driver for TI BQ25630 charger.
//
// Copyright (C) 2026 Axis Communications AB
//

// Registers.
pub const BQ25630_REG_CHARGE_CURRENT_LIMIT: c_uint = 0x02;
pub const BQ25630_REG_CHARGE_VOLTAGE_LIMIT: c_uint = 0x04;
pub const BQ25630_REG_INPUT_CURRENT_LIMIT: c_uint = 0x06;
pub const BQ25630_REG_INPUT_VOLTAGE_LIMIT: c_uint = 0x08;
pub const BQ25630_REG_IOTG_REGULATION: c_uint = 0x0a;
pub const BQ25630_REG_VOTG_REGULATION: c_uint = 0x0c;
pub const BQ25630_REG_MINIMAL_SYSTEM_VOLTAGE: c_uint = 0x0e;
pub const BQ25630_REG_PRECHARGE_CONTROL: c_uint = 0x10;
pub const BQ25630_REG_TERMINATION_CONTROL: c_uint = 0x12;
pub const BQ25630_REG_CHARGE_TIMER_CONTROL: c_uint = 0x14;
pub const BQ25630_REG_CHARGER_CONTROL_0: c_uint = 0x15;
pub const BQ25630_REG_CHARGER_CONTROL_1: c_uint = 0x16;
pub const BQ25630_REG_CHARGER_CONTROL_2: c_uint = 0x17;
pub const BQ25630_REG_CHARGER_CONTROL_3: c_uint = 0x18;
pub const BQ25630_REG_CHARGER_CONTROL_4: c_uint = 0x19;
pub const BQ25630_REG_CHARGER_CONTROL_5: c_uint = 0x1a;
pub const BQ25630_REG_NTC_CONTROL_0: c_uint = 0x1b;
pub const BQ25630_REG_NTC_CONTROL_1: c_uint = 0x1c;
pub const BQ25630_REG_NTC_CONTROL_2: c_uint = 0x1d;
pub const BQ25630_REG_NTC_CONTROL_3: c_uint = 0x1e;
pub const BQ25630_REG_CHARGER_STATUS_0: c_uint = 0x1f;
pub const BQ25630_REG_CHARGER_STATUS_1: c_uint = 0x20;
pub const BQ25630_REG_CHARGER_STATUS_2: c_uint = 0x21;
pub const BQ25630_REG_FAULT_STATUS: c_uint = 0x22;
pub const BQ25630_REG_CHARGER_FLAG_0: c_uint = 0x23;
pub const BQ25630_REG_CHARGER_FLAG_1: c_uint = 0x24;
pub const BQ25630_REG_FAULT_FLAG: c_uint = 0x25;
pub const BQ25630_REG_CHARGER_MASK_0: c_uint = 0x26;
pub const BQ25630_REG_CHARGER_MASK_1: c_uint = 0x27;
pub const BQ25630_REG_FAULT_MASK: c_uint = 0x28;
pub const BQ25630_REG_ICO_CURRENT_LIMIT: c_uint = 0x29;
pub const BQ25630_REG_ADC_CONTROL: c_uint = 0x2b;
pub const BQ25630_REG_ADC_CHANNEL_DISABLE_1: c_uint = 0x2c;
pub const BQ25630_REG_IBUS_ADC: c_uint = 0x32;
pub const BQ25630_REG_IBAT_ADC: c_uint = 0x34;
pub const BQ25630_REG_VBUS_ADC: c_uint = 0x36;
pub const BQ25630_REG_VPMID_ADC: c_uint = 0x38;
pub const BQ25630_REG_VBAT_ADC: c_uint = 0x3a;
pub const BQ25630_REG_VSYS_ADC: c_uint = 0x3c;
pub const BQ25630_REG_TS_ADC: c_uint = 0x3e;
pub const BQ25630_REG_TDIE_ADC: c_uint = 0x40;
pub const BQ25630_REG_USB_C_CONTROL_0: c_uint = 0x44;
pub const BQ25630_REG_USB_C_CONTROL_1: c_uint = 0x45;
pub const BQ25630_REG_LIQUID_CONTROL_0: c_uint = 0x46;
pub const BQ25630_REG_LIQUID_CONTROL_1: c_uint = 0x47;
pub const BQ25630_REG_USB_C_INFORMATION_0: c_uint = 0x48;
pub const BQ25630_REG_USB_C_INFORMATION_1: c_uint = 0x49;
pub const BQ25630_REG_USB_DAC_CONTROL_0: c_uint = 0x4a;
pub const BQ25630_REG_USB_DAC_CONTROL_1: c_uint = 0x4b;
pub const BQ25630_REG_PART_INFORMATION: c_uint = 0x4d;

    (BQ25630_REG_FAULT_FLAG - BQ25630_REG_CHARGER_STATUS_0 + 1)
// Charge current limits.
pub const BQ25630_ICHG_MIN_REGVAL: c_uint = 0x04;
pub const BQ25630_ICHG_MIN: c_int = 80000;
pub const BQ25630_ICHG_MAX: c_int = 5040000;
pub const BQ25630_ICHG_STEP: c_int = 20000;
// Charge voltage limits.
pub const BQ25630_VREG_MIN_REGVAL: c_uint = 0x15e;
pub const BQ25630_VREG_MIN: c_int = 3500000;
pub const BQ25630_VREG_MAX: c_int = 4800000;
pub const BQ25630_VREG_STEP: c_int = 10000;
// Input current limits.
pub const BQ25630_IINDPM_MIN_REGVAL: c_uint = 0x0a;
pub const BQ25630_IINDPM_MIN: c_int = 100000;
pub const BQ25630_IINDPM_MAX: c_int = 3200000;
pub const BQ25630_IINDPM_STEP: c_int = 10000;
// Input voltage limits.
pub const BQ25630_VINDPM_MIN_REGVAL: c_uint = 0x5f;
pub const BQ25630_VINDPM_MIN: c_int = 3800000;
pub const BQ25630_VINDPM_MAX: c_int = 16800000;
pub const BQ25630_VINDPM_STEP: c_int = 40000;
// Minimal system voltage limits.
pub const BQ25630_VSYSMIN_MIN_REGVAL: c_uint = 0x20;
pub const BQ25630_VSYSMIN_MIN: c_int = 2560000;
pub const BQ25630_VSYSMIN_MAX: c_int = 4000000;
pub const BQ25630_VSYSMIN_STEP: c_int = 80000;
// Pre-charge current limits.
pub const BQ25630_IPRECHG_MIN_REGVAL: c_uint = 0x02;
pub const BQ25630_IPRECHG_MIN: c_int = 40000;
pub const BQ25630_IPRECHG_MAX: c_int = 1000000;
pub const BQ25630_IPRECHG_STEP: c_int = 20000;
// Termination current limits.
pub const BQ25630_ITERM_MIN_REGVAL: c_uint = 0x03;
pub const BQ25630_ITERM_MIN: c_int = 30000;
pub const BQ25630_ITERM_MAX: c_int = 1000000;
pub const BQ25630_ITERM_STEP: c_int = 10000;
// Charge types.
pub const BQ25630_CHG_STAT_NOT_CHARGING: c_uint = 0x00;
pub const BQ25630_CHG_STAT_TRICKLE_CHARGE: c_uint = 0x01;
pub const BQ25630_CHG_STAT_PRE_CHARGE: c_uint = 0x02;
pub const BQ25630_CHG_STAT_FAST_CHARGE: c_uint = 0x03;
pub const BQ25630_CHG_STAT_TAPER_CHARGE: c_uint = 0x04;
pub const BQ25630_CHG_STAT_TERMINATION: c_uint = 0x07;
// USB types (VBUS).
pub const BQ25630_VBUS_STAT_NONE: c_uint = 0x00;
pub const BQ25630_VBUS_STAT_SDP: c_uint = 0x01;
pub const BQ25630_VBUS_STAT_CDP: c_uint = 0x02;
pub const BQ25630_VBUS_STAT_DCP: c_uint = 0x03;
pub const BQ25630_VBUS_STAT_HVDCP: c_uint = 0x06;
pub const BQ25630_VBUS_STAT_BOOST_OTG: c_uint = 0x07;
pub const BQ25630_VBUS_STAT_USB_C_DEFAULT: c_uint = 0x08;
pub const BQ25630_VBUS_STAT_USB_C_MEDIUM: c_uint = 0x09;
pub const BQ25630_VBUS_STAT_USB_C_HIGH: c_uint = 0x0a;
// Temperature status.
pub const BQ25630_TS_STAT_NORMAL: c_uint = 0x00;
pub const BQ25630_TS_STAT_COLD: c_uint = 0x01;
pub const BQ25630_TS_STAT_HOT: c_uint = 0x02;
pub const BQ25630_TS_STAT_COOL: c_uint = 0x03;
pub const BQ25630_TS_STAT_WARM: c_uint = 0x04;
pub const BQ25630_TS_STAT_PRECOOL: c_uint = 0x05;
pub const BQ25630_TS_STAT_PREWARM: c_uint = 0x06;
// Register fields.
    enum bq25630_regfield {
// Charge current limit.
    BQ25630_REGF_ICHG,
// Charge voltage limit.
    BQ25630_REGF_VREG,
// Input current limit.
    BQ25630_REGF_IINDPM,
// Input voltage limit.
    BQ25630_REGF_VINDPM,
// Minimal system voltage.
    BQ25630_REGF_VSYSMIN,
// Pre-charge current limit.
    BQ25630_REGF_IPRECHG,
// Termination current threshold.
    BQ25630_REGF_ITERM,
// IBUS ADC reading.
    BQ25630_REGF_IBUS_ADC,
// VBUS ADC reading.
    BQ25630_REGF_VBUS_ADC,
// Watchdog timer.
    BQ25630_REGF_WATCHDOG,
// Enable charger.
    BQ25630_REGF_EN_CHG,
// Register reset.
    BQ25630_REGF_REG_RST,
// BATFET control.
    BQ25630_REGF_BATFET_CTRL,
// Power good indicator.
    BQ25630_REGF_PG_STAT,
// Charge status.
    BQ25630_REGF_CHG_STAT,
// VBUS status.
    BQ25630_REGF_VBUS_STAT,
// Temperature zone.
    BQ25630_REGF_TS_STAT,
// Temperature shutdwon.
    BQ25630_REGF_TSHUT_STAT,
// OTG fault.
    BQ25630_REGF_OTG_FAULT_STAT,
// System voltage fault.
    BQ25630_REGF_VSYS_FAULT_STAT,
// Battery fault.
    BQ25630_REGF_BAT_FAULT_STAT,
// VBUS fault.
    BQ25630_REGF_VBUS_FAULT_STAT,
// Sentinel value.
    BQ25630_REGF_MAX
    };
    static const struct reg_field bq25630_regfields[] = {
    [BQ25630_REGF_ICHG] =
    REG_FIELD(BQ25630_REG_CHARGE_CURRENT_LIMIT, 4, 11),
    [BQ25630_REGF_VREG] =
    REG_FIELD(BQ25630_REG_CHARGE_VOLTAGE_LIMIT, 3, 11),
    [BQ25630_REGF_IINDPM] =
    REG_FIELD(BQ25630_REG_INPUT_CURRENT_LIMIT, 3, 11),
    [BQ25630_REGF_VINDPM] =
    REG_FIELD(BQ25630_REG_INPUT_VOLTAGE_LIMIT, 5, 13),
    [BQ25630_REGF_VSYSMIN] =
    REG_FIELD(BQ25630_REG_MINIMAL_SYSTEM_VOLTAGE, 6, 11),
    [BQ25630_REGF_IPRECHG] =
    REG_FIELD(BQ25630_REG_PRECHARGE_CONTROL, 4, 9),
    [BQ25630_REGF_ITERM] =
    REG_FIELD(BQ25630_REG_TERMINATION_CONTROL, 3, 9),
    [BQ25630_REGF_IBUS_ADC] = REG_FIELD(BQ25630_REG_IBUS_ADC, 1, 15),
    [BQ25630_REGF_VBUS_ADC] = REG_FIELD(BQ25630_REG_VBUS_ADC, 2, 14),
    [BQ25630_REGF_WATCHDOG] =
    REG_FIELD(BQ25630_REG_CHARGER_CONTROL_1, 0, 1),
    [BQ25630_REGF_EN_CHG] =
    REG_FIELD(BQ25630_REG_CHARGER_CONTROL_1, 5, 5),
    [BQ25630_REGF_REG_RST] =
    REG_FIELD(BQ25630_REG_CHARGER_CONTROL_2, 7, 7),
    [BQ25630_REGF_BATFET_CTRL] =
    REG_FIELD(BQ25630_REG_CHARGER_CONTROL_3, 0, 1),
    [BQ25630_REGF_PG_STAT] =
    REG_FIELD(BQ25630_REG_CHARGER_STATUS_0, 7, 7),
    [BQ25630_REGF_CHG_STAT] =
    REG_FIELD(BQ25630_REG_CHARGER_STATUS_1, 3, 5),
    [BQ25630_REGF_VBUS_STAT] =
    REG_FIELD(BQ25630_REG_CHARGER_STATUS_2, 4, 7),
    [BQ25630_REGF_TS_STAT] =
    REG_FIELD(BQ25630_REG_FAULT_STATUS, 0, 2),
    [BQ25630_REGF_TSHUT_STAT] =
    REG_FIELD(BQ25630_REG_FAULT_STATUS, 3, 3),
    [BQ25630_REGF_OTG_FAULT_STAT] =
    REG_FIELD(BQ25630_REG_FAULT_STATUS, 4, 4),
    [BQ25630_REGF_VSYS_FAULT_STAT] =
    REG_FIELD(BQ25630_REG_FAULT_STATUS, 5, 5),
    [BQ25630_REGF_BAT_FAULT_STAT] =
    REG_FIELD(BQ25630_REG_FAULT_STATUS, 6, 6),
    [BQ25630_REGF_VBUS_FAULT_STAT] =
    REG_FIELD(BQ25630_REG_FAULT_STATUS, 7, 7),
    };
// 8-bit value regmap.
    static const struct regmap_range bq25630_read_reg_range8[] = {
    regmap_reg_range(BQ25630_REG_CHARGE_TIMER_CONTROL,
    BQ25630_REG_FAULT_MASK),
    regmap_reg_range(BQ25630_REG_ADC_CONTROL,
    BQ25630_REG_ADC_CHANNEL_DISABLE_1),
    regmap_reg_range(BQ25630_REG_USB_C_CONTROL_0,
    BQ25630_REG_PART_INFORMATION),
    };
    static const struct regmap_range bq25630_write_reg_range8[] = {
    regmap_reg_range(BQ25630_REG_CHARGE_TIMER_CONTROL,
    BQ25630_REG_NTC_CONTROL_3),
    regmap_reg_range(BQ25630_REG_CHARGER_MASK_0,
    BQ25630_REG_FAULT_MASK),
    regmap_reg_range(BQ25630_REG_ADC_CONTROL,
    BQ25630_REG_ADC_CHANNEL_DISABLE_1),
    regmap_reg_range(BQ25630_REG_USB_C_CONTROL_0,
    BQ25630_REG_LIQUID_CONTROL_1),
    regmap_reg_range(BQ25630_REG_USB_DAC_CONTROL_0,
    BQ25630_REG_USB_DAC_CONTROL_1),
    };
    static const struct regmap_access_table bq25630_read_reg_access_table8 = {
    .yes_ranges = bq25630_read_reg_range8,
    .n_yes_ranges = ARRAY_SIZE(bq25630_read_reg_range8),
    };
    static const struct regmap_access_table bq25630_write_reg_access_table8 = {
    .yes_ranges = bq25630_write_reg_range8,
    .n_yes_ranges = ARRAY_SIZE(bq25630_write_reg_range8),
    };
    static const struct regmap_config bq25630_regmap_config8 = {
    .name = BQ25630_DRV_NAME "-8bit",
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = BQ25630_REG_PART_INFORMATION,
    .rd_table = &bq25630_read_reg_access_table8,
    .wr_table = &bq25630_write_reg_access_table8,
    };
// 16-bit little-endian value regmap.
    static const struct regmap_range bq25630_read_reg_range16le[] = {
    regmap_reg_range(BQ25630_REG_CHARGE_CURRENT_LIMIT,
    BQ25630_REG_TERMINATION_CONTROL),
    regmap_reg_range(BQ25630_REG_ICO_CURRENT_LIMIT,
    BQ25630_REG_ICO_CURRENT_LIMIT),
    };
    static const struct regmap_range bq25630_write_reg_range16le[] = {
    regmap_reg_range(BQ25630_REG_CHARGE_CURRENT_LIMIT,
    BQ25630_REG_TERMINATION_CONTROL),
    regmap_reg_range(BQ25630_REG_ICO_CURRENT_LIMIT,
    BQ25630_REG_ICO_CURRENT_LIMIT),
    };
    static const struct regmap_access_table bq25630_read_reg_access_table16le = {
    .yes_ranges = bq25630_read_reg_range16le,
    .n_yes_ranges = ARRAY_SIZE(bq25630_read_reg_range16le),
    };
    static const struct regmap_access_table bq25630_write_reg_access_table16le = {
    .yes_ranges = bq25630_write_reg_range16le,
    .n_yes_ranges = ARRAY_SIZE(bq25630_write_reg_range16le),
    };
    static const struct regmap_config bq25630_regmap_config16le = {
    .name = BQ25630_DRV_NAME "-16bit-le",
    .reg_bits = 8,
    .val_bits = 16,
    .reg_stride = 2,
    .val_format_endian = REGMAP_ENDIAN_LITTLE,
//
// Datasheet doesn't mention that this register is little-endian, but it
// looks like it?
//
    .max_register = BQ25630_REG_TERMINATION_CONTROL,
    .rd_table = &bq25630_read_reg_access_table16le,
    .wr_table = &bq25630_write_reg_access_table16le,
    };
// 16-bit big-endian value regmap.
    static const struct regmap_range bq25630_read_reg_range16be[] = {
    regmap_reg_range(BQ25630_REG_IBUS_ADC,
    BQ25630_REG_TDIE_ADC),
    };
    static const struct regmap_access_table bq25630_read_reg_access_table16be = {
    .yes_ranges = bq25630_read_reg_range16be,
    .n_yes_ranges = ARRAY_SIZE(bq25630_read_reg_range16be),
    };
    static const struct regmap_config bq25630_regmap_config16be = {
    .name = BQ25630_DRV_NAME "-16bit-be",
    .reg_bits = 8,
    .val_bits = 16,
    .reg_stride = 2,
    .val_format_endian = REGMAP_ENDIAN_BIG,
    .max_register = BQ25630_REG_TDIE_ADC,
    .rd_table = &bq25630_read_reg_access_table16be,
    .wr_table = core::ptr::null_mut(),
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bq25630_data {
    pub dev: *mut device,
    pub regmap8: *mut regmap,
    pub regmap16le: *mut regmap,
    pub regmap16be: *mut regmap,
    pub regfields: [*mut regmap_field; BQ25630_REGF_MAX],
    pub psy: *mut power_supply,
// State status from IRQs.
    pub statregs: [u8; BQ25630_NR_STAT_REGS],
}

    static int bq25630_alloc_regfield_range(struct bq25630_data *data,
    const enum bq25630_regfield from,
    const enum bq25630_regfield to,
    struct regmap *regmap)
    {
    int i;
    for (i = from; i <= to; ++i) {
    data.regfields[i] = devm_regmap_field_alloc(
    data.dev, regmap, bq25630_regfields[i]);
    if (IS_ERR(data.regfields[i]))
    return dev_err_probe(
    data.dev, PTR_ERR(data.regfields[i]),
    "Could not allocate register field %d\n", i);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bq25630_irq_thread(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t bq25630_irq_thread(int irq, void *dev_id)
    {
    struct bq25630_data *data = dev_id;
    u8 regbuf[BQ25630_NR_STAT_REGS];
    int ret;
    BUILD_BUG_ON(ARRAY_SIZE(regbuf) != ARRAY_SIZE(data.statregs));
    ret = regmap_bulk_read(data.regmap8, BQ25630_REG_CHARGER_STATUS_0,
    regbuf, ARRAY_SIZE(regbuf));
    if (ret) {
    dev_err(data.dev, "Could not bulk read IRQ registers (%d)\n",
    ret);
    goto out;
    }
    if (memcmp(data.statregs, regbuf, ARRAY_SIZE(data.statregs))) {
    power_supply_changed(data.psy);
    memcpy(data.statregs, regbuf, ARRAY_SIZE(data.statregs));
    }
    out:
    return IRQ_HANDLED;
    }
    static int bq25630_read_limit(struct bq25630_data *data,
    const enum bq25630_regfield regfield,
    const int minval, const int step,
    const int minregval, int *val)
    {
    unsigned int regval;
    int ret;
    ret = regmap_field_read(data.regfields[regfield], &regval);
    if (ret) {
    dev_err(data.dev, "Could not read limit (%d)\n", ret);
    return ret;
    }
// val = minval + step * (regval - minregval);
    return 0;
    }
    static int bq25630_write_limit(struct bq25630_data *data,
    const enum bq25630_regfield regfield,
    const int minval, const int maxval,
    const int step, const int minregval, int val)
    {
    unsigned int regval;
    int ret;
    val = clamp(val, minval, maxval);
    regval = minregval + ((val - minval) / step);
    ret = regmap_field_write(data.regfields[regfield], regval);
    if (ret) {
    dev_err(data.dev, "Could not write limit (%d)\n", ret);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bq25630_read_charge_type(data: *mut bq25630_data, val: *mut c_int) -> c_int {
    static int bq25630_read_charge_type(struct bq25630_data *data, int *val)
    {
    unsigned int regval;
    int ret;
    ret = regmap_field_read(data.regfields[BQ25630_REGF_CHG_STAT],
    &regval);
    if (ret) {
    dev_err(data.dev, "Could not read charge type (%d)\n", ret);
    return ret;
    }
    switch (regval) {
    case BQ25630_CHG_STAT_NOT_CHARGING:
// val = POWER_SUPPLY_CHARGE_TYPE_NONE;
    break;
    case BQ25630_CHG_STAT_TRICKLE_CHARGE:
    case BQ25630_CHG_STAT_PRE_CHARGE:
// val = POWER_SUPPLY_CHARGE_TYPE_TRICKLE;
    break;
    case BQ25630_CHG_STAT_FAST_CHARGE:
// val = POWER_SUPPLY_CHARGE_TYPE_FAST;
    break;
    case BQ25630_CHG_STAT_TAPER_CHARGE:
// val = POWER_SUPPLY_CHARGE_TYPE_LONGLIFE;
    break;
    case BQ25630_CHG_STAT_TERMINATION:
// val = POWER_SUPPLY_CHARGE_TYPE_BYPASS;
    break;
    default:
// val = POWER_SUPPLY_CHARGE_TYPE_UNKNOWN;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bq25630_read_health(data: *mut bq25630_data, val: *mut c_int) -> c_int {
    static int bq25630_read_health(struct bq25630_data *data, int *val)
    {
    unsigned int regval;
    int ret;
    u8 temp;
    ret = regmap_read(data.regmap8, BQ25630_REG_FAULT_STATUS, &regval);
    if (ret) {
    dev_err(data.dev, "Could not read fault status (%d)\n", ret);
    return ret;
    }
    temp = regval & GENMASK(bq25630_regfields[BQ25630_REGF_TS_STAT].msb,
    bq25630_regfields[BQ25630_REGF_TS_STAT].lsb);
    if (regval &
    GENMASK(bq25630_regfields[BQ25630_REGF_VBUS_FAULT_STAT].msb,
    bq25630_regfields[BQ25630_REGF_VBUS_FAULT_STAT].lsb)) {
// val = POWER_SUPPLY_HEALTH_OVERVOLTAGE;
    } else if (regval &
    GENMASK(bq25630_regfields[BQ25630_REGF_BAT_FAULT_STAT].msb,
    bq25630_regfields[BQ25630_REGF_BAT_FAULT_STAT].lsb)) {
//
// We can't differentiate between dead, under voltage or over
// voltage.
//
// val = POWER_SUPPLY_HEALTH_UNSPEC_FAILURE;
    } else if (regval &
    GENMASK(bq25630_regfields[BQ25630_REGF_VSYS_FAULT_STAT].msb,
    bq25630_regfields[BQ25630_REGF_VSYS_FAULT_STAT].lsb)) {
//
// We can't differentiate between under voltage or over voltage.
//
// val = POWER_SUPPLY_HEALTH_UNSPEC_FAILURE;
    } else if (regval &
    GENMASK(bq25630_regfields[BQ25630_REGF_OTG_FAULT_STAT].msb,
    bq25630_regfields[BQ25630_REGF_OTG_FAULT_STAT].lsb)) {
//
// We can't differentiate between under voltage or over voltage.
//
// val = POWER_SUPPLY_HEALTH_UNSPEC_FAILURE;
    } else if (regval &
    GENMASK(bq25630_regfields[BQ25630_REGF_TSHUT_STAT].msb,
    bq25630_regfields[BQ25630_REGF_TSHUT_STAT].lsb)) {
// Temperature shutdown is always due to hot temperatures.
// val = POWER_SUPPLY_HEALTH_HOT;
    } else if (temp) {
    switch (temp) {
    case BQ25630_TS_STAT_COLD:
// val = POWER_SUPPLY_HEALTH_COLD;
    break;
    case BQ25630_TS_STAT_COOL:
// val = POWER_SUPPLY_HEALTH_COOL;
    break;
    case BQ25630_TS_STAT_WARM:
// val = POWER_SUPPLY_HEALTH_WARM;
    break;
    case BQ25630_TS_STAT_HOT:
// val = POWER_SUPPLY_HEALTH_HOT;
    break;
    default:
// Interpret PRECOOL and PREWARM as NORMAL.
// val = POWER_SUPPLY_HEALTH_GOOD;
    }
    } else {
// val = POWER_SUPPLY_HEALTH_GOOD;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bq25630_read_vbus(data: *mut bq25630_data, val: *mut c_int) -> c_int {
    static int bq25630_read_vbus(struct bq25630_data *data, int *val)
    {
    unsigned int regval;
    int ret;
    ret = regmap_field_read(data.regfields[BQ25630_REGF_VBUS_STAT],
    &regval);
    if (ret) {
    dev_err(data.dev, "Could not read VBUS (%d)\n", ret);
    return ret;
    }
    switch (regval) {
    case BQ25630_VBUS_STAT_NONE:
// val = -1;
    break;
    case BQ25630_VBUS_STAT_SDP:
// val = POWER_SUPPLY_USB_TYPE_SDP;
    break;
    case BQ25630_VBUS_STAT_CDP:
// val = POWER_SUPPLY_USB_TYPE_CDP;
    break;
    case BQ25630_VBUS_STAT_DCP:
    case BQ25630_VBUS_STAT_HVDCP:
// val = POWER_SUPPLY_USB_TYPE_DCP;
    break;
    case BQ25630_VBUS_STAT_USB_C_DEFAULT:
    case BQ25630_VBUS_STAT_USB_C_MEDIUM:
    case BQ25630_VBUS_STAT_USB_C_HIGH:
// val = POWER_SUPPLY_USB_TYPE_C;
    break;
    default:
// val = POWER_SUPPLY_USB_TYPE_UNKNOWN;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bq25630_get_status(data: *mut bq25630_data, val: *mut c_int) -> c_int {
    static int bq25630_get_status(struct bq25630_data *data, int *val)
    {
    unsigned int regval;
    int ret;
    ret = regmap_field_read(data.regfields[BQ25630_REGF_PG_STAT], &regval);
    if (ret) {
    dev_err(data.dev, "Could not read PG status (%d)", ret);
    return ret;
    }
    if (!regval) {
// There is not enough power, battery must be discharging.
// val = POWER_SUPPLY_STATUS_DISCHARGING;
    return 0;
    }
    ret = regmap_field_read(data.regfields[BQ25630_REGF_EN_CHG], &regval);
    if (ret) {
    dev_err(data.dev, "Could not read charge status (%d)", ret);
    return ret;
    }
    if (!regval) {
// Charging is not enabled, battery must be discharging.
// val = POWER_SUPPLY_STATUS_DISCHARGING;
    return 0;
    }
    ret = bq25630_read_charge_type(data, val);
    if (ret)
    return ret;
    switch (*val) {
    case POWER_SUPPLY_CHARGE_TYPE_NONE:
// val = POWER_SUPPLY_STATUS_NOT_CHARGING;
    break;
    case POWER_SUPPLY_CHARGE_TYPE_BYPASS:
// Corresponds to BQ25630_CHG_STAT_TERMINATION.
// val = POWER_SUPPLY_STATUS_FULL;
    break;
    default:
// val = POWER_SUPPLY_STATUS_CHARGING;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bq25630_reset(data: *mut bq25630_data) -> c_int {
    static int bq25630_reset(struct bq25630_data *data)
    {
    let mut regval: c_uint = 1;
    int ret;
    ret = regmap_field_force_write(data.regfields[BQ25630_REGF_REG_RST],
    regval);
    if (ret) {
    dev_err(data.dev,
    "Could not force write reset register field (%d)\n",
    ret);
    return ret;
    }
//
// After a successful register reset, the device signals by resetting
// this register field to 0. Try reading it for some interrupt cycles.
//
    ret = regmap_field_read_poll_timeout(
    data.regfields[BQ25630_REGF_REG_RST], regval, regval == 0, 256,
    100000);
    if (ret) {
    dev_err(data.dev, "Could not read reset register field (%d)\n",
    ret);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bq25630_setup(psy: *mut power_supply) -> c_int {
    static int bq25630_setup(struct power_supply *psy)
    {
    struct bq25630_data *data = power_supply_get_drvdata(psy);
    struct power_supply_battery_info *batinfo;
    int ret;
    ret = bq25630_reset(data);
    if (ret) {
    dev_err(data.dev, "Could not reset device (%d)\n", ret);
    return ret;
    }
// Disable the watchdog.
    ret = regmap_field_write(data.regfields[BQ25630_REGF_WATCHDOG], 0);
    if (ret) {
    dev_err(data.dev, "Could not write watchdog timer (%d)\n",
    ret);
    return ret;
    }
    ret = power_supply_get_battery_info(psy, &batinfo);
    if (ret) {
    dev_err(data.dev, "Could not get battery info (%d)\n", ret);
    return ret;
    }
//
// Set values according to battery info. Warn on missing "dangerous"
// properties.
//
    if (batinfo.voltage_min_design_uv >= 0) {
    ret = bq25630_write_limit(data, BQ25630_REGF_VSYSMIN,
    BQ25630_VSYSMIN_MIN,
    BQ25630_VSYSMIN_MAX,
    BQ25630_VSYSMIN_STEP,
    BQ25630_VSYSMIN_MIN_REGVAL,
    batinfo.voltage_min_design_uv);
    if (ret)
    goto out_put_batinfo;
    } else
    dev_warn(data.dev,
    "Using default value for minimum voltage\n");
    if (batinfo.constant_charge_voltage_max_uv >= 0) {
    ret = bq25630_write_limit(
    data, BQ25630_REGF_VREG, BQ25630_VREG_MIN,
    BQ25630_VREG_MAX, BQ25630_VREG_STEP,
    BQ25630_VREG_MIN_REGVAL,
    batinfo.constant_charge_voltage_max_uv);
    if (ret)
    goto out_put_batinfo;
    } else
    dev_warn(data.dev,
    "Using default value for maximum constant charge voltage\n");
    if (batinfo.constant_charge_current_max_ua >= 0) {
    ret = bq25630_write_limit(
    data, BQ25630_REGF_ICHG, BQ25630_ICHG_MIN,
    BQ25630_ICHG_MAX, BQ25630_ICHG_STEP,
    BQ25630_ICHG_MIN_REGVAL,
    batinfo.constant_charge_current_max_ua);
    if (ret)
    goto out_put_batinfo;
    } else
    dev_warn(data.dev,
    "Using default value for maximum constant charge current\n");
    if (batinfo.charge_term_current_ua >= 0) {
    ret = bq25630_write_limit(
    data, BQ25630_REGF_ITERM, BQ25630_ITERM_MIN,
    BQ25630_ITERM_MAX, BQ25630_ITERM_STEP,
    BQ25630_ITERM_MIN_REGVAL,
    batinfo.charge_term_current_ua);
    if (ret)
    goto out_put_batinfo;
    }
    if (batinfo.precharge_current_ua >= 0) {
    ret = bq25630_write_limit(data, BQ25630_REGF_IPRECHG,
    BQ25630_IPRECHG_MIN,
    BQ25630_IPRECHG_MAX,
    BQ25630_IPRECHG_STEP,
    BQ25630_IPRECHG_MIN_REGVAL,
    batinfo.precharge_current_ua);
    if (ret)
    goto out_put_batinfo;
    }
    out_put_batinfo:
    power_supply_put_battery_info(psy, batinfo);
    return ret;
    }
    static int bq25630_charger_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct bq25630_data *data = power_supply_get_drvdata(psy);
    let mut ret: c_int = 0;
    switch (psp) {
    case POWER_SUPPLY_PROP_STATUS:
    ret = bq25630_get_status(data, &val.intval);
    break;
    case POWER_SUPPLY_PROP_CHARGE_TYPE:
    case POWER_SUPPLY_PROP_CHARGE_TYPES:
    ret = bq25630_read_charge_type(data, &val.intval);
    break;
    case POWER_SUPPLY_PROP_HEALTH:
    ret = bq25630_read_health(data, &val.intval);
    break;
    case POWER_SUPPLY_PROP_ONLINE:
    ret = regmap_field_read(data.regfields[BQ25630_REGF_EN_CHG],
    &val.intval);
    if (ret || !val.intval) {
// Charging is not even enabled.
    break;
    }
    ret = bq25630_read_vbus(data, &val.intval);
    val.intval = val.intval >= 0;
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_MIN:
    ret = bq25630_read_limit(data, BQ25630_REGF_VSYSMIN,
    BQ25630_VSYSMIN_MIN,
    BQ25630_VSYSMIN_STEP,
    BQ25630_VSYSMIN_MIN_REGVAL,
    &val.intval);
    break;
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT:
    ret = bq25630_read_limit(data, BQ25630_REGF_ICHG,
    BQ25630_ICHG_MIN, BQ25630_ICHG_STEP,
    BQ25630_ICHG_MIN_REGVAL, &val.intval);
    break;
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT_MAX:
    val.intval = BQ25630_ICHG_MAX;
    break;
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE:
    ret = bq25630_read_limit(data, BQ25630_REGF_VREG,
    BQ25630_VREG_MIN, BQ25630_VREG_STEP,
    BQ25630_VREG_MIN_REGVAL, &val.intval);
    break;
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE_MAX:
    val.intval = BQ25630_VREG_MAX;
    break;
    case POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT:
    ret = bq25630_read_limit(data, BQ25630_REGF_IINDPM,
    BQ25630_IINDPM_MIN,
    BQ25630_IINDPM_STEP,
    BQ25630_IINDPM_MIN_REGVAL,
    &val.intval);
    break;
    case POWER_SUPPLY_PROP_INPUT_VOLTAGE_LIMIT:
    ret = bq25630_read_limit(data, BQ25630_REGF_VINDPM,
    BQ25630_VINDPM_MIN,
    BQ25630_VINDPM_STEP,
    BQ25630_VINDPM_MIN_REGVAL,
    &val.intval);
    break;
    case POWER_SUPPLY_PROP_USB_TYPE:
    ret = bq25630_read_vbus(data, &val.intval);
    if (!ret && val.intval < 0) {
// Nothing connected.
    val.intval = POWER_SUPPLY_USB_TYPE_UNKNOWN;
    }
    break;
    case POWER_SUPPLY_PROP_PRECHARGE_CURRENT:
    ret = bq25630_read_limit(data, BQ25630_REGF_IPRECHG,
    BQ25630_IPRECHG_MIN,
    BQ25630_IPRECHG_STEP,
    BQ25630_IPRECHG_MIN_REGVAL,
    &val.intval);
    break;
    case POWER_SUPPLY_PROP_CHARGE_TERM_CURRENT:
    ret = bq25630_read_limit(data, BQ25630_REGF_ITERM,
    BQ25630_ITERM_MIN, BQ25630_ITERM_STEP,
    BQ25630_ITERM_MIN_REGVAL,
    &val.intval);
    break;
    case POWER_SUPPLY_PROP_MODEL_NAME:
    val.strval = "BQ25630";
    break;
    case POWER_SUPPLY_PROP_MANUFACTURER:
    val.strval = "Texas Instruments";
    break;
    default:
    return -EINVAL;
    }
    return ret;
    }
    static int bq25630_charger_set_property(struct power_supply *psy,
    enum power_supply_property psp,
    const union power_supply_propval *val)
    {
    struct bq25630_data *data = power_supply_get_drvdata(psy);
    let mut ret: c_int = 0;
    switch (psp) {
    case POWER_SUPPLY_PROP_ONLINE:
    ret = regmap_field_write(data.regfields[BQ25630_REGF_EN_CHG],
    !!val.intval);
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_MIN:
    ret = bq25630_write_limit(
    data, BQ25630_REGF_VSYSMIN, BQ25630_VSYSMIN_MIN,
    BQ25630_VSYSMIN_MAX, BQ25630_VSYSMIN_STEP,
    BQ25630_VSYSMIN_MIN_REGVAL, val.intval);
    break;
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT:
    ret = bq25630_write_limit(data, BQ25630_REGF_ICHG,
    BQ25630_ICHG_MIN, BQ25630_ICHG_MAX,
    BQ25630_ICHG_STEP,
    BQ25630_ICHG_MIN_REGVAL, val.intval);
    break;
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE:
    ret = bq25630_write_limit(data, BQ25630_REGF_VREG,
    BQ25630_VREG_MIN, BQ25630_VREG_MAX,
    BQ25630_VREG_STEP,
    BQ25630_VREG_MIN_REGVAL, val.intval);
    break;
    case POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT:
    ret = bq25630_write_limit(
    data, BQ25630_REGF_IINDPM, BQ25630_IINDPM_MIN,
    BQ25630_IINDPM_MAX, BQ25630_IINDPM_STEP,
    BQ25630_IINDPM_MIN_REGVAL, val.intval);
    break;
    case POWER_SUPPLY_PROP_INPUT_VOLTAGE_LIMIT:
    ret = bq25630_write_limit(
    data, BQ25630_REGF_VINDPM, BQ25630_VINDPM_MIN,
    BQ25630_VINDPM_MAX, BQ25630_VINDPM_STEP,
    BQ25630_VINDPM_MIN_REGVAL, val.intval);
    break;
    case POWER_SUPPLY_PROP_PRECHARGE_CURRENT:
    ret = bq25630_write_limit(
    data, BQ25630_REGF_IPRECHG, BQ25630_IPRECHG_MIN,
    BQ25630_IPRECHG_MAX, BQ25630_IPRECHG_STEP,
    BQ25630_IPRECHG_MIN_REGVAL, val.intval);
    break;
    case POWER_SUPPLY_PROP_CHARGE_TERM_CURRENT:
    ret = bq25630_write_limit(data, BQ25630_REGF_ITERM,
    BQ25630_ITERM_MIN, BQ25630_ITERM_MAX,
    BQ25630_ITERM_STEP,
    BQ25630_ITERM_MIN_REGVAL,
    val.intval);
    break;
    default:
    return -EINVAL;
    }
    return ret;
    }
    static int bq25630_charger_property_is_writeable(struct power_supply *psy,
    enum power_supply_property psp)
    {
    switch (psp) {
    case POWER_SUPPLY_PROP_ONLINE:
    case POWER_SUPPLY_PROP_VOLTAGE_MIN:
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT:
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE:
    case POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT:
    case POWER_SUPPLY_PROP_INPUT_VOLTAGE_LIMIT:
    case POWER_SUPPLY_PROP_PRECHARGE_CURRENT:
    case POWER_SUPPLY_PROP_CHARGE_TERM_CURRENT:
    return true;
    default:
    return false;
    }
    }
    static const enum power_supply_property bq25630_charger_properties[] = {
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_CHARGE_TYPE,
    POWER_SUPPLY_PROP_CHARGE_TYPES,
    POWER_SUPPLY_PROP_HEALTH,
    POWER_SUPPLY_PROP_ONLINE,
    POWER_SUPPLY_PROP_VOLTAGE_MIN,
    POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT,
    POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT_MAX,
    POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE,
    POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE_MAX,
    POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT,
    POWER_SUPPLY_PROP_INPUT_VOLTAGE_LIMIT,
    POWER_SUPPLY_PROP_USB_TYPE,
    POWER_SUPPLY_PROP_PRECHARGE_CURRENT,
    POWER_SUPPLY_PROP_CHARGE_TERM_CURRENT,
    POWER_SUPPLY_PROP_MODEL_NAME,
    POWER_SUPPLY_PROP_MANUFACTURER,
    };
    static const struct power_supply_desc bq25630_charger_psy_desc = {
    .name = BQ25630_DRV_NAME,
    .type = POWER_SUPPLY_TYPE_USB,
    .charge_types = BIT(POWER_SUPPLY_CHARGE_TYPE_NONE) |
    BIT(POWER_SUPPLY_CHARGE_TYPE_TRICKLE) |
    BIT(POWER_SUPPLY_CHARGE_TYPE_FAST) |
    BIT(POWER_SUPPLY_CHARGE_TYPE_LONGLIFE) |
    BIT(POWER_SUPPLY_CHARGE_TYPE_BYPASS) |
    BIT(POWER_SUPPLY_CHARGE_TYPE_UNKNOWN),
    .usb_types = BIT(POWER_SUPPLY_USB_TYPE_UNKNOWN) |
    BIT(POWER_SUPPLY_USB_TYPE_SDP) |
    BIT(POWER_SUPPLY_USB_TYPE_DCP) |
    BIT(POWER_SUPPLY_USB_TYPE_CDP) |
    BIT(POWER_SUPPLY_USB_TYPE_C),
    .properties = bq25630_charger_properties,
    .num_properties = ARRAY_SIZE(bq25630_charger_properties),
    .get_property = bq25630_charger_get_property,
    .set_property = bq25630_charger_set_property,
    .property_is_writeable = bq25630_charger_property_is_writeable,
    .init = bq25630_setup,
    };
#[no_mangle]
unsafe extern "C" fn bq25630_probe(client: *mut i2c_client) -> c_int {
    static int bq25630_probe(struct i2c_client *client)
    {
    let mut psy_cfg: power_supply_config = {};
    struct bq25630_data *data;
    int ret;
    data = devm_kzalloc(&client.dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.dev = &client.dev;
    data.regmap8 = devm_regmap_init_i2c(client, &bq25630_regmap_config8);
    if (IS_ERR(data.regmap8))
    return dev_err_probe(data.dev, PTR_ERR(data.regmap8),
    "Could not initialize regmap8\n");
    ret = bq25630_alloc_regfield_range(data, BQ25630_REGF_WATCHDOG,
    BQ25630_REGF_TS_STAT,
    data.regmap8);
    if (ret)
    return ret;
    data.regmap16le =
    devm_regmap_init_i2c(client, &bq25630_regmap_config16le);
    if (IS_ERR(data.regmap16le))
    return dev_err_probe(data.dev, PTR_ERR(data.regmap16le),
    "Could not initialize regmap16le\n");
    ret = bq25630_alloc_regfield_range(data, BQ25630_REGF_ICHG,
    BQ25630_REGF_ITERM,
    data.regmap16le);
    if (ret)
    return ret;
    data.regmap16be =
    devm_regmap_init_i2c(client, &bq25630_regmap_config16be);
    if (IS_ERR(data.regmap16be))
    return dev_err_probe(data.dev, PTR_ERR(data.regmap16be),
    "Could not initialize regmap16be\n");
    ret = bq25630_alloc_regfield_range(data, BQ25630_REGF_IBUS_ADC,
    BQ25630_REGF_VBUS_ADC,
    data.regmap16be);
    if (ret)
    return ret;
    psy_cfg.drv_data = data;
    psy_cfg.fwnode = dev_fwnode(data.dev);
    data.psy = devm_power_supply_register(
    data.dev, &bq25630_charger_psy_desc, &psy_cfg);
    if (IS_ERR(data.psy))
    return dev_err_probe(data.dev, PTR_ERR(data.psy),
    "Could not register power supply\n");
//
// Device sends active low 256 µs pulse to report status and fault.
//
// Note that we need to request this *after* registering the power
// supply so devm destructs it correctly in the reverse order. Otherwise
// spurious interrupts could call power_supply_changed() wrongly with a
// uninitialized/deallocated power supply.
//
    ret = devm_request_threaded_irq(data.dev, client.irq, core::ptr::null_mut(),
    bq25630_irq_thread,
    IRQF_TRIGGER_FALLING | IRQF_ONESHOT,
    core::ptr::null_mut(), data);
    if (ret)
    return dev_err_probe(data.dev, ret, "Could not request IRQ\n");
    return 0;
    }
    static const struct of_device_id bq25630_of_match[] = {
    {
    .compatible = "ti,bq25630",
    },
    {}
    };
    MODULE_DEVICE_TABLE(of, bq25630_of_match);
    static struct i2c_driver bq25630_driver = {
    .driver = {
    .name = BQ25630_DRV_NAME,
    .of_match_table = bq25630_of_match,
    },
    .probe = bq25630_probe,
    };
    module_i2c_driver(bq25630_driver);
    MODULE_AUTHOR("Waqar Hameed <waqar.hameed@axis.com>");
    MODULE_DESCRIPTION("TI BQ25630 charger driver");
    MODULE_LICENSE("GPL");
