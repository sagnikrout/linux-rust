//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/intel/qat/qat_c62x/adf_drv.c
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

#[no_mangle]
unsafe extern "C" fn adf_cleanup_pci_dev(accel_dev: *mut adf_accel_dev) {
    static void adf_cleanup_pci_dev(struct adf_accel_dev *accel_dev)
    {
    pci_release_regions(accel_dev.accel_pci_dev.pci_dev);
    pci_disable_device(accel_dev.accel_pci_dev.pci_dev);
    }
#[no_mangle]
unsafe extern "C" fn adf_cleanup_accel(accel_dev: *mut adf_accel_dev) {
    static void adf_cleanup_accel(struct adf_accel_dev *accel_dev)
    {
    struct adf_accel_pci *accel_pci_dev = &accel_dev.accel_pci_dev;
    int i;
    for (i = 0; i < ADF_PCI_MAX_BARS; i++) {
    struct adf_bar *bar = &accel_pci_dev.pci_bars[i];
    if (bar.virt_addr)
    pci_iounmap(accel_pci_dev.pci_dev, bar.virt_addr);
    }
    if (accel_dev.hw_device) {
    switch (accel_pci_dev.pci_dev.device) {
    case PCI_DEVICE_ID_INTEL_QAT_C62X:
    adf_clean_hw_data_c62x(accel_dev.hw_device);
    break;
    default:
    break;
    }
    kfree(accel_dev.hw_device);
    accel_dev.hw_device = core::ptr::null_mut();
    }
    adf_dbgfs_exit(accel_dev);
    adf_cfg_dev_remove(accel_dev);
    adf_devmgr_rm_dev(accel_dev, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn adf_probe(pdev: *mut pci_dev, ent: *const pci_device_id) -> c_int {
    static int adf_probe(struct pci_dev *pdev, const struct pci_device_id *ent)
    {
    struct adf_accel_dev *accel_dev;
    struct adf_accel_pci *accel_pci_dev;
    struct adf_hw_device_data *hw_data;
    unsigned int i, bar_nr;
    unsigned long bar_mask;
    int ret;
    switch (ent.device) {
    case PCI_DEVICE_ID_INTEL_QAT_C62X:
    break;
    default:
    dev_err(&pdev.dev, "Invalid device 0x%x.\n", ent.device);
    return -ENODEV;
    }
    if (num_possible_nodes() > 1 && dev_to_node(&pdev.dev) < 0) {
// If the accelerator is connected to a node with no memory
// there is no point in using the accelerator since the remote
// memory transaction will be very slow.
    dev_err(&pdev.dev, "Invalid NUMA configuration.\n");
    return -EINVAL;
    }
    accel_dev = kzalloc_node(sizeof(*accel_dev), GFP_KERNEL,
    dev_to_node(&pdev.dev));
    if (!accel_dev)
    return -ENOMEM;
    INIT_LIST_HEAD(&accel_dev.crypto_list);
    accel_pci_dev = &accel_dev.accel_pci_dev;
    accel_pci_dev.pci_dev = pdev;
// Add accel device to accel table.
// This should be called before adf_cleanup_accel is called
    if (adf_devmgr_add_dev(accel_dev, core::ptr::null_mut())) {
    dev_err(&pdev.dev, "Failed to add new accelerator device.\n");
    kfree(accel_dev);
    return -EFAULT;
    }
    accel_dev.owner = THIS_MODULE;
// Allocate and configure device configuration structure
    hw_data = kzalloc_node(sizeof(*hw_data), GFP_KERNEL,
    dev_to_node(&pdev.dev));
    if (!hw_data) {
    ret = -ENOMEM;
    goto out_err;
    }
    accel_dev.hw_device = hw_data;
    adf_init_hw_data_c62x(accel_dev.hw_device);
    pci_read_config_byte(pdev, PCI_REVISION_ID, &accel_pci_dev.revid);
    pci_read_config_dword(pdev, ADF_DEVICE_FUSECTL_OFFSET,
    &hw_data.fuses[ADF_FUSECTL0]);
    pci_read_config_dword(pdev, ADF_C62X_SOFTSTRAP_CSR_OFFSET,
    &hw_data.straps);
// Get Accelerators and Accelerators Engines masks
    hw_data.accel_mask = hw_data.get_accel_mask(hw_data);
    hw_data.ae_mask = hw_data.get_ae_mask(hw_data);
    accel_pci_dev.sku = hw_data.get_sku(hw_data);
// If the device has no acceleration engines then ignore it.
    if (!hw_data.accel_mask || !hw_data.ae_mask ||
    ((~hw_data.ae_mask) & 0x01)) {
    dev_err(&pdev.dev, "No acceleration units found");
    ret = -EFAULT;
    goto out_err;
    }
// Create device configuration table
    ret = adf_cfg_dev_add(accel_dev);
    if (ret)
    goto out_err;
// enable PCI device
    if (pci_enable_device(pdev)) {
    ret = -EFAULT;
    goto out_err;
    }
// set dma identifier
    ret = dma_set_mask_and_coherent(&pdev.dev, DMA_BIT_MASK(48));
    if (ret) {
    dev_err(&pdev.dev, "No usable DMA configuration\n");
    goto out_err_disable;
    }
    if (pci_request_regions(pdev, ADF_C62X_DEVICE_NAME)) {
    ret = -EFAULT;
    goto out_err_disable;
    }
// Get accelerator capabilities mask
    hw_data.accel_capabilities_mask = hw_data.get_accel_cap(accel_dev);
// Find and map all the device's BARS
    i = (hw_data.fuses[ADF_FUSECTL0] & ADF_DEVICE_FUSECTL_MASK) ? 1 : 0;
    bar_mask = pci_select_bars(pdev, IORESOURCE_MEM);
    for_each_set_bit(bar_nr, &bar_mask, ADF_PCI_MAX_BARS * 2) {
    struct adf_bar *bar = &accel_pci_dev.pci_bars[i++];
    bar.base_addr = pci_resource_start(pdev, bar_nr);
    if (!bar.base_addr)
    break;
    bar.size = pci_resource_len(pdev, bar_nr);
    bar.virt_addr = pci_iomap(accel_pci_dev.pci_dev, bar_nr, 0);
    if (!bar.virt_addr) {
    pci_err(pdev, "Failed to map BAR %d\n", bar_nr);
    ret = -EFAULT;
    goto out_err_free_reg;
    }
    }
    if (pci_save_state(pdev)) {
    pci_err(pdev, "Failed to save pci state\n");
    ret = -ENOMEM;
    goto out_err_free_reg;
    }
    adf_dbgfs_init(accel_dev);
    ret = adf_dev_up(accel_dev, true);
    if (ret)
    goto out_err_dev_stop;
    return ret;
    out_err_dev_stop:
    adf_dev_down(accel_dev);
    out_err_free_reg:
    pci_release_regions(accel_pci_dev.pci_dev);
    out_err_disable:
    pci_disable_device(accel_pci_dev.pci_dev);
    out_err:
    adf_cleanup_accel(accel_dev);
    kfree(accel_dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn adf_remove(pdev: *mut pci_dev) {
    static void adf_remove(struct pci_dev *pdev)
    {
    struct adf_accel_dev *accel_dev = adf_devmgr_pci_to_accel_dev(pdev);
    if (!accel_dev) {
    pr_err("QAT: Driver removal failed\n");
    return;
    }
    adf_dev_down(accel_dev);
    adf_cleanup_accel(accel_dev);
    adf_cleanup_pci_dev(accel_dev);
    kfree(accel_dev);
    }
#[no_mangle]
unsafe extern "C" fn adf_shutdown(pdev: *mut pci_dev) {
    static void adf_shutdown(struct pci_dev *pdev)
    {
    struct adf_accel_dev *accel_dev = adf_devmgr_pci_to_accel_dev(pdev);
    adf_dev_down(accel_dev);
    }
    static const struct pci_device_id adf_pci_tbl[] = {
    { PCI_VDEVICE(INTEL, PCI_DEVICE_ID_INTEL_QAT_C62X) },
    { }
    };
    MODULE_DEVICE_TABLE(pci, adf_pci_tbl);
    static struct pci_driver adf_driver = {
    .id_table = adf_pci_tbl,
    .name = ADF_C62X_DEVICE_NAME,
    .probe = adf_probe,
    .remove = adf_remove,
    .shutdown = adf_shutdown,
    .sriov_configure = adf_sriov_configure,
    .err_handler = &adf_err_handler,
    };
#[no_mangle]
unsafe extern "C" fn adfdrv_init() -> int __init {
    static int __init adfdrv_init(void)
    {
    request_module("intel_qat");
    if (pci_register_driver(&adf_driver)) {
    pr_err("QAT: Driver initialization failed\n");
    return -EFAULT;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adfdrv_release() -> void __exit {
    static void __exit adfdrv_release(void)
    {
    pci_unregister_driver(&adf_driver);
    }
    module_init(adfdrv_init);
    module_exit(adfdrv_release);
    MODULE_LICENSE("Dual BSD/GPL");
    MODULE_AUTHOR("Intel");
    MODULE_FIRMWARE(ADF_C62X_FW);
    MODULE_FIRMWARE(ADF_C62X_MMP);
    MODULE_DESCRIPTION("Intel(R) QuickAssist Technology");
    MODULE_IMPORT_NS("CRYPTO_QAT");
