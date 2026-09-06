//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/mem-events.h
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
pub struct perf_mem_event {
    pub supported: bool,
    pub ldlat: bool,
    pub aux_event: u32,
    pub tag: *const c_char,
    pub name: *const c_char,
    pub event_name: *const c_char,
}

extern "C" {
    pub fn perf_pmu__mem_events_parse(pmu: *mut perf_pmu, str: *const c_char) -> c_int;
}
extern "C" {
    pub fn perf_pmu__mem_events_init() -> c_int;
}
extern "C" {
    pub fn perf_pmu__mem_events_num_mem_pmus(pmu: *mut perf_pmu) -> c_int;
}
extern "C" {
    pub fn is_mem_loads_aux_event(leader: *mut evsel) -> bool;
}
extern "C" {
    pub fn perf_pmu__mem_events_list(pmu: *mut perf_pmu);
}
extern "C" {
    pub fn perf_mem__tlb_scnprintf(out: *mut c_char, sz: usize, mem_info: *const mem_info) -> c_int;
}
extern "C" {
    pub fn perf_mem__lvl_scnprintf(out: *mut c_char, sz: usize, mem_info: *const mem_info) -> c_int;
}
extern "C" {
    pub fn perf_mem__snp_scnprintf(out: *mut c_char, sz: usize, mem_info: *const mem_info) -> c_int;
}
extern "C" {
    pub fn perf_mem__lck_scnprintf(out: *mut c_char, sz: usize, mem_info: *const mem_info) -> c_int;
}
extern "C" {
    pub fn perf_mem__blk_scnprintf(out: *mut c_char, sz: usize, mem_info: *const mem_info) -> c_int;
}
extern "C" {
    pub fn perf_script__meminfo_scnprintf(bf: *mut c_char, size: usize, mem_info: *const mem_info) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c2c_stats {
    pub nr_entries: u32,
    pub /: *mut *mut u32 locks; / count of 'lock' transactions,
    pub /: *mut *mut u32 store; / count of all stores in trace,
    pub /: *mut *mut u32 st_uncache; / stores to uncacheable address,
    pub /: *mut *mut u32 st_noadrs; / cacheable store with no address,
    pub /: *mut *mut u32 st_l1hit; / count of stores that hit L1D,
    pub /: *mut *mut u32 st_l1miss; / count of stores that miss L1D,
    pub /: *mut *mut u32 st_na; / count of stores with memory level is not available,
    pub /: *mut *mut u32 load; / count of all loads in trace,
    pub /: *mut *mut u32 ld_excl; / exclusive loads, rmt/lcl DRAM - snp none/miss,
    pub /: *mut *mut u32 ld_shared; / shared loads, rmt/lcl DRAM - snp hit,
    pub /: *mut *mut u32 ld_uncache; / loads to uncacheable address,
    pub /: *mut *mut u32 ld_io; / loads to io address,
    pub /: *mut *mut u32 ld_miss; / loads miss,
    pub /: *mut *mut u32 ld_noadrs; / cacheable load with no address,
    pub /: *mut *mut u32 ld_fbhit; / count of loads hitting Fill Buffer,
    pub /: *mut *mut u32 ld_l1hit; / count of loads that hit L1D,
    pub /: *mut *mut u32 ld_l2hit; / count of loads that hit L2D,
    pub /: *mut *mut u32 ld_llchit; / count of loads that hit LLC,
    pub /: *mut *mut u32 lcl_hitm; / count of loads with local HITM,
    pub /: *mut *mut u32 rmt_hitm; / count of loads with remote HITM,
    pub /: *mut *mut u32 tot_hitm; / count of loads with local and remote HITM,
    pub /: *mut *mut u32 lcl_peer; / count of loads with local peer cache,
    pub /: *mut *mut u32 rmt_peer; / count of loads with remote peer cache,
    pub /: *mut *mut u32 tot_peer; / count of loads with local and remote peer cache,
    pub /: *mut *mut u32 rmt_hit; / count of loads with remote hit clean;,
    pub /: *mut *mut u32 lcl_dram; / count of loads miss to local DRAM,
    pub /: *mut *mut u32 rmt_dram; / count of loads miss to remote DRAM,
    pub /: *mut *mut u32 blk_data; / count of loads blocked by data,
    pub /: *mut *mut u32 blk_addr; / count of loads blocked by address conflict,
    pub /: *mut *mut u32 nomap; / count of load/stores with no phys addrs,
    pub /: *mut *mut u32 noparse; / count of unparsable data sources,
}

extern "C" {
    pub fn c2c_decode_stats(stats: *mut c2c_stats, mi: *mut mem_info) -> c_int;
}
extern "C" {
    pub fn c2c_add_stats(stats: *mut c2c_stats, add: *mut c2c_stats);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mem_stat_type {
    PERF_MEM_STAT_OP,
    PERF_MEM_STAT_CACHE,
    PERF_MEM_STAT_MEMORY,
    PERF_MEM_STAT_SNOOP,
    PERF_MEM_STAT_DTLB,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mem_stat_op {
    MEM_STAT_OP_LOAD,
    MEM_STAT_OP_STORE,
    MEM_STAT_OP_LDST,
    MEM_STAT_OP_PFETCH,
    MEM_STAT_OP_EXEC,
    MEM_STAT_OP_OTHER,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mem_stat_cache {
    MEM_STAT_CACHE_L1,
    MEM_STAT_CACHE_L2,
    MEM_STAT_CACHE_L3,
    MEM_STAT_CACHE_L4,
    MEM_STAT_CACHE_L1_BUF,
    MEM_STAT_CACHE_L2_BUF,
    MEM_STAT_CACHE_OTHER,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mem_stat_memory {
    MEM_STAT_MEMORY_RAM,
    MEM_STAT_MEMORY_MSC,
    MEM_STAT_MEMORY_UNC,
    MEM_STAT_MEMORY_CXL,
    MEM_STAT_MEMORY_IO,
    MEM_STAT_MEMORY_PMEM,
    MEM_STAT_MEMORY_OTHER,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mem_stat_snoop {
    MEM_STAT_SNOOP_HIT,
    MEM_STAT_SNOOP_HITM,
    MEM_STAT_SNOOP_MISS,
    MEM_STAT_SNOOP_OTHER,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mem_stat_dtlb {
    MEM_STAT_DTLB_L1_HIT,
    MEM_STAT_DTLB_L2_HIT,
    MEM_STAT_DTLB_ANY_HIT,
    MEM_STAT_DTLB_MISS,
    MEM_STAT_DTLB_OTHER,
}

extern "C" {
    pub fn mem_stat_index(mst: mem_stat_type, data_src: u64) -> c_int;
}
