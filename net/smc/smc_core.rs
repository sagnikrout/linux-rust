//! Automatically rewritten from C Header to Rust Module
//! Source: net/smc/smc_core.h
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
// Definitions for SMC Connections, Link Groups and Links
//
// Copyright IBM Corp. 2016
//
// Author(s):  Ursula Braun <ubraun@linux.vnet.ibm.com>
//

// also is the default value for SMC-R v1 and v2.0
//

// SMC-R v2.1 and later negotiation, vendors or
// distributions may modify it to a value between
// 16-255 as needed.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_lgr_list {
    pub list: list_head,
    pub /: *mut *mut spinlock_t lock; / protects list of link groups,
    pub /: *mut *mut u32 num; / unique link group number,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smc_lgr_role {
    SMC_CLNT,	/* client */
    SMC_SERV	/* server */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smc_link_state {
    SMC_LNK_UNUSED,		/* link is unused */
    SMC_LNK_INACTIVE,	/* link is inactive */
    SMC_LNK_ACTIVATING,	/* link is being activated */
    SMC_LNK_ACTIVE,		/* link is active */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_wr_buf {
    pub raw: [u8; SMC_WR_BUF_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_wr_v2_buf {
    pub raw: [u8; SMC_WR_BUF_V2_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smc_wr_reg_state {
    POSTED,		/* ib_wr_reg_mr request posted */
    CONFIRMED,	/* ib_wr_reg_mr response: successful */
    FAILED		/* ib_wr_reg_mr response: failure */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_rdma_sge {
    pub wr_tx_rdma_sge: [ib_sge; SMC_IB_MAX_SEND_SGE],
}

// message send
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_rdma_sges {
    pub tx_rdma_sge: [smc_rdma_sge; SMC_MAX_RDMA_WRITES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_rdma_wr {
// send
//
    pub wr_tx_rdma: [ib_rdma_wr; SMC_MAX_RDMA_WRITES],
}

pub const SMC_LGR_ID_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_link {
    pub /: *mut *mut *mut smc_ib_device smcibdev; / ib-device,
    pub /: *mut *mut u8 ibport; / port - values 1 | 2,
    pub domain,: *mut *mut *mut ib_pd roce_pd; / IB protection,
// unique for every RoCE QP
//
    pub /: *mut *mut *mut ib_qp roce_qp; / IB queue pair,
    pub /: *mut *mut ib_qp_attr qp_attr; / IB queue pair attributes,
    pub /: *mut *mut *mut smc_wr_buf wr_tx_bufs; / WR send payload buffers,
    pub /: *mut *mut *mut ib_send_wr wr_tx_ibs; / WR send meta data,
    pub /: *mut *mut *mut ib_sge wr_tx_sges; / WR send gather meta data,
    pub data*/: *mut *mut *mut smc_rdma_sges wr_tx_rdma_sges;/RDMA WRITE gather meta,
    pub /: *mut *mut *mut smc_rdma_wr wr_tx_rdmas; / WR RDMA WRITE,
    pub /: *mut *mut *mut smc_wr_tx_pend wr_tx_pends; / WR send waiting for CQE,
    pub /: *mut *mut *mut completion wr_tx_compl; / WR send CQE completion,
// above four vectors have wr_tx_cnt elements and use the same index
    pub /: *mut *mut *mut ib_send_wr wr_tx_v2_ib; / WR send v2 meta data,
    pub data*/: *mut *mut *mut ib_sge wr_tx_v2_sge; / WR send v2 gather meta,
    pub /: *mut *mut *mut smc_wr_tx_pend wr_tx_v2_pend; / WR send v2 waiting for CQE,
    pub /: *mut *mut dma_addr_t wr_tx_dma_addr; / DMA address of wr_tx_bufs,
    pub buf*/: *mut *mut dma_addr_t wr_tx_v2_dma_addr; / DMA address of v2 tx,
    pub /: *mut *mut atomic_long_t wr_tx_id; / seq # of last sent WR,
    pub /: *mut *mut *mut unsigned long wr_tx_mask; / bit mask of used indexes,
    pub /: *mut *mut u32 wr_tx_cnt; / number of WR send buffers,
    pub /: *mut *mut wait_queue_head_t wr_tx_wait; / wait for free WR send buf,
    pub wr_tx_refs: percpu_ref,
    pub ____cacheline_aligned_in_smp: },
    pub tx_ref_comp: completion,
    pub /: *mut *mut *mut u8 wr_rx_bufs; / WR recv payload buffers,
    pub /: *mut *mut *mut ib_recv_wr wr_rx_ibs; / WR recv meta data,
    pub /: *mut *mut *mut ib_sge wr_rx_sges; / WR recv scatter meta data,
// above three vectors have wr_rx_cnt elements and use the same index
    pub /: *mut *mut int wr_rx_sge_cnt; / rx sge, V1 is 1, V2 is either 2 or 1,
    pub the: *mut *mut int wr_rx_buflen; / buffer len for the first sge, len for,
// second sge is lgr shared if rx sge is 2.
//
    pub /: *mut *mut dma_addr_t wr_rx_dma_addr; / DMA address of wr_rx_bufs,
    pub buf*/: *mut *mut dma_addr_t wr_rx_v2_dma_addr; / DMA address of v2 rx,
    pub /: *mut *mut u64 wr_rx_id; / seq # of last recv WR,
    pub /: *mut *mut u64 wr_rx_id_compl; / seq # of last completed WR,
    pub /: *mut *mut u32 wr_rx_cnt; / number of WR recv buffers,
    pub /: *mut *mut unsigned long wr_rx_tstamp; / jiffies when last buf rx,
    pub /: *mut *mut wait_queue_head_t wr_rx_empty_wait; / wait for RQ empty,
    pub /: *mut *mut ib_reg_wr wr_reg; / WR register memory region,
    pub /: *mut *mut wait_queue_head_t wr_reg_wait; / wait for wr_reg result,
    pub wr_reg_refs: percpu_ref,
    pub ____cacheline_aligned_in_smp: },
    pub reg_ref_comp: completion,
    pub /: *mut *mut smc_wr_reg_state wr_reg_state; / state of wr_reg request,
    pub id*/: *mut *mut u8 gid[SMC_GID_SIZE];/ gid matching used vlan,
    pub /: *mut *mut u8 sgid_index; / gid index for vlan id,
    pub /: *mut *mut u32 peer_qpn; / QP number of peer,
    pub /: *mut *mut ib_mtu path_mtu; / used mtu,
    pub /: *mut *mut ib_mtu peer_mtu; / mtu size of peer,
    pub /: *mut *mut u32 psn_initial; / QP tx initial packet seqno,
    pub /: *mut *mut u32 peer_psn; / QP rx initial packet seqno,
    pub /: *mut *mut u8 peer_mac[ETH_ALEN]; / = gid[8:10||13:15],
    pub peer*/: *mut *mut u8 peer_gid[SMC_GID_SIZE]; / gid of,
    pub /: *mut *mut u8 link_id; / unique # within link group,
    pub /: *mut *mut u8 link_uid[SMC_LGR_ID_SIZE]; / unique lnk id,
    pub /: *mut *mut u8 peer_link_uid[SMC_LGR_ID_SIZE]; / peer uid,
    pub /: *mut *mut u8 link_idx; / index in lgr link array,
    pub /: *mut *mut u8 link_is_asym; / is link asymmetric?,
    pub /: *mut *mut u8 clearing : 1; / link is being cleared,
    pub /: *mut *mut refcount_t refcnt; / link reference count,
    pub /: *mut *mut *mut smc_link_group lgr; / parent link group,
    pub /: *mut *mut work_link_down_wrk; / wrk to bring link down,
    pub /: *mut *mut char ibname[IB_DEVICE_NAME_MAX]; / ib device name,
    pub /: *mut *mut int ndev_ifidx; / network device ifindex,
    pub /: *mut *mut smc_link_state state; / state of link,
    pub /: *mut *mut delayed_work llc_testlink_wrk; / testlink worker,
    pub /: *mut *mut completion llc_testlink_resp; / wait for rx of testlink,
    pub /: *mut *mut int llc_testlink_time; / testlink interval,
    pub /: *mut *mut atomic_t conn_cnt; / connections on this link,
    pub max_send_wr: u16,
    pub max_recv_wr: u16,
}

// For now we just allow one parallel link per link group. The SMC protocol
// allows more (up to 8).
//
pub const SMC_LINKS_PER_LGR_MAX: c_int = 3;
pub const SMC_SINGLE_LINK: c_int = 0;

// default value for smc-r v1.0 and v2.0
//

// SMC-R v2.1 and later negotiation, vendors or
// distributions may modify it to a value between
// 1-2 as needed.
//
// tx/rx buffer list element for sndbufs list and rmbs list of a lgr
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_buf_desc {
    pub list: list_head,
    pub /: *mut *mut *mut void cpu_addr; / virtual address of buffer,
    pub pages: *mut page,
    pub /: *mut *mut int len; / length of buffer,
    pub /: *mut *mut u32 used; / currently used / unused,
    pub sgt: [sg_table; SMC_LINKS_PER_LGR_MAX],
// virtual buffer
    pub mr: [*mut ib_mr; SMC_LINKS_PER_LGR_MAX],
// memory region: for rmb and
// vzalloced sndbuf
// incl. rkey provided to peer
// and lkey provided to local
//
    pub /: *mut *mut u32 order; / allocation order,
    pub is_conf_rkey: u8,
// confirm_rkey done
    pub is_reg_mr: [u8; SMC_LINKS_PER_LGR_MAX],
// mem region registered
    pub is_map_ib: [u8; SMC_LINKS_PER_LGR_MAX],
// mem region mapped to lnk
    pub is_dma_need_sync: u8,
    pub is_reg_err: u8,
// buffer registration err
    pub is_vm: u8,
// virtually contiguous
}

// SMC-D tx buffer
// no need for explicit writes
// SMC-D rx buffer:
// SBA index number
// DMB token number
// DMA address
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_rtoken {
    pub dma_addr: u64,
    pub rkey: u32,
}

// theoretically, the RFC states that largest size would be 512K,
// i.e. compressed 5 and thus 6 sizes (0..5), despite
// struct smc_clc_msg_accept_confirm.rmbe_size being a 4 bit value (0..15)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smc_lgr_type {
    SMC_LGR_NONE,			/* no active links, lgr to be deleted */
    SMC_LGR_SINGLE,			/* 1 active RNIC on each peer */
    SMC_LGR_SYMMETRIC,		/* 2 active RNICs on each peer */
    SMC_LGR_ASYMMETRIC_PEER,	/* local has 2, peer 1 active RNICs */
    SMC_LGR_ASYMMETRIC_LOCAL,	/* local has 1, peer 2 active RNICs */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smcr_buf_type {
    SMCR_PHYS_CONT_BUFS	= 0,
    SMCR_VIRT_CONT_BUFS	= 1,
    SMCR_MIXED_BUFS		= 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smc_llc_flowtype {
    SMC_LLC_FLOW_NONE	= 0,
    SMC_LLC_FLOW_ADD_LINK	= 2,
    SMC_LLC_FLOW_DEL_LINK	= 4,
    SMC_LLC_FLOW_REQ_ADD_LINK = 5,
    SMC_LLC_FLOW_RKEY	= 6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_llc_flow {
    pub type: smc_llc_flowtype,
    pub qentry: *mut smc_llc_qentry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_link_group {
    pub list: list_head,
    pub /: *mut *mut rb_root conns_all; / connection tree,
    pub /: *mut *mut rwlock_t conns_lock; / protects conns_all,
    pub /: *mut *mut unsigned int conns_num; / current # of connections,
    pub /: *mut *mut unsigned short vlan_id; / vlan id of link group,
    pub /: *mut *mut list_head sndbufs[SMC_RMBE_SIZES];/ tx buffers,
    pub /: *mut *mut rw_semaphore sndbufs_lock; / protects tx buffers,
    pub /: *mut *mut list_head rmbs[SMC_RMBE_SIZES]; / rx buffers,
    pub /: *mut *mut rw_semaphore rmbs_lock; / protects rx buffers,
    pub /: *mut *mut u64 alloc_sndbufs; / stats of tx buffers,
    pub /: *mut *mut u64 alloc_rmbs; / stats of rx buffers,
    pub /: *mut *mut u8 id[SMC_LGR_ID_SIZE]; / unique lgr id,
    pub /: *mut *mut delayed_work free_work; / delayed freeing of an lgr,
    pub /: *mut *mut work_terminate_work; / abnormal lgr termination,
    pub /: *mut *mut *mut workqueue_tx_wq; / wq for conn. tx workers,
    pub /: *mut *mut u8 sync_err : 1; / lgr no longer fits to peer,
    pub /: *mut *mut u8 terminating : 1;/ lgr is terminating,
    pub /: *mut *mut u8 freeing : 1; / lgr is being freed,
    pub /: *mut *mut refcount_t refcnt; / lgr reference count,
    pub /: *mut *mut bool is_smcd; / SMC-R or SMC-D,
    pub smc_version: u8,
    pub negotiated_eid: [u8; SMC_MAX_EID_LEN],
    pub /: *mut *mut u8 peer_os; / peer operating system,
    pub peer_smc_release: u8,
    pub peer_hostname: [u8; SMC_MAX_HOSTNAME_LEN],
    pub role: smc_lgr_role,
// client or server
    pub lnk: [smc_link; SMC_LINKS_PER_LGR_MAX],
// smc link
    pub wr_rx_buf_v2: *mut smc_wr_v2_buf,
// WR v2 recv payload buffer
    pub wr_tx_buf_v2: *mut smc_wr_v2_buf,
// WR v2 send payload buffer
    pub peer_systemid: [c_char; SMC_SYSTEMID_LEN],
// unique system_id of peer
// remote addr/key pairs
    pub SMC_RMBS_PER_LGR_MAX): DECLARE_BITMAP(rtokens_used_mask,,
// used rtoken elements
    pub next_link_id: u8,
    pub type: smc_lgr_type,
    pub buf_type: smcr_buf_type,
// redundancy state
    pub 1]: u8 pnet_id[SMC_MAX_PNETID_LEN +,
// pnet id of this lgr
    pub llc_event_q: list_head,
// queue for llc events
    pub llc_event_q_lock: spinlock_t,
// protects llc_event_q
    pub llc_conf_mutex: rw_semaphore,
// protects lgr reconfig.
    pub llc_add_link_work: work_struct,
    pub llc_del_link_work: work_struct,
    pub llc_event_work: work_struct,
// llc event worker
    pub llc_flow_waiter: wait_queue_head_t,
// w4 next llc event
    pub llc_msg_waiter: wait_queue_head_t,
// w4 next llc msg
    pub llc_flow_lcl: smc_llc_flow,
// llc local control field
    pub llc_flow_rmt: smc_llc_flow,
// llc remote control field
    pub delayed_event: *mut smc_llc_qentry,
// arrived when flow active
    pub llc_flow_lock: spinlock_t,
// protects llc flow
    pub llc_testlink_time: c_int,
// link keep alive time
    pub llc_termination_rsn: u32,
// rsn code for termination
    pub nexthop_mac: [u8; ETH_ALEN],
    pub uses_gateway: u8,
    pub saddr: __be32,
// net namespace
    pub net: *mut net,
    pub max_conns: u8,
// max conn can be assigned to lgr
    pub max_links: u8,
// max links can be added in lgr
    pub max_send_wr: u16,
// number of WR buffers on send
    pub max_recv_wr: u16,
// number of WR buffers on recv
}

// Peer GID (remote)
// ISM device for VLAN reg.
// peer triggered shutdownn
pub const GID_LIST_SIZE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_gidlist {
    pub len: u8,
    pub list: [u8; GID_LIST_SIZE][SMC_GID_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_init_info_smcrv2 {
// Input fields
    pub saddr: __be32,
    pub clc_sk: *mut sock,
    pub daddr: __be32,
// Output fields when saddr is set
    pub ib_dev_v2: *mut smc_ib_device,
    pub ib_port_v2: u8,
    pub ib_gid_v2: [u8; SMC_GID_SIZE],
// Additional output fields when clc_sk and daddr is set as well
    pub uses_gateway: u8,
    pub nexthop_mac: [u8; ETH_ALEN],
    pub gidlist: smc_gidlist,
}

// max # of proposed non-native ISM devices,
// which can't exceed the max # of CHID-GID
// entries in CLC proposal SMC-Dv2 extension.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_init_info {
    pub is_smcd: u8,
    pub smc_type_v1: u8,
    pub smc_type_v2: u8,
    pub release_nr: u8,
    pub max_conns: u8,
    pub max_links: u8,
    pub first_contact_peer: u8,
    pub first_contact_local: u8,
    pub feature_mask: u16,
    pub vlan_id: c_ushort,
    pub rc: u32,
    pub negotiated_eid: [u8; SMC_MAX_EID_LEN],
// SMC-R
    pub smcr_version: u8,
    pub check_smcrv2: u8,
    pub peer_gid: [u8; SMC_GID_SIZE],
    pub peer_mac: [u8; ETH_ALEN],
    pub peer_systemid: [u8; SMC_SYSTEMID_LEN],
    pub ib_dev: *mut smc_ib_device,
    pub ib_gid: [u8; SMC_GID_SIZE],
    pub ib_port: u8,
    pub ib_clcqpn: u32,
    pub smcrv2: smc_init_info_smcrv2,
// SMC-D
    pub 1]: smcd_gid ism_peer_gid[SMC_MAX_V2_ISM_DEVS +,
    pub 1]: *mut *mut smcd_dev ism_dev[SMC_MAX_V2_ISM_DEVS +,
    pub 1]: u16 ism_chid[SMC_MAX_V2_ISM_DEVS +,
    pub /: *mut *mut u8 ism_offered_cnt; / # of ISM devices offered,
    pub dev*/: *mut *mut u8 ism_selected; / index of selected ISM,
    pub smcd_version: u8,
}

// Find the connection associated with the given alert token in the link group.
// To use rbtrees we have to implement our own search core.
// Requires @conns_lock
// @token	alert token to search for
// @lgr		 link group to search in
// Returns connection associated with token if found, NULL otherwise.
//
// Returns true if the specified link is usable.
//
// usable means the link is ready to receive RDMA messages, map memory
// on the link, etc. This doesn't ensure we are able to send RDMA messages
// on this link, if sending RDMA messages is needed, use smc_link_sendable()
//
// Returns true if the specified link is ready to receive AND send RDMA
// messages.
//
// For the client side in first contact, the underlying QP may still in
// RESET or RTR when the link state is ACTIVATING, checks in smc_link_usable()
// is not strong enough. For those places that need to send any CDC or LLC
// messages, use smc_link_sendable(), otherwise, use smc_link_usable() instead
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_pci_dev {
    pub pci_fid: __u32,
    pub pci_pchid: __u16,
    pub pci_vendor: __u16,
    pub pci_device: __u16,
    pub pci_id: [__u8; SMC_PCI_ID_STR_LEN],
}

extern "C" {
    pub fn smc_lgr_cleanup_early(lgr: *mut smc_link_group);
}
extern "C" {
    pub fn smc_lgr_terminate_sched(lgr: *mut smc_link_group);
}
extern "C" {
    pub fn smc_lgr_hold(lgr: *mut smc_link_group);
}
extern "C" {
    pub fn smc_lgr_put(lgr: *mut smc_link_group);
}
extern "C" {
    pub fn smcr_port_add(smcibdev: *mut smc_ib_device, ibport: u8);
}
extern "C" {
    pub fn smcr_port_err(smcibdev: *mut smc_ib_device, ibport: u8);
}
extern "C" {
    pub fn smc_smcd_terminate_all(dev: *mut smcd_dev);
}
extern "C" {
    pub fn smc_smcr_terminate_all(smcibdev: *mut smc_ib_device);
}
extern "C" {
    pub fn smc_buf_create(smc: *mut smc_sock, is_smcd: bool) -> c_int;
}
extern "C" {
    pub fn smcd_buf_attach(smc: *mut smc_sock) -> c_int;
}
extern "C" {
    pub fn smc_uncompress_bufsize(compressed: u8) -> c_int;
}
extern "C" {
    pub fn smc_rtoken_add(lnk: *mut smc_link, nw_vaddr: __be64, nw_rkey: __be32) -> c_int;
}
extern "C" {
    pub fn smc_rtoken_delete(lnk: *mut smc_link, nw_rkey: __be32) -> c_int;
}
extern "C" {
    pub fn smc_sndbuf_sync_sg_for_device(conn: *mut smc_connection);
}
extern "C" {
    pub fn smc_rmb_sync_sg_for_cpu(conn: *mut smc_connection);
}
extern "C" {
    pub fn smc_vlan_by_tcpsk(clcsock: *mut socket, ini: *mut smc_init_info) -> c_int;
}
extern "C" {
    pub fn smc_conn_free(conn: *mut smc_connection);
}
extern "C" {
    pub fn smc_conn_create(smc: *mut smc_sock, ini: *mut smc_init_info) -> c_int;
}
extern "C" {
    pub fn smc_core_init() -> c_int;
}
extern "C" {
    pub fn smc_core_exit();
}
extern "C" {
    pub fn smcr_link_clear(lnk: *mut smc_link, log: bool);
}
extern "C" {
    pub fn smcr_link_hold(lnk: *mut smc_link);
}
extern "C" {
    pub fn smcr_link_put(lnk: *mut smc_link);
}
extern "C" {
    pub fn smcr_buf_map_lgr(lnk: *mut smc_link) -> c_int;
}
extern "C" {
    pub fn smcr_buf_reg_lgr(lnk: *mut smc_link) -> c_int;
}
extern "C" {
    pub fn smcr_lgr_set_type(lgr: *mut smc_link_group, new_type: smc_lgr_type);
}
extern "C" {
    pub fn smcr_link_reg_buf(link: *mut smc_link, rmb_desc: *mut smc_buf_desc) -> c_int;
}
extern "C" {
    pub fn smcr_link_down_cond(lnk: *mut smc_link);
}
extern "C" {
    pub fn smcr_link_down_cond_sched(lnk: *mut smc_link);
}
extern "C" {
    pub fn smc_nl_get_sys_info(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn smcr_nl_get_lgr(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn smcr_nl_get_link(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn smcd_nl_get_lgr(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
}
