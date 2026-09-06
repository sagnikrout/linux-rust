//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/fpu/api.h
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
// Copyright (C) 1994 Linus Torvalds
//
// Pentium III FXSR, SSE support
// General FPU state handling cleanups
// Gareth Hughes <gareth@valinux.com>, May 2000
// x86-64 work by Andi Kleen 2002
//

//
// Use kernel_fpu_begin/end() if you intend to use FPU in kernel context. It
// disables preemption and softirq processing, so be careful if you intend to
// use it for long periods of time.  Kernel-mode FPU cannot be used in all
// contexts -- see irq_fpu_usable() for details.
//
// Kernel FPU states to initialize in kernel_fpu_begin_mask()

extern "C" {
    pub fn kernel_fpu_begin_mask(kfpu_mask: c_uint);
}
extern "C" {
    pub fn kernel_fpu_end();
}
extern "C" {
    pub fn irq_fpu_usable() -> bool;
}
extern "C" {
    pub fn fpregs_mark_activate();
}
// Code that is unaware of kernel_fpu_begin_mask() can use this

//
// Any 64-bit code that uses 387 instructions must explicitly request
// KFPU_387.
//

//
// 32-bit kernel code may use 387 operations as well as SSE2, etc,
// as long as it checks that the CPU has the required capability.
//

//
// Use fpregs_lock() while editing CPU's FPU registers or fpu->fpstate, or while
// using the FPU in kernel mode.  A context switch will (and softirq might) save
// CPU's FPU registers to fpu->fpstate.regs and set TIF_NEED_FPU_LOAD leaving
// CPU's FPU registers in a random state.
//
// local_bh_disable() protects against both preemption and soft interrupts
// on !RT kernels.
//
// On RT kernels local_bh_disable() is not sufficient because it only
// serializes soft interrupt related sections via a local lock, but stays
// preemptible. Disabling preemption is the right choice here as bottom
// half processing is always in thread context on RT kernels so it
// implicitly prevents bottom half processing as well.
//
// FPU state gets lazily restored before returning to userspace. So when in the
// kernel, the valid FPU state may be kept in the buffer. This function will force
// restore all the fpu state to the registers early if needed, and lock them from
// being automatically saved/restored. Then FPU state can be modified safely in the
// registers, before unlocking with fpregs_unlock().
//
extern "C" {
    pub fn fpregs_lock_and_load();
}

extern "C" {
    pub fn fpregs_assert_state_consistent();
}

//
// Load the task FPU state before returning to userspace.
//
extern "C" {
    pub fn switch_fpu_return();
}
//
// Query the presence of one or more xfeatures. Works on any legacy CPU as well.
//
// If 'feature_name' is set then put a human-readable description of
// the feature there as well - this can be used to print error (or success)
// messages.
//
extern "C" {
    pub fn cpu_has_xfeatures(xfeatures_mask: u64, feature_name: *const c_char) -> c_int;
}
// Trap handling
extern "C" {
    pub fn fpu__exception_code(fpu: *mut fpu, trap_nr: c_int) -> c_int;
}
extern "C" {
    pub fn fpu_sync_fpstate(fpu: *mut fpu);
}
extern "C" {
    pub fn fpu_reset_from_exception_fixup();
}
// Boot, hotplug and resume
extern "C" {
    pub fn fpu__init_cpu();
}
extern "C" {
    pub fn fpu__init_system();
}
extern "C" {
    pub fn fpu__init_check_bugs();
}
extern "C" {
    pub fn fpu__resume_cpu();
}
// State tracking
// Process cleanup

extern "C" {
    pub fn fpstate_free(fpu: *mut fpu);
}

// fpstate-related functions which are exported to KVM
extern "C" {
    pub fn fpstate_clear_xstate_component(fpstate: *mut fpstate, xfeature: c_uint);
}
extern "C" {
    pub fn xstate_get_guest_group_perm() -> u64;
}
// KVM specific functions
extern "C" {
    pub fn fpu_alloc_guest_fpstate(gfpu: *mut fpu_guest) -> bool;
}
extern "C" {
    pub fn fpu_free_guest_fpstate(gfpu: *mut fpu_guest);
}
extern "C" {
    pub fn fpu_swap_kvm_fpstate(gfpu: *mut fpu_guest, enter_guest: bool) -> c_int;
}
extern "C" {
    pub fn fpu_enable_guest_xfd_features(guest_fpu: *mut fpu_guest, xfeatures: u64) -> c_int;
}

extern "C" {
    pub fn fpu_update_guest_xfd(guest_fpu: *mut fpu_guest, xfd: u64);
}
extern "C" {
    pub fn fpu_sync_guest_vmexit_xfd_state();
}

extern "C" {
    pub fn fpu_copy_uabi_to_guest_fpstate(gfpu: *mut fpu_guest, buf: *const c_void, xcr0: u64, vpkru: *mut u32) -> c_int;
}
// prctl
extern "C" {
    pub fn fpu_xstate_prctl(option: c_int, arg2: c_ulong) -> c_long;
}
extern "C" {
    pub fn fpu_idle_fpregs();
}
