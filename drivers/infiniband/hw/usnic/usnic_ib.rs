//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/usnic/usnic_ib.h
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
// Copyright (c) 2013, Cisco Systems, Inc. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// BSD license below:
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

pub const USNIC_IB_PORT_CNT: c_int = 1;
pub const USNIC_IB_NUM_COMP_VECTORS: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usnic_ib_ucontext {
    pub ibucontext: ib_ucontext,
// Protected by usnic_ib_dev->usdev_lock
    pub qp_grp_list: list_head,
    pub link: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usnic_ib_pd {
    pub ibpd: ib_pd,
    pub umem_pd: *mut usnic_uiom_pd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usnic_ib_cq {
    pub ibcq: ib_cq,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usnic_ib_mr {
    pub ibmr: ib_mr,
    pub umem: *mut usnic_uiom_reg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usnic_ib_dev {
    pub ib_dev: ib_device,
    pub pdev: *mut pci_dev,
    pub netdev: *mut net_device,
    pub ufdev: *mut usnic_fwd_dev,
    pub ib_dev_link: list_head,
    pub vf_dev_list: list_head,
    pub ctx_list: list_head,
    pub usdev_lock: mutex,
// provisioning information
    pub vf_cnt: kref,
    pub vf_res_cnt: [c_uint; USNIC_VNIC_RES_TYPE_MAX],
// sysfs vars for QPN reporting
    pub qpn_kobj: *mut kobject,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usnic_ib_vf {
    pub pf: *mut usnic_ib_dev,
    pub lock: mutex,
    pub vnic: *mut usnic_vnic,
    pub qp_grp_ref_cnt: c_uint,
    pub pd: *mut usnic_ib_pd,
    pub link: list_head,
}

extern "C" {
    pub fn container_of(_arg: ibdev, usnic_ib_dev: struct, _arg: ib_dev) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibucontext, usnic_ib_ucontext: struct, _arg: ibucontext) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibpd, usnic_ib_pd: struct, _arg: ibpd) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibucontext, usnic_ib_ucontext: struct, _arg: ibucontext) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibmr, usnic_ib_mr: struct, _arg: ibmr) -> return;
}
extern "C" {
    pub fn usnic_ib_log_vf(vf: *mut usnic_ib_vf);
}

