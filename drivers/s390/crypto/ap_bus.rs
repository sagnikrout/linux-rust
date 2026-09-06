//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/crypto/ap_bus.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright IBM Corp. 2006, 2023
// Author(s): Cornelia Huck <cornelia.huck@de.ibm.com>
// Martin Schwidefsky <schwidefsky@de.ibm.com>
// Ralph Wuerthner <rwuerthn@de.ibm.com>
// Felix Beck <felix.beck@de.ibm.com>
// Holger Dengler <hd@linux.vnet.ibm.com>
//
// Adjunct processor bus header file.
//

extern "C" {
    pub fn DECLARE_HASHTABLE(_arg: ap_queues, _arg: 8) -> extern;
}
pub const AP_RESPONSE_NORMAL: c_uint = 0x00;
pub const AP_RESPONSE_Q_NOT_AVAIL: c_uint = 0x01;
pub const AP_RESPONSE_RESET_IN_PROGRESS: c_uint = 0x02;
pub const AP_RESPONSE_DECONFIGURED: c_uint = 0x03;
pub const AP_RESPONSE_CHECKSTOPPED: c_uint = 0x04;
pub const AP_RESPONSE_BUSY: c_uint = 0x05;
pub const AP_RESPONSE_INVALID_ADDRESS: c_uint = 0x06;
pub const AP_RESPONSE_OTHERWISE_CHANGED: c_uint = 0x07;
pub const AP_RESPONSE_INVALID_GISA: c_uint = 0x08;
pub const AP_RESPONSE_Q_BOUND_TO_ANOTHER: c_uint = 0x09;
pub const AP_RESPONSE_STATE_CHANGE_IN_PROGRESS: c_uint = 0x0A;
pub const AP_RESPONSE_Q_NOT_BOUND: c_uint = 0x0B;
pub const AP_RESPONSE_Q_FULL: c_uint = 0x10;
pub const AP_RESPONSE_NO_PENDING_REPLY: c_uint = 0x10;
pub const AP_RESPONSE_INDEX_TOO_BIG: c_uint = 0x11;
pub const AP_RESPONSE_NO_FIRST_PART: c_uint = 0x13;
pub const AP_RESPONSE_MESSAGE_TOO_BIG: c_uint = 0x15;
pub const AP_RESPONSE_REQ_FAC_NOT_INST: c_uint = 0x16;
pub const AP_RESPONSE_Q_BIND_ERROR: c_uint = 0x30;
pub const AP_RESPONSE_Q_NOT_AVAIL_FOR_ASSOC: c_uint = 0x31;
pub const AP_RESPONSE_Q_NOT_EMPTY: c_uint = 0x32;
pub const AP_RESPONSE_BIND_LIMIT_EXCEEDED: c_uint = 0x33;
pub const AP_RESPONSE_INVALID_ASSOC_SECRET: c_uint = 0x34;
pub const AP_RESPONSE_ASSOC_SECRET_NOT_UNIQUE: c_uint = 0x35;
pub const AP_RESPONSE_ASSOC_FAILED: c_uint = 0x36;
pub const AP_RESPONSE_INVALID_DOMAIN: c_uint = 0x42;
//
// Supported AP device types
//
pub const AP_DEVICE_TYPE_CEX4: c_int = 10;
pub const AP_DEVICE_TYPE_CEX5: c_int = 11;
pub const AP_DEVICE_TYPE_CEX6: c_int = 12;
pub const AP_DEVICE_TYPE_CEX7: c_int = 13;
pub const AP_DEVICE_TYPE_CEX8: c_int = 14;
//
// AP queue state machine states
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ap_sm_state {
    AP_SM_STATE_RESET_START = 0,
    AP_SM_STATE_RESET_WAIT,
    AP_SM_STATE_SETIRQ_WAIT,
    AP_SM_STATE_IDLE,
    AP_SM_STATE_WORKING,
    AP_SM_STATE_QUEUE_FULL,
    AP_SM_STATE_ASSOC_WAIT,
    NR_AP_SM_STATES
}

//
// AP queue state machine events
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ap_sm_event {
    AP_SM_EVENT_POLL,
    AP_SM_EVENT_TIMEOUT,
    NR_AP_SM_EVENTS
}

//
// AP queue state wait behaviour
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ap_sm_wait {
    AP_SM_WAIT_AGAIN = 0,	 /* retry immediately */
    AP_SM_WAIT_HIGH_TIMEOUT, /* poll high freq, wait for timeout */
    AP_SM_WAIT_LOW_TIMEOUT,	 /* poll low freq, wait for timeout */
    AP_SM_WAIT_INTERRUPT,	 /* wait for thin interrupt (if available) */
    AP_SM_WAIT_NONE,	 /* no wait */
    NR_AP_SM_WAIT
}

//
// AP queue device states
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ap_dev_state {
    AP_DEV_STATE_UNINITIATED = 0,	/* fresh and virgin, not touched */
    AP_DEV_STATE_OPERATING,		/* queue dev is working normal */
    AP_DEV_STATE_SHUTDOWN,		/* remove/unbind/shutdown in progress */
    AP_DEV_STATE_ERROR,		/* device is in error state */
    NR_AP_DEV_STATES
}

//
// The ap driver struct includes a flags field which holds some info for
// the ap bus about the driver. Currently only one flag is supported and
// used: The DEFAULT flag marks an ap driver as a default driver which is
// used together with the apmask and aqmask whitelisting of the ap bus.
//
pub const AP_DRIVER_FLAG_DEFAULT: c_uint = 0x0001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ap_driver {
    pub driver: device_driver,
    pub ): *mut *mut int (probe)(struct ap_device,
    pub ): *mut *mut void (remove)(struct ap_device,
    pub aqm): *mut *mut *mut int (in_use)(unsigned long apm, unsigned long,
//
// Called at the start of the ap bus scan function when
// the crypto config information (qci) has changed.
// This callback is not invoked if there is no AP
// QCI support available.
//
    pub old_config_info): *mut ap_config_info,
//
// Called at the end of the ap bus scan function when
// the crypto config information (qci) has changed.
// This callback is not invoked if there is no AP
// QCI support available.
//
    pub old_config_info): *mut ap_config_info,
    pub ids: *mut ap_device_id,
    pub flags: c_uint,
}

extern "C" {
    pub fn ap_driver_register(: *mut ap_driver, : *mut module, : *mut c_char) -> c_int;
}
extern "C" {
    pub fn ap_driver_unregister(: *mut ap_driver);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ap_device {
    pub device: device,
    pub /: *mut *mut int device_type; / AP device type.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ap_card {
    pub ap_dev: ap_device,
    pub /: *mut *mut ap_tapq_hwinfo hwinfo; / TAPQ GR2 content,
    pub device.*/: *mut *mut atomic64_t total_request_count; / # requests ever for this AP,
    pub /: *mut *mut unsigned int maxmsgsize; / AP msg limit for this card,
    pub /: *mut *mut int id; / AP card number.,
    pub /: *mut *mut bool config; / configured state,
    pub /: *mut *mut bool chkstop; / checkstop state,
}

pub const TAPQ_CARD_HWINFO_MASK: c_uint = 0xFFFF0000FFFF0F1FUL;
pub const ASSOC_IDX_INVALID: c_uint = 0x10000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ap_queue {
    pub ap_dev: ap_device,
    pub /: *mut *mut hlist_node hnode; / Node for the ap_queues hashtable,
    pub /: *mut *mut *mut ap_card card; / Ptr to assoc. AP card.,
    pub /: *mut *mut spinlock_t lock; / Per device lock.,
    pub device.*/: *mut *mut u64 total_request_count; / # requests ever for this AP,
    pub /: *mut *mut ap_dev_state dev_state; / queue device state,
    pub /: *mut *mut ap_qid_t qid; / AP queue id.,
    pub /: *mut *mut unsigned int se_bstate; / SE bind state (BS),
    pub /: *mut *mut unsigned int assoc_idx; / SE association index,
    pub /: *mut *mut int queue_count; / # messages currently on AP queue.,
    pub /: *mut *mut int pendingq_count; / # requests on pendingq list.,
    pub /: *mut *mut int requestq_count; / # requests on requestq list.,
    pub /: *mut *mut int request_timeout; / Request timeout in jiffies.,
    pub /: *mut *mut timer_list timeout; / Timer for request timeouts.,
    pub /: *mut *mut list_head pendingq; / List of message sent to AP queue.,
    pub /: *mut *mut list_head requestq; / List of message yet to be sent.,
    pub /: *mut *mut *mut ap_message reply; / Per device reply message.,
    pub /: *mut *mut ap_sm_state sm_state; / ap queue state machine state,
    pub /: *mut *mut int rapq_fbit; / fbit arg for next rapq invocation,
    pub /: *mut *mut int last_err_rc; / last error state response code,
    pub /: *mut *mut bool config; / configured state,
    pub /: *mut *mut bool chkstop; / checkstop state,
}

extern "C" {
    pub fn ap_sm_wait(queue: *mut ap_func_t)(struct ap_queue) -> typedef enum;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ap_response_type {
    pub work: completion,
    pub type: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ap_message {
    pub /: *mut *mut list_head list; / Request queueing.,
    pub /: *mut *mut unsigned long psmid; / Message id.,
    pub /: *mut *mut *mut void msg; / Pointer to message buffer.,
    pub /: *mut *mut size_t len; / actual msg len in msg buffer,
    pub /: *mut *mut size_t bufsize; / allocated msg buffer size,
// receive is called from tasklet context
    pub ): *mut ap_message,
    pub response: ap_response_type,
    pub /: *mut *mut int rc; / Return code for this message,
    pub /: *mut *mut u16 flags; / Flags, see AP_MSG_FLAG_xxx,
}

pub const AP_MSG_FLAG_SPECIAL: c_uint = 0x0001	/* flag msg as 'special' with NQAP */;
pub const AP_MSG_FLAG_USAGE: c_uint = 0x0002	/* CCA, EP11: usage (no admin) msg */;
pub const AP_MSG_FLAG_ADMIN: c_uint = 0x0004	/* CCA, EP11: admin (=control) msg */;
pub const AP_MSG_FLAG_MEMPOOL: c_uint = 0x0008 /* ap msg buffer allocated via mempool */;
extern "C" {
    pub fn ap_init_apmsg(ap_msg: *mut ap_message, flags: u32) -> c_int;
}
extern "C" {
    pub fn ap_release_apmsg(ap_msg: *mut ap_message);
}
extern "C" {
    pub fn ap_sm_event(aq: *mut ap_queue, event: ap_sm_event) -> ap_sm_wait;
}
extern "C" {
    pub fn ap_sm_event_loop(aq: *mut ap_queue, event: ap_sm_event) -> ap_sm_wait;
}
extern "C" {
    pub fn ap_queue_message(aq: *mut ap_queue, ap_msg: *mut ap_message) -> c_int;
}
extern "C" {
    pub fn ap_cancel_message(aq: *mut ap_queue, ap_msg: *mut ap_message);
}
extern "C" {
    pub fn ap_flush_queue(aq: *mut ap_queue);
}
extern "C" {
    pub fn ap_queue_usable(aq: *mut ap_queue) -> bool;
}
extern "C" {
    pub fn ap_sb_available() -> c_int;
}
extern "C" {
    pub fn ap_is_se_guest() -> bool;
}
extern "C" {
    pub fn ap_wait(wait: ap_sm_wait);
}
extern "C" {
    pub fn ap_request_timeout(t: *mut timer_list);
}
extern "C" {
    pub fn ap_bus_force_rescan() -> bool;
}
extern "C" {
    pub fn ap_test_config_usage_domain(domain: c_uint) -> c_int;
}
extern "C" {
    pub fn ap_test_config_ctrl_domain(domain: c_uint) -> c_int;
}
extern "C" {
    pub fn ap_queue_init_reply(aq: *mut ap_queue, ap_msg: *mut ap_message);
}
extern "C" {
    pub fn ap_queue_prepare_remove(aq: *mut ap_queue);
}
extern "C" {
    pub fn ap_queue_remove(aq: *mut ap_queue);
}
extern "C" {
    pub fn ap_queue_init_state(aq: *mut ap_queue);
}
extern "C" {
    pub fn _ap_queue_init_state(aq: *mut ap_queue);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ap_perms {
    pub ioctlm: [c_ulong; BITS_TO_LONGS(AP_IOCTLS)],
    pub apm: [c_ulong; BITS_TO_LONGS(AP_DEVICES)],
    pub aqm: [c_ulong; BITS_TO_LONGS(AP_DOMAINS)],
    pub adm: [c_ulong; BITS_TO_LONGS(AP_DOMAINS)],
}

//
// Get ap_queue device for this qid.
// Returns ptr to the struct ap_queue device or NULL if there
// was no ap_queue device with this qid found. When something is
// found, the reference count of the embedded device is increased.
// So the caller has to decrease the reference count after use
// with a call to put_device(&aq->ap_dev.device).
//
// check APQN for owned/reserved by ap bus and default driver(s).
// Checks if this APQN is or will be in use by the ap bus
// and the default set of drivers.
// If yes, returns 1, if not returns 0. On error a negative
// errno value is returned.
//
extern "C" {
    pub fn ap_owned_by_def_drv(card: c_int, queue: c_int) -> c_int;
}
//
// check 'matrix' of APQNs for owned/reserved by ap bus and
// default driver(s).
// Checks if there is at least one APQN in the given 'matrix'
// marked as owned/reserved by the ap bus and default driver(s).
// If such an APQN is found the return value is 1, otherwise
// 0 is returned. On error a negative errno value is returned.
// The parameter apm is a bitmask which should be declared
// as DECLARE_BITMAP(apm, AP_DEVICES), the aqm parameter is
// similar, should be declared as DECLARE_BITMAP(aqm, AP_DOMAINS).
//
// ap_parse_mask_str() - helper function to parse a bitmap string
// and clear/set the bits in the bitmap accordingly. The string may be
// given as absolute value, a hex string like 0x1F2E3D4C5B6A" simple
// overwriting the current content of the bitmap. Or as relative string
// like "+1-16,-32,-0x40,+128" where only single bits or ranges of
// bits are cleared or set. Distinction is done based on the very
// first character which may be '+' or '-' for the relative string
// and otherwise assume to be an absolute value string. If parsing fails
// a negative errno value is returned. All arguments and bitmaps are
// big endian order.
//
// ap_hex2bitmap() - Convert a string containing a hexadecimal number (str)
// into a bitmap (bitmap) with bits set that correspond to the bits represented
// by the hex string. Input and output data is in big endian order.
//
// str - Input hex string of format "0x1234abcd". The leading "0x" is optional.
// At least one digit is required. Must be large enough to hold the number of
// bits represented by the bits parameter.
//
// bitmap - Pointer to a bitmap. Upon successful completion of this function,
// this bitmap will have bits set to match the value of str. If bitmap is longer
// than str, then the rightmost bits of bitmap are padded with zeros. Must be
// large enough to hold the number of bits represented by the bits parameter.
//
// bits - Length, in bits, of the bitmap represented by str. Must be a multiple
// of 8.
//
// Returns: 0		On success
// -EINVAL	If str format is invalid or bits is not a multiple of 8.
//
extern "C" {
    pub fn ap_hex2bitmap(str: *const c_char, bitmap: *mut c_ulong, bits: c_int) -> c_int;
}
//
// Interface to wait for the AP bus to have done one initial ap bus
// scan and all detected APQNs have been bound to device drivers.
// If these both conditions are not fulfilled, this function blocks
// on a condition with wait_for_completion_killable_timeout().
// If these both conditions are fulfilled (before the timeout hits)
// the return value is 0. If the timeout (in jiffies) hits instead
// -ETIME is returned. On failures negative return values are
// returned to the caller.
// It may be that the AP bus scan finds new devices. Then the
// condition that all APQNs are bound to their device drivers
// is reset to false and this call again blocks until either all
// APQNs are bound to a device driver or the timeout hits again.
//
extern "C" {
    pub fn ap_wait_apqn_bindings_complete(timeout: c_ulong) -> c_int;
}
extern "C" {
    pub fn ap_send_config_uevent(ap_dev: *mut ap_device, cfg: bool);
}
extern "C" {
    pub fn ap_send_online_uevent(ap_dev: *mut ap_device, online: c_int);
}
extern "C" {
    pub fn ap_send_se_bind_uevent(ap_dev: *mut ap_device);
}
extern "C" {
    pub fn ap_send_se_assoc_uevent(ap_dev: *mut ap_device, assoc_idx: c_uint);
}
