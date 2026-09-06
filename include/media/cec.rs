//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/cec.h
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
// cec - HDMI Consumer Electronics Control support header
//
// Copyright 2016 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
//

//
// struct cec_devnode - cec device node
// @dev:	cec device
// @cdev:	cec character device
// @minor:	device node minor number
// @lock:	lock to serialize open/release and registration
// @registered:	the device was correctly registered
// @unregistered: the device was unregistered
// @lock_fhs:	lock to control access to @fhs
// @fhs:	the list of open filehandles (cec_fh)
//
// This structure represents a cec-related device node.
//
// To add or remove filehandles from @fhs the @lock must be taken first,
// followed by @lock_fhs. It is safe to access @fhs if either lock is held.
//
// The @parent is a physical device. It must be set by core or device drivers
// before registering the node.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cec_devnode {
// sysfs
    pub dev: device,
    pub cdev: cdev,
// device info
    pub minor: c_int,
// serialize open/release and registration
    pub lock: mutex,
    pub registered: bool,
    pub unregistered: bool,
// protect access to fhs
    pub lock_fhs: mutex,
    pub fhs: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cec_data {
    pub list: list_head,
    pub xfer_list: list_head,
    pub adap: *mut cec_adapter,
    pub msg: cec_msg,
    pub match_len: u8,
    pub match_reply: [u8; 5],
    pub fh: *mut cec_fh,
    pub work: delayed_work,
    pub c: completion,
    pub attempts: u8,
    pub blocking: bool,
    pub completed: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cec_msg_entry {
    pub list: list_head,
    pub msg: cec_msg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cec_event_entry {
    pub list: list_head,
    pub ev: cec_event,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cec_fh {
    pub list: list_head,
    pub xfer_list: list_head,
    pub adap: *mut cec_adapter,
    pub mode_initiator: u8,
    pub mode_follower: u8,
// Events
    pub wait: wait_queue_head_t,
    pub lock: mutex,
    pub /: *mut *mut list_head events[CEC_NUM_EVENTS]; / queued events,
    pub queued_events: [u16; CEC_NUM_EVENTS],
    pub total_queued_events: c_uint,
    pub /: *mut *mut list_head msgs; / queued messages,
    pub queued_msgs: c_uint,
}

pub const CEC_SIGNAL_FREE_TIME_RETRY: c_int = 3;
pub const CEC_SIGNAL_FREE_TIME_NEW_INITIATOR: c_int = 5;
pub const CEC_SIGNAL_FREE_TIME_NEXT_XFER: c_int = 7;
// The nominal data bit period is 2.4 ms

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cec_adap_ops {
// Low-level callbacks, called with adap->lock held
    pub enable): *mut *mut *mut int (adap_enable)(struct cec_adapter adap, bool,
    pub enable): *mut *mut *mut int (adap_monitor_all_enable)(struct cec_adapter adap, bool,
    pub enable): *mut *mut *mut int (adap_monitor_pin_enable)(struct cec_adapter adap, bool,
    pub logical_addr): *mut *mut *mut int (adap_log_addr)(struct cec_adapter adap, u8,
    pub adap): *mut *mut void (adap_unconfigured)(struct cec_adapter,
    pub msg): *mut u32 signal_free_time, struct cec_msg,
    pub msg): *const cec_msg,
    pub file): *mut *mut *mut void (adap_status)(struct cec_adapter adap, struct seq_file,
    pub adap): *mut *mut void (adap_free)(struct cec_adapter,
// Error injection callbacks, called without adap->lock held
    pub sf): *mut *mut *mut int (error_inj_show)(struct cec_adapter adap, struct seq_file,
    pub line): *mut *mut *mut bool (error_inj_parse_line)(struct cec_adapter adap, char,
// High-level CEC message callback, called without adap->lock held
    pub adap): *mut *mut void (configured)(struct cec_adapter,
    pub msg): *mut *mut *mut int (received)(struct cec_adapter adap, struct cec_msg,
}

//
// The minimum message length you can receive (excepting poll messages) is 2.
// With a transfer rate of at most 36 bytes per second this makes 18 messages
// per second worst case.
//
// We queue at most 3 seconds worth of received messages. The CEC specification
// requires that messages are replied to within a second, so 3 seconds should
// give more than enough margin. Since most messages are actually more than 2
// bytes, this is in practice a lot more than 3 seconds.
//

//
// The transmit queue is limited to 1 second worth of messages (worst case).
// Messages can be transmitted by userspace and kernel space. But for both it
// makes no sense to have a lot of messages queued up. One second seems
// reasonable.
//

//
// struct cec_adapter - cec adapter structure
// @owner:		module owner
// @name:		name of the CEC adapter
// @devnode:		device node for the /dev/cecX device
// @lock:		mutex controlling access to this structure
// @rc:			remote control device
// @transmit_queue:	queue of pending transmits
// @transmit_queue_sz:	number of pending transmits
// @wait_queue:		queue of transmits waiting for a reply
// @transmitting:	CEC messages currently being transmitted
// @transmit_in_progress: true if a transmit is in progress
// @transmit_in_progress_aborted: true if a transmit is in progress is to be
// aborted. This happens if the logical address is
// invalidated while the transmit is ongoing. In that
// case the transmit will finish, but will not retransmit
// and be marked as ABORTED.
// @xfer_timeout_ms:	the transfer timeout in ms.
// If 0, then timeout after 2100 ms.
// @kthread_config:	kthread used to configure a CEC adapter
// @config_completion:	used to signal completion of the config kthread
// @kthread:		main CEC processing thread
// @kthread_waitq:	main CEC processing wait_queue
// @ops:		cec adapter ops
// @priv:		cec driver's private data
// @capabilities:	cec adapter capabilities
// @available_log_addrs: maximum number of available logical addresses
// @phys_addr:		the current physical address
// @needs_hpd:		if true, then the HDMI HotPlug Detect pin must be high
// in order to transmit or receive CEC messages. This is usually a HW
// limitation.
// @is_enabled:		the CEC adapter is enabled
// @is_claiming_log_addrs:  true if cec_claim_log_addrs() is running
// @is_configuring:	the CEC adapter is configuring (i.e. claiming LAs)
// @must_reconfigure:	while configuring, the PA changed, so reclaim LAs
// @is_configured:	the CEC adapter is configured (i.e. has claimed LAs)
// @cec_pin_is_high:	if true then the CEC pin is high. Only used with the
// CEC pin framework.
// @adap_controls_phys_addr: if true, then the CEC adapter controls the
// physical address, i.e. the CEC hardware can detect HPD changes and
// read the EDID and is not dependent on an external HDMI driver.
// Drivers that need this can set this field to true after the
// cec_allocate_adapter() call.
// @last_initiator:	the initiator of the last transmitted message.
// @monitor_all_cnt:	number of filehandles monitoring all msgs
// @monitor_pin_cnt:	number of filehandles monitoring pin changes
// @follower_cnt:	number of filehandles in follower mode
// @cec_follower:	filehandle of the exclusive follower
// @cec_initiator:	filehandle of the exclusive initiator
// @passthrough:	if true, then the exclusive follower is in
// passthrough mode.
// @log_addrs:		current logical addresses
// @conn_info:		current connector info
// @tx_timeout_cnt:	count the number of Timed Out transmits.
// Reset to 0 when this is reported in cec_adap_status().
// @tx_low_drive_cnt:	count the number of Low Drive transmits.
// Reset to 0 when this is reported in cec_adap_status().
// @tx_error_cnt:	count the number of Error transmits.
// Reset to 0 when this is reported in cec_adap_status().
// @tx_arb_lost_cnt:	count the number of Arb Lost transmits.
// Reset to 0 when this is reported in cec_adap_status().
// @tx_low_drive_log_cnt: number of logged Low Drive transmits since the
// adapter was enabled. Used to avoid flooding the kernel
// log if this happens a lot.
// @tx_error_log_cnt:	number of logged Error transmits since the adapter was
// enabled. Used to avoid flooding the kernel log if this
// happens a lot.
// @error_inj_tx_timeouts: error injection: the next @error_inj_tx_timeouts
// transmits will time out.
// @notifier:		CEC notifier
// @pin:		CEC pin status struct
// @cec_dir:		debugfs cec directory
// @sequence:		transmit sequence counter
// @input_phys:		remote control input_phys name
//
// This structure represents a cec adapter.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cec_adapter {
    pub owner: *mut module,
    pub name: [c_char; 32],
    pub devnode: cec_devnode,
    pub lock: mutex,
    pub rc: *mut rc_dev,
    pub transmit_queue: list_head,
    pub transmit_queue_sz: c_uint,
    pub wait_queue: list_head,
    pub transmitting: *mut cec_data,
    pub transmit_in_progress: bool,
    pub transmit_in_progress_aborted: bool,
    pub xfer_timeout_ms: c_uint,
    pub kthread_config: *mut task_struct,
    pub config_completion: completion,
    pub kthread: *mut task_struct,
    pub kthread_waitq: wait_queue_head_t,
    pub ops: *const cec_adap_ops,
    pub priv: *mut c_void,
    pub capabilities: u32,
    pub available_log_addrs: u8,
    pub phys_addr: u16,
    pub needs_hpd: bool,
    pub is_enabled: bool,
    pub is_claiming_log_addrs: bool,
    pub is_configuring: bool,
    pub must_reconfigure: bool,
    pub is_configured: bool,
    pub cec_pin_is_high: bool,
    pub adap_controls_phys_addr: bool,
    pub last_initiator: u8,
    pub monitor_all_cnt: u32,
    pub monitor_pin_cnt: u32,
    pub follower_cnt: u32,
    pub cec_follower: *mut cec_fh,
    pub cec_initiator: *mut cec_fh,
    pub passthrough: bool,
    pub log_addrs: cec_log_addrs,
    pub conn_info: cec_connector_info,
    pub tx_timeout_cnt: u32,
    pub tx_low_drive_cnt: u32,
    pub tx_error_cnt: u32,
    pub tx_arb_lost_cnt: u32,
    pub tx_low_drive_log_cnt: u32,
    pub tx_error_log_cnt: u32,
    pub error_inj_tx_timeouts: u32,

    pub notifier: *mut cec_notifier,

    pub pin: *mut cec_pin,

    pub cec_dir: *mut dentry,
    pub sequence: u32,
    pub input_phys: [c_char; 40],
}

//
// Check if the cec device is available. This needs to be done with
// the devnode->lock held to prevent an open/unregister race:
// without the lock, the device could be unregistered and freed between
// the devnode->registered check and get_device() calls, leading to
// a crash.
//
// return ENODEV if the cec device has been removed
// already or if it is not registered anymore.
//
// and increase the device refcount
//
// cec_is_registered() - is the CEC adapter registered?
//
// @adap:	the CEC adapter, may be NULL.
//
// Return: true if the adapter is registered, false otherwise.
//

extern "C" {
    pub fn cec_register_adapter(adap: *mut cec_adapter, parent: *mut device) -> c_int;
}
extern "C" {
    pub fn cec_unregister_adapter(adap: *mut cec_adapter);
}
extern "C" {
    pub fn cec_delete_adapter(adap: *mut cec_adapter);
}
// Called by the adapter
//
// Simplified version of cec_transmit_done for hardware that doesn't retry
// failed transmits. So this is always just one attempt in which case
// the status is sufficient.
//
// cec_queue_pin_cec_event() - queue a CEC pin event with a given timestamp.
//
// @adap:	pointer to the cec adapter
// @is_high:	when true the CEC pin is high, otherwise it is low
// @dropped_events: when true some events were dropped
// @ts:		the timestamp for this event
//
// cec_queue_pin_hpd_event() - queue a pin event with a given timestamp.
//
// @adap:	pointer to the cec adapter
// @is_high:	when true the HPD pin is high, otherwise it is low
// @ts:		the timestamp for this event
//
extern "C" {
    pub fn cec_queue_pin_hpd_event(adap: *mut cec_adapter, is_high: bool, ts: ktime_t);
}
//
// cec_queue_pin_5v_event() - queue a pin event with a given timestamp.
//
// @adap:	pointer to the cec adapter
// @is_high:	when true the 5V pin is high, otherwise it is low
// @ts:		the timestamp for this event
//
extern "C" {
    pub fn cec_queue_pin_5v_event(adap: *mut cec_adapter, is_high: bool, ts: ktime_t);
}
//
// cec_get_edid_phys_addr() - find and return the physical address
//
// @edid:	pointer to the EDID data
// @size:	size in bytes of the EDID data
// @offset:	If not %NULL then the location of the physical address
// bytes in the EDID will be returned here. This is set to 0
// if there is no physical address found.
//
// Return: the physical address or CEC_PHYS_ADDR_INVALID if there is none.
//

// offset = 0;

//
// cec_phys_addr_invalidate() - set the physical address to INVALID
//
// @adap:	the CEC adapter
//
// This is a simple helper function to invalidate the physical
// address.
//
// cec_get_edid_spa_location() - find location of the Source Physical Address
//
// @edid: the EDID
// @size: the size of the EDID
//
// This EDID is expected to be a CEA-861 compliant, which means that there are
// at least two blocks and one or more of the extensions blocks are CEA-861
// blocks.
//
// The returned location is guaranteed to be <= size-2.
//
// This is an inline function since it is used by both CEC and V4L2.
// Ideally this would go in a module shared by both, but it is overkill to do
// that for just a single function.
//
// Sanity check: at least 2 blocks and a multiple of the block size
//
// If there are fewer extension blocks than the size, then update
// 'blocks'. It is allowed to have more extension blocks than the size,
// since some hardware can only read e.g. 256 bytes of the EDID, even
// though more blocks are present. The first CEA-861 extension block
// should normally be in block 1 anyway.
//
// Skip any non-CEA-861 extension blocks
// search Vendor Specific Data Block (tag 3)
// Check if there are Data Blocks
// Note: 'end' is always < 'size'
