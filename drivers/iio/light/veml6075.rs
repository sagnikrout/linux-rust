//! Automatically rewritten from C to Rust
//! Source: drivers/iio/light/veml6075.c
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
// Vishay VEML6075 UVA and UVB light sensor
//
// Copyright 2023 Javier Carrasco <javier.carrasco.cruz@gmail.com>
//
// 7-bit I2C slave, address 0x10
//

pub const VEML6075_CMD_CONF: c_uint = 0x00 /* configuration register */;
pub const VEML6075_CMD_UVA: c_uint = 0x07 /* UVA channel */;
pub const VEML6075_CMD_UVB: c_uint = 0x09 /* UVB channel */;
pub const VEML6075_CMD_COMP1: c_uint = 0x0A /* visible light compensation */;
pub const VEML6075_CMD_COMP2: c_uint = 0x0B /* infrarred light compensation */;
pub const VEML6075_CMD_ID: c_uint = 0x0C /* device ID */;

pub const VEML6075_IT_50_MS: c_uint = 0x00;
pub const VEML6075_IT_100_MS: c_uint = 0x01;
pub const VEML6075_IT_200_MS: c_uint = 0x02;
pub const VEML6075_IT_400_MS: c_uint = 0x03;
pub const VEML6075_IT_800_MS: c_uint = 0x04;
pub const VEML6075_AF_DISABLE: c_uint = 0x00;
pub const VEML6075_AF_ENABLE: c_uint = 0x01;
pub const VEML6075_SD_DISABLE: c_uint = 0x00;
pub const VEML6075_SD_ENABLE: c_uint = 0x01;
// Open-air coefficients and responsivity
pub const VEML6075_A_COEF: c_int = 2220;
pub const VEML6075_B_COEF: c_int = 1330;
pub const VEML6075_C_COEF: c_int = 2950;
pub const VEML6075_D_COEF: c_int = 1740;
pub const VEML6075_UVA_RESP: c_int = 1461;
pub const VEML6075_UVB_RESP: c_int = 2591;
    static const int veml6075_it_ms[] = { 50, 100, 200, 400, 800 };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct veml6075_data {
    pub client: *mut i2c_client,
    pub regmap: *mut regmap,
//
// prevent integration time modification and triggering
// measurements while a measurement is underway.
//
    pub lock: mutex,
}

// channel number
    enum veml6075_chan {
    CH_UVA,
    CH_UVB,
    };
    static const struct iio_chan_spec veml6075_channels[] = {
    {
    .type = IIO_INTENSITY,
    .channel = CH_UVA,
    .modified = 1,
    .channel2 = IIO_MOD_LIGHT_UVA,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE),
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_INT_TIME),
    .info_mask_shared_by_all_available = BIT(IIO_CHAN_INFO_INT_TIME),
    },
    {
    .type = IIO_INTENSITY,
    .channel = CH_UVB,
    .modified = 1,
    .channel2 = IIO_MOD_LIGHT_UVB,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE),
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_INT_TIME),
    .info_mask_shared_by_all_available = BIT(IIO_CHAN_INFO_INT_TIME),
    },
    {
    .type = IIO_UVINDEX,
    .info_mask_separate = BIT(IIO_CHAN_INFO_PROCESSED),
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_INT_TIME),
    .info_mask_shared_by_all_available = BIT(IIO_CHAN_INFO_INT_TIME),
    },
    };
#[no_mangle]
unsafe extern "C" fn veml6075_request_measurement(data: *mut veml6075_data) -> c_int {
    static int veml6075_request_measurement(struct veml6075_data *data)
    {
    int ret, conf, int_time, int_index;
    ret = regmap_read(data.regmap, VEML6075_CMD_CONF, &conf);
    if (ret < 0)
    return ret;
// disable shutdown and trigger measurement
    ret = regmap_write(data.regmap, VEML6075_CMD_CONF,
    (conf | VEML6075_CONF_TRIG) & ~VEML6075_CONF_SD);
    if (ret < 0)
    return ret;
//
// A measurement requires between 1.30 and 1.40 times the integration
// time for all possible configurations. Using a 1.50 factor simplifies
// operations and ensures reliability under all circumstances.
//
    int_index = FIELD_GET(VEML6075_CONF_IT, conf);
    if (int_index >= ARRAY_SIZE(veml6075_it_ms))
    return -EINVAL;
    int_time = veml6075_it_ms[int_index];
    msleep(int_time + (int_time / 2));
// shutdown again, data registers are still accessible
    return regmap_update_bits(data.regmap, VEML6075_CMD_CONF,
    VEML6075_CONF_SD, VEML6075_CONF_SD);
    }
#[no_mangle]
unsafe extern "C" fn veml6075_uva_comp(raw_uva: c_int, comp1: c_int, comp2: c_int) -> c_int {
    static int veml6075_uva_comp(int raw_uva, int comp1, int comp2)
    {
    int comp1a_c, comp2a_c, uva_comp;
    comp1a_c = (comp1 * VEML6075_A_COEF) / 1000U;
    comp2a_c = (comp2 * VEML6075_B_COEF) / 1000U;
    uva_comp = raw_uva - comp1a_c - comp2a_c;
    return clamp_val(uva_comp, 0, U16_MAX);
    }
#[no_mangle]
unsafe extern "C" fn veml6075_uvb_comp(raw_uvb: c_int, comp1: c_int, comp2: c_int) -> c_int {
    static int veml6075_uvb_comp(int raw_uvb, int comp1, int comp2)
    {
    int comp1b_c, comp2b_c, uvb_comp;
    comp1b_c = (comp1 * VEML6075_C_COEF) / 1000U;
    comp2b_c = (comp2 * VEML6075_D_COEF) / 1000U;
    uvb_comp = raw_uvb - comp1b_c - comp2b_c;
    return clamp_val(uvb_comp, 0, U16_MAX);
    }
#[no_mangle]
unsafe extern "C" fn veml6075_read_comp(data: *mut veml6075_data, c1: *mut c_int, c2: *mut c_int) -> c_int {
    static int veml6075_read_comp(struct veml6075_data *data, int *c1, int *c2)
    {
    int ret;
    ret = regmap_read(data.regmap, VEML6075_CMD_COMP1, c1);
    if (ret < 0)
    return ret;
    return regmap_read(data.regmap, VEML6075_CMD_COMP2, c2);
    }
    static int veml6075_read_uv_direct(struct veml6075_data *data, int chan,
    int *val)
    {
    int c1, c2, ret;
    guard(mutex)(&data.lock);
    ret = veml6075_request_measurement(data);
    if (ret < 0)
    return ret;
    ret = veml6075_read_comp(data, &c1, &c2);
    if (ret < 0)
    return ret;
    switch (chan) {
    case CH_UVA:
    ret = regmap_read(data.regmap, VEML6075_CMD_UVA, val);
    if (ret < 0)
    return ret;
// val = veml6075_uva_comp(*val, c1, c2);
    return IIO_VAL_INT;
    case CH_UVB:
    ret = regmap_read(data.regmap, VEML6075_CMD_UVB, val);
    if (ret < 0)
    return ret;
// val = veml6075_uvb_comp(*val, c1, c2);
    return IIO_VAL_INT;
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn veml6075_read_int_time_index(data: *mut veml6075_data) -> c_int {
    static int veml6075_read_int_time_index(struct veml6075_data *data)
    {
    int ret, conf, int_index;
    ret = regmap_read(data.regmap, VEML6075_CMD_CONF, &conf);
    if (ret < 0)
    return ret;
    int_index = FIELD_GET(VEML6075_CONF_IT, conf);
    if (int_index >= ARRAY_SIZE(veml6075_it_ms))
    return -EINVAL;
    return int_index;
    }
#[no_mangle]
unsafe extern "C" fn veml6075_read_int_time_ms(data: *mut veml6075_data, val: *mut c_int) -> c_int {
    static int veml6075_read_int_time_ms(struct veml6075_data *data, int *val)
    {
    int int_index;
    guard(mutex)(&data.lock);
    int_index = veml6075_read_int_time_index(data);
    if (int_index < 0)
    return int_index;
// val = veml6075_it_ms[int_index];
    return IIO_VAL_INT;
    }
    static int veml6075_get_uvi_micro(struct veml6075_data *data, int uva_comp,
    int uvb_comp)
    {
    let mut uvia_micro: c_int = uva_comp * VEML6075_UVA_RESP;
    let mut uvib_micro: c_int = uvb_comp * VEML6075_UVB_RESP;
    int int_index;
    int_index = veml6075_read_int_time_index(data);
    if (int_index < 0)
    return int_index;
    switch (int_index) {
    case VEML6075_IT_50_MS:
    return uvia_micro + uvib_micro;
    case VEML6075_IT_100_MS:
    case VEML6075_IT_200_MS:
    case VEML6075_IT_400_MS:
    case VEML6075_IT_800_MS:
    return (uvia_micro + uvib_micro) / (2 << int_index);
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn veml6075_read_uvi(data: *mut veml6075_data, val: *mut c_int, val2: *mut c_int) -> c_int {
    static int veml6075_read_uvi(struct veml6075_data *data, int *val, int *val2)
    {
    int ret, c1, c2, uva, uvb, uvi_micro;
    guard(mutex)(&data.lock);
    ret = veml6075_request_measurement(data);
    if (ret < 0)
    return ret;
    ret = veml6075_read_comp(data, &c1, &c2);
    if (ret < 0)
    return ret;
    ret = regmap_read(data.regmap, VEML6075_CMD_UVA, &uva);
    if (ret < 0)
    return ret;
    ret = regmap_read(data.regmap, VEML6075_CMD_UVB, &uvb);
    if (ret < 0)
    return ret;
    uvi_micro = veml6075_get_uvi_micro(data, veml6075_uva_comp(uva, c1, c2),
    veml6075_uvb_comp(uvb, c1, c2));
    if (uvi_micro < 0)
    return uvi_micro;
// val = uvi_micro / MICRO;
// val2 = uvi_micro % MICRO;
    return IIO_VAL_INT_PLUS_MICRO;
    }
#[no_mangle]
unsafe extern "C" fn veml6075_read_responsivity(chan: c_int, val: *mut c_int, val2: *mut c_int) -> c_int {
    static int veml6075_read_responsivity(int chan, int *val, int *val2)
    {
// scale = 1 / resp
    switch (chan) {
    case CH_UVA:
// resp = 0.93 c/uW/cm2: scale = 1.75268817
// val = 1;
// val2 = 75268817;
    return IIO_VAL_INT_PLUS_NANO;
    case CH_UVB:
// resp = 2.1 c/uW/cm2: scale = 0.476190476
// val = 0;
// val2 = 476190476;
    return IIO_VAL_INT_PLUS_NANO;
    default:
    return -EINVAL;
    }
    }
    static int veml6075_read_avail(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    const int **vals, int *type, int *length,
    long mask)
    {
    switch (mask) {
    case IIO_CHAN_INFO_INT_TIME:
// length = ARRAY_SIZE(veml6075_it_ms);
// vals = veml6075_it_ms;
// type = IIO_VAL_INT;
    return IIO_AVAIL_LIST;
    default:
    return -EINVAL;
    }
    }
    static int veml6075_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct veml6075_data *data = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    return veml6075_read_uv_direct(data, chan.channel, val);
    case IIO_CHAN_INFO_PROCESSED:
    return veml6075_read_uvi(data, val, val2);
    case IIO_CHAN_INFO_INT_TIME:
    return veml6075_read_int_time_ms(data, val);
    case IIO_CHAN_INFO_SCALE:
    return veml6075_read_responsivity(chan.channel, val, val2);
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn veml6075_write_int_time_ms(data: *mut veml6075_data, val: c_int) -> c_int {
    static int veml6075_write_int_time_ms(struct veml6075_data *data, int val)
    {
    let mut i: c_int = ARRAY_SIZE(veml6075_it_ms);
    guard(mutex)(&data.lock);
    while (i-- > 0) {
    if (val == veml6075_it_ms[i])
    break;
    }
    if (i < 0)
    return -EINVAL;
    return regmap_update_bits(data.regmap, VEML6075_CMD_CONF,
    VEML6075_CONF_IT,
    FIELD_PREP(VEML6075_CONF_IT, i));
    }
    static int veml6075_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    struct veml6075_data *data = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_INT_TIME:
    return veml6075_write_int_time_ms(data, val);
    default:
    return -EINVAL;
    }
    }
    static const struct iio_info veml6075_info = {
    .read_avail = veml6075_read_avail,
    .read_raw = veml6075_read_raw,
    .write_raw = veml6075_write_raw,
    };
#[no_mangle]
unsafe extern "C" fn veml6075_readable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool veml6075_readable_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case VEML6075_CMD_CONF:
    case VEML6075_CMD_UVA:
    case VEML6075_CMD_UVB:
    case VEML6075_CMD_COMP1:
    case VEML6075_CMD_COMP2:
    case VEML6075_CMD_ID:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn veml6075_writable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool veml6075_writable_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case VEML6075_CMD_CONF:
    return true;
    default:
    return false;
    }
    }
    static const struct regmap_config veml6075_regmap_config = {
    .name = "veml6075",
    .reg_bits = 8,
    .val_bits = 16,
    .max_register = VEML6075_CMD_ID,
    .readable_reg = veml6075_readable_reg,
    .writeable_reg = veml6075_writable_reg,
    .val_format_endian = REGMAP_ENDIAN_LITTLE,
    };
#[no_mangle]
unsafe extern "C" fn veml6075_probe(client: *mut i2c_client) -> c_int {
    static int veml6075_probe(struct i2c_client *client)
    {
    struct veml6075_data *data;
    struct iio_dev *indio_dev;
    struct regmap *regmap;
    int config, ret;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    regmap = devm_regmap_init_i2c(client, &veml6075_regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    data = iio_priv(indio_dev);
    data.client = client;
    data.regmap = regmap;
    mutex_init(&data.lock);
    indio_dev.name = "veml6075";
    indio_dev.info = &veml6075_info;
    indio_dev.channels = veml6075_channels;
    indio_dev.num_channels = ARRAY_SIZE(veml6075_channels);
    indio_dev.modes = INDIO_DIRECT_MODE;
    ret = devm_regulator_get_enable(&client.dev, "vdd");
    if (ret < 0)
    return ret;
// default: 100ms integration time, active force enable, shutdown
    config = FIELD_PREP(VEML6075_CONF_IT, VEML6075_IT_100_MS) |
    FIELD_PREP(VEML6075_CONF_AF, VEML6075_AF_ENABLE) |
    FIELD_PREP(VEML6075_CONF_SD, VEML6075_SD_ENABLE);
    ret = regmap_write(data.regmap, VEML6075_CMD_CONF, config);
    if (ret < 0)
    return ret;
    return devm_iio_device_register(&client.dev, indio_dev);
    }
    static const struct i2c_device_id veml6075_id[] = {
    { .name = "veml6075" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, veml6075_id);
    static const struct of_device_id veml6075_of_match[] = {
    { .compatible = "vishay,veml6075" },
    { }
    };
    MODULE_DEVICE_TABLE(of, veml6075_of_match);
    static struct i2c_driver veml6075_driver = {
    .driver = {
    .name   = "veml6075",
    .of_match_table = veml6075_of_match,
    },
    .probe = veml6075_probe,
    .id_table = veml6075_id,
    };
    module_i2c_driver(veml6075_driver);
    MODULE_AUTHOR("Javier Carrasco <javier.carrasco.cruz@gmail.com>");
    MODULE_DESCRIPTION("Vishay VEML6075 UVA and UVB light sensor driver");
    MODULE_LICENSE("GPL");
