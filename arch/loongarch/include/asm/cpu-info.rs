//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/cpu-info.h
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
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

// cache_desc->flags
//
// Descriptor for a cache
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cache_desc {
    pub type: c_uchar,
    pub level: c_uchar,
    pub /: *mut *mut unsigned short sets; / Number of lines per set,
    pub /: *mut *mut unsigned char ways; / Number of ways,
    pub /: *mut *mut unsigned char linesz; / Size of line in bytes,
    pub /: *mut *mut unsigned char flags; / Flags describing cache properties,
}

pub const CACHE_LEVEL_MAX: c_int = 3;
pub const CACHE_LEAVES_MAX: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuinfo_loongarch {
    pub asid_cache: u64,
    pub asid_mask: c_ulong,
//
// Capability and feature descriptor structure for LoongArch CPU
//
    pub options: c_ulonglong,
    pub processor_id: c_uint,
    pub fpu_vers: c_uint,
    pub fpu_csr0: c_uint,
    pub fpu_mask: c_uint,
    pub cputype: c_uint,
    pub isa_level: c_int,
    pub tlbsize: c_int,
    pub tlbsizemtlb: c_int,
    pub tlbsizestlbsets: c_int,
    pub tlbsizestlbways: c_int,
    pub /: *mut *mut int cache_leaves_present; / number of cache_leaves[] elements,
    pub cache_leaves: [cache_desc; CACHE_LEAVES_MAX],
    pub /: *mut *mut int core; / physical core number in package,
    pub /: *mut *mut int package;/ physical package number,
    pub /: *mut *mut int global_id; / physical global thread number,
    pub /: *mut *mut int vabits; / Virtual Address size in bits,
    pub /: *mut *mut int pabits; / Physical Address size in bits,
    pub /: *mut *mut int timerbits; / Width of arch timer in bits,
    pub /: *mut *mut unsigned int ksave_mask; / Usable KSave mask.,
    pub /: *mut *mut unsigned int watch_dreg_count; / Number data breakpoints,
    pub /: *mut *mut unsigned int watch_ireg_count; / Number instruction breakpoints,
    pub /: *mut *mut unsigned int watch_reg_use_cnt; / min(NUM_WATCH_REGS, watch_dreg_count + watch_ireg_count), Usable by ptrace,
    pub __aligned(SMP_CACHE_BYTES): },
    pub cpu_data: [extern struct cpuinfo_loongarch; ],
    pub cpu_probe(void): extern void,
    pub __cpu_family: [*const extern char; ],
    pub __cpu_full_name: [*const extern char; ],
    pub &cpu_data[cpua]: *mut *mut cpuinfo_loongarch infoa =,
    pub &cpu_data[cpub]: *mut *mut cpuinfo_loongarch infob =,
    pub false: return,
    pub false: return,
    pub true: return,
    pub cpuinfo->asid_mask: return,
    pub asid_mask: cpuinfo->asid_mask =,
