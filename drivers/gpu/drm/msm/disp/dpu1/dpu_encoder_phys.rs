//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/dpu1/dpu_encoder_phys.h
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
// Copyright (c) 2022-2024 Qualcomm Innovation Center, Inc. All rights reserved.
// Copyright (c) 2015-2018 The Linux Foundation. All rights reserved.
//

pub const DPU_ENCODER_NAME_MAX: c_int = 16;
// wait for at most 2 vsync for lowest refresh rate (24hz)
pub const KICKOFF_TIMEOUT_MS: c_int = 84;

//
// enum dpu_enc_split_role - Role this physical encoder will play in a
// split-panel configuration, where one panel is master, and others slaves.
// Masters have extra responsibilities, like managing the VBLANK IRQ.
// @ENC_ROLE_SOLO:	This is the one and only panel. This encoder is master.
// @ENC_ROLE_MASTER:	This encoder is the master of a split panel config.
// @ENC_ROLE_SLAVE:	This encoder is not the master of a split panel config.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_enc_split_role {
    ENC_ROLE_SOLO,
    ENC_ROLE_MASTER,
    ENC_ROLE_SLAVE,
}

//
// enum dpu_enc_enable_state - current enabled state of the physical encoder
// @DPU_ENC_DISABLING:	Encoder transitioning to disable state
// Events bounding transition are encoder type specific
// @DPU_ENC_DISABLED:	Encoder is disabled
// @DPU_ENC_ENABLING:	Encoder transitioning to enabled
// Events bounding transition are encoder type specific
// @DPU_ENC_ENABLED:	Encoder is enabled
// @DPU_ENC_ERR_NEEDS_HW_RESET:	Encoder is enabled, but requires a hw_reset
// to recover from a previous error
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_enc_enable_state {
    DPU_ENC_DISABLING,
    DPU_ENC_DISABLED,
    DPU_ENC_ENABLING,
    DPU_ENC_ENABLED,
    DPU_ENC_ERR_NEEDS_HW_RESET
}

//
// struct dpu_encoder_phys_ops - Interface the physical encoders provide to
// the containing virtual encoder.
// @prepare_commit:		MSM Atomic Call, start of atomic commit sequence
// @is_master:			Whether this phys_enc is the current master
// encoder. Can be switched at enable time. Based
// on split_role and current mode (CMD/VID).
// @atomic_mode_set:		DRM Call. Set a DRM mode.
// This likely caches the mode, for use at enable.
// @enable:			DRM Call. Enable a DRM mode.
// @disable:			DRM Call. Disable mode.
// @control_vblank_irq		Register/Deregister for VBLANK IRQ
// @wait_for_commit_done:	Wait for hardware to have flushed the
// current pending frames to hardware
// @wait_for_tx_complete:	Wait for hardware to transfer the pixels
// to the panel
// @wait_for_vblank:		Wait for VBLANK, for sub-driver internal use
// @prepare_for_kickoff:	Do any work necessary prior to a kickoff
// For CMD encoder, may wait for previous tx done
// @handle_post_kickoff:	Do any work necessary post-kickoff work
// @trigger_start:		Process start event on physical encoder
// @needs_single_flush:		Whether encoder slaves need to be flushed
// @irq_enable:			Handler to enable all the encoder IRQs
// @irq_disable:		Handler to disable all the encoder IRQs
// @prepare_idle_pc:		phys encoder can update the vsync_enable status
// on idle power collapse prepare
// @restore:			Restore all the encoder configs.
// @get_line_count:		Obtain current vertical line count
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_encoder_phys_ops {
    pub encoder): *mut *mut void (prepare_commit)(struct dpu_encoder_phys,
    pub encoder): *mut *mut bool (is_master)(struct dpu_encoder_phys,
    pub conn_state): *mut drm_connector_state,
    pub encoder): *mut *mut void (enable)(struct dpu_encoder_phys,
    pub encoder): *mut *mut void (disable)(struct dpu_encoder_phys,
    pub enable): *mut *mut *mut int (control_vblank_irq)(struct dpu_encoder_phys enc, bool,
    pub phys_enc): *mut *mut int (wait_for_commit_done)(struct dpu_encoder_phys,
    pub phys_enc): *mut *mut int (wait_for_tx_complete)(struct dpu_encoder_phys,
    pub phys_enc): *mut *mut void (prepare_for_kickoff)(struct dpu_encoder_phys,
    pub phys_enc): *mut *mut void (handle_post_kickoff)(struct dpu_encoder_phys,
    pub phys_enc): *mut *mut void (trigger_start)(struct dpu_encoder_phys,
    pub phys_enc): *mut *mut bool (needs_single_flush)(struct dpu_encoder_phys,
    pub phys): *mut *mut void (irq_enable)(struct dpu_encoder_phys,
    pub phys): *mut *mut void (irq_disable)(struct dpu_encoder_phys,
    pub phys_enc): *mut *mut void (prepare_idle_pc)(struct dpu_encoder_phys,
    pub phys): *mut *mut void (restore)(struct dpu_encoder_phys,
    pub phys): *mut *mut int (get_line_count)(struct dpu_encoder_phys,
    pub phys): *mut *mut int (get_frame_count)(struct dpu_encoder_phys,
    pub job): *mut drm_writeback_job,
    pub job): *mut drm_writeback_job,
    pub phys_enc): *mut *mut bool (is_valid_for_commit)(struct dpu_encoder_phys,
}

//
// enum dpu_intr_idx - dpu encoder interrupt index
// @INTR_IDX_VSYNC:    Vsync interrupt for video mode panel
// @INTR_IDX_PINGPONG: Pingpong done interrupt for cmd mode panel
// @INTR_IDX_UNDERRUN: Underrun interrupt for video and cmd mode panel
// @INTR_IDX_RDPTR:    Readpointer done interrupt for cmd mode panel
// @INTR_IDX_WB_DONE:  Writeback done interrupt for virtual connector
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_intr_idx {
    INTR_IDX_VSYNC,
    INTR_IDX_PINGPONG,
    INTR_IDX_UNDERRUN,
    INTR_IDX_CTL_START,
    INTR_IDX_RDPTR,
    INTR_IDX_WB_DONE,
    INTR_IDX_MAX,
}

//
// struct dpu_encoder_phys - physical encoder that drives a single INTF block
// tied to a specific panel / sub-panel. Abstract type, sub-classed by
// phys_vid or phys_cmd for video mode or command mode encs respectively.
// @parent:		Pointer to the containing virtual encoder
// @ops:		Operations exposed to the virtual encoder
// @parent_ops:		Callbacks exposed by the parent to the phys_enc
// @hw_mdptop:		Hardware interface to the top registers
// @hw_ctl:		Hardware interface to the ctl registers
// @hw_pp:		Hardware interface to the ping pong registers
// @hw_intf:		Hardware interface to the intf registers
// @hw_wb:		Hardware interface to the wb registers
// @hw_cdm:		Hardware interface to the CDM registers
// @dpu_kms:		Pointer to the dpu_kms top level
// @cdm_cfg:		CDM block config needed to store WB/DP block's CDM configuration
// @cached_mode:	DRM mode cached at mode_set time, acted on in enable
// @vblank_ctl_lock:	Vblank ctl mutex lock to protect vblank_refcount
// @enabled:		Whether the encoder has enabled and running a mode
// @split_role:		Role to play in a split-panel configuration
// @intf_mode:		Interface mode
// @enc_spinlock:	Virtual-Encoder-Wide Spin Lock for IRQ purposes
// @enable_state:	Enable state tracking
// @vblank_refcount:	Reference count of vblank request
// @vsync_cnt:		Vsync count for the physical encoder
// @underrun_cnt:	Underrun count for the physical encoder
// @pending_kickoff_cnt:	Atomic counter tracking the number of kickoffs
// vs. the number of done/vblank irqs. Should hover
// between 0-2 Incremented when a new kickoff is
// scheduled. Decremented in irq handler
// @pending_ctlstart_cnt:	Atomic counter tracking the number of ctl start
// pending.
// @pending_kickoff_wq:		Wait queue for blocking until kickoff completes
// @irq:			IRQ indices
// @has_intf_te:		Interface TE configuration support
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_encoder_phys {
    pub parent: *mut drm_encoder,
    pub ops: dpu_encoder_phys_ops,
    pub hw_mdptop: *mut dpu_hw_mdp,
    pub hw_ctl: *mut dpu_hw_ctl,
    pub hw_pp: *mut dpu_hw_pingpong,
    pub hw_intf: *mut dpu_hw_intf,
    pub hw_wb: *mut dpu_hw_wb,
    pub hw_cdm: *mut dpu_hw_cdm,
    pub dpu_kms: *mut dpu_kms,
    pub cdm_cfg: dpu_hw_cdm_cfg,
    pub cached_mode: drm_display_mode,
    pub vblank_ctl_lock: mutex,
    pub split_role: dpu_enc_split_role,
    pub intf_mode: dpu_intf_mode,
    pub enc_spinlock: *mut spinlock_t,
    pub enable_state: dpu_enc_enable_state,
    pub vblank_refcount: c_int,
    pub vsync_cnt: core::sync::atomic::AtomicI32,
    pub underrun_cnt: core::sync::atomic::AtomicI32,
    pub pending_ctlstart_cnt: core::sync::atomic::AtomicI32,
    pub pending_kickoff_cnt: core::sync::atomic::AtomicI32,
    pub pending_kickoff_wq: wait_queue_head_t,
    pub irq: [c_uint; INTR_IDX_MAX],
    pub has_intf_te: bool,
}

extern "C" {
    pub fn atomic_inc_return(_arg: &phys->pending_kickoff_cnt) -> return;
}
//
// struct dpu_encoder_phys_wb - sub-class of dpu_encoder_phys to handle command
// mode specific operations
// @base:	Baseclass physical encoder structure
// @wbirq_refcount:     Reference count of writeback interrupt
// @wb_done_timeout_cnt: number of wb done irq timeout errors
// @wb_cfg:  writeback block config to store fb related details
// @wb_conn: backpointer to writeback connector
// @wb_job: backpointer to current writeback job
// @dest:   dpu buffer layout for current writeback output buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_encoder_phys_wb {
    pub base: dpu_encoder_phys,
    pub wbirq_refcount: core::sync::atomic::AtomicI32,
    pub wb_done_timeout_cnt: c_int,
    pub wb_cfg: dpu_hw_wb_cfg,
    pub wb_conn: *mut drm_writeback_connector,
    pub wb_job: *mut drm_writeback_job,
    pub dest: dpu_hw_fmt_layout,
}

//
// struct dpu_encoder_phys_cmd - sub-class of dpu_encoder_phys to handle command
// mode specific operations
// @base:	Baseclass physical encoder structure
// @intf_idx:	Intf Block index used by this phys encoder
// @stream_sel:	Stream selection for multi-stream interfaces
// @serialize_wait4pp:	serialize wait4pp feature waits for pp_done interrupt
// after ctl_start instead of before next frame kickoff
// @pp_timeout_report_cnt: number of pingpong done irq timeout errors
// @pending_vblank_cnt: Atomic counter tracking pending wait for VBLANK
// @pending_vblank_wq: Wait queue for blocking until VBLANK received
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_encoder_phys_cmd {
    pub base: dpu_encoder_phys,
    pub stream_sel: c_int,
    pub serialize_wait4pp: bool,
    pub pp_timeout_report_cnt: c_int,
    pub pending_vblank_cnt: core::sync::atomic::AtomicI32,
    pub pending_vblank_wq: wait_queue_head_t,
}

//
// struct dpu_enc_phys_init_params - initialization parameters for phys encs
// @dpu_kms:		Pointer to the dpu_kms top level
// @parent:		Pointer to the containing virtual encoder
// @parent_ops:		Callbacks exposed by the parent to the phys_enc
// @split_role:		Role to play in a split-panel configuration
// @hw_intf:		Hardware interface to the intf registers
// @hw_wb:		Hardware interface to the wb registers
// @enc_spinlock:	Virtual-Encoder-Wide Spin Lock for IRQ purposes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_enc_phys_init_params {
    pub dpu_kms: *mut dpu_kms,
    pub parent: *mut drm_encoder,
    pub split_role: dpu_enc_split_role,
    pub hw_intf: *mut dpu_hw_intf,
    pub hw_wb: *mut dpu_hw_wb,
    pub enc_spinlock: *mut spinlock_t,
}

//
// dpu_encoder_wait_info - container for passing arguments to irq wait functions
// @wq: wait queue structure
// @atomic_cnt: wait until atomic_cnt equals zero
// @timeout_ms: timeout value in milliseconds
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_encoder_wait_info {
    pub wq: *mut wait_queue_head_t,
    pub atomic_cnt: *mut core::sync::atomic::AtomicI32,
    pub timeout_ms: i64,
}

extern "C" {
    pub fn dpu_encoder_helper_trigger_start(phys_enc: *mut dpu_encoder_phys);
}
// Use merge_3d unless DSC MERGE topology is used
extern "C" {
    pub fn dpu_encoder_helper_get_cwb_mask(phys_enc: *mut dpu_encoder_phys) -> c_uint;
}
extern "C" {
    pub fn dpu_encoder_helper_get_dsc(phys_enc: *mut dpu_encoder_phys) -> c_uint;
}
extern "C" {
    pub fn dpu_encoder_get_drm_fmt(phys_enc: *mut dpu_encoder_phys) -> u32;
}
extern "C" {
    pub fn dpu_encoder_needs_periph_flush(phys_enc: *mut dpu_encoder_phys) -> bool;
}
extern "C" {
    pub fn dpu_encoder_helper_phys_cleanup(phys_enc: *mut dpu_encoder_phys);
}
