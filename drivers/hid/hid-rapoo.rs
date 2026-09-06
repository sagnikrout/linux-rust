//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-rapoo.c
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

pub const RAPOO_BTN_BACK: c_uint = 0x08;
pub const RAPOO_BTN_FORWARD: c_uint = 0x10;
    static const struct hid_device_id rapoo_devices[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_RAPOO, USB_DEVICE_ID_RAPOO_2_4G_RECEIVER) },
    { }
    };
    MODULE_DEVICE_TABLE(hid, rapoo_devices);
#[no_mangle]
unsafe extern "C" fn rapoo_probe(hdev: *mut hid_device, id: *const hid_device_id) -> c_int {
    static int rapoo_probe(struct hid_device *hdev, const struct hid_device_id *id)
    {
    int ret;
    struct input_dev *input;
    ret = hid_parse(hdev);
    if (ret) {
    hid_err(hdev, "parse failed\n");
    return ret;
    }
    ret = hid_hw_start(hdev, HID_CONNECT_DEFAULT);
    if (ret) {
    hid_err(hdev, "start failed\n");
    return ret;
    }
    if (hid_is_usb(hdev)) {
    struct usb_interface *intf = to_usb_interface(hdev.dev.parent);
    if (intf.cur_altsetting.desc.bInterfaceNumber != 1)
    return 0;
    }
    input = devm_input_allocate_device(&hdev.dev);
    if (!input)
    return -ENOMEM;
    input.name = "Rapoo 2.4G Wireless Mouse";
    input.phys = "rapoo/input1";
    input.id.bustype = hdev.bus;
    input.id.vendor = hdev.vendor;
    input.id.product = hdev.product;
    input.id.version = hdev.version;
    __set_bit(EV_KEY, input.evbit);
    __set_bit(KEY_BACK, input.keybit);
    __set_bit(KEY_FORWARD, input.keybit);
    ret = input_register_device(input);
    if (ret)
    return ret;
    hid_set_drvdata(hdev, input);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rapoo_raw_event(hdev: *mut hid_device, report: *mut hid_report, data: *mut u8, size: c_int) -> c_int {
    static int rapoo_raw_event(struct hid_device *hdev, struct hid_report *report, u8 *data, int size)
    {
    struct input_dev *input = hid_get_drvdata(hdev);
    if (!input)
    return 0;
    if (report.id == 1 && size >= 2) {
    let mut btn: u8 = data[1];
    input_report_key(input, KEY_BACK, btn & RAPOO_BTN_BACK);
    input_report_key(input, KEY_FORWARD, btn & RAPOO_BTN_FORWARD);
    input_sync(input);
    return 1;
    }
    return 0;
    }
    static struct hid_driver rapoo_driver = {
    .name = "hid-rapoo",
    .id_table = rapoo_devices,
    .probe = rapoo_probe,
    .raw_event = rapoo_raw_event,
    };
    module_hid_driver(rapoo_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Nguyen Dinh Dang Duong <dangduong31205@gmail.com>");
    MODULE_DESCRIPTION("RAPOO 2.4G Wireless Device Driver");
