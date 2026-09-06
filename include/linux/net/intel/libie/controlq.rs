//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/net/intel/libie/controlq.h
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
// Copyright (C) 2025 Intel Corporation

// Default mailbox control queue

//
// enum libie_ctlq_type - control queue type
// @LIBIE_CTLQ_TYPE_TX: basic Tx control queue
// @LIBIE_CTLQ_TYPE_RX: basic Rx control queue
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum libie_ctlq_type {
    LIBIE_CTLQ_TYPE_TX = 0,
    LIBIE_CTLQ_TYPE_RX = 1,
}

// Opcode used to send controlq message to the control plane
pub const LIBIE_CTLQ_SEND_MSG_TO_CP: c_uint = 0x801;
pub const LIBIE_CTLQ_SEND_MSG_TO_PEER: c_uint = 0x804;
pub const LIBIE_CP_TX_COPYBREAK: c_int = 128;
//
// struct libie_ctlq_ctx - contains controlq info and MMIO region info
// @mmio_info: MMIO region info structure
// @ctlqs: list that stores all the control queues
// @ctlqs_lock: lock for control queue list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libie_ctlq_ctx {
    pub mmio_info: libie_mmio_info,
    pub ctlqs: list_head,
    pub /: *mut *mut spinlock_t ctlqs_lock; / protects the ctlqs list,
}

//
// struct libie_ctlq_reg - structure representing virtual addresses of the
// controlq registers and masks
// @head: controlq head register address
// @tail: controlq tail register address
// @len: register address to write controlq length and enable bit
// @addr_high: register address to write the upper 32b of ring physical address
// @addr_low: register address to write the lower 32b of ring physical address
// @len_mask: mask to read the controlq length
// @len_ena_mask: mask to write the controlq enable bit
// @head_mask: mask to read the head value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libie_ctlq_reg {
    pub head: *mut void __iomem,
    pub tail: *mut void __iomem,
    pub len: *mut void __iomem,
    pub addr_high: *mut void __iomem,
    pub addr_low: *mut void __iomem,
    pub len_mask: u32,
    pub len_ena_mask: u32,
    pub head_mask: u32,
}

//
// struct libie_cp_dma_mem - structure for DMA memory
// @va: virtual address
// @pa: physical address
// @size: memory size
// @direction: memory to device or device to memory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libie_cp_dma_mem {
    pub va: *mut c_void,
    pub pa: dma_addr_t,
    pub size: usize,
    pub direction: c_int,
}

//
// struct libie_ctlq_msg - control queue message data
// @flags: refer to 'Flags sub-structure' definitions
// @opcode: infrastructure message opcode
// @data_len: size of the payload
// @func_id: queue id for mailbox selection, 0 for default mailbox (Tx)
// @hw_retval: execution status from the HW (Rx)
// @chnl_opcode: virtchnl message opcode
// @chnl_retval: virtchnl return value
// @param0: indirect message raw parameter0
// @sw_cookie: used to verify the response of the sent virtchnl message
// @virt_flags: virtchnl capability flags
// @addr_param: additional parameters in place of the address, given no buffer
// @recv_mem: virtual address and size of the buffer that contains
// the indirect response
// @send_mem: physical and virtual address of the DMA buffer,
// used for sending
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libie_ctlq_msg {
    pub flags: u16,
    pub opcode: u16,
    pub data_len: u16,
    pub func_id: u16,
    pub hw_retval: u16,
}

//
// struct libie_ctlq_create_info - control queue create information
// @type: control queue type (Rx or Tx)
// @id: queue offset passed as input, -1 for default mailbox
// @reg: registers accessed by control queue
// @len: controlq length
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libie_ctlq_create_info {
    pub type: libie_ctlq_type,
    pub id: c_int,
    pub reg: libie_ctlq_reg,
    pub len: u16,
}

//
// struct libie_ctlq_info - control queue information
// @list: used to add a controlq to the list of queues in libie_ctlq_ctx
// @type: control queue type
// @qid: queue identifier
// @lock: control queue lock
// @ring_mem: descriptor ring DMA memory
// @descs: array of descriptors
// @rx_fqes: array of controlq Rx buffers
// @tx_msg: Tx messages sent to hardware
// @reg: registers used by control queue
// @dev: device that owns this control queue
// @pp: page pool for controlq Rx buffers
// @truesize: size to allocate per buffer
// @next_to_clean: next descriptor to be cleaned
// @next_to_use: next available slot to send buffer (Tx queue)
// @next_to_post: next available slot to post buffers to (Rx queue)
// @ring_len: length of the descriptor ring
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libie_ctlq_info {
    pub list: list_head,
    pub type: libie_ctlq_type,
    pub qid: c_int,
    pub /: *mut *mut spinlock_t lock; / for concurrent processing,
    pub ring_mem: libie_cp_dma_mem,
    pub descs: *mut libie_ctlq_desc,
    pub rx_fqes: *mut libeth_fqe,
    pub tx_msg: *mut libie_ctlq_msg,
}

// libie controlq descriptor qword0 details
// Flags sub-structure
// |0  |1  |2  |3  |4  |5  |6  |7  |8  |9  |10 |11 |12 |13 |14 |15 |
// |DD |CMP|ERR|  * RSV *  |FTYPE  | *RSV* |RD |VFC|BUF|  HOST_ID  |
//

// libie controlq descriptor qword1 details

// libie controlq descriptor qword2 details

// libie controlq descriptor qword3 details

//
// struct libie_ctlq_desc - control queue descriptor format
// @qword0: flags, message opcode, data length etc
// @qword1: virtchnl opcode, descriptor type and return value
// @qword2: indirect message parameters
// @qword3: indirect message buffer address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libie_ctlq_desc {
    pub qword0: __le64,
    pub qword1: __le64,
    pub qword2: __le64,
    pub qword3: __le64,
}

//
// struct libie_ctlq_clean_params - cleaning parameters for Tx messages
// @rel_dma_mem: non-sleeping callback to put the DMA buffer after send
// @rel_ctx: additional context for release callback
// @ctlq: control queue information
// @num_msgs: number of messages to be cleaned
// @force: clean even if DD is not yet set, use only for final cleanup
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libie_ctlq_clean_params {
    pub dma_mem): *const *const *const void (rel_dma_mem)(void ctx, struct libie_cp_dma_mem,
    pub rel_ctx: *const c_void,
    pub ctlq: *mut libie_ctlq_info,
    pub num_msgs: u16,
    pub force: bool,
}

//
// libie_ctlq_release_rx_buf - Release Rx buffer for a specific control queue
// @rx_buf: Rx buffer to be freed
//
// Driver uses this function to post back the Rx buffer after the usage.
//
extern "C" {
    pub fn libie_ctlq_deinit(ctx: *mut libie_ctlq_ctx);
}
extern "C" {
    pub fn libie_ctlq_send_desc_avail(ctlq: *const libie_ctlq_info) -> u32;
}
extern "C" {
    pub fn libie_ctlq_send(ctlq: *mut libie_ctlq_info, num_q_msg: u32);
}
extern "C" {
    pub fn libie_ctlq_send_clean(params: *const libie_ctlq_clean_params) -> u32;
}
extern "C" {
    pub fn libie_ctlq_post_rx_buffs(ctlq: *mut libie_ctlq_info) -> c_int;
}
// Only 8 bits are available in descriptor for Xn index
pub const LIBIE_CTLQ_MAX_XN_ENTRIES: c_int = 256;

//
// enum libie_ctlq_xn_state - Transaction state of a virtchnl message
// @LIBIE_CTLQ_XN_IDLE: transaction is available to use
// @LIBIE_CTLQ_XN_WAITING: waiting for transaction to complete
// @LIBIE_CTLQ_XN_COMPLETED_SUCCESS: transaction completed with success
// @LIBIE_CTLQ_XN_COMPLETED_FAILED: transaction completed with failure
// @LIBIE_CTLQ_XN_ASYNC: asynchronous virtchnl message transaction type
// @LIBIE_CTLQ_XN_SHUTDOWN: transaction cannot be used anymore
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum libie_ctlq_xn_state {
    LIBIE_CTLQ_XN_IDLE = 0,
    LIBIE_CTLQ_XN_WAITING,
    LIBIE_CTLQ_XN_COMPLETED_SUCCESS,
    LIBIE_CTLQ_XN_COMPLETED_FAILED,
    LIBIE_CTLQ_XN_ASYNC,
    LIBIE_CTLQ_XN_SHUTDOWN,
}

//
// struct libie_ctlq_xn - structure representing a virtchnl transaction entry
// @resp_cb: non-sleeping callback to handle the response to an async message
// @xn_lock: lock to protect the transaction entry state
// @cmd_completion_event: wait until reply is received or xn is terminated
// @small_dma_mem: DMA memory for copying small send buffers from stack,
// is recycled when response is received or on timeout
// @send_dma_mem: DMA memory of send buffer
// @recv_mem: receive buffer
// @send_ctx: context for callback function
// @timeout_ms: Xn transaction timeout in msecs
// @timestamp: timestamp to record the Xn send
// @tx_msg: control queue Tx message slot to track small DMA usage
// @virtchnl_opcode: virtchnl command opcode used for Xn transaction
// @state: transaction state of a virtchnl message
// @cookie: unique message identifier, incremented every time the slot is used
// @index: index of the transaction entry
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libie_ctlq_xn {
    pub status): *mut *mut *mut *mut void (resp_cb)(void ctx, struct kvec mem, int,
    pub /: *mut *mut spinlock_t xn_lock; / protects state,
    pub cmd_completion_event: completion,
    pub small_dma_mem: libie_cp_dma_mem,
    pub send_dma_mem: libie_cp_dma_mem,
    pub recv_mem: kvec,
    pub send_ctx: *mut c_void,
    pub timeout_ms: u64,
    pub timestamp: ktime_t,
    pub tx_msg: *mut libie_ctlq_msg,
    pub virtchnl_opcode: u32,
    pub state: libie_ctlq_xn_state,
    pub cookie: u8,
    pub index: u8,
}

//
// struct libie_ctlq_xn_manager - structure representing the array of virtchnl
// transaction entries
// @ctx: pointer to controlq context structure
// @free_xns_bm_lock: lock to protect the free Xn entries bit map
// @free_xns_bm: bitmap that represents the free Xn entries
// @ring: array of Xn entries
// @small_buff_pool: DMA pool for small send buffers
// @can_destroy: completion, triggered by the last released transaction
// @shutdown: shutdown process has been started, no new transactions allowed
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libie_ctlq_xn_manager {
    pub ctx: *mut libie_ctlq_ctx,
    pub /: *mut *mut spinlock_t free_xns_bm_lock; / get/check entries,
    pub LIBIE_CTLQ_MAX_XN_ENTRIES): DECLARE_BITMAP(free_xns_bm,,
    pub ring: [libie_ctlq_xn; LIBIE_CTLQ_MAX_XN_ENTRIES],
    pub small_buff_pool: *mut dma_pool,
    pub can_destroy: completion,
    pub shutdown: bool,
}

//
// struct libie_ctlq_xn_send_params - structure representing send Xn entry
// @resp_cb: non-sleeping callback to handle the response to an async message
// @rel_tx_buf: non-sleeping callback for freeing the send buffer
// @xnm: Xn manager to process Xn entries
// @ctlq: send control queue information
// @ctlq_msg: control queue message information
// @send_buf: buffer that carries outgoing message data, buffers larger than
// LIBIE_CP_TX_COPYBREAK bytes will always be consumed
// @recv_mem: receive buffer
// @send_ctx: context for callback function
// @timeout_ms: virtchnl transaction timeout in msecs
// @chnl_opcode: virtchnl message opcode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libie_ctlq_xn_send_params {
    pub status): *mut *mut *mut *mut void (resp_cb)(void ctx, struct kvec mem, int,
    pub buf_va): *const *const void (rel_tx_buf)(void,
    pub xnm: *mut libie_ctlq_xn_manager,
    pub ctlq: *mut libie_ctlq_info,
    pub ctlq_msg: *mut libie_ctlq_msg,
    pub send_buf: kvec,
    pub recv_mem: kvec,
    pub send_ctx: *mut c_void,
    pub timeout_ms: u64,
    pub chnl_opcode: u32,
}

//
// libie_cp_can_send_onstack - can a message be sent using a stack variable
// @size: ctlq data buffer size
//
// Return: %true if the message size is small enough for caller to pass
// an on-stack buffer, %false if kmalloc is needed
//
// struct libie_ctlq_xn_recv_params - request to receive xn responses
// @ctlq_msg_handler: handler for Rx messages with no matching xn (mandatory)
// @xnm: Xn manager to process Xn entries
// @ctlq: control queue information
// @budget: maximum number of messages to process
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libie_ctlq_xn_recv_params {
    pub msg): *mut libie_ctlq_msg,
    pub xnm: *mut libie_ctlq_xn_manager,
    pub ctlq: *mut libie_ctlq_info,
    pub budget: u32,
}

//
// struct libie_ctlq_xn_init_params - xn transaction manager parameters
// @cctlq_info: control queue information
// @ctx: pointer to controlq context structure
// @xnm: Xn manager to process Xn entries
// @num_qs: number of control queues to be initialized
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libie_ctlq_xn_init_params {
    pub cctlq_info: *mut libie_ctlq_create_info,
    pub ctx: *mut libie_ctlq_ctx,
    pub xnm: *mut libie_ctlq_xn_manager,
    pub num_qs: u32,
}

extern "C" {
    pub fn libie_ctlq_xn_init(params: *mut libie_ctlq_xn_init_params) -> c_int;
}
extern "C" {
    pub fn libie_ctlq_xn_shutdown(xnm: *mut libie_ctlq_xn_manager);
}
extern "C" {
    pub fn libie_ctlq_xn_send(params: *mut libie_ctlq_xn_send_params) -> c_int;
}
extern "C" {
    pub fn libie_ctlq_xn_recv(params: *mut libie_ctlq_xn_recv_params) -> u32;
}
