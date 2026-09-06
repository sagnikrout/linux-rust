//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/ti-tlc4541.c
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
// TI tlc4541 ADC Driver
//
// Copyright (C) 2017 Phil Reid
//
// Datasheets can be found here:
// https://www.ti.com/lit/gpn/tlc3541
// https://www.ti.com/lit/gpn/tlc4541
//
// The tlc4541 requires 24 clock cycles to start a transfer.
// Conversion then takes 2.94us to complete before data is ready
// Data is returned MSB first.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlc4541_state {
    pub spi: *mut spi_device,
    pub reg: *mut regulator,
    pub scan_single_xfer: [spi_transfer; 3],
    pub scan_single_msg: spi_message,
//
// DMA (thus cache coherency maintenance) may require the
// transfer buffers to live in their own cache lines.
// 2 bytes data + 6 bytes padding + 8 bytes timestamp when
// call iio_push_to_buffers_with_timestamp.
//
    pub __aligned(IIO_DMA_MINALIGN): __be16 rx_buf[8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlc4541_chip_info {
    pub channels: *const iio_chan_spec,
    pub num_channels: c_uint,
}

    enum tlc4541_id {
    TLC3541,
    TLC4541,
    };

    .type = IIO_VOLTAGE,                                  \
    .info_mask_separate       = BIT(IIO_CHAN_INFO_RAW),   \
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE), \
    .scan_type = {                                        \
    .sign = 'u',                                  \
    .realbits = (bits),                           \
    .storagebits = 16,                            \
    .shift = (bitshift),                          \
    .endianness = IIO_BE,                         \
    },                                                    \
    }

    const struct iio_chan_spec name ## _channels[] = { \
    TLC4541_V_CHAN(bits, bitshift), \
    IIO_CHAN_SOFT_TIMESTAMP(1), \
    }
    static DECLARE_TLC4541_CHANNELS(tlc3541, 14, 2);
    static DECLARE_TLC4541_CHANNELS(tlc4541, 16, 0);
    static const struct tlc4541_chip_info tlc4541_chip_info[] = {
    [TLC3541] = {
    .channels = tlc3541_channels,
    .num_channels = ARRAY_SIZE(tlc3541_channels),
    },
    [TLC4541] = {
    .channels = tlc4541_channels,
    .num_channels = ARRAY_SIZE(tlc4541_channels),
    },
    };
#[no_mangle]
unsafe extern "C" fn tlc4541_trigger_handler(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t tlc4541_trigger_handler(int irq, void *p)
    {
    struct iio_poll_func *pf = p;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct tlc4541_state *st = iio_priv(indio_dev);
    int ret;
    ret = spi_sync(st.spi, &st.scan_single_msg);
    if (ret < 0)
    goto done;
    iio_push_to_buffers_with_ts(indio_dev, st.rx_buf, sizeof(st.rx_buf),
    iio_get_time_ns(indio_dev));
    done:
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn tlc4541_get_range(st: *mut tlc4541_state) -> c_int {
    static int tlc4541_get_range(struct tlc4541_state *st)
    {
    int vref;
    vref = regulator_get_voltage(st.reg);
    if (vref < 0)
    return vref;
    vref /= 1000;
    return vref;
    }
    static int tlc4541_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val,
    int *val2,
    long m)
    {
    let mut ret: c_int = 0;
    struct tlc4541_state *st = iio_priv(indio_dev);
    switch (m) {
    case IIO_CHAN_INFO_RAW:
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    ret = spi_sync(st.spi, &st.scan_single_msg);
    iio_device_release_direct(indio_dev);
    if (ret < 0)
    return ret;
// val = be16_to_cpu(st->rx_buf[0]);
// val = *val >> chan->scan_type.shift;
// val &= GENMASK(chan->scan_type.realbits - 1, 0);
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
    ret = tlc4541_get_range(st);
    if (ret < 0)
    return ret;
// val = ret;
// val2 = chan->scan_type.realbits;
    return IIO_VAL_FRACTIONAL_LOG2;
    }
    return -EINVAL;
    }
    static const struct iio_info tlc4541_info = {
    .read_raw = &tlc4541_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn tlc4541_probe(spi: *mut spi_device) -> c_int {
    static int tlc4541_probe(struct spi_device *spi)
    {
    struct tlc4541_state *st;
    struct iio_dev *indio_dev;
    const struct tlc4541_chip_info *info;
    int ret;
    let mut device_init: i8 = 0;
    indio_dev = devm_iio_device_alloc(&spi.dev, sizeof(*st));
    if (indio_dev == core::ptr::null_mut())
    return -ENOMEM;
    st = iio_priv(indio_dev);
    spi_set_drvdata(spi, indio_dev);
    st.spi = spi;
    info = &tlc4541_chip_info[spi_get_device_id(spi).driver_data];
    indio_dev.name = spi_get_device_id(spi).name;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = info.channels;
    indio_dev.num_channels = info.num_channels;
    indio_dev.info = &tlc4541_info;
// perform reset
    spi_write(spi, &device_init, 1);
// Setup default message
    st.scan_single_xfer[0].rx_buf = &st.rx_buf[0];
    st.scan_single_xfer[0].len = 3;
    st.scan_single_xfer[1].delay.value = 3;
    st.scan_single_xfer[1].delay.unit = SPI_DELAY_UNIT_NSECS;
    st.scan_single_xfer[2].rx_buf = &st.rx_buf[0];
    st.scan_single_xfer[2].len = 2;
    spi_message_init_with_transfers(&st.scan_single_msg,
    st.scan_single_xfer, 3);
    st.reg = devm_regulator_get(&spi.dev, "vref");
    if (IS_ERR(st.reg))
    return PTR_ERR(st.reg);
    ret = regulator_enable(st.reg);
    if (ret)
    return ret;
    ret = iio_triggered_buffer_setup(indio_dev, core::ptr::null_mut(),
    &tlc4541_trigger_handler, core::ptr::null_mut());
    if (ret)
    goto error_disable_reg;
    ret = iio_device_register(indio_dev);
    if (ret)
    goto error_cleanup_buffer;
    return 0;
    error_cleanup_buffer:
    iio_triggered_buffer_cleanup(indio_dev);
    error_disable_reg:
    regulator_disable(st.reg);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tlc4541_remove(spi: *mut spi_device) {
    static void tlc4541_remove(struct spi_device *spi)
    {
    struct iio_dev *indio_dev = spi_get_drvdata(spi);
    struct tlc4541_state *st = iio_priv(indio_dev);
    iio_device_unregister(indio_dev);
    iio_triggered_buffer_cleanup(indio_dev);
    regulator_disable(st.reg);
    }
    static const struct of_device_id tlc4541_dt_ids[] = {
    { .compatible = "ti,tlc3541", },
    { .compatible = "ti,tlc4541", },
    { }
    };
    MODULE_DEVICE_TABLE(of, tlc4541_dt_ids);
    static const struct spi_device_id tlc4541_id[] = {
    { .name = "tlc3541", .driver_data = TLC3541 },
    { .name = "tlc4541", .driver_data = TLC4541 },
    { }
    };
    MODULE_DEVICE_TABLE(spi, tlc4541_id);
    static struct spi_driver tlc4541_driver = {
    .driver = {
    .name   = "tlc4541",
    .of_match_table = tlc4541_dt_ids,
    },
    .probe          = tlc4541_probe,
    .remove         = tlc4541_remove,
    .id_table       = tlc4541_id,
    };
    module_spi_driver(tlc4541_driver);
    MODULE_AUTHOR("Phil Reid <preid@electromag.com.au>");
    MODULE_DESCRIPTION("Texas Instruments TLC4541 ADC");
    MODULE_LICENSE("GPL v2");
