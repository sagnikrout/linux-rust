//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/pmu.h
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

// retrieve a fixed counter bits out of IA32_FIXED_CTR_CTRL

pub const VMWARE_BACKDOOR_PMC_HOST_TSC: c_uint = 0x10000;
pub const VMWARE_BACKDOOR_PMC_REAL_TIME: c_uint = 0x10001;
pub const VMWARE_BACKDOOR_PMC_APPARENT_TIME: c_uint = 0x10002;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_pmu_ops {
    pub mask): *mut unsigned int idx, u64,
    pub msr): *mut *mut *mut *mut kvm_pmc (msr_idx_to_pmc)(kvm_vcpu vcpu, u32,
    pub idx): *mut *mut *mut int (check_rdpmc_early)(struct kvm_vcpu vcpu, unsigned int,
    pub msr): *mut *mut *mut bool (is_valid_msr)(struct kvm_vcpu vcpu, u32,
    pub msr_info): *mut *mut *mut int (get_msr)(struct kvm_vcpu vcpu, struct msr_data,
    pub msr_info): *mut *mut *mut int (set_msr)(struct kvm_vcpu vcpu, struct msr_data,
    pub vcpu): *mut *mut void (refresh)(struct kvm_vcpu,
    pub vcpu): *mut *mut void (init)(struct kvm_vcpu,
    pub vcpu): *mut *mut void (reset)(struct kvm_vcpu,
    pub vcpu): *mut *mut void (deliver_pmi)(struct kvm_vcpu,
    pub vcpu): *mut *mut void (cleanup)(struct kvm_vcpu,
    pub pmc): *mut *mut bool (pmc_is_disabled_in_current_mode)(struct kvm_pmc,
    pub host_pmu): *mut *mut bool (is_mediated_pmu_supported)(struct x86_pmu_capability,
    pub vcpu): *mut *mut void (mediated_load)(struct kvm_vcpu,
    pub vcpu): *mut *mut void (mediated_put)(struct kvm_vcpu,
    pub global_ctrl): *mut *mut void (write_global_ctrl)(u64,
    pub EVENTSEL_EVENT: u64,
    pub MAX_NR_GP_COUNTERS: c_int,
    pub MIN_NR_GP_COUNTERS: c_int,
    pub PERF_GLOBAL_CTRL: u32,
    pub GP_EVENTSEL_BASE: u32,
    pub GP_COUNTER_BASE: u32,
    pub FIXED_COUNTER_BASE: u32,
    pub MSR_STRIDE: u32,
}

extern "C" {
    pub fn kvm_pmu_ops_update(pmu_ops: *const kvm_pmu_ops);
}
extern "C" {
    pub fn kvm_handle_guest_mediated_pmi();
}
//
// Architecturally, Intel's SDM states that IA32_PERF_GLOBAL_CTRL is
// supported if "CPUID.0AH: EAX[7:0] > 0", i.e. if the PMU version is
// greater than zero.  However, KVM only exposes and emulates the MSR
// to/for the guest if the guest PMU supports at least "Architectural
// Performance Monitoring Version 2".
//
// AMD's version of PERF_GLOBAL_CTRL conveniently shows up with v2.
//
// KVM tracks all counters in 64-bit bitmaps, with general purpose counters
// mapped to bits 31:0 and fixed counters mapped to 63:32, e.g. fixed counter 0
// is tracked internally via index 32.  On Intel, (AMD doesn't support fixed
// counters), this mirrors how fixed counters are mapped to PERF_GLOBAL_CTRL
// and similar MSRs, i.e. tracking fixed counters at base index 32 reduces the
// amount of boilerplate needed to iterate over PMCs *and* simplifies common
// enable/disable/reset operations.
//
// WARNING!  This helper is only for lookups that are initiated by KVM, it is
// NOT safe for guest lookups, e.g. will do the wrong thing if passed a raw
// ECX value from RDPMC (fixed counters are accessed by setting bit 30 in ECX
// for RDPMC, not by adding 32 to the fixed counter index).
//

// FIXME: Scaling needed?
extern "C" {
    pub fn pmc_write_counter(pmc: *mut kvm_pmc, val: u64);
}
// returns general purpose PMC with the specified MSR. Note that it can be
// used for both PERFCTRn and EVNTSELn; that is why it accepts base as a
// parameter to tell them apart.
//
// returns fixed PMC with the specified MSR
extern "C" {
    pub fn kvm_init_pmu_capability(pmu_ops: *mut kvm_pmu_ops);
}
extern "C" {
    pub fn kvm_pmu_recalc_pmc_emulation(pmu: *mut kvm_pmu, pmc: *mut kvm_pmc);
}
extern "C" {
    pub fn kvm_pmu_handle_event(vcpu: *mut kvm_vcpu);
}
//
// Check if a PMC is enabled by comparing it against global_ctrl bits.
//
// If the vPMU doesn't have global_ctrl MSR, all vPMCs are enabled.
//
extern "C" {
    pub fn test_bit(_arg: pmc->idx, )&pmu->global_ctrl: *mut (unsigned long) -> return;
}
extern "C" {
    pub fn kvm_pmu_deliver_pmi(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_pmu_rdpmc(vcpu: *mut kvm_vcpu, pmc: unsigned, data: *mut u64) -> c_int;
}
extern "C" {
    pub fn kvm_pmu_check_rdpmc_early(vcpu: *mut kvm_vcpu, idx: c_uint) -> c_int;
}
extern "C" {
    pub fn kvm_pmu_is_valid_msr(vcpu: *mut kvm_vcpu, msr: u32) -> bool;
}
extern "C" {
    pub fn kvm_pmu_get_msr(vcpu: *mut kvm_vcpu, msr_info: *mut msr_data) -> c_int;
}
extern "C" {
    pub fn kvm_pmu_set_msr(vcpu: *mut kvm_vcpu, msr_info: *mut msr_data) -> c_int;
}
extern "C" {
    pub fn kvm_pmu_refresh(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_pmu_init(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_pmu_cleanup(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_pmu_destroy(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_vm_ioctl_set_pmu_event_filter(kvm: *mut kvm, argp: *mut void __user) -> c_int;
}
extern "C" {
    pub fn kvm_pmu_instruction_retired(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_pmu_branch_retired(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_mediated_pmu_load(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_mediated_pmu_put(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn is_vmware_backdoor_pmc(pmc_idx: u32) -> bool;
}
extern "C" {
    pub fn kvm_need_perf_global_ctrl_intercept(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn kvm_need_rdpmc_intercept(vcpu: *mut kvm_vcpu) -> bool;
}
