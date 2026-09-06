//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/env.h
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
pub struct cpu_topology_map {
    pub socket_id: c_int,
    pub die_id: c_int,
    pub cluster_id: c_int,
    pub core_id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_cache_level {
    pub level: u32,
    pub line_size: u32,
    pub sets: u32,
    pub ways: u32,
    pub type: *mut c_char,
    pub size: *mut c_char,
    pub map: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct numa_node {
    pub node: u32,
    pub mem_total: u64,
    pub mem_free: u64,
    pub map: *mut perf_cpu_map,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct memory_node {
    pub node: u64,
    pub size: u64,
    pub set: *mut c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hybrid_node {
    pub pmu_name: *mut c_char,
    pub cpus: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmu_caps {
    pub nr_caps: c_int,
    pub max_branches: c_uint,
    pub br_cntr_nr: c_uint,
    pub br_cntr_width: c_uint,
    pub caps: *mut c_char,
    pub pmu_name: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct domain_info {
    pub domain: u32,
    pub dname: *mut c_char,
    pub cpumask: *mut c_char,
    pub cpulist: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_domain_map {
    pub cpu: u32,
    pub nr_domains: u32,
    pub domains: *mut domain_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_env {
    pub hostname: *mut c_char,
    pub os_release: *mut c_char,
    pub version: *mut c_char,
    pub arch: *mut c_char,
// e_machine expanded from 16 to 32-bits for alignment.
    pub e_machine: u32,
    pub e_flags: u32,
    pub nr_cpus_online: c_int,
    pub nr_cpus_avail: c_int,
    pub cpu_desc: *mut c_char,
    pub cpuid: *mut c_char,
    pub total_mem: c_ulonglong,
    pub msr_pmu_type: c_uint,
    pub max_branches: c_uint,
    pub br_cntr_nr: c_uint,
    pub br_cntr_width: c_uint,
    pub schedstat_version: c_uint,
    pub max_sched_domains: c_uint,
    pub kernel_is_64_bit: c_int,
    pub nr_cmdline: c_int,
    pub nr_sibling_cores: c_int,
    pub nr_sibling_dies: c_int,
    pub nr_sibling_threads: c_int,
    pub nr_numa_nodes: c_int,
    pub nr_memory_nodes: c_int,
    pub nr_pmu_mappings: c_int,
    pub nr_groups: c_int,
    pub nr_cpu_pmu_caps: c_int,
    pub nr_hybrid_nodes: c_int,
    pub nr_pmus_with_caps: c_int,
    pub cmdline: *mut c_char,
    pub cmdline_argv: *const c_char,
    pub sibling_cores: *mut c_char,
    pub sibling_dies: *mut c_char,
    pub sibling_threads: *mut c_char,
    pub pmu_mappings: *mut c_char,
    pub cpu_pmu_caps: *mut c_char,
    pub cpu: *mut cpu_topology_map,
    pub caches: *mut cpu_cache_level,
    pub cpu_domain: *mut cpu_domain_map,
    pub caches_cnt: c_int,
    pub cln_size: c_uint,
    pub comp_ratio: u32,
    pub comp_ver: u32,
    pub comp_type: u32,
    pub comp_level: u32,
    pub comp_mmap_len: u32,
    pub numa_nodes: *mut numa_node,
    pub memory_nodes: *mut memory_node,
    pub memory_bsize: c_ulonglong,
    pub hybrid_nodes: *mut hybrid_node,
    pub pmu_caps: *mut pmu_caps,

//
// bpf_info_lock protects bpf rbtrees. This is needed because the
// trees are accessed by different threads in perf-top
//
    pub lock: rw_semaphore,
    pub infos: rb_root,
    pub infos_cnt: u32,
    pub btfs: rb_root,
    pub btfs_cnt: u32,
    pub bpf_progs: },

// same reason as above (for perf-top)
    pub lock: rw_semaphore,
    pub tree: rb_root,
    pub cgroups: },
// For fast cpu to numa node lookup via perf_env__numa_node
    pub numa_map: *mut c_int,
    pub nr_numa_map: c_int,
// For real clock time reference.
    pub tod_ns: u64,
    pub clockid_ns: u64,
    pub clockid_res_ns: u64,
    pub clockid: c_int,
//
// enabled is valid for report mode, and is true if above
// values are set, it's set in process_clock_data
//
    pub enabled: bool,
    pub clock: },
// Protects lazy environment initialization (e.g. os_release, e_machine).
    pub lock: mutex,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_compress_type {
    PERF_COMP_NONE = 0,
    PERF_COMP_ZSTD,
    PERF_COMP_MAX
}

extern "C" {
    pub fn perf_env__read_core_pmu_caps(env: *mut perf_env) -> c_int;
}
extern "C" {
    pub fn free_cpu_domain_info(cd_map: *mut cpu_domain_map, schedstat_version: u32, nr: u32);
}
extern "C" {
    pub fn perf_env__exit(env: *mut perf_env);
}
extern "C" {
    pub fn perf_env__kernel_is_64_bit(env: *mut perf_env) -> c_int;
}
extern "C" {
    pub fn perf_arch_is_big_endian(arch: *const c_char) -> bool;
}
extern "C" {
    pub fn perf_env__set_cmdline(env: *mut perf_env, argc: c_int, argv[]: *const c_char) -> c_int;
}
extern "C" {
    pub fn perf_env__read_cpuid(env: *mut perf_env) -> c_int;
}
extern "C" {
    pub fn perf_env__read_pmu_mappings(env: *mut perf_env) -> c_int;
}
extern "C" {
    pub fn perf_env__nr_pmu_mappings(env: *mut perf_env) -> c_int;
}
extern "C" {
    pub fn perf_env__read_cpu_topology_map(env: *mut perf_env) -> c_int;
}
//
// Safe accessor for env->cpu[] topology array.  env->cpu can be NULL when
// reading old-format perf.data that predates topology information —
// process_cpu_topology() in header.c frees it while nr_cpus_avail remains
// set, so callers must not index env->cpu[] without this check.
//
extern "C" {
    pub fn cpu_cache_level__free(cache: *mut cpu_cache_level);
}
extern "C" {
    pub fn perf_env__e_machine_nocache(env: *mut perf_env, e_flags: *mut u32) -> u16;
}
extern "C" {
    pub fn perf_env__e_machine(env: *mut perf_env, e_flags: *mut u32) -> u16;
}
extern "C" {
    pub fn perf_env__nr_cpus_avail(env: *mut perf_env) -> c_int;
}
extern "C" {
    pub fn perf_env__init(env: *mut perf_env);
}

extern "C" {
    pub fn perf_env__insert_btf(env: *mut perf_env, btf_node: *mut btf_node) -> bool;
}
extern "C" {
    pub fn __perf_env__insert_btf(env: *mut perf_env, btf_node: *mut btf_node) -> bool;
}

extern "C" {
    pub fn perf_env__numa_node(env: *mut perf_env, cpu: perf_cpu) -> c_int;
}
extern "C" {
    pub fn perf_env__has_pmu_mapping(env: *mut perf_env, pmu_name: *const c_char) -> bool;
}
extern "C" {
    pub fn x86__is_amd_cpu() -> bool;
}
extern "C" {
    pub fn perf_env__is_x86_amd_cpu(env: *mut perf_env) -> bool;
}
extern "C" {
    pub fn x86__is_intel_cpu() -> bool;
}
extern "C" {
    pub fn perf_env__is_x86_intel_cpu(env: *mut perf_env) -> bool;
}
