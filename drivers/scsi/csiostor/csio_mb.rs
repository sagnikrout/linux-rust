//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/csiostor/csio_mb.h
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


//
// This file is part of the Chelsio FCoE driver for Linux.
//
// Copyright (c) 2008-2012 Chelsio Communications, Inc. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_fcoe_port_cmd_params {
    pub portid: u8,
    pub idx: u8,
    pub nstats: u8,
}

pub const CSIO_MB_MAX_REGS: c_int = 8;
pub const CSIO_MAX_MB_SIZE: c_int = 64;

// Device master in HELLO command
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum csio_dev_master {

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum csio_mb_owner {

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum csio_dev_state {
    CSIO_DEV_STATE_UNINIT,
    CSIO_DEV_STATE_INIT,
    CSIO_DEV_STATE_ERR
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_mbm_stats {
    pub /: *mut *mut uint32_t n_req; / number of mbox req,
    pub /: *mut *mut uint32_t n_rsp; / number of mbox rsp,
    pub /: *mut *mut uint32_t n_activeq; / number of mbox req active Q,
    pub /: *mut *mut uint32_t n_cbfnq; / number of mbox req cbfn Q,
    pub /: *mut *mut uint32_t n_tmo; / number of mbox timeout,
    pub /: *mut *mut uint32_t n_cancel; / number of mbox cancel,
    pub /: *mut *mut uint32_t n_err; / number of mbox error,
}

// Driver version of Mailbox
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_mb {
    pub /: *mut *mut list_head list; / for req/resp,
// queue in driver
    pub /: *mut *mut __be64 mb[CSIO_MB_MAX_REGS]; / MB in HW format,
    pub this: *mut *mut int mb_size; / Size of,
// mailbox.
//
    pub /: *mut *mut uint32_t tmo; / Timeout,
    pub Completion: *mut *mut completion cmplobj; / MB,
// object
//
    pub ): *mut *mut *mut void (mb_cbfn) (struct csio_hw , struct csio_mb,
// Callback fn
    pub /: *mut *mut *mut void priv; / Owner private ptr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_mbm {
    pub /: *mut *mut uint32_t a_mbox; / Async mbox num,
    pub /: *mut *mut uint32_t intr_idx; / Interrupt index,
    pub /: *mut *mut timer_list timer; / Mbox timer,
    pub /: *mut *mut *mut csio_hw hw; / Hardware pointer,
    pub /: *mut *mut list_head req_q; / Mbox request queue,
    pub /: *mut *mut list_head cbfn_q; / Mbox completion q,
    pub /: *mut *mut *mut csio_mb mcurrent; / Current mailbox,
    pub mbox: *mut *mut uint32_t req_q_cnt; / Outstanding,
// cmds
//
    pub /: *mut *mut csio_mbm_stats stats; / Statistics,
}

extern "C" {
    pub fn csio_mb_fw_retval(: *mut csio_mb) -> fw_retval;
}
// MB helpers
// MB module functions
extern "C" {
    pub fn csio_mbm_exit(: *mut csio_mbm);
}
extern "C" {
    pub fn csio_mb_intr_enable(: *mut csio_hw);
}
extern "C" {
    pub fn csio_mb_intr_disable(: *mut csio_hw);
}
extern "C" {
    pub fn csio_mb_issue(: *mut csio_hw, : *mut csio_mb) -> c_int;
}
extern "C" {
    pub fn csio_mb_completions(: *mut csio_hw, : *mut list_head);
}
extern "C" {
    pub fn csio_mb_fwevt_handler(: *mut csio_hw, : *mut __be64) -> c_int;
}
extern "C" {
    pub fn csio_mb_isr_handler(: *mut csio_hw) -> c_int;
}
extern "C" {
    pub fn csio_mb_cancel_all(: *mut csio_hw, : *mut list_head);
}
