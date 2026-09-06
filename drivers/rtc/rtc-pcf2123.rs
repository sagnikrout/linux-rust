//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-pcf2123.c
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
// An SPI driver for the Philips PCF2123 RTC
// Copyright 2009 Cyber Switching, Inc.
//
// Author: Chris Verges <chrisv@cyberswitching.com>
// Maintainers: http://www.cyberswitching.com
//
// based on the RS5C348 driver in this same directory.
//
// Thanks to Christian Pellegrin <chripell@fsfe.org> for
// the sysfs contributions to this driver.
//
// Please note that the CS is active high, so platform data
// should look something like:
//
// static struct spi_board_info ek_spi_devices[] = {
// ...
// {
// .modalias		= "rtc-pcf2123",
// .chip_select		= 1,
// .controller_data	= (void *)AT91_PIN_PA10,
// .max_speed_hz		= 1000 * 1000,
// .mode			= SPI_CS_HIGH,
// .bus_num		= 0,
// },
// ...
// };
//

// REGISTERS

// PCF2123_REG_CTRL1 BITS

// PCF2123_REG_CTRL2 BITS

// PCF2123_REG_SC BITS

// PCF2123_REG_ALRM_XX BITS

// PCF2123_REG_TMR_CLKOUT BITS

// PCF2123_REG_OFFSET BITS

// READ/WRITE ADDRESS BITS

    static struct spi_driver pcf2123_driver;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcf2123_data {
    pub rtc: *mut rtc_device,
    pub map: *mut regmap,
}

    static const struct regmap_config pcf2123_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .read_flag_mask = PCF2123_READ,
    .write_flag_mask = PCF2123_WRITE,
    .max_register = PCF2123_REG_CTDWN_TMR,
    };
#[no_mangle]
unsafe extern "C" fn pcf2123_read_offset(dev: *mut device, offset: *mut c_long) -> c_int {
    static int pcf2123_read_offset(struct device *dev, long *offset)
    {
    struct pcf2123_data *pcf2123 = dev_get_drvdata(dev);
    int ret, val;
    unsigned int reg;
    ret = regmap_read(pcf2123.map, PCF2123_REG_OFFSET, &reg);
    if (ret)
    return ret;
    val = sign_extend32((reg & OFFSET_MASK), OFFSET_SIGN_BIT);
    if (reg & OFFSET_COARSE)
    val *= 2;
// offset = ((long)val) * OFFSET_STEP;
    return 0;
    }
//
// The offset register is a 7 bit signed value with a coarse bit in bit 7.
// The main difference between the two is normal offset adjusts the first
// second of n minutes every other hour, with 61, 62 and 63 being shoved
// into the 60th minute.
// The coarse adjustment does the same, but every hour.
// the two overlap, with every even normal offset value corresponding
// to a coarse offset. Based on this algorithm, it seems that despite the
// name, coarse offset is a better fit for overlapping values.
//
#[no_mangle]
unsafe extern "C" fn pcf2123_set_offset(dev: *mut device, offset: c_long) -> c_int {
    static int pcf2123_set_offset(struct device *dev, long offset)
    {
    struct pcf2123_data *pcf2123 = dev_get_drvdata(dev);
    s8 reg;
    if (offset > OFFSET_STEP * 127)
    reg = 127;
#[no_mangle]
pub unsafe extern "C" fn if(-128: *mut *mut offset < OFFSET_STEP) -> else {
    else if (offset < OFFSET_STEP * -128)
    reg = -128;
    else
    reg = DIV_ROUND_CLOSEST(offset, OFFSET_STEP);
// choose fine offset only for odd values in the normal range
    if (reg & 1 && reg <= 63 && reg >= -64) {
// Normal offset. Clear the coarse bit
    reg &= ~OFFSET_COARSE;
    } else {
// Coarse offset. Divide by 2 and set the coarse bit
    reg >>= 1;
    reg |= OFFSET_COARSE;
    }
    return regmap_write(pcf2123.map, PCF2123_REG_OFFSET, (unsigned int)reg);
    }
#[no_mangle]
unsafe extern "C" fn pcf2123_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int pcf2123_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct pcf2123_data *pcf2123 = dev_get_drvdata(dev);
    u8 rxbuf[7];
    int ret;
    ret = regmap_bulk_read(pcf2123.map, PCF2123_REG_SC, rxbuf,
    sizeof(rxbuf));
    if (ret)
    return ret;
    if (rxbuf[0] & OSC_HAS_STOPPED) {
    dev_info(dev, "clock was stopped. Time is not valid\n");
    return -EINVAL;
    }
    tm.tm_sec = bcd2bin(rxbuf[0] & 0x7F);
    tm.tm_min = bcd2bin(rxbuf[1] & 0x7F);
    tm.tm_hour = bcd2bin(rxbuf[2] & 0x3F); /* rtc hr 0-23 */
    tm.tm_mday = bcd2bin(rxbuf[3] & 0x3F);
    tm.tm_wday = rxbuf[4] & 0x07;
    tm.tm_mon = bcd2bin(rxbuf[5] & 0x1F) - 1; /* rtc mn 1-12 */
    tm.tm_year = bcd2bin(rxbuf[6]) + 100;
    dev_dbg(dev, "%s: tm is %ptR\n", __func__, tm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pcf2123_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int pcf2123_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct pcf2123_data *pcf2123 = dev_get_drvdata(dev);
    u8 txbuf[7];
    int ret;
    dev_dbg(dev, "%s: tm is %ptR\n", __func__, tm);
// Stop the counter first
    ret = regmap_write(pcf2123.map, PCF2123_REG_CTRL1, CTRL1_STOP);
    if (ret)
    return ret;
// Set the new time
    txbuf[0] = bin2bcd(tm.tm_sec & 0x7F);
    txbuf[1] = bin2bcd(tm.tm_min & 0x7F);
    txbuf[2] = bin2bcd(tm.tm_hour & 0x3F);
    txbuf[3] = bin2bcd(tm.tm_mday & 0x3F);
    txbuf[4] = tm.tm_wday & 0x07;
    txbuf[5] = bin2bcd((tm.tm_mon + 1) & 0x1F); /* rtc mn 1-12 */
    txbuf[6] = bin2bcd(tm.tm_year - 100);
    ret = regmap_bulk_write(pcf2123.map, PCF2123_REG_SC, txbuf,
    sizeof(txbuf));
    if (ret)
    return ret;
// Start the counter
    ret = regmap_write(pcf2123.map, PCF2123_REG_CTRL1, CTRL1_CLEAR);
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pcf2123_rtc_alarm_irq_enable(dev: *mut device, en: c_uint) -> c_int {
    static int pcf2123_rtc_alarm_irq_enable(struct device *dev, unsigned int en)
    {
    struct pcf2123_data *pcf2123 = dev_get_drvdata(dev);
    return regmap_update_bits(pcf2123.map, PCF2123_REG_CTRL2, CTRL2_AIE,
    en ? CTRL2_AIE : 0);
    }
#[no_mangle]
unsafe extern "C" fn pcf2123_rtc_read_alarm(dev: *mut device, alm: *mut rtc_wkalrm) -> c_int {
    static int pcf2123_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *alm)
    {
    struct pcf2123_data *pcf2123 = dev_get_drvdata(dev);
    u8 rxbuf[4];
    int ret;
    let mut val: c_uint = 0;
    ret = regmap_bulk_read(pcf2123.map, PCF2123_REG_ALRM_MN, rxbuf,
    sizeof(rxbuf));
    if (ret)
    return ret;
    alm.time.tm_min = bcd2bin(rxbuf[0] & 0x7F);
    alm.time.tm_hour = bcd2bin(rxbuf[1] & 0x3F);
    alm.time.tm_mday = bcd2bin(rxbuf[2] & 0x3F);
    alm.time.tm_wday = bcd2bin(rxbuf[3] & 0x07);
    dev_dbg(dev, "%s: alm is %ptR\n", __func__, &alm.time);
    ret = regmap_read(pcf2123.map, PCF2123_REG_CTRL2, &val);
    if (ret)
    return ret;
    alm.enabled = !!(val & CTRL2_AIE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pcf2123_rtc_set_alarm(dev: *mut device, alm: *mut rtc_wkalrm) -> c_int {
    static int pcf2123_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *alm)
    {
    struct pcf2123_data *pcf2123 = dev_get_drvdata(dev);
    u8 txbuf[4];
    int ret;
    dev_dbg(dev, "%s: alm is %ptR\n", __func__, &alm.time);
// Disable alarm interrupt
    ret = regmap_update_bits(pcf2123.map, PCF2123_REG_CTRL2, CTRL2_AIE, 0);
    if (ret)
    return ret;
// Ensure alarm flag is clear
    ret = regmap_update_bits(pcf2123.map, PCF2123_REG_CTRL2, CTRL2_AF, 0);
    if (ret)
    return ret;
// Set new alarm
    txbuf[0] = bin2bcd(alm.time.tm_min & 0x7F);
    txbuf[1] = bin2bcd(alm.time.tm_hour & 0x3F);
    txbuf[2] = bin2bcd(alm.time.tm_mday & 0x3F);
    txbuf[3] = ALRM_DISABLE;
    ret = regmap_bulk_write(pcf2123.map, PCF2123_REG_ALRM_MN, txbuf,
    sizeof(txbuf));
    if (ret)
    return ret;
    return pcf2123_rtc_alarm_irq_enable(dev, alm.enabled);
    }
#[no_mangle]
unsafe extern "C" fn pcf2123_rtc_irq(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t pcf2123_rtc_irq(int irq, void *dev)
    {
    struct pcf2123_data *pcf2123 = dev_get_drvdata(dev);
    let mut val: c_uint = 0;
    let mut ret: c_int = IRQ_NONE;
    rtc_lock(pcf2123.rtc);
    regmap_read(pcf2123.map, PCF2123_REG_CTRL2, &val);
// Alarm?
    if (val & CTRL2_AF) {
    ret = IRQ_HANDLED;
// Clear alarm flag
    regmap_update_bits(pcf2123.map, PCF2123_REG_CTRL2, CTRL2_AF, 0);
    rtc_update_irq(pcf2123.rtc, 1, RTC_IRQF | RTC_AF);
    }
    rtc_unlock(pcf2123.rtc);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pcf2123_reset(dev: *mut device) -> c_int {
    static int pcf2123_reset(struct device *dev)
    {
    struct pcf2123_data *pcf2123 = dev_get_drvdata(dev);
    int ret;
    let mut val: c_uint = 0;
    ret = regmap_write(pcf2123.map, PCF2123_REG_CTRL1, CTRL1_SW_RESET);
    if (ret)
    return ret;
// Stop the counter
    dev_dbg(dev, "stopping RTC\n");
    ret = regmap_write(pcf2123.map, PCF2123_REG_CTRL1, CTRL1_STOP);
    if (ret)
    return ret;
// See if the counter was actually stopped
    dev_dbg(dev, "checking for presence of RTC\n");
    ret = regmap_read(pcf2123.map, PCF2123_REG_CTRL1, &val);
    if (ret)
    return ret;
    dev_dbg(dev, "received data from RTC (0x%08X)\n", val);
    if (!(val & CTRL1_STOP))
    return -ENODEV;
// Start the counter
    ret = regmap_write(pcf2123.map, PCF2123_REG_CTRL1, CTRL1_CLEAR);
    if (ret)
    return ret;
    return 0;
    }
    static const struct rtc_class_ops pcf2123_rtc_ops = {
    .read_time	= pcf2123_rtc_read_time,
    .set_time	= pcf2123_rtc_set_time,
    .read_offset	= pcf2123_read_offset,
    .set_offset	= pcf2123_set_offset,
    .read_alarm	= pcf2123_rtc_read_alarm,
    .set_alarm	= pcf2123_rtc_set_alarm,
    .alarm_irq_enable = pcf2123_rtc_alarm_irq_enable,
    };
#[no_mangle]
unsafe extern "C" fn pcf2123_probe(spi: *mut spi_device) -> c_int {
    static int pcf2123_probe(struct spi_device *spi)
    {
    struct rtc_device *rtc;
    struct rtc_time tm;
    struct pcf2123_data *pcf2123;
    let mut ret: c_int = 0;
    pcf2123 = devm_kzalloc(&spi.dev, sizeof(struct pcf2123_data),
    GFP_KERNEL);
    if (!pcf2123)
    return -ENOMEM;
    dev_set_drvdata(&spi.dev, pcf2123);
    pcf2123.map = devm_regmap_init_spi(spi, &pcf2123_regmap_config);
    if (IS_ERR(pcf2123.map)) {
    dev_err(&spi.dev, "regmap init failed.\n");
    return PTR_ERR(pcf2123.map);
    }
    ret = pcf2123_rtc_read_time(&spi.dev, &tm);
    if (ret < 0) {
    ret = pcf2123_reset(&spi.dev);
    if (ret < 0) {
    dev_err(&spi.dev, "chip not found\n");
    return ret;
    }
    }
    dev_info(&spi.dev, "spiclk %u KHz.\n",
    (spi.max_speed_hz + 500) / 1000);
// Finalize the initialization
    rtc = devm_rtc_allocate_device(&spi.dev);
    if (IS_ERR(rtc))
    return PTR_ERR(rtc);
    pcf2123.rtc = rtc;
// Register alarm irq
    if (spi.irq > 0) {
    let mut irqflags: c_ulong = IRQF_TRIGGER_LOW;
    if (dev_fwnode(&spi.dev))
    irqflags = 0;
    ret = devm_request_threaded_irq(&spi.dev, spi.irq, core::ptr::null_mut(),
    pcf2123_rtc_irq,
    irqflags | IRQF_ONESHOT,
    pcf2123_driver.driver.name, &spi.dev);
    if (!ret)
    device_init_wakeup(&spi.dev, true);
    else
    dev_err(&spi.dev, "could not request irq.\n");
    }
// The PCF2123's alarm only has minute accuracy. Must add timer
// support to this driver to generate interrupts more than once
// per minute.
//
    set_bit(RTC_FEATURE_ALARM_RES_MINUTE, rtc.features);
    clear_bit(RTC_FEATURE_UPDATE_INTERRUPT, rtc.features);
    rtc.ops = &pcf2123_rtc_ops;
    rtc.range_min = RTC_TIMESTAMP_BEGIN_2000;
    rtc.range_max = RTC_TIMESTAMP_END_2099;
    rtc.set_start_time = true;
    ret = devm_rtc_register_device(rtc);
    if (ret)
    return ret;
    return 0;
    }

    static const struct of_device_id pcf2123_dt_ids[] = {
    { .compatible = "nxp,pcf2123", },
    { .compatible = "microcrystal,rv2123", },
// Deprecated, do not use
    { .compatible = "nxp,rtc-pcf2123", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, pcf2123_dt_ids);

    static const struct spi_device_id pcf2123_spi_ids[] = {
    { .name = "pcf2123", },
    { .name = "rv2123", },
    { .name = "rtc-pcf2123", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(spi, pcf2123_spi_ids);
    static struct spi_driver pcf2123_driver = {
    .driver	= {
    .name	= "rtc-pcf2123",
    .of_match_table = of_match_ptr(pcf2123_dt_ids),
    },
    .probe	= pcf2123_probe,
    .id_table = pcf2123_spi_ids,
    };
    module_spi_driver(pcf2123_driver);
    MODULE_AUTHOR("Chris Verges <chrisv@cyberswitching.com>");
    MODULE_DESCRIPTION("NXP PCF2123 RTC driver");
    MODULE_LICENSE("GPL");
