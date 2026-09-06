//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/issei/host_client.h
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
// Copyright (C) 2023-2026 Intel Corporation

//
// enum issei_host_client_state - host client states
// @ISSEI_HOST_CL_STATE_DISCONNECTED: host client is disconnected
// @ISSEI_HOST_CL_STATE_CONNECTED: host client is connected
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum issei_host_client_state {
    ISSEI_HOST_CL_STATE_DISCONNECTED,
    ISSEI_HOST_CL_STATE_CONNECTED,
}

//
// struct issei_host_client - represents host client
// @list: link in host clients list
// @idev: issei parent device
// @id: host client id
// @fp: file associated with client
//
// @write_wait: waitqueue for pending write data
// @write_in_progress: indicator for write in process
//
// @state: host client state
// @fw_cl: pointer to firmware client, if connected
//
// @read_wait: waitqueue for read object
// @read_data: received data pointer
// @read_data_size: received data size
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct issei_host_client {
    pub list: list_head,
    pub idev: *mut issei_device,
    pub id: u16,
    pub fp: *const file,
    pub write_wait: wait_queue_head_t,
    pub write_in_progress: bool,
    pub state: issei_host_client_state,
    pub fw_cl: *mut issei_fw_client,
    pub read_wait: wait_queue_head_t,
    pub read_data: *mut u8,
    pub read_data_size: usize,
}

extern "C" {
    pub fn issei_cl_remove(cl: *mut issei_host_client);
}
extern "C" {
    pub fn issei_cl_disconnect(cl: *mut issei_host_client) -> c_int;
}
extern "C" {
    pub fn issei_cl_all_disconnect(idev: *mut issei_device);
}
extern "C" {
    pub fn issei_cl_write(cl: *mut issei_host_client, buf: *const u8, buf_size: usize) -> isize;
}
extern "C" {
    pub fn issei_cl_write_from_queue(idev: *mut issei_device) -> c_int;
}
extern "C" {
    pub fn issei_cl_read_buf(idev: *mut issei_device, fw_id: u16, host_id: u16, buf: *mut u8, buf_size: usize) -> c_int;
}
extern "C" {
    pub fn issei_cl_read(cl: *mut issei_host_client, buf: *mut u8, buf_size: usize) -> isize;
}
extern "C" {
    pub fn issei_cl_check_read(cl: *mut issei_host_client) -> c_int;
}
extern "C" {
    pub fn issei_cl_check_write(cl: *mut issei_host_client) -> c_int;
}
extern "C" {
    pub fn issei_cl_clean_all_wbuf(cl: *mut issei_host_client);
}
