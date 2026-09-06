//! Automatically rewritten from C to Rust
//! Source: drivers/hid/intel-ish-hid/ishtp/init.c
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
// Initialization protocol for ISHTP driver
//
// Copyright (c) 2003-2016, Intel Corporation.
//

//
// ishtp_device_init() - ishtp device init
// @dev: ISHTP device instance
//
// After ISHTP device is alloacted, this function is used to initialize
// each field which includes spin lock, work struct and lists
//
#[no_mangle]
pub unsafe extern "C" fn ishtp_device_init(dev: *mut ishtp_device) {
    void ishtp_device_init(struct ishtp_device *dev)
    {
    int ret;
    dev.dev_state = ISHTP_DEV_INITIALIZING;
    INIT_LIST_HEAD(&dev.cl_list);
    INIT_LIST_HEAD(&dev.device_list);
    dev.rd_msg_fifo_head = 0;
    dev.rd_msg_fifo_tail = 0;
    spin_lock_init(&dev.rd_msg_spinlock);
    init_waitqueue_head(&dev.wait_hbm_recvd_msg);
    init_waitqueue_head(&dev.wait_loader_recvd_msg);
    spin_lock_init(&dev.read_list_spinlock);
    spin_lock_init(&dev.device_lock);
    spin_lock_init(&dev.device_list_lock);
    spin_lock_init(&dev.cl_list_lock);
    spin_lock_init(&dev.fw_clients_lock);
    INIT_WORK(&dev.bh_hbm_work, bh_hbm_work_fn);
    bitmap_zero(dev.host_clients_map, ISHTP_CLIENTS_MAX);
    dev.open_handle_count = 0;
//
// Reserving client ID 0 for ISHTP Bus Message communications
//
    bitmap_set(dev.host_clients_map, 0, 1);
    INIT_LIST_HEAD(&dev.read_list.list);
    ret = devm_work_autocancel(dev.devc, &dev.work_fw_loader, ishtp_loader_work);
    if (ret)
    dev_err_probe(dev.devc, ret, "Failed to initialise FW loader work\n");
    }
    EXPORT_SYMBOL(ishtp_device_init);
//
// ishtp_start() - Start ISH processing
// @dev: ISHTP device instance
//
// Start ISHTP processing by sending query subscriber message
//
// Return: 0 on success else -ENODEV
//
#[no_mangle]
pub unsafe extern "C" fn ishtp_start(dev: *mut ishtp_device) -> c_int {
    int ishtp_start(struct ishtp_device *dev)
    {
    if (ishtp_hbm_start_wait(dev)) {
    dev_err(dev.devc, "HBM haven't started");
    goto err;
    }
// suspend & resume notification - send QUERY_SUBSCRIBERS msg
    ishtp_query_subscribers(dev);
    return 0;
    err:
    dev_err(dev.devc, "link layer initialization failed.\n");
    dev.dev_state = ISHTP_DEV_DISABLED;
    return -ENODEV;
    }
    EXPORT_SYMBOL(ishtp_start);
