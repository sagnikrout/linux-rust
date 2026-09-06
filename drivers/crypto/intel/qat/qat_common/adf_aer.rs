//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/intel/qat/qat_common/adf_aer.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_fatal_error_data {
    pub accel_dev: *mut adf_accel_dev,
    pub work: work_struct,
}

    static struct workqueue_struct *device_reset_wq;
    static struct workqueue_struct *device_sriov_wq;
#[no_mangle]
unsafe extern "C" fn reset_prepare(pdev: *mut pci_dev) -> pci_ers_result_t {
    static pci_ers_result_t reset_prepare(struct pci_dev *pdev)
    {
    struct adf_accel_dev *accel_dev = adf_devmgr_pci_to_accel_dev(pdev);
    if (!accel_dev) {
    pci_err(pdev, "Can't find acceleration device\n");
    return PCI_ERS_RESULT_DISCONNECT;
    }
    if (!adf_dev_started(accel_dev))
    return PCI_ERS_RESULT_CAN_RECOVER;
    set_bit(ADF_STATUS_RESTARTING, &accel_dev.status);
    if (accel_dev.hw_device.exit_arb) {
    dev_dbg(&pdev.dev, "Disabling arbitration\n");
    accel_dev.hw_device.exit_arb(accel_dev);
    }
    adf_dev_restarting_notify(accel_dev);
    adf_dev_down(accel_dev);
    return PCI_ERS_RESULT_NEED_RESET;
    }
#[no_mangle]
unsafe extern "C" fn reset_done(pdev: *mut pci_dev) -> pci_ers_result_t {
    static pci_ers_result_t reset_done(struct pci_dev *pdev)
    {
    struct adf_accel_dev *accel_dev = adf_devmgr_pci_to_accel_dev(pdev);
    int res;
    if (!accel_dev) {
    pci_err(pdev, "Can't find acceleration device\n");
    return PCI_ERS_RESULT_DISCONNECT;
    }
    if (!adf_devmgr_in_reset(accel_dev))
    goto reset_complete;
    pci_restore_state(pdev);
    res = adf_dev_up(accel_dev, false);
    if (res && res != -EALREADY)
    return PCI_ERS_RESULT_DISCONNECT;
    adf_reenable_sriov(accel_dev);
    adf_pf2vf_notify_restarted(accel_dev);
    adf_dev_restarted_notify(accel_dev);
    clear_bit(ADF_STATUS_RESTARTING, &accel_dev.status);
    reset_complete:
    pci_info(pdev, "Device reset completed successfully\n");
    return PCI_ERS_RESULT_RECOVERED;
    }
    static pci_ers_result_t adf_error_detected(struct pci_dev *pdev,
    pci_channel_state_t state)
    {
    struct adf_accel_dev *accel_dev = adf_devmgr_pci_to_accel_dev(pdev);
    pci_info(pdev, "Acceleration driver hardware error detected.\n");
    if (!accel_dev) {
    pci_err(pdev, "Can't find acceleration device\n");
    return PCI_ERS_RESULT_DISCONNECT;
    }
    if (state == pci_channel_io_perm_failure) {
    pci_err(pdev, "Can't recover from device error\n");
    return PCI_ERS_RESULT_DISCONNECT;
    }
    adf_error_notifier(accel_dev);
    adf_pf2vf_notify_fatal_error(accel_dev);
    return reset_prepare(pdev);
    }
// reset dev data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_reset_dev_data {
    pub mode: c_int,
    pub accel_dev: *mut adf_accel_dev,
    pub compl: completion,
    pub reset_work: work_struct,
}

// sriov dev data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_sriov_dev_data {
    pub accel_dev: *mut adf_accel_dev,
    pub compl: completion,
    pub sriov_work: work_struct,
}

#[no_mangle]
pub unsafe extern "C" fn adf_reset_sbr(accel_dev: *mut adf_accel_dev) {
    void adf_reset_sbr(struct adf_accel_dev *accel_dev)
    {
    struct pci_dev *pdev = accel_to_pci_dev(accel_dev);
    struct pci_dev *parent = pdev.bus.self;
    let mut bridge_ctl: u16 = 0;
    if (!parent)
    parent = pdev;
    if (!pci_wait_for_pending_transaction(pdev))
    pci_info(pdev, "Transaction still in progress. Proceeding\n");
    pci_info(pdev, "Secondary bus reset\n");
    pci_read_config_word(parent, PCI_BRIDGE_CONTROL, &bridge_ctl);
    bridge_ctl |= PCI_BRIDGE_CTL_BUS_RESET;
    pci_write_config_word(parent, PCI_BRIDGE_CONTROL, bridge_ctl);
    msleep(100);
    bridge_ctl &= ~PCI_BRIDGE_CTL_BUS_RESET;
    pci_write_config_word(parent, PCI_BRIDGE_CONTROL, bridge_ctl);
    msleep(100);
    }
    EXPORT_SYMBOL_GPL(adf_reset_sbr);
#[no_mangle]
pub unsafe extern "C" fn adf_reset_flr(accel_dev: *mut adf_accel_dev) {
    void adf_reset_flr(struct adf_accel_dev *accel_dev)
    {
    pcie_flr(accel_to_pci_dev(accel_dev));
    }
    EXPORT_SYMBOL_GPL(adf_reset_flr);
#[no_mangle]
pub unsafe extern "C" fn adf_dev_restore(accel_dev: *mut adf_accel_dev) {
    void adf_dev_restore(struct adf_accel_dev *accel_dev)
    {
    struct adf_hw_device_data *hw_device = accel_dev.hw_device;
    struct pci_dev *pdev = accel_to_pci_dev(accel_dev);
    if (hw_device.reset_device) {
    dev_info(&GET_DEV(accel_dev), "Resetting device qat_dev%d\n",
    accel_dev.accel_id);
    hw_device.reset_device(accel_dev);
    pci_restore_state(pdev);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn adf_set_bme(accel_dev: *mut adf_accel_dev) {
    void adf_set_bme(struct adf_accel_dev *accel_dev)
    {
    struct pci_dev *pdev = accel_to_pci_dev(accel_dev);
    pci_set_master(pdev);
    }
#[no_mangle]
unsafe extern "C" fn adf_device_sriov_worker(work: *mut work_struct) {
    static void adf_device_sriov_worker(struct work_struct *work)
    {
    struct adf_sriov_dev_data *sriov_data =
    container_of(work, struct adf_sriov_dev_data, sriov_work);
    adf_reenable_sriov(sriov_data.accel_dev);
    complete(&sriov_data.compl);
    }
#[no_mangle]
unsafe extern "C" fn adf_device_reset_worker(work: *mut work_struct) {
    static void adf_device_reset_worker(struct work_struct *work)
    {
    struct adf_reset_dev_data *reset_data =
    container_of(work, struct adf_reset_dev_data, reset_work);
    struct adf_accel_dev *accel_dev = reset_data.accel_dev;
    let mut wait_jiffies: c_ulong = msecs_to_jiffies(10000);
    struct adf_sriov_dev_data sriov_data;
    adf_dev_restarting_notify(accel_dev);
    if (adf_dev_restart(accel_dev)) {
// The device hanged and we can't restart it so stop here
    dev_err(&GET_DEV(accel_dev), "Restart device failed\n");
    if (reset_data.mode == ADF_DEV_RESET_ASYNC)
    kfree(reset_data);
    WARN(1, "QAT: device restart failed. Device is unusable\n");
    return;
    }
    sriov_data.accel_dev = accel_dev;
    init_completion(&sriov_data.compl);
    INIT_WORK(&sriov_data.sriov_work, adf_device_sriov_worker);
    queue_work(device_sriov_wq, &sriov_data.sriov_work);
    if (wait_for_completion_timeout(&sriov_data.compl, wait_jiffies))
    adf_pf2vf_notify_restarted(accel_dev);
    else
    cancel_work_sync(&sriov_data.sriov_work);
    adf_dev_restarted_notify(accel_dev);
    clear_bit(ADF_STATUS_RESTARTING, &accel_dev.status);
// The dev is back alive. Notify the caller if in sync mode
    if (reset_data.mode == ADF_DEV_RESET_ASYNC)
    kfree(reset_data);
    else
    complete(&reset_data.compl);
    }
    static int adf_dev_aer_schedule_reset(struct adf_accel_dev *accel_dev,
    enum adf_dev_reset_mode mode)
    {
    struct adf_reset_dev_data *reset_data;
    if (!adf_dev_started(accel_dev) ||
    test_and_set_bit(ADF_STATUS_RESTARTING, &accel_dev.status))
    return 0;
    reset_data = kzalloc_obj(*reset_data);
    if (!reset_data) {
    clear_bit(ADF_STATUS_RESTARTING, &accel_dev.status);
    return -ENOMEM;
    }
    reset_data.accel_dev = accel_dev;
    init_completion(&reset_data.compl);
    reset_data.mode = mode;
    INIT_WORK(&reset_data.reset_work, adf_device_reset_worker);
    queue_work(device_reset_wq, &reset_data.reset_work);
// If in sync mode wait for the result
    if (mode == ADF_DEV_RESET_SYNC) {
    let mut ret: c_int = 0;
// Maximum device reset time is 10 seconds
    let mut wait_jiffies: c_ulong = msecs_to_jiffies(10000);
    unsigned long timeout = wait_for_completion_timeout(
    &reset_data.compl, wait_jiffies);
    if (!timeout) {
    dev_err(&GET_DEV(accel_dev),
    "Reset device timeout expired\n");
    cancel_work_sync(&reset_data.reset_work);
    ret = -EFAULT;
    }
    kfree(reset_data);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adf_slot_reset(pdev: *mut pci_dev) -> pci_ers_result_t {
    static pci_ers_result_t adf_slot_reset(struct pci_dev *pdev)
    {
    return reset_done(pdev);
    }
#[no_mangle]
unsafe extern "C" fn adf_resume(pdev: *mut pci_dev) {
    static void adf_resume(struct pci_dev *pdev)
    {
    pci_info(pdev, "Acceleration driver reset completed\n");
    pci_info(pdev, "Device is up and running\n");
    }
#[no_mangle]
unsafe extern "C" fn adf_reset_prepare(pdev: *mut pci_dev) {
    static void adf_reset_prepare(struct pci_dev *pdev)
    {
    reset_prepare(pdev);
    }
#[no_mangle]
unsafe extern "C" fn adf_reset_done(pdev: *mut pci_dev) {
    static void adf_reset_done(struct pci_dev *pdev)
    {
    reset_done(pdev);
    }
    const struct pci_error_handlers adf_err_handler = {
    .error_detected = adf_error_detected,
    .slot_reset = adf_slot_reset,
    .resume = adf_resume,
    .reset_prepare = adf_reset_prepare,
    .reset_done = adf_reset_done,
    };
    EXPORT_SYMBOL_GPL(adf_err_handler);
#[no_mangle]
unsafe extern "C" fn adf_dev_autoreset(accel_dev: *mut adf_accel_dev) -> c_int {
    static int adf_dev_autoreset(struct adf_accel_dev *accel_dev)
    {
    if (accel_dev.autoreset_on_error)
    return adf_dev_aer_schedule_reset(accel_dev, ADF_DEV_RESET_ASYNC);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adf_notify_fatal_error_worker(work: *mut work_struct) {
    static void adf_notify_fatal_error_worker(struct work_struct *work)
    {
    struct adf_fatal_error_data *wq_data =
    container_of(work, struct adf_fatal_error_data, work);
    struct adf_accel_dev *accel_dev = wq_data.accel_dev;
    struct adf_hw_device_data *hw_device = accel_dev.hw_device;
    adf_error_notifier(accel_dev);
    if (!accel_dev.is_vf) {
// Disable arbitration to stop processing of new requests
    if (accel_dev.autoreset_on_error && hw_device.exit_arb)
    hw_device.exit_arb(accel_dev);
    if (accel_dev.pf.vf_info)
    adf_pf2vf_notify_fatal_error(accel_dev);
    adf_dev_autoreset(accel_dev);
    }
    kfree(wq_data);
    }
#[no_mangle]
pub unsafe extern "C" fn adf_notify_fatal_error(accel_dev: *mut adf_accel_dev) -> c_int {
    int adf_notify_fatal_error(struct adf_accel_dev *accel_dev)
    {
    struct adf_fatal_error_data *wq_data;
    wq_data = kzalloc_obj(*wq_data, GFP_ATOMIC);
    if (!wq_data)
    return -ENOMEM;
    wq_data.accel_dev = accel_dev;
    INIT_WORK(&wq_data.work, adf_notify_fatal_error_worker);
    adf_misc_wq_queue_work(&wq_data.work);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn adf_init_aer() -> c_int {
    int adf_init_aer(void)
    {
    device_reset_wq = alloc_workqueue("qat_device_reset_wq",
    WQ_MEM_RECLAIM | WQ_PERCPU, 0);
    if (!device_reset_wq)
    return -EFAULT;
    device_sriov_wq = alloc_workqueue("qat_device_sriov_wq", WQ_PERCPU, 0);
    if (!device_sriov_wq) {
    destroy_workqueue(device_reset_wq);
    device_reset_wq = core::ptr::null_mut();
    return -EFAULT;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn adf_exit_aer() {
    void adf_exit_aer(void)
    {
    if (device_reset_wq)
    destroy_workqueue(device_reset_wq);
    device_reset_wq = core::ptr::null_mut();
    if (device_sriov_wq)
    destroy_workqueue(device_sriov_wq);
    device_sriov_wq = core::ptr::null_mut();
    }
