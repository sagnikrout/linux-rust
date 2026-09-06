//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wwan/iosm/iosm_ipc_protocol_ops.h
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
// Copyright (C) 2020-21 Intel Corporation.
//
pub const SIZE_MASK: c_uint = 0x00FFFFFF;
pub const COMPLETION_STATUS: c_int = 24;
pub const RESET_BIT: c_int = 7;
//
// enum ipc_mem_td_cs - Completion status of a TD
// @IPC_MEM_TD_CS_INVALID:	      Initial status - td not yet used.
// @IPC_MEM_TD_CS_PARTIAL_TRANSFER:   More data pending -> next TD used for this
// @IPC_MEM_TD_CS_END_TRANSFER:	      IO transfer is complete.
// @IPC_MEM_TD_CS_OVERFLOW:	      IO transfer to small for the buff to write
// @IPC_MEM_TD_CS_ABORT:	      TD marked as abort and shall be discarded
// by AP.
// @IPC_MEM_TD_CS_ERROR:	      General error.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipc_mem_td_cs {
    IPC_MEM_TD_CS_INVALID,
    IPC_MEM_TD_CS_PARTIAL_TRANSFER,
    IPC_MEM_TD_CS_END_TRANSFER,
    IPC_MEM_TD_CS_OVERFLOW,
    IPC_MEM_TD_CS_ABORT,
    IPC_MEM_TD_CS_ERROR,
}

//
// enum ipc_mem_msg_cs - Completion status of IPC Message
// @IPC_MEM_MSG_CS_INVALID:	Initial status.
// @IPC_MEM_MSG_CS_SUCCESS:	IPC Message completion success.
// @IPC_MEM_MSG_CS_ERROR:	Message send error.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipc_mem_msg_cs {
    IPC_MEM_MSG_CS_INVALID,
    IPC_MEM_MSG_CS_SUCCESS,
    IPC_MEM_MSG_CS_ERROR,
}

//
// struct ipc_msg_prep_args_pipe - struct for pipe args for message preparation
// @pipe:	Pipe to open/close
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_msg_prep_args_pipe {
    pub pipe: *mut ipc_pipe,
}

//
// struct ipc_msg_prep_args_sleep - struct for sleep args for message
// preparation
// @target:	0=host, 1=device
// @state:	0=enter sleep, 1=exit sleep
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_msg_prep_args_sleep {
    pub target: c_uint,
    pub state: c_uint,
}

//
// struct ipc_msg_prep_feature_set - struct for feature set argument for
// message preparation
// @reset_enable:	0=out-of-band, 1=in-band-crash notification
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_msg_prep_feature_set {
    pub reset_enable: u8,
}

//
// struct ipc_msg_prep_map - struct for map argument for message preparation
// @region_id:	Region to map
// @addr:	Pcie addr of region to map
// @size:	Size of the region to map
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_msg_prep_map {
    pub region_id: c_uint,
    pub addr: c_ulong,
    pub size: usize,
}

//
// struct ipc_msg_prep_unmap - struct for unmap argument for message preparation
// @region_id:	Region to unmap
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_msg_prep_unmap {
    pub region_id: c_uint,
}

//
// struct ipc_msg_prep_args - Union to handle different message types
// @pipe_open:		Pipe open message preparation struct
// @pipe_close:		Pipe close message preparation struct
// @sleep:		Sleep message preparation struct
// @feature_set:	Feature set message preparation struct
// @map:		Memory map message preparation struct
// @unmap:		Memory unmap message preparation struct
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union ipc_msg_prep_args {
    pub pipe_open: ipc_msg_prep_args_pipe,
    pub pipe_close: ipc_msg_prep_args_pipe,
    pub sleep: ipc_msg_prep_args_sleep,
    pub feature_set: ipc_msg_prep_feature_set,
    pub map: ipc_msg_prep_map,
    pub unmap: ipc_msg_prep_unmap,
}

//
// enum ipc_msg_prep_type - Enum for message prepare actions
// @IPC_MSG_PREP_SLEEP:		Sleep message preparation type
// @IPC_MSG_PREP_PIPE_OPEN:	Pipe open message preparation type
// @IPC_MSG_PREP_PIPE_CLOSE:	Pipe close message preparation type
// @IPC_MSG_PREP_FEATURE_SET:	Feature set message preparation type
// @IPC_MSG_PREP_MAP:		Memory map message preparation type
// @IPC_MSG_PREP_UNMAP:		Memory unmap message preparation type
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipc_msg_prep_type {
    IPC_MSG_PREP_SLEEP,
    IPC_MSG_PREP_PIPE_OPEN,
    IPC_MSG_PREP_PIPE_CLOSE,
    IPC_MSG_PREP_FEATURE_SET,
    IPC_MSG_PREP_MAP,
    IPC_MSG_PREP_UNMAP,
}

//
// struct ipc_rsp - Response to sent message
// @completion:	For waking up requestor
// @status:	Completion status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_rsp {
    pub completion: completion,
    pub status: ipc_mem_msg_cs,
}

//
// enum ipc_mem_msg - Type-definition of the messages.
// @IPC_MEM_MSG_OPEN_PIPE:	AP ->CP: Open a pipe
// @IPC_MEM_MSG_CLOSE_PIPE:	AP ->CP: Close a pipe
// @IPC_MEM_MSG_ABORT_PIPE:	AP ->CP: wait for completion of the
// running transfer and abort all pending
// IO-transfers for the pipe
// @IPC_MEM_MSG_SLEEP:		AP ->CP: host enter or exit sleep
// @IPC_MEM_MSG_FEATURE_SET:	AP ->CP: Intel feature configuration
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipc_mem_msg {
    IPC_MEM_MSG_OPEN_PIPE = 0x01,
    IPC_MEM_MSG_CLOSE_PIPE = 0x02,
    IPC_MEM_MSG_ABORT_PIPE = 0x03,
    IPC_MEM_MSG_SLEEP = 0x04,
    IPC_MEM_MSG_FEATURE_SET = 0xF0,
}

//
// struct ipc_mem_msg_open_pipe - Message structure for open pipe
// @tdr_addr:			Tdr address
// @tdr_entries:		Tdr entries
// @pipe_nr:			Pipe number
// @type_of_message:		Message type
// @irq_vector:			MSI vector number
// @accumulation_backoff:	Time in usec for data accumalation
// @completion_status:		Message Completion Status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_mem_msg_open_pipe {
    pub tdr_addr: __le64,
    pub tdr_entries: __le16,
    pub pipe_nr: u8,
    pub type_of_message: u8,
    pub irq_vector: __le32,
    pub accumulation_backoff: __le32,
    pub completion_status: __le32,
}

//
// struct ipc_mem_msg_close_pipe - Message structure for close pipe
// @reserved1:			Reserved
// @reserved2:			Reserved
// @pipe_nr:			Pipe number
// @type_of_message:		Message type
// @reserved3:			Reserved
// @reserved4:			Reserved
// @completion_status:		Message Completion Status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_mem_msg_close_pipe {
    pub reserved1: [__le32; 2],
    pub reserved2: __le16,
    pub pipe_nr: u8,
    pub type_of_message: u8,
    pub reserved3: __le32,
    pub reserved4: __le32,
    pub completion_status: __le32,
}

//
// struct ipc_mem_msg_abort_pipe - Message structure for abort pipe
// @reserved1:			Reserved
// @reserved2:			Reserved
// @pipe_nr:			Pipe number
// @type_of_message:		Message type
// @reserved3:			Reserved
// @reserved4:			Reserved
// @completion_status:		Message Completion Status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_mem_msg_abort_pipe {
    pub reserved1: [__le32; 2],
    pub reserved2: __le16,
    pub pipe_nr: u8,
    pub type_of_message: u8,
    pub reserved3: __le32,
    pub reserved4: __le32,
    pub completion_status: __le32,
}

//
// struct ipc_mem_msg_host_sleep - Message structure for sleep message.
// @reserved1:		Reserved
// @target:		0=host, 1=device, host or EP devie
// is the message target
// @state:		0=enter sleep, 1=exit sleep,
// 2=enter sleep no protocol
// @reserved2:		Reserved
// @type_of_message:	Message type
// @reserved3:		Reserved
// @reserved4:		Reserved
// @completion_status:	Message Completion Status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_mem_msg_host_sleep {
    pub reserved1: [__le32; 2],
    pub target: u8,
    pub state: u8,
    pub reserved2: u8,
    pub type_of_message: u8,
    pub reserved3: __le32,
    pub reserved4: __le32,
    pub completion_status: __le32,
}

//
// struct ipc_mem_msg_feature_set - Message structure for feature_set message
// @reserved1:			Reserved
// @reserved2:			Reserved
// @reset_enable:		0=out-of-band, 1=in-band-crash notification
// @type_of_message:		Message type
// @reserved3:			Reserved
// @reserved4:			Reserved
// @completion_status:		Message Completion Status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_mem_msg_feature_set {
    pub reserved1: [__le32; 2],
    pub reserved2: __le16,
    pub reset_enable: u8,
    pub type_of_message: u8,
    pub reserved3: __le32,
    pub reserved4: __le32,
    pub completion_status: __le32,
}

//
// struct ipc_mem_msg_common - Message structure for completion status update.
// @reserved1:			Reserved
// @reserved2:			Reserved
// @type_of_message:		Message type
// @reserved3:			Reserved
// @reserved4:			Reserved
// @completion_status:		Message Completion Status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_mem_msg_common {
    pub reserved1: [__le32; 2],
    pub reserved2: [u8; 3],
    pub type_of_message: u8,
    pub reserved3: __le32,
    pub reserved4: __le32,
    pub completion_status: __le32,
}

//
// union ipc_mem_msg_entry - Union with all possible messages.
// @open_pipe:		Open pipe message struct
// @close_pipe:		Close pipe message struct
// @abort_pipe:		Abort pipe message struct
// @host_sleep:		Host sleep message struct
// @feature_set:	Featuer set message struct
// @common:		Used to access msg_type and to set the completion status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union ipc_mem_msg_entry {
    pub open_pipe: ipc_mem_msg_open_pipe,
    pub close_pipe: ipc_mem_msg_close_pipe,
    pub abort_pipe: ipc_mem_msg_abort_pipe,
    pub host_sleep: ipc_mem_msg_host_sleep,
    pub feature_set: ipc_mem_msg_feature_set,
    pub common: ipc_mem_msg_common,
}

// Transfer descriptor definition.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_protocol_td {
// 0 :  63 - 64-bit address of a buffer in host memory.
    pub address: dma_addr_t,
// 0 :  31 - 32 bit address
    pub address: __le32,
// 32 :  63 - corresponding descriptor
    pub desc: __le32,
    pub shm: } __packed,
    pub buffer: },
// 0 - 2nd byte - Size of the buffer.
// The host provides the size of the buffer queued.
// The EP device reads this value and shall update
// it for downlink transfers to indicate the
// amount of data written in buffer.
// 3rd byte - This field provides the completion status
// of the TD. When queuing the TD, the host sets
// the status to 0. The EP device updates this
// field when completing the TD.
//
    pub scs: __le32,
// 0th - nr of following descriptors
// 1 - 3rd byte - reserved
//
    pub next: __le32,
    pub __packed: },
//
// ipc_protocol_msg_prep - Prepare message based upon message type
// @ipc_imem:	iosm_protocol instance
// @msg_type:	message prepare type
// @args:	message arguments
//
// Return: 0 on success and failure value on error
//
    pub args): *mut ipc_msg_prep_args,
//
// ipc_protocol_msg_hp_update - Function for head pointer update
// of message ring
// @ipc_imem:	iosm_protocol instance
//
    pub ipc_imem): *mut void ipc_protocol_msg_hp_update(struct iosm_imem,
//
// ipc_protocol_msg_process - Function for processing responses
// to IPC messages
// @ipc_imem:	iosm_protocol instance
// @irq:	IRQ vector
//
// Return:	True on success, false if error
//
    pub irq): *mut *mut bool ipc_protocol_msg_process(struct iosm_imem ipc_imem, int,
//
// ipc_protocol_ul_td_send - Function for sending the data to CP
// @ipc_protocol:	iosm_protocol instance
// @pipe:		Pipe instance
// @p_ul_list:		uplink sk_buff list
//
// Return: true in success, false in case of error
//
    pub p_ul_list): *mut sk_buff_head,
//
// ipc_protocol_ul_td_process - Function for processing the sent data
// @ipc_protocol:	iosm_protocol instance
// @pipe:		Pipe instance
//
// Return: sk_buff instance
//
    pub pipe): *mut ipc_pipe,
//
// ipc_protocol_dl_td_prepare - Function for providing DL TDs to CP
// @ipc_protocol:	iosm_protocol instance
// @pipe:		Pipe instance
//
// Return: true in success, false in case of error
//
    pub pipe): *mut ipc_pipe,
//
// ipc_protocol_dl_td_process - Function for processing the DL data
// @ipc_protocol:	iosm_protocol instance
// @pipe:		Pipe instance
//
// Return: sk_buff instance
//
    pub pipe): *mut ipc_pipe,
//
// ipc_protocol_get_head_tail_index - Function for getting Head and Tail
// pointer index of given pipe
// @ipc_protocol:	iosm_protocol instance
// @pipe:		Pipe Instance
// @head:		head pointer index of the given pipe
// @tail:		tail pointer index of the given pipe
//
    pub tail): *mut u32,
//
// ipc_protocol_get_ipc_status - Function for getting the IPC Status
// @ipc_protocol:	iosm_protocol instance
//
// Return: Returns IPC State
//
// ipc_protocol);
//
// ipc_protocol_pipe_cleanup - Function to cleanup pipe resources
// @ipc_protocol:	iosm_protocol instance
// @pipe:		Pipe instance
//
    pub pipe): *mut ipc_pipe,
//
// ipc_protocol_get_ap_exec_stage - Function for getting AP Exec Stage
// @ipc_protocol:	pointer to struct iosm protocol
//
// Return: returns BOOT Stages
//
    pub ipc_protocol): *mut ipc_protocol_get_ap_exec_stage(struct iosm_protocol,
//
// ipc_protocol_pm_dev_get_sleep_notification - Function for getting Dev Sleep
// notification
// @ipc_protocol:	iosm_protocol instance
//
// Return: Returns dev PM State
//
// ipc_protocol);
