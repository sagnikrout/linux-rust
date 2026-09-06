//! Automatically rewritten from C to Rust
//! Source: drivers/input/keyboard/tca8418_keypad.c
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


//
// Driver for TCA8418 I2C keyboard
//
// Copyright (C) 2011 Fuel7, Inc.  All rights reserved.
//
// Author: Kyle Manna <kyle.manna@fuel7.com>
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public
// License v2 as published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public
// License along with this program; if not, write to the
// Free Software Foundation, Inc., 59 Temple Place - Suite 330,
// Boston, MA 021110-1307, USA.
//
// If you can't comply with GPLv2, alternative licensing terms may be
// arranged. Please contact Fuel7, Inc. (http://fuel7.com/) for proprietary
// alternative licensing inquiries.
//

// TCA8418 hardware limits
pub const TCA8418_MAX_ROWS: c_int = 8;
pub const TCA8418_MAX_COLS: c_int = 10;
// TCA8418 register offsets
pub const REG_CFG: c_uint = 0x01;
pub const REG_INT_STAT: c_uint = 0x02;
pub const REG_KEY_LCK_EC: c_uint = 0x03;
pub const REG_KEY_EVENT_A: c_uint = 0x04;
pub const REG_KEY_EVENT_B: c_uint = 0x05;
pub const REG_KEY_EVENT_C: c_uint = 0x06;
pub const REG_KEY_EVENT_D: c_uint = 0x07;
pub const REG_KEY_EVENT_E: c_uint = 0x08;
pub const REG_KEY_EVENT_F: c_uint = 0x09;
pub const REG_KEY_EVENT_G: c_uint = 0x0A;
pub const REG_KEY_EVENT_H: c_uint = 0x0B;
pub const REG_KEY_EVENT_I: c_uint = 0x0C;
pub const REG_KEY_EVENT_J: c_uint = 0x0D;
pub const REG_KP_LCK_TIMER: c_uint = 0x0E;
pub const REG_UNLOCK1: c_uint = 0x0F;
pub const REG_UNLOCK2: c_uint = 0x10;
pub const REG_GPIO_INT_STAT1: c_uint = 0x11;
pub const REG_GPIO_INT_STAT2: c_uint = 0x12;
pub const REG_GPIO_INT_STAT3: c_uint = 0x13;
pub const REG_GPIO_DAT_STAT1: c_uint = 0x14;
pub const REG_GPIO_DAT_STAT2: c_uint = 0x15;
pub const REG_GPIO_DAT_STAT3: c_uint = 0x16;
pub const REG_GPIO_DAT_OUT1: c_uint = 0x17;
pub const REG_GPIO_DAT_OUT2: c_uint = 0x18;
pub const REG_GPIO_DAT_OUT3: c_uint = 0x19;
pub const REG_GPIO_INT_EN1: c_uint = 0x1A;
pub const REG_GPIO_INT_EN2: c_uint = 0x1B;
pub const REG_GPIO_INT_EN3: c_uint = 0x1C;
pub const REG_KP_GPIO1: c_uint = 0x1D;
pub const REG_KP_GPIO2: c_uint = 0x1E;
pub const REG_KP_GPIO3: c_uint = 0x1F;
pub const REG_GPI_EM1: c_uint = 0x20;
pub const REG_GPI_EM2: c_uint = 0x21;
pub const REG_GPI_EM3: c_uint = 0x22;
pub const REG_GPIO_DIR1: c_uint = 0x23;
pub const REG_GPIO_DIR2: c_uint = 0x24;
pub const REG_GPIO_DIR3: c_uint = 0x25;
pub const REG_GPIO_INT_LVL1: c_uint = 0x26;
pub const REG_GPIO_INT_LVL2: c_uint = 0x27;
pub const REG_GPIO_INT_LVL3: c_uint = 0x28;
pub const REG_DEBOUNCE_DIS1: c_uint = 0x29;
pub const REG_DEBOUNCE_DIS2: c_uint = 0x2A;
pub const REG_DEBOUNCE_DIS3: c_uint = 0x2B;
pub const REG_GPIO_PULL1: c_uint = 0x2C;
pub const REG_GPIO_PULL2: c_uint = 0x2D;
pub const REG_GPIO_PULL3: c_uint = 0x2E;
// TCA8418 bit definitions

// TCA8418 register masks
pub const KEY_LCK_EC_KEC: c_uint = 0x7;
pub const KEY_EVENT_CODE: c_uint = 0x7f;
pub const KEY_EVENT_VALUE: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tca8418_keypad {
    pub client: *mut i2c_client,
    pub input: *mut input_dev,
    pub row_shift: c_uint,
}

//
// Write a byte to the TCA8418
//
    static int tca8418_write_byte(struct tca8418_keypad *keypad_data,
    int reg, u8 val)
    {
    int error;
    error = i2c_smbus_write_byte_data(keypad_data.client, reg, val);
    if (error < 0) {
    dev_err(&keypad_data.client.dev,
    "%s failed, reg: %d, val: %d, error: %d\n",
    __func__, reg, val, error);
    return error;
    }
    return 0;
    }
//
// Read a byte from the TCA8418
//
    static int tca8418_read_byte(struct tca8418_keypad *keypad_data,
    int reg, u8 *val)
    {
    int error;
    error = i2c_smbus_read_byte_data(keypad_data.client, reg);
    if (error < 0) {
    dev_err(&keypad_data.client.dev,
    "%s failed, reg: %d, error: %d\n",
    __func__, reg, error);
    return error;
    }
// val = (u8)error;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tca8418_read_keypad(keypad_data: *mut tca8418_keypad) {
    static void tca8418_read_keypad(struct tca8418_keypad *keypad_data)
    {
    struct input_dev *input = keypad_data.input;
    unsigned short *keymap = input.keycode;
    int error, col, row;
    u8 reg, state, code;
    do {
    error = tca8418_read_byte(keypad_data, REG_KEY_EVENT_A, &reg);
    if (error < 0) {
    dev_err(&keypad_data.client.dev,
    "unable to read REG_KEY_EVENT_A\n");
    break;
    }
// Assume that key code 0 signifies empty FIFO
    if (reg <= 0)
    break;
    state = reg & KEY_EVENT_VALUE;
    code  = reg & KEY_EVENT_CODE;
    row = code / TCA8418_MAX_COLS;
    col = code % TCA8418_MAX_COLS;
    row = (col) ? row : row - 1;
    col = (col) ? col - 1 : TCA8418_MAX_COLS - 1;
    code = MATRIX_SCAN_CODE(row, col, keypad_data.row_shift);
    input_event(input, EV_MSC, MSC_SCAN, code);
    input_report_key(input, keymap[code], state);
    } while (1);
    input_sync(input);
    }
//
// Threaded IRQ handler and this can (and will) sleep.
//
#[no_mangle]
unsafe extern "C" fn tca8418_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t tca8418_irq_handler(int irq, void *dev_id)
    {
    struct tca8418_keypad *keypad_data = dev_id;
    u8 reg;
    int error;
    error = tca8418_read_byte(keypad_data, REG_INT_STAT, &reg);
    if (error) {
    dev_err(&keypad_data.client.dev,
    "unable to read REG_INT_STAT\n");
    return IRQ_NONE;
    }
    if (!reg)
    return IRQ_NONE;
    if (reg & INT_STAT_OVR_FLOW_INT)
    dev_warn(&keypad_data.client.dev, "overflow occurred\n");
    if (reg & INT_STAT_K_INT)
    tca8418_read_keypad(keypad_data);
// Clear all interrupts, even IRQs we didn't check (GPI, CAD, LCK)
    reg = 0xff;
    error = tca8418_write_byte(keypad_data, REG_INT_STAT, reg);
    if (error)
    dev_err(&keypad_data.client.dev,
    "unable to clear REG_INT_STAT\n");
    return IRQ_HANDLED;
    }
//
// Configure the TCA8418 for keypad operation
//
    static int tca8418_configure(struct tca8418_keypad *keypad_data,
    u32 rows, u32 cols)
    {
    int reg, error = 0;
// Assemble a mask for row and column registers
    reg  =  ~(~0 << rows);
    reg += (~(~0 << cols)) << 8;
// Set registers to keypad mode
    error |= tca8418_write_byte(keypad_data, REG_KP_GPIO1, reg);
    error |= tca8418_write_byte(keypad_data, REG_KP_GPIO2, reg >> 8);
    error |= tca8418_write_byte(keypad_data, REG_KP_GPIO3, reg >> 16);
// Enable column debouncing
    error |= tca8418_write_byte(keypad_data, REG_DEBOUNCE_DIS1, reg);
    error |= tca8418_write_byte(keypad_data, REG_DEBOUNCE_DIS2, reg >> 8);
    error |= tca8418_write_byte(keypad_data, REG_DEBOUNCE_DIS3, reg >> 16);
    if (error)
    return error;
    error = tca8418_write_byte(keypad_data, REG_CFG,
    CFG_INT_CFG | CFG_OVR_FLOW_IEN |
    CFG_OVR_FLOW_M | CFG_KE_IEN);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn tca8418_keypad_probe(client: *mut i2c_client) -> c_int {
    static int tca8418_keypad_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct tca8418_keypad *keypad_data;
    struct input_dev *input;
    let mut rows: u32 = 0, cols = 0;
    int error, row_shift;
    u8 reg;
// Check i2c driver capabilities
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_SMBUS_BYTE)) {
    dev_err(dev, "%s adapter not supported\n",
    dev_driver_string(&client.adapter.dev));
    return -ENODEV;
    }
    error = matrix_keypad_parse_properties(dev, &rows, &cols);
    if (error)
    return error;
    if (!rows || rows > TCA8418_MAX_ROWS) {
    dev_err(dev, "invalid rows\n");
    return -EINVAL;
    }
    if (!cols || cols > TCA8418_MAX_COLS) {
    dev_err(dev, "invalid columns\n");
    return -EINVAL;
    }
    row_shift = get_count_order(cols);
// Allocate memory for keypad_data and input device
    keypad_data = devm_kzalloc(dev, sizeof(*keypad_data), GFP_KERNEL);
    if (!keypad_data)
    return -ENOMEM;
    keypad_data.client = client;
    keypad_data.row_shift = row_shift;
// Read key lock register, if this fails assume device not present
    error = tca8418_read_byte(keypad_data, REG_KEY_LCK_EC, &reg);
    if (error)
    return -ENODEV;
// Configure input device
    input = devm_input_allocate_device(dev);
    if (!input)
    return -ENOMEM;
    keypad_data.input = input;
    input.name = client.name;
    input.id.bustype = BUS_I2C;
    input.id.vendor  = 0x0001;
    input.id.product = 0x001;
    input.id.version = 0x0001;
    error = matrix_keypad_build_keymap(core::ptr::null_mut(), core::ptr::null_mut(), rows, cols, core::ptr::null_mut(), input);
    if (error) {
    dev_err(dev, "Failed to build keymap\n");
    return error;
    }
    if (device_property_read_bool(dev, "keypad,autorepeat"))
    __set_bit(EV_REP, input.evbit);
    input_set_capability(input, EV_MSC, MSC_SCAN);
    error = devm_request_threaded_irq(dev, client.irq,
    core::ptr::null_mut(), tca8418_irq_handler,
    IRQF_SHARED | IRQF_ONESHOT,
    client.name, keypad_data);
    if (error) {
    dev_err(dev, "Unable to claim irq %d; error %d\n",
    client.irq, error);
    return error;
    }
// Initialize the chip
    error = tca8418_configure(keypad_data, rows, cols);
    if (error < 0)
    return error;
    error = input_register_device(input);
    if (error) {
    dev_err(dev, "Unable to register input device, error: %d\n",
    error);
    return error;
    }
    return 0;
    }
    static const struct i2c_device_id tca8418_id[] = {
    { .name = "tca8418", .driver_data = 8418 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, tca8418_id);
    static const struct of_device_id tca8418_dt_ids[] = {
    { .compatible = "ti,tca8418", },
    { }
    };
    MODULE_DEVICE_TABLE(of, tca8418_dt_ids);
    static struct i2c_driver tca8418_keypad_driver = {
    .driver = {
    .name	= "tca8418_keypad",
    .of_match_table = tca8418_dt_ids,
    },
    .probe		= tca8418_keypad_probe,
    .id_table	= tca8418_id,
    };
    module_i2c_driver(tca8418_keypad_driver);
    MODULE_AUTHOR("Kyle Manna <kyle.manna@fuel7.com>");
    MODULE_DESCRIPTION("Keypad driver for TCA8418");
    MODULE_LICENSE("GPL");
