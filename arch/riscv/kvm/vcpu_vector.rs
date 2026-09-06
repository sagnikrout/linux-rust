//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kvm/vcpu_vector.c
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
// Copyright (C) 2022 SiFive
//
// Authors:
// Vincent Chen <vincent.chen@sifive.com>
// Greentime Hu <greentime.hu@sifive.com>
//

#[no_mangle]
pub unsafe extern "C" fn kvm_riscv_vcpu_vector_reset(vcpu: *mut kvm_vcpu) {
    void kvm_riscv_vcpu_vector_reset(struct kvm_vcpu *vcpu)
    {
    unsigned long *isa = vcpu.arch.isa;
    struct kvm_cpu_context *cntx = &vcpu.arch.guest_context;
    cntx.sstatus &= ~SR_VS;
    cntx.vector.vlenb = riscv_v_vsize / 32;
    if (riscv_isa_extension_available(isa, V)) {
    cntx.sstatus |= SR_VS_INITIAL;
    WARN_ON(!cntx.vector.datap);
    memset(cntx.vector.datap, 0, riscv_v_vsize);
    } else {
    cntx.sstatus |= SR_VS_OFF;
    }
    }
#[no_mangle]
unsafe extern "C" fn kvm_riscv_vcpu_vector_clean(cntx: *mut kvm_cpu_context) {
    static void kvm_riscv_vcpu_vector_clean(struct kvm_cpu_context *cntx)
    {
    cntx.sstatus &= ~SR_VS;
    cntx.sstatus |= SR_VS_CLEAN;
    }
    void kvm_riscv_vcpu_guest_vector_save(struct kvm_cpu_context *cntx,
    unsigned long *isa)
    {
    if ((cntx.sstatus & SR_VS) == SR_VS_DIRTY) {
    if (riscv_isa_extension_available(isa, V))
    __kvm_riscv_vector_save(cntx);
    kvm_riscv_vcpu_vector_clean(cntx);
    }
    }
    void kvm_riscv_vcpu_guest_vector_restore(struct kvm_cpu_context *cntx,
    unsigned long *isa)
    {
    if ((cntx.sstatus & SR_VS) != SR_VS_OFF) {
    if (riscv_isa_extension_available(isa, V))
    riscv_v_flags_set(riscv_v_flags() | RISCV_V_VCPU_NEED_RESTORE);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_riscv_vcpu_host_vector_save(cntx: *mut kvm_cpu_context) {
    void kvm_riscv_vcpu_host_vector_save(struct kvm_cpu_context *cntx)
    {
// No need to check host sstatus as it can be modified outside
    if (!kvm_riscv_isa_check_host(V))
    __kvm_riscv_vector_save(cntx);
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_riscv_vcpu_host_vector_restore(cntx: *mut kvm_cpu_context) {
    void kvm_riscv_vcpu_host_vector_restore(struct kvm_cpu_context *cntx)
    {
    if (!kvm_riscv_isa_check_host(V))
    __kvm_riscv_vector_restore(cntx);
    riscv_v_flags_set(riscv_v_flags() & ~(RISCV_V_VCPU_CTX | RISCV_V_VCPU_NEED_RESTORE));
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_riscv_vcpu_alloc_vector_context(vcpu: *mut kvm_vcpu) -> c_int {
    int kvm_riscv_vcpu_alloc_vector_context(struct kvm_vcpu *vcpu)
    {
    vcpu.arch.guest_context.vector.datap = kzalloc(riscv_v_vsize, GFP_KERNEL_ACCOUNT);
    if (!vcpu.arch.guest_context.vector.datap)
    return -ENOMEM;
    vcpu.arch.host_context.vector.datap = kzalloc(riscv_v_vsize, GFP_KERNEL_ACCOUNT);
    if (!vcpu.arch.host_context.vector.datap) {
    kfree(vcpu.arch.guest_context.vector.datap);
    vcpu.arch.guest_context.vector.datap = core::ptr::null_mut();
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_riscv_vcpu_free_vector_context(vcpu: *mut kvm_vcpu) {
    void kvm_riscv_vcpu_free_vector_context(struct kvm_vcpu *vcpu)
    {
    kfree(vcpu.arch.guest_context.vector.datap);
    kfree(vcpu.arch.host_context.vector.datap);
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_riscv_vcpu_flush_vector() {
    void kvm_riscv_vcpu_flush_vector(void)
    {
    struct kvm_vcpu *vcpu = *this_cpu_ptr(kvm_get_running_vcpus());
//
// Only reached from __riscv_flush_vector_context() when RISCV_V_VCPU_CTX is set, which
// always have kvm_get_running_vcpus non-NULL.
//
    if (WARN_ON_ONCE(!vcpu))
    return;
    kvm_riscv_vcpu_guest_vector_save(&vcpu.arch.guest_context, vcpu.arch.isa);
    if ((vcpu.arch.guest_context.sstatus & SR_VS) != SR_VS_OFF)
    riscv_v_flags_set(riscv_v_flags() | RISCV_V_VCPU_NEED_RESTORE);
    }

    static int kvm_riscv_vcpu_vreg_addr(struct kvm_vcpu *vcpu,
    unsigned long reg_num,
    size_t reg_size,
    void **reg_addr)
    {
    struct kvm_cpu_context *cntx = &vcpu.arch.guest_context;
    let mut vlenb: usize = riscv_v_vsize / 32;
    if (reg_num < KVM_REG_RISCV_VECTOR_REG(0)) {
    if (reg_size != sizeof(unsigned long))
    return -EINVAL;
    switch (reg_num) {
    case KVM_REG_RISCV_VECTOR_CSR_REG(vstart):
// reg_addr = &cntx->vector.vstart;
    break;
    case KVM_REG_RISCV_VECTOR_CSR_REG(vl):
// reg_addr = &cntx->vector.vl;
    break;
    case KVM_REG_RISCV_VECTOR_CSR_REG(vtype):
// reg_addr = &cntx->vector.vtype;
    break;
    case KVM_REG_RISCV_VECTOR_CSR_REG(vcsr):
// reg_addr = &cntx->vector.vcsr;
    break;
    case KVM_REG_RISCV_VECTOR_CSR_REG(vlenb):
// reg_addr = &cntx->vector.vlenb;
    break;
    case KVM_REG_RISCV_VECTOR_CSR_REG(datap):
    default:
    return -ENOENT;
    }
    } else if (reg_num <= KVM_REG_RISCV_VECTOR_REG(31)) {
    unsigned long reg_offset;
    if (reg_size != vlenb)
    return -EINVAL;
    WARN_ON(!cntx.vector.datap);
//
// The reg_num is derived from the userspace-provided ONE_REG
// id. Sanitize it with array_index_nospec() to prevent
// speculative out-of-bounds access to the vector register
// buffer (32 vector registers: v0..v31).
//
    reg_offset = array_index_nospec(
    reg_num - KVM_REG_RISCV_VECTOR_REG(0), 32);
// reg_addr = cntx->vector.datap + reg_offset * vlenb;
    } else {
    return -ENOENT;
    }
    return 0;
    }
    int kvm_riscv_vcpu_get_reg_vector(struct kvm_vcpu *vcpu,
    const struct kvm_one_reg *reg)
    {
    unsigned long *isa = vcpu.arch.isa;
    unsigned long __user *uaddr =
    (unsigned long __user *)(unsigned long)reg.addr;
    unsigned long reg_num = reg.id & ~(KVM_REG_ARCH_MASK |
    KVM_REG_SIZE_MASK |
    KVM_REG_RISCV_VECTOR);
    let mut reg_size: usize = KVM_REG_SIZE(reg.id);
    void *reg_addr;
    int rc;
    if (!riscv_isa_extension_available(isa, V))
    return -ENOENT;
    rc = kvm_riscv_vcpu_vreg_addr(vcpu, reg_num, reg_size, &reg_addr);
    if (rc)
    return rc;
    if (copy_to_user(uaddr, reg_addr, reg_size))
    return -EFAULT;
    return 0;
    }
    int kvm_riscv_vcpu_set_reg_vector(struct kvm_vcpu *vcpu,
    const struct kvm_one_reg *reg)
    {
    unsigned long *isa = vcpu.arch.isa;
    unsigned long __user *uaddr =
    (unsigned long __user *)(unsigned long)reg.addr;
    unsigned long reg_num = reg.id & ~(KVM_REG_ARCH_MASK |
    KVM_REG_SIZE_MASK |
    KVM_REG_RISCV_VECTOR);
    let mut reg_size: usize = KVM_REG_SIZE(reg.id);
    void *reg_addr;
    int rc;
    if (!riscv_isa_extension_available(isa, V))
    return -ENOENT;
    if (reg_num == KVM_REG_RISCV_VECTOR_CSR_REG(vlenb)) {
    struct kvm_cpu_context *cntx = &vcpu.arch.guest_context;
    unsigned long reg_val;
    if (reg_size != sizeof(reg_val))
    return -EINVAL;
    if (copy_from_user(&reg_val, uaddr, reg_size))
    return -EFAULT;
    if (reg_val != cntx.vector.vlenb)
    return -EINVAL;
    return 0;
    }
    rc = kvm_riscv_vcpu_vreg_addr(vcpu, reg_num, reg_size, &reg_addr);
    if (rc)
    return rc;
    if (copy_from_user(reg_addr, uaddr, reg_size))
    return -EFAULT;
    return 0;
    }
