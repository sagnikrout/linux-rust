//! Automatically rewritten from C to Rust
//! Source: drivers/media/usb/pvrusb2/pvrusb2-main.c
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
// Copyright (C) 2005 Mike Isely <isely@pobox.com>
// Copyright (C) 2004 Aurelien Alleaume <slts@free.fr>
//

    PVR2_TRACE_INFO| \
    PVR2_TRACE_STD| \
    PVR2_TRACE_TOLERANCE| \
    PVR2_TRACE_TRAP| \
    0)
    let mut pvrusb2_debug: c_int = DEFAULT_DEBUG_MASK;
    module_param_named(debug,pvrusb2_debug,int,S_IRUGO|S_IWUSR);
    MODULE_PARM_DESC(debug, "Debug trace mask");
#[no_mangle]
unsafe extern "C" fn pvr_setup_attach(pvr: *mut pvr2_context) {
    static void pvr_setup_attach(struct pvr2_context *pvr)
    {
// Create association with v4l layer
    pvr2_v4l2_create(pvr);

// Create association with dvb layer
    pvr2_dvb_create(pvr);

    pvr2_sysfs_create(pvr);
    }
    static int pvr_probe(struct usb_interface *intf,
    const struct usb_device_id *devid)
    {
    struct pvr2_context *pvr;
// Create underlying hardware interface
    pvr = pvr2_context_create(intf,devid,pvr_setup_attach);
    if (!pvr) {
    pvr2_trace(PVR2_TRACE_ERROR_LEGS,
    "Failed to create hdw handler");
    return -ENOMEM;
    }
    pvr2_trace(PVR2_TRACE_INIT,"pvr_probe(pvr=%p)",pvr);
    usb_set_intfdata(intf, pvr);
    return 0;
    }
//
// pvr_disconnect()
//
#[no_mangle]
unsafe extern "C" fn pvr_disconnect(intf: *mut usb_interface) {
    static void pvr_disconnect(struct usb_interface *intf)
    {
    struct pvr2_context *pvr = usb_get_intfdata(intf);
    pvr2_trace(PVR2_TRACE_INIT,"pvr_disconnect(pvr=%p) BEGIN",pvr);
    usb_set_intfdata (intf, core::ptr::null_mut());
    pvr2_context_disconnect(pvr);
    pvr2_trace(PVR2_TRACE_INIT,"pvr_disconnect(pvr=%p) DONE",pvr);
    }
    static struct usb_driver pvr_driver = {
    .name =         "pvrusb2",
    .id_table =     pvr2_device_table,
    .probe =        pvr_probe,
    .disconnect =   pvr_disconnect
    };
//
// pvr_init() / pvr_exit()
//
// This code is run to initialize/exit the driver.
//
#[no_mangle]
unsafe extern "C" fn pvr_init() -> int __init {
    static int __init pvr_init(void)
    {
    int ret;
    pvr2_trace(PVR2_TRACE_INIT,"pvr_init");
    ret = pvr2_context_global_init();
    if (ret != 0) {
    pvr2_trace(PVR2_TRACE_INIT,"pvr_init failure code=%d",ret);
    return ret;
    }
    pvr2_sysfs_class_create();
    ret = usb_register(&pvr_driver);
    if (ret == 0)
    pr_info("pvrusb2: " DRIVER_VERSION ":"
    DRIVER_DESC "\n");
    if (pvrusb2_debug)
    pr_info("pvrusb2: Debug mask is %d (0x%x)\n",
    pvrusb2_debug,pvrusb2_debug);
    pvr2_trace(PVR2_TRACE_INIT,"pvr_init complete");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pvr_exit() -> void __exit {
    static void __exit pvr_exit(void)
    {
    pvr2_trace(PVR2_TRACE_INIT,"pvr_exit");
    usb_deregister(&pvr_driver);
    pvr2_context_global_done();
    pvr2_sysfs_class_destroy();
    pvr2_trace(PVR2_TRACE_INIT,"pvr_exit complete");
    }
    module_init(pvr_init);
    module_exit(pvr_exit);
    MODULE_AUTHOR(DRIVER_AUTHOR);
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_LICENSE("GPL");
    MODULE_VERSION("0.9.1");
