//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeontx2/af/npc.h
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
// Marvell RVU Admin Function driver
//
// Copyright (C) 2018 Marvell.
//
pub const NPC_KEX_CHAN_MASK: c_uint = 0xFFFULL;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum NPC_LID_E {
    NPC_LID_LA = 0,
    NPC_LID_LB,
    NPC_LID_LC,
    NPC_LID_LD,
    NPC_LID_LE,
    NPC_LID_LF,
    NPC_LID_LG,
    NPC_LID_LH,
}

pub const NPC_LT_NA: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npc_kpu_la_ltype {
    NPC_LT_LA_8023 = 1,
    NPC_LT_LA_ETHER,
    NPC_LT_LA_IH_NIX_ETHER,
    NPC_LT_LA_HIGIG2_ETHER = 7,
    NPC_LT_LA_IH_NIX_HIGIG2_ETHER,
    NPC_LT_LA_CUSTOM_L2_90B_ETHER,
    NPC_LT_LA_CPT_HDR,
    NPC_LT_LA_CUSTOM_L2_24B_ETHER,
    NPC_LT_LA_CUSTOM_PRE_L2_ETHER,
    NPC_LT_LA_FP_ETHER,
    NPC_LT_LA_CUSTOM0 = 0xE,
    NPC_LT_LA_CUSTOM1 = 0xF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npc_kpu_lb_ltype {
    NPC_LT_LB_ETAG = 1,
    NPC_LT_LB_CTAG,
    NPC_LT_LB_STAG_QINQ,
    NPC_LT_LB_BTAG,
    NPC_LT_LB_PPPOE,
    NPC_LT_LB_DSA,
    NPC_LT_LB_DSA_VLAN,
    NPC_LT_LB_EDSA,
    NPC_LT_LB_EDSA_VLAN,
    NPC_LT_LB_EXDSA,
    NPC_LT_LB_EXDSA_VLAN,
    NPC_LT_LB_FDSA,
    NPC_LT_LB_VLAN_EXDSA,
    NPC_LT_LB_CUSTOM0 = 0xE,
    NPC_LT_LB_CUSTOM1 = 0xF,
}

// Don't modify ltypes up to IP6_EXT, otherwise length and checksum of IP
// headers may not be checked correctly. IPv4 ltypes and IPv6 ltypes must
// differ only at bit 0 so mask 0xE can be used to detect extended headers.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npc_kpu_lc_ltype {
    NPC_LT_LC_PTP = 1,
    NPC_LT_LC_IP,
    NPC_LT_LC_IP_OPT,
    NPC_LT_LC_IP6,
    NPC_LT_LC_IP6_EXT,
    NPC_LT_LC_ARP,
    NPC_LT_LC_RARP,
    NPC_LT_LC_MPLS,
    NPC_LT_LC_NSH,
    NPC_LT_LC_FCOE,
    NPC_LT_LC_NGIO,
    NPC_LT_LC_CUSTOM0 = 0xE,
    NPC_LT_LC_CUSTOM1 = 0xF,
}

// Don't modify Ltypes upto SCTP, otherwise it will
// effect flow tag calculation and thus RSS.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npc_kpu_ld_ltype {
    NPC_LT_LD_TCP = 1,
    NPC_LT_LD_UDP,
    NPC_LT_LD_SCTP = 4,
    NPC_LT_LD_ICMP6,
    NPC_LT_LD_CUSTOM0,
    NPC_LT_LD_CUSTOM1,
    NPC_LT_LD_IGMP = 8,
    NPC_LT_LD_AH,
    NPC_LT_LD_GRE,
    NPC_LT_LD_NVGRE,
    NPC_LT_LD_NSH,
    NPC_LT_LD_TU_MPLS_IN_NSH,
    NPC_LT_LD_TU_MPLS_IN_IP,
    NPC_LT_LD_ICMP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npc_kpu_le_ltype {
    NPC_LT_LE_VXLAN = 1,
    NPC_LT_LE_GENEVE,
    NPC_LT_LE_ESP,
    NPC_LT_LE_GTPU = 4,
    NPC_LT_LE_VXLANGPE,
    NPC_LT_LE_GTPC,
    NPC_LT_LE_NSH,
    NPC_LT_LE_TU_MPLS_IN_GRE,
    NPC_LT_LE_TU_NSH_IN_GRE,
    NPC_LT_LE_TU_MPLS_IN_UDP,
    NPC_LT_LE_ROCEV2,
    NPC_LT_LE_CUSTOM0 = 0xE,
    NPC_LT_LE_CUSTOM1 = 0xF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npc_kpu_lf_ltype {
    NPC_LT_LF_TU_ETHER = 1,
    NPC_LT_LF_TU_PPP,
    NPC_LT_LF_TU_MPLS_IN_VXLANGPE,
    NPC_LT_LF_TU_NSH_IN_VXLANGPE,
    NPC_LT_LF_TU_MPLS_IN_NSH,
    NPC_LT_LF_TU_3RD_NSH,
    NPC_LT_LF_CUSTOM0 = 0xE,
    NPC_LT_LF_CUSTOM1 = 0xF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npc_kpu_lg_ltype {
    NPC_LT_LG_TU_IP = 1,
    NPC_LT_LG_TU_IP6,
    NPC_LT_LG_TU_ARP,
    NPC_LT_LG_TU_ETHER_IN_NSH,
    NPC_LT_LG_CUSTOM0 = 0xE,
    NPC_LT_LG_CUSTOM1 = 0xF,
}

// Don't modify Ltypes upto SCTP, otherwise it will
// effect flow tag calculation and thus RSS.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npc_kpu_lh_ltype {
    NPC_LT_LH_TU_TCP = 1,
    NPC_LT_LH_TU_UDP,
    NPC_LT_LH_TU_SCTP = 4,
    NPC_LT_LH_TU_ICMP6,
    NPC_LT_LH_CUSTOM0,
    NPC_LT_LH_CUSTOM1,
    NPC_LT_LH_TU_IGMP = 8,
    NPC_LT_LH_TU_ESP,
    NPC_LT_LH_TU_AH,
    NPC_LT_LH_TU_ICMP = 0xF,
}

// NPC port kind defines how the incoming or outgoing packets
// are processed. NPC accepts packets from up to 64 pkinds.
// Software assigns pkind for each incoming port such as CGX
// Ethernet interfaces, LBK interfaces, etc.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npc_pkind_type {
    NPC_RX_LBK_PKIND = 0ULL,
    NPC_RX_SKIP_SIZE_PKIND = 46ULL,
    NPC_RX_CPT_SKIP_SIZE_PKIND = 50ULL,
    NPC_RX_CPT_HDR_PTP_PKIND = 54ULL,
    NPC_RX_CUSTOM_PRE_L2_PKIND = 55ULL,
    NPC_RX_VLAN_EXDSA_PKIND = 56ULL,
    NPC_RX_CHLEN24B_PKIND = 57ULL,
    NPC_RX_CPT_HDR_PKIND,
    NPC_RX_CHLEN90B_PKIND,
    NPC_TX_HIGIG_PKIND,
    NPC_RX_HIGIG_PKIND,
    NPC_RX_EDSA_PKIND,
    NPC_TX_DEF_PKIND,	/* NIX-TX PKIND */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npc_interface_type {
    NPC_INTF_MODE_DEF,
}

// list of known and supported fields in packet header and
// fields present in key structure.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum key_fields {
    NPC_DMAC,
    NPC_SMAC,
    NPC_ETYPE,
    NPC_VLAN_ETYPE_CTAG, /* 0x8100 */
    NPC_VLAN_ETYPE_STAG, /* 0x88A8 */
    NPC_OUTER_VID,
    NPC_INNER_VID,
    NPC_TOS,
    NPC_IPFRAG_IPV4,
    NPC_SIP_IPV4,
    NPC_DIP_IPV4,
    NPC_IPFRAG_IPV6,
    NPC_SIP_IPV6,
    NPC_DIP_IPV6,
    NPC_IPPROTO_TCP,
    NPC_IPPROTO_UDP,
    NPC_IPPROTO_SCTP,
    NPC_IPPROTO_AH,
    NPC_IPPROTO_ESP,
    NPC_IPPROTO_ICMP,
    NPC_IPPROTO_ICMP6,
    NPC_SPORT_TCP,
    NPC_DPORT_TCP,
    NPC_SPORT_UDP,
    NPC_DPORT_UDP,
    NPC_SPORT_SCTP,
    NPC_DPORT_SCTP,
    NPC_IPSEC_SPI,
    NPC_MPLS1_LBTCBOS,
    NPC_MPLS1_TTL,
    NPC_MPLS2_LBTCBOS,
    NPC_MPLS2_TTL,
    NPC_MPLS3_LBTCBOS,
    NPC_MPLS3_TTL,
    NPC_MPLS4_LBTCBOS,
    NPC_MPLS4_TTL,
    NPC_TYPE_ICMP,
    NPC_CODE_ICMP,
    NPC_TCP_FLAGS,
    NPC_HEADER_FIELDS_MAX,
    NPC_CHAN = NPC_HEADER_FIELDS_MAX, /* Valid when Rx */
    NPC_PF_FUNC, /* Valid when Tx */
    NPC_ERRLEV,
    NPC_ERRCODE,
    NPC_LXMB,
    NPC_EXACT_RESULT,
    NPC_LA,
    NPC_LB,
    NPC_LC,
    NPC_LD,
    NPC_LE,
    NPC_LF,
    NPC_LG,
    NPC_LH,
// Ethertype for untagged frame
    NPC_ETYPE_ETHER,
// Ethertype for single tagged frame
    NPC_ETYPE_TAG1,
// Ethertype for double tagged frame
    NPC_ETYPE_TAG2,
// outer vlan tci for single tagged frame
    NPC_VLAN_TAG1,
// outer vlan tci for double tagged frame
    NPC_VLAN_TAG2,
// inner vlan tci for double tagged frame
    NPC_VLAN_TAG3,
// other header fields programmed to extract but not of our interest
    NPC_UNKNOWN,
    NPC_KEY_FIELDS_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_kpu_profile_cam {
    pub state: u8,
    pub state_mask: u8,
    pub dp0: u16,
    pub dp0_mask: u16,
    pub dp1: u16,
    pub dp1_mask: u16,
    pub dp2: u16,
    pub dp2_mask: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_kpu_profile_cam2 {
    pub state: u8,
    pub state_mask: u8,
    pub dp0: u16,
    pub dp0_mask: u16,
    pub dp1: u16,
    pub dp1_mask: u16,
    pub dp2: u16,
    pub dp2_mask: u16,
    pub ptype: u8,
    pub ptype_mask: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_kpu_profile_action {
    pub errlev: u8,
    pub errcode: u8,
    pub dp0_offset: u8,
    pub dp1_offset: u8,
    pub dp2_offset: u8,
    pub bypass_count: u8,
    pub parse_done: u8,
    pub next_state: u8,
    pub ptr_advance: u8,
    pub cap_ena: u8,
    pub lid: u8,
    pub ltype: u8,
    pub flags: u8,
    pub offset: u8,
    pub mask: u8,
    pub right: u8,
    pub shift: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_kpu_profile {
    pub cam_entries: c_int,
    pub action_entries: c_int,
    pub cam: *mut npc_kpu_profile_cam,
    pub action: *mut npc_kpu_profile_action,
    pub cam_entries2: c_int,
    pub action_entries2: c_int,
    pub action2: *mut npc_kpu_profile_action,
    pub cam2: *mut npc_kpu_profile_cam2,
}

// NPC KPU register formats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_kpu_cam {

    pub 8: u64 rsvd_63_56 :,
    pub 8: u64 state :,
    pub 16: u64 dp2_data :,
    pub 16: u64 dp1_data :,
    pub 16: u64 dp0_data :,

    pub 16: u64 dp0_data :,
    pub 16: u64 dp1_data :,
    pub 16: u64 dp2_data :,
    pub 8: u64 state :,
    pub 8: u64 rsvd_63_56 :,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_kpu_action0 {

    pub 7: u64 rsvd_63_57 :,
    pub 3: u64 byp_count :,
    pub 1: u64 capture_ena :,
    pub 1: u64 parse_done :,
    pub 8: u64 next_state :,
    pub 1: u64 rsvd_43 :,
    pub 3: u64 capture_lid :,
    pub 4: u64 capture_ltype :,
    pub 8: u64 capture_flags :,
    pub 8: u64 ptr_advance :,
    pub 8: u64 var_len_offset :,
    pub 8: u64 var_len_mask :,
    pub 1: u64 var_len_right :,
    pub 3: u64 var_len_shift :,

    pub 3: u64 var_len_shift :,
    pub 1: u64 var_len_right :,
    pub 8: u64 var_len_mask :,
    pub 8: u64 var_len_offset :,
    pub 8: u64 ptr_advance :,
    pub 8: u64 capture_flags :,
    pub 4: u64 capture_ltype :,
    pub 3: u64 capture_lid :,
    pub 1: u64 rsvd_43 :,
    pub 8: u64 next_state :,
    pub 1: u64 parse_done :,
    pub 1: u64 capture_ena :,
    pub 3: u64 byp_count :,
    pub 7: u64 rsvd_63_57 :,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_kpu_action1 {

    pub 28: u64 rsvd_63_36 :,
    pub 4: u64 errlev :,
    pub 8: u64 errcode :,
    pub 8: u64 dp2_offset :,
    pub 8: u64 dp1_offset :,
    pub 8: u64 dp0_offset :,

    pub 8: u64 dp0_offset :,
    pub 8: u64 dp1_offset :,
    pub 8: u64 dp2_offset :,
    pub 8: u64 errcode :,
    pub 4: u64 errlev :,
    pub 28: u64 rsvd_63_36 :,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_kpu_pkind_cpi_def {

    pub 1: u64 ena :,
    pub 4: u64 rsvd_62_59 :,
    pub 3: u64 lid :,
    pub 4: u64 ltype_match :,
    pub 4: u64 ltype_mask :,
    pub 8: u64 flags_match :,
    pub 8: u64 flags_mask :,
    pub 8: u64 add_offset :,
    pub 8: u64 add_mask :,
    pub 1: u64 rsvd_15 :,
    pub 3: u64 add_shift :,
    pub 2: u64 rsvd_11_10 :,
    pub 10: u64 cpi_base :,

    pub 10: u64 cpi_base :,
    pub 2: u64 rsvd_11_10 :,
    pub 3: u64 add_shift :,
    pub 1: u64 rsvd_15 :,
    pub 8: u64 add_mask :,
    pub 8: u64 add_offset :,
    pub 8: u64 flags_mask :,
    pub 8: u64 flags_match :,
    pub 4: u64 ltype_mask :,
    pub 4: u64 ltype_match :,
    pub 3: u64 lid :,
    pub 4: u64 rsvd_62_59 :,
    pub 1: u64 ena :,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_rx_action {

    pub :3: u64 rsvd_63_61,
    pub :5: u64 flow_key_alg,
    pub :16: u64 match_id,
    pub :20: u64 index,
    pub :16: u64 pf_func,
    pub :4: u64 op,

    pub :4: u64 op,
    pub :16: u64 pf_func,
    pub :20: u64 index,
    pub :16: u64 match_id,
    pub :5: u64 flow_key_alg,
    pub :3: u64 rsvd_63_61,

}

// NPC_AF_INTFX_KEX_CFG field masks
pub const NPC_EXACT_NIBBLE_START: c_int = 40;
pub const NPC_EXACT_NIBBLE_END: c_int = 43;

// NPC_EXACT_KEX_S nibble definitions for each field

// NPC_AF_INTFX_KEX_CFG field masks

pub const NPC_TOTAL_NIBBLE: c_int = 31;
// NPC_PARSE_KEX_S nibble definitions for each field

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_tx_action {

    pub :16: u64 rsvd_63_48,
    pub :16: u64 match_id,
    pub :20: u64 index,
    pub :8: u64 rsvd_11_8,
    pub :4: u64 op,

    pub :4: u64 op,
    pub :8: u64 rsvd_11_8,
    pub :20: u64 index,
    pub :16: u64 match_id,
    pub :16: u64 rsvd_63_48,

}

// NIX Receive Vtag Action Structure

// NIX Transmit Vtag Action Structure

// NPC MCAM reserved entry index per nixlf
pub const NIXLF_UCAST_ENTRY: c_int = 0;
pub const NIXLF_BCAST_ENTRY: c_int = 1;
pub const NIXLF_ALLMULTI_ENTRY: c_int = 2;
pub const NIXLF_PROMISC_ENTRY: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_coalesced_kpu_prfl {
pub const NPC_SIGN: c_uint = 0x00666f727063706e;

pub const NPC_NAME_LEN: c_int = 32;
    pub /: *mut *mut __le64 signature; / "npcprof\0" (8 bytes/ASCII characters),
    pub /: *mut *mut u8 name[NPC_NAME_LEN]; / KPU Profile name,
    pub /: *mut *mut u64 version; / KPU firmware/profile version,
    pub /: *mut *mut u8 num_prfl; / No of NPC profiles.,
    pub prfl_sz: [u16; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_mcam_kex {
// MKEX Profle Header
    pub /: *mut *mut u64 mkex_sign; / "mcam-kex-profile" (8 bytes/ASCII characters),
    pub /: *mut *mut u8 name[MKEX_NAME_LEN]; / MKEX Profile name,
    pub /: *mut *mut u64 cpu_model; / Format as profiled by CPU hardware,
    pub /: *mut *mut u64 kpu_version; / KPU firmware/profile version,
    pub /: *mut *mut u64 reserved; / Reserved for extension,
// MKEX Profle Data
    pub /: *mut *mut u64 keyx_cfg[NPC_MAX_INTF]; / NPC_AF_INTF(0..1)_KEX_CFG,
// NPC_AF_KEX_LDATA(0..1)_FLAGS_CFG
    pub kex_ld_flags: [u64; NPC_MAX_LD],
// NPC_AF_INTF(0..1)_LID(0..7)_LT(0..15)_LD(0..1)_CFG
    pub intf_lid_lt_ld: [u64; NPC_MAX_INTF][NPC_MAX_LID][NPC_MAX_LT][NPC_MAX_LD],
// NPC_AF_INTF(0..1)_LDATA(0..1)_FLAGS(0..15)_CFG
    pub intf_ld_flags: [u64; NPC_MAX_INTF][NPC_MAX_LD][NPC_MAX_LFL],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_kpu_fwdata {
    pub entries: c_int,
// What follows is:
// struct npc_kpu_profile_cam[entries];
// struct npc_kpu_profile_action[entries];
//
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_lt_def {
    pub ltype_mask: u8,
    pub ltype_match: u8,
    pub lid: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_lt_def_ipsec {
    pub ltype_mask: u8,
    pub ltype_match: u8,
    pub lid: u8,
    pub spi_offset: u8,
    pub spi_nz: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_lt_def_apad {
    pub ltype_mask: u8,
    pub ltype_match: u8,
    pub lid: u8,
    pub valid: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_lt_def_color {
    pub ltype_mask: u8,
    pub ltype_match: u8,
    pub lid: u8,
    pub noffset: u8,
    pub offset: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_lt_def_et {
    pub ltype_mask: u8,
    pub ltype_match: u8,
    pub lid: u8,
    pub valid: u8,
    pub offset: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_lt_def_cfg {
    pub rx_ol2: npc_lt_def,
    pub rx_oip4: npc_lt_def,
    pub rx_iip4: npc_lt_def,
    pub rx_oip6: npc_lt_def,
    pub rx_iip6: npc_lt_def,
    pub rx_otcp: npc_lt_def,
    pub rx_itcp: npc_lt_def,
    pub rx_oudp: npc_lt_def,
    pub rx_iudp: npc_lt_def,
    pub rx_osctp: npc_lt_def,
    pub rx_isctp: npc_lt_def,
    pub rx_ipsec: [npc_lt_def_ipsec; 2],
    pub pck_ol2: npc_lt_def,
    pub pck_oip4: npc_lt_def,
    pub pck_oip6: npc_lt_def,
    pub pck_iip4: npc_lt_def,
    pub rx_apad0: npc_lt_def_apad,
    pub rx_apad1: npc_lt_def_apad,
    pub ovlan: npc_lt_def_color,
    pub ivlan: npc_lt_def_color,
    pub rx_gen0_color: npc_lt_def_color,
    pub rx_gen1_color: npc_lt_def_color,
    pub rx_et: [npc_lt_def_et; 2],
    pub __packed: },
// Loadable KPU profile firmware data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_kpu_profile_fwdata {
pub const KPU_SIGN: c_uint = 0x00666f727075706b;
pub const KPU_NAME_LEN: c_int = 32;
// Maximum number of custom KPU entries supported by the built-in profile.
pub const KPU_MAX_CST_ENT: c_int = 6;
// KPU Profle Header
    pub /: *mut *mut __le64 signature; / "kpuprof\0" (8 bytes/ASCII characters),
    pub /: *mut *mut u8 name[KPU_NAME_LEN]; / KPU Profile name,
    pub /: *mut *mut __le64 version; / KPU profile version,
    pub kpus: u8,
    pub reserved: [u8; 7],
// Default MKEX profile to be used with this KPU profile. May be
// overridden with mkex_profile module parameter. Format is same as for
// the MKEX profile to streamline processing.
//
    pub mkex: npc_mcam_kex,
// LTYPE values for specific HW offloaded protocols.
    pub lt_def: npc_lt_def_cfg,
// Dynamically sized data:
// Custom KPU CAM and ACTION configuration entries.
// struct npc_kpu_fwdata kpu[kpus];
//
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvu_npc_mcam_rule {
    pub packet: flow_msg,
    pub mask: flow_msg,
    pub intf: u8,
    pub tx_action: nix_tx_action,
    pub rx_action: nix_rx_action,
}
