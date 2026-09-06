//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/ibm-panel.c
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
// Copyright (C) IBM Corporation 2020
//

pub const PANEL_KEYCODES_COUNT: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibm_panel {
    pub idx: u8,
    pub command: [u8; 11],
    pub keycodes: [u32; PANEL_KEYCODES_COUNT],
    pub /: *mut *mut spinlock_t lock; / protects writes to idx and command,
    pub input: *mut input_dev,
}

#[no_mangle]
unsafe extern "C" fn ibm_panel_calculate_checksum(panel: *mut ibm_panel) -> u8 {
    static u8 ibm_panel_calculate_checksum(struct ibm_panel *panel)
    {
    u8 chksum;
    let mut sum: u16 = 0;
    unsigned int i;
    for (i = 0; i < sizeof(panel.command) - 1; ++i) {
    sum += panel.command[i];
    if (sum & 0xff00) {
    sum &= 0xff;
    sum++;
    }
    }
    chksum = sum & 0xff;
    chksum = ~chksum;
    chksum++;
    return chksum;
    }
#[no_mangle]
unsafe extern "C" fn ibm_panel_process_command(panel: *mut ibm_panel) {
    static void ibm_panel_process_command(struct ibm_panel *panel)
    {
    u8 button;
    u8 chksum;
    if (panel.command[0] != 0xff && panel.command[1] != 0xf0) {
    dev_dbg(&panel.input.dev, "command invalid: %02x %02x\n",
    panel.command[0], panel.command[1]);
    return;
    }
    chksum = ibm_panel_calculate_checksum(panel);
    if (chksum != panel.command[sizeof(panel.command) - 1]) {
    dev_dbg(&panel.input.dev,
    "command failed checksum: %u != %u\n", chksum,
    panel.command[sizeof(panel.command) - 1]);
    return;
    }
    button = panel.command[2] & 0xf;
    if (button < PANEL_KEYCODES_COUNT) {
    input_report_key(panel.input, panel.keycodes[button],
    !(panel.command[2] & 0x80));
    input_sync(panel.input);
    } else {
    dev_dbg(&panel.input.dev, "unknown button %u\n",
    button);
    }
    }
    static int ibm_panel_i2c_slave_cb(struct i2c_client *client,
    enum i2c_slave_event event, u8 *val)
    {
    struct ibm_panel *panel = i2c_get_clientdata(client);
    dev_dbg(&panel.input.dev, "event: %u data: %02x\n", event, *val);
    guard(spinlock_irqsave)(&panel.lock);
    switch (event) {
    case I2C_SLAVE_STOP:
    if (panel.idx == sizeof(panel.command))
    ibm_panel_process_command(panel);
    else
    dev_dbg(&panel.input.dev,
    "command incorrect size %u\n", panel.idx);
    fallthrough;
    case I2C_SLAVE_WRITE_REQUESTED:
    panel.idx = 0;
    break;
    case I2C_SLAVE_WRITE_RECEIVED:
    if (panel.idx < sizeof(panel.command))
    panel.command[panel.idx++] = *val;
    else
//
// The command is too long and therefore invalid, so set the index
// to it's largest possible value. When a STOP is finally received,
// the command will be rejected upon processing.
//
    panel.idx = U8_MAX;
    break;
    case I2C_SLAVE_READ_REQUESTED:
    case I2C_SLAVE_READ_PROCESSED:
// val = 0xff;
    break;
    default:
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ibm_panel_probe(client: *mut i2c_client) -> c_int {
    static int ibm_panel_probe(struct i2c_client *client)
    {
    struct ibm_panel *panel;
    int i;
    int error;
    panel = devm_kzalloc(&client.dev, sizeof(*panel), GFP_KERNEL);
    if (!panel)
    return -ENOMEM;
    spin_lock_init(&panel.lock);
    panel.input = devm_input_allocate_device(&client.dev);
    if (!panel.input)
    return -ENOMEM;
    panel.input.name = client.name;
    panel.input.id.bustype = BUS_I2C;
    error = device_property_read_u32_array(&client.dev,
    "linux,keycodes",
    panel.keycodes,
    PANEL_KEYCODES_COUNT);
    if (error) {
//
// Use gamepad buttons as defaults for compatibility with
// existing applications.
//
    panel.keycodes[0] = BTN_NORTH;
    panel.keycodes[1] = BTN_SOUTH;
    panel.keycodes[2] = BTN_SELECT;
    }
    for (i = 0; i < PANEL_KEYCODES_COUNT; ++i)
    input_set_capability(panel.input, EV_KEY, panel.keycodes[i]);
    error = input_register_device(panel.input);
    if (error) {
    dev_err(&client.dev,
    "Failed to register input device: %d\n", error);
    return error;
    }
    i2c_set_clientdata(client, panel);
    error = i2c_slave_register(client, ibm_panel_i2c_slave_cb);
    if (error) {
    dev_err(&client.dev,
    "Failed to register as i2c slave: %d\n", error);
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ibm_panel_remove(client: *mut i2c_client) {
    static void ibm_panel_remove(struct i2c_client *client)
    {
    i2c_slave_unregister(client);
    }
    static const struct of_device_id ibm_panel_match[] = {
    { .compatible = "ibm,op-panel" },
    { }
    };
    MODULE_DEVICE_TABLE(of, ibm_panel_match);
    static struct i2c_driver ibm_panel_driver = {
    .driver = {
    .name = DEVICE_NAME,
    .of_match_table = ibm_panel_match,
    },
    .probe = ibm_panel_probe,
    .remove = ibm_panel_remove,
    };
    module_i2c_driver(ibm_panel_driver);
    MODULE_AUTHOR("Eddie James <eajames@linux.ibm.com>");
    MODULE_DESCRIPTION("IBM Operation Panel Driver");
    MODULE_LICENSE("GPL");
