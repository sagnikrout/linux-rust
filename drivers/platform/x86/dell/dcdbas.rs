//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/x86/dell/dcdbas.h
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
// dcdbas.h: Definitions for Dell Systems Management Base driver
//
// Copyright (C) 1995-2005 Dell Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smi_cmd {
    pub magic: __u32,
    pub ebx: __u32,
    pub ecx: __u32,
    pub command_address: __u16,
    pub command_code: __u8,
    pub reserved: __u8,
    pub command_buffer: [__u8; 1],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_cmd {
    pub command: __u8,
    pub status: __s8,
    pub reserved: __u16,
    pub parm: [__u8; MAX_SYSMGMT_SHORTCMD_PARMBUF_LEN],
// C attribute field omitted
    pub num_sg_entries: __u16,
    pub size: __u32,
    pub addr: __u64,
// C attribute field omitted
// C attribute field omitted
// C attribute field omitted
    pub smi_cmd): *mut int dcdbas_smi_request(struct smi_cmd,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smm_eps_table {
    pub smm_comm_buff_anchor: [c_char; 4],
    pub length: u8,
    pub checksum: u8,
    pub version: u8,
    pub smm_comm_buff_addr: u64,
    pub num_of_4k_pages: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smi_buffer {
    pub virt: *mut u8,
    pub size: c_ulong,
    pub dma: dma_addr_t,
}

extern "C" {
    pub fn dcdbas_smi_alloc(smi_buffer: *mut smi_buffer, size: c_ulong) -> c_int;
}
extern "C" {
    pub fn dcdbas_smi_free(smi_buffer: *mut smi_buffer);
}
