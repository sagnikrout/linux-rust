//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/virt/vboxguest/vboxguest_core.h
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


// SPDX-License-Identifier: (GPL-2.0 OR CDDL-1.0)
// Copyright (C) 2010-2016 Oracle Corporation

//
// The mainline kernel version (this version) of the vboxguest module
// contained a bug where it defined VBGL_IOCTL_VMMDEV_REQUEST_BIG and
// VBGL_IOCTL_LOG using _IOC(_IOC_READ | _IOC_WRITE, 'V', ...) instead
// of _IO(V, ...) as the out of tree VirtualBox upstream version does.
//
// These _ALT definitions keep compatibility with the wrong defines the
// mainline kernel version used for a while.
// Note the VirtualBox userspace bits have always been built against
// VirtualBox upstream's headers, so this is likely not necessary. But
// we must never break our ABI so we keep these around to be 100% sure.
//

// VBox guest memory balloon.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbg_mem_balloon {
// Work handling VMMDEV_EVENT_BALLOON_CHANGE_REQUEST events
    pub work: work_struct,
// Pre-allocated vmmdev_memballoon_info req for query
    pub get_req: *mut vmmdev_memballoon_info,
// Pre-allocated vmmdev_memballoon_change req for inflate / deflate
    pub change_req: *mut vmmdev_memballoon_change,
// The current number of chunks in the balloon.
    pub chunks: u32,
// The maximum number of chunks in the balloon.
    pub max_chunks: u32,
//
// Array of pointers to page arrays. A page * array is allocated for
// each chunk when inflating, and freed when the deflating.
//
    pub pages: *mut page,
}

//
// Per bit usage tracker for a u32 mask.
//
// Used for optimal handling of guest properties and event filter.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbg_bit_usage_tracker {
// Per bit usage counters.
    pub per_bit_usage: [u32; 32],
// The current mask according to per_bit_usage.
    pub mask: u32,
}

// VBox guest device (data) extension.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbg_dev {
    pub dev: *mut device,
// The base of the adapter I/O ports.
    pub io_port: u16,
// Pointer to the mapping of the VMMDev adapter memory.
    pub mmio: *mut vmmdev_memory,
// Host version
    pub host_version: [c_char; 64],
// Host features
    pub host_features: c_uint,
//
// Dummy page and vmap address for reserved kernel virtual-address
// space for the guest mappings, only used on hosts lacking vtx.
//
    pub guest_mappings_dummy_page: *mut page,
    pub guest_mappings: *mut c_void,
// Spinlock protecting pending_events.
    pub event_spinlock: spinlock_t,
// Preallocated struct vmmdev_events for the IRQ handler.
    pub ack_events_req: *mut vmmdev_events,
// Wait-for-event list for threads waiting for multiple events.
    pub event_wq: wait_queue_head_t,
// Mask of pending events.
    pub pending_events: u32,
// Wait-for-event list for threads waiting on HGCM async completion.
    pub hgcm_wq: wait_queue_head_t,
// Pre-allocated hgcm cancel2 req. for cancellation on timeout
    pub cancel_req: *mut vmmdev_hgcm_cancel2,
// Mutex protecting cancel_req accesses
    pub cancel_req_mutex: mutex,
// Pre-allocated mouse-status request for the input-device handling.
    pub mouse_status_req: *mut vmmdev_mouse_status,
// Input device for reporting abs mouse coordinates to the guest.
    pub input: *mut input_dev,
// Memory balloon information.
    pub mem_balloon: vbg_mem_balloon,
// Lock for session related items in vbg_dev and vbg_session
    pub session_mutex: mutex,
// Events we won't permit anyone to filter out.
    pub fixed_events: u32,
//
// Usage counters for the host events (excludes fixed events),
// Protected by session_mutex.
//
    pub event_filter_tracker: vbg_bit_usage_tracker,
//
// The event filter last reported to the host (or UINT32_MAX).
// Protected by session_mutex.
//
    pub event_filter_host: u32,
//
// Guest capabilities which have been switched to acquire_mode.
//
    pub acquire_mode_guest_caps: u32,
//
// Guest capabilities acquired by vbg_acquire_session_capabilities().
// Only one session can acquire a capability at a time.
//
    pub acquired_guest_caps: u32,
//
// Usage counters for guest capabilities requested through
// vbg_set_session_capabilities(). Indexed by capability bit
// number, one count per session using a capability.
// Protected by session_mutex.
//
    pub set_guest_caps_tracker: vbg_bit_usage_tracker,
//
// The guest capabilities last reported to the host (or UINT32_MAX).
// Protected by session_mutex.
//
    pub guest_caps_host: u32,
//
// Heartbeat timer which fires with interval
// cNsHearbeatInterval and its handler sends
// VMMDEVREQ_GUEST_HEARTBEAT to VMMDev.
//
    pub heartbeat_timer: timer_list,
// Heartbeat timer interval in ms.
    pub heartbeat_interval_ms: c_int,
// Preallocated VMMDEVREQ_GUEST_HEARTBEAT request.
    pub guest_heartbeat_req: *mut vmmdev_request_header,
// "vboxguest" char-device
    pub misc_device: miscdevice,
// "vboxuser" char-device
    pub misc_device_user: miscdevice,
}

// The VBoxGuest per session data.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbg_session {
// Pointer to the device extension.
    pub gdev: *mut vbg_dev,
//
// Array containing HGCM client IDs associated with this session.
// These will be automatically disconnected when the session is closed.
// Protected by vbg_gdev.session_mutex.
//
    pub hgcm_client_ids: [u32; 64],
//
// Host events requested by the session.
// An event type requested in any guest session will be added to the
// host filter. Protected by vbg_gdev.session_mutex.
//
    pub event_filter: u32,
//
// Guest capabilities acquired by vbg_acquire_session_capabilities().
// Only one session can acquire a capability at a time.
//
    pub acquired_guest_caps: u32,
//
// Guest capabilities set through vbg_set_session_capabilities().
// A capability claimed by any guest session will be reported to the
// host. Protected by vbg_gdev.session_mutex.
//
    pub set_guest_caps: u32,
// VMMDEV_REQUESTOR_* flags
    pub requestor: u32,
// Set on CANCEL_ALL_WAITEVENTS, protected by vbg_devevent_spinlock.
    pub cancel_waiters: bool,
}

extern "C" {
    pub fn vbg_core_init(gdev: *mut vbg_dev, fixed_events: u32) -> c_int;
}
extern "C" {
    pub fn vbg_core_exit(gdev: *mut vbg_dev);
}
extern "C" {
    pub fn vbg_core_close_session(session: *mut vbg_session);
}
extern "C" {
    pub fn vbg_core_ioctl(session: *mut vbg_session, req: c_uint, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn vbg_core_set_mouse_status(gdev: *mut vbg_dev, features: u32) -> c_int;
}
extern "C" {
    pub fn vbg_core_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn vbg_linux_mouse_event(gdev: *mut vbg_dev);
}
// Private (non exported) functions form vboxguest_utils.c
extern "C" {
    pub fn vbg_req_free(req: *mut c_void, len: usize);
}
extern "C" {
    pub fn vbg_req_perform(gdev: *mut vbg_dev, req: *mut c_void) -> c_int;
}
