//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/mvm/time-event.h
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
// Copyright (C) 2012-2014, 2019-2020, 2023, 2025 Intel Corporation
// Copyright (C) 2013-2014 Intel Mobile Communications GmbH
//

// Macro flag: #define __time_event_h__

//
// DOC: Time Events - what is it?
//
// Time Events are a fw feature that allows the driver to control the presence
// of the device on the channel. Since the fw supports multiple channels
// concurrently, the fw may choose to jump to another channel at any time.
// In order to make sure that the fw is on a specific channel at a certain time
// and for a certain duration, the driver needs to issue a time event.
//
// The simplest example is for BSS association. The driver issues a time event,
// waits for it to start, and only then tells mac80211 that we can start the
// association. This way, we make sure that the association will be done
// smoothly and won't be interrupted by channel switch decided within the fw.
//
// DOC: The flow against the fw
//
// When the driver needs to make sure we are in a certain channel, at a certain
// time and for a certain duration, it sends a Time Event. The flow against the
// fw goes like this:
// 1) Driver sends a TIME_EVENT_CMD to the fw
// 2) Driver gets the response for that command. This response contains the
// Unique ID (UID) of the event.
// 3) The fw sends notification when the event starts.
//
// Of course the API provides various options that allow to cover parameters
// of the flow.
// What is the duration of the event?
// What is the start time of the event?
// Is there an end-time for the event?
// How much can the event be delayed?
// Can the event be split?
// If yes what is the maximal number of chunks?
// etc...
//
// DOC: Abstraction to the driver
//
// In order to simplify the use of time events to the rest of the driver,
// we abstract the use of time events. This component provides the functions
// needed by the driver.
//
pub const IWL_MVM_TE_SESSION_PROTECTION_MAX_TIME_MS: c_int = 600;
pub const IWL_MVM_TE_SESSION_PROTECTION_MIN_TIME_MS: c_int = 400;
//
// iwl_mvm_protect_session - start / extend the session protection.
// @mvm: the mvm component
// @vif: the virtual interface for which the session is issued
// @duration: the duration of the session in TU.
// @min_duration: will start a new session if the current session will end
// in less than min_duration.
// @max_delay: maximum delay before starting the time event (in TU)
// @wait_for_notif: true if it is required that a time event notification be
// waited for (that the time event has been scheduled before returning)
//
// This function can be used to start a session protection which means that the
// fw will stay on the channel for %duration_ms milliseconds. This function
// can block (sleep) until the session starts. This function can also be used
// to extend a currently running session.
// This function is meant to be used for BSS association for example, where we
// want to make sure that the fw stays on the channel during the association.
//
// iwl_mvm_stop_session_protection - cancel the session protection.
// @mvm: the mvm component
// @vif: the virtual interface for which the session is issued
//
// This functions cancels the session protection which is an act of good
// citizenship. If it is not needed any more it should be canceled because
// the other bindings wait for the medium during that time.
// This funtions doesn't sleep.
//
// iwl_mvm_rx_time_event_notif - handles %TIME_EVENT_NOTIFICATION.
//
// iwl_mvm_rx_roc_notif - handles %DISCOVERY_ROC_NTF.
// @mvm: the mvm component
// @rxb: RX buffer
//
// iwl_mvm_start_p2p_roc - start remain on channel for p2p device functionality
// @mvm: the mvm component
// @vif: the virtual interface for which the roc is requested. It is assumed
// that the vif type is NL80211_IFTYPE_P2P_DEVICE
// @duration: the requested duration in millisecond for the fw to be on the
// channel that is bound to the vif.
// @type: the remain on channel request type
//
// This function can be used to issue a remain on channel session,
// which means that the fw will stay in the channel for the request %duration
// milliseconds. The function is async, meaning that it only issues the ROC
// request but does not wait for it to start. Once the FW is ready to serve the
// ROC request, it will issue a notification to the driver that it is on the
// requested channel. Once the FW completes the ROC request it will issue
// another notification to the driver.
//
// Return: negative error code or 0 on success
//
// iwl_mvm_stop_roc - stop remain on channel functionality
// @mvm: the mvm component
// @vif: the virtual interface for which the roc is stopped
//
// This function can be used to cancel an ongoing ROC session.
// The function is async, it will instruct the FW to stop serving the ROC
// session, but will not wait for the actual stopping of the session.
//
extern "C" {
    pub fn iwl_mvm_stop_roc(mvm: *mut iwl_mvm, vif: *mut ieee80211_vif);
}
//
// iwl_mvm_remove_time_event - general function to clean up of time event
// @mvm: the mvm component
// @mvmvif: the vif to which the time event belongs
// @te_data: the time event data that corresponds to that time event
//
// This function can be used to cancel a time event regardless its type.
// It is useful for cleaning up time events running before removing an
// interface.
//
// iwl_mvm_te_clear_data - remove time event from list
// @mvm: the mvm component
// @te_data: the time event data to remove
//
// This function is mostly internal, it is made available here only
// for firmware restart purposes.
//
extern "C" {
    pub fn iwl_mvm_cleanup_roc_te(mvm: *mut iwl_mvm);
}
extern "C" {
    pub fn iwl_mvm_roc_done_wk(wk: *mut work_struct);
}
//
// iwl_mvm_schedule_csa_period - request channel switch absence period
// @mvm: the mvm component
// @vif: the virtual interface for which the channel switch is issued
// @duration: the duration of the NoA in TU.
// @apply_time: NoA start time in GP2.
//
// This function is used to schedule NoA time event and is used to perform
// the channel switch flow.
//
// Return: negative error code or 0 on success
//
// iwl_mvm_te_scheduled - check if the fw received the TE cmd
// @te_data: the time event data that corresponds to that time event
//
// Return: %true if this TE is added to the fw, %false otherwise
//
// iwl_mvm_schedule_session_protection - schedule a session protection
// @mvm: the mvm component
// @vif: the virtual interface for which the protection issued
// @duration: the requested duration of the protection
// @min_duration: the minimum duration of the protection
// @wait_for_notif: if true, will block until the start of the protection
//
// iwl_mvm_rx_session_protect_notif - handles %SESSION_PROTECTION_NOTIF
// @mvm: the mvm component
// @rxb: the RX buffer containing the notification
//
