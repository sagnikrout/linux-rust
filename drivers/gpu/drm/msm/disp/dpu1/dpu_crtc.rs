//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/dpu1/dpu_crtc.h
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
// Copyright (c) 2022 Qualcomm Innovation Center, Inc. All rights reserved.
// Copyright (c) 2015-2021 The Linux Foundation. All rights reserved.
// Copyright (C) 2013 Red Hat
// Author: Rob Clark <robdclark@gmail.com>
//

pub const DPU_CRTC_NAME_SIZE: c_int = 12;
// define the maximum number of in-flight frame events
pub const DPU_CRTC_FRAME_EVENT_SIZE: c_int = 4;
//
// enum dpu_crtc_client_type: crtc client type
// @RT_CLIENT:	RealTime client like video/cmd mode display
// voting through apps rsc
// @NRT_CLIENT:	Non-RealTime client like WB display
// voting through apps rsc
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_crtc_client_type {
    RT_CLIENT,
    NRT_CLIENT,
}

//
// enum dpu_crtc_smmu_state:	smmu state
// @ATTACHED:	 all the context banks are attached.
// @DETACHED:	 all the context banks are detached.
// @ATTACH_ALL_REQ:	 transient state of attaching context banks.
// @DETACH_ALL_REQ:	 transient state of detaching context banks.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_crtc_smmu_state {
    ATTACHED = 0,
    DETACHED,
    ATTACH_ALL_REQ,
    DETACH_ALL_REQ,
}

//
// enum dpu_crtc_smmu_state_transition_type: state transition type
// @NONE: no pending state transitions
// @PRE_COMMIT: state transitions should be done before processing the commit
// @POST_COMMIT: state transitions to be done after processing the commit.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_crtc_smmu_state_transition_type {
    NONE,
    PRE_COMMIT,
    POST_COMMIT
}

//
// struct dpu_crtc_smmu_state_data: stores the smmu state and transition type
// @state: current state of smmu context banks
// @transition_type: transition request type
// @transition_error: whether there is error while transitioning the state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_crtc_smmu_state_data {
    pub state: u32,
    pub transition_type: u32,
    pub transition_error: u32,
}

//
// enum dpu_crtc_crc_source: CRC source
// @DPU_CRTC_CRC_SOURCE_NONE: no source set
// @DPU_CRTC_CRC_SOURCE_LAYER_MIXER: CRC in layer mixer
// @DPU_CRTC_CRC_SOURCE_ENCODER: CRC in encoder
// @DPU_CRTC_CRC_SOURCE_INVALID: Invalid source
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_crtc_crc_source {
    DPU_CRTC_CRC_SOURCE_NONE = 0,
    DPU_CRTC_CRC_SOURCE_LAYER_MIXER,
    DPU_CRTC_CRC_SOURCE_ENCODER,
    DPU_CRTC_CRC_SOURCE_MAX,
    DPU_CRTC_CRC_SOURCE_INVALID = -1
}

//
// struct dpu_crtc_mixer: stores the map for each virtual pipeline in the CRTC
// @hw_lm:	LM HW Driver context
// @lm_ctl:	CTL Path HW driver context
// @lm_dspp:	DSPP HW driver context
// @mixer_op_mode:	mixer blending operation mode
// @flush_mask:	mixer flush mask for ctl, mixer and pipe
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_crtc_mixer {
    pub hw_lm: *mut dpu_hw_mixer,
    pub lm_ctl: *mut dpu_hw_ctl,
    pub hw_dspp: *mut dpu_hw_dspp,
    pub mixer_op_mode: u32,
}

//
// struct dpu_crtc_frame_event: stores crtc frame event for crtc processing
// @work:	base work structure
// @crtc:	Pointer to crtc handling this event
// @list:	event list
// @ts:		timestamp at queue entry
// @event:	event identifier
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_crtc_frame_event {
    pub work: kthread_work,
    pub crtc: *mut drm_crtc,
    pub list: list_head,
    pub ts: ktime_t,
    pub event: u32,
}

//
// Maximum number of free event structures to cache
//
pub const DPU_CRTC_MAX_EVENT_COUNT: c_int = 16;
//
// struct dpu_crtc - virtualized CRTC data structure
// @base          : Base drm crtc structure
// @name          : ASCII description of this crtc
// @event         : Pointer to last received drm vblank event. If there is a
// pending vblank event, this will be non-null.
// @vsync_count   : Running count of received vsync events
// @drm_requested_vblank : Whether vblanks have been enabled in the encoder
// @property_info : Opaque structure for generic property support
// @property_defaults : Array of default values for generic property support
// @vblank_cb_count : count of vblank callback since last reset
// @play_count    : frame count between crtc enable and disable
// @vblank_cb_time  : ktime at vblank count reset
// @enabled       : whether the DPU CRTC is currently enabled. updated in the
// commit-thread, not state-swap time which is earlier, so
// safe to make decisions on during VBLANK on/off work
// @feature_list  : list of color processing features supported on a crtc
// @active_list   : list of color processing features are active
// @dirty_list    : list of color processing features are dirty
// @ad_dirty: list containing ad properties that are dirty
// @ad_active: list containing ad properties that are active
// @frame_pending : Whether or not an update is pending
// @frame_events  : static allocation of in-flight frame events
// @frame_event_list : available frame event list
// @spin_lock     : spin lock for frame event, transaction status, etc...
// @frame_done_comp    : for frame_event_done synchronization
// @event_thread  : Pointer to event handler thread
// @event_worker  : Event worker queue
// @event_lock    : Spinlock around event handling code
// @phandle: Pointer to power handler
// @cur_perf      : current performance committed to clock/bandwidth driver
// @crc_source    : CRC source
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_crtc {
    pub base: drm_crtc,
    pub name: [c_char; DPU_CRTC_NAME_SIZE],
    pub event: *mut drm_pending_vblank_event,
    pub vsync_count: u32,
    pub vblank_cb_count: u32,
    pub play_count: u64,
    pub vblank_cb_time: ktime_t,
    pub enabled: bool,
    pub feature_list: list_head,
    pub active_list: list_head,
    pub dirty_list: list_head,
    pub ad_dirty: list_head,
    pub ad_active: list_head,
    pub frame_pending: core::sync::atomic::AtomicI32,
    pub frame_events: [dpu_crtc_frame_event; DPU_CRTC_FRAME_EVENT_SIZE],
    pub frame_event_list: list_head,
    pub spin_lock: spinlock_t,
    pub frame_done_comp: completion,
// for handling internal event thread
    pub event_lock: spinlock_t,
    pub cur_perf: dpu_core_perf_params,
    pub smmu_state: dpu_crtc_smmu_state_data,
}

//
// struct dpu_crtc_state - dpu container for atomic crtc state
// @base: Base drm crtc state structure
// @bw_control    : true if bw/clk controlled by core bw/clk properties
// @bw_split_vote : true if bw controlled by llcc/dram bw properties
// @lm_bounds     : LM boundaries based on current mode full resolution, no ROI.
// Origin top left of CRTC.
// @property_state: Local storage for msm_prop properties
// @property_values: Current crtc property values
// @input_fence_timeout_ns : Cached input fence timeout, in ns
// @new_perf: new performance state being requested
// @num_mixers    : Number of mixers in use
// @mixers        : List of active mixers
// @num_ctls      : Number of ctl paths in use
// @hw_ctls       : List of active ctl paths
// @crc_source    : CRC source
// @crc_frame_skip_count: Number of frames skipped before getting CRC
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_crtc_state {
    pub base: drm_crtc_state,
    pub bw_control: bool,
    pub bw_split_vote: bool,
    pub lm_bounds: [drm_rect; CRTC_DUAL_MIXERS],
    pub input_fence_timeout_ns: u64,
    pub new_perf: dpu_core_perf_params,
// HW Resources reserved for the crtc
    pub num_mixers: u32,
    pub mixers: [dpu_crtc_mixer; CRTC_DUAL_MIXERS],
    pub num_ctls: u32,
    pub hw_ctls: [*mut dpu_hw_ctl; CRTC_DUAL_MIXERS],
    pub crc_source: dpu_crtc_crc_source,
    pub crc_frame_skip_count: c_int,
}

//
// dpu_crtc_frame_pending - return the number of pending frames
// @crtc: Pointer to drm crtc object
//
extern "C" {
    pub fn dpu_crtc_vblank(crtc: *mut drm_crtc, en: bool) -> c_int;
}
extern "C" {
    pub fn dpu_crtc_vblank_callback(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn dpu_crtc_commit_kickoff(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn dpu_crtc_complete_commit(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn dpu_crtc_get_intf_mode(crtc: *mut drm_crtc) -> dpu_intf_mode;
}
//
// dpu_crtc_get_client_type - check the crtc type- rt, nrt etc.
// @crtc: Pointer to crtc
//
extern "C" {
    pub fn dpu_crtc_frame_event_cb(crtc: *mut drm_crtc, event: u32);
}
extern "C" {
    pub fn dpu_crtc_get_num_lm(state: *const drm_crtc_state) -> c_uint;
}
