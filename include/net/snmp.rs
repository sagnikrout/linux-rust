//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/snmp.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// SNMP MIB entries for the IP subsystem.
//
// Alan Cox <gw4pts@gw4pts.ampr.org>
//
// We don't chose to implement SNMP in the kernel (this would
// be silly as SNMP is a pain in the backside in places). We do
// however need to collect the MIB statistics and export them
// out of /proc (eventually)
//

//
// Mibs are stored in array of unsigned long.
//
// struct snmp_mib{}
// - list of entries for particular API (such as /proc/net/snmp)
// - name of entries.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snmp_mib {
    pub name: *const c_char,
    pub entry: c_int,
}

//
// We use unsigned longs for most mibs but u64 for ipstats.
//

// IPstats

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipstats_mib {
// mibs[] must be first field of struct ipstats_mib
    pub mibs: [u64; IPSTATS_MIB_MAX],
    pub syncp: u64_stats_sync,
}

// ICMP

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmp_mib {
    pub mibs: [c_ulong; ICMP_MIB_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmpmsg_mib {
    pub mibs: [atomic_long_t; ICMPMSG_MIB_MAX],
}

// ICMP6 (IPv6-ICMP)

// per network ns counters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmpv6_mib {
    pub mibs: [c_ulong; ICMP6_MIB_MAX],
}

// per device counters, (shared on all cpus)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmpv6_mib_device {
    pub mibs: [atomic_long_t; ICMP6_MIB_MAX],
}

// per network ns counters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmpv6msg_mib {
    pub mibs: [atomic_long_t; ICMP6MSG_MIB_MAX],
}

// per device counters, (shared on all cpus)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmpv6msg_mib_device {
    pub mibs: [atomic_long_t; ICMP6MSG_MIB_MAX],
}

// TCP

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_mib {
    pub mibs: [c_ulong; TCP_MIB_MAX],
}

// UDP

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udp_mib {
    pub mibs: [c_ulong; UDP_MIB_MAX],
}

// Linux

#[repr(C)]
#[derive(Copy, Clone)]
pub struct linux_mib {
    pub mibs: [c_ulong; LINUX_MIB_MAX],
}

// Linux Xfrm

#[repr(C)]
#[derive(Copy, Clone)]
pub struct linux_xfrm_mib {
    pub mibs: [c_ulong; LINUX_MIB_XFRMMAX],
}

// Linux TLS

#[repr(C)]
#[derive(Copy, Clone)]
pub struct linux_tls_mib {
    pub mibs: [c_ulong; LINUX_MIB_TLSMAX],
}

