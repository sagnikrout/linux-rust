//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/bcm_sf2.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Broadcom Starfighter2 private context
//
// Copyright (C) 2014, Broadcom Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_sf2_hw_params {
    pub top_rev: u16,
    pub core_rev: u16,
    pub gphy_rev: u16,
    pub num_gphy: u32,
    pub num_acb_queue: u8,
    pub num_rgmii: u8,
    pub num_ports: u8,
    pub fcb_pause_override:1: u8,
    pub acb_packets_inflight:1: u8,
}

pub const BCM_SF2_REGS_NUM: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_sf2_port_status {
    pub mode: phy_interface_t,
    pub link: c_uint,
    pub enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_sf2_cfp_priv {
// Mutex protecting concurrent accesses to the CFP registers
    pub lock: mutex,
    pub CFP_NUM_RULES): DECLARE_BITMAP(used,,
    pub CFP_NUM_RULES): DECLARE_BITMAP(unique,,
    pub rules_cnt: c_uint,
    pub rules_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_sf2_priv {
// Base registers, keep those in order with BCM_SF2_REGS_NAME
    pub core: *mut void __iomem,
    pub reg: *mut void __iomem,
    pub intrl2_0: *mut void __iomem,
    pub intrl2_1: *mut void __iomem,
    pub fcb: *mut void __iomem,
    pub acb: *mut void __iomem,
    pub rcdev: *mut reset_control,
// Register offsets indirection tables
    pub type: u32,
    pub reg_offsets: *const u16,
    pub core_reg_align: c_uint,
    pub num_cfp_rules: c_uint,
    pub num_crossbar_int_ports: c_uint,
    pub num_crossbar_ext_bits: c_uint,
// spinlock protecting access to the indirect registers
    pub indir_lock: spinlock_t,
    pub irq0: c_int,
    pub irq1: c_int,
    pub irq0_stat: u32,
    pub irq0_mask: u32,
    pub irq1_stat: u32,
    pub irq1_mask: u32,
// Backing b53_device
    pub dev: *mut b53_device,
    pub hw_params: bcm_sf2_hw_params,
    pub port_sts: [bcm_sf2_port_status; DSA_MAX_PORTS],
// Mask of ports enabled for Wake-on-LAN
    pub wol_ports_mask: u32,
    pub clk: *mut clk,
    pub clk_mdiv: *mut clk,
// MoCA port location
    pub moca_port: c_int,
// Bitmask of ports having an integrated PHY
    pub int_phy_mask: c_uint,
// Master and slave MDIO bus controller
    pub indir_phy_mask: c_uint,
    pub user_mii_bus: *mut mii_bus,
    pub master_mii_bus: *mut mii_bus,
// Bitmask of ports needing BRCM tags
    pub brcm_tag_mask: c_uint,
// CFP rules context
    pub cfp: bcm_sf2_cfp_priv,
}

// Accesses to 64-bits register requires us to latch the hi/lo pairs
// using the REG_DIR_DATA_{READ,WRITE} ancillary registers. The 'indir_lock'
// spinlock is automatically grabbed and released to provide relative
// atomiticy with latched reads/writes.
//

extern "C" {
    pub fn readl_relaxed(tmp: priv->core +) -> return;
}
extern "C" {
    pub fn readl_relaxed(priv->reg_offsets[off]: priv->reg +) -> return;
}
extern "C" {
    pub fn readl_relaxed(reg: priv->reg + priv->reg_offsets[off] +) -> return;
}
// RXNFC
extern "C" {
    pub fn bcm_sf2_cfp_rst(priv: *mut bcm_sf2_priv) -> c_int;
}
extern "C" {
    pub fn bcm_sf2_cfp_exit(ds: *mut dsa_switch);
}
extern "C" {
    pub fn bcm_sf2_cfp_resume(ds: *mut dsa_switch) -> c_int;
}
extern "C" {
    pub fn bcm_sf2_cfp_get_sset_count(ds: *mut dsa_switch, port: c_int, sset: c_int) -> c_int;
}
