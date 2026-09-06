//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/raspberrypi/vchiq-mmal/mmal-msg-common.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Broadcom BCM2835 V4L2 driver
//
// Copyright © 2013 Raspberry Pi (Trading) Ltd.
//
// Authors: Vincent Sanders @ Collabora
// Dave Stevenson @ Broadcom
// (now dave.stevenson@raspberrypi.org)
// Simon Mellor @ Broadcom
// Luke Diamand @ Broadcom
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mmal_msg_status {
    MMAL_MSG_STATUS_SUCCESS = 0, /**< Success */
    MMAL_MSG_STATUS_ENOMEM,      /**< Out of memory */
    MMAL_MSG_STATUS_ENOSPC,      /**< Out of resources other than memory */
    MMAL_MSG_STATUS_EINVAL,      /**< Argument is invalid */
    MMAL_MSG_STATUS_ENOSYS,      /**< Function not implemented */
    MMAL_MSG_STATUS_ENOENT,      /**< No such file or directory */
    MMAL_MSG_STATUS_ENXIO,       /**< No such device or address */
    MMAL_MSG_STATUS_EIO,         /**< I/O error */
    MMAL_MSG_STATUS_ESPIPE,      /**< Illegal seek */
    MMAL_MSG_STATUS_ECORRUPT,    /**< Data is corrupt \attention */
    MMAL_MSG_STATUS_ENOTREADY,   /**< Component is not ready */
    MMAL_MSG_STATUS_ECONFIG,     /**< Component is not configured */
    MMAL_MSG_STATUS_EISCONN,     /**< Port is already connected */
    MMAL_MSG_STATUS_ENOTCONN,    /**< Port is disconnected */
    MMAL_MSG_STATUS_EAGAIN,      /**< Resource temporarily unavailable. */
    MMAL_MSG_STATUS_EFAULT,      /**< Bad address */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_rect {
    pub /: *mut *mut *mut s32 x; /< x coordinate (from left),
    pub /: *mut *mut *mut s32 y; /< y coordinate (from top),
    pub /: *mut *mut *mut s32 width; /< width,
    pub /: *mut *mut *mut s32 height; /< height,
}
