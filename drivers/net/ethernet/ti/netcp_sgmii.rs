//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/ti/netcp_sgmii.c
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
// SGMI module initialisation
//
// Copyright (C) 2014 Texas Instruments Incorporated
// Authors:	Sandeep Nair <sandeep_n@ti.com>
// Sandeep Paulraj <s-paulraj@ti.com>
// Wingman Kwok <w-kwok2@ti.com>
//

// SGMII registers

#[no_mangle]
unsafe extern "C" fn sgmii_write_reg(base: *mut void __iomem, reg: c_int, val: u32) {
    static void sgmii_write_reg(void __iomem *base, int reg, u32 val)
    {
    writel(val, base + reg);
    }
#[no_mangle]
unsafe extern "C" fn sgmii_read_reg(base: *mut void __iomem, reg: c_int) -> u32 {
    static u32 sgmii_read_reg(void __iomem *base, int reg)
    {
    return readl(base + reg);
    }
#[no_mangle]
unsafe extern "C" fn sgmii_write_reg_bit(base: *mut void __iomem, reg: c_int, val: u32) {
    static void sgmii_write_reg_bit(void __iomem *base, int reg, u32 val)
    {
    writel((readl(base + reg) | val), base + reg);
    }
// port is 0 based
#[no_mangle]
pub unsafe extern "C" fn netcp_sgmii_reset(sgmii_ofs: *mut void __iomem, port: c_int) -> c_int {
    int netcp_sgmii_reset(void __iomem *sgmii_ofs, int port)
    {
// Soft reset
    sgmii_write_reg_bit(sgmii_ofs, SGMII_SRESET_REG(port),
    SGMII_SRESET_RESET);
    while ((sgmii_read_reg(sgmii_ofs, SGMII_SRESET_REG(port)) &
    SGMII_SRESET_RESET) != 0x0)
    ;
    return 0;
    }
// port is 0 based
#[no_mangle]
pub unsafe extern "C" fn netcp_sgmii_rtreset(sgmii_ofs: *mut void __iomem, port: c_int, set: bool) -> bool {
    bool netcp_sgmii_rtreset(void __iomem *sgmii_ofs, int port, bool set)
    {
    u32 reg;
    bool oldval;
// Initiate a soft reset
    reg = sgmii_read_reg(sgmii_ofs, SGMII_SRESET_REG(port));
    oldval = (reg & SGMII_SRESET_RTRESET) != 0x0;
    if (set)
    reg |= SGMII_SRESET_RTRESET;
    else
    reg &= ~SGMII_SRESET_RTRESET;
    sgmii_write_reg(sgmii_ofs, SGMII_SRESET_REG(port), reg);
    wmb();
    return oldval;
    }
#[no_mangle]
pub unsafe extern "C" fn netcp_sgmii_get_port_link(sgmii_ofs: *mut void __iomem, port: c_int) -> c_int {
    int netcp_sgmii_get_port_link(void __iomem *sgmii_ofs, int port)
    {
    let mut status: u32 = 0, link = 0;
    status = sgmii_read_reg(sgmii_ofs, SGMII_STATUS_REG(port));
    if ((status & SGMII_REG_STATUS_LINK) != 0)
    link = 1;
    return link;
    }
#[no_mangle]
pub unsafe extern "C" fn netcp_sgmii_config(sgmii_ofs: *mut void __iomem, port: c_int, interface: u32) -> c_int {
    int netcp_sgmii_config(void __iomem *sgmii_ofs, int port, u32 interface)
    {
    unsigned int i, status, mask;
    u32 mr_adv_ability;
    u32 control;
    switch (interface) {
    case SGMII_LINK_MAC_MAC_AUTONEG:
    mr_adv_ability	= 0x9801;
    control		= 0x21;
    break;
    case SGMII_LINK_MAC_PHY:
    case SGMII_LINK_MAC_PHY_NO_MDIO:
    mr_adv_ability	= 1;
    control		= 1;
    break;
    case SGMII_LINK_MAC_MAC_FORCED:
    mr_adv_ability	= 0x9801;
    control		= 0x20;
    break;
    case SGMII_LINK_MAC_FIBER:
    mr_adv_ability	= 0x20;
    control		= 0x1;
    break;
    default:
    WARN_ONCE(1, "Invalid sgmii interface: %d\n", interface);
    return -EINVAL;
    }
    sgmii_write_reg(sgmii_ofs, SGMII_CTL_REG(port), 0);
// Wait for the SerDes pll to lock
    for (i = 0; i < 1000; i++)  {
    usleep_range(1000, 2000);
    status = sgmii_read_reg(sgmii_ofs, SGMII_STATUS_REG(port));
    if ((status & SGMII_REG_STATUS_LOCK) != 0)
    break;
    }
    if ((status & SGMII_REG_STATUS_LOCK) == 0)
    pr_err("serdes PLL not locked\n");
    sgmii_write_reg(sgmii_ofs, SGMII_MRADV_REG(port), mr_adv_ability);
    sgmii_write_reg(sgmii_ofs, SGMII_CTL_REG(port), control);
    mask = SGMII_REG_STATUS_LINK;
    if (control & SGMII_REG_CONTROL_AUTONEG)
    mask |= SGMII_REG_STATUS_AUTONEG;
    for (i = 0; i < 1000; i++)  {
    usleep_range(200, 500);
    status = sgmii_read_reg(sgmii_ofs, SGMII_STATUS_REG(port));
    if ((status & mask) == mask)
    break;
    }
    return 0;
    }
