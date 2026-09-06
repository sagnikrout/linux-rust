//! Automatically rewritten from C to Rust
//! Source: drivers/iio/dac/mcp4728.c
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
// Support for Microchip MCP4728
//
// Copyright (C) 2023 Andrea Collamati <andrea.collamati@gmail.com>
//
// Based on mcp4725 by Peter Meerwald <pmeerw@pmeerw.net>
//
// Driver for the Microchip I2C 12-bit digital-to-analog quad channels
// converter (DAC).
//
// (7-bit I2C slave address 0x60, the three LSBs can be configured in
// hardware)
//

pub const MCP4728_RESOLUTION: c_int = 12;
pub const MCP4728_N_CHANNELS: c_int = 4;

pub const MCP4728_MW_CMD: c_uint = 0x08 /* Multiwrite Command */;
pub const MCP4728_SW_CMD: c_uint = 0x0A /* Sequential Write Command with EEPROM */;

    enum vref_mode {
    MCP4728_VREF_EXTERNAL_VDD    = 0,
    MCP4728_VREF_INTERNAL_2048mV = 1,
    };
    enum gain_mode {
    MCP4728_GAIN_X1 = 0,
    MCP4728_GAIN_X2 = 1,
    };
    enum iio_powerdown_mode {
    MCP4728_IIO_1K,
    MCP4728_IIO_100K,
    MCP4728_IIO_500K,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp4728_channel_data {
    pub ref_mode: enum vref_mode,
    pub pd_mode: enum iio_powerdown_mode,
    pub g_mode: enum gain_mode,
    pub dac_value: u16,
}

// MCP4728 Full Scale Ranges
// the device available ranges are
// - VREF = VDD				FSR = from 0.0V to VDD
// - VREF = Internal	Gain = 1	FSR = from 0.0V to VREF
// - VREF = Internal	Gain = 2	FSR = from 0.0V to 2*VREF
//
    enum mcp4728_scale {
    MCP4728_SCALE_VDD,
    MCP4728_SCALE_VINT_NO_GAIN,
    MCP4728_SCALE_VINT_GAIN_X2,
    MCP4728_N_SCALES
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp4728_data {
    pub client: *mut i2c_client,
    pub powerdown: bool,
    pub 2]: *mut *mut int scales_avail[MCP4728_N_SCALES,
    pub chdata: [mcp4728_channel_data; MCP4728_N_CHANNELS],
}

    .type = IIO_VOLTAGE,						\
    .output = 1,							\
    .indexed = 1,							\
    .channel = chan,						\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW)	|		\
    BIT(IIO_CHAN_INFO_SCALE),			\
    .info_mask_shared_by_type_available = BIT(IIO_CHAN_INFO_SCALE),	\
    .ext_info = mcp4728_ext_info,					\
    }
    static int mcp4728_suspend(struct device *dev);
    static int mcp4728_resume(struct device *dev);
    static ssize_t mcp4728_store_eeprom(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t len)
    {
    struct iio_dev *indio_dev = dev_to_iio_dev(dev);
    struct mcp4728_data *data = iio_priv(indio_dev);
    u8 outbuf[MCP4728_WRITE_EEPROM_LEN];
    let mut tries: c_int = 20;
    u8 inbuf[3];
    bool state;
    int ret;
    unsigned int i;
    ret = kstrtobool(buf, &state);
    if (ret < 0)
    return ret;
    if (!state)
    return 0;
    outbuf[0] = FIELD_PREP(MCP4728_CMD_MASK, MCP4728_SW_CMD);
    for (i = 0; i < MCP4728_N_CHANNELS; i++) {
    struct mcp4728_channel_data *ch = &data.chdata[i];
    let mut offset: c_int = 1 + i * 2;
    outbuf[offset] = FIELD_PREP(MCP4728_VREF_MASK, ch.ref_mode);
    if (data.powerdown) {
    let mut mcp4728_pd_mode: u8 = ch.pd_mode + 1;
    outbuf[offset] |= FIELD_PREP(MCP4728_PDMODE_MASK,
    mcp4728_pd_mode);
    }
    outbuf[offset] |= FIELD_PREP(MCP4728_GAIN_MASK, ch.g_mode);
    outbuf[offset] |=
    FIELD_PREP(MCP4728_DAC_H_MASK, ch.dac_value >> 8);
    outbuf[offset + 1] =
    FIELD_PREP(MCP4728_DAC_L_MASK, ch.dac_value);
    }
    ret = i2c_master_send(data.client, outbuf, MCP4728_WRITE_EEPROM_LEN);
    if (ret < 0)
    return ret;
#[no_mangle]
pub unsafe extern "C" fn if(MCP4728_WRITE_EEPROM_LEN: ret !=) -> else {
    else if (ret != MCP4728_WRITE_EEPROM_LEN)
    return -EIO;
// wait RDY signal for write complete, takes up to 50ms
    while (tries--) {
    msleep(20);
    ret = i2c_master_recv(data.client, inbuf, 3);
    if (ret < 0)
    return ret;
#[no_mangle]
pub unsafe extern "C" fn if(3: ret !=) -> else {
    else if (ret != 3)
    return -EIO;
    if (FIELD_GET(MCP4728_RDY_MASK, inbuf[0]))
    break;
    }
    if (tries < 0) {
    dev_err(&data.client.dev, "%s failed, incomplete\n",
    __func__);
    return -EIO;
    }
    return len;
    }
    static IIO_DEVICE_ATTR(store_eeprom, 0200, core::ptr::null_mut(), mcp4728_store_eeprom, 0);
    static struct attribute *mcp4728_attributes[] = {
    &iio_dev_attr_store_eeprom.dev_attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group mcp4728_attribute_group = {
    .attrs = mcp4728_attributes,
    };
#[no_mangle]
unsafe extern "C" fn mcp4728_program_channel_cfg(channel: c_int, indio_dev: *mut iio_dev) -> c_int {
    static int mcp4728_program_channel_cfg(int channel, struct iio_dev *indio_dev)
    {
    struct mcp4728_data *data	= iio_priv(indio_dev);
    struct mcp4728_channel_data *ch = &data.chdata[channel];
    u8 outbuf[3];
    int ret;
    outbuf[0] = FIELD_PREP(MCP4728_CMD_MASK, MCP4728_MW_CMD);
    outbuf[0] |= FIELD_PREP(MCP4728_CHSEL_MASK, channel);
    outbuf[0] |= FIELD_PREP(MCP4728_UDAC_MASK, 0);
    outbuf[1] = FIELD_PREP(MCP4728_VREF_MASK, ch.ref_mode);
    if (data.powerdown)
    outbuf[1] |= FIELD_PREP(MCP4728_PDMODE_MASK, ch.pd_mode + 1);
    outbuf[1] |= FIELD_PREP(MCP4728_GAIN_MASK, ch.g_mode);
    outbuf[1] |= FIELD_PREP(MCP4728_DAC_H_MASK, ch.dac_value >> 8);
    outbuf[2] = FIELD_PREP(MCP4728_DAC_L_MASK, ch.dac_value);
    ret = i2c_master_send(data.client, outbuf, 3);
    if (ret < 0)
    return ret;
#[no_mangle]
pub unsafe extern "C" fn if(3: ret !=) -> else {
    else if (ret != 3)
    return -EIO;
    return 0;
    }
    static const char *const mcp4728_powerdown_modes[] = { "1kohm_to_gnd",
    "100kohm_to_gnd",
    "500kohm_to_gnd" };
    static int mcp4728_get_powerdown_mode(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan)
    {
    struct mcp4728_data *data = iio_priv(indio_dev);
    return data.chdata[chan.channel].pd_mode;
    }
    static int mcp4728_set_powerdown_mode(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    unsigned int mode)
    {
    struct mcp4728_data *data = iio_priv(indio_dev);
    data.chdata[chan.channel].pd_mode = mode;
    return 0;
    }
    static ssize_t mcp4728_read_powerdown(struct iio_dev *indio_dev,
    uintptr_t private,
    const struct iio_chan_spec *chan,
    char *buf)
    {
    struct mcp4728_data *data = iio_priv(indio_dev);
    return sysfs_emit(buf, "%d\n", data.powerdown);
    }
    static ssize_t mcp4728_write_powerdown(struct iio_dev *indio_dev,
    uintptr_t private,
    const struct iio_chan_spec *chan,
    const char *buf, size_t len)
    {
    struct mcp4728_data *data = iio_priv(indio_dev);
    bool state;
    int ret;
    ret = kstrtobool(buf, &state);
    if (ret)
    return ret;
    if (state)
    ret = mcp4728_suspend(&data.client.dev);
    else
    ret = mcp4728_resume(&data.client.dev);
    if (ret < 0)
    return ret;
    return len;
    }
    static const struct iio_enum mcp4728_powerdown_mode_enum = {
    .items	   = mcp4728_powerdown_modes,
    .num_items = ARRAY_SIZE(mcp4728_powerdown_modes),
    .get	   = mcp4728_get_powerdown_mode,
    .set	   = mcp4728_set_powerdown_mode,
    };
    static const struct iio_chan_spec_ext_info mcp4728_ext_info[] = {
    {
    .name	= "powerdown",
    .read	= mcp4728_read_powerdown,
    .write	= mcp4728_write_powerdown,
    .shared = IIO_SEPARATE,
    },
    IIO_ENUM("powerdown_mode", IIO_SEPARATE, &mcp4728_powerdown_mode_enum),
    IIO_ENUM_AVAILABLE("powerdown_mode", IIO_SHARED_BY_TYPE,
    &mcp4728_powerdown_mode_enum),
    { }
    };
    static const struct iio_chan_spec mcp4728_channels[MCP4728_N_CHANNELS] = {
    MCP4728_CHAN(0),
    MCP4728_CHAN(1),
    MCP4728_CHAN(2),
    MCP4728_CHAN(3),
    };
    static void mcp4728_get_scale_avail(enum mcp4728_scale scale,
    struct mcp4728_data *data, int *val,
    int *val2)
    {
// val  = data->scales_avail[scale * 2];
// val2 = data->scales_avail[scale * 2 + 1];
    }
    static void mcp4728_get_scale(int channel, struct mcp4728_data *data, int *val,
    int *val2)
    {
    let mut ref_mode: c_int = data.chdata[channel].ref_mode;
    let mut g_mode: c_int = data.chdata[channel].g_mode;
    if (ref_mode == MCP4728_VREF_EXTERNAL_VDD) {
    mcp4728_get_scale_avail(MCP4728_SCALE_VDD, data, val, val2);
    } else {
    if (g_mode == MCP4728_GAIN_X1) {
    mcp4728_get_scale_avail(MCP4728_SCALE_VINT_NO_GAIN,
    data, val, val2);
    } else {
    mcp4728_get_scale_avail(MCP4728_SCALE_VINT_GAIN_X2,
    data, val, val2);
    }
    }
    }
    static int mcp4728_find_matching_scale(struct mcp4728_data *data, int val,
    int val2)
    {
    for (int i = 0; i < MCP4728_N_SCALES; i++) {
    if (data.scales_avail[i * 2] == val &&
    data.scales_avail[i * 2 + 1] == val2)
    return i;
    }
    return -EINVAL;
    }
    static int mcp4728_set_scale(int channel, struct mcp4728_data *data, int val,
    int val2)
    {
    let mut scale: c_int = mcp4728_find_matching_scale(data, val, val2);
    if (scale < 0)
    return scale;
    switch (scale) {
    case MCP4728_SCALE_VDD:
    data.chdata[channel].ref_mode = MCP4728_VREF_EXTERNAL_VDD;
    return 0;
    case MCP4728_SCALE_VINT_NO_GAIN:
    data.chdata[channel].ref_mode = MCP4728_VREF_INTERNAL_2048mV;
    data.chdata[channel].g_mode   = MCP4728_GAIN_X1;
    return 0;
    case MCP4728_SCALE_VINT_GAIN_X2:
    data.chdata[channel].ref_mode = MCP4728_VREF_INTERNAL_2048mV;
    data.chdata[channel].g_mode   = MCP4728_GAIN_X2;
    return 0;
    default:
    return -EINVAL;
    }
    }
    static int mcp4728_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int *val,
    int *val2, long mask)
    {
    struct mcp4728_data *data = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
// val = data->chdata[chan->channel].dac_value;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
    mcp4728_get_scale(chan.channel, data, val, val2);
    return IIO_VAL_INT_PLUS_MICRO;
    }
    return -EINVAL;
    }
    static int mcp4728_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int val,
    int val2, long mask)
    {
    struct mcp4728_data *data = iio_priv(indio_dev);
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    if (val < 0 || val > GENMASK(MCP4728_RESOLUTION - 1, 0))
    return -EINVAL;
    data.chdata[chan.channel].dac_value = val;
    return mcp4728_program_channel_cfg(chan.channel, indio_dev);
    case IIO_CHAN_INFO_SCALE:
    ret = mcp4728_set_scale(chan.channel, data, val, val2);
    if (ret)
    return ret;
    return mcp4728_program_channel_cfg(chan.channel, indio_dev);
    default:
    return -EINVAL;
    }
    }
    static void mcp4728_init_scale_avail(enum mcp4728_scale scale, int vref_mv,
    struct mcp4728_data *data)
    {
    s64 tmp;
    int value_micro;
    int value_int;
    tmp	  = (s64)vref_mv * 1000000LL >> MCP4728_RESOLUTION;
    value_int = div_s64_rem(tmp, 1000000LL, &value_micro);
    data.scales_avail[scale * 2]	  = value_int;
    data.scales_avail[scale * 2 + 1] = value_micro;
    }
#[no_mangle]
unsafe extern "C" fn mcp4728_init_scales_avail(data: *mut mcp4728_data, vdd_mv: c_int) -> c_int {
    static int mcp4728_init_scales_avail(struct mcp4728_data *data, int vdd_mv)
    {
    mcp4728_init_scale_avail(MCP4728_SCALE_VDD, vdd_mv, data);
    mcp4728_init_scale_avail(MCP4728_SCALE_VINT_NO_GAIN, 2048, data);
    mcp4728_init_scale_avail(MCP4728_SCALE_VINT_GAIN_X2, 4096, data);
    return 0;
    }
    static int mcp4728_read_avail(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    const int **vals, int *type, int *length,
    long info)
    {
    struct mcp4728_data *data = iio_priv(indio_dev);
    switch (info) {
    case IIO_CHAN_INFO_SCALE:
// type = IIO_VAL_INT_PLUS_MICRO;
    switch (chan.type) {
    case IIO_VOLTAGE:
// vals	= data->scales_avail;
// length = MCP4728_N_SCALES * 2;
    return IIO_AVAIL_LIST;
    default:
    return -EINVAL;
    }
    default:
    return -EINVAL;
    }
    }
    static const struct iio_info mcp4728_info = {
    .read_raw   = mcp4728_read_raw,
    .write_raw  = mcp4728_write_raw,
    .read_avail = &mcp4728_read_avail,
    .attrs	    = &mcp4728_attribute_group,
    };
#[no_mangle]
unsafe extern "C" fn mcp4728_suspend(dev: *mut device) -> c_int {
    static int mcp4728_suspend(struct device *dev)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(dev);
    struct mcp4728_data *data = iio_priv(indio_dev);
    unsigned int i;
    data.powerdown = true;
    for (i = 0; i < MCP4728_N_CHANNELS; i++) {
    let mut err: c_int = mcp4728_program_channel_cfg(i, indio_dev);
    if (err)
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mcp4728_resume(dev: *mut device) -> c_int {
    static int mcp4728_resume(struct device *dev)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(dev);
    struct mcp4728_data *data = iio_priv(indio_dev);
    let mut err: c_int = 0;
    unsigned int i;
    data.powerdown = false;
    for (i = 0; i < MCP4728_N_CHANNELS; i++) {
    let mut ret: c_int = mcp4728_program_channel_cfg(i, indio_dev);
    if (ret)
    err = ret;
    }
    return err;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(mcp4728_pm_ops, mcp4728_suspend,
    mcp4728_resume);
#[no_mangle]
unsafe extern "C" fn mcp4728_init_channels_data(data: *mut mcp4728_data) -> c_int {
    static int mcp4728_init_channels_data(struct mcp4728_data *data)
    {
    u8 inbuf[MCP4728_READ_RESPONSE_LEN];
    int ret;
    unsigned int i;
    ret = i2c_master_recv(data.client, inbuf, MCP4728_READ_RESPONSE_LEN);
    if (ret < 0) {
    return dev_err_probe(&data.client.dev, ret,
    "failed to read mcp4728 conf.\n");
    } else if (ret != MCP4728_READ_RESPONSE_LEN) {
    return dev_err_probe(&data.client.dev, -EIO,
    "failed to read mcp4728 conf. Wrong Response Len ret=%d\n",
    ret);
    }
    for (i = 0; i < MCP4728_N_CHANNELS; i++) {
    struct mcp4728_channel_data *ch = &data.chdata[i];
    let mut r2: u8 = inbuf[i * 6 + 1];
    let mut r3: u8 = inbuf[i * 6 + 2];
    ch.dac_value = FIELD_GET(MCP4728_DAC_H_MASK, r2) << 8 |
    FIELD_GET(MCP4728_DAC_L_MASK, r3);
    ch.ref_mode = FIELD_GET(MCP4728_VREF_MASK, r2);
    ch.pd_mode  = FIELD_GET(MCP4728_PDMODE_MASK, r2);
    ch.g_mode   = FIELD_GET(MCP4728_GAIN_MASK, r2);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mcp4728_probe(client: *mut i2c_client) -> c_int {
    static int mcp4728_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    struct mcp4728_data *data;
    struct iio_dev *indio_dev;
    int ret, vdd_mv;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    i2c_set_clientdata(client, indio_dev);
    data.client = client;
    ret = devm_regulator_get_enable_read_voltage(&client.dev, "vdd");
    if (ret < 0)
    return ret;
    vdd_mv = ret / 1000;
//
// MCP4728 has internal EEPROM that save each channel boot
// configuration. It means that device configuration is unknown to the
// driver at kernel boot. mcp4728_init_channels_data() reads back DAC
// settings and stores them in data structure.
//
    ret = mcp4728_init_channels_data(data);
    if (ret) {
    return dev_err_probe(&client.dev, ret,
    "failed to read mcp4728 current configuration\n");
    }
    ret = mcp4728_init_scales_avail(data, vdd_mv);
    if (ret) {
    return dev_err_probe(&client.dev, ret,
    "failed to init scales\n");
    }
    indio_dev.name		= id.name;
    indio_dev.info		= &mcp4728_info;
    indio_dev.channels	= mcp4728_channels;
    indio_dev.num_channels = MCP4728_N_CHANNELS;
    indio_dev.modes	= INDIO_DIRECT_MODE;
    return devm_iio_device_register(&client.dev, indio_dev);
    }
    static const struct i2c_device_id mcp4728_id[] = {
    { .name = "mcp4728" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, mcp4728_id);
    static const struct of_device_id mcp4728_of_match[] = {
    { .compatible = "microchip,mcp4728" },
    { }
    };
    MODULE_DEVICE_TABLE(of, mcp4728_of_match);
    static struct i2c_driver mcp4728_driver = {
    .driver = {
    .name = "mcp4728",
    .of_match_table = mcp4728_of_match,
    .pm = pm_sleep_ptr(&mcp4728_pm_ops),
    },
    .probe = mcp4728_probe,
    .id_table = mcp4728_id,
    };
    module_i2c_driver(mcp4728_driver);
    MODULE_AUTHOR("Andrea Collamati <andrea.collamati@gmail.com>");
    MODULE_DESCRIPTION("MCP4728 12-bit DAC");
    MODULE_LICENSE("GPL");
