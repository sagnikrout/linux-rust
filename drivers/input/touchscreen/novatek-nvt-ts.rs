//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/novatek-nvt-ts.c
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
// Driver for Novatek NT11205 i2c touchscreen controller as found
// on the Acer Iconia One 7 B1-750 tablet.
//
// Copyright (c) 2023 Hans de Goede <hdegoede@redhat.com>
//

pub const NVT_TS_TOUCH_START: c_uint = 0x00;
pub const NVT_TS_TOUCH_SIZE: c_int = 6;
pub const NVT_TS_PARAMETERS_START: c_uint = 0x78;
// These are offsets from NVT_TS_PARAMETERS_START
pub const NVT_TS_PARAMS_WIDTH: c_uint = 0x04;
pub const NVT_TS_PARAMS_HEIGHT: c_uint = 0x06;
pub const NVT_TS_PARAMS_MAX_TOUCH: c_uint = 0x09;
pub const NVT_TS_PARAMS_MAX_BUTTONS: c_uint = 0x0a;
pub const NVT_TS_PARAMS_IRQ_TYPE: c_uint = 0x0b;
pub const NVT_TS_PARAMS_CHIP_ID: c_uint = 0x0e;
pub const NVT_TS_PARAMS_SIZE: c_uint = 0x0f;
pub const NVT_TS_MAX_TOUCHES: c_int = 10;
pub const NVT_TS_MAX_SIZE: c_int = 4096;
pub const NVT_TS_TOUCH_INVALID: c_uint = 0xff;
pub const NVT_TS_TOUCH_SLOT_SHIFT: c_int = 3;

pub const NVT_TS_TOUCH_NEW: c_int = 1;
pub const NVT_TS_TOUCH_UPDATE: c_int = 2;
pub const NVT_TS_TOUCH_RELEASE: c_int = 3;
    static const int nvt_ts_irq_type[4] = {
    IRQF_TRIGGER_RISING,
    IRQF_TRIGGER_FALLING,
    IRQF_TRIGGER_LOW,
    IRQF_TRIGGER_HIGH
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvt_ts_i2c_chip_data {
    pub chip_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvt_ts_data {
    pub client: *mut i2c_client,
    pub input: *mut input_dev,
    pub reset_gpio: *mut gpio_desc,
    pub regulators: [regulator_bulk_data; 2],
    pub prop: touchscreen_properties,
    pub max_touches: c_int,
    pub NVT_TS_MAX_TOUCHES]: *mut *mut u8 buf[NVT_TS_TOUCH_SIZE,
}

#[no_mangle]
unsafe extern "C" fn nvt_ts_read_data(client: *mut i2c_client, reg: u8, data: *mut u8, count: c_int) -> c_int {
    static int nvt_ts_read_data(struct i2c_client *client, u8 reg, u8 *data, int count)
    {
    struct i2c_msg msg[2] = {
    {
    .addr = client.addr,
    .len = 1,
    .buf = &reg,
    },
    {
    .addr = client.addr,
    .flags = I2C_M_RD,
    .len = count,
    .buf = data,
    }
    };
    int ret;
    ret = i2c_transfer(client.adapter, msg, ARRAY_SIZE(msg));
    if (ret != ARRAY_SIZE(msg)) {
    dev_err(&client.dev, "Error reading from 0x%02x: %d\n", reg, ret);
    return (ret < 0) ? ret : -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nvt_ts_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t nvt_ts_irq(int irq, void *dev_id)
    {
    struct nvt_ts_data *data = dev_id;
    struct device *dev = &data.client.dev;
    int i, error, slot, x, y;
    bool active;
    u8 *touch;
    error = nvt_ts_read_data(data.client, NVT_TS_TOUCH_START, data.buf,
    data.max_touches * NVT_TS_TOUCH_SIZE);
    if (error)
    return IRQ_HANDLED;
    for (i = 0; i < data.max_touches; i++) {
    touch = &data.buf[i * NVT_TS_TOUCH_SIZE];
    if (touch[0] == NVT_TS_TOUCH_INVALID)
    continue;
    slot = touch[0] >> NVT_TS_TOUCH_SLOT_SHIFT;
    if (slot < 1 || slot > data.max_touches) {
    dev_warn(dev, "slot %d out of range, ignoring\n", slot);
    continue;
    }
    switch (touch[0] & NVT_TS_TOUCH_TYPE_MASK) {
    case NVT_TS_TOUCH_NEW:
    case NVT_TS_TOUCH_UPDATE:
    active = true;
    break;
    case NVT_TS_TOUCH_RELEASE:
    active = false;
    break;
    default:
    dev_warn(dev, "slot %d unknown state %d\n", slot, touch[0] & 7);
    continue;
    }
    slot--;
    x = (touch[1] << 4) | (touch[3] >> 4);
    y = (touch[2] << 4) | (touch[3] & 0x0f);
    input_mt_slot(data.input, slot);
    input_mt_report_slot_state(data.input, MT_TOOL_FINGER, active);
    touchscreen_report_pos(data.input, &data.prop, x, y, true);
    }
    input_mt_sync_frame(data.input);
    input_sync(data.input);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn nvt_ts_start(dev: *mut input_dev) -> c_int {
    static int nvt_ts_start(struct input_dev *dev)
    {
    struct nvt_ts_data *data = input_get_drvdata(dev);
    int error;
    error = regulator_bulk_enable(ARRAY_SIZE(data.regulators), data.regulators);
    if (error) {
    dev_err(&data.client.dev, "failed to enable regulators\n");
    return error;
    }
    enable_irq(data.client.irq);
    gpiod_set_value_cansleep(data.reset_gpio, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nvt_ts_stop(dev: *mut input_dev) {
    static void nvt_ts_stop(struct input_dev *dev)
    {
    struct nvt_ts_data *data = input_get_drvdata(dev);
    disable_irq(data.client.irq);
    gpiod_set_value_cansleep(data.reset_gpio, 1);
    regulator_bulk_disable(ARRAY_SIZE(data.regulators), data.regulators);
    }
#[no_mangle]
unsafe extern "C" fn nvt_ts_suspend(dev: *mut device) -> c_int {
    static int nvt_ts_suspend(struct device *dev)
    {
    struct nvt_ts_data *data = i2c_get_clientdata(to_i2c_client(dev));
    guard(mutex)(&data.input.mutex);
    if (input_device_enabled(data.input))
    nvt_ts_stop(data.input);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nvt_ts_resume(dev: *mut device) -> c_int {
    static int nvt_ts_resume(struct device *dev)
    {
    struct nvt_ts_data *data = i2c_get_clientdata(to_i2c_client(dev));
    guard(mutex)(&data.input.mutex);
    if (input_device_enabled(data.input))
    nvt_ts_start(data.input);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(nvt_ts_pm_ops, nvt_ts_suspend, nvt_ts_resume);
#[no_mangle]
unsafe extern "C" fn nvt_ts_probe(client: *mut i2c_client) -> c_int {
    static int nvt_ts_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    int error, width, height, irq_type;
    struct nvt_ts_data *data;
    const struct nvt_ts_i2c_chip_data *chip;
    struct input_dev *input;
    if (!client.irq) {
    dev_err(dev, "Error no irq specified\n");
    return -EINVAL;
    }
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    chip = device_get_match_data(&client.dev);
    if (!chip)
    return -EINVAL;
    data.client = client;
    i2c_set_clientdata(client, data);
//
// VCC is the analog voltage supply
// IOVCC is the digital voltage supply
//
    data.regulators[0].supply = "vcc";
    data.regulators[1].supply = "iovcc";
    error = devm_regulator_bulk_get(dev, ARRAY_SIZE(data.regulators), data.regulators);
    if (error) {
    dev_err(dev, "cannot get regulators: %d\n", error);
    return error;
    }
    error = regulator_bulk_enable(ARRAY_SIZE(data.regulators), data.regulators);
    if (error) {
    dev_err(dev, "failed to enable regulators: %d\n", error);
    return error;
    }
    data.reset_gpio = devm_gpiod_get(dev, "reset", GPIOD_OUT_LOW);
    error = PTR_ERR_OR_ZERO(data.reset_gpio);
    if (error) {
    regulator_bulk_disable(ARRAY_SIZE(data.regulators), data.regulators);
    dev_err(dev, "failed to request reset GPIO: %d\n", error);
    return error;
    }
// Wait for controller to come out of reset before params read
    msleep(100);
    error = nvt_ts_read_data(data.client, NVT_TS_PARAMETERS_START,
    data.buf, NVT_TS_PARAMS_SIZE);
    gpiod_set_value_cansleep(data.reset_gpio, 1); /* Put back in reset */
    regulator_bulk_disable(ARRAY_SIZE(data.regulators), data.regulators);
    if (error)
    return error;
    width  = get_unaligned_be16(&data.buf[NVT_TS_PARAMS_WIDTH]);
    height = get_unaligned_be16(&data.buf[NVT_TS_PARAMS_HEIGHT]);
    data.max_touches = data.buf[NVT_TS_PARAMS_MAX_TOUCH];
    irq_type = data.buf[NVT_TS_PARAMS_IRQ_TYPE];
    if (width > NVT_TS_MAX_SIZE || height >= NVT_TS_MAX_SIZE ||
    data.max_touches > NVT_TS_MAX_TOUCHES ||
    irq_type >= ARRAY_SIZE(nvt_ts_irq_type) ||
    data.buf[NVT_TS_PARAMS_CHIP_ID] != chip.chip_id) {
    dev_err(dev, "Unsupported touchscreen parameters: %*ph\n",
    NVT_TS_PARAMS_SIZE, data.buf);
    return -EIO;
    }
    dev_dbg(dev, "Detected %dx%d touchscreen with %d max touches\n",
    width, height, data.max_touches);
    if (data.buf[NVT_TS_PARAMS_MAX_BUTTONS])
    dev_warn(dev, "Touchscreen buttons are not supported\n");
    input = devm_input_allocate_device(dev);
    if (!input)
    return -ENOMEM;
    input.name = client.name;
    input.id.bustype = BUS_I2C;
    input.open = nvt_ts_start;
    input.close = nvt_ts_stop;
    input_set_abs_params(input, ABS_MT_POSITION_X, 0, width - 1, 0, 0);
    input_set_abs_params(input, ABS_MT_POSITION_Y, 0, height - 1, 0, 0);
    touchscreen_parse_properties(input, true, &data.prop);
    error = input_mt_init_slots(input, data.max_touches,
    INPUT_MT_DIRECT | INPUT_MT_DROP_UNUSED);
    if (error)
    return error;
    data.input = input;
    input_set_drvdata(input, data);
    error = devm_request_threaded_irq(dev, client.irq, core::ptr::null_mut(), nvt_ts_irq,
    IRQF_ONESHOT | IRQF_NO_AUTOEN |
    nvt_ts_irq_type[irq_type],
    client.name, data);
    if (error) {
    dev_err(dev, "failed to request irq: %d\n", error);
    return error;
    }
    error = input_register_device(input);
    if (error) {
    dev_err(dev, "failed to register input device: %d\n", error);
    return error;
    }
    return 0;
    }
    static const struct nvt_ts_i2c_chip_data nvt_nt11205_ts_data = {
    .chip_id = 0x05,
    };
    static const struct nvt_ts_i2c_chip_data nvt_nt36672a_ts_data = {
    .chip_id = 0x08,
    };
    static const struct of_device_id nvt_ts_of_match[] = {
    { .compatible = "novatek,nt11205-ts", .data = &nvt_nt11205_ts_data },
    { .compatible = "novatek,nt36672a-ts", .data = &nvt_nt36672a_ts_data },
    { }
    };
    MODULE_DEVICE_TABLE(of, nvt_ts_of_match);
    static const struct i2c_device_id nvt_ts_i2c_id[] = {
    { .name = "nt11205-ts", .driver_data = (unsigned long)&nvt_nt11205_ts_data },
    { .name = "nt36672a-ts", .driver_data = (unsigned long)&nvt_nt36672a_ts_data },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, nvt_ts_i2c_id);
    static struct i2c_driver nvt_ts_driver = {
    .driver = {
    .name	= "novatek-nvt-ts",
    .pm	= pm_sleep_ptr(&nvt_ts_pm_ops),
    .of_match_table = nvt_ts_of_match,
    },
    .probe = nvt_ts_probe,
    .id_table = nvt_ts_i2c_id,
    };
    module_i2c_driver(nvt_ts_driver);
    MODULE_DESCRIPTION("Novatek NT11205 touchscreen driver");
    MODULE_AUTHOR("Hans de Goede <hdegoede@redhat.com>");
    MODULE_LICENSE("GPL");
