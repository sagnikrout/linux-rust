//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/intel_engine_types.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2019 Intel Corporation
//

// HW Engine class + instance
pub const RENDER_CLASS: c_int = 0;
pub const VIDEO_DECODE_CLASS: c_int = 1;
pub const VIDEO_ENHANCEMENT_CLASS: c_int = 2;
pub const COPY_ENGINE_CLASS: c_int = 3;
pub const OTHER_CLASS: c_int = 4;
pub const COMPUTE_CLASS: c_int = 5;
pub const MAX_ENGINE_CLASS: c_int = 5;
pub const MAX_ENGINE_INSTANCE: c_int = 8;
pub const I915_MAX_SLICES: c_int = 3;
pub const I915_MAX_SUBSLICES: c_int = 8;
pub const I915_CMD_HASH_ORDER: c_int = 9;
pub type intel_engine_mask_t = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_hw_status_page {
    pub timelines: list_head,
    pub vma: *mut i915_vma,
    pub addr: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_instdone {
    pub instdone: u32,
// The following exist only in the RCS engine
    pub slice_common: u32,
    pub slice_common_extra: [u32; 2],
    pub sampler: [u32; GEN_MAX_GSLICES][I915_MAX_SUBSLICES],
    pub row: [u32; GEN_MAX_GSLICES][I915_MAX_SUBSLICES],
// Added in XeHPG
    pub geom_svg: [u32; GEN_MAX_GSLICES][I915_MAX_SUBSLICES],
}

//
// we use a single page to load ctx workarounds so all of these
// values are referred in terms of dwords
//
// struct i915_wa_ctx_bb:
// offset: specifies batch starting position, also helpful in case
// if we want to have multiple batches at different offsets based on
// some criteria. It is not a requirement at the moment but provides
// an option for future use.
// size: size of the batch in DWORDS
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_ctx_workarounds {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_wa_ctx_bb {
    pub offset: u32,
    pub size: u32,
    pub per_ctx: } indirect_ctx,,
    pub vma: *mut i915_vma,
}

pub const I915_MAX_VCS: c_int = 8;
pub const I915_MAX_VECS: c_int = 4;

pub const I915_MAX_CCS: c_int = 4;
pub const I915_MAX_RCS: c_int = 1;
pub const I915_MAX_BCS: c_int = 9;
//
// Engine IDs definitions.
// Keep instances of the same type engine together.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_engine_id {
    RCS0 = 0,
    BCS0,
    BCS1,
    BCS2,
    BCS3,
    BCS4,
    BCS5,
    BCS6,
    BCS7,
    BCS8,

    VCS0,
    VCS1,
    VCS2,
    VCS3,
    VCS4,
    VCS5,
    VCS6,
    VCS7,

    VECS0,
    VECS1,
    VECS2,
    VECS3,

    CCS0,
    CCS1,
    CCS2,
    CCS3,

    GSC0,
    I915_NUM_ENGINES

}

// A simple estimator for the round-trip latency of an engine
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_preempt_hang {
    pub completion: completion,
    pub count: c_uint,
}

//
// struct intel_engine_execlists - execlist submission queue and port state
//
// The struct intel_engine_execlists represents the combined logical state of
// driver and the hardware state for execlist mode of submission.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_engine_execlists {
//
// @timer: kick the current context if its timeslice expires
//
    pub timer: timer_list,
//
// @preempt: reset the current context if it fails to give way
//
    pub preempt: timer_list,
//
// @preempt_target: active request at the time of the preemption request
//
// We force a preemption to occur if the pending contexts have not
// been promoted to active upon receipt of the CS ack event within
// the timeout. This timeout maybe chosen based on the target,
// using a very short timeout if the context is no longer schedulable.
// That short timeout may not be applicable to other contexts, so
// if a context switch should happen within before the preemption
// timeout, we may shoot early at an innocent context. To prevent this,
// we record which context was active at the time of the preemption
// request and only reset that context upon the timeout.
//
    pub preempt_target: *const i915_request,
//
// @ccid: identifier for contexts submitted to this engine
//
    pub ccid: u32,
//
// @yield: CCID at the time of the last semaphore-wait interrupt.
//
// Instead of leaving a semaphore busy-spinning on an engine, we would
// like to switch to another ready context, i.e. yielding the semaphore
// timeslice.
//
    pub yield: u32,
//
// @error_interrupt: CS Master EIR
//
// The CS generates an interrupt when it detects an error. We capture
// the first error interrupt, record the EIR and schedule the tasklet.
// In the tasklet, we process the pending CS events to ensure we have
// the guilty request, and then reset the engine.
//
// Low 16b are used by HW, with the upper 16b used as the enabling mask.
// Reserve the upper 16b for tracking internal errors.
//
    pub error_interrupt: u32,

//
// @reset_ccid: Active CCID [EXECLISTS_STATUS_HI] at the time of reset
//
    pub reset_ccid: u32,
//
// @submit_reg: gen-specific execlist submission register
// set to the ExecList Submission Port (elsp) register pre-Gen11 and to
// the ExecList Submission Queue Contents register array for Gen11+
//
    pub submit_reg: *mut u32 __iomem,
//
// @ctrl_reg: the enhanced execlists control register, used to load the
// submit queue on the HW and to request preemptions to idle
//
    pub ctrl_reg: *mut u32 __iomem,
pub const EXECLIST_MAX_PORTS: c_int = 2;
//
// @active: the currently known context executing on HW
//
    pub active: *const *const i915_request,
//
// @inflight: the set of contexts submitted and acknowledged by HW
//
// The set of inflight contexts is managed by reading CS events
// from the HW. On a context-switch event (not preemption), we
// know the HW has transitioned from port0 to port1, and we
// advance our inflight/active tracking accordingly.
//
    pub /]: *mut *mut *mut i915_request inflight[EXECLIST_MAX_PORTS + 1 / sentinel,
//
// @pending: the next set of contexts submitted to ELSP
//
// We store the array of contexts that we submit to HW (via ELSP) and
// promote them to the inflight array once HW has signaled the
// preemption or idle-to-active event.
//
    pub 1]: *mut *mut i915_request pending[EXECLIST_MAX_PORTS +,
//
// @port_mask: number of execlist ports - 1
//
    pub port_mask: c_uint,
//
// @virtual: Queue of requests on a virtual engine, sorted by priority.
// Each RB entry is a struct i915_priolist containing a list of requests
// of the same priority.
//
    pub virtual: rb_root_cached,
//
// @csb_write: control register for Context Switch buffer
//
// Note this register may be either mmio or HWSP shadow.
//
    pub csb_write: *mut u32,
//
// @csb_status: status array for Context Switch buffer
//
// Note these register may be either mmio or HWSP shadow.
//
    pub csb_status: *mut u64,
//
// @csb_size: context status buffer FIFO size
//
    pub csb_size: u8,
//
// @csb_head: context status buffer head
//
    pub csb_head: u8,
// private: selftest
    pub preempt_hang;): I915_SELFTEST_DECLARE(struct st_preempt_hang,
}

pub const INTEL_ENGINE_CS_MAX_NAME: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_engine_execlists_stats {
//
// @active: Number of contexts currently scheduled in.
//
    pub active: c_uint,
//
// @lock: Lock protecting the below fields.
//
    pub lock: seqcount_t,
//
// @total: Total time this engine was busy.
//
// Accumulated time not counting the most recent block in cases where
// engine is currently busy (active > 0).
//
    pub total: ktime_t,
//
// @start: Timestamp of the last idle to active transition.
//
// Idle is defined as active == 0, active is active > 0.
//
    pub start: ktime_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_engine_guc_stats {
//
// @running: Active state of the engine when busyness was last sampled.
//
    pub running: bool,
//
// @prev_total: Previous value of total runtime clock cycles.
//
    pub prev_total: u32,
//
// @total_gt_clks: Total gt clock cycles this engine was busy.
//
    pub total_gt_clks: u64,
//
// @start_gt_clk: GT clock time of last idle to active transition.
//
    pub start_gt_clk: u64,
//
// @total: The last value of total returned
//
    pub total: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union intel_engine_tlb_inv_reg {
    pub reg: i915_reg_t,
    pub mcr_reg: i915_mcr_reg_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_engine_tlb_inv {
    pub mcr: bool,
    pub reg: intel_engine_tlb_inv_reg,
    pub request: u32,
    pub done: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_engine_cs {
    pub i915: *mut drm_i915_private,
    pub gt: *mut intel_gt,
    pub uncore: *mut intel_uncore,
    pub name: [c_char; INTEL_ENGINE_CS_MAX_NAME],
    pub id: intel_engine_id,
    pub legacy_idx: intel_engine_id,
    pub guc_id: c_uint,
    pub mask: intel_engine_mask_t,
    pub reset_domain: u32,
//
// @logical_mask: logical mask of engine, reported to user space via
// query IOCTL and used to communicate with the GuC in logical space.
// The logical instance of a physical engine can change based on product
// and fusing.
//
    pub logical_mask: intel_engine_mask_t,
    pub class: u8,
    pub instance: u8,
    pub uabi_class: u16,
    pub uabi_instance: u16,
    pub uabi_capabilities: u32,
    pub context_size: u32,
    pub mmio_base: u32,
    pub tlb_inv: intel_engine_tlb_inv,
//
// Some w/a require forcewake to be held (which prevents RC6) while
// a particular engine is active. If so, we set fw_domain to which
// domains need to be held for the duration of request activity,
// and 0 if none. We try to limit the duration of the hold as much
// as possible.
//
    pub fw_domain: forcewake_domains,
    pub fw_active: c_uint,
    pub context_tag: c_ulong,
//
// The type evolves during initialization, see related comment for
// struct drm_i915_private's uabi_engines member.
//
    pub uabi_llist: llist_node,
    pub uabi_list: list_head,
    pub uabi_node: rb_node,
}

// keep a request in reserve for a [pm] barrier under oom
// mark the bind context's availability status
//
// pinned_contexts_list: List of pinned contexts. This list is only
// assumed to be manipulated during driver load- or unload time and
// does therefore not have any additional protection.
//
// We track the average duration of the idle pulse on parking the
// engine to keep an estimate of the how the fast the engine is
// under ideal conditions.
//
// Keep track of all the seqno used, a trail of breadcrumbs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_engine_pmu {
//
// @enable: Bitmask of enable sample events on this engine.
//
// Bits correspond to sample event types, for instance
// I915_SAMPLE_QUEUED is bit 0 etc.
//
    pub enable: u32,
//
// @enable_count: Reference count for the enabled samplers.
//
// Index number corresponds to @enum drm_i915_pmu_engine_sample.
//
    pub enable_count: [c_uint; I915_ENGINE_SAMPLE_COUNT],
//
// @sample: Counter values for sampling events.
//
// Our internal timer stores the current counters in this field.
//
// Index number corresponds to @enum drm_i915_pmu_engine_sample.
//
    pub sample: [i915_pmu_sample; I915_ENGINE_SAMPLE_COUNT],
    pub pmu: },
    pub status_page: intel_hw_status_page,
    pub wa_ctx: i915_ctx_workarounds,
    pub ctx_wa_list: i915_wa_list,
    pub wa_list: i915_wa_list,
    pub whitelist: i915_wa_list,
    pub /: *mut *mut u32 irq_keep_mask; / always keep these interrupts,
    pub /: *mut *mut u32 irq_enable_mask; / bitmask to enable ring interrupt,
    pub engine): *mut *mut void (irq_enable)(struct intel_engine_cs,
    pub engine): *mut *mut void (irq_disable)(struct intel_engine_cs,
    pub iir): *mut *mut *mut void (irq_handler)(struct intel_engine_cs engine, u16,
    pub engine): *mut *mut void (sanitize)(struct intel_engine_cs,
    pub engine): *mut *mut int (resume)(struct intel_engine_cs,
    pub engine): *mut *mut void (prepare)(struct intel_engine_cs,
    pub stalled): *mut *mut *mut void (rewind)(struct intel_engine_cs engine, bool,
    pub engine): *mut *mut void (cancel)(struct intel_engine_cs,
    pub engine): *mut *mut void (finish)(struct intel_engine_cs,
    pub reset: },
    pub engine): *mut *mut void (park)(struct intel_engine_cs,
    pub engine): *mut *mut void (unpark)(struct intel_engine_cs,
    pub engine): *mut *mut void (bump_serial)(struct intel_engine_cs,
    pub engine): *mut *mut void (set_default_submission)(struct intel_engine_cs,
    pub cops: *const intel_context_ops,
    pub rq): *mut *mut int (request_alloc)(struct i915_request,
    pub mode): *mut *mut *mut int (emit_flush)(struct i915_request request, u32,

    pub dispatch_flags): c_uint,

    pub rq): *mut *mut int (emit_init_breadcrumb)(struct i915_request,
    pub cs): *mut u32,
    pub emit_fini_breadcrumb_dw: c_uint,
// Pass the request to the hardware queue (e.g. directly into
// the legacy ringbuffer or to the end of an execlist).
//
// This is called from an atomic context with irqs disabled; must
// be irq safe.
//
    pub rq): *mut *mut void (submit_request)(struct i915_request,
    pub engine): *mut *mut void (release)(struct intel_engine_cs,
//
// Add / remove request from engine active tracking
//
    pub rq): *mut *mut void (add_active_request)(struct i915_request,
    pub rq): *mut *mut void (remove_active_request)(struct i915_request,
//
// Get engine busyness and the time at which the busyness was sampled.
//
    pub now): *mut ktime_t,
    pub execlists: intel_engine_execlists,
//
// Keep track of completed timelines on this engine for early
// retirement with the goal of quickly enabling powersaving as
// soon as the engine is idle.
//
    pub retire: *mut intel_timeline,
    pub retire_work: work_struct,
// status_notifier: list of callbacks for context-switch changes
    pub context_status_notifier: atomic_notifier_head,

    pub flags: c_uint,
//
// Table of commands the command parser needs to know about
// for this engine.
//
    pub I915_CMD_HASH_ORDER): DECLARE_HASHTABLE(cmd_hash,,
//
// Table of registers allowed in commands that read/write registers.
//
    pub reg_tables: *const drm_i915_reg_table,
    pub reg_table_count: c_int,
//
// Returns the bitmask for the length field of the specified command.
// Return 0 for an unrecognized/invalid command.
//
// If the command parser finds an entry for a command in the engine's
// cmd_tables, it gets the command's length based on the table entry.
// If not, it calls this function to determine the per-engine length
// field encoding for the command (i.e. different opcode ranges use
// certain bits to encode the command length in the header).
//
    pub cmd_header): *mut *mut u32 (get_cmd_length_mask)(u32,
    pub execlists: intel_engine_execlists_stats,
    pub guc: intel_engine_guc_stats,
}

//
// @rps: Utilisation at last RPS sampling.
//
// The perf group maps to one OA unit which controls one OA buffer. All
// reports corresponding to this engine will be reported to this OA
// buffer. An engine will map to a single OA unit, but a single OA unit
// can generate reports for multiple engines.
//
// Wa_14014475959:dg2
// Wa_16019325821
// Wa_14019159160
