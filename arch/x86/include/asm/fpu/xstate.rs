//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/fpu/xstate.h
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

// Bit 63 of XCR0 is reserved for future expansion

pub const FXSAVE_SIZE: c_int = 512;
pub const XSAVE_HDR_SIZE: c_int = 64;

pub const XSAVE_YMM_SIZE: c_int = 256;

pub const XSAVE_ALIGNMENT: c_int = 64;
// All currently supported user features

//
// Features which are restored when returning to user space.
// PKRU is not restored on return to user space because PKRU
// is switched eagerly in switch_to() and flush_thread()
//

// Features which are dynamically enabled for a process on request

// Supervisor features which are enabled only in guest FPUs

// All currently supported supervisor features

//
// A supervisor state component may not always contain valuable information,
// and its size may be huge. Saving/restoring such supervisor state components
// at each context switch can cause high CPU and space overhead, which should
// be avoided. Such supervisor state components should only be saved/restored
// on demand. The on-demand supervisor features are set in this mask.
//
// Unlike the existing supported supervisor features, an independent supervisor
// feature does not allocate a buffer in task->fpu, and the corresponding
// supervisor state component cannot be saved/restored at each context switch.
//
// To support an independent supervisor feature, a developer should follow the
// dos and don'ts as below:
// - Do dynamically allocate a buffer for the supervisor state component.
// - Do manually invoke the XSAVES/XRSTORS instruction to save/restore the
// state component to/from the buffer.
// - Don't set the bit corresponding to the independent supervisor feature in
// IA32_XSS at run time, since it has been set at boot time.
//

//
// Unsupported supervisor features. When a supervisor feature in this mask is
// supported in the future, move it to the supported supervisor feature mask.
//

// All supervisor states including supported and unsupported states.

//
// The feature mask required to restore FPU state:
// - All user states which are not eagerly switched in switch_to()/exec()
// - The suporvisor states
//

//
// Features in this mask have space allocated in the signal frame, but may not
// have that space initialized when the feature is in its init state.
//

extern "C" {
    pub fn xfeature_size(xfeature_nr: c_int) -> c_int;
}
extern "C" {
    pub fn xsaves(xsave: *mut xregs_state, mask: u64);
}
extern "C" {
    pub fn xrstors(xsave: *mut xregs_state, mask: u64);
}
extern "C" {
    pub fn xfd_enable_feature(xfd_err: u64) -> c_int;
}

extern "C" {
    pub fn static_branch_unlikely(_arg: &__fpu_state_size_dynamic) -> return;
}

