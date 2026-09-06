//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/cpuid/types.h
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
// Types for raw CPUID access:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuid_regs {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpuid_regs_idx {
    CPUID_EAX = 0,
    CPUID_EBX,
    CPUID_ECX,
    CPUID_EDX,
}

pub const CPUID_LEAF_MWAIT: c_uint = 0x05;
pub const CPUID_LEAF_DCA: c_uint = 0x09;
pub const CPUID_LEAF_XSTATE: c_uint = 0x0d;
pub const CPUID_LEAF_TSC: c_uint = 0x15;
pub const CPUID_LEAF_FREQ: c_uint = 0x16;
pub const CPUID_LEAF_TILE: c_uint = 0x1d;

pub const CPUID_BASE_START: c_uint = 0x00000000;

//
// Types for CPUID(0x2) parsing:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x2_reg {
    pub 1: invalid :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union leaf_0x2_regs {
    pub reg: [leaf_0x2_reg; 4],
    pub regv: [u32; 4],
    pub desc: [u8; 16],
}

//
// Leaf 0x2 1-byte descriptors' cache types
// To be used for their mappings at cpuid_0x2_table[]
//
// Start at 1 since type 0 is reserved for HW byte descriptors which are
// not recognized by the kernel; i.e., those without an explicit mapping.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum _cache_table_type {
    CACHE_L1_INST		= 1,
    CACHE_L1_DATA,
    CACHE_L2,
    CACHE_L3
// Adjust __TLB_TABLE_TYPE_BEGIN before adding more types
    } __packed;
    static_assert(sizeof(enum _cache_table_type) == 1);

//
// Ensure that leaf 0x2 cache and TLB type values do not intersect,
// since they share the same type field at struct cpuid_0x2_table.
//

//
// Leaf 0x2 1-byte descriptors' TLB types
// To be used for their mappings at cpuid_0x2_table[]
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum _tlb_table_type {
    TLB_INST_4K		= __TLB_TABLE_TYPE_BEGIN,
    TLB_INST_4M,
    TLB_INST_2M_4M,
    TLB_INST_ALL,

    TLB_DATA_4K,
    TLB_DATA_4M,
    TLB_DATA_2M_4M,
    TLB_DATA_4K_4M,
    TLB_DATA_1G,
    TLB_DATA_1G_2M_4M,

    TLB_DATA0_4K,
    TLB_DATA0_4M,
    TLB_DATA0_2M_4M,

    STLB_4K,
    STLB_4K_2M,
    } __packed;
    static_assert(sizeof(enum _tlb_table_type) == 1);

//
// Combined parsing table for leaf 0x2 cache and TLB descriptors.
//

    struct leaf_0x2_table {
    union {
    enum _cache_table_type	c_type;
    enum _tlb_table_type	t_type;
}

//
// All of leaf 0x2's one-byte TLB descriptors implies the same number of entries
// for their respective TLB types.  TLB descriptor 0x63 is an exception: it
// implies 4 dTLB entries for 1GB pages and 32 dTLB entries for 2MB or 4MB pages.
//
// Encode that descriptor's dTLB entry count for 2MB/4MB pages here, as the entry
// count for dTLB 1GB pages is already encoded at the cpuid_0x2_table[]'s mapping.
//
pub const TLB_0x63_2M_4M_ENTRIES: c_int = 32;
//
// Types for centralized CPUID tables:
//
// For internal use by the CPUID parser.
//
// struct leaf_parse_info - CPUID query parse info
// @nr_entries:	Number of valid entries filled by the CPUID parser
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_parse_info {
    pub nr_entries: c_uint,
}

//
// __CPUID_LEAF() - Define a CPUID output and parse info entry
// @_name:	Struct type name of the CPUID leaf/subleaf (e.g. 'leaf_0x7_0'). Such
// types are defined at <cpuid/leaf_types.h> and follow the leaf_0xM_N
// format, where 0xM is the leaf and N is the subleaf.
// @_count:	Number of storage entries to allocate for this leaf/subleaf.
//
// For a given leaf/subleaf, define an array of CPUID storage entries and an associated
// query info structure.
//
// Use an array of storage entries to accommodate CPUID leaves with multiple subleaves
// having the same output format.  This is common for hierarchical enumeration; e.g.,
// CPUID(0x4), CPUID(0x12), and CPUID(0x8000001d).
//

//
// CPUID_LEAF() - Define a 'struct cpuid_leaves' storage entry
// @_leaf:	Leaf number, in compile-time 0xN format
// @_subleaf:	Subleaf number, in compile-time decimal format
//
// Convenience wrapper around __CPUID_LEAF().
//

pub const __cpuid_leaf_subleaf_count_min(_l): c_int = 2;

//
// CPUID_LEAF_N() - Define a 'struct cpuid_leaves' storage entry
// @_leaf:	Leaf number, in compile-time 0xN format
// @_count:	Number of storage entries to allocate for that leaf. It must not exceed
// the limits defined at <cpuid/leaf_types.h>.
//
// Convenience wrapper around __CPUID_LEAF().
//

//
// struct cpuid_leaves - Parsed CPUID data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuid_leaves {
// Leaf		Subleaf number (or max number of subleaves)
    pub ): CPUID_LEAF ( 0x0, 0,
    pub ): CPUID_LEAF ( 0x1, 0,
}

//
// Types for centralized CPUID tables:
//
// For external use.
//
// struct cpuid_table - Per-CPU CPUID data repository
// @leaves:	Parsed CPUID queries output and their metadata
//
// This is to be embedded inside 'struct cpuinfo_x86' to provide parsed and
// sanitized CPUID data per CPU.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuid_table {
    pub leaves: cpuid_leaves,
}
