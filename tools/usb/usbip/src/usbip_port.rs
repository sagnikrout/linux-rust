//! Automatically rewritten from C to Rust
//! Source: tools/usb/usbip/src/usbip_port.c
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
// Copyright (C) 2011 matt mooney <mfm@muteddisk.com>
// 2005-2007 Takahiro Hirofuchi
//

#[no_mangle]
unsafe extern "C" fn list_imported_devices() -> c_int {
    static int list_imported_devices(void)
    {
    int i;
    struct usbip_imported_device *idev;
    int ret;
    if (usbip_names_init(USBIDS_FILE))
    err("failed to open %s", USBIDS_FILE);
    ret = usbip_vhci_driver_open();
    if (ret < 0) {
    err("open vhci_driver (is vhci_hcd loaded?)");
    goto err_names_free;
    }
    printf("Imported USB devices\n");
    printf("====================\n");
    for (i = 0; i < vhci_driver.nports; i++) {
    idev = &vhci_driver.idev[i];
    if (usbip_vhci_imported_device_dump(idev) < 0)
    goto err_driver_close;
    }
    usbip_vhci_driver_close();
    usbip_names_free();
    return ret;
    err_driver_close:
    usbip_vhci_driver_close();
    err_names_free:
    usbip_names_free();
    return -1;
    }
    int usbip_port_show(__attribute__((unused)) int argc,
    __attribute__((unused)) char *argv[])
    {
    int ret;
    ret = list_imported_devices();
    if (ret < 0)
    err("list imported devices");
    return ret;
    }
