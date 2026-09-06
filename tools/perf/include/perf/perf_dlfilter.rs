//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/include/perf/perf_dlfilter.h
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
// perf_dlfilter.h: API for perf --dlfilter shared object
// Copyright (c) 2021, Intel Corporation.
//

//
// The following macro can be used to determine if this header defines
// perf_dlfilter_sample machine_pid and vcpu.
//
// Macro flag: #define PERF_DLFILTER_HAS_MACHINE_PID
// Definitions for perf_dlfilter_sample flags
//
// perf sample event information (as per perf script and <linux/perf_event.h>)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_dlfilter_sample {
    pub /: *mut *mut __u32 size; / Size of this structure (for compatibility checking),
    pub /: *mut *mut __u16 ins_lat; / Refer PERF_SAMPLE_WEIGHT_TYPE in <linux/perf_event.h>,
    pub /: *mut *mut __u16 p_stage_cyc; / Refer PERF_SAMPLE_WEIGHT_TYPE in <linux/perf_event.h>,
    pub ip: __u64,
    pub pid: __s32,
    pub tid: __s32,
    pub time: __u64,
    pub addr: __u64,
    pub id: __u64,
    pub stream_id: __u64,
    pub period: __u64,
    pub /: *mut *mut __u64 weight; / Refer PERF_SAMPLE_WEIGHT_TYPE in <linux/perf_event.h>,
    pub /: *mut *mut __u64 transaction; / Refer PERF_SAMPLE_TRANSACTION in <linux/perf_event.h>,
    pub /: *mut *mut __u64 insn_cnt; / For instructions-per-cycle (IPC),
    pub /: *mut *mut __u64 cyc_cnt; / For instructions-per-cycle (IPC),
    pub cpu: __s32,
    pub /: *mut *mut *mut __u32 flags; / Refer PERF_DLFILTER_FLAG_ above,
    pub /: *mut *mut __u64 data_src; / Refer PERF_SAMPLE_DATA_SRC in <linux/perf_event.h>,
    pub /: *mut *mut __u64 phys_addr; / Refer PERF_SAMPLE_PHYS_ADDR in <linux/perf_event.h>,
    pub /: *mut *mut __u64 data_page_size; / Refer PERF_SAMPLE_DATA_PAGE_SIZE in <linux/perf_event.h>,
    pub /: *mut *mut __u64 code_page_size; / Refer PERF_SAMPLE_CODE_PAGE_SIZE in <linux/perf_event.h>,
    pub /: *mut *mut __u64 cgroup; / Refer PERF_SAMPLE_CGROUP in <linux/perf_event.h>,
    pub /: *mut *mut __u8 cpumode; / Refer CPUMODE_MASK etc in <linux/perf_event.h>,
    pub /: *mut *mut __u8 addr_correlates_sym; / True => resolve_addr() can be called,
    pub /: *mut *mut __u16 misc; / Refer perf_event_header in <linux/perf_event.h>,
    pub /: *mut *mut __u32 raw_size; / Refer PERF_SAMPLE_RAW in <linux/perf_event.h>,
    pub /: *const *const *const void raw_data; / Refer PERF_SAMPLE_RAW in <linux/perf_event.h>,
    pub /: *mut *mut __u64 brstack_nr; / Number of brstack entries,
    pub /: *const *const *const perf_branch_entry brstack; / Refer <linux/perf_event.h>,
    pub /: *mut *mut __u64 raw_callchain_nr; / Number of raw_callchain entries,
    pub /: *const *const *const __u64 raw_callchain; / Refer <linux/perf_event.h>,
    pub event: *const c_char,
    pub machine_pid: __s32,
    pub vcpu: __s32,
}

//
// Address location (as per perf script)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_dlfilter_al {
    pub /: *mut *mut __u32 size; / Size of this structure (for compatibility checking),
    pub symoff: __u32,
    pub sym: *const c_char,
    pub /: *mut *mut __u64 addr; / Mapped address (from dso),
    pub sym_start: __u64,
    pub sym_end: __u64,
    pub dso: *const c_char,
    pub /: *mut *mut __u8 sym_binding; / STB_LOCAL, STB_GLOBAL or STB_WEAK, refer <elf.h>,
    pub /: *mut *mut __u8 is_64_bit; / Only valid if dso is not NULL,
    pub /: *mut *mut __u8 is_kernel_ip; / True if in kernel space,
    pub buildid_size: __u32,
    pub buildid: *const __u8,
// Below members are only populated by resolve_ip()
    pub /: *mut *mut __u8 filtered; / True if this sample event will be filtered out,
    pub comm: *const c_char,
    pub /: *mut *mut *mut void priv; / Private data. Do not change,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_dlfilter_fns {
// Return information about ip
    pub ctx): *const *const *const perf_dlfilter_al (resolve_ip)(void,
// Return information about addr (if addr_correlates_sym)
    pub ctx): *const *const *const perf_dlfilter_al (resolve_addr)(void,
// Return arguments from --dlarg option
    pub dlargc): *mut *mut *mut *mut *mut char (args)(void ctx, int,
//
// Return information about address (al->size must be set before
// calling). Returns 0 on success, -1 otherwise. Call al_cleanup()
// when 'al' data is no longer needed.
//
    pub al): *mut *mut *mut __s32 (resolve_address)(void ctx, __u64 address, struct perf_dlfilter_al,
// Return instruction bytes and length
    pub length): *const *const *const *const __u8 (insn)(void ctx, __u32,
// Return source file name and line number
    pub line_number): *const *const *const *const char (srcline)(void ctx, __u32,
// Return perf_event_attr, refer <linux/perf_event.h>
    pub ctx): *mut *mut *mut perf_event_attr (attr)(void,
// Read object code, return numbers of bytes read
    pub len): *mut *mut *mut *mut __s32 (object_code)(void ctx, __u64 ip, void buf, __u32,
//
// If present (i.e. must check al_cleanup != NULL), call after
// resolve_address() to free any associated resources.
//
    pub al): *mut *mut *mut void (al_cleanup)(void ctx, struct perf_dlfilter_al,
// Reserved
    pub ): *mut *mut *mut void (reserved[119])(void,
}

//
// If implemented, 'start' will be called at the beginning,
// before any calls to 'filter_event'. Return 0 to indicate success,
// or return a negative error code. '*data' can be assigned for use
// by other functions. 'ctx' is needed for calls to perf_dlfilter_fns,
// but most perf_dlfilter_fns are not valid when called from 'start'.
//
extern "C" {
    pub fn start(data: *mut c_void, ctx: *mut c_void) -> c_int;
}
//
// If implemented, 'stop' will be called at the end,
// after any calls to 'filter_event'. Return 0 to indicate success, or
// return a negative error code. 'data' is set by start(). 'ctx' is
// needed for calls to perf_dlfilter_fns, but most perf_dlfilter_fns
// are not valid when called from 'stop'.
//
extern "C" {
    pub fn stop(data: *mut c_void, ctx: *mut c_void) -> c_int;
}
//
// If implemented, 'filter_event' will be called for each sample
// event. Return 0 to keep the sample event, 1 to filter it out, or
// return a negative error code. 'data' is set by start(). 'ctx' is
// needed for calls to perf_dlfilter_fns.
//
extern "C" {
    pub fn filter_event(data: *mut c_void, sample: *const perf_dlfilter_sample, ctx: *mut c_void) -> c_int;
}
//
// The same as 'filter_event' except it is called before internal
// filtering.
//
extern "C" {
    pub fn filter_event_early(data: *mut c_void, sample: *const perf_dlfilter_sample, ctx: *mut c_void) -> c_int;
}
//
// If implemented, return a one-line description of the filter, and optionally
// a longer description.
//
