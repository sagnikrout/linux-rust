//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/async_tx.h
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
// Copyright © 2006, Intel Corporation.
//

// on architectures without dma-mapping capabilities we need to ensure
// that the asynchronous path compiles away
//

// Macro flag: #define __async_inline

//
// dma_chan_ref - object used to manage dma channels received from the
// dmaengine core.
// @chan - the channel being tracked
// @node - node for the channel to be placed on async_tx_master_list
// @rcu - for list_del_rcu
// @count - number of times this channel is listed in the pool
// (for channels with multiple capabiities)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_chan_ref {
    pub chan: *mut dma_chan,
    pub node: list_head,
    pub rcu: rcu_head,
    pub count: core::sync::atomic::AtomicI32,
}

//
// async_tx_flags - modifiers for the async_* calls
// @ASYNC_TX_XOR_ZERO_DST: this flag must be used for xor operations where the
// destination address is not a source.  The asynchronous case handles this
// implicitly, the synchronous case needs to zero the destination block.
// @ASYNC_TX_XOR_DROP_DST: this flag must be used if the destination address is
// also one of the source addresses.  In the synchronous case the destination
// address is an implied source, whereas the asynchronous case it must be listed
// as a source.  The destination address must be the first address in the source
// array.
// @ASYNC_TX_ACK: immediately ack the descriptor, precludes setting up a
// dependency chain
// @ASYNC_TX_FENCE: specify that the next operation in the dependency
// chain uses this operation's result as an input
// @ASYNC_TX_PQ_XOR_DST: do not overwrite the syndrome but XOR it with the
// input data. Required for rmw case.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum async_tx_flags {
    ASYNC_TX_XOR_ZERO_DST	 = (1 << 0),
    ASYNC_TX_XOR_DROP_DST	 = (1 << 1),
    ASYNC_TX_ACK		 = (1 << 2),
    ASYNC_TX_FENCE		 = (1 << 3),
    ASYNC_TX_PQ_XOR_DST	 = (1 << 4),
}

//
// struct async_submit_ctl - async_tx submission/completion modifiers
// @flags: submission modifiers
// @depend_tx: parent dependency of the current operation being submitted
// @cb_fn: callback routine to run at operation completion
// @cb_param: parameter for the callback routine
// @scribble: caller provided space for dma/page address conversions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct async_submit_ctl {
    pub flags: async_tx_flags,
    pub depend_tx: *mut dma_async_tx_descriptor,
    pub cb_fn: dma_async_tx_callback,
    pub cb_param: *mut c_void,
    pub scribble: *mut c_void,
}

//
// async_tx_issue_pending - send pending descriptor to the hardware channel
// @tx: descriptor handle to retrieve hardware context
//
// Note: any dependent operations will have already been issued by
// async_tx_channel_switch, or (in the case of no channel switch) will
// be already pending on this channel.
//

//
// async_tx_sync_epilog - actions to take if an operation is run synchronously
// @cb_fn: function to call when the transaction completes
// @cb_fn_param: parameter to pass to the callback routine
//
extern "C" {
    pub fn async_tx_quiesce(tx: *mut dma_async_tx_descriptor);
}
