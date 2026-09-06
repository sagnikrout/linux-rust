//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sched/topology.h
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
// sched-domains (multiprocessor balancing) declarations:
//
// Generate SD flag indexes

// Generate SD flag bits

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sd_flag_debug {
    pub meta_flags: c_uint,
    pub name: *mut c_char,
}

extern "C" {
    pub fn cpu_smt_flags() -> c_int;
}

extern "C" {
    pub fn cpu_cluster_flags() -> c_int;
}

extern "C" {
    pub fn cpu_core_flags() -> c_int;
}

extern "C" {
    pub fn arch_asym_cpu_priority(cpu: c_int) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_domain_attr {
    pub relax_domain_level: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_domain_shared {
    pub ref: core::sync::atomic::AtomicI32,
    pub nr_busy_cpus: core::sync::atomic::AtomicI32,
    pub has_idle_cores: c_int,
    pub nr_idle_scan: c_int,
//
// Used during allocation to claim the sched_domain_shared
// object at multiple levels.
//
// Note: between build and the first periodic LB tick, which
// rewrites the union via update_idle_cpu_scan(), readers of
// nr_idle_scan may observe the transient SD_* flag value as
// the scan bound. The flag bits are small positive integers,
// so the effect is just a slightly relaxed scan bound for one
// window and self-heals on the first tick.
//
    pub alloc_flags: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_domain {
// These fields must be setup
    pub /: *mut *mut *mut sched_domain __rcu parent; / top domain must be null terminated,
    pub /: *mut *mut *mut sched_domain __rcu child; / bottom domain must be null terminated,
    pub /: *mut *mut *mut sched_group groups; / the balancing groups of the domain,
    pub /: *mut *mut unsigned long min_interval; / Minimum balance interval ms,
    pub /: *mut *mut unsigned long max_interval; / Maximum balance interval ms,
    pub /: *mut *mut unsigned int busy_factor; / less balancing by factor if busy,
    pub /: *mut *mut unsigned int imbalance_pct; / No balance until over watermark,
    pub /: *mut *mut unsigned int cache_nice_tries; / Leave cache hot tasks for # tries,
    pub /: *mut *mut unsigned int imb_numa_nr; / Nr running tasks that allows a NUMA imbalance,
    pub /: *mut *mut int nohz_idle; / NOHZ IDLE status,
    pub /: *mut *mut *mut int flags; / See SD_,
    pub level: c_int,
// Runtime fields.
    pub /: *mut *mut unsigned long last_balance; / init to jiffies. units in jiffies,
    pub /: *mut *mut unsigned int balance_interval; / initialise to 1. units in ms.,
    pub /: *mut *mut unsigned int nr_balance_failed; / initialise to 0,
// idle_balance() stats
    pub newidle_call: c_uint,
    pub newidle_success: c_uint,
    pub newidle_ratio: c_uint,
    pub newidle_stamp: u64,
    pub max_newidle_lb_cost: u64,
    pub last_decay_max_lb_cost: c_ulong,

    pub llc_max: c_uint,
    pub __counted_by_ptr(llc_max): *mut *mut unsigned int llc_counts,
    pub llc_bytes: c_ulong,

// sched_balance_rq() stats
    pub lb_count: [c_uint; CPU_MAX_IDLE_TYPES],
    pub lb_failed: [c_uint; CPU_MAX_IDLE_TYPES],
    pub lb_balanced: [c_uint; CPU_MAX_IDLE_TYPES],
    pub lb_imbalance_load: [c_uint; CPU_MAX_IDLE_TYPES],
    pub lb_imbalance_util: [c_uint; CPU_MAX_IDLE_TYPES],
    pub lb_imbalance_task: [c_uint; CPU_MAX_IDLE_TYPES],
    pub lb_imbalance_misfit: [c_uint; CPU_MAX_IDLE_TYPES],
    pub lb_gained: [c_uint; CPU_MAX_IDLE_TYPES],
    pub lb_hot_gained: [c_uint; CPU_MAX_IDLE_TYPES],
    pub lb_nobusyg: [c_uint; CPU_MAX_IDLE_TYPES],
    pub lb_nobusyq: [c_uint; CPU_MAX_IDLE_TYPES],
// Active load balancing
    pub alb_count: c_uint,
    pub alb_failed: c_uint,
    pub alb_pushed: c_uint,
// SD_BALANCE_EXEC stats
    pub sbe_count: c_uint,
    pub sbe_balanced: c_uint,
    pub sbe_pushed: c_uint,
// SD_BALANCE_FORK stats
    pub sbf_count: c_uint,
    pub sbf_balanced: c_uint,
    pub sbf_pushed: c_uint,
// try_to_wake_up() stats
    pub ttwu_wake_remote: c_uint,
    pub ttwu_move_affine: c_uint,
    pub ttwu_move_balance: c_uint,

    pub name: *mut c_char,
    pub /: *mut *mut *mut void private; / used during construction,
    pub /: *mut *mut rcu_head rcu; / used during destruction,
}

//
// See sched_domain_span(), on why flex arrays are broken.
//
// Turns out that C flexible arrays are fundamentally broken since it
// is allowed for offsetof(*sd, span) < sizeof(*sd), this means that
// structure initialzation *sd = { ... }; which writes every byte
// inside sizeof(*type), will over-write the start of the flexible
// array.
//
// Luckily, the way we allocate sched_domain is by:
//
// sizeof(*sd) + cpumask_size()
//
// this means that we have sufficient space for the whole flex array
// *outside* of sizeof(*sd). So use that, and avoid using sd->span.
//
extern "C" {
    pub fn to_cpumask(_arg: bitmap) -> return;
}
// Allocate an array of sched domains, for partition_sched_domains().
extern "C" {
    pub fn free_sched_domains(doms[]: cpumask_var_t, ndoms: c_uint);
}
extern "C" {
    pub fn cpus_equal_capacity(this_cpu: c_int, that_cpu: c_int) -> bool;
}
extern "C" {
    pub fn cpus_share_cache(this_cpu: c_int, that_cpu: c_int) -> bool;
}
extern "C" {
    pub fn cpus_share_resources(this_cpu: c_int, that_cpu: c_int) -> bool;
}
extern "C" {
    pub fn int(_arg: *mut sched_domain_flags_f)(void) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sd_data {
    pub sd: *mut *mut sched_domain __percpu,
    pub sg: *mut *mut sched_group __percpu,
    pub sgc: *mut *mut sched_group_capacity __percpu,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_domain_topology_level {
    pub mask: sched_domain_mask_f,
    pub sd_flags: sched_domain_flags_f,
    pub numa_level: c_int,
    pub data: sd_data,
    pub name: *mut c_char,
}

extern "C" {
    pub fn set_sched_topology(tl: *mut sched_domain_topology_level) -> void __init;
}
extern "C" {
    pub fn sched_update_asym_prefer_cpu(cpu: c_int, old_prio: c_int, new_prio: c_int);
}

extern "C" {
    pub fn rebuild_sched_domains_energy();
}

//
// arch_scale_cpu_capacity - get the capacity scale factor of a given CPU.
// @cpu: the CPU in question.
//
// Return: the CPU scale factor normalized against SCHED_CAPACITY_SCALE, i.e.
//
// max_perf(cpu)
// ----------------------------- * SCHED_CAPACITY_SCALE
// max(max_perf(c) : c \in CPUs)
//

extern "C" {
    pub fn cpu_to_node(_arg: task_cpu(p)) -> return;
}

extern "C" {
    pub fn sched_update_llc_bytes(cpu: c_uint);
}

