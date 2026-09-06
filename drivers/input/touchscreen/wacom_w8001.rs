//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/wacom_w8001.c
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
// Wacom W8001 penabled serial touchscreen driver
//
// Copyright (c) 2008 Jaya Kumar
// Copyright (c) 2010 Red Hat, Inc.
// Copyright (c) 2010 - 2011 Ping Cheng, Wacom. <pingc@wacom.com>
//
// This file is subject to the terms and conditions of the GNU General Public
// License. See the file COPYING in the main directory of this archive for
// more details.
//
// Layout based on Elo serial touchscreen driver by Vojtech Pavlik
//

    MODULE_AUTHOR("Jaya Kumar <jayakumar.lkml@gmail.com>");
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_LICENSE("GPL");
pub const W8001_MAX_PHYS: c_int = 42;
pub const W8001_MAX_LENGTH: c_int = 13;
pub const W8001_LEAD_MASK: c_uint = 0x80;
pub const W8001_LEAD_BYTE: c_uint = 0x80;
pub const W8001_TAB_MASK: c_uint = 0x40;
pub const W8001_TAB_BYTE: c_uint = 0x40;
// set in first byte of touch data packets

pub const W8001_QUERY_PACKET: c_uint = 0x20;

// length of data packets in bytes, depends on device.
pub const W8001_PKTLEN_TOUCH93: c_int = 5;
pub const W8001_PKTLEN_TOUCH9A: c_int = 7;
pub const W8001_PKTLEN_TPCPEN: c_int = 9;

pub const W8001_PKTLEN_TOUCH2FG: c_int = 13;
// resolution in points/mm
pub const W8001_PEN_RESOLUTION: c_int = 100;
pub const W8001_TOUCH_RESOLUTION: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct w8001_coord {
    pub rdy: u8,
    pub tsw: u8,
    pub f1: u8,
    pub f2: u8,
    pub x: u16,
    pub y: u16,
    pub pen_pressure: u16,
    pub tilt_x: u8,
    pub tilt_y: u8,
}

// touch query reply packet
#[repr(C)]
#[derive(Copy, Clone)]
pub struct w8001_touch_query {
    pub x: u16,
    pub y: u16,
    pub panel_res: u8,
    pub capacity_res: u8,
    pub sensor_id: u8,
}

//
// Per-touchscreen data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct w8001 {
    pub pen_dev: *mut input_dev,
    pub touch_dev: *mut input_dev,
    pub serio: *mut serio,
    pub cmd_done: completion,
    pub id: c_int,
    pub idx: c_int,
    pub response_type: c_uchar,
    pub response: [c_uchar; W8001_MAX_LENGTH],
    pub data: [c_uchar; W8001_MAX_LENGTH],
    pub phys: [c_char; W8001_MAX_PHYS],
    pub type: c_int,
    pub pktlen: c_uint,
    pub max_touch_x: u16,
    pub max_touch_y: u16,
    pub max_pen_x: u16,
    pub max_pen_y: u16,
    pub pen_name: [c_char; 64],
    pub touch_name: [c_char; 64],
    pub open_count: c_int,
    pub mutex: mutex,
}

#[no_mangle]
unsafe extern "C" fn parse_pen_data(data: *mut u8, coord: *mut w8001_coord) {
    static void parse_pen_data(u8 *data, struct w8001_coord *coord)
    {
    memset(coord, 0, sizeof(*coord));
    coord.rdy = data[0] & 0x20;
    coord.tsw = data[0] & 0x01;
    coord.f1 = data[0] & 0x02;
    coord.f2 = data[0] & 0x04;
    coord.x = (data[1] & 0x7F) << 9;
    coord.x |= (data[2] & 0x7F) << 2;
    coord.x |= (data[6] & 0x60) >> 5;
    coord.y = (data[3] & 0x7F) << 9;
    coord.y |= (data[4] & 0x7F) << 2;
    coord.y |= (data[6] & 0x18) >> 3;
    coord.pen_pressure = data[5] & 0x7F;
    coord.pen_pressure |= (data[6] & 0x07) << 7 ;
    coord.tilt_x = data[7] & 0x7F;
    coord.tilt_y = data[8] & 0x7F;
    }
#[no_mangle]
unsafe extern "C" fn parse_single_touch(data: *mut u8, coord: *mut w8001_coord) {
    static void parse_single_touch(u8 *data, struct w8001_coord *coord)
    {
    coord.x = (data[1] << 7) | data[2];
    coord.y = (data[3] << 7) | data[4];
    coord.tsw = data[0] & 0x01;
    }
    static void scale_touch_coordinates(struct w8001 *w8001,
    unsigned int *x, unsigned int *y)
    {
    if (w8001.max_pen_x && w8001.max_touch_x)
// x = *x * w8001->max_pen_x / w8001->max_touch_x;
    if (w8001.max_pen_y && w8001.max_touch_y)
// y = *y * w8001->max_pen_y / w8001->max_touch_y;
    }
#[no_mangle]
unsafe extern "C" fn parse_multi_touch(w8001: *mut w8001) {
    static void parse_multi_touch(struct w8001 *w8001)
    {
    struct input_dev *dev = w8001.touch_dev;
    unsigned char *data = w8001.data;
    unsigned int x, y;
    int i;
    let mut count: c_int = 0;
    for (i = 0; i < 2; i++) {
    let mut touch: bool = data[0] & (1 << i);
    input_mt_slot(dev, i);
    input_mt_report_slot_state(dev, MT_TOOL_FINGER, touch);
    if (touch) {
    x = (data[6 * i + 1] << 7) | data[6 * i + 2];
    y = (data[6 * i + 3] << 7) | data[6 * i + 4];
// data[5,6] and [11,12] is finger capacity
// scale to pen maximum
    scale_touch_coordinates(w8001, &x, &y);
    input_report_abs(dev, ABS_MT_POSITION_X, x);
    input_report_abs(dev, ABS_MT_POSITION_Y, y);
    count++;
    }
    }
// emulate single touch events when stylus is out of proximity.
// This is to make single touch backward support consistent
// across all Wacom single touch devices.
//
    if (w8001.type != BTN_TOOL_PEN &&
    w8001.type != BTN_TOOL_RUBBER) {
    w8001.type = count == 1 ? BTN_TOOL_FINGER : KEY_RESERVED;
    input_mt_report_pointer_emulation(dev, true);
    }
    input_sync(dev);
    }
#[no_mangle]
unsafe extern "C" fn parse_touchquery(data: *mut u8, query: *mut w8001_touch_query) {
    static void parse_touchquery(u8 *data, struct w8001_touch_query *query)
    {
    memset(query, 0, sizeof(*query));
    query.panel_res = data[1];
    query.sensor_id = data[2] & 0x7;
    query.capacity_res = data[7];
    query.x = data[3] << 9;
    query.x |= data[4] << 2;
    query.x |= (data[2] >> 5) & 0x3;
    query.y = data[5] << 9;
    query.y |= data[6] << 2;
    query.y |= (data[2] >> 3) & 0x3;
// Early days' single-finger touch models need the following defaults
    if (!query.x && !query.y) {
    query.x = 1024;
    query.y = 1024;
    if (query.panel_res)
    query.x = query.y = (1 << query.panel_res);
    query.panel_res = W8001_TOUCH_RESOLUTION;
    }
    }
#[no_mangle]
unsafe extern "C" fn report_pen_events(w8001: *mut w8001, coord: *mut w8001_coord) {
    static void report_pen_events(struct w8001 *w8001, struct w8001_coord *coord)
    {
    struct input_dev *dev = w8001.pen_dev;
//
// We have 1 bit for proximity (rdy) and 3 bits for tip, side,
// side2/eraser. If rdy && f2 are set, this can be either pen + side2,
// or eraser. Assume:
// - if dev is already in proximity and f2 is toggled → pen + side2
// - if dev comes into proximity with f2 set → eraser
// If f2 disappears after assuming eraser, fake proximity out for
// eraser and in for pen.
//
    switch (w8001.type) {
    case BTN_TOOL_RUBBER:
    if (!coord.f2) {
    input_report_abs(dev, ABS_PRESSURE, 0);
    input_report_key(dev, BTN_TOUCH, 0);
    input_report_key(dev, BTN_STYLUS, 0);
    input_report_key(dev, BTN_STYLUS2, 0);
    input_report_key(dev, BTN_TOOL_RUBBER, 0);
    input_sync(dev);
    w8001.type = BTN_TOOL_PEN;
    }
    break;
    case BTN_TOOL_FINGER:
    case KEY_RESERVED:
    w8001.type = coord.f2 ? BTN_TOOL_RUBBER : BTN_TOOL_PEN;
    break;
    default:
    input_report_key(dev, BTN_STYLUS2, coord.f2);
    break;
    }
    input_report_abs(dev, ABS_X, coord.x);
    input_report_abs(dev, ABS_Y, coord.y);
    input_report_abs(dev, ABS_PRESSURE, coord.pen_pressure);
    input_report_key(dev, BTN_TOUCH, coord.tsw);
    input_report_key(dev, BTN_STYLUS, coord.f1);
    input_report_key(dev, w8001.type, coord.rdy);
    input_sync(dev);
    if (!coord.rdy)
    w8001.type = KEY_RESERVED;
    }
#[no_mangle]
unsafe extern "C" fn report_single_touch(w8001: *mut w8001, coord: *mut w8001_coord) {
    static void report_single_touch(struct w8001 *w8001, struct w8001_coord *coord)
    {
    struct input_dev *dev = w8001.touch_dev;
    let mut x: c_uint = coord.x;
    let mut y: c_uint = coord.y;
// scale to pen maximum
    scale_touch_coordinates(w8001, &x, &y);
    input_report_abs(dev, ABS_X, x);
    input_report_abs(dev, ABS_Y, y);
    input_report_key(dev, BTN_TOUCH, coord.tsw);
    input_sync(dev);
    w8001.type = coord.tsw ? BTN_TOOL_FINGER : KEY_RESERVED;
    }
    static irqreturn_t w8001_interrupt(struct serio *serio,
    unsigned char data, unsigned int flags)
    {
    struct w8001 *w8001 = serio_get_drvdata(serio);
    struct w8001_coord coord;
    unsigned char tmp;
    w8001.data[w8001.idx] = data;
    switch (w8001.idx++) {
    case 0:
    if ((data & W8001_LEAD_MASK) != W8001_LEAD_BYTE) {
    pr_debug("w8001: unsynchronized data: 0x%02x\n", data);
    w8001.idx = 0;
    }
    break;
    case W8001_PKTLEN_TOUCH93 - 1:
    case W8001_PKTLEN_TOUCH9A - 1:
    tmp = w8001.data[0] & W8001_TOUCH_BYTE;
    if (tmp != W8001_TOUCH_BYTE)
    break;
    if (w8001.pktlen == w8001.idx) {
    w8001.idx = 0;
    if (w8001.type != BTN_TOOL_PEN &&
    w8001.type != BTN_TOOL_RUBBER) {
    parse_single_touch(w8001.data, &coord);
    report_single_touch(w8001, &coord);
    }
    }
    break;
// Pen coordinates packet
    case W8001_PKTLEN_TPCPEN - 1:
    tmp = w8001.data[0] & W8001_TAB_MASK;
    if (unlikely(tmp == W8001_TAB_BYTE))
    break;
    tmp = w8001.data[0] & W8001_TOUCH_BYTE;
    if (tmp == W8001_TOUCH_BYTE)
    break;
    w8001.idx = 0;
    parse_pen_data(w8001.data, &coord);
    report_pen_events(w8001, &coord);
    break;
// control packet
    case W8001_PKTLEN_TPCCTL - 1:
    tmp = w8001.data[0] & W8001_TOUCH_MASK;
    if (tmp == W8001_TOUCH_BYTE)
    break;
    w8001.idx = 0;
    memcpy(w8001.response, w8001.data, W8001_MAX_LENGTH);
    w8001.response_type = W8001_QUERY_PACKET;
    complete(&w8001.cmd_done);
    break;
// 2 finger touch packet
    case W8001_PKTLEN_TOUCH2FG - 1:
    w8001.idx = 0;
    parse_multi_touch(w8001);
    break;
    default:
//
// ThinkPad X60 Tablet PC (pen only device) sometimes
// sends invalid data packets that are larger than
// W8001_PKTLEN_TPCPEN. Let's start over again.
//
    if (!w8001.touch_dev && w8001.idx > W8001_PKTLEN_TPCPEN - 1)
    w8001.idx = 0;
    }
    return IRQ_HANDLED;
    }
    static int w8001_command(struct w8001 *w8001, unsigned char command,
    bool wait_response)
    {
    int rc;
    w8001.response_type = 0;
    init_completion(&w8001.cmd_done);
    rc = serio_write(w8001.serio, command);
    if (rc == 0 && wait_response) {
    wait_for_completion_timeout(&w8001.cmd_done, HZ);
    if (w8001.response_type != W8001_QUERY_PACKET)
    rc = -EIO;
    }
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn w8001_open(dev: *mut input_dev) -> c_int {
    static int w8001_open(struct input_dev *dev)
    {
    struct w8001 *w8001 = input_get_drvdata(dev);
    int err;
    scoped_guard(mutex_intr, &w8001.mutex) {
    if (w8001.open_count == 0) {
    err = w8001_command(w8001, W8001_CMD_START, false);
    if (err)
    return err;
    }
    w8001.open_count++;
    return 0;
    }
    return -EINTR;
    }
#[no_mangle]
unsafe extern "C" fn w8001_close(dev: *mut input_dev) {
    static void w8001_close(struct input_dev *dev)
    {
    struct w8001 *w8001 = input_get_drvdata(dev);
    guard(mutex)(&w8001.mutex);
    if (--w8001.open_count == 0)
    w8001_command(w8001, W8001_CMD_STOP, false);
    }
#[no_mangle]
unsafe extern "C" fn w8001_detect(w8001: *mut w8001) -> c_int {
    static int w8001_detect(struct w8001 *w8001)
    {
    int error;
    error = w8001_command(w8001, W8001_CMD_STOP, false);
    if (error)
    return error;
    msleep(250);	/* wait 250ms before querying the device */
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn w8001_append_suffix(dest: *mut c_char, suffix: *const c_char, dest_sz: usize) {
    static void w8001_append_suffix(char *dest, const char *suffix, size_t dest_sz)
    {
    let mut used: usize = strnlen(dest, dest_sz);
    strscpy(dest + used, suffix, dest_sz - used);
    }
    static int w8001_setup_pen(struct w8001 *w8001, char *basename,
    size_t basename_sz)
    {
    struct input_dev *dev = w8001.pen_dev;
    struct w8001_coord coord;
    int error;
// penabled?
    error = w8001_command(w8001, W8001_CMD_QUERY, true);
    if (error)
    return error;
    __set_bit(EV_KEY, dev.evbit);
    __set_bit(EV_ABS, dev.evbit);
    __set_bit(BTN_TOUCH, dev.keybit);
    __set_bit(BTN_TOOL_PEN, dev.keybit);
    __set_bit(BTN_TOOL_RUBBER, dev.keybit);
    __set_bit(BTN_STYLUS, dev.keybit);
    __set_bit(BTN_STYLUS2, dev.keybit);
    __set_bit(INPUT_PROP_DIRECT, dev.propbit);
    parse_pen_data(w8001.response, &coord);
    w8001.max_pen_x = coord.x;
    w8001.max_pen_y = coord.y;
    input_set_abs_params(dev, ABS_X, 0, coord.x, 0, 0);
    input_set_abs_params(dev, ABS_Y, 0, coord.y, 0, 0);
    input_abs_set_res(dev, ABS_X, W8001_PEN_RESOLUTION);
    input_abs_set_res(dev, ABS_Y, W8001_PEN_RESOLUTION);
    input_set_abs_params(dev, ABS_PRESSURE, 0, coord.pen_pressure, 0, 0);
    if (coord.tilt_x && coord.tilt_y) {
    input_set_abs_params(dev, ABS_TILT_X, 0, coord.tilt_x, 0, 0);
    input_set_abs_params(dev, ABS_TILT_Y, 0, coord.tilt_y, 0, 0);
    }
    w8001.id = 0x90;
    w8001_append_suffix(basename, " Penabled", basename_sz);
    return 0;
    }
    static int w8001_setup_touch(struct w8001 *w8001, char *basename,
    size_t basename_sz)
    {
    struct input_dev *dev = w8001.touch_dev;
    struct w8001_touch_query touch;
    int error;
// Touch enabled?
    error = w8001_command(w8001, W8001_CMD_TOUCHQUERY, true);
    if (error)
    return error;
//
// Some non-touch devices may reply to the touch query. But their
// second byte is empty, which indicates touch is not supported.
//
    if (!w8001.response[1])
    return -ENXIO;
    __set_bit(EV_KEY, dev.evbit);
    __set_bit(EV_ABS, dev.evbit);
    __set_bit(BTN_TOUCH, dev.keybit);
    __set_bit(INPUT_PROP_DIRECT, dev.propbit);
    parse_touchquery(w8001.response, &touch);
    w8001.max_touch_x = touch.x;
    w8001.max_touch_y = touch.y;
    if (w8001.max_pen_x && w8001.max_pen_y) {
// if pen is supported scale to pen maximum
    touch.x = w8001.max_pen_x;
    touch.y = w8001.max_pen_y;
    touch.panel_res = W8001_PEN_RESOLUTION;
    }
    input_set_abs_params(dev, ABS_X, 0, touch.x, 0, 0);
    input_set_abs_params(dev, ABS_Y, 0, touch.y, 0, 0);
    input_abs_set_res(dev, ABS_X, touch.panel_res);
    input_abs_set_res(dev, ABS_Y, touch.panel_res);
    switch (touch.sensor_id) {
    case 0:
    case 2:
    w8001.pktlen = W8001_PKTLEN_TOUCH93;
    w8001.id = 0x93;
    w8001_append_suffix(basename, " 1FG", basename_sz);
    break;
    case 1:
    case 3:
    case 4:
    w8001.pktlen = W8001_PKTLEN_TOUCH9A;
    w8001_append_suffix(basename, " 1FG", basename_sz);
    w8001.id = 0x9a;
    break;
    case 5:
    w8001.pktlen = W8001_PKTLEN_TOUCH2FG;
    __set_bit(BTN_TOOL_DOUBLETAP, dev.keybit);
    error = input_mt_init_slots(dev, 2, 0);
    if (error) {
    dev_err(&w8001.serio.dev,
    "failed to initialize MT slots: %d\n", error);
    return error;
    }
    input_set_abs_params(dev, ABS_MT_POSITION_X,
    0, touch.x, 0, 0);
    input_set_abs_params(dev, ABS_MT_POSITION_Y,
    0, touch.y, 0, 0);
    input_set_abs_params(dev, ABS_MT_TOOL_TYPE,
    0, MT_TOOL_MAX, 0, 0);
    input_abs_set_res(dev, ABS_MT_POSITION_X, touch.panel_res);
    input_abs_set_res(dev, ABS_MT_POSITION_Y, touch.panel_res);
    w8001_append_suffix(basename, " 2FG", basename_sz);
    if (w8001.max_pen_x && w8001.max_pen_y)
    w8001.id = 0xE3;
    else
    w8001.id = 0xE2;
    break;
    }
    w8001_append_suffix(basename, " Touchscreen", basename_sz);
    return 0;
    }
    static void w8001_set_devdata(struct input_dev *dev, struct w8001 *w8001,
    struct serio *serio)
    {
    dev.phys = w8001.phys;
    dev.id.bustype = BUS_RS232;
    dev.id.product = w8001.id;
    dev.id.vendor = 0x056a;
    dev.id.version = 0x0100;
    dev.open = w8001_open;
    dev.close = w8001_close;
    dev.dev.parent = &serio.dev;
    input_set_drvdata(dev, w8001);
    }
//
// w8001_disconnect() is the opposite of w8001_connect()
//
#[no_mangle]
unsafe extern "C" fn w8001_disconnect(serio: *mut serio) {
    static void w8001_disconnect(struct serio *serio)
    {
    struct w8001 *w8001 = serio_get_drvdata(serio);
    serio_close(serio);
    if (w8001.pen_dev)
    input_unregister_device(w8001.pen_dev);
    if (w8001.touch_dev)
    input_unregister_device(w8001.touch_dev);
    kfree(w8001);
    serio_set_drvdata(serio, core::ptr::null_mut());
    }
//
// w8001_connect() is the routine that is called when someone adds a
// new serio device that supports the w8001 protocol and registers it as
// an input device.
//
#[no_mangle]
unsafe extern "C" fn w8001_connect(serio: *mut serio, drv: *mut serio_driver) -> c_int {
    static int w8001_connect(struct serio *serio, struct serio_driver *drv)
    {
    struct w8001 *w8001;
    struct input_dev *input_dev_pen;
    struct input_dev *input_dev_touch;
    char basename[64] = "Wacom Serial";
    int err, err_pen, err_touch;
    w8001 = kzalloc_obj(*w8001);
    input_dev_pen = input_allocate_device();
    input_dev_touch = input_allocate_device();
    if (!w8001 || !input_dev_pen || !input_dev_touch) {
    err = -ENOMEM;
    goto fail1;
    }
    w8001.serio = serio;
    w8001.pen_dev = input_dev_pen;
    w8001.touch_dev = input_dev_touch;
    mutex_init(&w8001.mutex);
    init_completion(&w8001.cmd_done);
    snprintf(w8001.phys, sizeof(w8001.phys), "%s/input0", serio.phys);
    serio_set_drvdata(serio, w8001);
    err = serio_open(serio, drv);
    if (err)
    goto fail2;
    err = w8001_detect(w8001);
    if (err)
    goto fail3;
// For backwards-compatibility we compose the basename based on
// capabilities and then just append the tool type
//
    err_pen = w8001_setup_pen(w8001, basename, sizeof(basename));
    err_touch = w8001_setup_touch(w8001, basename, sizeof(basename));
    if (err_pen && err_touch) {
    err = -ENXIO;
    goto fail3;
    }
    if (!err_pen) {
    snprintf(w8001.pen_name, sizeof(w8001.pen_name),
    "%s Pen", basename);
    input_dev_pen.name = w8001.pen_name;
    w8001_set_devdata(input_dev_pen, w8001, serio);
    err = input_register_device(w8001.pen_dev);
    if (err)
    goto fail3;
    } else {
    input_free_device(input_dev_pen);
    input_dev_pen = core::ptr::null_mut();
    w8001.pen_dev = core::ptr::null_mut();
    }
    if (!err_touch) {
    snprintf(w8001.touch_name, sizeof(w8001.touch_name),
    "%s Finger", basename);
    input_dev_touch.name = w8001.touch_name;
    w8001_set_devdata(input_dev_touch, w8001, serio);
    err = input_register_device(w8001.touch_dev);
    if (err)
    goto fail4;
    } else {
    input_free_device(input_dev_touch);
    input_dev_touch = core::ptr::null_mut();
    w8001.touch_dev = core::ptr::null_mut();
    }
    return 0;
    fail4:
    if (w8001.pen_dev)
    input_unregister_device(w8001.pen_dev);
    fail3:
    serio_close(serio);
    fail2:
    serio_set_drvdata(serio, core::ptr::null_mut());
    fail1:
    input_free_device(input_dev_pen);
    input_free_device(input_dev_touch);
    kfree(w8001);
    return err;
    }
    static const struct serio_device_id w8001_serio_ids[] = {
    {
    .type	= SERIO_RS232,
    .proto	= SERIO_W8001,
    .id	= SERIO_ANY,
    .extra	= SERIO_ANY,
    },
    { 0 }
    };
    MODULE_DEVICE_TABLE(serio, w8001_serio_ids);
    static struct serio_driver w8001_drv = {
    .driver		= {
    .name	= "w8001",
    },
    .description	= DRIVER_DESC,
    .id_table	= w8001_serio_ids,
    .interrupt	= w8001_interrupt,
    .connect	= w8001_connect,
    .disconnect	= w8001_disconnect,
    };
    module_serio_driver(w8001_drv);
