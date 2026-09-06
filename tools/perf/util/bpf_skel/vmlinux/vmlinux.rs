//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/bpf_skel/vmlinux/vmlinux.h
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


// non-UAPI kernel data structures, used in the .bpf.c BPF tool component.
// Just the fields used in these tools preserving the access index so that
// libbpf can fixup offsets with the ones used in the kernel when loading the
// BPF bytecode, if they differ from what is used here.
pub type u8 = __u8;
pub type u32 = __u32;
pub type s32 = __s32;
pub type u64 = __u64;
pub type s64 = __s64;
pub type pid_t = c_int;
pub type time64_t = __s64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timespec64 {
    pub tv_sec: time64_t,
    pub tv_nsec: long int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cgroup_subsys_id {
    perf_event_cgrp_id  = 8,
}

pub type atomic_long_t = core::sync::atomic::AtomicI64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_spinlock {
    pub rawlock: c_int,
    pub __attribute__((preserve_access_index)): },
pub type raw_spinlock_t = raw_spinlock;
    pub rlock: raw_spinlock,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sighand_struct {
    pub siglock: spinlock_t,
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rw_semaphore {
    pub owner: atomic_long_t,
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mutex {
    pub owner: atomic_long_t,
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernfs_node {
    pub id: u64,
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgroup {
    pub kn: *mut kernfs_node,
    pub level: c_int,
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgroup_subsys_state {
    pub cgroup: *mut cgroup,
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct css_set {
    pub subsys: [*mut cgroup_subsys_state; 13],
    pub dfl_cgrp: *mut cgroup,
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm_struct {
    pub mmap_lock: rw_semaphore,
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct {
    pub flags: c_uint,
    pub mm: *mut mm_struct,
    pub pid: pid_t,
    pub tgid: pid_t,
    pub comm: [c_char; 16],
    pub sighand: *mut sighand_struct,
    pub cgroups: *mut css_set,
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_entry {
    pub type: short unsigned int,
    pub flags: c_uchar,
    pub preempt_count: c_uchar,
    pub pid: c_int,
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_event_raw_irq_handler_entry {
    pub ent: trace_entry,
    pub irq: c_int,
    pub __data_loc_name: u32,
    pub __data: [c_char; ],
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_event_raw_irq_handler_exit {
    pub ent: trace_entry,
    pub irq: c_int,
    pub ret: c_int,
    pub __data: [c_char; ],
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_event_raw_softirq {
    pub ent: trace_entry,
    pub vec: c_uint,
    pub __data: [c_char; ],
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_event_raw_workqueue_execute_start {
    pub ent: trace_entry,
    pub work: *mut c_void,
    pub function: *mut c_void,
    pub __data: [c_char; ],
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_event_raw_workqueue_execute_end {
    pub ent: trace_entry,
    pub work: *mut c_void,
    pub function: *mut c_void,
    pub __data: [c_char; ],
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_event_raw_workqueue_activate_work {
    pub ent: trace_entry,
    pub work: *mut c_void,
    pub __data: [c_char; ],
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_sample_data {
    pub addr: u64,
    pub period: u64,
    pub weight: perf_sample_weight,
    pub txn: u64,
    pub data_src: perf_mem_data_src,
    pub ip: u64,
    pub pid: u32,
    pub tid: u32,
    pub tid_entry: },
    pub time: u64,
    pub id: u64,
    pub cpu: u32,
    pub cpu_entry: },
    pub phys_addr: u64,
    pub cgroup: u64,
    pub data_page_size: u64,
    pub code_page_size: u64,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_event {
    pub parent: *mut perf_event,
    pub id: u64,
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_perf_event_data_kern {
    pub data: *mut perf_sample_data,
    pub event: *mut perf_event,
    pub __attribute__((preserve_access_index)): },
//
// If 'struct rq' isn't defined for lock_contention.bpf.c, for the sake of
// rq___old and rq___new, then the type for the 'runqueue' variable ends up
// being a forward declaration (BTF_KIND_FWD) while the kernel has it defined
// (BTF_KIND_STRUCT). The definition appears in vmlinux.h rather than
// lock_contention.bpf.c for consistency with a generated vmlinux.h.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rq {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kmem_cache {
    pub name: *const c_char,
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter__kmem_cache {
    pub s: *mut kmem_cache,
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zone {
    pub lock: spinlock_t,
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pglist_data {
    pub /: *mut *mut zone node_zones[6]; / value for all possible config,
    pub nr_zones: c_int,
    pub __attribute__((preserve_access_index)): },
