//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/cs-etm.h
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
// Copyright(C) 2015 Linaro Limited. All rights reserved.
// Author: Mathieu Poirier <mathieu.poirier@linaro.org>
//

// Macro flag: #define INCLUDE__UTIL_PERF_CS_ETM_H__

//
// Versioning header in case things need to change in the future.  That way
// decoding of old snapshot is still possible.
//
// Starting with 0x0
// PMU->type (32 bit), total # of CPUs (32 bit)
//
// Update the version for new format.
//
// Version 1: format adds a param count to the per cpu metadata.
// This allows easy adding of new metadata parameters.
// Requires that new params always added after current ones.
// Also allows client reader to handle file versions that are different by
// checking the number of params in the file vs the number expected.
//
// Version 2: Drivers will use PERF_RECORD_AUX_OUTPUT_HW_ID to output
// CoreSight Trace ID. ...TRACEIDR metadata will be set to legacy values
// but with addition flags.
//
pub const CS_HEADER_CURRENT_VERSION: c_int = 2;
// Beginning of header common to both ETMv3 and V4
// Number of trace config params in following ETM specific block
// ETMv3/PTM metadata
// Dynamic, configurable parameters
// RO, taken from sysFS
// define fixed version 0 length - allow new format reader to read old files.

// ETMv4 metadata
// Dynamic, configurable parameters
// RO, taken from sysFS
// define fixed version 0 length - allow new format reader to read old files.

//
// ETE metadata is ETMv4 plus TRCDEVARCH register and doesn't support header V0 since it was
// added in header V1
//
// Dynamic, configurable parameters
// RO, taken from sysFS
//
// Check for valid CoreSight trace ID. If an invalid value is present in the metadata,
// then IDs are present in the hardware ID packet in the data file.
//

//
// ETMv3 exception encoding number:
// See Embedded Trace Macrocell specification (ARM IHI 0014Q)
// table 7-12 Encoding of Exception[3:0] for non-ARMv7-M processors.
//
// ETMv4 exception encoding number:
// See ARM Embedded Trace Macrocell Architecture Specification (ARM IHI 0064D)
// table 6-12 Possible values for the TYPE field in an Exception instruction
// trace packet, for ARMv7-A/R and ARMv8-A/R PEs.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cs_etm_sample_type {
    CS_ETM_EMPTY,
    CS_ETM_RANGE,
    CS_ETM_DISCONTINUITY,
    CS_ETM_EXCEPTION,
    CS_ETM_EXCEPTION_RET,
    CS_ETM_CONTEXT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cs_etm_isa {
    CS_ETM_ISA_UNKNOWN,
    CS_ETM_ISA_A64,
    CS_ETM_ISA_A32,
    CS_ETM_ISA_T32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_etm_packet {
    pub sample_type: cs_etm_sample_type,
    pub isa: cs_etm_isa,
    pub start_addr: u64,
    pub end_addr: u64,
    pub instr_count: u32,
    pub last_instr_type: u32,
    pub last_instr_subtype: u32,
    pub flags: u32,
    pub exception_number: u32,
    pub last_instr_cond: bool,
    pub last_instr_taken_branch: bool,
    pub last_instr_size: u8,
    pub trace_chan_id: u8,
    pub cpu: c_int,
    pub el: c_int,
    pub tid: pid_t,
}

pub const CS_ETM_PACKET_MAX_BUFFER: c_int = 1024;
//
// When working with per-thread scenarios the process under trace can
// be scheduled on any CPU and as such, more than one traceID may be
// associated with the same process.  Since a traceID of '0' is illegal
// as per the CoreSight architecture, use that specific value to
// identify the queue where all packets (with any traceID) are
// aggregated.
//
pub const CS_ETM_PER_THREAD_TRACEID: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_etm_packet_queue {
    pub packet_count: u32,
    pub head: u32,
    pub tail: u32,
    pub instr_count: u32,
    pub /: *mut *mut u64 cs_timestamp; / Timestamp from trace data, converted to ns if possible,
    pub next_cs_timestamp: u64,
    pub packet_buffer: [cs_etm_packet; CS_ETM_PACKET_MAX_BUFFER],
}

pub const CS_ETM_INVAL_ADDR: c_uint = 0xdeadbeefdeadbeefUL;

pub const __perf_cs_etmv3_magic: c_uint = 0x3030303030303030ULL;
pub const __perf_cs_etmv4_magic: c_uint = 0x4040404040404040ULL;
pub const __perf_cs_ete_magic: c_uint = 0x5050505050505050ULL;

// CoreSight trace ID is currently the bottom 7 bits of the value

// ETMv4 CONFIGR register bits

// ETMv3 ETMCR register bits

extern "C" {
    pub fn cs_etm_get_default_config(pmu: *const perf_pmu, attr: *mut perf_event_attr);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cs_etm_pid_fmt {
    CS_ETM_PIDFMT_NONE,
    CS_ETM_PIDFMT_CTXTID,
    CS_ETM_PIDFMT_CTXTID2
}

extern "C" {
    pub fn cs_etm__get_cpu(etmq: *mut cs_etm_queue, trace_chan_id: u8, cpu: *mut c_int) -> c_int;
}
extern "C" {
    pub fn cs_etm__get_pid_fmt(etmq: *mut cs_etm_queue) -> cs_etm_pid_fmt;
}
extern "C" {
    pub fn cs_etm__etmq_is_timeless(etmq: *mut cs_etm_queue) -> bool;
}
// cs_etm__etmq_get_packet_queue(struct cs_etm_queue *etmq, u8 trace_chan_id);
extern "C" {
    pub fn cs_etm__convert_sample_time(etmq: *mut cs_etm_queue, cs_timestamp: u64) -> u64;
}

