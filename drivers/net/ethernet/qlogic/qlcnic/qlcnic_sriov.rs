//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qlogic/qlcnic/qlcnic_sriov.h
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
// QLogic qlcnic NIC Driver
// Copyright (c) 2009-2013 QLogic Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_bc_payload {
    pub payload: [u64; 126],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_bc_hdr {

    pub version: u8,
    pub msg_type:4: u8,
    pub rsvd1:3: u8,
    pub op_type:1: u8,
    pub num_cmds: u8,
    pub num_frags: u8,
    pub frag_num: u8,
    pub cmd_op: u8,
    pub seq_id: u16,
    pub rsvd3: u64,

    pub num_frags: u8,
    pub num_cmds: u8,
    pub op_type:1: u8,
    pub rsvd1:3: u8,
    pub msg_type:4: u8,
    pub version: u8,
    pub seq_id: u16,
    pub cmd_op: u8,
    pub frag_num: u8,
    pub rsvd3: u64,

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlcnic_bc_commands {
    QLCNIC_BC_CMD_CHANNEL_INIT = 0x0,
    QLCNIC_BC_CMD_CHANNEL_TERM = 0x1,
    QLCNIC_BC_CMD_GET_ACL = 0x2,
    QLCNIC_BC_CMD_CFG_GUEST_VLAN = 0x3,
}

pub const QLCNIC_83XX_SRIOV_VF_MAX_MAC: c_int = 2;
pub const QLC_BC_CMD: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_trans_list {
// Lock for manipulating list
    pub lock: spinlock_t,
    pub wait_list: list_head,
    pub count: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlcnic_trans_state {
    QLC_INIT = 0,
    QLC_WAIT_FOR_CHANNEL_FREE,
    QLC_WAIT_FOR_RESP,
    QLC_ABORT,
    QLC_END,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_bc_trans {
    pub func_id: u8,
    pub active: u8,
    pub curr_rsp_frag: u8,
    pub curr_req_frag: u8,
    pub cmd_id: u16,
    pub req_pay_size: u16,
    pub rsp_pay_size: u16,
    pub trans_id: u32,
    pub trans_state: qlcnic_trans_state,
    pub list: list_head,
    pub req_hdr: *mut qlcnic_bc_hdr,
    pub rsp_hdr: *mut qlcnic_bc_hdr,
    pub req_pay: *mut qlcnic_bc_payload,
    pub rsp_pay: *mut qlcnic_bc_payload,
    pub resp_cmpl: completion,
    pub vf: *mut qlcnic_vf_info,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlcnic_vf_state {
    QLC_BC_VF_SEND = 0,
    QLC_BC_VF_RECV,
    QLC_BC_VF_CHANNEL,
    QLC_BC_VF_STATE,
    QLC_BC_VF_FLR,
    QLC_BC_VF_SOFT_FLR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlcnic_vlan_mode {
    QLC_NO_VLAN_MODE = 0,
    QLC_PVID_MODE,
    QLC_GUEST_VLAN_MODE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_resources {
    pub num_tx_mac_filters: u16,
    pub num_rx_ucast_mac_filters: u16,
    pub num_rx_mcast_mac_filters: u16,
    pub num_txvlan_keys: u16,
    pub num_rx_queues: u16,
    pub num_tx_queues: u16,
    pub num_rx_buf_rings: u16,
    pub num_rx_status_rings: u16,
    pub num_destip: u16,
    pub num_lro_flows_supported: u32,
    pub max_local_ipv6_addrs: u16,
    pub max_remote_ipv6_addrs: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_vport {
    pub handle: u16,
    pub max_tx_bw: u16,
    pub min_tx_bw: u16,
    pub pvid: u16,
    pub vlan_mode: u8,
    pub qos: u8,
    pub spoofchk: bool,
    pub mac: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_vf_info {
    pub pci_func: u8,
    pub rx_ctx_id: u16,
    pub tx_ctx_id: u16,
    pub sriov_vlans: *mut u16,
    pub num_vlan: c_int,
    pub state: c_ulong,
    pub ch_free_cmpl: completion,
    pub trans_work: work_struct,
    pub flr_work: work_struct,
// It synchronizes commands sent from VF
    pub send_cmd_lock: mutex,
    pub send_cmd: *mut qlcnic_bc_trans,
    pub flr_trans: *mut qlcnic_bc_trans,
    pub rcv_act: qlcnic_trans_list,
    pub rcv_pend: qlcnic_trans_list,
    pub adapter: *mut qlcnic_adapter,
    pub vp: *mut qlcnic_vport,
    pub /: *mut *mut spinlock_t vlan_list_lock; / Lock for VLAN list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_async_cmd {
    pub list: list_head,
    pub cmd: *mut qlcnic_cmd_args,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_back_channel {
    pub trans_counter: u16,
    pub bc_trans_wq: *mut workqueue_struct,
    pub bc_async_wq: *mut workqueue_struct,
    pub bc_flr_wq: *mut workqueue_struct,
    pub adapter: *mut qlcnic_adapter,
    pub async_cmd_list: list_head,
    pub vf_async_work: work_struct,
    pub /: *mut *mut spinlock_t queue_lock; / async_cmd_list queue lock,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_sriov {
    pub vp_handle: u16,
    pub num_vfs: u8,
    pub any_vlan: u8,
    pub vlan_mode: u8,
    pub num_allowed_vlans: u16,
    pub allowed_vlans: *mut u16,
    pub vlan: u16,
    pub ff_max: qlcnic_resources,
    pub bc: qlcnic_back_channel,
    pub vf_info: *mut qlcnic_vf_info,
}

extern "C" {
    pub fn qlcnic_sriov_init(: *mut qlcnic_adapter, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn qlcnic_sriov_cleanup(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn __qlcnic_sriov_cleanup(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_sriov_vf_register_map(: *mut qlcnic_hardware_context);
}
extern "C" {
    pub fn qlcnic_sriov_vf_init(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_sriov_vf_set_ops(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_sriov_func_to_index(: *mut qlcnic_adapter, _arg: u8) -> c_int;
}
extern "C" {
    pub fn qlcnic_sriov_handle_bc_event(: *mut qlcnic_adapter, _arg: u32);
}
extern "C" {
    pub fn qlcnic_sriov_cfg_bc_intr(: *mut qlcnic_adapter, _arg: u8) -> c_int;
}
extern "C" {
    pub fn qlcnic_sriov_cleanup_async_list(: *mut qlcnic_back_channel);
}
extern "C" {
    pub fn qlcnic_sriov_cleanup_list(: *mut qlcnic_trans_list);
}
extern "C" {
    pub fn qlcnic_sriov_cfg_vf_guest_vlan(: *mut qlcnic_adapter, _arg: u16, _arg: u8) -> c_int;
}
extern "C" {
    pub fn qlcnic_sriov_free_vlans(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_sriov_alloc_vlans(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_sriov_check_any_vlan(: *mut qlcnic_vf_info) -> bool;
}

extern "C" {
    pub fn qlcnic_sriov_pf_disable(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_sriov_pf_cleanup(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_pci_sriov_configure(: *mut pci_dev, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn qlcnic_pf_set_interface_id_create_rx_ctx(: *mut qlcnic_adapter, : *mut u32);
}
extern "C" {
    pub fn qlcnic_pf_set_interface_id_create_tx_ctx(: *mut qlcnic_adapter, : *mut u32);
}
extern "C" {
    pub fn qlcnic_pf_set_interface_id_del_rx_ctx(: *mut qlcnic_adapter, : *mut u32);
}
extern "C" {
    pub fn qlcnic_pf_set_interface_id_del_tx_ctx(: *mut qlcnic_adapter, : *mut u32);
}
extern "C" {
    pub fn qlcnic_pf_set_interface_id_promisc(: *mut qlcnic_adapter, : *mut u32);
}
extern "C" {
    pub fn qlcnic_pf_set_interface_id_ipaddr(: *mut qlcnic_adapter, : *mut u32);
}
extern "C" {
    pub fn qlcnic_pf_set_interface_id_macaddr(: *mut qlcnic_adapter, : *mut u32);
}
extern "C" {
    pub fn qlcnic_sriov_pf_handle_flr(: *mut qlcnic_sriov, : *mut qlcnic_vf_info);
}
extern "C" {
    pub fn qlcnic_sriov_pf_reset(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_sriov_pf_reinit(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_sriov_set_vf_mac(: *mut net_device, _arg: c_int, : *mut u8) -> c_int;
}
extern "C" {
    pub fn qlcnic_sriov_set_vf_tx_rate(: *mut net_device, _arg: c_int, _arg: c_int, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn qlcnic_sriov_set_vf_vlan(: *mut net_device, _arg: c_int, _arg: u16, _arg: u8, _arg: __be16) -> c_int;
}
extern "C" {
    pub fn qlcnic_sriov_set_vf_spoofchk(: *mut net_device, _arg: c_int, _arg: bool) -> c_int;
}

