//! Automatically rewritten from C to Rust
//! Source: drivers/auxdisplay/lcd2s.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Console driver for LCD2S 4x20 character displays connected through i2c.
// The display also has a SPI interface, but the driver does not support
// this yet.
//
// This is a driver allowing you to use a LCD2S 4x20 from Modtronix
// engineering as auxdisplay character device.
//
// (C) 2019 by Lemonage Software GmbH
// Author: Lars Pöschel <poeschel@lemonage.de>
// All rights reserved.
//

pub const LCD2S_CMD_CUR_MOVES_FWD: c_uint = 0x09;
pub const LCD2S_CMD_CUR_BLINK_OFF: c_uint = 0x10;
pub const LCD2S_CMD_CUR_UL_OFF: c_uint = 0x11;
pub const LCD2S_CMD_DISPLAY_OFF: c_uint = 0x12;
pub const LCD2S_CMD_CUR_BLINK_ON: c_uint = 0x18;
pub const LCD2S_CMD_CUR_UL_ON: c_uint = 0x19;
pub const LCD2S_CMD_DISPLAY_ON: c_uint = 0x1a;
pub const LCD2S_CMD_BACKLIGHT_OFF: c_uint = 0x20;
pub const LCD2S_CMD_BACKLIGHT_ON: c_uint = 0x28;
pub const LCD2S_CMD_WRITE: c_uint = 0x80;
pub const LCD2S_CMD_MOV_CUR_RIGHT: c_uint = 0x83;
pub const LCD2S_CMD_MOV_CUR_LEFT: c_uint = 0x84;
pub const LCD2S_CMD_SHIFT_RIGHT: c_uint = 0x85;
pub const LCD2S_CMD_SHIFT_LEFT: c_uint = 0x86;
pub const LCD2S_CMD_SHIFT_UP: c_uint = 0x87;
pub const LCD2S_CMD_SHIFT_DOWN: c_uint = 0x88;
pub const LCD2S_CMD_CUR_ADDR: c_uint = 0x89;
pub const LCD2S_CMD_CUR_POS: c_uint = 0x8a;
pub const LCD2S_CMD_CUR_RESET: c_uint = 0x8b;
pub const LCD2S_CMD_CLEAR: c_uint = 0x8c;
pub const LCD2S_CMD_DEF_CUSTOM_CHAR: c_uint = 0x92;
pub const LCD2S_CMD_READ_STATUS: c_uint = 0xd0;
pub const LCD2S_CHARACTER_SIZE: c_int = 8;
pub const LCD2S_STATUS_BUF_MASK: c_uint = 0x7f;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lcd2s_data {
    pub i2c: *mut i2c_client,
    pub charlcd: *mut charlcd,
}

#[no_mangle]
unsafe extern "C" fn lcd2s_wait_buf_free(client: *const i2c_client, count: c_int) -> i32 {
    static s32 lcd2s_wait_buf_free(const struct i2c_client *client, int count)
    {
    s32 status;
    status = i2c_smbus_read_byte_data(client, LCD2S_CMD_READ_STATUS);
    if (status < 0)
    return status;
    while ((status & LCD2S_STATUS_BUF_MASK) < count) {
    mdelay(1);
    status = i2c_smbus_read_byte_data(client,
    LCD2S_CMD_READ_STATUS);
    if (status < 0)
    return status;
    }
    return 0;
    }
    static int lcd2s_i2c_master_send(const struct i2c_client *client,
    const char *buf, int count)
    {
    s32 status;
    status = lcd2s_wait_buf_free(client, count);
    if (status < 0)
    return status;
    return i2c_master_send(client, buf, count);
    }
#[no_mangle]
unsafe extern "C" fn lcd2s_i2c_smbus_write_byte(client: *const i2c_client, value: u8) -> c_int {
    static int lcd2s_i2c_smbus_write_byte(const struct i2c_client *client, u8 value)
    {
    s32 status;
    status = lcd2s_wait_buf_free(client, 1);
    if (status < 0)
    return status;
    return i2c_smbus_write_byte(client, value);
    }
#[no_mangle]
unsafe extern "C" fn lcd2s_print(lcd: *mut charlcd, c: c_int) -> c_int {
    static int lcd2s_print(struct charlcd *lcd, int c)
    {
    struct lcd2s_data *lcd2s = lcd.drvdata;
    u8 buf[2] = { LCD2S_CMD_WRITE, c };
    int ret;
    ret = lcd2s_i2c_master_send(lcd2s.i2c, buf, sizeof(buf));
    if (ret < 0)
    return ret;
    if (ret != sizeof(buf))
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lcd2s_gotoxy(lcd: *mut charlcd, x: c_uint, y: c_uint) -> c_int {
    static int lcd2s_gotoxy(struct charlcd *lcd, unsigned int x, unsigned int y)
    {
    struct lcd2s_data *lcd2s = lcd.drvdata;
    u8 buf[3] = { LCD2S_CMD_CUR_POS, y + 1, x + 1 };
    int ret;
    ret = lcd2s_i2c_master_send(lcd2s.i2c, buf, sizeof(buf));
    if (ret < 0)
    return ret;
    if (ret != sizeof(buf))
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lcd2s_home(lcd: *mut charlcd) -> c_int {
    static int lcd2s_home(struct charlcd *lcd)
    {
    struct lcd2s_data *lcd2s = lcd.drvdata;
    lcd2s_i2c_smbus_write_byte(lcd2s.i2c, LCD2S_CMD_CUR_RESET);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lcd2s_init_display(lcd: *mut charlcd) -> c_int {
    static int lcd2s_init_display(struct charlcd *lcd)
    {
    struct lcd2s_data *lcd2s = lcd.drvdata;
// turn everything off, but display on
    lcd2s_i2c_smbus_write_byte(lcd2s.i2c, LCD2S_CMD_DISPLAY_ON);
    lcd2s_i2c_smbus_write_byte(lcd2s.i2c, LCD2S_CMD_BACKLIGHT_OFF);
    lcd2s_i2c_smbus_write_byte(lcd2s.i2c, LCD2S_CMD_CUR_MOVES_FWD);
    lcd2s_i2c_smbus_write_byte(lcd2s.i2c, LCD2S_CMD_CUR_BLINK_OFF);
    lcd2s_i2c_smbus_write_byte(lcd2s.i2c, LCD2S_CMD_CUR_UL_OFF);
    lcd2s_i2c_smbus_write_byte(lcd2s.i2c, LCD2S_CMD_CLEAR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lcd2s_shift_cursor(lcd: *mut charlcd, dir: enum charlcd_shift_dir) -> c_int {
    static int lcd2s_shift_cursor(struct charlcd *lcd, enum charlcd_shift_dir dir)
    {
    struct lcd2s_data *lcd2s = lcd.drvdata;
    if (dir == CHARLCD_SHIFT_LEFT)
    lcd2s_i2c_smbus_write_byte(lcd2s.i2c, LCD2S_CMD_MOV_CUR_LEFT);
    else
    lcd2s_i2c_smbus_write_byte(lcd2s.i2c, LCD2S_CMD_MOV_CUR_RIGHT);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lcd2s_shift_display(lcd: *mut charlcd, dir: enum charlcd_shift_dir) -> c_int {
    static int lcd2s_shift_display(struct charlcd *lcd, enum charlcd_shift_dir dir)
    {
    struct lcd2s_data *lcd2s = lcd.drvdata;
    if (dir == CHARLCD_SHIFT_LEFT)
    lcd2s_i2c_smbus_write_byte(lcd2s.i2c, LCD2S_CMD_SHIFT_LEFT);
    else
    lcd2s_i2c_smbus_write_byte(lcd2s.i2c, LCD2S_CMD_SHIFT_RIGHT);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lcd2s_backlight(lcd: *mut charlcd, on: enum charlcd_onoff) {
    static void lcd2s_backlight(struct charlcd *lcd, enum charlcd_onoff on)
    {
    struct lcd2s_data *lcd2s = lcd.drvdata;
    if (on)
    lcd2s_i2c_smbus_write_byte(lcd2s.i2c, LCD2S_CMD_BACKLIGHT_ON);
    else
    lcd2s_i2c_smbus_write_byte(lcd2s.i2c, LCD2S_CMD_BACKLIGHT_OFF);
    }
#[no_mangle]
unsafe extern "C" fn lcd2s_display(lcd: *mut charlcd, on: enum charlcd_onoff) -> c_int {
    static int lcd2s_display(struct charlcd *lcd, enum charlcd_onoff on)
    {
    struct lcd2s_data *lcd2s = lcd.drvdata;
    if (on)
    lcd2s_i2c_smbus_write_byte(lcd2s.i2c, LCD2S_CMD_DISPLAY_ON);
    else
    lcd2s_i2c_smbus_write_byte(lcd2s.i2c, LCD2S_CMD_DISPLAY_OFF);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lcd2s_cursor(lcd: *mut charlcd, on: enum charlcd_onoff) -> c_int {
    static int lcd2s_cursor(struct charlcd *lcd, enum charlcd_onoff on)
    {
    struct lcd2s_data *lcd2s = lcd.drvdata;
    if (on)
    lcd2s_i2c_smbus_write_byte(lcd2s.i2c, LCD2S_CMD_CUR_UL_ON);
    else
    lcd2s_i2c_smbus_write_byte(lcd2s.i2c, LCD2S_CMD_CUR_UL_OFF);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lcd2s_blink(lcd: *mut charlcd, on: enum charlcd_onoff) -> c_int {
    static int lcd2s_blink(struct charlcd *lcd, enum charlcd_onoff on)
    {
    struct lcd2s_data *lcd2s = lcd.drvdata;
    if (on)
    lcd2s_i2c_smbus_write_byte(lcd2s.i2c, LCD2S_CMD_CUR_BLINK_ON);
    else
    lcd2s_i2c_smbus_write_byte(lcd2s.i2c, LCD2S_CMD_CUR_BLINK_OFF);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lcd2s_fontsize(lcd: *mut charlcd, size: enum charlcd_fontsize) -> c_int {
    static int lcd2s_fontsize(struct charlcd *lcd, enum charlcd_fontsize size)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lcd2s_lines(lcd: *mut charlcd, lines: enum charlcd_lines) -> c_int {
    static int lcd2s_lines(struct charlcd *lcd, enum charlcd_lines lines)
    {
    return 0;
    }
//
// Generator: LGcxxxxx...xx; must have <c> between '0' and '7',
// representing the numerical ASCII code of the redefined character,
// and <xx...xx> a sequence of 16 hex digits representing 8 bytes
// for each character. Most LCDs will only use 5 lower bits of
// the 7 first bytes.
//
#[no_mangle]
unsafe extern "C" fn lcd2s_redefine_char(lcd: *mut charlcd, esc: *mut c_char) -> c_int {
    static int lcd2s_redefine_char(struct charlcd *lcd, char *esc)
    {
    struct lcd2s_data *lcd2s = lcd.drvdata;
    u8 buf[LCD2S_CHARACTER_SIZE + 2] = { LCD2S_CMD_DEF_CUSTOM_CHAR };
    u8 value;
    int shift, i;
    if (!strchr(esc, ';'))
    return 0;
    esc++;
    buf[1] = *(esc++) - '0';
    if (buf[1] > 7)
    return 1;
    i = 2;
    shift = 0;
    value = 0;
    while (*esc && i < LCD2S_CHARACTER_SIZE + 2) {
    int half;
    shift ^= 4;
    half = hex_to_bin(*esc++);
    if (half < 0)
    continue;
    value |= half << shift;
    if (shift == 0) {
    buf[i++] = value;
    value = 0;
    }
    }
    lcd2s_i2c_master_send(lcd2s.i2c, buf, sizeof(buf));
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn lcd2s_clear_display(lcd: *mut charlcd) -> c_int {
    static int lcd2s_clear_display(struct charlcd *lcd)
    {
    struct lcd2s_data *lcd2s = lcd.drvdata;
// This implicitly sets cursor to first row and column
    lcd2s_i2c_smbus_write_byte(lcd2s.i2c, LCD2S_CMD_CLEAR);
    return 0;
    }
    static const struct charlcd_ops lcd2s_ops = {
    .print		= lcd2s_print,
    .backlight	= lcd2s_backlight,
    .gotoxy		= lcd2s_gotoxy,
    .home		= lcd2s_home,
    .clear_display	= lcd2s_clear_display,
    .init_display	= lcd2s_init_display,
    .shift_cursor	= lcd2s_shift_cursor,
    .shift_display	= lcd2s_shift_display,
    .display	= lcd2s_display,
    .cursor		= lcd2s_cursor,
    .blink		= lcd2s_blink,
    .fontsize	= lcd2s_fontsize,
    .lines		= lcd2s_lines,
    .redefine_char	= lcd2s_redefine_char,
    };
#[no_mangle]
unsafe extern "C" fn lcd2s_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int lcd2s_i2c_probe(struct i2c_client *i2c)
    {
    struct charlcd *lcd;
    struct lcd2s_data *lcd2s;
    int err;
    if (!i2c_check_functionality(i2c.adapter,
    I2C_FUNC_SMBUS_WRITE_BYTE_DATA |
    I2C_FUNC_SMBUS_WRITE_BLOCK_DATA))
    return -EIO;
// Test, if the display is responding
    err = lcd2s_i2c_smbus_write_byte(i2c, LCD2S_CMD_DISPLAY_OFF);
    if (err < 0)
    return err;
    lcd = charlcd_alloc(sizeof(*lcd2s));
    if (!lcd)
    return -ENOMEM;
    lcd.ops = &lcd2s_ops;
    lcd2s = lcd.drvdata;
    lcd2s.i2c = i2c;
    lcd2s.charlcd = lcd;
// Required properties
    err = device_property_read_u32(&i2c.dev, "display-height-chars",
    &lcd.height);
    if (err)
    goto fail1;
    err = device_property_read_u32(&i2c.dev, "display-width-chars",
    &lcd.width);
    if (err)
    goto fail1;
    err = charlcd_register(lcd2s.charlcd);
    if (err)
    goto fail1;
    i2c_set_clientdata(i2c, lcd2s);
    return 0;
    fail1:
    charlcd_free(lcd2s.charlcd);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn lcd2s_i2c_remove(i2c: *mut i2c_client) {
    static void lcd2s_i2c_remove(struct i2c_client *i2c)
    {
    struct lcd2s_data *lcd2s = i2c_get_clientdata(i2c);
    charlcd_unregister(lcd2s.charlcd);
    charlcd_free(lcd2s.charlcd);
    }
    static const struct i2c_device_id lcd2s_i2c_id[] = {
    { "lcd2s" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, lcd2s_i2c_id);
    static const struct of_device_id lcd2s_of_table[] = {
    { .compatible = "modtronix,lcd2s" },
    { }
    };
    MODULE_DEVICE_TABLE(of, lcd2s_of_table);
    static struct i2c_driver lcd2s_i2c_driver = {
    .driver = {
    .name = "lcd2s",
    .of_match_table = lcd2s_of_table,
    },
    .probe = lcd2s_i2c_probe,
    .remove = lcd2s_i2c_remove,
    .id_table = lcd2s_i2c_id,
    };
    module_i2c_driver(lcd2s_i2c_driver);
    MODULE_DESCRIPTION("LCD2S character display driver");
    MODULE_AUTHOR("Lars Poeschel");
    MODULE_LICENSE("GPL");
