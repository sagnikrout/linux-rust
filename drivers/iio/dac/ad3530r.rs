//! Automatically rewritten from C to Rust
//! Source: drivers/iio/dac/ad3530r.c
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
// AD3530R/AD3530 8-channel, 16-bit Voltage Output DAC Driver
// AD3531R/AD3531 4-channel, 16-bit Voltage Output DAC Driver
// AD3532R/AD3532 16-channel, 16-bit Voltage Output DAC Driver
//
// Copyright 2025 Analog Devices Inc.
//

pub const AD3530R_INTERFACE_CONFIG_A: c_uint = 0x00;
pub const AD3530R_OUTPUT_OPERATING_MODE_0: c_uint = 0x20;
pub const AD3530R_OUTPUT_OPERATING_MODE_1: c_uint = 0x21;
pub const AD3530R_OUTPUT_CONTROL_0: c_uint = 0x2A;
pub const AD3530R_REFERENCE_CONTROL_0: c_uint = 0x3C;
pub const AD3530R_SW_LDAC_TRIG_A: c_uint = 0xE5;
pub const AD3530R_INPUT_CH: c_uint = 0xEB;
pub const AD3530R_MAX_REG_ADDR: c_uint = 0xF9;
pub const AD3531R_SW_LDAC_TRIG_A: c_uint = 0xDD;
pub const AD3531R_INPUT_CH: c_uint = 0xE3;
// AD3532R/AD3532 bank 0 registers (channels 0-7)
pub const AD3532R_INTERFACE_CONFIG_A_0: c_uint = 0x1000;
pub const AD3532R_OUTPUT_OPERATING_MODE_0: c_uint = 0x1020;
pub const AD3532R_OUTPUT_OPERATING_MODE_1: c_uint = 0x1021;
pub const AD3532R_OUTPUT_CONTROL_0: c_uint = 0x102A;
pub const AD3532R_REFERENCE_CONTROL_0: c_uint = 0x103C;
pub const AD3532R_SW_LDAC_TRIG_0: c_uint = 0x10E5;
pub const AD3532R_INPUT_CH_0: c_uint = 0x10EB;
// AD3532R/AD3532 bank 1 registers (channels 8-15)
pub const AD3532R_INTERFACE_CONFIG_A_1: c_uint = 0x3000;
pub const AD3532R_OUTPUT_OPERATING_MODE_2: c_uint = 0x3020;
pub const AD3532R_OUTPUT_OPERATING_MODE_3: c_uint = 0x3021;
pub const AD3532R_OUTPUT_CONTROL_1: c_uint = 0x302A;
pub const AD3532R_REFERENCE_CONTROL_1: c_uint = 0x303C;
pub const AD3532R_SW_LDAC_TRIG_1: c_uint = 0x30E5;
pub const AD3532R_INPUT_CH_1: c_uint = 0x30EB;
pub const AD3532R_MAX_REG_ADDR: c_uint = 0x30F9;

pub const AD3530R_INTERNAL_VREF_mV: c_int = 2500;
pub const AD3530R_LDAC_PULSE_US: c_int = 100;

pub const AD3530R_CH_PER_REG: c_int = 4;
pub const AD3530R_CH_PER_BANK: c_int = 8;
pub const AD3531R_MAX_CHANNELS: c_int = 4;
pub const AD3532R_MAX_CHANNELS: c_int = 16;
    enum ad3530r_mode {
    AD3530R_NORMAL_OP,
    AD3530R_POWERDOWN_1K,
    AD3530R_POWERDOWN_7K7,
    AD3530R_POWERDOWN_32K,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad3530r_chan {
    pub powerdown_mode: enum ad3530r_mode,
    pub powerdown: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad3530r_chip_info {
    pub name: *const c_char,
    pub channels: *const iio_chan_spec,
    pub regmap_config: *const regmap_config,
    pub channel): *mut *mut int (input_ch_reg)(unsigned int,
    pub channel): *mut *mut int (sw_ldac_trig_reg)(unsigned int,
    pub interface_config_a: *const c_uint,
    pub output_control: *const c_uint,
    pub reference_control: *const c_uint,
    pub op_mode: *const c_uint,
    pub num_channels: c_uint,
    pub num_banks: c_uint,
    pub num_op_mode_regs: c_uint,
    pub internal_ref_support: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad3530r_state {
    pub regmap: *mut regmap,
// lock to protect against multiple access to the device and shared data
    pub lock: mutex,
    pub chan: [ad3530r_chan; AD3532R_MAX_CHANNELS],
    pub chip_info: *const ad3530r_chip_info,
    pub ldac_gpio: *mut gpio_desc,
    pub vref_mV: c_int,
//
// DMA (thus cache coherency maintenance) may require the transfer
// buffers to live in their own cache lines.
//
    pub __aligned(IIO_DMA_MINALIGN): __be16 buf,
}

#[no_mangle]
unsafe extern "C" fn ad3530r_input_ch_reg(channel: c_uint) -> c_int {
    static int ad3530r_input_ch_reg(unsigned int channel)
    {
    return 2 * channel + AD3530R_INPUT_CH;
    }
#[no_mangle]
unsafe extern "C" fn ad3531r_input_ch_reg(channel: c_uint) -> c_int {
    static int ad3531r_input_ch_reg(unsigned int channel)
    {
    return 2 * channel + AD3531R_INPUT_CH;
    }
#[no_mangle]
unsafe extern "C" fn ad3532r_input_ch_reg(channel: c_uint) -> c_int {
    static int ad3532r_input_ch_reg(unsigned int channel)
    {
    let mut bank: c_uint = channel / AD3530R_CH_PER_BANK;
    let mut local_ch: c_uint = channel % AD3530R_CH_PER_BANK;
    return 2 * local_ch + (bank ? AD3532R_INPUT_CH_1 : AD3532R_INPUT_CH_0);
    }
    static const char * const ad3530r_powerdown_modes[] = {
    "1kohm_to_gnd",
    "7.7kohm_to_gnd",
    "32kohm_to_gnd",
    };
    static const char * const ad3531r_powerdown_modes[] = {
    "500ohm_to_gnd",
    "3.85kohm_to_gnd",
    "16kohm_to_gnd",
    };
    static const char * const ad3532r_powerdown_modes[] = {
    "1kohm_to_gnd",
    "10kohm_to_gnd",
    "three_state",
    };
    static int ad3530r_get_powerdown_mode(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan)
    {
    struct ad3530r_state *st = iio_priv(indio_dev);
    guard(mutex)(&st.lock);
    return st.chan[chan.channel].powerdown_mode - 1;
    }
    static int ad3530r_set_powerdown_mode(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    unsigned int mode)
    {
    struct ad3530r_state *st = iio_priv(indio_dev);
    guard(mutex)(&st.lock);
    st.chan[chan.channel].powerdown_mode = mode + 1;
    return 0;
    }
    static const struct iio_enum ad3530r_powerdown_mode_enum = {
    .items = ad3530r_powerdown_modes,
    .num_items = ARRAY_SIZE(ad3530r_powerdown_modes),
    .get = ad3530r_get_powerdown_mode,
    .set = ad3530r_set_powerdown_mode,
    };
    static const struct iio_enum ad3531r_powerdown_mode_enum = {
    .items = ad3531r_powerdown_modes,
    .num_items = ARRAY_SIZE(ad3531r_powerdown_modes),
    .get = ad3530r_get_powerdown_mode,
    .set = ad3530r_set_powerdown_mode,
    };
    static const struct iio_enum ad3532r_powerdown_mode_enum = {
    .items = ad3532r_powerdown_modes,
    .num_items = ARRAY_SIZE(ad3532r_powerdown_modes),
    .get = ad3530r_get_powerdown_mode,
    .set = ad3530r_set_powerdown_mode,
    };
    static ssize_t ad3530r_get_dac_powerdown(struct iio_dev *indio_dev,
    uintptr_t private,
    const struct iio_chan_spec *chan,
    char *buf)
    {
    struct ad3530r_state *st = iio_priv(indio_dev);
    guard(mutex)(&st.lock);
    return sysfs_emit(buf, "%d\n", st.chan[chan.channel].powerdown);
    }
    static ssize_t ad3530r_set_dac_powerdown(struct iio_dev *indio_dev,
    uintptr_t private,
    const struct iio_chan_spec *chan,
    const char *buf, size_t len)
    {
    struct ad3530r_state *st = iio_priv(indio_dev);
    int ret;
    unsigned int reg, pdmode, mask, val;
    bool powerdown;
    ret = kstrtobool(buf, &powerdown);
    if (ret)
    return ret;
    guard(mutex)(&st.lock);
    reg = chan.channel < AD3531R_MAX_CHANNELS ?
    AD3530R_OUTPUT_OPERATING_MODE_0 :
    AD3530R_OUTPUT_OPERATING_MODE_1;
    pdmode = powerdown ? st.chan[chan.channel].powerdown_mode : 0;
    mask = chan.channel < AD3531R_MAX_CHANNELS ?
    AD3530R_OP_MODE_CHAN_MSK(chan.channel) :
    AD3530R_OP_MODE_CHAN_MSK(chan.channel - 4);
    val = field_prep(mask, pdmode);
    ret = regmap_update_bits(st.regmap, reg, mask, val);
    if (ret)
    return ret;
    st.chan[chan.channel].powerdown = powerdown;
    return len;
    }
    static ssize_t ad3532r_set_dac_powerdown(struct iio_dev *indio_dev,
    uintptr_t private,
    const struct iio_chan_spec *chan,
    const char *buf, size_t len)
    {
    struct ad3530r_state *st = iio_priv(indio_dev);
    unsigned int bank, local_ch, reg_in_bank, ch_in_reg;
    unsigned int reg, mask, val;
    bool powerdown;
    int ret;
    ret = kstrtobool(buf, &powerdown);
    if (ret)
    return ret;
    bank = chan.channel / AD3530R_CH_PER_BANK;
    local_ch = chan.channel % AD3530R_CH_PER_BANK;
    reg_in_bank = local_ch / AD3530R_CH_PER_REG;
    ch_in_reg = local_ch % AD3530R_CH_PER_REG;
    reg = reg_in_bank + (bank ? AD3532R_OUTPUT_OPERATING_MODE_2 :
    AD3532R_OUTPUT_OPERATING_MODE_0);
    mask = AD3530R_OP_MODE_CHAN_MSK(ch_in_reg);
    guard(mutex)(&st.lock);
    if (powerdown) {
    val = field_prep(mask, st.chan[chan.channel].powerdown_mode);
    ret = regmap_update_bits(st.regmap, reg, mask, val);
    } else {
    ret = regmap_clear_bits(st.regmap, reg, mask);
    }
    if (ret)
    return ret;
    st.chan[chan.channel].powerdown = powerdown;
    return len;
    }
#[no_mangle]
unsafe extern "C" fn ad3530r_trigger_sw_ldac_reg(channel: c_uint) -> c_int {
    static int ad3530r_trigger_sw_ldac_reg(unsigned int channel)
    {
    return AD3530R_SW_LDAC_TRIG_A;
    }
#[no_mangle]
unsafe extern "C" fn ad3531r_trigger_sw_ldac_reg(channel: c_uint) -> c_int {
    static int ad3531r_trigger_sw_ldac_reg(unsigned int channel)
    {
    return AD3531R_SW_LDAC_TRIG_A;
    }
#[no_mangle]
unsafe extern "C" fn ad3532r_trigger_sw_ldac_reg(channel: c_uint) -> c_int {
    static int ad3532r_trigger_sw_ldac_reg(unsigned int channel)
    {
    let mut bank: c_uint = channel / AD3530R_CH_PER_BANK;
    return bank ? AD3532R_SW_LDAC_TRIG_1 : AD3532R_SW_LDAC_TRIG_0;
    }
#[no_mangle]
unsafe extern "C" fn ad3530r_trigger_hw_ldac(ldac_gpio: *mut gpio_desc) -> c_int {
    static int ad3530r_trigger_hw_ldac(struct gpio_desc *ldac_gpio)
    {
    gpiod_set_value_cansleep(ldac_gpio, 1);
    fsleep(AD3530R_LDAC_PULSE_US);
    gpiod_set_value_cansleep(ldac_gpio, 0);
    return 0;
    }
    static int ad3530r_dac_write(struct ad3530r_state *st, unsigned int chan,
    unsigned int val)
    {
    int ret;
    guard(mutex)(&st.lock);
    st.buf = cpu_to_be16(val);
    ret = regmap_bulk_write(st.regmap, st.chip_info.input_ch_reg(chan),
    &st.buf, sizeof(st.buf));
    if (ret)
    return ret;
    if (st.ldac_gpio)
    return ad3530r_trigger_hw_ldac(st.ldac_gpio);
    return regmap_set_bits(st.regmap, st.chip_info.sw_ldac_trig_reg(chan),
    AD3530R_SLD_TRIG_A);
    }
    static int ad3530r_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long info)
    {
    struct ad3530r_state *st = iio_priv(indio_dev);
    int ret;
    guard(mutex)(&st.lock);
    switch (info) {
    case IIO_CHAN_INFO_RAW:
    ret = regmap_bulk_read(st.regmap,
    st.chip_info.input_ch_reg(chan.channel),
    &st.buf, sizeof(st.buf));
    if (ret)
    return ret;
// val = FIELD_GET(AD3530R_REG_VAL_MASK, be16_to_cpu(st->buf));
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
// val = st->vref_mV;
// val2 = 16;
    return IIO_VAL_FRACTIONAL_LOG2;
    default:
    return -EINVAL;
    }
    }
    static int ad3530r_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long info)
    {
    struct ad3530r_state *st = iio_priv(indio_dev);
    switch (info) {
    case IIO_CHAN_INFO_RAW:
    if (val < 0 || val > AD3530R_DAC_MAX_VAL)
    return -EINVAL;
    return ad3530r_dac_write(st, chan.channel, val);
    default:
    return -EINVAL;
    }
    }
    static int ad3530r_reg_access(struct iio_dev *indio_dev, unsigned int reg,
    unsigned int writeval, unsigned int *readval)
    {
    struct ad3530r_state *st = iio_priv(indio_dev);
    if (readval)
    return regmap_read(st.regmap, reg, readval);
    return regmap_write(st.regmap, reg, writeval);
    }
    static const struct iio_chan_spec_ext_info ad3530r_ext_info[] = {
    {
    .name = "powerdown",
    .shared = IIO_SEPARATE,
    .read = ad3530r_get_dac_powerdown,
    .write = ad3530r_set_dac_powerdown,
    },
    IIO_ENUM("powerdown_mode", IIO_SEPARATE, &ad3530r_powerdown_mode_enum),
    IIO_ENUM_AVAILABLE("powerdown_mode", IIO_SHARED_BY_TYPE,
    &ad3530r_powerdown_mode_enum),
    { }
    };
    static const struct iio_chan_spec_ext_info ad3531r_ext_info[] = {
    {
    .name = "powerdown",
    .shared = IIO_SEPARATE,
    .read = ad3530r_get_dac_powerdown,
    .write = ad3530r_set_dac_powerdown,
    },
    IIO_ENUM("powerdown_mode", IIO_SEPARATE, &ad3531r_powerdown_mode_enum),
    IIO_ENUM_AVAILABLE("powerdown_mode", IIO_SHARED_BY_TYPE,
    &ad3531r_powerdown_mode_enum),
    { }
    };
    static const struct iio_chan_spec_ext_info ad3532r_ext_info[] = {
    {
    .name = "powerdown",
    .shared = IIO_SEPARATE,
    .read = ad3530r_get_dac_powerdown,
    .write = ad3532r_set_dac_powerdown,
    },
    IIO_ENUM("powerdown_mode", IIO_SEPARATE, &ad3532r_powerdown_mode_enum),
    IIO_ENUM_AVAILABLE("powerdown_mode", IIO_SHARED_BY_TYPE,
    &ad3532r_powerdown_mode_enum),
    { }
    };

    {								\
    .type = IIO_VOLTAGE,					\
    .indexed = 1,						\
    .channel = _chan,					\
    .output = 1,						\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |		\
    BIT(IIO_CHAN_INFO_SCALE),		\
    .ext_info = _ext_info,					\
    }
    static const struct iio_chan_spec ad3530r_channels[] = {
    AD3530R_CHAN(0, ad3530r_ext_info),
    AD3530R_CHAN(1, ad3530r_ext_info),
    AD3530R_CHAN(2, ad3530r_ext_info),
    AD3530R_CHAN(3, ad3530r_ext_info),
    AD3530R_CHAN(4, ad3530r_ext_info),
    AD3530R_CHAN(5, ad3530r_ext_info),
    AD3530R_CHAN(6, ad3530r_ext_info),
    AD3530R_CHAN(7, ad3530r_ext_info),
    };
    static const struct iio_chan_spec ad3531r_channels[] = {
    AD3530R_CHAN(0, ad3531r_ext_info),
    AD3530R_CHAN(1, ad3531r_ext_info),
    AD3530R_CHAN(2, ad3531r_ext_info),
    AD3530R_CHAN(3, ad3531r_ext_info),
    };
    static const struct iio_chan_spec ad3532r_channels[] = {
    AD3530R_CHAN(0, ad3532r_ext_info),
    AD3530R_CHAN(1, ad3532r_ext_info),
    AD3530R_CHAN(2, ad3532r_ext_info),
    AD3530R_CHAN(3, ad3532r_ext_info),
    AD3530R_CHAN(4, ad3532r_ext_info),
    AD3530R_CHAN(5, ad3532r_ext_info),
    AD3530R_CHAN(6, ad3532r_ext_info),
    AD3530R_CHAN(7, ad3532r_ext_info),
    AD3530R_CHAN(8, ad3532r_ext_info),
    AD3530R_CHAN(9, ad3532r_ext_info),
    AD3530R_CHAN(10, ad3532r_ext_info),
    AD3530R_CHAN(11, ad3532r_ext_info),
    AD3530R_CHAN(12, ad3532r_ext_info),
    AD3530R_CHAN(13, ad3532r_ext_info),
    AD3530R_CHAN(14, ad3532r_ext_info),
    AD3530R_CHAN(15, ad3532r_ext_info),
    };
    static const unsigned int ad3530r_if_config[] = {
    AD3530R_INTERFACE_CONFIG_A,
    };
    static const unsigned int ad3530r_out_ctrl[] = {
    AD3530R_OUTPUT_CONTROL_0,
    };
    static const unsigned int ad3530r_ref_ctrl[] = {
    AD3530R_REFERENCE_CONTROL_0,
    };
    static const unsigned int ad3530r_op_mode[] = {
    AD3530R_OUTPUT_OPERATING_MODE_0,
    AD3530R_OUTPUT_OPERATING_MODE_1,
    };
    static const unsigned int ad3531r_op_mode[] = {
    AD3530R_OUTPUT_OPERATING_MODE_0,
    };
    static const unsigned int ad3532r_if_config[] = {
    AD3532R_INTERFACE_CONFIG_A_0,
    AD3532R_INTERFACE_CONFIG_A_1,
    };
    static const unsigned int ad3532r_out_ctrl[] = {
    AD3532R_OUTPUT_CONTROL_0,
    AD3532R_OUTPUT_CONTROL_1,
    };
    static const unsigned int ad3532r_ref_ctrl[] = {
    AD3532R_REFERENCE_CONTROL_0,
    AD3532R_REFERENCE_CONTROL_1,
    };
    static const unsigned int ad3532r_op_mode[] = {
    AD3532R_OUTPUT_OPERATING_MODE_0,
    AD3532R_OUTPUT_OPERATING_MODE_1,
    AD3532R_OUTPUT_OPERATING_MODE_2,
    AD3532R_OUTPUT_OPERATING_MODE_3,
    };
    static const struct regmap_config ad3530r_regmap_config = {
    .reg_bits = 16,
    .val_bits = 8,
    .max_register = AD3530R_MAX_REG_ADDR,
    };
    static const struct regmap_config ad3532r_regmap_config = {
    .reg_bits = 16,
    .val_bits = 8,
    .max_register = AD3532R_MAX_REG_ADDR,
    };
    static const struct ad3530r_chip_info ad3530_chip = {
    .name = "ad3530",
    .channels = ad3530r_channels,
    .regmap_config = &ad3530r_regmap_config,
    .num_channels = ARRAY_SIZE(ad3530r_channels),
    .sw_ldac_trig_reg = ad3530r_trigger_sw_ldac_reg,
    .input_ch_reg = ad3530r_input_ch_reg,
    .interface_config_a = ad3530r_if_config,
    .output_control = ad3530r_out_ctrl,
    .reference_control = ad3530r_ref_ctrl,
    .op_mode = ad3530r_op_mode,
    .num_banks = ARRAY_SIZE(ad3530r_if_config),
    .num_op_mode_regs = ARRAY_SIZE(ad3530r_op_mode),
    .internal_ref_support = false,
    };
    static const struct ad3530r_chip_info ad3530r_chip = {
    .name = "ad3530r",
    .channels = ad3530r_channels,
    .regmap_config = &ad3530r_regmap_config,
    .num_channels = ARRAY_SIZE(ad3530r_channels),
    .sw_ldac_trig_reg = ad3530r_trigger_sw_ldac_reg,
    .input_ch_reg = ad3530r_input_ch_reg,
    .interface_config_a = ad3530r_if_config,
    .output_control = ad3530r_out_ctrl,
    .reference_control = ad3530r_ref_ctrl,
    .op_mode = ad3530r_op_mode,
    .num_banks = ARRAY_SIZE(ad3530r_if_config),
    .num_op_mode_regs = ARRAY_SIZE(ad3530r_op_mode),
    .internal_ref_support = true,
    };
    static const struct ad3530r_chip_info ad3531_chip = {
    .name = "ad3531",
    .channels = ad3531r_channels,
    .regmap_config = &ad3530r_regmap_config,
    .num_channels = ARRAY_SIZE(ad3531r_channels),
    .sw_ldac_trig_reg = ad3531r_trigger_sw_ldac_reg,
    .input_ch_reg = ad3531r_input_ch_reg,
    .interface_config_a = ad3530r_if_config,
    .output_control = ad3530r_out_ctrl,
    .reference_control = ad3530r_ref_ctrl,
    .op_mode = ad3531r_op_mode,
    .num_banks = ARRAY_SIZE(ad3530r_if_config),
    .num_op_mode_regs = ARRAY_SIZE(ad3531r_op_mode),
    .internal_ref_support = false,
    };
    static const struct ad3530r_chip_info ad3531r_chip = {
    .name = "ad3531r",
    .channels = ad3531r_channels,
    .regmap_config = &ad3530r_regmap_config,
    .num_channels = ARRAY_SIZE(ad3531r_channels),
    .sw_ldac_trig_reg = ad3531r_trigger_sw_ldac_reg,
    .input_ch_reg = ad3531r_input_ch_reg,
    .interface_config_a = ad3530r_if_config,
    .output_control = ad3530r_out_ctrl,
    .reference_control = ad3530r_ref_ctrl,
    .op_mode = ad3531r_op_mode,
    .num_banks = ARRAY_SIZE(ad3530r_if_config),
    .num_op_mode_regs = ARRAY_SIZE(ad3531r_op_mode),
    .internal_ref_support = true,
    };
    static const struct ad3530r_chip_info ad3532_chip = {
    .name = "ad3532",
    .channels = ad3532r_channels,
    .regmap_config = &ad3532r_regmap_config,
    .num_channels = ARRAY_SIZE(ad3532r_channels),
    .sw_ldac_trig_reg = ad3532r_trigger_sw_ldac_reg,
    .input_ch_reg = ad3532r_input_ch_reg,
    .interface_config_a = ad3532r_if_config,
    .output_control = ad3532r_out_ctrl,
    .reference_control = ad3532r_ref_ctrl,
    .op_mode = ad3532r_op_mode,
    .num_banks = ARRAY_SIZE(ad3532r_if_config),
    .num_op_mode_regs = ARRAY_SIZE(ad3532r_op_mode),
    .internal_ref_support = false,
    };
    static const struct ad3530r_chip_info ad3532r_chip = {
    .name = "ad3532r",
    .channels = ad3532r_channels,
    .regmap_config = &ad3532r_regmap_config,
    .num_channels = ARRAY_SIZE(ad3532r_channels),
    .sw_ldac_trig_reg = ad3532r_trigger_sw_ldac_reg,
    .input_ch_reg = ad3532r_input_ch_reg,
    .interface_config_a = ad3532r_if_config,
    .output_control = ad3532r_out_ctrl,
    .reference_control = ad3532r_ref_ctrl,
    .op_mode = ad3532r_op_mode,
    .num_banks = ARRAY_SIZE(ad3532r_if_config),
    .num_op_mode_regs = ARRAY_SIZE(ad3532r_op_mode),
    .internal_ref_support = true,
    };
    static int ad3530r_set_reg_bank_bits(const struct ad3530r_state *st,
    const unsigned int *regs,
    unsigned int num_regs,
    unsigned int mask)
    {
    int ret;
    for (unsigned int i = 0; i < num_regs; i++) {
    ret = regmap_set_bits(st.regmap, regs[i], mask);
    if (ret)
    return ret;
    }
    return 0;
    }
    static int ad3530r_write_reg_banks(const struct ad3530r_state *st,
    const unsigned int *regs,
    unsigned int num_regs,
    unsigned int val)
    {
    int ret;
    for (unsigned int i = 0; i < num_regs; i++) {
    ret = regmap_write(st.regmap, regs[i], val);
    if (ret)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ad3530r_setup(st: *mut ad3530r_state, external_vref_uV: c_int) -> c_int {
    static int ad3530r_setup(struct ad3530r_state *st, int external_vref_uV)
    {
    const struct ad3530r_chip_info *chip_info = st.chip_info;
    struct device *dev = regmap_get_device(st.regmap);
    struct gpio_desc *reset_gpio;
    u8 range_multiplier, val;
    int ret;
    reset_gpio = devm_gpiod_get_optional(dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(reset_gpio))
    return dev_err_probe(dev, PTR_ERR(reset_gpio),
    "Failed to get reset GPIO\n");
    if (reset_gpio) {
// Perform hardware reset
    fsleep(1 * USEC_PER_MSEC);
    gpiod_set_value_cansleep(reset_gpio, 0);
    } else {
// Perform software reset
    ret = ad3530r_set_reg_bank_bits(st, chip_info.interface_config_a,
    chip_info.num_banks,
    AD3530R_SW_RESET);
    if (ret)
    return ret;
    }
    fsleep(10 * USEC_PER_MSEC);
    range_multiplier = 1;
    if (device_property_read_bool(dev, "adi,range-double")) {
    ret = ad3530r_set_reg_bank_bits(st, chip_info.output_control,
    chip_info.num_banks,
    AD3530R_OUTPUT_CONTROL_RANGE);
    if (ret)
    return ret;
    range_multiplier = 2;
    }
    if (external_vref_uV) {
    st.vref_mV = range_multiplier * external_vref_uV / MILLI;
    } else {
    ret = ad3530r_set_reg_bank_bits(st, chip_info.reference_control,
    chip_info.num_banks,
    AD3530R_REFERENCE_CONTROL_SEL);
    if (ret)
    return ret;
    st.vref_mV = range_multiplier * AD3530R_INTERNAL_VREF_mV;
    }
// Set normal operating mode for all channels
    val = FIELD_PREP(AD3530R_OP_MODE_CHAN_MSK(0), AD3530R_NORMAL_OP) |
    FIELD_PREP(AD3530R_OP_MODE_CHAN_MSK(1), AD3530R_NORMAL_OP) |
    FIELD_PREP(AD3530R_OP_MODE_CHAN_MSK(2), AD3530R_NORMAL_OP) |
    FIELD_PREP(AD3530R_OP_MODE_CHAN_MSK(3), AD3530R_NORMAL_OP);
    ret = ad3530r_write_reg_banks(st, st.chip_info.op_mode,
    st.chip_info.num_op_mode_regs, val);
    if (ret)
    return ret;
    for (unsigned int i = 0; i < st.chip_info.num_channels; i++)
    st.chan[i].powerdown_mode = AD3530R_POWERDOWN_32K;
    st.ldac_gpio = devm_gpiod_get_optional(dev, "ldac", GPIOD_OUT_LOW);
    if (IS_ERR(st.ldac_gpio))
    return dev_err_probe(dev, PTR_ERR(st.ldac_gpio),
    "Failed to get ldac GPIO\n");
    return 0;
    }
    static const struct iio_info ad3530r_info = {
    .read_raw = ad3530r_read_raw,
    .write_raw = ad3530r_write_raw,
    .debugfs_reg_access = ad3530r_reg_access,
    };
#[no_mangle]
unsafe extern "C" fn ad3530r_probe(spi: *mut spi_device) -> c_int {
    static int ad3530r_probe(struct spi_device *spi)
    {
    static const char * const regulators[] = { "vdd", "iovdd" };
    struct device *dev = &spi.dev;
    struct iio_dev *indio_dev;
    struct ad3530r_state *st;
    int ret, external_vref_uV;
    indio_dev = devm_iio_device_alloc(&spi.dev, sizeof(*st));
    if (!indio_dev)
    return -ENOMEM;
    st = iio_priv(indio_dev);
    st.chip_info = spi_get_device_match_data(spi);
    if (!st.chip_info)
    return -ENODEV;
    st.regmap = devm_regmap_init_spi(spi, st.chip_info.regmap_config);
    if (IS_ERR(st.regmap))
    return dev_err_probe(dev, PTR_ERR(st.regmap),
    "Failed to init regmap");
    ret = devm_mutex_init(dev, &st.lock);
    if (ret)
    return ret;
    ret = devm_regulator_bulk_get_enable(dev, ARRAY_SIZE(regulators),
    regulators);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to enable regulators\n");
    external_vref_uV = devm_regulator_get_enable_read_voltage(dev, "ref");
    if (external_vref_uV < 0 && external_vref_uV != -ENODEV)
    return external_vref_uV;
    if (external_vref_uV == -ENODEV)
    external_vref_uV = 0;
    if (!st.chip_info.internal_ref_support && external_vref_uV == 0)
    return -ENODEV;
    ret = ad3530r_setup(st, external_vref_uV);
    if (ret)
    return ret;
    indio_dev.name = st.chip_info.name;
    indio_dev.info = &ad3530r_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = st.chip_info.channels;
    indio_dev.num_channels = st.chip_info.num_channels;
    return devm_iio_device_register(&spi.dev, indio_dev);
    }
    static const struct spi_device_id ad3530r_id[] = {
    { .name = "ad3530", .driver_data = (kernel_ulong_t)&ad3530_chip },
    { .name = "ad3530r", .driver_data = (kernel_ulong_t)&ad3530r_chip },
    { .name = "ad3531", .driver_data = (kernel_ulong_t)&ad3531_chip },
    { .name = "ad3531r", .driver_data = (kernel_ulong_t)&ad3531r_chip },
    { .name = "ad3532", .driver_data = (kernel_ulong_t)&ad3532_chip },
    { .name = "ad3532r", .driver_data = (kernel_ulong_t)&ad3532r_chip },
    { }
    };
    MODULE_DEVICE_TABLE(spi, ad3530r_id);
    static const struct of_device_id ad3530r_of_match[] = {
    { .compatible = "adi,ad3530", .data = &ad3530_chip },
    { .compatible = "adi,ad3530r", .data = &ad3530r_chip },
    { .compatible = "adi,ad3531", .data = &ad3531_chip },
    { .compatible = "adi,ad3531r", .data = &ad3531r_chip },
    { .compatible = "adi,ad3532", .data = &ad3532_chip },
    { .compatible = "adi,ad3532r", .data = &ad3532r_chip },
    { }
    };
    MODULE_DEVICE_TABLE(of, ad3530r_of_match);
    static struct spi_driver ad3530r_driver = {
    .driver = {
    .name = "ad3530r",
    .of_match_table = ad3530r_of_match,
    },
    .probe = ad3530r_probe,
    .id_table = ad3530r_id,
    };
    module_spi_driver(ad3530r_driver);
    MODULE_AUTHOR("Kim Seer Paller <kimseer.paller@analog.com>");
    MODULE_DESCRIPTION("Analog Devices AD3530R and Similar DACs Driver");
    MODULE_LICENSE("GPL");
