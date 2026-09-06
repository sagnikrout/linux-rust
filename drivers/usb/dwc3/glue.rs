//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/dwc3/glue.h
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
// glue.h - DesignWare USB3 DRD glue header
//

//
// dwc3_properties: DWC3 core properties
// @gsbuscfg0_reqinfo: Value to be programmed in the GSBUSCFG0.REQINFO field
// @needs_full_reinit: indicate the controller may not remain power during system
// pm and need full initialization
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc3_properties {
    pub gsbuscfg0_reqinfo: u32,
    pub needs_full_reinit:1: unsigned,
}

//
// dwc3_probe_data: Initialization parameters passed to dwc3_core_probe()
// @dwc: Reference to dwc3 context structure
// @res: resource for the DWC3 core mmio region
// @ignore_clocks_and_resets: clocks and resets defined for the device should
// be ignored by the DWC3 core, as they are managed by the glue
// @skip_core_init_mode: Skip the finial initialization of the target mode, as
// it must be managed by the glue
// @properties: dwc3 software manage properties
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc3_probe_data {
    pub dwc: *mut dwc3,
    pub res: *mut resource,
    pub ignore_clocks_and_resets: bool,
    pub skip_core_init_mode: bool,
    pub properties: dwc3_properties,
}

//
// dwc3_core_probe - Initialize the core dwc3 driver
// @data: Initialization and configuration parameters for the controller
//
// Initializes the DesignWare USB3 core driver by setting up resources,
// registering interrupts, performing hardware setup, and preparing
// the controller for operation in the appropriate mode (host, gadget,
// or OTG). This is the main initialization function called by glue
// layer drivers to set up the core controller.
//
// Return: 0 on success, negative error code on failure
//
extern "C" {
    pub fn dwc3_core_probe(data: *const dwc3_probe_data) -> c_int;
}
//
// dwc3_core_remove - Deinitialize and remove the core dwc3 driver
// @dwc: Pointer to DWC3 controller context
//
// Cleans up resources and disables the dwc3 core driver. This should be called
// during driver removal or when the glue layer needs to shut down the
// controller completely.
//
extern "C" {
    pub fn dwc3_core_remove(dwc: *mut dwc3);
}
//
// The following callbacks are provided for glue drivers to call from their
// own pm callbacks provided in struct dev_pm_ops. Glue drivers can perform
// platform-specific work before or after calling these functions and delegate
// the core suspend/resume operations to the core driver.
//
extern "C" {
    pub fn dwc3_runtime_suspend(dwc: *mut dwc3) -> c_int;
}
extern "C" {
    pub fn dwc3_runtime_resume(dwc: *mut dwc3) -> c_int;
}
extern "C" {
    pub fn dwc3_runtime_idle(dwc: *mut dwc3) -> c_int;
}
extern "C" {
    pub fn dwc3_pm_suspend(dwc: *mut dwc3) -> c_int;
}
extern "C" {
    pub fn dwc3_pm_resume(dwc: *mut dwc3) -> c_int;
}
extern "C" {
    pub fn dwc3_pm_complete(dwc: *mut dwc3);
}
extern "C" {
    pub fn dwc3_pm_prepare(dwc: *mut dwc3) -> c_int;
}
// All of the following functions must only be used with skip_core_init_mode
//
// dwc3_core_init - Initialize DWC3 core hardware
// @dwc: Pointer to DWC3 controller context
//
// Configures and initializes the core hardware, usually done by dwc3_core_probe.
// This function is provided for platforms that use skip_core_init_mode and need
// to finalize the core initialization after some platform-specific setup.
// It must only be called when using skip_core_init_mode and before
// dwc3_host_init or dwc3_gadget_init.
//
// Return: 0 on success, negative error code on failure
//
extern "C" {
    pub fn dwc3_core_init(dwc: *mut dwc3) -> c_int;
}
//
// dwc3_core_exit - Shut down DWC3 core hardware
// @dwc: Pointer to DWC3 controller context
//
// Disables and cleans up the core hardware state. This is usually handled
// internally by dwc3 and must only be called when using skip_core_init_mode
// and only after dwc3_core_init. Afterwards, dwc3_core_init may be called
// again.
//
extern "C" {
    pub fn dwc3_core_exit(dwc: *mut dwc3);
}
//
// dwc3_host_init - Initialize host mode operation
// @dwc: Pointer to DWC3 controller context
//
// Initializes the controller for USB host mode operation, usually done by
// dwc3_core_probe or from within the dwc3 USB role switch callback.
// This function is provided for platforms that use skip_core_init_mode and need
// to finalize the host initialization after some platform-specific setup.
// It must not be called before dwc3_core_init or when skip_core_init_mode is
// not used. It must also not be called when gadget or host mode has already
// been initialized.
//
// Return: 0 on success, negative error code on failure
//
extern "C" {
    pub fn dwc3_host_init(dwc: *mut dwc3) -> c_int;
}
//
// dwc3_host_exit - Shut down host mode operation
// @dwc: Pointer to DWC3 controller context
//
// Disables and cleans up host mode resources, usually done by
// the dwc3 USB role switch callback before switching controller mode.
// It must only be called when skip_core_init_mode is used and only after
// dwc3_host_init.
//
extern "C" {
    pub fn dwc3_host_exit(dwc: *mut dwc3);
}
//
// dwc3_gadget_init - Initialize gadget mode operation
// @dwc: Pointer to DWC3 controller context
//
// Initializes the controller for USB gadget mode operation, usually done by
// dwc3_core_probe or from within the dwc3 USB role switch callback. This
// function is provided for platforms that use skip_core_init_mode and need to
// finalize the gadget initialization after some platform-specific setup.
// It must not be called before dwc3_core_init or when skip_core_init_mode is
// not used. It must also not be called when gadget or host mode has already
// been initialized.
//
// Return: 0 on success, negative error code on failure
//
extern "C" {
    pub fn dwc3_gadget_init(dwc: *mut dwc3) -> c_int;
}
//
// dwc3_gadget_exit - Shut down gadget mode operation
// @dwc: Pointer to DWC3 controller context
//
// Disables and cleans up gadget mode resources, usually done by
// the dwc3 USB role switch callback before switching controller mode.
// It must only be called when skip_core_init_mode is used and only after
// dwc3_gadget_init.
//
extern "C" {
    pub fn dwc3_gadget_exit(dwc: *mut dwc3);
}
//
// dwc3_enable_susphy - Control SUSPHY status for all USB ports
// @dwc: Pointer to DWC3 controller context
// @enable: True to enable SUSPHY, false to disable
//
// Enables or disables the USB3 PHY SUSPEND and USB2 PHY SUSPHY feature for
// all available ports.
// This is usually handled by the dwc3 core code and should only be used
// when skip_core_init_mode is used and the glue layer needs to manage SUSPHY
// settings itself, e.g., due to platform-specific requirements during mode
// switches.
//
extern "C" {
    pub fn dwc3_enable_susphy(dwc: *mut dwc3, enable: bool);
}
//
// dwc3_set_prtcap - Set the USB controller PRTCAP mode
// @dwc: Pointer to DWC3 controller context
// @mode: Target mode, must be one of DWC3_GCTL_PRTCAP_{HOST,DEVICE,OTG}
// @ignore_susphy: If true, skip disabling the SUSPHY and keep the current state
//
// Updates PRTCAP of the controller and current_dr_role inside the dwc3
// structure. For DRD controllers, this also disables SUSPHY unless explicitly
// told to skip via the ignore_susphy parameter.
//
// This is usually handled by the dwc3 core code and should only be used
// when skip_core_init_mode is used and the glue layer needs to manage mode
// transitions itself due to platform-specific requirements. It must be called
// with the correct mode before calling dwc3_host_init or dwc3_gadget_init.
//
extern "C" {
    pub fn dwc3_set_prtcap(dwc: *mut dwc3, mode: u32, ignore_susphy: bool);
}
