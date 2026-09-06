//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/cxgbi/libcxgbi.h
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
// libcxgbi.h: Chelsio common library for T3/T4 iSCSI driver.
//
// Copyright (c) 2010-2015 Chelsio Communications, Inc.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//
// Written by: Karen Xie (kxie@chelsio.com)
// Written by: Rakesh Ranjan (rranjan@chelsio.com)
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxgbi_dbg_flag {
    CXGBI_DBG_ISCSI,
    CXGBI_DBG_DDP,
    CXGBI_DBG_TOE,
    CXGBI_DBG_SOCK,

    CXGBI_DBG_PDU_TX,
    CXGBI_DBG_PDU_RX,
    CXGBI_DBG_DEV,
}

// max. connections per adapter
pub const CXGBI_MAX_CONN: c_int = 16384;
// always allocate rooms for AHS

//
// align pdu size to multiple of 512 for better performance
//

pub const ULP2_MODE_ISCSI: c_int = 2;
pub const ULP2_MAX_PKT_SIZE: c_int = 16224;

pub const CXGBI_ULP2_MAX_ISO_PAYLOAD: c_int = 65535;

//
// For iscsi connections HW may inserts digest bytes into the pdu. Those digest
// bytes are not sent by the host but are part of the TCP payload and therefore
// consume TCP sequence space.
//

//
// sge_opaque_hdr -
// Opaque version of structure the SGE stores at skb->head of TX_DATA packets
// and for which we must reserve space.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge_opaque_hdr {
    pub dev: *mut c_void,
    pub 1]: dma_addr_t addr[MAX_SKB_FRAGS +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgbi_sock {
    pub cdev: *mut cxgbi_device,
    pub tid: c_int,
    pub atid: c_int,
    pub flags: c_ulong,
    pub mtu: c_uint,
    pub rss_qid: c_ushort,
    pub txq_idx: c_ushort,
    pub advmss: c_ushort,
    pub tx_chan: c_uint,
    pub rx_chan: c_uint,
    pub mss_idx: c_uint,
    pub smac_idx: c_uint,
    pub port_id: c_uchar,
    pub wr_max_cred: c_int,
    pub wr_cred: c_int,
    pub wr_una_cred: c_int,

    pub dcb_priority: u8,

    pub hcrc_len: c_uchar,
    pub dcrc_len: c_uchar,
    pub l2t: *mut c_void,
    pub wr_pending_head: *mut sk_buff,
    pub wr_pending_tail: *mut sk_buff,
    pub cpl_close: *mut sk_buff,
    pub cpl_abort_req: *mut sk_buff,
    pub cpl_abort_rpl: *mut sk_buff,
    pub skb_ulp_lhdr: *mut sk_buff,
    pub lock: spinlock_t,
    pub refcnt: kref,
    pub state: c_uint,
    pub csk_family: c_uint,
    pub saddr: sockaddr_in,
    pub saddr6: sockaddr_in6,
}

//
// connection states
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxgbi_sock_states {
    CTP_CLOSED,
    CTP_CONNECTING,
    CTP_ACTIVE_OPEN,
    CTP_ESTABLISHED,
    CTP_ACTIVE_CLOSE,
    CTP_PASSIVE_CLOSE,
    CTP_CLOSE_WAIT_1,
    CTP_CLOSE_WAIT_2,
    CTP_ABORTING,
}

//
// Connection flags -- many to track some close related events.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxgbi_sock_flags {
    CTPF_ABORT_RPL_RCVD,	/*received one ABORT_RPL_RSS message */
    CTPF_ABORT_REQ_RCVD,	/*received one ABORT_REQ_RSS message */
    CTPF_ABORT_RPL_PENDING,	/* expecting an abort reply */
    CTPF_TX_DATA_SENT,	/* already sent a TX_DATA WR */
    CTPF_ACTIVE_CLOSE_NEEDED,/* need to be closed */
    CTPF_HAS_ATID,		/* reserved atid */
    CTPF_HAS_TID,		/* reserved hw tid */
    CTPF_OFFLOAD_DOWN,	/* offload function off */
    CTPF_LOGOUT_RSP_RCVD,   /* received logout response */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgbi_skb_rx_cb {
    pub ddigest: __u32,
    pub pdulen: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgbi_skb_tx_cb {
    pub handle: *mut c_void,
    pub arp_err_handler: *mut c_void,
    pub wr_next: *mut sk_buff,
    pub iscsi_hdr_len: u16,
    pub ulp_mode: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxgbi_skcb_flags {
    SKCBF_TX_NEED_HDR,	/* packet needs a header */
    SKCBF_TX_MEM_WRITE,     /* memory write */
    SKCBF_TX_FLAG_COMPL,    /* wr completion flag */
    SKCBF_RX_COALESCED,	/* received whole pdu */
    SKCBF_RX_HDR,		/* received pdu header */
    SKCBF_RX_DATA,		/* received pdu payload */
    SKCBF_RX_STATUS,	/* received ddp status */
    SKCBF_RX_ISCSI_COMPL,   /* received iscsi completion */
    SKCBF_RX_DATA_DDPD,	/* pdu payload ddp'd */
    SKCBF_RX_HCRC_ERR,	/* header digest error */
    SKCBF_RX_DCRC_ERR,	/* data digest error */
    SKCBF_RX_PAD_ERR,	/* padding byte error */
    SKCBF_TX_ISO,		/* iso cpl in tx skb */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgbi_skb_cb {
    pub rx: cxgbi_skb_rx_cb,
    pub tx: cxgbi_skb_tx_cb,
}

extern "C" {
    pub fn test_bit(_arg: flag, _arg: &(cxgbi_skcb_flags(skb))) -> return;
}
extern "C" {
    pub fn test_bit(_arg: flag, _arg: &csk->flags) -> return;
}

//
// The number of WRs needed for an skb depends on the number of fragments
// in the skb and whether it has any payload in its main body.  This maps the
// length of the gather list represented by an skb into the # of necessary WRs.
// The extra two fragments are for iscsi bhs and payload padding.
//

//
// We want to take an extra reference since both us and the driver
// need to free the packet before it's really freed.
//
extern "C" {
    pub fn cxgbi_sock_check_wr_invariants(: *const cxgbi_sock);
}
extern "C" {
    pub fn cxgbi_sock_purge_wr_queue(: *mut cxgbi_sock);
}
extern "C" {
    pub fn cxgbi_sock_skb_entail(: *mut cxgbi_sock, : *mut sk_buff);
}
extern "C" {
    pub fn cxgbi_sock_fail_act_open(: *mut cxgbi_sock, _arg: c_int);
}
extern "C" {
    pub fn cxgbi_sock_act_open_req_arp_failure(: *mut c_void, : *mut sk_buff);
}
extern "C" {
    pub fn cxgbi_sock_closed(: *mut cxgbi_sock);
}
extern "C" {
    pub fn cxgbi_sock_established(: *mut cxgbi_sock, int: unsigned, int: unsigned);
}
extern "C" {
    pub fn cxgbi_sock_rcv_abort_rpl(: *mut cxgbi_sock);
}
extern "C" {
    pub fn cxgbi_sock_rcv_peer_close(: *mut cxgbi_sock);
}
extern "C" {
    pub fn cxgbi_sock_rcv_close_conn_rpl(: *mut cxgbi_sock, _arg: u32);
}
extern "C" {
    pub fn cxgbi_sock_select_mss(: *mut cxgbi_sock, int: unsigned) -> c_uint;
}
extern "C" {
    pub fn cxgbi_sock_free_cpl_skbs(: *mut cxgbi_sock);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgbi_hba {
    pub ndev: *mut net_device,
    pub /: *mut *mut *mut net_device vdev; / vlan dev,
    pub shost: *mut Scsi_Host,
    pub cdev: *mut cxgbi_device,
    pub ipv4addr: __be32,
    pub port_id: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgbi_ports_map {
    pub max_connect: c_uint,
    pub used: c_uint,
    pub sport_base: c_ushort,
    pub lock: spinlock_t,
    pub next: c_uint,
    pub port_csk: *mut cxgbi_sock,
}

pub const CXGBI_FLAG_DEV_T3: c_uint = 0x1;
pub const CXGBI_FLAG_DEV_T4: c_uint = 0x2;
pub const CXGBI_FLAG_ADAPTER_RESET: c_uint = 0x4;
pub const CXGBI_FLAG_IPV4_SET: c_uint = 0x10;
pub const CXGBI_FLAG_USE_PPOD_OFLDQ: c_uint = 0x40;
pub const CXGBI_FLAG_DDP_OFF: c_uint = 0x100;
pub const CXGBI_FLAG_DEV_ISO_OFF: c_uint = 0x400;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgbi_device {
    pub list_head: list_head,
    pub rcu_node: list_head,
    pub flags: c_uint,
    pub ports: *mut net_device,
    pub lldev: *mut c_void,
    pub hbas: *mut cxgbi_hba,
    pub mtus: *const c_ushort,
    pub nmtus: c_uchar,
    pub nports: c_uchar,
    pub pdev: *mut pci_dev,
    pub itp: *mut iscsi_transport,
    pub owner: *mut module,
    pub pfvf: c_uint,
    pub rx_credit_thres: c_uint,
    pub skb_tx_rsvd: c_uint,
    pub skb_iso_txhdr: u32,
    pub /: *mut *mut unsigned int skb_rx_extra; / for msg coalesced mode,
    pub tx_max_size: c_uint,
    pub rx_max_size: c_uint,
    pub rxq_idx_cntr: c_uint,
    pub pmap: cxgbi_ports_map,
    pub ): *mut *mut *mut cxgbi_ppm (cdev2ppm)(cxgbi_device,
    pub ): *mut cxgbi_task_tag_info,
    pub ): *mut cxgbi_task_tag_info,
    pub int): unsigned int, int,,
    pub int): unsigned int,,
    pub ): *mut *mut void (csk_release_offload_resources)(struct cxgbi_sock,
    pub u32): *mut *mut *mut u32 (csk_send_rx_credits)(struct cxgbi_sock ,,
    pub int): *mut *mut *mut int (csk_push_tx_frames)(struct cxgbi_sock ,,
    pub ): *mut *mut void (csk_send_abort_req)(struct cxgbi_sock,
    pub ): *mut *mut void (csk_send_close_req)(struct cxgbi_sock,
    pub ): *mut *mut int (csk_alloc_cpls)(struct cxgbi_sock,
    pub ): *mut *mut int (csk_init_act_open)(struct cxgbi_sock,
    pub dd_data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgbi_conn {
    pub cep: *mut cxgbi_endpoint,
    pub iconn: *mut iscsi_conn,
    pub chba: *mut cxgbi_hba,
    pub task_idx_bits: u32,
    pub ddp_full: c_uint,
    pub ddp_tag_full: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgbi_endpoint {
    pub cconn: *mut cxgbi_conn,
    pub chba: *mut cxgbi_hba,
    pub csk: *mut cxgbi_sock,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgbi_task_data {
pub const CXGBI_TASK_SGL_CHECKED: c_uint = 0x1;
pub const CXGBI_TASK_SGL_COPY: c_uint = 0x2;
    pub flags: u8,
    pub nr_frags: c_ushort,
    pub frags: [page_frag; MAX_SKB_FRAGS],
    pub skb: *mut sk_buff,
    pub dlen: c_uint,
    pub offset: c_uint,
    pub count: c_uint,
    pub sgoffset: c_uint,
    pub total_count: u32,
    pub total_offset: u32,
    pub max_xmit_dlength: u32,
    pub ttinfo: cxgbi_task_tag_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgbi_iso_info {
pub const CXGBI_ISO_INFO_FSLICE: c_uint = 0x1;
pub const CXGBI_ISO_INFO_LSLICE: c_uint = 0x2;
pub const CXGBI_ISO_INFO_IMM_ENABLE: c_uint = 0x4;
    pub flags: u8,
    pub op: u8,
    pub ahs: u8,
    pub num_pdu: u8,
    pub mpdu: u32,
    pub burst_size: u32,
    pub len: u32,
    pub segment_offset: u32,
    pub datasn_offset: u32,
    pub buffer_offset: u32,
}

extern "C" {
    pub fn cxgbi_device_unregister(: *mut cxgbi_device);
}
extern "C" {
    pub fn cxgbi_device_unregister_all(flag: c_uint);
}
extern "C" {
    pub fn cxgbi_hbas_remove(: *mut cxgbi_device);
}
extern "C" {
    pub fn cxgbi_device_portmap_cleanup(cdev: *mut cxgbi_device);
}
extern "C" {
    pub fn cxgbi_conn_tx_open(: *mut cxgbi_sock);
}
extern "C" {
    pub fn cxgbi_conn_pdu_ready(: *mut cxgbi_sock);
}
extern "C" {
    pub fn cxgbi_conn_alloc_pdu(: *mut iscsi_task, _arg: u8) -> c_int;
}
extern "C" {
    pub fn cxgbi_conn_init_pdu(: *mut iscsi_task, int: unsigned, int: unsigned) -> c_int;
}
extern "C" {
    pub fn cxgbi_conn_xmit_pdu(: *mut iscsi_task) -> c_int;
}
extern "C" {
    pub fn cxgbi_cleanup_task(task: *mut iscsi_task);
}
extern "C" {
    pub fn cxgbi_attr_is_visible(param_type: c_int, param: c_int) -> umode_t;
}
extern "C" {
    pub fn cxgbi_get_conn_stats(: *mut iscsi_cls_conn, : *mut iscsi_stats);
}
extern "C" {
    pub fn cxgbi_get_ep_param(ep: *mut iscsi_endpoint, iscsi_param: enum, : *mut c_char) -> c_int;
}
extern "C" {
    pub fn cxgbi_destroy_session(: *mut iscsi_cls_session);
}
extern "C" {
    pub fn cxgbi_get_host_param(: *mut Scsi_Host, iscsi_host_param: enum, : *mut c_char) -> c_int;
}
extern "C" {
    pub fn cxgbi_ep_poll(: *mut iscsi_endpoint, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn cxgbi_ep_disconnect(: *mut iscsi_endpoint);
}
extern "C" {
    pub fn cxgbi_parse_pdu_itt(: *mut iscsi_conn, _arg: itt_t, : *mut c_int, : *mut c_int);
}
extern "C" {
    pub fn cxgbi_ddp_cleanup(: *mut cxgbi_device) -> c_int;
}
extern "C" {
    pub fn cxgbi_ddp_page_size_factor(: *mut c_int);
}
