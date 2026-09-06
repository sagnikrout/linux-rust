//! Automatically rewritten from C to Rust
//! Source: drivers/iio/light/max44000.c
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
// MAX44000 Ambient and Infrared Proximity Sensor
//
// Copyright (c) 2016, Intel Corporation.
//
// Data sheet: https://datasheets.maximintegrated.com/en/ds/MAX44000.pdf
//
// 7-bit I2C slave address 0x4a
//

// Registers in datasheet order
pub const MAX44000_REG_STATUS: c_uint = 0x00;
pub const MAX44000_REG_CFG_MAIN: c_uint = 0x01;
pub const MAX44000_REG_CFG_RX: c_uint = 0x02;
pub const MAX44000_REG_CFG_TX: c_uint = 0x03;
pub const MAX44000_REG_ALS_DATA_HI: c_uint = 0x04;
pub const MAX44000_REG_ALS_DATA_LO: c_uint = 0x05;
pub const MAX44000_REG_PRX_DATA: c_uint = 0x16;
pub const MAX44000_REG_ALS_UPTHR_HI: c_uint = 0x06;
pub const MAX44000_REG_ALS_UPTHR_LO: c_uint = 0x07;
pub const MAX44000_REG_ALS_LOTHR_HI: c_uint = 0x08;
pub const MAX44000_REG_ALS_LOTHR_LO: c_uint = 0x09;
pub const MAX44000_REG_PST: c_uint = 0x0a;
pub const MAX44000_REG_PRX_IND: c_uint = 0x0b;
pub const MAX44000_REG_PRX_THR: c_uint = 0x0c;
pub const MAX44000_REG_TRIM_GAIN_GREEN: c_uint = 0x0f;
pub const MAX44000_REG_TRIM_GAIN_IR: c_uint = 0x10;
// REG_CFG bits
pub const MAX44000_CFG_ALSINTE: c_uint = 0x01;
pub const MAX44000_CFG_PRXINTE: c_uint = 0x02;
pub const MAX44000_CFG_MASK: c_uint = 0x1c;
pub const MAX44000_CFG_MODE_SHUTDOWN: c_uint = 0x00;
pub const MAX44000_CFG_MODE_ALS_GIR: c_uint = 0x04;
pub const MAX44000_CFG_MODE_ALS_G: c_uint = 0x08;
pub const MAX44000_CFG_MODE_ALS_IR: c_uint = 0x0c;
pub const MAX44000_CFG_MODE_ALS_PRX: c_uint = 0x10;
pub const MAX44000_CFG_MODE_PRX: c_uint = 0x14;
pub const MAX44000_CFG_TRIM: c_uint = 0x20;
//
// Upper 4 bits are not documented but start as 1 on powerup
// Setting them to 0 causes proximity to misbehave so set them to 1
//
pub const MAX44000_REG_CFG_RX_DEFAULT: c_uint = 0xf0;
// REG_RX bits
pub const MAX44000_CFG_RX_ALSTIM_MASK: c_uint = 0x0c;
pub const MAX44000_CFG_RX_ALSTIM_SHIFT: c_int = 2;
pub const MAX44000_CFG_RX_ALSPGA_MASK: c_uint = 0x03;
pub const MAX44000_CFG_RX_ALSPGA_SHIFT: c_int = 0;
// REG_TX bits
pub const MAX44000_LED_CURRENT_MASK: c_uint = 0xf;
pub const MAX44000_LED_CURRENT_MAX: c_int = 11;
pub const MAX44000_LED_CURRENT_DEFAULT: c_int = 6;
pub const MAX44000_ALSDATA_OVERFLOW: c_uint = 0x4000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max44000_data {
    pub lock: mutex,
    pub regmap: *mut regmap,
}

// Default scale is set to the minimum of 0.03125 or 1 / (1 << 5) lux
pub const MAX44000_ALS_TO_LUX_DEFAULT_FRACTION_LOG2: c_int = 5;
// Scale can be multiplied by up to 128x via ALSPGA for measurement gain
    static const int max44000_alspga_shift[] = {0, 2, 4, 7};
pub const MAX44000_ALSPGA_MAX_SHIFT: c_int = 7;
//
// Scale can be multiplied by up to 64x via ALSTIM because of lost resolution
//
// This scaling factor is hidden from userspace and instead accounted for when
// reading raw values from the device.
//
// This makes it possible to cleanly expose ALSPGA as IIO_CHAN_INFO_SCALE and
// ALSTIM as IIO_CHAN_INFO_INT_TIME without the values affecting each other.
//
// Handling this internally is also required for buffer support because the
// channel's scan_type can't be modified dynamically.
//

// Available integration times with pretty manual alignment:
    static const int max44000_int_time_avail_ns_array[] = {
    100000000,
    25000000,
    6250000,
    1562500,
    };
    static const char max44000_int_time_avail_str[] =
    "0.100 "
    "0.025 "
    "0.00625 "
    "0.0015625";
// Available scales (internal to ulux) with pretty manual alignment:
    static const int max44000_scale_avail_ulux_array[] = {
    31250,
    125000,
    500000,
    4000000,
    };
    static const char max44000_scale_avail_str[] =
    "0.03125 "
    "0.125 "
    "0.5 "
    "4";
pub const MAX44000_SCAN_INDEX_ALS: c_int = 0;
pub const MAX44000_SCAN_INDEX_PRX: c_int = 1;
    static const struct iio_chan_spec max44000_channels[] = {
    {
    .type = IIO_LIGHT,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE) |
    BIT(IIO_CHAN_INFO_INT_TIME),
    .scan_index = MAX44000_SCAN_INDEX_ALS,
    .scan_type = {
    .sign		= 'u',
    .realbits	= 14,
    .storagebits	= 16,
    }
    },
    {
    .type = IIO_PROXIMITY,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),
    .scan_index = MAX44000_SCAN_INDEX_PRX,
    .scan_type = {
    .sign		= 'u',
    .realbits	= 8,
    .storagebits	= 16,
    }
    },
    IIO_CHAN_SOFT_TIMESTAMP(2),
    {
    .type = IIO_CURRENT,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE),
    .extend_name = "led",
    .output = 1,
    .scan_index = -1,
    },
    };
#[no_mangle]
unsafe extern "C" fn max44000_read_alstim(data: *mut max44000_data) -> c_int {
    static int max44000_read_alstim(struct max44000_data *data)
    {
    unsigned int val;
    int ret;
    ret = regmap_read(data.regmap, MAX44000_REG_CFG_RX, &val);
    if (ret < 0)
    return ret;
    return (val & MAX44000_CFG_RX_ALSTIM_MASK) >> MAX44000_CFG_RX_ALSTIM_SHIFT;
    }
#[no_mangle]
unsafe extern "C" fn max44000_write_alstim(data: *mut max44000_data, val: c_int) -> c_int {
    static int max44000_write_alstim(struct max44000_data *data, int val)
    {
    return regmap_write_bits(data.regmap, MAX44000_REG_CFG_RX,
    MAX44000_CFG_RX_ALSTIM_MASK,
    val << MAX44000_CFG_RX_ALSTIM_SHIFT);
    }
#[no_mangle]
unsafe extern "C" fn max44000_read_alspga(data: *mut max44000_data) -> c_int {
    static int max44000_read_alspga(struct max44000_data *data)
    {
    unsigned int val;
    int ret;
    ret = regmap_read(data.regmap, MAX44000_REG_CFG_RX, &val);
    if (ret < 0)
    return ret;
    return (val & MAX44000_CFG_RX_ALSPGA_MASK) >> MAX44000_CFG_RX_ALSPGA_SHIFT;
    }
#[no_mangle]
unsafe extern "C" fn max44000_write_alspga(data: *mut max44000_data, val: c_int) -> c_int {
    static int max44000_write_alspga(struct max44000_data *data, int val)
    {
    return regmap_write_bits(data.regmap, MAX44000_REG_CFG_RX,
    MAX44000_CFG_RX_ALSPGA_MASK,
    val << MAX44000_CFG_RX_ALSPGA_SHIFT);
    }
#[no_mangle]
unsafe extern "C" fn max44000_read_alsval(data: *mut max44000_data) -> c_int {
    static int max44000_read_alsval(struct max44000_data *data)
    {
    u16 regval;
    __be16 val;
    int alstim, ret;
    ret = regmap_bulk_read(data.regmap, MAX44000_REG_ALS_DATA_HI,
    &val, sizeof(val));
    if (ret < 0)
    return ret;
    alstim = ret = max44000_read_alstim(data);
    if (ret < 0)
    return ret;
    regval = be16_to_cpu(val);
//
// Overflow is explained on datasheet page 17.
//
// It's a warning that either the G or IR channel has become saturated
// and that the value in the register is likely incorrect.
//
// The recommendation is to change the scale (ALSPGA).
// The driver just returns the max representable value.
//
    if (regval & MAX44000_ALSDATA_OVERFLOW)
    return 0x3FFF;
    return regval << MAX44000_ALSTIM_SHIFT(alstim);
    }
#[no_mangle]
unsafe extern "C" fn max44000_write_led_current_raw(data: *mut max44000_data, val: c_int) -> c_int {
    static int max44000_write_led_current_raw(struct max44000_data *data, int val)
    {
// Maybe we should clamp the value instead?
    if (val < 0 || val > MAX44000_LED_CURRENT_MAX)
    return -ERANGE;
    if (val >= 8)
    val += 4;
    return regmap_write_bits(data.regmap, MAX44000_REG_CFG_TX,
    MAX44000_LED_CURRENT_MASK, val);
    }
#[no_mangle]
unsafe extern "C" fn max44000_read_led_current_raw(data: *mut max44000_data) -> c_int {
    static int max44000_read_led_current_raw(struct max44000_data *data)
    {
    unsigned int regval;
    int ret;
    ret = regmap_read(data.regmap, MAX44000_REG_CFG_TX, &regval);
    if (ret < 0)
    return ret;
    regval &= MAX44000_LED_CURRENT_MASK;
    if (regval >= 8)
    regval -= 4;
    return regval;
    }
    static int max44000_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct max44000_data *data = iio_priv(indio_dev);
    int alstim, alspga;
    unsigned int regval;
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    switch (chan.type) {
    case IIO_LIGHT:
    mutex_lock(&data.lock);
    ret = max44000_read_alsval(data);
    mutex_unlock(&data.lock);
    if (ret < 0)
    return ret;
// val = ret;
    return IIO_VAL_INT;
    case IIO_PROXIMITY:
    mutex_lock(&data.lock);
    ret = regmap_read(data.regmap, MAX44000_REG_PRX_DATA, &regval);
    mutex_unlock(&data.lock);
    if (ret < 0)
    return ret;
// val = regval;
    return IIO_VAL_INT;
    case IIO_CURRENT:
    mutex_lock(&data.lock);
    ret = max44000_read_led_current_raw(data);
    mutex_unlock(&data.lock);
    if (ret < 0)
    return ret;
// val = ret;
    return IIO_VAL_INT;
    default:
    return -EINVAL;
    }
    case IIO_CHAN_INFO_SCALE:
    switch (chan.type) {
    case IIO_CURRENT:
// Output register is in 10s of miliamps
// val = 10;
    return IIO_VAL_INT;
    case IIO_LIGHT:
    mutex_lock(&data.lock);
    alspga = ret = max44000_read_alspga(data);
    mutex_unlock(&data.lock);
    if (ret < 0)
    return ret;
// Avoid negative shifts
// val = (1 << MAX44000_ALSPGA_MAX_SHIFT);
// val2 = MAX44000_ALS_TO_LUX_DEFAULT_FRACTION_LOG2
    + MAX44000_ALSPGA_MAX_SHIFT
    - max44000_alspga_shift[alspga];
    return IIO_VAL_FRACTIONAL_LOG2;
    default:
    return -EINVAL;
    }
    case IIO_CHAN_INFO_INT_TIME:
    mutex_lock(&data.lock);
    alstim = ret = max44000_read_alstim(data);
    mutex_unlock(&data.lock);
    if (ret < 0)
    return ret;
// val = 0;
// val2 = max44000_int_time_avail_ns_array[alstim];
    return IIO_VAL_INT_PLUS_NANO;
    default:
    return -EINVAL;
    }
    }
    static int max44000_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    struct max44000_data *data = iio_priv(indio_dev);
    int ret;
    if (mask == IIO_CHAN_INFO_RAW && chan.type == IIO_CURRENT) {
    mutex_lock(&data.lock);
    ret = max44000_write_led_current_raw(data, val);
    mutex_unlock(&data.lock);
    return ret;
    } else if (mask == IIO_CHAN_INFO_INT_TIME && chan.type == IIO_LIGHT) {
    let mut valns: i64 = val * NSEC_PER_SEC + val2;
    int alstim = find_closest_descending(valns,
    max44000_int_time_avail_ns_array,
    ARRAY_SIZE(max44000_int_time_avail_ns_array));
    mutex_lock(&data.lock);
    ret = max44000_write_alstim(data, alstim);
    mutex_unlock(&data.lock);
    return ret;
    } else if (mask == IIO_CHAN_INFO_SCALE && chan.type == IIO_LIGHT) {
    let mut valus: i64 = val * USEC_PER_SEC + val2;
    int alspga = find_closest(valus,
    max44000_scale_avail_ulux_array,
    ARRAY_SIZE(max44000_scale_avail_ulux_array));
    mutex_lock(&data.lock);
    ret = max44000_write_alspga(data, alspga);
    mutex_unlock(&data.lock);
    return ret;
    }
    return -EINVAL;
    }
    static int max44000_write_raw_get_fmt(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    long mask)
    {
    if (mask == IIO_CHAN_INFO_INT_TIME && chan.type == IIO_LIGHT)
    return IIO_VAL_INT_PLUS_NANO;
#[no_mangle]
pub unsafe extern "C" fn if(IIO_LIGHT: mask == IIO_CHAN_INFO_SCALE && chan->type ==) -> else {
    else if (mask == IIO_CHAN_INFO_SCALE && chan.type == IIO_LIGHT)
    return IIO_VAL_INT_PLUS_MICRO;
    else
    return IIO_VAL_INT;
    }
    static IIO_CONST_ATTR(illuminance_integration_time_available, max44000_int_time_avail_str);
    static IIO_CONST_ATTR(illuminance_scale_available, max44000_scale_avail_str);
    static struct attribute *max44000_attributes[] = {
    &iio_const_attr_illuminance_integration_time_available.dev_attr.attr,
    &iio_const_attr_illuminance_scale_available.dev_attr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group max44000_attribute_group = {
    .attrs = max44000_attributes,
    };
    static const struct iio_info max44000_info = {
    .read_raw		= max44000_read_raw,
    .write_raw		= max44000_write_raw,
    .write_raw_get_fmt	= max44000_write_raw_get_fmt,
    .attrs			= &max44000_attribute_group,
    };
#[no_mangle]
unsafe extern "C" fn max44000_readable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool max44000_readable_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case MAX44000_REG_STATUS:
    case MAX44000_REG_CFG_MAIN:
    case MAX44000_REG_CFG_RX:
    case MAX44000_REG_CFG_TX:
    case MAX44000_REG_ALS_DATA_HI:
    case MAX44000_REG_ALS_DATA_LO:
    case MAX44000_REG_PRX_DATA:
    case MAX44000_REG_ALS_UPTHR_HI:
    case MAX44000_REG_ALS_UPTHR_LO:
    case MAX44000_REG_ALS_LOTHR_HI:
    case MAX44000_REG_ALS_LOTHR_LO:
    case MAX44000_REG_PST:
    case MAX44000_REG_PRX_IND:
    case MAX44000_REG_PRX_THR:
    case MAX44000_REG_TRIM_GAIN_GREEN:
    case MAX44000_REG_TRIM_GAIN_IR:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn max44000_writeable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool max44000_writeable_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case MAX44000_REG_CFG_MAIN:
    case MAX44000_REG_CFG_RX:
    case MAX44000_REG_CFG_TX:
    case MAX44000_REG_ALS_UPTHR_HI:
    case MAX44000_REG_ALS_UPTHR_LO:
    case MAX44000_REG_ALS_LOTHR_HI:
    case MAX44000_REG_ALS_LOTHR_LO:
    case MAX44000_REG_PST:
    case MAX44000_REG_PRX_IND:
    case MAX44000_REG_PRX_THR:
    case MAX44000_REG_TRIM_GAIN_GREEN:
    case MAX44000_REG_TRIM_GAIN_IR:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn max44000_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool max44000_volatile_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case MAX44000_REG_STATUS:
    case MAX44000_REG_ALS_DATA_HI:
    case MAX44000_REG_ALS_DATA_LO:
    case MAX44000_REG_PRX_DATA:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn max44000_precious_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool max44000_precious_reg(struct device *dev, unsigned int reg)
    {
    let mut reg: return = = MAX44000_REG_STATUS;
    }
    static const struct regmap_config max44000_regmap_config = {
    .reg_bits		= 8,
    .val_bits		= 8,
    .max_register		= MAX44000_REG_PRX_DATA,
    .readable_reg		= max44000_readable_reg,
    .writeable_reg		= max44000_writeable_reg,
    .volatile_reg		= max44000_volatile_reg,
    .precious_reg		= max44000_precious_reg,
    .use_single_read	= true,
    .use_single_write	= true,
    .cache_type		= REGCACHE_RBTREE,
    };
#[no_mangle]
unsafe extern "C" fn max44000_trigger_handler(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t max44000_trigger_handler(int irq, void *p)
    {
    struct iio_poll_func *pf = p;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct max44000_data *data = iio_priv(indio_dev);
    let mut index: c_int = 0;
    unsigned int regval;
    int ret;
    struct {
    u16 channels[2];
    aligned_s64 ts;
    } scan = { };
    mutex_lock(&data.lock);
    if (test_bit(MAX44000_SCAN_INDEX_ALS, indio_dev.active_scan_mask)) {
    ret = max44000_read_alsval(data);
    if (ret < 0)
    goto out_unlock;
    scan.channels[index++] = ret;
    }
    if (test_bit(MAX44000_SCAN_INDEX_PRX, indio_dev.active_scan_mask)) {
    ret = regmap_read(data.regmap, MAX44000_REG_PRX_DATA, &regval);
    if (ret < 0)
    goto out_unlock;
    scan.channels[index] = regval;
    }
    mutex_unlock(&data.lock);
    iio_push_to_buffers_with_ts(indio_dev, &scan, sizeof(scan),
    iio_get_time_ns(indio_dev));
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    out_unlock:
    mutex_unlock(&data.lock);
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn max44000_probe(client: *mut i2c_client) -> c_int {
    static int max44000_probe(struct i2c_client *client)
    {
    struct max44000_data *data;
    struct iio_dev *indio_dev;
    int ret, reg;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    data.regmap = devm_regmap_init_i2c(client, &max44000_regmap_config);
    if (IS_ERR(data.regmap)) {
    dev_err(&client.dev, "regmap_init failed!\n");
    return PTR_ERR(data.regmap);
    }
    mutex_init(&data.lock);
    indio_dev.info = &max44000_info;
    indio_dev.name = MAX44000_DRV_NAME;
    indio_dev.channels = max44000_channels;
    indio_dev.num_channels = ARRAY_SIZE(max44000_channels);
//
// The device doesn't have a reset function so we just clear some
// important bits at probe time to ensure sane operation.
//
// Since we don't support interrupts/events the threshold values are
// not important. We also don't touch trim values.
//
// Reset ALS scaling bits
    ret = regmap_write(data.regmap, MAX44000_REG_CFG_RX,
    MAX44000_REG_CFG_RX_DEFAULT);
    if (ret < 0) {
    dev_err(&client.dev, "failed to write default CFG_RX: %d\n",
    ret);
    return ret;
    }
//
// By default the LED pulse used for the proximity sensor is disabled.
// Set a middle value so that we get some sort of valid data by default.
//
    ret = max44000_write_led_current_raw(data, MAX44000_LED_CURRENT_DEFAULT);
    if (ret < 0) {
    dev_err(&client.dev, "failed to write init config: %d\n", ret);
    return ret;
    }
// Reset CFG bits to ALS_PRX mode which allows easy reading of both values.
    reg = MAX44000_CFG_TRIM | MAX44000_CFG_MODE_ALS_PRX;
    ret = regmap_write(data.regmap, MAX44000_REG_CFG_MAIN, reg);
    if (ret < 0) {
    dev_err(&client.dev, "failed to write init config: %d\n", ret);
    return ret;
    }
// Read status at least once to clear any stale interrupt bits.
    ret = regmap_read(data.regmap, MAX44000_REG_STATUS, &reg);
    if (ret < 0) {
    dev_err(&client.dev, "failed to read init status: %d\n", ret);
    return ret;
    }
    ret = devm_iio_triggered_buffer_setup(&client.dev, indio_dev, core::ptr::null_mut(),
    max44000_trigger_handler, core::ptr::null_mut());
    if (ret < 0) {
    dev_err(&client.dev, "iio triggered buffer setup failed\n");
    return ret;
    }
    return devm_iio_device_register(&client.dev, indio_dev);
    }
    static const struct i2c_device_id max44000_id[] = {
    { .name = "max44000" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, max44000_id);
    static const struct acpi_device_id max44000_acpi_match[] = {
    {"MAX44000", 0},
    { }
    };
    MODULE_DEVICE_TABLE(acpi, max44000_acpi_match);
    static struct i2c_driver max44000_driver = {
    .driver = {
    .name	= MAX44000_DRV_NAME,
    .acpi_match_table = max44000_acpi_match,
    },
    .probe		= max44000_probe,
    .id_table	= max44000_id,
    };
    module_i2c_driver(max44000_driver);
    MODULE_AUTHOR("Crestez Dan Leonard <leonard.crestez@intel.com>");
    MODULE_DESCRIPTION("MAX44000 Ambient and Infrared Proximity Sensor");
    MODULE_LICENSE("GPL v2");
