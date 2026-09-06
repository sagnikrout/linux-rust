//! Automatically rewritten from C to Rust
//! Source: tools/usb/usbip/src/usbip_bind.c
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

    enum unbind_status {
    UNBIND_ST_OK,
    UNBIND_ST_USBIP_HOST,
    UNBIND_ST_FAILED
    };
    static const char usbip_bind_usage_string[] =
    "usbip bind <args>\n"
    "    -b, --busid=<busid>    Bind " USBIP_HOST_DRV_NAME ".ko to device "
    "on <busid>\n";
#[no_mangle]
pub unsafe extern "C" fn usbip_bind_usage() {
    void usbip_bind_usage(void)
    {
    printf("usage: %s", usbip_bind_usage_string);
    }
// call at unbound state
#[no_mangle]
unsafe extern "C" fn bind_usbip(busid: *mut c_char) -> c_int {
    static int bind_usbip(char *busid)
    {
    char attr_name[] = "bind";
    char bind_attr_path[SYSFS_PATH_MAX];
    let mut rc: c_int = -1;
    snprintf(bind_attr_path, sizeof(bind_attr_path), "%s/%s/%s/%s/%s/%s",
    SYSFS_MNT_PATH, SYSFS_BUS_NAME, SYSFS_BUS_TYPE,
    SYSFS_DRIVERS_NAME, USBIP_HOST_DRV_NAME, attr_name);
    rc = write_sysfs_attribute(bind_attr_path, busid, strlen(busid));
    if (rc < 0) {
    err("error binding device %s to driver: %s", busid,
    strerror(errno));
    return -1;
    }
    return 0;
    }
// buggy driver may cause dead lock
#[no_mangle]
unsafe extern "C" fn unbind_other(busid: *mut c_char) -> c_int {
    static int unbind_other(char *busid)
    {
    let mut status: enum unbind_status = UNBIND_ST_OK;
    char attr_name[] = "unbind";
    char unbind_attr_path[SYSFS_PATH_MAX];
    let mut rc: c_int = -1;
    struct udev *udev;
    struct udev_device *dev;
    const char *driver;
    const char *bDevClass;
// Create libudev context.
    udev = udev_new();
// Get the device.
    dev = udev_device_new_from_subsystem_sysname(udev, "usb", busid);
    if (!dev) {
    dbg("unable to find device with bus ID %s", busid);
    goto err_close_busid_dev;
    }
// Check what kind of device it is.
    bDevClass  = udev_device_get_sysattr_value(dev, "bDeviceClass");
    if (!bDevClass) {
    dbg("unable to get bDevClass device attribute");
    goto err_close_busid_dev;
    }
    if (!strncmp(bDevClass, "09", strlen(bDevClass))) {
    dbg("skip unbinding of hub");
    goto err_close_busid_dev;
    }
// Get the device driver.
    driver = udev_device_get_driver(dev);
    if (!driver) {
// No driver bound to this device.
    goto out;
    }
    if (!strncmp(USBIP_HOST_DRV_NAME, driver,
    strlen(USBIP_HOST_DRV_NAME))) {
// Already bound to usbip-host.
    status = UNBIND_ST_USBIP_HOST;
    goto out;
    }
// Unbind device from driver.
    snprintf(unbind_attr_path, sizeof(unbind_attr_path), "%s/%s/%s/%s/%s/%s",
    SYSFS_MNT_PATH, SYSFS_BUS_NAME, SYSFS_BUS_TYPE,
    SYSFS_DRIVERS_NAME, driver, attr_name);
    rc = write_sysfs_attribute(unbind_attr_path, busid, strlen(busid));
    if (rc < 0) {
    err("error unbinding device %s from driver", busid);
    goto err_close_busid_dev;
    }
    goto out;
    err_close_busid_dev:
    status = UNBIND_ST_FAILED;
    out:
    udev_device_unref(dev);
    udev_unref(udev);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn bind_device(busid: *mut c_char) -> c_int {
    static int bind_device(char *busid)
    {
    int rc;
    struct udev *udev;
    struct udev_device *dev;
    const char *devpath;
// Check whether the device with this bus ID exists.
    udev = udev_new();
    dev = udev_device_new_from_subsystem_sysname(udev, "usb", busid);
    if (!dev) {
    err("device with the specified bus ID does not exist");
    return -1;
    }
    devpath = udev_device_get_devpath(dev);
    udev_unref(udev);
// If the device is already attached to vhci_hcd - bail out
    if (strstr(devpath, USBIP_VHCI_DRV_NAME)) {
    err("bind loop detected: device: %s is attached to %s\n",
    devpath, USBIP_VHCI_DRV_NAME);
    return -1;
    }
    rc = unbind_other(busid);
    if (rc == UNBIND_ST_FAILED) {
    err("could not unbind driver from device on busid %s", busid);
    return -1;
    } else if (rc == UNBIND_ST_USBIP_HOST) {
    err("device on busid %s is already bound to %s", busid,
    USBIP_HOST_DRV_NAME);
    return -1;
    }
    rc = modify_match_busid(busid, 1);
    if (rc < 0) {
    err("unable to bind device on %s", busid);
    return -1;
    }
    rc = bind_usbip(busid);
    if (rc < 0) {
    err("could not bind device to %s", USBIP_HOST_DRV_NAME);
    modify_match_busid(busid, 0);
    return -1;
    }
    info("bind device on busid %s: complete", busid);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn usbip_bind(argc: c_int, argv[]: *mut c_char) -> c_int {
    int usbip_bind(int argc, char *argv[])
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
    ret = bind_device(optarg);
    goto out;
    default:
    goto err_out;
    }
    }
    err_out:
    usbip_bind_usage();
    out:
    return ret;
    }
