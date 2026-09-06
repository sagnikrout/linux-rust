//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kvm/vcpu_sbi_pmu.c
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
// Copyright (c) 2023 Rivos Inc
//
// Authors:
// Atish Patra <atishp@rivosinc.com>
//

    static int kvm_sbi_ext_pmu_handler(struct kvm_vcpu *vcpu, struct kvm_run *run,
    struct kvm_vcpu_sbi_return *retdata)
    {
    let mut ret: c_int = 0;
    struct kvm_cpu_context *cp = &vcpu.arch.guest_context;
    struct kvm_pmu *kvpmu = vcpu_to_pmu(vcpu);
    let mut funcid: c_ulong = cp.a6;
    u64 temp;
    if (!kvpmu.init_done) {
    retdata.err_val = SBI_ERR_NOT_SUPPORTED;
    return 0;
    }
    switch (funcid) {
    case SBI_EXT_PMU_NUM_COUNTERS:
    ret = kvm_riscv_vcpu_pmu_num_ctrs(vcpu, retdata);
    break;
    case SBI_EXT_PMU_COUNTER_GET_INFO:
    ret = kvm_riscv_vcpu_pmu_ctr_info(vcpu, cp.a0, retdata);
    break;
    case SBI_EXT_PMU_COUNTER_CFG_MATCH:

    temp = ((uint64_t)cp.a5 << 32) | cp.a4;

    temp = cp.a4;

//
// This can fail if perf core framework fails to create an event.
// No need to forward the error to userspace and exit the guest.
// The operation can continue without profiling. Forward the
// appropriate SBI error to the guest.
//
    ret = kvm_riscv_vcpu_pmu_ctr_cfg_match(vcpu, cp.a0, cp.a1,
    cp.a2, cp.a3, temp, retdata);
    break;
    case SBI_EXT_PMU_COUNTER_START:

    temp = ((uint64_t)cp.a4 << 32) | cp.a3;

    temp = cp.a3;

    ret = kvm_riscv_vcpu_pmu_ctr_start(vcpu, cp.a0, cp.a1, cp.a2,
    temp, retdata);
    break;
    case SBI_EXT_PMU_COUNTER_STOP:
    ret = kvm_riscv_vcpu_pmu_ctr_stop(vcpu, cp.a0, cp.a1, cp.a2, retdata);
    break;
    case SBI_EXT_PMU_COUNTER_FW_READ:
    ret = kvm_riscv_vcpu_pmu_fw_ctr_read(vcpu, cp.a0, retdata);
    break;
    case SBI_EXT_PMU_COUNTER_FW_READ_HI:
    if (IS_ENABLED(CONFIG_32BIT))
    ret = kvm_riscv_vcpu_pmu_fw_ctr_read_hi(vcpu, cp.a0, retdata);
    else
    retdata.out_val = 0;
    break;
    case SBI_EXT_PMU_SNAPSHOT_SET_SHMEM:
    ret = kvm_riscv_vcpu_pmu_snapshot_set_shmem(vcpu, cp.a0, cp.a1, cp.a2, retdata);
    break;
    case SBI_EXT_PMU_EVENT_GET_INFO:
    ret = kvm_riscv_vcpu_pmu_event_info(vcpu, cp.a0, cp.a1, cp.a2, cp.a3, retdata);
    break;
    default:
    retdata.err_val = SBI_ERR_NOT_SUPPORTED;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn kvm_sbi_ext_pmu_probe(vcpu: *mut kvm_vcpu) -> c_ulong {
    static unsigned long kvm_sbi_ext_pmu_probe(struct kvm_vcpu *vcpu)
    {
    struct kvm_pmu *kvpmu = vcpu_to_pmu(vcpu);
    return kvpmu.init_done;
    }
    const struct kvm_vcpu_sbi_extension vcpu_sbi_ext_pmu = {
    .extid_start = SBI_EXT_PMU,
    .extid_end = SBI_EXT_PMU,
    .handler = kvm_sbi_ext_pmu_handler,
    .probe = kvm_sbi_ext_pmu_probe,
    };
