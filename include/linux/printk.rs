//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/printk.h
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

pub const PRINTK_MAX_SINGLE_HEADER_LEN: c_int = 2;
// printk's without a loglevel use this..

// We show everything that is MORE important than this..

//
// Default used to be hard-coded at 7, quiet used to be hardcoded at 4,
// we're now allowing both to be set from kernel config.
//

extern "C" {
    pub fn console_verbose();
}
// strlen("ratelimit") + 1
pub const DEVKMSG_STR_MAX_SIZE: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct va_format {
    pub fmt: *const c_char,
    pub va: *mut va_list,
}

//
// FW_BUG
// Add this to a message where you are sure the firmware is buggy or behaves
// really stupid or out of spec. Be aware that the responsible BIOS developer
// should be able to fix this issue or at least get a concrete idea of the
// problem by reading your message without the need of looking at the kernel
// code.
//
// Use it for definite and high priority BIOS bugs.
//
// FW_WARN
// Use it for not that clear (e.g. could the kernel messed up things already?)
// and medium priority BIOS bugs.
//
// FW_INFO
// Use this one if you want to tell the user or vendor about something
// suspicious, but generally harmless related to the firmware.
//
// Use it for information or very low priority BIOS bugs.
//

//
// HW_ERR
// Add this to a message for hardware errors, so that user can report
// it to hardware vendor instead of LKML or software vendor.
//

//
// DEPRECATED
// Add this to a message whenever you want to warn user space about the use
// of a deprecated aspect of an API so they can stop using it
//

//
// Dummy printk for disabled debugging statements to use whilst maintaining
// gcc's format checking.
//

extern "C" {
    pub fn early_printk(fmt: *const c_char, ...);
}

extern "C" {
    pub fn vprintk(fmt: *const c_char, args: va_list) -> c_int;
}
extern "C" {
    pub fn vprintk_deferred(fmt: *const c_char, args: va_list) -> c_int;
}
extern "C" {
    pub fn _printk(fmt: *const c_char, ...) -> c_int;
}
//
// Special printk facility for scheduler/timekeeping use only, _DO_NOT_USE_ !
//
extern "C" {
    pub fn __printk_deferred_enter();
}
extern "C" {
    pub fn __printk_deferred_exit();
}
extern "C" {
    pub fn printk_force_console_enter();
}
extern "C" {
    pub fn printk_force_console_exit();
}
//
// The printk_deferred_enter/exit macros are available only as a hack for
// some code paths that need to defer all printk console printing. Interrupts
// must be disabled for the deferred duration.
//

//
// Please don't use printk_ratelimit(), because it shares ratelimiting state
// with all other unrelated printk_ratelimit() callsites.  Instead use
// printk_ratelimited() or plain old __ratelimit().
//
extern "C" {
    pub fn __printk_ratelimit(func: *const c_char) -> c_int;
}

extern "C" {
    pub fn wake_up_klogd();
}
extern "C" {
    pub fn log_buf_len_get() -> u32;
}
extern "C" {
    pub fn log_buf_vmcoreinfo_setup();
}
extern "C" {
    pub fn setup_log_buf(early: c_int) -> void __init;
}
extern "C" {
    pub fn dump_stack_print_info(log_lvl: *const c_char);
}
extern "C" {
    pub fn show_regs_print_info(log_lvl: *const c_char);
}
extern "C" {
    pub fn printk_trigger_flush();
}
extern "C" {
    pub fn console_try_replay_all();
}
extern "C" {
    pub fn printk_legacy_allow_panic_sync();
}
extern "C" {
    pub fn nbcon_device_try_acquire(con: *mut console) -> bool;
}
extern "C" {
    pub fn nbcon_device_release(con: *mut console);
}
extern "C" {
    pub fn nbcon_atomic_flush_unsafe();
}
extern "C" {
    pub fn pr_flush(timeout_ms: c_int, reset_on_progress: bool) -> bool;
}

extern "C" {
    pub fn __printk_cpu_sync_try_get() -> c_int;
}
extern "C" {
    pub fn __printk_cpu_sync_wait();
}
extern "C" {
    pub fn __printk_cpu_sync_put();
}

// Macro flag: #define __printk_cpu_sync_wait()
// Macro flag: #define __printk_cpu_sync_put()

//
// printk_cpu_sync_get_irqsave() - Disable interrupts and acquire the printk
// cpu-reentrant spinning lock.
// @flags: Stack-allocated storage for saving local interrupt state,
// to be passed to printk_cpu_sync_put_irqrestore().
//
// If the lock is owned by another CPU, spin until it becomes available.
// Interrupts are restored while spinning.
//
// CAUTION: This function must be used carefully. It does not behave like a
// typical lock. Here are important things to watch out for...
//
// * This function is reentrant on the same CPU. Therefore the calling
// code must not assume exclusive access to data if code accessing the
// data can run reentrant or within NMI context on the same CPU.
//
// * If there exists usage of this function from NMI context, it becomes
// unsafe to perform any type of locking or spinning to wait for other
// CPUs after calling this function from any context. This includes
// using spinlocks or any other busy-waiting synchronization methods.
//

//
// printk_cpu_sync_put_irqrestore() - Release the printk cpu-reentrant spinning
// lock and restore interrupts.
// @flags: Caller's saved interrupt state, from printk_cpu_sync_get_irqsave().
//

//
// pr_fmt - used by the pr_*() macros to generate the printk format string
// @fmt: format string passed from a pr_*() macro
//
// This macro can be used to generate a unified format string for pr_*()
// macros. A common use is to prefix all pr_*() messages in a file with a common
// string. For example, defining this at the top of a source file:
//
// #define pr_fmt(fmt) KBUILD_MODNAME ": " fmt
//
// would prefix all pr_info, pr_emerg... messages in the file with the module
// name.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pi_entry {
    pub fmt: *const c_char,
    pub func: *const c_char,
    pub file: *const c_char,
    pub line: c_uint,
//
// While printk and pr_* have the level stored in the string at compile
// time, some subsystems dynamically add it at runtime through the
// format string. For these dynamic cases, we allow the subsystem to
// tell us the level at compile time.
//
// NULL indicates that the level, if any, is stored in fmt.
//
    pub level: *const c_char,
//
// The format string used by various subsystem specific printk()
// wrappers to prefix the message.
//
// Note that the static prefix defined by the pr_fmt() macro is stored
// directly in the message format (@fmt), not here.
//
    pub subsys_fmt_prefix: *const c_char,
    pub __packed: },

//
// We check __builtin_constant_p multiple times here
// for the same input because GCC will produce an error
// if we try to assign a static variable to fmt if it
// is not a constant, even with the outer if statement.
// \
}

//
// Some subsystems have their own custom printk that applies a va_format to a
// generic format, for example, to include a device number or other metadata
// alongside the format supplied by the caller.
//
// In order to store these in the way they would be emitted by the printk
// infrastructure, the subsystem provides us with the start, fixed string, and
// any subsequent text in the format string.
//
// We take a variable argument list as pr_fmt/dev_fmt/etc are sometimes passed
// as multiple arguments (eg: `"%s: ", "blah"`), and we must only take the
// first one.
//
// subsys_fmt_prefix must be known at compile time, or compilation will fail
// (since this is a mistake). If fmt or level is not known at compile time, no
// index entry will be made (since this can legitimately happen).
//

//
// printk - print a kernel message
// @fmt: format string
//
// This is printk(). It can be called from any context. We want it to work.
//
// If printk indexing is enabled, _printk() is called from printk_index_wrap.
// Otherwise, printk is simply #defined to _printk.
//
// We try to grab the console_lock. If we succeed, it's easy - we log the
// output and call the console drivers.  If we fail to get the semaphore, we
// place the output into the log buffer and return. The current holder of
// the console_sem will notice the new output in console_unlock(); and will
// send it to the consoles before releasing the lock.
//
// One effect of this deferred printing is that code which calls printk() and
// then changes console_loglevel may break. This is because console_loglevel
// is inspected when the actual printing occurs.
//
// See also:
// printf(3)
//
// See the vsnprintf() documentation for format string extensions over C99.
//

//
// pr_emerg - Print an emergency-level message
// @fmt: format string
// @...: arguments for the format string
//
// This macro expands to a printk with KERN_EMERG loglevel. It uses pr_fmt() to
// generate the format string.
//

//
// pr_alert - Print an alert-level message
// @fmt: format string
// @...: arguments for the format string
//
// This macro expands to a printk with KERN_ALERT loglevel. It uses pr_fmt() to
// generate the format string.
//

//
// pr_crit - Print a critical-level message
// @fmt: format string
// @...: arguments for the format string
//
// This macro expands to a printk with KERN_CRIT loglevel. It uses pr_fmt() to
// generate the format string.
//

//
// pr_err - Print an error-level message
// @fmt: format string
// @...: arguments for the format string
//
// This macro expands to a printk with KERN_ERR loglevel. It uses pr_fmt() to
// generate the format string.
//

//
// pr_warn - Print a warning-level message
// @fmt: format string
// @...: arguments for the format string
//
// This macro expands to a printk with KERN_WARNING loglevel. It uses pr_fmt()
// to generate the format string.
//

//
// pr_notice - Print a notice-level message
// @fmt: format string
// @...: arguments for the format string
//
// This macro expands to a printk with KERN_NOTICE loglevel. It uses pr_fmt() to
// generate the format string.
//

//
// pr_info - Print an info-level message
// @fmt: format string
// @...: arguments for the format string
//
// This macro expands to a printk with KERN_INFO loglevel. It uses pr_fmt() to
// generate the format string.
//

//
// pr_cont - Continues a previous log message in the same line.
// @fmt: format string
// @...: arguments for the format string
//
// This macro expands to a printk with KERN_CONT loglevel. It should only be
// used when continuing a log message with no newline ('\n') enclosed. Otherwise
// it defaults back to KERN_DEFAULT loglevel.
//

//
// pr_devel - Print a debug-level message conditionally
// @fmt: format string
// @...: arguments for the format string
//
// This macro expands to a printk with KERN_DEBUG loglevel if DEBUG is
// defined. Otherwise it does nothing.
//
// It uses pr_fmt() to generate the format string.
//

// If you are writing a driver, please use dev_dbg instead

//
// pr_debug - Print a debug-level message conditionally
// @fmt: format string
// @...: arguments for the format string
//
// This macro expands to dynamic_pr_debug() if CONFIG_DYNAMIC_DEBUG is
// set. Otherwise, if DEBUG is defined, it's equivalent to a printk with
// KERN_DEBUG loglevel. If DEBUG is not defined it does nothing.
//
// It uses pr_fmt() to generate the format string (dynamic_pr_debug() uses
// pr_fmt() internally).
//

//
// Print a one-time message (analogous to WARN_ONCE() et al):
//

// no pr_cont_once, don't do that...

// If you are writing a driver, please use dev_dbg instead

//
// ratelimited messages with local ratelimit_state,
// no local ratelimit_state used in the !PRINTK case
//

// no pr_cont_ratelimited, don't do that...

// If you are writing a driver, please use dev_dbg instead

// descriptor check is first to prevent flooding with "callbacks suppressed"

//
// print_hex_dump_bytes - shorthand form of print_hex_dump_debug() with default
// params
// @prefix_str: string to prefix each line with;
// caller supplies trailing spaces for alignment if desired
// @prefix_type: controls whether prefix of an offset, address, or none
// is printed (%DUMP_PREFIX_OFFSET, %DUMP_PREFIX_ADDRESS, %DUMP_PREFIX_NONE)
// @buf: data blob to dump
// @len: number of bytes in the @buf
//
// Calls print_hex_dump_debug(), with log level of KERN_DEBUG,
// rowsize of 16, groupsize of 1, and ASCII output included.
//

