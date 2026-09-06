//! Automatically rewritten from C to Rust
//! Source: drivers/iio/chemical/sgp30.c
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
// sgp30.c - Support for Sensirion SGP Gas Sensors
//
// Copyright (C) 2018 Andreas Brauchli <andreas.brauchli@sensirion.com>
//
// I2C slave address: 0x58
//
// Datasheets:
// https://www.sensirion.com/file/datasheet_sgp30
// https://www.sensirion.com/file/datasheet_sgpc3
//
// TODO:
// - baseline support
// - humidity compensation
// - power mode switching (SGPC3)
//

pub const SGP_WORD_LEN: c_int = 2;
pub const SGP_CRC8_POLYNOMIAL: c_uint = 0x31;
pub const SGP_CRC8_INIT: c_uint = 0xff;
pub const SGP_CRC8_LEN: c_int = 1;

pub const SGP_CMD_DURATION_US: c_int = 12000;
pub const SGP_MEASUREMENT_DURATION_US: c_int = 50000;

pub const SGP_MEASUREMENT_LEN: c_int = 2;
pub const SGP30_MEASURE_INTERVAL_HZ: c_int = 1;
pub const SGPC3_MEASURE_INTERVAL_HZ: c_int = 2;

    DECLARE_CRC8_TABLE(sgp_crc8_table);
    enum sgp_product_id {
    SGP30 = 0,
    SGPC3,
    };
    enum sgp30_channel_idx {
    SGP30_IAQ_TVOC_IDX = 0,
    SGP30_IAQ_CO2EQ_IDX,
    SGP30_SIG_ETOH_IDX,
    SGP30_SIG_H2_IDX,
    };
    enum sgpc3_channel_idx {
    SGPC3_IAQ_TVOC_IDX = 10,
    SGPC3_SIG_ETOH_IDX,
    };
    enum sgp_cmd {
    SGP_CMD_IAQ_INIT			= SGP_CMD(0x2003),
    SGP_CMD_IAQ_MEASURE			= SGP_CMD(0x2008),
    SGP_CMD_GET_FEATURE_SET			= SGP_CMD(0x202f),
    SGP_CMD_GET_SERIAL_ID			= SGP_CMD(0x3682),
    SGP30_CMD_MEASURE_SIGNAL		= SGP_CMD(0x2050),
    SGPC3_CMD_MEASURE_RAW			= SGP_CMD(0x2046),
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgp_version {
    pub major: u8,
    pub minor: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgp_crc_word {
    pub value: __be16,
    pub crc8: u8,
    pub __attribute__((__packed__)): },
    union sgp_reading {
    pub start: u8,
    pub raw_words: [sgp_crc_word; 4],
}

    enum _iaq_buffer_state {
    IAQ_BUFFER_EMPTY = 0,
    IAQ_BUFFER_DEFAULT_VALS,
    IAQ_BUFFER_VALID,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgp_data {
    pub client: *mut i2c_client,
    pub iaq_thread: *mut task_struct,
    pub data_lock: mutex,
    pub iaq_init_start_jiffies: c_ulong,
    pub iaq_defval_skip_jiffies: c_ulong,
    pub product_id: u16,
    pub feature_set: u16,
    pub measure_interval_jiffies: c_ulong,
    pub iaq_init_cmd: enum sgp_cmd,
    pub measure_iaq_cmd: enum sgp_cmd,
    pub measure_gas_signals_cmd: enum sgp_cmd,
    pub buffer: union sgp_reading,
    pub iaq_buffer: union sgp_reading,
    pub iaq_buffer_state: enum _iaq_buffer_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgp_device {
    pub product_id: c_ulong,
    pub channels: *const iio_chan_spec,
    pub num_channels: c_int,
}

    static const struct sgp_version supported_versions_sgp30[] = {
    {
    .major = 1,
    .minor = 0,
    },
    };
    static const struct sgp_version supported_versions_sgpc3[] = {
    {
    .major = 0,
    .minor = 4,
    },
    };
    static const struct iio_chan_spec sgp30_channels[] = {
    {
    .type = IIO_CONCENTRATION,
    .channel2 = IIO_MOD_VOC,
    .modified = 1,
    .info_mask_separate = BIT(IIO_CHAN_INFO_PROCESSED),
    .address = SGP30_IAQ_TVOC_IDX,
    },
    {
    .type = IIO_CONCENTRATION,
    .channel2 = IIO_MOD_CO2,
    .modified = 1,
    .info_mask_separate = BIT(IIO_CHAN_INFO_PROCESSED),
    .address = SGP30_IAQ_CO2EQ_IDX,
    },
    {
    .type = IIO_CONCENTRATION,
    .channel2 = IIO_MOD_ETHANOL,
    .modified = 1,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),
    .address = SGP30_SIG_ETOH_IDX,
    },
    {
    .type = IIO_CONCENTRATION,
    .channel2 = IIO_MOD_H2,
    .modified = 1,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),
    .address = SGP30_SIG_H2_IDX,
    },
    };
    static const struct iio_chan_spec sgpc3_channels[] = {
    {
    .type = IIO_CONCENTRATION,
    .channel2 = IIO_MOD_VOC,
    .modified = 1,
    .info_mask_separate = BIT(IIO_CHAN_INFO_PROCESSED),
    .address = SGPC3_IAQ_TVOC_IDX,
    },
    {
    .type = IIO_CONCENTRATION,
    .channel2 = IIO_MOD_ETHANOL,
    .modified = 1,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),
    .address = SGPC3_SIG_ETOH_IDX,
    },
    };
    static const struct sgp_device sgp_devices[] = {
    [SGP30] = {
    .product_id = SGP30,
    .channels = sgp30_channels,
    .num_channels = ARRAY_SIZE(sgp30_channels),
    },
    [SGPC3] = {
    .product_id = SGPC3,
    .channels = sgpc3_channels,
    .num_channels = ARRAY_SIZE(sgpc3_channels),
    },
    };
//
// sgp_verify_buffer() - verify the checksums of the data buffer words
//
// @data:       SGP data
// @buf:        Raw data buffer
// @word_count: Num data words stored in the buffer, excluding CRC bytes
//
// Return:      0 on success, negative error otherwise.
//
    static int sgp_verify_buffer(const struct sgp_data *data,
    union sgp_reading *buf, size_t word_count)
    {
    let mut size: usize = word_count * (SGP_WORD_LEN + SGP_CRC8_LEN);
    int i;
    u8 crc;
    u8 *data_buf = &buf.start;
    for (i = 0; i < size; i += SGP_WORD_LEN + SGP_CRC8_LEN) {
    crc = crc8(sgp_crc8_table, &data_buf[i], SGP_WORD_LEN,
    SGP_CRC8_INIT);
    if (crc != data_buf[i + SGP_WORD_LEN]) {
    dev_err(&data.client.dev, "CRC error\n");
    return -EIO;
    }
    }
    return 0;
    }
//
// sgp_read_cmd() - reads data from sensor after issuing a command
// The caller must hold data->data_lock for the duration of the call.
// @data:        SGP data
// @cmd:         SGP Command to issue
// @buf:         Raw data buffer to use
// @word_count:  Num words to read, excluding CRC bytes
// @duration_us: Time taken to sensor to take a reading and data to be ready.
//
// Return:       0 on success, negative error otherwise.
//
    static int sgp_read_cmd(struct sgp_data *data, enum sgp_cmd cmd,
    union sgp_reading *buf, size_t word_count,
    unsigned long duration_us)
    {
    int ret;
    struct i2c_client *client = data.client;
    let mut size: usize = word_count * (SGP_WORD_LEN + SGP_CRC8_LEN);
    u8 *data_buf;
    ret = i2c_master_send(client, (const char *)&cmd, SGP_CMD_LEN);
    if (ret != SGP_CMD_LEN)
    return -EIO;
    usleep_range(duration_us, duration_us + 1000);
    if (word_count == 0)
    return 0;
    data_buf = &buf.start;
    ret = i2c_master_recv(client, data_buf, size);
    if (ret < 0)
    return ret;
    if (ret != size)
    return -EIO;
    return sgp_verify_buffer(data, buf, word_count);
    }
//
// sgp_measure_iaq() - measure and retrieve IAQ values from sensor
// The caller must hold data->data_lock for the duration of the call.
// @data:       SGP data
//
// Return:      0 on success, -EBUSY on default values, negative error
// otherwise.
//
#[no_mangle]
unsafe extern "C" fn sgp_measure_iaq(data: *mut sgp_data) -> c_int {
    static int sgp_measure_iaq(struct sgp_data *data)
    {
    int ret;
// data contains default values
    bool default_vals = !time_after(jiffies, data.iaq_init_start_jiffies +
    data.iaq_defval_skip_jiffies);
    ret = sgp_read_cmd(data, data.measure_iaq_cmd, &data.iaq_buffer,
    SGP_MEASUREMENT_LEN, SGP_MEASUREMENT_DURATION_US);
    if (ret < 0)
    return ret;
    data.iaq_buffer_state = IAQ_BUFFER_DEFAULT_VALS;
    if (default_vals)
    return -EBUSY;
    data.iaq_buffer_state = IAQ_BUFFER_VALID;
    return 0;
    }
    static void sgp_iaq_thread_sleep_until(const struct sgp_data *data,
    unsigned long sleep_jiffies)
    {
    let mut IAQ_POLL: c_long = 50000;
    while (!time_after(jiffies, sleep_jiffies)) {
    usleep_range(IAQ_POLL, IAQ_POLL + 10000);
    if (kthread_should_stop() || data.iaq_init_start_jiffies == 0)
    return;
    }
    }
#[no_mangle]
unsafe extern "C" fn sgp_iaq_threadfn(p: *mut c_void) -> c_int {
    static int sgp_iaq_threadfn(void *p)
    {
    struct sgp_data *data = (struct sgp_data *)p;
    unsigned long next_update_jiffies;
    int ret;
    while (!kthread_should_stop()) {
    mutex_lock(&data.data_lock);
    if (data.iaq_init_start_jiffies == 0) {
    ret = sgp_read_cmd(data, data.iaq_init_cmd, core::ptr::null_mut(), 0,
    SGP_CMD_DURATION_US);
    if (ret < 0)
    goto unlock_sleep_continue;
    data.iaq_init_start_jiffies = jiffies;
    }
    ret = sgp_measure_iaq(data);
    if (ret && ret != -EBUSY) {
    dev_warn(&data.client.dev,
    "IAQ measurement error [%d]\n", ret);
    }
    unlock_sleep_continue:
    next_update_jiffies = jiffies + data.measure_interval_jiffies;
    mutex_unlock(&data.data_lock);
    sgp_iaq_thread_sleep_until(data, next_update_jiffies);
    }
    return 0;
    }
    static int sgp_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int *val,
    int *val2, long mask)
    {
    struct sgp_data *data = iio_priv(indio_dev);
    struct sgp_crc_word *words;
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_PROCESSED:
    mutex_lock(&data.data_lock);
    if (data.iaq_buffer_state != IAQ_BUFFER_VALID) {
    mutex_unlock(&data.data_lock);
    return -EBUSY;
    }
    words = data.iaq_buffer.raw_words;
    switch (chan.address) {
    case SGP30_IAQ_TVOC_IDX:
    case SGPC3_IAQ_TVOC_IDX:
// val = 0;
// val2 = be16_to_cpu(words[1].value);
    ret = IIO_VAL_INT_PLUS_NANO;
    break;
    case SGP30_IAQ_CO2EQ_IDX:
// val = 0;
// val2 = be16_to_cpu(words[0].value);
    ret = IIO_VAL_INT_PLUS_MICRO;
    break;
    default:
    ret = -EINVAL;
    break;
    }
    mutex_unlock(&data.data_lock);
    break;
    case IIO_CHAN_INFO_RAW:
    mutex_lock(&data.data_lock);
    if (chan.address == SGPC3_SIG_ETOH_IDX) {
    if (data.iaq_buffer_state == IAQ_BUFFER_EMPTY)
    ret = -EBUSY;
    else
    ret = 0;
    words = data.iaq_buffer.raw_words;
    } else {
    ret = sgp_read_cmd(data, data.measure_gas_signals_cmd,
    &data.buffer, SGP_MEASUREMENT_LEN,
    SGP_MEASUREMENT_DURATION_US);
    words = data.buffer.raw_words;
    }
    if (ret) {
    mutex_unlock(&data.data_lock);
    return ret;
    }
    switch (chan.address) {
    case SGP30_SIG_ETOH_IDX:
// val = be16_to_cpu(words[1].value);
    ret = IIO_VAL_INT;
    break;
    case SGPC3_SIG_ETOH_IDX:
    case SGP30_SIG_H2_IDX:
// val = be16_to_cpu(words[0].value);
    ret = IIO_VAL_INT;
    break;
    default:
    ret = -EINVAL;
    break;
    }
    mutex_unlock(&data.data_lock);
    break;
    default:
    return -EINVAL;
    }
    return ret;
    }
    static int sgp_check_compat(struct sgp_data *data,
    unsigned int product_id)
    {
    struct device *dev = &data.client.dev;
    const struct sgp_version *supported_versions;
    u16 ix, num_fs;
    u16 product, generation, major, minor;
// driver does not match product
    generation = SGP_VERS_GEN(data);
    if (generation != 0) {
    dev_err(dev,
    "incompatible product generation %d != 0", generation);
    return -ENODEV;
    }
    product = SGP_VERS_PRODUCT(data);
    if (product != product_id) {
    dev_err(dev, "sensor reports a different product: 0x%04x\n",
    product);
    return -ENODEV;
    }
    if (SGP_VERS_RESERVED(data))
    dev_warn(dev, "reserved bit is set\n");
// engineering samples are not supported: no interface guarantees
    if (SGP_VERS_ENG_BIT(data))
    return -ENODEV;
    switch (product) {
    case SGP30:
    supported_versions = supported_versions_sgp30;
    num_fs = ARRAY_SIZE(supported_versions_sgp30);
    break;
    case SGPC3:
    supported_versions = supported_versions_sgpc3;
    num_fs = ARRAY_SIZE(supported_versions_sgpc3);
    break;
    default:
    return -ENODEV;
    }
    major = SGP_VERS_MAJOR(data);
    minor = SGP_VERS_MINOR(data);
    for (ix = 0; ix < num_fs; ix++) {
    if (major == supported_versions[ix].major &&
    minor >= supported_versions[ix].minor)
    return 0;
    }
    dev_err(dev, "unsupported sgp version: %d.%d\n", major, minor);
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn sgp_init(data: *mut sgp_data) {
    static void sgp_init(struct sgp_data *data)
    {
    data.iaq_init_cmd = SGP_CMD_IAQ_INIT;
    data.iaq_init_start_jiffies = 0;
    data.iaq_buffer_state = IAQ_BUFFER_EMPTY;
    switch (SGP_VERS_PRODUCT(data)) {
    case SGP30:
    data.measure_interval_jiffies = SGP30_MEASURE_INTERVAL_HZ * HZ;
    data.measure_iaq_cmd = SGP_CMD_IAQ_MEASURE;
    data.measure_gas_signals_cmd = SGP30_CMD_MEASURE_SIGNAL;
    data.product_id = SGP30;
    data.iaq_defval_skip_jiffies = 15 * HZ;
    break;
    case SGPC3:
    data.measure_interval_jiffies = SGPC3_MEASURE_INTERVAL_HZ * HZ;
    data.measure_iaq_cmd = SGPC3_CMD_MEASURE_RAW;
    data.measure_gas_signals_cmd = SGPC3_CMD_MEASURE_RAW;
    data.product_id = SGPC3;
    data.iaq_defval_skip_jiffies =
    43 * data.measure_interval_jiffies;
    break;
    }
    }
    static const struct iio_info sgp_info = {
    .read_raw	= sgp_read_raw,
    };
    static const struct of_device_id sgp_dt_ids[] = {
    { .compatible = "sensirion,sgp30", .data = &sgp_devices[SGP30] },
    { .compatible = "sensirion,sgpc3", .data = &sgp_devices[SGPC3] },
    { }
    };
    MODULE_DEVICE_TABLE(of, sgp_dt_ids);
#[no_mangle]
unsafe extern "C" fn sgp_probe(client: *mut i2c_client) -> c_int {
    static int sgp_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    const struct sgp_device *match_data;
    struct device *dev = &client.dev;
    struct iio_dev *indio_dev;
    struct sgp_data *data;
    int ret;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    match_data = i2c_get_match_data(client);
    data = iio_priv(indio_dev);
    i2c_set_clientdata(client, indio_dev);
    data.client = client;
    crc8_populate_msb(sgp_crc8_table, SGP_CRC8_POLYNOMIAL);
    mutex_init(&data.data_lock);
// get feature set version and write it to client data
    ret = sgp_read_cmd(data, SGP_CMD_GET_FEATURE_SET, &data.buffer, 1,
    SGP_CMD_DURATION_US);
    if (ret < 0)
    return ret;
    data.feature_set = be16_to_cpu(data.buffer.raw_words[0].value);
    ret = sgp_check_compat(data, match_data.product_id);
    if (ret)
    return ret;
    indio_dev.info = &sgp_info;
    indio_dev.name = id.name;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = match_data.channels;
    indio_dev.num_channels = match_data.num_channels;
    sgp_init(data);
    ret = devm_iio_device_register(dev, indio_dev);
    if (ret) {
    dev_err(dev, "failed to register iio device\n");
    return ret;
    }
    data.iaq_thread = kthread_run(sgp_iaq_threadfn, data,
    "%s-iaq", data.client.name);
    if (IS_ERR(data.iaq_thread))
    return dev_err_probe(dev, PTR_ERR(data.iaq_thread),
    "failed to start IAQ thread\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sgp_remove(client: *mut i2c_client) {
    static void sgp_remove(struct i2c_client *client)
    {
    struct iio_dev *indio_dev = i2c_get_clientdata(client);
    struct sgp_data *data = iio_priv(indio_dev);
    if (data.iaq_thread)
    kthread_stop(data.iaq_thread);
    }
    static const struct i2c_device_id sgp_id[] = {
    { .name = "sgp30", .driver_data = (kernel_ulong_t)&sgp_devices[SGP30] },
    { .name = "sgpc3", .driver_data = (kernel_ulong_t)&sgp_devices[SGPC3] },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, sgp_id);
    static struct i2c_driver sgp_driver = {
    .driver = {
    .name = "sgp30",
    .of_match_table = sgp_dt_ids,
    },
    .probe = sgp_probe,
    .remove = sgp_remove,
    .id_table = sgp_id,
    };
    module_i2c_driver(sgp_driver);
    MODULE_AUTHOR("Andreas Brauchli <andreas.brauchli@sensirion.com>");
    MODULE_AUTHOR("Pascal Sachs <pascal.sachs@sensirion.com>");
    MODULE_DESCRIPTION("Sensirion SGP gas sensors");
    MODULE_LICENSE("GPL v2");
