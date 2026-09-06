//! Automatically rewritten from C to Rust
//! Source: drivers/input/keyboard/dlink-dir685-touchkeys.c
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
// D-Link DIR-685 router I2C-based Touchkeys input driver
// Copyright (C) 2017 Linus Walleij <linus.walleij@linaro.org>
//
// This is a one-off touchkey controller based on the Cypress Semiconductor
// CY8C214 MCU with some firmware in its internal 8KB flash. The circuit
// board inside the router is named E119921
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dir685_touchkeys {
    pub dev: *mut device,
    pub client: *mut i2c_client,
    pub input: *mut input_dev,
    pub cur_key: c_ulong,
    pub codes: [u16; 7],
}

#[no_mangle]
unsafe extern "C" fn dir685_tk_irq_thread(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t dir685_tk_irq_thread(int irq, void *data)
    {
    struct dir685_touchkeys *tk = data;
    let mut num_bits: c_int = min_t(int, ARRAY_SIZE(tk.codes), 16);
    unsigned long changed;
    u8 buf[6];
    unsigned long key;
    int i;
    int err;
    memset(buf, 0, sizeof(buf));
    err = i2c_master_recv(tk.client, buf, sizeof(buf));
    if (err != sizeof(buf)) {
    dev_err(tk.dev, "short read %d\n", err);
    return IRQ_HANDLED;
    }
    dev_dbg(tk.dev, "IN: %*ph\n", (int)sizeof(buf), buf);
    key = be16_to_cpup((__be16 *) &buf[4]);
// Figure out if any bits went high or low since last message
    changed = tk.cur_key ^ key;
    for_each_set_bit(i, &changed, num_bits) {
    dev_dbg(tk.dev, "key %d is %s\n", i,
    str_down_up(test_bit(i, &key)));
    input_report_key(tk.input, tk.codes[i], test_bit(i, &key));
    }
// Store currently down keys
    tk.cur_key = key;
    input_sync(tk.input);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn dir685_tk_probe(client: *mut i2c_client) -> c_int {
    static int dir685_tk_probe(struct i2c_client *client)
    {
    static const u8 bl_data[] = { 0xa7, 0x40 };
    struct device *dev = &client.dev;
    struct dir685_touchkeys *tk;
    int err;
    int i;
    tk = devm_kzalloc(&client.dev, sizeof(*tk), GFP_KERNEL);
    if (!tk)
    return -ENOMEM;
    tk.input = devm_input_allocate_device(dev);
    if (!tk.input)
    return -ENOMEM;
    tk.client = client;
    tk.dev = dev;
    tk.input.keycodesize = sizeof(u16);
    tk.input.keycodemax = ARRAY_SIZE(tk.codes);
    tk.input.keycode = tk.codes;
    tk.codes[0] = KEY_UP;
    tk.codes[1] = KEY_DOWN;
    tk.codes[2] = KEY_LEFT;
    tk.codes[3] = KEY_RIGHT;
    tk.codes[4] = KEY_ENTER;
    tk.codes[5] = KEY_WPS_BUTTON;
//
// This key appears in the vendor driver, but I have
// not been able to activate it.
//
    tk.codes[6] = KEY_RESERVED;
    __set_bit(EV_KEY, tk.input.evbit);
    for (i = 0; i < ARRAY_SIZE(tk.codes); i++)
    __set_bit(tk.codes[i], tk.input.keybit);
    __clear_bit(KEY_RESERVED, tk.input.keybit);
    tk.input.name = "D-Link DIR-685 touchkeys";
    tk.input.id.bustype = BUS_I2C;
    err = input_register_device(tk.input);
    if (err)
    return err;
// Set the brightness to max level
    err = i2c_master_send(client, bl_data, sizeof(bl_data));
    if (err != sizeof(bl_data))
    dev_warn(tk.dev, "error setting brightness level\n");
    if (!client.irq) {
    dev_err(dev, "no IRQ on the I2C device\n");
    return -ENODEV;
    }
    err = devm_request_threaded_irq(dev, client.irq,
    core::ptr::null_mut(), dir685_tk_irq_thread,
    IRQF_ONESHOT,
    "dir685-tk", tk);
    if (err) {
    dev_err(dev, "can't request IRQ\n");
    return err;
    }
    return 0;
    }
    static const struct i2c_device_id dir685_tk_id[] = {
    { .name = "dir685tk" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, dir685_tk_id);

    static const struct of_device_id dir685_tk_of_match[] = {
    { .compatible = "dlink,dir685-touchkeys" },
    {},
    };
    MODULE_DEVICE_TABLE(of, dir685_tk_of_match);

    static struct i2c_driver dir685_tk_i2c_driver = {
    .driver = {
    .name	= "dlink-dir685-touchkeys",
    .of_match_table = of_match_ptr(dir685_tk_of_match),
    },
    .probe		= dir685_tk_probe,
    .id_table	= dir685_tk_id,
    };
    module_i2c_driver(dir685_tk_i2c_driver);
    MODULE_AUTHOR("Linus Walleij <linus.walleij@linaro.org>");
    MODULE_DESCRIPTION("D-Link DIR-685 touchkeys driver");
    MODULE_LICENSE("GPL");
