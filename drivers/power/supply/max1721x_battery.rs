//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/max1721x_battery.c
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


//
// 1-Wire implementation for Maxim Semiconductor
// MAX7211/MAX17215 standalone fuel gauge chip
//
// Copyright (C) 2017 Radioavionica Corporation
// Author: Alex A. Mihaylov <minimumlaw@rambler.ru>
//
// Use consistent with the GNU GPL is permitted,
// provided that this copyright notice is
// preserved in its entirety in all copies and derived works.
//

pub const W1_MAX1721X_FAMILY_ID: c_uint = 0x26;

pub const PSY_MAX_NAME_LEN: c_int = 32;
// Number of valid register addresses in W1 mode
pub const MAX1721X_MAX_REG_NR: c_uint = 0x1EF;
// Factory settings (nonvolatile registers) (W1 specific)
pub const MAX1721X_REG_NRSENSE: c_uint = 0x1CF	/* RSense in 10^-5 Ohm */;
// Strings
pub const MAX1721X_REG_MFG_STR: c_uint = 0x1CC;
pub const MAX1721X_REG_MFG_NUMB: c_int = 3;
pub const MAX1721X_REG_DEV_STR: c_uint = 0x1DB;
pub const MAX1721X_REG_DEV_NUMB: c_int = 5;
// HEX Strings
pub const MAX1721X_REG_SER_HEX: c_uint = 0x1D8;
// MAX172XX Output Registers for W1 chips
pub const MAX172XX_REG_STATUS: c_uint = 0x000	/* status reg */;

pub const MAX172XX_REG_DEVNAME: c_uint = 0x021	/* chip config */;
pub const MAX172XX_DEV_MASK: c_uint = 0x000F	/* chip type mask */;
pub const MAX172X1_DEV: c_uint = 0x0001;
pub const MAX172X5_DEV: c_uint = 0x0005;
pub const MAX172XX_REG_TEMP: c_uint = 0x008	/* Temperature */;
pub const MAX172XX_REG_BATT: c_uint = 0x0DA	/* Battery voltage */;
pub const MAX172XX_REG_CURRENT: c_uint = 0x00A	/* Actual current */;
pub const MAX172XX_REG_AVGCURRENT: c_uint = 0x00B	/* Average current */;
pub const MAX172XX_REG_REPSOC: c_uint = 0x006	/* Percentage of charge */;
pub const MAX172XX_REG_DESIGNCAP: c_uint = 0x018	/* Design capacity */;
pub const MAX172XX_REG_REPCAP: c_uint = 0x005	/* Average capacity */;
pub const MAX172XX_REG_TTE: c_uint = 0x011	/* Time to empty */;
pub const MAX172XX_REG_TTF: c_uint = 0x020	/* Time to full */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max17211_device_info {
    pub name: [c_char; PSY_MAX_NAME_LEN],
    pub bat: *mut power_supply,
    pub bat_desc: power_supply_desc,
    pub w1_dev: *mut device,
    pub regmap: *mut regmap,
// battery design format
    pub /: *mut *mut unsigned int rsense; / in tenths uOhm,
    pub 1]: *mut *mut char DeviceName[2  MAX1721X_REG_DEV_NUMB +,
    pub 1]: *mut *mut char ManufacturerName[2  MAX1721X_REG_MFG_NUMB +,
    pub /: *mut *mut char SerialNumber[13]; / see get_sn_str() later for comment,
}

// Convert regs value to power_supply units
#[no_mangle]
pub unsafe extern "C" fn max172xx_time_to_ps(reg: c_uint) -> c_int {
    static inline int max172xx_time_to_ps(unsigned int reg)
    {
    return reg * 5625 / 1000;	/* in sec. */
    }
#[no_mangle]
pub unsafe extern "C" fn max172xx_percent_to_ps(reg: c_uint) -> c_int {
    static inline int max172xx_percent_to_ps(unsigned int reg)
    {
    return reg / 256;	/* in percent from 0 to 100 */
    }
#[no_mangle]
pub unsafe extern "C" fn max172xx_voltage_to_ps(reg: c_uint) -> c_int {
    static inline int max172xx_voltage_to_ps(unsigned int reg)
    {
    return reg * 1250;	/* in uV */
    }
#[no_mangle]
pub unsafe extern "C" fn max172xx_capacity_to_ps(reg: c_uint) -> c_int {
    static inline int max172xx_capacity_to_ps(unsigned int reg)
    {
    return reg * 500;	/* in uAh */
    }
//
// Current and temperature is signed values, so unsigned regs
// value must be converted to signed type
//
#[no_mangle]
pub unsafe extern "C" fn max172xx_temperature_to_ps(reg: c_uint) -> c_int {
    static inline int max172xx_temperature_to_ps(unsigned int reg)
    {
    let mut val: c_int = (int16_t)(reg);
    return val * 10 / 256; /* in tenths of deg. C */
    }
//
// Calculating current registers resolution:
//
// RSense stored in 10^-5 Ohm, so measurement voltage must be
// in 10^-11 Volts for get current in uA.
// 16 bit current reg fullscale +/-51.2mV is 102400 uV.
// So: 102400 / 65535 * 10^5 = 156252
//
#[no_mangle]
pub unsafe extern "C" fn max172xx_current_to_voltage(reg: c_uint) -> c_int {
    static inline int max172xx_current_to_voltage(unsigned int reg)
    {
    let mut val: c_int = (int16_t)(reg);
    return val * 156252;
    }
    static inline struct max17211_device_info *
    to_device_info(struct power_supply *psy)
    {
    return power_supply_get_drvdata(psy);
    }
    static int max1721x_battery_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct max17211_device_info *info = to_device_info(psy);
    let mut reg: c_uint = 0;
    let mut ret: c_int = 0;
    switch (psp) {
    case POWER_SUPPLY_PROP_PRESENT:
//
// POWER_SUPPLY_PROP_PRESENT will always readable via
// sysfs interface. Value return 0 if battery not
// present or unaccessible via W1.
//
    val.intval =
    regmap_read(info.regmap, MAX172XX_REG_STATUS,
    &reg) ? 0 : !(reg & MAX172XX_BAT_PRESENT);
    break;
    case POWER_SUPPLY_PROP_CAPACITY:
    ret = regmap_read(info.regmap, MAX172XX_REG_REPSOC, &reg);
    val.intval = max172xx_percent_to_ps(reg);
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_NOW:
    ret = regmap_read(info.regmap, MAX172XX_REG_BATT, &reg);
    val.intval = max172xx_voltage_to_ps(reg);
    break;
    case POWER_SUPPLY_PROP_CHARGE_FULL_DESIGN:
    ret = regmap_read(info.regmap, MAX172XX_REG_DESIGNCAP, &reg);
    val.intval = max172xx_capacity_to_ps(reg);
    break;
    case POWER_SUPPLY_PROP_CHARGE_AVG:
    ret = regmap_read(info.regmap, MAX172XX_REG_REPCAP, &reg);
    val.intval = max172xx_capacity_to_ps(reg);
    break;
    case POWER_SUPPLY_PROP_TIME_TO_EMPTY_AVG:
    ret = regmap_read(info.regmap, MAX172XX_REG_TTE, &reg);
    val.intval = max172xx_time_to_ps(reg);
    break;
    case POWER_SUPPLY_PROP_TIME_TO_FULL_AVG:
    ret = regmap_read(info.regmap, MAX172XX_REG_TTF, &reg);
    val.intval = max172xx_time_to_ps(reg);
    break;
    case POWER_SUPPLY_PROP_TEMP:
    ret = regmap_read(info.regmap, MAX172XX_REG_TEMP, &reg);
    val.intval = max172xx_temperature_to_ps(reg);
    break;
// We need signed current, so must cast info->rsense to signed type
    case POWER_SUPPLY_PROP_CURRENT_NOW:
    ret = regmap_read(info.regmap, MAX172XX_REG_CURRENT, &reg);
    val.intval =
    max172xx_current_to_voltage(reg) / (int)info.rsense;
    break;
    case POWER_SUPPLY_PROP_CURRENT_AVG:
    ret = regmap_read(info.regmap, MAX172XX_REG_AVGCURRENT, &reg);
    val.intval =
    max172xx_current_to_voltage(reg) / (int)info.rsense;
    break;
//
// Strings already received and inited by probe.
// We do dummy read for check battery still available.
//
    case POWER_SUPPLY_PROP_MODEL_NAME:
    ret = regmap_read(info.regmap, MAX1721X_REG_DEV_STR, &reg);
    val.strval = info.DeviceName;
    break;
    case POWER_SUPPLY_PROP_MANUFACTURER:
    ret = regmap_read(info.regmap, MAX1721X_REG_MFG_STR, &reg);
    val.strval = info.ManufacturerName;
    break;
    case POWER_SUPPLY_PROP_SERIAL_NUMBER:
    ret = regmap_read(info.regmap, MAX1721X_REG_SER_HEX, &reg);
    val.strval = info.SerialNumber;
    break;
    default:
    ret = -EINVAL;
    }
    return ret;
    }
    static enum power_supply_property max1721x_battery_props[] = {
// int
    POWER_SUPPLY_PROP_PRESENT,
    POWER_SUPPLY_PROP_CAPACITY,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    POWER_SUPPLY_PROP_CHARGE_FULL_DESIGN,
    POWER_SUPPLY_PROP_CHARGE_AVG,
    POWER_SUPPLY_PROP_TIME_TO_EMPTY_AVG,
    POWER_SUPPLY_PROP_TIME_TO_FULL_AVG,
    POWER_SUPPLY_PROP_TEMP,
    POWER_SUPPLY_PROP_CURRENT_NOW,
    POWER_SUPPLY_PROP_CURRENT_AVG,
// strings
    POWER_SUPPLY_PROP_MODEL_NAME,
    POWER_SUPPLY_PROP_MANUFACTURER,
    POWER_SUPPLY_PROP_SERIAL_NUMBER,
    };
    static int get_string(struct max17211_device_info *info,
    uint16_t reg, uint8_t nr, char *str)
    {
    unsigned int val;
    if (!str || !(reg == MAX1721X_REG_MFG_STR ||
    reg == MAX1721X_REG_DEV_STR))
    return -EFAULT;
    while (nr--) {
    if (regmap_read(info.regmap, reg++, &val))
    return -EFAULT;
// str++ = val>>8 & 0x00FF;
// str++ = val & 0x00FF;
    }
    return 0;
    }
// Maxim say: Serial number is a hex string up to 12 hex characters
#[no_mangle]
unsafe extern "C" fn get_sn_string(info: *mut max17211_device_info, str: *mut c_char) -> c_int {
    static int get_sn_string(struct max17211_device_info *info, char *str)
    {
    unsigned int val[3];
    if (!str)
    return -EFAULT;
    if (regmap_read(info.regmap, MAX1721X_REG_SER_HEX, &val[0]))
    return -EFAULT;
    if (regmap_read(info.regmap, MAX1721X_REG_SER_HEX + 1, &val[1]))
    return -EFAULT;
    if (regmap_read(info.regmap, MAX1721X_REG_SER_HEX + 2, &val[2]))
    return -EFAULT;
    snprintf(str, 13, "%04X%04X%04X", val[0], val[1], val[2]);
    return 0;
    }
//
// MAX1721x registers description for w1-regmap
//
    static const struct regmap_range max1721x_allow_range[] = {
    regmap_reg_range(0, 0xDF),	/* volatile data */
    regmap_reg_range(0x180, 0x1DF),	/* non-volatile memory */
    regmap_reg_range(0x1E0, 0x1EF),	/* non-volatile history (unused) */
    };
    static const struct regmap_range max1721x_deny_range[] = {
// volatile data unused registers
    regmap_reg_range(0x24, 0x26),
    regmap_reg_range(0x30, 0x31),
    regmap_reg_range(0x33, 0x34),
    regmap_reg_range(0x37, 0x37),
    regmap_reg_range(0x3B, 0x3C),
    regmap_reg_range(0x40, 0x41),
    regmap_reg_range(0x43, 0x44),
    regmap_reg_range(0x47, 0x49),
    regmap_reg_range(0x4B, 0x4C),
    regmap_reg_range(0x4E, 0xAF),
    regmap_reg_range(0xB1, 0xB3),
    regmap_reg_range(0xB5, 0xB7),
    regmap_reg_range(0xBF, 0xD0),
    regmap_reg_range(0xDB, 0xDB),
// hole between volatile and non-volatile registers
    regmap_reg_range(0xE0, 0x17F),
    };
    static const struct regmap_access_table max1721x_regs = {
    .yes_ranges	= max1721x_allow_range,
    .n_yes_ranges	= ARRAY_SIZE(max1721x_allow_range),
    .no_ranges	= max1721x_deny_range,
    .n_no_ranges	= ARRAY_SIZE(max1721x_deny_range),
    };
//
// Model Gauge M5 Algorithm output register
// Volatile data (must not be cached)
//
    static const struct regmap_range max1721x_volatile_allow[] = {
    regmap_reg_range(0, 0xDF),
    };
    static const struct regmap_access_table max1721x_volatile_regs = {
    .yes_ranges	= max1721x_volatile_allow,
    .n_yes_ranges	= ARRAY_SIZE(max1721x_volatile_allow),
    };
//
// W1-regmap config
//
    static const struct regmap_config max1721x_regmap_w1_config = {
    .reg_bits = 16,
    .val_bits = 16,
    .rd_table = &max1721x_regs,
    .volatile_table = &max1721x_volatile_regs,
    .max_register = MAX1721X_MAX_REG_NR,
    };
#[no_mangle]
unsafe extern "C" fn devm_w1_max1721x_add_device(sl: *mut w1_slave) -> c_int {
    static int devm_w1_max1721x_add_device(struct w1_slave *sl)
    {
    let mut psy_cfg: power_supply_config = {};
    struct max17211_device_info *info;
    info = devm_kzalloc(&sl.dev, sizeof(*info), GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    sl.family_data = (void *)info;
    info.w1_dev = &sl.dev;
//
// power_supply class battery name translated from W1 slave device
// unique ID (look like 26-0123456789AB) to "max1721x-0123456789AB\0"
// so, 26 (device family) correspond to max1721x devices.
// Device name still unique for any number of connected devices.
//
    snprintf(info.name, sizeof(info.name),
    "max1721x-%012X", (unsigned int)sl.reg_num.id);
    info.bat_desc.name = info.name;
//
// FixMe: battery device name exceed max len for thermal_zone device
// name and translation to thermal_zone must be disabled.
//
    info.bat_desc.no_thermal = true;
    info.bat_desc.type = POWER_SUPPLY_TYPE_BATTERY;
    info.bat_desc.properties = max1721x_battery_props;
    info.bat_desc.num_properties = ARRAY_SIZE(max1721x_battery_props);
    info.bat_desc.get_property = max1721x_battery_get_property;
    psy_cfg.drv_data = info;
// regmap init
    info.regmap = devm_regmap_init_w1(info.w1_dev,
    &max1721x_regmap_w1_config);
    if (IS_ERR(info.regmap)) {
    let mut err: c_int = PTR_ERR(info.regmap);
    dev_err(info.w1_dev, "Failed to allocate register map: %d\n",
    err);
    return err;
    }
// rsense init
    info.rsense = 0;
    if (regmap_read(info.regmap, MAX1721X_REG_NRSENSE, &info.rsense)) {
    dev_err(info.w1_dev, "Can't read RSense. Hardware error.\n");
    return -ENODEV;
    }
    if (!info.rsense) {
    dev_warn(info.w1_dev, "RSense not calibrated, set 10 mOhms!\n");
    info.rsense = 1000; /* in regs in 10^-5 */
    }
    dev_info(info.w1_dev, "RSense: %d mOhms.\n", info.rsense / 100);
    if (get_string(info, MAX1721X_REG_MFG_STR,
    MAX1721X_REG_MFG_NUMB, info.ManufacturerName)) {
    dev_err(info.w1_dev, "Can't read manufacturer. Hardware error.\n");
    return -ENODEV;
    }
    if (!info.ManufacturerName[0])
    strscpy(info.ManufacturerName, DEF_MFG_NAME,
    2 * MAX1721X_REG_MFG_NUMB);
    if (get_string(info, MAX1721X_REG_DEV_STR,
    MAX1721X_REG_DEV_NUMB, info.DeviceName)) {
    dev_err(info.w1_dev, "Can't read device. Hardware error.\n");
    return -ENODEV;
    }
    if (!info.DeviceName[0]) {
    unsigned int dev_name;
    if (regmap_read(info.regmap,
    MAX172XX_REG_DEVNAME, &dev_name)) {
    dev_err(info.w1_dev, "Can't read device name reg.\n");
    return -ENODEV;
    }
    switch (dev_name & MAX172XX_DEV_MASK) {
    case MAX172X1_DEV:
    strscpy(info.DeviceName, DEF_DEV_NAME_MAX17211,
    2 * MAX1721X_REG_DEV_NUMB);
    break;
    case MAX172X5_DEV:
    strscpy(info.DeviceName, DEF_DEV_NAME_MAX17215,
    2 * MAX1721X_REG_DEV_NUMB);
    break;
    default:
    strscpy(info.DeviceName, DEF_DEV_NAME_UNKNOWN,
    2 * MAX1721X_REG_DEV_NUMB);
    }
    }
    if (get_sn_string(info, info.SerialNumber)) {
    dev_err(info.w1_dev, "Can't read serial. Hardware error.\n");
    return -ENODEV;
    }
    info.bat = devm_power_supply_register(&sl.dev, &info.bat_desc,
    &psy_cfg);
    if (IS_ERR(info.bat)) {
    dev_err(info.w1_dev, "failed to register battery\n");
    return PTR_ERR(info.bat);
    }
    return 0;
    }
    static const struct w1_family_ops w1_max1721x_fops = {
    .add_slave = devm_w1_max1721x_add_device,
    };
    static struct w1_family w1_max1721x_family = {
    .fid = W1_MAX1721X_FAMILY_ID,
    .fops = &w1_max1721x_fops,
    };
    module_w1_family(w1_max1721x_family);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Alex A. Mihaylov <minimumlaw@rambler.ru>");
    MODULE_DESCRIPTION("Maxim MAX17211/MAX17215 Fuel Gauge IC driver");
    MODULE_ALIAS("w1-family-" __stringify(W1_MAX1721X_FAMILY_ID));
