//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/record.h
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
pub struct record_opts {
    pub target: target,
    pub inherit_stat: bool,
    pub no_buffering: bool,
    pub no_inherit: bool,
    pub no_inherit_set: bool,
    pub no_samples: bool,
    pub raw_samples: bool,
    pub sample_address: bool,
    pub sample_phys_addr: bool,
    pub sample_data_page_size: bool,
    pub sample_code_page_size: bool,
    pub sample_weight: bool,
    pub sample_time: bool,
    pub sample_time_set: bool,
    pub sample_cpu: bool,
    pub sample_identifier: bool,
    pub sample_data_src: bool,
    pub period: bool,
    pub period_set: bool,
    pub running_time: bool,
    pub full_auxtrace: bool,
    pub auxtrace_snapshot_mode: bool,
    pub auxtrace_snapshot_on_exit: bool,
    pub auxtrace_sample_mode: bool,
    pub record_namespaces: bool,
    pub record_cgroup: bool,
    pub record_switch_events: bool,
    pub record_switch_events_set: bool,
    pub record_data_mmap: bool,
    pub record_data_mmap_set: bool,
    pub all_kernel: bool,
    pub all_user: bool,
    pub kernel_callchains: bool,
    pub user_callchains: bool,
    pub tail_synthesize: bool,
    pub overwrite: bool,
    pub ignore_missing_thread: bool,
    pub strict_freq: bool,
    pub sample_id: bool,
    pub no_bpf_event: bool,
    pub kcore: bool,
    pub text_poke: bool,
    pub build_id: bool,
    pub freq: c_uint,
    pub mmap_pages: c_uint,
    pub auxtrace_mmap_pages: c_uint,
    pub user_freq: c_uint,
    pub branch_stack: u64,
    pub sample_intr_regs: u64,
    pub sample_user_regs: u64,
    pub default_interval: u64,
    pub user_interval: u64,
    pub auxtrace_snapshot_size: usize,
    pub auxtrace_snapshot_opts: *const c_char,
    pub auxtrace_sample_opts: *const c_char,
    pub sample_transaction: bool,
    pub use_clockid: bool,
    pub clockid: clockid_t,
    pub clockid_res_ns: u64,
    pub nr_cblocks: c_int,
    pub affinity: c_int,
    pub mmap_flush: c_int,
    pub comp_level: c_uint,
    pub nr_threads_synthesize: c_uint,
    pub ctl_fd: c_int,
    pub ctl_fd_ack: c_int,
    pub ctl_fd_close: bool,
    pub synth: c_int,
    pub threads_spec: c_int,
    pub threads_user_spec: *const c_char,
    pub off_cpu_thresh_ns: u64,
}

extern "C" {
    pub fn record__parse_freq(opt: *const option, str: *const c_char, unset: c_int) -> c_int;
}
