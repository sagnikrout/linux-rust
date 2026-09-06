//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/kvm/intc/dmsintc.c
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
// Copyright (C) 2025 Loongson Technology Corporation Limited
//

#[no_mangle]
pub unsafe extern "C" fn dmsintc_inject_irq(vcpu: *mut kvm_vcpu) {
    void dmsintc_inject_irq(struct kvm_vcpu *vcpu)
    {
    unsigned int i;
    unsigned long vector[4], old;
    struct dmsintc_state *ds = &vcpu.arch.dmsintc_state;
    if (!ds)
    return;
    for (i = 0; i < 4; i++) {
    old = atomic64_read(&(ds.vector_map[i]));
    vector[i] = old ? atomic64_xchg(&(ds.vector_map[i]), 0) : 0;
    }
    if (vector[0]) {
    old = kvm_read_hw_gcsr(LOONGARCH_CSR_ISR0);
    kvm_write_hw_gcsr(LOONGARCH_CSR_ISR0, vector[0] | old);
    }
    if (vector[1]) {
    old = kvm_read_hw_gcsr(LOONGARCH_CSR_ISR1);
    kvm_write_hw_gcsr(LOONGARCH_CSR_ISR1, vector[1] | old);
    }
    if (vector[2]) {
    old = kvm_read_hw_gcsr(LOONGARCH_CSR_ISR2);
    kvm_write_hw_gcsr(LOONGARCH_CSR_ISR2, vector[2] | old);
    }
    if (vector[3]) {
    old = kvm_read_hw_gcsr(LOONGARCH_CSR_ISR3);
    kvm_write_hw_gcsr(LOONGARCH_CSR_ISR3, vector[3] | old);
    }
    }
    int dmsintc_deliver_msi_to_vcpu(struct kvm *kvm,
    struct kvm_vcpu *vcpu, u32 vector, int level)
    {
    struct dmsintc_state *ds = &vcpu.arch.dmsintc_state;
    if (!level)
    return 0;
    if (!vcpu || vector >= 256)
    return -EINVAL;
    if (!ds)
    return -ENODEV;
    if (!kvm_guest_has_msgint(&vcpu.arch))
    return -EINVAL;
    set_bit(vector, (unsigned long *)&ds.vector_map);
    kvm_queue_irq(vcpu, INT_AVEC);
    kvm_vcpu_kick(vcpu);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dmsintc_set_irq(kvm: *mut kvm, addr: u64, data: c_int, level: c_int) -> c_int {
    int dmsintc_set_irq(struct kvm *kvm, u64 addr, int data, int level)
    {
    unsigned int irq, cpu;
    struct kvm_vcpu *vcpu;
    irq = (addr >> AVEC_IRQ_SHIFT) & AVEC_IRQ_MASK;
    cpu = (addr >> AVEC_CPU_SHIFT) & kvm.arch.dmsintc.cpu_mask;
    if (cpu >= KVM_MAX_VCPUS)
    return -EINVAL;
    vcpu = kvm_get_vcpu_by_cpuid(kvm, cpu);
    if (!vcpu)
    return -EINVAL;
    return dmsintc_deliver_msi_to_vcpu(kvm, vcpu, irq, level);
    }
    static int kvm_dmsintc_ctrl_access(struct kvm_device *dev,
    struct kvm_device_attr *attr, bool is_write)
    {
    let mut addr: c_int = attr.attr;
    unsigned long cpu_bit, val;
    void __user *data = (void __user *)attr.addr;
    struct loongarch_dmsintc *s = dev.kvm.arch.dmsintc;
    switch (addr) {
    case KVM_DEV_LOONGARCH_DMSINTC_MSG_ADDR_BASE:
    if (is_write) {
    if (copy_from_user(&val, data, sizeof(s.msg_addr_base)))
    return -EFAULT;
    if (s.msg_addr_base)
    return -EFAULT; /* Duplicate setting are not allowed. */
    if ((val & (BIT(AVEC_CPU_SHIFT) - 1)) != 0)
    return -EINVAL;
    s.msg_addr_base = val;
    cpu_bit = find_first_bit((unsigned long *)&(s.msg_addr_base), 64) - AVEC_CPU_SHIFT;
    cpu_bit = min(cpu_bit, AVEC_CPU_BIT);
    s.cpu_mask = GENMASK(cpu_bit - 1, 0) & AVEC_CPU_MASK;
    }
    break;
    case KVM_DEV_LOONGARCH_DMSINTC_MSG_ADDR_SIZE:
    if (is_write) {
    if (copy_from_user(&val, data, sizeof(s.msg_addr_size)))
    return -EFAULT;
    if (s.msg_addr_size)
    return -EFAULT; /*Duplicate setting are not allowed. */
    s.msg_addr_size = val;
    }
    break;
    default:
    kvm_pr_unimpl("%s: unknown dmsintc register, addr = %d\n", __func__, addr);
    return -ENXIO;
    }
    return 0;
    }
    static int kvm_dmsintc_set_attr(struct kvm_device *dev,
    struct kvm_device_attr *attr)
    {
    switch (attr.group) {
    case KVM_DEV_LOONGARCH_DMSINTC_GRP_CTRL:
    return kvm_dmsintc_ctrl_access(dev, attr, true);
    default:
    kvm_pr_unimpl("%s: unknown group (%d)\n", __func__, attr.group);
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn kvm_dmsintc_create(dev: *mut kvm_device, type: u32) -> c_int {
    static int kvm_dmsintc_create(struct kvm_device *dev, u32 type)
    {
    struct kvm *kvm;
    struct loongarch_dmsintc *s;
    if (!dev) {
    kvm_pr_unimpl("%s: kvm_device ptr is invalid!\n", __func__);
    return -EINVAL;
    }
    kvm = dev.kvm;
    if (kvm.arch.dmsintc) {
    kvm_pr_unimpl("%s: LoongArch DMSINTC has already been created!\n", __func__);
    return -EINVAL;
    }
    s = kzalloc_obj(struct loongarch_dmsintc);
    if (!s)
    return -ENOMEM;
    s.kvm = kvm;
    kvm.arch.dmsintc = s;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kvm_dmsintc_destroy(dev: *mut kvm_device) {
    static void kvm_dmsintc_destroy(struct kvm_device *dev)
    {
    if (!dev || !dev.kvm || !dev.kvm.arch.dmsintc)
    return;
    kfree(dev.kvm.arch.dmsintc);
    kfree(dev);
    }
    static struct kvm_device_ops kvm_dmsintc_dev_ops = {
    .name = "kvm-loongarch-dmsintc",
    .create = kvm_dmsintc_create,
    .destroy = kvm_dmsintc_destroy,
    .set_attr = kvm_dmsintc_set_attr,
    };
#[no_mangle]
pub unsafe extern "C" fn kvm_loongarch_register_dmsintc_device() -> c_int {
    int kvm_loongarch_register_dmsintc_device(void)
    {
    return kvm_register_device_ops(&kvm_dmsintc_dev_ops, KVM_DEV_TYPE_LOONGARCH_DMSINTC);
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_loongarch_unregister_dmsintc_device() {
    void kvm_loongarch_unregister_dmsintc_device(void)
    {
    kvm_unregister_device_ops(KVM_DEV_TYPE_LOONGARCH_DMSINTC);
    }
