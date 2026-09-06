//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/dpu1/dpu_hw_ctl.h
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
// Copyright (c) 2015-2018, The Linux Foundation. All rights reserved.
// Copyright (c) 2022-2024 Qualcomm Innovation Center, Inc. All rights reserved.
//

//
// enum dpu_ctl_mode_sel: Interface mode selection
// @DPU_CTL_MODE_SEL_VID:    Video mode interface
// @DPU_CTL_MODE_SEL_CMD:    Command mode interface
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_ctl_mode_sel {
    DPU_CTL_MODE_SEL_VID = 0,
    DPU_CTL_MODE_SEL_CMD
}

//
// struct dpu_hw_stage_cfg - blending stage cfg
// @stage : SSPP_ID at each stage
// @multirect_index: index of the rectangle of SSPP.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_stage_cfg {
    pub stage: [dpu_sspp; DPU_STAGE_MAX][PIPES_PER_STAGE],
}

//
// struct dpu_hw_intf_cfg :Describes how the DPU writes data to output interface
// @intf :                 Interface id
// @intf_master:           Master interface id in the dual pipe topology
// @wb:                    Writeback mode
// @mode_3d:               3d mux configuration
// @merge_3d:              3d merge block used
// @intf_mode_sel:         Interface mode, cmd / vid
// @cdm:                   CDM block used
// @stream_sel:            Stream selection for multi-stream interfaces
// @dsc:                   DSC BIT masks used
// @cwb:                   CWB BIT masks used
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_intf_cfg {
    pub intf: dpu_intf,
    pub intf_master: dpu_intf,
    pub wb: dpu_wb,
    pub mode_3d: dpu_3d_blend_mode,
    pub merge_3d: dpu_merge_3d,
    pub intf_mode_sel: dpu_ctl_mode_sel,
    pub cdm: dpu_cdm,
    pub stream_sel: c_int,
    pub cwb: c_uint,
    pub dsc: c_uint,
}

//
// struct dpu_hw_ctl_ops - Interface to the wb Hw driver functions
// Assumption is these functions will be called after clocks are enabled
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_ctl_ops {
//
// @trigger_start: kickoff hw operation for Sw controlled interfaces
// DSI cmd mode and WB interface are SW controlled
// @ctx       : ctl path ctx pointer
//
    pub ctx): *mut *mut void (trigger_start)(struct dpu_hw_ctl,
//
// @is_started: check if the ctl is started
// @ctx       : ctl path ctx pointer
// @Return: true if started, false if stopped
//
    pub ctx): *mut *mut bool (is_started)(struct dpu_hw_ctl,
//
// @trigger_pending: kickoff prepare is in progress hw operation for sw
// controlled interfaces: DSI cmd mode and WB interface
// are SW controlled
// @ctx       : ctl path ctx pointer
//
    pub ctx): *mut *mut void (trigger_pending)(struct dpu_hw_ctl,
//
// @clear_pending_flush: Clear the value of the cached pending_flush_mask
// No effect on hardware.
// Required to be implemented.
// @ctx       : ctl path ctx pointer
//
    pub ctx): *mut *mut void (clear_pending_flush)(struct dpu_hw_ctl,
//
// @get_pending_flush: Query the value of the cached pending_flush_mask
// No effect on hardware
// @ctx       : ctl path ctx pointer
//
    pub ctx): *mut *mut u32 (get_pending_flush)(struct dpu_hw_ctl,
//
// @update_pending_flush: OR in the given flushbits to the cached
// pending_flush_mask.
// No effect on hardware
// @ctx       : ctl path ctx pointer
// @flushbits : module flushmask
//
    pub flushbits): u32,
//
// @update_pending_flush_wb: OR in the given flushbits to the
// cached pending_(wb_)flush_mask.
// No effect on hardware
// @ctx       : ctl path ctx pointer
// @blk       : writeback block index
//
    pub blk): dpu_wb,
//
// @update_pending_flush_cwb: OR in the given flushbits to the
// cached pending_(cwb_)flush_mask.
// No effect on hardware
// @ctx       : ctl path ctx pointer
// @blk       : concurrent writeback block index
//
    pub blk): dpu_cwb,
//
// @update_pending_flush_intf: OR in the given flushbits to the
// cached pending_(intf_)flush_mask.
// No effect on hardware
// @ctx       : ctl path ctx pointer
// @blk       : interface block index
//
    pub blk): dpu_intf,
//
// @update_pending_flush_periph: OR in the given flushbits to the
// cached pending_(periph_)flush_mask.
// No effect on hardware
// @ctx       : ctl path ctx pointer
// @blk       : interface block index
//
    pub blk): dpu_intf,
//
// @update_pending_flush_merge_3d: OR in the given flushbits to the
// cached pending_(merge_3d_)flush_mask.
// No effect on hardware
// @ctx       : ctl path ctx pointer
// @blk       : interface block index
//
    pub blk): dpu_merge_3d,
//
// @update_pending_flush_sspp: OR in the given flushbits to the
// cached pending_flush_mask.
// No effect on hardware
// @ctx       : ctl path ctx pointer
// @blk       : SSPP block index
//
    pub blk): dpu_sspp,
//
// @update_pending_flush_mixer: OR in the given flushbits to the
// cached pending_flush_mask.
// No effect on hardware
// @ctx       : ctl path ctx pointer
// @blk       : LM block index
//
    pub blk): dpu_lm,
//
// @update_pending_flush_dspp: OR in the given flushbits to the
// cached pending_flush_mask.
// No effect on hardware
// @ctx       : ctl path ctx pointer
// @blk       : DSPP block index
// @dspp_sub_blk : DSPP sub-block index
//
    pub dspp_sub_blk): dpu_dspp blk, u32,
//
// @update_pending_flush_dsc: OR in the given flushbits to the
// cached pending_(dsc_)flush_mask.
// No effect on hardware
// @ctx: ctl path ctx pointer
// @blk: interface block index
//
    pub blk): dpu_dsc,
//
// @update_pending_flush_cdm: OR in the given flushbits to the
// cached pending_(cdm_)flush_mask.
// No effect on hardware
// @ctx: ctl path ctx pointer
// @cdm_num: idx of cdm to be flushed
//
    pub cdm_num): *mut *mut *mut void (update_pending_flush_cdm)(struct dpu_hw_ctl ctx, enum dpu_cdm,
//
// @trigger_flush: Write the value of the pending_flush_mask to hardware
// @ctx       : ctl path ctx pointer
//
    pub ctx): *mut *mut void (trigger_flush)(struct dpu_hw_ctl,
//
// @get_flush_register: Read the value of the flush register
// @ctx       : ctl path ctx pointer
// @Return: value of the ctl flush register.
//
    pub ctx): *mut *mut u32 (get_flush_register)(struct dpu_hw_ctl,
//
// @setup_intf_cfg: Setup ctl_path interface config
// @ctx
// @cfg    : interface config structure pointer
//
    pub cfg): *mut dpu_hw_intf_cfg,
//
// @reset_intf_cfg: reset ctl_path interface config
// @ctx    : ctl path ctx pointer
// @cfg    : interface config structure pointer
//
    pub cfg): *mut dpu_hw_intf_cfg,
//
// @reset: reset function for this ctl type
//
    pub c): *mut *mut int (reset)(struct dpu_hw_ctl,
//
// @wait_reset_status: checks ctl reset status
// @ctx       : ctl path ctx pointer
//
// This function checks the ctl reset status bit.
// If the reset bit is set, it keeps polling the status till the hw
// reset is complete.
// Returns: 0 on success or -error if reset incomplete within interval
//
    pub ctx): *mut *mut int (wait_reset_status)(struct dpu_hw_ctl,
//
// @clear_all_blendstages: Set all blend stages to disabled
// @ctx       : ctl path ctx pointer
//
    pub ctx): *mut *mut void (clear_all_blendstages)(struct dpu_hw_ctl,
//
// @setup_blendstage: Configure layer mixer to pipe configuration
// @ctx       : ctl path ctx pointer
// @lm        : layer mixer enumeration
// @cfg       : blend stage configuration
//
    pub cfg): *mut dpu_lm lm, struct dpu_hw_stage_cfg,
//
// @set_active_fetch_pipes: Set active pipes attached to this CTL
// @ctx: ctl path ctx pointer
// @active_pipes: bitmap of enum dpu_sspp
//
    pub fetch_active): *mut c_ulong,
//
// @set_active_pipes: Set active pipes attached to this CTL
// @ctx: ctl path ctx pointer
// @active_pipes: bitmap of enum dpu_sspp
//
    pub active_pipes): *mut c_ulong,
//
// @set_active_lms: Set active layer mixers attached to this CTL
// @ctx: ctl path ctx pointer
// @active_lms: bitmap of enum dpu_lm
//
    pub active_lms): *mut c_ulong,
}

//
// struct dpu_hw_ctl : CTL PATH driver object
// @base: hardware block base structure
// @hw: block register map object
// @idx: control path index
// @caps: control path capabilities
// @mixer_count: number of mixers
// @mixer_hw_caps: mixer hardware capabilities
// @pending_flush_mask: storage for pending ctl_flush managed via ops
// @pending_intf_flush_mask: pending INTF flush
// @pending_wb_flush_mask: pending WB flush
// @pending_cwb_flush_mask: pending CWB flush
// @pending_periph_flush_mask: pending PERIPH flush
// @pending_merge_3d_flush_mask: pending MERGE 3D flush
// @pending_dspp_flush_mask: pending DSPP flush
// @pending_dsc_flush_mask: pending DSC flush
// @pending_cdm_flush_mask: pending CDM flush
// @mdss_ver: MDSS revision information
// @ops: operation list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_ctl {
    pub base: dpu_hw_blk,
    pub hw: dpu_hw_blk_reg_map,
// ctl path
    pub idx: c_int,
    pub caps: *const dpu_ctl_cfg,
    pub mixer_count: c_int,
    pub mixer_hw_caps: *const dpu_lm_cfg,
    pub pending_flush_mask: u32,
    pub pending_intf_flush_mask: u32,
    pub pending_wb_flush_mask: u32,
    pub pending_cwb_flush_mask: u32,
    pub pending_periph_flush_mask: u32,
    pub pending_merge_3d_flush_mask: u32,
    pub DSPP_0]: u32 pending_dspp_flush_mask[DSPP_MAX -,
    pub pending_dsc_flush_mask: u32,
    pub pending_cdm_flush_mask: u32,
    pub mdss_ver: *const dpu_mdss_version,
// ops
    pub ops: dpu_hw_ctl_ops,
}

//
// to_dpu_hw_ctl - convert base object dpu_hw_base to container
// @hw: Pointer to base hardware block
// return: Pointer to hardware block container
//
extern "C" {
    pub fn container_of(_arg: hw, dpu_hw_ctl: struct, _arg: base) -> return;
}
