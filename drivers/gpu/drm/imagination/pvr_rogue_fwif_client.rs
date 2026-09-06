//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imagination/pvr_rogue_fwif_client.h
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
// Copyright (c) 2023 Imagination Technologies Ltd.

//
// Page size used for Parameter Management.
//

//
// Minimum/Maximum PB size.
//
// Base page size is dependent on core:
// S6/S6XT/S7               = 50 pages
// S8XE                     = 40 pages
// S8XE with BRN66011 fixed = 25 pages
//
// Minimum PB = Base Pages + (NUM_TE_PIPES-1)*16K + (NUM_VCE_PIPES-1)*64K +
// IF_PM_PREALLOC(NUM_TE_PIPES*16K + NUM_VCE_PIPES*16K)
//
// Maximum PB size must ensure that no PM address space can be fully used,
// because if the full address space was used it would wrap and corrupt itself.
// Since there are two freelists (local is always minimum sized) this can be
// described as following three conditions being met:
//
// (Minimum PB + Maximum PB)  <  ALIST PM address space size (16GB)
// (Minimum PB + Maximum PB)  <  TE PM address space size (16GB) / NUM_TE_PIPES
// (Minimum PB + Maximum PB)  <  VCE PM address space size (16GB) / NUM_VCE_PIPES
//
// Since the max of NUM_TE_PIPES and NUM_VCE_PIPES is 4, we have a hard limit
// of 4GB minus the Minimum PB. For convenience we take the smaller power-of-2
// value of 2GB. This is far more than any current applications use.
//

//
// Flags supported by the geometry DM command i.e. &struct rogue_fwif_cmd_geom.
//

// Use single core in a multi core setup.

//
// Flags supported by the fragment DM command i.e. &struct rogue_fwif_cmd_frag.
//
// Use single core in a multi core setup.

// Indicates whether this render produces visibility results.

// Indicates whether a depth buffer is present.

// Indicates whether a stencil buffer is present.

// Disable pixel merging for this render.

// Indicates whether a scratch buffer is present.

// Disallow compute overlapped with this render.

//
// Flags supported by the compute DM command i.e. &struct rogue_fwif_cmd_compute.
//

// !< Use single core in a multi core setup.

//
// Flags supported by the transfer DM command i.e. &struct rogue_fwif_cmd_transfer.
//
// !< Use single core in a multi core setup.

//
// Parameter/HWRTData control structures.
//
// Configuration registers which need to be loaded by the firmware before a geometry
// job can be started.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_geom_regs {
    pub vdm_ctrl_stream_base: u64,
    pub tpu_border_colour_table: u64,
// Only used when feature VDM_DRAWINDIRECT present.
    pub vdm_draw_indirect0: u64,
// Only used when feature VDM_DRAWINDIRECT present.
    pub vdm_draw_indirect1: u32,
    pub ppp_ctrl: u32,
    pub te_psg: u32,
// Only used when BRN 49927 present.
    pub tpu: u32,
    pub vdm_context_resume_task0_size: u32,
// Only used when feature VDM_OBJECT_LEVEL_LLS present.
    pub vdm_context_resume_task3_size: u32,
// Only used when BRN 56279 or BRN 67381 present.
    pub pds_ctrl: u32,
    pub view_idx: u32,
// Only used when feature TESSELLATION present
    pub pds_coeff_free_prog: u32,
    pub padding: u32,
}

// Only used when BRN 44455 or BRN 63027 present.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_dummy_rgnhdr_init_geom_regs {
    pub te_psgregion_addr: u64,
}

//
// Represents a geometry command that can be used to tile a whole scene's objects as
// per TA behavior.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_cmd_geom {
//
// rogue_fwif_cmd_geom_frag_shared field must always be at the beginning of the
// struct.
//
// The command struct (rogue_fwif_cmd_geom) is shared between Client and
// Firmware. Kernel is unable to perform read/write operations on the
// command struct, the SHARED region is the only exception from this rule.
// This region must be the first member so that Kernel can easily access it.
// For more info, see rogue_fwif_cmd_geom_frag_shared definition.
//
    pub cmd_shared: rogue_fwif_cmd_geom_frag_shared,
    pub __aligned(8): rogue_fwif_geom_regs regs,
    pub __aligned(8): u32 flags,
//
// Holds the geometry/fragment fence value to allow the fragment partial render command
// to go through.
//
    pub partial_render_geom_frag_fence: rogue_fwif_ufo,
// Only used when BRN 44455 or BRN 63027 present.
    pub __aligned(8): rogue_fwif_dummy_rgnhdr_init_geom_regs dummy_rgnhdr_init_geom_regs,
// Only used when BRN 61484 or BRN 66333 present.
    pub brn61484_66333_live_rt: u32,
    pub padding: u32,
}

//
// Configuration registers which need to be loaded by the firmware before ISP
// can be started.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_frag_regs {
    pub usc_pixel_output_ctrl: u32,
    pub usc_clear_register: [u32; ROGUE_MAXIMUM_OUTPUT_REGISTERS_PER_PIXEL],
    pub isp_bgobjdepth: u32,
    pub isp_bgobjvals: u32,
    pub isp_aa: u32,
// Only used when feature S7_TOP_INFRASTRUCTURE present.
    pub isp_xtp_pipe_enable: u32,
    pub isp_ctl: u32,
// Only used when BRN 49927 present.
    pub tpu: u32,
    pub event_pixel_pds_info: u32,
// Only used when feature CLUSTER_GROUPING present.
    pub pixel_phantom: u32,
    pub view_idx: u32,
    pub event_pixel_pds_data: u32,
// Only used when BRN 65101 present.
    pub brn65101_event_pixel_pds_data: u32,
// Only used when feature GPU_MULTICORE_SUPPORT or BRN 47217 present.
    pub isp_oclqry_stride: u32,
// Only used when feature ZLS_SUBTILE present.
    pub isp_zls_pixels: u32,
// Only used when feature ISP_ZLS_D24_S8_PACKING_OGL_MODE present.
    pub rgx_cr_blackpearl_fix: u32,
// All values below the ALIGN(8) must be 64 bit.
    pub isp_scissor_base: aligned_u64,
    pub isp_dbias_base: u64,
    pub isp_oclqry_base: u64,
    pub isp_zlsctl: u64,
    pub isp_zload_store_base: u64,
    pub isp_stencil_load_store_base: u64,
//
// Only used when feature FBCDC_ALGORITHM present and value < 3 or feature
// FB_CDC_V4 present. Additionally, BRNs 48754, 60227, 72310 and 72311 must
// not be present.
//
    pub fb_cdc_zls: u64,
    pub pbe_word: [u64; 8U][ROGUE_PBE_WORDS_REQUIRED_FOR_RENDERS],
    pub tpu_border_colour_table: u64,
    pub pds_bgnd: [u64; 3U],
// Only used when BRN 65101 present.
    pub pds_bgnd_brn65101: [u64; 3U],
    pub pds_pr_bgnd: [u64; 3U],
// Only used when BRN 62850 or 62865 present.
    pub isp_dummy_stencil_store_base: u64,
// Only used when BRN 66193 present.
    pub isp_dummy_depth_store_base: u64,
// Only used when BRN 67182 present.
    pub rgnhdr_single_rt_size: u32,
// Only used when BRN 67182 present.
    pub rgnhdr_scratch_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_cmd_frag {
    pub __aligned(8): rogue_fwif_cmd_geom_frag_shared cmd_shared,
    pub __aligned(8): rogue_fwif_frag_regs regs,
// command control flags.
    pub flags: u32,
// Stride IN BYTES for Z-Buffer in case of RTAs.
    pub zls_stride: u32,
// Stride IN BYTES for S-Buffer in case of RTAs.
    pub sls_stride: u32,
// Only used if feature GPU_MULTICORE_SUPPORT present.
    pub execute_count: u32,
}

//
// Configuration registers which need to be loaded by the firmware before CDM
// can be started.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_compute_regs {
    pub tpu_border_colour_table: u64,
// Only used when feature CDM_USER_MODE_QUEUE present.
    pub cdm_cb_queue: u64,
// Only used when feature CDM_USER_MODE_QUEUE present.
    pub cdm_cb_base: u64,
// Only used when feature CDM_USER_MODE_QUEUE present.
    pub cdm_cb: u64,
// Only used when feature CDM_USER_MODE_QUEUE is not present.
    pub cdm_ctrl_stream_base: u64,
    pub cdm_context_state_base_addr: u64,
// Only used when BRN 49927 is present.
    pub tpu: u32,
    pub cdm_resume_pds1: u32,
// Only used when feature COMPUTE_MORTON_CAPABLE present.
    pub cdm_item: u32,
// Only used when feature CLUSTER_GROUPING present.
    pub compute_cluster: u32,
// Only used when feature TPU_DM_GLOBAL_REGISTERS present.
    pub tpu_tag_cdm_ctrl: u32,
    pub padding: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_cmd_compute {
// Common command attributes
    pub __aligned(8): rogue_fwif_cmd_common common,
// CDM registers
    pub regs: rogue_fwif_compute_regs,
// Control flags
    pub __aligned(8): u32 flags,
// Only used when feature UNIFIED_STORE_VIRTUAL_PARTITIONING present.
    pub num_temp_regions: u32,
// Only used when feature CDM_USER_MODE_QUEUE present.
    pub stream_start_offset: u32,
// Only used when feature GPU_MULTICORE_SUPPORT present.
    pub execute_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_transfer_regs {
//
// All 32 bit values should be added in the top section. This then requires only a
// single RGXFW_ALIGN to align all the 64 bit values in the second section.
//
    pub isp_bgobjvals: u32,
    pub usc_pixel_output_ctrl: u32,
    pub usc_clear_register0: u32,
    pub usc_clear_register1: u32,
    pub usc_clear_register2: u32,
    pub usc_clear_register3: u32,
    pub isp_mtile_size: u32,
    pub isp_render_origin: u32,
    pub isp_ctl: u32,
// Only used when feature S7_TOP_INFRASTRUCTURE present.
    pub isp_xtp_pipe_enable: u32,
    pub isp_aa: u32,
    pub event_pixel_pds_info: u32,
    pub event_pixel_pds_code: u32,
    pub event_pixel_pds_data: u32,
    pub isp_render: u32,
    pub isp_rgn: u32,
// Only used when feature GPU_MULTICORE_SUPPORT present.
    pub frag_screen: u32,
// All values below the aligned_u64 must be 64 bit.
    pub pds_bgnd0_base: aligned_u64,
    pub pds_bgnd1_base: u64,
    pub pds_bgnd3_sizeinfo: u64,
    pub isp_mtile_base: u64,
pub const ROGUE_PBE_WORDS_REQUIRED_FOR_TQS: c_int = 3;
// TQ_MAX_RENDER_TARGETS * PBE_STATE_SIZE
    pub ROGUE_PBE_WORDS_REQUIRED_FOR_TQS]: *mut *mut u64 pbe_wordx_mrty[3U,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_cmd_transfer {
// Common command attributes
    pub __aligned(8): rogue_fwif_cmd_common common,
    pub __aligned(8): rogue_fwif_transfer_regs regs,
    pub flags: u32,
    pub padding: u32,
}

