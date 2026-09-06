//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/intel-family.h
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
// "Big Core" Processors (Branded as Core, Xeon, etc...)
//
// While adding a new CPUID for a new microarchitecture, add a new
// group to keep logically sorted out in chronological order. Within
// that group keep the CPUID for the variants sorted by model number.
//
// The defined symbol names have the following form:
// INTEL_{OPTFAMILY}_{MICROARCH}{OPTDIFF}
// where:
// OPTFAMILY	Describes the family of CPUs that this belongs to. Default
// is assumed to be "_CORE" (and should be omitted). Other values
// currently in use are _ATOM and _XEON_PHI
// MICROARCH	Is the code name for the micro-architecture for this core.
// N.B. Not the platform name.
// OPTDIFF	If needed, a short string to differentiate by market segment.
//
// Common OPTDIFFs:
//
// - regular client parts
// _L	- regular mobile parts
// _G	- parts with extra graphics on
// _X	- regular server parts
// _D	- micro server parts
// _N,_P	- other mobile parts
// _H	- premium mobile parts
// _S	- other client parts
// _R	- ruggedized for harsh environment
//
// Historical OPTDIFFs:
//
// _EP	- 2 socket server parts
// _EX	- 4+ socket server parts
//
// The #define line may optionally include a comment including platform or core
// names. An exception is made for skylake/kabylake where steppings seem to have gotten
// their own names :-(
//

// Wildcard match so X86_MATCH_VFM(ANY) works

// Family 5

// Family 6, 18, 19

// CASCADELAKE_X	0x55	   Sky Lake -- s: 7
// COOPERLAKE_X		0x55	   Sky Lake -- s: 11

// AMBERLAKE_L		0x8E	   Sky Lake -- s: 9
// COFFEELAKE_L		0x8E	   Sky Lake -- s: 10
// WHISKEYLAKE_L	0x8E       Sky Lake -- s: 11,12

// COFFEELAKE		0x9E	   Sky Lake -- s: 10-13

// "Hybrid" Processors (P-Core/E-Core)

// "Small Core" Processors (Atom/E-Core)

// Note: the micro-architecture is "Goldmont Plus"

// Xeon Phi

// Notational marker denoting the last Family 6 model

// Family 15 - NetBurst

//
// Intel CPU core types
//
// CPUID.1AH.EAX[31:0] uniquely identifies the microarchitecture
// of the core. Bits 31-24 indicates its core type (Core or Atom)
// and Bits [23:0] indicates the native model ID of the core.
// Core type and native model ID are defined in below enumerations.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_cpu_type {
    INTEL_CPU_TYPE_UNKNOWN,
    INTEL_CPU_TYPE_ATOM = 0x20,
    INTEL_CPU_TYPE_CORE = 0x40,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_native_id {
    INTEL_ATOM_CMT_NATIVE_ID = 0x2,  /* Crestmont */
    INTEL_ATOM_SKT_NATIVE_ID = 0x3,  /* Skymont */
}
