//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/link/protocols/link_dp_dpia_bw.h
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


//
// Copyright 2021 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: AMD
//

//
// Host Router BW type
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bw_type {
    HOST_ROUTER_BW_ESTIMATED,
    HOST_ROUTER_BW_ALLOCATED,
    HOST_ROUTER_BW_INVALID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb4_router_validation_set {
    pub is_valid: bool,
    pub cm_id: u8,
    pub dpia_count: u8,
    pub required_bw: u32,
    pub allocated_bw: u32,
    pub estimated_bw: u32,
    pub remaining_bw: u32,
}

//
// Enable USB4 DP BW allocation mode
//
// @link: pointer to the dc_link struct instance
//
// return: SUCCESS or FAILURE
//
extern "C" {
    pub fn link_dpia_enable_usb4_dp_bw_alloc_mode(link: *mut dc_link) -> bool;
}
//
// Allocates only what the stream needs for bw, so if:
// If (stream_req_bw < or > already_allocated_bw_at_HPD)
// => Deallocate Max Bw & then allocate only what the stream needs
//
// @link: pointer to the dc_link struct instance
// @req_bw: Bw requested by the stream
//
extern "C" {
    pub fn link_dp_dpia_allocate_usb4_bandwidth_for_stream(link: *mut dc_link, req_bw: c_int);
}
//
// Handle the USB4 BW Allocation related functionality here:
// Plug => Try to allocate max bw from timing parameters supported by the sink
// Unplug => de-allocate bw
//
// @link: pointer to the dc_link struct instance
// @peak_bw: Peak bw used by the link/sink
//
extern "C" {
    pub fn dpia_handle_usb4_bandwidth_allocation_for_link(link: *mut dc_link, peak_bw: c_int);
}
//
// Obtain all the DP overheads in dp tunneling for the dpia link
//
// @link: pointer to the dc_link struct instance
//
// return: DP overheads in DP tunneling
//
extern "C" {
    pub fn link_dpia_get_dp_overhead(link: *const dc_link) -> u32;
}
//
// Handle DP BW allocation status register
//
// @link: pointer to the dc_link struct instance
// @status: content of DP tunneling status register
//
// return: none
//
extern "C" {
    pub fn link_dp_dpia_handle_bw_alloc_status(link: *mut dc_link, status: u8);
}
//
// Aggregates the DPIA bandwidth usage for the respective USB4 Router.
//
// @dc_validation_dpia_set: pointer to the dc_validation_dpia_set
// @count: number of DPIA validation sets
//
// return: true if validation is succeeded
//
extern "C" {
    pub fn link_dpia_validate_dp_tunnel_bandwidth(dpia_link_sets: *const dc_validation_dpia_set, count: u8) -> bool;
}
