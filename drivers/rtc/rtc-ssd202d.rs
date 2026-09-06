//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-ssd202d.c
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
// Real time clocks driver for MStar/SigmaStar SSD202D SoCs.
//
// (C) 2021 Daniel Palmer
// (C) 2023 Romain Perier
//

pub const REG_CTRL: c_uint = 0x0;
pub const REG_CTRL1: c_uint = 0x4;
pub const REG_ISO_CTRL: c_uint = 0xc;
pub const REG_WRDATA_L: c_uint = 0x10;
pub const REG_WRDATA_H: c_uint = 0x14;
pub const REG_ISOACK: c_uint = 0x20;
pub const REG_RDDATA_L: c_uint = 0x24;
pub const REG_RDDATA_H: c_uint = 0x28;
pub const REG_RDCNT_L: c_uint = 0x30;
pub const REG_RDCNT_H: c_uint = 0x34;
pub const REG_CNT_TRIG: c_uint = 0x38;
pub const REG_PWRCTRL: c_uint = 0x3c;
pub const REG_RTC_TEST: c_uint = 0x54;

pub const ISO_CTRL_ACK_SHIFT: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssd202d_rtc {
    pub rtc_dev: *mut rtc_device,
    pub base: *mut void __iomem,
}

#[no_mangle]
unsafe extern "C" fn read_iso_en(base: *mut void __iomem) -> u8 {
    static u8 read_iso_en(void __iomem *base)
    {
    return readb(base + REG_RTC_TEST) & 0x1;
    }
#[no_mangle]
unsafe extern "C" fn read_iso_ctrl_ack(base: *mut void __iomem) -> u8 {
    static u8 read_iso_ctrl_ack(void __iomem *base)
    {
    return (readb(base + REG_ISOACK) & ISO_CTRL_ACK_MASK) >> ISO_CTRL_ACK_SHIFT;
    }
#[no_mangle]
unsafe extern "C" fn ssd202d_rtc_isoctrl(priv: *mut ssd202d_rtc) -> c_int {
    static int ssd202d_rtc_isoctrl(struct ssd202d_rtc *priv)
    {
    static const unsigned int sequence[] = { 0x0, 0x1, 0x3, 0x7, 0x5, 0x1, 0x0 };
    unsigned int val;
    struct device *dev = &priv.rtc_dev.dev;
    int i, ret;
//
// This gates iso_en by writing a special sequence of bytes to iso_ctrl
// and ensuring that it has been correctly applied by reading iso_ctrl_ack
//
    for (i = 0; i < ARRAY_SIZE(sequence); i++) {
    writeb(sequence[i] & ISO_CTRL_MASK, priv.base +  REG_ISO_CTRL);
    ret = read_poll_timeout(read_iso_ctrl_ack, val, val == (i % 2), 100,
    20 * 100, true, priv.base);
    if (ret) {
    dev_dbg(dev, "Timeout waiting for ack byte %i (%x) of sequence\n", i,
    sequence[i]);
    return ret;
    }
    }
//
// At this point iso_en should be raised for 1ms
//
    ret = read_poll_timeout(read_iso_en, val, val, 100, 22 * 100, true, priv.base);
    if (ret)
    dev_dbg(dev, "Timeout waiting for iso_en\n");
    mdelay(2);
    return 0;
    }
    static void ssd202d_rtc_read_reg(struct ssd202d_rtc *priv, unsigned int reg,
    unsigned int field, unsigned int *base)
    {
    unsigned int l, h;
    u16 val;
// Ask for the content of an RTC value into RDDATA by gating iso_en,
// then iso_en is gated and the content of RDDATA can be read
//
    val = readw(priv.base + reg);
    writew(val | field, priv.base + reg);
    ssd202d_rtc_isoctrl(priv);
    writew(val & ~field, priv.base + reg);
    l = readw(priv.base + REG_RDDATA_L);
    h = readw(priv.base + REG_RDDATA_H);
// base = (h << 16) | l;
    }
    static void ssd202d_rtc_write_reg(struct ssd202d_rtc *priv, unsigned int reg,
    unsigned int field, u32 base)
    {
    u16 val;
// Set the content of an RTC value from WRDATA by gating iso_en
    val = readw(priv.base + reg);
    writew(val | field, priv.base + reg);
    writew(base, priv.base + REG_WRDATA_L);
    writew(base >> 16, priv.base + REG_WRDATA_H);
    ssd202d_rtc_isoctrl(priv);
    writew(val & ~field, priv.base + reg);
    }
#[no_mangle]
unsafe extern "C" fn ssd202d_rtc_read_counter(priv: *mut ssd202d_rtc, counter: *mut c_uint) -> c_int {
    static int ssd202d_rtc_read_counter(struct ssd202d_rtc *priv, unsigned int *counter)
    {
    unsigned int l, h;
    u16 val;
    val = readw(priv.base + REG_CTRL1);
    writew(val | CNT_RD_BIT, priv.base + REG_CTRL1);
    ssd202d_rtc_isoctrl(priv);
    writew(val & ~CNT_RD_BIT, priv.base + REG_CTRL1);
    val = readw(priv.base + REG_CTRL1);
    writew(val | CNT_RD_TRIG_BIT, priv.base + REG_CNT_TRIG);
    writew(val & ~CNT_RD_TRIG_BIT, priv.base + REG_CNT_TRIG);
    l = readw(priv.base + REG_RDCNT_L);
    h = readw(priv.base + REG_RDCNT_H);
// counter = (h << 16) | l;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ssd202d_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int ssd202d_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct ssd202d_rtc *priv = dev_get_drvdata(dev);
    unsigned int sw0, base, counter;
    u32 seconds;
    int ret;
// Check that RTC is enabled by SW
    ssd202d_rtc_read_reg(priv, REG_CTRL, SW0_RD_BIT, &sw0);
    if (sw0 != 1)
    return -EINVAL;
// Get RTC base value from RDDATA
    ssd202d_rtc_read_reg(priv, REG_CTRL, BASE_RD_BIT, &base);
// Get RTC counter value from RDDATA
    ret = ssd202d_rtc_read_counter(priv, &counter);
    if (ret)
    return ret;
    seconds = base + counter;
    rtc_time64_to_tm(seconds, tm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ssd202d_rtc_reset_counter(priv: *mut ssd202d_rtc) -> c_int {
    static int ssd202d_rtc_reset_counter(struct ssd202d_rtc *priv)
    {
    u16 val;
    val = readw(priv.base + REG_CTRL);
    writew(val | CNT_RST_BIT, priv.base + REG_CTRL);
    ssd202d_rtc_isoctrl(priv);
    writew(val & ~CNT_RST_BIT, priv.base + REG_CTRL);
    ssd202d_rtc_isoctrl(priv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ssd202d_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int ssd202d_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct ssd202d_rtc *priv = dev_get_drvdata(dev);
    let mut seconds: c_ulong = rtc_tm_to_time64(tm);
    ssd202d_rtc_write_reg(priv, REG_CTRL, BASE_WR_BIT, seconds);
    ssd202d_rtc_reset_counter(priv);
    ssd202d_rtc_write_reg(priv, REG_CTRL, SW0_WR_BIT, 1);
    return 0;
    }
    static const struct rtc_class_ops ssd202d_rtc_ops = {
    .read_time = ssd202d_rtc_read_time,
    .set_time = ssd202d_rtc_set_time,
    };
#[no_mangle]
unsafe extern "C" fn ssd202d_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int ssd202d_rtc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct ssd202d_rtc *priv;
    priv = devm_kzalloc(&pdev.dev, sizeof(struct ssd202d_rtc), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    priv.rtc_dev = devm_rtc_allocate_device(dev);
    if (IS_ERR(priv.rtc_dev))
    return PTR_ERR(priv.rtc_dev);
    priv.rtc_dev.ops = &ssd202d_rtc_ops;
    priv.rtc_dev.range_max = U32_MAX;
    platform_set_drvdata(pdev, priv);
    return devm_rtc_register_device(priv.rtc_dev);
    }
    static const struct of_device_id ssd202d_rtc_of_match_table[] = {
    { .compatible = "mstar,ssd202d-rtc" },
    { }
    };
    MODULE_DEVICE_TABLE(of, ssd202d_rtc_of_match_table);
    static struct platform_driver ssd202d_rtc_driver = {
    .probe = ssd202d_rtc_probe,
    .driver = {
    .name = "ssd202d-rtc",
    .of_match_table = ssd202d_rtc_of_match_table,
    },
    };
    module_platform_driver(ssd202d_rtc_driver);
    MODULE_AUTHOR("Daniel Palmer <daniel@thingy.jp>");
    MODULE_AUTHOR("Romain Perier <romain.perier@gmail.com>");
    MODULE_DESCRIPTION("MStar SSD202D RTC Driver");
    MODULE_LICENSE("GPL");
