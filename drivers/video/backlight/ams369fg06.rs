//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/ams369fg06.c
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
// ams369fg06 AMOLED LCD panel driver.
//
// Copyright (c) 2011 Samsung Electronics Co., Ltd.
// Author: Jingoo Han  <jg1.han@samsung.com>
//
// Derived from drivers/video/s6e63m0.c
//

pub const SLEEPMSEC: c_uint = 0x1000;
pub const ENDDEF: c_uint = 0x2000;
pub const DEFMASK: c_uint = 0xFF00;
pub const COMMAND_ONLY: c_uint = 0xFE;
pub const DATA_ONLY: c_uint = 0xFF;
pub const MAX_GAMMA_LEVEL: c_int = 5;
pub const GAMMA_TABLE_COUNT: c_int = 21;
pub const MIN_BRIGHTNESS: c_int = 0;
pub const MAX_BRIGHTNESS: c_int = 255;
pub const DEFAULT_BRIGHTNESS: c_int = 150;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ams369fg06 {
    pub dev: *mut device,
    pub spi: *mut spi_device,
    pub power: c_uint,
    pub ld: *mut lcd_device,
    pub bd: *mut backlight_device,
    pub lcd_pd: *mut lcd_platform_data,
}

    static const unsigned short seq_display_on[] = {
    0x14, 0x03,
    ENDDEF, 0x0000
    };
    static const unsigned short seq_display_off[] = {
    0x14, 0x00,
    ENDDEF, 0x0000
    };
    static const unsigned short seq_stand_by_on[] = {
    0x1D, 0xA1,
    SLEEPMSEC, 200,
    ENDDEF, 0x0000
    };
    static const unsigned short seq_stand_by_off[] = {
    0x1D, 0xA0,
    SLEEPMSEC, 250,
    ENDDEF, 0x0000
    };
    static const unsigned short seq_setting[] = {
    0x31, 0x08,
    0x32, 0x14,
    0x30, 0x02,
    0x27, 0x01,
    0x12, 0x08,
    0x13, 0x08,
    0x15, 0x00,
    0x16, 0x00,
    0xef, 0xd0,
    DATA_ONLY, 0xe8,
    0x39, 0x44,
    0x40, 0x00,
    0x41, 0x3f,
    0x42, 0x2a,
    0x43, 0x27,
    0x44, 0x27,
    0x45, 0x1f,
    0x46, 0x44,
    0x50, 0x00,
    0x51, 0x00,
    0x52, 0x17,
    0x53, 0x24,
    0x54, 0x26,
    0x55, 0x1f,
    0x56, 0x43,
    0x60, 0x00,
    0x61, 0x3f,
    0x62, 0x2a,
    0x63, 0x25,
    0x64, 0x24,
    0x65, 0x1b,
    0x66, 0x5c,
    0x17, 0x22,
    0x18, 0x33,
    0x19, 0x03,
    0x1a, 0x01,
    0x22, 0xa4,
    0x23, 0x00,
    0x26, 0xa0,
    0x1d, 0xa0,
    SLEEPMSEC, 300,
    0x14, 0x03,
    ENDDEF, 0x0000
    };
// gamma value: 2.2
    static const unsigned int ams369fg06_22_250[] = {
    0x00, 0x3f, 0x2a, 0x27, 0x27, 0x1f, 0x44,
    0x00, 0x00, 0x17, 0x24, 0x26, 0x1f, 0x43,
    0x00, 0x3f, 0x2a, 0x25, 0x24, 0x1b, 0x5c,
    };
    static const unsigned int ams369fg06_22_200[] = {
    0x00, 0x3f, 0x28, 0x29, 0x27, 0x21, 0x3e,
    0x00, 0x00, 0x10, 0x25, 0x27, 0x20, 0x3d,
    0x00, 0x3f, 0x28, 0x27, 0x25, 0x1d, 0x53,
    };
    static const unsigned int ams369fg06_22_150[] = {
    0x00, 0x3f, 0x2d, 0x29, 0x28, 0x23, 0x37,
    0x00, 0x00, 0x0b, 0x25, 0x28, 0x22, 0x36,
    0x00, 0x3f, 0x2b, 0x28, 0x26, 0x1f, 0x4a,
    };
    static const unsigned int ams369fg06_22_100[] = {
    0x00, 0x3f, 0x30, 0x2a, 0x2b, 0x24, 0x2f,
    0x00, 0x00, 0x00, 0x25, 0x29, 0x24, 0x2e,
    0x00, 0x3f, 0x2f, 0x29, 0x29, 0x21, 0x3f,
    };
    static const unsigned int ams369fg06_22_50[] = {
    0x00, 0x3f, 0x3c, 0x2c, 0x2d, 0x27, 0x24,
    0x00, 0x00, 0x00, 0x22, 0x2a, 0x27, 0x23,
    0x00, 0x3f, 0x3b, 0x2c, 0x2b, 0x24, 0x31,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ams369fg06_gamma {
    pub gamma_22_table: [*mut c_uint; MAX_GAMMA_LEVEL],
}

    static struct ams369fg06_gamma gamma_table = {
    .gamma_22_table[0] = (unsigned int *)&ams369fg06_22_50,
    .gamma_22_table[1] = (unsigned int *)&ams369fg06_22_100,
    .gamma_22_table[2] = (unsigned int *)&ams369fg06_22_150,
    .gamma_22_table[3] = (unsigned int *)&ams369fg06_22_200,
    .gamma_22_table[4] = (unsigned int *)&ams369fg06_22_250,
    };
#[no_mangle]
unsafe extern "C" fn ams369fg06_spi_write_byte(lcd: *mut ams369fg06, addr: c_int, data: c_int) -> c_int {
    static int ams369fg06_spi_write_byte(struct ams369fg06 *lcd, int addr, int data)
    {
    u16 buf[1];
    struct spi_message msg;
    struct spi_transfer xfer = {
    .len		= 2,
    .tx_buf		= buf,
    };
    buf[0] = (addr << 8) | data;
    spi_message_init(&msg);
    spi_message_add_tail(&xfer, &msg);
    return spi_sync(lcd.spi, &msg);
    }
    static int ams369fg06_spi_write(struct ams369fg06 *lcd, unsigned char address,
    unsigned char command)
    {
    let mut ret: c_int = 0;
    if (address != DATA_ONLY)
    ret = ams369fg06_spi_write_byte(lcd, 0x70, address);
    if (command != COMMAND_ONLY)
    ret = ams369fg06_spi_write_byte(lcd, 0x72, command);
    return ret;
    }
    static int ams369fg06_panel_send_sequence(struct ams369fg06 *lcd,
    const unsigned short *wbuf)
    {
    let mut ret: c_int = 0, i = 0;
    while ((wbuf[i] & DEFMASK) != ENDDEF) {
    if ((wbuf[i] & DEFMASK) != SLEEPMSEC) {
    ret = ams369fg06_spi_write(lcd, wbuf[i], wbuf[i+1]);
    if (ret)
    break;
    } else {
    msleep(wbuf[i+1]);
    }
    i += 2;
    }
    return ret;
    }
    static int _ams369fg06_gamma_ctl(struct ams369fg06 *lcd,
    const unsigned int *gamma)
    {
    let mut i: c_uint = 0;
    let mut ret: c_int = 0;
    for (i = 0 ; i < GAMMA_TABLE_COUNT / 3; i++) {
    ret = ams369fg06_spi_write(lcd, 0x40 + i, gamma[i]);
    ret = ams369fg06_spi_write(lcd, 0x50 + i, gamma[i+7*1]);
    ret = ams369fg06_spi_write(lcd, 0x60 + i, gamma[i+7*2]);
    if (ret) {
    dev_err(lcd.dev, "failed to set gamma table.\n");
    goto gamma_err;
    }
    }
    gamma_err:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ams369fg06_gamma_ctl(lcd: *mut ams369fg06, brightness: c_int) -> c_int {
    static int ams369fg06_gamma_ctl(struct ams369fg06 *lcd, int brightness)
    {
    let mut ret: c_int = 0;
    let mut gamma: c_int = 0;
    if ((brightness >= 0) && (brightness <= 50))
    gamma = 0;
#[no_mangle]
pub unsafe extern "C" fn if(100): (brightness > 50) && (brightness <=) -> else {
    else if ((brightness > 50) && (brightness <= 100))
    gamma = 1;
#[no_mangle]
pub unsafe extern "C" fn if(150): (brightness > 100) && (brightness <=) -> else {
    else if ((brightness > 100) && (brightness <= 150))
    gamma = 2;
#[no_mangle]
pub unsafe extern "C" fn if(200): (brightness > 150) && (brightness <=) -> else {
    else if ((brightness > 150) && (brightness <= 200))
    gamma = 3;
#[no_mangle]
pub unsafe extern "C" fn if(255): (brightness > 200) && (brightness <=) -> else {
    else if ((brightness > 200) && (brightness <= 255))
    gamma = 4;
    ret = _ams369fg06_gamma_ctl(lcd, gamma_table.gamma_22_table[gamma]);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ams369fg06_ldi_init(lcd: *mut ams369fg06) -> c_int {
    static int ams369fg06_ldi_init(struct ams369fg06 *lcd)
    {
    int ret, i;
    static const unsigned short *init_seq[] = {
    seq_setting,
    seq_stand_by_off,
    };
    for (i = 0; i < ARRAY_SIZE(init_seq); i++) {
    ret = ams369fg06_panel_send_sequence(lcd, init_seq[i]);
    if (ret)
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ams369fg06_ldi_enable(lcd: *mut ams369fg06) -> c_int {
    static int ams369fg06_ldi_enable(struct ams369fg06 *lcd)
    {
    int ret, i;
    static const unsigned short *init_seq[] = {
    seq_stand_by_off,
    seq_display_on,
    };
    for (i = 0; i < ARRAY_SIZE(init_seq); i++) {
    ret = ams369fg06_panel_send_sequence(lcd, init_seq[i]);
    if (ret)
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ams369fg06_ldi_disable(lcd: *mut ams369fg06) -> c_int {
    static int ams369fg06_ldi_disable(struct ams369fg06 *lcd)
    {
    int ret, i;
    static const unsigned short *init_seq[] = {
    seq_display_off,
    seq_stand_by_on,
    };
    for (i = 0; i < ARRAY_SIZE(init_seq); i++) {
    ret = ams369fg06_panel_send_sequence(lcd, init_seq[i]);
    if (ret)
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ams369fg06_power_is_on(power: c_int) -> c_int {
    static int ams369fg06_power_is_on(int power)
    {
    return power <= BACKLIGHT_POWER_REDUCED;
    }
#[no_mangle]
unsafe extern "C" fn ams369fg06_power_on(lcd: *mut ams369fg06) -> c_int {
    static int ams369fg06_power_on(struct ams369fg06 *lcd)
    {
    let mut ret: c_int = 0;
    struct lcd_platform_data *pd;
    struct backlight_device *bd;
    pd = lcd.lcd_pd;
    bd = lcd.bd;
    if (pd.power_on) {
    pd.power_on(lcd.ld, 1);
    msleep(pd.power_on_delay);
    }
    if (!pd.reset) {
    dev_err(lcd.dev, "reset is core::ptr::null_mut().\n");
    return -EINVAL;
    }
    pd.reset(lcd.ld);
    msleep(pd.reset_delay);
    ret = ams369fg06_ldi_init(lcd);
    if (ret) {
    dev_err(lcd.dev, "failed to initialize ldi.\n");
    return ret;
    }
    ret = ams369fg06_ldi_enable(lcd);
    if (ret) {
    dev_err(lcd.dev, "failed to enable ldi.\n");
    return ret;
    }
// set brightness to current value after power on or resume.
    ret = ams369fg06_gamma_ctl(lcd, bd.props.brightness);
    if (ret) {
    dev_err(lcd.dev, "lcd gamma setting failed.\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ams369fg06_power_off(lcd: *mut ams369fg06) -> c_int {
    static int ams369fg06_power_off(struct ams369fg06 *lcd)
    {
    int ret;
    struct lcd_platform_data *pd;
    pd = lcd.lcd_pd;
    ret = ams369fg06_ldi_disable(lcd);
    if (ret) {
    dev_err(lcd.dev, "lcd setting failed.\n");
    return -EIO;
    }
    msleep(pd.power_off_delay);
    if (pd.power_on)
    pd.power_on(lcd.ld, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ams369fg06_power(lcd: *mut ams369fg06, power: c_int) -> c_int {
    static int ams369fg06_power(struct ams369fg06 *lcd, int power)
    {
    let mut ret: c_int = 0;
    if (ams369fg06_power_is_on(power) &&
    !ams369fg06_power_is_on(lcd.power))
    ret = ams369fg06_power_on(lcd);
    else if (!ams369fg06_power_is_on(power) &&
    ams369fg06_power_is_on(lcd.power))
    ret = ams369fg06_power_off(lcd);
    if (!ret)
    lcd.power = power;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ams369fg06_get_power(ld: *mut lcd_device) -> c_int {
    static int ams369fg06_get_power(struct lcd_device *ld)
    {
    struct ams369fg06 *lcd = lcd_get_data(ld);
    return lcd.power;
    }
#[no_mangle]
unsafe extern "C" fn ams369fg06_set_power(ld: *mut lcd_device, power: c_int) -> c_int {
    static int ams369fg06_set_power(struct lcd_device *ld, int power)
    {
    struct ams369fg06 *lcd = lcd_get_data(ld);
    if (power != BACKLIGHT_POWER_ON && power != BACKLIGHT_POWER_OFF &&
    power != BACKLIGHT_POWER_REDUCED) {
    dev_err(lcd.dev, "power value should be 0, 1 or 4.\n");
    return -EINVAL;
    }
    return ams369fg06_power(lcd, power);
    }
#[no_mangle]
unsafe extern "C" fn ams369fg06_set_brightness(bd: *mut backlight_device) -> c_int {
    static int ams369fg06_set_brightness(struct backlight_device *bd)
    {
    let mut ret: c_int = 0;
    let mut brightness: c_int = bd.props.brightness;
    struct ams369fg06 *lcd = bl_get_data(bd);
    if (brightness < MIN_BRIGHTNESS ||
    brightness > bd.props.max_brightness) {
    dev_err(&bd.dev, "lcd brightness should be %d to %d.\n",
    MIN_BRIGHTNESS, MAX_BRIGHTNESS);
    return -EINVAL;
    }
    ret = ams369fg06_gamma_ctl(lcd, bd.props.brightness);
    if (ret) {
    dev_err(&bd.dev, "lcd brightness setting failed.\n");
    return -EIO;
    }
    return ret;
    }
    static const struct lcd_ops ams369fg06_lcd_ops = {
    .get_power = ams369fg06_get_power,
    .set_power = ams369fg06_set_power,
    };
    static const struct backlight_ops ams369fg06_backlight_ops = {
    .update_status = ams369fg06_set_brightness,
    };
#[no_mangle]
unsafe extern "C" fn ams369fg06_probe(spi: *mut spi_device) -> c_int {
    static int ams369fg06_probe(struct spi_device *spi)
    {
    let mut ret: c_int = 0;
    struct ams369fg06 *lcd = core::ptr::null_mut();
    struct lcd_device *ld = core::ptr::null_mut();
    struct backlight_device *bd = core::ptr::null_mut();
    struct backlight_properties props;
    lcd = devm_kzalloc(&spi.dev, sizeof(struct ams369fg06), GFP_KERNEL);
    if (!lcd)
    return -ENOMEM;
// ams369fg06 lcd panel uses 3-wire 16bits SPI Mode.
    spi.bits_per_word = 16;
    ret = spi_setup(spi);
    if (ret < 0) {
    dev_err(&spi.dev, "spi setup failed.\n");
    return ret;
    }
    lcd.spi = spi;
    lcd.dev = &spi.dev;
    lcd.lcd_pd = dev_get_platdata(&spi.dev);
    if (!lcd.lcd_pd) {
    dev_err(&spi.dev, "platform data is core::ptr::null_mut()\n");
    return -EINVAL;
    }
    ld = devm_lcd_device_register(&spi.dev, "ams369fg06", &spi.dev, lcd,
    &ams369fg06_lcd_ops);
    if (IS_ERR(ld))
    return PTR_ERR(ld);
    lcd.ld = ld;
    memset(&props, 0, sizeof(struct backlight_properties));
    props.type = BACKLIGHT_RAW;
    props.max_brightness = MAX_BRIGHTNESS;
    bd = devm_backlight_device_register(&spi.dev, "ams369fg06-bl",
    &spi.dev, lcd,
    &ams369fg06_backlight_ops, &props);
    if (IS_ERR(bd))
    return PTR_ERR(bd);
    bd.props.brightness = DEFAULT_BRIGHTNESS;
    lcd.bd = bd;
    if (!lcd.lcd_pd.lcd_enabled) {
//
// if lcd panel was off from bootloader then
// current lcd status is powerdown and then
// it enables lcd panel.
//
    lcd.power = BACKLIGHT_POWER_OFF;
    ams369fg06_power(lcd, BACKLIGHT_POWER_ON);
    } else {
    lcd.power = BACKLIGHT_POWER_ON;
    }
    spi_set_drvdata(spi, lcd);
    dev_info(&spi.dev, "ams369fg06 panel driver has been probed.\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ams369fg06_remove(spi: *mut spi_device) {
    static void ams369fg06_remove(struct spi_device *spi)
    {
    struct ams369fg06 *lcd = spi_get_drvdata(spi);
    ams369fg06_power(lcd, BACKLIGHT_POWER_OFF);
    }

#[no_mangle]
unsafe extern "C" fn ams369fg06_suspend(dev: *mut device) -> c_int {
    static int ams369fg06_suspend(struct device *dev)
    {
    struct ams369fg06 *lcd = dev_get_drvdata(dev);
    dev_dbg(dev, "lcd.power = %d\n", lcd.power);
//
// when lcd panel is suspend, lcd panel becomes off
// regardless of status.
//
    return ams369fg06_power(lcd, BACKLIGHT_POWER_OFF);
    }
#[no_mangle]
unsafe extern "C" fn ams369fg06_resume(dev: *mut device) -> c_int {
    static int ams369fg06_resume(struct device *dev)
    {
    struct ams369fg06 *lcd = dev_get_drvdata(dev);
    lcd.power = BACKLIGHT_POWER_OFF;
    return ams369fg06_power(lcd, BACKLIGHT_POWER_ON);
    }

    static SIMPLE_DEV_PM_OPS(ams369fg06_pm_ops, ams369fg06_suspend,
    ams369fg06_resume);
#[no_mangle]
unsafe extern "C" fn ams369fg06_shutdown(spi: *mut spi_device) {
    static void ams369fg06_shutdown(struct spi_device *spi)
    {
    struct ams369fg06 *lcd = spi_get_drvdata(spi);
    ams369fg06_power(lcd, BACKLIGHT_POWER_OFF);
    }
    static struct spi_driver ams369fg06_driver = {
    .driver = {
    .name	= "ams369fg06",
    .pm	= &ams369fg06_pm_ops,
    },
    .probe		= ams369fg06_probe,
    .remove		= ams369fg06_remove,
    .shutdown	= ams369fg06_shutdown,
    };
    module_spi_driver(ams369fg06_driver);
    MODULE_AUTHOR("Jingoo Han <jg1.han@samsung.com>");
    MODULE_DESCRIPTION("ams369fg06 LCD Driver");
    MODULE_LICENSE("GPL");
