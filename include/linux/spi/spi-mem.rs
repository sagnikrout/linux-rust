//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/spi/spi-mem.h
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
// Copyright (C) 2018 Exceet Electronics GmbH
// Copyright (C) 2018 Bootlin
//
// Author:
// Peter Pan <peterpandong@micron.com>
// Boris Brezillon <boris.brezillon@bootlin.com>
//

//
// enum spi_mem_data_dir - describes the direction of a SPI memory data
// transfer from the controller perspective
// @SPI_MEM_NO_DATA: no data transferred
// @SPI_MEM_DATA_IN: data coming from the SPI memory
// @SPI_MEM_DATA_OUT: data sent to the SPI memory
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spi_mem_data_dir {
    SPI_MEM_NO_DATA,
    SPI_MEM_DATA_IN,
    SPI_MEM_DATA_OUT,
}

//
// struct spi_mem_op - describes a SPI memory operation
// @cmd: the complete command
// @cmd.nbytes: number of opcode bytes (only 1 or 2 are valid). The opcode is
// sent MSB-first.
// @cmd.buswidth: number of IO lines used to transmit the command
// @cmd.opcode: operation opcode
// @cmd.dtr: whether the command opcode should be sent in DTR mode or not
// @addr: the address attributes
// @addr.nbytes: number of address bytes to send. Can be zero if the operation
// does not need to send an address
// @addr.buswidth: number of IO lines used to transmit the address cycles
// @addr.dtr: whether the address should be sent in DTR mode or not
// @addr.val: address value. This value is always sent MSB first on the bus.
// Note that only @addr.nbytes are taken into account in this
// address value, so users should make sure the value fits in the
// assigned number of bytes.
// @dummy: data for dummy operation
// @dummy.nbytes: number of dummy bytes to send after an opcode or address. Can
// be zero if the operation does not require dummy bytes
// @dummy.buswidth: number of IO lanes used to transmit the dummy bytes
// @dummy.dtr: whether the dummy bytes should be sent in DTR mode or not
// @data: the data attributes
// @data.buswidth: number of IO lanes used to send/receive the data
// @data.dtr: whether the data should be sent in DTR mode or not
// @data.ecc: whether error correction is required or not
// @data.swap16: whether the byte order of 16-bit words is swapped when read
// or written in Octal DTR mode compared to STR mode.
// @data.dir: direction of the transfer
// @data.nbytes: number of data bytes to send/receive. Can be zero if the
// operation does not involve transferring data
// @data.buf.in: input buffer (must be DMA-able)
// @data.buf.out: output buffer (must be DMA-able)
// @max_freq: frequency limitation wrt this operation. 0 means there is no
// specific constraint and the highest achievable frequency can be
// attempted.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_mem_op {
    pub nbytes: u8,
    pub buswidth: u8,
    pub 1: u8 dtr :,
    pub 7: u8 __pad :,
    pub opcode: u16,
    pub cmd: },
    pub nbytes: u8,
    pub buswidth: u8,
    pub 1: u8 dtr :,
    pub 7: u8 __pad :,
    pub val: u64,
    pub addr: },
    pub nbytes: u8,
    pub buswidth: u8,
    pub 1: u8 dtr :,
    pub 7: u8 __pad :,
    pub dummy: },
    pub buswidth: u8,
    pub 1: u8 dtr :,
    pub 1: u8 ecc :,
    pub 1: u8 swap16 :,
    pub 5: u8 __pad :,
    pub dir: spi_mem_data_dir,
    pub nbytes: c_uint,
    pub in: *mut c_void,
    pub out: *const c_void,
    pub buf: },
    pub data: },
    pub max_freq: c_uint,
}

//
// struct spi_mem_dirmap_info - Direct mapping information
// @op_tmpl: operation template that should be used by the direct mapping when
// the memory device is accessed
// @secondary_op_tmpl: secondary template, may be used as an alternative to the
// primary template (decided by the upper layer)
// @offset: absolute offset this direct mapping is pointing to
// @length: length in byte of this direct mapping
//
// These information are used by the controller specific implementation to know
// the portion of memory that is directly mapped and the spi_mem_op that should
// be used to access the device.
// A direct mapping is only valid for one direction (read or write) and this
// direction is directly encoded in the ->op_tmpl.data.dir field.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_mem_dirmap_info {
    pub op_tmpl: *mut spi_mem_op,
    pub primary_op_tmpl: spi_mem_op,
    pub secondary_op_tmpl: spi_mem_op,
    pub offset: u64,
    pub length: u64,
}

//
// struct spi_mem_dirmap_desc - Direct mapping descriptor
// @mem: the SPI memory device this direct mapping is attached to
// @info: information passed at direct mapping creation time
// @nodirmap: set to 1 if the SPI controller does not implement
// ->mem_ops->dirmap_create() or when this function returned an
// error. If @nodirmap is true, all spi_mem_dirmap_{read,write}()
// calls will use spi_mem_exec_op() to access the memory. This is a
// degraded mode that allows spi_mem drivers to use the same code
// no matter whether the controller supports direct mapping or not
// @priv: field pointing to controller specific data
//
// Common part of a direct mapping descriptor. This object is created by
// spi_mem_dirmap_create() and controller implementation of ->create_dirmap()
// can create/attach direct mapping resources to the descriptor in the ->priv
// field.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_mem_dirmap_desc {
    pub mem: *mut spi_mem,
    pub info: spi_mem_dirmap_info,
    pub nodirmap: c_uint,
    pub priv: *mut c_void,
}

//
// struct spi_mem - describes a SPI memory device
// @spi: the underlying SPI device
// @drvpriv: spi_mem_driver private data
// @name: name of the SPI memory device
// @dqs: extra data trobe pin available for high frequency read operations
//
// Extra information that describe the SPI memory device and may be needed by
// the controller to properly handle this device should be placed here.
//
// One example would be the device size since some controller expose their SPI
// mem devices through a io-mapped region.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_mem {
    pub spi: *mut spi_device,
    pub drvpriv: *mut c_void,
    pub name: *const c_char,
    pub dqs: bool,
}

//
// spi_mem_set_drvdata() - attach driver private data to a SPI mem
// device
// @mem: memory device
// @data: data to attach to the memory device
//
// spi_mem_get_drvdata() - get driver private data attached to a SPI mem
// device
// @mem: memory device
//
// Return: the data attached to the mem device.
//
// struct spi_controller_mem_ops - SPI memory operations
// @adjust_op_size: shrink the data xfer of an operation to match controller's
// limitations (can be alignment or max RX/TX size
// limitations)
// @supports_op: check if an operation is supported by the controller
// @exec_op: execute a SPI memory operation
// not all driver provides supports_op(), so it can return -EOPNOTSUPP
// if the op is not supported by the driver/controller
// @get_name: get a custom name for the SPI mem device from the controller.
// This might be needed if the controller driver has been ported
// to use the SPI mem layer and a custom name is used to keep
// mtdparts compatible.
// Note that if the implementation of this function allocates memory
// dynamically, then it should do so with devm_xxx(), as we don't
// have a ->free_name() function.
// @dirmap_create: create a direct mapping descriptor that can later be used to
// access the memory device. This method is optional
// @dirmap_destroy: destroy a memory descriptor previous created by
// ->dirmap_create()
// @dirmap_read: read data from the memory device using the direct mapping
// created by ->dirmap_create(). The function can return less
// data than requested (for example when the request is crossing
// the currently mapped area), and the caller of
// spi_mem_dirmap_read() is responsible for calling it again in
// this case.
// @dirmap_write: write data to the memory device using the direct mapping
// created by ->dirmap_create(). The function can return less
// data than requested (for example when the request is crossing
// the currently mapped area), and the caller of
// spi_mem_dirmap_write() is responsible for calling it again in
// this case.
// @poll_status: poll memory device status until (status & mask) == match or
// when the timeout has expired. It fills the data buffer with
// the last status value.
//
// This interface should be implemented by SPI controllers providing an
// high-level interface to execute SPI memory operation, which is usually the
// case for QSPI controllers.
//
// Note on ->dirmap_{read,write}(): drivers should avoid accessing the direct
// mapping from the CPU because doing that can stall the CPU waiting for the
// SPI mem transaction to finish, and this will make real-time maintainers
// unhappy and might make your system less reactive. Instead, drivers should
// use DMA to access this direct mapping.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_controller_mem_ops {
    pub op): *mut *mut *mut int (adjust_op_size)(struct spi_mem mem, struct spi_mem_op,
    pub op): *const spi_mem_op,
    pub op): *const spi_mem_op,
    pub mem): *const *const *const char (get_name)(struct spi_mem,
    pub desc): *mut *mut int (dirmap_create)(struct spi_mem_dirmap_desc,
    pub desc): *mut *mut void (dirmap_destroy)(struct spi_mem_dirmap_desc,
    pub buf): *mut u64 offs, size_t len, void,
    pub buf): *const u64 offs, size_t len, void,
    pub timeout_ms): c_ulong,
}

//
// struct spi_controller_mem_caps - SPI memory controller capabilities
// @dtr: Supports DTR operations
// @ecc: Supports operations with error correction
// @swap16: Supports swapping bytes on a 16 bit boundary when configured in
// Octal DTR
// @per_op_freq: Supports per operation frequency switching
// @secondary_op_tmpl: Supports leveraging a secondary memory operation template
// @no_cs_assertion: The controller may automatically deassert the CS if there
// is a pause in the transfer (eg. internal bus contention or
// DMA arbitration on an interconnect). Features such as NAND
// continuous reads shall not be leveraged.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_controller_mem_caps {
    pub dtr: bool,
    pub ecc: bool,
    pub swap16: bool,
    pub per_op_freq: bool,
    pub secondary_op_tmpl: bool,
    pub no_cs_assertion: bool,
}

//
// struct spi_mem_driver - SPI memory driver
// @spidrv: inherit from a SPI driver
// @probe: probe a SPI memory. Usually where detection/initialization takes
// place
// @remove: remove a SPI memory
// @shutdown: take appropriate action when the system is shutdown
//
// This is just a thin wrapper around a spi_driver. The core takes care of
// allocating the spi_mem object and forwarding the probe/remove/shutdown
// request to the spi_mem_driver. The reason we use this wrapper is because
// we might have to stuff more information into the spi_mem struct to let
// SPI controllers know more about the SPI memory they interact with, and
// having this intermediate layer allows us to do that without adding more
// useless fields to the spi_device object.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_mem_driver {
    pub spidrv: spi_driver,
    pub mem): *mut *mut int (probe)(struct spi_mem,
    pub mem): *mut *mut int (remove)(struct spi_mem,
    pub mem): *mut *mut void (shutdown)(struct spi_mem,
}

extern "C" {
    pub fn spi_mem_set_dqs(mem: *mut spi_mem);
}
extern "C" {
    pub fn spi_mem_has_dqs(mem: *mut spi_mem) -> bool;
}
extern "C" {
    pub fn spi_mem_adjust_op_size(mem: *mut spi_mem, op: *mut spi_mem_op) -> c_int;
}
extern "C" {
    pub fn spi_mem_adjust_op_freq(mem: *mut spi_mem, op: *mut spi_mem_op);
}
extern "C" {
    pub fn spi_mem_calc_op_duration(mem: *mut spi_mem, op: *mut spi_mem_op) -> u64;
}
extern "C" {
    pub fn spi_mem_dirmap_destroy(desc: *mut spi_mem_dirmap_desc);
}
extern "C" {
    pub fn spi_mem_driver_unregister(drv: *mut spi_mem_driver);
}

