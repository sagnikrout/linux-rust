//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/virt/vboxguest/vmmdev.h
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
//
// Virtual Device for Guest <-> VMM/Host communication interface
//
// Copyright (C) 2006-2016 Oracle Corporation
//

// Port for generic request interface (relative offset).
pub const VMMDEV_PORT_OFF_REQUEST: c_int = 0;
// Layout of VMMDEV RAM region that contains information for guest.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmmdev_memory {
// The size of this structure.
    pub size: u32,
// The structure version. (VMMDEV_MEMORY_VERSION)
    pub version: u32,
// Flag telling that VMMDev has events pending.
    pub have_events: u8,
// Explicit padding, MBZ.
    pub padding: [u8; 3],
    pub V1_04: },
// Pending events flags, set by host.
    pub host_events: u32,
// Mask of events the guest wants, set by guest.
    pub guest_event_mask: u32,
    pub V1_03: },
    pub V: },
// struct vbva_memory, not used
}

// Version of vmmdev_memory structure (vmmdev_memory::version).

// Host mouse capabilities has been changed.

// HGCM event.

// A display change request has been issued.

// Credentials are available for judgement.

// The guest has been restored.

// Seamless mode state changed.

// Memory balloon size changed.

// Statistics interval changed.

// VRDP status changed.

// New mouse position data available.

// CPU hotplug event occurred.

// The mask of valid events, for sanity checking.
pub const VMMDEV_EVENT_VALID_EVENT_MASK: c_uint = 0x000007ffU;
//
// Additions are allowed to work only if additions_major == vmmdev_current &&
// additions_minor <= vmmdev_current. Additions version is reported to host
// (VMMDev) by VMMDEVREQ_REPORT_GUEST_INFO.
//
pub const VMMDEV_VERSION: c_uint = 0x00010004;

// Maximum request packet size.
pub const VMMDEV_MAX_VMMDEVREQ_SIZE: c_int = 1048576;
// Version of vmmdev_request_header structure.
pub const VMMDEV_REQUEST_HEADER_VERSION: c_uint = 0x10001;
// struct vmmdev_request_header - Generic VMMDev request header.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmmdev_request_header {
// IN: Size of the structure in bytes (including body).
    pub size: u32,
// IN: Version of the structure.
    pub version: u32,
// IN: Type of the request.
    pub request_type: vmmdev_request_type,
// OUT: Return code.
    pub rc: i32,
// Reserved field no.1. MBZ.
    pub reserved1: u32,
// IN: Requestor information (VMMDEV_REQUESTOR_*)
    pub requestor: u32,
}

//
// struct vmmdev_mouse_status - Mouse status request structure.
//
// Used by VMMDEVREQ_GET_MOUSE_STATUS and VMMDEVREQ_SET_MOUSE_STATUS.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmmdev_mouse_status {
// header
    pub header: vmmdev_request_header,
// Mouse feature mask. See VMMDEV_MOUSE_*.
    pub mouse_features: u32,
// Mouse x position.
    pub pointer_pos_x: i32,
// Mouse y position.
    pub pointer_pos_y: i32,
}

// The guest can (== wants to) handle absolute coordinates.

//
// The host can (== wants to) send absolute coordinates.
// (Input not captured.)
//

//
// The guest can *NOT* switch to software cursor and therefore depends on the
// host cursor.
//
// When guest additions are installed and the host has promised to display the
// cursor itself, the guest installs a hardware mouse driver. Don't ask the
// guest to switch to a software cursor then.
//

// The host does NOT provide support for drawing the cursor itself.

// The guest can read VMMDev events to find out about pointer movement

//
// If the guest changes the status of the VMMDEV_MOUSE_GUEST_NEEDS_HOST_CURSOR
// bit, the host will honour this.
//

//
// The host supplies an absolute pointing device.  The Guest Additions may
// wish to use this to decide whether to install their own driver.
//

// The minimum value our pointing device can return.
pub const VMMDEV_MOUSE_RANGE_MIN: c_int = 0;
// The maximum value our pointing device can return.
pub const VMMDEV_MOUSE_RANGE_MAX: c_uint = 0xFFFF;
//
// struct vmmdev_host_version - VirtualBox host version request structure.
//
// VBG uses this to detect the precense of new features in the interface.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmmdev_host_version {
// Header.
    pub header: vmmdev_request_header,
// Major version.
    pub major: u16,
// Minor version.
    pub minor: u16,
// Build number.
    pub build: u32,
// SVN revision.
    pub revision: u32,
// Feature mask.
    pub features: u32,
}

// Physical page lists are supported by HGCM.

//
// struct vmmdev_mask - Structure to set / clear bits in a mask used for
// VMMDEVREQ_SET_GUEST_CAPABILITIES and VMMDEVREQ_CTL_GUEST_FILTER_MASK.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmmdev_mask {
// Header.
    pub header: vmmdev_request_header,
// Mask of bits to be set.
    pub or_mask: u32,
// Mask of bits to be cleared.
    pub not_mask: u32,
}

// The guest supports seamless display rendering.

// The guest supports mapping guest to host windows.

//
// The guest graphical additions are active.
// Used for fast activation and deactivation of certain graphical operations
// (e.g. resizing & seamless). The legacy VMMDEVREQ_REPORT_GUEST_CAPABILITIES
// request sets this automatically, but VMMDEVREQ_SET_GUEST_CAPABILITIES does
// not.
//

// The mask of valid capabilities, for sanity checking.
pub const VMMDEV_GUEST_CAPABILITIES_MASK: c_uint = 0x00000007U;
// struct vmmdev_hypervisorinfo - Hypervisor info structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmmdev_hypervisorinfo {
// Header.
    pub header: vmmdev_request_header,
//
// Guest virtual address of proposed hypervisor start.
// Not used by VMMDEVREQ_GET_HYPERVISOR_INFO.
//
    pub hypervisor_start: u32,
// Hypervisor size in bytes.
    pub hypervisor_size: u32,
}

// struct vmmdev_events - Pending events structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmmdev_events {
// Header.
    pub header: vmmdev_request_header,
// OUT: Pending event mask.
    pub events: u32,
}

pub const VMMDEV_OSTYPE_LINUX26: c_uint = 0x53000;

// struct vmmdev_guestinfo - Guest information report.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmmdev_guest_info {
// Header.
    pub header: vmmdev_request_header,
//
// The VMMDev interface version expected by additions.
// *Deprecated*, do not use anymore! Will be removed.
//
    pub interface_version: u32,
// Guest OS type.
    pub os_type: u32,
}

// struct vmmdev_guestinfo2 - Guest information report, version 2.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmmdev_guest_info2 {
// Header.
    pub header: vmmdev_request_header,
// Major version.
    pub additions_major: u16,
// Minor version.
    pub additions_minor: u16,
// Build number.
    pub additions_build: u32,
// SVN revision.
    pub additions_revision: u32,
// Feature mask.
    pub additions_features: u32,
//
// The intentional meaning of this field was:
// Some additional information, for example 'Beta 1' or something like
// that.
//
// The way it was implemented was implemented: VBG_VERSION_STRING.
//
// This means the first three members are duplicated in this field (if
// the guest build config is sane). So, the user must check this and
// chop it off before usage. There is, because of the Main code's blind
// trust in the field's content, no way back.
//
    pub name: [c_char; 128],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vmmdev_guest_facility_type {
    VBOXGUEST_FACILITY_TYPE_UNKNOWN          = 0,
    VBOXGUEST_FACILITY_TYPE_VBOXGUEST_DRIVER = 20,
// VBoxGINA / VBoxCredProv / pam_vbox.
    VBOXGUEST_FACILITY_TYPE_AUTO_LOGON       = 90,
    VBOXGUEST_FACILITY_TYPE_VBOX_SERVICE     = 100,
// VBoxTray (Windows), VBoxClient (Linux, Unix).
    VBOXGUEST_FACILITY_TYPE_VBOX_TRAY_CLIENT = 101,
    VBOXGUEST_FACILITY_TYPE_SEAMLESS         = 1000,
    VBOXGUEST_FACILITY_TYPE_GRAPHICS         = 1100,
    VBOXGUEST_FACILITY_TYPE_ALL              = 0x7ffffffe,
// Ensure the enum is a 32 bit data-type
    VBOXGUEST_FACILITY_TYPE_SIZEHACK         = 0x7fffffff
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vmmdev_guest_facility_status {
    VBOXGUEST_FACILITY_STATUS_INACTIVE    = 0,
    VBOXGUEST_FACILITY_STATUS_PAUSED      = 1,
    VBOXGUEST_FACILITY_STATUS_PRE_INIT    = 20,
    VBOXGUEST_FACILITY_STATUS_INIT        = 30,
    VBOXGUEST_FACILITY_STATUS_ACTIVE      = 50,
    VBOXGUEST_FACILITY_STATUS_TERMINATING = 100,
    VBOXGUEST_FACILITY_STATUS_TERMINATED  = 101,
    VBOXGUEST_FACILITY_STATUS_FAILED      = 800,
    VBOXGUEST_FACILITY_STATUS_UNKNOWN     = 999,
// Ensure the enum is a 32 bit data-type
    VBOXGUEST_FACILITY_STATUS_SIZEHACK    = 0x7fffffff
}

// struct vmmdev_guest_status - Guest Additions status structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmmdev_guest_status {
// Header.
    pub header: vmmdev_request_header,
// Facility the status is indicated for.
    pub facility: vmmdev_guest_facility_type,
// Current guest status.
    pub status: vmmdev_guest_facility_status,
// Flags, not used at the moment.
    pub flags: u32,
}

// struct vmmdev_memballoon_info - Memory-balloon info structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmmdev_memballoon_info {
// Header.
    pub header: vmmdev_request_header,
// Balloon size in megabytes.
    pub balloon_chunks: u32,
// Guest ram size in megabytes.
    pub phys_mem_chunks: u32,
//
// Setting this to VMMDEV_EVENT_BALLOON_CHANGE_REQUEST indicates that
// the request is a response to that event.
// (Don't confuse this with VMMDEVREQ_ACKNOWLEDGE_EVENTS.)
//
    pub event_ack: u32,
}

// struct vmmdev_memballoon_change - Change the size of the balloon.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmmdev_memballoon_change {
// Header.
    pub header: vmmdev_request_header,
// The number of pages in the array.
    pub pages: u32,
// true = inflate, false = deflate.
    pub inflate: u32,
// Physical address (u64) of each page.
    pub phys_page: [u64; VMMDEV_MEMORY_BALLOON_CHUNK_PAGES],
}

// struct vmmdev_write_core_dump - Write Core Dump request data.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmmdev_write_core_dump {
// Header.
    pub header: vmmdev_request_header,
// Flags (reserved, MBZ).
    pub flags: u32,
}

// struct vmmdev_heartbeat - Heart beat check state structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmmdev_heartbeat {
// Header.
    pub header: vmmdev_request_header,
// OUT: Guest heartbeat interval in nanosec.
    pub interval_ns: u64,
// Heartbeat check flag.
    pub enabled: u8,
// Explicit padding, MBZ.
    pub padding: [u8; 3],
    pub __packed: },
    pub 12): VMMDEV_ASSERT_SIZE(vmmdev_heartbeat, 24 +,

// struct vmmdev_hgcmreq_header - vmmdev HGCM requests header.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmmdev_hgcmreq_header {
// Request header.
    pub header: vmmdev_request_header,
// HGCM flags.
    pub flags: u32,
// Result code.
    pub result: i32,
}

// struct vmmdev_hgcm_connect - HGCM connect request structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmmdev_hgcm_connect {
// HGCM request header.
    pub header: vmmdev_hgcmreq_header,
// IN: Description of service to connect to.
    pub loc: vmmdev_hgcm_service_location,
// OUT: Client identifier assigned by local instance of HGCM.
    pub client_id: u32,
}

// struct vmmdev_hgcm_disconnect - HGCM disconnect request structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmmdev_hgcm_disconnect {
// HGCM request header.
    pub header: vmmdev_hgcmreq_header,
// IN: Client identifier.
    pub client_id: u32,
}

pub const VMMDEV_HGCM_MAX_PARMS: c_int = 32;
// struct vmmdev_hgcm_call - HGCM call request structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmmdev_hgcm_call {
// request header
    pub header: vmmdev_hgcmreq_header,
// IN: Client identifier.
    pub client_id: u32,
// IN: Service function number.
    pub function: u32,
// IN: Number of parameters.
    pub parm_count: u32,
// Parameters follow in form: HGCMFunctionParameter32|64 parms[X];
}

//
// struct vmmdev_hgcm_cancel2 - HGCM cancel request structure, version 2.
//
// After the request header.rc will be:
//
// VINF_SUCCESS when cancelled.
// VERR_NOT_FOUND if the specified request cannot be found.
// VERR_INVALID_PARAMETER if the address is invalid valid.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmmdev_hgcm_cancel2 {
// Header.
    pub header: vmmdev_request_header,
// The physical address of the request to cancel.
    pub phys_req_to_cancel: u32,
}
