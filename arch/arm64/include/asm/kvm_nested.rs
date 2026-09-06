//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/kvm_nested.h
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

// Translation helpers from non-VHE EL2 to EL1
// Only preserve the minimal set of bits we support
// Clear the ASID field
extern "C" {
    pub fn forward_smc_trap(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn forward_debug_exception(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn kvm_init_nested(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_vcpu_init_nested(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_init_nested_s2_mmu(mmu: *mut kvm_s2_mmu);
}
extern "C" {
    pub fn kvm_vcpu_load_hw_mmu(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_vcpu_put_hw_mmu(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn check_nested_vcpu_requests(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_nested_flush_hwstate(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_nested_sync_hwstate(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_nested_setup_mdcr_el2(vcpu: *mut kvm_vcpu);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s2_trans {
    pub output: phys_addr_t,
    pub block_size: c_ulong,
    pub writable: bool,
    pub readable: bool,
    pub level: c_int,
    pub esr: u32,
    pub desc: u64,
}

extern "C" {
    pub fn kvm_inject_s2_fault(vcpu: *mut kvm_vcpu, esr_el2: u64) -> c_int;
}
extern "C" {
    pub fn kvm_nested_s2_wp(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_nested_s2_unmap(kvm: *mut kvm, may_block: bool);
}
extern "C" {
    pub fn kvm_nested_s2_flush(kvm: *mut kvm);
}
extern "C" {
    pub fn compute_tlb_inval_range(mmu: *mut kvm_s2_mmu, val: u64) -> c_ulong;
}
extern "C" {
    pub fn kvm_init_nv_sysregs(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn limit_nv_id_reg(kvm: *mut kvm, reg: u32, val: u64) -> u64;
}

extern "C" {
    pub fn kvm_auth_eretax(vcpu: *mut kvm_vcpu, elr: *mut u64) -> bool;
}

// We really should never execute this...
// elr = 0xbad9acc0debadbad;

extern "C" {
    pub fn FIELD_PREP(_arg: KVM_NV_GUEST_MAP_SZ, _arg: trans->level) -> return;
}
// Adjust alignment for the contiguous bit as per StageOA()

//
// We only deal with at most 48bit VA/IPA, so 48 is where we
// sign-extend from. Should we support FEAT_L{VP}A* at some point,
// this will need to be revisited.
//
// asid = FIELD_GET(TLBIR_ASID_MASK, val);
// range	= __TLBI_RANGE_PAGES(num, scale) << shift;
// Cap the range to the correct half of the address space
// range = min(*range, (BIT(48) - base));
// range = min(*range, ~base + 1);
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum trans_regime {
    TR_EL10,
    TR_EL20,
    TR_EL2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s1_walk_context {
    pub wi: *mut s1_walk_info,
    pub table_ipa: u64,
    pub level: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s1_walk_filter {
    pub ): *mut *mut *mut int (fn)(struct s1_walk_context , void,
    pub priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s1_walk_info {
    pub filter: *mut s1_walk_filter,
    pub baddr: u64,
    pub regime: trans_regime,
    pub max_oa_bits: c_uint,
    pub pgshift: c_uint,
    pub txsz: c_uint,
    pub sl: c_int,
    pub sh: u8,
    pub as_el0: bool,
    pub hpd: bool,
    pub e0poe: bool,
    pub poe: bool,
    pub pan: bool,
    pub be: bool,
    pub s2: bool,
    pub pa52bit: bool,
    pub ha: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s1_walk_result {
    pub desc: u64,
    pub pa: u64,
    pub level: i8,
    pub APTable: u8,
    pub nG: bool,
    pub asid: u16,
    pub UXNTable: bool,
    pub PXNTable: bool,
    pub uwxn: bool,
    pub uov: bool,
    pub ur: bool,
    pub uw: bool,
    pub ux: bool,
    pub pwxn: bool,
    pub pov: bool,
    pub pr: bool,
    pub pw: bool,
    pub px: bool,
}

// VNCR management
extern "C" {
    pub fn kvm_vcpu_allocate_vncr_tlb(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_handle_vncr_abort(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_handle_s1e2_tlbi(vcpu: *mut kvm_vcpu, inst: u32, val: u64);
}
extern "C" {
    pub fn get_asid_by_regime(vcpu: *mut kvm_vcpu, regime: trans_regime) -> u16;
}

extern "C" {
    pub fn __kvm_at_swap_desc(kvm: *mut kvm, ipa: gpa_t, old: u64, new: u64) -> c_int;
}
