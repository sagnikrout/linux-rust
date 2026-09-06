//! Automatically rewritten from C to Rust
//! Source: drivers/dca/dca-core.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright(c) 2007 - 2009 Intel Corporation. All rights reserved.
//
// This driver supports an interface for DCA clients and providers to meet.
//

    MODULE_VERSION(DCA_VERSION);
    MODULE_DESCRIPTION("Intel Direct Cache Access (DCA) service module");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Intel Corporation");
    static DEFINE_RAW_SPINLOCK(dca_lock);
    static LIST_HEAD(dca_domains);
    static BLOCKING_NOTIFIER_HEAD(dca_provider_chain);
    static int dca_providers_blocked;
    static struct pci_bus *dca_pci_rc_from_dev(struct device *dev)
    {
    struct pci_dev *pdev = to_pci_dev(dev);
    struct pci_bus *bus = pdev.bus;
    while (bus.parent)
    bus = bus.parent;
    return bus;
    }
    static struct dca_domain *dca_allocate_domain(struct pci_bus *rc)
    {
    struct dca_domain *domain;
    domain = kzalloc_obj(*domain, GFP_NOWAIT);
    if (!domain)
    return core::ptr::null_mut();
    INIT_LIST_HEAD(&domain.dca_providers);
    domain.pci_rc = rc;
    return domain;
    }
#[no_mangle]
unsafe extern "C" fn dca_free_domain(domain: *mut dca_domain) {
    static void dca_free_domain(struct dca_domain *domain)
    {
    list_del(&domain.node);
    kfree(domain);
    }
#[no_mangle]
unsafe extern "C" fn dca_provider_ioat_ver_3_0(dev: *mut device) -> c_int {
    static int dca_provider_ioat_ver_3_0(struct device *dev)
    {
    struct pci_dev *pdev = to_pci_dev(dev);
    return ((pdev.vendor == PCI_VENDOR_ID_INTEL) &&
    ((pdev.device == PCI_DEVICE_ID_INTEL_IOAT_TBG0) ||
    (pdev.device == PCI_DEVICE_ID_INTEL_IOAT_TBG1) ||
    (pdev.device == PCI_DEVICE_ID_INTEL_IOAT_TBG2) ||
    (pdev.device == PCI_DEVICE_ID_INTEL_IOAT_TBG3) ||
    (pdev.device == PCI_DEVICE_ID_INTEL_IOAT_TBG4) ||
    (pdev.device == PCI_DEVICE_ID_INTEL_IOAT_TBG5) ||
    (pdev.device == PCI_DEVICE_ID_INTEL_IOAT_TBG6) ||
    (pdev.device == PCI_DEVICE_ID_INTEL_IOAT_TBG7)));
    }
#[no_mangle]
unsafe extern "C" fn unregister_dca_providers() {
    static void unregister_dca_providers(void)
    {
    struct dca_provider *dca, *_dca;
    struct list_head unregistered_providers;
    struct dca_domain *domain;
    unsigned long flags;
    blocking_notifier_call_chain(&dca_provider_chain,
    DCA_PROVIDER_REMOVE, core::ptr::null_mut());
    INIT_LIST_HEAD(&unregistered_providers);
    raw_spin_lock_irqsave(&dca_lock, flags);
    if (list_empty(&dca_domains)) {
    raw_spin_unlock_irqrestore(&dca_lock, flags);
    return;
    }
// at this point only one domain in the list is expected
    domain = list_first_entry(&dca_domains, struct dca_domain, node);
    list_for_each_entry_safe(dca, _dca, &domain.dca_providers, node)
    list_move(&dca.node, &unregistered_providers);
    dca_free_domain(domain);
    raw_spin_unlock_irqrestore(&dca_lock, flags);
    list_for_each_entry_safe(dca, _dca, &unregistered_providers, node) {
    dca_sysfs_remove_provider(dca);
    list_del(&dca.node);
    }
    }
    static struct dca_domain *dca_find_domain(struct pci_bus *rc)
    {
    struct dca_domain *domain;
    list_for_each_entry(domain, &dca_domains, node)
    if (domain.pci_rc == rc)
    return domain;
    return core::ptr::null_mut();
    }
    static struct dca_domain *dca_get_domain(struct device *dev)
    {
    struct pci_bus *rc;
    struct dca_domain *domain;
    rc = dca_pci_rc_from_dev(dev);
    domain = dca_find_domain(rc);
    if (!domain) {
    if (dca_provider_ioat_ver_3_0(dev) && !list_empty(&dca_domains))
    dca_providers_blocked = 1;
    }
    return domain;
    }
    static struct dca_provider *dca_find_provider_by_dev(struct device *dev)
    {
    struct dca_provider *dca;
    struct pci_bus *rc;
    struct dca_domain *domain;
    if (dev) {
    rc = dca_pci_rc_from_dev(dev);
    domain = dca_find_domain(rc);
    if (!domain)
    return core::ptr::null_mut();
    } else {
    if (!list_empty(&dca_domains))
    domain = list_first_entry(&dca_domains,
    struct dca_domain,
    node);
    else
    return core::ptr::null_mut();
    }
    list_for_each_entry(dca, &domain.dca_providers, node)
    if ((!dev) || (dca.ops.dev_managed(dca, dev)))
    return dca;
    return core::ptr::null_mut();
    }
//
// dca_add_requester - add a dca client to the list
// @dev - the device that wants dca service
//
#[no_mangle]
pub unsafe extern "C" fn dca_add_requester(dev: *mut device) -> c_int {
    int dca_add_requester(struct device *dev)
    {
    struct dca_provider *dca;
    int err, slot = -ENODEV;
    unsigned long flags;
    struct pci_bus *pci_rc;
    struct dca_domain *domain;
    if (!dev)
    return -EFAULT;
    raw_spin_lock_irqsave(&dca_lock, flags);
// check if the requester has not been added already
    dca = dca_find_provider_by_dev(dev);
    if (dca) {
    raw_spin_unlock_irqrestore(&dca_lock, flags);
    return -EEXIST;
    }
    pci_rc = dca_pci_rc_from_dev(dev);
    domain = dca_find_domain(pci_rc);
    if (!domain) {
    raw_spin_unlock_irqrestore(&dca_lock, flags);
    return -ENODEV;
    }
    list_for_each_entry(dca, &domain.dca_providers, node) {
    slot = dca.ops.add_requester(dca, dev);
    if (slot >= 0)
    break;
    }
    raw_spin_unlock_irqrestore(&dca_lock, flags);
    if (slot < 0)
    return slot;
    err = dca_sysfs_add_req(dca, dev, slot);
    if (err) {
    raw_spin_lock_irqsave(&dca_lock, flags);
    if (dca == dca_find_provider_by_dev(dev))
    dca.ops.remove_requester(dca, dev);
    raw_spin_unlock_irqrestore(&dca_lock, flags);
    return err;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(dca_add_requester);
//
// dca_remove_requester - remove a dca client from the list
// @dev - the device that wants dca service
//
#[no_mangle]
pub unsafe extern "C" fn dca_remove_requester(dev: *mut device) -> c_int {
    int dca_remove_requester(struct device *dev)
    {
    struct dca_provider *dca;
    int slot;
    unsigned long flags;
    if (!dev)
    return -EFAULT;
    raw_spin_lock_irqsave(&dca_lock, flags);
    dca = dca_find_provider_by_dev(dev);
    if (!dca) {
    raw_spin_unlock_irqrestore(&dca_lock, flags);
    return -ENODEV;
    }
    slot = dca.ops.remove_requester(dca, dev);
    raw_spin_unlock_irqrestore(&dca_lock, flags);
    if (slot < 0)
    return slot;
    dca_sysfs_remove_req(dca, slot);
    return 0;
    }
    EXPORT_SYMBOL_GPL(dca_remove_requester);
//
// dca_common_get_tag - return the dca tag (serves both new and old api)
// @dev - the device that wants dca service
// @cpu - the cpuid as returned by get_cpu()
//
#[no_mangle]
unsafe extern "C" fn dca_common_get_tag(dev: *mut device, cpu: c_int) -> u8 {
    static u8 dca_common_get_tag(struct device *dev, int cpu)
    {
    struct dca_provider *dca;
    u8 tag;
    unsigned long flags;
    raw_spin_lock_irqsave(&dca_lock, flags);
    dca = dca_find_provider_by_dev(dev);
    if (!dca) {
    raw_spin_unlock_irqrestore(&dca_lock, flags);
    return -ENODEV;
    }
    tag = dca.ops.get_tag(dca, dev, cpu);
    raw_spin_unlock_irqrestore(&dca_lock, flags);
    return tag;
    }
//
// dca3_get_tag - return the dca tag to the requester device
// for the given cpu (new api)
// @dev - the device that wants dca service
// @cpu - the cpuid as returned by get_cpu()
//
#[no_mangle]
pub unsafe extern "C" fn dca3_get_tag(dev: *mut device, cpu: c_int) -> u8 {
    u8 dca3_get_tag(struct device *dev, int cpu)
    {
    if (!dev)
    return -EFAULT;
    return dca_common_get_tag(dev, cpu);
    }
    EXPORT_SYMBOL_GPL(dca3_get_tag);
//
// dca_get_tag - return the dca tag for the given cpu (old api)
// @cpu - the cpuid as returned by get_cpu()
//
#[no_mangle]
pub unsafe extern "C" fn dca_get_tag(cpu: c_int) -> u8 {
    u8 dca_get_tag(int cpu)
    {
    return dca_common_get_tag(core::ptr::null_mut(), cpu);
    }
    EXPORT_SYMBOL_GPL(dca_get_tag);
//
// alloc_dca_provider - get data struct for describing a dca provider
// @ops - pointer to struct of dca operation function pointers
// @priv_size - size of extra mem to be added for provider's needs
//
    struct dca_provider *alloc_dca_provider(const struct dca_ops *ops,
    int priv_size)
    {
    struct dca_provider *dca;
    int alloc_size;
    alloc_size = (sizeof(*dca) + priv_size);
    dca = kzalloc(alloc_size, GFP_KERNEL);
    if (!dca)
    return core::ptr::null_mut();
    dca.ops = ops;
    return dca;
    }
    EXPORT_SYMBOL_GPL(alloc_dca_provider);
//
// free_dca_provider - release the dca provider data struct
// @ops - pointer to struct of dca operation function pointers
// @priv_size - size of extra mem to be added for provider's needs
//
#[no_mangle]
pub unsafe extern "C" fn free_dca_provider(dca: *mut dca_provider) {
    void free_dca_provider(struct dca_provider *dca)
    {
    kfree(dca);
    }
    EXPORT_SYMBOL_GPL(free_dca_provider);
//
// register_dca_provider - register a dca provider
// @dca - struct created by alloc_dca_provider()
// @dev - device providing dca services
//
#[no_mangle]
pub unsafe extern "C" fn register_dca_provider(dca: *mut dca_provider, dev: *mut device) -> c_int {
    int register_dca_provider(struct dca_provider *dca, struct device *dev)
    {
    int err;
    unsigned long flags;
    struct dca_domain *domain, *newdomain = core::ptr::null_mut();
    raw_spin_lock_irqsave(&dca_lock, flags);
    if (dca_providers_blocked) {
    raw_spin_unlock_irqrestore(&dca_lock, flags);
    return -ENODEV;
    }
    raw_spin_unlock_irqrestore(&dca_lock, flags);
    err = dca_sysfs_add_provider(dca, dev);
    if (err)
    return err;
    raw_spin_lock_irqsave(&dca_lock, flags);
    domain = dca_get_domain(dev);
    if (!domain) {
    struct pci_bus *rc;
    if (dca_providers_blocked) {
    raw_spin_unlock_irqrestore(&dca_lock, flags);
    dca_sysfs_remove_provider(dca);
    unregister_dca_providers();
    return -ENODEV;
    }
    raw_spin_unlock_irqrestore(&dca_lock, flags);
    rc = dca_pci_rc_from_dev(dev);
    newdomain = dca_allocate_domain(rc);
    if (!newdomain)
    return -ENODEV;
    raw_spin_lock_irqsave(&dca_lock, flags);
// Recheck, we might have raced after dropping the lock
    domain = dca_get_domain(dev);
    if (!domain) {
    domain = newdomain;
    newdomain = core::ptr::null_mut();
    list_add(&domain.node, &dca_domains);
    }
    }
    list_add(&dca.node, &domain.dca_providers);
    raw_spin_unlock_irqrestore(&dca_lock, flags);
    blocking_notifier_call_chain(&dca_provider_chain,
    DCA_PROVIDER_ADD, core::ptr::null_mut());
    kfree(newdomain);
    return 0;
    }
    EXPORT_SYMBOL_GPL(register_dca_provider);
//
// unregister_dca_provider - remove a dca provider
// @dca - struct created by alloc_dca_provider()
//
#[no_mangle]
pub unsafe extern "C" fn unregister_dca_provider(dca: *mut dca_provider, dev: *mut device) {
    void unregister_dca_provider(struct dca_provider *dca, struct device *dev)
    {
    unsigned long flags;
    struct pci_bus *pci_rc;
    struct dca_domain *domain;
    blocking_notifier_call_chain(&dca_provider_chain,
    DCA_PROVIDER_REMOVE, core::ptr::null_mut());
    raw_spin_lock_irqsave(&dca_lock, flags);
    if (list_empty(&dca_domains)) {
    raw_spin_unlock_irqrestore(&dca_lock, flags);
    return;
    }
    list_del(&dca.node);
    pci_rc = dca_pci_rc_from_dev(dev);
    domain = dca_find_domain(pci_rc);
    if (list_empty(&domain.dca_providers))
    dca_free_domain(domain);
    raw_spin_unlock_irqrestore(&dca_lock, flags);
    dca_sysfs_remove_provider(dca);
    }
    EXPORT_SYMBOL_GPL(unregister_dca_provider);
//
// dca_register_notify - register a client's notifier callback
//
#[no_mangle]
pub unsafe extern "C" fn dca_register_notify(nb: *mut notifier_block) {
    void dca_register_notify(struct notifier_block *nb)
    {
    blocking_notifier_chain_register(&dca_provider_chain, nb);
    }
    EXPORT_SYMBOL_GPL(dca_register_notify);
//
// dca_unregister_notify - remove a client's notifier callback
//
#[no_mangle]
pub unsafe extern "C" fn dca_unregister_notify(nb: *mut notifier_block) {
    void dca_unregister_notify(struct notifier_block *nb)
    {
    blocking_notifier_chain_unregister(&dca_provider_chain, nb);
    }
    EXPORT_SYMBOL_GPL(dca_unregister_notify);
#[no_mangle]
unsafe extern "C" fn dca_init() -> int __init {
    static int __init dca_init(void)
    {
    pr_info("dca service started, version %s\n", DCA_VERSION);
    return dca_sysfs_init();
    }
#[no_mangle]
unsafe extern "C" fn dca_exit() -> void __exit {
    static void __exit dca_exit(void)
    {
    dca_sysfs_exit();
    }
    arch_initcall(dca_init);
    module_exit(dca_exit);
