//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/pensando/ionic/ionic_bus_pci.c
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
// Copyright(c) 2017 - 2019 Pensando Systems, Inc

// Supported devices
    static const struct pci_device_id ionic_id_table[] = {
    { PCI_VDEVICE(PENSANDO, PCI_DEVICE_ID_PENSANDO_IONIC_ETH_PF) },
    { PCI_VDEVICE(PENSANDO, PCI_DEVICE_ID_PENSANDO_IONIC_ETH_VF) },
    { 0, }	/* end of table */
    };
    MODULE_DEVICE_TABLE(pci, ionic_id_table);
#[no_mangle]
pub unsafe extern "C" fn ionic_bus_get_irq(ionic: *mut ionic, num: c_uint) -> c_int {
    int ionic_bus_get_irq(struct ionic *ionic, unsigned int num)
    {
    return pci_irq_vector(ionic.pdev, num);
    }
    const char *ionic_bus_info(struct ionic *ionic)
    {
    return pci_name(ionic.pdev);
    }
#[no_mangle]
pub unsafe extern "C" fn ionic_bus_alloc_irq_vectors(ionic: *mut ionic, nintrs: c_uint) -> c_int {
    int ionic_bus_alloc_irq_vectors(struct ionic *ionic, unsigned int nintrs)
    {
    return pci_alloc_irq_vectors(ionic.pdev, nintrs, nintrs,
    PCI_IRQ_MSIX);
    }
#[no_mangle]
pub unsafe extern "C" fn ionic_bus_free_irq_vectors(ionic: *mut ionic) {
    void ionic_bus_free_irq_vectors(struct ionic *ionic)
    {
    if (!ionic.nintrs)
    return;
    pci_free_irq_vectors(ionic.pdev);
    }
#[no_mangle]
unsafe extern "C" fn ionic_map_bars(ionic: *mut ionic) -> c_int {
    static int ionic_map_bars(struct ionic *ionic)
    {
    struct pci_dev *pdev = ionic.pdev;
    struct device *dev = ionic.dev;
    struct ionic_dev_bar *bars;
    unsigned int i, j;
    bars = ionic.bars;
    ionic.num_bars = 0;
    for (i = 0, j = 0; i < IONIC_BARS_MAX; i++) {
    if (!(pci_resource_flags(pdev, i) & IORESOURCE_MEM))
    continue;
    bars[j].len = pci_resource_len(pdev, i);
// only map the whole bar 0
    if (j > 0) {
    bars[j].vaddr = core::ptr::null_mut();
    } else {
    bars[j].vaddr = pci_iomap(pdev, i, bars[j].len);
    if (!bars[j].vaddr) {
    dev_err(dev,
    "Cannot memory-map BAR %d, aborting\n",
    i);
    return -ENODEV;
    }
    }
    bars[j].bus_addr = pci_resource_start(pdev, i);
    bars[j].res_index = i;
    ionic.num_bars++;
    j++;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ionic_unmap_bars(ionic: *mut ionic) {
    static void ionic_unmap_bars(struct ionic *ionic)
    {
    struct ionic_dev_bar *bars = ionic.bars;
    unsigned int i;
    for (i = 0; i < IONIC_BARS_MAX; i++) {
    if (bars[i].vaddr) {
    iounmap(bars[i].vaddr);
    bars[i].bus_addr = 0;
    bars[i].vaddr = core::ptr::null_mut();
    bars[i].len = 0;
    }
    }
    ionic.num_bars = 0;
    }
    void __iomem *ionic_bus_map_dbpage(struct ionic *ionic, int page_num)
    {
    return pci_iomap_range(ionic.pdev,
    ionic.bars[IONIC_PCI_BAR_DBELL].res_index,
    (u64)page_num << PAGE_SHIFT, PAGE_SIZE);
    }
#[no_mangle]
pub unsafe extern "C" fn ionic_bus_unmap_dbpage(ionic: *mut ionic, page: *mut void __iomem) {
    void ionic_bus_unmap_dbpage(struct ionic *ionic, void __iomem *page)
    {
    iounmap(page);
    }
#[no_mangle]
unsafe extern "C" fn ionic_vf_dealloc_locked(ionic: *mut ionic) {
    static void ionic_vf_dealloc_locked(struct ionic *ionic)
    {
    let mut vfc: ionic_vf_setattr_cmd = { .attr = IONIC_VF_ATTR_STATSADDR };
    struct ionic_vf *v;
    int i;
    if (!ionic.vfs)
    return;
    for (i = ionic.num_vfs - 1; i >= 0; i--) {
    v = &ionic.vfs[i];
    if (v.stats_pa) {
    vfc.stats_pa = 0;
    ionic_set_vf_config(ionic, i, &vfc);
    dma_unmap_single(ionic.dev, v.stats_pa,
    sizeof(v.stats), DMA_FROM_DEVICE);
    v.stats_pa = 0;
    }
    }
    kfree(ionic.vfs);
    ionic.vfs = core::ptr::null_mut();
    ionic.num_vfs = 0;
    }
#[no_mangle]
unsafe extern "C" fn ionic_vf_dealloc(ionic: *mut ionic) {
    static void ionic_vf_dealloc(struct ionic *ionic)
    {
    down_write(&ionic.vf_op_lock);
    ionic_vf_dealloc_locked(ionic);
    up_write(&ionic.vf_op_lock);
    }
#[no_mangle]
unsafe extern "C" fn ionic_vf_alloc(ionic: *mut ionic, num_vfs: c_int) -> c_int {
    static int ionic_vf_alloc(struct ionic *ionic, int num_vfs)
    {
    let mut vfc: ionic_vf_setattr_cmd = { .attr = IONIC_VF_ATTR_STATSADDR };
    struct ionic_vf *v;
    let mut err: c_int = 0;
    int i;
    down_write(&ionic.vf_op_lock);
    ionic.vfs = kzalloc_objs(struct ionic_vf, num_vfs);
    if (!ionic.vfs) {
    err = -ENOMEM;
    goto out;
    }
    for (i = 0; i < num_vfs; i++) {
    v = &ionic.vfs[i];
    v.stats_pa = dma_map_single(ionic.dev, &v.stats,
    sizeof(v.stats), DMA_FROM_DEVICE);
    if (dma_mapping_error(ionic.dev, v.stats_pa)) {
    v.stats_pa = 0;
    err = -ENODEV;
    goto out;
    }
    ionic.num_vfs++;
// ignore failures from older FW, we just won't get stats
    vfc.stats_pa = cpu_to_le64(v.stats_pa);
    ionic_set_vf_config(ionic, i, &vfc);
    }
    out:
    if (err)
    ionic_vf_dealloc_locked(ionic);
    up_write(&ionic.vf_op_lock);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ionic_sriov_configure(pdev: *mut pci_dev, num_vfs: c_int) -> c_int {
    static int ionic_sriov_configure(struct pci_dev *pdev, int num_vfs)
    {
    struct ionic *ionic = pci_get_drvdata(pdev);
    struct device *dev = ionic.dev;
    let mut ret: c_int = 0;
    if (ionic.lif &&
    test_bit(IONIC_LIF_F_FW_RESET, ionic.lif.state))
    return -EBUSY;
    if (num_vfs > 0) {
    ret = pci_enable_sriov(pdev, num_vfs);
    if (ret) {
    dev_err(dev, "Cannot enable SRIOV: %d\n", ret);
    goto out;
    }
    ret = ionic_vf_alloc(ionic, num_vfs);
    if (ret) {
    dev_err(dev, "Cannot alloc VFs: %d\n", ret);
    pci_disable_sriov(pdev);
    goto out;
    }
    ret = num_vfs;
    } else {
    pci_disable_sriov(pdev);
    ionic_vf_dealloc(ionic);
    }
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ionic_clear_pci(ionic: *mut ionic) {
    static void ionic_clear_pci(struct ionic *ionic)
    {
    if (ionic.num_bars) {
    ionic.idev.dev_info_regs = core::ptr::null_mut();
    ionic.idev.dev_cmd_regs = core::ptr::null_mut();
    ionic.idev.intr_status = core::ptr::null_mut();
    ionic.idev.intr_ctrl = core::ptr::null_mut();
    ionic_unmap_bars(ionic);
    pci_release_regions(ionic.pdev);
    }
    if (pci_is_enabled(ionic.pdev))
    pci_disable_device(ionic.pdev);
    }
#[no_mangle]
unsafe extern "C" fn ionic_setup_one(ionic: *mut ionic) -> c_int {
    static int ionic_setup_one(struct ionic *ionic)
    {
    struct pci_dev *pdev = ionic.pdev;
    struct device *dev = ionic.dev;
    int err;
    ionic_debugfs_add_dev(ionic);
// Setup PCI device
    err = pci_enable_device_mem(pdev);
    if (err) {
    dev_err(dev, "Cannot enable PCI device: %d, aborting\n", err);
    goto err_out_debugfs_del_dev;
    }
    err = pci_request_regions(pdev, IONIC_DRV_NAME);
    if (err) {
    dev_err(dev, "Cannot request PCI regions: %d, aborting\n", err);
    goto err_out_clear_pci;
    }
    pcie_print_link_status(pdev);
    err = ionic_map_bars(ionic);
    if (err)
    goto err_out_clear_pci;
// Configure the device
    err = ionic_setup(ionic);
    if (err) {
    dev_err(dev, "Cannot setup device: %d, aborting\n", err);
    goto err_out_clear_pci;
    }
    pci_set_master(pdev);
    err = ionic_identify(ionic);
    if (err) {
    dev_err(dev, "Cannot identify device: %d, aborting\n", err);
    goto err_out_teardown;
    }
    ionic_debugfs_add_ident(ionic);
    ionic_map_cmb(ionic);
    err = ionic_init(ionic);
    if (err) {
    dev_err(dev, "Cannot init device: %d, aborting\n", err);
    goto err_out_teardown;
    }
// Configure the port
    err = ionic_port_identify(ionic);
    if (err) {
    dev_err(dev, "Cannot identify port: %d, aborting\n", err);
    goto err_out_teardown;
    }
    err = ionic_port_init(ionic);
    if (err) {
    dev_err(dev, "Cannot init port: %d, aborting\n", err);
    goto err_out_teardown;
    }
    return 0;
    err_out_teardown:
    ionic_dev_teardown(ionic);
    err_out_clear_pci:
    ionic_clear_pci(ionic);
    err_out_debugfs_del_dev:
    ionic_debugfs_del_dev(ionic);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ionic_probe(pdev: *mut pci_dev, ent: *const pci_device_id) -> c_int {
    static int ionic_probe(struct pci_dev *pdev, const struct pci_device_id *ent)
    {
    struct device *dev = &pdev.dev;
    struct ionic *ionic;
    int num_vfs;
    int err;
    ionic = ionic_devlink_alloc(dev);
    if (!ionic)
    return -ENOMEM;
    ionic.pdev = pdev;
    ionic.dev = dev;
    pci_set_drvdata(pdev, ionic);
    mutex_init(&ionic.dev_cmd_lock);
// Query system for DMA addressing limitation for the device.
    err = dma_set_mask_and_coherent(dev, DMA_BIT_MASK(IONIC_ADDR_LEN));
    if (err) {
    dev_err(dev, "Unable to obtain 64-bit DMA for consistent allocations, aborting.  err=%d\n",
    err);
    goto err_out;
    }

// Ensure MSI/MSI-X interrupts lie within addressable physical memory
    pdev.msi_addr_mask = DMA_BIT_MASK(32);

    err = ionic_setup_one(ionic);
    if (err)
    goto err_out;
// Allocate and init the LIF
    err = ionic_lif_size(ionic);
    if (err) {
    dev_err(dev, "Cannot size LIF: %d, aborting\n", err);
    goto err_out_pci;
    }
    err = ionic_lif_alloc(ionic);
    if (err) {
    dev_err(dev, "Cannot allocate LIF: %d, aborting\n", err);
    goto err_out_free_irqs;
    }
    err = ionic_lif_init(ionic.lif);
    if (err) {
    dev_err(dev, "Cannot init LIF: %d, aborting\n", err);
    goto err_out_free_lifs;
    }
    init_rwsem(&ionic.vf_op_lock);
    num_vfs = pci_num_vf(pdev);
    if (num_vfs) {
    dev_info(dev, "%d VFs found already enabled\n", num_vfs);
    err = ionic_vf_alloc(ionic, num_vfs);
    if (err)
    dev_err(dev, "Cannot enable existing VFs: %d\n", err);
    }
    err = ionic_devlink_register(ionic);
    if (err) {
    dev_err(dev, "Cannot register devlink: %d\n", err);
    goto err_out_deinit_lifs;
    }
    err = ionic_lif_register(ionic.lif);
    if (err) {
    dev_err(dev, "Cannot register LIF: %d, aborting\n", err);
    goto err_out_deregister_devlink;
    }
    ionic_auxbus_register(ionic.lif);
    mod_timer(&ionic.watchdog_timer,
    round_jiffies(jiffies + ionic.watchdog_period));
    ionic_queue_doorbell_check(ionic, IONIC_NAPI_DEADLINE);
    return 0;
    err_out_deregister_devlink:
    ionic_devlink_unregister(ionic);
    err_out_deinit_lifs:
    ionic_vf_dealloc(ionic);
    ionic_lif_deinit(ionic.lif);
    err_out_free_lifs:
    ionic_lif_free(ionic.lif);
    ionic.lif = core::ptr::null_mut();
    err_out_free_irqs:
    ionic_bus_free_irq_vectors(ionic);
    err_out_pci:
    ionic_dev_teardown(ionic);
    ionic_clear_pci(ionic);
    ionic_debugfs_del_dev(ionic);
    err_out:
    mutex_destroy(&ionic.dev_cmd_lock);
    ionic_devlink_free(ionic);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ionic_remove(pdev: *mut pci_dev) {
    static void ionic_remove(struct pci_dev *pdev)
    {
    struct ionic *ionic = pci_get_drvdata(pdev);
    timer_shutdown_sync(&ionic.watchdog_timer);
    if (ionic.lif) {
    cancel_work_sync(&ionic.lif.deferred.work);
// prevent adminq cmds if already known as down
    if (test_and_clear_bit(IONIC_LIF_F_FW_RESET, ionic.lif.state))
    set_bit(IONIC_LIF_F_FW_STOPPING, ionic.lif.state);
    if (ionic.lif.doorbell_wa)
    cancel_delayed_work_sync(&ionic.doorbell_check_dwork);
    ionic_auxbus_unregister(ionic.lif);
    ionic_lif_unregister(ionic.lif);
    ionic_devlink_unregister(ionic);
    ionic_lif_deinit(ionic.lif);
    ionic_lif_free(ionic.lif);
    ionic.lif = core::ptr::null_mut();
    ionic_bus_free_irq_vectors(ionic);
    }
    ionic_port_reset(ionic);
    ionic_reset(ionic);
    ionic_dev_teardown(ionic);
    ionic_clear_pci(ionic);
    ionic_debugfs_del_dev(ionic);
    mutex_destroy(&ionic.dev_cmd_lock);
    ionic_devlink_free(ionic);
    }
#[no_mangle]
unsafe extern "C" fn ionic_reset_prepare(pdev: *mut pci_dev) {
    static void ionic_reset_prepare(struct pci_dev *pdev)
    {
    struct ionic *ionic = pci_get_drvdata(pdev);
    struct ionic_lif *lif = ionic.lif;
    dev_dbg(ionic.dev, "%s: device stopping\n", __func__);
    set_bit(IONIC_LIF_F_FW_RESET, lif.state);
    timer_delete_sync(&ionic.watchdog_timer);
    cancel_work_sync(&lif.deferred.work);
    ionic_auxbus_unregister(ionic.lif);
    mutex_lock(&lif.queue_lock);
    ionic_stop_queues_reconfig(lif);
    ionic_txrx_free(lif);
    ionic_lif_deinit(lif);
    ionic_qcqs_free(lif);
    ionic_debugfs_del_lif(lif);
    mutex_unlock(&lif.queue_lock);
    ionic_dev_teardown(ionic);
    ionic_clear_pci(ionic);
    ionic_debugfs_del_dev(ionic);
    }
#[no_mangle]
unsafe extern "C" fn ionic_reset_done(pdev: *mut pci_dev) {
    static void ionic_reset_done(struct pci_dev *pdev)
    {
    struct ionic *ionic = pci_get_drvdata(pdev);
    struct ionic_lif *lif = ionic.lif;
    int err;
    err = ionic_setup_one(ionic);
    if (err)
    goto err_out;
    ionic_debugfs_add_sizes(ionic);
    ionic_debugfs_add_lif(ionic.lif);
    err = ionic_restart_lif(lif);
    if (err)
    goto err_out;
    mod_timer(&ionic.watchdog_timer, jiffies + 1);
    err_out:
    dev_dbg(ionic.dev, "%s: device recovery %s\n",
    __func__, err ? "failed" : "done");
    }
    static pci_ers_result_t ionic_pci_error_detected(struct pci_dev *pdev,
    pci_channel_state_t error)
    {
    if (error == pci_channel_io_frozen) {
    ionic_reset_prepare(pdev);
    return PCI_ERS_RESULT_NEED_RESET;
    }
    return PCI_ERS_RESULT_NONE;
    }
#[no_mangle]
unsafe extern "C" fn ionic_pci_error_resume(pdev: *mut pci_dev) {
    static void ionic_pci_error_resume(struct pci_dev *pdev)
    {
    struct ionic *ionic = pci_get_drvdata(pdev);
    struct ionic_lif *lif = ionic.lif;
    if (lif && test_bit(IONIC_LIF_F_FW_RESET, lif.state))
    pci_reset_function_locked(pdev);
    }
    static const struct pci_error_handlers ionic_err_handler = {
// FLR handling
    .reset_prepare      = ionic_reset_prepare,
    .reset_done         = ionic_reset_done,
// PCI bus error detected on this device
    .error_detected     = ionic_pci_error_detected,
    .resume		    = ionic_pci_error_resume,
    };
    static struct pci_driver ionic_driver = {
    .name = IONIC_DRV_NAME,
    .id_table = ionic_id_table,
    .probe = ionic_probe,
    .remove = ionic_remove,
    .sriov_configure = ionic_sriov_configure,
    .err_handler = &ionic_err_handler
    };
#[no_mangle]
pub unsafe extern "C" fn ionic_bus_register_driver() -> c_int {
    int ionic_bus_register_driver(void)
    {
    return pci_register_driver(&ionic_driver);
    }
#[no_mangle]
pub unsafe extern "C" fn ionic_bus_unregister_driver() {
    void ionic_bus_unregister_driver(void)
    {
    pci_unregister_driver(&ionic_driver);
    }
