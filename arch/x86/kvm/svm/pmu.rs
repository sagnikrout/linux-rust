//! Automatically rewritten from C to Rust
//! Source: arch/x86/kvm/svm/pmu.c
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
// KVM PMU support for AMD
//
// Copyright 2015, Red Hat, Inc. and/or its affiliates.
//
// Author:
// Wei Huang <wei@redhat.com>
//
// Implementation is based on pmu_intel.c file
//

    enum pmu_type {
    PMU_TYPE_COUNTER = 0,
    PMU_TYPE_EVNTSEL,
    };
    static struct kvm_pmc *amd_pmu_get_pmc(struct kvm_pmu *pmu, int pmc_idx)
    {
    let mut num_counters: c_uint = pmu.nr_arch_gp_counters;
    if (pmc_idx >= num_counters)
    return core::ptr::null_mut();
    return &pmu.gp_counters[array_index_nospec(pmc_idx, num_counters)];
    }
    static inline struct kvm_pmc *get_gp_pmc_amd(struct kvm_pmu *pmu, u32 msr,
    enum pmu_type type)
    {
    struct kvm_vcpu *vcpu = pmu_to_vcpu(pmu);
    unsigned int idx;
    if (!pmu.version)
    return core::ptr::null_mut();
    switch (msr) {
    case MSR_F15H_PERF_CTL0 ... MSR_F15H_PERF_CTR5:
    if (!guest_cpu_cap_has(vcpu, X86_FEATURE_PERFCTR_CORE))
    return core::ptr::null_mut();
//
// Each PMU counter has a pair of CTL and CTR MSRs. CTLn
// MSRs (accessed via EVNTSEL) are even, CTRn MSRs are odd.
//
    idx = (unsigned int)((msr - MSR_F15H_PERF_CTL0) / 2);
    if (!(msr & 0x1) != (type == PMU_TYPE_EVNTSEL))
    return core::ptr::null_mut();
    break;
    case MSR_K7_EVNTSEL0 ... MSR_K7_EVNTSEL3:
    if (type != PMU_TYPE_EVNTSEL)
    return core::ptr::null_mut();
    idx = msr - MSR_K7_EVNTSEL0;
    break;
    case MSR_K7_PERFCTR0 ... MSR_K7_PERFCTR3:
    if (type != PMU_TYPE_COUNTER)
    return core::ptr::null_mut();
    idx = msr - MSR_K7_PERFCTR0;
    break;
    default:
    return core::ptr::null_mut();
    }
    return amd_pmu_get_pmc(pmu, idx);
    }
#[no_mangle]
unsafe extern "C" fn amd_check_rdpmc_early(vcpu: *mut kvm_vcpu, idx: c_uint) -> c_int {
    static int amd_check_rdpmc_early(struct kvm_vcpu *vcpu, unsigned int idx)
    {
    struct kvm_pmu *pmu = vcpu_to_pmu(vcpu);
    if (idx >= pmu.nr_arch_gp_counters)
    return -EINVAL;
    return 0;
    }
// idx is the ECX register of RDPMC instruction
    static struct kvm_pmc *amd_rdpmc_ecx_to_pmc(struct kvm_vcpu *vcpu,
    unsigned int idx, u64 *mask)
    {
    return amd_pmu_get_pmc(vcpu_to_pmu(vcpu), idx);
    }
    static struct kvm_pmc *amd_msr_idx_to_pmc(struct kvm_vcpu *vcpu, u32 msr)
    {
    struct kvm_pmu *pmu = vcpu_to_pmu(vcpu);
    struct kvm_pmc *pmc;
    pmc = get_gp_pmc_amd(pmu, msr, PMU_TYPE_COUNTER);
    pmc = pmc ? pmc : get_gp_pmc_amd(pmu, msr, PMU_TYPE_EVNTSEL);
    return pmc;
    }
#[no_mangle]
unsafe extern "C" fn amd_is_valid_msr(vcpu: *mut kvm_vcpu, msr: u32) -> bool {
    static bool amd_is_valid_msr(struct kvm_vcpu *vcpu, u32 msr)
    {
    struct kvm_pmu *pmu = vcpu_to_pmu(vcpu);
    switch (msr) {
    case MSR_K7_EVNTSEL0 ... MSR_K7_PERFCTR3:
    return pmu.version > 0;
    case MSR_F15H_PERF_CTL0 ... MSR_F15H_PERF_CTR5:
    return guest_cpu_cap_has(vcpu, X86_FEATURE_PERFCTR_CORE);
    case MSR_AMD64_PERF_CNTR_GLOBAL_STATUS:
    case MSR_AMD64_PERF_CNTR_GLOBAL_CTL:
    case MSR_AMD64_PERF_CNTR_GLOBAL_STATUS_CLR:
    case MSR_AMD64_PERF_CNTR_GLOBAL_STATUS_SET:
    return pmu.version > 1;
    default:
    if (msr > MSR_F15H_PERF_CTR5 &&
    msr < MSR_F15H_PERF_CTL0 + 2 * pmu.nr_arch_gp_counters)
    return pmu.version > 1;
    break;
    }
    return amd_msr_idx_to_pmc(vcpu, msr);
    }
#[no_mangle]
unsafe extern "C" fn amd_pmu_get_msr(vcpu: *mut kvm_vcpu, msr_info: *mut msr_data) -> c_int {
    static int amd_pmu_get_msr(struct kvm_vcpu *vcpu, struct msr_data *msr_info)
    {
    struct kvm_pmu *pmu = vcpu_to_pmu(vcpu);
    struct kvm_pmc *pmc;
    let mut msr: u32 = msr_info.index;
// MSR_PERFCTRn
    pmc = get_gp_pmc_amd(pmu, msr, PMU_TYPE_COUNTER);
    if (pmc) {
    msr_info.data = pmc_read_counter(pmc);
    return 0;
    }
// MSR_EVNTSELn
    pmc = get_gp_pmc_amd(pmu, msr, PMU_TYPE_EVNTSEL);
    if (pmc) {
    msr_info.data = pmc.eventsel;
    return 0;
    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn amd_pmu_set_msr(vcpu: *mut kvm_vcpu, msr_info: *mut msr_data) -> c_int {
    static int amd_pmu_set_msr(struct kvm_vcpu *vcpu, struct msr_data *msr_info)
    {
    struct kvm_pmu *pmu = vcpu_to_pmu(vcpu);
    struct kvm_pmc *pmc;
    let mut msr: u32 = msr_info.index;
    let mut data: u64 = msr_info.data;
// MSR_PERFCTRn
    pmc = get_gp_pmc_amd(pmu, msr, PMU_TYPE_COUNTER);
    if (pmc) {
    pmc_write_counter(pmc, data);
    return 0;
    }
// MSR_EVNTSELn
    pmc = get_gp_pmc_amd(pmu, msr, PMU_TYPE_EVNTSEL);
    if (pmc) {
    data &= ~pmu.reserved_bits;
    if (data != pmc.eventsel) {
    pmc.eventsel = data;
    pmc.eventsel_hw = (data & ~AMD64_EVENTSEL_HOSTONLY) |
    AMD64_EVENTSEL_GUESTONLY;
    if (data & AMD64_EVENTSEL_HOST_GUEST_MASK)
    __set_bit(pmc.idx, pmu.pmc_has_mode_specific_enables);
    else
    __clear_bit(pmc.idx, pmu.pmc_has_mode_specific_enables);
    kvm_pmu_request_counter_reprogram(pmc);
    }
    return 0;
    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn amd_pmu_refresh(vcpu: *mut kvm_vcpu) {
    static void amd_pmu_refresh(struct kvm_vcpu *vcpu)
    {
    struct kvm_pmu *pmu = vcpu_to_pmu(vcpu);
    union cpuid_0x80000022_ebx ebx;
    pmu.version = 1;
    if (guest_cpu_cap_has(vcpu, X86_FEATURE_PERFMON_V2)) {
    pmu.version = 2;
//
// Note, PERFMON_V2 is also in 0x80000022.0x0, i.e. the guest
// CPUID entry is guaranteed to be non-NULL.
//
    BUILD_BUG_ON(x86_feature_cpuid(X86_FEATURE_PERFMON_V2).function != 0x80000022 ||
    x86_feature_cpuid(X86_FEATURE_PERFMON_V2).index);
    ebx.full = kvm_find_cpuid_entry_index(vcpu, 0x80000022, 0).ebx;
    pmu.nr_arch_gp_counters = ebx.split.num_core_pmc;
    } else if (guest_cpu_cap_has(vcpu, X86_FEATURE_PERFCTR_CORE)) {
    pmu.nr_arch_gp_counters = AMD64_NUM_COUNTERS_CORE;
    } else {
    pmu.nr_arch_gp_counters = AMD64_NUM_COUNTERS;
    }
    pmu.nr_arch_gp_counters = min_t(unsigned int, pmu.nr_arch_gp_counters,
    kvm_pmu_cap.num_counters_gp);
    if (pmu.version > 1) {
    pmu.global_ctrl_rsvd = ~(BIT_ULL(pmu.nr_arch_gp_counters) - 1);
    pmu.global_status_rsvd = pmu.global_ctrl_rsvd;
    }
    pmu.counter_bitmask[KVM_PMC_GP] = BIT_ULL(48) - 1;
    pmu.reserved_bits = 0xfffffff000280000ull;
    if (guest_cpu_cap_has(vcpu, X86_FEATURE_SVM) && kvm_vcpu_has_mediated_pmu(vcpu))
    pmu.reserved_bits &= ~AMD64_EVENTSEL_HOST_GUEST_MASK;
    pmu.raw_event_mask = AMD64_RAW_EVENT_MASK;
// not applicable to AMD; but clean them to prevent any fall out
    pmu.counter_bitmask[KVM_PMC_FIXED] = 0;
    pmu.nr_arch_fixed_counters = 0;
    }
#[no_mangle]
unsafe extern "C" fn amd_pmu_init(vcpu: *mut kvm_vcpu) {
    static void amd_pmu_init(struct kvm_vcpu *vcpu)
    {
    struct kvm_pmu *pmu = vcpu_to_pmu(vcpu);
    int i;
    BUILD_BUG_ON(KVM_MAX_NR_AMD_GP_COUNTERS > AMD64_NUM_COUNTERS_CORE);
    for (i = 0; i < KVM_MAX_NR_AMD_GP_COUNTERS; i++) {
    pmu.gp_counters[i].type = KVM_PMC_GP;
    pmu.gp_counters[i].vcpu = vcpu;
    pmu.gp_counters[i].idx = i;
    pmu.gp_counters[i].current_config = 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn amd_pmu_is_mediated_pmu_supported(host_pmu: *mut x86_pmu_capability) -> bool {
    static bool amd_pmu_is_mediated_pmu_supported(struct x86_pmu_capability *host_pmu)
    {
    return host_pmu.version >= 2;
    }
#[no_mangle]
unsafe extern "C" fn amd_mediated_pmu_load(vcpu: *mut kvm_vcpu) {
    static void amd_mediated_pmu_load(struct kvm_vcpu *vcpu)
    {
    struct kvm_pmu *pmu = vcpu_to_pmu(vcpu);
    u64 global_status;
    rdmsrq(MSR_AMD64_PERF_CNTR_GLOBAL_STATUS, global_status);
// Clear host global_status MSR if non-zero.
    if (global_status)
    wrmsrq(MSR_AMD64_PERF_CNTR_GLOBAL_STATUS_CLR, global_status);
    wrmsrq(MSR_AMD64_PERF_CNTR_GLOBAL_STATUS_SET, pmu.global_status);
    wrmsrq(MSR_AMD64_PERF_CNTR_GLOBAL_CTL, pmu.global_ctrl);
    }
#[no_mangle]
unsafe extern "C" fn amd_mediated_pmu_put(vcpu: *mut kvm_vcpu) {
    static void amd_mediated_pmu_put(struct kvm_vcpu *vcpu)
    {
    struct kvm_pmu *pmu = vcpu_to_pmu(vcpu);
    wrmsrq(MSR_AMD64_PERF_CNTR_GLOBAL_CTL, 0);
    rdmsrq(MSR_AMD64_PERF_CNTR_GLOBAL_STATUS, pmu.global_status);
// Clear global status bits if non-zero
    if (pmu.global_status)
    wrmsrq(MSR_AMD64_PERF_CNTR_GLOBAL_STATUS_CLR, pmu.global_status);
    }
#[no_mangle]
unsafe extern "C" fn amd_pmc_is_disabled_in_current_mode(pmc: *mut kvm_pmc) -> bool {
    static bool amd_pmc_is_disabled_in_current_mode(struct kvm_pmc *pmc)
    {
    struct kvm_vcpu *vcpu = pmc.vcpu;
    u64 host_guest_bits;
    if (!kvm_vcpu_has_mediated_pmu(vcpu))
    return false;
// Common code is supposed to check the common enable bit
    if (WARN_ON_ONCE(!(pmc.eventsel & ARCH_PERFMON_EVENTSEL_ENABLE)))
    return false;
// If both bits are cleared, the counter is always enabled
    host_guest_bits = pmc.eventsel & AMD64_EVENTSEL_HOST_GUEST_MASK;
    if (!host_guest_bits)
    return false;
// If EFER.SVME=0 and either bit is set, the counter is disabled
    if (!(vcpu.arch.efer & EFER_SVME))
    return true;
//
// If EFER.SVME=1, the counter is disabled iff only one of the bits is
// set AND the set bit doesn't match the vCPU mode.
//
    if (host_guest_bits == AMD64_EVENTSEL_HOST_GUEST_MASK)
    return false;
    return !!(host_guest_bits & AMD64_EVENTSEL_GUESTONLY) != is_guest_mode(vcpu);
    }
    struct kvm_pmu_ops amd_pmu_ops __initdata = {
    .rdpmc_ecx_to_pmc = amd_rdpmc_ecx_to_pmc,
    .msr_idx_to_pmc = amd_msr_idx_to_pmc,
    .check_rdpmc_early = amd_check_rdpmc_early,
    .is_valid_msr = amd_is_valid_msr,
    .get_msr = amd_pmu_get_msr,
    .set_msr = amd_pmu_set_msr,
    .refresh = amd_pmu_refresh,
    .init = amd_pmu_init,
    .pmc_is_disabled_in_current_mode = amd_pmc_is_disabled_in_current_mode,
    .is_mediated_pmu_supported = amd_pmu_is_mediated_pmu_supported,
    .mediated_load = amd_mediated_pmu_load,
    .mediated_put = amd_mediated_pmu_put,
    .EVENTSEL_EVENT = AMD64_EVENTSEL_EVENT,
    .MAX_NR_GP_COUNTERS = KVM_MAX_NR_AMD_GP_COUNTERS,
    .MIN_NR_GP_COUNTERS = AMD64_NUM_COUNTERS,
    .PERF_GLOBAL_CTRL = MSR_AMD64_PERF_CNTR_GLOBAL_CTL,
    .GP_EVENTSEL_BASE = MSR_F15H_PERF_CTL0,
    .GP_COUNTER_BASE = MSR_F15H_PERF_CTR0,
    .FIXED_COUNTER_BASE = 0,
    .MSR_STRIDE = 2,
    };
