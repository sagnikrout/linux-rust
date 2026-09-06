//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/imagis.c
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

pub const IST30XX_REG_STATUS: c_uint = 0x20;

pub const IST30XX_WHOAMI: c_uint = 0x30003000;
pub const IST30XXA_WHOAMI: c_uint = 0x300a300a;
pub const IST30XXB_WHOAMI: c_uint = 0x300b300b;
pub const IST3038_WHOAMI: c_uint = 0x30383038;
pub const IST3032C_WHOAMI: c_uint = 0x32c;
pub const IST3038C_WHOAMI: c_uint = 0x38c;
pub const IST3038H_WHOAMI: c_uint = 0x38d;
pub const IST3038B_REG_CHIPID: c_uint = 0x30;
pub const IST3038B_WHOAMI: c_uint = 0x30380b;

pub const IST3038C_REG_HIB_BASE: c_uint = 0x30000100;

pub const IST3038C_CHIP_ON_DELAY_MS: c_int = 60;
pub const IST3038C_I2C_RETRY_COUNT: c_int = 3;
pub const IST3038C_MAX_FINGER_NUM: c_int = 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imagis_properties {
    pub interrupt_msg_cmd: c_uint,
    pub touch_coord_cmd: c_uint,
    pub whoami_cmd: c_uint,
    pub whoami_val: c_uint,
    pub protocol_b: bool,
    pub touch_keys_supported: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imagis_ts {
    pub client: *mut i2c_client,
    pub tdata: *const imagis_properties,
    pub input_dev: *mut input_dev,
    pub prop: touchscreen_properties,
    pub supplies: [regulator_bulk_data; 2],
    pub keycodes: [u32; 5],
    pub num_keycodes: c_int,
}

    static int imagis_i2c_read_reg(struct imagis_ts *ts,
    unsigned int reg, u32 *data)
    {
    __be32 ret_be;
    let mut reg_be: __be32 = cpu_to_be32(reg);
    struct i2c_msg msg[] = {
    {
    .addr = ts.client.addr,
    .flags = 0,
    .buf = (unsigned char *)&reg_be,
    .len = sizeof(reg_be),
    }, {
    .addr = ts.client.addr,
    .flags = I2C_M_RD,
    .buf = (unsigned char *)&ret_be,
    .len = sizeof(ret_be),
    },
    };
    int ret, error;
    let mut retry: c_int = IST3038C_I2C_RETRY_COUNT;
// Retry in case the controller fails to respond
    do {
    ret = i2c_transfer(ts.client.adapter, msg, ARRAY_SIZE(msg));
    if (ret == ARRAY_SIZE(msg)) {
// data = be32_to_cpu(ret_be);
    return 0;
    }
    error = ret < 0 ? ret : -EIO;
    dev_err(&ts.client.dev,
    "%s - i2c_transfer failed: %d (%d)\n",
    __func__, error, ret);
    } while (--retry);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn imagis_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t imagis_interrupt(int irq, void *dev_id)
    {
    struct imagis_ts *ts = dev_id;
    u32 intr_message, finger_status;
    unsigned int finger_count, finger_pressed, key_pressed;
    int i;
    int error;
    error = imagis_i2c_read_reg(ts, ts.tdata.interrupt_msg_cmd, &intr_message);
    if (error) {
    dev_err(&ts.client.dev,
    "failed to read the interrupt message: %d\n", error);
    goto out;
    }
    finger_count = FIELD_GET(IST3038C_FINGER_COUNT_MASK, intr_message);
    if (finger_count > IST3038C_MAX_FINGER_NUM) {
    dev_err(&ts.client.dev,
    "finger count %d is more than maximum supported\n",
    finger_count);
    goto out;
    }
    finger_pressed = FIELD_GET(IST3038C_FINGER_STATUS_MASK, intr_message);
    for (i = 0; i < finger_count; i++) {
    if (ts.tdata.protocol_b)
    error = imagis_i2c_read_reg(ts,
    ts.tdata.touch_coord_cmd + (i * 4),
    &finger_status);
    else
    error = imagis_i2c_read_reg(ts,
    ts.tdata.touch_coord_cmd, &finger_status);
    if (error) {
    dev_err(&ts.client.dev,
    "failed to read coordinates for finger %d: %d\n",
    i, error);
    goto out;
    }
    input_mt_slot(ts.input_dev, i);
    input_mt_report_slot_state(ts.input_dev, MT_TOOL_FINGER,
    finger_pressed & BIT(i));
    touchscreen_report_pos(ts.input_dev, &ts.prop,
    FIELD_GET(IST3038C_X_MASK, finger_status),
    FIELD_GET(IST3038C_Y_MASK, finger_status),
    true);
    input_report_abs(ts.input_dev, ABS_MT_TOUCH_MAJOR,
    FIELD_GET(IST3038C_AREA_MASK, finger_status));
    }
    key_pressed = FIELD_GET(IST3032C_KEY_STATUS_MASK, intr_message);
    for (int i = 0; i < ts.num_keycodes; i++)
    input_report_key(ts.input_dev, ts.keycodes[i],
    key_pressed & BIT(i));
    input_mt_sync_frame(ts.input_dev);
    input_sync(ts.input_dev);
    out:
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn imagis_power_off(_ts: *mut c_void) {
    static void imagis_power_off(void *_ts)
    {
    struct imagis_ts *ts = _ts;
    regulator_bulk_disable(ARRAY_SIZE(ts.supplies), ts.supplies);
    }
#[no_mangle]
unsafe extern "C" fn imagis_power_on(ts: *mut imagis_ts) -> c_int {
    static int imagis_power_on(struct imagis_ts *ts)
    {
    int error;
    error = regulator_bulk_enable(ARRAY_SIZE(ts.supplies), ts.supplies);
    if (error)
    return error;
    msleep(IST3038C_CHIP_ON_DELAY_MS);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imagis_start(ts: *mut imagis_ts) -> c_int {
    static int imagis_start(struct imagis_ts *ts)
    {
    int error;
    error = imagis_power_on(ts);
    if (error)
    return error;
    enable_irq(ts.client.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imagis_stop(ts: *mut imagis_ts) -> c_int {
    static int imagis_stop(struct imagis_ts *ts)
    {
    disable_irq(ts.client.irq);
    imagis_power_off(ts);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imagis_input_open(dev: *mut input_dev) -> c_int {
    static int imagis_input_open(struct input_dev *dev)
    {
    struct imagis_ts *ts = input_get_drvdata(dev);
    return imagis_start(ts);
    }
#[no_mangle]
unsafe extern "C" fn imagis_input_close(dev: *mut input_dev) {
    static void imagis_input_close(struct input_dev *dev)
    {
    struct imagis_ts *ts = input_get_drvdata(dev);
    imagis_stop(ts);
    }
#[no_mangle]
unsafe extern "C" fn imagis_init_input_dev(ts: *mut imagis_ts) -> c_int {
    static int imagis_init_input_dev(struct imagis_ts *ts)
    {
    struct input_dev *input_dev;
    int error;
    input_dev = devm_input_allocate_device(&ts.client.dev);
    if (!input_dev)
    return -ENOMEM;
    ts.input_dev = input_dev;
    input_dev.name = "Imagis capacitive touchscreen";
    input_dev.phys = "input/ts";
    input_dev.id.bustype = BUS_I2C;
    input_dev.open = imagis_input_open;
    input_dev.close = imagis_input_close;
    input_set_drvdata(input_dev, ts);
    input_set_capability(input_dev, EV_ABS, ABS_MT_POSITION_X);
    input_set_capability(input_dev, EV_ABS, ABS_MT_POSITION_Y);
    input_set_abs_params(input_dev, ABS_MT_TOUCH_MAJOR, 0, 16, 0, 0);
    if (ts.tdata.touch_keys_supported) {
    ts.num_keycodes = of_property_read_variable_u32_array(
    ts.client.dev.of_node, "linux,keycodes",
    ts.keycodes, 0, ARRAY_SIZE(ts.keycodes));
    if (ts.num_keycodes <= 0) {
    ts.keycodes[0] = KEY_APPSELECT;
    ts.keycodes[1] = KEY_BACK;
    ts.num_keycodes = 2;
    }
    input_dev.keycodemax = ts.num_keycodes;
    input_dev.keycodesize = sizeof(ts.keycodes[0]);
    input_dev.keycode = ts.keycodes;
    }
    for (int i = 0; i < ts.num_keycodes; i++)
    input_set_capability(input_dev, EV_KEY, ts.keycodes[i]);
    touchscreen_parse_properties(input_dev, true, &ts.prop);
    if (!ts.prop.max_x || !ts.prop.max_y) {
    dev_err(&ts.client.dev,
    "Touchscreen-size-x and/or touchscreen-size-y not set in dts\n");
    return -EINVAL;
    }
    error = input_mt_init_slots(input_dev,
    IST3038C_MAX_FINGER_NUM,
    INPUT_MT_DIRECT | INPUT_MT_DROP_UNUSED);
    if (error) {
    dev_err(&ts.client.dev,
    "Failed to initialize MT slots: %d", error);
    return error;
    }
    error = input_register_device(input_dev);
    if (error) {
    dev_err(&ts.client.dev,
    "Failed to register input device: %d", error);
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imagis_init_regulators(ts: *mut imagis_ts) -> c_int {
    static int imagis_init_regulators(struct imagis_ts *ts)
    {
    struct i2c_client *client = ts.client;
    ts.supplies[0].supply = "vdd";
    ts.supplies[1].supply = "vddio";
    return devm_regulator_bulk_get(&client.dev,
    ARRAY_SIZE(ts.supplies),
    ts.supplies);
    }
#[no_mangle]
unsafe extern "C" fn imagis_probe(i2c: *mut i2c_client) -> c_int {
    static int imagis_probe(struct i2c_client *i2c)
    {
    struct device *dev = &i2c.dev;
    struct imagis_ts *ts;
    int chip_id, error;
    ts = devm_kzalloc(dev, sizeof(*ts), GFP_KERNEL);
    if (!ts)
    return -ENOMEM;
    ts.client = i2c;
    ts.tdata = device_get_match_data(dev);
    if (!ts.tdata) {
    dev_err(dev, "missing chip data\n");
    return -EINVAL;
    }
    error = imagis_init_regulators(ts);
    if (error) {
    dev_err(dev, "regulator init error: %d\n", error);
    return error;
    }
    error = imagis_power_on(ts);
    if (error) {
    dev_err(dev, "failed to enable regulators: %d\n", error);
    return error;
    }
    error = devm_add_action_or_reset(dev, imagis_power_off, ts);
    if (error) {
    dev_err(dev, "failed to install poweroff action: %d\n", error);
    return error;
    }
    error = imagis_i2c_read_reg(ts, ts.tdata.whoami_cmd, &chip_id);
    if (error) {
    dev_err(dev, "chip ID read failure: %d\n", error);
    return error;
    }
    if (chip_id != ts.tdata.whoami_val) {
    dev_err(dev, "unknown chip ID: 0x%x\n", chip_id);
    return -EINVAL;
    }
    error = devm_request_threaded_irq(dev, i2c.irq,
    core::ptr::null_mut(), imagis_interrupt,
    IRQF_ONESHOT | IRQF_NO_AUTOEN,
    "imagis-touchscreen", ts);
    if (error) {
    dev_err(dev, "IRQ %d allocation failure: %d\n",
    i2c.irq, error);
    return error;
    }
    error = imagis_init_input_dev(ts);
    if (error)
    return error;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imagis_suspend(dev: *mut device) -> c_int {
    static int imagis_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct imagis_ts *ts = i2c_get_clientdata(client);
    int error;
    guard(mutex)(&ts.input_dev.mutex);
    if (input_device_enabled(ts.input_dev)) {
    error = imagis_stop(ts);
    if (error)
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imagis_resume(dev: *mut device) -> c_int {
    static int imagis_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct imagis_ts *ts = i2c_get_clientdata(client);
    int error;
    guard(mutex)(&ts.input_dev.mutex);
    if (input_device_enabled(ts.input_dev)) {
    error = imagis_start(ts);
    if (error)
    return error;
    }
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(imagis_pm_ops, imagis_suspend, imagis_resume);

    static const struct imagis_properties imagis_3032c_data = {
    .interrupt_msg_cmd = IST3038C_REG_INTR_MESSAGE,
    .touch_coord_cmd = IST3038C_REG_TOUCH_COORD,
    .whoami_cmd = IST3038C_REG_CHIPID,
    .whoami_val = IST3032C_WHOAMI,
    .touch_keys_supported = true,
    .protocol_b = true,
    };
    static const struct imagis_properties imagis_3038_data = {
    .interrupt_msg_cmd = IST30XX_REG_STATUS,
    .touch_coord_cmd = IST30XX_REG_STATUS,
    .whoami_cmd = IST30XX_REG_CHIPID,
    .whoami_val = IST3038_WHOAMI,
    .touch_keys_supported = true,
    };
    static const struct imagis_properties imagis_3038b_data = {
    .interrupt_msg_cmd = IST30XX_REG_STATUS,
    .touch_coord_cmd = IST30XX_REG_STATUS,
    .whoami_cmd = IST3038B_REG_CHIPID,
    .whoami_val = IST3038B_WHOAMI,
    };
    static const struct imagis_properties imagis_3038c_data = {
    .interrupt_msg_cmd = IST3038C_REG_INTR_MESSAGE,
    .touch_coord_cmd = IST3038C_REG_TOUCH_COORD,
    .whoami_cmd = IST3038C_REG_CHIPID,
    .whoami_val = IST3038C_WHOAMI,
    .protocol_b = true,
    };
    static const struct imagis_properties imagis_3038h_data = {
    .interrupt_msg_cmd = IST3038C_REG_INTR_MESSAGE,
    .touch_coord_cmd = IST3038C_REG_TOUCH_COORD,
    .whoami_cmd = IST3038C_REG_CHIPID,
    .whoami_val = IST3038H_WHOAMI,
    };
    static const struct of_device_id imagis_of_match[] = {
    { .compatible = "imagis,ist3032c", .data = &imagis_3032c_data },
    { .compatible = "imagis,ist3038", .data = &imagis_3038_data },
    { .compatible = "imagis,ist3038b", .data = &imagis_3038b_data },
    { .compatible = "imagis,ist3038c", .data = &imagis_3038c_data },
    { .compatible = "imagis,ist3038h", .data = &imagis_3038h_data },
    { },
    };
    MODULE_DEVICE_TABLE(of, imagis_of_match);

    static struct i2c_driver imagis_ts_driver = {
    .driver = {
    .name = "imagis-touchscreen",
    .pm = pm_sleep_ptr(&imagis_pm_ops),
    .of_match_table = of_match_ptr(imagis_of_match),
    },
    .probe = imagis_probe,
    };
    module_i2c_driver(imagis_ts_driver);
    MODULE_DESCRIPTION("Imagis IST3038C Touchscreen Driver");
    MODULE_AUTHOR("Markuss Broks <markuss.broks@gmail.com>");
    MODULE_LICENSE("GPL");
