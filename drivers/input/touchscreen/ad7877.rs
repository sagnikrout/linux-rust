//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/ad7877.c
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
// Copyright (C) 2006-2008 Michael Hennerich, Analog Devices Inc.
//
// Description:	AD7877 based touchscreen, sensor (ADCs), DAC and GPIO driver
// Based on:	ads7846.c
//
// Bugs:        Enter bugs at http://blackfin.uclinux.org
//
// History:
// Copyright (c) 2005 David Brownell
// Copyright (c) 2006 Nokia Corporation
// Various changes: Imre Deak <imre.deak@nokia.com>
//
// Using code from:
// - corgi_ts.c
// Copyright (C) 2004-2005 Richard Purdie
// - omap_ts.[hc], ads7846.h, ts_osk.c
// Copyright (C) 2002 MontaVista Software
// Copyright (C) 2004 Texas Instruments
// Copyright (C) 2005 Dirk Behme
//

pub const MAX_SPI_FREQ_HZ: c_int = 20000000;

pub const AD7877_REG_ZEROS: c_int = 0;
pub const AD7877_REG_CTRL1: c_int = 1;
pub const AD7877_REG_CTRL2: c_int = 2;
pub const AD7877_REG_ALERT: c_int = 3;
pub const AD7877_REG_AUX1HIGH: c_int = 4;
pub const AD7877_REG_AUX1LOW: c_int = 5;
pub const AD7877_REG_BAT1HIGH: c_int = 6;
pub const AD7877_REG_BAT1LOW: c_int = 7;
pub const AD7877_REG_BAT2HIGH: c_int = 8;
pub const AD7877_REG_BAT2LOW: c_int = 9;
pub const AD7877_REG_TEMP1HIGH: c_int = 10;
pub const AD7877_REG_TEMP1LOW: c_int = 11;
pub const AD7877_REG_SEQ0: c_int = 12;
pub const AD7877_REG_SEQ1: c_int = 13;
pub const AD7877_REG_DAC: c_int = 14;
pub const AD7877_REG_NONE1: c_int = 15;
pub const AD7877_REG_EXTWRITE: c_int = 15;
pub const AD7877_REG_XPLUS: c_int = 16;
pub const AD7877_REG_YPLUS: c_int = 17;
pub const AD7877_REG_Z2: c_int = 18;
pub const AD7877_REG_aux1: c_int = 19;
pub const AD7877_REG_aux2: c_int = 20;
pub const AD7877_REG_aux3: c_int = 21;
pub const AD7877_REG_bat1: c_int = 22;
pub const AD7877_REG_bat2: c_int = 23;
pub const AD7877_REG_temp1: c_int = 24;
pub const AD7877_REG_temp2: c_int = 25;
pub const AD7877_REG_Z1: c_int = 26;
pub const AD7877_REG_GPIOCTRL1: c_int = 27;
pub const AD7877_REG_GPIOCTRL2: c_int = 28;
pub const AD7877_REG_GPIODATA: c_int = 29;
pub const AD7877_REG_NONE2: c_int = 30;
pub const AD7877_REG_NONE3: c_int = 31;

    enum {
    AD7877_SEQ_YPOS  = 0,
    AD7877_SEQ_XPOS  = 1,
    AD7877_SEQ_Z2    = 2,
    AD7877_SEQ_AUX1  = 3,
    AD7877_SEQ_AUX2  = 4,
    AD7877_SEQ_AUX3  = 5,
    AD7877_SEQ_BAT1  = 6,
    AD7877_SEQ_BAT2  = 7,
    AD7877_SEQ_TEMP1 = 8,
    AD7877_SEQ_TEMP2 = 9,
    AD7877_SEQ_Z1    = 10,
    AD7877_NR_SENSE  = 11,
    };
// DAC Register Default RANGE 0 to Vcc, Volatge Mode, DAC On
pub const AD7877_DAC_CONF: c_uint = 0x1;
// If gpio3 is set AUX3/GPIO3 acts as GPIO Output
pub const AD7877_EXTW_GPIO_3_CONF: c_uint = 0x1C4;
pub const AD7877_EXTW_GPIO_DATA: c_uint = 0x200;
// Control REG 2

// Control REG 1

    AD7877_MODE_SCC | AD7877_CHANADD(AD7877_REG_ ## x) | \
    AD7877_READADD(AD7877_REG_ ## x))

    AD7877_SEQ_Z2_BIT | AD7877_SEQ_Z1_BIT)
//
// Non-touchscreen sensors only use single-ended conversions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ser_req {
    pub reset: u16,
    pub ref_on: u16,
    pub command: u16,
    pub msg: spi_message,
    pub xfer: [spi_transfer; 6],
//
// DMA (thus cache coherency maintenance) requires the
// transfer buffers to live in their own cache lines.
//
    pub ____cacheline_aligned: u16 sample,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad7877 {
    pub input: *mut input_dev,
    pub phys: [c_char; 32],
    pub spi: *mut spi_device,
    pub model: u16,
    pub vref_delay_usecs: u16,
    pub x_plate_ohms: u16,
    pub pressure_max: u16,
    pub cmd_crtl1: u16,
    pub cmd_crtl2: u16,
    pub cmd_dummy: u16,
    pub dac: u16,
    pub stopacq_polarity: u8,
    pub first_conversion_delay: u8,
    pub acquisition_time: u8,
    pub averaging: u8,
    pub pen_down_acc_interval: u8,
    pub 2]: spi_transfer xfer[AD7877_NR_SENSE +,
    pub msg: spi_message,
    pub mutex: mutex,
    pub /: *mut *mut bool disabled; / P: mutex,
    pub /: *mut *mut bool gpio3; / P: mutex,
    pub /: *mut *mut bool gpio4; / P: mutex,
    pub lock: spinlock_t,
    pub /: *mut *mut timer_list timer; / P: lock,
//
// DMA (thus cache coherency maintenance) requires the
// transfer buffers to live in their own cache lines.
//
    pub ____cacheline_aligned: u16 conversion_data[AD7877_NR_SENSE],
}

    static bool gpio3;
    module_param(gpio3, bool, 0);
    MODULE_PARM_DESC(gpio3, "If gpio3 is set to 1 AUX3 acts as GPIO3");
#[no_mangle]
unsafe extern "C" fn ad7877_read(spi: *mut spi_device, reg: u16) -> c_int {
    static int ad7877_read(struct spi_device *spi, u16 reg)
    {
    struct ser_req *req;
    int status, ret;
    req = kzalloc_obj(*req);
    if (!req)
    return -ENOMEM;
    spi_message_init(&req.msg);
    req.command = (u16) (AD7877_WRITEADD(AD7877_REG_CTRL1) |
    AD7877_READADD(reg));
    req.xfer[0].tx_buf = &req.command;
    req.xfer[0].len = 2;
    req.xfer[0].cs_change = 1;
    req.xfer[1].rx_buf = &req.sample;
    req.xfer[1].len = 2;
    spi_message_add_tail(&req.xfer[0], &req.msg);
    spi_message_add_tail(&req.xfer[1], &req.msg);
    status = spi_sync(spi, &req.msg);
    ret = status ? : req.sample;
    kfree(req);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ad7877_write(spi: *mut spi_device, reg: u16, val: u16) -> c_int {
    static int ad7877_write(struct spi_device *spi, u16 reg, u16 val)
    {
    struct ser_req *req;
    int status;
    req = kzalloc_obj(*req);
    if (!req)
    return -ENOMEM;
    spi_message_init(&req.msg);
    req.command = (u16) (AD7877_WRITEADD(reg) | (val & MAX_12BIT));
    req.xfer[0].tx_buf = &req.command;
    req.xfer[0].len = 2;
    spi_message_add_tail(&req.xfer[0], &req.msg);
    status = spi_sync(spi, &req.msg);
    kfree(req);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn ad7877_read_adc(spi: *mut spi_device, command: unsigned) -> c_int {
    static int ad7877_read_adc(struct spi_device *spi, unsigned command)
    {
    struct ad7877 *ts = spi_get_drvdata(spi);
    struct ser_req *req;
    int status;
    int sample;
    int i;
    req = kzalloc_obj(*req);
    if (!req)
    return -ENOMEM;
    spi_message_init(&req.msg);
// activate reference, so it has time to settle;
    req.ref_on = AD7877_WRITEADD(AD7877_REG_CTRL2) |
    AD7877_POL(ts.stopacq_polarity) |
    AD7877_AVG(0) | AD7877_PM(2) | AD7877_TMR(0) |
    AD7877_ACQ(ts.acquisition_time) | AD7877_FCD(0);
    req.reset = AD7877_WRITEADD(AD7877_REG_CTRL1) | AD7877_MODE_NOC;
    req.command = (u16) command;
    req.xfer[0].tx_buf = &req.reset;
    req.xfer[0].len = 2;
    req.xfer[0].cs_change = 1;
    req.xfer[1].tx_buf = &req.ref_on;
    req.xfer[1].len = 2;
    req.xfer[1].delay.value = ts.vref_delay_usecs;
    req.xfer[1].delay.unit = SPI_DELAY_UNIT_USECS;
    req.xfer[1].cs_change = 1;
    req.xfer[2].tx_buf = &req.command;
    req.xfer[2].len = 2;
    req.xfer[2].delay.value = ts.vref_delay_usecs;
    req.xfer[2].delay.unit = SPI_DELAY_UNIT_USECS;
    req.xfer[2].cs_change = 1;
    req.xfer[3].rx_buf = &req.sample;
    req.xfer[3].len = 2;
    req.xfer[3].cs_change = 1;
    req.xfer[4].tx_buf = &ts.cmd_crtl2;	/*REF OFF*/
    req.xfer[4].len = 2;
    req.xfer[4].cs_change = 1;
    req.xfer[5].tx_buf = &ts.cmd_crtl1;	/*DEFAULT*/
    req.xfer[5].len = 2;
// group all the transfers together, so we can't interfere with
// reading touchscreen state; disable penirq while sampling
//
    for (i = 0; i < 6; i++)
    spi_message_add_tail(&req.xfer[i], &req.msg);
    status = spi_sync(spi, &req.msg);
    sample = req.sample;
    kfree(req);
    return status ? : sample;
    }
#[no_mangle]
unsafe extern "C" fn ad7877_process_data(ts: *mut ad7877) -> c_int {
    static int ad7877_process_data(struct ad7877 *ts)
    {
    struct input_dev *input_dev = ts.input;
    unsigned Rt;
    u16 x, y, z1, z2;
    x = ts.conversion_data[AD7877_SEQ_XPOS] & MAX_12BIT;
    y = ts.conversion_data[AD7877_SEQ_YPOS] & MAX_12BIT;
    z1 = ts.conversion_data[AD7877_SEQ_Z1] & MAX_12BIT;
    z2 = ts.conversion_data[AD7877_SEQ_Z2] & MAX_12BIT;
//
// The samples processed here are already preprocessed by the AD7877.
// The preprocessing function consists of an averaging filter.
// The combination of 'first conversion delay' and averaging provides a robust solution,
// discarding the spurious noise in the signal and keeping only the data of interest.
// The size of the averaging filter is programmable. (dev.platform_data, see linux/spi/ad7877.h)
// Other user-programmable conversion controls include variable acquisition time,
// and first conversion delay. Up to 16 averages can be taken per conversion.
//
    if (likely(x && z1)) {
// compute touch pressure resistance using equation #1
    Rt = (z2 - z1) * x * ts.x_plate_ohms;
    Rt /= z1;
    Rt = (Rt + 2047) >> 12;
//
// Sample found inconsistent, pressure is beyond
// the maximum. Don't report it to user space.
//
    if (Rt > ts.pressure_max)
    return -EINVAL;
    if (!timer_pending(&ts.timer))
    input_report_key(input_dev, BTN_TOUCH, 1);
    input_report_abs(input_dev, ABS_X, x);
    input_report_abs(input_dev, ABS_Y, y);
    input_report_abs(input_dev, ABS_PRESSURE, Rt);
    input_sync(input_dev);
    return 0;
    }
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn ad7877_ts_event_release(ts: *mut ad7877) {
    static inline void ad7877_ts_event_release(struct ad7877 *ts)
    {
    struct input_dev *input_dev = ts.input;
    input_report_abs(input_dev, ABS_PRESSURE, 0);
    input_report_key(input_dev, BTN_TOUCH, 0);
    input_sync(input_dev);
    }
#[no_mangle]
unsafe extern "C" fn ad7877_timer(t: *mut timer_list) {
    static void ad7877_timer(struct timer_list *t)
    {
    struct ad7877 *ts = timer_container_of(ts, t, timer);
    guard(spinlock_irqsave)(&ts.lock);
    ad7877_ts_event_release(ts);
    }
#[no_mangle]
unsafe extern "C" fn ad7877_irq(irq: c_int, handle: *mut c_void) -> irqreturn_t {
    static irqreturn_t ad7877_irq(int irq, void *handle)
    {
    struct ad7877 *ts = handle;
    int error;
    error = spi_sync(ts.spi, &ts.msg);
    if (error) {
    dev_err(&ts.spi.dev, "spi_sync -. %d\n", error);
    goto out;
    }
    scoped_guard(spinlock_irqsave, &ts.lock) {
    error = ad7877_process_data(ts);
    if (error)
    goto out;
    mod_timer(&ts.timer, jiffies + TS_PEN_UP_TIMEOUT);
    }
    out:
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ad7877_disable(data: *mut c_void) {
    static void ad7877_disable(void *data)
    {
    struct ad7877 *ts = data;
    guard(mutex)(&ts.mutex);
    if (!ts.disabled) {
    ts.disabled = true;
    disable_irq(ts.spi.irq);
    if (timer_delete_sync(&ts.timer))
    ad7877_ts_event_release(ts);
    }
//
// We know the chip's in lowpower mode since we always
// leave it that way after every request
//
    }
#[no_mangle]
unsafe extern "C" fn ad7877_enable(ts: *mut ad7877) {
    static void ad7877_enable(struct ad7877 *ts)
    {
    guard(mutex)(&ts.mutex);
    if (ts.disabled) {
    ts.disabled = false;
    enable_irq(ts.spi.irq);
    }
    }

    name ## _show(struct device *dev, struct device_attribute *attr, char *buf) \
    { \
    struct ad7877 *ts = dev_get_drvdata(dev); \
    ssize_t v = ad7877_read_adc(ts.spi, \
    AD7877_READ_CHAN(name)); \
    if (v < 0) \
    return v; \
    return sprintf(buf, "%u\n", (unsigned) v); \
    } \
    static DEVICE_ATTR(name, S_IRUGO, name ## _show, core::ptr::null_mut());
    SHOW(aux1)
    SHOW(aux2)
    SHOW(aux3)
    SHOW(bat1)
    SHOW(bat2)
    SHOW(temp1)
    SHOW(temp2)
    static ssize_t ad7877_disable_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct ad7877 *ts = dev_get_drvdata(dev);
    return sprintf(buf, "%u\n", ts.disabled);
    }
    static ssize_t ad7877_disable_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct ad7877 *ts = dev_get_drvdata(dev);
    unsigned int val;
    int error;
    error = kstrtouint(buf, 10, &val);
    if (error)
    return error;
    if (val)
    ad7877_disable(ts);
    else
    ad7877_enable(ts);
    return count;
    }
    static DEVICE_ATTR(disable, 0664, ad7877_disable_show, ad7877_disable_store);
    static ssize_t ad7877_dac_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct ad7877 *ts = dev_get_drvdata(dev);
    return sprintf(buf, "%u\n", ts.dac);
    }
    static ssize_t ad7877_dac_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct ad7877 *ts = dev_get_drvdata(dev);
    unsigned int val;
    int error;
    error = kstrtouint(buf, 10, &val);
    if (error)
    return error;
    guard(mutex)(&ts.mutex);
    ts.dac = val & 0xFF;
    ad7877_write(ts.spi, AD7877_REG_DAC, (ts.dac << 4) | AD7877_DAC_CONF);
    return count;
    }
    static DEVICE_ATTR(dac, 0664, ad7877_dac_show, ad7877_dac_store);
    static ssize_t ad7877_gpio3_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct ad7877 *ts = dev_get_drvdata(dev);
    return sprintf(buf, "%u\n", ts.gpio3);
    }
    static ssize_t ad7877_gpio3_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct ad7877 *ts = dev_get_drvdata(dev);
    unsigned int val;
    int error;
    error = kstrtouint(buf, 10, &val);
    if (error)
    return error;
    guard(mutex)(&ts.mutex);
    ts.gpio3 = !!val;
    ad7877_write(ts.spi, AD7877_REG_EXTWRITE, AD7877_EXTW_GPIO_DATA |
    (ts.gpio4 << 4) | (ts.gpio3 << 5));
    return count;
    }
    static DEVICE_ATTR(gpio3, 0664, ad7877_gpio3_show, ad7877_gpio3_store);
    static ssize_t ad7877_gpio4_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct ad7877 *ts = dev_get_drvdata(dev);
    return sprintf(buf, "%u\n", ts.gpio4);
    }
    static ssize_t ad7877_gpio4_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct ad7877 *ts = dev_get_drvdata(dev);
    unsigned int val;
    int error;
    error = kstrtouint(buf, 10, &val);
    if (error)
    return error;
    guard(mutex)(&ts.mutex);
    ts.gpio4 = !!val;
    ad7877_write(ts.spi, AD7877_REG_EXTWRITE, AD7877_EXTW_GPIO_DATA |
    (ts.gpio4 << 4) | (ts.gpio3 << 5));
    return count;
    }
    static DEVICE_ATTR(gpio4, 0664, ad7877_gpio4_show, ad7877_gpio4_store);
    static struct attribute *ad7877_attributes[] = {
    &dev_attr_temp1.attr,
    &dev_attr_temp2.attr,
    &dev_attr_aux1.attr,
    &dev_attr_aux2.attr,
    &dev_attr_aux3.attr,
    &dev_attr_bat1.attr,
    &dev_attr_bat2.attr,
    &dev_attr_disable.attr,
    &dev_attr_dac.attr,
    &dev_attr_gpio3.attr,
    &dev_attr_gpio4.attr,
    core::ptr::null_mut()
    };
    static umode_t ad7877_attr_is_visible(struct kobject *kobj,
    struct attribute *attr, int n)
    {
    let mut mode: umode_t = attr.mode;
    if (attr == &dev_attr_aux3.attr) {
    if (gpio3)
    mode = 0;
    } else if (attr == &dev_attr_gpio3.attr) {
    if (!gpio3)
    mode = 0;
    }
    return mode;
    }
    static const struct attribute_group ad7877_group = {
    .is_visible	= ad7877_attr_is_visible,
    .attrs		= ad7877_attributes,
    };
    __ATTRIBUTE_GROUPS(ad7877);
#[no_mangle]
unsafe extern "C" fn ad7877_setup_ts_def_msg(spi: *mut spi_device, ts: *mut ad7877) {
    static void ad7877_setup_ts_def_msg(struct spi_device *spi, struct ad7877 *ts)
    {
    struct spi_message *m;
    int i;
    ts.cmd_crtl2 = AD7877_WRITEADD(AD7877_REG_CTRL2) |
    AD7877_POL(ts.stopacq_polarity) |
    AD7877_AVG(ts.averaging) | AD7877_PM(1) |
    AD7877_TMR(ts.pen_down_acc_interval) |
    AD7877_ACQ(ts.acquisition_time) |
    AD7877_FCD(ts.first_conversion_delay);
    ad7877_write(spi, AD7877_REG_CTRL2, ts.cmd_crtl2);
    ts.cmd_crtl1 = AD7877_WRITEADD(AD7877_REG_CTRL1) |
    AD7877_READADD(AD7877_REG_XPLUS-1) |
    AD7877_MODE_SEQ1 | AD7877_DFR;
    ad7877_write(spi, AD7877_REG_CTRL1, ts.cmd_crtl1);
    ts.cmd_dummy = 0;
    m = &ts.msg;
    spi_message_init(m);
    m.context = ts;
    ts.xfer[0].tx_buf = &ts.cmd_crtl1;
    ts.xfer[0].len = 2;
    ts.xfer[0].cs_change = 1;
    spi_message_add_tail(&ts.xfer[0], m);
    ts.xfer[1].tx_buf = &ts.cmd_dummy; /* Send ZERO */
    ts.xfer[1].len = 2;
    ts.xfer[1].cs_change = 1;
    spi_message_add_tail(&ts.xfer[1], m);
    for (i = 0; i < AD7877_NR_SENSE; i++) {
    ts.xfer[i + 2].rx_buf = &ts.conversion_data[AD7877_SEQ_YPOS + i];
    ts.xfer[i + 2].len = 2;
    if (i < (AD7877_NR_SENSE - 1))
    ts.xfer[i + 2].cs_change = 1;
    spi_message_add_tail(&ts.xfer[i + 2], m);
    }
    }
#[no_mangle]
unsafe extern "C" fn ad7877_probe(spi: *mut spi_device) -> c_int {
    static int ad7877_probe(struct spi_device *spi)
    {
    struct ad7877			*ts;
    struct input_dev		*input_dev;
    struct ad7877_platform_data	*pdata = dev_get_platdata(&spi.dev);
    int				err;
    u16				verify;
    if (!spi.irq) {
    dev_dbg(&spi.dev, "no IRQ?\n");
    return -ENODEV;
    }
    if (!pdata) {
    dev_dbg(&spi.dev, "no platform data?\n");
    return -ENODEV;
    }
// don't exceed max specified SPI CLK frequency
    if (spi.max_speed_hz > MAX_SPI_FREQ_HZ) {
    dev_dbg(&spi.dev, "SPI CLK %d Hz?\n",spi.max_speed_hz);
    return -EINVAL;
    }
    spi.bits_per_word = 16;
    err = spi_setup(spi);
    if (err) {
    dev_dbg(&spi.dev, "spi master doesn't support 16 bits/word\n");
    return err;
    }
    ts = devm_kzalloc(&spi.dev, sizeof(struct ad7877), GFP_KERNEL);
    if (!ts)
    return -ENOMEM;
    input_dev = devm_input_allocate_device(&spi.dev);
    if (!input_dev)
    return -ENOMEM;
    err = devm_add_action_or_reset(&spi.dev, ad7877_disable, ts);
    if (err)
    return err;
    spi_set_drvdata(spi, ts);
    ts.spi = spi;
    ts.input = input_dev;
    timer_setup(&ts.timer, ad7877_timer, 0);
    mutex_init(&ts.mutex);
    spin_lock_init(&ts.lock);
    ts.model = pdata.model ? : 7877;
    ts.vref_delay_usecs = pdata.vref_delay_usecs ? : 100;
    ts.x_plate_ohms = pdata.x_plate_ohms ? : 400;
    ts.pressure_max = pdata.pressure_max ? : ~0;
    ts.stopacq_polarity = pdata.stopacq_polarity;
    ts.first_conversion_delay = pdata.first_conversion_delay;
    ts.acquisition_time = pdata.acquisition_time;
    ts.averaging = pdata.averaging;
    ts.pen_down_acc_interval = pdata.pen_down_acc_interval;
    snprintf(ts.phys, sizeof(ts.phys), "%s/input0", dev_name(&spi.dev));
    input_dev.name = "AD7877 Touchscreen";
    input_dev.phys = ts.phys;
    input_dev.dev.parent = &spi.dev;
    __set_bit(EV_KEY, input_dev.evbit);
    __set_bit(BTN_TOUCH, input_dev.keybit);
    __set_bit(EV_ABS, input_dev.evbit);
    __set_bit(ABS_X, input_dev.absbit);
    __set_bit(ABS_Y, input_dev.absbit);
    __set_bit(ABS_PRESSURE, input_dev.absbit);
    input_set_abs_params(input_dev, ABS_X,
    pdata.x_min ? : 0,
    pdata.x_max ? : MAX_12BIT,
    0, 0);
    input_set_abs_params(input_dev, ABS_Y,
    pdata.y_min ? : 0,
    pdata.y_max ? : MAX_12BIT,
    0, 0);
    input_set_abs_params(input_dev, ABS_PRESSURE,
    pdata.pressure_min, pdata.pressure_max, 0, 0);
    ad7877_write(spi, AD7877_REG_SEQ1, AD7877_MM_SEQUENCE);
    verify = ad7877_read(spi, AD7877_REG_SEQ1);
    if (verify != AD7877_MM_SEQUENCE) {
    dev_err(&spi.dev, "%s: Failed to probe %s\n",
    dev_name(&spi.dev), input_dev.name);
    return -ENODEV;
    }
    if (gpio3)
    ad7877_write(spi, AD7877_REG_EXTWRITE, AD7877_EXTW_GPIO_3_CONF);
    ad7877_setup_ts_def_msg(spi, ts);
// Request AD7877 /DAV GPIO interrupt
    err = devm_request_threaded_irq(&spi.dev, spi.irq, core::ptr::null_mut(), ad7877_irq,
    IRQF_TRIGGER_FALLING | IRQF_ONESHOT,
    spi.dev.driver.name, ts);
    if (err) {
    dev_dbg(&spi.dev, "irq %d busy?\n", spi.irq);
    return err;
    }
    err = input_register_device(input_dev);
    if (err)
    return err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ad7877_suspend(dev: *mut device) -> c_int {
    static int ad7877_suspend(struct device *dev)
    {
    struct ad7877 *ts = dev_get_drvdata(dev);
    ad7877_disable(ts);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ad7877_resume(dev: *mut device) -> c_int {
    static int ad7877_resume(struct device *dev)
    {
    struct ad7877 *ts = dev_get_drvdata(dev);
    ad7877_enable(ts);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(ad7877_pm, ad7877_suspend, ad7877_resume);
    static struct spi_driver ad7877_driver = {
    .driver = {
    .name		= "ad7877",
    .dev_groups	= ad7877_groups,
    .pm		= pm_sleep_ptr(&ad7877_pm),
    },
    .probe		= ad7877_probe,
    };
    module_spi_driver(ad7877_driver);
    MODULE_AUTHOR("Michael Hennerich <hennerich@blackfin.uclinux.org>");
    MODULE_DESCRIPTION("AD7877 touchscreen Driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("spi:ad7877");
