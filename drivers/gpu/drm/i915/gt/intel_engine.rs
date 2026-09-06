//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/intel_engine.h
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

// Early gen2 devices have a cacheline of just 32 bytes, using 64 is overkill,
// but keeps the logic simple. Indeed, the whole purpose of this macro is just
// to give some inclination as to some of the magic values used in the various
// workarounds!
//
pub const CACHELINE_BYTES: c_int = 64;

//
// The register defines to be used with the following macros need to accept a
// base param, e.g:
//
// REG_FOO(base) _MMIO((base) + <relative offset>)
// ENGINE_READ(engine, REG_FOO);
//
// register arrays are to be defined and accessed as follows:
//
// REG_BAR(base, i) _MMIO((base) + <relative offset> + (i) * <shift>)
// ENGINE_READ_IDX(engine, REG_BAR, i)
//

// seqno size is actually only a uint32, but since we plan to use MI_FLUSH_DW to
// do the writes, and that must have qw aligned offsets, simply pretend it's 8b.
//
// Ensure that the compiler doesn't optimize away the load.
extern "C" {
    pub fn READ_ONCE(_arg: engine->status_page.addr[reg]) -> return;
}
// Writing into the status page should be done sparingly. Since
// we do when we are uncertain of the device state, we take a bit
// of extra paranoia to try and ensure that the HWS takes the value
// we give and that it doesn't end up trapped inside the CPU!
//
// Reads a dword out of the status page, which is written to from the command
// queue by automatic updates, MI_REPORT_HEAD, MI_STORE_DATA_INDEX, or
// MI_STORE_DATA_IMM.
//
// The following dwords have a reserved meaning:
// 0x00: ISR copy, updated when an ISR bit not set in the HWSTAM changes.
// 0x04: ring 0 head pointer
// 0x05: ring 1 head pointer (915-class)
// 0x06: ring 2 head pointer (915-class)
// 0x10-0x1b: Context status DWords (GM45)
// 0x1f: Last written status offset. (GM45)
// 0x20-0x2f: Reserved (Gen6+)
//
// The area from dword 0x30 to 0x3ff is available for driver usage.
//
pub const I915_GEM_HWS_PREEMPT: c_uint = 0x32;

pub const I915_GEM_HWS_SEQNO: c_uint = 0x40;

pub const I915_GEM_HWS_GGTT_BIND: c_uint = 0x46;

pub const I915_GEM_HWS_PXP: c_uint = 0x60;

pub const I915_GEM_HWS_GSC: c_uint = 0x62;

pub const I915_GEM_HWS_SCRATCH: c_uint = 0x80;
pub const I915_HWS_CSB_BUF0_INDEX: c_uint = 0x10;
pub const I915_HWS_CSB_WRITE_INDEX: c_uint = 0x1f;
pub const ICL_HWS_CSB_WRITE_INDEX: c_uint = 0x2f;

extern "C" {
    pub fn intel_engine_stop(engine: *mut intel_engine_cs);
}
extern "C" {
    pub fn intel_engine_cleanup(engine: *mut intel_engine_cs);
}
extern "C" {
    pub fn intel_engines_init_mmio(gt: *mut intel_gt) -> c_int;
}
extern "C" {
    pub fn intel_engines_init(gt: *mut intel_gt) -> c_int;
}
extern "C" {
    pub fn intel_engine_free_request_pool(engine: *mut intel_engine_cs);
}
extern "C" {
    pub fn intel_engines_release(gt: *mut intel_gt);
}
extern "C" {
    pub fn intel_engines_free(gt: *mut intel_gt);
}
extern "C" {
    pub fn intel_engine_init_common(engine: *mut intel_engine_cs) -> c_int;
}
extern "C" {
    pub fn intel_engine_cleanup_common(engine: *mut intel_engine_cs);
}
extern "C" {
    pub fn intel_engine_resume(engine: *mut intel_engine_cs) -> c_int;
}
extern "C" {
    pub fn intel_ring_submission_setup(engine: *mut intel_engine_cs) -> c_int;
}
extern "C" {
    pub fn intel_engine_stop_cs(engine: *mut intel_engine_cs) -> c_int;
}
extern "C" {
    pub fn intel_engine_cancel_stop_cs(engine: *mut intel_engine_cs);
}
extern "C" {
    pub fn intel_engine_wait_for_pending_mi_fw(engine: *mut intel_engine_cs);
}
extern "C" {
    pub fn intel_engine_set_hwsp_writemask(engine: *mut intel_engine_cs, mask: u32);
}
extern "C" {
    pub fn intel_engine_get_active_head(engine: *const intel_engine_cs) -> u64;
}
extern "C" {
    pub fn intel_engine_get_last_batch_head(engine: *const intel_engine_cs) -> u64;
}
extern "C" {
    pub fn intel_engine_init_execlists(engine: *mut intel_engine_cs);
}
extern "C" {
    pub fn intel_engine_irq_enable(engine: *mut intel_engine_cs) -> bool;
}
extern "C" {
    pub fn intel_engine_irq_disable(engine: *mut intel_engine_cs);
}
extern "C" {
    pub fn intel_engines_are_idle(gt: *mut intel_gt) -> bool;
}
extern "C" {
    pub fn intel_engine_is_idle(engine: *mut intel_engine_cs) -> bool;
}
extern "C" {
    pub fn __intel_engine_flush_submission(engine: *mut intel_engine_cs, sync: bool);
}
extern "C" {
    pub fn intel_engines_reset_default_submission(gt: *mut intel_gt);
}
extern "C" {
    pub fn intel_engine_can_store_dword(engine: *mut intel_engine_cs) -> bool;
}
extern "C" {
    pub fn intel_engine_context_size(gt: *mut intel_gt, class: u8) -> u32;
}
extern "C" {
    pub fn intel_engine_destroy_pinned_context(ce: *mut intel_context);
}
extern "C" {
    pub fn xehp_enable_ccs_engines(engine: *mut intel_engine_cs);
}
pub const ENGINE_PHYSICAL: c_int = 0;
pub const ENGINE_MOCK: c_int = 1;
pub const ENGINE_VIRTUAL: c_int = 2;
extern "C" {
    pub fn intel_engine_has_preemption(_arg: engine) -> return;
}

//
// For non-GuC submission we expect the back-end to look at the
// heartbeat status of the actual physical engine that the work
// has been (or is being) scheduled on, so we should only reach
// here with GuC submission enabled.
//
extern "C" {
    pub fn intel_guc_virtual_engine_has_heartbeat(_arg: engine) -> return;
}
extern "C" {
    pub fn intel_virtual_engine_has_heartbeat(_arg: engine) -> return;
}
extern "C" {
    pub fn READ_ONCE(_arg: engine->props.heartbeat_interval_ms) -> return;
}
extern "C" {
    pub fn intel_clamp_heartbeat_interval_ms(engine: *mut intel_engine_cs, value: u64) -> u64;
}
extern "C" {
    pub fn intel_clamp_max_busywait_duration_ns(engine: *mut intel_engine_cs, value: u64) -> u64;
}
extern "C" {
    pub fn intel_clamp_preempt_timeout_ms(engine: *mut intel_engine_cs, value: u64) -> u64;
}
extern "C" {
    pub fn intel_clamp_stop_timeout_ms(engine: *mut intel_engine_cs, value: u64) -> u64;
}
extern "C" {
    pub fn intel_clamp_timeslice_duration_ms(engine: *mut intel_engine_cs, value: u64) -> u64;
}

