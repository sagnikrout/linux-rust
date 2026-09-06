//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/gigabyte_waterforce.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// hwmon driver for Gigabyte AORUS Waterforce AIO CPU coolers: X240, X280 and X360.
//
// Copyright 2023 Aleksa Savic <savicaleksa83@gmail.com>
//

pub const USB_VENDOR_ID_GIGABYTE: c_uint = 0x1044;
pub const USB_PRODUCT_ID_WATERFORCE: c_uint = 0x7a4d	/* Gigabyte AORUS WATERFORCE X240, X280 and X360 */;

pub const MAX_REPORT_LENGTH: c_int = 6144;
pub const WATERFORCE_TEMP_SENSOR: c_uint = 0xD;
pub const WATERFORCE_FAN_SPEED: c_uint = 0x02;
pub const WATERFORCE_PUMP_SPEED: c_uint = 0x05;
pub const WATERFORCE_FAN_DUTY: c_uint = 0x08;
pub const WATERFORCE_PUMP_DUTY: c_uint = 0x09;
// Control commands, inner offsets and lengths
    static const u8 get_status_cmd[] = { 0x99, 0xDA };
pub const FIRMWARE_VER_START_OFFSET_1: c_int = 2;
pub const FIRMWARE_VER_START_OFFSET_2: c_int = 3;
    static const u8 get_firmware_ver_cmd[] = { 0x99, 0xD6 };
// Command lengths
pub const GET_STATUS_CMD_LENGTH: c_int = 2;
pub const GET_FIRMWARE_VER_CMD_LENGTH: c_int = 2;
    static const char *const waterforce_temp_label[] = {
    "Coolant temp"
    };
    static const char *const waterforce_speed_label[] = {
    "Fan speed",
    "Pump speed"
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct waterforce_data {
    pub hdev: *mut hid_device,
    pub hwmon_dev: *mut device,
    pub debugfs: *mut dentry,
// For locking access to buffer
    pub buffer_lock: mutex,
// For queueing multiple readers
    pub status_report_request_mutex: mutex,
// For reinitializing the completion below
    pub status_report_request_lock: spinlock_t,
    pub status_report_received: completion,
    pub fw_version_processed: completion,
// Sensor data
    pub temp_input: [i32; 1],
    pub /: *mut *mut u16 speed_input[2]; / Fan and pump speed in RPM,
    pub /: *mut *mut u8 duty_input[2]; / Fan and pump duty in 0-100%,
    pub buffer: *mut u8,
    pub firmware_version: c_int,
    pub /: *mut *mut unsigned long updated; / jiffies,
}

    static umode_t waterforce_is_visible(const void *data,
    enum hwmon_sensor_types type, u32 attr, int channel)
    {
    switch (type) {
    case hwmon_temp:
    switch (attr) {
    case hwmon_temp_label:
    case hwmon_temp_input:
    return 0444;
    default:
    break;
    }
    break;
    case hwmon_fan:
    switch (attr) {
    case hwmon_fan_label:
    case hwmon_fan_input:
    return 0444;
    default:
    break;
    }
    break;
    case hwmon_pwm:
    switch (attr) {
    case hwmon_pwm_input:
    return 0444;
    default:
    break;
    }
    break;
    default:
    break;
    }
    return 0;
    }
// Writes the command to the device with the rest of the report filled with zeroes
#[no_mangle]
unsafe extern "C" fn waterforce_write_expanded(priv: *mut waterforce_data, cmd: *const u8, cmd_length: c_int) -> c_int {
    static int waterforce_write_expanded(struct waterforce_data *priv, const u8 *cmd, int cmd_length)
    {
    int ret;
    mutex_lock(&priv.buffer_lock);
    memcpy_and_pad(priv.buffer, MAX_REPORT_LENGTH, cmd, cmd_length, 0x00);
    ret = hid_hw_output_report(priv.hdev, priv.buffer, MAX_REPORT_LENGTH);
    mutex_unlock(&priv.buffer_lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn waterforce_get_status(priv: *mut waterforce_data) -> c_int {
    static int waterforce_get_status(struct waterforce_data *priv)
    {
    let mut ret: c_int = mutex_lock_interruptible(&priv.status_report_request_mutex);
    if (ret < 0)
    return ret;
    if (!time_after(jiffies, priv.updated + msecs_to_jiffies(STATUS_VALIDITY))) {
// Data is up to date
    goto unlock_and_return;
    }
//
// Disable raw event parsing for a moment to safely reinitialize the
// completion. Reinit is done because hidraw could have triggered
// the raw event parsing and marked the priv->status_report_received
// completion as done.
//
    spin_lock_bh(&priv.status_report_request_lock);
    reinit_completion(&priv.status_report_received);
    spin_unlock_bh(&priv.status_report_request_lock);
// Send command for getting status
    ret = waterforce_write_expanded(priv, get_status_cmd, GET_STATUS_CMD_LENGTH);
    if (ret < 0)
    goto unlock_and_return;
    ret = wait_for_completion_interruptible_timeout(&priv.status_report_received,
    msecs_to_jiffies(STATUS_VALIDITY));
    if (ret == 0)
    ret = -ETIMEDOUT;
    unlock_and_return:
    mutex_unlock(&priv.status_report_request_mutex);
    if (ret < 0)
    return ret;
    return 0;
    }
    static int waterforce_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    struct waterforce_data *priv = dev_get_drvdata(dev);
    let mut ret: c_int = waterforce_get_status(priv);
    if (ret < 0)
    return ret;
    switch (type) {
    case hwmon_temp:
// val = priv->temp_input[channel];
    break;
    case hwmon_fan:
// val = priv->speed_input[channel];
    break;
    case hwmon_pwm:
    switch (attr) {
    case hwmon_pwm_input:
// val = DIV_ROUND_CLOSEST(priv->duty_input[channel] * 255, 100);
    break;
    default:
    return -EOPNOTSUPP;
    }
    break;
    default:
    return -EOPNOTSUPP;	/* unreachable */
    }
    return 0;
    }
    static int waterforce_read_string(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, const char **str)
    {
    switch (type) {
    case hwmon_temp:
// str = waterforce_temp_label[channel];
    break;
    case hwmon_fan:
// str = waterforce_speed_label[channel];
    break;
    default:
    return -EOPNOTSUPP;	/* unreachable */
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn waterforce_get_fw_ver(hdev: *mut hid_device) -> c_int {
    static int waterforce_get_fw_ver(struct hid_device *hdev)
    {
    struct waterforce_data *priv = hid_get_drvdata(hdev);
    int ret;
    ret = waterforce_write_expanded(priv, get_firmware_ver_cmd, GET_FIRMWARE_VER_CMD_LENGTH);
    if (ret < 0)
    return ret;
    ret = wait_for_completion_interruptible_timeout(&priv.fw_version_processed,
    msecs_to_jiffies(STATUS_VALIDITY));
    if (ret == 0)
    return -ETIMEDOUT;
#[no_mangle]
pub unsafe extern "C" fn if(0: ret <) -> else {
    else if (ret < 0)
    return ret;
    return 0;
    }
    static const struct hwmon_ops waterforce_hwmon_ops = {
    .is_visible = waterforce_is_visible,
    .read = waterforce_read,
    .read_string = waterforce_read_string
    };
    static const struct hwmon_channel_info *waterforce_info[] = {
    HWMON_CHANNEL_INFO(temp,
    HWMON_T_INPUT | HWMON_T_LABEL),
    HWMON_CHANNEL_INFO(fan,
    HWMON_F_INPUT | HWMON_F_LABEL,
    HWMON_F_INPUT | HWMON_F_LABEL),
    HWMON_CHANNEL_INFO(pwm,
    HWMON_PWM_INPUT,
    HWMON_PWM_INPUT),
    core::ptr::null_mut()
    };
    static const struct hwmon_chip_info waterforce_chip_info = {
    .ops = &waterforce_hwmon_ops,
    .info = waterforce_info,
    };
    static int waterforce_raw_event(struct hid_device *hdev, struct hid_report *report, u8 *data,
    int size)
    {
    struct waterforce_data *priv = hid_get_drvdata(hdev);
    if (data[0] == get_firmware_ver_cmd[0] && data[1] == get_firmware_ver_cmd[1]) {
// Received a firmware version report
    priv.firmware_version =
    data[FIRMWARE_VER_START_OFFSET_1] * 10 + data[FIRMWARE_VER_START_OFFSET_2];
    if (!completion_done(&priv.fw_version_processed))
    complete_all(&priv.fw_version_processed);
    return 0;
    }
    if (data[0] != get_status_cmd[0] || data[1] != get_status_cmd[1])
    return 0;
    priv.temp_input[0] = data[WATERFORCE_TEMP_SENSOR] * 1000;
    priv.speed_input[0] = get_unaligned_le16(data + WATERFORCE_FAN_SPEED);
    priv.speed_input[1] = get_unaligned_le16(data + WATERFORCE_PUMP_SPEED);
    priv.duty_input[0] = data[WATERFORCE_FAN_DUTY];
    priv.duty_input[1] = data[WATERFORCE_PUMP_DUTY];
    spin_lock(&priv.status_report_request_lock);
    if (!completion_done(&priv.status_report_received))
    complete_all(&priv.status_report_received);
    spin_unlock(&priv.status_report_request_lock);
    priv.updated = jiffies;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn firmware_version_show(seqf: *mut seq_file, unused: *mut c_void) -> c_int {
    static int firmware_version_show(struct seq_file *seqf, void *unused)
    {
    struct waterforce_data *priv = seqf.private;
    seq_printf(seqf, "%u\n", priv.firmware_version);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(firmware_version);
#[no_mangle]
unsafe extern "C" fn waterforce_debugfs_init(priv: *mut waterforce_data) {
    static void waterforce_debugfs_init(struct waterforce_data *priv)
    {
    char name[64];
    if (!priv.firmware_version)
    return;	/* There's nothing to show in debugfs */
    scnprintf(name, sizeof(name), "%s-%s", DRIVER_NAME, dev_name(&priv.hdev.dev));
    priv.debugfs = debugfs_create_dir(name, core::ptr::null_mut());
    debugfs_create_file("firmware_version", 0444, priv.debugfs, priv, &firmware_version_fops);
    }
#[no_mangle]
unsafe extern "C" fn waterforce_probe(hdev: *mut hid_device, id: *const hid_device_id) -> c_int {
    static int waterforce_probe(struct hid_device *hdev, const struct hid_device_id *id)
    {
    struct waterforce_data *priv;
    int ret;
    priv = devm_kzalloc(&hdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.hdev = hdev;
    hid_set_drvdata(hdev, priv);
//
// Initialize priv->updated to STATUS_VALIDITY seconds in the past, making
// the initial empty data invalid for waterforce_read() without the need for
// a special case there.
//
    priv.updated = jiffies - msecs_to_jiffies(STATUS_VALIDITY);
    ret = hid_parse(hdev);
    if (ret) {
    hid_err(hdev, "hid parse failed with %d\n", ret);
    return ret;
    }
//
// Enable hidraw so existing user-space tools can continue to work.
//
    ret = hid_hw_start(hdev, HID_CONNECT_HIDRAW);
    if (ret) {
    hid_err(hdev, "hid hw start failed with %d\n", ret);
    return ret;
    }
    ret = hid_hw_open(hdev);
    if (ret) {
    hid_err(hdev, "hid hw open failed with %d\n", ret);
    goto fail_and_stop;
    }
    priv.buffer = devm_kzalloc(&hdev.dev, MAX_REPORT_LENGTH, GFP_KERNEL);
    if (!priv.buffer) {
    ret = -ENOMEM;
    goto fail_and_close;
    }
    mutex_init(&priv.status_report_request_mutex);
    mutex_init(&priv.buffer_lock);
    spin_lock_init(&priv.status_report_request_lock);
    init_completion(&priv.status_report_received);
    init_completion(&priv.fw_version_processed);
    hid_device_io_start(hdev);
    ret = waterforce_get_fw_ver(hdev);
    if (ret < 0)
    hid_warn(hdev, "fw version request failed with %d\n", ret);
    priv.hwmon_dev = hwmon_device_register_with_info(&hdev.dev, "waterforce",
    priv, &waterforce_chip_info, core::ptr::null_mut());
    if (IS_ERR(priv.hwmon_dev)) {
    ret = PTR_ERR(priv.hwmon_dev);
    hid_err(hdev, "hwmon registration failed with %d\n", ret);
    goto fail_and_io_stop;
    }
    waterforce_debugfs_init(priv);
    return 0;
    fail_and_io_stop:
    hid_device_io_stop(hdev);
    fail_and_close:
    hid_hw_close(hdev);
    fail_and_stop:
    hid_hw_stop(hdev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn waterforce_remove(hdev: *mut hid_device) {
    static void waterforce_remove(struct hid_device *hdev)
    {
    struct waterforce_data *priv = hid_get_drvdata(hdev);
    debugfs_remove_recursive(priv.debugfs);
    hwmon_device_unregister(priv.hwmon_dev);
    hid_hw_close(hdev);
    hid_hw_stop(hdev);
    }
    static const struct hid_device_id waterforce_table[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_GIGABYTE, USB_PRODUCT_ID_WATERFORCE) },
    { }
    };
    MODULE_DEVICE_TABLE(hid, waterforce_table);
    static struct hid_driver waterforce_driver = {
    .name = "waterforce",
    .id_table = waterforce_table,
    .probe = waterforce_probe,
    .remove = waterforce_remove,
    .raw_event = waterforce_raw_event,
    };
#[no_mangle]
unsafe extern "C" fn waterforce_init() -> int __init {
    static int __init waterforce_init(void)
    {
    return hid_register_driver(&waterforce_driver);
    }
#[no_mangle]
unsafe extern "C" fn waterforce_exit() -> void __exit {
    static void __exit waterforce_exit(void)
    {
    hid_unregister_driver(&waterforce_driver);
    }
// When compiled into the kernel, initialize after the HID bus
    late_initcall(waterforce_init);
    module_exit(waterforce_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Aleksa Savic <savicaleksa83@gmail.com>");
    MODULE_DESCRIPTION("Hwmon driver for Gigabyte AORUS Waterforce AIO coolers");
