//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kernel/fpu/xstate.h
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
// XRSTORS requires these bits set in xcomp_bv, or it will
// trigger #GP:
//
// Pairs with WRITE_ONCE() in xstate_request_perm()
extern "C" {
    pub fn READ_ONCE(_arg: perm->__state_perm) -> return;
}
extern "C" {
    pub fn xstate_get_group_perm(_arg: false) -> return;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xstate_copy_mode {
    XSTATE_COPY_FP,
    XSTATE_COPY_FX,
    XSTATE_COPY_XSAVE,
}

extern "C" {
    pub fn copy_uabi_from_kernel_to_xstate(fpstate: *mut fpstate, kbuf: *const c_void, pkru: *mut u32) -> c_int;
}
extern "C" {
    pub fn copy_sigframe_from_user_to_xstate(tsk: *mut task_struct, ubuf: *const void __user) -> c_int;
}
extern "C" {
    pub fn fpu__init_cpu_xstate();
}
extern "C" {
    pub fn fpu__init_system_xstate(legacy_size: c_uint);
}
// Read the xfeatures value already saved in the user buffer
//
// Update the value of PKRU register that was already pushed onto the signal frame.
//
// Mark PKRU as in-use so that it is restored correctly.
// Update PKRU value in the userspace xsave buffer.
extern "C" {
    pub fn __put_user(_arg: pkru, )get_xsave_addr_user(buf: *mut (unsigned int __user, _arg: XFEATURE_PKRU)) -> return;
}
// XSAVE/XRSTOR wrapper functions

// Macro flag: #define REX_SUFFIX

//
// After this @err contains 0 on success or the trap number when the
// operation raises an exception.
//
// The [xa] input parameter below represents the struct xregs_state pointer
// and the asm symbolic name for the argument used in the XSAVE/XRSTOR insns
// above.
//

//
// If XSAVES is enabled, it replaces XSAVEC because it supports supervisor
// states in addition to XSAVEC.
//
// Otherwise if XSAVEC is enabled, it replaces XSAVEOPT because it supports
// compacted storage format in addition to XSAVEOPT.
//
// Otherwise, if XSAVEOPT is enabled, XSAVEOPT replaces XSAVE because XSAVEOPT
// supports modified optimization which is not supported by XSAVE.
//
// Use XSAVE as a fallback.
//

//
// Use XRSTORS to restore context if it is enabled. XRSTORS supports compact
// XSAVE area format.
//

extern "C" {
    pub fn xfd_validate_state(fpstate: *mut fpstate, mask: u64, rstor: bool);
}

extern "C" {
    pub fn __xfd_enable_feature(which: u64, guest_fpu: *mut fpu_guest) -> c_int;
}

//
// Save processor xstate to xsave area.
//
// Uses either XSAVE or XSAVEOPT or XSAVES depending on the CPU features
// and command line options. The choice is permanent until the next reboot.
//
// We should never fault when copying to a kernel buffer:
//
// Restore processor xstate from xsave area.
//
// Uses XRSTORS when XSAVES is used, XRSTOR otherwise.
//
// Restore of supervisor state. Does not require XFD
//
// XSAVE itself always writes all requested xfeatures.  Removing features
// from the request bitmap reduces the features which are written.
// Generate a mask of features which must be written to a sigframe.  The
// unset features can be optimized away and not written.
//
// This optimization is user-visible.  Only use for states where
// uninitialized sigframe contents are tolerable, like dynamic features.
//
// Users of buffers produced with this optimization must check XSTATE_BV
// to determine which features have been optimized out.
//
// In-use features must be written:
// Also write all non-optimizable sigframe features:
//
// Save xstate to user space xsave area.
//
// We don't use modified optimization because xrstor/xrstors might track
// a different application.
//
// We don't use compacted format xsave area for backward compatibility for
// old applications which don't understand the compacted format of the
// xsave area.
//
// The caller has to zero buf::header before calling this because XSAVE
// does not touch the reserved fields in the header.
//
// Include the features which are not xsaved/rstored by the kernel
// internally, e.g. PKRU. That's user space ABI and also required
// to allow the signal handler to modify PKRU.
//
// Optimize away writing unnecessary xfeatures:
//
// Restore xstate from user space xsave area.
//
// Restore xstate from kernel space xsave area, return an error code instead of
// an exception.
//
// Ensure that XFD is up to date
