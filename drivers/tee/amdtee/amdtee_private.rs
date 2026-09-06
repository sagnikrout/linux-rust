//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/tee/amdtee/amdtee_private.h
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
// Copyright 2019 Advanced Micro Devices, Inc.
//

// Some GlobalPlatform error codes used in this driver
pub const TEEC_SUCCESS: c_uint = 0x00000000;
pub const TEEC_ERROR_GENERIC: c_uint = 0xFFFF0000;
pub const TEEC_ERROR_BAD_PARAMETERS: c_uint = 0xFFFF0006;
pub const TEEC_ERROR_OUT_OF_MEMORY: c_uint = 0xFFFF000C;
pub const TEEC_ERROR_COMMUNICATION: c_uint = 0xFFFF000E;
pub const TEEC_ORIGIN_COMMS: c_uint = 0x00000002;
// Maximum number of sessions which can be opened with a Trusted Application
pub const TEE_NUM_SESSIONS: c_int = 32;

pub const TA_PATH_MAX: c_int = 60;
//
// struct amdtee - main service struct
// @teedev:		client device
// @pool:		shared memory pool
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdtee {
    pub teedev: *mut tee_device,
    pub pool: *mut tee_shm_pool,
}

//
// struct amdtee_session - Trusted Application (TA) session related information.
// @ta_handle:     handle to Trusted Application (TA) loaded in TEE environment
// @refcount:      counter to keep track of sessions opened for the TA instance
// @session_info:  an array pointing to TA allocated session data.
// @sess_mask:     session usage bit-mask. If a particular bit is set, then the
// corresponding @session_info entry is in use or valid.
//
// Session structure is updated on open_session and this information is used for
// subsequent operations with the Trusted Application.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdtee_session {
    pub list_node: list_head,
    pub ta_handle: u32,
    pub refcount: kref,
    pub session_info: [u32; TEE_NUM_SESSIONS],
    pub TEE_NUM_SESSIONS): DECLARE_BITMAP(sess_mask,,
    pub /: *mut *mut spinlock_t lock; / synchronizes access to @sess_mask,
}

//
// struct amdtee_context_data - AMD-TEE driver context data
// @sess_list:    Keeps track of sessions opened in current TEE context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdtee_context_data {
    pub sess_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdtee_driver_data {
    pub amdtee: *mut amdtee,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmem_desc {
    pub kaddr: *mut c_void,
    pub size: u64,
}

//
// struct amdtee_ta_data - Keeps track of all TAs loaded in AMD Secure
// Processor
// @ta_handle:	Handle to TA loaded in TEE
// @refcount:	Reference count for the loaded TA
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdtee_ta_data {
    pub list_node: list_head,
    pub ta_handle: u32,
    pub refcount: u32,
}

pub const LOWER_TWO_BYTE_MASK: c_uint = 0x0000FFFF;
//
// set_session_id() - Sets the session identifier.
// @ta_handle:      [in] handle of the loaded Trusted Application (TA)
// @session_index:  [in] Session index. Range: 0 to (TEE_NUM_SESSIONS - 1).
// @session:        [out] Pointer to session id
//
// Lower two bytes of the session identifier represents the TA handle and the
// upper two bytes is session index.
//
// session = (session_index << 16) | (LOWER_TWO_BYTE_MASK & ta_handle);
extern "C" {
    pub fn amdtee_close_session(ctx: *mut tee_context, session: u32) -> c_int;
}
extern "C" {
    pub fn amdtee_cancel_req(ctx: *mut tee_context, cancel_id: u32, session: u32) -> c_int;
}
extern "C" {
    pub fn amdtee_map_shmem(shm: *mut tee_shm) -> c_int;
}
extern "C" {
    pub fn amdtee_unmap_shmem(shm: *mut tee_shm);
}
extern "C" {
    pub fn handle_unload_ta(ta_handle: u32) -> c_int;
}
extern "C" {
    pub fn handle_close_session(ta_handle: u32, info: u32) -> c_int;
}
extern "C" {
    pub fn handle_map_shmem(count: u32, start: *mut shmem_desc, buf_id: *mut u32) -> c_int;
}
extern "C" {
    pub fn handle_unmap_shmem(buf_id: u32);
}
