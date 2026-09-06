//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/cgroup/cpuset-internal.h
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


// SPDX-License-Identifier: GPL-2.0-or-later

// See "Frequency meter" comments, below.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fmeter {
//     pub /: *mut *mut int cnt; / unprocessed events count,
//     pub /: *mut *mut int val; / most recent output value,
//     pub /: *mut *mut time64_t time; / clock (secs) when val computed,
//     pub /: *mut *mut spinlock_t lock; / guards read or write of above,
}

//
// Invalid partition error code
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prs_errcode {
    PERR_NONE = 0,
    PERR_INVCPUS,
    PERR_INVPARENT,
    PERR_NOTPART,
    PERR_NOTEXCL,
    PERR_NOCPUS,
    PERR_HOTPLUG,
    PERR_CPUSEMPTY,
    PERR_HKEEPING,
    PERR_ACCESS,
    PERR_REMOTE,
}

// bits in struct cpuset flags field
// The various types of files and directories in a cpuset file system
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuset {
    pub css: cgroup_subsys_state,
//     pub /: *mut *mut unsigned long flags; / "unsigned long" so bitops work,
//
// On default hierarchy:
//
// The user-configured masks can only be changed by writing to
// cpuset.cpus and cpuset.mems, and won't be limited by the
// parent masks.
//
// The effective masks is the real masks that apply to the tasks
// in the cpuset. They may be changed if the configured masks are
// changed or hotplug happens.
//
// effective_mask == configured_mask & parent's effective_mask,
// and if it ends up empty, it will inherit the parent's mask.
//
// On legacy hierarchy:
//
// The user-configured masks are always the same with effective masks.
//
// user-configured CPUs and Memory Nodes allow to tasks
    pub cpus_allowed: cpumask_var_t,
    pub mems_allowed: nodemask_t,
// effective CPUs and Memory Nodes allow to tasks
    pub effective_cpus: cpumask_var_t,
    pub effective_mems: nodemask_t,
//
// Exclusive CPUs dedicated to current cgroup (default hierarchy only)
//
// The effective_cpus of a valid partition root comes solely from its
// effective_xcpus and some of the effective_xcpus may be distributed
// to sub-partitions below & hence excluded from its effective_cpus.
// For a valid partition root, its effective_cpus have no relationship
// with cpus_allowed unless its exclusive_cpus isn't set.
//
// This value will only be set if either exclusive_cpus is set or
// when this cpuset becomes a local partition root.
//
    pub effective_xcpus: cpumask_var_t,
//
// Exclusive CPUs as requested by the user (default hierarchy only)
//
// Its value is independent of cpus_allowed and designates the set of
// CPUs that can be granted to the current cpuset or its children when
// it becomes a valid partition root. The effective set of exclusive
// CPUs granted (effective_xcpus) depends on whether those exclusive
// CPUs are passed down by its ancestors and not yet taken up by
// another sibling partition root along the way.
//
// If its value isn't set, it defaults to cpus_allowed.
//
    pub exclusive_cpus: cpumask_var_t,
//
// This is old Memory Nodes tasks took on.
//
// - top_cpuset.old_mems_allowed is initialized to mems_allowed.
// - A new cpuset's old_mems_allowed is initialized when some
// task is moved into it.
// - old_mems_allowed is used in cpuset_migrate_mm() when we change
// cpuset.mems_allowed and have tasks' nodemask updated, and
// then old_mems_allowed is updated to mems_allowed.
//
    pub old_mems_allowed: nodemask_t,
//
// For linking impacted cpusets during an attach operation.
//
    pub attach_node: llist_node,
// partition root state
    pub partition_root_state: c_int,
//
// Whether cpuset is a remote partition.
// It used to be a list anchoring all remote partitions — we can switch back
// to a list if we need to iterate over the remote partitions.
//
    pub remote_partition: bool,
//
// number of SCHED_DEADLINE tasks attached to this cpuset, so that we
// know when to rebuild associated root domain bandwidth information.
//
    pub nr_deadline_tasks: core::sync::atomic::AtomicI32,
    pub nr_migrate_dl_tasks: c_int,
// DL bandwidth that needs destination reservation for this attach.
    pub sum_migrate_dl_bw: u64,
//
// CPU used for temporary DL bandwidth allocation during attach;
// -1 if no DL bandwidth was allocated in the current attach.
//
    pub dl_bw_cpu: c_int,
// Invalid partition error code, not lock protected
    pub prs_err: prs_errcode,
// Handle for cpuset.cpus.partition
    pub partition_file: cgroup_file,

//     pub /: *mut *mut fmeter fmeter; / memory_pressure filter,
// for custom sched domain
    pub relax_domain_level: c_int,
// Used to merge intersecting subsets for generate_sched_domains
    pub node: uf_node,

}

// Retrieve the cpuset for a task
extern "C" {
    pub fn css_cs(_arg: task_css(task, _arg: cpuset_cgrp_id)) -> return;
}
extern "C" {
    pub fn css_cs(_arg: cs->css.parent) -> return;
}
// convenient tests for these bits
extern "C" {
    pub fn css_is_online(!css_is_dying(&cs->css: &cs->css) &&) -> return;
}
extern "C" {
    pub fn test_bit(_arg: CS_CPU_EXCLUSIVE, _arg: &cs->flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: CS_MEM_EXCLUSIVE, _arg: &cs->flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: CS_MEM_HARDWALL, _arg: &cs->flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: CS_SCHED_LOAD_BALANCE, _arg: &cs->flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: CS_MEMORY_MIGRATE, _arg: &cs->flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: CS_SPREAD_PAGE, _arg: &cs->flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: CS_SPREAD_SLAB, _arg: &cs->flags) -> return;
}
//
// Helper routine for generate_sched_domains().
// Do cpusets a, b have overlapping effective cpus_allowed masks?
//
extern "C" {
    pub fn cpumask_intersects(_arg: a->effective_cpus, _arg: b->effective_cpus) -> return;
}
// jump label reference count + the top-level cpuset
extern "C" {
    pub fn cgroup_is_populated(_arg: cs->css.cgroup) -> return;
}
//
// cpuset_for_each_child - traverse online children of a cpuset
// @child_cs: loop cursor pointing to the current child
// @pos_css: used for iteration
// @parent_cs: target cpuset to walk children of
//
// Walk @child_cs through the online children of @parent_cs.  Must be used
// with RCU read locked.
//

//
// cpuset_for_each_descendant_pre - pre-order walk of a cpuset's descendants
// @des_cs: loop cursor pointing to the current descendant
// @pos_css: used for iteration
// @root_cs: target cpuset to walk ancestor of
//
// Walk @des_cs through the online descendants of @root_cs.  Must be used
// with RCU read locked.  The caller may modify @pos_css by calling
// css_rightmost_descendant() to skip subtree.  @root_cs is included in the
// iteration and the first node to be visited.
//

extern "C" {
    pub fn rebuild_sched_domains_locked();
}
extern "C" {
    pub fn cpuset_callback_lock_irq();
}
extern "C" {
    pub fn cpuset_callback_unlock_irq();
}
extern "C" {
    pub fn cpuset_update_tasks_cpumask(cs: *mut cpuset, new_cpus: *mut cpumask);
}
extern "C" {
    pub fn cpuset_update_tasks_nodemask(cs: *mut cpuset);
}
extern "C" {
    pub fn cpuset_update_flag(bit: cpuset_flagbits_t, cs: *mut cpuset, turning_on: c_int) -> c_int;
}
extern "C" {
    pub fn cpuset_common_seq_show(sf: *mut seq_file, v: *mut c_void) -> c_int;
}
extern "C" {
    pub fn cpuset_full_lock();
}
extern "C" {
    pub fn cpuset_full_unlock();
}
//
// cpuset-v1.c
//

extern "C" {
    pub fn cpuset1_update_tasks_flags(cs: *mut cpuset);
}
extern "C" {
    pub fn cpuset1_validate_change(cur: *mut cpuset, trial: *mut cpuset) -> c_int;
}
extern "C" {
    pub fn cpuset1_cpus_excl_conflict(cs1: *mut cpuset, cs2: *mut cpuset) -> bool;
}
extern "C" {
    pub fn cpuset1_init(cs: *mut cpuset);
}
extern "C" {
    pub fn cpuset1_online_css(css: *mut cgroup_subsys_state);
}