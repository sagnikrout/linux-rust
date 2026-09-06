//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/olpc_battery.c
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
// Battery driver for One Laptop Per Child board.
//
// Copyright © 2006-2010  David Woodhouse <dwmw2@infradead.org>
//

pub const EC_BAT_VOLTAGE: c_uint = 0x10	/* uint16_t,	*9.76/32,    mV   */;
pub const EC_BAT_CURRENT: c_uint = 0x11	/* int16_t,	*15.625/120, mA   */;
pub const EC_BAT_ACR: c_uint = 0x12	/* int16_t,	*6250/15,    µAh  */;
pub const EC_BAT_TEMP: c_uint = 0x13	/* uint16_t,	*100/256,   °C  */;
pub const EC_AMB_TEMP: c_uint = 0x14	/* uint16_t,	*100/256,   °C  */;
pub const EC_BAT_STATUS: c_uint = 0x15	/* uint8_t,	bitmask */;
pub const EC_BAT_SOC: c_uint = 0x16	/* uint8_t,	percentage */;
pub const EC_BAT_SERIAL: c_uint = 0x17	/* uint8_t[6] */;
pub const EC_BAT_EEPROM: c_uint = 0x18	/* uint8_t adr as input, uint8_t output */;
pub const EC_BAT_ERRCODE: c_uint = 0x1f	/* uint8_t,	bitmask */;
pub const BAT_STAT_PRESENT: c_uint = 0x01;
pub const BAT_STAT_FULL: c_uint = 0x02;
pub const BAT_STAT_LOW: c_uint = 0x04;
pub const BAT_STAT_DESTROY: c_uint = 0x08;
pub const BAT_STAT_AC: c_uint = 0x10;
pub const BAT_STAT_CHARGING: c_uint = 0x20;
pub const BAT_STAT_DISCHARGING: c_uint = 0x40;
pub const BAT_STAT_TRICKLE: c_uint = 0x80;
pub const BAT_ERR_INFOFAIL: c_uint = 0x02;
pub const BAT_ERR_OVERVOLTAGE: c_uint = 0x04;
pub const BAT_ERR_OVERTEMP: c_uint = 0x05;
pub const BAT_ERR_GAUGESTOP: c_uint = 0x06;
pub const BAT_ERR_OUT_OF_CONTROL: c_uint = 0x07;
pub const BAT_ERR_ID_FAIL: c_uint = 0x09;
pub const BAT_ERR_ACR_FAIL: c_uint = 0x10;
pub const BAT_ADDR_MFR_TYPE: c_uint = 0x5F;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct olpc_battery_data {
    pub olpc_ac: *mut power_supply,
    pub olpc_bat: *mut power_supply,
    pub bat_serial: [c_char; 17],
    pub new_proto: bool,
    pub little_endian: bool,
}

//
// Power
//
    static int olpc_ac_get_prop(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    let mut ret: c_int = 0;
    uint8_t status;
    switch (psp) {
    case POWER_SUPPLY_PROP_ONLINE:
    ret = olpc_ec_cmd(EC_BAT_STATUS, core::ptr::null_mut(), 0, &status, 1);
    if (ret)
    return ret;
    val.intval = !!(status & BAT_STAT_AC);
    break;
    default:
    ret = -EINVAL;
    break;
    }
    return ret;
    }
    static enum power_supply_property olpc_ac_props[] = {
    POWER_SUPPLY_PROP_ONLINE,
    };
    static const struct power_supply_desc olpc_ac_desc = {
    .name = "olpc_ac",
    .type = POWER_SUPPLY_TYPE_MAINS,
    .properties = olpc_ac_props,
    .num_properties = ARRAY_SIZE(olpc_ac_props),
    .get_property = olpc_ac_get_prop,
    };
    static int olpc_bat_get_status(struct olpc_battery_data *data,
    union power_supply_propval *val, uint8_t ec_byte)
    {
    if (data.new_proto) {
    if (ec_byte & (BAT_STAT_CHARGING | BAT_STAT_TRICKLE))
    val.intval = POWER_SUPPLY_STATUS_CHARGING;
#[no_mangle]
pub unsafe extern "C" fn if(BAT_STAT_DISCHARGING: ec_byte &) -> else {
    else if (ec_byte & BAT_STAT_DISCHARGING)
    val.intval = POWER_SUPPLY_STATUS_DISCHARGING;
#[no_mangle]
pub unsafe extern "C" fn if(BAT_STAT_FULL: ec_byte &) -> else {
    else if (ec_byte & BAT_STAT_FULL)
    val.intval = POWER_SUPPLY_STATUS_FULL;
    else /* er,... */
    val.intval = POWER_SUPPLY_STATUS_NOT_CHARGING;
    } else {
// Older EC didn't report charge/discharge bits
    if (!(ec_byte & BAT_STAT_AC)) /* No AC means discharging */
    val.intval = POWER_SUPPLY_STATUS_DISCHARGING;
#[no_mangle]
pub unsafe extern "C" fn if(BAT_STAT_FULL: ec_byte &) -> else {
    else if (ec_byte & BAT_STAT_FULL)
    val.intval = POWER_SUPPLY_STATUS_FULL;
    else /* Not _necessarily_ true but EC doesn't tell all yet */
    val.intval = POWER_SUPPLY_STATUS_CHARGING;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn olpc_bat_get_health(val: *mut union power_supply_propval) -> c_int {
    static int olpc_bat_get_health(union power_supply_propval *val)
    {
    uint8_t ec_byte;
    int ret;
    ret = olpc_ec_cmd(EC_BAT_ERRCODE, core::ptr::null_mut(), 0, &ec_byte, 1);
    if (ret)
    return ret;
    switch (ec_byte) {
    case 0:
    val.intval = POWER_SUPPLY_HEALTH_GOOD;
    break;
    case BAT_ERR_OVERTEMP:
    val.intval = POWER_SUPPLY_HEALTH_OVERHEAT;
    break;
    case BAT_ERR_OVERVOLTAGE:
    val.intval = POWER_SUPPLY_HEALTH_OVERVOLTAGE;
    break;
    case BAT_ERR_INFOFAIL:
    case BAT_ERR_OUT_OF_CONTROL:
    case BAT_ERR_ID_FAIL:
    case BAT_ERR_ACR_FAIL:
    val.intval = POWER_SUPPLY_HEALTH_UNSPEC_FAILURE;
    break;
    default:
// Eep. We don't know this failure code
    ret = -EIO;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn olpc_bat_get_mfr(val: *mut union power_supply_propval) -> c_int {
    static int olpc_bat_get_mfr(union power_supply_propval *val)
    {
    uint8_t ec_byte;
    int ret;
    ec_byte = BAT_ADDR_MFR_TYPE;
    ret = olpc_ec_cmd(EC_BAT_EEPROM, &ec_byte, 1, &ec_byte, 1);
    if (ret)
    return ret;
    switch (ec_byte >> 4) {
    case 1:
    val.strval = "Gold Peak";
    break;
    case 2:
    val.strval = "BYD";
    break;
    default:
    val.strval = "Unknown";
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn olpc_bat_get_tech(val: *mut union power_supply_propval) -> c_int {
    static int olpc_bat_get_tech(union power_supply_propval *val)
    {
    uint8_t ec_byte;
    int ret;
    ec_byte = BAT_ADDR_MFR_TYPE;
    ret = olpc_ec_cmd(EC_BAT_EEPROM, &ec_byte, 1, &ec_byte, 1);
    if (ret)
    return ret;
    switch (ec_byte & 0xf) {
    case 1:
    val.intval = POWER_SUPPLY_TECHNOLOGY_NiMH;
    break;
    case 2:
    val.intval = POWER_SUPPLY_TECHNOLOGY_LiFe;
    break;
    default:
    val.intval = POWER_SUPPLY_TECHNOLOGY_UNKNOWN;
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn olpc_bat_get_charge_full_design(val: *mut union power_supply_propval) -> c_int {
    static int olpc_bat_get_charge_full_design(union power_supply_propval *val)
    {
    uint8_t ec_byte;
    union power_supply_propval tech;
    int ret, mfr;
    ret = olpc_bat_get_tech(&tech);
    if (ret)
    return ret;
    ec_byte = BAT_ADDR_MFR_TYPE;
    ret = olpc_ec_cmd(EC_BAT_EEPROM, &ec_byte, 1, &ec_byte, 1);
    if (ret)
    return ret;
    mfr = ec_byte >> 4;
    switch (tech.intval) {
    case POWER_SUPPLY_TECHNOLOGY_NiMH:
    switch (mfr) {
    case 1: /* Gold Peak */
    val.intval = 3000000*.8;
    break;
    default:
    return -EIO;
    }
    break;
    case POWER_SUPPLY_TECHNOLOGY_LiFe:
    switch (mfr) {
    case 1: /* Gold Peak, fall through */
    case 2: /* BYD */
    val.intval = 2800000;
    break;
    default:
    return -EIO;
    }
    break;
    default:
    return -EIO;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn olpc_bat_get_charge_now(val: *mut union power_supply_propval) -> c_int {
    static int olpc_bat_get_charge_now(union power_supply_propval *val)
    {
    uint8_t soc;
    union power_supply_propval full;
    int ret;
    ret = olpc_ec_cmd(EC_BAT_SOC, core::ptr::null_mut(), 0, &soc, 1);
    if (ret)
    return ret;
    ret = olpc_bat_get_charge_full_design(&full);
    if (ret)
    return ret;
    val.intval = soc * (full.intval / 100);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn olpc_bat_get_voltage_max_design(val: *mut union power_supply_propval) -> c_int {
    static int olpc_bat_get_voltage_max_design(union power_supply_propval *val)
    {
    uint8_t ec_byte;
    union power_supply_propval tech;
    int mfr;
    int ret;
    ret = olpc_bat_get_tech(&tech);
    if (ret)
    return ret;
    ec_byte = BAT_ADDR_MFR_TYPE;
    ret = olpc_ec_cmd(EC_BAT_EEPROM, &ec_byte, 1, &ec_byte, 1);
    if (ret)
    return ret;
    mfr = ec_byte >> 4;
    switch (tech.intval) {
    case POWER_SUPPLY_TECHNOLOGY_NiMH:
    switch (mfr) {
    case 1: /* Gold Peak */
    val.intval = 6000000;
    break;
    default:
    return -EIO;
    }
    break;
    case POWER_SUPPLY_TECHNOLOGY_LiFe:
    switch (mfr) {
    case 1: /* Gold Peak */
    val.intval = 6400000;
    break;
    case 2: /* BYD */
    val.intval = 6500000;
    break;
    default:
    return -EIO;
    }
    break;
    default:
    return -EIO;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ecword_to_cpu(data: *mut olpc_battery_data, ec_word: u16) -> u16 {
    static u16 ecword_to_cpu(struct olpc_battery_data *data, u16 ec_word)
    {
    if (data.little_endian)
    return le16_to_cpu(( __le16)ec_word);
    else
    return be16_to_cpu(( __be16)ec_word);
    }
//
// Battery properties
//
    static int olpc_bat_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct olpc_battery_data *data = power_supply_get_drvdata(psy);
    let mut ret: c_int = 0;
    u16 ec_word;
    uint8_t ec_byte;
    __be64 ser_buf;
    ret = olpc_ec_cmd(EC_BAT_STATUS, core::ptr::null_mut(), 0, &ec_byte, 1);
    if (ret)
    return ret;
// Theoretically there's a race here -- the battery could be
    removed immediately after we check whether it's present, and
    then we query for some other property of the now-absent battery.
    It doesn't matter though -- the EC will return the last-known
    information, and it's as if we just ran that _little_ bit faster
    and managed to read it out before the battery went away. */
    if (!(ec_byte & (BAT_STAT_PRESENT | BAT_STAT_TRICKLE)) &&
    psp != POWER_SUPPLY_PROP_PRESENT)
    return -ENODEV;
    switch (psp) {
    case POWER_SUPPLY_PROP_STATUS:
    ret = olpc_bat_get_status(data, val, ec_byte);
    if (ret)
    return ret;
    break;
    case POWER_SUPPLY_PROP_CHARGE_TYPE:
    if (ec_byte & BAT_STAT_TRICKLE)
    val.intval = POWER_SUPPLY_CHARGE_TYPE_TRICKLE;
#[no_mangle]
pub unsafe extern "C" fn if(BAT_STAT_CHARGING: ec_byte &) -> else {
    else if (ec_byte & BAT_STAT_CHARGING)
    val.intval = POWER_SUPPLY_CHARGE_TYPE_FAST;
    else
    val.intval = POWER_SUPPLY_CHARGE_TYPE_NONE;
    break;
    case POWER_SUPPLY_PROP_PRESENT:
    val.intval = !!(ec_byte & (BAT_STAT_PRESENT |
    BAT_STAT_TRICKLE));
    break;
    case POWER_SUPPLY_PROP_HEALTH:
    if (ec_byte & BAT_STAT_DESTROY)
    val.intval = POWER_SUPPLY_HEALTH_DEAD;
    else {
    ret = olpc_bat_get_health(val);
    if (ret)
    return ret;
    }
    break;
    case POWER_SUPPLY_PROP_MANUFACTURER:
    ret = olpc_bat_get_mfr(val);
    if (ret)
    return ret;
    break;
    case POWER_SUPPLY_PROP_TECHNOLOGY:
    ret = olpc_bat_get_tech(val);
    if (ret)
    return ret;
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_AVG:
    case POWER_SUPPLY_PROP_VOLTAGE_NOW:
    ret = olpc_ec_cmd(EC_BAT_VOLTAGE, core::ptr::null_mut(), 0, (void *)&ec_word, 2);
    if (ret)
    return ret;
    val.intval = ecword_to_cpu(data, ec_word) * 9760L / 32;
    break;
    case POWER_SUPPLY_PROP_CURRENT_AVG:
    case POWER_SUPPLY_PROP_CURRENT_NOW:
    ret = olpc_ec_cmd(EC_BAT_CURRENT, core::ptr::null_mut(), 0, (void *)&ec_word, 2);
    if (ret)
    return ret;
    val.intval = ecword_to_cpu(data, ec_word) * 15625L / 120;
    break;
    case POWER_SUPPLY_PROP_CAPACITY:
    ret = olpc_ec_cmd(EC_BAT_SOC, core::ptr::null_mut(), 0, &ec_byte, 1);
    if (ret)
    return ret;
    val.intval = ec_byte;
    break;
    case POWER_SUPPLY_PROP_CAPACITY_LEVEL:
    if (ec_byte & BAT_STAT_FULL)
    val.intval = POWER_SUPPLY_CAPACITY_LEVEL_FULL;
#[no_mangle]
pub unsafe extern "C" fn if(BAT_STAT_LOW: ec_byte &) -> else {
    else if (ec_byte & BAT_STAT_LOW)
    val.intval = POWER_SUPPLY_CAPACITY_LEVEL_LOW;
    else
    val.intval = POWER_SUPPLY_CAPACITY_LEVEL_NORMAL;
    break;
    case POWER_SUPPLY_PROP_CHARGE_FULL_DESIGN:
    ret = olpc_bat_get_charge_full_design(val);
    if (ret)
    return ret;
    break;
    case POWER_SUPPLY_PROP_CHARGE_NOW:
    ret = olpc_bat_get_charge_now(val);
    if (ret)
    return ret;
    break;
    case POWER_SUPPLY_PROP_TEMP:
    ret = olpc_ec_cmd(EC_BAT_TEMP, core::ptr::null_mut(), 0, (void *)&ec_word, 2);
    if (ret)
    return ret;
    val.intval = ecword_to_cpu(data, ec_word) * 10 / 256;
    break;
    case POWER_SUPPLY_PROP_TEMP_AMBIENT:
    ret = olpc_ec_cmd(EC_AMB_TEMP, core::ptr::null_mut(), 0, (void *)&ec_word, 2);
    if (ret)
    return ret;
    val.intval = (int)ecword_to_cpu(data, ec_word) * 10 / 256;
    break;
    case POWER_SUPPLY_PROP_CHARGE_COUNTER:
    ret = olpc_ec_cmd(EC_BAT_ACR, core::ptr::null_mut(), 0, (void *)&ec_word, 2);
    if (ret)
    return ret;
    val.intval = ecword_to_cpu(data, ec_word) * 6250 / 15;
    break;
    case POWER_SUPPLY_PROP_SERIAL_NUMBER:
    ret = olpc_ec_cmd(EC_BAT_SERIAL, core::ptr::null_mut(), 0, (void *)&ser_buf, 8);
    if (ret)
    return ret;
    sprintf(data.bat_serial, "%016llx", (long long)be64_to_cpu(ser_buf));
    val.strval = data.bat_serial;
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_MAX_DESIGN:
    ret = olpc_bat_get_voltage_max_design(val);
    if (ret)
    return ret;
    break;
    default:
    ret = -EINVAL;
    break;
    }
    return ret;
    }
    static enum power_supply_property olpc_xo1_bat_props[] = {
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_CHARGE_TYPE,
    POWER_SUPPLY_PROP_PRESENT,
    POWER_SUPPLY_PROP_HEALTH,
    POWER_SUPPLY_PROP_TECHNOLOGY,
    POWER_SUPPLY_PROP_VOLTAGE_AVG,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    POWER_SUPPLY_PROP_CURRENT_AVG,
    POWER_SUPPLY_PROP_CURRENT_NOW,
    POWER_SUPPLY_PROP_CAPACITY,
    POWER_SUPPLY_PROP_CAPACITY_LEVEL,
    POWER_SUPPLY_PROP_CHARGE_FULL_DESIGN,
    POWER_SUPPLY_PROP_CHARGE_NOW,
    POWER_SUPPLY_PROP_TEMP,
    POWER_SUPPLY_PROP_TEMP_AMBIENT,
    POWER_SUPPLY_PROP_MANUFACTURER,
    POWER_SUPPLY_PROP_SERIAL_NUMBER,
    POWER_SUPPLY_PROP_CHARGE_COUNTER,
    POWER_SUPPLY_PROP_VOLTAGE_MAX_DESIGN,
    };
// XO-1.5 does not have ambient temperature property
    static enum power_supply_property olpc_xo15_bat_props[] = {
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_CHARGE_TYPE,
    POWER_SUPPLY_PROP_PRESENT,
    POWER_SUPPLY_PROP_HEALTH,
    POWER_SUPPLY_PROP_TECHNOLOGY,
    POWER_SUPPLY_PROP_VOLTAGE_AVG,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    POWER_SUPPLY_PROP_CURRENT_AVG,
    POWER_SUPPLY_PROP_CURRENT_NOW,
    POWER_SUPPLY_PROP_CAPACITY,
    POWER_SUPPLY_PROP_CAPACITY_LEVEL,
    POWER_SUPPLY_PROP_CHARGE_FULL_DESIGN,
    POWER_SUPPLY_PROP_CHARGE_NOW,
    POWER_SUPPLY_PROP_TEMP,
    POWER_SUPPLY_PROP_MANUFACTURER,
    POWER_SUPPLY_PROP_SERIAL_NUMBER,
    POWER_SUPPLY_PROP_CHARGE_COUNTER,
    POWER_SUPPLY_PROP_VOLTAGE_MAX_DESIGN,
    };
// EEPROM reading goes completely around the power_supply API, sadly
pub const EEPROM_START: c_uint = 0x20;
pub const EEPROM_END: c_uint = 0x80;

    static ssize_t olpc_bat_eeprom_read(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *attr, char *buf, loff_t off, size_t count)
    {
    uint8_t ec_byte;
    int ret;
    int i;
    for (i = 0; i < count; i++) {
    ec_byte = EEPROM_START + off + i;
    ret = olpc_ec_cmd(EC_BAT_EEPROM, &ec_byte, 1, &buf[i], 1);
    if (ret) {
    pr_err("olpc-battery: "
    "EC_BAT_EEPROM cmd @ 0x%x failed - %d!\n",
    ec_byte, ret);
    return -EIO;
    }
    }
    return count;
    }
    static const struct bin_attribute olpc_bat_eeprom = {
    .attr = {
    .name = "eeprom",
    .mode = S_IRUGO,
    },
    .size = EEPROM_SIZE,
    .read = olpc_bat_eeprom_read,
    };
// Allow userspace to see the specific error value pulled from the EC
    static ssize_t olpc_bat_error_read(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    uint8_t ec_byte;
    ssize_t ret;
    ret = olpc_ec_cmd(EC_BAT_ERRCODE, core::ptr::null_mut(), 0, &ec_byte, 1);
    if (ret < 0)
    return ret;
    return sysfs_emit(buf, "%d\n", ec_byte);
    }
    static struct device_attribute olpc_bat_error = {
    .attr = {
    .name = "error",
    .mode = S_IRUGO,
    },
    .show = olpc_bat_error_read,
    };
    static struct attribute *olpc_bat_sysfs_attrs[] = {
    &olpc_bat_error.attr,
    core::ptr::null_mut()
    };
    static const struct bin_attribute *const olpc_bat_sysfs_bin_attrs[] = {
    &olpc_bat_eeprom,
    core::ptr::null_mut()
    };
    static const struct attribute_group olpc_bat_sysfs_group = {
    .attrs = olpc_bat_sysfs_attrs,
    .bin_attrs = olpc_bat_sysfs_bin_attrs,
    };
    static const struct attribute_group *olpc_bat_sysfs_groups[] = {
    &olpc_bat_sysfs_group,
    core::ptr::null_mut()
    };
//
// Initialisation
//
    static struct power_supply_desc olpc_bat_desc = {
    .name = "olpc_battery",
    .get_property = olpc_bat_get_property,
    .use_for_apm = 1,
    };
    static int olpc_battery_suspend(struct platform_device *pdev,
    pm_message_t state)
    {
    struct olpc_battery_data *data = platform_get_drvdata(pdev);
    if (device_may_wakeup(&data.olpc_ac.dev))
    olpc_ec_wakeup_set(EC_SCI_SRC_ACPWR);
    else
    olpc_ec_wakeup_clear(EC_SCI_SRC_ACPWR);
    if (device_may_wakeup(&data.olpc_bat.dev))
    olpc_ec_wakeup_set(EC_SCI_SRC_BATTERY | EC_SCI_SRC_BATSOC
    | EC_SCI_SRC_BATERR);
    else
    olpc_ec_wakeup_clear(EC_SCI_SRC_BATTERY | EC_SCI_SRC_BATSOC
    | EC_SCI_SRC_BATERR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn olpc_battery_probe(pdev: *mut platform_device) -> c_int {
    static int olpc_battery_probe(struct platform_device *pdev)
    {
    let mut bat_psy_cfg: power_supply_config = {};
    let mut ac_psy_cfg: power_supply_config = {};
    struct olpc_battery_data *data;
    struct device_node *np;
    uint8_t status;
    uint8_t ecver;
    int ret;
    data = devm_kzalloc(&pdev.dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    platform_set_drvdata(pdev, data);
// See if the EC is already there and get the EC revision
    ret = olpc_ec_cmd(EC_FIRMWARE_REV, core::ptr::null_mut(), 0, &ecver, 1);
    if (ret)
    return ret;
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "olpc,xo1.75-ec");
    if (np) {
    of_node_put(np);
// XO 1.75
    data.new_proto = true;
    data.little_endian = true;
    } else if (ecver > 0x44) {
// XO 1 or 1.5 with a new EC firmware.
    data.new_proto = true;
    } else if (ecver < 0x44) {
//
// We've seen a number of EC protocol changes; this driver
// requires the latest EC protocol, supported by 0x44 and above.
//
    printk(KERN_NOTICE "OLPC EC version 0x%02x too old for "
    "battery driver.\n", ecver);
    return -ENXIO;
    }
    ret = olpc_ec_cmd(EC_BAT_STATUS, core::ptr::null_mut(), 0, &status, 1);
    if (ret)
    return ret;
// Ignore the status. It doesn't actually matter
    ac_psy_cfg.fwnode = dev_fwnode(&pdev.dev);
    ac_psy_cfg.drv_data = data;
    data.olpc_ac = devm_power_supply_register(&pdev.dev, &olpc_ac_desc,
    &ac_psy_cfg);
    if (IS_ERR(data.olpc_ac))
    return PTR_ERR(data.olpc_ac);
    if (of_device_is_compatible(pdev.dev.of_node, "olpc,xo1.5-battery")) {
// XO-1.5
    olpc_bat_desc.properties = olpc_xo15_bat_props;
    olpc_bat_desc.num_properties = ARRAY_SIZE(olpc_xo15_bat_props);
    } else {
// XO-1
    olpc_bat_desc.properties = olpc_xo1_bat_props;
    olpc_bat_desc.num_properties = ARRAY_SIZE(olpc_xo1_bat_props);
    }
    bat_psy_cfg.fwnode = dev_fwnode(&pdev.dev);
    bat_psy_cfg.drv_data = data;
    bat_psy_cfg.attr_grp = olpc_bat_sysfs_groups;
    data.olpc_bat = devm_power_supply_register(&pdev.dev, &olpc_bat_desc,
    &bat_psy_cfg);
    if (IS_ERR(data.olpc_bat))
    return PTR_ERR(data.olpc_bat);
    if (olpc_ec_wakeup_available()) {
    device_set_wakeup_capable(&data.olpc_ac.dev, true);
    device_set_wakeup_capable(&data.olpc_bat.dev, true);
    }
    return 0;
    }
    static const struct of_device_id olpc_battery_ids[] = {
    { .compatible = "olpc,xo1-battery" },
    { .compatible = "olpc,xo1.5-battery" },
    {}
    };
    MODULE_DEVICE_TABLE(of, olpc_battery_ids);
    static struct platform_driver olpc_battery_driver = {
    .driver = {
    .name = "olpc-battery",
    .of_match_table = olpc_battery_ids,
    },
    .probe = olpc_battery_probe,
    .suspend = olpc_battery_suspend,
    };
    module_platform_driver(olpc_battery_driver);
    MODULE_AUTHOR("David Woodhouse <dwmw2@infradead.org>");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Battery driver for One Laptop Per Child 'XO' machine");
