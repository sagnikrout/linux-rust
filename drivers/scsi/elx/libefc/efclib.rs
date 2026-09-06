//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/elx/libefc/efclib.h
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
// Copyright (C) 2021 Broadcom. All Rights Reserved. The term
// “Broadcom” refers to Broadcom Inc. and/or its subsidiaries.
//

pub const EFC_SERVICE_PARMS_LENGTH: c_int = 120;
pub const EFC_NAME_LENGTH: c_int = 32;
pub const EFC_SM_NAME_LENGTH: c_int = 64;
pub const EFC_DISPLAY_BUS_INFO_LENGTH: c_int = 16;
pub const EFC_WWN_LENGTH: c_int = 32;
pub const EFC_FC_ELS_DEFAULT_RETRIES: c_int = 3;
// Timeouts
pub const EFC_FC_ELS_SEND_DEFAULT_TIMEOUT: c_int = 0;
pub const EFC_FC_FLOGI_TIMEOUT_SEC: c_int = 5;
pub const EFC_SHUTDOWN_TIMEOUT_USEC: c_int = 30000000;
// Return values for calls from base driver to libefc
pub const EFC_SCSI_CALL_COMPLETE: c_int = 0;
pub const EFC_SCSI_CALL_ASYNC: c_int = 1;
// Local port topology
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efc_nport_topology {
    EFC_NPORT_TOPO_UNKNOWN = 0,
    EFC_NPORT_TOPO_FABRIC,
    EFC_NPORT_TOPO_P2P,
    EFC_NPORT_TOPO_FC_AL,
}

pub const enable_target_rscn(efc): c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efc_node_shutd_rsn {
    EFC_NODE_SHUTDOWN_DEFAULT = 0,
    EFC_NODE_SHUTDOWN_EXPLICIT_LOGO,
    EFC_NODE_SHUTDOWN_IMPLICIT_LOGO,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efc_node_send_ls_acc {
    EFC_NODE_SEND_LS_ACC_NONE = 0,
    EFC_NODE_SEND_LS_ACC_PLOGI,
    EFC_NODE_SEND_LS_ACC_PRLI,
}

pub const EFC_LINK_STATUS_UP: c_int = 0;
pub const EFC_LINK_STATUS_DOWN: c_int = 1;
// State machine context header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efc_sm_ctx {
    pub arg): *mut efc_sm_event evt, void,
    pub description: *const c_char,
    pub app: *mut c_void,
}

// Description of discovered Fabric Domain
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efc_domain_record {
    pub index: u32,
    pub priority: u32,
    pub address: [u8; 6],
    pub wwn: [u8; 8],
    pub vlan: [u8; 512],
    pub loop: [u8; 128],
    pub map: },
    pub speed: u32,
    pub fc_id: u32,
    pub is_loop: bool,
    pub is_nport: bool,
}

// Domain events
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efc_hw_domain_event {
    EFC_HW_DOMAIN_ALLOC_OK,
    EFC_HW_DOMAIN_ALLOC_FAIL,
    EFC_HW_DOMAIN_ATTACH_OK,
    EFC_HW_DOMAIN_ATTACH_FAIL,
    EFC_HW_DOMAIN_FREE_OK,
    EFC_HW_DOMAIN_FREE_FAIL,
    EFC_HW_DOMAIN_LOST,
    EFC_HW_DOMAIN_FOUND,
    EFC_HW_DOMAIN_CHANGED,
}

//
// Fibre Channel port object
//
// @list_entry:		nport list entry
// @ref:		reference count, each node takes a reference
// @release:		function to free nport object
// @efc:		pointer back to efc
// @instance_index:	unique instance index value
// @display_name:	port display name
// @is_vport:		Is NPIV port
// @free_req_pending:	pending request to free resources
// @attached:		mark attached if reg VPI succeeds
// @p2p_winner:		TRUE if we're the point-to-point winner
// @domain:		pointer back to domain
// @wwpn:		port wwpn
// @wwnn:		port wwnn
// @tgt_data:		target backend private port data
// @ini_data:		initiator backend private port data
// @indicator:		VPI
// @fc_id:		port FC address
// @dma:		memory for Service Parameters
// @wwnn_str:		wwpn string
// @sli_wwpn:		SLI provided wwpn
// @sli_wwnn:		SLI provided wwnn
// @sm:			nport state machine context
// @lookup:		fc_id to node lookup object
// @enable_ini:		SCSI initiator enabled for this port
// @enable_tgt:		SCSI target enabled for this port
// @enable_rscn:	port will be expecting RSCN
// @shutting_down:	nport in process of shutting down
// @p2p_port_id:	our port id for point-to-point
// @topology:		topology: fabric/p2p/unknown
// @service_params:	login parameters
// @p2p_remote_port_id:	remote node's port id for point-to-point
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efc_nport {
    pub list_entry: list_head,
    pub ref: kref,
    pub arg): *mut *mut void (release)(struct kref,
    pub efc: *mut efc,
    pub instance_index: u32,
    pub display_name: [c_char; EFC_NAME_LENGTH],
    pub is_vport: bool,
    pub free_req_pending: bool,
    pub attached: bool,
    pub attaching: bool,
    pub p2p_winner: bool,
    pub domain: *mut efc_domain,
    pub wwpn: u64,
    pub wwnn: u64,
    pub tgt_data: *mut c_void,
    pub ini_data: *mut c_void,
    pub indicator: u32,
    pub fc_id: u32,
    pub dma: efc_dma,
    pub wwnn_str: [u8; EFC_WWN_LENGTH],
    pub sli_wwpn: __be64,
    pub sli_wwnn: __be64,
    pub sm: efc_sm_ctx,
    pub lookup: xarray,
    pub enable_ini: bool,
    pub enable_tgt: bool,
    pub enable_rscn: bool,
    pub shutting_down: bool,
    pub p2p_port_id: u32,
    pub topology: efc_nport_topology,
    pub service_params: [u8; EFC_SERVICE_PARMS_LENGTH],
    pub p2p_remote_port_id: u32,
}

//
// Fibre Channel domain object
//
// This object is a container for the various SLI components needed
// to connect to the domain of a FC or FCoE switch
// @efc:		pointer back to efc
// @instance_index:	unique instance index value
// @display_name:	Node display name
// @nport_list:		linked list of nports associated with this domain
// @ref:		Reference count, each nport takes a reference
// @release:		Function to free domain object
// @ini_domain:		initiator backend private domain data
// @tgt_domain:		target backend private domain data
// @sm:			state machine context
// @fcf:		FC Forwarder table index
// @fcf_indicator:	FCFI
// @indicator:		VFI
// @nport_count:	Number of nports allocated
// @dma:		memory for Service Parameters
// @fcf_wwn:		WWN for FCF/switch
// @drvsm:		driver domain sm context
// @attached:		set true after attach completes
// @is_fc:		is FC
// @is_loop:		is loop topology
// @is_nlport:		is public loop
// @domain_found_pending:A domain found is pending, drec is updated
// @req_domain_free:	True if domain object should be free'd
// @req_accept_frames:	set in domain state machine to enable frames
// @domain_notify_pend:	Set in domain SM to avoid duplicate node event post
// @pending_drec:	Pending drec if a domain found is pending
// @service_params:	any nports service parameters
// @flogi_service_params:Fabric/P2p service parameters from FLOGI
// @lookup:		d_id to node lookup object
// @nport:		Pointer to first (physical) SLI port
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efc_domain {
    pub efc: *mut efc,
    pub display_name: [c_char; EFC_NAME_LENGTH],
    pub nport_list: list_head,
    pub ref: kref,
    pub arg): *mut *mut void (release)(struct kref,
    pub ini_domain: *mut c_void,
    pub tgt_domain: *mut c_void,
// Declarations private to HW/SLI
    pub fcf: u32,
    pub fcf_indicator: u32,
    pub indicator: u32,
    pub nport_count: u32,
    pub dma: efc_dma,
// Declarations private to FC trannport
    pub fcf_wwn: u64,
    pub drvsm: efc_sm_ctx,
    pub attached: bool,
    pub is_fc: bool,
    pub is_loop: bool,
    pub is_nlport: bool,
    pub domain_found_pending: bool,
    pub req_domain_free: bool,
    pub req_accept_frames: bool,
    pub domain_notify_pend: bool,
    pub pending_drec: efc_domain_record,
    pub service_params: [u8; EFC_SERVICE_PARMS_LENGTH],
    pub flogi_service_params: [u8; EFC_SERVICE_PARMS_LENGTH],
    pub lookup: xarray,
    pub nport: *mut efc_nport,
}

//
// Remote Node object
//
// This object represents a connection between the SLI port and another
// Nx_Port on the fabric. Note this can be either a well known port such
// as a F_Port (i.e. ff:ff:fe) or another N_Port.
// @indicator:		RPI
// @fc_id:		FC address
// @attached:		true if attached
// @nport:		associated SLI port
// @node:		associated node
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efc_remote_node {
    pub indicator: u32,
    pub index: u32,
    pub fc_id: u32,
    pub attached: bool,
    pub nport: *mut efc_nport,
    pub node: *mut c_void,
}

//
// FC Node object
// @efc:		pointer back to efc structure
// @display_name:	Node display name
// @nort:		Assosiated nport pointer.
// @hold_frames:	hold incoming frames if true
// @els_io_enabled:	Enable allocating els ios for this node
// @els_ios_lock:	lock to protect the els ios list
// @els_ios_list:	ELS I/O's for this node
// @ini_node:		backend initiator private node data
// @tgt_node:		backend target private node data
// @rnode:		Remote node
// @sm:			state machine context
// @evtdepth:		current event posting nesting depth
// @req_free:		this node is to be free'd
// @attached:		node is attached (REGLOGIN complete)
// @fcp_enabled:	node is enabled to handle FCP
// @rscn_pending:	for name server node RSCN is pending
// @send_plogi:		send PLOGI accept, upon completion of node attach
// @send_plogi_acc:	TRUE if io_alloc() is enabled.
// @send_ls_acc:	type of LS acc to send
// @ls_acc_io:		SCSI IO for LS acc
// @ls_acc_oxid:	OX_ID for pending accept
// @ls_acc_did:		D_ID for pending accept
// @shutdown_reason:	reason for node shutdown
// @sparm_dma_buf:	service parameters buffer
// @service_params:	plogi/acc frame from remote device
// @pend_frames_lock:	lock for inbound pending frames list
// @pend_frames:	inbound pending frames list
// @pend_frames_processed:count of frames processed in hold frames interval
// @ox_id_in_use:	used to verify one at a time us of ox_id
// @els_retries_remaining:for ELS, number of retries remaining
// @els_req_cnt:	number of outstanding ELS requests
// @els_cmpl_cnt:	number of outstanding ELS completions
// @abort_cnt:		Abort counter for debugging purpos
// @current_state_name:	current node state
// @prev_state_name:	previous node state
// @current_evt:	current event
// @prev_evt:		previous event
// @targ:		node is target capable
// @init:		node is init capable
// @refound:		Handle node refound case when node is being deleted
// @els_io_pend_list:	list of pending (not yet processed) ELS IOs
// @els_io_active_list:	list of active (processed) ELS IOs
// @nodedb_state:	Node debugging, saved state
// @gidpt_delay_timer:	GIDPT delay timer
// @time_last_gidpt_msec:Start time of last target RSCN GIDPT
// @wwnn:		remote port WWNN
// @wwpn:		remote port WWPN
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efc_node {
    pub efc: *mut efc,
    pub display_name: [c_char; EFC_NAME_LENGTH],
    pub nport: *mut efc_nport,
    pub ref: kref,
    pub arg): *mut *mut void (release)(struct kref,
    pub hold_frames: bool,
    pub els_io_enabled: bool,
    pub send_plogi_acc: bool,
    pub send_plogi: bool,
    pub rscn_pending: bool,
    pub fcp_enabled: bool,
    pub attached: bool,
    pub req_free: bool,
    pub els_ios_lock: spinlock_t,
    pub els_ios_list: list_head,
    pub ini_node: *mut c_void,
    pub tgt_node: *mut c_void,
    pub rnode: efc_remote_node,
// Declarations private to FC trannport
    pub sm: efc_sm_ctx,
    pub evtdepth: u32,
    pub send_ls_acc: efc_node_send_ls_acc,
    pub ls_acc_io: *mut c_void,
    pub ls_acc_oxid: u32,
    pub ls_acc_did: u32,
    pub shutdown_reason: efc_node_shutd_rsn,
    pub targ: bool,
    pub init: bool,
    pub refound: bool,
    pub sparm_dma_buf: efc_dma,
    pub service_params: [u8; EFC_SERVICE_PARMS_LENGTH],
    pub pend_frames_lock: spinlock_t,
    pub pend_frames: list_head,
    pub pend_frames_processed: u32,
    pub ox_id_in_use: u32,
    pub els_retries_remaining: u32,
    pub els_req_cnt: u32,
    pub els_cmpl_cnt: u32,
    pub abort_cnt: u32,
    pub current_state_name: [c_char; EFC_SM_NAME_LENGTH],
    pub prev_state_name: [c_char; EFC_SM_NAME_LENGTH],
    pub current_evt: c_int,
    pub prev_evt: c_int,
    pub arg): *mut efc_sm_event evt, void,
    pub gidpt_delay_timer: timer_list,
    pub time_last_gidpt_msec: u64,
    pub wwnn: [c_char; EFC_WWN_LENGTH],
    pub wwpn: [c_char; EFC_WWN_LENGTH],
}

//
// NPIV port
//
// Collection of the information required to restore a virtual port across
// link events
// @wwnn:		node name
// @wwpn:		port name
// @fc_id:		port id
// @tgt_data:		target backend pointer
// @ini_data:		initiator backend pointe
// @nport:		Used to match record after attaching for update
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efc_vport {
    pub list_entry: list_head,
    pub wwnn: u64,
    pub wwpn: u64,
    pub fc_id: u32,
    pub enable_tgt: bool,
    pub enable_ini: bool,
    pub tgt_data: *mut c_void,
    pub ini_data: *mut c_void,
    pub nport: *mut efc_nport,
}

// Node SM IO Context Callback structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efc_node_cb {
    pub status: c_int,
    pub ext_status: c_int,
    pub header: *mut efc_hw_rq_buffer,
    pub payload: *mut efc_hw_rq_buffer,
    pub els_rsp: efc_dma,
// Actual length of data received
    pub rsp_len: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efc_hw_rq_buffer {
    pub rqindex: u16,
    pub dma: efc_dma,
}

//
// FC sequence object
//
// Defines a general FC sequence object
// @hw:			HW that owns this sequence
// @fcfi:		FCFI associated with sequence
// @header:		Received frame header
// @payload:		Received frame header
// @hw_priv:		HW private context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efc_hw_sequence {
    pub list_entry: list_head,
    pub hw: *mut c_void,
    pub fcfi: u8,
    pub header: *mut efc_hw_rq_buffer,
    pub payload: *mut efc_hw_rq_buffer,
    pub hw_priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efc_disc_io_type {
    EFC_DISC_IO_ELS_REQ,
    EFC_DISC_IO_ELS_RESP,
    EFC_DISC_IO_CT_REQ,
    EFC_DISC_IO_CT_RESP
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efc_io_els_params {
    pub s_id: u32,
    pub ox_id: u16,
    pub timeout: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efc_io_ct_params {
    pub r_ctl: u8,
    pub type: u8,
    pub df_ctl: u8,
    pub timeout: u8,
    pub ox_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union efc_disc_io_param {
    pub els: efc_io_els_params,
    pub ct: efc_io_ct_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efc_disc_io {
    pub /: *mut *mut efc_dma req; / send buffer,
    pub /: *mut *mut efc_dma rsp; / receive buffer,
    pub enum*/: *mut *mut efc_disc_io_type io_type; / EFC_DISC_IO_TYPE,
    pub request*/: *mut *mut u16 xmit_len; / Length of els,
    pub /: *mut *mut u16 rsp_len; / Max length of rsps to be rcvd,
    pub /: *mut *mut u32 rpi; / Registered RPI,
    pub /: *mut *mut u32 vpi; / VPI for this nport,
    pub s_id: u32,
    pub d_id: u32,
    pub /: *mut *mut bool rpi_registered; / if false, use tmp RPI,
    pub iparam: efc_disc_io_param,
}

// Return value indiacating the sequence can not be freed
pub const EFC_HW_SEQ_HOLD: c_int = 0;
// Return value indiacating the sequence can be freed
pub const EFC_HW_SEQ_FREE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libefc_function_template {
// Sport
    pub sp): *mut *mut *mut int (new_nport)(struct efc efc, struct efc_nport,
    pub sp): *mut *mut *mut void (del_nport)(struct efc efc, struct efc_nport,
// Scsi Node
    pub n): *mut *mut *mut int (scsi_new_node)(struct efc efc, struct efc_node,
    pub reason): *mut *mut *mut *mut int (scsi_del_node)(struct efc efc, struct efc_node n, int,
    pub arg): *mut *mut *mut *mut *mut int (issue_mbox_rqst)(void efct, void buf, void cb, void,
// Send ELS IO
    pub io): *mut *mut *mut int (send_els)(struct efc efc, struct efc_disc_io,
// Send BLS IO
    pub bls): *mut *mut *mut int (send_bls)(struct efc efc, u32 type, struct sli_bls_params,
// Free HW frame
    pub seq): *mut *mut *mut int (hw_seq_free)(struct efc efc, struct efc_hw_sequence,
}

pub const EFC_LOG_LIB: c_uint = 0x01;
pub const EFC_LOG_NODE: c_uint = 0x02;
pub const EFC_LOG_PORT: c_uint = 0x04;
pub const EFC_LOG_DOMAIN: c_uint = 0x08;
pub const EFC_LOG_ELS: c_uint = 0x10;
pub const EFC_LOG_DOMAIN_SM: c_uint = 0x20;
pub const EFC_LOG_SM: c_uint = 0x40;
// efc library port structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efc {
    pub base: *mut c_void,
    pub pci: *mut pci_dev,
    pub sli: *mut sli4,
    pub fcfi: u32,
    pub req_wwpn: u64,
    pub req_wwnn: u64,
    pub def_wwpn: u64,
    pub def_wwnn: u64,
    pub max_xfer_size: u64,
    pub node_pool: *mut mempool_t,
    pub node_dma_pool: *mut dma_pool,
    pub nodes_count: u32,
    pub link_status: u32,
    pub vport_list: list_head,
// lock to protect the vport list
    pub vport_lock: spinlock_t,
    pub tt: libefc_function_template,
// lock to protect the discovery library.
// Refer to efclib.c for more details.
//
    pub lock: spinlock_t,
    pub enable_ini: bool,
    pub enable_tgt: bool,
    pub log_level: u32,
    pub domain: *mut efc_domain,
    pub arg): *mut *mut *mut void (domain_free_cb)(struct efc efc, void,
    pub domain_free_cb_arg: *mut c_void,
    pub tgt_rscn_delay_msec: u64,
    pub tgt_rscn_period_msec: u64,
    pub external_loopback: bool,
    pub nodedb_mask: u32,
    pub logmask: u32,
    pub els_io_pool: *mut mempool_t,
    pub els_io_alloc_failed_count: core::sync::atomic::AtomicI32,
// hold pending frames
    pub hold_frames: bool,
// lock to protect pending frames list access
    pub pend_frames_lock: spinlock_t,
    pub pend_frames: list_head,
// count of pending frames that were processed
    pub pend_frames_processed: u32,
}

//
// EFC library registration
//
extern "C" {
    pub fn efcport_init(efc: *mut efc) -> c_int;
}
extern "C" {
    pub fn efcport_destroy(efc: *mut efc);
}
//
// EFC Domain
//
extern "C" {
    pub fn efc_domain_cb(arg: *mut c_void, event: c_int, data: *mut c_void) -> c_int;
}
//
// EFC nport
//
extern "C" {
    pub fn efc_nport_cb(arg: *mut c_void, event: c_int, data: *mut c_void);
}
extern "C" {
    pub fn efc_vport_del_all(efc: *mut efc);
}
//
// EFC Node
//
extern "C" {
    pub fn efc_remote_node_cb(arg: *mut c_void, event: c_int, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn efc_node_fcid_display(fc_id: u32, buffer: *mut c_char, buf_len: u32);
}
extern "C" {
    pub fn efc_node_post_shutdown(node: *mut efc_node, arg: *mut c_void);
}
extern "C" {
    pub fn efc_node_get_wwpn(node: *mut efc_node) -> u64;
}
//
// EFC FCP/ELS/CT interface
//
extern "C" {
    pub fn efc_dispatch_frame(efc: *mut efc, seq: *mut efc_hw_sequence);
}
//
// EFC SCSI INTERACTION LAYER
//
extern "C" {
    pub fn efc_scsi_sess_reg_complete(node: *mut efc_node, status: u32);
}
extern "C" {
    pub fn efc_scsi_del_initiator_complete(efc: *mut efc, node: *mut efc_node);
}
extern "C" {
    pub fn efc_scsi_del_target_complete(efc: *mut efc, node: *mut efc_node);
}
extern "C" {
    pub fn efc_scsi_io_list_empty(efc: *mut efc, node: *mut efc_node);
}
