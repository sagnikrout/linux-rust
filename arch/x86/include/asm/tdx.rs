//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/tdx.h
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


// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2021-2022 Intel Corporation

//
// SW-defined error codes.
//
// Bits 47:40 == 0xFF indicate Reserved status code class that never used by
// TDX module.
//

//
// TDX module SEAMCALL leaf function error codes
//

pub const TDX_RND_NO_ENTROPY: c_uint = 0x8000020300000000ULL;
// Bit definitions of TDX_FEATURES0 metadata field

//
// TDX module and P-SEAMLDR version convention: "major.minor.update"
// (e.g., "1.5.08") with zero-padded two-digit update field.
//

//
// Used by the #VE exception handler to gather the #VE exception
// info from the TDX module. This is a software only structure
// and not part of the TDX module/VMM ABI.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ve_info {
    pub exit_reason: u64,
    pub exit_qual: u64,
// Guest Linear (virtual) Address
    pub gla: u64,
// Guest Physical Address
    pub gpa: u64,
    pub instr_len: u32,
    pub instr_info: u32,
}

extern "C" {
    pub fn tdx_early_init() -> void __init;
}
extern "C" {
    pub fn tdx_get_ve_info(ve: *mut ve_info);
}
extern "C" {
    pub fn tdx_handle_virt_exception(regs: *mut pt_regs, ve: *mut ve_info) -> bool;
}
extern "C" {
    pub fn tdx_halt();
}
extern "C" {
    pub fn tdx_early_handle_ve(regs: *mut pt_regs) -> bool;
}
extern "C" {
    pub fn tdx_mcall_get_report0(reportdata: *mut u8, tdreport: *mut u8) -> c_int;
}
extern "C" {
    pub fn tdx_mcall_extend_rtmr(index: u8, data: *mut u8) -> c_int;
}
extern "C" {
    pub fn tdx_hcall_get_quote(buf: *mut u8, size: usize) -> u64;
}
extern "C" {
    pub fn tdx_dump_attributes(td_attr: u64) -> void __init;
}
extern "C" {
    pub fn tdx_dump_td_ctls(td_ctls: u64) -> void __init;
}

extern "C" {
    pub fn tdx_init();
}
extern "C" {
    pub fn tdx_cpu_enable() -> c_int;
}
extern "C" {
    pub fn tdx_guest_keyid_alloc() -> c_int;
}
extern "C" {
    pub fn tdx_get_nr_guest_keyids() -> u32;
}
extern "C" {
    pub fn tdx_guest_keyid_free(keyid: c_uint);
}
extern "C" {
    pub fn tdx_quirk_reset_paddr(base: c_ulong, size: c_ulong);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tdx_td {
// TD root structure:
    pub tdr_page: *mut page,
    pub tdcs_nr_pages: c_int,
// TD control structure:
    pub tdcs_pages: *mut page,
// Size of `tdcx_pages` in struct tdx_vp
    pub tdcx_nr_pages: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tdx_vp {
// TDVP root page
    pub tdvpr_page: *mut page,
// precalculated page_to_phys(tdvpr_page) for use in noinstr code
    pub tdvpr_pa: phys_addr_t,
// TD vCPU control structure:
    pub tdcx_pages: *mut page,
}

extern "C" {
    pub fn tdx_sys_disable();
}
extern "C" {
    pub fn tdh_vp_enter(vp: *mut tdx_vp, args: *mut tdx_module_args) -> u64;
}
extern "C" {
    pub fn tdh_mng_addcx(td: *mut tdx_td, tdcs_page: *mut page) -> u64;
}
extern "C" {
    pub fn tdh_mem_sept_add(td: *mut tdx_td, gpa: u64, level: pg_level, page: *mut page, ext_err1: *mut u64, ext_err2: *mut u64) -> u64;
}
extern "C" {
    pub fn tdh_vp_addcx(vp: *mut tdx_vp, tdcx_page: *mut page) -> u64;
}
extern "C" {
    pub fn tdh_mem_range_block(td: *mut tdx_td, gpa: u64, level: pg_level, ext_err1: *mut u64, ext_err2: *mut u64) -> u64;
}
extern "C" {
    pub fn tdh_mng_key_config(td: *mut tdx_td) -> u64;
}
extern "C" {
    pub fn tdh_mng_create(td: *mut tdx_td, hkid: u16) -> u64;
}
extern "C" {
    pub fn tdh_vp_create(td: *mut tdx_td, vp: *mut tdx_vp) -> u64;
}
extern "C" {
    pub fn tdh_mng_rd(td: *mut tdx_td, field: u64, data: *mut u64) -> u64;
}
extern "C" {
    pub fn tdh_mr_extend(td: *mut tdx_td, gpa: u64, ext_err1: *mut u64, ext_err2: *mut u64) -> u64;
}
extern "C" {
    pub fn tdh_mr_finalize(td: *mut tdx_td) -> u64;
}
extern "C" {
    pub fn tdh_vp_flush(vp: *mut tdx_vp) -> u64;
}
extern "C" {
    pub fn tdh_mng_vpflushdone(td: *mut tdx_td) -> u64;
}
extern "C" {
    pub fn tdh_mng_key_freeid(td: *mut tdx_td) -> u64;
}
extern "C" {
    pub fn tdh_mng_init(td: *mut tdx_td, td_params: u64, extended_err: *mut u64) -> u64;
}
extern "C" {
    pub fn tdh_vp_init(vp: *mut tdx_vp, initial_rcx: u64, x2apicid: u32) -> u64;
}
extern "C" {
    pub fn tdh_vp_rd(vp: *mut tdx_vp, field: u64, data: *mut u64) -> u64;
}
extern "C" {
    pub fn tdh_vp_wr(vp: *mut tdx_vp, field: u64, data: u64, mask: u64) -> u64;
}
extern "C" {
    pub fn tdh_phymem_page_reclaim(page: *mut page, tdx_pt: *mut u64, tdx_owner: *mut u64, tdx_size: *mut u64) -> u64;
}
extern "C" {
    pub fn tdh_mem_track(tdr: *mut tdx_td) -> u64;
}
extern "C" {
    pub fn tdh_mem_page_remove(td: *mut tdx_td, gpa: u64, level: pg_level, ext_err1: *mut u64, ext_err2: *mut u64) -> u64;
}
extern "C" {
    pub fn tdh_phymem_cache_wb(resume: bool) -> u64;
}
extern "C" {
    pub fn tdh_phymem_page_wbinvd_tdr(td: *mut tdx_td) -> u64;
}
extern "C" {
    pub fn tdh_phymem_page_wbinvd_hkid(hkid: u64, pfn: kvm_pfn_t) -> u64;
}

