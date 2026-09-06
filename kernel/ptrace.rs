//! Automatically rewritten from C to Rust
//! Source: kernel/ptrace.c
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



// SPDX-License-Identifier: GPL-2.0-only
//
// linux/kernel/ptrace.c
//
// (C) Copyright 1999 Linus Torvalds
//
// Common interfaces for "ptrace()" which we do not want
// to continually duplicate across every architecture.
//

//
// ptracer_access_allowed - may current peek/poke @tsk's address space?
// @tsk: tracee
//
// Per-access check used by ptrace_access_vm() and architecture-specific
// tag/register accessors.  Returns true iff current is the registered
// ptracer of @tsk and either @tsk is owner-dumpable or current holds
// CAP_SYS_PTRACE in @tsk's exec namespace.  Lighter than
// __ptrace_may_access(): it re-validates only dumpability and
// capability on every access, without re-running LSM hooks or
// cred_cap_issubset() checks performed at attach time.
//
#[no_mangle]
pub unsafe extern "C" fn ptracer_access_allowed(tsk: *mut task_struct) -> bool {
pub static mut es: *mut c_void = core::ptr::null_mut();
    guard(rcu)();
    if (ptrace_parent(tsk) != current) {
    return false;
    }
    es = task_exec_state_rcu(tsk);
    return READ_ONCE(es.dumpable) == TASK_DUMPABLE_OWNER ||
    ptracer_capable(tsk, es.user_ns);
    }
//
// Access another process' address space via ptrace.
// Source/target buffer must be kernel space,
// Do not walk the page table directly, use get_user_pages
//
#[no_mangle]
pub unsafe extern "C" fn ptrace_access_vm(tsk: *mut task_struct, addr: c_ulong, buf: *mut c_void, len: c_int, gup_flags: c_uint) -> c_int {
pub static mut mm: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    mm = get_task_mm(tsk);
    if (!mm) {
    return 0;
    }
    if (ptracer_access_allowed(tsk)) {
    ret = access_remote_vm(mm, addr, buf, len, gup_flags);
    }
    mmput(mm);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn __ptrace_link(child: *mut task_struct, new_parent: *mut task_struct, ptracer_cred: *mut cred) {
    BUG_ON!(!list_empty(&child.ptrace_entry));
    list_add(&child.ptrace_entry, &new_parent.ptraced);
    child.parent = new_parent;
    child.ptracer_cred = get_cred(ptracer_cred);
    }
//
// ptrace a task: make the debugger its new parent and
// move it to the ptrace list.
//
// Must be called with the tasklist lock write-held.
//
#[no_mangle]
unsafe extern "C" fn ptrace_link(child: *mut task_struct, new_parent: *mut task_struct) {
    __ptrace_link(child, new_parent, current_cred());
    }
//
// __ptrace_unlink - unlink ptracee and restore its execution state
// @child: ptracee to be unlinked
//
// Remove @child from the ptrace list, move it back to the original parent,
// and restore the execution state so that it conforms to the group stop
// state.
//
// Unlinking can happen via two paths - explicit PTRACE_DETACH or ptracer
// exiting.  For PTRACE_DETACH, unless the ptracee has been killed between
// ptrace_check_attach() and here, it's guaranteed to be in TASK_TRACED.
// If the ptracer is exiting, the ptracee can be in any state.
//
// After detach, the ptracee should be in a state which conforms to the
// group stop.  If the group is stopped or in the process of stopping, the
// ptracee should be put into TASK_STOPPED; otherwise, it should be woken
// up from TASK_TRACED.
//
// If the ptracee is in TASK_TRACED and needs to be moved to TASK_STOPPED,
// it goes through TRACED -> RUNNING -> STOPPED transition which is similar
// to but in the opposite direction of what happens while attaching to a
// stopped task.  However, in this direction, the intermediate RUNNING
// state is not hidden even from the current ptracer and if it immediately
// re-attaches and performs a WNOHANG wait(2), it may fail.
//
// CONTEXT:
// write_lock_irq(tasklist_lock)
//
#[no_mangle]
pub unsafe extern "C" fn __ptrace_unlink(child: *mut task_struct) {
pub static mut old_cred: *mut c_void = core::ptr::null_mut();
    BUG_ON!(!child.ptrace);
    clear_task_syscall_work(child, SYSCALL_TRACE);

    clear_task_syscall_work(child, SYSCALL_EMU);

    child.parent = child.real_parent;
    list_del_init(&child.ptrace_entry);
    old_cred = child.ptracer_cred;
    child.ptracer_cred = core::ptr::null_mut();
    put_cred(old_cred);
    spin_lock(&child.sighand.siglock);
    child.ptrace = 0;
//
// Clear all pending traps and TRAPPING.  TRAPPING should be
// cleared regardless of JOBCTL_STOP_PENDING.  Do it explicitly.
//
    task_clear_jobctl_pending(child, JOBCTL_TRAP_MASK);
    task_clear_jobctl_trapping(child);
//
// Reinstate JOBCTL_STOP_PENDING if group stop is in effect and
// @child isn't dead.
//
    if (!(child.flags & PF_EXITING) &&
    (child.signal.flags & SIGNAL_STOP_STOPPED ||
    child.signal.group_stop_count)) {
    child.jobctl |= JOBCTL_STOP_PENDING;
    }
//
// If transition to TASK_STOPPED is pending or in TASK_TRACED, kick
// @child in the butt.  Note that @resume should be used iff @child
// is in TASK_TRACED; otherwise, we might unduly disrupt
// TASK_KILLABLE sleeps.
//
    if (child.jobctl & JOBCTL_STOP_PENDING || task_is_traced(child)) {
    ptrace_signal_wake_up(child, true);
    }
    spin_unlock(&child.sighand.siglock);
    }
#[no_mangle]
unsafe extern "C" fn looks_like_a_spurious_pid(task: *mut task_struct) -> bool {
    if (task.exit_code != ((PTRACE_EVENT_EXEC << 8) | SIGTRAP)) {
    return false;
    }
    if (task_pid_vnr(task) == task.ptrace_message) {
    return false;
    }
//
// The tracee changed its pid but the PTRACE_EVENT_EXEC event
// was not wait()'ed, most probably debugger targets the old
// leader which was destroyed in de_thread().
//
    return true;
    }
//
// Ensure that nothing can wake it up, even SIGKILL
//
// A task is switched to this state while a ptrace operation is in progress;
// such that the ptrace operation is uninterruptible.
//
#[no_mangle]
unsafe extern "C" fn ptrace_freeze_traced(task: *mut task_struct) -> bool {
pub static mut ret: bool = false;
// Lockless, nobody but us can set this flag
    if (task.jobctl & JOBCTL_LISTENING) {
    return ret;
    }
    spin_lock_irq(&task.sighand.siglock);
    if (task_is_traced(task) && !looks_like_a_spurious_pid(task) &&
    !__fatal_signal_pending(task)) {
    task.jobctl |= JOBCTL_PTRACE_FROZEN;
    ret = true;
    }
    spin_unlock_irq(&task.sighand.siglock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ptrace_unfreeze_traced(task: *mut task_struct) {
    let mut flags = 0;
//
// The child may be awake and may have cleared
// JOBCTL_PTRACE_FROZEN (see ptrace_resume).  The child will
// not set JOBCTL_PTRACE_FROZEN or enter __TASK_TRACED anew.
//
    if (lock_task_sighand(task, &flags)) {
    task.jobctl &= ~JOBCTL_PTRACE_FROZEN;
    if (__fatal_signal_pending(task)) {
    task.jobctl &= ~JOBCTL_TRACED;
    wake_up_state(task, __TASK_TRACED);
    }
    unlock_task_sighand(task, &flags);
    }
    }
//
// ptrace_check_attach - check whether ptracee is ready for ptrace operation
// @child: ptracee to check for
// @ignore_state: don't check whether @child is currently %TASK_TRACED
//
// Check whether @child is being ptraced by %current and ready for further
// ptrace operations.  If @ignore_state is %false, @child also should be in
// %TASK_TRACED state and on return the child is guaranteed to be traced
// and not executing.  If @ignore_state is %true, @child can be in any
// state.
//
// CONTEXT:
// Grabs and releases tasklist_lock and @child->sighand->siglock.
//
// RETURNS:
// 0 on success, -ESRCH if %child is not ready.
//
#[no_mangle]
unsafe extern "C" fn ptrace_check_attach(child: *mut task_struct, ignore_state: bool) -> c_int {
pub static mut ret: c_int = 0;
//
// We take the read lock around doing both checks to close a
// possible race where someone else was tracing our child and
// detached between these two checks.  After this locked check,
// we are sure that this is our traced child and that can only
// be changed by us so it's not changing right after this.
//
    read_lock(&tasklist_lock);
    if (child.ptrace && child.parent == current) {
//
// child->sighand can't be NULL, release_task()
// does ptrace_unlink() before __exit_signal().
//
    if (ignore_state || ptrace_freeze_traced(child)) {
    ret = 0;
    }
    }
    read_unlock(&tasklist_lock);
    if (!ret && !ignore_state &&
    WARN_ON_ONCE!(!wait_task_inactive(child, __TASK_TRACED|TASK_FROZEN))) {
    ret = -ESRCH;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ptrace_has_cap(ns: *mut user_namespace, mode: c_uint) -> bool {
    if (mode & PTRACE_MODE_NOAUDIT) {
    return ns_capable_noaudit(ns, CAP_SYS_PTRACE);
    }
    return ns_capable(ns, CAP_SYS_PTRACE);
    }
#[no_mangle]
unsafe extern "C" fn task_still_dumpable(task: *mut task_struct, mode: c_uint) -> bool {
pub static mut exec_state: *mut c_void = core::ptr::null_mut();
    guard(rcu)();
    exec_state = task_exec_state_rcu(task);
    if (READ_ONCE(exec_state.dumpable) == TASK_DUMPABLE_OWNER) {
    return true;
    }
    return ptrace_has_cap(exec_state.user_ns, mode);
    }
// Returns 0 on success, -errno on denial.
#[no_mangle]
unsafe extern "C" fn __ptrace_may_access(task: *mut task_struct, mode: c_uint) -> c_int {
    let mut cred = current_cred(), *tcred;
    let mut caller_uid;
    let mut caller_gid;
    if (!(mode & PTRACE_MODE_FSCREDS) == !(mode & PTRACE_MODE_REALCREDS)) {
    WARN(1, "denying ptrace access check without PTRACE_MODE_*CREDS\n");
    return -EPERM;
    }
// May we inspect the given task?
// This check is used both for attaching with ptrace
// and for allowing access to sensitive information in /proc.
//
// ptrace_attach denies several cases that /proc allows
// because setting up the necessary parent/child relationship
// or halting the specified task is impossible.
//
// Don't let security modules deny introspection
    if (same_thread_group(task, current)) {
    return 0;
    }
    rcu_read_lock();
    if (mode & PTRACE_MODE_FSCREDS) {
    caller_uid = cred.fsuid;
    caller_gid = cred.fsgid;
    } else {
//
// Using the euid would make more sense here, but something
// in userland might rely on the old behavior, and this
// shouldn't be a security problem since
// PTRACE_MODE_REALCREDS implies that the caller explicitly
// used a syscall that requests access to another process
// (and not a filesystem syscall to procfs).
//
    caller_uid = cred.uid;
    caller_gid = cred.gid;
    }
    tcred = __task_cred(task);
    if (uid_eq(caller_uid, tcred.euid) &&
    uid_eq(caller_uid, tcred.suid) &&
    uid_eq(caller_uid, tcred.uid)  &&
    gid_eq(caller_gid, tcred.egid) &&
    gid_eq(caller_gid, tcred.sgid) &&
    gid_eq(caller_gid, tcred.gid)) {
// goto;
    }
    if (ptrace_has_cap(tcred.user_ns, mode)) {
// goto;
    }
    rcu_read_unlock();
    return -EPERM;
// label;
    rcu_read_unlock();
//
// If a task drops privileges and becomes nondumpable (through a syscall
// like setresuid()) while we are trying to access it, we must ensure
// that the dumpability is read after the credentials; otherwise,
// we may be able to attach to a task that we shouldn't be able to
// attach to (as if the task had dropped privileges without becoming
// nondumpable).
// Pairs with a write barrier in commit_creds().
//
    smp_rmb();
    if (!task_still_dumpable(task, mode)) {
    return -EPERM;
    }
    return security_ptrace_access_check(task, mode);
    }
#[no_mangle]
pub unsafe extern "C" fn ptrace_may_access(task: *mut task_struct, mode: c_uint) -> bool {
    let mut err = 0;
    task_lock(task);
    err = __ptrace_may_access(task, mode);
    task_unlock(task);
    return !err;
    }
#[no_mangle]
unsafe extern "C" fn check_ptrace_options(data: c_ulong) -> c_int {
    if (data & ~(unsigned long)PTRACE_O_MASK) {
    return -EINVAL;
    }
    if (unlikely(data & PTRACE_O_SUSPEND_SECCOMP)) {
    if (!IS_ENABLED!(CONFIG_CHECKPOINT_RESTORE) ||
    !IS_ENABLED!(CONFIG_SECCOMP)) {
    return -EINVAL;
    }
    if (!capable(CAP_SYS_ADMIN)) {
    return -EPERM;
    }
    if (seccomp_mode(&current.seccomp) != SECCOMP_MODE_DISABLED ||
    current.ptrace & PT_SUSPEND_SECCOMP) {
    return -EPERM;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ptrace_set_stopped(task: *mut task_struct, seize: bool) {
    guard(spinlock)(&task.sighand.siglock);
// SEIZE doesn't trap tracee on attach
    if (!seize) {
    send_signal_locked(SIGSTOP, SEND_SIG_PRIV, task, PIDTYPE_PID);
    }
//
// If the task is already STOPPED, set JOBCTL_TRAP_STOP and
// TRAPPING, and kick it so that it transits to TRACED.  TRAPPING
// will be cleared if the child completes the transition or any
// event which clears the group stop states happens.  We'll wait
// for the transition to complete before returning from this
// function.
//
// This hides STOPPED -> RUNNING -> TRACED transition from the
// attaching thread but a different thread in the same group can
// still observe the transient RUNNING state.  IOW, if another
// thread's WNOHANG wait(2) on the stopped tracee races against
// ATTACH, the wait(2) may fail due to the transient RUNNING.
//
// The following task_is_stopped() test is safe as both transitions
// in and out of STOPPED are protected by siglock.
//
    if (task_is_stopped(task) &&
    task_set_jobctl_pending(task, JOBCTL_TRAP_STOP | JOBCTL_TRAPPING)) {
    task.jobctl &= ~JOBCTL_STOPPED;
    signal_wake_up_state(task, __TASK_STOPPED);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ptrace_attach(task: *mut task_struct, request: c_long, addr: c_ulong, flags: c_ulong) -> c_int {
pub static mut seize: bool = false;
    let mut retval = 0;
    if (seize) {
    if (addr != 0) {
    return -EIO;
    }
//
// This duplicates the check in check_ptrace_options() because
// ptrace_attach() and ptrace_setoptions() have historically
// used different error codes for unknown ptrace options.
//
    if (flags & ~(unsigned long)PTRACE_O_MASK) {
    return -EIO;
    }
    retval = check_ptrace_options(flags);
    if (retval) {
    return retval;
    }
    flags = PT_PTRACED | PT_SEIZED | (flags << PT_OPT_FLAG_SHIFT);
    } else {
    flags = PT_PTRACED;
    }
    audit_ptrace(task);
    if (unlikely(task.flags & PF_KTHREAD)) {
    return -EPERM;
    }
    if (same_thread_group(task, current)) {
    return -EPERM;
    }
//
// Protect exec's credential calculations against our interference;
// SUID, SGID and LSM creds get determined differently
// under ptrace.
//
    scoped_cond_guard (mutex_intr, return -ERESTARTNOINTR,
    &task.signal.cred_guard_mutex) {
    scoped_guard (task_lock, task) {
    retval = __ptrace_may_access(task, PTRACE_MODE_ATTACH_REALCREDS);
    if (retval) {
    return retval;
    }
    }
    scoped_guard (write_lock_irq, &tasklist_lock) {
    if (unlikely(task.exit_state)) {
    return -EPERM;
    }
    if (task.ptrace) {
    return -EPERM;
    }
    task.ptrace = flags;
    ptrace_link(task, current);
    ptrace_set_stopped(task, seize);
    }
    }
//
// We do not bother to change retval or clear JOBCTL_TRAPPING
// if wait_on_bit() was interrupted by SIGKILL. The tracer will
// not return to user-mode, it will exit and clear this bit in
// __ptrace_unlink() if it wasn't already cleared by the tracee;
// and until then nobody can ptrace this task.
//
    wait_on_bit(&task.jobctl, JOBCTL_TRAPPING_BIT, TASK_KILLABLE);
    proc_ptrace_connector(task, PTRACE_ATTACH);
    return 0;
    }
//
// ptrace_traceme  --  helper for PTRACE_TRACEME
//
// Performs checks and sets PT_PTRACED.
// Should be used by all ptrace implementations for PTRACE_TRACEME.
//
#[no_mangle]
unsafe extern "C" fn ptrace_traceme() -> c_int {
pub static mut ret: c_int = 0;
    write_lock_irq(&tasklist_lock);
// Are we already being traced?
    if (!current.ptrace) {
    ret = security_ptrace_traceme(current.parent);
//
// Check PF_EXITING to ensure ->real_parent has not passed
// exit_ptrace(). Otherwise we don't report the error but
// pretend ->real_parent untraces us right after return.
//
    if (!ret && !(current.real_parent.flags & PF_EXITING)) {
    current.ptrace = PT_PTRACED;
    ptrace_link(current, current.real_parent);
    }
    }
    write_unlock_irq(&tasklist_lock);
    return ret;
    }
//
// Called with irqs disabled, returns true if childs should reap themselves.
//
#[no_mangle]
unsafe extern "C" fn ignoring_children(sigh: *mut sighand_struct) -> c_int {
    let mut ret = 0;
    spin_lock(&sigh.siglock);
    ret = (sigh.action[SIGCHLD-1].sa.sa_handler == SIG_IGN) ||
    (sigh.action[SIGCHLD-1].sa.sa_flags & SA_NOCLDWAIT);
    spin_unlock(&sigh.siglock);
    return ret;
    }
//
// Called with tasklist_lock held for writing.
// Unlink a traced task, and clean it up if it was a traced zombie.
// Return true if it needs to be reaped with release_task().
// (We can't call release_task() here because we already hold tasklist_lock.)
//
// If it's a zombie, our attachedness prevented normal parent notification
// or self-reaping.  Do notification now if it would have happened earlier.
// If it should reap itself, return true.
//
// If it's our own child, there is no notification to do. But if our normal
// children self-reap, then this child was prevented by ptrace and we must
// reap it now, in that case we must also wake up sub-threads sleeping in
// do_wait().
//
#[no_mangle]
unsafe extern "C" fn __ptrace_detach(tracer: *mut task_struct, p: *mut task_struct) -> bool {
    let mut dead = 0;
    __ptrace_unlink(p);
    if (p.exit_state != EXIT_ZOMBIE) {
    return false;
    }
    dead = !thread_group_leader(p);
    if (!dead && thread_group_empty(p)) {
    if (!same_thread_group(p.real_parent, tracer)) {
    dead = do_notify_parent(p, p.exit_signal);
    }
    else if (ignoring_children(tracer.sighand) ||
    p.signal.autoreap) {
    __wake_up_parent(p, tracer);
    dead = true;
    }
    }
// Mark it as in the process of being reaped.
    if (dead) {
    p.exit_state = EXIT_DEAD;
    }
    return dead;
    }
#[no_mangle]
unsafe extern "C" fn ptrace_detach(child: *mut task_struct, data: c_uint) -> c_int {
    if (!valid_signal(data)) {
    return -EIO;
    }
// Architecture-specific hardware disable ..
    ptrace_disable(child);
    write_lock_irq(&tasklist_lock);
//
// We rely on ptrace_freeze_traced(). It can't be killed and
// untraced by another thread, it can't be a zombie.
//
    WARN_ON!(!child.ptrace || child.exit_state);
//
// tasklist_lock avoids the race with wait_task_stopped(), see
// the comment in ptrace_resume().
//
    child.exit_code = data;
    __ptrace_detach(current, child);
    write_unlock_irq(&tasklist_lock);
    proc_ptrace_connector(child, PTRACE_DETACH);
    return 0;
    }
//
// Detach all tasks we were using ptrace on. Called with tasklist held
// for writing.
//
#[no_mangle]
pub unsafe extern "C" fn exit_ptrace(tracer: *mut task_struct, dead: *mut list_head) {
    let mut p = core::ptr::null_mut();
    let mut n = core::ptr::null_mut();
    list_for_each_entry_safe(p, n, &tracer.ptraced, ptrace_entry) {
    if (unlikely(p.ptrace & PT_EXITKILL)) {
    send_sig_info(SIGKILL, SEND_SIG_PRIV, p);
    }
    if (__ptrace_detach(tracer, p)) {
    list_add(&p.ptrace_entry, dead);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ptrace_readdata(tsk: *mut task_struct, src: c_ulong, dst: *mut char , len: c_int) -> c_int {
pub static mut copied: c_int = 0;
    while (len > 0) {
    char buf[128];
    let mut this_len = 0;
    let mut retval = 0;
    this_len = (len > sizeof!(buf)) ? sizeof!(buf) : len;
    retval = ptrace_access_vm(tsk, src, buf, this_len, FOLL_FORCE);
    if (!retval) {
    if (copied) {
    break;
    }
    return -EIO;
    }
    if (copy_to_user(dst, buf, retval)) {
    return -EFAULT;
    }
    copied += retval;
    src += retval;
    dst += retval;
    len -= retval;
    }
    return copied;
    }
#[no_mangle]
pub unsafe extern "C" fn ptrace_writedata(tsk: *mut task_struct, src: *mut char , dst: c_ulong, len: c_int) -> c_int {
pub static mut copied: c_int = 0;
    while (len > 0) {
    char buf[128];
    let mut this_len = 0;
    let mut retval = 0;
    this_len = (len > sizeof!(buf)) ? sizeof!(buf) : len;
    if (copy_from_user(buf, src, this_len)) {
    return -EFAULT;
    }
    retval = ptrace_access_vm(tsk, dst, buf, this_len,
    FOLL_FORCE | FOLL_WRITE);
    if (!retval) {
    if (copied) {
    break;
    }
    return -EIO;
    }
    copied += retval;
    src += retval;
    dst += retval;
    len -= retval;
    }
    return copied;
    }
#[no_mangle]
unsafe extern "C" fn ptrace_setoptions(child: *mut task_struct, data: c_ulong) -> c_int {
    let mut flags: c_uint = 0;
    let mut ret = 0;
    ret = check_ptrace_options(data);
    if (ret) {
    return ret;
    }
// Avoid intermediate state when all opts are cleared
    flags = child.ptrace;
    flags &= ~(PTRACE_O_MASK << PT_OPT_FLAG_SHIFT);
    flags |= (data << PT_OPT_FLAG_SHIFT);
    child.ptrace = flags;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ptrace_getsiginfo(child: *mut task_struct, info: *mut kernel_siginfo_t) -> c_int {
    let mut flags = 0;
pub static mut error: c_int = 0;
    if (lock_task_sighand(child, &flags)) {
    error = -EINVAL;
    if (likely(child.last_siginfo != core::ptr::null_mut())) {
    copy_siginfo(info, child.last_siginfo);
    error = 0;
    }
    unlock_task_sighand(child, &flags);
    }
    return error;
    }
#[no_mangle]
unsafe extern "C" fn ptrace_setsiginfo(child: *mut task_struct, info: *const kernel_siginfo_t) -> c_int {
    let mut flags = 0;
pub static mut error: c_int = 0;
    if (lock_task_sighand(child, &flags)) {
    error = -EINVAL;
    if (likely(child.last_siginfo != core::ptr::null_mut())) {
    copy_siginfo(child.last_siginfo, info);
    error = 0;
    }
    unlock_task_sighand(child, &flags);
    }
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn ptrace_peek_siginfo(child: *mut task_struct, addr: c_ulong, data: c_ulong) -> c_int {
pub static mut arg: usize = 0;
pub static mut pending: *mut c_void = core::ptr::null_mut();
pub static mut q: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    let mut i = 0;
    ret = copy_from_user(&arg,  addr,
    sizeof!(ptrace_peeksiginfo_args));
    if (ret) {
    return -EFAULT;
    }
    if (arg.flags & ~PTRACE_PEEKSIGINFO_SHARED) {
    return -EINVAL; /* unknown flags */
    }
    if (arg.nr < 0) {
    return -EINVAL;
    }
// Ensure arg.off fits in an unsigned long
    if (arg.off > ULONG_MAX) {
    return 0;
    }
    if (arg.flags & PTRACE_PEEKSIGINFO_SHARED) {
    pending = &child.signal.shared_pending;
    }
    else {
    pending = &child.pending;
    }
    while (i < arg.nr) {
    let mut info;
pub static mut off: c_ulong = 0;
pub static mut found: bool = false;
    spin_lock_irq(&child.sighand.siglock);
    list_for_each_entry(q, &pending.list, list) {
    if (!off--) {
    found = true;
    copy_siginfo(&info, &q.info);
    break;
    }
    }
    spin_unlock_irq(&child.sighand.siglock);
    if (!found) /* beyond the end of the list */ {
    break;
    }

    if (unlikely(in_compat_syscall())) {
    let mut uinfo = compat_ptr(data);
    if (copy_siginfo_to_user32(uinfo, &info)) {
    ret = -EFAULT;
    break;
    }
    } else {

    {
    }
    let mut uinfo =  data;
    if (copy_siginfo_to_user(uinfo, &info)) {
    ret = -EFAULT;
    break;
    }
    }
    data += sizeof!(siginfo_t);
    i += 1;
    if (signal_pending(current)) {
    break;
    }
    cond_resched();
    }
    if (i > 0) {
    return i;
    }
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn ptrace_get_rseq_configuration(task: *mut task_struct, size: c_ulong, data: *mut c_void) -> c_long {
pub static mut ptrace_rseq_configuration: usize = 0;
    size = min_t(unsigned long, size, sizeof!(conf));
    if (copy_to_user(data, &conf, size)) {
    return -EFAULT;
    }
    return sizeof!(conf);
    }

pub const is_singleblock(request): c_int = 0;

pub const is_sysemu_singlestep(request): c_int = 0;

#[no_mangle]
pub unsafe extern "C" fn ptrace_resume(child: *mut task_struct, request: c_long, data: c_ulong) -> c_int {
    if (!valid_signal(data)) {
    return -EIO;
    }
    if (request == PTRACE_SYSCALL) {
    set_task_syscall_work(child, SYSCALL_TRACE);
    }
    else {
    clear_task_syscall_work(child, SYSCALL_TRACE);
    }

    if (request == PTRACE_SYSEMU || request == PTRACE_SYSEMU_SINGLESTEP) {
    set_task_syscall_work(child, SYSCALL_EMU);
    }
    else {
    clear_task_syscall_work(child, SYSCALL_EMU);
    }

    if (is_singleblock(request)) {
    if (unlikely(!arch_has_block_step())) {
    return -EIO;
    }
    user_enable_block_step(child);
    } else if (is_singlestep(request) || is_sysemu_singlestep(request)) {
    if (unlikely(!arch_has_single_step())) {
    return -EIO;
    }
    user_enable_single_step(child);
    } else {
    user_disable_single_step(child);
    }
//
// Change ->exit_code and ->state under siglock to avoid the race
// with wait_task_stopped() in between; a non-zero ->exit_code will
// wrongly look like another report from tracee.
//
// Note that we need siglock even if ->exit_code == data and/or this
// status was not reported yet, the new status must not be cleared by
// wait_task_stopped() after resume.
//
    spin_lock_irq(&child.sighand.siglock);
    child.exit_code = data;
    child.jobctl &= ~JOBCTL_TRACED;
    wake_up_state(child, __TASK_TRACED);
    spin_unlock_irq(&child.sighand.siglock);
    return 0;
    }

    static const struct user_regset *
    find_regset(const struct user_regset_view *view, unsigned int type)
    {
pub static mut regset: *mut c_void = core::ptr::null_mut();
    let mut n = 0;
    while (n < view.n) {
    regset = view.regsets + n;
    if (regset.core_note_type == type) {
    return regset;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn ptrace_regset(task: *mut task_struct, req: c_int, type: c_uint, kiov: *mut iovec) -> c_int {
    let mut view = task_user_regset_view(task);
    let mut regset = find_regset(view, type);
    let mut regset_no = 0;
    if (!regset || (kiov.iov_len % regset.size) != 0) {
    return -EINVAL;
    }
    regset_no = regset - view.regsets;
    kiov.iov_len = min(kiov.iov_len,
    (__kernel_size_t) (regset.n * regset.size));
    if (req == PTRACE_GETREGSET) {
    return copy_regset_to_user(task, view, regset_no, 0,
    kiov.iov_len, kiov.iov_base);
    }
    else {
    return copy_regset_from_user(task, view, regset_no, 0,
    kiov.iov_len, kiov.iov_base);
    }
    }
//
// This is declared in linux/regset.h and defined in machine-dependent
// code.  We put the export here, near the primary machine-neutral use,
// to ensure no machine forgets it.
//
    EXPORT_SYMBOL_GPL(task_user_regset_view);
#[no_mangle]
pub unsafe extern "C" fn ptrace_get_syscall_info_entry(child: *mut task_struct, regs: *mut pt_regs, info: *mut ptrace_syscall_info) -> c_ulong {
    unsigned long args[ARRAY_SIZE!(info.entry.args)];
    let mut i = 0;
    info.entry.nr = syscall_get_nr(child, regs);
    syscall_get_arguments(child, regs, args);
    for (i = 0; i < ARRAY_SIZE!(args); i++) {
    info.entry.args[i] = args[i];
    }
// args is the last field in struct ptrace_syscall_info.entry
    return offsetofend(ptrace_syscall_info, entry.args);
    }
#[no_mangle]
pub unsafe extern "C" fn ptrace_get_syscall_info_seccomp(child: *mut task_struct, regs: *mut pt_regs, info: *mut ptrace_syscall_info) -> c_ulong {
//
// As struct ptrace_syscall_info.entry is currently a subset
// of struct ptrace_syscall_info.seccomp, it makes sense to
// initialize that subset using ptrace_get_syscall_info_entry().
// This can be reconsidered in the future if these structures
// diverge significantly enough.
//
    ptrace_get_syscall_info_entry(child, regs, info);
    info.seccomp.ret_data = child.ptrace_message;
//
// ret_data is the last non-reserved field
// in struct ptrace_syscall_info.seccomp
//
    return offsetofend(ptrace_syscall_info, seccomp.ret_data);
    }
#[no_mangle]
pub unsafe extern "C" fn ptrace_get_syscall_info_exit(child: *mut task_struct, regs: *mut pt_regs, info: *mut ptrace_syscall_info) -> c_ulong {
    info.exit.rval = syscall_get_error(child, regs);
    info.exit.is_error = !!info.exit.rval;
    if (!info.exit.is_error) {
    info.exit.rval = syscall_get_return_value(child, regs);
    }
// is_error is the last field in struct ptrace_syscall_info.exit
    return offsetofend(ptrace_syscall_info, exit.is_error);
    }
#[no_mangle]
pub unsafe extern "C" fn ptrace_get_syscall_info_op(child: *mut task_struct) -> c_int {
//
// This does not need lock_task_sighand() to access
// child->last_siginfo because ptrace_freeze_traced()
// called earlier by ptrace_check_attach() ensures that
// the tracee cannot go away and clear its last_siginfo.
//
    match (child.last_siginfo ? child.last_siginfo.si_code : 0) {
    SIGTRAP | 0x80 => {
    match (child.ptrace_message) {
    PTRACE_EVENTMSG_SYSCALL_ENTRY => {
    return PTRACE_SYSCALL_INFO_ENTRY;
    }
    PTRACE_EVENTMSG_SYSCALL_EXIT => {
    return PTRACE_SYSCALL_INFO_EXIT;
    }
    _ => {
    return PTRACE_SYSCALL_INFO_NONE;
    }
    }
    case SIGTRAP | (PTRACE_EVENT_SECCOMP << 8):
    return PTRACE_SYSCALL_INFO_SECCOMP;
// label;
    return PTRACE_SYSCALL_INFO_NONE;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ptrace_get_syscall_info(child: *mut task_struct, user_size: c_ulong, datavp: *mut c_void) -> c_int {
    let mut regs = task_pt_regs(child);
pub static mut ptrace_syscall_info: usize = 0;
pub static mut actual_size: c_ulong = 0;
    let mut write_size = 0;
    match (info.op) {
    PTRACE_SYSCALL_INFO_ENTRY => {
    actual_size = ptrace_get_syscall_info_entry(child, regs, &info);
    // break;
    }
    PTRACE_SYSCALL_INFO_EXIT => {
    actual_size = ptrace_get_syscall_info_exit(child, regs, &info);
    // break;
    }
    PTRACE_SYSCALL_INFO_SECCOMP => {
    actual_size = ptrace_get_syscall_info_seccomp(child, regs, &info);
    // break;
    }
    }
    write_size = min(actual_size, user_size);
    return copy_to_user(datavp, &info, write_size) ? -EFAULT : actual_size;
    }
#[no_mangle]
pub unsafe extern "C" fn ptrace_set_syscall_info_entry(child: *mut task_struct, regs: *mut pt_regs, info: *mut ptrace_syscall_info) -> c_int {
    unsigned long args[ARRAY_SIZE!(info.entry.args)];
pub static mut nr: c_int = 0;
    let mut i = 0;
//
// Check that the syscall number specified in info->entry.nr
// is either a value of type "int" or a sign-extended value
// of type "int".
//
    if (nr != info.entry.nr) {
    return -ERANGE;
    }
    while (i < ARRAY_SIZE!(args)) {
    args[i] = info.entry.args[i];
//
// Check that the syscall argument specified in
// info->entry.args[i] is either a value of type
// "unsigned long" or a sign-extended value of type "long".
//
    if (args[i] != info.entry.args[i]) {
    return -ERANGE;
    }
    }
    syscall_set_nr(child, regs, nr);
//
// If the syscall number is set to -1, setting syscall arguments is not
// just pointless, it would also clobber the syscall return value on
// those architectures that share the same register both for the first
// argument of syscall and its return value.
//
    if (nr != -1) {
    syscall_set_arguments(child, regs, args);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ptrace_set_syscall_info_seccomp(child: *mut task_struct, regs: *mut pt_regs, info: *mut ptrace_syscall_info) -> c_int {
//
// info->entry is currently a subset of info->seccomp,
// info->seccomp.ret_data is currently ignored.
//
    return ptrace_set_syscall_info_entry(child, regs, info);
    }
#[no_mangle]
pub unsafe extern "C" fn ptrace_set_syscall_info_exit(child: *mut task_struct, regs: *mut pt_regs, info: *mut ptrace_syscall_info) -> c_int {
pub static mut rval: c_long = 0;
//
// Check that the return value specified in info->exit.rval
// is either a value of type "long" or a sign-extended value
// of type "long".
//
    if (rval != info.exit.rval) {
    return -ERANGE;
    }
    if (info.exit.is_error) {
    syscall_set_return_value(child, regs, rval, 0);
    }
    else {
    syscall_set_return_value(child, regs, 0, rval);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ptrace_set_syscall_info(child: *mut task_struct, user_size: c_ulong, datavp: *mut c_void) -> c_int {
    let mut regs = task_pt_regs(child);
pub static mut info: usize = 0;
    if (user_size < sizeof!(info)) {
    return -EINVAL;
    }
//
// The compatibility is tracked by info.op and info.flags: if user-space
// does not instruct us to use unknown extra bits from future versions
// of ptrace_syscall_info, we are not going to read them either.
//
    if (copy_from_user(&info, datavp, sizeof!(info))) {
    return -EFAULT;
    }
// Reserved for future use.
    if (info.flags || info.reserved) {
    return -EINVAL;
    }
// Changing the type of the system call stop is not supported yet.
    if (ptrace_get_syscall_info_op(child) != info.op) {
    return -EINVAL;
    }
    match (info.op) {
    PTRACE_SYSCALL_INFO_ENTRY => {
    return ptrace_set_syscall_info_entry(child, regs, &info);
    }
    PTRACE_SYSCALL_INFO_EXIT => {
    return ptrace_set_syscall_info_exit(child, regs, &info);
    }
    PTRACE_SYSCALL_INFO_SECCOMP => {
    return ptrace_set_syscall_info_seccomp(child, regs, &info);
    }
    _ => {
// Other types of system call stops are not supported yet.
    return -EINVAL;
    }
    }
    }

#[no_mangle]
pub unsafe extern "C" fn ptrace_request(child: *mut task_struct, request: c_long, addr: c_ulong, data: c_ulong) -> c_int {
pub static mut seized: bool = false;
pub static mut ret: c_int = 0;
    kernel_siginfo_t siginfo, *si;
    let mut datavp =  data;
    let mut datalp = datavp;
    let mut flags = 0;
    match (request) {
    PTRACE_PEEKTEXT => {
    }
    PTRACE_PEEKDATA => {
    return generic_ptrace_peekdata(child, addr, data);
    }
    PTRACE_POKETEXT => {
    }
    PTRACE_POKEDATA => {
    return generic_ptrace_pokedata(child, addr, data);

    }
    PTRACE_OLDSETOPTIONS => {

    }
    PTRACE_SETOPTIONS => {
    ret = ptrace_setoptions(child, data);
    // break;
    }
    PTRACE_GETEVENTMSG => {
    ret = put_user(child.ptrace_message, datalp);
    // break;
    }
    PTRACE_PEEKSIGINFO => {
    ret = ptrace_peek_siginfo(child, addr, data);
    // break;
    }
    PTRACE_GETSIGINFO => {
    ret = ptrace_getsiginfo(child, &siginfo);
    if (!ret) {
    ret = copy_siginfo_to_user(datavp, &siginfo);
    }
    // break;
    }
    PTRACE_SETSIGINFO => {
    ret = copy_siginfo_from_user(&siginfo, datavp);
    if (!ret) {
    ret = ptrace_setsiginfo(child, &siginfo);
    }
    // break;
    }
    PTRACE_GETSIGMASK => {
pub static mut mask: *mut c_void = core::ptr::null_mut();
    if (addr != sizeof!(sigset_t)) {
    ret = -EINVAL;
    // break;
    }
    if (test_tsk_restore_sigmask(child)) {
    mask = &child.saved_sigmask;
    }
    else {
    mask = &child.blocked;
    }
    if (copy_to_user(datavp, mask, sizeof!(sigset_t))) {
    ret = -EFAULT;
    }
    else {
    ret = 0;
    }
    // break;
    }
    }
    case PTRACE_SETSIGMASK: {
    let mut new_set;
    if (addr != sizeof!(sigset_t)) {
    ret = -EINVAL;
    break;
    }
    if (copy_from_user(&new_set, datavp, sizeof!(sigset_t))) {
    ret = -EFAULT;
    break;
    }
    sigdelsetmask(&new_set, sigmask(SIGKILL)|sigmask(SIGSTOP));
//
// Every thread does recalc_sigpending() after resume, so
// retarget_shared_pending() and recalc_sigpending() are not
// called here.
//
    spin_lock_irq(&child.sighand.siglock);
    child.blocked = new_set;
    spin_unlock_irq(&child.sighand.siglock);
    clear_tsk_restore_sigmask(child);
    ret = 0;
    break;
    }
    case PTRACE_INTERRUPT:
//
// Stop tracee without any side-effect on signal or job
// control.  At least one trap is guaranteed to happen
// after this request.  If @child is already trapped, the
// current trap is not disturbed and another trap will
// happen after the current trap is ended with PTRACE_CONT.
//
// The actual trap might not be PTRACE_EVENT_STOP trap but
// the pending condition is cleared regardless.
//
    if (unlikely(!seized || !lock_task_sighand(child, &flags))) {
    break;
    }
//
// INTERRUPT doesn't disturb existing trap sans one
// exception.  If ptracer issued LISTEN for the current
// STOP, this INTERRUPT should clear LISTEN and re-trap
// tracee into STOP.
//
    if (likely(task_set_jobctl_pending(child, JOBCTL_TRAP_STOP))) {
    ptrace_signal_wake_up(child, child.jobctl & JOBCTL_LISTENING);
    }
    unlock_task_sighand(child, &flags);
    ret = 0;
    break;
    case PTRACE_LISTEN:
//
// Listen for events.  Tracee must be in STOP.  It's not
// resumed per-se but is not considered to be in TRACED by
// wait(2) or ptrace(2).  If an async event (e.g. group
// stop state change) happens, tracee will enter STOP trap
// again.  Alternatively, ptracer can issue INTERRUPT to
// finish listening and re-trap tracee into STOP.
//
    if (unlikely(!seized || !lock_task_sighand(child, &flags))) {
    break;
    }
    si = child.last_siginfo;
    if (likely(si && (si.si_code >> 8) == PTRACE_EVENT_STOP)) {
    child.jobctl |= JOBCTL_LISTENING;
//
// If NOTIFY is set, it means event happened between
// start of this trap and now.  Trigger re-trap.
//
    if (child.jobctl & JOBCTL_TRAP_NOTIFY) {
    ptrace_signal_wake_up(child, true);
    }
    ret = 0;
    }
    unlock_task_sighand(child, &flags);
    break;
    case PTRACE_DETACH:	 /* detach a process that was attached. */
    ret = ptrace_detach(child, data);
    break;

    case PTRACE_GETFDPIC: {
    let mut mm = get_task_mm(child);
pub static mut tmp: c_ulong = 0;
    ret = -ESRCH;
    if (!mm) {
    break;
    }
    match (addr) {
    PTRACE_GETFDPIC_EXEC => {
    tmp = mm.context.exec_fdpic_loadmap;
    // break;
    }
    PTRACE_GETFDPIC_INTERP => {
    tmp = mm.context.interp_fdpic_loadmap;
    // break;
    }
    _ => {
    // break;
    }
    }
    mmput(mm);
    ret = put_user(tmp, datalp);
    break;
    }

    case PTRACE_SINGLESTEP:

    case PTRACE_SINGLEBLOCK:

    case PTRACE_SYSEMU:
    case PTRACE_SYSEMU_SINGLESTEP:

    case PTRACE_SYSCALL:
    case PTRACE_CONT:
    return ptrace_resume(child, request, data);
    case PTRACE_KILL:
    send_sig_info(SIGKILL, SEND_SIG_NOINFO, child);
    return 0;

    case PTRACE_GETREGSET:
    case PTRACE_SETREGSET: {
pub static mut kiov: usize = 0;
    let mut uiov = datavp;
    if (!access_ok(uiov, sizeof!(*uiov))) {
    return -EFAULT;
    }
    if (__get_user(kiov.iov_base, &uiov.iov_base) ||
    __get_user(kiov.iov_len, &uiov.iov_len)) {
    return -EFAULT;
    }
    ret = ptrace_regset(child, request, addr, &kiov);
    if (!ret) {
    ret = __put_user(kiov.iov_len, &uiov.iov_len);
    }
    break;
    }
    case PTRACE_GET_SYSCALL_INFO:
    ret = ptrace_get_syscall_info(child, addr, datavp);
    break;
    case PTRACE_SET_SYSCALL_INFO:
    ret = ptrace_set_syscall_info(child, addr, datavp);
    break;

    case PTRACE_SECCOMP_GET_FILTER:
    ret = seccomp_get_filter(child, addr, datavp);
    break;
    case PTRACE_SECCOMP_GET_METADATA:
    ret = seccomp_get_metadata(child, addr, datavp);
    break;

    case PTRACE_GET_RSEQ_CONFIGURATION:
    ret = ptrace_get_rseq_configuration(child, addr, datavp);
    break;

    case PTRACE_SET_SYSCALL_USER_DISPATCH_CONFIG:
    ret = syscall_user_dispatch_set_config(child, addr, datavp);
    break;
    case PTRACE_GET_SYSCALL_USER_DISPATCH_CONFIG:
    ret = syscall_user_dispatch_get_config(child, addr, datavp);
    break;
// label;
    break;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_ptrace(request: usize, pid: usize, addr: usize, data: usize) -> c_long {
pub static mut child: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (request == PTRACE_TRACEME) {
    ret = ptrace_traceme();
// goto;
    }
    child = find_get_task_by_vpid(pid);
    if (!child) {
    ret = -ESRCH;
// goto;
    }
    if (request == PTRACE_ATTACH || request == PTRACE_SEIZE) {
    ret = ptrace_attach(child, request, addr, data);
// goto;
    }
    ret = ptrace_check_attach(child, request == PTRACE_KILL ||
    request == PTRACE_INTERRUPT);
    if (ret < 0) {
// goto;
    }
    ret = arch_ptrace(child, request, addr, data);
    if (ret || request != PTRACE_DETACH) {
    ptrace_unfreeze_traced(child);
    }
// label;
    put_task_struct(child);
// label;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn generic_ptrace_peekdata(tsk: *mut task_struct, addr: c_ulong, data: c_ulong) -> c_int {
    let mut tmp = 0;
    let mut copied = 0;
    copied = ptrace_access_vm(tsk, addr, &tmp, sizeof!(tmp), FOLL_FORCE);
    if (copied != sizeof!(tmp)) {
    return -EIO;
    }
    return put_user(tmp, data);
    }
#[no_mangle]
pub unsafe extern "C" fn generic_ptrace_pokedata(tsk: *mut task_struct, addr: c_ulong, data: c_ulong) -> c_int {
    let mut copied = 0;
    copied = ptrace_access_vm(tsk, addr, &data, sizeof!(data),
    FOLL_FORCE | FOLL_WRITE);
    return (copied == sizeof!(data)) ? 0 : -EIO;
    }

#[no_mangle]
pub unsafe extern "C" fn compat_ptrace_request(child: *mut task_struct, request: compat_long_t, addr: compat_ulong_t, data: compat_ulong_t) -> c_int {
    let mut datap = compat_ptr(data);
    let mut word;
    let mut siginfo;
    let mut ret = 0;
    match (request) {
    PTRACE_PEEKTEXT => {
    }
    PTRACE_PEEKDATA => {
    ret = ptrace_access_vm(child, addr, &word, sizeof!(word),
    FOLL_FORCE);
    if (ret != sizeof!(word)) {
    ret = -EIO;
    }
    else {
    ret = put_user(word, datap);
    }
    // break;
    }
    PTRACE_POKETEXT => {
    }
    PTRACE_POKEDATA => {
    ret = ptrace_access_vm(child, addr, &data, sizeof!(data),
    FOLL_FORCE | FOLL_WRITE);
    ret = (ret != sizeof!(data) ? -EIO : 0);
    // break;
    }
    PTRACE_GETEVENTMSG => {
    ret = put_user((compat_ulong_t) child.ptrace_message, datap);
    // break;
    }
    PTRACE_GETSIGINFO => {
    ret = ptrace_getsiginfo(child, &siginfo);
    if (!ret) {
    ret = copy_siginfo_to_user32(
     datap,
    &siginfo);
    }
    // break;
    }
    PTRACE_SETSIGINFO => {
    ret = copy_siginfo_from_user32(
    &siginfo,  datap);
    if (!ret) {
    ret = ptrace_setsiginfo(child, &siginfo);
    }
    // break;

    }
    PTRACE_GETREGSET => {
    }
    PTRACE_SETREGSET => {
    {
pub static mut kiov: usize = 0;
    let mut uiov =  datap;
    let mut ptr;
    let mut len;
    if (!access_ok(uiov, sizeof!(*uiov))) {
    return -EFAULT;
    }
    if (__get_user(ptr, &uiov.iov_base) ||
    __get_user(len, &uiov.iov_len)) {
    return -EFAULT;
    }
    kiov.iov_base = compat_ptr(ptr);
    kiov.iov_len = len;
    ret = ptrace_regset(child, request, addr, &kiov);
    if (!ret) {
    ret = __put_user(kiov.iov_len, &uiov.iov_len);
    }
    // break;
    }

    }
    _ => {
    ret = ptrace_request(child, request, addr, data);
    }
    }
    return ret;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: sys_ptrace
pub unsafe extern "C" fn sys_ptrace_dup(request: usize, pid: usize, addr: usize, data: usize) -> c_long {
pub static mut child: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (request == PTRACE_TRACEME) {
    ret = ptrace_traceme();
// goto;
    }
    child = find_get_task_by_vpid(pid);
    if (!child) {
    ret = -ESRCH;
// goto;
    }
    if (request == PTRACE_ATTACH || request == PTRACE_SEIZE) {
    ret = ptrace_attach(child, request, addr, data);
// goto;
    }
    ret = ptrace_check_attach(child, request == PTRACE_KILL ||
    request == PTRACE_INTERRUPT);
    if (!ret) {
    ret = compat_arch_ptrace(child, request, addr, data);
    if (ret || request != PTRACE_DETACH) {
    ptrace_unfreeze_traced(child);
    }
    }
// label;
    put_task_struct(child);
// label;
    return ret;
    }