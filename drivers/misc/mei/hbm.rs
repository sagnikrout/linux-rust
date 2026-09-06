//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/mei/hbm.h
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
// Copyright (c) 2003-2018, Intel Corporation. All rights reserved.
// Intel Management Engine Interface (Intel MEI) Linux driver
//
// enum mei_hbm_state - host bus message protocol state
//
// @MEI_HBM_IDLE : protocol not started
// @MEI_HBM_STARTING : start request message was sent
// @MEI_HBM_CAP_SETUP : capabilities request message was sent
// @MEI_HBM_DR_SETUP : dma ring setup request message was sent
// @MEI_HBM_ENUM_CLIENTS : enumeration request was sent
// @MEI_HBM_CLIENT_PROPERTIES : acquiring clients properties
// @MEI_HBM_STARTED : enumeration was completed
// @MEI_HBM_STOPPED : stopping exchange
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mei_hbm_state {
    MEI_HBM_IDLE = 0,
    MEI_HBM_STARTING,
    MEI_HBM_CAP_SETUP,
    MEI_HBM_DR_SETUP,
    MEI_HBM_ENUM_CLIENTS,
    MEI_HBM_CLIENT_PROPERTIES,
    MEI_HBM_STARTED,
    MEI_HBM_STOPPED,
}

extern "C" {
    pub fn mei_hbm_dispatch(dev: *mut mei_device, hdr: *mut mei_msg_hdr) -> c_int;
}
extern "C" {
    pub fn mei_hbm_idle(dev: *mut mei_device);
}
extern "C" {
    pub fn mei_hbm_reset(dev: *mut mei_device);
}
extern "C" {
    pub fn mei_hbm_start_req(dev: *mut mei_device) -> c_int;
}
extern "C" {
    pub fn mei_hbm_start_wait(dev: *mut mei_device) -> c_int;
}
extern "C" {
    pub fn mei_hbm_cl_flow_control_req(dev: *mut mei_device, cl: *mut mei_cl) -> c_int;
}
extern "C" {
    pub fn mei_hbm_cl_disconnect_req(dev: *mut mei_device, cl: *mut mei_cl) -> c_int;
}
extern "C" {
    pub fn mei_hbm_cl_disconnect_rsp(dev: *mut mei_device, cl: *mut mei_cl) -> c_int;
}
extern "C" {
    pub fn mei_hbm_cl_connect_req(dev: *mut mei_device, cl: *mut mei_cl) -> c_int;
}
extern "C" {
    pub fn mei_hbm_version_is_supported(dev: *mut mei_device) -> bool;
}
extern "C" {
    pub fn mei_hbm_pg(dev: *mut mei_device, pg_cmd: u8) -> c_int;
}
extern "C" {
    pub fn mei_hbm_pg_resume(dev: *mut mei_device);
}
extern "C" {
    pub fn mei_hbm_cl_dma_map_req(dev: *mut mei_device, cl: *mut mei_cl) -> c_int;
}
extern "C" {
    pub fn mei_hbm_cl_dma_unmap_req(dev: *mut mei_device, cl: *mut mei_cl) -> c_int;
}
