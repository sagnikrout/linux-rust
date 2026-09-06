//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/hx8357.c
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
// Driver for the Himax HX-8357 LCD Controller
//
// Copyright 2012 Free Electrons
//

pub const HX8357_NUM_IM_PINS: c_int = 3;
pub const HX8357_SWRESET: c_uint = 0x01;
pub const HX8357_GET_RED_CHANNEL: c_uint = 0x06;
pub const HX8357_GET_GREEN_CHANNEL: c_uint = 0x07;
pub const HX8357_GET_BLUE_CHANNEL: c_uint = 0x08;
pub const HX8357_GET_POWER_MODE: c_uint = 0x0a;
pub const HX8357_GET_MADCTL: c_uint = 0x0b;
pub const HX8357_GET_PIXEL_FORMAT: c_uint = 0x0c;
pub const HX8357_GET_DISPLAY_MODE: c_uint = 0x0d;
pub const HX8357_GET_SIGNAL_MODE: c_uint = 0x0e;
pub const HX8357_GET_DIAGNOSTIC_RESULT: c_uint = 0x0f;
pub const HX8357_ENTER_SLEEP_MODE: c_uint = 0x10;
pub const HX8357_EXIT_SLEEP_MODE: c_uint = 0x11;
pub const HX8357_ENTER_PARTIAL_MODE: c_uint = 0x12;
pub const HX8357_ENTER_NORMAL_MODE: c_uint = 0x13;
pub const HX8357_EXIT_INVERSION_MODE: c_uint = 0x20;
pub const HX8357_ENTER_INVERSION_MODE: c_uint = 0x21;
pub const HX8357_SET_DISPLAY_OFF: c_uint = 0x28;
pub const HX8357_SET_DISPLAY_ON: c_uint = 0x29;
pub const HX8357_SET_COLUMN_ADDRESS: c_uint = 0x2a;
pub const HX8357_SET_PAGE_ADDRESS: c_uint = 0x2b;
pub const HX8357_WRITE_MEMORY_START: c_uint = 0x2c;
pub const HX8357_READ_MEMORY_START: c_uint = 0x2e;
pub const HX8357_SET_PARTIAL_AREA: c_uint = 0x30;
pub const HX8357_SET_SCROLL_AREA: c_uint = 0x33;
pub const HX8357_SET_TEAR_OFF: c_uint = 0x34;
pub const HX8357_SET_TEAR_ON: c_uint = 0x35;
pub const HX8357_SET_ADDRESS_MODE: c_uint = 0x36;
pub const HX8357_SET_SCROLL_START: c_uint = 0x37;
pub const HX8357_EXIT_IDLE_MODE: c_uint = 0x38;
pub const HX8357_ENTER_IDLE_MODE: c_uint = 0x39;
pub const HX8357_SET_PIXEL_FORMAT: c_uint = 0x3a;

pub const HX8357_WRITE_MEMORY_CONTINUE: c_uint = 0x3c;
pub const HX8357_READ_MEMORY_CONTINUE: c_uint = 0x3e;
pub const HX8357_SET_TEAR_SCAN_LINES: c_uint = 0x44;
pub const HX8357_GET_SCAN_LINES: c_uint = 0x45;
pub const HX8357_READ_DDB_START: c_uint = 0xa1;
pub const HX8357_SET_DISPLAY_MODE: c_uint = 0xb4;

pub const HX8357_SET_PANEL_DRIVING: c_uint = 0xc0;
pub const HX8357_SET_DISPLAY_FRAME: c_uint = 0xc5;
pub const HX8357_SET_RGB: c_uint = 0xc6;

pub const HX8357_SET_GAMMA: c_uint = 0xc8;
pub const HX8357_SET_POWER: c_uint = 0xd0;
pub const HX8357_SET_VCOM: c_uint = 0xd1;
pub const HX8357_SET_POWER_NORMAL: c_uint = 0xd2;
pub const HX8357_SET_PANEL_RELATED: c_uint = 0xe9;
pub const HX8369_SET_DISPLAY_BRIGHTNESS: c_uint = 0x51;
pub const HX8369_WRITE_CABC_DISPLAY_VALUE: c_uint = 0x53;
pub const HX8369_WRITE_CABC_BRIGHT_CTRL: c_uint = 0x55;
pub const HX8369_WRITE_CABC_MIN_BRIGHTNESS: c_uint = 0x5e;
pub const HX8369_SET_POWER: c_uint = 0xb1;
pub const HX8369_SET_DISPLAY_MODE: c_uint = 0xb2;
pub const HX8369_SET_DISPLAY_WAVEFORM_CYC: c_uint = 0xb4;
pub const HX8369_SET_VCOM: c_uint = 0xb6;
pub const HX8369_SET_EXTENSION_COMMAND: c_uint = 0xb9;
pub const HX8369_SET_GIP: c_uint = 0xd5;
pub const HX8369_SET_GAMMA_CURVE_RELATED: c_uint = 0xe0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hx8357_data {
    pub im_pins: *mut gpio_descs,
    pub reset: *mut gpio_desc,
    pub spi: *mut spi_device,
    pub state: c_int,
}

    static u8 hx8357_seq_power[] = {
    HX8357_SET_POWER, 0x44, 0x41, 0x06,
    };
    static u8 hx8357_seq_vcom[] = {
    HX8357_SET_VCOM, 0x40, 0x10,
    };
    static u8 hx8357_seq_power_normal[] = {
    HX8357_SET_POWER_NORMAL, 0x05, 0x12,
    };
    static u8 hx8357_seq_panel_driving[] = {
    HX8357_SET_PANEL_DRIVING, 0x14, 0x3b, 0x00, 0x02, 0x11,
    };
    static u8 hx8357_seq_display_frame[] = {
    HX8357_SET_DISPLAY_FRAME, 0x0c,
    };
    static u8 hx8357_seq_panel_related[] = {
    HX8357_SET_PANEL_RELATED, 0x01,
    };
    static u8 hx8357_seq_undefined1[] = {
    0xea, 0x03, 0x00, 0x00,
    };
    static u8 hx8357_seq_undefined2[] = {
    0xeb, 0x40, 0x54, 0x26, 0xdb,
    };
    static u8 hx8357_seq_gamma[] = {
    HX8357_SET_GAMMA, 0x00, 0x15, 0x00, 0x22, 0x00,
    0x08, 0x77, 0x26, 0x77, 0x22, 0x04, 0x00,
    };
    static u8 hx8357_seq_address_mode[] = {
    HX8357_SET_ADDRESS_MODE, 0xc0,
    };
    static u8 hx8357_seq_pixel_format[] = {
    HX8357_SET_PIXEL_FORMAT,
    HX8357_SET_PIXEL_FORMAT_DPI_18BIT |
    HX8357_SET_PIXEL_FORMAT_DBI_18BIT,
    };
    static u8 hx8357_seq_column_address[] = {
    HX8357_SET_COLUMN_ADDRESS, 0x00, 0x00, 0x01, 0x3f,
    };
    static u8 hx8357_seq_page_address[] = {
    HX8357_SET_PAGE_ADDRESS, 0x00, 0x00, 0x01, 0xdf,
    };
    static u8 hx8357_seq_rgb[] = {
    HX8357_SET_RGB, 0x02,
    };
    static u8 hx8357_seq_display_mode[] = {
    HX8357_SET_DISPLAY_MODE,
    HX8357_SET_DISPLAY_MODE_RGB_THROUGH |
    HX8357_SET_DISPLAY_MODE_RGB_INTERFACE,
    };
    static u8 hx8369_seq_write_CABC_min_brightness[] = {
    HX8369_WRITE_CABC_MIN_BRIGHTNESS, 0x00,
    };
    static u8 hx8369_seq_write_CABC_control[] = {
    HX8369_WRITE_CABC_DISPLAY_VALUE, 0x24,
    };
    static u8 hx8369_seq_set_display_brightness[] = {
    HX8369_SET_DISPLAY_BRIGHTNESS, 0xFF,
    };
    static u8 hx8369_seq_write_CABC_control_setting[] = {
    HX8369_WRITE_CABC_BRIGHT_CTRL, 0x02,
    };
    static u8 hx8369_seq_extension_command[] = {
    HX8369_SET_EXTENSION_COMMAND, 0xff, 0x83, 0x69,
    };
    static u8 hx8369_seq_display_related[] = {
    HX8369_SET_DISPLAY_MODE, 0x00, 0x2b, 0x03, 0x03, 0x70, 0x00,
    0xff, 0x00, 0x00, 0x00, 0x00, 0x03, 0x03, 0x00,	0x01,
    };
    static u8 hx8369_seq_panel_waveform_cycle[] = {
    HX8369_SET_DISPLAY_WAVEFORM_CYC, 0x0a, 0x1d, 0x80, 0x06, 0x02,
    };
    static u8 hx8369_seq_set_address_mode[] = {
    HX8357_SET_ADDRESS_MODE, 0x00,
    };
    static u8 hx8369_seq_vcom[] = {
    HX8369_SET_VCOM, 0x3e, 0x3e,
    };
    static u8 hx8369_seq_gip[] = {
    HX8369_SET_GIP, 0x00, 0x01, 0x03, 0x25, 0x01, 0x02, 0x28, 0x70,
    0x11, 0x13, 0x00, 0x00, 0x40, 0x26, 0x51, 0x37, 0x00, 0x00, 0x71,
    0x35, 0x60, 0x24, 0x07, 0x0f, 0x04, 0x04,
    };
    static u8 hx8369_seq_power[] = {
    HX8369_SET_POWER, 0x01, 0x00, 0x34, 0x03, 0x00, 0x11, 0x11, 0x32,
    0x2f, 0x3f, 0x3f, 0x01, 0x3a, 0x01, 0xe6, 0xe6, 0xe6, 0xe6, 0xe6,
    };
    static u8 hx8369_seq_gamma_curve_related[] = {
    HX8369_SET_GAMMA_CURVE_RELATED, 0x00, 0x0d, 0x19, 0x2f, 0x3b, 0x3d,
    0x2e, 0x4a, 0x08, 0x0e, 0x0f, 0x14, 0x16, 0x14, 0x14, 0x14, 0x1e,
    0x00, 0x0d, 0x19, 0x2f, 0x3b, 0x3d, 0x2e, 0x4a, 0x08, 0x0e, 0x0f,
    0x14, 0x16, 0x14, 0x14, 0x14, 0x1e,
    };
    static int hx8357_spi_write_then_read(struct lcd_device *lcdev,
    u8 *txbuf, u16 txlen,
    u8 *rxbuf, u16 rxlen)
    {
    struct hx8357_data *lcd = lcd_get_data(lcdev);
    struct spi_message msg;
    struct spi_transfer xfer[2];
    u16 *local_txbuf = core::ptr::null_mut();
    let mut ret: c_int = 0;
    memset(xfer, 0, sizeof(xfer));
    spi_message_init(&msg);
    if (txlen) {
    int i;
    local_txbuf = kcalloc(txlen, sizeof(*local_txbuf), GFP_KERNEL);
    if (!local_txbuf)
    return -ENOMEM;
    for (i = 0; i < txlen; i++) {
    local_txbuf[i] = txbuf[i];
    if (i > 0)
    local_txbuf[i] |= 1 << 8;
    }
    xfer[0].len = 2 * txlen;
    xfer[0].bits_per_word = 9;
    xfer[0].tx_buf = local_txbuf;
    spi_message_add_tail(&xfer[0], &msg);
    }
    if (rxlen) {
    xfer[1].len = rxlen;
    xfer[1].bits_per_word = 8;
    xfer[1].rx_buf = rxbuf;
    spi_message_add_tail(&xfer[1], &msg);
    }
    ret = spi_sync(lcd.spi, &msg);
    if (ret < 0)
    dev_err(&lcdev.dev, "Couldn't send SPI data\n");
    if (txlen)
    kfree(local_txbuf);
    return ret;
    }
    static inline int hx8357_spi_write_array(struct lcd_device *lcdev,
    u8 *value, u8 len)
    {
    return hx8357_spi_write_then_read(lcdev, value, len, core::ptr::null_mut(), 0);
    }
    static inline int hx8357_spi_write_byte(struct lcd_device *lcdev,
    u8 value)
    {
    return hx8357_spi_write_then_read(lcdev, &value, 1, core::ptr::null_mut(), 0);
    }
#[no_mangle]
unsafe extern "C" fn hx8357_enter_standby(lcdev: *mut lcd_device) -> c_int {
    static int hx8357_enter_standby(struct lcd_device *lcdev)
    {
    int ret;
    ret = hx8357_spi_write_byte(lcdev, HX8357_SET_DISPLAY_OFF);
    if (ret < 0)
    return ret;
    usleep_range(10000, 12000);
    ret = hx8357_spi_write_byte(lcdev, HX8357_ENTER_SLEEP_MODE);
    if (ret < 0)
    return ret;
//
// The controller needs 120ms when entering in sleep mode before we can
// send the command to go off sleep mode
//
    msleep(120);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hx8357_exit_standby(lcdev: *mut lcd_device) -> c_int {
    static int hx8357_exit_standby(struct lcd_device *lcdev)
    {
    int ret;
    ret = hx8357_spi_write_byte(lcdev, HX8357_EXIT_SLEEP_MODE);
    if (ret < 0)
    return ret;
//
// The controller needs 120ms when exiting from sleep mode before we
// can send the command to enter in sleep mode
//
    msleep(120);
    ret = hx8357_spi_write_byte(lcdev, HX8357_SET_DISPLAY_ON);
    if (ret < 0)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hx8357_lcd_reset(lcdev: *mut lcd_device) {
    static void hx8357_lcd_reset(struct lcd_device *lcdev)
    {
    struct hx8357_data *lcd = lcd_get_data(lcdev);
// Reset the screen
    gpiod_set_value(lcd.reset, 0);
    usleep_range(10000, 12000);
    gpiod_set_value(lcd.reset, 1);
    usleep_range(10000, 12000);
    gpiod_set_value(lcd.reset, 0);
// The controller needs 120ms to recover from reset
    msleep(120);
    }
#[no_mangle]
unsafe extern "C" fn hx8357_lcd_init(lcdev: *mut lcd_device) -> c_int {
    static int hx8357_lcd_init(struct lcd_device *lcdev)
    {
    struct hx8357_data *lcd = lcd_get_data(lcdev);
    int ret;
//
// Set the interface selection pins to SPI mode, with three
// wires
//
    if (lcd.im_pins) {
    gpiod_set_value_cansleep(lcd.im_pins.desc[0], 1);
    gpiod_set_value_cansleep(lcd.im_pins.desc[1], 0);
    gpiod_set_value_cansleep(lcd.im_pins.desc[2], 1);
    }
    ret = hx8357_spi_write_array(lcdev, hx8357_seq_power,
    ARRAY_SIZE(hx8357_seq_power));
    if (ret < 0)
    return ret;
    ret = hx8357_spi_write_array(lcdev, hx8357_seq_vcom,
    ARRAY_SIZE(hx8357_seq_vcom));
    if (ret < 0)
    return ret;
    ret = hx8357_spi_write_array(lcdev, hx8357_seq_power_normal,
    ARRAY_SIZE(hx8357_seq_power_normal));
    if (ret < 0)
    return ret;
    ret = hx8357_spi_write_array(lcdev, hx8357_seq_panel_driving,
    ARRAY_SIZE(hx8357_seq_panel_driving));
    if (ret < 0)
    return ret;
    ret = hx8357_spi_write_array(lcdev, hx8357_seq_display_frame,
    ARRAY_SIZE(hx8357_seq_display_frame));
    if (ret < 0)
    return ret;
    ret = hx8357_spi_write_array(lcdev, hx8357_seq_panel_related,
    ARRAY_SIZE(hx8357_seq_panel_related));
    if (ret < 0)
    return ret;
    ret = hx8357_spi_write_array(lcdev, hx8357_seq_undefined1,
    ARRAY_SIZE(hx8357_seq_undefined1));
    if (ret < 0)
    return ret;
    ret = hx8357_spi_write_array(lcdev, hx8357_seq_undefined2,
    ARRAY_SIZE(hx8357_seq_undefined2));
    if (ret < 0)
    return ret;
    ret = hx8357_spi_write_array(lcdev, hx8357_seq_gamma,
    ARRAY_SIZE(hx8357_seq_gamma));
    if (ret < 0)
    return ret;
    ret = hx8357_spi_write_array(lcdev, hx8357_seq_address_mode,
    ARRAY_SIZE(hx8357_seq_address_mode));
    if (ret < 0)
    return ret;
    ret = hx8357_spi_write_array(lcdev, hx8357_seq_pixel_format,
    ARRAY_SIZE(hx8357_seq_pixel_format));
    if (ret < 0)
    return ret;
    ret = hx8357_spi_write_array(lcdev, hx8357_seq_column_address,
    ARRAY_SIZE(hx8357_seq_column_address));
    if (ret < 0)
    return ret;
    ret = hx8357_spi_write_array(lcdev, hx8357_seq_page_address,
    ARRAY_SIZE(hx8357_seq_page_address));
    if (ret < 0)
    return ret;
    ret = hx8357_spi_write_array(lcdev, hx8357_seq_rgb,
    ARRAY_SIZE(hx8357_seq_rgb));
    if (ret < 0)
    return ret;
    ret = hx8357_spi_write_array(lcdev, hx8357_seq_display_mode,
    ARRAY_SIZE(hx8357_seq_display_mode));
    if (ret < 0)
    return ret;
    ret = hx8357_spi_write_byte(lcdev, HX8357_EXIT_SLEEP_MODE);
    if (ret < 0)
    return ret;
//
// The controller needs 120ms to fully recover from exiting sleep mode
//
    msleep(120);
    ret = hx8357_spi_write_byte(lcdev, HX8357_SET_DISPLAY_ON);
    if (ret < 0)
    return ret;
    usleep_range(5000, 7000);
    ret = hx8357_spi_write_byte(lcdev, HX8357_WRITE_MEMORY_START);
    if (ret < 0)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hx8369_lcd_init(lcdev: *mut lcd_device) -> c_int {
    static int hx8369_lcd_init(struct lcd_device *lcdev)
    {
    int ret;
    ret = hx8357_spi_write_array(lcdev, hx8369_seq_extension_command,
    ARRAY_SIZE(hx8369_seq_extension_command));
    if (ret < 0)
    return ret;
    usleep_range(10000, 12000);
    ret = hx8357_spi_write_array(lcdev, hx8369_seq_display_related,
    ARRAY_SIZE(hx8369_seq_display_related));
    if (ret < 0)
    return ret;
    ret = hx8357_spi_write_array(lcdev, hx8369_seq_panel_waveform_cycle,
    ARRAY_SIZE(hx8369_seq_panel_waveform_cycle));
    if (ret < 0)
    return ret;
    ret = hx8357_spi_write_array(lcdev, hx8369_seq_set_address_mode,
    ARRAY_SIZE(hx8369_seq_set_address_mode));
    if (ret < 0)
    return ret;
    ret = hx8357_spi_write_array(lcdev, hx8369_seq_vcom,
    ARRAY_SIZE(hx8369_seq_vcom));
    if (ret < 0)
    return ret;
    ret = hx8357_spi_write_array(lcdev, hx8369_seq_gip,
    ARRAY_SIZE(hx8369_seq_gip));
    if (ret < 0)
    return ret;
    ret = hx8357_spi_write_array(lcdev, hx8369_seq_power,
    ARRAY_SIZE(hx8369_seq_power));
    if (ret < 0)
    return ret;
    ret = hx8357_spi_write_byte(lcdev, HX8357_EXIT_SLEEP_MODE);
    if (ret < 0)
    return ret;
//
// The controller needs 120ms to fully recover from exiting sleep mode
//
    msleep(120);
    ret = hx8357_spi_write_array(lcdev, hx8369_seq_gamma_curve_related,
    ARRAY_SIZE(hx8369_seq_gamma_curve_related));
    if (ret < 0)
    return ret;
    ret = hx8357_spi_write_byte(lcdev, HX8357_EXIT_SLEEP_MODE);
    if (ret < 0)
    return ret;
    usleep_range(1000, 1200);
    ret = hx8357_spi_write_array(lcdev, hx8369_seq_write_CABC_control,
    ARRAY_SIZE(hx8369_seq_write_CABC_control));
    if (ret < 0)
    return ret;
    usleep_range(10000, 12000);
    ret = hx8357_spi_write_array(lcdev,
    hx8369_seq_write_CABC_control_setting,
    ARRAY_SIZE(hx8369_seq_write_CABC_control_setting));
    if (ret < 0)
    return ret;
    ret = hx8357_spi_write_array(lcdev,
    hx8369_seq_write_CABC_min_brightness,
    ARRAY_SIZE(hx8369_seq_write_CABC_min_brightness));
    if (ret < 0)
    return ret;
    usleep_range(10000, 12000);
    ret = hx8357_spi_write_array(lcdev, hx8369_seq_set_display_brightness,
    ARRAY_SIZE(hx8369_seq_set_display_brightness));
    if (ret < 0)
    return ret;
    ret = hx8357_spi_write_byte(lcdev, HX8357_SET_DISPLAY_ON);
    if (ret < 0)
    return ret;
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn hx8357_set_power(lcdev: *mut lcd_device, power: c_int) -> c_int {
    static int hx8357_set_power(struct lcd_device *lcdev, int power)
    {
    struct hx8357_data *lcd = lcd_get_data(lcdev);
    let mut ret: c_int = 0;
    if (POWER_IS_ON(power) && !POWER_IS_ON(lcd.state))
    ret = hx8357_exit_standby(lcdev);
#[no_mangle]
pub unsafe extern "C" fn if(POWER_IS_ON(lcd->state): !POWER_IS_ON(power) &&) -> else {
    else if (!POWER_IS_ON(power) && POWER_IS_ON(lcd.state))
    ret = hx8357_enter_standby(lcdev);
    if (ret == 0)
    lcd.state = power;
    else
    dev_warn(&lcdev.dev, "failed to set power mode %d\n", power);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hx8357_get_power(lcdev: *mut lcd_device) -> c_int {
    static int hx8357_get_power(struct lcd_device *lcdev)
    {
    struct hx8357_data *lcd = lcd_get_data(lcdev);
    return lcd.state;
    }
    static const struct lcd_ops hx8357_ops = {
    .set_power	= hx8357_set_power,
    .get_power	= hx8357_get_power,
    };
    typedef int (*hx8357_init_fn)(struct lcd_device *);
#[no_mangle]
unsafe extern "C" fn hx8357_probe(spi: *mut spi_device) -> c_int {
    static int hx8357_probe(struct spi_device *spi)
    {
    struct device *dev = &spi.dev;
    struct lcd_device *lcdev;
    struct hx8357_data *lcd;
    hx8357_init_fn init_fn;
    int i, ret;
    lcd = devm_kzalloc(dev, sizeof(*lcd), GFP_KERNEL);
    if (!lcd)
    return -ENOMEM;
    ret = spi_setup(spi);
    if (ret < 0)
    return dev_err_probe(dev, ret, "SPI setup failed.\n");
    lcd.spi = spi;
    init_fn = device_get_match_data(dev);
    if (!init_fn)
    return -EINVAL;
    lcd.reset = devm_gpiod_get(dev, "reset", GPIOD_OUT_LOW);
    if (IS_ERR(lcd.reset))
    return dev_err_probe(dev, PTR_ERR(lcd.reset), "failed to request reset GPIO\n");
    gpiod_set_consumer_name(lcd.reset, "hx8357-reset");
    lcd.im_pins = devm_gpiod_get_array_optional(dev, "im", GPIOD_OUT_LOW);
    if (IS_ERR(lcd.im_pins))
    return dev_err_probe(dev, PTR_ERR(lcd.im_pins), "failed to request im GPIOs\n");
    if (lcd.im_pins) {
    if (lcd.im_pins.ndescs < HX8357_NUM_IM_PINS)
    return dev_err_probe(dev, -EINVAL, "not enough im GPIOs\n");
    for (i = 0; i < HX8357_NUM_IM_PINS; i++)
    gpiod_set_consumer_name(lcd.im_pins.desc[i], "im_pins");
    }
    lcdev = devm_lcd_device_register(dev, "mxsfb", dev, lcd, &hx8357_ops);
    if (IS_ERR(lcdev)) {
    ret = PTR_ERR(lcdev);
    return ret;
    }
    spi_set_drvdata(spi, lcdev);
    hx8357_lcd_reset(lcdev);
    ret = init_fn(lcdev);
    if (ret)
    return dev_err_probe(dev, ret, "Couldn't initialize panel\n");
    dev_info(dev, "Panel probed\n");
    return 0;
    }
    static const struct of_device_id hx8357_dt_ids[] = {
    {
    .compatible = "himax,hx8357",
    .data = hx8357_lcd_init,
    },
    {
    .compatible = "himax,hx8369",
    .data = hx8369_lcd_init,
    },
    {}
    };
    MODULE_DEVICE_TABLE(of, hx8357_dt_ids);
    static struct spi_driver hx8357_driver = {
    .probe  = hx8357_probe,
    .driver = {
    .name = "hx8357",
    .of_match_table = hx8357_dt_ids,
    },
    };
    module_spi_driver(hx8357_driver);
    MODULE_AUTHOR("Maxime Ripard <maxime.ripard@free-electrons.com>");
    MODULE_DESCRIPTION("Himax HX-8357 LCD Driver");
    MODULE_LICENSE("GPL");
