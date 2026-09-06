//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/zforce_ts.c
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
// Copyright (C) 2012-2013 MundoReader S.L.
// Author: Heiko Stuebner <heiko@sntech.de>
//
// based in parts on Nook zforce driver
//
// Copyright (C) 2010 Barnes & Noble, Inc.
// Author: Pieter Truter<ptruter@intrinsyc.com>
//

pub const FRAME_START: c_uint = 0xee;
pub const FRAME_MAXSIZE: c_int = 257;
// Offsets of the different parts of the payload the controller sends
pub const PAYLOAD_HEADER: c_int = 0;
pub const PAYLOAD_LENGTH: c_int = 1;
pub const PAYLOAD_BODY: c_int = 2;
// Response offsets
pub const RESPONSE_ID: c_int = 0;
pub const RESPONSE_DATA: c_int = 1;
// Commands
pub const COMMAND_DEACTIVATE: c_uint = 0x00;
pub const COMMAND_INITIALIZE: c_uint = 0x01;
pub const COMMAND_RESOLUTION: c_uint = 0x02;
pub const COMMAND_SETCONFIG: c_uint = 0x03;
pub const COMMAND_DATAREQUEST: c_uint = 0x04;
pub const COMMAND_SCANFREQ: c_uint = 0x08;

//
// Responses the controller sends as a result of
// command requests
//
pub const RESPONSE_DEACTIVATE: c_uint = 0x00;
pub const RESPONSE_INITIALIZE: c_uint = 0x01;
pub const RESPONSE_RESOLUTION: c_uint = 0x02;
pub const RESPONSE_SETCONFIG: c_uint = 0x03;
pub const RESPONSE_SCANFREQ: c_uint = 0x08;

//
// Notifications are sent by the touch controller without
// being requested by the driver and include for example
// touch indications
//
pub const NOTIFICATION_TOUCH: c_uint = 0x04;
pub const NOTIFICATION_BOOTCOMPLETE: c_uint = 0x07;
pub const NOTIFICATION_OVERRUN: c_uint = 0x25;
pub const NOTIFICATION_PROXIMITY: c_uint = 0x26;
pub const NOTIFICATION_INVALID_COMMAND: c_uint = 0xfe;
pub const ZFORCE_REPORT_POINTS: c_int = 2;
pub const ZFORCE_MAX_AREA: c_uint = 0xff;
pub const STATE_DOWN: c_int = 0;
pub const STATE_MOVE: c_int = 1;
pub const STATE_UP: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zforce_point {
    pub coord_x: c_int,
    pub coord_y: c_int,
    pub state: c_int,
    pub id: c_int,
    pub area_major: c_int,
    pub area_minor: c_int,
    pub orientation: c_int,
    pub pressure: c_int,
    pub prblty: c_int,
}

//
// @client		the i2c_client
// @input		the input device
// @suspending		in the process of going to suspend (don't emit wakeup
// events for commands executed to suspend the device)
// @suspended		device suspended
// @command_done	completion to wait for the command result
// @command_waiting	the id of the command that is currently waiting
// for a result
// @command_result	returned result of the command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zforce_ts {
    pub client: *mut i2c_client,
    pub input: *mut input_dev,
    pub prop: touchscreen_properties,
    pub phys: [c_char; 32],
    pub gpio_int: *mut gpio_desc,
    pub gpio_rst: *mut gpio_desc,
    pub suspending: bool,
    pub suspended: bool,
    pub boot_complete: bool,
// Firmware version information
    pub version_major: u16,
    pub version_minor: u16,
    pub version_build: u16,
    pub version_rev: u16,
    pub command_done: completion,
    pub command_waiting: c_int,
    pub command_result: c_int,
}

#[no_mangle]
unsafe extern "C" fn zforce_command(ts: *mut zforce_ts, cmd: u8) -> c_int {
    static int zforce_command(struct zforce_ts *ts, u8 cmd)
    {
    struct i2c_client *client = ts.client;
    char buf[3];
    int ret;
    dev_dbg(&client.dev, "%s: 0x%x\n", __func__, cmd);
    buf[0] = FRAME_START;
    buf[1] = 1; /* data size, command only */
    buf[2] = cmd;
    ret = i2c_master_send(client, &buf[0], ARRAY_SIZE(buf));
    if (ret < 0) {
    dev_err(&client.dev, "i2c send data request error: %d\n", ret);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn zforce_send_wait(ts: *mut zforce_ts, buf: *const c_char, len: c_int) -> c_int {
    static int zforce_send_wait(struct zforce_ts *ts, const char *buf, int len)
    {
    struct i2c_client *client = ts.client;
    int ret;
    dev_dbg(&client.dev, "sending %d bytes for command 0x%x\n",
    buf[1], buf[2]);
    ts.command_waiting = buf[2];
    ret = i2c_master_send(client, buf, len);
    if (ret < 0) {
    dev_err(&client.dev, "i2c send data request error: %d\n", ret);
    return ret;
    }
    dev_dbg(&client.dev, "waiting for result for command 0x%x\n", buf[2]);
    if (wait_for_completion_timeout(&ts.command_done, WAIT_TIMEOUT) == 0)
    return -ETIME;
    ret = ts.command_result;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn zforce_command_wait(ts: *mut zforce_ts, cmd: u8) -> c_int {
    static int zforce_command_wait(struct zforce_ts *ts, u8 cmd)
    {
    struct i2c_client *client = ts.client;
    char buf[3];
    int error;
    dev_dbg(&client.dev, "%s: 0x%x\n", __func__, cmd);
    buf[0] = FRAME_START;
    buf[1] = 1; /* data size, command only */
    buf[2] = cmd;
    error = zforce_send_wait(ts, &buf[0], ARRAY_SIZE(buf));
    if (error) {
    dev_err(&client.dev, "i2c send data request error: %d\n",
    error);
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn zforce_resolution(ts: *mut zforce_ts, x: u16, y: u16) -> c_int {
    static int zforce_resolution(struct zforce_ts *ts, u16 x, u16 y)
    {
    struct i2c_client *client = ts.client;
    char buf[7] = { FRAME_START, 5, COMMAND_RESOLUTION,
    (x & 0xff), ((x >> 8) & 0xff),
    (y & 0xff), ((y >> 8) & 0xff) };
    dev_dbg(&client.dev, "set resolution to (%d,%d)\n", x, y);
    return zforce_send_wait(ts, &buf[0], ARRAY_SIZE(buf));
    }
    static int zforce_scan_frequency(struct zforce_ts *ts, u16 idle, u16 finger,
    u16 stylus)
    {
    struct i2c_client *client = ts.client;
    char buf[9] = { FRAME_START, 7, COMMAND_SCANFREQ,
    (idle & 0xff), ((idle >> 8) & 0xff),
    (finger & 0xff), ((finger >> 8) & 0xff),
    (stylus & 0xff), ((stylus >> 8) & 0xff) };
    dev_dbg(&client.dev,
    "set scan frequency to (idle: %d, finger: %d, stylus: %d)\n",
    idle, finger, stylus);
    return zforce_send_wait(ts, &buf[0], ARRAY_SIZE(buf));
    }
#[no_mangle]
unsafe extern "C" fn zforce_setconfig(ts: *mut zforce_ts, b1: c_char) -> c_int {
    static int zforce_setconfig(struct zforce_ts *ts, char b1)
    {
    struct i2c_client *client = ts.client;
    char buf[7] = { FRAME_START, 5, COMMAND_SETCONFIG,
    b1, 0, 0, 0 };
    dev_dbg(&client.dev, "set config to (%d)\n", b1);
    return zforce_send_wait(ts, &buf[0], ARRAY_SIZE(buf));
    }
#[no_mangle]
unsafe extern "C" fn zforce_start(ts: *mut zforce_ts) -> c_int {
    static int zforce_start(struct zforce_ts *ts)
    {
    struct i2c_client *client = ts.client;
    int error;
    dev_dbg(&client.dev, "starting device\n");
    error = zforce_command_wait(ts, COMMAND_INITIALIZE);
    if (error) {
    dev_err(&client.dev, "Unable to initialize, %d\n", error);
    return error;
    }
    error = zforce_resolution(ts, ts.prop.max_x, ts.prop.max_y);
    if (error) {
    dev_err(&client.dev, "Unable to set resolution, %d\n", error);
    goto err_deactivate;
    }
    error = zforce_scan_frequency(ts, 10, 50, 50);
    if (error) {
    dev_err(&client.dev, "Unable to set scan frequency, %d\n",
    error);
    goto err_deactivate;
    }
    error = zforce_setconfig(ts, SETCONFIG_DUALTOUCH);
    if (error) {
    dev_err(&client.dev, "Unable to set config\n");
    goto err_deactivate;
    }
// start sending touch events
    error = zforce_command(ts, COMMAND_DATAREQUEST);
    if (error) {
    dev_err(&client.dev, "Unable to request data\n");
    goto err_deactivate;
    }
//
// Per NN, initial cal. take max. of 200msec.
// Allow time to complete this calibration
//
    msleep(200);
    return 0;
    err_deactivate:
    zforce_command_wait(ts, COMMAND_DEACTIVATE);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn zforce_stop(ts: *mut zforce_ts) -> c_int {
    static int zforce_stop(struct zforce_ts *ts)
    {
    struct i2c_client *client = ts.client;
    int error;
    dev_dbg(&client.dev, "stopping device\n");
// Deactivates touch sensing and puts the device into sleep.
    error = zforce_command_wait(ts, COMMAND_DEACTIVATE);
    if (error) {
    dev_err(&client.dev, "could not deactivate device, %d\n",
    error);
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn zforce_touch_event(ts: *mut zforce_ts, payload: *mut u8) -> c_int {
    static int zforce_touch_event(struct zforce_ts *ts, u8 *payload)
    {
    struct i2c_client *client = ts.client;
    struct zforce_point point;
    int count, i, num = 0;
    u8 *p;
    count = payload[0];
    if (count > ZFORCE_REPORT_POINTS) {
    dev_warn(&client.dev,
    "too many coordinates %d, expected max %d\n",
    count, ZFORCE_REPORT_POINTS);
    count = ZFORCE_REPORT_POINTS;
    }
    for (i = 0; i < count; i++) {
    p = &payload[i * 9 + 1];
    point.coord_x = get_unaligned_le16(&p[0]);
    point.coord_y = get_unaligned_le16(&p[2]);
    if (point.coord_x > ts.prop.max_x ||
    point.coord_y > ts.prop.max_y) {
    dev_warn(&client.dev, "coordinates (%d,%d) invalid\n",
    point.coord_x, point.coord_y);
    point.coord_x = point.coord_y = 0;
    }
    point.state = p[4] & 0x0f;
    point.id = (p[4] & 0xf0) >> 4;
// determine touch major, minor and orientation
    point.area_major = max(p[5], p[6]);
    point.area_minor = min(p[5], p[6]);
    point.orientation = p[5] > p[6];
    point.pressure = p[7];
    point.prblty = p[8];
    dev_dbg(&client.dev,
    "point %d/%d: state %d, id %d, pressure %d, prblty %d, x %d, y %d, amajor %d, aminor %d, ori %d\n",
    i, count, point.state, point.id,
    point.pressure, point.prblty,
    point.coord_x, point.coord_y,
    point.area_major, point.area_minor,
    point.orientation);
// the zforce id starts with "1", so needs to be decreased
    input_mt_slot(ts.input, point.id - 1);
    if (input_mt_report_slot_state(ts.input, MT_TOOL_FINGER,
    point.state != STATE_UP)) {
    touchscreen_report_pos(ts.input, &ts.prop,
    point.coord_x, point.coord_y,
    true);
    input_report_abs(ts.input, ABS_MT_TOUCH_MAJOR,
    point.area_major);
    input_report_abs(ts.input, ABS_MT_TOUCH_MINOR,
    point.area_minor);
    input_report_abs(ts.input, ABS_MT_ORIENTATION,
    point.orientation);
    num++;
    }
    }
    input_mt_sync_frame(ts.input);
    input_mt_report_finger_count(ts.input, num);
    input_sync(ts.input);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn zforce_read_packet(ts: *mut zforce_ts, buf: *mut u8) -> c_int {
    static int zforce_read_packet(struct zforce_ts *ts, u8 *buf)
    {
    struct i2c_client *client = ts.client;
    int ret;
// read 2 byte message header
    ret = i2c_master_recv(client, buf, 2);
    if (ret < 0) {
    dev_err(&client.dev, "error reading header: %d\n", ret);
    return ret;
    }
    if (buf[PAYLOAD_HEADER] != FRAME_START) {
    dev_err(&client.dev, "invalid frame start: %d\n", buf[0]);
    return -EIO;
    }
    if (buf[PAYLOAD_LENGTH] == 0) {
    dev_err(&client.dev, "invalid payload length: %d\n",
    buf[PAYLOAD_LENGTH]);
    return -EIO;
    }
// read the message
    ret = i2c_master_recv(client, &buf[PAYLOAD_BODY], buf[PAYLOAD_LENGTH]);
    if (ret < 0) {
    dev_err(&client.dev, "error reading payload: %d\n", ret);
    return ret;
    }
    dev_dbg(&client.dev, "read %d bytes for response command 0x%x\n",
    buf[PAYLOAD_LENGTH], buf[PAYLOAD_BODY]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn zforce_complete(ts: *mut zforce_ts, cmd: c_int, result: c_int) {
    static void zforce_complete(struct zforce_ts *ts, int cmd, int result)
    {
    struct i2c_client *client = ts.client;
    if (ts.command_waiting == cmd) {
    dev_dbg(&client.dev, "completing command 0x%x\n", cmd);
    ts.command_result = result;
    complete(&ts.command_done);
    } else {
    dev_dbg(&client.dev, "command %d not for us\n", cmd);
    }
    }
#[no_mangle]
unsafe extern "C" fn zforce_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t zforce_irq(int irq, void *dev_id)
    {
    struct zforce_ts *ts = dev_id;
    struct i2c_client *client = ts.client;
    if (ts.suspended && device_may_wakeup(&client.dev))
    pm_wakeup_event(&client.dev, 500);
    return IRQ_WAKE_THREAD;
    }
#[no_mangle]
unsafe extern "C" fn zforce_irq_thread(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t zforce_irq_thread(int irq, void *dev_id)
    {
    struct zforce_ts *ts = dev_id;
    struct i2c_client *client = ts.client;
    int error;
    u8 payload_buffer[FRAME_MAXSIZE];
    u8 *payload;
    bool suspending;
//
// When still suspended, return.
// Due to the level-interrupt we will get re-triggered later.
//
    if (ts.suspended) {
    msleep(20);
    return IRQ_HANDLED;
    }
    dev_dbg(&client.dev, "handling interrupt\n");
// Don't emit wakeup events from commands run by zforce_suspend
    suspending = READ_ONCE(ts.suspending);
    if (!suspending && device_may_wakeup(&client.dev))
    pm_stay_awake(&client.dev);
//
// Run at least once and exit the loop if
// - the optional interrupt GPIO isn't specified
// (there is only one packet read per ISR invocation, then)
// or
// - the GPIO isn't active any more
// (packet read until the level GPIO indicates that there is
// no IRQ any more)
//
    do {
    error = zforce_read_packet(ts, payload_buffer);
    if (error) {
    dev_err(&client.dev,
    "could not read packet, ret: %d\n", error);
    break;
    }
    payload =  &payload_buffer[PAYLOAD_BODY];
    switch (payload[RESPONSE_ID]) {
    case NOTIFICATION_TOUCH:
//
// Always report touch-events received while
// suspending, when being a wakeup source
//
    if (suspending && device_may_wakeup(&client.dev))
    pm_wakeup_event(&client.dev, 500);
    zforce_touch_event(ts, &payload[RESPONSE_DATA]);
    break;
    case NOTIFICATION_BOOTCOMPLETE:
    ts.boot_complete = payload[RESPONSE_DATA];
    zforce_complete(ts, payload[RESPONSE_ID], 0);
    break;
    case RESPONSE_INITIALIZE:
    case RESPONSE_DEACTIVATE:
    case RESPONSE_SETCONFIG:
    case RESPONSE_RESOLUTION:
    case RESPONSE_SCANFREQ:
    zforce_complete(ts, payload[RESPONSE_ID],
    payload[RESPONSE_DATA]);
    break;
    case RESPONSE_STATUS:
//
// Version Payload Results
// [2:major] [2:minor] [2:build] [2:rev]
//
    ts.version_major =
    get_unaligned_le16(&payload[RESPONSE_DATA]);
    ts.version_minor =
    get_unaligned_le16(&payload[RESPONSE_DATA + 2]);
    ts.version_build =
    get_unaligned_le16(&payload[RESPONSE_DATA + 4]);
    ts.version_rev =
    get_unaligned_le16(&payload[RESPONSE_DATA + 6]);
    dev_dbg(&ts.client.dev,
    "Firmware Version %04x:%04x %04x:%04x\n",
    ts.version_major, ts.version_minor,
    ts.version_build, ts.version_rev);
    zforce_complete(ts, payload[RESPONSE_ID], 0);
    break;
    case NOTIFICATION_INVALID_COMMAND:
    dev_err(&ts.client.dev, "invalid command: 0x%x\n",
    payload[RESPONSE_DATA]);
    break;
    default:
    dev_err(&ts.client.dev,
    "unrecognized response id: 0x%x\n",
    payload[RESPONSE_ID]);
    break;
    }
    } while (gpiod_get_value_cansleep(ts.gpio_int));
    if (!suspending && device_may_wakeup(&client.dev))
    pm_relax(&client.dev);
    dev_dbg(&client.dev, "finished interrupt\n");
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn zforce_input_open(dev: *mut input_dev) -> c_int {
    static int zforce_input_open(struct input_dev *dev)
    {
    struct zforce_ts *ts = input_get_drvdata(dev);
    return zforce_start(ts);
    }
#[no_mangle]
unsafe extern "C" fn zforce_input_close(dev: *mut input_dev) {
    static void zforce_input_close(struct input_dev *dev)
    {
    struct zforce_ts *ts = input_get_drvdata(dev);
    struct i2c_client *client = ts.client;
    int error;
    error = zforce_stop(ts);
    if (error)
    dev_warn(&client.dev, "stopping zforce failed\n");
    }
#[no_mangle]
unsafe extern "C" fn __zforce_suspend(ts: *mut zforce_ts) -> c_int {
    static int __zforce_suspend(struct zforce_ts *ts)
    {
    struct i2c_client *client = ts.client;
    struct input_dev *input = ts.input;
    int error;
    guard(mutex)(&input.mutex);
//
// When configured as a wakeup source device should always wake
// the system, therefore start device if necessary.
//
    if (device_may_wakeup(&client.dev)) {
    dev_dbg(&client.dev, "suspend while being a wakeup source\n");
// Need to start device, if not open, to be a wakeup source.
    if (!input_device_enabled(input)) {
    error = zforce_start(ts);
    if (error)
    return error;
    }
    enable_irq_wake(client.irq);
    } else if (input_device_enabled(input)) {
    dev_dbg(&client.dev,
    "suspend without being a wakeup source\n");
    error = zforce_stop(ts);
    if (error)
    return error;
    disable_irq(client.irq);
    }
    ts.suspended = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn zforce_suspend(dev: *mut device) -> c_int {
    static int zforce_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct zforce_ts *ts = i2c_get_clientdata(client);
    int ret;
    WRITE_ONCE(ts.suspending, true);
    smp_mb();
    ret = __zforce_suspend(ts);
    smp_mb();
    WRITE_ONCE(ts.suspending, false);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn zforce_resume(dev: *mut device) -> c_int {
    static int zforce_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct zforce_ts *ts = i2c_get_clientdata(client);
    struct input_dev *input = ts.input;
    int error;
    guard(mutex)(&input.mutex);
    ts.suspended = false;
    if (device_may_wakeup(&client.dev)) {
    dev_dbg(&client.dev, "resume from being a wakeup source\n");
    disable_irq_wake(client.irq);
// need to stop device if it was not open on suspend
    if (!input_device_enabled(input)) {
    error = zforce_stop(ts);
    if (error)
    return error;
    }
    } else if (input_device_enabled(input)) {
    dev_dbg(&client.dev, "resume without being a wakeup source\n");
    enable_irq(client.irq);
    error = zforce_start(ts);
    if (error)
    return error;
    }
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(zforce_pm_ops, zforce_suspend, zforce_resume);
#[no_mangle]
unsafe extern "C" fn zforce_reset(data: *mut c_void) {
    static void zforce_reset(void *data)
    {
    struct zforce_ts *ts = data;
    gpiod_set_value_cansleep(ts.gpio_rst, 1);
    udelay(10);
    }
#[no_mangle]
unsafe extern "C" fn zforce_ts_parse_legacy_properties(ts: *mut zforce_ts) {
    static void zforce_ts_parse_legacy_properties(struct zforce_ts *ts)
    {
    let mut x_max: u32 = 0;
    let mut y_max: u32 = 0;
    device_property_read_u32(&ts.client.dev, "x-size", &x_max);
    input_set_abs_params(ts.input, ABS_MT_POSITION_X, 0, x_max, 0, 0);
    device_property_read_u32(&ts.client.dev, "y-size", &y_max);
    input_set_abs_params(ts.input, ABS_MT_POSITION_Y, 0, y_max, 0, 0);
    }
#[no_mangle]
unsafe extern "C" fn zforce_probe(client: *mut i2c_client) -> c_int {
    static int zforce_probe(struct i2c_client *client)
    {
    struct zforce_ts *ts;
    struct input_dev *input_dev;
    int error;
    ts = devm_kzalloc(&client.dev, sizeof(struct zforce_ts), GFP_KERNEL);
    if (!ts)
    return -ENOMEM;
    ts.gpio_rst = devm_gpiod_get_optional(&client.dev, "reset",
    GPIOD_OUT_HIGH);
    error = PTR_ERR_OR_ZERO(ts.gpio_rst);
    if (error)
    return dev_err_probe(&client.dev, error,
    "failed to request reset GPIO\n");
    if (ts.gpio_rst) {
    ts.gpio_int = devm_gpiod_get_optional(&client.dev, "irq",
    GPIOD_IN);
    error = PTR_ERR_OR_ZERO(ts.gpio_int);
    if (error)
    return dev_err_probe(&client.dev, error,
    "failed to request interrupt GPIO\n");
    } else {
//
// Deprecated GPIO handling for compatibility
// with legacy binding.
//
// INT GPIO
    ts.gpio_int = devm_gpiod_get_index(&client.dev, core::ptr::null_mut(), 0,
    GPIOD_IN);
    error = PTR_ERR_OR_ZERO(ts.gpio_int);
    if (error)
    return dev_err_probe(&client.dev, error,
    "failed to request interrupt GPIO\n");
// RST GPIO
    ts.gpio_rst = devm_gpiod_get_index(&client.dev, core::ptr::null_mut(), 1,
    GPIOD_OUT_HIGH);
    error = PTR_ERR_OR_ZERO(ts.gpio_rst);
    if (error)
    return dev_err_probe(&client.dev, error,
    "failed to request reset GPIO\n");
    }
    error = devm_regulator_get_enable(&client.dev, "vdd");
    if (error)
    return dev_err_probe(&client.dev, error,
    "failed to request vdd supply\n");
//
// According to datasheet add 100us grace time after regular
// regulator enable delay.
//
    usleep_range(100, 200);
    error = devm_add_action_or_reset(&client.dev, zforce_reset, ts);
    if (error)
    return dev_err_probe(&client.dev, error,
    "failed to register reset action\n");
    snprintf(ts.phys, sizeof(ts.phys),
    "%s/input0", dev_name(&client.dev));
    input_dev = devm_input_allocate_device(&client.dev);
    if (!input_dev)
    return -ENOMEM;
    ts.client = client;
    ts.input = input_dev;
    input_dev.name = "Neonode zForce touchscreen";
    input_dev.phys = ts.phys;
    input_dev.id.bustype = BUS_I2C;
    input_dev.open = zforce_input_open;
    input_dev.close = zforce_input_close;
    zforce_ts_parse_legacy_properties(ts);
    touchscreen_parse_properties(input_dev, true, &ts.prop);
    if (ts.prop.max_x == 0 || ts.prop.max_y == 0)
    return dev_err_probe(&client.dev, -EINVAL, "no size specified");
    input_set_abs_params(input_dev, ABS_MT_TOUCH_MAJOR, 0,
    ZFORCE_MAX_AREA, 0, 0);
    input_set_abs_params(input_dev, ABS_MT_TOUCH_MINOR, 0,
    ZFORCE_MAX_AREA, 0, 0);
    input_set_abs_params(input_dev, ABS_MT_ORIENTATION, 0, 1, 0, 0);
    error = input_mt_init_slots(input_dev, ZFORCE_REPORT_POINTS,
    INPUT_MT_DIRECT);
    if (error)
    return error;
    input_set_drvdata(ts.input, ts);
    init_completion(&ts.command_done);
//
// The zforce pulls the interrupt low when it has data ready.
// After it is triggered the isr thread runs until all the available
// packets have been read and the interrupt is high again.
// Therefore we can trigger the interrupt anytime it is low and do
// not need to limit it to the interrupt edge.
//
    error = devm_request_threaded_irq(&client.dev, client.irq,
    zforce_irq, zforce_irq_thread,
    IRQF_ONESHOT, input_dev.name, ts);
    if (error)
    return dev_err_probe(&client.dev, error,
    "irq %d request failed\n", client.irq);
    i2c_set_clientdata(client, ts);
// let the controller boot
    gpiod_set_value_cansleep(ts.gpio_rst, 0);
    ts.command_waiting = NOTIFICATION_BOOTCOMPLETE;
    if (wait_for_completion_timeout(&ts.command_done, WAIT_TIMEOUT) == 0)
    dev_warn(&client.dev, "bootcomplete timed out\n");
// need to start device to get version information
    error = zforce_command_wait(ts, COMMAND_INITIALIZE);
    if (error)
    return dev_err_probe(&client.dev, error, "unable to initialize\n");
// this gets the firmware version among other information
    error = zforce_command_wait(ts, COMMAND_STATUS);
    if (error) {
    dev_err_probe(&client.dev, error, "couldn't get status\n");
    zforce_stop(ts);
    return error;
    }
// stop device and put it into sleep until it is opened
    error = zforce_stop(ts);
    if (error)
    return error;
    device_set_wakeup_capable(&client.dev, true);
    error = input_register_device(input_dev);
    if (error)
    return dev_err_probe(&client.dev, error,
    "could not register input device\n");
    return 0;
    }
    static const struct i2c_device_id zforce_idtable[] = {
    { .name = "zforce-ts" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, zforce_idtable);

    static const struct of_device_id zforce_dt_idtable[] = {
    { .compatible = "neonode,zforce" },
    {},
    };
    MODULE_DEVICE_TABLE(of, zforce_dt_idtable);

    static struct i2c_driver zforce_driver = {
    .driver = {
    .name	= "zforce-ts",
    .pm	= pm_sleep_ptr(&zforce_pm_ops),
    .of_match_table	= of_match_ptr(zforce_dt_idtable),
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    .probe		= zforce_probe,
    .id_table	= zforce_idtable,
    };
    module_i2c_driver(zforce_driver);
    MODULE_AUTHOR("Heiko Stuebner <heiko@sntech.de>");
    MODULE_DESCRIPTION("zForce TouchScreen Driver");
    MODULE_LICENSE("GPL");
