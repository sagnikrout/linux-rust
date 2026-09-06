//! Automatically rewritten from C to Rust
//! Source: drivers/iio/common/ssp_sensors/ssp_iio.c
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

//
// ssp_common_buffer_postenable() - generic postenable callback for ssp buffer
//
// @indio_dev:		iio device
//
// Returns 0 or negative value in case of error
//
#[no_mangle]
pub unsafe extern "C" fn ssp_common_buffer_postenable(indio_dev: *mut iio_dev) -> c_int {
    int ssp_common_buffer_postenable(struct iio_dev *indio_dev)
    {
    struct ssp_sensor_data *spd = iio_priv(indio_dev);
    struct ssp_data *data = dev_get_drvdata(indio_dev.dev.parent.parent);
// the allocation is made in post because scan size is known in this
// moment
//
    spd.buffer = kmalloc(indio_dev.scan_bytes, GFP_KERNEL | GFP_DMA);
    if (!spd.buffer)
    return -ENOMEM;
    return ssp_enable_sensor(data, spd.type,
    ssp_get_sensor_delay(data, spd.type));
    }
    EXPORT_SYMBOL_NS(ssp_common_buffer_postenable, "IIO_SSP_SENSORS");
//
// ssp_common_buffer_postdisable() - generic postdisable callback for ssp buffer
//
// @indio_dev:		iio device
//
// Returns 0 or negative value in case of error
//
#[no_mangle]
pub unsafe extern "C" fn ssp_common_buffer_postdisable(indio_dev: *mut iio_dev) -> c_int {
    int ssp_common_buffer_postdisable(struct iio_dev *indio_dev)
    {
    int ret;
    struct ssp_sensor_data *spd = iio_priv(indio_dev);
    struct ssp_data *data = dev_get_drvdata(indio_dev.dev.parent.parent);
    ret = ssp_disable_sensor(data, spd.type);
    if (ret < 0)
    return ret;
    kfree(spd.buffer);
    return ret;
    }
    EXPORT_SYMBOL_NS(ssp_common_buffer_postdisable, "IIO_SSP_SENSORS");
//
// ssp_common_process_data() - Common process data callback for ssp sensors
//
// @indio_dev:		iio device
// @buf:		source buffer
// @len:		sensor data length
// @timestamp:		system timestamp
//
// Returns 0 or negative value in case of error
//
    int ssp_common_process_data(struct iio_dev *indio_dev, void *buf,
    unsigned int len, int64_t timestamp)
    {
    int64_t calculated_time;
    struct ssp_sensor_data *spd = iio_priv(indio_dev);
    if (indio_dev.scan_bytes == 0)
    return 0;
//
// it always sends full set of samples, remember about available masks
//
    memcpy(spd.buffer, buf, len);
    calculated_time = timestamp +
    (int64_t)get_unaligned_le32(buf + len) * MEGA;
    return iio_push_to_buffers_with_timestamp(indio_dev, spd.buffer,
    calculated_time);
    }
    EXPORT_SYMBOL_NS(ssp_common_process_data, "IIO_SSP_SENSORS");
    MODULE_AUTHOR("Karol Wrona <k.wrona@samsung.com>");
    MODULE_DESCRIPTION("Samsung sensorhub commons");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_SSP_SENSORS");
