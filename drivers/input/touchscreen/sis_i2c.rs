//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/sis_i2c.c
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
// Touch Screen driver for SiS 9200 family I2C Touch panels
//
// Copyright (C) 2015 SiS, Inc.
// Copyright (C) 2016 Nextfour Group
//

//
// The I2C packet format:
// le16		byte count
// u8		Report ID
// <contact data - variable length>
// u8		Number of contacts
// le16		Scan Time (optional)
// le16		CRC
//
// One touch point information consists of 6+ bytes, the order is:
// u8		contact state
// u8		finger id
// le16		x axis
// le16		y axis
// u8		contact width (optional)
// u8		contact height (optional)
// u8		pressure (optional)
//
// Maximum amount of data transmitted in one shot is 64 bytes, if controller
// needs to report more contacts than fit in one packet it will send true
// number of contacts in first packet and 0 as number of contacts in second
// packet.
//
pub const SIS_MAX_PACKET_SIZE: c_int = 64;
pub const SIS_PKT_LEN_OFFSET: c_int = 0;

pub const SIS_SCAN_TIME_LEN: c_int = 2;
// Supported report types
pub const SIS_ALL_IN_ONE_PACKAGE: c_uint = 0x10;

// Contact properties within report

// Contact size
pub const SIS_BASE_LEN_PER_CONTACT: c_int = 6;
pub const SIS_AREA_LEN_PER_CONTACT: c_int = 2;
pub const SIS_PRESSURE_LEN_PER_CONTACT: c_int = 1;
// Offsets within contact data
pub const SIS_CONTACT_STATUS_OFFSET: c_int = 0;

pub const SIS_CONTACT_X_OFFSET: c_int = 2;
pub const SIS_CONTACT_Y_OFFSET: c_int = 4;
pub const SIS_CONTACT_WIDTH_OFFSET: c_int = 6;
pub const SIS_CONTACT_HEIGHT_OFFSET: c_int = 7;

// Individual contact state
pub const SIS_STATUS_UP: c_uint = 0x0;
pub const SIS_STATUS_DOWN: c_uint = 0x3;
// Touchscreen parameters
pub const SIS_MAX_FINGERS: c_int = 10;
pub const SIS_MAX_X: c_int = 4095;
pub const SIS_MAX_Y: c_int = 4095;
pub const SIS_MAX_PRESSURE: c_int = 255;
// Resolution diagonal
pub const SIS_AREA_LENGTH_LONGER: c_int = 5792;
// ((SIS_MAX_X^2) + (SIS_MAX_Y^2))^0.5
pub const SIS_AREA_LENGTH_SHORT: c_int = 5792;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sis_ts_data {
    pub client: *mut i2c_client,
    pub input: *mut input_dev,
    pub attn_gpio: *mut gpio_desc,
    pub reset_gpio: *mut gpio_desc,
    pub packet: [u8; SIS_MAX_PACKET_SIZE],
}

    static int sis_read_packet(struct i2c_client *client, u8 *buf,
    unsigned int *num_contacts,
    unsigned int *contact_size)
    {
    int count_idx;
    int ret;
    u16 len;
    u16 crc, pkg_crc;
    u8 report_id;
    ret = i2c_master_recv(client, buf, SIS_MAX_PACKET_SIZE);
    if (ret <= 0)
    return -EIO;
    len = get_unaligned_le16(&buf[SIS_PKT_LEN_OFFSET]);
    if (len > SIS_MAX_PACKET_SIZE) {
    dev_err(&client.dev,
    "%s: invalid packet length (%d vs %d)\n",
    __func__, len, SIS_MAX_PACKET_SIZE);
    return -E2BIG;
    }
    if (len < 10)
    return -EINVAL;
    report_id = buf[SIS_PKT_REPORT_OFFSET];
    count_idx  = len - 1;
// contact_size = SIS_BASE_LEN_PER_CONTACT;
    if (report_id != SIS_ALL_IN_ONE_PACKAGE) {
    if (SIS_PKT_IS_TOUCH(report_id)) {
//
// Calculate CRC ignoring packet length
// in the beginning and CRC transmitted
// at the end of the packet.
//
    crc = crc_itu_t(0, buf + 2, len - 2 - 2);
    pkg_crc = get_unaligned_le16(&buf[len - 2]);
    if (crc != pkg_crc) {
    dev_err(&client.dev,
    "%s: CRC Error (%d vs %d)\n",
    __func__, crc, pkg_crc);
    return -EINVAL;
    }
    count_idx -= 2;
    } else if (!SIS_PKT_IS_HIDI2C(report_id)) {
    dev_err(&client.dev,
    "%s: invalid packet ID %#02x\n",
    __func__, report_id);
    return -EINVAL;
    }
    if (SIS_PKT_HAS_SCANTIME(report_id))
    count_idx -= SIS_SCAN_TIME_LEN;
    if (SIS_PKT_HAS_AREA(report_id))
// contact_size += SIS_AREA_LEN_PER_CONTACT;
    if (SIS_PKT_HAS_PRESSURE(report_id))
// contact_size += SIS_PRESSURE_LEN_PER_CONTACT;
    }
// num_contacts = buf[count_idx];
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sis_ts_report_contact(ts: *mut sis_ts_data, data: *const u8, id: u8) -> c_int {
    static int sis_ts_report_contact(struct sis_ts_data *ts, const u8 *data, u8 id)
    {
    struct input_dev *input = ts.input;
    int slot;
    let mut status: u8 = data[SIS_CONTACT_STATUS_OFFSET];
    u8 pressure;
    u8 height, width;
    u16 x, y;
    if (status != SIS_STATUS_DOWN && status != SIS_STATUS_UP) {
    dev_err(&ts.client.dev, "Unexpected touch status: %#02x\n",
    data[SIS_CONTACT_STATUS_OFFSET]);
    return -EINVAL;
    }
    slot = input_mt_get_slot_by_key(input, data[SIS_CONTACT_ID_OFFSET]);
    if (slot < 0)
    return -ENOENT;
    input_mt_slot(input, slot);
    input_mt_report_slot_state(input, MT_TOOL_FINGER,
    status == SIS_STATUS_DOWN);
    if (status == SIS_STATUS_DOWN) {
    pressure = height = width = 1;
    if (id != SIS_ALL_IN_ONE_PACKAGE) {
    if (SIS_PKT_HAS_AREA(id)) {
    width = data[SIS_CONTACT_WIDTH_OFFSET];
    height = data[SIS_CONTACT_HEIGHT_OFFSET];
    }
    if (SIS_PKT_HAS_PRESSURE(id))
    pressure =
    data[SIS_CONTACT_PRESSURE_OFFSET(id)];
    }
    x = get_unaligned_le16(&data[SIS_CONTACT_X_OFFSET]);
    y = get_unaligned_le16(&data[SIS_CONTACT_Y_OFFSET]);
    input_report_abs(input, ABS_MT_TOUCH_MAJOR,
    width * SIS_AREA_UNIT);
    input_report_abs(input, ABS_MT_TOUCH_MINOR,
    height * SIS_AREA_UNIT);
    input_report_abs(input, ABS_MT_PRESSURE, pressure);
    input_report_abs(input, ABS_MT_POSITION_X, x);
    input_report_abs(input, ABS_MT_POSITION_Y, y);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sis_ts_handle_packet(ts: *mut sis_ts_data) {
    static void sis_ts_handle_packet(struct sis_ts_data *ts)
    {
    const u8 *contact;
    let mut num_to_report: c_uint = 0;
    unsigned int num_contacts;
    unsigned int num_reported;
    unsigned int contact_size;
    int error;
    u8 report_id;
    do {
    error = sis_read_packet(ts.client, ts.packet,
    &num_contacts, &contact_size);
    if (error)
    break;
    if (num_to_report == 0) {
    num_to_report = num_contacts;
    } else if (num_contacts != 0) {
    dev_err(&ts.client.dev,
    "%s: nonzero (%d) point count in tail packet\n",
    __func__, num_contacts);
    break;
    }
    report_id = ts.packet[SIS_PKT_REPORT_OFFSET];
    contact = &ts.packet[SIS_PKT_CONTACT_OFFSET];
    num_reported = 0;
    while (num_to_report > 0) {
    error = sis_ts_report_contact(ts, contact, report_id);
    if (error)
    break;
    contact += contact_size;
    num_to_report--;
    num_reported++;
    if (report_id != SIS_ALL_IN_ONE_PACKAGE &&
    num_reported >= 5) {
//
// The remainder of contacts is sent
// in the 2nd packet.
//
    break;
    }
    }
    } while (num_to_report > 0);
    input_mt_sync_frame(ts.input);
    input_sync(ts.input);
    }
#[no_mangle]
unsafe extern "C" fn sis_ts_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t sis_ts_irq_handler(int irq, void *dev_id)
    {
    struct sis_ts_data *ts = dev_id;
    do {
    sis_ts_handle_packet(ts);
    } while (ts.attn_gpio && gpiod_get_value_cansleep(ts.attn_gpio));
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn sis_ts_reset(ts: *mut sis_ts_data) {
    static void sis_ts_reset(struct sis_ts_data *ts)
    {
    if (ts.reset_gpio) {
// Get out of reset
    usleep_range(1000, 2000);
    gpiod_set_value(ts.reset_gpio, 1);
    usleep_range(1000, 2000);
    gpiod_set_value(ts.reset_gpio, 0);
    msleep(100);
    }
    }
#[no_mangle]
unsafe extern "C" fn sis_ts_probe(client: *mut i2c_client) -> c_int {
    static int sis_ts_probe(struct i2c_client *client)
    {
    struct sis_ts_data *ts;
    struct input_dev *input;
    int error;
    ts = devm_kzalloc(&client.dev, sizeof(*ts), GFP_KERNEL);
    if (!ts)
    return -ENOMEM;
    ts.client = client;
    ts.attn_gpio = devm_gpiod_get_optional(&client.dev,
    "attn", GPIOD_IN);
    if (IS_ERR(ts.attn_gpio))
    return dev_err_probe(&client.dev, PTR_ERR(ts.attn_gpio),
    "Failed to get attention GPIO\n");
    ts.reset_gpio = devm_gpiod_get_optional(&client.dev,
    "reset", GPIOD_OUT_LOW);
    if (IS_ERR(ts.reset_gpio))
    return dev_err_probe(&client.dev, PTR_ERR(ts.reset_gpio),
    "Failed to get reset GPIO\n");
    sis_ts_reset(ts);
    ts.input = input = devm_input_allocate_device(&client.dev);
    if (!input) {
    dev_err(&client.dev, "Failed to allocate input device\n");
    return -ENOMEM;
    }
    input.name = "SiS Touchscreen";
    input.id.bustype = BUS_I2C;
    input_set_abs_params(input, ABS_MT_POSITION_X, 0, SIS_MAX_X, 0, 0);
    input_set_abs_params(input, ABS_MT_POSITION_Y, 0, SIS_MAX_Y, 0, 0);
    input_set_abs_params(input, ABS_MT_PRESSURE, 0, SIS_MAX_PRESSURE, 0, 0);
    input_set_abs_params(input, ABS_MT_TOUCH_MAJOR,
    0, SIS_AREA_LENGTH_LONGER, 0, 0);
    input_set_abs_params(input, ABS_MT_TOUCH_MINOR,
    0, SIS_AREA_LENGTH_SHORT, 0, 0);
    error = input_mt_init_slots(input, SIS_MAX_FINGERS, INPUT_MT_DIRECT);
    if (error) {
    dev_err(&client.dev,
    "Failed to initialize MT slots: %d\n", error);
    return error;
    }
    error = devm_request_threaded_irq(&client.dev, client.irq,
    core::ptr::null_mut(), sis_ts_irq_handler,
    IRQF_ONESHOT,
    client.name, ts);
    if (error) {
    dev_err(&client.dev, "Failed to request IRQ: %d\n", error);
    return error;
    }
    error = input_register_device(ts.input);
    if (error) {
    dev_err(&client.dev,
    "Failed to register input device: %d\n", error);
    return error;
    }
    return 0;
    }

    static const struct of_device_id sis_ts_dt_ids[] = {
    { .compatible = "sis,9200-ts" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, sis_ts_dt_ids);

    static const struct i2c_device_id sis_ts_id[] = {
    { .name = SIS_I2C_NAME },
    { .name = "9200-ts" },
    { /* sentinel */  }
    };
    MODULE_DEVICE_TABLE(i2c, sis_ts_id);
    static struct i2c_driver sis_ts_driver = {
    .driver = {
    .name	= SIS_I2C_NAME,
    .of_match_table = of_match_ptr(sis_ts_dt_ids),
    },
    .probe		= sis_ts_probe,
    .id_table	= sis_ts_id,
    };
    module_i2c_driver(sis_ts_driver);
    MODULE_DESCRIPTION("SiS 9200 Family Touchscreen Driver");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Mika Penttilä <mika.penttila@nextfour.com>");
