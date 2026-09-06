//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/cavium-rng.c
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
// Hardware Random Number Generator support.
// Cavium Thunder, Marvell OcteonTx/Tx2 processor families.
//
// Copyright (C) 2016 Cavium, Inc.
//

pub const THUNDERX_RNM_ENT_EN: c_uint = 0x1;
pub const THUNDERX_RNM_RNG_EN: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cavium_rng_pf {
    pub control_status: *mut void __iomem,
}

// Enable the RNG hardware and activate the VF
    static int cavium_rng_probe(struct pci_dev *pdev,
    const struct pci_device_id *id)
    {
    struct	cavium_rng_pf *rng;
    int	iov_err;
    rng = devm_kzalloc(&pdev.dev, sizeof(*rng), GFP_KERNEL);
    if (!rng)
    return -ENOMEM;
// Map the RNG control
    rng.control_status = pcim_iomap(pdev, 0, 0);
    if (!rng.control_status) {
    dev_err(&pdev.dev,
    "Error iomap failed retrieving control_status.\n");
    return -ENOMEM;
    }
// Enable the RNG hardware and entropy source
    writeq(THUNDERX_RNM_RNG_EN | THUNDERX_RNM_ENT_EN,
    rng.control_status);
    pci_set_drvdata(pdev, rng);
// Enable the Cavium RNG as a VF
    iov_err = pci_enable_sriov(pdev, 1);
    if (iov_err != 0) {
// Disable the RNG hardware and entropy source
    writeq(0, rng.control_status);
    dev_err(&pdev.dev,
    "Error initializing RNG virtual function,(%i).\n",
    iov_err);
    return iov_err;
    }
    return 0;
    }
// Disable VF and RNG Hardware
#[no_mangle]
unsafe extern "C" fn cavium_rng_remove(pdev: *mut pci_dev) {
    static void cavium_rng_remove(struct pci_dev *pdev)
    {
    struct cavium_rng_pf *rng;
    rng = pci_get_drvdata(pdev);
// Remove the VF
    pci_disable_sriov(pdev);
// Disable the RNG hardware and entropy source
    writeq(0, rng.control_status);
    }
    static const struct pci_device_id cavium_rng_pf_id_table[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_CAVIUM, 0xa018) }, /* Thunder RNM */
    { },
    };
    MODULE_DEVICE_TABLE(pci, cavium_rng_pf_id_table);
    static struct pci_driver cavium_rng_pf_driver = {
    .name		= "cavium_rng_pf",
    .id_table	= cavium_rng_pf_id_table,
    .probe		= cavium_rng_probe,
    .remove		= cavium_rng_remove,
    };
    module_pci_driver(cavium_rng_pf_driver);
    MODULE_AUTHOR("Omer Khaliq <okhaliq@caviumnetworks.com>");
    MODULE_DESCRIPTION("Cavium ThunderX Random Number Generator support");
    MODULE_LICENSE("GPL v2");
