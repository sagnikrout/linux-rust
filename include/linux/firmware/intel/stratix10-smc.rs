//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/firmware/intel/stratix10-smc.h
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

//
// This file defines the Secure Monitor Call (SMC) message protocol used for
// service layer driver in normal world (EL1) to communicate with secure
// monitor software in Secure Monitor Exception Level 3 (EL3).
//
// This file is shared with secure firmware (FW) which is out of kernel tree.
//
// An ARM SMC instruction takes a function identifier and up to 6 64-bit
// register values as arguments, and can return up to 4 64-bit register
// value. The operation of the secure monitor is determined by the parameter
// values passed in through registers.
//
// EL1 and EL3 communicates pointer as physical address rather than the
// virtual address.
//
// Functions specified by ARM SMC Calling convention:
//
// FAST call executes atomic operations, returns when the requested operation
// has completed.
// STD call starts a operation which can be preempted by a non-secure
// interrupt. The call can return before the requested operation has
// completed.
//
// a0..a7 is used as register names in the descriptions below, on arm32
// that translates to r0..r7 and on arm64 to w0..w7.
//
// @func_num: function ID
//

//
// Return values in INTEL_SIP_SMC_* call
//
// INTEL_SIP_SMC_RETURN_UNKNOWN_FUNCTION:
// Secure monitor software doesn't recognize the request.
//
// INTEL_SIP_SMC_STATUS_OK:
// Secure monitor software accepts the service client's request.
//
// INTEL_SIP_SMC_STATUS_BUSY:
// Secure monitor software is still processing service client's request.
//
// INTEL_SIP_SMC_STATUS_REJECTED:
// Secure monitor software reject the service client's request.
//
// INTEL_SIP_SMC_STATUS_NO_RESPONSE:
// Secure monitor software has no response for the request yet.
//
// INTEL_SIP_SMC_STATUS_ERROR:
// There is error during the process of service request.
//
// INTEL_SIP_SMC_RSU_ERROR:
// There is error during the process of remote status update request.
//
pub const INTEL_SIP_SMC_RETURN_UNKNOWN_FUNCTION: c_uint = 0xFFFFFFFF;
pub const INTEL_SIP_SMC_STATUS_OK: c_uint = 0x0;
pub const INTEL_SIP_SMC_STATUS_BUSY: c_uint = 0x1;
pub const INTEL_SIP_SMC_STATUS_REJECTED: c_uint = 0x2;
pub const INTEL_SIP_SMC_STATUS_NO_RESPONSE: c_uint = 0x3;
pub const INTEL_SIP_SMC_STATUS_ERROR: c_uint = 0x4;
pub const INTEL_SIP_SMC_RSU_ERROR: c_uint = 0x7;
//
// Request INTEL_SIP_SMC_FPGA_CONFIG_START
//
// Sync call used by service driver at EL1 to request the FPGA in EL3 to
// be prepare to receive a new configuration.
//
// Call register usage:
// a0: INTEL_SIP_SMC_FPGA_CONFIG_START.
// a1: flag for full or partial configuration. 0 for full and 1 for partial
// configuration.
// a2-7: not used.
//
// Return status:
// a0: INTEL_SIP_SMC_STATUS_OK, or INTEL_SIP_SMC_STATUS_ERROR.
// a1-3: not used.
//
pub const INTEL_SIP_SMC_FUNCID_FPGA_CONFIG_START: c_int = 1;

//
// Request INTEL_SIP_SMC_FPGA_CONFIG_WRITE
//
// Async call used by service driver at EL1 to provide FPGA configuration data
// to secure world.
//
// Call register usage:
// a0: INTEL_SIP_SMC_FPGA_CONFIG_WRITE.
// a1: 64bit physical address of the configuration data memory block
// a2: Size of configuration data block.
// a3-7: not used.
//
// Return status:
// a0: INTEL_SIP_SMC_STATUS_OK, INTEL_SIP_SMC_STATUS_BUSY or
// INTEL_SIP_SMC_STATUS_ERROR.
// a1: 64bit physical address of 1st completed memory block if any completed
// block, otherwise zero value.
// a2: 64bit physical address of 2nd completed memory block if any completed
// block, otherwise zero value.
// a3: 64bit physical address of 3rd completed memory block if any completed
// block, otherwise zero value.
//
pub const INTEL_SIP_SMC_FUNCID_FPGA_CONFIG_WRITE: c_int = 2;

//
// Request INTEL_SIP_SMC_FPGA_CONFIG_COMPLETED_WRITE
//
// Sync call used by service driver at EL1 to track the completed write
// transactions. This request is called after INTEL_SIP_SMC_FPGA_CONFIG_WRITE
// call returns INTEL_SIP_SMC_STATUS_BUSY.
//
// Call register usage:
// a0: INTEL_SIP_SMC_FPGA_CONFIG_COMPLETED_WRITE.
// a1-7: not used.
//
// Return status:
// a0: INTEL_SIP_SMC_STATUS_OK, INTEL_SIP_SMC_FPGA_BUSY or
// INTEL_SIP_SMC_STATUS_ERROR.
// a1: 64bit physical address of 1st completed memory block.
// a2: 64bit physical address of 2nd completed memory block if
// any completed block, otherwise zero value.
// a3: 64bit physical address of 3rd completed memory block if
// any completed block, otherwise zero value.
//
pub const INTEL_SIP_SMC_FUNCID_FPGA_CONFIG_COMPLETED_WRITE: c_int = 3;

//
// Request INTEL_SIP_SMC_FPGA_CONFIG_ISDONE
//
// Sync call used by service driver at EL1 to inform secure world that all
// data are sent, to check whether or not the secure world had completed
// the FPGA configuration process.
//
// Call register usage:
// a0: INTEL_SIP_SMC_FPGA_CONFIG_ISDONE.
// a1-7: not used.
//
// Return status:
// a0: INTEL_SIP_SMC_STATUS_OK, INTEL_SIP_SMC_STATUS_BUSY or
// INTEL_SIP_SMC_STATUS_ERROR.
// a1-3: not used.
//
pub const INTEL_SIP_SMC_FUNCID_FPGA_CONFIG_ISDONE: c_int = 4;

//
// Request INTEL_SIP_SMC_FPGA_CONFIG_GET_MEM
//
// Sync call used by service driver at EL1 to query the physical address of
// memory block reserved by secure monitor software.
//
// Call register usage:
// a0:INTEL_SIP_SMC_FPGA_CONFIG_GET_MEM.
// a1-7: not used.
//
// Return status:
// a0: INTEL_SIP_SMC_STATUS_OK or INTEL_SIP_SMC_STATUS_ERROR.
// a1: start of physical address of reserved memory block.
// a2: size of reserved memory block.
// a3: not used.
//
pub const INTEL_SIP_SMC_FUNCID_FPGA_CONFIG_GET_MEM: c_int = 5;

//
// Request INTEL_SIP_SMC_FPGA_CONFIG_LOOPBACK
//
// For SMC loop-back mode only, used for internal integration, debugging
// or troubleshooting.
//
// Call register usage:
// a0: INTEL_SIP_SMC_FPGA_CONFIG_LOOPBACK.
// a1-7: not used.
//
// Return status:
// a0: INTEL_SIP_SMC_STATUS_OK or INTEL_SIP_SMC_STATUS_ERROR.
// a1-3: not used.
//
pub const INTEL_SIP_SMC_FUNCID_FPGA_CONFIG_LOOPBACK: c_int = 6;

//
// Request INTEL_SIP_SMC_REG_READ
//
// Read a protected register at EL3
//
// Call register usage:
// a0: INTEL_SIP_SMC_REG_READ.
// a1: register address.
// a2-7: not used.
//
// Return status:
// a0: INTEL_SIP_SMC_STATUS_OK or INTEL_SIP_SMC_REG_ERROR.
// a1: value in the register
// a2-3: not used.
//
pub const INTEL_SIP_SMC_FUNCID_REG_READ: c_int = 7;

//
// Request INTEL_SIP_SMC_REG_WRITE
//
// Write a protected register at EL3
//
// Call register usage:
// a0: INTEL_SIP_SMC_REG_WRITE.
// a1: register address
// a2: value to program into register.
// a3-7: not used.
//
// Return status:
// a0: INTEL_SIP_SMC_STATUS_OK or INTEL_SIP_SMC_REG_ERROR.
// a1-3: not used.
//
pub const INTEL_SIP_SMC_FUNCID_REG_WRITE: c_int = 8;

//
// Request INTEL_SIP_SMC_FUNCID_REG_UPDATE
//
// Update one or more bits in a protected register at EL3 using a
// read-modify-write operation.
//
// Call register usage:
// a0: INTEL_SIP_SMC_REG_UPDATE.
// a1: register address
// a2: write Mask.
// a3: value to write.
// a4-7: not used.
//
// Return status:
// a0: INTEL_SIP_SMC_STATUS_OK or INTEL_SIP_SMC_REG_ERROR.
// a1-3: Not used.
//
pub const INTEL_SIP_SMC_FUNCID_REG_UPDATE: c_int = 9;

//
// Request INTEL_SIP_SMC_RSU_STATUS
//
// Request remote status update boot log, call is synchronous.
//
// Call register usage:
// a0 INTEL_SIP_SMC_RSU_STATUS
// a1-7 not used
//
// Return status
// a0: Current Image
// a1: Last Failing Image
// a2: Version | State
// a3: Error details | Error location
//
// Or
//
// a0: INTEL_SIP_SMC_RSU_ERROR
//
pub const INTEL_SIP_SMC_FUNCID_RSU_STATUS: c_int = 11;

//
// Request INTEL_SIP_SMC_RSU_UPDATE
//
// Request to set the offset of the bitstream to boot after reboot, call
// is synchronous.
//
// Call register usage:
// a0 INTEL_SIP_SMC_RSU_UPDATE
// a1 64bit physical address of the configuration data memory in flash
// a2-7 not used
//
// Return status
// a0 INTEL_SIP_SMC_STATUS_OK
//
pub const INTEL_SIP_SMC_FUNCID_RSU_UPDATE: c_int = 12;

//
// Request INTEL_SIP_SMC_ECC_DBE
//
// Sync call used by service driver at EL1 to alert EL3 that a Double
// Bit ECC error has occurred.
//
// Call register usage:
// a0 INTEL_SIP_SMC_ECC_DBE
// a1 SysManager Double Bit Error value
// a2-7 not used
//
// Return status
// a0 INTEL_SIP_SMC_STATUS_OK
//
pub const INTEL_SIP_SMC_FUNCID_ECC_DBE: c_int = 13;

//
// Request INTEL_SIP_SMC_RSU_NOTIFY
//
// Sync call used by service driver at EL1 to report hard processor
// system execution stage to firmware
//
// Call register usage:
// a0 INTEL_SIP_SMC_RSU_NOTIFY
// a1 32bit value representing hard processor system execution stage
// a2-7 not used
//
// Return status
// a0 INTEL_SIP_SMC_STATUS_OK
//
pub const INTEL_SIP_SMC_FUNCID_RSU_NOTIFY: c_int = 14;

//
// Request INTEL_SIP_SMC_RSU_RETRY_COUNTER
//
// Sync call used by service driver at EL1 to query RSU retry counter
//
// Call register usage:
// a0 INTEL_SIP_SMC_RSU_RETRY_COUNTER
// a1-7 not used
//
// Return status
// a0 INTEL_SIP_SMC_STATUS_OK
// a1 the retry counter
//
// Or
//
// a0 INTEL_SIP_SMC_RSU_ERROR
//
pub const INTEL_SIP_SMC_FUNCID_RSU_RETRY_COUNTER: c_int = 15;

//
// Request INTEL_SIP_SMC_RSU_DCMF_VERSION
//
// Sync call used by service driver at EL1 to query DCMF (Decision
// Configuration Management Firmware) version from FW
//
// Call register usage:
// a0 INTEL_SIP_SMC_RSU_DCMF_VERSION
// a1-7 not used
//
// Return status
// a0 INTEL_SIP_SMC_STATUS_OK
// a1 dcmf1 | dcmf0
// a2 dcmf3 | dcmf2
//
// Or
//
// a0 INTEL_SIP_SMC_RSU_ERROR
//
pub const INTEL_SIP_SMC_FUNCID_RSU_DCMF_VERSION: c_int = 16;

//
// Request INTEL_SIP_SMC_RSU_MAX_RETRY
//
// Sync call used by service driver at EL1 to query max retry value from FW
//
// Call register usage:
// a0 INTEL_SIP_SMC_RSU_MAX_RETRY
// a1-7 not used
//
// Return status
// a0 INTEL_SIP_SMC_STATUS_OK
// a1 max retry value
//
// Or
// a0 INTEL_SIP_SMC_RSU_ERROR
//
pub const INTEL_SIP_SMC_FUNCID_RSU_MAX_RETRY: c_int = 18;

//
// Request INTEL_SIP_SMC_RSU_DCMF_STATUS
//
// Sync call used by service driver at EL1 to query DCMF status from FW
//
// Call register usage:
// a0 INTEL_SIP_SMC_RSU_DCMF_STATUS
// a1-7 not used
//
// Return status
// a0 INTEL_SIP_SMC_STATUS_OK
// a1 dcmf3 | dcmf2 | dcmf1 | dcmf0
//
// Or
//
// a0 INTEL_SIP_SMC_RSU_ERROR
//
pub const INTEL_SIP_SMC_FUNCID_RSU_DCMF_STATUS: c_int = 20;

//
// Request INTEL_SIP_SMC_RSU_GET_DEVICE_INFO
//
// Sync call used by service driver at EL1 to query QSPI device info from FW
//
// Call register usage:
// a0 INTEL_SIP_SMC_RSU_GET_DEVICE_INFO
// a1-7 not used
//
// Return status
// a0 INTEL_SIP_SMC_STATUS_OK
// a1 erasesize0 | size0
// a2 erasesize1 | size1
// a3 erasesize2 | size2
// a4 erasesize3 | size3
// Or
//
// a0 INTEL_SIP_SMC_RSU_ERROR
//
pub const INTEL_SIP_SMC_FUNCID_RSU_GET_DEVICE_INFO: c_int = 22;

//
// Request INTEL_SIP_SMC_SERVICE_COMPLETED
// Sync call to check if the secure world have completed service request
// or not.
//
// Call register usage:
// a0: INTEL_SIP_SMC_SERVICE_COMPLETED
// a1: this register is optional. If used, it is the physical address for
// secure firmware to put output data
// a2: this register is optional. If used, it is the size of output data
// a3-a7: not used
//
// Return status:
// a0: INTEL_SIP_SMC_STATUS_OK, INTEL_SIP_SMC_STATUS_ERROR,
// INTEL_SIP_SMC_REJECTED or INTEL_SIP_SMC_STATUS_BUSY
// a1: mailbox error if a0 is INTEL_SIP_SMC_STATUS_ERROR
// a2: physical address containing the process info
// for FCS certificate -- the data contains the certificate status
// for FCS cryption -- the data contains the actual data size FW processes
// a3: output data size
//
pub const INTEL_SIP_SMC_FUNCID_SERVICE_COMPLETED: c_int = 30;

//
// Request INTEL_SIP_SMC_FIRMWARE_VERSION
//
// Sync call used to query the version of running firmware
//
// Call register usage:
// a0 INTEL_SIP_SMC_FIRMWARE_VERSION
// a1-a7 not used
//
// Return status:
// a0 INTEL_SIP_SMC_STATUS_OK or INTEL_SIP_SMC_STATUS_ERROR
// a1 running firmware version
//
pub const INTEL_SIP_SMC_FUNCID_FIRMWARE_VERSION: c_int = 31;

//
// SMC call protocol for Mailbox, starting FUNCID from 60
//
// Call register usage:
// a0 INTEL_SIP_SMC_MBOX_SEND_CMD
// a1 mailbox command code
// a2 physical address that contain mailbox command data (not include header)
// a3 mailbox command data size in word
// a4 set to 0 for CASUAL, set to 1 for URGENT
// a5 physical address for secure firmware to put response data
// (not include header)
// a6 maximum size in word of physical address to store response data
// a7 not used
//
// Return status
// a0 INTEL_SIP_SMC_STATUS_OK, INTEL_SIP_SMC_STATUS_REJECTED or
// INTEL_SIP_SMC_STATUS_ERROR
// a1 mailbox error code
// a2 response data length in word
// a3 not used
//
pub const INTEL_SIP_SMC_FUNCID_MBOX_SEND_CMD: c_int = 60;

//
// Request INTEL_SIP_SMC_SVC_VERSION
//
// Sync call used to query the SIP SMC API Version
//
// Call register usage:
// a0 INTEL_SIP_SMC_SVC_VERSION
// a1-a7 not used
//
// Return status:
// a0 INTEL_SIP_SMC_STATUS_OK
// a1 Major
// a2 Minor
//
pub const INTEL_SIP_SMC_SVC_FUNCID_VERSION: c_int = 512;

//
// Request INTEL_SIP_SMC_ATF_BUILD_VER
//
// Sync call used to query the ATF Build Version
//
// Call register usage:
// a0 INTEL_SIP_SMC_ATF_BUILD_VER
// a1-a7 not used
//
// Return status:
// a0 INTEL_SIP_SMC_STATUS_OK
// a1 Major
// a2 Minor
// a3 Patch
//
pub const INTEL_SIP_SMC_ATF_BUILD_VERSION: c_int = 155;

//
// SMC call protocol for FPGA Crypto Service (FCS)
// FUNCID starts from 90
//
// Request INTEL_SIP_SMC_FCS_RANDOM_NUMBER
//
// Sync call used to query the random number generated by the firmware
//
// Call register usage:
// a0 INTEL_SIP_SMC_FCS_RANDOM_NUMBER
// a1 the physical address for firmware to write generated random data
// a2-a7 not used
//
// Return status:
// a0 INTEL_SIP_SMC_STATUS_OK, INTEL_SIP_SMC_FCS_ERROR or
// INTEL_SIP_SMC_FCS_REJECTED
// a1 mailbox error
// a2 the physical address of generated random number
// a3 size
//
pub const INTEL_SIP_SMC_FUNCID_FCS_RANDOM_NUMBER: c_int = 90;

//
// Request INTEL_SIP_SMC_FCS_CRYPTION
// Async call for data encryption and HMAC signature generation, or for
// data decryption and HMAC verification.
//
// Call INTEL_SIP_SMC_SERVICE_COMPLETED to get the output encrypted or
// decrypted data
//
// Call register usage:
// a0 INTEL_SIP_SMC_FCS_CRYPTION
// a1 cryption mode (1 for encryption and 0 for decryption)
// a2 physical address which stores to be encrypted or decrypted data
// a3 input data size
// a4 physical address which will hold the encrypted or decrypted output data
// a5 output data size
// a6-a7 not used
//
// Return status:
// a0 INTEL_SIP_SMC_STATUS_OK, INTEL_SIP_SMC_STATUS_ERROR or
// INTEL_SIP_SMC_STATUS_REJECTED
// a1-3 not used
//
pub const INTEL_SIP_SMC_FUNCID_FCS_CRYPTION: c_int = 91;

//
// Request INTEL_SIP_SMC_FCS_SERVICE_REQUEST
// Async call for authentication service of HPS software
//
// Call register usage:
// a0 INTEL_SIP_SMC_FCS_SERVICE_REQUEST
// a1 the physical address of data block
// a2 size of data block
// a3-a7 not used
//
// Return status:
// a0 INTEL_SIP_SMC_STATUS_OK, INTEL_SIP_SMC_ERROR or
// INTEL_SIP_SMC_REJECTED
// a1-a3 not used
//
pub const INTEL_SIP_SMC_FUNCID_FCS_SERVICE_REQUEST: c_int = 92;

//
// Request INTEL_SIP_SMC_FUNCID_FCS_SEND_CERTIFICATE
// Async call to send a signed certificate
//
// Call register usage:
// a0 INTEL_SIP_SMC_FCS_SEND_CERTIFICATE
// a1 the physical address of CERTIFICATE block
// a2 size of data block
// a3-a7 not used
//
// Return status:
// a0 INTEL_SIP_SMC_STATUS_OK or INTEL_SIP_SMC_REJECTED
// a1-a3 not used
//
pub const INTEL_SIP_SMC_FUNCID_FCS_SEND_CERTIFICATE: c_int = 93;

//
// Request INTEL_SIP_SMC_FCS_GET_PROVISION_DATA
// Async call to dump all the fuses and key hashes
//
// Call register usage:
// a0 INTEL_SIP_SMC_FCS_GET_PROVISION_DATA
// a1-a7 not used
//
// Return status:
// a0 INTEL_SIP_SMC_STATUS_OK, INTEL_SIP_SMC_STATUS_ERROR or
// INTEL_SIP_SMC_STATUS_REJECTED
// a1 mailbox error if a0 is INTEL_SIP_SMC_STATUS_ERROR
// a2 physical address for the structure of fuse and key hashes
// a3 the size of structure
//
pub const INTEL_SIP_SMC_FUNCID_FCS_GET_PROVISION_DATA: c_int = 94;

//
// Request INTEL_SIP_SMC_HWMON_READTEMP
// Sync call to request temperature
//
// Call register usage:
// a0 Temperature Channel
// a1-a7 not used
//
// Return status
// a0 INTEL_SIP_SMC_STATUS_OK
// a1 Temperature Value
// a2-a3 not used
//
pub const INTEL_SIP_SMC_FUNCID_HWMON_READTEMP: c_int = 32;

//
// Request INTEL_SIP_SMC_HWMON_READVOLT
// Sync call to request voltage
//
// Call register usage:
// a0 Voltage Channel
// a1-a7 not used
//
// Return status
// a0 INTEL_SIP_SMC_STATUS_OK
// a1 Voltage Value
// a2-a3 not used
//
pub const INTEL_SIP_SMC_FUNCID_HWMON_READVOLT: c_int = 33;

//
// Request INTEL_SIP_SMC_ASYNC_POLL
// Async call used by service driver at EL1 to query mailbox response from SDM.
//
// Call register usage:
// a0 INTEL_SIP_SMC_ASYNC_POLL
// a1 transaction job id
// a2-17 will be used to return the response data
//
// Return status
// a0 INTEL_SIP_SMC_STATUS_OK
// a1-17 will contain the response values from mailbox for the previous send
// transaction
// Or
// a0 INTEL_SIP_SMC_STATUS_NO_RESPONSE
// a1-17 not used
//

//
// Request INTEL_SIP_SMC_ASYNC_HWMON_READTEMP
// Async call to request temperature
//
// Call register usage:
// a0 INTEL_SIP_SMC_ASYNC_HWMON_READTEMP
// a1 transaction job id
// a2 Temperature Channel
// a3-a17 not used
//
// Return status
// a0 INTEL_SIP_SMC_STATUS_OK, INTEL_SIP_SMC_STATUS_REJECTED
// or INTEL_SIP_SMC_STATUS_BUSY
// a1-a17 not used
//
pub const INTEL_SIP_SMC_ASYNC_FUNC_ID_HWMON_READTEMP: c_uint = 0xE8;

//
// Request INTEL_SIP_SMC_ASYNC_HWMON_READVOLT
// Async call to request voltage
//
// Call register usage:
// a0 INTEL_SIP_SMC_ASYNC_HWMON_READVOLT
// a1 transaction job id
// a2 Voltage Channel
// a3-a17 not used
//
// Return status
// a0 INTEL_SIP_SMC_STATUS_OK, INTEL_SIP_SMC_STATUS_REJECTED
// or INTEL_SIP_SMC_STATUS_BUSY
// a1-a17 not used
//
pub const INTEL_SIP_SMC_ASYNC_FUNC_ID_HWMON_READVOLT: c_uint = 0xE9;

//
// Request INTEL_SIP_SMC_ASYNC_RSU_GET_SPT
// Async call to get RSU SPT from SDM.
// Call register usage:
// a0 INTEL_SIP_SMC_ASYNC_RSU_GET_SPT
// a1 transaction job id
// a2-a17 not used
//
// Return status:
// a0 INTEL_SIP_SMC_STATUS_OK ,INTEL_SIP_SMC_STATUS_REJECTED
// or INTEL_SIP_SMC_STATUS_BUSY
// a1-a17 not used
//

//
// Request INTEL_SIP_SMC_ASYNC_RSU_GET_ERROR_STATUS
// Async call to get RSU error status from SDM.
// Call register usage:
// a0 INTEL_SIP_SMC_ASYNC_RSU_GET_ERROR_STATUS
// a1 transaction job id
// a2-a17 not used
//
// Return status:
// a0 INTEL_SIP_SMC_STATUS_OK ,INTEL_SIP_SMC_STATUS_REJECTED
// or INTEL_SIP_SMC_STATUS_BUSY
// a1-a17 not used
//

//
// Request INTEL_SIP_SMC_ASYNC_RSU_NOTIFY
// Async call to send NOTIFY value to SDM.
// Call register usage:
// a0 INTEL_SIP_SMC_ASYNC_RSU_NOTIFY
// a1 transaction job id
// a2 notify value
// a3-a17 not used
//
// Return status:
// a0 INTEL_SIP_SMC_STATUS_OK ,INTEL_SIP_SMC_STATUS_REJECTED
// or INTEL_SIP_SMC_STATUS_BUSY
// a1-a17 not used
//

