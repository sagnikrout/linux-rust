//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/sync_core.h
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
// This function forces the icache and prefetched instruction stream to
// catch up with reality in two very specific cases:
//
// a) Text was modified using one virtual address and is about to be executed
// from the same physical page at a different virtual address.
//
// b) Text was modified on a different CPU, may subsequently be
// executed on this CPU, and you want to make sure the new version
// gets executed.  This generally means you're calling this in an IPI.
//
// If you're calling this for a different reason, you're probably doing
// it wrong.
//
// Like all of Linux's memory ordering operations, this is a
// compiler barrier as well.
//
// The SERIALIZE instruction is the most straightforward way to
// do this, but it is not universally available.
//
// For all other processors, there are quite a few ways to do this.
// IRET-to-self is nice because it works on every CPU, at any CPL
// (so it's compatible with paravirtualization), and it never exits
// to a hypervisor.  The only downsides are that it's a bit slow
// (it seems to be a bit more than 2x slower than the fastest
// options) and that it unmasks NMIs.  The "push %cs" is needed,
// because in paravirtual environments __KERNEL_CS may not be a
// valid CS value when we do IRET directly.
//
// In case NMI unmasking or performance ever becomes a problem,
// the next best option appears to be MOV-to-CR2 and an
// unconditional jump.  That sequence also works on all CPUs,
// but it will fault at CPL3 (i.e. Xen PV).
//
// CPUID is the conventional way, but it's nasty: it doesn't
// exist on some 486-like CPUs, and it usually exits to a
// hypervisor.
//
// Ensure that a core serializing instruction is issued before returning
// to user-mode. x86 implements return to user-space through sysexit,
// sysrel, and sysretq, which are not core serializing.
//
// With PTI, we unconditionally serialize before running user code.
//
// Even if we're in an interrupt, we might reschedule before returning,
// in which case we could switch to a different thread in the same mm
// and return using SYSRET or SYSEXIT.  Instead of trying to keep
// track of our need to sync the core, just sync right away.
//
