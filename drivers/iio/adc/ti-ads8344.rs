//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/ti-ads8344.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// ADS8344 16-bit 8-Channel ADC driver
//
// Author: Gregory CLEMENT <gregory.clement@bootlin.com>
//
// Datasheet: https://www.ti.com/lit/ds/symlink/ads8344.pdf
//

pub const ADS8344_CLOCK_INTERNAL: c_uint = 0x2 /* PD1 = 1 and PD0 = 0 */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ads8344 {
    pub spi: *mut spi_device,
    pub reg: *mut regulator,
//
// Lock protecting access to adc->tx_buff and rx_buff,
// especially from concurrent read on sysfs file.
//
    pub lock: mutex,
    pub __aligned(IIO_DMA_MINALIGN): u8 tx_buf,
    pub rx_buf: [u8; 3],
}

    {								\
    .type = IIO_VOLTAGE,					\
    .indexed = 1,						\
    .channel = chan,					\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),		\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE),	\
    .address = addr,					\
    }

    {								\
    .type = IIO_VOLTAGE,					\
    .indexed = 1,						\
    .channel = (chan1),					\
    .channel2 = (chan2),					\
    .differential = 1,					\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),		\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE),	\
    .address = addr,					\
    }
    static const struct iio_chan_spec ads8344_channels[] = {
    ADS8344_VOLTAGE_CHANNEL(0, 0),
    ADS8344_VOLTAGE_CHANNEL(1, 4),
    ADS8344_VOLTAGE_CHANNEL(2, 1),
    ADS8344_VOLTAGE_CHANNEL(3, 5),
    ADS8344_VOLTAGE_CHANNEL(4, 2),
    ADS8344_VOLTAGE_CHANNEL(5, 6),
    ADS8344_VOLTAGE_CHANNEL(6, 3),
    ADS8344_VOLTAGE_CHANNEL(7, 7),
    ADS8344_VOLTAGE_CHANNEL_DIFF(0, 1, 8),
    ADS8344_VOLTAGE_CHANNEL_DIFF(2, 3, 9),
    ADS8344_VOLTAGE_CHANNEL_DIFF(4, 5, 10),
    ADS8344_VOLTAGE_CHANNEL_DIFF(6, 7, 11),
    ADS8344_VOLTAGE_CHANNEL_DIFF(1, 0, 12),
    ADS8344_VOLTAGE_CHANNEL_DIFF(3, 2, 13),
    ADS8344_VOLTAGE_CHANNEL_DIFF(5, 4, 14),
    ADS8344_VOLTAGE_CHANNEL_DIFF(7, 6, 15),
    };
    static int ads8344_adc_conversion(struct ads8344 *adc, int channel,
    bool differential)
    {
    struct spi_device *spi = adc.spi;
    int ret;
    adc.tx_buf = ADS8344_START;
    if (!differential)
    adc.tx_buf |= ADS8344_SINGLE_END;
    adc.tx_buf |= ADS8344_CHANNEL(channel);
    adc.tx_buf |= ADS8344_CLOCK_INTERNAL;
    ret = spi_write(spi, &adc.tx_buf, 1);
    if (ret)
    return ret;
    udelay(9);
    ret = spi_read(spi, adc.rx_buf, sizeof(adc.rx_buf));
    if (ret)
    return ret;
    return adc.rx_buf[0] << 9 | adc.rx_buf[1] << 1 | adc.rx_buf[2] >> 7;
    }
    static int ads8344_read_raw(struct iio_dev *iio,
    struct iio_chan_spec const *channel, int *value,
    int *shift, long mask)
    {
    struct ads8344 *adc = iio_priv(iio);
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    mutex_lock(&adc.lock);
// value = ads8344_adc_conversion(adc, channel->address,
    channel.differential);
    mutex_unlock(&adc.lock);
    if (*value < 0)
    return *value;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
// value = regulator_get_voltage(adc->reg);
    if (*value < 0)
    return *value;
// convert regulator output voltage to mV
// value /= 1000;
// shift = 16;
    return IIO_VAL_FRACTIONAL_LOG2;
    default:
    return -EINVAL;
    }
    }
    static const struct iio_info ads8344_info = {
    .read_raw = ads8344_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn ads8344_reg_disable(data: *mut c_void) {
    static void ads8344_reg_disable(void *data)
    {
    regulator_disable(data);
    }
#[no_mangle]
unsafe extern "C" fn ads8344_probe(spi: *mut spi_device) -> c_int {
    static int ads8344_probe(struct spi_device *spi)
    {
    struct iio_dev *indio_dev;
    struct ads8344 *adc;
    int ret;
    indio_dev = devm_iio_device_alloc(&spi.dev, sizeof(*adc));
    if (!indio_dev)
    return -ENOMEM;
    adc = iio_priv(indio_dev);
    adc.spi = spi;
    mutex_init(&adc.lock);
    indio_dev.name = dev_name(&spi.dev);
    indio_dev.info = &ads8344_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = ads8344_channels;
    indio_dev.num_channels = ARRAY_SIZE(ads8344_channels);
    adc.reg = devm_regulator_get(&spi.dev, "vref");
    if (IS_ERR(adc.reg))
    return PTR_ERR(adc.reg);
    ret = regulator_enable(adc.reg);
    if (ret)
    return ret;
    ret = devm_add_action_or_reset(&spi.dev, ads8344_reg_disable, adc.reg);
    if (ret)
    return ret;
    return devm_iio_device_register(&spi.dev, indio_dev);
    }
    static const struct of_device_id ads8344_of_match[] = {
    { .compatible = "ti,ads8344", },
    { }
    };
    MODULE_DEVICE_TABLE(of, ads8344_of_match);
    static struct spi_driver ads8344_driver = {
    .driver = {
    .name = "ads8344",
    .of_match_table = ads8344_of_match,
    },
    .probe = ads8344_probe,
    };
    module_spi_driver(ads8344_driver);
    MODULE_AUTHOR("Gregory CLEMENT <gregory.clement@bootlin.com>");
    MODULE_DESCRIPTION("ADS8344 driver");
    MODULE_LICENSE("GPL");
