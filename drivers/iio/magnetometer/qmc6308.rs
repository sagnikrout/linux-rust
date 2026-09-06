//! Automatically rewritten from C to Rust
//! Source: drivers/iio/magnetometer/qmc6308.c
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
// Support for QST QMC6308 3-Axis Magnetic Sensor on I2C bus.
//
// Copyright (C) 2026 Jorijn van der Graaf <jorijnvdgraaf@catcrafts.net>
//
// Datasheet available at
// <https://qstcorp.com/upload/pdf/202202/13-52-15%20QMC6308%20Datasheet%20Rev.%20F(1).pdf>
//

pub const QMC6308_REG_ID: c_uint = 0x00;
pub const QMC6308_REG_X_LSB: c_uint = 0x01;
pub const QMC6308_REG_STATUS: c_uint = 0x09;
pub const QMC6308_REG_CTRL1: c_uint = 0x0A;
pub const QMC6308_REG_CTRL2: c_uint = 0x0B;
pub const QMC6308_REG_CTRL3: c_uint = 0x0D;
pub const QMC6308_REG_CTRL4: c_uint = 0x29;
pub const QMC6308_CHIP_ID: c_uint = 0x80;
// Control register 1

pub const QMC6308_MODE_SUSPEND: c_uint = 0x00;
pub const QMC6308_MODE_NORMAL: c_uint = 0x01;
pub const QMC6308_ODR_10HZ: c_uint = 0x00;
pub const QMC6308_ODR_50HZ: c_uint = 0x01;
pub const QMC6308_ODR_100HZ: c_uint = 0x02;
pub const QMC6308_ODR_200HZ: c_uint = 0x03;
pub const QMC6308_OSR1_8: c_uint = 0x00;
pub const QMC6308_OSR1_4: c_uint = 0x01;
pub const QMC6308_OSR1_2: c_uint = 0x02;
pub const QMC6308_OSR1_1: c_uint = 0x03;
// Control register 2

pub const QMC6308_SET_RESET_ON: c_uint = 0x00;
pub const QMC6308_RNG_30G: c_uint = 0x00;
pub const QMC6308_RNG_12G: c_uint = 0x01;
pub const QMC6308_RNG_8G: c_uint = 0x02;
pub const QMC6308_RNG_2G: c_uint = 0x03;
// Status register

// Power-on completion time (datasheet Table 7)
pub const QMC6308_POR_US: c_int = 250;
pub const QMC6308_AUTOSUSPEND_DELAY_MS: c_int = 500;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmc6308_data {
    pub regmap: *mut regmap,
// Protect data->range/odr/osr and serialize measurements
    pub mutex: mutex,
    pub orientation: iio_mount_matrix,
    pub range: u8,
    pub odr: u8,
    pub osr: u8,
}

    enum qmc6308_axis {
    QMC6308_AXIS_X,
    QMC6308_AXIS_Y,
    QMC6308_AXIS_Z,
    };
    static const int qmc6308_odr_avail[] = {
    [QMC6308_ODR_10HZ] = 10,
    [QMC6308_ODR_50HZ] = 50,
    [QMC6308_ODR_100HZ] = 100,
    [QMC6308_ODR_200HZ] = 200,
    };
    static const int qmc6308_osr1_avail[] = {
    [QMC6308_OSR1_8] = 8,
    [QMC6308_OSR1_4] = 4,
    [QMC6308_OSR1_2] = 2,
    [QMC6308_OSR1_1] = 1,
    };
//
// Sensitivity is 1000/2500/3750/15000 LSB/Gauss for the
// +-30/12/8/2 Gauss ranges respectively.
//
    static const int qmc6308_scales[][2] = {
    [QMC6308_RNG_30G] = { 0, 1000000 },
    [QMC6308_RNG_12G] = { 0, 400000 },
    [QMC6308_RNG_8G] = { 0, 266667 },
    [QMC6308_RNG_2G] = { 0, 66667 },
    };
#[no_mangle]
unsafe extern "C" fn qmc6308_set_mode(data: *mut qmc6308_data, mode: c_uint) -> c_int {
    static int qmc6308_set_mode(struct qmc6308_data *data, unsigned int mode)
    {
    return regmap_update_bits(data.regmap, QMC6308_REG_CTRL1,
    QMC6308_MODE_MASK,
    FIELD_PREP(QMC6308_MODE_MASK, mode));
    }
    static int qmc6308_take_measurement(struct iio_dev *indio_dev, int index,
    int *val)
    {
    struct qmc6308_data *data = iio_priv(indio_dev);
    struct device *dev = regmap_get_device(data.regmap);
    unsigned int status;
    __le16 buf[3];
    int ret;
    PM_RUNTIME_ACQUIRE_AUTOSUSPEND(dev, pm);
    ret = PM_RUNTIME_ACQUIRE_ERR(&pm);
    if (ret) {
    dev_err(dev, "Failed to power on (%d)\n", ret);
    return ret;
    }
    guard(mutex)(&data.mutex);
//
// Reading the status register clears DRDY, which is why the poll
// and the data read stay under one mutex hold. A runtime resume
// clears DRDY too, so a sample converted before the last suspend
// is never returned here.
//
// The timeout is 50ms of headroom over the slowest ODR (10Hz).
//
    ret = regmap_read_poll_timeout(data.regmap, QMC6308_REG_STATUS,
    status, (status & QMC6308_STATUS_DRDY),
    2 * USEC_PER_MSEC,
    150 * USEC_PER_MSEC);
    if (ret)
    return ret;
    ret = regmap_bulk_read(data.regmap, QMC6308_REG_X_LSB, buf,
    sizeof(buf));
    if (ret)
    return ret;
    if (status & QMC6308_STATUS_OVFL)
    return -ERANGE;
// val = (s16)le16_to_cpu(buf[index]);
    return 0;
    }
    static int qmc6308_read_raw(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    int *val, int *val2, long mask)
    {
    struct qmc6308_data *data = iio_priv(indio_dev);
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    ret = qmc6308_take_measurement(indio_dev, chan.address, val);
    if (ret)
    return ret;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE: {
    guard(mutex)(&data.mutex);
// val = qmc6308_scales[data->range][0];
// val2 = qmc6308_scales[data->range][1];
    return IIO_VAL_INT_PLUS_NANO;
    }
    case IIO_CHAN_INFO_SAMP_FREQ: {
    guard(mutex)(&data.mutex);
// val = qmc6308_odr_avail[data->odr];
    return IIO_VAL_INT;
    }
    case IIO_CHAN_INFO_OVERSAMPLING_RATIO: {
    guard(mutex)(&data.mutex);
// val = qmc6308_osr1_avail[data->osr];
    return IIO_VAL_INT;
    }
    default:
    return -EINVAL;
    }
    }
    static int qmc6308_write_raw(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    int val, int val2, long mask)
    {
    struct qmc6308_data *data = iio_priv(indio_dev);
    unsigned int status;
    unsigned int i;
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_SCALE: {
    if (val != 0)
    return -EINVAL;
    for (i = 0; i < ARRAY_SIZE(qmc6308_scales); i++) {
    if (val2 == qmc6308_scales[i][1])
    break;
    }
    if (i == ARRAY_SIZE(qmc6308_scales))
    return -EINVAL;
    guard(mutex)(&data.mutex);
    ret = regmap_update_bits(data.regmap, QMC6308_REG_CTRL2,
    QMC6308_RNG_MASK,
    FIELD_PREP(QMC6308_RNG_MASK, i));
    if (ret)
    return ret;
    data.range = i;
//
// The data registers still hold (and DRDY still
// advertises) a sample converted at the previous range;
// discard it so that a read does not pair old-range data
// with the new scale. A conversion already in flight may
// still complete at the old range, so this narrows the
// window rather than closing it. The range change itself
// took effect, so only log a failure here: an error
// would mislead userspace about an effective write.
//
    ret = regmap_read(data.regmap, QMC6308_REG_STATUS,
    &status);
    if (ret)
    dev_warn(regmap_get_device(data.regmap),
    "Failed to discard stale sample (%d)\n", ret);
    return 0;
    }
    case IIO_CHAN_INFO_SAMP_FREQ: {
    for (i = 0; i < ARRAY_SIZE(qmc6308_odr_avail); i++) {
    if (val == qmc6308_odr_avail[i])
    break;
    }
    if (i == ARRAY_SIZE(qmc6308_odr_avail))
    return -EINVAL;
    guard(mutex)(&data.mutex);
    ret = regmap_update_bits(data.regmap, QMC6308_REG_CTRL1,
    QMC6308_ODR_MASK,
    FIELD_PREP(QMC6308_ODR_MASK, i));
    if (ret)
    return ret;
    data.odr = i;
    return 0;
    }
    case IIO_CHAN_INFO_OVERSAMPLING_RATIO: {
    for (i = 0; i < ARRAY_SIZE(qmc6308_osr1_avail); i++) {
    if (val == qmc6308_osr1_avail[i])
    break;
    }
    if (i == ARRAY_SIZE(qmc6308_osr1_avail))
    return -EINVAL;
    guard(mutex)(&data.mutex);
    ret = regmap_update_bits(data.regmap, QMC6308_REG_CTRL1,
    QMC6308_OSR1_MASK,
    FIELD_PREP(QMC6308_OSR1_MASK, i));
    if (ret)
    return ret;
    data.osr = i;
    return 0;
    }
    default:
    return -EINVAL;
    }
    }
    static int qmc6308_read_avail(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    const int **vals, int *type, int *length,
    long mask)
    {
    switch (mask) {
    case IIO_CHAN_INFO_SAMP_FREQ:
// vals = qmc6308_odr_avail;
// type = IIO_VAL_INT;
// length = ARRAY_SIZE(qmc6308_odr_avail);
    return IIO_AVAIL_LIST;
    case IIO_CHAN_INFO_OVERSAMPLING_RATIO:
// vals = qmc6308_osr1_avail;
// type = IIO_VAL_INT;
// length = ARRAY_SIZE(qmc6308_osr1_avail);
    return IIO_AVAIL_LIST;
    case IIO_CHAN_INFO_SCALE:
// vals = (const int *)qmc6308_scales;
// type = IIO_VAL_INT_PLUS_NANO;
// length = ARRAY_SIZE(qmc6308_scales) * 2;
    return IIO_AVAIL_LIST;
    default:
    return -EINVAL;
    }
    }
    static int qmc6308_write_raw_get_fmt(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    long mask)
    {
    switch (mask) {
    case IIO_CHAN_INFO_SCALE:
    return IIO_VAL_INT_PLUS_NANO;
    default:
    return IIO_VAL_INT;
    }
    }
    static const struct iio_mount_matrix *
    qmc6308_get_mount_matrix(const struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan)
    {
    struct qmc6308_data *data = iio_priv(indio_dev);
    return &data.orientation;
    }
    static const struct iio_chan_spec_ext_info qmc6308_ext_info[] = {
    IIO_MOUNT_MATRIX(IIO_SHARED_BY_DIR, qmc6308_get_mount_matrix),
    { }
    };
    static const struct iio_info qmc6308_info = {
    .read_raw = qmc6308_read_raw,
    .write_raw = qmc6308_write_raw,
    .read_avail = qmc6308_read_avail,
    .write_raw_get_fmt = qmc6308_write_raw_get_fmt,
    };
#[no_mangle]
unsafe extern "C" fn qmc6308_init(data: *mut qmc6308_data) -> c_int {
    static int qmc6308_init(struct qmc6308_data *data)
    {
    struct regmap *map = data.regmap;
    unsigned int reg;
    int ret;
    ret = regmap_read(map, QMC6308_REG_ID, &reg);
    if (ret)
    return ret;
// Allow unknown IDs so that fallback compatibles work
    if (reg != QMC6308_CHIP_ID)
    dev_warn(regmap_get_device(map),
    "Unknown chip id: 0x%02x, continuing\n", reg);
// The SOFT_RST bit is not auto-cleared and must be written back 0
    ret = regmap_write(map, QMC6308_REG_CTRL2, QMC6308_SOFT_RST);
    if (ret)
    return ret;
//
// The datasheet gives no soft-reset completion figure; reuse the
// power-on time as a conservative bound.
//
    fsleep(QMC6308_POR_US);
    data.range = QMC6308_RNG_30G;
    data.odr = QMC6308_ODR_50HZ;
    data.osr = QMC6308_OSR1_8;
    ret = regmap_write(map, QMC6308_REG_CTRL2,
    FIELD_PREP(QMC6308_SET_RESET_MASK,
    QMC6308_SET_RESET_ON) |
    FIELD_PREP(QMC6308_RNG_MASK, data.range));
    if (ret)
    return ret;
// OSR2 (second-stage filter) set to its power-on default of 0
    return regmap_write(map, QMC6308_REG_CTRL1,
    FIELD_PREP(QMC6308_MODE_MASK,
    QMC6308_MODE_NORMAL) |
    FIELD_PREP(QMC6308_ODR_MASK, data.odr) |
    FIELD_PREP(QMC6308_OSR1_MASK, data.osr) |
    FIELD_PREP(QMC6308_OSR2_MASK, 0));
    }
#[no_mangle]
unsafe extern "C" fn qmc6308_power_down_action(priv: *mut c_void) {
    static void qmc6308_power_down_action(void *priv)
    {
    struct qmc6308_data *data = priv;
    if (!pm_runtime_status_suspended(regmap_get_device(data.regmap)))
    qmc6308_set_mode(data, QMC6308_MODE_SUSPEND);
    }
#[no_mangle]
unsafe extern "C" fn qmc6308_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool qmc6308_volatile_reg(struct device *dev, unsigned int reg)
    {
    return reg >= QMC6308_REG_X_LSB && reg <= QMC6308_REG_STATUS;
    }
#[no_mangle]
unsafe extern "C" fn qmc6308_writable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool qmc6308_writable_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case QMC6308_REG_CTRL1:
    case QMC6308_REG_CTRL2:
    case QMC6308_REG_CTRL3:
    case QMC6308_REG_CTRL4:
    return true;
    default:
    return false;
    }
    }
    static const struct regmap_config qmc6308_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = QMC6308_REG_CTRL4,
    .cache_type = REGCACHE_MAPLE,
    .volatile_reg = qmc6308_volatile_reg,
    .writeable_reg = qmc6308_writable_reg,
    };

    {                                                      \
    .type = IIO_MAGN,                              \
    .modified = 1,                                 \
    .channel2 = IIO_MOD_##_axis,                   \
    .address = QMC6308_AXIS_##_axis,               \
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),  \
    .info_mask_shared_by_type =                    \
    BIT(IIO_CHAN_INFO_SCALE) |             \
    BIT(IIO_CHAN_INFO_SAMP_FREQ) |         \
    BIT(IIO_CHAN_INFO_OVERSAMPLING_RATIO), \
    .info_mask_shared_by_type_available =          \
    BIT(IIO_CHAN_INFO_SCALE) |             \
    BIT(IIO_CHAN_INFO_SAMP_FREQ) |         \
    BIT(IIO_CHAN_INFO_OVERSAMPLING_RATIO), \
    .ext_info = qmc6308_ext_info,                  \
    }
    static const struct iio_chan_spec qmc6308_channels[] = {
    QMC6308_CHANNEL(X),
    QMC6308_CHANNEL(Y),
    QMC6308_CHANNEL(Z),
    };
#[no_mangle]
unsafe extern "C" fn qmc6308_probe(client: *mut i2c_client) -> c_int {
    static int qmc6308_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct qmc6308_data *data;
    struct iio_dev *indio_dev;
    struct regmap *map;
    int ret;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    i2c_set_clientdata(client, indio_dev);
    map = devm_regmap_init_i2c(client, &qmc6308_regmap_config);
    if (IS_ERR(map))
    return dev_err_probe(dev, PTR_ERR(map),
    "regmap initialization failed\n");
    ret = devm_regulator_get_enable(dev, "vdd");
    if (ret)
    return dev_err_probe(dev, ret,
    "Failed to enable VDD regulator\n");
    fsleep(QMC6308_POR_US);
    data = iio_priv(indio_dev);
    data.regmap = map;
    ret = devm_mutex_init(dev, &data.mutex);
    if (ret)
    return ret;
    ret = iio_read_mount_matrix(dev, &data.orientation);
    if (ret)
    return dev_err_probe(dev, ret,
    "Failed to read mount matrix\n");
    indio_dev.name = "qmc6308";
    indio_dev.info = &qmc6308_info;
    indio_dev.channels = qmc6308_channels;
    indio_dev.num_channels = ARRAY_SIZE(qmc6308_channels);
    indio_dev.modes = INDIO_DIRECT_MODE;
    ret = qmc6308_init(data);
    if (ret)
    return dev_err_probe(dev, ret, "qmc6308 init failed\n");
    ret = pm_runtime_set_active(dev);
    if (ret)
    return ret;
    ret = devm_add_action_or_reset(dev, qmc6308_power_down_action, data);
    if (ret)
    return ret;
    pm_runtime_use_autosuspend(dev);
    pm_runtime_set_autosuspend_delay(dev, QMC6308_AUTOSUSPEND_DELAY_MS);
    ret = devm_pm_runtime_enable(dev);
    if (ret)
    return ret;
    return devm_iio_device_register(dev, indio_dev);
    }
#[no_mangle]
unsafe extern "C" fn qmc6308_runtime_suspend(dev: *mut device) -> c_int {
    static int qmc6308_runtime_suspend(struct device *dev)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(dev);
    struct qmc6308_data *data = iio_priv(indio_dev);
    return qmc6308_set_mode(data, QMC6308_MODE_SUSPEND);
    }
#[no_mangle]
unsafe extern "C" fn qmc6308_runtime_resume(dev: *mut device) -> c_int {
    static int qmc6308_runtime_resume(struct device *dev)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(dev);
    struct qmc6308_data *data = iio_priv(indio_dev);
    unsigned int status;
    int ret;
    ret = qmc6308_set_mode(data, QMC6308_MODE_NORMAL);
    if (ret)
    return ret;
//
// DRDY may still be set for a sample converted before the last
// suspend; reading the status register clears it so the next
// measurement waits for fresh data.
//
    ret = regmap_read(data.regmap, QMC6308_REG_STATUS, &status);
    if (ret) {
// Best effort to leave the chip in a consistent state
    qmc6308_set_mode(data, QMC6308_MODE_SUSPEND);
    }
    return ret;
    }
    static DEFINE_RUNTIME_DEV_PM_OPS(qmc6308_pm_ops, qmc6308_runtime_suspend,
    qmc6308_runtime_resume, core::ptr::null_mut());
    static const struct of_device_id qmc6308_match[] = {
    { .compatible = "qstcorp,qmc6308" },
    { }
    };
    MODULE_DEVICE_TABLE(of, qmc6308_match);
    static const struct i2c_device_id qmc6308_id[] = {
    { .name = "qmc6308" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, qmc6308_id);
    static struct i2c_driver qmc6308_driver = {
    .driver = {
    .name = "qmc6308",
    .of_match_table = qmc6308_match,
    .pm = pm_ptr(&qmc6308_pm_ops),
    },
    .id_table = qmc6308_id,
    .probe = qmc6308_probe,
    };
    module_i2c_driver(qmc6308_driver);
    MODULE_DESCRIPTION("QST QMC6308 3-Axis Magnetic Sensor driver");
    MODULE_AUTHOR("Jorijn van der Graaf <jorijnvdgraaf@catcrafts.net>");
    MODULE_LICENSE("GPL");
