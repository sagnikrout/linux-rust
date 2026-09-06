//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kernel/cpu/cpuid_parser.h
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
// Since accessing the CPUID leaves at 'struct cpuid_leaves' require compile time
// tokenization, split the CPUID parser into two stages: compile time macros for
// tokenizing the leaf/subleaf output offsets within the table, and generic runtime
// code to write to the relevant CPUID leaves using such offsets.
//
// The output of the compile time macros is cached by a compile time "parse entry"
// table (see 'struct cpuid_parse_entry').  The runtime parser code will utilize
// such offsets by passing them to the cpuid_table_*_p() functions.
//
// Compile time CPUID table offset calculations:
//
// @_leaf:	CPUID leaf, in 0xN format
// @_subleaf:	CPUID subleaf, in decimal format
//

//
// Translation of compile time offsets to generic runtime pointers:
//
// struct cpuid_output - Output of a CPUID operation
// @regs:	Pointer to an array of CPUID results, where each array element covers the
// full EAX->EDX output range.
// @info:	Pointer to query info; for saving the number of filled elements at @regs.
//
// A CPUID parser read function like cpuid_read_generic() or cpuid_read_0xN() uses this
// structure to save the CPUID query outputs.  Actual storage for @regs and @info is
// provided by the read function caller, and is typically within the CPU's CPUID table.
//
// See struct cpuid_parse_entry.read().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuid_output {
    pub regs: *mut cpuid_regs,
    pub info: *mut leaf_parse_info,
}

//
// struct cpuid_parse_entry - CPUID parse table entry
// @leaf:	Leaf number to be parsed
// @subleaf:	Subleaf number to be parsed
// @regs_offs:	Offset within 'struct cpuid_leaves' for saving the CPUID query output; to be
// passed to cpuid_table_regs_p().
// @info_offs:	Offset within 'struct cpuid_leaves' for saving the CPUID query parse info; to be
// passed to cpuid_table_info_p().
// @maxcnt:	Maximum number of output storage entries available for the CPUID query.
// @read:	Read function for this entry.  It must save the parsed CPUID output to the passed
// 'struct cpuid_output'->regs array of size >= @maxcnt.  It must set
// 'struct cpuid_output'->info.nr_entries to the number of CPUID output entries
// parsed and filled.  A generic implementation is provided at cpuid_read_generic().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuid_parse_entry {
    pub leaf: c_uint,
    pub subleaf: c_uint,
    pub regs_offs: c_uint,
    pub info_offs: c_uint,
    pub maxcnt: c_uint,
    pub o): *const *const *const void (read)(struct cpuid_parse_entry e, struct cpuid_output,
}

//
// CPUID_PARSE_ENTRY_N() is for parsing CPUID leaves with a subleaf range.
// Check <asm/cpuid/types.h> __CPUID_LEAF() vs. CPUID_LEAF_N().
//

//
// CPUID parser table:
//

// Leaf		Subleaf		Reader function */		\
