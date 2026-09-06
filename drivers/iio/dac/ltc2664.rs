//! Automatically rewritten from C to Rust
//! Source: drivers/iio/dac/ltc2664.c
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
// LTC2664 4 channel, 12-/16-Bit Voltage Output SoftSpan DAC driver
// LTC2672 5 channel, 12-/16-Bit Current Output Softspan DAC driver
//
// Copyright 2024 Analog Devices Inc.
//

pub const LTC2664_CMD_WRITE_N_UPDATE_ALL: c_uint = 0x20;

pub const LTC2664_CMD_POWER_DOWN_ALL: c_uint = 0x50;

pub const LTC2664_CMD_CONFIG: c_uint = 0x70;
pub const LTC2664_CMD_MUX: c_uint = 0xB0;
pub const LTC2664_CMD_TOGGLE_SEL: c_uint = 0xC0;
pub const LTC2664_CMD_GLOBAL_TOGGLE: c_uint = 0xD0;
pub const LTC2664_CMD_NO_OPERATION: c_uint = 0xF0;
pub const LTC2664_REF_DISABLE: c_uint = 0x0001;
pub const LTC2664_MSPAN_SOFTSPAN: c_int = 7;
pub const LTC2672_MAX_CHANNEL: c_int = 5;
pub const LTC2672_MAX_SPAN: c_int = 7;

    enum {
    LTC2664_SPAN_RANGE_0V_5V,
    LTC2664_SPAN_RANGE_0V_10V,
    LTC2664_SPAN_RANGE_M5V_5V,
    LTC2664_SPAN_RANGE_M10V_10V,
    LTC2664_SPAN_RANGE_M2V5_2V5,
    };
    enum {
    LTC2664_INPUT_A,
    LTC2664_INPUT_B,
    LTC2664_INPUT_B_AVAIL,
    LTC2664_POWERDOWN,
    LTC2664_POWERDOWN_MODE,
    LTC2664_TOGGLE_EN,
    LTC2664_GLOBAL_TOGGLE,
    };
    static const u16 ltc2664_mspan_lut[8][2] = {
    { LTC2664_SPAN_RANGE_M10V_10V, 32768 }, /* MPS2=0, MPS1=0, MSP0=0 (0)*/
    { LTC2664_SPAN_RANGE_M5V_5V, 32768 }, /* MPS2=0, MPS1=0, MSP0=1 (1)*/
    { LTC2664_SPAN_RANGE_M2V5_2V5, 32768 }, /* MPS2=0, MPS1=1, MSP0=0 (2)*/
    { LTC2664_SPAN_RANGE_0V_10V, 0 }, /* MPS2=0, MPS1=1, MSP0=1 (3)*/
    { LTC2664_SPAN_RANGE_0V_10V, 32768 }, /* MPS2=1, MPS1=0, MSP0=0 (4)*/
    { LTC2664_SPAN_RANGE_0V_5V, 0 }, /* MPS2=1, MPS1=0, MSP0=1 (5)*/
    { LTC2664_SPAN_RANGE_0V_5V, 32768 }, /* MPS2=1, MPS1=1, MSP0=0 (6)*/
    { LTC2664_SPAN_RANGE_0V_5V, 0 } /* MPS2=1, MPS1=1, MSP0=1 (7)*/
    };
    struct ltc2664_state;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltc2664_chip_info {
    pub name: *const c_char,
    pub c): *const *const *const int (scale_get)(struct ltc2664_state st, int,
    pub c): *const *const *const int (offset_get)(struct ltc2664_state st, int,
    pub measurement_type: c_int,
    pub num_channels: c_uint,
    pub (*span_helper)[2]: *const c_int,
    pub num_span: c_uint,
    pub internal_vref_mv: c_uint,
    pub manual_span_support: bool,
    pub rfsadj_support: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltc2664_chan {
// indicates if the channel should be toggled
    pub toggle_chan: bool,
// indicates if the channel is in powered down state
    pub powerdown: bool,
// span code of the channel
    pub span: u8,
// raw data of the current state of the chip registers (A/B)
    pub raw: [u16; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltc2664_state {
    pub spi: *mut spi_device,
    pub regmap: *mut regmap,
    pub channels: [ltc2664_chan; LTC2672_MAX_CHANNEL],
// lock to protect against multiple access to the device and shared data
    pub lock: mutex,
    pub chip_info: *const ltc2664_chip_info,
    pub iio_channels: *mut iio_chan_spec,
    pub vref_mv: c_int,
    pub rfsadj_ohms: u32,
    pub toggle_sel: u32,
    pub global_toggle: bool,
}

    static const int ltc2664_span_helper[][2] = {
    { 0, 5000 },
    { 0, 10000 },
    { -5000, 5000 },
    { -10000, 10000 },
    { -2500, 2500 },
    };
    static const int ltc2672_span_helper[][2] = {
    { 0, 0 },
    { 0, 3125 },
    { 0, 6250 },
    { 0, 12500 },
    { 0, 25000 },
    { 0, 50000 },
    { 0, 100000 },
    { 0, 200000 },
    { 0, 300000 },
    };
#[no_mangle]
unsafe extern "C" fn ltc2664_scale_get(st: *const ltc2664_state, c: c_int) -> c_int {
    static int ltc2664_scale_get(const struct ltc2664_state *st, int c)
    {
    const struct ltc2664_chan *chan = &st.channels[c];
    const int (*span_helper)[2] = st.chip_info.span_helper;
    int span, fs;
    span = chan.span;
    if (span < 0)
    return span;
    fs = span_helper[span][1] - span_helper[span][0];
    return fs * st.vref_mv / 2500;
    }
#[no_mangle]
unsafe extern "C" fn ltc2672_scale_get(st: *const ltc2664_state, c: c_int) -> c_int {
    static int ltc2672_scale_get(const struct ltc2664_state *st, int c)
    {
    const struct ltc2664_chan *chan = &st.channels[c];
    int span, fs;
    span = chan.span - 1;
    if (span < 0)
    return span;
    fs = 1000 * st.vref_mv;
    if (span == LTC2672_MAX_SPAN)
    return mul_u64_u32_div(4800, fs, st.rfsadj_ohms);
    return mul_u64_u32_div(LTC2672_SCALE_MULTIPLIER(span), fs, st.rfsadj_ohms);
    }
#[no_mangle]
unsafe extern "C" fn ltc2664_offset_get(st: *const ltc2664_state, c: c_int) -> c_int {
    static int ltc2664_offset_get(const struct ltc2664_state *st, int c)
    {
    const struct ltc2664_chan *chan = &st.channels[c];
    int span;
    span = chan.span;
    if (span < 0)
    return span;
    if (st.chip_info.span_helper[span][0] < 0)
    return -32768;
    return 0;
    }
    static int ltc2664_dac_code_write(struct ltc2664_state *st, u32 chan, u32 input,
    u16 code)
    {
    struct ltc2664_chan *c = &st.channels[chan];
    int ret, reg;
    guard(mutex)(&st.lock);
// select the correct input register to write to
    if (c.toggle_chan) {
    ret = regmap_write(st.regmap, LTC2664_CMD_TOGGLE_SEL,
    input << chan);
    if (ret)
    return ret;
    }
//
// If in toggle mode the dac should be updated by an
// external signal (or sw toggle) and not here.
//
    if (st.toggle_sel & BIT(chan))
    reg = LTC2664_CMD_WRITE_N(chan);
    else
    reg = LTC2664_CMD_WRITE_N_UPDATE_N(chan);
    ret = regmap_write(st.regmap, reg, code);
    if (ret)
    return ret;
    c.raw[input] = code;
    if (c.toggle_chan) {
    ret = regmap_write(st.regmap, LTC2664_CMD_TOGGLE_SEL,
    st.toggle_sel);
    if (ret)
    return ret;
    }
    return 0;
    }
    static void ltc2664_dac_code_read(struct ltc2664_state *st, u32 chan, u32 input,
    u32 *code)
    {
    guard(mutex)(&st.lock);
// code = st->channels[chan].raw[input];
    }
    static const int ltc2664_raw_range[] = { 0, 1, U16_MAX };
    static int ltc2664_read_avail(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    const int **vals, int *type, int *length,
    long info)
    {
    switch (info) {
    case IIO_CHAN_INFO_RAW:
// vals = ltc2664_raw_range;
// type = IIO_VAL_INT;
    return IIO_AVAIL_RANGE;
    default:
    return -EINVAL;
    }
    }
    static int ltc2664_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int *val,
    int *val2, long info)
    {
    struct ltc2664_state *st = iio_priv(indio_dev);
    switch (info) {
    case IIO_CHAN_INFO_RAW:
    ltc2664_dac_code_read(st, chan.channel, LTC2664_INPUT_A, val);
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_OFFSET:
// val = st->chip_info->offset_get(st, chan->channel);
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
// val = st->chip_info->scale_get(st, chan->channel);
// val2 = 16;
    return IIO_VAL_FRACTIONAL_LOG2;
    default:
    return -EINVAL;
    }
    }
    static int ltc2664_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int val,
    int val2, long info)
    {
    struct ltc2664_state *st = iio_priv(indio_dev);
    switch (info) {
    case IIO_CHAN_INFO_RAW:
    if (val > U16_MAX || val < 0)
    return -EINVAL;
    return ltc2664_dac_code_write(st, chan.channel,
    LTC2664_INPUT_A, val);
    default:
    return -EINVAL;
    }
    }
    static ssize_t ltc2664_reg_bool_get(struct iio_dev *indio_dev,
    uintptr_t private,
    const struct iio_chan_spec *chan,
    char *buf)
    {
    struct ltc2664_state *st = iio_priv(indio_dev);
    u32 val;
    guard(mutex)(&st.lock);
    switch (private) {
    case LTC2664_POWERDOWN:
    val = st.channels[chan.channel].powerdown;
    return sysfs_emit(buf, "%u\n", val);
    case LTC2664_POWERDOWN_MODE:
    return sysfs_emit(buf, "42kohm_to_gnd\n");
    case LTC2664_TOGGLE_EN:
    val = !!(st.toggle_sel & BIT(chan.channel));
    return sysfs_emit(buf, "%u\n", val);
    case LTC2664_GLOBAL_TOGGLE:
    val = st.global_toggle;
    return sysfs_emit(buf, "%u\n", val);
    default:
    return -EINVAL;
    }
    }
    static ssize_t ltc2664_reg_bool_set(struct iio_dev *indio_dev,
    uintptr_t private,
    const struct iio_chan_spec *chan,
    const char *buf, size_t len)
    {
    struct ltc2664_state *st = iio_priv(indio_dev);
    int ret;
    bool en;
    ret = kstrtobool(buf, &en);
    if (ret)
    return ret;
    guard(mutex)(&st.lock);
    switch (private) {
    case LTC2664_POWERDOWN:
    ret = regmap_write(st.regmap,
    en ? LTC2664_CMD_POWER_DOWN_N(chan.channel) :
    LTC2664_CMD_UPDATE_N(chan.channel), en);
    if (ret)
    return ret;
    st.channels[chan.channel].powerdown = en;
    return len;
    case LTC2664_TOGGLE_EN:
    if (en)
    st.toggle_sel |= BIT(chan.channel);
    else
    st.toggle_sel &= ~BIT(chan.channel);
    ret = regmap_write(st.regmap, LTC2664_CMD_TOGGLE_SEL,
    st.toggle_sel);
    if (ret)
    return ret;
    return len;
    case LTC2664_GLOBAL_TOGGLE:
    ret = regmap_write(st.regmap, LTC2664_CMD_GLOBAL_TOGGLE, en);
    if (ret)
    return ret;
    st.global_toggle = en;
    return len;
    default:
    return -EINVAL;
    }
    }
    static ssize_t ltc2664_dac_input_read(struct iio_dev *indio_dev,
    uintptr_t private,
    const struct iio_chan_spec *chan,
    char *buf)
    {
    struct ltc2664_state *st = iio_priv(indio_dev);
    u32 val;
    if (private == LTC2664_INPUT_B_AVAIL)
    return sysfs_emit(buf, "[%u %u %u]\n", ltc2664_raw_range[0],
    ltc2664_raw_range[1],
    ltc2664_raw_range[2] / 4);
    ltc2664_dac_code_read(st, chan.channel, private, &val);
    return sysfs_emit(buf, "%u\n", val);
    }
    static ssize_t ltc2664_dac_input_write(struct iio_dev *indio_dev,
    uintptr_t private,
    const struct iio_chan_spec *chan,
    const char *buf, size_t len)
    {
    struct ltc2664_state *st = iio_priv(indio_dev);
    int ret;
    u16 val;
    if (private == LTC2664_INPUT_B_AVAIL)
    return -EINVAL;
    ret = kstrtou16(buf, 10, &val);
    if (ret)
    return ret;
    ret = ltc2664_dac_code_write(st, chan.channel, private, val);
    if (ret)
    return ret;
    return len;
    }
    static int ltc2664_reg_access(struct iio_dev *indio_dev,
    unsigned int reg,
    unsigned int writeval,
    unsigned int *readval)
    {
    struct ltc2664_state *st = iio_priv(indio_dev);
    if (readval)
    return -EOPNOTSUPP;
    return regmap_write(st.regmap, reg, writeval);
    }

    .name = _name,							\
    .read = (_read),						\
    .write = (_write),						\
    .private = (_what),						\
    .shared = (_shared),						\
    }
//
// For toggle mode we only expose the symbol attr (sw_toggle) in case a TGPx is
// not provided in dts.
//
    static const struct iio_chan_spec_ext_info ltc2664_toggle_sym_ext_info[] = {
    LTC2664_CHAN_EXT_INFO("raw0", LTC2664_INPUT_A, IIO_SEPARATE,
    ltc2664_dac_input_read, ltc2664_dac_input_write),
    LTC2664_CHAN_EXT_INFO("raw1", LTC2664_INPUT_B, IIO_SEPARATE,
    ltc2664_dac_input_read, ltc2664_dac_input_write),
    LTC2664_CHAN_EXT_INFO("powerdown", LTC2664_POWERDOWN, IIO_SEPARATE,
    ltc2664_reg_bool_get, ltc2664_reg_bool_set),
    LTC2664_CHAN_EXT_INFO("powerdown_mode", LTC2664_POWERDOWN_MODE,
    IIO_SEPARATE, ltc2664_reg_bool_get, core::ptr::null_mut()),
    LTC2664_CHAN_EXT_INFO("symbol", LTC2664_GLOBAL_TOGGLE, IIO_SEPARATE,
    ltc2664_reg_bool_get, ltc2664_reg_bool_set),
    LTC2664_CHAN_EXT_INFO("toggle_en", LTC2664_TOGGLE_EN,
    IIO_SEPARATE, ltc2664_reg_bool_get,
    ltc2664_reg_bool_set),
    { }
    };
    static const struct iio_chan_spec_ext_info ltc2664_ext_info[] = {
    LTC2664_CHAN_EXT_INFO("powerdown", LTC2664_POWERDOWN, IIO_SEPARATE,
    ltc2664_reg_bool_get, ltc2664_reg_bool_set),
    LTC2664_CHAN_EXT_INFO("powerdown_mode", LTC2664_POWERDOWN_MODE,
    IIO_SEPARATE, ltc2664_reg_bool_get, core::ptr::null_mut()),
    { }
    };
    static const struct iio_chan_spec ltc2664_channel_template = {
    .indexed = 1,
    .output = 1,
    .info_mask_separate = BIT(IIO_CHAN_INFO_SCALE) |
    BIT(IIO_CHAN_INFO_OFFSET) |
    BIT(IIO_CHAN_INFO_RAW),
    .info_mask_separate_available = BIT(IIO_CHAN_INFO_RAW),
    .ext_info = ltc2664_ext_info,
    };
    static const struct ltc2664_chip_info ltc2664_chip = {
    .name = "ltc2664",
    .scale_get = ltc2664_scale_get,
    .offset_get = ltc2664_offset_get,
    .measurement_type = IIO_VOLTAGE,
    .num_channels = 4,
    .span_helper = ltc2664_span_helper,
    .num_span = ARRAY_SIZE(ltc2664_span_helper),
    .internal_vref_mv = 2500,
    .manual_span_support = true,
    .rfsadj_support = false,
    };
    static const struct ltc2664_chip_info ltc2672_chip = {
    .name = "ltc2672",
    .scale_get = ltc2672_scale_get,
    .offset_get = ltc2664_offset_get,
    .measurement_type = IIO_CURRENT,
    .num_channels = 5,
    .span_helper = ltc2672_span_helper,
    .num_span = ARRAY_SIZE(ltc2672_span_helper),
    .internal_vref_mv = 1250,
    .manual_span_support = false,
    .rfsadj_support = true,
    };
    static int ltc2664_set_span(const struct ltc2664_state *st, int min, int max,
    int chan)
    {
    const struct ltc2664_chip_info *chip_info = st.chip_info;
    const int (*span_helper)[2] = chip_info.span_helper;
    int span, ret;
    for (span = 0; span < chip_info.num_span; span++) {
    if (min == span_helper[span][0] && max == span_helper[span][1])
    break;
    }
    if (span == chip_info.num_span)
    return -EINVAL;
    ret = regmap_write(st.regmap, LTC2664_CMD_SPAN_N(chan), span);
    if (ret)
    return ret;
    return span;
    }
#[no_mangle]
unsafe extern "C" fn ltc2664_channel_config(st: *mut ltc2664_state) -> c_int {
    static int ltc2664_channel_config(struct ltc2664_state *st)
    {
    const struct ltc2664_chip_info *chip_info = st.chip_info;
    struct device *dev = &st.spi.dev;
    u32 reg, tmp[2], mspan;
    int ret;
    mspan = LTC2664_MSPAN_SOFTSPAN;
    ret = device_property_read_u32(dev, "adi,manual-span-operation-config",
    &mspan);
    if (!ret) {
    if (!chip_info.manual_span_support)
    return dev_err_probe(dev, -EINVAL,
    "adi,manual-span-operation-config not supported\n");
    if (mspan >= ARRAY_SIZE(ltc2664_mspan_lut))
    return dev_err_probe(dev, -EINVAL,
    "adi,manual-span-operation-config not in range\n");
    }
    st.rfsadj_ohms = 20000;
    ret = device_property_read_u32(dev, "adi,rfsadj-ohms", &st.rfsadj_ohms);
    if (!ret) {
    if (!chip_info.rfsadj_support)
    return dev_err_probe(dev, -EINVAL,
    "adi,rfsadj-ohms not supported\n");
    if (st.rfsadj_ohms < 19000 || st.rfsadj_ohms > 41000)
    return dev_err_probe(dev, -EINVAL,
    "adi,rfsadj-ohms not in range\n");
    }
    device_for_each_child_node_scoped(dev, child) {
    struct ltc2664_chan *chan;
    ret = fwnode_property_read_u32(child, "reg", &reg);
    if (ret)
    return dev_err_probe(dev, ret,
    "Failed to get reg property\n");
    if (reg >= chip_info.num_channels)
    return dev_err_probe(dev, -EINVAL,
    "reg bigger than: %d\n",
    chip_info.num_channels);
    chan = &st.channels[reg];
    if (fwnode_property_read_bool(child, "adi,toggle-mode")) {
    chan.toggle_chan = true;
// assume sw toggle ABI
    st.iio_channels[reg].ext_info = ltc2664_toggle_sym_ext_info;
//
// Clear IIO_CHAN_INFO_RAW bit as toggle channels expose
// out_voltage/current_raw{0|1} files.
//
    __clear_bit(IIO_CHAN_INFO_RAW,
    &st.iio_channels[reg].info_mask_separate);
    }
    chan.raw[0] = ltc2664_mspan_lut[mspan][1];
    chan.raw[1] = ltc2664_mspan_lut[mspan][1];
    chan.span = ltc2664_mspan_lut[mspan][0];
    ret = fwnode_property_read_u32_array(child, "output-range-microvolt",
    tmp, ARRAY_SIZE(tmp));
    if (!ret && mspan == LTC2664_MSPAN_SOFTSPAN) {
    ret = ltc2664_set_span(st, tmp[0] / 1000, tmp[1] / 1000, reg);
    if (ret < 0)
    return dev_err_probe(dev, ret,
    "Failed to set span\n");
    chan.span = ret;
    }
    ret = fwnode_property_read_u32_array(child, "output-range-microamp",
    tmp, ARRAY_SIZE(tmp));
    if (!ret) {
    ret = ltc2664_set_span(st, 0, tmp[1] / 1000, reg);
    if (ret < 0)
    return dev_err_probe(dev, ret,
    "Failed to set span\n");
    chan.span = ret;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ltc2664_setup(st: *mut ltc2664_state) -> c_int {
    static int ltc2664_setup(struct ltc2664_state *st)
    {
    const struct ltc2664_chip_info *chip_info = st.chip_info;
    struct gpio_desc *gpio;
    int ret, i;
// If we have a clr/reset pin, use that to reset the chip.
    gpio = devm_gpiod_get_optional(&st.spi.dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(gpio))
    return dev_err_probe(&st.spi.dev, PTR_ERR(gpio),
    "Failed to get reset gpio");
    if (gpio) {
    fsleep(1000);
    gpiod_set_value_cansleep(gpio, 0);
    }
//
// Duplicate the default channel configuration as it can change during
// @ltc2664_channel_config()
//
    st.iio_channels = devm_kcalloc(&st.spi.dev,
    chip_info.num_channels,
    sizeof(struct iio_chan_spec),
    GFP_KERNEL);
    if (!st.iio_channels)
    return -ENOMEM;
    for (i = 0; i < chip_info.num_channels; i++) {
    st.iio_channels[i] = ltc2664_channel_template;
    st.iio_channels[i].type = chip_info.measurement_type;
    st.iio_channels[i].channel = i;
    }
    ret = ltc2664_channel_config(st);
    if (ret)
    return ret;
    return regmap_set_bits(st.regmap, LTC2664_CMD_CONFIG, LTC2664_REF_DISABLE);
    }
    static const struct regmap_config ltc2664_regmap_config = {
    .reg_bits = 8,
    .val_bits = 16,
    .max_register = LTC2664_CMD_NO_OPERATION,
    };
    static const struct iio_info ltc2664_info = {
    .write_raw = ltc2664_write_raw,
    .read_raw = ltc2664_read_raw,
    .read_avail = ltc2664_read_avail,
    .debugfs_reg_access = ltc2664_reg_access,
    };
#[no_mangle]
unsafe extern "C" fn ltc2664_probe(spi: *mut spi_device) -> c_int {
    static int ltc2664_probe(struct spi_device *spi)
    {
    static const char * const regulators[] = { "vcc", "iovcc", "v-neg" };
    const struct ltc2664_chip_info *chip_info;
    struct device *dev = &spi.dev;
    struct iio_dev *indio_dev;
    struct ltc2664_state *st;
    int ret;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*st));
    if (!indio_dev)
    return -ENOMEM;
    st = iio_priv(indio_dev);
    st.spi = spi;
    chip_info = spi_get_device_match_data(spi);
    if (!chip_info)
    return -ENODEV;
    st.chip_info = chip_info;
    ret = devm_mutex_init(dev, &st.lock);
    if (ret)
    return ret;
    st.regmap = devm_regmap_init_spi(spi, &ltc2664_regmap_config);
    if (IS_ERR(st.regmap))
    return dev_err_probe(dev, PTR_ERR(st.regmap),
    "Failed to init regmap");
    ret = devm_regulator_bulk_get_enable(dev, ARRAY_SIZE(regulators),
    regulators);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to enable regulators\n");
    ret = devm_regulator_get_enable_read_voltage(dev, "ref");
    if (ret < 0 && ret != -ENODEV)
    return ret;
    st.vref_mv = ret > 0 ? ret / 1000 :  chip_info.internal_vref_mv;
    ret = ltc2664_setup(st);
    if (ret)
    return ret;
    indio_dev.name = chip_info.name;
    indio_dev.info = &ltc2664_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = st.iio_channels;
    indio_dev.num_channels = chip_info.num_channels;
    return devm_iio_device_register(dev, indio_dev);
    }
    static const struct spi_device_id ltc2664_id[] = {
    { .name = "ltc2664", .driver_data = (kernel_ulong_t)&ltc2664_chip },
    { .name = "ltc2672", .driver_data = (kernel_ulong_t)&ltc2672_chip },
    { }
    };
    MODULE_DEVICE_TABLE(spi, ltc2664_id);
    static const struct of_device_id ltc2664_of_id[] = {
    { .compatible = "adi,ltc2664", .data = &ltc2664_chip },
    { .compatible = "adi,ltc2672", .data = &ltc2672_chip },
    { }
    };
    MODULE_DEVICE_TABLE(of, ltc2664_of_id);
    static struct spi_driver ltc2664_driver = {
    .driver = {
    .name = "ltc2664",
    .of_match_table = ltc2664_of_id,
    },
    .probe = ltc2664_probe,
    .id_table = ltc2664_id,
    };
    module_spi_driver(ltc2664_driver);
    MODULE_AUTHOR("Michael Hennerich <michael.hennerich@analog.com>");
    MODULE_AUTHOR("Kim Seer Paller <kimseer.paller@analog.com>");
    MODULE_DESCRIPTION("Analog Devices LTC2664 and LTC2672 DAC");
    MODULE_LICENSE("GPL");
