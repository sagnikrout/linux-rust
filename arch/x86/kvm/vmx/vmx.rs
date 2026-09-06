//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/vmx/vmx.h
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

pub const MAX_NR_USER_RETURN_MSRS: c_int = 7;

pub const MAX_NR_USER_RETURN_MSRS: c_int = 4;

pub const MAX_NR_LOADSTORE_MSRS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmx_msrs {
    pub nr: c_uint,
    pub val: [vmx_msr_entry; MAX_NR_LOADSTORE_MSRS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmx_uret_msr {
    pub load_into_hardware: bool,
    pub data: u64,
    pub mask: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum segment_cache_field {
    SEG_FIELD_SEL = 0,
    SEG_FIELD_BASE = 1,
    SEG_FIELD_LIMIT = 2,
    SEG_FIELD_AR = 3,

    SEG_FIELD_NR = 4
}

pub const RTIT_ADDR_RANGE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_ctx {
    pub ctl: u64,
    pub status: u64,
    pub output_base: u64,
    pub output_mask: u64,
    pub cr3_match: u64,
    pub addr_a: [u64; RTIT_ADDR_RANGE],
    pub addr_b: [u64; RTIT_ADDR_RANGE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_desc {
    pub ctl_bitmask: u64,
    pub num_address_ranges: u32,
    pub PT_CPUID_LEAVES]: *mut *mut u32 caps[PT_CPUID_REGS_NUM,
    pub host: pt_ctx,
    pub guest: pt_ctx,
}

//
// The nested_vmx structure is part of vcpu_vmx, and holds information we need
// for correct emulation of VMX (i.e., nested VMX) on this vcpu.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nested_vmx {
// Has the level1 guest done vmxon?
    pub vmxon: bool,
    pub vmxon_ptr: gpa_t,
    pub pml_full: bool,
// The guest-physical address of the current VMCS L1 keeps for L2
    pub current_vmptr: gpa_t,
//
// Cache of the guest's VMCS, existing outside of guest memory.
// Loaded from guest memory during VMPTRLD. Flushed to guest
// memory during VMCLEAR and VMPTRLD.
//
    pub cached_vmcs12: *mut vmcs12,
//
// Cache of the guest's shadow VMCS, existing outside of guest
// memory. Loaded from guest memory during VM entry. Flushed
// to guest memory during VM exit.
//
    pub cached_shadow_vmcs12: *mut vmcs12,
//
// GPA to HVA cache for accessing vmcs12->vmcs_link_pointer
//
    pub shadow_vmcs12_cache: gfn_to_hva_cache,
//
// GPA to HVA cache for VMCS12
//
    pub vmcs12_cache: gfn_to_hva_cache,
//
// Indicates if the shadow vmcs or enlightened vmcs must be updated
// with the data held by struct vmcs12.
//
    pub need_vmcs12_to_shadow_sync: bool,
    pub dirty_vmcs12: bool,
//
// Indicates whether MSR bitmap for L2 needs to be rebuilt due to
// changes in MSR bitmap for L1 or switching to a different L2. Note,
// this flag can only be used reliably in conjunction with a paravirt L1
// which informs L0 whether any changes to MSR bitmap for L2 were done
// on its side.
//
    pub force_msr_bitmap_recalc: bool,
//
// Indicates lazily loaded guest state has not yet been decached from
// vmcs02.
//
    pub need_sync_vmcs02_to_vmcs12_rare: bool,
//
// vmcs02 has been initialized, i.e. state that is constant for
// vmcs02 has been written to the backing VMCS.  Initialization
// is delayed until L1 actually attempts to run a nested VM.
//
    pub vmcs02_initialized: bool,
//
// Enlightened VMCS has been enabled. It does not mean that L1 has to
// use it. However, VMX features available to L1 will be limited based
// on what the enlightened VMCS supports.
//
    pub enlightened_vmcs_enabled: bool,
// Pending MTF VM-exit into L1.
    pub mtf_pending: bool,
    pub vmcs02: loaded_vmcs,
//
// Guest pages referred to in the vmcs02 with host-physical
// pointers, so we must keep them pinned while L2 runs.
//
    pub apic_access_page_map: kvm_host_map,
    pub virtual_apic_map: kvm_host_map,
    pub pi_desc_map: kvm_host_map,
    pub pi_desc: *mut pi_desc,
    pub pi_pending: bool,
    pub posted_intr_nv: u16,
    pub preemption_timer: hrtimer,
    pub preemption_timer_deadline: u64,
    pub has_preemption_timer_deadline: bool,
    pub preemption_timer_expired: bool,
//
// Used to restore L1's CR3 if hardware detects a VM-Fail Consistency
// Check that KVM does not, in which case KVM needs to unwind CR3 back
// to its pre-VM-Enter state, NOT to vmcs01.HOST_CR3.
//
    pub pre_vmenter_cr3: c_ulong,
//
// Used to snapshot MSRs that are conditionally loaded on VM-Enter in
// order to propagate the guest's pre-VM-Enter value into vmcs02.  For
// emulation of VMLAUNCH/VMRESUME, the snapshot will be of L1's value.
// For KVM_SET_NESTED_STATE, the snapshot is of L2's value, _if_
// userspace restores MSRs before nested state.  If userspace restores
// MSRs after nested state, the snapshot holds garbage, but KVM can't
// detect that, and the garbage value in vmcs02 will be overwritten by
// MSR restoration in any case.
//
    pub pre_vmenter_debugctl: u64,
    pub pre_vmenter_bndcfgs: u64,
    pub pre_vmenter_s_cet: u64,
    pub pre_vmenter_ssp: u64,
    pub pre_vmenter_ssp_tbl: u64,
    pub vpid02: u16,
    pub last_vpid: u16,
    pub tsc_autostore_slot: c_int,
    pub msrs: nested_vmx_msrs,
// SMM related state
// in VMX operation on SMM entry?
    pub vmxon: bool,
// in guest mode on SMM entry?
    pub guest_mode: bool,
    pub smm: },

    pub hv_evmcs_vmptr: gpa_t,
    pub hv_evmcs_map: kvm_host_map,
    pub hv_evmcs: *mut hv_enlightened_vmcs,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcpu_vmx {
    pub vcpu: kvm_vcpu,
    pub vt: vcpu_vt,
    pub fail: u8,
    pub x2apic_msr_bitmap_mode: u8,
    pub idt_vectoring_info: u32,
    pub rflags: c_ulong,
//
// User return MSRs are always emulated when enabled in the guest, but
// only loaded into hardware when necessary, e.g. SYSCALL #UDs outside
// of 64-bit mode or if EFER.SCE=1, thus the SYSCALL MSRs don't need to
// be loaded into hardware if those conditions aren't met.
//
    pub guest_uret_msrs: [vmx_uret_msr; MAX_NR_USER_RETURN_MSRS],
    pub guest_uret_msrs_loaded: bool,

    pub msr_guest_kernel_gs_base: u64,

    pub spec_ctrl: u64,
    pub msr_ia32_umwait_control: u32,
//
// loaded_vmcs points to the VMCS currently used in this vcpu. For a
// non-nested (L1) guest, it always points to vmcs01. For a nested
// guest (L2), it points to a different VMCS.
//
    pub vmcs01: loaded_vmcs,
    pub loaded_vmcs: *mut loaded_vmcs,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msr_autoload {
    pub guest: vmx_msrs,
    pub host: vmx_msrs,
    pub msr_autoload: },
    pub msr_autostore: vmx_msrs,
    pub vm86_active: c_int,
    pub save_rflags: c_ulong,
    pub segs: [kvm_segment; 8],
    pub rmode: },
    pub /: *mut *mut u32 bitmask; / 4 bits per segment (1 bit per field),
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_save_segment {
    pub selector: u16,
    pub base: c_ulong,
    pub limit: u32,
    pub ar: u32,
    pub seg: [}; 8],
    pub segment_cache: },
    pub vpid: c_int,
// Support for a guest hypervisor (nested VMX)
    pub nested: nested_vmx,
// Dynamic PLE window.
    pub ple_window: c_uint,
    pub ple_window_dirty: bool,
// Support for PML
pub const PML_LOG_NR_ENTRIES: c_int = 512;
// PML is written backwards: this is the first entry written by the CPU

    pub pml_pg: *mut page,
// apic deadline value in host tsc
    pub hv_deadline_tsc: u64,
//
// Only bits masked by msr_ia32_feature_control_valid_bits can be set in
// msr_ia32_feature_control. FEAT_CTL_LOCKED is always included
// in msr_ia32_feature_control_valid_bits.
//
    pub msr_ia32_feature_control: u64,
    pub msr_ia32_feature_control_valid_bits: u64,
// SGX Launch Control public key hash
    pub msr_ia32_sgxlepubkeyhash: [u64; 4],
    pub msr_ia32_mcu_opt_ctrl: u64,
    pub disable_fb_clear: bool,
    pub pt_desc: pt_desc,
    pub lbr_desc: lbr_desc,
// ve_info must be page aligned.
    pub ve_info: *mut vmx_ve_information,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vmx {
    pub kvm: kvm,
    pub tss_addr: c_uint,
    pub ept_identity_pagetable_done: bool,
    pub ept_identity_map_addr: gpa_t,
// Posted Interrupt Descriptor (PID) table for IPI virtualization
    pub pid_table: *mut u64,
}

extern "C" {
    pub fn vmx_vcpu_load_vmcs(vcpu: *mut kvm_vcpu, cpu: c_int);
}
extern "C" {
    pub fn allocate_vpid() -> c_int;
}
extern "C" {
    pub fn free_vpid(vpid: c_int);
}
extern "C" {
    pub fn vmx_set_constant_host_state(vmx: *mut vcpu_vmx);
}
extern "C" {
    pub fn vmx_prepare_switch_to_guest(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vmx_get_cpl(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn vmx_get_cpl_no_cache(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn vmx_emulation_required(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn vmx_get_rflags(vcpu: *mut kvm_vcpu) -> c_ulong;
}
extern "C" {
    pub fn vmx_set_rflags(vcpu: *mut kvm_vcpu, rflags: c_ulong);
}
extern "C" {
    pub fn vmx_get_interrupt_shadow(vcpu: *mut kvm_vcpu) -> u32;
}
extern "C" {
    pub fn vmx_set_interrupt_shadow(vcpu: *mut kvm_vcpu, mask: c_int);
}
extern "C" {
    pub fn vmx_set_efer(vcpu: *mut kvm_vcpu, efer: u64) -> c_int;
}
extern "C" {
    pub fn vmx_set_cr0(vcpu: *mut kvm_vcpu, cr0: c_ulong);
}
extern "C" {
    pub fn vmx_set_cr4(vcpu: *mut kvm_vcpu, cr4: c_ulong);
}
extern "C" {
    pub fn set_cr4_guest_host_mask(vmx: *mut vcpu_vmx);
}
extern "C" {
    pub fn ept_save_pdptrs(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vmx_get_segment(vcpu: *mut kvm_vcpu, var: *mut kvm_segment, seg: c_int);
}
extern "C" {
    pub fn __vmx_set_segment(vcpu: *mut kvm_vcpu, var: *mut kvm_segment, seg: c_int);
}
extern "C" {
    pub fn vmx_guest_inject_ac(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn vmx_update_exception_bitmap(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vmx_nmi_blocked(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn __vmx_interrupt_blocked(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn vmx_interrupt_blocked(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn vmx_get_nmi_mask(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn vmx_set_nmi_mask(vcpu: *mut kvm_vcpu, masked: bool);
}
extern "C" {
    pub fn vmx_set_virtual_apic_mode(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn pt_update_intercept_for_msr(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vmx_update_host_rsp(vmx: *mut vcpu_vmx, host_rsp: c_ulong);
}
extern "C" {
    pub fn __vmx_vcpu_enter_flags(vmx: *mut vcpu_vmx) -> c_uint;
}
extern "C" {
    pub fn __vmx_vcpu_run(vmx: *mut vcpu_vmx, flags: c_uint) -> bool;
}
extern "C" {
    pub fn vmx_ept_load_pdptrs(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vmx_set_intercept_for_msr(vcpu: *mut kvm_vcpu, msr: u32, type: c_int, set: bool);
}
extern "C" {
    pub fn vmx_get_l2_tsc_offset(vcpu: *mut kvm_vcpu) -> u64;
}
extern "C" {
    pub fn vmx_get_l2_tsc_multiplier(vcpu: *mut kvm_vcpu) -> u64;
}
extern "C" {
    pub fn vmx_get_untagged_addr(vcpu: *mut kvm_vcpu, gva: gva_t, flags: c_uint) -> gva_t;
}
extern "C" {
    pub fn vmx_update_cpu_dirty_logging(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vmx_get_supported_debugctl(vcpu: *mut kvm_vcpu, host_initiated: bool) -> u64;
}
extern "C" {
    pub fn vmx_is_valid_debugctl(vcpu: *mut kvm_vcpu, data: u64, host_initiated: bool) -> bool;
}

//
// Note, early Intel manuals have the write-low and read-high bitmap offsets
// the wrong way round.  The bitmaps control MSRs 0x00000000-0x00001fff and
// 0xc0000000-0xc0001fff.  The former (low) uses bytes 0-0x3ff for reads and
// 0x800-0xbff for writes.  The latter (high) uses 0x400-0x7ff for reads and
// 0xc00-0xfff for writes.  MSRs not covered by either of the ranges always
// VM-Exit.
//

pub const KVM_REQUIRED_VMX_SECONDARY_VM_EXEC_CONTROL: c_int = 0;

pub const KVM_REQUIRED_VMX_TERTIARY_VM_EXEC_CONTROL: c_int = 0;

//
// VMX_REGS_LAZY_LOAD_SET - The set of registers that will be updated in the
// cache on demand.  Other registers not listed here are synced to
// the cache immediately after VM-Exit.
//

//
// CR0.WP needs to be intercepted when KVM is shadowing legacy paging
// in order to construct shadow PTEs with the correct protections.
// Note!  CR0.WP technically can be passed through to the guest if
// paging is disabled, but checking CR0.PG would generate a cyclical
// dependency of sorts due to forcing the caller to ensure CR0 holds
// the correct value prior to determining which CR0 bits can be owned
// by L1.  Keep it simple and limit the optimization to EPT.
//
extern "C" {
    pub fn container_of(_arg: kvm, kvm_vmx: struct, _arg: kvm) -> return;
}
extern "C" {
    pub fn container_of(_arg: vcpu, vcpu_vmx: struct, _arg: vcpu) -> return;
}
extern "C" {
    pub fn intel_pmu_cross_mapped_check(pmu: *mut kvm_pmu);
}
extern "C" {
    pub fn intel_pmu_create_guest_lbr_event(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn vmx_passthrough_lbr_msrs(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn free_vmcs(vmcs: *mut vmcs);
}
extern "C" {
    pub fn alloc_loaded_vmcs(loaded_vmcs: *mut loaded_vmcs) -> c_int;
}
extern "C" {
    pub fn free_loaded_vmcs(loaded_vmcs: *mut loaded_vmcs);
}
extern "C" {
    pub fn __vmx_guest_state_valid(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn is_unrestricted_guest(__vmx_guest_state_valid(vcpu: vcpu) ||) -> return;
}
extern "C" {
    pub fn dump_vmcs(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vmx_init() -> c_int;
}
extern "C" {
    pub fn vmx_exit();
}
