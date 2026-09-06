//! Automatically rewritten from C to Rust
//! Source: drivers/platform/chrome/wilco_ec/sysfs.c
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
//
// Copyright 2019 Google LLC
//
// Sysfs properties to view and modify EC-controlled features on Wilco devices.
// The entries will appear under /sys/bus/platform/devices/GOOG000C:00
//
// See Documentation/ABI/testing/sysfs-platform-wilco-ec for more information.
//

pub const CMD_KB_CMOS: c_uint = 0x7C;
pub const SUB_CMD_KB_CMOS_AUTO_ON: c_uint = 0x03;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct boot_on_ac_request {
    pub /: *mut *mut u8 cmd; / Always CMD_KB_CMOS,
    pub reserved1: u8,
    pub /: *mut *mut u8 sub_cmd; / Always SUB_CMD_KB_CMOS_AUTO_ON,
    pub reserved3to5: [u8; 3],
    pub /: *mut *mut u8 val; / Either 0 or 1,
    pub reserved7: u8,
    pub __packed: },
pub const CMD_USB_CHARGE: c_uint = 0x39;
    enum usb_charge_op {
    USB_CHARGE_GET = 0,
    USB_CHARGE_SET = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_charge_request {
    pub /: *mut *mut u8 cmd; / Always CMD_USB_CHARGE,
    pub reserved: u8,
    pub /: *mut *mut u8 op; / One of enum usb_charge_op,
    pub /: *mut *mut u8 val; / When setting, either 0 or 1,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_charge_response {
    pub reserved: u8,
    pub /: *mut *mut u8 status; / Set by EC to 0 on success, other value on failure,
    pub /: *mut *mut u8 val; / When getting, set by EC to either 0 or 1,
    pub __packed: },
pub const CMD_EC_INFO: c_uint = 0x38;
    enum get_ec_info_op {
    CMD_GET_EC_LABEL	= 0,
    CMD_GET_EC_REV		= 1,
    CMD_GET_EC_MODEL	= 2,
    CMD_GET_EC_BUILD_DATE	= 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_ec_info_req {
    pub /: *mut *mut u8 cmd; / Always CMD_EC_INFO,
    pub reserved: u8,
    pub /: *mut *mut u8 op; / One of enum get_ec_info_op,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_ec_info_resp {
    pub reserved: [u8; 2],
    pub /: *mut *mut char value[9]; / __nonstring: might not be null terminated,
    pub __packed: },
    static ssize_t boot_on_ac_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    pub dev_get_drvdata(dev): *mut *mut wilco_ec_device ec =,
    pub rq: boot_on_ac_request,
    pub msg: wilco_ec_message,
    pub ret: c_int,
    pub val: u8,
    pub &val): ret = kstrtou8(buf, 10,,
    if (ret < 0)
    pub ret: return,
    if (val > 1)
    pub -EINVAL: return,
    pub sizeof(rq)): memset(&rq, 0,,
    pub CMD_KB_CMOS: rq.cmd =,
    pub SUB_CMD_KB_CMOS_AUTO_ON: rq.sub_cmd =,
    pub val: rq.val =,
    pub sizeof(msg)): memset(&msg, 0,,
    pub WILCO_EC_MSG_LEGACY: msg.type =,
    pub &rq: msg.request_data =,
    pub sizeof(rq): msg.request_size =,
    pub &msg): ret = wilco_ec_mailbox(ec,,
    if (ret < 0)
    pub ret: return,
    pub count: return,
    }
    pub DEVICE_ATTR_WO(boot_on_ac): static,
#[no_mangle]
unsafe extern "C" fn get_info(dev: *mut device, buf: *mut c_char, op: enum get_ec_info_op) -> isize {
    static ssize_t get_info(struct device *dev, char *buf, enum get_ec_info_op op)
    {
    pub dev_get_drvdata(dev): *mut *mut wilco_ec_device ec =,
    pub }: get_ec_info_req req = { .cmd = CMD_EC_INFO, .op = op,
    pub resp: get_ec_info_resp,
    pub ret: c_int,
    struct wilco_ec_message msg = {
    .type = WILCO_EC_MSG_LEGACY,
    .request_data = &req,
    .request_size = sizeof(req),
    .response_data = &resp,
    .response_size = sizeof(resp),
}

    ret = wilco_ec_mailbox(ec, &msg);
    if (ret < 0)
    return ret;
    return sysfs_emit(buf, "%.*s\n", (int)sizeof(resp.value), (char *)&resp.value);
    }
    static ssize_t version_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    return get_info(dev, buf, CMD_GET_EC_LABEL);
    }
    static DEVICE_ATTR_RO(version);
    static ssize_t build_revision_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    return get_info(dev, buf, CMD_GET_EC_REV);
    }
    static DEVICE_ATTR_RO(build_revision);
    static ssize_t build_date_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    return get_info(dev, buf, CMD_GET_EC_BUILD_DATE);
    }
    static DEVICE_ATTR_RO(build_date);
    static ssize_t model_number_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    return get_info(dev, buf, CMD_GET_EC_MODEL);
    }
    static DEVICE_ATTR_RO(model_number);
    static int send_usb_charge(struct wilco_ec_device *ec,
    struct usb_charge_request *rq,
    struct usb_charge_response *rs)
    {
    struct wilco_ec_message msg;
    int ret;
    memset(&msg, 0, sizeof(msg));
    msg.type = WILCO_EC_MSG_LEGACY;
    msg.request_data = rq;
    msg.request_size = sizeof(*rq);
    msg.response_data = rs;
    msg.response_size = sizeof(*rs);
    ret = wilco_ec_mailbox(ec, &msg);
    if (ret < 0)
    return ret;
    if (rs.status)
    return -EIO;
    return 0;
    }
    static ssize_t usb_charge_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct wilco_ec_device *ec = dev_get_drvdata(dev);
    struct usb_charge_request rq;
    struct usb_charge_response rs;
    int ret;
    memset(&rq, 0, sizeof(rq));
    rq.cmd = CMD_USB_CHARGE;
    rq.op = USB_CHARGE_GET;
    ret = send_usb_charge(ec, &rq, &rs);
    if (ret < 0)
    return ret;
    return sysfs_emit(buf, "%d\n", rs.val);
    }
    static ssize_t usb_charge_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct wilco_ec_device *ec = dev_get_drvdata(dev);
    struct usb_charge_request rq;
    struct usb_charge_response rs;
    int ret;
    u8 val;
    ret = kstrtou8(buf, 10, &val);
    if (ret < 0)
    return ret;
    if (val > 1)
    return -EINVAL;
    memset(&rq, 0, sizeof(rq));
    rq.cmd = CMD_USB_CHARGE;
    rq.op = USB_CHARGE_SET;
    rq.val = val;
    ret = send_usb_charge(ec, &rq, &rs);
    if (ret < 0)
    return ret;
    return count;
    }
    static DEVICE_ATTR_RW(usb_charge);
    static struct attribute *wilco_dev_attrs[] = {
    &dev_attr_boot_on_ac.attr,
    &dev_attr_build_date.attr,
    &dev_attr_build_revision.attr,
    &dev_attr_model_number.attr,
    &dev_attr_usb_charge.attr,
    &dev_attr_version.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group wilco_dev_attr_group = {
    .attrs = wilco_dev_attrs,
    };
#[no_mangle]
pub unsafe extern "C" fn wilco_ec_add_sysfs(ec: *mut wilco_ec_device) -> c_int {
    int wilco_ec_add_sysfs(struct wilco_ec_device *ec)
    {
    return sysfs_create_group(&ec.dev.kobj, &wilco_dev_attr_group);
    }
#[no_mangle]
pub unsafe extern "C" fn wilco_ec_remove_sysfs(ec: *mut wilco_ec_device) {
    void wilco_ec_remove_sysfs(struct wilco_ec_device *ec)
    {
    sysfs_remove_group(&ec.dev.kobj, &wilco_dev_attr_group);
    }
