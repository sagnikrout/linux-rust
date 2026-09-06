//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/perf/hv-24x7.h
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
pub enum hv_perf_domains {

    HV_PERF_DOMAIN_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_24x7_request {
// PHYSICAL domains require enabling via phyp/hmc.
    pub performance_domain: __u8,
    pub reserved: [__u8; 0x1],
// bytes to read starting at @data_offset. must be a multiple of 8
    pub data_size: __be16,
//
// byte offset within the perf domain to read from. must be 8 byte
// aligned
//
    pub data_offset: __be32,
//
// only valid for VIRTUAL_PROCESSOR domains, ignored for others.
// -1 means "current partition only"
// Enabling via phyp/hmc required for non-"-1" values. 0 forbidden
// unless requestor is 0.
//
    pub starting_lpar_ix: __be16,
//
// Ignored when @starting_lpar_ix == -1
// Ignored when @performance_domain is not VIRTUAL_PROCESSOR_
// -1 means "infinite" or all
//
    pub max_num_lpars: __be16,
// chip, core, or virtual processor based on @performance_domain
    pub starting_ix: __be16,
    pub max_ix: __be16,
// The following fields were added in v2 of the 24x7 interface.
    pub starting_thread_group_ix: __u8,
// -1 means all thread groups starting at @starting_thread_group_ix
    pub max_num_thread_groups: __u8,
    pub reserved2: [__u8; 0xE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_24x7_request_buffer {
// 0 - ?
// 1 - ?
    pub interface_version: __u8,
    pub num_requests: __u8,
    pub reserved: [__u8; 0xE],
    pub requests: [hv_24x7_request; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_24x7_result_element_v1 {
    pub lpar_ix: __be16,
//
// represents the core, chip, or virtual processor based on the
// request's @performance_domain
//
    pub domain_ix: __be16,
// -1 if @performance_domain does not refer to a virtual processor
    pub lpar_cfg_instance_id: __be32,
// size = @result_element_data_size of containing result.
    pub element_data: [__u64; ],
    pub __packed: },
//
// We need a separate struct for v2 because the offset of @element_data changed
// between versions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_24x7_result_element_v2 {
    pub lpar_ix: __be16,
//
// represents the core, chip, or virtual processor based on the
// request's @performance_domain
//
    pub domain_ix: __be16,
// -1 if @performance_domain does not refer to a virtual processor
    pub lpar_cfg_instance_id: __be32,
    pub thread_group_ix: __u8,
    pub reserved: [__u8; 7],
// size = @result_element_data_size of containing result.
    pub element_data: [__u64; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_24x7_result {
//
// The index of the 24x7 Request Structure in the 24x7 Request Buffer
// used to request this result.
//
    pub result_ix: __u8,
//
// 0 = not all result elements fit into the buffer, additional requests
// required
// 1 = all result elements were returned
//
    pub results_complete: __u8,
    pub num_elements_returned: __be16,
//
// This is a copy of @data_size from the corresponding hv_24x7_request
//
// Warning: to obtain the size of each element in @elements you have
// to add the size of the other members of the result_element struct.
//
    pub result_element_data_size: __be16,
    pub reserved: [__u8; 0x2],
//
// Either
// struct hv_24x7_result_element_v1[@num_elements_returned]
// or
// struct hv_24x7_result_element_v2[@num_elements_returned]
//
// depending on the interface_version field of the
// struct hv_24x7_data_result_buffer containing this result.
//
    pub elements: [c_char; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_24x7_data_result_buffer {
// See versioning for request buffer
    pub interface_version: __u8,
    pub num_results: __u8,
    pub reserved: [__u8; 0x1],
    pub failing_request_ix: __u8,
    pub detailed_rc: __be32,
    pub cec_cfg_instance_id: __be64,
    pub catalog_version_num: __be64,
    pub reserved2: [__u8; 0x8],
// WARNING: only valid for the first result due to variable sizes of
// results
    pub /: *mut *mut hv_24x7_result results[]; / [@num_results],
    pub __packed: },
