//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/qcom/tcs.h
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
// Copyright (c) 2016-2019, The Linux Foundation. All rights reserved.
//

pub const MAX_RPMH_PAYLOAD: c_int = 16;
//
// rpmh_state: state for the request
//
// RPMH_SLEEP_STATE:       State of the resource when the processor subsystem
// is powered down. There is no client using the
// resource actively.
// RPMH_WAKE_ONLY_STATE:   Resume resource state to the value previously
// requested before the processor was powered down.
// RPMH_ACTIVE_ONLY_STATE: Active or AMC mode requests. Resource state
// is aggregated immediately.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpmh_state {
    RPMH_SLEEP_STATE,
    RPMH_WAKE_ONLY_STATE,
    RPMH_ACTIVE_ONLY_STATE,
}

//
// struct tcs_cmd: an individual request to RPMH.
//
// @addr: the address of the resource slv_id:18:16 | offset:0:15
// @data: the resource state request
// @wait: ensure that this command is complete before returning.
// Setting "wait" here only makes sense during rpmh_write_batch() for
// active-only transfers, this is because:
// rpmh_write() - Always waits.
// (DEFINE_RPMH_MSG_ONSTACK will set .wait_for_compl)
// rpmh_write_async() - Never waits.
// (There's no request completion callback)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcs_cmd {
    pub addr: u32,
    pub data: u32,
    pub wait: u32,
}

//
// struct tcs_request: A set of tcs_cmds sent together in a TCS
//
// @state:          state for the request.
// @is_read:        set for read only requests
// @wait_for_compl: wait until we get a response from the h/w accelerator
// (same as setting cmd->wait for all commands in the request)
// @num_cmds:       the number of @cmds in this request
// @cmds:           an array of tcs_cmds
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcs_request {
    pub state: rpmh_state,
    pub is_read: bool,
    pub wait_for_compl: u32,
    pub num_cmds: u32,
    pub cmds: *mut tcs_cmd,
}

// Construct a Bus Clock Manager (BCM) specific TCS command

