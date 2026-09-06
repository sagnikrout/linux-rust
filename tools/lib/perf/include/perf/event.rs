//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/perf/include/perf/event.h
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
// Verify the full field fits within the event, not just its start offset.
// Only valid for fixed-size scalar fields — for trailing arrays like
// filename[PATH_MAX], sizeof() evaluates to the declared maximum, not
// the actual string length, so this would spuriously return false.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_mmap {
    pub header: perf_event_header,
    pub tid: __u32 pid,,
    pub start: __u64,
    pub len: __u64,
    pub pgoff: __u64,
    pub filename: [c_char; PATH_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_mmap2 {
    pub header: perf_event_header,
    pub tid: __u32 pid,,
    pub start: __u64,
    pub len: __u64,
    pub pgoff: __u64,
    pub maj: __u32,
    pub min: __u32,
    pub ino: __u64,
    pub ino_generation: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_comm {
    pub header: perf_event_header,
    pub tid: __u32 pid,,
    pub comm: [c_char; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_namespaces {
    pub header: perf_event_header,
    pub tid: __u32 pid,,
    pub nr_namespaces: __u64,
    pub link_info: [perf_ns_link_info; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_fork {
    pub header: perf_event_header,
    pub ppid: __u32 pid,,
    pub ptid: __u32 tid,,
    pub time: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_lost {
    pub header: perf_event_header,
    pub id: __u64,
    pub lost: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_lost_samples {
    pub header: perf_event_header,
    pub lost: __u64,
}

pub const MAX_ID_HDR_ENTRIES: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_lost_samples_and_ids {
    pub lost: perf_record_lost_samples,
    pub sample_ids: [__u64; MAX_ID_HDR_ENTRIES],
}

//
// PERF_FORMAT_ENABLED | PERF_FORMAT_RUNNING | PERF_FORMAT_ID | PERF_FORMAT_LOST
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_read {
    pub header: perf_event_header,
    pub tid: __u32 pid,,
    pub value: __u64,
    pub time_enabled: __u64,
    pub time_running: __u64,
    pub id: __u64,
    pub lost: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_throttle {
    pub header: perf_event_header,
    pub time: __u64,
    pub id: __u64,
    pub stream_id: __u64,
}

pub const KSYM_NAME_LEN: c_int = 512;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_ksymbol {
    pub header: perf_event_header,
    pub addr: __u64,
    pub len: __u32,
    pub ksym_type: __u16,
    pub flags: __u16,
    pub name: [c_char; KSYM_NAME_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_bpf_event {
    pub header: perf_event_header,
    pub type: __u16,
    pub flags: __u16,
    pub id: __u32,
// for bpf_prog types
    pub tag: __u8 tag[BPF_TAG_SIZE]; // prog,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_cgroup {
    pub header: perf_event_header,
    pub id: __u64,
    pub path: [c_char; PATH_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_text_poke_event {
    pub header: perf_event_header,
    pub addr: __u64,
    pub old_len: __u16,
    pub new_len: __u16,
    pub bytes: [__u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_sample {
    pub header: perf_event_header,
    pub array: [__u64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_switch {
    pub header: perf_event_header,
    pub next_prev_pid: __u32,
    pub next_prev_tid: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_callchain_deferred {
    pub header: perf_event_header,
//
// This is to match kernel and (deferred) user stacks together.
// The kernel part will be in the sample callchain array after
// the PERF_CONTEXT_USER_DEFERRED entry.
//
    pub cookie: __u64,
    pub nr: __u64,
    pub ips: [__u64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_header_attr {
    pub header: perf_event_header,
    pub attr: perf_event_attr,
//
// Array of u64 id follows here but we cannot use a flexible array
// because size of attr in the data can be different then current
// version.  Please use perf_record_header_attr_id() below.
//
// __u64		 id[];  // do not use this
//
}

// Returns the pointer to id array based on the actual attr size.

//
// Array encoding of a perf_cpu_map where nr is the number of entries in cpu[]
// and each entry is a value for a CPU in the map.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_map_entries {
    pub nr: __u16,
    pub cpu: [__u16; ],
}

// Bitmap encoding of a perf_cpu_map where bitmap entries are 32-bit.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_mask_cpu_map32 {
// Number of mask values.
    pub nr: __u16,
// Constant 4.
    pub long_size: __u16,
// Bitmap data.
    pub mask: [__u32; ],
}

// Bitmap encoding of a perf_cpu_map where bitmap entries are 64-bit.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_mask_cpu_map64 {
// Number of mask values.
    pub nr: __u16,
// Constant 8.
    pub long_size: __u16,
// Legacy padding.
    pub __pad: [c_char; 4],
// Bitmap data.
    pub mask: [__u64; ],
}

//
// 'struct perf_record_cpu_map_data' is packed as unfortunately an earlier
// version had unaligned data and we wish to retain file format compatibility.
// -irogers
//

//
// An encoding of a CPU map for a range starting at start_cpu through to
// end_cpu. If any_cpu is 1, an any CPU (-1) value (aka dummy value) is present.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_range_cpu_map {
    pub any_cpu: __u8,
    pub __pad: __u8,
    pub start_cpu: __u16,
    pub end_cpu: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_cpu_map_data {
    pub type: __u16,
// Used when type == PERF_CPU_MAP__CPUS.
    pub cpus_data: cpu_map_entries,
// Used when type == PERF_CPU_MAP__MASK and long_size == 4.
    pub mask32_data: perf_record_mask_cpu_map32,
// Used when type == PERF_CPU_MAP__MASK and long_size == 8.
    pub mask64_data: perf_record_mask_cpu_map64,
// Used when type == PERF_CPU_MAP__RANGE_CPUS.
    pub range_cpu_data: perf_record_range_cpu_map,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_cpu_map {
    pub header: perf_event_header,
    pub data: perf_record_cpu_map_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_event_update_cpus {
    pub cpus: perf_record_cpu_map_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_event_update_scale {
    pub scale: double,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_event_update {
    pub header: perf_event_header,
    pub type: __u64,
    pub id: __u64,
// Used when type == PERF_EVENT_UPDATE__SCALE.
    pub scale: perf_record_event_update_scale,
// Used when type == PERF_EVENT_UPDATE__UNIT.
    pub unit: [c_char; 0],
// Used when type == PERF_EVENT_UPDATE__NAME.
    pub name: [c_char; 0],
// Used when type == PERF_EVENT_UPDATE__CPUS.
    pub cpus: perf_record_event_update_cpus,
}

pub const MAX_EVENT_NAME: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_trace_event_type {
    pub event_id: __u64,
    pub name: [c_char; MAX_EVENT_NAME],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_header_event_type {
    pub header: perf_event_header,
    pub event_type: perf_trace_event_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_header_tracing_data {
    pub header: perf_event_header,
    pub size: __u32,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_header_build_id {
    pub header: perf_event_header,
    pub pid: pid_t,
    pub build_id: [__u8; 24],
    pub data: [__u8; 20],
    pub size: __u8,
    pub reserved1__: __u8,
    pub reserved2__: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct id_index_entry {
    pub id: __u64,
    pub idx: __u64,
    pub cpu: __u64,
    pub tid: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct id_index_entry_2 {
    pub machine_pid: __u64,
    pub vcpu: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_id_index {
    pub header: perf_event_header,
    pub nr: __u64,
    pub entries: [id_index_entry; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_auxtrace_info {
    pub header: perf_event_header,
    pub type: __u32,
    pub /: *mut *mut __u32 reserved__; / For alignment,
    pub priv: [__u64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_auxtrace {
    pub header: perf_event_header,
    pub size: __u64,
    pub offset: __u64,
    pub reference: __u64,
    pub idx: __u32,
    pub tid: __u32,
    pub cpu: __u32,
    pub /: *mut *mut __u32 reserved__; / For alignment,
}

pub const MAX_AUXTRACE_ERROR_MSG: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_auxtrace_error {
    pub header: perf_event_header,
    pub type: __u32,
    pub code: __u32,
    pub cpu: __u32,
    pub pid: __u32,
    pub tid: __u32,
    pub fmt: __u32,
    pub ip: __u64,
    pub time: __u64,
    pub msg: [c_char; MAX_AUXTRACE_ERROR_MSG],
    pub machine_pid: __u32,
    pub vcpu: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_aux {
    pub header: perf_event_header,
    pub aux_offset: __u64,
    pub aux_size: __u64,
    pub flags: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_itrace_start {
    pub header: perf_event_header,
    pub pid: __u32,
    pub tid: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_aux_output_hw_id {
    pub header: perf_event_header,
    pub hw_id: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_thread_map_entry {
    pub pid: __u64,
    pub comm: [c_char; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_thread_map {
    pub header: perf_event_header,
    pub nr: __u64,
    pub entries: [perf_record_thread_map_entry; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_stat_config_entry {
    pub tag: __u64,
    pub val: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_stat_config {
    pub header: perf_event_header,
    pub nr: __u64,
    pub data: [perf_record_stat_config_entry; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_stat {
    pub header: perf_event_header,
    pub id: __u64,
    pub cpu: __u32,
    pub thread: __u32,
    pub val: __u64,
    pub ena: __u64,
    pub run: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_stat_round {
    pub header: perf_event_header,
    pub type: __u64,
    pub time: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_time_conv {
    pub header: perf_event_header,
    pub time_shift: __u64,
    pub time_mult: __u64,
    pub time_zero: __u64,
    pub time_cycles: __u64,
    pub time_mask: __u64,
    pub cap_user_time_zero: __u8,
    pub cap_user_time_short: __u8,
    pub /: *mut *mut __u8 reserved[6]; / For alignment,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_header_feature {
    pub header: perf_event_header,
    pub feat_id: __u64,
    pub data: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_compressed {
    pub header: perf_event_header,
    pub data: [c_char; ],
}

//
// `header.size` includes the padding we are going to add while writing the record.
// `data_size` only includes the size of `data[]` itself.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_compressed2 {
    pub header: perf_event_header,
    pub data_size: __u64,
    pub data: [c_char; ],
}

pub const BPF_METADATA_KEY_LEN: c_int = 64;
pub const BPF_METADATA_VALUE_LEN: c_int = 256;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_bpf_metadata_entry {
    pub key: [c_char; BPF_METADATA_KEY_LEN],
    pub value: [c_char; BPF_METADATA_VALUE_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_bpf_metadata {
    pub header: perf_event_header,
    pub prog_name: [c_char; BPF_PROG_NAME_LEN],
    pub nr_entries: __u64,
    pub entries: [perf_record_bpf_metadata_entry; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_schedstat_cpu_v15 {

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_schedstat_cpu_v16 {

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_schedstat_cpu_v17 {

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_schedstat_cpu {
    pub header: perf_event_header,
    pub timestamp: __u64,
    pub cpu: __u32,
    pub version: __u16,
// Padding
    pub __pad: [c_char; 2],
    pub v15: perf_record_schedstat_cpu_v15,
    pub v16: perf_record_schedstat_cpu_v16,
    pub v17: perf_record_schedstat_cpu_v17,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_schedstat_domain_v15 {

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_schedstat_domain_v16 {

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_schedstat_domain_v17 {

}

pub const DOMAIN_NAME_LEN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_schedstat_domain {
    pub header: perf_event_header,
    pub timestamp: __u64,
    pub cpu: __u32,
    pub version: __u16,
    pub domain: __u16,
    pub v15: perf_record_schedstat_domain_v15,
    pub v16: perf_record_schedstat_domain_v16,
    pub v17: perf_record_schedstat_domain_v17,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_user_event_type {
    PERF_RECORD_USER_TYPE_START		= 64,
    PERF_RECORD_HEADER_ATTR			= 64,
    PERF_RECORD_HEADER_EVENT_TYPE		= 65, /* deprecated */
    PERF_RECORD_HEADER_TRACING_DATA		= 66,
    PERF_RECORD_HEADER_BUILD_ID		= 67,
    PERF_RECORD_FINISHED_ROUND		= 68,
    PERF_RECORD_ID_INDEX			= 69,
    PERF_RECORD_AUXTRACE_INFO		= 70,
    PERF_RECORD_AUXTRACE			= 71,
    PERF_RECORD_AUXTRACE_ERROR		= 72,
    PERF_RECORD_THREAD_MAP			= 73,
    PERF_RECORD_CPU_MAP			= 74,
    PERF_RECORD_STAT_CONFIG			= 75,
    PERF_RECORD_STAT			= 76,
    PERF_RECORD_STAT_ROUND			= 77,
    PERF_RECORD_EVENT_UPDATE		= 78,
    PERF_RECORD_TIME_CONV			= 79,
    PERF_RECORD_HEADER_FEATURE		= 80,
    PERF_RECORD_COMPRESSED			= 81,
    PERF_RECORD_FINISHED_INIT		= 82,
    PERF_RECORD_COMPRESSED2			= 83,
    PERF_RECORD_BPF_METADATA		= 84,
    PERF_RECORD_SCHEDSTAT_CPU		= 85,
    PERF_RECORD_SCHEDSTAT_DOMAIN		= 86,
    PERF_RECORD_HEADER_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union perf_event {
    pub header: perf_event_header,
    pub mmap: perf_record_mmap,
    pub mmap2: perf_record_mmap2,
    pub comm: perf_record_comm,
    pub namespaces: perf_record_namespaces,
    pub cgroup: perf_record_cgroup,
    pub fork: perf_record_fork,
    pub lost: perf_record_lost,
    pub lost_samples: perf_record_lost_samples,
    pub read: perf_record_read,
    pub throttle: perf_record_throttle,
    pub sample: perf_record_sample,
    pub callchain_deferred: perf_record_callchain_deferred,
    pub bpf: perf_record_bpf_event,
    pub ksymbol: perf_record_ksymbol,
    pub text_poke: perf_record_text_poke_event,
    pub attr: perf_record_header_attr,
    pub event_update: perf_record_event_update,
    pub event_type: perf_record_header_event_type,
    pub tracing_data: perf_record_header_tracing_data,
    pub build_id: perf_record_header_build_id,
    pub id_index: perf_record_id_index,
    pub auxtrace_info: perf_record_auxtrace_info,
    pub auxtrace: perf_record_auxtrace,
    pub auxtrace_error: perf_record_auxtrace_error,
    pub aux: perf_record_aux,
    pub itrace_start: perf_record_itrace_start,
    pub aux_output_hw_id: perf_record_aux_output_hw_id,
    pub context_switch: perf_record_switch,
    pub thread_map: perf_record_thread_map,
    pub cpu_map: perf_record_cpu_map,
    pub stat_config: perf_record_stat_config,
    pub stat: perf_record_stat,
    pub stat_round: perf_record_stat_round,
    pub time_conv: perf_record_time_conv,
    pub feat: perf_record_header_feature,
    pub pack: perf_record_compressed,
    pub pack2: perf_record_compressed2,
    pub bpf_metadata: perf_record_bpf_metadata,
    pub schedstat_cpu: perf_record_schedstat_cpu,
    pub schedstat_domain: perf_record_schedstat_domain,
}
