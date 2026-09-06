//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pds/pds_core_if.h
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


// SPDX-License-Identifier: (GPL-2.0 OR Linux-OpenIB) OR BSD-2-Clause
// Copyright(c) 2023 Advanced Micro Devices, Inc.
pub const PCI_VENDOR_ID_PENSANDO: c_uint = 0x1dd8;
pub const PCI_DEVICE_ID_PENSANDO_CORE_PF: c_uint = 0x100c;
pub const PCI_DEVICE_ID_VIRTIO_NET_TRANS: c_uint = 0x1000;
pub const PCI_DEVICE_ID_PENSANDO_IONIC_ETH_VF: c_uint = 0x1003;
pub const PCI_DEVICE_ID_PENSANDO_VDPA_VF: c_uint = 0x100b;
pub const PDS_CORE_BARS_MAX: c_int = 4;
pub const PDS_CORE_PCI_BAR_DBELL: c_int = 1;
// Bar0
pub const PDS_CORE_DEV_INFO_SIGNATURE: c_uint = 0x44455649 /* 'DEVI' */;
pub const PDS_CORE_BAR0_SIZE: c_uint = 0x8000;
pub const PDS_CORE_BAR0_DEV_INFO_REGS_OFFSET: c_uint = 0x0000;
pub const PDS_CORE_BAR0_DEV_CMD_REGS_OFFSET: c_uint = 0x0800;
pub const PDS_CORE_BAR0_DEV_CMD_DATA_REGS_OFFSET: c_uint = 0x0c00;
pub const PDS_CORE_BAR0_INTR_STATUS_OFFSET: c_uint = 0x1000;
pub const PDS_CORE_BAR0_INTR_CTRL_OFFSET: c_uint = 0x2000;
pub const PDS_CORE_DEV_CMD_DONE: c_uint = 0x00000001;
pub const PDS_CORE_DEVCMD_TIMEOUT: c_int = 5;
pub const PDS_CORE_CLIENT_ID: c_int = 0;
pub const PDS_CORE_ASIC_TYPE_CAPRI: c_int = 0;
//
// enum pds_core_cmd_opcode - Device commands
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pds_core_cmd_opcode {
// Core init
    PDS_CORE_CMD_NOP		= 0,
    PDS_CORE_CMD_IDENTIFY		= 1,
    PDS_CORE_CMD_RESET		= 2,
    PDS_CORE_CMD_INIT		= 3,

    PDS_CORE_CMD_FW_DOWNLOAD	= 4,
    PDS_CORE_CMD_FW_CONTROL		= 5,

    PDS_CORE_CMD_GET_COMPONENT_INFO	= 6,
    PDS_CORE_CMD_SEND_PKG_DATA	= 7,
    PDS_CORE_CMD_SEND_COMPONENT_TBL	= 8,
    PDS_CORE_CMD_SEND_COMPONENT	= 9,
    PDS_CORE_CMD_FINALIZE_UPDATE	= 10,
    PDS_CORE_CMD_MATCH_RECORD_DESC	= 11,
    PDS_CORE_CMD_HOST_MEM		= 12,

// SR/IOV commands
    PDS_CORE_CMD_VF_GETATTR		= 60,
    PDS_CORE_CMD_VF_SETATTR		= 61,
    PDS_CORE_CMD_VF_CTRL		= 62,

// Add commands before this line
    PDS_CORE_CMD_MAX,
    PDS_CORE_CMD_COUNT
}

//
// enum pds_core_status_code - Device command return codes
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pds_core_status_code {
    PDS_RC_SUCCESS	= 0,	/* Success */
    PDS_RC_EVERSION	= 1,	/* Incorrect version for request */
    PDS_RC_EOPCODE	= 2,	/* Invalid cmd opcode */
    PDS_RC_EIO	= 3,	/* I/O error */
    PDS_RC_EPERM	= 4,	/* Permission denied */
    PDS_RC_EQID	= 5,	/* Bad qid */
    PDS_RC_EQTYPE	= 6,	/* Bad qtype */
    PDS_RC_ENOENT	= 7,	/* No such element */
    PDS_RC_EINTR	= 8,	/* operation interrupted */
    PDS_RC_EAGAIN	= 9,	/* Try again */
    PDS_RC_ENOMEM	= 10,	/* Out of memory */
    PDS_RC_EFAULT	= 11,	/* Bad address */
    PDS_RC_EBUSY	= 12,	/* Device or resource busy */
    PDS_RC_EEXIST	= 13,	/* object already exists */
    PDS_RC_EINVAL	= 14,	/* Invalid argument */
    PDS_RC_ENOSPC	= 15,	/* No space left or alloc failure */
    PDS_RC_ERANGE	= 16,	/* Parameter out of range */
    PDS_RC_BAD_ADDR	= 17,	/* Descriptor contains a bad ptr */
    PDS_RC_DEV_CMD	= 18,	/* Device cmd attempted on AdminQ */
    PDS_RC_ENOSUPP	= 19,	/* Operation not supported */
    PDS_RC_ERROR	= 29,	/* Generic error */
    PDS_RC_ERDMA	= 30,	/* Generic RDMA error */
    PDS_RC_EVFID	= 31,	/* VF ID does not exist */
    PDS_RC_BAD_FW	= 32,	/* FW file is invalid or corrupted */
    PDS_RC_ECLIENT	= 33,   /* No such client id */
    PDS_RC_BAD_PCI	= 255,  /* Broken PCI when reading status */
}

//
// struct pds_core_drv_identity - Driver identity information
// @drv_type:         Driver type (enum pds_core_driver_type)
// @os_dist:          OS distribution, numeric format
// @os_dist_str:      OS distribution, string format
// @kernel_ver:       Kernel version, numeric format
// @kernel_ver_str:   Kernel version, string format
// @driver_ver_str:   Driver version, string format
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_drv_identity {
    pub drv_type: __le32,
    pub os_dist: __le32,
    pub os_dist_str: [c_char; 128],
    pub kernel_ver: __le32,
    pub kernel_ver_str: [c_char; 32],
    pub driver_ver_str: [c_char; 32],
}

//
// enum pds_core_dev_capability - Device capabilities
// @PDS_CORE_DEV_CAP_PLDM_FW_UPDATE: Device only supports FW update via PLDM
// @PDS_CORE_DEV_CAP_HOST_MEM: Device supports host memory for fw use
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pds_core_dev_capability {
    PDS_CORE_DEV_CAP_PLDM_FW_UPDATE = BIT(0),
    PDS_CORE_DEV_CAP_HOST_MEM = BIT(1),
}

pub const PDS_DEV_TYPE_MAX: c_int = 16;
//
// struct pds_core_dev_identity - Device identity information
// @version:	      Version of device identify
// @type:	      Identify type (0 for now)
// @state:	      Device state
// @rsvd:	      Word boundary padding
// @nlifs:	      Number of LIFs provisioned
// @nintrs:	      Number of interrupts provisioned
// @ndbpgs_per_lif:   Number of doorbell pages per LIF
// @intr_coal_mult:   Interrupt coalescing multiplication factor
// Scale user-supplied interrupt coalescing
// value in usecs to device units using:
// device units = usecs * mult / div
// @intr_coal_div:    Interrupt coalescing division factor
// Scale user-supplied interrupt coalescing
// value in usecs to device units using:
// device units = usecs * mult / div
// @vif_types:        How many of each VIF device type is supported
// @max_fw_slots:     Number of firmware components reported by device
// only supported on version >= PDS_CORE_IDENTITY_VERSION_2
// @rsvd2:	      Word boundary padding
// @capabilities:     Device capabilities
// only supported on version >= PDS_CORE_IDENTITY_VERSION_2
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_dev_identity {
    pub version: u8,
    pub type: u8,
    pub state: u8,
    pub rsvd: u8,
    pub nlifs: __le32,
    pub nintrs: __le32,
    pub ndbpgs_per_lif: __le32,
    pub intr_coal_mult: __le32,
    pub intr_coal_div: __le32,
    pub vif_types: [__le16; PDS_DEV_TYPE_MAX],
    pub max_fw_slots: __le16,
    pub rsvd2: [u8; 6],
    pub capabilities: __le64,
}

pub const PDS_CORE_IDENTITY_VERSION_1: c_int = 1;
pub const PDS_CORE_IDENTITY_VERSION_2: c_int = 2;
//
// struct pds_core_dev_identify_cmd - Driver/device identify command
// @opcode:	Opcode PDS_CORE_CMD_IDENTIFY
// @ver:	Highest version of identify supported by driver
//
// Expects to find driver identification info (struct pds_core_drv_identity)
// in cmd_regs->data.  Driver should keep the devcmd interface locked
// while preparing the driver info.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_dev_identify_cmd {
    pub opcode: u8,
    pub ver: u8,
}

//
// struct pds_core_dev_identify_comp - Device identify command completion
// @status:	Status of the command (enum pds_core_status_code)
// @ver:	Version of identify returned by device
//
// Device identification info (struct pds_core_dev_identity) can be found
// in cmd_regs->data.  Driver should keep the devcmd interface locked
// while reading the results.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_dev_identify_comp {
    pub status: u8,
    pub ver: u8,
}

//
// struct pds_core_dev_reset_cmd - Device reset command
// @opcode:	Opcode PDS_CORE_CMD_RESET
//
// Resets and clears all LIFs, VDevs, and VIFs on the device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_dev_reset_cmd {
    pub opcode: u8,
}

//
// struct pds_core_dev_reset_comp - Reset command completion
// @status:	Status of the command (enum pds_core_status_code)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_dev_reset_comp {
    pub status: u8,
}

//
// struct pds_core_dev_init_data - Pointers and info needed for the Core
// initialization PDS_CORE_CMD_INIT command.  The in and out structs are
// overlays on the pds_core_dev_cmd_regs.data space for passing data down
// to the firmware on init, and then returning initialization results.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_dev_init_data_in {
    pub adminq_q_base: __le64,
    pub adminq_cq_base: __le64,
    pub notifyq_cq_base: __le64,
    pub flags: __le32,
    pub intr_index: __le16,
    pub adminq_ring_size: u8,
    pub notifyq_ring_size: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_dev_init_data_out {
    pub core_hw_index: __le32,
    pub adminq_hw_index: __le32,
    pub notifyq_hw_index: __le32,
    pub adminq_hw_type: u8,
    pub notifyq_hw_type: u8,
}

//
// struct pds_core_dev_init_cmd - Core device initialize
// @opcode:          opcode PDS_CORE_CMD_INIT
//
// Initializes the core device and sets up the AdminQ and NotifyQ.
// Expects to find initialization data (struct pds_core_dev_init_data_in)
// in cmd_regs->data.  Driver should keep the devcmd interface locked
// while preparing the driver info.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_dev_init_cmd {
    pub opcode: u8,
}

//
// struct pds_core_dev_init_comp - Core init completion
// @status:     Status of the command (enum pds_core_status_code)
//
// Initialization result data (struct pds_core_dev_init_data_in)
// is found in cmd_regs->data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_dev_init_comp {
    pub status: u8,
}

//
// struct pds_core_fw_download_cmd - Firmware download command
// @opcode:     opcode
// @rsvd:	Word boundary padding
// @addr:       DMA address of the firmware buffer
// @offset:     offset of the firmware buffer within the full image
// @length:     number of valid bytes in the firmware buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_fw_download_cmd {
    pub opcode: u8,
    pub rsvd: [u8; 3],
    pub offset: __le32,
    pub addr: __le64,
    pub length: __le32,
}

//
// struct pds_core_fw_download_comp - Firmware download completion
// @status:     Status of the command (enum pds_core_status_code)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_fw_download_comp {
    pub status: u8,
}

//
// enum pds_core_fw_control_oper - FW control operations
// @PDS_CORE_FW_INSTALL_ASYNC:     Install firmware asynchronously
// @PDS_CORE_FW_INSTALL_STATUS:    Firmware installation status
// @PDS_CORE_FW_ACTIVATE_ASYNC:    Activate firmware asynchronously
// @PDS_CORE_FW_ACTIVATE_STATUS:   Firmware activate status
// @PDS_CORE_FW_UPDATE_CLEANUP:    Cleanup any firmware update leftovers
// @PDS_CORE_FW_GET_BOOT:          Return current active firmware slot
// @PDS_CORE_FW_SET_BOOT:          Set active firmware slot for next boot
// @PDS_CORE_FW_GET_LIST:          Return list of installed firmware images
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pds_core_fw_control_oper {
    PDS_CORE_FW_INSTALL_ASYNC          = 0,
    PDS_CORE_FW_INSTALL_STATUS         = 1,
    PDS_CORE_FW_ACTIVATE_ASYNC         = 2,
    PDS_CORE_FW_ACTIVATE_STATUS        = 3,
    PDS_CORE_FW_UPDATE_CLEANUP         = 4,
    PDS_CORE_FW_GET_BOOT               = 5,
    PDS_CORE_FW_SET_BOOT               = 6,
    PDS_CORE_FW_GET_LIST               = 7,
}

//
// enum pds_core_fw_slot - Firmware slot identifiers
// @PDS_CORE_FW_SLOT_INVALID: Let firmware select slot based on package metadata
// @PDS_CORE_FW_SLOT_A:       Primary firmware slot A
// @PDS_CORE_FW_SLOT_B:       Primary firmware slot B
// @PDS_CORE_FW_SLOT_GOLD:    Gold/recovery firmware slot
// @PDS_CORE_FW_SLOT_MAX:     Sentinel value indicating no slot resolved
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pds_core_fw_slot {
    PDS_CORE_FW_SLOT_INVALID    = 0,
    PDS_CORE_FW_SLOT_A	    = 1,
    PDS_CORE_FW_SLOT_B          = 2,
    PDS_CORE_FW_SLOT_GOLD       = 3,
    PDS_CORE_FW_SLOT_MAX        = 0xff,
}

//
// struct pds_core_fw_control_cmd - Firmware control command
// @opcode:    opcode
// @rsvd:      Word boundary padding
// @oper:      firmware control operation (enum pds_core_fw_control_oper)
// @slot:      slot to operate on (enum pds_core_fw_slot)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_fw_control_cmd {
    pub opcode: u8,
    pub rsvd: [u8; 3],
    pub oper: u8,
    pub slot: u8,
}

//
// struct pds_core_fw_control_comp - Firmware control copletion
// @status:	Status of the command (enum pds_core_status_code)
// @rsvd:	Word alignment space
// @slot:	Slot number (enum pds_core_fw_slot)
// @rsvd1:	Struct padding
// @color:	Color bit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_fw_control_comp {
    pub status: u8,
    pub rsvd: [u8; 3],
    pub slot: u8,
    pub rsvd1: [u8; 10],
    pub color: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_fw_name_info {
pub const PDS_CORE_FWSLOT_BUFLEN: c_int = 8;
pub const PDS_CORE_FWVERS_BUFLEN: c_int = 32;
    pub slotname: [c_char; PDS_CORE_FWSLOT_BUFLEN],
    pub fw_version: [c_char; PDS_CORE_FWVERS_BUFLEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_fw_list_info {
pub const PDS_CORE_FWVERS_LIST_LEN: c_int = 16;
    pub num_fw_slots: u8,
    pub fw_names: [pds_core_fw_name_info; PDS_CORE_FWVERS_LIST_LEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pds_core_vf_attr {
    PDS_CORE_VF_ATTR_SPOOFCHK	= 1,
    PDS_CORE_VF_ATTR_TRUST		= 2,
    PDS_CORE_VF_ATTR_MAC		= 3,
    PDS_CORE_VF_ATTR_LINKSTATE	= 4,
    PDS_CORE_VF_ATTR_VLAN		= 5,
    PDS_CORE_VF_ATTR_RATE		= 6,
    PDS_CORE_VF_ATTR_STATSADDR	= 7,
}

//
// enum pds_core_vf_link_status - Virtual Function link status
// @PDS_CORE_VF_LINK_STATUS_AUTO:   Use link state of the uplink
// @PDS_CORE_VF_LINK_STATUS_UP:     Link always up
// @PDS_CORE_VF_LINK_STATUS_DOWN:   Link always down
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pds_core_vf_link_status {
    PDS_CORE_VF_LINK_STATUS_AUTO = 0,
    PDS_CORE_VF_LINK_STATUS_UP   = 1,
    PDS_CORE_VF_LINK_STATUS_DOWN = 2,
}

//
// struct pds_core_vf_setattr_cmd - Set VF attributes on the NIC
// @opcode:     Opcode
// @attr:       Attribute type (enum pds_core_vf_attr)
// @vf_index:   VF index
// @macaddr:	mac address
// @vlanid:	vlan ID
// @maxrate:	max Tx rate in Mbps
// @spoofchk:	enable address spoof checking
// @trust:	enable VF trust
// @linkstate:	set link up or down
// @stats:	stats addr struct
// @stats.pa:	set DMA address for VF stats
// @stats.len:	length of VF stats space
// @pad:	force union to specific size
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_vf_setattr_cmd {
    pub opcode: u8,
    pub attr: u8,
    pub vf_index: __le16,
    pub macaddr: [u8; 6],
    pub vlanid: __le16,
    pub maxrate: __le32,
    pub spoofchk: u8,
    pub trust: u8,
    pub linkstate: u8,
    pub pa: __le64,
    pub len: __le32,
    pub stats: },
    pub pad: [u8; 60],
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_vf_setattr_comp {
    pub status: u8,
    pub attr: u8,
    pub vf_index: __le16,
    pub comp_index: __le16,
    pub rsvd: [u8; 9],
    pub color: u8,
}

//
// struct pds_core_vf_getattr_cmd - Get VF attributes from the NIC
// @opcode:     Opcode
// @attr:       Attribute type (enum pds_core_vf_attr)
// @vf_index:   VF index
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_vf_getattr_cmd {
    pub opcode: u8,
    pub attr: u8,
    pub vf_index: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_vf_getattr_comp {
    pub status: u8,
    pub attr: u8,
    pub vf_index: __le16,
    pub macaddr: [u8; 6],
    pub vlanid: __le16,
    pub maxrate: __le32,
    pub spoofchk: u8,
    pub trust: u8,
    pub linkstate: u8,
    pub stats_pa: __le64,
    pub pad: [u8; 11],
    pub __packed: },
    pub color: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pds_core_vf_ctrl_opcode {
    PDS_CORE_VF_CTRL_START_ALL	= 0,
    PDS_CORE_VF_CTRL_START		= 1,
}

//
// struct pds_core_vf_ctrl_cmd - VF control command
// @opcode:         Opcode for the command
// @ctrl_opcode:    VF control operation type
// @vf_index:       VF Index. It is unused if op START_ALL is used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_vf_ctrl_cmd {
    pub opcode: u8,
    pub ctrl_opcode: u8,
    pub vf_index: __le16,
}

//
// struct pds_core_vf_ctrl_comp - VF_CTRL command completion.
// @status:     Status of the command (enum pds_core_status_code)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_vf_ctrl_comp {
    pub status: u8,
}

//
// struct pds_core_send_pkg_data_cmd - Send package data command
// @opcode: Opcode PDS_CORE_CMD_SEND_PKG_DATA
// @ver: Driver's max support version of this command
// @total_len: Total length of the package data
// @offset: Offset in the package data, non-zero if multiple commands are
// needed for sending the package data
// @data_len: Length of data stored at data_pa
// @data_pa: Data physical address for DMA to device
//
// The package data may be too large to store in a single buffer, so multiple
// PDS_CORE_CMD_SEND_PKG_DATA devcmds may be needed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_send_pkg_data_cmd {
    pub opcode: u8,
    pub ver: u8,
    pub total_len: __le16,
    pub offset: __le16,
    pub data_len: __le16,
    pub data_pa: __le64,
}

//
// struct pds_core_send_pkg_data_comp - Send package data completion
// @status: Status of the command (enum pds_core_status_code)
// @ver: Device's max supported version of this command
// @rsvd: Word boundary padding
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_send_pkg_data_comp {
    pub status: u8,
    pub ver: u8,
    pub rsvd: [u8; 2],
}

//
// struct pds_core_component_tbl - Component table details
// @comparison_stamp: Comparison stamp used for component version checks
// @classification: Vendor specific classification info
// @identifier: Component's ID
// @transfer_flag: Part of the component table this request represents
// @version_str_type: The types of strings used
// @version_str_len: Length of @version_str
// @version_str: Component version information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_component_tbl {
    pub comparison_stamp: __le32,
    pub classification: __le16,
    pub identifier: __le16,
    pub transfer_flag: u8,
    pub version_str_type: u8,
    pub version_str_len: u8,
    pub version_str: [u8; ],
}

//
// struct pds_core_send_component_tbl_cmd - Send component table command
// @opcode: Opcode PDS_CORE_CMD_SEND_COMPONENT_TBL
// @ver: Driver's max support version of this command
// @slot_id: enum pds_core_fw_slot
// @rsvd: Word boundary padding
//
// Expects to find component table info (struct pds_core_component_tbl)
// in cmd_regs->data.  Driver should keep the devcmd interface locked
// while preparing the component table info.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_send_component_tbl_cmd {
    pub opcode: u8,
    pub ver: u8,
    pub slot_id: u8,
    pub rsvd: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pds_core_component_resp_code {
    PDS_CORE_COMPONENT_VALID = 0x0,
    PDS_CORE_COMPONENT_STAMP_IDENTICAL = 0x1,
    PDS_CORE_COMPONENT_STAMP_LOWER = 0x2,
    PDS_CORE_COMPONENT_STAMP_OR_VERSION_INVALID = 0x3,
    PDS_CORE_COMPONENT_CONFLICT = 0x4,
    PDS_CORE_COMPONENT_PREREQS_NOT_MET = 0x5,
    PDS_CORE_COMPONENT_NOT_SUPPORTED = 0x6,
    PDS_CORE_COMPONENT_FW_TYPE_INVALID = 0xd0,
}

//
// struct pds_core_send_component_tbl_comp - Send component table completion
// @status: Status of the command (enum pds_core_status_code)
// @ver: Device's max supported version of this command
// @completion_code: Component completion code
// @response: Component response
// @response_code: Component response code
// @slot_id: Actual slot_id of the component (enum pds_core_fw_slot)
// @rsvd: Word boundary padding
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_send_component_tbl_comp {
    pub status: u8,
    pub ver: u8,
    pub completion_code: u8,
    pub response: u8,
    pub response_code: u8,
    pub slot_id: u8,
    pub rsvd: [u8; 2],
}

//
// enum pds_core_send_component_op - PDS_CORE_CMD_SEND_COMPONENT operation
// @PDS_CORE_SEND_COMPONENT_START: Initial operation to start transfer
// @PDS_CORE_SEND_COMPONENT_STATUS: Subsequent calls to check on status
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pds_core_send_component_op {
    PDS_CORE_SEND_COMPONENT_START = 0,
    PDS_CORE_SEND_COMPONENT_STATUS = 1,
}

pub const PDS_CORE_FW_COMPONENT_ID_INVALID: c_uint = 0xFFFF;
//
// struct pds_core_flash_component - Component details
// @comparison_stamp: Comparison stamp used for component version checks
// @image_size: Component image size
// @classification: Vendor specific classification info
// @identifier: Component's ID
// @options: Component options
// @rsvd: Word boundary padding
// @version_str_type: The types of strings used
// @version_str_len: Length of @version_str
// @version_str: Component version information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_flash_component {
    pub comparison_stamp: __le32,
    pub image_size: __le32,
    pub classification: __le16,
    pub identifier: __le16,
    pub options: __le16,
    pub rsvd: [u8; 3],
    pub version_str_type: u8,
    pub version_str_len: u8,
    pub version_str: [u8; ],
}

//
// struct pds_core_send_component_cmd - Send component command
// @opcode: Opcode PDS_CORE_CMD_SEND_COMPONENT
// @ver: Driver's max supported version of this command
// @slot_id: enum pds_core_fw_slot
// @operation: enum pds_core_send_component_op
// @offset: Offset into the component, non-zero if multiple commands
// are needed for a single component
// @data_len: Length of this part of the component stored at @data_pa
// @rsvd: Word boundary padding
// @data_pa: DMA address of the component
//
// A component may be too large to store in a single buffer, so multiple
// PDS_CORE_CMD_SEND_COMPONENT devcmds may be needed.
//
// Expects to find flash component info (struct pds_core_flash_component)
// in cmd_regs->data. Driver should keep the devcmd interface locked
// while preparing and sending the flash component info.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_send_component_cmd {
    pub opcode: u8,
    pub ver: u8,
    pub slot_id: u8,
    pub operation: u8,
    pub offset: __le32,
    pub data_len: __le32,
    pub rsvd: [u8; 4],
    pub data_pa: __le64,
}

//
// struct pds_core_send_component_comp - Send component completion
// @status: Status of the command (enum pds_core_status_code)
// @ver: Device's max supported version of this command
// @completion_code: Completion code
// @compat_response: Compatibility response (0 = Component can be updated)
// @compat_response_code: Compatibility response code
// @rsvd: Word boundary padding
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_send_component_comp {
    pub status: u8,
    pub ver: u8,
    pub completion_code: u8,
    pub compat_response: u8,
    pub compat_response_code: u8,
    pub rsvd: [u8; 3],
}

//
// enum pds_core_fw_component_type - Firmware component type
// @PDS_CORE_FW_TYPE_UNKNOWN: Unknown component type
// @PDS_CORE_FW_TYPE_MAIN: Main firmware
// @PDS_CORE_FW_TYPE_BOOT: Boot loader
// @PDS_CORE_FW_TYPE_CPLD: CPLD firmware
// @PDS_CORE_FW_TYPE_SECURE: Secure firmware
// @PDS_CORE_FW_TYPE_FPGA: FPGA configuration
// @PDS_CORE_FW_TYPE_SUC_MAIN: System Unit Controller firmware
// @PDS_CORE_FW_TYPE_SUC_BOOT: System Unit Controller bootloader
// @PDS_CORE_FW_TYPE_UBOOT: U-Boot bootloader
//
// Gold/recovery variants are identified by slot_id == PDS_CORE_FW_SLOT_GOLD
// and reported with a ".gold" suffix (e.g., fw.gold).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pds_core_fw_component_type {
    PDS_CORE_FW_TYPE_UNKNOWN   = 0,
    PDS_CORE_FW_TYPE_MAIN      = 1,
    PDS_CORE_FW_TYPE_BOOT      = 2,
    PDS_CORE_FW_TYPE_CPLD      = 3,
    PDS_CORE_FW_TYPE_SECURE    = 4,
    PDS_CORE_FW_TYPE_FPGA      = 5,
    PDS_CORE_FW_TYPE_SUC_MAIN  = 6,
    PDS_CORE_FW_TYPE_SUC_BOOT  = 7,
    PDS_CORE_FW_TYPE_UBOOT     = 8,
}

//
// enum pds_core_component_info_flags - Component info flags
// @PDS_CORE_FW_COMPONENT_INFO_F_RUNNING: Component is currently running
// @PDS_CORE_FW_COMPONENT_INFO_F_STARTUP: Component version on next FW boot
// @PDS_CORE_FW_COMPONENT_INFO_F_FIXED: Component is fixed and cannot be updated
// @PDS_CORE_FW_COMPONENT_INFO_F_UPDATE_BY_NAME: Component can be updated
// by name
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pds_core_component_info_flags {
    PDS_CORE_FW_COMPONENT_INFO_F_RUNNING = BIT(0),
    PDS_CORE_FW_COMPONENT_INFO_F_STARTUP = BIT(1),
    PDS_CORE_FW_COMPONENT_INFO_F_FIXED = BIT(2),
    PDS_CORE_FW_COMPONENT_INFO_F_UPDATE_BY_NAME = BIT(3),
}

//
// struct pds_core_fw_component_info - GET_COMPONENT_INFO entry
// @name: Component's name
// @component_type: enum pds_core_fw_component_type
// @rsvd: Word boundary padding
// @flags: enum pds_core_component_info_flags
// @identifier: Component's identifier
// @slot_id: Component's slot identifier
// @version: Component's version
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_fw_component_info {
pub const PDS_CORE_FW_COMPONENT_NAME_BUFLEN: c_int = 24;
    pub name: [c_char; PDS_CORE_FW_COMPONENT_NAME_BUFLEN],
    pub component_type: u8,
    pub rsvd: [u8; 3],
    pub flags: __le16,
    pub identifier: u8,
    pub slot_id: u8,
pub const PDS_CORE_FW_COMPONENT_VER_BUFLEN: c_int = 32;
    pub version: [c_char; PDS_CORE_FW_COMPONENT_VER_BUFLEN],
}

//
// struct pds_core_component_list_info - GET_COMPONENT_INFO completion data
// @num_components: Number of valid components
// @rsvd: Word boundary padding
// @info: List of valid components
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_component_list_info {
    pub num_components: u8,
    pub rsvd: [u8; 7],
    pub info: [pds_core_fw_component_info; PDS_CORE_FW_COMPONENT_LIST_LEN],
}

//
// struct pds_core_get_component_info_cmd - GET_COMPONENT_INFO command
// @opcode: PDS_CORE_CMD_GET_COMPONENT_INFO
// @ver: Driver's max supported version of this command
// @data_len: Length of data at data_pa
// @rsvd: Word boundary padding
// @data_pa: DMA address of data
//
// FW populates struct pds_core_component_list_info pointed to by @data_pa
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_get_component_info_cmd {
    pub opcode: u8,
    pub ver: u8,
    pub data_len: __le16,
    pub rsvd: [u8; 4],
    pub data_pa: __le64,
}

//
// struct pds_core_get_component_info_comp - GET_COMPONENT_INFO completion
// @status: enum pds_core_status_code
// @ver: Device's max supported version of this command
// @rsvd: Word boundary padding
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_get_component_info_comp {
    pub status: u8,
    pub ver: u8,
    pub rsvd: [u8; 2],
}

//
// struct pds_core_finalize_update_cmd - FINALIZE_UPDATE command
// @opcode: PDS_CORE_CMD_FINALIZE_UPDATE
// @ver: Driver's max support version of this command
// @rsvd: Word boundary padding
//
// Driver sends at the end of updating all components to finalize the update
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_finalize_update_cmd {
    pub opcode: u8,
    pub ver: u8,
    pub rsvd: [u8; 2],
}

//
// struct pds_core_finalize_update_comp - FINALIZE_UPDATE completion
// @status: enum pds_core_status_code
// @ver: Device's max supported version of this command
// @rsvd: Word boundary padding
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_finalize_update_comp {
    pub status: u8,
    pub ver: u8,
    pub rsvd: [u8; 2],
}

//
// struct pds_core_match_record_desc_cmd - MATCH_RECORD_DESC command
// @opcode: PDS_CORE_CMD_MATCH_RECORD_DESC
// @ver: Driver's max supported version of this command
// @type: PLDM Descriptor Identifier Type
// @size: Length of the Descriptor Identifier Value
// @rsvd: Word boundary padding
//
// Expects to find the Descriptor Identifier Data in cmd_regs->data. Driver
// should keep the devcmd interface locked while preparing and sending this
// command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_match_record_desc_cmd {
    pub opcode: u8,
    pub ver: u8,
    pub type: __le16,
    pub size: __le16,
    pub rsvd: [u8; 2],
}

//
// struct pds_core_match_record_desc_comp - MATCH_RECORD_DESC completion
// @status: enum pds_core_status_code
// @ver: Device's max supported version of this command
// @match: Whether or not the Record Descriptor matches the device
// @rsvd: Word boundary padding
//
// When status is PDS_RC_SUCCESS, then @match is valid, otherwise it's
// undefined.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_match_record_desc_comp {
    pub status: u8,
    pub ver: u8,
    pub match: u8,
    pub rsvd: u8,
}

//
// enum pds_core_host_mem_oper - HOST_MEM sub-operations
// @PDS_CORE_HOST_MEM_GET_COUNT: Query number of memory requests
// @PDS_CORE_HOST_MEM_QUERY:     Query details of a memory request
// @PDS_CORE_HOST_MEM_ADD:       Provide allocated memory to firmware
// @PDS_CORE_HOST_MEM_DEL:       Notify firmware of memory deallocation
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pds_core_host_mem_oper {
    PDS_CORE_HOST_MEM_GET_COUNT	= 0,
    PDS_CORE_HOST_MEM_QUERY		= 1,
    PDS_CORE_HOST_MEM_ADD		= 2,
    PDS_CORE_HOST_MEM_DEL		= 3,
}

//
// struct pds_core_host_mem_cmd - HOST_MEM command
// @opcode:     Opcode PDS_CORE_CMD_HOST_MEM
// @oper:       Operation (enum pds_core_host_mem_oper)
// @index:      Memory request index (GET_COUNT: max_count, QUERY: index)
// @tag:        Tag for this memory request (ADD/DEL)
// @reason:     Reason for deletion (DEL only)
// @rsvd:       Reserved
// @max_contig: Maximum contiguous memory size (GET_COUNT only)
// @size:       Size of memory in bytes (ADD only)
// @buf_pa:     DMA address of memory (ADD only)
//
// Unified command for all host memory operations. Fields are reused
// across operations to minimize opcode space usage.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_host_mem_cmd {
    pub opcode: u8,
    pub oper: u8,
    pub index: __le16,
    pub tag: __le16,
    pub reason: u8,
    pub rsvd: u8,
    pub max_contig: __le32,
    pub size: __le32,
    pub buf_pa: __le64,
}

//
// struct pds_core_host_mem_comp - HOST_MEM completion
// @status:       Status of the command (enum pds_core_status_code)
// @oper:         Operation that was performed
// @count:        Number of memory requests (GET_COUNT)
// @size:         Size of memory request in bytes (QUERY)
// @tag:          Tag for this memory request (QUERY/DEL)
// @rsvd:         Reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_host_mem_comp {
    pub status: u8,
    pub oper: u8,
    pub count: __le16,
    pub size: __le32,
    pub tag: __le16,
    pub rsvd: [u8; 6],
}

//
// union pds_core_dev_cmd - Overlay of core device command structures
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union pds_core_dev_cmd {
    pub opcode: u8,
    pub words: [u32; 16],
    pub identify: pds_core_dev_identify_cmd,
    pub init: pds_core_dev_init_cmd,
    pub reset: pds_core_dev_reset_cmd,
    pub fw_download: pds_core_fw_download_cmd,
    pub fw_control: pds_core_fw_control_cmd,
    pub vf_setattr: pds_core_vf_setattr_cmd,
    pub vf_getattr: pds_core_vf_getattr_cmd,
    pub vf_ctrl: pds_core_vf_ctrl_cmd,
    pub get_component_info: pds_core_get_component_info_cmd,
    pub send_pkg_data: pds_core_send_pkg_data_cmd,
    pub send_component_tbl: pds_core_send_component_tbl_cmd,
    pub send_component: pds_core_send_component_cmd,
    pub finalize_update: pds_core_finalize_update_cmd,
    pub match_record_desc: pds_core_match_record_desc_cmd,
    pub host_mem: pds_core_host_mem_cmd,
}

//
// union pds_core_dev_comp - Overlay of core device completion structures
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union pds_core_dev_comp {
    pub status: u8,
    pub bytes: [u8; 16],
    pub identify: pds_core_dev_identify_comp,
    pub reset: pds_core_dev_reset_comp,
    pub init: pds_core_dev_init_comp,
    pub fw_download: pds_core_fw_download_comp,
    pub fw_control: pds_core_fw_control_comp,
    pub vf_setattr: pds_core_vf_setattr_comp,
    pub vf_getattr: pds_core_vf_getattr_comp,
    pub vf_ctrl: pds_core_vf_ctrl_comp,
    pub get_component_info: pds_core_get_component_info_comp,
    pub send_pkg_data: pds_core_send_pkg_data_comp,
    pub send_component_tbl: pds_core_send_component_tbl_comp,
    pub send_component: pds_core_send_component_comp,
    pub finalize_update: pds_core_finalize_update_comp,
    pub match_record_desc: pds_core_match_record_desc_comp,
    pub host_mem: pds_core_host_mem_comp,
}

//
// struct pds_core_dev_hwstamp_regs - Hardware current timestamp registers
// @tick_low:        Low 32 bits of hardware timestamp
// @tick_high:       High 32 bits of hardware timestamp
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_dev_hwstamp_regs {
    pub tick_low: u32,
    pub tick_high: u32,
}

//
// struct pds_core_dev_info_regs - Device info register format (read-only)
// @signature:       Signature value of 0x44455649 ('DEVI')
// @version:         Current version of info
// @asic_type:       Asic type
// @asic_rev:        Asic revision
// @fw_status:       Firmware status
// bit 0   - 1 = fw running
// bit 4-7 - 4 bit generation number, changes on fw restart
// @fw_heartbeat:    Firmware heartbeat counter
// @serial_num:      Serial number
// @fw_version:      Firmware version
// @oprom_regs:      oprom_regs to store oprom debug enable/disable and bmp
// @rsvd_pad1024:    Struct padding
// @hwstamp:         Hardware current timestamp registers
// @rsvd_pad2048:    Struct padding
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_dev_info_regs {
pub const PDS_CORE_DEVINFO_FWVERS_BUFLEN: c_int = 32;
pub const PDS_CORE_DEVINFO_SERIAL_BUFLEN: c_int = 32;
    pub signature: u32,
    pub version: u8,
    pub asic_type: u8,
    pub asic_rev: u8,
pub const PDS_CORE_FW_STS_F_STOPPED: c_uint = 0x00;
pub const PDS_CORE_FW_STS_F_RUNNING: c_uint = 0x01;
pub const PDS_CORE_FW_STS_F_GENERATION: c_uint = 0xF0;
    pub fw_status: u8,
    pub fw_heartbeat: __le32,
    pub fw_version: [c_char; PDS_CORE_DEVINFO_FWVERS_BUFLEN],
    pub serial_num: [c_char; PDS_CORE_DEVINFO_SERIAL_BUFLEN],
    pub /: *mut *mut u8 oprom_regs[32]; / reserved,
    pub rsvd_pad1024: [u8; 916],
    pub /: *mut *mut pds_core_dev_hwstamp_regs hwstamp; / on 1k boundary,
    pub rsvd_pad2048: [u8; 1016],
    pub __packed: },
//
// struct pds_core_dev_cmd_regs - Device command register format (read-write)
// @doorbell:	Device Cmd Doorbell, write-only
// Write a 1 to signal device to process cmd
// @done:	Command completed indicator, poll for completion
// bit 0 == 1 when command is complete
// @cmd:	Opcode-specific command bytes
// @comp:	Opcode-specific response bytes
// @rsvd:	Struct padding
// @data:	Opcode-specific side-data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_dev_cmd_regs {
    pub doorbell: u32,
    pub done: u32,
    pub cmd: pds_core_dev_cmd,
    pub comp: pds_core_dev_comp,
    pub rsvd: [u8; 48],
    pub data: [u32; 478],
    pub __packed: },
//
// struct pds_core_dev_regs - Device register format for bar 0 page 0
// @info:            Device info registers
// @devcmd:          Device command registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_dev_regs {
    pub info: pds_core_dev_info_regs,
    pub devcmd: pds_core_dev_cmd_regs,
    pub __packed: },
    pub 1912): static_assert(sizeof(struct pds_core_drv_identity) <=,
    pub 1912): static_assert(sizeof(struct pds_core_dev_identity) <=,
    pub 64): static_assert(sizeof(union pds_core_dev_cmd) ==,
    pub 16): static_assert(sizeof(union pds_core_dev_comp) ==,
    pub 2048): static_assert(sizeof(struct pds_core_dev_info_regs) ==,
    pub 2048): static_assert(sizeof(struct pds_core_dev_cmd_regs) ==,
    pub 4096): static_assert(sizeof(struct pds_core_dev_regs) ==,

