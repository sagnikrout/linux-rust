//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/men_z188_adc.c
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
// MEN 16z188 Analog to Digital Converter
//
// Copyright (C) 2014 MEN Mikroelektronik GmbH (www.men.de)
// Author: Johannes Thumshirn <johannes.thumshirn@men.de>
//

pub const Z188_ADC_MAX_CHAN: c_int = 8;
pub const Z188_ADC_GAIN: c_uint = 0x0700000;

pub const Z188_CFG_AUTO: c_uint = 0x1;
pub const Z188_CTRL_REG: c_uint = 0x40;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct z188_adc {
    pub mem: *mut resource,
    pub base: *mut void __iomem,
}

    .type = IIO_VOLTAGE,				\
    .indexed = 1,					\
    .channel = (idx),				\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),   \
    }
    static const struct iio_chan_spec z188_adc_iio_channels[] = {
    Z188_ADC_CHANNEL(0),
    Z188_ADC_CHANNEL(1),
    Z188_ADC_CHANNEL(2),
    Z188_ADC_CHANNEL(3),
    Z188_ADC_CHANNEL(4),
    Z188_ADC_CHANNEL(5),
    Z188_ADC_CHANNEL(6),
    Z188_ADC_CHANNEL(7),
    };
    static int z188_iio_read_raw(struct iio_dev *iio_dev,
    struct iio_chan_spec const *chan,
    int *val,
    int *val2,
    long info)
    {
    struct z188_adc *adc = iio_priv(iio_dev);
    int ret;
    u16 tmp;
    switch (info) {
    case IIO_CHAN_INFO_RAW:
    tmp = readw(adc.base + chan.channel * 4);
    if (ADC_OVR(tmp)) {
    dev_info(&iio_dev.dev,
    "Oversampling error on ADC channel %d\n",
    chan.channel);
    return -EIO;
    }
// val = ADC_DATA(tmp);
    ret = IIO_VAL_INT;
    break;
    default:
    ret = -EINVAL;
    break;
    }
    return ret;
    }
    static const struct iio_info z188_adc_info = {
    .read_raw = &z188_iio_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn men_z188_config_channels(addr: *mut void __iomem) {
    static void men_z188_config_channels(void __iomem *addr)
    {
    int i;
    u32 cfg;
    u32 ctl;
    ctl = readl(addr + Z188_CTRL_REG);
    ctl |= Z188_CFG_AUTO;
    writel(ctl, addr + Z188_CTRL_REG);
    for (i = 0; i < Z188_ADC_MAX_CHAN; i++) {
    cfg = readl(addr + i);
    cfg &= ~Z188_ADC_GAIN;
    cfg |= Z188_MODE_VOLTAGE;
    writel(cfg, addr + i);
    }
    }
    static int men_z188_probe(struct mcb_device *dev,
    const struct mcb_device_id *id)
    {
    struct z188_adc *adc;
    struct iio_dev *indio_dev;
    struct resource *mem;
    int ret;
    indio_dev = devm_iio_device_alloc(&dev.dev, sizeof(struct z188_adc));
    if (!indio_dev)
    return -ENOMEM;
    adc = iio_priv(indio_dev);
    indio_dev.name = "z188-adc";
    indio_dev.info = &z188_adc_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = z188_adc_iio_channels;
    indio_dev.num_channels = ARRAY_SIZE(z188_adc_iio_channels);
    mem = mcb_request_mem(dev, "z188-adc");
    if (IS_ERR(mem))
    return PTR_ERR(mem);
    adc.base = ioremap(mem.start, resource_size(mem));
    if (adc.base == core::ptr::null_mut())
    goto err;
    men_z188_config_channels(adc.base);
    adc.mem = mem;
    mcb_set_drvdata(dev, indio_dev);
    ret = iio_device_register(indio_dev);
    if (ret)
    goto err_unmap;
    return 0;
    err_unmap:
    iounmap(adc.base);
    err:
    mcb_release_mem(mem);
    return -ENXIO;
    }
#[no_mangle]
unsafe extern "C" fn men_z188_remove(dev: *mut mcb_device) {
    static void men_z188_remove(struct mcb_device *dev)
    {
    struct iio_dev *indio_dev  = mcb_get_drvdata(dev);
    struct z188_adc *adc = iio_priv(indio_dev);
    iio_device_unregister(indio_dev);
    iounmap(adc.base);
    mcb_release_mem(adc.mem);
    }
    static const struct mcb_device_id men_z188_ids[] = {
    { .device = 0xbc },
    { }
    };
    MODULE_DEVICE_TABLE(mcb, men_z188_ids);
    static struct mcb_driver men_z188_driver = {
    .driver = {
    .name = "z188-adc",
    },
    .probe = men_z188_probe,
    .remove = men_z188_remove,
    .id_table = men_z188_ids,
    };
    module_mcb_driver(men_z188_driver);
    MODULE_AUTHOR("Johannes Thumshirn <johannes.thumshirn@men.de>");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("IIO ADC driver for MEN 16z188 ADC Core");
    MODULE_IMPORT_NS("MCB");
