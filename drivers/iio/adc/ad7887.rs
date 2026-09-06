//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/ad7887.c
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
// AD7887 SPI ADC driver
//
// Copyright 2010-2011 Analog Devices Inc.
//

    enum ad7887_channels {
    AD7887_CH0,
    AD7887_CH0_CH1,
    AD7887_CH1,
    };
//
// struct ad7887_chip_info - chip specific information
// @int_vref_mv:	the internal reference voltage
// @channels:		channels specification
// @num_channels:	number of channels
// @dual_channels:	channels specification in dual mode
// @num_dual_channels:	number of channels in dual mode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad7887_chip_info {
    pub int_vref_mv: u16,
    pub channels: *const iio_chan_spec,
    pub num_channels: c_uint,
    pub dual_channels: *const iio_chan_spec,
    pub num_dual_channels: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad7887_state {
    pub spi: *mut spi_device,
    pub chip_info: *const ad7887_chip_info,
    pub reg: *mut regulator,
    pub xfer: [spi_transfer; 4],
    pub msg: [spi_message; 3],
    pub ring_msg: *mut spi_message,
    pub tx_cmd_buf: [c_uchar; 4],
//
// DMA (thus cache coherency maintenance) may require the
// transfer buffers to live in their own cache lines.
// Buffer needs to be large enough to hold two 16 bit samples and a
// 64 bit aligned 64 bit timestamp.
//
    pub __aligned(IIO_DMA_MINALIGN): unsigned char data[ALIGN(4, sizeof(s64)) + sizeof(s64)],
}

    enum ad7887_supported_device_ids {
    ID_AD7887
    };
#[no_mangle]
unsafe extern "C" fn ad7887_ring_preenable(indio_dev: *mut iio_dev) -> c_int {
    static int ad7887_ring_preenable(struct iio_dev *indio_dev)
    {
    struct ad7887_state *st = iio_priv(indio_dev);
// We know this is a single long so can 'cheat'
    switch (*indio_dev.active_scan_mask) {
    case (1 << 0):
    st.ring_msg = &st.msg[AD7887_CH0];
    break;
    case (1 << 1):
    st.ring_msg = &st.msg[AD7887_CH1];
// Dummy read: push CH1 setting down to hardware
    spi_sync(st.spi, st.ring_msg);
    break;
    case ((1 << 1) | (1 << 0)):
    st.ring_msg = &st.msg[AD7887_CH0_CH1];
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ad7887_ring_postdisable(indio_dev: *mut iio_dev) -> c_int {
    static int ad7887_ring_postdisable(struct iio_dev *indio_dev)
    {
    struct ad7887_state *st = iio_priv(indio_dev);
// dummy read: restore default CH0 settings
    return spi_sync(st.spi, &st.msg[AD7887_CH0]);
    }
#[no_mangle]
unsafe extern "C" fn ad7887_trigger_handler(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t ad7887_trigger_handler(int irq, void *p)
    {
    struct iio_poll_func *pf = p;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct ad7887_state *st = iio_priv(indio_dev);
    int b_sent;
    b_sent = spi_sync(st.spi, st.ring_msg);
    if (b_sent)
    goto done;
    iio_push_to_buffers_with_timestamp(indio_dev, st.data,
    iio_get_time_ns(indio_dev));
    done:
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    }
    static const struct iio_buffer_setup_ops ad7887_ring_setup_ops = {
    .preenable = &ad7887_ring_preenable,
    .postdisable = &ad7887_ring_postdisable,
    };
#[no_mangle]
unsafe extern "C" fn ad7887_scan_direct(st: *mut ad7887_state, ch: unsigned) -> c_int {
    static int ad7887_scan_direct(struct ad7887_state *st, unsigned ch)
    {
    let mut ret: c_int = spi_sync(st.spi, &st.msg[ch]);
    if (ret)
    return ret;
    return (st.data[(ch * 2)] << 8) | st.data[(ch * 2) + 1];
    }
    static int ad7887_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val,
    int *val2,
    long m)
    {
    int ret;
    struct ad7887_state *st = iio_priv(indio_dev);
    switch (m) {
    case IIO_CHAN_INFO_RAW:
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    ret = ad7887_scan_direct(st, chan.address);
    iio_device_release_direct(indio_dev);
    if (ret < 0)
    return ret;
// val = ret >> chan->scan_type.shift;
// val &= GENMASK(chan->scan_type.realbits - 1, 0);
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
    if (st.reg) {
// val = regulator_get_voltage(st->reg);
    if (*val < 0)
    return *val;
// val /= 1000;
    } else {
// val = st->chip_info->int_vref_mv;
    }
// val2 = chan->scan_type.realbits;
    return IIO_VAL_FRACTIONAL_LOG2;
    }
    return -EINVAL;
    }

    .type = IIO_VOLTAGE, \
    .indexed = 1, \
    .channel = (x), \
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW), \
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE), \
    .address = (x), \
    .scan_index = (x), \
    .scan_type = { \
    .sign = 'u', \
    .realbits = 12, \
    .storagebits = 16, \
    .shift = 0, \
    .endianness = IIO_BE, \
    }, \
    }
    static const struct iio_chan_spec ad7887_channels[] = {
    AD7887_CHANNEL(0),
    IIO_CHAN_SOFT_TIMESTAMP(1),
    };
    static const struct iio_chan_spec ad7887_dual_channels[] = {
    AD7887_CHANNEL(0),
    AD7887_CHANNEL(1),
    IIO_CHAN_SOFT_TIMESTAMP(2),
    };
    static const struct ad7887_chip_info ad7887_chip_info_tbl[] = {
//
// More devices added in future
//
    [ID_AD7887] = {
    .channels = ad7887_channels,
    .num_channels = ARRAY_SIZE(ad7887_channels),
    .dual_channels = ad7887_dual_channels,
    .num_dual_channels = ARRAY_SIZE(ad7887_dual_channels),
    .int_vref_mv = 2500,
    },
    };
    static const struct iio_info ad7887_info = {
    .read_raw = &ad7887_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn ad7887_reg_disable(data: *mut c_void) {
    static void ad7887_reg_disable(void *data)
    {
    struct regulator *reg = data;
    regulator_disable(reg);
    }
#[no_mangle]
unsafe extern "C" fn ad7887_probe(spi: *mut spi_device) -> c_int {
    static int ad7887_probe(struct spi_device *spi)
    {
    const struct ad7887_platform_data *pdata = dev_get_platdata(&spi.dev);
    struct ad7887_state *st;
    struct iio_dev *indio_dev;
    uint8_t mode;
    int ret;
    indio_dev = devm_iio_device_alloc(&spi.dev, sizeof(*st));
    if (indio_dev == core::ptr::null_mut())
    return -ENOMEM;
    st = iio_priv(indio_dev);
    st.reg = devm_regulator_get_optional(&spi.dev, "vref");
    if (IS_ERR(st.reg)) {
    if (PTR_ERR(st.reg) != -ENODEV)
    return PTR_ERR(st.reg);
    st.reg = core::ptr::null_mut();
    }
    if (st.reg) {
    ret = regulator_enable(st.reg);
    if (ret)
    return ret;
    ret = devm_add_action_or_reset(&spi.dev, ad7887_reg_disable, st.reg);
    if (ret)
    return ret;
    }
    st.chip_info =
    &ad7887_chip_info_tbl[spi_get_device_id(spi).driver_data];
    st.spi = spi;
    indio_dev.name = spi_get_device_id(spi).name;
    indio_dev.info = &ad7887_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
// Setup default message
    mode = AD7887_PM_MODE4;
    if (!st.reg)
    mode |= AD7887_REF_DIS;
    if (pdata && pdata.en_dual)
    mode |= AD7887_DUAL;
    st.tx_cmd_buf[0] = AD7887_CH_AIN0 | mode;
    st.xfer[0].rx_buf = &st.data[0];
    st.xfer[0].tx_buf = &st.tx_cmd_buf[0];
    st.xfer[0].len = 2;
    spi_message_init(&st.msg[AD7887_CH0]);
    spi_message_add_tail(&st.xfer[0], &st.msg[AD7887_CH0]);
    if (pdata && pdata.en_dual) {
    st.tx_cmd_buf[2] = AD7887_CH_AIN1 | mode;
    st.xfer[1].rx_buf = &st.data[0];
    st.xfer[1].tx_buf = &st.tx_cmd_buf[2];
    st.xfer[1].len = 2;
    st.xfer[2].rx_buf = &st.data[2];
    st.xfer[2].tx_buf = &st.tx_cmd_buf[0];
    st.xfer[2].len = 2;
    spi_message_init(&st.msg[AD7887_CH0_CH1]);
    spi_message_add_tail(&st.xfer[1], &st.msg[AD7887_CH0_CH1]);
    spi_message_add_tail(&st.xfer[2], &st.msg[AD7887_CH0_CH1]);
    st.xfer[3].rx_buf = &st.data[2];
    st.xfer[3].tx_buf = &st.tx_cmd_buf[2];
    st.xfer[3].len = 2;
    spi_message_init(&st.msg[AD7887_CH1]);
    spi_message_add_tail(&st.xfer[3], &st.msg[AD7887_CH1]);
    indio_dev.channels = st.chip_info.dual_channels;
    indio_dev.num_channels = st.chip_info.num_dual_channels;
    } else {
    indio_dev.channels = st.chip_info.channels;
    indio_dev.num_channels = st.chip_info.num_channels;
    }
    ret = devm_iio_triggered_buffer_setup(&spi.dev, indio_dev,
    &iio_pollfunc_store_time,
    &ad7887_trigger_handler, &ad7887_ring_setup_ops);
    if (ret)
    return ret;
    return devm_iio_device_register(&spi.dev, indio_dev);
    }
    static const struct spi_device_id ad7887_id[] = {
    { .name = "ad7887", .driver_data = ID_AD7887 },
    { }
    };
    MODULE_DEVICE_TABLE(spi, ad7887_id);
    static struct spi_driver ad7887_driver = {
    .driver = {
    .name	= "ad7887",
    },
    .probe		= ad7887_probe,
    .id_table	= ad7887_id,
    };
    module_spi_driver(ad7887_driver);
    MODULE_AUTHOR("Michael Hennerich <michael.hennerich@analog.com>");
    MODULE_DESCRIPTION("Analog Devices AD7887 ADC");
    MODULE_LICENSE("GPL v2");
