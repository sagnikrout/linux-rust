//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/cavium/nitrox/nitrox_sriov.c
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
// num_vfs_valid - validate VF count
// @num_vfs: number of VF(s)
//
#[no_mangle]
pub unsafe extern "C" fn num_vfs_valid(num_vfs: c_int) -> bool {
    static inline bool num_vfs_valid(int num_vfs)
    {
    let mut valid: bool = false;
    switch (num_vfs) {
    case 16:
    case 32:
    case 64:
    case 128:
    valid = true;
    break;
    }
    return valid;
    }
#[no_mangle]
pub unsafe extern "C" fn num_vfs_to_mode(num_vfs: c_int) -> enum vf_mode {
    static inline enum vf_mode num_vfs_to_mode(int num_vfs)
    {
    let mut mode: enum vf_mode = 0;
    switch (num_vfs) {
    case 0:
    mode = __NDEV_MODE_PF;
    break;
    case 16:
    mode = __NDEV_MODE_VF16;
    break;
    case 32:
    mode = __NDEV_MODE_VF32;
    break;
    case 64:
    mode = __NDEV_MODE_VF64;
    break;
    case 128:
    mode = __NDEV_MODE_VF128;
    break;
    }
    return mode;
    }
#[no_mangle]
pub unsafe extern "C" fn vf_mode_to_nr_queues(mode: enum vf_mode) -> c_int {
    static inline int vf_mode_to_nr_queues(enum vf_mode mode)
    {
    let mut nr_queues: c_int = 0;
    switch (mode) {
    case __NDEV_MODE_PF:
    nr_queues = MAX_PF_QUEUES;
    break;
    case __NDEV_MODE_VF16:
    nr_queues = 8;
    break;
    case __NDEV_MODE_VF32:
    nr_queues = 4;
    break;
    case __NDEV_MODE_VF64:
    nr_queues = 2;
    break;
    case __NDEV_MODE_VF128:
    nr_queues = 1;
    break;
    }
    return nr_queues;
    }
#[no_mangle]
unsafe extern "C" fn nitrox_pf_cleanup(ndev: *mut nitrox_device) {
    static void nitrox_pf_cleanup(struct nitrox_device *ndev)
    {
// PF has no queues in SR-IOV mode
    atomic_set(&ndev.state, __NDEV_NOT_READY);
// unregister crypto algorithms
    nitrox_crypto_unregister();
// cleanup PF resources
    nitrox_unregister_interrupts(ndev);
    nitrox_common_sw_cleanup(ndev);
    }
//
// nitrox_pf_reinit - re-initialize PF resources once SR-IOV is disabled
// @ndev: NITROX device
//
#[no_mangle]
unsafe extern "C" fn nitrox_pf_reinit(ndev: *mut nitrox_device) -> c_int {
    static int nitrox_pf_reinit(struct nitrox_device *ndev)
    {
    int err;
// allocate resources for PF
    err = nitrox_common_sw_init(ndev);
    if (err)
    return err;
    err = nitrox_register_interrupts(ndev);
    if (err) {
    nitrox_common_sw_cleanup(ndev);
    return err;
    }
// configure the AQM queues
    nitrox_config_aqm_rings(ndev);
// configure the packet queues
    nitrox_config_pkt_input_rings(ndev);
    nitrox_config_pkt_solicit_ports(ndev);
// set device to ready state
    atomic_set(&ndev.state, __NDEV_READY);
// register crypto algorithms
    return nitrox_crypto_register();
    }
#[no_mangle]
unsafe extern "C" fn nitrox_sriov_cleanup(ndev: *mut nitrox_device) {
    static void nitrox_sriov_cleanup(struct nitrox_device *ndev)
    {
// unregister interrupts for PF in SR-IOV
    nitrox_sriov_unregister_interrupts(ndev);
    nitrox_mbox_cleanup(ndev);
    }
#[no_mangle]
unsafe extern "C" fn nitrox_sriov_init(ndev: *mut nitrox_device) -> c_int {
    static int nitrox_sriov_init(struct nitrox_device *ndev)
    {
    int ret;
// register interrupts for PF in SR-IOV
    ret = nitrox_sriov_register_interupts(ndev);
    if (ret)
    return ret;
    ret = nitrox_mbox_init(ndev);
    if (ret)
    goto sriov_init_fail;
    return 0;
    sriov_init_fail:
    nitrox_sriov_cleanup(ndev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn nitrox_sriov_enable(pdev: *mut pci_dev, num_vfs: c_int) -> c_int {
    static int nitrox_sriov_enable(struct pci_dev *pdev, int num_vfs)
    {
    struct nitrox_device *ndev = pci_get_drvdata(pdev);
    int err;
    if (!num_vfs_valid(num_vfs)) {
    dev_err(DEV(ndev), "Invalid num_vfs %d\n", num_vfs);
    return -EINVAL;
    }
    if (pci_num_vf(pdev) == num_vfs)
    return num_vfs;
    err = pci_enable_sriov(pdev, num_vfs);
    if (err) {
    dev_err(DEV(ndev), "failed to enable PCI sriov %d\n", err);
    return err;
    }
    dev_info(DEV(ndev), "Enabled VF(s) %d\n", num_vfs);
    ndev.mode = num_vfs_to_mode(num_vfs);
    ndev.iov.num_vfs = num_vfs;
    ndev.iov.max_vf_queues = vf_mode_to_nr_queues(ndev.mode);
// set bit in flags
    set_bit(__NDEV_SRIOV_BIT, &ndev.flags);
// cleanup PF resources
    nitrox_pf_cleanup(ndev);
// PF SR-IOV mode initialization
    err = nitrox_sriov_init(ndev);
    if (err)
    goto iov_fail;
    config_nps_core_vfcfg_mode(ndev, ndev.mode);
    return num_vfs;
    iov_fail:
    pci_disable_sriov(pdev);
// clear bit in flags
    clear_bit(__NDEV_SRIOV_BIT, &ndev.flags);
    ndev.iov.num_vfs = 0;
    ndev.mode = __NDEV_MODE_PF;
// reset back to working mode in PF
    nitrox_pf_reinit(ndev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn nitrox_sriov_disable(pdev: *mut pci_dev) -> c_int {
    static int nitrox_sriov_disable(struct pci_dev *pdev)
    {
    struct nitrox_device *ndev = pci_get_drvdata(pdev);
    if (!test_bit(__NDEV_SRIOV_BIT, &ndev.flags))
    return 0;
    if (pci_vfs_assigned(pdev)) {
    dev_warn(DEV(ndev), "VFs are attached to VM. Can't disable SR-IOV\n");
    return -EPERM;
    }
    pci_disable_sriov(pdev);
// clear bit in flags
    clear_bit(__NDEV_SRIOV_BIT, &ndev.flags);
    ndev.iov.num_vfs = 0;
    ndev.iov.max_vf_queues = 0;
    ndev.mode = __NDEV_MODE_PF;
// cleanup PF SR-IOV resources
    nitrox_sriov_cleanup(ndev);
    config_nps_core_vfcfg_mode(ndev, ndev.mode);
    return nitrox_pf_reinit(ndev);
    }
#[no_mangle]
pub unsafe extern "C" fn nitrox_sriov_configure(pdev: *mut pci_dev, num_vfs: c_int) -> c_int {
    int nitrox_sriov_configure(struct pci_dev *pdev, int num_vfs)
    {
    if (!num_vfs)
    return nitrox_sriov_disable(pdev);
    return nitrox_sriov_enable(pdev, num_vfs);
    }
