//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/qcom/qdsp6/q6apm.h
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

pub const APM_PORT_MAX_AUDIO_CHAN_CNT: c_int = 8;
pub const PCM_CHANNEL_NULL: c_int = 0;

pub const APM_TIMESTAMP_FLAG: c_uint = 0x80000000;
pub const FORMAT_LINEAR_PCM: c_uint = 0x0000;
// APM client callback events
pub const APM_CMD_EOS: c_uint = 0x0003;
pub const APM_CLIENT_EVENT_CMD_EOS_DONE: c_uint = 0x1003;
pub const APM_CMD_CLOSE: c_uint = 0x0004;
pub const APM_CLIENT_EVENT_CMD_CLOSE_DONE: c_uint = 0x1004;
pub const APM_CLIENT_EVENT_CMD_RUN_DONE: c_uint = 0x1008;
pub const APM_CLIENT_EVENT_DATA_WRITE_DONE: c_uint = 0x1009;
pub const APM_CLIENT_EVENT_DATA_READ_DONE: c_uint = 0x100a;
pub const APM_CLIENT_EVENT_WATERMARK_EVENT: c_uint = 0x100b;

pub const APM_WRITE_TOKEN_LEN_SHIFT: c_int = 16;
pub const APM_MAX_SESSIONS: c_int = 8;

pub const NO_TIMESTAMP: c_uint = 0xFF00;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct q6apm {
    pub dev: *mut device,
    pub port: *mut gpr_port_t,
    pub gdev: *mut gpr_device_t,
// For Graph OPEN/START/STOP/CLOSE operations
    pub wait: wait_queue_head_t,
    pub result: gpr_ibasic_rsp_result_t,
    pub cmd_lock: mutex,
    pub lock: mutex,
    pub state: u32,
    pub widget_list: list_head,
    pub graph_idr: idr,
    pub graph_info_idr: idr,
    pub sub_graphs_idr: idr,
    pub containers_idr: idr,
    pub modules_idr: idr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audio_buffer {
    pub phys: phys_addr_t,
    pub /: *mut *mut uint32_t size; / size of buffer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audioreach_graph_data {
    pub buf: *mut audio_buffer,
    pub num_periods: u32,
    pub dsp_buf: u32,
    pub hw_ptr: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audioreach_graph {
    pub info: *mut audioreach_graph_info,
    pub id: u32,
    pub state: c_int,
    pub start_count: c_int,
// Cached Graph data
    pub graph: *mut c_void,
    pub refcount: kref,
    pub apm: *mut q6apm,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct q6apm_graph {
    pub priv: *mut c_void,
    pub cb: q6apm_cb,
    pub id: u32,
    pub shm_iid: u32,
    pub dev: *mut device,
    pub apm: *mut q6apm,
    pub port: *mut gpr_port_t,
    pub rx_data: audioreach_graph_data,
    pub tx_data: audioreach_graph_data,
    pub result: gpr_ibasic_rsp_result_t,
    pub cmd_wait: wait_queue_head_t,
    pub lock: mutex,
    pub ar_graph: *mut audioreach_graph,
    pub info: *mut audioreach_graph_info,
}

// Graph Operations
extern "C" {
    pub fn q6apm_graph_close(graph: *mut q6apm_graph) -> c_int;
}
extern "C" {
    pub fn q6apm_graph_prepare(graph: *mut q6apm_graph) -> c_int;
}
extern "C" {
    pub fn q6apm_graph_start(graph: *mut q6apm_graph) -> c_int;
}
extern "C" {
    pub fn q6apm_graph_stop(graph: *mut q6apm_graph) -> c_int;
}
extern "C" {
    pub fn q6apm_graph_flush(graph: *mut q6apm_graph) -> c_int;
}
// Media Format
// read/write related
extern "C" {
    pub fn q6apm_read(graph: *mut q6apm_graph) -> c_int;
}
// Memory Map related
extern "C" {
    pub fn q6apm_unmap_pos_buffer(dev: *mut device, graph_id: c_uint) -> c_int;
}
extern "C" {
    pub fn q6apm_free_fragments(graph: *mut q6apm_graph, dir: c_uint) -> c_int;
}
extern "C" {
    pub fn q6apm_unmap_memory_fixed_region(dev: *mut device, graph_id: c_uint) -> c_int;
}
// Helpers
// Callback for graph specific
extern "C" {
    pub fn q6apm_is_adsp_ready() -> bool;
}
extern "C" {
    pub fn q6apm_enable_compress_module(dev: *mut device, graph: *mut q6apm_graph, en: bool) -> c_int;
}
extern "C" {
    pub fn q6apm_remove_initial_silence(dev: *mut device, graph: *mut q6apm_graph, samples: u32) -> c_int;
}
extern "C" {
    pub fn q6apm_remove_trailing_silence(dev: *mut device, graph: *mut q6apm_graph, samples: u32) -> c_int;
}
extern "C" {
    pub fn q6apm_set_real_module_id(dev: *mut device, graph: *mut q6apm_graph, codec_id: u32) -> c_int;
}
extern "C" {
    pub fn q6apm_get_hw_pointer(graph: *mut q6apm_graph, dir: c_int) -> c_int;
}
extern "C" {
    pub fn q6apm_is_graph_in_push_pull_mode(graph: *mut q6apm_graph) -> bool;
}
extern "C" {
    pub fn q6apm_is_graph_in_push_pull_mode_from_id(dev: *mut device, graph_id: c_uint, dir: c_int) -> bool;
}
extern "C" {
    pub fn q6apm_register_watermark_event(graph: *mut q6apm_graph, watermark_bytes: c_int, num_levels: c_int) -> c_int;
}
