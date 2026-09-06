//! Automatically rewritten from C to Rust
//! Source: drivers/usb/misc/ehset.c
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
// Copyright (c) 2010-2013, The Linux Foundation. All rights reserved.
//

pub const TEST_SE0_NAK_PID: c_uint = 0x0101;
pub const TEST_J_PID: c_uint = 0x0102;
pub const TEST_K_PID: c_uint = 0x0103;
pub const TEST_PACKET_PID: c_uint = 0x0104;
pub const TEST_HS_HOST_PORT_SUSPEND_RESUME: c_uint = 0x0106;
pub const TEST_SINGLE_STEP_GET_DEV_DESC: c_uint = 0x0107;
pub const TEST_SINGLE_STEP_SET_FEATURE: c_uint = 0x0108;
    extern const struct usb_device_id *usb_device_match_id(struct usb_device *udev,
    const struct usb_device_id *id);
//
// A list of USB hubs which requires to disable the power
// to the port before starting the testing procedures.
//
    static const struct usb_device_id ehset_hub_list[] = {
    { USB_DEVICE(0x0424, 0x4502) },
    { USB_DEVICE(0x0424, 0x4913) },
    { USB_DEVICE(0x0451, 0x8027) },
    { }
    };
#[no_mangle]
unsafe extern "C" fn ehset_prepare_port_for_testing(hub_udev: *mut usb_device, portnum: u16) -> c_int {
    static int ehset_prepare_port_for_testing(struct usb_device *hub_udev, u16 portnum)
    {
    let mut ret: c_int = 0;
//
// The USB2.0 spec chapter 11.24.2.13 says that the USB port which is
// going under test needs to be put in suspend before sending the
// test command. Most hubs don't enforce this precondition, but there
// are some hubs which needs to disable the power to the port before
// starting the test.
//
    if (usb_device_match_id(hub_udev, ehset_hub_list)) {
    ret = usb_control_msg_send(hub_udev, 0, USB_REQ_CLEAR_FEATURE,
    USB_RT_PORT, USB_PORT_FEAT_ENABLE,
    portnum, core::ptr::null_mut(), 0, 1000, GFP_KERNEL);
//
// Wait for the port to be disabled. It's an arbitrary value
// which worked every time.
//
    msleep(100);
    } else {
//
// For the hubs which are compliant with the spec,
// put the port in SUSPEND.
//
    ret = usb_control_msg_send(hub_udev, 0, USB_REQ_SET_FEATURE,
    USB_RT_PORT, USB_PORT_FEAT_SUSPEND,
    portnum, core::ptr::null_mut(), 0, 1000, GFP_KERNEL);
    }
    return ret;
    }
    static int ehset_probe(struct usb_interface *intf,
    const struct usb_device_id *id)
    {
    let mut ret: c_int = -EINVAL;
    struct usb_device *dev = interface_to_usbdev(intf);
    struct usb_device *hub_udev = dev.parent;
    struct usb_device_descriptor buf;
    let mut portnum: u8 = dev.portnum;
    let mut test_pid: u16 = le16_to_cpu(dev.descriptor.idProduct);
    switch (test_pid) {
    case TEST_SE0_NAK_PID:
    ret = ehset_prepare_port_for_testing(hub_udev, portnum);
    if (ret < 0)
    break;
    ret = usb_control_msg_send(hub_udev, 0, USB_REQ_SET_FEATURE,
    USB_RT_PORT, USB_PORT_FEAT_TEST,
    (USB_TEST_SE0_NAK << 8) | portnum,
    core::ptr::null_mut(), 0, 1000, GFP_KERNEL);
    break;
    case TEST_J_PID:
    ret = ehset_prepare_port_for_testing(hub_udev, portnum);
    if (ret < 0)
    break;
    ret = usb_control_msg_send(hub_udev, 0, USB_REQ_SET_FEATURE,
    USB_RT_PORT, USB_PORT_FEAT_TEST,
    (USB_TEST_J << 8) | portnum, core::ptr::null_mut(), 0,
    1000, GFP_KERNEL);
    break;
    case TEST_K_PID:
    ret = ehset_prepare_port_for_testing(hub_udev, portnum);
    if (ret < 0)
    break;
    ret = usb_control_msg_send(hub_udev, 0, USB_REQ_SET_FEATURE,
    USB_RT_PORT, USB_PORT_FEAT_TEST,
    (USB_TEST_K << 8) | portnum, core::ptr::null_mut(), 0,
    1000, GFP_KERNEL);
    break;
    case TEST_PACKET_PID:
    ret = ehset_prepare_port_for_testing(hub_udev, portnum);
    if (ret < 0)
    break;
    ret = usb_control_msg_send(hub_udev, 0, USB_REQ_SET_FEATURE,
    USB_RT_PORT, USB_PORT_FEAT_TEST,
    (USB_TEST_PACKET << 8) | portnum,
    core::ptr::null_mut(), 0, 1000, GFP_KERNEL);
    break;
    case TEST_HS_HOST_PORT_SUSPEND_RESUME:
// Test: wait for 15secs -> suspend -> 15secs delay -> resume
    msleep(15 * 1000);
    ret = usb_control_msg_send(hub_udev, 0, USB_REQ_SET_FEATURE,
    USB_RT_PORT, USB_PORT_FEAT_SUSPEND,
    portnum, core::ptr::null_mut(), 0, 1000, GFP_KERNEL);
    if (ret < 0)
    break;
    msleep(15 * 1000);
    ret = usb_control_msg_send(hub_udev, 0, USB_REQ_CLEAR_FEATURE,
    USB_RT_PORT, USB_PORT_FEAT_SUSPEND,
    portnum, core::ptr::null_mut(), 0, 1000, GFP_KERNEL);
    break;
    case TEST_SINGLE_STEP_GET_DEV_DESC:
// Test: wait for 15secs -> GetDescriptor request
    msleep(15 * 1000);
    ret = usb_control_msg_recv(dev, 0, USB_REQ_GET_DESCRIPTOR,
    USB_DIR_IN, USB_DT_DEVICE << 8, 0,
    &buf, USB_DT_DEVICE_SIZE,
    USB_CTRL_GET_TIMEOUT, GFP_KERNEL);
    break;
    case TEST_SINGLE_STEP_SET_FEATURE:
//
// GetDescriptor SETUP request -> 15secs delay -> IN & STATUS
//
// Note, this test is only supported on root hubs since the
// SetPortFeature handling can only be done inside the HCD's
// hub_control callback function.
//
    if (hub_udev != dev.bus.root_hub) {
    dev_err(&intf.dev, "SINGLE_STEP_SET_FEATURE test only supported on root hub\n");
    break;
    }
    ret = usb_control_msg_send(hub_udev, 0, USB_REQ_SET_FEATURE,
    USB_RT_PORT, USB_PORT_FEAT_TEST,
    (6 << 8) | portnum, core::ptr::null_mut(), 0,
    60 * 1000, GFP_KERNEL);
    break;
    default:
    dev_err(&intf.dev, "%s: unsupported PID: 0x%x\n",
    __func__, test_pid);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ehset_disconnect(intf: *mut usb_interface) {
    static void ehset_disconnect(struct usb_interface *intf)
    {
    }
    static const struct usb_device_id ehset_id_table[] = {
    { USB_DEVICE(0x1a0a, TEST_SE0_NAK_PID) },
    { USB_DEVICE(0x1a0a, TEST_J_PID) },
    { USB_DEVICE(0x1a0a, TEST_K_PID) },
    { USB_DEVICE(0x1a0a, TEST_PACKET_PID) },
    { USB_DEVICE(0x1a0a, TEST_HS_HOST_PORT_SUSPEND_RESUME) },
    { USB_DEVICE(0x1a0a, TEST_SINGLE_STEP_GET_DEV_DESC) },
    { USB_DEVICE(0x1a0a, TEST_SINGLE_STEP_SET_FEATURE) },
    { }			/* Terminating entry */
    };
    MODULE_DEVICE_TABLE(usb, ehset_id_table);
    static struct usb_driver ehset_driver = {
    .name =		"usb_ehset_test",
    .probe =	ehset_probe,
    .disconnect =	ehset_disconnect,
    .id_table =	ehset_id_table,
    };
    module_usb_driver(ehset_driver);
    MODULE_DESCRIPTION("USB Driver for EHSET Test Fixture");
    MODULE_LICENSE("GPL v2");
