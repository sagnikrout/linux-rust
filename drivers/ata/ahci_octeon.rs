//! Automatically rewritten from C to Rust
//! Source: drivers/ata/ahci_octeon.c
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
// SATA glue for Cavium Octeon III SOCs.
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file "COPYING" in the main directory of this archive
// for more details.
//
// Copyright (C) 2010-2015 Cavium Networks
//

pub const CVMX_SATA_UCTL_SHIM_CFG: c_uint = 0xE8;
pub const SATA_UCTL_ENDIAN_MODE_BIG: c_int = 1;
pub const SATA_UCTL_ENDIAN_MODE_LITTLE: c_int = 0;
pub const SATA_UCTL_ENDIAN_MODE_MASK: c_int = 3;
pub const SATA_UCTL_DMA_ENDIAN_MODE_SHIFT: c_int = 8;
pub const SATA_UCTL_CSR_ENDIAN_MODE_SHIFT: c_int = 0;
pub const SATA_UCTL_DMA_READ_CMD_SHIFT: c_int = 12;
#[no_mangle]
unsafe extern "C" fn ahci_octeon_probe(pdev: *mut platform_device) -> c_int {
    static int ahci_octeon_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *node = dev.of_node;
    void __iomem *base;
    u64 cfg;
    int ret;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    cfg = cvmx_readq_csr(base + CVMX_SATA_UCTL_SHIM_CFG);
    cfg &= ~(SATA_UCTL_ENDIAN_MODE_MASK << SATA_UCTL_DMA_ENDIAN_MODE_SHIFT);
    cfg &= ~(SATA_UCTL_ENDIAN_MODE_MASK << SATA_UCTL_CSR_ENDIAN_MODE_SHIFT);

    cfg |= SATA_UCTL_ENDIAN_MODE_BIG << SATA_UCTL_DMA_ENDIAN_MODE_SHIFT;
    cfg |= SATA_UCTL_ENDIAN_MODE_BIG << SATA_UCTL_CSR_ENDIAN_MODE_SHIFT;

    cfg |= SATA_UCTL_ENDIAN_MODE_LITTLE << SATA_UCTL_DMA_ENDIAN_MODE_SHIFT;
    cfg |= SATA_UCTL_ENDIAN_MODE_LITTLE << SATA_UCTL_CSR_ENDIAN_MODE_SHIFT;

    cfg |= 1 << SATA_UCTL_DMA_READ_CMD_SHIFT;
    cvmx_writeq_csr(base + CVMX_SATA_UCTL_SHIM_CFG, cfg);
    if (!node) {
    dev_err(dev, "no device node, failed to add octeon sata\n");
    return -ENODEV;
    }
    ret = of_platform_populate(node, core::ptr::null_mut(), core::ptr::null_mut(), dev);
    if (ret) {
    dev_err(dev, "failed to add ahci-platform core\n");
    return ret;
    }
    return 0;
    }
    static const struct of_device_id octeon_ahci_match[] = {
    { .compatible = "cavium,octeon-7130-sata-uctl", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, octeon_ahci_match);
    static struct platform_driver ahci_octeon_driver = {
    .probe          = ahci_octeon_probe,
    .driver         = {
    .name   = "octeon-ahci",
    .of_match_table = octeon_ahci_match,
    },
    };
    module_platform_driver(ahci_octeon_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Cavium, Inc. <support@cavium.com>");
    MODULE_DESCRIPTION("Cavium Inc. sata config.");
