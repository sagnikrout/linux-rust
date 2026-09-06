//! Automatically rewritten from C to Rust
//! Source: drivers/iio/imu/adis16460.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// ADIS16460 IMU driver
//
// Copyright 2019 Analog Devices Inc.
//

pub const ADIS16460_REG_FLASH_CNT: c_uint = 0x00;
pub const ADIS16460_REG_DIAG_STAT: c_uint = 0x02;
pub const ADIS16460_REG_X_GYRO_LOW: c_uint = 0x04;
pub const ADIS16460_REG_X_GYRO_OUT: c_uint = 0x06;
pub const ADIS16460_REG_Y_GYRO_LOW: c_uint = 0x08;
pub const ADIS16460_REG_Y_GYRO_OUT: c_uint = 0x0A;
pub const ADIS16460_REG_Z_GYRO_LOW: c_uint = 0x0C;
pub const ADIS16460_REG_Z_GYRO_OUT: c_uint = 0x0E;
pub const ADIS16460_REG_X_ACCL_LOW: c_uint = 0x10;
pub const ADIS16460_REG_X_ACCL_OUT: c_uint = 0x12;
pub const ADIS16460_REG_Y_ACCL_LOW: c_uint = 0x14;
pub const ADIS16460_REG_Y_ACCL_OUT: c_uint = 0x16;
pub const ADIS16460_REG_Z_ACCL_LOW: c_uint = 0x18;
pub const ADIS16460_REG_Z_ACCL_OUT: c_uint = 0x1A;
pub const ADIS16460_REG_SMPL_CNTR: c_uint = 0x1C;
pub const ADIS16460_REG_TEMP_OUT: c_uint = 0x1E;
pub const ADIS16460_REG_X_DELT_ANG: c_uint = 0x24;
pub const ADIS16460_REG_Y_DELT_ANG: c_uint = 0x26;
pub const ADIS16460_REG_Z_DELT_ANG: c_uint = 0x28;
pub const ADIS16460_REG_X_DELT_VEL: c_uint = 0x2A;
pub const ADIS16460_REG_Y_DELT_VEL: c_uint = 0x2C;
pub const ADIS16460_REG_Z_DELT_VEL: c_uint = 0x2E;
pub const ADIS16460_REG_MSC_CTRL: c_uint = 0x32;
pub const ADIS16460_REG_SYNC_SCAL: c_uint = 0x34;
pub const ADIS16460_REG_DEC_RATE: c_uint = 0x36;
pub const ADIS16460_REG_FLTR_CTRL: c_uint = 0x38;
pub const ADIS16460_REG_GLOB_CMD: c_uint = 0x3E;
pub const ADIS16460_REG_X_GYRO_OFF: c_uint = 0x40;
pub const ADIS16460_REG_Y_GYRO_OFF: c_uint = 0x42;
pub const ADIS16460_REG_Z_GYRO_OFF: c_uint = 0x44;
pub const ADIS16460_REG_X_ACCL_OFF: c_uint = 0x46;
pub const ADIS16460_REG_Y_ACCL_OFF: c_uint = 0x48;
pub const ADIS16460_REG_Z_ACCL_OFF: c_uint = 0x4A;
pub const ADIS16460_REG_LOT_ID1: c_uint = 0x52;
pub const ADIS16460_REG_LOT_ID2: c_uint = 0x54;
pub const ADIS16460_REG_PROD_ID: c_uint = 0x56;
pub const ADIS16460_REG_SERIAL_NUM: c_uint = 0x58;
pub const ADIS16460_REG_CAL_SGNTR: c_uint = 0x60;
pub const ADIS16460_REG_CAL_CRC: c_uint = 0x62;
pub const ADIS16460_REG_CODE_SGNTR: c_uint = 0x64;
pub const ADIS16460_REG_CODE_CRC: c_uint = 0x66;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adis16460_chip_info {
    pub num_channels: c_uint,
    pub channels: *const iio_chan_spec,
    pub gyro_max_val: c_uint,
    pub gyro_max_scale: c_uint,
    pub accel_max_val: c_uint,
    pub accel_max_scale: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adis16460 {
    pub chip_info: *const adis16460_chip_info,
    pub adis: adis,
}

#[no_mangle]
unsafe extern "C" fn adis16460_show_serial_number(arg: *mut c_void, val: *mut u64) -> c_int {
    static int adis16460_show_serial_number(void *arg, u64 *val)
    {
    struct adis16460 *adis16460 = arg;
    u16 serial;
    int ret;
    ret = adis_read_reg_16(&adis16460.adis, ADIS16460_REG_SERIAL_NUM,
    &serial);
    if (ret)
    return ret;
// val = serial;
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(adis16460_serial_number_fops,
    adis16460_show_serial_number, core::ptr::null_mut(), "0x%.4llx\n");
#[no_mangle]
unsafe extern "C" fn adis16460_show_product_id(arg: *mut c_void, val: *mut u64) -> c_int {
    static int adis16460_show_product_id(void *arg, u64 *val)
    {
    struct adis16460 *adis16460 = arg;
    u16 prod_id;
    int ret;
    ret = adis_read_reg_16(&adis16460.adis, ADIS16460_REG_PROD_ID,
    &prod_id);
    if (ret)
    return ret;
// val = prod_id;
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(adis16460_product_id_fops,
    adis16460_show_product_id, core::ptr::null_mut(), "%llu\n");
#[no_mangle]
unsafe extern "C" fn adis16460_show_flash_count(arg: *mut c_void, val: *mut u64) -> c_int {
    static int adis16460_show_flash_count(void *arg, u64 *val)
    {
    struct adis16460 *adis16460 = arg;
    u32 flash_count;
    int ret;
    ret = adis_read_reg_32(&adis16460.adis, ADIS16460_REG_FLASH_CNT,
    &flash_count);
    if (ret)
    return ret;
// val = flash_count;
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(adis16460_flash_count_fops,
    adis16460_show_flash_count, core::ptr::null_mut(), "%lld\n");
#[no_mangle]
unsafe extern "C" fn adis16460_debugfs_init(indio_dev: *mut iio_dev) {
    static void adis16460_debugfs_init(struct iio_dev *indio_dev)
    {
    struct adis16460 *adis16460 = iio_priv(indio_dev);
    struct dentry *d = iio_get_debugfs_dentry(indio_dev);
    if (!IS_ENABLED(CONFIG_DEBUG_FS))
    return;
    debugfs_create_file_unsafe("serial_number", 0400,
    d, adis16460, &adis16460_serial_number_fops);
    debugfs_create_file_unsafe("product_id", 0400,
    d, adis16460, &adis16460_product_id_fops);
    debugfs_create_file_unsafe("flash_count", 0400,
    d, adis16460, &adis16460_flash_count_fops);
    }
#[no_mangle]
unsafe extern "C" fn adis16460_set_freq(indio_dev: *mut iio_dev, val: c_int, val2: c_int) -> c_int {
    static int adis16460_set_freq(struct iio_dev *indio_dev, int val, int val2)
    {
    struct adis16460 *st = iio_priv(indio_dev);
    int t;
    t =  val * 1000 + val2 / 1000;
    if (t <= 0)
    return -EINVAL;
    t = 2048000 / t;
    if (t > 2048)
    t = 2048;
    if (t != 0)
    t--;
    return adis_write_reg_16(&st.adis, ADIS16460_REG_DEC_RATE, t);
    }
#[no_mangle]
unsafe extern "C" fn adis16460_get_freq(indio_dev: *mut iio_dev, val: *mut c_int, val2: *mut c_int) -> c_int {
    static int adis16460_get_freq(struct iio_dev *indio_dev, int *val, int *val2)
    {
    struct adis16460 *st = iio_priv(indio_dev);
    uint16_t t;
    int ret;
    unsigned int freq;
    ret = adis_read_reg_16(&st.adis, ADIS16460_REG_DEC_RATE, &t);
    if (ret)
    return ret;
    freq = 2048000 / (t + 1);
// val = freq / 1000;
// val2 = (freq % 1000) * 1000;
    return IIO_VAL_INT_PLUS_MICRO;
    }
    static int adis16460_read_raw(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan, int *val, int *val2, long info)
    {
    struct adis16460 *st = iio_priv(indio_dev);
    switch (info) {
    case IIO_CHAN_INFO_RAW:
    return adis_single_conversion(indio_dev, chan, 0, val);
    case IIO_CHAN_INFO_SCALE:
    switch (chan.type) {
    case IIO_ANGL_VEL:
// val = st->chip_info->gyro_max_scale;
// val2 = st->chip_info->gyro_max_val;
    return IIO_VAL_FRACTIONAL;
    case IIO_ACCEL:
// val = st->chip_info->accel_max_scale;
// val2 = st->chip_info->accel_max_val;
    return IIO_VAL_FRACTIONAL;
    case IIO_TEMP:
// val = 50; /* 50 milli degrees Celsius/LSB
    return IIO_VAL_INT;
    default:
    return -EINVAL;
    }
    case IIO_CHAN_INFO_OFFSET:
// val = 500; /* 25 degrees Celsius = 0x0000
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SAMP_FREQ:
    return adis16460_get_freq(indio_dev, val, val2);
    default:
    return -EINVAL;
    }
    }
    static int adis16460_write_raw(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan, int val, int val2, long info)
    {
    switch (info) {
    case IIO_CHAN_INFO_SAMP_FREQ:
    return adis16460_set_freq(indio_dev, val, val2);
    default:
    return -EINVAL;
    }
    }
    enum {
    ADIS16460_SCAN_GYRO_X,
    ADIS16460_SCAN_GYRO_Y,
    ADIS16460_SCAN_GYRO_Z,
    ADIS16460_SCAN_ACCEL_X,
    ADIS16460_SCAN_ACCEL_Y,
    ADIS16460_SCAN_ACCEL_Z,
    ADIS16460_SCAN_TEMP,
    };

    { \
    .type = (_type), \
    .modified = 1, \
    .channel2 = (_mod), \
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW), \
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE), \
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_SAMP_FREQ), \
    .address = (_address), \
    .scan_index = (_si), \
    .scan_type = { \
    .sign = 's', \
    .realbits = (_bits), \
    .storagebits = (_bits), \
    .endianness = IIO_BE, \
    }, \
    }

    ADIS16460_MOD_CHANNEL(IIO_ANGL_VEL, IIO_MOD_ ## _mod, \
    ADIS16460_REG_ ## _mod ## _GYRO_LOW, ADIS16460_SCAN_GYRO_ ## _mod, \
    32)

    ADIS16460_MOD_CHANNEL(IIO_ACCEL, IIO_MOD_ ## _mod, \
    ADIS16460_REG_ ## _mod ## _ACCL_LOW, ADIS16460_SCAN_ACCEL_ ## _mod, \
    32)

    .type = IIO_TEMP, \
    .indexed = 1, \
    .channel = 0, \
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) | \
    BIT(IIO_CHAN_INFO_SCALE) | \
    BIT(IIO_CHAN_INFO_OFFSET), \
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_SAMP_FREQ), \
    .address = ADIS16460_REG_TEMP_OUT, \
    .scan_index = ADIS16460_SCAN_TEMP, \
    .scan_type = { \
    .sign = 's', \
    .realbits = 16, \
    .storagebits = 16, \
    .endianness = IIO_BE, \
    }, \
    }
    static const struct iio_chan_spec adis16460_channels[] = {
    ADIS16460_GYRO_CHANNEL(X),
    ADIS16460_GYRO_CHANNEL(Y),
    ADIS16460_GYRO_CHANNEL(Z),
    ADIS16460_ACCEL_CHANNEL(X),
    ADIS16460_ACCEL_CHANNEL(Y),
    ADIS16460_ACCEL_CHANNEL(Z),
    ADIS16460_TEMP_CHANNEL(),
    IIO_CHAN_SOFT_TIMESTAMP(7)
    };
    static const struct adis16460_chip_info adis16460_chip_info = {
    .channels = adis16460_channels,
    .num_channels = ARRAY_SIZE(adis16460_channels),
//
// storing the value in rad/degree and the scale in degree
// gives us the result in rad and better precession than
// storing the scale directly in rad.
//
    .gyro_max_val = IIO_RAD_TO_DEGREE(200 << 16),
    .gyro_max_scale = 1,
    .accel_max_val = IIO_M_S_2_TO_G(20000 << 16),
    .accel_max_scale = 5,
    };
    static const struct iio_info adis16460_info = {
    .read_raw = &adis16460_read_raw,
    .write_raw = &adis16460_write_raw,
    .update_scan_mode = adis_update_scan_mode,
    .debugfs_reg_access = adis_debugfs_reg_access,
    };
pub const ADIS16460_DIAG_STAT_IN_CLK_OOS: c_int = 7;
pub const ADIS16460_DIAG_STAT_FLASH_MEM: c_int = 6;
pub const ADIS16460_DIAG_STAT_SELF_TEST: c_int = 5;
pub const ADIS16460_DIAG_STAT_OVERRANGE: c_int = 4;
pub const ADIS16460_DIAG_STAT_SPI_COMM: c_int = 3;
pub const ADIS16460_DIAG_STAT_FLASH_UPT: c_int = 2;
    static const char * const adis16460_status_error_msgs[] = {
    [ADIS16460_DIAG_STAT_IN_CLK_OOS] = "Input clock out of sync",
    [ADIS16460_DIAG_STAT_FLASH_MEM] = "Flash memory failure",
    [ADIS16460_DIAG_STAT_SELF_TEST] = "Self test diagnostic failure",
    [ADIS16460_DIAG_STAT_OVERRANGE] = "Sensor overrange",
    [ADIS16460_DIAG_STAT_SPI_COMM] = "SPI communication failure",
    [ADIS16460_DIAG_STAT_FLASH_UPT] = "Flash update failure",
    };
    static const struct adis_timeout adis16460_timeouts = {
    .reset_ms = 225,
    .sw_reset_ms = 225,
    .self_test_ms = 10,
    };
    static const struct adis_data adis16460_data = {
    .diag_stat_reg = ADIS16460_REG_DIAG_STAT,
    .glob_cmd_reg = ADIS16460_REG_GLOB_CMD,
    .prod_id_reg = ADIS16460_REG_PROD_ID,
    .prod_id = 16460,
    .self_test_mask = BIT(2),
    .self_test_reg = ADIS16460_REG_GLOB_CMD,
    .has_paging = false,
    .read_delay = 5,
    .write_delay = 5,
    .cs_change_delay = 16,
    .status_error_msgs = adis16460_status_error_msgs,
    .status_error_mask = BIT(ADIS16460_DIAG_STAT_IN_CLK_OOS) |
    BIT(ADIS16460_DIAG_STAT_FLASH_MEM) |
    BIT(ADIS16460_DIAG_STAT_SELF_TEST) |
    BIT(ADIS16460_DIAG_STAT_OVERRANGE) |
    BIT(ADIS16460_DIAG_STAT_SPI_COMM) |
    BIT(ADIS16460_DIAG_STAT_FLASH_UPT),
    .unmasked_drdy = true,
    .timeouts = &adis16460_timeouts,
    };
#[no_mangle]
unsafe extern "C" fn adis16460_probe(spi: *mut spi_device) -> c_int {
    static int adis16460_probe(struct spi_device *spi)
    {
    struct iio_dev *indio_dev;
    struct adis16460 *st;
    int ret;
    indio_dev = devm_iio_device_alloc(&spi.dev, sizeof(*st));
    if (indio_dev == core::ptr::null_mut())
    return -ENOMEM;
    st = iio_priv(indio_dev);
    st.chip_info = &adis16460_chip_info;
    indio_dev.name = spi_get_device_id(spi).name;
    indio_dev.channels = st.chip_info.channels;
    indio_dev.num_channels = st.chip_info.num_channels;
    indio_dev.info = &adis16460_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    ret = adis_init(&st.adis, indio_dev, spi, &adis16460_data);
    if (ret)
    return ret;
    ret = devm_adis_setup_buffer_and_trigger(&st.adis, indio_dev, core::ptr::null_mut());
    if (ret)
    return ret;
    ret = __adis_initial_startup(&st.adis);
    if (ret)
    return ret;
    ret = devm_iio_device_register(&spi.dev, indio_dev);
    if (ret)
    return ret;
    adis16460_debugfs_init(indio_dev);
    return 0;
    }
    static const struct spi_device_id adis16460_ids[] = {
    { .name = "adis16460" },
    { }
    };
    MODULE_DEVICE_TABLE(spi, adis16460_ids);
    static const struct of_device_id adis16460_of_match[] = {
    { .compatible = "adi,adis16460" },
    { }
    };
    MODULE_DEVICE_TABLE(of, adis16460_of_match);
    static struct spi_driver adis16460_driver = {
    .driver = {
    .name = "adis16460",
    .of_match_table = adis16460_of_match,
    },
    .id_table = adis16460_ids,
    .probe = adis16460_probe,
    };
    module_spi_driver(adis16460_driver);
    MODULE_AUTHOR("Dragos Bogdan <dragos.bogdan@analog.com>");
    MODULE_DESCRIPTION("Analog Devices ADIS16460 IMU driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_ADISLIB");
