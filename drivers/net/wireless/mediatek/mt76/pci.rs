//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/mediatek/mt76/pci.c
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
//
// Copyright (C) 2019 Lorenzo Bianconi <lorenzo@kernel.org>
//

#[no_mangle]
pub unsafe extern "C" fn mt76_pci_disable_aspm(pdev: *mut pci_dev) {
    void mt76_pci_disable_aspm(struct pci_dev *pdev)
    {
    struct pci_dev *parent = pdev.bus.self;
    u16 aspm_conf, parent_aspm_conf = 0;
    pcie_capability_read_word(pdev, PCI_EXP_LNKCTL, &aspm_conf);
    aspm_conf &= PCI_EXP_LNKCTL_ASPMC;
    if (parent) {
    pcie_capability_read_word(parent, PCI_EXP_LNKCTL,
    &parent_aspm_conf);
    parent_aspm_conf &= PCI_EXP_LNKCTL_ASPMC;
    }
    if (!aspm_conf && (!parent || !parent_aspm_conf)) {
// aspm already disabled
    return;
    }
    dev_info(&pdev.dev, "disabling ASPM %s %s\n",
    (aspm_conf & PCI_EXP_LNKCTL_ASPM_L0S) ? "L0s" : "",
    (aspm_conf & PCI_EXP_LNKCTL_ASPM_L1) ? "L1" : "");
    if (IS_ENABLED(CONFIG_PCIEASPM)) {
    int err;
    let mut state: c_int = 0;
    if (aspm_conf & PCI_EXP_LNKCTL_ASPM_L0S)
    state |= PCIE_LINK_STATE_L0S;
    if (aspm_conf & PCI_EXP_LNKCTL_ASPM_L1)
    state |= PCIE_LINK_STATE_L1;
    err = pci_disable_link_state(pdev, state);
    if (!err)
    return;
    }
// both device and parent should have the same ASPM setting.
// disable ASPM in downstream component first and then upstream.
//
    pcie_capability_clear_word(pdev, PCI_EXP_LNKCTL, aspm_conf);
    if (parent)
    pcie_capability_clear_word(parent, PCI_EXP_LNKCTL,
    aspm_conf);
    }
    EXPORT_SYMBOL_GPL(mt76_pci_disable_aspm);
#[no_mangle]
pub unsafe extern "C" fn mt76_pci_aspm_supported(pdev: *mut pci_dev) -> bool {
    bool mt76_pci_aspm_supported(struct pci_dev *pdev)
    {
    struct pci_dev *parent = pdev.bus.self;
    u16 aspm_conf, parent_aspm_conf = 0;
    let mut result: bool = true;
    pcie_capability_read_word(pdev, PCI_EXP_LNKCTL, &aspm_conf);
    aspm_conf &= PCI_EXP_LNKCTL_ASPMC;
    if (parent) {
    pcie_capability_read_word(parent, PCI_EXP_LNKCTL,
    &parent_aspm_conf);
    parent_aspm_conf &= PCI_EXP_LNKCTL_ASPMC;
    }
    if (!aspm_conf && (!parent || !parent_aspm_conf)) {
// aspm already disabled
    result = false;
    }
    return result;
    }
    EXPORT_SYMBOL_GPL(mt76_pci_aspm_supported);
