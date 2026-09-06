//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/brocade/bna/bfi.h
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

// BFI FW image type

pub const BFI_FLASH_IMAGE_SZ: c_uint = 0x100000;
// Msg header common to all msgs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_mhdr {
    pub /: *mut *mut u8 msg_class; /!< @ref enum bfi_mclass,
    pub /: *mut *mut u8 msg_id; /!< msg opcode with in the class,
    pub qid: u8,
    pub /: *mut *mut u8 fn_lpu; /!< msg destination,
    pub h2i: } __packed,
    pub /: *mut *mut u16 i2htok; /!< token in msgs to host,
    pub mtag: } __packed,
    pub __packed: },

    pub \: (_mh).msg_class = (_mc);,
    pub \: (_mh).msg_id = (_op);,
    pub \: (_mh).mtag.h2i.fn_lpu = (_fn_lpu);,

    pub \: (_mh).msg_class = (_mc);,
    pub \: (_mh).msg_id = (_op);,
    pub \: (_mh).mtag.i2htok = (_i2htok);,
//
// Message opcodes: 0-127 to firmware, 128-255 to host
//
pub const BFI_I2H_OPCODE_BASE: c_int = 128;

//
// Scatter Gather Element and Page definition
//
// DMA addresses
#[repr(C)]
#[derive(Copy, Clone)]
pub union bfi_addr_u {
    pub addr_lo: u32,
    pub addr_hi: u32,
    pub a32: } __packed,
    pub __packed: },
// Generic DMA addr-len pair.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_alen {
    pub /: *mut *mut bfi_addr_u al_addr; / DMA addr of buffer,
    pub /: *mut *mut u32 al_len; / length of buffer,
    pub __packed: },
//
// Large Message structure - 128 Bytes size Msgs
//
pub const BFI_LMSG_SZ: c_int = 128;

// Mailbox message structure
pub const BFI_MBMSG_SZ: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_mbmsg {
    pub mh: bfi_mhdr,
    pub pl: [u32; BFI_MBMSG_SZ],
    pub __packed: },
// Supported PCI function class codes (personality)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_pcifn_class {
    BFI_PCIFN_CLASS_FC	= 0x0c04,
    BFI_PCIFN_CLASS_ETH	= 0x0200,
}

// Message Classes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_mclass {
    BFI_MC_IOC		= 1,	/*!< IO Controller (IOC)	    */
    BFI_MC_DIAG		= 2,	/*!< Diagnostic Msgs		    */
    BFI_MC_FLASH		= 3,	/*!< Flash message class	    */
    BFI_MC_CEE		= 4,	/*!< CEE			    */
    BFI_MC_FCPORT		= 5,	/*!< FC port			    */
    BFI_MC_IOCFC		= 6,	/*!< FC - IO Controller (IOC)	    */
    BFI_MC_LL		= 7,	/*!< Link Layer			    */
    BFI_MC_UF		= 8,	/*!< Unsolicited frame receive	    */
    BFI_MC_FCXP		= 9,	/*!< FC Transport		    */
    BFI_MC_LPS		= 10,	/*!< lport fc login services	    */
    BFI_MC_RPORT		= 11,	/*!< Remote port		    */
    BFI_MC_ITNIM		= 12,	/*!< I-T nexus (Initiator mode)	    */
    BFI_MC_IOIM_READ	= 13,	/*!< read IO (Initiator mode)	    */
    BFI_MC_IOIM_WRITE	= 14,	/*!< write IO (Initiator mode)	    */
    BFI_MC_IOIM_IO		= 15,	/*!< IO (Initiator mode)	    */
    BFI_MC_IOIM		= 16,	/*!< IO (Initiator mode)	    */
    BFI_MC_IOIM_IOCOM	= 17,	/*!< good IO completion		    */
    BFI_MC_TSKIM		= 18,	/*!< Initiator Task management	    */
    BFI_MC_SBOOT		= 19,	/*!< SAN boot services		    */
    BFI_MC_IPFC		= 20,	/*!< IP over FC Msgs		    */
    BFI_MC_PORT		= 21,	/*!< Physical port		    */
    BFI_MC_SFP		= 22,	/*!< SFP module			    */
    BFI_MC_MSGQ		= 23,	/*!< MSGQ			    */
    BFI_MC_ENET		= 24,	/*!< ENET commands/responses	    */
    BFI_MC_PHY		= 25,	/*!< External PHY message class	    */
    BFI_MC_NBOOT		= 26,	/*!< Network Boot		    */
    BFI_MC_TIO_READ		= 27,	/*!< read IO (Target mode)	    */
    BFI_MC_TIO_WRITE	= 28,	/*!< write IO (Target mode)	    */
    BFI_MC_TIO_DATA_XFERED	= 29,	/*!< ds transferred (target mode)   */
    BFI_MC_TIO_IO		= 30,	/*!< IO (Target mode)		    */
    BFI_MC_TIO		= 31,	/*!< IO (target mode)		    */
    BFI_MC_MFG		= 32,	/*!< MFG/ASIC block commands	    */
    BFI_MC_EDMA		= 33,	/*!< EDMA copy commands		    */
    BFI_MC_MAX		= 34
}

pub const BFI_FWBOOT_ENV_OS: c_int = 0;
// ----------------------------------------------------------------------
// IOC
// ----------------------------------------------------------------------
//
// Different asic generations
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_asic_gen {
    BFI_ASIC_GEN_CB		= 1,
    BFI_ASIC_GEN_CT		= 2,
    BFI_ASIC_GEN_CT2	= 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_asic_mode {
    BFI_ASIC_MODE_FC	= 1,	/* FC up to 8G speed		*/
    BFI_ASIC_MODE_FC16	= 2,	/* FC up to 16G speed		*/
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
}

// BFI_IOC_H2I_GETATTR_REQ message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_ioc_getattr_req {
    pub mh: bfi_mhdr,
    pub attr_addr: bfi_addr_u,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_ioc_attr {
    pub /: *mut *mut u64 mfg_pwwn; /!< Mfg port wwn,
    pub /: *mut *mut u64 mfg_nwwn; /!< Mfg node wwn,
    pub /: *mut *mut u8 mfg_mac[ETH_ALEN]; /!< Mfg mac,
    pub /: *mut *mut u8 port_mode; / enum bfi_port_mode,
    pub rsvd_a: u8,
    pub pwwn: u64,
    pub nwwn: u64,
    pub /: *mut *mut u8 mac[ETH_ALEN]; /!< PBC or Mfg mac,
    pub rsvd_b: u16,
    pub fcoe_mac: [u8; ETH_ALEN],
    pub rsvd_c: u16,
    pub brcd_serialnum: [c_char; STRSZ(BFA_MFG_SERIALNUM_SIZE)],
    pub pcie_gen: u8,
    pub pcie_lanes_orig: u8,
    pub pcie_lanes: u8,
    pub /: *mut *mut u8 rx_bbcredit; /!< receive buffer credits,
    pub /: *mut *mut u32 adapter_prop; /!< adapter properties,
    pub /: *mut *mut u16 maxfrsize; /!< max receive frame size,
    pub asic_rev: c_char,
    pub rsvd_d: u8,
    pub fw_version: [c_char; BFA_VERSION_LEN],
    pub optrom_version: [c_char; BFA_VERSION_LEN],
    pub vpd: bfa_mfg_vpd,
    pub /: *mut *mut u32 card_type; /!< card type,
    pub __packed: },
// BFI_IOC_I2H_GETATTR_REPLY message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_ioc_getattr_reply {
    pub /: *mut *mut bfi_mhdr mh; /!< Common msg header,
    pub /: *mut *mut u8 status; /!< cfg reply status,
    pub rsvd: [u8; 3],
    pub __packed: },
// Firmware memory page offsets

// Firmware statistic offset

// Firmware trace offset

pub const BFI_IOC_TRC_ENTS: c_int = 256;
pub const BFI_IOC_TRC_ENT_SZ: c_int = 16;
pub const BFI_IOC_TRC_HDR_SZ: c_int = 32;

pub const BFI_IOC_MD5SUM_SZ: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_ioc_fwver {

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
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_ioc_image_hdr {
    pub /: *mut *mut u32 signature; /!< constant signature,
    pub /: *mut *mut u8 asic_gen; /!< asic generation,
    pub asic_mode: u8,
    pub /: *mut *mut u8 port0_mode; /!< device mode for port 0,
    pub /: *mut *mut u8 port1_mode; /!< device mode for port 1,
    pub /: *mut *mut u32 exec; /!< exec vector,
    pub /: *mut *mut u32 bootenv; /!< firmware boot env,
    pub rsvd_b: [u32; 2],
    pub fwver: bfi_ioc_fwver,
    pub md5sum: [u32; BFI_IOC_MD5SUM_SZ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_ioc_img_ver_cmp {
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
    BFI_FWBOOT_TYPE_NORMAL	= 0,
    BFI_FWBOOT_TYPE_FLASH	= 1,
    BFI_FWBOOT_TYPE_MEMTEST	= 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_port_mode {
    BFI_PORT_MODE_FC	= 1,
    BFI_PORT_MODE_ETH	= 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_ioc_hbeat {
    pub /: *mut *mut bfi_mhdr mh; /!< common msg header,
    pub /: *mut *mut u32 hb_count; /!< current heart beat count,
    pub __packed: },
// IOC hardware/firmware state
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_ioc_state {
    BFI_IOC_UNINIT		= 0,	/*!< not initialized		     */
    BFI_IOC_INITING		= 1,	/*!< h/w is being initialized	     */
    BFI_IOC_HWINIT		= 2,	/*!< h/w is initialized		     */
    BFI_IOC_CFG		= 3,	/*!< IOC configuration in progress   */
    BFI_IOC_OP		= 4,	/*!< IOC is operational		     */
    BFI_IOC_DISABLING	= 5,	/*!< IOC is being disabled	     */
    BFI_IOC_DISABLED	= 6,	/*!< IOC is disabled		     */
    BFI_IOC_CFG_DISABLED	= 7,	/*!< IOC is being disabled;transient */
    BFI_IOC_FAIL		= 8,	/*!< IOC heart-beat failure	     */
    BFI_IOC_MEMTEST		= 9,	/*!< IOC is doing memtest	     */
}

}

// BFI_IOC_H2I_ENABLE_REQ & BFI_IOC_H2I_DISABLE_REQ messages
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_ioc_ctrl_req {
    pub mh: bfi_mhdr,
    pub clscode: u16,
    pub rsvd: u16,
    pub tv_sec: u32,
    pub __packed: },
// BFI_IOC_I2H_ENABLE_REPLY & BFI_IOC_I2H_DISABLE_REPLY messages
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_ioc_ctrl_reply {
    pub /: *mut *mut bfi_mhdr mh; /!< Common msg header,
    pub /: *mut *mut u8 status; /!< enable/disable status,
    pub /: *mut *mut u8 port_mode; /!< enum bfa_mode,
    pub /: *mut *mut u8 cap_bm; /!< capability bit mask,
    pub rsvd: u8,
    pub __packed: },
pub const BFI_IOC_MSGSZ: c_int = 8;
// H2I Messages
#[repr(C)]
#[derive(Copy, Clone)]
pub union bfi_ioc_h2i_msg_u {
    pub mh: bfi_mhdr,
    pub enable_req: bfi_ioc_ctrl_req,
    pub disable_req: bfi_ioc_ctrl_req,
    pub getattr_req: bfi_ioc_getattr_req,
    pub mboxmsg: [u32; BFI_IOC_MSGSZ],
    pub __packed: },
// I2H Messages
#[repr(C)]
#[derive(Copy, Clone)]
pub union bfi_ioc_i2h_msg_u {
    pub mh: bfi_mhdr,
    pub fw_event: bfi_ioc_ctrl_reply,
    pub mboxmsg: [u32; BFI_IOC_MSGSZ],
    pub __packed: },
// ----------------------------------------------------------------------
// MSGQ
// ----------------------------------------------------------------------
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_msgq_h2i_msgs {
    BFI_MSGQ_H2I_INIT_REQ	   = 1,
    BFI_MSGQ_H2I_DOORBELL_PI	= 2,
    BFI_MSGQ_H2I_DOORBELL_CI	= 3,
    BFI_MSGQ_H2I_CMDQ_COPY_RSP      = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_msgq_i2h_msgs {
    BFI_MSGQ_I2H_INIT_RSP	   = BFA_I2HM(BFI_MSGQ_H2I_INIT_REQ),
    BFI_MSGQ_I2H_DOORBELL_PI	= BFA_I2HM(BFI_MSGQ_H2I_DOORBELL_PI),
    BFI_MSGQ_I2H_DOORBELL_CI	= BFA_I2HM(BFI_MSGQ_H2I_DOORBELL_CI),
    BFI_MSGQ_I2H_CMDQ_COPY_REQ      = BFA_I2HM(BFI_MSGQ_H2I_CMDQ_COPY_RSP),
}

// Messages(commands/responsed/AENS will have the following header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_msgq_mhdr {
    pub msg_class: u8,
    pub msg_id: u8,
    pub msg_token: u16,
    pub num_entries: u16,
    pub enet_id: u8,
    pub rsvd: u8,
    pub __packed: },

    pub \: (_mh).msg_class = (_mc);,
    pub \: (_mh).msg_id = (_mid);,
    pub \: (_mh).msg_token = (_tok);,
    pub \: (_mh).enet_id = (_enet_id);,
//
// Mailbox  for messaging interface
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_msgq {
    pub addr: bfi_addr_u,
    pub /: *mut *mut u16 q_depth; / Total num of entries in the queue,
    pub rsvd: [u8; 2],
    pub __packed: },
// BFI_ENET_MSGQ_CFG_REQ TBD init or cfg?
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_msgq_cfg_req {
    pub mh: bfi_mhdr,
    pub cmdq: bfi_msgq,
    pub rspq: bfi_msgq,
    pub __packed: },
// BFI_ENET_MSGQ_CFG_RSP
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_msgq_cfg_rsp {
    pub mh: bfi_mhdr,
    pub cmd_status: u8,
    pub rsvd: [u8; 3],
    pub __packed: },
// BFI_MSGQ_H2I_DOORBELL
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_msgq_h2i_db {
    pub mh: bfi_mhdr,
    pub cmdq_pi: u16,
    pub rspq_ci: u16,
    pub idx: } __packed,
    pub __packed: },
// BFI_MSGQ_I2H_DOORBELL
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_msgq_i2h_db {
    pub mh: bfi_mhdr,
    pub rspq_pi: u16,
    pub cmdq_ci: u16,
    pub idx: } __packed,
    pub __packed: },
pub const BFI_CMD_COPY_SZ: c_int = 28;
// BFI_MSGQ_H2I_CMD_COPY_RSP
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_msgq_h2i_cmdq_copy_rsp {
    pub mh: bfi_mhdr,
    pub data: [u8; BFI_CMD_COPY_SZ],
    pub __packed: },
// BFI_MSGQ_I2H_CMD_COPY_REQ
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_msgq_i2h_cmdq_copy_req {
    pub mh: bfi_mhdr,
    pub offset: u16,
    pub len: u16,
    pub __packed: },
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
pub struct bfi_flash_query_req {
    pub /: *mut *mut bfi_mhdr mh; / Common msg header,
    pub alen: bfi_alen,
    pub __packed: },
//
// Flash write request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_flash_write_req {
    pub /: *mut *mut bfi_mhdr mh; / Common msg header,
    pub alen: bfi_alen,
    pub /: *mut *mut u32 type; / partition type,
    pub /: *mut *mut u8 instance; / partition instance,
    pub last: u8,
    pub rsv: [u8; 2],
    pub offset: u32,
    pub length: u32,
    pub __packed: },
//
// Flash read request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_flash_read_req {
    pub /: *mut *mut bfi_mhdr mh; / Common msg header,
    pub /: *mut *mut u32 type; / partition type,
    pub /: *mut *mut u8 instance; / partition instance,
    pub rsv: [u8; 3],
    pub offset: u32,
    pub length: u32,
    pub alen: bfi_alen,
    pub __packed: },
//
// Flash query response
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_flash_query_rsp {
    pub /: *mut *mut bfi_mhdr mh; / Common msg header,
    pub status: u32,
    pub __packed: },
//
// Flash read response
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_flash_read_rsp {
    pub /: *mut *mut bfi_mhdr mh; / Common msg header,
    pub /: *mut *mut u32 type; / partition type,
    pub /: *mut *mut u8 instance; / partition instance,
    pub rsv: [u8; 3],
    pub status: u32,
    pub length: u32,
    pub __packed: },
//
// Flash write response
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_flash_write_rsp {
    pub /: *mut *mut bfi_mhdr mh; / Common msg header,
    pub /: *mut *mut u32 type; / partition type,
    pub /: *mut *mut u8 instance; / partition instance,
    pub rsv: [u8; 3],
    pub status: u32,
    pub length: u32,
    pub __packed: },
