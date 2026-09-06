//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/bfa/bfi.h
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

// Per dma segment max size

// Get number of dma segments required

// Get num dma reqs - that fit in a segment

// Get segment num from tag

// Get dma req offset in a segment

//
// BFI FW image type
//

pub const BFI_FLASH_IMAGE_SZ: c_uint = 0x100000;
//
// Msg header common to all msgs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_mhdr_s {
    pub /: *mut *mut u8 msg_class; / @ref bfi_mclass_t,
    pub /: *mut *mut u8 msg_id; / msg opcode with in the class,
    pub qid: u8,
    pub /: *mut *mut u8 fn_lpu; / msg destination,
    pub h2i: },
    pub /: *mut *mut u16 i2htok; / token in msgs to host,
    pub mtag: },
}

//
// Message opcodes: 0-127 to firmware, 128-255 to host
//
pub const BFI_I2H_OPCODE_BASE: c_int = 128;

//
// Scatter Gather Element and Page definition
//
pub const BFI_SGE_INLINE: c_int = 1;

//
// SG Flags
//
// DMA addresses
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union bfi_addr_u {
    pub addr_lo: __be32,
    pub addr_hi: __be32,
    pub a32: },
}

//
// Scatter Gather Element used for fast-path IO requests
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_sge_s {

    pub sga: bfi_addr_u,
}

//
// Generic DMA addr-len pair.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_alen_s {
    pub /: *mut *mut bfi_addr_u al_addr; / DMA addr of buffer,
    pub /: *mut *mut u32 al_len; / length of buffer,
}

//
// Scatter Gather Page
//
pub const BFI_SGPG_DATA_SGES: c_int = 7;

pub const BFI_SGPG_RSVD_WD_LEN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_sgpg_s {
    pub sges: [bfi_sge_s; BFI_SGPG_SGES_MAX],
    pub rsvd: [u32; BFI_SGPG_RSVD_WD_LEN],
}

// FCP module definitions

//
// Large Message structure - 128 Bytes size Msgs
//
pub const BFI_LMSG_SZ: c_int = 128;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_msg_s {
    pub mhdr: bfi_mhdr_s,
    pub pl: [u32; BFI_LMSG_PL_WSZ],
}

//
// Mailbox message structure
//
pub const BFI_MBMSG_SZ: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_mbmsg_s {
    pub mh: bfi_mhdr_s,
    pub pl: [u32; BFI_MBMSG_SZ],
}

//
// Supported PCI function class codes (personality)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_pcifn_class {
    BFI_PCIFN_CLASS_FC  = 0x0c04,
    BFI_PCIFN_CLASS_ETH = 0x0200,
}

//
// Message Classes
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_mclass {
    BFI_MC_IOC		= 1,	/*  IO Controller (IOC)	    */
    BFI_MC_DIAG		= 2,    /*  Diagnostic Msgs            */
    BFI_MC_FLASH		= 3,	/*  Flash message class	*/
    BFI_MC_CEE		= 4,	/*  CEE	*/
    BFI_MC_FCPORT		= 5,	/*  FC port			    */
    BFI_MC_IOCFC		= 6,	/*  FC - IO Controller (IOC)	    */
    BFI_MC_ABLK		= 7,	/*  ASIC block configuration	    */
    BFI_MC_UF		= 8,	/*  Unsolicited frame receive	    */
    BFI_MC_FCXP		= 9,	/*  FC Transport		    */
    BFI_MC_LPS		= 10,	/*  lport fc login services	    */
    BFI_MC_RPORT		= 11,	/*  Remote port		    */
    BFI_MC_ITN		= 12,	/*  I-T nexus (Initiator mode)	    */
    BFI_MC_IOIM_READ	= 13,	/*  read IO (Initiator mode)	    */
    BFI_MC_IOIM_WRITE	= 14,	/*  write IO (Initiator mode)	    */
    BFI_MC_IOIM_IO		= 15,	/*  IO (Initiator mode)	    */
    BFI_MC_IOIM		= 16,	/*  IO (Initiator mode)	    */
    BFI_MC_IOIM_IOCOM	= 17,	/*  good IO completion		    */
    BFI_MC_TSKIM		= 18,	/*  Initiator Task management	    */
    BFI_MC_PORT		= 21,	/*  Physical port		    */
    BFI_MC_SFP		= 22,	/*  SFP module	*/
    BFI_MC_PHY		= 25,   /*  External PHY message class	*/
    BFI_MC_FRU		= 34,
    BFI_MC_MAX		= 35
}

pub const BFI_IOC_MAX_CQS: c_int = 4;
pub const BFI_IOC_MAX_CQS_ASIC: c_int = 8;

//
// ----------------------------------------------------------------------
// IOC
// ----------------------------------------------------------------------
//
// Different asic generations
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_asic_gen {
    BFI_ASIC_GEN_CB		= 1,	/* crossbow 8G FC		*/
    BFI_ASIC_GEN_CT		= 2,	/* catapult 8G FC or 10G CNA	*/
    BFI_ASIC_GEN_CT2	= 3,	/* catapult-2 16G FC or 10G CNA	*/
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_asic_mode {
    BFI_ASIC_MODE_FC	= 1,	/* FC upto 8G speed		*/
    BFI_ASIC_MODE_FC16	= 2,	/* FC upto 16G speed		*/
    BFI_ASIC_MODE_ETH	= 3,	/* Ethernet ports		*/
    BFI_ASIC_MODE_COMBO	= 4,	/* FC 16G and Ethernet 10G port	*/
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_ioc_h2i_msgs {
    BFI_IOC_H2I_ENABLE_REQ		= 1,
    BFI_IOC_H2I_DISABLE_REQ		= 2,
    BFI_IOC_H2I_GETATTR_REQ		= 3,
    BFI_IOC_H2I_DBG_SYNC		= 4,
    BFI_IOC_H2I_DBG_DUMP		= 5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_ioc_i2h_msgs {
    BFI_IOC_I2H_ENABLE_REPLY	= BFA_I2HM(1),
    BFI_IOC_I2H_DISABLE_REPLY	= BFA_I2HM(2),
    BFI_IOC_I2H_GETATTR_REPLY	= BFA_I2HM(3),
    BFI_IOC_I2H_HBEAT		= BFA_I2HM(4),
    BFI_IOC_I2H_ACQ_ADDR_REPLY	= BFA_I2HM(5),
}

//
// BFI_IOC_H2I_GETATTR_REQ message
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_ioc_getattr_req_s {
    pub mh: bfi_mhdr_s,
    pub attr_addr: bfi_addr_u,
}

pub const BFI_IOC_ATTR_UUID_SZ: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_ioc_attr_s {
    pub /: *mut *mut wwn_t mfg_pwwn; / Mfg port wwn,
    pub /: *mut *mut wwn_t mfg_nwwn; / Mfg node wwn,
    pub /: *mut *mut mac_t mfg_mac; / Mfg mac,
    pub /: *mut *mut u8 port_mode; / bfi_port_mode,
    pub rsvd_a: u8,
    pub pwwn: wwn_t,
    pub nwwn: wwn_t,
    pub /: *mut *mut mac_t mac; / PBC or Mfg mac,
    pub rsvd_b: u16,
    pub fcoe_mac: mac_t,
    pub rsvd_c: u16,
    pub brcd_serialnum: [c_char; STRSZ(BFA_MFG_SERIALNUM_SIZE)],
    pub pcie_gen: u8,
    pub pcie_lanes_orig: u8,
    pub pcie_lanes: u8,
    pub /: *mut *mut u8 rx_bbcredit; / receive buffer credits,
    pub /: *mut *mut u32 adapter_prop; / adapter properties,
    pub /: *mut *mut u16 maxfrsize; / max receive frame size,
    pub asic_rev: c_char,
    pub rsvd_d: u8,
    pub fw_version: [c_char; BFA_VERSION_LEN],
    pub optrom_version: [c_char; BFA_VERSION_LEN],
    pub vpd: bfa_mfg_vpd_s,
    pub /: *mut *mut u32 card_type; / card type,
    pub /: *mut *mut u8 mfg_day; / manufacturing day,
    pub /: *mut *mut u8 mfg_month; / manufacturing month,
    pub /: *mut *mut u16 mfg_year; / manufacturing year,
    pub /: *mut *mut u8 uuid[BFI_IOC_ATTR_UUID_SZ]; /!< chinook uuid,
}

//
// BFI_IOC_I2H_GETATTR_REPLY message
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_ioc_getattr_reply_s {
    pub /: *mut *mut bfi_mhdr_s mh; / Common msg header,
    pub /: *mut *mut u8 status; / cfg reply status,
    pub rsvd: [u8; 3],
}

//
// Firmware memory page offsets
//

//
// Firmware statistic offset
//

//
// Firmware trace offset
//

pub const BFI_IOC_TRC_ENTS: c_int = 256;

pub const BFI_IOC_MD5SUM_SZ: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_ioc_fwver_s {

    pub patch: u8,
    pub maint: u8,
    pub minor: u8,
    pub major: u8,
    pub rsvd: [u8; 2],
    pub build: u8,
    pub phase: u8,

    pub major: u8,
    pub minor: u8,
    pub maint: u8,
    pub patch: u8,
    pub phase: u8,
    pub build: u8,
    pub rsvd: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_ioc_image_hdr_s {
    pub /: *mut *mut u32 signature; / constant signature,
    pub /: *mut *mut u8 asic_gen; / asic generation,
    pub asic_mode: u8,
    pub /: *mut *mut u8 port0_mode; / device mode for port 0,
    pub /: *mut *mut u8 port1_mode; / device mode for port 1,
    pub /: *mut *mut u32 exec; / exec vector,
    pub /: *mut *mut u32 bootenv; / firmware boot env,
    pub rsvd_b: [u32; 2],
    pub fwver: bfi_ioc_fwver_s,
    pub md5sum: [u32; BFI_IOC_MD5SUM_SZ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_ioc_img_ver_cmp_e {
    BFI_IOC_IMG_VER_INCOMP,
    BFI_IOC_IMG_VER_OLD,
    BFI_IOC_IMG_VER_SAME,
    BFI_IOC_IMG_VER_BETTER
}

pub const BFI_FWBOOT_DEVMODE_OFF: c_int = 4;
pub const BFI_FWBOOT_TYPE_OFF: c_int = 8;
pub const BFI_FWBOOT_ENV_OFF: c_int = 12;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_fwboot_type {
    BFI_FWBOOT_TYPE_NORMAL  = 0,
    BFI_FWBOOT_TYPE_FLASH   = 1,
    BFI_FWBOOT_TYPE_MEMTEST = 2,
}

pub const BFI_FWBOOT_TYPE_NORMAL: c_int = 0;
pub const BFI_FWBOOT_TYPE_MEMTEST: c_int = 2;
pub const BFI_FWBOOT_ENV_OS: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_port_mode {
    BFI_PORT_MODE_FC	= 1,
    BFI_PORT_MODE_ETH	= 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_ioc_hbeat_s {
    pub /: *mut *mut bfi_mhdr_s mh; / common msg header,
    pub /: *mut *mut u32 hb_count; / current heart beat count,
}

//
// IOC hardware/firmware state
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_ioc_state {
    BFI_IOC_UNINIT		= 0,	/*  not initialized		     */
    BFI_IOC_INITING		= 1,	/*  h/w is being initialized	     */
    BFI_IOC_HWINIT		= 2,	/*  h/w is initialized		     */
    BFI_IOC_CFG		= 3,	/*  IOC configuration in progress   */
    BFI_IOC_OP		= 4,	/*  IOC is operational		     */
    BFI_IOC_DISABLING	= 5,	/*  IOC is being disabled	     */
    BFI_IOC_DISABLED	= 6,	/*  IOC is disabled		     */
    BFI_IOC_CFG_DISABLED	= 7,	/*  IOC is being disabled;transient */
    BFI_IOC_FAIL		= 8,	/*  IOC heart-beat failure	     */
    BFI_IOC_MEMTEST		= 9,	/*  IOC is doing memtest	     */
}

pub const BFA_IOC_CB_JOIN_SH: c_int = 16;
pub const BFA_IOC_CB_FWSTATE_MASK: c_uint = 0x0000ffff;
pub const BFA_IOC_CB_JOIN_MASK: c_uint = 0xffff0000;
pub const BFI_IOC_ENDIAN_SIG: c_uint = 0x12345678;

//
// BFI_IOC_H2I_ENABLE_REQ & BFI_IOC_H2I_DISABLE_REQ messages
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_ioc_ctrl_req_s {
    pub mh: bfi_mhdr_s,
    pub clscode: u16,
    pub rsvd: u16,
    pub tv_sec: u32,
}

//
// BFI_IOC_I2H_ENABLE_REPLY & BFI_IOC_I2H_DISABLE_REPLY messages
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_ioc_ctrl_reply_s {
    pub /: *mut *mut bfi_mhdr_s mh; / Common msg header,
    pub /: *mut *mut u8 status; / enable/disable status,
    pub /: *mut *mut u8 port_mode; / bfa_mode_s,
    pub /: *mut *mut u8 cap_bm; / capability bit mask,
    pub rsvd: u8,
}

pub const BFI_IOC_MSGSZ: c_int = 8;
//
// H2I Messages
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union bfi_ioc_h2i_msg_u {
    pub mh: bfi_mhdr_s,
    pub enable_req: bfi_ioc_ctrl_req_s,
    pub disable_req: bfi_ioc_ctrl_req_s,
    pub getattr_req: bfi_ioc_getattr_req_s,
    pub mboxmsg: [u32; BFI_IOC_MSGSZ],
}

//
// I2H Messages
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union bfi_ioc_i2h_msg_u {
    pub mh: bfi_mhdr_s,
    pub fw_event: bfi_ioc_ctrl_reply_s,
    pub mboxmsg: [u32; BFI_IOC_MSGSZ],
}

//
// ----------------------------------------------------------------------
// PBC
// ----------------------------------------------------------------------
//
pub const BFI_PBC_MAX_BLUNS: c_int = 8;
pub const BFI_PBC_MAX_VPORTS: c_int = 16;
pub const BFI_PBC_PORT_DISABLED: c_int = 2;
//
// PBC boot lun configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_pbc_blun_s {
    pub tgt_pwwn: wwn_t,
    pub tgt_lun: scsi_lun,
}

//
// PBC virtual port configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_pbc_vport_s {
    pub vp_pwwn: wwn_t,
    pub vp_nwwn: wwn_t,
}

//
// BFI pre-boot configuration information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_pbc_s {
    pub port_enabled: u8,
    pub boot_enabled: u8,
    pub nbluns: u8,
    pub nvports: u8,
    pub port_speed: u8,
    pub rsvd_a: u8,
    pub hss: u16,
    pub pbc_pwwn: wwn_t,
    pub pbc_nwwn: wwn_t,
    pub blun: [bfi_pbc_blun_s; BFI_PBC_MAX_BLUNS],
    pub vport: [bfi_pbc_vport_s; BFI_PBC_MAX_VPORTS],
}

//
// ----------------------------------------------------------------------
// MSGQ
// ----------------------------------------------------------------------
//

// q_depth must be power of 2

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_msgq_h2i_msgs_e {
    BFI_MSGQ_H2I_INIT_REQ	= 1,
    BFI_MSGQ_H2I_DOORBELL	= 2,
    BFI_MSGQ_H2I_SHUTDOWN	= 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_msgq_i2h_msgs_e {
    BFI_MSGQ_I2H_INIT_RSP	= 1,
    BFI_MSGQ_I2H_DOORBELL	= 2,
}

// Messages(commands/responsed/AENS will have the following header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_msgq_mhdr_s {
    pub msg_class: u8,
    pub msg_id: u8,
    pub msg_token: u16,
    pub num_entries: u16,
    pub enet_id: u8,
    pub rsvd: [u8; 1],
}

//
// Mailbox  for messaging interface
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_msgq_s {
    pub addr: bfi_addr_u,
    pub /: *mut *mut u16 q_depth; / Total num of entries in the queue,
    pub rsvd: [u8; 2],
}

// BFI_ENET_MSGQ_CFG_REQ TBD init or cfg?
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_msgq_cfg_req_s {
    pub mh: bfi_mhdr_s,
    pub cmdq: bfi_msgq_s,
    pub rspq: bfi_msgq_s,
}

// BFI_ENET_MSGQ_CFG_RSP
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_msgq_cfg_rsp_s {
    pub mh: bfi_mhdr_s,
    pub cmd_status: u8,
    pub rsvd: [u8; 3],
}

// BFI_MSGQ_H2I_DOORBELL
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_msgq_h2i_db_s {
    pub mh: bfi_mhdr_s,
    pub cmdq_pi: u16,
    pub rspq_ci: u16,
}

// BFI_MSGQ_I2H_DOORBELL
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_msgq_i2h_db_s {
    pub mh: bfi_mhdr_s,
    pub rspq_pi: u16,
    pub cmdq_ci: u16,
}

// BFI port specific

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_port_h2i {
    BFI_PORT_H2I_ENABLE_REQ         = (1),
    BFI_PORT_H2I_DISABLE_REQ        = (2),
    BFI_PORT_H2I_GET_STATS_REQ      = (3),
    BFI_PORT_H2I_CLEAR_STATS_REQ    = (4),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_port_i2h {
    BFI_PORT_I2H_ENABLE_RSP         = BFA_I2HM(1),
    BFI_PORT_I2H_DISABLE_RSP        = BFA_I2HM(2),
    BFI_PORT_I2H_GET_STATS_RSP      = BFA_I2HM(3),
    BFI_PORT_I2H_CLEAR_STATS_RSP    = BFA_I2HM(4),
}

//
// Generic REQ type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_port_generic_req_s {
    pub /: *mut *mut bfi_mhdr_s mh; / msg header,
    pub /: *mut *mut u32 msgtag; / msgtag for reply,
    pub rsvd: u32,
}

//
// Generic RSP type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_port_generic_rsp_s {
    pub /: *mut *mut bfi_mhdr_s mh; / common msg header,
    pub /: *mut *mut u8 status; / port enable status,
    pub rsvd: [u8; 3],
    pub /: *mut *mut u32 msgtag; / msgtag for reply,
}

//
// BFI_PORT_H2I_GET_STATS_REQ
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_port_get_stats_req_s {
    pub /: *mut *mut bfi_mhdr_s mh; / common msg header,
    pub dma_addr: bfi_addr_u,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union bfi_port_h2i_msg_u {
    pub mh: bfi_mhdr_s,
    pub enable_req: bfi_port_generic_req_s,
    pub disable_req: bfi_port_generic_req_s,
    pub getstats_req: bfi_port_get_stats_req_s,
    pub clearstats_req: bfi_port_generic_req_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union bfi_port_i2h_msg_u {
    pub mh: bfi_mhdr_s,
    pub enable_rsp: bfi_port_generic_rsp_s,
    pub disable_rsp: bfi_port_generic_rsp_s,
    pub getstats_rsp: bfi_port_generic_rsp_s,
    pub clearstats_rsp: bfi_port_generic_rsp_s,
}

//
// ----------------------------------------------------------------------
// ABLK
// ----------------------------------------------------------------------
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_ablk_h2i_msgs_e {
    BFI_ABLK_H2I_QUERY		= 1,
    BFI_ABLK_H2I_ADPT_CONFIG	= 2,
    BFI_ABLK_H2I_PORT_CONFIG	= 3,
    BFI_ABLK_H2I_PF_CREATE		= 4,
    BFI_ABLK_H2I_PF_DELETE		= 5,
    BFI_ABLK_H2I_PF_UPDATE		= 6,
    BFI_ABLK_H2I_OPTROM_ENABLE	= 7,
    BFI_ABLK_H2I_OPTROM_DISABLE	= 8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_ablk_i2h_msgs_e {
    BFI_ABLK_I2H_QUERY		= BFA_I2HM(BFI_ABLK_H2I_QUERY),
    BFI_ABLK_I2H_ADPT_CONFIG	= BFA_I2HM(BFI_ABLK_H2I_ADPT_CONFIG),
    BFI_ABLK_I2H_PORT_CONFIG	= BFA_I2HM(BFI_ABLK_H2I_PORT_CONFIG),
    BFI_ABLK_I2H_PF_CREATE		= BFA_I2HM(BFI_ABLK_H2I_PF_CREATE),
    BFI_ABLK_I2H_PF_DELETE		= BFA_I2HM(BFI_ABLK_H2I_PF_DELETE),
    BFI_ABLK_I2H_PF_UPDATE		= BFA_I2HM(BFI_ABLK_H2I_PF_UPDATE),
    BFI_ABLK_I2H_OPTROM_ENABLE	= BFA_I2HM(BFI_ABLK_H2I_OPTROM_ENABLE),
    BFI_ABLK_I2H_OPTROM_DISABLE	= BFA_I2HM(BFI_ABLK_H2I_OPTROM_DISABLE),
}

// BFI_ABLK_H2I_QUERY
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_ablk_h2i_query_s {
    pub mh: bfi_mhdr_s,
    pub addr: bfi_addr_u,
}

// BFI_ABL_H2I_ADPT_CONFIG, BFI_ABLK_H2I_PORT_CONFIG
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_ablk_h2i_cfg_req_s {
    pub mh: bfi_mhdr_s,
    pub mode: u8,
    pub port: u8,
    pub max_pf: u8,
    pub max_vf: u8,
}

//
// BFI_ABLK_H2I_PF_CREATE, BFI_ABLK_H2I_PF_DELETE,
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_ablk_h2i_pf_req_s {
    pub mh: bfi_mhdr_s,
    pub pcifn: u8,
    pub port: u8,
    pub pers: u16,
    pub /: *mut *mut u16 bw_min; / percent BW @ max speed,
    pub /: *mut *mut u16 bw_max; / percent BW @ max speed,
}

// BFI_ABLK_H2I_OPTROM_ENABLE, BFI_ABLK_H2I_OPTROM_DISABLE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_ablk_h2i_optrom_s {
    pub mh: bfi_mhdr_s,
}

//
// BFI_ABLK_I2H_QUERY
// BFI_ABLK_I2H_PORT_CONFIG
// BFI_ABLK_I2H_PF_CREATE
// BFI_ABLK_I2H_PF_DELETE
// BFI_ABLK_I2H_PF_UPDATE
// BFI_ABLK_I2H_OPTROM_ENABLE
// BFI_ABLK_I2H_OPTROM_DISABLE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_ablk_i2h_rsp_s {
    pub mh: bfi_mhdr_s,
    pub status: u8,
    pub pcifn: u8,
    pub port_mode: u8,
}

//
// CEE module specific messages
//
// Mailbox commands from host to firmware
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_cee_h2i_msgs_e {
    BFI_CEE_H2I_GET_CFG_REQ = 1,
    BFI_CEE_H2I_RESET_STATS = 2,
    BFI_CEE_H2I_GET_STATS_REQ = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_cee_i2h_msgs_e {
    BFI_CEE_I2H_GET_CFG_RSP = BFA_I2HM(1),
    BFI_CEE_I2H_RESET_STATS_RSP = BFA_I2HM(2),
    BFI_CEE_I2H_GET_STATS_RSP = BFA_I2HM(3),
}

//
// H2I command structure for resetting the stats
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_cee_reset_stats_s {
    pub mh: bfi_mhdr_s,
}

//
// Get configuration  command from host
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_cee_get_req_s {
    pub mh: bfi_mhdr_s,
    pub dma_addr: bfi_addr_u,
}

//
// Reply message from firmware
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_cee_get_rsp_s {
    pub mh: bfi_mhdr_s,
    pub cmd_status: u8,
    pub rsvd: [u8; 3],
}

//
// Reply message from firmware
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_cee_stats_rsp_s {
    pub mh: bfi_mhdr_s,
    pub cmd_status: u8,
    pub rsvd: [u8; 3],
}

// Mailbox message structures from firmware to host
#[repr(C)]
#[derive(Copy, Clone)]
pub union bfi_cee_i2h_msg_u {
    pub mh: bfi_mhdr_s,
    pub get_rsp: bfi_cee_get_rsp_s,
    pub stats_rsp: bfi_cee_stats_rsp_s,
}

//
// SFP related
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_sfp_h2i_e {
    BFI_SFP_H2I_SHOW	= 1,
    BFI_SFP_H2I_SCN		= 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_sfp_i2h_e {
    BFI_SFP_I2H_SHOW = BFA_I2HM(BFI_SFP_H2I_SHOW),
    BFI_SFP_I2H_SCN	 = BFA_I2HM(BFI_SFP_H2I_SCN),
}

//
// SFP state change notification
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_sfp_scn_s {
    pub /: *mut *mut bfi_mhdr_s mhr; / host msg header,
    pub event: u8,
    pub sfpid: u8,
    pub /: *mut *mut u8 pomlvl; / pom level: normal/warning/alarm,
    pub /: *mut *mut u8 is_elb; / e-loopback,
}

//
// SFP state
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_sfp_stat_e {
    BFA_SFP_STATE_INIT	= 0,	/* SFP state is uninit	*/
    BFA_SFP_STATE_REMOVED	= 1,	/* SFP is removed	*/
    BFA_SFP_STATE_INSERTED	= 2,	/* SFP is inserted	*/
    BFA_SFP_STATE_VALID	= 3,	/* SFP is valid		*/
    BFA_SFP_STATE_UNSUPPORT	= 4,	/* SFP is unsupport	*/
    BFA_SFP_STATE_FAILED	= 5,	/* SFP i2c read fail	*/
}

//
// SFP memory access type
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_sfp_mem_e {
    BFI_SFP_MEM_ALL		= 0x1,  /* access all data field */
    BFI_SFP_MEM_DIAGEXT	= 0x2,  /* access diag ext data field only */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_sfp_req_s {
    pub mh: bfi_mhdr_s,
    pub memtype: u8,
    pub rsvd: [u8; 3],
    pub alen: bfi_alen_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_sfp_rsp_s {
    pub mh: bfi_mhdr_s,
    pub status: u8,
    pub state: u8,
    pub rsvd: [u8; 2],
}

//
// FLASH module specific
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_flash_h2i_msgs {
    BFI_FLASH_H2I_QUERY_REQ = 1,
    BFI_FLASH_H2I_ERASE_REQ = 2,
    BFI_FLASH_H2I_WRITE_REQ = 3,
    BFI_FLASH_H2I_READ_REQ = 4,
    BFI_FLASH_H2I_BOOT_VER_REQ = 5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_flash_i2h_msgs {
    BFI_FLASH_I2H_QUERY_RSP = BFA_I2HM(1),
    BFI_FLASH_I2H_ERASE_RSP = BFA_I2HM(2),
    BFI_FLASH_I2H_WRITE_RSP = BFA_I2HM(3),
    BFI_FLASH_I2H_READ_RSP = BFA_I2HM(4),
    BFI_FLASH_I2H_BOOT_VER_RSP = BFA_I2HM(5),
    BFI_FLASH_I2H_EVENT = BFA_I2HM(127),
}

//
// Flash query request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_flash_query_req_s {
    pub /: *mut *mut bfi_mhdr_s mh; / Common msg header,
    pub alen: bfi_alen_s,
}

//
// Flash erase request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_flash_erase_req_s {
    pub /: *mut *mut bfi_mhdr_s mh; / Common msg header,
    pub /: *mut *mut u32 type; / partition type,
    pub /: *mut *mut u8 instance; / partition instance,
    pub rsv: [u8; 3],
}

//
// Flash write request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_flash_write_req_s {
    pub /: *mut *mut bfi_mhdr_s mh; / Common msg header,
    pub alen: bfi_alen_s,
    pub /: *mut *mut u32 type; / partition type,
    pub /: *mut *mut u8 instance; / partition instance,
    pub last: u8,
    pub rsv: [u8; 2],
    pub offset: u32,
    pub length: u32,
}

//
// Flash read request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_flash_read_req_s {
    pub /: *mut *mut bfi_mhdr_s mh; / Common msg header,
    pub /: *mut *mut u32 type; / partition type,
    pub /: *mut *mut u8 instance; / partition instance,
    pub rsv: [u8; 3],
    pub offset: u32,
    pub length: u32,
    pub alen: bfi_alen_s,
}

//
// Flash query response
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_flash_query_rsp_s {
    pub /: *mut *mut bfi_mhdr_s mh; / Common msg header,
    pub status: u32,
}

//
// Flash read response
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_flash_read_rsp_s {
    pub /: *mut *mut bfi_mhdr_s mh; / Common msg header,
    pub /: *mut *mut u32 type; / partition type,
    pub /: *mut *mut u8 instance; / partition instance,
    pub rsv: [u8; 3],
    pub status: u32,
    pub length: u32,
}

//
// Flash write response
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_flash_write_rsp_s {
    pub /: *mut *mut bfi_mhdr_s mh; / Common msg header,
    pub /: *mut *mut u32 type; / partition type,
    pub /: *mut *mut u8 instance; / partition instance,
    pub rsv: [u8; 3],
    pub status: u32,
    pub length: u32,
}

//
// Flash erase response
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_flash_erase_rsp_s {
    pub /: *mut *mut bfi_mhdr_s mh; / Common msg header,
    pub /: *mut *mut u32 type; / partition type,
    pub /: *mut *mut u8 instance; / partition instance,
    pub rsv: [u8; 3],
    pub status: u32,
}

//
// Flash event notification
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_flash_event_s {
    pub /: *mut *mut bfi_mhdr_s mh; / Common msg header,
    pub status: bfa_status_t,
    pub param: u32,
}

//
// ----------------------------------------------------------------------
// DIAG
// ----------------------------------------------------------------------
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_diag_h2i {
    BFI_DIAG_H2I_PORTBEACON = 1,
    BFI_DIAG_H2I_LOOPBACK = 2,
    BFI_DIAG_H2I_FWPING = 3,
    BFI_DIAG_H2I_TEMPSENSOR = 4,
    BFI_DIAG_H2I_LEDTEST = 5,
    BFI_DIAG_H2I_QTEST      = 6,
    BFI_DIAG_H2I_DPORT	= 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_diag_i2h {
    BFI_DIAG_I2H_PORTBEACON = BFA_I2HM(BFI_DIAG_H2I_PORTBEACON),
    BFI_DIAG_I2H_LOOPBACK = BFA_I2HM(BFI_DIAG_H2I_LOOPBACK),
    BFI_DIAG_I2H_FWPING = BFA_I2HM(BFI_DIAG_H2I_FWPING),
    BFI_DIAG_I2H_TEMPSENSOR = BFA_I2HM(BFI_DIAG_H2I_TEMPSENSOR),
    BFI_DIAG_I2H_LEDTEST = BFA_I2HM(BFI_DIAG_H2I_LEDTEST),
    BFI_DIAG_I2H_QTEST      = BFA_I2HM(BFI_DIAG_H2I_QTEST),
    BFI_DIAG_I2H_DPORT	= BFA_I2HM(BFI_DIAG_H2I_DPORT),
    BFI_DIAG_I2H_DPORT_SCN	= BFA_I2HM(8),
}

pub const BFI_DIAG_MAX_SGES: c_int = 2;

pub const BFI_BOOT_MEMTEST_RES_ADDR: c_uint = 0x900;
pub const BFI_BOOT_MEMTEST_RES_SIG: c_uint = 0xA0A1A2A3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_diag_lb_req_s {
    pub mh: bfi_mhdr_s,
    pub loopcnt: u32,
    pub pattern: u32,
    pub /: *mut *mut u8 lb_mode; /!< bfa_port_opmode_t,
    pub /: *mut *mut u8 speed; /!< bfa_port_speed_t,
    pub rsvd: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_diag_lb_rsp_s {
    pub /: *mut *mut bfi_mhdr_s mh; / 4 bytes,
    pub /: *mut *mut bfa_diag_loopback_result_s res; / 16 bytes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_diag_fwping_req_s {
    pub /: *mut *mut bfi_mhdr_s mh; / 4 bytes,
    pub /: *mut *mut bfi_alen_s alen; / 12 bytes,
    pub /: *mut *mut u32 data; / user input data pattern,
    pub /: *mut *mut u32 count; / user input dma count,
    pub /: *mut *mut u8 qtag; / track CPE vc,
    pub rsv: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_diag_fwping_rsp_s {
    pub /: *mut *mut bfi_mhdr_s mh; / 4 bytes,
    pub /: *mut *mut u32 data; / user input data pattern,
    pub /: *mut *mut u8 qtag; / track CPE vc,
    pub /: *mut *mut u8 dma_status; / dma status,
    pub rsv: [u8; 2],
}

//
// Temperature Sensor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_diag_ts_req_s {
    pub /: *mut *mut bfi_mhdr_s mh; / 4 bytes,
    pub /: *mut *mut u16 temp; / 10-bit A/D value,
    pub /: *mut *mut u16 brd_temp; / 9-bit board temp,
    pub status: u8,
    pub /: *mut *mut u8 ts_junc; / show junction tempsensor,
    pub /: *mut *mut u8 ts_brd; / show board tempsensor,
    pub rsv: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_diag_ledtest_req_s {
    pub /: *mut *mut bfi_mhdr_s mh; / 4 bytes,
    pub cmd: u8,
    pub color: u8,
    pub portid: u8,
    pub /: *mut *mut u8 led; / bitmap of LEDs to be tested,
    pub /: *mut *mut u16 freq; / no. of blinks every 10 secs,
    pub rsv: [u8; 2],
}

// notify host led operation is done
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_diag_ledtest_rsp_s {
    pub /: *mut *mut bfi_mhdr_s mh; / 4 bytes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_diag_portbeacon_req_s {
    pub /: *mut *mut bfi_mhdr_s mh; / 4 bytes,
    pub /: *mut *mut u32 period; / beaconing period,
    pub /: *mut *mut u8 beacon; / 1: beacon on,
    pub rsvd: [u8; 3],
}

// notify host the beacon is off
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_diag_portbeacon_rsp_s {
    pub /: *mut *mut bfi_mhdr_s mh; / 4 bytes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_diag_qtest_req_s {
    pub /: *mut *mut bfi_mhdr_s mh; / 4 bytes,
    pub /: *mut *mut u32 data[BFI_LMSG_PL_WSZ]; / fill up tcm prefetch area,
}

//
// D-port test
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_dport_req {
    BFI_DPORT_DISABLE	= 0,	/* disable dport request	*/
    BFI_DPORT_ENABLE	= 1,	/* enable dport request		*/
    BFI_DPORT_START		= 2,	/* start dport request	*/
    BFI_DPORT_SHOW		= 3,	/* show dport request	*/
    BFI_DPORT_DYN_DISABLE	= 4,	/* disable dynamic dport request */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_dport_scn {
    BFI_DPORT_SCN_TESTSTART		= 1,
    BFI_DPORT_SCN_TESTCOMP		= 2,
    BFI_DPORT_SCN_SFP_REMOVED	= 3,
    BFI_DPORT_SCN_DDPORT_ENABLE	= 4,
    BFI_DPORT_SCN_DDPORT_DISABLE	= 5,
    BFI_DPORT_SCN_FCPORT_DISABLE	= 6,
    BFI_DPORT_SCN_SUBTESTSTART	= 7,
    BFI_DPORT_SCN_TESTSKIP		= 8,
    BFI_DPORT_SCN_DDPORT_DISABLED	= 9,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_diag_dport_req_s {
    pub /: *mut *mut bfi_mhdr_s mh; / 4 bytes,
    pub /: *mut *mut u8 req; / request 1: enable 0: disable,
    pub rsvd: [u8; 3],
    pub lpcnt: u32,
    pub payload: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_diag_dport_rsp_s {
    pub /: *mut *mut bfi_mhdr_s mh; / header 4 bytes,
    pub /: *mut *mut bfa_status_t status; / reply status,
    pub /: *mut *mut wwn_t pwwn; / switch port wwn. 8 bytes,
    pub /: *mut *mut wwn_t nwwn; / switch node wwn. 8 bytes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_diag_dport_scn_teststart_s {
    pub /: *mut *mut wwn_t pwwn; / switch port wwn. 8 bytes,
    pub /: *mut *mut wwn_t nwwn; / switch node wwn. 8 bytes,
    pub /: *mut *mut u8 type; / bfa_diag_dport_test_type_e,
    pub /: *mut *mut u8 mode; / bfa_diag_dport_test_opmode,
    pub rsvd: [u8; 2],
    pub /: *mut *mut u32 numfrm; / from switch uint in 1M,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_diag_dport_scn_testcomp_s {
    pub /: *mut *mut u8 status; / bfa_diag_dport_test_status_e,
    pub /: *mut *mut u8 speed; / bfa_port_speed_t,
    pub /: *mut *mut u16 numbuffer; / from switch,
    pub /: *mut *mut u8 subtest_status[DPORT_TEST_MAX]; / 4 bytes,
    pub /: *mut *mut u32 latency; / from switch,
    pub /: *mut *mut u32 distance; / from switch unit in meters,
// Buffers required to saturate the link
    pub /: *mut *mut u16 frm_sz; / from switch for buf_reqd,
    pub rsvd: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_diag_dport_scn_s {
    pub /: *mut *mut bfi_mhdr_s mh; / header 4 bytes,
    pub /: *mut *mut u8 state; / new state,
    pub rsvd: [u8; 3],
    pub teststart: bfi_diag_dport_scn_teststart_s,
    pub testcomp: bfi_diag_dport_scn_testcomp_s,
    pub info: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union bfi_diag_dport_msg_u {
    pub req: bfi_diag_dport_req_s,
    pub rsp: bfi_diag_dport_rsp_s,
    pub scn: bfi_diag_dport_scn_s,
}

//
// PHY module specific
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_phy_h2i_msgs_e {
    BFI_PHY_H2I_QUERY_REQ = 1,
    BFI_PHY_H2I_STATS_REQ = 2,
    BFI_PHY_H2I_WRITE_REQ = 3,
    BFI_PHY_H2I_READ_REQ = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_phy_i2h_msgs_e {
    BFI_PHY_I2H_QUERY_RSP = BFA_I2HM(1),
    BFI_PHY_I2H_STATS_RSP = BFA_I2HM(2),
    BFI_PHY_I2H_WRITE_RSP = BFA_I2HM(3),
    BFI_PHY_I2H_READ_RSP = BFA_I2HM(4),
}

//
// External PHY query request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_phy_query_req_s {
    pub /: *mut *mut bfi_mhdr_s mh; / Common msg header,
    pub instance: u8,
    pub rsv: [u8; 3],
    pub alen: bfi_alen_s,
}

//
// External PHY stats request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_phy_stats_req_s {
    pub /: *mut *mut bfi_mhdr_s mh; / Common msg header,
    pub instance: u8,
    pub rsv: [u8; 3],
    pub alen: bfi_alen_s,
}

//
// External PHY write request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_phy_write_req_s {
    pub /: *mut *mut bfi_mhdr_s mh; / Common msg header,
    pub instance: u8,
    pub last: u8,
    pub rsv: [u8; 2],
    pub offset: u32,
    pub length: u32,
    pub alen: bfi_alen_s,
}

//
// External PHY read request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_phy_read_req_s {
    pub /: *mut *mut bfi_mhdr_s mh; / Common msg header,
    pub instance: u8,
    pub rsv: [u8; 3],
    pub offset: u32,
    pub length: u32,
    pub alen: bfi_alen_s,
}

//
// External PHY query response
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_phy_query_rsp_s {
    pub /: *mut *mut bfi_mhdr_s mh; / Common msg header,
    pub status: u32,
}

//
// External PHY stats response
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_phy_stats_rsp_s {
    pub /: *mut *mut bfi_mhdr_s mh; / Common msg header,
    pub status: u32,
}

//
// External PHY read response
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_phy_read_rsp_s {
    pub /: *mut *mut bfi_mhdr_s mh; / Common msg header,
    pub status: u32,
    pub length: u32,
}

//
// External PHY write response
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_phy_write_rsp_s {
    pub /: *mut *mut bfi_mhdr_s mh; / Common msg header,
    pub status: u32,
    pub length: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_fru_h2i_msgs {
    BFI_FRUVPD_H2I_WRITE_REQ = 1,
    BFI_FRUVPD_H2I_READ_REQ = 2,
    BFI_TFRU_H2I_WRITE_REQ = 3,
    BFI_TFRU_H2I_READ_REQ = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_fru_i2h_msgs {
    BFI_FRUVPD_I2H_WRITE_RSP = BFA_I2HM(1),
    BFI_FRUVPD_I2H_READ_RSP = BFA_I2HM(2),
    BFI_TFRU_I2H_WRITE_RSP = BFA_I2HM(3),
    BFI_TFRU_I2H_READ_RSP = BFA_I2HM(4),
}

//
// FRU write request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_fru_write_req_s {
    pub /: *mut *mut bfi_mhdr_s mh; / Common msg header,
    pub last: u8,
    pub rsv_1: [u8; 3],
    pub trfr_cmpl: u8,
    pub rsv_2: [u8; 3],
    pub offset: u32,
    pub length: u32,
    pub alen: bfi_alen_s,
}

//
// FRU read request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_fru_read_req_s {
    pub /: *mut *mut bfi_mhdr_s mh; / Common msg header,
    pub offset: u32,
    pub length: u32,
    pub alen: bfi_alen_s,
}

//
// FRU response
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_fru_rsp_s {
    pub /: *mut *mut bfi_mhdr_s mh; / Common msg header,
    pub status: u32,
    pub length: u32,
}

