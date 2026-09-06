//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/ili922x.c
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
// (C) Copyright 2008
// Stefano Babic, DENX Software Engineering, sbabic@denx.de.
//
// This driver implements a lcd device for the ILITEK 922x display
// controller. The interface to the display is SPI and the display's
// memory is cyclically updated over the RGB interface.
//

// Register offset, see manual section 8.2
pub const REG_START_OSCILLATION: c_uint = 0x00;
pub const REG_DRIVER_CODE_READ: c_uint = 0x00;
pub const REG_DRIVER_OUTPUT_CONTROL: c_uint = 0x01;
pub const REG_LCD_AC_DRIVEING_CONTROL: c_uint = 0x02;
pub const REG_ENTRY_MODE: c_uint = 0x03;
pub const REG_COMPARE_1: c_uint = 0x04;
pub const REG_COMPARE_2: c_uint = 0x05;
pub const REG_DISPLAY_CONTROL_1: c_uint = 0x07;
pub const REG_DISPLAY_CONTROL_2: c_uint = 0x08;
pub const REG_DISPLAY_CONTROL_3: c_uint = 0x09;
pub const REG_FRAME_CYCLE_CONTROL: c_uint = 0x0B;
pub const REG_EXT_INTF_CONTROL: c_uint = 0x0C;
pub const REG_POWER_CONTROL_1: c_uint = 0x10;
pub const REG_POWER_CONTROL_2: c_uint = 0x11;
pub const REG_POWER_CONTROL_3: c_uint = 0x12;
pub const REG_POWER_CONTROL_4: c_uint = 0x13;
pub const REG_RAM_ADDRESS_SET: c_uint = 0x21;
pub const REG_WRITE_DATA_TO_GRAM: c_uint = 0x22;
pub const REG_RAM_WRITE_MASK1: c_uint = 0x23;
pub const REG_RAM_WRITE_MASK2: c_uint = 0x24;
pub const REG_GAMMA_CONTROL_1: c_uint = 0x30;
pub const REG_GAMMA_CONTROL_2: c_uint = 0x31;
pub const REG_GAMMA_CONTROL_3: c_uint = 0x32;
pub const REG_GAMMA_CONTROL_4: c_uint = 0x33;
pub const REG_GAMMA_CONTROL_5: c_uint = 0x34;
pub const REG_GAMMA_CONTROL_6: c_uint = 0x35;
pub const REG_GAMMA_CONTROL_7: c_uint = 0x36;
pub const REG_GAMMA_CONTROL_8: c_uint = 0x37;
pub const REG_GAMMA_CONTROL_9: c_uint = 0x38;
pub const REG_GAMMA_CONTROL_10: c_uint = 0x39;
pub const REG_GATE_SCAN_CONTROL: c_uint = 0x40;
pub const REG_VERT_SCROLL_CONTROL: c_uint = 0x41;
pub const REG_FIRST_SCREEN_DRIVE_POS: c_uint = 0x42;
pub const REG_SECOND_SCREEN_DRIVE_POS: c_uint = 0x43;
pub const REG_RAM_ADDR_POS_H: c_uint = 0x44;
pub const REG_RAM_ADDR_POS_V: c_uint = 0x45;
pub const REG_OSCILLATOR_CONTROL: c_uint = 0x4F;
pub const REG_GPIO: c_uint = 0x60;
pub const REG_OTP_VCM_PROGRAMMING: c_uint = 0x61;
pub const REG_OTP_VCM_STATUS_ENABLE: c_uint = 0x62;
pub const REG_OTP_PROGRAMMING_ID_KEY: c_uint = 0x65;
//
// maximum frequency for register access
// (not for the GRAM access)
//
pub const ILITEK_MAX_FREQ_REG: c_int = 4000000;
//
// Device ID as found in the datasheet (supports 9221 and 9222)
//
pub const ILITEK_DEVICE_ID: c_uint = 0x9220;
pub const ILITEK_DEVICE_ID_MASK: c_uint = 0xFFF0;
// Last two bits in the START BYTE
pub const START_RS_INDEX: c_int = 0;
pub const START_RS_REG: c_int = 1;
pub const START_RW_WRITE: c_int = 0;
pub const START_RW_READ: c_int = 1;
//
// START_BYTE(id, rs, rw)
//
// Set the start byte according to the required operation.
// The start byte is defined as:
// ----------------------------------
// | 0 | 1 | 1 | 1 | 0 | ID | RS | RW |
// ----------------------------------
// @id: display's id as set by the manufacturer
// @rs: operation type bit, one of:
// - START_RS_INDEX	set the index register
// - START_RS_REG	write/read registers/GRAM
// @rw: read/write operation
// - START_RW_WRITE	write
// - START_RW_READ	read
//

    (0x70 | (((id) & 0x01) << 2) | (((rs) & 0x01) << 1) | ((rw) & 0x01))
//
// CHECK_FREQ_REG(spi_device s, spi_transfer x) - Check the frequency
// for the SPI transfer. According to the datasheet, the controller
// accept higher frequency for the GRAM transfer, but it requires
// lower frequency when the registers are read/written.
// The macro sets the frequency in the spi_transfer structure if
// the frequency exceeds the maximum value.
// @s: pointer to an SPI device
// @x: pointer to the read/write buffer pair
//

    do {			\
    if (s.max_speed_hz > ILITEK_MAX_FREQ_REG)	\
    ((struct spi_transfer *)x).speed_hz =	\
    ILITEK_MAX_FREQ_REG;	\
    } while (0)
pub const CMD_BUFSIZE: c_int = 16;

//
// ili922x_id - id as set by manufacturer
//
    let mut ili922x_id: static int = 1;
    module_param(ili922x_id, int, 0);
    static int tx_invert;
    module_param(tx_invert, int, 0);
//
// driver's private structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ili922x {
    pub spi: *mut spi_device,
    pub ld: *mut lcd_device,
    pub power: c_int,
}

//
// ili922x_read_status - read status register from display
// @spi: spi device
// @rs:  output value
//
#[no_mangle]
unsafe extern "C" fn ili922x_read_status(spi: *mut spi_device, rs: *mut u16) -> c_int {
    static int ili922x_read_status(struct spi_device *spi, u16 *rs)
    {
    struct spi_message msg;
    struct spi_transfer xfer;
    unsigned char tbuf[CMD_BUFSIZE];
    unsigned char rbuf[CMD_BUFSIZE];
    int ret, i;
    memset(&xfer, 0, sizeof(struct spi_transfer));
    spi_message_init(&msg);
    xfer.tx_buf = tbuf;
    xfer.rx_buf = rbuf;
    xfer.cs_change = 1;
    CHECK_FREQ_REG(spi, &xfer);
    tbuf[0] = set_tx_byte(START_BYTE(ili922x_id, START_RS_INDEX,
    START_RW_READ));
//
// we need 4-byte xfer here due to invalid dummy byte
// received after start byte
//
    for (i = 1; i < 4; i++)
    tbuf[i] = set_tx_byte(0);	/* dummy */
    xfer.bits_per_word = 8;
    xfer.len = 4;
    spi_message_add_tail(&xfer, &msg);
    ret = spi_sync(spi, &msg);
    if (ret < 0) {
    dev_dbg(&spi.dev, "Error sending SPI message 0x%x", ret);
    return ret;
    }
// rs = (rbuf[2] << 8) + rbuf[3];
    return 0;
    }
//
// ili922x_read - read register from display
// @spi: spi device
// @reg: offset of the register to be read
// @rx:  output value
//
#[no_mangle]
unsafe extern "C" fn ili922x_read(spi: *mut spi_device, reg: u8, rx: *mut u16) -> c_int {
    static int ili922x_read(struct spi_device *spi, u8 reg, u16 *rx)
    {
    struct spi_message msg;
    struct spi_transfer xfer_regindex, xfer_regvalue;
    unsigned char tbuf[CMD_BUFSIZE];
    unsigned char rbuf[CMD_BUFSIZE];
    int ret, len = 0, send_bytes;
    memset(&xfer_regindex, 0, sizeof(struct spi_transfer));
    memset(&xfer_regvalue, 0, sizeof(struct spi_transfer));
    spi_message_init(&msg);
    xfer_regindex.tx_buf = tbuf;
    xfer_regindex.rx_buf = rbuf;
    xfer_regindex.cs_change = 1;
    CHECK_FREQ_REG(spi, &xfer_regindex);
    tbuf[0] = set_tx_byte(START_BYTE(ili922x_id, START_RS_INDEX,
    START_RW_WRITE));
    tbuf[1] = set_tx_byte(0);
    tbuf[2] = set_tx_byte(reg);
    xfer_regindex.bits_per_word = 8;
    len = xfer_regindex.len = 3;
    spi_message_add_tail(&xfer_regindex, &msg);
    send_bytes = len;
    tbuf[len++] = set_tx_byte(START_BYTE(ili922x_id, START_RS_REG,
    START_RW_READ));
    tbuf[len++] = set_tx_byte(0);
    tbuf[len] = set_tx_byte(0);
    xfer_regvalue.cs_change = 1;
    xfer_regvalue.len = 3;
    xfer_regvalue.tx_buf = &tbuf[send_bytes];
    xfer_regvalue.rx_buf = &rbuf[send_bytes];
    CHECK_FREQ_REG(spi, &xfer_regvalue);
    spi_message_add_tail(&xfer_regvalue, &msg);
    ret = spi_sync(spi, &msg);
    if (ret < 0) {
    dev_dbg(&spi.dev, "Error sending SPI message 0x%x", ret);
    return ret;
    }
// rx = (rbuf[1 + send_bytes] << 8) + rbuf[2 + send_bytes];
    return 0;
    }
//
// ili922x_write - write a controller register
// @spi: struct spi_device
// @reg: offset of the register to be written
// @value: value to be written
//
#[no_mangle]
unsafe extern "C" fn ili922x_write(spi: *mut spi_device, reg: u8, value: u16) -> c_int {
    static int ili922x_write(struct spi_device *spi, u8 reg, u16 value)
    {
    struct spi_message msg;
    struct spi_transfer xfer_regindex, xfer_regvalue;
    unsigned char tbuf[CMD_BUFSIZE];
    unsigned char rbuf[CMD_BUFSIZE];
    int ret;
    memset(&xfer_regindex, 0, sizeof(struct spi_transfer));
    memset(&xfer_regvalue, 0, sizeof(struct spi_transfer));
    spi_message_init(&msg);
    xfer_regindex.tx_buf = tbuf;
    xfer_regindex.rx_buf = rbuf;
    xfer_regindex.cs_change = 1;
    CHECK_FREQ_REG(spi, &xfer_regindex);
    tbuf[0] = set_tx_byte(START_BYTE(ili922x_id, START_RS_INDEX,
    START_RW_WRITE));
    tbuf[1] = set_tx_byte(0);
    tbuf[2] = set_tx_byte(reg);
    xfer_regindex.bits_per_word = 8;
    xfer_regindex.len = 3;
    spi_message_add_tail(&xfer_regindex, &msg);
    ret = spi_sync(spi, &msg);
    if (ret < 0) {
    dev_err(&spi.dev, "Error sending SPI message 0x%x", ret);
    return ret;
    }
    spi_message_init(&msg);
    tbuf[0] = set_tx_byte(START_BYTE(ili922x_id, START_RS_REG,
    START_RW_WRITE));
    tbuf[1] = set_tx_byte((value & 0xFF00) >> 8);
    tbuf[2] = set_tx_byte(value & 0x00FF);
    xfer_regvalue.cs_change = 1;
    xfer_regvalue.len = 3;
    xfer_regvalue.tx_buf = tbuf;
    xfer_regvalue.rx_buf = rbuf;
    CHECK_FREQ_REG(spi, &xfer_regvalue);
    spi_message_add_tail(&xfer_regvalue, &msg);
    ret = spi_sync(spi, &msg);
    if (ret < 0) {
    dev_err(&spi.dev, "Error sending SPI message 0x%x", ret);
    return ret;
    }
    return 0;
    }

//
// ili922x_reg_dump - dump all registers
//
// @spi: pointer to an SPI device
//
#[no_mangle]
unsafe extern "C" fn ili922x_reg_dump(spi: *mut spi_device) {
    static void ili922x_reg_dump(struct spi_device *spi)
    {
    u8 reg;
    u16 rx;
    dev_dbg(&spi.dev, "ILI922x configuration registers:\n");
    for (reg = REG_START_OSCILLATION;
    reg <= REG_OTP_PROGRAMMING_ID_KEY; reg++) {
    ili922x_read(spi, reg, &rx);
    dev_dbg(&spi.dev, "reg @ 0x%02X: 0x%04X\n", reg, rx);
    }
    }

    static inline void ili922x_reg_dump(struct spi_device *spi) {}

//
// set_write_to_gram_reg - initialize the display to write the GRAM
// @spi: spi device
//
#[no_mangle]
unsafe extern "C" fn set_write_to_gram_reg(spi: *mut spi_device) {
    static void set_write_to_gram_reg(struct spi_device *spi)
    {
    struct spi_message msg;
    struct spi_transfer xfer;
    unsigned char tbuf[CMD_BUFSIZE];
    memset(&xfer, 0, sizeof(struct spi_transfer));
    spi_message_init(&msg);
    xfer.tx_buf = tbuf;
    xfer.rx_buf = core::ptr::null_mut();
    xfer.cs_change = 1;
    tbuf[0] = START_BYTE(ili922x_id, START_RS_INDEX, START_RW_WRITE);
    tbuf[1] = 0;
    tbuf[2] = REG_WRITE_DATA_TO_GRAM;
    xfer.bits_per_word = 8;
    xfer.len = 3;
    spi_message_add_tail(&xfer, &msg);
    spi_sync(spi, &msg);
    }
//
// ili922x_poweron - turn the display on
// @spi: spi device
//
// The sequence to turn on the display is taken from
// the datasheet and/or the example code provided by the
// manufacturer.
//
#[no_mangle]
unsafe extern "C" fn ili922x_poweron(spi: *mut spi_device) -> c_int {
    static int ili922x_poweron(struct spi_device *spi)
    {
    int ret;
// Power on
    ret = ili922x_write(spi, REG_POWER_CONTROL_1, 0x0000);
    usleep_range(10000, 10500);
    ret += ili922x_write(spi, REG_POWER_CONTROL_2, 0x0000);
    ret += ili922x_write(spi, REG_POWER_CONTROL_3, 0x0000);
    msleep(40);
    ret += ili922x_write(spi, REG_POWER_CONTROL_4, 0x0000);
    msleep(40);
// register 0x56 is not documented in the datasheet
    ret += ili922x_write(spi, 0x56, 0x080F);
    ret += ili922x_write(spi, REG_POWER_CONTROL_1, 0x4240);
    usleep_range(10000, 10500);
    ret += ili922x_write(spi, REG_POWER_CONTROL_2, 0x0000);
    ret += ili922x_write(spi, REG_POWER_CONTROL_3, 0x0014);
    msleep(40);
    ret += ili922x_write(spi, REG_POWER_CONTROL_4, 0x1319);
    msleep(40);
    return ret;
    }
//
// ili922x_poweroff - turn the display off
// @spi: spi device
//
#[no_mangle]
unsafe extern "C" fn ili922x_poweroff(spi: *mut spi_device) -> c_int {
    static int ili922x_poweroff(struct spi_device *spi)
    {
    int ret;
// Power off
    ret = ili922x_write(spi, REG_POWER_CONTROL_1, 0x0000);
    usleep_range(10000, 10500);
    ret += ili922x_write(spi, REG_POWER_CONTROL_2, 0x0000);
    ret += ili922x_write(spi, REG_POWER_CONTROL_3, 0x0000);
    msleep(40);
    ret += ili922x_write(spi, REG_POWER_CONTROL_4, 0x0000);
    msleep(40);
    return ret;
    }
//
// ili922x_display_init - initialize the display by setting
// the configuration registers
// @spi: spi device
//
#[no_mangle]
unsafe extern "C" fn ili922x_display_init(spi: *mut spi_device) {
    static void ili922x_display_init(struct spi_device *spi)
    {
    ili922x_write(spi, REG_START_OSCILLATION, 1);
    usleep_range(10000, 10500);
    ili922x_write(spi, REG_DRIVER_OUTPUT_CONTROL, 0x691B);
    ili922x_write(spi, REG_LCD_AC_DRIVEING_CONTROL, 0x0700);
    ili922x_write(spi, REG_ENTRY_MODE, 0x1030);
    ili922x_write(spi, REG_COMPARE_1, 0x0000);
    ili922x_write(spi, REG_COMPARE_2, 0x0000);
    ili922x_write(spi, REG_DISPLAY_CONTROL_1, 0x0037);
    ili922x_write(spi, REG_DISPLAY_CONTROL_2, 0x0202);
    ili922x_write(spi, REG_DISPLAY_CONTROL_3, 0x0000);
    ili922x_write(spi, REG_FRAME_CYCLE_CONTROL, 0x0000);
// Set RGB interface
    ili922x_write(spi, REG_EXT_INTF_CONTROL, 0x0110);
    ili922x_poweron(spi);
    ili922x_write(spi, REG_GAMMA_CONTROL_1, 0x0302);
    ili922x_write(spi, REG_GAMMA_CONTROL_2, 0x0407);
    ili922x_write(spi, REG_GAMMA_CONTROL_3, 0x0304);
    ili922x_write(spi, REG_GAMMA_CONTROL_4, 0x0203);
    ili922x_write(spi, REG_GAMMA_CONTROL_5, 0x0706);
    ili922x_write(spi, REG_GAMMA_CONTROL_6, 0x0407);
    ili922x_write(spi, REG_GAMMA_CONTROL_7, 0x0706);
    ili922x_write(spi, REG_GAMMA_CONTROL_8, 0x0000);
    ili922x_write(spi, REG_GAMMA_CONTROL_9, 0x0C06);
    ili922x_write(spi, REG_GAMMA_CONTROL_10, 0x0F00);
    ili922x_write(spi, REG_RAM_ADDRESS_SET, 0x0000);
    ili922x_write(spi, REG_GATE_SCAN_CONTROL, 0x0000);
    ili922x_write(spi, REG_VERT_SCROLL_CONTROL, 0x0000);
    ili922x_write(spi, REG_FIRST_SCREEN_DRIVE_POS, 0xDB00);
    ili922x_write(spi, REG_SECOND_SCREEN_DRIVE_POS, 0xDB00);
    ili922x_write(spi, REG_RAM_ADDR_POS_H, 0xAF00);
    ili922x_write(spi, REG_RAM_ADDR_POS_V, 0xDB00);
    ili922x_reg_dump(spi);
    set_write_to_gram_reg(spi);
    }
#[no_mangle]
unsafe extern "C" fn ili922x_lcd_power(lcd: *mut ili922x, power: c_int) -> c_int {
    static int ili922x_lcd_power(struct ili922x *lcd, int power)
    {
    let mut ret: c_int = 0;
    if (POWER_IS_ON(power) && !POWER_IS_ON(lcd.power))
    ret = ili922x_poweron(lcd.spi);
#[no_mangle]
pub unsafe extern "C" fn if(POWER_IS_ON(lcd->power): !POWER_IS_ON(power) &&) -> else {
    else if (!POWER_IS_ON(power) && POWER_IS_ON(lcd.power))
    ret = ili922x_poweroff(lcd.spi);
    if (!ret)
    lcd.power = power;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ili922x_set_power(ld: *mut lcd_device, power: c_int) -> c_int {
    static int ili922x_set_power(struct lcd_device *ld, int power)
    {
    struct ili922x *ili = lcd_get_data(ld);
    return ili922x_lcd_power(ili, power);
    }
#[no_mangle]
unsafe extern "C" fn ili922x_get_power(ld: *mut lcd_device) -> c_int {
    static int ili922x_get_power(struct lcd_device *ld)
    {
    struct ili922x *ili = lcd_get_data(ld);
    return ili.power;
    }
    static const struct lcd_ops ili922x_ops = {
    .get_power = ili922x_get_power,
    .set_power = ili922x_set_power,
    };
#[no_mangle]
unsafe extern "C" fn ili922x_probe(spi: *mut spi_device) -> c_int {
    static int ili922x_probe(struct spi_device *spi)
    {
    struct ili922x *ili;
    struct lcd_device *lcd;
    int ret;
    let mut reg: u16 = 0;
    ili = devm_kzalloc(&spi.dev, sizeof(*ili), GFP_KERNEL);
    if (!ili)
    return -ENOMEM;
    ili.spi = spi;
    spi_set_drvdata(spi, ili);
// check if the device is connected
    ret = ili922x_read(spi, REG_DRIVER_CODE_READ, &reg);
    if (ret || ((reg & ILITEK_DEVICE_ID_MASK) != ILITEK_DEVICE_ID)) {
    dev_err(&spi.dev,
    "no LCD found: Chip ID 0x%x, ret %d\n",
    reg, ret);
    return -ENODEV;
    }
    dev_info(&spi.dev, "ILI%x found, SPI freq %d, mode %d\n",
    reg, spi.max_speed_hz, spi.mode);
    ret = ili922x_read_status(spi, &reg);
    if (ret) {
    dev_err(&spi.dev, "reading RS failed...\n");
    return ret;
    }
    dev_dbg(&spi.dev, "status: 0x%x\n", reg);
    ili922x_display_init(spi);
    ili.power = LCD_POWER_OFF;
    lcd = devm_lcd_device_register(&spi.dev, "ili922xlcd", &spi.dev, ili,
    &ili922x_ops);
    if (IS_ERR(lcd)) {
    dev_err(&spi.dev, "cannot register LCD\n");
    return PTR_ERR(lcd);
    }
    ili.ld = lcd;
    spi_set_drvdata(spi, ili);
    ili922x_lcd_power(ili, LCD_POWER_ON);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ili922x_remove(spi: *mut spi_device) {
    static void ili922x_remove(struct spi_device *spi)
    {
    ili922x_poweroff(spi);
    }
    static struct spi_driver ili922x_driver = {
    .driver = {
    .name = "ili922x",
    },
    .probe = ili922x_probe,
    .remove = ili922x_remove,
    };
    module_spi_driver(ili922x_driver);
    MODULE_AUTHOR("Stefano Babic <sbabic@denx.de>");
    MODULE_DESCRIPTION("ILI9221/9222 LCD driver");
    MODULE_LICENSE("GPL");
    MODULE_PARM_DESC(ili922x_id, "set controller identifier (default=1)");
    MODULE_PARM_DESC(tx_invert, "invert bytes before sending");
