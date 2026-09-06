//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdkfd/kfd_pm4_headers_vi.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Copyright 2014-2022 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//

// Macro flag: #define PM4_MES_HEADER_DEFINED
#[repr(C)]
#[derive(Copy, Clone)]
pub union PM4_MES_TYPE_3_HEADER {
    pub /: *mut *mut uint32_t reserved1 : 8; / < reserved,
    pub /: *mut *mut uint32_t opcode : 8; / < IT opcode,
    pub the: *mut *mut uint32_t count : 14;/ < Number of DWORDS - 1 in,
// information body
//
    pub identifier: *mut *mut uint32_t type : 2; / < packet,
// It should be 3 for type 3 packets
//
}

// --------------------MES_SET_RESOURCES--------------------

// Macro flag: #define PM4_MES_SET_RESOURCES_DEFINED
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mes_set_resources_queue_type_enum {
    queue_type__mes_set_resources__kernel_interface_queue_kiq = 0,
    queue_type__mes_set_resources__hsa_interface_queue_hiq = 1,
    queue_type__mes_set_resources__hsa_debug_interface_queue = 4
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm4_mes_set_resources {
    pub /: *mut *mut PM4_MES_TYPE_3_HEADER header; / header,
    pub ordinal1: u32,
}

// --------------------MES_RUN_LIST--------------------

// Macro flag: #define PM4_MES_RUN_LIST_DEFINED
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm4_mes_runlist {
    pub /: *mut *mut PM4_MES_TYPE_3_HEADER header; / header,
    pub ordinal1: u32,
}

// --------------------MES_MAP_PROCESS--------------------

// Macro flag: #define PM4_MES_MAP_PROCESS_DEFINED
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm4_mes_map_process {
    pub /: *mut *mut PM4_MES_TYPE_3_HEADER header; / header,
    pub ordinal1: u32,
}

// --------------------MES_MAP_QUEUES--------------------

// Macro flag: #define PM4_MES_MAP_QUEUES_VI_DEFINED
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mes_map_queues_queue_sel_vi_enum {
    queue_sel__mes_map_queues__map_to_specified_queue_slots_vi = 0,
    queue_sel__mes_map_queues__map_to_hws_determined_queue_slots_vi = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mes_map_queues_queue_type_vi_enum {
    queue_type__mes_map_queues__normal_compute_vi = 0,
    queue_type__mes_map_queues__debug_interface_queue_vi = 1,
    queue_type__mes_map_queues__normal_latency_static_queue_vi = 2,
    queue_type__mes_map_queues__low_latency_static_queue_vi = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mes_map_queues_engine_sel_vi_enum {
    engine_sel__mes_map_queues__compute_vi = 0,
    engine_sel__mes_map_queues__sdma0_vi = 2,
    engine_sel__mes_map_queues__sdma1_vi = 3
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm4_mes_map_queues {
    pub /: *mut *mut PM4_MES_TYPE_3_HEADER header; / header,
    pub ordinal1: u32,
}

// --------------------MES_QUERY_STATUS--------------------

// Macro flag: #define PM4_MES_QUERY_STATUS_DEFINED
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mes_query_status_interrupt_sel_enum {
    interrupt_sel__mes_query_status__completion_status = 0,
    interrupt_sel__mes_query_status__process_status = 1,
    interrupt_sel__mes_query_status__queue_status = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mes_query_status_command_enum {
    command__mes_query_status__interrupt_only = 0,
    command__mes_query_status__fence_only_immediate = 1,
    command__mes_query_status__fence_only_after_write_ack = 2,
    command__mes_query_status__fence_wait_for_write_ack_send_interrupt = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mes_query_status_engine_sel_enum {
    engine_sel__mes_query_status__compute = 0,
    engine_sel__mes_query_status__sdma0_queue = 2,
    engine_sel__mes_query_status__sdma1_queue = 3
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm4_mes_query_status {
    pub /: *mut *mut PM4_MES_TYPE_3_HEADER header; / header,
    pub ordinal1: u32,
}

// --------------------MES_UNMAP_QUEUES--------------------

// Macro flag: #define PM4_MES_UNMAP_QUEUES_DEFINED
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mes_unmap_queues_action_enum {
    action__mes_unmap_queues__preempt_queues = 0,
    action__mes_unmap_queues__reset_queues = 1,
    action__mes_unmap_queues__disable_process_queues = 2,
    action__mes_unmap_queues__reserved = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mes_unmap_queues_queue_sel_enum {
    queue_sel__mes_unmap_queues__perform_request_on_specified_queues = 0,
    queue_sel__mes_unmap_queues__perform_request_on_pasid_queues = 1,
    queue_sel__mes_unmap_queues__unmap_all_queues = 2,
    queue_sel__mes_unmap_queues__unmap_all_non_static_queues = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mes_unmap_queues_engine_sel_enum {
    engine_sel__mes_unmap_queues__compute = 0,
    engine_sel__mes_unmap_queues__sdma0 = 2,
    engine_sel__mes_unmap_queues__sdmal = 3
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm4_mes_unmap_queues {
    pub /: *mut *mut PM4_MES_TYPE_3_HEADER header; / header,
    pub ordinal1: u32,
}

// Macro flag: #define PM4_MEC_RELEASE_MEM_DEFINED
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RELEASE_MEM_event_index_enum {
    event_index___release_mem__end_of_pipe = 5,
    event_index___release_mem__shader_done = 6
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RELEASE_MEM_cache_policy_enum {
    cache_policy___release_mem__lru = 0,
    cache_policy___release_mem__stream = 1,
    cache_policy___release_mem__bypass = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RELEASE_MEM_dst_sel_enum {
    dst_sel___release_mem__memory_controller = 0,
    dst_sel___release_mem__tc_l2 = 1,
    dst_sel___release_mem__queue_write_pointer_register = 2,
    dst_sel___release_mem__queue_write_pointer_poll_mask_bit = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RELEASE_MEM_int_sel_enum {
    int_sel___release_mem__none = 0,
    int_sel___release_mem__send_interrupt_only = 1,
    int_sel___release_mem__send_interrupt_after_write_confirm = 2,
    int_sel___release_mem__send_data_after_write_confirm = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RELEASE_MEM_data_sel_enum {
    data_sel___release_mem__none = 0,
    data_sel___release_mem__send_32_bit_low = 1,
    data_sel___release_mem__send_64_bit_data = 2,
    data_sel___release_mem__send_gpu_clock_counter = 3,
    data_sel___release_mem__send_cp_perfcounter_hi_lo = 4,
    data_sel___release_mem__store_gds_data_to_memory = 5
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm4_mec_release_mem {
    pub /: *mut *mut PM4_MES_TYPE_3_HEADER header; /header,
    pub ordinal1: c_uint,
}

