//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/evlist.h
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
pub const __PERF_EVLIST_H: c_int = 1;

//
// State machine of bkw_mmap_state:
//
// .________________(forbid)_____________.
// |                                     V
// NOTREADY --(0)--> RUNNING --(1)--> DATA_PENDING --(2)--> EMPTY
// ^  ^              |   ^               |
// |  |__(forbid)____/   |___(forbid)___/|
// |                                     |
// \_________________(3)_______________
//
// NOTREADY     : Backward ring buffers are not ready
// RUNNING      : Backward ring buffers are recording
// DATA_PENDING : We are required to collect data from backward ring buffers
// EMPTY        : We have collected data from backward ring buffers.
//
// (0): Setup backward ring buffer
// (1): Pause ring buffers for reading
// (2): Read from ring buffers
// (3): Resume ring buffers for recording
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bkw_mmap_state {
    BKW_MMAP_NOTREADY,
    BKW_MMAP_RUNNING,
    BKW_MMAP_DATA_PENDING,
    BKW_MMAP_EMPTY,
}

//
// @metric_events: A list of struct metric_event which each have a list
// of struct metric_expr.
//
// samples with deferred_callchain would wait here.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evsel_str_handler {
    pub name: *const c_char,
    pub handler: *mut c_void,
}

extern "C" {
    pub fn evlist__put(evlist: *mut evlist);
}
extern "C" {
    pub fn evlist__add(evlist: *mut evlist, entry: *mut evsel);
}
extern "C" {
    pub fn evlist__remove(evlist: *mut evlist, evsel: *mut evsel);
}
extern "C" {
    pub fn arch_evlist__cmp(lhs: *const evsel, rhs: *const evsel) -> c_int;
}
extern "C" {
    pub fn arch_evlist__add_required_events(list: *mut list_head) -> c_int;
}
extern "C" {
    pub fn evlist__add_dummy(evlist: *mut evlist) -> c_int;
}
extern "C" {
    pub fn evlist__add_aux_dummy(_arg: evlist, _arg: true) -> return;
}

extern "C" {
    pub fn evlist__set_cb(evlist: *mut evlist, cb: evsel__sb_cb_t, data: *mut c_void);
}
extern "C" {
    pub fn evlist__start_sb_thread(evlist: *mut evlist, target: *mut target) -> c_int;
}
extern "C" {
    pub fn evlist__stop_sb_thread(evlist: *mut evlist);
}

extern "C" {
    pub fn evlist__add_newtp(evlist: *mut evlist, sys: *const c_char, name: *const c_char, handler: *mut c_void) -> c_int;
}

extern "C" {
    pub fn evlist__set_tp_filter(evlist: *mut evlist, filter: *const c_char) -> c_int;
}
extern "C" {
    pub fn evlist__set_tp_filter_pids(evlist: *mut evlist, npids: usize, pids: *mut pid_t) -> c_int;
}
extern "C" {
    pub fn evlist__append_tp_filter(evlist: *mut evlist, filter: *const c_char) -> c_int;
}
extern "C" {
    pub fn evlist__append_tp_filter_pid(evlist: *mut evlist, pid: pid_t) -> c_int;
}
extern "C" {
    pub fn evlist__append_tp_filter_pids(evlist: *mut evlist, npids: usize, pids: *mut pid_t) -> c_int;
}
extern "C" {
    pub fn evlist__add_pollfd(evlist: *mut evlist, fd: c_int) -> c_int;
}
extern "C" {
    pub fn evlist__filter_pollfd(evlist: *mut evlist, revents_and_mask: c_short) -> c_int;
}

extern "C" {
    pub fn evlist__add_wakeup_eventfd(evlist: *mut evlist, fd: c_int) -> c_int;
}

extern "C" {
    pub fn evlist__poll(evlist: *mut evlist, timeout: c_int) -> c_int;
}
extern "C" {
    pub fn evlist__toggle_bkw_mmap(evlist: *mut evlist, state: bkw_mmap_state);
}
extern "C" {
    pub fn evlist__mmap_consume(evlist: *mut evlist, idx: c_int);
}
extern "C" {
    pub fn evlist__open(evlist: *mut evlist) -> c_int;
}
extern "C" {
    pub fn evlist__close(evlist: *mut evlist);
}
extern "C" {
    pub fn evlist__set_id_pos(evlist: *mut evlist);
}
extern "C" {
    pub fn evlist__config(evlist: *mut evlist, opts: *mut record_opts, callchain: *mut callchain_param);
}
extern "C" {
    pub fn record_opts__config(opts: *mut record_opts) -> c_int;
}
extern "C" {
    pub fn evlist__start_workload(evlist: *mut evlist) -> c_int;
}
extern "C" {
    pub fn evlist__cancel_workload(evlist: *mut evlist);
}
extern "C" {
    pub fn __evlist__parse_mmap_pages(mmap_pages: *mut c_uint, str: *const c_char) -> c_int;
}
extern "C" {
    pub fn evlist__parse_mmap_pages(opt: *const option, str: *const c_char, unset: c_int) -> c_int;
}
extern "C" {
    pub fn perf_event_mlock_kb_in_pages() -> c_ulong;
}
extern "C" {
    pub fn evlist__do_mmap(evlist: *mut evlist, pages: c_uint) -> c_int;
}
extern "C" {
    pub fn evlist__do_munmap(evlist: *mut evlist);
}
extern "C" {
    pub fn evlist__mmap_size(pages: c_ulong) -> usize;
}
extern "C" {
    pub fn evlist__disable(evlist: *mut evlist);
}
extern "C" {
    pub fn evlist__enable(evlist: *mut evlist);
}
extern "C" {
    pub fn evlist__toggle_enable(evlist: *mut evlist);
}
extern "C" {
    pub fn evlist__disable_evsel(evlist: *mut evlist, evsel_name: *mut c_char);
}
extern "C" {
    pub fn evlist__enable_evsel(evlist: *mut evlist, evsel_name: *mut c_char);
}
extern "C" {
    pub fn evlist__disable_non_dummy(evlist: *mut evlist);
}
extern "C" {
    pub fn evlist__enable_non_dummy(evlist: *mut evlist);
}
extern "C" {
    pub fn evlist__create_maps(evlist: *mut evlist, target: *mut target) -> c_int;
}
extern "C" {
    pub fn __evlist__combined_sample_type(evlist: *mut evlist) -> u64;
}
extern "C" {
    pub fn evlist__combined_sample_type(evlist: *mut evlist) -> u64;
}
extern "C" {
    pub fn evlist__combined_branch_type(evlist: *mut evlist) -> u64;
}
extern "C" {
    pub fn evlist__update_br_cntr(evlist: *mut evlist);
}
extern "C" {
    pub fn evlist__sample_id_all(evlist: *mut evlist) -> bool;
}
extern "C" {
    pub fn evlist__id_hdr_size(evlist: *mut evlist) -> u16;
}
extern "C" {
    pub fn evlist__parse_sample(evlist: *mut evlist, event: *mut perf_event, sample: *mut perf_sample) -> c_int;
}
extern "C" {
    pub fn evlist__parse_sample_timestamp(evlist: *mut evlist, event: *mut perf_event, timestamp: *mut u64) -> c_int;
}
extern "C" {
    pub fn evlist__valid_sample_type(evlist: *mut evlist) -> bool;
}
extern "C" {
    pub fn evlist__valid_sample_id_all(evlist: *mut evlist) -> bool;
}
extern "C" {
    pub fn evlist__valid_read_format(evlist: *mut evlist) -> bool;
}
extern "C" {
    pub fn evlist__splice_list_tail(evlist: *mut evlist, list: *mut list_head);
}
extern "C" {
    pub fn list_empty(_arg: &evlist__core(evlist)->entries) -> return;
}
extern "C" {
    pub fn container_of(_arg: evsel, evsel: struct, _arg: core) -> return;
}
extern "C" {
    pub fn container_of(_arg: evsel, evsel: struct, _arg: core) -> return;
}
extern "C" {
    pub fn perf_evlist__nr_groups(_arg: evlist__core(evlist)) -> return;
}
extern "C" {
    pub fn evlist__strerror_open(evlist: *mut evlist, err: c_int, buf: *mut c_char, size: usize) -> c_int;
}
extern "C" {
    pub fn evlist__strerror_mmap(evlist: *mut evlist, err: c_int, buf: *mut c_char, size: usize) -> c_int;
}
extern "C" {
    pub fn evlist__can_select_event(evlist: *mut evlist, str: *const c_char) -> bool;
}
extern "C" {
    pub fn evlist__to_front(evlist: *mut evlist, move_evsel: *mut evsel);
}
//
// __evlist__for_each_entry - iterate thru all the evsels
// @list: list_head instance to iterate
// @evsel: struct evsel iterator
//

//
// evlist__for_each_entry - iterate thru all the evsels
// @evlist: evlist instance to iterate
// @evsel: struct evsel iterator
//

//
// __evlist__for_each_entry_continue - continue iteration thru all the evsels
// @list: list_head instance to iterate
// @evsel: struct evsel iterator
//

//
// evlist__for_each_entry_continue - continue iteration thru all the evsels
// @evlist: evlist instance to iterate
// @evsel: struct evsel iterator
//

//
// __evlist__for_each_entry_from - continue iteration from @evsel (included)
// @list: list_head instance to iterate
// @evsel: struct evsel iterator
//

//
// evlist__for_each_entry_from - continue iteration from @evsel (included)
// @evlist: evlist instance to iterate
// @evsel: struct evsel iterator
//

//
// __evlist__for_each_entry_reverse - iterate thru all the evsels in reverse order
// @list: list_head instance to iterate
// @evsel: struct evsel iterator
//

//
// evlist__for_each_entry_reverse - iterate thru all the evsels in reverse order
// @evlist: evlist instance to iterate
// @evsel: struct evsel iterator
//

//
// __evlist__for_each_entry_safe - safely iterate thru all the evsels
// @list: list_head instance to iterate
// @tmp: struct evsel temp iterator
// @evsel: struct evsel iterator
//

//
// evlist__for_each_entry_safe - safely iterate thru all the evsels
// @evlist: evlist instance to iterate
// @evsel: struct evsel iterator
// @tmp: struct evsel temp iterator
//

// Iterator state for evlist__for_each_cpu
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evlist_cpu_iterator {
// The list being iterated through.
    pub container: *mut evlist,
// The current evsel of the iterator.
    pub evsel: *mut evsel,
// The CPU map index corresponding to the evsel->core.cpus for the current CPU.
    pub cpu_map_idx: c_int,
//
// The CPU map index corresponding to evlist->core.all_cpus for the
// current CPU.  Distinct from cpu_map_idx as the evsel's cpu map may
// contain fewer entries.
//
    pub evlist_cpu_map_idx: c_int,
// The number of CPU map entries in evlist->core.all_cpus.
    pub evlist_cpu_map_nr: c_int,
// The current CPU of the iterator.
    pub cpu: perf_cpu,
// If present, used to set the affinity when switching between CPUs.
    pub affinity: *mut affinity,
// Maybe be used to hold affinity state prior to iterating.
    pub saved_affinity: affinity,
}

//
// evlist__for_each_cpu - without affinity, iterate over the evlist. With
// affinity, iterate over all CPUs and then the evlist
// for each evsel on that CPU. When switching between
// CPUs the affinity is set to the CPU to avoid IPIs
// during syscalls. The affinity is set up and removed
// automatically, if the loop is broken a call to
// evlist_cpu_iterator__exit is necessary.
// @evlist_cpu_itr: the iterator instance.
// @evlist: evlist instance to iterate.
//

// Setup an iterator set to the first CPU/evsel of evlist.
extern "C" {
    pub fn evlist_cpu_iterator__init(itr: *mut evlist_cpu_iterator, evlist: *mut evlist);
}
//
// Cleans up the iterator, automatically done by evlist_cpu_iterator__next when
// the end of the list is reached. Multiple calls are safe.
//
extern "C" {
    pub fn evlist_cpu_iterator__exit(itr: *mut evlist_cpu_iterator);
}
// Move to next element in iterator, updating CPU, evsel and the affinity.
extern "C" {
    pub fn evlist_cpu_iterator__next(evlist_cpu_itr: *mut evlist_cpu_iterator);
}
// Returns true when iterator is at the end of the CPUs and evlist.
extern "C" {
    pub fn evlist__set_tracking_event(evlist: *mut evlist, tracking_evsel: *mut evsel);
}
extern "C" {
    pub fn evlist__exclude_kernel(evlist: *mut evlist) -> bool;
}
extern "C" {
    pub fn evlist_leader(evlist: *mut evlist);
}

pub const EVLIST_CTL_CMD_MAX_LEN: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum evlist_ctl_cmd {
    EVLIST_CTL_CMD_UNSUPPORTED = 0,
    EVLIST_CTL_CMD_ENABLE,
    EVLIST_CTL_CMD_DISABLE,
    EVLIST_CTL_CMD_ACK,
    EVLIST_CTL_CMD_SNAPSHOT,
    EVLIST_CTL_CMD_EVLIST,
    EVLIST_CTL_CMD_STOP,
    EVLIST_CTL_CMD_PING,
}

extern "C" {
    pub fn evlist__parse_control(str: *const c_char, ctl_fd: *mut c_int, ctl_fd_ack: *mut c_int, ctl_fd_close: *mut bool) -> c_int;
}
extern "C" {
    pub fn evlist__close_control(ctl_fd: c_int, ctl_fd_ack: c_int, ctl_fd_close: *mut bool);
}
extern "C" {
    pub fn evlist__initialize_ctlfd(evlist: *mut evlist, ctl_fd: c_int, ctl_fd_ack: c_int) -> c_int;
}
extern "C" {
    pub fn evlist__finalize_ctlfd(evlist: *mut evlist) -> c_int;
}
extern "C" {
    pub fn evlist__ctlfd_initialized(evlist: *mut evlist) -> bool;
}
extern "C" {
    pub fn evlist__ctlfd_process(evlist: *mut evlist, cmd: *mut evlist_ctl_cmd) -> c_int;
}
extern "C" {
    pub fn evlist__ctlfd_ack(evlist: *mut evlist) -> c_int;
}

extern "C" {
    pub fn event_enable_timer__start(eet: *mut event_enable_timer) -> c_int;
}
extern "C" {
    pub fn event_enable_timer__process(eet: *mut event_enable_timer) -> c_int;
}
extern "C" {
    pub fn evlist__format_evsels(evlist: *mut evlist, sb: *mut strbuf, max_length: usize);
}
extern "C" {
    pub fn evlist__check_mem_load_aux(evlist: *mut evlist);
}
extern "C" {
    pub fn evlist__warn_user_requested_cpus(evlist: *mut evlist, cpu_list: *const c_char);
}
extern "C" {
    pub fn evlist__uniquify_evsel_names(evlist: *mut evlist, config: *const perf_stat_config);
}
extern "C" {
    pub fn evlist__has_bpf_output(evlist: *mut evlist) -> bool;
}
extern "C" {
    pub fn evlist__needs_bpf_sb_event(evlist: *mut evlist) -> bool;
}
