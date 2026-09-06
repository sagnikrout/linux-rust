//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-roccat-konepure.c
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
// Roccat KonePure driver for Linux
//
// Copyright (c) 2012 Stefan Achatz <erazor_de@users.sourceforge.net>
//
// Roccat KonePure is a smaller version of KoneXTD with less buttons and lights.
//

    enum {
    KONEPURE_MOUSE_REPORT_NUMBER_BUTTON = 3,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct konepure_mouse_report_button {
    pub /: *mut *mut uint8_t report_number; / always KONEPURE_MOUSE_REPORT_NUMBER_BUTTON,
    pub zero: u8,
    pub type: u8,
    pub data1: u8,
    pub data2: u8,
    pub zero2: u8,
    pub unknown: [u8; 2],
    pub __packed: },
    pub 0x03): ROCCAT_COMMON2_BIN_ATTRIBUTE_W(control, 0x04,,
    pub 0x03): ROCCAT_COMMON2_BIN_ATTRIBUTE_RW(actual_profile, 0x05,,
    pub 0x1f): ROCCAT_COMMON2_BIN_ATTRIBUTE_RW(profile_settings, 0x06,,
    pub 0x3b): ROCCAT_COMMON2_BIN_ATTRIBUTE_RW(profile_buttons, 0x07,,
    pub 0x0822): ROCCAT_COMMON2_BIN_ATTRIBUTE_W(macro, 0x08,,
    pub 0x06): ROCCAT_COMMON2_BIN_ATTRIBUTE_RW(info, 0x09,,
    pub 0x04): ROCCAT_COMMON2_BIN_ATTRIBUTE_RW(tcu, 0x0c,,
    pub 0x0404): ROCCAT_COMMON2_BIN_ATTRIBUTE_R(tcu_image, 0x0c,,
    pub 0x06): ROCCAT_COMMON2_BIN_ATTRIBUTE_RW(sensor, 0x0f,,
    pub 0x10): ROCCAT_COMMON2_BIN_ATTRIBUTE_W(talk, 0x10,,
    static const struct bin_attribute *const konepure_bin_attrs[] = {
    &bin_attr_actual_profile,
    &bin_attr_control,
    &bin_attr_info,
    &bin_attr_talk,
    &bin_attr_macro,
    &bin_attr_sensor,
    &bin_attr_tcu,
    &bin_attr_tcu_image,
    &bin_attr_profile_settings,
    &bin_attr_profile_buttons,
    core::ptr::null_mut(),
}

    static const struct attribute_group konepure_group = {
    .bin_attrs = konepure_bin_attrs,
    };
    static const struct attribute_group *konepure_groups[] = {
    &konepure_group,
    core::ptr::null_mut(),
    };
    static const struct class konepure_class = {
    .name = "konepure",
    .dev_groups = konepure_groups,
    };
#[no_mangle]
unsafe extern "C" fn konepure_init_specials(hdev: *mut hid_device) -> c_int {
    static int konepure_init_specials(struct hid_device *hdev)
    {
    struct usb_interface *intf = to_usb_interface(hdev.dev.parent);
    struct usb_device *usb_dev = interface_to_usbdev(intf);
    struct roccat_common2_device *konepure;
    int retval;
    if (intf.cur_altsetting.desc.bInterfaceProtocol
    != USB_INTERFACE_PROTOCOL_MOUSE) {
    hid_set_drvdata(hdev, core::ptr::null_mut());
    return 0;
    }
    konepure = kzalloc_obj(*konepure);
    if (!konepure) {
    hid_err(hdev, "can't alloc device descriptor\n");
    return -ENOMEM;
    }
    hid_set_drvdata(hdev, konepure);
    retval = roccat_common2_device_init_struct(usb_dev, konepure);
    if (retval) {
    hid_err(hdev, "couldn't init KonePure device\n");
    goto exit_free;
    }
    retval = roccat_connect(&konepure_class, hdev,
    sizeof(struct konepure_mouse_report_button));
    if (retval < 0) {
    hid_err(hdev, "couldn't init char dev\n");
    } else {
    konepure.chrdev_minor = retval;
    konepure.roccat_claimed = 1;
    }
    return 0;
    exit_free:
    kfree(konepure);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn konepure_remove_specials(hdev: *mut hid_device) {
    static void konepure_remove_specials(struct hid_device *hdev)
    {
    struct usb_interface *intf = to_usb_interface(hdev.dev.parent);
    struct roccat_common2_device *konepure;
    if (intf.cur_altsetting.desc.bInterfaceProtocol
    != USB_INTERFACE_PROTOCOL_MOUSE)
    return;
    konepure = hid_get_drvdata(hdev);
    if (konepure.roccat_claimed)
    roccat_disconnect(konepure.chrdev_minor);
    kfree(konepure);
    }
    static int konepure_probe(struct hid_device *hdev,
    const struct hid_device_id *id)
    {
    int retval;
    if (!hid_is_usb(hdev))
    return -EINVAL;
    retval = hid_parse(hdev);
    if (retval) {
    hid_err(hdev, "parse failed\n");
    goto exit;
    }
    retval = hid_hw_start(hdev, HID_CONNECT_DEFAULT);
    if (retval) {
    hid_err(hdev, "hw start failed\n");
    goto exit;
    }
    retval = konepure_init_specials(hdev);
    if (retval) {
    hid_err(hdev, "couldn't install mouse\n");
    goto exit_stop;
    }
    return 0;
    exit_stop:
    hid_hw_stop(hdev);
    exit:
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn konepure_remove(hdev: *mut hid_device) {
    static void konepure_remove(struct hid_device *hdev)
    {
    konepure_remove_specials(hdev);
    hid_hw_stop(hdev);
    }
    static int konepure_raw_event(struct hid_device *hdev,
    struct hid_report *report, u8 *data, int size)
    {
    struct usb_interface *intf = to_usb_interface(hdev.dev.parent);
    struct roccat_common2_device *konepure = hid_get_drvdata(hdev);
    if (intf.cur_altsetting.desc.bInterfaceProtocol
    != USB_INTERFACE_PROTOCOL_MOUSE)
    return 0;
    if (data[0] != KONEPURE_MOUSE_REPORT_NUMBER_BUTTON)
    return 0;
    if (konepure != core::ptr::null_mut() && konepure.roccat_claimed)
    roccat_report_event(konepure.chrdev_minor, data);
    return 0;
    }
    static const struct hid_device_id konepure_devices[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_ROCCAT, USB_DEVICE_ID_ROCCAT_KONEPURE) },
    { HID_USB_DEVICE(USB_VENDOR_ID_ROCCAT, USB_DEVICE_ID_ROCCAT_KONEPURE_OPTICAL) },
    { }
    };
    MODULE_DEVICE_TABLE(hid, konepure_devices);
    static struct hid_driver konepure_driver = {
    .name = "konepure",
    .id_table = konepure_devices,
    .probe = konepure_probe,
    .remove = konepure_remove,
    .raw_event = konepure_raw_event
    };
#[no_mangle]
unsafe extern "C" fn konepure_init() -> int __init {
    static int __init konepure_init(void)
    {
    int retval;
    retval = class_register(&konepure_class);
    if (retval)
    return retval;
    retval = hid_register_driver(&konepure_driver);
    if (retval)
    class_unregister(&konepure_class);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn konepure_exit() -> void __exit {
    static void __exit konepure_exit(void)
    {
    hid_unregister_driver(&konepure_driver);
    class_unregister(&konepure_class);
    }
    module_init(konepure_init);
    module_exit(konepure_exit);
    MODULE_AUTHOR("Stefan Achatz");
    MODULE_DESCRIPTION("USB Roccat KonePure/Optical driver");
    MODULE_LICENSE("GPL v2");
