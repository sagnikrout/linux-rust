//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/eeh_sysfs.c
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
// Sysfs entries for PCI Error Recovery for PAPR-compliant platform.
// Copyright IBM Corporation 2007
// Copyright Linas Vepstas <linas@austin.ibm.com> 2007
//
// Send comments and feedback to Linas Vepstas <linas@austin.ibm.com>
//

//
// EEH_SHOW_ATTR -- Create sysfs entry for eeh statistic
// @_name: name of file in sysfs directory
// @_memb: name of member in struct eeh_dev to access
// @_format: printf format for display
//
// All of the attributes look very similar, so just
// auto-gen a cut-n-paste routine to display them.
//

    static ssize_t eeh_show_##_name(struct device *dev,      \
    struct device_attribute *attr, char *buf)          \
    {                                                        \
    struct pci_dev *pdev = to_pci_dev(dev);               \
    struct eeh_dev *edev = pci_dev_to_eeh_dev(pdev);      \
    \
    if (!edev)                                            \
    return 0;                                     \
    \
    return sysfs_emit(buf, _format "\n", edev._memb);    \
    }                                                        \
    static DEVICE_ATTR(_name, 0444, eeh_show_##_name, core::ptr::null_mut());
    EEH_SHOW_ATTR(eeh_mode,            mode,            "0x%x");
    EEH_SHOW_ATTR(eeh_pe_config_addr,  pe_config_addr,  "0x%x");
    static ssize_t eeh_pe_state_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct pci_dev *pdev = to_pci_dev(dev);
    struct eeh_dev *edev = pci_dev_to_eeh_dev(pdev);
    int state;
    if (!edev || !edev.pe)
    return -ENODEV;
    state = eeh_ops.get_state(edev.pe, core::ptr::null_mut());
    return sysfs_emit(buf, "0x%08x 0x%08x\n", state, edev.pe.state);
    }
    static ssize_t eeh_pe_state_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct pci_dev *pdev = to_pci_dev(dev);
    struct eeh_dev *edev = pci_dev_to_eeh_dev(pdev);
    if (!edev || !edev.pe)
    return -ENODEV;
// Nothing to do if it's not frozen
    if (!(edev.pe.state & EEH_PE_ISOLATED))
    return count;
    if (eeh_unfreeze_pe(edev.pe))
    return -EIO;
    eeh_pe_state_clear(edev.pe, EEH_PE_ISOLATED, true);
    return count;
    }
    static DEVICE_ATTR_RW(eeh_pe_state);

    static ssize_t eeh_notify_resume_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct pci_dev *pdev = to_pci_dev(dev);
    struct eeh_dev *edev = pci_dev_to_eeh_dev(pdev);
    struct pci_dn *pdn = pci_get_pdn(pdev);
    if (!edev || !edev.pe)
    return -ENODEV;
    return sysfs_emit(buf, "%d\n", pdn.last_allow_rc);
    }
    static ssize_t eeh_notify_resume_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct pci_dev *pdev = to_pci_dev(dev);
    struct eeh_dev *edev = pci_dev_to_eeh_dev(pdev);
    if (!edev || !edev.pe || !eeh_ops.notify_resume)
    return -ENODEV;
    if (eeh_ops.notify_resume(edev))
    return -EIO;
    return count;
    }
    static DEVICE_ATTR_RW(eeh_notify_resume);
#[no_mangle]
unsafe extern "C" fn eeh_notify_resume_add(pdev: *mut pci_dev) -> c_int {
    static int eeh_notify_resume_add(struct pci_dev *pdev)
    {
    struct device_node *np;
    let mut rc: c_int = 0;
    np = pci_device_to_OF_node(pdev.is_physfn ? pdev : pdev.physfn);
    if (of_property_read_bool(np, "ibm,is-open-sriov-pf"))
    rc = device_create_file(&pdev.dev, &dev_attr_eeh_notify_resume);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn eeh_notify_resume_remove(pdev: *mut pci_dev) {
    static void eeh_notify_resume_remove(struct pci_dev *pdev)
    {
    struct device_node *np;
    np = pci_device_to_OF_node(pdev.is_physfn ? pdev : pdev.physfn);
    if (of_property_read_bool(np, "ibm,is-open-sriov-pf"))
    device_remove_file(&pdev.dev, &dev_attr_eeh_notify_resume);
    }

    static inline int eeh_notify_resume_add(struct pci_dev *pdev) { return 0; }
    static inline void eeh_notify_resume_remove(struct pci_dev *pdev) { }

#[no_mangle]
pub unsafe extern "C" fn eeh_sysfs_add_device(pdev: *mut pci_dev) {
    void eeh_sysfs_add_device(struct pci_dev *pdev)
    {
    struct eeh_dev *edev = pci_dev_to_eeh_dev(pdev);
    let mut rc: c_int = 0;
    if (!eeh_enabled())
    return;
    if (edev && (edev.mode & EEH_DEV_SYSFS))
    return;
    rc += device_create_file(&pdev.dev, &dev_attr_eeh_mode);
    rc += device_create_file(&pdev.dev, &dev_attr_eeh_pe_config_addr);
    rc += device_create_file(&pdev.dev, &dev_attr_eeh_pe_state);
    rc += eeh_notify_resume_add(pdev);
    if (rc)
    pr_warn("EEH: Unable to create sysfs entries\n");
#[no_mangle]
pub unsafe extern "C" fn if(_arg: edev) -> else {
    else if (edev)
    edev.mode |= EEH_DEV_SYSFS;
    }
#[no_mangle]
pub unsafe extern "C" fn eeh_sysfs_remove_device(pdev: *mut pci_dev) {
    void eeh_sysfs_remove_device(struct pci_dev *pdev)
    {
    struct eeh_dev *edev = pci_dev_to_eeh_dev(pdev);
    if (!edev) {
    WARN_ON(eeh_enabled());
    return;
    }
    edev.mode &= ~EEH_DEV_SYSFS;
//
// The parent directory might have been removed. We needn't
// continue for that case.
//
    if (!pdev.dev.kobj.sd)
    return;
    device_remove_file(&pdev.dev, &dev_attr_eeh_mode);
    device_remove_file(&pdev.dev, &dev_attr_eeh_pe_config_addr);
    device_remove_file(&pdev.dev, &dev_attr_eeh_pe_state);
    eeh_notify_resume_remove(pdev);
    }
