//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/perf/hv-gpci-requests.h
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

//
// Based on the document "getPerfCountInfo v1.07"
//
// #define REQUEST_NAME counter_request_name
// #define REQUEST_NUM r_num
// #define REQUEST_IDX_KIND starting_index_kind
// #include I(REQUEST_BEGIN)
// REQUEST(
// __field(...)
// __array(...)
// __count(...)
// )
// #include I(REQUEST_END)
//
// - starting_index_kind is one of the following, depending on the event:
//
// hw_chip_id: hardware chip id or -1 for current hw chip
// partition_id
// sibling_part_id,
// phys_processor_idx:
// 0xffffffffffffffff: or -1, which means it is irrelavant for the event
//
// __count(offset, bytes, name):
// a counter that should be exposed via perf
// __field(offset, bytes, name)
// a normal field
// __array(offset, bytes, name)
// an array of bytes
//
// @bytes for __count, and __field _must_ be a numeral token
// in decimal, not an expression and not in hex.
//
// TODO:
// - expose secondary index (if any counter ever uses it, only 0xA0
// appears to use it right now, and it doesn't have any counters)
// - embed versioning info
// - include counter descriptions
//

pub const REQUEST_NUM: c_uint = 0x10;

pub const REQUEST_NUM: c_uint = 0x20;

//
// Not available for counter_info_version >= 0x8, use
// run_instruction_cycles_by_partition(0x100) instead.
//

pub const REQUEST_NUM: c_uint = 0x30;

pub const REQUEST_NUM: c_uint = 0x40;

pub const REQUEST_NUM: c_uint = 0x50;

pub const REQUEST_NUM: c_uint = 0x60;

pub const REQUEST_NUM: c_uint = 0x70;

pub const REQUEST_NUM: c_uint = 0x80;

// Processor_config (0x90) skipped, no counters
// Current_processor_frequency (0x91) skipped, no counters

pub const REQUEST_NUM: c_uint = 0x94;

// Processor_core_power_mode (0x95) skipped, no counters
// Affinity_domain_information_by_virtual_processor (0xA0) skipped,
// no counters
// Affinity_domain_information_by_domain (0xB0) skipped, no counters
// Affinity_domain_information_by_partition (0xB1) skipped, no counters
// Physical_memory_info (0xC0) skipped, no counters
// Processor_bus_topology (0xD0) skipped, no counters

pub const REQUEST_NUM: c_uint = 0xE0;

pub const REQUEST_NUM: c_uint = 0xF0;

pub const REQUEST_NUM: c_uint = 0xF4;

//
// FIXME: The spec says the offset here is 0x10, which I suspect
// is wrong.
//

pub const REQUEST_NUM: c_uint = 0x100;

// set_mmcrh (0x80001000) skipped, no counters
// retrieve_hpmcx (0x80002000) skipped, no counters
