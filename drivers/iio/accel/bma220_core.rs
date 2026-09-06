//! Automatically rewritten from C to Rust
//! Source: drivers/iio/accel/bma220_core.c
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
// BMA220 Digital triaxial acceleration sensor driver
//
// Copyright (c) 2016,2020 Intel Corporation.
// Copyright (c) 2025 Petre Rodan  <petre.rodan@subdimension.ro>
//

pub const BMA220_REG_ID: c_uint = 0x00;
pub const BMA220_REG_REVISION_ID: c_uint = 0x01;
pub const BMA220_REG_ACCEL_X: c_uint = 0x02;
pub const BMA220_REG_ACCEL_Y: c_uint = 0x03;
pub const BMA220_REG_ACCEL_Z: c_uint = 0x04;
pub const BMA220_REG_CONF0: c_uint = 0x05;

pub const BMA220_REG_CONF1: c_uint = 0x06;

pub const BMA220_REG_CONF2: c_uint = 0x07;

pub const BMA220_REG_CONF3: c_uint = 0x08;

pub const BMA220_REG_CONF4: c_uint = 0x09;

pub const BMA220_REG_CONF5: c_uint = 0x0a;

pub const BMA220_REG_IF0: c_uint = 0x0b;
pub const BMA220_REG_IF1: c_uint = 0x0c;

pub const BMA220_REG_IE0: c_uint = 0x0d;

pub const BMA220_REG_IE1: c_uint = 0x0e;

pub const BMA220_REG_IE2: c_uint = 0x0f;
pub const BMA220_REG_FILTER: c_uint = 0x10;

pub const BMA220_REG_RANGE: c_uint = 0x11;

pub const BMA220_REG_SUSPEND: c_uint = 0x18;
pub const BMA220_REG_SOFTRESET: c_uint = 0x19;
pub const BMA220_CHIP_ID: c_uint = 0xDD;
pub const BMA220_SUSPEND_SLEEP: c_uint = 0xFF;
pub const BMA220_SUSPEND_WAKE: c_uint = 0x00;
pub const BMA220_RESET_MODE: c_uint = 0xFF;
pub const BMA220_NONRESET_MODE: c_uint = 0x00;

pub const BMA220_COF_1000Hz: c_uint = 0x0;
pub const BMA220_COF_500Hz: c_uint = 0x1;
pub const BMA220_COF_250Hz: c_uint = 0x2;
pub const BMA220_COF_125Hz: c_uint = 0x3;
pub const BMA220_COF_64Hz: c_uint = 0x4;
pub const BMA220_COF_32Hz: c_uint = 0x5;

    .type = IIO_ACCEL,						\
    .address = reg,							\
    .modified = 1,							\
    .channel2 = IIO_MOD_##axis,					\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),			\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE) |		\
    BIT(IIO_CHAN_INFO_LOW_PASS_FILTER_3DB_FREQUENCY),		\
    .info_mask_shared_by_type_available = BIT(IIO_CHAN_INFO_SCALE) |\
    BIT(IIO_CHAN_INFO_LOW_PASS_FILTER_3DB_FREQUENCY),		\
    .scan_index = index,						\
    .scan_type = {							\
    .sign = 's',						\
    .realbits = 6,						\
    .storagebits = 8,					\
    .shift = 2,						\
    .endianness = IIO_CPU,					\
    },								\
    }
    enum bma220_axis {
    AXIS_X,
    AXIS_Y,
    AXIS_Z,
    };
    static const int bma220_scale_table[][2] = {
    { 0, 623000 }, { 1, 248000 }, { 2, 491000 }, { 4, 983000 },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bma220_data {
    pub regmap: *mut regmap,
    pub lock: mutex,
    pub lpf_3dB_freq_idx: u8,
    pub range_idx: u8,
    pub trig: *mut iio_trigger,
    struct {
    pub chans: [i8; 3],
// Ensure timestamp is naturally aligned.
    pub timestamp: aligned_s64,
    pub __aligned(IIO_DMA_MINALIGN): } scan,
}

    static const struct iio_chan_spec bma220_channels[] = {
    BMA220_ACCEL_CHANNEL(0, BMA220_REG_ACCEL_X, X),
    BMA220_ACCEL_CHANNEL(1, BMA220_REG_ACCEL_Y, Y),
    BMA220_ACCEL_CHANNEL(2, BMA220_REG_ACCEL_Z, Z),
    IIO_CHAN_SOFT_TIMESTAMP(3),
    };
// Available cut-off frequencies of the low pass filter in Hz.
    static const int bma220_lpf_3dB_freq_Hz_table[] = {
    [BMA220_COF_1000Hz] = 1000,
    [BMA220_COF_500Hz] = 500,
    [BMA220_COF_250Hz] = 250,
    [BMA220_COF_125Hz] = 125,
    [BMA220_COF_64Hz] = 64,
    [BMA220_COF_32Hz] = 32,
    };
    static const unsigned long bma220_accel_scan_masks[] = {
    BIT(AXIS_X) | BIT(AXIS_Y) | BIT(AXIS_Z),
    0
    };
#[no_mangle]
unsafe extern "C" fn bma220_is_writable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool bma220_is_writable_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case BMA220_REG_CONF0:
    case BMA220_REG_CONF1:
    case BMA220_REG_CONF2:
    case BMA220_REG_CONF3:
    case BMA220_REG_CONF4:
    case BMA220_REG_CONF5:
    case BMA220_REG_IE0:
    case BMA220_REG_IE1:
    case BMA220_REG_IE2:
    case BMA220_REG_FILTER:
    case BMA220_REG_RANGE:
    case BMA220_REG_WDT:
    return true;
    default:
    return false;
    }
    }
    const struct regmap_config bma220_spi_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .read_flag_mask = BIT(7),
    .max_register = BMA220_REG_SOFTRESET,
    .cache_type = REGCACHE_NONE,
    .writeable_reg = bma220_is_writable_reg,
    };
    EXPORT_SYMBOL_NS_GPL(bma220_spi_regmap_config, "IIO_BOSCH_BMA220");
//
// Based on the datasheet the memory map differs between the SPI and the I2C
// implementations. I2C register addresses are simply shifted to the left
// by 1 bit yet the register size remains unchanged.
// This driver employs the SPI memory map to correlate register names to
// addresses regardless of the bus type.
//
    const struct regmap_config bma220_i2c_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .reg_shift = -1,
    .max_register = BMA220_REG_SOFTRESET,
    .cache_type = REGCACHE_NONE,
    .writeable_reg = bma220_is_writable_reg,
    };
    EXPORT_SYMBOL_NS_GPL(bma220_i2c_regmap_config, "IIO_BOSCH_BMA220");
    static int bma220_data_rdy_trigger_set_state(struct iio_trigger *trig,
    bool state)
    {
    struct iio_dev *indio_dev = iio_trigger_get_drvdata(trig);
    struct bma220_data *data = iio_priv(indio_dev);
    return regmap_update_bits(data.regmap, BMA220_REG_IE0,
    BMA220_INT_EN_DRDY_MSK,
    FIELD_PREP(BMA220_INT_EN_DRDY_MSK, state));
    }
    static const struct iio_trigger_ops bma220_trigger_ops = {
    .set_trigger_state = &bma220_data_rdy_trigger_set_state,
    .validate_device = &iio_trigger_validate_own_device,
    };
#[no_mangle]
unsafe extern "C" fn bma220_trigger_handler(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t bma220_trigger_handler(int irq, void *p)
    {
    int ret;
    struct iio_poll_func *pf = p;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct bma220_data *data = iio_priv(indio_dev);
    ret = regmap_bulk_read(data.regmap, BMA220_REG_ACCEL_X,
    &data.scan.chans,
    sizeof(data.scan.chans));
    if (ret < 0)
    return IRQ_NONE;
    iio_push_to_buffers_with_ts(indio_dev, &data.scan, sizeof(data.scan),
    iio_get_time_ns(indio_dev));
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    }
    static int bma220_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    int ret;
    u8 index;
    unsigned int reg;
    struct bma220_data *data = iio_priv(indio_dev);
    guard(mutex)(&data.lock);
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    ret = regmap_read(data.regmap, chan.address, &reg);
    if (ret < 0)
    return -EINVAL;
// val = sign_extend32(reg >> chan->scan_type.shift,
    chan.scan_type.realbits - 1);
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
    index = data.range_idx;
// val = bma220_scale_table[index][0];
// val2 = bma220_scale_table[index][1];
    return IIO_VAL_INT_PLUS_MICRO;
    case IIO_CHAN_INFO_LOW_PASS_FILTER_3DB_FREQUENCY:
    index = data.lpf_3dB_freq_idx;
// val = bma220_lpf_3dB_freq_Hz_table[index];
    return IIO_VAL_INT;
    }
    return -EINVAL;
    }
    static int bma220_find_match_2dt(const int (*tbl)[2], const int n,
    const int val, const int val2)
    {
    int i;
    for (i = 0; i < n; i++) {
    if (tbl[i][0] == val && tbl[i][1] == val2)
    return i;
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn bma220_find_match(arr: *const c_int, n: c_int, val: c_int) -> c_int {
    static int bma220_find_match(const int *arr, const int n, const int val)
    {
    int i;
    for (i = 0; i < n; i++) {
    if (arr[i] == val)
    return i;
    }
    return -EINVAL;
    }
    static int bma220_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    int ret;
    let mut index: c_int = -1;
    struct bma220_data *data = iio_priv(indio_dev);
    guard(mutex)(&data.lock);
    switch (mask) {
    case IIO_CHAN_INFO_SCALE:
    index = bma220_find_match_2dt(bma220_scale_table,
    ARRAY_SIZE(bma220_scale_table),
    val, val2);
    if (index < 0)
    return -EINVAL;
    ret = regmap_update_bits(data.regmap, BMA220_REG_RANGE,
    BMA220_RANGE_MASK,
    FIELD_PREP(BMA220_RANGE_MASK, index));
    if (ret < 0)
    return ret;
    data.range_idx = index;
    return 0;
    case IIO_CHAN_INFO_LOW_PASS_FILTER_3DB_FREQUENCY:
    index = bma220_find_match(bma220_lpf_3dB_freq_Hz_table,
    ARRAY_SIZE(bma220_lpf_3dB_freq_Hz_table),
    val);
    if (index < 0)
    return -EINVAL;
    ret = regmap_update_bits(data.regmap, BMA220_REG_FILTER,
    BMA220_FILTER_MASK,
    FIELD_PREP(BMA220_FILTER_MASK, index));
    if (ret < 0)
    return ret;
    data.lpf_3dB_freq_idx = index;
    return 0;
    }
    return -EINVAL;
    }
    static int bma220_read_avail(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    const int **vals, int *type, int *length,
    long mask)
    {
    switch (mask) {
    case IIO_CHAN_INFO_SCALE:
// vals = (int *)bma220_scale_table;
// type = IIO_VAL_INT_PLUS_MICRO;
// length = ARRAY_SIZE(bma220_scale_table) * 2;
    return IIO_AVAIL_LIST;
    case IIO_CHAN_INFO_LOW_PASS_FILTER_3DB_FREQUENCY:
// vals = (const int *)bma220_lpf_3dB_freq_Hz_table;
// type = IIO_VAL_INT;
// length = ARRAY_SIZE(bma220_lpf_3dB_freq_Hz_table);
    return IIO_AVAIL_LIST;
    default:
    return -EINVAL;
    }
    }
    static int bma220_reg_access(struct iio_dev *indio_dev, unsigned int reg,
    unsigned int writeval, unsigned int *readval)
    {
    struct bma220_data *data = iio_priv(indio_dev);
    if (readval)
    return regmap_read(data.regmap, reg, readval);
    return regmap_write(data.regmap, reg, writeval);
    }
    static const struct iio_info bma220_info = {
    .read_raw		= bma220_read_raw,
    .write_raw		= bma220_write_raw,
    .read_avail		= bma220_read_avail,
    .debugfs_reg_access	= &bma220_reg_access,
    };
#[no_mangle]
unsafe extern "C" fn bma220_reset(data: *mut bma220_data, up: bool) -> c_int {
    static int bma220_reset(struct bma220_data *data, bool up)
    {
    int ret;
    unsigned int i, val;
//
// The chip can be reset by a simple register read.
// We need up to 2 register reads of the softreset register
// to make sure that the device is in the desired state.
//
    for (i = 0; i < 2; i++) {
    ret = regmap_read(data.regmap, BMA220_REG_SOFTRESET, &val);
    if (ret < 0)
    return ret;
    if (up && val == BMA220_RESET_MODE)
    return 0;
    if (!up && val == BMA220_NONRESET_MODE)
    return 0;
    }
    return -EBUSY;
    }
#[no_mangle]
unsafe extern "C" fn bma220_power(data: *mut bma220_data, up: bool) -> c_int {
    static int bma220_power(struct bma220_data *data, bool up)
    {
    int ret;
    unsigned int i, val;
//
// The chip can be suspended/woken up by a simple register read.
// So, we need up to 2 register reads of the suspend register
// to make sure that the device is in the desired state.
//
    for (i = 0; i < 2; i++) {
    ret = regmap_read(data.regmap, BMA220_REG_SUSPEND, &val);
    if (ret < 0)
    return ret;
    if (up && val == BMA220_SUSPEND_SLEEP)
    return 0;
    if (!up && val == BMA220_SUSPEND_WAKE)
    return 0;
    }
    return -EBUSY;
    }
#[no_mangle]
unsafe extern "C" fn bma220_init(dev: *mut device, data: *mut bma220_data) -> c_int {
    static int bma220_init(struct device *dev, struct bma220_data *data)
    {
    int ret;
    unsigned int val;
    static const char * const regulator_names[] = { "vddd", "vddio", "vdda" };
    ret = devm_regulator_bulk_get_enable(dev,
    ARRAY_SIZE(regulator_names),
    regulator_names);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to get regulators\n");
    ret = regmap_read(data.regmap, BMA220_REG_ID, &val);
    if (ret)
    return dev_err_probe(dev, ret,
    "Failed to read chip id register\n");
    if (val != BMA220_CHIP_ID)
    dev_info(dev, "Unknown chip found: 0x%02x\n", val);
    ret = bma220_power(data, true);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to power-on chip\n");
    ret = bma220_reset(data, true);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to soft reset chip\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bma220_deinit(data_ptr: *mut c_void) {
    static void bma220_deinit(void *data_ptr)
    {
    struct bma220_data *data = data_ptr;
    int ret;
    struct device *dev = regmap_get_device(data.regmap);
    ret = bma220_power(data, false);
    if (ret)
    dev_warn(dev,
    "Failed to put device into suspend mode (%pe)\n",
    ERR_PTR(ret));
    }
#[no_mangle]
unsafe extern "C" fn bma220_irq_handler(irq: c_int, private: *mut c_void) -> irqreturn_t {
    static irqreturn_t bma220_irq_handler(int irq, void *private)
    {
    struct iio_dev *indio_dev = private;
    struct bma220_data *data = iio_priv(indio_dev);
    int ret;
    unsigned int bma220_reg_if1;
    ret = regmap_read(data.regmap, BMA220_REG_IF1, &bma220_reg_if1);
    if (ret)
    return IRQ_NONE;
    if (FIELD_GET(BMA220_IF_DRDY, bma220_reg_if1))
    iio_trigger_poll_nested(data.trig);
    return IRQ_HANDLED;
    }
#[no_mangle]
pub unsafe extern "C" fn bma220_common_probe(dev: *mut device, regmap: *mut regmap, irq: c_int) -> c_int {
    int bma220_common_probe(struct device *dev, struct regmap *regmap, int irq)
    {
    int ret;
    struct iio_dev *indio_dev;
    struct bma220_data *data;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    data.regmap = regmap;
    ret = bma220_init(dev, data);
    if (ret)
    return ret;
    ret = devm_mutex_init(dev, &data.lock);
    if (ret)
    return ret;
    indio_dev.info = &bma220_info;
    indio_dev.name = BMA220_DEVICE_NAME;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = bma220_channels;
    indio_dev.num_channels = ARRAY_SIZE(bma220_channels);
    indio_dev.available_scan_masks = bma220_accel_scan_masks;
    if (irq > 0) {
    data.trig = devm_iio_trigger_alloc(dev, "%s-dev%d",
    indio_dev.name,
    iio_device_id(indio_dev));
    if (!data.trig)
    return -ENOMEM;
    data.trig.ops = &bma220_trigger_ops;
    iio_trigger_set_drvdata(data.trig, indio_dev);
    ret = devm_iio_trigger_register(dev, data.trig);
    if (ret)
    return dev_err_probe(dev, ret,
    "iio trigger register fail\n");
    indio_dev.trig = iio_trigger_get(data.trig);
    ret = devm_request_threaded_irq(dev, irq, core::ptr::null_mut(),
    &bma220_irq_handler, IRQF_ONESHOT,
    indio_dev.name, indio_dev);
    if (ret)
    return ret;
    }
    ret = devm_add_action_or_reset(dev, bma220_deinit, data);
    if (ret)
    return ret;
    ret = devm_iio_triggered_buffer_setup(dev, indio_dev, core::ptr::null_mut(),
    bma220_trigger_handler, core::ptr::null_mut());
    if (ret < 0)
    dev_err_probe(dev, ret, "iio triggered buffer setup failed\n");
    return devm_iio_device_register(dev, indio_dev);
    }
    EXPORT_SYMBOL_NS_GPL(bma220_common_probe, "IIO_BOSCH_BMA220");
#[no_mangle]
unsafe extern "C" fn bma220_suspend(dev: *mut device) -> c_int {
    static int bma220_suspend(struct device *dev)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(dev);
    struct bma220_data *data = iio_priv(indio_dev);
    return bma220_power(data, false);
    }
#[no_mangle]
unsafe extern "C" fn bma220_resume(dev: *mut device) -> c_int {
    static int bma220_resume(struct device *dev)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(dev);
    struct bma220_data *data = iio_priv(indio_dev);
    return bma220_power(data, true);
    }
    EXPORT_NS_SIMPLE_DEV_PM_OPS(bma220_pm_ops, bma220_suspend, bma220_resume,
    IIO_BOSCH_BMA220);
    MODULE_AUTHOR("Tiberiu Breana <tiberiu.a.breana@intel.com>");
    MODULE_DESCRIPTION("BMA220 acceleration sensor driver");
    MODULE_LICENSE("GPL");
