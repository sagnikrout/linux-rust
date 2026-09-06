//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/intel-pt-decoder/intel-pt-decoder.h
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
// intel_pt_decoder.h: Intel Processor Trace support
// Copyright (c) 2013-2014, Intel Corporation.
//

// Macro flag: #define INCLUDE__INTEL_PT_DECODER_H__

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_pt_sample_type {
    INTEL_PT_BRANCH		= 1 << 0,
    INTEL_PT_INSTRUCTION	= 1 << 1,
    INTEL_PT_TRANSACTION	= 1 << 2,
    INTEL_PT_PTW		= 1 << 3,
    INTEL_PT_MWAIT_OP	= 1 << 4,
    INTEL_PT_PWR_ENTRY	= 1 << 5,
    INTEL_PT_EX_STOP	= 1 << 6,
    INTEL_PT_PWR_EXIT	= 1 << 7,
    INTEL_PT_CBR_CHG	= 1 << 8,
    INTEL_PT_TRACE_BEGIN	= 1 << 9,
    INTEL_PT_TRACE_END	= 1 << 10,
    INTEL_PT_BLK_ITEMS	= 1 << 11,
    INTEL_PT_PSB_EVT	= 1 << 12,
    INTEL_PT_EVT		= 1 << 13,
    INTEL_PT_IFLAG_CHG	= 1 << 14,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_pt_period_type {
    INTEL_PT_PERIOD_NONE,
    INTEL_PT_PERIOD_INSTRUCTIONS,
    INTEL_PT_PERIOD_TICKS,
    INTEL_PT_PERIOD_MTC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_pt_param_flags {
//
// FUP packet can contain next linear instruction pointer instead of
// current linear instruction pointer.
//
    INTEL_PT_FUP_WITH_NLIP	= 1 << 0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_pt_blk_type {
    INTEL_PT_GP_REGS	= 1,
    INTEL_PT_PEBS_BASIC	= 4,
    INTEL_PT_PEBS_MEM	= 5,
    INTEL_PT_LBR_0		= 8,
    INTEL_PT_LBR_1		= 9,
    INTEL_PT_LBR_2		= 10,
    INTEL_PT_XMM		= 16,
    INTEL_PT_BLK_TYPE_MAX
}

//
// The block type numbers are not sequential but here they are given sequential
// positions to avoid wasting space for array placement.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_pt_blk_type_pos {
    INTEL_PT_GP_REGS_POS,
    INTEL_PT_PEBS_BASIC_POS,
    INTEL_PT_PEBS_MEM_POS,
    INTEL_PT_LBR_0_POS,
    INTEL_PT_LBR_1_POS,
    INTEL_PT_LBR_2_POS,
    INTEL_PT_XMM_POS,
    INTEL_PT_BLK_TYPE_CNT
}

// Get the array position for a block type

pub const INTEL_PT_BLK_ITEM_ID_CNT: c_int = 32;
//
// Use unions so that the block items can be accessed by name or by array index.
// There is an array of 32-bit masks for each block type, which indicate which
// values are present. Then arrays of 32 64-bit values for each block type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_pt_blk_items {
    pub mask: [u32; INTEL_PT_BLK_TYPE_CNT],
    pub has_rflags:1: u32,
    pub has_rip:1: u32,
    pub has_rax:1: u32,
    pub has_rcx:1: u32,
    pub has_rdx:1: u32,
    pub has_rbx:1: u32,
    pub has_rsp:1: u32,
    pub has_rbp:1: u32,
    pub has_rsi:1: u32,
    pub has_rdi:1: u32,
    pub has_r8:1: u32,
    pub has_r9:1: u32,
    pub has_r10:1: u32,
    pub has_r11:1: u32,
    pub has_r12:1: u32,
    pub has_r13:1: u32,
    pub has_r14:1: u32,
    pub has_r15:1: u32,
    pub has_unused_0:14: u32,
    pub has_ip:1: u32,
    pub has_applicable_counters:1: u32,
    pub has_timestamp:1: u32,
    pub has_unused_1:29: u32,
    pub has_mem_access_address:1: u32,
    pub has_mem_aux_info:1: u32,
    pub has_mem_access_latency:1: u32,
    pub has_tsx_aux_info:1: u32,
    pub has_unused_2:28: u32,
    pub has_lbr_0: u32,
    pub has_lbr_1: u32,
    pub has_lbr_2: u32,
    pub has_xmm: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_pt_vmcs_info {
    pub rb_node: rb_node,
    pub vmcs: u64,
    pub tsc_offset: u64,
    pub reliable: bool,
    pub error_printed: bool,
}

//
// Maximum number of event trace data in one go, assuming at most 1 per type
// and 6-bits of type in the EVD packet.
//
pub const INTEL_PT_MAX_EVDS: c_int = 64;
// Event trace data from EVD packet
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_pt_evd {
    pub type: c_int,
    pub payload: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_pt_state {
    pub type: intel_pt_sample_type,
    pub from_nr: bool,
    pub to_nr: bool,
    pub from_iflag: bool,
    pub to_iflag: bool,
    pub err: c_int,
    pub from_ip: u64,
    pub to_ip: u64,
    pub tot_insn_cnt: u64,
    pub tot_cyc_cnt: u64,
    pub cycles: u64,
    pub timestamp: u64,
    pub est_timestamp: u64,
    pub trace_nr: u64,
    pub ptw_payload: u64,
    pub mwait_payload: u64,
    pub pwre_payload: u64,
    pub pwrx_payload: u64,
    pub cbr_payload: u64,
    pub psb_offset: u64,
    pub cbr: u32,
    pub flags: u32,
    pub insn_op: intel_pt_insn_op,
    pub insn_len: c_int,
    pub insn: [c_char; INTEL_PT_INSN_BUF_SZ],
    pub items: intel_pt_blk_items,
    pub cfe_type: c_int,
    pub cfe_vector: c_int,
    pub evd_cnt: c_int,
    pub evd: *mut intel_pt_evd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_pt_buffer {
    pub buf: *const c_uchar,
    pub len: usize,
    pub consecutive: bool,
    pub ref_timestamp: u64,
    pub trace_nr: u64,
}

extern "C" {
    pub fn int(: *mut *mut intel_pt_lookahead_cb_t)(struct intel_pt_buffer, : *mut c_void) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_pt_params {
    pub data): *mut *mut *mut int (get_trace)(struct intel_pt_buffer buffer, void,
    pub data): *mut uint64_t max_insn_cnt, void,
    pub data): *mut *mut bool (pgd_ip)(uint64_t ip, void,
    pub cb_data): *mut *mut *mut int (lookahead)(void data, intel_pt_lookahead_cb_t cb, void,
    pub vmcs): *mut *mut *mut *mut intel_pt_vmcs_info (findnew_vmcs_info)(void data, uint64_t,
    pub data: *mut c_void,
    pub return_compression: bool,
    pub branch_enable: bool,
    pub vm_time_correlation: bool,
    pub vm_tm_corr_dry_run: bool,
    pub first_timestamp: u64,
    pub ctl: u64,
    pub period: u64,
    pub period_type: intel_pt_period_type,
    pub max_non_turbo_ratio: unsigned,
    pub mtc_period: c_uint,
    pub tsc_ctc_ratio_n: u32,
    pub tsc_ctc_ratio_d: u32,
    pub flags: intel_pt_param_flags,
    pub quick: c_uint,
    pub max_loops: c_int,
}

extern "C" {
    pub fn intel_pt_decoder_free(decoder: *mut intel_pt_decoder);
}
extern "C" {
    pub fn intel_pt_fast_forward(decoder: *mut intel_pt_decoder, timestamp: u64) -> c_int;
}
extern "C" {
    pub fn intel_pt__strerror(code: c_int, buf: *mut c_char, buflen: usize) -> c_int;
}
