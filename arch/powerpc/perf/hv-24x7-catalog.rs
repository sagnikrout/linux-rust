//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/perf/hv-24x7-catalog.h
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

// From document "24x7 Event and Group Catalog Formats Proposal" v0.15
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_24x7_catalog_page_0 {
pub const HV_24X7_CATALOG_MAGIC: c_uint = 0x32347837 /* "24x7" in ASCII */;
    pub magic: __be32,
    pub /: *mut *mut __be32 length; / In 4096 byte pages,
    pub /: *mut *mut __be64 version; / XXX: arbitrary? what's the meaning/useage/purpose?,
    pub /: *mut *mut __u8 build_time_stamp[16]; / "YYYYMMDDHHMMSS\0\0",
    pub reserved2: [__u8; 32],
    pub /: *mut *mut __be16 schema_data_offs; / in 4096 byte pages,
    pub /: *mut *mut __be16 schema_data_len; / in 4096 byte pages,
    pub schema_entry_count: __be16,
    pub reserved3: [__u8; 2],
    pub event_data_offs: __be16,
    pub event_data_len: __be16,
    pub event_entry_count: __be16,
    pub reserved4: [__u8; 2],
    pub /: *mut *mut __be16 group_data_offs; / in 4096 byte pages,
    pub /: *mut *mut __be16 group_data_len; / in 4096 byte pages,
    pub group_entry_count: __be16,
    pub reserved5: [__u8; 2],
    pub /: *mut *mut __be16 formula_data_offs; / in 4096 byte pages,
    pub /: *mut *mut __be16 formula_data_len; / in 4096 byte pages,
    pub formula_entry_count: __be16,
    pub reserved6: [__u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_24x7_event_data {
    pub /: *mut *mut __be16 length; / in bytes, must be a multiple of 16,
    pub reserved1: [__u8; 2],
    pub /: *mut *mut __u8 domain; / Chip = 1, Core = 2,
    pub reserved2: [__u8; 1],
    pub /: *mut *mut __be16 event_group_record_offs; / in bytes, must be 8 byte aligned,
    pub /: *mut *mut __be16 event_group_record_len; / in bytes,
// in bytes, offset from event_group_record
    pub event_counter_offs: __be16,
// verified_state, unverified_state, caveat_state, broken_state, ...
    pub flags: __be32,
    pub primary_group_ix: __be16,
    pub group_count: __be16,
    pub event_name_len: __be16,
    pub remainder: [__u8; ],
// __u8 event_name[event_name_len - 2];
// __be16 event_description_len;
// __u8 event_desc[event_description_len - 2];
// __be16 detailed_desc_len;
// __u8 detailed_desc[detailed_desc_len - 2];
    pub __packed: },
