//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/nau7802.c
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
// Driver for the Nuvoton NAU7802 ADC
//
// Copyright 2013 Free Electrons
//

pub const NAU7802_REG_PUCTRL: c_uint = 0x00;

pub const NAU7802_REG_CTRL1: c_uint = 0x01;

pub const NAU7802_CTRL1_GAINS_BITS: c_uint = 0x07;
pub const NAU7802_REG_CTRL2: c_uint = 0x02;

pub const NAU7802_SAMP_FREQ_320: c_uint = 0x07;

pub const NAU7802_REG_ADC_B2: c_uint = 0x12;
pub const NAU7802_REG_ADC_B1: c_uint = 0x13;
pub const NAU7802_REG_ADC_B0: c_uint = 0x14;
pub const NAU7802_REG_ADC_CTRL: c_uint = 0x15;
pub const NAU7802_MIN_CONVERSIONS: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nau7802_state {
    pub client: *mut i2c_client,
    pub last_value: i32,
    pub lock: mutex,
    pub data_lock: mutex,
    pub vref_mv: u32,
    pub conversion_count: u32,
    pub sample_rate: u8,
    pub scale_avail: [u32; 8],
    pub value_ok: completion,
}

    .type = IIO_VOLTAGE,					\
    .indexed = 1,						\
    .channel = (chan),					\
    .scan_index = (chan),					\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),		\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE) |	\
    BIT(IIO_CHAN_INFO_SAMP_FREQ)	\
    }
    static const struct iio_chan_spec nau7802_chan_array[] = {
    NAU7802_CHANNEL(0),
    NAU7802_CHANNEL(1),
    };
    static const u16 nau7802_sample_freq_avail[] = {10, 20, 40, 80,
    10, 10, 10, 320};
    static ssize_t nau7802_show_scales(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct nau7802_state *st = iio_priv(dev_to_iio_dev(dev));
    int i, len = 0;
    for (i = 0; i < ARRAY_SIZE(st.scale_avail); i++)
    len += scnprintf(buf + len, PAGE_SIZE - len, "0.%09d ",
    st.scale_avail[i]);
    buf[len-1] = '\n';
    return len;
    }
    static IIO_CONST_ATTR_SAMP_FREQ_AVAIL("10 40 80 320");
    static IIO_DEVICE_ATTR(in_voltage_scale_available, S_IRUGO, nau7802_show_scales,
    core::ptr::null_mut(), 0);
    static struct attribute *nau7802_attributes[] = {
    &iio_const_attr_sampling_frequency_available.dev_attr.attr,
    &iio_dev_attr_in_voltage_scale_available.dev_attr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group nau7802_attribute_group = {
    .attrs = nau7802_attributes,
    };
#[no_mangle]
unsafe extern "C" fn nau7802_set_gain(st: *mut nau7802_state, gain: c_int) -> c_int {
    static int nau7802_set_gain(struct nau7802_state *st, int gain)
    {
    int ret;
    mutex_lock(&st.lock);
    st.conversion_count = 0;
    ret = i2c_smbus_read_byte_data(st.client, NAU7802_REG_CTRL1);
    if (ret < 0)
    goto nau7802_sysfs_set_gain_out;
    ret = i2c_smbus_write_byte_data(st.client, NAU7802_REG_CTRL1,
    (ret & (~NAU7802_CTRL1_GAINS_BITS)) |
    gain);
    nau7802_sysfs_set_gain_out:
    mutex_unlock(&st.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn nau7802_read_conversion(st: *mut nau7802_state) -> c_int {
    static int nau7802_read_conversion(struct nau7802_state *st)
    {
    int data;
    mutex_lock(&st.data_lock);
    data = i2c_smbus_read_byte_data(st.client, NAU7802_REG_ADC_B2);
    if (data < 0)
    goto nau7802_read_conversion_out;
    st.last_value = data << 16;
    data = i2c_smbus_read_byte_data(st.client, NAU7802_REG_ADC_B1);
    if (data < 0)
    goto nau7802_read_conversion_out;
    st.last_value |= data << 8;
    data = i2c_smbus_read_byte_data(st.client, NAU7802_REG_ADC_B0);
    if (data < 0)
    goto nau7802_read_conversion_out;
    st.last_value |= data;
    st.last_value = sign_extend32(st.last_value, 23);
    nau7802_read_conversion_out:
    mutex_unlock(&st.data_lock);
    return data;
    }
//
// Conversions are synchronised on the rising edge of NAU7802_PUCTRL_CS_BIT
//
#[no_mangle]
unsafe extern "C" fn nau7802_sync(st: *mut nau7802_state) -> c_int {
    static int nau7802_sync(struct nau7802_state *st)
    {
    int ret;
    ret = i2c_smbus_read_byte_data(st.client, NAU7802_REG_PUCTRL);
    if (ret < 0)
    return ret;
    ret = i2c_smbus_write_byte_data(st.client, NAU7802_REG_PUCTRL,
    ret | NAU7802_PUCTRL_CS_BIT);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn nau7802_eoc_trigger(irq: c_int, private: *mut c_void) -> irqreturn_t {
    static irqreturn_t nau7802_eoc_trigger(int irq, void *private)
    {
    struct iio_dev *indio_dev = private;
    struct nau7802_state *st = iio_priv(indio_dev);
    int status;
    status = i2c_smbus_read_byte_data(st.client, NAU7802_REG_PUCTRL);
    if (status < 0)
    return IRQ_HANDLED;
    if (!(status & NAU7802_PUCTRL_CR_BIT))
    return IRQ_NONE;
    if (nau7802_read_conversion(st) < 0)
    return IRQ_HANDLED;
//
// Because there is actually only one ADC for both channels, we have to
// wait for enough conversions to happen before getting a significant
// value when changing channels and the values are far apart.
//
    if (st.conversion_count < NAU7802_MIN_CONVERSIONS)
    st.conversion_count++;
    if (st.conversion_count >= NAU7802_MIN_CONVERSIONS)
    complete(&st.value_ok);
    return IRQ_HANDLED;
    }
    static int nau7802_read_irq(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val)
    {
    struct nau7802_state *st = iio_priv(indio_dev);
    int ret;
    reinit_completion(&st.value_ok);
    enable_irq(st.client.irq);
    nau7802_sync(st);
// read registers to ensure we flush everything
    ret = nau7802_read_conversion(st);
    if (ret < 0)
    goto read_chan_info_failure;
// Wait for a conversion to finish
    ret = wait_for_completion_interruptible_timeout(&st.value_ok,
    msecs_to_jiffies(1000));
    if (ret == 0)
    ret = -ETIMEDOUT;
    if (ret < 0)
    goto read_chan_info_failure;
    disable_irq(st.client.irq);
// val = st->last_value;
    return IIO_VAL_INT;
    read_chan_info_failure:
    disable_irq(st.client.irq);
    return ret;
    }
    static int nau7802_read_poll(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val)
    {
    struct nau7802_state *st = iio_priv(indio_dev);
    int ret;
    nau7802_sync(st);
// read registers to ensure we flush everything
    ret = nau7802_read_conversion(st);
    if (ret < 0)
    return ret;
//
// Because there is actually only one ADC for both channels, we have to
// wait for enough conversions to happen before getting a significant
// value when changing channels and the values are far apart.
//
    do {
    ret = i2c_smbus_read_byte_data(st.client, NAU7802_REG_PUCTRL);
    if (ret < 0)
    return ret;
    while (!(ret & NAU7802_PUCTRL_CR_BIT)) {
    if (st.sample_rate != NAU7802_SAMP_FREQ_320)
    msleep(20);
    else
    mdelay(4);
    ret = i2c_smbus_read_byte_data(st.client,
    NAU7802_REG_PUCTRL);
    if (ret < 0)
    return ret;
    }
    ret = nau7802_read_conversion(st);
    if (ret < 0)
    return ret;
    if (st.conversion_count < NAU7802_MIN_CONVERSIONS)
    st.conversion_count++;
    } while (st.conversion_count < NAU7802_MIN_CONVERSIONS);
// val = st->last_value;
    return IIO_VAL_INT;
    }
    static int nau7802_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct nau7802_state *st = iio_priv(indio_dev);
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    mutex_lock(&st.lock);
//
// Select the channel to use
// - Channel 1 is value 0 in the CHS register
// - Channel 2 is value 1 in the CHS register
//
    ret = i2c_smbus_read_byte_data(st.client, NAU7802_REG_CTRL2);
    if (ret < 0) {
    mutex_unlock(&st.lock);
    return ret;
    }
    if (((ret & NAU7802_CTRL2_CHS_BIT) && !chan.channel) ||
    (!(ret & NAU7802_CTRL2_CHS_BIT) &&
    chan.channel)) {
    st.conversion_count = 0;
    ret = i2c_smbus_write_byte_data(st.client,
    NAU7802_REG_CTRL2,
    NAU7802_CTRL2_CHS(chan.channel) |
    NAU7802_CTRL2_CRS(st.sample_rate));
    if (ret < 0) {
    mutex_unlock(&st.lock);
    return ret;
    }
    }
    if (st.client.irq)
    ret = nau7802_read_irq(indio_dev, chan, val);
    else
    ret = nau7802_read_poll(indio_dev, chan, val);
    mutex_unlock(&st.lock);
    return ret;
    case IIO_CHAN_INFO_SCALE:
    ret = i2c_smbus_read_byte_data(st.client, NAU7802_REG_CTRL1);
    if (ret < 0)
    return ret;
//
// We have 24 bits of signed data, that means 23 bits of data
// plus the sign bit
//
// val = st->vref_mv;
// val2 = 23 + (ret & NAU7802_CTRL1_GAINS_BITS);
    return IIO_VAL_FRACTIONAL_LOG2;
    case IIO_CHAN_INFO_SAMP_FREQ:
// val =  nau7802_sample_freq_avail[st->sample_rate];
// val2 = 0;
    return IIO_VAL_INT;
    default:
    break;
    }
    return -EINVAL;
    }
    static int nau7802_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    struct nau7802_state *st = iio_priv(indio_dev);
    int i, ret;
    switch (mask) {
    case IIO_CHAN_INFO_SCALE:
    for (i = 0; i < ARRAY_SIZE(st.scale_avail); i++)
    if (val2 == st.scale_avail[i])
    return nau7802_set_gain(st, i);
    break;
    case IIO_CHAN_INFO_SAMP_FREQ:
    for (i = 0; i < ARRAY_SIZE(nau7802_sample_freq_avail); i++)
    if (val == nau7802_sample_freq_avail[i]) {
    mutex_lock(&st.lock);
    st.sample_rate = i;
    st.conversion_count = 0;
    ret = i2c_smbus_write_byte_data(st.client,
    NAU7802_REG_CTRL2,
    NAU7802_CTRL2_CRS(st.sample_rate));
    mutex_unlock(&st.lock);
    return ret;
    }
    break;
    default:
    break;
    }
    return -EINVAL;
    }
    static int nau7802_write_raw_get_fmt(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    long mask)
    {
    return IIO_VAL_INT_PLUS_NANO;
    }
    static const struct iio_info nau7802_info = {
    .read_raw = &nau7802_read_raw,
    .write_raw = &nau7802_write_raw,
    .write_raw_get_fmt = nau7802_write_raw_get_fmt,
    .attrs = &nau7802_attribute_group,
    };
#[no_mangle]
unsafe extern "C" fn nau7802_probe(client: *mut i2c_client) -> c_int {
    static int nau7802_probe(struct i2c_client *client)
    {
    struct iio_dev *indio_dev;
    struct nau7802_state *st;
    int i, ret;
    u8 data;
    let mut tmp: u32 = 0;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*st));
    if (indio_dev == core::ptr::null_mut())
    return -ENOMEM;
    st = iio_priv(indio_dev);
    indio_dev.name = dev_name(&client.dev);
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.info = &nau7802_info;
    st.client = client;
// Reset the device
    ret = i2c_smbus_write_byte_data(st.client, NAU7802_REG_PUCTRL,
    NAU7802_PUCTRL_RR_BIT);
    if (ret < 0)
    return ret;
// Enter normal operation mode
    ret = i2c_smbus_write_byte_data(st.client, NAU7802_REG_PUCTRL,
    NAU7802_PUCTRL_PUD_BIT);
    if (ret < 0)
    return ret;
//
// After about 200 usecs, the device should be ready and then
// the Power Up bit will be set to 1. If not, wait for it.
//
    udelay(210);
    ret = i2c_smbus_read_byte_data(st.client, NAU7802_REG_PUCTRL);
    if (ret < 0)
    return ret;
    if (!(ret & NAU7802_PUCTRL_PUR_BIT))
    return ret;
    device_property_read_u32(&client.dev, "nuvoton,vldo", &tmp);
    st.vref_mv = tmp;
    data = NAU7802_PUCTRL_PUD_BIT | NAU7802_PUCTRL_PUA_BIT |
    NAU7802_PUCTRL_CS_BIT;
    if (tmp >= 2400)
    data |= NAU7802_PUCTRL_AVDDS_BIT;
    ret = i2c_smbus_write_byte_data(st.client, NAU7802_REG_PUCTRL, data);
    if (ret < 0)
    return ret;
    ret = i2c_smbus_write_byte_data(st.client, NAU7802_REG_ADC_CTRL, 0x30);
    if (ret < 0)
    return ret;
    if (tmp >= 2400) {
    data = NAU7802_CTRL1_VLDO((4500 - tmp) / 300);
    ret = i2c_smbus_write_byte_data(st.client, NAU7802_REG_CTRL1,
    data);
    if (ret < 0)
    return ret;
    }
// Populate available ADC input ranges
    for (i = 0; i < ARRAY_SIZE(st.scale_avail); i++)
    st.scale_avail[i] = (((u64)st.vref_mv) * 1000000000ULL)
    >> (23 + i);
    init_completion(&st.value_ok);
//
// The ADC fires continuously and we can't do anything about
// it. So we need to have the IRQ disabled by default, and we
// will enable them back when we will need them..
//
    if (client.irq) {
    ret = devm_request_threaded_irq(&client.dev, client.irq,
    core::ptr::null_mut(),
    nau7802_eoc_trigger,
    IRQF_TRIGGER_HIGH | IRQF_ONESHOT |
    IRQF_NO_AUTOEN,
    client.dev.driver.name,
    indio_dev);
    if (ret) {
//
// What may happen here is that our IRQ controller is
// not able to get level interrupt but this is required
// by this ADC as when going over 40 sample per second,
// the interrupt line may stay high between conversions.
// So, we continue no matter what but we switch to
// polling mode.
//
    dev_info(&client.dev,
    "Failed to allocate IRQ, using polling mode\n");
    client.irq = 0;
    }
    }
    if (!client.irq) {
//
// We are polling, use the fastest sample rate by
// default
//
    st.sample_rate = NAU7802_SAMP_FREQ_320;
    ret = i2c_smbus_write_byte_data(st.client, NAU7802_REG_CTRL2,
    NAU7802_CTRL2_CRS(st.sample_rate));
    if (ret)
    return ret;
    }
// Setup the ADC channels available on the board
    indio_dev.num_channels = ARRAY_SIZE(nau7802_chan_array);
    indio_dev.channels = nau7802_chan_array;
    mutex_init(&st.lock);
    mutex_init(&st.data_lock);
    return devm_iio_device_register(&client.dev, indio_dev);
    }
    static const struct i2c_device_id nau7802_i2c_id[] = {
    { .name = "nau7802" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, nau7802_i2c_id);
    static const struct of_device_id nau7802_dt_ids[] = {
    { .compatible = "nuvoton,nau7802" },
    { }
    };
    MODULE_DEVICE_TABLE(of, nau7802_dt_ids);
    static struct i2c_driver nau7802_driver = {
    .probe = nau7802_probe,
    .id_table = nau7802_i2c_id,
    .driver = {
    .name = "nau7802",
    .of_match_table = nau7802_dt_ids,
    },
    };
    module_i2c_driver(nau7802_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Nuvoton NAU7802 ADC Driver");
    MODULE_AUTHOR("Maxime Ripard <maxime.ripard@free-electrons.com>");
    MODULE_AUTHOR("Alexandre Belloni <alexandre.belloni@free-electrons.com>");
