//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rseq_types.h
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
// rseq_event::has_rseq contains the ABI version number so preserving it
// in AND operations requires a mask.
//
pub const RSEQ_HAS_RSEQ_VERSION_MASK: c_uint = 0xff;
//
// struct rseq_event - Storage for rseq related event management
// @all:		Compound to initialize and clear the data efficiently
// @events:		Compound to access events with a single load/store
// @sched_switch:	True if the task was scheduled and needs update on
// exit to user
// @ids_changed:	Indicator that IDs need to be updated
// @user_irq:		True on interrupt entry from user mode
// @has_rseq:		Greater than 0 if the task has a rseq pointer installed.
// Contains the RSEQ version number
// @error:		Compound error code for the slow path to analyze
// @fatal:		User space data corrupted or invalid
// @slowpath:		Indicator that slow path processing via TIF_NOTIFY_RESUME
// is required
//
// @sched_switch and @ids_changed must be adjacent and the combo must be
// 16bit aligned to allow a single store, when both are set at the same
// time in the scheduler.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rseq_event {
    pub all: u64,
    pub events: u32,
    pub sched_switch: u8,
    pub ids_changed: u8,
    pub user_irq: u8,
}

//
// struct rseq_ids - Cache for ids, which need to be updated
// @cpu_cid:	Compound of @cpu_id and @mm_cid to make the
// compiler emit a single compare on 64-bit
// @cpu_id:	The CPU ID which was written last to user space
// @mm_cid:	The MM CID which was written last to user space
// @node_id:	The node ID which was written last to user space
//
// @cpu_id, @mm_cid and @node_id are updated when the data is written to user space.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rseq_ids {
    pub cpu_cid: u64,
    pub cpu_id: u32,
    pub mm_cid: u32,
}

//
// union rseq_slice_state - Status information for rseq time slice extension
// @state:	Compound to access the overall state
// @enabled:	Time slice extension is enabled for the task
// @granted:	Time slice extension was granted to the task
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union rseq_slice_state {
    pub state: u16,
    pub enabled: u8,
    pub granted: u8,
}

//
// struct rseq_slice - Status information for rseq time slice extension
// @state:	Time slice extension state
// @expires:	The time when a grant expires
// @yielded:	Indicator for rseq_slice_yield()
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rseq_slice {
    pub state: rseq_slice_state,
    pub expires: u64,
    pub yielded: u8,
}

//
// struct rseq_data - Storage for all rseq related data
// @usrptr:	Pointer to the registered user space RSEQ memory
// @len:	Length of the RSEQ region
// @sig:	Signature of critical section abort IPs
// @event:	Storage for event management
// @ids:	Storage for cached CPU ID and MM CID
// @slice:	Storage for time slice extension data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rseq_data {
    pub usrptr: *mut rseq __user,
    pub len: u32,
    pub sig: u32,
    pub event: rseq_event,
    pub ids: rseq_ids,

    pub slice: rseq_slice,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rseq_data {

//
// struct sched_mm_cid - Storage for per task MM CID data
// @active:	MM CID is active for the task
// @cid:	The CID associated to the task either permanently or
// borrowed from the CPU
// @node:	Queued in the per MM MMCID list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_mm_cid {
    pub active: c_uint,
    pub cid: c_uint,
    pub node: hlist_node,
}

//
// struct mm_cid_pcpu - Storage for per CPU MM_CID data
// @cid:	The CID associated to the CPU either permanently or
// while a task with a CID is running
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm_cid_pcpu {
    pub cid: c_uint,
//
// struct mm_mm_cid - Storage for per MM CID data
// @pcpu:		Per CPU storage for CIDs associated to a CPU
// @mode:		Indicates per CPU and transition mode
// @max_cids:		The exclusive maximum CID value for allocation and convergence
// @irq_work:		irq_work to handle the affinity mode change case
// @work:		Regular work to handle the affinity mode change case
// @lock:		Spinlock to protect against affinity setting which can't take @mutex
// @mutex:		Mutex to serialize forks and exits related to this mm
// @user_list:		List of the MM CID users of a MM
// @nr_cpus_allowed:	The number of CPUs in the per MM allowed CPUs map. The map
// is growth only.
// @users:		The number of tasks sharing this MM. Separate from mm::mm_users
// as that is modified by mmget()/mm_put() by other entities which
// do not actually share the MM.
// @pcpu_thrs:		Threshold for switching back from per CPU mode
// @update_deferred:	A deferred switch back to per task mode is pending.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm_mm_cid {
// Hotpath read mostly members
    pub pcpu: *mut mm_cid_pcpu __percpu,
    pub mode: c_uint,
    pub max_cids: c_uint,
// Rarely used. Moves @lock and @mutex into the second cacheline
    pub irq_work: irq_work,
    pub work: work_struct,
    pub lock: raw_spinlock_t,
    pub mutex: mutex,
    pub user_list: hlist_head,
// Low frequency modified
    pub nr_cpus_allowed: c_uint,
    pub users: c_uint,
    pub pcpu_thrs: c_uint,
    pub update_deferred: c_uint,
    pub ____cacheline_aligned: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm_mm_cid {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_mm_cid {

