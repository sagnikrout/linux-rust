//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/virtio_rtc.h
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


// SPDX-License-Identifier: ((GPL-2.0+ WITH Linux-syscall-note) OR BSD-3-Clause)
//
// Copyright (C) 2022-2024 OpenSynergy GmbH
// Copyright (c) 2024 Qualcomm Innovation Center, Inc. All rights reserved.
//

// alarm feature
pub const VIRTIO_RTC_F_ALARM: c_int = 0;
// read request message types
pub const VIRTIO_RTC_REQ_READ: c_uint = 0x0001;
pub const VIRTIO_RTC_REQ_READ_CROSS: c_uint = 0x0002;
// control request message types
pub const VIRTIO_RTC_REQ_CFG: c_uint = 0x1000;
pub const VIRTIO_RTC_REQ_CLOCK_CAP: c_uint = 0x1001;
pub const VIRTIO_RTC_REQ_CROSS_CAP: c_uint = 0x1002;
pub const VIRTIO_RTC_REQ_READ_ALARM: c_uint = 0x1003;
pub const VIRTIO_RTC_REQ_SET_ALARM: c_uint = 0x1004;
pub const VIRTIO_RTC_REQ_SET_ALARM_ENABLED: c_uint = 0x1005;
// alarmq message types
pub const VIRTIO_RTC_NOTIF_ALARM: c_uint = 0x2000;
// Message headers
// common request header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_req_head {
    pub msg_type: __le16,
    pub reserved: [__u8; 6],
}

// common response header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_resp_head {
pub const VIRTIO_RTC_S_OK: c_int = 0;
pub const VIRTIO_RTC_S_EOPNOTSUPP: c_int = 2;
pub const VIRTIO_RTC_S_ENODEV: c_int = 3;
pub const VIRTIO_RTC_S_EINVAL: c_int = 4;
pub const VIRTIO_RTC_S_EIO: c_int = 5;
    pub status: __u8,
    pub reserved: [__u8; 7],
}

// common notification header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_notif_head {
    pub msg_type: __le16,
    pub reserved: [__u8; 6],
}

// read requests
// VIRTIO_RTC_REQ_READ message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_req_read {
    pub head: virtio_rtc_req_head,
    pub clock_id: __le16,
    pub reserved: [__u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_resp_read {
    pub head: virtio_rtc_resp_head,
    pub clock_reading: __le64,
}

// VIRTIO_RTC_REQ_READ_CROSS message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_req_read_cross {
    pub head: virtio_rtc_req_head,
    pub clock_id: __le16,
// Arm Generic Timer Counter-timer Virtual Count Register (CNTVCT_EL0)
pub const VIRTIO_RTC_COUNTER_ARM_VCT: c_int = 0;
// x86 Time-Stamp Counter
pub const VIRTIO_RTC_COUNTER_X86_TSC: c_int = 1;
// Invalid
pub const VIRTIO_RTC_COUNTER_INVALID: c_uint = 0xFF;
    pub hw_counter: __u8,
    pub reserved: [__u8; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_resp_read_cross {
    pub head: virtio_rtc_resp_head,
    pub clock_reading: __le64,
    pub counter_cycles: __le64,
}

// control requests
// VIRTIO_RTC_REQ_CFG message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_req_cfg {
    pub head: virtio_rtc_req_head,
// no request params
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_resp_cfg {
    pub head: virtio_rtc_resp_head,
// # of clocks -> clock ids < num_clocks are valid
    pub num_clocks: __le16,
    pub reserved: [__u8; 6],
}

// VIRTIO_RTC_REQ_CLOCK_CAP message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_req_clock_cap {
    pub head: virtio_rtc_req_head,
    pub clock_id: __le16,
    pub reserved: [__u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_resp_clock_cap {
    pub head: virtio_rtc_resp_head,
pub const VIRTIO_RTC_CLOCK_UTC: c_int = 0;
pub const VIRTIO_RTC_CLOCK_TAI: c_int = 1;
pub const VIRTIO_RTC_CLOCK_MONOTONIC: c_int = 2;
pub const VIRTIO_RTC_CLOCK_UTC_SMEARED: c_int = 3;
pub const VIRTIO_RTC_CLOCK_UTC_MAYBE_SMEARED: c_int = 4;
    pub type: __u8,
pub const VIRTIO_RTC_SMEAR_UNSPECIFIED: c_int = 0;
pub const VIRTIO_RTC_SMEAR_NOON_LINEAR: c_int = 1;
pub const VIRTIO_RTC_SMEAR_UTC_SLS: c_int = 2;
    pub leap_second_smearing: __u8,

    pub flags: __u8,
    pub reserved: [__u8; 5],
}

// VIRTIO_RTC_REQ_CROSS_CAP message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_req_cross_cap {
    pub head: virtio_rtc_req_head,
    pub clock_id: __le16,
    pub hw_counter: __u8,
    pub reserved: [__u8; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_resp_cross_cap {
    pub head: virtio_rtc_resp_head,

    pub flags: __u8,
    pub reserved: [__u8; 7],
}

// VIRTIO_RTC_REQ_READ_ALARM message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_req_read_alarm {
    pub head: virtio_rtc_req_head,
    pub clock_id: __le16,
    pub reserved: [__u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_resp_read_alarm {
    pub head: virtio_rtc_resp_head,
    pub alarm_time: __le64,

    pub flags: __u8,
    pub reserved: [__u8; 7],
}

// VIRTIO_RTC_REQ_SET_ALARM message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_req_set_alarm {
    pub head: virtio_rtc_req_head,
    pub alarm_time: __le64,
    pub clock_id: __le16,
// flag VIRTIO_RTC_FLAG_ALARM_ENABLED
    pub flags: __u8,
    pub reserved: [__u8; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_resp_set_alarm {
    pub head: virtio_rtc_resp_head,
// no response params
}

// VIRTIO_RTC_REQ_SET_ALARM_ENABLED message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_req_set_alarm_enabled {
    pub head: virtio_rtc_req_head,
    pub clock_id: __le16,
// flag VIRTIO_RTC_ALARM_ENABLED
    pub flags: __u8,
    pub reserved: [__u8; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_resp_set_alarm_enabled {
    pub head: virtio_rtc_resp_head,
// no response params
}

// Union of request types for requestq
#[repr(C)]
#[derive(Copy, Clone)]
pub union virtio_rtc_req_requestq {
    pub read: virtio_rtc_req_read,
    pub read_cross: virtio_rtc_req_read_cross,
    pub cfg: virtio_rtc_req_cfg,
    pub clock_cap: virtio_rtc_req_clock_cap,
    pub cross_cap: virtio_rtc_req_cross_cap,
    pub read_alarm: virtio_rtc_req_read_alarm,
    pub set_alarm: virtio_rtc_req_set_alarm,
    pub set_alarm_enabled: virtio_rtc_req_set_alarm_enabled,
}

// Union of response types for requestq
#[repr(C)]
#[derive(Copy, Clone)]
pub union virtio_rtc_resp_requestq {
    pub read: virtio_rtc_resp_read,
    pub read_cross: virtio_rtc_resp_read_cross,
    pub cfg: virtio_rtc_resp_cfg,
    pub clock_cap: virtio_rtc_resp_clock_cap,
    pub cross_cap: virtio_rtc_resp_cross_cap,
    pub read_alarm: virtio_rtc_resp_read_alarm,
    pub set_alarm: virtio_rtc_resp_set_alarm,
    pub set_alarm_enabled: virtio_rtc_resp_set_alarm_enabled,
}

// alarmq notifications
// VIRTIO_RTC_NOTIF_ALARM notification
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_notif_alarm {
    pub head: virtio_rtc_notif_head,
    pub clock_id: __le16,
    pub reserved: [__u8; 6],
}

// Union of notification types for alarmq
#[repr(C)]
#[derive(Copy, Clone)]
pub union virtio_rtc_notif_alarmq {
    pub alarm: virtio_rtc_notif_alarm,
}
