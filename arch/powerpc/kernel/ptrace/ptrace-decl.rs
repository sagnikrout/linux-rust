//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/kernel/ptrace/ptrace-decl.h
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
// Set of msr bits that gdb can change on behalf of a process.
//

pub const MSR_DEBUGCHANGE: c_int = 0;

//
// Max register writeable via put_reg
//

//
// These are our native regset flavors.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum powerpc_regset {
    REGSET_GPR,
    REGSET_FPR,

    REGSET_VMX,

    REGSET_VSX,

    REGSET_SPE,

    REGSET_TM_CGPR,		/* TM checkpointed GPR registers */
    REGSET_TM_CFPR,		/* TM checkpointed FPR registers */
    REGSET_TM_CVMX,		/* TM checkpointed VMX registers */
    REGSET_TM_CVSX,		/* TM checkpointed VSX registers */
    REGSET_TM_SPR,		/* TM specific SPR registers */
    REGSET_TM_CTAR,		/* TM checkpointed TAR register */
    REGSET_TM_CPPR,		/* TM checkpointed PPR register */
    REGSET_TM_CDSCR,	/* TM checkpointed DSCR register */

    REGSET_PPR,		/* PPR register */
    REGSET_DSCR,		/* DSCR register */

    REGSET_TAR,		/* TAR register */
    REGSET_EBB,		/* EBB registers */
    REGSET_PMR,		/* Performance Monitor Registers */
    REGSET_DEXCR,		/* DEXCR registers */

    REGSET_HASHKEYR,	/* HASHKEYR register */

    REGSET_PKEY,		/* AMR register */

}

// ptrace-(no)vsx
// ptrace-vsx
extern "C" {
    pub fn vsr_active(target: *mut task_struct, regset: *const user_regset) -> c_int;
}
// ptrace-altivec
extern "C" {
    pub fn vr_active(target: *mut task_struct, regset: *const user_regset) -> c_int;
}
// ptrace-spe
extern "C" {
    pub fn evr_active(target: *mut task_struct, regset: *const user_regset) -> c_int;
}
// ptrace
// ptrace-tm

extern "C" {
    pub fn flush_tmregs_to_thread(tsk: *mut task_struct);
}

extern "C" {
    pub fn tm_cgpr_active(target: *mut task_struct, regset: *const user_regset) -> c_int;
}
extern "C" {
    pub fn tm_cfpr_active(target: *mut task_struct, regset: *const user_regset) -> c_int;
}
extern "C" {
    pub fn tm_cvmx_active(target: *mut task_struct, regset: *const user_regset) -> c_int;
}
extern "C" {
    pub fn tm_cvsx_active(target: *mut task_struct, regset: *const user_regset) -> c_int;
}
extern "C" {
    pub fn tm_spr_active(target: *mut task_struct, regset: *const user_regset) -> c_int;
}
extern "C" {
    pub fn tm_tar_active(target: *mut task_struct, regset: *const user_regset) -> c_int;
}
extern "C" {
    pub fn tm_ppr_active(target: *mut task_struct, regset: *const user_regset) -> c_int;
}
extern "C" {
    pub fn tm_dscr_active(target: *mut task_struct, regset: *const user_regset) -> c_int;
}
// ptrace-view
extern "C" {
    pub fn ptrace_get_reg(task: *mut task_struct, regno: c_int, data: *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn ptrace_put_reg(task: *mut task_struct, regno: c_int, data: c_ulong) -> c_int;
}
// ptrace-fpu
extern "C" {
    pub fn ptrace_get_fpr(child: *mut task_struct, index: c_int, data: *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn ptrace_put_fpr(child: *mut task_struct, index: c_int, data: c_ulong) -> c_int;
}
// ptrace-(no)adv
extern "C" {
    pub fn ppc_gethwdinfo(dbginfo: *mut ppc_debug_info);
}
extern "C" {
    pub fn ptrace_set_debugreg(task: *mut task_struct, addr: c_ulong, data: c_ulong) -> c_int;
}
extern "C" {
    pub fn ppc_set_hwdebug(child: *mut task_struct, bp_info: *mut ppc_hw_breakpoint) -> c_long;
}
extern "C" {
    pub fn ppc_del_hwdebug(child: *mut task_struct, data: c_long) -> c_long;
}
