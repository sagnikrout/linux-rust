//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cisco/enic/vnic_devcmd.h
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
// Copyright 2008-2010 Cisco Systems, Inc.  All rights reserved.
// Copyright 2007 Nuova Systems, Inc.  All rights reserved.
//
pub const _CMD_NBITS: c_int = 14;
pub const _CMD_VTYPEBITS: c_int = 10;
pub const _CMD_FLAGSBITS: c_int = 6;
pub const _CMD_DIRBITS: c_int = 2;

pub const _CMD_NSHIFT: c_int = 0;

//
// Direction bits (from host perspective).
//

//
// Flag bits.
//

//
// vNIC type bits.
//

//
// Used to create cmds..
//

//
// Used to decode cmds..
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vnic_devcmd_cmd {
    CMD_NONE                = _CMDC(_CMD_DIR_NONE, _CMD_VTYPE_NONE, 0),

//
// mcpu fw info in mem:
// in:
// (u64)a0=paddr to struct vnic_devcmd_fw_info
// action:
// Fills in struct vnic_devcmd_fw_info (128 bytes)
// note:
// An old definition of CMD_MCPU_FW_INFO
//
    CMD_MCPU_FW_INFO_OLD    = _CMDC(_CMD_DIR_WRITE, _CMD_VTYPE_ALL, 1),

//
// mcpu fw info in mem:
// in:
// (u64)a0=paddr to struct vnic_devcmd_fw_info
// (u16)a1=size of the structure
// out:
// (u16)a1=0                          for in:a1 = 0,
// data size actually written for other values.
// action:
// Fills in first 128 bytes of vnic_devcmd_fw_info for in:a1 = 0,
// first in:a1 bytes               for 0 < in:a1 <= 132,
// 132 bytes                       for other values of in:a1.
// note:
// CMD_MCPU_FW_INFO and CMD_MCPU_FW_INFO_OLD have the same enum 1
// for source compatibility.
//
    CMD_MCPU_FW_INFO        = _CMDC(_CMD_DIR_RW, _CMD_VTYPE_ALL, 1),

// dev-specific block member:
// in: (u16)a0=offset,(u8)a1=size
// out: a0=value
    CMD_DEV_SPEC            = _CMDC(_CMD_DIR_RW, _CMD_VTYPE_ALL, 2),

// stats clear
    CMD_STATS_CLEAR         = _CMDCNW(_CMD_DIR_NONE, _CMD_VTYPE_ALL, 3),

// stats dump in mem: (u64)a0=paddr to stats area,
// (u16)a1=sizeof stats area
    CMD_STATS_DUMP          = _CMDC(_CMD_DIR_WRITE, _CMD_VTYPE_ALL, 4),

// set Rx packet filter: (u32)a0=filters (see CMD_PFILTER_*)
    CMD_PACKET_FILTER	= _CMDCNW(_CMD_DIR_WRITE, _CMD_VTYPE_ENET, 7),

// set Rx packet filter for all: (u32)a0=filters (see CMD_PFILTER_*)
    CMD_PACKET_FILTER_ALL   = _CMDCNW(_CMD_DIR_WRITE, _CMD_VTYPE_ALL, 7),

// hang detection notification
    CMD_HANG_NOTIFY         = _CMDC(_CMD_DIR_NONE, _CMD_VTYPE_ALL, 8),

// MAC address in (u48)a0
    CMD_GET_MAC_ADDR        = _CMDC(_CMD_DIR_READ,
    _CMD_VTYPE_ENET | _CMD_VTYPE_FC, 9),

// add addr from (u48)a0
    CMD_ADDR_ADD            = _CMDCNW(_CMD_DIR_WRITE,
    _CMD_VTYPE_ENET | _CMD_VTYPE_FC, 12),

// del addr from (u48)a0
    CMD_ADDR_DEL            = _CMDCNW(_CMD_DIR_WRITE,
    _CMD_VTYPE_ENET | _CMD_VTYPE_FC, 13),

// add VLAN id in (u16)a0
    CMD_VLAN_ADD            = _CMDCNW(_CMD_DIR_WRITE, _CMD_VTYPE_ENET, 14),

// del VLAN id in (u16)a0
    CMD_VLAN_DEL            = _CMDCNW(_CMD_DIR_WRITE, _CMD_VTYPE_ENET, 15),

// nic_cfg (no wait, always succeeds)
// in: (u32)a0
//
// Capability query:
// out: (u64) a0 = 1 if a1 is valid
// (u64) a1 = (NIC_CFG bits supported) | (flags << 32)
//
// flags are CMD_NIC_CFG_CAPF_xxx
//
    CMD_NIC_CFG             = _CMDCNW(_CMD_DIR_WRITE, _CMD_VTYPE_ALL, 16),
// nic_cfg_chk (will return error if flags are invalid)
// in: (u32)a0
//
// Capability query:
// out: (u64) a0 = 1 if a1 is valid
// (u64) a1 = (NIC_CFG bits supported) | (flags << 32)
//
// flags are CMD_NIC_CFG_CAPF_xxx
//
    CMD_NIC_CFG_CHK		= _CMDC(_CMD_DIR_WRITE, _CMD_VTYPE_ALL, 16),

// union vnic_rss_key in mem: (u64)a0=paddr, (u16)a1=len
    CMD_RSS_KEY             = _CMDC(_CMD_DIR_WRITE, _CMD_VTYPE_ENET, 17),

// union vnic_rss_cpu in mem: (u64)a0=paddr, (u16)a1=len
    CMD_RSS_CPU             = _CMDC(_CMD_DIR_WRITE, _CMD_VTYPE_ENET, 18),

// initiate softreset
    CMD_SOFT_RESET          = _CMDCNW(_CMD_DIR_NONE, _CMD_VTYPE_ALL, 19),

// softreset status:
// out: a0=0 reset complete, a0=1 reset in progress
    CMD_SOFT_RESET_STATUS   = _CMDC(_CMD_DIR_READ, _CMD_VTYPE_ALL, 20),

// set struct vnic_devcmd_notify buffer in mem:
// in:
// (u64)a0=paddr to notify (set paddr=0 to unset)
// (u32)a1 & 0x00000000ffffffff=sizeof(struct vnic_devcmd_notify)
// (u16)a1 & 0x0000ffff00000000=intr num (-1 for no intr)
// out:
// (u32)a1 = effective size
//
    CMD_NOTIFY              = _CMDC(_CMD_DIR_RW, _CMD_VTYPE_ALL, 21),

// UNDI API: (u64)a0=paddr to s_PXENV_UNDI_ struct,
// (u8)a1=PXENV_UNDI_xxx
    CMD_UNDI                = _CMDC(_CMD_DIR_WRITE, _CMD_VTYPE_ENET, 22),

// initiate open sequence (u32)a0=flags (see CMD_OPENF_*)
    CMD_OPEN		= _CMDCNW(_CMD_DIR_WRITE, _CMD_VTYPE_ALL, 23),

// open status:
// out: a0=0 open complete, a0=1 open in progress
    CMD_OPEN_STATUS		= _CMDC(_CMD_DIR_READ, _CMD_VTYPE_ALL, 24),

// close vnic
    CMD_CLOSE		= _CMDC(_CMD_DIR_NONE, _CMD_VTYPE_ALL, 25),

// initialize virtual link: (u32)a0=flags (see CMD_INITF_*)
// Replaced by CMD_INIT
    CMD_INIT_v1		= _CMDCNW(_CMD_DIR_READ, _CMD_VTYPE_ALL, 26),

// variant of CMD_INIT, with provisioning info
// (u64)a0=paddr of vnic_devcmd_provinfo
// (u32)a1=sizeof provision info
    CMD_INIT_PROV_INFO	= _CMDC(_CMD_DIR_WRITE, _CMD_VTYPE_ENET, 27),

// enable virtual link
    CMD_ENABLE		= _CMDCNW(_CMD_DIR_WRITE, _CMD_VTYPE_ALL, 28),

// enable virtual link, waiting variant.
    CMD_ENABLE_WAIT		= _CMDC(_CMD_DIR_WRITE, _CMD_VTYPE_ALL, 28),

// disable virtual link
    CMD_DISABLE		= _CMDC(_CMD_DIR_NONE, _CMD_VTYPE_ALL, 29),

// stats dump sum of all vnic stats on same uplink in mem:
// (u64)a0=paddr
// (u16)a1=sizeof stats area
    CMD_STATS_DUMP_ALL	= _CMDC(_CMD_DIR_WRITE, _CMD_VTYPE_ALL, 30),

// init status:
// out: a0=0 init complete, a0=1 init in progress
// if a0=0, a1=errno
    CMD_INIT_STATUS		= _CMDC(_CMD_DIR_READ, _CMD_VTYPE_ALL, 31),

// INT13 API: (u64)a0=paddr to vnic_int13_params struct
// (u32)a1=INT13_CMD_xxx
    CMD_INT13               = _CMDC(_CMD_DIR_WRITE, _CMD_VTYPE_FC, 32),

// logical uplink enable/disable: (u64)a0: 0/1=disable/enable
    CMD_LOGICAL_UPLINK      = _CMDCNW(_CMD_DIR_WRITE, _CMD_VTYPE_ENET, 33),

// undo initialize of virtual link
    CMD_DEINIT		= _CMDCNW(_CMD_DIR_NONE, _CMD_VTYPE_ALL, 34),

// initialize virtual link: (u32)a0=flags (see CMD_INITF_*)
    CMD_INIT		= _CMDCNW(_CMD_DIR_WRITE, _CMD_VTYPE_ALL, 35),

// check fw capability of a cmd:
// in:  (u32)a0=cmd
// out: (u32)a0=errno, 0:valid cmd, a1=supported VNIC_STF_* bits
    CMD_CAPABILITY		= _CMDC(_CMD_DIR_RW, _CMD_VTYPE_ALL, 36),

// persistent binding info
// in:  (u64)a0=paddr of arg
// (u32)a1=CMD_PERBI_XXX
    CMD_PERBI		= _CMDC(_CMD_DIR_RW, _CMD_VTYPE_FC, 37),

// Interrupt Assert Register functionality
// in: (u16)a0=interrupt number to assert
//
    CMD_IAR			= _CMDCNW(_CMD_DIR_WRITE, _CMD_VTYPE_ALL, 38),

// initiate hangreset, like softreset after hang detected
    CMD_HANG_RESET		= _CMDC(_CMD_DIR_NONE, _CMD_VTYPE_ALL, 39),

// hangreset status:
// out: a0=0 reset complete, a0=1 reset in progress
    CMD_HANG_RESET_STATUS   = _CMDC(_CMD_DIR_READ, _CMD_VTYPE_ALL, 40),

//
// Set hw ingress packet vlan rewrite mode:
// in:  (u32)a0=new vlan rewrite mode
// out: (u32)a0=old vlan rewrite mode
    CMD_IG_VLAN_REWRITE_MODE = _CMDC(_CMD_DIR_RW, _CMD_VTYPE_ENET, 41),

//
// in:  (u16)a0=bdf of target vnic
// (u32)a1=cmd to proxy
// a2-a15=args to cmd in a1
// out: (u32)a0=status of proxied cmd
// a1-a15=out args of proxied cmd
    CMD_PROXY_BY_BDF =	_CMDC(_CMD_DIR_RW, _CMD_VTYPE_ALL, 42),

//
// As for BY_BDF except a0 is index of hvnlink subordinate vnic
// or SR-IOV virtual vnic
//
    CMD_PROXY_BY_INDEX =    _CMDC(_CMD_DIR_RW, _CMD_VTYPE_ALL, 43),

//
// For HPP toggle:
// adapter-info-get
// in:  (u64)a0=phsical address of buffer passed in from caller.
// (u16)a1=size of buffer specified in a0.
// out: (u64)a0=phsical address of buffer passed in from caller.
// (u16)a1=actual bytes from VIF-CONFIG-INFO TLV, or
// 0 if no VIF-CONFIG-INFO TLV was ever received.
    CMD_CONFIG_INFO_GET     = _CMDC(_CMD_DIR_RW, _CMD_VTYPE_ALL, 44),

// INT13 API: (u64)a0=paddr to vnic_int13_params struct
// (u32)a1=INT13_CMD_xxx
//
    CMD_INT13_ALL = _CMDC(_CMD_DIR_WRITE, _CMD_VTYPE_ALL, 45),

// Set default vlan:
// in: (u16)a0=new default vlan
// (u16)a1=zero for overriding vlan with param a0,
// non-zero for resetting vlan to the default
// out: (u16)a0=old default vlan
//
    CMD_SET_DEFAULT_VLAN = _CMDC(_CMD_DIR_RW, _CMD_VTYPE_ALL, 46),

// init_prov_info2:
// Variant of CMD_INIT_PROV_INFO, where it will not try to enable
// the vnic until CMD_ENABLE2 is issued.
// (u64)a0=paddr of vnic_devcmd_provinfo
// (u32)a1=sizeof provision info
//
    CMD_INIT_PROV_INFO2  = _CMDC(_CMD_DIR_WRITE, _CMD_VTYPE_ENET, 47),

// enable2:
// (u32)a0=0                  ==> standby
// =CMD_ENABLE2_ACTIVE ==> active
//
    CMD_ENABLE2 = _CMDC(_CMD_DIR_WRITE, _CMD_VTYPE_ENET, 48),

//
// cmd_status:
// Returns the status of the specified command
// Input:
// a0 = command for which status is being queried.
// Possible values are:
// CMD_SOFT_RESET
// CMD_HANG_RESET
// CMD_OPEN
// CMD_INIT
// CMD_INIT_PROV_INFO
// CMD_DEINIT
// CMD_INIT_PROV_INFO2
// CMD_ENABLE2
// Output:
// if status == STAT_ERROR
// a0 = ERR_ENOTSUPPORTED - status for command in a0 is
// not supported
// if status == STAT_NONE
// a0 = status of the devcmd specified in a0 as follows.
// ERR_SUCCESS   - command in a0 completed successfully
// ERR_EINPROGRESS - command in a0 is still in progress
//
    CMD_STATUS = _CMDC(_CMD_DIR_RW, _CMD_VTYPE_ALL, 49),

//
// Returns interrupt coalescing timer conversion factors.
// After calling this devcmd, ENIC driver can convert
// interrupt coalescing timer in usec into CPU cycles as follows:
//
// intr_timer_cycles = intr_timer_usec * multiplier / divisor
//
// Interrupt coalescing timer in usecs can be obtained from
// CPU cycles as follows:
//
// intr_timer_usec = intr_timer_cycles * divisor / multiplier
//
// in: none
// out: (u32)a0 = multiplier
// (u32)a1 = divisor
// (u32)a2 = maximum timer value in usec
//
    CMD_INTR_COAL_CONVERT = _CMDC(_CMD_DIR_READ, _CMD_VTYPE_ALL, 50),

//
// Set the predefined mac address as default
// in:
// (u48)a0 = mac addr
//
    CMD_SET_MAC_ADDR = _CMDC(_CMD_DIR_WRITE, _CMD_VTYPE_ENET, 55),

// Update the provisioning info of the given VIF
// (u64)a0=paddr of vnic_devcmd_provinfo
// (u32)a1=sizeof provision info
//
    CMD_PROV_INFO_UPDATE = _CMDC(_CMD_DIR_WRITE, _CMD_VTYPE_ENET, 56),

// Initialization for the devcmd2 interface.
// in: (u64) a0 = host result buffer physical address
// in: (u16) a1 = number of entries in result buffer
//
    CMD_INITIALIZE_DEVCMD2 = _CMDC(_CMD_DIR_WRITE, _CMD_VTYPE_ALL, 57),

// Add a filter.
// in: (u64) a0= filter address
// (u32) a1= size of filter
// out: (u32) a0=filter identifier
//
    CMD_ADD_FILTER = _CMDC(_CMD_DIR_RW, _CMD_VTYPE_ENET, 58),

// Delete a filter.
// in: (u32) a0=filter identifier
//
    CMD_DEL_FILTER = _CMDC(_CMD_DIR_WRITE, _CMD_VTYPE_ENET, 59),

// Enable a Queue Pair in User space NIC
// in: (u32) a0=Queue Pair number
// (u32) a1= command
//
    CMD_QP_ENABLE = _CMDC(_CMD_DIR_WRITE, _CMD_VTYPE_ENET, 60),

// Disable a Queue Pair in User space NIC
// in: (u32) a0=Queue Pair number
// (u32) a1= command
//
    CMD_QP_DISABLE = _CMDC(_CMD_DIR_WRITE, _CMD_VTYPE_ENET, 61),

// Stats dump Queue Pair in User space NIC
// in: (u32) a0=Queue Pair number
// (u64) a1=host buffer addr for status dump
// (u32) a2=length of the buffer
//
    CMD_QP_STATS_DUMP = _CMDC(_CMD_DIR_WRITE, _CMD_VTYPE_ENET, 62),

// Clear stats for Queue Pair in User space NIC
// in: (u32) a0=Queue Pair number
//
    CMD_QP_STATS_CLEAR = _CMDC(_CMD_DIR_WRITE, _CMD_VTYPE_ENET, 63),

// Use this devcmd for agreeing on the highest common version supported
// by both driver and fw for features who need such a facility.
// in:	(u64) a0 = feature (driver requests for the supported versions
// on this feature)
// out: (u64) a0 = bitmap of all supported versions for that feature
//
    CMD_GET_SUPP_FEATURE_VER = _CMDC(_CMD_DIR_RW, _CMD_VTYPE_ENET, 69),

// Control (Enable/Disable) overlay offloads on the given vnic
// in: (u8) a0 = OVERLAY_FEATURE_NVGRE : NVGRE
// a0 = OVERLAY_FEATURE_VXLAN : VxLAN
// in: (u8) a1 = OVERLAY_OFFLOAD_ENABLE : Enable or
// a1 = OVERLAY_OFFLOAD_DISABLE : Disable or
// a1 = OVERLAY_OFFLOAD_ENABLE_V2 : Enable with version 2
//
    CMD_OVERLAY_OFFLOAD_CTRL = _CMDC(_CMD_DIR_WRITE, _CMD_VTYPE_ENET, 72),

// Configuration of overlay offloads feature on a given vNIC
// in: (u8) a0 = DEVCMD_OVERLAY_NVGRE : NVGRE
// a0 = DEVCMD_OVERLAY_VXLAN : VxLAN
// in: (u8) a1 = VXLAN_PORT_UPDATE : VxLAN
// in: (u16) a2 = unsigned short int port information
//
    CMD_OVERLAY_OFFLOAD_CFG = _CMDC(_CMD_DIR_WRITE, _CMD_VTYPE_ENET, 73),

//
// Set extended CQ field in MREGS of RQ (or all RQs)
// for given vNIC
// in: (u64) a0 = RQ selection (VNIC_RQ_ALL for all RQs)
// (u32) a1 = CQ entry size
// VNIC_RQ_CQ_ENTRY_SIZE_16 --> 16 bytes
// VNIC_RQ_CQ_ENTRY_SIZE_32 --> 32 bytes
// VNIC_RQ_CQ_ENTRY_SIZE_64 --> 64 bytes
//
// Capability query:
// out: (u32) a0 = errno, 0:valid cmd
// (u32) a1 = value consisting of supported entries
// bit 0: 16 bytes
// bit 1: 32 bytes
// bit 2: 64 bytes
//
    CMD_CQ_ENTRY_SIZE_SET = _CMDC(_CMD_DIR_WRITE, _CMD_VTYPE_ENET, 90),

//
// Set queue pair type (admin or data)
// in: (u32) a0 = queue pair type (0 = admin, 1 = data)
// in: (u32) a1 = enable (1) / disable (0)
//
    CMD_QP_TYPE_SET = _CMDC(_CMD_DIR_WRITE, _CMD_VTYPE_ENET, 97),
}

pub const QP_TYPE_ADMIN: c_int = 0;
pub const QP_TYPE_DATA: c_int = 1;
pub const QP_ENABLE: c_int = 1;
pub const QP_DISABLE: c_int = 0;
// CMD_ENABLE2 flags
pub const CMD_ENABLE2_STANDBY: c_uint = 0x0;
pub const CMD_ENABLE2_ACTIVE: c_uint = 0x1;
// flags for CMD_OPEN
pub const CMD_OPENF_OPROM: c_uint = 0x1	/* open coming from option rom */;
pub const CMD_OPENF_IG_DESCCACHE: c_uint = 0x2	/* Do not flush IG DESC cache */;
// flags for CMD_INIT
pub const CMD_INITF_DEFAULT_MAC: c_uint = 0x1	/* init with default mac addr */;
// flags for CMD_PACKET_FILTER
pub const CMD_PFILTER_DIRECTED: c_uint = 0x01;
pub const CMD_PFILTER_MULTICAST: c_uint = 0x02;
pub const CMD_PFILTER_BROADCAST: c_uint = 0x04;
pub const CMD_PFILTER_PROMISCUOUS: c_uint = 0x08;
pub const CMD_PFILTER_ALL_MULTICAST: c_uint = 0x10;
// Commands for CMD_QP_ENABLE/CM_QP_DISABLE
pub const CMD_QP_RQWQ: c_uint = 0x0;
// rewrite modes for CMD_IG_VLAN_REWRITE_MODE
pub const IG_VLAN_REWRITE_MODE_DEFAULT_TRUNK: c_int = 0;
pub const IG_VLAN_REWRITE_MODE_UNTAG_DEFAULT_VLAN: c_int = 1;
pub const IG_VLAN_REWRITE_MODE_PRIORITY_TAG_DEFAULT_VLAN: c_int = 2;
pub const IG_VLAN_REWRITE_MODE_PASS_THRU: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vnic_devcmd_status {
    STAT_NONE = 0,
    STAT_BUSY = 1 << 0,	/* cmd in progress */
    STAT_ERROR = 1 << 1,	/* last cmd caused error (code in a0) */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vnic_devcmd_error {
    ERR_SUCCESS = 0,
    ERR_EINVAL = 1,
    ERR_EFAULT = 2,
    ERR_EPERM = 3,
    ERR_EBUSY = 4,
    ERR_ECMDUNKNOWN = 5,
    ERR_EBADSTATE = 6,
    ERR_ENOMEM = 7,
    ERR_ETIMEDOUT = 8,
    ERR_ELINKDOWN = 9,
    ERR_EMAXRES = 10,
    ERR_ENOTSUPPORTED = 11,
    ERR_EINPROGRESS = 12,
    ERR_MAX
}

//
// note: hw_version and asic_rev refer to the same thing,
// but have different formats. hw_version is
// a 32-byte string (e.g. "A2") and asic_rev is
// a 16-bit integer (e.g. 0xA2).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vnic_devcmd_fw_info {
    pub fw_version: [c_char; 32],
    pub fw_build: [c_char; 32],
    pub hw_version: [c_char; 32],
    pub hw_serial_number: [c_char; 32],
    pub asic_type: u16,
    pub asic_rev: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vnic_devcmd_notify {
    pub /: *mut *mut u32 csum; / checksum over following words,
    pub /: *mut *mut u32 link_state; / link up == 1,
    pub /: *mut *mut u32 port_speed; / effective port speed (rate limit),
    pub /: *mut *mut u32 mtu; / MTU,
    pub /: *mut *mut u32 msglvl; / requested driver msg lvl,
    pub /: *mut *mut u32 uif; / uplink interface,
    pub /: *mut *mut *mut u32 status; / status bits (see VNIC_STF_),
    pub /: *mut *mut *mut u32 error; / error code (see ERR_) for first ERR,
    pub /: *mut *mut u32 link_down_cnt; / running count of link down transitions,
    pub /: *mut *mut u32 perbi_rebuild_cnt; / running count of perbi rebuilds,
}

pub const VNIC_STF_FATAL_ERR: c_uint = 0x0001	/* fatal fw error */;
pub const VNIC_STF_STD_PAUSE: c_uint = 0x0002	/* standard link-level pause on */;
pub const VNIC_STF_PFC_PAUSE: c_uint = 0x0004	/* priority flow control pause on */;
// all supported status flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vnic_devcmd_provinfo {
    pub oui: [u8; 3],
    pub type: u8,
    pub data: [u8; ],
}

// These are used in flags field of different filters to denote
// valid fields used.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct filter_usnic_id {
    pub flags: u32,
    pub vlan: u16,
    pub ethtype: u16,
    pub proto_version: u8,
    pub usnic_id: u32,
    pub __packed: },

// Enums for the protocol field.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum protocol_e {
    PROTO_UDP = 0,
    PROTO_TCP = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct filter_ipv4_5tuple {
    pub flags: u32,
    pub protocol: u32,
    pub src_addr: u32,
    pub dst_addr: u32,
    pub src_port: u16,
    pub dst_port: u16,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct filter_mac_vlan {
    pub flags: u32,
    pub vlan: u16,
    pub mac_addr: [u8; 6],
    pub __packed: },
// Specifies the filter_action type.
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct filter_action {
    pub type: u32,
    pub rq_idx: u32,
    pub u: },
    pub __packed: },
// Specifies the filter type.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum filter_type {
    FILTER_USNIC_ID = 0,
    FILTER_IPV4_5TUPLE = 1,
    FILTER_MAC_VLAN = 2,
    FILTER_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct filter {
    pub type: u32,
    pub usnic: filter_usnic_id,
    pub ipv4: filter_ipv4_5tuple,
    pub mac_vlan: filter_mac_vlan,
    pub u: },
    pub __packed: },
}

// Maximum size of buffer to CMD_ADD_FILTER
pub const FILTER_MAX_BUF_SIZE: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct filter_tlv {
    pub type: u32,
    pub length: u32,
    pub val: [u32; ],
}

//
// Writing cmd register causes STAT_BUSY to get set in status register.
// When cmd completes, STAT_BUSY will be cleared.
//
// If cmd completed successfully STAT_ERROR will be clear
// and args registers contain cmd-specific results.
//
// If cmd error, STAT_ERROR will be set and args[0] contains error code.
//
// status register is read-only.  While STAT_BUSY is set,
// all other register contents are read-only.
//
// Make sizeof(vnic_devcmd) a power-of-2 for I/O BAR.
pub const VNIC_DEVCMD_NARGS: c_int = 15;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vnic_devcmd {
    pub /: *mut *mut u32 status; / RO,
    pub /: *mut *mut u32 cmd; / RW,
    pub /: *mut *mut u64 args[VNIC_DEVCMD_NARGS]; / RW cmd args (little-endian),
}

pub const DEVCMD2_FNORESULT: c_uint = 0x1	/* Don't copy result to host */;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vnic_devcmd2 {
    pub pad: u16,
    pub flags: u16,
    pub cmd: u32,
    pub args: [u64; VNIC_DEVCMD2_NARGS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct devcmd2_result {
    pub results: [u64; VNIC_DEVCMD2_NRESULTS],
    pub pad: u32,
    pub completed_index: u16,
    pub error: u8,
    pub color: u8,
}

pub const DEVCMD2_RING_SIZE: c_int = 32;
pub const DEVCMD2_DESC_SIZE: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum overlay_feature_t {
    OVERLAY_FEATURE_NVGRE = 1,
    OVERLAY_FEATURE_VXLAN,
    OVERLAY_FEATURE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum overlay_ofld_cmd {
    OVERLAY_OFFLOAD_ENABLE,
    OVERLAY_OFFLOAD_DISABLE,
    OVERLAY_OFFLOAD_ENABLE_P2,
    OVERLAY_OFFLOAD_MAX,
}

pub const OVERLAY_CFG_VXLAN_PORT_UPDATE: c_int = 0;

// Use this enum to get the supported versions for each of these features
// If you need to use the devcmd_get_supported_feature_version(), add
// the new feature into this enum and install function handler in devcmd.c
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vic_feature_t {
    VIC_FEATURE_VXLAN,
    VIC_FEATURE_RDMA,
    VIC_FEATURE_VXLAN_PATCH,
// slot 3 reserved for firmware VIC_FEATURE_PTP
    VIC_FEATURE_SRIOV	= 4,
    VIC_FEATURE_MAX,
}
