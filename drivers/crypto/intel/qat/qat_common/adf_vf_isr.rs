//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/intel/qat/qat_common/adf_vf_isr.c
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

pub const ADF_VINTSOU_OFFSET: c_uint = 0x204;
pub const ADF_VINTMSK_OFFSET: c_uint = 0x208;

    static struct workqueue_struct *adf_vf_stop_wq;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_vf_stop_data {
    pub accel_dev: *mut adf_accel_dev,
    pub work: work_struct,
}

#[no_mangle]
pub unsafe extern "C" fn adf_enable_pf2vf_interrupts(accel_dev: *mut adf_accel_dev) {
    void adf_enable_pf2vf_interrupts(struct adf_accel_dev *accel_dev)
    {
    void __iomem *pmisc_addr = adf_get_pmisc_base(accel_dev);
    ADF_CSR_WR(pmisc_addr, ADF_VINTMSK_OFFSET, 0x0);
    }
#[no_mangle]
pub unsafe extern "C" fn adf_disable_pf2vf_interrupts(accel_dev: *mut adf_accel_dev) {
    void adf_disable_pf2vf_interrupts(struct adf_accel_dev *accel_dev)
    {
    void __iomem *pmisc_addr = adf_get_pmisc_base(accel_dev);
    ADF_CSR_WR(pmisc_addr, ADF_VINTMSK_OFFSET, 0x2);
    }
    EXPORT_SYMBOL_GPL(adf_disable_pf2vf_interrupts);
#[no_mangle]
unsafe extern "C" fn adf_enable_msi(accel_dev: *mut adf_accel_dev) -> c_int {
    static int adf_enable_msi(struct adf_accel_dev *accel_dev)
    {
    struct adf_accel_pci *pci_dev_info = &accel_dev.accel_pci_dev;
    int stat = pci_alloc_irq_vectors(pci_dev_info.pci_dev, 1, 1,
    PCI_IRQ_MSI);
    if (unlikely(stat < 0)) {
    dev_err(&GET_DEV(accel_dev),
    "Failed to enable MSI interrupt: %d\n", stat);
    return stat;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adf_disable_msi(accel_dev: *mut adf_accel_dev) {
    static void adf_disable_msi(struct adf_accel_dev *accel_dev)
    {
    struct pci_dev *pdev = accel_to_pci_dev(accel_dev);
    pci_free_irq_vectors(pdev);
    }
#[no_mangle]
unsafe extern "C" fn adf_dev_stop_async(work: *mut work_struct) {
    static void adf_dev_stop_async(struct work_struct *work)
    {
    struct adf_vf_stop_data *stop_data =
    container_of(work, struct adf_vf_stop_data, work);
    struct adf_accel_dev *accel_dev = stop_data.accel_dev;
    adf_dev_restarting_notify(accel_dev);
    adf_dev_down(accel_dev);
// Re-enable PF2VF interrupts
    adf_enable_pf2vf_interrupts(accel_dev);
    adf_vf2pf_notify_restart_complete(accel_dev);
    kfree(stop_data);
    }
#[no_mangle]
pub unsafe extern "C" fn adf_pf2vf_handle_pf_restarting(accel_dev: *mut adf_accel_dev) -> c_int {
    int adf_pf2vf_handle_pf_restarting(struct adf_accel_dev *accel_dev)
    {
    struct adf_vf_stop_data *stop_data;
    clear_bit(ADF_STATUS_PF_RUNNING, &accel_dev.status);
    stop_data = kzalloc_obj(*stop_data, GFP_ATOMIC);
    if (!stop_data) {
    dev_err(&GET_DEV(accel_dev),
    "Couldn't schedule stop for vf_%d\n",
    accel_dev.accel_id);
    return -ENOMEM;
    }
    stop_data.accel_dev = accel_dev;
    INIT_WORK(&stop_data.work, adf_dev_stop_async);
    queue_work(adf_vf_stop_wq, &stop_data.work);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adf_pf2vf_bh_handler(data: *mut c_void) {
    static void adf_pf2vf_bh_handler(void *data)
    {
    struct adf_accel_dev *accel_dev = data;
    bool ret;
    ret = adf_recv_and_handle_pf2vf_msg(accel_dev);
    if (ret)
// Re-enable PF2VF interrupts
    adf_enable_pf2vf_interrupts(accel_dev);
    return;
    }
#[no_mangle]
unsafe extern "C" fn adf_setup_pf2vf_bh(accel_dev: *mut adf_accel_dev) -> c_int {
    static int adf_setup_pf2vf_bh(struct adf_accel_dev *accel_dev)
    {
    tasklet_init(&accel_dev.vf.pf2vf_bh_tasklet,
    (void *)adf_pf2vf_bh_handler, (unsigned long)accel_dev);
    mutex_init(&accel_dev.vf.vf2pf_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adf_cleanup_pf2vf_bh(accel_dev: *mut adf_accel_dev) {
    static void adf_cleanup_pf2vf_bh(struct adf_accel_dev *accel_dev)
    {
    tasklet_disable(&accel_dev.vf.pf2vf_bh_tasklet);
    tasklet_kill(&accel_dev.vf.pf2vf_bh_tasklet);
    mutex_destroy(&accel_dev.vf.vf2pf_lock);
    }
#[no_mangle]
unsafe extern "C" fn adf_isr(irq: c_int, privdata: *mut c_void) -> irqreturn_t {
    static irqreturn_t adf_isr(int irq, void *privdata)
    {
    struct adf_accel_dev *accel_dev = privdata;
    struct adf_hw_device_data *hw_data = accel_dev.hw_device;
    struct adf_hw_csr_ops *csr_ops = &hw_data.csr_ops;
    struct adf_bar *pmisc =
    &GET_BARS(accel_dev)[hw_data.get_misc_bar_id(hw_data)];
    void __iomem *pmisc_bar_addr = pmisc.virt_addr;
    let mut handled: bool = false;
    u32 v_int, v_mask;
// Read VF INT source CSR to determine the source of VF interrupt
    v_int = ADF_CSR_RD(pmisc_bar_addr, ADF_VINTSOU_OFFSET);
// Read VF INT mask CSR to determine which sources are masked
    v_mask = ADF_CSR_RD(pmisc_bar_addr, ADF_VINTMSK_OFFSET);
//
// Recompute v_int ignoring sources that are masked. This is to
// avoid rescheduling the tasklet for interrupts already handled
//
    v_int &= ~v_mask;
// Check for PF2VF interrupt
    if (v_int & ADF_VINTSOU_PF2VF) {
// Disable PF to VF interrupt
    adf_disable_pf2vf_interrupts(accel_dev);
// Schedule tasklet to handle interrupt BH
    tasklet_hi_schedule(&accel_dev.vf.pf2vf_bh_tasklet);
    handled = true;
    }
// Check bundle interrupt
    if (v_int & ADF_VINTSOU_BUN) {
    struct adf_etr_data *etr_data = accel_dev.transport;
    struct adf_etr_bank_data *bank = &etr_data.banks[0];
// Disable Flag and Coalesce Ring Interrupts
    csr_ops.write_csr_int_flag_and_col(bank.csr_addr,
    bank.bank_number, 0);
    tasklet_hi_schedule(&bank.resp_handler);
    handled = true;
    }
    return handled ? IRQ_HANDLED : IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn adf_request_msi_irq(accel_dev: *mut adf_accel_dev) -> c_int {
    static int adf_request_msi_irq(struct adf_accel_dev *accel_dev)
    {
    struct pci_dev *pdev = accel_to_pci_dev(accel_dev);
    unsigned int cpu;
    int ret;
    snprintf(accel_dev.vf.irq_name, ADF_MAX_MSIX_VECTOR_NAME,
    "qat_%02x:%02d.%02d", pdev.bus.number, PCI_SLOT(pdev.devfn),
    PCI_FUNC(pdev.devfn));
    ret = request_irq(pdev.irq, adf_isr, 0, accel_dev.vf.irq_name,
    (void *)accel_dev);
    if (ret) {
    dev_err(&GET_DEV(accel_dev), "failed to enable irq for %s\n",
    accel_dev.vf.irq_name);
    return ret;
    }
    cpu = accel_dev.accel_id % num_online_cpus();
    irq_set_affinity_hint(pdev.irq, get_cpu_mask(cpu));
    accel_dev.vf.irq_enabled = true;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn adf_setup_bh(accel_dev: *mut adf_accel_dev) -> c_int {
    static int adf_setup_bh(struct adf_accel_dev *accel_dev)
    {
    struct adf_etr_data *priv_data = accel_dev.transport;
    tasklet_init(&priv_data.banks[0].resp_handler, adf_response_handler,
    (unsigned long)priv_data.banks);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adf_cleanup_bh(accel_dev: *mut adf_accel_dev) {
    static void adf_cleanup_bh(struct adf_accel_dev *accel_dev)
    {
    struct adf_etr_data *priv_data = accel_dev.transport;
    tasklet_disable(&priv_data.banks[0].resp_handler);
    tasklet_kill(&priv_data.banks[0].resp_handler);
    }
//
// adf_vf_isr_resource_free() - Free IRQ for acceleration device
// @accel_dev:  Pointer to acceleration device.
//
// Function frees interrupts for acceleration device virtual function.
//
#[no_mangle]
pub unsafe extern "C" fn adf_vf_isr_resource_free(accel_dev: *mut adf_accel_dev) {
    void adf_vf_isr_resource_free(struct adf_accel_dev *accel_dev)
    {
    struct pci_dev *pdev = accel_to_pci_dev(accel_dev);
    if (accel_dev.vf.irq_enabled) {
    irq_set_affinity_hint(pdev.irq, core::ptr::null_mut());
    free_irq(pdev.irq, accel_dev);
    }
    adf_cleanup_bh(accel_dev);
    adf_cleanup_pf2vf_bh(accel_dev);
    adf_disable_msi(accel_dev);
    }
    EXPORT_SYMBOL_GPL(adf_vf_isr_resource_free);
//
// adf_vf_isr_resource_alloc() - Allocate IRQ for acceleration device
// @accel_dev:  Pointer to acceleration device.
//
// Function allocates interrupts for acceleration device virtual function.
//
// Return: 0 on success, error code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn adf_vf_isr_resource_alloc(accel_dev: *mut adf_accel_dev) -> c_int {
    int adf_vf_isr_resource_alloc(struct adf_accel_dev *accel_dev)
    {
    if (adf_enable_msi(accel_dev))
    goto err_out;
    if (adf_setup_pf2vf_bh(accel_dev))
    goto err_disable_msi;
    if (adf_setup_bh(accel_dev))
    goto err_cleanup_pf2vf_bh;
    if (adf_request_msi_irq(accel_dev))
    goto err_cleanup_bh;
    return 0;
    err_cleanup_bh:
    adf_cleanup_bh(accel_dev);
    err_cleanup_pf2vf_bh:
    adf_cleanup_pf2vf_bh(accel_dev);
    err_disable_msi:
    adf_disable_msi(accel_dev);
    err_out:
    return -EFAULT;
    }
    EXPORT_SYMBOL_GPL(adf_vf_isr_resource_alloc);
//
// adf_flush_vf_wq() - Flush workqueue for VF
// @accel_dev:  Pointer to acceleration device.
//
// Function disables the PF/VF interrupts on the VF so that no new messages
// are received and flushes the workqueue 'adf_vf_stop_wq'.
//
// Return: void.
//
#[no_mangle]
pub unsafe extern "C" fn adf_flush_vf_wq(accel_dev: *mut adf_accel_dev) {
    void adf_flush_vf_wq(struct adf_accel_dev *accel_dev)
    {
    adf_disable_pf2vf_interrupts(accel_dev);
    flush_workqueue(adf_vf_stop_wq);
    }
    EXPORT_SYMBOL_GPL(adf_flush_vf_wq);
//
// adf_init_vf_wq() - Init workqueue for VF
//
// Return: 0 on success, error code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn adf_init_vf_wq() -> int __init {
    int __init adf_init_vf_wq(void)
    {
    adf_vf_stop_wq = alloc_workqueue("adf_vf_stop_wq",
    WQ_MEM_RECLAIM | WQ_PERCPU, 0);
    return !adf_vf_stop_wq ? -EFAULT : 0;
    }
#[no_mangle]
pub unsafe extern "C" fn adf_exit_vf_wq() {
    void adf_exit_vf_wq(void)
    {
    if (adf_vf_stop_wq)
    destroy_workqueue(adf_vf_stop_wq);
    adf_vf_stop_wq = core::ptr::null_mut();
    }
