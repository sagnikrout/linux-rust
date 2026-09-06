//! Automatically rewritten from C to Rust
//! Source: drivers/media/v4l2-core/v4l2-fh.c
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
// v4l2-fh.c
//
// V4L2 file handles.
//
// Copyright (C) 2009--2010 Nokia Corporation.
//
// Contact: Sakari Ailus <sakari.ailus@iki.fi>
//

#[no_mangle]
pub unsafe extern "C" fn v4l2_fh_init(fh: *mut v4l2_fh, vdev: *mut video_device) {
    void v4l2_fh_init(struct v4l2_fh *fh, struct video_device *vdev)
    {
    fh.vdev = vdev;
// Inherit from video_device. May be overridden by the driver.
    fh.ctrl_handler = vdev.ctrl_handler;
    INIT_LIST_HEAD(&fh.list);
    set_bit(V4L2_FL_USES_V4L2_FH, &fh.vdev.flags);
//
// determine_valid_ioctls() does not know if struct v4l2_fh
// is used by this driver, but here we do. So enable the
// prio ioctls here.
//
    set_bit(_IOC_NR(VIDIOC_G_PRIORITY), vdev.valid_ioctls);
    set_bit(_IOC_NR(VIDIOC_S_PRIORITY), vdev.valid_ioctls);
    fh.prio = V4L2_PRIORITY_UNSET;
    init_waitqueue_head(&fh.wait);
    INIT_LIST_HEAD(&fh.available);
    INIT_LIST_HEAD(&fh.subscribed);
    fh.sequence = -1;
    mutex_init(&fh.subscribe_lock);
    }
    EXPORT_SYMBOL_GPL(v4l2_fh_init);
#[no_mangle]
pub unsafe extern "C" fn v4l2_fh_add(fh: *mut v4l2_fh, filp: *mut file) {
    void v4l2_fh_add(struct v4l2_fh *fh, struct file *filp)
    {
    unsigned long flags;
    filp.private_data = fh;
    v4l2_prio_open(fh.vdev.prio, &fh.prio);
    spin_lock_irqsave(&fh.vdev.fh_lock, flags);
    list_add(&fh.list, &fh.vdev.fh_list);
    spin_unlock_irqrestore(&fh.vdev.fh_lock, flags);
    }
    EXPORT_SYMBOL_GPL(v4l2_fh_add);
#[no_mangle]
pub unsafe extern "C" fn v4l2_fh_open(filp: *mut file) -> c_int {
    int v4l2_fh_open(struct file *filp)
    {
    struct video_device *vdev = video_devdata(filp);
    struct v4l2_fh *fh = kzalloc_obj(*fh);
    if (fh == core::ptr::null_mut())
    return -ENOMEM;
    v4l2_fh_init(fh, vdev);
    v4l2_fh_add(fh, filp);
    return 0;
    }
    EXPORT_SYMBOL_GPL(v4l2_fh_open);
#[no_mangle]
pub unsafe extern "C" fn v4l2_fh_del(fh: *mut v4l2_fh, filp: *mut file) {
    void v4l2_fh_del(struct v4l2_fh *fh, struct file *filp)
    {
    unsigned long flags;
    spin_lock_irqsave(&fh.vdev.fh_lock, flags);
    list_del_init(&fh.list);
    spin_unlock_irqrestore(&fh.vdev.fh_lock, flags);
    v4l2_prio_close(fh.vdev.prio, fh.prio);
    filp.private_data = core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(v4l2_fh_del);
#[no_mangle]
pub unsafe extern "C" fn v4l2_fh_exit(fh: *mut v4l2_fh) {
    void v4l2_fh_exit(struct v4l2_fh *fh)
    {
    if (fh.vdev == core::ptr::null_mut())
    return;
    v4l_disable_media_source(fh.vdev);
    v4l2_event_unsubscribe_all(fh);
    mutex_destroy(&fh.subscribe_lock);
    fh.vdev = core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(v4l2_fh_exit);
#[no_mangle]
pub unsafe extern "C" fn v4l2_fh_release(filp: *mut file) -> c_int {
    int v4l2_fh_release(struct file *filp)
    {
    struct v4l2_fh *fh = file_to_v4l2_fh(filp);
    if (fh) {
    v4l2_fh_del(fh, filp);
    v4l2_fh_exit(fh);
    kfree(fh);
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(v4l2_fh_release);
#[no_mangle]
pub unsafe extern "C" fn v4l2_fh_is_singular(fh: *mut v4l2_fh) -> c_int {
    int v4l2_fh_is_singular(struct v4l2_fh *fh)
    {
    unsigned long flags;
    int is_singular;
    if (fh == core::ptr::null_mut() || fh.vdev == core::ptr::null_mut())
    return 0;
    spin_lock_irqsave(&fh.vdev.fh_lock, flags);
    is_singular = list_is_singular(&fh.list);
    spin_unlock_irqrestore(&fh.vdev.fh_lock, flags);
    return is_singular;
    }
    EXPORT_SYMBOL_GPL(v4l2_fh_is_singular);
