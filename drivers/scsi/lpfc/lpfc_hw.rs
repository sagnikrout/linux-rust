//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/lpfc/lpfc_hw.h
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


//
// This file is part of the Emulex Linux Device Driver for
// Fibre Channel Host Bus Adapters.
// Copyright (C) 2017-2026 Broadcom. All Rights Reserved. The term
// “Broadcom” refers to Broadcom Inc. and/or its subsidiaries.
// Copyright (C) 2004-2016 Emulex.  All rights reserved.
// EMULEX and SLI are trademarks of Emulex.
// www.broadcom.com
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General
// Public License as published by the Free Software Foundation.
// This program is distributed in the hope that it will be useful.
// ALL EXPRESS OR IMPLIED CONDITIONS, REPRESENTATIONS AND
// WARRANTIES, INCLUDING ANY IMPLIED WARRANTY OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE, OR NON-INFRINGEMENT, ARE
// DISCLAIMED, EXCEPT TO THE EXTENT THAT SUCH DISCLAIMERS ARE HELD
// TO BE LEGALLY INVALID.  See the GNU General Public License for
// more details, a copy of which can be found in the file COPYING
// included with this package.
//
pub const FDMI_DID: c_uint = 0xfffffaU;
pub const NameServer_DID: c_uint = 0xfffffcU;
pub const Fabric_Cntl_DID: c_uint = 0xfffffdU;
pub const Fabric_DID: c_uint = 0xfffffeU;
pub const Bcast_DID: c_uint = 0xffffffU;
pub const Mask_DID: c_uint = 0xffffffU;
pub const CT_DID_MASK: c_uint = 0xffff00U;
pub const Fabric_DID_MASK: c_uint = 0xfff000U;
pub const WELL_KNOWN_DID_MASK: c_uint = 0xfffff0U;
pub const PT2PT_LocalID: c_int = 1;
pub const PT2PT_RemoteID: c_int = 2;

pub const SLI2_IOCB_CMD_R3_ENTRIES: c_int = 0;
pub const SLI2_IOCB_RSP_R3_ENTRIES: c_int = 0;
pub const SLI2_IOCB_CMD_R3XTRA_ENTRIES: c_int = 24;
pub const SLI2_IOCB_RSP_R3XTRA_ENTRIES: c_int = 32;
pub const SLI2_IOCB_CMD_SIZE: c_int = 32;
pub const SLI2_IOCB_RSP_SIZE: c_int = 32;
pub const SLI3_IOCB_CMD_SIZE: c_int = 128;
pub const SLI3_IOCB_RSP_SIZE: c_int = 64;
pub const LPFC_UNREG_ALL_RPIS_VPORT: c_uint = 0xffff;
pub const LPFC_UNREG_ALL_DFLT_RPIS: c_uint = 0xffffffff;
// vendor ID used in SCSI netlink calls

pub const FW_REV_STR_SIZE: c_int = 32;
// Common Transport structures and definitions
#[repr(C)]
#[derive(Copy, Clone)]
pub union CtRevisionId {
// Structure is in Big Endian format
    pub Revision:8: u32,
    pub InId:24: u32,
    pub bits: },
    pub word: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union CtCommandResponse {
// Structure is in Big Endian format
    pub CmdRsp: __be16,
    pub Size: __be16,
    pub bits: },
    pub word: u32,
}

// FC4 Feature bits for RFF_ID
pub const FC4_FEATURE_TARGET: c_uint = 0x1;
pub const FC4_FEATURE_INIT: c_uint = 0x2;
pub const FC4_FEATURE_NVME_DISC: c_uint = 0x4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rft_word0 {
    RFT_FCP_REG	= (0x1 << 8),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rft_word1 {
    RFT_NVME_REG	= (0x1 << 8),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rft_word3 {
    RFT_APP_SERV_REG	= (0x1 << 0),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_sli_ct_request {
// Structure is in Big Endian format
    pub RevisionId: CtRevisionId,
    pub FsType: u8,
    pub FsSubType: u8,
    pub Options: u8,
    pub Rsrvd1: u8,
    pub CommandResponse: CtCommandResponse,
    pub Rsrvd2: u8,
    pub ReasonCode: u8,
    pub Explanation: u8,
    pub VendorUnique: u8,

    pub PortID: __be32,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gid {
    pub /: *mut *mut uint8_t PortType; / for GID_PT requests,
pub const GID_PT_N_PORT: c_int = 1;
    pub DomainScope: u8,
    pub AreaScope: u8,
    pub /: *mut *mut uint8_t Fc4Type; / for GID_FT requests,
    pub gid: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gid_ff {
    pub Flags: u8,
    pub DomainScope: u8,
    pub AreaScope: u8,
    pub rsvd1: u8,
    pub rsvd2: u8,
    pub rsvd3: u8,
    pub Fc4FBits: u8,
    pub Fc4Type: u8,
    pub gid_ff: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rft {
    pub /: *mut *mut __be32 port_id; / For RFT_ID requests,
    pub /: *mut *mut __be32 fcp_reg; / rsvd 31:9, fcp_reg 8, rsvd 7:0,
    pub /: *mut *mut __be32 nvme_reg; / rsvd 31:9, nvme_reg 8, rsvd 7:0,
    pub word2: __be32,
    pub /: *mut *mut __be32 app_serv_reg; / rsvd 31:1, app_serv_reg 0,
    pub word: [__be32; 4],
    pub rft: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rnn {
    pub /: *mut *mut uint32_t PortId; / For RNN_ID requests,
    pub wwnn: [u8; 8],
    pub rnn: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsnn {
    pub wwnn: [u8; 8],
    pub len: u8,
    pub symbname: [u8; 255],
    pub rsnn: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct da_id {
    pub port_id: u32,
    pub da_id: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rspn {
    pub PortId: u32,
    pub len: u8,
    pub symbname: [u8; 255],
    pub rspn: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rspni {
    pub pni: __be64,
    pub len: u8,
    pub symbname: [u8; 255],
    pub rspni: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gff {
    pub PortId: u32,
    pub gff: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gff_acc {
    pub fbits: [u8; 128],
    pub gff_acc: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gft {
    pub PortId: u32,
    pub gft: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gft_acc {
    pub fc4_types: [u32; 8],
    pub gft_acc: },
pub const FCP_TYPE_FEATURE_OFFSET: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rff {
    pub PortId: u32,
    pub reserved: [u8; 2],
    pub fbits: u8,
    pub /: *mut *mut uint8_t type_code; / type=8 for FCP,
    pub rff: },
    pub un: },
}

pub const SLI_CT_REVISION: c_int = 1;

//
// FsType Definitions
//
pub const SLI_CT_MANAGEMENT_SERVICE: c_uint = 0xFA;
pub const SLI_CT_TIME_SERVICE: c_uint = 0xFB;
pub const SLI_CT_DIRECTORY_SERVICE: c_uint = 0xFC;
pub const SLI_CT_FABRIC_CONTROLLER_SERVICE: c_uint = 0xFD;
//
// Directory Service Subtypes
//
pub const SLI_CT_DIRECTORY_NAME_SERVER: c_uint = 0x02;
//
// Response Codes
//
pub const SLI_CT_RESPONSE_FS_RJT: c_uint = 0x8001;
pub const SLI_CT_RESPONSE_FS_ACC: c_uint = 0x8002;
//
// Reason Codes
//
pub const SLI_CT_NO_ADDITIONAL_EXPL: c_uint = 0x0;
pub const SLI_CT_INVALID_COMMAND: c_uint = 0x01;
pub const SLI_CT_INVALID_VERSION: c_uint = 0x02;
pub const SLI_CT_LOGICAL_ERROR: c_uint = 0x03;
pub const SLI_CT_INVALID_IU_SIZE: c_uint = 0x04;
pub const SLI_CT_LOGICAL_BUSY: c_uint = 0x05;
pub const SLI_CT_PROTOCOL_ERROR: c_uint = 0x07;
pub const SLI_CT_UNABLE_TO_PERFORM_REQ: c_uint = 0x09;
pub const SLI_CT_REQ_NOT_SUPPORTED: c_uint = 0x0b;
pub const SLI_CT_HBA_INFO_NOT_REGISTERED: c_uint = 0x10;
pub const SLI_CT_MULTIPLE_HBA_ATTR_OF_SAME_TYPE: c_uint = 0x11;
pub const SLI_CT_INVALID_HBA_ATTR_BLOCK_LEN: c_uint = 0x12;
pub const SLI_CT_HBA_ATTR_NOT_PRESENT: c_uint = 0x13;
pub const SLI_CT_PORT_INFO_NOT_REGISTERED: c_uint = 0x20;
pub const SLI_CT_MULTIPLE_PORT_ATTR_OF_SAME_TYPE: c_uint = 0x21;
pub const SLI_CT_INVALID_PORT_ATTR_BLOCK_LEN: c_uint = 0x22;
pub const SLI_CT_VENDOR_UNIQUE: c_uint = 0xff;
//
// Name Server SLI_CT_UNABLE_TO_PERFORM_REQ Explanations
//
pub const SLI_CT_NO_PORT_ID: c_uint = 0x01;
pub const SLI_CT_NO_PORT_NAME: c_uint = 0x02;
pub const SLI_CT_NO_NODE_NAME: c_uint = 0x03;
pub const SLI_CT_NO_CLASS_OF_SERVICE: c_uint = 0x04;
pub const SLI_CT_NO_IP_ADDRESS: c_uint = 0x05;
pub const SLI_CT_NO_IPA: c_uint = 0x06;
pub const SLI_CT_NO_FC4_TYPES: c_uint = 0x07;
pub const SLI_CT_NO_SYMBOLIC_PORT_NAME: c_uint = 0x08;
pub const SLI_CT_NO_SYMBOLIC_NODE_NAME: c_uint = 0x09;
pub const SLI_CT_NO_PORT_TYPE: c_uint = 0x0A;
pub const SLI_CT_ACCESS_DENIED: c_uint = 0x10;
pub const SLI_CT_INVALID_PORT_ID: c_uint = 0x11;
pub const SLI_CT_DATABASE_EMPTY: c_uint = 0x12;
pub const SLI_CT_APP_ID_NOT_AVAILABLE: c_uint = 0x40;
//
// Name Server Command Codes
//
pub const SLI_CTNS_GA_NXT: c_uint = 0x0100;
pub const SLI_CTNS_GPN_ID: c_uint = 0x0112;
pub const SLI_CTNS_GNN_ID: c_uint = 0x0113;
pub const SLI_CTNS_GCS_ID: c_uint = 0x0114;
pub const SLI_CTNS_GFT_ID: c_uint = 0x0117;
pub const SLI_CTNS_GSPN_ID: c_uint = 0x0118;
pub const SLI_CTNS_GPT_ID: c_uint = 0x011A;
pub const SLI_CTNS_GFF_ID: c_uint = 0x011F;
pub const SLI_CTNS_GID_PN: c_uint = 0x0121;
pub const SLI_CTNS_GID_NN: c_uint = 0x0131;
pub const SLI_CTNS_GIP_NN: c_uint = 0x0135;
pub const SLI_CTNS_GIPA_NN: c_uint = 0x0136;
pub const SLI_CTNS_GSNN_NN: c_uint = 0x0139;
pub const SLI_CTNS_GNN_IP: c_uint = 0x0153;
pub const SLI_CTNS_GIPA_IP: c_uint = 0x0156;
pub const SLI_CTNS_GID_FT: c_uint = 0x0171;
pub const SLI_CTNS_GID_FF: c_uint = 0x01F1;
pub const SLI_CTNS_GID_PT: c_uint = 0x01A1;
pub const SLI_CTNS_RPN_ID: c_uint = 0x0212;
pub const SLI_CTNS_RNN_ID: c_uint = 0x0213;
pub const SLI_CTNS_RCS_ID: c_uint = 0x0214;
pub const SLI_CTNS_RFT_ID: c_uint = 0x0217;
pub const SLI_CTNS_RSPN_ID: c_uint = 0x0218;
pub const SLI_CTNS_RPT_ID: c_uint = 0x021A;
pub const SLI_CTNS_RFF_ID: c_uint = 0x021F;
pub const SLI_CTNS_RIP_NN: c_uint = 0x0235;
pub const SLI_CTNS_RIPA_NN: c_uint = 0x0236;
pub const SLI_CTNS_RSNN_NN: c_uint = 0x0239;
pub const SLI_CTNS_RSPNI_PNI: c_uint = 0x0240;
pub const SLI_CTNS_DA_ID: c_uint = 0x0300;
//
// Port Types
//
pub const SLI_CTPT_N_PORT: c_uint = 0x01;
pub const SLI_CTPT_NL_PORT: c_uint = 0x02;
pub const SLI_CTPT_FNL_PORT: c_uint = 0x03;
pub const SLI_CTPT_IP: c_uint = 0x04;
pub const SLI_CTPT_FCP: c_uint = 0x08;
pub const SLI_CTPT_NVME: c_uint = 0x28;
pub const SLI_CTPT_NX_PORT: c_uint = 0x7F;
pub const SLI_CTPT_F_PORT: c_uint = 0x81;
pub const SLI_CTPT_FL_PORT: c_uint = 0x82;
pub const SLI_CTPT_E_PORT: c_uint = 0x84;
pub const SLI_CT_LAST_ENTRY: c_uint = 0x80000000;
// Fibre Channel Service Parameter definitions

pub const FC_PH3: c_uint = 0x20		/* FC-PH-3 version */;
pub const FF_FRAME_SIZE: c_int = 2048;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_name {

    pub /: *mut *mut uint8_t nameType:4; / FC Word 0, bit 28:31,
    pub bit: *mut *mut uint8_t IEEEextMsn:4; / FC Word 0, bit 24:27,,

    pub bit: *mut *mut uint8_t IEEEextMsn:4; / FC Word 0, bit 24:27,,
    pub /: *mut *mut uint8_t nameType:4; / FC Word 0, bit 28:31,

pub const NAME_IEEE: c_uint = 0x1	/* IEEE name - nameType */;
pub const NAME_IEEE_EXT: c_uint = 0x2	/* IEEE extended name */;
pub const NAME_FC_TYPE: c_uint = 0x3	/* FC native name type */;
pub const NAME_IP_TYPE: c_uint = 0x4	/* IP address */;
pub const NAME_CCITT_TYPE: c_uint = 0xC;
pub const NAME_CCITT_GR_TYPE: c_uint = 0xE;
    pub IEEE: *mut *mut uint8_t IEEEextLsb; / FC Word 0, bit 16:23,,
    pub /: *mut *mut uint8_t IEEE[6]; / FC IEEE address,
    pub s: },
    pub wwn: [u8; 8],
    pub __aligned(4): uint64_t name __packed,
    pub __aligned(4): __be64 wwn_be __packed,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csp {
    pub /: *mut *mut uint8_t fcphHigh; / FC Word 0, byte 0,
    pub fcphLow: u8,
    pub bbCreditMsb: u8,
    pub /: *mut *mut uint8_t bbCreditLsb; / FC Word 0, byte 3,
//
// Word 1 Bit 31 in common service parameter is overloaded.
// Word 1 Bit 31 in FLOGI request is multiple NPort request
// Word 1 Bit 31 in FLOGI response is clean address bit
//

//
// Word 1 Bit 30 in common service parameter is overloaded.
// Word 1 Bit 30 in FLOGI request is Virtual Fabrics
// Word 1 Bit 30 in PLOGI request is random offset
//

//
// Word 1 Bit 29 in common service parameter is overloaded.
// Word 1 Bit 29 in FLOGI response is multiple NPort assignment
// Word 1 Bit 29 in FLOGI/PLOGI request is Valid Vendor Version Level
//

    pub /: *mut *mut uint16_t request_multiple_Nport:1; / FC Word 1, bit 31,
    pub /: *mut *mut uint16_t randomOffset:1; / FC Word 1, bit 30,
    pub /: *mut *mut uint16_t response_multiple_NPort:1; / FC Word 1, bit 29,
    pub /: *mut *mut uint16_t fPort:1; / FC Word 1, bit 28,
    pub /: *mut *mut uint16_t altBbCredit:1; / FC Word 1, bit 27,
    pub /: *mut *mut uint16_t edtovResolution:1; / FC Word 1, bit 26,
    pub /: *mut *mut uint16_t multicast:1; / FC Word 1, bit 25,
    pub /: *mut *mut uint16_t app_hdr_support:1; / FC Word 1, bit 24,
    pub /: *mut *mut uint16_t priority_tagging:1; / FC Word 1, bit 23,
    pub /: *mut *mut uint16_t simplex:1; / FC Word 1, bit 22,
    pub /: *mut *mut uint16_t word1Reserved1:3; / FC Word 1, bit 21:19,
    pub /: *mut *mut uint16_t dhd:1; / FC Word 1, bit 18,
    pub /: *mut *mut uint16_t contIncSeqCnt:1; / FC Word 1, bit 17,
    pub /: *mut *mut uint16_t payloadlength:1; / FC Word 1, bit 16,

    pub /: *mut *mut uint16_t app_hdr_support:1; / FC Word 1, bit 24,
    pub /: *mut *mut uint16_t multicast:1; / FC Word 1, bit 25,
    pub /: *mut *mut uint16_t edtovResolution:1; / FC Word 1, bit 26,
    pub /: *mut *mut uint16_t altBbCredit:1; / FC Word 1, bit 27,
    pub /: *mut *mut uint16_t fPort:1; / FC Word 1, bit 28,
    pub /: *mut *mut uint16_t response_multiple_NPort:1; / FC Word 1, bit 29,
    pub /: *mut *mut uint16_t randomOffset:1; / FC Word 1, bit 30,
    pub /: *mut *mut uint16_t request_multiple_Nport:1; / FC Word 1, bit 31,
    pub /: *mut *mut uint16_t payloadlength:1; / FC Word 1, bit 16,
    pub /: *mut *mut uint16_t contIncSeqCnt:1; / FC Word 1, bit 17,
    pub /: *mut *mut uint16_t dhd:1; / FC Word 1, bit 18,
    pub /: *mut *mut uint16_t word1Reserved1:3; / FC Word 1, bit 21:19,
    pub /: *mut *mut uint16_t simplex:1; / FC Word 1, bit 22,
    pub /: *mut *mut uint16_t priority_tagging:1; / FC Word 1, bit 23,

    pub /: *mut *mut uint8_t bbRcvSizeMsb; / Upper nibble is reserved,
    pub /: *mut *mut uint8_t bbRcvSizeLsb; / FC Word 1, byte 3,
    pub /: *mut *mut uint8_t word2Reserved1; / FC Word 2 byte 0,
    pub /: *mut *mut uint8_t totalConcurrSeq; / FC Word 2 byte 1,
    pub /: *mut *mut uint8_t roByCategoryMsb; / FC Word 2 byte 2,
    pub /: *mut *mut uint8_t roByCategoryLsb; / FC Word 2 byte 3,
    pub nPort: },
    pub /: *mut *mut uint32_t r_a_tov; / R_A_TOV must be in B.E. format,
    pub w2: },
    pub /: *mut *mut uint32_t e_d_tov; / E_D_TOV must be in B.E. format,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct class_parms {

    pub /: *mut *mut uint8_t classValid:1; / FC Word 0, bit 31,
    pub /: *mut *mut uint8_t intermix:1; / FC Word 0, bit 30,
    pub /: *mut *mut uint8_t stackedXparent:1; / FC Word 0, bit 29,
    pub /: *mut *mut uint8_t stackedLockDown:1; / FC Word 0, bit 28,
    pub /: *mut *mut uint8_t seqDelivery:1; / FC Word 0, bit 27,
    pub /: *mut *mut uint8_t word0Reserved1:3; / FC Word 0, bit 24:26,

    pub /: *mut *mut uint8_t word0Reserved1:3; / FC Word 0, bit 24:26,
    pub /: *mut *mut uint8_t seqDelivery:1; / FC Word 0, bit 27,
    pub /: *mut *mut uint8_t stackedLockDown:1; / FC Word 0, bit 28,
    pub /: *mut *mut uint8_t stackedXparent:1; / FC Word 0, bit 29,
    pub /: *mut *mut uint8_t intermix:1; / FC Word 0, bit 30,
    pub /: *mut *mut uint8_t classValid:1; / FC Word 0, bit 31,

    pub /: *mut *mut uint8_t word0Reserved2; / FC Word 0, bit 16:23,

    pub /: *mut *mut uint8_t iCtlXidReAssgn:2; / FC Word 0, Bit 14:15,
    pub /: *mut *mut uint8_t iCtlInitialPa:2; / FC Word 0, bit 12:13,
    pub /: *mut *mut uint8_t iCtlAck0capable:1; / FC Word 0, bit 11,
    pub /: *mut *mut uint8_t iCtlAckNcapable:1; / FC Word 0, bit 10,
    pub /: *mut *mut uint8_t word0Reserved3:2; / FC Word 0, bit 8: 9,

    pub /: *mut *mut uint8_t word0Reserved3:2; / FC Word 0, bit 8: 9,
    pub /: *mut *mut uint8_t iCtlAckNcapable:1; / FC Word 0, bit 10,
    pub /: *mut *mut uint8_t iCtlAck0capable:1; / FC Word 0, bit 11,
    pub /: *mut *mut uint8_t iCtlInitialPa:2; / FC Word 0, bit 12:13,
    pub /: *mut *mut uint8_t iCtlXidReAssgn:2; / FC Word 0, Bit 14:15,

    pub /: *mut *mut uint8_t word0Reserved4; / FC Word 0, bit 0: 7,

    pub /: *mut *mut uint8_t rCtlAck0capable:1; / FC Word 1, bit 31,
    pub /: *mut *mut uint8_t rCtlAckNcapable:1; / FC Word 1, bit 30,
    pub /: *mut *mut uint8_t rCtlXidInterlck:1; / FC Word 1, bit 29,
    pub /: *mut *mut uint8_t rCtlErrorPolicy:2; / FC Word 1, bit 27:28,
    pub /: *mut *mut uint8_t word1Reserved1:1; / FC Word 1, bit 26,
    pub /: *mut *mut uint8_t rCtlCatPerSeq:2; / FC Word 1, bit 24:25,

    pub /: *mut *mut uint8_t rCtlCatPerSeq:2; / FC Word 1, bit 24:25,
    pub /: *mut *mut uint8_t word1Reserved1:1; / FC Word 1, bit 26,
    pub /: *mut *mut uint8_t rCtlErrorPolicy:2; / FC Word 1, bit 27:28,
    pub /: *mut *mut uint8_t rCtlXidInterlck:1; / FC Word 1, bit 29,
    pub /: *mut *mut uint8_t rCtlAckNcapable:1; / FC Word 1, bit 30,
    pub /: *mut *mut uint8_t rCtlAck0capable:1; / FC Word 1, bit 31,

    pub /: *mut *mut uint8_t word1Reserved2; / FC Word 1, bit 16:23,
    pub /: *mut *mut uint8_t rcvDataSizeMsb; / FC Word 1, bit 8:15,
    pub /: *mut *mut uint8_t rcvDataSizeLsb; / FC Word 1, bit 0: 7,
    pub /: *mut *mut uint8_t concurrentSeqMsb; / FC Word 2, bit 24:31,
    pub /: *mut *mut uint8_t concurrentSeqLsb; / FC Word 2, bit 16:23,
    pub /: *mut *mut uint8_t EeCreditSeqMsb; / FC Word 2, bit 8:15,
    pub /: *mut *mut uint8_t EeCreditSeqLsb; / FC Word 2, bit 0: 7,
    pub /: *mut *mut uint8_t openSeqPerXchgMsb; / FC Word 3, bit 24:31,
    pub /: *mut *mut uint8_t openSeqPerXchgLsb; / FC Word 3, bit 16:23,
    pub /: *mut *mut uint8_t word3Reserved1; / Fc Word 3, bit 8:15,
    pub /: *mut *mut uint8_t word3Reserved2; / Fc Word 3, bit 0: 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aux_parm_flags {
    AUX_PARM_PNI_VALID = 0x20,	/* FC Word 0, bit 29 */
    AUX_PARM_DATA_VALID = 0x40,	/* FC Word 0, bit 30 */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aux_parm {
    pub /: *mut *mut u8 flags; / FC Word 0, bit 31:24,
    pub /: *mut *mut u8 ext_feat[3]; / FC Word 0, bit 23:0,
    pub /: *mut *mut __be64 pni; / FC Word 1 and 2, platform name identifier,
    pub /: *mut *mut __be16 rsvd; / FC Word 3, bit 31:16,
    pub /: *mut *mut __be16 npiv_cnt; / FC Word 3, bit 15:0,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct serv_parm {
    pub cmn: csp,
    pub portName: lpfc_name,
    pub nodeName: lpfc_name,
    pub cls1: class_parms,
    pub cls2: class_parms,
    pub cls3: class_parms,
    pub aux: aux_parm,
    pub vendorVersion: [u8; 16],
    pub vid: u32,
pub const LPFC_VV_EMLX_ID: c_uint = 0x454d4c58	/* EMLX */;
    pub flags: u32,
pub const LPFC_VV_SUPPRESS_RSP: c_int = 1;
    pub vv: },
    pub un: },
}

//
// Virtual Fabric Tagging Header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_vft_header {
    pub word0: u32,
pub const fc_vft_hdr_r_ctl_SHIFT: c_int = 24;
pub const fc_vft_hdr_r_ctl_MASK: c_uint = 0xFF;

pub const fc_vft_hdr_ver_SHIFT: c_int = 22;
pub const fc_vft_hdr_ver_MASK: c_uint = 0x3;

pub const fc_vft_hdr_type_SHIFT: c_int = 18;
pub const fc_vft_hdr_type_MASK: c_uint = 0xF;

pub const fc_vft_hdr_e_SHIFT: c_int = 16;
pub const fc_vft_hdr_e_MASK: c_uint = 0x1;

pub const fc_vft_hdr_priority_SHIFT: c_int = 13;
pub const fc_vft_hdr_priority_MASK: c_uint = 0x7;

pub const fc_vft_hdr_vf_id_SHIFT: c_int = 1;
pub const fc_vft_hdr_vf_id_MASK: c_uint = 0xFFF;

    pub word1: u32,
pub const fc_vft_hdr_hopct_SHIFT: c_int = 24;
pub const fc_vft_hdr_hopct_MASK: c_uint = 0xFF;

}

//
// Application Header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_app_header {
    pub dst_app_id: u32,
    pub src_app_id: u32,
pub const LOOPBACK_SRC_APPID: c_uint = 0x4321;
    pub word2: u32,
    pub word3: u32,
}

//
// dfctl optional header definition
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lpfc_fc_dfctl {
    LPFC_FC_NO_DEVICE_HEADER,
    LPFC_FC_16B_DEVICE_HEADER,
    LPFC_FC_32B_DEVICE_HEADER,
    LPFC_FC_64B_DEVICE_HEADER,
}

//
// Extended Link Service LS_COMMAND codes (Payload Word 0)
//

pub const ELS_CMD_MASK: c_uint = 0xffff0000;
pub const ELS_RSP_MASK: c_uint = 0xff000000;
pub const ELS_CMD_LS_RJT: c_uint = 0x01000000;
pub const ELS_CMD_ACC: c_uint = 0x02000000;
pub const ELS_CMD_PLOGI: c_uint = 0x03000000;
pub const ELS_CMD_FLOGI: c_uint = 0x04000000;
pub const ELS_CMD_LOGO: c_uint = 0x05000000;
pub const ELS_CMD_ABTX: c_uint = 0x06000000;
pub const ELS_CMD_RCS: c_uint = 0x07000000;
pub const ELS_CMD_RES: c_uint = 0x08000000;
pub const ELS_CMD_RSS: c_uint = 0x09000000;
pub const ELS_CMD_RSI: c_uint = 0x0A000000;
pub const ELS_CMD_ESTS: c_uint = 0x0B000000;
pub const ELS_CMD_ESTC: c_uint = 0x0C000000;
pub const ELS_CMD_ADVC: c_uint = 0x0D000000;
pub const ELS_CMD_RTV: c_uint = 0x0E000000;
pub const ELS_CMD_RLS: c_uint = 0x0F000000;
pub const ELS_CMD_ECHO: c_uint = 0x10000000;
pub const ELS_CMD_TEST: c_uint = 0x11000000;
pub const ELS_CMD_RRQ: c_uint = 0x12000000;
pub const ELS_CMD_REC: c_uint = 0x13000000;
pub const ELS_CMD_RDP: c_uint = 0x18000000;
pub const ELS_CMD_RDF: c_uint = 0x19000000;
pub const ELS_CMD_PRLI: c_uint = 0x20100014;
pub const ELS_CMD_NVMEPRLI: c_uint = 0x20140018;
pub const ELS_CMD_PRLO: c_uint = 0x21100014;
pub const ELS_CMD_PRLO_ACC: c_uint = 0x02100014;
pub const ELS_CMD_PDISC: c_uint = 0x50000000;
pub const ELS_CMD_FDISC: c_uint = 0x51000000;
pub const ELS_CMD_ADISC: c_uint = 0x52000000;
pub const ELS_CMD_FARP: c_uint = 0x54000000;
pub const ELS_CMD_FARPR: c_uint = 0x55000000;
pub const ELS_CMD_RPL: c_uint = 0x57000000;
pub const ELS_CMD_FAN: c_uint = 0x60000000;
pub const ELS_CMD_RSCN: c_uint = 0x61040000;
pub const ELS_CMD_RSCN_XMT: c_uint = 0x61040008;
pub const ELS_CMD_SCR: c_uint = 0x62000000;
pub const ELS_CMD_RNID: c_uint = 0x78000000;
pub const ELS_CMD_LIRR: c_uint = 0x7A000000;
pub const ELS_CMD_LCB: c_uint = 0x81000000;
pub const ELS_CMD_FPIN: c_uint = 0x16000000;
pub const ELS_CMD_EDC: c_uint = 0x17000000;
pub const ELS_CMD_QFPA: c_uint = 0xB0000000;
pub const ELS_CMD_UVEM: c_uint = 0xB1000000;

pub const ELS_CMD_MASK: c_uint = 0xffff;
pub const ELS_RSP_MASK: c_uint = 0xff;
pub const ELS_CMD_LS_RJT: c_uint = 0x01;
pub const ELS_CMD_ACC: c_uint = 0x02;
pub const ELS_CMD_PLOGI: c_uint = 0x03;
pub const ELS_CMD_FLOGI: c_uint = 0x04;
pub const ELS_CMD_LOGO: c_uint = 0x05;
pub const ELS_CMD_ABTX: c_uint = 0x06;
pub const ELS_CMD_RCS: c_uint = 0x07;
pub const ELS_CMD_RES: c_uint = 0x08;
pub const ELS_CMD_RSS: c_uint = 0x09;
pub const ELS_CMD_RSI: c_uint = 0x0A;
pub const ELS_CMD_ESTS: c_uint = 0x0B;
pub const ELS_CMD_ESTC: c_uint = 0x0C;
pub const ELS_CMD_ADVC: c_uint = 0x0D;
pub const ELS_CMD_RTV: c_uint = 0x0E;
pub const ELS_CMD_RLS: c_uint = 0x0F;
pub const ELS_CMD_ECHO: c_uint = 0x10;
pub const ELS_CMD_TEST: c_uint = 0x11;
pub const ELS_CMD_RRQ: c_uint = 0x12;
pub const ELS_CMD_REC: c_uint = 0x13;
pub const ELS_CMD_RDP: c_uint = 0x18;
pub const ELS_CMD_RDF: c_uint = 0x19;
pub const ELS_CMD_PRLI: c_uint = 0x14001020;
pub const ELS_CMD_NVMEPRLI: c_uint = 0x18001420;
pub const ELS_CMD_PRLO: c_uint = 0x14001021;
pub const ELS_CMD_PRLO_ACC: c_uint = 0x14001002;
pub const ELS_CMD_PDISC: c_uint = 0x50;
pub const ELS_CMD_FDISC: c_uint = 0x51;
pub const ELS_CMD_ADISC: c_uint = 0x52;
pub const ELS_CMD_FARP: c_uint = 0x54;
pub const ELS_CMD_FARPR: c_uint = 0x55;
pub const ELS_CMD_RPL: c_uint = 0x57;
pub const ELS_CMD_FAN: c_uint = 0x60;
pub const ELS_CMD_RSCN: c_uint = 0x0461;
pub const ELS_CMD_RSCN_XMT: c_uint = 0x08000461;
pub const ELS_CMD_SCR: c_uint = 0x62;
pub const ELS_CMD_RNID: c_uint = 0x78;
pub const ELS_CMD_LIRR: c_uint = 0x7A;
pub const ELS_CMD_LCB: c_uint = 0x81;

pub const ELS_CMD_QFPA: c_uint = 0xB0;
pub const ELS_CMD_UVEM: c_uint = 0xB1;

//
// LS_RJT Payload Definition
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ls_rjt {
    pub ls_rjt_error_be: __be32,
    pub lsRjtError: u32,
    pub /: *mut *mut uint8_t lsRjtRsvd0; / FC Word 0, bit 24:31,
    pub /: *mut *mut uint8_t lsRjtRsnCode; / FC Word 0, bit 16:23,
// LS_RJT reason codes
pub const LSRJT_INVALID_CMD: c_uint = 0x01;
pub const LSRJT_LOGICAL_ERR: c_uint = 0x03;
pub const LSRJT_LOGICAL_BSY: c_uint = 0x05;
pub const LSRJT_PROTOCOL_ERR: c_uint = 0x07;
pub const LSRJT_UNABLE_TPC: c_uint = 0x09	/* Unable to perform command */;
pub const LSRJT_CMD_UNSUPPORTED: c_uint = 0x0B;
pub const LSRJT_VENDOR_UNIQUE: c_uint = 0xFF	/* See Byte 3 */;
    pub /: *mut *mut uint8_t lsRjtRsnCodeExp; / FC Word 0, bit 8:15,
// LS_RJT reason explanation
pub const LSEXP_NOTHING_MORE: c_uint = 0x00;
pub const LSEXP_SPARM_OPTIONS: c_uint = 0x01;
pub const LSEXP_SPARM_ICTL: c_uint = 0x03;
pub const LSEXP_SPARM_RCTL: c_uint = 0x05;
pub const LSEXP_SPARM_RCV_SIZE: c_uint = 0x07;
pub const LSEXP_SPARM_CONCUR_SEQ: c_uint = 0x09;
pub const LSEXP_SPARM_CREDIT: c_uint = 0x0B;
pub const LSEXP_INVALID_PNAME: c_uint = 0x0D;
pub const LSEXP_INVALID_NNAME: c_uint = 0x0E;
pub const LSEXP_INVALID_CSP: c_uint = 0x0F;
pub const LSEXP_INVALID_ASSOC_HDR: c_uint = 0x11;
pub const LSEXP_ASSOC_HDR_REQ: c_uint = 0x13;
pub const LSEXP_INVALID_O_SID: c_uint = 0x15;
pub const LSEXP_INVALID_OX_RX: c_uint = 0x17;
pub const LSEXP_CMD_IN_PROGRESS: c_uint = 0x19;
pub const LSEXP_PORT_LOGIN_REQ: c_uint = 0x1E;
pub const LSEXP_INVALID_NPORT_ID: c_uint = 0x1F;
pub const LSEXP_INVALID_SEQ_ID: c_uint = 0x21;
pub const LSEXP_INVALID_XCHG: c_uint = 0x23;
pub const LSEXP_INACTIVE_XCHG: c_uint = 0x25;
pub const LSEXP_RQ_REQUIRED: c_uint = 0x27;
pub const LSEXP_OUT_OF_RESOURCE: c_uint = 0x29;
pub const LSEXP_CANT_GIVE_DATA: c_uint = 0x2A;
pub const LSEXP_REQ_UNSUPPORTED: c_uint = 0x2C;
pub const LSEXP_AUTH_REQ: c_uint = 0x48;
pub const LSEXP_NO_RSRC_ASSIGN: c_uint = 0x52;
    pub /: *mut *mut uint8_t vendorUnique; / FC Word 0, bit 0: 7,
    pub b: },
    pub un: },
}

//
// N_Port Login (FLOGO/PLOGO Request) Payload Definition
//
// FCP Login (PRLI Request / ACC) Payload Definition
//
pub const PRLX_PAGE_LEN: c_uint = 0x10;
pub const TPRLO_PAGE_LEN: c_uint = 0x14;
pub const PRLI_FCP_TYPE: c_uint = 0x08;
pub const PRLI_NVME_TYPE: c_uint = 0x28;

// ACC = imagePairEstablished

// ACC = imagePairEstablished

pub const PRLI_REQ_EXECUTED: c_uint = 0x1	/* acceptRspCode */;
pub const PRLI_NO_RESOURCES: c_uint = 0x2;
pub const PRLI_INIT_INCOMPLETE: c_uint = 0x3;
pub const PRLI_NO_SUCH_PA: c_uint = 0x4;
pub const PRLI_PREDEF_CONFIG: c_uint = 0x5;
pub const PRLI_PARTIAL_SUCCESS: c_uint = 0x6;
pub const PRLI_INVALID_PAGE_CNT: c_uint = 0x7;
pub const PRLI_INV_SRV_PARM: c_uint = 0x8;

//
// FCP Logout (PRLO Request / ACC) Payload Definition
//
pub const PRLO_FCP_TYPE: c_uint = 0x08;

pub const PRLO_REQ_EXECUTED: c_uint = 0x1	/* acceptRspCode */;
pub const PRLO_NO_SUCH_IMAGE: c_uint = 0x4;
pub const PRLO_INVALID_PAGE_CNT: c_uint = 0x7;

pub const FARP_MATCH_PORT: c_uint = 0x1	/* Match on Responder Port Name */;
pub const FARP_MATCH_NODE: c_uint = 0x2	/* Match on Responder Node Name */;
pub const FARP_MATCH_IP: c_uint = 0x4	/* Match on IP address, not supported */;
pub const FARP_MATCH_IPV4: c_uint = 0x5	/* Match on IPV4 address, not;
pub const FARP_MATCH_IPV6: c_uint = 0x6	/* Match on IPV6 address, not;
pub const FARP_REQUEST_PLOGI: c_uint = 0x1	/* Request for PLOGI */;
pub const FARP_REQUEST_FARPR: c_uint = 0x2	/* Request for FARP Response */;
pub const SCR_FUNC_FABRIC: c_uint = 0x01;
pub const SCR_FUNC_NPORT: c_uint = 0x02;
pub const SCR_FUNC_FULL: c_uint = 0x03;
pub const SCR_CLEAR: c_uint = 0xff;
pub const RNID_HBA: c_uint = 0x7;
pub const RNID_HOST: c_uint = 0xa;
pub const RNID_DRIVER: c_uint = 0xd;
pub const RNID_IPV4: c_uint = 0x1;
pub const RNID_IPV6: c_uint = 0x2;
pub const RNID_TD_SUPPORT: c_uint = 0x1;
pub const RNID_LP_VALID: c_uint = 0x2;
pub const RNID_TOPOLOGY_DISC: c_uint = 0xdf;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RLS {
    pub rls: u32,
pub const rls_rsvd_SHIFT: c_int = 24;
pub const rls_rsvd_MASK: c_uint = 0x000000ff;

pub const rls_did_SHIFT: c_int = 0;
pub const rls_did_MASK: c_uint = 0x00ffffff;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RLS_RSP {
    pub linkFailureCnt: u32,
    pub lossSyncCnt: u32,
    pub lossSignalCnt: u32,
    pub primSeqErrCnt: u32,
    pub invalidXmitWord: u32,
    pub crcCnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RRQ {
    pub rrq: u32,
pub const rrq_rsvd_SHIFT: c_int = 24;
pub const rrq_rsvd_MASK: c_uint = 0x000000ff;

pub const rrq_did_SHIFT: c_int = 0;
pub const rrq_did_MASK: c_uint = 0x00ffffff;

    pub rrq_exchg: u32,
pub const rrq_oxid_SHIFT: c_int = 16;
pub const rrq_oxid_MASK: c_uint = 0xffff;

pub const rrq_rxid_SHIFT: c_int = 0;
pub const rrq_rxid_MASK: c_uint = 0xffff;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RTV_RSP {
    pub ratov: u32,
    pub edtov: u32,
    pub qtov: u32,
pub const qtov_rsvd0_SHIFT: c_int = 28;
pub const qtov_rsvd0_MASK: c_uint = 0x0000000f;

pub const qtov_edtovres_SHIFT: c_int = 27;
pub const qtov_edtovres_MASK: c_uint = 0x00000001;

pub const qtov__rsvd1_SHIFT: c_int = 19;
pub const qtov_rsvd1_MASK: c_uint = 0x0000003f;

pub const qtov_rttov_SHIFT: c_int = 18;
pub const qtov_rttov_MASK: c_uint = 0x00000001;

pub const qtov_rsvd2_SHIFT: c_int = 0;
pub const qtov_rsvd2_MASK: c_uint = 0x0003ffff;

}

// This is used for RSCN command

pub const RSCN_ADDRESS_FORMAT_PORT: c_uint = 0x0;
pub const RSCN_ADDRESS_FORMAT_AREA: c_uint = 0x1;
pub const RSCN_ADDRESS_FORMAT_DOMAIN: c_uint = 0x2;
pub const RSCN_ADDRESS_FORMAT_FABRIC: c_uint = 0x3;
pub const RSCN_ADDRESS_FORMAT_MASK: c_uint = 0x3;
//
// Structure to define all ELS Payload types
//
// Link Cable Beacon (LCB) ELS Frame
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_lcb_request_frame {
    pub /: *mut *mut uint32_t lcb_command; / ELS command opcode (0x81),
    pub /: *mut *mut uint8_t lcb_sub_command;/ LCB Payload Word 1, bit 24:31,
pub const LPFC_LCB_ON: c_uint = 0x1;
pub const LPFC_LCB_OFF: c_uint = 0x2;
    pub reserved: [u8; 2],
    pub /: *mut *mut uint8_t capability; / LCB Payload Word 1, bit 0:7,
    pub /: *mut *mut uint8_t lcb_type; / LCB Payload Word 2, bit 24:31,
pub const LPFC_LCB_GREEN: c_uint = 0x1;
pub const LPFC_LCB_AMBER: c_uint = 0x2;
    pub /: *mut *mut uint8_t lcb_frequency; / LCB Payload Word 2, bit 16:23,
pub const LCB_CAPABILITY_DURATION: c_int = 1;
pub const BEACON_VERSION_V1: c_int = 1;
pub const BEACON_VERSION_V0: c_int = 0;
    pub /: *mut *mut uint16_t lcb_duration; / LCB Payload Word 2, bit 15:0,
}

//
// Link Cable Beacon (LCB) ELS Response Frame
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_lcb_res_frame {
    pub /: *mut *mut uint32_t lcb_ls_acc; / Acceptance of LCB request (0x02),
    pub /: *mut *mut uint8_t lcb_sub_command;/ LCB Payload Word 1, bit 24:31,
    pub reserved: [u8; 2],
    pub /: *mut *mut uint8_t capability; / LCB Payload Word 1, bit 0:7,
    pub /: *mut *mut uint8_t lcb_type; / LCB Payload Word 2, bit 24:31,
    pub /: *mut *mut uint8_t lcb_frequency; / LCB Payload Word 2, bit 16:23,
    pub /: *mut *mut uint16_t lcb_duration; / LCB Payload Word 2, bit 15:0,
}

//
// Read Diagnostic Parameters (RDP) ELS frame.
//
pub const SFF_PG0_IDENT_SFP: c_uint = 0x3;
pub const SFP_FLAG_PT_OPTICAL: c_uint = 0x0;
pub const SFP_FLAG_PT_SWLASER: c_uint = 0x01;
pub const SFP_FLAG_PT_LWLASER_LC1310: c_uint = 0x02;
pub const SFP_FLAG_PT_LWLASER_LL1550: c_uint = 0x03;
pub const SFP_FLAG_PT_MASK: c_uint = 0x0F;
pub const SFP_FLAG_PT_SHIFT: c_int = 0;
pub const SFP_FLAG_IS_OPTICAL_PORT: c_uint = 0x01;
pub const SFP_FLAG_IS_OPTICAL_MASK: c_uint = 0x010;
pub const SFP_FLAG_IS_OPTICAL_SHIFT: c_int = 4;
pub const SFP_FLAG_IS_DESC_VALID: c_uint = 0x01;
pub const SFP_FLAG_IS_DESC_VALID_MASK: c_uint = 0x020;
pub const SFP_FLAG_IS_DESC_VALID_SHIFT: c_int = 5;
pub const SFP_FLAG_CT_UNKNOWN: c_uint = 0x0;
pub const SFP_FLAG_CT_SFP_PLUS: c_uint = 0x01;
pub const SFP_FLAG_CT_MASK: c_uint = 0x3C;
pub const SFP_FLAG_CT_SHIFT: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rdp_port_name_info {
    pub wwnn: [u8; 8],
    pub wwpn: [u8; 8],
}

//
// Link Error Status Block Structure (FC-FS-3) for RDP
// This similar to RPS ELS
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_link_status {
    pub link_failure_cnt: u32,
    pub loss_of_synch_cnt: u32,
    pub loss_of_signal_cnt: u32,
    pub primitive_seq_proto_err: u32,
    pub invalid_trans_word: u32,
    pub invalid_crc_cnt: u32,
}

pub const RDP_PORT_NAMES_DESC_TAG: c_uint = 0x00010003;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rdp_port_name_desc {
    pub /: *mut *mut uint32_t tag; / 0001 0003h,
    pub /: *mut *mut uint32_t length; / set to size of payload struct,
    pub port_names: fc_rdp_port_name_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rdp_fec_info {
    pub CorrectedBlocks: u32,
    pub UncorrectableBlocks: u32,
}

pub const RDP_FEC_DESC_TAG: c_uint = 0x00010005;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_fec_rdp_desc {
    pub tag: u32,
    pub length: u32,
    pub info: fc_rdp_fec_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rdp_link_error_status_payload_info {
    pub /: *mut *mut fc_link_status link_status; / 24 bytes,
    pub /: *mut *mut uint32_t port_type; / bits 31-30 only,
}

pub const RDP_LINK_ERROR_STATUS_DESC_TAG: c_uint = 0x00010002;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rdp_link_error_status_desc {
    pub /: *mut *mut uint32_t tag; / 0001 0002h,
    pub /: *mut *mut uint32_t length; / set to size of payload struct,
    pub info: fc_rdp_link_error_status_payload_info,
}

pub const VN_PT_PHY_UNKNOWN: c_uint = 0x00;
pub const VN_PT_PHY_PF_PORT: c_uint = 0x01;
pub const VN_PT_PHY_ETH_MAC: c_uint = 0x10;
pub const VN_PT_PHY_SHIFT: c_int = 30;
pub const RDP_PS_1GB: c_uint = 0x8000;
pub const RDP_PS_2GB: c_uint = 0x4000;
pub const RDP_PS_4GB: c_uint = 0x2000;
pub const RDP_PS_10GB: c_uint = 0x1000;
pub const RDP_PS_8GB: c_uint = 0x0800;
pub const RDP_PS_16GB: c_uint = 0x0400;
pub const RDP_PS_32GB: c_uint = 0x0200;
pub const RDP_PS_64GB: c_uint = 0x0100;
pub const RDP_PS_128GB: c_uint = 0x0080;
pub const RDP_PS_256GB: c_uint = 0x0040;
pub const RDP_CAP_USER_CONFIGURED: c_uint = 0x0002;
pub const RDP_CAP_UNKNOWN: c_uint = 0x0001;
pub const RDP_PS_UNKNOWN: c_uint = 0x0002;
pub const RDP_PS_NOT_ESTABLISHED: c_uint = 0x0001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rdp_port_speed {
    pub capabilities: u16,
    pub speed: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rdp_port_speed_info {
    pub port_speed: fc_rdp_port_speed,
}

pub const RDP_PORT_SPEED_DESC_TAG: c_uint = 0x00010001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rdp_port_speed_desc {
    pub /: *mut *mut uint32_t tag; / 00010001h,
    pub /: *mut *mut uint32_t length; / set to size of payload struct,
    pub info: fc_rdp_port_speed_info,
}

pub const RDP_NPORT_ID_SIZE: c_int = 4;
pub const RDP_N_PORT_DESC_TAG: c_uint = 0x00000003;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rdp_nport_desc {
    pub /: *mut *mut uint32_t tag; / 0000 0003h, big endian,
    pub /: *mut *mut uint32_t length; / size of RDP_N_PORT_ID struct,
    pub 12: uint32_t nport_id :,
    pub 8: uint32_t reserved :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rdp_link_service_info {
    pub value.*/: *mut *mut uint32_t els_req; / Request payload word 0,
}

pub const RDP_LINK_SERVICE_DESC_TAG: c_uint = 0x00000001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rdp_link_service_desc {
    pub /: *mut *mut uint32_t tag; / Descriptor tag 1,
    pub /: *mut *mut uint32_t length; / set to size of payload struct.,
    pub payload: fc_rdp_link_service_info,
// must be ELS req Word 0(0x18)
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rdp_sfp_info {
    pub temperature: u16,
    pub vcc: u16,
    pub tx_bias: u16,
    pub tx_power: u16,
    pub rx_power: u16,
    pub flags: u16,
}

pub const RDP_SFP_DESC_TAG: c_uint = 0x00010000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rdp_sfp_desc {
    pub tag: u32,
    pub /: *mut *mut uint32_t length; / set to size of sfp_info struct,
    pub sfp_info: fc_rdp_sfp_info,
}

// Buffer Credit Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rdp_bbc_info {
    pub /: *mut *mut uint32_t port_bbc; / FC_Port buffer-to-buffer credit,
    pub attached_port_bbc: u32,
    pub /: *mut *mut uint32_t rtt; / Round trip time,
}

pub const RDP_BBC_DESC_TAG: c_uint = 0x00010006;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rdp_bbc_desc {
    pub tag: u32,
    pub length: u32,
    pub bbc_info: fc_rdp_bbc_info,
}

// Optical Element Type Transgression Flags
pub const RDP_OET_LOW_WARNING: c_uint = 0x1;
pub const RDP_OET_HIGH_WARNING: c_uint = 0x2;
pub const RDP_OET_LOW_ALARM: c_uint = 0x4;
pub const RDP_OET_HIGH_ALARM: c_uint = 0x8;
pub const RDP_OED_TEMPERATURE: c_uint = 0x1;
pub const RDP_OED_VOLTAGE: c_uint = 0x2;
pub const RDP_OED_TXBIAS: c_uint = 0x3;
pub const RDP_OED_TXPOWER: c_uint = 0x4;
pub const RDP_OED_RXPOWER: c_uint = 0x5;
pub const RDP_OED_TYPE_SHIFT: c_int = 28;
// Optical Element Data descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rdp_oed_info {
    pub hi_alarm: u16,
    pub lo_alarm: u16,
    pub hi_warning: u16,
    pub lo_warning: u16,
    pub function_flags: u32,
}

pub const RDP_OED_DESC_TAG: c_uint = 0x00010007;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rdp_oed_sfp_desc {
    pub tag: u32,
    pub length: u32,
    pub oed_info: fc_rdp_oed_info,
}

// Optical Product Data descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rdp_opd_sfp_info {
    pub vendor_name: [u8; 16],
    pub model_number: [u8; 16],
    pub serial_number: [u8; 16],
    pub revision: [u8; 4],
    pub date: [u8; 8],
}

pub const RDP_OPD_DESC_TAG: c_uint = 0x00010008;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rdp_opd_sfp_desc {
    pub tag: u32,
    pub length: u32,
    pub opd_info: fc_rdp_opd_sfp_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rdp_req_frame {
    pub (0x18)*/: *mut *mut uint32_t rdp_command; / ELS command opcode,
    pub /: *mut *mut uint32_t rdp_des_length; / RDP Payload Word 1,
    pub /: *mut *mut fc_rdp_nport_desc nport_id_desc; / RDP Payload Word 2 - 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rdp_res_frame {
    pub /: *mut *mut uint32_t reply_sequence; / FC word0 LS_ACC or LS_RJT,
    pub /: *mut *mut uint32_t length; / FC Word 1,
    pub /: *mut *mut fc_rdp_link_service_desc link_service_desc; / Word 2 -4,
    pub /: *mut *mut fc_rdp_sfp_desc sfp_desc; / Word 5 -9,
    pub /: *mut *mut fc_rdp_port_speed_desc portspeed_desc; / Word 10 -12,
    pub /: *mut *mut fc_rdp_link_error_status_desc link_error_desc; / Word 13 -21,
    pub /: *mut *mut fc_rdp_port_name_desc diag_port_names_desc; / Word 22 -27,
    pub /: *mut *mut fc_rdp_port_name_desc attached_port_names_desc;/ Word 28 -33,
    pub 34-37*/: *mut *mut fc_fec_rdp_desc fec_desc; / FC word,
    pub 38-42*/: *mut *mut fc_rdp_bbc_desc bbc_desc; / FC Word,
    pub 43-47*/: *mut *mut fc_rdp_oed_sfp_desc oed_temp_desc; / FC Word,
    pub 48-52*/: *mut *mut fc_rdp_oed_sfp_desc oed_voltage_desc; / FC word,
    pub 53-57*/: *mut *mut fc_rdp_oed_sfp_desc oed_txbias_desc; / FC word,
    pub 58-62*/: *mut *mut fc_rdp_oed_sfp_desc oed_txpower_desc; / FC word,
    pub 63-67*/: *mut *mut fc_rdp_oed_sfp_desc oed_rxpower_desc; / FC word,
    pub 68-84*/: *mut *mut fc_rdp_opd_sfp_desc opd_desc; / FC word,
}

// UVEM
pub const LPFC_UVEM_SIZE: c_int = 60;
pub const LPFC_UVEM_VEM_ID_DESC_SIZE: c_int = 16;
pub const LPFC_UVEM_VE_MAP_DESC_SIZE: c_int = 20;
pub const VEM_ID_DESC_TAG: c_uint = 0x0001000A;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_vem_id_desc {
    pub tag: u32,
    pub length: u32,
    pub vem_id: [u8; 16],
}

pub const LPFC_QFPA_SIZE: c_int = 4;
pub const INSTANTIATED_VE_DESC_TAG: c_uint = 0x0001000B;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct instantiated_ve_desc {
    pub tag: u32,
    pub length: u32,
    pub global_vem_id: [u8; 16],
    pub word6: u32,
pub const lpfc_instantiated_local_id_SHIFT: c_int = 0;
pub const lpfc_instantiated_local_id_MASK: c_uint = 0x000000ff;

pub const lpfc_instantiated_nport_id_SHIFT: c_int = 8;
pub const lpfc_instantiated_nport_id_MASK: c_uint = 0x00ffffff;

}

pub const DEINSTANTIATED_VE_DESC_TAG: c_uint = 0x0001000C;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct deinstantiated_ve_desc {
    pub tag: u32,
    pub length: u32,
    pub global_vem_id: [u8; 16],
    pub word6: u32,
pub const lpfc_deinstantiated_nport_id_SHIFT: c_int = 0;
pub const lpfc_deinstantiated_nport_id_MASK: c_uint = 0x000000ff;

pub const lpfc_deinstantiated_local_id_SHIFT: c_int = 24;
pub const lpfc_deinstantiated_local_id_MASK: c_uint = 0x00ffffff;

}

// Query Fabric Priority Allocation Response
pub const LPFC_PRIORITY_RANGE_DESC_SIZE: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct priority_range_desc {
    pub tag: u32,
    pub length: u32,
    pub lo_range: u8,
    pub hi_range: u8,
    pub qos_priority: u8,
    pub local_ve_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_qfpa_res {
    pub /: *mut *mut uint32_t reply_sequence; / LS_ACC or LS_RJT,
    pub /: *mut *mut uint32_t length; / FC Word 1,
    pub desc: [priority_range_desc; 1],
}

// Application Server command code
// VMID
pub const SLI_CT_APP_SEV_Subtypes: c_uint = 0x20	/* Application Server subtype */;
pub const SLI_CTAS_GAPPIA_ENT: c_uint = 0x0100	/* Get Application Identifier */;
pub const SLI_CTAS_GALLAPPIA: c_uint = 0x0101	/* Get All Application Identifier */;
pub const SLI_CTAS_GALLAPPIA_ID: c_uint = 0x0102	/* Get All Application Identifier */;
// for Nport
pub const SLI_CTAS_GAPPIA_IDAPP: c_uint = 0x0103	/* Get Application Identifier */;
// for Nport
pub const SLI_CTAS_RAPP_IDENT: c_uint = 0x0200	/* Register Application Identifier */;
pub const SLI_CTAS_DAPP_IDENT: c_uint = 0x0300	/* Deregister Application */;
// Identifier
pub const SLI_CTAS_DALLAPP_ID: c_uint = 0x0301	/* Deregister All Application */;
// Identifier
#[repr(C)]
#[derive(Copy, Clone)]
pub struct entity_id_object {
    pub entity_id_len: u8,
    pub /: *mut *mut uint8_t entity_id[255]; / VM UUID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct app_id_object {
    pub port_id: __be32,
    pub app_id: __be32,
    pub obj: entity_id_object,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_vmid_rapp_ident_list {
    pub no_of_objects: __be32,
    pub obj: [entity_id_object; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_vmid_dapp_ident_list {
    pub no_of_objects: __be32,
    pub obj: [entity_id_object; ],
}

pub const GALLAPPIA_ID_LAST: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_vmid_gallapp_ident_list {
    pub control: u8,
    pub reserved: [u8; 3],
    pub app_id: app_id_object,
}

// FDMI
// lpfc_sli_ct_request defines the CT_IU preamble for FDMI commands
pub const SLI_CT_FDMI_Subtypes: c_uint = 0x10	/* Management Service Subtype */;
// Definitions for HBA / Port attribute entries
// Attribute Entry Structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_fdmi_attr_u32 {
    pub type: __be16,
    pub len: __be16,
    pub value_u32: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_fdmi_attr_wwn {
    pub type: __be16,
    pub len: __be16,
// Keep as u8[8] instead of __be64 to avoid accidental zero padding
// by compiler
//
    pub name: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_fdmi_attr_fullwwn {
    pub type: __be16,
    pub len: __be16,
// Keep as u8[8] instead of __be64 to avoid accidental zero padding
// by compiler
//
    pub nname: [u8; 8],
    pub pname: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_fdmi_attr_fc4types {
    pub type: __be16,
    pub len: __be16,
    pub value_types: [u8; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_fdmi_attr_string {
    pub type: __be16,
    pub len: __be16,
    pub value_string: [c_char; 256],
}

// Maximum FDMI attribute length is Type+Len (4 bytes) + 256 byte string

//
// HBA Attribute Block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_fdmi_attr_block {
    pub /: *mut *mut uint32_t EntryCnt; / Number of HBA attribute entries,
// Variable Length Attribute Entry TLV's follow
}

//
// Port Entry
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_fdmi_port_entry {
    pub PortName: lpfc_name,
}

//
// HBA Identifier
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_fdmi_hba_ident {
    pub PortName: lpfc_name,
}

//
// Registered Port List Format
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_fdmi_reg_port_list {
    pub EntryCnt: __be32,
    pub pe: lpfc_fdmi_port_entry,
}

//
// Register HBA(RHBA)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_fdmi_reg_hba {
    pub hi: lpfc_fdmi_hba_ident,
    pub rpl: lpfc_fdmi_reg_port_list,
}

// MI MIB
pub const SLI_CT_MIB_Subtypes: c_uint = 0x11;
//
// Register HBA Attributes (RHAT)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_fdmi_reg_hbaattr {
    pub HBA_PortName: lpfc_name,
    pub ab: lpfc_fdmi_attr_block,
}

//
// Register Port Attributes (RPA)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_fdmi_reg_portattr {
    pub PortName: lpfc_name,
    pub ab: lpfc_fdmi_attr_block,
}

//
// HBA MAnagement Operations Command Codes
//
pub const SLI_MGMT_GRHL: c_uint = 0x100	/* Get registered HBA list */;
pub const SLI_MGMT_GHAT: c_uint = 0x101	/* Get HBA attributes */;
pub const SLI_MGMT_GRPL: c_uint = 0x102	/* Get registered Port list */;
pub const SLI_MGMT_GPAT: c_uint = 0x110	/* Get Port attributes */;
pub const SLI_MGMT_GPAS: c_uint = 0x120	/* Get Port Statistics */;
pub const SLI_MGMT_RHBA: c_uint = 0x200	/* Register HBA */;
pub const SLI_MGMT_RHAT: c_uint = 0x201	/* Register HBA attributes */;
pub const SLI_MGMT_RPRT: c_uint = 0x210	/* Register Port */;
pub const SLI_MGMT_RPA: c_uint = 0x211	/* Register Port attributes */;
pub const SLI_MGMT_DHBA: c_uint = 0x300	/* De-register HBA */;
pub const SLI_MGMT_DHAT: c_uint = 0x301	/* De-register HBA attributes */;
pub const SLI_MGMT_DPRT: c_uint = 0x310	/* De-register Port */;
pub const SLI_MGMT_DPA: c_uint = 0x311	/* De-register Port attributes */;

//
// HBA Attribute Types
//
pub const RHBA_NODENAME: c_uint = 0x1 /* 8 byte WWNN */;
pub const RHBA_MANUFACTURER: c_uint = 0x2 /* 4 to 64 byte ASCII string */;
pub const RHBA_SERIAL_NUMBER: c_uint = 0x3 /* 4 to 64 byte ASCII string */;
pub const RHBA_MODEL: c_uint = 0x4 /* 4 to 256 byte ASCII string */;
pub const RHBA_MODEL_DESCRIPTION: c_uint = 0x5 /* 4 to 256 byte ASCII string */;
pub const RHBA_HARDWARE_VERSION: c_uint = 0x6 /* 4 to 256 byte ASCII string */;
pub const RHBA_DRIVER_VERSION: c_uint = 0x7 /* 4 to 256 byte ASCII string */;
pub const RHBA_OPTION_ROM_VERSION: c_uint = 0x8 /* 4 to 256 byte ASCII string */;
pub const RHBA_FIRMWARE_VERSION: c_uint = 0x9 /* 4 to 256 byte ASCII string */;
pub const RHBA_OS_NAME_VERSION: c_uint = 0xa /* 4 to 256 byte ASCII string */;
pub const RHBA_MAX_CT_PAYLOAD_LEN: c_uint = 0xb /* 32-bit unsigned int */;
pub const RHBA_SYM_NODENAME: c_uint = 0xc /* 4 to 256 byte ASCII string */;
pub const RHBA_VENDOR_INFO: c_uint = 0xd  /* 32-bit unsigned int */;
pub const RHBA_NUM_PORTS: c_uint = 0xe  /* 32-bit unsigned int */;
pub const RHBA_FABRIC_WWNN: c_uint = 0xf  /* 8 byte WWNN */;
pub const RHBA_BIOS_VERSION: c_uint = 0x10 /* 4 to 256 byte ASCII string */;
pub const RHBA_BIOS_STATE: c_uint = 0x11 /* 32-bit unsigned int */;
pub const RHBA_VENDOR_ID: c_uint = 0xe0 /* 8 byte ASCII string */;
// Bit mask for all individual HBA attributes
pub const LPFC_FDMI_HBA_ATTR_wwnn: c_uint = 0x00000001;
pub const LPFC_FDMI_HBA_ATTR_manufacturer: c_uint = 0x00000002;
pub const LPFC_FDMI_HBA_ATTR_sn: c_uint = 0x00000004;
pub const LPFC_FDMI_HBA_ATTR_model: c_uint = 0x00000008;
pub const LPFC_FDMI_HBA_ATTR_description: c_uint = 0x00000010;
pub const LPFC_FDMI_HBA_ATTR_hdw_ver: c_uint = 0x00000020;
pub const LPFC_FDMI_HBA_ATTR_drvr_ver: c_uint = 0x00000040;
pub const LPFC_FDMI_HBA_ATTR_rom_ver: c_uint = 0x00000080;
pub const LPFC_FDMI_HBA_ATTR_fmw_ver: c_uint = 0x00000100;
pub const LPFC_FDMI_HBA_ATTR_os_ver: c_uint = 0x00000200;
pub const LPFC_FDMI_HBA_ATTR_ct_len: c_uint = 0x00000400;
pub const LPFC_FDMI_HBA_ATTR_symbolic_name: c_uint = 0x00000800;
pub const LPFC_FDMI_HBA_ATTR_vendor_info: c_uint = 0x00001000 /* Not used */;
pub const LPFC_FDMI_HBA_ATTR_num_ports: c_uint = 0x00002000;
pub const LPFC_FDMI_HBA_ATTR_fabric_wwnn: c_uint = 0x00004000;
pub const LPFC_FDMI_HBA_ATTR_bios_ver: c_uint = 0x00008000;
pub const LPFC_FDMI_HBA_ATTR_bios_state: c_uint = 0x00010000 /* Not used */;
pub const LPFC_FDMI_HBA_ATTR_vendor_id: c_uint = 0x00020000;
// Bit mask for FDMI-1 defined HBA attributes
pub const LPFC_FDMI1_HBA_ATTR: c_uint = 0x000007ff;
// Bit mask for FDMI-2 defined HBA attributes
// Skip vendor_info and bios_state
pub const LPFC_FDMI2_HBA_ATTR: c_uint = 0x0002efff;
//
// Port Attribute Types
//
pub const RPRT_SUPPORTED_FC4_TYPES: c_uint = 0x1 /* 32 byte binary array */;
pub const RPRT_SUPPORTED_SPEED: c_uint = 0x2 /* 32-bit unsigned int */;
pub const RPRT_PORT_SPEED: c_uint = 0x3 /* 32-bit unsigned int */;
pub const RPRT_MAX_FRAME_SIZE: c_uint = 0x4 /* 32-bit unsigned int */;
pub const RPRT_OS_DEVICE_NAME: c_uint = 0x5 /* 4 to 256 byte ASCII string */;
pub const RPRT_HOST_NAME: c_uint = 0x6 /* 4 to 256 byte ASCII string */;
pub const RPRT_NODENAME: c_uint = 0x7 /* 8 byte WWNN */;
pub const RPRT_PORTNAME: c_uint = 0x8 /* 8 byte WWPN */;
pub const RPRT_SYM_PORTNAME: c_uint = 0x9 /* 4 to 256 byte ASCII string */;
pub const RPRT_PORT_TYPE: c_uint = 0xa /* 32-bit unsigned int */;
pub const RPRT_SUPPORTED_CLASS: c_uint = 0xb /* 32-bit unsigned int */;
pub const RPRT_FABRICNAME: c_uint = 0xc /* 8 byte Fabric WWPN */;
pub const RPRT_ACTIVE_FC4_TYPES: c_uint = 0xd /* 32 byte binary array */;
pub const RPRT_PORT_STATE: c_uint = 0x101 /* 32-bit unsigned int */;
pub const RPRT_DISC_PORT: c_uint = 0x102 /* 32-bit unsigned int */;
pub const RPRT_PORT_ID: c_uint = 0x103 /* 32-bit unsigned int */;
pub const RPRT_VENDOR_MI: c_uint = 0xf047 /* vendor ascii string */;
pub const RPRT_SMART_SERVICE: c_uint = 0xf100 /* 4 to 256 byte ASCII string */;
pub const RPRT_SMART_GUID: c_uint = 0xf101 /* 8 byte WWNN + 8 byte WWPN */;
pub const RPRT_SMART_VERSION: c_uint = 0xf102 /* 4 to 256 byte ASCII string */;
pub const RPRT_SMART_MODEL: c_uint = 0xf103 /* 4 to 256 byte ASCII string */;
pub const RPRT_SMART_PORT_INFO: c_uint = 0xf104 /* 32-bit unsigned int */;
pub const RPRT_SMART_QOS: c_uint = 0xf105 /* 32-bit unsigned int */;
pub const RPRT_SMART_SECURITY: c_uint = 0xf106 /* 32-bit unsigned int */;
// Bit mask for all individual PORT attributes
pub const LPFC_FDMI_PORT_ATTR_fc4type: c_uint = 0x00000001;
pub const LPFC_FDMI_PORT_ATTR_support_speed: c_uint = 0x00000002;
pub const LPFC_FDMI_PORT_ATTR_speed: c_uint = 0x00000004;
pub const LPFC_FDMI_PORT_ATTR_max_frame: c_uint = 0x00000008;
pub const LPFC_FDMI_PORT_ATTR_os_devname: c_uint = 0x00000010;
pub const LPFC_FDMI_PORT_ATTR_host_name: c_uint = 0x00000020;
pub const LPFC_FDMI_PORT_ATTR_wwnn: c_uint = 0x00000040;
pub const LPFC_FDMI_PORT_ATTR_wwpn: c_uint = 0x00000080;
pub const LPFC_FDMI_PORT_ATTR_symbolic_name: c_uint = 0x00000100;
pub const LPFC_FDMI_PORT_ATTR_port_type: c_uint = 0x00000200;
pub const LPFC_FDMI_PORT_ATTR_class: c_uint = 0x00000400;
pub const LPFC_FDMI_PORT_ATTR_fabric_wwpn: c_uint = 0x00000800;
pub const LPFC_FDMI_PORT_ATTR_port_state: c_uint = 0x00001000;
pub const LPFC_FDMI_PORT_ATTR_active_fc4type: c_uint = 0x00002000;
pub const LPFC_FDMI_PORT_ATTR_num_disc: c_uint = 0x00004000;
pub const LPFC_FDMI_PORT_ATTR_nportid: c_uint = 0x00008000;
pub const LPFC_FDMI_SMART_ATTR_service: c_uint = 0x00010000 /* Vendor specific */;
pub const LPFC_FDMI_SMART_ATTR_guid: c_uint = 0x00020000 /* Vendor specific */;
pub const LPFC_FDMI_SMART_ATTR_version: c_uint = 0x00040000 /* Vendor specific */;
pub const LPFC_FDMI_SMART_ATTR_model: c_uint = 0x00080000 /* Vendor specific */;
pub const LPFC_FDMI_SMART_ATTR_port_info: c_uint = 0x00100000 /* Vendor specific */;
pub const LPFC_FDMI_SMART_ATTR_qos: c_uint = 0x00200000 /* Vendor specific */;
pub const LPFC_FDMI_SMART_ATTR_security: c_uint = 0x00400000 /* Vendor specific */;
pub const LPFC_FDMI_VENDOR_ATTR_mi: c_uint = 0x00800000 /* Vendor specific */;
// Bit mask for FDMI-1 defined PORT attributes
pub const LPFC_FDMI1_PORT_ATTR: c_uint = 0x0000003f;
// Bit mask for FDMI-2 defined PORT attributes
pub const LPFC_FDMI2_PORT_ATTR: c_uint = 0x0000ffff;
// Bit mask for Smart SAN defined PORT attributes
pub const LPFC_FDMI2_SMART_ATTR: c_uint = 0x007fffff;
// Defines for PORT port state attribute
pub const LPFC_FDMI_PORTSTATE_UNKNOWN: c_int = 1;
pub const LPFC_FDMI_PORTSTATE_ONLINE: c_int = 2;
// Defines for PORT port type attribute
pub const LPFC_FDMI_PORTTYPE_UNKNOWN: c_int = 0;
pub const LPFC_FDMI_PORTTYPE_NPORT: c_int = 1;
pub const LPFC_FDMI_PORTTYPE_NLPORT: c_int = 2;
//
// Begin HBA configuration parameters.
// The PCI configuration register BAR assignments are:
// BAR0, offset 0x10 - SLIM base memory address
// BAR1, offset 0x14 - SLIM base memory high address
// BAR2, offset 0x18 - REGISTER base memory address
// BAR3, offset 0x1c - REGISTER base memory high address
// BAR4, offset 0x20 - BIU I/O registers
// BAR5, offset 0x24 - REGISTER base io high address
//
// Number of rings currently used and available.
pub const MAX_SLI3_CONFIGURED_RINGS: c_int = 3;
pub const MAX_SLI3_RINGS: c_int = 4;
// IOCB / Mailbox is owned by FireFly
pub const OWN_CHIP: c_int = 1;
// IOCB / Mailbox is owned by Host
pub const OWN_HOST: c_int = 0;
// Number of 4-byte words in an IOCB.
pub const IOCB_WORD_SZ: c_int = 8;
// network headers for Dfctl field
pub const FC_NET_HDR: c_uint = 0x20;
// Start FireFly Register definitions
pub const PCI_VENDOR_ID_EMULEX: c_uint = 0x10df;
pub const PCI_DEVICE_ID_FIREFLY: c_uint = 0x1ae5;
pub const PCI_DEVICE_ID_PROTEUS_VF: c_uint = 0xe100;
pub const PCI_DEVICE_ID_BALIUS: c_uint = 0xe131;
pub const PCI_DEVICE_ID_PROTEUS_PF: c_uint = 0xe180;
pub const PCI_DEVICE_ID_LANCER_FC: c_uint = 0xe200;
pub const PCI_DEVICE_ID_LANCER_FC_VF: c_uint = 0xe208;
pub const PCI_DEVICE_ID_LANCER_FCOE: c_uint = 0xe260;
pub const PCI_DEVICE_ID_LANCER_FCOE_VF: c_uint = 0xe268;
pub const PCI_DEVICE_ID_LANCER_G6_FC: c_uint = 0xe300;
pub const PCI_DEVICE_ID_LANCER_G7_FC: c_uint = 0xf400;
pub const PCI_DEVICE_ID_LANCER_G7P_FC: c_uint = 0xf500;
pub const PCI_DEVICE_ID_LANCER_G8_FC: c_uint = 0xd300;
pub const PCI_DEVICE_ID_SAT_SMB: c_uint = 0xf011;
pub const PCI_DEVICE_ID_SAT_MID: c_uint = 0xf015;
pub const PCI_DEVICE_ID_RFLY: c_uint = 0xf095;
pub const PCI_DEVICE_ID_PFLY: c_uint = 0xf098;
pub const PCI_DEVICE_ID_LP101: c_uint = 0xf0a1;
pub const PCI_DEVICE_ID_TFLY: c_uint = 0xf0a5;
pub const PCI_DEVICE_ID_BSMB: c_uint = 0xf0d1;
pub const PCI_DEVICE_ID_BMID: c_uint = 0xf0d5;
pub const PCI_DEVICE_ID_ZSMB: c_uint = 0xf0e1;
pub const PCI_DEVICE_ID_ZMID: c_uint = 0xf0e5;
pub const PCI_DEVICE_ID_NEPTUNE: c_uint = 0xf0f5;
pub const PCI_DEVICE_ID_NEPTUNE_SCSP: c_uint = 0xf0f6;
pub const PCI_DEVICE_ID_NEPTUNE_DCSP: c_uint = 0xf0f7;
pub const PCI_DEVICE_ID_SAT: c_uint = 0xf100;
pub const PCI_DEVICE_ID_SAT_SCSP: c_uint = 0xf111;
pub const PCI_DEVICE_ID_SAT_DCSP: c_uint = 0xf112;
pub const PCI_DEVICE_ID_FALCON: c_uint = 0xf180;
pub const PCI_DEVICE_ID_SUPERFLY: c_uint = 0xf700;
pub const PCI_DEVICE_ID_DRAGONFLY: c_uint = 0xf800;
pub const PCI_DEVICE_ID_CENTAUR: c_uint = 0xf900;
pub const PCI_DEVICE_ID_PEGASUS: c_uint = 0xf980;
pub const PCI_DEVICE_ID_THOR: c_uint = 0xfa00;
pub const PCI_DEVICE_ID_VIPER: c_uint = 0xfb00;
pub const PCI_DEVICE_ID_LP10000S: c_uint = 0xfc00;
pub const PCI_DEVICE_ID_LP11000S: c_uint = 0xfc10;
pub const PCI_DEVICE_ID_LPE11000S: c_uint = 0xfc20;
pub const PCI_DEVICE_ID_SAT_S: c_uint = 0xfc40;
pub const PCI_DEVICE_ID_PROTEUS_S: c_uint = 0xfc50;
pub const PCI_DEVICE_ID_HELIOS: c_uint = 0xfd00;
pub const PCI_DEVICE_ID_HELIOS_SCSP: c_uint = 0xfd11;
pub const PCI_DEVICE_ID_HELIOS_DCSP: c_uint = 0xfd12;
pub const PCI_DEVICE_ID_ZEPHYR: c_uint = 0xfe00;
pub const PCI_DEVICE_ID_ZEPHYR_SCSP: c_uint = 0xfe11;
pub const PCI_DEVICE_ID_ZEPHYR_DCSP: c_uint = 0xfe12;
pub const PCI_VENDOR_ID_SERVERENGINE: c_uint = 0x19a2;
pub const PCI_DEVICE_ID_TIGERSHARK: c_uint = 0x0704;
pub const PCI_DEVICE_ID_TOMCAT: c_uint = 0x0714;
pub const PCI_DEVICE_ID_SKYHAWK: c_uint = 0x0724;
pub const PCI_DEVICE_ID_SKYHAWK_VF: c_uint = 0x072c;
pub const PCI_VENDOR_ID_ATTO: c_uint = 0x117c;
pub const PCI_DEVICE_ID_CLRY_16XE: c_uint = 0x0064;
pub const PCI_DEVICE_ID_CLRY_161E: c_uint = 0x0063;
pub const PCI_DEVICE_ID_CLRY_162E: c_uint = 0x0064;
pub const PCI_DEVICE_ID_CLRY_164E: c_uint = 0x0065;
pub const PCI_DEVICE_ID_CLRY_16XP: c_uint = 0x0094;
pub const PCI_DEVICE_ID_CLRY_161P: c_uint = 0x00a0;
pub const PCI_DEVICE_ID_CLRY_162P: c_uint = 0x0094;
pub const PCI_DEVICE_ID_CLRY_164P: c_uint = 0x00a1;
pub const PCI_DEVICE_ID_CLRY_32XE: c_uint = 0x0094;
pub const PCI_DEVICE_ID_CLRY_321E: c_uint = 0x00a2;
pub const PCI_DEVICE_ID_CLRY_322E: c_uint = 0x00a3;
pub const PCI_DEVICE_ID_CLRY_324E: c_uint = 0x00ac;
pub const PCI_DEVICE_ID_CLRY_32XP: c_uint = 0x00bb;
pub const PCI_DEVICE_ID_CLRY_321P: c_uint = 0x00bc;
pub const PCI_DEVICE_ID_CLRY_322P: c_uint = 0x00bd;
pub const PCI_DEVICE_ID_CLRY_324P: c_uint = 0x00be;
pub const PCI_DEVICE_ID_TLFC_2: c_uint = 0x0064;
pub const PCI_DEVICE_ID_TLFC_2XX2: c_uint = 0x4064;
pub const PCI_DEVICE_ID_TLFC_3: c_uint = 0x0094;
pub const PCI_DEVICE_ID_TLFC_3162: c_uint = 0x40a6;
pub const PCI_DEVICE_ID_TLFC_3322: c_uint = 0x40a7;
pub const JEDEC_ID_ADDRESS: c_uint = 0x0080001c;
pub const FIREFLY_JEDEC_ID: c_uint = 0x1ACC;
pub const SUPERFLY_JEDEC_ID: c_uint = 0x0020;
pub const DRAGONFLY_JEDEC_ID: c_uint = 0x0021;
pub const DRAGONFLY_V2_JEDEC_ID: c_uint = 0x0025;
pub const CENTAUR_2G_JEDEC_ID: c_uint = 0x0026;
pub const CENTAUR_1G_JEDEC_ID: c_uint = 0x0028;
pub const PEGASUS_ORION_JEDEC_ID: c_uint = 0x0036;
pub const PEGASUS_JEDEC_ID: c_uint = 0x0038;
pub const THOR_JEDEC_ID: c_uint = 0x0012;
pub const HELIOS_JEDEC_ID: c_uint = 0x0364;
pub const ZEPHYR_JEDEC_ID: c_uint = 0x0577;
pub const VIPER_JEDEC_ID: c_uint = 0x4838;
pub const SATURN_JEDEC_ID: c_uint = 0x1004;
pub const JEDEC_ID_MASK: c_uint = 0x0FFFF000;
pub const JEDEC_ID_SHIFT: c_int = 12;

// IO Register size in bytes
pub const FF_REG_AREA_SIZE: c_int = 256;
// Host Attention Register

pub const HA_R0RE_REQ: c_uint = 0x00000001	/* Bit  0 */;
pub const HA_R0CE_RSP: c_uint = 0x00000002	/* Bit  1 */;
pub const HA_R0ATT: c_uint = 0x00000008	/* Bit  3 */;
pub const HA_R1RE_REQ: c_uint = 0x00000010	/* Bit  4 */;
pub const HA_R1CE_RSP: c_uint = 0x00000020	/* Bit  5 */;
pub const HA_R1ATT: c_uint = 0x00000080	/* Bit  7 */;
pub const HA_R2RE_REQ: c_uint = 0x00000100	/* Bit  8 */;
pub const HA_R2CE_RSP: c_uint = 0x00000200	/* Bit  9 */;
pub const HA_R2ATT: c_uint = 0x00000800	/* Bit 11 */;
pub const HA_R3RE_REQ: c_uint = 0x00001000	/* Bit 12 */;
pub const HA_R3CE_RSP: c_uint = 0x00002000	/* Bit 13 */;
pub const HA_R3ATT: c_uint = 0x00008000	/* Bit 15 */;
pub const HA_LATT: c_uint = 0x20000000	/* Bit 29 */;
pub const HA_MBATT: c_uint = 0x40000000	/* Bit 30 */;
pub const HA_ERATT: c_uint = 0x80000000	/* Bit 31 */;
pub const HA_RXRE_REQ: c_uint = 0x00000001	/* Bit  0 */;
pub const HA_RXCE_RSP: c_uint = 0x00000002	/* Bit  1 */;
pub const HA_RXATT: c_uint = 0x00000008	/* Bit  3 */;
pub const HA_RXMASK: c_uint = 0x0000000f;

pub const HA_R0_POS: c_int = 3;
pub const HA_R1_POS: c_int = 7;
pub const HA_R2_POS: c_int = 11;
pub const HA_R3_POS: c_int = 15;
pub const HA_LE_POS: c_int = 29;
pub const HA_MB_POS: c_int = 30;
pub const HA_ER_POS: c_int = 31;
// Chip Attention Register

pub const CA_R0CE_REQ: c_uint = 0x00000001	/* Bit  0 */;
pub const CA_R0RE_RSP: c_uint = 0x00000002	/* Bit  1 */;
pub const CA_R0ATT: c_uint = 0x00000008	/* Bit  3 */;
pub const CA_R1CE_REQ: c_uint = 0x00000010	/* Bit  4 */;
pub const CA_R1RE_RSP: c_uint = 0x00000020	/* Bit  5 */;
pub const CA_R1ATT: c_uint = 0x00000080	/* Bit  7 */;
pub const CA_R2CE_REQ: c_uint = 0x00000100	/* Bit  8 */;
pub const CA_R2RE_RSP: c_uint = 0x00000200	/* Bit  9 */;
pub const CA_R2ATT: c_uint = 0x00000800	/* Bit 11 */;
pub const CA_R3CE_REQ: c_uint = 0x00001000	/* Bit 12 */;
pub const CA_R3RE_RSP: c_uint = 0x00002000	/* Bit 13 */;
pub const CA_R3ATT: c_uint = 0x00008000	/* Bit 15 */;
pub const CA_MBATT: c_uint = 0x40000000	/* Bit 30 */;
// Host Status Register

pub const HS_MBRDY: c_uint = 0x00400000	/* Bit 22 */;
pub const HS_FFRDY: c_uint = 0x00800000	/* Bit 23 */;
pub const HS_FFER8: c_uint = 0x01000000	/* Bit 24 */;
pub const HS_FFER7: c_uint = 0x02000000	/* Bit 25 */;
pub const HS_FFER6: c_uint = 0x04000000	/* Bit 26 */;
pub const HS_FFER5: c_uint = 0x08000000	/* Bit 27 */;
pub const HS_FFER4: c_uint = 0x10000000	/* Bit 28 */;
pub const HS_FFER3: c_uint = 0x20000000	/* Bit 29 */;
pub const HS_FFER2: c_uint = 0x40000000	/* Bit 30 */;
pub const HS_FFER1: c_uint = 0x80000000	/* Bit 31 */;
pub const HS_CRIT_TEMP: c_uint = 0x00000100	/* Bit 8  */;
pub const HS_FFERM: c_uint = 0xFF000100	/* Mask for error bits 31:24 and 8 */;
pub const UNPLUG_ERR: c_uint = 0x00000001	/* Indicate pci hot unplug */;
// Host Control Register

pub const HC_MBINT_ENA: c_uint = 0x00000001	/* Bit  0 */;
pub const HC_R0INT_ENA: c_uint = 0x00000002	/* Bit  1 */;
pub const HC_R1INT_ENA: c_uint = 0x00000004	/* Bit  2 */;
pub const HC_R2INT_ENA: c_uint = 0x00000008	/* Bit  3 */;
pub const HC_R3INT_ENA: c_uint = 0x00000010	/* Bit  4 */;
pub const HC_INITHBI: c_uint = 0x02000000	/* Bit 25 */;
pub const HC_INITMB: c_uint = 0x04000000	/* Bit 26 */;
pub const HC_INITFF: c_uint = 0x08000000	/* Bit 27 */;
pub const HC_LAINT_ENA: c_uint = 0x20000000	/* Bit 29 */;
pub const HC_ERINT_ENA: c_uint = 0x80000000	/* Bit 31 */;
// Message Signaled Interrupt eXtension (MSI-X) message identifiers
pub const MSIX_DFLT_ID: c_int = 0;
pub const MSIX_RNG0_ID: c_int = 0;
pub const MSIX_RNG1_ID: c_int = 1;
pub const MSIX_RNG2_ID: c_int = 2;
pub const MSIX_RNG3_ID: c_int = 3;
pub const MSIX_LINK_ID: c_int = 4;
pub const MSIX_MBOX_ID: c_int = 5;
pub const MSIX_SPARE0_ID: c_int = 6;
pub const MSIX_SPARE1_ID: c_int = 7;
// Mailbox Commands
pub const MBX_SHUTDOWN: c_uint = 0x00	/* terminate testing */;
pub const MBX_LOAD_SM: c_uint = 0x01;
pub const MBX_READ_NV: c_uint = 0x02;
pub const MBX_WRITE_NV: c_uint = 0x03;
pub const MBX_RUN_BIU_DIAG: c_uint = 0x04;
pub const MBX_INIT_LINK: c_uint = 0x05;
pub const MBX_DOWN_LINK: c_uint = 0x06;
pub const MBX_CONFIG_LINK: c_uint = 0x07;
pub const MBX_CONFIG_RING: c_uint = 0x09;
pub const MBX_RESET_RING: c_uint = 0x0A;
pub const MBX_READ_CONFIG: c_uint = 0x0B;
pub const MBX_READ_RCONFIG: c_uint = 0x0C;
pub const MBX_READ_SPARM: c_uint = 0x0D;
pub const MBX_READ_STATUS: c_uint = 0x0E;
pub const MBX_READ_RPI: c_uint = 0x0F;
pub const MBX_READ_XRI: c_uint = 0x10;
pub const MBX_READ_REV: c_uint = 0x11;
pub const MBX_READ_LNK_STAT: c_uint = 0x12;
pub const MBX_REG_LOGIN: c_uint = 0x13;
pub const MBX_UNREG_LOGIN: c_uint = 0x14;
pub const MBX_CLEAR_LA: c_uint = 0x16;
pub const MBX_DUMP_MEMORY: c_uint = 0x17;
pub const MBX_DUMP_CONTEXT: c_uint = 0x18;
pub const MBX_RUN_DIAGS: c_uint = 0x19;
pub const MBX_RESTART: c_uint = 0x1A;
pub const MBX_UPDATE_CFG: c_uint = 0x1B;
pub const MBX_DOWN_LOAD: c_uint = 0x1C;
pub const MBX_DEL_LD_ENTRY: c_uint = 0x1D;
pub const MBX_RUN_PROGRAM: c_uint = 0x1E;
pub const MBX_SET_MASK: c_uint = 0x20;
pub const MBX_SET_VARIABLE: c_uint = 0x21;
pub const MBX_UNREG_D_ID: c_uint = 0x23;
pub const MBX_KILL_BOARD: c_uint = 0x24;
pub const MBX_CONFIG_FARP: c_uint = 0x25;
pub const MBX_BEACON: c_uint = 0x2A;
pub const MBX_CONFIG_MSI: c_uint = 0x30;
pub const MBX_HEARTBEAT: c_uint = 0x31;
pub const MBX_WRITE_VPARMS: c_uint = 0x32;
pub const MBX_ASYNCEVT_ENABLE: c_uint = 0x33;
pub const MBX_READ_EVENT_LOG_STATUS: c_uint = 0x37;
pub const MBX_READ_EVENT_LOG: c_uint = 0x38;
pub const MBX_WRITE_EVENT_LOG: c_uint = 0x39;
pub const MBX_PORT_CAPABILITIES: c_uint = 0x3B;
pub const MBX_PORT_IOV_CONTROL: c_uint = 0x3C;
pub const MBX_CONFIG_HBQ: c_uint = 0x7C;
pub const MBX_LOAD_AREA: c_uint = 0x81;
pub const MBX_RUN_BIU_DIAG64: c_uint = 0x84;
pub const MBX_CONFIG_PORT: c_uint = 0x88;
pub const MBX_READ_SPARM64: c_uint = 0x8D;
pub const MBX_READ_RPI64: c_uint = 0x8F;
pub const MBX_REG_LOGIN64: c_uint = 0x93;
pub const MBX_READ_TOPOLOGY: c_uint = 0x95;
pub const MBX_REG_VPI: c_uint = 0x96;
pub const MBX_UNREG_VPI: c_uint = 0x97;
pub const MBX_WRITE_WWN: c_uint = 0x98;
pub const MBX_SET_DEBUG: c_uint = 0x99;
pub const MBX_LOAD_EXP_ROM: c_uint = 0x9C;
pub const MBX_SLI4_CONFIG: c_uint = 0x9B;
pub const MBX_SLI4_REQ_FTRS: c_uint = 0x9D;
pub const MBX_MAX_CMDS: c_uint = 0x9E;
pub const MBX_RESUME_RPI: c_uint = 0x9E;
pub const MBX_SLI2_CMD_MASK: c_uint = 0x80;
pub const MBX_REG_VFI: c_uint = 0x9F;
pub const MBX_REG_FCFI: c_uint = 0xA0;
pub const MBX_UNREG_VFI: c_uint = 0xA1;
pub const MBX_UNREG_FCFI: c_uint = 0xA2;
pub const MBX_INIT_VFI: c_uint = 0xA3;
pub const MBX_INIT_VPI: c_uint = 0xA4;
pub const MBX_ACCESS_VDATA: c_uint = 0xA5;
pub const MBX_REG_FCFI_MRQ: c_uint = 0xAF;
pub const MBX_AUTH_PORT: c_uint = 0xF8;
pub const MBX_SECURITY_MGMT: c_uint = 0xF9;
// IOCB Commands
pub const CMD_RCV_SEQUENCE_CX: c_uint = 0x01;
pub const CMD_XMIT_SEQUENCE_CR: c_uint = 0x02;
pub const CMD_XMIT_SEQUENCE_CX: c_uint = 0x03;
pub const CMD_XMIT_BCAST_CN: c_uint = 0x04;
pub const CMD_XMIT_BCAST_CX: c_uint = 0x05;
pub const CMD_QUE_RING_BUF_CN: c_uint = 0x06;
pub const CMD_QUE_XRI_BUF_CX: c_uint = 0x07;
pub const CMD_IOCB_CONTINUE_CN: c_uint = 0x08;
pub const CMD_RET_XRI_BUF_CX: c_uint = 0x09;
pub const CMD_ELS_REQUEST_CR: c_uint = 0x0A;
pub const CMD_ELS_REQUEST_CX: c_uint = 0x0B;
pub const CMD_RCV_ELS_REQ_CX: c_uint = 0x0D;
pub const CMD_ABORT_XRI_CN: c_uint = 0x0E;
pub const CMD_ABORT_XRI_CX: c_uint = 0x0F;
pub const CMD_CLOSE_XRI_CN: c_uint = 0x10;
pub const CMD_CLOSE_XRI_CX: c_uint = 0x11;
pub const CMD_CREATE_XRI_CR: c_uint = 0x12;
pub const CMD_CREATE_XRI_CX: c_uint = 0x13;
pub const CMD_GET_RPI_CN: c_uint = 0x14;
pub const CMD_XMIT_ELS_RSP_CX: c_uint = 0x15;
pub const CMD_GET_RPI_CR: c_uint = 0x16;
pub const CMD_XRI_ABORTED_CX: c_uint = 0x17;
pub const CMD_FCP_IWRITE_CR: c_uint = 0x18;
pub const CMD_FCP_IWRITE_CX: c_uint = 0x19;
pub const CMD_FCP_IREAD_CR: c_uint = 0x1A;
pub const CMD_FCP_IREAD_CX: c_uint = 0x1B;
pub const CMD_FCP_ICMND_CR: c_uint = 0x1C;
pub const CMD_FCP_ICMND_CX: c_uint = 0x1D;
pub const CMD_FCP_TSEND_CX: c_uint = 0x1F;
pub const CMD_FCP_TRECEIVE_CX: c_uint = 0x21;
pub const CMD_FCP_TRSP_CX: c_uint = 0x23;
pub const CMD_FCP_AUTO_TRSP_CX: c_uint = 0x29;
pub const CMD_ADAPTER_MSG: c_uint = 0x20;
pub const CMD_ADAPTER_DUMP: c_uint = 0x22;
// SLI_2 IOCB Command Set
pub const CMD_ASYNC_STATUS: c_uint = 0x7C;
pub const CMD_RCV_SEQUENCE64_CX: c_uint = 0x81;
pub const CMD_XMIT_SEQUENCE64_CR: c_uint = 0x82;
pub const CMD_XMIT_SEQUENCE64_CX: c_uint = 0x83;
pub const CMD_XMIT_BCAST64_CN: c_uint = 0x84;
pub const CMD_XMIT_BCAST64_CX: c_uint = 0x85;
pub const CMD_QUE_RING_BUF64_CN: c_uint = 0x86;
pub const CMD_QUE_XRI_BUF64_CX: c_uint = 0x87;
pub const CMD_IOCB_CONTINUE64_CN: c_uint = 0x88;
pub const CMD_RET_XRI_BUF64_CX: c_uint = 0x89;
pub const CMD_ELS_REQUEST64_CR: c_uint = 0x8A;
pub const CMD_ELS_REQUEST64_CX: c_uint = 0x8B;
pub const CMD_ABORT_MXRI64_CN: c_uint = 0x8C;
pub const CMD_RCV_ELS_REQ64_CX: c_uint = 0x8D;
pub const CMD_XMIT_ELS_RSP64_CX: c_uint = 0x95;
pub const CMD_XMIT_BLS_RSP64_CX: c_uint = 0x97;
pub const CMD_FCP_IWRITE64_CR: c_uint = 0x98;
pub const CMD_FCP_IWRITE64_CX: c_uint = 0x99;
pub const CMD_FCP_IREAD64_CR: c_uint = 0x9A;
pub const CMD_FCP_IREAD64_CX: c_uint = 0x9B;
pub const CMD_FCP_ICMND64_CR: c_uint = 0x9C;
pub const CMD_FCP_ICMND64_CX: c_uint = 0x9D;
pub const CMD_FCP_TSEND64_CX: c_uint = 0x9F;
pub const CMD_FCP_TRECEIVE64_CX: c_uint = 0xA1;
pub const CMD_FCP_TRSP64_CX: c_uint = 0xA3;
pub const CMD_QUE_XRI64_CX: c_uint = 0xB3;
pub const CMD_IOCB_RCV_SEQ64_CX: c_uint = 0xB5;
pub const CMD_IOCB_RCV_ELS64_CX: c_uint = 0xB7;
pub const CMD_IOCB_RET_XRI64_CX: c_uint = 0xB9;
pub const CMD_IOCB_RCV_CONT64_CX: c_uint = 0xBB;
pub const CMD_GEN_REQUEST64_CR: c_uint = 0xC2;
pub const CMD_GEN_REQUEST64_CX: c_uint = 0xC3;
// Unhandled SLI-3 Commands
pub const CMD_IOCB_XMIT_MSEQ64_CR: c_uint = 0xB0;
pub const CMD_IOCB_XMIT_MSEQ64_CX: c_uint = 0xB1;
pub const CMD_IOCB_RCV_SEQ_LIST64_CX: c_uint = 0xC1;
pub const CMD_IOCB_RCV_ELS_LIST64_CX: c_uint = 0xCD;
pub const CMD_IOCB_CLOSE_EXTENDED_CN: c_uint = 0xB6;
pub const CMD_IOCB_ABORT_EXTENDED_CN: c_uint = 0xBA;
pub const CMD_IOCB_RET_HBQE64_CN: c_uint = 0xCA;
pub const CMD_IOCB_FCP_IBIDIR64_CR: c_uint = 0xAC;
pub const CMD_IOCB_FCP_IBIDIR64_CX: c_uint = 0xAD;
pub const CMD_IOCB_FCP_ITASKMGT64_CX: c_uint = 0xAF;
pub const CMD_IOCB_LOGENTRY_CN: c_uint = 0x94;
pub const CMD_IOCB_LOGENTRY_ASYNC_CN: c_uint = 0x96;
// Data Security SLI Commands
pub const DSSCMD_IWRITE64_CR: c_uint = 0xF8;
pub const DSSCMD_IWRITE64_CX: c_uint = 0xF9;
pub const DSSCMD_IREAD64_CR: c_uint = 0xFA;
pub const DSSCMD_IREAD64_CX: c_uint = 0xFB;
pub const CMD_MAX_IOCB_CMD: c_uint = 0xFB;
pub const CMD_IOCB_MASK: c_uint = 0xff;

//
// Define Status
//
pub const MBX_SUCCESS: c_int = 0;
pub const MBXERR_NUM_RINGS: c_int = 1;
pub const MBXERR_NUM_IOCBS: c_int = 2;
pub const MBXERR_IOCBS_EXCEEDED: c_int = 3;
pub const MBXERR_BAD_RING_NUMBER: c_int = 4;
pub const MBXERR_MASK_ENTRIES_RANGE: c_int = 5;
pub const MBXERR_MASKS_EXCEEDED: c_int = 6;
pub const MBXERR_BAD_PROFILE: c_int = 7;
pub const MBXERR_BAD_DEF_CLASS: c_int = 8;
pub const MBXERR_BAD_MAX_RESPONDER: c_int = 9;
pub const MBXERR_BAD_MAX_ORIGINATOR: c_int = 10;
pub const MBXERR_RPI_REGISTERED: c_int = 11;
pub const MBXERR_RPI_FULL: c_int = 12;
pub const MBXERR_NO_RESOURCES: c_int = 13;
pub const MBXERR_BAD_RCV_LENGTH: c_int = 14;
pub const MBXERR_DMA_ERROR: c_int = 15;
pub const MBXERR_ERROR: c_int = 16;
pub const MBXERR_LINK_DOWN: c_uint = 0x33;
pub const MBXERR_SEC_NO_PERMISSION: c_uint = 0xF02;
pub const MBX_NOT_FINISHED: c_int = 255;
pub const MBX_BUSY: c_uint = 0xffffff /* Attempted cmd to busy Mailbox */;
pub const MBX_TIMEOUT: c_uint = 0xfffffe /* time-out expired waiting for */;
pub const TEMPERATURE_OFFSET: c_uint = 0xB0	/* Slim offset for critical temperature event */;
//
// return code Fail
//
pub const FAILURE: c_int = 1;
//
// Begin Structure Definitions for Mailbox Commands
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ulp_bde {
    pub bdeAddress: u32,

    pub bdeReserved:4: u32,
    pub bdeAddrHigh:4: u32,
    pub bdeSize:24: u32,

    pub bdeSize:24: u32,
    pub bdeAddrHigh:4: u32,
    pub bdeReserved:4: u32,

}

//
// BlockGuard Definitions
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lpfc_protgrp_type {
    LPFC_PG_TYPE_INVALID = 0, /* used to indicate errors                  */
    LPFC_PG_TYPE_NO_DIF,	  /* no DIF data pointed to by prot grp       */
    LPFC_PG_TYPE_EMBD_DIF,	  /* DIF is embedded (inline) with data       */
    LPFC_PG_TYPE_DIF_BUF	  /* DIF has its own scatter/gather list      */
}

// PDE Descriptors
pub const LPFC_PDE5_DESCRIPTOR: c_uint = 0x85;
pub const LPFC_PDE6_DESCRIPTOR: c_uint = 0x86;
pub const LPFC_PDE7_DESCRIPTOR: c_uint = 0x87;
// BlockGuard Opcodes
pub const BG_OP_IN_NODIF_OUT_CRC: c_uint = 0x0;
pub const BG_OP_IN_CRC_OUT_NODIF: c_uint = 0x1;
pub const BG_OP_IN_NODIF_OUT_CSUM: c_uint = 0x2;
pub const BG_OP_IN_CSUM_OUT_NODIF: c_uint = 0x3;
pub const BG_OP_IN_CRC_OUT_CRC: c_uint = 0x4;
pub const BG_OP_IN_CSUM_OUT_CSUM: c_uint = 0x5;
pub const BG_OP_IN_CRC_OUT_CSUM: c_uint = 0x6;
pub const BG_OP_IN_CSUM_OUT_CRC: c_uint = 0x7;
pub const BG_OP_RAW_MODE: c_uint = 0x8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_pde5 {
    pub word0: u32,
pub const pde5_type_SHIFT: c_int = 24;
pub const pde5_type_MASK: c_uint = 0x000000ff;

pub const pde5_rsvd0_SHIFT: c_int = 0;
pub const pde5_rsvd0_MASK: c_uint = 0x00ffffff;

    pub /: *mut *mut uint32_t reftag; / Reference Tag Value,
    pub /: *mut *mut uint32_t reftagtr; / Reference Tag Translation Value,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_pde6 {
    pub word0: u32,
pub const pde6_type_SHIFT: c_int = 24;
pub const pde6_type_MASK: c_uint = 0x000000ff;

pub const pde6_rsvd0_SHIFT: c_int = 0;
pub const pde6_rsvd0_MASK: c_uint = 0x00ffffff;

    pub word1: u32,
pub const pde6_rsvd1_SHIFT: c_int = 26;
pub const pde6_rsvd1_MASK: c_uint = 0x0000003f;

pub const pde6_na_SHIFT: c_int = 25;
pub const pde6_na_MASK: c_uint = 0x00000001;

pub const pde6_rsvd2_SHIFT: c_int = 16;
pub const pde6_rsvd2_MASK: c_uint = 0x000001FF;

pub const pde6_apptagtr_SHIFT: c_int = 0;
pub const pde6_apptagtr_MASK: c_uint = 0x0000ffff;

    pub word2: u32,
pub const pde6_optx_SHIFT: c_int = 28;
pub const pde6_optx_MASK: c_uint = 0x0000000f;

pub const pde6_oprx_SHIFT: c_int = 24;
pub const pde6_oprx_MASK: c_uint = 0x0000000f;

pub const pde6_nr_SHIFT: c_int = 23;
pub const pde6_nr_MASK: c_uint = 0x00000001;

pub const pde6_ce_SHIFT: c_int = 22;
pub const pde6_ce_MASK: c_uint = 0x00000001;

pub const pde6_re_SHIFT: c_int = 21;
pub const pde6_re_MASK: c_uint = 0x00000001;

pub const pde6_ae_SHIFT: c_int = 20;
pub const pde6_ae_MASK: c_uint = 0x00000001;

pub const pde6_ai_SHIFT: c_int = 19;
pub const pde6_ai_MASK: c_uint = 0x00000001;

pub const pde6_bs_SHIFT: c_int = 16;
pub const pde6_bs_MASK: c_uint = 0x00000007;

pub const pde6_apptagval_SHIFT: c_int = 0;
pub const pde6_apptagval_MASK: c_uint = 0x0000ffff;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_pde7 {
    pub word0: u32,
pub const pde7_type_SHIFT: c_int = 24;
pub const pde7_type_MASK: c_uint = 0x000000ff;

pub const pde7_rsvd0_SHIFT: c_int = 0;
pub const pde7_rsvd0_MASK: c_uint = 0x00ffffff;

    pub addrHigh: u32,
    pub addrLow: u32,
}

// Structure for MB Command LOAD_SM and DOWN_LOAD

// Structure for MB Command READ_NVPARM (02)

// Structure for MB Command WRITE_NVPARMS (03)

// Structure for MB Command RUN_BIU_DIAG (04)
// Structure for MB Command RUN_BIU_DIAG64 (0x84)
// Structure for MB command READ_EVENT_LOG (0x38)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct READ_EVENT_LOG_VAR {
    pub word1: u32,
pub const lpfc_event_log_SHIFT: c_int = 29;
pub const lpfc_event_log_MASK: c_uint = 0x00000001;

pub const USE_MAILBOX_RESPONSE: c_int = 1;
    pub offset: u32,
    pub rcv_bde64: ulp_bde64,
}

// Structure for MB Command INIT_LINK (05)

pub const FLAGS_TOPOLOGY_MODE_LOOP_PT: c_uint = 0x00 /* Attempt loop then pt-pt */;
pub const FLAGS_LOCAL_LB: c_uint = 0x01 /* link_flags (=1) ENDEC loopback */;
pub const FLAGS_TOPOLOGY_MODE_PT_PT: c_uint = 0x02 /* Attempt pt-pt only */;
pub const FLAGS_TOPOLOGY_MODE_LOOP: c_uint = 0x04 /* Attempt loop only */;
pub const FLAGS_TOPOLOGY_MODE_PT_LOOP: c_uint = 0x06 /* Attempt pt-pt then loop */;
pub const FLAGS_UNREG_LOGIN_ALL: c_uint = 0x08 /* UNREG_LOGIN all on link down */;
pub const FLAGS_LIRP_LILP: c_uint = 0x80 /* LIRP / LILP is disabled */;
pub const FLAGS_TOPOLOGY_FAILOVER: c_uint = 0x0400	/* Bit 10 */;
pub const FLAGS_LINK_SPEED: c_uint = 0x0800	/* Bit 11 */;
pub const FLAGS_IMED_ABORT: c_uint = 0x04000	/* Bit 14 */;
pub const LINK_SPEED_AUTO: c_uint = 0x0     /* Auto selection */;
pub const LINK_SPEED_1G: c_uint = 0x1     /* 1 Gigabaud */;
pub const LINK_SPEED_2G: c_uint = 0x2     /* 2 Gigabaud */;
pub const LINK_SPEED_4G: c_uint = 0x4     /* 4 Gigabaud */;
pub const LINK_SPEED_8G: c_uint = 0x8     /* 8 Gigabaud */;
pub const LINK_SPEED_10G: c_uint = 0x10    /* 10 Gigabaud */;
pub const LINK_SPEED_16G: c_uint = 0x11    /* 16 Gigabaud */;
pub const LINK_SPEED_32G: c_uint = 0x14    /* 32 Gigabaud */;
pub const LINK_SPEED_64G: c_uint = 0x17    /* 64 Gigabaud */;
pub const LINK_SPEED_128G: c_uint = 0x1A    /* 128 Gigabaud */;
pub const LINK_SPEED_256G: c_uint = 0x1D    /* 256 Gigabaud */;
// Structure for MB Command DOWN_LINK (06)
// Structure for MB Command CONFIG_LINK (07)

// Structure for MB Command PART_SLIM (08)
// will be removed since SLI1 is no longer supported!
//

// Structure for MB Command CONFIG_RING (09)

// Structure for MB Command RESET_RING (10)
// Structure for MB Command READ_CONFIG (11)

// Defines for topology (defined previously)

pub const LMT_RESERVED: c_uint = 0x000    /* Not used */;
pub const LMT_1Gb: c_uint = 0x004;
pub const LMT_2Gb: c_uint = 0x008;
pub const LMT_4Gb: c_uint = 0x040;
pub const LMT_8Gb: c_uint = 0x080;
pub const LMT_10Gb: c_uint = 0x100;
pub const LMT_16Gb: c_uint = 0x200;
pub const LMT_32Gb: c_uint = 0x400;
pub const LMT_64Gb: c_uint = 0x800;
pub const LMT_128Gb: c_uint = 0x1000;
pub const LMT_256Gb: c_uint = 0x2000;
// Structure for MB Command READ_RCONFIG (12)

// Structure for MB Command READ_SPARM (13)
// Structure for MB Command READ_SPARM64 (0x8D)

// Structure for MB Command READ_STATUS (14)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum read_status_word1 {
    RD_ST_CC	= 0x01,
    RD_ST_XKB	= 0x80,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum read_status_word17 {
    RD_ST_XMIT_XKB_MASK = 0x3fffff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum read_status_word18 {
    RD_ST_RCV_XKB_MASK = 0x3fffff,
}

// Structure for MB Command READ_RPI (15)
// Structure for MB Command READ_RPI64 (0x8F)

// Structure for MB Command READ_XRI (16)

// Structure for MB Command READ_REV (17)

// Structure for MB Command READ_LINK_STAT (18)
pub const lpfc_read_link_stat_rec_SHIFT: c_int = 0;
pub const lpfc_read_link_stat_rec_MASK: c_uint = 0x1;

pub const lpfc_read_link_stat_gec_SHIFT: c_int = 1;
pub const lpfc_read_link_stat_gec_MASK: c_uint = 0x1;

pub const lpfc_read_link_stat_w02oftow23of_SHIFT: c_int = 2;
pub const lpfc_read_link_stat_w02oftow23of_MASK: c_uint = 0x3FFFFF;

pub const lpfc_read_link_stat_rsvd_SHIFT: c_int = 24;
pub const lpfc_read_link_stat_rsvd_MASK: c_uint = 0x1F;

pub const lpfc_read_link_stat_gec2_SHIFT: c_int = 29;
pub const lpfc_read_link_stat_gec2_MASK: c_uint = 0x1;

pub const lpfc_read_link_stat_clrc_SHIFT: c_int = 30;
pub const lpfc_read_link_stat_clrc_MASK: c_uint = 0x1;

pub const lpfc_read_link_stat_clof_SHIFT: c_int = 31;
pub const lpfc_read_link_stat_clof_MASK: c_uint = 0x1;

// Structure for MB Command REG_LOGIN (19)
// Structure for MB Command REG_LOGIN64 (0x93)

// Word 30 contents for REG_LOGIN

// Structure for MB Command UNREG_LOGIN (20)

// Structure for MB Command REG_VPI (0x96)

// Structure for MB Command UNREG_VPI (0x97)

// Structure for MB Command UNREG_D_ID (0x23)

// Structure for MB Command READ_TOPOLOGY (0x95)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_read_top {
    pub /: *mut *mut uint32_t eventTag; / Event tag,
    pub word2: u32,
pub const lpfc_mbx_read_top_fa_SHIFT: c_int = 12;
pub const lpfc_mbx_read_top_fa_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_read_top_mm_SHIFT: c_int = 11;
pub const lpfc_mbx_read_top_mm_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_read_top_pb_SHIFT: c_int = 9;

pub const lpfc_mbx_read_top_il_SHIFT: c_int = 8;
pub const lpfc_mbx_read_top_il_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_read_top_att_type_SHIFT: c_int = 0;
pub const lpfc_mbx_read_top_att_type_MASK: c_uint = 0x000000FF;

pub const LPFC_ATT_RESERVED: c_uint = 0x00	/* Reserved - attType */;
pub const LPFC_ATT_LINK_UP: c_uint = 0x01	/* Link is up */;
pub const LPFC_ATT_LINK_DOWN: c_uint = 0x02	/* Link is down */;
pub const LPFC_ATT_UNEXP_WWPN: c_uint = 0x06	/* Link is down Unexpected WWWPN */;
    pub word3: u32,
pub const lpfc_mbx_read_top_alpa_granted_SHIFT: c_int = 24;
pub const lpfc_mbx_read_top_alpa_granted_MASK: c_uint = 0x000000FF;

pub const lpfc_mbx_read_top_lip_alps_SHIFT: c_int = 16;
pub const lpfc_mbx_read_top_lip_alps_MASK: c_uint = 0x000000FF;

pub const lpfc_mbx_read_top_lip_type_SHIFT: c_int = 8;
pub const lpfc_mbx_read_top_lip_type_MASK: c_uint = 0x000000FF;

pub const lpfc_mbx_read_top_topology_SHIFT: c_int = 0;
pub const lpfc_mbx_read_top_topology_MASK: c_uint = 0x000000FF;

pub const LPFC_TOPOLOGY_PT_PT: c_uint = 0x01	/* Topology is pt-pt / pt-fabric */;
pub const LPFC_TOPOLOGY_LOOP: c_uint = 0x02	/* Topology is FC-AL */;
// store the LILP AL_PA position map into
    pub lilpBde64: ulp_bde64,
pub const LPFC_ALPA_MAP_SIZE: c_int = 128;
    pub word7: u32,
pub const lpfc_mbx_read_top_ld_lu_SHIFT: c_int = 31;
pub const lpfc_mbx_read_top_ld_lu_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_read_top_ld_tf_SHIFT: c_int = 30;
pub const lpfc_mbx_read_top_ld_tf_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_read_top_ld_link_spd_SHIFT: c_int = 8;
pub const lpfc_mbx_read_top_ld_link_spd_MASK: c_uint = 0x000000FF;

pub const lpfc_mbx_read_top_ld_nl_port_SHIFT: c_int = 4;
pub const lpfc_mbx_read_top_ld_nl_port_MASK: c_uint = 0x0000000F;

pub const lpfc_mbx_read_top_ld_tx_SHIFT: c_int = 2;
pub const lpfc_mbx_read_top_ld_tx_MASK: c_uint = 0x00000003;

pub const lpfc_mbx_read_top_ld_rx_SHIFT: c_int = 0;
pub const lpfc_mbx_read_top_ld_rx_MASK: c_uint = 0x00000003;

    pub word8: u32,
pub const lpfc_mbx_read_top_lu_SHIFT: c_int = 31;
pub const lpfc_mbx_read_top_lu_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_read_top_tf_SHIFT: c_int = 30;
pub const lpfc_mbx_read_top_tf_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_read_top_link_spd_SHIFT: c_int = 8;
pub const lpfc_mbx_read_top_link_spd_MASK: c_uint = 0x000000FF;

pub const lpfc_mbx_read_top_nl_port_SHIFT: c_int = 4;
pub const lpfc_mbx_read_top_nl_port_MASK: c_uint = 0x0000000F;

pub const lpfc_mbx_read_top_tx_SHIFT: c_int = 2;
pub const lpfc_mbx_read_top_tx_MASK: c_uint = 0x00000003;

pub const lpfc_mbx_read_top_rx_SHIFT: c_int = 0;
pub const lpfc_mbx_read_top_rx_MASK: c_uint = 0x00000003;

pub const LPFC_LINK_SPEED_UNKNOWN: c_uint = 0x0;
pub const LPFC_LINK_SPEED_1GHZ: c_uint = 0x04;
pub const LPFC_LINK_SPEED_2GHZ: c_uint = 0x08;
pub const LPFC_LINK_SPEED_4GHZ: c_uint = 0x10;
pub const LPFC_LINK_SPEED_8GHZ: c_uint = 0x20;
pub const LPFC_LINK_SPEED_10GHZ: c_uint = 0x40;
pub const LPFC_LINK_SPEED_16GHZ: c_uint = 0x80;
pub const LPFC_LINK_SPEED_32GHZ: c_uint = 0x90;
pub const LPFC_LINK_SPEED_64GHZ: c_uint = 0xA0;
pub const LPFC_LINK_SPEED_128GHZ: c_uint = 0xB0;
pub const LPFC_LINK_SPEED_256GHZ: c_uint = 0xC0;
}

// Structure for MB Command CLEAR_LA (22)
// Structure for MB Command DUMP

pub const DMP_MEM_REG: c_uint = 0x1;
pub const DMP_NV_PARAMS: c_uint = 0x2;
pub const DMP_LMSD: c_uint = 0x3 /* Link Module Serial Data */;
pub const DMP_WELL_KNOWN: c_uint = 0x4;
pub const DMP_REGION_VPD: c_uint = 0xe;
pub const DMP_VPD_SIZE: c_uint = 0x400  /* maximum amount of VPD */;
pub const DMP_RSP_OFFSET: c_uint = 0x14   /* word 5 contains first word of rsp */;
pub const DMP_RSP_SIZE: c_uint = 0x6C   /* maximum of 27 words of rsp data */;
pub const DMP_REGION_VPORT: c_uint = 0x16   /* VPort info region */;
pub const DMP_VPORT_REGION_SIZE: c_uint = 0x200;
pub const DMP_MBOX_OFFSET_WORD: c_uint = 0x5;
pub const DMP_REGION_23: c_uint = 0x17   /* fcoe param  and port state region */;
pub const DMP_RGN23_SIZE: c_uint = 0x400;
pub const WAKE_UP_PARMS_REGION_ID: c_int = 4;
pub const WAKE_UP_PARMS_WORD_SIZE: c_int = 15;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vport_rec {
    pub wwpn: [u8; 8],
    pub wwnn: [u8; 8],
}

pub const VPORT_INFO_SIG: c_uint = 0x32324752;
pub const VPORT_INFO_REV_MASK: c_uint = 0xff;
pub const VPORT_INFO_REV: c_uint = 0x1;
pub const MAX_STATIC_VPORT_COUNT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct static_vport_info {
    pub signature: u32,
    pub rev: u32,
    pub vport_list: [vport_rec; MAX_STATIC_VPORT_COUNT],
    pub resvd: [u32; 66],
}

// Option rom version structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prog_id {

    pub type: u8,
    pub id: u8,
    pub /: *mut *mut uint32_t ver:4; / Major Version,
    pub /: *mut *mut uint32_t rev:4; / Revision,
    pub /: *mut *mut uint32_t lev:2; / Level,
    pub /: *mut *mut uint32_t dist:2; / Dist Type,
    pub /: *mut *mut uint32_t num:4; / number after dist type,

    pub /: *mut *mut uint32_t num:4; / number after dist type,
    pub /: *mut *mut uint32_t dist:2; / Dist Type,
    pub /: *mut *mut uint32_t lev:2; / Level,
    pub /: *mut *mut uint32_t rev:4; / Revision,
    pub /: *mut *mut uint32_t ver:4; / Major Version,
    pub id: u8,
    pub type: u8,

}

// Structure for MB Command UPDATE_CFG (0x1B)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct update_cfg_var {

    pub rsvd2:16: u32,
    pub type:8: u32,
    pub rsvd:1: u32,
    pub ra:1: u32,
    pub co:1: u32,
    pub cv:1: u32,
    pub req:4: u32,
    pub entry_length:16: u32,
    pub region_id:16: u32,

    pub req:4: u32,
    pub cv:1: u32,
    pub co:1: u32,
    pub ra:1: u32,
    pub rsvd:1: u32,
    pub type:8: u32,
    pub rsvd2:16: u32,
    pub region_id:16: u32,
    pub entry_length:16: u32,

    pub resp_info: u32,
    pub byte_cnt: u32,
    pub data_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbq_mask {

    pub tmatch: u8,
    pub tmask: u8,
    pub rctlmatch: u8,
    pub rctlmask: u8,

    pub rctlmask: u8,
    pub rctlmatch: u8,
    pub tmask: u8,
    pub tmatch: u8,

}

// Structure for MB Command CONFIG_HBQ (7c)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct config_hbq_var {

    pub :7: uint32_t rsvd1,
    pub /: *mut *mut uint32_t recvNotify :1; / Receive Notification,
    pub /: *mut *mut uint32_t numMask :8; / # Mask Entries,
    pub /: *mut *mut uint32_t profile :8; / Selection Profile,
    pub :8: uint32_t rsvd2,

    pub :8: uint32_t rsvd2,
    pub /: *mut *mut uint32_t profile :8; / Selection Profile,
    pub /: *mut *mut uint32_t numMask :8; / # Mask Entries,
    pub /: *mut *mut uint32_t recvNotify :1; / Receive Notification,
    pub :7: uint32_t rsvd1,

    pub :16: uint32_t hbqId,
    pub :12: uint32_t rsvd3,
    pub :4: uint32_t ringMask,

    pub :4: uint32_t ringMask,
    pub :12: uint32_t rsvd3,
    pub :16: uint32_t hbqId,

    pub :16: uint32_t entry_count,
    pub :8: uint32_t rsvd4,
    pub :8: uint32_t headerLen,

    pub :8: uint32_t headerLen,
    pub :8: uint32_t rsvd4,
    pub :16: uint32_t entry_count,

    pub hbqaddrLow: u32,
    pub hbqaddrHigh: u32,

    pub :31: uint32_t rsvd5,
    pub :1: uint32_t logEntry,

    pub :1: uint32_t logEntry,
    pub :31: uint32_t rsvd5,

    pub /: *mut *mut uint32_t rsvd6; / w7,
    pub /: *mut *mut uint32_t rsvd7; / w8,
    pub /: *mut *mut uint32_t rsvd8; / w9,
    pub hbqMasks: [hbq_mask; 6],
    pub allprofiles: [u32; 12],
    pub :16: uint32_t seqlenoff,
    pub :16: uint32_t maxlen,

    pub :16: uint32_t maxlen,
    pub :16: uint32_t seqlenoff,

    pub :28: uint32_t rsvd1,
    pub :4: uint32_t seqlenbcnt,

    pub :4: uint32_t seqlenbcnt,
    pub :28: uint32_t rsvd1,
    pub rsvd: [u32; 10],
    pub profile2: },

    pub :16: uint32_t seqlenoff,
    pub :16: uint32_t maxlen,

    pub :16: uint32_t maxlen,
    pub :16: uint32_t seqlenoff,

    pub :28: uint32_t cmdcodeoff,
    pub :12: uint32_t rsvd1,
    pub :4: uint32_t seqlenbcnt,

    pub :4: uint32_t seqlenbcnt,
    pub :12: uint32_t rsvd1,
    pub :28: uint32_t cmdcodeoff,
    pub cmdmatch: [u32; 8],
    pub rsvd: [u32; 2],
    pub profile3: },

    pub :16: uint32_t seqlenoff,
    pub :16: uint32_t maxlen,

    pub :16: uint32_t maxlen,
    pub :16: uint32_t seqlenoff,

    pub :28: uint32_t cmdcodeoff,
    pub :12: uint32_t rsvd1,
    pub :4: uint32_t seqlenbcnt,

    pub :4: uint32_t seqlenbcnt,
    pub :12: uint32_t rsvd1,
    pub :28: uint32_t cmdcodeoff,
    pub cmdmatch: [u32; 8],
    pub rsvd: [u32; 2],
    pub profile5: },
    pub profiles: },
}

// Structure for MB Command CONFIG_PORT (0x88)

// config block

// config block

// Structure for MB Command CONFIG_MSI (0x30)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct config_msi_var {

    pub /: *mut *mut uint32_t dfltMsgNum:8; / Default message number,
    pub /: *mut *mut uint32_t rsvd1:11; / Reserved,
    pub /: *mut *mut uint32_t NID:5; / Number of secondary attention IDs,
    pub /: *mut *mut uint32_t rsvd2:5; / Reserved,
    pub /: *mut *mut uint32_t dfltPresent:1; / Default message number present,
    pub /: *mut *mut uint32_t addFlag:1; / Add association flag,
    pub /: *mut *mut uint32_t reportFlag:1; / Report association flag,

    pub /: *mut *mut uint32_t reportFlag:1; / Report association flag,
    pub /: *mut *mut uint32_t addFlag:1; / Add association flag,
    pub /: *mut *mut uint32_t dfltPresent:1; / Default message number present,
    pub /: *mut *mut uint32_t rsvd2:5; / Reserved,
    pub /: *mut *mut uint32_t NID:5; / Number of secondary attention IDs,
    pub /: *mut *mut uint32_t rsvd1:11; / Reserved,
    pub /: *mut *mut uint32_t dfltMsgNum:8; / Default message number,
    pub attentionConditions: [u32; 2],
    pub attentionId: [u8; 16],
    pub messageNumberByHA: [u8; 64],
    pub messageNumberByID: [u8; 16],
    pub autoClearHA: [u32; 2],
    pub rsvd3:16: u32,
    pub autoClearID:16: u32,

    pub autoClearID:16: u32,
    pub rsvd3:16: u32,

    pub rsvd4: u32,
}

// SLI-2 Port Control Block
// SLIM POINTER
pub const SLIMOFF: c_uint = 0x30		/* WORD */;

pub const TYPE_NATIVE_SLI2: c_uint = 0x01;
pub const FEATURE_INITIAL_SLI2: c_uint = 0x01;

pub const FEATURE_INITIAL_SLI2: c_uint = 0x01;
pub const TYPE_NATIVE_SLI2: c_uint = 0x01;

// NEW_FEATURE

// Structure for MB Command MBX_ASYNCEVT_ENABLE (0x33)

// Union of all Mailbox Command types
pub const MAILBOX_CMD_WSIZE: c_int = 32;

// ext_wsize times 4 bytes should not be greater than max xmit size
pub const MAILBOX_EXT_WSIZE: c_int = 512;

pub const MAILBOX_HBA_EXT_OFFSET: c_uint = 0x100;
// max mbox xmit size is a page size for sysfs IO operations
pub const MAILBOX_SYSFS_MAX: c_int = 4096;
// feature/max ring number
//
// NEW_FEATURE
//
// (READ_EVENT_LOG)
//
// SLI-2 specific structures
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_hgp {
    pub cmdPutInx: __le32,
    pub rspGetInx: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_pgp {
    pub cmdGetInx: __le32,
    pub rspPutInx: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli2_desc {
    pub unused1: [u32; 16],
    pub host: [lpfc_hgp; MAX_SLI3_RINGS],
    pub port: [lpfc_pgp; MAX_SLI3_RINGS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli3_desc {
    pub host: [lpfc_hgp; MAX_SLI3_RINGS],
    pub reserved: [u32; 8],
    pub hbq_put: [u32; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli3_pgp {
    pub port: [lpfc_pgp; MAX_SLI3_RINGS],
    pub hbq_get: [u32; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union sli_var {
    pub s2: sli2_desc,
    pub s3: sli3_desc,
    pub s3_pgp: sli3_pgp,
}

//
// Begin Structure Definitions for IOCB Commands
//

// statRsn  P/F_RJT reason codes
pub const RJT_BAD_D_ID: c_uint = 0x01	/* Invalid D_ID field */;
pub const RJT_BAD_S_ID: c_uint = 0x02	/* Invalid S_ID field */;
pub const RJT_UNAVAIL_TEMP: c_uint = 0x03	/* N_Port unavailable temp. */;
pub const RJT_UNAVAIL_PERM: c_uint = 0x04	/* N_Port unavailable perm. */;
pub const RJT_UNSUP_CLASS: c_uint = 0x05	/* Class not supported */;
pub const RJT_DELIM_ERR: c_uint = 0x06	/* Delimiter usage error */;
pub const RJT_UNSUP_TYPE: c_uint = 0x07	/* Type not supported */;
pub const RJT_BAD_CONTROL: c_uint = 0x08	/* Invalid link conrtol */;
pub const RJT_BAD_RCTL: c_uint = 0x09	/* R_CTL invalid */;
pub const RJT_BAD_FCTL: c_uint = 0x0A	/* F_CTL invalid */;
pub const RJT_BAD_OXID: c_uint = 0x0B	/* OX_ID invalid */;
pub const RJT_BAD_RXID: c_uint = 0x0C	/* RX_ID invalid */;
pub const RJT_BAD_SEQID: c_uint = 0x0D	/* SEQ_ID invalid */;
pub const RJT_BAD_DFCTL: c_uint = 0x0E	/* DF_CTL invalid */;
pub const RJT_BAD_SEQCNT: c_uint = 0x0F	/* SEQ_CNT invalid */;
pub const RJT_BAD_PARM: c_uint = 0x10	/* Param. field invalid */;
pub const RJT_XCHG_ERR: c_uint = 0x11	/* Exchange error */;
pub const RJT_PROT_ERR: c_uint = 0x12	/* Protocol error */;
pub const RJT_BAD_LENGTH: c_uint = 0x13	/* Invalid Length */;
pub const RJT_UNEXPECTED_ACK: c_uint = 0x14	/* Unexpected ACK */;
pub const RJT_LOGIN_REQUIRED: c_uint = 0x16	/* Login required */;
pub const RJT_TOO_MANY_SEQ: c_uint = 0x17	/* Excessive sequences */;
pub const RJT_XCHG_NOT_STRT: c_uint = 0x18	/* Exchange not started */;
pub const RJT_UNSUP_SEC_HDR: c_uint = 0x19	/* Security hdr not supported */;
pub const RJT_UNAVAIL_PATH: c_uint = 0x1A	/* Fabric Path not available */;
pub const RJT_VENDOR_UNIQUE: c_uint = 0xFF	/* Vendor unique error */;
pub const IOERR_SUCCESS: c_uint = 0x00	/* statLocalError */;
pub const IOERR_MISSING_CONTINUE: c_uint = 0x01;
pub const IOERR_SEQUENCE_TIMEOUT: c_uint = 0x02;
pub const IOERR_INTERNAL_ERROR: c_uint = 0x03;
pub const IOERR_INVALID_RPI: c_uint = 0x04;
pub const IOERR_NO_XRI: c_uint = 0x05;
pub const IOERR_ILLEGAL_COMMAND: c_uint = 0x06;
pub const IOERR_XCHG_DROPPED: c_uint = 0x07;
pub const IOERR_ILLEGAL_FIELD: c_uint = 0x08;
pub const IOERR_RPI_SUSPENDED: c_uint = 0x09;
pub const IOERR_TOO_MANY_BUFFERS: c_uint = 0x0A;
pub const IOERR_RCV_BUFFER_WAITING: c_uint = 0x0B;
pub const IOERR_NO_CONNECTION: c_uint = 0x0C;
pub const IOERR_TX_DMA_FAILED: c_uint = 0x0D;
pub const IOERR_RX_DMA_FAILED: c_uint = 0x0E;
pub const IOERR_ILLEGAL_FRAME: c_uint = 0x0F;
pub const IOERR_EXTRA_DATA: c_uint = 0x10;
pub const IOERR_NO_RESOURCES: c_uint = 0x11;
pub const IOERR_RESERVED: c_uint = 0x12;
pub const IOERR_ILLEGAL_LENGTH: c_uint = 0x13;
pub const IOERR_UNSUPPORTED_FEATURE: c_uint = 0x14;
pub const IOERR_ABORT_IN_PROGRESS: c_uint = 0x15;
pub const IOERR_ABORT_REQUESTED: c_uint = 0x16;
pub const IOERR_RECEIVE_BUFFER_TIMEOUT: c_uint = 0x17;
pub const IOERR_LOOP_OPEN_FAILURE: c_uint = 0x18;
pub const IOERR_RING_RESET: c_uint = 0x19;
pub const IOERR_LINK_DOWN: c_uint = 0x1A;
pub const IOERR_CORRUPTED_DATA: c_uint = 0x1B;
pub const IOERR_CORRUPTED_RPI: c_uint = 0x1C;
pub const IOERR_OUT_OF_ORDER_DATA: c_uint = 0x1D;
pub const IOERR_OUT_OF_ORDER_ACK: c_uint = 0x1E;
pub const IOERR_DUP_FRAME: c_uint = 0x1F;
pub const IOERR_LINK_CONTROL_FRAME: c_uint = 0x20	/* ACK_N received */;
pub const IOERR_BAD_HOST_ADDRESS: c_uint = 0x21;
pub const IOERR_RCV_HDRBUF_WAITING: c_uint = 0x22;
pub const IOERR_MISSING_HDR_BUFFER: c_uint = 0x23;
pub const IOERR_MSEQ_CHAIN_CORRUPTED: c_uint = 0x24;
pub const IOERR_ABORTMULT_REQUESTED: c_uint = 0x25;
pub const IOERR_BUFFER_SHORTAGE: c_uint = 0x28;
pub const IOERR_DEFAULT: c_uint = 0x29;
pub const IOERR_CNT: c_uint = 0x2A;
pub const IOERR_SLER_FAILURE: c_uint = 0x46;
pub const IOERR_SLER_CMD_RCV_FAILURE: c_uint = 0x47;
pub const IOERR_SLER_REC_RJT_ERR: c_uint = 0x48;
pub const IOERR_SLER_REC_SRR_RETRY_ERR: c_uint = 0x49;
pub const IOERR_SLER_SRR_RJT_ERR: c_uint = 0x4A;
pub const IOERR_SLER_RRQ_RJT_ERR: c_uint = 0x4C;
pub const IOERR_SLER_RRQ_RETRY_ERR: c_uint = 0x4D;
pub const IOERR_SLER_ABTS_ERR: c_uint = 0x4E;
pub const IOERR_ELXSEC_KEY_UNWRAP_ERROR: c_uint = 0xF0;
pub const IOERR_ELXSEC_KEY_UNWRAP_COMPARE_ERROR: c_uint = 0xF1;
pub const IOERR_ELXSEC_CRYPTO_ERROR: c_uint = 0xF2;
pub const IOERR_ELXSEC_CRYPTO_COMPARE_ERROR: c_uint = 0xF3;
pub const IOERR_DRVR_MASK: c_uint = 0x100;
pub const IOERR_SLI_DOWN: c_uint = 0x101  /* ulpStatus  - Driver defined */;
pub const IOERR_SLI_BRESET: c_uint = 0x102;
pub const IOERR_SLI_ABORTED: c_uint = 0x103;
pub const IOERR_PARAM_MASK: c_uint = 0x1ff;

pub const BC: c_uint = 0x02		/* Broadcast Received  - Fctl */;
pub const SI: c_uint = 0x04		/* Sequence Initiative */;
pub const LA: c_uint = 0x08		/* Ignore Link Attention state */;
pub const LS: c_uint = 0x80		/* Last Sequence */;
// IOCB Command template for a generic response
// IOCB Command template for XMIT / XMIT_BCAST / RCV_SEQUENCE / XMIT_ELS
// IOCB Command template for ELS_REQUEST

// IOCB Command template for RCV_ELS_REQ

// IOCB Command template for ABORT / CLOSE_XRI
pub const ABORT_TYPE_ABTX: c_uint = 0x00000000;
pub const ABORT_TYPE_ABTS: c_uint = 0x00000001;

// IOCB Command template for ABORT_MXRI64
// IOCB Command template for GET_RPI

// IOCB Command template for all FCP Initiator commands
// IOCB Command template for all FCP Target commands
// SLI-2 IOCB structure definitions
// IOCB Command template for 64 bit XMIT / XMIT_BCAST / XMIT_ELS
// This word is remote ports D_ID for XMIT_ELS_RSP64

// IOCB Command template for 64 bit RCV_SEQUENCE64
// IOCB Command template for ELS_REQUEST64

// IOCB Command template for GEN_REQUEST64
// IOCB Command template for RCV_ELS_REQ64

// IOCB Command template for RCV_SEQ64
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcv_seq64 {
    pub elsReq: ulp_bde64,
    pub hbq_1: u32,
    pub parmRo: u32,

    pub rctl:8: u32,
    pub type:8: u32,
    pub dfctl:8: u32,
    pub ls:1: u32,
    pub fs:1: u32,
    pub rsvd2:3: u32,
    pub si:1: u32,
    pub bc:1: u32,
    pub rsvd3:1: u32,

    pub rsvd3:1: u32,
    pub bc:1: u32,
    pub si:1: u32,
    pub rsvd2:3: u32,
    pub fs:1: u32,
    pub ls:1: u32,
    pub dfctl:8: u32,
    pub type:8: u32,
    pub rctl:8: u32,

}

// IOCB Command template for all 64 bit FCP Initiator commands
// IOCB Command template for all 64 bit FCP Target commands
// IOCB Command template for Async Status iocb commands

pub const ASYNC_TEMP_WARN: c_uint = 0x100;
pub const ASYNC_TEMP_SAFE: c_uint = 0x101;
pub const ASYNC_STATUS_CN: c_uint = 0x102;
// IOCB Command template for CMD_IOCB_RCV_ELS64_CX (0xB7)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcv_sli3 {

    pub ox_id: u16,
    pub seq_cnt: u16,
    pub vpi: u16,
    pub word9Rsvd: u16,

    pub seq_cnt: u16,
    pub ox_id: u16,
    pub word9Rsvd: u16,
    pub vpi: u16,

    pub word10Rsvd: u32,
    pub /: *mut *mut uint32_t acc_len; / accumulated length,
    pub bde2: ulp_bde64,
}

// Structure used for a single HBQ entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_hbq_entry {
    pub bde: ulp_bde64,
    pub buffer_tag: u32,
}

// IOCB Command template for QUE_XRI64_CX (0xB3) command
#[repr(C)]
#[derive(Copy, Clone)]
pub struct que_xri64cx_ext_fields {
    pub iotag64_low: u32,
    pub iotag64_high: u32,
    pub ebde_count: u32,
    pub rsvd: u32,
    pub buff: [lpfc_hbq_entry; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli3_bg_fields {
    pub /: *mut *mut uint32_t filler[6]; / word 8-13 in IOCB,
    pub /: *mut *mut uint32_t bghm; / word 14 - BlockGuard High Water Mark,
// Bitfields for bgstat (BlockGuard Status - word 15 of IOCB)
pub const BGS_BIDIR_BG_PROF_MASK: c_uint = 0xff000000;
pub const BGS_BIDIR_BG_PROF_SHIFT: c_int = 24;
pub const BGS_BIDIR_ERR_COND_FLAGS_MASK: c_uint = 0x003f0000;
pub const BGS_BIDIR_ERR_COND_SHIFT: c_int = 16;
pub const BGS_BG_PROFILE_MASK: c_uint = 0x0000ff00;
pub const BGS_BG_PROFILE_SHIFT: c_int = 8;
pub const BGS_INVALID_PROF_MASK: c_uint = 0x00000020;
pub const BGS_INVALID_PROF_SHIFT: c_int = 5;
pub const BGS_UNINIT_DIF_BLOCK_MASK: c_uint = 0x00000010;
pub const BGS_UNINIT_DIF_BLOCK_SHIFT: c_int = 4;
pub const BGS_HI_WATER_MARK_PRESENT_MASK: c_uint = 0x00000008;
pub const BGS_HI_WATER_MARK_PRESENT_SHIFT: c_int = 3;
pub const BGS_REFTAG_ERR_MASK: c_uint = 0x00000004;
pub const BGS_REFTAG_ERR_SHIFT: c_int = 2;
pub const BGS_APPTAG_ERR_MASK: c_uint = 0x00000002;
pub const BGS_APPTAG_ERR_SHIFT: c_int = 1;
pub const BGS_GUARD_ERR_MASK: c_uint = 0x00000001;
pub const BGS_GUARD_ERR_SHIFT: c_int = 0;
    pub /: *mut *mut uint32_t bgstat; / word 15 - BlockGuard Status,
}

pub const LPFC_EXT_DATA_BDE_COUNT: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcp_irw_ext {
    pub io_tag64_low: u32,
    pub io_tag64_high: u32,

    pub reserved1: u8,
    pub reserved2: u8,
    pub reserved3: u8,
    pub ebde_count: u8,

    pub ebde_count: u8,
    pub reserved3: u8,
    pub reserved2: u8,
    pub reserved1: u8,

    pub reserved4: u32,
    pub /: *mut *mut ulp_bde64 rbde; / response bde,
    pub /: *mut *mut ulp_bde64 dbde[LPFC_EXT_DATA_BDE_COUNT]; / data BDE or BPL,
    pub /: *mut *mut uint8_t icd[32]; / immediate command data (32 bytes),
}

// SLI-2 structures
// bde_64s

// words 8-31 used for que_xri_cx iocb
// words 8-15 for BlockGuard

pub const PARM_NPIV_DID: c_int = 3;

pub const IOSTAT_SUCCESS: c_uint = 0x0	/* ulpStatus  - HBA defined */;
pub const IOSTAT_FCP_RSP_ERROR: c_uint = 0x1;
pub const IOSTAT_REMOTE_STOP: c_uint = 0x2;
pub const IOSTAT_LOCAL_REJECT: c_uint = 0x3;
pub const IOSTAT_NPORT_RJT: c_uint = 0x4;
pub const IOSTAT_FABRIC_RJT: c_uint = 0x5;
pub const IOSTAT_NPORT_BSY: c_uint = 0x6;
pub const IOSTAT_FABRIC_BSY: c_uint = 0x7;
pub const IOSTAT_INTERMED_RSP: c_uint = 0x8;
pub const IOSTAT_LS_RJT: c_uint = 0x9;
pub const IOSTAT_BA_RJT: c_uint = 0xA;
pub const IOSTAT_RSVD1: c_uint = 0xB;
pub const IOSTAT_RSVD2: c_uint = 0xC;
pub const IOSTAT_RSVD3: c_uint = 0xD;
pub const IOSTAT_RSVD4: c_uint = 0xE;
pub const IOSTAT_NEED_BUFFER: c_uint = 0xF;
pub const IOSTAT_DRIVER_REJECT: c_uint = 0x10   /* ulpStatus  - Driver defined */;
pub const IOSTAT_DEFAULT: c_uint = 0xF    /* Same as rsvd5 for now */;
pub const IOSTAT_CNT: c_uint = 0x11;

// Up to 498 IOCBs will fit into 16k
// 256 (MAILBOX_t) + 140 (PCB_t) + ( 32 (IOCB_t) * 498 ) = < 16384
//

// Maximum IOCBs that will fit in SLI2 slim
pub const MAX_SLI2_IOCB: c_int = 498;

// HBQ entries are 4 words each = 4k

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_sli2_slim {
    pub mbx: MAILBOX_t,
    pub mbx_ext_words: [u32; MAILBOX_EXT_WSIZE],
    pub pcb: PCB_t,
    pub IOCBs: [IOCB_t; MAX_SLIM_IOCB_SIZE],
}

//
// This function checks PCI device to allow special handling for LC HBAs.
//
// Parameters:
// device : struct pci_dev 's device field
//
// return 1 => TRUE
// 0 => FALSE
//
