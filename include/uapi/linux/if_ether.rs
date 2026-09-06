//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/if_ether.h
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
// Global definitions for the Ethernet IEEE 802.3 interface.
//
// Version:	@(#)if_ether.h	1.0.1a	02/08/94
//
// Author:	Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
// Donald Becker, <becker@super.org>
// Alan Cox, <alan@lxorguk.ukuu.org.uk>
// Steve Whitehouse, <gw7rrm@eeshack3.swan.ac.uk>
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//

//
// IEEE 802.3 Ethernet magic constants.  The frame sizes omit the preamble
// and FCS/CRC (frame check sequence).
//

pub const ETH_MAX_MTU: c_uint = 0xFFFFU		/* 65535, same as IP_MAX_MTU	*/;
//
// These are the defined Ethernet Protocol ID's.
//
pub const ETH_P_LOOP: c_uint = 0x0060		/* Ethernet Loopback packet	*/;
pub const ETH_P_PUP: c_uint = 0x0200		/* Xerox PUP packet		*/;
pub const ETH_P_PUPAT: c_uint = 0x0201		/* Xerox PUP Addr Trans packet	*/;
pub const ETH_P_TSN: c_uint = 0x22F0		/* TSN (IEEE 1722) packet	*/;
pub const ETH_P_ERSPAN2: c_uint = 0x22EB		/* ERSPAN version 2 (type III)	*/;
pub const ETH_P_IP: c_uint = 0x0800		/* Internet Protocol packet	*/;
pub const ETH_P_X25: c_uint = 0x0805		/* CCITT X.25			*/;
pub const ETH_P_ARP: c_uint = 0x0806		/* Address Resolution packet	*/;
pub const ETH_P_BPQ: c_uint = 0x08FF		/* G8BPQ AX.25 Ethernet Packet	[ NOT AN OFFICIALLY REGISTERED ID ] */;
pub const ETH_P_IEEEPUP: c_uint = 0x0a00		/* Xerox IEEE802.3 PUP packet */;
pub const ETH_P_IEEEPUPAT: c_uint = 0x0a01		/* Xerox IEEE802.3 PUP Addr Trans packet */;
pub const ETH_P_BATMAN: c_uint = 0x4305		/* B.A.T.M.A.N.-Advanced packet [ NOT AN OFFICIALLY REGISTERED ID ] */;
pub const ETH_P_DEC: c_uint = 0x6000          /* DEC Assigned proto           */;
pub const ETH_P_DNA_DL: c_uint = 0x6001          /* DEC DNA Dump/Load            */;
pub const ETH_P_DNA_RC: c_uint = 0x6002          /* DEC DNA Remote Console       */;
pub const ETH_P_DNA_RT: c_uint = 0x6003          /* DEC DNA Routing              */;
pub const ETH_P_LAT: c_uint = 0x6004          /* DEC LAT                      */;
pub const ETH_P_DIAG: c_uint = 0x6005          /* DEC Diagnostics              */;
pub const ETH_P_CUST: c_uint = 0x6006          /* DEC Customer use             */;
pub const ETH_P_SCA: c_uint = 0x6007          /* DEC Systems Comms Arch       */;
pub const ETH_P_TEB: c_uint = 0x6558		/* Trans Ether Bridging		*/;
pub const ETH_P_RARP: c_uint = 0x8035		/* Reverse Addr Res packet	*/;
pub const ETH_P_ATALK: c_uint = 0x809B		/* Appletalk DDP		*/;
pub const ETH_P_AARP: c_uint = 0x80F3		/* Appletalk AARP		*/;
pub const ETH_P_8021Q: c_uint = 0x8100          /* 802.1Q VLAN Extended Header  */;
pub const ETH_P_ERSPAN: c_uint = 0x88BE		/* ERSPAN type II		*/;
pub const ETH_P_IPX: c_uint = 0x8137		/* IPX over DIX			*/;
pub const ETH_P_IPV6: c_uint = 0x86DD		/* IPv6 over bluebook		*/;
pub const ETH_P_PAUSE: c_uint = 0x8808		/* IEEE Pause frames. See 802.3 31B */;
pub const ETH_P_SLOW: c_uint = 0x8809		/* Slow Protocol. See 802.3ad 43B */;
pub const ETH_P_WCCP: c_uint = 0x883E		/* Web-cache coordination protocol;
// defined in draft-wilson-wrec-wccp-v2-00.txt
pub const ETH_P_MPLS_UC: c_uint = 0x8847		/* MPLS Unicast traffic		*/;
pub const ETH_P_MPLS_MC: c_uint = 0x8848		/* MPLS Multicast traffic	*/;
pub const ETH_P_ATMMPOA: c_uint = 0x884c		/* MultiProtocol Over ATM	*/;
pub const ETH_P_PPP_DISC: c_uint = 0x8863		/* PPPoE discovery messages     */;
pub const ETH_P_PPP_SES: c_uint = 0x8864		/* PPPoE session messages	*/;
pub const ETH_P_LINK_CTL: c_uint = 0x886c		/* HPNA, wlan link local tunnel */;
pub const ETH_P_8021AC: c_uint = 0x8870		/* 802.1AC LLC > 1500 bytes     */;
pub const ETH_P_ATMFATE: c_uint = 0x8884		/* Frame-based ATM Transport;
// over Ethernet
//
pub const ETH_P_PAE: c_uint = 0x888E		/* Port Access Entity (IEEE 802.1X) */;
pub const ETH_P_PROFINET: c_uint = 0x8892		/* PROFINET			*/;
pub const ETH_P_REALTEK: c_uint = 0x8899          /* Multiple proprietary protocols */;
pub const ETH_P_AOE: c_uint = 0x88A2		/* ATA over Ethernet		*/;
pub const ETH_P_ETHERCAT: c_uint = 0x88A4		/* EtherCAT			*/;
pub const ETH_P_8021AD: c_uint = 0x88A8          /* 802.1ad Service VLAN		*/;
pub const ETH_P_802_EX1: c_uint = 0x88B5		/* 802.1 Local Experimental 1.  */;
pub const ETH_P_MXLGSW: c_uint = 0x88C3		/* Infineon Technologies Corporate Research ST;
// Used by MaxLinear GSW DSA
//
pub const ETH_P_PREAUTH: c_uint = 0x88C7		/* 802.11 Preauthentication */;
pub const ETH_P_TIPC: c_uint = 0x88CA		/* TIPC 			*/;
pub const ETH_P_LLDP: c_uint = 0x88CC		/* Link Layer Discovery Protocol */;
pub const ETH_P_MRP: c_uint = 0x88E3		/* Media Redundancy Protocol	*/;
pub const ETH_P_MACSEC: c_uint = 0x88E5		/* 802.1ae MACsec */;
pub const ETH_P_8021AH: c_uint = 0x88E7          /* 802.1ah Backbone Service Tag */;
pub const ETH_P_MVRP: c_uint = 0x88F5          /* 802.1Q MVRP                  */;
pub const ETH_P_1588: c_uint = 0x88F7		/* IEEE 1588 Timesync */;
pub const ETH_P_NCSI: c_uint = 0x88F8		/* NCSI protocol		*/;
pub const ETH_P_PRP: c_uint = 0x88FB		/* IEC 62439-3 PRP/HSRv0	*/;
pub const ETH_P_CFM: c_uint = 0x8902		/* Connectivity Fault Management */;
pub const ETH_P_FCOE: c_uint = 0x8906		/* Fibre Channel over Ethernet  */;
pub const ETH_P_IBOE: c_uint = 0x8915		/* Infiniband over Ethernet	*/;
pub const ETH_P_TDLS: c_uint = 0x890D          /* TDLS */;
pub const ETH_P_FIP: c_uint = 0x8914		/* FCoE Initialization Protocol */;
pub const ETH_P_80221: c_uint = 0x8917		/* IEEE 802.21 Media Independent Handover Protocol */;
pub const ETH_P_HSR: c_uint = 0x892F		/* IEC 62439-3 HSRv1	*/;
pub const ETH_P_NSH: c_uint = 0x894F		/* Network Service Header */;
pub const ETH_P_LOOPBACK: c_uint = 0x9000		/* Ethernet loopback packet, per IEEE 802.3 */;
pub const ETH_P_QINQ1: c_uint = 0x9100		/* deprecated QinQ VLAN [ NOT AN OFFICIALLY REGISTERED ID ] */;
pub const ETH_P_QINQ2: c_uint = 0x9200		/* deprecated QinQ VLAN [ NOT AN OFFICIALLY REGISTERED ID ] */;
pub const ETH_P_QINQ3: c_uint = 0x9300		/* deprecated QinQ VLAN [ NOT AN OFFICIALLY REGISTERED ID ] */;
pub const ETH_P_YT921X: c_uint = 0x9988		/* Motorcomm YT921x DSA [ NOT AN OFFICIALLY REGISTERED ID ] */;
pub const ETH_P_EDSA: c_uint = 0xDADA		/* Ethertype DSA [ NOT AN OFFICIALLY REGISTERED ID ] */;
pub const ETH_P_DSA_8021Q: c_uint = 0xDADB		/* Fake VLAN Header for DSA [ NOT AN OFFICIALLY REGISTERED ID ] */;
pub const ETH_P_DSA_A5PSW: c_uint = 0xE001		/* A5PSW Tag Value [ NOT AN OFFICIALLY REGISTERED ID ] */;
pub const ETH_P_IFE: c_uint = 0xED3E		/* ForCES inter-FE LFB type */;
pub const ETH_P_AF_IUCV: c_uint = 0xFBFB		/* IBM af_iucv [ NOT AN OFFICIALLY REGISTERED ID ] */;
pub const ETH_P_NXP_NETC: c_uint = 0xFD3A		/* NXP NETC DSA [ NOT AN OFFICIALLY REGISTERED ID ] */;
pub const ETH_P_802_3_MIN: c_uint = 0x0600		/* If the value in the ethernet type is more than this value;
// then the frame is Ethernet II. Else it is 802.3
//
// Non DIX types. Won't clash for 1500 types.
//
pub const ETH_P_802_3: c_uint = 0x0001		/* Dummy type for 802.3 frames  */;
pub const ETH_P_AX25: c_uint = 0x0002		/* Dummy protocol id for AX.25  */;
pub const ETH_P_ALL: c_uint = 0x0003		/* Every packet (be careful!!!) */;
pub const ETH_P_802_2: c_uint = 0x0004		/* 802.2 frames 		*/;
pub const ETH_P_SNAP: c_uint = 0x0005		/* Internal only		*/;
pub const ETH_P_DDCMP: c_uint = 0x0006          /* DEC DDCMP: Internal only     */;
pub const ETH_P_WAN_PPP: c_uint = 0x0007          /* Dummy type for WAN PPP frames*/;
pub const ETH_P_PPP_MP: c_uint = 0x0008          /* Dummy type for PPP MP frames */;
pub const ETH_P_LOCALTALK: c_uint = 0x0009		/* Localtalk pseudo type 	*/;
pub const ETH_P_CAN: c_uint = 0x000C		/* CAN: Controller Area Network */;
pub const ETH_P_CANFD: c_uint = 0x000D		/* CANFD: CAN flexible data rate*/;
pub const ETH_P_CANXL: c_uint = 0x000E		/* CANXL: eXtended frame Length */;
pub const ETH_P_PPPTALK: c_uint = 0x0010		/* Dummy type for Atalk over PPP*/;
pub const ETH_P_TR_802_2: c_uint = 0x0011		/* 802.2 frames 		*/;
pub const ETH_P_MOBITEX: c_uint = 0x0015		/* Mobitex (kaz@cafe.net)	*/;
pub const ETH_P_CONTROL: c_uint = 0x0016		/* Card specific control frames */;
pub const ETH_P_IRDA: c_uint = 0x0017		/* Linux-IrDA			*/;
pub const ETH_P_ECONET: c_uint = 0x0018		/* Acorn Econet			*/;
pub const ETH_P_HDLC: c_uint = 0x0019		/* HDLC frames			*/;
pub const ETH_P_ARCNET: c_uint = 0x001A		/* 1A for ArcNet :-)            */;
pub const ETH_P_DSA: c_uint = 0x001B		/* Distributed Switch Arch.	*/;
pub const ETH_P_TRAILER: c_uint = 0x001C		/* Trailer switch tagging	*/;
pub const ETH_P_PHONET: c_uint = 0x00F5		/* Nokia Phonet frames          */;
pub const ETH_P_IEEE802154: c_uint = 0x00F6		/* IEEE802.15.4 frame		*/;
pub const ETH_P_CAIF: c_uint = 0x00F7		/* ST-Ericsson CAIF protocol	*/;
pub const ETH_P_XDSA: c_uint = 0x00F8		/* Multiplexed DSA protocol	*/;
pub const ETH_P_MAP: c_uint = 0x00F9		/* Qualcomm multiplexing and;
// aggregation protocol
//
pub const ETH_P_MCTP: c_uint = 0x00FA		/* Management component transport;
// protocol packets
//
pub const ETH_P_GRE_OSI: c_uint = 0x00FE		/* GRE tunnels: LLC "fe fe 03" analog,;
// used primarily for IS-IS over GRE
// WARNING: not internal, used on wire!
//
// This is an Ethernet frame header.
//
// allow libcs like musl to deactivate this, glibc does not implement this.
pub const __UAPI_DEF_ETHHDR: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethhdr {
    pub /: *mut *mut unsigned char h_dest[ETH_ALEN]; / destination eth addr,
    pub /: *mut *mut unsigned char h_source[ETH_ALEN]; / source ether addr,
    pub /: *mut *mut __be16 h_proto; / packet type ID field,
    pub __attribute__((packed)): },

