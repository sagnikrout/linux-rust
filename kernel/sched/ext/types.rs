//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/sched/ext/types.h
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
// Early sched_ext type definitions.
//
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.
// Copyright (c) 2026 Tejun Heo <tj@kernel.org>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scx_consts {
    SCX_DSP_DFL_MAX_BATCH		= 32,
    SCX_DSP_MAX_LOOPS		= 32,
    SCX_WATCHDOG_MAX_TIMEOUT	= 30 * HZ,

// rescue knob defaults and limits, see scx_rescue_timerfn()
    SCX_RESCUE_DFL_BW_PPT		= 20,		/* parts per thousand, 2% */
    SCX_RESCUE_MAX_BW_PPT		= 250,		/* 25% */
    SCX_RESCUE_DISABLE		= U32_MAX,	/* disables rescue */
    SCX_RESCUE_DFL_QUANTUM_US	= 5000,
    SCX_RESCUE_MIN_QUANTUM_US	= 1000,
    SCX_RESCUE_MAX_QUANTUM_US	= 100000,
    SCX_RESCUE_MIN_SLICE_US		= 1000,		/* floor of the divided slice */
    SCX_RESCUE_OVERLOAD_MULT	= 16,		/* overload threshold in funding periods */
    SCX_RESCUE_MIN_OVERLOAD_MS	= 1000,
    SCX_RESCUE_MAX_OVERLOAD_MS	= 15000,

// per-CPU chunk size for p->scx.tid allocation, see scx_alloc_tid()
    SCX_TID_CHUNK			= 1024,

    SCX_EXIT_BT_LEN			= 64,
    SCX_EXIT_MSG_LEN		= 1024,
    SCX_EXIT_DUMP_DFL_LEN		= 32768,

    SCX_CPUPERF_ONE			= SCHED_CAPACITY_SCALE,

//
// Iterating all tasks may take a while. Periodically drop
// scx_tasks_lock to avoid causing e.g. CSD and RCU stalls.
//
    SCX_TASK_ITER_BATCH		= 32,

    SCX_BYPASS_HOST_NTH		= 2,

    SCX_BYPASS_LB_DFL_INTV_US	= 500 * USEC_PER_MSEC,
    SCX_BYPASS_LB_DONOR_PCT		= 125,
    SCX_BYPASS_LB_MIN_DELTA_DIV	= 4,
    SCX_BYPASS_LB_BATCH		= 256,

    SCX_REENQ_MAX_REPEAT		= 256,

    SCX_SUB_MAX_DEPTH		= 4,
}

//
// Per-cid topology info. For each topology level (core, LLC, node) and shard,
// records the first cid in the unit and its global index. Global indices are
// consecutive integers assigned in cid-walk order, so e.g. core_idx ranges over
// [0, nr_cores_at_init) with no gaps. No-topo cids have core/LLC/node fields
// set to -1 but always have valid shard assignments.
//
// Shards are contiguous CID ranges used as scalable locking/work domains for
// sub-scheduler operations. By default each LLC becomes one shard, split into
// smaller shards if the LLC exceeds the target size. No-topo cids are packed
// into their own max-sized shards.
//
// @core_cid: first cid of this cid's core (smt-sibling group)
// @core_idx: global index of that core, in [0, nr_cores_at_init)
// @llc_cid: first cid of this cid's LLC
// @llc_idx: global index of that LLC, in [0, nr_llcs_at_init)
// @node_cid: first cid of this cid's NUMA node
// @node_idx: global index of that node, in [0, nr_nodes_at_init)
// @shard_cid: first cid of this cid's shard
// @shard_idx: global index of that shard, in [0, scx_nr_cid_shards)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scx_cid_topo {
    pub core_cid: i32,
    pub core_idx: i32,
    pub llc_cid: i32,
    pub llc_idx: i32,
    pub node_cid: i32,
    pub node_idx: i32,
    pub shard_cid: i32,
    pub shard_idx: i32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scx_cid_consts {
    SCX_CID_SHARD_SIZE_DFL		= 24,
    SCX_CID_SHARD_MAX_CPUS		= 512,
}

//
// Per-shard metadata for O(1) shard->cid-range lookup.
//
// @base_cid: first cid of the shard
// @nr_cids: number of cids in the shard
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scx_cid_shard {
    pub base_cid: i32,
    pub nr_cids: i32,
}

//
// cmask: variable-length, base-windowed bitmap over cid space
// -----------------------------------------------------------
//
// A cmask covers the cid range [base, base + nr_cids). bits[] is aligned to the
// global 64-cid grid: bits[0] spans [base & ~63, (base & ~63) + 64), so the
// first (base & 63) bits of bits[0] are head padding and the trailing bits of
// the last active word past base + nr_cids are tail padding. Both stay zero;
// all mutating helpers preserve that. Words past the last active word are not
// read by any helper and have no constraint.
//
// Grid alignment means two cmasks always address bits[] against the same global
// 64-cid windows, so cross-cmask word ops (AND, OR, ...) reduce to
//
// dst->bits[i] OP= src->bits[i - delta]
//
// with no bit-shifting, regardless of how the two bases relate mod 64.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scx_cmask {
    pub base: u32,
    pub nr_cids: u32,
    pub alloc_words: u32,
    pub bits: [u64; 0],
}

//
// Number of u64 words of bits[] storage that covers @nr_cids regardless of base
// alignment. The +1 absorbs up to 63 bits of head padding when base is not
// 64-aligned - always allocating one extra word beats branching on base or
// splitting the compute. The u64 cast keeps the +63 from wrapping when @nr_cids
// is near U32_MAX, so callers bounds-checking the result against @alloc_words
// catch the overflow instead of seeing a small value.
//

//
// __SCX_CMASK_DEFINE - Define an on-stack cmask with explicit storage capacity
// @NAME: variable name to define
// @BASE: first cid of the active range
// @NR_CIDS: active range length
// @ALLOC_CIDS: storage capacity in cids, at least @NR_CIDS
//
// @NAME aliases zero-initialized storage with the active range set to
// [BASE, BASE + NR_CIDS). Use scx_cmask_reframe() to reshape later, up to
// @ALLOC_CIDS.
//

//
// SCX_CMASK_DEFINE - Define an on-stack cmask on tight storage
// @NAME: variable name to define
// @BASE: first cid of the active range
// @NR_CIDS: active range length, also storage capacity
//
// @NAME aliases zero-initialized storage with the active range and storage
// both [BASE, BASE + NR_CIDS).
//

//
// SCX_CMASK_DEFINE_SHARD - Define an on-stack cmask sized to one shard
// @NAME: variable name to define
// @BASE: first cid of the active range
// @NR_CIDS: active range length, must be <= SCX_CID_SHARD_MAX_CPUS
//
// Storage is fixed at SCX_CID_SHARD_MAX_CPUS, active range framed by
// (BASE, NR_CIDS). Passing NR_CIDS > SCX_CID_SHARD_MAX_CPUS leaves the
// cmask claiming more bits than storage holds and subsequent cmask
// operations will overrun.
//

//
// scx_cmask_ref: validated reference to a BPF-arena cmask.
//
// scx_cmask_ref_init() snapshots @base/@nr_cids. The snapshot is what
// downstream code uses for sizing - the live header can be mutated concurrently
// by BPF.
//
// scx_cmask_ref_shard() reads one shard into a cmask. scx_cmask_ref_or() and
// scx_cmask_ref_copy() write back into the referenced arena cmask, bounded by
// the snapshot.
//
// Typical input use:
//
// struct scx_cmask_ref ref;
// SCX_CMASK_DEFINE(shard, 0, SCX_CID_SHARD_MAX_CPUS);
// s32 idx, ret;
//
// ret = scx_cmask_ref_init(sch, src, &ref);
// if (ret < 0)
// return ret;
//
// while (idx < ref.shard_end) {
// scx_cmask_ref_shard(&ref, idx, shard);
// if (!shard->nr_cids)
// continue;
// ... use idx and shard ...
// }
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scx_cmask_ref {
    pub sch: *mut scx_sched,
    pub src: *mut scx_cmask,
    pub base: u32,
    pub nr_cids: u32,
    pub shard_first: i32,
    pub shard_end: i32,
}