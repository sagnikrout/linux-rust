//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/sysctl.h
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
// sysctl.h: General linux system control interface
//
// Begun 24 March 1995, Stephen Tweedie
//
// WARNING:
// The values in this file are exported to user space via
// the sysctl() binary interface.  Do *NOT* change the
// numbering of any existing values here, and do not change
// any numbers within any one set of values.  If you have to
// redefine an existing interface, use a new number for it.
// The kernel will then return -ENOTDIR to any application using
// the old binary interface.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __sysctl_args {
    pub name: *mut int __user,
    pub nlen: c_int,
    pub oldval: *mut void __user,
    pub oldlenp: *mut size_t __user,
    pub newval: *mut void __user,
    pub newlen: usize,
    pub __unused: [c_ulong; 4],
}

// Define sysctl names first
// Top-level names:
// CTL_BUS names:
// /proc/sys/fs/inotify/
// CTL_KERN names:
// CTL_VM names:
// CTL_NET names:
// /proc/sys/kernel/random
// /proc/sys/kernel/pty
// /proc/sys/bus/isa
// /proc/sys/net/core
// was	NET_CORE_DESTROY_DELAY
// /proc/sys/net/ethernet
// /proc/sys/net/802
// /proc/sys/net/unix
// /proc/sys/net/netfilter
// /proc/sys/net/ipv4
// v2.0 compatibile variables
// And device ifindices ...
// /proc/sys/net/ipv4/netfilter
// /proc/sys/net/ipv6
// /proc/sys/net/ipv6/icmp
// /proc/sys/net/<protocol>/neigh/<dev>
// /proc/sys/net/dccp
// /proc/sys/net/ipx
// /proc/sys/net/llc
// /proc/sys/net/llc/llc2
// /proc/sys/net/llc/station
// /proc/sys/net/llc/llc2/timeout
// /proc/sys/net/appletalk
// /proc/sys/net/netrom
// /proc/sys/net/ax25
// /proc/sys/net/rose
// /proc/sys/net/x25
// /proc/sys/net/token-ring
// /proc/sys/net/decnet/
// /proc/sys/net/decnet/conf/<dev>
// ... and ifindex of devices
// /proc/sys/net/decnet/conf/<dev>/
// /proc/sys/net/sctp
// /proc/sys/net/bridge
// CTL_FS names:
// /proc/sys/fs/quota/
// CTL_DEBUG names:
// CTL_DEV names:
// /proc/sys/dev/cdrom
// /proc/sys/dev/parport
// /proc/sys/dev/raid
// /proc/sys/dev/parport/default
// /proc/sys/dev/parport/parport n
// /proc/sys/dev/parport/parport n/devices/
// /proc/sys/dev/parport/parport n/devices/device n
// /proc/sys/dev/mac_hid
// /proc/sys/dev/scsi
// /proc/sys/dev/ipmi
// /proc/sys/abi
