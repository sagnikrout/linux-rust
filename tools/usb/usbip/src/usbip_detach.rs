//! Automatically rewritten from C to Rust
//! Source: tools/usb/usbip/src/usbip_detach.c
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

    static const char usbip_detach_usage_string[] =
    "usbip detach <args>\n"
    "    -p, --port=<port>    " USBIP_VHCI_DRV_NAME
    " port the device is on\n";
#[no_mangle]
pub unsafe extern "C" fn usbip_detach_usage() {
    void usbip_detach_usage(void)
    {
    printf("usage: %s", usbip_detach_usage_string);
    }
#[no_mangle]
unsafe extern "C" fn detach_port(port: *mut c_char) -> c_int {
    static int detach_port(char *port)
    {
    let mut ret: c_int = 0;
    uint8_t portnum;
    char path[PATH_MAX+1];
    int i;
    struct usbip_imported_device *idev;
    let mut found: c_int = 0;
    let mut port_len: c_uint = strlen(port);
    for (unsigned int i = 0; i < port_len; i++)
    if (!isdigit(port[i])) {
    err("invalid port %s", port);
    return -1;
    }
    portnum = atoi(port);
    ret = usbip_vhci_driver_open();
    if (ret < 0) {
    err("open vhci_driver (is vhci_hcd loaded?)");
    return -1;
    }
// check for invalid port
    for (i = 0; i < vhci_driver.nports; i++) {
    idev = &vhci_driver.idev[i];
    if (idev.port == portnum) {
    found = 1;
    if (idev.status != VDEV_ST_NULL)
    break;
    info("Port %d is already detached!\n", idev.port);
    goto call_driver_close;
    }
    }
    if (!found) {
    ret = -1;
    err("Invalid port %s > maxports %d",
    port, vhci_driver.nports);
    goto call_driver_close;
    }
// remove the port state file
    snprintf(path, PATH_MAX, VHCI_STATE_PATH"/port%d", portnum);
    remove(path);
    rmdir(VHCI_STATE_PATH);
    ret = usbip_vhci_detach_device(portnum);
    if (ret < 0) {
    ret = -1;
    err("Port %d detach request failed!\n", portnum);
    goto call_driver_close;
    }
    info("Port %d is now detached!\n", portnum);
    call_driver_close:
    usbip_vhci_driver_close();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn usbip_detach(argc: c_int, argv[]: *mut c_char) -> c_int {
    int usbip_detach(int argc, char *argv[])
    {
    static const struct option opts[] = {
    { "port", required_argument, core::ptr::null_mut(), 'p' },
    { core::ptr::null_mut(), 0, core::ptr::null_mut(), 0 }
    };
    int opt;
    let mut ret: c_int = -1;
    for (;;) {
    opt = getopt_long(argc, argv, "p:", opts, core::ptr::null_mut());
    if (opt == -1)
    break;
    switch (opt) {
    case 'p':
    ret = detach_port(optarg);
    goto out;
    default:
    goto err_out;
    }
    }
    err_out:
    usbip_detach_usage();
    out:
    return ret;
    }
