//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/kvm/irqfd.c
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
// Copyright (C) 2024 Loongson Technology Corporation Limited
//

    static int kvm_set_pic_irq(struct kvm_kernel_irq_routing_entry *e,
    struct kvm *kvm, int irq_source_id, int level, bool line_status)
    {
// PCH-PIC pin (0 ~ 64) <---> GSI (0 ~ 64)
    pch_pic_set_irq(kvm.arch.pch_pic, e.irqchip.pin, level);
    return 0;
    }
//
// kvm_set_msi: inject the MSI corresponding to the
// MSI routing entry
//
// This is the entry point for irqfd MSI injection
// and userspace MSI injection.
//
    int kvm_set_msi(struct kvm_kernel_irq_routing_entry *e,
    struct kvm *kvm, int irq_source_id, int level, bool line_status)
    {
    if (!level)
    return -1;
    return pch_msi_set_irq(kvm, e, level);
    }
//
// kvm_set_routing_entry: populate a kvm routing entry
// from a user routing entry
//
// @kvm: the VM this entry is applied to
// @e: kvm kernel routing entry handle
// @ue: user api routing entry handle
// return 0 on success, -EINVAL on errors.
//
    int kvm_set_routing_entry(struct kvm *kvm,
    struct kvm_kernel_irq_routing_entry *e,
    const struct kvm_irq_routing_entry *ue)
    {
    switch (ue.type) {
    case KVM_IRQ_ROUTING_IRQCHIP:
    e.set = kvm_set_pic_irq;
    e.irqchip.irqchip = ue.u.irqchip.irqchip;
    e.irqchip.pin = ue.u.irqchip.pin;
    if (e.irqchip.pin >= KVM_IRQCHIP_NUM_PINS ||
    e.irqchip.irqchip >= KVM_NR_IRQCHIPS)
    return -EINVAL;
    return 0;
    case KVM_IRQ_ROUTING_MSI:
    e.set = kvm_set_msi;
    e.msi.address_lo = ue.u.msi.address_lo;
    e.msi.address_hi = ue.u.msi.address_hi;
    e.msi.data = ue.u.msi.data;
    return 0;
    default:
    return -EINVAL;
    }
    }
    int kvm_arch_set_irq_inatomic(struct kvm_kernel_irq_routing_entry *e,
    struct kvm *kvm, int irq_source_id, int level, bool line_status)
    {
    if (!level)
    return -EWOULDBLOCK;
    switch (e.type) {
    case KVM_IRQ_ROUTING_IRQCHIP:
    pch_pic_set_irq(kvm.arch.pch_pic, e.irqchip.pin, level);
    return 0;
    case KVM_IRQ_ROUTING_MSI:
    return pch_msi_set_irq(kvm, e, level);
    default:
    return -EWOULDBLOCK;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_arch_intc_initialized(kvm: *mut kvm) -> bool {
    bool kvm_arch_intc_initialized(struct kvm *kvm)
    {
    return kvm_arch_irqchip_in_kernel(kvm);
    }
