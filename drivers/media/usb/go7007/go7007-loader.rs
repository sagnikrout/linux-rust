//! Automatically rewritten from C to Rust
//! Source: drivers/media/usb/go7007/go7007-loader.c
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
// Copyright (C) 2008 Sensoray Company Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_config {
    pub vendor: u16,
    pub product: u16,
    pub fw_name1: *const *const c_char,
    pub fw_name2: *const *const c_char,
}

    static struct fw_config fw_configs[] = {
    { 0x1943, 0xa250, "go7007/s2250-1.fw", "go7007/s2250-2.fw" },
    { 0x093b, 0xa002, "go7007/px-m402u.fw", core::ptr::null_mut() },
    { 0x093b, 0xa004, "go7007/px-tv402u.fw", core::ptr::null_mut() },
    { 0x0eb1, 0x6666, "go7007/lr192.fw", core::ptr::null_mut() },
    { 0x0eb1, 0x6668, "go7007/wis-startrek.fw", core::ptr::null_mut() },
    { 0, 0, core::ptr::null_mut(), core::ptr::null_mut() }
    };
    MODULE_FIRMWARE("go7007/s2250-1.fw");
    MODULE_FIRMWARE("go7007/s2250-2.fw");
    MODULE_FIRMWARE("go7007/px-m402u.fw");
    MODULE_FIRMWARE("go7007/px-tv402u.fw");
    MODULE_FIRMWARE("go7007/lr192.fw");
    MODULE_FIRMWARE("go7007/wis-startrek.fw");
    static int go7007_loader_probe(struct usb_interface *interface,
    const struct usb_device_id *id)
    {
    struct usb_device *usbdev;
    const struct firmware *fw;
    u16 vendor, product;
    const char *fw1, *fw2;
    int ret;
    int i;
    usbdev = interface_to_usbdev(interface);
    if (usbdev.descriptor.bNumConfigurations != 1) {
    dev_err(&interface.dev, "can't handle multiple config\n");
    goto failed2;
    }
    vendor = le16_to_cpu(usbdev.descriptor.idVendor);
    product = le16_to_cpu(usbdev.descriptor.idProduct);
    for (i = 0; fw_configs[i].fw_name1; i++)
    if (fw_configs[i].vendor == vendor &&
    fw_configs[i].product == product)
    break;
// Should never happen
    if (fw_configs[i].fw_name1 == core::ptr::null_mut())
    goto failed2;
    fw1 = fw_configs[i].fw_name1;
    fw2 = fw_configs[i].fw_name2;
    dev_info(&interface.dev, "loading firmware %s\n", fw1);
    if (request_firmware(&fw, fw1, &usbdev.dev)) {
    dev_err(&interface.dev,
    "unable to load firmware from file \"%s\"\n", fw1);
    goto failed2;
    }
    ret = cypress_load_firmware(usbdev, fw, CYPRESS_FX2);
    release_firmware(fw);
    if (0 != ret) {
    dev_err(&interface.dev, "loader download failed\n");
    goto failed2;
    }
    if (fw2 == core::ptr::null_mut())
    return 0;
    if (request_firmware(&fw, fw2, &usbdev.dev)) {
    dev_err(&interface.dev,
    "unable to load firmware from file \"%s\"\n", fw2);
    goto failed2;
    }
    ret = cypress_load_firmware(usbdev, fw, CYPRESS_FX2);
    release_firmware(fw);
    if (0 != ret) {
    dev_err(&interface.dev, "firmware download failed\n");
    goto failed2;
    }
    return 0;
    failed2:
    dev_err(&interface.dev, "probe failed\n");
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn go7007_loader_disconnect(interface: *mut usb_interface) {
    static void go7007_loader_disconnect(struct usb_interface *interface)
    {
    dev_info(&interface.dev, "disconnect\n");
    usb_set_intfdata(interface, core::ptr::null_mut());
    }
    static const struct usb_device_id go7007_loader_ids[] = {
    { USB_DEVICE(0x1943, 0xa250) },
    { USB_DEVICE(0x093b, 0xa002) },
    { USB_DEVICE(0x093b, 0xa004) },
    { USB_DEVICE(0x0eb1, 0x6666) },
    { USB_DEVICE(0x0eb1, 0x6668) },
    {}                          /* Terminating entry */
    };
    MODULE_DEVICE_TABLE(usb, go7007_loader_ids);
    static struct usb_driver go7007_loader_driver = {
    .name		= "go7007-loader",
    .probe		= go7007_loader_probe,
    .disconnect	= go7007_loader_disconnect,
    .id_table	= go7007_loader_ids,
    };
    module_usb_driver(go7007_loader_driver);
    MODULE_AUTHOR("");
    MODULE_DESCRIPTION("firmware loader for go7007-usb");
    MODULE_LICENSE("GPL v2");
