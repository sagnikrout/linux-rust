//! Automatically rewritten from C to Rust
//! Source: kernel/cfi.c
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

macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notifier_block { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_notifier_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;


























// SPDX-License-Identifier: GPL-2.0
//
// Clang Control Flow Integrity (CFI) error handling.
//
// Copyright (C) 2022 Google LLC
//

pub static mut __ro_after_init: bool cfi_warn = IS_ENABLED(CONFIG_CFI_PERMISSIVE);
    enum bug_trap_type report_cfi_failure(struct pt_regs *regs, unsigned long addr,
    unsigned long *target, u32 type)
    {
    if (target) {
    pr_err("CFI failure at %pS (target: %pS; expected type: 0x%08x)\n",
    }
    addr, *target, type);
    else {
    pr_err("CFI failure at %pS (no target information)\n",
    addr);
    }
    if (cfi_warn) {
    __warn(core::ptr::null_mut(), 0, addr, 0, regs, core::ptr::null_mut());
    return BUG_TRAP_TYPE_WARN;
    }
    return BUG_TRAP_TYPE_BUG;
    }
//
// Declare two non-existent functions with types that match bpf_func_t and
// bpf_callback_t pointers, and use DEFINE_CFI_TYPE to define type hash
// variables for each function type. The cfi_bpf_* variables are used by
// arch-specific BPF JIT implementations to ensure indirectly callable JIT
// code has matching CFI type hashes.
//
    extern typeof(*(bpf_func_t)0) __bpf_prog_runX;
// DEFINE_CFI_TYPE;
    extern typeof(*(bpf_callback_t)0) __bpf_callback_fn;
// DEFINE_CFI_TYPE;

#[no_mangle]
pub unsafe extern "C" fn trap_address(p: *mut i32) -> c_ulong {
    return (unsigned long)((long)p + (long)*p);
    }
#[no_mangle]
unsafe extern "C" fn is_trap(addr: c_ulong, start: *mut i32, end: *mut i32) -> bool {
    let mut p = core::ptr::null_mut();
    for (p = start; p < end; ++p) {
    if (trap_address(p) == addr) {
    return true;
    }
    }
    return false;
    }

// Populates `kcfi_trap(_end)?` fields in `struct module`.
#[no_mangle]
pub unsafe extern "C" fn module_cfi_finalize() {
    let mut secstrings = core::ptr::null_mut();
    let mut i = 0;
    mod.kcfi_traps = core::ptr::null_mut();
    mod.kcfi_traps_end = core::ptr::null_mut();
    secstrings = hdr + sechdrs[hdr.e_shstrndx].sh_offset;
    for (i = 1; i < hdr.e_shnum; i++) {
    if (strcmp(secstrings + sechdrs[i].sh_name, "__kcfi_traps")) {
    continue;
    }
    mod.kcfi_traps = sechdrs[i].sh_addr;
    mod.kcfi_traps_end = (sechdrs[i].sh_addr + sechdrs[i].sh_size);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn is_module_cfi_trap(addr: c_ulong) -> bool {
    let mut mod = core::ptr::null_mut();
pub static mut found: bool = false;
    guard(rcu)();
    mod = __module_address(addr);
    if (mod) {
    found = is_trap(addr, mod.kcfi_traps, mod.kcfi_traps_end);
    }
    return found;
    }

#[no_mangle]
pub unsafe extern "C" fn is_module_cfi_trap(addr: c_ulong) -> bool {
    return false;
    }

    extern s32 __start___kcfi_traps[];
    extern s32 __stop___kcfi_traps[];
#[no_mangle]
pub unsafe extern "C" fn is_cfi_trap(addr: c_ulong) -> bool {
    if (is_trap(addr, __start___kcfi_traps, __stop___kcfi_traps)) {
    return true;
    }
    return is_module_cfi_trap(addr);
    }