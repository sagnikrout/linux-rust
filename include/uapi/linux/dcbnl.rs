//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/dcbnl.h
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
// Copyright (c) 2008-2011, Intel Corporation.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms and conditions of the GNU General Public License,
// version 2, as published by the Free Software Foundation.
//
// This program is distributed in the hope it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for
// more details.
//
// You should have received a copy of the GNU General Public License along with
// this program; if not, write to the Free Software Foundation, Inc., 59 Temple
// Place - Suite 330, Boston, MA 02111-1307 USA.
//
// Author: Lucy Liu <lucy.liu@intel.com>
//

// IEEE 802.1Qaz std supported values
pub const IEEE_8021QAZ_MAX_TCS: c_int = 8;
pub const IEEE_8021QAZ_TSA_STRICT: c_int = 0;
pub const IEEE_8021QAZ_TSA_CB_SHAPER: c_int = 1;
pub const IEEE_8021QAZ_TSA_ETS: c_int = 2;
pub const IEEE_8021QAZ_TSA_VENDOR: c_int = 255;
// This structure contains the IEEE 802.1Qaz ETS managed object
//
// @willing: willing bit in ETS configuration TLV
// @ets_cap: indicates supported capacity of ets feature
// @cbs: credit based shaper ets algorithm supported
// @tc_tx_bw: tc tx bandwidth indexed by traffic class
// @tc_rx_bw: tc rx bandwidth indexed by traffic class
// @tc_tsa: TSA Assignment table, indexed by traffic class
// @prio_tc: priority assignment table mapping 8021Qp to traffic class
// @tc_reco_bw: recommended tc bandwidth indexed by traffic class for TLV
// @tc_reco_tsa: recommended tc bandwidth indexed by traffic class for TLV
// @reco_prio_tc: recommended tc tx bandwidth indexed by traffic class for TLV
//
// Recommended values are used to set fields in the ETS recommendation TLV
// with hardware offloaded LLDP.
//
// ----
// TSA Assignment 8 bit identifiers
// 0	strict priority
// 1	credit-based shaper
// 2	enhanced transmission selection
// 3-254	reserved
// 255	vendor specific
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee_ets {
    pub willing: __u8,
    pub ets_cap: __u8,
    pub cbs: __u8,
    pub tc_tx_bw: [__u8; IEEE_8021QAZ_MAX_TCS],
    pub tc_rx_bw: [__u8; IEEE_8021QAZ_MAX_TCS],
    pub tc_tsa: [__u8; IEEE_8021QAZ_MAX_TCS],
    pub prio_tc: [__u8; IEEE_8021QAZ_MAX_TCS],
    pub tc_reco_bw: [__u8; IEEE_8021QAZ_MAX_TCS],
    pub tc_reco_tsa: [__u8; IEEE_8021QAZ_MAX_TCS],
    pub reco_prio_tc: [__u8; IEEE_8021QAZ_MAX_TCS],
}

// This structure contains rate limit extension to the IEEE 802.1Qaz ETS
// managed object.
// Values are 64 bits long and specified in Kbps to enable usage over both
// slow and very fast networks.
//
// @tc_maxrate: maximal tc tx bandwidth indexed by traffic class
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee_maxrate {
    pub tc_maxrate: [__u64; IEEE_8021QAZ_MAX_TCS],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dcbnl_cndd_states {
    DCB_CNDD_RESET = 0,
    DCB_CNDD_EDGE,
    DCB_CNDD_INTERIOR,
    DCB_CNDD_INTERIOR_READY,
}

// This structure contains the IEEE 802.1Qau QCN managed object.
//
// @rpg_enable: enable QCN RP
// @rppp_max_rps: maximum number of RPs allowed for this CNPV on this port
// @rpg_time_reset: time between rate increases if no CNMs received.
// given in u-seconds
// @rpg_byte_reset: transmitted data between rate increases if no CNMs received.
// given in Bytes
// @rpg_threshold: The number of times rpByteStage or rpTimeStage can count
// before RP rate control state machine advances states
// @rpg_max_rate: the maxinun rate, in Mbits per second,
// at which an RP can transmit
// @rpg_ai_rate: The rate, in Mbits per second,
// used to increase rpTargetRate in the RPR_ACTIVE_INCREASE
// @rpg_hai_rate: The rate, in Mbits per second,
// used to increase rpTargetRate in the RPR_HYPER_INCREASE state
// @rpg_gd: Upon CNM receive, flow rate is limited to (Fb/Gd)*CurrentRate.
// rpgGd is given as log2(Gd), where Gd may only be powers of 2
// @rpg_min_dec_fac: The minimum factor by which the current transmit rate
// can be changed by reception of a CNM.
// value is given as percentage (1-100)
// @rpg_min_rate: The minimum value, in bits per second, for rate to limit
// @cndd_state_machine: The state of the congestion notification domain
// defense state machine, as defined by IEEE 802.3Qau
// section 32.1.1. In the interior ready state,
// the QCN capable hardware may add CN-TAG TLV to the
// outgoing traffic, to specifically identify outgoing
// flows.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee_qcn {
    pub rpg_enable: [__u8; IEEE_8021QAZ_MAX_TCS],
    pub rppp_max_rps: [__u32; IEEE_8021QAZ_MAX_TCS],
    pub rpg_time_reset: [__u32; IEEE_8021QAZ_MAX_TCS],
    pub rpg_byte_reset: [__u32; IEEE_8021QAZ_MAX_TCS],
    pub rpg_threshold: [__u32; IEEE_8021QAZ_MAX_TCS],
    pub rpg_max_rate: [__u32; IEEE_8021QAZ_MAX_TCS],
    pub rpg_ai_rate: [__u32; IEEE_8021QAZ_MAX_TCS],
    pub rpg_hai_rate: [__u32; IEEE_8021QAZ_MAX_TCS],
    pub rpg_gd: [__u32; IEEE_8021QAZ_MAX_TCS],
    pub rpg_min_dec_fac: [__u32; IEEE_8021QAZ_MAX_TCS],
    pub rpg_min_rate: [__u32; IEEE_8021QAZ_MAX_TCS],
    pub cndd_state_machine: [__u32; IEEE_8021QAZ_MAX_TCS],
}

// This structure contains the IEEE 802.1Qau QCN statistics.
//
// @rppp_rp_centiseconds: the number of RP-centiseconds accumulated
// by RPs at this priority level on this Port
// @rppp_created_rps: number of active RPs(flows) that react to CNMs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee_qcn_stats {
    pub rppp_rp_centiseconds: [__u64; IEEE_8021QAZ_MAX_TCS],
    pub rppp_created_rps: [__u32; IEEE_8021QAZ_MAX_TCS],
}

// This structure contains the IEEE 802.1Qaz PFC managed object
//
// @pfc_cap: Indicates the number of traffic classes on the local device
// that may simultaneously have PFC enabled.
// @pfc_en: bitmap indicating pfc enabled traffic classes
// @mbc: enable macsec bypass capability
// @delay: the allowance made for a round-trip propagation delay of the
// link in bits.
// @requests: count of the sent pfc frames
// @indications: count of the received pfc frames
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee_pfc {
    pub pfc_cap: __u8,
    pub pfc_en: __u8,
    pub mbc: __u8,
    pub delay: __u16,
    pub requests: [__u64; IEEE_8021QAZ_MAX_TCS],
    pub indications: [__u64; IEEE_8021QAZ_MAX_TCS],
}

pub const IEEE_8021Q_MAX_PRIORITIES: c_int = 8;
pub const DCBX_MAX_BUFFERS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcbnl_buffer {
// priority to buffer mapping
    pub prio2buffer: [__u8; IEEE_8021Q_MAX_PRIORITIES],
// buffer size in Bytes
    pub buffer_size: [__u32; DCBX_MAX_BUFFERS],
    pub total_size: __u32,
}

// CEE DCBX std supported values
pub const CEE_DCBX_MAX_PGS: c_int = 8;
pub const CEE_DCBX_MAX_PRIO: c_int = 8;
//
// struct cee_pg - CEE Priority-Group managed object
//
// @willing: willing bit in the PG tlv
// @error: error bit in the PG tlv
// @pg_en: enable bit of the PG feature
// @tcs_supported: number of traffic classes supported
// @pg_bw: bandwidth percentage for each priority group
// @prio_pg: priority to PG mapping indexed by priority
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cee_pg {
    pub willing: __u8,
    pub error: __u8,
    pub pg_en: __u8,
    pub tcs_supported: __u8,
    pub pg_bw: [__u8; CEE_DCBX_MAX_PGS],
    pub prio_pg: [__u8; CEE_DCBX_MAX_PGS],
}

//
// struct cee_pfc - CEE PFC managed object
//
// @willing: willing bit in the PFC tlv
// @error: error bit in the PFC tlv
// @pfc_en: bitmap indicating pfc enabled traffic classes
// @tcs_supported: number of traffic classes supported
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cee_pfc {
    pub willing: __u8,
    pub error: __u8,
    pub pfc_en: __u8,
    pub tcs_supported: __u8,
}

// IEEE 802.1Qaz std supported values
pub const IEEE_8021QAZ_APP_SEL_ETHERTYPE: c_int = 1;
pub const IEEE_8021QAZ_APP_SEL_STREAM: c_int = 2;
pub const IEEE_8021QAZ_APP_SEL_DGRAM: c_int = 3;
pub const IEEE_8021QAZ_APP_SEL_ANY: c_int = 4;
pub const IEEE_8021QAZ_APP_SEL_DSCP: c_int = 5;
// Non-std selector values
pub const DCB_APP_SEL_PCP: c_int = 255;
// This structure contains the IEEE 802.1Qaz APP managed object. This
// object is also used for the CEE std as well.
//
// @selector: protocol identifier type
// @protocol: protocol of type indicated
// @priority: 3-bit unsigned integer indicating priority for IEEE
// 8-bit 802.1p user priority bitmap for CEE
//
// ----
// Selector field values for IEEE 802.1Qaz
// 0	Reserved
// 1	Ethertype
// 2	Well known port number over TCP or SCTP
// 3	Well known port number over UDP or DCCP
// 4	Well known port number over TCP, SCTP, UDP, or DCCP
// 5	Differentiated Services Code Point (DSCP) value
// 6-7	Reserved
//
// Selector field values for CEE
// 0	Ethertype
// 1	Well known port number over TCP or UDP
// 2-3	Reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcb_app {
    pub selector: __u8,
    pub priority: __u8,
    pub protocol: __u16,
}

pub const IEEE_8021QAZ_APP_SEL_MAX: c_int = 255;
//
// struct dcb_peer_app_info - APP feature information sent by the peer
//
// @willing: willing bit in the peer APP tlv
// @error: error bit in the peer APP tlv
//
// In addition to this information the full peer APP tlv also contains
// a table of 'app_count' APP objects defined above.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcb_peer_app_info {
    pub willing: __u8,
    pub error: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcbmsg {
    pub dcb_family: __u8,
    pub cmd: __u8,
    pub dcb_pad: __u16,
}

//
// enum dcbnl_commands - supported DCB commands
//
// @DCB_CMD_UNDEFINED: unspecified command to catch errors
// @DCB_CMD_GSTATE: request the state of DCB in the device
// @DCB_CMD_SSTATE: set the state of DCB in the device
// @DCB_CMD_PGTX_GCFG: request the priority group configuration for Tx
// @DCB_CMD_PGTX_SCFG: set the priority group configuration for Tx
// @DCB_CMD_PGRX_GCFG: request the priority group configuration for Rx
// @DCB_CMD_PGRX_SCFG: set the priority group configuration for Rx
// @DCB_CMD_PFC_GCFG: request the priority flow control configuration
// @DCB_CMD_PFC_SCFG: set the priority flow control configuration
// @DCB_CMD_SET_ALL: apply all changes to the underlying device
// @DCB_CMD_GPERM_HWADDR: get the permanent MAC address of the underlying
// device.  Only useful when using bonding.
// @DCB_CMD_GCAP: request the DCB capabilities of the device
// @DCB_CMD_GNUMTCS: get the number of traffic classes currently supported
// @DCB_CMD_SNUMTCS: set the number of traffic classes
// @DCB_CMD_GBCN: set backward congestion notification configuration
// @DCB_CMD_SBCN: get backward congestion notification configuration.
// @DCB_CMD_GAPP: get application protocol configuration
// @DCB_CMD_SAPP: set application protocol configuration
// @DCB_CMD_IEEE_SET: set IEEE 802.1Qaz configuration
// @DCB_CMD_IEEE_GET: get IEEE 802.1Qaz configuration
// @DCB_CMD_GDCBX: get DCBX engine configuration
// @DCB_CMD_SDCBX: set DCBX engine configuration
// @DCB_CMD_GFEATCFG: get DCBX features flags
// @DCB_CMD_SFEATCFG: set DCBX features negotiation flags
// @DCB_CMD_CEE_GET: get CEE aggregated configuration
// @DCB_CMD_IEEE_DEL: delete IEEE 802.1Qaz configuration
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dcbnl_commands {
    DCB_CMD_UNDEFINED,

    DCB_CMD_GSTATE,
    DCB_CMD_SSTATE,

    DCB_CMD_PGTX_GCFG,
    DCB_CMD_PGTX_SCFG,
    DCB_CMD_PGRX_GCFG,
    DCB_CMD_PGRX_SCFG,

    DCB_CMD_PFC_GCFG,
    DCB_CMD_PFC_SCFG,

    DCB_CMD_SET_ALL,

    DCB_CMD_GPERM_HWADDR,

    DCB_CMD_GCAP,

    DCB_CMD_GNUMTCS,
    DCB_CMD_SNUMTCS,

    DCB_CMD_PFC_GSTATE,
    DCB_CMD_PFC_SSTATE,

    DCB_CMD_BCN_GCFG,
    DCB_CMD_BCN_SCFG,

    DCB_CMD_GAPP,
    DCB_CMD_SAPP,

    DCB_CMD_IEEE_SET,
    DCB_CMD_IEEE_GET,

    DCB_CMD_GDCBX,
    DCB_CMD_SDCBX,

    DCB_CMD_GFEATCFG,
    DCB_CMD_SFEATCFG,

    DCB_CMD_CEE_GET,
    DCB_CMD_IEEE_DEL,

    __DCB_CMD_ENUM_MAX,
    DCB_CMD_MAX = __DCB_CMD_ENUM_MAX - 1,
}

//
// enum dcbnl_attrs - DCB top-level netlink attributes
//
// @DCB_ATTR_UNDEFINED: unspecified attribute to catch errors
// @DCB_ATTR_IFNAME: interface name of the underlying device (NLA_STRING)
// @DCB_ATTR_STATE: enable state of DCB in the device (NLA_U8)
// @DCB_ATTR_PFC_STATE: enable state of PFC in the device (NLA_U8)
// @DCB_ATTR_PFC_CFG: priority flow control configuration (NLA_NESTED)
// @DCB_ATTR_NUM_TC: number of traffic classes supported in the device (NLA_U8)
// @DCB_ATTR_PG_CFG: priority group configuration (NLA_NESTED)
// @DCB_ATTR_SET_ALL: bool to commit changes to hardware or not (NLA_U8)
// @DCB_ATTR_PERM_HWADDR: MAC address of the physical device (NLA_NESTED)
// @DCB_ATTR_CAP: DCB capabilities of the device (NLA_NESTED)
// @DCB_ATTR_NUMTCS: number of traffic classes supported (NLA_NESTED)
// @DCB_ATTR_BCN: backward congestion notification configuration (NLA_NESTED)
// @DCB_ATTR_IEEE: IEEE 802.1Qaz supported attributes (NLA_NESTED)
// @DCB_ATTR_DCBX: DCBX engine configuration in the device (NLA_U8)
// @DCB_ATTR_FEATCFG: DCBX features flags (NLA_NESTED)
// @DCB_ATTR_CEE: CEE std supported attributes (NLA_NESTED)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dcbnl_attrs {
    DCB_ATTR_UNDEFINED,

    DCB_ATTR_IFNAME,
    DCB_ATTR_STATE,
    DCB_ATTR_PFC_STATE,
    DCB_ATTR_PFC_CFG,
    DCB_ATTR_NUM_TC,
    DCB_ATTR_PG_CFG,
    DCB_ATTR_SET_ALL,
    DCB_ATTR_PERM_HWADDR,
    DCB_ATTR_CAP,
    DCB_ATTR_NUMTCS,
    DCB_ATTR_BCN,
    DCB_ATTR_APP,

// IEEE std attributes
    DCB_ATTR_IEEE,

    DCB_ATTR_DCBX,
    DCB_ATTR_FEATCFG,

// CEE nested attributes
    DCB_ATTR_CEE,

    __DCB_ATTR_ENUM_MAX,
    DCB_ATTR_MAX = __DCB_ATTR_ENUM_MAX - 1,
}

//
// enum ieee_attrs - IEEE 802.1Qaz get/set attributes
//
// @DCB_ATTR_IEEE_UNSPEC: unspecified
// @DCB_ATTR_IEEE_ETS: negotiated ETS configuration
// @DCB_ATTR_IEEE_PFC: negotiated PFC configuration
// @DCB_ATTR_IEEE_APP_TABLE: negotiated APP configuration
// @DCB_ATTR_IEEE_PEER_ETS: peer ETS configuration - get only
// @DCB_ATTR_IEEE_PEER_PFC: peer PFC configuration - get only
// @DCB_ATTR_IEEE_PEER_APP: peer APP tlv - get only
// @DCB_ATTR_DCB_APP_TRUST_TABLE: selector trust table
// @DCB_ATTR_DCB_REWR_TABLE: rewrite configuration
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee_attrs {
    DCB_ATTR_IEEE_UNSPEC,
    DCB_ATTR_IEEE_ETS,
    DCB_ATTR_IEEE_PFC,
    DCB_ATTR_IEEE_APP_TABLE,
    DCB_ATTR_IEEE_PEER_ETS,
    DCB_ATTR_IEEE_PEER_PFC,
    DCB_ATTR_IEEE_PEER_APP,
    DCB_ATTR_IEEE_MAXRATE,
    DCB_ATTR_IEEE_QCN,
    DCB_ATTR_IEEE_QCN_STATS,
    DCB_ATTR_DCB_BUFFER,
    DCB_ATTR_DCB_APP_TRUST_TABLE,
    DCB_ATTR_DCB_REWR_TABLE,
    __DCB_ATTR_IEEE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee_attrs_app {
    DCB_ATTR_IEEE_APP_UNSPEC,
    DCB_ATTR_IEEE_APP,
    DCB_ATTR_DCB_APP,
    __DCB_ATTR_IEEE_APP_MAX
}

//
// enum cee_attrs - CEE DCBX get attributes.
//
// @DCB_ATTR_CEE_UNSPEC: unspecified
// @DCB_ATTR_CEE_PEER_PG: peer PG configuration - get only
// @DCB_ATTR_CEE_PEER_PFC: peer PFC configuration - get only
// @DCB_ATTR_CEE_PEER_APP_TABLE: peer APP tlv - get only
// @DCB_ATTR_CEE_TX_PG: TX PG configuration (DCB_CMD_PGTX_GCFG)
// @DCB_ATTR_CEE_RX_PG: RX PG configuration (DCB_CMD_PGRX_GCFG)
// @DCB_ATTR_CEE_PFC: PFC configuration (DCB_CMD_PFC_GCFG)
// @DCB_ATTR_CEE_APP_TABLE: APP configuration (multi DCB_CMD_GAPP)
// @DCB_ATTR_CEE_FEAT: DCBX features flags (DCB_CMD_GFEATCFG)
//
// An aggregated collection of the cee std negotiated parameters.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cee_attrs {
    DCB_ATTR_CEE_UNSPEC,
    DCB_ATTR_CEE_PEER_PG,
    DCB_ATTR_CEE_PEER_PFC,
    DCB_ATTR_CEE_PEER_APP_TABLE,
    DCB_ATTR_CEE_TX_PG,
    DCB_ATTR_CEE_RX_PG,
    DCB_ATTR_CEE_PFC,
    DCB_ATTR_CEE_APP_TABLE,
    DCB_ATTR_CEE_FEAT,
    __DCB_ATTR_CEE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum peer_app_attr {
    DCB_ATTR_CEE_PEER_APP_UNSPEC,
    DCB_ATTR_CEE_PEER_APP_INFO,
    DCB_ATTR_CEE_PEER_APP,
    __DCB_ATTR_CEE_PEER_APP_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cee_attrs_app {
    DCB_ATTR_CEE_APP_UNSPEC,
    DCB_ATTR_CEE_APP,
    __DCB_ATTR_CEE_APP_MAX
}

//
// enum dcbnl_pfc_attrs - DCB Priority Flow Control user priority nested attrs
//
// @DCB_PFC_UP_ATTR_UNDEFINED: unspecified attribute to catch errors
// @DCB_PFC_UP_ATTR_0: Priority Flow Control value for User Priority 0 (NLA_U8)
// @DCB_PFC_UP_ATTR_1: Priority Flow Control value for User Priority 1 (NLA_U8)
// @DCB_PFC_UP_ATTR_2: Priority Flow Control value for User Priority 2 (NLA_U8)
// @DCB_PFC_UP_ATTR_3: Priority Flow Control value for User Priority 3 (NLA_U8)
// @DCB_PFC_UP_ATTR_4: Priority Flow Control value for User Priority 4 (NLA_U8)
// @DCB_PFC_UP_ATTR_5: Priority Flow Control value for User Priority 5 (NLA_U8)
// @DCB_PFC_UP_ATTR_6: Priority Flow Control value for User Priority 6 (NLA_U8)
// @DCB_PFC_UP_ATTR_7: Priority Flow Control value for User Priority 7 (NLA_U8)
// @DCB_PFC_UP_ATTR_MAX: highest attribute number currently defined
// @DCB_PFC_UP_ATTR_ALL: apply to all priority flow control attrs (NLA_FLAG)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dcbnl_pfc_up_attrs {
    DCB_PFC_UP_ATTR_UNDEFINED,

    DCB_PFC_UP_ATTR_0,
    DCB_PFC_UP_ATTR_1,
    DCB_PFC_UP_ATTR_2,
    DCB_PFC_UP_ATTR_3,
    DCB_PFC_UP_ATTR_4,
    DCB_PFC_UP_ATTR_5,
    DCB_PFC_UP_ATTR_6,
    DCB_PFC_UP_ATTR_7,
    DCB_PFC_UP_ATTR_ALL,

    __DCB_PFC_UP_ATTR_ENUM_MAX,
    DCB_PFC_UP_ATTR_MAX = __DCB_PFC_UP_ATTR_ENUM_MAX - 1,
}

//
// enum dcbnl_pg_attrs - DCB Priority Group attributes
//
// @DCB_PG_ATTR_UNDEFINED: unspecified attribute to catch errors
// @DCB_PG_ATTR_TC_0: Priority Group Traffic Class 0 configuration (NLA_NESTED)
// @DCB_PG_ATTR_TC_1: Priority Group Traffic Class 1 configuration (NLA_NESTED)
// @DCB_PG_ATTR_TC_2: Priority Group Traffic Class 2 configuration (NLA_NESTED)
// @DCB_PG_ATTR_TC_3: Priority Group Traffic Class 3 configuration (NLA_NESTED)
// @DCB_PG_ATTR_TC_4: Priority Group Traffic Class 4 configuration (NLA_NESTED)
// @DCB_PG_ATTR_TC_5: Priority Group Traffic Class 5 configuration (NLA_NESTED)
// @DCB_PG_ATTR_TC_6: Priority Group Traffic Class 6 configuration (NLA_NESTED)
// @DCB_PG_ATTR_TC_7: Priority Group Traffic Class 7 configuration (NLA_NESTED)
// @DCB_PG_ATTR_TC_MAX: highest attribute number currently defined
// @DCB_PG_ATTR_TC_ALL: apply to all traffic classes (NLA_NESTED)
// @DCB_PG_ATTR_BW_ID_0: Percent of link bandwidth for Priority Group 0 (NLA_U8)
// @DCB_PG_ATTR_BW_ID_1: Percent of link bandwidth for Priority Group 1 (NLA_U8)
// @DCB_PG_ATTR_BW_ID_2: Percent of link bandwidth for Priority Group 2 (NLA_U8)
// @DCB_PG_ATTR_BW_ID_3: Percent of link bandwidth for Priority Group 3 (NLA_U8)
// @DCB_PG_ATTR_BW_ID_4: Percent of link bandwidth for Priority Group 4 (NLA_U8)
// @DCB_PG_ATTR_BW_ID_5: Percent of link bandwidth for Priority Group 5 (NLA_U8)
// @DCB_PG_ATTR_BW_ID_6: Percent of link bandwidth for Priority Group 6 (NLA_U8)
// @DCB_PG_ATTR_BW_ID_7: Percent of link bandwidth for Priority Group 7 (NLA_U8)
// @DCB_PG_ATTR_BW_ID_MAX: highest attribute number currently defined
// @DCB_PG_ATTR_BW_ID_ALL: apply to all priority groups (NLA_FLAG)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dcbnl_pg_attrs {
    DCB_PG_ATTR_UNDEFINED,

    DCB_PG_ATTR_TC_0,
    DCB_PG_ATTR_TC_1,
    DCB_PG_ATTR_TC_2,
    DCB_PG_ATTR_TC_3,
    DCB_PG_ATTR_TC_4,
    DCB_PG_ATTR_TC_5,
    DCB_PG_ATTR_TC_6,
    DCB_PG_ATTR_TC_7,
    DCB_PG_ATTR_TC_MAX,
    DCB_PG_ATTR_TC_ALL,

    DCB_PG_ATTR_BW_ID_0,
    DCB_PG_ATTR_BW_ID_1,
    DCB_PG_ATTR_BW_ID_2,
    DCB_PG_ATTR_BW_ID_3,
    DCB_PG_ATTR_BW_ID_4,
    DCB_PG_ATTR_BW_ID_5,
    DCB_PG_ATTR_BW_ID_6,
    DCB_PG_ATTR_BW_ID_7,
    DCB_PG_ATTR_BW_ID_MAX,
    DCB_PG_ATTR_BW_ID_ALL,

    __DCB_PG_ATTR_ENUM_MAX,
    DCB_PG_ATTR_MAX = __DCB_PG_ATTR_ENUM_MAX - 1,
}

//
// enum dcbnl_tc_attrs - DCB Traffic Class attributes
//
// @DCB_TC_ATTR_PARAM_UNDEFINED: unspecified attribute to catch errors
// @DCB_TC_ATTR_PARAM_PGID: (NLA_U8) Priority group the traffic class belongs to
// Valid values are:  0-7
// @DCB_TC_ATTR_PARAM_UP_MAPPING: (NLA_U8) Traffic class to user priority map
// Some devices may not support changing the
// user priority map of a TC.
// @DCB_TC_ATTR_PARAM_STRICT_PRIO: (NLA_U8) Strict priority setting
// 0 - none
// 1 - group strict
// 2 - link strict
// @DCB_TC_ATTR_PARAM_BW_PCT: optional - (NLA_U8) If supported by the device and
// not configured to use link strict priority,
// this is the percentage of bandwidth of the
// priority group this traffic class belongs to
// @DCB_TC_ATTR_PARAM_ALL: (NLA_FLAG) all traffic class parameters
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dcbnl_tc_attrs {
    DCB_TC_ATTR_PARAM_UNDEFINED,

    DCB_TC_ATTR_PARAM_PGID,
    DCB_TC_ATTR_PARAM_UP_MAPPING,
    DCB_TC_ATTR_PARAM_STRICT_PRIO,
    DCB_TC_ATTR_PARAM_BW_PCT,
    DCB_TC_ATTR_PARAM_ALL,

    __DCB_TC_ATTR_PARAM_ENUM_MAX,
    DCB_TC_ATTR_PARAM_MAX = __DCB_TC_ATTR_PARAM_ENUM_MAX - 1,
}

//
// enum dcbnl_cap_attrs - DCB Capability attributes
//
// @DCB_CAP_ATTR_UNDEFINED: unspecified attribute to catch errors
// @DCB_CAP_ATTR_ALL: (NLA_FLAG) all capability parameters
// @DCB_CAP_ATTR_PG: (NLA_U8) device supports Priority Groups
// @DCB_CAP_ATTR_PFC: (NLA_U8) device supports Priority Flow Control
// @DCB_CAP_ATTR_UP2TC: (NLA_U8) device supports user priority to
// traffic class mapping
// @DCB_CAP_ATTR_PG_TCS: (NLA_U8) bitmap where each bit represents a
// number of traffic classes the device
// can be configured to use for Priority Groups
// @DCB_CAP_ATTR_PFC_TCS: (NLA_U8) bitmap where each bit represents a
// number of traffic classes the device can be
// configured to use for Priority Flow Control
// @DCB_CAP_ATTR_GSP: (NLA_U8) device supports group strict priority
// @DCB_CAP_ATTR_BCN: (NLA_U8) device supports Backwards Congestion
// Notification
// @DCB_CAP_ATTR_DCBX: (NLA_U8) device supports DCBX engine
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dcbnl_cap_attrs {
    DCB_CAP_ATTR_UNDEFINED,
    DCB_CAP_ATTR_ALL,
    DCB_CAP_ATTR_PG,
    DCB_CAP_ATTR_PFC,
    DCB_CAP_ATTR_UP2TC,
    DCB_CAP_ATTR_PG_TCS,
    DCB_CAP_ATTR_PFC_TCS,
    DCB_CAP_ATTR_GSP,
    DCB_CAP_ATTR_BCN,
    DCB_CAP_ATTR_DCBX,

    __DCB_CAP_ATTR_ENUM_MAX,
    DCB_CAP_ATTR_MAX = __DCB_CAP_ATTR_ENUM_MAX - 1,
}

//
// DCBX capability flags
//
// @DCB_CAP_DCBX_HOST: DCBX negotiation is performed by the host LLDP agent.
// 'set' routines are used to configure the device with
// the negotiated parameters
//
// @DCB_CAP_DCBX_LLD_MANAGED: DCBX negotiation is not performed in the host but
// by another entity
// 'get' routines are used to retrieve the
// negotiated parameters
// 'set' routines can be used to set the initial
// negotiation configuration
//
// @DCB_CAP_DCBX_VER_CEE: for a non-host DCBX engine, indicates the engine
// supports the CEE protocol flavor
//
// @DCB_CAP_DCBX_VER_IEEE: for a non-host DCBX engine, indicates the engine
// supports the IEEE protocol flavor
//
// @DCB_CAP_DCBX_STATIC: for a non-host DCBX engine, indicates the engine
// supports static configuration (i.e no actual
// negotiation is performed negotiated parameters equal
// the initial configuration)
//
pub const DCB_CAP_DCBX_HOST: c_uint = 0x01;
pub const DCB_CAP_DCBX_LLD_MANAGED: c_uint = 0x02;
pub const DCB_CAP_DCBX_VER_CEE: c_uint = 0x04;
pub const DCB_CAP_DCBX_VER_IEEE: c_uint = 0x08;
pub const DCB_CAP_DCBX_STATIC: c_uint = 0x10;
//
// enum dcbnl_numtcs_attrs - number of traffic classes
//
// @DCB_NUMTCS_ATTR_UNDEFINED: unspecified attribute to catch errors
// @DCB_NUMTCS_ATTR_ALL: (NLA_FLAG) all traffic class attributes
// @DCB_NUMTCS_ATTR_PG: (NLA_U8) number of traffic classes used for
// priority groups
// @DCB_NUMTCS_ATTR_PFC: (NLA_U8) number of traffic classes which can
// support priority flow control
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dcbnl_numtcs_attrs {
    DCB_NUMTCS_ATTR_UNDEFINED,
    DCB_NUMTCS_ATTR_ALL,
    DCB_NUMTCS_ATTR_PG,
    DCB_NUMTCS_ATTR_PFC,

    __DCB_NUMTCS_ATTR_ENUM_MAX,
    DCB_NUMTCS_ATTR_MAX = __DCB_NUMTCS_ATTR_ENUM_MAX - 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dcbnl_bcn_attrs {
    DCB_BCN_ATTR_UNDEFINED = 0,

    DCB_BCN_ATTR_RP_0,
    DCB_BCN_ATTR_RP_1,
    DCB_BCN_ATTR_RP_2,
    DCB_BCN_ATTR_RP_3,
    DCB_BCN_ATTR_RP_4,
    DCB_BCN_ATTR_RP_5,
    DCB_BCN_ATTR_RP_6,
    DCB_BCN_ATTR_RP_7,
    DCB_BCN_ATTR_RP_ALL,

    DCB_BCN_ATTR_BCNA_0,
    DCB_BCN_ATTR_BCNA_1,
    DCB_BCN_ATTR_ALPHA,
    DCB_BCN_ATTR_BETA,
    DCB_BCN_ATTR_GD,
    DCB_BCN_ATTR_GI,
    DCB_BCN_ATTR_TMAX,
    DCB_BCN_ATTR_TD,
    DCB_BCN_ATTR_RMIN,
    DCB_BCN_ATTR_W,
    DCB_BCN_ATTR_RD,
    DCB_BCN_ATTR_RU,
    DCB_BCN_ATTR_WRTT,
    DCB_BCN_ATTR_RI,
    DCB_BCN_ATTR_C,
    DCB_BCN_ATTR_ALL,

    __DCB_BCN_ATTR_ENUM_MAX,
    DCB_BCN_ATTR_MAX = __DCB_BCN_ATTR_ENUM_MAX - 1,
}

//
// enum dcb_general_attr_values - general DCB attribute values
//
// @DCB_ATTR_UNDEFINED: value used to indicate an attribute is not supported
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dcb_general_attr_values {
    DCB_ATTR_VALUE_UNDEFINED = 0xff
}

pub const DCB_APP_IDTYPE_ETHTYPE: c_uint = 0x00;
pub const DCB_APP_IDTYPE_PORTNUM: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dcbnl_app_attrs {
    DCB_APP_ATTR_UNDEFINED,

    DCB_APP_ATTR_IDTYPE,
    DCB_APP_ATTR_ID,
    DCB_APP_ATTR_PRIORITY,

    __DCB_APP_ATTR_ENUM_MAX,
    DCB_APP_ATTR_MAX = __DCB_APP_ATTR_ENUM_MAX - 1,
}

//
// enum dcbnl_featcfg_attrs - features conifiguration flags
//
// @DCB_FEATCFG_ATTR_UNDEFINED: unspecified attribute to catch errors
// @DCB_FEATCFG_ATTR_ALL: (NLA_FLAG) all features configuration attributes
// @DCB_FEATCFG_ATTR_PG: (NLA_U8) configuration flags for priority groups
// @DCB_FEATCFG_ATTR_PFC: (NLA_U8) configuration flags for priority
// flow control
// @DCB_FEATCFG_ATTR_APP: (NLA_U8) configuration flags for application TLV
//
pub const DCB_FEATCFG_ERROR: c_uint = 0x01	/* error in feature resolution */;
pub const DCB_FEATCFG_ENABLE: c_uint = 0x02	/* enable feature */;
pub const DCB_FEATCFG_WILLING: c_uint = 0x04	/* feature is willing */;
pub const DCB_FEATCFG_ADVERTISE: c_uint = 0x08	/* advertise feature */;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dcbnl_featcfg_attrs {
    DCB_FEATCFG_ATTR_UNDEFINED,
    DCB_FEATCFG_ATTR_ALL,
    DCB_FEATCFG_ATTR_PG,
    DCB_FEATCFG_ATTR_PFC,
    DCB_FEATCFG_ATTR_APP,

    __DCB_FEATCFG_ATTR_ENUM_MAX,
    DCB_FEATCFG_ATTR_MAX = __DCB_FEATCFG_ATTR_ENUM_MAX - 1,
}
