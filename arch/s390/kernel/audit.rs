//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/audit.c
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

    static unsigned dir_class[] = {

    ~0U
    };
    static unsigned read_class[] = {

    ~0U
    };
    static unsigned write_class[] = {

    ~0U
    };
    static unsigned chattr_class[] = {

    ~0U
    };
    static unsigned signal_class[] = {

    ~0U
    };
#[no_mangle]
pub unsafe extern "C" fn audit_classify_arch(arch: c_int) -> c_int {
    int audit_classify_arch(int arch)
    {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn audit_classify_syscall(abi: c_int, syscall: unsigned) -> c_int {
    int audit_classify_syscall(int abi, unsigned syscall)
    {
    switch(syscall) {
    case __NR_open:
    return AUDITSC_OPEN;
    case __NR_openat:
    return AUDITSC_OPENAT;
    case __NR_socketcall:
    return AUDITSC_SOCKETCALL;
    case __NR_execve:
    return AUDITSC_EXECVE;
    case __NR_openat2:
    return AUDITSC_OPENAT2;
    default:
    return AUDITSC_NATIVE;
    }
    }
#[no_mangle]
unsafe extern "C" fn audit_classes_init() -> int __init {
    static int __init audit_classes_init(void)
    {
    audit_register_class(AUDIT_CLASS_WRITE, write_class);
    audit_register_class(AUDIT_CLASS_READ, read_class);
    audit_register_class(AUDIT_CLASS_DIR_WRITE, dir_class);
    audit_register_class(AUDIT_CLASS_CHATTR, chattr_class);
    audit_register_class(AUDIT_CLASS_SIGNAL, signal_class);
    return 0;
    }
    __initcall(audit_classes_init);
