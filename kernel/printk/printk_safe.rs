//! Automatically rewritten from C to Rust
//! Source: kernel/printk/printk_safe.c
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
// printk_safe.c - Safe printk for printk-deadlock-prone contexts
//

// Context where printk messages are never suppressed
    static atomic_t force_con;
#[no_mangle]
pub unsafe extern "C" fn printk_force_console_enter() {
    void printk_force_console_enter(void)
    {
    atomic_inc(&force_con);
    }
#[no_mangle]
pub unsafe extern "C" fn printk_force_console_exit() {
    void printk_force_console_exit(void)
    {
    atomic_dec(&force_con);
    }
#[no_mangle]
pub unsafe extern "C" fn is_printk_force_console() -> bool {
    bool is_printk_force_console(void)
    {
    return atomic_read(&force_con);
    }
    static DEFINE_PER_CPU(int, printk_context);
// Can be preempted by NMI.
#[no_mangle]
pub unsafe extern "C" fn __printk_safe_enter() {
    void __printk_safe_enter(void)
    {
    this_cpu_inc(printk_context);
    }
// Can be preempted by NMI.
#[no_mangle]
pub unsafe extern "C" fn __printk_safe_exit() {
    void __printk_safe_exit(void)
    {
    this_cpu_dec(printk_context);
    }
#[no_mangle]
pub unsafe extern "C" fn __printk_deferred_enter() {
    void __printk_deferred_enter(void)
    {
    cant_migrate();
    __printk_safe_enter();
    }
#[no_mangle]
pub unsafe extern "C" fn __printk_deferred_exit() {
    void __printk_deferred_exit(void)
    {
    cant_migrate();
    __printk_safe_exit();
    }
#[no_mangle]
pub unsafe extern "C" fn is_printk_legacy_deferred() -> bool {
    bool is_printk_legacy_deferred(void)
    {
//
// The per-CPU variable @printk_context can be read safely in any
// context. CPU migration is always disabled when set.
//
// A context holding the printk_cpu_sync must not spin waiting for
// another CPU. For legacy printing, it could be the console_lock
// or the port lock.
//
    return (force_legacy_kthread() ||
    this_cpu_read(printk_context) ||
    in_nmi() ||
    is_printk_cpu_sync_owner());
    }
#[no_mangle]
pub unsafe extern "C" fn vprintk(fmt: *const c_char, args: va_list) -> asmlinkage int {
    asmlinkage int vprintk(const char *fmt, va_list args)
    {

// Allow to pass printk() to kdb but avoid a recursion.
    if (unlikely(kdb_trap_printk && kdb_printf_cpu < 0))
    return vkdb_printf(KDB_MSGSRC_PRINTK, fmt, args);

    return vprintk_default(fmt, args);
    }
    EXPORT_SYMBOL(vprintk);
