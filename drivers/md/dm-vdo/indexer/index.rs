//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/indexer/index.h
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
// Copyright 2023 Red Hat
//

//
// The index is a high-level structure which represents the totality of the UDS index. It manages
// the queues for incoming requests and dispatches them to the appropriate sub-components like the
// volume or the volume index. It also manages administrative tasks such as saving and loading the
// index.
//
// The index is divided into a number of independent zones and assigns each request to a zone based
// on its name. Most sub-components are similarly divided into zones as well so that requests in
// each zone usually operate without interference or coordination between zones.
//
extern "C" {
    pub fn void(request: *mut *mut index_callback_fn)(struct uds_request) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct index_zone {
    pub index: *mut uds_index,
    pub open_chapter: *mut open_chapter_zone,
    pub writing_chapter: *mut open_chapter_zone,
    pub oldest_virtual_chapter: u64,
    pub newest_virtual_chapter: u64,
    pub id: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uds_index {
    pub has_saved_open_chapter: bool,
    pub need_to_save: bool,
    pub load_context: *mut index_load_context,
    pub layout: *mut index_layout,
    pub volume_index: *mut volume_index,
    pub volume: *mut volume,
    pub zone_count: c_uint,
    pub zones: *mut index_zone,
    pub oldest_virtual_chapter: u64,
    pub newest_virtual_chapter: u64,
    pub last_save: u64,
    pub prev_save: u64,
    pub chapter_writer: *mut chapter_writer,
    pub callback: index_callback_fn,
    pub triage_queue: *mut uds_request_queue,
    pub __counted_by(zone_count): *mut *mut uds_request_queue zone_queues[],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum request_stage {
    STAGE_TRIAGE,
    STAGE_INDEX,
    STAGE_MESSAGE,
}

extern "C" {
    pub fn uds_save_index(index: *mut uds_index) -> int __must_check;
}
extern "C" {
    pub fn uds_free_index(index: *mut uds_index);
}
extern "C" {
    pub fn uds_get_index_stats(index: *mut uds_index, counters: *mut uds_index_stats);
}
extern "C" {
    pub fn uds_enqueue_request(request: *mut uds_request, stage: request_stage);
}
extern "C" {
    pub fn uds_wait_for_idle_index(index: *mut uds_index);
}
