//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/exc3000.c
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
// Driver for I2C connected EETI EXC3000 multiple touch controller
//
// Copyright (C) 2017 Ahmet Inan <inan@distec.de>
//
// minimal implementation based on egalax_ts.c and egalax_i2c.c
//

pub const EXC3000_NUM_SLOTS: c_int = 10;
pub const EXC3000_SLOTS_PER_FRAME: c_int = 5;
pub const EXC3000_LEN_FRAME: c_int = 66;
pub const EXC3000_LEN_VENDOR_REQUEST: c_int = 68;
pub const EXC3000_LEN_POINT: c_int = 10;
pub const EXC3000_LEN_MODEL_NAME: c_int = 16;
pub const EXC3000_LEN_FW_VERSION: c_int = 16;
pub const EXC3000_VENDOR_EVENT: c_uint = 0x03;
pub const EXC3000_MT1_EVENT: c_uint = 0x06;
pub const EXC3000_MT2_EVENT: c_uint = 0x18;
pub const EXC3000_TIMEOUT_MS: c_int = 100;
pub const EXC3000_RESET_MS: c_int = 10;
pub const EXC3000_READY_MS: c_int = 100;
    static const struct i2c_device_id exc3000_id[];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eeti_dev_info {
    pub name: *const c_char,
    pub max_xy: c_int,
}

    enum eeti_dev_id {
    EETI_EXC3000,
    EETI_EXC80H60,
    EETI_EXC80H84,
    EETI_EXC81W32,
    };
    static struct eeti_dev_info exc3000_info[] = {
    [EETI_EXC3000] = {
    .name = "EETI EXC3000 Touch Screen",
    .max_xy = SZ_4K - 1,
    },
    [EETI_EXC80H60] = {
    .name = "EETI EXC80H60 Touch Screen",
    .max_xy = SZ_16K - 1,
    },
    [EETI_EXC80H84] = {
    .name = "EETI EXC80H84 Touch Screen",
    .max_xy = SZ_16K - 1,
    },
    [EETI_EXC81W32] = {
    .name = "EETI EXC81W32 Touch Screen",
    .max_xy = SZ_16K - 1,
    },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exc3000_data {
    pub client: *mut i2c_client,
    pub info: *const eeti_dev_info,
    pub input: *mut input_dev,
    pub prop: touchscreen_properties,
    pub reset: *mut gpio_desc,
    pub timer: timer_list,
    pub EXC3000_LEN_FRAME]: *mut *mut u8 buf[2,
    pub wait_event: completion,
    pub query_lock: mutex,
}

    static void exc3000_report_slots(struct input_dev *input,
    struct touchscreen_properties *prop,
    const u8 *buf, int num)
    {
    for (; num--; buf += EXC3000_LEN_POINT) {
    if (buf[0] & BIT(0)) {
    input_mt_slot(input, buf[1]);
    input_mt_report_slot_state(input, MT_TOOL_FINGER, true);
    touchscreen_report_pos(input, prop,
    get_unaligned_le16(buf + 2),
    get_unaligned_le16(buf + 4),
    true);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn exc3000_timer(t: *mut timer_list) {
    static void exc3000_timer(struct timer_list *t)
    {
    struct exc3000_data *data = timer_container_of(data, t, timer);
    input_mt_sync_frame(data.input);
    input_sync(data.input);
    }
#[no_mangle]
pub unsafe extern "C" fn exc3000_schedule_timer(data: *mut exc3000_data) {
    static inline void exc3000_schedule_timer(struct exc3000_data *data)
    {
    mod_timer(&data.timer, jiffies + msecs_to_jiffies(EXC3000_TIMEOUT_MS));
    }
#[no_mangle]
unsafe extern "C" fn exc3000_shutdown_timer(timer: *mut c_void) {
    static void exc3000_shutdown_timer(void *timer)
    {
    timer_shutdown_sync(timer);
    }
#[no_mangle]
unsafe extern "C" fn exc3000_read_frame(data: *mut exc3000_data, buf: *mut u8) -> c_int {
    static int exc3000_read_frame(struct exc3000_data *data, u8 *buf)
    {
    struct i2c_client *client = data.client;
    int ret;
    ret = i2c_master_send(client, "'", 2);
    if (ret < 0)
    return ret;
    if (ret != 2)
    return -EIO;
    ret = i2c_master_recv(client, buf, EXC3000_LEN_FRAME);
    if (ret < 0)
    return ret;
    if (ret != EXC3000_LEN_FRAME)
    return -EIO;
    if (get_unaligned_le16(buf) != EXC3000_LEN_FRAME)
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exc3000_handle_mt_event(data: *mut exc3000_data) -> c_int {
    static int exc3000_handle_mt_event(struct exc3000_data *data)
    {
    struct input_dev *input = data.input;
    int ret, total_slots;
    u8 *buf = data.buf;
    total_slots = buf[3];
    if (!total_slots || total_slots > EXC3000_NUM_SLOTS) {
    ret = -EINVAL;
    goto out_fail;
    }
    if (total_slots > EXC3000_SLOTS_PER_FRAME) {
// Read 2nd frame to get the rest of the contacts.
    ret = exc3000_read_frame(data, buf + EXC3000_LEN_FRAME);
    if (ret)
    goto out_fail;
// 2nd chunk must have number of contacts set to 0.
    if (buf[EXC3000_LEN_FRAME + 3] != 0) {
    ret = -EINVAL;
    goto out_fail;
    }
    }
//
// We read full state successfully, no contacts will be "stuck".
//
    timer_delete_sync(&data.timer);
    while (total_slots > 0) {
    let mut slots: c_int = min(total_slots, EXC3000_SLOTS_PER_FRAME);
    exc3000_report_slots(input, &data.prop, buf + 4, slots);
    total_slots -= slots;
    buf += EXC3000_LEN_FRAME;
    }
    input_mt_sync_frame(input);
    input_sync(input);
    return 0;
    out_fail:
// Schedule a timer to release "stuck" contacts
    exc3000_schedule_timer(data);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn exc3000_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t exc3000_interrupt(int irq, void *dev_id)
    {
    struct exc3000_data *data = dev_id;
    u8 *buf = data.buf;
    int ret;
    ret = exc3000_read_frame(data, buf);
    if (ret) {
// Schedule a timer to release "stuck" contacts
    exc3000_schedule_timer(data);
    goto out;
    }
    switch (buf[2]) {
    case EXC3000_VENDOR_EVENT:
    complete(&data.wait_event);
    break;
    case EXC3000_MT1_EVENT:
    case EXC3000_MT2_EVENT:
    exc3000_handle_mt_event(data);
    break;
    default:
    break;
    }
    out:
    return IRQ_HANDLED;
    }
    static int exc3000_vendor_data_request(struct exc3000_data *data, u8 *request,
    u8 request_len, u8 *response, int timeout)
    {
    u8 buf[EXC3000_LEN_VENDOR_REQUEST] = { 0x67, 0x00, 0x42, 0x00, 0x03 };
    int ret;
    unsigned long time_left;
    guard(mutex)(&data.query_lock);
    reinit_completion(&data.wait_event);
    buf[5] = request_len;
    memcpy(&buf[6], request, request_len);
    ret = i2c_master_send(data.client, buf, EXC3000_LEN_VENDOR_REQUEST);
    if (ret < 0)
    return ret;
    time_left = wait_for_completion_timeout(&data.wait_event,
    timeout * HZ);
    if (time_left == 0)
    return -ETIMEDOUT;
    if (data.buf[3] >= EXC3000_LEN_FRAME)
    return -ENOSPC;
    memcpy(response, &data.buf[4], data.buf[3]);
    return data.buf[3];
    }
    static ssize_t fw_version_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct exc3000_data *data = i2c_get_clientdata(client);
    u8 response[EXC3000_LEN_FRAME];
    int ret;
// query bootloader info
    ret = exc3000_vendor_data_request(data,
    (u8[]){0x39, 0x02}, 2, response, 1);
    if (ret < 0)
    return ret;
//
// If the bootloader version is non-zero then the device is in
// bootloader mode and won't answer a query for the application FW
// version, so we just use the bootloader version info.
//
    if (response[2] || response[3])
    return sprintf(buf, "%d.%d\n", response[2], response[3]);
    ret = exc3000_vendor_data_request(data, (u8[]){'D'}, 1, response, 1);
    if (ret < 0)
    return ret;
    return sprintf(buf, "%s\n", &response[1]);
    }
    static DEVICE_ATTR_RO(fw_version);
    static ssize_t model_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct exc3000_data *data = i2c_get_clientdata(client);
    u8 response[EXC3000_LEN_FRAME];
    int ret;
    ret = exc3000_vendor_data_request(data, (u8[]){'E'}, 1, response, 1);
    if (ret < 0)
    return ret;
    return sprintf(buf, "%s\n", &response[1]);
    }
    static DEVICE_ATTR_RO(model);
    static ssize_t type_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct exc3000_data *data = i2c_get_clientdata(client);
    u8 response[EXC3000_LEN_FRAME];
    int ret;
    ret = exc3000_vendor_data_request(data, (u8[]){'F'}, 1, response, 1);
    if (ret < 0)
    return ret;
    return sprintf(buf, "%s\n", &response[1]);
    }
    static DEVICE_ATTR_RO(type);
    static struct attribute *exc3000_attrs[] = {
    &dev_attr_fw_version.attr,
    &dev_attr_model.attr,
    &dev_attr_type.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(exc3000);
#[no_mangle]
unsafe extern "C" fn exc3000_probe(client: *mut i2c_client) -> c_int {
    static int exc3000_probe(struct i2c_client *client)
    {
    struct exc3000_data *data;
    struct input_dev *input;
    int error, max_xy, retry;
    data = devm_kzalloc(&client.dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.client = client;
    data.info = device_get_match_data(&client.dev);
    if (!data.info) {
    enum eeti_dev_id eeti_dev_id =
    i2c_match_id(exc3000_id, client).driver_data;
    data.info = &exc3000_info[eeti_dev_id];
    }
    timer_setup(&data.timer, exc3000_timer, 0);
    init_completion(&data.wait_event);
    mutex_init(&data.query_lock);
    data.reset = devm_gpiod_get_optional(&client.dev, "reset",
    GPIOD_OUT_HIGH);
    if (IS_ERR(data.reset))
    return PTR_ERR(data.reset);
// For proper reset sequence, enable power while reset asserted
    error = devm_regulator_get_enable(&client.dev, "vdd");
    if (error && error != -ENODEV)
    return dev_err_probe(&client.dev, error,
    "failed to request vdd regulator\n");
    if (data.reset) {
    msleep(EXC3000_RESET_MS);
    gpiod_set_value_cansleep(data.reset, 0);
    msleep(EXC3000_READY_MS);
    }
    input = devm_input_allocate_device(&client.dev);
    if (!input)
    return -ENOMEM;
    data.input = input;
    input_set_drvdata(input, data);
    input.name = data.info.name;
    input.id.bustype = BUS_I2C;
    max_xy = data.info.max_xy;
    input_set_abs_params(input, ABS_MT_POSITION_X, 0, max_xy, 0, 0);
    input_set_abs_params(input, ABS_MT_POSITION_Y, 0, max_xy, 0, 0);
    touchscreen_parse_properties(input, true, &data.prop);
    error = input_mt_init_slots(input, EXC3000_NUM_SLOTS,
    INPUT_MT_DIRECT | INPUT_MT_DROP_UNUSED);
    if (error)
    return error;
    error = input_register_device(input);
    if (error)
    return error;
    error = devm_add_action_or_reset(&client.dev, exc3000_shutdown_timer,
    &data.timer);
    if (error)
    return error;
    error = devm_request_threaded_irq(&client.dev, client.irq,
    core::ptr::null_mut(), exc3000_interrupt, IRQF_ONESHOT,
    client.name, data);
    if (error)
    return error;
//
// I²C does not have built-in recovery, so retry on failure. This
// ensures, that the device probe will not fail for temporary issues
// on the bus.  This is not needed for the sysfs calls (userspace
// will receive the error code and can start another query) and
// cannot be done for touch events (but that only means loosing one
// or two touch events anyways).
//
    for (retry = 0; retry < 3; retry++) {
    u8 response[EXC3000_LEN_FRAME];
    error = exc3000_vendor_data_request(data, (u8[]){'E'}, 1,
    response, 1);
    if (error > 0) {
    dev_dbg(&client.dev, "TS Model: %s", &response[1]);
    error = 0;
    break;
    }
    dev_warn(&client.dev, "Retry %d get EETI EXC3000 model: %d\n",
    retry + 1, error);
    }
    if (error)
    return error;
    i2c_set_clientdata(client, data);
    return 0;
    }
    static const struct i2c_device_id exc3000_id[] = {
    { .name = "exc3000", .driver_data = EETI_EXC3000 },
    { .name = "exc80h60", .driver_data = EETI_EXC80H60 },
    { .name = "exc80h84", .driver_data = EETI_EXC80H84 },
    { .name = "exc81w32", .driver_data = EETI_EXC81W32 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, exc3000_id);

    static const struct of_device_id exc3000_of_match[] = {
    { .compatible = "eeti,exc3000", .data = &exc3000_info[EETI_EXC3000] },
    { .compatible = "eeti,exc80h60", .data = &exc3000_info[EETI_EXC80H60] },
    { .compatible = "eeti,exc80h84", .data = &exc3000_info[EETI_EXC80H84] },
    { .compatible = "eeti,exc81w32", .data = &exc3000_info[EETI_EXC81W32] },
    { }
    };
    MODULE_DEVICE_TABLE(of, exc3000_of_match);

    static const struct acpi_device_id exc3000_acpi_match[] = {
    { "EGA00001", .driver_data = (kernel_ulong_t)&exc3000_info[EETI_EXC80H60] },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, exc3000_acpi_match);

    static struct i2c_driver exc3000_driver = {
    .driver = {
    .name	= "exc3000",
    .dev_groups = exc3000_groups,
    .of_match_table = of_match_ptr(exc3000_of_match),
    .acpi_match_table = ACPI_PTR(exc3000_acpi_match),
    },
    .id_table	= exc3000_id,
    .probe		= exc3000_probe,
    };
    module_i2c_driver(exc3000_driver);
    MODULE_AUTHOR("Ahmet Inan <inan@distec.de>");
    MODULE_DESCRIPTION("I2C connected EETI EXC3000 multiple touch controller driver");
    MODULE_LICENSE("GPL v2");
