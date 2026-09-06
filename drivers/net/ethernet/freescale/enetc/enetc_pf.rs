//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/freescale/enetc/enetc_pf.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
// Copyright 2017-2019 NXP

pub const ENETC_PF_NUM_RINGS: c_int = 8;
pub const ENETC_VLAN_HT_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum enetc_vf_flags {
    ENETC_VF_FLAG_PF_SET_MAC	= BIT(0),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enetc_vf_state {
    pub /: *mut *mut mutex lock; / Prevent concurrent access,
    pub flags: enetc_vf_flags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enetc_port_caps {
    pub num_msix: c_int,
    pub num_rx_bdr: c_int,
    pub num_tx_bdr: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enetc_pf_ops {
    pub addr): *const *const *const void (set_si_primary_mac)(struct enetc_hw hw, int si, u8,
    pub addr): *mut *mut *mut void (get_si_primary_mac)(struct enetc_hw hw, int si, u8,
    pub bus): *mut *mut *mut *mut phylink_pcs (create_pcs)(enetc_pf pf, mii_bus,
    pub pcs): *mut *mut void (destroy_pcs)(struct phylink_pcs,
    pub priv): *mut *mut int (enable_psfp)(struct enetc_ndev_priv,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enetc_pf {
    pub si: *mut enetc_si,
    pub /: *mut *mut int num_vfs; / number of active VFs, after sriov_init,
    pub /: *mut *mut int total_vfs; / max number of VFs, set for PF at probe,
    pub vf_state: *mut enetc_vf_state,
    pub mac_filter: [enetc_mac_filter; MADDR_TYPE],
    pub rxmsg: *mut enetc_msg_swbd,
    pub msg_task: work_struct,
    pub msg_int_name: [c_char; ENETC_INT_NAME_MAX],
    pub ENETC_VLAN_HT_SIZE): DECLARE_BITMAP(vlan_ht_filter,,
    pub VLAN_N_VID): DECLARE_BITMAP(active_vlans,,
    pub /: *mut *mut *mut mii_bus mdio; / saved for cleanup,
    pub imdio: *mut mii_bus,
    pub pcs: *mut phylink_pcs,
    pub if_mode: phy_interface_t,
    pub phylink_config: phylink_config,
    pub caps: enetc_port_caps,
    pub ops: *const enetc_pf_ops,
}
