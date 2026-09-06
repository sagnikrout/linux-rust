//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/cpufeatures.h
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
// Defines x86 CPU feature bits
//

//
// Note: If the comment begins with a quoted string, that string is used
// in /proc/cpuinfo instead of the macro name.  Otherwise, this feature
// bit is not displayed in /proc/cpuinfo at all.
//
// When adding new features here that depend on other features,
// please update the table in kernel/cpu/cpuid-deps.c as well.
//
// Intel-defined CPU features, CPUID level 0x00000001 (EDX), word 0

// AMD-defined CPU features, CPUID level 0x80000001, word 1
// Don't duplicate feature flags which are redundant with Intel!

// Transmeta-defined CPU features, CPUID level 0x80860001, word 2

// Other features, Linux-defined mapping, word 3
// This range is used for feature bits which conflict or are synthesized

// Free                                 ( 3*32+ 7)

// free: was #define X86_FEATURE_UP	( 3*32+ 9) * "up" SMP kernel running on UP

// Intel-defined CPU features, CPUID level 0x00000001 (ECX), word 4

// VIA/Cyrix/Centaur-defined CPU features, CPUID level 0xC0000001, word 5

// More extended AMD flags: CPUID level 0x80000001, ECX, word 6

//
// Auxiliary flags: Linux defined - For features scattered in various
// CPUID levels like 0x6, 0xA etc, word 7.
//
// Reuse free bits when adding new feature flags!
//

// Virtualization flags: Linux defined, word 8

// free: was #define X86_FEATURE_PVUNLOCK               ( 8*32+20) /* PV unlock function

// Intel-defined CPU features, CPUID level 0x00000007:0 (EBX), word 9

// Extended state features, CPUID level 0x0000000d:1 (EAX), word 10

//
// Extended auxiliary flags: Linux defined - for features scattered in various
// CPUID levels like 0xf, etc.
//
// Reuse free bits when adding new feature flags!
//

// Intel-defined CPU features, CPUID level 0x00000007:1 (EAX), word 12

// AMD-defined CPU features, CPUID level 0x80000008 (EBX), word 13

// Thermal and Power Management Leaf, CPUID level 0x00000006 (EAX), word 14

// AMD SVM Feature Identification, CPUID level 0x8000000a (EDX), word 15

// Intel-defined CPU features, CPUID level 0x00000007:0 (ECX), word 16

//
// Linux-defined word for use with scattered/synthetic bits.
//

// Intel-defined CPU features, CPUID level 0x00000007:0 (EDX), word 18

// AMD-defined memory encryption features, CPUID level 0x8000001f (EAX), word 19

// AMD-defined Extended Feature 2 EAX, CPUID level 0x80000021 (EAX), word 20

// BP_CFG[BpSpecReduce] can be used to mitigate SRSO for VMs.
// (SRSO_MSR_FIX in the official doc).
//
// Extended auxiliary flags: Linux defined - for features scattered in various
// CPUID levels like 0x80000022, etc and Linux defined features.
//
// Reuse free bits when adding new feature flags!
//

// Clear CPU buffers before VM-Enter if the vCPU
// can access host MMIO (ignored for all intents
// and purposes if CLEAR_CPU_BUF_VM is set).
//

//
// BUG word(s)
//

//
// 64-bit kernels don't use X86_BUG_ESPFIX.  Make the define conditional
// to avoid confusion.
//

// unused, was #define X86_BUG_MMIO_UNKNOWN		X86_BUG(26) "mmio_unknown" CPU is too old and its MMIO Stale Data status is unknown

// BUG word 2

