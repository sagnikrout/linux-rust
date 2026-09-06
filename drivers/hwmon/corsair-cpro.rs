//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/corsair-cpro.c
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
// corsair-cpro.c - Linux driver for Corsair Commander Pro
// Copyright (C) 2020 Marius Zachmann <mail@mariuszachmann.de>
//
// This driver uses hid reports to communicate with the device to allow hidraw userspace drivers
// still being used. The device does not use report ids. When using hidraw and this driver
// simultaniously, reports could be switched.
//

pub const USB_VENDOR_ID_CORSAIR: c_uint = 0x1b1c;
pub const USB_PRODUCT_ID_CORSAIR_COMMANDERPRO: c_uint = 0x0c10;
pub const USB_PRODUCT_ID_CORSAIR_1000D: c_uint = 0x1d00;
pub const OUT_BUFFER_SIZE: c_int = 63;
pub const IN_BUFFER_SIZE: c_int = 16;
pub const LABEL_LENGTH: c_int = 11;
pub const REQ_TIMEOUT: c_int = 300;
pub const CTL_GET_FW_VER: c_uint = 0x02	/* returns the firmware version in bytes 1-3 */;
pub const CTL_GET_BL_VER: c_uint = 0x06	/* returns the bootloader version in bytes 1-2 */;
pub const CTL_GET_TMP_CNCT: c_uint = 0x10	/*;
// returns in bytes 1-4 for each temp sensor:
// 0 not connected
// 1 connected
//
pub const CTL_GET_TMP: c_uint = 0x11	/*;
// send: byte 1 is channel, rest zero
// rcv:  returns temp for channel in centi-degree celsius
// in bytes 1 and 2 as a two's complement value
// returns 0x11 in byte 0 if no sensor is connected
//
pub const CTL_GET_VOLT: c_uint = 0x12	/*;
// send: byte 1 is rail number: 0 = 12v, 1 = 5v, 2 = 3.3v
// rcv:  returns millivolt in bytes 1,2
// returns error 0x10 if request is invalid
//
pub const CTL_GET_FAN_CNCT: c_uint = 0x20	/*;
// returns in bytes 1-6 for each fan:
// 0 not connected
// 1 3pin
// 2 4pin
//
pub const CTL_GET_FAN_RPM: c_uint = 0x21	/*;
// send: byte 1 is channel, rest zero
// rcv:  returns rpm in bytes 1,2
//
pub const CTL_GET_FAN_PWM: c_uint = 0x22	/*;
// send: byte 1 is channel, rest zero
// rcv:  returns pwm in byte 1 if it was set
// returns error 0x12 if fan is controlled via
// fan_target or fan curve
//
pub const CTL_SET_FAN_FPWM: c_uint = 0x23	/*;
// set fixed pwm
// send: byte 1 is fan number
// send: byte 2 is percentage from 0 - 100
//
pub const CTL_SET_FAN_TARGET: c_uint = 0x24	/*;
// set target rpm
// send: byte 1 is fan number
// send: byte 2-3 is target
// device accepts all values from 0x00 - 0xFFFF
//
pub const NUM_FANS: c_int = 6;
pub const NUM_TEMP_SENSORS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_device {
    pub hdev: *mut hid_device,
    pub hwmon_dev: *mut device,
    pub debugfs: *mut dentry,
// For reinitializing the completion below
    pub wait_input_report_lock: spinlock_t,
    pub wait_input_report: completion,
    pub /: *mut *mut mutex mutex; / whenever buffer is used, lock before send_usb_cmd,
    pub cmd_buffer: *mut u8,
    pub buffer: *mut u8,
    pub /: *mut *mut int buffer_recv_size; / number of received bytes in buffer,
    pub target: [c_int; NUM_FANS],
    pub NUM_TEMP_SENSORS): DECLARE_BITMAP(temp_cnct,,
    pub NUM_FANS): DECLARE_BITMAP(fan_cnct,,
    pub fan_label: [c_char; NUM_FANS][LABEL_LENGTH],
    pub firmware_ver: [u8; 3],
    pub bootloader_ver: [u8; 2],
}

// converts response error in buffer to errno
#[no_mangle]
unsafe extern "C" fn ccp_get_errno(ccp: *mut ccp_device) -> c_int {
    static int ccp_get_errno(struct ccp_device *ccp)
    {
    switch (ccp.buffer[0]) {
    case 0x00: /* success */
    return 0;
    case 0x01: /* called invalid command */
    return -EOPNOTSUPP;
    case 0x10: /* called GET_VOLT / GET_TMP with invalid arguments */
    return -EINVAL;
    case 0x11: /* requested temps of disconnected sensors */
    case 0x12: /* requested pwm of not pwm controlled channels */
    return -ENODATA;
    default:
    hid_dbg(ccp.hdev, "unknown device response error: %d", ccp.buffer[0]);
    return -EIO;
    }
    }
// send command, check for error in response, response in ccp->buffer
#[no_mangle]
unsafe extern "C" fn send_usb_cmd(ccp: *mut ccp_device, command: u8, byte1: u8, byte2: u8, byte3: u8) -> c_int {
    static int send_usb_cmd(struct ccp_device *ccp, u8 command, u8 byte1, u8 byte2, u8 byte3)
    {
    unsigned long t;
    int ret;
    memset(ccp.cmd_buffer, 0x00, OUT_BUFFER_SIZE);
    ccp.cmd_buffer[0] = command;
    ccp.cmd_buffer[1] = byte1;
    ccp.cmd_buffer[2] = byte2;
    ccp.cmd_buffer[3] = byte3;
//
// Disable raw event parsing for a moment to safely reinitialize the
// completion. Reinit is done because hidraw could have triggered
// the raw event parsing and marked the ccp->wait_input_report
// completion as done.
//
    spin_lock_bh(&ccp.wait_input_report_lock);
    reinit_completion(&ccp.wait_input_report);
    spin_unlock_bh(&ccp.wait_input_report_lock);
    ret = hid_hw_output_report(ccp.hdev, ccp.cmd_buffer, OUT_BUFFER_SIZE);
    if (ret < 0)
    return ret;
    t = wait_for_completion_timeout(&ccp.wait_input_report, msecs_to_jiffies(REQ_TIMEOUT));
    if (!t)
    return -ETIMEDOUT;
    if (ccp.buffer_recv_size != IN_BUFFER_SIZE)
    return -EPROTO;
    return ccp_get_errno(ccp);
    }
#[no_mangle]
unsafe extern "C" fn ccp_raw_event(hdev: *mut hid_device, report: *mut hid_report, data: *mut u8, size: c_int) -> c_int {
    static int ccp_raw_event(struct hid_device *hdev, struct hid_report *report, u8 *data, int size)
    {
    struct ccp_device *ccp = hid_get_drvdata(hdev);
// only copy buffer when requested
    spin_lock(&ccp.wait_input_report_lock);
    if (!completion_done(&ccp.wait_input_report)) {
    memcpy(ccp.buffer, data, min(IN_BUFFER_SIZE, size));
    ccp.buffer_recv_size = size;
    complete_all(&ccp.wait_input_report);
    }
    spin_unlock(&ccp.wait_input_report_lock);
    return 0;
    }
// requests and returns single data values depending on channel
#[no_mangle]
unsafe extern "C" fn get_data(ccp: *mut ccp_device, command: c_int, channel: c_int, two_byte_data: bool) -> c_int {
    static int get_data(struct ccp_device *ccp, int command, int channel, bool two_byte_data)
    {
    int ret;
    mutex_lock(&ccp.mutex);
    ret = send_usb_cmd(ccp, command, channel, 0, 0);
    if (ret)
    goto out_unlock;
    ret = ccp.buffer[1];
    if (two_byte_data)
    ret = (ret << 8) + ccp.buffer[2];
    out_unlock:
    mutex_unlock(&ccp.mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn set_pwm(ccp: *mut ccp_device, channel: c_int, val: c_long) -> c_int {
    static int set_pwm(struct ccp_device *ccp, int channel, long val)
    {
    int ret;
    if (val < 0 || val > 255)
    return -EINVAL;
// The Corsair Commander Pro uses values from 0-100
    val = DIV_ROUND_CLOSEST(val * 100, 255);
    mutex_lock(&ccp.mutex);
    ret = send_usb_cmd(ccp, CTL_SET_FAN_FPWM, channel, val, 0);
    if (!ret)
    ccp.target[channel] = -ENODATA;
    mutex_unlock(&ccp.mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn set_target(ccp: *mut ccp_device, channel: c_int, val: c_long) -> c_int {
    static int set_target(struct ccp_device *ccp, int channel, long val)
    {
    int ret;
    val = clamp_val(val, 0, 0xFFFF);
    ccp.target[channel] = val;
    mutex_lock(&ccp.mutex);
    ret = send_usb_cmd(ccp, CTL_SET_FAN_TARGET, channel, val >> 8, val);
    mutex_unlock(&ccp.mutex);
    return ret;
    }
    static int ccp_read_string(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, const char **str)
    {
    struct ccp_device *ccp = dev_get_drvdata(dev);
    switch (type) {
    case hwmon_fan:
    switch (attr) {
    case hwmon_fan_label:
// str = ccp->fan_label[channel];
    return 0;
    default:
    break;
    }
    break;
    default:
    break;
    }
    return -EOPNOTSUPP;
    }
    static int ccp_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    struct ccp_device *ccp = dev_get_drvdata(dev);
    int ret;
    switch (type) {
    case hwmon_temp:
    switch (attr) {
    case hwmon_temp_input:
    ret = get_data(ccp, CTL_GET_TMP, channel, true);
    if (ret < 0)
    return ret;
// val = (s16)ret * 10;
    return 0;
    default:
    break;
    }
    break;
    case hwmon_fan:
    switch (attr) {
    case hwmon_fan_input:
    ret = get_data(ccp, CTL_GET_FAN_RPM, channel, true);
    if (ret < 0)
    return ret;
// val = ret;
    return 0;
    case hwmon_fan_target:
// how to read target values from the device is unknown
// driver returns last set value or 0
    if (ccp.target[channel] < 0)
    return -ENODATA;
// val = ccp->target[channel];
    return 0;
    default:
    break;
    }
    break;
    case hwmon_pwm:
    switch (attr) {
    case hwmon_pwm_input:
    ret = get_data(ccp, CTL_GET_FAN_PWM, channel, false);
    if (ret < 0)
    return ret;
// val = DIV_ROUND_CLOSEST(ret * 255, 100);
    return 0;
    default:
    break;
    }
    break;
    case hwmon_in:
    switch (attr) {
    case hwmon_in_input:
    ret = get_data(ccp, CTL_GET_VOLT, channel, true);
    if (ret < 0)
    return ret;
// val = ret;
    return 0;
    default:
    break;
    }
    break;
    default:
    break;
    }
    return -EOPNOTSUPP;
    };
    static int ccp_write(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long val)
    {
    struct ccp_device *ccp = dev_get_drvdata(dev);
    switch (type) {
    case hwmon_pwm:
    switch (attr) {
    case hwmon_pwm_input:
    return set_pwm(ccp, channel, val);
    default:
    break;
    }
    break;
    case hwmon_fan:
    switch (attr) {
    case hwmon_fan_target:
    return set_target(ccp, channel, val);
    default:
    break;
    }
    break;
    default:
    break;
    }
    return -EOPNOTSUPP;
    };
    static umode_t ccp_is_visible(const void *data, enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    const struct ccp_device *ccp = data;
    switch (type) {
    case hwmon_temp:
    if (!test_bit(channel, ccp.temp_cnct))
    break;
    switch (attr) {
    case hwmon_temp_input:
    return 0444;
    case hwmon_temp_label:
    return 0444;
    default:
    break;
    }
    break;
    case hwmon_fan:
    if (!test_bit(channel, ccp.fan_cnct))
    break;
    switch (attr) {
    case hwmon_fan_input:
    return 0444;
    case hwmon_fan_label:
    return 0444;
    case hwmon_fan_target:
    return 0644;
    default:
    break;
    }
    break;
    case hwmon_pwm:
    if (!test_bit(channel, ccp.fan_cnct))
    break;
    switch (attr) {
    case hwmon_pwm_input:
    return 0644;
    default:
    break;
    }
    break;
    case hwmon_in:
    switch (attr) {
    case hwmon_in_input:
    return 0444;
    default:
    break;
    }
    break;
    default:
    break;
    }
    return 0;
    };
    static const struct hwmon_ops ccp_hwmon_ops = {
    .is_visible = ccp_is_visible,
    .read = ccp_read,
    .read_string = ccp_read_string,
    .write = ccp_write,
    };
    static const struct hwmon_channel_info * const ccp_info[] = {
    HWMON_CHANNEL_INFO(chip,
    HWMON_C_REGISTER_TZ),
    HWMON_CHANNEL_INFO(temp,
    HWMON_T_INPUT,
    HWMON_T_INPUT,
    HWMON_T_INPUT,
    HWMON_T_INPUT
    ),
    HWMON_CHANNEL_INFO(fan,
    HWMON_F_INPUT | HWMON_F_LABEL | HWMON_F_TARGET,
    HWMON_F_INPUT | HWMON_F_LABEL | HWMON_F_TARGET,
    HWMON_F_INPUT | HWMON_F_LABEL | HWMON_F_TARGET,
    HWMON_F_INPUT | HWMON_F_LABEL | HWMON_F_TARGET,
    HWMON_F_INPUT | HWMON_F_LABEL | HWMON_F_TARGET,
    HWMON_F_INPUT | HWMON_F_LABEL | HWMON_F_TARGET
    ),
    HWMON_CHANNEL_INFO(pwm,
    HWMON_PWM_INPUT,
    HWMON_PWM_INPUT,
    HWMON_PWM_INPUT,
    HWMON_PWM_INPUT,
    HWMON_PWM_INPUT,
    HWMON_PWM_INPUT
    ),
    HWMON_CHANNEL_INFO(in,
    HWMON_I_INPUT,
    HWMON_I_INPUT,
    HWMON_I_INPUT
    ),
    core::ptr::null_mut()
    };
    static const struct hwmon_chip_info ccp_chip_info = {
    .ops = &ccp_hwmon_ops,
    .info = ccp_info,
    };
// read fan connection status and set labels
#[no_mangle]
unsafe extern "C" fn get_fan_cnct(ccp: *mut ccp_device) -> c_int {
    static int get_fan_cnct(struct ccp_device *ccp)
    {
    int channel;
    int mode;
    int ret;
    ret = send_usb_cmd(ccp, CTL_GET_FAN_CNCT, 0, 0, 0);
    if (ret)
    return ret;
    for (channel = 0; channel < NUM_FANS; channel++) {
    mode = ccp.buffer[channel + 1];
    if (mode == 0)
    continue;
    set_bit(channel, ccp.fan_cnct);
    ccp.target[channel] = -ENODATA;
    switch (mode) {
    case 1:
    scnprintf(ccp.fan_label[channel], LABEL_LENGTH,
    "fan%d 3pin", channel + 1);
    break;
    case 2:
    scnprintf(ccp.fan_label[channel], LABEL_LENGTH,
    "fan%d 4pin", channel + 1);
    break;
    default:
    scnprintf(ccp.fan_label[channel], LABEL_LENGTH,
    "fan%d other", channel + 1);
    break;
    }
    }
    return 0;
    }
// read temp sensor connection status
#[no_mangle]
unsafe extern "C" fn get_temp_cnct(ccp: *mut ccp_device) -> c_int {
    static int get_temp_cnct(struct ccp_device *ccp)
    {
    int channel;
    int mode;
    int ret;
    ret = send_usb_cmd(ccp, CTL_GET_TMP_CNCT, 0, 0, 0);
    if (ret)
    return ret;
    for (channel = 0; channel < NUM_TEMP_SENSORS; channel++) {
    mode = ccp.buffer[channel + 1];
    if (mode == 0)
    continue;
    set_bit(channel, ccp.temp_cnct);
    }
    return 0;
    }
// read firmware version
#[no_mangle]
unsafe extern "C" fn get_fw_version(ccp: *mut ccp_device) -> c_int {
    static int get_fw_version(struct ccp_device *ccp)
    {
    int ret;
    ret = send_usb_cmd(ccp, CTL_GET_FW_VER, 0, 0, 0);
    if (ret) {
    hid_notice(ccp.hdev, "Failed to read firmware version.\n");
    return ret;
    }
    ccp.firmware_ver[0] = ccp.buffer[1];
    ccp.firmware_ver[1] = ccp.buffer[2];
    ccp.firmware_ver[2] = ccp.buffer[3];
    return 0;
    }
// read bootloader version
#[no_mangle]
unsafe extern "C" fn get_bl_version(ccp: *mut ccp_device) -> c_int {
    static int get_bl_version(struct ccp_device *ccp)
    {
    int ret;
    ret = send_usb_cmd(ccp, CTL_GET_BL_VER, 0, 0, 0);
    if (ret) {
    hid_notice(ccp.hdev, "Failed to read bootloader version.\n");
    return ret;
    }
    ccp.bootloader_ver[0] = ccp.buffer[1];
    ccp.bootloader_ver[1] = ccp.buffer[2];
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn firmware_show(seqf: *mut seq_file, unused: *mut c_void) -> c_int {
    static int firmware_show(struct seq_file *seqf, void *unused)
    {
    struct ccp_device *ccp = seqf.private;
    seq_printf(seqf, "%d.%d.%d\n",
    ccp.firmware_ver[0],
    ccp.firmware_ver[1],
    ccp.firmware_ver[2]);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(firmware);
#[no_mangle]
unsafe extern "C" fn bootloader_show(seqf: *mut seq_file, unused: *mut c_void) -> c_int {
    static int bootloader_show(struct seq_file *seqf, void *unused)
    {
    struct ccp_device *ccp = seqf.private;
    seq_printf(seqf, "%d.%d\n",
    ccp.bootloader_ver[0],
    ccp.bootloader_ver[1]);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(bootloader);
#[no_mangle]
unsafe extern "C" fn ccp_debugfs_init(ccp: *mut ccp_device) {
    static void ccp_debugfs_init(struct ccp_device *ccp)
    {
    char name[32];
    int ret;
    scnprintf(name, sizeof(name), "corsaircpro-%s", dev_name(&ccp.hdev.dev));
    ccp.debugfs = debugfs_create_dir(name, core::ptr::null_mut());
    ret = get_fw_version(ccp);
    if (!ret)
    debugfs_create_file("firmware_version", 0444,
    ccp.debugfs, ccp, &firmware_fops);
    ret = get_bl_version(ccp);
    if (!ret)
    debugfs_create_file("bootloader_version", 0444,
    ccp.debugfs, ccp, &bootloader_fops);
    }
#[no_mangle]
unsafe extern "C" fn ccp_probe(hdev: *mut hid_device, id: *const hid_device_id) -> c_int {
    static int ccp_probe(struct hid_device *hdev, const struct hid_device_id *id)
    {
    struct ccp_device *ccp;
    int ret;
    ccp = devm_kzalloc(&hdev.dev, sizeof(*ccp), GFP_KERNEL);
    if (!ccp)
    return -ENOMEM;
    ccp.cmd_buffer = devm_kmalloc(&hdev.dev, OUT_BUFFER_SIZE, GFP_KERNEL);
    if (!ccp.cmd_buffer)
    return -ENOMEM;
    ccp.buffer = devm_kmalloc(&hdev.dev, IN_BUFFER_SIZE, GFP_KERNEL);
    if (!ccp.buffer)
    return -ENOMEM;
    ret = hid_parse(hdev);
    if (ret)
    return ret;
    ret = hid_hw_start(hdev, HID_CONNECT_HIDRAW);
    if (ret)
    return ret;
    ret = hid_hw_open(hdev);
    if (ret)
    goto out_hw_stop;
    ccp.hdev = hdev;
    hid_set_drvdata(hdev, ccp);
    mutex_init(&ccp.mutex);
    spin_lock_init(&ccp.wait_input_report_lock);
    init_completion(&ccp.wait_input_report);
    hid_device_io_start(hdev);
// temp and fan connection status only updates when device is powered on
    ret = get_temp_cnct(ccp);
    if (ret)
    goto out_hw_close;
    ret = get_fan_cnct(ccp);
    if (ret)
    goto out_hw_close;
    ccp_debugfs_init(ccp);
    ccp.hwmon_dev = hwmon_device_register_with_info(&hdev.dev, "corsaircpro",
    ccp, &ccp_chip_info, core::ptr::null_mut());
    if (IS_ERR(ccp.hwmon_dev)) {
    ret = PTR_ERR(ccp.hwmon_dev);
    goto out_hw_close;
    }
    return 0;
    out_hw_close:
    hid_hw_close(hdev);
    hid_device_io_stop(hdev);
    out_hw_stop:
    hid_hw_stop(hdev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ccp_remove(hdev: *mut hid_device) {
    static void ccp_remove(struct hid_device *hdev)
    {
    struct ccp_device *ccp = hid_get_drvdata(hdev);
    debugfs_remove_recursive(ccp.debugfs);
    hwmon_device_unregister(ccp.hwmon_dev);
    hid_hw_close(hdev);
    hid_hw_stop(hdev);
    }
    static const struct hid_device_id ccp_devices[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_CORSAIR, USB_PRODUCT_ID_CORSAIR_COMMANDERPRO) },
    { HID_USB_DEVICE(USB_VENDOR_ID_CORSAIR, USB_PRODUCT_ID_CORSAIR_1000D) },
    { }
    };
    static struct hid_driver ccp_driver = {
    .name = "corsair-cpro",
    .id_table = ccp_devices,
    .probe = ccp_probe,
    .remove = ccp_remove,
    .raw_event = ccp_raw_event,
    };
    MODULE_DEVICE_TABLE(hid, ccp_devices);
    MODULE_DESCRIPTION("Corsair Commander Pro controller driver");
    MODULE_LICENSE("GPL");
#[no_mangle]
unsafe extern "C" fn ccp_init() -> int __init {
    static int __init ccp_init(void)
    {
    return hid_register_driver(&ccp_driver);
    }
#[no_mangle]
unsafe extern "C" fn ccp_exit() -> void __exit {
    static void __exit ccp_exit(void)
    {
    hid_unregister_driver(&ccp_driver);
    }
//
// When compiling this driver as built-in, hwmon initcalls will get called before the
// hid driver and this driver would fail to register. late_initcall solves this.
//
    late_initcall(ccp_init);
    module_exit(ccp_exit);
