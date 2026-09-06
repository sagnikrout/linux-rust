//! Automatically rewritten from C to Rust
//! Source: drivers/iio/gyro/ssp_gyro_sensor.c
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
// Copyright (C) 2014, Samsung Electronics Co. Ltd. All Rights Reserved.
//

pub const SSP_CHANNEL_COUNT: c_int = 3;

    static const char ssp_gyro_name[] = SSP_GYROSCOPE_NAME;
    enum ssp_gyro_3d_channel {
    SSP_CHANNEL_SCAN_INDEX_X,
    SSP_CHANNEL_SCAN_INDEX_Y,
    SSP_CHANNEL_SCAN_INDEX_Z,
    SSP_CHANNEL_SCAN_INDEX_TIME,
    };
    static int ssp_gyro_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int *val,
    int *val2, long mask)
    {
    u32 t;
    struct ssp_data *data = dev_get_drvdata(indio_dev.dev.parent.parent);
    switch (mask) {
    case IIO_CHAN_INFO_SAMP_FREQ:
    t = ssp_get_sensor_delay(data, SSP_GYROSCOPE_SENSOR);
    ssp_convert_to_freq(t, val, val2);
    return IIO_VAL_INT_PLUS_MICRO;
    default:
    break;
    }
    return -EINVAL;
    }
    static int ssp_gyro_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int val,
    int val2, long mask)
    {
    int ret;
    struct ssp_data *data = dev_get_drvdata(indio_dev.dev.parent.parent);
    switch (mask) {
    case IIO_CHAN_INFO_SAMP_FREQ:
    ret = ssp_convert_to_time(val, val2);
    ret = ssp_change_delay(data, SSP_GYROSCOPE_SENSOR, ret);
    if (ret < 0)
    dev_err(&indio_dev.dev, "gyro sensor enable fail\n");
    return ret;
    default:
    break;
    }
    return -EINVAL;
    }
    static const struct iio_info ssp_gyro_iio_info = {
    .read_raw = &ssp_gyro_read_raw,
    .write_raw = &ssp_gyro_write_raw,
    };
    static const unsigned long ssp_gyro_scan_mask[] = { 0x07, 0, };
    static const struct iio_chan_spec ssp_gyro_channels[] = {
    SSP_CHANNEL_AG(IIO_ANGL_VEL, IIO_MOD_X, SSP_CHANNEL_SCAN_INDEX_X),
    SSP_CHANNEL_AG(IIO_ANGL_VEL, IIO_MOD_Y, SSP_CHANNEL_SCAN_INDEX_Y),
    SSP_CHANNEL_AG(IIO_ANGL_VEL, IIO_MOD_Z, SSP_CHANNEL_SCAN_INDEX_Z),
    IIO_CHAN_SOFT_TIMESTAMP(SSP_CHANNEL_SCAN_INDEX_TIME),
    };
    static int ssp_process_gyro_data(struct iio_dev *indio_dev, void *buf,
    int64_t timestamp)
    {
    return ssp_common_process_data(indio_dev, buf, SSP_GYROSCOPE_SIZE,
    timestamp);
    }
    static const struct iio_buffer_setup_ops ssp_gyro_buffer_ops = {
    .postenable = &ssp_common_buffer_postenable,
    .postdisable = &ssp_common_buffer_postdisable,
    };
#[no_mangle]
unsafe extern "C" fn ssp_gyro_probe(pdev: *mut platform_device) -> c_int {
    static int ssp_gyro_probe(struct platform_device *pdev)
    {
    int ret;
    struct iio_dev *indio_dev;
    struct ssp_sensor_data *spd;
    indio_dev = devm_iio_device_alloc(&pdev.dev, sizeof(*spd));
    if (!indio_dev)
    return -ENOMEM;
    spd = iio_priv(indio_dev);
    spd.process_data = ssp_process_gyro_data;
    spd.type = SSP_GYROSCOPE_SENSOR;
    indio_dev.name = ssp_gyro_name;
    indio_dev.info = &ssp_gyro_iio_info;
    indio_dev.channels = ssp_gyro_channels;
    indio_dev.num_channels = ARRAY_SIZE(ssp_gyro_channels);
    indio_dev.available_scan_masks = ssp_gyro_scan_mask;
    ret = devm_iio_kfifo_buffer_setup(&pdev.dev, indio_dev,
    &ssp_gyro_buffer_ops);
    if (ret)
    return ret;
    platform_set_drvdata(pdev, indio_dev);
    ret = devm_iio_device_register(&pdev.dev, indio_dev);
    if (ret < 0)
    return ret;
// ssp registering should be done after all iio setup
    ssp_register_consumer(indio_dev, SSP_GYROSCOPE_SENSOR);
    return 0;
    }
    static struct platform_driver ssp_gyro_driver = {
    .driver = {
    .name = SSP_GYROSCOPE_NAME,
    },
    .probe = ssp_gyro_probe,
    };
    module_platform_driver(ssp_gyro_driver);
    MODULE_AUTHOR("Karol Wrona <k.wrona@samsung.com>");
    MODULE_DESCRIPTION("Samsung sensorhub gyroscopes driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_SSP_SENSORS");
