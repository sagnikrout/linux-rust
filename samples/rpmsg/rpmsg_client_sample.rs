//! Automatically rewritten from C to Rust
//! Source: samples/rpmsg/rpmsg_client_sample.c
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
// Remote processor messaging - sample client driver
//
// Copyright (C) 2011 Texas Instruments, Inc.
// Copyright (C) 2011 Google, Inc.
//
// Ohad Ben-Cohen <ohad@wizery.com>
// Brian Swetland <swetland@google.com>
//

    let mut count: static int = 100;
    module_param(count, int, 0644);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct instance_data {
    pub rx_count: c_int,
}

    static int rpmsg_sample_cb(struct rpmsg_device *rpdev, void *data, int len,
    void *priv, u32 src)
    {
    int ret;
    struct instance_data *idata = dev_get_drvdata(&rpdev.dev);
    dev_info(&rpdev.dev, "incoming msg %d (src: 0x%x)\n",
    ++idata.rx_count, src);
    print_hex_dump_debug(__func__, DUMP_PREFIX_NONE, 16, 1, data, len,
    true);
// samples should not live forever
    if (idata.rx_count >= count) {
    dev_info(&rpdev.dev, "goodbye!\n");
    return 0;
    }
// send a new message now
    ret = rpmsg_send(rpdev.ept, MSG, strlen(MSG));
    if (ret)
    dev_err(&rpdev.dev, "rpmsg_send failed: %d\n", ret);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rpmsg_sample_probe(rpdev: *mut rpmsg_device) -> c_int {
    static int rpmsg_sample_probe(struct rpmsg_device *rpdev)
    {
    int ret;
    struct instance_data *idata;
    dev_info(&rpdev.dev, "new channel: 0x%x . 0x%x!\n",
    rpdev.src, rpdev.dst);
    idata = devm_kzalloc(&rpdev.dev, sizeof(*idata), GFP_KERNEL);
    if (!idata)
    return -ENOMEM;
    dev_set_drvdata(&rpdev.dev, idata);
// send a message to our remote processor
    ret = rpmsg_send(rpdev.ept, MSG, strlen(MSG));
    if (ret) {
    dev_err(&rpdev.dev, "rpmsg_send failed: %d\n", ret);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rpmsg_sample_remove(rpdev: *mut rpmsg_device) {
    static void rpmsg_sample_remove(struct rpmsg_device *rpdev)
    {
    dev_info(&rpdev.dev, "rpmsg sample client driver is removed\n");
    }
    static struct rpmsg_device_id rpmsg_driver_sample_id_table[] = {
    { .name	= "rpmsg-client-sample" },
    { },
    };
    MODULE_DEVICE_TABLE(rpmsg, rpmsg_driver_sample_id_table);
    static struct rpmsg_driver rpmsg_sample_client = {
    .drv.name	= KBUILD_MODNAME,
    .id_table	= rpmsg_driver_sample_id_table,
    .probe		= rpmsg_sample_probe,
    .callback	= rpmsg_sample_cb,
    .remove		= rpmsg_sample_remove,
    };
    module_rpmsg_driver(rpmsg_sample_client);
    MODULE_DESCRIPTION("Remote processor messaging sample client driver");
    MODULE_LICENSE("GPL v2");
