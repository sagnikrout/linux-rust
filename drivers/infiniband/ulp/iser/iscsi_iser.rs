//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/ulp/iser/iscsi_iser.h
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
// iSER transport for the Open iSCSI Initiator & iSER transport internals
//
// Copyright (C) 2004 Dmitry Yusupov
// Copyright (C) 2004 Alex Aizman
// Copyright (C) 2005 Mike Christie
// based on code maintained by open-iscsi@googlegroups.com
//
// Copyright (c) 2004, 2005, 2006 Voltaire, Inc. All rights reserved.
// Copyright (c) 2005, 2006 Cisco Systems.  All rights reserved.
// Copyright (c) 2013-2014 Mellanox Technologies. All rights reserved.
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

// Default support is 512KB I/O size
pub const ISER_DEF_MAX_SECTORS: c_int = 1024;

// Maximum support is 16MB I/O size

pub const ISER_DEF_XMIT_CMDS_DEFAULT: c_int = 512;

// QP settings
// Maximal bounds on received asynchronous PDUs

// SCSI_TMFUNC(2), LOGOUT(1)

// the max TX (send) WR supported by the iSER QP is defined by
// max_send_wr = T * (1 + D) + C ; D is how many inflight dataouts we expect
// to have at max for SCSI command. The tx posting & completion handling code
// supports -EAGAIN scheme where tx is suspended till the QP has room for more
// send WR. D=8 comes from 64K/8K
pub const ISER_INFLIGHT_DATAOUTS: c_int = 8;

// Max registration work requests per command
pub const ISER_MAX_REG_WR_PER_CMD: c_int = 5;
// For Signature we don't support DATAOUTs so no need to make room for them

// Constant PDU lengths calculations

pub const ISER_RECV_DATA_SEG_LEN: c_int = 128;

// Length of an object name string
pub const ISER_OBJECT_NAME_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iser_conn_state {
    ISER_CONN_INIT,		   /* descriptor allocd, no conn          */
    ISER_CONN_PENDING,	   /* in the process of being established */
    ISER_CONN_UP,		   /* up and running                      */
    ISER_CONN_TERMINATING,	   /* in the process of being terminated  */
    ISER_CONN_DOWN,		   /* shut down                           */
    ISER_CONN_STATES_NUM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iser_task_status {
    ISER_TASK_STATUS_INIT = 0,
    ISER_TASK_STATUS_STARTED,
    ISER_TASK_STATUS_COMPLETED
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iser_data_dir {
    ISER_DIR_IN = 0,	   /* to initiator */
    ISER_DIR_OUT,		   /* from initiator */
    ISER_DIRS_NUM
}

//
// struct iser_data_buf - iSER data buffer
//
// @sg:           pointer to the sg list
// @size:         num entries of this sg
// @data_len:     total buffer byte len
// @dma_nents:    returned by dma_map_sg
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iser_data_buf {
    pub sg: *mut scatterlist,
    pub size: c_int,
    pub data_len: c_ulong,
    pub dma_nents: c_int,
}

// fwd declarations
//
// struct iser_mem_reg - iSER memory registration info
//
// @sge:          memory region sg element
// @rkey:         memory region remote key
// @desc:         pointer to fast registration context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iser_mem_reg {
    pub sge: ib_sge,
    pub rkey: u32,
    pub desc: *mut iser_fr_desc,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iser_desc_type {
    ISCSI_TX_CONTROL ,
    ISCSI_TX_SCSI_COMMAND,
    ISCSI_TX_DATAOUT
}

//
// struct iser_tx_desc - iSER TX descriptor
//
// @iser_header:   iser header
// @iscsi_header:  iscsi header
// @type:          command/control/dataout
// @dma_addr:      header buffer dma_address
// @tx_sg:         sg[0] points to iser/iscsi headers
// sg[1] optionally points to either of immediate data
// unsolicited data-out or control
// @num_sge:       number sges used on this TX task
// @cqe:           completion handler
// @mapped:        Is the task header mapped
// @reg_wr:        registration WR
// @send_wr:       send WR
// @inv_wr:        invalidate WR
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iser_tx_desc {
    pub iser_header: iser_ctrl,
    pub iscsi_header: iscsi_hdr,
    pub type: iser_desc_type,
    pub dma_addr: u64,
    pub tx_sg: [ib_sge; 2],
    pub num_sge: c_int,
    pub cqe: ib_cqe,
    pub mapped: bool,
    pub reg_wr: ib_reg_wr,
    pub send_wr: ib_send_wr,
    pub inv_wr: ib_send_wr,
}

//
// struct iser_rx_desc - iSER RX descriptor
//
// @iser_header:   iser header
// @iscsi_header:  iscsi header
// @data:          received data segment
// @dma_addr:      receive buffer dma address
// @rx_sg:         ib_sge of receive buffer
// @cqe:           completion handler
// @pad:           for sense data TODO: Modify to maximum sense length supported
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iser_rx_desc {
    pub iser_header: iser_ctrl,
    pub iscsi_header: iscsi_hdr,
    pub data: [c_char; ISER_RECV_DATA_SEG_LEN],
    pub dma_addr: u64,
    pub rx_sg: ib_sge,
    pub cqe: ib_cqe,
    pub pad: [c_char; ISER_RX_PAD_SIZE],
    pub __packed: },
//
// struct iser_login_desc - iSER login descriptor
//
// @req:           pointer to login request buffer
// @rsp:           pointer to login response buffer
// @req_dma:       DMA address of login request buffer
// @rsp_dma:       DMA address of login response buffer
// @sge:           IB sge for login post recv
// @cqe:           completion handler
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iser_login_desc {
    pub req: *mut c_void,
    pub rsp: *mut c_void,
    pub req_dma: u64,
    pub rsp_dma: u64,
    pub sge: ib_sge,
    pub cqe: ib_cqe,
    pub __packed: },
    pub iser_conn: struct,
    pub ib_conn: struct,
//
// struct iser_device - iSER device handle
//
// @ib_device:     RDMA device
// @pd:            Protection Domain for this device
// @event_handler: IB events handle routine
// @ig_list:	   entry in devices list
// @refcount:      Reference counter, dominated by open iser connections
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iser_device {
    pub ib_device: *mut ib_device,
    pub pd: *mut ib_pd,
    pub event_handler: ib_event_handler,
    pub ig_list: list_head,
    pub refcount: c_int,
}

//
// struct iser_reg_resources - Fast registration resources
//
// @mr:         memory region
// @sig_mr:     signature memory region
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iser_reg_resources {
    pub mr: *mut ib_mr,
    pub sig_mr: *mut ib_mr,
}

//
// struct iser_fr_desc - Fast registration descriptor
//
// @list:           entry in connection fastreg pool
// @rsc:            data buffer registration resources
// @sig_protected:  is region protected indicator
// @all_list:       first and last list members
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iser_fr_desc {
    pub list: list_head,
    pub rsc: iser_reg_resources,
    pub sig_protected: bool,
    pub all_list: list_head,
}

//
// struct iser_fr_pool - connection fast registration pool
//
// @list:                list of fastreg descriptors
// @lock:                protects fastreg pool
// @size:                size of the pool
// @all_list:            first and last list members
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iser_fr_pool {
    pub list: list_head,
    pub lock: spinlock_t,
    pub size: c_int,
    pub all_list: list_head,
}

//
// struct ib_conn - Infiniband related objects
//
// @cma_id:              rdma_cm connection maneger handle
// @qp:                  Connection Queue-pair
// @cq:                  Connection completion queue
// @cq_size:             The number of max outstanding completions
// @device:              reference to iser device
// @fr_pool:             connection fast registration pool
// @pi_support:          Indicate device T10-PI support
// @reg_cqe:             completion handler
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_conn {
    pub cma_id: *mut rdma_cm_id,
    pub qp: *mut ib_qp,
    pub cq: *mut ib_cq,
    pub cq_size: u32,
    pub device: *mut iser_device,
    pub fr_pool: iser_fr_pool,
    pub pi_support: bool,
    pub reg_cqe: ib_cqe,
}

//
// struct iser_conn - iSER connection context
//
// @ib_conn:          connection RDMA resources
// @iscsi_conn:       link to matching iscsi connection
// @ep:               transport handle
// @state:            connection logical state
// @qp_max_recv_dtos: maximum number of data outs, corresponds
// to max number of post recvs
// @max_cmds:         maximum cmds allowed for this connection
// @name:             connection peer portal
// @release_work:     deferred work for release job
// @state_mutex:      protects iser onnection state
// @stop_completion:  conn_stop completion
// @ib_completion:    RDMA cleanup completion
// @up_completion:    connection establishment completed
// (state is ISER_CONN_UP)
// @conn_list:        entry in ig conn list
// @login_desc:       login descriptor
// @rx_descs:         rx buffers array (cyclic buffer)
// @num_rx_descs:     number of rx descriptors
// @scsi_sg_tablesize: scsi host sg_tablesize
// @pages_per_mr:     maximum pages available for registration
// @snd_w_inv:        connection uses remote invalidation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iser_conn {
    pub ib_conn: ib_conn,
    pub iscsi_conn: *mut iscsi_conn,
    pub ep: *mut iscsi_endpoint,
    pub state: iser_conn_state,
    pub qp_max_recv_dtos: unsigned,
    pub max_cmds: u16,
    pub name: [c_char; ISER_OBJECT_NAME_SIZE],
    pub release_work: work_struct,
    pub state_mutex: mutex,
    pub stop_completion: completion,
    pub ib_completion: completion,
    pub up_completion: completion,
    pub conn_list: list_head,
    pub login_desc: iser_login_desc,
    pub rx_descs: *mut iser_rx_desc,
    pub num_rx_descs: u32,
    pub scsi_sg_tablesize: c_ushort,
    pub pages_per_mr: c_ushort,
    pub snd_w_inv: bool,
}

//
// struct iscsi_iser_task - iser task context
//
// @desc:     TX descriptor
// @iser_conn:        link to iser connection
// @status:           current task status
// @sc:               link to scsi command
// @command_sent:     indicate if command was sent
// @dir:              iser data direction
// @rdma_reg:         task rdma registration desc
// @data:             iser data buffer desc
// @prot:             iser protection buffer desc
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_iser_task {
    pub desc: iser_tx_desc,
    pub iser_conn: *mut iser_conn,
    pub status: iser_task_status,
    pub sc: *mut scsi_cmnd,
    pub command_sent: c_int,
    pub dir: [c_int; ISER_DIRS_NUM],
    pub rdma_reg: [iser_mem_reg; ISER_DIRS_NUM],
    pub data: [iser_data_buf; ISER_DIRS_NUM],
    pub prot: [iser_data_buf; ISER_DIRS_NUM],
}

//
// struct iser_global - iSER global context
//
// @device_list_mutex:    protects device_list
// @device_list:          iser devices global list
// @connlist_mutex:       protects connlist
// @connlist:             iser connections global list
// @desc_cache:           kmem cache for tx dataout
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iser_global {
    pub device_list_mutex: mutex,
    pub device_list: list_head,
    pub connlist_mutex: mutex,
    pub connlist: list_head,
    pub desc_cache: *mut kmem_cache,
}

extern "C" {
    pub fn iser_conn_init(iser_conn: *mut iser_conn);
}
extern "C" {
    pub fn iser_conn_release(iser_conn: *mut iser_conn);
}
extern "C" {
    pub fn iser_conn_terminate(iser_conn: *mut iser_conn) -> c_int;
}
extern "C" {
    pub fn iser_release_work(work: *mut work_struct);
}
extern "C" {
    pub fn iser_err_comp(wc: *mut ib_wc, type: *const c_char);
}
extern "C" {
    pub fn iser_login_rsp(cq: *mut ib_cq, wc: *mut ib_wc);
}
extern "C" {
    pub fn iser_task_rsp(cq: *mut ib_cq, wc: *mut ib_wc);
}
extern "C" {
    pub fn iser_cmd_comp(cq: *mut ib_cq, wc: *mut ib_wc);
}
extern "C" {
    pub fn iser_ctrl_comp(cq: *mut ib_cq, wc: *mut ib_wc);
}
extern "C" {
    pub fn iser_dataout_comp(cq: *mut ib_cq, wc: *mut ib_wc);
}
extern "C" {
    pub fn iser_reg_comp(cq: *mut ib_cq, wc: *mut ib_wc);
}
extern "C" {
    pub fn iser_task_rdma_init(task: *mut iscsi_iser_task);
}
extern "C" {
    pub fn iser_task_rdma_finalize(task: *mut iscsi_iser_task);
}
extern "C" {
    pub fn iser_free_rx_descriptors(iser_conn: *mut iser_conn);
}
extern "C" {
    pub fn iser_post_recvl(iser_conn: *mut iser_conn) -> c_int;
}
extern "C" {
    pub fn iser_post_send(ib_conn: *mut ib_conn, tx_desc: *mut iser_tx_desc) -> c_int;
}
extern "C" {
    pub fn iser_free_fastreg_pool(ib_conn: *mut ib_conn);
}
extern "C" {
    pub fn container_of(_arg: ib_conn, iser_conn: struct, _arg: ib_conn) -> return;
}
extern "C" {
    pub fn container_of(_arg: cqe, iser_rx_desc: struct, _arg: cqe) -> return;
}
extern "C" {
    pub fn container_of(_arg: cqe, iser_tx_desc: struct, _arg: cqe) -> return;
}
extern "C" {
    pub fn container_of(_arg: cqe, iser_login_desc: struct, _arg: cqe) -> return;
}
