//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/vdso/gettimeofday.h
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
// Fast user context implementation of clock_gettime, gettimeofday, and time.
//
// Copyright (C) 2019 ARM Limited.
// Copyright 2006 Andi Kleen, SUSE Labs.
// 32 Bit compat layer by Stefani Seibold <stefani@seibold.net>
// sponsored by Rohde & Schwarz GmbH & Co. KG Munich/Germany
//

pub const VDSO_HAS_TIME: c_int = 1;
pub const VDSO_HAS_CLOCK_GETRES: c_int = 1;
//
// Declare the memory-mapped vclock data pages.  These come from hypervisors.
// If we ever reintroduce something like direct access to an MMIO clock like
// the HPET again, it will go here as well.
//
// A load from any of these pages will segfault if the clock in question is
// disabled, so appropriate compiler barriers and checks need to be used
// to prevent stray loads.
//
// These declarations MUST NOT be const.  The compiler will assume that
// an extern const variable has genuinely constant contents, and the
// resulting code won't work, since the whole point is that these pages
// change over time, possibly while we're accessing them.
//

//
// This is the vCPU 0 pvclock page.  We only use pvclock from the vDSO
// if the hypervisor tells us that all vCPUs can get valid data from the
// vCPU 0 page.
//

extern "C" {
    pub fn VDSO_SYSCALL2(_arg: clock_gettime, _arg: 64, _arg: _clkid, _arg: _ts) -> return;
}
extern "C" {
    pub fn VDSO_SYSCALL2(_arg: gettimeofday, _arg: , _arg: _tv, _arg: _tz) -> return;
}
extern "C" {
    pub fn VDSO_SYSCALL2(_arg: clock_getres, _arg: _time64, _arg: _clkid, _arg: _ts) -> return;
}

extern "C" {
    pub fn VDSO_SYSCALL2(_arg: clock_gettime, _arg: , _arg: _clkid, _arg: _ts) -> return;
}
extern "C" {
    pub fn VDSO_SYSCALL2(_arg: clock_getres, _arg: , _arg: _clkid, _arg: _ts) -> return;
}

//
// Note: The kernel and hypervisor must guarantee that cpu ID
// number maps 1:1 to per-CPU pvclock time info.
//
// Because the hypervisor is entirely unaware of guest userspace
// preemption, it cannot guarantee that per-CPU pvclock time
// info is updated if the underlying CPU changes or that that
// version is increased whenever underlying CPU changes.
//
// On KVM, we are guaranteed that pvti updates for any vCPU are
// atomic as seen by *all* vCPUs.  This is an even stronger
// guarantee than we get with a normal seqlock.
//
// On Xen, we don't appear to have that guarantee, but Xen still
// supplies a valid seqlock using the version field.
//
// We only do pvclock vdso timing at all if
// PVCLOCK_TSC_STABLE_BIT is set, and we interpret that bit to
// mean that all vCPUs have matching pvti and that the TSC is
// synced, so we can just look at vCPU 0's pvti.
//

//
// For any memory-mapped vclock type, we need to make sure that gcc
// doesn't cleverly hoist a load before the mode check.  Otherwise we
// might end up touching the memory-mapped page even if the vclock in
// question isn't enabled, which will segfault.  Hence the barriers.
//

extern "C" {
    pub fn vread_pvclock() -> return;
}

extern "C" {
    pub fn vread_hvclock() -> return;
}

//
// Clocksource read value validation to handle PV and HyperV clocksources
// which can be invalidated asynchronously and indicate invalidation by
// returning U64_MAX, which can be effectively tested by checking for a
// negative value after casting it to s64.
//
// This effectively forces a S64_MAX mask on the calculations, unlike the
// U64_MAX mask normally used by x86 clocksources.
//

//
// x86 specific calculation of nanoseconds for the current cycle count
//
// The regular implementation assumes that clocksource reads are globally
// monotonic. The TSC can be slightly off across sockets which can cause
// the regular delta calculation (@cycles - @last) to return a huge time
// jump.
//
// Therefore it needs to be verified that @cycles are greater than
// @vd->cycles_last. If not then use @vd->cycles_last, which is the base
// time of the current conversion period.
//
// This variant also uses a custom mask because while the clocksource mask of
// all the VDSO capable clocksources on x86 is U64_MAX, the above code uses
// U64_MASK as an exception value, additionally arch_vdso_cycles_ok() above
// declares everything with the MSB/Sign-bit set as invalid. Therefore the
// effective mask is S64_MAX.
//
// Negative motion and deltas which can cause multiplication
// overflow require special treatment. This check covers both as
// negative motion is guaranteed to be greater than @vc::max_cycles
// due to unsigned comparison.
//
// Due to the MSB/Sign-bit being used as invalid marker (see
// arch_vdso_cycles_ok() above), the effective mask is S64_MAX, but that
// case is also unlikely and will also take the unlikely path here.
//
// Due to the above mentioned TSC wobbles, filter out
// negative motion.  Per the above masking, the effective
// sign bit is now bit 62.
//
// Handle multiplication overflow gracefully
extern "C" {
    pub fn mul_u64_u32_add_u64_shr(S64_MAX: delta &, _arg: vc->mult, _arg: base, _arg: vc->shift) -> return;
}

