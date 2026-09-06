//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/vga_switcheroo.h
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
// vga_switcheroo.h - Support for laptop with dual GPU using one set of outputs
//
// Copyright (c) 2010 Red Hat Inc.
// Author : Dave Airlie <airlied@redhat.com>
//
// Copyright (c) 2015 Lukas Wunner <lukas@wunner.de>
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS
// IN THE SOFTWARE.
//

//
// enum vga_switcheroo_handler_flags_t - handler flags bitmask
// @VGA_SWITCHEROO_CAN_SWITCH_DDC: whether the handler is able to switch the
// DDC lines separately. This signals to clients that they should call
// drm_get_edid_switcheroo() to probe the EDID
// @VGA_SWITCHEROO_NEEDS_EDP_CONFIG: whether the handler is unable to switch
// the AUX channel separately. This signals to clients that the active
// GPU needs to train the link and communicate the link parameters to the
// inactive GPU (mediated by vga_switcheroo). The inactive GPU may then
// skip the AUX handshake and set up its output with these pre-calibrated
// values (DisplayPort specification v1.1a, section 2.5.3.3)
//
// Handler flags bitmask. Used by handlers to declare their capabilities upon
// registering with vga_switcheroo.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vga_switcheroo_handler_flags_t {
    VGA_SWITCHEROO_CAN_SWITCH_DDC	= (1 << 0),
    VGA_SWITCHEROO_NEEDS_EDP_CONFIG	= (1 << 1),
}

//
// enum vga_switcheroo_state - client power state
// @VGA_SWITCHEROO_OFF: off
// @VGA_SWITCHEROO_ON: on
// @VGA_SWITCHEROO_NOT_FOUND: client has not registered with vga_switcheroo.
// Only used in vga_switcheroo_get_client_state() which in turn is only
// called from hda_intel.c
//
// Client power state.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vga_switcheroo_state {
    VGA_SWITCHEROO_OFF,
    VGA_SWITCHEROO_ON,
// below are referred only from vga_switcheroo_get_client_state()
    VGA_SWITCHEROO_NOT_FOUND,
}

//
// enum vga_switcheroo_client_id - client identifier
// @VGA_SWITCHEROO_UNKNOWN_ID: initial identifier assigned to vga clients.
// Determining the id requires the handler, so GPUs are given their
// true id in a delayed fashion in vga_switcheroo_enable()
// @VGA_SWITCHEROO_IGD: integrated graphics device
// @VGA_SWITCHEROO_DIS: discrete graphics device
// @VGA_SWITCHEROO_MAX_CLIENTS: currently no more than two GPUs are supported
//
// Client identifier. Audio clients use the same identifier & 0x100.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vga_switcheroo_client_id {
    VGA_SWITCHEROO_UNKNOWN_ID = 0x1000,
    VGA_SWITCHEROO_IGD = 0,
    VGA_SWITCHEROO_DIS,
    VGA_SWITCHEROO_MAX_CLIENTS,
}

//
// struct vga_switcheroo_handler - handler callbacks
// @init: initialize handler.
// Optional. This gets called when vga_switcheroo is enabled, i.e. when
// two vga clients have registered. It allows the handler to perform
// some delayed initialization that depends on the existence of the
// vga clients. Currently only the radeon and amdgpu drivers use this.
// The return value is ignored
// @switchto: switch outputs to given client.
// Mandatory. For muxless machines this should be a no-op. Returning 0
// denotes success, anything else failure (in which case the switch is
// aborted)
// @switch_ddc: switch DDC lines to given client.
// Optional. Should return the previous DDC owner on success or a
// negative int on failure
// @power_state: cut or reinstate power of given client.
// Optional. The return value is ignored
// @get_client_id: determine if given pci device is integrated or discrete GPU.
// Mandatory
//
// Handler callbacks. The multiplexer itself. The @switchto and @get_client_id
// methods are mandatory, all others may be set to NULL.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vga_switcheroo_handler {
    pub (*init)(void): *mut c_int,
    pub id): *mut *mut int (switchto)(enum vga_switcheroo_client_id,
    pub id): *mut *mut int (switch_ddc)(enum vga_switcheroo_client_id,
    pub state): vga_switcheroo_state,
    pub pdev): *mut *mut vga_switcheroo_client_id (get_client_id)(struct pci_dev,
}

//
// struct vga_switcheroo_client_ops - client callbacks
// @set_gpu_state: do the equivalent of suspend/resume for the card.
// Mandatory. This should not cut power to the discrete GPU,
// which is the job of the handler
// @reprobe: poll outputs.
// Optional. This gets called after waking the GPU and switching
// the outputs to it
// @can_switch: check if the device is in a position to switch now.
// Mandatory. The client should return false if a user space process
// has one of its device files open
// @gpu_bound: notify the client id to audio client when the GPU is bound.
//
// Client callbacks. A client can be either a GPU or an audio device on a GPU.
// The @set_gpu_state and @can_switch methods are mandatory, @reprobe may be
// set to NULL. For audio clients, the @reprobe member is bogus.
// OTOH, @gpu_bound is only for audio clients, and not used for GPU clients.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vga_switcheroo_client_ops {
    pub vga_switcheroo_state): *mut *mut *mut void (set_gpu_state)(struct pci_dev dev, enum,
    pub dev): *mut *mut void (reprobe)(struct pci_dev,
    pub dev): *mut *mut bool (can_switch)(struct pci_dev,
    pub vga_switcheroo_client_id): *mut *mut *mut void (gpu_bound)(struct pci_dev dev, enum,
}

extern "C" {
    pub fn vga_switcheroo_unregister_client(dev: *mut pci_dev);
}
extern "C" {
    pub fn vga_switcheroo_unregister_handler();
}
extern "C" {
    pub fn vga_switcheroo_handler_flags() -> vga_switcheroo_handler_flags_t;
}
extern "C" {
    pub fn vga_switcheroo_lock_ddc(pdev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn vga_switcheroo_unlock_ddc(pdev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn vga_switcheroo_process_delayed_switch() -> c_int;
}
extern "C" {
    pub fn vga_switcheroo_client_probe_defer(pdev: *mut pci_dev) -> bool;
}
extern "C" {
    pub fn vga_switcheroo_get_client_state(dev: *mut pci_dev) -> vga_switcheroo_state;
}
extern "C" {
    pub fn vga_switcheroo_init_domain_pm_ops(dev: *mut device, domain: *mut dev_pm_domain) -> c_int;
}
extern "C" {
    pub fn vga_switcheroo_fini_domain_pm_ops(dev: *mut device);
}

