//! Automatically rewritten from C to Rust
//! Source: lib/dump_stack.c
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
// Provide a default dump_stack() function for architectures
// which don't implement their own.
//

    static char dump_stack_arch_desc_str[128];
//
// dump_stack_set_arch_desc - set arch-specific str to show with task dumps
// @fmt: printf-style format string
// @...: arguments for the format string
//
// The configured string will be printed right after utsname during task
// dumps.  Usually used to add arch-specific system identifiers.  If an
// arch wants to make use of such an ID string, it should initialize this
// as soon as possible during boot.
//
#[no_mangle]
pub unsafe extern "C" fn dump_stack_set_arch_desc(fmt: *const c_char, ...) -> void __init {
    void __init dump_stack_set_arch_desc(const char *fmt, ...)
    {
    va_list args;
    va_start(args, fmt);
    vsnprintf(dump_stack_arch_desc_str, sizeof(dump_stack_arch_desc_str),
    fmt, args);
    va_end(args);
    }

//
// dump_stack_print_info - print generic debug info for dump_stack()
// @log_lvl: log level
//
// Arch-specific dump_stack() implementations can use this function to
// print out the same debug information as the generic dump_stack().
//
#[no_mangle]
pub unsafe extern "C" fn dump_stack_print_info(log_lvl: *const c_char) {
    void dump_stack_print_info(const char *log_lvl)
    {
    printk("%sCPU: %d UID: %u PID: %d Comm: %.20s %s%s %s %.*s %s " BUILD_ID_FMT "\n",
    log_lvl, raw_smp_processor_id(),
    __kuid_val(current_real_cred().euid),
    current.pid, current.comm,
    kexec_crash_loaded() ? "Kdump: loaded " : "",
    print_tainted(),
    init_utsname().release,
    (int)strcspn(init_utsname().version, " "),
    init_utsname().version, preempt_model_str(), BUILD_ID_VAL);
    if (get_taint())
    printk("%s%s\n", log_lvl, print_tainted_verbose());
    if (dump_stack_arch_desc_str[0] != '\0')
    printk("%sHardware name: %s\n",
    log_lvl, dump_stack_arch_desc_str);
    print_worker_info(log_lvl, current);
    print_stop_info(log_lvl, current);
    print_scx_info(log_lvl, current);
    }
//
// show_regs_print_info - print generic debug info for show_regs()
// @log_lvl: log level
//
// show_regs() implementations can use this function to print out generic
// debug information.
//
#[no_mangle]
pub unsafe extern "C" fn show_regs_print_info(log_lvl: *const c_char) {
    void show_regs_print_info(const char *log_lvl)
    {
    dump_stack_print_info(log_lvl);
    }
#[no_mangle]
unsafe extern "C" fn __dump_stack(log_lvl: *const c_char) {
    static void __dump_stack(const char *log_lvl)
    {
    dump_stack_print_info(log_lvl);
    show_stack(core::ptr::null_mut(), core::ptr::null_mut(), log_lvl);
    }
//
// dump_stack_lvl - dump the current task information and its stack trace
// @log_lvl: log level
//
// Architectures can override this implementation by implementing its own.
//
#[no_mangle]
pub unsafe extern "C" fn dump_stack_lvl(log_lvl: *const c_char) -> asmlinkage __visible void {
    asmlinkage __visible void dump_stack_lvl(const char *log_lvl)
    {
    let mut in_panic: bool = panic_on_this_cpu();
    unsigned long flags;
//
// Permit this cpu to perform nested stack dumps while serialising
// against other CPUs, unless this CPU is in panic.
//
// When in panic, non-panic CPUs are not permitted to store new
// printk messages so there is no need to synchronize the output.
// This avoids potential deadlock in panic() if another CPU is
// holding and unable to release the printk_cpu_sync.
//
    if (!in_panic)
    printk_cpu_sync_get_irqsave(flags);
    __dump_stack(log_lvl);
    if (!in_panic)
    printk_cpu_sync_put_irqrestore(flags);
    }
    EXPORT_SYMBOL(dump_stack_lvl);
#[no_mangle]
pub unsafe extern "C" fn dump_stack() -> asmlinkage __visible void {
    asmlinkage __visible void dump_stack(void)
    {
    dump_stack_lvl(KERN_DEFAULT);
    }
    EXPORT_SYMBOL(dump_stack);
