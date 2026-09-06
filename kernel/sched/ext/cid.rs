//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/sched/ext/cid.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Topological CPU IDs (cids)
// --------------------------
//
// Raw cpu numbers are clumsy for sharding work and communication across
// topology units, especially from BPF: the space can be sparse, numerical
// closeness doesn't imply topological closeness (x86 hyperthreading often puts
// SMT siblings far apart), and a range of cpu ids doesn't mean anything.
// Sub-scheds make this acute - cpu allocation, revocation and other state are
// constantly communicated across sub-scheds, and passing whole cpumasks scales
// poorly with cpu count. cpumasks are also awkward in BPF: a variable-length
// kernel type sized for the maximum NR_CPUS (4k), with verbose helper sequences
// for every op.
//
// cids give every cpu a dense, topology-ordered id. CPUs sharing a core, LLC or
// NUMA node get contiguous cid ranges, so a topology unit becomes a (start,
// length) slice of cid space. Communication can pass a slice instead of a
// cpumask, and BPF code can process, for example, a u64 word's worth of cids at
// a time.
//
// The mapping is built once at root scheduler enable time by walking the
// topology of online cpus only. Going by online cpus is out of necessity:
// depending on the arch, topology info isn't reliably available for offline
// cpus. The expected usage model is restarting the scheduler on hotplug events
// so the mapping is rebuilt against the new online set. A scheduler that wants
// to handle hotplug without a restart can provide its own cid and shard mapping
// through the override interface.
//
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.
// Copyright (c) 2026 Tejun Heo <tj@kernel.org>
//

//
// Cid space (total is always num_possible_cpus()) is laid out with
// topology-annotated cids first, then no-topo cids at the tail. The
// topology-annotated block covers the cpus that were online when scx_cid_init()
// ran and remains valid even after those cpus go offline. The tail block covers
// possible-but-not-online cpus and carries all-(-1) topo info (see
// scx_cid_topo); callers detect it via the -1 sentinels.
//
// See the comment above the table definitions in cid.c for the
// memory-ordering and visibility contract.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scx_cid_tables {
    pub nr_shards: u32,
//     pub /: *mut *mut *mut s16 cid_to_cpu; / [num_possible_cpus()],
//     pub /: *mut *mut *mut s16 cpu_to_cid; / [nr_cpu_ids],
//     pub /: *mut *mut *mut s32 cid_to_shard; / [num_possible_cpus()],
//     pub /: *mut *mut *mut s32 shard_node; / [num_possible_cpus()],
//     pub /: *mut *mut *mut scx_cid_shard shard_ranges; / [num_possible_cpus()],
//     pub /: *mut *mut *mut scx_cid_topo topo; / [num_possible_cpus()],
    pub rcu: rcu_head,
}

extern "C" {
    pub fn scx_cmask_clear(m: *mut scx_cmask);
}
extern "C" {
    pub fn scx_cmask_fill(m: *mut scx_cmask);
}
extern "C" {
    pub fn scx_cmask_and(dst: *mut scx_cmask, src: *const scx_cmask);
}
extern "C" {
    pub fn scx_cmask_or(dst: *mut scx_cmask, src: *const scx_cmask);
}
extern "C" {
    pub fn scx_cmask_copy(dst: *mut scx_cmask, src: *const scx_cmask);
}
extern "C" {
    pub fn scx_cmask_andnot(dst: *mut scx_cmask, src: *const scx_cmask);
}
extern "C" {
    pub fn scx_cmask_subset(sub: *const scx_cmask, super: *const scx_cmask) -> bool;
}
extern "C" {
    pub fn scx_cmask_intersects(a: *const scx_cmask, b: *const scx_cmask) -> bool;
}
extern "C" {
    pub fn scx_cmask_empty(m: *const scx_cmask) -> bool;
}
extern "C" {
    pub fn scx_cid_init(sch: *mut scx_sched) -> i32;
}
extern "C" {
    pub fn scx_cid_publish_tables();
}
extern "C" {
    pub fn scx_cid_retire_tables();
}
extern "C" {
    pub fn scx_cid_kfunc_init() -> c_int;
}
//
// cid_valid - Verify a cid value, to be used on ops input args
// @sch: scx_sched to abort on error
// @cid: cid which came from a BPF ops
//
// Return true if @cid is in [0, num_possible_cpus()). On failure, trigger
// scx_error() and return false.
//
// __scx_cid_to_cpu - Unchecked cid->cpu table lookup
// @cid: cid to look up. Must be in [0, num_possible_cpus()).
//
// Intended for callsites that have already validated @cid and that run on a
// live scheduler, which guarantees the tables are published and stable.
//
// __scx_cpu_to_cid - Unchecked cpu->cid table lookup
// @cpu: cpu to look up. Must be a valid possible cpu id.
//
// Same usage constraints as __scx_cid_to_cpu().
//
// scx_cid_to_cpu - Translate @cid to its cpu
// @sch: scx_sched for error reporting
// @cid: cid to look up
//
// Return the cpu for @cid or a negative errno on failure. Invalid cid triggers
// scx_error() on @sch. The mapping is stable while the scheduler is live.
//
// Return -EINVAL without triggering scx_error() if no tables have been
// published yet, which a prog-facing kfunc can observe while racing the root
// scheduler enable.
//
// scx_cpu_to_cid - Translate @cpu to its cid
// @sch: scx_sched for error reporting
// @cpu: cpu to look up
//
// Return the cid for @cpu or a negative errno on failure. Invalid cpu triggers
// scx_error() on @sch. Same usage rules as scx_cid_to_cpu().
//
// scx_is_cid_type - Test whether the active scheduler hierarchy is cid-form
//
extern "C" {
    pub fn static_branch_unlikely(_arg: &__scx_is_cid_type) -> return;
}
extern "C" {
    pub fn likely(m->nr_cids: cid >= m->base && cid < m->base +) -> return;
}
// Word in bits[] covering @cid. @cid must satisfy __scx_cmask_contains().
//
// __scx_cmask_init - Initialize @m with explicit storage capacity
// @m: cmask to initialize
// @base: first cid of the active range
// @nr_cids: number of cids in the active range
// @alloc_cids: storage capacity in cids, at least @nr_cids
//
// Use when storage is sized larger than the initial active range. All of
// bits[] is zeroed.
//
// scx_cmask_init - Initialize @m on tight storage
// @m: cmask to initialize
// @base: first cid of the active range
// @nr_cids: number of cids in the active range
//
// All of bits[] is zeroed.
//
// scx_cmask_reframe - Reshape @m's active range without resizing storage
// @m: cmask to reframe
// @base: new active range base
// @nr_cids: new active range length, must fit within @m->alloc_words
//
// Body bits within the new range become garbage - only the head and tail
// words are zeroed to keep the padding invariant.
//
// __scx_cmask_word(cid, m) |= BIT_U64(cid & 63);
//
// scx_cmask_test - test whether @cid is set in @m
// @cid: cid to test
// @m: cmask to test
//
// Return %false if @cid is outside @m's active range. Otherwise return the
// bit's value. Read via READ_ONCE so callers can race set/clear writers.
//
extern "C" {
    pub fn READ_ONCE(_arg: *mut __scx_cmask_word(cid, 63: m)) & BIT_U64(cid &) -> return;
}
//
// Words of bits[] the active range spans, 0 if empty. Tighter than the storage
// SCX_CMASK_NR_WORDS() sizes for the worst-case base alignment.
//
// scx_cmask_for_each_cid - iterate set cids in @m
// @cid: s32 loop var that receives each set cid in turn
// @m: cmask to iterate
//
// Visits set bits within @m's active range in ascending order. Scans only the
// words the active range spans, where head and tail padding is kept zero, so
// no per-cid range check is needed.
//

//
// scx_cpu_arg() wraps a cpu arg being handed to an SCX op. For cid-form
// schedulers it resolves to the matching cid; for cpu-form it passes @cpu
// through. scx_cpu_ret() is the inverse for a cpu/cid returned from an op
// (currently only ops.select_cpu); it validates the BPF-supplied cid and
// triggers scx_error() on @sch if invalid.
//
extern "C" {
    pub fn __scx_cpu_to_cid(_arg: cpu) -> return;
}
extern "C" {
    pub fn scx_cid_to_cpu(_arg: sch, _arg: cpu_or_cid) -> return;
}
extern "C" {
    pub fn scx_cmask_ref_or(ref: *const scx_cmask_ref, src: *const scx_cmask);
}
extern "C" {
    pub fn scx_cmask_ref_copy(ref: *const scx_cmask_ref, src: *const scx_cmask);
}