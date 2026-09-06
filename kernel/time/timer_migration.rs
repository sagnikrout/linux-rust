//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/time/timer_migration.h
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
// Per group capacity. Must be a power of 2!
pub const TMIGR_CHILDREN_PER_GROUP: c_int = 8;
//
// struct tmigr_hierarchy - a hierarchy associated to a given CPU capacity.
// Homogeneous systems have only one hierarchy.
// Heterogenous have one hierarchy per CPU capacity.
// @cpumask:	CPUs belonging to this hierarchy
// @root:	The current root of the hierarchy
// @capacity:	CPU capacity associated to this hierarchy
// @node:	Node in the global hierarchy list
// @level_list:	Per level lists of tmigr groups
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmigr_hierarchy {
    pub cpumask: *mut cpumask,
    pub root: *mut tmigr_group,
    pub capacity: c_ulong,
    pub node: list_head,
    pub level_list: [list_head; 0],
}

//
// struct tmigr_event - a timer event associated to a CPU
// @nextevt:	The node to enqueue an event in the parent group queue
// @cpu:	The CPU to which this event belongs
// @ignore:	Hint whether the event could be ignored; it is set when
// CPU or group is active;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmigr_event {
    pub nextevt: timerqueue_node,
    pub cpu: c_uint,
    pub ignore: bool,
}

//
// struct tmigr_group - timer migration hierarchy group
// @lock:		Lock protecting the event information and group hierarchy
// information during setup
// @parent:		Pointer to the parent group. Pointer is updated when a
// new hierarchy level is added because of a CPU coming
// online the first time. Once it is set, the pointer will
// not be removed or updated. When accessing parent pointer
// lock less to decide whether to abort a propagation or
// not, it is not a problem. The worst outcome is an
// unnecessary/early CPU wake up. But do not access parent
// pointer several times in the same 'action' (like
// activation, deactivation, check for remote expiry,...)
// without holding the lock as it is not ensured that value
// will not change.
// @groupevt:		Next event of the group which is only used when the
// group is !active. The group event is then queued into
// the parent timer queue.
// Ignore bit of @groupevt is set when the group is active.
// @next_expiry:	Base monotonic expiry time of the next event of the
// group; It is used for the racy lockless check whether a
// remote expiry is required; it is always reliable
// @events:		Timer queue for child events queued in the group
// @migr_state:		State of the group (see union tmigr_state)
// @level:		Hierarchy level of the group; Required during setup
// @numa_node:		Required for setup only to make sure CPU and low level
// group information is NUMA local. It is set to NUMA node
// as long as the group level is per NUMA node (level <
// tmigr_crossnode_level); otherwise it is set to
// NUMA_NO_NODE
// @num_children:	Counter of group children to make sure the group is only
// filled with TMIGR_CHILDREN_PER_GROUP; Required for setup
// only
// @groupmask:		mask of the group in the parent group; is set during
// setup and will never change; can be read lockless
// @list:		List head that is added to the per level
// tmigr_level_list; is required during setup when a
// new group needs to be connected to the existing
// hierarchy groups
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmigr_group {
    pub lock: raw_spinlock_t,
    pub parent: *mut tmigr_group,
    pub groupevt: tmigr_event,
    pub next_expiry: u64,
    pub events: timerqueue_head,
    pub migr_state: core::sync::atomic::AtomicI32,
    pub level: c_uint,
    pub numa_node: c_int,
    pub num_children: c_uint,
    pub groupmask: u8,
    pub list: list_head,
}

//
// struct tmigr_cpu - timer migration per CPU group
// @lock:		Lock protecting the tmigr_cpu group information
// @available:		Indicates whether the CPU is available for handling
// global timers. In the deactivate path it is required to
// know whether the migrator in the top level group is to
// be set offline, while a timer is pending. Then another
// available CPU needs to be notified to take over the
// migrator role. Furthermore the information is required
// in the CPU hotplug path as the CPU is able to go idle
// before the timer migration hierarchy hotplug callback is
// reached.  During this phase, the CPU has to handle the
// global timers on its own and must not act as a migrator.
//
// @idle:		Indicates whether the CPU is idle in the timer migration
// hierarchy
// @remote:		Is set when timers of the CPU are expired remotely
// @tmgroup:		Pointer to the parent group
// @groupmask:		mask of tmigr_cpu in the parent group
// @wakeup:		Stores the first timer when the timer migration
// hierarchy is completely idle and remote expiry was done;
// is returned to timer code in the idle path and is only
// used in idle path.
// @cpuevt:		CPU event which could be enqueued into the parent group
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmigr_cpu {
    pub lock: raw_spinlock_t,
    pub available: bool,
    pub idle: bool,
    pub remote: bool,
    pub tmgroup: *mut tmigr_group,
    pub groupmask: u8,
    pub wakeup: u64,
    pub cpuevt: tmigr_event,
}

//
// union tmigr_state - state of tmigr_group
// @state:	Combined version of the state - only used for atomic
// read/cmpxchg function
// &anon struct: Split version of the state - only use the struct members to
// update information to stay independent of endianness
// @active:	Contains each mask bit of the active children
// @migrator:	Contains mask of the child which is migrator
// @seq:	Sequence counter needs to be increased when an update
// to the tmigr_state is done. It prevents a race when
// updates in the child groups are propagated in changed
// order. Detailed information about the scenario is
// given in the documentation at the begin of
// timer_migration.c.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union tmigr_state {
    pub state: u32,
    pub active: u8,
    pub migrator: u8,
    pub seq: u16,
    pub __packed: },
}

extern "C" {
    pub fn tmigr_handle_remote();
}
extern "C" {
    pub fn tmigr_requires_handle_remote() -> bool;
}
extern "C" {
    pub fn tmigr_cpu_activate();
}
extern "C" {
    pub fn tmigr_cpu_deactivate(nextevt: u64) -> u64;
}
extern "C" {
    pub fn tmigr_cpu_new_timer(nextevt: u64) -> u64;
}
extern "C" {
    pub fn tmigr_quick_check(nextevt: u64) -> u64;