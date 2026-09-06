//! Automatically rewritten from C to Rust
//! Source: drivers/iio/dac/cio-dac.c
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
// IIO driver for the Measurement Computing CIO-DAC
// Copyright (C) 2016 William Breathitt Gray
//
// This driver supports the following Measurement Computing devices: CIO-DAC16,
// CIO-DAC08, and PC104-DAC06.
//

pub const CIO_DAC_NUM_CHAN: c_int = 16;

    .type = IIO_VOLTAGE,				\
    .channel = chan,				\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),	\
    .indexed = 1,					\
    .output = 1					\
    }
pub const CIO_DAC_EXTENT: c_int = 32;
    static unsigned int base[max_num_isa_dev(CIO_DAC_EXTENT)];
    static unsigned int num_cio_dac;
    module_param_hw_array(base, uint, ioport, &num_cio_dac, 0);
    MODULE_PARM_DESC(base, "Measurement Computing CIO-DAC base addresses");
pub const CIO_DAC_BASE: c_uint = 0x00;
pub const CIO_DAC_CHANNEL_STRIDE: c_int = 2;
#[no_mangle]
unsafe extern "C" fn cio_dac_precious_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool cio_dac_precious_reg(struct device *dev, unsigned int reg)
    {
//
// All registers are considered precious; if the XFER jumper is set on
// the device, then no update occurs until a DAC register is read.
//
    return true;
    }
    static const struct regmap_config cio_dac_regmap_config = {
    .reg_bits = 16,
    .reg_stride = 2,
    .val_bits = 16,
    .io_port = true,
    .max_register = 0x1F,
    .precious_reg = cio_dac_precious_reg,
    };
//
// struct cio_dac_iio - IIO device private data structure
// @map: Regmap for the device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cio_dac_iio {
    pub map: *mut regmap,
}

    static int cio_dac_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int *val, int *val2, long mask)
    {
    let mut priv: *mut cio_dac_iio const = iio_priv(indio_dev);
    let mut offset: c_uint = chan.channel * CIO_DAC_CHANNEL_STRIDE;
    int err;
    unsigned int dac_val;
    if (mask != IIO_CHAN_INFO_RAW)
    return -EINVAL;
    err = regmap_read(priv.map, CIO_DAC_BASE + offset, &dac_val);
    if (err)
    return err;
// val = dac_val;
    return IIO_VAL_INT;
    }
    static int cio_dac_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int val, int val2, long mask)
    {
    let mut priv: *mut cio_dac_iio const = iio_priv(indio_dev);
    let mut offset: c_uint = chan.channel * CIO_DAC_CHANNEL_STRIDE;
    if (mask != IIO_CHAN_INFO_RAW)
    return -EINVAL;
// DAC can only accept up to a 12-bit value
    if ((unsigned int)val > 4095)
    return -EINVAL;
    return regmap_write(priv.map, CIO_DAC_BASE + offset, val);
    }
    static const struct iio_info cio_dac_info = {
    .read_raw = cio_dac_read_raw,
    .write_raw = cio_dac_write_raw
    };
    static const struct iio_chan_spec cio_dac_channels[CIO_DAC_NUM_CHAN] = {
    CIO_DAC_CHAN(0), CIO_DAC_CHAN(1), CIO_DAC_CHAN(2), CIO_DAC_CHAN(3),
    CIO_DAC_CHAN(4), CIO_DAC_CHAN(5), CIO_DAC_CHAN(6), CIO_DAC_CHAN(7),
    CIO_DAC_CHAN(8), CIO_DAC_CHAN(9), CIO_DAC_CHAN(10), CIO_DAC_CHAN(11),
    CIO_DAC_CHAN(12), CIO_DAC_CHAN(13), CIO_DAC_CHAN(14), CIO_DAC_CHAN(15)
    };
#[no_mangle]
unsafe extern "C" fn cio_dac_probe(dev: *mut device, id: c_uint) -> c_int {
    static int cio_dac_probe(struct device *dev, unsigned int id)
    {
    struct iio_dev *indio_dev;
    struct cio_dac_iio *priv;
    void __iomem *regs;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*priv));
    if (!indio_dev)
    return -ENOMEM;
    if (!devm_request_region(dev, base[id], CIO_DAC_EXTENT,
    dev_name(dev))) {
    dev_err(dev, "Unable to request port addresses (0x%X-0x%X)\n",
    base[id], base[id] + CIO_DAC_EXTENT);
    return -EBUSY;
    }
    regs = devm_ioport_map(dev, base[id], CIO_DAC_EXTENT);
    if (!regs)
    return -ENOMEM;
    priv = iio_priv(indio_dev);
    priv.map = devm_regmap_init_mmio(dev, regs, &cio_dac_regmap_config);
    if (IS_ERR(priv.map))
    return dev_err_probe(dev, PTR_ERR(priv.map),
    "Unable to initialize register map\n");
    indio_dev.info = &cio_dac_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = cio_dac_channels;
    indio_dev.num_channels = CIO_DAC_NUM_CHAN;
    indio_dev.name = dev_name(dev);
    return devm_iio_device_register(dev, indio_dev);
    }
    static struct isa_driver cio_dac_driver = {
    .probe = cio_dac_probe,
    .driver = {
    .name = "cio-dac"
    }
    };
    module_isa_driver(cio_dac_driver, num_cio_dac);
    MODULE_AUTHOR("William Breathitt Gray <vilhelm.gray@gmail.com>");
    MODULE_DESCRIPTION("Measurement Computing CIO-DAC IIO driver");
    MODULE_LICENSE("GPL v2");
