//! Automatically rewritten from C to Rust
//! Source: tools/usb/usbip/src/usbip_unbind.c
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

    static const char usbip_unbind_usage_string[] =
    "usbip unbind <args>\n"
    "    -b, --busid=<busid>    Unbind " USBIP_HOST_DRV_NAME ".ko from "
    "device on <busid>\n";
#[no_mangle]
pub unsafe extern "C" fn usbip_unbind_usage() {
    void usbip_unbind_usage(void)
    {
    printf("usage: %s", usbip_unbind_usage_string);
    }
#[no_mangle]
unsafe extern "C" fn unbind_device(busid: *mut c_char) -> c_int {
    static int unbind_device(char *busid)
    {
    char bus_type[] = "usb";
    int rc, ret = -1;
    char unbind_attr_name[] = "unbind";
    char unbind_attr_path[SYSFS_PATH_MAX];
    char rebind_attr_name[] = "rebind";
    char rebind_attr_path[SYSFS_PATH_MAX];
    struct udev *udev;
    struct udev_device *dev;
    const char *driver;
// Create libudev context.
    udev = udev_new();
// Check whether the device with this bus ID exists.
    dev = udev_device_new_from_subsystem_sysname(udev, "usb", busid);
    if (!dev) {
    err("device with the specified bus ID does not exist");
    goto err_close_udev;
    }
// Check whether the device is using usbip-host driver.
    driver = udev_device_get_driver(dev);
    if (!driver || strcmp(driver, "usbip-host")) {
    err("device is not bound to usbip-host driver");
    goto err_close_udev;
    }
// Unbind device from driver.
    snprintf(unbind_attr_path, sizeof(unbind_attr_path), "%s/%s/%s/%s/%s/%s",
    SYSFS_MNT_PATH, SYSFS_BUS_NAME, bus_type, SYSFS_DRIVERS_NAME,
    USBIP_HOST_DRV_NAME, unbind_attr_name);
    rc = write_sysfs_attribute(unbind_attr_path, busid, strlen(busid));
    if (rc < 0) {
    err("error unbinding device %s from driver", busid);
    goto err_close_udev;
    }
// Notify driver of unbind.
    rc = modify_match_busid(busid, 0);
    if (rc < 0) {
    err("unable to unbind device on %s", busid);
    goto err_close_udev;
    }
// Trigger new probing.
    snprintf(rebind_attr_path, sizeof(unbind_attr_path), "%s/%s/%s/%s/%s/%s",
    SYSFS_MNT_PATH, SYSFS_BUS_NAME, bus_type, SYSFS_DRIVERS_NAME,
    USBIP_HOST_DRV_NAME, rebind_attr_name);
    rc = write_sysfs_attribute(rebind_attr_path, busid, strlen(busid));
    if (rc < 0) {
    err("error rebinding");
    goto err_close_udev;
    }
    ret = 0;
    info("unbind device on busid %s: complete", busid);
    err_close_udev:
    udev_device_unref(dev);
    udev_unref(udev);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn usbip_unbind(argc: c_int, argv[]: *mut c_char) -> c_int {
    int usbip_unbind(int argc, char *argv[])
    {
    static const struct option opts[] = {
    { "busid", required_argument, core::ptr::null_mut(), 'b' },
    { core::ptr::null_mut(),    0,                 core::ptr::null_mut(),  0  }
    };
    int opt;
    let mut ret: c_int = -1;
    for (;;) {
    opt = getopt_long(argc, argv, "b:", opts, core::ptr::null_mut());
    if (opt == -1)
    break;
    switch (opt) {
    case 'b':
    ret = unbind_device(optarg);
    goto out;
    default:
    goto err_out;
    }
    }
    err_out:
    usbip_unbind_usage();
    out:
    return ret;
    }
