//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/uas.h
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

// Common header for all IUs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iu {
    pub iu_id: __u8,
    pub rsvd1: __u8,
    pub tag: __be16,
    pub __attribute__((__packed__)): },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct command_iu {
    pub iu_id: __u8,
    pub rsvd1: __u8,
    pub tag: __be16,
    pub prio_attr: __u8,
    pub rsvd5: __u8,
    pub len: __u8,
    pub rsvd7: __u8,
    pub lun: scsi_lun,
    pub /: *mut *mut __u8 cdb[16]; / XXX: Overflow-checking tools may misunderstand,
    pub __attribute__((__packed__)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_mgmt_iu {
    pub iu_id: __u8,
    pub rsvd1: __u8,
    pub tag: __be16,
    pub function: __u8,
    pub rsvd2: __u8,
    pub task_tag: __be16,
    pub lun: scsi_lun,
    pub __attribute__((__packed__)): },
//
// Also used for the Read Ready and Write Ready IUs since they have the
// same first four bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sense_iu {
    pub iu_id: __u8,
    pub rsvd1: __u8,
    pub tag: __be16,
    pub status_qual: __be16,
    pub status: __u8,
    pub rsvd7: [__u8; 7],
    pub len: __be16,
    pub sense: [__u8; SCSI_SENSE_BUFFERSIZE],
    pub __attribute__((__packed__)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct response_iu {
    pub iu_id: __u8,
    pub rsvd1: __u8,
    pub tag: __be16,
    pub add_response_info: [__u8; 3],
    pub response_code: __u8,
    pub __attribute__((__packed__)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_pipe_usage_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bPipeID: __u8,
    pub Reserved: __u8,
    pub __attribute__((__packed__)): },
}
