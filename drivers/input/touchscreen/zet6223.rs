//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/zet6223.c
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
// Copyright (C) 2016, Jelle van der Waa <jelle@vdwaa.nl>
//

pub const ZET6223_MAX_FINGERS: c_int = 16;

pub const ZET6223_CMD_INFO: c_uint = 0xB2;
pub const ZET6223_CMD_INFO_LENGTH: c_int = 17;
pub const ZET6223_VALID_PACKET: c_uint = 0x3c;
pub const ZET6223_POWER_ON_DELAY_MSEC: c_int = 30;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zet6223_ts {
    pub client: *mut i2c_client,
    pub input: *mut input_dev,
    pub prop: touchscreen_properties,
    pub supplies: [regulator_bulk_data; 2],
    pub max_x: u16,
    pub max_y: u16,
    pub fingernum: u8,
}

#[no_mangle]
unsafe extern "C" fn zet6223_start(dev: *mut input_dev) -> c_int {
    static int zet6223_start(struct input_dev *dev)
    {
    struct zet6223_ts *ts = input_get_drvdata(dev);
    enable_irq(ts.client.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn zet6223_stop(dev: *mut input_dev) {
    static void zet6223_stop(struct input_dev *dev)
    {
    struct zet6223_ts *ts = input_get_drvdata(dev);
    disable_irq(ts.client.irq);
    }
#[no_mangle]
unsafe extern "C" fn zet6223_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t zet6223_irq(int irq, void *dev_id)
    {
    struct zet6223_ts *ts = dev_id;
    u16 finger_bits;
//
// First 3 bytes are an identifier, two bytes of finger data.
// X, Y data per finger is 4 bytes.
//
    let mut bufsize: u8 = 3 + 4 * ts.fingernum;
    u8 buf[ZET6223_MAX_PKT_SIZE];
    int i;
    int ret;
    int error;
    ret = i2c_master_recv(ts.client, buf, bufsize);
    if (ret != bufsize) {
    error = ret < 0 ? ret : -EIO;
    dev_err_ratelimited(&ts.client.dev,
    "Error reading input data: %d\n", error);
    return IRQ_HANDLED;
    }
    if (buf[0] != ZET6223_VALID_PACKET)
    return IRQ_HANDLED;
    finger_bits = get_unaligned_be16(buf + 1);
    for (i = 0; i < ts.fingernum; i++) {
    if (!(finger_bits & BIT(15 - i)))
    continue;
    input_mt_slot(ts.input, i);
    input_mt_report_slot_state(ts.input, MT_TOOL_FINGER, true);
    input_event(ts.input, EV_ABS, ABS_MT_POSITION_X,
    ((buf[i + 3] >> 4) << 8) + buf[i + 4]);
    input_event(ts.input, EV_ABS, ABS_MT_POSITION_Y,
    ((buf[i + 3] & 0xF) << 8) + buf[i + 5]);
    }
    input_mt_sync_frame(ts.input);
    input_sync(ts.input);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn zet6223_power_off(_ts: *mut c_void) {
    static void zet6223_power_off(void *_ts)
    {
    struct zet6223_ts *ts = _ts;
    regulator_bulk_disable(ARRAY_SIZE(ts.supplies), ts.supplies);
    }
#[no_mangle]
unsafe extern "C" fn zet6223_power_on(ts: *mut zet6223_ts) -> c_int {
    static int zet6223_power_on(struct zet6223_ts *ts)
    {
    struct device *dev = &ts.client.dev;
    int error;
    ts.supplies[0].supply = "vio";
    ts.supplies[1].supply = "vcc";
    error = devm_regulator_bulk_get(dev, ARRAY_SIZE(ts.supplies),
    ts.supplies);
    if (error)
    return error;
    error = regulator_bulk_enable(ARRAY_SIZE(ts.supplies), ts.supplies);
    if (error)
    return error;
    msleep(ZET6223_POWER_ON_DELAY_MSEC);
    error = devm_add_action_or_reset(dev, zet6223_power_off, ts);
    if (error) {
    dev_err(dev, "failed to install poweroff action: %d\n", error);
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn zet6223_query_device(ts: *mut zet6223_ts) -> c_int {
    static int zet6223_query_device(struct zet6223_ts *ts)
    {
    u8 buf[ZET6223_CMD_INFO_LENGTH];
    let mut cmd: u8 = ZET6223_CMD_INFO;
    int ret;
    int error;
    ret = i2c_master_send(ts.client, &cmd, sizeof(cmd));
    if (ret != sizeof(cmd)) {
    error = ret < 0 ? ret : -EIO;
    dev_err(&ts.client.dev,
    "touchpanel info cmd failed: %d\n", error);
    return error;
    }
    ret = i2c_master_recv(ts.client, buf, sizeof(buf));
    if (ret != sizeof(buf)) {
    error = ret < 0 ? ret : -EIO;
    dev_err(&ts.client.dev,
    "failed to retrieve touchpanel info: %d\n", error);
    return error;
    }
    ts.fingernum = buf[15] & 0x7F;
    if (ts.fingernum > ZET6223_MAX_FINGERS) {
    dev_warn(&ts.client.dev,
    "touchpanel reports %d fingers, limiting to %d\n",
    ts.fingernum, ZET6223_MAX_FINGERS);
    ts.fingernum = ZET6223_MAX_FINGERS;
    }
    ts.max_x = get_unaligned_le16(&buf[8]);
    ts.max_y = get_unaligned_le16(&buf[10]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn zet6223_probe(client: *mut i2c_client) -> c_int {
    static int zet6223_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct zet6223_ts *ts;
    struct input_dev *input;
    int error;
    if (!client.irq) {
    dev_err(dev, "no irq specified\n");
    return -EINVAL;
    }
    ts = devm_kzalloc(dev, sizeof(*ts), GFP_KERNEL);
    if (!ts)
    return -ENOMEM;
    ts.client = client;
    error = zet6223_power_on(ts);
    if (error)
    return error;
    error = zet6223_query_device(ts);
    if (error)
    return error;
    ts.input = input = devm_input_allocate_device(dev);
    if (!input)
    return -ENOMEM;
    input_set_drvdata(input, ts);
    input.name = client.name;
    input.id.bustype = BUS_I2C;
    input.open = zet6223_start;
    input.close = zet6223_stop;
    input_set_abs_params(input, ABS_MT_POSITION_X, 0, ts.max_x, 0, 0);
    input_set_abs_params(input, ABS_MT_POSITION_Y, 0, ts.max_y, 0, 0);
    touchscreen_parse_properties(input, true, &ts.prop);
    error = input_mt_init_slots(input, ts.fingernum,
    INPUT_MT_DIRECT | INPUT_MT_DROP_UNUSED);
    if (error)
    return error;
    error = devm_request_threaded_irq(dev, client.irq, core::ptr::null_mut(), zet6223_irq,
    IRQF_ONESHOT, client.name, ts);
    if (error) {
    dev_err(dev, "failed to request irq %d: %d\n",
    client.irq, error);
    return error;
    }
    zet6223_stop(input);
    error = input_register_device(input);
    if (error)
    return error;
    return 0;
    }
    static const struct of_device_id zet6223_of_match[] = {
    { .compatible = "zeitec,zet6223" },
    { }
    };
    MODULE_DEVICE_TABLE(of, zet6223_of_match);
    static const struct i2c_device_id zet6223_id[] = {
    { .name = "zet6223" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, zet6223_id);
    static struct i2c_driver zet6223_driver = {
    .driver = {
    .name = "zet6223",
    .of_match_table = zet6223_of_match,
    },
    .probe = zet6223_probe,
    .id_table = zet6223_id
    };
    module_i2c_driver(zet6223_driver);
    MODULE_AUTHOR("Jelle van der Waa <jelle@vdwaa.nl>");
    MODULE_DESCRIPTION("ZEITEC zet622x I2C touchscreen driver");
    MODULE_LICENSE("GPL");
