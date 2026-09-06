//! Automatically rewritten from C to Rust
//! Source: drivers/pci/controller/pci-hyperv-intf.c
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
// Copyright (c) Microsoft Corporation.
//
// Author:
// Haiyang Zhang <haiyangz@microsoft.com>
//
// This small module is a helper driver allows other drivers to
// have a common interface with the Hyper-V PCI frontend driver.
//

    struct hyperv_pci_block_ops hvpci_block_ops;
    EXPORT_SYMBOL_GPL(hvpci_block_ops);
    int hyperv_read_cfg_blk(struct pci_dev *dev, void *buf, unsigned int buf_len,
    unsigned int block_id, unsigned int *bytes_returned)
    {
    if (!hvpci_block_ops.read_block)
    return -EOPNOTSUPP;
    return hvpci_block_ops.read_block(dev, buf, buf_len, block_id,
    bytes_returned);
    }
    EXPORT_SYMBOL_GPL(hyperv_read_cfg_blk);
    int hyperv_write_cfg_blk(struct pci_dev *dev, void *buf, unsigned int len,
    unsigned int block_id)
    {
    if (!hvpci_block_ops.write_block)
    return -EOPNOTSUPP;
    return hvpci_block_ops.write_block(dev, buf, len, block_id);
    }
    EXPORT_SYMBOL_GPL(hyperv_write_cfg_blk);
    int hyperv_reg_block_invalidate(struct pci_dev *dev, void *context,
    void (*block_invalidate)(void *context,
    u64 block_mask))
    {
    if (!hvpci_block_ops.reg_blk_invalidate)
    return -EOPNOTSUPP;
    return hvpci_block_ops.reg_blk_invalidate(dev, context,
    block_invalidate);
    }
    EXPORT_SYMBOL_GPL(hyperv_reg_block_invalidate);
    MODULE_DESCRIPTION("Hyper-V PCI Interface");
    MODULE_LICENSE("GPL v2");
