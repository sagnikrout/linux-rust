//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/session.h
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
#[derive(Copy, Clone)]
pub struct decomp_data {
    pub decomp: *mut decomp,
    pub decomp_last: *mut decomp,
    pub zstd_decomp: *mut zstd_data,
}

//
// struct perf_session- A Perf session holds the main state when the program is
// working with live perf events or reading data from an input file.
//
// The rough organization of a perf_session is:
// ```
// +--------------+           +-----------+           +------------+
// |   Session    |1..* ----->|  Machine  |1..* ----->|   Thread   |
// +--------------+           +-----------+           +------------+
// ```
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_session {
//
// @header: The read version of a perf_file_header, or captures global
// information from a live session.
//
    pub header: perf_header,
// @machines: Machines within the session a host and 0 or more guests.
    pub machines: machines,
// @evlist: List of evsels/events of the session.
    pub evlist: *mut evlist,
// @auxtrace: callbacks to allow AUX area data decoding.
    pub auxtrace: *const auxtrace,
// @itrace_synth_opts: AUX area tracing synthesis options.
    pub itrace_synth_opts: *mut itrace_synth_opts,
// @auxtrace_index: index of AUX area tracing events within a perf.data file.
    pub auxtrace_index: list_head,

// @tevent: handles for libtraceevent and plugins.
    pub tevent: trace_event,

// @time_conv: Holds contents of last PERF_RECORD_TIME_CONV event.
    pub time_conv: perf_record_time_conv,
// @trace_event_repipe: When set causes read trace events to be written to stdout.
    pub trace_event_repipe: bool,
//
// @one_mmap: The reader will use a single mmap by default. There may be
// multiple data files in particular for aux events. If this is true
// then the single big mmap for the data file can be assumed.
//
    pub one_mmap: bool,
// @one_mmap_addr: Address of initial perf data file reader mmap.
    pub one_mmap_addr: *mut c_void,
// @one_mmap_offset: File offset in perf.data file when mapped.
    pub one_mmap_offset: u64,
// @one_mmap_size: Size of the single mmap in bytes.
    pub one_mmap_size: u64,
// @ordered_events: Used to turn unordered events into ordered ones.
    pub ordered_events: ordered_events,
// @data: Optional perf data file being read from.
    pub data: *mut perf_data,
// @tool: callbacks for event handling.
    pub tool: *const perf_tool,
//
// @bytes_transferred: Used by perf record to count written bytes before
// compression.
//
    pub bytes_transferred: u64,
//
// @bytes_compressed: Used by perf record to count written bytes after
// compression.
//
    pub bytes_compressed: u64,
// @zstd_data: Owner of global compression state, buffers, etc.
    pub zstd_data: zstd_data,
    pub decomp_data: decomp_data,
    pub active_decomp: *mut decomp_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct decomp {
    pub next: *mut decomp,
    pub file_pos: u64,
    pub file_path: *const c_char,
    pub mmap_len: usize,
    pub head: u64,
    pub size: usize,
    pub data: [c_char; ],
}

extern "C" {
    pub fn __perf_session__new(_arg: data, _arg: tool, _arg: *mut *mut /trace_event_repipe=/false, _arg: *mut *mut /host_env=/NULL) -> return;
}
extern "C" {
    pub fn perf_session__delete(session: *mut perf_session);
}
extern "C" {
    pub fn perf_event_header__bswap(hdr: *mut perf_event_header);
}
extern "C" {
    pub fn perf_event__too_small(event: *const perf_event, min: *mut u32) -> bool;
}
extern "C" {
    pub fn perf_session__process_events(session: *mut perf_session) -> c_int;
}
extern "C" {
    pub fn perf_session__has_traces(session: *mut perf_session, msg: *const c_char) -> bool;
}
extern "C" {
    pub fn perf_session__has_switch_events(session: *mut perf_session) -> bool;
}
extern "C" {
    pub fn perf_event__attr_swap(attr: *mut perf_event_attr);
}
extern "C" {
    pub fn perf_session__create_kernel_maps(session: *mut perf_session) -> c_int;
}
extern "C" {
    pub fn perf_session__set_id_hdr_size(session: *mut perf_session);
}
extern "C" {
    pub fn machines__find(_arg: &session->machines, _arg: pid) -> return;
}
extern "C" {
    pub fn machines__findnew(_arg: &session->machines, _arg: pid) -> return;
}
extern "C" {
    pub fn perf_session__register_idle_thread(session: *mut perf_session) -> c_int;
}
extern "C" {
    pub fn perf_session__fprintf(session: *mut perf_session, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn perf_session__fprintf_dsos(session: *mut perf_session, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn perf_session__fprintf_nr_events(session: *mut perf_session, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn perf_session__dump_kmaps(session: *mut perf_session);
}
extern "C" {
    pub fn perf_session__fprintf_info(s: *mut perf_session, fp: *mut FILE, full: bool);
}

extern "C" {
    pub fn perf_session__dsos_hit_all(session: *mut perf_session) -> c_int;
}
extern "C" {
    pub fn perf_session__e_machine(session: *mut perf_session, e_flags: *mut u32) -> u16;
}
