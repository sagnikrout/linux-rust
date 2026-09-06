//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/uc/intel_guc.h
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
// Copyright © 2014-2019 Intel Corporation
//

//
// struct intel_guc - Top level structure of GuC.
//
// It handles firmware loading and manages client pool. intel_guc owns an
// i915_sched_engine for submission.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_guc {
// @fw: the GuC firmware
    pub fw: intel_uc_fw,
// @log: sub-structure containing GuC log related data and objects
    pub log: intel_guc_log,
// @ct: the command transport communication channel
    pub ct: intel_guc_ct,
// @slpc: sub-structure containing SLPC related data and objects
    pub slpc: intel_guc_slpc,
// @capture: the error-state-capture module's data and objects
    pub capture: *mut intel_guc_state_capture,
// @dbgfs_node: debugfs node
    pub dbgfs_node: *mut dentry,
// @sched_engine: Global engine used to submit requests to GuC
    pub sched_engine: *mut i915_sched_engine,
//
// @stalled_request: if GuC can't process a request for any reason, we
// save it until GuC restarts processing. No other request can be
// submitted until the stalled request is processed.
//
    pub stalled_request: *mut i915_request,
//
// @submission_stall_reason: reason why submission is stalled
//
    pub submission_stall_reason: },
// intel_guc_recv interrupt related state
// @irq_lock: protects GuC irq state
    pub irq_lock: spinlock_t,
//
// @msg_enabled_mask: mask of events that are processed when receiving
// an INTEL_GUC_ACTION_DEFAULT G2H message.
//
    pub msg_enabled_mask: c_uint,
//
// @outstanding_submission_g2h: number of outstanding GuC to Host
// responses related to GuC submission, used to determine if the GT is
// idle
//
    pub outstanding_submission_g2h: core::sync::atomic::AtomicI32,
// @tlb_lookup: xarray to store all pending TLB invalidation requests
    pub tlb_lookup: xarray,
//
// @serial_slot: id to the initial waiter created in tlb_lookup,
// which is used only when failed to allocate new waiter.
//
    pub serial_slot: u32,
// @next_seqno: the next id (sequence number) to allocate.
    pub next_seqno: u32,
// @interrupts: pointers to GuC interrupt-managing functions.
    pub enabled: bool,
    pub guc): *mut *mut void (reset)(struct intel_guc,
    pub guc): *mut *mut void (enable)(struct intel_guc,
    pub guc): *mut *mut void (disable)(struct intel_guc,
    pub interrupts: },
//
// @submission_state: sub-structure for submission state protected by
// single lock
//
// @submission_state.lock: protects everything in
// submission_state, ce->guc_id.id, and ce->guc_id.ref
// when transitioning in and out of zero
//
    pub lock: spinlock_t,
//
// @submission_state.guc_ids: used to allocate new
// guc_ids, single-lrc
//
    pub guc_ids: ida,
//
// @submission_state.num_guc_ids: Number of guc_ids, selftest
// feature to be able to reduce this number while testing.
//
    pub num_guc_ids: c_int,
//
// @submission_state.guc_ids_bitmap: used to allocate
// new guc_ids, multi-lrc
//
    pub guc_ids_bitmap: *mut c_ulong,
//
// @submission_state.guc_id_list: list of intel_context
// with valid guc_ids but no refs
//
    pub guc_id_list: list_head,
//
// @submission_state.guc_ids_in_use: Number single-lrc
// guc_ids in use
//
    pub guc_ids_in_use: c_uint,
//
// @submission_state.destroyed_contexts: list of contexts
// waiting to be destroyed (deregistered with the GuC)
//
    pub destroyed_contexts: list_head,
//
// @submission_state.destroyed_worker: worker to deregister
// contexts, need as we need to take a GT PM reference and
// can't from destroy function as it might be in an atomic
// context (no sleeping)
//
    pub destroyed_worker: work_struct,
//
// @submission_state.reset_fail_worker: worker to trigger
// a GT reset after an engine reset fails
//
    pub reset_fail_worker: work_struct,
//
// @submission_state.reset_fail_mask: mask of engines that
// failed to reset
//
    pub reset_fail_mask: intel_engine_mask_t,
//
// @submission_state.sched_disable_delay_ms: schedule
// disable delay, in ms, for contexts
//
    pub sched_disable_delay_ms: c_uint,
//
// @submission_state.sched_disable_gucid_threshold:
// threshold of min remaining available guc_ids before
// we start bypassing the schedule disable delay
//
    pub sched_disable_gucid_threshold: c_uint,
    pub submission_state: },
//
// @submission_supported: tracks whether we support GuC submission on
// the current platform
//
    pub submission_supported: bool,
// @submission_selected: tracks whether the user enabled GuC submission
    pub submission_selected: bool,
// @submission_initialized: tracks whether GuC submission has been initialised
    pub submission_initialized: bool,
// @submission_version: Submission API version of the currently loaded firmware
    pub submission_version: intel_uc_fw_ver,
//
// @rc_supported: tracks whether we support GuC rc on the current platform
//
    pub rc_supported: bool,
// @rc_selected: tracks whether the user enabled GuC rc
    pub rc_selected: bool,
// @ads_vma: object allocated to hold the GuC ADS
    pub ads_vma: *mut i915_vma,
// @ads_map: contents of the GuC ADS
    pub ads_map: iosys_map,
// @ads_regset_size: size of the save/restore regsets in the ADS
    pub ads_regset_size: u32,
//
// @ads_regset_count: number of save/restore registers in the ADS for
// each engine
//
    pub ads_regset_count: [u32; I915_NUM_ENGINES],
// @ads_regset: save/restore regsets in the ADS
    pub ads_regset: *mut guc_mmio_reg,
// @ads_golden_ctxt_size: size of the golden contexts in the ADS
    pub ads_golden_ctxt_size: u32,
// @ads_waklv_size: size of workaround KLVs
    pub ads_waklv_size: u32,
// @ads_capture_size: size of register lists in the ADS used for error capture
    pub ads_capture_size: u32,
// @lrc_desc_pool_v69: object allocated to hold the GuC LRC descriptor pool
    pub lrc_desc_pool_v69: *mut i915_vma,
// @lrc_desc_pool_vaddr_v69: contents of the GuC LRC descriptor pool
    pub lrc_desc_pool_vaddr_v69: *mut c_void,
//
// @context_lookup: used to resolve intel_context from guc_id, if a
// context is present in this structure it is registered with the GuC
//
    pub context_lookup: xarray,
// @params: Control params for fw initialization
    pub params: [u32; GUC_CTL_MAX_DWORDS],
// @send_regs: GuC's FW specific registers used for sending MMIO H2G
    pub base: u32,
    pub count: c_uint,
    pub fw_domains: forcewake_domains,
    pub send_regs: },
// @notify_reg: register used to send interrupts to the GuC FW
    pub notify_reg: i915_reg_t,
//
// @mmio_msg: notification bitmask that the GuC writes in one of its
// registers when the CT channel is disabled, to be processed when the
// channel is back up.
//
    pub mmio_msg: u32,
// @send_mutex: used to serialize the intel_guc_send actions
    pub send_mutex: mutex,
//
// @timestamp: GT timestamp object that stores a copy of the timestamp
// and adjusts it for overflow using a worker.
//
// @timestamp.lock: Lock protecting the below fields and
// the engine stats.
//
    pub lock: spinlock_t,
//
// @timestamp.gt_stamp: 64-bit extended value of the GT
// timestamp.
//
    pub gt_stamp: u64,
//
// @timestamp.ping_delay: Period for polling the GT
// timestamp for overflow.
//
    pub ping_delay: c_ulong,
//
// @timestamp.work: Periodic work to adjust GT timestamp,
// engine and context usage for overflows.
//
    pub work: delayed_work,
//
// @timestamp.shift: Right shift value for the gpm timestamp
//
    pub shift: u32,
//
// @timestamp.last_stat_jiffies: jiffies at last actual
// stats collection time. We use this timestamp to ensure
// we don't oversample the stats because runtime power
// management events can trigger stats collection at much
// higher rates than required.
//
    pub last_stat_jiffies: c_ulong,
    pub timestamp: },
//
// @dead_guc_worker: Asynchronous worker thread for forcing a GuC reset.
// Specifically used when the G2H handler wants to issue a reset. Resets
// require flushing the G2H queue. So, the G2H processing itself must not
// trigger a reset directly. Instead, go via this worker.
//
    pub dead_guc_worker: work_struct,
//
// @last_dead_guc_jiffies: timestamp of previous 'dead guc' occurrence
// used to prevent a fundamentally broken system from continuously
// reloading the GuC.
//
    pub last_dead_guc_jiffies: c_ulong,

//
// @number_guc_id_stolen: The number of guc_ids that have been stolen
//
    pub number_guc_id_stolen: c_int,
//
// @fast_response_selftest: Backdoor to CT handler for fast response selftest
//
    pub fast_response_selftest: u32,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_guc_tlb_wait {
    pub wq: wait_queue_head,
    pub busy: bool,
}

//
// GuC version number components are only 8-bit, so converting to a 32bit 8.8.8
// integer works.
//

extern "C" {
    pub fn container_of(_arg: log, intel_guc: struct, _arg: log) -> return;
}
extern "C" {
    pub fn intel_guc_ct_send(_arg: &guc->ct, _arg: action, _arg: len, _arg: NULL, _arg: 0, _arg: 0) -> return;
}
//
// FIXME: Have caller pass in if we are in an atomic context to avoid
// using in_atomic(). It is likely safe here as we check for irqs
// disabled which basically all the spin locks in the i915 do but
// regardless this should be cleaned up.
//
// No sleeping with spin locks, just busy loop
// Only call this from the interrupt handler code
// GuC addresses above GUC_GGTT_TOP also don't map through the GTT
pub const GUC_GGTT_TOP: c_uint = 0xFEE00000;
//
// intel_guc_ggtt_offset() - Get and validate the GGTT offset of @vma
// @guc: intel_guc structure.
// @vma: i915 graphics virtual memory area.
//
// GuC does not allow any gfx GGTT address that falls into range
// [0, ggtt.pin_bias), which is reserved for Boot ROM, SRAM and WOPCM.
// Currently, in order to exclude [0, ggtt.pin_bias) address space from
// GGTT, all gfx objects used by GuC are allocated with intel_guc_allocate_vma()
// and pinned with PIN_OFFSET_BIAS along with the value of ggtt.pin_bias.
//
// Return: GGTT offset of the @vma.
//
extern "C" {
    pub fn intel_guc_init_early(guc: *mut intel_guc);
}
extern "C" {
    pub fn intel_guc_init_late(guc: *mut intel_guc);
}
extern "C" {
    pub fn intel_guc_init_send_regs(guc: *mut intel_guc);
}
extern "C" {
    pub fn intel_guc_write_params(guc: *mut intel_guc);
}
extern "C" {
    pub fn intel_guc_init(guc: *mut intel_guc) -> c_int;
}
extern "C" {
    pub fn intel_guc_fini(guc: *mut intel_guc);
}
extern "C" {
    pub fn intel_guc_notify(guc: *mut intel_guc);
}
extern "C" {
    pub fn intel_guc_auth_huc(guc: *mut intel_guc, rsa_offset: u32) -> c_int;
}
extern "C" {
    pub fn intel_guc_suspend(guc: *mut intel_guc) -> c_int;
}
extern "C" {
    pub fn intel_guc_resume(guc: *mut intel_guc) -> c_int;
}
extern "C" {
    pub fn intel_guc_self_cfg32(guc: *mut intel_guc, key: u16, value: u32) -> c_int;
}
extern "C" {
    pub fn intel_guc_self_cfg64(guc: *mut intel_guc, key: u16, value: u64) -> c_int;
}
extern "C" {
    pub fn intel_uc_fw_is_supported(_arg: &guc->fw) -> return;
}
extern "C" {
    pub fn intel_uc_fw_is_enabled(_arg: &guc->fw) -> return;
}
extern "C" {
    pub fn intel_uc_fw_is_available(_arg: &guc->fw) -> return;
}
extern "C" {
    pub fn intel_uc_fw_is_running(_arg: &guc->fw) -> return;
}
extern "C" {
    pub fn intel_guc_is_fw_running(intel_guc_ct_enabled(&guc->ct: guc) &&) -> return;
}
extern "C" {
    pub fn intel_guc_wait_for_idle(guc: *mut intel_guc, timeout: c_long) -> c_int;
}
extern "C" {
    pub fn intel_guc_crash_process_msg(guc: *mut intel_guc, action: u32) -> c_int;
}
extern "C" {
    pub fn intel_guc_find_hung_context(engine: *mut intel_engine_cs);
}
extern "C" {
    pub fn intel_guc_global_policies_update(guc: *mut intel_guc) -> c_int;
}
extern "C" {
    pub fn intel_guc_context_ban(ce: *mut intel_context, rq: *mut i915_request);
}
extern "C" {
    pub fn intel_guc_submission_reset_prepare(guc: *mut intel_guc);
}
extern "C" {
    pub fn intel_guc_submission_reset(guc: *mut intel_guc, stalled: intel_engine_mask_t);
}
extern "C" {
    pub fn intel_guc_submission_reset_finish(guc: *mut intel_guc);
}
extern "C" {
    pub fn intel_guc_submission_cancel_requests(guc: *mut intel_guc);
}
extern "C" {
    pub fn intel_guc_load_status(guc: *mut intel_guc, p: *mut drm_printer);
}
extern "C" {
    pub fn intel_guc_write_barrier(guc: *mut intel_guc);
}
extern "C" {
    pub fn intel_guc_dump_time_info(guc: *mut intel_guc, p: *mut drm_printer);
}
extern "C" {
    pub fn intel_guc_sched_disable_gucid_threshold_max(guc: *mut intel_guc) -> c_int;
}
extern "C" {
    pub fn intel_guc_tlb_invalidation_is_available(guc: *mut intel_guc) -> bool;
}
extern "C" {
    pub fn intel_guc_invalidate_tlb_engines(guc: *mut intel_guc) -> c_int;
}
extern "C" {
    pub fn intel_guc_invalidate_tlb_guc(guc: *mut intel_guc) -> c_int;
}
extern "C" {
    pub fn wake_up_all_tlb_invalidate(guc: *mut intel_guc);
}
