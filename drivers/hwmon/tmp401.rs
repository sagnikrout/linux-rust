//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/tmp401.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
// tmp401.c
//
// Copyright (C) 2007,2008 Hans de Goede <hdegoede@redhat.com>
// Preliminary tmp411 support by:
// Gabriel Konat, Sander Leget, Wouter Willems
// Copyright (C) 2009 Andre Prendel <andre.prendel@gmx.de>
//
// Cleanup and support for TMP431 and TMP432 by Guenter Roeck
// Copyright (c) 2013 Guenter Roeck <linux@roeck-us.net>
//
// Driver for the Texas Instruments TMP401 SMBUS temperature sensor IC.
//
// Note this IC is in some aspect similar to the LM90, but it has quite a
// few differences too, for example the local temp has a higher resolution
// and thus has 16 bits registers for its value and limit instead of 8 bits.
//

// Addresses to scan
    static const unsigned short normal_i2c[] = { 0x48, 0x49, 0x4a, 0x4c, 0x4d,
    0x4e, 0x4f, I2C_CLIENT_END };
    enum chips { tmp401, tmp411, tmp431, tmp432, tmp435 };
//
// The TMP401 registers, note some registers have different addresses for
// reading and writing
//
pub const TMP401_STATUS: c_uint = 0x02;
pub const TMP401_CONFIG: c_uint = 0x03;
pub const TMP401_CONVERSION_RATE: c_uint = 0x04;
pub const TMP4XX_N_FACTOR_REG: c_uint = 0x18;
pub const TMP43X_BETA_RANGE: c_uint = 0x25;
pub const TMP401_TEMP_CRIT_HYST: c_uint = 0x21;
pub const TMP401_MANUFACTURER_ID_REG: c_uint = 0xFE;
pub const TMP401_DEVICE_ID_REG: c_uint = 0xFF;
    static const u8 TMP401_TEMP_MSB[7][3] = {
    { 0x00, 0x01, 0x23 },	/* temp */
    { 0x06, 0x08, 0x16 },	/* low limit */
    { 0x05, 0x07, 0x15 },	/* high limit */
    { 0x20, 0x19, 0x1a },	/* therm (crit) limit */
    { 0x30, 0x34, 0x00 },	/* lowest */
    { 0x32, 0xf6, 0x00 },	/* highest */
    };
// [0] = fault, [1] = low, [2] = high, [3] = therm/crit
    static const u8 TMP432_STATUS_REG[] = {
    0x1b, 0x36, 0x35, 0x37 };
// Flags

// On TMP432, each status has its own register

// Manufacturer / Device ID's
pub const TMP401_MANUFACTURER_ID: c_uint = 0x55;
pub const TMP401_DEVICE_ID: c_uint = 0x11;
pub const TMP411A_DEVICE_ID: c_uint = 0x12;
pub const TMP411B_DEVICE_ID: c_uint = 0x13;
pub const TMP411C_DEVICE_ID: c_uint = 0x10;
pub const TMP431_DEVICE_ID: c_uint = 0x31;
pub const TMP432_DEVICE_ID: c_uint = 0x32;
pub const TMP435_DEVICE_ID: c_uint = 0x35;
//
// Driver data (common to all clients)
//
    static const struct i2c_device_id tmp401_id[] = {
    { .name = "tmp401", .driver_data = tmp401 },
    { .name = "tmp411", .driver_data = tmp411 },
    { .name = "tmp431", .driver_data = tmp431 },
    { .name = "tmp432", .driver_data = tmp432 },
    { .name = "tmp435", .driver_data = tmp435 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, tmp401_id);
//
// Client data (each client gets its own)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmp401_data {
    pub client: *mut i2c_client,
    pub regmap: *mut regmap,
    pub kind: enum chips,
    pub extended_range: bool,
// hwmon API configuration data
    pub chip_channel_config: [u32; 4],
    pub chip_info: hwmon_channel_info,
    pub temp_channel_config: [u32; 4],
    pub temp_info: hwmon_channel_info,
    pub info: [*const hwmon_channel_info; 3],
    pub chip: hwmon_chip_info,
}

// regmap
#[no_mangle]
unsafe extern "C" fn tmp401_regmap_is_volatile(dev: *mut device, reg: c_uint) -> bool {
    static bool tmp401_regmap_is_volatile(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case 0:			/* local temp msb */
    case 1:			/* remote temp msb */
    case 2:			/* status */
    case 0x10:		/* remote temp lsb */
    case 0x15:		/* local temp lsb */
    case 0x1b:		/* status (tmp432) */
    case 0x23 ... 0x24:	/* remote temp 2 msb / lsb */
    case 0x30 ... 0x37:	/* lowest/highest temp; status (tmp432) */
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn tmp401_reg_read(context: *mut c_void, reg: c_uint, val: *mut c_uint) -> c_int {
    static int tmp401_reg_read(void *context, unsigned int reg, unsigned int *val)
    {
    struct tmp401_data *data = context;
    struct i2c_client *client = data.client;
    int regval;
    switch (reg) {
    case 0:			/* local temp msb */
    case 1:			/* remote temp msb */
    case 5:			/* local temp high limit msb */
    case 6:			/* local temp low limit msb */
    case 7:			/* remote temp ligh limit msb */
    case 8:			/* remote temp low limit msb */
    case 0x15:		/* remote temp 2 high limit msb */
    case 0x16:		/* remote temp 2 low limit msb */
    case 0x23:		/* remote temp 2 msb */
    case 0x30:		/* local temp minimum, tmp411 */
    case 0x32:		/* local temp maximum, tmp411 */
    case 0x34:		/* remote temp minimum, tmp411 */
    case 0xf6:		/* remote temp maximum, tmp411 (really 0x36) */
// work around register overlap between TMP411 and TMP432
    if (reg == 0xf6)
    reg = 0x36;
    regval = i2c_smbus_read_word_swapped(client, reg);
    if (regval < 0)
    return regval;
// val = regval;
    break;
    case 0x19:		/* critical limits, 8-bit registers */
    case 0x1a:
    case 0x20:
    regval = i2c_smbus_read_byte_data(client, reg);
    if (regval < 0)
    return regval;
// val = regval << 8;
    break;
    case 0x1b:
    case 0x35 ... 0x37:
    if (data.kind == tmp432) {
    regval = i2c_smbus_read_byte_data(client, reg);
    if (regval < 0)
    return regval;
// val = regval;
    break;
    }
// simulate TMP432 status registers
    regval = i2c_smbus_read_byte_data(client, TMP401_STATUS);
    if (regval < 0)
    return regval;
// val = 0;
    switch (reg) {
    case 0x1b:	/* open / fault */
    if (regval & TMP401_STATUS_REMOTE_OPEN)
// val |= BIT(1);
    break;
    case 0x35:	/* high limit */
    if (regval & TMP401_STATUS_LOCAL_HIGH)
// val |= BIT(0);
    if (regval & TMP401_STATUS_REMOTE_HIGH)
// val |= BIT(1);
    break;
    case 0x36:	/* low limit */
    if (regval & TMP401_STATUS_LOCAL_LOW)
// val |= BIT(0);
    if (regval & TMP401_STATUS_REMOTE_LOW)
// val |= BIT(1);
    break;
    case 0x37:	/* therm / crit limit */
    if (regval & TMP401_STATUS_LOCAL_CRIT)
// val |= BIT(0);
    if (regval & TMP401_STATUS_REMOTE_CRIT)
// val |= BIT(1);
    break;
    }
    break;
    default:
    regval = i2c_smbus_read_byte_data(client, reg);
    if (regval < 0)
    return regval;
// val = regval;
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tmp401_reg_write(context: *mut c_void, reg: c_uint, val: c_uint) -> c_int {
    static int tmp401_reg_write(void *context, unsigned int reg, unsigned int val)
    {
    struct tmp401_data *data = context;
    struct i2c_client *client = data.client;
    switch (reg) {
    case 0x05:		/* local temp high limit msb */
    case 0x06:		/* local temp low limit msb */
    case 0x07:		/* remote temp ligh limit msb */
    case 0x08:		/* remote temp low limit msb */
    reg += 6;	/* adjust for register write address */
    fallthrough;
    case 0x15:		/* remote temp 2 high limit msb */
    case 0x16:		/* remote temp 2 low limit msb */
    return i2c_smbus_write_word_swapped(client, reg, val);
    case 0x19:		/* critical limits, 8-bit registers */
    case 0x1a:
    case 0x20:
    return i2c_smbus_write_byte_data(client, reg, val >> 8);
    case TMP401_CONVERSION_RATE:
    case TMP401_CONFIG:
    reg += 6;	/* adjust for register write address */
    fallthrough;
    default:
    return i2c_smbus_write_byte_data(client, reg, val);
    }
    }
    static const struct regmap_config tmp401_regmap_config = {
    .reg_bits = 8,
    .val_bits = 16,
    .cache_type = REGCACHE_MAPLE,
    .volatile_reg = tmp401_regmap_is_volatile,
    .reg_read = tmp401_reg_read,
    .reg_write = tmp401_reg_write,
    };
// temperature conversion
#[no_mangle]
unsafe extern "C" fn tmp401_register_to_temp(reg: u16, extended: bool) -> c_int {
    static int tmp401_register_to_temp(u16 reg, bool extended)
    {
    let mut temp: c_int = reg;
    if (extended)
    temp -= 64 * 256;
    return DIV_ROUND_CLOSEST(temp * 125, 32);
    }
#[no_mangle]
unsafe extern "C" fn tmp401_temp_to_register(temp: c_long, extended: bool, zbits: c_int) -> u16 {
    static u16 tmp401_temp_to_register(long temp, bool extended, int zbits)
    {
    if (extended) {
    temp = clamp_val(temp, -64000, 191000);
    temp += 64000;
    } else {
    temp = clamp_val(temp, 0, 127000);
    }
    return DIV_ROUND_CLOSEST(temp * (1 << (8 - zbits)), 1000) << zbits;
    }
// hwmon API functions
    static const u8 tmp401_temp_reg_index[] = {
    [hwmon_temp_input] = 0,
    [hwmon_temp_min] = 1,
    [hwmon_temp_max] = 2,
    [hwmon_temp_crit] = 3,
    [hwmon_temp_lowest] = 4,
    [hwmon_temp_highest] = 5,
    };
    static const u8 tmp401_status_reg_index[] = {
    [hwmon_temp_fault] = 0,
    [hwmon_temp_min_alarm] = 1,
    [hwmon_temp_max_alarm] = 2,
    [hwmon_temp_crit_alarm] = 3,
    };
#[no_mangle]
unsafe extern "C" fn tmp401_temp_read(dev: *mut device, attr: u32, channel: c_int, val: *mut c_long) -> c_int {
    static int tmp401_temp_read(struct device *dev, u32 attr, int channel, long *val)
    {
    struct tmp401_data *data = dev_get_drvdata(dev);
    struct regmap *regmap = data.regmap;
    unsigned int regs[2] = { TMP401_TEMP_MSB[3][channel], TMP401_TEMP_CRIT_HYST };
    unsigned int regval;
    u16 regvals[2];
    int reg, ret;
    switch (attr) {
    case hwmon_temp_input:
    case hwmon_temp_min:
    case hwmon_temp_max:
    case hwmon_temp_crit:
    case hwmon_temp_lowest:
    case hwmon_temp_highest:
    reg = TMP401_TEMP_MSB[tmp401_temp_reg_index[attr]][channel];
    ret = regmap_read(regmap, reg, &regval);
    if (ret < 0)
    return ret;
// val = tmp401_register_to_temp(regval, data->extended_range);
    break;
    case hwmon_temp_crit_hyst:
    ret = regmap_multi_reg_read(regmap, regs, regvals, 2);
    if (ret < 0)
    return ret;
// val = tmp401_register_to_temp(regvals[0], data->extended_range) -
    (regvals[1] * 1000);
    break;
    case hwmon_temp_fault:
    case hwmon_temp_min_alarm:
    case hwmon_temp_max_alarm:
    case hwmon_temp_crit_alarm:
    reg = TMP432_STATUS_REG[tmp401_status_reg_index[attr]];
    ret = regmap_read(regmap, reg, &regval);
    if (ret < 0)
    return ret;
// val = !!(regval & BIT(channel));
    break;
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
    static int tmp401_temp_write(struct device *dev, u32 attr, int channel,
    long val)
    {
    struct tmp401_data *data = dev_get_drvdata(dev);
    struct regmap *regmap = data.regmap;
    unsigned int regval;
    int reg, ret, temp;
    switch (attr) {
    case hwmon_temp_min:
    case hwmon_temp_max:
    case hwmon_temp_crit:
    reg = TMP401_TEMP_MSB[tmp401_temp_reg_index[attr]][channel];
    regval = tmp401_temp_to_register(val, data.extended_range,
    attr == hwmon_temp_crit ? 8 : 4);
    ret = regmap_write(regmap, reg, regval);
    break;
    case hwmon_temp_crit_hyst:
    if (data.extended_range)
    val = clamp_val(val, -64000, 191000);
    else
    val = clamp_val(val, 0, 127000);
    reg = TMP401_TEMP_MSB[3][channel];
    ret = regmap_read(regmap, reg, &regval);
    if (ret < 0)
    break;
    temp = tmp401_register_to_temp(regval, data.extended_range);
    val = clamp_val(val, temp - 255000, temp);
    regval = ((temp - val) + 500) / 1000;
    ret = regmap_write(regmap, TMP401_TEMP_CRIT_HYST, regval);
    break;
    default:
    ret = -EOPNOTSUPP;
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tmp401_chip_read(dev: *mut device, attr: u32, channel: c_int, val: *mut c_long) -> c_int {
    static int tmp401_chip_read(struct device *dev, u32 attr, int channel, long *val)
    {
    struct tmp401_data *data = dev_get_drvdata(dev);
    u32 regval;
    int ret;
    switch (attr) {
    case hwmon_chip_update_interval:
    ret = regmap_read(data.regmap, TMP401_CONVERSION_RATE, &regval);
    if (ret < 0)
    return ret;
// val = (1 << (7 - min(regval, 7))) * 125;
    break;
    case hwmon_chip_temp_reset_history:
// val = 0;
    break;
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tmp401_set_convrate(regmap: *mut regmap, val: c_long) -> c_int {
    static int tmp401_set_convrate(struct regmap *regmap, long val)
    {
    int rate;
//
// For valid rates, interval can be calculated as
// interval = (1 << (7 - rate)) * 125;
// Rounded rate is therefore
// rate = 7 - __fls(interval * 4 / (125 * 3));
// Use clamp_val() to avoid overflows, and to ensure valid input
// for __fls.
//
    val = clamp_val(val, 125, 16000);
    rate = 7 - __fls(val * 4 / (125 * 3));
    return regmap_write(regmap, TMP401_CONVERSION_RATE, rate);
    }
#[no_mangle]
unsafe extern "C" fn tmp401_chip_write(dev: *mut device, attr: u32, channel: c_int, val: c_long) -> c_int {
    static int tmp401_chip_write(struct device *dev, u32 attr, int channel, long val)
    {
    struct tmp401_data *data = dev_get_drvdata(dev);
    struct regmap *regmap = data.regmap;
    int err;
    switch (attr) {
    case hwmon_chip_update_interval:
    err = tmp401_set_convrate(regmap, val);
    break;
    case hwmon_chip_temp_reset_history:
    if (val != 1) {
    err = -EINVAL;
    break;
    }
//
// Reset history by writing any value to any of the
// minimum/maximum registers (0x30-0x37).
//
    err = regmap_write(regmap, 0x30, 0);
    break;
    default:
    err = -EOPNOTSUPP;
    break;
    }
    return err;
    }
    static int tmp401_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    switch (type) {
    case hwmon_chip:
    return tmp401_chip_read(dev, attr, channel, val);
    case hwmon_temp:
    return tmp401_temp_read(dev, attr, channel, val);
    default:
    return -EOPNOTSUPP;
    }
    }
    static int tmp401_write(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long val)
    {
    switch (type) {
    case hwmon_chip:
    return tmp401_chip_write(dev, attr, channel, val);
    case hwmon_temp:
    return tmp401_temp_write(dev, attr, channel, val);
    default:
    return -EOPNOTSUPP;
    }
    }
    static umode_t tmp401_is_visible(const void *data, enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    switch (type) {
    case hwmon_chip:
    switch (attr) {
    case hwmon_chip_update_interval:
    case hwmon_chip_temp_reset_history:
    return 0644;
    default:
    break;
    }
    break;
    case hwmon_temp:
    switch (attr) {
    case hwmon_temp_input:
    case hwmon_temp_min_alarm:
    case hwmon_temp_max_alarm:
    case hwmon_temp_crit_alarm:
    case hwmon_temp_fault:
    case hwmon_temp_lowest:
    case hwmon_temp_highest:
    return 0444;
    case hwmon_temp_min:
    case hwmon_temp_max:
    case hwmon_temp_crit:
    case hwmon_temp_crit_hyst:
    return 0644;
    default:
    break;
    }
    break;
    default:
    break;
    }
    return 0;
    }
    static const struct hwmon_ops tmp401_ops = {
    .is_visible = tmp401_is_visible,
    .read = tmp401_read,
    .write = tmp401_write,
    };
// chip initialization, detect, probe
#[no_mangle]
unsafe extern "C" fn tmp401_init_client(data: *mut tmp401_data) -> c_int {
    static int tmp401_init_client(struct tmp401_data *data)
    {
    struct regmap *regmap = data.regmap;
    u32 config, config_orig;
    int ret;
    let mut val: u32 = 0;
    let mut nfactor: i32 = 0;
// Set conversion rate to 2 Hz
    ret = regmap_write(regmap, TMP401_CONVERSION_RATE, 5);
    if (ret < 0)
    return ret;
// Start conversions (disable shutdown if necessary)
    ret = regmap_read(regmap, TMP401_CONFIG, &config);
    if (ret < 0)
    return ret;
    config_orig = config;
    config &= ~TMP401_CONFIG_SHUTDOWN;
    if (of_property_read_bool(data.client.dev.of_node, "ti,extended-range-enable")) {
// Enable measurement over extended temperature range
    config |= TMP401_CONFIG_RANGE;
    }
    data.extended_range = !!(config & TMP401_CONFIG_RANGE);
    if (config != config_orig) {
    ret = regmap_write(regmap, TMP401_CONFIG, config);
    if (ret < 0)
    return ret;
    }
    ret = of_property_read_s32(data.client.dev.of_node, "ti,n-factor", &nfactor);
    if (!ret) {
    if (data.kind == tmp401) {
    dev_err(&data.client.dev, "ti,tmp401 does not support n-factor correction\n");
    return -EINVAL;
    }
    if (nfactor < -128 || nfactor > 127) {
    dev_err(&data.client.dev, "n-factor is invalid (%d)\n", nfactor);
    return -EINVAL;
    }
    ret = regmap_write(regmap, TMP4XX_N_FACTOR_REG, (unsigned int)nfactor);
    if (ret < 0)
    return ret;
    }
    ret = of_property_read_u32(data.client.dev.of_node, "ti,beta-compensation", &val);
    if (!ret) {
    if (data.kind == tmp401 || data.kind == tmp411) {
    dev_err(&data.client.dev, "ti,tmp401 or ti,tmp411 does not support beta compensation\n");
    return -EINVAL;
    }
    if (val > 15) {
    dev_err(&data.client.dev, "beta-compensation is invalid (%u)\n", val);
    return -EINVAL;
    }
    ret = regmap_write(regmap, TMP43X_BETA_RANGE, val);
    if (ret < 0)
    return ret;
    }
    return 0;
    }
    static int tmp401_detect(struct i2c_client *client,
    struct i2c_board_info *info)
    {
    enum chips kind;
    struct i2c_adapter *adapter = client.adapter;
    u8 reg;
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_BYTE_DATA))
    return -ENODEV;
// Detect and identify the chip
    reg = i2c_smbus_read_byte_data(client, TMP401_MANUFACTURER_ID_REG);
    if (reg != TMP401_MANUFACTURER_ID)
    return -ENODEV;
    reg = i2c_smbus_read_byte_data(client, TMP401_DEVICE_ID_REG);
    switch (reg) {
    case TMP401_DEVICE_ID:
    if (client.addr != 0x4c)
    return -ENODEV;
    kind = tmp401;
    break;
    case TMP411A_DEVICE_ID:
    if (client.addr != 0x4c)
    return -ENODEV;
    kind = tmp411;
    break;
    case TMP411B_DEVICE_ID:
    if (client.addr != 0x4d)
    return -ENODEV;
    kind = tmp411;
    break;
    case TMP411C_DEVICE_ID:
    if (client.addr != 0x4e)
    return -ENODEV;
    kind = tmp411;
    break;
    case TMP431_DEVICE_ID:
    if (client.addr != 0x4c && client.addr != 0x4d)
    return -ENODEV;
    kind = tmp431;
    break;
    case TMP432_DEVICE_ID:
    if (client.addr != 0x4c && client.addr != 0x4d)
    return -ENODEV;
    kind = tmp432;
    break;
    case TMP435_DEVICE_ID:
    kind = tmp435;
    break;
    default:
    return -ENODEV;
    }
    reg = i2c_smbus_read_byte_data(client, TMP401_CONFIG);
    if (reg & 0x1b)
    return -ENODEV;
    reg = i2c_smbus_read_byte_data(client, TMP401_CONVERSION_RATE);
// Datasheet says: 0x1-0x6
    if (reg > 15)
    return -ENODEV;
    strscpy(info.type, tmp401_id[kind].name, I2C_NAME_SIZE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tmp401_probe(client: *mut i2c_client) -> c_int {
    static int tmp401_probe(struct i2c_client *client)
    {
    static const char * const names[] = {
    "TMP401", "TMP411", "TMP431", "TMP432", "TMP435"
    };
    struct device *dev = &client.dev;
    struct hwmon_channel_info *info;
    struct device *hwmon_dev;
    struct tmp401_data *data;
    int status;
    data = devm_kzalloc(dev, sizeof(struct tmp401_data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.client = client;
    data.kind = (uintptr_t)i2c_get_match_data(client);
    data.regmap = devm_regmap_init(dev, core::ptr::null_mut(), data, &tmp401_regmap_config);
    if (IS_ERR(data.regmap))
    return PTR_ERR(data.regmap);
// initialize configuration data
    data.chip.ops = &tmp401_ops;
    data.chip.info = data.info;
    data.info[0] = &data.chip_info;
    data.info[1] = &data.temp_info;
    info = &data.chip_info;
    info.type = hwmon_chip;
    info.config = data.chip_channel_config;
    data.chip_channel_config[0] = HWMON_C_REGISTER_TZ | HWMON_C_UPDATE_INTERVAL;
    info = &data.temp_info;
    info.type = hwmon_temp;
    info.config = data.temp_channel_config;
    data.temp_channel_config[0] = HWMON_T_INPUT | HWMON_T_MIN | HWMON_T_MAX |
    HWMON_T_CRIT | HWMON_T_CRIT_HYST | HWMON_T_MIN_ALARM |
    HWMON_T_MAX_ALARM | HWMON_T_CRIT_ALARM;
    data.temp_channel_config[1] = HWMON_T_INPUT | HWMON_T_MIN | HWMON_T_MAX |
    HWMON_T_CRIT | HWMON_T_CRIT_HYST | HWMON_T_MIN_ALARM |
    HWMON_T_MAX_ALARM | HWMON_T_CRIT_ALARM | HWMON_T_FAULT;
    if (data.kind == tmp411) {
    data.temp_channel_config[0] |= HWMON_T_HIGHEST | HWMON_T_LOWEST;
    data.temp_channel_config[1] |= HWMON_T_HIGHEST | HWMON_T_LOWEST;
    data.chip_channel_config[0] |= HWMON_C_TEMP_RESET_HISTORY;
    }
    if (data.kind == tmp432) {
    data.temp_channel_config[2] = HWMON_T_INPUT | HWMON_T_MIN | HWMON_T_MAX |
    HWMON_T_CRIT | HWMON_T_CRIT_HYST | HWMON_T_MIN_ALARM |
    HWMON_T_MAX_ALARM | HWMON_T_CRIT_ALARM | HWMON_T_FAULT;
    }
// Initialize the TMP401 chip
    status = tmp401_init_client(data);
    if (status < 0)
    return status;
    hwmon_dev = devm_hwmon_device_register_with_info(dev, client.name, data,
    &data.chip, core::ptr::null_mut());
    if (IS_ERR(hwmon_dev))
    return PTR_ERR(hwmon_dev);
    dev_info(dev, "Detected TI %s chip\n", names[data.kind]);
    return 0;
    }
    static const struct of_device_id __maybe_unused tmp4xx_of_match[] = {
    { .compatible = "ti,tmp401", },
    { .compatible = "ti,tmp411", },
    { .compatible = "ti,tmp431", },
    { .compatible = "ti,tmp432", },
    { .compatible = "ti,tmp435", },
    { },
    };
    MODULE_DEVICE_TABLE(of, tmp4xx_of_match);
    static struct i2c_driver tmp401_driver = {
    .class		= I2C_CLASS_HWMON,
    .driver = {
    .name	= "tmp401",
    .of_match_table = of_match_ptr(tmp4xx_of_match),
    },
    .probe		= tmp401_probe,
    .id_table	= tmp401_id,
    .detect		= tmp401_detect,
    .address_list	= normal_i2c,
    };
    module_i2c_driver(tmp401_driver);
    MODULE_AUTHOR("Hans de Goede <hdegoede@redhat.com>");
    MODULE_DESCRIPTION("Texas Instruments TMP401 temperature sensor driver");
    MODULE_LICENSE("GPL");
