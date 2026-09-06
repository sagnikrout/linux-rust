//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/events/intel/pt.h
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
// Intel(R) Processor Trace PMU driver for perf
// Copyright (c) 2013-2014, Intel Corporation.
//
// Intel PT is specified in the Intel Architecture Instruction Set Extensions
// Programming Reference:
// http://software.intel.com/en-us/intel-isa-extensions
//
// Single-entry ToPA: when this close to region boundary, switch
// buffers to avoid losing data.
//
pub const TOPA_PMI_MARGIN: c_int = 512;
pub const TOPA_SHIFT: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct topa_entry {
    pub 1: u64 end :,
    pub 1: u64 rsvd0 :,
    pub 1: u64 intr :,
    pub 1: u64 rsvd1 :,
    pub 1: u64 stop :,
    pub 1: u64 rsvd2 :,
    pub 4: u64 size :,
    pub 2: u64 rsvd3 :,
    pub 40: u64 base :,
    pub 12: u64 rsvd4 :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_pmu {
    pub pmu: pmu,
    pub PT_CPUID_LEAVES]: *mut *mut u32 caps[PT_CPUID_REGS_NUM,
    pub vmx: bool,
    pub branch_en_always_on: bool,
    pub max_nonturbo_ratio: c_ulong,
    pub tsc_art_num: c_uint,
    pub tsc_art_den: c_uint,
}

//
// struct pt_buffer - buffer configuration; one buffer per task_struct or
// cpu, depending on perf event configuration
// @tables:	list of ToPA tables in this buffer
// @first:	shorthand for first topa table
// @last:	shorthand for last topa table
// @cur:	current topa table
// @nr_pages:	buffer size in pages
// @cur_idx:	current output region's index within @cur table
// @output_off:	offset within the current output region
// @data_size:	running total of the amount of data in this buffer
// @head:	logical write offset inside the buffer
// @snapshot:	if this is for a snapshot/overwrite counter
// @single:	use Single Range Output instead of ToPA
// @wrapped:	buffer advance wrapped back to the first topa table
// @stop_pos:	STOP topa entry index
// @intr_pos:	INT topa entry index
// @stop_te:	STOP topa entry pointer
// @intr_te:	INT topa entry pointer
// @data_pages:	array of pages from perf
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_buffer {
    pub tables: list_head,
    pub cur: *mut *mut *mut topa first, last,,
    pub cur_idx: c_uint,
    pub output_off: usize,
    pub nr_pages: c_ulong,
    pub data_size: local_t,
    pub head: local64_t,
    pub snapshot: bool,
    pub single: bool,
    pub wrapped: bool,
    pub intr_pos: long stop_pos,,
    pub intr_te: *mut *mut topa_entry stop_te,,
    pub data_pages: *mut c_void,
}

pub const PT_FILTERS_NUM: c_int = 4;
//
// struct pt_filter - IP range filter configuration
// @msr_a:	range start, goes to RTIT_ADDRn_A
// @msr_b:	range end, goes to RTIT_ADDRn_B
// @config:	4-bit field in RTIT_CTL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_filter {
    pub msr_a: c_ulong,
    pub msr_b: c_ulong,
    pub config: c_ulong,
}

//
// struct pt_filters - IP range filtering context
// @filter:	filters defined for this context
// @nr_filters:	number of defined filters in the @filter array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_filters {
    pub filter: [pt_filter; PT_FILTERS_NUM],
    pub nr_filters: c_uint,
}

//
// struct pt - per-cpu pt context
// @handle:		perf output handle
// @filters:		last configured filters
// @handle_nmi:		do handle PT PMI on this cpu, there's an active event
// @vmx_on:		1 if VMX is ON on this cpu
// @pause_allowed:	PERF_EF_PAUSE is allowed to stop tracing
// @resume_allowed:	PERF_EF_RESUME is allowed to start tracing
// @output_base:	cached RTIT_OUTPUT_BASE MSR value
// @output_mask:	cached RTIT_OUTPUT_MASK MSR value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt {
    pub handle: perf_output_handle,
    pub filters: pt_filters,
    pub handle_nmi: c_int,
    pub vmx_on: c_int,
    pub pause_allowed: c_int,
    pub resume_allowed: c_int,
    pub output_base: u64,
    pub output_mask: u64,
}
