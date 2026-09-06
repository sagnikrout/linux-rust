//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/viperboard_adc.c
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
// Nano River Technologies viperboard IIO ADC driver
//
// (C) 2012 by Lemonage GmbH
// Author: Lars Poeschel <poeschel@lemonage.de>
// All rights reserved.
//

pub const VPRBRD_ADC_CMD_GET: c_uint = 0x00;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vprbrd_adc_msg {
    pub cmd: u8,
    pub chan: u8,
    pub val: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vprbrd_adc {
    pub vb: *mut vprbrd,
}

    .type = IIO_VOLTAGE,				\
    .indexed = 1,					\
    .channel = _index,				\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),	\
    }
    static struct iio_chan_spec const vprbrd_adc_iio_channels[] = {
    VPRBRD_ADC_CHANNEL(0),
    VPRBRD_ADC_CHANNEL(1),
    VPRBRD_ADC_CHANNEL(2),
    VPRBRD_ADC_CHANNEL(3),
    };
    static int vprbrd_iio_read_raw(struct iio_dev *iio_dev,
    struct iio_chan_spec const *chan,
    int *val,
    int *val2,
    long info)
    {
    int ret, error = 0;
    struct vprbrd_adc *adc = iio_priv(iio_dev);
    struct vprbrd *vb = adc.vb;
    struct vprbrd_adc_msg *admsg = (struct vprbrd_adc_msg *)vb.buf;
    switch (info) {
    case IIO_CHAN_INFO_RAW:
    mutex_lock(&vb.lock);
    admsg.cmd = VPRBRD_ADC_CMD_GET;
    admsg.chan = chan.channel;
    admsg.val = 0x00;
    ret = usb_control_msg(vb.usb_dev,
    usb_sndctrlpipe(vb.usb_dev, 0), VPRBRD_USB_REQUEST_ADC,
    VPRBRD_USB_TYPE_OUT, 0x0000, 0x0000, admsg,
    sizeof(struct vprbrd_adc_msg), VPRBRD_USB_TIMEOUT_MS);
    if (ret != sizeof(struct vprbrd_adc_msg)) {
    mutex_unlock(&vb.lock);
    error = -EREMOTEIO;
    dev_err(&iio_dev.dev, "usb send error on adc read\n");
    goto error;
    }
    ret = usb_control_msg(vb.usb_dev,
    usb_rcvctrlpipe(vb.usb_dev, 0), VPRBRD_USB_REQUEST_ADC,
    VPRBRD_USB_TYPE_IN, 0x0000, 0x0000, admsg,
    sizeof(struct vprbrd_adc_msg), VPRBRD_USB_TIMEOUT_MS);
// val = admsg->val;
    mutex_unlock(&vb.lock);
    if (ret != sizeof(struct vprbrd_adc_msg)) {
    dev_err(&iio_dev.dev, "usb recv error on adc read\n");
    error = -EREMOTEIO;
    }
    if (error)
    goto error;
    return IIO_VAL_INT;
    default:
    error = -EINVAL;
    break;
    }
    error:
    return error;
    }
    static const struct iio_info vprbrd_adc_iio_info = {
    .read_raw = &vprbrd_iio_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn vprbrd_adc_probe(pdev: *mut platform_device) -> c_int {
    static int vprbrd_adc_probe(struct platform_device *pdev)
    {
    struct vprbrd *vb = dev_get_drvdata(pdev.dev.parent);
    struct vprbrd_adc *adc;
    struct iio_dev *indio_dev;
    int ret;
// registering iio
    indio_dev = devm_iio_device_alloc(&pdev.dev, sizeof(*adc));
    if (!indio_dev)
    return -ENOMEM;
    adc = iio_priv(indio_dev);
    adc.vb = vb;
    indio_dev.name = "viperboard adc";
    indio_dev.info = &vprbrd_adc_iio_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = vprbrd_adc_iio_channels;
    indio_dev.num_channels = ARRAY_SIZE(vprbrd_adc_iio_channels);
    ret = devm_iio_device_register(&pdev.dev, indio_dev);
    if (ret) {
    dev_err(&pdev.dev, "could not register iio (adc)");
    return ret;
    }
    return 0;
    }
    static struct platform_driver vprbrd_adc_driver = {
    .driver = {
    .name	= "viperboard-adc",
    },
    .probe		= vprbrd_adc_probe,
    };
    module_platform_driver(vprbrd_adc_driver);
    MODULE_AUTHOR("Lars Poeschel <poeschel@lemonage.de>");
    MODULE_DESCRIPTION("IIO ADC driver for Nano River Techs Viperboard");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:viperboard-adc");
