//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/thunderbolt/tunnel.h
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
// Thunderbolt driver - Tunneling support
//
// Copyright (c) 2014 Andreas Noever <andreas.noever@gmail.com>
// Copyright (C) 2019, Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tb_tunnel_type {
    TB_TUNNEL_PCI,
    TB_TUNNEL_DP,
    TB_TUNNEL_DMA,
    TB_TUNNEL_USB3,
}

//
// enum tb_tunnel_state - State of a tunnel
// @TB_TUNNEL_INACTIVE: tb_tunnel_activate() is not called for the tunnel
// @TB_TUNNEL_ACTIVATING: tb_tunnel_activate() returned successfully for the tunnel
// @TB_TUNNEL_ACTIVE: The tunnel is fully active
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tb_tunnel_state {
    TB_TUNNEL_INACTIVE,
    TB_TUNNEL_ACTIVATING,
    TB_TUNNEL_ACTIVE,
}

//
// struct tb_tunnel - Tunnel between two ports
// @kref: Reference count
// @tb: Pointer to the domain
// @src_port: Source port of the tunnel
// @dst_port: Destination port of the tunnel. For discovered incomplete
// tunnels may be %NULL or null adapter port instead.
// @npaths: Number of paths in @paths
// @pre_activate: Optional tunnel specific initialization called before
// activation. Can touch hardware.
// @activate: Optional tunnel specific activation/deactivation
// @post_deactivate: Optional tunnel specific de-initialization called
// after deactivation. Can touch hardware.
// @destroy: Optional tunnel specific callback called when the tunnel
// memory is being released. Should not touch hardware.
// @maximum_bandwidth: Returns maximum possible bandwidth for this tunnel
// @allocated_bandwidth: Return how much bandwidth is allocated for the tunnel
// @alloc_bandwidth: Change tunnel bandwidth allocation
// @consumed_bandwidth: Return how much bandwidth the tunnel consumes
// @release_unused_bandwidth: Release all unused bandwidth
// @reclaim_available_bandwidth: Reclaim back available bandwidth
// @list: Tunnels are linked using this field
// @type: Type of the tunnel
// @state: Current state of the tunnel
// @max_up: Maximum upstream bandwidth (Mb/s) available for the tunnel.
// Only set if the bandwidth needs to be limited.
// @max_down: Maximum downstream bandwidth (Mb/s) available for the tunnel.
// Only set if the bandwidth needs to be limited.
// @allocated_up: Allocated upstream bandwidth (only for USB3)
// @allocated_down: Allocated downstream bandwidth (only for USB3)
// @bw_mode: DP bandwidth allocation mode registers can be used to
// determine consumed and allocated bandwidth
// @dprx_started: DPRX negotiation was started (tb_dp_dprx_start() was called for it)
// @dprx_canceled: Was DPRX capabilities read poll canceled
// @dprx_timeout: If set DPRX capabilities read poll work will timeout after this passes
// @dprx_work: Worker that is scheduled to poll completion of DPRX capabilities read
// @callback: Optional callback called when DP tunnel is fully activated
// @callback_data: Optional data for @callback
// @paths: All paths required by the tunnel
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_tunnel {
    pub kref: kref,
    pub tb: *mut tb,
    pub src_port: *mut tb_port,
    pub dst_port: *mut tb_port,
    pub npaths: usize,
    pub tunnel): *mut *mut int (pre_activate)(struct tb_tunnel,
    pub activate): *mut *mut *mut int (activate)(struct tb_tunnel tunnel, bool,
    pub tunnel): *mut *mut void (post_deactivate)(struct tb_tunnel,
    pub tunnel): *mut *mut void (destroy)(struct tb_tunnel,
    pub max_down): *mut c_int,
    pub allocated_down): *mut c_int,
    pub alloc_down): *mut c_int,
    pub consumed_down): *mut c_int,
    pub tunnel): *mut *mut int (release_unused_bandwidth)(struct tb_tunnel,
    pub available_down): *mut c_int,
    pub list: list_head,
    pub type: tb_tunnel_type,
    pub state: tb_tunnel_state,
    pub max_up: c_int,
    pub max_down: c_int,
    pub allocated_up: c_int,
    pub allocated_down: c_int,
    pub bw_mode: bool,
    pub dprx_started: bool,
    pub dprx_canceled: bool,
    pub dprx_timeout: ktime_t,
    pub dprx_work: delayed_work,
    pub data): *mut *mut *mut void (callback)(struct tb_tunnel tunnel, void,
    pub callback_data: *mut c_void,
    pub __counted_by(npaths): *mut *mut tb_path paths[],
}

extern "C" {
    pub fn tb_tunnel_put(tunnel: *mut tb_tunnel);
}
extern "C" {
    pub fn tb_tunnel_activate(tunnel: *mut tb_tunnel) -> c_int;
}
extern "C" {
    pub fn tb_tunnel_deactivate(tunnel: *mut tb_tunnel);
}
//
// tb_tunnel_is_active() - Is tunnel fully activated
// @tunnel: Tunnel to check
//
// Return: %true if @tunnel is fully activated.
//
// Note for DP tunnels this returns %true only once the DPRX capabilities
// read has been issued successfully. For other tunnels, this function
// returns %true pretty much once tb_tunnel_activate() returns successfully.
//
extern "C" {
    pub fn tb_tunnel_is_invalid(tunnel: *mut tb_tunnel) -> bool;
}
extern "C" {
    pub fn tb_tunnel_release_unused_bandwidth(tunnel: *mut tb_tunnel) -> c_int;
}
//
// enum tb_tunnel_event - Tunnel related events
// @TB_TUNNEL_ACTIVATED: A tunnel was activated
// @TB_TUNNEL_CHANGED: There is a tunneling change in the domain. Includes
// full %TUNNEL_DETAILS if the tunnel in question is known
// (ICM does not provide that information).
// @TB_TUNNEL_DEACTIVATED: A tunnel was torn down
// @TB_TUNNEL_LOW_BANDWIDTH: Tunnel bandwidth is not optimal
// @TB_TUNNEL_NO_BANDWIDTH: There is not enough bandwidth for a tunnel
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tb_tunnel_event {
    TB_TUNNEL_ACTIVATED,
    TB_TUNNEL_CHANGED,
    TB_TUNNEL_DEACTIVATED,
    TB_TUNNEL_LOW_BANDWIDTH,
    TB_TUNNEL_NO_BANDWIDTH,
}

