//! Automatically rewritten from C to Rust
//! Source: drivers/input/keyboard/max7359_keypad.c
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
// max7359_keypad.c - MAX7359 Key Switch Controller Driver
//
// Copyright (C) 2009 Samsung Electronics
// Kim Kyuwon <q1.kim@samsung.com>
//
// Based on pxa27x_keypad.c
//
// Datasheet: http://www.maxim-ic.com/quick_view2.cfm/qv_pk/5456
//

pub const MAX7359_MAX_KEY_ROWS: c_int = 8;
pub const MAX7359_MAX_KEY_COLS: c_int = 8;

pub const MAX7359_ROW_SHIFT: c_int = 3;
//
// MAX7359 registers
//
pub const MAX7359_REG_KEYFIFO: c_uint = 0x00;
pub const MAX7359_REG_CONFIG: c_uint = 0x01;
pub const MAX7359_REG_DEBOUNCE: c_uint = 0x02;
pub const MAX7359_REG_INTERRUPT: c_uint = 0x03;
pub const MAX7359_REG_PORTS: c_uint = 0x04;
pub const MAX7359_REG_KEYREP: c_uint = 0x05;
pub const MAX7359_REG_SLEEP: c_uint = 0x06;
//
// Configuration register bits
//

//
// Autosleep register values (ms)
//
pub const MAX7359_AUTOSLEEP_8192: c_uint = 0x01;
pub const MAX7359_AUTOSLEEP_4096: c_uint = 0x02;
pub const MAX7359_AUTOSLEEP_2048: c_uint = 0x03;
pub const MAX7359_AUTOSLEEP_1024: c_uint = 0x04;
pub const MAX7359_AUTOSLEEP_512: c_uint = 0x05;
pub const MAX7359_AUTOSLEEP_256: c_uint = 0x06;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max7359_keypad {
// matrix key code map
    pub keycodes: [c_ushort; MAX7359_MAX_KEY_NUM],
    pub input_dev: *mut input_dev,
    pub client: *mut i2c_client,
}

#[no_mangle]
unsafe extern "C" fn max7359_write_reg(client: *mut i2c_client, reg: u8, val: u8) -> c_int {
    static int max7359_write_reg(struct i2c_client *client, u8 reg, u8 val)
    {
    let mut ret: c_int = i2c_smbus_write_byte_data(client, reg, val);
    if (ret < 0)
    dev_err(&client.dev, "%s: reg 0x%x, val 0x%x, err %d\n",
    __func__, reg, val, ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn max7359_read_reg(client: *mut i2c_client, reg: c_int) -> c_int {
    static int max7359_read_reg(struct i2c_client *client, int reg)
    {
    let mut ret: c_int = i2c_smbus_read_byte_data(client, reg);
    if (ret < 0)
    dev_err(&client.dev, "%s: reg 0x%x, err %d\n",
    __func__, reg, ret);
    return ret;
    }
// runs in an IRQ thread -- can (and will!) sleep
#[no_mangle]
unsafe extern "C" fn max7359_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t max7359_interrupt(int irq, void *dev_id)
    {
    struct max7359_keypad *keypad = dev_id;
    struct input_dev *input_dev = keypad.input_dev;
    int val, row, col, release, code;
    val = max7359_read_reg(keypad.client, MAX7359_REG_KEYFIFO);
    row = val & 0x7;
    col = (val >> 3) & 0x7;
    release = val & 0x40;
    code = MATRIX_SCAN_CODE(row, col, MAX7359_ROW_SHIFT);
    dev_dbg(&keypad.client.dev,
    "key[%d:%d] %s\n", row, col, release ? "release" : "press");
    input_event(input_dev, EV_MSC, MSC_SCAN, code);
    input_report_key(input_dev, keypad.keycodes[code], !release);
    input_sync(input_dev);
    return IRQ_HANDLED;
    }
//
// Let MAX7359 fall into a deep sleep:
// If no keys are pressed, enter sleep mode for 8192 ms. And if any
// key is pressed, the MAX7359 returns to normal operating mode.
//
#[no_mangle]
pub unsafe extern "C" fn max7359_fall_deepsleep(client: *mut i2c_client) {
    static inline void max7359_fall_deepsleep(struct i2c_client *client)
    {
    max7359_write_reg(client, MAX7359_REG_SLEEP, MAX7359_AUTOSLEEP_8192);
    }
//
// Let MAX7359 take a catnap:
// Autosleep just for 256 ms.
//
#[no_mangle]
pub unsafe extern "C" fn max7359_take_catnap(client: *mut i2c_client) {
    static inline void max7359_take_catnap(struct i2c_client *client)
    {
    max7359_write_reg(client, MAX7359_REG_SLEEP, MAX7359_AUTOSLEEP_256);
    }
#[no_mangle]
unsafe extern "C" fn max7359_open(dev: *mut input_dev) -> c_int {
    static int max7359_open(struct input_dev *dev)
    {
    struct max7359_keypad *keypad = input_get_drvdata(dev);
    max7359_take_catnap(keypad.client);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max7359_close(dev: *mut input_dev) {
    static void max7359_close(struct input_dev *dev)
    {
    struct max7359_keypad *keypad = input_get_drvdata(dev);
    max7359_fall_deepsleep(keypad.client);
    }
#[no_mangle]
unsafe extern "C" fn max7359_initialize(client: *mut i2c_client) {
    static void max7359_initialize(struct i2c_client *client)
    {
    max7359_write_reg(client, MAX7359_REG_CONFIG,
    MAX7359_CFG_KEY_RELEASE | /* Key release enable */
    MAX7359_CFG_WAKEUP); /* Key press wakeup enable */
// Full key-scan functionality
    max7359_write_reg(client, MAX7359_REG_DEBOUNCE, 0x1F);
// nINT asserts every debounce cycles
    max7359_write_reg(client, MAX7359_REG_INTERRUPT, 0x01);
    max7359_fall_deepsleep(client);
    }
#[no_mangle]
unsafe extern "C" fn max7359_probe(client: *mut i2c_client) -> c_int {
    static int max7359_probe(struct i2c_client *client)
    {
    const struct matrix_keymap_data *keymap_data =
    dev_get_platdata(&client.dev);
    struct max7359_keypad *keypad;
    struct input_dev *input_dev;
    int ret;
    int error;
    if (!client.irq) {
    dev_err(&client.dev, "The irq number should not be zero\n");
    return -EINVAL;
    }
// Detect MAX7359: The initial Keys FIFO value is '0x3F'
    ret = max7359_read_reg(client, MAX7359_REG_KEYFIFO);
    if (ret < 0) {
    dev_err(&client.dev, "failed to detect device\n");
    return -ENODEV;
    }
    dev_dbg(&client.dev, "keys FIFO is 0x%02x\n", ret);
    keypad = devm_kzalloc(&client.dev, sizeof(struct max7359_keypad),
    GFP_KERNEL);
    if (!keypad) {
    dev_err(&client.dev, "failed to allocate memory\n");
    return -ENOMEM;
    }
    input_dev = devm_input_allocate_device(&client.dev);
    if (!input_dev) {
    dev_err(&client.dev, "failed to allocate input device\n");
    return -ENOMEM;
    }
    keypad.client = client;
    keypad.input_dev = input_dev;
    input_dev.name = client.name;
    input_dev.id.bustype = BUS_I2C;
    input_dev.open = max7359_open;
    input_dev.close = max7359_close;
    input_dev.dev.parent = &client.dev;
    input_dev.evbit[0] = BIT_MASK(EV_KEY) | BIT_MASK(EV_REP);
    input_dev.keycodesize = sizeof(keypad.keycodes[0]);
    input_dev.keycodemax = ARRAY_SIZE(keypad.keycodes);
    input_dev.keycode = keypad.keycodes;
    input_set_capability(input_dev, EV_MSC, MSC_SCAN);
    input_set_drvdata(input_dev, keypad);
    error = matrix_keypad_build_keymap(keymap_data, core::ptr::null_mut(),
    MAX7359_MAX_KEY_ROWS,
    MAX7359_MAX_KEY_COLS,
    keypad.keycodes,
    input_dev);
    if (error) {
    dev_err(&client.dev, "failed to build keymap\n");
    return error;
    }
    error = devm_request_threaded_irq(&client.dev, client.irq, core::ptr::null_mut(),
    max7359_interrupt,
    IRQF_TRIGGER_LOW | IRQF_ONESHOT,
    client.name, keypad);
    if (error) {
    dev_err(&client.dev, "failed to register interrupt\n");
    return error;
    }
// Register the input device
    error = input_register_device(input_dev);
    if (error) {
    dev_err(&client.dev, "failed to register input device\n");
    return error;
    }
// Initialize MAX7359
    max7359_initialize(client);
    device_init_wakeup(&client.dev, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max7359_suspend(dev: *mut device) -> c_int {
    static int max7359_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    max7359_fall_deepsleep(client);
    if (device_may_wakeup(&client.dev))
    enable_irq_wake(client.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max7359_resume(dev: *mut device) -> c_int {
    static int max7359_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    if (device_may_wakeup(&client.dev))
    disable_irq_wake(client.irq);
// Restore the default setting
    max7359_take_catnap(client);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(max7359_pm, max7359_suspend, max7359_resume);
    static const struct i2c_device_id max7359_ids[] = {
    { .name = "max7359" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, max7359_ids);
    static struct i2c_driver max7359_i2c_driver = {
    .driver = {
    .name = "max7359",
    .pm   = pm_sleep_ptr(&max7359_pm),
    },
    .probe		= max7359_probe,
    .id_table	= max7359_ids,
    };
    module_i2c_driver(max7359_i2c_driver);
    MODULE_AUTHOR("Kim Kyuwon <q1.kim@samsung.com>");
    MODULE_DESCRIPTION("MAX7359 Key Switch Controller Driver");
    MODULE_LICENSE("GPL v2");
