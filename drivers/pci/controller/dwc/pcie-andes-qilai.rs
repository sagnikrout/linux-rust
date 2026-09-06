//! Automatically rewritten from C to Rust
//! Source: drivers/pci/controller/dwc/pcie-andes-qilai.c
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
// Driver for the PCIe Controller in QiLai from Andes
//
// Copyright (C) 2026 Andes Technology Corporation
//

pub const PCIE_INTR_CONTROL1: c_uint = 0x15c;

pub const PCIE_LOGIC_COHERENCY_CONTROL3: c_uint = 0x8e8;
//
// Refer to Table A4-5 (Memory type encoding) in the
// AMBA AXI and ACE Protocol Specification.
//
// The selected value corresponds to the Memory type field:
// "Write-back, Read and Write-allocate".
//
// The last three rows in the table A4-5 in
// AMBA AXI and ACE Protocol Specification:
// ARCACHE        AWCACHE        Memory type
// ------------------------------------------------------------------
// 1111 (0111)    0111           Write-back Read-allocate
// 1011           1111 (1011)    Write-back Write-allocate
// 1111           1111           Write-back Read and Write-allocate (selected)
//

pub const PCIE_GEN_CONTROL2: c_uint = 0x54;

pub const PCIE_REGS_PCIE_SII_PM_STATE: c_uint = 0xc0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qilai_pcie {
    pub pci: dw_pcie,
    pub apb_base: *mut void __iomem,
}

#[no_mangle]
unsafe extern "C" fn qilai_pcie_link_up(pci: *mut dw_pcie) -> bool {
    static bool qilai_pcie_link_up(struct dw_pcie *pci)
    {
    struct qilai_pcie *pcie = to_qilai_pcie(pci);
    u32 val;
    val = readl(pcie.apb_base + PCIE_REGS_PCIE_SII_PM_STATE);
    return FIELD_GET(SMLH_LINK_UP, val) && FIELD_GET(RDLH_LINK_UP, val);
    }
#[no_mangle]
unsafe extern "C" fn qilai_pcie_start_link(pci: *mut dw_pcie) -> c_int {
    static int qilai_pcie_start_link(struct dw_pcie *pci)
    {
    struct qilai_pcie *pcie = to_qilai_pcie(pci);
    u32 val;
    val = readl(pcie.apb_base + PCIE_GEN_CONTROL2);
    val |= PCIE_CFG_LTSSM_EN;
    writel(val, pcie.apb_base + PCIE_GEN_CONTROL2);
    return 0;
    }
    static const struct dw_pcie_ops qilai_pcie_ops = {
    .link_up = qilai_pcie_link_up,
    .start_link = qilai_pcie_start_link,
    };
//
// Set up the QiLai PCIe IOCP (IO Coherence Port) Read/Write Behaviors to the
// Write-Back, Read and Write Allocate mode.
//
// The IOCP HW target is SoC last-level cache (L2 Cache), which serves as the
// system cache. The IOCP HW helps maintain cache monitoring, ensuring that
// the device can snoop data from/to the cache.
//
#[no_mangle]
unsafe extern "C" fn qilai_pcie_iocp_cache_setup(pp: *mut dw_pcie_rp) {
    static void qilai_pcie_iocp_cache_setup(struct dw_pcie_rp *pp)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    u32 val;
    dw_pcie_dbi_ro_wr_en(pci);
    val = dw_pcie_readl_dbi(pci, PCIE_LOGIC_COHERENCY_CONTROL3);
    FIELD_MODIFY(PCIE_CFG_MSTR_ARCACHE_MODE, &val, IOCP_ARCACHE);
    FIELD_MODIFY(PCIE_CFG_MSTR_AWCACHE_MODE, &val, IOCP_AWCACHE);
    FIELD_MODIFY(PCIE_CFG_MSTR_ARCACHE_VALUE, &val, IOCP_ARCACHE);
    FIELD_MODIFY(PCIE_CFG_MSTR_AWCACHE_VALUE, &val, IOCP_AWCACHE);
    dw_pcie_writel_dbi(pci, PCIE_LOGIC_COHERENCY_CONTROL3, val);
    dw_pcie_dbi_ro_wr_dis(pci);
    }
#[no_mangle]
unsafe extern "C" fn qilai_pcie_enable_msi(pcie: *mut qilai_pcie) {
    static void qilai_pcie_enable_msi(struct qilai_pcie *pcie)
    {
    u32 val;
    val = readl(pcie.apb_base + PCIE_INTR_CONTROL1);
    val |= PCIE_MSI_CTRL_INT_EN;
    writel(val, pcie.apb_base + PCIE_INTR_CONTROL1);
    }
#[no_mangle]
unsafe extern "C" fn qilai_pcie_host_init(pp: *mut dw_pcie_rp) -> c_int {
    static int qilai_pcie_host_init(struct dw_pcie_rp *pp)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    struct qilai_pcie *pcie = to_qilai_pcie(pci);
    qilai_pcie_enable_msi(pcie);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qilai_pcie_host_post_init(pp: *mut dw_pcie_rp) {
    static void qilai_pcie_host_post_init(struct dw_pcie_rp *pp)
    {
    qilai_pcie_iocp_cache_setup(pp);
    }
    static const struct dw_pcie_host_ops qilai_pcie_host_ops = {
    .init = qilai_pcie_host_init,
    .post_init = qilai_pcie_host_post_init,
    };
#[no_mangle]
unsafe extern "C" fn qilai_pcie_probe(pdev: *mut platform_device) -> c_int {
    static int qilai_pcie_probe(struct platform_device *pdev)
    {
    struct qilai_pcie *pcie;
    struct dw_pcie *pci;
    struct device *dev = &pdev.dev;
    int ret;
    pcie = devm_kzalloc(&pdev.dev, sizeof(*pcie), GFP_KERNEL);
    if (!pcie)
    return -ENOMEM;
    platform_set_drvdata(pdev, pcie);
    pci = &pcie.pci;
    pcie.pci.dev = dev;
    pcie.pci.ops = &qilai_pcie_ops;
    pcie.pci.pp.ops = &qilai_pcie_host_ops;
    pci.use_parent_dt_ranges = true;
    dw_pcie_cap_set(&pcie.pci, REQ_RES);
    pcie.apb_base = devm_platform_ioremap_resource_byname(pdev, "apb");
    if (IS_ERR(pcie.apb_base))
    return PTR_ERR(pcie.apb_base);
    pm_runtime_set_active(dev);
    pm_runtime_no_callbacks(dev);
    devm_pm_runtime_enable(dev);
    ret = dw_pcie_host_init(&pcie.pci.pp);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to initialize PCIe host\n");
    return 0;
    }
    static const struct of_device_id qilai_pcie_of_match[] = {
    { .compatible = "andestech,qilai-pcie" },
    {},
    };
    MODULE_DEVICE_TABLE(of, qilai_pcie_of_match);
    static struct platform_driver qilai_pcie_driver = {
    .probe = qilai_pcie_probe,
    .driver = {
    .name	= "qilai-pcie",
    .of_match_table = qilai_pcie_of_match,
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    };
    builtin_platform_driver(qilai_pcie_driver);
    MODULE_AUTHOR("Randolph Lin <randolph@andestech.com>");
    MODULE_DESCRIPTION("Andes QiLai PCIe driver");
    MODULE_LICENSE("GPL");
