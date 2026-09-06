//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/iscsi_if.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// iSCSI User/Kernel Shares (Defines, Constants, Protocol definitions, etc)
//
// Copyright (C) 2005 Dmitry Yusupov
// Copyright (C) 2005 Alex Aizman
// maintained by open-iscsi@googlegroups.com
//

pub const ISCSI_NL_GRP_ISCSID: c_int = 1;
pub const ISCSI_NL_GRP_UIP: c_int = 2;
pub const UEVENT_BASE: c_int = 10;
pub const KEVENT_BASE: c_int = 100;
pub const ISCSI_ERR_BASE: c_int = 1000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iscsi_uevent_e {
    ISCSI_UEVENT_UNKNOWN		= 0,

// down events
    ISCSI_UEVENT_CREATE_SESSION	= UEVENT_BASE + 1,
    ISCSI_UEVENT_DESTROY_SESSION	= UEVENT_BASE + 2,
    ISCSI_UEVENT_CREATE_CONN	= UEVENT_BASE + 3,
    ISCSI_UEVENT_DESTROY_CONN	= UEVENT_BASE + 4,
    ISCSI_UEVENT_BIND_CONN		= UEVENT_BASE + 5,
    ISCSI_UEVENT_SET_PARAM		= UEVENT_BASE + 6,
    ISCSI_UEVENT_START_CONN		= UEVENT_BASE + 7,
    ISCSI_UEVENT_STOP_CONN		= UEVENT_BASE + 8,
    ISCSI_UEVENT_SEND_PDU		= UEVENT_BASE + 9,
    ISCSI_UEVENT_GET_STATS		= UEVENT_BASE + 10,
    ISCSI_UEVENT_GET_PARAM		= UEVENT_BASE + 11,

    ISCSI_UEVENT_TRANSPORT_EP_CONNECT	= UEVENT_BASE + 12,
    ISCSI_UEVENT_TRANSPORT_EP_POLL		= UEVENT_BASE + 13,
    ISCSI_UEVENT_TRANSPORT_EP_DISCONNECT	= UEVENT_BASE + 14,

    ISCSI_UEVENT_TGT_DSCVR		= UEVENT_BASE + 15,
    ISCSI_UEVENT_SET_HOST_PARAM	= UEVENT_BASE + 16,
    ISCSI_UEVENT_UNBIND_SESSION	= UEVENT_BASE + 17,
    ISCSI_UEVENT_CREATE_BOUND_SESSION		= UEVENT_BASE + 18,
    ISCSI_UEVENT_TRANSPORT_EP_CONNECT_THROUGH_HOST	= UEVENT_BASE + 19,

    ISCSI_UEVENT_PATH_UPDATE	= UEVENT_BASE + 20,
    ISCSI_UEVENT_SET_IFACE_PARAMS	= UEVENT_BASE + 21,
    ISCSI_UEVENT_PING		= UEVENT_BASE + 22,
    ISCSI_UEVENT_GET_CHAP		= UEVENT_BASE + 23,
    ISCSI_UEVENT_DELETE_CHAP	= UEVENT_BASE + 24,
    ISCSI_UEVENT_SET_FLASHNODE_PARAMS	= UEVENT_BASE + 25,
    ISCSI_UEVENT_NEW_FLASHNODE	= UEVENT_BASE + 26,
    ISCSI_UEVENT_DEL_FLASHNODE	= UEVENT_BASE + 27,
    ISCSI_UEVENT_LOGIN_FLASHNODE	= UEVENT_BASE + 28,
    ISCSI_UEVENT_LOGOUT_FLASHNODE	= UEVENT_BASE + 29,
    ISCSI_UEVENT_LOGOUT_FLASHNODE_SID	= UEVENT_BASE + 30,
    ISCSI_UEVENT_SET_CHAP		= UEVENT_BASE + 31,
    ISCSI_UEVENT_GET_HOST_STATS	= UEVENT_BASE + 32,
    ISCSI_UEVENT_DESTROY_SESSION_ASYNC	= UEVENT_BASE + 33,

// up events
    ISCSI_KEVENT_RECV_PDU		= KEVENT_BASE + 1,
    ISCSI_KEVENT_CONN_ERROR		= KEVENT_BASE + 2,
    ISCSI_KEVENT_IF_ERROR		= KEVENT_BASE + 3,
    ISCSI_KEVENT_DESTROY_SESSION	= KEVENT_BASE + 4,
    ISCSI_KEVENT_UNBIND_SESSION	= KEVENT_BASE + 5,
    ISCSI_KEVENT_CREATE_SESSION	= KEVENT_BASE + 6,

    ISCSI_KEVENT_PATH_REQ		= KEVENT_BASE + 7,
    ISCSI_KEVENT_IF_DOWN		= KEVENT_BASE + 8,
    ISCSI_KEVENT_CONN_LOGIN_STATE   = KEVENT_BASE + 9,
    ISCSI_KEVENT_HOST_EVENT		= KEVENT_BASE + 10,
    ISCSI_KEVENT_PING_COMP		= KEVENT_BASE + 11,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iscsi_tgt_dscvr {
    ISCSI_TGT_DSCVR_SEND_TARGETS	= 1,
    ISCSI_TGT_DSCVR_ISNS		= 2,
    ISCSI_TGT_DSCVR_SLP		= 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iscsi_host_event_code {
    ISCSI_EVENT_LINKUP		= 1,
    ISCSI_EVENT_LINKDOWN,
// must always be last
    ISCSI_EVENT_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_uevent {
    pub /: *mut *mut uint32_t type; / k/u events type,
    pub /: *mut *mut uint32_t iferror; / carries interface or resource errors,
    pub transport_handle: u64,
// messages u -> k
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_create_session {
    pub initial_cmdsn: u32,
    pub cmds_max: u16,
    pub queue_depth: u16,
    pub c_session: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_create_bound_session {
    pub ep_handle: u64,
    pub initial_cmdsn: u32,
    pub cmds_max: u16,
    pub queue_depth: u16,
    pub c_bound_session: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_destroy_session {
    pub sid: u32,
    pub d_session: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_create_conn {
    pub sid: u32,
    pub cid: u32,
    pub c_conn: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_bind_conn {
    pub sid: u32,
    pub cid: u32,
    pub transport_eph: u64,
    pub is_leading: u32,
    pub b_conn: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_destroy_conn {
    pub sid: u32,
    pub cid: u32,
    pub d_conn: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_send_pdu {
    pub sid: u32,
    pub cid: u32,
    pub hdr_size: u32,
    pub data_size: u32,
    pub send_pdu: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_set_param {
    pub sid: u32,
    pub cid: u32,
    pub /: *mut *mut uint32_t param; / enum iscsi_param,
    pub len: u32,
    pub set_param: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_start_conn {
    pub sid: u32,
    pub cid: u32,
    pub start_conn: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_stop_conn {
    pub sid: u32,
    pub cid: u32,
    pub conn_handle: u64,
    pub flag: u32,
    pub stop_conn: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_get_stats {
    pub sid: u32,
    pub cid: u32,
    pub get_stats: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_transport_connect {
    pub non_blocking: u32,
    pub ep_connect: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_transport_connect_through_host {
    pub host_no: u32,
    pub non_blocking: u32,
    pub ep_connect_through_host: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_transport_poll {
    pub ep_handle: u64,
    pub timeout_ms: u32,
    pub ep_poll: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_transport_disconnect {
    pub ep_handle: u64,
    pub ep_disconnect: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_tgt_dscvr {
    pub type: iscsi_tgt_dscvr,
    pub host_no: u32,
//
// enable = 1 to establish a new connection
// with the server. enable = 0 to disconnect
// from the server. Used primarily to switch
// from one iSNS server to another.
//
    pub enable: u32,
    pub tgt_dscvr: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_set_host_param {
    pub host_no: u32,
    pub /: *mut *mut uint32_t param; / enum iscsi_host_param,
    pub len: u32,
    pub set_host_param: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_set_path {
    pub host_no: u32,
    pub set_path: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_set_iface_params {
    pub host_no: u32,
    pub count: u32,
    pub set_iface_params: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_iscsi_ping {
    pub host_no: u32,
    pub iface_num: u32,
    pub iface_type: u32,
    pub payload_size: u32,
    pub associated: *mut *mut uint32_t pid; / unique ping id,
    pub iscsi_ping: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_get_chap {
    pub host_no: u32,
    pub entries: *mut *mut uint32_t num_entries; / number of CHAP,
// on request, number of
// valid CHAP entries on
// response
    pub chap_tbl_idx: u16,
    pub get_chap: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_delete_chap {
    pub host_no: u32,
    pub chap_tbl_idx: u16,
    pub delete_chap: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_set_flashnode_param {
    pub host_no: u32,
    pub flashnode_idx: u32,
    pub count: u32,
    pub set_flashnode: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_new_flashnode {
    pub host_no: u32,
    pub len: u32,
    pub new_flashnode: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_del_flashnode {
    pub host_no: u32,
    pub flashnode_idx: u32,
    pub del_flashnode: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_login_flashnode {
    pub host_no: u32,
    pub flashnode_idx: u32,
    pub login_flashnode: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_logout_flashnode {
    pub host_no: u32,
    pub flashnode_idx: u32,
    pub logout_flashnode: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_logout_flashnode_sid {
    pub host_no: u32,
    pub sid: u32,
    pub logout_flashnode_sid: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_get_host_stats {
    pub host_no: u32,
    pub get_host_stats: },
    pub u: },
// messages k -> u
    pub retcode: c_int,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_create_session_ret {
    pub sid: u32,
    pub host_no: u32,
    pub c_session_ret: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_create_conn_ret {
    pub sid: u32,
    pub cid: u32,
    pub c_conn_ret: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_unbind_session {
    pub sid: u32,
    pub host_no: u32,
    pub unbind_session: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_recv_req {
    pub sid: u32,
    pub cid: u32,
    pub recv_handle: u64,
    pub recv_req: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_conn_login {
    pub sid: u32,
    pub cid: u32,
    pub /: *mut *mut uint32_t state; / enum iscsi_conn_state,
    pub conn_login: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_conn_error {
    pub sid: u32,
    pub cid: u32,
    pub /: *mut *mut uint32_t error; / enum iscsi_err,
    pub connerror: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_session_destroyed {
    pub host_no: u32,
    pub sid: u32,
    pub d_session: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_transport_connect_ret {
    pub handle: u64,
    pub ep_connect_ret: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_req_path {
    pub host_no: u32,
    pub req_path: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_notify_if_down {
    pub host_no: u32,
    pub notify_if_down: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_host_event {
    pub host_no: u32,
    pub data_size: u32,
    pub code: iscsi_host_event_code,
    pub host_event: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_ping_comp {
    pub host_no: u32,
    pub enum: *mut *mut uint32_t status; /,
// iscsi_ping_status_code
    pub associated: *mut *mut uint32_t pid; / unique ping id,
    pub data_size: u32,
    pub ping_comp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_new_flashnode_ret {
    pub flashnode_idx: u32,
    pub new_flashnode_ret: },
    pub r: },
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iscsi_param_type {
    ISCSI_PARAM,		/* iscsi_param (session, conn, target, LU) */
    ISCSI_HOST_PARAM,	/* iscsi_host_param */
    ISCSI_NET_PARAM,	/* iscsi_net_param */
    ISCSI_FLASHNODE_PARAM,	/* iscsi_flashnode_param */
    ISCSI_CHAP_PARAM,	/* iscsi_chap_param */
    ISCSI_IFACE_PARAM,	/* iscsi_iface_param */
}

// structure for minimalist usecase
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_param_info {
    pub /: *mut *mut uint32_t len; / Actual length of the param value,
    pub /: *mut *mut uint16_t param; / iscsi param,
    pub /: *mut *mut uint8_t value[]; / length sized value follows,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_iface_param_info {
    pub /: *mut *mut uint32_t iface_num; / iface number, 0 - n,
    pub /: *mut *mut uint32_t len; / Actual length of the param,
    pub /: *mut *mut uint16_t param; / iscsi param value,
    pub /: *mut *mut uint8_t iface_type; / IPv4 or IPv6,
    pub /: *mut *mut uint8_t param_type; / iscsi_param_type,
    pub /: *mut *mut uint8_t value[]; / length sized value follows,
    pub __packed: },
//
// To keep the struct iscsi_uevent size the same for userspace code
// compatibility, the main structure for ISCSI_UEVENT_PATH_UPDATE and
// ISCSI_KEVENT_PATH_REQ is defined separately and comes after the
// struct iscsi_uevent in the NETLINK_ISCSI message.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_path {
    pub handle: u64,
    pub mac_addr: [u8; 6],
    pub mac_addr_old: [u8; 6],
    pub /: *mut *mut uint32_t ip_addr_len; / 4 or 16,
    pub v4_addr: in_addr,
    pub v6_addr: in6_addr,
    pub src: },
    pub v4_addr: in_addr,
    pub v6_addr: in6_addr,
    pub dst: },
    pub vlan_id: u16,
    pub pmtu: u16,
// C attribute field omitted
// iscsi iface enabled/disabled setting
pub const ISCSI_IFACE_DISABLE: c_uint = 0x01;
pub const ISCSI_IFACE_ENABLE: c_uint = 0x02;
// ipv4 bootproto
pub const ISCSI_BOOTPROTO_STATIC: c_uint = 0x01;
pub const ISCSI_BOOTPROTO_DHCP: c_uint = 0x02;
// ipv6 addr autoconfig type
pub const ISCSI_IPV6_AUTOCFG_DISABLE: c_uint = 0x01;
pub const ISCSI_IPV6_AUTOCFG_ND_ENABLE: c_uint = 0x02;
pub const ISCSI_IPV6_AUTOCFG_DHCPV6_ENABLE: c_uint = 0x03;
// ipv6 link local addr type
pub const ISCSI_IPV6_LINKLOCAL_AUTOCFG_ENABLE: c_uint = 0x01;
pub const ISCSI_IPV6_LINKLOCAL_AUTOCFG_DISABLE: c_uint = 0x02;
// ipv6 router addr type
pub const ISCSI_IPV6_ROUTER_AUTOCFG_ENABLE: c_uint = 0x01;
pub const ISCSI_IPV6_ROUTER_AUTOCFG_DISABLE: c_uint = 0x02;
pub const ISCSI_IFACE_TYPE_IPV4: c_uint = 0x01;
pub const ISCSI_IFACE_TYPE_IPV6: c_uint = 0x02;
pub const ISCSI_MAX_VLAN_ID: c_int = 4095;
pub const ISCSI_MAX_VLAN_PRIORITY: c_int = 7;
// iscsi vlan enable/disabled setting
pub const ISCSI_VLAN_DISABLE: c_uint = 0x01;
pub const ISCSI_VLAN_ENABLE: c_uint = 0x02;
// iscsi generic enable/disabled setting for various features
pub const ISCSI_NET_PARAM_DISABLE: c_uint = 0x01;
pub const ISCSI_NET_PARAM_ENABLE: c_uint = 0x02;
// iSCSI network params
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iscsi_net_param {
    ISCSI_NET_PARAM_IPV4_ADDR		= 1,
    ISCSI_NET_PARAM_IPV4_SUBNET,
    ISCSI_NET_PARAM_IPV4_GW,
    ISCSI_NET_PARAM_IPV4_BOOTPROTO,
    ISCSI_NET_PARAM_MAC,
    ISCSI_NET_PARAM_IPV6_LINKLOCAL,
    ISCSI_NET_PARAM_IPV6_ADDR,
    ISCSI_NET_PARAM_IPV6_ROUTER,
    ISCSI_NET_PARAM_IPV6_ADDR_AUTOCFG,
    ISCSI_NET_PARAM_IPV6_LINKLOCAL_AUTOCFG,
    ISCSI_NET_PARAM_IPV6_ROUTER_AUTOCFG,
    ISCSI_NET_PARAM_IFACE_ENABLE,
    ISCSI_NET_PARAM_VLAN_ID,
    ISCSI_NET_PARAM_VLAN_PRIORITY,
    ISCSI_NET_PARAM_VLAN_ENABLED,
    ISCSI_NET_PARAM_VLAN_TAG,
    ISCSI_NET_PARAM_IFACE_TYPE,
    ISCSI_NET_PARAM_IFACE_NAME,
    ISCSI_NET_PARAM_MTU,
    ISCSI_NET_PARAM_PORT,
    ISCSI_NET_PARAM_IPADDR_STATE,
    ISCSI_NET_PARAM_IPV6_LINKLOCAL_STATE,
    ISCSI_NET_PARAM_IPV6_ROUTER_STATE,
    ISCSI_NET_PARAM_DELAYED_ACK_EN,
    ISCSI_NET_PARAM_TCP_NAGLE_DISABLE,
    ISCSI_NET_PARAM_TCP_WSF_DISABLE,
    ISCSI_NET_PARAM_TCP_WSF,
    ISCSI_NET_PARAM_TCP_TIMER_SCALE,
    ISCSI_NET_PARAM_TCP_TIMESTAMP_EN,
    ISCSI_NET_PARAM_CACHE_ID,
    ISCSI_NET_PARAM_IPV4_DHCP_DNS_ADDR_EN,
    ISCSI_NET_PARAM_IPV4_DHCP_SLP_DA_EN,
    ISCSI_NET_PARAM_IPV4_TOS_EN,
    ISCSI_NET_PARAM_IPV4_TOS,
    ISCSI_NET_PARAM_IPV4_GRAT_ARP_EN,
    ISCSI_NET_PARAM_IPV4_DHCP_ALT_CLIENT_ID_EN,
    ISCSI_NET_PARAM_IPV4_DHCP_ALT_CLIENT_ID,
    ISCSI_NET_PARAM_IPV4_DHCP_REQ_VENDOR_ID_EN,
    ISCSI_NET_PARAM_IPV4_DHCP_USE_VENDOR_ID_EN,
    ISCSI_NET_PARAM_IPV4_DHCP_VENDOR_ID,
    ISCSI_NET_PARAM_IPV4_DHCP_LEARN_IQN_EN,
    ISCSI_NET_PARAM_IPV4_FRAGMENT_DISABLE,
    ISCSI_NET_PARAM_IPV4_IN_FORWARD_EN,
    ISCSI_NET_PARAM_IPV4_TTL,
    ISCSI_NET_PARAM_IPV6_GRAT_NEIGHBOR_ADV_EN,
    ISCSI_NET_PARAM_IPV6_MLD_EN,
    ISCSI_NET_PARAM_IPV6_FLOW_LABEL,
    ISCSI_NET_PARAM_IPV6_TRAFFIC_CLASS,
    ISCSI_NET_PARAM_IPV6_HOP_LIMIT,
    ISCSI_NET_PARAM_IPV6_ND_REACHABLE_TMO,
    ISCSI_NET_PARAM_IPV6_ND_REXMIT_TIME,
    ISCSI_NET_PARAM_IPV6_ND_STALE_TMO,
    ISCSI_NET_PARAM_IPV6_DUP_ADDR_DETECT_CNT,
    ISCSI_NET_PARAM_IPV6_RTR_ADV_LINK_MTU,
    ISCSI_NET_PARAM_REDIRECT_EN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iscsi_ipaddress_state {
    ISCSI_IPDDRESS_STATE_UNCONFIGURED,
    ISCSI_IPDDRESS_STATE_ACQUIRING,
    ISCSI_IPDDRESS_STATE_TENTATIVE,
    ISCSI_IPDDRESS_STATE_VALID,
    ISCSI_IPDDRESS_STATE_DISABLING,
    ISCSI_IPDDRESS_STATE_INVALID,
    ISCSI_IPDDRESS_STATE_DEPRECATED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iscsi_router_state {
    ISCSI_ROUTER_STATE_UNKNOWN,
    ISCSI_ROUTER_STATE_ADVERTISED,
    ISCSI_ROUTER_STATE_MANUAL,
    ISCSI_ROUTER_STATE_STALE,
}

// iSCSI specific settings params for iface
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iscsi_iface_param {
    ISCSI_IFACE_PARAM_DEF_TASKMGMT_TMO,
    ISCSI_IFACE_PARAM_HDRDGST_EN,
    ISCSI_IFACE_PARAM_DATADGST_EN,
    ISCSI_IFACE_PARAM_IMM_DATA_EN,
    ISCSI_IFACE_PARAM_INITIAL_R2T_EN,
    ISCSI_IFACE_PARAM_DATASEQ_INORDER_EN,
    ISCSI_IFACE_PARAM_PDU_INORDER_EN,
    ISCSI_IFACE_PARAM_ERL,
    ISCSI_IFACE_PARAM_MAX_RECV_DLENGTH,
    ISCSI_IFACE_PARAM_FIRST_BURST,
    ISCSI_IFACE_PARAM_MAX_R2T,
    ISCSI_IFACE_PARAM_MAX_BURST,
    ISCSI_IFACE_PARAM_CHAP_AUTH_EN,
    ISCSI_IFACE_PARAM_BIDI_CHAP_EN,
    ISCSI_IFACE_PARAM_DISCOVERY_AUTH_OPTIONAL,
    ISCSI_IFACE_PARAM_DISCOVERY_LOGOUT_EN,
    ISCSI_IFACE_PARAM_STRICT_LOGIN_COMP_EN,
    ISCSI_IFACE_PARAM_INITIATOR_NAME,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iscsi_conn_state {
    ISCSI_CONN_STATE_FREE,
    ISCSI_CONN_STATE_XPT_WAIT,
    ISCSI_CONN_STATE_IN_LOGIN,
    ISCSI_CONN_STATE_LOGGED_IN,
    ISCSI_CONN_STATE_IN_LOGOUT,
    ISCSI_CONN_STATE_LOGOUT_REQUESTED,
    ISCSI_CONN_STATE_CLEANUP_WAIT,
}

//
// Common error codes
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iscsi_err {
    ISCSI_OK			= 0,

    ISCSI_ERR_DATASN		= ISCSI_ERR_BASE + 1,
    ISCSI_ERR_DATA_OFFSET		= ISCSI_ERR_BASE + 2,
    ISCSI_ERR_MAX_CMDSN		= ISCSI_ERR_BASE + 3,
    ISCSI_ERR_EXP_CMDSN		= ISCSI_ERR_BASE + 4,
    ISCSI_ERR_BAD_OPCODE		= ISCSI_ERR_BASE + 5,
    ISCSI_ERR_DATALEN		= ISCSI_ERR_BASE + 6,
    ISCSI_ERR_AHSLEN		= ISCSI_ERR_BASE + 7,
    ISCSI_ERR_PROTO			= ISCSI_ERR_BASE + 8,
    ISCSI_ERR_LUN			= ISCSI_ERR_BASE + 9,
    ISCSI_ERR_BAD_ITT		= ISCSI_ERR_BASE + 10,
    ISCSI_ERR_CONN_FAILED		= ISCSI_ERR_BASE + 11,
    ISCSI_ERR_R2TSN			= ISCSI_ERR_BASE + 12,
    ISCSI_ERR_SESSION_FAILED	= ISCSI_ERR_BASE + 13,
    ISCSI_ERR_HDR_DGST		= ISCSI_ERR_BASE + 14,
    ISCSI_ERR_DATA_DGST		= ISCSI_ERR_BASE + 15,
    ISCSI_ERR_PARAM_NOT_FOUND	= ISCSI_ERR_BASE + 16,
    ISCSI_ERR_NO_SCSI_CMD		= ISCSI_ERR_BASE + 17,
    ISCSI_ERR_INVALID_HOST		= ISCSI_ERR_BASE + 18,
    ISCSI_ERR_XMIT_FAILED		= ISCSI_ERR_BASE + 19,
    ISCSI_ERR_TCP_CONN_CLOSE	= ISCSI_ERR_BASE + 20,
    ISCSI_ERR_SCSI_EH_SESSION_RST	= ISCSI_ERR_BASE + 21,
    ISCSI_ERR_NOP_TIMEDOUT		= ISCSI_ERR_BASE + 22,
}

//
// iSCSI Parameters (RFC3720)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iscsi_param {
// passed in using netlink set param
    ISCSI_PARAM_MAX_RECV_DLENGTH,
    ISCSI_PARAM_MAX_XMIT_DLENGTH,
    ISCSI_PARAM_HDRDGST_EN,
    ISCSI_PARAM_DATADGST_EN,
    ISCSI_PARAM_INITIAL_R2T_EN,
    ISCSI_PARAM_MAX_R2T,
    ISCSI_PARAM_IMM_DATA_EN,
    ISCSI_PARAM_FIRST_BURST,
    ISCSI_PARAM_MAX_BURST,
    ISCSI_PARAM_PDU_INORDER_EN,
    ISCSI_PARAM_DATASEQ_INORDER_EN,
    ISCSI_PARAM_ERL,
    ISCSI_PARAM_IFMARKER_EN,
    ISCSI_PARAM_OFMARKER_EN,
    ISCSI_PARAM_EXP_STATSN,
    ISCSI_PARAM_TARGET_NAME,
    ISCSI_PARAM_TPGT,
    ISCSI_PARAM_PERSISTENT_ADDRESS,
    ISCSI_PARAM_PERSISTENT_PORT,
    ISCSI_PARAM_SESS_RECOVERY_TMO,

// passed in through bind conn using transport_fd
    ISCSI_PARAM_CONN_PORT,
    ISCSI_PARAM_CONN_ADDRESS,

    ISCSI_PARAM_USERNAME,
    ISCSI_PARAM_USERNAME_IN,
    ISCSI_PARAM_PASSWORD,
    ISCSI_PARAM_PASSWORD_IN,

    ISCSI_PARAM_FAST_ABORT,
    ISCSI_PARAM_ABORT_TMO,
    ISCSI_PARAM_LU_RESET_TMO,
    ISCSI_PARAM_HOST_RESET_TMO,

    ISCSI_PARAM_PING_TMO,
    ISCSI_PARAM_RECV_TMO,

    ISCSI_PARAM_IFACE_NAME,
    ISCSI_PARAM_ISID,
    ISCSI_PARAM_INITIATOR_NAME,

    ISCSI_PARAM_TGT_RESET_TMO,
    ISCSI_PARAM_TARGET_ALIAS,

    ISCSI_PARAM_CHAP_IN_IDX,
    ISCSI_PARAM_CHAP_OUT_IDX,

    ISCSI_PARAM_BOOT_ROOT,
    ISCSI_PARAM_BOOT_NIC,
    ISCSI_PARAM_BOOT_TARGET,

    ISCSI_PARAM_AUTO_SND_TGT_DISABLE,
    ISCSI_PARAM_DISCOVERY_SESS,
    ISCSI_PARAM_PORTAL_TYPE,
    ISCSI_PARAM_CHAP_AUTH_EN,
    ISCSI_PARAM_DISCOVERY_LOGOUT_EN,
    ISCSI_PARAM_BIDI_CHAP_EN,
    ISCSI_PARAM_DISCOVERY_AUTH_OPTIONAL,

    ISCSI_PARAM_DEF_TIME2WAIT,
    ISCSI_PARAM_DEF_TIME2RETAIN,
    ISCSI_PARAM_MAX_SEGMENT_SIZE,
    ISCSI_PARAM_STATSN,
    ISCSI_PARAM_KEEPALIVE_TMO,
    ISCSI_PARAM_LOCAL_PORT,
    ISCSI_PARAM_TSID,
    ISCSI_PARAM_DEF_TASKMGMT_TMO,

    ISCSI_PARAM_TCP_TIMESTAMP_STAT,
    ISCSI_PARAM_TCP_WSF_DISABLE,
    ISCSI_PARAM_TCP_NAGLE_DISABLE,
    ISCSI_PARAM_TCP_TIMER_SCALE,
    ISCSI_PARAM_TCP_TIMESTAMP_EN,
    ISCSI_PARAM_TCP_XMIT_WSF,
    ISCSI_PARAM_TCP_RECV_WSF,
    ISCSI_PARAM_IP_FRAGMENT_DISABLE,
    ISCSI_PARAM_IPV4_TOS,
    ISCSI_PARAM_IPV6_TC,
    ISCSI_PARAM_IPV6_FLOW_LABEL,
    ISCSI_PARAM_IS_FW_ASSIGNED_IPV6,

    ISCSI_PARAM_DISCOVERY_PARENT_IDX,
    ISCSI_PARAM_DISCOVERY_PARENT_TYPE,
    ISCSI_PARAM_LOCAL_IPADDR,
// must always be last
    ISCSI_PARAM_MAX,
}

// iSCSI HBA params
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iscsi_host_param {
    ISCSI_HOST_PARAM_HWADDRESS,
    ISCSI_HOST_PARAM_INITIATOR_NAME,
    ISCSI_HOST_PARAM_NETDEV_NAME,
    ISCSI_HOST_PARAM_IPADDRESS,
    ISCSI_HOST_PARAM_PORT_STATE,
    ISCSI_HOST_PARAM_PORT_SPEED,
    ISCSI_HOST_PARAM_MAX,
}

// portal type

// iSCSI Flash Target params
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iscsi_flashnode_param {
    ISCSI_FLASHNODE_IS_FW_ASSIGNED_IPV6,
    ISCSI_FLASHNODE_PORTAL_TYPE,
    ISCSI_FLASHNODE_AUTO_SND_TGT_DISABLE,
    ISCSI_FLASHNODE_DISCOVERY_SESS,
    ISCSI_FLASHNODE_ENTRY_EN,
    ISCSI_FLASHNODE_HDR_DGST_EN,
    ISCSI_FLASHNODE_DATA_DGST_EN,
    ISCSI_FLASHNODE_IMM_DATA_EN,
    ISCSI_FLASHNODE_INITIAL_R2T_EN,
    ISCSI_FLASHNODE_DATASEQ_INORDER,
    ISCSI_FLASHNODE_PDU_INORDER,
    ISCSI_FLASHNODE_CHAP_AUTH_EN,
    ISCSI_FLASHNODE_SNACK_REQ_EN,
    ISCSI_FLASHNODE_DISCOVERY_LOGOUT_EN,
    ISCSI_FLASHNODE_BIDI_CHAP_EN,
// make authentication for discovery sessions optional
    ISCSI_FLASHNODE_DISCOVERY_AUTH_OPTIONAL,
    ISCSI_FLASHNODE_ERL,
    ISCSI_FLASHNODE_TCP_TIMESTAMP_STAT,
    ISCSI_FLASHNODE_TCP_NAGLE_DISABLE,
    ISCSI_FLASHNODE_TCP_WSF_DISABLE,
    ISCSI_FLASHNODE_TCP_TIMER_SCALE,
    ISCSI_FLASHNODE_TCP_TIMESTAMP_EN,
    ISCSI_FLASHNODE_IP_FRAG_DISABLE,
    ISCSI_FLASHNODE_MAX_RECV_DLENGTH,
    ISCSI_FLASHNODE_MAX_XMIT_DLENGTH,
    ISCSI_FLASHNODE_FIRST_BURST,
    ISCSI_FLASHNODE_DEF_TIME2WAIT,
    ISCSI_FLASHNODE_DEF_TIME2RETAIN,
    ISCSI_FLASHNODE_MAX_R2T,
    ISCSI_FLASHNODE_KEEPALIVE_TMO,
    ISCSI_FLASHNODE_ISID,
    ISCSI_FLASHNODE_TSID,
    ISCSI_FLASHNODE_PORT,
    ISCSI_FLASHNODE_MAX_BURST,
    ISCSI_FLASHNODE_DEF_TASKMGMT_TMO,
    ISCSI_FLASHNODE_IPADDR,
    ISCSI_FLASHNODE_ALIAS,
    ISCSI_FLASHNODE_REDIRECT_IPADDR,
    ISCSI_FLASHNODE_MAX_SEGMENT_SIZE,
    ISCSI_FLASHNODE_LOCAL_PORT,
    ISCSI_FLASHNODE_IPV4_TOS,
    ISCSI_FLASHNODE_IPV6_TC,
    ISCSI_FLASHNODE_IPV6_FLOW_LABEL,
    ISCSI_FLASHNODE_NAME,
    ISCSI_FLASHNODE_TPGT,
    ISCSI_FLASHNODE_LINK_LOCAL_IPV6,
    ISCSI_FLASHNODE_DISCOVERY_PARENT_IDX,
    ISCSI_FLASHNODE_DISCOVERY_PARENT_TYPE,
    ISCSI_FLASHNODE_TCP_XMIT_WSF,
    ISCSI_FLASHNODE_TCP_RECV_WSF,
    ISCSI_FLASHNODE_CHAP_IN_IDX,
    ISCSI_FLASHNODE_CHAP_OUT_IDX,
    ISCSI_FLASHNODE_USERNAME,
    ISCSI_FLASHNODE_USERNAME_IN,
    ISCSI_FLASHNODE_PASSWORD,
    ISCSI_FLASHNODE_PASSWORD_IN,
    ISCSI_FLASHNODE_STATSN,
    ISCSI_FLASHNODE_EXP_STATSN,
    ISCSI_FLASHNODE_IS_BOOT_TGT,

    ISCSI_FLASHNODE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_flashnode_param_info {
    pub /: *mut *mut uint32_t len; / Actual length of the param,
    pub /: *mut *mut uint16_t param; / iscsi param value,
    pub /: *mut *mut uint8_t value[]; / length sized value follows,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iscsi_discovery_parent_type {
    ISCSI_DISC_PARENT_UNKNOWN	= 0x1,
    ISCSI_DISC_PARENT_SENDTGT	= 0x2,
    ISCSI_DISC_PARENT_ISNS		= 0x3,
}

// iSCSI port Speed
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iscsi_port_speed {
    ISCSI_PORT_SPEED_UNKNOWN	= 0x1,
    ISCSI_PORT_SPEED_10MBPS		= 0x2,
    ISCSI_PORT_SPEED_100MBPS	= 0x4,
    ISCSI_PORT_SPEED_1GBPS		= 0x8,
    ISCSI_PORT_SPEED_10GBPS		= 0x10,
    ISCSI_PORT_SPEED_25GBPS         = 0x20,
    ISCSI_PORT_SPEED_40GBPS         = 0x40,
}

// iSCSI port state
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iscsi_port_state {
    ISCSI_PORT_STATE_DOWN		= 0x1,
    ISCSI_PORT_STATE_UP		= 0x2,
}

// iSCSI PING status/error code
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iscsi_ping_status_code {
    ISCSI_PING_SUCCESS			= 0,
    ISCSI_PING_FW_DISABLED			= 0x1,
    ISCSI_PING_IPADDR_INVALID		= 0x2,
    ISCSI_PING_LINKLOCAL_IPV6_ADDR_INVALID	= 0x3,
    ISCSI_PING_TIMEOUT			= 0x4,
    ISCSI_PING_INVALID_DEST_ADDR		= 0x5,
    ISCSI_PING_OVERSIZE_PACKET		= 0x6,
    ISCSI_PING_ICMP_ERROR			= 0x7,
    ISCSI_PING_MAX_REQ_EXCEEDED		= 0x8,
    ISCSI_PING_NO_ARP_RECEIVED		= 0x9,
}

//
// These flags presents iSCSI Data-Path capabilities.
//
pub const CAP_RECOVERY_L0: c_uint = 0x1;
pub const CAP_RECOVERY_L1: c_uint = 0x2;
pub const CAP_RECOVERY_L2: c_uint = 0x4;
pub const CAP_MULTI_R2T: c_uint = 0x8;
pub const CAP_HDRDGST: c_uint = 0x10;
pub const CAP_DATADGST: c_uint = 0x20;
pub const CAP_MULTI_CONN: c_uint = 0x40;
pub const CAP_TEXT_NEGO: c_uint = 0x80;
pub const CAP_MARKERS: c_uint = 0x100;
pub const CAP_FW_DB: c_uint = 0x200;
pub const CAP_SENDTARGETS_OFFLOAD: c_uint = 0x400	/* offload discovery process */;
pub const CAP_DATA_PATH_OFFLOAD: c_uint = 0x800	/* offload entire IO path */;
pub const CAP_DIGEST_OFFLOAD: c_uint = 0x1000	/* offload hdr and data digests */;
pub const CAP_PADDING_OFFLOAD: c_uint = 0x2000	/* offload padding insertion, removal,;
pub const CAP_LOGIN_OFFLOAD: c_uint = 0x4000  /* offload session login */;
//
// These flags describes reason of stop_conn() call
//
pub const STOP_CONN_TERM: c_uint = 0x1;
pub const STOP_CONN_SUSPEND: c_uint = 0x2;
pub const STOP_CONN_RECOVER: c_uint = 0x3;
pub const ISCSI_STATS_CUSTOM_MAX: c_int = 32;
pub const ISCSI_STATS_CUSTOM_DESC_MAX: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_stats_custom {
    pub desc: [c_char; ISCSI_STATS_CUSTOM_DESC_MAX],
    pub value: u64,
}

//
// struct iscsi_stats - iSCSI Statistics (iSCSI MIB)
//
// Note: this structure contains counters collected on per-connection basis.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_stats {
// octets
    pub txdata_octets: u64,
    pub rxdata_octets: u64,
// xmit pdus
    pub noptx_pdus: u32,
    pub scsicmd_pdus: u32,
    pub tmfcmd_pdus: u32,
    pub login_pdus: u32,
    pub text_pdus: u32,
    pub dataout_pdus: u32,
    pub logout_pdus: u32,
    pub snack_pdus: u32,
// recv pdus
    pub noprx_pdus: u32,
    pub scsirsp_pdus: u32,
    pub tmfrsp_pdus: u32,
    pub textrsp_pdus: u32,
    pub datain_pdus: u32,
    pub logoutrsp_pdus: u32,
    pub r2t_pdus: u32,
    pub async_pdus: u32,
    pub rjt_pdus: u32,
// errors
    pub digest_err: u32,
    pub timeout_err: u32,
//
// iSCSI Custom Statistics support, i.e. Transport could
// extend existing MIB statistics with its own specific statistics
// up to ISCSI_STATS_CUSTOM_MAX
//
    pub custom_length: u32,
// C attribute field omitted
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum chap_type_e {
    CHAP_TYPE_OUT,
    CHAP_TYPE_IN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iscsi_chap_param {
    ISCSI_CHAP_PARAM_INDEX,
    ISCSI_CHAP_PARAM_CHAP_TYPE,
    ISCSI_CHAP_PARAM_USERNAME,
    ISCSI_CHAP_PARAM_PASSWORD,
    ISCSI_CHAP_PARAM_PASSWORD_LEN
}

pub const ISCSI_CHAP_AUTH_NAME_MAX_LEN: c_int = 256;
pub const ISCSI_CHAP_AUTH_SECRET_MAX_LEN: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_chap_rec {
    pub chap_tbl_idx: u16,
    pub chap_type: chap_type_e,
    pub username: [c_char; ISCSI_CHAP_AUTH_NAME_MAX_LEN],
    pub password: [u8; ISCSI_CHAP_AUTH_SECRET_MAX_LEN],
    pub password_length: u8,
}

pub const ISCSI_HOST_STATS_CUSTOM_MAX: c_int = 32;
pub const ISCSI_HOST_STATS_CUSTOM_DESC_MAX: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_host_stats_custom {
    pub desc: [c_char; ISCSI_HOST_STATS_CUSTOM_DESC_MAX],
    pub value: u64,
}

// struct iscsi_offload_host_stats: Host statistics,
// Include statistics for MAC, IP, TCP & iSCSI.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_offload_host_stats {
// MAC
    pub mactx_frames: u64,
    pub mactx_bytes: u64,
    pub mactx_multicast_frames: u64,
    pub mactx_broadcast_frames: u64,
    pub mactx_pause_frames: u64,
    pub mactx_control_frames: u64,
    pub mactx_deferral: u64,
    pub mactx_excess_deferral: u64,
    pub mactx_late_collision: u64,
    pub mactx_abort: u64,
    pub mactx_single_collision: u64,
    pub mactx_multiple_collision: u64,
    pub mactx_collision: u64,
    pub mactx_frames_dropped: u64,
    pub mactx_jumbo_frames: u64,
    pub macrx_frames: u64,
    pub macrx_bytes: u64,
    pub macrx_unknown_control_frames: u64,
    pub macrx_pause_frames: u64,
    pub macrx_control_frames: u64,
    pub macrx_dribble: u64,
    pub macrx_frame_length_error: u64,
    pub macrx_jabber: u64,
    pub macrx_carrier_sense_error: u64,
    pub macrx_frame_discarded: u64,
    pub macrx_frames_dropped: u64,
    pub mac_crc_error: u64,
    pub mac_encoding_error: u64,
    pub macrx_length_error_large: u64,
    pub macrx_length_error_small: u64,
    pub macrx_multicast_frames: u64,
    pub macrx_broadcast_frames: u64,
// IP
    pub iptx_packets: u64,
    pub iptx_bytes: u64,
    pub iptx_fragments: u64,
    pub iprx_packets: u64,
    pub iprx_bytes: u64,
    pub iprx_fragments: u64,
    pub ip_datagram_reassembly: u64,
    pub ip_invalid_address_error: u64,
    pub ip_error_packets: u64,
    pub ip_fragrx_overlap: u64,
    pub ip_fragrx_outoforder: u64,
    pub ip_datagram_reassembly_timeout: u64,
    pub ipv6tx_packets: u64,
    pub ipv6tx_bytes: u64,
    pub ipv6tx_fragments: u64,
    pub ipv6rx_packets: u64,
    pub ipv6rx_bytes: u64,
    pub ipv6rx_fragments: u64,
    pub ipv6_datagram_reassembly: u64,
    pub ipv6_invalid_address_error: u64,
    pub ipv6_error_packets: u64,
    pub ipv6_fragrx_overlap: u64,
    pub ipv6_fragrx_outoforder: u64,
    pub ipv6_datagram_reassembly_timeout: u64,
// TCP
    pub tcptx_segments: u64,
    pub tcptx_bytes: u64,
    pub tcprx_segments: u64,
    pub tcprx_byte: u64,
    pub tcp_duplicate_ack_retx: u64,
    pub tcp_retx_timer_expired: u64,
    pub tcprx_duplicate_ack: u64,
    pub tcprx_pure_ackr: u64,
    pub tcptx_delayed_ack: u64,
    pub tcptx_pure_ack: u64,
    pub tcprx_segment_error: u64,
    pub tcprx_segment_outoforder: u64,
    pub tcprx_window_probe: u64,
    pub tcprx_window_update: u64,
    pub tcptx_window_probe_persist: u64,
// ECC
    pub ecc_error_correction: u64,
// iSCSI
    pub iscsi_pdu_tx: u64,
    pub iscsi_data_bytes_tx: u64,
    pub iscsi_pdu_rx: u64,
    pub iscsi_data_bytes_rx: u64,
    pub iscsi_io_completed: u64,
    pub iscsi_unexpected_io_rx: u64,
    pub iscsi_format_error: u64,
    pub iscsi_hdr_digest_error: u64,
    pub iscsi_data_digest_error: u64,
    pub iscsi_sequence_error: u64,
//
// iSCSI Custom Host Statistics support, i.e. Transport could
// extend existing host statistics with its own specific statistics
// up to ISCSI_HOST_STATS_CUSTOM_MAX
//
    pub custom_length: u32,
}
