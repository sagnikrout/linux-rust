//! Automatically rewritten from C to Rust
//! Source: drivers/comedi/drivers/ni_routing/ni_device_routes/pci-6534.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// comedi/drivers/ni_routing/ni_device_routes/pci-6534.c
// List of valid routes for specific NI boards.
//
// COMEDI - Linux Control and Measurement Device Interface
// Copyright (C) 2016 Spencer E. Olson <olsonse@umich.edu>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// The contents of this file are generated using the tools in
// comedi/drivers/ni_routing/tools
//
// Please use those tools to help maintain the contents of this file.
//

    struct ni_device_routes ni_pci_6534_device_routes = {
    .device = "pci-6534",
    .routes = (struct ni_route_set[]){
    {
    .dest = NI_PFI(0),
    .src = (int[]){
    TRIGGER_LINE(0),
    TRIGGER_LINE(1),
    TRIGGER_LINE(2),
    TRIGGER_LINE(3),
    TRIGGER_LINE(4),
    TRIGGER_LINE(5),
    TRIGGER_LINE(6),
    0, /* Termination */
    }
    },
    {
    .dest = NI_PFI(1),
    .src = (int[]){
    TRIGGER_LINE(0),
    TRIGGER_LINE(1),
    TRIGGER_LINE(2),
    TRIGGER_LINE(3),
    TRIGGER_LINE(4),
    TRIGGER_LINE(5),
    TRIGGER_LINE(6),
    0, /* Termination */
    }
    },
    {
    .dest = NI_PFI(2),
    .src = (int[]){
    TRIGGER_LINE(0),
    TRIGGER_LINE(1),
    TRIGGER_LINE(2),
    TRIGGER_LINE(3),
    TRIGGER_LINE(4),
    TRIGGER_LINE(5),
    TRIGGER_LINE(6),
    0, /* Termination */
    }
    },
    {
    .dest = NI_PFI(3),
    .src = (int[]){
    TRIGGER_LINE(0),
    TRIGGER_LINE(1),
    TRIGGER_LINE(2),
    TRIGGER_LINE(3),
    TRIGGER_LINE(4),
    TRIGGER_LINE(5),
    TRIGGER_LINE(6),
    0, /* Termination */
    }
    },
    {
    .dest = NI_PFI(4),
    .src = (int[]){
    TRIGGER_LINE(0),
    TRIGGER_LINE(1),
    TRIGGER_LINE(2),
    TRIGGER_LINE(3),
    TRIGGER_LINE(4),
    TRIGGER_LINE(5),
    TRIGGER_LINE(6),
    0, /* Termination */
    }
    },
    {
    .dest = NI_PFI(5),
    .src = (int[]){
    TRIGGER_LINE(0),
    TRIGGER_LINE(1),
    TRIGGER_LINE(2),
    TRIGGER_LINE(3),
    TRIGGER_LINE(4),
    TRIGGER_LINE(5),
    TRIGGER_LINE(6),
    0, /* Termination */
    }
    },
    {
    .dest = NI_PFI(6),
    .src = (int[]){
    TRIGGER_LINE(0),
    TRIGGER_LINE(1),
    TRIGGER_LINE(2),
    TRIGGER_LINE(3),
    TRIGGER_LINE(4),
    TRIGGER_LINE(5),
    TRIGGER_LINE(6),
    0, /* Termination */
    }
    },
    {
    .dest = NI_PFI(7),
    .src = (int[]){
    TRIGGER_LINE(0),
    TRIGGER_LINE(1),
    TRIGGER_LINE(2),
    TRIGGER_LINE(3),
    TRIGGER_LINE(4),
    TRIGGER_LINE(5),
    TRIGGER_LINE(6),
    0, /* Termination */
    }
    },
    {
    .dest = TRIGGER_LINE(0),
    .src = (int[]){
    NI_PFI(0),
    NI_PFI(1),
    NI_PFI(2),
    NI_PFI(3),
    NI_PFI(4),
    NI_PFI(5),
    NI_PFI(6),
    TRIGGER_LINE(1),
    TRIGGER_LINE(2),
    TRIGGER_LINE(3),
    TRIGGER_LINE(4),
    TRIGGER_LINE(5),
    TRIGGER_LINE(6),
    0, /* Termination */
    }
    },
    {
    .dest = TRIGGER_LINE(1),
    .src = (int[]){
    NI_PFI(0),
    NI_PFI(1),
    NI_PFI(2),
    NI_PFI(3),
    NI_PFI(4),
    NI_PFI(5),
    NI_PFI(6),
    TRIGGER_LINE(0),
    TRIGGER_LINE(2),
    TRIGGER_LINE(3),
    TRIGGER_LINE(4),
    TRIGGER_LINE(5),
    TRIGGER_LINE(6),
    0, /* Termination */
    }
    },
    {
    .dest = TRIGGER_LINE(2),
    .src = (int[]){
    NI_PFI(0),
    NI_PFI(1),
    NI_PFI(2),
    NI_PFI(3),
    NI_PFI(4),
    NI_PFI(5),
    NI_PFI(6),
    TRIGGER_LINE(0),
    TRIGGER_LINE(1),
    TRIGGER_LINE(3),
    TRIGGER_LINE(4),
    TRIGGER_LINE(5),
    TRIGGER_LINE(6),
    0, /* Termination */
    }
    },
    {
    .dest = TRIGGER_LINE(3),
    .src = (int[]){
    NI_PFI(0),
    NI_PFI(1),
    NI_PFI(2),
    NI_PFI(3),
    NI_PFI(4),
    NI_PFI(5),
    NI_PFI(6),
    TRIGGER_LINE(0),
    TRIGGER_LINE(1),
    TRIGGER_LINE(2),
    TRIGGER_LINE(4),
    TRIGGER_LINE(5),
    TRIGGER_LINE(6),
    0, /* Termination */
    }
    },
    {
    .dest = TRIGGER_LINE(4),
    .src = (int[]){
    NI_PFI(0),
    NI_PFI(1),
    NI_PFI(2),
    NI_PFI(3),
    NI_PFI(4),
    NI_PFI(5),
    NI_PFI(6),
    TRIGGER_LINE(0),
    TRIGGER_LINE(1),
    TRIGGER_LINE(2),
    TRIGGER_LINE(3),
    TRIGGER_LINE(5),
    TRIGGER_LINE(6),
    0, /* Termination */
    }
    },
    {
    .dest = TRIGGER_LINE(5),
    .src = (int[]){
    NI_PFI(0),
    NI_PFI(1),
    NI_PFI(2),
    NI_PFI(3),
    NI_PFI(4),
    NI_PFI(5),
    NI_PFI(6),
    TRIGGER_LINE(0),
    TRIGGER_LINE(1),
    TRIGGER_LINE(2),
    TRIGGER_LINE(3),
    TRIGGER_LINE(4),
    TRIGGER_LINE(6),
    0, /* Termination */
    }
    },
    {
    .dest = TRIGGER_LINE(6),
    .src = (int[]){
    NI_PFI(0),
    NI_PFI(1),
    NI_PFI(2),
    NI_PFI(3),
    NI_PFI(4),
    NI_PFI(5),
    NI_PFI(6),
    TRIGGER_LINE(0),
    TRIGGER_LINE(1),
    TRIGGER_LINE(2),
    TRIGGER_LINE(3),
    TRIGGER_LINE(4),
    TRIGGER_LINE(5),
    0, /* Termination */
    }
    },
    {
    .dest = TRIGGER_LINE(7),
    .src = (int[]){
    NI_20MHzTimebase,
    0, /* Termination */
    }
    },
    {
    .dest = NI_MasterTimebase,
    .src = (int[]){
    TRIGGER_LINE(7),
    NI_20MHzTimebase,
    0, /* Termination */
    }
    },
    { /* Termination of list */
    .dest = 0,
    },
    },
    };
