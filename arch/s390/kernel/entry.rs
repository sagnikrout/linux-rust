//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/kernel/entry.h
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
pub const PGM_FLAG_GUEST_FAULT: c_int = 1;
pub const SYSCALL_FLAG_PER_TRAP: c_int = 1;

extern "C" {
    pub fn system_call();
}
extern "C" {
    pub fn pgm_check_handler();
}
extern "C" {
    pub fn ext_int_handler();
}
extern "C" {
    pub fn io_int_handler();
}
extern "C" {
    pub fn mcck_int_handler();
}
extern "C" {
    pub fn restart_int_handler();
}
extern "C" {
    pub fn early_pgm_check_handler();
}
extern "C" {
    pub fn __ret_from_fork(prev: *mut task_struct, regs: *mut pt_regs);
}
extern "C" {
    pub fn __do_pgm_check(regs: *mut pt_regs, flags: c_ulong);
}
extern "C" {
    pub fn __do_syscall(regs: *mut pt_regs, flags: c_ulong);
}
extern "C" {
    pub fn __do_early_pgm_check(regs: *mut pt_regs);
}
extern "C" {
    pub fn do_protection_exception(regs: *mut pt_regs);
}
extern "C" {
    pub fn do_dat_exception(regs: *mut pt_regs);
}
extern "C" {
    pub fn do_secure_storage_access(regs: *mut pt_regs);
}
extern "C" {
    pub fn do_non_secure_storage_access(regs: *mut pt_regs);
}
extern "C" {
    pub fn do_secure_storage_violation(regs: *mut pt_regs);
}
extern "C" {
    pub fn do_report_trap(regs: *mut pt_regs, si_signo: c_int, si_code: c_int, str: *mut c_char);
}
extern "C" {
    pub fn kernel_stack_invalid(regs: *mut pt_regs);
}
extern "C" {
    pub fn do_io_irq(regs: *mut pt_regs);
}
extern "C" {
    pub fn do_ext_irq(regs: *mut pt_regs);
}
extern "C" {
    pub fn do_restart(arg: *mut c_void);
}
extern "C" {
    pub fn startup_init() -> void __init;
}
extern "C" {
    pub fn die(regs: *mut pt_regs, str: *const c_char);
}
extern "C" {
    pub fn setup_profiling_timer(multiplier: c_uint) -> c_int;
}
extern "C" {
    pub fn sys_rt_sigreturn() -> c_long;
}
extern "C" {
    pub fn sys_sigreturn() -> c_long;
}
extern "C" {
    pub fn sys_s390_personality(personality: c_uint) -> c_long;
}
extern "C" {
    pub fn sys_s390_runtime_instr(command: c_int, signum: c_int) -> c_long;
}
extern "C" {
    pub fn sys_s390_guarded_storage(command: c_int, : *mut gs_cb __user) -> c_long;
}
extern "C" {
    pub fn sys_s390_pci_mmio_write(long: unsigned, : *const void __user, _arg: usize) -> c_long;
}
extern "C" {
    pub fn sys_s390_pci_mmio_read(long: unsigned, : *mut void __user, _arg: usize) -> c_long;
}
extern "C" {
    pub fn sys_s390_sthyi(function_code: c_ulong, buffer: *mut void __user, return_code: *mut u64 __user, flags: c_ulong) -> c_long;
}
extern "C" {
    pub fn stack_alloc() -> c_ulong;
}
extern "C" {
    pub fn stack_free(stack: c_ulong);
}

