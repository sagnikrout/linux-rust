//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/surface3_spi.c
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
// Driver for Ntrig/Microsoft Touchscreens over SPI
//
// Copyright (c) 2016 Red Hat Inc.
//

pub const SURFACE3_PACKET_SIZE: c_int = 264;
pub const SURFACE3_REPORT_TOUCH: c_uint = 0xd2;
pub const SURFACE3_REPORT_PEN: c_uint = 0x16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct surface3_ts_data {
    pub spi: *mut spi_device,
    pub gpiod_rst: [*mut gpio_desc; 2],
    pub input_dev: *mut input_dev,
    pub pen_input_dev: *mut input_dev,
    pub pen_tool: c_int,
    pub ____cacheline_aligned: u8 rd_buf[SURFACE3_PACKET_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct surface3_ts_data_finger {
    pub status: u8,
    pub tracking_id: __le16,
    pub x: __le16,
    pub cx: __le16,
    pub y: __le16,
    pub cy: __le16,
    pub width: __le16,
    pub height: __le16,
    pub padding: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct surface3_ts_data_pen {
    pub status: u8,
    pub x: __le16,
    pub y: __le16,
    pub pressure: __le16,
    pub padding: u8,
    pub __packed: },
#[no_mangle]
unsafe extern "C" fn surface3_spi_read(ts_data: *mut surface3_ts_data) -> c_int {
    static int surface3_spi_read(struct surface3_ts_data *ts_data)
    {
    pub ts_data->spi: *mut *mut spi_device spi =,
    pub sizeof(ts_data->rd_buf)): memset(ts_data->rd_buf, 0,,
    pub sizeof(ts_data->rd_buf)): return spi_read(spi, ts_data->rd_buf,,
    }
    static void surface3_spi_report_touch(struct surface3_ts_data *ts_data,
    struct surface3_ts_data_finger *finger)
    {
    pub 0x01: int st = finger->status &,
    pub slot: c_int,
    slot = input_mt_get_slot_by_key(ts_data.input_dev,
    if (slot < 0)
    pub slot): input_mt_slot(ts_data->input_dev,,
    pub st): input_mt_report_slot_state(ts_data->input_dev, MT_TOOL_FINGER,,
    if (st) {
    input_report_abs(ts_data.input_dev,
    ABS_MT_POSITION_X,
    input_report_abs(ts_data.input_dev,
    ABS_MT_POSITION_Y,
    input_report_abs(ts_data.input_dev,
    ABS_MT_WIDTH_MAJOR,
    input_report_abs(ts_data.input_dev,
    ABS_MT_WIDTH_MINOR,
    }
    }
#[no_mangle]
unsafe extern "C" fn surface3_spi_process_touch(ts_data: *mut surface3_ts_data, data: *mut u8) {
    static void surface3_spi_process_touch(struct surface3_ts_data *ts_data, u8 *data)
    {
    pub i: c_uint,
    pub {: for (i = 0; i < 13; i++),
    pub finger: *mut surface3_ts_data_finger,
    finger = (struct surface3_ts_data_finger *)&data[17 +
    pub surface3_ts_data_finger)]: *mut *mut i  sizeof(struct,
//
// When bit 5 of status is 1, it marks the end of the report:
// - touch present: 0xe7
// - touch released: 0xe4
// - nothing valuable: 0xff
//
    if (finger.status & 0x10)
    pub finger): surface3_spi_report_touch(ts_data,,
    }
    }
    static void surface3_spi_report_pen(struct surface3_ts_data *ts_data,
    struct surface3_ts_data_pen *pen)
    {
    pub ts_data->pen_input_dev: *mut *mut input_dev dev =,
    pub pen->status: int st =,
    pub 0x01: int prox = st &,
    pub 0x18: int rubber = st &,
    pub BTN_TOOL_PEN: int tool = (prox && rubber) ? BTN_TOOL_RUBBER :,
// fake proximity out to switch tools
    if (ts_data.pen_tool != tool) {
    pub 0): input_report_key(dev, ts_data->pen_tool,,
    pub tool: ts_data->pen_tool =,
    }
    pub 0x12): input_report_key(dev, BTN_TOUCH, st &,
    pub prox): input_report_key(dev, ts_data->pen_tool,,
    if (st) {
    input_report_key(dev,
    BTN_STYLUS,
    pub 0x04): st &,
    input_report_abs(dev,
    ABS_X,
    input_report_abs(dev,
    ABS_Y,
    input_report_abs(dev,
    ABS_PRESSURE,
    }
    }
#[no_mangle]
unsafe extern "C" fn surface3_spi_process_pen(ts_data: *mut surface3_ts_data, data: *mut u8) {
    static void surface3_spi_process_pen(struct surface3_ts_data *ts_data, u8 *data)
    {
    pub pen: *mut surface3_ts_data_pen,
    pub )&data[15]: *mut pen = (struct surface3_ts_data_pen,
    pub pen): surface3_spi_report_pen(ts_data,,
    }
#[no_mangle]
unsafe extern "C" fn surface3_spi_process(ts_data: *mut surface3_ts_data) {
    static void surface3_spi_process(struct surface3_ts_data *ts_data)
    {
    static const char header[] = {
    0xff, 0xff, 0xff, 0xff, 0xa5, 0x5a, 0xe7, 0x7e, 0x01
}

    u8 *data = ts_data.rd_buf;
    if (memcmp(header, data, sizeof(header)))
    dev_err(&ts_data.spi.dev,
    "%s header error: %*ph, ignoring...\n",
    __func__, (int)sizeof(header), data);
    switch (data[9]) {
    case SURFACE3_REPORT_TOUCH:
    surface3_spi_process_touch(ts_data, data);
    break;
    case SURFACE3_REPORT_PEN:
    surface3_spi_process_pen(ts_data, data);
    break;
    default:
    dev_err(&ts_data.spi.dev,
    "%s unknown packet type: %x, ignoring...\n",
    __func__, data[9]);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn surface3_spi_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t surface3_spi_irq_handler(int irq, void *dev_id)
    {
    struct surface3_ts_data *data = dev_id;
    if (surface3_spi_read(data))
    return IRQ_HANDLED;
    dev_dbg(&data.spi.dev, "%s received . %*ph\n",
    __func__, SURFACE3_PACKET_SIZE, data.rd_buf);
    surface3_spi_process(data);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn surface3_spi_power(data: *mut surface3_ts_data, on: bool) {
    static void surface3_spi_power(struct surface3_ts_data *data, bool on)
    {
    gpiod_set_value(data.gpiod_rst[0], on);
    gpiod_set_value(data.gpiod_rst[1], on);
// let the device settle a little
    msleep(20);
    }
//
// surface3_spi_get_gpio_config - Get GPIO config from ACPI/DT
//
// @data: surface3_spi_ts_data pointer
//
#[no_mangle]
unsafe extern "C" fn surface3_spi_get_gpio_config(data: *mut surface3_ts_data) -> c_int {
    static int surface3_spi_get_gpio_config(struct surface3_ts_data *data)
    {
    struct device *dev;
    struct gpio_desc *gpiod;
    int i;
    dev = &data.spi.dev;
// Get the reset lines GPIO pin number
    for (i = 0; i < 2; i++) {
    gpiod = devm_gpiod_get_index(dev, core::ptr::null_mut(), i, GPIOD_OUT_LOW);
    if (IS_ERR(gpiod))
    return dev_err_probe(dev, PTR_ERR(gpiod),
    "Failed to get power GPIO %d\n", i);
    data.gpiod_rst[i] = gpiod;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn surface3_spi_create_touch_input(data: *mut surface3_ts_data) -> c_int {
    static int surface3_spi_create_touch_input(struct surface3_ts_data *data)
    {
    struct input_dev *input;
    int error;
    input = devm_input_allocate_device(&data.spi.dev);
    if (!input)
    return -ENOMEM;
    data.input_dev = input;
    input_set_abs_params(input, ABS_MT_POSITION_X, 0, 9600, 0, 0);
    input_abs_set_res(input, ABS_MT_POSITION_X, 40);
    input_set_abs_params(input, ABS_MT_POSITION_Y, 0, 7200, 0, 0);
    input_abs_set_res(input, ABS_MT_POSITION_Y, 48);
    input_set_abs_params(input, ABS_MT_WIDTH_MAJOR, 0, 1024, 0, 0);
    input_set_abs_params(input, ABS_MT_WIDTH_MINOR, 0, 1024, 0, 0);
    input_mt_init_slots(input, 10, INPUT_MT_DIRECT);
    input.name = "Surface3 SPI Capacitive TouchScreen";
    input.phys = "input/ts";
    input.id.bustype = BUS_SPI;
    input.id.vendor = 0x045e;	/* Microsoft */
    input.id.product = 0x0001;
    input.id.version = 0x0000;
    error = input_register_device(input);
    if (error) {
    dev_err(&data.spi.dev,
    "Failed to register input device: %d", error);
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn surface3_spi_create_pen_input(data: *mut surface3_ts_data) -> c_int {
    static int surface3_spi_create_pen_input(struct surface3_ts_data *data)
    {
    struct input_dev *input;
    int error;
    input = devm_input_allocate_device(&data.spi.dev);
    if (!input)
    return -ENOMEM;
    data.pen_input_dev = input;
    data.pen_tool = BTN_TOOL_PEN;
    __set_bit(INPUT_PROP_DIRECT, input.propbit);
    __set_bit(INPUT_PROP_POINTER, input.propbit);
    input_set_abs_params(input, ABS_X, 0, 9600, 0, 0);
    input_abs_set_res(input, ABS_X, 40);
    input_set_abs_params(input, ABS_Y, 0, 7200, 0, 0);
    input_abs_set_res(input, ABS_Y, 48);
    input_set_abs_params(input, ABS_PRESSURE, 0, 1024, 0, 0);
    input_set_capability(input, EV_KEY, BTN_TOUCH);
    input_set_capability(input, EV_KEY, BTN_STYLUS);
    input_set_capability(input, EV_KEY, BTN_TOOL_PEN);
    input_set_capability(input, EV_KEY, BTN_TOOL_RUBBER);
    input.name = "Surface3 SPI Pen Input";
    input.phys = "input/ts";
    input.id.bustype = BUS_SPI;
    input.id.vendor = 0x045e;     /* Microsoft */
    input.id.product = 0x0002;
    input.id.version = 0x0000;
    error = input_register_device(input);
    if (error) {
    dev_err(&data.spi.dev,
    "Failed to register input device: %d", error);
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn surface3_spi_probe(spi: *mut spi_device) -> c_int {
    static int surface3_spi_probe(struct spi_device *spi)
    {
    struct surface3_ts_data *data;
    int error;
// Set up SPI
    spi.bits_per_word = 8;
    spi.mode = SPI_MODE_0;
    error = spi_setup(spi);
    if (error)
    return error;
    data = devm_kzalloc(&spi.dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.spi = spi;
    spi_set_drvdata(spi, data);
    error = surface3_spi_get_gpio_config(data);
    if (error)
    return error;
    surface3_spi_power(data, true);
    surface3_spi_power(data, false);
    surface3_spi_power(data, true);
    error = surface3_spi_create_touch_input(data);
    if (error)
    return error;
    error = surface3_spi_create_pen_input(data);
    if (error)
    return error;
    error = devm_request_threaded_irq(&spi.dev, spi.irq,
    core::ptr::null_mut(), surface3_spi_irq_handler,
    IRQF_ONESHOT,
    "Surface3-irq", data);
    if (error)
    return error;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn surface3_spi_suspend(dev: *mut device) -> c_int {
    static int surface3_spi_suspend(struct device *dev)
    {
    struct spi_device *spi = to_spi_device(dev);
    struct surface3_ts_data *data = spi_get_drvdata(spi);
    disable_irq(data.spi.irq);
    surface3_spi_power(data, false);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn surface3_spi_resume(dev: *mut device) -> c_int {
    static int surface3_spi_resume(struct device *dev)
    {
    struct spi_device *spi = to_spi_device(dev);
    struct surface3_ts_data *data = spi_get_drvdata(spi);
    surface3_spi_power(data, true);
    enable_irq(data.spi.irq);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(surface3_spi_pm_ops,
    surface3_spi_suspend,
    surface3_spi_resume);

    static const struct acpi_device_id surface3_spi_acpi_match[] = {
    { "MSHW0037", 0 },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, surface3_spi_acpi_match);

    static struct spi_driver surface3_spi_driver = {
    .driver = {
    .name	= "Surface3-spi",
    .acpi_match_table = ACPI_PTR(surface3_spi_acpi_match),
    .pm = pm_sleep_ptr(&surface3_spi_pm_ops),
    },
    .probe = surface3_spi_probe,
    };
    module_spi_driver(surface3_spi_driver);
    MODULE_AUTHOR("Benjamin Tissoires <benjamin.tissoires@gmail.com>");
    MODULE_DESCRIPTION("Surface 3 SPI touchscreen driver");
    MODULE_LICENSE("GPL v2");
