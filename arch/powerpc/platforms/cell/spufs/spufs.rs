//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/platforms/cell/spufs/spufs.h
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
// SPU file system
//
// (C) Copyright IBM Deutschland Entwicklung GmbH 2005
//
// Author: Arnd Bergmann <arndb@de.ibm.com>
//

pub const SPUFS_PS_MAP_SIZE: c_uint = 0x20000;
pub const SPUFS_MFC_MAP_SIZE: c_uint = 0x1000;
pub const SPUFS_CNTL_MAP_SIZE: c_uint = 0x1000;

pub const SPUFS_MSS_MAP_SIZE: c_uint = 0x1000;
// The magic number for our file system
// ctx->sched_flags
#[repr(C)]
#[derive(Copy, Clone)]
pub struct switch_log {
    pub wait: wait_queue_head_t,
    pub head: c_ulong,
    pub tail: c_ulong,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct switch_log_entry {
    pub tstamp: timespec64,
    pub spu_id: i32,
    pub type: u32,
    pub val: u32,
    pub timebase: u64,
    pub log: [}; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spu_context {
    pub /: *mut *mut *mut spu spu; / pointer to a physical SPU,
    pub /: *mut *mut spu_state csa; / SPU context save area.,
    pub /: *mut *mut spinlock_t mmio_lock; / protects mmio access,
    pub /: *mut *mut *mut address_space local_store; / local store mapping.,
    pub /: *mut *mut *mut address_space mfc; / 'mfc' area mappings.,
    pub /: *mut *mut *mut address_space cntl; / 'control' area mappings.,
    pub /: *mut *mut *mut address_space signal1; / 'signal1' area mappings.,
    pub /: *mut *mut *mut address_space signal2; / 'signal2' area mappings.,
    pub /: *mut *mut *mut address_space mss; / 'mss' area mappings.,
    pub /: *mut *mut *mut address_space psmap; / 'psmap' area mappings.,
    pub mapping_lock: mutex,
    pub /: *mut *mut u64 object_id; / user space pointer for GNU Debugger,
    pub state: { SPU_STATE_RUNNABLE, SPU_STATE_SAVED },
    pub state_mutex: mutex,
    pub run_mutex: mutex,
    pub owner: *mut mm_struct,
    pub kref: kref,
    pub ibox_wq: wait_queue_head_t,
    pub wbox_wq: wait_queue_head_t,
    pub stop_wq: wait_queue_head_t,
    pub mfc_wq: wait_queue_head_t,
    pub run_wq: wait_queue_head_t,
    pub tagwait: u32,
    pub ops: *mut spu_context_ops,
    pub reap_work: work_struct,
    pub flags: c_ulong,
    pub event_return: c_ulong,
    pub gang_list: list_head,
    pub gang: *mut spu_gang,
    pub prof_priv_kref: *mut kref,
    pub kref): *mut *mut void (  prof_priv_release) (struct kref,
// owner thread
    pub tid: pid_t,
// scheduler fields
    pub rq: list_head,
    pub time_slice: c_uint,
    pub sched_flags: c_ulong,
    pub cpus_allowed: cpumask_t,
    pub policy: c_int,
    pub prio: c_int,
    pub last_ran: c_int,
// statistics
// updates protected by ctx->state_mutex
    pub util_state: spu_utilization_state,
    pub /: *mut *mut unsigned long long tstamp; / time of last state switch,
    pub times: [c_ulonglong; SPU_UTIL_MAX],
    pub vol_ctx_switch: c_ulonglong,
    pub invol_ctx_switch: c_ulonglong,
    pub min_flt: c_ulonglong,
    pub maj_flt: c_ulonglong,
    pub hash_flt: c_ulonglong,
    pub slb_flt: c_ulonglong,
    pub /: *mut *mut unsigned long long slb_flt_base; / # at last ctx switch,
    pub class2_intr: c_ulonglong,
    pub /: *mut *mut unsigned long long class2_intr_base; / # at last ctx switch,
    pub libassist: c_ulonglong,
    pub stats: },
// context switch log
    pub switch_log: *mut switch_log,
    pub aff_list: list_head,
    pub aff_head: c_int,
    pub aff_offset: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spu_gang {
    pub list: list_head,
    pub mutex: mutex,
    pub kref: kref,
    pub contexts: c_int,
    pub aff_ref_ctx: *mut spu_context,
    pub aff_list_head: list_head,
    pub aff_mutex: mutex,
    pub aff_flags: c_int,
    pub aff_ref_spu: *mut spu,
    pub aff_sched_count: core::sync::atomic::AtomicI32,
    pub alive: c_int,
}

// Flag bits for spu_gang aff_flags
pub const AFF_OFFSETS_SET: c_int = 1;
pub const AFF_MERGED: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mfc_dma_command {
    pub /: *mut *mut int32_t pad; / reserved,
    pub /: *mut *mut uint32_t lsa; / local storage address,
    pub /: *mut *mut uint64_t ea; / effective address,
    pub /: *mut *mut uint16_t size; / transfer size,
    pub /: *mut *mut uint16_t tag; / command tag,
    pub /: *mut *mut uint16_t class; / class ID,
    pub /: *mut *mut uint16_t cmd; / command opcode,
}

// SPU context query/set operations.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spu_context_ops {
    pub data): *mut *mut *mut *mut int (mbox_read) (struct spu_context  ctx, u32,
    pub ctx): *mut *mut *mut u32(mbox_stat_read) (struct spu_context,
    pub events): *mut *mut *mut __poll_t (mbox_stat_poll)(struct spu_context ctx, __poll_t,
    pub data): *mut *mut *mut *mut int (ibox_read) (struct spu_context  ctx, u32,
    pub data): *mut *mut *mut int (wbox_write) (struct spu_context  ctx, u32,
    pub ctx): *mut *mut *mut u32(signal1_read) (struct spu_context,
    pub data): *mut *mut *mut void (signal1_write) (struct spu_context  ctx, u32,
    pub ctx): *mut *mut *mut u32(signal2_read) (struct spu_context,
    pub data): *mut *mut *mut void (signal2_write) (struct spu_context  ctx, u32,
    pub val): *mut *mut *mut void (signal1_type_set) (struct spu_context  ctx, u64,
    pub ctx): *mut *mut *mut u64(signal1_type_get) (struct spu_context,
    pub val): *mut *mut *mut void (signal2_type_set) (struct spu_context  ctx, u64,
    pub ctx): *mut *mut *mut u64(signal2_type_get) (struct spu_context,
    pub ctx): *mut *mut *mut u32(npc_read) (struct spu_context,
    pub data): *mut *mut *mut void (npc_write) (struct spu_context  ctx, u32,
    pub ctx): *mut *mut *mut u32(status_read) (struct spu_context,
    pub ctx): *mut *mut *mut *mut char(get_ls) (struct spu_context,
    pub data): *mut *mut *mut void (privcntl_write) (struct spu_context ctx, u64,
    pub ctx): *mut *mut *mut u32 (runcntl_read) (struct spu_context,
    pub data): *mut *mut *mut void (runcntl_write) (struct spu_context  ctx, u32,
    pub ctx): *mut *mut *mut void (runcntl_stop) (struct spu_context,
    pub ctx): *mut *mut *mut void (master_start) (struct spu_context,
    pub ctx): *mut *mut *mut void (master_stop) (struct spu_context,
    pub mode): *mut *mut *mut int (set_mfc_query)(struct spu_context  ctx, u32 mask, u32,
    pub ctx): *mut *mut *mut u32 (read_mfc_tagstatus)(struct spu_context,
    pub ctx): *mut *mut u32 (get_mfc_free_elements)(struct spu_context,
    pub cmd): *mut *mut mfc_dma_command,
    pub info): *mut *mut spu_dma_info,
    pub info): *mut *mut spu_proxydma_info,
    pub ctx): *mut *mut void (restart_dma)(struct spu_context,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spufs_inode_info {
    pub i_ctx: *mut spu_context,
    pub i_gang: *mut spu_gang,
    pub vfs_inode: inode,
    pub i_openers: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spufs_tree_descr {
    pub name: *const c_char,
    pub ops: *const file_operations,
    pub mode: umode_t,
    pub size: usize,
}

// system call implementation
extern "C" {
    pub fn spufs_run_spu(ctx: *mut spu_context, npc: *mut u32, status: *mut u32) -> c_long;
}
// ELF coredump callbacks for writing SPU ELF notes
extern "C" {
    pub fn spufs_coredump_extra_notes_size() -> c_int;
}
extern "C" {
    pub fn spufs_coredump_extra_notes_write(cprm: *mut coredump_params) -> c_int;
}
// gang management
extern "C" {
    pub fn put_spu_gang(gang: *mut spu_gang) -> c_int;
}
extern "C" {
    pub fn spu_gang_remove_ctx(gang: *mut spu_gang, ctx: *mut spu_context);
}
extern "C" {
    pub fn spu_gang_add_ctx(gang: *mut spu_gang, ctx: *mut spu_context);
}
// fault handling
extern "C" {
    pub fn spufs_handle_class1(ctx: *mut spu_context) -> c_int;
}
extern "C" {
    pub fn spufs_handle_class0(ctx: *mut spu_context) -> c_int;
}
// affinity
// context management
extern "C" {
    pub fn mutex_lock_interruptible(_arg: &ctx->state_mutex) -> return;
}
extern "C" {
    pub fn alloc_spu_context(gang: *mut spu_gang) -> *mut spu_context;
}
extern "C" {
    pub fn destroy_spu_context(kref: *mut kref);
}
extern "C" {
    pub fn get_spu_context(ctx: *mut spu_context) -> *mut spu_context;
}
extern "C" {
    pub fn put_spu_context(ctx: *mut spu_context) -> c_int;
}
extern "C" {
    pub fn spu_unmap_mappings(ctx: *mut spu_context);
}
extern "C" {
    pub fn spu_forget(ctx: *mut spu_context);
}
extern "C" {
    pub fn spu_acquire_saved(ctx: *mut spu_context) -> int __must_check;
}
extern "C" {
    pub fn spu_release_saved(ctx: *mut spu_context);
}
extern "C" {
    pub fn spu_stopped(ctx: *mut spu_context, stat: *mut *mut u32) -> c_int;
}
extern "C" {
    pub fn spu_del_from_rq(ctx: *mut spu_context);
}
extern "C" {
    pub fn spu_activate(ctx: *mut spu_context, flags: c_ulong) -> c_int;
}
extern "C" {
    pub fn spu_deactivate(ctx: *mut spu_context);
}
extern "C" {
    pub fn spu_yield(ctx: *mut spu_context);
}
extern "C" {
    pub fn spu_set_timeslice(ctx: *mut spu_context);
}
extern "C" {
    pub fn spu_update_sched_info(ctx: *mut spu_context);
}
extern "C" {
    pub fn __spu_update_sched_info(ctx: *mut spu_context);
}
extern "C" {
    pub fn spu_sched_init() -> int __init;
}
extern "C" {
    pub fn spu_sched_exit();
}
//
// spufs_wait
// Same as wait_event_interruptible(), except that here
// we need to call spu_release(ctx) before sleeping, and
// then spu_acquire(ctx) when awoken.
//
// Returns with state_mutex re-acquired when successful or
// with -ERESTARTSYS and the state_mutex dropped when interrupted.
//

extern "C" {
    pub fn spu_wbox_write(ctx: *mut spu_context, data: u32) -> usize;
}
extern "C" {
    pub fn spu_ibox_read(ctx: *mut spu_context, data: *mut u32) -> usize;
}
// irq callback funcs.
extern "C" {
    pub fn spufs_ibox_callback(spu: *mut spu);
}
extern "C" {
    pub fn spufs_wbox_callback(spu: *mut spu);
}
extern "C" {
    pub fn spufs_stop_callback(spu: *mut spu, irq: c_int);
}
extern "C" {
    pub fn spufs_mfc_callback(spu: *mut spu);
}
extern "C" {
    pub fn spufs_dma_callback(spu: *mut spu, type: c_int);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spufs_coredump_reader {
    pub name: *mut c_char,
    pub cprm): *mut *mut *mut ssize_t (dump)(struct spu_context ctx, struct coredump_params,
    pub ctx): *mut *mut u64 (get)(struct spu_context,
    pub size: usize,
}

extern "C" {
    pub fn spu_init_csa(csa: *mut spu_state) -> c_int;
}
extern "C" {
    pub fn spu_fini_csa(csa: *mut spu_state);
}
extern "C" {
    pub fn spu_save(prev: *mut spu_state, spu: *mut spu) -> c_int;
}
extern "C" {
    pub fn spu_restore(new: *mut spu_state, spu: *mut spu) -> c_int;
}
extern "C" {
    pub fn spu_alloc_lscsa(csa: *mut spu_state) -> c_int;
}
extern "C" {
    pub fn spu_free_lscsa(csa: *mut spu_state);
}
