//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/v4l2-fh.h
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
// v4l2-fh.h
//
// V4L2 file handle. Store per file handle data for the V4L2
// framework. Using file handles is mandatory for the drivers.
//
// Copyright (C) 2009--2010 Nokia Corporation.
//
// Contact: Sakari Ailus <sakari.ailus@iki.fi>
//

//
// struct v4l2_fh - Describes a V4L2 file handler
//
// @list: list of file handlers
// @vdev: pointer to &struct video_device
// @ctrl_handler: pointer to &struct v4l2_ctrl_handler
// @prio: priority of the file handler, as defined by &enum v4l2_priority
//
// @wait: event' s wait queue
// @subscribe_lock: serialise changes to the subscribed list; guarantee that
// the add and del event callbacks are orderly called
// @subscribed: list of subscribed events
// @available: list of events waiting to be dequeued
// @navailable: number of available events at @available list
// @sequence: event sequence number
//
// @m2m_ctx: pointer to &struct v4l2_m2m_ctx
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_fh {
    pub list: list_head,
    pub vdev: *mut video_device,
    pub ctrl_handler: *mut v4l2_ctrl_handler,
    pub prio: v4l2_priority,
// Events
    pub wait: wait_queue_head_t,
    pub subscribe_lock: mutex,
    pub subscribed: list_head,
    pub available: list_head,
    pub navailable: c_uint,
    pub sequence: u32,
    pub m2m_ctx: *mut v4l2_m2m_ctx,
}

//
// file_to_v4l2_fh - Return the v4l2_fh associated with a struct file
//
// @filp: pointer to &struct file
//
// This function should be used by drivers to retrieve the &struct v4l2_fh
// instance pointer stored in the file private_data instead of accessing the
// private_data field directly.
//
// v4l2_fh_init - Initialise the file handle.
//
// @fh: pointer to &struct v4l2_fh
// @vdev: pointer to &struct video_device
//
// Parts of the V4L2 framework using the
// file handles should be initialised in this function. Must be called
// from driver's v4l2_file_operations->open\(\) handler if the driver
// uses &struct v4l2_fh.
//
extern "C" {
    pub fn v4l2_fh_init(fh: *mut v4l2_fh, vdev: *mut video_device);
}
//
// v4l2_fh_add - Add the fh to the list of file handles on a video_device.
//
// @fh: pointer to &struct v4l2_fh
// @filp: pointer to &struct file associated with @fh
//
// The function sets filp->private_data to point to @fh.
//
// .. note::
// The @fh file handle must be initialised first.
//
extern "C" {
    pub fn v4l2_fh_add(fh: *mut v4l2_fh, filp: *mut file);
}
//
// v4l2_fh_open - Ancillary routine that can be used as the open\(\) op
// of v4l2_file_operations.
//
// @filp: pointer to struct file
//
// It allocates a v4l2_fh and inits and adds it to the &struct video_device
// associated with the file pointer.
//
// On error filp->private_data will be %NULL, otherwise it will point to
// the &struct v4l2_fh.
//
extern "C" {
    pub fn v4l2_fh_open(filp: *mut file) -> c_int;
}
//
// v4l2_fh_del - Remove file handle from the list of file handles.
//
// @fh: pointer to &struct v4l2_fh
// @filp: pointer to &struct file associated with @fh
//
// The function resets filp->private_data to NULL.
//
// .. note::
// Must be called in v4l2_file_operations->release\(\) handler if the driver
// uses &struct v4l2_fh.
//
extern "C" {
    pub fn v4l2_fh_del(fh: *mut v4l2_fh, filp: *mut file);
}
//
// v4l2_fh_exit - Release resources related to a file handle.
//
// @fh: pointer to &struct v4l2_fh
//
// Parts of the V4L2 framework using the v4l2_fh must release their
// resources here, too.
//
// .. note::
// Must be called in v4l2_file_operations->release\(\) handler if the
// driver uses &struct v4l2_fh.
//
extern "C" {
    pub fn v4l2_fh_exit(fh: *mut v4l2_fh);
}
//
// v4l2_fh_release - Ancillary routine that can be used as the release\(\) op
// of v4l2_file_operations.
//
// @filp: pointer to struct file
//
// It deletes and exits the v4l2_fh associated with the file pointer and
// frees it. It will do nothing if filp->private_data (the pointer to the
// v4l2_fh struct) is %NULL.
//
// This function always returns 0.
//
extern "C" {
    pub fn v4l2_fh_release(filp: *mut file) -> c_int;
}
//
// v4l2_fh_is_singular - Returns 1 if this filehandle is the only filehandle
// opened for the associated video_device.
//
// @fh: pointer to &struct v4l2_fh
//
// If @fh is NULL, then it returns 0.
//
extern "C" {
    pub fn v4l2_fh_is_singular(fh: *mut v4l2_fh) -> c_int;
}
//
// v4l2_fh_is_singular_file - Returns 1 if this filehandle is the only
// filehandle opened for the associated video_device.
//
// @filp: pointer to struct file
//
// This is a helper function variant of v4l2_fh_is_singular() with uses
// struct file as argument.
//
// If filp->private_data is %NULL, then it will return 0.
//
extern "C" {
    pub fn v4l2_fh_is_singular(_arg: filp->private_data) -> return;
}
