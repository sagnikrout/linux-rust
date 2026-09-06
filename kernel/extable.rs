//! Automatically rewritten from C to Rust
//! Source: kernel/extable.c
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


























// SPDX-License-Identifier: GPL-2.0-or-later
// Rewritten by Rusty Russell, on the backs of many others...
    Copyright (C) 2001 Rusty Russell, 2002 Rusty Russell IBM.
//

//
// mutex protecting text section modification (dynamic code patching).
// some users need to sleep (allocating memory...) while they hold this lock.
//
// Note: Also protects SMP-alternatives modification on x86.
//
// NOT exported to modules - patching kernel text is a really delicate matter.
//
// DEFINE_MUTEX;
    extern struct exception_table_entry __start___ex_table[];
    extern struct exception_table_entry __stop___ex_table[];
// Cleared by build time tools if the table is already sorted.
pub static mut main_extable_sort_needed: u32 __initdata __visible = 1;
// Sort the kernel's built-in exception table
#[no_mangle]
pub unsafe extern "C" fn sort_main_extable() -> c_int {
    if (main_extable_sort_needed &&
    &__stop___ex_table > &__start___ex_table) {
    pr_notice("Sorting __ex_table...\n");
    sort_extable(__start___ex_table, __stop___ex_table);
    }
    }
// Given an address, look for it in the kernel exception table
    const
#[no_mangle]
pub unsafe extern "C" fn search_kernel_exception_table() {
    return search_extable(__start___ex_table,
    __stop___ex_table - __start___ex_table, addr);
    }
// Given an address, look for it in the exception tables.
    const struct exception_table_entry *search_exception_tables(unsigned long addr)
    {
    let mut e = core::ptr::null_mut();
    e = search_kernel_exception_table(addr);
    if (!e) {
    e = search_module_extables(addr);
    }
    if (!e) {
    e = search_bpf_extables(addr);
    }
    return e;
    }
#[no_mangle]
pub unsafe extern "C" fn core_kernel_text(addr: c_ulong) -> int notrace {
    if (is_kernel_text(addr)) {
    return 1;
    }
    if (system_state < SYSTEM_FREEING_INITMEM &&
    is_kernel_inittext(addr))
    return 1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __kernel_text_address(addr: c_ulong) -> c_int {
    if (kernel_text_address(addr)) {
    return 1;
    }
//
// There might be init symbols in saved stacktraces.
// Give those symbols a chance to be printed in
// backtraces (such as lockdep traces).
//
// Since we are after the module-symbols check, there's
// no danger of address overlap:
//
    if (is_kernel_inittext(addr)) {
    return 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kernel_text_address(addr: c_ulong) -> c_int {
    let mut no_rcu = 0;
pub static mut ret: c_int = 1;
    if (core_kernel_text(addr)) {
    return 1;
    }
//
// If a stack dump happens while RCU is not watching, then
// RCU needs to be notified that it requires to start
// watching again. This can happen either by tracing that
// triggers a stack trace, or a WARN() that happens during
// coming back from idle, or cpu on or offlining.
//
// is_module_text_address() as well as the kprobe slots,
// is_bpf_text_address() and is_bpf_image_address require
// RCU to be watching.
//
    no_rcu = !rcu_is_watching();
// Treat this like an NMI as it can happen anywhere
    if (no_rcu) {
    ct_nmi_enter();
    }
    if (is_module_text_address(addr)) {
    goto out;
    }
    if (is_ftrace_trampoline(addr)) {
    goto out;
    }
    if (is_kprobe_optinsn_slot(addr) || is_kprobe_insn_slot(addr)) {
    goto out;
    }
    if (is_bpf_text_address(addr)) {
    goto out;
    }
    ret = 0;
    out:
    if (no_rcu) {
    ct_nmi_exit();
    }
    return ret;
    }
//
// On some architectures (PPC64, IA64, PARISC) function pointers
// are actually only tokens to some data that then holds the
// real function address. As a result, to find if a function
// pointer is part of the kernel text, we need to do some
// special dereferencing first.
//

#[no_mangle]
pub unsafe extern "C" fn dereference_function_descriptor() {
    func_desc_t *desc = ptr;
    let mut p = core::ptr::null_mut();
    if (!get_kernel_nofault(p, &desc.addr)) {
    ptr = p;
    }
    return ptr;
    }
// EXPORT_SYMBOL_GPL;
#[no_mangle]
pub unsafe extern "C" fn dereference_kernel_function_descriptor() {
    if (ptr < __start_opd || ptr >= __end_opd) {
    return ptr;
    }
    return dereference_function_descriptor(ptr);
    }

#[no_mangle]
pub unsafe extern "C" fn func_ptr_is_kernel_text(ptr: *mut c_void) -> c_int {
    let mut addr = 0;
    addr = (unsigned long) dereference_function_descriptor(ptr);
    if (core_kernel_text(addr)) {
    return 1;
    }
    return is_module_text_address(addr);
    }