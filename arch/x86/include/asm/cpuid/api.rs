//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/cpuid/api.h
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
// Raw CPUID accessors:
//

extern "C" {
    pub fn cpuid_feature() -> bool;
}

// ecx is often an input as well as an output.

//
// Native CPUID functions returning a single datum:
//

//
// Generic CPUID function
//
// Clear ECX since some CPUs (Cyrix MII) do not set or clear ECX
// resulting in stale register contents being returned.
//
// eax = op;
// ecx = 0;
// Some CPUID calls want 'count' to be placed in ECX
// eax = op;
// ecx = count;
//
// CPUID functions returning a single datum:
//

// reg = regs[regidx];

//
// Hypervisor-related APIs:
//

//
// This must not compile to "call memcmp" because it's called
// from PVH early boot code before instrumentation is set up
// and memcmp() itself may be instrumented.
//
// CPUID(0x2) parsing:
//
// cpuid_leaf_0x2() - Return sanitized CPUID(0x2) register output
// @regs:	Output parameter
//
// Query CPUID(0x2) and store its output in @regs.  Force set any
// invalid 1-byte descriptor returned by the hardware to zero (the NULL
// cache/TLB descriptor) before returning it to the caller.
//
// Use for_each_cpuid_0x2_desc() to iterate over the register output in
// parsed form.
//
// All Intel CPUs must report an iteration count of 1.	In case
// of bogus hardware, treat all returned descriptors as NULL.
//
// The most significant bit (MSB) of each register must be clear.
// If a register is invalid, replace its descriptors with NULL.
//
// for_each_cpuid_0x2_desc() - Iterator for parsed CPUID(0x2) descriptors
// @_regs:	CPUID(0x2) register output, as returned by cpuid_leaf_0x2()
// @_ptr:	u8 pointer, for macro internal use only
// @_desc:	Pointer to the parsed CPUID(0x2) descriptor at each iteration
//
// Loop over the 1-byte descriptors in the passed CPUID(0x2) output registers
// @_regs.  Provide the parsed information for each descriptor through @_desc.
//
// To handle cache-specific descriptors, switch on @_desc->c_type.  For TLB
// descriptors, switch on @_desc->t_type.
//
// Example usage for cache descriptors::
//
// const struct leaf_0x2_table *desc;
// union leaf_0x2_regs regs;
// u8 *ptr;
//
// cpuid_leaf_0x2(&regs);
// for_each_cpuid_0x2_desc(regs, ptr, desc) {
// switch (desc->c_type) {
// ...
// }
//

//
// CPUID(0x80000006) parsing:
//
extern "C" {
    pub fn cpuid_edx(_arg: 0x80000006) -> return;
}
//
// 'struct cpuid_leaves' accessors (without sanity checks):
//
// For internal use by the CPUID parser.
//
// Return constified pointers for all call-site APIs

//
// 'struct cpuid_table' accessors (with sanity checks):
//
// For internal use by the CPUID parser.
//

// Return NULL if the parser did not fill that leaf.  Check cpuid_subleaf().

//
// Return NULL if the CPUID parser did not fill this leaf, or if the given
// dynamic subleaf value is out of range.  Check cpuid_subleaf_n().
//

// CPUID parser might not have filled the entire subleaf range */			\
//
// Compile-time checks for leaves with a subleaf range:
//

//
// CPUID Parser Call-site APIs
//
// Call sites should use below APIs instead of invoking direct CPUID queries.
//
// Benefits include:
//
// - Return CPUID output as typed C structures that are auto-generated from a
// centralized database (see <asm/cpuid/leaf_types.h).  Such data types have a
// full C99 bitfield layout per CPUID leaf/subleaf combination.  Call sites
// can thus avoid doing ugly and cryptic bitwise operations on raw CPUID data.
//
// - Return cached, per-CPU, CPUID output.  Below APIs do not invoke any CPUID
// queries, thus avoiding their side effects like serialization and VM exits.
// Call-site-specific hard coded constants and macros for caching CPUID query
// outputs can also be avoided.
//
// - Return sanitized CPUID data.  Below APIs return NULL if the given CPUID
// leaf/subleaf input is not supported by hardware, or if the hardware CPUID
// output was deemed invalid by the CPUID parser.  This centralizes all CPUID
// data sanitization in one place (the kernel's CPUID parser.)
//
// - A centralized global view of system CPUID data.  Below APIs will reflect
// any kernel-enforced feature masking or overrides, unlike ad hoc parsing of
// raw CPUID output by drivers and individual call sites.
//
// Call-site APIs for CPUID leaves with a single subleaf:
//
// cpuid_subleaf() - Access parsed CPUID
// @_cpuinfo:	CPU capability structure reference ('struct cpuinfo_x86')
// @_leaf:	CPUID leaf, in compile-time 0xN format; e.g. 0x7, 0xf
// @_subleaf:	CPUID subleaf, in compile-time decimal format; e.g. 0, 1, 3
//
// Returns a pointer to parsed CPUID output, from the CPUID table inside
// @_cpuinfo, as a <cpuid/leaf_types.h> data type: 'struct leaf_0xM_N', where
// 0xM is the token provided at @_leaf, and N is the token provided at
// @_subleaf; e.g. struct leaf_0x7_0.
//
// Returns NULL if the requested CPUID @_leaf/@_subleaf query output is not
// present at the parsed CPUID table inside @_cpuinfo.  This can happen if:
//
// - The CPUID table inside @_cpuinfo has not yet been populated.
// - The CPUID table inside @_cpuinfo was populated, but the CPU does not
// implement the requested CPUID @_leaf/@_subleaf combination.
// - The CPUID table inside @_cpuinfo was populated, but the kernel's CPUID
// parser has predetermined that the requested CPUID @_leaf/@_subleaf
// hardware output is invalid or unsupported.
//
// Example usage::
//
// const struct leaf_0x7_0 *l7_0 = cpuid_subleaf(c, 0x7, 0);
// if (!l7_0) {
// // Handle error
// }
//
// const struct leaf_0x7_1 *l7_1 = cpuid_subleaf(c, 0x7, 1);
// if (!l7_1) {
// // Handle error
// }
//

//
// cpuid_leaf() - Access parsed CPUID data
// @_cpuinfo:	CPU capability structure reference ('struct cpuinfo_x86')
// @_leaf:	CPUID leaf, in compile-time 0xN format; e.g. 0x0, 0x2, 0x80000000
//
// Similar to cpuid_subleaf(), but with a CPUID subleaf = 0.
//
// Example usage::
//
// const struct leaf_0x0_0 *l0 = cpuid_leaf(c, 0x0);
// if (!l0) {
// // Handle error
// }
//
// const struct leaf_0x80000000_0 *el0 = cpuid_leaf(c, 0x80000000);
// if (!el0) {
// // Handle error
// }
//

//
// cpuid_leaf_raw() - Access parsed CPUID data in raw format
// @_cpuinfo:	CPU capability structure reference ('struct cpuinfo_x86')
// @_leaf:	CPUID leaf, in compile-time 0xN format
//
// Similar to cpuid_leaf(), but returns a raw 'struct cpuid_regs' pointer to
// the parsed CPUID data instead of a "typed" <asm/cpuid/leaf_types.h> pointer.
//

//
// Call-site APIs for CPUID leaves with a subleaf range:
//
// cpuid_subleaf_n() - Access parsed CPUID data for leaf with a subleaf range
// @_cpuinfo:	CPU capability structure reference ('struct cpuinfo_x86')
// @_leaf:	CPUID leaf, in compile-time 0xN format; e.g. 0x4, 0x8000001d
// @_subleaf:	Subleaf number, which can be passed dynamically.  It must be smaller
// than cpuid_subleaf_count(@_cpuinfo, @_leaf).
//
// Build-time errors will be emitted in the following cases:
//
// - @_leaf has no subleaf range.  Leaves with a subleaf range have an '_n' type
// suffix and are listed at <asm/cpuid/types.h> using the CPUID_LEAF_N() macro.
//
// - @_subleaf is known at compile-time but is out of range.
//
// Example usage::
//
// const struct leaf_0x4_n *l4;
//
// for (int i = 0; i < cpuid_subleaf_count(c, 0x4); i++) {
// l4 = cpuid_subleaf_n(c, 0x4, i);
// if (!l4) {
// // Handle error
// }
// ...
// }
//
// Beside the standard error situations detailed at cpuid_subleaf(), this
// macro will also return NULL if @_subleaf is out of the leaf's subleaf range.
//

//
// cpuid_subleaf_n_raw() - Access parsed CPUID data for leaf with subleaf range
// @_cpuinfo:	CPU capability structure reference ('struct cpuinfo_x86')
// @_leaf:	CPUID leaf, in compile-time 0xN format; e.g. 0x4, 0x8000001d
// @_subleaf:	Subleaf number, which can be passed dynamically.  It must be smaller
// than cpuid_subleaf_count(@_cpuinfo, @_leaf).
//
// Similar to cpuid_subleaf_n(), but returns a raw 'struct cpuid_regs' pointer to
// the parsed CPUID data instead of a "typed" <asm/cpuid/leaf_types.h> pointer.
//

//
// cpuid_subleaf_count() - Number of filled subleaves for @_leaf
// @_cpuinfo:	CPU capability structure reference ('struct cpuinfo_x86')
// @_leaf:	CPUID leaf, in compile-time 0xN format; e.g. 0x4, 0x8000001d
//
// Return the number of subleaves filled by the CPUID parser for @_leaf.
//
// @_leaf must have subleaf range.  Leaves with a subleaf range have an '_n' type
// suffix and are listed at <asm/cpuid/types.h> using the CPUID_LEAF_N() macro.
//

//
// CPUID parser exported APIs:
//
extern "C" {
    pub fn cpuid_scan_cpu(c: *mut cpuinfo_x86);
}
extern "C" {
    pub fn cpuid_refresh_leaf(c: *mut cpuinfo_x86, leaf: u32);
}
extern "C" {
    pub fn cpuid_refresh_range(c: *mut cpuinfo_x86, start: u32, end: u32);
}
