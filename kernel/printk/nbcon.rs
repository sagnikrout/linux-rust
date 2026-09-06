//! Automatically rewritten from C to Rust
//! Source: kernel/printk/nbcon.c
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
// === KERNEL_MACRO_PRELUDE_START ===
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
macro_rules! ARRAY_SIZE { ($($tt:tt)*) => { 1 }; }
macro_rules! container_of { ($($tt:tt)*) => { core::ptr::null_mut() }; }
macro_rules! sizeof { ($($tt:tt)*) => { 0usize }; }
macro_rules! IS_ENABLED { ($($tt:tt)*) => { false }; }
macro_rules! DECLARE_WORK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_WAKE_Q { ($($tt:tt)*) => {}; }
macro_rules! LLIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! LIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! SET_UID { ($($tt:tt)*) => {}; }
macro_rules! SET_GID { ($($tt:tt)*) => {}; }
macro_rules! list_for_each_entry { ($($tt:tt)*) => { if false }; }
macro_rules! list_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! llist_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! pr_info_once { ($($tt:tt)*) => {}; }
macro_rules! pr_info { ($($tt:tt)*) => {}; }
macro_rules! pr_warn { ($($tt:tt)*) => {}; }
macro_rules! pr_err { ($($tt:tt)*) => {}; }
macro_rules! pr_debug { ($($tt:tt)*) => {}; }
macro_rules! early_param { ($($tt:tt)*) => {}; }
macro_rules! BUILD_BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! WARN_ON { ($($tt:tt)*) => { false }; }
macro_rules! WARN_ON_ONCE { ($($tt:tt)*) => { false }; }
macro_rules! BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! BUG { () => {}; }
macro_rules! IS_ERR { ($($tt:tt)*) => { false }; }
macro_rules! PTR_ERR { ($($tt:tt)*) => { 0 }; }
macro_rules! ERR_PTR { ($($tt:tt)*) => { core::ptr::null_mut() }; }

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
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kern_ipc_perm { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_params { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_queue { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msgseg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_sender { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_receiver { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sembuf { pub sem_num: u16, pub sem_op: i16, pub sem_flg: i16 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem_array { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmid_kernel { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shm_file_data { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wake_q_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct work_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct llist_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;
pub type key_t = i32;
pub type kuid_t = u32;
pub type kgid_t = u32;
pub type int = c_int;
pub type uint = c_uint;
pub type ulong = c_ulong;
pub type long = c_long;
pub type void = c_void;

// Standard Linux Error Codes
pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const ESRCH: c_int = 3;
pub const EINTR: c_int = 4;
pub const EIO: c_int = 5;
pub const ENXIO: c_int = 6;
pub const E2BIG: c_int = 7;
pub const ENOEXEC: c_int = 8;
pub const EBADF: c_int = 9;
pub const ECHILD: c_int = 10;
pub const EAGAIN: c_int = 11;
pub const ENOMEM: c_int = 12;
pub const EACCES: c_int = 13;
pub const EFAULT: c_int = 14;
pub const EBUSY: c_int = 16;
pub const EEXIST: c_int = 17;
pub const EXDEV: c_int = 18;
pub const ENODEV: c_int = 19;
pub const ENOTDIR: c_int = 20;
pub const EISDIR: c_int = 21;
pub const EINVAL: c_int = 22;
pub const ENFILE: c_int = 23;
pub const EMFILE: c_int = 24;
pub const ENOSPC: c_int = 28;
pub const EROFS: c_int = 30;
pub const EIDRM: c_int = 43;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUPP: c_int = 524;

// Standard Memory Constants
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const GFP_KERNEL: c_uint = 0xcc0;
pub const GFP_ATOMIC: c_uint = 0x80000;
pub const GFP_NOWAIT: c_uint = 0;

// Standard Core Primitives
extern "C" {
    pub static current: *mut task_struct;
    pub fn printk(fmt: *const c_char, ...) -> c_int;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
}
// === KERNEL_MACRO_PRELUDE_END ===


// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2022 Linutronix GmbH, John Ogness
// Copyright (C) 2022 Intel, Thomas Gleixner

//
// Printk console printing implementation for consoles which does not depend
// on the legacy style console_lock mechanism.
//
// The state of the console is maintained in the "nbcon_state" atomic
// variable.
//
// The console is locked when:
//
// - The 'prio' field contains the priority of the context that owns the
// console. Only higher priority contexts are allowed to take over the
// lock. A value of 0 (NBCON_PRIO_NONE) means the console is not locked.
//
// - The 'cpu' field denotes on which CPU the console is locked. It is used
// to prevent busy waiting on the same CPU. Also it informs the lock owner
// that it has lost the lock in a more complex scenario when the lock was
// taken over by a higher priority context, released, and taken on another
// CPU with the same priority as the interrupted owner.
//
// The acquire mechanism uses a few more fields:
//
// - The 'req_prio' field is used by the handover approach to make the
// current owner aware that there is a context with a higher priority
// waiting for the friendly handover.
//
// - The 'unsafe' field allows to take over the console in a safe way in the
// middle of emitting a message. The field is set only when accessing some
// shared resources or when the console device is manipulated. It can be
// cleared, for example, after emitting one character when the console
// device is in a consistent state.
//
// - The 'unsafe_takeover' field is set when a hostile takeover took the
// console in an unsafe state. The console will stay in the unsafe state
// until re-initialized.
//
// The acquire mechanism uses three approaches:
//
// 1) Direct acquire when the console is not owned or is owned by a lower
// priority context and is in a safe state.
//
// 2) Friendly handover mechanism uses a request/grant handshake. It is used
// when the current owner has lower priority and the console is in an
// unsafe state.
//
// The requesting context:
//
// a) Sets its priority into the 'req_prio' field.
//
// b) Waits (with a timeout) for the owning context to unlock the
// console.
//
// c) Takes the lock and clears the 'req_prio' field.
//
// The owning context:
//
// a) Observes the 'req_prio' field set on exit from the unsafe
// console state.
//
// b) Gives up console ownership by clearing the 'prio' field.
//
// 3) Unsafe hostile takeover allows to take over the lock even when the
// console is an unsafe state. It is used only in panic() by the final
// attempt to flush consoles in a try and hope mode.
//
// Note that separate record buffers are used in panic(). As a result,
// the messages can be read and formatted without any risk even after
// using the hostile takeover in unsafe state.
//
// The release function simply clears the 'prio' field.
//
// All operations on @console::nbcon_state are atomic cmpxchg based to
// handle concurrency.
//
// The acquire/release functions implement only minimal policies:
//
// - Preference for higher priority contexts.
// - Protection of the panic CPU.
//
// All other policy decisions must be made at the call sites:
//
// - What is marked as an unsafe section.
// - Whether to spin-wait if there is already an owner and the console is
// in an unsafe state.
// - Whether to attempt an unsafe hostile takeover.
//
// The design allows to implement the well known:
//
// acquire()
// output_one_printk_record()
// release()
//
// The output of one printk record might be interrupted with a higher priority
// context. The new owner is supposed to reprint the entire interrupted record
// from scratch.
//
// Counter of active nbcon emergency contexts.
pub static mut nbcon_cpu_emergency_cnt: atomic_t = 0;
//
// nbcon_state_set - Helper function to set the console state
// @con:	Console to update
// @new:	The new state to write
//
// Only to be used when the console is not yet or no longer visible in the
// system. Otherwise use nbcon_state_try_cmpxchg().
//
#[no_mangle]
pub unsafe extern "C" fn nbcon_state_set(con: *mut console, new: *mut nbcon_state) {
    atomic_set(&ACCESS_PRIVATE(con, nbcon_state), new.atom);
    }
//
// nbcon_state_read - Helper function to read the console state
// @con:	Console to read
// @state:	The state to store the result
//
#[no_mangle]
pub unsafe extern "C" fn nbcon_state_read(con: *mut console, state: *mut nbcon_state) {
    state.atom = atomic_read(&ACCESS_PRIVATE(con, nbcon_state));
    }
//
// nbcon_state_try_cmpxchg() - Helper function for atomic_try_cmpxchg() on console state
// @con:	Console to update
// @cur:	Old/expected state
// @new:	New state
//
// Return: True on success. False on fail and @cur is updated.
//
#[no_mangle]
pub unsafe extern "C" fn nbcon_state_try_cmpxchg(con: *mut console, cur: *mut nbcon_state, new: *mut nbcon_state) -> bool {
    return atomic_try_cmpxchg(&ACCESS_PRIVATE(con, nbcon_state), &cur.atom, new.atom);
    }
//
// nbcon_seq_read - Read the current console sequence
// @con:	Console to read the sequence of
//
// Return:	Sequence number of the next record to print on @con.
//
#[no_mangle]
pub unsafe extern "C" fn nbcon_seq_read(con: *mut console) -> u64 {
pub static mut nbcon_seq: c_ulong = 0;
    return __ulseq_to_u64seq(prb, nbcon_seq);
    }
//
// nbcon_seq_force - Force console sequence to a specific value
// @con:	Console to work on
// @seq:	Sequence number value to set
//
// Only to be used during init (before registration) or in extreme situations
// (such as panic with CONSOLE_REPLAY_ALL).
//
#[no_mangle]
pub unsafe extern "C" fn nbcon_seq_force(con: *mut console, seq: u64) {
//
// If the specified record no longer exists, the oldest available record
// is chosen. This is especially important on 32bit systems because only
// the lower 32 bits of the sequence number are stored. The upper 32 bits
// are derived from the sequence numbers available in the ringbuffer.
//
pub static mut valid_seq: u64 = 0;
    atomic_long_set(&ACCESS_PRIVATE(con, nbcon_seq), __u64seq_to_ulseq(valid_seq));
    }
//
// nbcon_seq_try_update - Try to update the console sequence number
// @ctxt:	Pointer to an acquire context that contains
// all information about the acquire mode
// @new_seq:	The new sequence number to set
//
// @ctxt->seq is updated to the new value of @con::nbcon_seq (expanded to
// the 64bit value). This could be a different value than @new_seq if
// nbcon_seq_force() was used or the current context no longer owns the
// console. In the later case, it will stop printing anyway.
//
#[no_mangle]
unsafe extern "C" fn nbcon_seq_try_update(ctxt: *mut nbcon_context, new_seq: u64) {
pub static mut nbcon_seq: c_ulong = 0;
    let mut con = ctxt.console;
    if (atomic_long_try_cmpxchg(&ACCESS_PRIVATE(con, nbcon_seq), &nbcon_seq,
    __u64seq_to_ulseq(new_seq))) {
    ctxt.seq = new_seq;
    } else {
    ctxt.seq = nbcon_seq_read(con);
    }
    }
//
// nbcon_context_try_acquire_direct - Try to acquire directly
// @ctxt:		The context of the caller
// @cur:		The current console state
// @is_reacquire:	This acquire is a reacquire
//
// Acquire the console when it is released. Also acquire the console when
// the current owner has a lower priority and the console is in a safe state.
//
// Return:	0 on success. Otherwise, an error code on failure. Also @cur
// is updated to the latest state when failed to modify it.
//
// Errors:
//
// -EPERM:		A panic is in progress and this is neither the panic
// CPU nor is this a reacquire. Or the current owner or
// waiter has the same or higher priority. No acquire
// method can be successful in these cases.
//
// -EBUSY:		The current owner has a lower priority but the console
// in an unsafe state. The caller should try using
// the handover acquire method.
//
#[no_mangle]
pub unsafe extern "C" fn nbcon_context_try_acquire_direct(ctxt: *mut nbcon_context, cur: *mut nbcon_state, is_reacquire: bool) -> c_int {
pub static mut cpu: c_uint = 0;
    let mut con = ctxt.console;
pub static mut new: usize = 0;
    do {
//
// Panic does not imply that the console is owned. However,
// since all non-panic CPUs are stopped during panic(), it
// is safer to have them avoid gaining console ownership.
//
// One exception is when kdb has locked for printing on this CPU.
//
// Second exception is a reacquire (and an unsafe takeover
// has not previously occurred) then it is allowed to attempt
// a direct acquire in panic. This gives console drivers an
// opportunity to perform any necessary cleanup if they were
// interrupted by the panic CPU while printing.
//
    if (panic_on_other_cpu() &&
    !kdb_printf_on_this_cpu() &&
    (!is_reacquire || cur.unsafe_takeover)) {
    return -EPERM;
    }
    if (ctxt.prio <= cur.prio || ctxt.prio <= cur.req_prio) {
    return -EPERM;
    }
    if (cur.unsafe) {
    return -EBUSY;
    }
//
// The console should never be safe for a direct acquire
// if an unsafe hostile takeover has ever happened.
//
    WARN_ON_ONCE!(cur.unsafe_takeover);
    new.atom = cur.atom;
    new.prio	= ctxt.prio;
    new.req_prio	= NBCON_PRIO_NONE;
    new.unsafe	= cur.unsafe_takeover;
    new.cpu		= cpu;
    } while (!nbcon_state_try_cmpxchg(con, cur, &new));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nbcon_waiter_matches(cur: *mut nbcon_state, expected_prio: c_int) -> bool {
//
// The request context is well defined by the @req_prio because:
//
// - Only a context with a priority higher than the owner can become
// a waiter.
// - Only a context with a priority higher than the waiter can
// directly take over the request.
// - There are only three priorities.
// - Only one CPU is allowed to request PANIC priority.
// - Lower priorities are ignored during panic() until reboot.
//
// As a result, the following scenario is *not* possible:
//
// 1. This context is currently a waiter.
// 2. Another context with a higher priority than this context
// directly takes ownership.
// 3. The higher priority context releases the ownership.
// 4. Another lower priority context takes the ownership.
// 5. Another context with the same priority as this context
// creates a request and starts waiting.
//
// Event #1 implies this context is EMERGENCY.
// Event #2 implies the new context is PANIC.
// Event #3 occurs when panic() has flushed the console.
// Event #4 occurs when a non-panic CPU reacquires.
// Event #5 is not possible due to the panic_on_other_cpu() check
// in nbcon_context_try_acquire_handover().
//
    return (cur.req_prio == expected_prio);
    }
//
// nbcon_context_try_acquire_requested - Try to acquire after having
// requested a handover
// @ctxt:	The context of the caller
// @cur:	The current console state
//
// This is a helper function for nbcon_context_try_acquire_handover().
// It is called when the console is in an unsafe state. The current
// owner will release the console on exit from the unsafe region.
//
// Return:	0 on success and @cur is updated to the new console state.
// Otherwise an error code on failure.
//
// Errors:
//
// -EPERM:		A panic is in progress and this is not the panic CPU
// or this context is no longer the waiter.
//
// -EBUSY:		The console is still locked. The caller should
// continue waiting.
//
// Note: The caller must still remove the request when an error has occurred
// except when this context is no longer the waiter.
//
#[no_mangle]
pub unsafe extern "C" fn nbcon_context_try_acquire_requested(ctxt: *mut nbcon_context, cur: *mut nbcon_state) -> c_int {
pub static mut cpu: c_uint = 0;
    let mut con = ctxt.console;
pub static mut new: usize = 0;
// Note that the caller must still remove the request!
    if (panic_on_other_cpu()) {
    return -EPERM;
    }
//
// Note that the waiter will also change if there was an unsafe
// hostile takeover.
//
    if (!nbcon_waiter_matches(cur, ctxt.prio)) {
    return -EPERM;
    }
// If still locked, caller should continue waiting.
    if (cur.prio != NBCON_PRIO_NONE) {
    return -EBUSY;
    }
//
// The previous owner should have never released ownership
// in an unsafe region.
//
    WARN_ON_ONCE!(cur.unsafe);
    new.atom = cur.atom;
    new.prio	= ctxt.prio;
    new.req_prio	= NBCON_PRIO_NONE;
    new.unsafe	= cur.unsafe_takeover;
    new.cpu		= cpu;
    if (!nbcon_state_try_cmpxchg(con, cur, &new)) {
//
// The acquire could fail only when it has been taken
// over by a higher priority context.
//
    WARN_ON_ONCE!(nbcon_waiter_matches(cur, ctxt.prio));
    return -EPERM;
    }
// Handover success. This context now owns the console.
    return 0;
    }
//
// nbcon_context_try_acquire_handover - Try to acquire via handover
// @ctxt:	The context of the caller
// @cur:	The current console state
//
// The function must be called only when the context has higher priority
// than the current owner and the console is in an unsafe state.
// It is the case when nbcon_context_try_acquire_direct() returns -EBUSY.
//
// The function sets "req_prio" field to make the current owner aware of
// the request. Then it waits until the current owner releases the console,
// or an even higher context takes over the request, or timeout expires.
//
// The current owner checks the "req_prio" field on exit from the unsafe
// region and releases the console. It does not touch the "req_prio" field
// so that the console stays reserved for the waiter.
//
// Return:	0 on success. Otherwise, an error code on failure. Also @cur
// is updated to the latest state when failed to modify it.
//
// Errors:
//
// -EPERM:		A panic is in progress and this is not the panic CPU.
// Or a higher priority context has taken over the
// console or the handover request.
//
// -EBUSY:		The current owner is on the same CPU so that the hand
// shake could not work. Or the current owner is not
// willing to wait (zero timeout). Or the console does
// not enter the safe state before timeout passed. The
// caller might still use the unsafe hostile takeover
// when allowed.
//
// -EAGAIN:	@cur has changed when creating the handover request.
// The caller should retry with direct acquire.
//
#[no_mangle]
pub unsafe extern "C" fn nbcon_context_try_acquire_handover(ctxt: *mut nbcon_context, cur: *mut nbcon_state) -> c_int {
pub static mut cpu: c_uint = 0;
    let mut con = ctxt.console;
pub static mut new: usize = 0;
    let mut timeout = 0;
pub static mut request_err: c_int = 0;
//
// Check that the handover is called when the direct acquire failed
// with -EBUSY.
//
    WARN_ON_ONCE!(ctxt.prio <= cur.prio || ctxt.prio <= cur.req_prio);
    WARN_ON_ONCE!(!cur.unsafe);
//
// Panic does not imply that the console is owned. However, it
// is critical that non-panic CPUs during panic are unable to
// wait for a handover in order to satisfy the assumptions of
// nbcon_waiter_matches(). In particular, the assumption that
// lower priorities are ignored during panic.
//
    if (panic_on_other_cpu()) {
    return -EPERM;
    }
// Handover is not possible on the same CPU.
    if (cur.cpu == cpu) {
    return -EBUSY;
    }
//
// Console stays unsafe after an unsafe takeover until re-initialized.
// Waiting is not going to help in this case.
//
    if (cur.unsafe_takeover) {
    return -EBUSY;
    }
// Is the caller willing to wait?
    if (ctxt.spinwait_max_us == 0) {
    return -EBUSY;
    }
//
// Setup a request for the handover. The caller should try to acquire
// the console directly when the current state has been modified.
//
    new.atom = cur.atom;
    new.req_prio = ctxt.prio;
    if (!nbcon_state_try_cmpxchg(con, cur, &new)) {
    return -EAGAIN;
    }
    cur.atom = new.atom;
// Wait until there is no owner and then acquire the console.
    while (timeout >= 0) {
// On successful acquire, this request is cleared.
    request_err = nbcon_context_try_acquire_requested(ctxt, cur);
    if (!request_err) {
    return 0;
    }
//
// If the acquire should be aborted, it must be ensured
// that the request is removed before returning to caller.
//
    if (request_err == -EPERM) {
    break;
    }
    udelay(1);
// Re-read the state because some time has passed.
    nbcon_state_read(con, cur);
    }
// Timed out or aborted. Carefully remove handover request.
    do {
//
// No need to remove request if there is a new waiter. This
// can only happen if a higher priority context has taken over
// the console or the handover request.
//
    if (!nbcon_waiter_matches(cur, ctxt.prio)) {
    return -EPERM;
    }
// Unset request for handover.
    new.atom = cur.atom;
    new.req_prio = NBCON_PRIO_NONE;
    if (nbcon_state_try_cmpxchg(con, cur, &new)) {
//
// Request successfully unset. Report failure of
// acquiring via handover.
//
    cur.atom = new.atom;
    return request_err;
    }
//
// Unable to remove request. Try to acquire in case
// the owner has released the lock.
//
    } while (nbcon_context_try_acquire_requested(ctxt, cur));
// Lucky timing. The acquire succeeded while removing the request.
    return 0;
    }
//
// nbcon_context_try_acquire_hostile - Acquire via unsafe hostile takeover
// @ctxt:	The context of the caller
// @cur:	The current console state
//
// Acquire the console even in the unsafe state.
//
// It can be permitted by setting the 'allow_unsafe_takeover' field only
// by the final attempt to flush messages in panic().
//
// Return:	0 on success. -EPERM when not allowed by the context.
//
#[no_mangle]
pub unsafe extern "C" fn nbcon_context_try_acquire_hostile(ctxt: *mut nbcon_context, cur: *mut nbcon_state) -> c_int {
pub static mut cpu: c_uint = 0;
    let mut con = ctxt.console;
pub static mut new: usize = 0;
    if (!ctxt.allow_unsafe_takeover) {
    return -EPERM;
    }
// Ensure caller is allowed to perform unsafe hostile takeovers.
    if (WARN_ON_ONCE!(ctxt.prio != NBCON_PRIO_PANIC)) {
    return -EPERM;
    }
//
// Check that try_acquire_direct() and try_acquire_handover() returned
// -EBUSY in the right situation.
//
    WARN_ON_ONCE!(ctxt.prio <= cur.prio || ctxt.prio <= cur.req_prio);
    WARN_ON_ONCE!(cur.unsafe != true);
    do {
    new.atom = cur.atom;
    new.cpu			= cpu;
    new.prio		= ctxt.prio;
    new.unsafe		|= cur.unsafe_takeover;
    new.unsafe_takeover	|= cur.unsafe;
    } while (!nbcon_state_try_cmpxchg(con, cur, &new));
    return 0;
    }
pub static mut panic_nbcon_pbufs: usize = 0;
//
// nbcon_context_try_acquire - Try to acquire nbcon console
// @ctxt:		The context of the caller
// @is_reacquire:	This acquire is a reacquire
//
// Context:	Under @ctxt->con->device_lock() or local_irq_save().
// Return:	True if the console was acquired. False otherwise.
//
// If the caller allowed an unsafe hostile takeover, on success the
// caller should check the current console state to see if it is
// in an unsafe state. Otherwise, on success the caller may assume
// the console is not in an unsafe state.
//
#[no_mangle]
unsafe extern "C" fn nbcon_context_try_acquire(ctxt: *mut nbcon_context, is_reacquire: bool) -> bool {
    let mut con = ctxt.console;
pub static mut cur: usize = 0;
    let mut err = 0;
    nbcon_state_read(con, &cur);
// label;
    err = nbcon_context_try_acquire_direct(ctxt, &cur, is_reacquire);
    if (err != -EBUSY) {
// goto;
    }
    err = nbcon_context_try_acquire_handover(ctxt, &cur);
    if (err == -EAGAIN) {
// goto;
    }
    if (err != -EBUSY) {
// goto;
    }
    err = nbcon_context_try_acquire_hostile(ctxt, &cur);
// label;
    if (err) {
    return false;
    }
// Acquire succeeded.
// Assign the appropriate buffer for this context.
    if (panic_on_this_cpu()) {
    ctxt.pbufs = &panic_nbcon_pbufs;
    }
    else {
    ctxt.pbufs = con.pbufs;
    }
// Set the record sequence for this context to print.
    ctxt.seq = nbcon_seq_read(ctxt.console);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn nbcon_owner_matches(cur: *mut nbcon_state, expected_cpu: c_int, expected_prio: c_int) -> bool {
//
// A similar function, nbcon_waiter_matches(), only deals with
// EMERGENCY and PANIC priorities. However, this function must also
// deal with the NORMAL priority, which requires additional checks
// and constraints.
//
// For the case where preemption and interrupts are disabled, it is
// enough to also verify that the owning CPU has not changed.
//
// For the case where preemption or interrupts are enabled, an
// external synchronization method *must* be used. In particular,
the driver-specific locking mechanism used in device_lock()
// (including disabling migration) should be used. It prevents
// scenarios such as:
//
// 1. [Task A] owns a context with NBCON_PRIO_NORMAL on [CPU X] and
// is scheduled out.
// 2. Another context takes over the lock with NBCON_PRIO_EMERGENCY
// and releases it.
// 3. [Task B] acquires a context with NBCON_PRIO_NORMAL on [CPU X]
// and is scheduled out.
// 4. [Task A] gets running on [CPU X] and sees that the console is
// still owned by a task on [CPU X] with NBON_PRIO_NORMAL. Thus
// [Task A] thinks it is the owner when it is not.
//
    if (cur.prio != expected_prio) {
    return false;
    }
    if (cur.cpu != expected_cpu) {
    return false;
    }
    return true;
    }
//
// nbcon_context_release - Release the console
// @ctxt:	The nbcon context from nbcon_context_try_acquire()
//
#[no_mangle]
unsafe extern "C" fn nbcon_context_release(ctxt: *mut nbcon_context) {
pub static mut cpu: c_uint = 0;
    let mut con = ctxt.console;
pub static mut cur: usize = 0;
pub static mut new: usize = 0;
    nbcon_state_read(con, &cur);
    do {
    if (!nbcon_owner_matches(&cur, cpu, ctxt.prio)) {
    break;
    }
    new.atom = cur.atom;
    new.prio = NBCON_PRIO_NONE;
//
// If @unsafe_takeover is set, it is kept set so that
// the state remains permanently unsafe.
//
    new.unsafe |= cur.unsafe_takeover;
    } while (!nbcon_state_try_cmpxchg(con, &cur, &new));
    ctxt.pbufs = core::ptr::null_mut();
    }
//
// nbcon_context_can_proceed - Check whether ownership can proceed
// @ctxt:	The nbcon context from nbcon_context_try_acquire()
// @cur:	The current console state
//
// Return:	True if this context still owns the console. False if
// ownership was handed over or taken.
//
// Must be invoked when entering the unsafe state to make sure that it still
// owns the lock. Also must be invoked when exiting the unsafe context
// to eventually free the lock for a higher priority context which asked
// for the friendly handover.
//
// It can be called inside an unsafe section when the console is just
// temporary in safe state instead of exiting and entering the unsafe
// state.
//
// Also it can be called in the safe context before doing an expensive
// safe operation. It does not make sense to do the operation when
// a higher priority context took the lock.
//
// When this function returns false then the calling context no longer owns
// the console and is no longer allowed to go forward. In this case it must
// back out immediately and carefully. The buffer content is also no longer
// trusted since it no longer belongs to the calling context.
//
#[no_mangle]
unsafe extern "C" fn nbcon_context_can_proceed(ctxt: *mut nbcon_context, cur: *mut nbcon_state) -> bool {
pub static mut cpu: c_uint = 0;
// Make sure this context still owns the console.
    if (!nbcon_owner_matches(cur, cpu, ctxt.prio)) {
    return false;
    }
// The console owner can proceed if there is no waiter.
    if (cur.req_prio == NBCON_PRIO_NONE) {
    return true;
    }
//
// A console owner within an unsafe region is always allowed to
// proceed, even if there are waiters. It can perform a handover
// when exiting the unsafe region. Otherwise the waiter will
// need to perform an unsafe hostile takeover.
//
    if (cur.unsafe) {
    return true;
    }
// Waiters always have higher priorities than owners.
    WARN_ON_ONCE!(cur.req_prio <= cur.prio);
//
// Having a safe point for take over and eventually a few
// duplicated characters or a full line is way better than a
// hostile takeover. Post processing can take care of the garbage.
// Release and hand over.
//
    nbcon_context_release(ctxt);
//
// It is not clear whether the waiter really took over ownership. The
// outermost callsite must make the final decision whether console
// ownership is needed for it to proceed. If yes, it must reacquire
// ownership (possibly hostile) before carefully proceeding.
//
// The calling context no longer owns the console so go back all the
// way instead of trying to implement reacquire heuristics in tons of
// places.
//
    return false;
    }
//
// nbcon_can_proceed - Check whether ownership can proceed
// @wctxt:	The write context that was handed to the write function
//
// Return:	True if this context still owns the console. False if
// ownership was handed over or taken.
//
// It is used in nbcon_enter_unsafe() to make sure that it still owns the
// lock. Also it is used in nbcon_exit_unsafe() to eventually free the lock
// for a higher priority context which asked for the friendly handover.
//
// It can be called inside an unsafe section when the console is just
// temporary in safe state instead of exiting and entering the unsafe state.
//
// Also it can be called in the safe context before doing an expensive safe
// operation. It does not make sense to do the operation when a higher
// priority context took the lock.
//
// When this function returns false then the calling context no longer owns
// the console and is no longer allowed to go forward. In this case it must
// back out immediately and carefully. The buffer content is also no longer
// trusted since it no longer belongs to the calling context.
//
#[no_mangle]
pub unsafe extern "C" fn nbcon_can_proceed(wctxt: *mut nbcon_write_context) -> bool {
    let mut ctxt = &ACCESS_PRIVATE(wctxt, ctxt);
    let mut con = ctxt.console;
pub static mut cur: usize = 0;
    nbcon_state_read(con, &cur);
    return nbcon_context_can_proceed(ctxt, &cur);
    }
    EXPORT_SYMBOL_GPL(nbcon_can_proceed);

//
// __nbcon_context_update_unsafe - Update the unsafe bit in @con->nbcon_state
// @ctxt:	The nbcon context from nbcon_context_try_acquire()
// @unsafe:	The new value for the unsafe bit
//
// Return:	True if the unsafe state was updated and this context still
// owns the console. Otherwise false if ownership was handed
// over or taken.
//
// This function allows console owners to modify the unsafe status of the
// console.
//
// When this function returns false then the calling context no longer owns
// the console and is no longer allowed to go forward. In this case it must
// back out immediately and carefully. The buffer content is also no longer
// trusted since it no longer belongs to the calling context.
//
// Internal helper to avoid duplicated code.
//
#[no_mangle]
unsafe extern "C" fn __nbcon_context_update_unsafe(ctxt: *mut nbcon_context, unsafe: bool) -> bool {
    let mut con = ctxt.console;
pub static mut cur: usize = 0;
pub static mut new: usize = 0;
    nbcon_state_read(con, &cur);
    do {
//
// The unsafe bit must not be cleared if an
// unsafe hostile takeover has occurred.
//
    if (!unsafe && cur.unsafe_takeover) {
// goto;
    }
    if (!nbcon_context_can_proceed(ctxt, &cur)) {
    return false;
    }
    new.atom = cur.atom;
    new.unsafe = unsafe;
    } while (!nbcon_state_try_cmpxchg(con, &cur, &new));
    cur.atom = new.atom;
// label;
    return nbcon_context_can_proceed(ctxt, &cur);
    }
#[no_mangle]
pub unsafe extern "C" fn nbcon_write_context_set_buf(wctxt: *mut nbcon_write_context, buf: *mut c_char, len: c_uint) {
    let mut ctxt = &ACCESS_PRIVATE(wctxt, ctxt);
    let mut con = ctxt.console;
pub static mut cur: usize = 0;
    wctxt.outbuf = buf;
    wctxt.len = len;
    nbcon_state_read(con, &cur);
    wctxt.unsafe_takeover = cur.unsafe_takeover;
    }
//
// nbcon_enter_unsafe - Enter an unsafe region in the driver
// @wctxt:	The write context that was handed to the write function
//
// Return:	True if this context still owns the console. False if
// ownership was handed over or taken.
//
// When this function returns false then the calling context no longer owns
// the console and is no longer allowed to go forward. In this case it must
// back out immediately and carefully. The buffer content is also no longer
// trusted since it no longer belongs to the calling context.
//
#[no_mangle]
pub unsafe extern "C" fn nbcon_enter_unsafe(wctxt: *mut nbcon_write_context) -> bool {
    let mut ctxt = &ACCESS_PRIVATE(wctxt, ctxt);
    let mut is_owner = 0;
    is_owner = nbcon_context_enter_unsafe(ctxt);
    if (!is_owner) {
    nbcon_write_context_set_buf(wctxt, core::ptr::null_mut(), 0);
    }
    return is_owner;
    }
    EXPORT_SYMBOL_GPL(nbcon_enter_unsafe);
//
// nbcon_exit_unsafe - Exit an unsafe region in the driver
// @wctxt:	The write context that was handed to the write function
//
// Return:	True if this context still owns the console. False if
// ownership was handed over or taken.
//
// When this function returns false then the calling context no longer owns
// the console and is no longer allowed to go forward. In this case it must
// back out immediately and carefully. The buffer content is also no longer
// trusted since it no longer belongs to the calling context.
//
#[no_mangle]
pub unsafe extern "C" fn nbcon_exit_unsafe(wctxt: *mut nbcon_write_context) -> bool {
    let mut ctxt = &ACCESS_PRIVATE(wctxt, ctxt);
    let mut ret = 0;
    ret = nbcon_context_exit_unsafe(ctxt);
    if (!ret) {
    nbcon_write_context_set_buf(wctxt, core::ptr::null_mut(), 0);
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(nbcon_exit_unsafe);
//
// nbcon_reacquire_nobuf - Reacquire a console after losing ownership
// while printing
// @wctxt:	The write context that was handed to the write callback
//
// Since ownership can be lost at any time due to handover or takeover, a
// printing context _must_ be prepared to back out immediately and
// carefully. However, there are scenarios where the printing context must
// reacquire ownership in order to finalize or revert hardware changes.
//
// This function allows a printing context to reacquire ownership using the
// same priority as its previous ownership.
//
// Note that after a successful reacquire the printing context will have no
// output buffer because that has been lost. This function cannot be used to
// resume printing.
//
#[no_mangle]
pub unsafe extern "C" fn nbcon_reacquire_nobuf(wctxt: *mut nbcon_write_context) {
    let mut ctxt = &ACCESS_PRIVATE(wctxt, ctxt);
    while (!nbcon_context_try_acquire(ctxt, true)) {
    cpu_relax();
    }
    nbcon_write_context_set_buf(wctxt, core::ptr::null_mut(), 0);
    }
    EXPORT_SYMBOL_GPL(nbcon_reacquire_nobuf);

#[no_mangle]
pub unsafe extern "C" fn wctxt_load_execution_ctx(wctxt: *mut nbcon_write_context, pmsg: *mut printk_message) {
    wctxt.cpu = pmsg.cpu;
    wctxt.pid = pmsg.pid;
    memcpy(wctxt.comm, pmsg.comm, sizeof!(wctxt.comm));
    static_assert(sizeof!(wctxt.comm) == sizeof!(pmsg.comm));
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: wctxt_load_execution_ctx
pub unsafe extern "C" fn wctxt_load_execution_ctx_dup(wctxt: *mut nbcon_write_context, pmsg: *mut printk_message) {}

//
// nbcon_emit_next_record - Emit a record in the acquired context
// @wctxt:	The write context that will be handed to the write function
// @use_atomic:	True if the write_atomic() callback is to be used
//
// Return:	True if this context still owns the console. False if
// ownership was handed over or taken.
//
// When this function returns false then the calling context no longer owns
// the console and is no longer allowed to go forward. In this case it must
// back out immediately and carefully. The buffer content is also no longer
// trusted since it no longer belongs to the calling context. If the caller
// wants to do more it must reacquire the console first.
//
// When true is returned, @wctxt->ctxt.backlog indicates whether there are
// still records pending in the ringbuffer,
//
#[no_mangle]
unsafe extern "C" fn nbcon_emit_next_record(wctxt: *mut nbcon_write_context, use_atomic: bool) -> bool {
    let mut ctxt = &ACCESS_PRIVATE(wctxt, ctxt);
    let mut con = ctxt.console;
pub static mut is_extended: bool = false;
pub static mut printk_message: usize = 0;
    let mut con_dropped = 0;
pub static mut cur: usize = 0;
    let mut dropped = 0;
    let mut ulseq = 0;
//
// This function should never be called for consoles that have not
// implemented the necessary callback for writing: i.e. legacy
// consoles and, when atomic, nbcon consoles with no write_atomic().
// Handle it as if ownership was lost and try to continue.
//
// Note that for nbcon consoles the write_thread() callback is
// mandatory and was already checked in nbcon_alloc().
//
    if (WARN_ON_ONCE!((use_atomic && !con.write_atomic) ||
    !(console_srcu_read_flags(con) & CON_NBCON))) {
    nbcon_context_release(ctxt);
    return false;
    }
//
// The printk buffers are filled within an unsafe section. This
// prevents NBCON_PRIO_NORMAL and NBCON_PRIO_EMERGENCY from
// clobbering each other.
//
    if (!nbcon_context_enter_unsafe(ctxt)) {
    return false;
    }
    ctxt.backlog = printk_get_next_message(&pmsg, ctxt.seq, is_extended, true);
    if (!ctxt.backlog) {
    return nbcon_context_exit_unsafe(ctxt);
    }
//
// @con->dropped is not protected in case of an unsafe hostile
// takeover. In that situation the update can be racy so
// annotate it accordingly.
//
    con_dropped = data_race(READ_ONCE(con.dropped));
    dropped = con_dropped + pmsg.dropped;
    if (dropped && !is_extended) {
    console_prepend_dropped(&pmsg, dropped);
    }
//
// If the previous owner was assigned the same record, this context
// has taken over ownership and is replaying the record. Prepend a
// message to let the user know the record is replayed.
//
    ulseq = atomic_long_read(&ACCESS_PRIVATE(con, nbcon_prev_seq));
    if (__ulseq_to_u64seq(prb, ulseq) == pmsg.seq) {
    console_prepend_replay(&pmsg);
    } else {
//
// Ensure this context is still the owner before trying to
// update @nbcon_prev_seq. Otherwise the value in @ulseq may
// not be from the previous owner and instead be some later
// value from the context that took over ownership.
//
    nbcon_state_read(con, &cur);
    if (!nbcon_context_can_proceed(ctxt, &cur)) {
    return false;
    }
    atomic_long_try_cmpxchg(&ACCESS_PRIVATE(con, nbcon_prev_seq), &ulseq,
    __u64seq_to_ulseq(pmsg.seq));
    }
    if (!nbcon_context_exit_unsafe(ctxt)) {
    return false;
    }
// For skipped records just update seq/dropped in @con.
    if (pmsg.outbuf_len == 0) {
// goto;
    }
// Initialize the write context for driver callbacks.
    nbcon_write_context_set_buf(wctxt, &pmsg.pbufs.outbuf[0], pmsg.outbuf_len);
    wctxt_load_execution_ctx(wctxt, &pmsg);
    if (use_atomic) {
    con.write_atomic(con, wctxt);
    }
    else {
    con.write_thread(con, wctxt);
    }
    if (!wctxt.outbuf) {
//
// Ownership was lost and reacquired by the driver. Handle it
// as if ownership was lost.
//
    nbcon_context_release(ctxt);
    return false;
    }
//
// Ownership may have been lost but _not_ reacquired by the driver.
// This case is detected and handled when entering unsafe to update
// dropped/seq values.
//
// Since any dropped message was successfully output, reset the
// dropped count for the console.
//
    dropped = 0;
// label;
//
// The dropped count and the sequence number are updated within an
// unsafe section. This limits update races to the panic context and
// allows the panic context to win.
//
    if (!nbcon_context_enter_unsafe(ctxt)) {
    return false;
    }
    if (dropped != con_dropped) {
// Counterpart to the READ_ONCE() above.
    WRITE_ONCE(con.dropped, dropped);
    }
    nbcon_seq_try_update(ctxt, pmsg.seq + 1);
    return nbcon_context_exit_unsafe(ctxt);
    }
//
// nbcon_emit_one - Print one record for an nbcon console using the
// specified callback
// @wctxt:	An initialized write context struct to use for this context
// @use_atomic:	True if the write_atomic() callback is to be used
//
// Return:	True, when a record has been printed and there are still
// pending records. The caller might want to continue flushing.
//
// False, when there is no pending record, or when the console
// context cannot be acquired, or the ownership has been lost.
// The caller should give up. Either the job is done, cannot be
// done, or will be handled by the owning context.
//
// This is an internal helper to handle the locking of the console before
// calling nbcon_emit_next_record().
//
#[no_mangle]
unsafe extern "C" fn nbcon_emit_one(wctxt: *mut nbcon_write_context, use_atomic: bool) -> bool {
    let mut ctxt = &ACCESS_PRIVATE(wctxt, ctxt);
    let mut con = ctxt.console;
    let mut flags = 0;
pub static mut ret: bool = false;
    if (!use_atomic) {
    con.device_lock(con, &flags);
//
// Ensure this stays on the CPU to make handover and
// takeover possible.
//
    cant_migrate();
    }
    if (!nbcon_context_try_acquire(ctxt, false)) {
// goto;
    }
//
// nbcon_emit_next_record() returns false when the console was
// handed over or taken over. In both cases the context is no
// longer valid.
//
// The higher priority printing context takes over responsibility
// to print the pending records.
//
    if (!nbcon_emit_next_record(wctxt, use_atomic)) {
// goto;
    }
    nbcon_context_release(ctxt);
    ret = ctxt.backlog;
// label;
    if (!use_atomic) {
    con.device_unlock(con, flags);
    }
    return ret;
    }
//
// nbcon_kthread_should_wakeup - Check whether a printer thread should wakeup
// @con:	Console to operate on
// @ctxt:	The nbcon context from nbcon_context_try_acquire()
//
// Return:	True if the thread should shutdown or if the console is
// allowed to print and a record is available. False otherwise.
//
// After the thread wakes up, it must first check if it should shutdown before
// attempting any printing.
//
#[no_mangle]
unsafe extern "C" fn nbcon_kthread_should_wakeup(con: *mut console, ctxt: *mut nbcon_context) -> bool {
pub static mut ret: bool = false;
    let mut flags = 0;
    let mut cookie = 0;
    if (kthread_should_stop()) {
    return true;
    }
//
// Block the kthread when the system is in an emergency or panic mode.
// It increases the chance that these contexts would be able to show
// the messages directly. And it reduces the risk of interrupted writes
// where the context with a higher priority takes over the nbcon console
// ownership in the middle of a message.
//
    if (unlikely(atomic_read(&nbcon_cpu_emergency_cnt)) ||
    unlikely(panic_in_progress())) {
    return false;
    }
    cookie = console_srcu_read_lock();
    flags = console_srcu_read_flags(con);
    if (console_is_usable(con, flags, false)) {
// Bring the sequence in @ctxt up to date
    ctxt.seq = nbcon_seq_read(con);
    ret = prb_read_valid(prb, ctxt.seq, core::ptr::null_mut());
    }
    console_srcu_read_unlock(cookie);
    return ret;
    }
//
// nbcon_kthread_func - The printer thread function
// @__console:	Console to operate on
//
// Return:	0
//
#[no_mangle]
unsafe extern "C" fn nbcon_kthread_func(__console: *mut c_void) -> c_int {
    let mut con = __console;
pub static mut nbcon_write_context: usize = 0;
    let mut ctxt = &ACCESS_PRIVATE(&wctxt, ctxt);
    let mut con_flags = 0;
    let mut backlog = 0;
    let mut cookie = 0;
// label;
//
// Guarantee this task is visible on the rcuwait before
// checking the wake condition.
//
// The full memory barrier within set_current_state() of
// ___rcuwait_wait_event() pairs with the full memory
// barrier within rcuwait_has_sleeper().
//
// This pairs with rcuwait_has_sleeper:A and nbcon_kthread_wake:A.
//
    rcuwait_wait_event(&con.rcuwait,
    nbcon_kthread_should_wakeup(con, ctxt),
    TASK_INTERRUPTIBLE); /* LMM(nbcon_kthread_func:A) */
    do {
    if (kthread_should_stop()) {
    return 0;
    }
//
// Block the kthread when the system is in an emergency or panic
// mode. See nbcon_kthread_should_wakeup() for more details.
//
    if (unlikely(atomic_read(&nbcon_cpu_emergency_cnt)) ||
    unlikely(panic_in_progress())) {
// goto;
    }
    backlog = false;
//
// Keep the srcu read lock around the entire operation so that
// synchronize_srcu() can guarantee that the kthread stopped
// or suspended printing.
//
    cookie = console_srcu_read_lock();
    con_flags = console_srcu_read_flags(con);
    if (console_is_usable(con, con_flags, false)) {
    backlog = nbcon_emit_one(&wctxt, false);
    }
    console_srcu_read_unlock(cookie);
    cond_resched();
    } while (backlog);
// goto;
    }
//
// nbcon_irq_work - irq work to wake console printer thread
// @irq_work:	The irq work to operate on
//
#[no_mangle]
unsafe extern "C" fn nbcon_irq_work(irq_work: *mut irq_work) {
    let mut con = container_of!(irq_work, console, irq_work);
    nbcon_kthread_wake(con);
    }
#[no_mangle]
pub unsafe extern "C" fn rcuwait_has_sleeper(w: *mut rcuwait) -> bool {
//
// Guarantee any new records can be seen by tasks preparing to wait
// before this context checks if the rcuwait is empty.
//
// This full memory barrier pairs with the full memory barrier within
// set_current_state() of ___rcuwait_wait_event(), which is called
// after prepare_to_rcuwait() adds the waiter but before it has
// checked the wait condition.
//
// This pairs with nbcon_kthread_func:A.
//
    smp_mb(); /* LMM(rcuwait_has_sleeper:A) */
    return rcuwait_active(w);
    }
//
// nbcon_kthreads_wake - Wake up printing threads using irq_work
//
#[no_mangle]
pub unsafe extern "C" fn nbcon_kthreads_wake() {
pub static mut con: *mut c_void = core::ptr::null_mut();
    let mut cookie = 0;
    if (!printk_kthreads_running) {
    return;
    }
//
// It is not allowed to call this function when console irq_work
// is blocked.
//
    if (WARN_ON_ONCE!(console_irqwork_blocked)) {
    return;
    }
    cookie = console_srcu_read_lock();
    for_each_console_srcu(con) {
    if (!(console_srcu_read_flags(con) & CON_NBCON)) {
    continue;
    }
//
// Only schedule irq_work if the printing thread is
// actively waiting. If not waiting, the thread will
// notice by itself that it has work to do.
//
    if (rcuwait_has_sleeper(&con.rcuwait)) {
    irq_work_queue(&con.irq_work);
    }
    }
    console_srcu_read_unlock(cookie);
    }
//
// nbcon_kthread_stop - Stop a console printer thread
// @con:	Console to operate on
//
#[no_mangle]
pub unsafe extern "C" fn nbcon_kthread_stop(con: *mut console) {
    lockdep_assert_console_list_lock_held();
    if (!con.kthread) {
    return;
    }
    kthread_stop(con.kthread);
    con.kthread = core::ptr::null_mut();
    }
//
// nbcon_kthread_create - Create a console printer thread
// @con:	Console to operate on
//
// Return:	True if the kthread was started or already exists.
// Otherwise false and @con must not be registered.
//
// This function is called when it will be expected that nbcon consoles are
// flushed using the kthread. The messages printed with NBCON_PRIO_NORMAL
// will be no longer flushed by the legacy loop. This is why failure must
// be fatal for console registration.
//
// If @con was already registered and this function fails, @con must be
// unregistered before the global state variable @printk_kthreads_running
// can be set.
//
#[no_mangle]
pub unsafe extern "C" fn nbcon_kthread_create(con: *mut console) -> bool {
pub static mut kt: *mut c_void = core::ptr::null_mut();
    lockdep_assert_console_list_lock_held();
    if (con.kthread) {
    return true;
    }
    kt = kthread_run(nbcon_kthread_func, con, "pr/%s%d", con.name, con.index);
    if (WARN_ON!(IS_ERR(kt))) {
    con_printk(KERN_ERR, con, "failed to start printing thread\n");
    return false;
    }
    con.kthread = kt;
//
// It is important that console printing threads are scheduled
// shortly after a printk call and with generous runtime budgets.
//
    sched_set_normal(con.kthread, -20);
    return true;
    }
// Track the nbcon emergency nesting per CPU.
pub static mut unsigned int: usize = 0;
    static unsigned int early_nbcon_pcpu_emergency_nesting __initdata;
//
// nbcon_get_cpu_emergency_nesting - Get the per CPU emergency nesting pointer
//
// Context:	For reading, any context. For writing, any context which could
// not be migrated to another CPU.
// Return:	Either a pointer to the per CPU emergency nesting counter of
// the current CPU or to the init data during early boot.
//
// The function is safe for reading per-CPU variables in any context because
// preemption is disabled if the current CPU is in the emergency state. See
// also nbcon_cpu_emergency_enter().
//
    static __ref unsigned int *nbcon_get_cpu_emergency_nesting(void)
    {
//
// The value of __printk_percpu_data_ready gets set in normal
// context and before SMP initialization. As a result it could
// never change while inside an nbcon emergency section.
//
    if (!printk_percpu_data_ready()) {
    return &early_nbcon_pcpu_emergency_nesting;
    }
    return raw_cpu_ptr(&nbcon_pcpu_emergency_nesting);
    }
//
// nbcon_get_default_prio - The appropriate nbcon priority to use for nbcon
// printing on the current CPU
//
// Context:	Any context.
// Return:	The nbcon_prio to use for acquiring an nbcon console in this
// context for printing.
//
// The function is safe for reading per-CPU data in any context because
// preemption is disabled if the current CPU is in the emergency or panic
// state.
//
#[no_mangle]
pub unsafe extern "C" fn nbcon_get_default_prio() -> enum nbcon_prio {
pub static mut cpu_emergency_nesting: *mut c_void = core::ptr::null_mut();
    if (panic_on_this_cpu()) {
    return NBCON_PRIO_PANIC;
    }
    cpu_emergency_nesting = nbcon_get_cpu_emergency_nesting();
    if (*cpu_emergency_nesting) {
    return NBCON_PRIO_EMERGENCY;
    }
    return NBCON_PRIO_NORMAL;
    }
//
// Track if it is allowed to perform unsafe hostile takeovers of console
// ownership. When true, console drivers might perform unsafe actions while
// printing. It is externally available via nbcon_allow_unsafe_takeover().
//
    static bool panic_nbcon_allow_unsafe_takeover;
//
// nbcon_allow_unsafe_takeover - Check if unsafe console takeovers are allowed
//
// Return:	True, when it is permitted to perform unsafe console printing
//
// This is also used by console_is_usable() to determine if it is allowed to
// call write_atomic() callbacks flagged as unsafe (CON_NBCON_ATOMIC_UNSAFE).
//
#[no_mangle]
pub unsafe extern "C" fn nbcon_allow_unsafe_takeover() -> bool {
    return panic_on_this_cpu() && panic_nbcon_allow_unsafe_takeover;
    }
//
// nbcon_legacy_emit_next_record - Print one record for an nbcon console
// in legacy contexts
// @con:	The console to print on
// @handover:	Will be set to true if a printk waiter has taken over the
// console_lock, in which case the caller is no longer holding
// both the console_lock and the SRCU read lock. Otherwise it
// is set to false.
// @cookie:	The cookie from the SRCU read lock.
// @use_atomic: Set true when called in an atomic or unknown context.
// It affects which nbcon callback will be used: write_atomic()
// or write_thread().
//
// When false, the write_thread() callback is used and would be
// called in a preemptible context unless disabled by the
// device_lock. The legacy handover is not allowed in this mode.
//
// Context:	Any context except NMI.
// Return:	True, when a record has been printed and there are still
// pending records. The caller might want to continue flushing.
//
// False, when there is no pending record, or when the console
// context cannot be acquired, or the ownership has been lost.
// The caller should give up. Either the job is done, cannot be
// done, or will be handled by the owning context.
//
// This function is meant to be called by console_flush_all() to print records
// on nbcon consoles from legacy context (printing via console unlocking).
// Essentially it is the nbcon version of console_emit_next_record().
//
#[no_mangle]
pub unsafe extern "C" fn nbcon_legacy_emit_next_record(con: *mut console, handover: *mut bool, cookie: c_int, use_atomic: bool) -> bool {
pub static mut wctxt: nbcon_write_context = 0;
    let mut ctxt = &ACCESS_PRIVATE(&wctxt, ctxt);
    let mut flags = 0;
    let mut progress = 0;
    ctxt.console	= con;
    ctxt.prio	= nbcon_get_default_prio();
    if (use_atomic) {
//
// In an atomic or unknown context, use the same procedure as
// in console_emit_next_record(). It allows to handover.
//
    printk_safe_enter_irqsave(flags);
    console_lock_spinning_enable();
    stop_critical_timings();
    }
    progress = nbcon_emit_one(&wctxt, use_atomic);
    if (use_atomic) {
    start_critical_timings();
// handover = console_lock_spinning_disable_and_check(cookie);
    printk_safe_exit_irqrestore(flags);
    } else {
// Non-atomic does not perform legacy spinning handovers.
// handover = false;
    }
    return progress;
    }
//
// __nbcon_atomic_flush_pending_con - Flush specified nbcon console using its
// write_atomic() callback
// @con:			The nbcon console to flush
// @stop_seq:			Flush up until this record
//
// Return:	0 if @con was flushed up to @stop_seq Otherwise, error code on
// failure.
//
// Errors:
//
// -EPERM:		Unable to acquire console ownership.
//
// -EAGAIN:	Another context took over ownership while printing.
//
// -ENOENT:	A record before @stop_seq is not available.
//
// If flushing up to @stop_seq was not successful, it only makes sense for the
// caller to try again when -EAGAIN was returned. When -EPERM is returned,
// this context is not allowed to acquire the console. When -ENOENT is
// returned, it cannot be expected that the unfinalized record will become
// available.
//
#[no_mangle]
unsafe extern "C" fn __nbcon_atomic_flush_pending_con(con: *mut console, stop_seq: u64) -> c_int {
pub static mut wctxt: nbcon_write_context = 0;
    let mut ctxt = &ACCESS_PRIVATE(&wctxt, ctxt);
pub static mut err: c_int = 0;
    ctxt.console			= con;
    ctxt.spinwait_max_us		= 2000;
    ctxt.prio			= nbcon_get_default_prio();
    ctxt.allow_unsafe_takeover	= nbcon_allow_unsafe_takeover();
    while (nbcon_seq_read(con) < stop_seq) {
//
// Atomic flushing does not use console driver synchronization
// (i.e. it does not hold the port lock for uart consoles).
// Therefore IRQs must be disabled to avoid being interrupted
// and then calling into a driver that will deadlock trying
// to acquire console ownership.
//
    scoped_guard(irqsave) {
    if (!nbcon_context_try_acquire(ctxt, false)) {
    return -EPERM;
    }
//
// nbcon_emit_next_record() returns false when
// the console was handed over or taken over.
// In both cases the context is no longer valid.
//
    if (!nbcon_emit_next_record(&wctxt, true)) {
    return -EAGAIN;
    }
    nbcon_context_release(ctxt);
    }
    if (!ctxt.backlog) {
// Are there reserved but not yet finalized records?
    if (nbcon_seq_read(con) < stop_seq) {
    err = -ENOENT;
    }
    break;
    }
    }
    return err;
    }
//
// nbcon_atomic_flush_pending_con - Flush specified nbcon console using its
// write_atomic() callback
// @con:			The nbcon console to flush
// @stop_seq:			Flush up until this record
//
// This will stop flushing before @stop_seq if another context has ownership.
// That context is then responsible for the flushing. Likewise, if new records
// are added while this context was flushing and there is no other context
// to handle the printing, this context must also flush those records.
//
#[no_mangle]
unsafe extern "C" fn nbcon_atomic_flush_pending_con(con: *mut console, stop_seq: u64) {
pub static mut ft: usize = 0;
    let mut err = 0;
// label;
    err = __nbcon_atomic_flush_pending_con(con, stop_seq);
//
// If there was a new owner (-EPERM, -EAGAIN), that context is
// responsible for completing.
//
// Do not wait for records not yet finalized (-ENOENT) to avoid a
// possible deadlock. They will either get flushed by the writer or
// eventually skipped on panic CPU.
//
    if (err) {
    return;
    }
//
// If flushing was successful but more records are available, this
// context must flush those remaining records if the printer thread
// is not available do it.
//
    printk_get_console_flush_type(&ft);
    if (!ft.nbcon_offload &&
    prb_read_valid(prb, nbcon_seq_read(con), core::ptr::null_mut())) {
    stop_seq = prb_next_reserve_seq(prb);
// goto;
    }
    }
//
// __nbcon_atomic_flush_pending - Flush all nbcon consoles using their
// write_atomic() callback
// @stop_seq:			Flush up until this record
//
#[no_mangle]
unsafe extern "C" fn __nbcon_atomic_flush_pending(stop_seq: u64) {
pub static mut con: *mut c_void = core::ptr::null_mut();
    let mut cookie = 0;
    cookie = console_srcu_read_lock();
    for_each_console_srcu(con) {
pub static mut flags: c_short = 0;
    if (!(flags & CON_NBCON)) {
    continue;
    }
    if (!console_is_usable(con, flags, true)) {
    continue;
    }
    if (nbcon_seq_read(con) >= stop_seq) {
    continue;
    }
    nbcon_atomic_flush_pending_con(con, stop_seq);
    }
    console_srcu_read_unlock(cookie);
    }
//
// nbcon_atomic_flush_pending - Flush all nbcon consoles using their
// write_atomic() callback
//
// Flush the backlog up through the currently newest record. Any new
// records added while flushing will not be flushed if there is another
// context available to handle the flushing. This is to avoid one CPU
// printing unbounded because other CPUs continue to add records.
//
#[no_mangle]
pub unsafe extern "C" fn nbcon_atomic_flush_pending() {
    __nbcon_atomic_flush_pending(prb_next_reserve_seq(prb));
    }
//
// nbcon_atomic_flush_unsafe - Flush all nbcon consoles using their
// write_atomic() callback and allowing unsafe hostile takeovers
//
// Flush the backlog up through the currently newest record. Unsafe hostile
// takeovers will be performed, if necessary.
//
#[no_mangle]
pub unsafe extern "C" fn nbcon_atomic_flush_unsafe() {
    panic_nbcon_allow_unsafe_takeover = true;
    __nbcon_atomic_flush_pending(prb_next_reserve_seq(prb));
    panic_nbcon_allow_unsafe_takeover = false;
    }
//
// nbcon_cpu_emergency_enter - Enter an emergency section where printk()
// messages for that CPU are flushed directly
//
// Context:	Any context. Disables preemption.
//
// When within an emergency section, printk() calls will attempt to flush any
// pending messages in the ringbuffer.
//
#[no_mangle]
pub unsafe extern "C" fn nbcon_cpu_emergency_enter() {
pub static mut cpu_emergency_nesting: *mut c_void = core::ptr::null_mut();
    preempt_disable();
    atomic_inc(&nbcon_cpu_emergency_cnt);
    cpu_emergency_nesting = nbcon_get_cpu_emergency_nesting();
    (*cpu_emergency_nesting)++;
    }
//
// nbcon_cpu_emergency_exit - Exit an emergency section
//
// Context:	Within an emergency section. Enables preemption.
//
#[no_mangle]
pub unsafe extern "C" fn nbcon_cpu_emergency_exit() {
pub static mut cpu_emergency_nesting: *mut c_void = core::ptr::null_mut();
    cpu_emergency_nesting = nbcon_get_cpu_emergency_nesting();
    if (!WARN_ON_ONCE!(*cpu_emergency_nesting == 0)) {
    (*cpu_emergency_nesting)--;
    }
//
// Wake up kthreads because there might be some pending messages
// added by other CPUs with normal priority since the last flush
// in the emergency context.
//
    if (!WARN_ON_ONCE!(atomic_read(&nbcon_cpu_emergency_cnt) == 0)) {
    if (atomic_dec_return(&nbcon_cpu_emergency_cnt) == 0) {
pub static mut ft: usize = 0;
    printk_get_console_flush_type(&ft);
    if (ft.nbcon_offload) {
    nbcon_kthreads_wake();
    }
    }
    }
    preempt_enable();
    }
//
// nbcon_alloc - Allocate and init the nbcon console specific data
// @con:	Console to initialize
//
// Return:	True if the console was fully allocated and initialized.
// Otherwise @con must not be registered.
//
// When allocation and init was successful, the console must be properly
// freed using nbcon_free() once it is no longer needed.
//
#[no_mangle]
pub unsafe extern "C" fn nbcon_alloc(con: *mut console) -> bool {
pub static mut state: nbcon_state = 0;
// Synchronize the kthread start.
    lockdep_assert_console_list_lock_held();
// Check for mandatory nbcon callbacks.
    if (WARN_ON!(!con.write_thread ||
    !con.device_lock ||
    !con.device_unlock)) {
    return false;
    }
    rcuwait_init(&con.rcuwait);
    init_irq_work(&con.irq_work, nbcon_irq_work);
    atomic_long_set(&ACCESS_PRIVATE(con, nbcon_prev_seq), -1UL);
    nbcon_state_set(con, &state);
//
// Initialize @nbcon_seq to the highest possible sequence number so
// that practically speaking it will have nothing to print until a
// desired initial sequence number has been set via nbcon_seq_force().
//
    atomic_long_set(&ACCESS_PRIVATE(con, nbcon_seq), ULSEQ_MAX(prb));
    if (con.flags & CON_BOOT) {
//
// Boot console printing is synchronized with legacy console
// printing, so boot consoles can share the same global printk
// buffers.
//
    con.pbufs = &printk_shared_pbufs;
    } else {
    con.pbufs = kmalloc_obj(*con.pbufs);
    if (!con.pbufs) {
    con_printk(KERN_ERR, con, "failed to allocate printing buffer\n");
    return false;
    }
    if (printk_kthreads_ready && !have_boot_console) {
    if (!nbcon_kthread_create(con)) {
    kfree(con.pbufs);
    con.pbufs = core::ptr::null_mut();
    return false;
    }
// Might be the first kthread.
    printk_kthreads_running = true;
    }
    }
    return true;
    }
//
// nbcon_free - Free and cleanup the nbcon console specific data
// @con:	Console to free/cleanup nbcon data
//
// Important: @have_nbcon_console must be updated before calling
// this function. In particular, it can be set only when there
// is still another nbcon console registered.
//
#[no_mangle]
pub unsafe extern "C" fn nbcon_free(con: *mut console) {
pub static mut state: nbcon_state = 0;
// Synchronize the kthread stop.
    lockdep_assert_console_list_lock_held();
    if (printk_kthreads_running) {
    nbcon_kthread_stop(con);
// Might be the last nbcon console.
//
// Do not rely on printk_kthreads_check_locked(). It is not
// called in some code paths, see nbcon_free() callers.
//
    if (!have_nbcon_console) {
    printk_kthreads_running = false;
    }
    }
    nbcon_state_set(con, &state);
// Boot consoles share global printk buffers.
    if (!(con.flags & CON_BOOT)) {
    kfree(con.pbufs);
    }
    con.pbufs = core::ptr::null_mut();
    }
//
// nbcon_device_try_acquire - Try to acquire nbcon console and enter unsafe
// section
// @con:	The nbcon console to acquire
//
// Context:	Under the locking mechanism implemented in
// @con->device_lock() including disabling migration.
// Return:	True if the console was acquired. False otherwise.
//
// Console drivers will usually use their own internal synchronization
// mechanism to synchronize between console printing and non-printing
// activities (such as setting baud rates). However, nbcon console drivers
// supporting atomic consoles may also want to mark unsafe sections when
// performing non-printing activities in order to synchronize against their
// atomic_write() callback.
//
// This function acquires the nbcon console using priority NBCON_PRIO_NORMAL
// and marks it unsafe for handover/takeover.
//
#[no_mangle]
pub unsafe extern "C" fn nbcon_device_try_acquire(con: *mut console) -> bool {
    let mut ctxt = &ACCESS_PRIVATE(con, nbcon_device_ctxt);
    cant_migrate();
    memset(ctxt, 0, sizeof!(*ctxt));
    ctxt.console	= con;
    ctxt.prio	= NBCON_PRIO_NORMAL;
    if (!nbcon_context_try_acquire(ctxt, false)) {
    return false;
    }
    if (!nbcon_context_enter_unsafe(ctxt)) {
    return false;
    }
    return true;
    }
    EXPORT_SYMBOL_GPL(nbcon_device_try_acquire);
//
// nbcon_device_release - Exit unsafe section and release the nbcon console
// @con:	The nbcon console acquired in nbcon_device_try_acquire()
//
#[no_mangle]
pub unsafe extern "C" fn nbcon_device_release(con: *mut console) {
    let mut ctxt = &ACCESS_PRIVATE(con, nbcon_device_ctxt);
pub static mut ft: usize = 0;
    let mut cookie = 0;
    if (!nbcon_context_exit_unsafe(ctxt)) {
    return;
    }
    nbcon_context_release(ctxt);
//
// This context must flush any new records added while the console
// was locked if the printer thread is not available to do it. The
// console_srcu_read_lock must be taken to ensure the console is
// usable throughout flushing.
//
    cookie = console_srcu_read_lock();
    printk_get_console_flush_type(&ft);
    if (console_is_usable(con, console_srcu_read_flags(con), true) &&
    !ft.nbcon_offload &&
    prb_read_valid(prb, nbcon_seq_read(con), core::ptr::null_mut())) {
//
// If nbcon_atomic flushing is not available, fallback to
// using the legacy loop.
//
    if (ft.nbcon_atomic) {
    __nbcon_atomic_flush_pending_con(con, prb_next_reserve_seq(prb));
    } else if (ft.legacy_direct) {
    if (console_trylock()) {
    console_unlock();
    }
    } else if (ft.legacy_offload) {
    defer_console_output();
    }
    }
    console_srcu_read_unlock(cookie);
    }
    EXPORT_SYMBOL_GPL(nbcon_device_release);
//
// nbcon_kdb_try_acquire - Try to acquire nbcon console and enter unsafe
// section
// @con:	The nbcon console to acquire
// @wctxt:	The nbcon write context to be used on success
//
// Context:	Under console_srcu_read_lock() for emitting a single kdb message
// using the given con->write_atomic() callback. Can be called
// only when the console is usable at the moment.
//
// Return:	True if the console was acquired. False otherwise.
//
// kdb emits messages on consoles registered for printk() without
// storing them into the ring buffer. It has to acquire the console
// ownership so that it could call con->write_atomic() callback a safe way.
//
// This function acquires the nbcon console using priority NBCON_PRIO_EMERGENCY
// and marks it unsafe for handover/takeover.
//
#[no_mangle]
pub unsafe extern "C" fn nbcon_kdb_try_acquire(con: *mut console, wctxt: *mut nbcon_write_context) -> bool {
    let mut ctxt = &ACCESS_PRIVATE(wctxt, ctxt);
    memset(ctxt, 0, sizeof!(*ctxt));
    ctxt.console = con;
    ctxt.prio    = NBCON_PRIO_EMERGENCY;
    if (!nbcon_context_try_acquire(ctxt, false)) {
    return false;
    }
    if (!nbcon_context_enter_unsafe(ctxt)) {
    return false;
    }
    return true;
    }
//
// nbcon_kdb_release - Exit unsafe section and release the nbcon console
//
// @wctxt:	The nbcon write context initialized by a successful
// nbcon_kdb_try_acquire()
//
#[no_mangle]
pub unsafe extern "C" fn nbcon_kdb_release(wctxt: *mut nbcon_write_context) {
    let mut ctxt = &ACCESS_PRIVATE(wctxt, ctxt);
    if (!nbcon_context_exit_unsafe(ctxt)) {
    return;
    }
    nbcon_context_release(ctxt);
//
// Flush any new printk() messages added when the console was blocked.
// Only the console used by the given write context was	blocked.
// The console was locked only when the write_atomic() callback
// was usable.
//
    __nbcon_atomic_flush_pending_con(ctxt.console, prb_next_reserve_seq(prb));
    }