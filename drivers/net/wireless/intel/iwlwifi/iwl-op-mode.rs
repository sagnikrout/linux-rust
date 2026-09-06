//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/iwl-op-mode.h
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
// Copyright (C) 2005-2014, 2018-2021, 2024-2025 Intel Corporation
// Copyright (C) 2013-2014 Intel Mobile Communications GmbH
// Copyright (C) 2015 Intel Deutschland GmbH
//

// Macro flag: #define __iwl_op_mode_h__

//
// DOC: Operational mode - what is it ?
//
// The operational mode (a.k.a. op_mode) is the layer that implements
// mac80211's handlers. It knows two APIs: mac80211's and the fw's. It uses
// the transport API to access the HW. The op_mode doesn't need to know how the
// underlying HW works, since the transport layer takes care of that.
//
// There can be several op_mode: i.e. different fw APIs will require two
// different op_modes. This is why the op_mode is virtualized.
//
// DOC: Life cycle of the Operational mode
//
// The operational mode has a very simple life cycle.
//
// 1) The driver layer (iwl-drv.c) chooses the op_mode based on the
// capabilities advertised by the fw file (in TLV format).
// 2) The driver layer starts the op_mode (ops->start)
// 3) The op_mode registers mac80211
// 4) The op_mode is governed by mac80211
// 5) The driver layer stops the op_mode
//
// enum iwl_fw_error_type - FW error types/sources
// @IWL_ERR_TYPE_IRQ: "normal" FW error through an IRQ
// @IWL_ERR_TYPE_NMI_FORCED: NMI was forced by driver
// @IWL_ERR_TYPE_RESET_HS_TIMEOUT: reset handshake timed out,
// any debug collection must happen synchronously as
// the device will be shut down
// @IWL_ERR_TYPE_CMD_QUEUE_FULL: command queue was full
// @IWL_ERR_TYPE_TOP_RESET_BY_BT: TOP reset initiated by BT
// @IWL_ERR_TYPE_TOP_FATAL_ERROR: TOP fatal error
// @IWL_ERR_TYPE_TOP_RESET_FAILED: TOP reset failed
// @IWL_ERR_TYPE_DEBUGFS: error/reset indication from debugfs
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_fw_error_type {
    IWL_ERR_TYPE_IRQ,
    IWL_ERR_TYPE_NMI_FORCED,
    IWL_ERR_TYPE_RESET_HS_TIMEOUT,
    IWL_ERR_TYPE_CMD_QUEUE_FULL,
    IWL_ERR_TYPE_TOP_RESET_BY_BT,
    IWL_ERR_TYPE_TOP_FATAL_ERROR,
    IWL_ERR_TYPE_TOP_RESET_FAILED,
    IWL_ERR_TYPE_DEBUGFS,
}

//
// enum iwl_fw_error_context - error dump context
// @IWL_ERR_CONTEXT_WORKER: regular from worker context,
// opmode must acquire locks and must also check
// for @IWL_ERR_CONTEXT_ABORT after acquiring locks
// @IWL_ERR_CONTEXT_FROM_OPMODE: context is in a call
// originating from the opmode, e.g. while resetting
// or stopping the device, so opmode must not acquire
// any locks
// @IWL_ERR_CONTEXT_ABORT: after lock acquisition, indicates
// that the dump already happened via another callback
// (currently only while stopping the device) via the
// @IWL_ERR_CONTEXT_FROM_OPMODE context, and this call
// must be aborted
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_fw_error_context {
    IWL_ERR_CONTEXT_WORKER,
    IWL_ERR_CONTEXT_FROM_OPMODE,
    IWL_ERR_CONTEXT_ABORT,
}

//
// struct iwl_fw_error_dump_mode - error dump mode for callback
// @type: The reason for the dump, per &enum iwl_fw_error_type.
// @context: The context for the dump, may also indicate this
// call needs to be skipped. This MUST be checked before
// and after acquiring any locks in the op-mode!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_fw_error_dump_mode {
    pub type: iwl_fw_error_type,
    pub context: iwl_fw_error_context,
}

//
// struct iwl_op_mode_ops - op_mode specific operations
//
// The op_mode exports its ops so that external components can start it and
// interact with it. The driver layer typically calls the start and stop
// handlers, the transport layer calls the others.
//
// All the handlers MUST be implemented, except @rx_rss which can be left
// out *iff* the opmode will never run on hardware with multi-queue capability.
//
// @start: start the op_mode. The transport layer is already allocated.
// May sleep
// @stop: stop the op_mode. Must free all the memory allocated.
// May sleep
// @rx: Rx notification to the op_mode. rxb is the Rx buffer itself. Cmd is the
// HCMD this Rx responds to. Can't sleep.
// @rx_rss: data queue RX notification to the op_mode, for (data) notifications
// received on the RSS queue(s). The queue parameter indicates which of the
// RSS queues received this frame; it will always be non-zero.
// This method must not sleep.
// @queue_full: notifies that a HW queue is full.
// Must be atomic and called with BH disabled.
// @queue_not_full: notifies that a HW queue is not full any more.
// Must be atomic and called with BH disabled.
// @hw_rf_kill: notifies of a change in the HW rf kill switch. True means that
// the radio is killed. Return %true if the device should be stopped by
// the transport immediately after the call. May sleep.
// Note that this must not return %true for newer devices using gen2 PCIe
// transport.
// @free_skb: allows the transport layer to free skbs that haven't been
// reclaimed by the op_mode. This can happen when the driver is freed and
// there are Tx packets pending in the transport layer.
// Must be atomic
// @nic_error: error notification. Must be atomic, the op mode should handle
// the error (e.g. abort notification waiters) and print the error if
// applicable
// @dump_error: NIC error dump collection (can sleep, synchronous)
// @sw_reset: (maybe) initiate a software reset, return %true if started
// @nic_config: configure NIC, called before firmware is started.
// May sleep
// @wimax_active: invoked when WiMax becomes active. May sleep
// @time_point: called when transport layer wants to collect debug data
// @device_powered_off: called upon resume from hibernation but not only.
// Op_mode needs to reset its internal state because the device did not
// survive the system state transition. The firmware is no longer running,
// etc...
// @dump: Op_mode needs to collect the firmware dump upon this handler
// being called.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_op_mode_ops {
    pub dbgfs_dir): *mut dentry,
    pub op_mode): *mut *mut void (stop)(struct iwl_op_mode,
    pub rxb): *mut iwl_rx_cmd_buffer,
    pub queue): *mut *mut iwl_rx_cmd_buffer rxb, unsigned int,
    pub queue): *mut *mut *mut void (queue_full)(struct iwl_op_mode op_mode, int,
    pub queue): *mut *mut *mut void (queue_not_full)(struct iwl_op_mode op_mode, int,
    pub state): *mut *mut *mut bool (hw_rf_kill)(struct iwl_op_mode op_mode, bool,
    pub skb): *mut *mut *mut void (free_skb)(struct iwl_op_mode op_mode, struct sk_buff,
    pub type): iwl_fw_error_type,
    pub mode): *mut iwl_fw_error_dump_mode,
    pub type): iwl_fw_error_type,
    pub op_mode): *mut *mut void (nic_config)(struct iwl_op_mode,
    pub op_mode): *mut *mut void (wimax_active)(struct iwl_op_mode,
    pub tp_data): *mut iwl_dbg_tlv_tp_data,
    pub op_mode): *mut *mut void (device_powered_off)(struct iwl_op_mode,
    pub op_mode): *mut *mut void (dump)(struct iwl_op_mode,
}

extern "C" {
    pub fn iwl_opmode_register(name: *const c_char, ops: *const iwl_op_mode_ops) -> c_int;
}
extern "C" {
    pub fn iwl_opmode_deregister(name: *const c_char);
}
//
// struct iwl_op_mode - operational mode
// @ops: pointer to its own ops
// @op_mode_specific: per-opmode data
//
// This holds an implementation of the mac80211 / fw API.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_op_mode {
    pub ops: *const iwl_op_mode_ops,
    pub )): *mut char op_mode_specific[] __aligned(sizeof(void,
}
