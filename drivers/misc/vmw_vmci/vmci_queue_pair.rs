//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/vmw_vmci/vmci_queue_pair.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// VMware VMCI Driver
//
// Copyright (C) 2012 VMware, Inc. All rights reserved.
//

// Callback needed for correctly waiting on events.
extern "C" {
    pub fn int(client_data: *mut *mut vmci_event_release_cb) (void) -> typedef;
}
// Guest device port I/O.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppn_set {
    pub num_produce_pages: u64,
    pub num_consume_pages: u64,
    pub produce_ppns: *mut u64,
    pub consume_ppns: *mut u64,
    pub initialized: bool,
}

// VMCIqueue_pairAllocInfo
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_qp_alloc_info {
    pub handle: vmci_handle,
    pub peer: u32,
    pub flags: u32,
    pub produce_size: u64,
    pub consume_size: u64,
    pub /: *mut *mut u64 ppn_va; / Start VA of queue pair PPNs.,
    pub num_ppns: u64,
    pub result: i32,
    pub version: u32,
}

// VMCIqueue_pairSetVAInfo
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_qp_set_va_info {
    pub handle: vmci_handle,
    pub /: *mut *mut u64 va; / Start VA of queue pair PPNs.,
    pub num_ppns: u64,
    pub version: u32,
    pub result: i32,
}

//
// For backwards compatibility, here is a version of the
// VMCIqueue_pairPageFileInfo before host support end-points was added.
// Note that the current version of that structure requires VMX to
// pass down the VA of the mapped file.  Before host support was added
// there was nothing of the sort.  So, when the driver sees the ioctl
// with a parameter that is the sizeof
// VMCIqueue_pairPageFileInfo_NoHostQP then it can infer that the version
// of VMX running can't attach to host end points because it doesn't
// provide the VA of the mapped files.
//
// The Linux driver doesn't get an indication of the size of the
// structure passed down from user space.  So, to fix a long standing
// but unfiled bug, the _pad field has been renamed to version.
// Existing versions of VMX always initialize the PageFileInfo
// structure so that _pad, er, version is set to 0.
//
// A version value of 1 indicates that the size of the structure has
// been increased to include two UVA's: produce_uva and consume_uva.
// These UVA's are of the mmap()'d queue contents backing files.
//
// In addition, if when VMX is sending down the
// VMCIqueue_pairPageFileInfo structure it gets an error then it will
// try again with the _NoHostQP version of the file to see if an older
// VMCI kernel module is running.
//
// VMCIqueue_pairPageFileInfo
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_qp_page_file_info {
    pub handle: vmci_handle,
    pub /: *mut *mut u64 produce_page_file; / User VA.,
    pub /: *mut *mut u64 consume_page_file; / User VA.,
    pub /: *mut *mut u64 produce_page_file_size; / Size of the file name array.,
    pub /: *mut *mut u64 consume_page_file_size; / Size of the file name array.,
    pub result: i32,
    pub /: *mut *mut u32 version; / Was _pad.,
    pub /: *mut *mut u64 produce_va; / User VA of the mapped file.,
    pub /: *mut *mut u64 consume_va; / User VA of the mapped file.,
}

// vmci queuepair detach info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_qp_dtch_info {
    pub handle: vmci_handle,
    pub result: i32,
    pub _pad: u32,
}

//
// struct vmci_qp_page_store describes how the memory of a given queue pair
// is backed. When the queue pair is between the host and a guest, the
// page store consists of references to the guest pages. On vmkernel,
// this is a list of PPNs, and on hosted, it is a user VA where the
// queue pair is mapped into the VMX address space.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_qp_page_store {
// Reference to pages backing the queue pair.
    pub pages: u64,
// Length of pageList/virtual address range (in pages).
    pub len: u32,
}

//
// This data type contains the information about a queue.
// There are two queues (hence, queue pairs) per transaction model between a
// pair of end points, A & B.  One queue is used by end point A to transmit
// commands and responses to B.  The other queue is used by B to transmit
// commands and responses.
//
// struct vmci_queue_kern_if is a per-OS defined Queue structure.  It contains
// either a direct pointer to the linear address of the buffer contents or a
// pointer to structures which help the OS locate those data pages.  See
// vmciKernelIf.c for each platform for its definition.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_queue {
    pub q_header: *mut vmci_queue_header,
    pub saved_header: *mut vmci_queue_header,
    pub kernel_if: *mut vmci_queue_kern_if,
}

//
// Utility function that checks whether the fields of the page
// store contain valid values.
// Result:
// true if the page store is wellformed. false otherwise.
//
extern "C" {
    pub fn vmci_qp_broker_exit();
}
extern "C" {
    pub fn vmci_qp_broker_detach(handle: vmci_handle, context: *mut vmci_ctx) -> c_int;
}
extern "C" {
    pub fn vmci_qp_guest_endpoints_exit();
}
