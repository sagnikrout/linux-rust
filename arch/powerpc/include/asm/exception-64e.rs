//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/exception-64e.h
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
// Definitions for use by exception code on Book3-E
//
// Copyright (C) 2008 Ben. Herrenschmidt (benh@kernel.crashing.org), IBM Corp.
//
// SPRGs usage an other considerations...
//
// Since TLB miss and other standard exceptions can be interrupted by
// critical exceptions which can themselves be interrupted by machine
// checks, and since the two later can themselves cause a TLB miss when
// hitting the linear mapping for the kernel stacks, we need to be a bit
// creative on how we use SPRGs.
//
// The base idea is that we have one SRPG reserved for critical and one
// for machine check interrupts. Those are used to save a GPR that can
// then be used to get the PACA, and store as much context as we need
// to save in there. That includes saving the SPRGs used by the TLB miss
// handler for linear mapping misses and the associated SRR0/1 due to
// the above re-entrancy issue.
//
// So here's the current usage pattern. It's done regardless of which
// SPRGs are user-readable though, thus we might have to change some of
// this later. In order to do that more easily, we use special constants
// for naming them
//
// WARNING: Some of these SPRGs are user readable. We need to do something
// about it as some point by making sure they can't be used to leak kernel
// critical data
//

// We are out of SPRGs so we save some things in the PACA. The normal
// exception frame is smaller than the CRIT or MC one though
//

//
// The TLB miss exception uses different slots.
//
// The bolted variant uses only the first six fields,
// which in combination with pgd and kernel_pgd fits in
// one 64-byte cache line.
//

// TLB miss exception prolog
//
// This prolog handles re-entrancy (up to 3 levels supported in the PACA
// though we currently don't test for overflow). It provides you with a
// re-entrancy safe working space of r10...r16 and CR with r12 being used
// as the exception area pointer in the PACA for that level of re-entrancy
// and r13 containing the PACA pointer.
//
// SRR0 and SRR1 are saved, but DEAR and ESR are not, since they don't apply
// as-is for instruction exceptions. It's up to the actual exception code
// to save them as well if required.
//

// And these are the matching epilogs that restores things
//
// There are 3 epilogs:
//
// - SUCCESS       : Unwinds one level
// - ERROR         : restore from level 0 and reset
// - ERROR_SPECIAL : restore from current level and reset
//
// Normal errors use ERROR, that is, they restore the initial fault context
// and trigger a fault. However, there is a special case for linear mapping
// errors. Those should basically never happen, but if they do happen, we
// want the error to point out the context that did that linear mapping
// fault, not the initial level 0 (basically, we got a bogus PGF or something
// like that). For userland errors on the linear mapping, there is no
// difference since those are always level 0 anyway
//

//
// powerpc relies on return from interrupt/syscall being context synchronising
// (which rfi is) to support ARCH_HAS_MEMBARRIER_SYNC_CORE without additional
// synchronisation instructions.
//

