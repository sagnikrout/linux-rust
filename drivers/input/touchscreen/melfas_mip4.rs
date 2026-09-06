//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/melfas_mip4.c
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
// MELFAS MIP4 Touchscreen
//
// Copyright (C) 2016 MELFAS Inc.
//
// Author : Sangwon Jee <jeesw@melfas.com>
//

//
// Protocol
// Version : MIP 4.0 Rev 5.4
//
// Address
pub const MIP4_R0_BOOT: c_uint = 0x00;
pub const MIP4_R1_BOOT_MODE: c_uint = 0x01;
pub const MIP4_R1_BOOT_BUF_ADDR: c_uint = 0x10;
pub const MIP4_R1_BOOT_STATUS: c_uint = 0x20;
pub const MIP4_R1_BOOT_CMD: c_uint = 0x30;
pub const MIP4_R1_BOOT_TARGET_ADDR: c_uint = 0x40;
pub const MIP4_R1_BOOT_SIZE: c_uint = 0x44;
pub const MIP4_R0_INFO: c_uint = 0x01;
pub const MIP4_R1_INFO_PRODUCT_NAME: c_uint = 0x00;
pub const MIP4_R1_INFO_RESOLUTION_X: c_uint = 0x10;
pub const MIP4_R1_INFO_RESOLUTION_Y: c_uint = 0x12;
pub const MIP4_R1_INFO_NODE_NUM_X: c_uint = 0x14;
pub const MIP4_R1_INFO_NODE_NUM_Y: c_uint = 0x15;
pub const MIP4_R1_INFO_KEY_NUM: c_uint = 0x16;
pub const MIP4_R1_INFO_PRESSURE_NUM: c_uint = 0x17;
pub const MIP4_R1_INFO_LENGTH_X: c_uint = 0x18;
pub const MIP4_R1_INFO_LENGTH_Y: c_uint = 0x1A;
pub const MIP4_R1_INFO_PPM_X: c_uint = 0x1C;
pub const MIP4_R1_INFO_PPM_Y: c_uint = 0x1D;
pub const MIP4_R1_INFO_VERSION_BOOT: c_uint = 0x20;
pub const MIP4_R1_INFO_VERSION_CORE: c_uint = 0x22;
pub const MIP4_R1_INFO_VERSION_APP: c_uint = 0x24;
pub const MIP4_R1_INFO_VERSION_PARAM: c_uint = 0x26;
pub const MIP4_R1_INFO_SECT_BOOT_START: c_uint = 0x30;
pub const MIP4_R1_INFO_SECT_BOOT_END: c_uint = 0x31;
pub const MIP4_R1_INFO_SECT_CORE_START: c_uint = 0x32;
pub const MIP4_R1_INFO_SECT_CORE_END: c_uint = 0x33;
pub const MIP4_R1_INFO_SECT_APP_START: c_uint = 0x34;
pub const MIP4_R1_INFO_SECT_APP_END: c_uint = 0x35;
pub const MIP4_R1_INFO_SECT_PARAM_START: c_uint = 0x36;
pub const MIP4_R1_INFO_SECT_PARAM_END: c_uint = 0x37;
pub const MIP4_R1_INFO_BUILD_DATE: c_uint = 0x40;
pub const MIP4_R1_INFO_BUILD_TIME: c_uint = 0x44;
pub const MIP4_R1_INFO_CHECKSUM_PRECALC: c_uint = 0x48;
pub const MIP4_R1_INFO_CHECKSUM_REALTIME: c_uint = 0x4A;
pub const MIP4_R1_INFO_PROTOCOL_NAME: c_uint = 0x50;
pub const MIP4_R1_INFO_PROTOCOL_VERSION: c_uint = 0x58;
pub const MIP4_R1_INFO_IC_ID: c_uint = 0x70;
pub const MIP4_R1_INFO_IC_NAME: c_uint = 0x71;
pub const MIP4_R1_INFO_IC_VENDOR_ID: c_uint = 0x75;
pub const MIP4_R1_INFO_IC_HW_CATEGORY: c_uint = 0x77;
pub const MIP4_R1_INFO_CONTACT_THD_SCR: c_uint = 0x78;
pub const MIP4_R1_INFO_CONTACT_THD_KEY: c_uint = 0x7A;
pub const MIP4_R1_INFO_PID: c_uint = 0x7C;
pub const MIP4_R1_INFO_VID: c_uint = 0x7E;
pub const MIP4_R1_INFO_SLAVE_ADDR: c_uint = 0x80;
pub const MIP4_R0_EVENT: c_uint = 0x02;
pub const MIP4_R1_EVENT_SUPPORTED_FUNC: c_uint = 0x00;
pub const MIP4_R1_EVENT_FORMAT: c_uint = 0x04;
pub const MIP4_R1_EVENT_SIZE: c_uint = 0x06;
pub const MIP4_R1_EVENT_PACKET_INFO: c_uint = 0x10;
pub const MIP4_R1_EVENT_PACKET_DATA: c_uint = 0x11;
pub const MIP4_R0_CTRL: c_uint = 0x06;
pub const MIP4_R1_CTRL_READY_STATUS: c_uint = 0x00;
pub const MIP4_R1_CTRL_EVENT_READY: c_uint = 0x01;
pub const MIP4_R1_CTRL_MODE: c_uint = 0x10;
pub const MIP4_R1_CTRL_EVENT_TRIGGER_TYPE: c_uint = 0x11;
pub const MIP4_R1_CTRL_RECALIBRATE: c_uint = 0x12;
pub const MIP4_R1_CTRL_POWER_STATE: c_uint = 0x13;
pub const MIP4_R1_CTRL_GESTURE_TYPE: c_uint = 0x14;
pub const MIP4_R1_CTRL_DISABLE_ESD_ALERT: c_uint = 0x18;
pub const MIP4_R1_CTRL_CHARGER_MODE: c_uint = 0x19;
pub const MIP4_R1_CTRL_HIGH_SENS_MODE: c_uint = 0x1A;
pub const MIP4_R1_CTRL_WINDOW_MODE: c_uint = 0x1B;
pub const MIP4_R1_CTRL_PALM_REJECTION: c_uint = 0x1C;
pub const MIP4_R1_CTRL_EDGE_CORRECTION: c_uint = 0x1D;
pub const MIP4_R1_CTRL_ENTER_GLOVE_MODE: c_uint = 0x1E;
pub const MIP4_R1_CTRL_I2C_ON_LPM: c_uint = 0x1F;
pub const MIP4_R1_CTRL_GESTURE_DEBUG: c_uint = 0x20;
pub const MIP4_R1_CTRL_PALM_EVENT: c_uint = 0x22;
pub const MIP4_R1_CTRL_PROXIMITY_SENSING: c_uint = 0x23;
// Value
pub const MIP4_BOOT_MODE_BOOT: c_uint = 0x01;
pub const MIP4_BOOT_MODE_APP: c_uint = 0x02;
pub const MIP4_BOOT_STATUS_BUSY: c_uint = 0x05;
pub const MIP4_BOOT_STATUS_ERROR: c_uint = 0x0E;
pub const MIP4_BOOT_STATUS_DONE: c_uint = 0xA0;
pub const MIP4_BOOT_CMD_MASS_ERASE: c_uint = 0x15;
pub const MIP4_BOOT_CMD_PROGRAM: c_uint = 0x54;
pub const MIP4_BOOT_CMD_ERASE: c_uint = 0x8F;
pub const MIP4_BOOT_CMD_WRITE: c_uint = 0xA5;
pub const MIP4_BOOT_CMD_READ: c_uint = 0xC2;
pub const MIP4_EVENT_INPUT_TYPE_KEY: c_int = 0;
pub const MIP4_EVENT_INPUT_TYPE_SCREEN: c_int = 1;
pub const MIP4_EVENT_INPUT_TYPE_PROXIMITY: c_int = 2;

pub const MIP4_BUF_SIZE: c_int = 128;
pub const MIP4_MAX_FINGERS: c_int = 10;
pub const MIP4_MAX_KEYS: c_int = 4;
pub const MIP4_TOUCH_MAJOR_MIN: c_int = 0;
pub const MIP4_TOUCH_MAJOR_MAX: c_int = 255;
pub const MIP4_TOUCH_MINOR_MIN: c_int = 0;
pub const MIP4_TOUCH_MINOR_MAX: c_int = 255;
pub const MIP4_PRESSURE_MIN: c_int = 0;
pub const MIP4_PRESSURE_MAX: c_int = 255;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mip4_fw_version {
    pub boot: u16,
    pub core: u16,
    pub app: u16,
    pub param: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mip4_ts {
    pub client: *mut i2c_client,
    pub input: *mut input_dev,
    pub gpio_ce: *mut gpio_desc,
    pub phys: [c_char; 32],
    pub product_name: [c_char; 16],
    pub product_id: u16,
    pub ic_name: [c_char; 4],
    pub fw_name: [c_char; 32],
    pub max_x: c_uint,
    pub max_y: c_uint,
    pub node_x: u8,
    pub node_y: u8,
    pub node_key: u8,
    pub ppm_x: c_uint,
    pub ppm_y: c_uint,
    pub fw_version: mip4_fw_version,
    pub event_size: c_uint,
    pub event_format: c_uint,
    pub key_num: c_uint,
    pub key_code: [c_ushort; MIP4_MAX_KEYS],
    pub wake_irq_enabled: bool,
    pub buf: [u8; MIP4_BUF_SIZE],
}

    static int mip4_i2c_xfer(struct mip4_ts *ts,
    char *write_buf, unsigned int write_len,
    char *read_buf, unsigned int read_len)
    {
    struct i2c_msg msg[] = {
    {
    .addr = ts.client.addr,
    .flags = 0,
    .buf = write_buf,
    .len = write_len,
    }, {
    .addr = ts.client.addr,
    .flags = I2C_M_RD,
    .buf = read_buf,
    .len = read_len,
    },
    };
    let mut retry: c_int = I2C_RETRY_COUNT;
    int res;
    int error;
    do {
    res = i2c_transfer(ts.client.adapter, msg, ARRAY_SIZE(msg));
    if (res == ARRAY_SIZE(msg))
    return 0;
    error = res < 0 ? res : -EIO;
    dev_err(&ts.client.dev,
    "%s - i2c_transfer failed: %d (%d)\n",
    __func__, error, res);
    } while (--retry);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn mip4_parse_fw_version(buf: *const u8, v: *mut mip4_fw_version) {
    static void mip4_parse_fw_version(const u8 *buf, struct mip4_fw_version *v)
    {
    v.boot  = get_unaligned_le16(buf + 0);
    v.core  = get_unaligned_le16(buf + 2);
    v.app   = get_unaligned_le16(buf + 4);
    v.param = get_unaligned_le16(buf + 6);
    }
//
// Read chip firmware version
//
#[no_mangle]
unsafe extern "C" fn mip4_get_fw_version(ts: *mut mip4_ts) -> c_int {
    static int mip4_get_fw_version(struct mip4_ts *ts)
    {
    u8 cmd[] = { MIP4_R0_INFO, MIP4_R1_INFO_VERSION_BOOT };
    u8 buf[sizeof(ts.fw_version)];
    int error;
    error = mip4_i2c_xfer(ts, cmd, sizeof(cmd), buf, sizeof(buf));
    if (error) {
    memset(&ts.fw_version, 0xff, sizeof(ts.fw_version));
    return error;
    }
    mip4_parse_fw_version(buf, &ts.fw_version);
    return 0;
    }
//
// Fetch device characteristics
//
#[no_mangle]
unsafe extern "C" fn mip4_query_device(ts: *mut mip4_ts) -> c_int {
    static int mip4_query_device(struct mip4_ts *ts)
    {
    union i2c_smbus_data dummy;
    int error;
    u8 cmd[2];
    u8 buf[14];
//
// Make sure there is something at this address as we do not
// consider subsequent failures as fatal.
//
    if (i2c_smbus_xfer(ts.client.adapter, ts.client.addr,
    0, I2C_SMBUS_READ, 0, I2C_SMBUS_BYTE, &dummy) < 0) {
    dev_err(&ts.client.dev, "nothing at this address\n");
    return -ENXIO;
    }
// Product name
    cmd[0] = MIP4_R0_INFO;
    cmd[1] = MIP4_R1_INFO_PRODUCT_NAME;
    error = mip4_i2c_xfer(ts, cmd, sizeof(cmd),
    ts.product_name, sizeof(ts.product_name));
    if (error)
    dev_warn(&ts.client.dev,
    "Failed to retrieve product name: %d\n", error);
    else
    dev_dbg(&ts.client.dev, "product name: %.*s\n",
    (int)sizeof(ts.product_name), ts.product_name);
// Product ID
    cmd[0] = MIP4_R0_INFO;
    cmd[1] = MIP4_R1_INFO_PID;
    error = mip4_i2c_xfer(ts, cmd, sizeof(cmd), buf, 2);
    if (error) {
    dev_warn(&ts.client.dev,
    "Failed to retrieve product id: %d\n", error);
    } else {
    ts.product_id = get_unaligned_le16(&buf[0]);
    dev_dbg(&ts.client.dev, "product id: %04X\n", ts.product_id);
    }
// Firmware name
    snprintf(ts.fw_name, sizeof(ts.fw_name),
    "melfas_mip4_%04X.fw", ts.product_id);
    dev_dbg(&ts.client.dev, "firmware name: %s\n", ts.fw_name);
// IC name
    cmd[0] = MIP4_R0_INFO;
    cmd[1] = MIP4_R1_INFO_IC_NAME;
    error = mip4_i2c_xfer(ts, cmd, sizeof(cmd),
    ts.ic_name, sizeof(ts.ic_name));
    if (error)
    dev_warn(&ts.client.dev,
    "Failed to retrieve IC name: %d\n", error);
    else
    dev_dbg(&ts.client.dev, "IC name: %.*s\n",
    (int)sizeof(ts.ic_name), ts.ic_name);
// Firmware version
    error = mip4_get_fw_version(ts);
    if (error)
    dev_warn(&ts.client.dev,
    "Failed to retrieve FW version: %d\n", error);
    else
    dev_dbg(&ts.client.dev, "F/W Version: %04X %04X %04X %04X\n",
    ts.fw_version.boot, ts.fw_version.core,
    ts.fw_version.app, ts.fw_version.param);
// Resolution
    cmd[0] = MIP4_R0_INFO;
    cmd[1] = MIP4_R1_INFO_RESOLUTION_X;
    error = mip4_i2c_xfer(ts, cmd, sizeof(cmd), buf, 14);
    if (error) {
    dev_warn(&ts.client.dev,
    "Failed to retrieve touchscreen parameters: %d\n",
    error);
    } else {
    ts.max_x = get_unaligned_le16(&buf[0]);
    ts.max_y = get_unaligned_le16(&buf[2]);
    dev_dbg(&ts.client.dev, "max_x: %d, max_y: %d\n",
    ts.max_x, ts.max_y);
    ts.node_x = buf[4];
    ts.node_y = buf[5];
    ts.node_key = buf[6];
    dev_dbg(&ts.client.dev,
    "node_x: %d, node_y: %d, node_key: %d\n",
    ts.node_x, ts.node_y, ts.node_key);
    ts.ppm_x = buf[12];
    ts.ppm_y = buf[13];
    dev_dbg(&ts.client.dev, "ppm_x: %d, ppm_y: %d\n",
    ts.ppm_x, ts.ppm_y);
// Key ts
    if (ts.node_key > 0)
    ts.key_num = ts.node_key;
    }
// Protocol
    cmd[0] = MIP4_R0_EVENT;
    cmd[1] = MIP4_R1_EVENT_SUPPORTED_FUNC;
    error = mip4_i2c_xfer(ts, cmd, sizeof(cmd), buf, 7);
    if (error) {
    dev_warn(&ts.client.dev,
    "Failed to retrieve device type: %d\n", error);
    ts.event_format = 0xff;
    } else {
    ts.event_format = get_unaligned_le16(&buf[4]);
    ts.event_size = buf[6];
    dev_dbg(&ts.client.dev, "event_format: %d, event_size: %d\n",
    ts.event_format, ts.event_size);
    if (ts.event_format == 2 || ts.event_format > 3)
    dev_warn(&ts.client.dev,
    "Unknown event format %d\n", ts.event_format);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mip4_power_on(ts: *mut mip4_ts) -> c_int {
    static int mip4_power_on(struct mip4_ts *ts)
    {
    if (ts.gpio_ce) {
    gpiod_set_value_cansleep(ts.gpio_ce, 1);
// Booting delay : 200~300ms
    usleep_range(200 * 1000, 300 * 1000);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mip4_power_off(ts: *mut mip4_ts) {
    static void mip4_power_off(struct mip4_ts *ts)
    {
    if (ts.gpio_ce)
    gpiod_set_value_cansleep(ts.gpio_ce, 0);
    }
//
// Clear touch input event status
//
#[no_mangle]
unsafe extern "C" fn mip4_clear_input(ts: *mut mip4_ts) {
    static void mip4_clear_input(struct mip4_ts *ts)
    {
    int i;
// Screen
    for (i = 0; i < MIP4_MAX_FINGERS; i++) {
    input_mt_slot(ts.input, i);
    input_mt_report_slot_inactive(ts.input);
    }
// Keys
    for (i = 0; i < ts.key_num; i++)
    input_report_key(ts.input, ts.key_code[i], 0);
    input_sync(ts.input);
    }
#[no_mangle]
unsafe extern "C" fn mip4_enable(ts: *mut mip4_ts) -> c_int {
    static int mip4_enable(struct mip4_ts *ts)
    {
    int error;
    error = mip4_power_on(ts);
    if (error)
    return error;
    enable_irq(ts.client.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mip4_disable(ts: *mut mip4_ts) {
    static void mip4_disable(struct mip4_ts *ts)
    {
    disable_irq(ts.client.irq);
    mip4_power_off(ts);
    mip4_clear_input(ts);
    }
//
// Input handling
//
#[no_mangle]
unsafe extern "C" fn mip4_report_keys(ts: *mut mip4_ts, packet: *mut u8) {
    static void mip4_report_keys(struct mip4_ts *ts, u8 *packet)
    {
    u8 key;
    bool down;
    switch (ts.event_format) {
    case 0:
    case 1:
    key = packet[0] & 0x0F;
    down = packet[0] & 0x80;
    break;
    case 3:
    default:
    key = packet[0] & 0x0F;
    down = packet[1] & 0x01;
    break;
    }
// Report key event
    if (key >= 1 && key <= ts.key_num) {
    let mut keycode: c_ushort = ts.key_code[key - 1];
    dev_dbg(&ts.client.dev,
    "Key - ID: %d, keycode: %d, state: %d\n",
    key, keycode, down);
    input_event(ts.input, EV_MSC, MSC_SCAN, keycode);
    input_report_key(ts.input, keycode, down);
    } else {
    dev_err(&ts.client.dev, "Unknown key: %d\n", key);
    }
    }
#[no_mangle]
unsafe extern "C" fn mip4_report_touch(ts: *mut mip4_ts, packet: *mut u8) {
    static void mip4_report_touch(struct mip4_ts *ts, u8 *packet)
    {
    int id;
    bool __always_unused hover;
    bool palm;
    bool state;
    u16 x, y;
    let mut pressure_stage: u8 __always_unused = 0;
    u8 pressure;
    u8 __always_unused size;
    u8 touch_major;
    u8 touch_minor;
    switch (ts.event_format) {
    case 0:
    case 1:
// Touch only
    state = packet[0] & BIT(7);
    hover = packet[0] & BIT(5);
    palm = packet[0] & BIT(4);
    id = (packet[0] & 0x0F) - 1;
    x = ((packet[1] & 0x0F) << 8) | packet[2];
    y = (((packet[1] >> 4) & 0x0F) << 8) |
    packet[3];
    pressure = packet[4];
    size = packet[5];
    if (ts.event_format == 0) {
    touch_major = packet[5];
    touch_minor = packet[5];
    } else {
    touch_major = packet[6];
    touch_minor = packet[7];
    }
    break;
    case 3:
    default:
// Touch + Force(Pressure)
    id = (packet[0] & 0x0F) - 1;
    hover = packet[1] & BIT(2);
    palm = packet[1] & BIT(1);
    state = packet[1] & BIT(0);
    x = ((packet[2] & 0x0F) << 8) | packet[3];
    y = (((packet[2] >> 4) & 0x0F) << 8) |
    packet[4];
    size = packet[6];
    pressure_stage = (packet[7] & 0xF0) >> 4;
    pressure = ((packet[7] & 0x0F) << 8) |
    packet[8];
    touch_major = packet[9];
    touch_minor = packet[10];
    break;
    }
    dev_dbg(&ts.client.dev,
    "Screen - Slot: %d State: %d X: %04d Y: %04d Z: %d\n",
    id, state, x, y, pressure);
    if (unlikely(id < 0 || id >= MIP4_MAX_FINGERS)) {
    dev_err(&ts.client.dev, "Screen - invalid slot ID: %d\n", id);
    goto out;
    }
    input_mt_slot(ts.input, id);
    if (input_mt_report_slot_state(ts.input,
    palm ? MT_TOOL_PALM : MT_TOOL_FINGER,
    state)) {
    input_report_abs(ts.input, ABS_MT_POSITION_X, x);
    input_report_abs(ts.input, ABS_MT_POSITION_Y, y);
    input_report_abs(ts.input, ABS_MT_PRESSURE, pressure);
    input_report_abs(ts.input, ABS_MT_TOUCH_MAJOR, touch_major);
    input_report_abs(ts.input, ABS_MT_TOUCH_MINOR, touch_minor);
    }
    out:
    input_mt_sync_frame(ts.input);
    }
#[no_mangle]
unsafe extern "C" fn mip4_handle_packet(ts: *mut mip4_ts, packet: *mut u8) -> c_int {
    static int mip4_handle_packet(struct mip4_ts *ts, u8 *packet)
    {
    u8 type;
    switch (ts.event_format) {
    case 0:
    case 1:
    type = (packet[0] & 0x40) >> 6;
    break;
    case 3:
    type = (packet[0] & 0xF0) >> 4;
    break;
    default:
// Should not happen unless we have corrupted firmware
    return -EINVAL;
    }
    dev_dbg(&ts.client.dev, "Type: %d\n", type);
// Report input event
    switch (type) {
    case MIP4_EVENT_INPUT_TYPE_KEY:
    mip4_report_keys(ts, packet);
    break;
    case MIP4_EVENT_INPUT_TYPE_SCREEN:
    mip4_report_touch(ts, packet);
    break;
    default:
    dev_err(&ts.client.dev, "Unknown event type: %d\n", type);
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mip4_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t mip4_interrupt(int irq, void *dev_id)
    {
    struct mip4_ts *ts = dev_id;
    struct i2c_client *client = ts.client;
    unsigned int i;
    int error;
    u8 cmd[2];
    u8 size;
    bool alert;
// Read packet info
    cmd[0] = MIP4_R0_EVENT;
    cmd[1] = MIP4_R1_EVENT_PACKET_INFO;
    error = mip4_i2c_xfer(ts, cmd, sizeof(cmd), ts.buf, 1);
    if (error) {
    dev_err(&client.dev,
    "Failed to read packet info: %d\n", error);
    goto out;
    }
    size = ts.buf[0] & 0x7F;
    alert = ts.buf[0] & BIT(7);
    dev_dbg(&client.dev, "packet size: %d, alert: %d\n", size, alert);
// Check size
    if (!size) {
    dev_err(&client.dev, "Empty packet\n");
    goto out;
    }
// Read packet data
    cmd[0] = MIP4_R0_EVENT;
    cmd[1] = MIP4_R1_EVENT_PACKET_DATA;
    error = mip4_i2c_xfer(ts, cmd, sizeof(cmd), ts.buf, size);
    if (error) {
    dev_err(&client.dev,
    "Failed to read packet data: %d\n", error);
    goto out;
    }
    if (alert) {
    dev_dbg(&client.dev, "Alert: %d\n", ts.buf[0]);
    } else {
    for (i = 0; i < size; i += ts.event_size) {
    error = mip4_handle_packet(ts, &ts.buf[i]);
    if (error)
    break;
    }
    input_sync(ts.input);
    }
    out:
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn mip4_input_open(dev: *mut input_dev) -> c_int {
    static int mip4_input_open(struct input_dev *dev)
    {
    struct mip4_ts *ts = input_get_drvdata(dev);
    return mip4_enable(ts);
    }
#[no_mangle]
unsafe extern "C" fn mip4_input_close(dev: *mut input_dev) {
    static void mip4_input_close(struct input_dev *dev)
    {
    struct mip4_ts *ts = input_get_drvdata(dev);
    mip4_disable(ts);
    }
//
// Firmware update
//
// Firmware Info

//
// Firmware binary tail info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mip4_bin_tail {
    pub tail_mark: [u8; 4],
    pub chip_name: [u8; 4],
    pub bin_start_addr: __le32,
    pub bin_length: __le32,
    pub ver_boot: __le16,
    pub ver_core: __le16,
    pub ver_app: __le16,
    pub ver_param: __le16,
    pub boot_start: u8,
    pub boot_end: u8,
    pub core_start: u8,
    pub core_end: u8,
    pub app_start: u8,
    pub app_end: u8,
    pub param_start: u8,
    pub param_end: u8,
    pub checksum_type: u8,
    pub hw_category: u8,
    pub param_id: __le16,
    pub param_length: __le32,
    pub build_date: __le32,
    pub build_time: __le32,
    pub reserved1: __le32,
    pub reserved2: __le32,
    pub reserved3: __le16,
    pub tail_size: __le16,
    pub crc: __le32,
    pub __packed: },

//
// Bootloader - Read status
//
#[no_mangle]
unsafe extern "C" fn mip4_bl_read_status(ts: *mut mip4_ts) -> c_int {
    static int mip4_bl_read_status(struct mip4_ts *ts)
    {
    pub }: u8 cmd[] = { MIP4_R0_BOOT, MIP4_R1_BOOT_STATUS,
    pub result: u8,
    struct i2c_msg msg[] = {
    {
    .addr = ts.client.addr,
    .flags = 0,
    .buf = cmd,
    .len = sizeof(cmd),
    }, {
    .addr = ts.client.addr,
    .flags = I2C_M_RD,
    .buf = &result,
    .len = sizeof(result),
    },
}

    int ret;
    int error;
    let mut retry: c_int = 1000;
    do {
    ret = i2c_transfer(ts.client.adapter, msg, ARRAY_SIZE(msg));
    if (ret != ARRAY_SIZE(msg)) {
    error = ret < 0 ? ret : -EIO;
    dev_err(&ts.client.dev,
    "Failed to read bootloader status: %d\n",
    error);
    return error;
    }
    switch (result) {
    case MIP4_BOOT_STATUS_DONE:
    dev_dbg(&ts.client.dev, "%s - done\n", __func__);
    return 0;
    case MIP4_BOOT_STATUS_ERROR:
    dev_err(&ts.client.dev, "Bootloader failure\n");
    return -EIO;
    case MIP4_BOOT_STATUS_BUSY:
    dev_dbg(&ts.client.dev, "%s - Busy\n", __func__);
    error = -EBUSY;
    break;
    default:
    dev_err(&ts.client.dev,
    "Unexpected bootloader status: %#02x\n",
    result);
    error = -EINVAL;
    break;
    }
    usleep_range(1000, 2000);
    } while (--retry);
    return error;
    }
//
// Bootloader - Change mode
//
#[no_mangle]
unsafe extern "C" fn mip4_bl_change_mode(ts: *mut mip4_ts, mode: u8) -> c_int {
    static int mip4_bl_change_mode(struct mip4_ts *ts, u8 mode)
    {
    u8 mode_chg_cmd[] = { MIP4_R0_BOOT, MIP4_R1_BOOT_MODE, mode };
    u8 mode_read_cmd[] = { MIP4_R0_BOOT, MIP4_R1_BOOT_MODE };
    u8 result;
    struct i2c_msg msg[] = {
    {
    .addr = ts.client.addr,
    .flags = 0,
    .buf = mode_read_cmd,
    .len = sizeof(mode_read_cmd),
    }, {
    .addr = ts.client.addr,
    .flags = I2C_M_RD,
    .buf = &result,
    .len = sizeof(result),
    },
    };
    let mut retry: c_int = 10;
    int ret;
    int error;
    do {
// Send mode change command
    ret = i2c_master_send(ts.client,
    mode_chg_cmd, sizeof(mode_chg_cmd));
    if (ret != sizeof(mode_chg_cmd)) {
    error = ret < 0 ? ret : -EIO;
    dev_err(&ts.client.dev,
    "Failed to send %d mode change: %d (%d)\n",
    mode, error, ret);
    return error;
    }
    dev_dbg(&ts.client.dev,
    "Sent mode change request (mode: %d)\n", mode);
// Wait
    msleep(1000);
// Verify target mode
    ret = i2c_transfer(ts.client.adapter, msg, ARRAY_SIZE(msg));
    if (ret != ARRAY_SIZE(msg)) {
    error = ret < 0 ? ret : -EIO;
    dev_err(&ts.client.dev,
    "Failed to read device mode: %d\n", error);
    return error;
    }
    dev_dbg(&ts.client.dev,
    "Current device mode: %d, want: %d\n", result, mode);
    if (result == mode)
    return 0;
    } while (--retry);
    return -EIO;
    }
//
// Bootloader - Start bootloader mode
//
#[no_mangle]
unsafe extern "C" fn mip4_bl_enter(ts: *mut mip4_ts) -> c_int {
    static int mip4_bl_enter(struct mip4_ts *ts)
    {
    return mip4_bl_change_mode(ts, MIP4_BOOT_MODE_BOOT);
    }
//
// Bootloader - Exit bootloader mode
//
#[no_mangle]
unsafe extern "C" fn mip4_bl_exit(ts: *mut mip4_ts) -> c_int {
    static int mip4_bl_exit(struct mip4_ts *ts)
    {
    return mip4_bl_change_mode(ts, MIP4_BOOT_MODE_APP);
    }
#[no_mangle]
unsafe extern "C" fn mip4_bl_get_address(ts: *mut mip4_ts, buf_addr: *mut u16) -> c_int {
    static int mip4_bl_get_address(struct mip4_ts *ts, u16 *buf_addr)
    {
    u8 cmd[] = { MIP4_R0_BOOT, MIP4_R1_BOOT_BUF_ADDR };
    u8 result[sizeof(u16)];
    struct i2c_msg msg[] = {
    {
    .addr = ts.client.addr,
    .flags = 0,
    .buf = cmd,
    .len = sizeof(cmd),
    }, {
    .addr = ts.client.addr,
    .flags = I2C_M_RD,
    .buf = result,
    .len = sizeof(result),
    },
    };
    int ret;
    int error;
    ret = i2c_transfer(ts.client.adapter, msg, ARRAY_SIZE(msg));
    if (ret != ARRAY_SIZE(msg)) {
    error = ret < 0 ? ret : -EIO;
    dev_err(&ts.client.dev,
    "Failed to retrieve bootloader buffer address: %d\n",
    error);
    return error;
    }
// buf_addr = get_unaligned_le16(result);
    dev_dbg(&ts.client.dev,
    "Bootloader buffer address %#04x\n", *buf_addr);
    return 0;
    }
    static int mip4_bl_program_page(struct mip4_ts *ts, int offset,
    const u8 *data, int length, u16 buf_addr)
    {
    u8 cmd[6];
    int ret;
    int error;
    dev_dbg(&ts.client.dev, "Writing page @%#06x (%d)\n",
    offset, length);
    if (length > MIP4_BL_PAGE_SIZE || length % MIP4_BL_PACKET_SIZE) {
    dev_err(&ts.client.dev,
    "Invalid page length: %d\n", length);
    return -EINVAL;
    }
    u8 *data_buf __free(kfree) = kmalloc(2 + MIP4_BL_PACKET_SIZE,
    GFP_KERNEL);
    if (!data_buf)
    return -ENOMEM;
// Addr
    cmd[0] = MIP4_R0_BOOT;
    cmd[1] = MIP4_R1_BOOT_TARGET_ADDR;
    put_unaligned_le32(offset, &cmd[2]);
    ret = i2c_master_send(ts.client, cmd, 6);
    if (ret != 6) {
    error = ret < 0 ? ret : -EIO;
    dev_err(&ts.client.dev,
    "Failed to send write page address: %d\n", error);
    return error;
    }
// Size
    cmd[0] = MIP4_R0_BOOT;
    cmd[1] = MIP4_R1_BOOT_SIZE;
    put_unaligned_le32(length, &cmd[2]);
    ret = i2c_master_send(ts.client, cmd, 6);
    if (ret != 6) {
    error = ret < 0 ? ret : -EIO;
    dev_err(&ts.client.dev,
    "Failed to send write page size: %d\n", error);
    return error;
    }
// Data
    for (int buf_offset = 0;
    buf_offset < length;
    buf_offset += MIP4_BL_PACKET_SIZE) {
    dev_dbg(&ts.client.dev,
    "writing chunk at %#04x (size %d)\n",
    buf_offset, MIP4_BL_PACKET_SIZE);
    put_unaligned_be16(buf_addr + buf_offset, data_buf);
    memcpy(&data_buf[2], &data[buf_offset], MIP4_BL_PACKET_SIZE);
    ret = i2c_master_send(ts.client,
    data_buf, 2 + MIP4_BL_PACKET_SIZE);
    if (ret != 2 + MIP4_BL_PACKET_SIZE) {
    error = ret < 0 ? ret : -EIO;
    dev_err(&ts.client.dev,
    "Failed to read chunk at %#04x (size %d): %d\n",
    buf_offset, MIP4_BL_PACKET_SIZE, error);
    return error;
    }
    }
// Command
    cmd[0] = MIP4_R0_BOOT;
    cmd[1] = MIP4_R1_BOOT_CMD;
    cmd[2] = MIP4_BOOT_CMD_PROGRAM;
    ret = i2c_master_send(ts.client, cmd, 3);
    if (ret != 3) {
    error = ret < 0 ? ret : -EIO;
    dev_err(&ts.client.dev,
    "Failed to send 'write' command: %d\n", error);
    return error;
    }
// Status
    error = mip4_bl_read_status(ts);
    if (error)
    return error;
    return 0;
    }
    static int mip4_bl_verify_page(struct mip4_ts *ts, int offset,
    const u8 *data, int length, int buf_addr)
    {
    u8 cmd[8];
    int ret;
    int error;
    dev_dbg(&ts.client.dev, "Validating page @%#06x (%d)\n",
    offset, length);
// Addr
    cmd[0] = MIP4_R0_BOOT;
    cmd[1] = MIP4_R1_BOOT_TARGET_ADDR;
    put_unaligned_le32(offset, &cmd[2]);
    ret = i2c_master_send(ts.client, cmd, 6);
    if (ret != 6) {
    error = ret < 0 ? ret : -EIO;
    dev_err(&ts.client.dev,
    "Failed to send read page address: %d\n", error);
    return error;
    }
// Size
    cmd[0] = MIP4_R0_BOOT;
    cmd[1] = MIP4_R1_BOOT_SIZE;
    put_unaligned_le32(length, &cmd[2]);
    ret = i2c_master_send(ts.client, cmd, 6);
    if (ret != 6) {
    error = ret < 0 ? ret : -EIO;
    dev_err(&ts.client.dev,
    "Failed to send read page size: %d\n", error);
    return error;
    }
// Command
    cmd[0] = MIP4_R0_BOOT;
    cmd[1] = MIP4_R1_BOOT_CMD;
    cmd[2] = MIP4_BOOT_CMD_READ;
    ret = i2c_master_send(ts.client, cmd, 3);
    if (ret != 3) {
    error = ret < 0 ? ret : -EIO;
    dev_err(&ts.client.dev,
    "Failed to send 'read' command: %d\n", error);
    return error;
    }
// Status
    error = mip4_bl_read_status(ts);
    if (error)
    return error;
// Read
    u8 *read_buf __free(kfree) = kmalloc(MIP4_BL_PACKET_SIZE, GFP_KERNEL);
    if (!read_buf)
    return -ENOMEM;
    struct i2c_msg msg[] = {
    {
    .addr = ts.client.addr,
    .flags = 0,
    .buf = cmd,
    .len = 2,
    }, {
    .addr = ts.client.addr,
    .flags = I2C_M_RD,
    .buf = read_buf,
    .len = MIP4_BL_PACKET_SIZE,
    },
    };
    for (int buf_offset = 0;
    buf_offset < length;
    buf_offset += MIP4_BL_PACKET_SIZE) {
    dev_dbg(&ts.client.dev,
    "reading chunk at %#04x (size %d)\n",
    buf_offset, MIP4_BL_PACKET_SIZE);
    put_unaligned_be16(buf_addr + buf_offset, cmd);
    ret = i2c_transfer(ts.client.adapter, msg, ARRAY_SIZE(msg));
    if (ret != ARRAY_SIZE(msg)) {
    error = ret < 0 ? ret : -EIO;
    dev_err(&ts.client.dev,
    "Failed to read chunk at %#04x (size %d): %d\n",
    buf_offset, MIP4_BL_PACKET_SIZE, error);
    return error;
    }
    if (memcmp(&data[buf_offset], read_buf, MIP4_BL_PACKET_SIZE)) {
    dev_err(&ts.client.dev,
    "Failed to validate chunk at %#04x (size %d)\n",
    buf_offset, MIP4_BL_PACKET_SIZE);

    print_hex_dump(KERN_DEBUG,
    MIP4_DEVICE_NAME " F/W File: ",
    DUMP_PREFIX_OFFSET, 16, 1,
    data + offset, MIP4_BL_PACKET_SIZE,
    false);
    print_hex_dump(KERN_DEBUG,
    MIP4_DEVICE_NAME " F/W Chip: ",
    DUMP_PREFIX_OFFSET, 16, 1,
    read_buf, MIP4_BL_PAGE_SIZE, false);

    return -EINVAL;
    }
    }
    return 0;
    }
//
// Flash chip firmware
//
    static int mip4_flash_fw(struct mip4_ts *ts,
    const u8 *fw_data, u32 fw_size, u32 fw_offset)
    {
    struct i2c_client *client = ts.client;
    int offset;
    u16 buf_addr;
    int error, error2;
// Enter bootloader mode
    dev_dbg(&client.dev, "Entering bootloader mode\n");
    error = mip4_bl_enter(ts);
    if (error) {
    dev_err(&client.dev,
    "Failed to enter bootloader mode: %d\n",
    error);
    return error;
    }
// Read info
    error = mip4_bl_get_address(ts, &buf_addr);
    if (error)
    goto exit_bl;
// Program & Verify
    dev_dbg(&client.dev,
    "Program & Verify, page size: %d, packet size: %d\n",
    MIP4_BL_PAGE_SIZE, MIP4_BL_PACKET_SIZE);
    for (offset = fw_offset;
    offset < fw_offset + fw_size;
    offset += MIP4_BL_PAGE_SIZE) {
// Program
    error = mip4_bl_program_page(ts, offset, fw_data + offset,
    MIP4_BL_PAGE_SIZE, buf_addr);
    if (error)
    break;
// Verify
    error = mip4_bl_verify_page(ts, offset, fw_data + offset,
    MIP4_BL_PAGE_SIZE, buf_addr);
    if (error)
    break;
    }
    exit_bl:
// Exit bootloader mode
    dev_dbg(&client.dev, "Exiting bootloader mode\n");
    error2 = mip4_bl_exit(ts);
    if (error2) {
    dev_err(&client.dev,
    "Failed to exit bootloader mode: %d\n", error2);
    if (!error)
    error = error2;
    }
// Reset chip
    mip4_power_off(ts);
    mip4_power_on(ts);
    mip4_query_device(ts);
// Refresh device parameters
    input_set_abs_params(ts.input, ABS_MT_POSITION_X, 0, ts.max_x, 0, 0);
    input_set_abs_params(ts.input, ABS_MT_POSITION_Y, 0, ts.max_y, 0, 0);
    input_set_abs_params(ts.input, ABS_X, 0, ts.max_x, 0, 0);
    input_set_abs_params(ts.input, ABS_Y, 0, ts.max_y, 0, 0);
    input_abs_set_res(ts.input, ABS_MT_POSITION_X, ts.ppm_x);
    input_abs_set_res(ts.input, ABS_MT_POSITION_Y, ts.ppm_y);
    input_abs_set_res(ts.input, ABS_X, ts.ppm_x);
    input_abs_set_res(ts.input, ABS_Y, ts.ppm_y);
    return error ? error : 0;
    }
    static int mip4_parse_firmware(struct mip4_ts *ts, const struct firmware *fw,
    u32 *fw_offset_start, u32 *fw_size,
    const struct mip4_bin_tail **pfw_info)
    {
    const struct mip4_bin_tail *fw_info;
    struct mip4_fw_version fw_version;
    u16 tail_size;
    if (fw.size < MIP4_BIN_TAIL_SIZE) {
    dev_err(&ts.client.dev,
    "Invalid firmware, size mismatch (tail %zd vs %zd)\n",
    MIP4_BIN_TAIL_SIZE, fw.size);
    return -EINVAL;
    }
    fw_info = (const void *)&fw.data[fw.size - MIP4_BIN_TAIL_SIZE];

    print_hex_dump(KERN_ERR, MIP4_DEVICE_NAME " Bin Info: ",
    DUMP_PREFIX_OFFSET, 16, 1, *fw_info, tail_size, false);

    tail_size = get_unaligned_le16(&fw_info.tail_size);
    if (tail_size != MIP4_BIN_TAIL_SIZE) {
    dev_err(&ts.client.dev,
    "wrong tail size: %d (expected %zd)\n",
    tail_size, MIP4_BIN_TAIL_SIZE);
    return -EINVAL;
    }
// Check bin format
    if (memcmp(fw_info.tail_mark, MIP4_BIN_TAIL_MARK,
    sizeof(fw_info.tail_mark))) {
    dev_err(&ts.client.dev,
    "unable to locate tail marker (%*ph vs %*ph)\n",
    (int)sizeof(fw_info.tail_mark), fw_info.tail_mark,
    (int)sizeof(fw_info.tail_mark), MIP4_BIN_TAIL_MARK);
    return -EINVAL;
    }
// fw_offset_start = get_unaligned_le32(&fw_info->bin_start_addr);
// fw_size = get_unaligned_le32(&fw_info->bin_length);
    dev_dbg(&ts.client.dev,
    "F/W Data offset: %#08x, size: %d\n",
// fw_offset_start, *fw_size);
    if (*fw_size % MIP4_BL_PAGE_SIZE) {
    dev_err(&ts.client.dev,
    "encoded fw length %d is not multiple of pages (%d)\n",
// fw_size, MIP4_BL_PAGE_SIZE);
    return -EINVAL;
    }
    if (fw.size != *fw_offset_start + *fw_size) {
    dev_err(&ts.client.dev,
    "Wrong firmware size, expected %d bytes, got %zd\n",
// fw_offset_start + *fw_size, fw->size);
    return -EINVAL;
    }
    mip4_parse_fw_version((const u8 *)&fw_info.ver_boot, &fw_version);
    dev_dbg(&ts.client.dev,
    "F/W file version %04X %04X %04X %04X\n",
    fw_version.boot, fw_version.core,
    fw_version.app, fw_version.param);
    dev_dbg(&ts.client.dev, "F/W chip version: %04X %04X %04X %04X\n",
    ts.fw_version.boot, ts.fw_version.core,
    ts.fw_version.app, ts.fw_version.param);
// Check F/W type
    if (fw_version.boot != 0xEEEE && fw_version.boot != 0xFFFF &&
    fw_version.core == 0xEEEE &&
    fw_version.app == 0xEEEE &&
    fw_version.param == 0xEEEE) {
    dev_dbg(&ts.client.dev, "F/W type: Bootloader\n");
    } else if (fw_version.boot == 0xEEEE &&
    fw_version.core != 0xEEEE && fw_version.core != 0xFFFF &&
    fw_version.app != 0xEEEE && fw_version.app != 0xFFFF &&
    fw_version.param != 0xEEEE && fw_version.param != 0xFFFF) {
    dev_dbg(&ts.client.dev, "F/W type: Main\n");
    } else {
    dev_err(&ts.client.dev, "Wrong firmware type\n");
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mip4_execute_fw_update(ts: *mut mip4_ts, fw: *const firmware) -> c_int {
    static int mip4_execute_fw_update(struct mip4_ts *ts, const struct firmware *fw)
    {
    const struct mip4_bin_tail *fw_info;
    u32 fw_start_offset;
    u32 fw_size;
    let mut retires: c_int = 3;
    int error;
    error = mip4_parse_firmware(ts, fw,
    &fw_start_offset, &fw_size, &fw_info);
    if (error)
    return error;
    if (input_device_enabled(ts.input)) {
    disable_irq(ts.client.irq);
    } else {
    error = mip4_power_on(ts);
    if (error)
    return error;
    }
// Update firmware
    do {
    error = mip4_flash_fw(ts, fw.data, fw_size, fw_start_offset);
    if (!error)
    break;
    } while (--retires);
    if (error)
    dev_err(&ts.client.dev,
    "Failed to flash firmware: %d\n", error);
// Enable IRQ
    if (input_device_enabled(ts.input))
    enable_irq(ts.client.irq);
    else
    mip4_power_off(ts);
    return error ? error : 0;
    }
    static ssize_t mip4_sysfs_fw_update(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct mip4_ts *ts = i2c_get_clientdata(client);
    int error;
    const struct firmware *fw __free(firmware) = core::ptr::null_mut();
    error = request_firmware(&fw, ts.fw_name, dev);
    if (error) {
    dev_err(&ts.client.dev,
    "Failed to retrieve firmware %s: %d\n",
    ts.fw_name, error);
    return error;
    }
//
// Take input mutex to prevent racing with itself and also with
// userspace opening and closing the device and also suspend/resume
// transitions.
//
    guard(mutex)(&ts.input.mutex);
    error = mip4_execute_fw_update(ts, fw);
    if (error) {
    dev_err(&ts.client.dev,
    "Firmware update failed: %d\n", error);
    return error;
    }
    return count;
    }
    static DEVICE_ATTR(update_fw, S_IWUSR, core::ptr::null_mut(), mip4_sysfs_fw_update);
    static ssize_t mip4_sysfs_read_fw_version(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct mip4_ts *ts = i2c_get_clientdata(client);
// Take lock to prevent racing with firmware update
    guard(mutex)(&ts.input.mutex);
    return sysfs_emit(buf, "%04X %04X %04X %04X\n",
    ts.fw_version.boot, ts.fw_version.core,
    ts.fw_version.app, ts.fw_version.param);
    }
    static DEVICE_ATTR(fw_version, S_IRUGO, mip4_sysfs_read_fw_version, core::ptr::null_mut());
    static ssize_t mip4_sysfs_read_hw_version(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct mip4_ts *ts = i2c_get_clientdata(client);
// Take lock to prevent racing with firmware update
    guard(mutex)(&ts.input.mutex);
//
// product_name shows the name or version of the hardware
// paired with current firmware in the chip.
//
    return sysfs_emit(buf, "%.*s\n",
    (int)sizeof(ts.product_name), ts.product_name);
    }
    static DEVICE_ATTR(hw_version, S_IRUGO, mip4_sysfs_read_hw_version, core::ptr::null_mut());
    static ssize_t mip4_sysfs_read_product_id(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct mip4_ts *ts = i2c_get_clientdata(client);
    guard(mutex)(&ts.input.mutex);
    return sysfs_emit(buf, "%04X\n", ts.product_id);
    }
    static DEVICE_ATTR(product_id, S_IRUGO, mip4_sysfs_read_product_id, core::ptr::null_mut());
    static ssize_t mip4_sysfs_read_ic_name(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct mip4_ts *ts = i2c_get_clientdata(client);
    guard(mutex)(&ts.input.mutex);
    return sysfs_emit(buf, "%.*s\n", (int)sizeof(ts.ic_name), ts.ic_name);
    }
    static DEVICE_ATTR(ic_name, S_IRUGO, mip4_sysfs_read_ic_name, core::ptr::null_mut());
    static struct attribute *mip4_attrs[] = {
    &dev_attr_fw_version.attr,
    &dev_attr_hw_version.attr,
    &dev_attr_product_id.attr,
    &dev_attr_ic_name.attr,
    &dev_attr_update_fw.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(mip4);
#[no_mangle]
unsafe extern "C" fn mip4_probe(client: *mut i2c_client) -> c_int {
    static int mip4_probe(struct i2c_client *client)
    {
    struct mip4_ts *ts;
    struct input_dev *input;
    int error;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_I2C)) {
    dev_err(&client.dev, "Not supported I2C adapter\n");
    return -ENXIO;
    }
    ts = devm_kzalloc(&client.dev, sizeof(*ts), GFP_KERNEL);
    if (!ts)
    return -ENOMEM;
    input = devm_input_allocate_device(&client.dev);
    if (!input)
    return -ENOMEM;
    ts.client = client;
    ts.input = input;
    snprintf(ts.phys, sizeof(ts.phys),
    "%s/input0", dev_name(&client.dev));
    ts.gpio_ce = devm_gpiod_get_optional(&client.dev,
    "ce", GPIOD_OUT_LOW);
    if (IS_ERR(ts.gpio_ce))
    return dev_err_probe(&client.dev, PTR_ERR(ts.gpio_ce), "Failed to get gpio\n");
    error = mip4_power_on(ts);
    if (error)
    return error;
    error = mip4_query_device(ts);
    mip4_power_off(ts);
    if (error)
    return error;
    input.name = "MELFAS MIP4 Touchscreen";
    input.phys = ts.phys;
    input.id.bustype = BUS_I2C;
    input.id.vendor = 0x13c5;
    input.id.product = ts.product_id;
    input.open = mip4_input_open;
    input.close = mip4_input_close;
    input_set_drvdata(input, ts);
    input.keycode = ts.key_code;
    input.keycodesize = sizeof(*ts.key_code);
    input.keycodemax = ts.key_num;
    input_set_abs_params(input, ABS_MT_TOOL_TYPE, 0, MT_TOOL_PALM, 0, 0);
    input_set_abs_params(input, ABS_MT_POSITION_X, 0, ts.max_x, 0, 0);
    input_set_abs_params(input, ABS_MT_POSITION_Y, 0, ts.max_y, 0, 0);
    input_set_abs_params(input, ABS_MT_PRESSURE,
    MIP4_PRESSURE_MIN, MIP4_PRESSURE_MAX, 0, 0);
    input_set_abs_params(input, ABS_MT_TOUCH_MAJOR,
    MIP4_TOUCH_MAJOR_MIN, MIP4_TOUCH_MAJOR_MAX, 0, 0);
    input_set_abs_params(input, ABS_MT_TOUCH_MINOR,
    MIP4_TOUCH_MINOR_MIN, MIP4_TOUCH_MINOR_MAX, 0, 0);
    input_abs_set_res(ts.input, ABS_MT_POSITION_X, ts.ppm_x);
    input_abs_set_res(ts.input, ABS_MT_POSITION_Y, ts.ppm_y);
    error = input_mt_init_slots(input, MIP4_MAX_FINGERS, INPUT_MT_DIRECT);
    if (error)
    return error;
    i2c_set_clientdata(client, ts);
    error = devm_request_threaded_irq(&client.dev, client.irq,
    core::ptr::null_mut(), mip4_interrupt,
    IRQF_ONESHOT | IRQF_NO_AUTOEN,
    MIP4_DEVICE_NAME, ts);
    if (error) {
    dev_err(&client.dev,
    "Failed to request interrupt %d: %d\n",
    client.irq, error);
    return error;
    }
    error = input_register_device(input);
    if (error) {
    dev_err(&client.dev,
    "Failed to register input device: %d\n", error);
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mip4_suspend(dev: *mut device) -> c_int {
    static int mip4_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct mip4_ts *ts = i2c_get_clientdata(client);
    struct input_dev *input = ts.input;
    guard(mutex)(&input.mutex);
    if (device_may_wakeup(dev))
    ts.wake_irq_enabled = enable_irq_wake(client.irq) == 0;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: input_device_enabled(input)) -> else {
    else if (input_device_enabled(input))
    mip4_disable(ts);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mip4_resume(dev: *mut device) -> c_int {
    static int mip4_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct mip4_ts *ts = i2c_get_clientdata(client);
    struct input_dev *input = ts.input;
    guard(mutex)(&input.mutex);
    if (ts.wake_irq_enabled)
    disable_irq_wake(client.irq);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: input_device_enabled(input)) -> else {
    else if (input_device_enabled(input))
    mip4_enable(ts);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(mip4_pm_ops, mip4_suspend, mip4_resume);

    static const struct of_device_id mip4_of_match[] = {
    { .compatible = "melfas,mip4_ts", },
    { },
    };
    MODULE_DEVICE_TABLE(of, mip4_of_match);

    static const struct acpi_device_id mip4_acpi_match[] = {
    { "MLFS0000", 0},
    { },
    };
    MODULE_DEVICE_TABLE(acpi, mip4_acpi_match);

    static const struct i2c_device_id mip4_i2c_ids[] = {
    { .name = MIP4_DEVICE_NAME },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, mip4_i2c_ids);
    static struct i2c_driver mip4_driver = {
    .id_table = mip4_i2c_ids,
    .probe = mip4_probe,
    .driver = {
    .name = MIP4_DEVICE_NAME,
    .dev_groups = mip4_groups,
    .of_match_table = of_match_ptr(mip4_of_match),
    .acpi_match_table = ACPI_PTR(mip4_acpi_match),
    .pm = pm_sleep_ptr(&mip4_pm_ops),
    },
    };
    module_i2c_driver(mip4_driver);
    MODULE_DESCRIPTION("MELFAS MIP4 Touchscreen");
    MODULE_AUTHOR("Sangwon Jee <jeesw@melfas.com>");
    MODULE_LICENSE("GPL");
