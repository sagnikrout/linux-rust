//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/bfa/bfa_defs.h
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
// Copyright (c) 2005-2014 Brocade Communications Systems, Inc.
// Copyright (c) 2014- QLogic Corporation.
// All rights reserved
// www.qlogic.com
//
// Linux driver for QLogic BR-series Fibre Channel Host Bus Adapter.
//

pub const BFA_MFG_SERIALNUM_SIZE: c_int = 11;

//
// Manufacturing card type
//

//
// Check if Mezz card
//

//
// Check if the card having old wwn/mac handling
//

//
// VPD data length
//
pub const BFA_MFG_VPD_LEN: c_int = 512;
//
// VPD vendor tag
//
// All numerical fields are in big-endian format.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_mfg_vpd_s {
    pub /: *mut *mut u8 version; / vpd data version,
    pub /: *mut *mut u8 vpd_sig[3]; / characters 'V', 'P', 'D',
    pub /: *mut *mut u8 chksum; / u8 checksum,
    pub /: *mut *mut u8 vendor; / vendor,
    pub /: *mut *mut u8 len; / vpd data length excluding header,
    pub rsv: u8,
    pub /: *mut *mut u8 data[BFA_MFG_VPD_LEN]; / vpd data,
}

//
// Status return values
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_status {
    BFA_STATUS_OK		= 0,	/*  Success */
    BFA_STATUS_FAILED	= 1,	/*  Operation failed */
    BFA_STATUS_EINVAL	= 2,	/*  Invalid params Check input
// parameters
    BFA_STATUS_ENOMEM	= 3,	/*  Out of resources */
    BFA_STATUS_ETIMER	= 5,	/*  Timer expired - Retry, if persists,
// contact support
    BFA_STATUS_EPROTOCOL	= 6,	/*  Protocol error */
    BFA_STATUS_BADFLASH	= 9,	/*  Flash is bad */
    BFA_STATUS_SFP_UNSUPP	= 10,	/*  Unsupported SFP - Replace SFP */
    BFA_STATUS_UNKNOWN_VFID = 11,	/*  VF_ID not found */
    BFA_STATUS_DATACORRUPTED = 12,  /*  Diag returned data corrupted */
    BFA_STATUS_DEVBUSY	= 13,	/*  Device busy - Retry operation */
    BFA_STATUS_HDMA_FAILED  = 16,   /* Host dma failed contact support */
    BFA_STATUS_FLASH_BAD_LEN = 17,	/*  Flash bad length */
    BFA_STATUS_UNKNOWN_LWWN = 18,	/*  LPORT PWWN not found */
    BFA_STATUS_UNKNOWN_RWWN = 19,	/*  RPORT PWWN not found */
    BFA_STATUS_VPORT_EXISTS = 21,	/*  VPORT already exists */
    BFA_STATUS_VPORT_MAX	= 22,	/*  Reached max VPORT supported limit */
    BFA_STATUS_UNSUPP_SPEED	= 23,	/*  Invalid Speed Check speed setting */
    BFA_STATUS_INVLD_DFSZ	= 24,	/*  Invalid Max data field size */
    BFA_STATUS_CMD_NOTSUPP  = 26,   /*  Command/API not supported */
    BFA_STATUS_FABRIC_RJT	= 29,	/*  Reject from attached fabric */
    BFA_STATUS_UNKNOWN_VWWN = 30,	/*  VPORT PWWN not found */
    BFA_STATUS_PORT_OFFLINE = 34,	/*  Port is not online */
    BFA_STATUS_VPORT_WWN_BP	= 46,	/*  WWN is same as base port's WWN */
    BFA_STATUS_PORT_NOT_DISABLED = 47, /* Port not disabled disable port */
    BFA_STATUS_NO_FCPIM_NEXUS = 52,	/* No FCP Nexus exists with the rport */
    BFA_STATUS_IOC_FAILURE	= 56,	/* IOC failure - Retry, if persists
// contact support
    BFA_STATUS_INVALID_WWN	= 57,	/*  Invalid WWN */
    BFA_STATUS_ADAPTER_ENABLED = 60, /* Adapter is not disabled */
    BFA_STATUS_IOC_NON_OP   = 61,	/* IOC is not operational */
    BFA_STATUS_VERSION_FAIL = 70, /* Application/Driver version mismatch */
    BFA_STATUS_DIAG_BUSY	= 71,	/*  diag busy */
    BFA_STATUS_BEACON_ON    = 72,   /* Port Beacon already on */
    BFA_STATUS_ENOFSAVE	= 78,	/*  No saved firmware trace */
    BFA_STATUS_IOC_DISABLED = 82,   /* IOC is already disabled */
    BFA_STATUS_ERROR_TRL_ENABLED  = 87,   /* TRL is enabled */
    BFA_STATUS_ERROR_QOS_ENABLED  = 88,   /* QoS is enabled */
    BFA_STATUS_NO_SFP_DEV = 89,	/* No SFP device check or replace SFP */
    BFA_STATUS_MEMTEST_FAILED = 90, /* Memory test failed contact support */
    BFA_STATUS_LEDTEST_OP = 109, /* LED test is operating */
    BFA_STATUS_INVALID_MAC  = 134, /*  Invalid MAC address */
    BFA_STATUS_CMD_NOTSUPP_CNA = 146, /* Command not supported for CNA */
    BFA_STATUS_PBC		= 154, /*  Operation not allowed for pre-boot
// configuration
    BFA_STATUS_BAD_FWCFG = 156,	/* Bad firmware configuration */
    BFA_STATUS_INVALID_VENDOR = 158, /* Invalid switch vendor */
    BFA_STATUS_SFP_NOT_READY = 159,	/* SFP info is not ready. Retry */
    BFA_STATUS_TRUNK_ENABLED = 164, /* Trunk is already enabled on
// this adapter
    BFA_STATUS_TRUNK_DISABLED  = 165, /* Trunking is disabled on
// the adapter
    BFA_STATUS_IOPROFILE_OFF = 175, /* IO profile OFF */
    BFA_STATUS_PHY_NOT_PRESENT = 183, /* PHY module not present */
    BFA_STATUS_FEATURE_NOT_SUPPORTED = 192,	/* Feature not supported */
    BFA_STATUS_ENTRY_EXISTS = 193,	/* Entry already exists */
    BFA_STATUS_ENTRY_NOT_EXISTS = 194, /* Entry does not exist */
    BFA_STATUS_NO_CHANGE = 195,	/* Feature already in that state */
    BFA_STATUS_FAA_ENABLED = 197,	/* FAA is already enabled */
    BFA_STATUS_FAA_DISABLED = 198,	/* FAA is already disabled */
    BFA_STATUS_FAA_ACQUIRED = 199,	/* FAA is already acquired */
    BFA_STATUS_FAA_ACQ_ADDR = 200,	/* Acquiring addr */
    BFA_STATUS_BBCR_FC_ONLY = 201, /*!< BBCredit Recovery is supported for *
// FC mode only
    BFA_STATUS_ERROR_TRUNK_ENABLED = 203,	/* Trunk enabled on adapter */
    BFA_STATUS_MAX_ENTRY_REACHED = 212,	/* MAX entry reached */
    BFA_STATUS_TOPOLOGY_LOOP = 230, /* Topology is set to Loop */
    BFA_STATUS_LOOP_UNSUPP_MEZZ = 231, /* Loop topology is not supported
// on mezz cards
    BFA_STATUS_INVALID_BW = 233,	/* Invalid bandwidth value */
    BFA_STATUS_QOS_BW_INVALID = 234,   /* Invalid QOS bandwidth
// configuration
    BFA_STATUS_DPORT_ENABLED = 235, /* D-port mode is already enabled */
    BFA_STATUS_DPORT_DISABLED = 236, /* D-port mode is already disabled */
    BFA_STATUS_CMD_NOTSUPP_MEZZ = 239, /* Cmd not supported for MEZZ card */
    BFA_STATUS_FRU_NOT_PRESENT = 240, /* fru module not present */
    BFA_STATUS_DPORT_NO_SFP = 243, /* SFP is not present.\n D-port will be
// enabled but it will be operational
// only after inserting a valid SFP.
    BFA_STATUS_DPORT_ERR = 245,	/* D-port mode is enabled */
    BFA_STATUS_DPORT_ENOSYS = 254, /* Switch has no D_Port functionality */
    BFA_STATUS_DPORT_CANT_PERF = 255, /* Switch port is not D_Port capable
// or D_Port is disabled
    BFA_STATUS_DPORT_LOGICALERR = 256, /* Switch D_Port fail */
    BFA_STATUS_DPORT_SWBUSY = 257, /* Switch port busy */
    BFA_STATUS_ERR_BBCR_SPEED_UNSUPPORT = 258, /*!< BB credit recovery is
// supported at max port speed alone
    BFA_STATUS_ERROR_BBCR_ENABLED  = 259, /*!< BB credit recovery
// is enabled
    BFA_STATUS_INVALID_BBSCN = 260, /*!< Invalid BBSCN value.
// Valid range is [1-15]
    BFA_STATUS_DDPORT_ERR = 261, /* Dynamic D_Port mode is active.\n To
// exit dynamic mode, disable D_Port on
// the remote port
    BFA_STATUS_DPORT_SFPWRAP_ERR = 262, /* Clear e/o_wrap fail, check or
// replace SFP
    BFA_STATUS_BBCR_CFG_NO_CHANGE = 265, /*!< BBCR is operational.
// Disable BBCR and try this operation again.
    BFA_STATUS_DPORT_SW_NOTREADY = 268, /* Remote port is not ready to
// start dport test. Check remote
// port status.
    BFA_STATUS_DPORT_INV_SFP = 271, /* Invalid SFP for D-PORT mode. */
    BFA_STATUS_DPORT_CMD_NOTSUPP    = 273, /* Dport is not supported by
// remote port
    BFA_STATUS_MAX_VAL		/* Unknown error code */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_eproto_status {
    BFA_EPROTO_BAD_ACCEPT = 0,
    BFA_EPROTO_UNKNOWN_RSP = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_boolean {
    BFA_FALSE = 0,
    BFA_TRUE  = 1
}

pub const BFA_STRING_32: c_int = 32;
pub const BFA_VERSION_LEN: c_int = 64;
//
// ---------------------- adapter definitions ------------
//
// BFA adapter level attributes.
//
// !< adapter serial num length
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_adapter_attr_s {
    pub manufacturer: [c_char; BFA_ADAPTER_MFG_NAME_LEN],
    pub serial_num: [c_char; BFA_ADAPTER_SERIAL_NUM_LEN],
    pub card_type: u32,
    pub model: [c_char; BFA_ADAPTER_MODEL_NAME_LEN],
    pub model_descr: [c_char; BFA_ADAPTER_MODEL_DESCR_LEN],
    pub pwwn: wwn_t,
    pub node_symname: [c_char; FC_SYMNAME_MAX],
    pub hw_ver: [c_char; BFA_VERSION_LEN],
    pub fw_ver: [c_char; BFA_VERSION_LEN],
    pub optrom_ver: [c_char; BFA_VERSION_LEN],
    pub os_type: [c_char; BFA_ADAPTER_OS_TYPE_LEN],
    pub vpd: bfa_mfg_vpd_s,
    pub mac: mac_s,
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
    pub /: *mut *mut u8 mfg_day; / manufacturing day,
    pub /: *mut *mut u8 mfg_month; / manufacturing month,
    pub /: *mut *mut u16 mfg_year; / manufacturing year,
    pub rsvd: u16,
    pub uuid: [u8; BFA_ADAPTER_UUID_LEN],
}

//
// ---------------------- IOC definitions ------------
//
// Driver and firmware versions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_ioc_driver_attr_s {
    pub /: *mut *mut char driver[BFA_IOC_DRIVER_LEN]; / driver name,
    pub /: *mut *mut char driver_ver[BFA_VERSION_LEN]; / driver version,
    pub /: *mut *mut char fw_ver[BFA_VERSION_LEN]; / firmware version,
    pub /: *mut *mut char bios_ver[BFA_VERSION_LEN]; / bios version,
    pub /: *mut *mut char efi_ver[BFA_VERSION_LEN]; / EFI version,
    pub /: *mut *mut char ob_ver[BFA_VERSION_LEN]; / openboot version,
}

//
// IOC PCI device attributes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_ioc_pci_attr_s {
    pub /: *mut *mut u16 vendor_id; / PCI vendor ID,
    pub /: *mut *mut u16 device_id; / PCI device ID,
    pub /: *mut *mut u16 ssid; / subsystem ID,
    pub /: *mut *mut u16 ssvid; / subsystem vendor ID,
    pub /: *mut *mut u32 pcifn; / PCI device function,
    pub /: *mut *mut u32 rsvd; / padding,
    pub /: *mut *mut char chip_rev[BFA_IOC_CHIP_REV_LEN]; / chip revision,
}

//
// IOC states
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_ioc_state {
    BFA_IOC_UNINIT		= 1,	/*  IOC is in uninit state */
    BFA_IOC_RESET		= 2,	/*  IOC is in reset state */
    BFA_IOC_SEMWAIT		= 3,	/*  Waiting for IOC h/w semaphore */
    BFA_IOC_HWINIT		= 4,	/*  IOC h/w is being initialized */
    BFA_IOC_GETATTR		= 5,	/*  IOC is being configured */
    BFA_IOC_OPERATIONAL	= 6,	/*  IOC is operational */
    BFA_IOC_INITFAIL	= 7,	/*  IOC hardware failure */
    BFA_IOC_FAIL		= 8,	/*  IOC heart-beat failure */
    BFA_IOC_DISABLING	= 9,	/*  IOC is being disabled */
    BFA_IOC_DISABLED	= 10,	/*  IOC is disabled */
    BFA_IOC_FWMISMATCH	= 11,	/*  IOC f/w different from drivers */
    BFA_IOC_ENABLING	= 12,	/*  IOC is being enabled */
    BFA_IOC_HWFAIL		= 13,	/*  PCI mapping doesn't exist */
    BFA_IOC_ACQ_ADDR	= 14,	/*  Acquiring addr from fabric */
}

//
// IOC firmware stats
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fw_ioc_stats_s {
    pub enable_reqs: u32,
    pub disable_reqs: u32,
    pub get_attr_reqs: u32,
    pub dbg_sync: u32,
    pub dbg_dump: u32,
    pub unknown_reqs: u32,
}

//
// IOC driver stats
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_ioc_drv_stats_s {
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

//
// IOC statistics
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_ioc_stats_s {
    pub /: *mut *mut bfa_ioc_drv_stats_s drv_stats; / driver IOC stats,
    pub /: *mut *mut bfa_fw_ioc_stats_s fw_stats; / firmware IOC stats,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_ioc_type_e {
    BFA_IOC_TYPE_FC		= 1,
    BFA_IOC_TYPE_FCoE	= 2,
    BFA_IOC_TYPE_LL		= 3,
}

//
// IOC attributes returned in queries
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_ioc_attr_s {
    pub ioc_type: bfa_ioc_type_e,
    pub /: *mut *mut bfa_ioc_state state; / IOC state,
    pub /: *mut *mut bfa_adapter_attr_s adapter_attr; / HBA attributes,
    pub /: *mut *mut bfa_ioc_driver_attr_s driver_attr; / driver attr,
    pub pci_attr: bfa_ioc_pci_attr_s,
    pub /: *mut *mut u8 port_id; / port number,
    pub /: *mut *mut u8 port_mode; / bfa_mode_s,
    pub /: *mut *mut u8 cap_bm; / capability,
    pub /: *mut *mut u8 port_mode_cfg; / bfa_mode_s,
    pub /: *mut *mut u8 def_fn; / 1 if default fn,
    pub /: *mut *mut u8 rsvd[3]; / 64bit align,
}

//
// AEN related definitions
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_aen_category {
    BFA_AEN_CAT_ADAPTER	= 1,
    BFA_AEN_CAT_PORT	= 2,
    BFA_AEN_CAT_LPORT	= 3,
    BFA_AEN_CAT_RPORT	= 4,
    BFA_AEN_CAT_ITNIM	= 5,
    BFA_AEN_CAT_AUDIT	= 8,
    BFA_AEN_CAT_IOC		= 9,
}

// BFA adapter level events
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_adapter_aen_event {
    BFA_ADAPTER_AEN_ADD	= 1,	/* New Adapter found event */
    BFA_ADAPTER_AEN_REMOVE	= 2,	/* Adapter removed event */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_adapter_aen_data_s {
    pub serial_num: [c_char; BFA_ADAPTER_SERIAL_NUM_LEN],
    pub /: *mut *mut u32 nports; / Number of NPorts,
    pub /: *mut *mut wwn_t pwwn; / WWN of one of its physical port,
}

// BFA physical port Level events
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_port_aen_event {
    BFA_PORT_AEN_ONLINE	= 1,    /* Physical Port online event */
    BFA_PORT_AEN_OFFLINE	= 2,    /* Physical Port offline event */
    BFA_PORT_AEN_RLIR	= 3,    /* RLIR event, not supported */
    BFA_PORT_AEN_SFP_INSERT	= 4,    /* SFP inserted event */
    BFA_PORT_AEN_SFP_REMOVE	= 5,    /* SFP removed event */
    BFA_PORT_AEN_SFP_POM	= 6,    /* SFP POM event */
    BFA_PORT_AEN_ENABLE	= 7,    /* Physical Port enable event */
    BFA_PORT_AEN_DISABLE	= 8,    /* Physical Port disable event */
    BFA_PORT_AEN_AUTH_ON	= 9,    /* Physical Port auth success event */
    BFA_PORT_AEN_AUTH_OFF	= 10,   /* Physical Port auth fail event */
    BFA_PORT_AEN_DISCONNECT	= 11,   /* Physical Port disconnect event */
    BFA_PORT_AEN_QOS_NEG	= 12,   /* Base Port QOS negotiation event */
    BFA_PORT_AEN_FABRIC_NAME_CHANGE	= 13, /* Fabric Name/WWN change */
    BFA_PORT_AEN_SFP_ACCESS_ERROR	= 14, /* SFP read error event */
    BFA_PORT_AEN_SFP_UNSUPPORT	= 15, /* Unsupported SFP event */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_port_aen_sfp_pom {
    BFA_PORT_AEN_SFP_POM_GREEN = 1, /* Normal */
    BFA_PORT_AEN_SFP_POM_AMBER = 2, /* Warning */
    BFA_PORT_AEN_SFP_POM_RED   = 3, /* Critical */
    BFA_PORT_AEN_SFP_POM_MAX   = BFA_PORT_AEN_SFP_POM_RED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_port_aen_data_s {
    pub /: *mut *mut wwn_t pwwn; / WWN of the physical port,
    pub /: *mut *mut wwn_t fwwn; / WWN of the fabric port,
    pub /: *mut *mut u32 phy_port_num; / For SFP related events,
    pub ioc_type: u16,
    pub /: *mut *mut u16 level; / Only transitions will be informed,
    pub /: *mut *mut mac_t mac; / MAC address of the ethernet port,
    pub rsvd: u16,
}

// BFA AEN logical port events
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_lport_aen_event {
    BFA_LPORT_AEN_NEW	= 1,		/* LPort created event */
    BFA_LPORT_AEN_DELETE	= 2,		/* LPort deleted event */
    BFA_LPORT_AEN_ONLINE	= 3,		/* LPort online event */
    BFA_LPORT_AEN_OFFLINE	= 4,		/* LPort offline event */
    BFA_LPORT_AEN_DISCONNECT = 5,		/* LPort disconnect event */
    BFA_LPORT_AEN_NEW_PROP	= 6,		/* VPort created event */
    BFA_LPORT_AEN_DELETE_PROP = 7,		/* VPort deleted event */
    BFA_LPORT_AEN_NEW_STANDARD = 8,		/* VPort created event */
    BFA_LPORT_AEN_DELETE_STANDARD = 9,	/* VPort deleted event */
    BFA_LPORT_AEN_NPIV_DUP_WWN = 10,	/* VPort with duplicate WWN */
    BFA_LPORT_AEN_NPIV_FABRIC_MAX = 11,	/* Max NPIV in fabric/fport */
    BFA_LPORT_AEN_NPIV_UNKNOWN = 12,	/* Unknown NPIV Error code */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_lport_aen_data_s {
    pub /: *mut *mut u16 vf_id; / vf_id of this logical port,
    pub /: *mut *mut u16 roles; / Logical port mode,IM/TM/IP etc,
    pub rsvd: u32,
    pub /: *mut *mut wwn_t ppwwn; / WWN of its physical port,
    pub /: *mut *mut wwn_t lpwwn; / WWN of this logical port,
}

// BFA ITNIM events
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_itnim_aen_event {
    BFA_ITNIM_AEN_ONLINE	 = 1,	/* Target online */
    BFA_ITNIM_AEN_OFFLINE	 = 2,	/* Target offline */
    BFA_ITNIM_AEN_DISCONNECT = 3,	/* Target disconnected */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_itnim_aen_data_s {
    pub /: *mut *mut u16 vf_id; / vf_id of the IT nexus,
    pub rsvd: [u16; 3],
    pub /: *mut *mut wwn_t ppwwn; / WWN of its physical port,
    pub /: *mut *mut wwn_t lpwwn; / WWN of logical port,
    pub /: *mut *mut wwn_t rpwwn; / WWN of remote(target) port,
}

// BFA audit events
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_audit_aen_event {
    BFA_AUDIT_AEN_AUTH_ENABLE	= 1,
    BFA_AUDIT_AEN_AUTH_DISABLE	= 2,
    BFA_AUDIT_AEN_FLASH_ERASE	= 3,
    BFA_AUDIT_AEN_FLASH_UPDATE	= 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_audit_aen_data_s {
    pub pwwn: wwn_t,
    pub partition_inst: c_int,
    pub partition_type: c_int,
}

// BFA IOC level events
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_ioc_aen_event {
    BFA_IOC_AEN_HBGOOD  = 1,	/* Heart Beat restore event	*/
    BFA_IOC_AEN_HBFAIL  = 2,	/* Heart Beat failure event	*/
    BFA_IOC_AEN_ENABLE  = 3,	/* IOC enabled event		*/
    BFA_IOC_AEN_DISABLE = 4,	/* IOC disabled event		*/
    BFA_IOC_AEN_FWMISMATCH  = 5,	/* IOC firmware mismatch	*/
    BFA_IOC_AEN_FWCFG_ERROR = 6,	/* IOC firmware config error	*/
    BFA_IOC_AEN_INVALID_VENDOR = 7,
    BFA_IOC_AEN_INVALID_NWWN = 8,	/* Zero NWWN			*/
    BFA_IOC_AEN_INVALID_PWWN = 9	/* Zero PWWN			*/
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_ioc_aen_data_s {
    pub pwwn: wwn_t,
    pub ioc_type: u16,
    pub mac: mac_t,
}

//
// ---------------------- mfg definitions ------------
//
// Checksum size
//
pub const BFA_MFG_CHKSUM_SIZE: c_int = 16;
pub const BFA_MFG_PARTNUM_SIZE: c_int = 14;
pub const BFA_MFG_SUPPLIER_ID_SIZE: c_int = 10;
pub const BFA_MFG_SUPPLIER_PARTNUM_SIZE: c_int = 20;
pub const BFA_MFG_SUPPLIER_SERIALNUM_SIZE: c_int = 20;
pub const BFA_MFG_SUPPLIER_REVISION_SIZE: c_int = 4;
//
// Initial capability definition
//
pub const BFA_MFG_IC_FC: c_uint = 0x01;
pub const BFA_MFG_IC_ETH: c_uint = 0x02;
//
// Adapter capability mask definition
//
pub const BFA_CM_HBA: c_uint = 0x01;
pub const BFA_CM_CNA: c_uint = 0x02;
pub const BFA_CM_NIC: c_uint = 0x04;
pub const BFA_CM_FC16G: c_uint = 0x08;
pub const BFA_CM_SRIOV: c_uint = 0x10;
pub const BFA_CM_MEZZ: c_uint = 0x20;

//
// All numerical fields are in big-endian format.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_mfg_block_s {
    pub /: *mut *mut u8 version; /!< manufacturing block version,
    pub /: *mut *mut u8 mfg_sig[3]; /!< characters 'M', 'F', 'G',
    pub /: *mut *mut u16 mfgsize; /!< mfg block size,
    pub /: *mut *mut u16 u16_chksum; /!< old u16 checksum,
    pub brcd_serialnum: [c_char; STRSZ(BFA_MFG_SERIALNUM_SIZE)],
    pub brcd_partnum: [c_char; STRSZ(BFA_MFG_PARTNUM_SIZE)],
    pub /: *mut *mut u8 mfg_day; /!< manufacturing day,
    pub /: *mut *mut u8 mfg_month; /!< manufacturing month,
    pub /: *mut *mut u16 mfg_year; /!< manufacturing year,
    pub /: *mut *mut wwn_t mfg_wwn; /!< wwn base for this adapter,
    pub /: *mut *mut u8 num_wwn; /!< number of wwns assigned,
    pub /: *mut *mut u8 mfg_speeds; /!< speeds allowed for this adapter,
    pub rsv: [u8; 2],
    pub supplier_id: [c_char; STRSZ(BFA_MFG_SUPPLIER_ID_SIZE)],
    pub supplier_partnum: [c_char; STRSZ(BFA_MFG_SUPPLIER_PARTNUM_SIZE)],
    pub supplier_serialnum: [c_char; STRSZ(BFA_MFG_SUPPLIER_SERIALNUM_SIZE)],
    pub supplier_revision: [c_char; STRSZ(BFA_MFG_SUPPLIER_REVISION_SIZE)],
    pub /: *mut *mut mac_t mfg_mac; /!< base mac address,
    pub /: *mut *mut u8 num_mac; /!< number of mac addresses,
    pub rsv2: u8,
    pub /: *mut *mut u32 card_type; /!< card type,
    pub /: *mut *mut char cap_nic; /!< capability nic,
    pub /: *mut *mut char cap_cna; /!< capability cna,
    pub /: *mut *mut char cap_hba; /!< capability hba,
    pub /: *mut *mut char cap_fc16g; /!< capability fc 16g,
    pub /: *mut *mut char cap_sriov; /!< capability sriov,
    pub /: *mut *mut char cap_mezz; /!< capability mezz,
    pub rsv3: u8,
    pub /: *mut *mut u8 mfg_nports; /!< number of ports,
    pub /: *mut *mut char media[8]; /!< xfi/xaui,
    pub /: *mut *mut char initial_mode[8]; /!< initial mode: hba/cna/nic,
    pub rsv4: [u8; 84],
    pub /: *mut *mut u8 md5_chksum[BFA_MFG_CHKSUM_SIZE]; /!< md5 checksum,
}

//
// ---------------------- pci definitions ------------
//
// PCI device and vendor ID information
//

//
// PCI sub-system device and vendor ID information
//
// Maximum number of device address ranges mapped through different BAR(s)
//
pub const BFA_PCI_ACCESS_RANGES: c_int = 1;
//
// Port speed settings. Each specific speed is a bit field. Use multiple
// bits to specify speeds to be selected for auto-negotiation.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_port_speed {
    BFA_PORT_SPEED_UNKNOWN = 0,
    BFA_PORT_SPEED_1GBPS	= 1,
    BFA_PORT_SPEED_2GBPS	= 2,
    BFA_PORT_SPEED_4GBPS	= 4,
    BFA_PORT_SPEED_8GBPS	= 8,
    BFA_PORT_SPEED_10GBPS	= 10,
    BFA_PORT_SPEED_16GBPS	= 16,
    BFA_PORT_SPEED_AUTO	= 0xf,
}

pub const BOOT_CFG_REV1: c_int = 1;
pub const BOOT_CFG_VLAN: c_int = 1;
//
// Boot options setting. Boot options setting determines from where
// to get the boot lun information
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_boot_bootopt {
    BFA_BOOT_AUTO_DISCOVER  = 0, /*  Boot from blun provided by fabric */
    BFA_BOOT_STORED_BLUN = 1, /*  Boot from bluns stored in flash */
    BFA_BOOT_FIRST_LUN      = 2, /*  Boot from first discovered blun */
    BFA_BOOT_PBC    = 3, /*  Boot from pbc configured blun  */
}

//
// Boot lun information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_boot_bootlun_s {
    pub /: *mut *mut wwn_t pwwn; / port wwn of target,
    pub /: *mut *mut scsi_lun lun; / 64-bit lun,
}

//
// BOOT boot configuraton
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_boot_cfg_s {
    pub version: u8,
    pub rsvd1: u8,
    pub chksum: u16,
    pub /: *mut *mut u8 enable; / enable/disable SAN boot,
    pub /: *mut *mut u8 speed; / boot speed settings,
    pub /: *mut *mut u8 topology; / boot topology setting,
    pub /: *mut *mut u8 bootopt; / bfa_boot_bootopt_t,
    pub /: *mut *mut u32 nbluns; / number of boot luns,
    pub rsvd2: u32,
    pub blun: [bfa_boot_bootlun_s; BFA_BOOT_BOOTLUN_MAX],
    pub blun_disc: [bfa_boot_bootlun_s; BFA_BOOT_BOOTLUN_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_boot_pbc_s {
    pub /: *mut *mut u8 enable; / enable/disable SAN boot,
    pub /: *mut *mut u8 speed; / boot speed settings,
    pub /: *mut *mut u8 topology; / boot topology setting,
    pub rsvd1: u8,
    pub /: *mut *mut u32 nbluns; / number of boot luns,
    pub pblun: [bfa_boot_bootlun_s; BFA_PREBOOT_BOOTLUN_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_ethboot_cfg_s {
    pub version: u8,
    pub rsvd1: u8,
    pub chksum: u16,
    pub /: *mut *mut u8 enable; / enable/disable Eth/PXE boot,
    pub rsvd2: u8,
    pub vlan: u16,
}

//
// ASIC block configuration related structures
//
pub const BFA_ABLK_MAX_PORTS: c_int = 2;
pub const BFA_ABLK_MAX_PFS: c_int = 16;
pub const BFA_ABLK_MAX: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_mode_s {
    BFA_MODE_HBA	= 1,
    BFA_MODE_CNA	= 2,
    BFA_MODE_NIC	= 3
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_adapter_cfg_mode_s {
    pub max_pf: u16,
    pub max_vf: u16,
    pub mode: bfa_mode_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_ablk_cfg_pf_s {
    pub pers: u16,
    pub port_id: u8,
    pub optrom: u8,
    pub valid: u8,
    pub sriov: u8,
    pub max_vfs: u8,
    pub rsvd: [u8; 1],
    pub num_qpairs: u16,
    pub num_vectors: u16,
    pub bw_min: u16,
    pub bw_max: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_ablk_cfg_port_s {
    pub mode: u8,
    pub type: u8,
    pub max_pfs: u8,
    pub rsvd: [u8; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_ablk_cfg_inst_s {
    pub nports: u8,
    pub max_pfs: u8,
    pub rsvd: [u8; 6],
    pub pf_cfg: [bfa_ablk_cfg_pf_s; BFA_ABLK_MAX_PFS],
    pub port_cfg: [bfa_ablk_cfg_port_s; BFA_ABLK_MAX_PORTS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_ablk_cfg_s {
    pub inst: [bfa_ablk_cfg_inst_s; BFA_ABLK_MAX],
}

//
// SFP module specific
//

// SFP state change notification event
pub const BFA_SFP_SCN_REMOVED: c_int = 0;
pub const BFA_SFP_SCN_INSERTED: c_int = 1;
pub const BFA_SFP_SCN_POM: c_int = 2;
pub const BFA_SFP_SCN_FAILED: c_int = 3;
pub const BFA_SFP_SCN_UNSUPPORT: c_int = 4;
pub const BFA_SFP_SCN_VALID: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_defs_sfp_media_e {
    BFA_SFP_MEDIA_UNKNOWN	= 0x00,
    BFA_SFP_MEDIA_CU	= 0x01,
    BFA_SFP_MEDIA_LW	= 0x02,
    BFA_SFP_MEDIA_SW	= 0x03,
    BFA_SFP_MEDIA_EL	= 0x04,
    BFA_SFP_MEDIA_UNSUPPORT	= 0x05,
}

//
// values for xmtr_tech above
//
// Serial ID: Data Fields -- Address A0h
// Basic ID field total 64 bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfp_srlid_base_s {
    pub /: *mut *mut u8 id; / 00: Identifier,
    pub /: *mut *mut u8 extid; / 01: Extended Identifier,
    pub /: *mut *mut u8 connector; / 02: Connector,
    pub /: *mut *mut u8 xcvr[8]; / 03-10: Transceiver,
    pub /: *mut *mut u8 encoding; / 11: Encoding,
    pub /: *mut *mut u8 br_norm; / 12: BR, Nominal,
    pub /: *mut *mut u8 rate_id; / 13: Rate Identifier,
    pub /: *mut *mut u8 len_km; / 14: Length single mode km,
    pub /: *mut *mut u8 len_100m; / 15: Length single mode 100m,
    pub /: *mut *mut u8 len_om2; / 16: Length om2 fiber 10m,
    pub /: *mut *mut u8 len_om1; / 17: Length om1 fiber 10m,
    pub /: *mut *mut u8 len_cu; / 18: Length copper 1m,
    pub /: *mut *mut u8 len_om3; / 19: Length om3 fiber 10m,
    pub /: *mut *mut u8 vendor_name[16];/ 20-35,
    pub unalloc1: u8,
    pub /: *mut *mut u8 vendor_oui[3]; / 37-39,
    pub /: *mut *mut u8 vendor_pn[16]; / 40-55,
    pub /: *mut *mut u8 vendor_rev[4]; / 56-59,
    pub /: *mut *mut u8 wavelen[2]; / 60-61,
    pub unalloc2: u8,
    pub /: *mut *mut u8 cc_base; / 63: check code for base id field,
}

//
// Serial ID: Data Fields -- Address A0h
// Extended id field total 32 bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfp_srlid_ext_s {
    pub options: [u8; 2],
    pub br_max: u8,
    pub br_min: u8,
    pub vendor_sn: [u8; 16],
    pub date_code: [u8; 8],
    pub /: *mut *mut u8 diag_mon_type; / 92: Diagnostic Monitoring type,
    pub en_options: u8,
    pub sff_8472: u8,
    pub cc_ext: u8,
}

//
// Diagnostic: Data Fields -- Address A2h
// Diagnostic and control/status base field total 96 bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfp_diag_base_s {
//
// Alarm and warning Thresholds 40 bytes
//
    pub /: *mut *mut u8 temp_high_alarm[2]; / 00-01,
    pub /: *mut *mut u8 temp_low_alarm[2]; / 02-03,
    pub /: *mut *mut u8 temp_high_warning[2]; / 04-05,
    pub /: *mut *mut u8 temp_low_warning[2]; / 06-07,
    pub /: *mut *mut u8 volt_high_alarm[2]; / 08-09,
    pub /: *mut *mut u8 volt_low_alarm[2]; / 10-11,
    pub /: *mut *mut u8 volt_high_warning[2]; / 12-13,
    pub /: *mut *mut u8 volt_low_warning[2]; / 14-15,
    pub /: *mut *mut u8 bias_high_alarm[2]; / 16-17,
    pub /: *mut *mut u8 bias_low_alarm[2]; / 18-19,
    pub /: *mut *mut u8 bias_high_warning[2]; / 20-21,
    pub /: *mut *mut u8 bias_low_warning[2]; / 22-23,
    pub /: *mut *mut u8 tx_pwr_high_alarm[2]; / 24-25,
    pub /: *mut *mut u8 tx_pwr_low_alarm[2]; / 26-27,
    pub /: *mut *mut u8 tx_pwr_high_warning[2]; / 28-29,
    pub /: *mut *mut u8 tx_pwr_low_warning[2]; / 30-31,
    pub /: *mut *mut u8 rx_pwr_high_alarm[2]; / 32-33,
    pub /: *mut *mut u8 rx_pwr_low_alarm[2]; / 34-35,
    pub /: *mut *mut u8 rx_pwr_high_warning[2]; / 36-37,
    pub /: *mut *mut u8 rx_pwr_low_warning[2]; / 38-39,
    pub unallocate_1: [u8; 16],
//
// ext_cal_const[36]
//
    pub rx_pwr: [u8; 20],
    pub tx_i: [u8; 4],
    pub tx_pwr: [u8; 4],
    pub temp: [u8; 4],
    pub volt: [u8; 4],
    pub unallocate_2: [u8; 3],
    pub cc_dmi: u8,
}

//
// Diagnostic: Data Fields -- Address A2h
// Diagnostic and control/status extended field total 24 bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfp_diag_ext_s {
    pub diag: [u8; SFP_DIAGMON_SIZE],
    pub unalloc1: [u8; 4],
    pub status_ctl: u8,
    pub rsvd: u8,
    pub alarm_flags: [u8; 2],
    pub unalloc2: [u8; 2],
    pub warning_flags: [u8; 2],
    pub ext_status_ctl: [u8; 2],
}

//
// Diagnostic: Data Fields -- Address A2h
// General Use Fields: User Writable Table - Features's Control Registers
// Total 32 bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfp_usr_eeprom_s {
    pub /: *mut *mut u8 rsvd1[2]; / 128-129,
    pub /: *mut *mut u8 ewrap; / 130,
    pub /: *mut *mut u8 rsvd2[2]; /,
    pub /: *mut *mut u8 owrap; / 133,
    pub /: *mut *mut u8 rsvd3[2]; /,
    pub /: *mut *mut u8 prbs; / 136: PRBS 7 generator,
    pub /: *mut *mut u8 rsvd4[2]; /,
    pub /: *mut *mut u8 tx_eqz_16; / 139: TX Equalizer (16xFC),
    pub /: *mut *mut u8 tx_eqz_8; / 140: TX Equalizer (8xFC),
    pub /: *mut *mut u8 rsvd5[2]; /,
    pub /: *mut *mut u8 rx_emp_16; / 143: RX Emphasis (16xFC),
    pub /: *mut *mut u8 rx_emp_8; / 144: RX Emphasis (8xFC),
    pub /: *mut *mut u8 rsvd6[2]; /,
    pub /: *mut *mut u8 tx_eye_adj; / 147: TX eye Threshold Adjust,
    pub /: *mut *mut u8 rsvd7[3]; /,
    pub /: *mut *mut u8 tx_eye_qctl; / 151: TX eye Quality Control,
    pub /: *mut *mut u8 tx_eye_qres; / 152: TX eye Quality Result,
    pub /: *mut *mut u8 rsvd8[2]; /,
    pub /: *mut *mut u8 poh[3]; / 155-157: Power On Hours,
    pub /: *mut *mut u8 rsvd9[2]; /,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfp_mem_s {
    pub srlid_base: sfp_srlid_base_s,
    pub srlid_ext: sfp_srlid_ext_s,
    pub diag_base: sfp_diag_base_s,
    pub diag_ext: sfp_diag_ext_s,
    pub usr_eeprom: sfp_usr_eeprom_s,
}

//
// transceiver codes (SFF-8472 Rev 10.2 Table 3.5)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union sfp_xcvr_e10g_code_u {
    pub b: u8,

    pub /: *mut *mut u8 e10g_unall:1; / 10G Ethernet compliance,
    pub e10g_lrm:1: u8,
    pub e10g_lr:1: u8,
    pub e10g_sr:1: u8,
    pub /: *mut *mut u8 ib_sx:1; / Infiniband compliance,
    pub ib_lx:1: u8,
    pub ib_cu_a:1: u8,
    pub ib_cu_p:1: u8,

    pub ib_cu_p:1: u8,
    pub ib_cu_a:1: u8,
    pub ib_lx:1: u8,
    pub /: *mut *mut u8 ib_sx:1; / Infiniband compliance,
    pub e10g_sr:1: u8,
    pub e10g_lr:1: u8,
    pub e10g_lrm:1: u8,
    pub /: *mut *mut u8 e10g_unall:1; / 10G Ethernet compliance,

    pub r: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union sfp_xcvr_so1_code_u {
    pub b: u8,
    pub /: *mut *mut u8 escon:2; / ESCON compliance code,
    pub /: *mut *mut u8 oc192_reach:1; / SONET compliance code,
    pub so_reach:2: u8,
    pub oc48_reach:3: u8,
    pub r: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union sfp_xcvr_so2_code_u {
    pub b: u8,
    pub reserved:1: u8,
    pub /: *mut *mut u8 oc12_reach:3; / OC12 reach,
    pub reserved1:1: u8,
    pub /: *mut *mut u8 oc3_reach:3; / OC3 reach,
    pub r: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union sfp_xcvr_eth_code_u {
    pub b: u8,
    pub base_px:1: u8,
    pub base_bx10:1: u8,
    pub e100base_fx:1: u8,
    pub e100base_lx:1: u8,
    pub e1000base_t:1: u8,
    pub e1000base_cx:1: u8,
    pub e1000base_lx:1: u8,
    pub e1000base_sx:1: u8,
    pub r: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfp_xcvr_fc1_code_s {
    pub /: *mut *mut u8 link_len:5; / FC link length,
    pub xmtr_tech2:3: u8,
    pub /: *mut *mut u8 xmtr_tech1:7; / FC transmitter technology,
    pub reserved1:1: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union sfp_xcvr_fc2_code_u {
    pub b: u8,
    pub /: *mut *mut u8 tw_media:1; / twin axial pair (tw),
    pub /: *mut *mut u8 tp_media:1; / shielded twisted pair (sp),
    pub /: *mut *mut u8 mi_media:1; / miniature coax (mi),
    pub /: *mut *mut u8 tv_media:1; / video coax (tv),
    pub /: *mut *mut u8 m6_media:1; / multimode, 62.5m (m6),
    pub /: *mut *mut u8 m5_media:1; / multimode, 50m (m5),
    pub reserved:1: u8,
    pub /: *mut *mut u8 sm_media:1; / single mode (sm),
    pub r: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union sfp_xcvr_fc3_code_u {
    pub b: u8,

    pub rsv4:1: u8,
    pub /: *mut *mut u8 mb800:1; / 800 Mbytes/sec,
    pub /: *mut *mut u8 mb1600:1; / 1600 Mbytes/sec,
    pub /: *mut *mut u8 mb400:1; / 400 Mbytes/sec,
    pub rsv2:1: u8,
    pub /: *mut *mut u8 mb200:1; / 200 Mbytes/sec,
    pub rsv1:1: u8,
    pub /: *mut *mut u8 mb100:1; / 100 Mbytes/sec,

    pub /: *mut *mut u8 mb100:1; / 100 Mbytes/sec,
    pub rsv1:1: u8,
    pub /: *mut *mut u8 mb200:1; / 200 Mbytes/sec,
    pub rsv2:1: u8,
    pub /: *mut *mut u8 mb400:1; / 400 Mbytes/sec,
    pub /: *mut *mut u8 mb1600:1; / 1600 Mbytes/sec,
    pub /: *mut *mut u8 mb800:1; / 800 Mbytes/sec,
    pub rsv4:1: u8,

    pub r: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfp_xcvr_s {
    pub e10g: sfp_xcvr_e10g_code_u,
    pub so1: sfp_xcvr_so1_code_u,
    pub so2: sfp_xcvr_so2_code_u,
    pub eth: sfp_xcvr_eth_code_u,
    pub fc1: sfp_xcvr_fc1_code_s,
    pub fc2: sfp_xcvr_fc2_code_u,
    pub fc3: sfp_xcvr_fc3_code_u,
}

//
// Flash module specific
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_flash_part_type {
    BFA_FLASH_PART_OPTROM   = 1,    /* option rom partition */
    BFA_FLASH_PART_FWIMG    = 2,    /* firmware image partition */
    BFA_FLASH_PART_FWCFG    = 3,    /* firmware tuneable config */
    BFA_FLASH_PART_DRV      = 4,    /* IOC driver config */
    BFA_FLASH_PART_BOOT     = 5,    /* boot config */
    BFA_FLASH_PART_ASIC     = 6,    /* asic bootstrap configuration */
    BFA_FLASH_PART_MFG      = 7,    /* manufacturing block partition */
    BFA_FLASH_PART_OPTROM2  = 8,    /* 2nd option rom partition */
    BFA_FLASH_PART_VPD      = 9,    /* vpd data of OEM info */
    BFA_FLASH_PART_PBC      = 10,   /* pre-boot config */
    BFA_FLASH_PART_BOOTOVL  = 11,   /* boot overlay partition */
    BFA_FLASH_PART_LOG      = 12,   /* firmware log partition */
    BFA_FLASH_PART_PXECFG   = 13,   /* pxe boot config partition */
    BFA_FLASH_PART_PXEOVL   = 14,   /* pxe boot overlay partition */
    BFA_FLASH_PART_PORTCFG  = 15,   /* port cfg partition */
    BFA_FLASH_PART_ASICBK   = 16,   /* asic backup partition */
}

//
// flash partition attributes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_flash_part_attr_s {
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
pub struct bfa_flash_attr_s {
    pub /: *mut *mut u32 status; / flash overall status,
    pub /: *mut *mut u32 npart; / num of partitions,
    pub part: [bfa_flash_part_attr_s; BFA_FLASH_PART_MAX],
}

//
// DIAG module specific
//
pub const LB_PATTERN_DEFAULT: c_uint = 0xB5B5B5B5;
pub const QTEST_CNT_DEFAULT: c_int = 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_diag_memtest_s {
    pub algo: u8,
    pub rsvd: [u8; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_diag_memtest_result {
    pub status: u32,
    pub addr: u32,
    pub /: *mut *mut u32 exp; / expect value read from reg,
    pub /: *mut *mut u32 act; / actually value read,
    pub /: *mut *mut u32 err_status; / error status reg,
    pub /: *mut *mut u32 err_status1; / extra error info reg,
    pub /: *mut *mut u32 err_addr; / error address reg,
    pub algo: u8,
    pub rsv: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_diag_loopback_result_s {
    pub /: *mut *mut u32 numtxmfrm; / no. of transmit frame,
    pub /: *mut *mut u32 numosffrm; / no. of outstanding frame,
    pub /: *mut *mut u32 numrcvfrm; / no. of received good frame,
    pub /: *mut *mut u32 badfrminf; / mis-match info,
    pub /: *mut *mut u32 badfrmnum; / mis-match fram number,
    pub /: *mut *mut u8 status; / loopback test result,
    pub rsvd: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_diag_dport_test_status {
    DPORT_TEST_ST_IDLE	= 0,    /* the test has not started yet. */
    DPORT_TEST_ST_FINAL	= 1,    /* the test done successfully */
    DPORT_TEST_ST_SKIP	= 2,    /* the test skipped */
    DPORT_TEST_ST_FAIL	= 3,    /* the test failed */
    DPORT_TEST_ST_INPRG	= 4,    /* the testing is in progress */
    DPORT_TEST_ST_RESPONDER	= 5,    /* test triggered from remote port */
    DPORT_TEST_ST_STOPPED	= 6,    /* the test stopped by user. */
    DPORT_TEST_ST_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_diag_dport_test_type {
    DPORT_TEST_ELOOP	= 0,
    DPORT_TEST_OLOOP	= 1,
    DPORT_TEST_ROLOOP	= 2,
    DPORT_TEST_LINK		= 3,
    DPORT_TEST_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_diag_dport_test_opmode {
    BFA_DPORT_OPMODE_AUTO	= 0,
    BFA_DPORT_OPMODE_MANU	= 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_diag_dport_subtest_result_s {
    pub /: *mut *mut u8 status; / bfa_diag_dport_test_status,
    pub /: *mut *mut u8 rsvd[7]; / 64bit align,
    pub /: *mut *mut u64 start_time; / timestamp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_diag_dport_result_s {
    pub /: *mut *mut wwn_t rp_pwwn; / switch port wwn,
    pub /: *mut *mut wwn_t rp_nwwn; / switch node wwn,
    pub /: *mut *mut u64 start_time; / user/sw start time,
    pub /: *mut *mut u64 end_time; / timestamp,
    pub /: *mut *mut u8 status; / bfa_diag_dport_test_status,
    pub /: *mut *mut u8 mode; / bfa_diag_dport_test_opmode,
    pub /: *mut *mut u8 rsvd; / 64bit align,
    pub /: *mut *mut u8 speed; / link speed for buf_reqd,
    pub buffer_required: u16,
    pub /: *mut *mut u16 frmsz; / frame size for buf_reqd,
    pub /: *mut *mut u32 lpcnt; / Frame count,
    pub /: *mut *mut u32 pat; / Pattern,
    pub /: *mut *mut u32 roundtrip_latency; / in nano sec,
    pub /: *mut *mut u32 est_cable_distance; / in meter,
    pub subtest: [bfa_diag_dport_subtest_result_s; DPORT_TEST_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_diag_ledtest_s {
    pub /: *mut *mut u32 cmd; / bfa_led_op_t,
    pub /: *mut *mut u32 color; / bfa_led_color_t,
    pub /: *mut *mut u16 freq; / no. of blinks every 10 secs,
    pub /: *mut *mut u8 led; / bitmap of LEDs to be tested,
    pub rsvd: [u8; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_diag_loopback_s {
    pub loopcnt: u32,
    pub pattern: u32,
    pub /: *mut *mut u8 lb_mode; / bfa_port_opmode_t,
    pub /: *mut *mut u8 speed; / bfa_port_speed_t,
    pub rsvd: [u8; 2],
}

//
// PHY module specific
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_phy_status_e {
    BFA_PHY_STATUS_GOOD	= 0, /* phy is good */
    BFA_PHY_STATUS_NOT_PRESENT	= 1, /* phy does not exist */
    BFA_PHY_STATUS_BAD	= 2, /* phy is bad */
}

//
// phy attributes for phy query
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_phy_attr_s {
    pub /: *mut *mut u32 status; / phy present/absent status,
    pub /: *mut *mut u32 length; / firmware length,
    pub /: *mut *mut u32 fw_ver; / firmware version,
    pub /: *mut *mut u32 an_status; / AN status,
    pub /: *mut *mut u32 pma_pmd_status; / PMA/PMD link status,
    pub /: *mut *mut u32 pma_pmd_signal; / PMA/PMD signal detect,
    pub /: *mut *mut u32 pcs_status; / PCS link status,
}

//
// phy stats
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_phy_stats_s {
    pub /: *mut *mut u32 status; / phy stats status,
    pub /: *mut *mut u32 link_breaks; / Num of link breaks after linkup,
    pub /: *mut *mut u32 pma_pmd_fault; / NPMA/PMD fault,
    pub /: *mut *mut u32 pcs_fault; / PCS fault,
    pub /: *mut *mut u32 speed_neg; / Num of speed negotiation,
    pub /: *mut *mut u32 tx_eq_training; / Num of TX EQ training,
    pub /: *mut *mut u32 tx_eq_timeout; / Num of TX EQ timeout,
    pub /: *mut *mut u32 crc_error; / Num of CRC errors,
}

