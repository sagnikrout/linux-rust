//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/vbox_vmmdev_types.h
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
// Virtual Device for Guest <-> VMM/Host communication, type definitions
// which are also used for the vboxguest ioctl interface / by vboxsf
//
// Copyright (C) 2006-2016 Oracle Corporation
//

//
// We cannot use linux' compiletime_assert here because it expects to be used
// inside a function only. Use a typedef to a char array with a negative size.
//

// enum vmmdev_request_type - VMMDev request types.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vmmdev_request_type {
    VMMDEVREQ_INVALID_REQUEST              =  0,
    VMMDEVREQ_GET_MOUSE_STATUS             =  1,
    VMMDEVREQ_SET_MOUSE_STATUS             =  2,
    VMMDEVREQ_SET_POINTER_SHAPE            =  3,
    VMMDEVREQ_GET_HOST_VERSION             =  4,
    VMMDEVREQ_IDLE                         =  5,
    VMMDEVREQ_GET_HOST_TIME                = 10,
    VMMDEVREQ_GET_HYPERVISOR_INFO          = 20,
    VMMDEVREQ_SET_HYPERVISOR_INFO          = 21,
    VMMDEVREQ_REGISTER_PATCH_MEMORY        = 22, /* since version 3.0.6 */
    VMMDEVREQ_DEREGISTER_PATCH_MEMORY      = 23, /* since version 3.0.6 */
    VMMDEVREQ_SET_POWER_STATUS             = 30,
    VMMDEVREQ_ACKNOWLEDGE_EVENTS           = 41,
    VMMDEVREQ_CTL_GUEST_FILTER_MASK        = 42,
    VMMDEVREQ_REPORT_GUEST_INFO            = 50,
    VMMDEVREQ_REPORT_GUEST_INFO2           = 58, /* since version 3.2.0 */
    VMMDEVREQ_REPORT_GUEST_STATUS          = 59, /* since version 3.2.8 */
    VMMDEVREQ_REPORT_GUEST_USER_STATE      = 74, /* since version 4.3 */
// Retrieve a display resize request sent by the host, deprecated.
    VMMDEVREQ_GET_DISPLAY_CHANGE_REQ       = 51,
    VMMDEVREQ_VIDEMODE_SUPPORTED           = 52,
    VMMDEVREQ_GET_HEIGHT_REDUCTION         = 53,
//
// @VMMDEVREQ_GET_DISPLAY_CHANGE_REQ2:
// Retrieve a display resize request sent by the host.
//
// Queries a display resize request sent from the host.  If the
// event_ack member is sent to true and there is an unqueried request
// available for one of the virtual display then that request will
// be returned.  If several displays have unqueried requests the lowest
// numbered display will be chosen first.  Only the most recent unseen
// request for each display is remembered.
// If event_ack is set to false, the last host request queried with
// event_ack set is resent, or failing that the most recent received
// from the host.  If no host request was ever received then all zeros
// are returned.
//
    VMMDEVREQ_GET_DISPLAY_CHANGE_REQ2      = 54,
    VMMDEVREQ_REPORT_GUEST_CAPABILITIES    = 55,
    VMMDEVREQ_SET_GUEST_CAPABILITIES       = 56,
    VMMDEVREQ_VIDEMODE_SUPPORTED2          = 57, /* since version 3.2.0 */
    VMMDEVREQ_GET_DISPLAY_CHANGE_REQEX     = 80, /* since version 4.2.4 */
    VMMDEVREQ_GET_DISPLAY_CHANGE_REQ_MULTI = 81,
    VMMDEVREQ_HGCM_CONNECT                 = 60,
    VMMDEVREQ_HGCM_DISCONNECT              = 61,
    VMMDEVREQ_HGCM_CALL32                  = 62,
    VMMDEVREQ_HGCM_CALL64                  = 63,
    VMMDEVREQ_HGCM_CANCEL                  = 64,
    VMMDEVREQ_HGCM_CANCEL2                 = 65,
    VMMDEVREQ_VIDEO_ACCEL_ENABLE           = 70,
    VMMDEVREQ_VIDEO_ACCEL_FLUSH            = 71,
    VMMDEVREQ_VIDEO_SET_VISIBLE_REGION     = 72,
    VMMDEVREQ_GET_SEAMLESS_CHANGE_REQ      = 73,
    VMMDEVREQ_QUERY_CREDENTIALS            = 100,
    VMMDEVREQ_REPORT_CREDENTIALS_JUDGEMENT = 101,
    VMMDEVREQ_REPORT_GUEST_STATS           = 110,
    VMMDEVREQ_GET_MEMBALLOON_CHANGE_REQ    = 111,
    VMMDEVREQ_GET_STATISTICS_CHANGE_REQ    = 112,
    VMMDEVREQ_CHANGE_MEMBALLOON            = 113,
    VMMDEVREQ_GET_VRDPCHANGE_REQ           = 150,
    VMMDEVREQ_LOG_STRING                   = 200,
    VMMDEVREQ_GET_CPU_HOTPLUG_REQ          = 210,
    VMMDEVREQ_SET_CPU_HOTPLUG_STATUS       = 211,
    VMMDEVREQ_REGISTER_SHARED_MODULE       = 212,
    VMMDEVREQ_UNREGISTER_SHARED_MODULE     = 213,
    VMMDEVREQ_CHECK_SHARED_MODULES         = 214,
    VMMDEVREQ_GET_PAGE_SHARING_STATUS      = 215,
    VMMDEVREQ_DEBUG_IS_PAGE_SHARED         = 216,
    VMMDEVREQ_GET_SESSION_ID               = 217, /* since version 3.2.8 */
    VMMDEVREQ_WRITE_COREDUMP               = 218,
    VMMDEVREQ_GUEST_HEARTBEAT              = 219,
    VMMDEVREQ_HEARTBEAT_CONFIGURE          = 220,
    VMMDEVREQ_NT_BUG_CHECK                 = 221,
    VMMDEVREQ_VIDEO_UPDATE_MONITOR_POSITIONS = 222,
// Ensure the enum is a 32 bit data-type
    VMMDEVREQ_SIZEHACK                     = 0x7fffffff
}

// vmmdev_request_header.requestor defines
// Requestor user not given.
pub const VMMDEV_REQUESTOR_USR_NOT_GIVEN: c_uint = 0x00000000;
// The kernel driver (vboxguest) is the requestor.
pub const VMMDEV_REQUESTOR_USR_DRV: c_uint = 0x00000001;
// Some other kernel driver is the requestor.
pub const VMMDEV_REQUESTOR_USR_DRV_OTHER: c_uint = 0x00000002;
// The root or a admin user is the requestor.
pub const VMMDEV_REQUESTOR_USR_ROOT: c_uint = 0x00000003;
// Regular joe user is making the request.
pub const VMMDEV_REQUESTOR_USR_USER: c_uint = 0x00000006;
// User classification mask.
pub const VMMDEV_REQUESTOR_USR_MASK: c_uint = 0x00000007;
// Kernel mode request. Note this is 0, check for !USERMODE instead.
pub const VMMDEV_REQUESTOR_KERNEL: c_uint = 0x00000000;
// User mode request.
pub const VMMDEV_REQUESTOR_USERMODE: c_uint = 0x00000008;
// User or kernel mode classification mask.
pub const VMMDEV_REQUESTOR_MODE_MASK: c_uint = 0x00000008;
// Don't know the physical console association of the requestor.
pub const VMMDEV_REQUESTOR_CON_DONT_KNOW: c_uint = 0x00000000;
//
// The request originates with a process that is NOT associated with the
// physical console.
//
pub const VMMDEV_REQUESTOR_CON_NO: c_uint = 0x00000010;
// Requestor process is associated with the physical console.
pub const VMMDEV_REQUESTOR_CON_YES: c_uint = 0x00000020;
// Console classification mask.
pub const VMMDEV_REQUESTOR_CON_MASK: c_uint = 0x00000030;
// Requestor is member of special VirtualBox user group.
pub const VMMDEV_REQUESTOR_GRP_VBOX: c_uint = 0x00000080;
// Note: trust level is for windows guests only, linux always uses not-given
// Requestor trust level: Unspecified
pub const VMMDEV_REQUESTOR_TRUST_NOT_GIVEN: c_uint = 0x00000000;
// Requestor trust level: Untrusted (SID S-1-16-0)
pub const VMMDEV_REQUESTOR_TRUST_UNTRUSTED: c_uint = 0x00001000;
// Requestor trust level: Untrusted (SID S-1-16-4096)
pub const VMMDEV_REQUESTOR_TRUST_LOW: c_uint = 0x00002000;
// Requestor trust level: Medium (SID S-1-16-8192)
pub const VMMDEV_REQUESTOR_TRUST_MEDIUM: c_uint = 0x00003000;
// Requestor trust level: Medium plus (SID S-1-16-8448)
pub const VMMDEV_REQUESTOR_TRUST_MEDIUM_PLUS: c_uint = 0x00004000;
// Requestor trust level: High (SID S-1-16-12288)
pub const VMMDEV_REQUESTOR_TRUST_HIGH: c_uint = 0x00005000;
// Requestor trust level: System (SID S-1-16-16384)
pub const VMMDEV_REQUESTOR_TRUST_SYSTEM: c_uint = 0x00006000;
// Requestor trust level >= Protected (SID S-1-16-20480, S-1-16-28672)
pub const VMMDEV_REQUESTOR_TRUST_PROTECTED: c_uint = 0x00007000;
// Requestor trust level mask
pub const VMMDEV_REQUESTOR_TRUST_MASK: c_uint = 0x00007000;
// Requestor is using the less trusted user device node (/dev/vboxuser)
pub const VMMDEV_REQUESTOR_USER_DEVICE: c_uint = 0x00008000;
// HGCM service location types.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vmmdev_hgcm_service_location_type {
    VMMDEV_HGCM_LOC_INVALID    = 0,
    VMMDEV_HGCM_LOC_LOCALHOST  = 1,
    VMMDEV_HGCM_LOC_LOCALHOST_EXISTING = 2,
// Ensure the enum is a 32 bit data-type
    VMMDEV_HGCM_LOC_SIZEHACK   = 0x7fffffff
}

// HGCM host service location.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmmdev_hgcm_service_location_localhost {
// Service name
    pub service_name: [c_char; 128],
}

// HGCM service location.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmmdev_hgcm_service_location {
// Type of the location.
    pub type: vmmdev_hgcm_service_location_type,
    pub localhost: vmmdev_hgcm_service_location_localhost,
    pub u: },
}

// HGCM function parameter type.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vmmdev_hgcm_function_parameter_type {
    VMMDEV_HGCM_PARM_TYPE_INVALID            = 0,
    VMMDEV_HGCM_PARM_TYPE_32BIT              = 1,
    VMMDEV_HGCM_PARM_TYPE_64BIT              = 2,
// Deprecated Doesn't work, use PAGELIST.
    VMMDEV_HGCM_PARM_TYPE_PHYSADDR           = 3,
// In and Out, user-memory
    VMMDEV_HGCM_PARM_TYPE_LINADDR            = 4,
// In, user-memory  (read;  host<-guest)
    VMMDEV_HGCM_PARM_TYPE_LINADDR_IN         = 5,
// Out, user-memory (write; host->guest)
    VMMDEV_HGCM_PARM_TYPE_LINADDR_OUT        = 6,
// In and Out, kernel-memory
    VMMDEV_HGCM_PARM_TYPE_LINADDR_KERNEL     = 7,
// In, kernel-memory  (read;  host<-guest)
    VMMDEV_HGCM_PARM_TYPE_LINADDR_KERNEL_IN  = 8,
// Out, kernel-memory (write; host->guest)
    VMMDEV_HGCM_PARM_TYPE_LINADDR_KERNEL_OUT = 9,
// Physical addresses of locked pages for a buffer.
    VMMDEV_HGCM_PARM_TYPE_PAGELIST           = 10,
// Ensure the enum is a 32 bit data-type
    VMMDEV_HGCM_PARM_TYPE_SIZEHACK           = 0x7fffffff
}

// HGCM function parameter, 32-bit client.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmmdev_hgcm_function_parameter32 {
    pub type: vmmdev_hgcm_function_parameter_type,
    pub value32: __u32,
    pub value64: __u64,
    pub size: __u32,
    pub phys_addr: __u32,
    pub linear_addr: __u32,
    pub u: },
    pub pointer: },
// Size of the buffer described by the page list.
    pub size: __u32,
// Relative to the request header.
    pub offset: __u32,
    pub page_list: },
    pub u: } __packed,
    pub __packed: },
    pub 8): VMMDEV_ASSERT_SIZE(vmmdev_hgcm_function_parameter32, 4 +,
// HGCM function parameter, 64-bit client.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmmdev_hgcm_function_parameter64 {
    pub type: vmmdev_hgcm_function_parameter_type,
    pub value32: __u32,
    pub value64: __u64,
    pub size: __u32,
    pub phys_addr: __u64,
    pub linear_addr: __u64,
    pub u: } __packed,
    pub pointer: } __packed,
// Size of the buffer described by the page list.
    pub size: __u32,
// Relative to the request header.
    pub offset: __u32,
    pub page_list: },
    pub u: } __packed,
    pub __packed: },
    pub 12): VMMDEV_ASSERT_SIZE(vmmdev_hgcm_function_parameter64, 4 +,

pub const VMMDEV_HGCM_F_PARM_DIRECTION_NONE: c_uint = 0x00000000U;
pub const VMMDEV_HGCM_F_PARM_DIRECTION_TO_HOST: c_uint = 0x00000001U;
pub const VMMDEV_HGCM_F_PARM_DIRECTION_FROM_HOST: c_uint = 0x00000002U;
pub const VMMDEV_HGCM_F_PARM_DIRECTION_BOTH: c_uint = 0x00000003U;
//
// struct vmmdev_hgcm_pagelist - VMMDEV_HGCM_PARM_TYPE_PAGELIST parameters
// point to this structure to actually describe the buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmmdev_hgcm_pagelist {
    pub /: *mut *mut *mut *mut __u32 flags; / VMMDEV_HGCM_F_PARM_.,
    pub /: *mut *mut *mut __u16 offset_first_page; / Data offset in the first page.,
    pub /: *mut *mut *mut __u16 page_count; / Number of pages.,
    pub /: *mut *mut *mut __u64 unused; / Deprecated place-holder for first "pages" entry.,
    pub /: *mut *mut *mut __DECLARE_FLEX_ARRAY(__u64, pages); / Page addresses.,
}
