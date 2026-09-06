//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/pxp/intel_pxp_types.h
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
// Copyright(c) 2020, Intel Corporation. All rights reserved.
//

//
// struct intel_pxp - pxp state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_pxp {
//
// @ctrl_gt: pointer to the tile that owns the controls for PXP subsystem assets that
// the VDBOX, the KCR engine (and GSC CS depending on the platform)
//
    pub ctrl_gt: *mut intel_gt,
//
// @platform_cfg_is_bad: used to track if any prior arb session creation resulted
// in a failure that was caused by a platform configuration issue, meaning that
// failure will not get resolved without a change to the platform (not kernel)
// such as BIOS configuration, firwmware update, etc. This bool gets reflected when
// GET_PARAM:I915_PARAM_PXP_STATUS is called.
//
    pub platform_cfg_is_bad: bool,
//
// @kcr_base: base mmio offset for the KCR engine which is different on legacy platforms
// vs newer platforms where the KCR is inside the media-tile.
//
    pub kcr_base: u32,
//
// @gsccs_res: resources for request submission for platforms that have a GSC engine.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsccs_session_resources {
    pub /: *mut *mut u64 host_session_handle; / used by firmware to link commands to sessions,
    pub /: *mut *mut *mut intel_context ce; / context for gsc command submission,
    pub /: *mut *mut *mut i915_vma pkt_vma; / GSC FW cmd packet vma,
    pub /: *mut *mut *mut void pkt_vaddr; / GSC FW cmd packet virt pointer,
    pub /: *mut *mut *mut i915_vma bb_vma; / HECI_PKT batch buffer vma,
    pub /: *mut *mut *mut void bb_vaddr; / HECI_PKT batch buffer virt pointer,
    pub gsccs_res: },
//
// @pxp_component: i915_pxp_component struct of the bound mei_pxp
// module. Only set and cleared inside component bind/unbind functions,
// which are protected by &tee_mutex.
//
    pub pxp_component: *mut i915_pxp_component,
//
// @dev_link: Enforce module relationship for power management ordering.
//
    pub dev_link: *mut device_link,
//
// @pxp_component_added: track if the pxp component has been added.
// Set and cleared in tee init and fini functions respectively.
//
    pub pxp_component_added: bool,
// @ce: kernel-owned context used for PXP operations
    pub ce: *mut intel_context,
// @arb_mutex: protects arb session start
    pub arb_mutex: mutex,
//
// @arb_is_valid: tracks arb session status.
// After a teardown, the arb session can still be in play on the HW
// even if the keys are gone, so we can't rely on the HW state of the
// session to know if it's valid and need to track the status in SW.
//
    pub arb_is_valid: bool,
//
// @key_instance: tracks which key instance we're on, so we can use it
// to determine if an object was created using the current key or a
// previous one.
//
    pub key_instance: u32,
// @tee_mutex: protects the tee channel binding and messaging.
    pub tee_mutex: mutex,
// @stream_cmd: LMEM obj used to send stream PXP commands to the GSC
    pub /: *mut *mut *mut drm_i915_gem_object obj; / contains PXP command memory,
    pub /: *mut *mut *mut void vaddr; / virtual memory for PXP command,
    pub stream_cmd: },
//
// @hw_state_invalidated: if the HW perceives an attack on the integrity
// of the encryption it will invalidate the keys and expect SW to
// re-initialize the session. We keep track of this state to make sure
// we only re-start the arb session when required.
//
    pub hw_state_invalidated: bool,
// @irq_enabled: tracks the status of the kcr irqs
    pub irq_enabled: bool,
//
// @termination: tracks the status of a pending termination. Only
// re-initialized under gt->irq_lock and completed in &session_work.
//
    pub termination: completion,
// @session_work: worker that manages session events.
    pub session_work: work_struct,
// @session_events: pending session events, protected with gt->irq_lock.
    pub session_events: u32,

}
