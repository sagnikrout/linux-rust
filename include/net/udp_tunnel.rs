//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/udp_tunnel.h
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


// SPDX-License-Identifier: GPL-2.0

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udp_port_cfg {
    pub family: u8,
// Used only for kernel-created sockets
    pub local_ip: in_addr,

    pub local_ip6: in6_addr,

}

extern "C" {
    pub fn udp_sock_create4(_arg: net, _arg: cfg, _arg: sockp) -> return;
}
extern "C" {
    pub fn udp_sock_create6(_arg: net, _arg: cfg, _arg: sockp) -> return;
}
extern "C" {
    pub fn int(sk: *mut *mut udp_tunnel_encap_rcv_t)(struct sock, skb: *mut sk_buff) -> typedef;
}
extern "C" {
    pub fn void(sk: *mut *mut udp_tunnel_encap_destroy_t)(struct sock) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct udp_tunnel_sock_cfg {
    pub /: *mut *mut *mut void sk_user_data; / user data used by encap_rcv call back,
// Used for setting up udp_sock fields, see udp.h for details
    pub encap_type: __u8,
    pub encap_rcv: udp_tunnel_encap_rcv_t,
    pub encap_err_lookup: udp_tunnel_encap_err_lookup_t,
    pub encap_err_rcv: udp_tunnel_encap_err_rcv_t,
    pub encap_destroy: udp_tunnel_encap_destroy_t,
    pub gro_receive: udp_tunnel_gro_receive_t,
    pub gro_complete: udp_tunnel_gro_complete_t,
}

// Setup the given (UDP) sock to receive UDP encapsulated packets
// -- List of parsable UDP tunnel types --
//
// Adding to this list will result in serious debate.  The main issue is
// that this list is essentially a list of workarounds for either poorly
// designed tunnels, or poorly designed device offloads.
//
// The parsing supported via these types should really be used for Rx
// traffic only as the network stack will have already inserted offsets for
// the location of the headers in the skb.  In addition any ports that are
// pushed should be kept within the namespace without leaking to other
// devices such as VFs or other ports on the same device.
//
// It is strongly encouraged to use CHECKSUM_COMPLETE for Rx to avoid the
// need to use this for Rx checksum offload.  It should not be necessary to
// call this function to perform Tx offloads on outgoing traffic.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum udp_parsable_tunnel_type {
    UDP_TUNNEL_TYPE_VXLAN	  = BIT(0), /* RFC 7348 */
    UDP_TUNNEL_TYPE_GENEVE	  = BIT(1), /* draft-ietf-nvo3-geneve */
    UDP_TUNNEL_TYPE_VXLAN_GPE = BIT(2), /* draft-ietf-nvo3-vxlan-gpe */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udp_tunnel_info {
    pub type: c_ushort,
    pub sa_family: sa_family_t,
    pub port: __be16,
    pub hw_priv: u8,
}

// Notify network devices of offloadable types
extern "C" {
    pub fn udp_tunnel_notify_add_rx_port(sk: *mut sock, type: c_ushort);
}
extern "C" {
    pub fn udp_tunnel_notify_del_rx_port(sk: *mut sock, type: c_ushort);
}
// Transmit the skb using UDP encapsulation.
//
// If the skb went through partial segmentation, lower devices
// will not need to offload the related features - except for
// UDP_TUNNEL, that will be re-added by the later
// udp_tunnel_handle_offloads().
//
// The inner protocol has been set by the nested tunnel, don't
// overraid it.
//
extern "C" {
    pub fn udp_tunnel_sock_release(sk: *mut sock);
}

extern "C" {
    pub fn iptunnel_handle_offloads(_arg: skb, _arg: type) -> return;
}

extern "C" {
    pub fn udp_tunnel_update_gro_lookup(net: *mut net, sk: *mut sock, add: bool);
}
extern "C" {
    pub fn udp_tunnel_update_gro_rcv(sk: *mut sock, add: bool);
}

pub const UDP_TUNNEL_NIC_MAX_TABLES: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum udp_tunnel_nic_info_flags {
// Device only supports offloads when it's open, all ports
// will be removed before close and re-added after open.
//
    UDP_TUNNEL_NIC_INFO_OPEN_ONLY	= BIT(0),
// Device supports only IPv4 tunnels
    UDP_TUNNEL_NIC_INFO_IPV4_ONLY	= BIT(1),
// Device has hard-coded the IANA VXLAN port (4789) as VXLAN.
// This port must not be counted towards n_entries of any table.
// Driver will not receive any callback associated with port 4789.
//
    UDP_TUNNEL_NIC_INFO_STATIC_IANA_VXLAN	= BIT(2),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udp_tunnel_nic_shared {
    pub udp_tunnel_nic_info: *mut udp_tunnel_nic,
    pub devices: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udp_tunnel_nic_shared_node {
    pub dev: *mut net_device,
    pub list: list_head,
}

//
// struct udp_tunnel_nic_info - driver UDP tunnel offload information
// @set_port:	callback for adding a new port
// @unset_port:	callback for removing a port
// @sync_table:	callback for syncing the entire port table at once
// @shared:	reference to device global state (optional)
// @flags:	device flags from enum udp_tunnel_nic_info_flags
// @tables:	UDP port tables this device has
// @tables.n_entries:		number of entries in this table
// @tables.tunnel_types:	types of tunnels this table accepts
//
// Drivers are expected to provide either @set_port and @unset_port callbacks
// or the @sync_table callback. Callbacks are invoked with rtnl lock held.
//
// Devices which (misguidedly) share the UDP tunnel port table across multiple
// netdevs should allocate an instance of struct udp_tunnel_nic_shared and
// point @shared at it.
// There must never be more than %UDP_TUNNEL_NIC_MAX_SHARING_DEVICES devices
// sharing a table.
//
// Known limitations:
// - UDP tunnel port notifications are fundamentally best-effort -
// it is likely the driver will both see skbs which use a UDP tunnel port,
// while not being a tunneled skb, and tunnel skbs from other ports -
// drivers should only use these ports for non-critical RX-side offloads,
// e.g. the checksum offload;
// - none of the devices care about the socket family at present, so we don't
// track it. Please extend this code if you care.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct udp_tunnel_nic_info {
// one-by-one
    pub ti): *mut udp_tunnel_info,
    pub ti): *mut udp_tunnel_info,
// all at once
    pub table): *mut *mut *mut int (sync_table)(struct net_device dev, unsigned int,
    pub shared: *mut udp_tunnel_nic_shared,
    pub flags: c_uint,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct udp_tunnel_nic_table_info {
    pub n_entries: c_uint,
    pub tunnel_types: c_uint,
    pub tables: [}; UDP_TUNNEL_NIC_MAX_TABLES],
}

// UDP tunnel module dependencies
//
// Tunnel drivers are expected to have a hard dependency on the udp_tunnel
// module. NIC drivers are not, they just attach their
// struct udp_tunnel_nic_info to the netdev and wait for callbacks to come.
// Loading a tunnel driver will cause the udp_tunnel module to be loaded
// and only then will all the required state structures be allocated.
// Since we want a weak dependency from the drivers and the core to udp_tunnel
// we call things through the following stubs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct udp_tunnel_nic_ops {
    pub ti): *mut unsigned int idx, struct udp_tunnel_info,
    pub priv): unsigned int idx, u8,
    pub ti): *mut *mut *mut void (add_port)(struct net_device dev, struct udp_tunnel_info,
    pub ti): *mut *mut *mut void (del_port)(struct net_device dev, struct udp_tunnel_info,
    pub dev): *mut *mut void (reset_ntf)(struct net_device,
    pub table): *mut *mut *mut size_t (dump_size)(struct net_device dev, unsigned int,
    pub skb): *mut sk_buff,
    pub dev): *mut *mut void (assert_locked)(struct net_device,
    pub dev): *mut *mut void (lock)(struct net_device,
    pub dev): *mut *mut void (unlock)(struct net_device,
}

// This helper is used from .sync_table, we indicate empty entries
// by zero'ed @ti. Drivers which need to know the details of a port
// when it gets deleted should use the .set_port / .unset_port
// callbacks.
// Zero out here, otherwise !CONFIG_INET causes uninitilized warnings.
//
// udp_tunnel_nic_reset_ntf() - device-originating reset notification
// @dev: network interface device structure
//
// Called by the driver to inform the core that the entire UDP tunnel port
// state has been lost, usually due to device reset. Core will assume device
// forgot all the ports and issue .set_port and .sync_table callbacks as
// necessary.
//
// This function must be called with rtnl lock held, and will issue all
// the callbacks before returning.
//
