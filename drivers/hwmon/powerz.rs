//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/powerz.c
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
// Copyright (C) 2023 Thomas Weißschuh <linux@weissschuh.net>
//

pub const POWERZ_EP_CMD_OUT: c_uint = 0x01;
pub const POWERZ_EP_DATA_IN: c_uint = 0x81;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct powerz_sensor_data {
    pub _unknown_1: [u8; 8],
    pub V_bus: __le32,
    pub I_bus: __le32,
    pub V_bus_avg: __le32,
    pub I_bus_avg: __le32,
    pub _unknown_2: [u8; 8],
    pub temp: [u8; 2],
    pub V_cc1: __le16,
    pub V_cc2: __le16,
    pub V_dp: __le16,
    pub V_dm: __le16,
    pub V_dd: __le16,
    pub _unknown_3: [u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct powerz_priv {
    pub transfer_buffer: [c_char; 64],
    pub mutex: mutex,
    pub completion: completion,
    pub urb: *mut urb,
    pub status: c_int,
}

    static const struct hwmon_channel_info *const powerz_info[] = {
    HWMON_CHANNEL_INFO(in,
    HWMON_I_INPUT | HWMON_I_LABEL | HWMON_I_AVERAGE,
    HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL),
    HWMON_CHANNEL_INFO(curr,
    HWMON_C_INPUT | HWMON_C_LABEL | HWMON_C_AVERAGE),
    HWMON_CHANNEL_INFO(temp, HWMON_T_INPUT | HWMON_T_LABEL),
    core::ptr::null_mut()
    };
    static int powerz_read_string(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, const char **str)
    {
    if (type == hwmon_curr && attr == hwmon_curr_label) {
// str = "IBUS";
    } else if (type == hwmon_in && attr == hwmon_in_label) {
    if (channel == 0)
// str = "VBUS";
#[no_mangle]
pub unsafe extern "C" fn if(1: channel ==) -> else {
    else if (channel == 1)
// str = "VCC1";
#[no_mangle]
pub unsafe extern "C" fn if(2: channel ==) -> else {
    else if (channel == 2)
// str = "VCC2";
#[no_mangle]
pub unsafe extern "C" fn if(3: channel ==) -> else {
    else if (channel == 3)
// str = "VDP";
#[no_mangle]
pub unsafe extern "C" fn if(4: channel ==) -> else {
    else if (channel == 4)
// str = "VDM";
#[no_mangle]
pub unsafe extern "C" fn if(5: channel ==) -> else {
    else if (channel == 5)
// str = "VDD";
    else
    return -EOPNOTSUPP;
    } else if (type == hwmon_temp && attr == hwmon_temp_label) {
// str = "TEMP";
    } else {
    return -EOPNOTSUPP;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn powerz_usb_data_complete(urb: *mut urb) {
    static void powerz_usb_data_complete(struct urb *urb)
    {
    struct powerz_priv *priv = urb.context;
    complete(&priv.completion);
    }
#[no_mangle]
unsafe extern "C" fn powerz_usb_cmd_complete(urb: *mut urb) {
    static void powerz_usb_cmd_complete(struct urb *urb)
    {
    struct powerz_priv *priv = urb.context;
    usb_fill_bulk_urb(urb, urb.dev,
    usb_rcvbulkpipe(urb.dev, POWERZ_EP_DATA_IN),
    priv.transfer_buffer, sizeof(priv.transfer_buffer),
    powerz_usb_data_complete, priv);
    priv.status = usb_submit_urb(urb, GFP_ATOMIC);
    if (priv.status)
    complete(&priv.completion);
    }
#[no_mangle]
unsafe extern "C" fn powerz_read_data(udev: *mut usb_device, priv: *mut powerz_priv) -> c_int {
    static int powerz_read_data(struct usb_device *udev, struct powerz_priv *priv)
    {
    long rc;
    int ret;
    if (!priv.urb)
    return -ENODEV;
    priv.status = -ETIMEDOUT;
    reinit_completion(&priv.completion);
    priv.transfer_buffer[0] = 0x0c;
    priv.transfer_buffer[1] = 0x00;
    priv.transfer_buffer[2] = 0x02;
    priv.transfer_buffer[3] = 0x00;
    usb_fill_bulk_urb(priv.urb, udev,
    usb_sndbulkpipe(udev, POWERZ_EP_CMD_OUT),
    priv.transfer_buffer, 4, powerz_usb_cmd_complete,
    priv);
    ret = usb_submit_urb(priv.urb, GFP_KERNEL);
    if (ret)
    return ret;
    rc = wait_for_completion_interruptible_timeout(&priv.completion,
    msecs_to_jiffies(5));
    if (rc < 0) {
    usb_kill_urb(priv.urb);
    return rc;
    }
    if (rc == 0) {
    usb_kill_urb(priv.urb);
    return -EIO;
    }
    if (priv.urb.actual_length < sizeof(struct powerz_sensor_data))
    return -EIO;
    return priv.status;
    }
    static int powerz_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    struct usb_interface *intf = to_usb_interface(dev.parent);
    struct usb_device *udev = interface_to_usbdev(intf);
    struct powerz_priv *priv = usb_get_intfdata(intf);
    struct powerz_sensor_data *data;
    int ret;
    if (!priv)
    return -EIO;	/* disconnected */
    mutex_lock(&priv.mutex);
    ret = powerz_read_data(udev, priv);
    if (ret)
    goto out;
    data = (struct powerz_sensor_data *)priv.transfer_buffer;
    if (type == hwmon_curr) {
    if (attr == hwmon_curr_input)
// val = ((s32)le32_to_cpu(data->I_bus)) / 1000;
#[no_mangle]
pub unsafe extern "C" fn if(hwmon_curr_average: attr ==) -> else {
    else if (attr == hwmon_curr_average)
// val = ((s32)le32_to_cpu(data->I_bus_avg)) / 1000;
    else
    ret = -EOPNOTSUPP;
    } else if (type == hwmon_in) {
    if (attr == hwmon_in_input) {
    if (channel == 0)
// val = le32_to_cpu(data->V_bus) / 1000;
#[no_mangle]
pub unsafe extern "C" fn if(1: channel ==) -> else {
    else if (channel == 1)
// val = le16_to_cpu(data->V_cc1) / 10;
#[no_mangle]
pub unsafe extern "C" fn if(2: channel ==) -> else {
    else if (channel == 2)
// val = le16_to_cpu(data->V_cc2) / 10;
#[no_mangle]
pub unsafe extern "C" fn if(3: channel ==) -> else {
    else if (channel == 3)
// val = le16_to_cpu(data->V_dp) / 10;
#[no_mangle]
pub unsafe extern "C" fn if(4: channel ==) -> else {
    else if (channel == 4)
// val = le16_to_cpu(data->V_dm) / 10;
#[no_mangle]
pub unsafe extern "C" fn if(5: channel ==) -> else {
    else if (channel == 5)
// val = le16_to_cpu(data->V_dd) / 10;
    else
    ret = -EOPNOTSUPP;
    } else if (attr == hwmon_in_average && channel == 0) {
// val = le32_to_cpu(data->V_bus_avg) / 1000;
    } else {
    ret = -EOPNOTSUPP;
    }
    } else if (type == hwmon_temp && attr == hwmon_temp_input) {
// val = data->temp[1] * 2000 + data->temp[0] * 1000 / 128;
    } else {
    ret = -EOPNOTSUPP;
    }
    out:
    mutex_unlock(&priv.mutex);
    return ret;
    }
    static const struct hwmon_ops powerz_hwmon_ops = {
    .visible = 0444,
    .read = powerz_read,
    .read_string = powerz_read_string,
    };
    static const struct hwmon_chip_info powerz_chip_info = {
    .ops = &powerz_hwmon_ops,
    .info = powerz_info,
    };
    static int powerz_probe(struct usb_interface *intf,
    const struct usb_device_id *id)
    {
    struct powerz_priv *priv;
    struct device *hwmon_dev;
    struct device *parent;
    parent = &intf.dev;
    priv = devm_kzalloc(parent, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.urb = usb_alloc_urb(0, GFP_KERNEL);
    if (!priv.urb)
    return -ENOMEM;
    mutex_init(&priv.mutex);
    init_completion(&priv.completion);
    usb_set_intfdata(intf, priv);
    hwmon_dev =
    devm_hwmon_device_register_with_info(parent, DRIVER_NAME, priv,
    &powerz_chip_info, core::ptr::null_mut());
    if (IS_ERR(hwmon_dev)) {
    usb_free_urb(priv.urb);
    return PTR_ERR(hwmon_dev);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn powerz_disconnect(intf: *mut usb_interface) {
    static void powerz_disconnect(struct usb_interface *intf)
    {
    struct powerz_priv *priv = usb_get_intfdata(intf);
    mutex_lock(&priv.mutex);
    usb_kill_urb(priv.urb);
    usb_free_urb(priv.urb);
    priv.urb = core::ptr::null_mut();
    mutex_unlock(&priv.mutex);
    }
    static const struct usb_device_id powerz_id_table[] = {
    { USB_DEVICE_INTERFACE_NUMBER(0x5FC9, 0x0061, 0x00) },	/* ChargerLAB POWER-Z KM002C */
    { USB_DEVICE_INTERFACE_NUMBER(0x5FC9, 0x0063, 0x00) },	/* ChargerLAB POWER-Z KM003C */
    { }
    };
    MODULE_DEVICE_TABLE(usb, powerz_id_table);
    static struct usb_driver powerz_driver = {
    .name = DRIVER_NAME,
    .id_table = powerz_id_table,
    .probe = powerz_probe,
    .disconnect = powerz_disconnect,
    };
    module_usb_driver(powerz_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Thomas Weißschuh <linux@weissschuh.net>");
    MODULE_DESCRIPTION("ChargerLAB POWER-Z USB-C tester");
