//! Automatically rewritten from C to Rust
//! Source: drivers/pci/pcie/tlp.c
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
// PCIe TLP Log handling
//
// Copyright (C) 2024 Intel Corporation
//

//
// aer_tlp_log_len - Calculate AER Capability TLP Header/Prefix Log length
// @dev: PCIe device
// @aercc: AER Capabilities and Control register value
//
// Return: TLP Header/Prefix Log length
//
#[no_mangle]
pub unsafe extern "C" fn aer_tlp_log_len(dev: *mut pci_dev, aercc: u32) -> c_uint {
    unsigned int aer_tlp_log_len(struct pci_dev *dev, u32 aercc)
    {
    if (aercc & PCI_ERR_CAP_TLP_LOG_FLIT)
    return FIELD_GET(PCI_ERR_CAP_TLP_LOG_SIZE, aercc);
    return PCIE_STD_NUM_TLP_HEADERLOG +
    ((aercc & PCI_ERR_CAP_PREFIX_LOG_PRESENT) ?
    dev.eetlp_prefix_max : 0);
    }

//
// dpc_tlp_log_len - Calculate DPC RP PIO TLP Header/Prefix Log length
// @dev: PCIe device
//
// Return: TLP Header/Prefix Log length
//
#[no_mangle]
pub unsafe extern "C" fn dpc_tlp_log_len(dev: *mut pci_dev) -> c_uint {
    unsigned int dpc_tlp_log_len(struct pci_dev *dev)
    {
// Remove ImpSpec Log register from the count
    if (dev.dpc_rp_log_size >= PCIE_STD_NUM_TLP_HEADERLOG + 1)
    return dev.dpc_rp_log_size - 1;
    return dev.dpc_rp_log_size;
    }

//
// pcie_read_tlp_log - read TLP Header Log
// @dev: PCIe device
// @where: PCI Config offset of TLP Header Log
// @where2: PCI Config offset of TLP Prefix Log
// @tlp_len: TLP Log length (Header Log + TLP Prefix Log in DWORDs)
// @flit: TLP Logged in Flit mode
// @log: TLP Log structure to fill
//
// Fill @log from TLP Header Log registers, e.g., AER or DPC.
//
// Return: 0 on success and filled TLP Log structure, <0 on error.
//
    int pcie_read_tlp_log(struct pci_dev *dev, int where, int where2,
    unsigned int tlp_len, bool flit, struct pcie_tlp_log *log)
    {
    unsigned int i;
    int off, ret;
    if (tlp_len > ARRAY_SIZE(log.dw))
    tlp_len = ARRAY_SIZE(log.dw);
    memset(log, 0, sizeof(*log));
    for (i = 0; i < tlp_len; i++) {
    if (i < PCIE_STD_NUM_TLP_HEADERLOG)
    off = where + i * 4;
    else
    off = where2 + (i - PCIE_STD_NUM_TLP_HEADERLOG) * 4;
    ret = pci_read_config_dword(dev, off, &log.dw[i]);
    if (ret)
    return pcibios_err_to_errno(ret);
    }
//
// Hard-code non-Flit mode to 4 DWORDs, for now. The exact length
// can only be known if the TLP is parsed.
//
    log.header_len = flit ? tlp_len : 4;
    log.flit = flit;
    return 0;
    }

//
// pcie_print_tlp_log - Print TLP Header / Prefix Log contents
// @dev: PCIe device
// @log: TLP Log structure
// @level: Printk log level
// @pfx: String prefix
//
// Prints TLP Header and Prefix Log information held by @log.
//
    void pcie_print_tlp_log(const struct pci_dev *dev,
    const struct pcie_tlp_log *log, const char *level,
    const char *pfx)
    {
// EE_PREFIX_STR fits the extended DW space needed for the Flit mode
    char buf[11 * PCIE_STD_MAX_TLP_HEADERLOG + 1];
    unsigned int i;
    int len;
    len = scnprintf(buf, sizeof(buf), "%#010x %#010x %#010x %#010x",
    log.dw[0], log.dw[1], log.dw[2], log.dw[3]);
    if (log.flit) {
    for (i = PCIE_STD_NUM_TLP_HEADERLOG; i < log.header_len; i++) {
    len += scnprintf(buf + len, sizeof(buf) - len,
    " %#010x", log.dw[i]);
    }
    } else {
    if (log.prefix[0])
    len += scnprintf(buf + len, sizeof(buf) - len,
    EE_PREFIX_STR);
    for (i = 0; i < ARRAY_SIZE(log.prefix); i++) {
    if (!log.prefix[i])
    break;
    len += scnprintf(buf + len, sizeof(buf) - len,
    " %#010x", log.prefix[i]);
    }
    }
    dev_printk(level, &dev.dev, "%sTLP Header%s: %s\n", pfx,
    log.flit ? " (Flit)" : "", buf);
    }
