//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kvm/vcpu_fp.c
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
// Copyright (C) 2021 Western Digital Corporation or its affiliates.
//
// Authors:
// Atish Patra <atish.patra@wdc.com>
// Anup Patel <anup.patel@wdc.com>
//

#[no_mangle]
pub unsafe extern "C" fn kvm_riscv_vcpu_fp_reset(vcpu: *mut kvm_vcpu) {
    void kvm_riscv_vcpu_fp_reset(struct kvm_vcpu *vcpu)
    {
    struct kvm_cpu_context *cntx = &vcpu.arch.guest_context;
    cntx.sstatus &= ~SR_FS;
    if (riscv_isa_extension_available(vcpu.arch.isa, F) ||
    riscv_isa_extension_available(vcpu.arch.isa, D))
    cntx.sstatus |= SR_FS_INITIAL;
    else
    cntx.sstatus |= SR_FS_OFF;
    }
#[no_mangle]
unsafe extern "C" fn kvm_riscv_vcpu_fp_clean(cntx: *mut kvm_cpu_context) {
    static void kvm_riscv_vcpu_fp_clean(struct kvm_cpu_context *cntx)
    {
    cntx.sstatus &= ~SR_FS;
    cntx.sstatus |= SR_FS_CLEAN;
    }
    void kvm_riscv_vcpu_guest_fp_save(struct kvm_cpu_context *cntx,
    const unsigned long *isa)
    {
    if ((cntx.sstatus & SR_FS) == SR_FS_DIRTY) {
    if (riscv_isa_extension_available(isa, D))
    __kvm_riscv_fp_d_save(cntx);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: riscv_isa_extension_available(isa, _arg: F)) -> else {
    else if (riscv_isa_extension_available(isa, F))
    __kvm_riscv_fp_f_save(cntx);
    kvm_riscv_vcpu_fp_clean(cntx);
    }
    }
    void kvm_riscv_vcpu_guest_fp_restore(struct kvm_cpu_context *cntx,
    const unsigned long *isa)
    {
    if ((cntx.sstatus & SR_FS) != SR_FS_OFF) {
    if (riscv_isa_extension_available(isa, D))
    __kvm_riscv_fp_d_restore(cntx);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: riscv_isa_extension_available(isa, _arg: F)) -> else {
    else if (riscv_isa_extension_available(isa, F))
    __kvm_riscv_fp_f_restore(cntx);
    kvm_riscv_vcpu_fp_clean(cntx);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_riscv_vcpu_host_fp_save(cntx: *mut kvm_cpu_context) {
    void kvm_riscv_vcpu_host_fp_save(struct kvm_cpu_context *cntx)
    {
// No need to check host sstatus as it can be modified outside
    if (!kvm_riscv_isa_check_host(D))
    __kvm_riscv_fp_d_save(cntx);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !kvm_riscv_isa_check_host(F)) -> else {
    else if (!kvm_riscv_isa_check_host(F))
    __kvm_riscv_fp_f_save(cntx);
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_riscv_vcpu_host_fp_restore(cntx: *mut kvm_cpu_context) {
    void kvm_riscv_vcpu_host_fp_restore(struct kvm_cpu_context *cntx)
    {
    if (!kvm_riscv_isa_check_host(D))
    __kvm_riscv_fp_d_restore(cntx);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !kvm_riscv_isa_check_host(F)) -> else {
    else if (!kvm_riscv_isa_check_host(F))
    __kvm_riscv_fp_f_restore(cntx);
    }

    int kvm_riscv_vcpu_get_reg_fp(struct kvm_vcpu *vcpu,
    const struct kvm_one_reg *reg,
    unsigned long rtype)
    {
    struct kvm_cpu_context *cntx = &vcpu.arch.guest_context;
    unsigned long __user *uaddr =
    (unsigned long __user *)(unsigned long)reg.addr;
    unsigned long reg_num = reg.id & ~(KVM_REG_ARCH_MASK |
    KVM_REG_SIZE_MASK |
    rtype);
    void *reg_val;
    if ((rtype == KVM_REG_RISCV_FP_F) &&
    riscv_isa_extension_available(vcpu.arch.isa, F)) {
    if (KVM_REG_SIZE(reg.id) != sizeof(u32))
    return -EINVAL;
    if (reg_num == KVM_REG_RISCV_FP_F_REG(fcsr))
    reg_val = &cntx.fp.f.fcsr;
    else if ((KVM_REG_RISCV_FP_F_REG(f[0]) <= reg_num) &&
    reg_num <= KVM_REG_RISCV_FP_F_REG(f[31])) {
    reg_num = array_index_nospec(reg_num,
    ARRAY_SIZE(cntx.fp.f.f));
    reg_val = &cntx.fp.f.f[reg_num];
    } else
    return -ENOENT;
    } else if ((rtype == KVM_REG_RISCV_FP_D) &&
    riscv_isa_extension_available(vcpu.arch.isa, D)) {
    if (reg_num == KVM_REG_RISCV_FP_D_REG(fcsr)) {
    if (KVM_REG_SIZE(reg.id) != sizeof(u32))
    return -EINVAL;
    reg_val = &cntx.fp.d.fcsr;
    } else if ((KVM_REG_RISCV_FP_D_REG(f[0]) <= reg_num) &&
    reg_num <= KVM_REG_RISCV_FP_D_REG(f[31])) {
    if (KVM_REG_SIZE(reg.id) != sizeof(u64))
    return -EINVAL;
    reg_num = array_index_nospec(reg_num,
    ARRAY_SIZE(cntx.fp.d.f));
    reg_val = &cntx.fp.d.f[reg_num];
    } else
    return -ENOENT;
    } else
    return -ENOENT;
    if (copy_to_user(uaddr, reg_val, KVM_REG_SIZE(reg.id)))
    return -EFAULT;
    return 0;
    }
    int kvm_riscv_vcpu_set_reg_fp(struct kvm_vcpu *vcpu,
    const struct kvm_one_reg *reg,
    unsigned long rtype)
    {
    struct kvm_cpu_context *cntx = &vcpu.arch.guest_context;
    unsigned long __user *uaddr =
    (unsigned long __user *)(unsigned long)reg.addr;
    unsigned long reg_num = reg.id & ~(KVM_REG_ARCH_MASK |
    KVM_REG_SIZE_MASK |
    rtype);
    void *reg_val;
    if ((rtype == KVM_REG_RISCV_FP_F) &&
    riscv_isa_extension_available(vcpu.arch.isa, F)) {
    if (KVM_REG_SIZE(reg.id) != sizeof(u32))
    return -EINVAL;
    if (reg_num == KVM_REG_RISCV_FP_F_REG(fcsr))
    reg_val = &cntx.fp.f.fcsr;
    else if ((KVM_REG_RISCV_FP_F_REG(f[0]) <= reg_num) &&
    reg_num <= KVM_REG_RISCV_FP_F_REG(f[31])) {
    reg_num = array_index_nospec(reg_num,
    ARRAY_SIZE(cntx.fp.f.f));
    reg_val = &cntx.fp.f.f[reg_num];
    } else
    return -ENOENT;
    } else if ((rtype == KVM_REG_RISCV_FP_D) &&
    riscv_isa_extension_available(vcpu.arch.isa, D)) {
    if (reg_num == KVM_REG_RISCV_FP_D_REG(fcsr)) {
    if (KVM_REG_SIZE(reg.id) != sizeof(u32))
    return -EINVAL;
    reg_val = &cntx.fp.d.fcsr;
    } else if ((KVM_REG_RISCV_FP_D_REG(f[0]) <= reg_num) &&
    reg_num <= KVM_REG_RISCV_FP_D_REG(f[31])) {
    if (KVM_REG_SIZE(reg.id) != sizeof(u64))
    return -EINVAL;
    reg_num = array_index_nospec(reg_num,
    ARRAY_SIZE(cntx.fp.d.f));
    reg_val = &cntx.fp.d.f[reg_num];
    } else
    return -ENOENT;
    } else
    return -ENOENT;
    if (copy_from_user(reg_val, uaddr, KVM_REG_SIZE(reg.id)))
    return -EFAULT;
    return 0;
    }
