//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/vboxguest.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR CDDL-1.0)
//
// VBoxGuest - VirtualBox Guest Additions Driver Interface.
//
// Copyright (C) 2006-2016 Oracle Corporation
//

// Version of vbg_ioctl_hdr structure.
pub const VBG_IOCTL_HDR_VERSION: c_uint = 0x10001;
// Default request type.  Use this for non-VMMDev requests.
pub const VBG_IOCTL_HDR_TYPE_DEFAULT: c_int = 0;
//
// Common ioctl header.
//
// This is a mirror of vmmdev_request_header to prevent duplicating data and
// needing to verify things multiple times.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbg_ioctl_hdr {
// IN: The request input size, and output size if size_out is zero.
    pub size_in: __u32,
// IN: Structure version (VBG_IOCTL_HDR_VERSION)
    pub version: __u32,
// IN: The VMMDev request type or VBG_IOCTL_HDR_TYPE_DEFAULT.
    pub type: __u32,
//
// OUT: The VBox status code of the operation, out direction only.
// This is a VINF_ or VERR_ value as defined in vbox_err.h.
//
    pub rc: __s32,
// IN: Output size. Set to zero to use size_in as output size.
    pub size_out: __u32,
// Reserved, MBZ.
    pub reserved: __u32,
}

//
// The VBoxGuest I/O control version.
//
// As usual, the high word contains the major version and changes to it
// signifies incompatible changes.
//
// The lower word is the minor version number, it is increased when new
// functions are added or existing changed in a backwards compatible manner.
//
pub const VBG_IOC_VERSION: c_uint = 0x00010000u;
//
// VBG_IOCTL_DRIVER_VERSION_INFO data structure
//
// Note VBG_IOCTL_DRIVER_VERSION_INFO may switch the session to a backwards
// compatible interface version if uClientVersion indicates older client code.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbg_ioctl_driver_version_info {
// The header.
    pub hdr: vbg_ioctl_hdr,
// Requested interface version (VBG_IOC_VERSION).
    pub req_version: __u32,
//
// Minimum interface version number (typically the
// major version part of VBG_IOC_VERSION).
//
    pub min_version: __u32,
// Reserved, MBZ.
    pub reserved1: __u32,
// Reserved, MBZ.
    pub reserved2: __u32,
    pub in: },
// Version for this session (typ. VBG_IOC_VERSION).
    pub session_version: __u32,
// Version of the IDC interface (VBG_IOC_VERSION).
    pub driver_version: __u32,
// The SVN revision of the driver, or 0.
    pub driver_revision: __u32,
// Reserved \#1 (zero until defined).
    pub reserved1: __u32,
// Reserved \#2 (zero until defined).
    pub reserved2: __u32,
    pub out: },
    pub u: },
}

// IOCTL to perform a VMM Device request less than 1KB in size.

// IOCTL to perform a VMM Device request larger then 1KB.

// VBG_IOCTL_HGCM_CONNECT data structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbg_ioctl_hgcm_connect {
    pub hdr: vbg_ioctl_hdr,
    pub loc: vmmdev_hgcm_service_location,
    pub in: },
    pub client_id: __u32,
    pub out: },
    pub u: },
}

// VBG_IOCTL_HGCM_DISCONNECT data structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbg_ioctl_hgcm_disconnect {
    pub hdr: vbg_ioctl_hdr,
    pub client_id: __u32,
    pub in: },
    pub u: },
}

// VBG_IOCTL_HGCM_CALL data structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbg_ioctl_hgcm_call {
// The header.
    pub hdr: vbg_ioctl_hdr,
// Input: The id of the caller.
    pub client_id: __u32,
// Input: Function number.
    pub function: __u32,
//
// Input: How long to wait (milliseconds) for completion before
// cancelling the call. Set to -1 to wait indefinitely.
//
    pub timeout_ms: __u32,
// Interruptable flag, ignored for userspace calls.
    pub interruptible: __u8,
// Explicit padding, MBZ.
    pub reserved: __u8,
//
// Input: How many parameters following this structure.
//
// The parameters are either HGCMFunctionParameter64 or 32,
// depending on whether we're receiving a 64-bit or 32-bit request.
//
// The current maximum is 61 parameters (given a 1KB max request size,
// and a 64-bit parameter size of 16 bytes).
//
    pub parm_count: __u16,
//
// Parameters follow in form:
// struct hgcm_function_parameter<32|64> parms[parm_count]
//
}

// VBG_IOCTL_LOG data structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbg_ioctl_log {
// The header.
    pub hdr: vbg_ioctl_hdr,
//
// The log message, this may be zero terminated. If it
// is not zero terminated then the length is determined
// from the input size.
//
    pub msg: [c_char; 1],
    pub in: },
    pub u: },
}

// VBG_IOCTL_WAIT_FOR_EVENTS data structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbg_ioctl_wait_for_events {
// The header.
    pub hdr: vbg_ioctl_hdr,
// Timeout in milliseconds.
    pub timeout_ms: __u32,
// Events to wait for.
    pub events: __u32,
    pub in: },
// Events that occurred.
    pub events: __u32,
    pub out: },
    pub u: },
}

//
// IOCTL to VBoxGuest to interrupt (cancel) any pending
// VBG_IOCTL_WAIT_FOR_EVENTS and return.
//
// Handled inside the vboxguest driver and not seen by the host at all.
// After calling this, VBG_IOCTL_WAIT_FOR_EVENTS should no longer be called in
// the same session. Any VBOXGUEST_IOCTL_WAITEVENT calls in the same session
// done after calling this will directly exit with -EINTR.
//

// VBG_IOCTL_CHANGE_FILTER_MASK data structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbg_ioctl_change_filter {
// The header.
    pub hdr: vbg_ioctl_hdr,
// Flags to set.
    pub or_mask: __u32,
// Flags to remove.
    pub not_mask: __u32,
    pub in: },
    pub u: },
}

// IOCTL to VBoxGuest to control the event filter mask.

// VBG_IOCTL_ACQUIRE_GUEST_CAPABILITIES data structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbg_ioctl_acquire_guest_caps {
// The header.
    pub hdr: vbg_ioctl_hdr,
// Flags (VBGL_IOC_AGC_FLAGS_XXX).
    pub flags: __u32,
// Capabilities to set (VMMDEV_GUEST_SUPPORTS_XXX).
    pub or_mask: __u32,
// Capabilities to drop (VMMDEV_GUEST_SUPPORTS_XXX).
    pub not_mask: __u32,
    pub in: },
    pub u: },
}

pub const VBGL_IOC_AGC_FLAGS_CONFIG_ACQUIRE_MODE: c_uint = 0x00000001;
pub const VBGL_IOC_AGC_FLAGS_VALID_MASK: c_uint = 0x00000001;

// VBG_IOCTL_CHANGE_GUEST_CAPABILITIES data structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbg_ioctl_set_guest_caps {
// The header.
    pub hdr: vbg_ioctl_hdr,
// Capabilities to set (VMMDEV_GUEST_SUPPORTS_XXX).
    pub or_mask: __u32,
// Capabilities to drop (VMMDEV_GUEST_SUPPORTS_XXX).
    pub not_mask: __u32,
    pub in: },
// Capabilities held by the session after the call.
    pub session_caps: __u32,
// Capabilities for all the sessions after the call.
    pub global_caps: __u32,
    pub out: },
    pub u: },
}

// VBG_IOCTL_CHECK_BALLOON data structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbg_ioctl_check_balloon {
// The header.
    pub hdr: vbg_ioctl_hdr,
// The size of the balloon in chunks of 1MB.
    pub balloon_chunks: __u32,
//
// false = handled in R0, no further action required.
// true = allocate balloon memory in R3.
//
    pub handle_in_r3: __u8,
// Explicit padding, MBZ.
    pub padding: [__u8; 3],
    pub out: },
    pub u: },
}

//
// IOCTL to check memory ballooning.
//
// The guest kernel module will ask the host for the current size of the
// balloon and adjust the size. Or it will set handle_in_r3 = true and R3 is
// responsible for allocating memory and calling VBG_IOCTL_CHANGE_BALLOON.
//

// VBG_IOCTL_WRITE_CORE_DUMP data structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbg_ioctl_write_coredump {
    pub hdr: vbg_ioctl_hdr,
    pub /: *mut *mut *mut __u32 flags; / Flags (reserved, MBZ).,
    pub in: },
    pub u: },
}

