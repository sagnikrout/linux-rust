//! Automatically rewritten from C to Rust
//! Source: drivers/vfio/platform/reset/vfio_platform_calxedaxgmac.c
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
//
// VFIO platform driver specialized for Calxeda xgmac reset
// reset code is inherited from calxeda xgmac native driver
//
// Copyright 2010-2011 Calxeda, Inc.
// Copyright (c) 2015 Linaro Ltd.
// www.linaro.org
//

// XGMAC Register definitions
pub const XGMAC_CONTROL: c_uint = 0x00000000      /* MAC Configuration */;
// DMA Control and Status Registers
pub const XGMAC_DMA_CONTROL: c_uint = 0x00000f18      /* Ctrl (Operational Mode) */;
pub const XGMAC_DMA_INTR_ENA: c_uint = 0x00000f1c      /* Interrupt Enable */;
// DMA Control register defines
pub const DMA_CONTROL_ST: c_uint = 0x00002000      /* Start/Stop Transmission */;
pub const DMA_CONTROL_SR: c_uint = 0x00000002      /* Start/Stop Receive */;
// Common MAC defines
pub const MAC_ENABLE_TX: c_uint = 0x00000008      /* Transmitter Enable */;
pub const MAC_ENABLE_RX: c_uint = 0x00000004      /* Receiver Enable */;
#[no_mangle]
pub unsafe extern "C" fn xgmac_mac_disable(ioaddr: *mut void __iomem) {
    static inline void xgmac_mac_disable(void __iomem *ioaddr)
    {
    let mut value: u32 = readl(ioaddr + XGMAC_DMA_CONTROL);
    value &= ~(DMA_CONTROL_ST | DMA_CONTROL_SR);
    writel(value, ioaddr + XGMAC_DMA_CONTROL);
    value = readl(ioaddr + XGMAC_CONTROL);
    value &= ~(MAC_ENABLE_TX | MAC_ENABLE_RX);
    writel(value, ioaddr + XGMAC_CONTROL);
    }
#[no_mangle]
unsafe extern "C" fn vfio_platform_calxedaxgmac_reset(vdev: *mut vfio_platform_device) -> c_int {
    static int vfio_platform_calxedaxgmac_reset(struct vfio_platform_device *vdev)
    {
    struct vfio_platform_region *reg = &vdev.regions[0];
    dev_err_once(vdev.device, "DEPRECATION: VFIO Calxeda xgmac platform reset is deprecated and will be removed in a future kernel release\n");
    if (!reg.ioaddr) {
    reg.ioaddr =
    ioremap(reg.addr, reg.size);
    if (!reg.ioaddr)
    return -ENOMEM;
    }
// disable IRQ
    writel(0, reg.ioaddr + XGMAC_DMA_INTR_ENA);
// Disable the MAC core
    xgmac_mac_disable(reg.ioaddr);
    return 0;
    }
    module_vfio_reset_handler("calxeda,hb-xgmac", vfio_platform_calxedaxgmac_reset);
    MODULE_VERSION(DRIVER_VERSION);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR(DRIVER_AUTHOR);
    MODULE_DESCRIPTION(DRIVER_DESC);
