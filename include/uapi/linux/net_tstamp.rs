//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/net_tstamp.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Userspace API for hardware time stamping of network packets
//
// Copyright (C) 2008,2009 Intel Corporation
// Author: Patrick Ohly <patrick.ohly@intel.com>
//

//
// Possible type of hwtstamp provider. Mainly "precise" the default one
// is for IEEE 1588 quality and "approx" is for NICs DMA point.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hwtstamp_provider_qualifier {
    HWTSTAMP_PROVIDER_QUALIFIER_PRECISE,
    HWTSTAMP_PROVIDER_QUALIFIER_APPROX,

    HWTSTAMP_PROVIDER_QUALIFIER_CNT,
}

// SO_TIMESTAMPING flags
//
// SO_TIMESTAMPING flags are either for recording a packet timestamp or for
// reporting the timestamp to user space.
// Recording flags can be set both via socket options and control messages.
//

//
// struct so_timestamping - SO_TIMESTAMPING parameter
//
// @flags:	SO_TIMESTAMPING flags
// @bind_phc:	Index of PTP virtual clock bound to sock. This is available
// if flag SOF_TIMESTAMPING_BIND_PHC is set.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct so_timestamping {
    pub flags: c_int,
    pub bind_phc: c_int,
}

//
// struct hwtstamp_config - %SIOCGHWTSTAMP and %SIOCSHWTSTAMP parameter
//
// @flags:	one of HWTSTAMP_FLAG_
// @tx_type:	one of HWTSTAMP_TX_
// @rx_filter:	one of HWTSTAMP_FILTER_
//
// %SIOCGHWTSTAMP and %SIOCSHWTSTAMP expect a &struct ifreq with a
// ifr_data pointer to this structure.  For %SIOCSHWTSTAMP, if the
// driver or hardware does not support the requested @rx_filter value,
// the driver may use a more general filter mode.  In this case
// @rx_filter will indicate the actual mode on return.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwtstamp_config {
    pub flags: c_int,
    pub tx_type: c_int,
    pub rx_filter: c_int,
}

// possible values for hwtstamp_config->flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hwtstamp_flags {
//
// With this flag, the user could get bond active interface's
// PHC index. Note this PHC index is not stable as when there
// is a failover, the bond active interface will be changed, so
// will be the PHC index.
//
    HWTSTAMP_FLAG_BONDED_PHC_INDEX = (1<<0),

    HWTSTAMP_FLAG_LAST = HWTSTAMP_FLAG_BONDED_PHC_INDEX,
    HWTSTAMP_FLAG_MASK = (HWTSTAMP_FLAG_LAST - 1) | HWTSTAMP_FLAG_LAST
}

// possible values for hwtstamp_config->tx_type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hwtstamp_tx_types {
//
// No outgoing packet will need hardware time stamping;
// should a packet arrive which asks for it, no hardware
// time stamping will be done.
//
    HWTSTAMP_TX_OFF,

//
// Enables hardware time stamping for outgoing packets;
// the sender of the packet decides which are to be
// time stamped by setting %SOF_TIMESTAMPING_TX_SOFTWARE
// before sending the packet.
//
    HWTSTAMP_TX_ON,

//
// Enables time stamping for outgoing packets just as
// HWTSTAMP_TX_ON does, but also enables time stamp insertion
// directly into Sync packets. In this case, transmitted Sync
// packets will not received a time stamp via the socket error
// queue.
//
    HWTSTAMP_TX_ONESTEP_SYNC,

//
// Same as HWTSTAMP_TX_ONESTEP_SYNC, but also enables time
// stamp insertion directly into PDelay_Resp packets. In this
// case, neither transmitted Sync nor PDelay_Resp packets will
// receive a time stamp via the socket error queue.
//
    HWTSTAMP_TX_ONESTEP_P2P,

// add new constants above here
    __HWTSTAMP_TX_CNT
}

// possible values for hwtstamp_config->rx_filter
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hwtstamp_rx_filters {
// time stamp no incoming packet at all
    HWTSTAMP_FILTER_NONE,

// time stamp any incoming packet
    HWTSTAMP_FILTER_ALL,

// return value: time stamp all packets requested plus some others
    HWTSTAMP_FILTER_SOME,

// PTP v1, UDP, any kind of event packet
    HWTSTAMP_FILTER_PTP_V1_L4_EVENT,
// PTP v1, UDP, Sync packet
    HWTSTAMP_FILTER_PTP_V1_L4_SYNC,
// PTP v1, UDP, Delay_req packet
    HWTSTAMP_FILTER_PTP_V1_L4_DELAY_REQ,
// PTP v2, UDP, any kind of event packet
    HWTSTAMP_FILTER_PTP_V2_L4_EVENT,
// PTP v2, UDP, Sync packet
    HWTSTAMP_FILTER_PTP_V2_L4_SYNC,
// PTP v2, UDP, Delay_req packet
    HWTSTAMP_FILTER_PTP_V2_L4_DELAY_REQ,

// 802.AS1, Ethernet, any kind of event packet
    HWTSTAMP_FILTER_PTP_V2_L2_EVENT,
// 802.AS1, Ethernet, Sync packet
    HWTSTAMP_FILTER_PTP_V2_L2_SYNC,
// 802.AS1, Ethernet, Delay_req packet
    HWTSTAMP_FILTER_PTP_V2_L2_DELAY_REQ,

// PTP v2/802.AS1, any layer, any kind of event packet
    HWTSTAMP_FILTER_PTP_V2_EVENT,
// PTP v2/802.AS1, any layer, Sync packet
    HWTSTAMP_FILTER_PTP_V2_SYNC,
// PTP v2/802.AS1, any layer, Delay_req packet
    HWTSTAMP_FILTER_PTP_V2_DELAY_REQ,

// NTP, UDP, all versions and packet modes
    HWTSTAMP_FILTER_NTP_ALL,

// add new constants above here
    __HWTSTAMP_FILTER_CNT
}

// SCM_TIMESTAMPING_PKTINFO control message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scm_ts_pktinfo {
    pub if_index: __u32,
    pub pkt_length: __u32,
    pub reserved: [__u32; 2],
}

//
// SO_TXTIME gets a struct sock_txtime with flags being an integer bit
// field comprised of these values.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum txtime_flags {
    SOF_TXTIME_DEADLINE_MODE = (1 << 0),
    SOF_TXTIME_REPORT_ERRORS = (1 << 1),

    SOF_TXTIME_FLAGS_LAST = SOF_TXTIME_REPORT_ERRORS,
    SOF_TXTIME_FLAGS_MASK = (SOF_TXTIME_FLAGS_LAST - 1) |
    SOF_TXTIME_FLAGS_LAST
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sock_txtime {
    pub /: *mut *mut __kernel_clockid_t clockid;/ reference clockid,
    pub /: *mut *mut __u32 flags; / as defined by enum txtime_flags,
}
