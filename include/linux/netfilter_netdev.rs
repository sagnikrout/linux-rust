//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/netfilter_netdev.h
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

extern "C" {
    pub fn rcu_access_pointer(_arg: skb->dev->nf_hooks_ingress) -> return;
}
// caller must hold rcu_read_lock
// Must recheck the ingress hook head, in the event it became NULL
// after the check in nf_hook_ingress_active evaluated to true.
//

//
// nf_hook_egress - classify packets before transmission
// @skb: packet to be classified
// @rc: result code which shall be returned by __dev_queue_xmit() on failure
// @dev: netdev whose egress hooks shall be applied to @skb
//
// Caller must hold rcu_read_lock.
//
// On ingress, packets are classified first by tc, then by netfilter.
// On egress, the order is reversed for symmetry.  Conceptually, tc and
// netfilter can be thought of as layers, with netfilter layered above tc:
// When tc redirects a packet to another interface, netfilter is not applied
// because the packet is on the tc layer.
//
// The nf_skip_egress flag controls whether netfilter is applied on egress.
// It is updated by __netif_receive_skb_core() and __dev_queue_xmit() when the
// packet passes through tc and netfilter.  Because __dev_queue_xmit() may be
// called recursively by tunnel drivers such as vxlan, the flag is reverted to
// false after sch_handle_egress().  This ensures that netfilter is applied
// both on the overlay and underlying network.
//
// Returns: @skb on success or %NULL if the packet was consumed or filtered.
//

// nf assumes rcu_read_lock, not just read_lock_bh
// rc = NET_XMIT_DROP;
// rc = NET_XMIT_SUCCESS;

