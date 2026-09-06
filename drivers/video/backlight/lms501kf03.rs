//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/lms501kf03.c
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
// lms501kf03 TFT LCD panel driver.
//
// Copyright (c) 2012 Samsung Electronics Co., Ltd.
// Author: Jingoo Han  <jg1.han@samsung.com>
//

pub const COMMAND_ONLY: c_uint = 0x00;
pub const DATA_ONLY: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lms501kf03 {
    pub dev: *mut device,
    pub spi: *mut spi_device,
    pub power: c_uint,
    pub ld: *mut lcd_device,
    pub lcd_pd: *mut lcd_platform_data,
}

    static const unsigned char seq_password[] = {
    0xb9, 0xff, 0x83, 0x69,
    };
    static const unsigned char seq_power[] = {
    0xb1, 0x01, 0x00, 0x34, 0x06, 0x00, 0x14, 0x14, 0x20, 0x28,
    0x12, 0x12, 0x17, 0x0a, 0x01, 0xe6, 0xe6, 0xe6, 0xe6, 0xe6,
    };
    static const unsigned char seq_display[] = {
    0xb2, 0x00, 0x2b, 0x03, 0x03, 0x70, 0x00, 0xff, 0x00, 0x00,
    0x00, 0x00, 0x03, 0x03, 0x00, 0x01,
    };
    static const unsigned char seq_rgb_if[] = {
    0xb3, 0x09,
    };
    static const unsigned char seq_display_inv[] = {
    0xb4, 0x01, 0x08, 0x77, 0x0e, 0x06,
    };
    static const unsigned char seq_vcom[] = {
    0xb6, 0x4c, 0x2e,
    };
    static const unsigned char seq_gate[] = {
    0xd5, 0x00, 0x05, 0x03, 0x29, 0x01, 0x07, 0x17, 0x68, 0x13,
    0x37, 0x20, 0x31, 0x8a, 0x46, 0x9b, 0x57, 0x13, 0x02, 0x75,
    0xb9, 0x64, 0xa8, 0x07, 0x0f, 0x04, 0x07,
    };
    static const unsigned char seq_panel[] = {
    0xcc, 0x02,
    };
    static const unsigned char seq_col_mod[] = {
    0x3a, 0x77,
    };
    static const unsigned char seq_w_gamma[] = {
    0xe0, 0x00, 0x04, 0x09, 0x0f, 0x1f, 0x3f, 0x1f, 0x2f, 0x0a,
    0x0f, 0x10, 0x16, 0x18, 0x16, 0x17, 0x0d, 0x15, 0x00, 0x04,
    0x09, 0x0f, 0x38, 0x3f, 0x20, 0x39, 0x0a, 0x0f, 0x10, 0x16,
    0x18, 0x16, 0x17, 0x0d, 0x15,
    };
    static const unsigned char seq_rgb_gamma[] = {
    0xc1, 0x01, 0x03, 0x07, 0x0f, 0x1a, 0x22, 0x2c, 0x33, 0x3c,
    0x46, 0x4f, 0x58, 0x60, 0x69, 0x71, 0x79, 0x82, 0x89, 0x92,
    0x9a, 0xa1, 0xa9, 0xb1, 0xb9, 0xc1, 0xc9, 0xcf, 0xd6, 0xde,
    0xe5, 0xec, 0xf3, 0xf9, 0xff, 0xdd, 0x39, 0x07, 0x1c, 0xcb,
    0xab, 0x5f, 0x49, 0x80, 0x03, 0x07, 0x0f, 0x19, 0x20, 0x2a,
    0x31, 0x39, 0x42, 0x4b, 0x53, 0x5b, 0x63, 0x6b, 0x73, 0x7b,
    0x83, 0x8a, 0x92, 0x9b, 0xa2, 0xaa, 0xb2, 0xba, 0xc2, 0xca,
    0xd0, 0xd8, 0xe1, 0xe8, 0xf0, 0xf8, 0xff, 0xf7, 0xd8, 0xbe,
    0xa7, 0x39, 0x40, 0x85, 0x8c, 0xc0, 0x04, 0x07, 0x0c, 0x17,
    0x1c, 0x23, 0x2b, 0x34, 0x3b, 0x43, 0x4c, 0x54, 0x5b, 0x63,
    0x6a, 0x73, 0x7a, 0x82, 0x8a, 0x91, 0x98, 0xa1, 0xa8, 0xb0,
    0xb7, 0xc1, 0xc9, 0xcf, 0xd9, 0xe3, 0xea, 0xf4, 0xff, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    };
    static const unsigned char seq_sleep_out[] = {
    0x11,
    };
    static const unsigned char seq_display_on[] = {
    0x29,
    };
    static const unsigned char seq_display_off[] = {
    0x10,
    };
#[no_mangle]
unsafe extern "C" fn lms501kf03_spi_write_byte(lcd: *mut lms501kf03, addr: c_int, data: c_int) -> c_int {
    static int lms501kf03_spi_write_byte(struct lms501kf03 *lcd, int addr, int data)
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
    static int lms501kf03_spi_write(struct lms501kf03 *lcd, unsigned char address,
    unsigned char command)
    {
    return lms501kf03_spi_write_byte(lcd, address, command);
    }
    static int lms501kf03_panel_send_sequence(struct lms501kf03 *lcd,
    const unsigned char *wbuf,
    unsigned int len)
    {
    let mut ret: c_int = 0, i = 0;
    while (i < len) {
    if (i == 0)
    ret = lms501kf03_spi_write(lcd, COMMAND_ONLY, wbuf[i]);
    else
    ret = lms501kf03_spi_write(lcd, DATA_ONLY, wbuf[i]);
    if (ret)
    break;
    i += 1;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn lms501kf03_ldi_init(lcd: *mut lms501kf03) -> c_int {
    static int lms501kf03_ldi_init(struct lms501kf03 *lcd)
    {
    int ret, i;
    static const unsigned char *init_seq[] = {
    seq_password,
    seq_power,
    seq_display,
    seq_rgb_if,
    seq_display_inv,
    seq_vcom,
    seq_gate,
    seq_panel,
    seq_col_mod,
    seq_w_gamma,
    seq_rgb_gamma,
    seq_sleep_out,
    };
    static const unsigned int size_seq[] = {
    ARRAY_SIZE(seq_password),
    ARRAY_SIZE(seq_power),
    ARRAY_SIZE(seq_display),
    ARRAY_SIZE(seq_rgb_if),
    ARRAY_SIZE(seq_display_inv),
    ARRAY_SIZE(seq_vcom),
    ARRAY_SIZE(seq_gate),
    ARRAY_SIZE(seq_panel),
    ARRAY_SIZE(seq_col_mod),
    ARRAY_SIZE(seq_w_gamma),
    ARRAY_SIZE(seq_rgb_gamma),
    ARRAY_SIZE(seq_sleep_out),
    };
    for (i = 0; i < ARRAY_SIZE(init_seq); i++) {
    ret = lms501kf03_panel_send_sequence(lcd, init_seq[i],
    size_seq[i]);
    if (ret)
    break;
    }
//
// According to the datasheet, 120ms delay time is required.
// After sleep out sequence, command is blocked for 120ms.
// Thus, LDI should wait for 120ms.
//
    msleep(120);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn lms501kf03_ldi_enable(lcd: *mut lms501kf03) -> c_int {
    static int lms501kf03_ldi_enable(struct lms501kf03 *lcd)
    {
    return lms501kf03_panel_send_sequence(lcd, seq_display_on,
    ARRAY_SIZE(seq_display_on));
    }
#[no_mangle]
unsafe extern "C" fn lms501kf03_ldi_disable(lcd: *mut lms501kf03) -> c_int {
    static int lms501kf03_ldi_disable(struct lms501kf03 *lcd)
    {
    return lms501kf03_panel_send_sequence(lcd, seq_display_off,
    ARRAY_SIZE(seq_display_off));
    }
#[no_mangle]
unsafe extern "C" fn lms501kf03_power_is_on(power: c_int) -> c_int {
    static int lms501kf03_power_is_on(int power)
    {
    return (power) <= LCD_POWER_REDUCED;
    }
#[no_mangle]
unsafe extern "C" fn lms501kf03_power_on(lcd: *mut lms501kf03) -> c_int {
    static int lms501kf03_power_on(struct lms501kf03 *lcd)
    {
    let mut ret: c_int = 0;
    struct lcd_platform_data *pd;
    pd = lcd.lcd_pd;
    if (!pd.power_on) {
    dev_err(lcd.dev, "power_on is core::ptr::null_mut().\n");
    return -EINVAL;
    }
    pd.power_on(lcd.ld, 1);
    msleep(pd.power_on_delay);
    if (!pd.reset) {
    dev_err(lcd.dev, "reset is core::ptr::null_mut().\n");
    return -EINVAL;
    }
    pd.reset(lcd.ld);
    msleep(pd.reset_delay);
    ret = lms501kf03_ldi_init(lcd);
    if (ret) {
    dev_err(lcd.dev, "failed to initialize ldi.\n");
    return ret;
    }
    ret = lms501kf03_ldi_enable(lcd);
    if (ret) {
    dev_err(lcd.dev, "failed to enable ldi.\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lms501kf03_power_off(lcd: *mut lms501kf03) -> c_int {
    static int lms501kf03_power_off(struct lms501kf03 *lcd)
    {
    let mut ret: c_int = 0;
    struct lcd_platform_data *pd;
    pd = lcd.lcd_pd;
    ret = lms501kf03_ldi_disable(lcd);
    if (ret) {
    dev_err(lcd.dev, "lcd setting failed.\n");
    return -EIO;
    }
    msleep(pd.power_off_delay);
    pd.power_on(lcd.ld, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lms501kf03_power(lcd: *mut lms501kf03, power: c_int) -> c_int {
    static int lms501kf03_power(struct lms501kf03 *lcd, int power)
    {
    let mut ret: c_int = 0;
    if (lms501kf03_power_is_on(power) &&
    !lms501kf03_power_is_on(lcd.power))
    ret = lms501kf03_power_on(lcd);
    else if (!lms501kf03_power_is_on(power) &&
    lms501kf03_power_is_on(lcd.power))
    ret = lms501kf03_power_off(lcd);
    if (!ret)
    lcd.power = power;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn lms501kf03_get_power(ld: *mut lcd_device) -> c_int {
    static int lms501kf03_get_power(struct lcd_device *ld)
    {
    struct lms501kf03 *lcd = lcd_get_data(ld);
    return lcd.power;
    }
#[no_mangle]
unsafe extern "C" fn lms501kf03_set_power(ld: *mut lcd_device, power: c_int) -> c_int {
    static int lms501kf03_set_power(struct lcd_device *ld, int power)
    {
    struct lms501kf03 *lcd = lcd_get_data(ld);
    if (power != LCD_POWER_ON && power != LCD_POWER_OFF &&
    power != LCD_POWER_REDUCED) {
    dev_err(lcd.dev, "power value should be 0, 1 or 4.\n");
    return -EINVAL;
    }
    return lms501kf03_power(lcd, power);
    }
    static const struct lcd_ops lms501kf03_lcd_ops = {
    .get_power = lms501kf03_get_power,
    .set_power = lms501kf03_set_power,
    };
#[no_mangle]
unsafe extern "C" fn lms501kf03_probe(spi: *mut spi_device) -> c_int {
    static int lms501kf03_probe(struct spi_device *spi)
    {
    struct lms501kf03 *lcd = core::ptr::null_mut();
    struct lcd_device *ld = core::ptr::null_mut();
    let mut ret: c_int = 0;
    lcd = devm_kzalloc(&spi.dev, sizeof(struct lms501kf03), GFP_KERNEL);
    if (!lcd)
    return -ENOMEM;
// lms501kf03 lcd panel uses 3-wire 9-bit SPI Mode.
    spi.bits_per_word = 9;
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
    ld = devm_lcd_device_register(&spi.dev, "lms501kf03", &spi.dev, lcd,
    &lms501kf03_lcd_ops);
    if (IS_ERR(ld))
    return PTR_ERR(ld);
    lcd.ld = ld;
    if (!lcd.lcd_pd.lcd_enabled) {
//
// if lcd panel was off from bootloader then
// current lcd status is powerdown and then
// it enables lcd panel.
//
    lcd.power = LCD_POWER_OFF;
    lms501kf03_power(lcd, LCD_POWER_ON);
    } else {
    lcd.power = LCD_POWER_ON;
    }
    spi_set_drvdata(spi, lcd);
    dev_info(&spi.dev, "lms501kf03 panel driver has been probed.\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lms501kf03_remove(spi: *mut spi_device) {
    static void lms501kf03_remove(struct spi_device *spi)
    {
    struct lms501kf03 *lcd = spi_get_drvdata(spi);
    lms501kf03_power(lcd, LCD_POWER_OFF);
    }

#[no_mangle]
unsafe extern "C" fn lms501kf03_suspend(dev: *mut device) -> c_int {
    static int lms501kf03_suspend(struct device *dev)
    {
    struct lms501kf03 *lcd = dev_get_drvdata(dev);
    dev_dbg(dev, "lcd.power = %d\n", lcd.power);
//
// when lcd panel is suspend, lcd panel becomes off
// regardless of status.
//
    return lms501kf03_power(lcd, LCD_POWER_OFF);
    }
#[no_mangle]
unsafe extern "C" fn lms501kf03_resume(dev: *mut device) -> c_int {
    static int lms501kf03_resume(struct device *dev)
    {
    struct lms501kf03 *lcd = dev_get_drvdata(dev);
    lcd.power = LCD_POWER_OFF;
    return lms501kf03_power(lcd, LCD_POWER_ON);
    }

    static SIMPLE_DEV_PM_OPS(lms501kf03_pm_ops, lms501kf03_suspend,
    lms501kf03_resume);
#[no_mangle]
unsafe extern "C" fn lms501kf03_shutdown(spi: *mut spi_device) {
    static void lms501kf03_shutdown(struct spi_device *spi)
    {
    struct lms501kf03 *lcd = spi_get_drvdata(spi);
    lms501kf03_power(lcd, LCD_POWER_OFF);
    }
    static struct spi_driver lms501kf03_driver = {
    .driver = {
    .name	= "lms501kf03",
    .pm	= &lms501kf03_pm_ops,
    },
    .probe		= lms501kf03_probe,
    .remove		= lms501kf03_remove,
    .shutdown	= lms501kf03_shutdown,
    };
    module_spi_driver(lms501kf03_driver);
    MODULE_AUTHOR("Jingoo Han <jg1.han@samsung.com>");
    MODULE_DESCRIPTION("lms501kf03 LCD Driver");
    MODULE_LICENSE("GPL");
