//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/ibmvscsi_tgt/libsrp.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum srp_valid {
    INVALIDATE_CMD_RESP_EL = 0,
    VALID_CMD_RESP_EL = 0x80,
    VALID_INIT_MSG = 0xC0,
    VALID_TRANS_EVENT = 0xFF
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum srp_format {
    SRP_FORMAT = 1,
    MAD_FORMAT = 2,
    OS400_FORMAT = 3,
    AIX_FORMAT = 4,
    LINUX_FORMAT = 5,
    MESSAGE_IN_CRQ = 6
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum srp_init_msg {
    INIT_MSG = 1,
    INIT_COMPLETE_MSG = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum srp_trans_event {
    UNUSED_FORMAT = 0,
    PARTNER_FAILED = 1,
    PARTNER_DEREGISTER = 2,
    MIGRATED = 6,
    PREPARE_FOR_SUSPEND = 9,
    RESUME_FROM_SUSP = 0xA
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum srp_status {
    CRQ_ENTRY_OVERWRITTEN = 0x20,
    HEADER_DESCRIPTOR = 0xF1,
    PING = 0xF5,
    PING_RESPONSE = 0xF6
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum srp_mad_version {
    MAD_VERSION_1 = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum srp_os_type {
    OS400 = 1,
    LINUX = 2,
    AIX = 3,
    OFW = 4
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum srp_task_attributes {
    SRP_SIMPLE_TASK = 0,
    SRP_HEAD_TASK = 1,
    SRP_ORDERED_TASK = 2,
    SRP_ACA_TASK = 4
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct srp_buf {
    pub dma: dma_addr_t,
    pub buf: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct srp_queue {
    pub pool: *mut c_void,
    pub items: *mut c_void,
    pub queue: kfifo,
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct srp_target {
    pub dev: *mut device,
    pub lock: spinlock_t,
    pub cmd_queue: list_head,
    pub srp_iu_size: usize,
    pub iu_queue: srp_queue,
    pub rx_ring_size: usize,
    pub rx_ring: *mut srp_buf,
    pub ldata: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iu_entry {
    pub target: *mut srp_target,
    pub ilist: list_head,
    pub remote_token: dma_addr_t,
    pub flags: c_ulong,
    pub sbuf: *mut srp_buf,
    pub iu_len: u16,
}

extern "C" {
    pub fn srp_target_alloc(: *mut srp_target, : *mut device, _arg: usize, _arg: usize) -> c_int;
}
extern "C" {
    pub fn srp_target_free(: *mut srp_target);
}
extern "C" {
    pub fn srp_iu_put(: *mut iu_entry);
}
extern "C" {
    pub fn srp_data_length(cmd: *mut srp_cmd, dir: dma_data_direction) -> u64;
}
