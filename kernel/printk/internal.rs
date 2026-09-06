//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/printk/internal.h
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
// internal.h - printk internal definitions
//

extern "C" {
    pub fn printk_sysctl_init() -> void __init;
}

//
// Identify if legacy printing is forced in a dedicated kthread. If
// true, all printing via console lock occurs within a dedicated
// legacy printer thread. The only exception is on panic, after the
// nbcon consoles have had their chance to print the panic messages
// first.
//

pub const PRINTK_PREFIX_MAX: c_int = 48;

pub const PRINTK_PREFIX_MAX: c_int = 32;

//
// the maximum size of a formatted record (i.e. with prefix added
// per line and dropped messages or in extended message format)
//
pub const PRINTK_MESSAGE_MAX: c_int = 2048;
// the maximum size allowed to be reserved for a record
pub const PRINTKRB_RECORD_MAX: c_int = 1024;
// Flags for a single printk record.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum printk_info_flags {
// always show on console, ignore console_loglevel
    LOG_FORCE_CON	= 1,
    LOG_NEWLINE	= 2,	/* text ended with a newline */
    LOG_CONT	= 8,	/* text is a fragment of a continuation line */
}

extern "C" {
    pub fn __printk_safe_enter();
}
extern "C" {
    pub fn __printk_safe_exit();
}
extern "C" {
    pub fn printk_percpu_data_ready() -> bool;
}

extern "C" {
    pub fn defer_console_output();
}
extern "C" {
    pub fn is_printk_legacy_deferred() -> bool;
}
extern "C" {
    pub fn is_printk_force_console() -> bool;
}
extern "C" {
    pub fn console_lock_spinning_enable();
}
extern "C" {
    pub fn console_lock_spinning_disable_and_check(cookie: c_int) -> c_int;
}
extern "C" {
    pub fn nbcon_seq_read(con: *mut console) -> u64;
}
extern "C" {
    pub fn nbcon_seq_force(con: *mut console, seq: u64);
}
extern "C" {
    pub fn nbcon_alloc(con: *mut console) -> bool;
}
extern "C" {
    pub fn nbcon_free(con: *mut console);
}
extern "C" {
    pub fn nbcon_get_default_prio() -> nbcon_prio;
}
extern "C" {
    pub fn nbcon_atomic_flush_pending();
}
extern "C" {
    pub fn nbcon_kthread_create(con: *mut console) -> bool;
}
extern "C" {
    pub fn nbcon_kthread_stop(con: *mut console);
}
extern "C" {
    pub fn nbcon_kthreads_wake();
}
//
// nbcon_kthread_wake - Wake up a console printing thread
// @con:	Console to operate on
//
// Guarantee any new records can be seen by tasks preparing to wait
// before this context checks if the rcuwait is empty.
//
// The full memory barrier in rcuwait_wake_up() pairs with the full
// memory barrier within set_current_state() of
// ___rcuwait_wait_event(), which is called after prepare_to_rcuwait()
// adds the waiter but before it has checked the wait condition.
//
// This pairs with nbcon_kthread_func:A.
//

pub const PRINTK_PREFIX_MAX: c_int = 0;
pub const PRINTK_MESSAGE_MAX: c_int = 0;
pub const PRINTKRB_RECORD_MAX: c_int = 0;

//
// In !PRINTK builds we still export console_sem
// semaphore and some of console functions (console_unlock()/etc.), so
// printk-safe must preserve the existing local IRQ guarantees.
//

//
// struct console_flush_type - Define available console flush methods
// @nbcon_atomic:	Flush directly using nbcon_atomic() callback
// @nbcon_offload:	Offload flush to printer thread
// @legacy_direct:	Call the legacy loop in this context
// @legacy_offload:	Offload the legacy loop into IRQ or legacy thread
//
// Note that the legacy loop also flushes the nbcon consoles.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct console_flush_type {
    pub nbcon_atomic: bool,
    pub nbcon_offload: bool,
    pub legacy_direct: bool,
    pub legacy_offload: bool,
}

//
// Identify which console flushing methods should be used in the context of
// the caller.
//
// Legacy consoles are flushed directly when possible.
//
// In panic, the nbcon consoles will directly print. But
// only allowed if there are no boot consoles.
//
// This is the same decision as NBCON_PRIO_NORMAL
// except that offloading never occurs in panic.
//
// Note that console_flush_on_panic() will flush
// legacy consoles anyway, even if unsafe.
//
// In panic, if nbcon atomic printing occurs,
// the legacy consoles must remain silent until
// explicitly allowed.
//
// struct printk_buffers - Buffers to read/format/output printk messages.
// @outbuf:	After formatting, contains text to output.
// @scratchbuf:	Used as temporary ringbuffer reading and string-print space.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct printk_buffers {
    pub outbuf: [c_char; PRINTK_MESSAGE_MAX],
    pub scratchbuf: [c_char; PRINTKRB_RECORD_MAX],
}

//
// struct printk_message - Container for a prepared printk message.
// @pbufs:	printk buffers used to prepare the message.
// @outbuf_len:	The length of prepared text in @pbufs->outbuf to output. This
// does not count the terminator. A value of 0 means there is
// nothing to output and this record should be skipped.
// @seq:	The sequence number of the record used for @pbufs->outbuf.
// @dropped:	The number of dropped records from reading @seq.
// @cpu:	CPU on which the message was generated.
// @pid:	PID of the task that generated the message
// @comm:	Name of the task that generated the message.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct printk_message {
    pub pbufs: *mut printk_buffers,
    pub outbuf_len: c_uint,
    pub seq: u64,
    pub dropped: c_ulong,

    pub cpu: c_int,
    pub pid: pid_t,
    pub comm: [c_char; TASK_COMM_LEN],
}

extern "C" {
    pub fn console_prepend_dropped(pmsg: *mut printk_message, dropped: c_ulong);
}
extern "C" {
    pub fn console_prepend_replay(pmsg: *mut printk_message);
}

extern "C" {
    pub fn is_printk_cpu_sync_owner() -> bool;
}

