//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/scsi/fc/fc_els.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Copyright(c) 2007 Intel Corporation. All rights reserved.
//
// Maintained at www.Open-FCoE.org
//

//
// Fibre Channel Switch - Enhanced Link Services definitions.
// From T11 FC-LS Rev 1.2 June 7, 2005.
//
// ELS Command codes - byte 0 of the frame payload
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_els_cmd {
    ELS_LS_RJT =	0x01,	/* ESL reject */
    ELS_LS_ACC =	0x02,	/* ESL Accept */
    ELS_PLOGI =	0x03,	/* N_Port login */
    ELS_FLOGI =	0x04,	/* F_Port login */
    ELS_LOGO =	0x05,	/* Logout */
    ELS_ABTX =	0x06,	/* Abort exchange - obsolete */
    ELS_RCS =	0x07,	/* read connection status */
    ELS_RES =	0x08,	/* read exchange status block */
    ELS_RSS =	0x09,	/* read sequence status block */
    ELS_RSI =	0x0a,	/* read sequence initiative */
    ELS_ESTS =	0x0b,	/* establish streaming */
    ELS_ESTC =	0x0c,	/* estimate credit */
    ELS_ADVC =	0x0d,	/* advise credit */
    ELS_RTV =	0x0e,	/* read timeout value */
    ELS_RLS =	0x0f,	/* read link error status block */
    ELS_ECHO =	0x10,	/* echo */
    ELS_TEST =	0x11,	/* test */
    ELS_RRQ =	0x12,	/* reinstate recovery qualifier */
    ELS_REC =	0x13,	/* read exchange concise */
    ELS_SRR =	0x14,	/* sequence retransmission request */
    ELS_FPIN =	0x16,	/* Fabric Performance Impact Notification */
    ELS_EDC =	0x17,	/* Exchange Diagnostic Capabilities */
    ELS_RDP =	0x18,	/* Read Diagnostic Parameters */
    ELS_RDF =	0x19,	/* Register Diagnostic Functions */
    ELS_PRLI =	0x20,	/* process login */
    ELS_PRLO =	0x21,	/* process logout */
    ELS_SCN =	0x22,	/* state change notification */
    ELS_TPLS =	0x23,	/* test process login state */
    ELS_TPRLO =	0x24,	/* third party process logout */
    ELS_LCLM =	0x25,	/* login control list mgmt (obs) */
    ELS_GAID =	0x30,	/* get alias_ID */
    ELS_FACT =	0x31,	/* fabric activate alias_id */
    ELS_FDACDT =	0x32,	/* fabric deactivate alias_id */
    ELS_NACT =	0x33,	/* N-port activate alias_id */
    ELS_NDACT =	0x34,	/* N-port deactivate alias_id */
    ELS_QOSR =	0x40,	/* quality of service request */
    ELS_RVCS =	0x41,	/* read virtual circuit status */
    ELS_PDISC =	0x50,	/* discover N_port service params */
    ELS_FDISC =	0x51,	/* discover F_port service params */
    ELS_ADISC =	0x52,	/* discover address */
    ELS_RNC =	0x53,	/* report node cap (obs) */
    ELS_FARP_REQ =	0x54,	/* FC ARP request */
    ELS_FARP_REPL =	0x55,	/* FC ARP reply */
    ELS_RPS =	0x56,	/* read port status block */
    ELS_RPL =	0x57,	/* read port list */
    ELS_RPBC =	0x58,	/* read port buffer condition */
    ELS_FAN =	0x60,	/* fabric address notification */
    ELS_RSCN =	0x61,	/* registered state change notification */
    ELS_SCR =	0x62,	/* state change registration */
    ELS_RNFT =	0x63,	/* report node FC-4 types */
    ELS_CSR =	0x68,	/* clock synch. request */
    ELS_CSU =	0x69,	/* clock synch. update */
    ELS_LINIT =	0x70,	/* loop initialize */
    ELS_LSTS =	0x72,	/* loop status */
    ELS_RNID =	0x78,	/* request node ID data */
    ELS_RLIR =	0x79,	/* registered link incident report */
    ELS_LIRR =	0x7a,	/* link incident record registration */
    ELS_SRL =	0x7b,	/* scan remote loop */
    ELS_SBRP =	0x7c,	/* set bit-error reporting params */
    ELS_RPSC =	0x7d,	/* report speed capabilities */
    ELS_QSA =	0x7e,	/* query security attributes */
    ELS_EVFP =	0x7f,	/* exchange virt. fabrics params */
    ELS_LKA =	0x80,	/* link keep-alive */
    ELS_AUTH_ELS =	0x90,	/* authentication ELS */
}

//
// Initializer useful for decoding table.
// Please keep this in sync with the above definitions.
//

//
// LS_ACC payload.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_ls_acc {
    pub /: *mut *mut __u8 la_cmd; / command code ELS_LS_ACC,
    pub /: *mut *mut __u8 la_resv[3]; / reserved,
}

//
// ELS reject payload.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_ls_rjt {
    pub /: *mut *mut __u8 er_cmd; / command code ELS_LS_RJT,
    pub /: *mut *mut __u8 er_resv[4]; / reserved must be zero,
    pub /: *mut *mut __u8 er_reason; / reason (enum fc_els_rjt_reason below),
    pub /: *mut *mut __u8 er_explan; / explanation (enum fc_els_rjt_explan below),
    pub /: *mut *mut __u8 er_vendor; / vendor specific code,
}

//
// ELS reject reason codes (er_reason).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_els_rjt_reason {
    ELS_RJT_NONE =		0,	/* no reject - not to be sent */
    ELS_RJT_INVAL =		0x01,	/* invalid ELS command code */
    ELS_RJT_LOGIC =		0x03,	/* logical error */
    ELS_RJT_BUSY =		0x05,	/* logical busy */
    ELS_RJT_PROT =		0x07,	/* protocol error */
    ELS_RJT_UNAB =		0x09,	/* unable to perform command request */
    ELS_RJT_UNSUP =		0x0b,	/* command not supported */
    ELS_RJT_INPROG =	0x0e,	/* command already in progress */
    ELS_RJT_FIP =		0x20,	/* FIP error */
    ELS_RJT_VENDOR =	0xff,	/* vendor specific error */
}

//
// reason code explanation (er_explan).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_els_rjt_explan {
    ELS_EXPL_NONE =		0x00,	/* No additional explanation */
    ELS_EXPL_SPP_OPT_ERR =	0x01,	/* service parameter error - options */
    ELS_EXPL_SPP_ICTL_ERR =	0x03,	/* service parm error - initiator ctl */
    ELS_EXPL_AH =		0x11,	/* invalid association header */
    ELS_EXPL_AH_REQ =	0x13,	/* association_header required */
    ELS_EXPL_SID =		0x15,	/* invalid originator S_ID */
    ELS_EXPL_OXID_RXID =	0x17,	/* invalid OX_ID-RX_ID combination */
    ELS_EXPL_INPROG =	0x19,	/* Request already in progress */
    ELS_EXPL_PLOGI_REQD =	0x1e,	/* N_Port login required */
    ELS_EXPL_INSUF_RES =	0x29,	/* insufficient resources */
    ELS_EXPL_UNAB_DATA =	0x2a,	/* unable to supply requested data */
    ELS_EXPL_UNSUPR =	0x2c,	/* Request not supported */
    ELS_EXPL_INV_LEN =	0x2d,	/* Invalid payload length */
    ELS_EXPL_NOT_NEIGHBOR = 0x62,	/* VN2VN_Port not in neighbor set */
// TBD - above definitions incomplete
}

//
// Link Service TLV Descriptor Tag Values
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_ls_tlv_dtag {
    ELS_DTAG_LS_REQ_INFO =		0x00000001,
// Link Service Request Information Descriptor
    ELS_DTAG_LNK_FAULT_CAP =	0x0001000D,
// Link Fault Capability Descriptor
    ELS_DTAG_CG_SIGNAL_CAP =	0x0001000F,
// Congestion Signaling Capability Descriptor
    ELS_DTAG_LNK_INTEGRITY =	0x00020001,
// Link Integrity Notification Descriptor
    ELS_DTAG_DELIVERY =		0x00020002,
// Delivery Notification Descriptor
    ELS_DTAG_PEER_CONGEST =		0x00020003,
// Peer Congestion Notification Descriptor
    ELS_DTAG_CONGESTION =		0x00020004,
// Congestion Notification Descriptor
    ELS_DTAG_FPIN_REGISTER =	0x00030001,
// FPIN Registration Descriptor
}

//
// Initializer useful for decoding table.
// Please keep this in sync with the above definitions.
//

//
// Generic Link Service TLV Descriptor format
//
// This structure, as it defines no payload, will also be referred to
// as the "tlv header" - which contains the tag and len fields.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_tlv_desc {
    pub /: *mut *mut __be32 desc_tag; / Notification Descriptor Tag,
    pub bytes).: *mut *mut __be32 desc_len; / Length of Descriptor (in,
// Size of descriptor excluding
// desc_tag and desc_len fields.
//
    pub /: *mut *mut __u8 desc_value[]; / Descriptor Value,
}

// Descriptor tag and len fields are considered the mandatory header
// for a descriptor
//

//
// Macro, used when initializing payloads, to return the descriptor length.
// Length is size of descriptor minus the tag and len fields.
//

// Macro, used on received payloads, to return the descriptor length

//
// This helper is used to walk descriptors in a descriptor list.
// Given the address of the current descriptor, which minimally contains a
// tag and len field, calculate the address of the next descriptor based
// on the len field.
//
// Link Service Request Information Descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_lsri_desc {
    pub /: *mut *mut __be32 desc_tag; / descriptor tag (0x0000 0001),
    pub (4).: *mut *mut __be32 desc_len; / Length of Descriptor (in bytes),
// Size of descriptor excluding
// desc_tag and desc_len fields.
//
    pub /: *mut *mut __u8 cmd; / ELS cmd byte,
    pub /: *mut *mut __u8 bytes[3]; / bytes 1..3,
    pub /: *mut *mut } rqst_w0; / Request word 0,
}

//
// Common service parameters (N ports).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_csp {
    pub /: *mut *mut __u8 sp_hi_ver; / highest version supported (obs.),
    pub /: *mut *mut __u8 sp_lo_ver; / highest version supported (obs.),
    pub /: *mut *mut __be16 sp_bb_cred; / buffer-to-buffer credits,
    pub /: *mut *mut __be16 sp_features; / common feature flags,
    pub /: *mut *mut __be16 sp_bb_data; / b-b state number and data field sz,
    pub /: *mut *mut __be16 _sp_tot_seq; / total concurrent sequences,
    pub /: *mut *mut __be16 _sp_rel_off; / rel. offset by info cat,
    pub sp_plogi: },
    pub /: *mut *mut __be32 _sp_r_a_tov; / resource alloc. timeout msec,
    pub sp_flogi_acc: },
    pub sp_u: },
    pub /: *mut *mut __be32 sp_e_d_tov; / error detect timeout value,
}

pub const FC_SP_BB_DATA_MASK: c_uint = 0xfff /* mask for data field size in sp_bb_data */;
//
// Minimum and maximum values for max data field size in service parameters.
//

//
// sp_features
//
pub const FC_SP_FT_NPIV: c_uint = 0x8000	/* multiple N_Port_ID support (FLOGI) */;
pub const FC_SP_FT_CIRO: c_uint = 0x8000	/* continuously increasing rel off (PLOGI) */;
pub const FC_SP_FT_CLAD: c_uint = 0x8000	/* clean address (in FLOGI LS_ACC) */;
pub const FC_SP_FT_RAND: c_uint = 0x4000	/* random relative offset */;
pub const FC_SP_FT_VAL: c_uint = 0x2000	/* valid vendor version level */;
pub const FC_SP_FT_NPIV_ACC: c_uint = 0x2000	/* NPIV assignment (FLOGI LS_ACC) */;
pub const FC_SP_FT_FPORT: c_uint = 0x1000	/* F port (1) vs. N port (0) */;
pub const FC_SP_FT_ABB: c_uint = 0x0800	/* alternate BB_credit management */;
pub const FC_SP_FT_EDTR: c_uint = 0x0400	/* E_D_TOV Resolution is nanoseconds */;
pub const FC_SP_FT_MCAST: c_uint = 0x0200	/* multicast */;
pub const FC_SP_FT_BCAST: c_uint = 0x0100	/* broadcast */;
pub const FC_SP_FT_HUNT: c_uint = 0x0080	/* hunt group */;
pub const FC_SP_FT_SIMP: c_uint = 0x0040	/* dedicated simplex */;
pub const FC_SP_FT_SEC: c_uint = 0x0020	/* reserved for security */;
pub const FC_SP_FT_CSYN: c_uint = 0x0010	/* clock synch. supported */;
pub const FC_SP_FT_RTTOV: c_uint = 0x0008	/* R_T_TOV value 100 uS, else 100 mS */;
pub const FC_SP_FT_HALF: c_uint = 0x0004	/* dynamic half duplex */;
pub const FC_SP_FT_SEQC: c_uint = 0x0002	/* SEQ_CNT */;
pub const FC_SP_FT_PAYL: c_uint = 0x0001	/* FLOGI payload length 256, else 116 */;
//
// Class-specific service parameters.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_cssp {
    pub /: *mut *mut __be16 cp_class; / class flags,
    pub /: *mut *mut __be16 cp_init; / initiator flags,
    pub /: *mut *mut __be16 cp_recip; / recipient flags,
    pub /: *mut *mut __be16 cp_rdfs; / receive data field size,
    pub /: *mut *mut __be16 cp_con_seq; / concurrent sequences,
    pub /: *mut *mut __be16 cp_ee_cred; / N-port end-to-end credit,
    pub /: *mut *mut __u8 cp_resv1; / reserved,
    pub /: *mut *mut __u8 cp_open_seq; / open sequences per exchange,
    pub /: *mut *mut __u8 _cp_resv2[2]; / reserved,
}

//
// cp_class flags.
//
pub const FC_CPC_VALID: c_uint = 0x8000		/* class valid */;
pub const FC_CPC_IMIX: c_uint = 0x4000		/* intermix mode */;
pub const FC_CPC_SEQ: c_uint = 0x0800		/* sequential delivery */;
pub const FC_CPC_CAMP: c_uint = 0x0200		/* camp-on */;
pub const FC_CPC_PRI: c_uint = 0x0080		/* priority */;
//
// cp_init flags.
// (TBD: not all flags defined here).
//
pub const FC_CPI_CSYN: c_uint = 0x0010		/* clock synch. capable */;
//
// cp_recip flags.
//
pub const FC_CPR_CSYN: c_uint = 0x0008		/* clock synch. capable */;
//
// NFC_ELS_FLOGI: Fabric login request.
// NFC_ELS_PLOGI: Port login request (same format).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_flogi {
    pub /: *mut *mut __u8 fl_cmd; / command,
    pub /: *mut *mut __u8 _fl_resvd[3]; / must be zero,
    pub /: *mut *mut fc_els_csp fl_csp; / common service parameters,
    pub /: *mut *mut __be64 fl_wwpn; / port name,
    pub /: *mut *mut __be64 fl_wwnn; / node name,
    pub /: *mut *mut fc_els_cssp fl_cssp[4]; / class 1-4 service parameters,
    pub /: *mut *mut __u8 fl_vend[16]; / vendor version level,
    pub __attribute__((__packed__)): },
//
// Process login service parameter page.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_spp {
    pub /: *mut *mut __u8 spp_type; / type code or common service params,
    pub /: *mut *mut __u8 spp_type_ext; / type code extension,
    pub spp_flags: __u8,
    pub _spp_resvd: __u8,
    pub /: *mut *mut __be32 spp_orig_pa; / originator process associator,
    pub /: *mut *mut __be32 spp_resp_pa; / responder process associator,
    pub /: *mut *mut __be32 spp_params; / service parameters,
}

//
// spp_flags.
//
pub const FC_SPP_OPA_VAL: c_uint = 0x80	/* originator proc. assoc. valid */;
pub const FC_SPP_RPA_VAL: c_uint = 0x40	/* responder proc. assoc. valid */;
pub const FC_SPP_EST_IMG_PAIR: c_uint = 0x20	/* establish image pair */;
pub const FC_SPP_RESP_MASK: c_uint = 0x0f	/* mask for response code (below) */;
//
// SPP response code in spp_flags - lower 4 bits.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_els_spp_resp {
    FC_SPP_RESP_ACK	=	1,	/* request executed */
    FC_SPP_RESP_RES =	2,	/* unable due to lack of resources */
    FC_SPP_RESP_INIT =	3,	/* initialization not complete */
    FC_SPP_RESP_NO_PA = 	4,	/* unknown process associator */
    FC_SPP_RESP_CONF = 	5,	/* configuration precludes image pair */
    FC_SPP_RESP_COND = 	6,	/* request completed conditionally */
    FC_SPP_RESP_MULT = 	7,	/* unable to handle multiple SPPs */
    FC_SPP_RESP_INVL = 	8,	/* SPP is invalid */
}

//
// ELS_RRQ - Reinstate Recovery Qualifier
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_rrq {
    pub /: *mut *mut __u8 rrq_cmd; / command (0x12),
    pub /: *mut *mut __u8 rrq_zero[3]; / specified as zero - part of cmd,
    pub /: *mut *mut __u8 rrq_resvd; / reserved,
    pub /: *mut *mut __u8 rrq_s_id[3]; / originator FID,
    pub /: *mut *mut __be16 rrq_ox_id; / originator exchange ID,
    pub /: *mut *mut __be16 rrq_rx_id; / responders exchange ID,
}

//
// ELS_REC - Read exchange concise.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_rec {
    pub /: *mut *mut __u8 rec_cmd; / command (0x13),
    pub /: *mut *mut __u8 rec_zero[3]; / specified as zero - part of cmd,
    pub /: *mut *mut __u8 rec_resvd; / reserved,
    pub /: *mut *mut __u8 rec_s_id[3]; / originator FID,
    pub /: *mut *mut __be16 rec_ox_id; / originator exchange ID,
    pub /: *mut *mut __be16 rec_rx_id; / responders exchange ID,
}

//
// ELS_REC LS_ACC payload.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_rec_acc {
    pub /: *mut *mut __u8 reca_cmd; / accept (0x02),
    pub /: *mut *mut __u8 reca_zero[3]; / specified as zero - part of cmd,
    pub /: *mut *mut __be16 reca_ox_id; / originator exchange ID,
    pub /: *mut *mut __be16 reca_rx_id; / responders exchange ID,
    pub /: *mut *mut __u8 reca_resvd1; / reserved,
    pub /: *mut *mut __u8 reca_ofid[3]; / originator FID,
    pub /: *mut *mut __u8 reca_resvd2; / reserved,
    pub /: *mut *mut __u8 reca_rfid[3]; / responder FID,
    pub /: *mut *mut __be32 reca_fc4value; / FC4 value,
    pub /: *mut *mut __be32 reca_e_stat; / ESB (exchange status block) status,
}

//
// ELS_PRLI - Process login request and response.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_prli {
    pub /: *mut *mut __u8 prli_cmd; / command,
    pub /: *mut *mut __u8 prli_spp_len; / length of each serv. parm. page,
    pub /: *mut *mut __be16 prli_len; / length of entire payload,
// service parameter pages follow
}

//
// ELS_PRLO - Process logout request and response.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_prlo {
    pub /: *mut *mut __u8 prlo_cmd; / command,
    pub /: *mut *mut __u8 prlo_obs; / obsolete, but shall be set to 10h,
    pub /: *mut *mut __be16 prlo_len; / payload length,
}

//
// ELS_ADISC payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_adisc {
    pub adisc_cmd: __u8,
    pub adisc_resv: [__u8; 3],
    pub adisc_resv1: __u8,
    pub adisc_hard_addr: [__u8; 3],
    pub adisc_wwpn: __be64,
    pub adisc_wwnn: __be64,
    pub adisc_resv2: __u8,
    pub adisc_port_id: [__u8; 3],
    pub __attribute__((__packed__)): },
//
// ELS_LOGO - process or fabric logout.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_logo {
    pub /: *mut *mut __u8 fl_cmd; / command code,
    pub /: *mut *mut __u8 fl_zero[3]; / specified as zero - part of cmd,
    pub /: *mut *mut __u8 fl_resvd; / reserved,
    pub /: *mut *mut __u8 fl_n_port_id[3];/ N port ID,
    pub /: *mut *mut __be64 fl_n_port_wwn; / port name,
}

//
// ELS_RTV - read timeout value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_rtv {
    pub /: *mut *mut __u8 rtv_cmd; / command code 0x0e,
    pub /: *mut *mut __u8 rtv_zero[3]; / specified as zero - part of cmd,
}

//
// LS_ACC for ELS_RTV - read timeout value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_rtv_acc {
    pub /: *mut *mut __u8 rtv_cmd; / command code 0x02,
    pub /: *mut *mut __u8 rtv_zero[3]; / specified as zero - part of cmd,
    pub /: *mut *mut __be32 rtv_r_a_tov; / resource allocation timeout value,
    pub /: *mut *mut __be32 rtv_e_d_tov; / error detection timeout value,
    pub /: *mut *mut __be32 rtv_toq; / timeout qualifier (see below),
}

//
// rtv_toq bits.
//

//
// ELS_SCR - state change registration payload.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_scr {
    pub /: *mut *mut __u8 scr_cmd; / command code,
    pub /: *mut *mut __u8 scr_resv[6]; / reserved,
    pub /: *mut *mut __u8 scr_reg_func; / registration function (see below),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_els_scr_func {
    ELS_SCRF_FAB =	1,	/* fabric-detected registration */
    ELS_SCRF_NPORT = 2,	/* Nx_Port-detected registration */
    ELS_SCRF_FULL =	3,	/* full registration */
    ELS_SCRF_CLEAR = 255,	/* remove any current registrations */
}

//
// ELS_RSCN - registered state change notification payload.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_rscn {
    pub /: *mut *mut __u8 rscn_cmd; / RSCN opcode (0x61),
    pub /: *mut *mut __u8 rscn_page_len; / page length (4),
    pub /: *mut *mut __be16 rscn_plen; / payload length including this word,
// followed by 4-byte generic affected Port_ID pages
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_rscn_page {
    pub /: *mut *mut __u8 rscn_page_flags; / event and address format,
    pub /: *mut *mut __u8 rscn_fid[3]; / fabric ID,
}

pub const ELS_RSCN_EV_QUAL_MASK: c_uint = 0xf	/* mask for event qualifier */;

pub const ELS_RSCN_ADDR_FMT_MASK: c_uint = 0x3	/* mask for address format */;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_els_rscn_ev_qual {
    ELS_EV_QUAL_NONE = 0,		/* unspecified */
    ELS_EV_QUAL_NS_OBJ = 1,		/* changed name server object */
    ELS_EV_QUAL_PORT_ATTR = 2,	/* changed port attribute */
    ELS_EV_QUAL_SERV_OBJ = 3,	/* changed service object */
    ELS_EV_QUAL_SW_CONFIG = 4,	/* changed switch configuration */
    ELS_EV_QUAL_REM_OBJ = 5,	/* removed object */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_els_rscn_addr_fmt {
    ELS_ADDR_FMT_PORT = 0,	/* rscn_fid is a port address */
    ELS_ADDR_FMT_AREA = 1,	/* rscn_fid is a area address */
    ELS_ADDR_FMT_DOM = 2,	/* rscn_fid is a domain address */
    ELS_ADDR_FMT_FAB = 3,	/* anything on fabric may have changed */
}

//
// ELS_RNID - request Node ID.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_rnid {
    pub /: *mut *mut __u8 rnid_cmd; / RNID opcode (0x78),
    pub /: *mut *mut __u8 rnid_resv[3]; / reserved,
    pub /: *mut *mut __u8 rnid_fmt; / data format,
    pub /: *mut *mut __u8 rnid_resv2[3]; / reserved,
}

//
// Node Identification Data formats (rnid_fmt)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_els_rnid_fmt {
    ELS_RNIDF_NONE = 0,		/* no specific identification data */
    ELS_RNIDF_GEN = 0xdf,		/* general topology discovery format */
}

//
// ELS_RNID response.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_rnid_resp {
    pub /: *mut *mut __u8 rnid_cmd; / response code (LS_ACC),
    pub /: *mut *mut __u8 rnid_resv[3]; / reserved,
    pub /: *mut *mut __u8 rnid_fmt; / data format,
    pub /: *mut *mut __u8 rnid_cid_len; / common ID data length,
    pub /: *mut *mut __u8 rnid_resv2; / reserved,
    pub /: *mut *mut __u8 rnid_sid_len; / specific ID data length,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_rnid_cid {
    pub /: *mut *mut __be64 rnid_wwpn; / N port name,
    pub /: *mut *mut __be64 rnid_wwnn; / node name,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_rnid_gen {
    pub /: *mut *mut __u8 rnid_vend_id[16]; / vendor-unique ID,
    pub /: *mut *mut __be32 rnid_atype; / associated type (see below),
    pub /: *mut *mut __be32 rnid_phys_port; / physical port number,
    pub /: *mut *mut __be32 rnid_att_nodes; / number of attached nodes,
    pub /: *mut *mut __u8 rnid_node_mgmt; / node management (see below),
    pub /: *mut *mut __u8 rnid_ip_ver; / IP version (see below),
    pub /: *mut *mut __be16 rnid_prot_port; / UDP / TCP port number,
    pub /: *mut *mut __be32 rnid_ip_addr[4]; / IP address,
    pub /: *mut *mut __u8 rnid_resvd[2]; / reserved,
    pub /: *mut *mut __be16 rnid_vend_spec; / vendor-specific field,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_els_rnid_atype {
    ELS_RNIDA_UNK =		0x01,	/* unknown */
    ELS_RNIDA_OTHER =	0x02,	/* none of the following */
    ELS_RNIDA_HUB =		0x03,
    ELS_RNIDA_SWITCH =	0x04,
    ELS_RNIDA_GATEWAY =	0x05,
    ELS_RNIDA_CONV =	0x06,   /* Obsolete, do not use this value */
    ELS_RNIDA_HBA =	        0x07,   /* Obsolete, do not use this value */
    ELS_RNIDA_PROXY =       0x08,   /* Obsolete, do not use this value */
    ELS_RNIDA_STORAGE =	0x09,
    ELS_RNIDA_HOST =	0x0a,
    ELS_RNIDA_SUBSYS =	0x0b,	/* storage subsystem (e.g., RAID) */
    ELS_RNIDA_ACCESS =	0x0e,	/* access device (e.g. media changer) */
    ELS_RNIDA_NAS =		0x11,	/* NAS server */
    ELS_RNIDA_BRIDGE =	0x12,	/* bridge */
    ELS_RNIDA_VIRT =	0x13,	/* virtualization device */
    ELS_RNIDA_MF =		0xff,	/* multifunction device (bits below) */
    ELS_RNIDA_MF_HUB =	1UL << 31, 	/* hub */
    ELS_RNIDA_MF_SW =	1UL << 30, 	/* switch */
    ELS_RNIDA_MF_GW =	1UL << 29,	/* gateway */
    ELS_RNIDA_MF_ST =	1UL << 28,	/* storage */
    ELS_RNIDA_MF_HOST =	1UL << 27,	/* host */
    ELS_RNIDA_MF_SUB =	1UL << 26,	/* storage subsystem */
    ELS_RNIDA_MF_ACC =	1UL << 25,	/* storage access dev */
    ELS_RNIDA_MF_WDM =	1UL << 24,	/* wavelength division mux */
    ELS_RNIDA_MF_NAS =	1UL << 23,	/* NAS server */
    ELS_RNIDA_MF_BR =	1UL << 22,	/* bridge */
    ELS_RNIDA_MF_VIRT =	1UL << 21,	/* virtualization device */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_els_rnid_mgmt {
    ELS_RNIDM_SNMP =	0,
    ELS_RNIDM_TELNET =	1,
    ELS_RNIDM_HTTP =	2,
    ELS_RNIDM_HTTPS =	3,
    ELS_RNIDM_XML =		4,	/* HTTP + XML */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_els_rnid_ipver {
    ELS_RNIDIP_NONE =	0,	/* no IP support or node mgmt. */
    ELS_RNIDIP_V4 =		1,	/* IPv4 */
    ELS_RNIDIP_V6 =		2,	/* IPv6 */
}

//
// ELS RPL - Read Port List.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_rpl {
    pub /: *mut *mut __u8 rpl_cmd; / command,
    pub /: *mut *mut __u8 rpl_resv[5]; / reserved - must be zero,
    pub /: *mut *mut __be16 rpl_max_size; / maximum response size or zero,
    pub /: *mut *mut __u8 rpl_resv1; / reserved - must be zero,
    pub /: *mut *mut __u8 rpl_index[3]; / starting index,
}

//
// Port number block in RPL response.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_pnb {
    pub /: *mut *mut __be32 pnb_phys_pn; / physical port number,
    pub /: *mut *mut __u8 pnb_resv; / reserved,
    pub /: *mut *mut __u8 pnb_port_id[3]; / port ID,
    pub /: *mut *mut __be64 pnb_wwpn; / port name,
}

//
// RPL LS_ACC response.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_rpl_resp {
    pub /: *mut *mut __u8 rpl_cmd; / ELS_LS_ACC,
    pub /: *mut *mut __u8 rpl_resv1; / reserved - must be zero,
    pub /: *mut *mut __be16 rpl_plen; / payload length,
    pub /: *mut *mut __u8 rpl_resv2; / reserved - must be zero,
    pub /: *mut *mut __u8 rpl_llen[3]; / list length,
    pub /: *mut *mut __u8 rpl_resv3; / reserved - must be zero,
    pub /: *mut *mut __u8 rpl_index[3]; / starting index,
    pub /: *mut *mut fc_els_pnb rpl_pnb[1]; / variable number of PNBs,
}

//
// Link Error Status Block.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_lesb {
    pub /: *mut *mut __be32 lesb_link_fail; / link failure count,
    pub /: *mut *mut __be32 lesb_sync_loss; / loss of synchronization count,
    pub /: *mut *mut __be32 lesb_sig_loss; / loss of signal count,
    pub /: *mut *mut __be32 lesb_prim_err; / primitive sequence error count,
    pub /: *mut *mut __be32 lesb_inv_word; / invalid transmission word count,
    pub /: *mut *mut __be32 lesb_inv_crc; / invalid CRC count,
}

//
// ELS RPS - Read Port Status Block request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_rps {
    pub /: *mut *mut __u8 rps_cmd; / command,
    pub /: *mut *mut __u8 rps_resv[2]; / reserved - must be zero,
    pub /: *mut *mut __u8 rps_flag; / flag - see below,
    pub /: *mut *mut __be64 rps_port_spec; / port selection,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_els_rps_flag {
    FC_ELS_RPS_DID =	0x00,	/* port identified by D_ID of req. */
    FC_ELS_RPS_PPN =	0x01,	/* port_spec is physical port number */
    FC_ELS_RPS_WWPN =	0x02,	/* port_spec is port WWN */
}

//
// ELS RPS LS_ACC response.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_rps_resp {
    pub /: *mut *mut __u8 rps_cmd; / command - LS_ACC,
    pub /: *mut *mut __u8 rps_resv[2]; / reserved - must be zero,
    pub /: *mut *mut __u8 rps_flag; / flag - see below,
    pub /: *mut *mut __u8 rps_resv2[2]; / reserved,
    pub /: *mut *mut __be16 rps_status; / port status - see below,
    pub /: *mut *mut fc_els_lesb rps_lesb; / link error status block,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_els_rps_resp_flag {
    FC_ELS_RPS_LPEV =	0x01,	/* L_port extension valid */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_els_rps_resp_status {
    FC_ELS_RPS_PTP =	1 << 5,	/* point-to-point connection */
    FC_ELS_RPS_LOOP =	1 << 4,	/* loop mode */
    FC_ELS_RPS_FAB =	1 << 3,	/* fabric present */
    FC_ELS_RPS_NO_SIG =	1 << 2,	/* loss of signal */
    FC_ELS_RPS_NO_SYNC =	1 << 1,	/* loss of synchronization */
    FC_ELS_RPS_RESET =	1 << 0,	/* in link reset protocol */
}

//
// ELS LIRR - Link Incident Record Registration request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_lirr {
    pub /: *mut *mut __u8 lirr_cmd; / command,
    pub /: *mut *mut __u8 lirr_resv[3]; / reserved - must be zero,
    pub /: *mut *mut __u8 lirr_func; / registration function,
    pub /: *mut *mut __u8 lirr_fmt; / FC-4 type of RLIR requested,
    pub /: *mut *mut __u8 lirr_resv2[2]; / reserved - must be zero,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_els_lirr_func {
    ELS_LIRR_SET_COND = 	0x01,	/* set - conditionally receive */
    ELS_LIRR_SET_UNCOND = 	0x02,	/* set - unconditionally receive */
    ELS_LIRR_CLEAR = 	0xff	/* clear registration */
}

//
// ELS SRL - Scan Remote Loop request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_srl {
    pub /: *mut *mut __u8 srl_cmd; / command,
    pub /: *mut *mut __u8 srl_resv[3]; / reserved - must be zero,
    pub /: *mut *mut __u8 srl_flag; / flag - see below,
    pub /: *mut *mut __u8 srl_flag_param[3]; / flag parameter,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_els_srl_flag {
    FC_ELS_SRL_ALL =	0x00,	/* scan all FL ports */
    FC_ELS_SRL_ONE =	0x01,	/* scan specified loop */
    FC_ELS_SRL_EN_PER =	0x02,	/* enable periodic scanning (param) */
    FC_ELS_SRL_DIS_PER =	0x03,	/* disable periodic scanning */
}

//
// ELS RLS - Read Link Error Status Block request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_rls {
    pub /: *mut *mut __u8 rls_cmd; / command,
    pub /: *mut *mut __u8 rls_resv[4]; / reserved - must be zero,
    pub /: *mut *mut __u8 rls_port_id[3]; / port ID,
}

//
// ELS RLS LS_ACC Response.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_rls_resp {
    pub /: *mut *mut __u8 rls_cmd; / ELS_LS_ACC,
    pub /: *mut *mut __u8 rls_resv[3]; / reserved - must be zero,
    pub /: *mut *mut fc_els_lesb rls_lesb; / link error status block,
}

//
// ELS RLIR - Registered Link Incident Report.
// This is followed by the CLIR and the CLID, described below.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_rlir {
    pub /: *mut *mut __u8 rlir_cmd; / command,
    pub /: *mut *mut __u8 rlir_resv[3]; / reserved - must be zero,
    pub /: *mut *mut __u8 rlir_fmt; / format (FC4-type if type specific),
    pub /: *mut *mut __u8 rlir_clr_len; / common link incident record length,
    pub /: *mut *mut __u8 rlir_cld_len; / common link incident desc. length,
    pub /: *mut *mut __u8 rlir_slr_len; / spec. link incident record length,
}

//
// CLIR - Common Link Incident Record Data. - Sent via RLIR.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_clir {
    pub /: *mut *mut __be64 clir_wwpn; / incident port name,
    pub /: *mut *mut __be64 clir_wwnn; / incident port node name,
    pub /: *mut *mut __u8 clir_port_type; / incident port type,
    pub /: *mut *mut __u8 clir_port_id[3]; / incident port ID,
    pub /: *mut *mut __be64 clir_conn_wwpn; / connected port name,
    pub /: *mut *mut __be64 clir_conn_wwnn; / connected node name,
    pub /: *mut *mut __be64 clir_fab_name; / fabric name,
    pub /: *mut *mut __be32 clir_phys_port; / physical port number,
    pub /: *mut *mut __be32 clir_trans_id; / transaction ID,
    pub /: *mut *mut __u8 clir_resv[3]; / reserved,
    pub /: *mut *mut __u8 clir_ts_fmt; / time stamp format,
    pub /: *mut *mut __be64 clir_timestamp; / time stamp,
}

//
// CLIR clir_ts_fmt - time stamp format values.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_els_clir_ts_fmt {
    ELS_CLIR_TS_UNKNOWN = 	0,	/* time stamp field unknown */
    ELS_CLIR_TS_SEC_FRAC = 	1,	/* time in seconds and fractions */
    ELS_CLIR_TS_CSU =	2,	/* time in clock synch update format */
}

//
// Common Link Incident Descriptor - sent via RLIR.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_clid {
    pub /: *mut *mut __u8 clid_iq; / incident qualifier flags,
    pub /: *mut *mut __u8 clid_ic; / incident code,
    pub /: *mut *mut __be16 clid_epai; / domain/area of ISL,
}

//
// CLID incident qualifier flags.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_els_clid_iq {
    ELS_CLID_SWITCH =	0x20,	/* incident port is a switch node */
    ELS_CLID_E_PORT =	0x10,	/* incident is an ISL (E) port */
    ELS_CLID_SEV_MASK =	0x0c,	/* severity 2-bit field mask */
    ELS_CLID_SEV_INFO =	0x00,	/* report is informational */
    ELS_CLID_SEV_INOP =	0x08,	/* link not operational */
    ELS_CLID_SEV_DEG =	0x04,	/* link degraded but operational */
    ELS_CLID_LASER =	0x02,	/* subassembly is a laser */
    ELS_CLID_FRU =		0x01,	/* format can identify a FRU */
}

//
// CLID incident code.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_els_clid_ic {
    ELS_CLID_IC_IMPL =	1,	/* implicit incident */
    ELS_CLID_IC_BER =	2,	/* bit-error-rate threshold exceeded */
    ELS_CLID_IC_LOS =	3,	/* loss of synch or signal */
    ELS_CLID_IC_NOS =	4,	/* non-operational primitive sequence */
    ELS_CLID_IC_PST =	5,	/* primitive sequence timeout */
    ELS_CLID_IC_INVAL =	6,	/* invalid primitive sequence */
    ELS_CLID_IC_LOOP_TO =	7,	/* loop initialization time out */
    ELS_CLID_IC_LIP =	8,	/* receiving LIP */
}

//
// Link Integrity event types
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_fpin_li_event_types {
    FPIN_LI_UNKNOWN =		0x0,
    FPIN_LI_LINK_FAILURE =		0x1,
    FPIN_LI_LOSS_OF_SYNC =		0x2,
    FPIN_LI_LOSS_OF_SIG =		0x3,
    FPIN_LI_PRIM_SEQ_ERR =		0x4,
    FPIN_LI_INVALID_TX_WD =		0x5,
    FPIN_LI_INVALID_CRC =		0x6,
    FPIN_LI_DEVICE_SPEC =		0xF,
}

//
// Initializer useful for decoding table.
// Please keep this in sync with the above definitions.
//

//
// Delivery event types
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_fpin_deli_event_types {
    FPIN_DELI_UNKNOWN =		0x0,
    FPIN_DELI_TIMEOUT =		0x1,
    FPIN_DELI_UNABLE_TO_ROUTE =	0x2,
    FPIN_DELI_DEVICE_SPEC =		0xF,
}

//
// Initializer useful for decoding table.
// Please keep this in sync with the above definitions.
//

//
// Congestion event types
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_fpin_congn_event_types {
    FPIN_CONGN_CLEAR =		0x0,
    FPIN_CONGN_LOST_CREDIT =	0x1,
    FPIN_CONGN_CREDIT_STALL =	0x2,
    FPIN_CONGN_OVERSUBSCRIPTION =	0x3,
    FPIN_CONGN_DEVICE_SPEC =	0xF,
}

//
// Initializer useful for decoding table.
// Please keep this in sync with the above definitions.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_fpin_congn_severity_types {
    FPIN_CONGN_SEVERITY_WARNING =	0xF1,
    FPIN_CONGN_SEVERITY_ERROR =	0xF7,
}

//
// Link Integrity Notification Descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_fn_li_desc {
    pub /: *mut *mut __be32 desc_tag; / Descriptor Tag (0x00020001),
    pub bytes).: *mut *mut __be32 desc_len; / Length of Descriptor (in,
// Size of descriptor excluding
// desc_tag and desc_len fields.
//
    pub /: *mut *mut __be64 detecting_wwpn; / Port Name that detected event,
    pub to: *mut *mut __be64 attached_wwpn; / Port Name of device attached,
// detecting Port Name
//
    pub /: *mut *mut __be16 event_type; / see enum fc_fpin_li_event_types,
    pub value: *mut *mut __be16 event_modifier; / Implementation specific,
// describing the event type
//
    pub link: *mut *mut __be32 event_threshold;/ duration in ms of the,
// integrity detection cycle
//
    pub event: *mut *mut __be32 event_count; / minimum number of,
// occurrences during the event
// threshold to cause the LI event
//
    pub /: *mut *mut __be32 pname_count; / number of portname_list elements,
    pub accessible: *mut *mut __be64 pname_list[]; / list of N_Port_Names,
// through the attached port
//
}

//
// Delivery Notification Descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_fn_deli_desc {
    pub /: *mut *mut __be32 desc_tag; / Descriptor Tag (0x00020002),
    pub bytes).: *mut *mut __be32 desc_len; / Length of Descriptor (in,
// Size of descriptor excluding
// desc_tag and desc_len fields.
//
    pub /: *mut *mut __be64 detecting_wwpn; / Port Name that detected event,
    pub to: *mut *mut __be64 attached_wwpn; / Port Name of device attached,
// detecting Port Name
//
    pub /: *mut *mut __be32 deli_reason_code;/ see enum fc_fpin_deli_event_types,
}

//
// Peer Congestion Notification Descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_fn_peer_congn_desc {
    pub /: *mut *mut __be32 desc_tag; / Descriptor Tag (0x00020003),
    pub bytes).: *mut *mut __be32 desc_len; / Length of Descriptor (in,
// Size of descriptor excluding
// desc_tag and desc_len fields.
//
    pub /: *mut *mut __be64 detecting_wwpn; / Port Name that detected event,
    pub to: *mut *mut __be64 attached_wwpn; / Port Name of device attached,
// detecting Port Name
//
    pub /: *mut *mut __be16 event_type; / see enum fc_fpin_congn_event_types,
    pub value: *mut *mut __be16 event_modifier; / Implementation specific,
// describing the event type
//
    pub detected: *mut *mut __be32 event_period; / duration (ms) of the,
// congestion event
//
    pub /: *mut *mut __be32 pname_count; / number of portname_list elements,
    pub accessible: *mut *mut __be64 pname_list[]; / list of N_Port_Names,
// through the attached port
//
}

//
// Congestion Notification Descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_fn_congn_desc {
    pub /: *mut *mut __be32 desc_tag; / Descriptor Tag (0x00020004),
    pub bytes).: *mut *mut __be32 desc_len; / Length of Descriptor (in,
// Size of descriptor excluding
// desc_tag and desc_len fields.
//
    pub /: *mut *mut __be16 event_type; / see enum fc_fpin_congn_event_types,
    pub value: *mut *mut __be16 event_modifier; / Implementation specific,
// describing the event type
//
    pub detected: *mut *mut __be32 event_period; / duration (ms) of the,
// congestion event
//
    pub /: *mut *mut __u8 severity; / command,
    pub /: *mut *mut __u8 resv[3]; / reserved - must be zero,
}

//
// ELS_FPIN - Fabric Performance Impact Notification
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_fpin {
    pub /: *mut *mut __u8 fpin_cmd; / command (0x16),
    pub /: *mut *mut __u8 fpin_zero[3]; / specified as zero - part of cmd,
    pub bytes).: *mut *mut __be32 desc_len; / Length of Descriptor List (in,
// Size of ELS excluding fpin_cmd,
// fpin_zero and desc_len fields.
//
    pub /: *mut *mut fc_tlv_desc fpin_desc[]; / Descriptor list,
}

// Diagnostic Function Descriptor - FPIN Registration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_df_desc_fpin_reg {
// New members MUST be added within the __struct_group() macro below.
    pub /: *mut *mut __be32 desc_tag; / FPIN Registration (0x00030001),
    pub bytes).: *mut *mut __be32 desc_len; / Length of Descriptor (in,
// Size of descriptor excluding
// desc_tag and desc_len fields.
//
    pub /: *mut *mut __be32 count; / Number of desc_tags elements,
    pub Tags.: *mut *mut __be32 desc_tags[]; / Array of Descriptor,
// Each tag indicates a function
// supported by the N_Port (request)
// or by the  N_Port and Fabric
// Controller (reply; may be a subset
// of the request).
// See ELS_FN_DTAG_xxx for tag values.
//
}

//
// ELS_RDF - Register Diagnostic Functions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_rdf {
// New members MUST be added within the __struct_group() macro below.
    pub /: *mut *mut __u8 fpin_cmd; / command (0x19),
    pub /: *mut *mut __u8 fpin_zero[3]; / specified as zero - part of cmd,
    pub bytes).: *mut *mut __be32 desc_len; / Length of Descriptor List (in,
// Size of ELS excluding fpin_cmd,
// fpin_zero and desc_len fields.
//
    pub /: *mut *mut fc_tlv_desc desc[]; / Descriptor list,
}

//
// ELS RDF LS_ACC Response.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_rdf_resp {
// New members MUST be added within the __struct_group() macro below.
    pub acc_hdr: fc_els_ls_acc,
    pub (in: *mut *mut __be32 desc_list_len; / Length of response,
// bytes). Excludes acc_hdr
// and desc_list_len fields.
//
    pub lsri: fc_els_lsri_desc,
    pub /: *mut *mut fc_tlv_desc desc[]; / Supported Descriptor list,
}

//
// Diagnostic Capability Descriptors for EDC ELS
//
// Diagnostic: Link Fault Capability Descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_diag_lnkflt_desc {
    pub /: *mut *mut __be32 desc_tag; / Descriptor Tag (0x0001000D),
    pub bytes).: *mut *mut __be32 desc_len; / Length of Descriptor (in,
// Size of descriptor excluding
// desc_tag and desc_len fields.
// 12 bytes
//
    pub degrade_activate_threshold: __be32,
    pub degrade_deactivate_threshold: __be32,
    pub fec_degrade_interval: __be32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_edc_cg_signal_cap_types {
// Note: Capability: bits 31:4 Rsvd; bits 3:0 are capabilities
    EDC_CG_SIG_NOTSUPPORTED =	0x00, /* neither supported */
    EDC_CG_SIG_WARN_ONLY =		0x01,
    EDC_CG_SIG_WARN_ALARM =		0x02, /* both supported */
}

//
// Initializer useful for decoding table.
// Please keep this in sync with the above definitions.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_diag_cg_sig_freq_types {
    EDC_CG_SIGFREQ_CNT_MIN =	1,	/* Min Frequency Count */
    EDC_CG_SIGFREQ_CNT_MAX =	999,	/* Max Frequency Count */

    EDC_CG_SIGFREQ_SEC =		0x1,	/* Units: seconds */
    EDC_CG_SIGFREQ_MSEC =		0x2,	/* Units: milliseconds */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_diag_cg_sig_freq {
    pub signals: *mut *mut __be16 count; / Time between,
// note: upper 6 bits rsvd
//
    pub count: *mut *mut __be16 units; / Time unit for,
// note: upper 12 bits rsvd
//
}

//
// Diagnostic: Congestion Signaling Capability Descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_diag_cg_sig_desc {
    pub /: *mut *mut __be32 desc_tag; / Descriptor Tag (0x0001000F),
    pub bytes).: *mut *mut __be32 desc_len; / Length of Descriptor (in,
// Size of descriptor excluding
// desc_tag and desc_len fields.
// 16 bytes
//
    pub xmt_signal_capability: __be32,
    pub xmt_signal_frequency: fc_diag_cg_sig_freq,
    pub rcv_signal_capability: __be32,
    pub rcv_signal_frequency: fc_diag_cg_sig_freq,
}

//
// ELS_EDC - Exchange Diagnostic Capabilities
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_edc {
    pub /: *mut *mut __u8 edc_cmd; / command (0x17),
    pub /: *mut *mut __u8 edc_zero[3]; / specified as zero - part of cmd,
    pub bytes).: *mut *mut __be32 desc_len; / Length of Descriptor List (in,
// Size of ELS excluding edc_cmd,
// edc_zero and desc_len fields.
//
    pub desc: [fc_tlv_desc; ],
// Diagnostic Descriptor list
}

//
// ELS EDC LS_ACC Response.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_edc_resp {
    pub acc_hdr: fc_els_ls_acc,
    pub (in: *mut *mut __be32 desc_list_len; / Length of response,
// bytes). Excludes acc_hdr
// and desc_list_len fields.
//
    pub lsri: fc_els_lsri_desc,
    pub desc: [fc_tlv_desc; ],
// Supported Diagnostic Descriptor list
}
