//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/if.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// INET		An implementation of the TCP/IP protocol suite for the LINUX
// operating system.  INET is implemented using the  BSD Socket
// interface as the means of communication with the user level.
//
// Global definitions for the INET interface module.
//
// Version:	@(#)if.h	1.0.2	04/18/93
//
// Authors:	Original taken from Berkeley UNIX 4.3, (c) UCB 1982-1988
// Ross Biro
// Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//

pub const IFNAMSIZ: c_int = 16;

pub const IFALIASZ: c_int = 256;
pub const ALTIFNAMSIZ: c_int = 128;

// For glibc compatibility. An empty enum does not compile.

//
// enum net_device_flags - &struct net_device flags
//
// These are the &struct net_device flags, they can be set by drivers, the
// kernel and some can be triggered by userspace. Userspace can query and
// set these flags using userspace utilities but there is also a sysfs
// entry available for all dev flags which can be queried and set. These flags
// are shared for all types of net_devices. The sysfs entries are available
// via /sys/class/net/<dev>/flags. Flags which can be toggled through sysfs
// are annotated below, note that only a few flags can be toggled and some
// other flags are always preserved from the original net_device flags
// even if you try to set them via sysfs. Flags which are always preserved
// are kept under the flag grouping @IFF_VOLATILE. Flags which are volatile
// are annotated below as such.
//
// You should have a pretty good reason to be extending these flags.
//
// @IFF_UP: interface is up. Can be toggled through sysfs.
// @IFF_BROADCAST: broadcast address valid. Volatile.
// @IFF_DEBUG: turn on debugging. Can be toggled through sysfs.
// @IFF_LOOPBACK: is a loopback net. Volatile.
// @IFF_POINTOPOINT: interface is has p-p link. Volatile.
// @IFF_NOTRAILERS: avoid use of trailers. Can be toggled through sysfs.
// Volatile.
// @IFF_RUNNING: interface RFC2863 OPER_UP. Volatile.
// @IFF_NOARP: no ARP protocol. Can be toggled through sysfs. Volatile.
// @IFF_PROMISC: receive all packets. Can be toggled through sysfs.
// @IFF_ALLMULTI: receive all multicast packets. Can be toggled through
// sysfs.
// @IFF_MASTER: master of a load balancer. Volatile.
// @IFF_SLAVE: slave of a load balancer. Volatile.
// @IFF_MULTICAST: Supports multicast. Can be toggled through sysfs.
// @IFF_PORTSEL: can set media type. Can be toggled through sysfs.
// @IFF_AUTOMEDIA: auto media select active. Can be toggled through sysfs.
// @IFF_DYNAMIC: dialup device with changing addresses. Can be toggled
// through sysfs.
// @IFF_LOWER_UP: driver signals L1 up. Volatile.
// @IFF_DORMANT: driver signals dormant. Volatile.
// @IFF_ECHO: echo sent packets. Volatile.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum net_device_flags {
// for compatibility with glibc net/if.h

    IFF_UP				= 1<<0,  /* sysfs */
    IFF_BROADCAST			= 1<<1,  /* volatile */
    IFF_DEBUG			= 1<<2,  /* sysfs */
    IFF_LOOPBACK			= 1<<3,  /* volatile */
    IFF_POINTOPOINT			= 1<<4,  /* volatile */
    IFF_NOTRAILERS			= 1<<5,  /* sysfs */
    IFF_RUNNING			= 1<<6,  /* volatile */
    IFF_NOARP			= 1<<7,  /* sysfs */
    IFF_PROMISC			= 1<<8,  /* sysfs */
    IFF_ALLMULTI			= 1<<9,  /* sysfs */
    IFF_MASTER			= 1<<10, /* volatile */
    IFF_SLAVE			= 1<<11, /* volatile */
    IFF_MULTICAST			= 1<<12, /* sysfs */
    IFF_PORTSEL			= 1<<13, /* sysfs */
    IFF_AUTOMEDIA			= 1<<14, /* sysfs */
    IFF_DYNAMIC			= 1<<15, /* sysfs */

    IFF_LOWER_UP			= 1<<16, /* volatile */
    IFF_DORMANT			= 1<<17, /* volatile */
    IFF_ECHO			= 1<<18, /* volatile */

}

// for compatibility with glibc net/if.h

pub const IF_GET_IFACE: c_uint = 0x0001		/* for querying only */;
pub const IF_GET_PROTO: c_uint = 0x0002;
// For definitions see hdlc.h
pub const IF_IFACE_V35: c_uint = 0x1000		/* V.35 serial interface	*/;
pub const IF_IFACE_V24: c_uint = 0x1001		/* V.24 serial interface	*/;
pub const IF_IFACE_X21: c_uint = 0x1002		/* X.21 serial interface	*/;
pub const IF_IFACE_T1: c_uint = 0x1003		/* T1 telco serial interface	*/;
pub const IF_IFACE_E1: c_uint = 0x1004		/* E1 telco serial interface	*/;
pub const IF_IFACE_SYNC_SERIAL: c_uint = 0x1005	/* can't be set by software	*/;
pub const IF_IFACE_X21D: c_uint = 0x1006          /* X.21 Dual Clocking (FarSite) */;
// For definitions see hdlc.h
pub const IF_PROTO_HDLC: c_uint = 0x2000		/* raw HDLC protocol		*/;
pub const IF_PROTO_PPP: c_uint = 0x2001		/* PPP protocol			*/;
pub const IF_PROTO_CISCO: c_uint = 0x2002		/* Cisco HDLC protocol		*/;
pub const IF_PROTO_FR: c_uint = 0x2003		/* Frame Relay protocol		*/;
pub const IF_PROTO_FR_ADD_PVC: c_uint = 0x2004	/*    Create FR PVC		*/;
pub const IF_PROTO_FR_DEL_PVC: c_uint = 0x2005	/*    Delete FR PVC		*/;
pub const IF_PROTO_X25: c_uint = 0x2006		/* X.25				*/;
pub const IF_PROTO_HDLC_ETH: c_uint = 0x2007	/* raw HDLC, Ethernet emulation	*/;
pub const IF_PROTO_FR_ADD_ETH_PVC: c_uint = 0x2008	/*  Create FR Ethernet-bridged PVC */;
pub const IF_PROTO_FR_DEL_ETH_PVC: c_uint = 0x2009	/*  Delete FR Ethernet-bridged PVC */;
pub const IF_PROTO_FR_PVC: c_uint = 0x200A		/* for reading PVC status	*/;
pub const IF_PROTO_FR_ETH_PVC: c_uint = 0x200B;
pub const IF_PROTO_RAW: c_uint = 0x200C          /* RAW Socket                   */;
// RFC 2863 operational status
// link modes
//
// Device mapping structure. I'd just gone off and designed a
// beautiful scheme using only loadable modules with arguments
// for driver options and along come the PCMCIA people 8)
//
// Ah well. The get() side of this is good for WDSETUP, and it'll
// be handy for debugging things. The set side is fine for now and
// being very small might be worth keeping for clean configuration.
//
// for compatibility with glibc net/if.h

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifmap {
    pub mem_start: c_ulong,
    pub mem_end: c_ulong,
    pub base_addr: c_ushort,
    pub irq: c_uchar,
    pub dma: c_uchar,
    pub port: c_uchar,
// 3 bytes spare
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct if_settings {
    pub /: *mut *mut unsigned int type; / Type of physical device or protocol,
    pub /: *mut *mut unsigned int size; / Size of the data allocated by the caller,
// {atm/eth/dsl}_settings anyone ?
    pub raw_hdlc: *mut raw_hdlc_proto __user,
    pub cisco: *mut cisco_proto __user,
    pub fr: *mut fr_proto __user,
    pub fr_pvc: *mut fr_proto_pvc __user,
    pub fr_pvc_info: *mut fr_proto_pvc_info __user,
    pub x25: *mut x25_hdlc_proto __user,
// interface settings
    pub sync: *mut sync_serial_settings __user,
    pub te1: *mut te1_settings __user,
    pub ifs_ifsu: },
}

//
// Interface request structure used for socket
// ioctl's.  All interface ioctl's must have parameter
// definitions which begin with ifr_name.  The
// remainder may be interface specific.
//
// for compatibility with glibc net/if.h

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifreq {
pub const IFHWADDRLEN: c_int = 6;
    pub /: *mut *mut char ifrn_name[IFNAMSIZ]; / if name, e.g. "en0",
    pub ifr_ifrn: },
    pub ifru_addr: sockaddr,
    pub ifru_dstaddr: sockaddr,
    pub ifru_broadaddr: sockaddr,
    pub ifru_netmask: sockaddr,
    pub ifru_hwaddr: sockaddr,
    pub ifru_flags: c_short,
    pub ifru_ivalue: c_int,
    pub ifru_mtu: c_int,
    pub ifru_map: ifmap,
    pub /: *mut *mut char ifru_slave[IFNAMSIZ]; / Just fits the size,
    pub ifru_newname: [c_char; IFNAMSIZ],
    pub ifru_data: *mut *mut void __user,
    pub ifru_settings: if_settings,
    pub ifr_ifru: },
}

//
// Structure used in SIOCGIFCONF request.
// Used to retrieve interface configuration
// for machine (useful for programs which
// must know all networks accessible).
//
// for compatibility with glibc net/if.h

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifconf {
    pub /: *mut *mut int ifc_len; / size of buffer,
    pub ifcu_buf: *mut char __user,
    pub ifcu_req: *mut ifreq __user,
    pub ifc_ifcu: },
}

