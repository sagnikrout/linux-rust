//! Automatically rewritten from C to Rust
//! Source: arch/x86/platform/ts5500/ts5500.c
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
// Technologic Systems TS-5500 Single Board Computer support
//
// Copyright (C) 2013-2014 Savoir-faire Linux Inc.
// Vivien Didelot <vivien.didelot@savoirfairelinux.com>
//
// This driver registers the Technologic Systems TS-5500 Single Board Computer
// (SBC) and its devices, and exposes information to userspace such as jumpers'
// state or available options. For further information about sysfs entries, see
// Documentation/ABI/testing/sysfs-platform-ts5500.
//
// This code may be extended to support similar x86-based platforms.
// Actually, the TS-5500 and TS-5400 are supported.
//

// Product code register
pub const TS5500_PRODUCT_CODE_ADDR: c_uint = 0x74;
pub const TS5500_PRODUCT_CODE: c_uint = 0x60	/* TS-5500 product code */;
pub const TS5400_PRODUCT_CODE: c_uint = 0x40	/* TS-5400 product code */;
// SRAM/RS-485/ADC options, and RS-485 RTS/Automatic RS-485 flags register
pub const TS5500_SRAM_RS485_ADC_ADDR: c_uint = 0x75;

// External Reset/Industrial Temperature Range options register
pub const TS5500_ERESET_ITR_ADDR: c_uint = 0x76;

// LED/Jumpers register
pub const TS5500_LED_JP_ADDR: c_uint = 0x77;

// A/D Converter registers
pub const TS5500_ADC_CONV_BUSY_ADDR: c_uint = 0x195	/* Conversion state register */;

pub const TS5500_ADC_CONV_INIT_LSB_ADDR: c_uint = 0x196	/* Start conv. / LSB register */;
pub const TS5500_ADC_CONV_MSB_ADDR: c_uint = 0x197	/* MSB register */;

//
// struct ts5500_sbc - TS-5500 board description
// @name:	Board model name.
// @id:		Board product ID.
// @sram:	Flag for SRAM option.
// @rs485:	Flag for RS-485 option.
// @adc:	Flag for Analog/Digital converter option.
// @ereset:	Flag for External Reset option.
// @itr:	Flag for Industrial Temperature Range option.
// @jumpers:	Bitfield for jumpers' state.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ts5500_sbc {
    pub name: *const c_char,
    pub id: c_int,
    pub sram: bool,
    pub rs485: bool,
    pub adc: bool,
    pub ereset: bool,
    pub itr: bool,
    pub jumpers: u8,
}

// Board signatures in BIOS shadow RAM
    static const struct {
    const char * const string;
    const ssize_t offset;
    } ts5500_signatures[] __initconst = {
    { "TS-5x00 AMD Elan", 0xb14 },
    };
#[no_mangle]
unsafe extern "C" fn ts5500_check_signature() -> int __init {
    static int __init ts5500_check_signature(void)
    {
    void __iomem *bios;
    int i, ret = -ENODEV;
    bios = ioremap(0xf0000, 0x10000);
    if (!bios)
    return -ENOMEM;
    for (i = 0; i < ARRAY_SIZE(ts5500_signatures); i++) {
    if (check_signature(bios + ts5500_signatures[i].offset,
    ts5500_signatures[i].string,
    strlen(ts5500_signatures[i].string))) {
    ret = 0;
    break;
    }
    }
    iounmap(bios);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ts5500_detect_config(sbc: *mut ts5500_sbc) -> int __init {
    static int __init ts5500_detect_config(struct ts5500_sbc *sbc)
    {
    u8 tmp;
    let mut ret: c_int = 0;
    if (!request_region(TS5500_PRODUCT_CODE_ADDR, 4, "ts5500"))
    return -EBUSY;
    sbc.id = inb(TS5500_PRODUCT_CODE_ADDR);
    if (sbc.id == TS5500_PRODUCT_CODE) {
    sbc.name = "TS-5500";
    } else if (sbc.id == TS5400_PRODUCT_CODE) {
    sbc.name = "TS-5400";
    } else {
    pr_err("ts5500: unknown product code 0x%x\n", sbc.id);
    ret = -ENODEV;
    goto cleanup;
    }
    tmp = inb(TS5500_SRAM_RS485_ADC_ADDR);
    sbc.sram = tmp & TS5500_SRAM;
    sbc.rs485 = tmp & TS5500_RS485;
    sbc.adc = tmp & TS5500_ADC;
    tmp = inb(TS5500_ERESET_ITR_ADDR);
    sbc.ereset = tmp & TS5500_ERESET;
    sbc.itr = tmp & TS5500_ITR;
    tmp = inb(TS5500_LED_JP_ADDR);
    sbc.jumpers = tmp & ~TS5500_LED;
    cleanup:
    release_region(TS5500_PRODUCT_CODE_ADDR, 4);
    return ret;
    }
    static ssize_t name_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct ts5500_sbc *sbc = dev_get_drvdata(dev);
    return sprintf(buf, "%s\n", sbc.name);
    }
    static DEVICE_ATTR_RO(name);
    static ssize_t id_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct ts5500_sbc *sbc = dev_get_drvdata(dev);
    return sprintf(buf, "0x%.2x\n", sbc.id);
    }
    static DEVICE_ATTR_RO(id);
    static ssize_t jumpers_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct ts5500_sbc *sbc = dev_get_drvdata(dev);
    return sprintf(buf, "0x%.2x\n", sbc.jumpers >> 1);
    }
    static DEVICE_ATTR_RO(jumpers);

    static ssize_t _field##_show(struct device *dev,		\
    struct device_attribute *attr, char *buf)	\
    {								\
    struct ts5500_sbc *sbc = dev_get_drvdata(dev);		\
    \
    return sprintf(buf, "%d\n", sbc._field);		\
    }								\
#[no_mangle]
pub unsafe extern "C" fn DEVICE_ATTR_RO(_arg: _field) -> static {
    static DEVICE_ATTR_RO(_field)
    TS5500_ATTR_BOOL(sram);
    TS5500_ATTR_BOOL(rs485);
    TS5500_ATTR_BOOL(adc);
    TS5500_ATTR_BOOL(ereset);
    TS5500_ATTR_BOOL(itr);
    static struct attribute *ts5500_attributes[] = {
    &dev_attr_id.attr,
    &dev_attr_name.attr,
    &dev_attr_jumpers.attr,
    &dev_attr_sram.attr,
    &dev_attr_rs485.attr,
    &dev_attr_adc.attr,
    &dev_attr_ereset.attr,
    &dev_attr_itr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group ts5500_attr_group = {
    .attrs = ts5500_attributes,
    };
    static struct resource ts5500_dio1_resource[] = {
    DEFINE_RES_IRQ_NAMED(7, "DIO1 interrupt"),
    };
    static struct platform_device ts5500_dio1_pdev = {
    .name = "ts5500-dio1",
    .id = -1,
    .resource = ts5500_dio1_resource,
    .num_resources = 1,
    };
    static struct resource ts5500_dio2_resource[] = {
    DEFINE_RES_IRQ_NAMED(6, "DIO2 interrupt"),
    };
    static struct platform_device ts5500_dio2_pdev = {
    .name = "ts5500-dio2",
    .id = -1,
    .resource = ts5500_dio2_resource,
    .num_resources = 1,
    };
    static void ts5500_led_set(struct led_classdev *led_cdev,
    enum led_brightness brightness)
    {
    outb(!!brightness, TS5500_LED_JP_ADDR);
    }
#[no_mangle]
unsafe extern "C" fn ts5500_led_get(led_cdev: *mut led_classdev) -> enum led_brightness {
    static enum led_brightness ts5500_led_get(struct led_classdev *led_cdev)
    {
    return (inb(TS5500_LED_JP_ADDR) & TS5500_LED) ? LED_FULL : LED_OFF;
    }
    static struct led_classdev ts5500_led_cdev = {
    .name = "ts5500:green:",
    .brightness_set = ts5500_led_set,
    .brightness_get = ts5500_led_get,
    };
#[no_mangle]
unsafe extern "C" fn ts5500_adc_convert(ctrl: u8) -> c_int {
    static int ts5500_adc_convert(u8 ctrl)
    {
    u8 lsb, msb;
// Start conversion (ensure the 3 MSB are set to 0)
    outb(ctrl & 0x1f, TS5500_ADC_CONV_INIT_LSB_ADDR);
//
// The platform has CPLD logic driving the A/D converter.
// The conversion must complete within 11 microseconds,
// otherwise we have to re-initiate a conversion.
//
    udelay(TS5500_ADC_CONV_DELAY);
    if (inb(TS5500_ADC_CONV_BUSY_ADDR) & TS5500_ADC_CONV_BUSY)
    return -EBUSY;
// Read the raw data
    lsb = inb(TS5500_ADC_CONV_INIT_LSB_ADDR);
    msb = inb(TS5500_ADC_CONV_MSB_ADDR);
    return (msb << 8) | lsb;
    }
    static struct max197_platform_data ts5500_adc_pdata = {
    .convert = ts5500_adc_convert,
    };
    static struct platform_device ts5500_adc_pdev = {
    .name = "max197",
    .id = -1,
    .dev = {
    .platform_data = &ts5500_adc_pdata,
    },
    };
#[no_mangle]
unsafe extern "C" fn ts5500_init() -> int __init {
    static int __init ts5500_init(void)
    {
    struct platform_device *pdev;
    struct ts5500_sbc *sbc;
    int err;
//
// There is no DMI available or PCI bridge subvendor info,
// only the BIOS provides a 16-bit identification call.
// It is safer to find a signature in the BIOS shadow RAM.
//
    err = ts5500_check_signature();
    if (err)
    return err;
    pdev = platform_device_register_simple("ts5500", -1, core::ptr::null_mut(), 0);
    if (IS_ERR(pdev))
    return PTR_ERR(pdev);
    sbc = devm_kzalloc(&pdev.dev, sizeof(struct ts5500_sbc), GFP_KERNEL);
    if (!sbc) {
    err = -ENOMEM;
    goto error;
    }
    err = ts5500_detect_config(sbc);
    if (err)
    goto error;
    platform_set_drvdata(pdev, sbc);
    err = sysfs_create_group(&pdev.dev.kobj, &ts5500_attr_group);
    if (err)
    goto error;
    if (sbc.id == TS5500_PRODUCT_CODE) {
    ts5500_dio1_pdev.dev.parent = &pdev.dev;
    if (platform_device_register(&ts5500_dio1_pdev))
    dev_warn(&pdev.dev, "DIO1 block registration failed\n");
    ts5500_dio2_pdev.dev.parent = &pdev.dev;
    if (platform_device_register(&ts5500_dio2_pdev))
    dev_warn(&pdev.dev, "DIO2 block registration failed\n");
    }
    if (led_classdev_register(&pdev.dev, &ts5500_led_cdev))
    dev_warn(&pdev.dev, "LED registration failed\n");
    if (sbc.adc) {
    ts5500_adc_pdev.dev.parent = &pdev.dev;
    if (platform_device_register(&ts5500_adc_pdev))
    dev_warn(&pdev.dev, "ADC registration failed\n");
    }
    return 0;
    error:
    platform_device_unregister(pdev);
    return err;
    }
    device_initcall(ts5500_init);
