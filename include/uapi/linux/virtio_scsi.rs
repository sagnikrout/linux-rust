//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/virtio_scsi.h
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
// This header is BSD licensed so anyone can use the definitions to implement
// compatible drivers/servers.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY AUTHOR AND CONTRIBUTORS ``AS IS'' AND
// ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED.  IN NO EVENT SHALL AUTHOR OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
// OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
// LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
// OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
// SUCH DAMAGE.
//

// Default values of the CDB and sense data size configuration fields
pub const VIRTIO_SCSI_CDB_DEFAULT_SIZE: c_int = 32;
pub const VIRTIO_SCSI_SENSE_DEFAULT_SIZE: c_int = 96;

// SCSI command request, followed by data-out
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_scsi_cmd_req {
    pub /: *mut *mut __u8 lun[8]; / Logical Unit Number,
    pub /: *mut *mut __virtio64 tag; / Command identifier,
    pub /: *mut *mut __u8 task_attr; / Task attribute,
    pub /: *mut *mut __u8 prio; / SAM command priority field,
    pub crn: __u8,
    pub cdb: [__u8; VIRTIO_SCSI_CDB_SIZE],
    pub __attribute__((packed)): },
// SCSI command request, followed by protection information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_scsi_cmd_req_pi {
    pub /: *mut *mut __u8 lun[8]; / Logical Unit Number,
    pub /: *mut *mut __virtio64 tag; / Command identifier,
    pub /: *mut *mut __u8 task_attr; / Task attribute,
    pub /: *mut *mut __u8 prio; / SAM command priority field,
    pub crn: __u8,
    pub /: *mut *mut __virtio32 pi_bytesout; / DataOUT PI Number of bytes,
    pub /: *mut *mut __virtio32 pi_bytesin; / DataIN PI Number of bytes,
    pub cdb: [__u8; VIRTIO_SCSI_CDB_SIZE],
    pub __attribute__((packed)): },
// Response, followed by sense data and data-in
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_scsi_cmd_resp {
    pub /: *mut *mut __virtio32 sense_len; / Sense data length,
    pub /: *mut *mut __virtio32 resid; / Residual bytes in data buffer,
    pub /: *mut *mut __virtio16 status_qualifier; / Status qualifier,
    pub /: *mut *mut __u8 status; / Command completion status,
    pub /: *mut *mut __u8 response; / Response values,
    pub sense: [__u8; VIRTIO_SCSI_SENSE_SIZE],
    pub __attribute__((packed)): },
// Task Management Request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_scsi_ctrl_tmf_req {
    pub type: __virtio32,
    pub subtype: __virtio32,
    pub lun: [__u8; 8],
    pub tag: __virtio64,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_scsi_ctrl_tmf_resp {
    pub response: __u8,
    pub __attribute__((packed)): },
// Asynchronous notification query/subscription
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_scsi_ctrl_an_req {
    pub type: __virtio32,
    pub lun: [__u8; 8],
    pub event_requested: __virtio32,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_scsi_ctrl_an_resp {
    pub event_actual: __virtio32,
    pub response: __u8,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_scsi_event {
    pub event: __virtio32,
    pub lun: [__u8; 8],
    pub reason: __virtio32,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_scsi_config {
    pub num_queues: __virtio32,
    pub seg_max: __virtio32,
    pub max_sectors: __virtio32,
    pub cmd_per_lun: __virtio32,
    pub event_info_size: __virtio32,
    pub sense_size: __virtio32,
    pub cdb_size: __virtio32,
    pub max_channel: __virtio16,
    pub max_target: __virtio16,
    pub max_lun: __virtio32,
    pub __attribute__((packed)): },
// Feature Bits
pub const VIRTIO_SCSI_F_INOUT: c_int = 0;
pub const VIRTIO_SCSI_F_HOTPLUG: c_int = 1;
pub const VIRTIO_SCSI_F_CHANGE: c_int = 2;
pub const VIRTIO_SCSI_F_T10_PI: c_int = 3;
// Response codes
pub const VIRTIO_SCSI_S_OK: c_int = 0;
pub const VIRTIO_SCSI_S_OVERRUN: c_int = 1;
pub const VIRTIO_SCSI_S_ABORTED: c_int = 2;
pub const VIRTIO_SCSI_S_BAD_TARGET: c_int = 3;
pub const VIRTIO_SCSI_S_RESET: c_int = 4;
pub const VIRTIO_SCSI_S_BUSY: c_int = 5;
pub const VIRTIO_SCSI_S_TRANSPORT_FAILURE: c_int = 6;
pub const VIRTIO_SCSI_S_TARGET_FAILURE: c_int = 7;
pub const VIRTIO_SCSI_S_NEXUS_FAILURE: c_int = 8;
pub const VIRTIO_SCSI_S_FAILURE: c_int = 9;
pub const VIRTIO_SCSI_S_FUNCTION_SUCCEEDED: c_int = 10;
pub const VIRTIO_SCSI_S_FUNCTION_REJECTED: c_int = 11;
pub const VIRTIO_SCSI_S_INCORRECT_LUN: c_int = 12;
// Controlq type codes.
pub const VIRTIO_SCSI_T_TMF: c_int = 0;
pub const VIRTIO_SCSI_T_AN_QUERY: c_int = 1;
pub const VIRTIO_SCSI_T_AN_SUBSCRIBE: c_int = 2;
// Valid TMF subtypes.
pub const VIRTIO_SCSI_T_TMF_ABORT_TASK: c_int = 0;
pub const VIRTIO_SCSI_T_TMF_ABORT_TASK_SET: c_int = 1;
pub const VIRTIO_SCSI_T_TMF_CLEAR_ACA: c_int = 2;
pub const VIRTIO_SCSI_T_TMF_CLEAR_TASK_SET: c_int = 3;
pub const VIRTIO_SCSI_T_TMF_I_T_NEXUS_RESET: c_int = 4;
pub const VIRTIO_SCSI_T_TMF_LOGICAL_UNIT_RESET: c_int = 5;
pub const VIRTIO_SCSI_T_TMF_QUERY_TASK: c_int = 6;
pub const VIRTIO_SCSI_T_TMF_QUERY_TASK_SET: c_int = 7;
// Events.
pub const VIRTIO_SCSI_T_EVENTS_MISSED: c_uint = 0x80000000;
pub const VIRTIO_SCSI_T_NO_EVENT: c_int = 0;
pub const VIRTIO_SCSI_T_TRANSPORT_RESET: c_int = 1;
pub const VIRTIO_SCSI_T_ASYNC_NOTIFY: c_int = 2;
pub const VIRTIO_SCSI_T_PARAM_CHANGE: c_int = 3;
// Reasons of transport reset event
pub const VIRTIO_SCSI_EVT_RESET_HARD: c_int = 0;
pub const VIRTIO_SCSI_EVT_RESET_RESCAN: c_int = 1;
pub const VIRTIO_SCSI_EVT_RESET_REMOVED: c_int = 2;
pub const VIRTIO_SCSI_S_SIMPLE: c_int = 0;
pub const VIRTIO_SCSI_S_ORDERED: c_int = 1;
pub const VIRTIO_SCSI_S_HEAD: c_int = 2;
pub const VIRTIO_SCSI_S_ACA: c_int = 3;
