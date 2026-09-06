//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/tool.h
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
pub enum show_feature_header {
    SHOW_FEAT_NO_HEADER = 0,
    SHOW_FEAT_HEADER,
    SHOW_FEAT_HEADER_FULL_INFO,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_tool {
    pub attr: event_attr_op,
    pub event_update: event_attr_op,
    pub tracing_data: event_op2,
    pub finished_round: event_oe,
    pub compressed: event_op4,
    pub auxtrace: event_op3,
    pub ordered_events: bool,
    pub ordering_requires_timestamps: bool,
    pub namespace_events: bool,
    pub cgroup_events: bool,
    pub no_warn: bool,
    pub dont_split_sample_group: bool,
    pub merge_deferred_callchains: bool,
    pub show_feat_hdr: show_feature_header,
}

extern "C" {
    pub fn perf_tool__init(tool: *mut perf_tool, ordered_events: bool);
}
extern "C" {
    pub fn perf_tool__compressed_is_stub(tool: *const perf_tool) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct delegate_tool {
// @tool: The actual tool that calls the delegate.
    pub tool: perf_tool,
// @delegate: The tool that is delegated to.
    pub delegate: *mut perf_tool,
}

extern "C" {
    pub fn delegate_tool__init(tool: *mut delegate_tool, delegate: *mut perf_tool);
}
