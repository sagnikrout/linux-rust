//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/kwork.h
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


#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kwork_class_type {
    KWORK_CLASS_IRQ,
    KWORK_CLASS_SOFTIRQ,
    KWORK_CLASS_WORKQUEUE,
    KWORK_CLASS_SCHED,
    KWORK_CLASS_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kwork_report_type {
    KWORK_REPORT_RUNTIME,
    KWORK_REPORT_LATENCY,
    KWORK_REPORT_TIMEHIST,
    KWORK_REPORT_TOP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kwork_trace_type {
    KWORK_TRACE_RAISE,
    KWORK_TRACE_ENTRY,
    KWORK_TRACE_EXIT,
    KWORK_TRACE_MAX,
}

//
// data structure:
//
// +==================+ +============+ +======================+
// |      class       | |    work    | |         atom         |
// +==================+ +============+ +======================+
// +------------+  |  +-----+         | |  +------+  | |  +-------+   +-----+ |
// | perf_kwork | +-> | irq | --------|+-> | eth0 | --+-> | raise | - | ... | --+   +-----------+
// +-----+------+ ||  +-----+         |||  +------+  |||  +-------+   +-----+ | |   |           |
// |        ||                  |||            |||                      | +-> | atom_page |
// |        ||                  |||            |||  +-------+   +-----+ |     |           |
// |  class_list                |||            |+-> | entry | - | ... | ----> |           |
// |        ||                  |||            |||  +-------+   +-----+ |     |           |
// |        ||                  |||            |||                      | +-> |           |
// |        ||                  |||            |||  +-------+   +-----+ | |   |           |
// |        ||                  |||            |+-> | exit  | - | ... | --+   +-----+-----+
// |        ||                  |||            | |  +-------+   +-----+ |           |
// |        ||                  |||            | |                      |           |
// |        ||                  |||  +-----+   | |                      |           |
// |        ||                  |+-> | ... |   | |                      |           |
// |        ||                  | |  +-----+   | |                      |           |
// |        ||                  | |            | |                      |           |
// |        ||  +---------+     | |  +-----+   | |  +-------+   +-----+ |           |
// |        +-> | softirq | -------> | RCU | ---+-> | raise | - | ... | --+   +-----+-----+
// |        ||  +---------+     | |  +-----+   |||  +-------+   +-----+ | |   |           |
// |        ||                  | |            |||                      | +-> | atom_page |
// |        ||                  | |            |||  +-------+   +-----+ |     |           |
// |        ||                  | |            |+-> | entry | - | ... | ----> |           |
// |        ||                  | |            |||  +-------+   +-----+ |     |           |
// |        ||                  | |            |||                      | +-> |           |
// |        ||                  | |            |||  +-------+   +-----+ | |   |           |
// |        ||                  | |            |+-> | exit  | - | ... | --+   +-----+-----+
// |        ||                  | |            | |  +-------+   +-----+ |           |
// |        ||                  | |            | |                      |           |
// |        ||  +-----------+   | |  +-----+   | |                      |           |
// |        +-> | workqueue | -----> | ... |   | |                      |           |
// |         |  +-----------+   | |  +-----+   | |                      |           |
// |         +==================+ +============+ +======================+           |
// |                                                                                |
// +---->  atom_page_list  ---------------------------------------------------------+
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kwork_atom {
    pub list: list_head,
    pub time: u64,
    pub prev: *mut kwork_atom,
    pub page_addr: *mut c_void,
    pub bit_inpage: c_ulong,
}

pub const NR_ATOM_PER_PAGE: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kwork_atom_page {
    pub list: list_head,
    pub atoms: [kwork_atom; NR_ATOM_PER_PAGE],
    pub NR_ATOM_PER_PAGE): DECLARE_BITMAP(bitmap,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kwork_work {
//
// class field
//
    pub node: rb_node,
    pub class: *mut kwork_class,
//
// work field
//
    pub id: u64,
    pub cpu: c_int,
    pub name: *mut c_char,
//
// atom field
//
    pub nr_atoms: u64,
    pub atom_list: [list_head; KWORK_TRACE_MAX],
//
// runtime report
//
    pub max_runtime: u64,
    pub max_runtime_start: u64,
    pub max_runtime_end: u64,
    pub total_runtime: u64,
//
// latency report
//
    pub max_latency: u64,
    pub max_latency_start: u64,
    pub max_latency_end: u64,
    pub total_latency: u64,
//
// top report
//
    pub cpu_usage: u32,
    pub tgid: u32,
    pub is_kthread: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kwork_class {
    pub list: list_head,
    pub name: *const c_char,
    pub type: kwork_class_type,
    pub nr_tracepoints: c_uint,
    pub tp_handlers: *const evsel_str_handler,
    pub work_root: rb_root_cached,
    pub session): *mut perf_session,
    pub machine): *mut machine,
    pub len): *mut *mut char buf, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_kwork_handler {
    pub machine): *mut *mut perf_sample sample, machine,
    pub machine): *mut *mut perf_sample sample, machine,
    pub machine): *mut *mut perf_sample sample, machine,
    pub machine): *mut *mut perf_sample sample, machine,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __top_cpus_runtime {
    pub load: u64,
    pub idle: u64,
    pub irq: u64,
    pub softirq: u64,
    pub total: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kwork_top_stat {
    pub MAX_NR_CPUS): DECLARE_BITMAP(all_cpus_bitmap,,
    pub cpus_runtime: *mut __top_cpus_runtime,
    pub nr_skipped_cpu: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_kwork {
//
// metadata
//
    pub tool: perf_tool,
    pub class_list: list_head,
    pub atom_page_list: list_head,
    pub cmp_id: list_head sort_list,,
    pub sorted_work_root: rb_root_cached,
    pub tp_handler: *const trace_kwork_handler,
//
// profile filters
//
    pub profile_name: *const c_char,
    pub cpu_list: *const c_char,
    pub MAX_NR_CPUS): DECLARE_BITMAP(cpu_bitmap,,
    pub time_str: *const c_char,
    pub ptime: perf_time_interval,
//
// options for command
//
    pub force: bool,
    pub event_list_str: *const c_char,
    pub report: kwork_report_type,
//
// options for subcommand
//
    pub summary: bool,
    pub sort_order: *const c_char,
    pub show_callchain: bool,
    pub max_stack: c_uint,
    pub use_bpf: bool,
//
// statistics
//
    pub timestart: u64,
    pub timeend: u64,
    pub nr_events: c_ulong,
    pub nr_lost_chunks: c_ulong,
    pub nr_lost_events: c_ulong,
    pub all_runtime: u64,
    pub all_count: u64,
    pub 1]: u64 nr_skipped_events[KWORK_TRACE_MAX +,
//
// perf kwork top data
//
    pub top_stat: kwork_top_stat,
// Add work callback.
    pub key): *mut kwork_work,
}

extern "C" {
    pub fn perf_kwork__trace_prepare_bpf(kwork: *mut perf_kwork) -> c_int;
}
extern "C" {
    pub fn perf_kwork__report_read_bpf(kwork: *mut perf_kwork) -> c_int;
}
extern "C" {
    pub fn perf_kwork__report_cleanup_bpf();
}
extern "C" {
    pub fn perf_kwork__trace_start();
}
extern "C" {
    pub fn perf_kwork__trace_finish();
}
extern "C" {
    pub fn perf_kwork__top_prepare_bpf(kwork: *mut perf_kwork) -> c_int;
}
extern "C" {
    pub fn perf_kwork__top_read_bpf(kwork: *mut perf_kwork) -> c_int;
}
extern "C" {
    pub fn perf_kwork__top_cleanup_bpf();
}
extern "C" {
    pub fn perf_kwork__top_start();
}
extern "C" {
    pub fn perf_kwork__top_finish();
}

