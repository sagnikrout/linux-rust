//! Automatically rewritten from C Header to Rust Module
//! Source: net/smc/smc_clc.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Shared Memory Communications over RDMA (SMC-R) and RoCE
//
// CLC (connection layer control) handshake over initial TCP socket to
// prepare for RDMA traffic
//
// Copyright IBM Corp. 2016
//
// Author(s):  Ursula Braun <ubraun@linux.vnet.ibm.com>
//

pub const SMC_CLC_PROPOSAL: c_uint = 0x01;
pub const SMC_CLC_ACCEPT: c_uint = 0x02;
pub const SMC_CLC_CONFIRM: c_uint = 0x03;
pub const SMC_CLC_DECLINE: c_uint = 0x04;

pub const SMC_CLC_DECL_MEM: c_uint = 0x01010000  /* insufficient memory resources  */;
pub const SMC_CLC_DECL_TIMEOUT_CL: c_uint = 0x02010000  /* timeout w4 QP confirm link     */;
pub const SMC_CLC_DECL_TIMEOUT_AL: c_uint = 0x02020000  /* timeout w4 QP add link	      */;
pub const SMC_CLC_DECL_CNFERR: c_uint = 0x03000000  /* configuration error            */;
pub const SMC_CLC_DECL_PEERNOSMC: c_uint = 0x03010000  /* peer did not indicate SMC      */;
pub const SMC_CLC_DECL_IPSEC: c_uint = 0x03020000  /* IPsec usage		      */;
pub const SMC_CLC_DECL_NOSMCDEV: c_uint = 0x03030000  /* no SMC device found (R or D)   */;
pub const SMC_CLC_DECL_NOSMCDDEV: c_uint = 0x03030001  /* no SMC-D device found	      */;
pub const SMC_CLC_DECL_NOSMCRDEV: c_uint = 0x03030002  /* no SMC-R device found	      */;
pub const SMC_CLC_DECL_NOISM2SUPP: c_uint = 0x03030003  /* hardware has no ISMv2 support  */;
pub const SMC_CLC_DECL_NOV2EXT: c_uint = 0x03030004  /* peer sent no clc v2 extension  */;
pub const SMC_CLC_DECL_NOV2DEXT: c_uint = 0x03030005  /* peer sent no clc SMC-Dv2 ext.  */;
pub const SMC_CLC_DECL_NOSEID: c_uint = 0x03030006  /* peer sent no SEID	      */;
pub const SMC_CLC_DECL_NOSMCD2DEV: c_uint = 0x03030007  /* no SMC-Dv2 device found	      */;
pub const SMC_CLC_DECL_NOUEID: c_uint = 0x03030008  /* peer sent no UEID	      */;
pub const SMC_CLC_DECL_RELEASEERR: c_uint = 0x03030009  /* release version negotiate failed */;
pub const SMC_CLC_DECL_MAXCONNERR: c_uint = 0x0303000a  /* max connections negotiate failed */;
pub const SMC_CLC_DECL_MAXLINKERR: c_uint = 0x0303000b  /* max links negotiate failed */;
pub const SMC_CLC_DECL_MODEUNSUPP: c_uint = 0x03040000  /* smc modes do not match (R or D)*/;
pub const SMC_CLC_DECL_RMBE_EC: c_uint = 0x03050000  /* peer has eyecatcher in RMBE    */;
pub const SMC_CLC_DECL_OPTUNSUPP: c_uint = 0x03060000  /* fastopen sockopt not supported */;
pub const SMC_CLC_DECL_DIFFPREFIX: c_uint = 0x03070000  /* IP prefix / subnet mismatch    */;
pub const SMC_CLC_DECL_GETVLANERR: c_uint = 0x03080000  /* err to get vlan id of ip device*/;
pub const SMC_CLC_DECL_ISMVLANERR: c_uint = 0x03090000  /* err to reg vlan id on ism dev  */;
pub const SMC_CLC_DECL_NOACTLINK: c_uint = 0x030a0000  /* no active smc-r link in lgr    */;
pub const SMC_CLC_DECL_NOSRVLINK: c_uint = 0x030b0000  /* SMC-R link from srv not found  */;
pub const SMC_CLC_DECL_VERSMISMAT: c_uint = 0x030c0000  /* SMC version mismatch	      */;
pub const SMC_CLC_DECL_MAX_DMB: c_uint = 0x030d0000  /* SMC-D DMB limit exceeded       */;
pub const SMC_CLC_DECL_NOROUTE: c_uint = 0x030e0000  /* SMC-Rv2 conn. no route to peer */;
pub const SMC_CLC_DECL_NOINDIRECT: c_uint = 0x030f0000  /* SMC-Rv2 conn. indirect mismatch*/;
pub const SMC_CLC_DECL_SYNCERR: c_uint = 0x04000000  /* synchronization error          */;
pub const SMC_CLC_DECL_PEERDECL: c_uint = 0x05000000  /* peer declined during handshake */;
pub const SMC_CLC_DECL_INTERR: c_uint = 0x09990000  /* internal error		      */;
pub const SMC_CLC_DECL_ERR_RTOK: c_uint = 0x09990001  /*	 rtoken handling failed       */;
pub const SMC_CLC_DECL_ERR_RDYLNK: c_uint = 0x09990002  /*	 ib ready link failed	      */;
pub const SMC_CLC_DECL_ERR_REGBUF: c_uint = 0x09990003  /*	 reg rdma bufs failed	      */;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_clc_msg_hdr {
    pub /: *mut *mut u8 eyecatcher[4]; / eye catcher,
    pub /: *mut *mut u8 type; / proposal / accept / confirm / decline,
    pub length: __be16,

    pub 2: typev1 :,

    pub 4: version :,

    pub /: *mut *mut } __packed; / format defined in RFC7609,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_clc_msg_trail {
    pub eyecatcher: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_clc_msg_local {
    pub /: *mut *mut u8 id_for_peer[SMC_SYSTEMID_LEN]; / unique system id,
    pub /: *mut *mut u8 gid[16]; / gid of ib_device port,
    pub /: *mut *mut u8 mac[6]; / mac of ib_device port,
}

// Struct would be 4 byte aligned, but it is used in an array that is sent
// to peers and must conform to RFC7609, hence we need to use packed here.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_clc_ipv6_prefix {
    pub prefix: in6_addr,
    pub prefix_len: u8,
    pub /: *mut *mut } __packed; / format defined in RFC7609,

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_clc_v2_flag {
    pub 1: seid :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_clc_v2_flag {
    pub 4: release :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_clnt_opts_area_hdr {
    pub /: *mut *mut u8 eid_cnt; / number of user defined EIDs,
    pub /: *mut *mut u8 ism_gid_cnt; / number of ISMv2 GIDs,
    pub reserved1: u8,
    pub flag: smc_clc_v2_flag,
    pub reserved2: [u8; 2],
    pub /: *mut *mut __be16 smcd_v2_ext_offset; / SMC-Dv2 Extension Offset,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_clc_smcd_gid_chid {
    pub /: *mut *mut __be64 gid; / ISM GID,
    pub /: *mut *mut __be16 chid; / ISMv2 CHID,
    pub in: *mut *mut } __packed; / format defined,
// IBM Shared Memory Communications Version 2
// (https://www.ibm.com/support/pages/node/6326337)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_clc_v2_extension {
// New members must be added within the struct_group() macro below.
    pub hdr: smc_clnt_opts_area_hdr,
    pub /: *mut *mut u8 roce[16]; / RoCEv2 GID,
    pub max_conns: u8,
    pub max_links: u8,
    pub feature_mask: __be16,
    pub reserved: [u8; 12],
    pub user_eids: [u8; ][SMC_MAX_EID_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_clc_msg_proposal_prefix {
    pub /: *mut *mut __be32 outgoing_subnet; / subnet mask,
    pub /: *mut *mut u8 prefix_len; / number of significant bits in mask,
    pub reserved: [u8; 2],
    pub /: *mut *mut u8 ipv6_prefixes_cnt; / number of IPv6 prefixes in prefix array,
    pub __aligned(4): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_clc_msg_smcd {
    pub /: *mut *mut smc_clc_smcd_gid_chid ism; / ISM native GID+CHID of requester,
    pub /: *mut *mut __be16 v2_ext_offset; / SMC Version 2 Extension Offset,
    pub /: *mut *mut u8 vendor_oui[3]; / vendor organizationally unique identifier,
    pub vendor_exp_options: [u8; 5],
    pub reserved: [u8; 20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_clc_smcd_v2_extension {
// New members must be added within the struct_group() macro below.
    pub system_eid: [u8; SMC_MAX_EID_LEN],
    pub reserved: [u8; 16],
    pub gidchid: [smc_clc_smcd_gid_chid; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_clc_msg_proposal {
    pub hdr: smc_clc_msg_hdr,
    pub lcl: smc_clc_msg_local,
    pub /: *mut *mut __be16 iparea_offset; / offset to IP address information area,
    pub __aligned(4): },
pub const SMC_CLC_MAX_V6_PREFIX: c_int = 8;
pub const SMC_CLC_MAX_UEID: c_int = 8;

// proposal SMC-Dv2 extension.
// each ISM device takes one entry and
// each Emulated-ISM takes two entries
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_clc_msg_proposal_area {
    pub pclc_base: smc_clc_msg_proposal,
    pub pclc_smcd: smc_clc_msg_smcd,
    pub pclc_prfx: smc_clc_msg_proposal_prefix,
    pub pclc_prfx_ipv6: [smc_clc_ipv6_prefix; SMC_CLC_MAX_V6_PREFIX],
    pub pclc_v2_ext: smc_clc_v2_extension_fixed,
    pub user_eids: [u8; SMC_CLC_MAX_UEID][SMC_MAX_EID_LEN],
    pub pclc_smcd_v2_ext: smc_clc_smcd_v2_extension_fixed,
    pub pclc_trl: smc_clc_msg_trail,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smcr_clc_msg_accept_confirm {
    pub lcl: smc_clc_msg_local,
    pub /: *mut *mut u8 qpn[3]; / QP number,
    pub /: *mut *mut __be32 rmb_rkey; / RMB rkey,
    pub /: *mut *mut u8 rmbe_idx; / Index of RMBE in RMB,
    pub /: *mut *mut __be32 rmbe_alert_token; / unique connection id,

    pub /: *mut *mut qp_mtu : 4; / QP mtu,

    pub 4: rmbe_size :,

    pub reserved: u8,
    pub /: *mut *mut __be64 rmb_dma_addr; / RMB virtual address,
    pub reserved2: u8,
    pub /: *mut *mut u8 psn[3]; / packet sequence number,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smcd_clc_msg_accept_confirm_common {
    pub /: *mut *mut __be64 gid; / Sender GID,
    pub /: *mut *mut __be64 token; / DMB token,
    pub /: *mut *mut u8 dmbe_idx; / DMBE index,

    pub 4: reserved3 :,

    pub 4: dmbe_size :,

    pub reserved4: u16,
    pub /: *mut *mut __be32 linkid; / Link identifier,
    pub __packed: },
pub const SMC_CLC_OS_ZOS: c_int = 1;
pub const SMC_CLC_OS_LINUX: c_int = 2;
pub const SMC_CLC_OS_AIX: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_clc_first_contact_ext {

    pub 7: reserved :,
    pub 4: release :,

    pub 1: v2_direct :,
    pub 4: os_type :,
    pub reserved2: [u8; 2],
    pub hostname: [u8; SMC_MAX_HOSTNAME_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_clc_first_contact_ext_v2x {
    pub fce_v2_base: smc_clc_first_contact_ext,
    pub /: *mut *mut u8 max_conns; / for SMC-R only,
    pub /: *mut *mut u8 max_links; / for SMC-R only,
}

// IBM Shared Memory Communications Version 2 (Third Edition)
// (https://www.ibm.com/support/pages/node/7009315)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_clc_fce_gid_ext {
    pub gid_cnt: u8,
    pub reserved2: [u8; 3],
    pub gid: [u8; ][SMC_GID_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_clc_msg_accept_confirm {
    pub hdr: smc_clc_msg_hdr,
    pub r0: smcr_clc_msg_accept_confirm,
    pub eid: [u8; SMC_MAX_EID_LEN],
    pub reserved6: [u8; 8],
    pub r1: } __packed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_clc_msg_decline {
    pub hdr: smc_clc_msg_hdr,
    pub /: *mut *mut u8 id_for_peer[SMC_SYSTEMID_LEN]; / sender peer_id,
    pub /: *mut *mut __be32 peer_diagnosis; / diagnosis information,

    pub 4: reserved :,

    pub 4: os_type :,
    pub reserved2: [u8; 3],
    pub /: *mut *mut smc_clc_msg_trail trl; / eye catcher "SMCD" or "SMCR" EBCDIC,
    pub __aligned(4): },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_clc_msg_decline_v2 {
    pub hdr: smc_clc_msg_hdr,
    pub /: *mut *mut u8 id_for_peer[SMC_SYSTEMID_LEN]; / sender peer_id,
    pub /: *mut *mut __be32 peer_diagnosis; / diagnosis information,

    pub 4: reserved :,

    pub 4: os_type :,
    pub reserved2: [u8; 3],
    pub peer_diagnosis_v2: [__be32; SMC_DECL_DIAG_COUNT_V2],
    pub /: *mut *mut smc_clc_msg_trail trl; / eye catcher "SMCD" or "SMCR" EBCDIC,
    pub __aligned(4): },
// determine start of the prefix area within the proposal message
    pub ntohs(pclc->iparea_offset): u16 offset =,
    pub NULL: return,
    pub offset): *mut *mut *mut ((u8 )pclc + sizeof(pclc) +,
    pub SMC_TYPE_B: return smc_type == SMC_TYPE_R || smc_type ==,
    pub SMC_TYPE_B: return smc_type == SMC_TYPE_D || smc_type ==,
    pub SMC_TYPE_B: return,
    pub SMC_TYPE_D: return,
    pub SMC_TYPE_R: return,
    pub SMC_TYPE_N: return,
// get SMC-D info from proposal message
    pub NULL: return,
    pub 1): *mut *mut return (struct smc_clc_msg_smcd )(prop +,
    pub smc_get_clc_msg_smcd(prop): *mut *mut smc_clc_msg_smcd prop_smcd =,
    pub max_offset: u16,
    pub v2_ext_offset): offsetofend(struct smc_clc_msg_smcd,,
    pub NULL: return,
    pub smcd_v2_ext_offset): offsetofend(struct smc_clnt_opts_area_hdr,,
    pub NULL: return,
    pub NULL: return,
    pub clc_v2_len: c_int,
    pub NULL: return,
    pub d1): offsetofend(struct smc_clc_msg_accept_confirm,,
    pub r1): offsetofend(struct smc_clc_msg_accept_confirm,,
    pub clc_v2_len): *mut *mut *mut return (struct smc_clc_first_contact_ext )(((u8 )clc) +,
    pub smcd_dev: struct,
    pub smc_init_info: struct,
    pub prop): *mut smc_clc_msg_proposal_prefix,
    pub timeout): u8 expected_type, unsigned long,
    pub version): *mut *mut int smc_clc_send_decline(struct smc_sock smc, u32 peer_diag_info, u8,
    pub ini): *mut *mut int smc_clc_send_proposal(struct smc_sock smc, struct smc_init_info,
    pub ini): *mut *mut u8 version, u8 eid, struct smc_init_info,
    pub ini): *mut *mut u8 version, u8 negotiated_eid, struct smc_init_info,
    pub ini): *mut smc_init_info,
    pub ini): *mut smc_init_info,
    pub ini): *mut smc_init_info,
    pub __init: void smc_clc_init(void),
    pub smc_clc_exit(void): c_void,
    pub host): *mut void smc_clc_get_hostname(u8,
    pub local_eid): *mut *mut u8 peer_eid, u8,
    pub smc_clc_ueid_count(void): c_int,
    pub cb): *mut *mut int smc_nl_dump_ueid(struct sk_buff skb, struct netlink_callback,
    pub info): *mut *mut int smc_nl_add_ueid(struct sk_buff skb, struct genl_info,
    pub info): *mut *mut int smc_nl_remove_ueid(struct sk_buff skb, struct genl_info,
    pub info): *mut *mut int smc_nl_flush_ueid(struct sk_buff skb, struct genl_info,
    pub cb): *mut *mut int smc_nl_dump_seid(struct sk_buff skb, struct netlink_callback,
    pub info): *mut *mut int smc_nl_enable_seid(struct sk_buff skb, struct genl_info,
    pub info): *mut *mut int smc_nl_disable_seid(struct sk_buff skb, struct genl_info,
