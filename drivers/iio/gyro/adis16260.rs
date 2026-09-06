//! Automatically rewritten from C to Rust
//! Source: drivers/iio/gyro/adis16260.c
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
// ADIS16260/ADIS16265 Programmable Digital Gyroscope Sensor Driver
//
// Copyright 2010 Analog Devices Inc.
//

pub const ADIS16260_FLASH_CNT: c_uint = 0x00 /* Flash memory write count */;
pub const ADIS16260_SUPPLY_OUT: c_uint = 0x02 /* Power supply measurement */;
pub const ADIS16260_GYRO_OUT: c_uint = 0x04 /* X-axis gyroscope output */;
pub const ADIS16260_AUX_ADC: c_uint = 0x0A /* analog input channel measurement */;
pub const ADIS16260_TEMP_OUT: c_uint = 0x0C /* internal temperature measurement */;
pub const ADIS16260_ANGL_OUT: c_uint = 0x0E /* angle displacement */;
pub const ADIS16260_GYRO_OFF: c_uint = 0x14 /* Calibration, offset/bias adjustment */;
pub const ADIS16260_GYRO_SCALE: c_uint = 0x16 /* Calibration, scale adjustment */;
pub const ADIS16260_ALM_MAG1: c_uint = 0x20 /* Alarm 1 magnitude/polarity setting */;
pub const ADIS16260_ALM_MAG2: c_uint = 0x22 /* Alarm 2 magnitude/polarity setting */;
pub const ADIS16260_ALM_SMPL1: c_uint = 0x24 /* Alarm 1 dynamic rate of change setting */;
pub const ADIS16260_ALM_SMPL2: c_uint = 0x26 /* Alarm 2 dynamic rate of change setting */;
pub const ADIS16260_ALM_CTRL: c_uint = 0x28 /* Alarm control */;
pub const ADIS16260_AUX_DAC: c_uint = 0x30 /* Auxiliary DAC data */;
pub const ADIS16260_GPIO_CTRL: c_uint = 0x32 /* Control, digital I/O line */;
pub const ADIS16260_MSC_CTRL: c_uint = 0x34 /* Control, data ready, self-test settings */;
pub const ADIS16260_SMPL_PRD: c_uint = 0x36 /* Control, internal sample rate */;
pub const ADIS16260_SENS_AVG: c_uint = 0x38 /* Control, dynamic range, filtering */;
pub const ADIS16260_SLP_CNT: c_uint = 0x3A /* Control, sleep mode initiation */;
pub const ADIS16260_DIAG_STAT: c_uint = 0x3C /* Diagnostic, error flags */;
pub const ADIS16260_GLOB_CMD: c_uint = 0x3E /* Control, global commands */;
pub const ADIS16260_LOT_ID1: c_uint = 0x52 /* Lot Identification Code 1 */;
pub const ADIS16260_LOT_ID2: c_uint = 0x54 /* Lot Identification Code 2 */;
pub const ADIS16260_PROD_ID: c_uint = 0x56 /* Product identifier;;
// convert to decimal = 16,265/16,260
pub const ADIS16260_SERIAL_NUM: c_uint = 0x58 /* Serial number */;

// MSC_CTRL

// Internal self-test enable

// SMPL_PRD
// Time base (tB): 0 = 1.953 ms, 1 = 60.54 ms

pub const ADIS16260_SMPL_PRD_DIV_MASK: c_uint = 0x7F;
// SLP_CNT
pub const ADIS16260_SLP_CNT_POWER_OFF: c_uint = 0x80;
// DIAG_STAT

pub const ADIS16260_DIAG_STAT_FLASH_CHK_BIT: c_int = 6;
pub const ADIS16260_DIAG_STAT_SELF_TEST_BIT: c_int = 5;
pub const ADIS16260_DIAG_STAT_OVERFLOW_BIT: c_int = 4;
pub const ADIS16260_DIAG_STAT_SPI_FAIL_BIT: c_int = 3;
pub const ADIS16260_DIAG_STAT_FLASH_UPT_BIT: c_int = 2;
pub const ADIS16260_DIAG_STAT_POWER_HIGH_BIT: c_int = 1;
pub const ADIS16260_DIAG_STAT_POWER_LOW_BIT: c_int = 0;
// GLOB_CMD

// At the moment triggers are only used for ring buffer
// filling. This may change!
//
pub const ADIS16260_SCAN_GYRO: c_int = 0;
pub const ADIS16260_SCAN_SUPPLY: c_int = 1;
pub const ADIS16260_SCAN_AUX_ADC: c_int = 2;
pub const ADIS16260_SCAN_TEMP: c_int = 3;
pub const ADIS16260_SCAN_ANGL: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adis16260_chip_info {
    pub gyro_max_val: c_uint,
    pub gyro_max_scale: c_uint,
    pub channels: *const iio_chan_spec,
    pub num_channels: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adis16260 {
    pub info: *const adis16260_chip_info,
    pub adis: adis,
}

    enum adis16260_type {
    ADIS16251,
    ADIS16260,
    ADIS16266,
    };
    static const struct iio_chan_spec adis16260_channels[] = {
    ADIS_GYRO_CHAN(X, ADIS16260_GYRO_OUT, ADIS16260_SCAN_GYRO,
    BIT(IIO_CHAN_INFO_CALIBBIAS) |
    BIT(IIO_CHAN_INFO_CALIBSCALE),
    BIT(IIO_CHAN_INFO_SAMP_FREQ), 14),
    ADIS_INCLI_CHAN(X, ADIS16260_ANGL_OUT, ADIS16260_SCAN_ANGL, 0,
    BIT(IIO_CHAN_INFO_SAMP_FREQ), 14),
    ADIS_TEMP_CHAN(ADIS16260_TEMP_OUT, ADIS16260_SCAN_TEMP,
    BIT(IIO_CHAN_INFO_SAMP_FREQ), 12),
    ADIS_SUPPLY_CHAN(ADIS16260_SUPPLY_OUT, ADIS16260_SCAN_SUPPLY,
    BIT(IIO_CHAN_INFO_SAMP_FREQ), 12),
    ADIS_AUX_ADC_CHAN(ADIS16260_AUX_ADC, ADIS16260_SCAN_AUX_ADC,
    BIT(IIO_CHAN_INFO_SAMP_FREQ), 12),
    IIO_CHAN_SOFT_TIMESTAMP(5),
    };
    static const struct iio_chan_spec adis16266_channels[] = {
    ADIS_GYRO_CHAN(X, ADIS16260_GYRO_OUT, ADIS16260_SCAN_GYRO,
    BIT(IIO_CHAN_INFO_CALIBBIAS) |
    BIT(IIO_CHAN_INFO_CALIBSCALE),
    BIT(IIO_CHAN_INFO_SAMP_FREQ), 14),
    ADIS_TEMP_CHAN(ADIS16260_TEMP_OUT, ADIS16260_SCAN_TEMP,
    BIT(IIO_CHAN_INFO_SAMP_FREQ), 12),
    ADIS_SUPPLY_CHAN(ADIS16260_SUPPLY_OUT, ADIS16260_SCAN_SUPPLY,
    BIT(IIO_CHAN_INFO_SAMP_FREQ), 12),
    ADIS_AUX_ADC_CHAN(ADIS16260_AUX_ADC, ADIS16260_SCAN_AUX_ADC,
    BIT(IIO_CHAN_INFO_SAMP_FREQ), 12),
    IIO_CHAN_SOFT_TIMESTAMP(4),
    };
    static const struct adis16260_chip_info adis16260_chip_info_table[] = {
    [ADIS16251] = {
    .gyro_max_scale = 80,
    .gyro_max_val = IIO_RAD_TO_DEGREE(4368),
    .channels = adis16260_channels,
    .num_channels = ARRAY_SIZE(adis16260_channels),
    },
    [ADIS16260] = {
    .gyro_max_scale = 320,
    .gyro_max_val = IIO_RAD_TO_DEGREE(4368),
    .channels = adis16260_channels,
    .num_channels = ARRAY_SIZE(adis16260_channels),
    },
    [ADIS16266] = {
    .gyro_max_scale = 14000,
    .gyro_max_val = IIO_RAD_TO_DEGREE(3357),
    .channels = adis16266_channels,
    .num_channels = ARRAY_SIZE(adis16266_channels),
    },
    };
// Power down the device
#[no_mangle]
unsafe extern "C" fn adis16260_stop_device(indio_dev: *mut iio_dev) -> c_int {
    static int adis16260_stop_device(struct iio_dev *indio_dev)
    {
    struct adis16260 *adis16260 = iio_priv(indio_dev);
    int ret;
    let mut val: u16 = ADIS16260_SLP_CNT_POWER_OFF;
    ret = adis_write_reg_16(&adis16260.adis, ADIS16260_SLP_CNT, val);
    if (ret)
    dev_err(&indio_dev.dev, "problem with turning device off: SLP_CNT");
    return ret;
    }
    static const u8 adis16260_addresses[][2] = {
    [ADIS16260_SCAN_GYRO] = { ADIS16260_GYRO_OFF, ADIS16260_GYRO_SCALE },
    };
    static int adis16260_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2,
    long mask)
    {
    struct adis16260 *adis16260 = iio_priv(indio_dev);
    const struct adis16260_chip_info *info = adis16260.info;
    struct adis *adis = &adis16260.adis;
    int ret;
    u8 addr;
    s16 val16;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    return adis_single_conversion(indio_dev, chan,
    ADIS16260_ERROR_ACTIVE, val);
    case IIO_CHAN_INFO_SCALE:
    switch (chan.type) {
    case IIO_ANGL_VEL:
// val = info->gyro_max_scale;
// val2 = info->gyro_max_val;
    return IIO_VAL_FRACTIONAL;
    case IIO_INCLI:
// val = 0;
// val2 = IIO_DEGREE_TO_RAD(36630);
    return IIO_VAL_INT_PLUS_MICRO;
    case IIO_VOLTAGE:
    if (chan.channel == 0) {
// val = 1;
// val2 = 831500; /* 1.8315 mV
    } else {
// val = 0;
// val2 = 610500; /* 610.5 uV
    }
    return IIO_VAL_INT_PLUS_MICRO;
    case IIO_TEMP:
// val = 145;
// val2 = 300000; /* 0.1453 C
    return IIO_VAL_INT_PLUS_MICRO;
    default:
    return -EINVAL;
    }
    case IIO_CHAN_INFO_OFFSET:
// val = 250000 / 1453; /* 25 C = 0x00
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_CALIBBIAS:
    addr = adis16260_addresses[chan.scan_index][0];
    ret = adis_read_reg_16(adis, addr, &val16);
    if (ret)
    return ret;
// val = sign_extend32(val16, 11);
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_CALIBSCALE:
    addr = adis16260_addresses[chan.scan_index][1];
    ret = adis_read_reg_16(adis, addr, &val16);
    if (ret)
    return ret;
// val = val16;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SAMP_FREQ:
    ret = adis_read_reg_16(adis, ADIS16260_SMPL_PRD, &val16);
    if (ret)
    return ret;
    if (spi_get_device_id(adis.spi).driver_data)
// If an adis16251
// val = (val16 & ADIS16260_SMPL_PRD_TIME_BASE) ?
    8 : 256;
    else
// val = (val16 & ADIS16260_SMPL_PRD_TIME_BASE) ?
    66 : 2048;
// val /= (val16 & ADIS16260_SMPL_PRD_DIV_MASK) + 1;
    return IIO_VAL_INT;
    }
    return -EINVAL;
    }
    static int adis16260_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val,
    int val2,
    long mask)
    {
    struct adis16260 *adis16260 = iio_priv(indio_dev);
    struct adis *adis = &adis16260.adis;
    u8 addr;
    u8 t;
    switch (mask) {
    case IIO_CHAN_INFO_CALIBBIAS:
    if (val < -2048 || val >= 2048)
    return -EINVAL;
    addr = adis16260_addresses[chan.scan_index][0];
    return adis_write_reg_16(adis, addr, val);
    case IIO_CHAN_INFO_CALIBSCALE:
    if (val < 0 || val >= 4096)
    return -EINVAL;
    addr = adis16260_addresses[chan.scan_index][1];
    return adis_write_reg_16(adis, addr, val);
    case IIO_CHAN_INFO_SAMP_FREQ:
    if (val <= 0)
    return -EINVAL;
    if (spi_get_device_id(adis.spi).driver_data)
    t = 256 / val;
    else
    t = 2048 / val;
    if (t > ADIS16260_SMPL_PRD_DIV_MASK)
    t = ADIS16260_SMPL_PRD_DIV_MASK;
#[no_mangle]
pub unsafe extern "C" fn if(0: t >) -> else {
    else if (t > 0)
    t--;
    adis_dev_auto_scoped_lock(adis) {
    if (t >= 0x0A)
    adis.spi.max_speed_hz = ADIS16260_SPI_SLOW;
    else
    adis.spi.max_speed_hz = ADIS16260_SPI_FAST;
    return __adis_write_reg_8(adis, ADIS16260_SMPL_PRD, t);
    }
    unreachable();
    }
    return -EINVAL;
    }
    static const struct iio_info adis16260_info = {
    .read_raw = &adis16260_read_raw,
    .write_raw = &adis16260_write_raw,
    .update_scan_mode = adis_update_scan_mode,
    };
    static const char * const adis1620_status_error_msgs[] = {
    [ADIS16260_DIAG_STAT_FLASH_CHK_BIT] = "Flash checksum error",
    [ADIS16260_DIAG_STAT_SELF_TEST_BIT] = "Self test error",
    [ADIS16260_DIAG_STAT_OVERFLOW_BIT] = "Sensor overrange",
    [ADIS16260_DIAG_STAT_SPI_FAIL_BIT] = "SPI failure",
    [ADIS16260_DIAG_STAT_FLASH_UPT_BIT] = "Flash update failed",
    [ADIS16260_DIAG_STAT_POWER_HIGH_BIT] = "Power supply above 5.25",
    [ADIS16260_DIAG_STAT_POWER_LOW_BIT] = "Power supply below 4.75",
    };
    static const struct adis_timeout adis16260_timeouts = {
    .reset_ms = ADIS16260_STARTUP_DELAY,
    .sw_reset_ms = ADIS16260_STARTUP_DELAY,
    .self_test_ms = ADIS16260_STARTUP_DELAY,
    };
    static const struct adis_data adis16260_data = {
    .write_delay = 30,
    .read_delay = 30,
    .msc_ctrl_reg = ADIS16260_MSC_CTRL,
    .glob_cmd_reg = ADIS16260_GLOB_CMD,
    .diag_stat_reg = ADIS16260_DIAG_STAT,
    .self_test_mask = ADIS16260_MSC_CTRL_MEM_TEST,
    .self_test_reg = ADIS16260_MSC_CTRL,
    .timeouts = &adis16260_timeouts,
    .status_error_msgs = adis1620_status_error_msgs,
    .status_error_mask = BIT(ADIS16260_DIAG_STAT_FLASH_CHK_BIT) |
    BIT(ADIS16260_DIAG_STAT_SELF_TEST_BIT) |
    BIT(ADIS16260_DIAG_STAT_OVERFLOW_BIT) |
    BIT(ADIS16260_DIAG_STAT_SPI_FAIL_BIT) |
    BIT(ADIS16260_DIAG_STAT_FLASH_UPT_BIT) |
    BIT(ADIS16260_DIAG_STAT_POWER_HIGH_BIT) |
    BIT(ADIS16260_DIAG_STAT_POWER_LOW_BIT),
    };
#[no_mangle]
unsafe extern "C" fn adis16260_stop(data: *mut c_void) {
    static void adis16260_stop(void *data)
    {
    adis16260_stop_device(data);
    }
#[no_mangle]
unsafe extern "C" fn adis16260_probe(spi: *mut spi_device) -> c_int {
    static int adis16260_probe(struct spi_device *spi)
    {
    const struct spi_device_id *id;
    struct adis16260 *adis16260;
    struct iio_dev *indio_dev;
    int ret;
    id = spi_get_device_id(spi);
    if (!id)
    return -ENODEV;
// setup the industrialio driver allocated elements
    indio_dev = devm_iio_device_alloc(&spi.dev, sizeof(*adis16260));
    if (!indio_dev)
    return -ENOMEM;
    adis16260 = iio_priv(indio_dev);
// this is only used for removal purposes
    spi_set_drvdata(spi, indio_dev);
    adis16260.info = &adis16260_chip_info_table[id.driver_data];
    indio_dev.name = id.name;
    indio_dev.info = &adis16260_info;
    indio_dev.channels = adis16260.info.channels;
    indio_dev.num_channels = adis16260.info.num_channels;
    indio_dev.modes = INDIO_DIRECT_MODE;
    ret = adis_init(&adis16260.adis, indio_dev, spi, &adis16260_data);
    if (ret)
    return ret;
    ret = devm_adis_setup_buffer_and_trigger(&adis16260.adis, indio_dev, core::ptr::null_mut());
    if (ret)
    return ret;
// Get the device into a sane initial state
    ret = __adis_initial_startup(&adis16260.adis);
    if (ret)
    return ret;
    ret = devm_add_action_or_reset(&spi.dev, adis16260_stop, indio_dev);
    if (ret)
    return ret;
    return devm_iio_device_register(&spi.dev, indio_dev);
    }
//
// These parts do not need to be differentiated until someone adds
// support for the on chip filtering.
//
    static const struct spi_device_id adis16260_id[] = {
    { .name = "adis16260", .driver_data = ADIS16260 },
    { .name = "adis16265", .driver_data = ADIS16260 },
    { .name = "adis16266", .driver_data = ADIS16266 },
    { .name = "adis16250", .driver_data = ADIS16260 },
    { .name = "adis16255", .driver_data = ADIS16260 },
    { .name = "adis16251", .driver_data = ADIS16251 },
    { }
    };
    MODULE_DEVICE_TABLE(spi, adis16260_id);
    static struct spi_driver adis16260_driver = {
    .driver = {
    .name = "adis16260",
    },
    .probe = adis16260_probe,
    .id_table = adis16260_id,
    };
    module_spi_driver(adis16260_driver);
    MODULE_AUTHOR("Barry Song <21cnbao@gmail.com>");
    MODULE_DESCRIPTION("Analog Devices ADIS16260/5 Digital Gyroscope Sensor");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_ADISLIB");
