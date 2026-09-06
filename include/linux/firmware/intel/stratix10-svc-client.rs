//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/firmware/intel/stratix10-svc-client.h
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
// Copyright (C) 2017-2018, Intel Corporation
// Copyright (C) 2025, Altera Corporation
//
// Service layer driver supports client names
//
// fpga: for FPGA configuration
// rsu: for remote status update
// hwmon: for hardware monitoring (voltage and temperature)
//

//
// Status of the sent command, in bit number
//
// SVC_STATUS_OK:
// Secure firmware accepts the request issued by one of service clients.
//
// SVC_STATUS_BUFFER_SUBMITTED:
// Service client successfully submits data buffer to secure firmware.
//
// SVC_STATUS_BUFFER_DONE:
// Secure firmware completes data process, ready to accept the
// next WRITE transaction.
//
// SVC_STATUS_COMPLETED:
// Secure firmware completes service request successfully. In case of
// FPGA configuration, FPGA should be in user mode.
//
// SVC_COMMAND_STATUS_BUSY:
// Service request is still in process.
//
// SVC_COMMAND_STATUS_ERROR:
// Error encountered during the process of the service request.
//
// SVC_STATUS_NO_SUPPORT:
// Secure firmware doesn't support requested features such as RSU retry
// or RSU notify.
//
pub const SVC_STATUS_OK: c_int = 0;
pub const SVC_STATUS_BUFFER_SUBMITTED: c_int = 1;
pub const SVC_STATUS_BUFFER_DONE: c_int = 2;
pub const SVC_STATUS_COMPLETED: c_int = 3;
pub const SVC_STATUS_BUSY: c_int = 4;
pub const SVC_STATUS_ERROR: c_int = 5;
pub const SVC_STATUS_NO_SUPPORT: c_int = 6;
pub const SVC_STATUS_INVALID_PARAM: c_int = 7;
//
// Flag bit for COMMAND_RECONFIG
//
// COMMAND_RECONFIG_FLAG_PARTIAL:
// Set to FPGA configuration type (full or partial).
//
pub const COMMAND_RECONFIG_FLAG_PARTIAL: c_int = 0;
//
// Timeout settings for service clients:
// timeout value used in Stratix10 FPGA manager driver.
// timeout value used in RSU driver
//
pub const SVC_RECONFIG_REQUEST_TIMEOUT_MS: c_int = 5000;
pub const SVC_RECONFIG_BUFFER_TIMEOUT_MS: c_int = 5000;
pub const SVC_RSU_REQUEST_TIMEOUT_MS: c_int = 2000;
pub const SVC_FCS_REQUEST_TIMEOUT_MS: c_int = 2000;
pub const SVC_COMPLETED_TIMEOUT_MS: c_int = 30000;
pub const SVC_HWMON_REQUEST_TIMEOUT_MS: c_int = 2000;
//
// enum stratix10_svc_command_code - supported service commands
//
// @COMMAND_NOOP: do 'dummy' request for integration/debug/trouble-shooting
//
// @COMMAND_RECONFIG: ask for FPGA configuration preparation, return status
// is SVC_STATUS_OK
//
// @COMMAND_RECONFIG_DATA_SUBMIT: submit buffer(s) of bit-stream data for the
// FPGA configuration, return status is SVC_STATUS_SUBMITTED or SVC_STATUS_ERROR
//
// @COMMAND_RECONFIG_DATA_CLAIM: check the status of the configuration, return
// status is SVC_STATUS_COMPLETED, or SVC_STATUS_BUSY, or SVC_STATUS_ERROR
//
// @COMMAND_RECONFIG_STATUS: check the status of the configuration, return
// status is SVC_STATUS_COMPLETED, or SVC_STATUS_BUSY, or SVC_STATUS_ERROR
//
// @COMMAND_RSU_STATUS: request remote system update boot log, return status
// is log data or SVC_STATUS_RSU_ERROR
//
// @COMMAND_RSU_UPDATE: set the offset of the bitstream to boot after reboot,
// return status is SVC_STATUS_OK or SVC_STATUS_ERROR
//
// @COMMAND_RSU_NOTIFY: report the status of hard processor system
// software to firmware, return status is SVC_STATUS_OK or
// SVC_STATUS_ERROR
//
// @COMMAND_RSU_RETRY: query firmware for the current image's retry counter,
// return status is SVC_STATUS_OK or SVC_STATUS_ERROR
//
// @COMMAND_RSU_MAX_RETRY: query firmware for the max retry value,
// return status is SVC_STATUS_OK or SVC_STATUS_ERROR
//
// @COMMAND_RSU_DCMF_VERSION: query firmware for the DCMF version, return status
// is SVC_STATUS_OK or SVC_STATUS_ERROR
//
// @COMMAND_POLL_SERVICE_STATUS: poll if the service request is complete,
// return statis is SVC_STATUS_OK, SVC_STATUS_ERROR or SVC_STATUS_BUSY
//
// @COMMAND_FIRMWARE_VERSION: query running firmware version, return status
// is SVC_STATUS_OK or SVC_STATUS_ERROR
//
// @COMMAND_SMC_SVC_VERSION: Non-mailbox SMC SVC API Version,
// return status is SVC_STATUS_OK
//
// @COMMAND_MBOX_SEND_CMD: send generic mailbox command, return status is
// SVC_STATUS_OK or SVC_STATUS_ERROR
//
// @COMMAND_RSU_DCMF_STATUS: query firmware for the DCMF status
// return status is SVC_STATUS_OK or SVC_STATUS_ERROR
//
// @COMMAND_RSU_GET_DEVICE_INFO: query firmware for QSPI device info;
// return status is SVC_STATUS_OK, SVC_STATUS_ERROR, or SVC_STATUS_NO_SUPPORT
// (unsupported command / firmware compatibility path in the service layer).
//
// @COMMAND_RSU_GET_SPT_TABLE: query firmware for SPT table
// return status is SVC_STATUS_OK or SVC_STATUS_ERROR
//
// @COMMAND_FCS_REQUEST_SERVICE: request validation of image from firmware,
// return status is SVC_STATUS_OK, SVC_STATUS_INVALID_PARAM
//
// @COMMAND_FCS_SEND_CERTIFICATE: send a certificate, return status is
// SVC_STATUS_OK, SVC_STATUS_INVALID_PARAM, SVC_STATUS_ERROR
//
// @COMMAND_FCS_GET_PROVISION_DATA: read the provisioning data, return status is
// SVC_STATUS_OK, SVC_STATUS_INVALID_PARAM, SVC_STATUS_ERROR
//
// @COMMAND_FCS_DATA_ENCRYPTION: encrypt the data, return status is
// SVC_STATUS_OK, SVC_STATUS_INVALID_PARAM, SVC_STATUS_ERROR
//
// @COMMAND_FCS_DATA_DECRYPTION: decrypt the data, return status is
// SVC_STATUS_OK, SVC_STATUS_INVALID_PARAM, SVC_STATUS_ERROR
//
// @COMMAND_FCS_RANDOM_NUMBER_GEN: generate a random number, return status
// is SVC_STATUS_OK, SVC_STATUS_ERROR
//
// @COMMAND_HWMON_READTEMP: query the temperature from the hardware monitor,
// return status is SVC_STATUS_OK or SVC_STATUS_ERROR
//
// @COMMAND_HWMON_READVOLT: query the voltage from the hardware monitor,
// return status is SVC_STATUS_OK or SVC_STATUS_ERROR
//
// @COMMAND_SMC_ATF_BUILD_VER: Non-mailbox SMC ATF Build Version,
// return status is SVC_STATUS_OK
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stratix10_svc_command_code {
// for FPGA
    COMMAND_NOOP = 0,
    COMMAND_RECONFIG,
    COMMAND_RECONFIG_DATA_SUBMIT,
    COMMAND_RECONFIG_DATA_CLAIM,
    COMMAND_RECONFIG_STATUS,
// for RSU
    COMMAND_RSU_STATUS = 10,
    COMMAND_RSU_UPDATE,
    COMMAND_RSU_NOTIFY,
    COMMAND_RSU_RETRY,
    COMMAND_RSU_MAX_RETRY,
    COMMAND_RSU_DCMF_VERSION,
    COMMAND_RSU_DCMF_STATUS,
    COMMAND_RSU_GET_DEVICE_INFO,
    COMMAND_FIRMWARE_VERSION,
    COMMAND_RSU_GET_SPT_TABLE,
// for FCS
    COMMAND_FCS_REQUEST_SERVICE = 20,
    COMMAND_FCS_SEND_CERTIFICATE,
    COMMAND_FCS_GET_PROVISION_DATA,
    COMMAND_FCS_DATA_ENCRYPTION,
    COMMAND_FCS_DATA_DECRYPTION,
    COMMAND_FCS_RANDOM_NUMBER_GEN,
// for general status poll
    COMMAND_POLL_SERVICE_STATUS = 40,
// for generic mailbox send command
    COMMAND_MBOX_SEND_CMD = 100,
// Non-mailbox SMC Call
    COMMAND_SMC_SVC_VERSION = 200,
// for HWMON
    COMMAND_HWMON_READTEMP,
    COMMAND_HWMON_READVOLT,
    COMMAND_SMC_ATF_BUILD_VER
}

//
// struct stratix10_svc_client_msg - message sent by client to service
// @payload: starting address of data need be processed
// @payload_length: to be processed data size in bytes
// @payload_output: starting address of processed data
// @payload_length_output: processed data size in bytes
// @command: service command
// @arg: args to be passed via registers and not physically mapped buffers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stratix10_svc_client_msg {
    pub payload: *mut c_void,
    pub payload_length: usize,
    pub payload_output: *mut c_void,
    pub payload_length_output: usize,
    pub command: stratix10_svc_command_code,
    pub arg: [u64; 3],
}

//
// struct stratix10_svc_command_config_type - config type
// @flags: flag bit for the type of FPGA configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stratix10_svc_command_config_type {
    pub flags: u32,
}

//
// struct stratix10_svc_cb_data - callback data structure from service layer
// @status: the status of sent command
// @kaddr1: address of 1st completed data block, or command-specific payload.
// For COMMAND_RSU_GET_DEVICE_INFO on SVC_STATUS_OK or SVC_STATUS_ERROR,
// points to struct arm_smccc_1_2_regs filled by the SMC/HVC return
// registers (a0 status, a1-a4 packed device words per
// INTEL_SIP_SMC_RSU_GET_DEVICE_INFO).  On SVC_STATUS_NO_SUPPORT (older
// firmware that does not handle this command), kaddr1 is NULL.
// @kaddr2: address of 2nd completed data block
// @kaddr3: address of 3rd completed data block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stratix10_svc_cb_data {
    pub status: u32,
    pub kaddr1: *mut c_void,
    pub kaddr2: *mut c_void,
    pub kaddr3: *mut c_void,
}

//
// struct stratix10_svc_client - service client structure
// @dev: the client device
// @receive_cb: callback to provide service client the received data
// @priv: client private data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stratix10_svc_client {
    pub dev: *mut device,
    pub cb_data): *mut stratix10_svc_cb_data,
    pub priv: *mut c_void,
}

//
// stratix10_svc_request_channel_byname() - request service channel
// @client: identity of the client requesting the channel
// @name: supporting client name defined above
//
// Return: a pointer to channel assigned to the client on success,
// or ERR_PTR() on error.
//
// stratix10_svc_request_channel_byname(struct stratix10_svc_client *client,
//
// stratix10_svc_free_channel() - free service channel.
// @chan: service channel to be freed
//
extern "C" {
    pub fn stratix10_svc_free_channel(chan: *mut stratix10_svc_chan);
}
//
// stratix10_svc_allocate_memory() - allocate the momory
// @chan: service channel assigned to the client
// @size: number of bytes client requests
//
// Service layer allocates the requested number of bytes from the memory
// pool for the client.
//
// Return: the starting address of allocated memory on success, or
// ERR_PTR() on error.
//
// stratix10_svc_free_memory() - free allocated memory
// @chan: service channel assigned to the client
// @kaddr: starting address of memory to be free back to pool
//
extern "C" {
    pub fn stratix10_svc_free_memory(chan: *mut stratix10_svc_chan, kaddr: *mut c_void);
}
//
// stratix10_svc_send() - send a message to the remote
// @chan: service channel assigned to the client
// @msg: message data to be sent, in the format of
// struct stratix10_svc_client_msg
//
// Return: 0 for success, -ENOMEM or -ENOBUFS on error.
//
extern "C" {
    pub fn stratix10_svc_send(chan: *mut stratix10_svc_chan, msg: *mut c_void) -> c_int;
}
//
// stratix10_svc_done() - complete service request
// @chan: service channel assigned to the client
//
// This function is used by service client to inform service layer that
// client's service requests are completed, or there is an error in the
// request process.
//
extern "C" {
    pub fn stratix10_svc_done(chan: *mut stratix10_svc_chan);
}
//
// typedef async_callback_t - A type definition for an asynchronous callback function.
//
// This type defines a function pointer for an asynchronous callback.
// The callback function takes a single argument, which is a pointer to
// user-defined data.
//
// @cb_arg: Argument to be passed to the callback function.
//
extern "C" {
    pub fn void(cb_arg: *mut *mut async_callback_t)(void) -> typedef;
}
//
// stratix10_svc_add_async_client - Add an asynchronous client to a Stratix 10
// service channel.
// @chan: Pointer to the Stratix 10 service channel structure.
// @use_unique_clientid: Boolean flag indicating whether to use a unique client ID.
//
// This function registers an asynchronous client with the specified Stratix 10
// service channel. If the use_unique_clientid flag is set to true, a unique client
// ID will be assigned to the client.
//
// Return: 0 on success, or a negative error code on failure:
// -EINVAL if the channel is NULL or the async controller is not initialized.
// -EALREADY if the async channel is already allocated.
// -ENOMEM if memory allocation fails.
// Other negative values if ID allocation fails
//
extern "C" {
    pub fn stratix10_svc_add_async_client(chan: *mut stratix10_svc_chan, use_unique_clientid: bool) -> c_int;
}
//
// stratix10_svc_remove_async_client - Remove an asynchronous client from the Stratix 10
// service channel.
// @chan: Pointer to the Stratix 10 service channel structure.
//
// This function removes an asynchronous client from the specified Stratix 10 service channel.
// It is typically used to clean up and release resources associated with the client.
//
// Return: 0 on success, -EINVAL if the channel or asynchronous channel is invalid.
//
extern "C" {
    pub fn stratix10_svc_remove_async_client(chan: *mut stratix10_svc_chan) -> c_int;
}
//
// stratix10_svc_async_send - Send an asynchronous message to the SDM mailbox
// in EL3 secure firmware.
// @chan: Pointer to the service channel structure.
// @msg: Pointer to the message to be sent.
// @handler: Pointer to the handler object used by caller to track the transaction.
// @cb: Callback function to be called upon completion.
// @cb_arg: Argument to be passed to the callback function.
//
// This function sends a message asynchronously to the SDM mailbox in EL3 secure firmware.
// and registers a callback function to be invoked when the operation completes.
//
// Return: 0 on success,and negative error codes on failure.
//
// stratix10_svc_async_poll - Polls the status of an asynchronous service request.
// @chan: Pointer to the service channel structure.
// @tx_handle: Handle to the transaction being polled.
// @data: Pointer to the callback data structure to be filled with the result.
//
// This function checks the status of an asynchronous service request
// and fills the provided callback data structure with the result.
//
// Return: 0 on success, -EINVAL if any input parameter is invalid or if the
// async controller is not initialized, -EAGAIN if the transaction is
// still in progress, or other negative error codes on failure.
//
// stratix10_svc_async_done - Complete an asynchronous transaction
// @chan: Pointer to the service channel structure
// @tx_handle: Pointer to the transaction handle
//
// This function completes an asynchronous transaction by removing the
// transaction from the hash table and deallocating the associated resources.
//
// Return: 0 on success, -EINVAL on invalid input or errors.
//
extern "C" {
    pub fn stratix10_svc_async_done(chan: *mut stratix10_svc_chan, tx_handle: *mut c_void) -> c_int;
}
