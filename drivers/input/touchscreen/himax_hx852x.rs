//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/himax_hx852x.c
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
// Himax HX852x(ES) Touchscreen Driver
// Copyright (c) 2020-2024 Stephan Gerhold <stephan@gerhold.net>
// Copyright (c) 2020 Jonathan Albrieux <jonathan.albrieux@gmail.com>
//
// Based on the Himax Android Driver Sample Code Ver 0.3 for HMX852xES chipset:
// Copyright (c) 2014 Himax Corporation.
//

    HX852X_WIDTH_SIZE(fingers) + \
    sizeof(struct hx852x_touch_info))
pub const HX852X_MAX_FINGERS: c_int = 12;
pub const HX852X_MAX_KEY_COUNT: c_int = 4;

pub const HX852X_TS_SLEEP_IN: c_uint = 0x80;
pub const HX852X_TS_SLEEP_OUT: c_uint = 0x81;
pub const HX852X_TS_SENSE_OFF: c_uint = 0x82;
pub const HX852X_TS_SENSE_ON: c_uint = 0x83;
pub const HX852X_READ_ONE_EVENT: c_uint = 0x85;
pub const HX852X_READ_ALL_EVENTS: c_uint = 0x86;
pub const HX852X_READ_LATEST_EVENT: c_uint = 0x87;
pub const HX852X_CLEAR_EVENT_STACK: c_uint = 0x88;
pub const HX852X_REG_SRAM_SWITCH: c_uint = 0x8c;
pub const HX852X_REG_SRAM_ADDR: c_uint = 0x8b;
pub const HX852X_REG_FLASH_RPLACE: c_uint = 0x5a;
pub const HX852X_SRAM_SWITCH_TEST_MODE: c_uint = 0x14;
pub const HX852X_SRAM_ADDR_CONFIG: c_uint = 0x7000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hx852x {
    pub client: *mut i2c_client,
    pub input_dev: *mut input_dev,
    pub props: touchscreen_properties,
    pub reset_gpiod: *mut gpio_desc,
    pub supplies: [regulator_bulk_data; 2],
    pub max_fingers: c_uint,
    pub keycount: c_uint,
    pub keycodes: [c_uint; HX852X_MAX_KEY_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hx852x_config {
    pub rx_num: u8,
    pub tx_num: u8,
    pub max_pt: u8,
    pub padding1: [u8; 3],
    pub x_res: __be16,
    pub y_res: __be16,
    pub padding2: [u8; 2],
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hx852x_coord {
    pub x: __be16,
    pub y: __be16,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hx852x_touch_info {
    pub finger_num: u8,
    pub finger_pressed: __le16,
    pub padding: u8,
    pub __aligned(4): } __packed,
#[no_mangle]
unsafe extern "C" fn hx852x_i2c_read(hx: *mut hx852x, cmd: u8, data: *mut c_void, len: u16) -> c_int {
    static int hx852x_i2c_read(struct hx852x *hx, u8 cmd, void *data, u16 len)
    {
    pub hx->client: *mut *mut i2c_client client =,
    pub error: c_int,
    pub ret: c_int,
    struct i2c_msg msg[] = {
    {
    .addr = client.addr,
    .flags = 0,
    .len = 1,
    .buf = &cmd,
    },
    {
    .addr = client.addr,
    .flags = I2C_M_RD,
    .len = len,
    .buf = data,
    },
}

    ret = i2c_transfer(client.adapter, msg, ARRAY_SIZE(msg));
    if (ret != ARRAY_SIZE(msg)) {
    error = ret < 0 ? ret : -EIO;
    dev_err(&client.dev, "failed to read %#x: %d\n", cmd, error);
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hx852x_power_on(hx: *mut hx852x) -> c_int {
    static int hx852x_power_on(struct hx852x *hx)
    {
    struct device *dev = &hx.client.dev;
    int error;
    error = regulator_bulk_enable(ARRAY_SIZE(hx.supplies), hx.supplies);
    if (error) {
    dev_err(dev, "failed to enable regulators: %d\n", error);
    return error;
    }
    gpiod_set_value_cansleep(hx.reset_gpiod, 1);
    msleep(20);
    gpiod_set_value_cansleep(hx.reset_gpiod, 0);
    msleep(50);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hx852x_start(hx: *mut hx852x) -> c_int {
    static int hx852x_start(struct hx852x *hx)
    {
    struct device *dev = &hx.client.dev;
    int error;
    error = i2c_smbus_write_byte(hx.client, HX852X_TS_SLEEP_OUT);
    if (error) {
    dev_err(dev, "failed to send TS_SLEEP_OUT: %d\n", error);
    return error;
    }
    msleep(30);
    error = i2c_smbus_write_byte(hx.client, HX852X_TS_SENSE_ON);
    if (error) {
    dev_err(dev, "failed to send TS_SENSE_ON: %d\n", error);
    return error;
    }
    msleep(20);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hx852x_stop(hx: *mut hx852x) -> c_int {
    static int hx852x_stop(struct hx852x *hx)
    {
    struct device *dev = &hx.client.dev;
    int error;
    error = i2c_smbus_write_byte(hx.client, HX852X_TS_SENSE_OFF);
    if (error) {
    dev_err(dev, "failed to send TS_SENSE_OFF: %d\n", error);
    return error;
    }
    msleep(20);
    error = i2c_smbus_write_byte(hx.client, HX852X_TS_SLEEP_IN);
    if (error) {
    dev_err(dev, "failed to send TS_SLEEP_IN: %d\n", error);
    return error;
    }
    msleep(30);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hx852x_power_off(hx: *mut hx852x) -> c_int {
    static int hx852x_power_off(struct hx852x *hx)
    {
    struct device *dev = &hx.client.dev;
    int error;
    error = regulator_bulk_disable(ARRAY_SIZE(hx.supplies), hx.supplies);
    if (error) {
    dev_err(dev, "failed to disable regulators: %d\n", error);
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hx852x_read_config(hx: *mut hx852x) -> c_int {
    static int hx852x_read_config(struct hx852x *hx)
    {
    struct device *dev = &hx.client.dev;
    struct hx852x_config conf;
    int x_res, y_res;
    int error, error2;
    error = hx852x_power_on(hx);
    if (error)
    return error;
// Sensing must be turned on briefly to load the config
    error = hx852x_start(hx);
    if (error)
    goto err_power_off;
    error = hx852x_stop(hx);
    if (error)
    goto err_power_off;
    error = i2c_smbus_write_byte_data(hx.client, HX852X_REG_SRAM_SWITCH,
    HX852X_SRAM_SWITCH_TEST_MODE);
    if (error)
    goto err_power_off;
    error = i2c_smbus_write_word_data(hx.client, HX852X_REG_SRAM_ADDR,
    HX852X_SRAM_ADDR_CONFIG);
    if (error)
    goto err_test_mode;
    error = hx852x_i2c_read(hx, HX852X_REG_FLASH_RPLACE, &conf, sizeof(conf));
    if (error)
    goto err_test_mode;
    x_res = be16_to_cpu(conf.x_res);
    y_res = be16_to_cpu(conf.y_res);
    hx.max_fingers = (conf.max_pt & 0xf0) >> 4;
    dev_dbg(dev, "x res: %u, y res: %u, max fingers: %u\n",
    x_res, y_res, hx.max_fingers);
    if (hx.max_fingers > HX852X_MAX_FINGERS) {
    dev_err(dev, "max supported fingers: %u, found: %u\n",
    HX852X_MAX_FINGERS, hx.max_fingers);
    error = -EINVAL;
    goto err_test_mode;
    }
    if (x_res && y_res) {
    input_set_abs_params(hx.input_dev, ABS_MT_POSITION_X, 0, x_res - 1, 0, 0);
    input_set_abs_params(hx.input_dev, ABS_MT_POSITION_Y, 0, y_res - 1, 0, 0);
    }
    err_test_mode:
    error2 = i2c_smbus_write_byte_data(hx.client, HX852X_REG_SRAM_SWITCH, 0);
    error = error ?: error2;
    err_power_off:
    error2 = hx852x_power_off(hx);
    return error ?: error2;
    }
#[no_mangle]
unsafe extern "C" fn hx852x_handle_events(hx: *mut hx852x) -> c_int {
    static int hx852x_handle_events(struct hx852x *hx)
    {
//
// The event packets have variable size, depending on the amount of
// supported fingers (hx->max_fingers). They are laid out as follows:
// - struct hx852x_coord[hx->max_fingers]: Coordinates for each finger
// - u8[ALIGN(hx->max_fingers, 4)]: Touch width for each finger
// with padding for 32-bit alignment
// - struct hx852x_touch_info
//
// Load everything into a 32-bit aligned buffer so the coordinates
// can be assigned directly, without using get_unaligned_*().
//
    u8 buf[HX852X_MAX_BUF_SIZE] __aligned(4);
    struct hx852x_coord *coord = (struct hx852x_coord *)buf;
    u8 *width = &buf[HX852X_COORD_SIZE(hx.max_fingers)];
    struct hx852x_touch_info *info = (struct hx852x_touch_info *)
    &width[HX852X_WIDTH_SIZE(hx.max_fingers)];
    unsigned long finger_pressed, key_pressed;
    unsigned int i, x, y, w;
    int error;
    error = hx852x_i2c_read(hx, HX852X_READ_ALL_EVENTS, buf,
    HX852X_BUF_SIZE(hx.max_fingers));
    if (error)
    return error;
    finger_pressed = get_unaligned_le16(&info.finger_pressed);
    key_pressed = finger_pressed >> HX852X_MAX_FINGERS;
// All bits are set when no touch is detected
    if (info.finger_num == 0xff || !(info.finger_num & 0x0f))
    finger_pressed = 0;
    if (key_pressed == 0xf)
    key_pressed = 0;
    for_each_set_bit(i, &finger_pressed, hx.max_fingers) {
    x = be16_to_cpu(coord[i].x);
    y = be16_to_cpu(coord[i].y);
    w = width[i];
    input_mt_slot(hx.input_dev, i);
    input_mt_report_slot_state(hx.input_dev, MT_TOOL_FINGER, 1);
    touchscreen_report_pos(hx.input_dev, &hx.props, x, y, true);
    input_report_abs(hx.input_dev, ABS_MT_TOUCH_MAJOR, w);
    }
    input_mt_sync_frame(hx.input_dev);
    for (i = 0; i < hx.keycount; i++)
    input_report_key(hx.input_dev, hx.keycodes[i], key_pressed & BIT(i));
    input_sync(hx.input_dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hx852x_interrupt(irq: c_int, ptr: *mut c_void) -> irqreturn_t {
    static irqreturn_t hx852x_interrupt(int irq, void *ptr)
    {
    struct hx852x *hx = ptr;
    int error;
    error = hx852x_handle_events(hx);
    if (error) {
    dev_err_ratelimited(&hx.client.dev,
    "failed to handle events: %d\n", error);
    return IRQ_NONE;
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn hx852x_input_open(dev: *mut input_dev) -> c_int {
    static int hx852x_input_open(struct input_dev *dev)
    {
    struct hx852x *hx = input_get_drvdata(dev);
    int error;
    error = hx852x_power_on(hx);
    if (error)
    return error;
    error = hx852x_start(hx);
    if (error) {
    hx852x_power_off(hx);
    return error;
    }
    enable_irq(hx.client.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hx852x_input_close(dev: *mut input_dev) {
    static void hx852x_input_close(struct input_dev *dev)
    {
    struct hx852x *hx = input_get_drvdata(dev);
    hx852x_stop(hx);
    disable_irq(hx.client.irq);
    hx852x_power_off(hx);
    }
#[no_mangle]
unsafe extern "C" fn hx852x_parse_properties(hx: *mut hx852x) -> c_int {
    static int hx852x_parse_properties(struct hx852x *hx)
    {
    struct device *dev = &hx.client.dev;
    int error, count;
    count = device_property_count_u32(dev, "linux,keycodes");
    if (count == -EINVAL) {
// Property does not exist, keycodes are optional
    return 0;
    } else if (count < 0) {
    dev_err(dev, "Failed to read linux,keycodes: %d\n", count);
    return count;
    } else if (count > HX852X_MAX_KEY_COUNT) {
    dev_err(dev, "max supported keys: %u, found: %u\n",
    HX852X_MAX_KEY_COUNT, hx.keycount);
    return -EINVAL;
    }
    hx.keycount = count;
    error = device_property_read_u32_array(dev, "linux,keycodes",
    hx.keycodes, hx.keycount);
    if (error) {
    dev_err(dev, "failed to read linux,keycodes: %d\n", error);
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hx852x_probe(client: *mut i2c_client) -> c_int {
    static int hx852x_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct hx852x *hx;
    int error, i;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_I2C |
    I2C_FUNC_SMBUS_WRITE_BYTE |
    I2C_FUNC_SMBUS_WRITE_BYTE_DATA |
    I2C_FUNC_SMBUS_WRITE_WORD_DATA)) {
    dev_err(dev, "not all required i2c functionality supported\n");
    return -ENXIO;
    }
    hx = devm_kzalloc(dev, sizeof(*hx), GFP_KERNEL);
    if (!hx)
    return -ENOMEM;
    hx.client = client;
    hx.input_dev = devm_input_allocate_device(dev);
    if (!hx.input_dev)
    return -ENOMEM;
    hx.input_dev.name = "Himax HX852x";
    hx.input_dev.id.bustype = BUS_I2C;
    hx.input_dev.open = hx852x_input_open;
    hx.input_dev.close = hx852x_input_close;
    i2c_set_clientdata(client, hx);
    input_set_drvdata(hx.input_dev, hx);
    hx.supplies[0].supply = "vcca";
    hx.supplies[1].supply = "vccd";
    error = devm_regulator_bulk_get(dev, ARRAY_SIZE(hx.supplies), hx.supplies);
    if (error)
    return dev_err_probe(dev, error, "failed to get regulators\n");
    hx.reset_gpiod = devm_gpiod_get(dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(hx.reset_gpiod))
    return dev_err_probe(dev, PTR_ERR(hx.reset_gpiod),
    "failed to get reset gpio\n");
    error = devm_request_threaded_irq(dev, client.irq, core::ptr::null_mut(), hx852x_interrupt,
    IRQF_ONESHOT | IRQF_NO_AUTOEN, core::ptr::null_mut(), hx);
    if (error)
    return dev_err_probe(dev, error, "failed to request irq %d", client.irq);
    error = hx852x_read_config(hx);
    if (error)
    return error;
    input_set_capability(hx.input_dev, EV_ABS, ABS_MT_POSITION_X);
    input_set_capability(hx.input_dev, EV_ABS, ABS_MT_POSITION_Y);
    input_set_abs_params(hx.input_dev, ABS_MT_TOUCH_MAJOR, 0, 255, 0, 0);
    touchscreen_parse_properties(hx.input_dev, true, &hx.props);
    error = hx852x_parse_properties(hx);
    if (error)
    return error;
    hx.input_dev.keycode = hx.keycodes;
    hx.input_dev.keycodemax = hx.keycount;
    hx.input_dev.keycodesize = sizeof(hx.keycodes[0]);
    for (i = 0; i < hx.keycount; i++)
    input_set_capability(hx.input_dev, EV_KEY, hx.keycodes[i]);
    error = input_mt_init_slots(hx.input_dev, hx.max_fingers,
    INPUT_MT_DIRECT | INPUT_MT_DROP_UNUSED);
    if (error)
    return dev_err_probe(dev, error, "failed to init MT slots\n");
    error = input_register_device(hx.input_dev);
    if (error)
    return dev_err_probe(dev, error, "failed to register input device\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hx852x_suspend(dev: *mut device) -> c_int {
    static int hx852x_suspend(struct device *dev)
    {
    struct hx852x *hx = dev_get_drvdata(dev);
    guard(mutex)(&hx.input_dev.mutex);
    if (input_device_enabled(hx.input_dev))
    return hx852x_stop(hx);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hx852x_resume(dev: *mut device) -> c_int {
    static int hx852x_resume(struct device *dev)
    {
    struct hx852x *hx = dev_get_drvdata(dev);
    guard(mutex)(&hx.input_dev.mutex);
    if (input_device_enabled(hx.input_dev))
    return hx852x_start(hx);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(hx852x_pm_ops, hx852x_suspend, hx852x_resume);

    static const struct of_device_id hx852x_of_match[] = {
    { .compatible = "himax,hx852es" },
    { }
    };
    MODULE_DEVICE_TABLE(of, hx852x_of_match);

    static struct i2c_driver hx852x_driver = {
    .probe = hx852x_probe,
    .driver = {
    .name = "himax_hx852x",
    .pm = pm_sleep_ptr(&hx852x_pm_ops),
    .of_match_table = of_match_ptr(hx852x_of_match),
    },
    };
    module_i2c_driver(hx852x_driver);
    MODULE_DESCRIPTION("Himax HX852x(ES) Touchscreen Driver");
    MODULE_AUTHOR("Jonathan Albrieux <jonathan.albrieux@gmail.com>");
    MODULE_AUTHOR("Stephan Gerhold <stephan@gerhold.net>");
    MODULE_LICENSE("GPL");
