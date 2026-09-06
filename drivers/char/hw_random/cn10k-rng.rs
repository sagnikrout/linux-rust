//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/cn10k-rng.c
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
// Marvell CN10K RVU Hardware Random Number Generator.
//
// Copyright (C) 2021 Marvell.
//

// CSRs
pub const RNM_CTL_STATUS: c_uint = 0x000;
pub const RNM_ENTROPY_STATUS: c_uint = 0x008;
pub const RNM_CONST: c_uint = 0x030;
pub const RNM_EBG_ENT: c_uint = 0x048;
pub const RNM_PF_EBG_HEALTH: c_uint = 0x050;
pub const RNM_PF_RANDOM: c_uint = 0x400;
pub const RNM_TRNG_RESULT: c_uint = 0x408;
// Extended TRNG Read and Status Registers
pub const RNM_PF_TRNG_DAT: c_uint = 0x1000;
pub const RNM_PF_TRNG_RES: c_uint = 0x1008;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cn10k_rng {
    pub reg_base: *mut void __iomem,
    pub ops: hwrng,
    pub pdev: *mut pci_dev,
// Octeon CN10K-A A0/A1, CNF10K-A A0/A1 and CNF10K-B A0/B0
// does not support extended TRNG registers
//
    pub extended_trng_regs: bool,
}

pub const PLAT_OCTEONTX_RESET_RNG_EBG_HEALTH_STATE: c_uint = 0xc2000b0f;
pub const PCI_SUBSYS_DEVID_CN10K_A_RNG: c_uint = 0xB900;
pub const PCI_SUBSYS_DEVID_CNF10K_A_RNG: c_uint = 0xBA00;
pub const PCI_SUBSYS_DEVID_CNF10K_B_RNG: c_uint = 0xBC00;
#[no_mangle]
unsafe extern "C" fn cn10k_is_extended_trng_regs_supported(pdev: *mut pci_dev) -> bool {
    static bool cn10k_is_extended_trng_regs_supported(struct pci_dev *pdev)
    {
// CN10K-A A0/A1
    if ((pdev.subsystem_device == PCI_SUBSYS_DEVID_CN10K_A_RNG) &&
    (!pdev.revision || (pdev.revision & 0xff) == 0x50 ||
    (pdev.revision & 0xff) == 0x51))
    return false;
// CNF10K-A A0
    if ((pdev.subsystem_device == PCI_SUBSYS_DEVID_CNF10K_A_RNG) &&
    (!pdev.revision || (pdev.revision & 0xff) == 0x60 ||
    (pdev.revision & 0xff) == 0x61))
    return false;
// CNF10K-B A0/B0
    if ((pdev.subsystem_device == PCI_SUBSYS_DEVID_CNF10K_B_RNG) &&
    (!pdev.revision || (pdev.revision & 0xff) == 0x70 ||
    (pdev.revision & 0xff) == 0x74))
    return false;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn reset_rng_health_state(rng: *mut cn10k_rng) -> c_ulong {
    static unsigned long reset_rng_health_state(struct cn10k_rng *rng)
    {
    struct arm_smccc_res res;
// Send SMC service call to reset EBG health state
    arm_smccc_smc(PLAT_OCTEONTX_RESET_RNG_EBG_HEALTH_STATE, 0, 0, 0, 0, 0, 0, 0, &res);
    return res.a0;
    }
#[no_mangle]
unsafe extern "C" fn check_rng_health(rng: *mut cn10k_rng) -> c_int {
    static int check_rng_health(struct cn10k_rng *rng)
    {
    u64 status;
    unsigned long err;
// Skip checking health
    if (!rng.reg_base)
    return -ENODEV;
    status = readq(rng.reg_base + RNM_PF_EBG_HEALTH);
    if (status & BIT_ULL(20)) {
    err = reset_rng_health_state(rng);
    if (err) {
    dev_err(&rng.pdev.dev, "HWRNG: Health test failed (status=%llx)\n",
    status);
    dev_err(&rng.pdev.dev, "HWRNG: error during reset (error=%lx)\n",
    err);
    return -EIO;
    }
    }
    return 0;
    }
// Returns true when valid data available otherwise return false
#[no_mangle]
unsafe extern "C" fn cn10k_read_trng(rng: *mut cn10k_rng, value: *mut u64) -> bool {
    static bool cn10k_read_trng(struct cn10k_rng *rng, u64 *value)
    {
    let mut retry_count: u16 = 0;
    u64 upper, lower;
    u64 status;
    if (rng.extended_trng_regs) {
    do {
// value = readq(rng->reg_base + RNM_PF_TRNG_DAT);
    if (*value)
    return true;
    status = readq(rng.reg_base + RNM_PF_TRNG_RES);
    if (!status && (retry_count++ > 0x1000))
    return false;
    } while (!status);
    }
// value = readq(rng->reg_base + RNM_PF_RANDOM);
// HW can run out of entropy if large amount random data is read in
// quick succession. Zeros may not be real random data from HW.
//
    if (!*value) {
    upper = readq(rng.reg_base + RNM_PF_RANDOM);
    lower = readq(rng.reg_base + RNM_PF_RANDOM);
    while (!(upper & 0x00000000FFFFFFFFULL))
    upper = readq(rng.reg_base + RNM_PF_RANDOM);
    while (!(lower & 0xFFFFFFFF00000000ULL))
    lower = readq(rng.reg_base + RNM_PF_RANDOM);
// value = (upper & 0xFFFFFFFF00000000) | (lower & 0xFFFFFFFF);
    }
    return true;
    }
    static int cn10k_rng_read(struct hwrng *hwrng, void *data,
    size_t max, bool wait)
    {
    struct cn10k_rng *rng = (struct cn10k_rng *)hwrng.priv;
    unsigned int size;
    u8 *pos = data;
    let mut err: c_int = 0;
    u64 value;
    err = check_rng_health(rng);
    if (err)
    return err;
    size = max;
    while (size >= 8) {
    if (!cn10k_read_trng(rng, &value))
    goto out;
// ((u64 *)pos) = value;
    size -= 8;
    pos += 8;
    }
    if (size > 0) {
    if (!cn10k_read_trng(rng, &value))
    goto out;
    while (size > 0) {
// pos = (u8)value;
    value >>= 8;
    size--;
    pos++;
    }
    }
    out:
    return max - size;
    }
#[no_mangle]
unsafe extern "C" fn cn10k_rng_probe(pdev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int cn10k_rng_probe(struct pci_dev *pdev, const struct pci_device_id *id)
    {
    struct	cn10k_rng *rng;
    int	err;
    rng = devm_kzalloc(&pdev.dev, sizeof(*rng), GFP_KERNEL);
    if (!rng)
    return -ENOMEM;
    rng.pdev = pdev;
    pci_set_drvdata(pdev, rng);
    rng.reg_base = pcim_iomap(pdev, 0, 0);
    if (!rng.reg_base)
    return -ENOMEM;
    rng.ops.name = devm_kasprintf(&pdev.dev, GFP_KERNEL,
    "cn10k-rng-%s", dev_name(&pdev.dev));
    if (!rng.ops.name)
    return -ENOMEM;
    rng.ops.read = cn10k_rng_read;
    rng.ops.priv = (unsigned long)rng;
    rng.extended_trng_regs = cn10k_is_extended_trng_regs_supported(pdev);
    reset_rng_health_state(rng);
    err = devm_hwrng_register(&pdev.dev, &rng.ops);
    if (err)
    return dev_err_probe(&pdev.dev, err, "Could not register hwrng device.\n");
    return 0;
    }
    static const struct pci_device_id cn10k_rng_id_table[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_CAVIUM, 0xA098) }, /* RNG PF */
    {0,},
    };
    MODULE_DEVICE_TABLE(pci, cn10k_rng_id_table);
    static struct pci_driver cn10k_rng_driver = {
    .name		= "cn10k_rng",
    .id_table	= cn10k_rng_id_table,
    .probe		= cn10k_rng_probe,
    };
    module_pci_driver(cn10k_rng_driver);
    MODULE_AUTHOR("Sunil Goutham <sgoutham@marvell.com>");
    MODULE_DESCRIPTION("Marvell CN10K HW RNG Driver");
    MODULE_LICENSE("GPL v2");
