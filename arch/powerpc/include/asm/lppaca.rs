//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/lppaca.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// lppaca.h
// Copyright (C) 2001  Mike Corrigan IBM Corporation
//

//
// These definitions relate to hypervisors that only exist when using
// a server type processor
//

//
// This control block contains the data that is shared between the
// hypervisor and the OS.
//

//
// The lppaca is the "virtual processor area" registered with the hypervisor,
// H_REGISTER_VPA etc.
//
// According to PAPR, the structure is 640 bytes long, must be L1 cache line
// aligned, and must not cross a 4kB boundary. Its size field must be at
// least 640 bytes (but may be more).
//
// Pre-v4.14 KVM hypervisors reject the VPA if its size field is smaller than
// 1kB, so we dynamically allocate 1kB and advertise size as 1kB, but keep
// this structure as the canonical 640 byte size.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lppaca {
// cacheline 1 contains read-only data
    pub /: *mut *mut __be32 desc; / Eye catcher 0xD397D781,
    pub /: *mut *mut __be16 size; / Size of this struct,
    pub reserved1: [u8; 3],
    pub /: *mut *mut u8 __old_status; / Old status, including shared proc,
    pub reserved3: [u8; 14],
    pub /: *mut *mut volatile __be32 dyn_hw_node_id; / Dynamic hardware node id,
    pub /: *mut *mut volatile __be32 dyn_hw_proc_id; / Dynamic hardware proc id,
    pub reserved4: [u8; 56],
    pub /: *mut *mut volatile u8 vphn_assoc_counts[8]; / Virtual processor home node,
// associativity change counters
    pub reserved5: [u8; 32],
// cacheline 2 contains local read-write data
    pub reserved6: [u8; 48],
    pub cede_latency_hint: u8,
    pub ebb_regs_in_use: u8,
    pub reserved7: [u8; 6],
    pub /: *mut *mut u8 dtl_enable_mask; / Dispatch Trace Log mask,
    pub /: *mut *mut u8 donate_dedicated_cpu; / Donate dedicated CPU cycles,
    pub fpregs_in_use: u8,
    pub pmcregs_in_use: u8,
    pub /: *mut *mut u8 l2_counters_enable; / Enable usage of counters for KVM guest,
    pub reserved8: [u8; 27],
    pub /: *mut *mut __be64 wait_state_cycles; / Wait cycles for this proc,
    pub reserved9: [u8; 28],
    pub /: *mut *mut __be16 slb_count; / # of SLBs to maintain,
    pub /: *mut *mut u8 idle; / Indicate OS is idle,
    pub vmxregs_in_use: u8,
// cacheline 3 is shared with other processors
//
// This is the yield_count.  An "odd" value (low bit on) means that
// the processor is yielded (either because of an OS yield or a
// hypervisor preempt).  An even value implies that the processor is
// currently executing.
// NOTE: Even dedicated processor partitions can yield so this
// field cannot be used to determine if we are shared or dedicated.
//
    pub yield_count: volatile __be32,
    pub /: *mut *mut volatile __be32 dispersion_count; / dispatch changed physical cpu,
    pub /: *mut *mut volatile __be64 cmo_faults; / CMO page fault count,
    pub /: *mut *mut volatile __be64 cmo_fault_time; / CMO page fault time,
    pub /: *mut *mut u8 reserved10[64]; / [S]PURR expropriated/donated,
    pub /: *mut *mut volatile __be64 enqueue_dispatch_tb; / Total TB enqueue->dispatch,
    pub /: *mut *mut volatile __be64 ready_enqueue_tb; / Total TB ready->enqueue,
    pub /: *mut *mut volatile __be64 wait_ready_tb; / Total TB wait->ready,
    pub reserved11: [u8; 16],
// cacheline 4-5
    pub /: *mut *mut __be32 page_ins; / CMO Hint - # page ins by OS,
    pub reserved12: [u8; 28],
    pub l1_to_l2_cs_tb: volatile __be64,
    pub l2_to_l1_cs_tb: volatile __be64,
    pub l2_runtime_tb: volatile __be64,
    pub reserved13: [u8; 96],
    pub /: *mut *mut volatile __be64 dtl_idx; / Dispatch Trace Log head index,
    pub reserved14: [u8; 96],
    pub ____cacheline_aligned: },

//
// We are using a non architected field to determine if a partition is
// shared or dedicated. This currently works on both KVM and PHYP, but
// we will have to transition to something better.
//
pub const LPPACA_OLD_SHARED_PROC: c_int = 2;

//
// All CPUs should have the same shared proc value, so directly access the PACA
// to avoid false positives from DEBUG_PREEMPT.
//
    pub local_paca->lppaca_ptr: *mut *mut lppaca l =,
    pub false: return,
    pub LPPACA_OLD_SHARED_PROC): return !!(l->__old_status &,

//
// SLB shadow buffer structure as defined in the PAPR.  The save_area
// contains adjacent ESID and VSID pairs for each shadowed SLB.  The
// ESID is stored in the lower 64bits, then the VSID.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slb_shadow {
    pub /: *mut *mut __be32 persistent; / Number of persistent SLBs,
    pub /: *mut *mut __be32 buffer_length; / Total shadow buffer length,
    pub reserved: __be64,
    pub esid: __be64,
    pub vsid: __be64,
    pub save_area: [}; SLB_NUM_BOLTED],
    pub ____cacheline_aligned: },

