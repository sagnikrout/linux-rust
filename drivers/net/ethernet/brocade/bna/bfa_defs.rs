//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/brocade/bna/bfa_defs.h
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
// Linux network driver for QLogic BR-series Converged Network Adapter.
//
// Copyright (c) 2005-2014 Brocade Communications Systems, Inc.
// Copyright (c) 2014-2015 QLogic Corporation
// All rights reserved
// www.qlogic.com
//

pub const BFA_VERSION_LEN: c_int = 64;
// ---------------------- adapter definitions ------------
// BFA adapter level attributes.
//
// !< adapter serial num length
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_adapter_attr {
    pub manufacturer: [c_char; BFA_ADAPTER_MFG_NAME_LEN],
    pub serial_num: [c_char; BFA_ADAPTER_SERIAL_NUM_LEN],
    pub card_type: u32,
    pub model: [c_char; BFA_ADAPTER_MODEL_NAME_LEN],
    pub model_descr: [c_char; BFA_ADAPTER_MODEL_DESCR_LEN],
    pub pwwn: u64,
    pub node_symname: [c_char; FC_SYMNAME_MAX],
    pub hw_ver: [c_char; BFA_VERSION_LEN],
    pub fw_ver: [c_char; BFA_VERSION_LEN],
    pub optrom_ver: [c_char; BFA_VERSION_LEN],
    pub os_type: [c_char; BFA_ADAPTER_OS_TYPE_LEN],
    pub vpd: bfa_mfg_vpd,
    pub mac: [u8; ETH_ALEN],
    pub nports: u8,
    pub max_speed: u8,
    pub prototype: u8,
    pub asic_rev: c_char,
    pub pcie_gen: u8,
    pub pcie_lanes_orig: u8,
    pub pcie_lanes: u8,
    pub cna_capable: u8,
    pub is_mezz: u8,
    pub trunk_capable: u8,
}

// ---------------------- IOC definitions ------------
// Driver and firmware versions.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_ioc_driver_attr {
    pub /: *mut *mut char driver[BFA_IOC_DRIVER_LEN]; /!< driver name,
    pub /: *mut *mut char driver_ver[BFA_VERSION_LEN]; /!< driver version,
    pub /: *mut *mut char fw_ver[BFA_VERSION_LEN]; /!< firmware version,
    pub /: *mut *mut char bios_ver[BFA_VERSION_LEN]; /!< bios version,
    pub /: *mut *mut char efi_ver[BFA_VERSION_LEN]; /!< EFI version,
    pub /: *mut *mut char ob_ver[BFA_VERSION_LEN]; /!< openboot version,
}

// IOC PCI device attributes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_ioc_pci_attr {
    pub /: *mut *mut u16 vendor_id; /!< PCI vendor ID,
    pub /: *mut *mut u16 device_id; /!< PCI device ID,
    pub /: *mut *mut u16 ssid; /!< subsystem ID,
    pub /: *mut *mut u16 ssvid; /!< subsystem vendor ID,
    pub /: *mut *mut u32 pcifn; /!< PCI device function,
    pub /: *mut *mut u32 rsvd; / padding,
    pub /: *mut *mut char chip_rev[BFA_IOC_CHIP_REV_LEN]; /!< chip revision,
}

// IOC states
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_ioc_state {
    BFA_IOC_UNINIT		= 1,	/*!< IOC is in uninit state */
    BFA_IOC_RESET		= 2,	/*!< IOC is in reset state */
    BFA_IOC_SEMWAIT		= 3,	/*!< Waiting for IOC h/w semaphore */
    BFA_IOC_HWINIT		= 4,	/*!< IOC h/w is being initialized */
    BFA_IOC_GETATTR		= 5,	/*!< IOC is being configured */
    BFA_IOC_OPERATIONAL	= 6,	/*!< IOC is operational */
    BFA_IOC_INITFAIL	= 7,	/*!< IOC hardware failure */
    BFA_IOC_FAIL		= 8,	/*!< IOC heart-beat failure */
    BFA_IOC_DISABLING	= 9,	/*!< IOC is being disabled */
    BFA_IOC_DISABLED	= 10,	/*!< IOC is disabled */
    BFA_IOC_FWMISMATCH	= 11,	/*!< IOC f/w different from drivers */
    BFA_IOC_ENABLING	= 12,	/*!< IOC is being enabled */
    BFA_IOC_HWFAIL		= 13,	/*!< PCI mapping doesn't exist */
}

// IOC firmware stats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fw_ioc_stats {
    pub enable_reqs: u32,
    pub disable_reqs: u32,
    pub get_attr_reqs: u32,
    pub dbg_sync: u32,
    pub dbg_dump: u32,
    pub unknown_reqs: u32,
}

// IOC driver stats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_ioc_drv_stats {
    pub ioc_isrs: u32,
    pub ioc_enables: u32,
    pub ioc_disables: u32,
    pub ioc_hbfails: u32,
    pub ioc_boots: u32,
    pub stats_tmos: u32,
    pub hb_count: u32,
    pub disable_reqs: u32,
    pub enable_reqs: u32,
    pub disable_replies: u32,
    pub enable_replies: u32,
    pub rsvd: u32,
}

// IOC statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_ioc_stats {
    pub /: *mut *mut bfa_ioc_drv_stats drv_stats; /!< driver IOC stats,
    pub /: *mut *mut bfa_fw_ioc_stats fw_stats; /!< firmware IOC stats,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_ioc_type {
    BFA_IOC_TYPE_FC		= 1,
    BFA_IOC_TYPE_FCoE	= 2,
    BFA_IOC_TYPE_LL		= 3,
}

// IOC attributes returned in queries
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_ioc_attr {
    pub ioc_type: bfa_ioc_type,
    pub /: *mut *mut bfa_ioc_state state; /!< IOC state,
    pub /: *mut *mut bfa_adapter_attr adapter_attr; /!< HBA attributes,
    pub /: *mut *mut bfa_ioc_driver_attr driver_attr; /!< driver attr,
    pub pci_attr: bfa_ioc_pci_attr,
    pub /: *mut *mut u8 port_id; /!< port number,
    pub /: *mut *mut u8 port_mode; /!< enum bfa_mode,
    pub /: *mut *mut u8 cap_bm; /!< capability,
    pub /: *mut *mut u8 port_mode_cfg; /!< enum bfa_mode,
    pub /: *mut *mut u8 def_fn; /!< 1 if default fn,
    pub /: *mut *mut u8 rsvd[3]; /!< 64bit align,
}

// Adapter capability mask definition
// ---------------------- mfg definitions ------------
// Checksum size
pub const BFA_MFG_CHKSUM_SIZE: c_int = 16;
pub const BFA_MFG_PARTNUM_SIZE: c_int = 14;
pub const BFA_MFG_SUPPLIER_ID_SIZE: c_int = 10;
pub const BFA_MFG_SUPPLIER_PARTNUM_SIZE: c_int = 20;
pub const BFA_MFG_SUPPLIER_SERIALNUM_SIZE: c_int = 20;
pub const BFA_MFG_SUPPLIER_REVISION_SIZE: c_int = 4;
// BFA adapter manufacturing block definition.
//
// All numerical fields are in big-endian format.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_mfg_block {
    pub /: *mut *mut u8 version; / manufacturing block version,
    pub /: *mut *mut u8 mfg_sig[3]; / characters 'M', 'F', 'G',
    pub /: *mut *mut u16 mfgsize; / mfg block size,
    pub /: *mut *mut u16 u16_chksum; / old u16 checksum,
    pub brcd_serialnum: [c_char; STRSZ(BFA_MFG_SERIALNUM_SIZE)],
    pub brcd_partnum: [c_char; STRSZ(BFA_MFG_PARTNUM_SIZE)],
    pub /: *mut *mut u8 mfg_day; / manufacturing day,
    pub /: *mut *mut u8 mfg_month; / manufacturing month,
    pub /: *mut *mut u16 mfg_year; / manufacturing year,
    pub /: *mut *mut u64 mfg_wwn; / wwn base for this adapter,
    pub /: *mut *mut u8 num_wwn; / number of wwns assigned,
    pub /: *mut *mut u8 mfg_speeds; / speeds allowed for this adapter,
    pub rsv: [u8; 2],
    pub supplier_id: [c_char; STRSZ(BFA_MFG_SUPPLIER_ID_SIZE)],
    pub supplier_partnum: [c_char; STRSZ(BFA_MFG_SUPPLIER_PARTNUM_SIZE)],
    pub supplier_serialnum: [c_char; STRSZ(BFA_MFG_SUPPLIER_SERIALNUM_SIZE)],
    pub supplier_revision: [c_char; STRSZ(BFA_MFG_SUPPLIER_REVISION_SIZE)],
    pub /: *mut *mut u8 mfg_mac[ETH_ALEN]; / base mac address,
    pub /: *mut *mut u8 num_mac; / number of mac addresses,
    pub rsv2: u8,
    pub /: *mut *mut u32 card_type; / card type,
    pub /: *mut *mut char cap_nic; / capability nic,
    pub /: *mut *mut char cap_cna; / capability cna,
    pub /: *mut *mut char cap_hba; / capability hba,
    pub /: *mut *mut char cap_fc16g; / capability fc 16g,
    pub /: *mut *mut char cap_sriov; / capability sriov,
    pub /: *mut *mut char cap_mezz; / capability mezz,
    pub rsv3: u8,
    pub /: *mut *mut u8 mfg_nports; / number of ports,
    pub /: *mut *mut char media[8]; / xfi/xaui,
    pub /: *mut *mut char initial_mode[8]; / initial mode: hba/cna/nic,
    pub rsv4: [u8; 84],
    pub /: *mut *mut u8 md5_chksum[BFA_MFG_CHKSUM_SIZE]; / md5 checksum,
    pub __packed: },
// ---------------------- pci definitions ------------
//
// PCI device ID information
//
}

// PCI sub-system device and vendor ID information
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_mode {
    BFA_MODE_HBA		= 1,
    BFA_MODE_CNA		= 2,
    BFA_MODE_NIC		= 3
}

//
// Flash module specific
//

pub const BFA_TOTAL_FLASH_SIZE: c_uint = 0x400000;
pub const BFA_FLASH_PART_FWIMG: c_int = 2;
pub const BFA_FLASH_PART_MFG: c_int = 7;
//
// flash partition attributes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_flash_part_attr {
    pub /: *mut *mut u32 part_type; / partition type,
    pub /: *mut *mut u32 part_instance; / partition instance,
    pub /: *mut *mut u32 part_off; / partition offset,
    pub /: *mut *mut u32 part_size; / partition size,
    pub /: *mut *mut u32 part_len; / partition content length,
    pub /: *mut *mut u32 part_status; / partition status,
    pub 24]: char rsv[BFA_FLASH_PART_ENTRY_SIZE -,
}

//
// flash attributes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_flash_attr {
    pub /: *mut *mut u32 status; / flash overall status,
    pub /: *mut *mut u32 npart; / num of partitions,
    pub part: [bfa_flash_part_attr; BFA_FLASH_PART_MAX],
}
