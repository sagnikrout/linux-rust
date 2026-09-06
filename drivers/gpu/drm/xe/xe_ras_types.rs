//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_ras_types.h
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
// Copyright © 2026 Intel Corporation
//

pub const XE_RAS_NUM_COUNTERS: c_int = 16;
pub const XE_RAS_NUM_ERROR_ARR: c_int = 3;
// Error bits in IEH global error status register

// Device memory error categories

//
// enum xe_ras_recovery_action - RAS recovery actions
//
// @XE_RAS_RECOVERY_ACTION_RECOVERED: Error recovered
// @XE_RAS_RECOVERY_ACTION_RESET: Requires reset
// @XE_RAS_RECOVERY_ACTION_DISCONNECT: Requires disconnect
// @XE_RAS_RECOVERY_ACTION_MAX: Max action value
//
// This enum defines the possible recovery actions that can be taken in response
// to RAS errors.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_ras_recovery_action {
    XE_RAS_RECOVERY_ACTION_RECOVERED = 0,
    XE_RAS_RECOVERY_ACTION_RESET,
    XE_RAS_RECOVERY_ACTION_DISCONNECT,
    XE_RAS_RECOVERY_ACTION_MAX
}

//
// struct xe_ras_error_common - Error fields that are common across all products
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_ras_error_common {
// @severity: Error severity
    pub severity: u8,
// @component: IP block where error originated
    pub component: u8,
    pub __packed: },
//
// struct xe_ras_error_unit - Error unit information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_ras_error_unit {
// @tile: Tile identifier
    pub tile: u8,
// @instance: Instance identifier specific to IP
    pub instance: u32,
    pub __packed: },
//
// struct xe_ras_error_cause - Error cause information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_ras_error_cause {
// @cause: Cause/checker
    pub cause: u32,
// @reserved: For future use
    pub reserved: u8,
    pub __packed: },
//
// struct xe_ras_error_product - Error fields that are specific to the product
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_ras_error_product {
// @unit: Unit within IP block
    pub unit: xe_ras_error_unit,
// @cause: Cause/checker
    pub cause: xe_ras_error_cause,
    pub __packed: },
//
// struct xe_ras_error_class - Combines common and product-specific parts
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_ras_error_class {
// @common: Common error type and component
    pub common: xe_ras_error_common,
// @product: Product-specific unit and cause
    pub product: xe_ras_error_product,
    pub __packed: },
//
// struct xe_ras_threshold_crossed - Data for threshold crossed event
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_ras_threshold_crossed {
// @ncounters: Number of error counters that crossed thresholds
    pub ncounters: u32,
// @counters: Array of error counters that crossed threshold
    pub counters: [xe_ras_error_class; XE_RAS_NUM_COUNTERS],
    pub __packed: },
//
// struct xe_ras_get_counter_request - Request structure for get counter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_ras_get_counter_request {
// @counter: Error counter to be queried
    pub counter: xe_ras_error_class,
// @reserved: Reserved for future use
    pub reserved: u32,
    pub __packed: },
//
// struct xe_ras_get_counter_response - Response structure for get counter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_ras_get_counter_response {
// @counter: Error counter that was queried
    pub counter: xe_ras_error_class,
// @value: Current counter value
    pub value: u32,
// @timestamp: Timestamp when counter was last updated
    pub timestamp: u64,
// @threshold: Threshold value for the counter
    pub threshold: u32,
// @reserved: Reserved
    pub reserved: [u32; 57],
    pub __packed: },
//
// struct xe_ras_clear_counter_request - Request structure for clear counter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_ras_clear_counter_request {
// @counter: Counter class to be cleared
    pub counter: xe_ras_error_class,
// @reserved: Reserved for future use
    pub reserved: u32,
    pub __packed: },
//
// struct xe_ras_clear_counter_response - Response structure for clear counter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_ras_clear_counter_response {
// @counter: Counter class that was cleared
    pub counter: xe_ras_error_class,
// @reserved: Reserved
    pub reserved: u32,
// @timestamp: Timestamp when the counter was cleared
    pub timestamp: u64,
// @status: Status of the clear operation
    pub status: u32,
// @reserved1: Reserved for future use
    pub reserved1: [u32; 3],
    pub __packed: },
//
// struct xe_ras_error_array - Details of the error types
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_ras_error_array {
// @value: Counter value of the detailed error
    pub value: u32,
// @counter: Error counter
    pub counter: xe_ras_error_class,
// @timestamp: Timestamp
    pub timestamp: u64,
// @details: Error details specific to the counter
    pub details: [u32; XE_RAS_NUM_COUNTERS],
    pub __packed: },
//
// struct xe_ras_get_soc_error - Response from get soc error command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_ras_get_soc_error {
// @num_errors: Number of errors reported in this response
    pub num_errors: u8,
// @additional_errors: Indicates if the errors are pending
    pub additional_errors: u8,
// @arr: Array of up to 3 errors
    pub arr: [xe_ras_error_array; XE_RAS_NUM_ERROR_ARR],
    pub __packed: },
//
// struct xe_ras_compute_error - Error details of Core Compute error
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_ras_compute_error {
// @log_header: Error Source and type
    pub log_header: u32,
// @reserved: Reserved
    pub reserved: [u32; 15],
    pub __packed: },
//
// struct xe_ras_soc_error_source - Source of SoC error
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_ras_soc_error_source {
// @csc: CSC
    pub csc:1: u32,
// @ieh: IEH (Integrated Error Handler)
    pub ieh:1: u32,
// @reserved: Reserved for future use
    pub reserved:30: u32,
    pub __packed: },
//
// struct xe_ras_soc_error - Error details of SoC internal error
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_ras_soc_error {
// @source: Error source
    pub source: xe_ras_soc_error_source,
// @details: Error details specific to the error source
    pub details: [u32; 15],
    pub __packed: },
//
// struct xe_ras_csc_error - CSC error details
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_ras_csc_error {
// @reserved: Reserved for future use
    pub reserved: u32,
// @hec_fw_error: CSC firmware error
    pub hec_fw_error: u32,
    pub __packed: },
//
// struct xe_ras_ieh_error - IEH (Integrated Error Handler) error details
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_ras_ieh_error {
// @reserved: Reserved for future use
    pub reserved: u32,
// @global_error_status: Global error status
    pub global_error_status: u32,
// @reserved1: Reserved for future use
    pub reserved1: [u32; 2],
// @info: Additional information
    pub info: [u32; 10],
    pub __packed: },
//
// struct xe_ras_memory_error - Device memory error details
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_ras_memory_error {
// @category: Device memory error category
    pub category: u8,
// @reserved: Reserved for future use
    pub reserved: [u8; 7],
// @reserved1: Reserved for future use
    pub reserved1: u64,
// @sw_address: Software address where error occurred
    pub sw_address: u64,
// @reserved2: Reserved for future use
    pub reserved2: [u32; 10],
    pub __packed: },
//
// struct xe_ras_get_health_request - Request structure for obtaining gpu health
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_ras_get_health_request {
// @reserved: Reserved for future use.
    pub reserved: [u32; 2],
    pub __packed: },
//
// struct xe_ras_get_health_response - Response structure for obtaining gpu health
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_ras_get_health_response {
// @health: gpu health value
    pub health: u8,
// @reserved: Reserved for future use
    pub reserved: [u8; 3],
    pub __packed: },
//
// struct xe_ras_set_health_request - Request structure for setting gpu health
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_ras_set_health_request {
// @health: gpu health value
    pub health: u8,
// @reserved: Reserved for future use
    pub reserved: [u8; 3],
    pub __packed: },
//
// struct xe_ras_set_health_response - Response structure for setting gpu health
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_ras_set_health_response {
// @status: Status of set health operation
    pub status: u32,
// @health: Resulting gpu health value
    pub health: u8,
// @reserved: Reserved for future use
    pub reserved: [u8; 3],
// @reserved1: Reserved for future use
    pub reserved1: [u32; 2],
    pub __packed: },
