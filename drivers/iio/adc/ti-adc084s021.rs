//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/ti-adc084s021.c
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
// Copyright (C) 2017 Axis Communications AB
//
// Driver for Texas Instruments' ADC084S021 ADC chip.
// Datasheets can be found here:
// https://www.ti.com/lit/ds/symlink/adc084s021.pdf
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adc084s021 {
    pub spi: *mut spi_device,
    pub message: spi_message,
    pub spi_trans: spi_transfer,
    pub reg: *mut regulator,
    pub lock: mutex,
// Buffer used to align data
    struct {
    pub channels: [__be16; 4],
    pub ts: aligned_s64,
    pub scan: },
//
// DMA (thus cache coherency maintenance) may require the
// transfer buffers to live in their own cache line.
//
    pub __aligned(IIO_DMA_MINALIGN): u16 tx_buf[4],
    pub /: *mut *mut __be16 rx_buf[5]; / First 16-bits are trash,
}

    {                                                      \
    .type = IIO_VOLTAGE,                                 \
    .channel = (num),                                    \
    .indexed = 1,                                        \
    .scan_index = (num),                                 \
    .scan_type = {                                       \
    .sign = 'u',                                       \
    .realbits = 8,                                     \
    .storagebits = 16,                                 \
    .shift = 4,                                        \
    .endianness = IIO_BE,                              \
    },                                                   \
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),        \
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE),\
    }
    static const struct iio_chan_spec adc084s021_channels[] = {
    ADC084S021_VOLTAGE_CHANNEL(0),
    ADC084S021_VOLTAGE_CHANNEL(1),
    ADC084S021_VOLTAGE_CHANNEL(2),
    ADC084S021_VOLTAGE_CHANNEL(3),
    IIO_CHAN_SOFT_TIMESTAMP(4),
    };
//
// adc084s021_adc_conversion() - Read an ADC channel and return its value.
//
// @adc: The ADC SPI data.
// @data: Buffer for converted data.
//
#[no_mangle]
unsafe extern "C" fn adc084s021_adc_conversion(adc: *mut adc084s021, data: *mut __be16) -> c_int {
    static int adc084s021_adc_conversion(struct adc084s021 *adc, __be16 *data)
    {
    int n_words = (adc.spi_trans.len >> 1) - 1; /* Discard first word */
    int ret, i = 0;
// Do the transfer
    ret = spi_sync(adc.spi, &adc.message);
    if (ret < 0)
    return ret;
    for (; i < n_words; i++)
// (data + i) = adc->rx_buf[i + 1];
    return ret;
    }
    static int adc084s021_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *channel, int *val,
    int *val2, long mask)
    {
    struct adc084s021 *adc = iio_priv(indio_dev);
    int ret;
    __be16 be_val;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    ret = regulator_enable(adc.reg);
    if (ret) {
    iio_device_release_direct(indio_dev);
    return ret;
    }
    adc.tx_buf[0] = channel.channel << 3;
    ret = adc084s021_adc_conversion(adc, &be_val);
    iio_device_release_direct(indio_dev);
    regulator_disable(adc.reg);
    if (ret < 0)
    return ret;
// val = be16_to_cpu(be_val);
// val = (*val >> channel->scan_type.shift) & 0xff;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
    ret = regulator_enable(adc.reg);
    if (ret)
    return ret;
    ret = regulator_get_voltage(adc.reg);
    regulator_disable(adc.reg);
    if (ret < 0)
    return ret;
// val = ret / 1000;
    return IIO_VAL_INT;
    default:
    return -EINVAL;
    }
    }
//
// adc084s021_buffer_trigger_handler() - Read ADC channels and push to buffer.
//
// @irq: The interrupt number (not used).
// @pollfunc: Pointer to the poll func.
//
#[no_mangle]
unsafe extern "C" fn adc084s021_buffer_trigger_handler(irq: c_int, pollfunc: *mut c_void) -> irqreturn_t {
    static irqreturn_t adc084s021_buffer_trigger_handler(int irq, void *pollfunc)
    {
    struct iio_poll_func *pf = pollfunc;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct adc084s021 *adc = iio_priv(indio_dev);
    mutex_lock(&adc.lock);
    if (adc084s021_adc_conversion(adc, adc.scan.channels) < 0)
    dev_err(&adc.spi.dev, "Failed to read data\n");
    iio_push_to_buffers_with_ts(indio_dev, &adc.scan, sizeof(adc.scan),
    iio_get_time_ns(indio_dev));
    mutex_unlock(&adc.lock);
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn adc084s021_buffer_preenable(indio_dev: *mut iio_dev) -> c_int {
    static int adc084s021_buffer_preenable(struct iio_dev *indio_dev)
    {
    struct adc084s021 *adc = iio_priv(indio_dev);
    int scan_index;
    let mut i: c_int = 0;
    iio_for_each_active_channel(indio_dev, scan_index) {
    const struct iio_chan_spec *channel =
    &indio_dev.channels[scan_index];
    adc.tx_buf[i++] = channel.channel << 3;
    }
    adc.spi_trans.len = 2 + (i * sizeof(__be16)); /* Trash + channels */
    return regulator_enable(adc.reg);
    }
#[no_mangle]
unsafe extern "C" fn adc084s021_buffer_postdisable(indio_dev: *mut iio_dev) -> c_int {
    static int adc084s021_buffer_postdisable(struct iio_dev *indio_dev)
    {
    struct adc084s021 *adc = iio_priv(indio_dev);
    adc.spi_trans.len = 4; /* Trash + single channel */
    return regulator_disable(adc.reg);
    }
    static const struct iio_info adc084s021_info = {
    .read_raw = adc084s021_read_raw,
    };
    static const struct iio_buffer_setup_ops adc084s021_buffer_setup_ops = {
    .preenable = adc084s021_buffer_preenable,
    .postdisable = adc084s021_buffer_postdisable,
    };
#[no_mangle]
unsafe extern "C" fn adc084s021_probe(spi: *mut spi_device) -> c_int {
    static int adc084s021_probe(struct spi_device *spi)
    {
    struct iio_dev *indio_dev;
    struct adc084s021 *adc;
    int ret;
    indio_dev = devm_iio_device_alloc(&spi.dev, sizeof(*adc));
    if (!indio_dev)
    return -ENOMEM;
    adc = iio_priv(indio_dev);
    adc.spi = spi;
// Initiate the Industrial I/O device
    indio_dev.name = spi_get_device_id(spi).name;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.info = &adc084s021_info;
    indio_dev.channels = adc084s021_channels;
    indio_dev.num_channels = ARRAY_SIZE(adc084s021_channels);
// Create SPI transfer for channel reads
    adc.spi_trans.tx_buf = adc.tx_buf;
    adc.spi_trans.rx_buf = adc.rx_buf;
    adc.spi_trans.len = 4; /* Trash + single channel */
    spi_message_init_with_transfers(&adc.message, &adc.spi_trans, 1);
    adc.reg = devm_regulator_get(&spi.dev, "vref");
    if (IS_ERR(adc.reg))
    return PTR_ERR(adc.reg);
    mutex_init(&adc.lock);
// Setup triggered buffer with pollfunction
    ret = devm_iio_triggered_buffer_setup(&spi.dev, indio_dev, core::ptr::null_mut(),
    adc084s021_buffer_trigger_handler,
    &adc084s021_buffer_setup_ops);
    if (ret)
    return dev_err_probe(&spi.dev, ret, "Failed to setup triggered buffer\n");
    return devm_iio_device_register(&spi.dev, indio_dev);
    }
    static const struct of_device_id adc084s021_of_match[] = {
    { .compatible = "ti,adc084s021", },
    { }
    };
    MODULE_DEVICE_TABLE(of, adc084s021_of_match);
    static const struct spi_device_id adc084s021_id[] = {
    { .name = ADC084S021_DRIVER_NAME },
    { }
    };
    MODULE_DEVICE_TABLE(spi, adc084s021_id);
    static struct spi_driver adc084s021_driver = {
    .driver = {
    .name = ADC084S021_DRIVER_NAME,
    .of_match_table = adc084s021_of_match,
    },
    .probe = adc084s021_probe,
    .id_table = adc084s021_id,
    };
    module_spi_driver(adc084s021_driver);
    MODULE_AUTHOR("Mårten Lindahl <martenli@axis.com>");
    MODULE_DESCRIPTION("Texas Instruments ADC084S021");
    MODULE_LICENSE("GPL v2");
    MODULE_VERSION("1.0");
