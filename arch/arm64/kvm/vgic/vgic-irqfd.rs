//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kvm/vgic/vgic-irqfd.c
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
// Copyright (C) 2015, 2016 ARM Ltd.
//

//
// vgic_irqfd_set_irq: inject the IRQ corresponding to the
// irqchip routing entry
//
// This is the entry point for irqfd IRQ injection
//
    static int vgic_irqfd_set_irq(struct kvm_kernel_irq_routing_entry *e,
    struct kvm *kvm, int irq_source_id,
    int level, bool line_status)
    {
    let mut spi_id: c_uint = e.irqchip.pin + VGIC_NR_PRIVATE_IRQS;
    int ret;
    if (!vgic_valid_spi(kvm, spi_id))
    return -EINVAL;
    ret = vgic_lazy_init(kvm);
    if (ret)
    return ret;
    return kvm_vgic_inject_irq(kvm, core::ptr::null_mut(), spi_id, level, core::ptr::null_mut());
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
    let mut r: c_int = -EINVAL;
    switch (ue.type) {
    case KVM_IRQ_ROUTING_IRQCHIP:
    e.set = vgic_irqfd_set_irq;
    e.irqchip.irqchip = ue.u.irqchip.irqchip;
    e.irqchip.pin = ue.u.irqchip.pin;
    if ((e.irqchip.pin >= KVM_IRQCHIP_NUM_PINS) ||
    (e.irqchip.irqchip >= KVM_NR_IRQCHIPS))
    goto out;
    break;
    case KVM_IRQ_ROUTING_MSI:
    e.set = kvm_set_msi;
    e.msi.address_lo = ue.u.msi.address_lo;
    e.msi.address_hi = ue.u.msi.address_hi;
    e.msi.data = ue.u.msi.data;
    e.msi.flags = ue.flags;
    e.msi.devid = ue.u.msi.devid;
    break;
    default:
    goto out;
    }
    r = 0;
    out:
    return r;
    }
    static void kvm_populate_msi(struct kvm_kernel_irq_routing_entry *e,
    struct kvm_msi *msi)
    {
    msi.address_lo = e.msi.address_lo;
    msi.address_hi = e.msi.address_hi;
    msi.data = e.msi.data;
    msi.flags = e.msi.flags;
    msi.devid = e.msi.devid;
    }
//
// kvm_set_msi: inject the MSI corresponding to the
// MSI routing entry
//
// This is the entry point for irqfd MSI injection
// and userspace MSI injection.
//
    int kvm_set_msi(struct kvm_kernel_irq_routing_entry *e,
    struct kvm *kvm, int irq_source_id,
    int level, bool line_status)
    {
    struct kvm_msi msi;
    if (!vgic_has_its(kvm))
    return -ENODEV;
    if (!level)
    return -1;
    kvm_populate_msi(e, &msi);
    return vgic_its_inject_msi(kvm, &msi);
    }
//
// kvm_arch_set_irq_inatomic: fast-path for irqfd injection
//
    int kvm_arch_set_irq_inatomic(struct kvm_kernel_irq_routing_entry *e,
    struct kvm *kvm, int irq_source_id, int level,
    bool line_status)
    {
    if (!level)
    return -EWOULDBLOCK;
    switch (e.type) {
    case KVM_IRQ_ROUTING_MSI: {
    struct kvm_msi msi;
    if (!vgic_has_its(kvm))
    break;
    kvm_populate_msi(e, &msi);
    return vgic_its_inject_cached_translation(kvm, &msi);
    }
    case KVM_IRQ_ROUTING_IRQCHIP:
//
// Injecting SPIs is always possible in atomic context
// as long as the damn vgic is initialized.
//
    if (unlikely(!vgic_initialized(kvm)))
    break;
    return vgic_irqfd_set_irq(e, kvm, irq_source_id, 1, line_status);
    }
    return -EWOULDBLOCK;
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_vgic_setup_default_irq_routing(kvm: *mut kvm) -> c_int {
    int kvm_vgic_setup_default_irq_routing(struct kvm *kvm)
    {
    struct kvm_irq_routing_entry *entries;
    struct vgic_dist *dist = &kvm.arch.vgic;
    let mut nr: u32 = dist.nr_spis;
    int i, ret;
    entries = kzalloc_objs(*entries, nr, GFP_KERNEL_ACCOUNT);
    if (!entries)
    return -ENOMEM;
    for (i = 0; i < nr; i++) {
    entries[i].gsi = i;
    entries[i].type = KVM_IRQ_ROUTING_IRQCHIP;
    entries[i].u.irqchip.irqchip = 0;
    entries[i].u.irqchip.pin = i;
    }
    ret = kvm_set_irq_routing(kvm, entries, nr, 0);
    kfree(entries);
    return ret;
    }
