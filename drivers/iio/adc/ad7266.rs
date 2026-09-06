//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/ad7266.c
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
// AD7266/65 SPI ADC driver
//
// Copyright 2012 Analog Devices Inc.
//

pub const AD7266_INTERNAL_REF_MV: c_int = 2500;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad7266_state {
    pub spi: *mut spi_device,
    pub vref_mv: c_ulong,
    pub single_xfer: [spi_transfer; 3],
    pub single_msg: spi_message,
    pub range: enum ad7266_range,
    pub mode: enum ad7266_mode,
    pub fixed_addr: bool,
    pub gpios: [*mut gpio_desc; 3],
//
// DMA (thus cache coherency maintenance) may require the
// transfer buffers to live in their own cache lines.
// The buffer needs to be large enough to hold two samples (4 bytes) and
// the naturally aligned timestamp (8 bytes).
//
    struct {
    pub sample: [__be16; 2],
    pub timestamp: aligned_s64,
    pub __aligned(IIO_DMA_MINALIGN): } data,
}

#[no_mangle]
unsafe extern "C" fn ad7266_wakeup(st: *mut ad7266_state) -> c_int {
    static int ad7266_wakeup(struct ad7266_state *st)
    {
// Any read with >= 2 bytes will wake the device
    return spi_read(st.spi, &st.data.sample[0], 2);
    }
#[no_mangle]
unsafe extern "C" fn ad7266_powerdown(st: *mut ad7266_state) -> c_int {
    static int ad7266_powerdown(struct ad7266_state *st)
    {
// Any read with < 2 bytes will powerdown the device
    return spi_read(st.spi, &st.data.sample[0], 1);
    }
#[no_mangle]
unsafe extern "C" fn ad7266_preenable(indio_dev: *mut iio_dev) -> c_int {
    static int ad7266_preenable(struct iio_dev *indio_dev)
    {
    struct ad7266_state *st = iio_priv(indio_dev);
    return ad7266_wakeup(st);
    }
#[no_mangle]
unsafe extern "C" fn ad7266_postdisable(indio_dev: *mut iio_dev) -> c_int {
    static int ad7266_postdisable(struct iio_dev *indio_dev)
    {
    struct ad7266_state *st = iio_priv(indio_dev);
    return ad7266_powerdown(st);
    }
    static const struct iio_buffer_setup_ops iio_triggered_buffer_setup_ops = {
    .preenable = &ad7266_preenable,
    .postdisable = &ad7266_postdisable,
    };
#[no_mangle]
unsafe extern "C" fn ad7266_trigger_handler(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t ad7266_trigger_handler(int irq, void *p)
    {
    struct iio_poll_func *pf = p;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct ad7266_state *st = iio_priv(indio_dev);
    int ret;
    ret = spi_read(st.spi, st.data.sample, 4);
    if (ret == 0)
    iio_push_to_buffers_with_ts(indio_dev, &st.data, sizeof(st.data),
    pf.timestamp);
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ad7266_select_input(st: *mut ad7266_state, nr: c_uint) {
    static void ad7266_select_input(struct ad7266_state *st, unsigned int nr)
    {
    unsigned int i;
    if (st.fixed_addr)
    return;
    switch (st.mode) {
    case AD7266_MODE_SINGLE_ENDED:
    nr >>= 1;
    break;
    case AD7266_MODE_PSEUDO_DIFF:
    nr |= 1;
    break;
    case AD7266_MODE_DIFF:
    nr &= ~1;
    break;
    }
    for (i = 0; i < 3; ++i)
    gpiod_set_value(st.gpios[i], (bool)(nr & BIT(i)));
    }
    static int ad7266_update_scan_mode(struct iio_dev *indio_dev,
    const unsigned long *scan_mask)
    {
    struct ad7266_state *st = iio_priv(indio_dev);
    unsigned int nr = find_first_bit(scan_mask,
    iio_get_masklength(indio_dev));
    ad7266_select_input(st, nr);
    return 0;
    }
    static int ad7266_read_single(struct ad7266_state *st, int *val,
    unsigned int address)
    {
    int ret;
    ad7266_select_input(st, address);
    ret = spi_sync(st.spi, &st.single_msg);
// val = be16_to_cpu(st->data.sample[address % 2]);
    return ret;
    }
    static int ad7266_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int *val, int *val2, long m)
    {
    struct ad7266_state *st = iio_priv(indio_dev);
    unsigned long scale_mv;
    int ret;
    switch (m) {
    case IIO_CHAN_INFO_RAW:
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    ret = ad7266_read_single(st, val, chan.address);
    iio_device_release_direct(indio_dev);
    if (ret < 0)
    return ret;
// val = (*val >> 2) & 0xfff;
    if (chan.scan_type.sign == 's')
// val = sign_extend32(*val,
    chan.scan_type.realbits - 1);
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
    scale_mv = st.vref_mv;
    if (st.mode == AD7266_MODE_DIFF)
    scale_mv *= 2;
    if (st.range == AD7266_RANGE_2VREF)
    scale_mv *= 2;
// val = scale_mv;
// val2 = chan->scan_type.realbits;
    return IIO_VAL_FRACTIONAL_LOG2;
    case IIO_CHAN_INFO_OFFSET:
    if (st.range == AD7266_RANGE_2VREF &&
    st.mode != AD7266_MODE_DIFF)
// val = 2048;
    else
// val = 0;
    return IIO_VAL_INT;
    }
    return -EINVAL;
    }

    .type = IIO_VOLTAGE,				\
    .indexed = 1,					\
    .channel = (_chan),				\
    .address = (_chan),				\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),	\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE) \
    | BIT(IIO_CHAN_INFO_OFFSET),			\
    .scan_index = (_chan),				\
    .scan_type = {					\
    .sign = (_sign),			\
    .realbits = 12,				\
    .storagebits = 16,			\
    .shift = 2,				\
    .endianness = IIO_BE,			\
    },						\
    }

    const struct iio_chan_spec ad7266_channels_##_name[] = { \
    AD7266_CHAN(0, (_sign)), \
    AD7266_CHAN(1, (_sign)), \
    AD7266_CHAN(2, (_sign)), \
    AD7266_CHAN(3, (_sign)), \
    AD7266_CHAN(4, (_sign)), \
    AD7266_CHAN(5, (_sign)), \
    AD7266_CHAN(6, (_sign)), \
    AD7266_CHAN(7, (_sign)), \
    AD7266_CHAN(8, (_sign)), \
    AD7266_CHAN(9, (_sign)), \
    AD7266_CHAN(10, (_sign)), \
    AD7266_CHAN(11, (_sign)), \
    IIO_CHAN_SOFT_TIMESTAMP(13), \
    }

    const struct iio_chan_spec ad7266_channels_##_name##_fixed[] = { \
    AD7266_CHAN(0, (_sign)), \
    AD7266_CHAN(1, (_sign)), \
    IIO_CHAN_SOFT_TIMESTAMP(2), \
    }
    static AD7266_DECLARE_SINGLE_ENDED_CHANNELS(u, 'u');
    static AD7266_DECLARE_SINGLE_ENDED_CHANNELS(s, 's');
    static AD7266_DECLARE_SINGLE_ENDED_CHANNELS_FIXED(u, 'u');
    static AD7266_DECLARE_SINGLE_ENDED_CHANNELS_FIXED(s, 's');

    .type = IIO_VOLTAGE,				\
    .indexed = 1,					\
    .channel = (_chan) * 2,				\
    .channel2 = (_chan) * 2 + 1,			\
    .address = (_chan),				\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),	\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE)	\
    | BIT(IIO_CHAN_INFO_OFFSET),			\
    .scan_index = (_chan),				\
    .scan_type = {					\
    .sign = _sign,			\
    .realbits = 12,				\
    .storagebits = 16,			\
    .shift = 2,				\
    .endianness = IIO_BE,			\
    },						\
    .differential = 1,				\
    }

    const struct iio_chan_spec ad7266_channels_diff_##_name[] = { \
    AD7266_CHAN_DIFF(0, (_sign)), \
    AD7266_CHAN_DIFF(1, (_sign)), \
    AD7266_CHAN_DIFF(2, (_sign)), \
    AD7266_CHAN_DIFF(3, (_sign)), \
    AD7266_CHAN_DIFF(4, (_sign)), \
    AD7266_CHAN_DIFF(5, (_sign)), \
    IIO_CHAN_SOFT_TIMESTAMP(6), \
    }
    static AD7266_DECLARE_DIFF_CHANNELS(s, 's');
    static AD7266_DECLARE_DIFF_CHANNELS(u, 'u');

    const struct iio_chan_spec ad7266_channels_diff_fixed_##_name[] = { \
    AD7266_CHAN_DIFF(0, (_sign)), \
    AD7266_CHAN_DIFF(1, (_sign)), \
    IIO_CHAN_SOFT_TIMESTAMP(2), \
    }
    static AD7266_DECLARE_DIFF_CHANNELS_FIXED(s, 's');
    static AD7266_DECLARE_DIFF_CHANNELS_FIXED(u, 'u');
    static const struct iio_info ad7266_info = {
    .read_raw = &ad7266_read_raw,
    .update_scan_mode = &ad7266_update_scan_mode,
    };
    static const unsigned long ad7266_available_scan_masks[] = {
    0x003,
    0x00c,
    0x030,
    0x0c0,
    0x300,
    0xc00,
    0x000,
    };
    static const unsigned long ad7266_available_scan_masks_diff[] = {
    0x003,
    0x00c,
    0x030,
    0x000,
    };
    static const unsigned long ad7266_available_scan_masks_fixed[] = {
    0x003,
    0x000,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad7266_chan_info {
    pub channels: *const iio_chan_spec,
    pub num_channels: c_uint,
    pub scan_masks: *const c_ulong,
}

    (((_differential) << 2) | ((_signed) << 1) | ((_fixed) << 0))
    static const struct ad7266_chan_info ad7266_chan_infos[] = {
    [AD7266_CHAN_INFO_INDEX(0, 0, 0)] = {
    .channels = ad7266_channels_u,
    .num_channels = ARRAY_SIZE(ad7266_channels_u),
    .scan_masks = ad7266_available_scan_masks,
    },
    [AD7266_CHAN_INFO_INDEX(0, 0, 1)] = {
    .channels = ad7266_channels_u_fixed,
    .num_channels = ARRAY_SIZE(ad7266_channels_u_fixed),
    .scan_masks = ad7266_available_scan_masks_fixed,
    },
    [AD7266_CHAN_INFO_INDEX(0, 1, 0)] = {
    .channels = ad7266_channels_s,
    .num_channels = ARRAY_SIZE(ad7266_channels_s),
    .scan_masks = ad7266_available_scan_masks,
    },
    [AD7266_CHAN_INFO_INDEX(0, 1, 1)] = {
    .channels = ad7266_channels_s_fixed,
    .num_channels = ARRAY_SIZE(ad7266_channels_s_fixed),
    .scan_masks = ad7266_available_scan_masks_fixed,
    },
    [AD7266_CHAN_INFO_INDEX(1, 0, 0)] = {
    .channels = ad7266_channels_diff_u,
    .num_channels = ARRAY_SIZE(ad7266_channels_diff_u),
    .scan_masks = ad7266_available_scan_masks_diff,
    },
    [AD7266_CHAN_INFO_INDEX(1, 0, 1)] = {
    .channels = ad7266_channels_diff_fixed_u,
    .num_channels = ARRAY_SIZE(ad7266_channels_diff_fixed_u),
    .scan_masks = ad7266_available_scan_masks_fixed,
    },
    [AD7266_CHAN_INFO_INDEX(1, 1, 0)] = {
    .channels = ad7266_channels_diff_s,
    .num_channels = ARRAY_SIZE(ad7266_channels_diff_s),
    .scan_masks = ad7266_available_scan_masks_diff,
    },
    [AD7266_CHAN_INFO_INDEX(1, 1, 1)] = {
    .channels = ad7266_channels_diff_fixed_s,
    .num_channels = ARRAY_SIZE(ad7266_channels_diff_fixed_s),
    .scan_masks = ad7266_available_scan_masks_fixed,
    },
    };
#[no_mangle]
unsafe extern "C" fn ad7266_init_channels(indio_dev: *mut iio_dev) {
    static void ad7266_init_channels(struct iio_dev *indio_dev)
    {
    struct ad7266_state *st = iio_priv(indio_dev);
    bool is_differential, is_signed;
    const struct ad7266_chan_info *chan_info;
    int i;
    is_differential = st.mode != AD7266_MODE_SINGLE_ENDED;
    is_signed = (st.range == AD7266_RANGE_2VREF) |
    (st.mode == AD7266_MODE_DIFF);
    i = AD7266_CHAN_INFO_INDEX(is_differential, is_signed, st.fixed_addr);
    chan_info = &ad7266_chan_infos[i];
    indio_dev.channels = chan_info.channels;
    indio_dev.num_channels = chan_info.num_channels;
    indio_dev.available_scan_masks = chan_info.scan_masks;
    }
    static const char * const ad7266_gpio_labels[] = {
    "ad0", "ad1", "ad2",
    };
#[no_mangle]
unsafe extern "C" fn ad7266_probe(spi: *mut spi_device) -> c_int {
    static int ad7266_probe(struct spi_device *spi)
    {
    const struct ad7266_platform_data *pdata = dev_get_platdata(&spi.dev);
    struct iio_dev *indio_dev;
    struct ad7266_state *st;
    unsigned int i;
    int ret;
    indio_dev = devm_iio_device_alloc(&spi.dev, sizeof(*st));
    if (indio_dev == core::ptr::null_mut())
    return -ENOMEM;
    st = iio_priv(indio_dev);
    ret = devm_regulator_get_enable_read_voltage(&spi.dev, "vref");
    if (ret < 0 && ret != -ENODEV)
    return ret;
    st.vref_mv = ret == -ENODEV ? AD7266_INTERNAL_REF_MV : ret / 1000;
    if (pdata) {
    st.fixed_addr = pdata.fixed_addr;
    st.mode = pdata.mode;
    st.range = pdata.range;
    if (!st.fixed_addr) {
    for (i = 0; i < ARRAY_SIZE(st.gpios); ++i) {
    st.gpios[i] = devm_gpiod_get(&spi.dev,
    ad7266_gpio_labels[i],
    GPIOD_OUT_LOW);
    if (IS_ERR(st.gpios[i]))
    return PTR_ERR(st.gpios[i]);
    }
    }
    } else {
    st.fixed_addr = true;
    st.range = AD7266_RANGE_VREF;
    st.mode = AD7266_MODE_DIFF;
    }
    st.spi = spi;
    indio_dev.name = spi_get_device_id(spi).name;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.info = &ad7266_info;
    ad7266_init_channels(indio_dev);
// wakeup
    st.single_xfer[0].rx_buf = &st.data.sample[0];
    st.single_xfer[0].len = 2;
    st.single_xfer[0].cs_change = 1;
// conversion
    st.single_xfer[1].rx_buf = st.data.sample;
    st.single_xfer[1].len = 4;
    st.single_xfer[1].cs_change = 1;
// powerdown
    st.single_xfer[2].tx_buf = &st.data.sample[0];
    st.single_xfer[2].len = 1;
    spi_message_init(&st.single_msg);
    spi_message_add_tail(&st.single_xfer[0], &st.single_msg);
    spi_message_add_tail(&st.single_xfer[1], &st.single_msg);
    spi_message_add_tail(&st.single_xfer[2], &st.single_msg);
    ret = devm_iio_triggered_buffer_setup(&spi.dev, indio_dev, &iio_pollfunc_store_time,
    &ad7266_trigger_handler, &iio_triggered_buffer_setup_ops);
    if (ret)
    return ret;
    return devm_iio_device_register(&spi.dev, indio_dev);
    }
    static const struct spi_device_id ad7266_id[] = {
    { .name = "ad7265" },
    { .name = "ad7266" },
    { }
    };
    MODULE_DEVICE_TABLE(spi, ad7266_id);
    static struct spi_driver ad7266_driver = {
    .driver = {
    .name	= "ad7266",
    },
    .probe		= ad7266_probe,
    .id_table	= ad7266_id,
    };
    module_spi_driver(ad7266_driver);
    MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
    MODULE_DESCRIPTION("Analog Devices AD7266/65 ADC");
    MODULE_LICENSE("GPL v2");
