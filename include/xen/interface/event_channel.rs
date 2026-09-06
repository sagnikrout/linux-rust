//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/interface/event_channel.h
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


// SPDX-License-Identifier: MIT
//
// event_channel.h
//
// Event channels between domains.
//
// Copyright (c) 2003-2004, K A Fraser.
//

pub type evtchn_port_t = u32;
//
// EVTCHNOP_alloc_unbound: Allocate a port in domain <dom> and mark as
// accepting interdomain bindings from domain <remote_dom>. A fresh port
// is allocated in <dom> and returned as <port>.
// NOTES:
// 1. If the caller is unprivileged then <dom> must be DOMID_SELF.
// 2. <rdom> may be DOMID_SELF, allowing loopback connections.
//
pub const EVTCHNOP_alloc_unbound: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evtchn_alloc_unbound {
// IN parameters
    pub remote_dom: domid_t dom,,
// OUT parameters
    pub port: evtchn_port_t,
}

//
// EVTCHNOP_bind_interdomain: Construct an interdomain event channel between
// the calling domain and <remote_dom>. <remote_dom,remote_port> must identify
// a port that is unbound and marked as accepting bindings from the calling
// domain. A fresh port is allocated in the calling domain and returned as
// <local_port>.
// NOTES:
// 2. <remote_dom> may be DOMID_SELF, allowing loopback connections.
//
pub const EVTCHNOP_bind_interdomain: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evtchn_bind_interdomain {
// IN parameters.
    pub remote_dom: domid_t,
    pub remote_port: evtchn_port_t,
// OUT parameters.
    pub local_port: evtchn_port_t,
}

//
// EVTCHNOP_bind_virq: Bind a local event channel to VIRQ <irq> on specified
// vcpu.
// NOTES:
// 1. A virtual IRQ may be bound to at most one event channel per vcpu.
// 2. The allocated event channel is bound to the specified vcpu. The binding
// may not be changed.
//
pub const EVTCHNOP_bind_virq: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evtchn_bind_virq {
// IN parameters.
    pub virq: u32,
    pub vcpu: u32,
// OUT parameters.
    pub port: evtchn_port_t,
}

//
// EVTCHNOP_bind_pirq: Bind a local event channel to PIRQ <irq>.
// NOTES:
// 1. A physical IRQ may be bound to at most one event channel per domain.
// 2. Only a sufficiently-privileged domain may bind to a physical IRQ.
//
pub const EVTCHNOP_bind_pirq: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evtchn_bind_pirq {
// IN parameters.
    pub pirq: u32,
pub const BIND_PIRQ__WILL_SHARE: c_int = 1;
    pub /: *mut *mut *mut uint32_t flags; / BIND_PIRQ__,
// OUT parameters.
    pub port: evtchn_port_t,
}

//
// EVTCHNOP_bind_ipi: Bind a local event channel to receive events.
// NOTES:
// 1. The allocated event channel is bound to the specified vcpu. The binding
// may not be changed.
//
pub const EVTCHNOP_bind_ipi: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evtchn_bind_ipi {
    pub vcpu: u32,
// OUT parameters.
    pub port: evtchn_port_t,
}

//
// EVTCHNOP_close: Close a local event channel <port>. If the channel is
// interdomain then the remote end is placed in the unbound state
// (EVTCHNSTAT_unbound), awaiting a new connection.
//
pub const EVTCHNOP_close: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evtchn_close {
// IN parameters.
    pub port: evtchn_port_t,
}

//
// EVTCHNOP_send: Send an event to the remote end of the channel whose local
// endpoint is <port>.
//
pub const EVTCHNOP_send: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evtchn_send {
// IN parameters.
    pub port: evtchn_port_t,
}

//
// EVTCHNOP_status: Get the current status of the communication channel which
// has an endpoint at <dom, port>.
// NOTES:
// 1. <dom> may be specified as DOMID_SELF.
// 2. Only a sufficiently-privileged domain may obtain the status of an event
// channel for which <dom> is not DOMID_SELF.
//
pub const EVTCHNOP_status: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evtchn_status {
// IN parameters
    pub dom: domid_t,
    pub port: evtchn_port_t,
// OUT parameters

    pub status: u32,
    pub /: *mut *mut uint32_t vcpu; / VCPU to which this channel is bound.,
    pub dom: domid_t,
    pub /: *mut *mut } unbound; / EVTCHNSTAT_unbound,
    pub dom: domid_t,
    pub port: evtchn_port_t,
    pub /: *mut *mut } interdomain; / EVTCHNSTAT_interdomain,
    pub /: *mut *mut uint32_t pirq; / EVTCHNSTAT_pirq,
    pub /: *mut *mut uint32_t virq; / EVTCHNSTAT_virq,
    pub u: },
}

//
// EVTCHNOP_bind_vcpu: Specify which vcpu a channel should notify when an
// event is pending.
// NOTES:
// 1. IPI- and VIRQ-bound channels always notify the vcpu that initialised
// the binding. This binding cannot be changed.
// 2. All other channels notify vcpu0 by default. This default is set when
// the channel is allocated (a port that is freed and subsequently reused
// has its binding reset to vcpu0).
//
pub const EVTCHNOP_bind_vcpu: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evtchn_bind_vcpu {
// IN parameters.
    pub port: evtchn_port_t,
    pub vcpu: u32,
}

//
// EVTCHNOP_unmask: Unmask the specified local event-channel port and deliver
// a notification to the appropriate VCPU if an event is pending.
//
pub const EVTCHNOP_unmask: c_int = 9;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evtchn_unmask {
// IN parameters.
    pub port: evtchn_port_t,
}

//
// EVTCHNOP_reset: Close all event channels associated with specified domain.
// NOTES:
// 1. <dom> may be specified as DOMID_SELF.
// 2. Only a sufficiently-privileged domain may specify other than DOMID_SELF.
//
pub const EVTCHNOP_reset: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evtchn_reset {
// IN parameters.
    pub dom: domid_t,
}

pub type evtchn_reset_t = evtchn_reset;
//
// EVTCHNOP_init_control: initialize the control block for the FIFO ABI.
//
pub const EVTCHNOP_init_control: c_int = 11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evtchn_init_control {
// IN parameters.
    pub control_gfn: u64,
    pub offset: u32,
    pub vcpu: u32,
// OUT parameters.
    pub link_bits: u8,
    pub _pad: [u8; 7],
}

//
// EVTCHNOP_expand_array: add an additional page to the event array.
//
pub const EVTCHNOP_expand_array: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evtchn_expand_array {
// IN parameters.
    pub array_gfn: u64,
}

//
// EVTCHNOP_set_priority: set the priority for an event channel.
//
pub const EVTCHNOP_set_priority: c_int = 13;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evtchn_set_priority {
// IN parameters.
    pub port: evtchn_port_t,
    pub priority: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct evtchn_op {
    pub /: *mut *mut *mut uint32_t cmd; / EVTCHNOP_,
    pub alloc_unbound: evtchn_alloc_unbound,
    pub bind_interdomain: evtchn_bind_interdomain,
    pub bind_virq: evtchn_bind_virq,
    pub bind_pirq: evtchn_bind_pirq,
    pub bind_ipi: evtchn_bind_ipi,
    pub close: evtchn_close,
    pub send: evtchn_send,
    pub status: evtchn_status,
    pub bind_vcpu: evtchn_bind_vcpu,
    pub unmask: evtchn_unmask,
    pub u: },
}

//
// 2-level ABI
//

//
// FIFO ABI
//
// Events may have priorities from 0 (highest) to 15 (lowest).
pub const EVTCHN_FIFO_PRIORITY_MAX: c_int = 0;
pub const EVTCHN_FIFO_PRIORITY_DEFAULT: c_int = 7;
pub const EVTCHN_FIFO_PRIORITY_MIN: c_int = 15;

pub type event_word_t = u32;
pub const EVTCHN_FIFO_PENDING: c_int = 31;
pub const EVTCHN_FIFO_MASKED: c_int = 30;
pub const EVTCHN_FIFO_LINKED: c_int = 29;
pub const EVTCHN_FIFO_BUSY: c_int = 28;
pub const EVTCHN_FIFO_LINK_BITS: c_int = 17;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct evtchn_fifo_control_block {
    pub ready: u32,
    pub _rsvd: u32,
    pub head: [event_word_t; EVTCHN_FIFO_MAX_QUEUES],
}
