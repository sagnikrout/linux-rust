//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/kernel/image-vars.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Linker script variables to be set after section resolution, as
// ld.lld does not like variables assigned before SECTIONS is processed.
//

// Macro flag: #define ASSERT(...)

//
// The EFI stub has its own symbol namespace prefixed by __efistub_, to
// isolate it from the kernel proper. The following symbols are legally
// accessed by the stub, so provide some aliases to make them accessible.
// Only include data symbols here, or text symbols of functions that are
// guaranteed to be safe when executed at another offset than they were
// linked at. The routines below are all implemented in assembler in a
// position independent manner
//

//
// KVM nVHE code has its own symbol namespace prefixed with __kvm_nvhe_, to
// separate it from the kernel proper. The following symbols are legally
// accessed by it, therefore provide aliases to make them linkable.
// Do not include symbols which may not be safely accessed under hypervisor
// memory mappings.
//
// Alternative callbacks for init-time patching of nVHE hyp code.
// Global kernel state accessed by nVHE hyp code.
// Kernel symbols used to call panic() from nVHE hyp code (via ERET).
// Vectors installed by hyp-init on reset HVC.
// Static keys which are set if a vGIC trap should be handled in hyp.
// Static key indicating whether GICv3 has GICv2 compatibility
// Static key which is set if CNTVOFF_EL2 is unusable
// EL2 exception handling
// Position-independent library routines

// Hyp memory sections

// pKVM static key

//
// LLD will occasionally error out with a '__init_end does not converge' error
// if INIT_IDMAP_DIR_SIZE is defined in terms of _end, as this results in a
// circular dependency. Counter this by dimensioning the initial IDMAP page
// tables based on kimage_limit, which is defined such that its value should
// not change as a result of the initdata segment being pushed over a 64k
// segment boundary due to changes in INIT_IDMAP_DIR_SIZE, provided that its
// value doesn't change by more than 2M between linker passes.
//

