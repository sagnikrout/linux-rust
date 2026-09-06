//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_guc_types.h
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
// Copyright © 2022 Intel Corporation
//

//
// struct xe_guc_db_mgr - GuC Doorbells Manager.
//
// Note: GuC Doorbells Manager is relying on &xe_guc::submission_state.lock
// to protect its members.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_guc_db_mgr {
// @count: number of doorbells to manage
    pub count: c_uint,
// @bitmap: bitmap to track allocated doorbells
    pub bitmap: *mut c_ulong,
}

//
// struct xe_guc_id_mgr - GuC context ID Manager.
//
// Note: GuC context ID Manager is relying on &xe_guc::submission_state.lock
// to protect its members.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_guc_id_mgr {
// @bitmap: bitmap to track allocated IDs
    pub bitmap: *mut c_ulong,
// @total: total number of IDs being managed
    pub total: c_uint,
// @used: number of IDs currently in use
    pub used: c_uint,
}

//
// struct xe_guc - Graphic micro controller
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_guc {
// @fw: Generic uC firmware management
    pub fw: xe_uc_fw,
// @log: GuC log
    pub log: xe_guc_log,
// @ads: GuC ads
    pub ads: xe_guc_ads,
// @ct: GuC ct
    pub ct: xe_guc_ct,
// @buf: GuC Buffer Cache manager
    pub buf: xe_guc_buf_cache,
// @capture: the error-state-capture module's data and objects
    pub capture: *mut xe_guc_state_capture,
// @pc: GuC Power Conservation
    pub pc: xe_guc_pc,
// @dbm: GuC Doorbell Manager
    pub dbm: xe_guc_db_mgr,
// @g2g: GuC to GuC communication state
// @g2g.bo: Storage for GuC to GuC communication channels
    pub bo: *mut xe_bo,
// @g2g.owned: Is the BO owned by this GT or just mapped in
    pub owned: bool,
    pub g2g: },
// @submission_state: GuC submission state
// @submission_state.idm: GuC context ID Manager
    pub idm: xe_guc_id_mgr,
// @submission_state.exec_queue_lookup: Lookup an xe_engine from guc_id
    pub exec_queue_lookup: xarray,
// @submission_state.stopped: submissions are stopped
    pub stopped: core::sync::atomic::AtomicI32,
//
// @submission_state.reset_blocked: reset attempts are blocked;
// blocking reset in order to delay it may be required if running
// an operation which is sensitive to resets.
//
    pub reset_blocked: core::sync::atomic::AtomicI32,
// @submission_state.lock: protects submission state
    pub lock: mutex,
// @submission_state.enabled: submission is enabled
    pub enabled: bool,
//
// @submission_state.initialized: mark when submission state is
// even initialized - before that not even the lock is valid
//
    pub initialized: bool,
    pub submission_state: },
// @hwconfig: Hardware config state
// @hwconfig.bo: buffer object of the hardware config
    pub bo: *mut xe_bo,
// @hwconfig.size: size of the hardware config
    pub size: u32,
    pub hwconfig: },
// @relay: GuC Relay Communication used in SR-IOV
    pub relay: xe_guc_relay,
// @engine_activity: Device specific engine activity
    pub engine_activity: xe_guc_engine_activity,
//
// @notify_reg: Register which is written to notify GuC of H2G messages
//
    pub notify_reg: xe_reg,
// @params: Control params for fw initialization
    pub params: [u32; GUC_CTL_MAX_DWORDS],
}
