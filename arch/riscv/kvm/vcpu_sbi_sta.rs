//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kvm/vcpu_sbi_sta.c
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
// Copyright (c) 2023 Ventana Micro Systems Inc.
//

#[no_mangle]
unsafe extern "C" fn kvm_riscv_vcpu_sbi_sta_reset(vcpu: *mut kvm_vcpu) {
    static void kvm_riscv_vcpu_sbi_sta_reset(struct kvm_vcpu *vcpu)
    {
    vcpu.arch.sta.shmem = INVALID_GPA;
    vcpu.arch.sta.last_steal = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_riscv_vcpu_record_steal_time(vcpu: *mut kvm_vcpu) {
    void kvm_riscv_vcpu_record_steal_time(struct kvm_vcpu *vcpu)
    {
    let mut shmem: gpa_t = vcpu.arch.sta.shmem;
    let mut last_steal: u64 = vcpu.arch.sta.last_steal;
    __le32 __user *sequence_ptr;
    __le64 __user *steal_ptr;
    __le32 sequence_le;
    __le64 steal_le;
    u32 sequence;
    u64 steal;
    unsigned long hva;
    gfn_t gfn;
    if (shmem == INVALID_GPA)
    return;
//
// shmem is 64-byte aligned (see the enforcement in
// kvm_sbi_sta_steal_time_set_shmem()) and the size of sbi_sta_struct
// is 64 bytes, so we know all its offsets are in the same page.
//
    gfn = shmem >> PAGE_SHIFT;
    hva = kvm_vcpu_gfn_to_hva(vcpu, gfn);
    if (kvm_is_error_hva(hva)) {
    vcpu.arch.sta.shmem = INVALID_GPA;
    return;
    }
    sequence_ptr = (__le32 __user *)(hva + offset_in_page(shmem) +
    offsetof(struct sbi_sta_struct, sequence));
    steal_ptr = (__le64 __user *)(hva + offset_in_page(shmem) +
    offsetof(struct sbi_sta_struct, steal));
    if (WARN_ON(get_user(sequence_le, sequence_ptr)))
    return;
    sequence = le32_to_cpu(sequence_le);
    sequence += 1;
    if (WARN_ON(put_user(cpu_to_le32(sequence), sequence_ptr)))
    return;
    if (!WARN_ON(get_user(steal_le, steal_ptr))) {
    steal = le64_to_cpu(steal_le);
    vcpu.arch.sta.last_steal = READ_ONCE(current.sched_info.run_delay);
    steal += vcpu.arch.sta.last_steal - last_steal;
    WARN_ON(put_user(cpu_to_le64(steal), steal_ptr));
    }
    sequence += 1;
    WARN_ON(put_user(cpu_to_le32(sequence), sequence_ptr));
    kvm_vcpu_mark_page_dirty(vcpu, gfn);
    }
#[no_mangle]
unsafe extern "C" fn kvm_sbi_sta_steal_time_set_shmem(vcpu: *mut kvm_vcpu) -> c_int {
    static int kvm_sbi_sta_steal_time_set_shmem(struct kvm_vcpu *vcpu)
    {
    struct kvm_cpu_context *cp = &vcpu.arch.guest_context;
    let mut shmem_phys_lo: c_ulong = cp.a0;
    let mut shmem_phys_hi: c_ulong = cp.a1;
    let mut flags: u32 = cp.a2;
    let mut zero_sta: sbi_sta_struct = {0};
    gpa_t shmem;
    int ret;
    if (flags != 0)
    return SBI_ERR_INVALID_PARAM;
    if (shmem_phys_lo == SBI_SHMEM_DISABLE &&
    shmem_phys_hi == SBI_SHMEM_DISABLE) {
    vcpu.arch.sta.shmem = INVALID_GPA;
    return 0;
    }
    if (shmem_phys_lo & (SZ_64 - 1))
    return SBI_ERR_INVALID_PARAM;
    shmem = shmem_phys_lo;
    if (shmem_phys_hi != 0) {
    if (IS_ENABLED(CONFIG_32BIT))
    shmem |= ((gpa_t)shmem_phys_hi << 32);
    else
    return SBI_ERR_INVALID_ADDRESS;
    }
// No need to check writable slot explicitly as kvm_vcpu_write_guest does it internally
    ret = kvm_vcpu_write_guest(vcpu, shmem, &zero_sta, sizeof(zero_sta));
    if (ret)
    return SBI_ERR_INVALID_ADDRESS;
    vcpu.arch.sta.shmem = shmem;
    vcpu.arch.sta.last_steal = current.sched_info.run_delay;
    return 0;
    }
    static int kvm_sbi_ext_sta_handler(struct kvm_vcpu *vcpu, struct kvm_run *run,
    struct kvm_vcpu_sbi_return *retdata)
    {
    struct kvm_cpu_context *cp = &vcpu.arch.guest_context;
    let mut funcid: c_ulong = cp.a6;
    int ret;
    switch (funcid) {
    case SBI_EXT_STA_STEAL_TIME_SET_SHMEM:
    ret = kvm_sbi_sta_steal_time_set_shmem(vcpu);
    break;
    default:
    ret = SBI_ERR_NOT_SUPPORTED;
    break;
    }
    retdata.err_val = ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kvm_sbi_ext_sta_probe(vcpu: *mut kvm_vcpu) -> c_ulong {
    static unsigned long kvm_sbi_ext_sta_probe(struct kvm_vcpu *vcpu)
    {
    return !!sched_info_on();
    }
#[no_mangle]
unsafe extern "C" fn kvm_sbi_ext_sta_get_state_reg_count(vcpu: *mut kvm_vcpu) -> c_ulong {
    static unsigned long kvm_sbi_ext_sta_get_state_reg_count(struct kvm_vcpu *vcpu)
    {
    return sizeof(struct kvm_riscv_sbi_sta) / sizeof(unsigned long);
    }
    static int kvm_sbi_ext_sta_get_reg(struct kvm_vcpu *vcpu, unsigned long reg_num,
    unsigned long reg_size, void *reg_val)
    {
    unsigned long *value;
    if (reg_size != sizeof(unsigned long))
    return -EINVAL;
    value = reg_val;
    switch (reg_num) {
    case KVM_REG_RISCV_SBI_STA_REG(shmem_lo):
// value = (unsigned long)vcpu->arch.sta.shmem;
    break;
    case KVM_REG_RISCV_SBI_STA_REG(shmem_hi):
    if (IS_ENABLED(CONFIG_32BIT))
// value = upper_32_bits(vcpu->arch.sta.shmem);
    else
// value = 0;
    break;
    default:
    return -ENOENT;
    }
    return 0;
    }
    static int kvm_sbi_ext_sta_set_reg(struct kvm_vcpu *vcpu, unsigned long reg_num,
    unsigned long reg_size, const void *reg_val)
    {
    unsigned long value;
    let mut new_shmem: gpa_t = INVALID_GPA;
    if (reg_size != sizeof(unsigned long))
    return -EINVAL;
    value = *(const unsigned long *)reg_val;
    switch (reg_num) {
    case KVM_REG_RISCV_SBI_STA_REG(shmem_lo):
    if (IS_ENABLED(CONFIG_32BIT)) {
    let mut hi: gpa_t = upper_32_bits(vcpu.arch.sta.shmem);
    new_shmem = value;
    new_shmem |= hi << 32;
    } else {
    new_shmem = value;
    }
    break;
    case KVM_REG_RISCV_SBI_STA_REG(shmem_hi):
    if (IS_ENABLED(CONFIG_32BIT)) {
    let mut lo: gpa_t = lower_32_bits(vcpu.arch.sta.shmem);
    new_shmem = ((gpa_t)value << 32);
    new_shmem |= lo;
    } else if (value != 0) {
    return -EINVAL;
    }
    break;
    default:
    return -ENOENT;
    }
    if (new_shmem != INVALID_GPA && !IS_ALIGNED(new_shmem, 64))
    return -EINVAL;
    vcpu.arch.sta.shmem = new_shmem;
    return 0;
    }
    const struct kvm_vcpu_sbi_extension vcpu_sbi_ext_sta = {
    .extid_start = SBI_EXT_STA,
    .extid_end = SBI_EXT_STA,
    .handler = kvm_sbi_ext_sta_handler,
    .probe = kvm_sbi_ext_sta_probe,
    .reset = kvm_riscv_vcpu_sbi_sta_reset,
    .state_reg_subtype = KVM_REG_RISCV_SBI_STA,
    .get_state_reg_count = kvm_sbi_ext_sta_get_state_reg_count,
    .get_state_reg = kvm_sbi_ext_sta_get_reg,
    .set_state_reg = kvm_sbi_ext_sta_set_reg,
    };
