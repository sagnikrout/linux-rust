//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/hist.h
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
pub enum hist_filter {
    HIST_FILTER__DSO,
    HIST_FILTER__THREAD,
    HIST_FILTER__PARENT,
    HIST_FILTER__SYMBOL,
    HIST_FILTER__GUEST,
    HIST_FILTER__HOST,
    HIST_FILTER__SOCKET,
    HIST_FILTER__C2C,
    HIST_FILTER__PARALLELISM,
}

pub type filter_mask_t = u16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hist_column {
    HISTC_SYMBOL,
    HISTC_TIME,
    HISTC_DSO,
    HISTC_THREAD,
    HISTC_TGID,
    HISTC_COMM,
    HISTC_COMM_NODIGIT,
    HISTC_CGROUP_ID,
    HISTC_CGROUP,
    HISTC_PARENT,
    HISTC_PARALLELISM,
    HISTC_CPU,
    HISTC_SOCKET,
    HISTC_SRCLINE,
    HISTC_SRCFILE,
    HISTC_MISPREDICT,
    HISTC_IN_TX,
    HISTC_ABORT,
    HISTC_SYMBOL_FROM,
    HISTC_SYMBOL_TO,
    HISTC_DSO_FROM,
    HISTC_DSO_TO,
    HISTC_LOCAL_WEIGHT,
    HISTC_GLOBAL_WEIGHT,
    HISTC_CODE_PAGE_SIZE,
    HISTC_MEM_DADDR_SYMBOL,
    HISTC_MEM_DADDR_DSO,
    HISTC_MEM_PHYS_DADDR,
    HISTC_MEM_DATA_PAGE_SIZE,
    HISTC_MEM_LOCKED,
    HISTC_MEM_TLB,
    HISTC_MEM_LVL,
    HISTC_MEM_SNOOP,
    HISTC_MEM_DCACHELINE,
    HISTC_MEM_IADDR_SYMBOL,
    HISTC_TRANSACTION,
    HISTC_CYCLES,
    HISTC_SRCLINE_FROM,
    HISTC_SRCLINE_TO,
    HISTC_TRACE,
    HISTC_SYM_SIZE,
    HISTC_DSO_SIZE,
    HISTC_SYMBOL_IPC,
    HISTC_MEM_BLOCKED,
    HISTC_LOCAL_INS_LAT,
    HISTC_GLOBAL_INS_LAT,
    HISTC_LOCAL_P_STAGE_CYC,
    HISTC_GLOBAL_P_STAGE_CYC,
    HISTC_ADDR_FROM,
    HISTC_ADDR_TO,
    HISTC_ADDR,
    HISTC_SIMD,
    HISTC_TYPE,
    HISTC_TYPE_OFFSET,
    HISTC_SYMBOL_OFFSET,
    HISTC_TYPE_CACHELINE,
    HISTC_CALLCHAIN_BRANCH_PREDICTED,
    HISTC_CALLCHAIN_BRANCH_ABORT,
    HISTC_CALLCHAIN_BRANCH_CYCLES,
    HISTC_NR_COLS, /* Last entry */
}

pub const MEM_STAT_LEN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct he_mem_stat {
// meaning of entries depends on enum mem_stat_type
    pub entries: [u64; MEM_STAT_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hists {
    pub entries_in_array: [rb_root_cached; 2],
    pub entries_in: *mut rb_root_cached,
    pub entries: rb_root_cached,
    pub entries_collapsed: rb_root_cached,
    pub nr_entries: u64,
    pub nr_non_filtered_entries: u64,
    pub callchain_period: u64,
    pub callchain_non_filtered_period: u64,
    pub callchain_latency: u64,
    pub callchain_non_filtered_latency: u64,
    pub thread_filter: *mut thread,
    pub dso_filter: *const dso,
    pub uid_filter_str: *const c_char,
    pub symbol_filter_str: *const c_char,
    pub parallelism_filter: *mut c_ulong,
    pub lock: mutex,
    pub stats: hists_stats,
    pub event_stream: u64,
    pub col_len: [u16; HISTC_NR_COLS],
    pub has_callchains: bool,
    pub socket_filter: c_int,
    pub hpp_list: *mut perf_hpp_list,
    pub hpp_formats: list_head,
    pub nr_hpp_node: c_int,
    pub nr_mem_stats: c_int,
    pub mem_stat_types: *mut mem_stat_type,
    pub mem_stat_total: *mut he_mem_stat,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hist_iter_ops {
    pub ): *mut *mut *mut int (prepare_entry)(struct hist_entry_iter , struct addr_location,
    pub ): *mut *mut *mut int (add_single_entry)(struct hist_entry_iter , struct addr_location,
    pub ): *mut *mut *mut int (next_entry)(struct hist_entry_iter , struct addr_location,
    pub ): *mut *mut *mut int (add_next_entry)(struct hist_entry_iter , struct addr_location,
    pub ): *mut *mut *mut int (finish_entry)(struct hist_entry_iter , struct addr_location,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hist_entry_iter {
    pub total: c_int,
    pub curr: c_int,
    pub sample: *mut perf_sample,
    pub he: *mut hist_entry,
    pub parent: *mut symbol,
    pub mi: *mut mem_info,
    pub bi: *mut branch_info,
    pub he_cache: *mut hist_entry,
    pub ops: *const hist_iter_ops,
// user-defined callback function (optional)
    pub arg): *mut *mut addr_location al, bool single, void,
    pub hide_unresolved: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct res_sample {
    pub time: u64,
    pub cpu: c_int,
    pub tid: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct he_stat {
    pub period: u64,
//
// Period re-scaled from CPU time to wall-clock time (divided by the
// parallelism at the time of the sample). This represents effect of
// the event on latency rather than CPU consumption.
//
    pub latency: u64,
    pub period_sys: u64,
    pub period_us: u64,
    pub period_guest_sys: u64,
    pub period_guest_us: u64,
    pub weight1: u64,
    pub weight2: u64,
    pub weight3: u64,
    pub nr_events: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct namespace_id {
    pub dev: u64,
    pub ino: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hist_entry_diff {
    pub computed: bool,
// PERF_HPP__DELTA
    pub period_ratio_delta: double,
// PERF_HPP__RATIO
    pub period_ratio: double,
// HISTC_WEIGHTED_DIFF
    pub wdiff: i64,
// PERF_HPP_DIFF__CYCLES
    pub cycles: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hist_entry_ops {
    pub size): *mut *mut *mut void (new)(size_t,
    pub ptr): *mut *mut void (free)(void,
}

//
// struct hist_entry - histogram entry
//
// @row_offset - offset from the first callchain expanded to appear on screen
// @nr_rows - rows expanded in callchain, recalculated on folding/unfolding
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hist_entry {
    pub rb_node_in: rb_node,
    pub rb_node: rb_node,
    pub node: list_head,
    pub head: list_head,
    pub pairs: },
    pub stat: he_stat,
    pub stat_acc: *mut he_stat,
    pub mem_stat: *mut he_mem_stat,
    pub ms: map_symbol,
    pub thread: *mut thread,
    pub comm: *mut comm,
    pub cgroup_id: namespace_id,
    pub cgroup: u64,
    pub ip: u64,
    pub transaction: u64,
    pub code_page_size: u64,
    pub weight: u64,
    pub ins_lat: u64,
// @weight3: On x86 holds retire_lat, on powerpc holds p_stage_cyc.
    pub weight3: u64,
    pub socket: i32,
    pub cpu: i32,
    pub parallelism: c_int,
    pub mem_type_off: c_int,
    pub cpumode: u8,
    pub depth: u8,
    pub simd_flags: simd_flags,
// We are added by hists__add_dummy_entry.
    pub dummy: bool,
    pub leaf: bool,
    pub level: c_char,
    pub filtered: filter_mask_t,
    pub callchain_size: u16,
//
// Since perf diff only supports the stdio output, TUI
// fields are only accessed from perf report (or perf
// top).  So make it a union to reduce memory usage.
//
    pub diff: hist_entry_diff,
    pub row_offset: u16,
    pub nr_rows: u16,
    pub init_have_children: bool,
    pub unfolded: bool,
    pub has_children: bool,
    pub has_no_entry: bool,
}

// this is for hierarchical entry structure
extern "C" {
    pub fn list_entry(_arg: he->pairs.node.next, hist_entry: struct, _arg: pairs.node) -> return;
}
extern "C" {
    pub fn hist_entry__transaction_len() -> c_int;
}
extern "C" {
    pub fn hist_entry__delete(he: *mut hist_entry);
}
extern "C" {
    pub fn int(he: *mut *mut hists__resort_cb_t)(struct hist_entry, arg: *mut c_void) -> typedef;
}
extern "C" {
    pub fn evsel__output_resort(evsel: *mut evsel, prog: *mut ui_progress);
}
extern "C" {
    pub fn hists__output_resort(hists: *mut hists, prog: *mut ui_progress);
}
extern "C" {
    pub fn hists__collapse_resort(hists: *mut hists, prog: *mut ui_progress) -> c_int;
}
extern "C" {
    pub fn hists__decay_entries(hists: *mut hists, zap_user: bool, zap_kernel: bool);
}
extern "C" {
    pub fn hists__delete_entries(hists: *mut hists);
}
extern "C" {
    pub fn hists__delete_all_entries(hists: *mut hists);
}
extern "C" {
    pub fn hists__output_recalc_col_len(hists: *mut hists, max_rows: c_int);
}
extern "C" {
    pub fn hists__total_period(hists: *mut hists) -> u64;
}
extern "C" {
    pub fn hists__total_latency(hists: *mut hists) -> u64;
}
extern "C" {
    pub fn hists__reset_stats(hists: *mut hists);
}
extern "C" {
    pub fn hists__inc_stats(hists: *mut hists, h: *mut hist_entry);
}
extern "C" {
    pub fn hists__inc_nr_events(hists: *mut hists);
}
extern "C" {
    pub fn hists__inc_nr_samples(hists: *mut hists, filtered: bool);
}
extern "C" {
    pub fn hists__inc_nr_lost_samples(hists: *mut hists, lost: u32);
}
extern "C" {
    pub fn hists__inc_nr_dropped_samples(hists: *mut hists, lost: u32);
}
extern "C" {
    pub fn evlist__fprintf_nr_events(evlist: *mut evlist, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn hists__filter_by_dso(hists: *mut hists);
}
extern "C" {
    pub fn hists__filter_by_thread(hists: *mut hists);
}
extern "C" {
    pub fn hists__filter_by_symbol(hists: *mut hists);
}
extern "C" {
    pub fn hists__filter_by_socket(hists: *mut hists);
}
extern "C" {
    pub fn hists__filter_by_parallelism(hists: *mut hists);
}
extern "C" {
    pub fn hists__col_len(hists: *mut hists, col: hist_column) -> u16;
}
extern "C" {
    pub fn hists__set_col_len(hists: *mut hists, col: hist_column, len: u16);
}
extern "C" {
    pub fn hists__new_col_len(hists: *mut hists, col: hist_column, len: u16) -> bool;
}
extern "C" {
    pub fn hists__reset_col_len(hists: *mut hists);
}
extern "C" {
    pub fn hists__calc_col_len(hists: *mut hists, he: *mut hist_entry);
}
extern "C" {
    pub fn hists__match(leader: *mut hists, other: *mut hists);
}
extern "C" {
    pub fn hists__link(leader: *mut hists, other: *mut hists) -> c_int;
}
extern "C" {
    pub fn hists__unlink(hists: *mut hists) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hists_evsel {
    pub evsel: evsel,
    pub hists: hists,
}

extern "C" {
    pub fn hists__init() -> c_int;
}
extern "C" {
    pub fn __hists__init(hists: *mut hists, hpp_list: *mut perf_hpp_list) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_hpp {
    pub buf: *mut c_char,
    pub size: usize,
    pub sep: *const c_char,
    pub ptr: *mut c_void,
    pub skip: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_hpp_fmt {
    pub name: *const c_char,
    pub span): *mut *mut hists hists, int line, int,
    pub hists): *mut hists,
    pub he): *mut *mut *mut void (init)(struct perf_hpp_fmt fmt, struct hist_entry,
    pub he): *mut hist_entry,
    pub he): *mut hist_entry,
    pub cmp: perf_hpp_fmt_cmp_t,
    pub collapse: perf_hpp_fmt_cmp_t,
    pub sort: perf_hpp_fmt_cmp_t,
    pub b): *mut *mut *mut bool (equal)(struct perf_hpp_fmt a, struct perf_hpp_fmt,
    pub fmt): *mut *mut void (free)(struct perf_hpp_fmt,
    pub list: list_head,
    pub sort_list: list_head,
    pub elide: bool,
    pub len: c_int,
    pub user_len: c_int,
    pub idx: c_int,
    pub level: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_hpp_list {
    pub fields: list_head,
    pub sorts: list_head,
    pub nr_header_lines: c_int,
    pub need_collapse: c_int,
    pub parent: c_int,
    pub sym: c_int,
    pub dso: c_int,
    pub socket: c_int,
    pub thread: c_int,
    pub comm: c_int,
    pub comm_nodigit: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_hpp_list_node {
    pub list: list_head,
    pub hpp: perf_hpp_list,
    pub level: c_int,
    pub skip: bool,
}

// Matches perf_hpp__format array.
extern "C" {
    pub fn perf_hpp__init();
}
extern "C" {
    pub fn perf_hpp__cancel_cumulate(evlist: *mut evlist);
}
extern "C" {
    pub fn perf_hpp__cancel_latency(evlist: *mut evlist);
}
extern "C" {
    pub fn perf_hpp__setup_output_field(list: *mut perf_hpp_list);
}
extern "C" {
    pub fn perf_hpp__reset_output_field(list: *mut perf_hpp_list);
}
extern "C" {
    pub fn perf_hpp__append_sort_keys(list: *mut perf_hpp_list);
}
extern "C" {
    pub fn perf_hpp__is_sort_entry(format: *mut perf_hpp_fmt) -> bool;
}
extern "C" {
    pub fn perf_hpp__is_dynamic_entry(format: *mut perf_hpp_fmt) -> bool;
}
extern "C" {
    pub fn perf_hpp__defined_dynamic_entry(fmt: *mut perf_hpp_fmt, hists: *mut hists) -> bool;
}
extern "C" {
    pub fn perf_hpp__is_trace_entry(fmt: *mut perf_hpp_fmt) -> bool;
}
extern "C" {
    pub fn perf_hpp__is_srcline_entry(fmt: *mut perf_hpp_fmt) -> bool;
}
extern "C" {
    pub fn perf_hpp__is_srcfile_entry(fmt: *mut perf_hpp_fmt) -> bool;
}
extern "C" {
    pub fn perf_hpp__is_thread_entry(fmt: *mut perf_hpp_fmt) -> bool;
}
extern "C" {
    pub fn perf_hpp__is_comm_entry(fmt: *mut perf_hpp_fmt) -> bool;
}
extern "C" {
    pub fn perf_hpp__is_dso_entry(fmt: *mut perf_hpp_fmt) -> bool;
}
extern "C" {
    pub fn perf_hpp__is_sym_entry(fmt: *mut perf_hpp_fmt) -> bool;
}
extern "C" {
    pub fn perf_hpp__is_parallelism_entry(fmt: *mut perf_hpp_fmt) -> bool;
}
extern "C" {
    pub fn hist_entry__filter(he: *mut hist_entry, type: c_int, arg: *const c_void) -> c_int;
}
extern "C" {
    pub fn perf_hpp__reset_width(fmt: *mut perf_hpp_fmt, hists: *mut hists);
}
extern "C" {
    pub fn perf_hpp__reset_sort_width(fmt: *mut perf_hpp_fmt, hists: *mut hists);
}
extern "C" {
    pub fn perf_hpp__set_user_width(width_list_str: *const c_char);
}
extern "C" {
    pub fn hists__reset_column_width(hists: *mut hists);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_hpp_fmt_type {
    PERF_HPP_FMT_TYPE__RAW,
    PERF_HPP_FMT_TYPE__PERCENT,
    PERF_HPP_FMT_TYPE__LATENCY,
    PERF_HPP_FMT_TYPE__AVERAGE,
}

extern "C" {
    pub fn u64(he: *mut *mut hpp_field_fn)(struct hist_entry) -> typedef;
}
extern "C" {
    pub fn int(hpp: *mut *mut hpp_callback_fn)(struct perf_hpp, front: bool) -> typedef;
}
extern "C" {
    pub fn int(hpp: *mut *mut hpp_snprint_fn)(struct perf_hpp, fmt: *const c_char, ...) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hist_browser_timer {
    pub arg): *mut *mut void (timer)(void,
    pub arg: *mut c_void,
    pub refresh: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rstype {
    A_NORMAL,
    A_ASM,
    A_SOURCE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct block_hist {
    pub block_hists: hists,
    pub block_list: perf_hpp_list,
    pub block_fmt: perf_hpp_fmt,
    pub block_idx: c_int,
    pub valid: bool,
    pub he: hist_entry,
}

pub const NO_ADDR: c_int = 0;

extern "C" {
    pub fn attr_to_script(buf: *mut c_char, attr: *mut perf_event_attr);
}
extern "C" {
    pub fn script_browse(script_opt: *const c_char, evsel: *mut evsel) -> c_int;
}
extern "C" {
    pub fn run_script(cmd: *mut c_char);
}
extern "C" {
    pub fn res_sample_init();
}

extern "C" {
    pub fn hists__sort_list_width(hists: *mut hists) -> c_uint;
}
extern "C" {
    pub fn hists__overhead_width(hists: *mut hists) -> c_uint;
}
extern "C" {
    pub fn parse_filter_percentage(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
}
extern "C" {
    pub fn perf_hist_config(var: *const c_char, value: *const c_char) -> c_int;
}
extern "C" {
    pub fn perf_hpp_list__init(list: *mut perf_hpp_list);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hierarchy_move_dir {
    HMD_NORMAL,
    HMD_FORCE_SIBLING,
    HMD_FORCE_CHILD,
}

extern "C" {
    pub fn __rb_hierarchy_next(_arg: node, _arg: HMD_NORMAL) -> return;
}
pub const HIERARCHY_INDENT: c_int = 3;
extern "C" {
    pub fn hist_entry__has_hierarchy_children(he: *mut hist_entry, limit: float) -> bool;
}
extern "C" {
    pub fn hpp_color_scnprintf(hpp: *mut perf_hpp, fmt: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn __hpp__slsmg_color_printf(hpp: *mut perf_hpp, fmt: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn hists__fprintf_headers(hists: *mut hists, fp: *mut FILE) -> c_int;
}
extern "C" {
    pub fn __hists__scnprintf_title(hists: *mut hists, bf: *mut c_char, size: usize, show_freq: bool) -> c_int;
}
extern "C" {
    pub fn __hists__scnprintf_title(_arg: hists, _arg: bf, _arg: size, _arg: true) -> return;
}
