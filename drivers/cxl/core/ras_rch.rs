//! Automatically rewritten from C to Rust
//! Source: drivers/cxl/core/ras_rch.c
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright(c) 2025 AMD Corporation. All rights reserved.

#[no_mangle]
pub unsafe extern "C" fn cxl_dport_map_rch_aer(dport: *mut cxl_dport) {
    void cxl_dport_map_rch_aer(struct cxl_dport *dport)
    {
    resource_size_t aer_phys;
    struct device *host;
    u16 aer_cap;
    aer_cap = cxl_rcrb_to_aer(dport.dport_dev, dport.rcrb.base);
    if (aer_cap) {
    host = dport.reg_map.host;
    aer_phys = aer_cap + dport.rcrb.base;
    dport.regs.dport_aer =
    devm_cxl_iomap_block(host, aer_phys,
    sizeof(struct aer_capability_regs));
    }
    }
#[no_mangle]
pub unsafe extern "C" fn cxl_disable_rch_root_ints(dport: *mut cxl_dport) {
    void cxl_disable_rch_root_ints(struct cxl_dport *dport)
    {
    void __iomem *aer_base = dport.regs.dport_aer;
    u32 aer_cmd_mask, aer_cmd;
    if (!aer_base)
    return;
//
// Disable RCH root port command interrupts.
// CXL 3.0 12.2.1.1 - RCH Downstream Port-detected Errors
//
// This sequence may not be necessary. CXL spec states disabling
// the root cmd register's interrupts is required. But, PCI spec
// shows these are disabled by default on reset.
//
    aer_cmd_mask = (PCI_ERR_ROOT_CMD_COR_EN |
    PCI_ERR_ROOT_CMD_NONFATAL_EN |
    PCI_ERR_ROOT_CMD_FATAL_EN);
    aer_cmd = readl(aer_base + PCI_ERR_ROOT_COMMAND);
    aer_cmd &= ~aer_cmd_mask;
    writel(aer_cmd, aer_base + PCI_ERR_ROOT_COMMAND);
    }
//
// Copy the AER capability registers using 32 bit read accesses.
// This is necessary because RCRB AER capability is MMIO mapped. Clear the
// status after copying.
//
// @aer_base: base address of AER capability block in RCRB
// @aer_regs: destination for copying AER capability
//
    static bool cxl_rch_get_aer_info(void __iomem *aer_base,
    struct aer_capability_regs *aer_regs)
    {
//
// Bound the copy to the physically-defined AER registers (header
// through the 16-byte Header Log). struct aer_capability_regs is a
// software layout whose embedded struct pcie_tlp_log is larger than
// the on-wire AER capability; copying sizeof(*aer_regs) would
// over-read the RCRB-mapped MMIO block.
//
    let mut read_cnt: c_int = (PCI_ERR_HEADER_LOG + 16) / sizeof(u32);
    u32 *aer_regs_buf = (u32 *)aer_regs;
    int n;
    if (!aer_base)
    return false;
//
// Zero the destination so the software-only tail fields
// (e.g. header_log.header_len) are deterministic rather than
// left as uninitialized stack, which could drive a bogus loop
// length in pcie_print_tlp_log().
//
    memset(aer_regs, 0, sizeof(*aer_regs));
// Use readl() to guarantee 32-bit accesses
    for (n = 0; n < read_cnt; n++)
    aer_regs_buf[n] = readl(aer_base + n * sizeof(u32));
    writel(aer_regs.uncor_status, aer_base + PCI_ERR_UNCOR_STATUS);
    writel(aer_regs.cor_status, aer_base + PCI_ERR_COR_STATUS);
    return true;
    }
// Get AER severity. Return false if there is no error.
    static bool cxl_rch_get_aer_severity(struct aer_capability_regs *aer_regs,
    int *severity)
    {
    let mut uncor_status: u32 = aer_regs.uncor_status & ~aer_regs.uncor_mask;
    if (uncor_status) {
// severity = (uncor_status & aer_regs->uncor_severity) ?
    AER_FATAL : AER_NONFATAL;
    return true;
    }
    if (aer_regs.cor_status & ~aer_regs.cor_mask) {
// severity = AER_CORRECTABLE;
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn cxl_handle_rdport_errors(cxlds: *mut cxl_dev_state) {
    void cxl_handle_rdport_errors(struct cxl_dev_state *cxlds)
    {
    struct pci_dev *pdev = to_pci_dev(cxlds.dev);
    struct aer_capability_regs aer_regs;
    struct cxl_dport *dport;
    int severity;
    struct cxl_port *port __free(put_cxl_port) =
    cxl_pci_find_port(pdev, &dport);
    if (!port)
    return;
    if (!cxl_rch_get_aer_info(dport.regs.dport_aer, &aer_regs))
    return;
    if (!cxl_rch_get_aer_severity(&aer_regs, &severity))
    return;
    pci_print_aer(pdev, severity, &aer_regs);
    if (severity == AER_CORRECTABLE)
    cxl_handle_cor_ras(&cxlds.cxlmd.dev, dport.regs.ras);
    else
    cxl_handle_ras(&cxlds.cxlmd.dev, dport.regs.ras);
    }
