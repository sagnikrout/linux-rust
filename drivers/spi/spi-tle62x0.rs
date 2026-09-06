//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-tle62x0.c
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
// Support Infineon TLE62x0 driver chips
//
// Copyright (c) 2007 Simtec Electronics
// Ben Dooks, <ben@simtec.co.uk>
//

pub const CMD_READ: c_uint = 0x00;
pub const CMD_SET: c_uint = 0xff;
pub const DIAG_NORMAL: c_uint = 0x03;
pub const DIAG_OVERLOAD: c_uint = 0x02;
pub const DIAG_OPEN: c_uint = 0x01;
pub const DIAG_SHORTGND: c_uint = 0x00;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tle62x0_state {
    pub us: *mut spi_device,
    pub lock: mutex,
    pub nr_gpio: c_uint,
    pub gpio_state: c_uint,
    pub tx_buff: [c_uchar; 4],
    pub rx_buff: [c_uchar; 4],
}

    static int to_gpio_num(struct device_attribute *attr);
#[no_mangle]
pub unsafe extern "C" fn tle62x0_write(st: *mut tle62x0_state) -> c_int {
    static inline int tle62x0_write(struct tle62x0_state *st)
    {
    unsigned char *buff = st.tx_buff;
    let mut gpio_state: c_uint = st.gpio_state;
    buff[0] = CMD_SET;
    if (st.nr_gpio == 16) {
    buff[1] = gpio_state >> 8;
    buff[2] = gpio_state;
    } else {
    buff[1] = gpio_state;
    }
    dev_dbg(&st.us.dev, "buff %3ph\n", buff);
    return spi_write(st.us, buff, (st.nr_gpio == 16) ? 3 : 2);
    }
#[no_mangle]
pub unsafe extern "C" fn tle62x0_read(st: *mut tle62x0_state) -> c_int {
    static inline int tle62x0_read(struct tle62x0_state *st)
    {
    unsigned char *txbuff = st.tx_buff;
    struct spi_transfer xfer = {
    .tx_buf		= txbuff,
    .rx_buf		= st.rx_buff,
    .len		= (st.nr_gpio * 2) / 8,
    };
    struct spi_message msg;
    txbuff[0] = CMD_READ;
    txbuff[1] = 0x00;
    txbuff[2] = 0x00;
    txbuff[3] = 0x00;
    spi_message_init(&msg);
    spi_message_add_tail(&xfer, &msg);
    return spi_sync(st.us, &msg);
    }
    static unsigned char *decode_fault(unsigned int fault_code)
    {
    fault_code &= 3;
    switch (fault_code) {
    case DIAG_NORMAL:
    return "N";
    case DIAG_OVERLOAD:
    return "V";
    case DIAG_OPEN:
    return "O";
    case DIAG_SHORTGND:
    return "G";
    }
    return "?";
    }
    static ssize_t tle62x0_status_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct tle62x0_state *st = dev_get_drvdata(dev);
    char *bp = buf;
    unsigned char *buff = st.rx_buff;
    let mut fault: c_ulong = 0;
    int ptr;
    int ret;
    mutex_lock(&st.lock);
    ret = tle62x0_read(st);
    dev_dbg(dev, "tle62x0_read() returned %d\n", ret);
    if (ret < 0) {
    mutex_unlock(&st.lock);
    return ret;
    }
    for (ptr = 0; ptr < (st.nr_gpio * 2)/8; ptr += 1) {
    fault <<= 8;
    fault  |= ((unsigned long)buff[ptr]);
    dev_dbg(dev, "byte %d is %02x\n", ptr, buff[ptr]);
    }
    for (ptr = 0; ptr < st.nr_gpio; ptr++) {
    bp += sprintf(bp, "%s ", decode_fault(fault >> (ptr * 2)));
    }
// bp++ = '\n';
    mutex_unlock(&st.lock);
    return bp - buf;
    }
    static DEVICE_ATTR(status_show, S_IRUGO, tle62x0_status_show, core::ptr::null_mut());
    static ssize_t tle62x0_gpio_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct tle62x0_state *st = dev_get_drvdata(dev);
    let mut gpio_num: c_int = to_gpio_num(attr);
    int value;
    mutex_lock(&st.lock);
    value = (st.gpio_state >> gpio_num) & 1;
    mutex_unlock(&st.lock);
    return sysfs_emit(buf, "%d\n", value);
    }
    static ssize_t tle62x0_gpio_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t len)
    {
    struct tle62x0_state *st = dev_get_drvdata(dev);
    let mut gpio_num: c_int = to_gpio_num(attr);
    unsigned long val;
    char *endp;
    val = simple_strtoul(buf, &endp, 0);
    if (buf == endp)
    return -EINVAL;
    dev_dbg(dev, "setting gpio %d to %ld\n", gpio_num, val);
    mutex_lock(&st.lock);
    if (val)
    st.gpio_state |= 1 << gpio_num;
    else
    st.gpio_state &= ~(1 << gpio_num);
    tle62x0_write(st);
    mutex_unlock(&st.lock);
    return len;
    }
    static DEVICE_ATTR(gpio1, S_IWUSR|S_IRUGO,
    tle62x0_gpio_show, tle62x0_gpio_store);
    static DEVICE_ATTR(gpio2, S_IWUSR|S_IRUGO,
    tle62x0_gpio_show, tle62x0_gpio_store);
    static DEVICE_ATTR(gpio3, S_IWUSR|S_IRUGO,
    tle62x0_gpio_show, tle62x0_gpio_store);
    static DEVICE_ATTR(gpio4, S_IWUSR|S_IRUGO,
    tle62x0_gpio_show, tle62x0_gpio_store);
    static DEVICE_ATTR(gpio5, S_IWUSR|S_IRUGO,
    tle62x0_gpio_show, tle62x0_gpio_store);
    static DEVICE_ATTR(gpio6, S_IWUSR|S_IRUGO,
    tle62x0_gpio_show, tle62x0_gpio_store);
    static DEVICE_ATTR(gpio7, S_IWUSR|S_IRUGO,
    tle62x0_gpio_show, tle62x0_gpio_store);
    static DEVICE_ATTR(gpio8, S_IWUSR|S_IRUGO,
    tle62x0_gpio_show, tle62x0_gpio_store);
    static DEVICE_ATTR(gpio9, S_IWUSR|S_IRUGO,
    tle62x0_gpio_show, tle62x0_gpio_store);
    static DEVICE_ATTR(gpio10, S_IWUSR|S_IRUGO,
    tle62x0_gpio_show, tle62x0_gpio_store);
    static DEVICE_ATTR(gpio11, S_IWUSR|S_IRUGO,
    tle62x0_gpio_show, tle62x0_gpio_store);
    static DEVICE_ATTR(gpio12, S_IWUSR|S_IRUGO,
    tle62x0_gpio_show, tle62x0_gpio_store);
    static DEVICE_ATTR(gpio13, S_IWUSR|S_IRUGO,
    tle62x0_gpio_show, tle62x0_gpio_store);
    static DEVICE_ATTR(gpio14, S_IWUSR|S_IRUGO,
    tle62x0_gpio_show, tle62x0_gpio_store);
    static DEVICE_ATTR(gpio15, S_IWUSR|S_IRUGO,
    tle62x0_gpio_show, tle62x0_gpio_store);
    static DEVICE_ATTR(gpio16, S_IWUSR|S_IRUGO,
    tle62x0_gpio_show, tle62x0_gpio_store);
    static struct device_attribute *gpio_attrs[] = {
    [0]		= &dev_attr_gpio1,
    [1]		= &dev_attr_gpio2,
    [2]		= &dev_attr_gpio3,
    [3]		= &dev_attr_gpio4,
    [4]		= &dev_attr_gpio5,
    [5]		= &dev_attr_gpio6,
    [6]		= &dev_attr_gpio7,
    [7]		= &dev_attr_gpio8,
    [8]		= &dev_attr_gpio9,
    [9]		= &dev_attr_gpio10,
    [10]		= &dev_attr_gpio11,
    [11]		= &dev_attr_gpio12,
    [12]		= &dev_attr_gpio13,
    [13]		= &dev_attr_gpio14,
    [14]		= &dev_attr_gpio15,
    [15]		= &dev_attr_gpio16
    };
#[no_mangle]
unsafe extern "C" fn to_gpio_num(attr: *mut device_attribute) -> c_int {
    static int to_gpio_num(struct device_attribute *attr)
    {
    int ptr;
    for (ptr = 0; ptr < ARRAY_SIZE(gpio_attrs); ptr++) {
    if (gpio_attrs[ptr] == attr)
    return ptr;
    }
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn tle62x0_probe(spi: *mut spi_device) -> c_int {
    static int tle62x0_probe(struct spi_device *spi)
    {
    struct tle62x0_state *st;
    struct tle62x0_pdata *pdata;
    int ptr;
    int ret;
    pdata = dev_get_platdata(&spi.dev);
    if (pdata == core::ptr::null_mut()) {
    dev_err(&spi.dev, "no device data specified\n");
    return -EINVAL;
    }
    st = kzalloc_obj(struct tle62x0_state);
    if (st == core::ptr::null_mut())
    return -ENOMEM;
    st.us = spi;
    st.nr_gpio = pdata.gpio_count;
    st.gpio_state = pdata.init_state;
    mutex_init(&st.lock);
    ret = device_create_file(&spi.dev, &dev_attr_status_show);
    if (ret) {
    dev_err(&spi.dev, "cannot create status attribute\n");
    goto err_status;
    }
    for (ptr = 0; ptr < pdata.gpio_count; ptr++) {
    ret = device_create_file(&spi.dev, gpio_attrs[ptr]);
    if (ret) {
    dev_err(&spi.dev, "cannot create gpio attribute\n");
    goto err_gpios;
    }
    }
// tle62x0_write(st);
    spi_set_drvdata(spi, st);
    return 0;
    err_gpios:
    while (--ptr >= 0)
    device_remove_file(&spi.dev, gpio_attrs[ptr]);
    device_remove_file(&spi.dev, &dev_attr_status_show);
    err_status:
    kfree(st);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tle62x0_remove(spi: *mut spi_device) {
    static void tle62x0_remove(struct spi_device *spi)
    {
    struct tle62x0_state *st = spi_get_drvdata(spi);
    int ptr;
    for (ptr = 0; ptr < st.nr_gpio; ptr++)
    device_remove_file(&spi.dev, gpio_attrs[ptr]);
    device_remove_file(&spi.dev, &dev_attr_status_show);
    kfree(st);
    }
    static struct spi_driver tle62x0_driver = {
    .driver = {
    .name	= "tle62x0",
    },
    .probe		= tle62x0_probe,
    .remove		= tle62x0_remove,
    };
    module_spi_driver(tle62x0_driver);
    MODULE_AUTHOR("Ben Dooks <ben@simtec.co.uk>");
    MODULE_DESCRIPTION("TLE62x0 SPI driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("spi:tle62x0");
