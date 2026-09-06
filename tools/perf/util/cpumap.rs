//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/cpumap.h
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

// Identify where counts are aggregated, -1 implies not to aggregate.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aggr_cpu_id {
// A value in the range 0 to number of threads.
    pub thread_idx: c_int,
// The numa node X as read from /sys/devices/system/node/nodeX.
    pub node: c_int,
//
// The socket number as read from
// /sys/devices/system/cpu/cpuX/topology/physical_package_id.
//
    pub socket: c_int,
// The die id as read from /sys/devices/system/cpu/cpuX/topology/die_id.
    pub die: c_int,
// The cluster id as read from /sys/devices/system/cpu/cpuX/topology/cluster_id
    pub cluster: c_int,
// The cache level as read from /sys/devices/system/cpu/cpuX/cache/indexY/level
    pub cache_lvl: c_int,
//
// The cache instance ID, which is the first CPU in the
// /sys/devices/system/cpu/cpuX/cache/indexY/shared_cpu_list
//
    pub cache: c_int,
// The core id as read from /sys/devices/system/cpu/cpuX/topology/core_id.
    pub core: c_int,
// CPU aggregation, note there is one CPU for each SMT thread.
    pub cpu: perf_cpu,
}

// A collection of aggr_cpu_id values, the "built" version is sorted and uniqued.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_aggr_map {
// Number of valid entries.
    pub nr: c_int,
// The entries.
    pub map: [aggr_cpu_id; ],
}

extern "C" {
    pub fn perf_record_cpu_map_data__test_bit(i: c_int, data: *const perf_record_cpu_map_data) -> bool;
}
extern "C" {
    pub fn cpu_map__snprint(map: *mut perf_cpu_map, buf: *mut c_char, size: usize) -> usize;
}
extern "C" {
    pub fn cpu_map__snprint_mask(map: *mut perf_cpu_map, buf: *mut c_char, size: usize) -> usize;
}
extern "C" {
    pub fn cpu_map__fprintf(map: *mut perf_cpu_map, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn cpu__setup_cpunode_map() -> c_int;
}
extern "C" {
    pub fn cpu__max_node() -> c_int;
}
extern "C" {
    pub fn cpu__max_cpu() -> perf_cpu;
}
extern "C" {
    pub fn cpu__max_present_cpu() -> perf_cpu;
}
//
// cpu_map__is_dummy - Events associated with a pid, rather than a CPU, use a single dummy map with an entry of -1.
//
// cpu__get_node - Returns the numa node X as read from
// /sys/devices/system/node/nodeX for the given CPU.
//
extern "C" {
    pub fn cpu__get_node(cpu: perf_cpu) -> c_int;
}
//
// cpu__get_socket_id - Returns the socket number as read from
// /sys/devices/system/cpu/cpuX/topology/physical_package_id for the given CPU.
//
extern "C" {
    pub fn cpu__get_socket_id(cpu: perf_cpu) -> c_int;
}
//
// cpu__get_die_id - Returns the die id as read from
// /sys/devices/system/cpu/cpuX/topology/die_id for the given CPU.
//
extern "C" {
    pub fn cpu__get_die_id(cpu: perf_cpu) -> c_int;
}
//
// cpu__get_cluster_id - Returns the cluster id as read from
// /sys/devices/system/cpu/cpuX/topology/cluster_id for the given CPU
//
extern "C" {
    pub fn cpu__get_cluster_id(cpu: perf_cpu) -> c_int;
}
//
// cpu__get_core_id - Returns the core id as read from
// /sys/devices/system/cpu/cpuX/topology/core_id for the given CPU.
//
extern "C" {
    pub fn cpu__get_core_id(cpu: perf_cpu) -> c_int;
}
//
// cpu_aggr_map__empty_new - Create a cpu_aggr_map of size nr with every entry
// being empty.
//
extern "C" {
    pub fn aggr_cpu_id(cpu: *mut *mut aggr_cpu_id_get_t)(struct perf_cpu, data: *mut c_void) -> typedef struct;
}
//
// cpu_aggr_map__new - Create a cpu_aggr_map with an aggr_cpu_id for each cpu in
// cpus. The aggr_cpu_id is created with 'get_id' that may have a data value
// passed to it. The cpu_aggr_map is sorted with duplicate values removed.
//
extern "C" {
    pub fn aggr_cpu_id__equal(a: *const aggr_cpu_id, b: *const aggr_cpu_id) -> bool;
}
extern "C" {
    pub fn aggr_cpu_id__is_empty(a: *const aggr_cpu_id) -> bool;
}
extern "C" {
    pub fn aggr_cpu_id__empty() -> aggr_cpu_id;
}
//
// aggr_cpu_id__socket - Create an aggr_cpu_id with the socket populated with
// the socket for cpu. The function signature is compatible with
// aggr_cpu_id_get_t.
//
extern "C" {
    pub fn aggr_cpu_id__socket(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id;
}
//
// aggr_cpu_id__die - Create an aggr_cpu_id with the die and socket populated
// with the die and socket for cpu. The function signature is compatible with
// aggr_cpu_id_get_t.
//
extern "C" {
    pub fn aggr_cpu_id__die(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id;
}
//
// aggr_cpu_id__cluster - Create an aggr_cpu_id with cluster, die and socket
// populated with the cluster, die and socket for cpu. The function signature
// is compatible with aggr_cpu_id_get_t.
//
extern "C" {
    pub fn aggr_cpu_id__cluster(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id;
}
//
// aggr_cpu_id__core - Create an aggr_cpu_id with the core, cluster, die and
// socket populated with the core, die and socket for cpu. The function
// signature is compatible with aggr_cpu_id_get_t.
//
extern "C" {
    pub fn aggr_cpu_id__core(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id;
}
//
// aggr_cpu_id__core - Create an aggr_cpu_id with the cpu, core, die and socket
// populated with the cpu, core, die and socket for cpu. The function signature
// is compatible with aggr_cpu_id_get_t.
//
extern "C" {
    pub fn aggr_cpu_id__cpu(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id;
}
//
// aggr_cpu_id__node - Create an aggr_cpu_id with the numa node populated for
// cpu. The function signature is compatible with aggr_cpu_id_get_t.
//
extern "C" {
    pub fn aggr_cpu_id__node(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id;
}
//
// aggr_cpu_id__global - Create an aggr_cpu_id for global aggregation.
// The function signature is compatible with aggr_cpu_id_get_t.
//
extern "C" {
    pub fn aggr_cpu_id__global(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id;
}
