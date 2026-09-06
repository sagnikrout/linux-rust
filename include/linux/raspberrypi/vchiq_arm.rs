//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/raspberrypi/vchiq_arm.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (c) 2014 Raspberry Pi (Trading) Ltd. All rights reserved.
// Copyright (c) 2010-2012 Broadcom. All rights reserved.
//

// Some per-instance constants
pub const MAX_COMPLETIONS: c_int = 128;
pub const MAX_SERVICES: c_int = 64;
pub const MAX_ELEMENTS: c_int = 8;
pub const MSG_QUEUE_SIZE: c_int = 128;
pub const VCHIQ_DRV_MAX_CALLBACKS: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum USE_TYPE_E {
    USE_TYPE_SERVICE,
    USE_TYPE_VCHIQ
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_platform_info {
    pub cache_line_size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_drv_mgmt {
    pub fw: *mut rpi_firmware,
    pub info: *const vchiq_platform_info,
    pub connected: bool,
    pub num_deferred_callbacks: c_int,
// Protects connected and num_deferred_callbacks
    pub connected_mutex: mutex,
    pub (*deferred_callback[VCHIQ_DRV_MAX_CALLBACKS])(void): *mut c_void,
    pub free_fragments_sema: semaphore,
    pub free_fragments_mutex: semaphore,
    pub fragments_base: *mut c_char,
    pub free_fragments: *mut c_char,
    pub fragments_size: c_uint,
    pub regs: *mut void __iomem,
    pub state: vchiq_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_service {
    pub service: *mut vchiq_service,
    pub userdata: *mut void __user,
    pub instance: *mut vchiq_instance,
    pub is_vchi: c_char,
    pub dequeue_pending: c_char,
    pub close_pending: c_char,
    pub message_available_pos: c_int,
    pub msg_insert: c_int,
    pub msg_remove: c_int,
    pub insert_event: completion,
    pub remove_event: completion,
    pub close_event: completion,
    pub msg_queue: [*mut vchiq_header; MSG_QUEUE_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bulk_waiter_node {
    pub bulk_waiter: bulk_waiter,
    pub pid: c_int,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_instance {
    pub state: *mut vchiq_state,
    pub completions: [vchiq_completion_data_kernel; MAX_COMPLETIONS],
    pub completion_insert: c_int,
    pub completion_remove: c_int,
    pub insert_event: completion,
    pub remove_event: completion,
    pub completion_mutex: mutex,
    pub connected: c_int,
    pub closing: c_int,
    pub pid: c_int,
    pub mark: c_int,
    pub use_close_delivered: c_int,
    pub trace: c_int,
    pub bulk_waiter_list: list_head,
    pub bulk_waiter_list_mutex: mutex,
    pub debugfs_node: vchiq_debugfs_node,
}

