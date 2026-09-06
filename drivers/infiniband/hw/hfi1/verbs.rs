//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/hfi1/verbs.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright(c) 2015 - 2018 Intel Corporation.
//

pub const HFI1_MAX_RDMA_ATOMIC: c_int = 16;
//
// Increment this value if any changes that break userspace ABI
// compatibility are made.
//
pub const HFI1_UVERBS_ABI_VERSION: c_int = 2;
// IB Performance Manager status values
pub const IB_PMA_SAMPLE_STATUS_DONE: c_uint = 0x00;
pub const IB_PMA_SAMPLE_STATUS_STARTED: c_uint = 0x01;
pub const IB_PMA_SAMPLE_STATUS_RUNNING: c_uint = 0x02;
// Mandatory IB performance counter select values.

// flags passed by hfi1_ib_rcv()

// 24Bits for qpn, upper 8Bits reserved
#[repr(C)]
#[derive(Copy, Clone)]
pub struct opa_16b_mgmt {
    pub dest_qpn: __be32,
    pub src_qpn: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_16b_header {
    pub lrh: [u32; 4],
    pub grh: ib_grh,
    pub oth: ib_other_headers,
    pub l: },
    pub oth: ib_other_headers,
    pub mgmt: opa_16b_mgmt,
    pub u: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_opa_header {
    pub /: *mut *mut ib_header ibh; / 9B header,
    pub /: *mut *mut hfi1_16b_header opah; / 16B header,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_ahg_info {
    pub ahgdesc: [u32; 2],
    pub tx_flags: u16,
    pub ahgcount: u8,
    pub ahgidx: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_sdma_header {
    pub pbc: __le64,
    pub hdr: hfi1_opa_header,
    pub __packed: },
//
// hfi1 specific data structures that will be hidden from rvt after the queue
// pair is made common
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_qp_priv {
    pub /: *mut *mut *mut hfi1_ahg_info s_ahg; / ahg info for next header,
    pub /: *mut *mut *mut sdma_engine s_sde; / current sde,
    pub /: *mut *mut *mut send_context s_sendcontext; / current sendcontext,
    pub /: *mut *mut *mut hfi1_ctxtdata rcd; / QP's receive context,
    pub /: *mut *mut *mut *mut page pages; / for TID page scan,
    pub /: *mut *mut u32 tid_enqueue; / saved when tid waited,
    pub /: *mut *mut u8 s_sc; / SC[0..4] for next packet,
    pub s_iowait: iowait,
    pub /: *mut *mut timer_list s_tid_timer; / for timing tid wait,
    pub /: *mut *mut timer_list s_tid_retry_timer; / for timing tid ack,
    pub /: *mut *mut list_head tid_wait; / for queueing tid space,
    pub opfn: hfi1_opfn_data,
    pub flow_state: tid_flow_state,
    pub tid_rdma: tid_rdma_qp_params,
    pub owner: *mut rvt_qp,
    pub s_running_pkt_size: u16,
    pub /: *mut *mut u8 hdr_type; / 9B or 16B,
    pub /: *mut *mut rvt_sge_state tid_ss; / SGE state pointer for 2nd leg,
    pub /: *mut *mut atomic_t n_requests; / # of TID RDMA requests in the,
// queue
    pub /: *mut *mut atomic_t n_tid_requests; / # of sent TID RDMA requests,
    pub tid_timer_timeout_jiffies: c_ulong,
    pub tid_retry_timeout_jiffies: c_ulong,
// variables for the TID RDMA SE state machine
    pub s_state: u8,
    pub s_retry: u8,
    pub /: *mut *mut u8 rnr_nak_state; / RNR NAK state,
    pub s_nak_state: u8,
    pub s_nak_psn: u32,
    pub s_flags: u32,
    pub s_tid_cur: u32,
    pub s_tid_head: u32,
    pub s_tid_tail: u32,
    pub /: *mut *mut u32 r_tid_head; / Most recently added TID RDMA request,
    pub /: *mut *mut u32 r_tid_tail; / the last completed TID RDMA request,
    pub /: *mut *mut u32 r_tid_ack; / the TID RDMA request to be ACK'ed,
    pub /: *mut *mut u32 r_tid_alloc; / Request for which we are allocating resources,
    pub /: *mut *mut u32 pending_tid_w_segs; / Num of pending tid write segments,
    pub /: *mut *mut u32 pending_tid_w_resp; / Num of pending tid write responses,
    pub /: *mut *mut u32 alloc_w_segs; / Number of segments for which write,
// resources have been allocated for this QP
// For TID RDMA READ
    pub /: *mut *mut u32 tid_r_reqs; / Num of tid reads requested,
    pub /: *mut *mut u32 tid_r_comp; / Num of tid reads completed,
    pub /: *mut *mut u32 pending_tid_r_segs; / Num of pending tid read segments,
    pub /: *mut *mut u16 pkts_ps; / packets per segment,
    pub /: *mut *mut u8 timeout_shift; / account for number of packets per segment,
    pub r_next_psn_kdeth: u32,
    pub r_next_psn_kdeth_save: u32,
    pub s_resync_psn: u32,
    pub /: *mut *mut u8 sync_pt; / Set when QP reaches sync point,
    pub resync: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_swqe_priv {
    pub tid_req: tid_rdma_request,
    pub /: *mut *mut rvt_sge_state ss; / Used for TID RDMA READ Request,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_ack_priv {
    pub /: *mut *mut rvt_sge_state ss; / used for TID WRITE RESP,
    pub tid_req: tid_rdma_request,
}

//
// This structure is used to hold commonly lookedup and computed values during
// the send engine progress.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_pkt_state {
    pub dev: *mut hfi1_ibdev,
    pub ibp: *mut hfi1_ibport,
    pub ppd: *mut hfi1_pportdata,
    pub s_txreq: *mut verbs_txreq,
    pub wait: *mut iowait_work,
    pub flags: c_ulong,
    pub timeout: c_ulong,
    pub timeout_int: c_ulong,
    pub cpu: c_int,
    pub opcode: u8,
    pub in_thread: bool,
    pub pkts_sent: bool,
}

pub const HFI1_PSN_CREDIT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_opcode_stats {
    pub /: *mut *mut u64 n_packets; / number of packets,
    pub /: *mut *mut u64 n_bytes; / total number of bytes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_opcode_stats_perctx {
    pub stats: [hfi1_opcode_stats; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_ibport {
    pub qp: [*mut rvt_qp __rcu; 2],
    pub rvp: rvt_ibport,
// the first 16 entries are sl_to_vl for !OPA
    pub sl_to_sc: [u8; 32],
    pub sc_to_sl: [u8; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_ibdev {
    pub /: *mut *mut rvt_dev_info rdi; / Must be first,
// QP numbers are shared by all IB ports
// protect txwait list
    pub ____cacheline_aligned_in_smp: seqlock_t txwait_lock,
    pub /: *mut *mut list_head txwait; / list for wait verbs_txreq,
    pub /: *mut *mut list_head memwait; / list for wait kernel memory,
    pub verbs_txreq_cache: *mut kmem_cache,
    pub n_txwait: u64,
    pub n_kmem_wait: u64,
    pub n_tidwait: u64,
// protect iowait lists
    pub ____cacheline_aligned_in_smp: seqlock_t iowait_lock,
    pub n_piowait: u64,
    pub n_piodrain: u64,
    pub mem_timer: timer_list,

// per HFI debugfs
    pub hfi1_ibdev_dbg: *mut dentry,
// per HFI symlinks to above
    pub hfi1_ibdev_link: *mut dentry,

    pub fault: *mut fault,

}

extern "C" {
    pub fn container_of(_arg: rdi, hfi1_ibdev: struct, _arg: rdi) -> return;
}
//
// This must be called with s_lock held.
//
extern "C" {
    pub fn hfi1_cap_mask_chg(rdi: *mut rvt_dev_info, port_num: u32);
}
extern "C" {
    pub fn hfi1_sys_guid_chg(ibp: *mut hfi1_ibport);
}
extern "C" {
    pub fn hfi1_node_desc_chg(ibp: *mut hfi1_ibport);
}
//
// The PSN_MASK and PSN_SHIFT allow for
// 1) comparing two PSNs
// 2) returning the PSN with any upper bits masked
// 3) returning the difference between to PSNs
//
// The number of significant bits in the PSN must
// necessarily be at least one bit less than
// the container holding the PSN.
//
pub const PSN_MASK: c_uint = 0x7FFFFFFF;
pub const PSN_SHIFT: c_int = 1;
pub const PSN_MODIFY_MASK: c_uint = 0xFFFFFF;
//
// Compare two PSNs
// Returns an integer <, ==, or > than zero.
//
// Return masked PSN
//
// Return delta between two PSNs
//
// Look through all the active flows for a TID RDMA request and find
// the one (if it exists) that contains the specified PSN.
//
extern "C" {
    pub fn __full_flow_psn(_arg: &flow->flow_state, _arg: psn) -> return;
}
extern "C" {
    pub fn hfi1_put_txreq(tx: *mut verbs_txreq);
}
extern "C" {
    pub fn hfi1_verbs_send(qp: *mut rvt_qp, ps: *mut hfi1_pkt_state) -> c_int;
}
extern "C" {
    pub fn hfi1_cnp_rcv(packet: *mut hfi1_packet);
}
extern "C" {
    pub fn hfi1_uc_rcv(packet: *mut hfi1_packet);
}
extern "C" {
    pub fn hfi1_rc_rcv(packet: *mut hfi1_packet);
}
extern "C" {
    pub fn ah_to_sc(ibdev: *mut ib_device, ah_attr: *mut rdma_ah_attr) -> u8;
}
extern "C" {
    pub fn hfi1_rc_verbs_aborted(qp: *mut rvt_qp, opah: *mut hfi1_opa_header);
}
extern "C" {
    pub fn hfi1_rc_send_complete(qp: *mut rvt_qp, opah: *mut hfi1_opa_header);
}
extern "C" {
    pub fn hfi1_ud_rcv(packet: *mut hfi1_packet);
}
extern "C" {
    pub fn hfi1_lookup_pkey_idx(ibp: *mut hfi1_ibport, pkey: u16) -> c_int;
}
extern "C" {
    pub fn hfi1_migrate_qp(qp: *mut rvt_qp);
}
extern "C" {
    pub fn hfi1_restart_rc(qp: *mut rvt_qp, psn: u32, wait: c_int);
}
extern "C" {
    pub fn hfi1_ruc_check_hdr(ibp: *mut hfi1_ibport, packet: *mut hfi1_packet) -> c_int;
}
extern "C" {
    pub fn _hfi1_do_send(work: *mut work_struct);
}
extern "C" {
    pub fn hfi1_do_send_from_rvt(qp: *mut rvt_qp);
}
extern "C" {
    pub fn hfi1_do_send(qp: *mut rvt_qp, in_thread: bool);
}
extern "C" {
    pub fn hfi1_send_rc_ack(packet: *mut hfi1_packet, is_fecn: bool);
}
extern "C" {
    pub fn hfi1_make_rc_req(qp: *mut rvt_qp, ps: *mut hfi1_pkt_state) -> c_int;
}
extern "C" {
    pub fn hfi1_make_uc_req(qp: *mut rvt_qp, ps: *mut hfi1_pkt_state) -> c_int;
}
extern "C" {
    pub fn hfi1_make_ud_req(qp: *mut rvt_qp, ps: *mut hfi1_pkt_state) -> c_int;
}
extern "C" {
    pub fn hfi1_register_ib_device(: *mut hfi1_devdata) -> c_int;
}
extern "C" {
    pub fn hfi1_unregister_ib_device(: *mut hfi1_devdata);
}
extern "C" {
    pub fn hfi1_kdeth_eager_rcv(packet: *mut hfi1_packet);
}
extern "C" {
    pub fn hfi1_kdeth_expected_rcv(packet: *mut hfi1_packet);
}
extern "C" {
    pub fn hfi1_ib_rcv(packet: *mut hfi1_packet);
}
extern "C" {
    pub fn hfi1_16B_rcv(packet: *mut hfi1_packet);
}
extern "C" {
    pub fn hfi1_get_npkeys(: *mut hfi1_devdata) -> unsigned;
}
extern "C" {
    pub fn hfi1_wait_kmem(qp: *mut rvt_qp);
}
