//! Automatically rewritten from C to Rust
//! Source: drivers/iio/dac/ad5761.c
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
// AD5721, AD5721R, AD5761, AD5761R, Voltage Output Digital to Analog Converter
//
// Copyright 2016 Qtechnology A/S
// 2016 Ricardo Ribalda <ribalda@kernel.org>
//

pub const AD5761_ADDR_NOOP: c_uint = 0x0;
pub const AD5761_ADDR_DAC_WRITE: c_uint = 0x3;
pub const AD5761_ADDR_CTRL_WRITE_REG: c_uint = 0x4;
pub const AD5761_ADDR_SW_DATA_RESET: c_uint = 0x7;
pub const AD5761_ADDR_DAC_READ: c_uint = 0xb;
pub const AD5761_ADDR_CTRL_READ_REG: c_uint = 0xc;
pub const AD5761_ADDR_SW_FULL_RESET: c_uint = 0xf;

//
// struct ad5761_chip_info - chip specific information
// @int_vref:	Value of the internal reference voltage in mV - 0 if external
// reference voltage is used
// @channel:	channel specification
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad5761_chip_info {
    pub int_vref: c_ulong,
    pub channel: iio_chan_spec,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad5761_range_params {
    pub m: c_int,
    pub c: c_int,
}

    enum ad5761_supported_device_ids {
    ID_AD5721,
    ID_AD5721R,
    ID_AD5761,
    ID_AD5761R,
    };
//
// struct ad5761_state - driver instance specific data
// @spi:		spi_device
// @use_intref:		true when the internal voltage reference is used
// @vref:		actual voltage reference in mVolts
// @range:		output range mode used
// @lock:		lock to protect the data buffer during SPI ops
// @data:		cache aligned spi buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad5761_state {
    pub spi: *mut spi_device,
    pub lock: mutex,
    pub use_intref: bool,
    pub vref: c_int,
    pub range: enum ad5761_voltage_range,
//
// DMA (thus cache coherency maintenance) may require the
// transfer buffers to live in their own cache lines.
//
    union {
    pub d32: __be32,
    pub d8: [u8; 4],
    pub __aligned(IIO_DMA_MINALIGN): } data[3],
}

    static const struct ad5761_range_params ad5761_range_params[] = {
    [AD5761_VOLTAGE_RANGE_M10V_10V] = {
    .m = 80,
    .c = 40,
    },
    [AD5761_VOLTAGE_RANGE_0V_10V] = {
    .m = 40,
    .c = 0,
    },
    [AD5761_VOLTAGE_RANGE_M5V_5V] = {
    .m = 40,
    .c = 20,
    },
    [AD5761_VOLTAGE_RANGE_0V_5V] = {
    .m = 20,
    .c = 0,
    },
    [AD5761_VOLTAGE_RANGE_M2V5_7V5] = {
    .m = 40,
    .c = 10,
    },
    [AD5761_VOLTAGE_RANGE_M3V_3V] = {
    .m = 24,
    .c = 12,
    },
    [AD5761_VOLTAGE_RANGE_0V_16V] = {
    .m = 64,
    .c = 0,
    },
    [AD5761_VOLTAGE_RANGE_0V_20V] = {
    .m = 80,
    .c = 0,
    },
    };
#[no_mangle]
unsafe extern "C" fn _ad5761_spi_write(st: *mut ad5761_state, addr: u8, val: u16) -> c_int {
    static int _ad5761_spi_write(struct ad5761_state *st, u8 addr, u16 val)
    {
    st.data[0].d32 = cpu_to_be32(AD5761_ADDR(addr) | val);
    return spi_write(st.spi, &st.data[0].d8[1], 3);
    }
#[no_mangle]
unsafe extern "C" fn ad5761_spi_write(indio_dev: *mut iio_dev, addr: u8, val: u16) -> c_int {
    static int ad5761_spi_write(struct iio_dev *indio_dev, u8 addr, u16 val)
    {
    struct ad5761_state *st = iio_priv(indio_dev);
    int ret;
    mutex_lock(&st.lock);
    ret = _ad5761_spi_write(st, addr, val);
    mutex_unlock(&st.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn _ad5761_spi_read(st: *mut ad5761_state, addr: u8, val: *mut u16) -> c_int {
    static int _ad5761_spi_read(struct ad5761_state *st, u8 addr, u16 *val)
    {
    int ret;
    struct spi_transfer xfers[] = {
    {
    .tx_buf = &st.data[0].d8[1],
    .len = 3,
    .cs_change = true,
    }, {
    .tx_buf = &st.data[1].d8[1],
    .rx_buf = &st.data[2].d8[1],
    .len = 3,
    },
    };
    st.data[0].d32 = cpu_to_be32(AD5761_ADDR(addr));
    st.data[1].d32 = cpu_to_be32(AD5761_ADDR(AD5761_ADDR_NOOP));
    ret = spi_sync_transfer(st.spi, xfers, ARRAY_SIZE(xfers));
// val = be32_to_cpu(st->data[2].d32);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ad5761_spi_read(indio_dev: *mut iio_dev, addr: u8, val: *mut u16) -> c_int {
    static int ad5761_spi_read(struct iio_dev *indio_dev, u8 addr, u16 *val)
    {
    struct ad5761_state *st = iio_priv(indio_dev);
    int ret;
    mutex_lock(&st.lock);
    ret = _ad5761_spi_read(st, addr, val);
    mutex_unlock(&st.lock);
    return ret;
    }
    static int ad5761_spi_set_range(struct ad5761_state *st,
    enum ad5761_voltage_range range)
    {
    u16 aux;
    int ret;
    aux = (range & 0x7) | AD5761_CTRL_ETS;
    if (st.use_intref)
    aux |= AD5761_CTRL_USE_INTVREF;
    ret = _ad5761_spi_write(st, AD5761_ADDR_SW_FULL_RESET, 0);
    if (ret)
    return ret;
    ret = _ad5761_spi_write(st, AD5761_ADDR_CTRL_WRITE_REG, aux);
    if (ret)
    return ret;
    st.range = range;
    return 0;
    }
    static int ad5761_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val,
    int *val2,
    long mask)
    {
    struct ad5761_state *st;
    int ret;
    u16 aux;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    ret = ad5761_spi_read(indio_dev, AD5761_ADDR_DAC_READ, &aux);
    if (ret)
    return ret;
// val = aux >> chan->scan_type.shift;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
    st = iio_priv(indio_dev);
// val = st->vref * ad5761_range_params[st->range].m;
// val /= 10;
// val2 = chan->scan_type.realbits;
    return IIO_VAL_FRACTIONAL_LOG2;
    case IIO_CHAN_INFO_OFFSET:
    st = iio_priv(indio_dev);
// val = -(1 << chan->scan_type.realbits);
// val *=	ad5761_range_params[st->range].c;
// val /=	ad5761_range_params[st->range].m;
    return IIO_VAL_INT;
    default:
    return -EINVAL;
    }
    }
    static int ad5761_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val,
    int val2,
    long mask)
    {
    u16 aux;
    if (mask != IIO_CHAN_INFO_RAW)
    return -EINVAL;
    if (val2 || (val << chan.scan_type.shift) > 0xffff || val < 0)
    return -EINVAL;
    aux = val << chan.scan_type.shift;
    return ad5761_spi_write(indio_dev, AD5761_ADDR_DAC_WRITE, aux);
    }
    static const struct iio_info ad5761_info = {
    .read_raw = &ad5761_read_raw,
    .write_raw = &ad5761_write_raw,
    };

    .type = IIO_VOLTAGE,				\
    .output = 1,					\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),	\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE) |	\
    BIT(IIO_CHAN_INFO_OFFSET),		\
    .scan_type = {					\
    .sign = 'u',				\
    .realbits = (_bits),			\
    .storagebits = 16,			\
    .shift = 16 - (_bits),			\
    },						\
    }
    static const struct ad5761_chip_info ad5761_chip_infos[] = {
    [ID_AD5721] = {
    .int_vref = 0,
    .channel = AD5761_CHAN(12),
    },
    [ID_AD5721R] = {
    .int_vref = 2500,
    .channel = AD5761_CHAN(12),
    },
    [ID_AD5761] = {
    .int_vref = 0,
    .channel = AD5761_CHAN(16),
    },
    [ID_AD5761R] = {
    .int_vref = 2500,
    .channel = AD5761_CHAN(16),
    },
    };
#[no_mangle]
unsafe extern "C" fn ad5761_probe(spi: *mut spi_device) -> c_int {
    static int ad5761_probe(struct spi_device *spi)
    {
    struct iio_dev *iio_dev;
    struct ad5761_state *st;
    int ret;
    const struct ad5761_chip_info *chip_info =
    &ad5761_chip_infos[spi_get_device_id(spi).driver_data];
    let mut voltage_range: enum ad5761_voltage_range = AD5761_VOLTAGE_RANGE_0V_5V;
    struct ad5761_platform_data *pdata = dev_get_platdata(&spi.dev);
    iio_dev = devm_iio_device_alloc(&spi.dev, sizeof(*st));
    if (!iio_dev)
    return -ENOMEM;
    st = iio_priv(iio_dev);
    st.spi = spi;
    ret = devm_regulator_get_enable_read_voltage(&spi.dev, "vref");
    if (ret < 0 && ret != -ENODEV)
    return dev_err_probe(&spi.dev, ret,
    "Failed to get voltage reference value\n");
    if (ret == -ENODEV) {
// Use Internal regulator
    if (!chip_info.int_vref)
    return dev_err_probe(&spi.dev, -EIO,
    "Voltage reference not found\n");
    st.use_intref = true;
    st.vref = chip_info.int_vref;
    } else {
    if (ret < 2000000 || ret > 3000000)
    return dev_err_probe(&spi.dev, -EIO,
    "Invalid external voltage ref. value %d uV\n",
    ret);
    st.use_intref = false;
    st.vref = ret / 1000;
    }
    if (pdata)
    voltage_range = pdata.voltage_range;
    mutex_init(&st.lock);
    ret = ad5761_spi_set_range(st, voltage_range);
    if (ret)
    return ret;
    iio_dev.info = &ad5761_info;
    iio_dev.modes = INDIO_DIRECT_MODE;
    iio_dev.channels = &chip_info.channel;
    iio_dev.num_channels = 1;
    iio_dev.name = spi_get_device_id(st.spi).name;
    return devm_iio_device_register(&spi.dev, iio_dev);
    }
    static const struct spi_device_id ad5761_id[] = {
    { .name = "ad5721", .driver_data = ID_AD5721 },
    { .name = "ad5721r", .driver_data = ID_AD5721R },
    { .name = "ad5761", .driver_data = ID_AD5761 },
    { .name = "ad5761r", .driver_data = ID_AD5761R },
    { }
    };
    MODULE_DEVICE_TABLE(spi, ad5761_id);
    static struct spi_driver ad5761_driver = {
    .driver = {
    .name = "ad5761",
    },
    .probe = ad5761_probe,
    .id_table = ad5761_id,
    };
    module_spi_driver(ad5761_driver);
    MODULE_AUTHOR("Ricardo Ribalda <ribalda@kernel.org>");
    MODULE_DESCRIPTION("Analog Devices AD5721, AD5721R, AD5761, AD5761R driver");
    MODULE_LICENSE("GPL v2");
