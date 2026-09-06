//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imagination/pvr_rogue_fwif_shared.h
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

// Maximum number of UFOs in a CCB command.

//
// This is a generic limit imposed on any DM (GEOMETRY,FRAGMENT,CDM,TDM,2D,TRANSFER)
// command passed through the bridge.
// Just across the bridge in the server, any incoming kick command size is
// checked against this maximum limit.
// In case the incoming command size is larger than the specified limit,
// the bridge call is retired with error.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_dma_addr {
    pub dev_addr: aligned_u64,
    pub fw_addr: u32,
    pub padding: u32,
    pub __aligned(8): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_ufo {
    pub addr: u32,
    pub value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_sync_checkpoint {
    pub state: u32,
    pub fw_ref_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_cleanup_ctl {
// Number of commands received by the FW
    pub submitted_commands: u32,
// Number of commands executed by the FW
    pub executed_commands: u32,
    pub __aligned(8): },
//
// Used to share frame numbers across UM-KM-FW,
// frame number is set in UM,
// frame number is required in both KM for HTB and FW for FW trace.
//
// May be used to house Kick flags in the future.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_cmd_common {
// associated frame number
    pub frame_num: u32,
}

//
// Geometry and fragment commands require set of firmware addresses that are stored in the Kernel.
// Client has handle(s) to Kernel containers storing these addresses, instead of raw addresses. We
// have to patch/write these addresses in KM to prevent UM from controlling FW addresses directly.
// Typedefs for geometry and fragment commands are shared between Client and Firmware (both
// single-BVNC). Kernel is implemented in a multi-BVNC manner, so it can't use geometry|fragment
// CMD type definitions directly. Therefore we have a SHARED block that is shared between UM-KM-FW
// across all BVNC configurations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_cmd_geom_frag_shared {
// Common command attributes
    pub cmn: rogue_fwif_cmd_common,
//
// RTData associated with this command, this is used for context
// selection and for storing out HW-context, when TA is switched out for
// continuing later
//
    pub hwrt_data_fw_addr: u32,
// Supported PR Buffers like Z/S/MSAA Scratch
    pub pr_buffer_fw_addr: [u32; ROGUE_FWIF_PRBUFFER_MAXSUPPORTED],
}

//
// Client Circular Command Buffer (CCCB) control structure.
// This is shared between the Server and the Firmware and holds byte offsets
// into the CCCB as well as the wrapping mask to aid wrap around. A given
// snapshot of this queue with Cmd 1 running on the GPU might be:
//
// Roff                           Doff                 Woff
// [..........|-1----------|=2===|=3===|=4===|~5~~~~|~6~~~~|~7~~~~|..........]
// <      runnable commands       ><   !ready to run   >
//
// Cmd 1    : Currently executing on the GPU data master.
// Cmd 2,3,4: Fence dependencies met, commands runnable.
// Cmd 5... : Fence dependency not met yet.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_cccb_ctl {
// Host write offset into CCB. This must be aligned to 16 bytes.
    pub write_offset: u32,
//
// Firmware read offset into CCB. Points to the command that is runnable
// on GPU, if R!=W
//
    pub read_offset: u32,
//
// Firmware fence dependency offset. Points to commands not ready, i.e.
// fence dependencies are not met.
//
    pub dep_offset: u32,
// Offset wrapping mask, total capacity in bytes of the CCB-1
    pub wrap_mask: u32,
// Only used if SUPPORT_AGP is present.
    pub read_offset2: u32,
// Only used if SUPPORT_AGP4 is present.
    pub read_offset3: u32,
// Only used if SUPPORT_AGP4 is present.
    pub read_offset4: u32,
    pub padding: u32,
    pub __aligned(8): },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_geom_registers_caswitch {
    pub geom_reg_vdm_context_state_base_addr: u64,
    pub geom_reg_vdm_context_state_resume_addr: u64,
    pub geom_reg_ta_context_state_base_addr: u64,
    pub geom_reg_vdm_context_store_task0: u64,
    pub geom_reg_vdm_context_store_task1: u64,
    pub geom_reg_vdm_context_store_task2: u64,
// VDM resume state update controls
    pub geom_reg_vdm_context_resume_task0: u64,
    pub geom_reg_vdm_context_resume_task1: u64,
    pub geom_reg_vdm_context_resume_task2: u64,
    pub geom_reg_vdm_context_store_task3: u64,
    pub geom_reg_vdm_context_store_task4: u64,
    pub geom_reg_vdm_context_resume_task3: u64,
    pub geom_reg_vdm_context_resume_task4: u64,
    pub geom_state: [}; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_cdm_registers_cswitch {
    pub cdmreg_cdm_context_pds0: u64,
    pub cdmreg_cdm_context_pds1: u64,
    pub cdmreg_cdm_terminate_pds: u64,
    pub cdmreg_cdm_terminate_pds1: u64,
// CDM resume controls
    pub cdmreg_cdm_resume_pds0: u64,
    pub cdmreg_cdm_context_pds0_b: u64,
    pub cdmreg_cdm_resume_pds0_b: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_static_rendercontext_state {
// Geom registers for ctx switch
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_static_computecontext_state {
// CDM registers for ctx switch
    pub __aligned(8): rogue_fwif_cdm_registers_cswitch ctxswitch_regs,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rogue_fwif_prbuffer_state {
    ROGUE_FWIF_PRBUFFER_UNBACKED = 0,
    ROGUE_FWIF_PRBUFFER_BACKED,
    ROGUE_FWIF_PRBUFFER_BACKING_PENDING,
    ROGUE_FWIF_PRBUFFER_UNBACKING_PENDING,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_prbuffer {
// Buffer ID
    pub buffer_id: u32,
// Needs On-demand Z/S/MSAA Buffer allocation
    pub __aligned(4): bool on_demand,
// Z/S/MSAA -Buffer state
    pub state: rogue_fwif_prbuffer_state,
// Cleanup state
    pub cleanup_sate: rogue_fwif_cleanup_ctl,
// Compatibility and other flags
    pub prbuffer_flags: u32,
    pub __aligned(8): },
// Last reset reason for a context.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rogue_context_reset_reason {
// No reset reason recorded
    ROGUE_CONTEXT_RESET_REASON_NONE = 0,
// Caused a reset due to locking up
    ROGUE_CONTEXT_RESET_REASON_GUILTY_LOCKUP = 1,
// Affected by another context locking up
    ROGUE_CONTEXT_RESET_REASON_INNOCENT_LOCKUP = 2,
// Overran the global deadline
    ROGUE_CONTEXT_RESET_REASON_GUILTY_OVERRUNING = 3,
// Affected by another context overrunning
    ROGUE_CONTEXT_RESET_REASON_INNOCENT_OVERRUNING = 4,
// Forced reset to ensure scheduling requirements
    ROGUE_CONTEXT_RESET_REASON_HARD_CONTEXT_SWITCH = 5,
// CDM Mission/safety checksum mismatch
    ROGUE_CONTEXT_RESET_REASON_WGP_CHECKSUM = 6,
// TRP checksum mismatch
    ROGUE_CONTEXT_RESET_REASON_TRP_CHECKSUM = 7,
// GPU ECC error (corrected, OK)
    ROGUE_CONTEXT_RESET_REASON_GPU_ECC_OK = 8,
// GPU ECC error (uncorrected, HWR)
    ROGUE_CONTEXT_RESET_REASON_GPU_ECC_HWR = 9,
// FW ECC error (corrected, OK)
    ROGUE_CONTEXT_RESET_REASON_FW_ECC_OK = 10,
// FW ECC error (uncorrected, ERR)
    ROGUE_CONTEXT_RESET_REASON_FW_ECC_ERR = 11,
// FW Safety watchdog triggered
    ROGUE_CONTEXT_RESET_REASON_FW_WATCHDOG = 12,
// FW page fault (no HWR)
    ROGUE_CONTEXT_RESET_REASON_FW_PAGEFAULT = 13,
// FW execution error (GPU reset requested)
    ROGUE_CONTEXT_RESET_REASON_FW_EXEC_ERR = 14,
// Host watchdog detected FW error
    ROGUE_CONTEXT_RESET_REASON_HOST_WDG_FW_ERR = 15,
// Geometry DM OOM event is not allowed
    ROGUE_CONTEXT_GEOM_OOM_DISABLED = 16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_context_reset_reason_data {
//
// The valid values for reset_reason are the ones from
// enum rogue_context_reset_reason
//
    pub reset_reason: u32,
    pub reset_ext_job_ref: u32,
}

