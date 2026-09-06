//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mailbox_controller.h
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

// Sentinel value distinguishing "no active request" from "NULL message data"

//
// struct mbox_chan_ops - methods to control mailbox channels
// @send_data:	The API asks the MBOX controller driver, in atomic
// context try to transmit a message on the bus. Returns 0 if
// data is accepted for transmission, -EBUSY while rejecting
// if the remote hasn't yet read the last data sent. Actual
// transmission of data is reported by the controller via
// mbox_chan_txdone (if it has some TX ACK irq). It must not
// sleep.
// @flush:	Called when a client requests transmissions to be blocking but
// the context doesn't allow sleeping. Typically the controller
// will implement a busy loop waiting for the data to flush out.
// @startup:	Called when a client requests the chan. The controller
// could ask clients for additional parameters of communication
// to be provided via client's chan_data. This call may
// block. After this call the Controller must forward any
// data received on the chan by calling mbox_chan_received_data.
// The controller may do stuff that need to sleep.
// @shutdown:	Called when a client relinquishes control of a chan.
// This call may block too. The controller must not forward
// any received data anymore.
// The controller may do stuff that need to sleep.
// @last_tx_done: If the controller sets 'txdone_poll', the API calls
// this to poll status of last TX. The controller must
// give priority to IRQ method over polling and never
// set both txdone_poll and txdone_irq. Only in polling
// mode 'send_data' is expected to return -EBUSY.
// The controller may do stuff that need to sleep/block.
// Used only if txdone_poll:=true && txdone_irq:=false
// @peek_data: Atomic check for any received data. Return true if controller
// has some data to push to the client. False otherwise.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbox_chan_ops {
    pub data): *mut *mut *mut int (send_data)(struct mbox_chan chan, void,
    pub timeout): *mut *mut *mut int (flush)(struct mbox_chan chan, unsigned long,
    pub chan): *mut *mut int (startup)(struct mbox_chan,
    pub chan): *mut *mut void (shutdown)(struct mbox_chan,
    pub chan): *mut *mut bool (last_tx_done)(struct mbox_chan,
    pub chan): *mut *mut bool (peek_data)(struct mbox_chan,
}

//
// struct mbox_controller - Controller of a class of communication channels
// @dev:		Device backing this controller. Required.
// @ops:		Operators that work on each communication chan. Required.
// @chans:		Array of channels. Required.
// @num_chans:		Number of channels in the 'chans' array. Required.
// @txdone_irq:		Indicates if the controller can report to API when
// the last transmitted data was read by the remote.
// Eg, if it has some TX ACK irq.
// @txdone_poll:	If the controller can read but not report the TX
// done. Ex, some register shows the TX status but
// no interrupt rises. Ignored if 'txdone_irq' is set.
// @txpoll_period:	If 'txdone_poll' is in effect, the API polls for
// last TX's status after these many millisecs
// @fw_xlate:		Controller driver specific mapping of channel via fwnode
// @of_xlate:		Controller driver specific mapping of channel via DT
// @poll_hrt:		API private. hrtimer used to poll for TXDONE on all
// channels.
// @poll_hrt_lock:	API private. Lock protecting access to poll_hrt.
// @node:		API private. To hook into list of controllers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbox_controller {
    pub dev: *mut device,
    pub ops: *const mbox_chan_ops,
    pub chans: *mut mbox_chan,
    pub num_chans: c_int,
    pub txdone_irq: bool,
    pub txdone_poll: bool,
    pub txpoll_period: unsigned,
    pub sp): *const fwnode_reference_args,
    pub sp): *const of_phandle_args,
// Internal to API
    pub poll_hrt: hrtimer,
    pub poll_hrt_lock: spinlock_t,
    pub node: list_head,
}

//
// The length of circular buffer for queuing messages from a client.
// 'msg_count' tracks the number of buffered messages while 'msg_free'
// is the index where the next message would be buffered.
// We shouldn't need it too big because every transfer is interrupt
// triggered and if we have lots of data to transfer, the interrupt
// latencies are going to be the bottleneck, not the buffer length.
// Besides, mbox_send_message could be called from atomic context and
// the client could also queue another message from the notifier 'tx_done'
// of the last transfer done.
// REVISIT: If too many platforms see the "Try increasing MBOX_TX_QUEUE_LEN"
// print, it needs to be taken from config option or somesuch.
//
pub const MBOX_TX_QUEUE_LEN: c_int = 20;
//
// struct mbox_chan - s/w representation of a communication chan
// @mbox:		Pointer to the parent/provider of this channel
// @txdone_method:	Way to detect TXDone chosen by the API
// @cl:			Pointer to the current owner of this channel
// @tx_complete:	Transmission completion
// @tx_status:		Transmission status
// @active_req:		Currently active request hook
// @msg_count:		No. of mssg currently queued
// @msg_free:		Index of next available mssg slot
// @msg_data:		Hook for data packet
// @lock:		Serialise access to the channel
// @con_priv:		Hook for controller driver to attach private data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbox_chan {
    pub mbox: *mut mbox_controller,
    pub txdone_method: unsigned,
    pub cl: *mut mbox_client,
    pub tx_complete: completion,
    pub tx_status: c_int,
    pub active_req: *mut c_void,
    pub msg_free: unsigned msg_count,,
    pub msg_data: [*mut c_void; MBOX_TX_QUEUE_LEN],
    pub /: *mut *mut spinlock_t lock; / Serialise access to the channel,
    pub con_priv: *mut c_void,
}
