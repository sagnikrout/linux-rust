//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_events_user.c
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
//
// Copyright (c) 2021, Microsoft Corporation.
//
// Authors:
// Beau Belgrave <beaub@linux.microsoft.com>
//

pub const FIELD_DEPTH_TYPE: c_int = 0;
pub const FIELD_DEPTH_NAME: c_int = 1;
pub const FIELD_DEPTH_SIZE: c_int = 2;
// Limit how long of an event name plus args within the subsystem.
pub const MAX_EVENT_DESC: c_int = 512;

pub const MAX_FIELD_ARRAY_SIZE: c_int = 1024;
//
// Internal bits (kernel side only) to keep track of connected probes:
// These are used when status is requested in text form about an event. These
// bits are compared against an internal byte on the event to determine which
// probes to print out to the user.
//
// These do not reflect the mapped bytes between the user and kernel space.
//

//
// Stores the system name, tables, and locks for a group of events. This
// allows isolation for events by various means.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_event_group {
    pub system_name: *mut c_char,
    pub system_multi_name: *mut c_char,
    pub node: hlist_node,
    pub reg_mutex: mutex,
    pub 8): DECLARE_HASHTABLE(register_table,,
// ID that moves forward within the group for multi-event names
    pub multi_id: u64,
}

// Group for init_user_ns mapping, top-most group
pub static mut init_group: *mut c_void = core::ptr::null_mut();
// Max allowed events for the whole system
pub static mut max_user_events: unsigned int = 32768;
// Current number of events on the whole system
    static unsigned int current_user_events;
//
// Stores per-event properties, as users register events
// within a file a user_event might be created if it does not
// already exist. These are globally used and their lifetime
// is tied to the refcnt member. These cannot go away until the
// refcnt reaches one.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_event {
    pub group: *mut user_event_group,
    pub reg_name: *mut c_char,
    pub tracepoint: tracepoint,
    pub call: trace_event_call,
    pub class: trace_event_class,
    pub devent: dyn_event,
    pub node: hlist_node,
    pub fields: list_head,
    pub validators: list_head,
    pub put_work: work_struct,
    pub refcnt: refcount_t,
    pub min_size: c_int,
    pub reg_flags: c_int,
    pub status: c_char,
}

//
// Stores per-mm/event properties that enable an address to be
// updated properly for each task. As tasks are forked, we use
// these to track enablement sites that are tied to an event.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_event_enabler {
    pub mm_enablers_link: list_head,
    pub event: *mut user_event,
    pub addr: c_ulong,
// Track enable bit, flags, etc. Aligned for bitops.
    pub values: c_ulong,
// Defer the event put and enabler free past an RCU grace period.
    pub put_rwork: rcu_work,
}

// Bits 0-5 are for the bit to update upon enable/disable (0-63 allowed)
pub const ENABLE_VAL_BIT_MASK: c_uint = 0x3F;
// Bit 6 is for faulting status of enablement
pub const ENABLE_VAL_FAULTING_BIT: c_int = 6;
// Bit 7 is for freeing status of enablement
pub const ENABLE_VAL_FREEING_BIT: c_int = 7;
// Bit 8 is for marking 32-bit on 64-bit
pub const ENABLE_VAL_32_ON_64_BIT: c_int = 8;

// Only duplicate the bit and compat values

// Used for asynchronous faulting in of pages
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_event_enabler_fault {
    pub work: work_struct,
    pub mm: *mut user_event_mm,
    pub enabler: *mut user_event_enabler,
    pub attempt: c_int,
}

pub static mut fault_cache: *mut c_void = core::ptr::null_mut();
// Global list of memory descriptors using user_events
pub static mut user_event_mms: usize = 0;
pub static mut user_event_mms_lock: usize = 0;
//
// Stores per-file events references, as users register events
// within a file this structure is modified and freed via RCU.
// The lifetime of this struct is tied to the lifetime of the file.
// These are not shared and only accessible by the file that created it.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_event_refs {
    pub rcu: rcu_head,
    pub count: c_int,
    pub events: [*mut user_event; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_event_file_info {
    pub group: *mut user_event_group,
    pub refs: *mut user_event_refs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_event_validator {
    pub user_event_link: list_head,
    pub offset: c_int,
    pub flags: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn align_addr_bit(addr: *mut c_ulong, bit: *mut c_int, flags: *mut c_ulong) {
    if (IS_ALIGNED(*addr, sizeof!(long))) {

// 32 bit on BE 64 bit requires a 32 bit offset when aligned.
    if (test_bit(ENABLE_VAL_32_ON_64_BIT, flags)) {
// bit += 32;
    }

    return;
    }
// addr = ALIGN_DOWN(*addr, sizeof!(long));
//
// We only support 32 and 64 bit values. The only time we need
// to align is a 32 bit value on a 64 bit kernel, which on LE
// is always 32 bits, and on BE requires no change when unaligned.
//

// bit += 32;

    }
    typedef void (*user_event_func_t) (user_event *user, iov_iter *i,
    void *tpdata, bool *faulted);
// forward_decl: user_event_parse;
// forward_decl: user_event_mm_get;
// forward_decl: user_event_mm_get_all;
// forward_decl: user_event_mm_put;
// forward_decl: destroy_user_event;
// forward_decl: user_fields_match;
#[no_mangle]
unsafe extern "C" fn user_event_key(name: *mut c_char) -> u32 {
    return jhash(name, strlen(name), 0);
    }
#[no_mangle]
unsafe extern "C" fn user_event_capable(reg_flags: u16) -> bool {
// Persistent events require CAP_PERFMON / CAP_SYS_ADMIN
    if (reg_flags & USER_EVENT_REG_PERSIST) {
    if (!perfmon_capable()) {
    return false;
    }
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn user_event_get(user: *mut user_event) -> *mut c_void {
    refcount_inc(&user.refcnt);
    return user;
    }
#[no_mangle]
unsafe extern "C" fn delayed_destroy_user_event(work: *mut work_struct) {
    let mut user = container_of!(work, user_event, put_work);
    mutex_lock(&event_mutex);
    if (!refcount_dec_and_test(&user.refcnt)) {
// goto;
    }
    if (destroy_user_event(user)) {
//
// The only reason this would fail here is if we cannot
// update the visibility of the event. In this case the
// event stays in the hashtable, waiting for someone to
// attempt to delete it later.
//
    pr_warn!("user_events: Unable to delete event\n");
    refcount_set(&user.refcnt, 1);
    }
// label;
    mutex_unlock(&event_mutex);
    }
#[no_mangle]
unsafe extern "C" fn user_event_put(user: *mut user_event, locked: bool) {
    let mut delete = 0;
    if (unlikely(!user)) {
    return;
    }
//
// When the event is not enabled for auto-delete there will always
// be at least 1 reference to the event. During the event creation
// we initially set the refcnt to 2 to achieve this. In those cases
// the caller must acquire event_mutex and after decrement check if
// the refcnt is 1, meaning this is the last reference. When auto
// delete is enabled, there will only be 1 ref, IE: refcnt will be
// only set to 1 during creation to allow the below checks to go
// through upon the last put. The last put must always be done with
// the event mutex held.
//
    if (!locked) {
    lockdep_assert_not_held(&event_mutex);
    delete = refcount_dec_and_mutex_lock(&user.refcnt, &event_mutex);
    } else {
    lockdep_assert_held(&event_mutex);
    delete = refcount_dec_and_test(&user.refcnt);
    }
    if (!delete) {
    return;
    }
//
// We now have the event_mutex in all cases, which ensures that
// no new references will be taken until event_mutex is released.
// New references come through find_user_event(), which requires
// the event_mutex to be held.
//
    if (user.reg_flags & USER_EVENT_REG_PERSIST) {
// We should not get here when persist flag is set
    pr_alert("BUG: Auto-delete engaged on persistent event\n");
// goto;
    }
//
// Unfortunately we have to attempt the actual destroy in a work
// queue. This is because not all cases handle a trace_event_call
// being removed within the class->reg() operation for unregister.
//
    INIT_WORK(&user.put_work, delayed_destroy_user_event);
//
// Since the event is still in the hashtable, we have to re-inc
// the ref count to 1. This count will be decremented and checked
// in the work queue to ensure it's still the last ref. This is
// needed because a user-process could register the same event in
// between the time of event_mutex release and the work queue
// running the delayed destroy. If we removed the item now from
// the hashtable, this would result in a timing window where a
// user process would fail a register because the trace_event_call
// register would fail in the tracing layers.
//
    refcount_set(&user.refcnt, 1);
    if (WARN_ON_ONCE!(!schedule_work(&user.put_work))) {
//
// If we fail we must wait for an admin to attempt delete or
// another register/close of the event, whichever is first.
//
    pr_warn!("user_events: Unable to queue delayed destroy\n");
    }
// label;
// Ensure if we didn't have event_mutex before we unlock it
    if (!locked) {
    mutex_unlock(&event_mutex);
    }
    }
#[no_mangle]
unsafe extern "C" fn user_event_group_destroy(group: *mut user_event_group) {
    kfree(group.system_name);
    kfree(group.system_multi_name);
    kfree(group);
    }
#[no_mangle]
pub unsafe extern "C" fn user_event_group_system_name() -> *mut c_void {
pub static mut system_name: *mut c_void = core::ptr::null_mut();
pub static mut len: c_int = 0;
    system_name = kmalloc(len, GFP_KERNEL);
    if (!system_name) {
    return core::ptr::null_mut();
    }
    snprintf(system_name, len, "%s", USER_EVENTS_SYSTEM);
    return system_name;
    }
#[no_mangle]
pub unsafe extern "C" fn user_event_group_system_multi_name() -> *mut c_void {
    return kstrdup(USER_EVENTS_MULTI_SYSTEM, GFP_KERNEL);
    }
#[no_mangle]
pub unsafe extern "C" fn current_user_event_group() -> *mut c_void {
    return init_group;
    }
#[no_mangle]
pub unsafe extern "C" fn user_event_group_create() -> *mut c_void {
pub static mut group: *mut c_void = core::ptr::null_mut();
    group = kzalloc_obj(*group);
    if (!group) {
    return core::ptr::null_mut();
    }
    group.system_name = user_event_group_system_name();
    if (!group.system_name) {
// goto;
    }
    group.system_multi_name = user_event_group_system_multi_name();
    if (!group.system_multi_name) {
// goto;
    }
    mutex_init(&group.reg_mutex);
    hash_init(group.register_table);
    return group;
// label;
    if (group) {
    user_event_group_destroy(group);
    }
    return core::ptr::null_mut();
    };
#[no_mangle]
unsafe extern "C" fn delayed_user_event_enabler_put(work: *mut work_struct) {
    let mut enabler = container_of!(to_rcu_work(work), user_event_enabler, put_rwork);
// No longer tracking the event via the enabler
    user_event_put(enabler.event, false);
// Run from queue_rcu_work(), the RCU grace period has elapsed
    kfree(enabler);
    }
#[no_mangle]
unsafe extern "C" fn user_event_enabler_destroy(enabler: *mut user_event_enabler) {
    list_del_rcu(&enabler.mm_enablers_link);
//
// The enabler is removed from an RCU-traversed list
// (user_event_mm_dup() walks mm->enablers under rcu_read_lock() only),
// and readers there dereference enabler->event and take a new ref on
// it. Both the put of that event reference and the free of the enabler
// therefore have to wait for a grace period so no reader can be looking
// at the enabler or racing the last put of its event.
//
// The put itself must not run in RCU context: when it drops the last
// reference user_event_put() takes event_mutex, which cannot be taken
// from a softirq/RCU callback. Defer both to a work item scheduled
// after a grace period via queue_rcu_work().
//
    INIT_RCU_WORK(&enabler.put_rwork, delayed_user_event_enabler_put);
    queue_rcu_work(system_percpu_wq, &enabler.put_rwork);
    }
#[no_mangle]
pub unsafe extern "C" fn user_event_mm_fault_in(mm: *mut user_event_mm, uaddr: c_ulong, attempt: c_int) -> c_int {
    let mut unlocked = 0;
    let mut ret = 0;
//
// Normally this is low, ensure that it cannot be taken advantage of by
// bad user processes to cause excessive looping.
//
    if (attempt > 10) {
    return -EFAULT;
    }
    mmap_read_lock(mm.mm);
// Ensure MM has tasks, cannot use after exit_mm()
    if (refcount_read(&mm.tasks) == 0) {
    ret = -ENOENT;
// goto;
    }
    ret = fixup_user_fault(mm.mm, uaddr, FAULT_FLAG_WRITE | FAULT_FLAG_REMOTE,
    &unlocked);
// label;
    mmap_read_unlock(mm.mm);
    return ret;
    }
// forward_decl: user_event_enabler_write;
#[no_mangle]
unsafe extern "C" fn user_event_enabler_fault_fixup(work: *mut work_struct) {
    let mut fault = container_of!(work, user_event_enabler_fault, work);
    let mut enabler = fault.enabler;
    let mut mm = fault.mm;
pub static mut uaddr: c_ulong = 0;
pub static mut attempt: c_int = 0;
    let mut ret = 0;
    ret = user_event_mm_fault_in(mm, uaddr, attempt);
    if (ret && ret != -ENOENT) {
    let mut user = enabler.event;
    pr_warn!("user_events: Fault for mm: 0x%p @ 0x%llx event: %s\n",
    mm.mm, (unsigned long long)uaddr, EVENT_NAME(user));
    }
// Prevent state changes from racing
    mutex_lock(&event_mutex);
// User asked for enabler to be removed during fault
    if (test_bit(ENABLE_VAL_FREEING_BIT, ENABLE_BITOPS(enabler))) {
    user_event_enabler_destroy(enabler);
// goto;
    }
//
// If we managed to get the page, re-issue the write. We do not
// want to get into a possible infinite loop, which is why we only
// attempt again directly if the page came in. If we couldn't get
// the page here, then we will try again the next time the event is
// enabled/disabled.
//
    clear_bit(ENABLE_VAL_FAULTING_BIT, ENABLE_BITOPS(enabler));
    if (!ret) {
    mmap_read_lock(mm.mm);
    user_event_enabler_write(mm, enabler, true, &attempt);
    mmap_read_unlock(mm.mm);
    }
// label;
    mutex_unlock(&event_mutex);
// In all cases we no longer need the mm or fault
    user_event_mm_put(mm);
    kmem_cache_free(fault_cache, fault);
    }
#[no_mangle]
pub unsafe extern "C" fn user_event_enabler_queue_fault(mm: *mut user_event_mm, enabler: *mut user_event_enabler, attempt: c_int) -> bool {
pub static mut fault: *mut c_void = core::ptr::null_mut();
    fault = kmem_cache_zalloc(fault_cache, GFP_NOWAIT);
    if (!fault) {
    return false;
    }
    INIT_WORK(&fault.work, user_event_enabler_fault_fixup);
    fault.mm = user_event_mm_get(mm);
    fault.enabler = enabler;
    fault.attempt = attempt;
// Don't try to queue in again while we have a pending fault
    set_bit(ENABLE_VAL_FAULTING_BIT, ENABLE_BITOPS(enabler));
    if (!schedule_work(&fault.work)) {
// Allow another attempt later
    clear_bit(ENABLE_VAL_FAULTING_BIT, ENABLE_BITOPS(enabler));
    user_event_mm_put(mm);
    kmem_cache_free(fault_cache, fault);
    return false;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn user_event_enabler_write(mm: *mut user_event_mm, enabler: *mut user_event_enabler, fixup_fault: bool, attempt: *mut c_int) -> c_int {
pub static mut uaddr: c_ulong = 0;
pub static mut ptr: *mut c_void = core::ptr::null_mut();
pub static mut page: *mut c_void = core::ptr::null_mut();
pub static mut kaddr: *mut c_void = core::ptr::null_mut();
pub static mut bit: c_int = 0;
    let mut ret = 0;
    lockdep_assert_held(&event_mutex);
    mmap_assert_locked(mm.mm);
// attempt += 1;
// Ensure MM has tasks, cannot use after exit_mm()
    if (refcount_read(&mm.tasks) == 0) {
    return -ENOENT;
    }
    if (unlikely(test_bit(ENABLE_VAL_FAULTING_BIT, ENABLE_BITOPS(enabler)) ||
    test_bit(ENABLE_VAL_FREEING_BIT, ENABLE_BITOPS(enabler)))) {
    return -EBUSY;
    }
    align_addr_bit(&uaddr, &bit, ENABLE_BITOPS(enabler));
    ret = pin_user_pages_remote(mm.mm, uaddr, 1, FOLL_WRITE | FOLL_NOFAULT,
    &page, core::ptr::null_mut());
    if (unlikely(ret <= 0)) {
    if (!fixup_fault) {
    return -EFAULT;
    }
    if (!user_event_enabler_queue_fault(mm, enabler, *attempt)) {
    pr_warn!("user_events: Unable to queue fault handler\n");
    }
    return -EFAULT;
    }
    kaddr = kmap_local_page(page);
    ptr = kaddr + (uaddr & ~PAGE_MASK);
// Update bit atomically, user tracers must be atomic as well
    if (enabler.event && enabler.event.status) {
    set_bit(bit, ptr);
    }
    else {
    clear_bit(bit, ptr);
    }
    kunmap_local(kaddr);
    unpin_user_pages_dirty_lock(&page, 1, true);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn user_event_enabler_exists(mm: *mut user_event_mm, uaddr: c_ulong, bit: c_uchar) -> bool {
pub static mut enabler: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(enabler, &mm.enablers, mm_enablers_link) {
    if (enabler.addr == uaddr && ENABLE_BIT(enabler) == bit) {
    return true;
    }
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn user_event_enabler_update(user: *mut user_event) {
pub static mut enabler: *mut c_void = core::ptr::null_mut();
pub static mut next: *mut c_void = core::ptr::null_mut();
pub static mut mm: *mut c_void = core::ptr::null_mut();
    let mut attempt = 0;
    lockdep_assert_held(&event_mutex);
//
// We need to build a one-shot list of all the mms that have an
// enabler for the user_event passed in. This list is only valid
// while holding the event_mutex. The only reason for this is due
// to the global mm list being RCU protected and we use methods
// which can wait (mmap_read_lock and pin_user_pages_remote).
//
// NOTE: user_event_mm_get_all() increments the ref count of each
// mm that is added to the list to prevent removal timing windows.
// We must always put each mm after they are used, which may wait.
//
    mm = user_event_mm_get_all(user);
    while (mm) {
    next = mm.next;
    mmap_read_lock(mm.mm);
    list_for_each_entry(enabler, &mm.enablers, mm_enablers_link) {
    if (enabler.event == user) {
    attempt = 0;
    user_event_enabler_write(mm, enabler, true, &attempt);
    }
    }
    mmap_read_unlock(mm.mm);
    user_event_mm_put(mm);
    mm = next;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn user_event_enabler_dup(orig: *mut user_event_enabler, mm: *mut user_event_mm) -> bool {
pub static mut enabler: *mut c_void = core::ptr::null_mut();
// Skip pending frees
    if (unlikely(test_bit(ENABLE_VAL_FREEING_BIT, ENABLE_BITOPS(orig)))) {
    return true;
    }
    enabler = kzalloc_obj(*enabler, GFP_NOWAIT | __GFP_ACCOUNT);
    if (!enabler) {
    return false;
    }
    enabler.event = user_event_get(orig.event);
    enabler.addr = orig.addr;
// Only dup part of value (ignore future flags, etc)
    enabler.values = orig.values & ENABLE_VAL_DUP_MASK;
// Enablers not exposed yet, RCU not required
    list_add(&enabler.mm_enablers_link, &mm.enablers);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn user_event_mm_get(mm: *mut user_event_mm) -> *mut c_void {
    refcount_inc(&mm.refcnt);
    return mm;
    }
#[no_mangle]
pub unsafe extern "C" fn user_event_mm_get_all(user: *mut user_event) -> *mut c_void {
    let mut found = core::ptr::null_mut();
pub static mut enabler: *mut c_void = core::ptr::null_mut();
pub static mut mm: *mut c_void = core::ptr::null_mut();
//
// We use the mm->next field to build a one-shot list from the global
// RCU protected list. To build this list the event_mutex must be held.
// This lets us build a list without requiring allocs that could fail
// when user based events are most wanted for diagnostics.
//
    lockdep_assert_held(&event_mutex);
//
// We do not want to block fork/exec while enablements are being
// updated, so we use RCU to walk the current tasks that have used
// user_events ABI for 1 or more events. Each enabler found in each
// task that matches the event being updated has a write to reflect
// the kernel state back into the process. Waits/faults must not occur
// during this. So we scan the list under RCU for all the mm that have
// the event within it. This is needed because mm_read_lock() can wait.
// Each user mm returned has a ref inc to handle remove RCU races.
//
    rcu_read_lock();
    list_for_each_entry_rcu(mm, &user_event_mms, mms_link) {
    list_for_each_entry_rcu(enabler, &mm.enablers, mm_enablers_link) {
    if (enabler.event == user) {
    mm.next = found;
    found = user_event_mm_get(mm);
    break;
    }
    }
    }
    rcu_read_unlock();
    return found;
    }
#[no_mangle]
pub unsafe extern "C" fn user_event_mm_alloc(t: *mut task_struct) -> *mut c_void {
pub static mut user_mm: *mut c_void = core::ptr::null_mut();
    user_mm = kzalloc_obj(*user_mm, GFP_KERNEL_ACCOUNT);
    if (!user_mm) {
    return core::ptr::null_mut();
    }
    user_mm.mm = t.mm;
    INIT_LIST_HEAD(&user_mm.enablers);
    refcount_set(&user_mm.refcnt, 1);
    refcount_set(&user_mm.tasks, 1);
//
// The lifetime of the memory descriptor can slightly outlast
// the task lifetime if a ref to the user_event_mm is taken
// between list_del_rcu() and call_rcu(). Therefore we need
// to take a reference to it to ensure it can live this long
// under this corner case. This can also occur in clones that
// outlast the parent.
//
    mmgrab(user_mm.mm);
    return user_mm;
    }
#[no_mangle]
unsafe extern "C" fn user_event_mm_attach(user_mm: *mut user_event_mm, t: *mut task_struct) {
    let mut flags = 0;
    spin_lock_irqsave(&user_event_mms_lock, flags);
    list_add_rcu(&user_mm.mms_link, &user_event_mms);
    spin_unlock_irqrestore(&user_event_mms_lock, flags);
    t.user_event_mm = user_mm;
    }
#[no_mangle]
pub unsafe extern "C" fn current_user_event_mm() -> *mut c_void {
    let mut user_mm = current.user_event_mm;
    if (user_mm) {
// goto;
    }
    user_mm = user_event_mm_alloc(current);
    if (!user_mm) {
// goto;
    }
    user_event_mm_attach(user_mm, current);
// label;
    refcount_inc(&user_mm.refcnt);
// label;
    return user_mm;
    }
#[no_mangle]
unsafe extern "C" fn user_event_mm_destroy(mm: *mut user_event_mm) {
    let mut enabler = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    list_for_each_entry_safe(enabler, next, &mm.enablers, mm_enablers_link) {
    user_event_enabler_destroy(enabler);
    }
    mmdrop(mm.mm);
    kfree(mm);
    }
#[no_mangle]
unsafe extern "C" fn user_event_mm_put(mm: *mut user_event_mm) {
    if (mm && refcount_dec_and_test(&mm.refcnt)) {
    user_event_mm_destroy(mm);
    }
    }
#[no_mangle]
unsafe extern "C" fn delayed_user_event_mm_put(work: *mut work_struct) {
pub static mut mm: *mut c_void = core::ptr::null_mut();
    mm = container_of!(to_rcu_work(work), user_event_mm, put_rwork);
    user_event_mm_put(mm);
    }
#[no_mangle]
pub unsafe extern "C" fn user_event_mm_remove(t: *mut task_struct) {
pub static mut mm: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    might_sleep();
    mm = t.user_event_mm;
    t.user_event_mm = core::ptr::null_mut();
// Clone will increment the tasks, only remove if last clone
    if (!refcount_dec_and_test(&mm.tasks)) {
    return;
    }
// Remove the mm from the list, so it can no longer be enabled
    spin_lock_irqsave(&user_event_mms_lock, flags);
    list_del_rcu(&mm.mms_link);
    spin_unlock_irqrestore(&user_event_mms_lock, flags);
//
// We need to wait for currently occurring writes to stop within
// the mm. This is required since exit_mm() snaps the current rss
// stats and clears them. On the final mmdrop(), check_mm() will
// report a bug if these increment.
//
// All writes/pins are done under mmap_read lock, take the write
// lock to ensure in-progress faults have completed. Faults that
// are pending but yet to run will check the task count and skip
// the fault since the mm is going away.
//
    mmap_write_lock(mm.mm);
    mmap_write_unlock(mm.mm);
//
// Put for mm must be done after RCU delay to handle new refs in
// between the list_del_rcu() and now. This ensures any get refs
// during rcu_read_lock() are accounted for during list removal.
//
// CPU A			|	CPU B
// ---------------------------------------------------------------
// user_event_mm_remove()	|	rcu_read_lock();
// list_del_rcu()		|	list_for_each_entry_rcu();
// call_rcu()			|	refcount_inc();
// .				|	rcu_read_unlock();
// schedule_work()		|	.
// user_event_mm_put()		|	.
//
// mmdrop() cannot be called in the softirq context of call_rcu()
// so we use a work queue after call_rcu() to run within.
//
    INIT_RCU_WORK(&mm.put_rwork, delayed_user_event_mm_put);
    queue_rcu_work(system_percpu_wq, &mm.put_rwork);
    }
#[no_mangle]
pub unsafe extern "C" fn user_event_mm_dup(t: *mut task_struct, old_mm: *mut user_event_mm) {
    let mut mm = user_event_mm_alloc(t);
pub static mut enabler: *mut c_void = core::ptr::null_mut();
// On failure, do not free parent's copy
    t.user_event_mm = core::ptr::null_mut();
    if (!mm) {
    return;
    }
    rcu_read_lock();
    list_for_each_entry_rcu(enabler, &old_mm.enablers, mm_enablers_link) {
    if (!user_event_enabler_dup(enabler, mm)) {
// goto;
    }
    }
    rcu_read_unlock();
    user_event_mm_attach(mm, t);
    return;
// label;
    rcu_read_unlock();
    user_event_mm_destroy(mm);
    }
#[no_mangle]
pub unsafe extern "C" fn current_user_event_enabler_exists(uaddr: c_ulong, bit: c_uchar) -> bool {
    let mut user_mm = current_user_event_mm();
    let mut exists = 0;
    if (!user_mm) {
    return false;
    }
    exists = user_event_enabler_exists(user_mm, uaddr, bit);
    user_event_mm_put(user_mm);
    return exists;
    }
    static struct user_event_enabler
// user_event_enabler_create(user_reg *reg, user_event *user,
    int *write_result)
    {
pub static mut enabler: *mut c_void = core::ptr::null_mut();
pub static mut user_mm: *mut c_void = core::ptr::null_mut();
pub static mut uaddr: c_ulong = 0;
pub static mut attempt: c_int = 0;
    user_mm = current_user_event_mm();
    if (!user_mm) {
    return core::ptr::null_mut();
    }
    enabler = kzalloc_obj(*enabler, GFP_KERNEL_ACCOUNT);
    if (!enabler) {
// goto;
    }
    enabler.event = user;
    enabler.addr = uaddr;
    enabler.values = reg.enable_bit;

    if (reg.enable_size == 4) {
    set_bit(ENABLE_VAL_32_ON_64_BIT, ENABLE_BITOPS(enabler));
    }

// label;
// Prevents state changes from racing with new enablers
    mutex_lock(&event_mutex);
// Attempt to reflect the current state within the process
    mmap_read_lock(user_mm.mm);
// write_result = user_event_enabler_write(user_mm, enabler, false,
    &attempt);
    mmap_read_unlock(user_mm.mm);
//
// If the write works, then we will track the enabler. A ref to the
// underlying user_event is held by the enabler to prevent it going
// away while the enabler is still in use by a process. The ref is
// removed when the enabler is destroyed. This means a event cannot
// be forcefully deleted from the system until all tasks using it
// exit or run exec(), which includes forks and clones.
//
    if (!*write_result) {
    user_event_get(user);
    list_add_rcu(&enabler.mm_enablers_link, &user_mm.enablers);
    }
    mutex_unlock(&event_mutex);
    if (*write_result) {
// Attempt to fault-in and retry if it worked
    if (!user_event_mm_fault_in(user_mm, uaddr, attempt)) {
// goto;
    }
    kfree(enabler);
    enabler = core::ptr::null_mut();
    }
// label;
    user_event_mm_put(user_mm);
    return enabler;
    }
    static __always_inline __must_check
#[no_mangle]
pub unsafe extern "C" fn user_event_last_ref(user: *mut user_event) -> bool {
pub static mut last: c_int = 0;
    if (user.reg_flags & USER_EVENT_REG_PERSIST) {
    last = 1;
    }
    return refcount_read(&user.refcnt) == last;
    }
    static __always_inline __must_check
#[no_mangle]
pub unsafe extern "C" fn copy_nofault(addr: *mut c_void, bytes: usize, i: *mut iov_iter) -> usize {
    let mut ret = 0;
    pagefault_disable();
    ret = copy_from_iter_nocache(addr, bytes, i);
    pagefault_enable();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn user_event_get_fields(call: *mut trace_event_call) -> *mut c_void {
    let mut user = call.data;
    return &user.fields;
    }
//
// Parses a register command for user_events
// Format: event_name[:FLAG1[,FLAG2...]] [field1[;field2...]]
//
// Example event named 'test' with a 20 char 'msg' field with an unsigned int
// 'id' field after:
// test char[20] msg;unsigned int id
//
// NOTE: Offsets are from the user data perspective, they are not from the
// trace_entry/buffer perspective. We automatically add the common properties
// sizes to the offset for the user.
//
// Upon success user_event has its ref count increased by 1.
//
#[no_mangle]
pub unsafe extern "C" fn user_event_parse_cmd(group: *mut user_event_group, raw_command: *mut c_char, newuser: *mut *mut user_event, reg_flags: c_int) -> c_int {
    let mut name = raw_command;
    let mut args = strpbrk(name, " ");
pub static mut flags: *mut c_void = core::ptr::null_mut();
    if (args) {
// args++ = '\0';
    }
    flags = strpbrk(name, ":");
    if (flags) {
// flags++ = '\0';
    }
    return user_event_parse(group, name, args, flags, newuser, reg_flags);
    }
#[no_mangle]
unsafe extern "C" fn user_field_array_size(type: *const c_char) -> c_int {
    let mut start = strchr(type, '[');
    char val[8];
pub static mut bracket: *mut c_void = core::ptr::null_mut();
pub static mut size: c_int = 0;
    if (start == core::ptr::null_mut()) {
    return -EINVAL;
    }
    if (strscpy(val, start + 1, sizeof!(val)) <= 0) {
    return -EINVAL;
    }
    bracket = strchr(val, ']');
    if (!bracket) {
    return -EINVAL;
    }
// bracket = '\0';
    if (kstrtouint(val, 0, &size)) {
    return -EINVAL;
    }
    if (size > MAX_FIELD_ARRAY_SIZE) {
    return -EINVAL;
    }
    return size;
    }
#[no_mangle]
unsafe extern "C" fn user_field_size(type: *const c_char) -> c_int {
// long is not allowed from a user, since it's ambiguous in size
    if (strcmp(type, "s64") == 0) {
    return sizeof!(s64);
    }
    if (strcmp(type, "u64") == 0) {
    return sizeof!(u64);
    }
    if (strcmp(type, "s32") == 0) {
    return sizeof!(s32);
    }
    if (strcmp(type, "u32") == 0) {
    return sizeof!(u32);
    }
    if (strcmp(type, "int") == 0) {
    return sizeof!(int);
    }
    if (strcmp(type, "unsigned int") == 0) {
    return sizeof!(unsigned int);
    }
    if (strcmp(type, "s16") == 0) {
    return sizeof!(s16);
    }
    if (strcmp(type, "u16") == 0) {
    return sizeof!(u16);
    }
    if (strcmp(type, "short") == 0) {
    return sizeof!(short);
    }
    if (strcmp(type, "unsigned short") == 0) {
    return sizeof!(unsigned short);
    }
    if (strcmp(type, "s8") == 0) {
    return sizeof!(s8);
    }
    if (strcmp(type, "u8") == 0) {
    return sizeof!(u8);
    }
    if (strcmp(type, "char") == 0) {
    return sizeof!(char);
    }
    if (strcmp(type, "unsigned char") == 0) {
    return sizeof!(unsigned char);
    }
    if (str_has_prefix(type, "char[")) {
    return user_field_array_size(type);
    }
    if (str_has_prefix(type, "unsigned char[")) {
    return user_field_array_size(type);
    }
    if (str_has_prefix(type, "__data_loc ")) {
    return sizeof!(u32);
    }
    if (str_has_prefix(type, "__rel_loc ")) {
    return sizeof!(u32);
    }
// Unknown basic type, error
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn user_event_destroy_validators(user: *mut user_event) {
    let mut validator = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    let mut head = &user.validators;
    list_for_each_entry_safe(validator, next, head, user_event_link) {
    list_del(&validator.user_event_link);
    kfree(validator);
    }
    }
#[no_mangle]
unsafe extern "C" fn user_event_destroy_fields(user: *mut user_event) {
    let mut field = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    let mut head = &user.fields;
    list_for_each_entry_safe(field, next, head, link) {
    list_del(&field.link);
    kfree(field);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn user_event_add_field(user: *mut user_event, type: *mut c_char, name: *mut c_char, offset: c_int, size: c_int, is_signed: c_int, filter_type: c_int) -> c_int {
pub static mut validator: *mut c_void = core::ptr::null_mut();
pub static mut field: *mut c_void = core::ptr::null_mut();
pub static mut validator_flags: c_int = 0;
    field = kmalloc_obj(*field, GFP_KERNEL_ACCOUNT);
    if (!field) {
    return -ENOMEM;
    }
    if (str_has_prefix(type, "__data_loc ")) {
// goto;
    }
    if (str_has_prefix(type, "__rel_loc ")) {
    validator_flags |= VALIDATOR_REL;
// goto;
    }
// goto;
// label;
    if (strstr(type, "char") != core::ptr::null_mut()) {
    validator_flags |= VALIDATOR_ENSURE_NULL;
    }
    validator = kmalloc_obj(*validator, GFP_KERNEL_ACCOUNT);
    if (!validator) {
    kfree(field);
    return -ENOMEM;
    }
    validator.flags = validator_flags;
    validator.offset = offset;
// Want sequential access when validating
    list_add_tail(&validator.user_event_link, &user.validators);
// label;
    field.type = type;
    field.name = name;
    field.offset = offset;
    field.size = size;
    field.is_signed = is_signed;
    field.filter_type = filter_type;
    if (filter_type == FILTER_OTHER) {
    field.filter_type = filter_assign_type(type);
    }
    list_add(&field.link, &user.fields);
//
// Min size from user writes that are required, this does not include
// the size of trace_entry (common fields).
//
    user.min_size = (offset + size) - sizeof!(trace_entry);
    return 0;
    }
//
// Parses the values of a field within the description
// Format: type name [size]
//
#[no_mangle]
pub unsafe extern "C" fn user_event_parse_field(field: *mut c_char, user: *mut user_event, offset: *mut u32) -> c_int {
    let mut part = core::ptr::null_mut();
    let mut type = core::ptr::null_mut();
    let mut name = core::ptr::null_mut();
pub static mut depth: u32 = 0;
    int len, size = -EINVAL;
pub static mut is_struct: bool = false;
    field = skip_spaces(field);
    if (*field == '\0') {
    return 0;
    }
// Handle types that have a space within
    len = str_has_prefix(field, "unsigned ");
    if (len) {
// goto;
    }
    len = str_has_prefix(field, "struct ");
    if (len) {
    is_struct = true;
// goto;
    }
    len = str_has_prefix(field, "__data_loc unsigned ");
    if (len) {
// goto;
    }
    len = str_has_prefix(field, "__data_loc ");
    if (len) {
// goto;
    }
    len = str_has_prefix(field, "__rel_loc unsigned ");
    if (len) {
// goto;
    }
    len = str_has_prefix(field, "__rel_loc ");
    if (len) {
// goto;
    }
// goto;
// label;
    type = field;
    field = strpbrk(field + len, " ");
    if (field == core::ptr::null_mut()) {
    return -EINVAL;
    }
// field++ = '\0';
    depth += 1;
// label;
    name = core::ptr::null_mut();
    while ((part = strsep(&field, " ")) != core::ptr::null_mut()) {
    match (depth++) {
    FIELD_DEPTH_TYPE => {
    type = part;
    // break;
    }
    FIELD_DEPTH_NAME => {
    name = part;
    // break;
    }
    FIELD_DEPTH_SIZE => {
    if (!is_struct) {
    return -EINVAL;
    }
    if (kstrtou32(part, 10, &size)) {
    return -EINVAL;
    }
    // break;
    }
    _ => {
    return -EINVAL;
    }
    }
    }
    if (depth < FIELD_DEPTH_SIZE || !name) {
    return -EINVAL;
    }
    if (depth == FIELD_DEPTH_SIZE) {
    size = user_field_size(type);
    }
    if (size == 0) {
    return -EINVAL;
    }
    if (size < 0) {
    return size;
    }
// offset = saved_offset + size;
    return user_event_add_field(user, type, name, saved_offset, size,
    type[0] != 'u', FILTER_OTHER);
    }
#[no_mangle]
unsafe extern "C" fn user_event_parse_fields(user: *mut user_event, args: *mut c_char) -> c_int {
pub static mut field: *mut c_void = core::ptr::null_mut();
pub static mut offset: u32 = 0;
pub static mut ret: c_int = 0;
    if (args == core::ptr::null_mut()) {
    return 0;
    }
    while ((field = strsep(&args, ";")) != core::ptr::null_mut()) {
    ret = user_event_parse_field(field, user, &offset);
    if (ret) {
    break;
    }
    }
    return ret;
    }
    static struct trace_event_fields user_event_fields_array[1];
    static const char *user_field_format(const char *type)
    {
    if (strcmp(type, "s64") == 0) {
    return "%lld";
    }
    if (strcmp(type, "u64") == 0) {
    return "%llu";
    }
    if (strcmp(type, "s32") == 0) {
    return "%d";
    }
    if (strcmp(type, "u32") == 0) {
    return "%u";
    }
    if (strcmp(type, "int") == 0) {
    return "%d";
    }
    if (strcmp(type, "unsigned int") == 0) {
    return "%u";
    }
    if (strcmp(type, "s16") == 0) {
    return "%d";
    }
    if (strcmp(type, "u16") == 0) {
    return "%u";
    }
    if (strcmp(type, "short") == 0) {
    return "%d";
    }
    if (strcmp(type, "unsigned short") == 0) {
    return "%u";
    }
    if (strcmp(type, "s8") == 0) {
    return "%d";
    }
    if (strcmp(type, "u8") == 0) {
    return "%u";
    }
    if (strcmp(type, "char") == 0) {
    return "%d";
    }
    if (strcmp(type, "unsigned char") == 0) {
    return "%u";
    }
    if (strstr(type, "char[") != core::ptr::null_mut()) {
    return "%s";
    }
// Unknown, likely struct, allowed treat as 64-bit
    return "%llu";
    }
#[no_mangle]
unsafe extern "C" fn user_field_is_dyn_string(type: *const c_char, str_func: *const c_char) -> bool {
    if (str_has_prefix(type, "__data_loc ")) {
// str_func = "__get_str";
// goto;
    }
    if (str_has_prefix(type, "__rel_loc ")) {
// str_func = "__get_rel_str";
// goto;
    }
    return false;
// label;
    return strstr(type, "char") != core::ptr::null_mut();
    }

#[no_mangle]
pub unsafe extern "C" fn user_dyn_field_set_string(argc: c_int, argv: *mut *mut c_char, iout: *mut c_int, buf: *mut c_char, len: c_int, colon: *mut bool) -> c_int {
pub static mut pos: c_int = 0;
// colon = false;
    while (i < argc) {
    if (i != *iout) {
    pos += snprintf(buf + pos, LEN_OR_ZERO, " ");
    }
    pos += snprintf(buf + pos, LEN_OR_ZERO, "%s", argv[i]);
    if (strchr(argv[i], ';')) {
    i += 1;
// colon = true;
    break;
    }
    }
// Actual set, advance i
    if (len != 0) {
// iout = i;
    }
    return pos + 1;
    }
#[no_mangle]
pub unsafe extern "C" fn user_field_set_string(field: *mut ftrace_event_field, buf: *mut c_char, len: c_int, colon: bool) -> c_int {
pub static mut pos: c_int = 0;
    pos += snprintf(buf + pos, LEN_OR_ZERO, "%s", field.type);
    pos += snprintf(buf + pos, LEN_OR_ZERO, " ");
    pos += snprintf(buf + pos, LEN_OR_ZERO, "%s", field.name);
    if (str_has_prefix(field.type, "struct ")) {
    pos += snprintf(buf + pos, LEN_OR_ZERO, " %d", field.size);
    }
    if (colon) {
    pos += snprintf(buf + pos, LEN_OR_ZERO, ";");
    }
    return pos + 1;
    }
#[no_mangle]
unsafe extern "C" fn user_event_set_print_fmt(user: *mut user_event, buf: *mut c_char, len: c_int) -> c_int {
pub static mut field: *mut c_void = core::ptr::null_mut();
    let mut head = &user.fields;
pub static mut pos: c_int = 0;
pub static mut str_func: *mut c_void = core::ptr::null_mut();
    pos += snprintf(buf + pos, LEN_OR_ZERO, "\"");
    list_for_each_entry_reverse(field, head, link) {
    if (depth != 0) {
    pos += snprintf(buf + pos, LEN_OR_ZERO, " ");
    }
    pos += snprintf(buf + pos, LEN_OR_ZERO, "%s=%s",
    field.name, user_field_format(field.type));
    depth += 1;
    }
    pos += snprintf(buf + pos, LEN_OR_ZERO, "\"");
    list_for_each_entry_reverse(field, head, link) {
    if (user_field_is_dyn_string(field.type, &str_func)) {
    pos += snprintf(buf + pos, LEN_OR_ZERO,
    ", %s(%s)", str_func, field.name);
    }
    else {
    pos += snprintf(buf + pos, LEN_OR_ZERO,
    ", REC.%s", field.name);
    }
    }
    return pos + 1;
    }

#[no_mangle]
unsafe extern "C" fn user_event_create_print_fmt(user: *mut user_event) -> c_int {
pub static mut print_fmt: *mut c_void = core::ptr::null_mut();
    let mut len = 0;
    len = user_event_set_print_fmt(user, core::ptr::null_mut(), 0);
    print_fmt = kmalloc(len, GFP_KERNEL_ACCOUNT);
    if (!print_fmt) {
    return -ENOMEM;
    }
    user_event_set_print_fmt(user, print_fmt, len);
    user.call.print_fmt = print_fmt;
    return 0;
    }
    static enum print_line_t user_event_print_trace(trace_iterator *iter,
    int flags, trace_event *event)
    {
    return print_event_fields(iter, event);
    }
pub static mut trace_event_functions: usize = 0;
#[no_mangle]
unsafe extern "C" fn user_event_set_call_visible(user: *mut user_event, visible: bool) -> c_int {
    CLASS(prepare_creds, cred)();
    if (!cred) {
    return -ENOMEM;
    }
//
// While by default tracefs is locked down, systems can be configured
// to allow user_event files to be less locked down. The extreme case
// being "other" has read/write access to user_events_data/status.
//
// When not locked down, processes may not have permissions to
// add/remove calls themselves to tracefs. We need to temporarily
// switch to root file permission to allow for this scenario.
//
    cred.fsuid = GLOBAL_ROOT_UID;
    scoped_with_creds(cred) {
    if (visible) {
    return trace_add_event_call(&user.call);
    }
    return trace_remove_event_call(&user.call);
    }
    }
#[no_mangle]
unsafe extern "C" fn destroy_user_event(user: *mut user_event) -> c_int {
pub static mut ret: c_int = 0;
    lockdep_assert_held(&event_mutex);
// Must destroy fields before call removal
    user_event_destroy_fields(user);
    ret = user_event_set_call_visible(user, false);
    if (ret) {
    return ret;
    }
    dyn_event_remove(&user.devent);
    hash_del(&user.node);
    user_event_destroy_validators(user);
// If we have different names, both must be freed
    if (EVENT_NAME(user) != EVENT_TP_NAME(user)) {
    kfree(EVENT_TP_NAME(user));
    }
    kfree(user.call.print_fmt);
    kfree(EVENT_NAME(user));
    kfree(user);
    if (current_user_events > 0) {
    current_user_events -= 1;
    }
    else {
    pr_alert("BUG: Bad current_user_events\n");
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn find_user_event(group: *mut user_event_group, name: *mut c_char, argc: c_int, argv: *mut *mut c_char, flags: u32, outkey: *mut u32) -> *mut c_void {
pub static mut user: *mut c_void = core::ptr::null_mut();
pub static mut key: u32 = 0;
// outkey = key;
    hash_for_each_possible(group.register_table, user, node, key) {
//
// Single-format events shouldn't return multi-format
// events. Callers expect the underlying tracepoint to match
// the name exactly in these cases. Only check like-formats.
//
    if (EVENT_MULTI_FORMAT(flags) != EVENT_MULTI_FORMAT(user.reg_flags)) {
    continue;
    }
    if (strcmp(EVENT_NAME(user), name)) {
    continue;
    }
    if (user_fields_match(user, argc, argv)) {
    return user_event_get(user);
    }
// Scan others if this is a multi-format event
    if (EVENT_MULTI_FORMAT(flags)) {
    continue;
    }
    return ERR_PTR(-EADDRINUSE);
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn user_event_validate(user: *mut user_event, data: *mut c_void, len: c_int) -> c_int {
    let mut head = &user.validators;
pub static mut validator: *mut c_void = core::ptr::null_mut();
    void *pos, *end = data + len;
    u32 loc, offset, size;
    list_for_each_entry(validator, head, user_event_link) {
    pos = data + validator.offset;
// Already done min_size check, no bounds check here
    loc = *pos;
    offset = loc & 0xffff;
    size = loc >> 16;
    if (likely(validator.flags & VALIDATOR_REL)) {
    pos += offset + sizeof!(loc);
    }
    else {
    pos = data + offset;
    }
    pos += size;
    if (unlikely(pos > end)) {
    return -EFAULT;
    }
    if (likely(validator.flags & VALIDATOR_ENSURE_NULL)) {
    if (unlikely(*(pos - 1) != '\0'))
    return -EFAULT;
    }
    }
    return 0;
    }
//
// Writes the user supplied payload out to a trace file.
//
#[no_mangle]
pub unsafe extern "C" fn user_event_ftrace(user: *mut user_event, i: *mut iov_iter, tpdata: *mut c_void, faulted: *mut bool) {
pub static mut file: *mut c_void = core::ptr::null_mut();
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut event_buffer: usize = 0;
pub static mut size: usize = 0;
    file = tpdata;
    if (!file ||
    !(file.flags & EVENT_FILE_FL_ENABLED) ||
    trace_trigger_soft_disabled(file)) {
    return;
    }
// Allocates and fills trace_entry, + 1 of this is data payload
    entry = trace_event_buffer_reserve(&event_buffer, file, size);
    if (unlikely(!entry)) {
    return;
    }
    if (unlikely(i.count != 0 && !copy_nofault(entry + 1, i.count, i))) {
// goto;
    }
    if (!list_empty(&user.validators) &&
    unlikely(user_event_validate(user, entry, size))) {
// goto;
    }
    trace_event_buffer_commit(&event_buffer);
    return;
// label;
// faulted = true;
    __trace_event_discard_commit(event_buffer.buffer,
    event_buffer.event);
    }

//
// Writes the user supplied payload out to perf ring buffer.
//
#[no_mangle]
pub unsafe extern "C" fn user_event_perf(user: *mut user_event, i: *mut iov_iter, tpdata: *mut c_void, faulted: *mut bool) {
pub static mut perf_head: *mut c_void = core::ptr::null_mut();
    perf_head = this_cpu_ptr(user.call.perf_events);
    if (perf_head && !hlist_empty(perf_head)) {
pub static mut perf_entry: *mut c_void = core::ptr::null_mut();
pub static mut regs: *mut c_void = core::ptr::null_mut();
pub static mut size: usize = 0;
    let mut context = 0;
    perf_entry = perf_trace_buf_alloc(ALIGN(size, 8),
    &regs, &context);
    if (unlikely(!perf_entry)) {
    return;
    }
    perf_fetch_caller_regs(regs);
    if (unlikely(i.count != 0 && !copy_nofault(perf_entry + 1, i.count, i))) {
// goto;
    }
    if (!list_empty(&user.validators) &&
    unlikely(user_event_validate(user, perf_entry, size))) {
// goto;
    }
    perf_trace_buf_submit(perf_entry, size, context,
    user.call.event.type, 1, regs,
    perf_head, core::ptr::null_mut());
    return;
// label;
// faulted = true;
    perf_swevent_put_recursion_context(context);
    }
    }

//
// Update the enabled bit among all user processes.
//
#[no_mangle]
unsafe extern "C" fn update_enable_bit_for(user: *mut user_event) {
    let mut tp = &user.tracepoint;
pub static mut status: c_char = 0;
    if (static_key_enabled(&tp.key)) {
pub static mut probe_func_ptr: *mut c_void = core::ptr::null_mut();
    let mut probe_func;
    rcu_read_lock_sched();
    probe_func_ptr = rcu_dereference_sched(tp.funcs);
    if (probe_func_ptr) {
    do {
    probe_func = probe_func_ptr.func;
    if (probe_func == user_event_ftrace) {
    status |= EVENT_STATUS_FTRACE;
    }


    else if (probe_func == user_event_perf) {
    status |= EVENT_STATUS_PERF;
    }

    else {
    status |= EVENT_STATUS_OTHER;
    }
    } while ((++probe_func_ptr).func);
    }
    rcu_read_unlock_sched();
    }
    user.status = status;
    user_event_enabler_update(user);
    }
//
// Register callback for our events from tracing sub-systems.
//
#[no_mangle]
pub unsafe extern "C" fn user_event_reg(call: *mut trace_event_call, type: trace_reg, data: *mut c_void) -> c_int {
    let mut user = call.data;
pub static mut ret: c_int = 0;
    if (!user) {
    return -ENOENT;
    }
    match (type) {
    TRACE_REG_REGISTER => {
    ret = tracepoint_probe_register(call.tp,
    call.class.probe,
    data);
    if (!ret) {
// goto;
    }
    // break;
    }
    TRACE_REG_UNREGISTER => {
    tracepoint_probe_unregister(call.tp,
    call.class.probe,
    data);
// goto;

    }
    TRACE_REG_PERF_REGISTER => {
    ret = tracepoint_probe_register(call.tp,
    call.class.perf_probe,
    data);
    if (!ret) {
// goto;
    }
    // break;
    }
    TRACE_REG_PERF_UNREGISTER => {
    tracepoint_probe_unregister(call.tp,
    call.class.perf_probe,
    data);
// goto;
    }
    TRACE_REG_PERF_OPEN => {
    }
    TRACE_REG_PERF_CLOSE => {
    }
    TRACE_REG_PERF_ADD => {
    }
    TRACE_REG_PERF_DEL => {
    // break;

    }
    }
    return ret;
// label;
    user_event_get(user);
    update_enable_bit_for(user);
    return 0;
// label;
    update_enable_bit_for(user);
    user_event_put(user, true);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn user_event_create(raw_command: *const c_char) -> c_int {
pub static mut group: *mut c_void = core::ptr::null_mut();
pub static mut user: *mut c_void = core::ptr::null_mut();
pub static mut name: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (!str_has_prefix(raw_command, USER_EVENTS_PREFIX)) {
    return -ECANCELED;
    }
    raw_command += USER_EVENTS_PREFIX_LEN;
    raw_command = skip_spaces(raw_command);
    name = kstrdup(raw_command, GFP_KERNEL_ACCOUNT);
    if (!name) {
    return -ENOMEM;
    }
    group = current_user_event_group();
    if (!group) {
    kfree(name);
    return -ENOENT;
    }
    mutex_lock(&group.reg_mutex);
// Dyn events persist, otherwise they would cleanup immediately
    ret = user_event_parse_cmd(group, name, &user, USER_EVENT_REG_PERSIST);
    if (!ret) {
    user_event_put(user, false);
    }
    mutex_unlock(&group.reg_mutex);
    if (ret) {
    kfree(name);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn user_event_show(m: *mut seq_file, ev: *mut dyn_event) -> c_int {
    let mut user = container_of!(ev, user_event, devent);
pub static mut field: *mut c_void = core::ptr::null_mut();
pub static mut head: *mut c_void = core::ptr::null_mut();
pub static mut depth: c_int = 0;
    seq_printf(m, "%s%s", USER_EVENTS_PREFIX, EVENT_NAME(user));
    head = trace_get_fields(&user.call);
    list_for_each_entry_reverse(field, head, link) {
    if (depth == 0) {
    seq_putc(m, ' ');
    }
    else {
    seq_puts(m, "; ");
    }
    seq_printf(m, "%s %s", field.type, field.name);
    if (str_has_prefix(field.type, "struct ")) {
    seq_printf(m, " %d", field.size);
    }
    depth += 1;
    }
    seq_putc(m, '\n');
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn user_event_is_busy(ev: *mut dyn_event) -> bool {
    let mut user = container_of!(ev, user_event, devent);
    return !user_event_last_ref(user);
    }
#[no_mangle]
unsafe extern "C" fn user_event_free(ev: *mut dyn_event) -> c_int {
    let mut user = container_of!(ev, user_event, devent);
    if (!user_event_last_ref(user)) {
    return -EBUSY;
    }
    if (!user_event_capable(user.reg_flags)) {
    return -EPERM;
    }
    return destroy_user_event(user);
    }
#[no_mangle]
pub unsafe extern "C" fn user_field_match(field: *mut ftrace_event_field, argc: c_int, argv: *mut *mut c_char, iout: *mut c_int) -> bool {
    let mut field_name = core::ptr::null_mut(), *dyn_field_name = core::ptr::null_mut();
pub static mut colon: bool = false;
    let mut dyn_len = 0;
    let mut len = 0;
    if (*iout >= argc) {
    return false;
    }
    dyn_len = user_dyn_field_set_string(argc, argv, iout, dyn_field_name,
    0, &colon);
    len = user_field_set_string(field, field_name, 0, colon);
    if (dyn_len != len) {
    return false;
    }
    dyn_field_name = kmalloc(dyn_len, GFP_KERNEL);
    field_name = kmalloc(len, GFP_KERNEL);
    if (!dyn_field_name || !field_name) {
// goto;
    }
    user_dyn_field_set_string(argc, argv, iout, dyn_field_name,
    dyn_len, &colon);
    user_field_set_string(field, field_name, len, colon);
    match = strcmp(dyn_field_name, field_name) == 0;
// label;
    kfree(dyn_field_name);
    kfree(field_name);
    return match;
    }
#[no_mangle]
pub unsafe extern "C" fn user_fields_match(user: *mut user_event, argc: c_int, argv: *mut *mut c_char) -> bool {
pub static mut field: *mut c_void = core::ptr::null_mut();
    let mut head = &user.fields;
pub static mut i: c_int = 0;
    if (argc == 0) {
    return list_empty(head);
    }
    list_for_each_entry_reverse(field, head, link) {
    if (!user_field_match(field, argc, argv, &i)) {
    return false;
    }
    }
    if (i != argc) {
    return false;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn user_event_match(system: *mut c_char, event: *mut c_char, argc: c_int, argv: *mut *mut c_char, ev: *mut dyn_event) -> bool {
    let mut user = container_of!(ev, user_event, devent);
    let mut match = 0;
    match = strcmp(EVENT_NAME(user), event) == 0;
    if (match && system) {
    match = strcmp(system, user.group.system_name) == 0 ||
    strcmp(system, user.group.system_multi_name) == 0;
    }
    if (match) {
    match = user_fields_match(user, argc, argv);
    }
    return match;
    }
pub static mut dyn_event_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn user_event_trace_register(user: *mut user_event) -> c_int {
    let mut ret = 0;
    ret = register_trace_event(&user.call.event);
    if (!ret) {
    return -ENODEV;
    }
    ret = user_event_set_call_visible(user, true);
    if (ret) {
    unregister_trace_event(&user.call.event);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn user_event_set_tp_name(user: *mut user_event) -> c_int {
    lockdep_assert_held(&user.group.reg_mutex);
    if (EVENT_MULTI_FORMAT(user.reg_flags)) {
pub static mut multi_name: *mut c_void = core::ptr::null_mut();
    multi_name = kasprintf(GFP_KERNEL_ACCOUNT, "%s.%llx",
    user.reg_name, user.group.multi_id);
    if (!multi_name) {
    return -ENOMEM;
    }
    user.call.name = multi_name;
    user.tracepoint.name = multi_name;
// Inc to ensure unique multi-event name next time
    user.group.multi_id += 1;
    } else {
// Non Multi-format uses register name
    user.call.name = user.reg_name;
    user.tracepoint.name = user.reg_name;
    }
    return 0;
    }
//
// Counts how many ';' without a trailing space are in the args.
//
#[no_mangle]
unsafe extern "C" fn count_semis_no_space(args: *mut c_char) -> c_int {
pub static mut count: c_int = 0;
    while ((args = strchr(args, ';'))) {
    args += 1;
    if (!isspace(*args)) {
    count += 1;
    }
    }
    return count;
    }
//
// Copies the arguments while ensuring all ';' have a trailing space.
//
#[no_mangle]
pub unsafe extern "C" fn insert_space_after_semis(args: *mut c_char, count: c_int) -> *mut c_void {
    let mut fixed = core::ptr::null_mut();
    let mut pos = core::ptr::null_mut();
    let mut len = 0;
    len = strlen(args) + count;
    fixed = kmalloc(len + 1, GFP_KERNEL);
    if (!fixed) {
    return core::ptr::null_mut();
    }
    pos = fixed;
// Insert a space after ';' if there is no trailing space.
    while (*args) {
// pos = *args += 1;
    if (*pos++ == ';' && !isspace(*args)) {
// pos++ = ' ';
    }
    }
// pos = '\0';
    return fixed;
    }
    static char **user_event_argv_split(char *args, int *argc)
    {
pub static mut split: *mut c_void = core::ptr::null_mut();
pub static mut fixed: *mut c_void = core::ptr::null_mut();
    let mut count = 0;
// Count how many ';' without a trailing space
    count = count_semis_no_space(args);
// No fixup is required
    if (!count) {
    return argv_split(GFP_KERNEL, args, argc);
    }
// We must fixup 'field;field' to 'field; field'
    fixed = insert_space_after_semis(args, count);
    if (!fixed) {
    return core::ptr::null_mut();
    }
// We do a normal split afterwards
    split = argv_split(GFP_KERNEL, fixed, argc);
// We can free since argv_split makes a copy
    kfree(fixed);
    return split;
    }
//
// Parses the event name, arguments and flags then registers if successful.
// The name buffer lifetime is owned by this method for success cases only.
// Upon success the returned user_event has its ref count increased by 1.
//
#[no_mangle]
pub unsafe extern "C" fn user_event_parse(group: *mut user_event_group, name: *mut c_char, args: *mut c_char, flags: *mut c_char, newuser: *mut *mut user_event, reg_flags: c_int) -> c_int {
pub static mut user: *mut c_void = core::ptr::null_mut();
    let mut argv = core::ptr::null_mut();
pub static mut argc: c_int = 0;
    let mut ret = 0;
    let mut key = 0;
// Currently don't support any text based flags
    if (flags != core::ptr::null_mut()) {
    return -EINVAL;
    }
    if (!user_event_capable(reg_flags)) {
    return -EPERM;
    }
    if (args) {
    argv = user_event_argv_split(args, &argc);
    if (!argv) {
    return -ENOMEM;
    }
    }
// Prevent dyn_event from racing
    mutex_lock(&event_mutex);
    user = find_user_event(group, name, argc, argv,
    reg_flags, &key);
    mutex_unlock(&event_mutex);
    if (argv) {
    argv_free(argv);
    }
    if (IS_ERR(user)) {
    return PTR_ERR(user);
    }
    if (user) {
// newuser = user;
//
// Name is allocated by caller, free it since it already exists.
// Caller only worries about failure cases for freeing.
//
    kfree(name);
    return 0;
    }
    user = kzalloc_obj(*user, GFP_KERNEL_ACCOUNT);
    if (!user) {
    return -ENOMEM;
    }
    INIT_LIST_HEAD(&user.class.fields);
    INIT_LIST_HEAD(&user.fields);
    INIT_LIST_HEAD(&user.validators);
    user.group = group;
    user.reg_name = name;
    user.reg_flags = reg_flags;
    ret = user_event_set_tp_name(user);
    if (ret) {
// goto;
    }
    ret = user_event_parse_fields(user, args);
    if (ret) {
// goto;
    }
    ret = user_event_create_print_fmt(user);
    if (ret) {
// goto;
    }
    user.call.data = user;
    user.call.class = &user.class;
    user.call.flags = TRACE_EVENT_FL_TRACEPOINT;
    user.call.tp = &user.tracepoint;
    user.call.event.funcs = &user_event_funcs;
    if (EVENT_MULTI_FORMAT(user.reg_flags)) {
    user.class.system = group.system_multi_name;
    }
    else {
    user.class.system = group.system_name;
    }
    user.class.fields_array = user_event_fields_array;
    user.class.get_fields = user_event_get_fields;
    user.class.reg = user_event_reg;
    user.class.probe = user_event_ftrace;

    user.class.perf_probe = user_event_perf;

    mutex_lock(&event_mutex);
    if (current_user_events >= max_user_events) {
    ret = -EMFILE;
// goto;
    }
    ret = user_event_trace_register(user);
    if (ret) {
// goto;
    }
    if (user.reg_flags & USER_EVENT_REG_PERSIST) {
// Ensure we track self ref and caller ref (2)
    refcount_set(&user.refcnt, 2);
    } else {
// Ensure we track only caller ref (1)
    refcount_set(&user.refcnt, 1);
    }
    dyn_event_init(&user.devent, &user_event_dops);
    dyn_event_add(&user.devent, &user.call);
    hash_add(group.register_table, &user.node, key);
    current_user_events += 1;
    mutex_unlock(&event_mutex);
// newuser = user;
    return 0;
// label;
    mutex_unlock(&event_mutex);
// label;
    user_event_destroy_fields(user);
    user_event_destroy_validators(user);
    kfree(user.call.print_fmt);
// Caller frees reg_name on error, but not multi-name
    if (EVENT_NAME(user) != EVENT_TP_NAME(user)) {
    kfree(EVENT_TP_NAME(user));
    }
    kfree(user);
    return ret;
    }
//
// Deletes previously created events if they are no longer being used.
//
#[no_mangle]
unsafe extern "C" fn delete_user_event(group: *mut user_event_group, name: *mut c_char) -> c_int {
pub static mut user: *mut c_void = core::ptr::null_mut();
pub static mut tmp: *mut c_void = core::ptr::null_mut();
pub static mut key: u32 = 0;
pub static mut ret: c_int = 0;
// Attempt to delete all event(s) with the name passed in
    hash_for_each_possible_safe(group.register_table, user, tmp, node, key) {
    if (strcmp(EVENT_NAME(user), name)) {
    continue;
    }
    if (!user_event_last_ref(user)) {
    return -EBUSY;
    }
    if (!user_event_capable(user.reg_flags)) {
    return -EPERM;
    }
    ret = destroy_user_event(user);
    if (ret) {
// goto;
    }
    }
// label;
    return ret;
    }
//
// Validates the user payload and writes via iterator.
//
#[no_mangle]
unsafe extern "C" fn user_events_write_core(file: *mut file, i: *mut iov_iter) -> isize {
    let mut info = file.private_data;
pub static mut refs: *mut c_void = core::ptr::null_mut();
    let mut user = core::ptr::null_mut();
pub static mut tp: *mut c_void = core::ptr::null_mut();
pub static mut ret: isize = 0;
    let mut idx = 0;
    if (unlikely(copy_from_iter(&idx, sizeof!(idx), i) != sizeof!(idx))) {
    return -EFAULT;
    }
    if (idx < 0) {
    return -EINVAL;
    }
    rcu_read_lock_sched();
    refs = rcu_dereference_sched(info.refs);
//
// The refs->events array is protected by RCU, and new items may be
// added. But the user retrieved from indexing into the events array
// shall be immutable while the file is opened.
//
    if (likely(refs && idx < refs.count)) {
    user = refs.events[idx];
    }
    rcu_read_unlock_sched();
    if (unlikely(user == core::ptr::null_mut())) {
    return -ENOENT;
    }
    if (unlikely(i.count < user.min_size)) {
    return -EINVAL;
    }
    tp = &user.tracepoint;
//
// It's possible key.enabled disables after this check, however
// we don't mind if a few events are included in this condition.
//
    if (likely(static_key_enabled(&tp.key))) {
pub static mut probe_func_ptr: *mut c_void = core::ptr::null_mut();
    let mut probe_func;
pub static mut copy: usize = 0;
pub static mut tpdata: *mut c_void = core::ptr::null_mut();
    let mut faulted = 0;
    if (unlikely(fault_in_iov_iter_readable(i, i.count))) {
    return -EFAULT;
    }
    faulted = false;
    rcu_read_lock_sched();
    probe_func_ptr = rcu_dereference_sched(tp.funcs);
    if (probe_func_ptr) {
    do {
    copy = *i;
    probe_func = probe_func_ptr.func;
    tpdata = probe_func_ptr.data;
    probe_func(user, &copy, tpdata, &faulted);
    } while ((++probe_func_ptr).func);
    }
    rcu_read_unlock_sched();
    if (unlikely(faulted)) {
    return -EFAULT;
    }
    } else {
    return -EBADF;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn user_events_open(node: *mut inode, file: *mut file) -> c_int {
pub static mut group: *mut c_void = core::ptr::null_mut();
pub static mut info: *mut c_void = core::ptr::null_mut();
    group = current_user_event_group();
    if (!group) {
    return -ENOENT;
    }
    info = kzalloc_obj(*info, GFP_KERNEL_ACCOUNT);
    if (!info) {
    return -ENOMEM;
    }
    info.group = group;
    file.private_data = info;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn user_events_write(file: *mut file, ubuf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
pub static mut i: usize = 0;
    if (unlikely(*ppos != 0)) {
    return -EFAULT;
    }
    if (unlikely(import_ubuf(ITER_SOURCE, ubuf, count, &i))) {
    return -EFAULT;
    }
    return user_events_write_core(file, &i);
    }
#[no_mangle]
unsafe extern "C" fn user_events_write_iter(kp: *mut kiocb, i: *mut iov_iter) -> isize {
    return user_events_write_core(kp.ki_filp, i);
    }
#[no_mangle]
pub unsafe extern "C" fn user_events_ref_add(info: *mut user_event_file_info, user: *mut user_event) -> c_int {
    let mut group = info.group;
    let mut refs = core::ptr::null_mut();
    let mut new_refs = core::ptr::null_mut();
    int i, size, count = 0;
    refs = rcu_dereference_protected(info.refs,
    lockdep_is_held(&group.reg_mutex));
    if (refs) {
    count = refs.count;
    for (i = 0; i < count; ++i) {
    if (refs.events[i] == user)
    return i;
    }
    }
    size = struct_size(refs, events, count + 1);
    new_refs = kzalloc(size, GFP_KERNEL_ACCOUNT);
    if (!new_refs) {
    return -ENOMEM;
    }
    new_refs.count = count + 1;
    for (i = 0; i < count; ++i) {
    new_refs.events[i] = refs.events[i];
    }
    new_refs.events[i] = user_event_get(user);
    rcu_assign_pointer(info.refs, new_refs);
    if (refs) {
    kfree_rcu(refs, rcu);
    }
    return i;
    }
#[no_mangle]
unsafe extern "C" fn user_reg_get(ureg: *mut user_reg , kreg: *mut user_reg) -> c_long {
    let mut size = 0;
    let mut ret = 0;
    ret = get_user(size, &ureg.size);
    if (ret) {
    return ret;
    }
    if (size > PAGE_SIZE) {
    return -E2BIG;
    }
    if (size < offsetofend(user_reg, write_index)) {
    return -EINVAL;
    }
    ret = copy_struct_from_user(kreg, sizeof!(*kreg), ureg, size);
    if (ret) {
    return ret;
    }
// Ensure only valid flags
    if (kreg.flags & ~(USER_EVENT_REG_MAX-1)) {
    return -EINVAL;
    }
// Ensure supported size
    match (kreg.enable_size) {
    4 => {
// 32-bit
    // break;

    }
    8 => {
// 64-bit
    // break;

    }
    _ => {
    return -EINVAL;
    }
    }
// Ensure natural alignment
    if (kreg.enable_addr % kreg.enable_size) {
    return -EINVAL;
    }
// Ensure bit range for size
    if (kreg.enable_bit > (kreg.enable_size * BITS_PER_BYTE) - 1) {
    return -EINVAL;
    }
// Ensure accessible
    if (!access_ok((uintptr_t)kreg.enable_addr,
    kreg.enable_size)) {
    return -EFAULT;
    }
    kreg.size = size;
    return 0;
    }
//
// Registers a user_event on behalf of a user process.
//
#[no_mangle]
pub unsafe extern "C" fn user_events_ioctl_reg(info: *mut user_event_file_info, uarg: c_ulong) -> c_long {
    let mut ureg = uarg;
pub static mut reg: usize = 0;
pub static mut user: *mut c_void = core::ptr::null_mut();
pub static mut enabler: *mut c_void = core::ptr::null_mut();
pub static mut name: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    let mut write_result = 0;
    ret = user_reg_get(ureg, &reg);
    if (ret) {
    return ret;
    }
//
// Prevent users from using the same address and bit multiple times
// within the same mm address space. This can cause unexpected behavior
// for user processes that is far easier to debug if this is explicitly
// an error upon registering.
//
    if (current_user_event_enabler_exists((unsigned long)reg.enable_addr,
    reg.enable_bit)) {
    return -EADDRINUSE;
    }
    name = strndup_user((uintptr_t)reg.name_args,
    MAX_EVENT_DESC);
    if (IS_ERR(name)) {
    ret = PTR_ERR(name);
    return ret;
    }
    ret = user_event_parse_cmd(info.group, name, &user, reg.flags);
    if (ret) {
    kfree(name);
    return ret;
    }
    ret = user_events_ref_add(info, user);
// No longer need parse ref, ref_add either worked or not
    user_event_put(user, false);
// Positive number is index and valid
    if (ret < 0) {
    return ret;
    }
//
// user_events_ref_add succeeded:
// At this point we have a user_event, it's lifetime is bound by the
// reference count, not this file. If anything fails, the user_event
// still has a reference until the file is released. During release
// any remaining references (from user_events_ref_add) are decremented.
//
// Attempt to create an enabler, which too has a lifetime tied in the
// same way for the event. Once the task that caused the enabler to be
// created exits or issues exec() then the enablers it has created
// will be destroyed and the ref to the event will be decremented.
//
    enabler = user_event_enabler_create(&reg, user, &write_result);
    if (!enabler) {
    return -ENOMEM;
    }
// Write failed/faulted, give error back to caller
    if (write_result) {
    return write_result;
    }
    put_user((u32)ret, &ureg.write_index);
    return 0;
    }
//
// Deletes a user_event on behalf of a user process.
//
#[no_mangle]
pub unsafe extern "C" fn user_events_ioctl_del(info: *mut user_event_file_info, uarg: c_ulong) -> c_long {
    let mut ubuf = uarg;
pub static mut name: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    name = strndup_user(ubuf, MAX_EVENT_DESC);
    if (IS_ERR(name)) {
    return PTR_ERR(name);
    }
// event_mutex prevents dyn_event from racing
    mutex_lock(&event_mutex);
    ret = delete_user_event(info.group, name);
    mutex_unlock(&event_mutex);
    kfree(name);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn user_unreg_get(ureg: *mut user_unreg, kreg: *mut user_unreg) -> c_long {
    let mut size = 0;
    let mut ret = 0;
    ret = get_user(size, &ureg.size);
    if (ret) {
    return ret;
    }
    if (size > PAGE_SIZE) {
    return -E2BIG;
    }
    if (size < offsetofend(user_unreg, disable_addr)) {
    return -EINVAL;
    }
    ret = copy_struct_from_user(kreg, sizeof!(*kreg), ureg, size);
// Ensure no reserved values, since we don't support any yet
    if (kreg.__reserved || kreg.__reserved2) {
    return -EINVAL;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn user_event_mm_clear_bit(user_mm: *mut user_event_mm, uaddr: c_ulong, bit: c_uchar, flags: c_ulong) -> c_int {
pub static mut enabler: usize = 0;
    let mut result = 0;
pub static mut attempt: c_int = 0;
    memset(&enabler, 0, sizeof!(enabler));
    enabler.addr = uaddr;
    enabler.values = bit | flags;
// label;
// Prevents state changes from racing with new enablers
    mutex_lock(&event_mutex);
// Force the bit to be cleared, since no event is attached
    mmap_read_lock(user_mm.mm);
    result = user_event_enabler_write(user_mm, &enabler, false, &attempt);
    mmap_read_unlock(user_mm.mm);
    mutex_unlock(&event_mutex);
    if (result) {
// Attempt to fault-in and retry if it worked
    if (!user_event_mm_fault_in(user_mm, uaddr, attempt)) {
// goto;
    }
    }
    return result;
    }
//
// Unregisters an enablement address/bit within a task/user mm.
//
#[no_mangle]
unsafe extern "C" fn user_events_ioctl_unreg(uarg: c_ulong) -> c_long {
    let mut ureg = uarg;
    let mut mm = current.user_event_mm;
    let mut enabler = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
pub static mut reg: usize = 0;
    let mut flags = 0;
    let mut ret = 0;
    ret = user_unreg_get(ureg, &reg);
    if (ret) {
    return ret;
    }
    if (!mm) {
    return -ENOENT;
    }
    flags = 0;
    ret = -ENOENT;
//
// Flags freeing and faulting are used to indicate if the enabler is in
// use at all. When faulting is set a page-fault is occurring asyncly.
// During async fault if freeing is set, the enabler will be destroyed.
// If no async fault is happening, we can destroy it now since we hold
// the event_mutex during these checks.
//
    mutex_lock(&event_mutex);
    list_for_each_entry_safe(enabler, next, &mm.enablers, mm_enablers_link) {
    if (enabler.addr == reg.disable_addr &&
    ENABLE_BIT(enabler) == reg.disable_bit) {
    set_bit(ENABLE_VAL_FREEING_BIT, ENABLE_BITOPS(enabler));
// We must keep compat flags for the clear
    flags |= enabler.values & ENABLE_VAL_COMPAT_MASK;
    if (!test_bit(ENABLE_VAL_FAULTING_BIT, ENABLE_BITOPS(enabler))) {
    user_event_enabler_destroy(enabler);
    }
// Removed at least one
    ret = 0;
    }
    }
    mutex_unlock(&event_mutex);
// Ensure bit is now cleared for user, regardless of event status
    if (!ret) {
    ret = user_event_mm_clear_bit(mm, reg.disable_addr,
    reg.disable_bit, flags);
    }
    return ret;
    }
//
// Handles the ioctl from user mode to register or alter operations.
//
#[no_mangle]
pub unsafe extern "C" fn user_events_ioctl(file: *mut file, cmd: c_uint, uarg: c_ulong) -> c_long {
    let mut info = file.private_data;
    let mut group = info.group;
pub static mut ret: c_long = 0;
    match (cmd) {
    DIAG_IOCSREG => {
    mutex_lock(&group.reg_mutex);
    ret = user_events_ioctl_reg(info, uarg);
    mutex_unlock(&group.reg_mutex);
    // break;
    }
    DIAG_IOCSDEL => {
    mutex_lock(&group.reg_mutex);
    ret = user_events_ioctl_del(info, uarg);
    mutex_unlock(&group.reg_mutex);
    // break;
    }
    DIAG_IOCSUNREG => {
    mutex_lock(&group.reg_mutex);
    ret = user_events_ioctl_unreg(uarg);
    mutex_unlock(&group.reg_mutex);
    // break;
    }
    }
    return ret;
    }
//
// Handles the final close of the file from user mode.
//
#[no_mangle]
unsafe extern "C" fn user_events_release(node: *mut inode, file: *mut file) -> c_int {
    let mut info = file.private_data;
pub static mut group: *mut c_void = core::ptr::null_mut();
pub static mut refs: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    if (!info) {
    return -EINVAL;
    }
    group = info.group;
//
// Ensure refs cannot change under any situation by taking the
// register mutex during the final freeing of the references.
//
    mutex_lock(&group.reg_mutex);
    refs = info.refs;
    if (!refs) {
// goto;
    }
//
// The lifetime of refs has reached an end, it's tied to this file.
// The underlying user_events are ref counted, and cannot be freed.
// After this decrement, the user_events may be freed elsewhere.
//
    for (i = 0; i < refs.count; ++i) {
    user_event_put(refs.events[i], false);
    }
// label;
    file.private_data = core::ptr::null_mut();
    mutex_unlock(&group.reg_mutex);
    kfree(refs);
    kfree(info);
    return 0;
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn user_seq_start(m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    if (*pos) {
    return core::ptr::null_mut();
    }
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn user_seq_next(m: *mut seq_file, p: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    ++*pos;
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn user_seq_stop(m: *mut seq_file, p: *mut c_void) {
    }
#[no_mangle]
unsafe extern "C" fn user_seq_show(m: *mut seq_file, p: *mut c_void) -> c_int {
    let mut group = m.private;
pub static mut user: *mut c_void = core::ptr::null_mut();
    let mut status = 0;
    int i, active = 0, busy = 0;
    if (!group) {
    return -EINVAL;
    }
    mutex_lock(&group.reg_mutex);
    hash_for_each(group.register_table, i, user, node) {
    status = user.status;
    seq_puts(m, EVENT_TP_NAME(user));
    if (status != 0) {
    seq_puts(m, " # Used by");
    if (status & EVENT_STATUS_FTRACE) {
    seq_puts(m, " ftrace");
    }
    if (status & EVENT_STATUS_PERF) {
    seq_puts(m, " perf");
    }
    if (status & EVENT_STATUS_OTHER) {
    seq_puts(m, " other");
    }
    busy += 1;
    }
    seq_putc(m, '\n');
    active += 1;
    }
    mutex_unlock(&group.reg_mutex);
    seq_putc(m, '\n');
    seq_printf(m, "Active: %d\n", active);
    seq_printf(m, "Busy: %d\n", busy);
    return 0;
    }
pub static mut seq_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn user_status_open(node: *mut inode, file: *mut file) -> c_int {
pub static mut group: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    group = current_user_event_group();
    if (!group) {
    return -ENOENT;
    }
    ret = seq_open(file, &user_seq_ops);
    if (!ret) {
// Chain group to seq_file
    let mut m = file.private_data;
    m.private = group;
    }
    return ret;
    }
pub static mut file_operations: usize = 0;
//
// Creates a set of tracefs files to allow user mode interactions.
//
#[no_mangle]
unsafe extern "C" fn create_user_tracefs() -> c_int {
    let mut edata = core::ptr::null_mut();
    let mut emmap = core::ptr::null_mut();
    edata = tracefs_create_file("user_events_data", TRACE_MODE_WRITE,
    core::ptr::null_mut(), core::ptr::null_mut(), &user_data_fops);
    if (!edata) {
    pr_warn!("Could not create tracefs 'user_events_data' entry\n");
// goto;
    }
    emmap = tracefs_create_file("user_events_status", TRACE_MODE_READ,
    core::ptr::null_mut(), core::ptr::null_mut(), &user_status_fops);
    if (!emmap) {
    tracefs_remove(edata);
    pr_warn!("Could not create tracefs 'user_events_mmap' entry\n");
// goto;
    }
    return 0;
// label;
    return -ENODEV;
    }
#[no_mangle]
pub unsafe extern "C" fn set_max_user_events_sysctl(table: *mut ctl_table, write: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    let mut ret = 0;
    mutex_lock(&event_mutex);
    ret = proc_douintvec(table, write, buffer, lenp, ppos);
    mutex_unlock(&event_mutex);
    return ret;
    }
pub static mut ctl_table: usize = 0;
#[no_mangle]
unsafe extern "C" fn trace_events_user_init() -> c_int {
    let mut ret = 0;
    fault_cache = KMEM_CACHE(user_event_enabler_fault, 0);
    if (!fault_cache) {
    return -ENOMEM;
    }
    init_group = user_event_group_create();
    if (!init_group) {
    kmem_cache_destroy(fault_cache);
    return -ENOMEM;
    }
    ret = create_user_tracefs();
    if (ret) {
    pr_warn!("user_events could not register with tracefs\n");
    user_event_group_destroy(init_group);
    kmem_cache_destroy(fault_cache);
    init_group = core::ptr::null_mut();
    return ret;
    }
    if (dyn_event_register(&user_event_dops)) {
    pr_warn!("user_events could not register with dyn_events\n");
    }
    register_sysctl_init("kernel", user_event_sysctls);
    return 0;
    }
    fs_initcall!(trace_events_user_init);