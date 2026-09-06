//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/netronome/nfp/nfp_main.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2015-2018 Netronome Systems, Inc.
//
// nfp_main.h
// Author: Jason McMullan <jason.mcmullan@netronome.com>
//

//
// struct nfp_dumpspec - NFP FW dump specification structure
// @size:	Size of the data
// @data:	Sequence of TLVs, each being an instruction to dump some data
// from FW
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_dumpspec {
    pub size: u32,
    pub data: [u8; ],
}

//
// struct nfp_pf - NFP PF-specific device structure
// @pdev:		Backpointer to PCI device
// @dev_info:		NFP ASIC params
// @cpp:		Pointer to the CPP handle
// @app:		Pointer to the APP handle
// @data_vnic_bar:	Pointer to the CPP area for the data vNICs' BARs
// @ctrl_vnic_bar:	Pointer to the CPP area for the ctrl vNIC's BAR
// @qc_area:		Pointer to the CPP area for the queues
// @mac_stats_bar:	Pointer to the CPP area for the MAC stats
// @mac_stats_mem:	Pointer to mapped MAC stats area
// @vf_cfg_bar:		Pointer to the CPP area for the VF configuration BAR
// @vf_cfg_mem:		Pointer to mapped VF configuration area
// @vfcfg_tbl2_area:	Pointer to the CPP area for the VF config table
// @vfcfg_tbl2:		Pointer to mapped VF config table
// @mbox:		RTSym of per-PCI PF mailbox (under devlink lock)
// @irq_entries:	Array of MSI-X entries for all vNICs
// @limit_vfs:		Number of VFs supported by firmware (~0 for PCI limit)
// @num_vfs:		Number of SR-IOV VFs enabled
// @fw_loaded:		Is the firmware loaded?
// @unload_fw_on_remove:Do we need to unload firmware on driver removal?
// @ctrl_vnic:		Pointer to the control vNIC if available
// @mip:		MIP handle
// @rtbl:		RTsym table
// @hwinfo:		HWInfo table
// @dumpspec:		Debug dump specification
// @dump_flag:		Store dump flag between set_dump and get_dump_flag
// @dump_len:		Store dump length between set_dump and get_dump_flag
// @eth_tbl:		NSP ETH table
// @nspi:		NSP identification info
// @hwmon_dev:		pointer to hwmon device
// @ddir:		Per-device debugfs directory
// @max_data_vnics:	Number of data vNICs app firmware supports
// @num_vnics:		Number of vNICs spawned
// @vnics:		Linked list of vNIC structures (struct nfp_net)
// @ports:		Linked list of port structures (struct nfp_port)
// @wq:			Workqueue for running works which need to grab @lock
// @port_refresh_work:	Work entry for taking netdevs out
// @shared_bufs:	Array of shared buffer structures if FW has any SBs
// @num_shared_bufs:	Number of elements in @shared_bufs
//
// Fields which may change after proble are protected by devlink instance lock.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_pf {
    pub pdev: *mut pci_dev,
    pub dev_info: *const nfp_dev_info,
    pub cpp: *mut nfp_cpp,
    pub app: *mut nfp_app,
    pub data_vnic_bar: *mut nfp_cpp_area,
    pub ctrl_vnic_bar: *mut nfp_cpp_area,
    pub qc_area: *mut nfp_cpp_area,
    pub mac_stats_bar: *mut nfp_cpp_area,
    pub mac_stats_mem: *mut u8 __iomem,
    pub vf_cfg_bar: *mut nfp_cpp_area,
    pub vf_cfg_mem: *mut u8 __iomem,
    pub vfcfg_tbl2_area: *mut nfp_cpp_area,
    pub vfcfg_tbl2: *mut u8 __iomem,
    pub mbox: *const nfp_rtsym,
    pub irq_entries: *mut msix_entry,
    pub limit_vfs: c_uint,
    pub num_vfs: c_uint,
    pub fw_loaded: bool,
    pub unload_fw_on_remove: bool,
    pub ctrl_vnic: *mut nfp_net,
    pub mip: *const nfp_mip,
    pub rtbl: *mut nfp_rtsym_table,
    pub hwinfo: *mut nfp_hwinfo,
    pub dumpspec: *mut nfp_dumpspec,
    pub dump_flag: u32,
    pub dump_len: u32,
    pub eth_tbl: *mut nfp_eth_table,
    pub nspi: *mut nfp_nsp_identify,
    pub hwmon_dev: *mut device,
    pub ddir: *mut dentry,
    pub max_data_vnics: c_uint,
    pub num_vnics: c_uint,
    pub vnics: list_head,
    pub ports: list_head,
    pub wq: *mut workqueue_struct,
    pub port_refresh_work: work_struct,
    pub shared_bufs: *mut nfp_shared_buf,
    pub num_shared_bufs: c_uint,
}

extern "C" {
    pub fn nfp_net_pci_probe(pf: *mut nfp_pf) -> c_int;
}
extern "C" {
    pub fn nfp_net_pci_remove(pf: *mut nfp_pf);
}
extern "C" {
    pub fn nfp_hwmon_register(pf: *mut nfp_pf) -> c_int;
}
extern "C" {
    pub fn nfp_hwmon_unregister(pf: *mut nfp_pf);
}
extern "C" {
    pub fn nfp_ctrl_tx(nn: *mut nfp_net, skb: *mut sk_buff) -> bool;
}
extern "C" {
    pub fn nfp_net_pf_get_app_id(pf: *mut nfp_pf) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfp_dump_diag {
    NFP_DUMP_NSP_DIAG = 0,
}

extern "C" {
    pub fn nfp_shared_buf_register(pf: *mut nfp_pf) -> c_int;
}
extern "C" {
    pub fn nfp_shared_buf_unregister(pf: *mut nfp_pf);
}
extern "C" {
    pub fn nfp_devlink_params_register(pf: *mut nfp_pf) -> c_int;
}
extern "C" {
    pub fn nfp_devlink_params_unregister(pf: *mut nfp_pf);
}
extern "C" {
    pub fn nfp_net_lr2speed(linkrate: c_uint) -> c_uint;
}
extern "C" {
    pub fn nfp_net_speed2lr(speed: c_uint) -> c_uint;
}
