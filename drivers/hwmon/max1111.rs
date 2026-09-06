//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/max1111.c
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
// max1111.c - +2.7V, Low-Power, Multichannel, Serial 8-bit ADCs
//
// Based on arch/arm/mach-pxa/corgi_ssp.c
//
// Copyright (C) 2004-2005 Richard Purdie
//
// Copyright (C) 2008 Marvell International Ltd.
// Eric Miao <eric.miao@marvell.com>
//

    enum chips { max1110, max1111, max1112, max1113 };
pub const MAX1111_TX_BUF_SIZE: c_int = 1;
pub const MAX1111_RX_BUF_SIZE: c_int = 2;
// MAX1111 Commands

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max1111_data {
    pub spi: *mut spi_device,
    pub hwmon_dev: *mut device,
    pub msg: spi_message,
    pub xfer: [spi_transfer; 2],
    pub tx_buf: [u8; MAX1111_TX_BUF_SIZE],
    pub rx_buf: [u8; MAX1111_RX_BUF_SIZE],
    pub drvdata_lock: mutex,
// protect msg, xfer and buffers from multiple access
    pub sel_sh: c_int,
    pub lsb: c_int,
}

#[no_mangle]
unsafe extern "C" fn max1111_read(dev: *mut device, channel: c_int) -> c_int {
    static int max1111_read(struct device *dev, int channel)
    {
    struct max1111_data *data = dev_get_drvdata(dev);
    uint8_t v1, v2;
    int err;
// writing to drvdata struct is not thread safe, wait on mutex
    mutex_lock(&data.drvdata_lock);
    data.tx_buf[0] = (channel << data.sel_sh) |
    MAX1111_CTRL_PD0 | MAX1111_CTRL_PD1 |
    MAX1111_CTRL_SGL | MAX1111_CTRL_UNI | MAX1111_CTRL_STR;
    err = spi_sync(data.spi, &data.msg);
    if (err < 0) {
    dev_err(dev, "spi_sync failed with %d\n", err);
    mutex_unlock(&data.drvdata_lock);
    return err;
    }
    v1 = data.rx_buf[0];
    v2 = data.rx_buf[1];
    mutex_unlock(&data.drvdata_lock);
    if ((v1 & 0xc0) || (v2 & 0x3f))
    return -EINVAL;
    return (v1 << 2) | (v2 >> 6);
    }

    static struct max1111_data *the_max1111;
    int max1111_read_channel(int channel);
#[no_mangle]
pub unsafe extern "C" fn max1111_read_channel(channel: c_int) -> c_int {
    int max1111_read_channel(int channel)
    {
    if (!the_max1111 || !the_max1111.spi)
    return -ENODEV;
    return max1111_read(&the_max1111.spi.dev, channel);
    }
    EXPORT_SYMBOL(max1111_read_channel);

//
// NOTE: SPI devices do not have a default 'name' attribute, which is
// likely to be used by hwmon applications to distinguish between
// different devices, explicitly add a name attribute here.
//
    static ssize_t name_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    return sprintf(buf, "%s\n", to_spi_device(dev).modalias);
    }
    static ssize_t show_adc(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct max1111_data *data = dev_get_drvdata(dev);
    let mut channel: c_int = to_sensor_dev_attr(attr).index;
    int ret;
    ret = max1111_read(dev, channel);
    if (ret < 0)
    return ret;
//
// Assume the reference voltage to be 2.048V or 4.096V, with an 8-bit
// sample. The LSB weight is 8mV or 16mV depending on the chip type.
//
    return sprintf(buf, "%d\n", ret * data.lsb);
    }

    SENSOR_DEVICE_ATTR(in##_id##_input, S_IRUGO, show_adc, core::ptr::null_mut(), _id)
    static DEVICE_ATTR_RO(name);
    static MAX1111_ADC_ATTR(0);
    static MAX1111_ADC_ATTR(1);
    static MAX1111_ADC_ATTR(2);
    static MAX1111_ADC_ATTR(3);
    static MAX1111_ADC_ATTR(4);
    static MAX1111_ADC_ATTR(5);
    static MAX1111_ADC_ATTR(6);
    static MAX1111_ADC_ATTR(7);
    static struct attribute *max1111_attributes[] = {
    &dev_attr_name.attr,
    &sensor_dev_attr_in0_input.dev_attr.attr,
    &sensor_dev_attr_in1_input.dev_attr.attr,
    &sensor_dev_attr_in2_input.dev_attr.attr,
    &sensor_dev_attr_in3_input.dev_attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group max1111_attr_group = {
    .attrs	= max1111_attributes,
    };
    static struct attribute *max1110_attributes[] = {
    &sensor_dev_attr_in4_input.dev_attr.attr,
    &sensor_dev_attr_in5_input.dev_attr.attr,
    &sensor_dev_attr_in6_input.dev_attr.attr,
    &sensor_dev_attr_in7_input.dev_attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group max1110_attr_group = {
    .attrs	= max1110_attributes,
    };
#[no_mangle]
unsafe extern "C" fn setup_transfer(data: *mut max1111_data) -> c_int {
    static int setup_transfer(struct max1111_data *data)
    {
    struct spi_message *m;
    struct spi_transfer *x;
    m = &data.msg;
    x = &data.xfer[0];
    spi_message_init(m);
    x.tx_buf = &data.tx_buf[0];
    x.len = MAX1111_TX_BUF_SIZE;
    spi_message_add_tail(x, m);
    x++;
    x.rx_buf = &data.rx_buf[0];
    x.len = MAX1111_RX_BUF_SIZE;
    spi_message_add_tail(x, m);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max1111_probe(spi: *mut spi_device) -> c_int {
    static int max1111_probe(struct spi_device *spi)
    {
    let mut chip: enum chips = spi_get_device_id(spi).driver_data;
    struct max1111_data *data;
    int err;
    spi.bits_per_word = 8;
    spi.mode = SPI_MODE_0;
    err = spi_setup(spi);
    if (err < 0)
    return err;
    data = devm_kzalloc(&spi.dev, sizeof(struct max1111_data), GFP_KERNEL);
    if (data == core::ptr::null_mut())
    return -ENOMEM;
    switch (chip) {
    case max1110:
    data.lsb = 8;
    data.sel_sh = MAX1110_CTRL_SEL_SH;
    break;
    case max1111:
    data.lsb = 8;
    data.sel_sh = MAX1111_CTRL_SEL_SH;
    break;
    case max1112:
    data.lsb = 16;
    data.sel_sh = MAX1110_CTRL_SEL_SH;
    break;
    case max1113:
    data.lsb = 16;
    data.sel_sh = MAX1111_CTRL_SEL_SH;
    break;
    }
    err = setup_transfer(data);
    if (err)
    return err;
    mutex_init(&data.drvdata_lock);
    data.spi = spi;
    spi_set_drvdata(spi, data);
    err = sysfs_create_group(&spi.dev.kobj, &max1111_attr_group);
    if (err) {
    dev_err(&spi.dev, "failed to create attribute group\n");
    return err;
    }
    if (chip == max1110 || chip == max1112) {
    err = sysfs_create_group(&spi.dev.kobj, &max1110_attr_group);
    if (err) {
    dev_err(&spi.dev,
    "failed to create extended attribute group\n");
    goto err_remove;
    }
    }
    data.hwmon_dev = hwmon_device_register(&spi.dev);
    if (IS_ERR(data.hwmon_dev)) {
    dev_err(&spi.dev, "failed to create hwmon device\n");
    err = PTR_ERR(data.hwmon_dev);
    goto err_remove;
    }

    the_max1111 = data;

    return 0;
    err_remove:
    sysfs_remove_group(&spi.dev.kobj, &max1110_attr_group);
    sysfs_remove_group(&spi.dev.kobj, &max1111_attr_group);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn max1111_remove(spi: *mut spi_device) {
    static void max1111_remove(struct spi_device *spi)
    {
    struct max1111_data *data = spi_get_drvdata(spi);

    the_max1111 = core::ptr::null_mut();

    hwmon_device_unregister(data.hwmon_dev);
    sysfs_remove_group(&spi.dev.kobj, &max1110_attr_group);
    sysfs_remove_group(&spi.dev.kobj, &max1111_attr_group);
    mutex_destroy(&data.drvdata_lock);
    }
    static const struct spi_device_id max1111_ids[] = {
    { "max1110", max1110 },
    { "max1111", max1111 },
    { "max1112", max1112 },
    { "max1113", max1113 },
    { },
    };
    MODULE_DEVICE_TABLE(spi, max1111_ids);
    static struct spi_driver max1111_driver = {
    .driver		= {
    .name	= "max1111",
    },
    .id_table	= max1111_ids,
    .probe		= max1111_probe,
    .remove		= max1111_remove,
    };
    module_spi_driver(max1111_driver);
    MODULE_AUTHOR("Eric Miao <eric.miao@marvell.com>");
    MODULE_DESCRIPTION("MAX1110/MAX1111/MAX1112/MAX1113 ADC Driver");
    MODULE_LICENSE("GPL");
