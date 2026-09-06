//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/himax_hx83112b.c
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
// Driver for Himax hx83112b touchscreens
//
// Copyright (C) 2022 Job Noorman <job@noorman.info>
//
// HX83100A support
// Copyright (C) 2024 Felix Kaechele <felix@kaechele.ca>
//
// This code is based on "Himax Android Driver Sample Code for QCT platform":
//
// Copyright (C) 2017 Himax Corporation.
//

pub const HIMAX_MAX_POINTS: c_int = 10;
pub const HIMAX_AHB_ADDR_BYTE_0: c_uint = 0x00;
pub const HIMAX_AHB_ADDR_RDATA_BYTE_0: c_uint = 0x08;
pub const HIMAX_AHB_ADDR_ACCESS_DIRECTION: c_uint = 0x0c;
pub const HIMAX_AHB_ADDR_INCR4: c_uint = 0x0d;
pub const HIMAX_AHB_ADDR_CONTI: c_uint = 0x13;
pub const HIMAX_AHB_ADDR_EVENT_STACK: c_uint = 0x30;
pub const HIMAX_AHB_CMD_ACCESS_DIRECTION_READ: c_uint = 0x00;
pub const HIMAX_AHB_CMD_INCR4: c_uint = 0x10;
pub const HIMAX_AHB_CMD_CONTI: c_uint = 0x31;
pub const HIMAX_REG_ADDR_ICID: c_uint = 0x900000d0;
pub const HX83100A_REG_FW_EVENT_STACK: c_uint = 0x90060000;
pub const HIMAX_INVALID_COORD: c_uint = 0xffff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct himax_event_point {
    pub x: __be16,
    pub y: __be16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct himax_event {
    pub points: [himax_event_point; HIMAX_MAX_POINTS],
    pub majors: [u8; HIMAX_MAX_POINTS],
    pub pad0: [u8; 2],
    pub num_points: u8,
    pub pad1: [u8; 2],
    pub checksum_fix: u8,
    pub __packed: },
    pub 56): static_assert(sizeof(struct himax_event) ==,
    pub himax_ts_data: struct,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct himax_chip {
    pub id: u32,
    pub ts): *mut *mut int (check_id)(struct himax_ts_data,
    int (*read_events)(struct himax_ts_data *ts, struct himax_event *event,
    pub length): usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct himax_ts_data {
    pub chip: *const himax_chip,
    pub gpiod_rst: *mut gpio_desc,
    pub input_dev: *mut input_dev,
    pub client: *mut i2c_client,
    pub regmap: *mut regmap,
    pub props: touchscreen_properties,
}

    static const struct regmap_config himax_regmap_config = {
    .reg_bits = 8,
    .val_bits = 32,
    .val_format_endian = REGMAP_ENDIAN_LITTLE,
    };
#[no_mangle]
unsafe extern "C" fn himax_bus_enable_burst(ts: *mut himax_ts_data) -> c_int {
    static int himax_bus_enable_burst(struct himax_ts_data *ts)
    {
    int error;
    error = regmap_write(ts.regmap, HIMAX_AHB_ADDR_CONTI,
    HIMAX_AHB_CMD_CONTI);
    if (error)
    return error;
    error = regmap_write(ts.regmap, HIMAX_AHB_ADDR_INCR4,
    HIMAX_AHB_CMD_INCR4);
    if (error)
    return error;
    return 0;
    }
    static int himax_bus_read(struct himax_ts_data *ts, u32 address, void *dst,
    size_t length)
    {
    int error;
    if (length > 4) {
    error = himax_bus_enable_burst(ts);
    if (error)
    return error;
    }
    error = regmap_write(ts.regmap, HIMAX_AHB_ADDR_BYTE_0, address);
    if (error)
    return error;
    error = regmap_write(ts.regmap, HIMAX_AHB_ADDR_ACCESS_DIRECTION,
    HIMAX_AHB_CMD_ACCESS_DIRECTION_READ);
    if (error)
    return error;
    if (length > 4)
    error = regmap_noinc_read(ts.regmap, HIMAX_AHB_ADDR_RDATA_BYTE_0,
    dst, length);
    else
    error = regmap_read(ts.regmap, HIMAX_AHB_ADDR_RDATA_BYTE_0,
    dst);
    if (error)
    return error;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn himax_reset(ts: *mut himax_ts_data) {
    static void himax_reset(struct himax_ts_data *ts)
    {
    gpiod_set_value_cansleep(ts.gpiod_rst, 1);
// Delay copied from downstream driver
    msleep(20);
    gpiod_set_value_cansleep(ts.gpiod_rst, 0);
//
// The downstream driver doesn't contain this delay but is seems safer
// to include it. The range is just a guess that seems to work well.
//
    usleep_range(1000, 1100);
    }
#[no_mangle]
unsafe extern "C" fn himax_read_product_id(ts: *mut himax_ts_data, product_id: *mut u32) -> c_int {
    static int himax_read_product_id(struct himax_ts_data *ts, u32 *product_id)
    {
    int error;
    error = himax_bus_read(ts, HIMAX_REG_ADDR_ICID, product_id,
    sizeof(*product_id));
    if (error)
    return error;
// product_id >>= 8;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn himax_check_product_id(ts: *mut himax_ts_data) -> c_int {
    static int himax_check_product_id(struct himax_ts_data *ts)
    {
    int error;
    u32 product_id;
    error = himax_read_product_id(ts, &product_id);
    if (error)
    return error;
    dev_dbg(&ts.client.dev, "Product id: %x\n", product_id);
    if (product_id == ts.chip.id)
    return 0;
    dev_err(&ts.client.dev, "Unknown product id: %x\n",
    product_id);
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn himax_input_register(ts: *mut himax_ts_data) -> c_int {
    static int himax_input_register(struct himax_ts_data *ts)
    {
    int error;
    ts.input_dev = devm_input_allocate_device(&ts.client.dev);
    if (!ts.input_dev) {
    dev_err(&ts.client.dev, "Failed to allocate input device\n");
    return -ENOMEM;
    }
    ts.input_dev.name = "Himax Touchscreen";
    input_set_capability(ts.input_dev, EV_ABS, ABS_MT_POSITION_X);
    input_set_capability(ts.input_dev, EV_ABS, ABS_MT_POSITION_Y);
    input_set_abs_params(ts.input_dev, ABS_MT_WIDTH_MAJOR, 0, 200, 0, 0);
    input_set_abs_params(ts.input_dev, ABS_MT_TOUCH_MAJOR, 0, 200, 0, 0);
    touchscreen_parse_properties(ts.input_dev, true, &ts.props);
    error = input_mt_init_slots(ts.input_dev, HIMAX_MAX_POINTS,
    INPUT_MT_DIRECT | INPUT_MT_DROP_UNUSED);
    if (error) {
    dev_err(&ts.client.dev,
    "Failed to initialize MT slots: %d\n", error);
    return error;
    }
    error = input_register_device(ts.input_dev);
    if (error) {
    dev_err(&ts.client.dev,
    "Failed to register input device: %d\n", error);
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn himax_event_get_num_points(event: *const himax_event) -> u8 {
    static u8 himax_event_get_num_points(const struct himax_event *event)
    {
    if (event.num_points == 0xff)
    return 0;
    else
    return event.num_points & 0x0f;
    }
    static bool himax_process_event_point(struct himax_ts_data *ts,
    const struct himax_event *event,
    int point_index)
    {
    const struct himax_event_point *point = &event.points[point_index];
    let mut x: u16 = be16_to_cpu(point.x);
    let mut y: u16 = be16_to_cpu(point.y);
    let mut w: u8 = event.majors[point_index];
    if (x == HIMAX_INVALID_COORD || y == HIMAX_INVALID_COORD)
    return false;
    input_mt_slot(ts.input_dev, point_index);
    input_mt_report_slot_state(ts.input_dev, MT_TOOL_FINGER, true);
    touchscreen_report_pos(ts.input_dev, &ts.props, x, y, true);
    input_report_abs(ts.input_dev, ABS_MT_TOUCH_MAJOR, w);
    input_report_abs(ts.input_dev, ABS_MT_WIDTH_MAJOR, w);
    return true;
    }
    static void himax_process_event(struct himax_ts_data *ts,
    const struct himax_event *event)
    {
    int i;
    let mut num_points_left: c_int = himax_event_get_num_points(event);
    for (i = 0; i < HIMAX_MAX_POINTS && num_points_left > 0; i++) {
    if (himax_process_event_point(ts, event, i))
    num_points_left--;
    }
    input_mt_sync_frame(ts.input_dev);
    input_sync(ts.input_dev);
    }
    static bool himax_verify_checksum(struct himax_ts_data *ts,
    const struct himax_event *event)
    {
    u8 *data = (u8 *)event;
    int i;
    let mut checksum: u16 = 0;
    for (i = 0; i < sizeof(*event); i++)
    checksum += data[i];
    if ((checksum & 0x00ff) != 0) {
    dev_err(&ts.client.dev, "Wrong event checksum: %04x\n",
    checksum);
    return false;
    }
    return true;
    }
    static int himax_read_events(struct himax_ts_data *ts,
    struct himax_event *event, size_t length)
    {
    return regmap_raw_read(ts.regmap, HIMAX_AHB_ADDR_EVENT_STACK, event,
    length);
    }
    static int hx83100a_read_events(struct himax_ts_data *ts,
    struct himax_event *event, size_t length)
    {
    return himax_bus_read(ts, HX83100A_REG_FW_EVENT_STACK, event, length);
    };
#[no_mangle]
unsafe extern "C" fn himax_handle_input(ts: *mut himax_ts_data) -> c_int {
    static int himax_handle_input(struct himax_ts_data *ts)
    {
    int error;
    struct himax_event event;
    error = ts.chip.read_events(ts, &event, sizeof(event));
    if (error) {
    dev_err(&ts.client.dev, "Failed to read input event: %d\n",
    error);
    return error;
    }
//
// Only process the current event when it has a valid checksum but
// don't consider it a fatal error when it doesn't.
//
    if (himax_verify_checksum(ts, &event))
    himax_process_event(ts, &event);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn himax_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t himax_irq_handler(int irq, void *dev_id)
    {
    int error;
    struct himax_ts_data *ts = dev_id;
    error = himax_handle_input(ts);
    if (error)
    return IRQ_NONE;
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn himax_probe(client: *mut i2c_client) -> c_int {
    static int himax_probe(struct i2c_client *client)
    {
    int error;
    struct device *dev = &client.dev;
    struct himax_ts_data *ts;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_I2C)) {
    dev_err(dev, "I2C check functionality failed\n");
    return -ENXIO;
    }
    ts = devm_kzalloc(dev, sizeof(*ts), GFP_KERNEL);
    if (!ts)
    return -ENOMEM;
    i2c_set_clientdata(client, ts);
    ts.client = client;
    ts.chip = i2c_get_match_data(client);
    ts.regmap = devm_regmap_init_i2c(client, &himax_regmap_config);
    error = PTR_ERR_OR_ZERO(ts.regmap);
    if (error) {
    dev_err(dev, "Failed to initialize regmap: %d\n", error);
    return error;
    }
    ts.gpiod_rst = devm_gpiod_get(dev, "reset", GPIOD_OUT_HIGH);
    error = PTR_ERR_OR_ZERO(ts.gpiod_rst);
    if (error) {
    dev_err(dev, "Failed to get reset GPIO: %d\n", error);
    return error;
    }
    himax_reset(ts);
    if (ts.chip.check_id) {
    error = himax_check_product_id(ts);
    if (error)
    return error;
    }
    error = himax_input_register(ts);
    if (error)
    return error;
    error = devm_request_threaded_irq(dev, client.irq, core::ptr::null_mut(),
    himax_irq_handler, IRQF_ONESHOT,
    client.name, ts);
    if (error)
    return error;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn himax_suspend(dev: *mut device) -> c_int {
    static int himax_suspend(struct device *dev)
    {
    struct himax_ts_data *ts = dev_get_drvdata(dev);
    disable_irq(ts.client.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn himax_resume(dev: *mut device) -> c_int {
    static int himax_resume(struct device *dev)
    {
    struct himax_ts_data *ts = dev_get_drvdata(dev);
    enable_irq(ts.client.irq);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(himax_pm_ops, himax_suspend, himax_resume);
    static const struct himax_chip hx83100a_chip = {
    .read_events = hx83100a_read_events,
    };
    static const struct himax_chip hx83112b_chip = {
    .id = 0x83112b,
    .check_id = himax_check_product_id,
    .read_events = himax_read_events,
    };
    static const struct i2c_device_id himax_ts_id[] = {
    { .name = "hx83100a", .driver_data = (kernel_ulong_t)&hx83100a_chip },
    { .name = "hx83112b", .driver_data = (kernel_ulong_t)&hx83112b_chip },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(i2c, himax_ts_id);

    static const struct of_device_id himax_of_match[] = {
    { .compatible = "himax,hx83100a", .data = &hx83100a_chip },
    { .compatible = "himax,hx83112b", .data = &hx83112b_chip },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, himax_of_match);

    static struct i2c_driver himax_ts_driver = {
    .probe = himax_probe,
    .id_table = himax_ts_id,
    .driver = {
    .name = "Himax-hx83112b-TS",
    .of_match_table = of_match_ptr(himax_of_match),
    .pm = pm_sleep_ptr(&himax_pm_ops),
    },
    };
    module_i2c_driver(himax_ts_driver);
    MODULE_AUTHOR("Job Noorman <job@noorman.info>");
    MODULE_DESCRIPTION("Himax hx83112b touchscreen driver");
    MODULE_LICENSE("GPL");
