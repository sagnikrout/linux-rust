//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_gsc_types.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2023 Intel Corporation
//

//
// struct xe_gsc - GSC
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_gsc {
// @fw: Generic uC firmware management
    pub fw: xe_uc_fw,
// @security_version: SVN found in the fetched blob
    pub security_version: u32,
// @private: Private data for use by the GSC FW
    pub private: *mut xe_bo,
// @q: Default queue used for submissions to GSC FW
    pub q: *mut xe_exec_queue,
// @wq: workqueue to handle jobs for delayed load and proxy handling
    pub wq: *mut workqueue_struct,
// @work: delayed load and proxy handling work
    pub work: work_struct,
// @lock: protects access to the work_actions mask
    pub lock: spinlock_t,
// @work_actions: mask of actions to be performed in the work
    pub work_actions: u32,

// @proxy: sub-structure containing the SW proxy-related variables
// @proxy.component: struct for communication with mei component
    pub component: *mut i915_gsc_proxy_component,
// @proxy.mutex: protects the component binding and usage
    pub mutex: mutex,
// @proxy.component_added: whether the component has been added
    pub component_added: bool,
// @proxy.started: whether the proxy has been started
    pub started: bool,
// @proxy.bo: object to store message to and from the GSC
    pub bo: *mut xe_bo,
// @proxy.to_gsc: map of the memory used to send messages to the GSC
    pub to_gsc: iosys_map,
// @proxy.from_gsc: map of the memory used to recv messages from the GSC
    pub from_gsc: iosys_map,
// @proxy.to_csme: pointer to the memory used to send messages to CSME
    pub to_csme: *mut c_void,
// @proxy.from_csme: pointer to the memory used to recv messages from CSME
    pub from_csme: *mut c_void,
    pub proxy: },
}
