//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/msrs.h
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
// The first...last VMX feature MSRs that are emulated by KVM.  This may or may
// not cover all known VMX MSRs, as KVM doesn't emulate an MSR until there's an
// associated feature that KVM supports for nested virtualization.
//

//
// KVM's internal, non-ABI indices for synthetic MSRs. The values themselves
// are arbitrary and have no meaning, the only requirement is that they don't
// conflict with "real" MSRs that KVM supports. Use values at the upper end
// of KVM's reserved paravirtual MSR range to minimize churn, i.e. these values
// will be usable until KVM exhausts its supply of paravirtual MSR indices.
//
pub const MSR_KVM_INTERNAL_GUEST_SSP: c_uint = 0x4b564dff;

extern "C" {
    pub fn kvm_init_msr_lists();
}
extern "C" {
    pub fn kvm_get_msr_index_list(user_msr_list: *mut kvm_msr_list __user) -> c_int;
}
extern "C" {
    pub fn kvm_get_feature_msr_index_list(user_msr_list: *mut kvm_msr_list __user) -> c_int;
}
extern "C" {
    pub fn kvm_get_feature_msrs(user_msrs: *mut kvm_msrs __user) -> c_int;
}
extern "C" {
    pub fn kvm_get_msrs(vcpu: *mut kvm_vcpu, user_msrs: *mut kvm_msrs __user) -> c_int;
}
extern "C" {
    pub fn kvm_set_msrs(vcpu: *mut kvm_vcpu, user_msrs: *mut kvm_msrs __user) -> c_int;
}
extern "C" {
    pub fn kvm_valid_efer(vcpu: *mut kvm_vcpu, efer: u64) -> bool;
}
extern "C" {
    pub fn kvm_emulate_msr_read(vcpu: *mut kvm_vcpu, index: u32, data: *mut u64) -> c_int;
}
extern "C" {
    pub fn kvm_emulate_msr_write(vcpu: *mut kvm_vcpu, index: u32, data: u64) -> c_int;
}
extern "C" {
    pub fn __kvm_emulate_msr_read(vcpu: *mut kvm_vcpu, index: u32, data: *mut u64) -> c_int;
}
extern "C" {
    pub fn __kvm_emulate_msr_write(vcpu: *mut kvm_vcpu, index: u32, data: u64) -> c_int;
}
extern "C" {
    pub fn kvm_msr_read(vcpu: *mut kvm_vcpu, index: u32, data: *mut u64) -> c_int;
}
extern "C" {
    pub fn kvm_msr_write(vcpu: *mut kvm_vcpu, index: u32, data: u64) -> c_int;
}
extern "C" {
    pub fn kvm_emulate_rdmsr(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_emulate_rdmsr_imm(vcpu: *mut kvm_vcpu, msr: u32, reg: c_int) -> c_int;
}
extern "C" {
    pub fn kvm_emulate_wrmsr(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_emulate_wrmsr_imm(vcpu: *mut kvm_vcpu, msr: u32, reg: c_int) -> c_int;
}
extern "C" {
    pub fn handle_fastpath_wrmsr(vcpu: *mut kvm_vcpu) -> fastpath_t;
}
extern "C" {
    pub fn handle_fastpath_wrmsr_imm(vcpu: *mut kvm_vcpu, msr: u32, reg: c_int) -> fastpath_t;
}
extern "C" {
    pub fn kvm_get_msr_common(vcpu: *mut kvm_vcpu, msr: *mut msr_data) -> c_int;
}
extern "C" {
    pub fn kvm_set_msr_common(vcpu: *mut kvm_vcpu, msr: *mut msr_data) -> c_int;
}
extern "C" {
    pub fn kvm_add_user_return_msr(msr: u32) -> c_int;
}
extern "C" {
    pub fn kvm_find_user_return_msr(msr: u32) -> c_int;
}
extern "C" {
    pub fn kvm_set_user_return_msr(index: unsigned, val: u64, mask: u64) -> c_int;
}
extern "C" {
    pub fn kvm_get_user_return_msr(slot: c_uint) -> u64;
}
extern "C" {
    pub fn kvm_user_return_msr_cpu_online();
}
extern "C" {
    pub fn drop_user_return_notifiers();
}
extern "C" {
    pub fn kvm_destroy_user_return_msrs();
}
extern "C" {
    pub fn kvm_emulator_get_msr(vcpu: *mut kvm_vcpu, msr_index: u32, pdata: *mut u64) -> c_int;
}
extern "C" {
    pub fn kvm_msr_allowed(vcpu: *mut kvm_vcpu, index: u32, type: u32) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvm_msr_access {
    MSR_TYPE_R	= BIT(0),
    MSR_TYPE_W	= BIT(1),
    MSR_TYPE_RW	= MSR_TYPE_R | MSR_TYPE_W,
}

//
// Internal error codes that are used to indicate that MSR emulation encountered
// an error that should result in #GP in the guest, unless userspace handles it.
// Note, '1', '0', and negative numbers are off limits, as they are used by KVM
// as part of KVM's lightly documented internal KVM_RUN return codes.
//
// UNSUPPORTED	- The MSR isn't supported, either because it is completely
// unknown to KVM, or because the MSR should not exist according
// to the vCPU model.
//
// FILTERED	- Access to the MSR is denied by a userspace MSR filter.
//
pub const KVM_MSR_RET_UNSUPPORTED: c_int = 2;
pub const KVM_MSR_RET_FILTERED: c_int = 3;
extern "C" {
    pub fn kvm_vm_ioctl_set_msr_filter(kvm: *mut kvm, filter: *mut kvm_msr_filter) -> c_int;
}
extern "C" {
    pub fn kvm_free_msr_filter(msr_filter: *mut kvm_x86_msr_filter);
}
extern "C" {
    pub fn kvm_mtrr_set_msr(vcpu: *mut kvm_vcpu, msr: u32, data: u64) -> c_int;
}
extern "C" {
    pub fn kvm_mtrr_get_msr(vcpu: *mut kvm_vcpu, msr: u32, pdata: *mut u64) -> c_int;
}
extern "C" {
    pub fn kvm_get_arch_capabilities() -> u64;
}
extern "C" {
    pub fn kvm_spec_ctrl_test_value(value: u64) -> c_int;
}

// IBT can be suppressed iff the TRACKER isn't WAIT_ENDBR.
