//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/v3d/v3d_drv.h
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright (C) 2015-2018 Broadcom

pub const V3D_MMU_PAGE_SHIFT: c_int = 12;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v3d_stats {
    pub refcount: kref,
    pub start_ns: u64,
    pub enabled_ns: u64,
    pub jobs_completed: u64,
//
// This seqcount is used to protect the access to the GPU stats
// variables. It must be used as, while we are reading the stats,
// IRQs can happen and the stats can be updated.
//
// However, we use the raw seqcount helpers to interact with this lock
// to avoid false positives from lockdep, which is unable to detect that
// our readers are never from irq or softirq context, and that, for CPU
// job queues, even the write side never is.
//
    pub lock: seqcount_t,
    pub reset_counter: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v3d_queue_state {
    pub sched: drm_gpu_scheduler,
    pub fence_context: u64,
    pub emit_seqno: u64,
// Stores the GPU stats for this queue in the global context.
    pub stats: *mut v3d_stats,
// Currently active job for this queue
    pub active_job: *mut v3d_job,
    pub queue_lock: spinlock_t,
}

// Performance monitor object
//
// The performance monitor (perfmon) lifetime is controlled by userspace using
// perfmon related ioctls. A perfmon can be attached to a CL or CSD submission
// request, and when it is, HW performance counters will be activated just
// before the job is submitted to the GPU and disabled when the job is done.
// This way, only events related to a specific submission will be counted.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v3d_perfmon {
// Tracks the number of users of the perfmon, when this counter reaches
// zero the perfmon is destroyed.
//
    pub refcnt: refcount_t,
// Number of counters activated in this perfmon instance
// (should be less than DRM_V3D_MAX_PERF_COUNTERS).
//
    pub ncounters: u8,
// Events counted by the HW perf counters.
    pub counters: [u8; DRM_V3D_MAX_PERF_COUNTERS],
// Storage for counter values. Counters are incremented by the
// HW perf counter values every time the perfmon is attached
// to a GPU job.  This way, perfmon users don't have to
// retrieve the results after each job if they want to track
// events covering several submissions.  Note that counter
// values can't be reset, but you can fake a reset by
// destroying the perfmon and creating a new one.
//
    pub __counted_by(ncounters): u64 values[],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v3d_gen {
    V3D_GEN_33 = 33,
    V3D_GEN_41 = 41,
    V3D_GEN_42 = 42,
    V3D_GEN_71 = 71,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v3d_irq {
    V3D_CORE_IRQ,
    V3D_HUB_IRQ,
    V3D_MAX_IRQS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v3d_dev {
    pub drm: drm_device,
// Short representation (e.g. 33, 41) of the V3D tech version
    pub ver: v3d_gen,
// Short representation (e.g. 5, 6) of the V3D tech revision
    pub rev: c_int,
    pub single_irq_line: bool,
    pub irq: [c_int; V3D_MAX_IRQS],
    pub perfmon_info: v3d_perfmon_info,
    pub hub_regs: *mut void __iomem,
    pub core_regs: [*mut void __iomem; 3],
    pub bridge_regs: *mut void __iomem,
    pub gca_regs: *mut void __iomem,
    pub sms_regs: *mut void __iomem,
    pub clk: *mut clk,
    pub reset: *mut reset_control,
// Virtual and DMA addresses of the single shared page table.
    pub pt: *mut volatile u32,
    pub pt_paddr: dma_addr_t,
// Virtual and DMA addresses of the MMU's scratch page.  When
// a read or write is invalid in the MMU, it will be
// redirected here.
//
    pub mmu_scratch: *mut c_void,
    pub mmu_scratch_paddr: dma_addr_t,
// virtual address bits from V3D to the MMU.
    pub va_width: c_int,
// Number of V3D cores.
    pub cores: u32,
// Allocator managing the address space.  All units are in
// number of pages.
//
    pub mm: drm_mm,
    pub mm_lock: spinlock_t,
    pub overflow_mem_work: work_struct,
    pub queue: [v3d_queue_state; V3D_MAX_QUEUES],
//
// Tracks the performance monitor state and consistency.
//
// When a non-global perfmon is attached to a job, the scheduler must
// not run any other job on the HW concurrently (otherwise, the
// counters would be polluted by unrelated work).
//
// Protects @active.
    pub lock: spinlock_t,
// Perfmon currently programmed in HW (or NULL if none).
    pub active: *mut v3d_perfmon,
// Finished fence of the most recently submitted job that
// opened a serialization window (i.e. a job with a non-global
// perfmon attached).
//
    pub fence: *mut dma_fence,
// Finished fence of the most recently submitted job on each HW
// queue. Used so that a new perfmon-carrying job can depend on
// every job currently in-flight across all queues.
//
    pub last_hw_fence: [*mut dma_fence; V3D_MAX_QUEUES],
    pub perfmon_state: },
// Protects bo_stats
    pub bo_lock: mutex,
// Lock taken when resetting the GPU, to keep multiple
// processes from trying to park the scheduler threads and
// reset at once.
//
    pub reset_lock: mutex,
// Ordered workqueue shared by every queue's scheduler timeout work.
// V3D reset is global to all queues, so the timeout handlers must not
// run concurrently.
//
    pub reset_wq: *mut workqueue_struct,
// Lock taken when creating and pushing the GPU scheduler
// jobs, to keep the sched-fence seqnos in order.
//
    pub sched_lock: mutex,
// Lock taken during a cache clean and when initiating an L2
// flush, to keep L2 flushes from interfering with the
// synchronous L2 cleans.
//
    pub cache_clean_lock: mutex,
    pub num_allocated: u32,
    pub pages_allocated: u32,
    pub bo_stats: },
// To support a performance analysis tool in user space, we require
// a single, globally configured performance monitor (perfmon) for
// all jobs.
//
    pub global_perfmon: *mut v3d_perfmon,
// Global reset counter incremented on each GPU reset.
    pub reset_counter: core::sync::atomic::AtomicI32,
}

extern "C" {
    pub fn container_of(_arg: dev, v3d_dev: struct, _arg: drm) -> return;
}

// The per-fd struct, which tracks the MMU mappings.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v3d_file_priv {
    pub v3d: *mut v3d_dev,
    pub perfmons: xarray,
    pub sched_entity: [drm_sched_entity; V3D_MAX_QUEUES],
// Stores the GPU stats for a specific queue for this fd.
    pub stats: [*mut v3d_stats; V3D_MAX_QUEUES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v3d_bo {
    pub base: drm_gem_shmem_object,
    pub node: drm_mm_node,
// List entry for the BO's position in
// v3d_render_job->unref_list
//
    pub unref_head: list_head,
    pub vaddr: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v3d_fence {
    pub base: dma_fence,
    pub dev: *mut drm_device,
// v3d seqno for signaled() test
    pub seqno: u64,
    pub queue: v3d_queue,
}

pub const V3D_SMS_IDLE: c_uint = 0x0;
pub const V3D_SMS_ISOLATING_FOR_RESET: c_uint = 0xa;
pub const V3D_SMS_RESETTING: c_uint = 0xb;
pub const V3D_SMS_ISOLATING_FOR_POWER_OFF: c_uint = 0xc;
pub const V3D_SMS_POWER_OFF_STATE: c_uint = 0xd;

pub const V3D_MAX_JOBS_PER_SUBMISSION: c_int = 3;
// Per-ioctl submission context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v3d_submit {
    pub v3d: *mut v3d_dev,
    pub file_priv: *mut drm_file,
// DRM exec context for this submission.
    pub exec: drm_exec,
// Ordered array of jobs forming the submission chain. Jobs are
// appended via v3d_submit_add_job(), then chained and pushed to
// the scheduler by v3d_submit_jobs().
//
    pub jobs: [*mut v3d_job; V3D_MAX_JOBS_PER_SUBMISSION],
// Number of jobs currently in @jobs.
    pub job_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v3d_job {
    pub base: drm_sched_job,
    pub refcount: kref,
    pub v3d: *mut v3d_dev,
// The queue that the job was submitted on.
    pub queue: v3d_queue,
// This is the array of BOs that were looked up at the start
// of submission.
//
    pub bo: *mut drm_gem_object,
    pub bo_count: u32,
// v3d fence to be signaled by IRQ handler when the job is complete.
    pub irq_fence: *mut dma_fence,
// scheduler fence for when the job is considered complete and
// the BO reservations can be released.
//
    pub done_fence: *mut dma_fence,
// Pointer to a performance monitor object if the user requested it,
// NULL otherwise.
//
    pub perfmon: *mut v3d_perfmon,
// File descriptor of the process that submitted the job that could be used
// to collect per-process information about the GPU.
//
    pub file_priv: *mut v3d_file_priv,
// Pointers to this job's per-fd and global queue stats.
    pub client_stats: *mut v3d_stats,
    pub global_stats: *mut v3d_stats,
// Callback for the freeing of the job on refcount going to 0.
    pub ref): *mut *mut void (free)(struct kref,
    pub has_pm_ref: bool,
// Whether the job needs implicit dependencies, i.e. must wait for
// other contexts still writing its BOs.
//
    pub has_implicit_dep: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v3d_bin_job {
    pub base: v3d_job,
// GPU virtual addresses of the start/end of the CL job.
    pub end: u32 start,,
    pub timedout_ctra: u32 timedout_ctca,,
// Corresponding render job, for attaching our overflow memory.
    pub render: *mut v3d_render_job,
// Submitted tile memory allocation start/size, tile state.
    pub qts: u32 qma, qms,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v3d_render_job {
    pub base: v3d_job,
// GPU virtual addresses of the start/end of the CL job.
    pub end: u32 start,,
    pub timedout_ctra: u32 timedout_ctca,,
// List of overflow BOs used in the job that need to be
// released once the job is complete.
//
    pub unref_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v3d_tfu_job {
    pub base: v3d_job,
    pub args: drm_v3d_submit_tfu,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v3d_csd_job {
    pub base: v3d_job,
    pub timedout_batches: u32,
    pub args: drm_v3d_submit_csd,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v3d_cpu_job_type {
    V3D_CPU_JOB_TYPE_INDIRECT_CSD = 1,
    V3D_CPU_JOB_TYPE_TIMESTAMP_QUERY,
    V3D_CPU_JOB_TYPE_RESET_TIMESTAMP_QUERY,
    V3D_CPU_JOB_TYPE_COPY_TIMESTAMP_QUERY,
    V3D_CPU_JOB_TYPE_RESET_PERFORMANCE_QUERY,
    V3D_CPU_JOB_TYPE_COPY_PERFORMANCE_QUERY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v3d_timestamp_query {
// Offset of this query in the timestamp BO for its value.
    pub offset: u32,
// Syncobj that indicates the timestamp availability
    pub syncobj: *mut drm_syncobj,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v3d_performance_query {
// Performance monitor IDs for this query
    pub kperfmon_ids: *mut u32,
// Syncobj that indicates the query availability
    pub syncobj: *mut drm_syncobj,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v3d_indirect_csd_info {
// Indirect CSD
    pub job: *mut v3d_csd_job,
// Indirect CSD args, stashed by the extension parser and later used
// to create the CSD job from them.
//
    pub args: drm_v3d_submit_csd,
// Offset within the BO where the workgroup counts are stored
    pub offset: u32,
// Workgroups size
    pub wg_size: u32,
// Indices of the uniforms with the workgroup dispatch counts
// in the uniform stream.
//
    pub wg_uniform_offsets: [u32; 3],
// Indirect BO
    pub indirect: *mut drm_gem_object,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v3d_timestamp_query_info {
    pub queries: *mut v3d_timestamp_query,
    pub count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v3d_performance_query_info {
    pub queries: *mut v3d_performance_query,
// Number of performance queries
    pub count: u32,
// Number of performance monitors related to that query pool
    pub nperfmons: u32,
// Number of performance counters related to that query pool
    pub ncounters: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v3d_copy_query_results_info {
// Define if should write to buffer using 64 or 32 bits
    pub do_64bit: bool,
// Define if it can write to buffer even if the query is not available
    pub do_partial: bool,
// Define if it should write availability bit to buffer
    pub availability_bit: bool,
// Offset of the copy buffer in the BO
    pub offset: u32,
// Stride of the copy buffer in the BO
    pub stride: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v3d_cpu_job {
    pub base: v3d_job,
    pub job_type: v3d_cpu_job_type,
    pub indirect_csd: v3d_indirect_csd_info,
    pub timestamp_query: v3d_timestamp_query_info,
    pub copy: v3d_copy_query_results_info,
    pub performance_query: v3d_performance_query_info,
}

extern "C" {
    pub fn void(: *mut *mut v3d_cpu_job_fn)(struct v3d_cpu_job) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v3d_submit_outsync {
    pub syncobj: *mut drm_syncobj,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v3d_submit_ext {
    pub flags: u32,
    pub wait_stage: u32,
    pub in_sync_count: u32,
    pub in_syncs: u64,
    pub out_sync_count: u32,
    pub out_syncs: *mut v3d_submit_outsync,
}

//
// __wait_for - magic wait macro
//
// Macro to help avoid open coding check/wait/timeout patterns. Note that it's
// important that we check the condition again after having timed out, since the
// timeout could be due to preemption or similar and we've never had a chance to
// check the condition before the timeout.
//

// Guarantee COND check prior to timeout */		\

// nsecs_to_jiffies64() does not guard against overflow
extern "C" {
    pub fn min_t(_arg: u64, _arg: MAX_JIFFY_OFFSET, 1: nsecs_to_jiffies64(n) +) -> return;
}
// v3d_bo.c
extern "C" {
    pub fn v3d_free_object(gem_obj: *mut drm_gem_object);
}
extern "C" {
    pub fn v3d_get_bo_vaddr(bo: *mut v3d_bo);
}
extern "C" {
    pub fn v3d_put_bo_vaddr(bo: *mut v3d_bo);
}
// v3d_debugfs.c
extern "C" {
    pub fn v3d_debugfs_init(minor: *mut drm_minor);
}
// v3d_drv.c
// v3d_fence.c
// v3d_gem.c
extern "C" {
    pub fn v3d_init_hw_state(v3d: *mut v3d_dev);
}
extern "C" {
    pub fn v3d_gem_init(dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn v3d_gem_destroy(dev: *mut drm_device);
}
extern "C" {
    pub fn v3d_idle_axi(v3d: *mut v3d_dev, core: c_int);
}
extern "C" {
    pub fn v3d_idle_gca(v3d: *mut v3d_dev);
}
extern "C" {
    pub fn v3d_reset_sms(v3d: *mut v3d_dev);
}
extern "C" {
    pub fn v3d_reset(v3d: *mut v3d_dev);
}
extern "C" {
    pub fn v3d_invalidate_caches(v3d: *mut v3d_dev);
}
extern "C" {
    pub fn v3d_clean_caches(v3d: *mut v3d_dev);
}
// v3d_submit.c
extern "C" {
    pub fn v3d_job_cleanup(job: *mut v3d_job);
}
extern "C" {
    pub fn v3d_job_put(job: *mut v3d_job);
}
// v3d_irq.c
extern "C" {
    pub fn v3d_irq_init(v3d: *mut v3d_dev) -> c_int;
}
extern "C" {
    pub fn v3d_irq_enable(v3d: *mut v3d_dev);
}
extern "C" {
    pub fn v3d_irq_disable(v3d: *mut v3d_dev);
}
extern "C" {
    pub fn v3d_irq_reset(v3d: *mut v3d_dev);
}
// v3d_mmu.c
extern "C" {
    pub fn v3d_mmu_flush_all(v3d: *mut v3d_dev) -> c_int;
}
extern "C" {
    pub fn v3d_mmu_set_page_table(v3d: *mut v3d_dev) -> c_int;
}
extern "C" {
    pub fn v3d_mmu_insert_ptes(bo: *mut v3d_bo);
}
extern "C" {
    pub fn v3d_mmu_remove_ptes(bo: *mut v3d_bo);
}
// v3d_power.c
extern "C" {
    pub fn v3d_power_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn v3d_power_resume(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pm_runtime_resume_and_get(_arg: v3d->drm.dev) -> return;
}
extern "C" {
    pub fn pm_runtime_put_autosuspend(_arg: v3d->drm.dev) -> return;
}
// v3d_sched.c
extern "C" {
    pub fn v3d_stats_release(refcount: *mut kref);
}
extern "C" {
    pub fn v3d_job_update_stats(job: *mut v3d_job);
}
extern "C" {
    pub fn v3d_sched_init(v3d: *mut v3d_dev) -> c_int;
}
extern "C" {
    pub fn v3d_sched_fini(v3d: *mut v3d_dev);
}
// v3d_perfmon.c
extern "C" {
    pub fn v3d_perfmon_init(v3d: *mut v3d_dev);
}
extern "C" {
    pub fn v3d_perfmon_get(perfmon: *mut v3d_perfmon);
}
extern "C" {
    pub fn v3d_perfmon_put(perfmon: *mut v3d_perfmon);
}
extern "C" {
    pub fn v3d_perfmon_start(v3d: *mut v3d_dev, perfmon: *mut v3d_perfmon);
}
extern "C" {
    pub fn v3d_perfmon_suspend(v3d: *mut v3d_dev);
}
extern "C" {
    pub fn v3d_perfmon_resume(v3d: *mut v3d_dev);
}
extern "C" {
    pub fn v3d_perfmon_open_file(v3d_priv: *mut v3d_file_priv);
}
extern "C" {
    pub fn v3d_perfmon_close_file(v3d_priv: *mut v3d_file_priv);
}
// v3d_sysfs.c
extern "C" {
    pub fn v3d_sysfs_init(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn v3d_sysfs_destroy(dev: *mut device);
}
