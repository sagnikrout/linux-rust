//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/kvm-stat.h
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

pub const KVM_EVENT_NAME_LEN: c_int = 40;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_key {

    pub key: u64,
    pub info: c_int,
    pub exit_reasons: *mut exit_reasons_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_info {
    pub name: [c_char; KVM_EVENT_NAME_LEN],
    pub refcnt: refcount_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_event_stats {
    pub time: u64,
    pub stats: stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_event {
    pub hash_entry: list_head,
    pub perf_kvm: *mut perf_kvm_stat,
    pub key: event_key,
    pub total: kvm_event_stats,
pub const DEFAULT_VCPU_NUM: c_int = 8;
    pub max_vcpu: c_int,
    pub vcpu: *mut kvm_event_stats,
    pub he: hist_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct child_event_ops {
    pub key): *mut event_key,
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_events_ops {
    pub key): *mut event_key,
    pub key): *mut *mut *mut bool (is_end_event)(struct perf_sample sample, struct event_key,
    pub child_ops: *const child_event_ops,
    pub decode): *mut c_char,
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exit_reasons_table {
    pub exit_code: u64,
    pub reason: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_kvm_stat {
    pub tool: perf_tool,
    pub opts: record_opts,
    pub evlist: *mut evlist,
    pub session: *mut perf_session,
    pub file_name: *const c_char,
    pub report_event: *const c_char,
    pub sort_key: *const c_char,
    pub trace_vcpu: c_int,
// Used when process events
    pub al: addr_location,
    pub exit_reasons: *mut exit_reasons_table,
    pub exit_reasons_isa: *const c_char,
    pub events_ops: *const kvm_events_ops,
    pub total_time: u64,
    pub total_count: u64,
    pub lost_events: u64,
    pub duration: u64,
    pub pid_list: *mut intlist,
    pub timerfd: c_int,
    pub display_time: c_uint,
    pub live: bool,
    pub force: bool,
    pub use_stdio: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_reg_events_ops {
    pub name: *const c_char,
    pub ops: *const kvm_events_ops,
}

extern "C" {
    pub fn kvm_exit_event(evsel: *mut evsel) -> bool;
}
extern "C" {
    pub fn kvm_entry_event(evsel: *mut evsel) -> bool;
}

//
// arch specific callbacks and data structures
//
extern "C" {
    pub fn setup_kvm_events_tp(kvm: *mut perf_kvm_stat, e_machine: u16) -> c_int;
}
extern "C" {
    pub fn __setup_kvm_events_tp_powerpc(kvm: *mut perf_kvm_stat) -> c_int;
}
extern "C" {
    pub fn cpu_isa_init(kvm: *mut perf_kvm_stat, e_machine: u16, cpuid: *const c_char) -> c_int;
}
extern "C" {
    pub fn __cpu_isa_init_arm64(kvm: *mut perf_kvm_stat) -> c_int;
}
extern "C" {
    pub fn __cpu_isa_init_loongarch(kvm: *mut perf_kvm_stat) -> c_int;
}
extern "C" {
    pub fn __cpu_isa_init_powerpc(kvm: *mut perf_kvm_stat) -> c_int;
}
extern "C" {
    pub fn __cpu_isa_init_riscv(kvm: *mut perf_kvm_stat) -> c_int;
}
extern "C" {
    pub fn __cpu_isa_init_s390(kvm: *mut perf_kvm_stat, cpuid: *const c_char) -> c_int;
}
extern "C" {
    pub fn __cpu_isa_init_x86(kvm: *mut perf_kvm_stat, cpuid: *const c_char) -> c_int;
}
extern "C" {
    pub fn kvm_need_default_arch_event(e_machine: u16, argc: c_int, argv: *const c_char) -> bool;
}
extern "C" {
    pub fn kvm_add_default_arch_event(e_machine: u16, argc: *mut c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn __kvm_add_default_arch_event_powerpc(argc: *mut c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn __kvm_add_default_arch_event_x86(argc: *mut c_int, argv: *const c_char) -> c_int;
}

// ki = NULL;

