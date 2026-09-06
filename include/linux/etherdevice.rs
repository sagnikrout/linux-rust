//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/etherdevice.h
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
// INET		An implementation of the TCP/IP protocol suite for the LINUX
// operating system.  NET  is implemented using the  BSD Socket
// interface as the means of communication with the user level.
//
// Definitions for the Ethernet handlers.
//
// Version:	@(#)eth.h	1.0.4	05/13/93
//
// Authors:	Ross Biro
// Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
//
// Relocated to include/linux where it belongs by Alan Cox
// <gw4pts@gw4pts.ampr.org>
//

extern "C" {
    pub fn eth_platform_get_mac_address(dev: *mut device, mac_addr: *mut u8) -> c_int;
}
extern "C" {
    pub fn platform_get_ethdev_address(dev: *mut device, netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn nvmem_get_mac_address(dev: *mut device, addrbuf: *mut c_void) -> c_int;
}
extern "C" {
    pub fn device_get_mac_address(dev: *mut device, addr: *mut c_char) -> c_int;
}
extern "C" {
    pub fn device_get_ethdev_address(dev: *mut device, netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn fwnode_get_mac_address(fwnode: *mut fwnode_handle, addr: *mut c_char) -> c_int;
}
extern "C" {
    pub fn eth_get_headlen(dev: *const net_device, data: *const c_void, len: u32) -> u32;
}
extern "C" {
    pub fn eth_type_trans(skb: *mut sk_buff, dev: *mut net_device) -> __be16;
}
extern "C" {
    pub fn eth_header_parse_protocol(skb: *const sk_buff) -> __be16;
}
extern "C" {
    pub fn eth_prepare_mac_addr_change(dev: *mut net_device, p: *mut c_void) -> c_int;
}
extern "C" {
    pub fn eth_commit_mac_addr_change(dev: *mut net_device, p: *mut c_void);
}
extern "C" {
    pub fn eth_mac_addr(dev: *mut net_device, p: *mut c_void) -> c_int;
}
extern "C" {
    pub fn eth_validate_addr(dev: *mut net_device) -> c_int;
}

extern "C" {
    pub fn eth_gro_complete(skb: *mut sk_buff, nhoff: c_int) -> c_int;
}
// Reserved Ethernet Addresses per IEEE 802.1Q

//
// is_link_local_ether_addr - Determine if given Ethernet address is link-local
// @addr: Pointer to a six-byte array containing the Ethernet address
//
// Return: true if address is link local reserved addr (01:80:c2:00:00:0X) per
// IEEE 802.1Q 8.6.3 Frame filtering.
//
// Please note: addr must be aligned to u16.
//

//
// is_zero_ether_addr - Determine if give Ethernet address is all zeros.
// @addr: Pointer to a six-byte array containing the Ethernet address
//
// Return: true if the address is all zeroes.
//
// Please note: addr must be aligned to u16.
//

// (const u16 *)(addr + 2) |
// (const u16 *)(addr + 4)) == 0;

//
// is_multicast_ether_addr - Determine if the Ethernet address is a multicast.
// @addr: Pointer to a six-byte array containing the Ethernet address
//
// Return: true if the address is a multicast address.
// By definition the broadcast address is also a multicast address.
//

extern "C" {
    pub fn is_multicast_ether_addr(_arg: addr) -> return;
}

//
// is_local_ether_addr - Determine if the Ethernet address is locally-assigned one (IEEE 802).
// @addr: Pointer to a six-byte array containing the Ethernet address
//
// Return: true if the address is a local address.
//
// is_broadcast_ether_addr - Determine if the Ethernet address is broadcast
// @addr: Pointer to a six-byte array containing the Ethernet address
//
// Return: true if the address is the broadcast address.
//
// Please note: addr must be aligned to u16.
//
// (const u16 *)(addr + 2) &
// (const u16 *)(addr + 4)) == 0xffff;
//
// is_unicast_ether_addr - Determine if the Ethernet address is unicast
// @addr: Pointer to a six-byte array containing the Ethernet address
//
// Return: true if the address is a unicast address.
//
// is_valid_ether_addr - Determine if the given Ethernet address is valid
// @addr: Pointer to a six-byte array containing the Ethernet address
//
// Check that the Ethernet address (MAC) is not 00:00:00:00:00:00, is not
// a multicast address, and is not FF:FF:FF:FF:FF:FF.
//
// Return: true if the address is valid.
//
// Please note: addr must be aligned to u16.
//
// FF:FF:FF:FF:FF:FF is a multicast address so we don't need to
// explicitly check for it here.
//
// eth_proto_is_802_3 - Determine if a given Ethertype/length is a protocol
// @proto: Ethertype/length value to be tested
//
// Check that the value from the Ethertype/length field is a valid Ethertype.
//
// Return: true if the valid is an 802.3 supported Ethertype.
//
// if CPU is little endian mask off bits representing LSB

// cast both to u16 and compare since LSB can be ignored
//
// eth_random_addr - Generate software assigned random Ethernet address
// @addr: Pointer to a six-byte array containing the Ethernet address
//
// Generate a random Ethernet address (MAC) that is not multicast
// and has the local assigned bit set.
//
// eth_broadcast_addr - Assign broadcast address
// @addr: Pointer to a six-byte array containing the Ethernet address
//
// Assign the broadcast address to the given address array.
//
// eth_zero_addr - Assign zero address
// @addr: Pointer to a six-byte array containing the Ethernet address
//
// Assign the zero address to the given address array.
//
// eth_hw_addr_random - Generate software assigned random Ethernet and
// set device flag
// @dev: pointer to net_device structure
//
// Generate a random Ethernet address (MAC) to be used by a net device
// and set addr_assign_type so the state can be read by sysfs and be
// used by userspace.
//
// eth_hw_addr_crc - Calculate CRC from netdev_hw_addr
// @ha: pointer to hardware address
//
// Calculate CRC from a hardware address as basis for filter hashes.
//
extern "C" {
    pub fn ether_crc(_arg: ETH_ALEN, _arg: ha->addr) -> return;
}
//
// ether_addr_copy - Copy an Ethernet address
// @dst: Pointer to a six-byte array Ethernet address destination
// @src: Pointer to a six-byte array Ethernet address source
//
// Please note: dst & src must both be aligned to u16.
//

// (u32 *)dst = *(const u32 *)src;
// (u16 *)(dst + 4) = *(const u16 *)(src + 4);

//
// eth_hw_addr_set - Assign Ethernet address to a net_device
// @dev: pointer to net_device structure
// @addr: address to assign
//
// Assign given address to the net_device, addr_assign_type is not changed.
//
// eth_hw_addr_inherit - Copy dev_addr from another net_device
// @dst: pointer to net_device to copy dev_addr to
// @src: pointer to net_device to copy dev_addr from
//
// Copy the Ethernet address from one net_device to another along with
// the address attributes (addr_assign_type).
//
// ether_addr_equal - Compare two Ethernet addresses
// @addr1: Pointer to a six-byte array containing the Ethernet address
// @addr2: Pointer other six-byte array containing the Ethernet address
//
// Compare two Ethernet addresses, returns true if equal
//
// Please note: addr1 & addr2 must both be aligned to u16.
//

//
// ether_addr_equal_64bits - Compare two Ethernet addresses
// @addr1: Pointer to an array of 8 bytes
// @addr2: Pointer to an other array of 8 bytes
//
// Compare two Ethernet addresses, returns true if equal, false otherwise.
//
// The function doesn't need any conditional branches and possibly uses
// word memory accesses on CPU allowing cheap unaligned memory reads.
// arrays = { byte1, byte2, byte3, byte4, byte5, byte6, pad1, pad2 }
//
// Please note that alignment of addr1 & addr2 are only guaranteed to be 16 bits.
//

extern "C" {
    pub fn ether_addr_equal(_arg: addr1, _arg: addr2) -> return;
}

//
// ether_addr_equal_unaligned - Compare two not u16 aligned Ethernet addresses
// @addr1: Pointer to a six-byte array containing the Ethernet address
// @addr2: Pointer other six-byte array containing the Ethernet address
//
// Compare two Ethernet addresses, returns true if equal
//
// Please note: Use only when any Ethernet address may not be u16 aligned.
//

extern "C" {
    pub fn ether_addr_equal(_arg: addr1, _arg: addr2) -> return;
}

//
// ether_addr_equal_masked - Compare two Ethernet addresses with a mask
// @addr1: Pointer to a six-byte array containing the 1st Ethernet address
// @addr2: Pointer to a six-byte array containing the 2nd Ethernet address
// @mask: Pointer to a six-byte array containing the Ethernet address bitmask
//
// Compare two Ethernet addresses with a mask, returns true if for every bit
// set in the bitmask the equivalent bits in the ethernet addresses are equal.
// Using a mask with all bits set is a slower ether_addr_equal.
//
extern "C" {
    pub fn ether_addr_equal_masked(_arg: addr, _arg: eth_ipv4_mcast_addr_base, _arg: mask) -> return;
}
extern "C" {
    pub fn ether_addr_equal_masked(_arg: addr, _arg: eth_ipv6_mcast_addr_base, _arg: mask) -> return;
}
//
// ether_addr_to_u64 - Convert an Ethernet address into a u64 value.
// @addr: Pointer to a six-byte array containing the Ethernet address
//
// Return: a u64 value of the address
//
// u64_to_ether_addr - Convert a u64 to an Ethernet address.
// @u: u64 to convert to an Ethernet MAC address
// @addr: Pointer to a six-byte array to contain the Ethernet address
//
// eth_addr_dec - Decrement the given MAC address
//
// @addr: Pointer to a six-byte array containing Ethernet address to decrement
//
// eth_addr_inc() - Increment the given MAC address.
// @addr: Pointer to a six-byte array containing Ethernet address to increment.
//
// eth_addr_add() - Add (or subtract) an offset to/from the given MAC address.
//
// @offset: Offset to add.
// @addr: Pointer to a six-byte array containing Ethernet address to increment.
//
// is_etherdev_addr - Tell if given Ethernet address belongs to the device.
// @dev: Pointer to a device structure
// @addr: Pointer to a six-byte array containing the Ethernet address
//
// Compare passed address with all addresses of the device. Return true if the
// address if one of the device addresses.
//
// Note that this function calls ether_addr_equal_64bits() so take care of
// the right padding.
//

//
// compare_ether_header - Compare two Ethernet headers
// @a: Pointer to Ethernet header
// @b: Pointer to Ethernet header
//
// Compare two Ethernet headers, returns 0 if equal.
// This assumes that the network header (i.e., IP header) is 4-byte
// aligned OR the platform can handle unaligned access.  This is the
// case for all packets coming into netif_receive_skb or similar
// entry points.
//

//
// We want to compare 14 bytes:
// [a0 ... a13] ^ [b0 ... b13]
// Use two long XOR, ORed together, with an overlap of two bytes.
// [a0  a1  a2  a3  a4  a5  a6  a7 ] ^ [b0  b1  b2  b3  b4  b5  b6  b7 ] |
// [a6  a7  a8  a9  a10 a11 a12 a13] ^ [b6  b7  b8  b9  b10 b11 b12 b13]
// This means the [a6 a7] ^ [b6 b7] part is done two times.
//

//
// eth_hw_addr_gen - Generate and assign Ethernet address to a port
// @dev: pointer to port's net_device structure
// @base_addr: base Ethernet address
// @id: offset to add to the base address
//
// Generate a MAC address using a base address and an offset and assign it
// to a net_device. Commonly used by switch drivers which need to compute
// addresses for all their ports. addr_assign_type is not changed.
//
// eth_skb_pkt_type - Assign packet type if destination address does not match
// @skb: Assigned a packet type if address does not match @dev address
// @dev: Network device used to compare packet address against
//
// If the destination MAC address of the packet does not match the network
// device address, assign an appropriate packet type.
//
// eth_skb_pad - Pad buffer to minimum number of octets for Ethernet frame
// @skb: Buffer to pad
//
// An Ethernet frame should have a minimum size of 60 bytes.  This function
// takes short frames and pads them with zeros up to the 60 byte limit.
//
extern "C" {
    pub fn skb_put_padto(_arg: skb, _arg: ETH_ZLEN) -> return;
}
