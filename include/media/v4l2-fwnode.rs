//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/v4l2-fwnode.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// V4L2 fwnode binding parsing library
//
// Copyright (c) 2016 Intel Corporation.
// Author: Sakari Ailus <sakari.ailus@linux.intel.com>
//
// Copyright (C) 2012 - 2013 Samsung Electronics Co., Ltd.
// Author: Sylwester Nawrocki <s.nawrocki@samsung.com>
//
// Copyright (C) 2012 Renesas Electronics Corp.
// Author: Guennadi Liakhovetski <g.liakhovetski@gmx.de>
//

//
// struct v4l2_fwnode_endpoint - the endpoint data structure
// @base: fwnode endpoint of the v4l2_fwnode
// @bus_type: bus type
// @bus: bus configuration data structure
// @bus.parallel: embedded &struct v4l2_mbus_config_parallel.
// Used if the bus is parallel.
// @bus.mipi_csi1: embedded &struct v4l2_mbus_config_mipi_csi1.
// Used if the bus is MIPI Alliance's Camera Serial
// Interface version 1 (MIPI CSI1) or Standard
// Mobile Imaging Architecture's Compact Camera Port 2
// (SMIA CCP2).
// @bus.mipi_csi2: embedded &struct v4l2_mbus_config_mipi_csi2.
// Used if the bus is MIPI Alliance's Camera Serial
// Interface version 2 (MIPI CSI2).
// @link_frequencies: array of supported link frequencies
// @nr_of_link_frequencies: number of elements in link_frequenccies array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_fwnode_endpoint {
    pub base: fwnode_endpoint,
    pub bus_type: v4l2_mbus_type,
    pub parallel: v4l2_mbus_config_parallel,
    pub mipi_csi1: v4l2_mbus_config_mipi_csi1,
    pub mipi_csi2: v4l2_mbus_config_mipi_csi2,
    pub bus: },
    pub link_frequencies: *mut u64,
    pub nr_of_link_frequencies: c_uint,
}

//
// V4L2_FWNODE_PROPERTY_UNSET - identify a non initialized property
//
// All properties in &struct v4l2_fwnode_device_properties are initialized
// to this value.
//

//
// enum v4l2_fwnode_orientation - possible device orientation
// @V4L2_FWNODE_ORIENTATION_FRONT: device installed on the front side
// @V4L2_FWNODE_ORIENTATION_BACK: device installed on the back side
// @V4L2_FWNODE_ORIENTATION_EXTERNAL: device externally located
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_fwnode_orientation {
    V4L2_FWNODE_ORIENTATION_FRONT,
    V4L2_FWNODE_ORIENTATION_BACK,
    V4L2_FWNODE_ORIENTATION_EXTERNAL
}

//
// struct v4l2_fwnode_device_properties - fwnode device properties
// @orientation: device orientation. See &enum v4l2_fwnode_orientation
// @rotation: device rotation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_fwnode_device_properties {
    pub orientation: v4l2_fwnode_orientation,
    pub rotation: c_uint,
}

//
// struct v4l2_fwnode_link - a link between two endpoints
// @local_node: pointer to device_node of this endpoint
// @local_port: identifier of the port this endpoint belongs to
// @local_id: identifier of the id this endpoint belongs to
// @remote_node: pointer to device_node of the remote endpoint
// @remote_port: identifier of the port the remote endpoint belongs to
// @remote_id: identifier of the id the remote endpoint belongs to
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_fwnode_link {
    pub local_node: *mut fwnode_handle,
    pub local_port: c_uint,
    pub local_id: c_uint,
    pub remote_node: *mut fwnode_handle,
    pub remote_port: c_uint,
    pub remote_id: c_uint,
}

//
// enum v4l2_connector_type - connector type
// @V4L2_CONN_UNKNOWN:   unknown connector type, no V4L2 connector configuration
// @V4L2_CONN_COMPOSITE: analog composite connector
// @V4L2_CONN_SVIDEO:    analog svideo connector
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_connector_type {
    V4L2_CONN_UNKNOWN,
    V4L2_CONN_COMPOSITE,
    V4L2_CONN_SVIDEO,
}

//
// struct v4l2_connector_link - connector link data structure
// @head: structure to be used to add the link to the
// &struct v4l2_fwnode_connector
// @fwnode_link: &struct v4l2_fwnode_link link between the connector and the
// device the connector belongs to.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_connector_link {
    pub head: list_head,
    pub fwnode_link: v4l2_fwnode_link,
}

//
// struct v4l2_fwnode_connector_analog - analog connector data structure
// @sdtv_stds: sdtv standards this connector supports, set to V4L2_STD_ALL
// if no restrictions are specified.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_fwnode_connector_analog {
    pub sdtv_stds: v4l2_std_id,
}

//
// struct v4l2_fwnode_connector - the connector data structure
// @name: the connector device name
// @label: optional connector label
// @type: connector type
// @links: list of all connector &struct v4l2_connector_link links
// @nr_of_links: total number of links
// @connector: connector configuration
// @connector.analog: analog connector configuration
// &struct v4l2_fwnode_connector_analog
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_fwnode_connector {
    pub name: *const c_char,
    pub label: *const c_char,
    pub type: v4l2_connector_type,
    pub links: list_head,
    pub nr_of_links: c_uint,
    pub analog: v4l2_fwnode_connector_analog,
// future connectors
    pub connector: },
}

//
// enum v4l2_fwnode_bus_type - Video bus types defined by firmware properties
// @V4L2_FWNODE_BUS_TYPE_GUESS: Default value if no bus-type fwnode property
// @V4L2_FWNODE_BUS_TYPE_CSI2_CPHY: MIPI CSI-2 bus, C-PHY physical layer
// @V4L2_FWNODE_BUS_TYPE_CSI1: MIPI CSI-1 bus
// @V4L2_FWNODE_BUS_TYPE_CCP2: SMIA Compact Camera Port 2 bus
// @V4L2_FWNODE_BUS_TYPE_CSI2_DPHY: MIPI CSI-2 bus, D-PHY physical layer
// @V4L2_FWNODE_BUS_TYPE_PARALLEL: Camera Parallel Interface bus
// @V4L2_FWNODE_BUS_TYPE_BT656: BT.656 video format bus-type
// @V4L2_FWNODE_BUS_TYPE_DPI: Video Parallel Interface bus
// @NR_OF_V4L2_FWNODE_BUS_TYPE: Number of bus-types
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_fwnode_bus_type {
    V4L2_FWNODE_BUS_TYPE_GUESS = 0,
    V4L2_FWNODE_BUS_TYPE_CSI2_CPHY,
    V4L2_FWNODE_BUS_TYPE_CSI1,
    V4L2_FWNODE_BUS_TYPE_CCP2,
    V4L2_FWNODE_BUS_TYPE_CSI2_DPHY,
    V4L2_FWNODE_BUS_TYPE_PARALLEL,
    V4L2_FWNODE_BUS_TYPE_BT656,
    V4L2_FWNODE_BUS_TYPE_DPI,
    NR_OF_V4L2_FWNODE_BUS_TYPE
}

//
// v4l2_fwnode_endpoint_parse() - parse all fwnode node properties
// @fwnode: pointer to the endpoint's fwnode handle (may be NULL)
// @vep: pointer to the V4L2 fwnode data structure
//
// This function parses the V4L2 fwnode endpoint specific parameters from the
// firmware. There are two ways to use this function, either by letting it
// obtain the type of the bus (by setting the @vep.bus_type field to
// V4L2_MBUS_UNKNOWN) or specifying the bus type explicitly to one of the &enum
// v4l2_mbus_type types.
//
// When @vep.bus_type is V4L2_MBUS_UNKNOWN, the function will use the "bus-type"
// property to determine the type when it is available. The caller is
// responsible for validating the contents of @vep.bus_type field after the call
// returns.
//
// As a deprecated functionality to support older DT bindings without "bus-type"
// property for devices that support multiple types, if the "bus-type" property
// does not exist, the function will attempt to guess the type based on the
// endpoint properties available. NEVER RELY ON GUESSING THE BUS TYPE IN NEW
// DRIVERS OR BINDINGS.
//
// It is also possible to set @vep.bus_type corresponding to an actual bus. In
// this case the function will only attempt to parse properties related to this
// bus, and it will return an error if the value of the "bus-type" property
// corresponds to a different bus.
//
// The caller is required to initialise all fields of @vep, either with
// explicitly values, or by zeroing them.
//
// The function does not change the V4L2 fwnode endpoint state if it fails.
//
// NOTE: This function does not parse "link-frequencies" property as its size is
// not known in advance. Please use v4l2_fwnode_endpoint_alloc_parse() if you
// need properties of variable size.
//
// Return: %0 on success or a negative error code on failure:
// %-ENOMEM on memory allocation failure
// %-EINVAL on parsing failure
// %-ENXIO on mismatching bus types
// %-EPROBE_DEFER on NULL @fwnode
//
// v4l2_fwnode_endpoint_free() - free the V4L2 fwnode acquired by
// v4l2_fwnode_endpoint_alloc_parse()
// @vep: the V4L2 fwnode the resources of which are to be released
//
// It is safe to call this function with NULL argument or on a V4L2 fwnode the
// parsing of which failed.
//
extern "C" {
    pub fn v4l2_fwnode_endpoint_free(vep: *mut v4l2_fwnode_endpoint);
}
//
// v4l2_fwnode_endpoint_alloc_parse() - parse all fwnode node properties
// @fwnode: pointer to the endpoint's fwnode handle (may be NULL)
// @vep: pointer to the V4L2 fwnode data structure
//
// This function parses the V4L2 fwnode endpoint specific parameters from the
// firmware. There are two ways to use this function, either by letting it
// obtain the type of the bus (by setting the @vep.bus_type field to
// V4L2_MBUS_UNKNOWN) or specifying the bus type explicitly to one of the &enum
// v4l2_mbus_type types.
//
// When @vep.bus_type is V4L2_MBUS_UNKNOWN, the function will use the "bus-type"
// property to determine the type when it is available. The caller is
// responsible for validating the contents of @vep.bus_type field after the call
// returns.
//
// As a deprecated functionality to support older DT bindings without "bus-type"
// property for devices that support multiple types, if the "bus-type" property
// does not exist, the function will attempt to guess the type based on the
// endpoint properties available. NEVER RELY ON GUESSING THE BUS TYPE IN NEW
// DRIVERS OR BINDINGS.
//
// It is also possible to set @vep.bus_type corresponding to an actual bus. In
// this case the function will only attempt to parse properties related to this
// bus, and it will return an error if the value of the "bus-type" property
// corresponds to a different bus.
//
// The caller is required to initialise all fields of @vep, either with
// explicitly values, or by zeroing them.
//
// The function does not change the V4L2 fwnode endpoint state if it fails.
//
// v4l2_fwnode_endpoint_alloc_parse() has two important differences to
// v4l2_fwnode_endpoint_parse():
//
// 1. It also parses variable size data.
//
// 2. The memory it has allocated to store the variable size data must be freed
// using v4l2_fwnode_endpoint_free() when no longer needed.
//
// Return: %0 on success or a negative error code on failure:
// %-ENOMEM on memory allocation failure
// %-EINVAL on parsing failure
// %-ENXIO on mismatching bus types
// %-EPROBE_DEFER on NULL @fwnode
//
// v4l2_fwnode_parse_link() - parse a link between two endpoints
// @fwnode: pointer to the endpoint's fwnode at the local end of the link
// @link: pointer to the V4L2 fwnode link data structure
//
// Fill the link structure with the local and remote nodes and port numbers.
// The local_node and remote_node fields are set to point to the local and
// remote port's parent nodes respectively (the port parent node being the
// parent node of the port node if that node isn't a 'ports' node, or the
// grand-parent node of the port node otherwise).
//
// A reference is taken to both the local and remote nodes, the caller must use
// v4l2_fwnode_put_link() to drop the references when done with the
// link.
//
// Return: 0 on success, or -ENOLINK if the remote endpoint fwnode can't be
// found.
//
// v4l2_fwnode_put_link() - drop references to nodes in a link
// @link: pointer to the V4L2 fwnode link data structure
//
// Drop references to the local and remote nodes in the link. This function
// must be called on every link parsed with v4l2_fwnode_parse_link().
//
extern "C" {
    pub fn v4l2_fwnode_put_link(link: *mut v4l2_fwnode_link);
}
//
// v4l2_fwnode_connector_free() - free the V4L2 connector acquired memory
// @connector: the V4L2 connector resources of which are to be released
//
// Free all allocated memory and put all links acquired by
// v4l2_fwnode_connector_parse() and v4l2_fwnode_connector_add_link().
//
// It is safe to call this function with NULL argument or on a V4L2 connector
// the parsing of which failed.
//
extern "C" {
    pub fn v4l2_fwnode_connector_free(connector: *mut v4l2_fwnode_connector);
}
//
// v4l2_fwnode_connector_parse() - initialize the 'struct v4l2_fwnode_connector'
// @fwnode: pointer to the subdev endpoint's fwnode handle where the connector
// is connected to or to the connector endpoint fwnode handle.
// @connector: pointer to the V4L2 fwnode connector data structure
//
// Fill the &struct v4l2_fwnode_connector with the connector type, label and
// all &enum v4l2_connector_type specific connector data. The label is optional
// so it is set to %NULL if no one was found. The function initialize the links
// to zero. Adding links to the connector is done by calling
// v4l2_fwnode_connector_add_link().
//
// The memory allocated for the label must be freed when no longer needed.
// Freeing the memory is done by v4l2_fwnode_connector_free().
//
// Return:
// * %0 on success or a negative error code on failure:
// * %-EINVAL if @fwnode is invalid
// * %-ENOTCONN if connector type is unknown or connector device can't be found
//
// v4l2_fwnode_connector_add_link - add a link between a connector node and
// a v4l2-subdev node.
// @fwnode: pointer to the subdev endpoint's fwnode handle where the connector
// is connected to
// @connector: pointer to the V4L2 fwnode connector data structure
//
// Add a new &struct v4l2_connector_link link to the
// &struct v4l2_fwnode_connector connector links list. The link local_node
// points to the connector node, the remote_node to the host v4l2 (sub)dev.
//
// The taken references to remote_node and local_node must be dropped and the
// allocated memory must be freed when no longer needed. Both is done by calling
// v4l2_fwnode_connector_free().
//
// Return:
// * %0 on success or a negative error code on failure:
// * %-EINVAL if @fwnode or @connector is invalid or @connector type is unknown
// * %-ENOMEM on link memory allocation failure
// * %-ENOTCONN if remote connector device can't be found
// * %-ENOLINK if link parsing between v4l2 (sub)dev and connector fails
//
// v4l2_fwnode_device_parse() - parse fwnode device properties
// @dev: pointer to &struct device
// @props: pointer to &struct v4l2_fwnode_device_properties where to store the
// parsed properties values
//
// This function parses and validates the V4L2 fwnode device properties from the
// firmware interface, and fills the @struct v4l2_fwnode_device_properties
// provided by the caller.
//
// Return:
// % 0 on success
// %-EINVAL if a parsed property value is not valid
//
// Helper macros to access the connector links.
// v4l2_connector_last_link - Helper macro to get the first
// &struct v4l2_fwnode_connector link
// @v4l2c: &struct v4l2_fwnode_connector owning the connector links
//
// This marco returns the first added &struct v4l2_connector_link connector
// link or @NULL if the connector has no links.
//

// v4l2_connector_last_link - Helper macro to get the last
// &struct v4l2_fwnode_connector link
// @v4l2c: &struct v4l2_fwnode_connector owning the connector links
//
// This marco returns the last &struct v4l2_connector_link added connector link.
//

