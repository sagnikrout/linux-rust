//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/intel/qat/qat_common/adf_isr.c
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


// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
// Copyright(c) 2014 - 2020 Intel Corporation

pub const ADF_MAX_NUM_VFS: c_int = 32;
    static struct workqueue_struct *adf_misc_wq;
#[no_mangle]
unsafe extern "C" fn adf_enable_msix(accel_dev: *mut adf_accel_dev) -> c_int {
    static int adf_enable_msix(struct adf_accel_dev *accel_dev)
    {
    struct adf_accel_pci *pci_dev_info = &accel_dev.accel_pci_dev;
    struct adf_hw_device_data *hw_data = accel_dev.hw_device;
    let mut msix_num_entries: u32 = hw_data.num_banks + 1;
    int ret;
    if (hw_data.set_msix_rttable)
    hw_data.set_msix_rttable(accel_dev);
    ret = pci_alloc_irq_vectors(pci_dev_info.pci_dev, msix_num_entries,
    msix_num_entries, PCI_IRQ_MSIX);
    if (unlikely(ret < 0)) {
    dev_err(&GET_DEV(accel_dev),
    "Failed to allocate %d MSI-X vectors\n",
    msix_num_entries);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adf_disable_msix(pci_dev_info: *mut adf_accel_pci) {
    static void adf_disable_msix(struct adf_accel_pci *pci_dev_info)
    {
    pci_free_irq_vectors(pci_dev_info.pci_dev);
    }
#[no_mangle]
unsafe extern "C" fn adf_msix_isr_bundle(irq: c_int, bank_ptr: *mut c_void) -> irqreturn_t {
    static irqreturn_t adf_msix_isr_bundle(int irq, void *bank_ptr)
    {
    struct adf_etr_bank_data *bank = bank_ptr;
    struct adf_hw_csr_ops *csr_ops = GET_CSR_OPS(bank.accel_dev);
    csr_ops.write_csr_int_flag_and_col(bank.csr_addr, bank.bank_number,
    0);
    tasklet_hi_schedule(&bank.resp_handler);
    return IRQ_HANDLED;
    }

#[no_mangle]
pub unsafe extern "C" fn adf_enable_vf2pf_interrupts(accel_dev: *mut adf_accel_dev, vf_mask: u32) {
    void adf_enable_vf2pf_interrupts(struct adf_accel_dev *accel_dev, u32 vf_mask)
    {
    void __iomem *pmisc_addr = adf_get_pmisc_base(accel_dev);
    unsigned long flags;
    spin_lock_irqsave(&accel_dev.pf.vf2pf_ints_lock, flags);
    if (!READ_ONCE(accel_dev.pf.vf2pf_disabled))
    GET_PFVF_OPS(accel_dev).enable_vf2pf_interrupts(pmisc_addr, vf_mask);
    spin_unlock_irqrestore(&accel_dev.pf.vf2pf_ints_lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn adf_enable_all_vf2pf_interrupts(accel_dev: *mut adf_accel_dev, num_vfs: u32) {
    void adf_enable_all_vf2pf_interrupts(struct adf_accel_dev *accel_dev, u32 num_vfs)
    {
    void __iomem *pmisc_addr = adf_get_pmisc_base(accel_dev);
    unsigned long flags;
    u32 vf_mask;
    vf_mask = BIT_ULL(num_vfs) - 1;
    if (!vf_mask)
    return;
    spin_lock_irqsave(&accel_dev.pf.vf2pf_ints_lock, flags);
    WRITE_ONCE(accel_dev.pf.vf2pf_disabled, false);
    GET_PFVF_OPS(accel_dev).enable_vf2pf_interrupts(pmisc_addr, vf_mask);
    spin_unlock_irqrestore(&accel_dev.pf.vf2pf_ints_lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn adf_disable_all_vf2pf_interrupts(accel_dev: *mut adf_accel_dev) {
    void adf_disable_all_vf2pf_interrupts(struct adf_accel_dev *accel_dev)
    {
    void __iomem *pmisc_addr = adf_get_pmisc_base(accel_dev);
    unsigned long flags;
    spin_lock_irqsave(&accel_dev.pf.vf2pf_ints_lock, flags);
    WRITE_ONCE(accel_dev.pf.vf2pf_disabled, true);
    GET_PFVF_OPS(accel_dev).disable_all_vf2pf_interrupts(pmisc_addr);
    spin_unlock_irqrestore(&accel_dev.pf.vf2pf_ints_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn adf_disable_pending_vf2pf_interrupts(accel_dev: *mut adf_accel_dev) -> u32 {
    static u32 adf_disable_pending_vf2pf_interrupts(struct adf_accel_dev *accel_dev)
    {
    void __iomem *pmisc_addr = adf_get_pmisc_base(accel_dev);
    u32 pending;
    spin_lock(&accel_dev.pf.vf2pf_ints_lock);
    pending = GET_PFVF_OPS(accel_dev).disable_pending_vf2pf_interrupts(pmisc_addr);
    spin_unlock(&accel_dev.pf.vf2pf_ints_lock);
    return pending;
    }
#[no_mangle]
unsafe extern "C" fn adf_handle_vf2pf_int(accel_dev: *mut adf_accel_dev) -> bool {
    static bool adf_handle_vf2pf_int(struct adf_accel_dev *accel_dev)
    {
    let mut irq_handled: bool = false;
    unsigned long vf_mask;
// Get the interrupt sources triggered by VFs, except for those already disabled
    vf_mask = adf_disable_pending_vf2pf_interrupts(accel_dev);
    if (vf_mask) {
    struct adf_accel_vf_info *vf_info;
    int i;
//
// Handle VF2PF interrupt unless the VF is malicious and
// is attempting to flood the host OS with VF2PF interrupts.
//
    for_each_set_bit(i, &vf_mask, ADF_MAX_NUM_VFS) {
    vf_info = accel_dev.pf.vf_info + i;
    if (!__ratelimit(&vf_info.vf2pf_ratelimit)) {
    dev_info(&GET_DEV(accel_dev),
    "Too many ints from VF%d\n",
    vf_info.vf_nr);
    continue;
    }
    adf_schedule_vf2pf_handler(vf_info);
    irq_handled = true;
    }
    }
    return irq_handled;
    }

#[no_mangle]
unsafe extern "C" fn adf_handle_pm_int(accel_dev: *mut adf_accel_dev) -> bool {
    static bool adf_handle_pm_int(struct adf_accel_dev *accel_dev)
    {
    struct adf_hw_device_data *hw_data = accel_dev.hw_device;
    if (hw_data.handle_pm_interrupt &&
    hw_data.handle_pm_interrupt(accel_dev))
    return true;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn adf_handle_ras_int(accel_dev: *mut adf_accel_dev) -> bool {
    static bool adf_handle_ras_int(struct adf_accel_dev *accel_dev)
    {
    struct adf_ras_ops *ras_ops = &accel_dev.hw_device.ras_ops;
    bool reset_required;
    if (ras_ops.handle_interrupt &&
    ras_ops.handle_interrupt(accel_dev, &reset_required)) {
    if (reset_required) {
    dev_err(&GET_DEV(accel_dev), "Fatal error, reset required\n");
    if (adf_notify_fatal_error(accel_dev))
    dev_err(&GET_DEV(accel_dev),
    "Failed to notify fatal error\n");
    }
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn adf_msix_isr_ae(irq: c_int, dev_ptr: *mut c_void) -> irqreturn_t {
    static irqreturn_t adf_msix_isr_ae(int irq, void *dev_ptr)
    {
    struct adf_accel_dev *accel_dev = dev_ptr;

// If SR-IOV is enabled (vf_info is non-NULL), check for VF->PF ints
    if (accel_dev.pf.vf_info && adf_handle_vf2pf_int(accel_dev))
    return IRQ_HANDLED;

    if (adf_handle_pm_int(accel_dev))
    return IRQ_HANDLED;
    if (adf_handle_ras_int(accel_dev))
    return IRQ_HANDLED;
    dev_dbg(&GET_DEV(accel_dev), "qat_dev%d spurious AE interrupt\n",
    accel_dev.accel_id);
    return IRQ_NONE;
    }
#[no_mangle]
pub unsafe extern "C" fn adf_isr_sync_ae_cluster(accel_dev: *mut adf_accel_dev) {
    void adf_isr_sync_ae_cluster(struct adf_accel_dev *accel_dev)
    {
    struct adf_accel_pci *pci_dev_info = &accel_dev.accel_pci_dev;
    struct adf_hw_device_data *hw_data = GET_HW_DATA(accel_dev);
    let mut num_entries: u32 = pci_dev_info.msix_entries.num_entries;
    struct adf_irq *irqs = pci_dev_info.msix_entries.irqs;
    u32 irq_idx;
    int irq;
    if (!test_bit(ADF_STATUS_IRQ_ALLOCATED, &accel_dev.status) || !irqs)
    return;
    irq_idx = num_entries > 1 ? hw_data.num_banks : 0;
    if (irq_idx >= num_entries || !irqs[irq_idx].enabled)
    return;
    irq = pci_irq_vector(pci_dev_info.pci_dev, hw_data.num_banks);
    if (irq > 0)
    synchronize_irq(irq);
    }
#[no_mangle]
unsafe extern "C" fn adf_free_irqs(accel_dev: *mut adf_accel_dev) {
    static void adf_free_irqs(struct adf_accel_dev *accel_dev)
    {
    struct adf_accel_pci *pci_dev_info = &accel_dev.accel_pci_dev;
    struct adf_hw_device_data *hw_data = accel_dev.hw_device;
    struct adf_irq *irqs = pci_dev_info.msix_entries.irqs;
    struct adf_etr_data *etr_data = accel_dev.transport;
    let mut clust_irq: c_int = hw_data.num_banks;
    int irq, i = 0;
    if (pci_dev_info.msix_entries.num_entries > 1) {
    for (i = 0; i < hw_data.num_banks; i++) {
    if (irqs[i].enabled) {
    irq = pci_irq_vector(pci_dev_info.pci_dev, i);
    irq_set_affinity_hint(irq, core::ptr::null_mut());
    free_irq(irq, &etr_data.banks[i]);
    }
    }
    }
    if (irqs[i].enabled) {
    irq = pci_irq_vector(pci_dev_info.pci_dev, clust_irq);
    free_irq(irq, accel_dev);
    }
    }
#[no_mangle]
unsafe extern "C" fn adf_request_irqs(accel_dev: *mut adf_accel_dev) -> c_int {
    static int adf_request_irqs(struct adf_accel_dev *accel_dev)
    {
    struct adf_accel_pci *pci_dev_info = &accel_dev.accel_pci_dev;
    struct adf_hw_device_data *hw_data = accel_dev.hw_device;
    struct adf_irq *irqs = pci_dev_info.msix_entries.irqs;
    struct adf_etr_data *etr_data = accel_dev.transport;
    let mut clust_irq: c_int = hw_data.num_banks;
    int ret, irq, i = 0;
    char *name;
// Request msix irq for all banks unless SR-IOV enabled
    if (!accel_dev.pf.vf_info) {
    for (i = 0; i < hw_data.num_banks; i++) {
    struct adf_etr_bank_data *bank = &etr_data.banks[i];
    unsigned int cpu, cpus = num_online_cpus();
    name = irqs[i].name;
    snprintf(name, ADF_MAX_MSIX_VECTOR_NAME,
    "qat%d-bundle%d", accel_dev.accel_id, i);
    irq = pci_irq_vector(pci_dev_info.pci_dev, i);
    if (unlikely(irq < 0)) {
    dev_err(&GET_DEV(accel_dev),
    "Failed to get IRQ number of device vector %d - %s\n",
    i, name);
    ret = irq;
    goto err;
    }
    ret = request_irq(irq, adf_msix_isr_bundle, 0,
    &name[0], bank);
    if (ret) {
    dev_err(&GET_DEV(accel_dev),
    "Failed to allocate IRQ %d for %s\n",
    irq, name);
    goto err;
    }
    cpu = ((accel_dev.accel_id * hw_data.num_banks) +
    i) % cpus;
    irq_set_affinity_hint(irq, get_cpu_mask(cpu));
    irqs[i].enabled = true;
    }
    }
// Request msix irq for AE
    name = irqs[i].name;
    snprintf(name, ADF_MAX_MSIX_VECTOR_NAME,
    "qat%d-ae-cluster", accel_dev.accel_id);
    irq = pci_irq_vector(pci_dev_info.pci_dev, clust_irq);
    if (unlikely(irq < 0)) {
    dev_err(&GET_DEV(accel_dev),
    "Failed to get IRQ number of device vector %d - %s\n",
    i, name);
    ret = irq;
    goto err;
    }
    ret = request_irq(irq, adf_msix_isr_ae, 0, &name[0], accel_dev);
    if (ret) {
    dev_err(&GET_DEV(accel_dev),
    "Failed to allocate IRQ %d for %s\n", irq, name);
    goto err;
    }
    irqs[i].enabled = true;
    return ret;
    err:
    adf_free_irqs(accel_dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn adf_isr_alloc_msix_vectors_data(accel_dev: *mut adf_accel_dev) -> c_int {
    static int adf_isr_alloc_msix_vectors_data(struct adf_accel_dev *accel_dev)
    {
    struct adf_hw_device_data *hw_data = accel_dev.hw_device;
    let mut msix_num_entries: u32 = 1;
    struct adf_irq *irqs;
// If SR-IOV is disabled (vf_info is NULL), add entries for each bank
    if (!accel_dev.pf.vf_info)
    msix_num_entries += hw_data.num_banks;
    irqs = kcalloc_node(msix_num_entries, sizeof(*irqs),
    GFP_KERNEL, dev_to_node(&GET_DEV(accel_dev)));
    if (!irqs)
    return -ENOMEM;
    accel_dev.accel_pci_dev.msix_entries.num_entries = msix_num_entries;
    accel_dev.accel_pci_dev.msix_entries.irqs = irqs;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adf_isr_free_msix_vectors_data(accel_dev: *mut adf_accel_dev) {
    static void adf_isr_free_msix_vectors_data(struct adf_accel_dev *accel_dev)
    {
    kfree(accel_dev.accel_pci_dev.msix_entries.irqs);
    accel_dev.accel_pci_dev.msix_entries.irqs = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn adf_setup_bh(accel_dev: *mut adf_accel_dev) -> c_int {
    static int adf_setup_bh(struct adf_accel_dev *accel_dev)
    {
    struct adf_etr_data *priv_data = accel_dev.transport;
    struct adf_hw_device_data *hw_data = accel_dev.hw_device;
    int i;
    for (i = 0; i < hw_data.num_banks; i++)
    tasklet_init(&priv_data.banks[i].resp_handler,
    adf_response_handler,
    (unsigned long)&priv_data.banks[i]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adf_cleanup_bh(accel_dev: *mut adf_accel_dev) {
    static void adf_cleanup_bh(struct adf_accel_dev *accel_dev)
    {
    struct adf_etr_data *priv_data = accel_dev.transport;
    struct adf_hw_device_data *hw_data = accel_dev.hw_device;
    int i;
    for (i = 0; i < hw_data.num_banks; i++) {
    tasklet_disable(&priv_data.banks[i].resp_handler);
    tasklet_kill(&priv_data.banks[i].resp_handler);
    }
    }
//
// adf_isr_resource_free() - Free IRQ for acceleration device
// @accel_dev:  Pointer to acceleration device.
//
// Function frees interrupts for acceleration device.
//
#[no_mangle]
pub unsafe extern "C" fn adf_isr_resource_free(accel_dev: *mut adf_accel_dev) {
    void adf_isr_resource_free(struct adf_accel_dev *accel_dev)
    {
    adf_free_irqs(accel_dev);
    adf_cleanup_bh(accel_dev);
    adf_disable_msix(&accel_dev.accel_pci_dev);
    adf_isr_free_msix_vectors_data(accel_dev);
    }
    EXPORT_SYMBOL_GPL(adf_isr_resource_free);
//
// adf_isr_resource_alloc() - Allocate IRQ for acceleration device
// @accel_dev:  Pointer to acceleration device.
//
// Function allocates interrupts for acceleration device.
//
// Return: 0 on success, error code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn adf_isr_resource_alloc(accel_dev: *mut adf_accel_dev) -> c_int {
    int adf_isr_resource_alloc(struct adf_accel_dev *accel_dev)
    {
    int ret;
    ret = adf_isr_alloc_msix_vectors_data(accel_dev);
    if (ret)
    goto err_out;
    ret = adf_enable_msix(accel_dev);
    if (ret)
    goto err_free_msix_table;
    ret = adf_setup_bh(accel_dev);
    if (ret)
    goto err_disable_msix;
    ret = adf_request_irqs(accel_dev);
    if (ret)
    goto err_cleanup_bh;
    return 0;
    err_cleanup_bh:
    adf_cleanup_bh(accel_dev);
    err_disable_msix:
    adf_disable_msix(&accel_dev.accel_pci_dev);
    err_free_msix_table:
    adf_isr_free_msix_vectors_data(accel_dev);
    err_out:
    return ret;
    }
    EXPORT_SYMBOL_GPL(adf_isr_resource_alloc);
//
// adf_init_misc_wq() - Init misc workqueue
//
// Return: 0 on success, error code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn adf_init_misc_wq() -> int __init {
    int __init adf_init_misc_wq(void)
    {
    adf_misc_wq = alloc_workqueue("qat_misc_wq",
    WQ_MEM_RECLAIM | WQ_PERCPU, 0);
    return !adf_misc_wq ? -ENOMEM : 0;
    }
#[no_mangle]
pub unsafe extern "C" fn adf_exit_misc_wq() {
    void adf_exit_misc_wq(void)
    {
    if (adf_misc_wq)
    destroy_workqueue(adf_misc_wq);
    adf_misc_wq = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn adf_misc_wq_queue_work(work: *mut work_struct) -> bool {
    bool adf_misc_wq_queue_work(struct work_struct *work)
    {
    return queue_work(adf_misc_wq, work);
    }
    bool adf_misc_wq_queue_delayed_work(struct delayed_work *work,
    unsigned long delay)
    {
    return queue_delayed_work(adf_misc_wq, work, delay);
    }
#[no_mangle]
pub unsafe extern "C" fn adf_misc_wq_flush() {
    void adf_misc_wq_flush(void)
    {
    flush_workqueue(adf_misc_wq);
    }
