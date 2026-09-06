//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/vmx/nested.h
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

//
// Status returned by nested_vmx_enter_non_root_mode():
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvmx_vmentry_status {
    NVMX_VMENTRY_SUCCESS,		/* Entered VMX non-root mode */
    NVMX_VMENTRY_VMFAIL,		/* Consistency check VMFail */
    NVMX_VMENTRY_VMEXIT,		/* Consistency check VMExit */
    NVMX_VMENTRY_KVM_INTERNAL_ERROR,/* KVM internal error */
}

extern "C" {
    pub fn vmx_leave_nested(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn nested_vmx_setup_ctls_msrs(vmcs_conf: *mut vmcs_config, ept_caps: u32);
}
extern "C" {
    pub fn nested_vmx_hardware_unsetup();
}
extern "C" {
    pub fn nested_vmx_hardware_setup(): *mut *mut int (exit_handlers[])(struct kvm_vcpu) -> __init int;
}
extern "C" {
    pub fn nested_vmx_set_vmcs_shadowing_bitmap();
}
extern "C" {
    pub fn nested_vmx_check_restored_vmcs12(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn nested_vmx_free_vcpu(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn nested_vmx_reflect_vmexit(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn nested_sync_vmcs12_to_shadow(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vmx_set_vmx_msr(vcpu: *mut kvm_vcpu, msr_index: u32, data: u64) -> c_int;
}
extern "C" {
    pub fn vmx_get_vmx_msr(msrs: *mut nested_vmx_msrs, msr_index: u32, pdata: *mut u64) -> c_int;
}
//
// Note: the same condition is checked against the state provided by userspace
// in vmx_set_nested_state; if it is satisfied, the nested state must include
// the VMCS12.
//
// 'hv_evmcs_vmptr' can also be EVMPTR_MAP_PENDING here
// return the page table to be shadowed - in our case, EPT12
//
// Return the cr0/4 value that a nested guest would read. This is a combination
// of L1's "real" cr0 used to run the guest (guest_cr0), and the bits shadowed
// by the L1 hypervisor (cr0_read_shadow).  KVM must emulate CPU behavior as
// the value+mask loaded into vmcs02 may not match the vmcs12 fields.
//
extern "C" {
    pub fn vmx_misc_cr3_count(_arg: to_vmx(vcpu)->nested.msrs.misc_low) -> return;
}
//
// Do the virtual VMX capability MSRs specify that L1 can use VMWRITE
// to modify any valid field of the VMCS, or are the VM-exit
// information fields read-only?
//
extern "C" {
    pub fn nested_cpu_has(_arg: vmcs12, _arg: CPU_BASED_MONITOR_TRAP_FLAG) -> return;
}
extern "C" {
    pub fn nested_cpu_has2(_arg: vmcs12, _arg: SECONDARY_EXEC_ENABLE_EPT) -> return;
}
extern "C" {
    pub fn nested_cpu_has2(_arg: vmcs12, _arg: SECONDARY_EXEC_ENABLE_XSAVES) -> return;
}
extern "C" {
    pub fn nested_cpu_has2(_arg: vmcs12, _arg: SECONDARY_EXEC_ENABLE_PML) -> return;
}
extern "C" {
    pub fn nested_cpu_has2(_arg: vmcs12, _arg: SECONDARY_EXEC_VIRTUALIZE_X2APIC_MODE) -> return;
}
extern "C" {
    pub fn nested_cpu_has2(_arg: vmcs12, _arg: SECONDARY_EXEC_ENABLE_VPID) -> return;
}
extern "C" {
    pub fn nested_cpu_has2(_arg: vmcs12, _arg: SECONDARY_EXEC_APIC_REGISTER_VIRT) -> return;
}
extern "C" {
    pub fn nested_cpu_has2(_arg: vmcs12, _arg: SECONDARY_EXEC_VIRTUAL_INTR_DELIVERY) -> return;
}
extern "C" {
    pub fn nested_cpu_has2(_arg: vmcs12, _arg: SECONDARY_EXEC_ENABLE_VMFUNC) -> return;
}
extern "C" {
    pub fn nested_cpu_has2(_arg: vmcs12, _arg: SECONDARY_EXEC_SHADOW_VMCS) -> return;
}
extern "C" {
    pub fn nested_cpu_has_nmi_exiting(_arg: get_vmcs12(vcpu)) -> return;
}
//
// In nested virtualization, check if L1 asked to exit on external interrupts.
// For most existing hypervisors, this will always return true.
//
extern "C" {
    pub fn nested_cpu_has2(_arg: vmcs12, _arg: SECONDARY_EXEC_ENCLS_EXITING) -> return;
}
//
// if fixed0[i] == 1: val[i] must be 1
// if fixed1[i] == 0: val[i] must be 0
//
extern "C" {
    pub fn fixed_bits_valid(_arg: val, _arg: fixed0, _arg: fixed1) -> return;
}
extern "C" {
    pub fn fixed_bits_valid(_arg: val, _arg: fixed0, _arg: fixed1) -> return;
}
// No difference in the restrictions on guest and host CR4 in VMX operation.

