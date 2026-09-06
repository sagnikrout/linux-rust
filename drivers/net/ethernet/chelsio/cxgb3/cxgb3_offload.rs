//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb3/cxgb3_offload.h
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
// Copyright (c) 2006-2008 Chelsio, Inc. All rights reserved.
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

extern "C" {
    pub fn cxgb3_offload_init();
}
extern "C" {
    pub fn cxgb3_adapter_ofld(adapter: *mut adapter);
}
extern "C" {
    pub fn cxgb3_adapter_unofld(adapter: *mut adapter);
}
extern "C" {
    pub fn cxgb3_offload_activate(adapter: *mut adapter) -> c_int;
}
extern "C" {
    pub fn cxgb3_offload_deactivate(adapter: *mut adapter);
}
extern "C" {
    pub fn cxgb3_set_dummy_ops(dev: *mut t3cdev);
}
//
// Client registration.  Users of T3 driver must register themselves.
// The T3 driver will call the add function of every client for each T3
// adapter activated, passing up the t3cdev ptr.  Each client fills out an
// array of callback functions to process CPL messages.
//
extern "C" {
    pub fn cxgb3_register_client(client: *mut cxgb3_client);
}
extern "C" {
    pub fn cxgb3_unregister_client(client: *mut cxgb3_client);
}
extern "C" {
    pub fn cxgb3_add_clients(tdev: *mut t3cdev);
}
extern "C" {
    pub fn cxgb3_remove_clients(tdev: *mut t3cdev);
}
extern "C" {
    pub fn cxgb3_event_notify(tdev: *mut t3cdev, event: u32, port: u32);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgb3_client {
    pub name: *mut c_char,
    pub ): *mut *mut void (add) (struct t3cdev,
    pub ): *mut *mut void (remove) (struct t3cdev,
    pub handlers: *mut cxgb3_cpl_handler_func,
    pub l2t): *mut *mut dst_entry new, l2t_entry,
    pub client_list: list_head,
    pub port): *mut *mut *mut void (event_handler)(struct t3cdev tdev, u32 event, u32,
}

//
// TID allocation services.
//
extern "C" {
    pub fn cxgb3_queue_tid_release(dev: *mut t3cdev, tid: c_uint);
}
extern "C" {
    pub fn cxgb3_remove_tid(dev: *mut t3cdev, ctx: *mut c_void, tid: c_uint);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct t3c_tid_entry {
    pub client: *mut cxgb3_client,
    pub ctx: *mut c_void,
}

// CPL message priority levels
// Flags for return value of CPL message handlers
extern "C" {
    pub fn int(dev: *mut *mut cpl_handler_func)(struct t3cdev, skb: *mut sk_buff) -> typedef;
}
//
// Returns a pointer to the first byte of the CPL header in an sk_buff that
// contains a CPL message.
//
extern "C" {
    pub fn t3_register_cpl_handler(opcode: c_uint, h: cpl_handler_func);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union listen_entry {
    pub t3c_tid: t3c_tid_entry,
    pub next: *mut listen_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union active_open_entry {
    pub t3c_tid: t3c_tid_entry,
    pub next: *mut active_open_entry,
}

//
// Holds the size, base address, free list start, etc of the TID, server TID,
// and active-open TID tables for a offload device.
// The tables themselves are allocated dynamically.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tid_info {
    pub tid_tab: *mut t3c_tid_entry,
    pub ntids: c_uint,
    pub tids_in_use: core::sync::atomic::AtomicI32,
    pub stid_tab: *mut listen_entry,
    pub nstids: c_uint,
    pub stid_base: c_uint,
    pub atid_tab: *mut active_open_entry,
    pub natids: c_uint,
    pub atid_base: c_uint,
//
// The following members are accessed R/W so we put them in their own
// cache lines.
//
// XXX We could combine the atid fields above with the lock here since
// atids are use once (unlike other tids).  OTOH the above fields are
// usually in cache due to tid_tab.
//
    pub ____cacheline_aligned_in_smp: spinlock_t atid_lock,
    pub afree: *mut active_open_entry,
    pub atids_in_use: c_uint,
    pub ____cacheline_aligned: spinlock_t stid_lock,
    pub sfree: *mut listen_entry,
    pub stids_in_use: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct t3c_data {
    pub list_node: list_head,
    pub dev: *mut t3cdev,
    pub /: *mut *mut unsigned int tx_max_chunk; / max payload for TX_DATA,
    pub /: *mut *mut unsigned int max_wrs; / max in-flight WRs per connection,
    pub nmtus: c_uint,
    pub mtus: *const c_ushort,
    pub tid_maps: tid_info,
    pub tid_release_list: *mut t3c_tid_entry,
    pub tid_release_lock: spinlock_t,
    pub tid_release_task: work_struct,
    pub nofail_skb: *mut sk_buff,
    pub release_list_incomplete: c_uint,
}

//
// t3cdev -> t3c_data accessor
//

