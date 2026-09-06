//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pds/pds_adminq.h
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
// Copyright(c) 2023 Advanced Micro Devices, Inc

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pds_core_adminq_flags {
    PDS_AQ_FLAG_FASTPOLL	= BIT(1),	/* completion poll at 1ms */
}

//
// enum pds_core_adminq_opcode - AdminQ command opcodes
// These commands are only processed on AdminQ, not available in devcmd
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pds_core_adminq_opcode {
    PDS_AQ_CMD_NOP			= 0,

// Client control
    PDS_AQ_CMD_CLIENT_REG		= 6,
    PDS_AQ_CMD_CLIENT_UNREG		= 7,
    PDS_AQ_CMD_CLIENT_CMD		= 8,

// LIF commands
    PDS_AQ_CMD_LIF_IDENTIFY		= 20,
    PDS_AQ_CMD_LIF_INIT		= 21,
    PDS_AQ_CMD_LIF_RESET		= 22,
    PDS_AQ_CMD_LIF_GETATTR		= 23,
    PDS_AQ_CMD_LIF_SETATTR		= 24,
    PDS_AQ_CMD_LIF_SETPHC		= 25,

    PDS_AQ_CMD_RX_MODE_SET		= 30,
    PDS_AQ_CMD_RX_FILTER_ADD	= 31,
    PDS_AQ_CMD_RX_FILTER_DEL	= 32,

// Queue commands
    PDS_AQ_CMD_Q_IDENTIFY		= 39,
    PDS_AQ_CMD_Q_INIT		= 40,
    PDS_AQ_CMD_Q_CONTROL		= 41,

// SR/IOV commands
    PDS_AQ_CMD_VF_GETATTR		= 60,
    PDS_AQ_CMD_VF_SETATTR		= 61,
}

//
// enum pds_core_notifyq_opcode - NotifyQ event codes
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pds_core_notifyq_opcode {
    PDS_EVENT_LINK_CHANGE		= 1,
    PDS_EVENT_RESET			= 2,
    PDS_EVENT_XCVR			= 5,
    PDS_EVENT_CLIENT		= 6,
}

pub const PDS_COMP_COLOR_MASK: c_uint = 0x80;
//
// struct pds_core_notifyq_event - Generic event reporting structure
// @eid:   event number
// @ecode: event code
//
// This is the generic event report struct from which the other
// actual events will be formed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_notifyq_event {
    pub eid: __le64,
    pub ecode: __le16,
}

//
// struct pds_core_link_change_event - Link change event notification
// @eid:		event number
// @ecode:		event code = PDS_EVENT_LINK_CHANGE
// @link_status:	link up/down, with error bits
// @link_speed:		speed of the network link
//
// Sent when the network link state changes between UP and DOWN
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_link_change_event {
    pub eid: __le64,
    pub ecode: __le16,
    pub link_status: __le16,
    pub /: *mut *mut __le32 link_speed; / units of 1Mbps: e.g. 10000 = 10Gbps,
}

//
// struct pds_core_reset_event - Reset event notification
// @eid:		event number
// @ecode:		event code = PDS_EVENT_RESET
// @reset_code:		reset type
// @state:		0=pending, 1=complete, 2=error
//
// Sent when the NIC or some subsystem is going to be or
// has been reset.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_reset_event {
    pub eid: __le64,
    pub ecode: __le16,
    pub reset_code: u8,
    pub state: u8,
}

//
// struct pds_core_client_event - Client event notification
// @eid:		event number
// @ecode:		event code = PDS_EVENT_CLIENT
// @client_id:          client to sent event to
// @client_event:       wrapped event struct for the client
//
// Sent when an event needs to be passed on to a client
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_client_event {
    pub eid: __le64,
    pub ecode: __le16,
    pub client_id: __le16,
    pub client_event: [u8; 54],
}

//
// struct pds_core_notifyq_cmd - Placeholder for building qcq
// @data:      anonymous field for building the qcq
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_notifyq_cmd {
    pub /: *mut *mut __le32 data; / Not used but needed for qcq structure,
}

//
// union pds_core_notifyq_comp - Overlay of notifyq event structures
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union pds_core_notifyq_comp {
    pub eid: __le64,
    pub ecode: __le16,
}

pub const PDS_DEVNAME_LEN: c_int = 32;
//
// struct pds_core_client_reg_cmd - Register a new client with DSC
// @opcode:         opcode PDS_AQ_CMD_CLIENT_REG
// @rsvd:           word boundary padding
// @devname:        text name of client device
// @vif_type:       what type of device (enum pds_core_vif_types)
//
// Tell the DSC of the new client, and receive a client_id from DSC.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_client_reg_cmd {
    pub opcode: u8,
    pub rsvd: [u8; 3],
    pub devname: [c_char; PDS_DEVNAME_LEN],
    pub vif_type: u8,
}

//
// struct pds_core_client_reg_comp - Client registration completion
// @status:     Status of the command (enum pdc_core_status_code)
// @rsvd:       Word boundary padding
// @comp_index: Index in the descriptor ring for which this is the completion
// @client_id:  New id assigned by DSC
// @rsvd1:      Word boundary padding
// @color:      Color bit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_client_reg_comp {
    pub status: u8,
    pub rsvd: u8,
    pub comp_index: __le16,
    pub client_id: __le16,
    pub rsvd1: [u8; 9],
    pub color: u8,
}

//
// struct pds_core_client_unreg_cmd - Unregister a client from DSC
// @opcode:     opcode PDS_AQ_CMD_CLIENT_UNREG
// @rsvd:       word boundary padding
// @client_id:  id of client being removed
//
// Tell the DSC this client is going away and remove its context
// This uses the generic completion.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_client_unreg_cmd {
    pub opcode: u8,
    pub rsvd: u8,
    pub client_id: __le16,
}

//
// struct pds_core_client_request_cmd - Pass along a wrapped client AdminQ cmd
// @opcode:     opcode PDS_AQ_CMD_CLIENT_CMD
// @rsvd:       word boundary padding
// @client_id:  id of client being removed
// @client_cmd: the wrapped client command
//
// Proxy post an adminq command for the client.
// This uses the generic completion.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_client_request_cmd {
    pub opcode: u8,
    pub rsvd: u8,
    pub client_id: __le16,
    pub client_cmd: [u8; 60],
}

pub const PDS_CORE_MAX_FRAGS: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pds_core_lif_type {
    PDS_CORE_LIF_TYPE_DEFAULT = 0,
}

pub const PDS_CORE_IFNAMSIZ: c_int = 16;
//
// enum pds_core_logical_qtype - Logical Queue Types
// @PDS_CORE_QTYPE_ADMINQ:    Administrative Queue
// @PDS_CORE_QTYPE_NOTIFYQ:   Notify Queue
// @PDS_CORE_QTYPE_RXQ:       Receive Queue
// @PDS_CORE_QTYPE_TXQ:       Transmit Queue
// @PDS_CORE_QTYPE_EQ:        Event Queue
// @PDS_CORE_QTYPE_MAX:       Max queue type supported
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pds_core_logical_qtype {
    PDS_CORE_QTYPE_ADMINQ  = 0,
    PDS_CORE_QTYPE_NOTIFYQ = 1,
    PDS_CORE_QTYPE_RXQ     = 2,
    PDS_CORE_QTYPE_TXQ     = 3,
    PDS_CORE_QTYPE_EQ      = 4,

    PDS_CORE_QTYPE_MAX     = 16   /* don't change - used in struct size */
}

//
// union pds_core_lif_config - LIF configuration
// @state:	    LIF state (enum pds_core_lif_state)
// @rsvd:           Word boundary padding
// @name:	    LIF name
// @rsvd2:          Word boundary padding
// @features:	    LIF features active (enum pds_core_hw_features)
// @queue_count:    Queue counts per queue-type
// @words:          Full union buffer size
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union pds_core_lif_config {
    pub state: u8,
    pub rsvd: [u8; 3],
    pub name: [c_char; PDS_CORE_IFNAMSIZ],
    pub rsvd2: [u8; 12],
    pub features: __le64,
    pub queue_count: [__le32; PDS_CORE_QTYPE_MAX],
    pub __packed: },
    pub words: [__le32; 64],
}

//
// struct pds_core_lif_status - LIF status register
// @eid:	     most recent NotifyQ event id
// @rsvd:            full struct size
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_lif_status {
    pub eid: __le64,
    pub rsvd: [u8; 56],
}

//
// struct pds_core_lif_info - LIF info structure
// @config:	LIF configuration structure
// @status:	LIF status structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_lif_info {
    pub config: pds_core_lif_config,
    pub status: pds_core_lif_status,
}

//
// struct pds_core_lif_identity - LIF identity information (type-specific)
// @features:		LIF features (see enum pds_core_hw_features)
// @version:		Identify structure version
// @hw_index:		LIF hardware index
// @rsvd:		Word boundary padding
// @max_nb_sessions:	Maximum number of sessions supported
// @rsvd2:		buffer padding
// @config:		LIF config struct with features, q counts
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_lif_identity {
    pub features: __le64,
    pub version: u8,
    pub hw_index: u8,
    pub rsvd: [u8; 2],
    pub max_nb_sessions: __le32,
    pub rsvd2: [u8; 120],
    pub config: pds_core_lif_config,
}

//
// struct pds_core_lif_identify_cmd - Get LIF identity info command
// @opcode:	Opcode PDS_AQ_CMD_LIF_IDENTIFY
// @type:	LIF type (enum pds_core_lif_type)
// @client_id:	Client identifier
// @ver:	Version of identify returned by device
// @rsvd:       Word boundary padding
// @ident_pa:	DMA address to receive identity info
//
// Firmware will copy LIF identity data (struct pds_core_lif_identity)
// into the buffer address given.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_lif_identify_cmd {
    pub opcode: u8,
    pub type: u8,
    pub client_id: __le16,
    pub ver: u8,
    pub rsvd: [u8; 3],
    pub ident_pa: __le64,
}

//
// struct pds_core_lif_identify_comp - LIF identify command completion
// @status:	Status of the command (enum pds_core_status_code)
// @ver:	Version of identify returned by device
// @bytes:	Bytes copied into the buffer
// @rsvd:       Word boundary padding
// @color:      Color bit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_lif_identify_comp {
    pub status: u8,
    pub ver: u8,
    pub bytes: __le16,
    pub rsvd: [u8; 11],
    pub color: u8,
}

//
// struct pds_core_lif_init_cmd - LIF init command
// @opcode:	Opcode PDS_AQ_CMD_LIF_INIT
// @type:	LIF type (enum pds_core_lif_type)
// @client_id:	Client identifier
// @rsvd:       Word boundary padding
// @info_pa:	Destination address for LIF info (struct pds_core_lif_info)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_lif_init_cmd {
    pub opcode: u8,
    pub type: u8,
    pub client_id: __le16,
    pub rsvd: __le32,
    pub info_pa: __le64,
}

//
// struct pds_core_lif_init_comp - LIF init command completion
// @status:	Status of the command (enum pds_core_status_code)
// @rsvd:       Word boundary padding
// @hw_index:	Hardware index of the initialized LIF
// @rsvd1:      Word boundary padding
// @color:      Color bit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_lif_init_comp {
    pub status: u8,
    pub rsvd: u8,
    pub hw_index: __le16,
    pub rsvd1: [u8; 11],
    pub color: u8,
}

//
// struct pds_core_lif_reset_cmd - LIF reset command
// Will reset only the specified LIF.
// @opcode:	Opcode PDS_AQ_CMD_LIF_RESET
// @rsvd:       Word boundary padding
// @client_id:	Client identifier
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_lif_reset_cmd {
    pub opcode: u8,
    pub rsvd: u8,
    pub client_id: __le16,
}

//
// enum pds_core_lif_attr - List of LIF attributes
// @PDS_CORE_LIF_ATTR_STATE:		LIF state attribute
// @PDS_CORE_LIF_ATTR_NAME:		LIF name attribute
// @PDS_CORE_LIF_ATTR_FEATURES:		LIF features attribute
// @PDS_CORE_LIF_ATTR_STATS_CTRL:	LIF statistics control attribute
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pds_core_lif_attr {
    PDS_CORE_LIF_ATTR_STATE		= 0,
    PDS_CORE_LIF_ATTR_NAME		= 1,
    PDS_CORE_LIF_ATTR_FEATURES	= 4,
    PDS_CORE_LIF_ATTR_STATS_CTRL	= 6,
}

//
// struct pds_core_lif_setattr_cmd - Set LIF attributes on the NIC
// @opcode:	Opcode PDS_AQ_CMD_LIF_SETATTR
// @attr:	Attribute type (enum pds_core_lif_attr)
// @client_id:	Client identifier
// @state:	LIF state (enum pds_core_lif_state)
// @name:	The name string, 0 terminated
// @features:	Features (enum pds_core_hw_features)
// @stats_ctl:	Stats control commands (enum pds_core_stats_ctl_cmd)
// @rsvd:       Command Buffer padding
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_lif_setattr_cmd {
    pub opcode: u8,
    pub attr: u8,
    pub client_id: __le16,
    pub state: u8,
    pub name: [c_char; PDS_CORE_IFNAMSIZ],
    pub features: __le64,
    pub stats_ctl: u8,
    pub rsvd: [u8; 60],
    pub __packed: },
}

//
// struct pds_core_lif_setattr_comp - LIF set attr command completion
// @status:	Status of the command (enum pds_core_status_code)
// @rsvd:       Word boundary padding
// @comp_index: Index in the descriptor ring for which this is the completion
// @features:	Features (enum pds_core_hw_features)
// @rsvd2:      Word boundary padding
// @color:	Color bit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_lif_setattr_comp {
    pub status: u8,
    pub rsvd: u8,
    pub comp_index: __le16,
    pub features: __le64,
    pub rsvd2: [u8; 11],
    pub __packed: },
    pub color: u8,
}

//
// struct pds_core_lif_getattr_cmd - Get LIF attributes from the NIC
// @opcode:	Opcode PDS_AQ_CMD_LIF_GETATTR
// @attr:	Attribute type (enum pds_core_lif_attr)
// @client_id:	Client identifier
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_lif_getattr_cmd {
    pub opcode: u8,
    pub attr: u8,
    pub client_id: __le16,
}

//
// struct pds_core_lif_getattr_comp - LIF get attr command completion
// @status:	Status of the command (enum pds_core_status_code)
// @rsvd:       Word boundary padding
// @comp_index: Index in the descriptor ring for which this is the completion
// @state:	LIF state (enum pds_core_lif_state)
// @features:	Features (enum pds_core_hw_features)
// @rsvd2:      Word boundary padding
// @color:	Color bit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_lif_getattr_comp {
    pub status: u8,
    pub rsvd: u8,
    pub comp_index: __le16,
    pub state: u8,
    pub features: __le64,
    pub rsvd2: [u8; 11],
    pub __packed: },
    pub color: u8,
}

//
// union pds_core_q_identity - Queue identity information
// @version:	Queue type version that can be used with FW
// @supported:	Bitfield of queue versions, first bit = ver 0
// @rsvd:       Word boundary padding
// @features:	Queue features
// @desc_sz:	Descriptor size
// @comp_sz:	Completion descriptor size
// @rsvd2:      Word boundary padding
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_q_identity {
    pub version: u8,
    pub supported: u8,
    pub rsvd: [u8; 6],
pub const PDS_CORE_QIDENT_F_CQ: c_uint = 0x01	/* queue has completion ring */;
    pub features: __le64,
    pub desc_sz: __le16,
    pub comp_sz: __le16,
    pub rsvd2: [u8; 6],
}

//
// struct pds_core_q_identify_cmd - queue identify command
// @opcode:	Opcode PDS_AQ_CMD_Q_IDENTIFY
// @type:	Logical queue type (enum pds_core_logical_qtype)
// @client_id:	Client identifier
// @ver:	Highest queue type version that the driver supports
// @rsvd:       Word boundary padding
// @ident_pa:   DMA address to receive the data (struct pds_core_q_identity)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_q_identify_cmd {
    pub opcode: u8,
    pub type: u8,
    pub client_id: __le16,
    pub ver: u8,
    pub rsvd: [u8; 3],
    pub ident_pa: __le64,
}

//
// struct pds_core_q_identify_comp - queue identify command completion
// @status:	Status of the command (enum pds_core_status_code)
// @rsvd:       Word boundary padding
// @comp_index:	Index in the descriptor ring for which this is the completion
// @ver:	Queue type version that can be used with FW
// @rsvd1:      Word boundary padding
// @color:      Color bit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_q_identify_comp {
    pub status: u8,
    pub rsvd: u8,
    pub comp_index: __le16,
    pub ver: u8,
    pub rsvd1: [u8; 10],
    pub color: u8,
}

//
// struct pds_core_q_init_cmd - Queue init command
// @opcode:	  Opcode PDS_AQ_CMD_Q_INIT
// @type:	  Logical queue type
// @client_id:	  Client identifier
// @ver:	  Queue type version
// @rsvd:         Word boundary padding
// @index:	  (LIF, qtype) relative admin queue index
// @intr_index:	  Interrupt control register index, or Event queue index
// @pid:	  Process ID
// @flags:
// IRQ:	  Interrupt requested on completion
// ENA:	  Enable the queue.  If ENA=0 the queue is initialized
// but remains disabled, to be later enabled with the
// Queue Enable command. If ENA=1, then queue is
// initialized and then enabled.
// @cos:	  Class of service for this queue
// @ring_size:	  Queue ring size, encoded as a log2(size), in
// number of descriptors.  The actual ring size is
// (1 << ring_size).  For example, to select a ring size
// of 64 descriptors write ring_size = 6. The minimum
// ring_size value is 2 for a ring of 4 descriptors.
// The maximum ring_size value is 12 for a ring of 4k
// descriptors. Values of ring_size <2 and >12 are
// reserved.
// @ring_base:	  Queue ring base address
// @cq_ring_base: Completion queue ring base address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_q_init_cmd {
    pub opcode: u8,
    pub type: u8,
    pub client_id: __le16,
    pub ver: u8,
    pub rsvd: [u8; 3],
    pub index: __le32,
    pub pid: __le16,
    pub intr_index: __le16,
    pub flags: __le16,
pub const PDS_CORE_QINIT_F_IRQ: c_uint = 0x01	/* Request interrupt on completion */;
pub const PDS_CORE_QINIT_F_ENA: c_uint = 0x02	/* Enable the queue */;
    pub cos: u8,
pub const PDS_CORE_QSIZE_MIN_LG2: c_int = 2;
pub const PDS_CORE_QSIZE_MAX_LG2: c_int = 12;
    pub ring_size: u8,
    pub ring_base: __le64,
    pub cq_ring_base: __le64,
    pub __packed: },
//
// struct pds_core_q_init_comp - Queue init command completion
// @status:	Status of the command (enum pds_core_status_code)
// @rsvd:       Word boundary padding
// @comp_index:	Index in the descriptor ring for which this is the completion
// @hw_index:	Hardware Queue ID
// @hw_type:	Hardware Queue type
// @rsvd2:      Word boundary padding
// @color:	Color
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_q_init_comp {
    pub status: u8,
    pub rsvd: u8,
    pub comp_index: __le16,
    pub hw_index: __le32,
    pub hw_type: u8,
    pub rsvd2: [u8; 6],
    pub color: u8,
}

//
// enum pds_vdpa_cmd_opcode - vDPA Device commands
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pds_vdpa_cmd_opcode {
    PDS_VDPA_CMD_INIT		= 48,
    PDS_VDPA_CMD_IDENT		= 49,
    PDS_VDPA_CMD_RESET		= 51,
    PDS_VDPA_CMD_VQ_RESET		= 52,
    PDS_VDPA_CMD_VQ_INIT		= 53,
    PDS_VDPA_CMD_STATUS_UPDATE	= 54,
    PDS_VDPA_CMD_SET_FEATURES	= 55,
    PDS_VDPA_CMD_SET_ATTR		= 56,
}

//
// struct pds_vdpa_cmd - generic command
// @opcode:	Opcode
// @vdpa_index:	Index for vdpa subdevice
// @vf_id:	VF id
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_vdpa_cmd {
    pub opcode: u8,
    pub vdpa_index: u8,
    pub vf_id: __le16,
}

//
// struct pds_vdpa_init_cmd - INIT command
// @opcode:	Opcode PDS_VDPA_CMD_INIT
// @vdpa_index: Index for vdpa subdevice
// @vf_id:	VF id
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_vdpa_init_cmd {
    pub opcode: u8,
    pub vdpa_index: u8,
    pub vf_id: __le16,
}

//
// struct pds_vdpa_ident - vDPA identification data
// @hw_features:	vDPA features supported by device
// @max_vqs:		max queues available (2 queues for a single queuepair)
// @max_qlen:		log(2) of maximum number of descriptors
// @min_qlen:		log(2) of minimum number of descriptors
//
// This struct is used in a DMA block that is set up for the PDS_VDPA_CMD_IDENT
// transaction.  Set up the DMA block and send the address in the IDENT cmd
// data, the DSC will write the ident information, then we can remove the DMA
// block after reading the answer.  If the completion status is 0, then there
// is valid information, else there was an error and the data should be invalid.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_vdpa_ident {
    pub hw_features: __le64,
    pub max_vqs: __le16,
    pub max_qlen: __le16,
    pub min_qlen: __le16,
}

//
// struct pds_vdpa_ident_cmd - IDENT command
// @opcode:	Opcode PDS_VDPA_CMD_IDENT
// @rsvd:       Word boundary padding
// @vf_id:	VF id
// @len:	length of ident info DMA space
// @ident_pa:	address for DMA of ident info (struct pds_vdpa_ident)
// only used for this transaction, then forgotten by DSC
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_vdpa_ident_cmd {
    pub opcode: u8,
    pub rsvd: u8,
    pub vf_id: __le16,
    pub len: __le32,
    pub ident_pa: __le64,
}

//
// struct pds_vdpa_status_cmd - STATUS_UPDATE command
// @opcode:	Opcode PDS_VDPA_CMD_STATUS_UPDATE
// @vdpa_index: Index for vdpa subdevice
// @vf_id:	VF id
// @status:	new status bits
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_vdpa_status_cmd {
    pub opcode: u8,
    pub vdpa_index: u8,
    pub vf_id: __le16,
    pub status: u8,
}

//
// enum pds_vdpa_attr - List of VDPA device attributes
// @PDS_VDPA_ATTR_MAC:          MAC address
// @PDS_VDPA_ATTR_MAX_VQ_PAIRS: Max virtqueue pairs
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pds_vdpa_attr {
    PDS_VDPA_ATTR_MAC          = 1,
    PDS_VDPA_ATTR_MAX_VQ_PAIRS = 2,
}

//
// struct pds_vdpa_setattr_cmd - SET_ATTR command
// @opcode:		Opcode PDS_VDPA_CMD_SET_ATTR
// @vdpa_index:		Index for vdpa subdevice
// @vf_id:		VF id
// @attr:		attribute to be changed (enum pds_vdpa_attr)
// @pad:		Word boundary padding
// @mac:		new mac address to be assigned as vdpa device address
// @max_vq_pairs:	new limit of virtqueue pairs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_vdpa_setattr_cmd {
    pub opcode: u8,
    pub vdpa_index: u8,
    pub vf_id: __le16,
    pub attr: u8,
    pub pad: [u8; 3],
    pub mac: [u8; 6],
    pub max_vq_pairs: __le16,
    pub __packed: },
}

//
// struct pds_vdpa_vq_init_cmd - queue init command
// @opcode: Opcode PDS_VDPA_CMD_VQ_INIT
// @vdpa_index:	Index for vdpa subdevice
// @vf_id:	VF id
// @qid:	Queue id (bit0 clear = rx, bit0 set = tx, qid=N is ctrlq)
// @len:	log(2) of max descriptor count
// @desc_addr:	DMA address of descriptor area
// @avail_addr:	DMA address of available descriptors (aka driver area)
// @used_addr:	DMA address of used descriptors (aka device area)
// @intr_index:	interrupt index
// @avail_index:	initial device position in available ring
// @used_index:	initial device position in used ring
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_vdpa_vq_init_cmd {
    pub opcode: u8,
    pub vdpa_index: u8,
    pub vf_id: __le16,
    pub qid: __le16,
    pub len: __le16,
    pub desc_addr: __le64,
    pub avail_addr: __le64,
    pub used_addr: __le64,
    pub intr_index: __le16,
    pub avail_index: __le16,
    pub used_index: __le16,
}

//
// struct pds_vdpa_vq_init_comp - queue init completion
// @status:	Status of the command (enum pds_core_status_code)
// @hw_qtype:	HW queue type, used in doorbell selection
// @hw_qindex:	HW queue index, used in doorbell selection
// @rsvd:	Word boundary padding
// @color:	Color bit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_vdpa_vq_init_comp {
    pub status: u8,
    pub hw_qtype: u8,
    pub hw_qindex: __le16,
    pub rsvd: [u8; 11],
    pub color: u8,
}

//
// struct pds_vdpa_vq_reset_cmd - queue reset command
// @opcode:	Opcode PDS_VDPA_CMD_VQ_RESET
// @vdpa_index:	Index for vdpa subdevice
// @vf_id:	VF id
// @qid:	Queue id
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_vdpa_vq_reset_cmd {
    pub opcode: u8,
    pub vdpa_index: u8,
    pub vf_id: __le16,
    pub qid: __le16,
}

//
// struct pds_vdpa_vq_reset_comp - queue reset completion
// @status:	Status of the command (enum pds_core_status_code)
// @rsvd0:	Word boundary padding
// @avail_index:	current device position in available ring
// @used_index:	current device position in used ring
// @rsvd:	Word boundary padding
// @color:	Color bit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_vdpa_vq_reset_comp {
    pub status: u8,
    pub rsvd0: u8,
    pub avail_index: __le16,
    pub used_index: __le16,
    pub rsvd: [u8; 9],
    pub color: u8,
}

//
// struct pds_vdpa_set_features_cmd - set hw features
// @opcode: Opcode PDS_VDPA_CMD_SET_FEATURES
// @vdpa_index:	Index for vdpa subdevice
// @vf_id:	VF id
// @rsvd:       Word boundary padding
// @features:	Feature bit mask
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_vdpa_set_features_cmd {
    pub opcode: u8,
    pub vdpa_index: u8,
    pub vf_id: __le16,
    pub rsvd: __le32,
    pub features: __le64,
}

pub const PDS_LM_DEVICE_STATE_LENGTH: c_int = 65536;

//
// enum pds_lm_cmd_opcode - Live Migration Device commands
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pds_lm_cmd_opcode {
    PDS_LM_CMD_HOST_VF_STATUS  = 1,

// Device state commands
    PDS_LM_CMD_STATE_SIZE	   = 16,
    PDS_LM_CMD_SUSPEND         = 18,
    PDS_LM_CMD_SUSPEND_STATUS  = 19,
    PDS_LM_CMD_RESUME          = 20,
    PDS_LM_CMD_SAVE            = 21,
    PDS_LM_CMD_RESTORE         = 22,

// Dirty page tracking commands
    PDS_LM_CMD_DIRTY_STATUS    = 32,
    PDS_LM_CMD_DIRTY_ENABLE    = 33,
    PDS_LM_CMD_DIRTY_DISABLE   = 34,
    PDS_LM_CMD_DIRTY_READ_SEQ  = 35,
    PDS_LM_CMD_DIRTY_WRITE_ACK = 36,
}

//
// struct pds_lm_cmd - generic command
// @opcode:	Opcode
// @rsvd:	Word boundary padding
// @vf_id:	VF id
// @rsvd2:	Structure padding to 60 Bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_lm_cmd {
    pub opcode: u8,
    pub rsvd: u8,
    pub vf_id: __le16,
    pub rsvd2: [u8; 56],
}

//
// struct pds_lm_state_size_cmd - STATE_SIZE command
// @opcode:	Opcode
// @rsvd:	Word boundary padding
// @vf_id:	VF id
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_lm_state_size_cmd {
    pub opcode: u8,
    pub rsvd: u8,
    pub vf_id: __le16,
}

//
// struct pds_lm_state_size_comp - STATE_SIZE command completion
// @status:		Status of the command (enum pds_core_status_code)
// @rsvd:		Word boundary padding
// @comp_index:		Index in the desc ring for which this is the completion
// @size:		Size of the device state
// @rsvd2:		Word boundary padding
// @color:		Color bit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_lm_state_size_comp {
    pub status: u8,
    pub rsvd: u8,
    pub comp_index: __le16,
    pub size: __le64,
    pub rsvd2: [u8; 11],
    pub __packed: },
    pub color: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pds_lm_suspend_resume_type {
    PDS_LM_SUSPEND_RESUME_TYPE_FULL = 0,
    PDS_LM_SUSPEND_RESUME_TYPE_P2P = 1,
}

//
// struct pds_lm_suspend_cmd - SUSPEND command
// @opcode:	Opcode PDS_LM_CMD_SUSPEND
// @rsvd:	Word boundary padding
// @vf_id:	VF id
// @type:	Type of suspend (enum pds_lm_suspend_resume_type)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_lm_suspend_cmd {
    pub opcode: u8,
    pub rsvd: u8,
    pub vf_id: __le16,
    pub type: u8,
}

//
// struct pds_lm_suspend_status_cmd - SUSPEND status command
// @opcode:	Opcode PDS_AQ_CMD_LM_SUSPEND_STATUS
// @rsvd:	Word boundary padding
// @vf_id:	VF id
// @type:	Type of suspend (enum pds_lm_suspend_resume_type)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_lm_suspend_status_cmd {
    pub opcode: u8,
    pub rsvd: u8,
    pub vf_id: __le16,
    pub type: u8,
}

//
// struct pds_lm_resume_cmd - RESUME command
// @opcode:	Opcode PDS_LM_CMD_RESUME
// @rsvd:	Word boundary padding
// @vf_id:	VF id
// @type:	Type of resume (enum pds_lm_suspend_resume_type)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_lm_resume_cmd {
    pub opcode: u8,
    pub rsvd: u8,
    pub vf_id: __le16,
    pub type: u8,
}

//
// struct pds_lm_sg_elem - Transmit scatter-gather (SG) descriptor element
// @addr:	DMA address of SG element data buffer
// @len:	Length of SG element data buffer, in bytes
// @rsvd:	Word boundary padding
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_lm_sg_elem {
    pub addr: __le64,
    pub len: __le32,
    pub rsvd: [__le16; 2],
}

//
// struct pds_lm_save_cmd - SAVE command
// @opcode:	Opcode PDS_LM_CMD_SAVE
// @rsvd:	Word boundary padding
// @vf_id:	VF id
// @rsvd2:	Word boundary padding
// @sgl_addr:	IOVA address of the SGL to dma the device state
// @num_sge:	Total number of SG elements
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_lm_save_cmd {
    pub opcode: u8,
    pub rsvd: u8,
    pub vf_id: __le16,
    pub rsvd2: [u8; 4],
    pub sgl_addr: __le64,
    pub num_sge: __le32,
    pub __packed: },
//
// struct pds_lm_restore_cmd - RESTORE command
// @opcode:	Opcode PDS_LM_CMD_RESTORE
// @rsvd:	Word boundary padding
// @vf_id:	VF id
// @rsvd2:	Word boundary padding
// @sgl_addr:	IOVA address of the SGL to dma the device state
// @num_sge:	Total number of SG elements
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_lm_restore_cmd {
    pub opcode: u8,
    pub rsvd: u8,
    pub vf_id: __le16,
    pub rsvd2: [u8; 4],
    pub sgl_addr: __le64,
    pub num_sge: __le32,
    pub __packed: },
//
// union pds_lm_dev_state - device state information
// @words:	Device state words
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union pds_lm_dev_state {
    pub sizeof(__le32)]: __le32 words[PDS_LM_DEVICE_STATE_LENGTH /,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pds_lm_host_vf_status {
    PDS_LM_STA_NONE = 0,
    PDS_LM_STA_IN_PROGRESS,
    PDS_LM_STA_MAX,
}

//
// struct pds_lm_dirty_region_info - Memory region info for STATUS and ENABLE
// @dma_base:		Base address of the DMA-contiguous memory region
// @page_count:		Number of pages in the memory region
// @page_size_log2:	Log2 page size in the memory region
// @rsvd:		Word boundary padding
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_lm_dirty_region_info {
    pub dma_base: __le64,
    pub page_count: __le32,
    pub page_size_log2: u8,
    pub rsvd: [u8; 3],
}

//
// struct pds_lm_dirty_status_cmd - DIRTY_STATUS command
// @opcode:		Opcode PDS_LM_CMD_DIRTY_STATUS
// @rsvd:		Word boundary padding
// @vf_id:		VF id
// @max_regions:	Capacity of the region info buffer
// @rsvd2:		Word boundary padding
// @regions_dma:	DMA address of the region info buffer
//
// The minimum of max_regions (from the command) and num_regions (from the
// completion) of struct pds_lm_dirty_region_info will be written to
// regions_dma.
//
// The max_regions may be zero, in which case regions_dma is ignored.  In that
// case, the completion will only report the maximum number of regions
// supported by the device, and the number of regions currently enabled.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_lm_dirty_status_cmd {
    pub opcode: u8,
    pub rsvd: u8,
    pub vf_id: __le16,
    pub max_regions: u8,
    pub rsvd2: [u8; 3],
    pub regions_dma: __le64,
    pub __packed: },
//
// enum pds_lm_dirty_bmp_type - Type of dirty page bitmap
// @PDS_LM_DIRTY_BMP_TYPE_NONE: No bitmap / disabled
// @PDS_LM_DIRTY_BMP_TYPE_SEQ_ACK: Seq/Ack bitmap representation
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pds_lm_dirty_bmp_type {
    PDS_LM_DIRTY_BMP_TYPE_NONE     = 0,
    PDS_LM_DIRTY_BMP_TYPE_SEQ_ACK  = 1,
}

//
// struct pds_lm_dirty_status_comp - STATUS command completion
// @status:		Status of the command (enum pds_core_status_code)
// @rsvd:		Word boundary padding
// @comp_index:		Index in the desc ring for which this is the completion
// @max_regions:	Maximum number of regions supported by the device
// @num_regions:	Number of regions currently enabled
// @bmp_type:		Type of dirty bitmap representation
// @rsvd2:		Word boundary padding
// @bmp_type_mask:	Mask of supported bitmap types, bit index per type
// @rsvd3:		Word boundary padding
// @color:		Color bit
//
// This completion descriptor is used for STATUS, ENABLE, and DISABLE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_lm_dirty_status_comp {
    pub status: u8,
    pub rsvd: u8,
    pub comp_index: __le16,
    pub max_regions: u8,
    pub num_regions: u8,
    pub bmp_type: u8,
    pub rsvd2: u8,
    pub bmp_type_mask: __le32,
    pub rsvd3: [u8; 3],
    pub color: u8,
}

//
// struct pds_lm_dirty_enable_cmd - DIRTY_ENABLE command
// @opcode:		Opcode PDS_LM_CMD_DIRTY_ENABLE
// @rsvd:		Word boundary padding
// @vf_id:		VF id
// @bmp_type:		Type of dirty bitmap representation
// @num_regions:	Number of entries in the region info buffer
// @rsvd2:		Word boundary padding
// @regions_dma:	DMA address of the region info buffer
//
// The num_regions must be nonzero, and less than or equal to the maximum
// number of regions supported by the device.
//
// The memory regions should not overlap.
//
// The information should be initialized by the driver.  The device may modify
// the information on successful completion, such as by size-aligning the
// number of pages in a region.
//
// The modified number of pages will be greater than or equal to the page count
// given in the enable command, and at least as coarsly aligned as the given
// value.  For example, the count might be aligned to a multiple of 64, but
// if the value is already a multiple of 128 or higher, it will not change.
// If the driver requires its own minimum alignment of the number of pages, the
// driver should account for that already in the region info of this command.
//
// This command uses struct pds_lm_dirty_status_comp for its completion.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_lm_dirty_enable_cmd {
    pub opcode: u8,
    pub rsvd: u8,
    pub vf_id: __le16,
    pub bmp_type: u8,
    pub num_regions: u8,
    pub rsvd2: [u8; 2],
    pub regions_dma: __le64,
    pub __packed: },
//
// struct pds_lm_dirty_disable_cmd - DIRTY_DISABLE command
// @opcode:	Opcode PDS_LM_CMD_DIRTY_DISABLE
// @rsvd:	Word boundary padding
// @vf_id:	VF id
//
// Dirty page tracking will be disabled.  This may be called in any state, as
// long as dirty page tracking is supported by the device, to ensure that dirty
// page tracking is disabled.
//
// This command uses struct pds_lm_dirty_status_comp for its completion.  On
// success, num_regions will be zero.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_lm_dirty_disable_cmd {
    pub opcode: u8,
    pub rsvd: u8,
    pub vf_id: __le16,
}

//
// struct pds_lm_dirty_seq_ack_cmd - DIRTY_READ_SEQ or _WRITE_ACK command
// @opcode:	Opcode PDS_LM_CMD_DIRTY_[READ_SEQ|WRITE_ACK]
// @rsvd:	Word boundary padding
// @vf_id:	VF id
// @off_bytes:	Byte offset in the bitmap
// @len_bytes:	Number of bytes to transfer
// @num_sge:	Number of DMA scatter gather elements
// @rsvd2:	Word boundary padding
// @sgl_addr:	DMA address of scatter gather list
//
// Read bytes from the SEQ bitmap, or write bytes into the ACK bitmap.
//
// This command treats the entire bitmap as a byte buffer.  It does not
// distinguish between guest memory regions.  The driver should refer to the
// number of pages in each region, according to PDS_LM_CMD_DIRTY_STATUS, to
// determine the region boundaries in the bitmap.  Each region will be
// represented by exactly the number of bits as the page count for that region,
// immediately following the last bit of the previous region.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_lm_dirty_seq_ack_cmd {
    pub opcode: u8,
    pub rsvd: u8,
    pub vf_id: __le16,
    pub off_bytes: __le32,
    pub len_bytes: __le32,
    pub num_sge: __le16,
    pub rsvd2: [u8; 2],
    pub sgl_addr: __le64,
    pub __packed: },
//
// struct pds_lm_host_vf_status_cmd - HOST_VF_STATUS command
// @opcode:	Opcode PDS_LM_CMD_HOST_VF_STATUS
// @rsvd:	Word boundary padding
// @vf_id:	VF id
// @status:	Current LM status of host VF driver (enum pds_lm_host_status)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_lm_host_vf_status_cmd {
    pub opcode: u8,
    pub rsvd: u8,
    pub vf_id: __le16,
    pub status: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pds_fwctl_cmd_opcode {
    PDS_FWCTL_CMD_IDENT = 70,
    PDS_FWCTL_CMD_RPC   = 71,
    PDS_FWCTL_CMD_QUERY = 72,
}

//
// struct pds_fwctl_cmd - Firmware control command structure
// @opcode: Opcode
// @rsvd:   Reserved
// @ep:     Endpoint identifier
// @op:     Operation identifier
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_fwctl_cmd {
    pub opcode: u8,
    pub rsvd: [u8; 3],
    pub ep: __le32,
    pub op: __le32,
    pub __packed: },
//
// struct pds_fwctl_comp - Firmware control completion structure
// @status:     Status of the firmware control operation
// @rsvd:       Reserved
// @comp_index: Completion index in little-endian format
// @rsvd2:      Reserved
// @color:      Color bit indicating the state of the completion
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_fwctl_comp {
    pub status: u8,
    pub rsvd: u8,
    pub comp_index: __le16,
    pub rsvd2: [u8; 11],
    pub color: u8,
    pub __packed: },
//
// struct pds_fwctl_ident_cmd - Firmware control identification command structure
// @opcode:   Operation code for the command
// @rsvd:     Reserved
// @version:  Interface version
// @rsvd2:    Reserved
// @len:      Length of the identification data
// @ident_pa: Physical address of the identification data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_fwctl_ident_cmd {
    pub opcode: u8,
    pub rsvd: u8,
    pub version: u8,
    pub rsvd2: u8,
    pub len: __le32,
    pub ident_pa: __le64,
    pub __packed: },
// future feature bits here
// enum pds_fwctl_features {
// };
// (compilers don't like empty enums)
//
// struct pds_fwctl_ident - Firmware control identification structure
// @features:    Supported features (enum pds_fwctl_features)
// @version:     Interface version
// @rsvd:        Reserved
// @max_req_sz:  Maximum request size
// @max_resp_sz: Maximum response size
// @max_req_sg_elems:  Maximum number of request SGs
// @max_resp_sg_elems: Maximum number of response SGs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_fwctl_ident {
    pub features: __le64,
    pub version: u8,
    pub rsvd: [u8; 3],
    pub max_req_sz: __le32,
    pub max_resp_sz: __le32,
    pub max_req_sg_elems: u8,
    pub max_resp_sg_elems: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pds_fwctl_query_entity {
    PDS_FWCTL_RPC_ROOT	= 0,
    PDS_FWCTL_RPC_ENDPOINT	= 1,
    PDS_FWCTL_RPC_OPERATION	= 2,
}

pub const PDS_FWCTL_RPC_OPCODE_CMD_SHIFT: c_int = 0;

pub const PDS_FWCTL_RPC_OPCODE_VER_SHIFT: c_int = 16;

//
// FW command attributes that map to the FWCTL scope values
//
pub const PDSFC_FW_CMD_ATTR_READ: c_uint = 0x00;
pub const PDSFC_FW_CMD_ATTR_DEBUG_READ: c_uint = 0x02;
pub const PDSFC_FW_CMD_ATTR_WRITE: c_uint = 0x04;
pub const PDSFC_FW_CMD_ATTR_DEBUG_WRITE: c_uint = 0x08;
pub const PDSFC_FW_CMD_ATTR_SYNC: c_uint = 0x10;
//
// struct pds_fwctl_query_cmd - Firmware control query command structure
// @opcode: Operation code for the command
// @entity:  Entity type to query (enum pds_fwctl_query_entity)
// @version: Version of the query data structure supported by the driver
// @rsvd:    Reserved
// @query_data_buf_len: Length of the query data buffer
// @query_data_buf_pa:  Physical address of the query data buffer
// @ep:      Endpoint identifier to query  (when entity is PDS_FWCTL_RPC_ENDPOINT)
// @op:      Operation identifier to query (when entity is PDS_FWCTL_RPC_OPERATION)
//
// This structure is used to send a query command to the firmware control
// interface. The structure is packed to ensure there is no padding between
// the fields.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_fwctl_query_cmd {
    pub opcode: u8,
    pub entity: u8,
    pub version: u8,
    pub rsvd: u8,
    pub query_data_buf_len: __le32,
    pub query_data_buf_pa: __le64,
    pub ep: __le32,
    pub op: __le32,
}

//
// struct pds_fwctl_query_comp - Firmware control query completion structure
// @status:     Status of the query command
// @rsvd:       Reserved
// @comp_index: Completion index in little-endian format
// @version:    Version of the query data structure returned by firmware. This
// should be less than or equal to the version supported by the driver
// @rsvd2:      Reserved
// @color:      Color bit indicating the state of the completion
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_fwctl_query_comp {
    pub status: u8,
    pub rsvd: u8,
    pub comp_index: __le16,
    pub version: u8,
    pub rsvd2: [u8; 2],
    pub color: u8,
    pub __packed: },
//
// struct pds_fwctl_query_data_endpoint - query data for entity PDS_FWCTL_RPC_ROOT
// @id: The identifier for the data endpoint
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_fwctl_query_data_endpoint {
    pub id: __le32,
    pub __packed: },
//
// struct pds_fwctl_query_data_operation - query data for entity PDS_FWCTL_RPC_ENDPOINT
// @id:    Operation identifier
// @scope: Scope of the operation (enum fwctl_rpc_scope)
// @rsvd:  Reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_fwctl_query_data_operation {
    pub id: __le32,
    pub scope: u8,
    pub rsvd: [u8; 3],
    pub __packed: },
//
// struct pds_fwctl_query_data - query data structure
// @version:     Version of the query data structure
// @rsvd:        Reserved
// @num_entries: Number of entries in the union
// @entries:     Array of query data entries, depending on the entity type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_fwctl_query_data {
    pub version: u8,
    pub rsvd: [u8; 3],
    pub num_entries: __le32,
    pub __counted_by_le(num_entries): u8 entries[],
    pub __packed: },
//
// struct pds_fwctl_rpc_cmd - Firmware control RPC command
// @opcode:        opcode PDS_FWCTL_CMD_RPC
// @rsvd:          Reserved
// @flags:         Indicates indirect request and/or response handling
// @ep:            Endpoint identifier
// @op:            Operation identifier
// @inline_req0:   Buffer for inline request
// @inline_req1:   Buffer for inline request
// @req_pa:        Physical address of request data
// @req_sz:        Size of the request
// @req_sg_elems:  Number of request SGs
// @req_rsvd:      Reserved
// @inline_req2:   Buffer for inline request
// @resp_pa:       Physical address of response data
// @resp_sz:       Size of the response
// @resp_sg_elems: Number of response SGs
// @resp_rsvd:     Reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_fwctl_rpc_cmd {
    pub opcode: u8,
    pub rsvd: u8,
    pub flags: __le16,
pub const PDS_FWCTL_RPC_IND_REQ: c_uint = 0x1;
pub const PDS_FWCTL_RPC_IND_RESP: c_uint = 0x2;
    pub ep: __le32,
    pub op: __le32,
    pub inline_req0: [u8; 16],
    pub inline_req1: [u8; 16],
    pub req_pa: __le64,
    pub req_sz: __le32,
    pub req_sg_elems: u8,
    pub req_rsvd: [u8; 3],
}

//
// struct pds_sg_elem - Transmit scatter-gather (SG) descriptor element
// @addr:	DMA address of SG element data buffer
// @len:	Length of SG element data buffer, in bytes
// @rsvd:	Reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_sg_elem {
    pub addr: __le64,
    pub len: __le32,
    pub rsvd: [u8; 4],
    pub __packed: },
//
// struct pds_fwctl_rpc_comp - Completion of a firmware control RPC
// @status:     Status of the command
// @rsvd:       Reserved
// @comp_index: Completion index of the command
// @err:        Error code, if any, from the RPC
// @resp_sz:    Size of the response
// @rsvd2:      Reserved
// @color:      Color bit indicating the state of the completion
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_fwctl_rpc_comp {
    pub status: u8,
    pub rsvd: u8,
    pub comp_index: __le16,
    pub err: __le32,
    pub resp_sz: __le32,
    pub rsvd2: [u8; 3],
    pub color: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union pds_core_adminq_cmd {
    pub opcode: u8,
    pub bytes: [u8; 64],
    pub client_reg: pds_core_client_reg_cmd,
    pub client_unreg: pds_core_client_unreg_cmd,
    pub client_request: pds_core_client_request_cmd,
    pub lif_ident: pds_core_lif_identify_cmd,
    pub lif_init: pds_core_lif_init_cmd,
    pub lif_reset: pds_core_lif_reset_cmd,
    pub lif_setattr: pds_core_lif_setattr_cmd,
    pub lif_getattr: pds_core_lif_getattr_cmd,
    pub q_ident: pds_core_q_identify_cmd,
    pub q_init: pds_core_q_init_cmd,
    pub vdpa: pds_vdpa_cmd,
    pub vdpa_init: pds_vdpa_init_cmd,
    pub vdpa_ident: pds_vdpa_ident_cmd,
    pub vdpa_status: pds_vdpa_status_cmd,
    pub vdpa_setattr: pds_vdpa_setattr_cmd,
    pub vdpa_set_features: pds_vdpa_set_features_cmd,
    pub vdpa_vq_init: pds_vdpa_vq_init_cmd,
    pub vdpa_vq_reset: pds_vdpa_vq_reset_cmd,
    pub lm_suspend: pds_lm_suspend_cmd,
    pub lm_suspend_status: pds_lm_suspend_status_cmd,
    pub lm_resume: pds_lm_resume_cmd,
    pub lm_state_size: pds_lm_state_size_cmd,
    pub lm_save: pds_lm_save_cmd,
    pub lm_restore: pds_lm_restore_cmd,
    pub lm_host_vf_status: pds_lm_host_vf_status_cmd,
    pub lm_dirty_status: pds_lm_dirty_status_cmd,
    pub lm_dirty_enable: pds_lm_dirty_enable_cmd,
    pub lm_dirty_disable: pds_lm_dirty_disable_cmd,
    pub lm_dirty_seq_ack: pds_lm_dirty_seq_ack_cmd,
    pub fwctl: pds_fwctl_cmd,
    pub fwctl_ident: pds_fwctl_ident_cmd,
    pub fwctl_rpc: pds_fwctl_rpc_cmd,
    pub fwctl_query: pds_fwctl_query_cmd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pds_core_adminq_comp {
    pub status: u8,
    pub rsvd: u8,
    pub comp_index: __le16,
    pub rsvd2: [u8; 11],
    pub color: u8,
}

// The color bit is a 'done' bit for the completion descriptors
// where the meaning alternates between '1' and '0' for alternating
// passes through the completion descriptor ring.
//
