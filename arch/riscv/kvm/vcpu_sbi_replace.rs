//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kvm/vcpu_sbi_replace.c
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
// Copyright (c) 2021 Western Digital Corporation or its affiliates.
//
// Authors:
// Atish Patra <atish.patra@wdc.com>
//

    static int kvm_sbi_ext_time_handler(struct kvm_vcpu *vcpu, struct kvm_run *run,
    struct kvm_vcpu_sbi_return *retdata)
    {
    struct kvm_cpu_context *cp = &vcpu.arch.guest_context;
    u64 next_cycle;
    if (cp.a6 != SBI_EXT_TIME_SET_TIMER) {
    retdata.err_val = SBI_ERR_NOT_SUPPORTED;
    return 0;
    }
    kvm_riscv_vcpu_pmu_incr_fw(vcpu, SBI_PMU_FW_SET_TIMER);

    next_cycle = ((u64)cp.a1 << 32) | (u64)cp.a0;

    next_cycle = (u64)cp.a0;

    kvm_riscv_vcpu_timer_next_event(vcpu, next_cycle);
    return 0;
    }
    const struct kvm_vcpu_sbi_extension vcpu_sbi_ext_time = {
    .extid_start = SBI_EXT_TIME,
    .extid_end = SBI_EXT_TIME,
    .handler = kvm_sbi_ext_time_handler,
    };
    static int kvm_sbi_ext_ipi_handler(struct kvm_vcpu *vcpu, struct kvm_run *run,
    struct kvm_vcpu_sbi_return *retdata)
    {
    let mut ret: c_int = 0;
    unsigned long i;
    struct kvm_vcpu *tmp;
    struct kvm_cpu_context *cp = &vcpu.arch.guest_context;
    let mut hmask: c_ulong = cp.a0;
    let mut hbase: c_ulong = cp.a1;
    let mut hart_bit: c_ulong = 0, sentmask = 0;
    if (cp.a6 != SBI_EXT_IPI_SEND_IPI) {
    retdata.err_val = SBI_ERR_NOT_SUPPORTED;
    return 0;
    }
    kvm_riscv_vcpu_pmu_incr_fw(vcpu, SBI_PMU_FW_IPI_SENT);
    kvm_for_each_vcpu(i, tmp, vcpu.kvm) {
    if (hbase != -1UL) {
    if (tmp.vcpu_id < hbase)
    continue;
    hart_bit = tmp.vcpu_id - hbase;
    if (hart_bit >= __riscv_xlen)
    goto done;
    if (!(hmask & (1UL << hart_bit)))
    continue;
    }
    ret = kvm_riscv_vcpu_set_interrupt(tmp, IRQ_VS_SOFT);
    if (ret < 0)
    break;
    sentmask |= 1UL << hart_bit;
    kvm_riscv_vcpu_pmu_incr_fw(tmp, SBI_PMU_FW_IPI_RCVD);
    }
    done:
    if (hbase != -1UL && (hmask ^ sentmask))
    retdata.err_val = SBI_ERR_INVALID_PARAM;
    return ret;
    }
    const struct kvm_vcpu_sbi_extension vcpu_sbi_ext_ipi = {
    .extid_start = SBI_EXT_IPI,
    .extid_end = SBI_EXT_IPI,
    .handler = kvm_sbi_ext_ipi_handler,
    };
    static int kvm_sbi_ext_rfence_handler(struct kvm_vcpu *vcpu, struct kvm_run *run,
    struct kvm_vcpu_sbi_return *retdata)
    {
    struct kvm_cpu_context *cp = &vcpu.arch.guest_context;
    let mut hmask: c_ulong = cp.a0;
    let mut hbase: c_ulong = cp.a1;
    let mut funcid: c_ulong = cp.a6;
    unsigned long vmid;
    switch (funcid) {
    case SBI_EXT_RFENCE_REMOTE_FENCE_I:
    kvm_riscv_fence_i(vcpu.kvm, hbase, hmask);
    kvm_riscv_vcpu_pmu_incr_fw(vcpu, SBI_PMU_FW_FENCE_I_SENT);
    break;
    case SBI_EXT_RFENCE_REMOTE_SFENCE_VMA:
    vmid = READ_ONCE(vcpu.kvm.arch.vmid.vmid);
    if ((cp.a2 == 0 && cp.a3 == 0) || cp.a3 == -1UL)
    kvm_riscv_hfence_vvma_all(vcpu.kvm, hbase, hmask, vmid);
    else
    kvm_riscv_hfence_vvma_gva(vcpu.kvm, hbase, hmask,
    cp.a2, cp.a3, PAGE_SHIFT, vmid);
    kvm_riscv_vcpu_pmu_incr_fw(vcpu, SBI_PMU_FW_HFENCE_VVMA_SENT);
    break;
    case SBI_EXT_RFENCE_REMOTE_SFENCE_VMA_ASID:
    vmid = READ_ONCE(vcpu.kvm.arch.vmid.vmid);
    if ((cp.a2 == 0 && cp.a3 == 0) || cp.a3 == -1UL)
    kvm_riscv_hfence_vvma_asid_all(vcpu.kvm, hbase, hmask,
    cp.a4, vmid);
    else
    kvm_riscv_hfence_vvma_asid_gva(vcpu.kvm, hbase, hmask, cp.a2,
    cp.a3, PAGE_SHIFT, cp.a4, vmid);
    kvm_riscv_vcpu_pmu_incr_fw(vcpu, SBI_PMU_FW_HFENCE_VVMA_ASID_SENT);
    break;
    case SBI_EXT_RFENCE_REMOTE_HFENCE_GVMA:
    case SBI_EXT_RFENCE_REMOTE_HFENCE_GVMA_VMID:
    case SBI_EXT_RFENCE_REMOTE_HFENCE_VVMA:
    case SBI_EXT_RFENCE_REMOTE_HFENCE_VVMA_ASID:
//
// Until nested virtualization is implemented, the
// SBI HFENCE calls should return not supported
// hence fallthrough.
//
    default:
    retdata.err_val = SBI_ERR_NOT_SUPPORTED;
    }
    return 0;
    }
    const struct kvm_vcpu_sbi_extension vcpu_sbi_ext_rfence = {
    .extid_start = SBI_EXT_RFENCE,
    .extid_end = SBI_EXT_RFENCE,
    .handler = kvm_sbi_ext_rfence_handler,
    };
    static int kvm_sbi_ext_srst_handler(struct kvm_vcpu *vcpu,
    struct kvm_run *run,
    struct kvm_vcpu_sbi_return *retdata)
    {
    struct kvm_cpu_context *cp = &vcpu.arch.guest_context;
    let mut funcid: c_ulong = cp.a6;
    let mut reason: u32 = cp.a1;
    let mut type: u32 = cp.a0;
    switch (funcid) {
    case SBI_EXT_SRST_RESET:
    switch (type) {
    case SBI_SRST_RESET_TYPE_SHUTDOWN:
    kvm_riscv_vcpu_sbi_system_reset(vcpu, run,
    KVM_SYSTEM_EVENT_SHUTDOWN,
    reason);
    retdata.uexit = true;
    break;
    case SBI_SRST_RESET_TYPE_COLD_REBOOT:
    case SBI_SRST_RESET_TYPE_WARM_REBOOT:
    kvm_riscv_vcpu_sbi_system_reset(vcpu, run,
    KVM_SYSTEM_EVENT_RESET,
    reason);
    retdata.uexit = true;
    break;
    default:
    retdata.err_val = SBI_ERR_NOT_SUPPORTED;
    }
    break;
    default:
    retdata.err_val = SBI_ERR_NOT_SUPPORTED;
    }
    return 0;
    }
    const struct kvm_vcpu_sbi_extension vcpu_sbi_ext_srst = {
    .extid_start = SBI_EXT_SRST,
    .extid_end = SBI_EXT_SRST,
    .handler = kvm_sbi_ext_srst_handler,
    };
