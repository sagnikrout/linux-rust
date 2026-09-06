//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/cdx/controller/mc_cdx_pcol.h
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
// Driver for AMD network controllers and boards
//
// Copyright (C) 2021, Xilinx, Inc.
// Copyright (C) 2022-2023, Advanced Micro Devices, Inc.
//
// The current version of the MCDI protocol.
pub const MCDI_PCOL_VERSION: c_int = 2;
//
// Each MCDI request starts with an MCDI_HEADER, which is a 32bit
// structure, filled in by the client.
//
// 0       7  8     16    20     22  23  24    31
// | CODE | R | LEN | SEQ | Rsvd | E | R | XFLAGS |
// |                      |   |
// |                      |   \--- Response
// |                      \------- Error
// \------------------------------ Resync (always set)
//
// The client writes its request into MC shared memory, and rings the
// doorbell. Each request is completed either by the MC writing
// back into shared memory, or by writing out an event.
//
// All MCDI commands support completion by shared memory response. Each
// request may also contain additional data (accounted for by HEADER.LEN),
// and some responses may also contain additional data (again, accounted
// for by HEADER.LEN).
//
// Some MCDI commands support completion by event, in which any associated
// response data is included in the event.
//
// The protocol requires one response to be delivered for every request; a
// request should not be sent unless the response for the previous request
// has been received (either by polling shared memory, or by receiving
// an event).
//
// Request/Response structure
pub const MCDI_HEADER_OFST: c_int = 0;
pub const MCDI_HEADER_CODE_LBN: c_int = 0;
pub const MCDI_HEADER_CODE_WIDTH: c_int = 7;
pub const MCDI_HEADER_RESYNC_LBN: c_int = 7;
pub const MCDI_HEADER_RESYNC_WIDTH: c_int = 1;
pub const MCDI_HEADER_DATALEN_LBN: c_int = 8;
pub const MCDI_HEADER_DATALEN_WIDTH: c_int = 8;
pub const MCDI_HEADER_SEQ_LBN: c_int = 16;
pub const MCDI_HEADER_SEQ_WIDTH: c_int = 4;
pub const MCDI_HEADER_RSVD_LBN: c_int = 20;
pub const MCDI_HEADER_RSVD_WIDTH: c_int = 1;
pub const MCDI_HEADER_NOT_EPOCH_LBN: c_int = 21;
pub const MCDI_HEADER_NOT_EPOCH_WIDTH: c_int = 1;
pub const MCDI_HEADER_ERROR_LBN: c_int = 22;
pub const MCDI_HEADER_ERROR_WIDTH: c_int = 1;
pub const MCDI_HEADER_RESPONSE_LBN: c_int = 23;
pub const MCDI_HEADER_RESPONSE_WIDTH: c_int = 1;
pub const MCDI_HEADER_XFLAGS_LBN: c_int = 24;
pub const MCDI_HEADER_XFLAGS_WIDTH: c_int = 8;
// Request response using event
pub const MCDI_HEADER_XFLAGS_EVREQ: c_uint = 0x01;
// Request (and signal) early doorbell return
pub const MCDI_HEADER_XFLAGS_DBRET: c_uint = 0x02;
// Maximum number of payload bytes
pub const MCDI_CTL_SDU_LEN_MAX_V2: c_uint = 0x400;

//
// The MC can generate events for two reasons:
// - To advance a shared memory request if XFLAGS_EVREQ was set
// - As a notification (link state, i2c event), controlled
// via MC_CMD_LOG_CTRL
//
// Both events share a common structure:
//
// 0      32     33      36    44     52     60
// | Data | Cont | Level | Src | Code | Rsvd |
// |
// \ There is another event pending in this notification
//
// If Code==CMDDONE, then the fields are further interpreted as:
//
// - LEVEL==INFO    Command succeeded
// - LEVEL==ERR     Command failed
//
// 0     8         16      24     32
// | Seq | Datalen | Errno | Rsvd |
//
// These fields are taken directly out of the standard MCDI header, i.e.,
// LEVEL==ERR, Datalen == 0 => Reboot
//
// Events can be squirted out of the UART (using LOG_CTRL) without a
// MCDI header.  An event can be distinguished from a MCDI response by
// examining the first byte which is 0xc0.  This corresponds to the
// non-existent MCDI command MC_CMD_DEBUG_LOG.
//
// 0         7        8
// | command | Resync |     = 0xc0
//
// Since the event is written in big-endian byte order, this works
// providing bits 56-63 of the event are 0xc0.
//
// 56     60  63
// | Rsvd | Code |    = 0xc0
//
// Which means for convenience the event code is 0xc for all MC
// generated events.
//
// the errno value may be followed by the (0-based) number of the
// first argument that could not be processed.
//
pub const MC_CMD_ERR_ARG_OFST: c_int = 4;
// MC_CMD_ERR MCDI error codes.
// Operation not permitted.
pub const MC_CMD_ERR_EPERM: c_uint = 0x1;
// Non-existent command target
pub const MC_CMD_ERR_ENOENT: c_uint = 0x2;
// assert() has killed the MC
pub const MC_CMD_ERR_EINTR: c_uint = 0x4;
// I/O failure
pub const MC_CMD_ERR_EIO: c_uint = 0x5;
// Already exists
pub const MC_CMD_ERR_EEXIST: c_uint = 0x6;
// Try again
pub const MC_CMD_ERR_EAGAIN: c_uint = 0xb;
// Out of memory
pub const MC_CMD_ERR_ENOMEM: c_uint = 0xc;
// Caller does not hold required locks
pub const MC_CMD_ERR_EACCES: c_uint = 0xd;
// Resource is currently unavailable (e.g. lock contention)
pub const MC_CMD_ERR_EBUSY: c_uint = 0x10;
// No such device
pub const MC_CMD_ERR_ENODEV: c_uint = 0x13;
// Invalid argument to target
pub const MC_CMD_ERR_EINVAL: c_uint = 0x16;
// No space
pub const MC_CMD_ERR_ENOSPC: c_uint = 0x1c;
// Read-only
pub const MC_CMD_ERR_EROFS: c_uint = 0x1e;
// Broken pipe
pub const MC_CMD_ERR_EPIPE: c_uint = 0x20;
// Out of range
pub const MC_CMD_ERR_ERANGE: c_uint = 0x22;
// Non-recursive resource is already acquired
pub const MC_CMD_ERR_EDEADLK: c_uint = 0x23;
// Operation not implemented
pub const MC_CMD_ERR_ENOSYS: c_uint = 0x26;
// Operation timed out
pub const MC_CMD_ERR_ETIME: c_uint = 0x3e;
// Link has been severed
pub const MC_CMD_ERR_ENOLINK: c_uint = 0x43;
// Protocol error
pub const MC_CMD_ERR_EPROTO: c_uint = 0x47;
// Bad message
pub const MC_CMD_ERR_EBADMSG: c_uint = 0x4a;
// Operation not supported
pub const MC_CMD_ERR_ENOTSUP: c_uint = 0x5f;
// Address not available
pub const MC_CMD_ERR_EADDRNOTAVAIL: c_uint = 0x63;
// Not connected
pub const MC_CMD_ERR_ENOTCONN: c_uint = 0x6b;
// Operation already in progress
pub const MC_CMD_ERR_EALREADY: c_uint = 0x72;
// Stale handle. The handle references resource that no longer exists
pub const MC_CMD_ERR_ESTALE: c_uint = 0x74;
// Resource allocation failed.
pub const MC_CMD_ERR_ALLOC_FAIL: c_uint = 0x1000;
// V-adaptor not found.
pub const MC_CMD_ERR_NO_VADAPTOR: c_uint = 0x1001;
// EVB port not found.
pub const MC_CMD_ERR_NO_EVB_PORT: c_uint = 0x1002;
// V-switch not found.
pub const MC_CMD_ERR_NO_VSWITCH: c_uint = 0x1003;
// Too many VLAN tags.
pub const MC_CMD_ERR_VLAN_LIMIT: c_uint = 0x1004;
// Bad PCI function number.
pub const MC_CMD_ERR_BAD_PCI_FUNC: c_uint = 0x1005;
// Invalid VLAN mode.
pub const MC_CMD_ERR_BAD_VLAN_MODE: c_uint = 0x1006;
// Invalid v-switch type.
pub const MC_CMD_ERR_BAD_VSWITCH_TYPE: c_uint = 0x1007;
// Invalid v-port type.
pub const MC_CMD_ERR_BAD_VPORT_TYPE: c_uint = 0x1008;
// MAC address exists.
pub const MC_CMD_ERR_MAC_EXIST: c_uint = 0x1009;
// Slave core not present
pub const MC_CMD_ERR_SLAVE_NOT_PRESENT: c_uint = 0x100a;
// The datapath is disabled.
pub const MC_CMD_ERR_DATAPATH_DISABLED: c_uint = 0x100b;
// The requesting client is not a function
pub const MC_CMD_ERR_CLIENT_NOT_FN: c_uint = 0x100c;
//
// The requested operation might require the command to be passed between
// MCs, and the transport doesn't support that. Should only ever been seen over
// the UART.
//
pub const MC_CMD_ERR_NO_PRIVILEGE: c_uint = 0x1013;
//
// Workaround 26807 could not be turned on/off because some functions
// have already installed filters. See the comment at
// MC_CMD_WORKAROUND_BUG26807. May also returned for other operations such as
// sub-variant switching.
//
pub const MC_CMD_ERR_FILTERS_PRESENT: c_uint = 0x1014;
// The clock whose frequency you've attempted to set doesn't exist
pub const MC_CMD_ERR_NO_CLOCK: c_uint = 0x1015;
//
// Returned by MC_CMD_TESTASSERT if the action that should have caused an
// assertion failed to do so.
//
pub const MC_CMD_ERR_UNREACHABLE: c_uint = 0x1016;
//
// This command needs to be processed in the background but there were no
// resources to do so. Send it again after a command has completed.
//
pub const MC_CMD_ERR_QUEUE_FULL: c_uint = 0x1017;
//
// The operation could not be completed because the PCIe link has gone
// away. This error code is never expected to be returned over the TLP
// transport.
//
pub const MC_CMD_ERR_NO_PCIE: c_uint = 0x1018;
//
// The operation could not be completed because the datapath has gone
// away. This is distinct from MC_CMD_ERR_DATAPATH_DISABLED in that the
// datapath absence may be temporary
//
pub const MC_CMD_ERR_NO_DATAPATH: c_uint = 0x1019;
// The operation could not complete because some VIs are allocated
pub const MC_CMD_ERR_VIS_PRESENT: c_uint = 0x101a;
//
// The operation could not complete because some PIO buffers are
// allocated
//
pub const MC_CMD_ERR_PIOBUFS_PRESENT: c_uint = 0x101b;
//
// MC_CMD_CDX_BUS_ENUM_BUSES
// CDX bus hosts devices (functions) that are implemented using the Composable
// DMA subsystem and directly mapped into the memory space of the FGPA PSX
// Application Processors (APUs). As such, they only apply to the PSX APU side,
// not the host (PCIe). Unlike PCIe, these devices have no native configuration
// space or enumeration mechanism, so this message set provides a minimal
// interface for discovery and management (bus reset, FLR, BME) of such
// devices. This command returns the number of CDX buses present in the system.
//
pub const MC_CMD_CDX_BUS_ENUM_BUSES: c_uint = 0x1;
pub const MC_CMD_CDX_BUS_ENUM_BUSES_MSGSET: c_uint = 0x1;

// MC_CMD_CDX_BUS_ENUM_BUSES_IN msgrequest
pub const MC_CMD_CDX_BUS_ENUM_BUSES_IN_LEN: c_int = 0;
// MC_CMD_CDX_BUS_ENUM_BUSES_OUT msgresponse
pub const MC_CMD_CDX_BUS_ENUM_BUSES_OUT_LEN: c_int = 4;
//
// Number of CDX buses present in the system. Buses are numbered 0 to
// BUS_COUNT-1
//
pub const MC_CMD_CDX_BUS_ENUM_BUSES_OUT_BUS_COUNT_OFST: c_int = 0;
pub const MC_CMD_CDX_BUS_ENUM_BUSES_OUT_BUS_COUNT_LEN: c_int = 4;
//
// MC_CMD_CDX_BUS_ENUM_DEVICES
// Enumerate CDX bus devices on a given bus
//
pub const MC_CMD_CDX_BUS_ENUM_DEVICES: c_uint = 0x2;
pub const MC_CMD_CDX_BUS_ENUM_DEVICES_MSGSET: c_uint = 0x2;

// MC_CMD_CDX_BUS_ENUM_DEVICES_IN msgrequest
pub const MC_CMD_CDX_BUS_ENUM_DEVICES_IN_LEN: c_int = 4;
//
// Bus number to enumerate, in range 0 to BUS_COUNT-1, as returned by
// MC_CMD_CDX_BUS_ENUM_BUSES_OUT
//
pub const MC_CMD_CDX_BUS_ENUM_DEVICES_IN_BUS_OFST: c_int = 0;
pub const MC_CMD_CDX_BUS_ENUM_DEVICES_IN_BUS_LEN: c_int = 4;
// MC_CMD_CDX_BUS_ENUM_DEVICES_OUT msgresponse
pub const MC_CMD_CDX_BUS_ENUM_DEVICES_OUT_LEN: c_int = 4;
//
// Number of devices present on the bus. Devices on the bus are numbered 0 to
// DEVICE_COUNT-1. Returns EAGAIN if number of devices unknown or if the target
// devices are not ready (e.g. undergoing a bus reset)
//
pub const MC_CMD_CDX_BUS_ENUM_DEVICES_OUT_DEVICE_COUNT_OFST: c_int = 0;
pub const MC_CMD_CDX_BUS_ENUM_DEVICES_OUT_DEVICE_COUNT_LEN: c_int = 4;
//
// MC_CMD_CDX_BUS_GET_DEVICE_CONFIG
// Returns device identification and MMIO/MSI resource data for a CDX device.
// The expected usage is for the caller to first retrieve the number of devices
// on the bus using MC_CMD_BUS_ENUM_DEVICES, then loop through the range (0,
// DEVICE_COUNT - 1), retrieving device resource data. May return EAGAIN if the
// number of exposed devices or device resources change during enumeration (due
// to e.g. a PL reload / bus reset), in which case the caller is expected to
// restart the enumeration loop. MMIO addresses are specified in terms of bus
// addresses (prior to any potential IOMMU translation). For versal-net, these
// are equivalent to APU physical addresses. Implementation note - for this to
// work, the implementation needs to keep state (generation count) per client.
//
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG: c_uint = 0x3;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_MSGSET: c_uint = 0x3;

// MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_IN msgrequest
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_IN_LEN: c_int = 8;
// Device bus number, in range 0 to BUS_COUNT-1
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_IN_BUS_OFST: c_int = 0;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_IN_BUS_LEN: c_int = 4;
// Device number relative to the bus, in range 0 to DEVICE_COUNT-1 for that bus
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_IN_DEVICE_OFST: c_int = 4;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_IN_DEVICE_LEN: c_int = 4;
// MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT msgresponse
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_LEN: c_int = 88;
// 16-bit Vendor identifier, compliant with PCI-SIG VendorID assignment.
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_VENDOR_ID_OFST: c_int = 0;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_VENDOR_ID_LEN: c_int = 2;
// 16-bit Device ID assigned by the vendor
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_DEVICE_ID_OFST: c_int = 2;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_DEVICE_ID_LEN: c_int = 2;
//
// 16-bit Subsystem Vendor ID, , compliant with PCI-SIG VendorID assignment.
// For further device differentiation, as required. 0 if unused.
//
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_SUBSYS_VENDOR_ID_OFST: c_int = 4;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_SUBSYS_VENDOR_ID_LEN: c_int = 2;
//
// 16-bit Subsystem Device ID assigned by the vendor. For further device
// differentiation, as required. 0 if unused.
//
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_SUBSYS_DEVICE_ID_OFST: c_int = 6;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_SUBSYS_DEVICE_ID_LEN: c_int = 2;
// 24-bit Device Class code, compliant with PCI-SIG Device Class codes
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_DEVICE_CLASS_OFST: c_int = 8;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_DEVICE_CLASS_LEN: c_int = 3;
// 8-bit vendor-assigned revision
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_DEVICE_REVISION_OFST: c_int = 11;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_DEVICE_REVISION_LEN: c_int = 1;
// Reserved (alignment)
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_RESERVED_OFST: c_int = 12;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_RESERVED_LEN: c_int = 4;
// MMIO region 0 base address (bus address), 0 if unused
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION0_BASE_OFST: c_int = 16;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION0_BASE_LEN: c_int = 8;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION0_BASE_LO_OFST: c_int = 16;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION0_BASE_LO_LEN: c_int = 4;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION0_BASE_LO_LBN: c_int = 128;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION0_BASE_LO_WIDTH: c_int = 32;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION0_BASE_HI_OFST: c_int = 20;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION0_BASE_HI_LEN: c_int = 4;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION0_BASE_HI_LBN: c_int = 160;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION0_BASE_HI_WIDTH: c_int = 32;
// MMIO region 0 size, 0 if unused
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION0_SIZE_OFST: c_int = 24;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION0_SIZE_LEN: c_int = 8;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION0_SIZE_LO_OFST: c_int = 24;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION0_SIZE_LO_LEN: c_int = 4;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION0_SIZE_LO_LBN: c_int = 192;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION0_SIZE_LO_WIDTH: c_int = 32;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION0_SIZE_HI_OFST: c_int = 28;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION0_SIZE_HI_LEN: c_int = 4;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION0_SIZE_HI_LBN: c_int = 224;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION0_SIZE_HI_WIDTH: c_int = 32;
// MMIO region 1 base address (bus address), 0 if unused
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION1_BASE_OFST: c_int = 32;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION1_BASE_LEN: c_int = 8;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION1_BASE_LO_OFST: c_int = 32;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION1_BASE_LO_LEN: c_int = 4;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION1_BASE_LO_LBN: c_int = 256;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION1_BASE_LO_WIDTH: c_int = 32;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION1_BASE_HI_OFST: c_int = 36;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION1_BASE_HI_LEN: c_int = 4;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION1_BASE_HI_LBN: c_int = 288;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION1_BASE_HI_WIDTH: c_int = 32;
// MMIO region 1 size, 0 if unused
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION1_SIZE_OFST: c_int = 40;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION1_SIZE_LEN: c_int = 8;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION1_SIZE_LO_OFST: c_int = 40;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION1_SIZE_LO_LEN: c_int = 4;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION1_SIZE_LO_LBN: c_int = 320;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION1_SIZE_LO_WIDTH: c_int = 32;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION1_SIZE_HI_OFST: c_int = 44;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION1_SIZE_HI_LEN: c_int = 4;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION1_SIZE_HI_LBN: c_int = 352;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION1_SIZE_HI_WIDTH: c_int = 32;
// MMIO region 2 base address (bus address), 0 if unused
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION2_BASE_OFST: c_int = 48;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION2_BASE_LEN: c_int = 8;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION2_BASE_LO_OFST: c_int = 48;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION2_BASE_LO_LEN: c_int = 4;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION2_BASE_LO_LBN: c_int = 384;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION2_BASE_LO_WIDTH: c_int = 32;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION2_BASE_HI_OFST: c_int = 52;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION2_BASE_HI_LEN: c_int = 4;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION2_BASE_HI_LBN: c_int = 416;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION2_BASE_HI_WIDTH: c_int = 32;
// MMIO region 2 size, 0 if unused
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION2_SIZE_OFST: c_int = 56;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION2_SIZE_LEN: c_int = 8;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION2_SIZE_LO_OFST: c_int = 56;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION2_SIZE_LO_LEN: c_int = 4;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION2_SIZE_LO_LBN: c_int = 448;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION2_SIZE_LO_WIDTH: c_int = 32;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION2_SIZE_HI_OFST: c_int = 60;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION2_SIZE_HI_LEN: c_int = 4;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION2_SIZE_HI_LBN: c_int = 480;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION2_SIZE_HI_WIDTH: c_int = 32;
// MMIO region 3 base address (bus address), 0 if unused
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION3_BASE_OFST: c_int = 64;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION3_BASE_LEN: c_int = 8;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION3_BASE_LO_OFST: c_int = 64;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION3_BASE_LO_LEN: c_int = 4;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION3_BASE_LO_LBN: c_int = 512;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION3_BASE_LO_WIDTH: c_int = 32;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION3_BASE_HI_OFST: c_int = 68;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION3_BASE_HI_LEN: c_int = 4;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION3_BASE_HI_LBN: c_int = 544;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION3_BASE_HI_WIDTH: c_int = 32;
// MMIO region 3 size, 0 if unused
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION3_SIZE_OFST: c_int = 72;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION3_SIZE_LEN: c_int = 8;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION3_SIZE_LO_OFST: c_int = 72;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION3_SIZE_LO_LEN: c_int = 4;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION3_SIZE_LO_LBN: c_int = 576;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION3_SIZE_LO_WIDTH: c_int = 32;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION3_SIZE_HI_OFST: c_int = 76;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION3_SIZE_HI_LEN: c_int = 4;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION3_SIZE_HI_LBN: c_int = 608;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION3_SIZE_HI_WIDTH: c_int = 32;
// MSI vector count
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MSI_COUNT_OFST: c_int = 80;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_MSI_COUNT_LEN: c_int = 4;
// Requester ID used by device (SMMU StreamID, GIC ITS DeviceID)
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_REQUESTER_ID_OFST: c_int = 84;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_REQUESTER_ID_LEN: c_int = 4;
// MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_V2 msgresponse
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_V2_LEN: c_int = 92;
// Requester ID used by device for GIC ITS DeviceID
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_V2_REQUESTER_DEVICE_ID_OFST: c_int = 88;
pub const MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_V2_REQUESTER_DEVICE_ID_LEN: c_int = 4;
//
// MC_CMD_CDX_BUS_DOWN
// Asserting reset on the CDX bus causes all devices on the bus to be quiesced.
// DMA bus mastering is disabled and any pending DMA request are flushed. Once
// the response is returned, the devices are guaranteed to no longer issue DMA
// requests or raise MSI interrupts. Further device MMIO accesses may have
// undefined results. While the bus reset is asserted, any of the enumeration
// or device configuration MCDIs will fail with EAGAIN. It is only legal to
// reload the relevant PL region containing CDX devices if the corresponding CDX
// bus is in reset. Depending on the implementation, the firmware may or may
// not enforce this restriction and it is up to the caller to make sure this
// requirement is satisfied.
//
pub const MC_CMD_CDX_BUS_DOWN: c_uint = 0x4;
pub const MC_CMD_CDX_BUS_DOWN_MSGSET: c_uint = 0x4;
// MC_CMD_CDX_BUS_DOWN_IN msgrequest
pub const MC_CMD_CDX_BUS_DOWN_IN_LEN: c_int = 4;
// Bus number to put in reset, in range 0 to BUS_COUNT-1
pub const MC_CMD_CDX_BUS_DOWN_IN_BUS_OFST: c_int = 0;
pub const MC_CMD_CDX_BUS_DOWN_IN_BUS_LEN: c_int = 4;
//
// MC_CMD_CDX_BUS_DOWN_OUT msgresponse: The bus is quiesced, no further
// upstream traffic for devices on this bus.
//
pub const MC_CMD_CDX_BUS_DOWN_OUT_LEN: c_int = 0;
//
// MC_CMD_CDX_BUS_UP
// After bus reset is de-asserted, devices are in a state which is functionally
// equivalent to each device having been reset with MC_CMD_CDX_DEVICE_RESET. In
// other words, device logic is reset in a hardware-specific way, MMIO accesses
// are forwarded to the device, DMA bus mastering is disabled and needs to be
// re-enabled with MC_CMD_CDX_DEVICE_DMA_ENABLE once the driver is ready to
// start servicing DMA. If the underlying number of devices or device resources
// changed (e.g. if PL was reloaded) while the bus was in reset, the bus driver
// is expected to re-enumerate the bus. Returns EALREADY if the bus was already
// up before the call.
//
pub const MC_CMD_CDX_BUS_UP: c_uint = 0x5;
pub const MC_CMD_CDX_BUS_UP_MSGSET: c_uint = 0x5;
// MC_CMD_CDX_BUS_UP_IN msgrequest
pub const MC_CMD_CDX_BUS_UP_IN_LEN: c_int = 4;
// Bus number to take out of reset, in range 0 to BUS_COUNT-1
pub const MC_CMD_CDX_BUS_UP_IN_BUS_OFST: c_int = 0;
pub const MC_CMD_CDX_BUS_UP_IN_BUS_LEN: c_int = 4;
// MC_CMD_CDX_BUS_UP_OUT msgresponse: The bus can now be enumerated.
pub const MC_CMD_CDX_BUS_UP_OUT_LEN: c_int = 0;
//
// MC_CMD_CDX_DEVICE_RESET
// After this call completes, device DMA and interrupts are quiesced, devices
// logic is reset in a hardware-specific way and DMA bus mastering is disabled.
//
pub const MC_CMD_CDX_DEVICE_RESET: c_uint = 0x6;
pub const MC_CMD_CDX_DEVICE_RESET_MSGSET: c_uint = 0x6;

// MC_CMD_CDX_DEVICE_RESET_IN msgrequest
pub const MC_CMD_CDX_DEVICE_RESET_IN_LEN: c_int = 8;
// Device bus number, in range 0 to BUS_COUNT-1
pub const MC_CMD_CDX_DEVICE_RESET_IN_BUS_OFST: c_int = 0;
pub const MC_CMD_CDX_DEVICE_RESET_IN_BUS_LEN: c_int = 4;
// Device number relative to the bus, in range 0 to DEVICE_COUNT-1 for that bus
pub const MC_CMD_CDX_DEVICE_RESET_IN_DEVICE_OFST: c_int = 4;
pub const MC_CMD_CDX_DEVICE_RESET_IN_DEVICE_LEN: c_int = 4;
//
// MC_CMD_CDX_DEVICE_RESET_OUT msgresponse: The device is quiesced and all
// pending device initiated DMA has completed.
//
pub const MC_CMD_CDX_DEVICE_RESET_OUT_LEN: c_int = 0;
//
// MC_CMD_CDX_DEVICE_CONTROL_SET
// If BUS_MASTER is set to disabled, device DMA and interrupts are quiesced.
// Pending DMA requests and MSI interrupts are flushed and no further DMA or
// interrupts are issued after this command returns. If BUS_MASTER is set to
// enabled, device is allowed to initiate DMA. Whether interrupts are enabled
// also depends on the value of MSI_ENABLE bit. Note that, in this case, the
// device may start DMA before the host receives and processes the MCDI
// response. MSI_ENABLE masks or unmasks device interrupts only. Note that for
// interrupts to be delivered to the host, both BUS_MASTER and MSI_ENABLE needs
// to be set. MMIO_REGIONS_ENABLE enables or disables host accesses to device
// MMIO regions. Note that an implementation is allowed to permanently set this
// bit to 1, in which case MC_CMD_CDX_DEVICE_CONTROL_GET will always return 1
// for this bit, regardless of the value set here.
//
pub const MC_CMD_CDX_DEVICE_CONTROL_SET: c_uint = 0x7;
pub const MC_CMD_CDX_DEVICE_CONTROL_SET_MSGSET: c_uint = 0x7;

// MC_CMD_CDX_DEVICE_CONTROL_SET_IN msgrequest
pub const MC_CMD_CDX_DEVICE_CONTROL_SET_IN_LEN: c_int = 12;
// Device bus number, in range 0 to BUS_COUNT-1
pub const MC_CMD_CDX_DEVICE_CONTROL_SET_IN_BUS_OFST: c_int = 0;
pub const MC_CMD_CDX_DEVICE_CONTROL_SET_IN_BUS_LEN: c_int = 4;
// Device number relative to the bus, in range 0 to DEVICE_COUNT-1 for that bus
pub const MC_CMD_CDX_DEVICE_CONTROL_SET_IN_DEVICE_OFST: c_int = 4;
pub const MC_CMD_CDX_DEVICE_CONTROL_SET_IN_DEVICE_LEN: c_int = 4;
pub const MC_CMD_CDX_DEVICE_CONTROL_SET_IN_FLAGS_OFST: c_int = 8;
pub const MC_CMD_CDX_DEVICE_CONTROL_SET_IN_FLAGS_LEN: c_int = 4;
pub const MC_CMD_CDX_DEVICE_CONTROL_SET_IN_BUS_MASTER_ENABLE_OFST: c_int = 8;
pub const MC_CMD_CDX_DEVICE_CONTROL_SET_IN_BUS_MASTER_ENABLE_LBN: c_int = 0;
pub const MC_CMD_CDX_DEVICE_CONTROL_SET_IN_BUS_MASTER_ENABLE_WIDTH: c_int = 1;
pub const MC_CMD_CDX_DEVICE_CONTROL_SET_IN_MSI_ENABLE_OFST: c_int = 8;
pub const MC_CMD_CDX_DEVICE_CONTROL_SET_IN_MSI_ENABLE_LBN: c_int = 1;
pub const MC_CMD_CDX_DEVICE_CONTROL_SET_IN_MSI_ENABLE_WIDTH: c_int = 1;
pub const MC_CMD_CDX_DEVICE_CONTROL_SET_IN_MMIO_REGIONS_ENABLE_OFST: c_int = 8;
pub const MC_CMD_CDX_DEVICE_CONTROL_SET_IN_MMIO_REGIONS_ENABLE_LBN: c_int = 2;
pub const MC_CMD_CDX_DEVICE_CONTROL_SET_IN_MMIO_REGIONS_ENABLE_WIDTH: c_int = 1;
// MC_CMD_CDX_DEVICE_CONTROL_SET_OUT msgresponse
pub const MC_CMD_CDX_DEVICE_CONTROL_SET_OUT_LEN: c_int = 0;
//
// MC_CMD_CDX_DEVICE_CONTROL_GET
// Returns device DMA, interrupt and MMIO region access control bits. See
// MC_CMD_CDX_DEVICE_CONTROL_SET for definition of the available control bits.
//
pub const MC_CMD_CDX_DEVICE_CONTROL_GET: c_uint = 0x8;
pub const MC_CMD_CDX_DEVICE_CONTROL_GET_MSGSET: c_uint = 0x8;

// MC_CMD_CDX_DEVICE_CONTROL_GET_IN msgrequest
pub const MC_CMD_CDX_DEVICE_CONTROL_GET_IN_LEN: c_int = 8;
// Device bus number, in range 0 to BUS_COUNT-1
pub const MC_CMD_CDX_DEVICE_CONTROL_GET_IN_BUS_OFST: c_int = 0;
pub const MC_CMD_CDX_DEVICE_CONTROL_GET_IN_BUS_LEN: c_int = 4;
// Device number relative to the bus, in range 0 to DEVICE_COUNT-1 for that bus
pub const MC_CMD_CDX_DEVICE_CONTROL_GET_IN_DEVICE_OFST: c_int = 4;
pub const MC_CMD_CDX_DEVICE_CONTROL_GET_IN_DEVICE_LEN: c_int = 4;
// MC_CMD_CDX_DEVICE_CONTROL_GET_OUT msgresponse
pub const MC_CMD_CDX_DEVICE_CONTROL_GET_OUT_LEN: c_int = 4;
pub const MC_CMD_CDX_DEVICE_CONTROL_GET_OUT_FLAGS_OFST: c_int = 0;
pub const MC_CMD_CDX_DEVICE_CONTROL_GET_OUT_FLAGS_LEN: c_int = 4;
pub const MC_CMD_CDX_DEVICE_CONTROL_GET_OUT_BUS_MASTER_ENABLE_OFST: c_int = 0;
pub const MC_CMD_CDX_DEVICE_CONTROL_GET_OUT_BUS_MASTER_ENABLE_LBN: c_int = 0;
pub const MC_CMD_CDX_DEVICE_CONTROL_GET_OUT_BUS_MASTER_ENABLE_WIDTH: c_int = 1;
pub const MC_CMD_CDX_DEVICE_CONTROL_GET_OUT_MSI_ENABLE_OFST: c_int = 0;
pub const MC_CMD_CDX_DEVICE_CONTROL_GET_OUT_MSI_ENABLE_LBN: c_int = 1;
pub const MC_CMD_CDX_DEVICE_CONTROL_GET_OUT_MSI_ENABLE_WIDTH: c_int = 1;
pub const MC_CMD_CDX_DEVICE_CONTROL_GET_OUT_MMIO_REGIONS_ENABLE_OFST: c_int = 0;
pub const MC_CMD_CDX_DEVICE_CONTROL_GET_OUT_MMIO_REGIONS_ENABLE_LBN: c_int = 2;
pub const MC_CMD_CDX_DEVICE_CONTROL_GET_OUT_MMIO_REGIONS_ENABLE_WIDTH: c_int = 1;
//
// MC_CMD_CDX_DEVICE_WRITE_MSI_MSG
// Populates the MSI message to be used by the hardware to raise the specified
// interrupt vector. Versal-net implementation specific limitations are that
// only 4 CDX devices with MSI interrupt capability are supported and all
// vectors within a device must use the same write address. The command will
// return EINVAL if any of these limitations is violated.
//
pub const MC_CMD_CDX_DEVICE_WRITE_MSI_MSG: c_uint = 0x9;
pub const MC_CMD_CDX_DEVICE_WRITE_MSI_MSG_MSGSET: c_uint = 0x9;

// MC_CMD_CDX_DEVICE_WRITE_MSI_MSG_IN msgrequest
pub const MC_CMD_CDX_DEVICE_WRITE_MSI_MSG_IN_LEN: c_int = 28;
// Device bus number, in range 0 to BUS_COUNT-1
pub const MC_CMD_CDX_DEVICE_WRITE_MSI_MSG_IN_BUS_OFST: c_int = 0;
pub const MC_CMD_CDX_DEVICE_WRITE_MSI_MSG_IN_BUS_LEN: c_int = 4;
// Device number relative to the bus, in range 0 to DEVICE_COUNT-1 for that bus
pub const MC_CMD_CDX_DEVICE_WRITE_MSI_MSG_IN_DEVICE_OFST: c_int = 4;
pub const MC_CMD_CDX_DEVICE_WRITE_MSI_MSG_IN_DEVICE_LEN: c_int = 4;
//
// Device-relative MSI vector number. Must be < MSI_COUNT reported for the
// device.
//
pub const MC_CMD_CDX_DEVICE_WRITE_MSI_MSG_IN_MSI_VECTOR_OFST: c_int = 8;
pub const MC_CMD_CDX_DEVICE_WRITE_MSI_MSG_IN_MSI_VECTOR_LEN: c_int = 4;
// Reserved (alignment)
pub const MC_CMD_CDX_DEVICE_WRITE_MSI_MSG_IN_RESERVED_OFST: c_int = 12;
pub const MC_CMD_CDX_DEVICE_WRITE_MSI_MSG_IN_RESERVED_LEN: c_int = 4;
//
// MSI address to be used by the hardware. Typically, on ARM systems this
// address is translated by the IOMMU (if enabled) and it is the responsibility
// of the entity managing the IOMMU (APU kernel) to supply the correct IOVA
// here.
//
pub const MC_CMD_CDX_DEVICE_WRITE_MSI_MSG_IN_MSI_ADDRESS_OFST: c_int = 16;
pub const MC_CMD_CDX_DEVICE_WRITE_MSI_MSG_IN_MSI_ADDRESS_LEN: c_int = 8;
pub const MC_CMD_CDX_DEVICE_WRITE_MSI_MSG_IN_MSI_ADDRESS_LO_OFST: c_int = 16;
pub const MC_CMD_CDX_DEVICE_WRITE_MSI_MSG_IN_MSI_ADDRESS_LO_LEN: c_int = 4;
pub const MC_CMD_CDX_DEVICE_WRITE_MSI_MSG_IN_MSI_ADDRESS_LO_LBN: c_int = 128;
pub const MC_CMD_CDX_DEVICE_WRITE_MSI_MSG_IN_MSI_ADDRESS_LO_WIDTH: c_int = 32;
pub const MC_CMD_CDX_DEVICE_WRITE_MSI_MSG_IN_MSI_ADDRESS_HI_OFST: c_int = 20;
pub const MC_CMD_CDX_DEVICE_WRITE_MSI_MSG_IN_MSI_ADDRESS_HI_LEN: c_int = 4;
pub const MC_CMD_CDX_DEVICE_WRITE_MSI_MSG_IN_MSI_ADDRESS_HI_LBN: c_int = 160;
pub const MC_CMD_CDX_DEVICE_WRITE_MSI_MSG_IN_MSI_ADDRESS_HI_WIDTH: c_int = 32;
//
// MSI data to be used by the hardware. On versal-net, only the lower 16-bits
// are used, the remaining bits are ignored and should be set to zero.
//
pub const MC_CMD_CDX_DEVICE_WRITE_MSI_MSG_IN_MSI_DATA_OFST: c_int = 24;
pub const MC_CMD_CDX_DEVICE_WRITE_MSI_MSG_IN_MSI_DATA_LEN: c_int = 4;
// MC_CMD_CDX_DEVICE_WRITE_MSI_MSG_OUT msgresponse
pub const MC_CMD_CDX_DEVICE_WRITE_MSI_MSG_OUT_LEN: c_int = 0;
//
// MC_CMD_V2_EXTN - Encapsulation for a v2 extended command
pub const MC_CMD_V2_EXTN: c_uint = 0x7f;
// MC_CMD_V2_EXTN_IN msgrequest
pub const MC_CMD_V2_EXTN_IN_LEN: c_int = 4;
// the extended command number
pub const MC_CMD_V2_EXTN_IN_EXTENDED_CMD_LBN: c_int = 0;
pub const MC_CMD_V2_EXTN_IN_EXTENDED_CMD_WIDTH: c_int = 15;
pub const MC_CMD_V2_EXTN_IN_UNUSED_LBN: c_int = 15;
pub const MC_CMD_V2_EXTN_IN_UNUSED_WIDTH: c_int = 1;
// the actual length of the encapsulated command
pub const MC_CMD_V2_EXTN_IN_ACTUAL_LEN_LBN: c_int = 16;
pub const MC_CMD_V2_EXTN_IN_ACTUAL_LEN_WIDTH: c_int = 10;
pub const MC_CMD_V2_EXTN_IN_UNUSED2_LBN: c_int = 26;
pub const MC_CMD_V2_EXTN_IN_UNUSED2_WIDTH: c_int = 2;
// Type of command/response
pub const MC_CMD_V2_EXTN_IN_MESSAGE_TYPE_LBN: c_int = 28;
pub const MC_CMD_V2_EXTN_IN_MESSAGE_TYPE_WIDTH: c_int = 4;
//
// enum: MCDI command directed to versal-net. MCDI responses of this type
// are not defined.
//
pub const MC_CMD_V2_EXTN_IN_MCDI_MESSAGE_TYPE_PLATFORM: c_uint = 0x2;
