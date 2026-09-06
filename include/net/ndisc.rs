//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/ndisc.h
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
//
// ICMP codes for neighbour discovery messages
//
pub const NDISC_ROUTER_SOLICITATION: c_int = 133;
pub const NDISC_ROUTER_ADVERTISEMENT: c_int = 134;
pub const NDISC_NEIGHBOUR_SOLICITATION: c_int = 135;
pub const NDISC_NEIGHBOUR_ADVERTISEMENT: c_int = 136;
pub const NDISC_REDIRECT: c_int = 137;
//
// Router type: cross-layer information from link-layer to
// IPv6 layer reported by certain link types (e.g., RFC4214).
//

//
// ndisc options
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_msg {
    pub icmph: icmp6hdr,
    pub target: in6_addr,
    pub opt: [__u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rs_msg {
    pub icmph: icmp6hdr,
    pub opt: [__u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ra_msg {
    pub icmph: icmp6hdr,
    pub reachable_time: __be32,
    pub retrans_timer: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rd_msg {
    pub icmph: icmp6hdr,
    pub target: in6_addr,
    pub dest: in6_addr,
    pub opt: [__u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_opt_hdr {
    pub nd_opt_type: __u8,
    pub nd_opt_len: __u8,
    pub __packed: },
// ND options
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ndisc_options {
    pub nd_opt_array: [*mut nd_opt_hdr; __ND_OPT_ARRAY_MAX],
    pub nd_opts_ri: *mut nd_opt_hdr,
    pub nd_opts_ri_end: *mut nd_opt_hdr,

    pub nd_useropts: *mut nd_opt_hdr,
    pub nd_useropts_end: *mut nd_opt_hdr,

    pub 1]: *mut *mut nd_opt_hdr nd_802154_opt_array[ND_OPT_TARGET_LL_ADDR +,

}

pub const NDISC_OPS_REDIRECT_DATA_SPACE: c_int = 2;
//
// This structure defines the hooks for IPv6 neighbour discovery.
// The following hooks can be defined; unless noted otherwise, they are
// optional and can be filled with a null pointer.
//
// int (*parse_options)(const struct net_device *dev,
// struct nd_opt_hdr *nd_opt,
// struct ndisc_options *ndopts):
// This function is called while parsing ndisc ops and put each position
// as pointer into ndopts. If this function return unequal 0, then this
// function took care about the ndisc option, if 0 then the IPv6 ndisc
// option parser will take care about that option.
//
// void (*update)(const struct net_device *dev, struct neighbour *n,
// u32 flags, u8 icmp6_type,
// const struct ndisc_options *ndopts):
// This function is called when IPv6 ndisc updates the neighbour cache
// entry. Additional options which can be updated may be previously
// parsed by parse_opts callback and accessible over ndopts parameter.
//
// int (*opt_addr_space)(const struct net_device *dev, u8 icmp6_type,
// struct neighbour *neigh, u8 *ha_buf,
// u8 **ha):
// This function is called when the necessary option space will be
// calculated before allocating a skb. The parameters neigh, ha_buf
// abd ha are available on NDISC_REDIRECT messages only.
//
// void (*fill_addr_option)(const struct net_device *dev,
// struct sk_buff *skb, u8 icmp6_type,
// const u8 *ha):
// This function is called when the skb will finally fill the option
// fields inside skb. NOTE: this callback should fill the option
// fields to the skb which are previously indicated by opt_space
// parameter. That means the decision to add such option should
// not lost between these two callbacks, e.g. protected by interface
// up state.
//
// void (*prefix_rcv_add_addr)(struct net *net, struct net_device *dev,
// const struct prefix_info *pinfo,
// struct inet6_dev *in6_dev,
// struct in6_addr *addr,
// int addr_type, u32 addr_flags,
// bool sllao, bool tokenized,
// __u32 valid_lft, u32 prefered_lft,
// bool dev_addr_generated):
// This function is called when a RA messages is received with valid
// PIO option fields and an IPv6 address will be added to the interface
// for autoconfiguration. The parameter dev_addr_generated reports about
// if the address was based on dev->dev_addr or not. This can be used
// to add a second address if link-layer operates with two link layer
// addresses. E.g. 802.15.4 6LoWPAN.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ndisc_ops {
    pub ndopts): *mut ndisc_options,
    pub ndopts): *const ndisc_options,
    pub ha): *mut u8,
    pub ha): *const u8,
    pub dev_addr_generated): bool,
}

//
// Return the padding between the option length and the start of the
// link addr.  Currently only IP-over-InfiniBand needs this, although
// if RFC 3831 IPv6-over-Fibre Channel is ever implemented it may
// also need a pad of 2.
//
extern "C" {
    pub fn NDISC_OPT_SPACE(pad: addr_len +) -> return;
}

extern "C" {
    pub fn ___neigh_lookup_noref(_arg: &nd_tbl, _arg: neigh_key_eq128, _arg: ndisc_hashfn, _arg: pkey, _arg: dev) -> return;
}

extern "C" {
    pub fn ERR_PTR(_arg: -EAFNOSUPPORT) -> return;
}

extern "C" {
    pub fn ndisc_init() -> c_int;
}
extern "C" {
    pub fn ndisc_late_init() -> c_int;
}
extern "C" {
    pub fn ndisc_late_cleanup();
}
extern "C" {
    pub fn ndisc_cleanup();
}
extern "C" {
    pub fn ndisc_rcv(skb: *mut sk_buff) -> skb_drop_reason;
}
extern "C" {
    pub fn ndisc_send_redirect(skb: *mut sk_buff, target: *const in6_addr);
}
extern "C" {
    pub fn ndisc_check_ns_na(skb: *mut sk_buff) -> c_int;
}
//
// IGMP
//
extern "C" {
    pub fn igmp6_init() -> c_int;
}
extern "C" {
    pub fn igmp6_late_init() -> c_int;
}
extern "C" {
    pub fn igmp6_cleanup();
}
extern "C" {
    pub fn igmp6_late_cleanup();
}
extern "C" {
    pub fn igmp6_event_query(skb: *mut sk_buff);
}
extern "C" {
    pub fn igmp6_event_report(skb: *mut sk_buff);
}

extern "C" {
    pub fn inet6_ifinfo_notify(event: c_int, idev: *mut inet6_dev);
}
