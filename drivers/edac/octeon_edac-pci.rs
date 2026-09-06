//! Automatically rewritten from C to Rust
//! Source: drivers/edac/octeon_edac-pci.c
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


//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file "COPYING" in the main directory of this archive
// for more details.
//
// Copyright (C) 2012 Cavium, Inc.
// Copyright (C) 2009 Wind River Systems,
// written by Ralf Baechle <ralf@linux-mips.org>
//

#[no_mangle]
unsafe extern "C" fn octeon_pci_poll(pci: *mut edac_pci_ctl_info) {
    static void octeon_pci_poll(struct edac_pci_ctl_info *pci)
    {
    union cvmx_pci_cfg01 cfg01;
    cfg01.u32 = octeon_npi_read32(CVMX_NPI_PCI_CFG01);
    if (cfg01.s.dpe) {		/* Detected parity error */
    edac_pci_handle_pe(pci, pci.ctl_name);
    cfg01.s.dpe = 1;		/* Reset  */
    octeon_npi_write32(CVMX_NPI_PCI_CFG01, cfg01.u32);
    }
    if (cfg01.s.sse) {
    edac_pci_handle_npe(pci, "Signaled System Error");
    cfg01.s.sse = 1;		/* Reset */
    octeon_npi_write32(CVMX_NPI_PCI_CFG01, cfg01.u32);
    }
    if (cfg01.s.rma) {
    edac_pci_handle_npe(pci, "Received Master Abort");
    cfg01.s.rma = 1;		/* Reset */
    octeon_npi_write32(CVMX_NPI_PCI_CFG01, cfg01.u32);
    }
    if (cfg01.s.rta) {
    edac_pci_handle_npe(pci, "Received Target Abort");
    cfg01.s.rta = 1;		/* Reset */
    octeon_npi_write32(CVMX_NPI_PCI_CFG01, cfg01.u32);
    }
    if (cfg01.s.sta) {
    edac_pci_handle_npe(pci, "Signaled Target Abort");
    cfg01.s.sta = 1;		/* Reset */
    octeon_npi_write32(CVMX_NPI_PCI_CFG01, cfg01.u32);
    }
    if (cfg01.s.mdpe) {
    edac_pci_handle_npe(pci, "Master Data Parity Error");
    cfg01.s.mdpe = 1;		/* Reset */
    octeon_npi_write32(CVMX_NPI_PCI_CFG01, cfg01.u32);
    }
    }
#[no_mangle]
unsafe extern "C" fn octeon_pci_probe(pdev: *mut platform_device) -> c_int {
    static int octeon_pci_probe(struct platform_device *pdev)
    {
    struct edac_pci_ctl_info *pci;
    let mut res: c_int = 0;
    pci = edac_pci_alloc_ctl_info(0, "octeon_pci_err");
    if (!pci)
    return -ENOMEM;
    pci.dev = &pdev.dev;
    platform_set_drvdata(pdev, pci);
    pci.dev_name = dev_name(&pdev.dev);
    pci.mod_name = "octeon-pci";
    pci.ctl_name = "octeon_pci_err";
    pci.edac_check = octeon_pci_poll;
    if (edac_pci_add_device(pci, 0) > 0) {
    pr_err("%s: edac_pci_add_device() failed\n", __func__);
    goto err;
    }
    return 0;
    err:
    edac_pci_free_ctl_info(pci);
    return res;
    }
#[no_mangle]
unsafe extern "C" fn octeon_pci_remove(pdev: *mut platform_device) {
    static void octeon_pci_remove(struct platform_device *pdev)
    {
    struct edac_pci_ctl_info *pci = platform_get_drvdata(pdev);
    edac_pci_del_device(&pdev.dev);
    edac_pci_free_ctl_info(pci);
    }
    static struct platform_driver octeon_pci_driver = {
    .probe = octeon_pci_probe,
    .remove = octeon_pci_remove,
    .driver = {
    .name = "octeon_pci_edac",
    }
    };
    module_platform_driver(octeon_pci_driver);
    MODULE_DESCRIPTION("Cavium Octeon PCI Controller EDAC driver");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Ralf Baechle <ralf@linux-mips.org>");
