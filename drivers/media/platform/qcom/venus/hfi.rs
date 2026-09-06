//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/venus/hfi.h
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
// Copyright (c) 2012-2016, The Linux Foundation. All rights reserved.
// Copyright (C) 2017 Linaro Ltd.
//

pub const VIDC_SESSION_TYPE_VPE: c_int = 0;
pub const VIDC_SESSION_TYPE_ENC: c_int = 1;
pub const VIDC_SESSION_TYPE_DEC: c_int = 2;
pub const VIDC_RESOURCE_NONE: c_int = 0;
pub const VIDC_RESOURCE_OCMEM: c_int = 1;
pub const VIDC_RESOURCE_VMEM: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_buffer_desc {
    pub buffer_type: u32,
    pub buffer_size: u32,
    pub num_buffers: u32,
    pub device_addr: u32,
    pub extradata_addr: u32,
    pub extradata_size: u32,
    pub response_required: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_frame_data {
    pub buffer_type: u32,
    pub device_addr: u32,
    pub extradata_addr: u32,
    pub timestamp: u64,
    pub flags: u32,
    pub offset: u32,
    pub alloc_len: u32,
    pub filled_len: u32,
    pub mark_target: u32,
    pub mark_data: u32,
    pub clnt_data: u32,
    pub extradata_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hfi_get_property {
    pub profile_level: hfi_profile_level,
    pub bufreq: [hfi_buffer_requirements; HFI_BUFFER_TYPE_MAX],
}

// HFI events
pub const EVT_SYS_EVENT_CHANGE: c_int = 1;
pub const EVT_SYS_WATCHDOG_TIMEOUT: c_int = 2;
pub const EVT_SYS_ERROR: c_int = 3;
pub const EVT_SESSION_ERROR: c_int = 4;
// HFI event callback structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_event_data {
    pub error: u32,
    pub height: u32,
    pub width: u32,
    pub event_type: u32,
    pub packet_buffer: u32,
    pub extradata_buffer: u32,
    pub tag: u32,
    pub profile: u32,
    pub level: u32,
// the following properties start appear from v4 onwards
    pub bit_depth: u32,
    pub pic_struct: u32,
    pub colour_space: u32,
    pub entropy_mode: u32,
    pub buf_count: u32,
    pub top: u32 left,,
    pub height: u32 width,,
    pub input_crop: },
}

// define core states
pub const CORE_UNINIT: c_int = 0;
pub const CORE_INIT: c_int = 1;
// define instance states
pub const INST_UNINIT: c_int = 2;
pub const INST_INIT: c_int = 3;
pub const INST_LOAD_RESOURCES: c_int = 4;
pub const INST_START: c_int = 5;
pub const INST_STOP: c_int = 6;
pub const INST_RELEASE_RESOURCES: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_core_ops {
    pub event): *mut *mut *mut void (event_notify)(struct venus_core core, u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_inst_ops {
    pub timestamp_us): u32 hfi_flags, u64,
    pub data): *mut hfi_event_data,
    pub inst): *mut *mut void (flush_done)(struct venus_inst,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_ops {
    pub core): *mut *mut int (core_init)(struct venus_core,
    pub core): *mut *mut int (core_deinit)(struct venus_core,
    pub trigger_type): *mut *mut *mut int (core_trigger_ssr)(struct venus_core core, u32,
    pub codec): u32,
    pub inst): *mut *mut int (session_end)(struct venus_inst,
    pub inst): *mut *mut int (session_abort)(struct venus_inst,
    pub flush_mode): *mut *mut *mut int (session_flush)(struct venus_inst inst, u32,
    pub inst): *mut *mut int (session_start)(struct venus_inst,
    pub inst): *mut *mut int (session_stop)(struct venus_inst,
    pub inst): *mut *mut int (session_continue)(struct venus_inst,
    pub fd): *mut *mut *mut int (session_etb)(struct venus_inst inst, struct hfi_frame_data,
    pub fd): *mut *mut *mut int (session_ftb)(struct venus_inst inst, struct hfi_frame_data,
    pub bd): *mut hfi_buffer_desc,
    pub bd): *mut hfi_buffer_desc,
    pub inst): *mut *mut int (session_load_res)(struct venus_inst,
    pub inst): *mut *mut int (session_release_res)(struct venus_inst,
    pub seq_hdr_len): u32,
    pub seq_hdr_len): u32,
    pub pdata): *mut c_void,
    pub ptype): *mut *mut *mut int (session_get_property)(struct venus_inst inst, u32,
    pub core): *mut *mut int (resume)(struct venus_core,
    pub core): *mut *mut int (suspend)(struct venus_core,
// interrupt operations
    pub core): *mut *mut irqreturn_t (isr)(struct venus_core,
    pub core): *mut *mut irqreturn_t (isr_thread)(struct venus_core,
}

extern "C" {
    pub fn hfi_create(core: *mut venus_core, ops: *const hfi_core_ops) -> c_int;
}
extern "C" {
    pub fn hfi_destroy(core: *mut venus_core);
}
extern "C" {
    pub fn hfi_reinit(core: *mut venus_core);
}
extern "C" {
    pub fn hfi_core_init(core: *mut venus_core) -> c_int;
}
extern "C" {
    pub fn hfi_core_deinit(core: *mut venus_core, blocking: bool) -> c_int;
}
extern "C" {
    pub fn hfi_core_suspend(core: *mut venus_core) -> c_int;
}
extern "C" {
    pub fn hfi_core_resume(core: *mut venus_core, force: bool) -> c_int;
}
extern "C" {
    pub fn hfi_core_trigger_ssr(core: *mut venus_core, type: u32) -> c_int;
}
extern "C" {
    pub fn hfi_session_create(inst: *mut venus_inst, ops: *const hfi_inst_ops) -> c_int;
}
extern "C" {
    pub fn hfi_session_destroy(inst: *mut venus_inst);
}
extern "C" {
    pub fn hfi_session_init(inst: *mut venus_inst, pixfmt: u32) -> c_int;
}
extern "C" {
    pub fn hfi_session_deinit(inst: *mut venus_inst) -> c_int;
}
extern "C" {
    pub fn hfi_session_start(inst: *mut venus_inst) -> c_int;
}
extern "C" {
    pub fn hfi_session_stop(inst: *mut venus_inst) -> c_int;
}
extern "C" {
    pub fn hfi_session_continue(inst: *mut venus_inst) -> c_int;
}
extern "C" {
    pub fn hfi_session_abort(inst: *mut venus_inst) -> c_int;
}
extern "C" {
    pub fn hfi_session_load_res(inst: *mut venus_inst) -> c_int;
}
extern "C" {
    pub fn hfi_session_unload_res(inst: *mut venus_inst) -> c_int;
}
extern "C" {
    pub fn hfi_session_flush(inst: *mut venus_inst, type: u32, block: bool) -> c_int;
}
extern "C" {
    pub fn hfi_session_set_property(inst: *mut venus_inst, ptype: u32, pdata: *mut c_void) -> c_int;
}
extern "C" {
    pub fn hfi_session_process_buf(inst: *mut venus_inst, f: *mut hfi_frame_data) -> c_int;
}
extern "C" {
    pub fn hfi_isr_thread(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn hfi_isr(irq: c_int, dev: *mut c_void) -> irqreturn_t;
}
