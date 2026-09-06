//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/sockios.h
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
// Definitions of the socket-level I/O control calls.
//
// Version:	@(#)sockios.h	1.0.2	03/09/93
//
// Authors:	Ross Biro
// Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//

// Linux-specific socket ioctls

pub const SOCK_IOC_TYPE: c_uint = 0x89;
//
// the timeval/timespec data structure layout is defined by libc,
// so we need to cover both possible versions on 32-bit.
//
// Get stamp (timeval)

// Get stamp (timespec)

// on 64-bit and x32, avoid the ?: operator

// Routing table calls.
pub const SIOCADDRT: c_uint = 0x890B		/* add routing table entry	*/;
pub const SIOCDELRT: c_uint = 0x890C		/* delete routing table entry	*/;
pub const SIOCRTMSG: c_uint = 0x890D		/* unused			*/;
// Socket configuration controls.
pub const SIOCGIFNAME: c_uint = 0x8910		/* get iface name		*/;
pub const SIOCSIFLINK: c_uint = 0x8911		/* set iface channel		*/;
pub const SIOCGIFCONF: c_uint = 0x8912		/* get iface list		*/;
pub const SIOCGIFFLAGS: c_uint = 0x8913		/* get flags			*/;
pub const SIOCSIFFLAGS: c_uint = 0x8914		/* set flags			*/;
pub const SIOCGIFADDR: c_uint = 0x8915		/* get PA address		*/;
pub const SIOCSIFADDR: c_uint = 0x8916		/* set PA address		*/;
pub const SIOCGIFDSTADDR: c_uint = 0x8917		/* get remote PA address	*/;
pub const SIOCSIFDSTADDR: c_uint = 0x8918		/* set remote PA address	*/;
pub const SIOCGIFBRDADDR: c_uint = 0x8919		/* get broadcast PA address	*/;
pub const SIOCSIFBRDADDR: c_uint = 0x891a		/* set broadcast PA address	*/;
pub const SIOCGIFNETMASK: c_uint = 0x891b		/* get network PA mask		*/;
pub const SIOCSIFNETMASK: c_uint = 0x891c		/* set network PA mask		*/;
pub const SIOCGIFMETRIC: c_uint = 0x891d		/* get metric			*/;
pub const SIOCSIFMETRIC: c_uint = 0x891e		/* set metric			*/;
pub const SIOCGIFMEM: c_uint = 0x891f		/* get memory address (BSD)	*/;
pub const SIOCSIFMEM: c_uint = 0x8920		/* set memory address (BSD)	*/;
pub const SIOCGIFMTU: c_uint = 0x8921		/* get MTU size			*/;
pub const SIOCSIFMTU: c_uint = 0x8922		/* set MTU size			*/;
pub const SIOCSIFNAME: c_uint = 0x8923		/* set interface name */;
pub const SIOCSIFHWADDR: c_uint = 0x8924		/* set hardware address 	*/;
pub const SIOCGIFENCAP: c_uint = 0x8925		/* get/set encapsulations       */;
pub const SIOCSIFENCAP: c_uint = 0x8926;
pub const SIOCGIFHWADDR: c_uint = 0x8927		/* Get hardware address		*/;
pub const SIOCGIFSLAVE: c_uint = 0x8929		/* Driver slaving support	*/;
pub const SIOCSIFSLAVE: c_uint = 0x8930;
pub const SIOCADDMULTI: c_uint = 0x8931		/* Multicast address lists	*/;
pub const SIOCDELMULTI: c_uint = 0x8932;
pub const SIOCGIFINDEX: c_uint = 0x8933		/* name -> if_index mapping	*/;

pub const SIOCSIFPFLAGS: c_uint = 0x8934		/* set/get extended flags set	*/;
pub const SIOCGIFPFLAGS: c_uint = 0x8935;
pub const SIOCDIFADDR: c_uint = 0x8936		/* delete PA address		*/;
pub const SIOCSIFHWBROADCAST: c_uint = 0x8937	/* set hardware broadcast addr	*/;
pub const SIOCGIFCOUNT: c_uint = 0x8938		/* get number of devices */;
pub const SIOCGIFBR: c_uint = 0x8940		/* Bridging support		*/;
pub const SIOCSIFBR: c_uint = 0x8941		/* Set bridging options 	*/;
pub const SIOCGIFTXQLEN: c_uint = 0x8942		/* Get the tx queue length	*/;
pub const SIOCSIFTXQLEN: c_uint = 0x8943		/* Set the tx queue length 	*/;
// SIOCGIFDIVERT was:	0x8944		Frame diversion support
// SIOCSIFDIVERT was:	0x8945		Set frame diversion options
pub const SIOCETHTOOL: c_uint = 0x8946		/* Ethtool interface		*/;
pub const SIOCGMIIPHY: c_uint = 0x8947		/* Get address of MII PHY in use. */;
pub const SIOCGMIIREG: c_uint = 0x8948		/* Read MII PHY register.	*/;
pub const SIOCSMIIREG: c_uint = 0x8949		/* Write MII PHY register.	*/;
pub const SIOCWANDEV: c_uint = 0x894A		/* get/set netdev parameters	*/;
pub const SIOCOUTQNSD: c_uint = 0x894B		/* output queue size (not sent only) */;
pub const SIOCGSKNS: c_uint = 0x894C		/* get socket network namespace */;
// ARP cache control calls.
// 0x8950 - 0x8952  * obsolete calls, don't re-use
pub const SIOCDARP: c_uint = 0x8953		/* delete ARP table entry	*/;
pub const SIOCGARP: c_uint = 0x8954		/* get ARP table entry		*/;
pub const SIOCSARP: c_uint = 0x8955		/* set ARP table entry		*/;
// RARP cache control calls.
pub const SIOCDRARP: c_uint = 0x8960		/* delete RARP table entry	*/;
pub const SIOCGRARP: c_uint = 0x8961		/* get RARP table entry		*/;
pub const SIOCSRARP: c_uint = 0x8962		/* set RARP table entry		*/;
// Driver configuration calls
pub const SIOCGIFMAP: c_uint = 0x8970		/* Get device parameters	*/;
pub const SIOCSIFMAP: c_uint = 0x8971		/* Set device parameters	*/;
// DLCI configuration calls
pub const SIOCADDDLCI: c_uint = 0x8980		/* Create new DLCI device	*/;
pub const SIOCDELDLCI: c_uint = 0x8981		/* Delete DLCI device		*/;
pub const SIOCGIFVLAN: c_uint = 0x8982		/* 802.1Q VLAN support		*/;
pub const SIOCSIFVLAN: c_uint = 0x8983		/* Set 802.1Q VLAN options 	*/;
// bonding calls
pub const SIOCBONDENSLAVE: c_uint = 0x8990		/* enslave a device to the bond */;
pub const SIOCBONDRELEASE: c_uint = 0x8991		/* release a slave from the bond*/;
pub const SIOCBONDSETHWADDR: c_uint = 0x8992	/* set the hw addr of the bond  */;
pub const SIOCBONDSLAVEINFOQUERY: c_uint = 0x8993   /* rtn info about slave state   */;
pub const SIOCBONDINFOQUERY: c_uint = 0x8994	/* rtn info about bond state    */;
pub const SIOCBONDCHANGEACTIVE: c_uint = 0x8995   /* update to a new active slave */;
// bridge calls
pub const SIOCBRADDBR: c_uint = 0x89a0		/* create new bridge device     */;
pub const SIOCBRDELBR: c_uint = 0x89a1		/* remove bridge device         */;
pub const SIOCBRADDIF: c_uint = 0x89a2		/* add interface to bridge      */;
pub const SIOCBRDELIF: c_uint = 0x89a3		/* remove interface from bridge */;
// hardware time stamping: parameters in linux/net_tstamp.h
pub const SIOCSHWTSTAMP: c_uint = 0x89b0		/* set and get config		*/;
pub const SIOCGHWTSTAMP: c_uint = 0x89b1		/* get config			*/;
// Device private ioctl calls
//
// These 16 ioctls are available to devices via the do_ioctl() device
// vector. Each device should include this file and redefine these names
// as their own. Because these are device dependent it is a good idea
// _NOT_ to issue them to random objects and hope.
//
// THESE IOCTLS ARE _DEPRECATED_ AND WILL DISAPPEAR IN 2.5.X -DaveM
//
pub const SIOCDEVPRIVATE: c_uint = 0x89F0	/* to 89FF */;
//
// These 16 ioctl calls are protocol private
//
pub const SIOCPROTOPRIVATE: c_uint = 0x89E0 /* to 89EF */;
