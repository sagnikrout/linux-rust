//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/of.h
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
// Definitions for talking to the Open Firmware PROM on
// Power Macintosh and other computers.
//
// Copyright (C) 1996-2005 Paul Mackerras.
//
// Updates for PPC64 by Peter Bergner & David Engebretsen, IBM Corp.
// Updates for SPARC64 by David S. Miller
// Derived from PowerPC and Sparc prom.h files by Stephen Rothwell, IBM Corp.
//

pub type phandle = u32;
pub type ihandle = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct property {
    pub name: *mut c_char,
    pub length: c_int,
    pub value: *mut c_void,
    pub next: *mut property,

    pub _flags: c_ulong,

    pub unique_id: c_uint,

    pub attr: bin_attribute,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_node {
    pub name: *const c_char,
    pub phandle: phandle,
    pub full_name: *const c_char,
    pub fwnode: fwnode_handle,
    pub properties: *mut property,
    pub /: *mut *mut *mut property deadprops; / removed properties,
    pub parent: *mut device_node,
    pub child: *mut device_node,
    pub sibling: *mut device_node,

    pub kobj: kobject,

    pub _flags: c_ulong,
    pub data: *mut c_void,

    pub unique_id: c_uint,
    pub irq_trans: *mut of_irq_controller,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct of_phandle_args {
    pub np: *mut device_node,
    pub args_count: c_int,
    pub args: [u32; MAX_PHANDLE_ARGS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct of_phandle_iterator {
// Common iterator information
    pub cells_name: *const c_char,
    pub cell_count: c_int,
    pub parent: *const device_node,
// List size information
    pub list_end: *const __be32,
    pub phandle_end: *const __be32,
// Current position state
    pub cur: *const __be32,
    pub cur_count: u32,
    pub phandle: phandle,
    pub node: *mut device_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct of_reconfig_data {
    pub dn: *mut device_node,
    pub prop: *mut property,
    pub old_prop: *mut property,
}

//
// of_node_init - initialize a devicetree node
// @node: Pointer to device node that has been created by kzalloc()
//
// On return the device_node refcount is set to one.  Use of_node_put()
// on @node when done to free the memory allocated for it.  If the node
// is NOT a dynamic node the memory will not be freed. The decision of
// whether to free the memory will be done by node->release(), which is
// of_node_release().
//

extern "C" {
    pub fn of_node_put(node: *mut device_node);
}

// Dummy ref counting routines - to be implemented later

// Pointer for first entry in chain of all nodes.
//
// struct device_node flag descriptions
// (need to be visible even when !CONFIG_OF)
//

extern "C" {
    pub fn of_core_init();
}

extern "C" {
    pub fn test_bit(_arg: flag, _arg: &n->_flags) -> return;
}
extern "C" {
    pub fn test_and_set_bit(_arg: flag, _arg: &n->_flags) -> return;
}

extern "C" {
    pub fn test_bit(_arg: flag, _arg: &p->_flags) -> return;
}

//
// OF address retrieval & translation
//
// Helper to read a big number; size is in cells (not bytes)
// Like of_read_number, but we want an unsigned long result
// toss away upper bits if unsigned long is smaller than u64
extern "C" {
    pub fn of_read_number(_arg: cell, _arg: size) -> return;
}

extern "C" {
    pub fn of_node_name_eq(np: *const device_node, name: *const c_char) -> bool;
}
extern "C" {
    pub fn of_node_name_prefix(np: *const device_node, prefix: *const c_char) -> bool;
}

extern "C" {
    pub fn of_find_node_opts_by_path(_arg: path, _arg: NULL) -> return;
}
// cache lookup
extern "C" {
    pub fn of_find_last_cache_level(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn of_property_read_bool(np: *const device_node, propname: *const c_char) -> bool;
}
extern "C" {
    pub fn of_device_is_available(device: *const device_node) -> bool;
}
extern "C" {
    pub fn of_device_is_big_endian(device: *const device_node) -> bool;
}
extern "C" {
    pub fn of_cpu_node_to_id(np: *mut device_node) -> c_int;
}
extern "C" {
    pub fn of_get_cpu_hwid(cpun: *mut device_node, thread: c_uint) -> u64;
}
extern "C" {
    pub fn of_n_addr_cells(np: *mut device_node) -> c_int;
}
extern "C" {
    pub fn of_n_size_cells(np: *mut device_node) -> c_int;
}
extern "C" {
    pub fn of_print_phandle_args(msg: *const c_char, args: *const of_phandle_args);
}
// module functions
extern "C" {
    pub fn of_modalias(np: *const device_node, str: *mut c_char, len: isize) -> isize;
}
extern "C" {
    pub fn of_request_module(np: *const device_node) -> c_int;
}
// phandle iterator functions
extern "C" {
    pub fn of_phandle_iterator_next(it: *mut of_phandle_iterator) -> c_int;
}
extern "C" {
    pub fn of_alias_get_id(np: *const device_node, stem: *const c_char) -> c_int;
}
extern "C" {
    pub fn of_alias_get_highest_id(stem: *const c_char) -> c_int;
}
extern "C" {
    pub fn of_machine_compatible_match(compats: *const *const c_char) -> bool;
}
//
// of_machine_is_compatible - Test root of device tree for a given compatible value
// @compat: compatible string to look for in root node's compatible property.
//
// Return: true if the root node has the given value in its compatible property.
//
extern "C" {
    pub fn of_machine_compatible_match(_arg: compats) -> return;
}
extern "C" {
    pub fn of_machine_read_compatible(compatible: *const c_char, index: c_uint) -> c_int;
}
extern "C" {
    pub fn of_machine_read_model(model: *const c_char) -> c_int;
}
extern "C" {
    pub fn of_add_property(np: *mut device_node, prop: *mut property) -> c_int;
}
extern "C" {
    pub fn of_remove_property(np: *mut device_node, prop: *mut property) -> c_int;
}
extern "C" {
    pub fn of_update_property(np: *mut device_node, newprop: *mut property) -> c_int;
}
// For updating the device tree at runtime
pub const OF_RECONFIG_ATTACH_NODE: c_uint = 0x0001;
pub const OF_RECONFIG_DETACH_NODE: c_uint = 0x0002;
pub const OF_RECONFIG_ADD_PROPERTY: c_uint = 0x0003;
pub const OF_RECONFIG_REMOVE_PROPERTY: c_uint = 0x0004;
pub const OF_RECONFIG_UPDATE_PROPERTY: c_uint = 0x0005;
extern "C" {
    pub fn of_attach_node(: *mut device_node) -> c_int;
}
extern "C" {
    pub fn of_detach_node(: *mut device_node) -> c_int;
}

//
// u32 u;
//
// of_property_for_each_u32(np, "propname", u)
// printk("U32 value: %x\n", u);
//
// struct property *prop;
// const char *s;
//
// of_property_for_each_string(np, "propname", prop, s)
// printk("String value: %s\n", s);
//
extern "C" {
    pub fn of_console_check(dn: *const device_node, name: *mut c_char, index: c_int) -> bool;
}
extern "C" {
    pub fn of_dma_get_max_cpu_address(np: *mut device_node) -> phys_addr_t;
}

// Default string compare functions, Allow arch asm/prom.h to override

extern "C" {
    pub fn of_node_to_nid(np: *mut device_node) -> c_int;
}

extern "C" {
    pub fn of_numa_init() -> c_int;
}

extern "C" {
    pub fn of_find_matching_node_and_match(_arg: from, _arg: matches, _arg: NULL) -> return;
}
extern "C" {
    pub fn of_get_property(_arg: np, _arg: "device_type", _arg: NULL) -> return;
}
//
// of_parse_phandle - Resolve a phandle property to a device_node pointer
// @np: Pointer to device node holding phandle property
// @phandle_name: Name of property holding a phandle value
// @index: For properties holding a table of phandles, this is the index into
// the table
//
// Return: The device_node pointer with refcount incremented.  Use
// of_node_put() on it when done.
//
// of_parse_phandle_with_args() - Find a node pointed by phandle in a list
// @np:		pointer to a device tree node containing a list
// @list_name:	property name that contains a list
// @cells_name:	property name that specifies phandles' arguments count
// @index:	index of a phandle to parse out
// @out_args:	optional pointer to output arguments structure (will be filled)
//
// This function is useful to parse lists of phandles and their arguments.
// Returns 0 on success and fills out_args, on error returns appropriate
// errno value.
//
// Caller is responsible to call of_node_put() on the returned out_args->np
// pointer.
//
// Example::
//
// phandle1: node1 {
// #list-cells = <2>;
// };
//
// phandle2: node2 {
// #list-cells = <1>;
// };
//
// node3 {
// list = <&phandle1 1 2 &phandle2 3>;
// };
//
// To get a device_node of the ``node2`` node you may call this:
// of_parse_phandle_with_args(node3, "list", "#list-cells", 1, &args);
//
// If cells_name is NULL we assume a cell count of 0
//
// of_parse_phandle_with_fixed_args() - Find a node pointed by phandle in a list
// @np:		pointer to a device tree node containing a list
// @list_name:	property name that contains a list
// @cell_count: number of argument cells following the phandle
// @index:	index of a phandle to parse out
// @out_args:	optional pointer to output arguments structure (will be filled)
//
// This function is useful to parse lists of phandles and their arguments.
// Returns 0 on success and fills out_args, on error returns appropriate
// errno value.
//
// Caller is responsible to call of_node_put() on the returned out_args->np
// pointer.
//
// Example::
//
// phandle1: node1 {
// };
//
// phandle2: node2 {
// };
//
// node3 {
// list = <&phandle1 0 2 &phandle2 2 3>;
// };
//
// To get a device_node of the ``node2`` node you may call this:
// of_parse_phandle_with_fixed_args(node3, "list", 2, 1, &args);
//
// of_parse_phandle_with_optional_args() - Find a node pointed by phandle in a list
// @np:		pointer to a device tree node containing a list
// @list_name:	property name that contains a list
// @cells_name:	property name that specifies phandles' arguments count
// @index:	index of a phandle to parse out
// @out_args:	optional pointer to output arguments structure (will be filled)
//
// Same as of_parse_phandle_with_args() except that if the cells_name property
// is not found, cell_count of 0 is assumed.
//
// This is used to useful, if you have a phandle which didn't have arguments
// before and thus doesn't have a '#*-cells' property but is now migrated to
// having arguments while retaining backwards compatibility.
//
// of_phandle_args_equal() - Compare two of_phandle_args
// @a1:		First of_phandle_args to compare
// @a2:		Second of_phandle_args to compare
//
// Return: True if a1 and a2 are the same (same node pointer, same phandle
// args), false otherwise.
//
// of_property_count_u8_elems - Count the number of u8 elements in a property
//
// @np:		device node from which the property value is to be read.
// @propname:	name of the property to be searched.
//
// Search for a property in a device node and count the number of u8 elements
// in it.
//
// Return: The number of elements on success, -EINVAL if the property does
// not exist or its length does not match a multiple of u8 and -ENODATA if the
// property does not have a value.
//
extern "C" {
    pub fn of_property_count_elems_of_size(_arg: np, _arg: propname, _arg: sizeof(u8)) -> return;
}
//
// of_property_count_u16_elems - Count the number of u16 elements in a property
//
// @np:		device node from which the property value is to be read.
// @propname:	name of the property to be searched.
//
// Search for a property in a device node and count the number of u16 elements
// in it.
//
// Return: The number of elements on success, -EINVAL if the property does
// not exist or its length does not match a multiple of u16 and -ENODATA if the
// property does not have a value.
//
extern "C" {
    pub fn of_property_count_elems_of_size(_arg: np, _arg: propname, _arg: sizeof(u16)) -> return;
}
//
// of_property_count_u32_elems - Count the number of u32 elements in a property
//
// @np:		device node from which the property value is to be read.
// @propname:	name of the property to be searched.
//
// Search for a property in a device node and count the number of u32 elements
// in it.
//
// Return: The number of elements on success, -EINVAL if the property does
// not exist or its length does not match a multiple of u32 and -ENODATA if the
// property does not have a value.
//
extern "C" {
    pub fn of_property_count_elems_of_size(_arg: np, _arg: propname, _arg: sizeof(u32)) -> return;
}
//
// of_property_count_u64_elems - Count the number of u64 elements in a property
//
// @np:		device node from which the property value is to be read.
// @propname:	name of the property to be searched.
//
// Search for a property in a device node and count the number of u64 elements
// in it.
//
// Return: The number of elements on success, -EINVAL if the property does
// not exist or its length does not match a multiple of u64 and -ENODATA if the
// property does not have a value.
//
extern "C" {
    pub fn of_property_count_elems_of_size(_arg: np, _arg: propname, _arg: sizeof(u64)) -> return;
}
//
// of_property_read_string_array() - Read an array of strings from a multiple
// strings property.
// @np:		device node from which the property value is to be read.
// @propname:	name of the property to be searched.
// @out_strs:	output array of string pointers.
// @sz:		number of array elements to read.
//
// Search for a property in a device tree node and retrieve a list of
// terminated string values (pointer to data, not a copy) in that property.
//
// Return: If @out_strs is NULL, the number of strings in the property is returned.
//
extern "C" {
    pub fn of_property_read_string_helper(_arg: np, _arg: propname, _arg: out_strs, _arg: sz, _arg: 0) -> return;
}
//
// of_property_count_strings() - Find and return the number of strings from a
// multiple strings property.
// @np:		device node from which the property value is to be read.
// @propname:	name of the property to be searched.
//
// Search for a property in a device tree node and retrieve the number of null
// terminated string contain in it.
//
// Return: The number of strings on success, -EINVAL if the property does not
// exist, -ENODATA if property does not have a value, and -EILSEQ if the string
// is not null-terminated within the length of the property data.
//
extern "C" {
    pub fn of_property_read_string_helper(_arg: np, _arg: propname, _arg: NULL, _arg: 0, _arg: 0) -> return;
}
//
// of_property_read_string_index() - Find and read a string from a multiple
// strings property.
// @np:		device node from which the property value is to be read.
// @propname:	name of the property to be searched.
// @index:	index of the string in the list of strings
// @output:	pointer to null terminated return string, modified only if
// return value is 0.
//
// Search for a property in a device tree node and retrieve a null
// terminated string value (pointer to data, not a copy) in the list of strings
// contained in that property.
//
// Return: 0 on success, -EINVAL if the property does not exist, -ENODATA if
// property does not have a value, and -EILSEQ if the string is not
// null-terminated within the length of the property data.
//
// The out_string pointer is modified only if a valid string can be decoded.
//
// of_property_present - Test if a property is present in a node
// @np:		device node to search for the property.
// @propname:	name of the property to be searched.
//
// Test for a property present in a device node.
//
// Return: true if the property exists false otherwise.
//
// of_property_read_u8_array - Find and read an array of u8 from a property.
//
// @np:		device node from which the property value is to be read.
// @propname:	name of the property to be searched.
// @out_values:	pointer to return value, modified only if return value is 0.
// @sz:		number of array elements to read
//
// Search for a property in a device node and read 8-bit value(s) from
// it.
//
// dts entry of array should be like:
// ``property = /bits/ 8 <0x50 0x60 0x70>;``
//
// Return: 0 on success, -EINVAL if the property does not exist,
// -ENODATA if property does not have a value, and -EOVERFLOW if the
// property data isn't large enough.
//
// The out_values is modified only if a valid u8 value can be decoded.
//
// of_property_read_u16_array - Find and read an array of u16 from a property.
//
// @np:		device node from which the property value is to be read.
// @propname:	name of the property to be searched.
// @out_values:	pointer to return value, modified only if return value is 0.
// @sz:		number of array elements to read
//
// Search for a property in a device node and read 16-bit value(s) from
// it.
//
// dts entry of array should be like:
// ``property = /bits/ 16 <0x5000 0x6000 0x7000>;``
//
// Return: 0 on success, -EINVAL if the property does not exist,
// -ENODATA if property does not have a value, and -EOVERFLOW if the
// property data isn't large enough.
//
// The out_values is modified only if a valid u16 value can be decoded.
//
// of_property_read_u32_array - Find and read an array of 32 bit integers
// from a property.
//
// @np:		device node from which the property value is to be read.
// @propname:	name of the property to be searched.
// @out_values:	pointer to return value, modified only if return value is 0.
// @sz:		number of array elements to read
//
// Search for a property in a device node and read 32-bit value(s) from
// it.
//
// Return: 0 on success, -EINVAL if the property does not exist,
// -ENODATA if property does not have a value, and -EOVERFLOW if the
// property data isn't large enough.
//
// The out_values is modified only if a valid u32 value can be decoded.
//
// of_property_read_u64_array - Find and read an array of 64 bit integers
// from a property.
//
// @np:		device node from which the property value is to be read.
// @propname:	name of the property to be searched.
// @out_values:	pointer to return value, modified only if return value is 0.
// @sz:		number of array elements to read
//
// Search for a property in a device node and read 64-bit value(s) from
// it.
//
// Return: 0 on success, -EINVAL if the property does not exist,
// -ENODATA if property does not have a value, and -EOVERFLOW if the
// property data isn't large enough.
//
// The out_values is modified only if a valid u64 value can be decoded.
//
extern "C" {
    pub fn of_property_read_u8_array(_arg: np, _arg: propname, _arg: out_value, _arg: 1) -> return;
}
extern "C" {
    pub fn of_property_read_u16_array(_arg: np, _arg: propname, _arg: out_value, _arg: 1) -> return;
}
extern "C" {
    pub fn of_property_read_u32_array(_arg: np, _arg: propname, _arg: out_value, _arg: 1) -> return;
}
extern "C" {
    pub fn of_property_read_u32(_arg: np, _arg: propname, out_value: *mut *mut (u32)) -> return;
}
extern "C" {
    pub fn of_property_read_u32_index(_arg: np, _arg: propname, _arg: index, )out_value: *mut (u32) -> return;
}

extern "C" {
    pub fn int(: *mut *mut of_init_fn_2)(struct device_node, : *mut device_node) -> typedef;
}
extern "C" {
    pub fn int(: *mut *mut of_init_fn_1_ret)(struct device_node) -> typedef;
}
extern "C" {
    pub fn void(: *mut *mut of_init_fn_1)(struct device_node) -> typedef;
}

//
// struct of_changeset_entry	- Holds a changeset entry
//
// @node:	list_head for the log list
// @action:	notifier action
// @np:		pointer to the device node affected
// @prop:	pointer to the property affected
// @old_prop:	hold a pointer to the original property
//
// Every modification of the device tree during a changeset
// is held in a list of of_changeset_entry structures.
// That way we can recover from a partial application, or we can
// revert the changeset
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct of_changeset_entry {
    pub node: list_head,
    pub action: c_ulong,
    pub np: *mut device_node,
    pub prop: *mut property,
    pub old_prop: *mut property,
}

//
// struct of_changeset - changeset tracker structure
//
// @entries:	list_head for the changeset entries
//
// changesets are a convenient way to apply bulk changes to the
// live tree. In case of an error, changes are rolled-back.
// changesets live on after initial application, and if not
// destroyed after use, they can be reverted in one single call.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct of_changeset {
    pub entries: list_head,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum of_reconfig_change {
    OF_RECONFIG_NO_CHANGE = 0,
    OF_RECONFIG_CHANGE_ADD,
    OF_RECONFIG_CHANGE_REMOVE,
}

extern "C" {
    pub fn of_reconfig_notifier_register(: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn of_reconfig_notifier_unregister(: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn of_reconfig_notify(long: unsigned, rd: *mut of_reconfig_data) -> c_int;
}
extern "C" {
    pub fn of_changeset_init(ocs: *mut of_changeset);
}
extern "C" {
    pub fn of_changeset_destroy(ocs: *mut of_changeset);
}
extern "C" {
    pub fn of_changeset_apply(ocs: *mut of_changeset) -> c_int;
}
extern "C" {
    pub fn of_changeset_revert(ocs: *mut of_changeset) -> c_int;
}
extern "C" {
    pub fn of_changeset_action(_arg: ocs, _arg: OF_RECONFIG_ATTACH_NODE, _arg: np, _arg: NULL) -> return;
}
extern "C" {
    pub fn of_changeset_action(_arg: ocs, _arg: OF_RECONFIG_DETACH_NODE, _arg: np, _arg: NULL) -> return;
}
extern "C" {
    pub fn of_changeset_action(_arg: ocs, _arg: OF_RECONFIG_ADD_PROPERTY, _arg: np, _arg: prop) -> return;
}
extern "C" {
    pub fn of_changeset_action(_arg: ocs, _arg: OF_RECONFIG_REMOVE_PROPERTY, _arg: np, _arg: prop) -> return;
}
extern "C" {
    pub fn of_changeset_action(_arg: ocs, _arg: OF_RECONFIG_UPDATE_PROPERTY, _arg: np, _arg: prop) -> return;
}
extern "C" {
    pub fn of_changeset_add_prop_u32_array(_arg: ocs, _arg: np, _arg: prop_name, _arg: &val, _arg: 1) -> return;
}

//
// of_device_is_system_power_controller - Tells if system-power-controller is found for device_node
// @np: Pointer to the given device_node
//
// Return: true if present false otherwise
//
extern "C" {
    pub fn of_property_read_bool(_arg: np, _arg: "system-power-controller") -> return;
}
//
// of_have_populated_dt() - Has DT been populated by bootloader
//
// Return: True if a DTB has been populated by the bootloader and it isn't the
// empty builtin one. False otherwise.
//

extern "C" {
    pub fn of_property_present(_arg: of_root, _arg: "compatible") -> return;
}

//
// Overlay support
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum of_overlay_notify_action {
    OF_OVERLAY_INIT = 0,	/* kzalloc() of ovcs sets this value */
    OF_OVERLAY_PRE_APPLY,
    OF_OVERLAY_POST_APPLY,
    OF_OVERLAY_PRE_REMOVE,
    OF_OVERLAY_POST_REMOVE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct of_overlay_notify_data {
    pub overlay: *mut device_node,
    pub target: *mut device_node,
}

extern "C" {
    pub fn of_overlay_remove(ovcs_id: *mut c_int) -> c_int;
}
extern "C" {
    pub fn of_overlay_remove_all() -> c_int;
}
extern "C" {
    pub fn of_overlay_notifier_register(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn of_overlay_notifier_unregister(nb: *mut notifier_block) -> c_int;
}

