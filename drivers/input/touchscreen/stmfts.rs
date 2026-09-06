//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/stmfts.c
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


// SPDX-License-Identifier: GPL-2.0
// STMicroelectronics FTS Touchscreen device driver
//
// Copyright (c) 2017 Samsung Electronics Co., Ltd.
// Copyright (c) 2017 Andi Shyti <andi@etezian.org>

// I2C commands
pub const STMFTS_READ_INFO: c_uint = 0x80;
pub const STMFTS_READ_STATUS: c_uint = 0x84;
pub const STMFTS_READ_ONE_EVENT: c_uint = 0x85;
pub const STMFTS_READ_ALL_EVENT: c_uint = 0x86;
pub const STMFTS_LATEST_EVENT: c_uint = 0x87;
pub const STMFTS_SLEEP_IN: c_uint = 0x90;
pub const STMFTS_SLEEP_OUT: c_uint = 0x91;
pub const STMFTS_MS_MT_SENSE_OFF: c_uint = 0x92;
pub const STMFTS_MS_MT_SENSE_ON: c_uint = 0x93;
pub const STMFTS_SS_HOVER_SENSE_OFF: c_uint = 0x94;
pub const STMFTS_SS_HOVER_SENSE_ON: c_uint = 0x95;
pub const STMFTS_MS_KEY_SENSE_OFF: c_uint = 0x9a;
pub const STMFTS_MS_KEY_SENSE_ON: c_uint = 0x9b;
pub const STMFTS_SYSTEM_RESET: c_uint = 0xa0;
pub const STMFTS_CLEAR_EVENT_STACK: c_uint = 0xa1;
pub const STMFTS_FULL_FORCE_CALIBRATION: c_uint = 0xa2;
pub const STMFTS_MS_CX_TUNING: c_uint = 0xa3;
pub const STMFTS_SS_CX_TUNING: c_uint = 0xa4;
// events
pub const STMFTS_EV_NO_EVENT: c_uint = 0x00;
pub const STMFTS_EV_MULTI_TOUCH_DETECTED: c_uint = 0x02;
pub const STMFTS_EV_MULTI_TOUCH_ENTER: c_uint = 0x03;
pub const STMFTS_EV_MULTI_TOUCH_LEAVE: c_uint = 0x04;
pub const STMFTS_EV_MULTI_TOUCH_MOTION: c_uint = 0x05;
pub const STMFTS_EV_HOVER_ENTER: c_uint = 0x07;
pub const STMFTS_EV_HOVER_LEAVE: c_uint = 0x08;
pub const STMFTS_EV_HOVER_MOTION: c_uint = 0x09;
pub const STMFTS_EV_KEY_STATUS: c_uint = 0x0e;
pub const STMFTS_EV_ERROR: c_uint = 0x0f;
pub const STMFTS_EV_CONTROLLER_READY: c_uint = 0x10;
pub const STMFTS_EV_SLEEP_OUT_CONTROLLER_READY: c_uint = 0x11;
pub const STMFTS_EV_STATUS: c_uint = 0x16;
pub const STMFTS_EV_DEBUG: c_uint = 0xdb;
// multi touch related event masks
pub const STMFTS_MASK_EVENT_ID: c_uint = 0x0f;
pub const STMFTS_MASK_TOUCH_ID: c_uint = 0xf0;
pub const STMFTS_MASK_LEFT_EVENT: c_uint = 0x0f;
pub const STMFTS_MASK_X_MSB: c_uint = 0x0f;
pub const STMFTS_MASK_Y_LSB: c_uint = 0xf0;
// key related event masks
pub const STMFTS_MASK_KEY_NO_TOUCH: c_uint = 0x00;
pub const STMFTS_MASK_KEY_MENU: c_uint = 0x01;
pub const STMFTS_MASK_KEY_BACK: c_uint = 0x02;
pub const STMFTS_EVENT_SIZE: c_int = 8;
pub const STMFTS_STACK_DEPTH: c_int = 32;

pub const STMFTS_MAX_FINGERS: c_int = 10;

    static const struct regulator_bulk_data stmfts_supplies[] = {
    { .supply = "vdd" },
    { .supply = "avdd" },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmfts_data {
    pub client: *mut i2c_client,
    pub input: *mut input_dev,
    pub reset_gpio: *mut gpio_desc,
    pub led_cdev: led_classdev,
    pub mutex: mutex,
    pub prop: touchscreen_properties,
    pub supplies: *mut regulator_bulk_data,
//
// Presence of ledvdd will be used also to check
// whether the LED is supported.
//
    pub ledvdd: *mut regulator,
    pub chip_id: u16,
    pub chip_ver: u8,
    pub fw_ver: u16,
    pub config_id: u8,
    pub config_ver: u8,
    pub data: [u8; STMFTS_DATA_MAX_SIZE],
    pub cmd_done: completion,
    pub use_key: bool,
    pub led_status: bool,
    pub hover_enabled: bool,
    pub running: bool,
}

    static int stmfts_brightness_set(struct led_classdev *led_cdev,
    enum led_brightness value)
    {
    struct stmfts_data *sdata = container_of(led_cdev,
    struct stmfts_data, led_cdev);
    int err;
    if (value != sdata.led_status && sdata.ledvdd) {
    if (!value) {
    regulator_disable(sdata.ledvdd);
    } else {
    err = regulator_enable(sdata.ledvdd);
    if (err) {
    dev_warn(&sdata.client.dev,
    "failed to enable ledvdd regulator: %d\n",
    err);
    return err;
    }
    }
    sdata.led_status = value;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stmfts_brightness_get(led_cdev: *mut led_classdev) -> enum led_brightness {
    static enum led_brightness stmfts_brightness_get(struct led_classdev *led_cdev)
    {
    struct stmfts_data *sdata = container_of(led_cdev,
    struct stmfts_data, led_cdev);
    return !!regulator_is_enabled(sdata.ledvdd);
    }
//
// We can't simply use i2c_smbus_read_i2c_block_data because we
// need to read 256 bytes, which exceeds the 255-byte SMBus block limit.
//
#[no_mangle]
unsafe extern "C" fn stmfts_read_events(sdata: *mut stmfts_data) -> c_int {
    static int stmfts_read_events(struct stmfts_data *sdata)
    {
    let mut cmd: u8 = STMFTS_READ_ALL_EVENT;
    struct i2c_msg msgs[2] = {
    {
    .addr	= sdata.client.addr,
    .len	= 1,
    .buf	= &cmd,
    },
    {
    .addr	= sdata.client.addr,
    .flags	= I2C_M_RD,
    .len	= STMFTS_DATA_MAX_SIZE,
    .buf	= sdata.data,
    },
    };
    int ret;
    ret = i2c_transfer(sdata.client.adapter, msgs, ARRAY_SIZE(msgs));
    if (ret < 0)
    return ret;
    let mut ret: return = = ARRAY_SIZE(msgs) ? 0 : -EIO;
    }
    static void stmfts_report_contact_event(struct stmfts_data *sdata,
    const u8 event[])
    {
    let mut slot_id: u8 = (event[0] & STMFTS_MASK_TOUCH_ID) >> 4;
    let mut x: u16 = event[1] | ((event[2] & STMFTS_MASK_X_MSB) << 8);
    let mut y: u16 = (event[2] >> 4) | (event[3] << 4);
    let mut maj: u8 = event[4];
    let mut min: u8 = event[5];
    let mut orientation: u8 = event[6];
    let mut area: u8 = event[7];
    input_mt_slot(sdata.input, slot_id);
    input_mt_report_slot_state(sdata.input, MT_TOOL_FINGER, true);
    input_report_abs(sdata.input, ABS_MT_POSITION_X, x);
    input_report_abs(sdata.input, ABS_MT_POSITION_Y, y);
    input_report_abs(sdata.input, ABS_MT_TOUCH_MAJOR, maj);
    input_report_abs(sdata.input, ABS_MT_TOUCH_MINOR, min);
    input_report_abs(sdata.input, ABS_MT_PRESSURE, area);
    input_report_abs(sdata.input, ABS_MT_ORIENTATION, orientation);
    input_sync(sdata.input);
    }
    static void stmfts_report_contact_release(struct stmfts_data *sdata,
    const u8 event[])
    {
    let mut slot_id: u8 = (event[0] & STMFTS_MASK_TOUCH_ID) >> 4;
    input_mt_slot(sdata.input, slot_id);
    input_mt_report_slot_inactive(sdata.input);
    input_sync(sdata.input);
    }
    static void stmfts_report_hover_event(struct stmfts_data *sdata,
    const u8 event[])
    {
    let mut x: u16 = (event[2] << 4) | (event[4] >> 4);
    let mut y: u16 = (event[3] << 4) | (event[4] & STMFTS_MASK_Y_LSB);
    let mut z: u8 = event[5];
    input_report_abs(sdata.input, ABS_X, x);
    input_report_abs(sdata.input, ABS_Y, y);
    input_report_abs(sdata.input, ABS_DISTANCE, z);
    input_sync(sdata.input);
    }
#[no_mangle]
unsafe extern "C" fn stmfts_report_key_event(sdata: *mut stmfts_data, event[]: u8) {
    static void stmfts_report_key_event(struct stmfts_data *sdata, const u8 event[])
    {
    switch (event[2]) {
    case 0:
    input_report_key(sdata.input, KEY_BACK, 0);
    input_report_key(sdata.input, KEY_MENU, 0);
    break;
    case STMFTS_MASK_KEY_BACK:
    input_report_key(sdata.input, KEY_BACK, 1);
    break;
    case STMFTS_MASK_KEY_MENU:
    input_report_key(sdata.input, KEY_MENU, 1);
    break;
    default:
    dev_warn(&sdata.client.dev,
    "unknown key event: %#02x\n", event[2]);
    break;
    }
    input_sync(sdata.input);
    }
#[no_mangle]
unsafe extern "C" fn stmfts_parse_events(sdata: *mut stmfts_data) {
    static void stmfts_parse_events(struct stmfts_data *sdata)
    {
    int i;
    for (i = 0; i < STMFTS_STACK_DEPTH; i++) {
    u8 *event = &sdata.data[i * STMFTS_EVENT_SIZE];
    switch (event[0]) {
    case STMFTS_EV_CONTROLLER_READY:
    case STMFTS_EV_SLEEP_OUT_CONTROLLER_READY:
    case STMFTS_EV_STATUS:
    complete(&sdata.cmd_done);
    fallthrough;
    case STMFTS_EV_NO_EVENT:
    case STMFTS_EV_DEBUG:
    return;
    }
    switch (event[0] & STMFTS_MASK_EVENT_ID) {
    case STMFTS_EV_MULTI_TOUCH_ENTER:
    case STMFTS_EV_MULTI_TOUCH_MOTION:
    stmfts_report_contact_event(sdata, event);
    break;
    case STMFTS_EV_MULTI_TOUCH_LEAVE:
    stmfts_report_contact_release(sdata, event);
    break;
    case STMFTS_EV_HOVER_ENTER:
    case STMFTS_EV_HOVER_LEAVE:
    case STMFTS_EV_HOVER_MOTION:
    stmfts_report_hover_event(sdata, event);
    break;
    case STMFTS_EV_KEY_STATUS:
    stmfts_report_key_event(sdata, event);
    break;
    case STMFTS_EV_ERROR:
    dev_warn(&sdata.client.dev,
    "error code: 0x%x%x%x%x%x%x",
    event[6], event[5], event[4],
    event[3], event[2], event[1]);
    break;
    default:
    dev_err(&sdata.client.dev,
    "unknown event %#02x\n", event[0]);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn stmfts_irq_handler(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t stmfts_irq_handler(int irq, void *dev)
    {
    struct stmfts_data *sdata = dev;
    int err;
    guard(mutex)(&sdata.mutex);
    err = stmfts_read_events(sdata);
    if (unlikely(err))
    dev_err(&sdata.client.dev,
    "failed to read events: %d\n", err);
    else
    stmfts_parse_events(sdata);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn stmfts_command(sdata: *mut stmfts_data, cmd: u8) -> c_int {
    static int stmfts_command(struct stmfts_data *sdata, const u8 cmd)
    {
    int err;
    reinit_completion(&sdata.cmd_done);
    err = i2c_smbus_write_byte(sdata.client, cmd);
    if (err)
    return err;
    if (!wait_for_completion_timeout(&sdata.cmd_done,
    msecs_to_jiffies(1000)))
    return -ETIMEDOUT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stmfts_input_open(dev: *mut input_dev) -> c_int {
    static int stmfts_input_open(struct input_dev *dev)
    {
    struct stmfts_data *sdata = input_get_drvdata(dev);
    int err;
    err = pm_runtime_resume_and_get(&sdata.client.dev);
    if (err)
    return err;
    err = i2c_smbus_write_byte(sdata.client, STMFTS_MS_MT_SENSE_ON);
    if (err) {
    pm_runtime_put_sync(&sdata.client.dev);
    return err;
    }
    scoped_guard(mutex, &sdata.mutex) {
    sdata.running = true;
    if (sdata.hover_enabled) {
    err = i2c_smbus_write_byte(sdata.client,
    STMFTS_SS_HOVER_SENSE_ON);
    if (err)
    dev_warn(&sdata.client.dev,
    "failed to enable hover\n");
    }
    }
    if (sdata.use_key) {
    err = i2c_smbus_write_byte(sdata.client,
    STMFTS_MS_KEY_SENSE_ON);
    if (err)
// I can still use only the touch screen
    dev_warn(&sdata.client.dev,
    "failed to enable touchkey\n");
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stmfts_input_close(dev: *mut input_dev) {
    static void stmfts_input_close(struct input_dev *dev)
    {
    struct stmfts_data *sdata = input_get_drvdata(dev);
    int err;
    err = i2c_smbus_write_byte(sdata.client, STMFTS_MS_MT_SENSE_OFF);
    if (err)
    dev_warn(&sdata.client.dev,
    "failed to disable touchscreen: %d\n", err);
    scoped_guard(mutex, &sdata.mutex) {
    sdata.running = false;
    if (sdata.hover_enabled) {
    err = i2c_smbus_write_byte(sdata.client,
    STMFTS_SS_HOVER_SENSE_OFF);
    if (err)
    dev_warn(&sdata.client.dev,
    "failed to disable hover: %d\n", err);
    }
    }
    if (sdata.use_key) {
    err = i2c_smbus_write_byte(sdata.client,
    STMFTS_MS_KEY_SENSE_OFF);
    if (err)
    dev_warn(&sdata.client.dev,
    "failed to disable touchkey: %d\n", err);
    }
    pm_runtime_put_sync(&sdata.client.dev);
    }
    static ssize_t stmfts_sysfs_chip_id(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct stmfts_data *sdata = dev_get_drvdata(dev);
    return sysfs_emit(buf, "%#x\n", sdata.chip_id);
    }
    static ssize_t stmfts_sysfs_chip_version(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct stmfts_data *sdata = dev_get_drvdata(dev);
    return sysfs_emit(buf, "%u\n", sdata.chip_ver);
    }
    static ssize_t stmfts_sysfs_fw_ver(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct stmfts_data *sdata = dev_get_drvdata(dev);
    return sysfs_emit(buf, "%u\n", sdata.fw_ver);
    }
    static ssize_t stmfts_sysfs_config_id(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct stmfts_data *sdata = dev_get_drvdata(dev);
    return sysfs_emit(buf, "%#x\n", sdata.config_id);
    }
    static ssize_t stmfts_sysfs_config_version(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct stmfts_data *sdata = dev_get_drvdata(dev);
    return sysfs_emit(buf, "%u\n", sdata.config_ver);
    }
    static ssize_t stmfts_sysfs_read_status(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct stmfts_data *sdata = dev_get_drvdata(dev);
    u8 status[4];
    int err;
    err = i2c_smbus_read_i2c_block_data(sdata.client, STMFTS_READ_STATUS,
    sizeof(status), status);
    if (err)
    return err;
    return sysfs_emit(buf, "%#02x\n", status[0]);
    }
    static ssize_t stmfts_sysfs_hover_enable_read(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct stmfts_data *sdata = dev_get_drvdata(dev);
    return sysfs_emit(buf, "%u\n", sdata.hover_enabled);
    }
    static ssize_t stmfts_sysfs_hover_enable_write(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t len)
    {
    struct stmfts_data *sdata = dev_get_drvdata(dev);
    unsigned long value;
    bool hover;
    int err;
    if (kstrtoul(buf, 0, &value))
    return -EINVAL;
    hover = !!value;
    guard(mutex)(&sdata.mutex);
    if (hover != sdata.hover_enabled) {
    if (sdata.running) {
    err = i2c_smbus_write_byte(sdata.client,
    value ? STMFTS_SS_HOVER_SENSE_ON :
    STMFTS_SS_HOVER_SENSE_OFF);
    if (err)
    return err;
    }
    sdata.hover_enabled = hover;
    }
    return len;
    }
    static DEVICE_ATTR(chip_id, 0444, stmfts_sysfs_chip_id, core::ptr::null_mut());
    static DEVICE_ATTR(chip_version, 0444, stmfts_sysfs_chip_version, core::ptr::null_mut());
    static DEVICE_ATTR(fw_ver, 0444, stmfts_sysfs_fw_ver, core::ptr::null_mut());
    static DEVICE_ATTR(config_id, 0444, stmfts_sysfs_config_id, core::ptr::null_mut());
    static DEVICE_ATTR(config_version, 0444, stmfts_sysfs_config_version, core::ptr::null_mut());
    static DEVICE_ATTR(status, 0444, stmfts_sysfs_read_status, core::ptr::null_mut());
    static DEVICE_ATTR(hover_enable, 0644, stmfts_sysfs_hover_enable_read,
    stmfts_sysfs_hover_enable_write);
    static struct attribute *stmfts_sysfs_attrs[] = {
    &dev_attr_chip_id.attr,
    &dev_attr_chip_version.attr,
    &dev_attr_fw_ver.attr,
    &dev_attr_config_id.attr,
    &dev_attr_config_version.attr,
    &dev_attr_status.attr,
    &dev_attr_hover_enable.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(stmfts_sysfs);
#[no_mangle]
unsafe extern "C" fn stmfts_read_system_info(sdata: *mut stmfts_data) -> c_int {
    static int stmfts_read_system_info(struct stmfts_data *sdata)
    {
    int err;
    u8 reg[8];
    err = i2c_smbus_read_i2c_block_data(sdata.client, STMFTS_READ_INFO,
    sizeof(reg), reg);
    if (err < 0)
    return err;
    if (err != sizeof(reg))
    return -EIO;
    sdata.chip_id = be16_to_cpup((__be16 *)&reg[6]);
    sdata.chip_ver = reg[0];
    sdata.fw_ver = be16_to_cpup((__be16 *)&reg[2]);
    sdata.config_id = reg[4];
    sdata.config_ver = reg[5];
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stmfts_reset(sdata: *mut stmfts_data) {
    static void stmfts_reset(struct stmfts_data *sdata)
    {
    gpiod_set_value_cansleep(sdata.reset_gpio, 1);
    msleep(20);
    gpiod_set_value_cansleep(sdata.reset_gpio, 0);
    msleep(50);
    }
#[no_mangle]
unsafe extern "C" fn stmfts_configure(sdata: *mut stmfts_data) -> c_int {
    static int stmfts_configure(struct stmfts_data *sdata)
    {
    int err;
    err = stmfts_command(sdata, STMFTS_SYSTEM_RESET);
    if (err)
    return err;
    err = stmfts_command(sdata, STMFTS_SLEEP_OUT);
    if (err)
    return err;
// optional tuning
    err = stmfts_command(sdata, STMFTS_MS_CX_TUNING);
    if (err)
    dev_warn(&sdata.client.dev,
    "failed to perform mutual auto tune: %d\n", err);
// optional tuning
    err = stmfts_command(sdata, STMFTS_SS_CX_TUNING);
    if (err)
    dev_warn(&sdata.client.dev,
    "failed to perform self auto tune: %d\n", err);
    err = stmfts_command(sdata, STMFTS_FULL_FORCE_CALIBRATION);
    if (err)
    return err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stmfts_power_on(sdata: *mut stmfts_data) -> c_int {
    static int stmfts_power_on(struct stmfts_data *sdata)
    {
    int err;
    err = regulator_bulk_enable(ARRAY_SIZE(stmfts_supplies),
    sdata.supplies);
    if (err)
    return err;
//
// The datasheet does not specify the power on time, but considering
// that the reset time is < 10ms, I sleep 20ms to be sure
//
    msleep(20);
    if (sdata.reset_gpio)
    stmfts_reset(sdata);
    err = stmfts_read_system_info(sdata);
    if (err)
    goto err_disable_regulators;
    enable_irq(sdata.client.irq);
    msleep(50);
    err = stmfts_configure(sdata);
    if (err)
    goto err_disable_irq;
//
// At this point no one is using the touchscreen
// and I don't really care about the return value
//
    (void)i2c_smbus_write_byte(sdata.client, STMFTS_SLEEP_IN);
    return 0;
    err_disable_irq:
    disable_irq(sdata.client.irq);
    err_disable_regulators:
    regulator_bulk_disable(ARRAY_SIZE(stmfts_supplies), sdata.supplies);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn stmfts_power_off(data: *mut c_void) {
    static void stmfts_power_off(void *data)
    {
    struct stmfts_data *sdata = data;
    disable_irq(sdata.client.irq);
    if (sdata.reset_gpio)
    gpiod_set_value_cansleep(sdata.reset_gpio, 1);
    regulator_bulk_disable(ARRAY_SIZE(stmfts_supplies), sdata.supplies);
    }
#[no_mangle]
unsafe extern "C" fn stmfts_enable_led(sdata: *mut stmfts_data) -> c_int {
    static int stmfts_enable_led(struct stmfts_data *sdata)
    {
    int err;
// get the regulator for powering the leds on
    sdata.ledvdd = devm_regulator_get(&sdata.client.dev, "ledvdd");
    if (IS_ERR(sdata.ledvdd))
    return PTR_ERR(sdata.ledvdd);
    sdata.led_cdev.name = STMFTS_DEV_NAME;
    sdata.led_cdev.max_brightness = LED_ON;
    sdata.led_cdev.brightness = LED_OFF;
    sdata.led_cdev.brightness_set_blocking = stmfts_brightness_set;
    sdata.led_cdev.brightness_get = stmfts_brightness_get;
    err = devm_led_classdev_register(&sdata.client.dev, &sdata.led_cdev);
    if (err) {
    devm_regulator_put(sdata.ledvdd);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stmfts_probe(client: *mut i2c_client) -> c_int {
    static int stmfts_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    int err;
    struct stmfts_data *sdata;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_I2C |
    I2C_FUNC_SMBUS_BYTE_DATA |
    I2C_FUNC_SMBUS_I2C_BLOCK))
    return -ENODEV;
    sdata = devm_kzalloc(dev, sizeof(*sdata), GFP_KERNEL);
    if (!sdata)
    return -ENOMEM;
    i2c_set_clientdata(client, sdata);
    sdata.client = client;
    mutex_init(&sdata.mutex);
    init_completion(&sdata.cmd_done);
    err = devm_regulator_bulk_get_const(dev,
    ARRAY_SIZE(stmfts_supplies),
    stmfts_supplies,
    &sdata.supplies);
    if (err)
    return err;
    sdata.reset_gpio = devm_gpiod_get_optional(dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(sdata.reset_gpio))
    return dev_err_probe(dev, PTR_ERR(sdata.reset_gpio),
    "Failed to get GPIO 'reset'\n");
    sdata.input = devm_input_allocate_device(dev);
    if (!sdata.input)
    return -ENOMEM;
    sdata.input.name = STMFTS_DEV_NAME;
    sdata.input.id.bustype = BUS_I2C;
    sdata.input.open = stmfts_input_open;
    sdata.input.close = stmfts_input_close;
    input_set_capability(sdata.input, EV_ABS, ABS_MT_POSITION_X);
    input_set_capability(sdata.input, EV_ABS, ABS_MT_POSITION_Y);
    touchscreen_parse_properties(sdata.input, true, &sdata.prop);
    input_set_abs_params(sdata.input, ABS_MT_TOUCH_MAJOR, 0, 255, 0, 0);
    input_set_abs_params(sdata.input, ABS_MT_TOUCH_MINOR, 0, 255, 0, 0);
    input_set_abs_params(sdata.input, ABS_MT_ORIENTATION, 0, 255, 0, 0);
    input_set_abs_params(sdata.input, ABS_MT_PRESSURE, 0, 255, 0, 0);
    input_set_abs_params(sdata.input, ABS_DISTANCE, 0, 255, 0, 0);
    sdata.use_key = device_property_read_bool(dev, "touch-key-connected");
    if (sdata.use_key) {
    input_set_capability(sdata.input, EV_KEY, KEY_MENU);
    input_set_capability(sdata.input, EV_KEY, KEY_BACK);
    }
    err = input_mt_init_slots(sdata.input,
    STMFTS_MAX_FINGERS, INPUT_MT_DIRECT);
    if (err)
    return err;
    input_set_drvdata(sdata.input, sdata);
//
// stmfts_power_on expects interrupt to be disabled, but
// at this point the device is still off and I do not trust
// the status of the irq line that can generate some spurious
// interrupts. To be on the safe side it's better to not enable
// the interrupts during their request.
//
    err = devm_request_threaded_irq(dev, client.irq,
    core::ptr::null_mut(), stmfts_irq_handler,
    IRQF_ONESHOT | IRQF_NO_AUTOEN,
    "stmfts_irq", sdata);
    if (err)
    return err;
    dev_dbg(dev, "initializing ST-Microelectronics FTS...\n");
    err = stmfts_power_on(sdata);
    if (err)
    return err;
    err = devm_add_action_or_reset(dev, stmfts_power_off, sdata);
    if (err)
    return err;
    err = input_register_device(sdata.input);
    if (err)
    return err;
    if (sdata.use_key) {
    err = stmfts_enable_led(sdata);
    if (err) {
//
// Even if the LEDs have failed to be initialized and
// used in the driver, I can still use the device even
// without LEDs. The ledvdd regulator pointer will be
// used as a flag.
//
    dev_warn(dev, "unable to use touchkey leds\n");
    sdata.ledvdd = core::ptr::null_mut();
    }
    }
    pm_runtime_enable(dev);
    device_enable_async_suspend(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stmfts_remove(client: *mut i2c_client) {
    static void stmfts_remove(struct i2c_client *client)
    {
    pm_runtime_disable(&client.dev);
    }
#[no_mangle]
unsafe extern "C" fn stmfts_runtime_suspend(dev: *mut device) -> c_int {
    static int stmfts_runtime_suspend(struct device *dev)
    {
    struct stmfts_data *sdata = dev_get_drvdata(dev);
    int ret;
    ret = i2c_smbus_write_byte(sdata.client, STMFTS_SLEEP_IN);
    if (ret)
    dev_warn(dev, "failed to suspend device: %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stmfts_runtime_resume(dev: *mut device) -> c_int {
    static int stmfts_runtime_resume(struct device *dev)
    {
    struct stmfts_data *sdata = dev_get_drvdata(dev);
    struct i2c_client *client = sdata.client;
    int ret;
    ret = i2c_smbus_write_byte(client, STMFTS_SLEEP_OUT);
    if (ret)
    dev_err(dev, "failed to resume device: %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stmfts_suspend(dev: *mut device) -> c_int {
    static int stmfts_suspend(struct device *dev)
    {
    struct stmfts_data *sdata = dev_get_drvdata(dev);
    stmfts_power_off(sdata);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stmfts_resume(dev: *mut device) -> c_int {
    static int stmfts_resume(struct device *dev)
    {
    struct stmfts_data *sdata = dev_get_drvdata(dev);
    return stmfts_power_on(sdata);
    }
    static const struct dev_pm_ops stmfts_pm_ops = {
    SYSTEM_SLEEP_PM_OPS(stmfts_suspend, stmfts_resume)
    RUNTIME_PM_OPS(stmfts_runtime_suspend, stmfts_runtime_resume, core::ptr::null_mut())
    };

    static const struct of_device_id stmfts_of_match[] = {
    { .compatible = "st,stmfts", },
    { },
    };
    MODULE_DEVICE_TABLE(of, stmfts_of_match);

    static const struct i2c_device_id stmfts_id[] = {
    { .name = "stmfts" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, stmfts_id);
    static struct i2c_driver stmfts_driver = {
    .driver = {
    .name = STMFTS_DEV_NAME,
    .dev_groups = stmfts_sysfs_groups,
    .of_match_table = of_match_ptr(stmfts_of_match),
    .pm = pm_ptr(&stmfts_pm_ops),
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    .probe = stmfts_probe,
    .remove = stmfts_remove,
    .id_table = stmfts_id,
    };
    module_i2c_driver(stmfts_driver);
    MODULE_AUTHOR("Andi Shyti <andi.shyti@samsung.com>");
    MODULE_DESCRIPTION("STMicroelectronics FTS Touch Screen");
    MODULE_LICENSE("GPL");
