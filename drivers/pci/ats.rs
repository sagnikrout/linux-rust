//! Automatically rewritten from C to Rust
//! Source: drivers/pci/ats.c
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
// PCI Express I/O Virtualization (IOV) support
// Address Translation Service 1.0
// Page Request Interface added by Joerg Roedel <joerg.roedel@amd.com>
// PASID support added by Joerg Roedel <joerg.roedel@amd.com>
//
// Copyright (C) 2009 Intel Corporation, Yu Zhao <yu.zhao@intel.com>
// Copyright (C) 2011 Advanced Micro Devices,
//

#[no_mangle]
pub unsafe extern "C" fn pci_ats_init(dev: *mut pci_dev) {
    void pci_ats_init(struct pci_dev *dev)
    {
    int pos;
    if (pci_ats_disabled())
    return;
    pos = pci_find_ext_capability(dev, PCI_EXT_CAP_ID_ATS);
    if (!pos)
    return;
    dev.ats_cap = pos;
    }
//
// pci_ats_supported - check if the device can use ATS
// @dev: the PCI device
//
// Returns true if the device supports ATS and is allowed to use it, false
// otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn pci_ats_supported(dev: *mut pci_dev) -> bool {
    bool pci_ats_supported(struct pci_dev *dev)
    {
    if (!dev.ats_cap || dev.untrusted)
    return false;
    if (dev.is_virtfn)
    return pci_ats_supported(pci_physfn(dev));
    return true;
    }
    EXPORT_SYMBOL_GPL(pci_ats_supported);
//
// pci_prepare_ats - Setup the PS for ATS
// @dev: the PCI device
// @ps: the IOMMU page shift
//
// This must be done by the IOMMU driver on the PF before any VFs are created to
// ensure that the VF can have ATS enabled.
//
// Returns 0 on success, or negative on failure.
//
#[no_mangle]
pub unsafe extern "C" fn pci_prepare_ats(dev: *mut pci_dev, ps: c_int) -> c_int {
    int pci_prepare_ats(struct pci_dev *dev, int ps)
    {
    u16 ctrl;
    if (!pci_ats_supported(dev))
    return -EINVAL;
    if (WARN_ON(dev.ats_enabled))
    return -EBUSY;
    if (ps < PCI_ATS_MIN_STU)
    return -EINVAL;
    if (dev.is_virtfn) {
    if (pci_physfn(dev).ats_stu != ps)
    return -EINVAL;
    return 0;
    }
    dev.ats_stu = ps;
    ctrl = PCI_ATS_CTRL_STU(dev.ats_stu - PCI_ATS_MIN_STU);
    pci_write_config_word(dev, dev.ats_cap + PCI_ATS_CTRL, ctrl);
    return 0;
    }
    EXPORT_SYMBOL_GPL(pci_prepare_ats);
//
// pci_enable_ats - enable the ATS capability
// @dev: the PCI device
// @ps: the IOMMU page shift
//
// Returns 0 on success, or negative on failure.
//
#[no_mangle]
pub unsafe extern "C" fn pci_enable_ats(dev: *mut pci_dev, ps: c_int) -> c_int {
    int pci_enable_ats(struct pci_dev *dev, int ps)
    {
    u16 ctrl;
    struct pci_dev *pdev;
    if (!pci_ats_supported(dev))
    return -EINVAL;
    if (WARN_ON(dev.ats_enabled))
    return -EBUSY;
    if (ps < PCI_ATS_MIN_STU)
    return -EINVAL;
//
// Note that enabling ATS on a VF fails unless it's already enabled
// with the same STU on the PF.
//
    ctrl = PCI_ATS_CTRL_ENABLE;
    if (dev.is_virtfn) {
    pdev = pci_physfn(dev);
    if (pdev.ats_stu != ps)
    return -EINVAL;
    } else {
    dev.ats_stu = ps;
    ctrl |= PCI_ATS_CTRL_STU(dev.ats_stu - PCI_ATS_MIN_STU);
    }
    pci_write_config_word(dev, dev.ats_cap + PCI_ATS_CTRL, ctrl);
    dev.ats_enabled = 1;
    return 0;
    }
    EXPORT_SYMBOL_GPL(pci_enable_ats);
//
// pci_disable_ats - disable the ATS capability
// @dev: the PCI device
//
#[no_mangle]
pub unsafe extern "C" fn pci_disable_ats(dev: *mut pci_dev) {
    void pci_disable_ats(struct pci_dev *dev)
    {
    u16 ctrl;
    if (WARN_ON(!dev.ats_enabled))
    return;
    pci_read_config_word(dev, dev.ats_cap + PCI_ATS_CTRL, &ctrl);
    ctrl &= ~PCI_ATS_CTRL_ENABLE;
    pci_write_config_word(dev, dev.ats_cap + PCI_ATS_CTRL, ctrl);
    dev.ats_enabled = 0;
    }
    EXPORT_SYMBOL_GPL(pci_disable_ats);
#[no_mangle]
pub unsafe extern "C" fn pci_restore_ats_state(dev: *mut pci_dev) {
    void pci_restore_ats_state(struct pci_dev *dev)
    {
    u16 ctrl;
    if (!dev.ats_enabled)
    return;
    ctrl = PCI_ATS_CTRL_ENABLE;
    if (!dev.is_virtfn)
    ctrl |= PCI_ATS_CTRL_STU(dev.ats_stu - PCI_ATS_MIN_STU);
    pci_write_config_word(dev, dev.ats_cap + PCI_ATS_CTRL, ctrl);
    }
//
// pci_ats_queue_depth - query the ATS Invalidate Queue Depth
// @dev: the PCI device
//
// Returns the queue depth on success, or negative on failure.
//
// The ATS spec uses 0 in the Invalidate Queue Depth field to
// indicate that the function can accept 32 Invalidate Request.
// But here we use the `real' values (i.e. 1~32) for the Queue
// Depth; and 0 indicates the function shares the Queue with
// other functions (doesn't exclusively own a Queue).
//
#[no_mangle]
pub unsafe extern "C" fn pci_ats_queue_depth(dev: *mut pci_dev) -> c_int {
    int pci_ats_queue_depth(struct pci_dev *dev)
    {
    u16 cap;
    if (!dev.ats_cap)
    return -EINVAL;
    if (dev.is_virtfn)
    return 0;
    pci_read_config_word(dev, dev.ats_cap + PCI_ATS_CAP, &cap);
    return PCI_ATS_CAP_QDEP(cap) ? PCI_ATS_CAP_QDEP(cap) : PCI_ATS_MAX_QDEP;
    }
//
// pci_ats_page_aligned - Return Page Aligned Request bit status.
// @pdev: the PCI device
//
// Returns 1, if the Untranslated Addresses generated by the device
// are always aligned or 0 otherwise.
//
// Per PCIe spec r4.0, sec 10.5.1.2, if the Page Aligned Request bit
// is set, it indicates the Untranslated Addresses generated by the
// device are always aligned to a 4096 byte boundary.
//
#[no_mangle]
pub unsafe extern "C" fn pci_ats_page_aligned(pdev: *mut pci_dev) -> c_int {
    int pci_ats_page_aligned(struct pci_dev *pdev)
    {
    u16 cap;
    if (!pdev.ats_cap)
    return 0;
    pci_read_config_word(pdev, pdev.ats_cap + PCI_ATS_CAP, &cap);
    if (cap & PCI_ATS_CAP_PAGE_ALIGNED)
    return 1;
    return 0;
    }
//
// CXL r4.0, sec 3.2.5.13 Memory Type on CXL.cache notes: to source requests on
// CXL.cache, devices need to get the Host Physical Address (HPA) from the Host
// by means of an ATS request on CXL.io.
//
// In other words, CXL.cache devices cannot access host physical memory without
// ATS.
//
// Check Cache_Capable instead of Cache_Enable because CXL.cache may be enabled
// after the caller uses this to make its ATS decision.
//
#[no_mangle]
unsafe extern "C" fn pci_cxl_ats_required(pdev: *mut pci_dev) -> bool {
    static bool pci_cxl_ats_required(struct pci_dev *pdev)
    {
    int offset;
    u16 cap;
    offset = pci_find_dvsec_capability(pdev, PCI_VENDOR_ID_CXL,
    PCI_DVSEC_CXL_DEVICE);
    if (!offset)
    return false;
    if (pci_read_config_word(pdev, offset + PCI_DVSEC_CXL_CAP, &cap))
    return false;
    return cap & PCI_DVSEC_CXL_CACHE_CAPABLE;
    }
//
// pci_ats_required - Whether the PCI device requires ATS
// @pdev: the PCI device
//
// Returns true, if the PCI device requires ATS for basic functional operation.
//
#[no_mangle]
pub unsafe extern "C" fn pci_ats_required(pdev: *mut pci_dev) -> bool {
    bool pci_ats_required(struct pci_dev *pdev)
    {
    if (!pci_ats_supported(pdev))
    return false;
// A VF inherits its PF's requirement for ATS function
    if (pdev.is_virtfn)
    pdev = pci_physfn(pdev);
    return pci_cxl_ats_required(pdev) ||
    pci_dev_specific_ats_required(pdev);
    }
    EXPORT_SYMBOL_GPL(pci_ats_required);

#[no_mangle]
pub unsafe extern "C" fn pci_pri_init(pdev: *mut pci_dev) {
    void pci_pri_init(struct pci_dev *pdev)
    {
    u16 status;
    pdev.pri_cap = pci_find_ext_capability(pdev, PCI_EXT_CAP_ID_PRI);
    if (!pdev.pri_cap)
    return;
    pci_read_config_word(pdev, pdev.pri_cap + PCI_PRI_STATUS, &status);
    if (status & PCI_PRI_STATUS_PASID)
    pdev.pasid_required = 1;
    }
//
// pci_enable_pri - Enable PRI capability
// @pdev: PCI device structure
// @reqs: outstanding requests
//
// Returns 0 on success, negative value on error
//
#[no_mangle]
pub unsafe extern "C" fn pci_enable_pri(pdev: *mut pci_dev, reqs: u32) -> c_int {
    int pci_enable_pri(struct pci_dev *pdev, u32 reqs)
    {
    u16 control, status;
    u32 max_requests;
    let mut pri: c_int = pdev.pri_cap;
//
// VFs must not implement the PRI Capability.  If their PF
// implements PRI, it is shared by the VFs, so if the PF PRI is
// enabled, it is also enabled for the VF.
//
    if (pdev.is_virtfn) {
    if (pci_physfn(pdev).pri_enabled)
    return 0;
    return -EINVAL;
    }
    if (WARN_ON(pdev.pri_enabled))
    return -EBUSY;
    if (!pri)
    return -EINVAL;
    pci_read_config_word(pdev, pri + PCI_PRI_STATUS, &status);
    if (!(status & PCI_PRI_STATUS_STOPPED))
    return -EBUSY;
    pci_read_config_dword(pdev, pri + PCI_PRI_MAX_REQ, &max_requests);
    reqs = min(max_requests, reqs);
    pdev.pri_reqs_alloc = reqs;
    pci_write_config_dword(pdev, pri + PCI_PRI_ALLOC_REQ, reqs);
    control = PCI_PRI_CTRL_ENABLE;
    pci_write_config_word(pdev, pri + PCI_PRI_CTRL, control);
    pdev.pri_enabled = 1;
    return 0;
    }
//
// pci_disable_pri - Disable PRI capability
// @pdev: PCI device structure
//
// Only clears the enabled-bit, regardless of its former value
//
#[no_mangle]
pub unsafe extern "C" fn pci_disable_pri(pdev: *mut pci_dev) {
    void pci_disable_pri(struct pci_dev *pdev)
    {
    u16 control;
    let mut pri: c_int = pdev.pri_cap;
// VFs share the PF PRI
    if (pdev.is_virtfn)
    return;
    if (WARN_ON(!pdev.pri_enabled))
    return;
    if (!pri)
    return;
    pci_read_config_word(pdev, pri + PCI_PRI_CTRL, &control);
    control &= ~PCI_PRI_CTRL_ENABLE;
    pci_write_config_word(pdev, pri + PCI_PRI_CTRL, control);
    pdev.pri_enabled = 0;
    }
    EXPORT_SYMBOL_GPL(pci_disable_pri);
//
// pci_restore_pri_state - Restore PRI
// @pdev: PCI device structure
//
#[no_mangle]
pub unsafe extern "C" fn pci_restore_pri_state(pdev: *mut pci_dev) {
    void pci_restore_pri_state(struct pci_dev *pdev)
    {
    let mut control: u16 = PCI_PRI_CTRL_ENABLE;
    let mut reqs: u32 = pdev.pri_reqs_alloc;
    let mut pri: c_int = pdev.pri_cap;
    if (pdev.is_virtfn)
    return;
    if (!pdev.pri_enabled)
    return;
    if (!pri)
    return;
    pci_write_config_dword(pdev, pri + PCI_PRI_ALLOC_REQ, reqs);
    pci_write_config_word(pdev, pri + PCI_PRI_CTRL, control);
    }
//
// pci_reset_pri - Resets device's PRI state
// @pdev: PCI device structure
//
// The PRI capability must be disabled before this function is called.
// Returns 0 on success, negative value on error.
//
#[no_mangle]
pub unsafe extern "C" fn pci_reset_pri(pdev: *mut pci_dev) -> c_int {
    int pci_reset_pri(struct pci_dev *pdev)
    {
    u16 control;
    let mut pri: c_int = pdev.pri_cap;
    if (pdev.is_virtfn)
    return 0;
    if (WARN_ON(pdev.pri_enabled))
    return -EBUSY;
    if (!pri)
    return -EINVAL;
    control = PCI_PRI_CTRL_RESET;
    pci_write_config_word(pdev, pri + PCI_PRI_CTRL, control);
    return 0;
    }
//
// pci_prg_resp_pasid_required - Return PRG Response PASID Required bit
// status.
// @pdev: PCI device structure
//
// Returns 1 if PASID is required in PRG Response Message, 0 otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn pci_prg_resp_pasid_required(pdev: *mut pci_dev) -> c_int {
    int pci_prg_resp_pasid_required(struct pci_dev *pdev)
    {
    if (pdev.is_virtfn)
    pdev = pci_physfn(pdev);
    return pdev.pasid_required;
    }
//
// pci_pri_supported - Check if PRI is supported.
// @pdev: PCI device structure
//
// Returns true if PRI capability is present, false otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn pci_pri_supported(pdev: *mut pci_dev) -> bool {
    bool pci_pri_supported(struct pci_dev *pdev)
    {
// VFs share the PF PRI
    if (pci_physfn(pdev).pri_cap)
    return true;
    return false;
    }
    EXPORT_SYMBOL_GPL(pci_pri_supported);

#[no_mangle]
pub unsafe extern "C" fn pci_pasid_init(pdev: *mut pci_dev) {
    void pci_pasid_init(struct pci_dev *pdev)
    {
    pdev.pasid_cap = pci_find_ext_capability(pdev, PCI_EXT_CAP_ID_PASID);
    }
//
// pci_enable_pasid - Enable the PASID capability
// @pdev: PCI device structure
// @features: Features to enable
//
// Returns 0 on success, negative value on error. This function checks
// whether the features are actually supported by the device and returns
// an error if not.
//
#[no_mangle]
pub unsafe extern "C" fn pci_enable_pasid(pdev: *mut pci_dev, features: c_int) -> c_int {
    int pci_enable_pasid(struct pci_dev *pdev, int features)
    {
    u16 control, supported;
    let mut pasid: c_int = pdev.pasid_cap;
//
// VFs must not implement the PASID Capability, but if a PF
// supports PASID, its VFs share the PF PASID configuration.
//
    if (pdev.is_virtfn) {
    if (pci_physfn(pdev).pasid_enabled)
    return 0;
    return -EINVAL;
    }
    if (WARN_ON(pdev.pasid_enabled))
    return -EBUSY;
    if (!pdev.eetlp_prefix_max && !pdev.pasid_no_tlp)
    return -EINVAL;
    if (!pasid)
    return -EINVAL;
    if (!pci_acs_path_enabled(pdev, core::ptr::null_mut(), PCI_ACS_RR | PCI_ACS_UF))
    return -EINVAL;
    pci_read_config_word(pdev, pasid + PCI_PASID_CAP, &supported);
    supported &= PCI_PASID_CAP_EXEC | PCI_PASID_CAP_PRIV;
// User wants to enable anything unsupported?
    if ((supported & features) != features)
    return -EINVAL;
    control = PCI_PASID_CTRL_ENABLE | features;
    pdev.pasid_features = features;
    pci_write_config_word(pdev, pasid + PCI_PASID_CTRL, control);
    pdev.pasid_enabled = 1;
    return 0;
    }
    EXPORT_SYMBOL_GPL(pci_enable_pasid);
//
// pci_disable_pasid - Disable the PASID capability
// @pdev: PCI device structure
//
#[no_mangle]
pub unsafe extern "C" fn pci_disable_pasid(pdev: *mut pci_dev) {
    void pci_disable_pasid(struct pci_dev *pdev)
    {
    let mut control: u16 = 0;
    let mut pasid: c_int = pdev.pasid_cap;
// VFs share the PF PASID configuration
    if (pdev.is_virtfn)
    return;
    if (WARN_ON(!pdev.pasid_enabled))
    return;
    if (!pasid)
    return;
    pci_write_config_word(pdev, pasid + PCI_PASID_CTRL, control);
    pdev.pasid_enabled = 0;
    }
    EXPORT_SYMBOL_GPL(pci_disable_pasid);
//
// pci_restore_pasid_state - Restore PASID capabilities
// @pdev: PCI device structure
//
#[no_mangle]
pub unsafe extern "C" fn pci_restore_pasid_state(pdev: *mut pci_dev) {
    void pci_restore_pasid_state(struct pci_dev *pdev)
    {
    u16 control;
    let mut pasid: c_int = pdev.pasid_cap;
    if (pdev.is_virtfn)
    return;
    if (!pdev.pasid_enabled)
    return;
    if (!pasid)
    return;
    control = PCI_PASID_CTRL_ENABLE | pdev.pasid_features;
    pci_write_config_word(pdev, pasid + PCI_PASID_CTRL, control);
    }
//
// pci_pasid_features - Check which PASID features are supported
// @pdev: PCI device structure
//
// Return a negative value when no PASID capability is present.
// Otherwise return a bitmask with supported features. Current
// features reported are:
// PCI_PASID_CAP_EXEC - Execute permission supported
// PCI_PASID_CAP_PRIV - Privileged mode supported
//
#[no_mangle]
pub unsafe extern "C" fn pci_pasid_features(pdev: *mut pci_dev) -> c_int {
    int pci_pasid_features(struct pci_dev *pdev)
    {
    u16 supported;
    int pasid;
    if (pdev.is_virtfn)
    pdev = pci_physfn(pdev);
    pasid = pdev.pasid_cap;
    if (!pasid)
    return -EINVAL;
    pci_read_config_word(pdev, pasid + PCI_PASID_CAP, &supported);
    supported &= PCI_PASID_CAP_EXEC | PCI_PASID_CAP_PRIV;
    return supported;
    }
    EXPORT_SYMBOL_GPL(pci_pasid_features);
//
// pci_max_pasids - Get maximum number of PASIDs supported by device
// @pdev: PCI device structure
//
// Returns negative value when PASID capability is not present.
// Otherwise it returns the number of supported PASIDs.
//
#[no_mangle]
pub unsafe extern "C" fn pci_max_pasids(pdev: *mut pci_dev) -> c_int {
    int pci_max_pasids(struct pci_dev *pdev)
    {
    u16 supported;
    int pasid;
    if (pdev.is_virtfn)
    pdev = pci_physfn(pdev);
    pasid = pdev.pasid_cap;
    if (!pasid)
    return -EINVAL;
    pci_read_config_word(pdev, pasid + PCI_PASID_CAP, &supported);
    return (1 << FIELD_GET(PCI_PASID_CAP_WIDTH, supported));
    }
    EXPORT_SYMBOL_GPL(pci_max_pasids);
//
// pci_pasid_status - Check the PASID status
// @pdev: PCI device structure
//
// Returns a negative value when no PASID capability is present.
// Otherwise the value of the control register is returned.
// Status reported are:
//
// PCI_PASID_CTRL_ENABLE - PASID enabled
// PCI_PASID_CTRL_EXEC - Execute permission enabled
// PCI_PASID_CTRL_PRIV - Privileged mode enabled
//
#[no_mangle]
pub unsafe extern "C" fn pci_pasid_status(pdev: *mut pci_dev) -> c_int {
    int pci_pasid_status(struct pci_dev *pdev)
    {
    int pasid;
    u16 ctrl;
    if (pdev.is_virtfn)
    pdev = pci_physfn(pdev);
    pasid = pdev.pasid_cap;
    if (!pasid)
    return -EINVAL;
    pci_read_config_word(pdev, pasid + PCI_PASID_CTRL, &ctrl);
    ctrl &= PCI_PASID_CTRL_ENABLE | PCI_PASID_CTRL_EXEC |
    PCI_PASID_CTRL_PRIV;
    return ctrl;
    }
    EXPORT_SYMBOL_GPL(pci_pasid_status);
