//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/nfc/nci_core.h
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
// The NFC Controller Interface is the communication protocol between an
// NFC Controller (NFCC) and a Device Host (DH).
//
// Copyright (C) 2011 Texas Instruments, Inc.
// Copyright (C) 2013 Intel Corporation. All rights reserved.
// Copyright (C) 2014 Marvell International Ltd.
//
// Written by Ilan Elias <ilane@ti.com>
//
// Acknowledgements:
// This file is based on hci_core.h, which was written
// by Maxim Krasnyansky.
//

// NCI device flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nci_flag {
    NCI_INIT,
    NCI_UP,
    NCI_DATA_EXCHANGE,
    NCI_DATA_EXCHANGE_TO,
    NCI_UNREG,
}

// NCI device states
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nci_state {
    NCI_IDLE,
    NCI_DISCOVERY,
    NCI_W4_ALL_DISCOVERIES,
    NCI_W4_HOST_SELECT,
    NCI_POLL_ACTIVE,
    NCI_LISTEN_ACTIVE,
    NCI_LISTEN_SLEEP,
}

// NCI timeouts
pub const NCI_RESET_TIMEOUT: c_int = 5000;
pub const NCI_INIT_TIMEOUT: c_int = 5000;
pub const NCI_SET_CONFIG_TIMEOUT: c_int = 5000;
pub const NCI_RF_DISC_TIMEOUT: c_int = 5000;
pub const NCI_RF_DISC_SELECT_TIMEOUT: c_int = 5000;
pub const NCI_RF_DEACTIVATE_TIMEOUT: c_int = 30000;
pub const NCI_CMD_TIMEOUT: c_int = 5000;
pub const NCI_DATA_TIMEOUT: c_int = 3000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_driver_ops {
    pub opcode: __u16,
    pub skb): *mut *mut *mut int (rsp)(struct nci_dev dev, struct sk_buff,
    pub skb): *mut *mut *mut int (ntf)(struct nci_dev dev, struct sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_ops {
    pub ndev): *mut *mut int (init)(struct nci_dev,
    pub ndev): *mut *mut int (open)(struct nci_dev,
    pub ndev): *mut *mut int (close)(struct nci_dev,
    pub skb): *mut *mut *mut int (send)(struct nci_dev ndev, struct sk_buff,
    pub ndev): *mut *mut int (setup)(struct nci_dev,
    pub ndev): *mut *mut int (post_setup)(struct nci_dev,
    pub firmware_name): *const *const *const int (fw_download)(struct nci_dev ndev, char,
    pub rf_protocol): *mut *mut *mut __u32 (get_rfprotocol)(struct nci_dev ndev, __u8,
    pub ndev): *mut *mut int (discover_se)(struct nci_dev,
    pub se_idx): *mut *mut *mut int (disable_se)(struct nci_dev ndev, u32,
    pub se_idx): *mut *mut *mut int (enable_se)(struct nci_dev ndev, u32,
    pub cb_context): *mut se_io_cb_t cb, void,
    pub ndev): *mut *mut int (hci_load_session)(struct nci_dev,
    pub skb): *mut sk_buff,
    pub skb): *mut sk_buff,
    pub prop_ops: *const nci_driver_ops,
    pub n_prop_ops: usize,
    pub core_ops: *const nci_driver_ops,
    pub n_core_ops: usize,
}

pub const NCI_MAX_SUPPORTED_RF_INTERFACES: c_int = 4;
pub const NCI_MAX_DISCOVERED_TARGETS: c_int = 10;
pub const NCI_MAX_NUM_NFCEE: c_int = 255;
pub const NCI_MAX_CONN_ID: c_int = 7;
pub const NCI_MAX_PROPRIETARY_CMD: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_conn_info {
    pub list: list_head,
// NCI specification 4.4.2 Connection Creation
// The combination of destination type and destination specific
// parameters shall uniquely identify a single destination for the
// Logical Connection
//
    pub dest_params: *mut dest_spec_params,
    pub dest_type: __u8,
    pub conn_id: __u8,
    pub max_pkt_payload_len: __u8,
    pub credits_cnt: core::sync::atomic::AtomicI32,
    pub initial_num_credits: __u8,
    pub data_exchange_cb: data_exchange_cb_t,
    pub data_exchange_cb_context: *mut c_void,
    pub rx_skb: *mut sk_buff,
}

pub const NCI_INVALID_CONN_ID: c_uint = 0x80;
pub const NCI_HCI_ANY_OPEN_PIPE: c_uint = 0x03;
// Gates
pub const NCI_HCI_ADMIN_GATE: c_uint = 0x00;
pub const NCI_HCI_LOOPBACK_GATE: c_uint = 0x04;
pub const NCI_HCI_IDENTITY_MGMT_GATE: c_uint = 0x05;
pub const NCI_HCI_LINK_MGMT_GATE: c_uint = 0x06;
// Pipes
pub const NCI_HCI_LINK_MGMT_PIPE: c_uint = 0x00;
pub const NCI_HCI_ADMIN_PIPE: c_uint = 0x01;
// Generic responses
pub const NCI_HCI_ANY_OK: c_uint = 0x00;
pub const NCI_HCI_ANY_E_NOT_CONNECTED: c_uint = 0x01;
pub const NCI_HCI_ANY_E_CMD_PAR_UNKNOWN: c_uint = 0x02;
pub const NCI_HCI_ANY_E_NOK: c_uint = 0x03;
pub const NCI_HCI_ANY_E_PIPES_FULL: c_uint = 0x04;
pub const NCI_HCI_ANY_E_REG_PAR_UNKNOWN: c_uint = 0x05;
pub const NCI_HCI_ANY_E_PIPE_NOT_OPENED: c_uint = 0x06;
pub const NCI_HCI_ANY_E_CMD_NOT_SUPPORTED: c_uint = 0x07;
pub const NCI_HCI_ANY_E_INHIBITED: c_uint = 0x08;
pub const NCI_HCI_ANY_E_TIMEOUT: c_uint = 0x09;
pub const NCI_HCI_ANY_E_REG_ACCESS_DENIED: c_uint = 0x0a;
pub const NCI_HCI_ANY_E_PIPE_ACCESS_DENIED: c_uint = 0x0b;
pub const NCI_HCI_DO_NOT_OPEN_PIPE: c_uint = 0x81;
pub const NCI_HCI_INVALID_PIPE: c_uint = 0x80;
pub const NCI_HCI_INVALID_GATE: c_uint = 0xFF;
pub const NCI_HCI_INVALID_HOST: c_uint = 0x80;
pub const NCI_HCI_MAX_CUSTOM_GATES: c_int = 50;
//
// According to specification 102 622 chapter 4.4 Pipes,
// the pipe identifier is 7 bits long.
//
pub const NCI_HCI_MAX_PIPES: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_hci_gate {
    pub gate: u8,
    pub pipe: u8,
    pub dest_host: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_hci_pipe {
    pub gate: u8,
    pub host: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_hci_init_data {
    pub gate_count: u8,
    pub gates: [nci_hci_gate; NCI_HCI_MAX_CUSTOM_GATES],
    pub session_id: [c_char; 9],
}

pub const NCI_HCI_MAX_GATES: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_hci_dev {
    pub nfcee_id: u8,
    pub ndev: *mut nci_dev,
    pub conn_info: *mut nci_conn_info,
    pub init_data: nci_hci_init_data,
    pub pipes: [nci_hci_pipe; NCI_HCI_MAX_PIPES],
    pub gate2pipe: [u8; NCI_HCI_MAX_GATES],
    pub expected_pipes: c_int,
    pub count_pipes: c_int,
    pub rx_hcp_frags: sk_buff_head,
    pub msg_rx_work: work_struct,
    pub msg_rx_queue: sk_buff_head,
}

// NCI Core structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_dev {
    pub nfc_dev: *mut nfc_dev,
    pub ops: *const nci_ops,
    pub hci_dev: *mut nci_hci_dev,
    pub tx_headroom: c_int,
    pub tx_tailroom: c_int,
    pub state: core::sync::atomic::AtomicI32,
    pub flags: c_ulong,
    pub cmd_cnt: core::sync::atomic::AtomicI32,
    pub cur_conn_id: __u8,
    pub conn_info_list: list_head,
    pub rf_conn_info: *mut nci_conn_info,
    pub cmd_timer: timer_list,
    pub data_timer: timer_list,
    pub cmd_wq: *mut workqueue_struct,
    pub cmd_work: work_struct,
    pub rx_wq: *mut workqueue_struct,
    pub rx_work: work_struct,
    pub tx_wq: *mut workqueue_struct,
    pub tx_work: work_struct,
    pub cmd_q: sk_buff_head,
    pub rx_q: sk_buff_head,
    pub tx_q: sk_buff_head,
    pub req_lock: mutex,
    pub req_completion: completion,
    pub req_status: __u32,
    pub req_result: __u32,
    pub driver_data: *mut c_void,
    pub poll_prots: __u32,
    pub target_active_prot: __u32,
    pub targets: [nfc_target; NCI_MAX_DISCOVERED_TARGETS],
    pub n_targets: c_int,
// received during NCI_OP_CORE_RESET_RSP
    pub nci_ver: __u8,
// received during NCI_OP_CORE_INIT_RSP
    pub nfcc_features: __u32,
    pub num_supported_rf_interfaces: __u8,
    pub max_logical_connections: __u8,
    pub max_routing_table_size: __u16,
    pub max_ctrl_pkt_payload_len: __u8,
    pub max_size_for_large_params: __u16,
    pub manufact_id: __u8,
    pub manufact_specific_info: __u32,
// Save RF Discovery ID or NFCEE ID under conn_create
    pub cur_params: dest_spec_params,
// Save destination type under conn_create
    pub cur_dest_type: __u8,
// stored during nci_data_exchange
    pub rx_data_reassembly: *mut sk_buff,
// stored during intf_activated_ntf
    pub remote_gb: [__u8; NFC_MAX_GT_LEN],
    pub remote_gb_len: __u8,
// stored during intf_activated_ntf
    pub target_ats: [__u8; NFC_ATS_MAXSIZE],
    pub target_ats_len: __u8,
}

// ----- NCI Devices -----
extern "C" {
    pub fn nci_free_device(ndev: *mut nci_dev);
}
extern "C" {
    pub fn nci_register_device(ndev: *mut nci_dev) -> c_int;
}
extern "C" {
    pub fn nci_unregister_device(ndev: *mut nci_dev);
}
extern "C" {
    pub fn nci_core_reset(ndev: *mut nci_dev) -> c_int;
}
extern "C" {
    pub fn nci_core_init(ndev: *mut nci_dev) -> c_int;
}
extern "C" {
    pub fn nci_recv_frame(ndev: *mut nci_dev, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn nci_send_frame(ndev: *mut nci_dev, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn nci_set_config(ndev: *mut nci_dev, id: __u8, len: usize, val: *const __u8) -> c_int;
}
extern "C" {
    pub fn nci_nfcee_discover(ndev: *mut nci_dev, action: u8) -> c_int;
}
extern "C" {
    pub fn nci_nfcee_mode_set(ndev: *mut nci_dev, nfcee_id: u8, nfcee_mode: u8) -> c_int;
}
extern "C" {
    pub fn nci_core_conn_close(ndev: *mut nci_dev, conn_id: u8) -> c_int;
}
extern "C" {
    pub fn nci_hci_deallocate(ndev: *mut nci_dev);
}
extern "C" {
    pub fn nci_hci_open_pipe(ndev: *mut nci_dev, pipe: u8) -> c_int;
}
extern "C" {
    pub fn nci_hci_clear_all_pipes(ndev: *mut nci_dev) -> c_int;
}
extern "C" {
    pub fn nci_hci_dev_session_init(ndev: *mut nci_dev) -> c_int;
}
extern "C" {
    pub fn nfc_set_vendor_cmds(_arg: ndev->nfc_dev, _arg: cmds, _arg: n_cmds) -> return;
}
extern "C" {
    pub fn nci_rsp_packet(ndev: *mut nci_dev, skb: *mut sk_buff);
}
extern "C" {
    pub fn nci_ntf_packet(ndev: *mut nci_dev, skb: *mut sk_buff);
}
extern "C" {
    pub fn nci_rx_data_packet(ndev: *mut nci_dev, skb: *mut sk_buff);
}
extern "C" {
    pub fn nci_send_cmd(ndev: *mut nci_dev, opcode: __u16, plen: __u8, payload: *const c_void) -> c_int;
}
extern "C" {
    pub fn nci_send_data(ndev: *mut nci_dev, conn_id: __u8, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn nci_conn_max_data_pkt_payload_size(ndev: *mut nci_dev, conn_id: __u8) -> c_int;
}
extern "C" {
    pub fn nci_hci_data_received_cb(context: *mut c_void, skb: *mut sk_buff, err: c_int);
}
extern "C" {
    pub fn nci_clear_target_list(ndev: *mut nci_dev);
}
// ----- NCI requests -----
pub const NCI_REQ_DONE: c_int = 0;
pub const NCI_REQ_PEND: c_int = 1;
pub const NCI_REQ_CANCELED: c_int = 2;
extern "C" {
    pub fn nci_req_complete(ndev: *mut nci_dev, result: c_int);
}
// ----- NCI status code -----
extern "C" {
    pub fn nci_to_errno(code: __u8) -> c_int;
}
// ----- NCI over SPI acknowledge modes -----
pub const NCI_SPI_CRC_DISABLED: c_uint = 0x00;
pub const NCI_SPI_CRC_ENABLED: c_uint = 0x01;
// ----- NCI SPI structures -----
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_spi {
    pub ndev: *mut nci_dev,
    pub spi: *mut spi_device,
    pub between: *mut *mut unsigned int xfer_udelay; / microseconds delay,
    pub /*: *mut unsigned int xfer_speed_hz;,
// SPI clock frequency
// 0 => default clock
//
    pub acknowledge_mode: u8,
    pub req_completion: completion,
    pub req_result: u8,
}

// ----- NCI SPI -----
// ----- NCI UART ----
// Ioctl

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nci_uart_driver {
    NCI_UART_DRIVER_MARVELL = 0,
    NCI_UART_DRIVER_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_uart_ops {
    pub nci_uart): *mut *mut int (open)(struct nci_uart,
    pub nci_uart): *mut *mut void (close)(struct nci_uart,
    pub skb): *mut *mut *mut int (recv)(struct nci_uart nci_uart, struct sk_buff,
    pub skb): *mut *mut *mut int (send)(struct nci_uart nci_uart, struct sk_buff,
    pub nci_uart): *mut *mut void (tx_start)(struct nci_uart,
    pub nci_uart): *mut *mut void (tx_done)(struct nci_uart,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_uart {
    pub owner: *mut module,
    pub ops: nci_uart_ops,
    pub name: *const c_char,
    pub driver: nci_uart_driver,
// Dynamic data
    pub ndev: *mut nci_dev,
    pub rx_lock: spinlock_t,
    pub write_work: work_struct,
    pub tty: *mut tty_struct,
    pub tx_state: c_ulong,
    pub tx_q: sk_buff_head,
    pub tx_skb: *mut sk_buff,
    pub rx_skb: *mut sk_buff,
    pub rx_packet_len: c_int,
    pub drv_data: *mut c_void,
}

extern "C" {
    pub fn nci_uart_register(nu: *mut nci_uart) -> c_int;
}
extern "C" {
    pub fn nci_uart_unregister(nu: *mut nci_uart);
}
extern "C" {
    pub fn nci_uart_set_config(nu: *mut nci_uart, baudrate: c_int, flow_ctrl: c_int);
}
