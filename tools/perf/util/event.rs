//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/event.h
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
// The linux/stddef.h isn't need here, but is needed for __always_inline used
// in files included from uapi/linux/perf_event.h such as
// /usr/include/linux/swab.h and /usr/include/linux/byteorder/little_endian.h,
// detected in at least musl libc, used in Alpine Linux. -acme
//

//
// /usr/include/inttypes.h uses just 'lu' for PRIu64, but we end up defining
// __u64 as long long unsigned int, and then -Werror=format= kicks in and
// complains of the mismatched types, so use these two special extra PRI
// macros to overcome that.
//

// perf sample has 16 bits size limit

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_callchain {
    pub nr: u64,
    pub ips: [u64; ],
}

// Attribute type for custom synthesized events

// Attribute config for custom synthesized events
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_synth_id {
    PERF_SYNTH_INTEL_PTWRITE,
    PERF_SYNTH_INTEL_MWAIT,
    PERF_SYNTH_INTEL_PWRE,
    PERF_SYNTH_INTEL_EXSTOP,
    PERF_SYNTH_INTEL_PWRX,
    PERF_SYNTH_INTEL_CBR,
    PERF_SYNTH_INTEL_PSB,
    PERF_SYNTH_INTEL_EVT,
    PERF_SYNTH_INTEL_IFLAG_CHG,
    PERF_SYNTH_POWERPC_VPA_DTL,
}

//
// Raw data formats for synthesized events. Note that 4 bytes of padding are
// present to match the 'size' member of PERF_SAMPLE_RAW data which is always
// 8-byte aligned. That means we must dereference raw_data with an offset of 4.
// Refer perf_sample__synth_ptr() and perf_synth__raw_data().  It also means the
// structure sizes are 4 bytes bigger than the raw_size, refer
// perf_synth__raw_size().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_synth_intel_ptwrite {
    pub padding: u32,
    pub 31: reserved :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_synth_intel_mwait {
    pub padding: u32,
    pub reserved: u32,
    pub 30: reserved2 :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_synth_intel_pwre {
    pub padding: u32,
    pub reserved: u32,
    pub 48: reserved2 :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_synth_intel_exstop {
    pub padding: u32,
    pub 31: reserved :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_synth_intel_pwrx {
    pub padding: u32,
    pub reserved: u32,
    pub 52: reserved1 :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_synth_intel_cbr {
    pub padding: u32,
    pub 8: reserved2 :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_synth_intel_psb {
    pub padding: u32,
    pub reserved: u32,
    pub offset: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_synth_intel_evd {
    pub evd_type: u8,
    pub reserved: [u8; 7],
}

// Intel PT Event Trace
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_synth_intel_evt {
    pub padding: u32,
    pub 16: evd_cnt :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_synth_intel_iflag_chg {
    pub padding: u32,
    pub 1: via_branch :,
}

//
// The powerpc VPA DTL entries are of below format
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct powerpc_vpadtl_entry {
    pub dispatch_reason: u8,
    pub preempt_reason: u8,
    pub processor_id: u16,
    pub enqueue_to_dispatch_time: u32,
    pub ready_to_enqueue_time: u32,
    pub waiting_to_ready_time: u32,
    pub timebase: u64,
    pub fault_addr: u64,
    pub srr0: u64,
    pub srr1: u64,
}

extern "C" {
    pub fn perf_event__print_totals();
}
extern "C" {
    pub fn is_bts_event(attr: *mut perf_event_attr) -> bool;
}
extern "C" {
    pub fn sample_addr_correlates_sym(attr: *mut perf_event_attr) -> bool;
}
extern "C" {
    pub fn perf_event__fprintf_comm(event: *mut perf_event, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn perf_event__fprintf_mmap(event: *mut perf_event, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn perf_event__fprintf_mmap2(event: *mut perf_event, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn perf_event__fprintf_task(event: *mut perf_event, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn perf_event__fprintf_aux(event: *mut perf_event, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn perf_event__fprintf_itrace_start(event: *mut perf_event, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn perf_event__fprintf_aux_output_hw_id(event: *mut perf_event, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn perf_event__fprintf_switch(event: *mut perf_event, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn perf_event__fprintf_thread_map(event: *mut perf_event, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn perf_event__fprintf_cpu_map(event: *mut perf_event, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn perf_event__fprintf_namespaces(event: *mut perf_event, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn perf_event__fprintf_cgroup(event: *mut perf_event, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn perf_event__fprintf_ksymbol(event: *mut perf_event, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn perf_event__fprintf_bpf(event: *mut perf_event, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn perf_event__fprintf_bpf_metadata(event: *mut perf_event, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn perf_event__fprintf_text_poke(event: *mut perf_event, machine: *mut machine, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn perf_event__fprintf_schedstat_cpu(event: *mut perf_event, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn perf_event__fprintf_schedstat_domain(event: *mut perf_event, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn perf_event__fprintf(event: *mut perf_event, machine: *mut machine, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn event_attr_init(attr: *mut perf_event_attr);
}
extern "C" {
    pub fn perf_event_paranoid() -> c_int;
}
extern "C" {
    pub fn perf_event_paranoid_check(max_level: c_int) -> bool;
}
pub const PAGE_SIZE_NAME_LEN: c_int = 32;
extern "C" {
    pub fn perf_event_header__cpumode_is_guest(PERF_RECORD_MISC_CPUMODE_MASK: misc &) -> return;
}
extern "C" {
    pub fn perf_event_header__misc_is_guest(_arg: header->misc) -> return;
}
extern "C" {
    pub fn perf_event_header__is_guest(_arg: &event->header) -> return;
}
