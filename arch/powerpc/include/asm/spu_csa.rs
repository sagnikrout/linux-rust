//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/spu_csa.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// spu_csa.h: Definitions for SPU context save area (CSA).
//
// (C) Copyright IBM 2005
//
// Author: Mark Nutter <mnutter@us.ibm.com>
//

//
// Total number of 128-bit registers.
//
pub const NR_SPU_GPRS: c_int = 128;
pub const NR_SPU_SPRS: c_int = 9;
pub const NR_SPU_REGS_PAD: c_int = 7;

pub const SPU_SAVE_COMPLETE: c_uint = 0x3FFB;
pub const SPU_RESTORE_COMPLETE: c_uint = 0x3FFC;
//
// Definitions for various 'stopped' status conditions,
// to be recreated during context restore.
//
pub const SPU_STOPPED_STATUS_P: c_int = 1;
pub const SPU_STOPPED_STATUS_I: c_int = 2;
pub const SPU_STOPPED_STATUS_H: c_int = 3;
pub const SPU_STOPPED_STATUS_S: c_int = 4;
pub const SPU_STOPPED_STATUS_S_I: c_int = 5;
pub const SPU_STOPPED_STATUS_S_P: c_int = 6;
pub const SPU_STOPPED_STATUS_P_H: c_int = 7;
pub const SPU_STOPPED_STATUS_P_I: c_int = 8;
pub const SPU_STOPPED_STATUS_R: c_int = 9;
//
// Definitions for software decrementer status flag.
//
pub const SPU_DECR_STATUS_RUNNING: c_uint = 0x1;
pub const SPU_DECR_STATUS_WRAPPED: c_uint = 0x2;
//
// spu_reg128 - generic 128-bit register definition.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spu_reg128 {
    pub slot: [u32; 4],
}

//
// struct spu_lscsa - Local Store Context Save Area.
// @gprs: Array of saved registers.
// @fpcr: Saved floating point status control register.
// @decr: Saved decrementer value.
// @decr_status: Indicates software decrementer status flags.
// @ppu_mb: Saved PPU mailbox data.
// @ppuint_mb: Saved PPU interrupting mailbox data.
// @tag_mask: Saved tag group mask.
// @event_mask: Saved event mask.
// @srr0: Saved SRR0.
// @stopped_status: Conditions to be recreated by restore.
// @ls: Saved contents of Local Storage Area.
//
// The LSCSA represents state that is primarily saved and
// restored by SPU-side code.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spu_lscsa {
    pub gprs: [spu_reg128; 128],
    pub fpcr: spu_reg128,
    pub decr: spu_reg128,
    pub decr_status: spu_reg128,
    pub ppu_mb: spu_reg128,
    pub ppuint_mb: spu_reg128,
    pub tag_mask: spu_reg128,
    pub event_mask: spu_reg128,
    pub srr0: spu_reg128,
    pub stopped_status: spu_reg128,
//
// 'ls' must be page-aligned on all configurations.
// Since we don't want to rely on having the spu-gcc
// installed to build the kernel and this structure
// is used in the SPU-side code, make it 64k-page
// aligned for now.
//
    pub __attribute__((aligned(65536))): unsigned char ls[LS_SIZE],
}

//
// struct spu_problem_collapsed - condensed problem state area, w/o pads.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spu_problem_collapsed {
    pub spc_mssync_RW: u64,
    pub mfc_lsa_W: u32,
    pub unused_pad0: u32,
    pub mfc_ea_W: u64,
    pub mfc_union_W: mfc_tag_size_class_cmd,
    pub dma_qstatus_R: u32,
    pub dma_querytype_RW: u32,
    pub dma_querymask_RW: u32,
    pub dma_tagstatus_R: u32,
    pub pu_mb_R: u32,
    pub spu_mb_W: u32,
    pub mb_stat_R: u32,
    pub spu_runcntl_RW: u32,
    pub spu_status_R: u32,
    pub spu_spc_R: u32,
    pub spu_npc_RW: u32,
    pub signal_notify1: u32,
    pub signal_notify2: u32,
    pub unused_pad1: u32,
}

//
// struct spu_priv1_collapsed - condensed privileged 1 area, w/o pads.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spu_priv1_collapsed {
    pub mfc_sr1_RW: u64,
    pub mfc_lpid_RW: u64,
    pub spu_idr_RW: u64,
    pub mfc_vr_RO: u64,
    pub spu_vr_RO: u64,
    pub int_mask_class0_RW: u64,
    pub int_mask_class1_RW: u64,
    pub int_mask_class2_RW: u64,
    pub int_stat_class0_RW: u64,
    pub int_stat_class1_RW: u64,
    pub int_stat_class2_RW: u64,
    pub int_route_RW: u64,
    pub mfc_atomic_flush_RW: u64,
    pub resource_allocation_groupID_RW: u64,
    pub resource_allocation_enable_RW: u64,
    pub mfc_fir_R: u64,
    pub mfc_fir_status_or_W: u64,
    pub mfc_fir_status_and_W: u64,
    pub mfc_fir_mask_R: u64,
    pub mfc_fir_mask_or_W: u64,
    pub mfc_fir_mask_and_W: u64,
    pub mfc_fir_chkstp_enable_RW: u64,
    pub smf_sbi_signal_sel: u64,
    pub smf_ato_signal_sel: u64,
    pub tlb_index_hint_RO: u64,
    pub tlb_index_W: u64,
    pub tlb_vpn_RW: u64,
    pub tlb_rpn_RW: u64,
    pub tlb_invalidate_entry_W: u64,
    pub tlb_invalidate_all_W: u64,
    pub smm_hid: u64,
    pub mfc_accr_RW: u64,
    pub mfc_dsisr_RW: u64,
    pub mfc_dar_RW: u64,
    pub rmt_index_RW: u64,
    pub rmt_data1_RW: u64,
    pub mfc_dsir_R: u64,
    pub mfc_lsacr_RW: u64,
    pub mfc_lscrr_R: u64,
    pub mfc_tclass_id_RW: u64,
    pub mfc_rm_boundary: u64,
    pub smf_dma_signal_sel: u64,
    pub smm_signal_sel: u64,
    pub mfc_cer_R: u64,
    pub pu_ecc_cntl_RW: u64,
    pub pu_ecc_stat_RW: u64,
    pub spu_ecc_addr_RW: u64,
    pub spu_err_mask_RW: u64,
    pub spu_trig0_sel: u64,
    pub spu_trig1_sel: u64,
    pub spu_trig2_sel: u64,
    pub spu_trig3_sel: u64,
    pub spu_trace_sel: u64,
    pub spu_event0_sel: u64,
    pub spu_event1_sel: u64,
    pub spu_event2_sel: u64,
    pub spu_event3_sel: u64,
    pub spu_trace_cntl: u64,
}

//
// struct spu_priv2_collapsed - condensed privileged 2 area, w/o pads.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spu_priv2_collapsed {
    pub slb_index_W: u64,
    pub slb_esid_RW: u64,
    pub slb_vsid_RW: u64,
    pub slb_invalidate_entry_W: u64,
    pub slb_invalidate_all_W: u64,
    pub spuq: [mfc_cq_sr; 16],
    pub puq: [mfc_cq_sr; 8],
    pub mfc_control_RW: u64,
    pub puint_mb_R: u64,
    pub spu_privcntl_RW: u64,
    pub spu_lslr_RW: u64,
    pub spu_chnlcntptr_RW: u64,
    pub spu_chnlcnt_RW: u64,
    pub spu_chnldata_RW: u64,
    pub spu_cfg_RW: u64,
    pub spu_tag_status_query_RW: u64,
    pub spu_cmd_buf1_RW: u64,
    pub spu_cmd_buf2_RW: u64,
    pub spu_atomic_status_RW: u64,
}

//
// struct spu_state
// @lscsa: Local Store Context Save Area.
// @prob: Collapsed Problem State Area, w/o pads.
// @priv1: Collapsed Privileged 1 Area, w/o pads.
// @priv2: Collapsed Privileged 2 Area, w/o pads.
// @spu_chnlcnt_RW: Array of saved channel counts.
// @spu_chnldata_RW: Array of saved channel data.
// @suspend_time: Time stamp when decrementer disabled.
//
// Structure representing the whole of the SPU
// context save area (CSA).  This struct contains
// all of the state necessary to suspend and then
// later optionally resume execution of an SPU
// context.
//
// The @lscsa region is by far the largest, and is
// allocated separately so that it may either be
// pinned or mapped to/from application memory, as
// appropriate for the OS environment.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spu_state {
    pub lscsa: *mut spu_lscsa,
    pub prob: spu_problem_collapsed,
    pub priv1: spu_priv1_collapsed,
    pub priv2: spu_priv2_collapsed,
    pub spu_chnlcnt_RW: [u64; 32],
    pub spu_chnldata_RW: [u64; 32],
    pub spu_mailbox_data: [u32; 4],
    pub pu_mailbox_data: [u32; 1],
    pub class_0_pending: u64 class_0_dar,,
    pub class_1_dsisr: u64 class_1_dar,,
    pub suspend_time: c_ulong,
    pub register_lock: spinlock_t,
}

