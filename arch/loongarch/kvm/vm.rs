//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/kvm/vm.c
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
// Copyright (C) 2020-2023 Loongson Technology Corporation Limited
//

    const struct kvm_stats_desc kvm_vm_stats_desc[] = {
    KVM_GENERIC_VM_STATS(),
    STATS_DESC_ICOUNTER(VM, pages),
    STATS_DESC_ICOUNTER(VM, hugepages),
    };
    const struct kvm_stats_header kvm_vm_stats_header = {
    .name_size = KVM_STATS_NAME_SIZE,
    .num_desc = ARRAY_SIZE(kvm_vm_stats_desc),
    .id_offset =  sizeof(struct kvm_stats_header),
    .desc_offset = sizeof(struct kvm_stats_header) + KVM_STATS_NAME_SIZE,
    .data_offset = sizeof(struct kvm_stats_header) + KVM_STATS_NAME_SIZE +
    sizeof(kvm_vm_stats_desc),
    };
#[no_mangle]
unsafe extern "C" fn kvm_vm_init_features(kvm: *mut kvm) {
    static void kvm_vm_init_features(struct kvm *kvm)
    {
    unsigned long val;
    if (cpu_has_lsx)
    kvm.arch.kvm_features |= BIT(KVM_LOONGARCH_VM_FEAT_LSX);
    if (cpu_has_lasx)
    kvm.arch.kvm_features |= BIT(KVM_LOONGARCH_VM_FEAT_LASX);
    if (cpu_has_lbt_x86)
    kvm.arch.kvm_features |= BIT(KVM_LOONGARCH_VM_FEAT_X86BT);
    if (cpu_has_lbt_arm)
    kvm.arch.kvm_features |= BIT(KVM_LOONGARCH_VM_FEAT_ARMBT);
    if (cpu_has_lbt_mips)
    kvm.arch.kvm_features |= BIT(KVM_LOONGARCH_VM_FEAT_MIPSBT);
    if (cpu_has_ptw)
    kvm.arch.kvm_features |= BIT(KVM_LOONGARCH_VM_FEAT_PTW);
    if (cpu_has_msgint)
    kvm.arch.kvm_features |= BIT(KVM_LOONGARCH_VM_FEAT_MSGINT);
    val = read_csr_gcfg();
    if (val & CSR_GCFG_GPMP)
    kvm.arch.kvm_features |= BIT(KVM_LOONGARCH_VM_FEAT_PMU);
// Enable all PV features by default
    kvm.arch.pv_features |= BIT(KVM_FEATURE_IPI);
    kvm.arch.kvm_features |= BIT(KVM_LOONGARCH_VM_FEAT_PV_IPI);
    if (kvm_pvtime_supported()) {
    kvm.arch.pv_features |= BIT(KVM_FEATURE_PREEMPT);
    kvm.arch.pv_features |= BIT(KVM_FEATURE_STEAL_TIME);
    kvm.arch.kvm_features |= BIT(KVM_LOONGARCH_VM_FEAT_PV_PREEMPT);
    kvm.arch.kvm_features |= BIT(KVM_LOONGARCH_VM_FEAT_PV_STEALTIME);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_arch_init_vm(kvm: *mut kvm, type: c_ulong) -> c_int {
    int kvm_arch_init_vm(struct kvm *kvm, unsigned long type)
    {
    int i;
// Allocate page table to map GPA -> RPA
    kvm.arch.pgd = kvm_pgd_alloc();
    if (!kvm.arch.pgd)
    return -ENOMEM;
    kvm.arch.phyid_map = kvzalloc_obj(struct kvm_phyid_map,
    GFP_KERNEL_ACCOUNT);
    if (!kvm.arch.phyid_map) {
    free_page((unsigned long)kvm.arch.pgd);
    kvm.arch.pgd = core::ptr::null_mut();
    return -ENOMEM;
    }
    spin_lock_init(&kvm.arch.phyid_map_lock);
    spin_lock_init(&kvm.arch.pv_setting_lock);
    kvm_init_vmcs(kvm);
    kvm_vm_init_features(kvm);
//
// cpu_vabits means user address space only (a half of total).
// GPA size of VM is the same with the size of user address space.
//
    kvm.arch.gpa_size = BIT(cpu_vabits);
    kvm.arch.root_level = CONFIG_PGTABLE_LEVELS - 1;
    kvm.arch.invalid_ptes[0] = 0;
    kvm.arch.invalid_ptes[1] = (unsigned long)invalid_pte_table;

    kvm.arch.invalid_ptes[2] = (unsigned long)invalid_pmd_table;

    kvm.arch.invalid_ptes[3] = (unsigned long)invalid_pud_table;

    for (i = 0; i <= kvm.arch.root_level; i++)
    kvm.arch.pte_shifts[i] = PAGE_SHIFT + i * (PAGE_SHIFT - 3);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_arch_destroy_vm(kvm: *mut kvm) {
    void kvm_arch_destroy_vm(struct kvm *kvm)
    {
    kvm_destroy_vcpus(kvm);
    free_page((unsigned long)kvm.arch.pgd);
    kvm.arch.pgd = core::ptr::null_mut();
    kvfree(kvm.arch.phyid_map);
    kvm.arch.phyid_map = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_vm_ioctl_check_extension(kvm: *mut kvm, ext: c_long) -> c_int {
    int kvm_vm_ioctl_check_extension(struct kvm *kvm, long ext)
    {
    int r;
    switch (ext) {
    case KVM_CAP_IRQCHIP:
    case KVM_CAP_ONE_REG:
    case KVM_CAP_ENABLE_CAP:
    case KVM_CAP_READONLY_MEM:
    case KVM_CAP_IMMEDIATE_EXIT:
    case KVM_CAP_IOEVENTFD:
    case KVM_CAP_MP_STATE:
    case KVM_CAP_SET_GUEST_DEBUG:
    case KVM_CAP_VCPU_ATTRIBUTES:
    r = 1;
    break;
    case KVM_CAP_NR_VCPUS:
    r = min_t(unsigned int, num_online_cpus(), KVM_MAX_VCPUS);
    break;
    case KVM_CAP_MAX_VCPUS:
    r = KVM_MAX_VCPUS;
    break;
    case KVM_CAP_MAX_VCPU_ID:
    r = KVM_MAX_VCPU_IDS;
    break;
    case KVM_CAP_NR_MEMSLOTS:
    r = KVM_USER_MEM_SLOTS;
    break;
    case KVM_CAP_STEAL_TIME:
    r = kvm_pvtime_supported();
    break;
    default:
    r = 0;
    break;
    }
    return r;
    }
#[no_mangle]
unsafe extern "C" fn kvm_vm_feature_has_attr(kvm: *mut kvm, attr: *mut kvm_device_attr) -> c_int {
    static int kvm_vm_feature_has_attr(struct kvm *kvm, struct kvm_device_attr *attr)
    {
    switch (attr.attr) {
    case KVM_LOONGARCH_VM_FEAT_LSX:
    case KVM_LOONGARCH_VM_FEAT_LASX:
    case KVM_LOONGARCH_VM_FEAT_X86BT:
    case KVM_LOONGARCH_VM_FEAT_ARMBT:
    case KVM_LOONGARCH_VM_FEAT_MIPSBT:
    case KVM_LOONGARCH_VM_FEAT_PTW:
    case KVM_LOONGARCH_VM_FEAT_MSGINT:
    case KVM_LOONGARCH_VM_FEAT_PMU:
    case KVM_LOONGARCH_VM_FEAT_PV_IPI:
    case KVM_LOONGARCH_VM_FEAT_PV_PREEMPT:
    case KVM_LOONGARCH_VM_FEAT_PV_STEALTIME:
    if (kvm_vm_support(&kvm.arch, attr.attr))
    return 0;
    return -ENXIO;
    default:
    return -ENXIO;
    }
    }
#[no_mangle]
unsafe extern "C" fn kvm_vm_has_attr(kvm: *mut kvm, attr: *mut kvm_device_attr) -> c_int {
    static int kvm_vm_has_attr(struct kvm *kvm, struct kvm_device_attr *attr)
    {
    switch (attr.group) {
    case KVM_LOONGARCH_VM_FEAT_CTRL:
    return kvm_vm_feature_has_attr(kvm, attr);
    default:
    return -ENXIO;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_arch_vm_ioctl(filp: *mut file, ioctl: c_uint, arg: c_ulong) -> c_int {
    int kvm_arch_vm_ioctl(struct file *filp, unsigned int ioctl, unsigned long arg)
    {
    void __user *argp = (void __user *)arg;
    struct kvm *kvm = filp.private_data;
    struct kvm_device_attr attr;
    switch (ioctl) {
    case KVM_CREATE_IRQCHIP:
    return 0;
    case KVM_HAS_DEVICE_ATTR:
    if (copy_from_user(&attr, argp, sizeof(attr)))
    return -EFAULT;
    return kvm_vm_has_attr(kvm, &attr);
    default:
    return -ENOIOCTLCMD;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_vm_ioctl_irq_line(kvm: *mut kvm, irq_event: *mut kvm_irq_level, line_status: bool) -> c_int {
    int kvm_vm_ioctl_irq_line(struct kvm *kvm, struct kvm_irq_level *irq_event, bool line_status)
    {
    if (!kvm_arch_irqchip_in_kernel(kvm))
    return -ENXIO;
    irq_event.status = kvm_set_irq(kvm, KVM_USERSPACE_IRQ_SOURCE_ID,
    irq_event.irq, irq_event.level, line_status);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_arch_irqchip_in_kernel(kvm: *mut kvm) -> bool {
    bool kvm_arch_irqchip_in_kernel(struct kvm *kvm)
    {
    return (kvm.arch.ipi && kvm.arch.eiointc && kvm.arch.pch_pic);
    }
