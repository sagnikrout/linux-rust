//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/msm_gpu.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2013 Red Hat
// Author: Rob Clark <robdclark@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_gpu_config {
    pub ioname: *const c_char,
    pub nr_rings: c_uint,
}

// So far, with hardware that I've seen to date, we can have:
// + zero, one, or two z180 2d cores
// + a3xx or a2xx 3d core, which share a common CP (the firmware
// for the CP seems to implement some different PM4 packet types
// but the basics of cmdstream submission are the same)
//
// Which means that the eventual complete "class" hierarchy, once
// support for all past and present hw is in place, becomes:
// + msm_gpu
// + adreno_gpu
// + a3xx_gpu
// + a2xx_gpu
// + z180_gpu
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_gpu_funcs {
    pub len): *mut *mut uint32_t param, uint64_t value, uint32_t,
    pub len): uint32_t param, uint64_t value, uint32_t,
    pub gpu): *mut *mut int (hw_init)(struct msm_gpu,
//
// @ucode_load: Optional hook to upload fw to GEM objs
//
    pub gpu): *mut *mut int (ucode_load)(struct msm_gpu,
    pub gpu): *mut *mut int (pm_suspend)(struct msm_gpu,
    pub gpu): *mut *mut int (pm_resume)(struct msm_gpu,
    pub submit): *mut *mut *mut void (submit)(struct msm_gpu gpu, struct msm_gem_submit,
    pub ring): *mut *mut *mut void (flush)(struct msm_gpu gpu, struct msm_ringbuffer,
    pub irq): *mut *mut irqreturn_t (irq)(struct msm_gpu,
    pub gpu): *mut *mut *mut msm_ringbuffer (active_ring)(msm_gpu,
    pub gpu): *mut *mut void (recover)(struct msm_gpu,
    pub gpu): *mut *mut void (destroy)(struct msm_gpu,

// show GPU status in debugfs:
    pub p): *mut drm_printer,
// for generation specific debugfs:
    pub minor): *mut *mut *mut void (debugfs_init)(struct msm_gpu gpu, struct drm_minor,

// note: gpu_busy() can assume that we have been pm_resumed
    pub out_sample_rate): *mut *mut *mut u64 (gpu_busy)(struct msm_gpu gpu, unsigned long,
    pub gpu): *mut *mut *mut msm_gpu_state (gpu_state_get)(msm_gpu,
    pub state): *mut *mut int (gpu_state_put)(struct msm_gpu_state,
    pub gpu): *mut *mut unsigned long (gpu_get_freq)(struct msm_gpu,
// note: gpu_set_freq() can assume that we have been pm_resumed
    pub suspended): bool,
    pub pdev): *mut *mut *mut *mut drm_gpuvm (create_vm)(msm_gpu gpu, platform_device,
    pub kernel_managed): *mut *mut *mut *mut drm_gpuvm (create_private_vm)(msm_gpu gpu, bool,
    pub ring): *mut *mut *mut uint32_t (get_rptr)(struct msm_gpu gpu, struct msm_ringbuffer,
//
// progress: Has the GPU made progress?
//
// Return true if GPU position in cmdstream has advanced (or changed)
// since the last call.  To avoid false negatives, this should account
// for cmdstream that is buffered in this FIFO upstream of the CP fw.
//
    pub ring): *mut *mut *mut bool (progress)(struct msm_gpu gpu, struct msm_ringbuffer,
    pub force_on): *mut *mut *mut void (sysprof_setup)(struct msm_gpu gpu, bool,
// Configure perfcntr SELect regs:
    pub stream): *const msm_perfcntr_stream,
// Flush perfcntrs before reading (optional):
    pub gpu): *mut *mut void (perfcntr_flush)(struct msm_gpu,
}

// Additional state for iommu faults:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_gpu_fault_info {
    pub ttbr0: u64,
    pub iova: c_ulong,
    pub flags: c_int,
    pub type: *const c_char,
    pub block: *const c_char,
// Information about what we think/expect is the current SMMU state,
// for example expected_ttbr0 should match smmu_info.ttbr0 which
// was read back from SMMU registers.
//
    pub pgtbl_ttbr0: phys_addr_t,
    pub ptes: [u64; 4],
    pub asid: c_int,
}

//
// struct msm_gpu_devfreq - devfreq related state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_gpu_devfreq {
// @devfreq: devfreq instance
    pub devfreq: *mut devfreq,
// @lock: lock for "suspended", "busy_cycles", and "time"
    pub lock: mutex,
//
// @idle_freq:
// Shadow frequency used while the GPU is idle.  From the PoV of
// the devfreq governor, we are continuing to sample busyness and
// adjust frequency while the GPU is idle, but we use this shadow
// value as the GPU is actually clamped to minimum frequency while
// it is inactive.
//
    pub idle_freq: c_ulong,
//
// @boost_freq:
// A PM QoS constraint to boost min freq for a period of time
// until the boost expires.
//
    pub boost_freq: dev_pm_qos_request,
//
// @busy_cycles: Last busy counter value, for calculating elapsed busy
// cycles since last sampling period.
//
    pub busy_cycles: u64,
// @time: Time of last sampling period.
    pub time: ktime_t,
// @idle_time: Time of last transition to idle.
    pub idle_time: ktime_t,
//
// @idle_work:
// Used to delay clamping to idle freq on active->idle transition.
//
    pub idle_work: msm_hrtimer_work,
//
// @boost_work:
// Used to reset the boost_constraint after the boost period has
// elapsed
//
    pub boost_work: msm_hrtimer_work,
// @suspended: tracks if we're suspended
    pub suspended: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_gpu {
    pub name: *const c_char,
    pub dev: *mut drm_device,
    pub pdev: *mut platform_device,
    pub funcs: *const msm_gpu_funcs,
    pub adreno_smmu: adreno_smmu_priv,
    pub rb: [*mut msm_ringbuffer; MSM_GPU_MAX_RINGS],
    pub nr_rings: c_int,
//
// sysprof_active:
//
// The count of contexts that have enabled system profiling.
//
    pub sysprof_active: refcount_t,
//
// lock:
//
// General lock for serializing all the gpu things.
//
// TODO move to per-ring locking where feasible (ie. submit/retire
// path, etc)
//
    pub lock: mutex,
//
// active_submits:
//
// The number of submitted but not yet retired submits, used to
// determine transitions between active and idle.
//
// Protected by active_lock
//
    pub active_submits: c_int,
// lock: protects active_submits and idle/active transitions
    pub active_lock: mutex,
// does gpu need hw_init?
    pub needs_hw_init: bool,
//
// global_faults: number of GPU hangs not attributed to a particular
// address space
//
    pub global_faults: c_int,
    pub mmio: *mut void __iomem,
    pub irq: c_int,
    pub vm: *mut drm_gpuvm,
// Power Control:
    pub gpu_cx: *mut *mut regulator gpu_reg,,
    pub grp_clks: *mut clk_bulk_data,
    pub nr_clocks: c_int,
    pub rbbmtimer_clk: *mut *mut *mut clk ebi1_clk, core_clk,,
    pub fast_rate: u32,
// Hang and Inactivity Detection:
//

pub const DRM_MSM_HANGCHECK_PROGRESS_RETRIES: c_int = 3;
    pub hangcheck_timer: timer_list,
// work for handling GPU recovery:
    pub recover_work: kthread_work,
// retire_event: notified when submits are retired:
    pub retire_event: wait_queue_head_t,
// work for handling active-list retiring:
    pub retire_work: kthread_work,
// worker for retire/recover:
    pub worker: *mut kthread_worker,
    pub memptrs_bo: *mut drm_gem_object,
    pub devfreq: msm_gpu_devfreq,
    pub suspend_count: u32,
    pub crashstate: *mut msm_gpu_state,
// True if the hardware supports expanded apriv (a650 and newer)
    pub hw_apriv: bool,
//
// @allow_relocs: allow relocs in SUBMIT ioctl
//
// Mesa won't use relocs for driver version 1.4.0 and later.  This
// switch-over happened early enough in mesa a6xx bringup that we
// can disallow relocs for a6xx and newer.
//
    pub allow_relocs: bool,
    pub cooling: *mut thermal_cooling_device,
    pub perfcntr_groups: *const msm_perfcntr_group,
    pub num_perfcntr_groups: unsigned,
    pub perfcntrs: *mut msm_perfcntr_state,
// @perfcntr_lock: protects perfcntr related state
    pub perfcntr_lock: mutex,
}

extern "C" {
    pub fn container_of(_arg: adreno_smmu, msm_gpu: struct, _arg: adreno_smmu) -> return;
}
// It turns out that all targets use the same ringbuffer size

pub const MSM_GPU_RINGBUFFER_BLKSIZE: c_int = 32;

//
// struct msm_perfcntr_group_state - Tracking for the currently allocated counter state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_perfcntr_group_state {
//
// @allocated_counters:
//
// allocated counters for global counter collection.  The
// corresponding counters are allocated from highest to
// lowest, to minimize chance of conflict with old userspace
// allocating from lowest to highest.
//
    pub allocated_counters: unsigned,
//
// @countables:
//
// The corresponding SELect reg values for the allocated counters
//
    pub countables: [u32; ],
}

//
// struct msm_perfcntr_state - overall global perfcntr state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_perfcntr_state {
// @stream: current global counter stream if active
    pub stream: *mut msm_perfcntr_stream,
// @sel_seqno: counter for sel_fence
    pub sel_seqno: u32,
//
// @groups: Global perfcntr stream group state.
//
// Conceptually this is part of msm_perfcntr_stream state, but is
// statically pre-allocated when the gpu is initialized to simplify
// error path cleanup in PERFCNTR_CONFIG ioctl.  (__free(kfree)
// doesn't really help with variable length arrays of allocated
// pointers.)
//
    pub groups: [*mut msm_perfcntr_group_state; ],
}

//
// The number of priority levels provided by drm gpu scheduler.  The
// DRM_SCHED_PRIORITY_KERNEL priority level is treated specially in some
// cases, so we don't use it (no need for kernel generated jobs).
//

//
// struct msm_context - per-drm_file context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_context {
// @ctxlock: synchronizes access to submitqueues list, etc
    pub ctxlock: rw_semaphore,
// @submitqueues: list of &msm_gpu_submitqueue created by userspace
    pub submitqueues: list_head,
//
// @queueid:
// Counter incremented each time a submitqueue is created, used to
// assign &msm_gpu_submitqueue.id
//
    pub queueid: c_int,
//
// @closed: The device file associated with this context has been closed.
// Once the device is closed, any submits that have not been written
// to the ring buffer are no-op'd.
//
    pub closed: bool,
//
// @userspace_managed_vm:
// Has userspace opted-in to userspace managed VM (ie. VM_BIND) via
// MSM_PARAM_EN_VM_BIND?
//
    pub userspace_managed_vm: bool,
//
// @vm:
// The per-process GPU address-space.  Do not access directly, use
// msm_context_vm().
//
    pub vm: *mut drm_gpuvm,
// @ref: the reference count
    pub ref: kref,
//
// @seqno:
// A unique per-process sequence number.  Used to detect context
// switches, without relying on keeping a, potentially dangling,
// pointer to the previous context.
//
    pub seqno: c_int,
//
// @sysprof:
// The value of MSM_PARAM_SYSPROF set by userspace.  This is
// intended to be used by system profiling tools like Mesa's
// pps-producer (perfetto), and restricted to CAP_SYS_ADMIN.
//
// Setting a value of 1 will preserve performance counters across
// context switches.  Setting a value of 2 will in addition
// suppress suspend.  (Performance counters lose state across
// power collapse, which is undesirable for profiling in some
// cases.)
//
// The value automatically reverts to zero when the drm device
// file is closed.
//
    pub sysprof: c_int,
//
// @comm: Overridden task comm, see MSM_PARAM_COMM
//
// Accessed under msm_gpu::lock
//
    pub comm: *mut c_char,
//
// @cmdline: Overridden task cmdline, see MSM_PARAM_CMDLINE
//
// Accessed under msm_gpu::lock
//
    pub cmdline: *mut c_char,
//
// @elapsed_ns:
// The total (cumulative) elapsed time GPU was busy with rendering
// from this context in ns.
//
    pub elapsed_ns: u64,
//
// @cycles:
// The total (cumulative) GPU cycles elapsed attributed to this
// context.
//
    pub cycles: u64,
//
// @entities:
// Table of per-priority-level sched entities used by submitqueues
// associated with this &drm_file.  Because some userspace apps
// make assumptions about rendering from multiple gl contexts
// (of the same priority) within the process happening in FIFO
// order without requiring any fencing beyond MakeCurrent(), we
// create at most one &drm_sched_entity per-process per-priority-
// level.
//
    pub MSM_GPU_MAX_RINGS]: *mut *mut *mut drm_sched_entity entities[NR_SCHED_PRIORITIES,
//
// @ctx_mem:
// Total amount of memory of GEM buffers with handles attached for
// this context.
//
    pub ctx_mem: core::sync::atomic::AtomicI64,
//
// @perfcntrs: Per-context reserved perfcntrs state
//
    pub perfctx: *mut msm_perfcntr_context_state,
}

//
// msm_context_is_vmbind() - has userspace opted in to VM_BIND?
//
// @ctx: the drm_file context
//
// See MSM_PARAM_EN_VM_BIND.  If userspace is managing the VM, it can
// do sparse binding including having multiple, potentially partial,
// mappings in the VM.  Therefore certain legacy uabi (ie. GET_IOVA,
// SET_IOVA) are rejected because they don't have a sensible meaning.
//
// Returns: %true if userspace is managing the VM, %false otherwise.
//
// msm_gpu_convert_priority - Map userspace priority to ring # and sched priority
//
// @gpu:        the gpu instance
// @prio:       the userspace priority level
// @ring_nr:    [out] the ringbuffer the userspace priority maps to
// @sched_prio: [out] the gpu scheduler priority level which the userspace
// priority maps to
//
// With drm/scheduler providing it's own level of prioritization, our total
// number of available priority levels is (nr_rings * NR_SCHED_PRIORITIES).
// Each ring is associated with it's own scheduler instance.  However, our
// UABI is that lower numerical values are higher priority.  So mapping the
// single userspace priority level into ring_nr and sched_prio takes some
// care.  The userspace provided priority (when a submitqueue is created)
// is mapped to ring nr and scheduler priority as such:
//
// ring_nr    = userspace_prio / NR_SCHED_PRIORITIES
// sched_prio = NR_SCHED_PRIORITIES -
// (userspace_prio % NR_SCHED_PRIORITIES) - 1
//
// This allows generations without preemption (nr_rings==1) to have some
// amount of prioritization, and provides more priority levels for gens
// that do have preemption.
//
// Returns: %0 on success, %-errno on error.
//
// invert sched priority to map to higher-numeric-is-higher-
// priority convention
//
// ring_nr = rn;
// sched_prio = sp;
//
// struct msm_gpu_submitqueue - Userspace created context.
//
// A submitqueue is associated with a gl context or vk queue (or equiv)
// in userspace.
//
// @id:        userspace id for the submitqueue, unique within the drm_file
// @flags:     userspace flags for the submitqueue, specified at creation
// (currently unusued)
// @ring_nr:   the ringbuffer used by this submitqueue, which is determined
// by the submitqueue's priority
// @faults:    the number of GPU hangs associated with this submitqueue
// @last_fence: the sequence number of the last allocated fence (for error
// checking)
// @ctx:       the per-drm_file context associated with the submitqueue (ie.
// which set of pgtables do submits jobs associated with the
// submitqueue use)
// @node:      node in the context's list of submitqueues
// @fence_idr: maps fence-id to dma_fence for userspace visible fence
// seqno, protected by submitqueue lock
// @idr_lock:  for serializing access to fence_idr
// @lock:      submitqueue lock for serializing submits on a queue
// @ref:       reference count
// @entity:    the submit job-queue
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_gpu_submitqueue {
    pub id: c_int,
    pub flags: u32,
    pub ring_nr: u32,
    pub faults: c_int,
    pub last_fence: u32,
    pub ctx: *mut msm_context,
    pub node: list_head,
    pub fence_idr: idr,
    pub idr_lock: spinlock,
    pub lock: mutex,
    pub ref: kref,
    pub entity: *mut drm_sched_entity,
// @_vm_bind_entity: used for @entity pointer for VM_BIND queues
    pub _vm_bind_entity: [drm_sched_entity; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_gpu_state_bo {
    pub iova: u64,
    pub size: usize,
    pub flags: u32,
    pub data: *mut c_void,
    pub encoded: bool,
    pub name: [c_char; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_gpu_state {
    pub ref: kref,
    pub time: timespec64,
    pub iova: u64,
    pub fence: u32,
    pub seqno: u32,
    pub rptr: u32,
    pub wptr: u32,
    pub data: *mut c_void,
    pub data_size: c_int,
    pub encoded: bool,
    pub ring: [}; MSM_GPU_MAX_RINGS],
    pub nr_registers: c_int,
    pub registers: *mut u32,
    pub rbbm_status: u32,
    pub comm: *mut c_char,
    pub cmd: *mut c_char,
    pub fault_info: msm_gpu_fault_info,
    pub nr_vm_logs: c_int,
    pub vm_logs: *mut msm_gem_vm_log_entry,
    pub nr_bos: c_int,
    pub bos: *mut msm_gpu_state_bo,
}

extern "C" {
    pub fn readl(2): gpu->mmio + (reg <<) -> return;
}
//
// Why not a readq here? Two reasons: 1) many of the LO registers are
// not quad word aligned and 2) the GPU hardware designers have a bit
// of a history of putting registers where they fit, especially in
// spins. The longer a GPU family goes the higher the chance that
// we'll get burned.  We could do a series of validity checks if we
// wanted to, but really is a readq() that much better? Nah.
//
// For some lo/hi registers (like perfcounters), the hi value is latched
// when the lo is read, so make sure to read the lo first to trigger
// that
//
// Why not a writeq here? Read the screed above
extern "C" {
    pub fn msm_gpu_pm_suspend(gpu: *mut msm_gpu) -> c_int;
}
extern "C" {
    pub fn msm_gpu_pm_resume(gpu: *mut msm_gpu) -> c_int;
}
extern "C" {
    pub fn msm_submitqueue_init(drm: *mut drm_device, ctx: *mut msm_context) -> c_int;
}
extern "C" {
    pub fn msm_submitqueue_remove(ctx: *mut msm_context, id: u32) -> c_int;
}
extern "C" {
    pub fn msm_submitqueue_close(ctx: *mut msm_context);
}
extern "C" {
    pub fn msm_submitqueue_destroy(kref: *mut kref);
}
extern "C" {
    pub fn msm_context_set_sysprof(ctx: *mut msm_context, gpu: *mut msm_gpu, sysprof: c_int) -> c_int;
}
extern "C" {
    pub fn __msm_context_destroy(kref: *mut kref);
}
extern "C" {
    pub fn msm_devfreq_init(gpu: *mut msm_gpu);
}
extern "C" {
    pub fn msm_devfreq_cleanup(gpu: *mut msm_gpu);
}
extern "C" {
    pub fn msm_devfreq_resume(gpu: *mut msm_gpu);
}
extern "C" {
    pub fn msm_devfreq_suspend(gpu: *mut msm_gpu);
}
extern "C" {
    pub fn msm_devfreq_boost(gpu: *mut msm_gpu, factor: unsigned);
}
extern "C" {
    pub fn msm_devfreq_active(gpu: *mut msm_gpu);
}
extern "C" {
    pub fn msm_devfreq_idle(gpu: *mut msm_gpu);
}
extern "C" {
    pub fn msm_gpu_hw_init(gpu: *mut msm_gpu) -> c_int;
}
extern "C" {
    pub fn msm_gpu_retire(gpu: *mut msm_gpu);
}
extern "C" {
    pub fn msm_gpu_submit(gpu: *mut msm_gpu, submit: *mut msm_gem_submit);
}
extern "C" {
    pub fn msm_gpu_cleanup(gpu: *mut msm_gpu);
}
extern "C" {
    pub fn adreno_has_gpu(node: *mut device_node) -> bool;
}
extern "C" {
    pub fn adreno_register() -> void __init;
}
extern "C" {
    pub fn adreno_unregister() -> void __exit;
}
extern "C" {
    pub fn msm_gpu_fault_crashstate_capture(gpu: *mut msm_gpu, fault_info: *mut msm_gpu_fault_info);
}
//
// Simple macro to semi-cleanly add the MAP_PRIV flag for targets that can
// support expanded privileges
//

