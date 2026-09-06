//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/max6621.c
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
//
// Hardware monitoring driver for Maxim MAX6621
//
// Copyright (c) 2017 Mellanox Technologies. All rights reserved.
// Copyright (c) 2017 Vadim Pasternak <vadimp@mellanox.com>
//

pub const MAX6621_TEMP_INPUT_REG_NUM: c_int = 9;

pub const MAX6621_TEMP_INPUT_MAX: c_int = 127000;
pub const MAX6621_TEMP_ALERT_CHAN_SHIFT: c_int = 1;
pub const MAX6621_TEMP_S0D0_REG: c_uint = 0x00;
pub const MAX6621_TEMP_S0D1_REG: c_uint = 0x01;
pub const MAX6621_TEMP_S1D0_REG: c_uint = 0x02;
pub const MAX6621_TEMP_S1D1_REG: c_uint = 0x03;
pub const MAX6621_TEMP_S2D0_REG: c_uint = 0x04;
pub const MAX6621_TEMP_S2D1_REG: c_uint = 0x05;
pub const MAX6621_TEMP_S3D0_REG: c_uint = 0x06;
pub const MAX6621_TEMP_S3D1_REG: c_uint = 0x07;
pub const MAX6621_TEMP_MAX_REG: c_uint = 0x08;
pub const MAX6621_TEMP_MAX_ADDR_REG: c_uint = 0x0a;
pub const MAX6621_TEMP_ALERT_CAUSE_REG: c_uint = 0x0b;
pub const MAX6621_CONFIG0_REG: c_uint = 0x0c;
pub const MAX6621_CONFIG1_REG: c_uint = 0x0d;
pub const MAX6621_CONFIG2_REG: c_uint = 0x0e;
pub const MAX6621_CONFIG3_REG: c_uint = 0x0f;
pub const MAX6621_TEMP_S0_ALERT_REG: c_uint = 0x10;
pub const MAX6621_TEMP_S1_ALERT_REG: c_uint = 0x11;
pub const MAX6621_TEMP_S2_ALERT_REG: c_uint = 0x12;
pub const MAX6621_TEMP_S3_ALERT_REG: c_uint = 0x13;
pub const MAX6621_CLEAR_ALERT_REG: c_uint = 0x15;

pub const MAX6621_REG_TEMP_SHIFT: c_uint = 0x06;
pub const MAX6621_ENABLE_TEMP_ALERTS_BIT: c_int = 4;
pub const MAX6621_ENABLE_I2C_CRC_BIT: c_int = 5;
pub const MAX6621_ENABLE_ALTERNATE_DATA: c_int = 6;
pub const MAX6621_ENABLE_LOCKUP_TO: c_int = 7;
pub const MAX6621_ENABLE_S0D0_BIT: c_int = 8;
pub const MAX6621_ENABLE_S3D1_BIT: c_int = 15;

    MAX6621_ENABLE_S0D0_BIT)
pub const MAX6621_POLL_DELAY_MASK: c_uint = 0x5;

    BIT(MAX6621_ENABLE_LOCKUP_TO) | \
    BIT(MAX6621_ENABLE_I2C_CRC_BIT) | \
    MAX6621_POLL_DELAY_MASK)
pub const MAX6621_PECI_BIT_TIME: c_uint = 0x2;
pub const MAX6621_PECI_RETRY_NUM: c_uint = 0x3;

    MAX6621_PECI_RETRY_NUM)
// Error codes
pub const MAX6621_TRAN_FAILED: c_uint = 0x8100	/*;
// PECI transaction failed for more
// than the configured number of
// consecutive retries.
//
pub const MAX6621_POOL_DIS: c_uint = 0x8101	/*;
// Polling disabled for requested
// socket/domain.
//
pub const MAX6621_POOL_UNCOMPLETE: c_uint = 0x8102	/*;
// First poll not yet completed for
// requested socket/domain (on
// startup).
//
pub const MAX6621_SD_DIS: c_uint = 0x8103	/*;
// Read maximum temperature requested,
// but no sockets/domains enabled or
// all enabled sockets/domains have
// errors; or read maximum temperature
// address requested, but read maximum
// temperature was not called.
//
pub const MAX6621_ALERT_DIS: c_uint = 0x8104	/*;
// Get alert socket/domain requested,
// but no alert active.
//
pub const MAX6621_PECI_ERR_MIN: c_uint = 0x8000	/* Intel spec PECI error min value. */;
pub const MAX6621_PECI_ERR_MAX: c_uint = 0x80ff	/* Intel spec PECI error max value. */;
    static const u32 max6621_temp_regs[] = {
    MAX6621_TEMP_MAX_REG, MAX6621_TEMP_S0D0_REG, MAX6621_TEMP_S1D0_REG,
    MAX6621_TEMP_S2D0_REG, MAX6621_TEMP_S3D0_REG, MAX6621_TEMP_S0D1_REG,
    MAX6621_TEMP_S1D1_REG, MAX6621_TEMP_S2D1_REG, MAX6621_TEMP_S3D1_REG,
    };
    static const char *const max6621_temp_labels[] = {
    "maximum",
    "socket0_0",
    "socket1_0",
    "socket2_0",
    "socket3_0",
    "socket0_1",
    "socket1_1",
    "socket2_1",
    "socket3_1",
    };
    static const int max6621_temp_alert_chan2reg[] = {
    MAX6621_TEMP_S0_ALERT_REG,
    MAX6621_TEMP_S1_ALERT_REG,
    MAX6621_TEMP_S2_ALERT_REG,
    MAX6621_TEMP_S3_ALERT_REG,
    };
//
// struct max6621_data - private data:
//
// @client: I2C client;
// @regmap: register map handle;
// @input_chan2reg: mapping from channel to register;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max6621_data {
    pub client: *mut i2c_client,
    pub regmap: *mut regmap,
    pub 1]: int input_chan2reg[MAX6621_TEMP_INPUT_REG_NUM +,
}

#[no_mangle]
unsafe extern "C" fn max6621_temp_mc2reg(val: c_long) -> c_long {
    static long max6621_temp_mc2reg(long val)
    {
    return (val / 1000L) << MAX6621_REG_TEMP_SHIFT;
    }
    static umode_t
    max6621_is_visible(const void *data, enum hwmon_sensor_types type, u32 attr,
    int channel)
    {
// Skip channels which are not physically conncted.
    if (((struct max6621_data *)data).input_chan2reg[channel] < 0)
    return 0;
    switch (type) {
    case hwmon_temp:
    switch (attr) {
    case hwmon_temp_input:
    case hwmon_temp_label:
    case hwmon_temp_crit_alarm:
    return 0444;
    case hwmon_temp_offset:
    case hwmon_temp_crit:
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
#[no_mangle]
unsafe extern "C" fn max6621_verify_reg_data(dev: *mut device, regval: c_int) -> c_int {
    static int max6621_verify_reg_data(struct device *dev, int regval)
    {
    if (regval >= MAX6621_PECI_ERR_MIN &&
    regval <= MAX6621_PECI_ERR_MAX) {
    dev_dbg(dev, "PECI error code - err 0x%04x.\n",
    regval);
    return -EIO;
    }
    switch (regval) {
    case MAX6621_TRAN_FAILED:
    dev_dbg(dev, "PECI transaction failed - err 0x%04x.\n",
    regval);
    return -EIO;
    case MAX6621_POOL_DIS:
    dev_dbg(dev, "Polling disabled - err 0x%04x.\n", regval);
    return -EOPNOTSUPP;
    case MAX6621_POOL_UNCOMPLETE:
    dev_dbg(dev, "First poll not completed on startup - err 0x%04x.\n",
    regval);
    return -EIO;
    case MAX6621_SD_DIS:
    dev_dbg(dev, "Resource is disabled - err 0x%04x.\n", regval);
    return -EOPNOTSUPP;
    case MAX6621_ALERT_DIS:
    dev_dbg(dev, "No alert active - err 0x%04x.\n", regval);
    return -EOPNOTSUPP;
    default:
    return 0;
    }
    }
    static int
    max6621_read(struct device *dev, enum hwmon_sensor_types type, u32 attr,
    int channel, long *val)
    {
    struct max6621_data *data = dev_get_drvdata(dev);
    u32 regval;
    int reg;
    s8 temp;
    int ret;
    switch (type) {
    case hwmon_temp:
    switch (attr) {
    case hwmon_temp_input:
    reg = data.input_chan2reg[channel];
    ret = regmap_read(data.regmap, reg, &regval);
    if (ret)
    return ret;
    ret = max6621_verify_reg_data(dev, regval);
    if (ret)
    return ret;
//
// Bit MAX6621_REG_TEMP_SHIFT represents 1 degree step.
// The temperature is given in two's complement and 8
// bits is used for the register conversion.
//
    temp = (regval >> MAX6621_REG_TEMP_SHIFT);
// val = temp * 1000L;
    break;
    case hwmon_temp_offset:
    ret = regmap_read(data.regmap, MAX6621_CONFIG2_REG,
    &regval);
    if (ret)
    return ret;
    ret = max6621_verify_reg_data(dev, regval);
    if (ret)
    return ret;
// val = ((s16)regval >> MAX6621_REG_TEMP_SHIFT)
    1000L;
    break;
    case hwmon_temp_crit:
    channel -= MAX6621_TEMP_ALERT_CHAN_SHIFT;
    reg = max6621_temp_alert_chan2reg[channel];
    ret = regmap_read(data.regmap, reg, &regval);
    if (ret)
    return ret;
    ret = max6621_verify_reg_data(dev, regval);
    if (ret)
    return ret;
// val = (s16)regval * 1000L;
    break;
    case hwmon_temp_crit_alarm:
//
// Set val to zero to recover the case, when reading
// MAX6621_TEMP_ALERT_CAUSE_REG results in for example
// MAX6621_ALERT_DIS. Reading will return with error,
// but in such case alarm should be returned as 0.
//
// val = 0;
    ret = regmap_read(data.regmap,
    MAX6621_TEMP_ALERT_CAUSE_REG,
    &regval);
    if (ret)
    return ret;
    ret = max6621_verify_reg_data(dev, regval);
    if (ret) {
// Do not report error if alert is disabled.
    if (regval == MAX6621_ALERT_DIS)
    return 0;
    else
    return ret;
    }
//
// Clear the alert automatically, using send-byte
// smbus protocol for clearing alert.
//
    if (regval) {
    ret = i2c_smbus_write_byte(data.client,
    MAX6621_CLEAR_ALERT_REG);
    if (ret)
    return ret;
    }
// val = !!regval;
    break;
    default:
    return -EOPNOTSUPP;
    }
    break;
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
    static int
    max6621_write(struct device *dev, enum hwmon_sensor_types type, u32 attr,
    int channel, long val)
    {
    struct max6621_data *data = dev_get_drvdata(dev);
    u32 reg;
    switch (type) {
    case hwmon_temp:
    switch (attr) {
    case hwmon_temp_offset:
// Clamp to allowed range to prevent overflow.
    val = clamp_val(val, MAX6621_TEMP_INPUT_MIN,
    MAX6621_TEMP_INPUT_MAX);
    val = max6621_temp_mc2reg(val);
    return regmap_write(data.regmap,
    MAX6621_CONFIG2_REG, val);
    case hwmon_temp_crit:
    channel -= MAX6621_TEMP_ALERT_CHAN_SHIFT;
    reg = max6621_temp_alert_chan2reg[channel];
// Clamp to allowed range to prevent overflow.
    val = clamp_val(val, MAX6621_TEMP_INPUT_MIN,
    MAX6621_TEMP_INPUT_MAX);
    val = val / 1000L;
    return regmap_write(data.regmap, reg, val);
    default:
    return -EOPNOTSUPP;
    }
    break;
    default:
    return -EOPNOTSUPP;
    }
    return -EOPNOTSUPP;
    }
    static int
    max6621_read_string(struct device *dev, enum hwmon_sensor_types type, u32 attr,
    int channel, const char **str)
    {
    switch (type) {
    case hwmon_temp:
    switch (attr) {
    case hwmon_temp_label:
// str = max6621_temp_labels[channel];
    return 0;
    default:
    return -EOPNOTSUPP;
    }
    break;
    default:
    return -EOPNOTSUPP;
    }
    return -EOPNOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn max6621_writeable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool max6621_writeable_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case MAX6621_CONFIG0_REG:
    case MAX6621_CONFIG1_REG:
    case MAX6621_CONFIG2_REG:
    case MAX6621_CONFIG3_REG:
    case MAX6621_TEMP_S0_ALERT_REG:
    case MAX6621_TEMP_S1_ALERT_REG:
    case MAX6621_TEMP_S2_ALERT_REG:
    case MAX6621_TEMP_S3_ALERT_REG:
    case MAX6621_TEMP_ALERT_CAUSE_REG:
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn max6621_readable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool max6621_readable_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case MAX6621_TEMP_S0D0_REG:
    case MAX6621_TEMP_S0D1_REG:
    case MAX6621_TEMP_S1D0_REG:
    case MAX6621_TEMP_S1D1_REG:
    case MAX6621_TEMP_S2D0_REG:
    case MAX6621_TEMP_S2D1_REG:
    case MAX6621_TEMP_S3D0_REG:
    case MAX6621_TEMP_S3D1_REG:
    case MAX6621_TEMP_MAX_REG:
    case MAX6621_TEMP_MAX_ADDR_REG:
    case MAX6621_CONFIG0_REG:
    case MAX6621_CONFIG1_REG:
    case MAX6621_CONFIG2_REG:
    case MAX6621_CONFIG3_REG:
    case MAX6621_TEMP_S0_ALERT_REG:
    case MAX6621_TEMP_S1_ALERT_REG:
    case MAX6621_TEMP_S2_ALERT_REG:
    case MAX6621_TEMP_S3_ALERT_REG:
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn max6621_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool max6621_volatile_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case MAX6621_TEMP_S0D0_REG:
    case MAX6621_TEMP_S0D1_REG:
    case MAX6621_TEMP_S1D0_REG:
    case MAX6621_TEMP_S1D1_REG:
    case MAX6621_TEMP_S2D0_REG:
    case MAX6621_TEMP_S2D1_REG:
    case MAX6621_TEMP_S3D0_REG:
    case MAX6621_TEMP_S3D1_REG:
    case MAX6621_TEMP_MAX_REG:
    case MAX6621_TEMP_S0_ALERT_REG:
    case MAX6621_TEMP_S1_ALERT_REG:
    case MAX6621_TEMP_S2_ALERT_REG:
    case MAX6621_TEMP_S3_ALERT_REG:
    case MAX6621_TEMP_ALERT_CAUSE_REG:
    return true;
    }
    return false;
    }
    static const struct reg_default max6621_regmap_default[] = {
    { MAX6621_CONFIG0_REG, MAX6621_CONFIG0_INIT },
    { MAX6621_CONFIG1_REG, MAX6621_CONFIG1_INIT },
    };
    static const struct regmap_config max6621_regmap_config = {
    .reg_bits = 8,
    .val_bits = 16,
    .max_register = MAX6621_REG_MAX,
    .val_format_endian = REGMAP_ENDIAN_LITTLE,
    .cache_type = REGCACHE_FLAT,
    .writeable_reg = max6621_writeable_reg,
    .readable_reg = max6621_readable_reg,
    .volatile_reg = max6621_volatile_reg,
    .reg_defaults = max6621_regmap_default,
    .num_reg_defaults = ARRAY_SIZE(max6621_regmap_default),
    };
    static const struct hwmon_channel_info * const max6621_info[] = {
    HWMON_CHANNEL_INFO(chip,
    HWMON_C_REGISTER_TZ),
    HWMON_CHANNEL_INFO(temp,
    HWMON_T_INPUT | HWMON_T_LABEL | HWMON_T_OFFSET,
    HWMON_T_INPUT | HWMON_T_CRIT | HWMON_T_CRIT_ALARM | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_CRIT | HWMON_T_CRIT_ALARM | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_CRIT | HWMON_T_CRIT_ALARM | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_CRIT | HWMON_T_CRIT_ALARM | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops max6621_hwmon_ops = {
    .read = max6621_read,
    .write = max6621_write,
    .read_string = max6621_read_string,
    .is_visible = max6621_is_visible,
    };
    static const struct hwmon_chip_info max6621_chip_info = {
    .ops = &max6621_hwmon_ops,
    .info = max6621_info,
    };
#[no_mangle]
unsafe extern "C" fn max6621_probe(client: *mut i2c_client) -> c_int {
    static int max6621_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct max6621_data *data;
    struct device *hwmon_dev;
    int i;
    int ret;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.regmap = devm_regmap_init_i2c(client, &max6621_regmap_config);
    if (IS_ERR(data.regmap))
    return PTR_ERR(data.regmap);
    i2c_set_clientdata(client, data);
    data.client = client;
// Set CONFIG0 register masking temperature alerts and PEC.
    ret = regmap_write(data.regmap, MAX6621_CONFIG0_REG,
    MAX6621_CONFIG0_INIT);
    if (ret)
    return ret;
// Set CONFIG1 register for PEC access retry number.
    ret = regmap_write(data.regmap, MAX6621_CONFIG1_REG,
    MAX6621_CONFIG1_INIT);
    if (ret)
    return ret;
// Sync registers with hardware.
    regcache_mark_dirty(data.regmap);
    ret = regcache_sync(data.regmap);
    if (ret)
    return ret;
// Verify which temperature input registers are enabled.
    for (i = 0; i < MAX6621_TEMP_INPUT_REG_NUM; i++) {
    ret = i2c_smbus_read_word_data(client, max6621_temp_regs[i]);
    if (ret < 0)
    return ret;
    ret = max6621_verify_reg_data(dev, ret);
    if (ret) {
    data.input_chan2reg[i] = -1;
    continue;
    }
    data.input_chan2reg[i] = max6621_temp_regs[i];
    }
    hwmon_dev = devm_hwmon_device_register_with_info(dev, client.name,
    data,
    &max6621_chip_info,
    core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static const struct i2c_device_id max6621_id[] = {
    { .name = MAX6621_DRV_NAME },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, max6621_id);
    static const struct of_device_id __maybe_unused max6621_of_match[] = {
    { .compatible = "maxim,max6621" },
    { }
    };
    MODULE_DEVICE_TABLE(of, max6621_of_match);
    static struct i2c_driver max6621_driver = {
    .driver = {
    .name = MAX6621_DRV_NAME,
    .of_match_table = of_match_ptr(max6621_of_match),
    },
    .probe		= max6621_probe,
    .id_table	= max6621_id,
    };
    module_i2c_driver(max6621_driver);
    MODULE_AUTHOR("Vadim Pasternak <vadimp@mellanox.com>");
    MODULE_DESCRIPTION("Driver for Maxim MAX6621");
    MODULE_LICENSE("GPL");
