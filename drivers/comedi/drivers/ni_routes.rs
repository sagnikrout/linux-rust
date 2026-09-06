//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/comedi/drivers/ni_routes.h
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
// comedi/drivers/ni_routes.h
// Route information for NI boards.
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

//
// struct ni_route_set - Set of destinations with a common source.
// @dest: Destination of all sources in this route set.
// @n_src: Number of sources for this route set.
// @src: List of sources that all map to the same destination.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ni_route_set {
    pub dest: c_int,
    pub n_src: c_int,
    pub src: *mut c_int,
}

//
// struct ni_device_routes - List of all src->dest sets for a particular device.
// @device: Name of board/device (e.g. pxi-6733).
// @n_route_sets: Number of route sets that are valid for this device.
// @routes: List of route sets that are valid for this device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ni_device_routes {
    pub device: *const c_char,
    pub n_route_sets: c_int,
    pub routes: *mut ni_route_set,
}

//
// struct ni_route_tables - Register values and valid routes for a device.
// @valid_routes: Pointer to a all valid route sets for a single device.
// @route_values: Pointer to register values for all routes for the family to
// which the device belongs.
//
// Link to the valid src->dest routes and the register values used to assign
// such routes for that particular device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ni_route_tables {
    pub valid_routes: *const ni_device_routes,
    pub route_values: *const u8,
}

//
// ni_assign_device_routes() - Assign the proper lookup table for NI signal
// routing to the specified NI device.
//
// Return: -ENODATA if assignment was not successful; 0 if successful.
//
// ni_find_route_set() - Finds the proper route set with the specified
// destination.
// @destination: Destination of which to search for the route set.
// @valid_routes: Pointer to device routes within which to search.
//
// Return: NULL if no route_set is found with the specified @destination;
// otherwise, a pointer to the route_set if found.
//
// ni_route_set_has_source() - Determines whether the given source is in
// included given route_set.
//
// Return: true if found; false otherwise.
//
extern "C" {
    pub fn ni_route_set_has_source(routes: *const ni_route_set, src: c_int) -> bool;
}
//
// ni_route_to_register() - Validates and converts the specified signal route
// (src-->dest) to the value used at the appropriate
// register.
// @src:	global-identifier for route source
// @dest:	global-identifier for route destination
// @tables:	pointer to relevant set of routing tables.
//
// Generally speaking, most routes require the first six bits and a few require
// 7 bits.  Special handling is given for the return value when the route is to
// be handled by the RTSI sub-device.  In this case, the returned register may
// not be sufficient to define the entire route path, but rather may only
// indicate the intermediate route.  For example, if the route must go through
// the RGOUT0 pin, the (src->RGOUT0) register value will be returned.
// Similarly, if the route must go through the NI_RTSI_BRD lines, the BIT(6)
// will be set:
//
// if route does not need RTSI_BRD lines:
// bits 0:7 : register value
// for a route that must go through RGOUT0 pin, this will be equal
// to the (src->RGOUT0) register value.
// else: * route is (src->RTSI_BRD(x), RTSI_BRD(x)->TRIGGER_LINE(i))
// bits 0:5 : zero
// bits 6   : set to 1
// bits 7:7 : zero
//
// Return: register value to be used for source at destination with special
// cases given above; Otherwise, -1 if the specified route is not valid for
// this particular device.
//
// ni_lookup_route_register() - Look up a register value for a particular route
// without checking whether the route is valid for
// the particular device.
// @src:	global-identifier for route source
// @dest:	global-identifier for route destination
// @tables:	pointer to relevant set of routing tables.
//
// Return: -EINVAL if the specified route is not valid for this device family.
//
// route_is_valid() - Determines whether the specified signal route (src-->dest)
// is valid for the given NI comedi_device.
// @src:	global-identifier for route source
// @dest:	global-identifier for route destination
// @tables:	pointer to relevant set of routing tables.
//
// Return: True if the route is valid, otherwise false.
//
// ni_is_cmd_dest() - Determine whether the given destination is only
// configurable via a comedi_cmd struct.
// @dest: Destination to test.
//
extern "C" {
    pub fn ni_is_cmd_dest(dest: c_int) -> bool;
}
extern "C" {
    pub fn NI_PFI(NI_PFI(-1: 0) <= channel && channel <=) -> return;
}
extern "C" {
    pub fn TRIGGER_LINE(TRIGGER_LINE(-1: 0) <= channel && channel <=) -> return;
}
//
// ni_count_valid_routes() - Count the number of valid routes.
// @tables: Routing tables for which to count all valid routes.
//
extern "C" {
    pub fn ni_count_valid_routes(tables: *const ni_route_tables) -> c_uint;
}
//
// ni_get_valid_routes() - Implements INSN_DEVICE_CONFIG_GET_ROUTES.
// @tables:	pointer to relevant set of routing tables.
// @n_pairs:	Number of pairs for which memory is allocated by the user.  If
// the user specifies '0', only the number of available pairs is
// returned.
// @pair_data:	Pointer to memory allocated to return pairs back to user.  Each
// even, odd indexed member of this array will hold source,
// destination of a route pair respectively.
//
// Return: the number of valid routes if n_pairs == 0; otherwise, the number of
// valid routes copied.
//
// ni_sort_device_routes() - Sort the list of valid device signal routes in
// preparation for use.
// @valid_routes:	pointer to ni_device_routes struct to sort.
//
extern "C" {
    pub fn ni_sort_device_routes(valid_routes: *mut ni_device_routes);
}
//
// ni_find_route_source() - Finds the signal source corresponding to a signal
// route (src-->dest) of the specified routing register
// value and the specified route destination on the
// specified device.
//
// Note that this function does _not_ validate the source based on device
// routes.
//
// Return: The NI signal value (e.g. NI_PFI(0) or PXI_Clk10) if found.
// If the source was not found (i.e. the register value is not
// valid for any routes to the destination), -EINVAL is returned.
//
// route_register_is_valid() - Determines whether the register value for the
// specified route destination on the specified
// device is valid.
//
// ni_get_reg_value_roffs() - Determines the proper register value for a
// particular valid NI signal/terminal route.
// @src:	Either a direct register value or one of NI_* signal names.
// @dest:	global-identifier for route destination
// @tables:	pointer to relevant set of routing tables.
// @direct_reg_offset:
// Compatibility compensation argument.  This argument allows us to
// arbitrarily apply an offset to src if src is a direct register
// value reference.  This is necessary to be compatible with
// definitions of register values as previously exported directly
// to user space.
//
// Return: the register value (>0) to be used at the destination if the src is
// valid for the given destination; -1 otherwise.
//
// In this case, the src is expected to actually be a register
// value.
//
// Otherwise, the src is expected to be one of the abstracted NI
// signal/terminal names.
//
extern "C" {
    pub fn ni_route_to_register(_arg: src, _arg: dest, _arg: tables) -> return;
}
extern "C" {
    pub fn ni_get_reg_value_roffs(_arg: src, _arg: dest, _arg: tables, _arg: 0) -> return;
}
//
// ni_check_trigger_arg_roffs() - Checks the trigger argument (*_arg) of an NI
// device to ensure that the *_arg value
// corresponds to _either_ a valid register value
// to define a trigger source, _or_ a valid NI
// signal/terminal name that has a valid route to
// the destination on the particular device.
// @src:	Either a direct register value or one of NI_* signal names.
// @dest:	global-identifier for route destination
// @tables:	pointer to relevant set of routing tables.
// @direct_reg_offset:
// Compatibility compensation argument.  This argument allows us to
// arbitrarily apply an offset to src if src is a direct register
// value reference.  This is necessary to be compatible with
// definitions of register values as previously exported directly
// to user space.
//
// Return: 0 if the src (either register value or NI signal/terminal name) is
// valid for the destination; -EINVAL otherwise.
//
extern "C" {
    pub fn ni_check_trigger_arg_roffs(_arg: src, _arg: dest, _arg: tables, _arg: 0) -> return;
}
