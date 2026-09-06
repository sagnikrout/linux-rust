//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pmdomain/mediatek/mtk-pm-domains.h
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

// can't set MTK_SCPD_KEEP_DEFAULT_OFF at the same time

pub const SPM_VDE_PWR_CON: c_uint = 0x0210;
pub const SPM_MFG_PWR_CON: c_uint = 0x0214;
pub const SPM_VEN_PWR_CON: c_uint = 0x0230;
pub const SPM_ISP_PWR_CON: c_uint = 0x0238;
pub const SPM_DIS_PWR_CON: c_uint = 0x023c;
pub const SPM_CONN_PWR_CON: c_uint = 0x0280;
pub const SPM_MD1_PWR_CON: c_uint = 0x0284;
pub const SPM_VEN2_PWR_CON: c_uint = 0x0298;
pub const SPM_AUDIO_PWR_CON: c_uint = 0x029c;
pub const SPM_MFG_2D_PWR_CON: c_uint = 0x02c0;
pub const SPM_MFG_ASYNC_PWR_CON: c_uint = 0x02c4;
pub const SPM_USB_PWR_CON: c_uint = 0x02cc;
pub const SPM_PWR_STATUS: c_uint = 0x060c;
pub const SPM_PWR_STATUS_2ND: c_uint = 0x0610;

pub const SPM_MAX_BUS_PROT_DATA: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scpsys_bus_prot_flags {
    BUS_PROT_REG_UPDATE = BIT(1),
    BUS_PROT_IGNORE_CLR_ACK = BIT(2),
    BUS_PROT_INVERTED = BIT(3),
    BUS_PROT_IGNORE_SUBCLK = BIT(4),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scpsys_bus_prot_block {
    BUS_PROT_BLOCK_INFRA,
    BUS_PROT_BLOCK_INFRA_NAO,
    BUS_PROT_BLOCK_SMI,
    BUS_PROT_BLOCK_SPM,
    BUS_PROT_BLOCK_IMG_SUB0,
    BUS_PROT_BLOCK_CAM_SUB1,
    BUS_PROT_BLOCK_CAM_SUB0,
    BUS_PROT_BLOCK_IPE_SUB0,
    BUS_PROT_BLOCK_VLP,
    BUS_PROT_BLOCK_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scpsys_bus_prot_data {
    pub bus_prot_block: u8,
    pub bus_prot_sta_block: u8,
    pub bus_prot_set_clr_mask: u32,
    pub bus_prot_set: u32,
    pub bus_prot_clr: u32,
    pub bus_prot_sta_mask: u32,
    pub bus_prot_sta: u32,
    pub flags: u8,
}

//
// enum scpsys_rtff_type - Type of RTFF Hardware for power domain
// @SCPSYS_RTFF_NONE:          RTFF HW not present or domain not RTFF managed
// @SCPSYS_RTFF_TYPE_GENERIC:  Non-CPU, peripheral-generic RTFF HW
// @SCPSYS_RTFF_TYPE_PCIE_PHY: PCI-Express PHY specific RTFF HW
// @SCPSYS_RTFF_TYPE_STOR_UFS: Storage (UFS) specific RTFF HW
// @SCPSYS_RTFF_TYPE_MAX:      Number of supported RTFF HW Types
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scpsys_rtff_type {
    SCPSYS_RTFF_NONE = 0,
    SCPSYS_RTFF_TYPE_GENERIC,
    SCPSYS_RTFF_TYPE_PCIE_PHY,
    SCPSYS_RTFF_TYPE_STOR_UFS,
    SCPSYS_RTFF_TYPE_MAX
}

//
// enum scpsys_mtcmos_type - Type of power domain controller
// @SCPSYS_MTCMOS_TYPE_DIRECT_CTL: Power domains are controlled with direct access
// @SCPSYS_MTCMOS_TYPE_HW_VOTER:   Hardware-assisted voted power domain control
// @SCPSYS_MTCMOS_TYPE_MAX:        Number of supported power domain types
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scpsys_mtcmos_type {
    SCPSYS_MTCMOS_TYPE_DIRECT_CTL = 0,
    SCPSYS_MTCMOS_TYPE_HW_VOTER,
    SCPSYS_MTCMOS_TYPE_MAX
}

//
// struct scpsys_domain_data - scp domain data for power on/off flow
// @name: The name of the power domain.
// @sta_mask: The mask for power on/off status bit.
// @sta2nd_mask: The mask for second power on/off status bit.
// @ctl_offs: The offset for main power control register.
// @sram_pdn_bits: The mask for sram power control bits.
// @sram_pdn_ack_bits: The mask for sram power control acked bits.
// @ext_buck_iso_offs: The offset for external buck isolation
// @ext_buck_iso_mask: The mask for external buck isolation
// @caps: The flag for active wake-up action.
// @rtff_type: The power domain RTFF HW type
// @bp_cfg: bus protection configuration for any subsystem
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scpsys_domain_data {
    pub name: *const c_char,
    pub sta_mask: u32,
    pub sta2nd_mask: u32,
    pub ctl_offs: c_int,
    pub sram_pdn_bits: u32,
    pub sram_pdn_ack_bits: u32,
    pub ext_buck_iso_offs: c_int,
    pub ext_buck_iso_mask: u32,
    pub caps: u16,
    pub rtff_type: scpsys_rtff_type,
    pub bp_cfg: [scpsys_bus_prot_data; SPM_MAX_BUS_PROT_DATA],
    pub pwr_sta_offs: c_int,
    pub pwr_sta2nd_offs: c_int,
}

//
// struct scpsys_hwv_domain_data - Hardware Voter power domain data
// @name:       Name of the power domain
// @set:        Offset of the HWV SET register
// @clr:        Offset of the HWV CLEAR register
// @done:       Offset of the HWV DONE register
// @en:         Offset of the HWV ENABLE register
// @set_sta:    Offset of the HWV SET STATUS register
// @clr_sta:    Offset of the HWV CLEAR STATUS register
// @setclr_bit: The SET/CLR bit to enable/disable the power domain
// @sta_bit:    The SET/CLR STA bit to check for on/off ACK
// @caps:       The flag for active wake-up action
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scpsys_hwv_domain_data {
    pub name: *const c_char,
    pub set: u16,
    pub clr: u16,
    pub done: u16,
    pub en: u16,
    pub set_sta: u16,
    pub clr_sta: u16,
    pub setclr_bit: u8,
    pub sta_bit: u8,
    pub caps: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scpsys_soc_data {
    pub domains_data: *const scpsys_domain_data,
    pub num_domains: c_int,
    pub hwv_domains_data: *const scpsys_hwv_domain_data,
    pub num_hwv_domains: c_int,
    pub bus_prot_blocks: *mut scpsys_bus_prot_block,
    pub num_bus_prot_blocks: c_int,
    pub type: scpsys_mtcmos_type,
}
