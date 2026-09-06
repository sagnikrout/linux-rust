//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/ulp/srpt/ib_srpt.h
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
// Copyright (c) 2006 - 2009 Mellanox Technology Inc.  All rights reserved.
// Copyright (C) 2009 - 2010 Bart Van Assche <bvanassche@acm.org>.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

//
// The prefix the ServiceName field must start with in the device management
// ServiceEntries attribute pair. See also the SRP specification.
//

//
// SRP IOControllerProfile attributes for SRP target ports that have
// not been defined in <scsi/srp.h>. Source: section B.7, table B.7
// in the SRP specification.
//
// srp_login_cmd.req_flags bitmasks. See also table 9 in the SRP
// specification.
//
// srp_cmd.sol_nt / srp_tsk_mgmt.sol_not bitmasks. See also tables
// 18 and 20 in the SRP specification.
//
// srp_rsp.sol_not / srp_t_logout.sol_not bitmasks. See also tables
// 16 and 22 in the SRP specification.
//
// See also table 24 in the SRP specification.
// See also table 21 in the SRP specification.
//
// enum srpt_command_state - SCSI command state managed by SRPT
// @SRPT_STATE_NEW:           New command arrived and is being processed.
// @SRPT_STATE_NEED_DATA:     Processing a write or bidir command and waiting
// for data arrival.
// @SRPT_STATE_DATA_IN:       Data for the write or bidir command arrived and is
// being processed.
// @SRPT_STATE_CMD_RSP_SENT:  SRP_RSP for SRP_CMD has been sent.
// @SRPT_STATE_MGMT:          Processing a SCSI task management command.
// @SRPT_STATE_MGMT_RSP_SENT: SRP_RSP for SRP_TSK_MGMT has been sent.
// @SRPT_STATE_DONE:          Command processing finished successfully, command
// processing has been aborted or command processing
// failed.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum srpt_command_state {
    SRPT_STATE_NEW		 = 0,
    SRPT_STATE_NEED_DATA	 = 1,
    SRPT_STATE_DATA_IN	 = 2,
    SRPT_STATE_CMD_RSP_SENT	 = 3,
    SRPT_STATE_MGMT		 = 4,
    SRPT_STATE_MGMT_RSP_SENT = 5,
    SRPT_STATE_DONE		 = 6,
}

//
// struct srpt_ioctx - shared SRPT I/O context information
// @cqe:   Completion queue element.
// @buf:   Pointer to the buffer.
// @dma:   DMA address of the buffer.
// @offset: Offset of the first byte in @buf and @dma that is actually used.
// @index: Index of the I/O context in its ioctx_ring array.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srpt_ioctx {
    pub cqe: ib_cqe,
    pub buf: *mut c_void,
    pub dma: dma_addr_t,
    pub offset: u32,
    pub index: u32,
}

//
// struct srpt_recv_ioctx - SRPT receive I/O context
// @ioctx:     See above.
// @wait_list: Node for insertion in srpt_rdma_ch.cmd_wait_list.
// @byte_len:  Number of bytes in @ioctx.buf.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srpt_recv_ioctx {
    pub ioctx: srpt_ioctx,
    pub wait_list: list_head,
    pub byte_len: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct srpt_rw_ctx {
    pub rw: rdma_rw_ctx,
    pub sg: *mut scatterlist,
    pub nents: c_uint,
}

//
// struct srpt_send_ioctx - SRPT send I/O context
// @ioctx:       See above.
// @ch:          Channel pointer.
// @recv_ioctx:  Receive I/O context associated with this send I/O context.
// Only used for processing immediate data.
// @s_rw_ctx:    @rw_ctxs points here if only a single rw_ctx is needed.
// @rw_ctxs:     RDMA read/write contexts.
// @imm_sg:      Scatterlist for immediate data.
// @rdma_cqe:    RDMA completion queue element.
// @state:       I/O context state.
// @cmd:         Target core command data structure.
// @sense_data:  SCSI sense data.
// @n_rdma:      Number of work requests needed to transfer this ioctx.
// @n_rw_ctx:    Size of rw_ctxs array.
// @queue_status_only: Send a SCSI status back to the initiator but no data.
// @sense_data:  Sense data to be sent to the initiator.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srpt_send_ioctx {
    pub ioctx: srpt_ioctx,
    pub ch: *mut srpt_rdma_ch,
    pub recv_ioctx: *mut srpt_recv_ioctx,
    pub s_rw_ctx: srpt_rw_ctx,
    pub rw_ctxs: *mut srpt_rw_ctx,
    pub imm_sg: scatterlist,
    pub rdma_cqe: ib_cqe,
    pub state: srpt_command_state,
    pub cmd: se_cmd,
    pub n_rdma: u8,
    pub n_rw_ctx: u8,
    pub queue_status_only: bool,
    pub sense_data: [u8; TRANSPORT_SENSE_BUFFER],
}

//
// enum rdma_ch_state - SRP channel state
// @CH_CONNECTING:    QP is in RTR state; waiting for RTU.
// @CH_LIVE:	      QP is in RTS state.
// @CH_DISCONNECTING: DREQ has been sent and waiting for DREP or DREQ has
// been received.
// @CH_DRAINING:      DREP has been received or waiting for DREP timed out
// and last work request has been queued.
// @CH_DISCONNECTED:  Last completion has been received.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_ch_state {
    CH_CONNECTING,
    CH_LIVE,
    CH_DISCONNECTING,
    CH_DRAINING,
    CH_DISCONNECTED,
}

//
// struct srpt_rdma_ch - RDMA channel
// @nexus:         I_T nexus this channel is associated with.
// @qp:            IB queue pair used for communicating over this channel.
// @ib_cm:	   See below.
// @ib_cm.cm_id:   IB CM ID associated with the channel.
// @rdma_cm:	   See below.
// @rdma_cm.cm_id: RDMA CM ID associated with the channel.
// @cq:            IB completion queue for this channel.
// @cq_size:	   Number of CQEs in @cq.
// @zw_cqe:	   Zero-length write CQE.
// @rcu:           RCU head.
// @kref:	   kref for this channel.
// @closed:	   Completion object that will be signaled as soon as a new
// channel object with the same identity can be created.
// @rq_size:       IB receive queue size.
// @max_rsp_size:  Maximum size of an RSP response message in bytes.
// @sq_wr_avail:   number of work requests available in the send queue.
// @sport:         pointer to the information of the HCA port used by this
// channel.
// @max_ti_iu_len: maximum target-to-initiator information unit length.
// @req_lim:       request limit: maximum number of requests that may be sent
// by the initiator without having received a response.
// @req_lim_delta: Number of credits not yet sent back to the initiator.
// @imm_data_offset: Offset from start of SRP_CMD for immediate data.
// @spinlock:      Protects free_list and state.
// @state:         channel state. See also enum rdma_ch_state.
// @using_rdma_cm: Whether the RDMA/CM or IB/CM is used for this channel.
// @processing_wait_list: Whether or not cmd_wait_list is being processed.
// @rsp_buf_cache: kmem_cache for @ioctx_ring.
// @ioctx_ring:    Send ring.
// @req_buf_cache: kmem_cache for @ioctx_recv_ring.
// @ioctx_recv_ring: Receive I/O context ring.
// @list:          Node in srpt_nexus.ch_list.
// @cmd_wait_list: List of SCSI commands that arrived before the RTU event. This
// list contains struct srpt_ioctx elements and is protected
// against concurrent modification by the cm_id spinlock.
// @pkey:          P_Key of the IB partition for this SRP channel.
// @sess:          Session information associated with this SRP channel.
// @sess_name:     Session name.
// @release_work:  Allows scheduling of srpt_release_channel().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srpt_rdma_ch {
    pub nexus: *mut srpt_nexus,
    pub qp: *mut ib_qp,
    pub cm_id: *mut ib_cm_id,
    pub ib_cm: },
    pub cm_id: *mut rdma_cm_id,
    pub rdma_cm: },
}

//
// struct srpt_nexus - I_T nexus
// @rcu:       RCU head for this data structure.
// @entry:     srpt_port.nexus_list list node.
// @ch_list:   struct srpt_rdma_ch list. Protected by srpt_port.mutex.
// @i_port_id: 128-bit initiator port identifier copied from SRP_LOGIN_REQ.
// @t_port_id: 128-bit target port identifier copied from SRP_LOGIN_REQ.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srpt_nexus {
    pub rcu: rcu_head,
    pub entry: list_head,
    pub ch_list: list_head,
    pub i_port_id: [u8; 16],
    pub t_port_id: [u8; 16],
}

//
// struct srpt_port_attrib - attributes for SRPT port
// @srp_max_rdma_size: Maximum size of SRP RDMA transfers for new connections.
// @srp_max_rsp_size: Maximum size of SRP response messages in bytes.
// @srp_sq_size: Shared receive queue (SRQ) size.
// @use_srq: Whether or not to use SRQ.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srpt_port_attrib {
    pub srp_max_rdma_size: u32,
    pub srp_max_rsp_size: u32,
    pub srp_sq_size: u32,
    pub use_srq: bool,
}

//
// struct srpt_tpg - information about a single "target portal group"
// @entry:	Entry in @sport_id->tpg_list.
// @sport_id:	Port name this TPG is associated with.
// @tpg:	LIO TPG data structure.
//
// Zero or more target portal groups are associated with each port name
// (srpt_port_id). With each TPG an ACL list is associated.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srpt_tpg {
    pub entry: list_head,
    pub sport_id: *mut srpt_port_id,
    pub tpg: se_portal_group,
}

//
// struct srpt_port_id - LIO RDMA port information
// @mutex:	Protects @tpg_list changes.
// @tpg_list:	TPGs associated with the RDMA port name.
// @wwn:	WWN associated with the RDMA port name.
// @name:	ASCII representation of the port name.
//
// Multiple sysfs directories can be associated with a single RDMA port. This
// data structure represents a single (port, name) pair.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srpt_port_id {
    pub mutex: mutex,
    pub tpg_list: list_head,
    pub wwn: se_wwn,
    pub name: [c_char; 64],
}

//
// struct srpt_port - SRPT RDMA port information
// @sdev:      backpointer to the HCA information.
// @mad_agent: per-port management datagram processing information.
// @enabled:   Whether or not this target port is enabled.
// @port:      one-based port number.
// @sm_lid:    cached value of the port's sm_lid.
// @lid:       cached value of the port's lid.
// @gid:       cached value of the port's gid.
// @work:      work structure for refreshing the aforementioned cached values.
// @guid_name: port name in GUID format.
// @guid_id:   LIO target port information for the port name in GUID format.
// @gid_name:  port name in GID format.
// @gid_id:    LIO target port information for the port name in GID format.
// @port_attrib:   Port attributes that can be accessed through configfs.
// @refcount:	   Number of objects associated with this port.
// @freed_channels: Completion that will be signaled once @refcount becomes 0.
// @mutex:	   Protects nexus_list.
// @nexus_list:	   Nexus list. See also srpt_nexus.entry.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srpt_port {
    pub sdev: *mut srpt_device,
    pub mad_agent: *mut ib_mad_agent,
    pub enabled: bool,
    pub port: u8,
    pub sm_lid: u32,
    pub lid: u32,
    pub gid: ib_gid,
    pub work: work_struct,
    pub guid_name: [c_char; 64],
    pub guid_id: *mut srpt_port_id,
    pub gid_name: [c_char; 64],
    pub gid_id: *mut srpt_port_id,
    pub port_attrib: srpt_port_attrib,
    pub refcount: core::sync::atomic::AtomicI32,
    pub freed_channels: *mut completion,
    pub mutex: mutex,
    pub nexus_list: list_head,
}

//
// struct srpt_device - information associated by SRPT with a single HCA
// @refcnt:	   Reference count for this device.
// @device:        Backpointer to the struct ib_device managed by the IB core.
// @pd:            IB protection domain.
// @lkey:          L_Key (local key) with write access to all local memory.
// @srq:           Per-HCA SRQ (shared receive queue).
// @cm_id:         Connection identifier.
// @srq_size:      SRQ size.
// @sdev_mutex:	   Serializes use_srq changes.
// @use_srq:       Whether or not to use SRQ.
// @req_buf_cache: kmem_cache for @ioctx_ring buffers.
// @ioctx_ring:    Per-HCA SRQ.
// @event_handler: Per-HCA asynchronous IB event handler.
// @list:          Node in srpt_dev_list.
// @port:          Information about the ports owned by this HCA.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srpt_device {
    pub refcnt: kref,
    pub device: *mut ib_device,
    pub pd: *mut ib_pd,
    pub lkey: u32,
    pub srq: *mut ib_srq,
    pub cm_id: *mut ib_cm_id,
    pub srq_size: c_int,
    pub sdev_mutex: mutex,
    pub use_srq: bool,
    pub req_buf_cache: *mut kmem_cache,
    pub ioctx_ring: *mut srpt_recv_ioctx,
    pub event_handler: ib_event_handler,
    pub list: list_head,
    pub port: [srpt_port; ],
}
