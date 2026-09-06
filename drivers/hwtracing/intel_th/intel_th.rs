//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hwtracing/intel_th/intel_th.h
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
// Intel(R) Trace Hub data structures
//
// Copyright (C) 2014-2015 Intel Corporation.
//

// intel_th_device device types
// Devices that generate trace data
// Output ports (MSC, PTI)
// Switch, the Global Trace Hub (GTH)
//
// struct intel_th_output - descriptor INTEL_TH_OUTPUT type devices
// @port:	output port number, assigned by the switch
// @type:	GTH_{MSU,CTP,PTI}
// @scratchpad:	scratchpad bits to flag when this output is enabled
// @multiblock:	true for multiblock output configuration
// @active:	true when this output is enabled
// @wait_empty:	wait for device pipeline to be empty
//
// Output port descriptor, used by switch driver to tell which output
// port this output device corresponds to. Filled in at output device's
// probe time by switch::assign(). Passed from output device driver to
// switch related code to enable/disable its port.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_th_output {
    pub port: c_int,
    pub type: c_uint,
    pub scratchpad: c_uint,
    pub multiblock: bool,
    pub active: bool,
}

//
// struct intel_th_drvdata - describes hardware capabilities and quirks
// @tscu_enable:	device needs SW to enable time stamping unit
// @multi_is_broken:	device has multiblock mode is broken
// @has_mintctl:	device has interrupt control (MINTCTL) register
// @host_mode_only:	device can only operate in 'host debugger' mode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_th_drvdata {
    pub 1: host_mode_only :,
}

//
// struct intel_th_device - device on the intel_th bus
// @dev:		device
// @drvdata:		hardware capabilities/quirks
// @resource:		array of resources available to this device
// @num_resources:	number of resources in @resource array
// @type:		INTEL_TH_{SOURCE,OUTPUT,SWITCH}
// @id:			device instance or -1
// @host_mode:		Intel TH is controlled by an external debug host
// @output:		output descriptor for INTEL_TH_OUTPUT devices
// @name:		device name to match the driver
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_th_device {
    pub dev: device,
    pub drvdata: *const intel_th_drvdata,
    pub resource: *mut resource,
    pub num_resources: c_uint,
    pub type: c_uint,
    pub id: c_int,
// INTEL_TH_SWITCH specific
    pub host_mode: bool,
// INTEL_TH_OUTPUT specific
    pub output: intel_th_output,
    pub name: [c_char; ],
}

//
// intel_th_device_get_resource() - obtain @num'th resource of type @type
// @thdev:	the device to search the resource for
// @type:	resource type
// @num:	number of the resource
//
// GTH, output ports configuration
//
// intel_th_output_assigned() - if an output device is assigned to a switch port
// @thdev:	the output device
//
// Return:	true if the device is INTEL_TH_OUTPUT *and* is assigned a port
//
// struct intel_th_driver - driver for an intel_th_device device
// @driver:	generic driver
// @probe:	probe method
// @remove:	remove method
// @assign:	match a given output type device against available outputs
// @unassign:	deassociate an output type device from an output port
// @prepare:	prepare output port for tracing
// @enable:	enable tracing for a given output device
// @disable:	disable tracing for a given output device
// @irq:	interrupt callback
// @activate:	enable tracing on the output's side
// @deactivate:	disable tracing on the output's side
// @fops:	file operations for device nodes
// @attr_group:	attributes provided by the driver
//
// Callbacks @probe and @remove are required for all device types.
// Switch device driver needs to fill in @assign, @enable and @disable
// callbacks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_th_driver {
    pub driver: device_driver,
    pub thdev): *mut *mut int (probe)(struct intel_th_device,
    pub thdev): *mut *mut void (remove)(struct intel_th_device,
// switch (GTH) ops
    pub othdev): *mut intel_th_device,
    pub othdev): *mut intel_th_device,
    pub output): *mut intel_th_output,
    pub output): *mut intel_th_output,
    pub output): *mut intel_th_output,
    pub output): *mut intel_th_output,
// output ops
    pub thdev): *mut *mut irqreturn_t (irq)(struct intel_th_device,
    pub thdev): *mut *mut void (wait_empty)(struct intel_th_device,
    pub thdev): *mut *mut int (activate)(struct intel_th_device,
    pub thdev): *mut *mut void (deactivate)(struct intel_th_device,
// file_operations for those who want a device node
    pub fops: *const file_operations,
// optional attributes
    pub attr_group: *const attribute_group,
// source ops
    pub master): c_uint,
}

//
// Subdevice tree structure is as follows:
// + struct intel_th device (pci; dev_{get,set}_drvdata()
// + struct intel_th_device INTEL_TH_SWITCH (GTH)
// + struct intel_th_device INTEL_TH_OUTPUT (MSU, PTI)
// + struct intel_th_device INTEL_TH_SOURCE (STH)
//
// In other words, INTEL_TH_OUTPUT devices are children of INTEL_TH_SWITCH;
// INTEL_TH_SWITCH and INTEL_TH_SOURCE are children of the intel_th device.
//
extern "C" {
    pub fn to_intel_th_device(_arg: parent) -> return;
}
extern "C" {
    pub fn dev_get_drvdata(_arg: thdev->dev.parent) -> return;
}
extern "C" {
    pub fn intel_th_free(th: *mut intel_th);
}
extern "C" {
    pub fn intel_th_driver_register(thdrv: *mut intel_th_driver) -> c_int;
}
extern "C" {
    pub fn intel_th_driver_unregister(thdrv: *mut intel_th_driver);
}
extern "C" {
    pub fn intel_th_trace_enable(thdev: *mut intel_th_device) -> c_int;
}
extern "C" {
    pub fn intel_th_trace_switch(thdev: *mut intel_th_device) -> c_int;
}
extern "C" {
    pub fn intel_th_trace_disable(thdev: *mut intel_th_device) -> c_int;
}
extern "C" {
    pub fn intel_th_output_enable(th: *mut intel_th, otype: c_uint) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum th_mmio_idx {
    TH_MMIO_CONFIG = 0,
    TH_MMIO_SW = 1,
    TH_MMIO_RTIT = 2,
    TH_MMIO_END,
}

pub const TH_POSSIBLE_OUTPUTS: c_int = 8;
// Total number of possible subdevices: outputs + GTH + STH

pub const TH_CONFIGURABLE_MASTERS: c_int = 256;
pub const TH_MSC_MAX: c_int = 2;
// Maximum IRQ vectors
pub const TH_NVEC_MAX: c_int = 8;
//
// struct intel_th - Intel TH controller
// @dev:	driver core's device
// @thdev:	subdevices
// @hub:	"switch" subdevice (GTH)
// @resource:	resources of the entire controller
// @num_thdevs:	number of devices in the @thdev array
// @num_resources:	number of resources in the @resource array
// @irq:	irq number
// @num_irqs:	number of IRQs is use
// @id:		this Intel TH controller's device ID in the system
// @major:	device node major for output devices
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_th {
    pub dev: *mut device,
    pub thdev: [*mut intel_th_device; TH_SUBDEVICE_MAX],
    pub hub: *mut intel_th_device,
    pub drvdata: *const intel_th_drvdata,
    pub resource: [resource; TH_MMIO_END],
    pub ): *mut *mut int (activate)(struct intel_th,
    pub ): *mut *mut void (deactivate)(struct intel_th,
    pub num_thdevs: c_uint,
    pub num_resources: c_uint,
    pub irq: c_int,
    pub num_irqs: c_int,
    pub id: c_int,
    pub major: c_int,

    pub request_module_work: work_struct,

    pub dbg: *mut dentry,

}

extern "C" {
    pub fn to_intel_th_parent(_arg: thdev) -> return;
}
//
// Register windows
//
// Global Trace Hub (GTH)
// Timestamp counter unit (TSCU)
// Software Trace Hub (STH) [0x4000..0x4fff]
// Memory Storage Unit (MSU) [0xa0000..0xa1fff]
// Internal MSU trace buffer [0x80000..0x9ffff]
// PTI output == same window as GTH
// DCI Handler (DCIH) == some window as MSU
//
// Scratchpad bits: tell firmware and external debuggers
// what we are up to.
//
// Memory is the primary destination
// XHCI DbC is the primary destination
// PTI is the primary destination
// BSSB is the primary destination
// PTI is the alternate destination
// BSSB is the alternate destination
// DeepSx exit occurred
// S4 exit occurred
// S5 exit occurred
// MSU controller 0/1 is enabled
// Sx exit occurred
// Trigger Unit is enabled
// External debugger is using Intel TH
