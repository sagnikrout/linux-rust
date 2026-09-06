//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/siena/mcdi_pcol.h
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
// Driver for Solarflare network controllers and boards
// Copyright 2009-2018 Solarflare Communications Inc.
// Copyright 2019-2020 Xilinx Inc.
//
// Values to be written into FMCR_CZ_RESET_STATE_REG to control boot.
// Power-on reset state

// If this is set in MC_RESET_STATE_REG then it should be
// possible to jump into IMEM without loading code from flash.

// The MC main image has started to boot.

// The Scheduler has started.

// If this is set in MC_RESET_STATE_REG then it should be
// possible to jump into IMEM without loading code from flash.
// Unlike a warm boot, assume DMEM has been reloaded, so that
// the MC persistent data must be reinitialised.

// We have entered the main firmware via recovery mode.  This
// means that MC persistent data must be reinitialised, but that
// we shouldn't touch PCIe config.

// BIST state has been initialized

// Siena MC shared memmory offsets
// The 'doorbell' addresses are hard-wired to alert the MC when written
pub const MC_SMEM_P0_DOORBELL_OFST: c_uint = 0x000;
pub const MC_SMEM_P1_DOORBELL_OFST: c_uint = 0x004;
// The rest of these are firmware-defined
pub const MC_SMEM_P0_PDU_OFST: c_uint = 0x008;
pub const MC_SMEM_P1_PDU_OFST: c_uint = 0x108;
pub const MC_SMEM_PDU_LEN: c_uint = 0x100;
pub const MC_SMEM_P0_PTP_TIME_OFST: c_uint = 0x7f0;
pub const MC_SMEM_P0_STATUS_OFST: c_uint = 0x7f8;
pub const MC_SMEM_P1_STATUS_OFST: c_uint = 0x7fc;
// Values to be written to the per-port status dword in shared
// memory on reboot and assert

// Check whether an mcfw version (in host order) belongs to a bootloader

// The current version of the MCDI protocol.
//
// Note that the ROM burnt into the card only talks V0, so at the very
// least every driver must support version 0 and MCDI_PCOL_VERSION
//
pub const MCDI_PCOL_VERSION: c_int = 2;
// Unused commands: 0x23, 0x27, 0x30, 0x31
// MCDI version 1
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
// The client writes it's request into MC shared memory, and rings the
// doorbell. Each request is completed by either by the MC writing
// back into shared memory, or by writing out an event.
//
// All MCDI commands support completion by shared memory response. Each
// request may also contain additional data (accounted for by HEADER.LEN),
// and some response's may also contain additional data (again, accounted
// for by HEADER.LEN).
//
// Some MCDI commands support completion by event, in which any associated
// response data is included in the event.
//
// The protocol requires one response to be delivered for every request, a
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
pub const MCDI_CTL_SDU_LEN_MAX_V1: c_uint = 0xfc;
pub const MCDI_CTL_SDU_LEN_MAX_V2: c_uint = 0x400;

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
pub const FSE_AZ_EV_CODE_MCDI_EVRESPONSE: c_uint = 0xc;
// Operation not permitted.
pub const MC_CMD_ERR_EPERM: c_int = 1;
// Non-existent command target
pub const MC_CMD_ERR_ENOENT: c_int = 2;
// assert() has killed the MC
pub const MC_CMD_ERR_EINTR: c_int = 4;
// I/O failure
pub const MC_CMD_ERR_EIO: c_int = 5;
// Already exists
pub const MC_CMD_ERR_EEXIST: c_int = 6;
// Try again
pub const MC_CMD_ERR_EAGAIN: c_int = 11;
// Out of memory
pub const MC_CMD_ERR_ENOMEM: c_int = 12;
// Caller does not hold required locks
pub const MC_CMD_ERR_EACCES: c_int = 13;
// Resource is currently unavailable (e.g. lock contention)
pub const MC_CMD_ERR_EBUSY: c_int = 16;
// No such device
pub const MC_CMD_ERR_ENODEV: c_int = 19;
// Invalid argument to target
pub const MC_CMD_ERR_EINVAL: c_int = 22;
// Broken pipe
pub const MC_CMD_ERR_EPIPE: c_int = 32;
// Read-only
pub const MC_CMD_ERR_EROFS: c_int = 30;
// Out of range
pub const MC_CMD_ERR_ERANGE: c_int = 34;
// Non-recursive resource is already acquired
pub const MC_CMD_ERR_EDEADLK: c_int = 35;
// Operation not implemented
pub const MC_CMD_ERR_ENOSYS: c_int = 38;
// Operation timed out
pub const MC_CMD_ERR_ETIME: c_int = 62;
// Link has been severed
pub const MC_CMD_ERR_ENOLINK: c_int = 67;
// Protocol error
pub const MC_CMD_ERR_EPROTO: c_int = 71;
// Operation not supported
pub const MC_CMD_ERR_ENOTSUP: c_int = 95;
// Address not available
pub const MC_CMD_ERR_EADDRNOTAVAIL: c_int = 99;
// Not connected
pub const MC_CMD_ERR_ENOTCONN: c_int = 107;
// Operation already in progress
pub const MC_CMD_ERR_EALREADY: c_int = 114;
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
// The requested operation might require the
pub const MC_CMD_ERR_TRANSPORT_NOPROXY: c_uint = 0x100d;
// VLAN tag(s) exists
pub const MC_CMD_ERR_VLAN_EXIST: c_uint = 0x100e;
// No MAC address assigned to an EVB port
pub const MC_CMD_ERR_NO_MAC_ADDR: c_uint = 0x100f;
// Notifies the driver that the request has been relayed
// to an admin function for authorization. The driver should
// wait for a PROXY_RESPONSE event and then resend its request.
// This error code is followed by a 32-bit handle that
// helps matching it with the respective PROXY_RESPONSE event.
pub const MC_CMD_ERR_PROXY_PENDING: c_uint = 0x1010;
pub const MC_CMD_ERR_PROXY_PENDING_HANDLE_OFST: c_int = 4;
// The request cannot be passed for authorization because
// another request from the same function is currently being
// authorized. The drvier should try again later.
pub const MC_CMD_ERR_PROXY_INPROGRESS: c_uint = 0x1011;
// Returned by MC_CMD_PROXY_COMPLETE if the caller is not the function
// that has enabled proxying or BLOCK_INDEX points to a function that
// doesn't await an authorization.
pub const MC_CMD_ERR_PROXY_UNEXPECTED: c_uint = 0x1012;
// This code is currently only used internally in FW. Its meaning is that
// an operation failed due to lack of SR-IOV privilege.
// Normally it is translated to EPERM by send_cmd_err(),
// but it may also be used to trigger some special mechanism
// for handling such case, e.g. to relay the failed request
// to a designated admin function for authorization.
pub const MC_CMD_ERR_NO_PRIVILEGE: c_uint = 0x1013;
// Workaround 26807 could not be turned on/off because some functions
// have already installed filters. See the comment at
// MC_CMD_WORKAROUND_BUG26807.
// May also returned for other operations such as sub-variant switching.
pub const MC_CMD_ERR_FILTERS_PRESENT: c_uint = 0x1014;
// The clock whose frequency you've attempted to set
// doesn't exist on this NIC
pub const MC_CMD_ERR_NO_CLOCK: c_uint = 0x1015;
// Returned by MC_CMD_TESTASSERT if the action that should
// have caused an assertion failed to do so.
pub const MC_CMD_ERR_UNREACHABLE: c_uint = 0x1016;
// This command needs to be processed in the background but there were no
// resources to do so. Send it again after a command has completed.
pub const MC_CMD_ERR_QUEUE_FULL: c_uint = 0x1017;
// The operation could not be completed because the PCIe link has gone
// away.  This error code is never expected to be returned over the TLP
// transport.
pub const MC_CMD_ERR_NO_PCIE: c_uint = 0x1018;
// The operation could not be completed because the datapath has gone
// away.  This is distinct from MC_CMD_ERR_DATAPATH_DISABLED in that the
// datapath absence may be temporary
pub const MC_CMD_ERR_NO_DATAPATH: c_uint = 0x1019;
// The operation could not complete because some VIs are allocated
pub const MC_CMD_ERR_VIS_PRESENT: c_uint = 0x101a;
// The operation could not complete because some PIO buffers are allocated
pub const MC_CMD_ERR_PIOBUFS_PRESENT: c_uint = 0x101b;
pub const MC_CMD_ERR_CODE_OFST: c_int = 0;
// We define 8 "escape" commands to allow
pub const MC_CMD_CMD_SPACE_ESCAPE_0: c_uint = 0x78;
pub const MC_CMD_CMD_SPACE_ESCAPE_1: c_uint = 0x79;
pub const MC_CMD_CMD_SPACE_ESCAPE_2: c_uint = 0x7A;
pub const MC_CMD_CMD_SPACE_ESCAPE_3: c_uint = 0x7B;
pub const MC_CMD_CMD_SPACE_ESCAPE_4: c_uint = 0x7C;
pub const MC_CMD_CMD_SPACE_ESCAPE_5: c_uint = 0x7D;
pub const MC_CMD_CMD_SPACE_ESCAPE_6: c_uint = 0x7E;
pub const MC_CMD_CMD_SPACE_ESCAPE_7: c_uint = 0x7F;
// Vectors in the boot ROM
// Point to the copycode entry point.

// Points to the recovery mode entry point. Misnamed but kept for compatibility.

// Points to the recovery mode entry point. Same as above, but the right name.

// Points to noflash mode entry point.

// The command set exported by the boot ROM (MCDI v0)

// This may be ORed with an EVB_PORT_ID_xxx constant to pass a non-default
// stack ID (which must be in the range 1-255) along with an EVB port ID.
//

// Version 2 adds an optional argument to error returns: the errno value
// may be followed by the (0-based) number of the first argument that
// could not be processed.
//
pub const MC_CMD_ERR_ARG_OFST: c_int = 4;
// No space
pub const MC_CMD_ERR_ENOSPC: c_int = 28;
// MCDI_EVENT structuredef
pub const MCDI_EVENT_LEN: c_int = 8;
pub const MCDI_EVENT_CONT_LBN: c_int = 32;
pub const MCDI_EVENT_CONT_WIDTH: c_int = 1;
pub const MCDI_EVENT_LEVEL_LBN: c_int = 33;
pub const MCDI_EVENT_LEVEL_WIDTH: c_int = 3;
// enum: Info.
pub const MCDI_EVENT_LEVEL_INFO: c_uint = 0x0;
// enum: Warning.
pub const MCDI_EVENT_LEVEL_WARN: c_uint = 0x1;
// enum: Error.
pub const MCDI_EVENT_LEVEL_ERR: c_uint = 0x2;
// enum: Fatal.
pub const MCDI_EVENT_LEVEL_FATAL: c_uint = 0x3;
pub const MCDI_EVENT_DATA_OFST: c_int = 0;
pub const MCDI_EVENT_DATA_LEN: c_int = 4;
pub const MCDI_EVENT_CMDDONE_SEQ_OFST: c_int = 0;
pub const MCDI_EVENT_CMDDONE_SEQ_LBN: c_int = 0;
pub const MCDI_EVENT_CMDDONE_SEQ_WIDTH: c_int = 8;
pub const MCDI_EVENT_CMDDONE_DATALEN_OFST: c_int = 0;
pub const MCDI_EVENT_CMDDONE_DATALEN_LBN: c_int = 8;
pub const MCDI_EVENT_CMDDONE_DATALEN_WIDTH: c_int = 8;
pub const MCDI_EVENT_CMDDONE_ERRNO_OFST: c_int = 0;
pub const MCDI_EVENT_CMDDONE_ERRNO_LBN: c_int = 16;
pub const MCDI_EVENT_CMDDONE_ERRNO_WIDTH: c_int = 8;
pub const MCDI_EVENT_LINKCHANGE_LP_CAP_OFST: c_int = 0;
pub const MCDI_EVENT_LINKCHANGE_LP_CAP_LBN: c_int = 0;
pub const MCDI_EVENT_LINKCHANGE_LP_CAP_WIDTH: c_int = 16;
pub const MCDI_EVENT_LINKCHANGE_SPEED_OFST: c_int = 0;
pub const MCDI_EVENT_LINKCHANGE_SPEED_LBN: c_int = 16;
pub const MCDI_EVENT_LINKCHANGE_SPEED_WIDTH: c_int = 4;
// enum: Link is down or link speed could not be determined
pub const MCDI_EVENT_LINKCHANGE_SPEED_UNKNOWN: c_uint = 0x0;
// enum: 100Mbs
pub const MCDI_EVENT_LINKCHANGE_SPEED_100M: c_uint = 0x1;
// enum: 1Gbs
pub const MCDI_EVENT_LINKCHANGE_SPEED_1G: c_uint = 0x2;
// enum: 10Gbs
pub const MCDI_EVENT_LINKCHANGE_SPEED_10G: c_uint = 0x3;
// enum: 40Gbs
pub const MCDI_EVENT_LINKCHANGE_SPEED_40G: c_uint = 0x4;
// enum: 25Gbs
pub const MCDI_EVENT_LINKCHANGE_SPEED_25G: c_uint = 0x5;
// enum: 50Gbs
pub const MCDI_EVENT_LINKCHANGE_SPEED_50G: c_uint = 0x6;
// enum: 100Gbs
pub const MCDI_EVENT_LINKCHANGE_SPEED_100G: c_uint = 0x7;
pub const MCDI_EVENT_LINKCHANGE_FCNTL_OFST: c_int = 0;
pub const MCDI_EVENT_LINKCHANGE_FCNTL_LBN: c_int = 20;
pub const MCDI_EVENT_LINKCHANGE_FCNTL_WIDTH: c_int = 4;
pub const MCDI_EVENT_LINKCHANGE_LINK_FLAGS_OFST: c_int = 0;
pub const MCDI_EVENT_LINKCHANGE_LINK_FLAGS_LBN: c_int = 24;
pub const MCDI_EVENT_LINKCHANGE_LINK_FLAGS_WIDTH: c_int = 8;
pub const MCDI_EVENT_SENSOREVT_MONITOR_OFST: c_int = 0;
pub const MCDI_EVENT_SENSOREVT_MONITOR_LBN: c_int = 0;
pub const MCDI_EVENT_SENSOREVT_MONITOR_WIDTH: c_int = 8;
pub const MCDI_EVENT_SENSOREVT_STATE_OFST: c_int = 0;
pub const MCDI_EVENT_SENSOREVT_STATE_LBN: c_int = 8;
pub const MCDI_EVENT_SENSOREVT_STATE_WIDTH: c_int = 8;
pub const MCDI_EVENT_SENSOREVT_VALUE_OFST: c_int = 0;
pub const MCDI_EVENT_SENSOREVT_VALUE_LBN: c_int = 16;
pub const MCDI_EVENT_SENSOREVT_VALUE_WIDTH: c_int = 16;
pub const MCDI_EVENT_FWALERT_DATA_OFST: c_int = 0;
pub const MCDI_EVENT_FWALERT_DATA_LBN: c_int = 8;
pub const MCDI_EVENT_FWALERT_DATA_WIDTH: c_int = 24;
pub const MCDI_EVENT_FWALERT_REASON_OFST: c_int = 0;
pub const MCDI_EVENT_FWALERT_REASON_LBN: c_int = 0;
pub const MCDI_EVENT_FWALERT_REASON_WIDTH: c_int = 8;
// enum: SRAM Access.
pub const MCDI_EVENT_FWALERT_REASON_SRAM_ACCESS: c_uint = 0x1;
pub const MCDI_EVENT_FLR_VF_OFST: c_int = 0;
pub const MCDI_EVENT_FLR_VF_LBN: c_int = 0;
pub const MCDI_EVENT_FLR_VF_WIDTH: c_int = 8;
pub const MCDI_EVENT_TX_ERR_TXQ_OFST: c_int = 0;
pub const MCDI_EVENT_TX_ERR_TXQ_LBN: c_int = 0;
pub const MCDI_EVENT_TX_ERR_TXQ_WIDTH: c_int = 12;
pub const MCDI_EVENT_TX_ERR_TYPE_OFST: c_int = 0;
pub const MCDI_EVENT_TX_ERR_TYPE_LBN: c_int = 12;
pub const MCDI_EVENT_TX_ERR_TYPE_WIDTH: c_int = 4;
// enum: Descriptor loader reported failure
pub const MCDI_EVENT_TX_ERR_DL_FAIL: c_uint = 0x1;
// enum: Descriptor ring empty and no EOP seen for packet
pub const MCDI_EVENT_TX_ERR_NO_EOP: c_uint = 0x2;
// enum: Overlength packet
pub const MCDI_EVENT_TX_ERR_2BIG: c_uint = 0x3;
// enum: Malformed option descriptor
pub const MCDI_EVENT_TX_BAD_OPTDESC: c_uint = 0x5;
// enum: Option descriptor part way through a packet
pub const MCDI_EVENT_TX_OPT_IN_PKT: c_uint = 0x8;
// enum: DMA or PIO data access error
pub const MCDI_EVENT_TX_ERR_BAD_DMA_OR_PIO: c_uint = 0x9;
pub const MCDI_EVENT_TX_ERR_INFO_OFST: c_int = 0;
pub const MCDI_EVENT_TX_ERR_INFO_LBN: c_int = 16;
pub const MCDI_EVENT_TX_ERR_INFO_WIDTH: c_int = 16;
pub const MCDI_EVENT_TX_FLUSH_TO_DRIVER_OFST: c_int = 0;
pub const MCDI_EVENT_TX_FLUSH_TO_DRIVER_LBN: c_int = 12;
pub const MCDI_EVENT_TX_FLUSH_TO_DRIVER_WIDTH: c_int = 1;
pub const MCDI_EVENT_TX_FLUSH_TXQ_OFST: c_int = 0;
pub const MCDI_EVENT_TX_FLUSH_TXQ_LBN: c_int = 0;
pub const MCDI_EVENT_TX_FLUSH_TXQ_WIDTH: c_int = 12;
pub const MCDI_EVENT_PTP_ERR_TYPE_OFST: c_int = 0;
pub const MCDI_EVENT_PTP_ERR_TYPE_LBN: c_int = 0;
pub const MCDI_EVENT_PTP_ERR_TYPE_WIDTH: c_int = 8;
// enum: PLL lost lock
pub const MCDI_EVENT_PTP_ERR_PLL_LOST: c_uint = 0x1;
// enum: Filter overflow (PDMA)
pub const MCDI_EVENT_PTP_ERR_FILTER: c_uint = 0x2;
// enum: FIFO overflow (FPGA)
pub const MCDI_EVENT_PTP_ERR_FIFO: c_uint = 0x3;
// enum: Merge queue overflow
pub const MCDI_EVENT_PTP_ERR_QUEUE: c_uint = 0x4;
pub const MCDI_EVENT_AOE_ERR_TYPE_OFST: c_int = 0;
pub const MCDI_EVENT_AOE_ERR_TYPE_LBN: c_int = 0;
pub const MCDI_EVENT_AOE_ERR_TYPE_WIDTH: c_int = 8;
// enum: AOE failed to load - no valid image?
pub const MCDI_EVENT_AOE_NO_LOAD: c_uint = 0x1;
// enum: AOE FC reported an exception
pub const MCDI_EVENT_AOE_FC_ASSERT: c_uint = 0x2;
// enum: AOE FC watchdogged
pub const MCDI_EVENT_AOE_FC_WATCHDOG: c_uint = 0x3;
// enum: AOE FC failed to start
pub const MCDI_EVENT_AOE_FC_NO_START: c_uint = 0x4;
// enum: Generic AOE fault - likely to have been reported via other means too
// but intended for use by aoex driver.
//
pub const MCDI_EVENT_AOE_FAULT: c_uint = 0x5;
// enum: Results of reprogramming the CPLD (status in AOE_ERR_DATA)
pub const MCDI_EVENT_AOE_CPLD_REPROGRAMMED: c_uint = 0x6;
// enum: AOE loaded successfully
pub const MCDI_EVENT_AOE_LOAD: c_uint = 0x7;
// enum: AOE DMA operation completed (LSB of HOST_HANDLE in AOE_ERR_DATA)
pub const MCDI_EVENT_AOE_DMA: c_uint = 0x8;
// enum: AOE byteblaster connected/disconnected (Connection status in
// AOE_ERR_DATA)
//
pub const MCDI_EVENT_AOE_BYTEBLASTER: c_uint = 0x9;
// enum: DDR ECC status update
pub const MCDI_EVENT_AOE_DDR_ECC_STATUS: c_uint = 0xa;
// enum: PTP status update
pub const MCDI_EVENT_AOE_PTP_STATUS: c_uint = 0xb;
// enum: FPGA header incorrect
pub const MCDI_EVENT_AOE_FPGA_LOAD_HEADER_ERR: c_uint = 0xc;
// enum: FPGA Powered Off due to error in powering up FPGA
pub const MCDI_EVENT_AOE_FPGA_POWER_OFF: c_uint = 0xd;
// enum: AOE FPGA load failed due to MC to MUM communication failure
pub const MCDI_EVENT_AOE_FPGA_LOAD_FAILED: c_uint = 0xe;
// enum: Notify that invalid flash type detected
pub const MCDI_EVENT_AOE_INVALID_FPGA_FLASH_TYPE: c_uint = 0xf;
// enum: Notify that the attempt to run FPGA Controller firmware timedout
pub const MCDI_EVENT_AOE_FC_RUN_TIMEDOUT: c_uint = 0x10;
// enum: Failure to probe one or more FPGA boot flash chips
pub const MCDI_EVENT_AOE_FPGA_BOOT_FLASH_INVALID: c_uint = 0x11;
// enum: FPGA boot-flash contains an invalid image header
pub const MCDI_EVENT_AOE_FPGA_BOOT_FLASH_HDR_INVALID: c_uint = 0x12;
// enum: Failed to program clocks required by the FPGA
pub const MCDI_EVENT_AOE_FPGA_CLOCKS_PROGRAM_FAILED: c_uint = 0x13;
// enum: Notify that FPGA Controller is alive to serve MCDI requests
pub const MCDI_EVENT_AOE_FC_RUNNING: c_uint = 0x14;
pub const MCDI_EVENT_AOE_ERR_DATA_OFST: c_int = 0;
pub const MCDI_EVENT_AOE_ERR_DATA_LBN: c_int = 8;
pub const MCDI_EVENT_AOE_ERR_DATA_WIDTH: c_int = 8;
pub const MCDI_EVENT_AOE_ERR_FC_ASSERT_INFO_OFST: c_int = 0;
pub const MCDI_EVENT_AOE_ERR_FC_ASSERT_INFO_LBN: c_int = 8;
pub const MCDI_EVENT_AOE_ERR_FC_ASSERT_INFO_WIDTH: c_int = 8;
// enum: FC Assert happened, but the register information is not available
pub const MCDI_EVENT_AOE_ERR_FC_ASSERT_SEEN: c_uint = 0x0;
// enum: The register information for FC Assert is ready for readinng by driver
//
pub const MCDI_EVENT_AOE_ERR_FC_ASSERT_DATA_READY: c_uint = 0x1;
pub const MCDI_EVENT_AOE_ERR_CODE_FPGA_HEADER_VERIFY_FAILED_OFST: c_int = 0;
pub const MCDI_EVENT_AOE_ERR_CODE_FPGA_HEADER_VERIFY_FAILED_LBN: c_int = 8;
pub const MCDI_EVENT_AOE_ERR_CODE_FPGA_HEADER_VERIFY_FAILED_WIDTH: c_int = 8;
// enum: Reading from NV failed
pub const MCDI_EVENT_AOE_ERR_FPGA_HEADER_NV_READ_FAIL: c_uint = 0x0;
// enum: Invalid Magic Number if FPGA header
pub const MCDI_EVENT_AOE_ERR_FPGA_HEADER_MAGIC_FAIL: c_uint = 0x1;
// enum: Invalid Silicon type detected in header
pub const MCDI_EVENT_AOE_ERR_FPGA_HEADER_SILICON_TYPE: c_uint = 0x2;
// enum: Unsupported VRatio
pub const MCDI_EVENT_AOE_ERR_FPGA_HEADER_VRATIO: c_uint = 0x3;
// enum: Unsupported DDR Type
pub const MCDI_EVENT_AOE_ERR_FPGA_HEADER_DDR_TYPE: c_uint = 0x4;
// enum: DDR Voltage out of supported range
pub const MCDI_EVENT_AOE_ERR_FPGA_HEADER_DDR_VOLTAGE: c_uint = 0x5;
// enum: Unsupported DDR speed
pub const MCDI_EVENT_AOE_ERR_FPGA_HEADER_DDR_SPEED: c_uint = 0x6;
// enum: Unsupported DDR size
pub const MCDI_EVENT_AOE_ERR_FPGA_HEADER_DDR_SIZE: c_uint = 0x7;
// enum: Unsupported DDR rank
pub const MCDI_EVENT_AOE_ERR_FPGA_HEADER_DDR_RANK: c_uint = 0x8;
pub const MCDI_EVENT_AOE_ERR_CODE_INVALID_FPGA_FLASH_TYPE_INFO_OFST: c_int = 0;
pub const MCDI_EVENT_AOE_ERR_CODE_INVALID_FPGA_FLASH_TYPE_INFO_LBN: c_int = 8;
pub const MCDI_EVENT_AOE_ERR_CODE_INVALID_FPGA_FLASH_TYPE_INFO_WIDTH: c_int = 8;
// enum: Primary boot flash
pub const MCDI_EVENT_AOE_FLASH_TYPE_BOOT_PRIMARY: c_uint = 0x0;
// enum: Secondary boot flash
pub const MCDI_EVENT_AOE_FLASH_TYPE_BOOT_SECONDARY: c_uint = 0x1;
pub const MCDI_EVENT_AOE_ERR_CODE_FPGA_POWER_OFF_OFST: c_int = 0;
pub const MCDI_EVENT_AOE_ERR_CODE_FPGA_POWER_OFF_LBN: c_int = 8;
pub const MCDI_EVENT_AOE_ERR_CODE_FPGA_POWER_OFF_WIDTH: c_int = 8;
pub const MCDI_EVENT_AOE_ERR_CODE_FPGA_LOAD_FAILED_OFST: c_int = 0;
pub const MCDI_EVENT_AOE_ERR_CODE_FPGA_LOAD_FAILED_LBN: c_int = 8;
pub const MCDI_EVENT_AOE_ERR_CODE_FPGA_LOAD_FAILED_WIDTH: c_int = 8;
pub const MCDI_EVENT_RX_ERR_RXQ_OFST: c_int = 0;
pub const MCDI_EVENT_RX_ERR_RXQ_LBN: c_int = 0;
pub const MCDI_EVENT_RX_ERR_RXQ_WIDTH: c_int = 12;
pub const MCDI_EVENT_RX_ERR_TYPE_OFST: c_int = 0;
pub const MCDI_EVENT_RX_ERR_TYPE_LBN: c_int = 12;
pub const MCDI_EVENT_RX_ERR_TYPE_WIDTH: c_int = 4;
pub const MCDI_EVENT_RX_ERR_INFO_OFST: c_int = 0;
pub const MCDI_EVENT_RX_ERR_INFO_LBN: c_int = 16;
pub const MCDI_EVENT_RX_ERR_INFO_WIDTH: c_int = 16;
pub const MCDI_EVENT_RX_FLUSH_TO_DRIVER_OFST: c_int = 0;
pub const MCDI_EVENT_RX_FLUSH_TO_DRIVER_LBN: c_int = 12;
pub const MCDI_EVENT_RX_FLUSH_TO_DRIVER_WIDTH: c_int = 1;
pub const MCDI_EVENT_RX_FLUSH_RXQ_OFST: c_int = 0;
pub const MCDI_EVENT_RX_FLUSH_RXQ_LBN: c_int = 0;
pub const MCDI_EVENT_RX_FLUSH_RXQ_WIDTH: c_int = 12;
pub const MCDI_EVENT_MC_REBOOT_COUNT_OFST: c_int = 0;
pub const MCDI_EVENT_MC_REBOOT_COUNT_LBN: c_int = 0;
pub const MCDI_EVENT_MC_REBOOT_COUNT_WIDTH: c_int = 16;
pub const MCDI_EVENT_MUM_ERR_TYPE_OFST: c_int = 0;
pub const MCDI_EVENT_MUM_ERR_TYPE_LBN: c_int = 0;
pub const MCDI_EVENT_MUM_ERR_TYPE_WIDTH: c_int = 8;
// enum: MUM failed to load - no valid image?
pub const MCDI_EVENT_MUM_NO_LOAD: c_uint = 0x1;
// enum: MUM f/w reported an exception
pub const MCDI_EVENT_MUM_ASSERT: c_uint = 0x2;
// enum: MUM not kicking watchdog
pub const MCDI_EVENT_MUM_WATCHDOG: c_uint = 0x3;
pub const MCDI_EVENT_MUM_ERR_DATA_OFST: c_int = 0;
pub const MCDI_EVENT_MUM_ERR_DATA_LBN: c_int = 8;
pub const MCDI_EVENT_MUM_ERR_DATA_WIDTH: c_int = 8;
pub const MCDI_EVENT_DBRET_SEQ_OFST: c_int = 0;
pub const MCDI_EVENT_DBRET_SEQ_LBN: c_int = 0;
pub const MCDI_EVENT_DBRET_SEQ_WIDTH: c_int = 8;
pub const MCDI_EVENT_SUC_ERR_TYPE_OFST: c_int = 0;
pub const MCDI_EVENT_SUC_ERR_TYPE_LBN: c_int = 0;
pub const MCDI_EVENT_SUC_ERR_TYPE_WIDTH: c_int = 8;
// enum: Corrupted or bad SUC application.
pub const MCDI_EVENT_SUC_BAD_APP: c_uint = 0x1;
// enum: SUC application reported an assert.
pub const MCDI_EVENT_SUC_ASSERT: c_uint = 0x2;
// enum: SUC application reported an exception.
pub const MCDI_EVENT_SUC_EXCEPTION: c_uint = 0x3;
// enum: SUC watchdog timer expired.
pub const MCDI_EVENT_SUC_WATCHDOG: c_uint = 0x4;
pub const MCDI_EVENT_SUC_ERR_ADDRESS_OFST: c_int = 0;
pub const MCDI_EVENT_SUC_ERR_ADDRESS_LBN: c_int = 8;
pub const MCDI_EVENT_SUC_ERR_ADDRESS_WIDTH: c_int = 24;
pub const MCDI_EVENT_SUC_ERR_DATA_OFST: c_int = 0;
pub const MCDI_EVENT_SUC_ERR_DATA_LBN: c_int = 8;
pub const MCDI_EVENT_SUC_ERR_DATA_WIDTH: c_int = 24;
pub const MCDI_EVENT_LINKCHANGE_V2_LP_CAP_OFST: c_int = 0;
pub const MCDI_EVENT_LINKCHANGE_V2_LP_CAP_LBN: c_int = 0;
pub const MCDI_EVENT_LINKCHANGE_V2_LP_CAP_WIDTH: c_int = 24;
pub const MCDI_EVENT_LINKCHANGE_V2_SPEED_OFST: c_int = 0;
pub const MCDI_EVENT_LINKCHANGE_V2_SPEED_LBN: c_int = 24;
pub const MCDI_EVENT_LINKCHANGE_V2_SPEED_WIDTH: c_int = 4;
// Enum values, see field(s):
// MCDI_EVENT/LINKCHANGE_SPEED
pub const MCDI_EVENT_LINKCHANGE_V2_FLAGS_LINK_UP_OFST: c_int = 0;
pub const MCDI_EVENT_LINKCHANGE_V2_FLAGS_LINK_UP_LBN: c_int = 28;
pub const MCDI_EVENT_LINKCHANGE_V2_FLAGS_LINK_UP_WIDTH: c_int = 1;
pub const MCDI_EVENT_LINKCHANGE_V2_FCNTL_OFST: c_int = 0;
pub const MCDI_EVENT_LINKCHANGE_V2_FCNTL_LBN: c_int = 29;
pub const MCDI_EVENT_LINKCHANGE_V2_FCNTL_WIDTH: c_int = 3;
// Enum values, see field(s):
// MC_CMD_SET_MAC/MC_CMD_SET_MAC_IN/FCNTL
pub const MCDI_EVENT_MODULECHANGE_LD_CAP_OFST: c_int = 0;
pub const MCDI_EVENT_MODULECHANGE_LD_CAP_LBN: c_int = 0;
pub const MCDI_EVENT_MODULECHANGE_LD_CAP_WIDTH: c_int = 30;
pub const MCDI_EVENT_MODULECHANGE_SEQ_OFST: c_int = 0;
pub const MCDI_EVENT_MODULECHANGE_SEQ_LBN: c_int = 30;
pub const MCDI_EVENT_MODULECHANGE_SEQ_WIDTH: c_int = 2;
pub const MCDI_EVENT_DATA_LBN: c_int = 0;
pub const MCDI_EVENT_DATA_WIDTH: c_int = 32;
// Alias for PTP_DATA.
pub const MCDI_EVENT_SRC_LBN: c_int = 36;
pub const MCDI_EVENT_SRC_WIDTH: c_int = 8;
// Data associated with PTP events which doesn't fit into the main DATA field
//
pub const MCDI_EVENT_PTP_DATA_LBN: c_int = 36;
pub const MCDI_EVENT_PTP_DATA_WIDTH: c_int = 8;
// EF100 specific. Defined by QDMA. The phase bit, changes each time round the
// event ring
//
pub const MCDI_EVENT_EV_EVQ_PHASE_LBN: c_int = 59;
pub const MCDI_EVENT_EV_EVQ_PHASE_WIDTH: c_int = 1;
pub const MCDI_EVENT_EV_CODE_LBN: c_int = 60;
pub const MCDI_EVENT_EV_CODE_WIDTH: c_int = 4;
pub const MCDI_EVENT_CODE_LBN: c_int = 44;
pub const MCDI_EVENT_CODE_WIDTH: c_int = 8;
// enum: Event generated by host software
pub const MCDI_EVENT_SW_EVENT: c_uint = 0x0;
// enum: Bad assert.
pub const MCDI_EVENT_CODE_BADSSERT: c_uint = 0x1;
// enum: PM Notice.
pub const MCDI_EVENT_CODE_PMNOTICE: c_uint = 0x2;
// enum: Command done.
pub const MCDI_EVENT_CODE_CMDDONE: c_uint = 0x3;
// enum: Link change.
pub const MCDI_EVENT_CODE_LINKCHANGE: c_uint = 0x4;
// enum: Sensor Event.
pub const MCDI_EVENT_CODE_SENSOREVT: c_uint = 0x5;
// enum: Schedule error.
pub const MCDI_EVENT_CODE_SCHEDERR: c_uint = 0x6;
// enum: Reboot.
pub const MCDI_EVENT_CODE_REBOOT: c_uint = 0x7;
// enum: Mac stats DMA.
pub const MCDI_EVENT_CODE_MAC_STATS_DMA: c_uint = 0x8;
// enum: Firmware alert.
pub const MCDI_EVENT_CODE_FWALERT: c_uint = 0x9;
// enum: Function level reset.
pub const MCDI_EVENT_CODE_FLR: c_uint = 0xa;
// enum: Transmit error
pub const MCDI_EVENT_CODE_TX_ERR: c_uint = 0xb;
// enum: Tx flush has completed
pub const MCDI_EVENT_CODE_TX_FLUSH: c_uint = 0xc;
// enum: PTP packet received timestamp
pub const MCDI_EVENT_CODE_PTP_RX: c_uint = 0xd;
// enum: PTP NIC failure
pub const MCDI_EVENT_CODE_PTP_FAULT: c_uint = 0xe;
// enum: PTP PPS event
pub const MCDI_EVENT_CODE_PTP_PPS: c_uint = 0xf;
// enum: Rx flush has completed
pub const MCDI_EVENT_CODE_RX_FLUSH: c_uint = 0x10;
// enum: Receive error
pub const MCDI_EVENT_CODE_RX_ERR: c_uint = 0x11;
// enum: AOE fault
pub const MCDI_EVENT_CODE_AOE: c_uint = 0x12;
// enum: Network port calibration failed (VCAL).
pub const MCDI_EVENT_CODE_VCAL_FAIL: c_uint = 0x13;
// enum: HW PPS event
pub const MCDI_EVENT_CODE_HW_PPS: c_uint = 0x14;
// enum: The MC has rebooted (huntington and later, siena uses CODE_REBOOT and
// a different format)
//
pub const MCDI_EVENT_CODE_MC_REBOOT: c_uint = 0x15;
// enum: the MC has detected a parity error
pub const MCDI_EVENT_CODE_PAR_ERR: c_uint = 0x16;
// enum: the MC has detected a correctable error
pub const MCDI_EVENT_CODE_ECC_CORR_ERR: c_uint = 0x17;
// enum: the MC has detected an uncorrectable error
pub const MCDI_EVENT_CODE_ECC_FATAL_ERR: c_uint = 0x18;
// enum: The MC has entered offline BIST mode
pub const MCDI_EVENT_CODE_MC_BIST: c_uint = 0x19;
// enum: PTP tick event providing current NIC time
pub const MCDI_EVENT_CODE_PTP_TIME: c_uint = 0x1a;
// enum: MUM fault
pub const MCDI_EVENT_CODE_MUM: c_uint = 0x1b;
// enum: notify the designated PF of a new authorization request
pub const MCDI_EVENT_CODE_PROXY_REQUEST: c_uint = 0x1c;
// enum: notify a function that awaits an authorization that its request has
// been processed and it may now resend the command
//
pub const MCDI_EVENT_CODE_PROXY_RESPONSE: c_uint = 0x1d;
// enum: MCDI command accepted. New commands can be issued but this command is
// not done yet.
//
pub const MCDI_EVENT_CODE_DBRET: c_uint = 0x1e;
// enum: The MC has detected a fault on the SUC
pub const MCDI_EVENT_CODE_SUC: c_uint = 0x1f;
// enum: Link change. This event is sent instead of LINKCHANGE if
// WANT_V2_LINKCHANGES was set on driver attach.
//
pub const MCDI_EVENT_CODE_LINKCHANGE_V2: c_uint = 0x20;
// enum: This event is sent if WANT_V2_LINKCHANGES was set on driver attach
// when the local device capabilities changes. This will usually correspond to
// a module change.
//
pub const MCDI_EVENT_CODE_MODULECHANGE: c_uint = 0x21;
// enum: Notification that the sensors have been added and/or removed from the
// sensor table. This event includes the new sensor table generation count, if
// this does not match the driver's local copy it is expected to call
// DYNAMIC_SENSORS_LIST to refresh it.
//
pub const MCDI_EVENT_CODE_DYNAMIC_SENSORS_CHANGE: c_uint = 0x22;
// enum: Notification that a sensor has changed state as a result of a reading
// crossing a threshold. This is sent as two events, the first event contains
// the handle and the sensor's state (in the SRC field), and the second
// contains the value.
//
pub const MCDI_EVENT_CODE_DYNAMIC_SENSORS_STATE_CHANGE: c_uint = 0x23;
// enum: Notification that a descriptor proxy function configuration has been
// pushed to "live" status (visible to host). SRC field contains the handle of
// the affected descriptor proxy function. DATA field contains the generation
// count of configuration set applied. See MC_CMD_DESC_PROXY_FUNC_CONFIG_SET
// MC_CMD_DESC_PROXY_FUNC_CONFIG_COMMIT and SF-122927-TC for details.
//
pub const MCDI_EVENT_CODE_DESC_PROXY_FUNC_CONFIG_COMMITTED: c_uint = 0x24;
// enum: Notification that a descriptor proxy function has been reset. SRC
// field contains the handle of the affected descriptor proxy function. See
// SF-122927-TC for details.
//
pub const MCDI_EVENT_CODE_DESC_PROXY_FUNC_RESET: c_uint = 0x25;
// enum: Notification that a driver attached to a descriptor proxy function.
// SRC field contains the handle of the affected descriptor proxy function. For
// Virtio proxy functions this message consists of two MCDI events, where the
// first event's (CONT=1) DATA field carries negotiated virtio feature bits 0
// to 31 and the second (CONT=0) carries bits 32 to 63. For EF100 proxy
// functions event length and meaning of DATA field is not yet defined. See
// SF-122927-TC for details.
//
pub const MCDI_EVENT_CODE_DESC_PROXY_FUNC_DRIVER_ATTACH: c_uint = 0x26;
// enum: Artificial event generated by host and posted via MC for test
// purposes.
//
pub const MCDI_EVENT_CODE_TESTGEN: c_uint = 0xfa;
pub const MCDI_EVENT_CMDDONE_DATA_OFST: c_int = 0;
pub const MCDI_EVENT_CMDDONE_DATA_LEN: c_int = 4;
pub const MCDI_EVENT_CMDDONE_DATA_LBN: c_int = 0;
pub const MCDI_EVENT_CMDDONE_DATA_WIDTH: c_int = 32;
pub const MCDI_EVENT_LINKCHANGE_DATA_OFST: c_int = 0;
pub const MCDI_EVENT_LINKCHANGE_DATA_LEN: c_int = 4;
pub const MCDI_EVENT_LINKCHANGE_DATA_LBN: c_int = 0;
pub const MCDI_EVENT_LINKCHANGE_DATA_WIDTH: c_int = 32;
pub const MCDI_EVENT_SENSOREVT_DATA_OFST: c_int = 0;
pub const MCDI_EVENT_SENSOREVT_DATA_LEN: c_int = 4;
pub const MCDI_EVENT_SENSOREVT_DATA_LBN: c_int = 0;
pub const MCDI_EVENT_SENSOREVT_DATA_WIDTH: c_int = 32;
pub const MCDI_EVENT_MAC_STATS_DMA_GENERATION_OFST: c_int = 0;
pub const MCDI_EVENT_MAC_STATS_DMA_GENERATION_LEN: c_int = 4;
pub const MCDI_EVENT_MAC_STATS_DMA_GENERATION_LBN: c_int = 0;
pub const MCDI_EVENT_MAC_STATS_DMA_GENERATION_WIDTH: c_int = 32;
pub const MCDI_EVENT_TX_ERR_DATA_OFST: c_int = 0;
pub const MCDI_EVENT_TX_ERR_DATA_LEN: c_int = 4;
pub const MCDI_EVENT_TX_ERR_DATA_LBN: c_int = 0;
pub const MCDI_EVENT_TX_ERR_DATA_WIDTH: c_int = 32;
// For CODE_PTP_RX, CODE_PTP_PPS and CODE_HW_PPS events the seconds field of
// timestamp
//
pub const MCDI_EVENT_PTP_SECONDS_OFST: c_int = 0;
pub const MCDI_EVENT_PTP_SECONDS_LEN: c_int = 4;
pub const MCDI_EVENT_PTP_SECONDS_LBN: c_int = 0;
pub const MCDI_EVENT_PTP_SECONDS_WIDTH: c_int = 32;
// For CODE_PTP_RX, CODE_PTP_PPS and CODE_HW_PPS events the major field of
// timestamp
//
pub const MCDI_EVENT_PTP_MAJOR_OFST: c_int = 0;
pub const MCDI_EVENT_PTP_MAJOR_LEN: c_int = 4;
pub const MCDI_EVENT_PTP_MAJOR_LBN: c_int = 0;
pub const MCDI_EVENT_PTP_MAJOR_WIDTH: c_int = 32;
// For CODE_PTP_RX, CODE_PTP_PPS and CODE_HW_PPS events the nanoseconds field
// of timestamp
//
pub const MCDI_EVENT_PTP_NANOSECONDS_OFST: c_int = 0;
pub const MCDI_EVENT_PTP_NANOSECONDS_LEN: c_int = 4;
pub const MCDI_EVENT_PTP_NANOSECONDS_LBN: c_int = 0;
pub const MCDI_EVENT_PTP_NANOSECONDS_WIDTH: c_int = 32;
// For CODE_PTP_RX, CODE_PTP_PPS and CODE_HW_PPS events the minor field of
// timestamp
//
pub const MCDI_EVENT_PTP_MINOR_OFST: c_int = 0;
pub const MCDI_EVENT_PTP_MINOR_LEN: c_int = 4;
pub const MCDI_EVENT_PTP_MINOR_LBN: c_int = 0;
pub const MCDI_EVENT_PTP_MINOR_WIDTH: c_int = 32;
// For CODE_PTP_RX events, the lowest four bytes of sourceUUID from PTP packet
//
pub const MCDI_EVENT_PTP_UUID_OFST: c_int = 0;
pub const MCDI_EVENT_PTP_UUID_LEN: c_int = 4;
pub const MCDI_EVENT_PTP_UUID_LBN: c_int = 0;
pub const MCDI_EVENT_PTP_UUID_WIDTH: c_int = 32;
pub const MCDI_EVENT_RX_ERR_DATA_OFST: c_int = 0;
pub const MCDI_EVENT_RX_ERR_DATA_LEN: c_int = 4;
pub const MCDI_EVENT_RX_ERR_DATA_LBN: c_int = 0;
pub const MCDI_EVENT_RX_ERR_DATA_WIDTH: c_int = 32;
pub const MCDI_EVENT_PAR_ERR_DATA_OFST: c_int = 0;
pub const MCDI_EVENT_PAR_ERR_DATA_LEN: c_int = 4;
pub const MCDI_EVENT_PAR_ERR_DATA_LBN: c_int = 0;
pub const MCDI_EVENT_PAR_ERR_DATA_WIDTH: c_int = 32;
pub const MCDI_EVENT_ECC_CORR_ERR_DATA_OFST: c_int = 0;
pub const MCDI_EVENT_ECC_CORR_ERR_DATA_LEN: c_int = 4;
pub const MCDI_EVENT_ECC_CORR_ERR_DATA_LBN: c_int = 0;
pub const MCDI_EVENT_ECC_CORR_ERR_DATA_WIDTH: c_int = 32;
pub const MCDI_EVENT_ECC_FATAL_ERR_DATA_OFST: c_int = 0;
pub const MCDI_EVENT_ECC_FATAL_ERR_DATA_LEN: c_int = 4;
pub const MCDI_EVENT_ECC_FATAL_ERR_DATA_LBN: c_int = 0;
pub const MCDI_EVENT_ECC_FATAL_ERR_DATA_WIDTH: c_int = 32;
// For CODE_PTP_TIME events, the major value of the PTP clock
pub const MCDI_EVENT_PTP_TIME_MAJOR_OFST: c_int = 0;
pub const MCDI_EVENT_PTP_TIME_MAJOR_LEN: c_int = 4;
pub const MCDI_EVENT_PTP_TIME_MAJOR_LBN: c_int = 0;
pub const MCDI_EVENT_PTP_TIME_MAJOR_WIDTH: c_int = 32;
// For CODE_PTP_TIME events, bits 19-26 of the minor value of the PTP clock
pub const MCDI_EVENT_PTP_TIME_MINOR_26_19_LBN: c_int = 36;
pub const MCDI_EVENT_PTP_TIME_MINOR_26_19_WIDTH: c_int = 8;
// For CODE_PTP_TIME events, most significant bits of the minor value of the
// PTP clock. This is a more generic equivalent of PTP_TIME_MINOR_26_19.
//
pub const MCDI_EVENT_PTP_TIME_MINOR_MS_8BITS_LBN: c_int = 36;
pub const MCDI_EVENT_PTP_TIME_MINOR_MS_8BITS_WIDTH: c_int = 8;
// For CODE_PTP_TIME events where report sync status is enabled, indicates
// whether the NIC clock has ever been set
//
pub const MCDI_EVENT_PTP_TIME_NIC_CLOCK_VALID_LBN: c_int = 36;
pub const MCDI_EVENT_PTP_TIME_NIC_CLOCK_VALID_WIDTH: c_int = 1;
// For CODE_PTP_TIME events where report sync status is enabled, indicates
// whether the NIC and System clocks are in sync
//
pub const MCDI_EVENT_PTP_TIME_HOST_NIC_IN_SYNC_LBN: c_int = 37;
pub const MCDI_EVENT_PTP_TIME_HOST_NIC_IN_SYNC_WIDTH: c_int = 1;
// For CODE_PTP_TIME events where report sync status is enabled, bits 21-26 of
// the minor value of the PTP clock
//
pub const MCDI_EVENT_PTP_TIME_MINOR_26_21_LBN: c_int = 38;
pub const MCDI_EVENT_PTP_TIME_MINOR_26_21_WIDTH: c_int = 6;
// For CODE_PTP_TIME events, most significant bits of the minor value of the
// PTP clock. This is a more generic equivalent of PTP_TIME_MINOR_26_21.
//
pub const MCDI_EVENT_PTP_TIME_MINOR_MS_6BITS_LBN: c_int = 38;
pub const MCDI_EVENT_PTP_TIME_MINOR_MS_6BITS_WIDTH: c_int = 6;
pub const MCDI_EVENT_PROXY_REQUEST_BUFF_INDEX_OFST: c_int = 0;
pub const MCDI_EVENT_PROXY_REQUEST_BUFF_INDEX_LEN: c_int = 4;
pub const MCDI_EVENT_PROXY_REQUEST_BUFF_INDEX_LBN: c_int = 0;
pub const MCDI_EVENT_PROXY_REQUEST_BUFF_INDEX_WIDTH: c_int = 32;
pub const MCDI_EVENT_PROXY_RESPONSE_HANDLE_OFST: c_int = 0;
pub const MCDI_EVENT_PROXY_RESPONSE_HANDLE_LEN: c_int = 4;
pub const MCDI_EVENT_PROXY_RESPONSE_HANDLE_LBN: c_int = 0;
pub const MCDI_EVENT_PROXY_RESPONSE_HANDLE_WIDTH: c_int = 32;
// Zero means that the request has been completed or authorized, and the driver
// should resend it. A non-zero value means that the authorization has been
// denied, and gives the reason. Typically it will be EPERM.
//
pub const MCDI_EVENT_PROXY_RESPONSE_RC_LBN: c_int = 36;
pub const MCDI_EVENT_PROXY_RESPONSE_RC_WIDTH: c_int = 8;
pub const MCDI_EVENT_DBRET_DATA_OFST: c_int = 0;
pub const MCDI_EVENT_DBRET_DATA_LEN: c_int = 4;
pub const MCDI_EVENT_DBRET_DATA_LBN: c_int = 0;
pub const MCDI_EVENT_DBRET_DATA_WIDTH: c_int = 32;
pub const MCDI_EVENT_LINKCHANGE_V2_DATA_OFST: c_int = 0;
pub const MCDI_EVENT_LINKCHANGE_V2_DATA_LEN: c_int = 4;
pub const MCDI_EVENT_LINKCHANGE_V2_DATA_LBN: c_int = 0;
pub const MCDI_EVENT_LINKCHANGE_V2_DATA_WIDTH: c_int = 32;
pub const MCDI_EVENT_MODULECHANGE_DATA_OFST: c_int = 0;
pub const MCDI_EVENT_MODULECHANGE_DATA_LEN: c_int = 4;
pub const MCDI_EVENT_MODULECHANGE_DATA_LBN: c_int = 0;
pub const MCDI_EVENT_MODULECHANGE_DATA_WIDTH: c_int = 32;
// The new generation count after a sensor has been added or deleted.
pub const MCDI_EVENT_DYNAMIC_SENSORS_GENERATION_OFST: c_int = 0;
pub const MCDI_EVENT_DYNAMIC_SENSORS_GENERATION_LEN: c_int = 4;
pub const MCDI_EVENT_DYNAMIC_SENSORS_GENERATION_LBN: c_int = 0;
pub const MCDI_EVENT_DYNAMIC_SENSORS_GENERATION_WIDTH: c_int = 32;
// The handle of a dynamic sensor.
pub const MCDI_EVENT_DYNAMIC_SENSORS_HANDLE_OFST: c_int = 0;
pub const MCDI_EVENT_DYNAMIC_SENSORS_HANDLE_LEN: c_int = 4;
pub const MCDI_EVENT_DYNAMIC_SENSORS_HANDLE_LBN: c_int = 0;
pub const MCDI_EVENT_DYNAMIC_SENSORS_HANDLE_WIDTH: c_int = 32;
// The current values of a sensor.
pub const MCDI_EVENT_DYNAMIC_SENSORS_VALUE_OFST: c_int = 0;
pub const MCDI_EVENT_DYNAMIC_SENSORS_VALUE_LEN: c_int = 4;
pub const MCDI_EVENT_DYNAMIC_SENSORS_VALUE_LBN: c_int = 0;
pub const MCDI_EVENT_DYNAMIC_SENSORS_VALUE_WIDTH: c_int = 32;
// The current state of a sensor.
pub const MCDI_EVENT_DYNAMIC_SENSORS_STATE_LBN: c_int = 36;
pub const MCDI_EVENT_DYNAMIC_SENSORS_STATE_WIDTH: c_int = 8;
pub const MCDI_EVENT_DESC_PROXY_DATA_OFST: c_int = 0;
pub const MCDI_EVENT_DESC_PROXY_DATA_LEN: c_int = 4;
pub const MCDI_EVENT_DESC_PROXY_DATA_LBN: c_int = 0;
pub const MCDI_EVENT_DESC_PROXY_DATA_WIDTH: c_int = 32;
// Generation count of applied configuration set
pub const MCDI_EVENT_DESC_PROXY_GENERATION_OFST: c_int = 0;
pub const MCDI_EVENT_DESC_PROXY_GENERATION_LEN: c_int = 4;
pub const MCDI_EVENT_DESC_PROXY_GENERATION_LBN: c_int = 0;
pub const MCDI_EVENT_DESC_PROXY_GENERATION_WIDTH: c_int = 32;
// Virtio features negotiated with the host driver. First event (CONT=1)
// carries bits 0 to 31. Second event (CONT=0) carries bits 32 to 63.
//
pub const MCDI_EVENT_DESC_PROXY_VIRTIO_FEATURES_OFST: c_int = 0;
pub const MCDI_EVENT_DESC_PROXY_VIRTIO_FEATURES_LEN: c_int = 4;
pub const MCDI_EVENT_DESC_PROXY_VIRTIO_FEATURES_LBN: c_int = 0;
pub const MCDI_EVENT_DESC_PROXY_VIRTIO_FEATURES_WIDTH: c_int = 32;
// FCDI_EVENT structuredef
pub const FCDI_EVENT_LEN: c_int = 8;
pub const FCDI_EVENT_CONT_LBN: c_int = 32;
pub const FCDI_EVENT_CONT_WIDTH: c_int = 1;
pub const FCDI_EVENT_LEVEL_LBN: c_int = 33;
pub const FCDI_EVENT_LEVEL_WIDTH: c_int = 3;
// enum: Info.
pub const FCDI_EVENT_LEVEL_INFO: c_uint = 0x0;
// enum: Warning.
pub const FCDI_EVENT_LEVEL_WARN: c_uint = 0x1;
// enum: Error.
pub const FCDI_EVENT_LEVEL_ERR: c_uint = 0x2;
// enum: Fatal.
pub const FCDI_EVENT_LEVEL_FATAL: c_uint = 0x3;
pub const FCDI_EVENT_DATA_OFST: c_int = 0;
pub const FCDI_EVENT_DATA_LEN: c_int = 4;
pub const FCDI_EVENT_LINK_STATE_STATUS_OFST: c_int = 0;
pub const FCDI_EVENT_LINK_STATE_STATUS_LBN: c_int = 0;
pub const FCDI_EVENT_LINK_STATE_STATUS_WIDTH: c_int = 1;
pub const FCDI_EVENT_LINK_DOWN: c_uint = 0x0 /* enum */;
pub const FCDI_EVENT_LINK_UP: c_uint = 0x1 /* enum */;
pub const FCDI_EVENT_DATA_LBN: c_int = 0;
pub const FCDI_EVENT_DATA_WIDTH: c_int = 32;
pub const FCDI_EVENT_SRC_LBN: c_int = 36;
pub const FCDI_EVENT_SRC_WIDTH: c_int = 8;
pub const FCDI_EVENT_EV_CODE_LBN: c_int = 60;
pub const FCDI_EVENT_EV_CODE_WIDTH: c_int = 4;
pub const FCDI_EVENT_CODE_LBN: c_int = 44;
pub const FCDI_EVENT_CODE_WIDTH: c_int = 8;
// enum: The FC was rebooted.
pub const FCDI_EVENT_CODE_REBOOT: c_uint = 0x1;
// enum: Bad assert.
pub const FCDI_EVENT_CODE_ASSERT: c_uint = 0x2;
// enum: DDR3 test result.
pub const FCDI_EVENT_CODE_DDR_TEST_RESULT: c_uint = 0x3;
// enum: Link status.
pub const FCDI_EVENT_CODE_LINK_STATE: c_uint = 0x4;
// enum: A timed read is ready to be serviced.
pub const FCDI_EVENT_CODE_TIMED_READ: c_uint = 0x5;
// enum: One or more PPS IN events
pub const FCDI_EVENT_CODE_PPS_IN: c_uint = 0x6;
// enum: Tick event from PTP clock
pub const FCDI_EVENT_CODE_PTP_TICK: c_uint = 0x7;
// enum: ECC error counters
pub const FCDI_EVENT_CODE_DDR_ECC_STATUS: c_uint = 0x8;
// enum: Current status of PTP
pub const FCDI_EVENT_CODE_PTP_STATUS: c_uint = 0x9;
// enum: Port id config to map MC-FC port idx
pub const FCDI_EVENT_CODE_PORT_CONFIG: c_uint = 0xa;
// enum: Boot result or error code
pub const FCDI_EVENT_CODE_BOOT_RESULT: c_uint = 0xb;
pub const FCDI_EVENT_REBOOT_SRC_LBN: c_int = 36;
pub const FCDI_EVENT_REBOOT_SRC_WIDTH: c_int = 8;
pub const FCDI_EVENT_REBOOT_FC_FW: c_uint = 0x0 /* enum */;
pub const FCDI_EVENT_REBOOT_FC_BOOTLOADER: c_uint = 0x1 /* enum */;
pub const FCDI_EVENT_ASSERT_INSTR_ADDRESS_OFST: c_int = 0;
pub const FCDI_EVENT_ASSERT_INSTR_ADDRESS_LEN: c_int = 4;
pub const FCDI_EVENT_ASSERT_INSTR_ADDRESS_LBN: c_int = 0;
pub const FCDI_EVENT_ASSERT_INSTR_ADDRESS_WIDTH: c_int = 32;
pub const FCDI_EVENT_ASSERT_TYPE_LBN: c_int = 36;
pub const FCDI_EVENT_ASSERT_TYPE_WIDTH: c_int = 8;
pub const FCDI_EVENT_DDR_TEST_RESULT_STATUS_CODE_LBN: c_int = 36;
pub const FCDI_EVENT_DDR_TEST_RESULT_STATUS_CODE_WIDTH: c_int = 8;
pub const FCDI_EVENT_DDR_TEST_RESULT_RESULT_OFST: c_int = 0;
pub const FCDI_EVENT_DDR_TEST_RESULT_RESULT_LEN: c_int = 4;
pub const FCDI_EVENT_DDR_TEST_RESULT_RESULT_LBN: c_int = 0;
pub const FCDI_EVENT_DDR_TEST_RESULT_RESULT_WIDTH: c_int = 32;
pub const FCDI_EVENT_LINK_STATE_DATA_OFST: c_int = 0;
pub const FCDI_EVENT_LINK_STATE_DATA_LEN: c_int = 4;
pub const FCDI_EVENT_LINK_STATE_DATA_LBN: c_int = 0;
pub const FCDI_EVENT_LINK_STATE_DATA_WIDTH: c_int = 32;
pub const FCDI_EVENT_PTP_STATE_OFST: c_int = 0;
pub const FCDI_EVENT_PTP_STATE_LEN: c_int = 4;
pub const FCDI_EVENT_PTP_UNDEFINED: c_uint = 0x0 /* enum */;
pub const FCDI_EVENT_PTP_SETUP_FAILED: c_uint = 0x1 /* enum */;
pub const FCDI_EVENT_PTP_OPERATIONAL: c_uint = 0x2 /* enum */;
pub const FCDI_EVENT_PTP_STATE_LBN: c_int = 0;
pub const FCDI_EVENT_PTP_STATE_WIDTH: c_int = 32;
pub const FCDI_EVENT_DDR_ECC_STATUS_BANK_ID_LBN: c_int = 36;
pub const FCDI_EVENT_DDR_ECC_STATUS_BANK_ID_WIDTH: c_int = 8;
pub const FCDI_EVENT_DDR_ECC_STATUS_STATUS_OFST: c_int = 0;
pub const FCDI_EVENT_DDR_ECC_STATUS_STATUS_LEN: c_int = 4;
pub const FCDI_EVENT_DDR_ECC_STATUS_STATUS_LBN: c_int = 0;
pub const FCDI_EVENT_DDR_ECC_STATUS_STATUS_WIDTH: c_int = 32;
// Index of MC port being referred to
pub const FCDI_EVENT_PORT_CONFIG_SRC_LBN: c_int = 36;
pub const FCDI_EVENT_PORT_CONFIG_SRC_WIDTH: c_int = 8;
// FC Port index that matches the MC port index in SRC
pub const FCDI_EVENT_PORT_CONFIG_DATA_OFST: c_int = 0;
pub const FCDI_EVENT_PORT_CONFIG_DATA_LEN: c_int = 4;
pub const FCDI_EVENT_PORT_CONFIG_DATA_LBN: c_int = 0;
pub const FCDI_EVENT_PORT_CONFIG_DATA_WIDTH: c_int = 32;
pub const FCDI_EVENT_BOOT_RESULT_OFST: c_int = 0;
pub const FCDI_EVENT_BOOT_RESULT_LEN: c_int = 4;
// Enum values, see field(s):
// MC_CMD_AOE/MC_CMD_AOE_OUT_INFO/FC_BOOT_RESULT
pub const FCDI_EVENT_BOOT_RESULT_LBN: c_int = 0;
pub const FCDI_EVENT_BOOT_RESULT_WIDTH: c_int = 32;
// FCDI_EXTENDED_EVENT_PPS structuredef: Extended FCDI event to send PPS events
// to the MC. Note that this structure | is overlayed over a normal FCDI event
// such that bits 32-63 containing | event code, level, source etc remain the
// same. In this case the data | field of the header is defined to be the
// number of timestamps
//
pub const FCDI_EXTENDED_EVENT_PPS_LENMIN: c_int = 16;
pub const FCDI_EXTENDED_EVENT_PPS_LENMAX: c_int = 248;
pub const FCDI_EXTENDED_EVENT_PPS_LENMAX_MCDI2: c_int = 1016;

// Number of timestamps following
pub const FCDI_EXTENDED_EVENT_PPS_COUNT_OFST: c_int = 0;
pub const FCDI_EXTENDED_EVENT_PPS_COUNT_LEN: c_int = 4;
pub const FCDI_EXTENDED_EVENT_PPS_COUNT_LBN: c_int = 0;
pub const FCDI_EXTENDED_EVENT_PPS_COUNT_WIDTH: c_int = 32;
// Seconds field of a timestamp record
pub const FCDI_EXTENDED_EVENT_PPS_SECONDS_OFST: c_int = 8;
pub const FCDI_EXTENDED_EVENT_PPS_SECONDS_LEN: c_int = 4;
pub const FCDI_EXTENDED_EVENT_PPS_SECONDS_LBN: c_int = 64;
pub const FCDI_EXTENDED_EVENT_PPS_SECONDS_WIDTH: c_int = 32;
// Nanoseconds field of a timestamp record
pub const FCDI_EXTENDED_EVENT_PPS_NANOSECONDS_OFST: c_int = 12;
pub const FCDI_EXTENDED_EVENT_PPS_NANOSECONDS_LEN: c_int = 4;
pub const FCDI_EXTENDED_EVENT_PPS_NANOSECONDS_LBN: c_int = 96;
pub const FCDI_EXTENDED_EVENT_PPS_NANOSECONDS_WIDTH: c_int = 32;
// Timestamp records comprising the event
pub const FCDI_EXTENDED_EVENT_PPS_TIMESTAMPS_OFST: c_int = 8;
pub const FCDI_EXTENDED_EVENT_PPS_TIMESTAMPS_LEN: c_int = 8;
pub const FCDI_EXTENDED_EVENT_PPS_TIMESTAMPS_LO_OFST: c_int = 8;
pub const FCDI_EXTENDED_EVENT_PPS_TIMESTAMPS_HI_OFST: c_int = 12;
pub const FCDI_EXTENDED_EVENT_PPS_TIMESTAMPS_MINNUM: c_int = 1;
pub const FCDI_EXTENDED_EVENT_PPS_TIMESTAMPS_MAXNUM: c_int = 30;
pub const FCDI_EXTENDED_EVENT_PPS_TIMESTAMPS_MAXNUM_MCDI2: c_int = 126;
pub const FCDI_EXTENDED_EVENT_PPS_TIMESTAMPS_LBN: c_int = 64;
pub const FCDI_EXTENDED_EVENT_PPS_TIMESTAMPS_WIDTH: c_int = 64;
// MUM_EVENT structuredef
pub const MUM_EVENT_LEN: c_int = 8;
pub const MUM_EVENT_CONT_LBN: c_int = 32;
pub const MUM_EVENT_CONT_WIDTH: c_int = 1;
pub const MUM_EVENT_LEVEL_LBN: c_int = 33;
pub const MUM_EVENT_LEVEL_WIDTH: c_int = 3;
// enum: Info.
pub const MUM_EVENT_LEVEL_INFO: c_uint = 0x0;
// enum: Warning.
pub const MUM_EVENT_LEVEL_WARN: c_uint = 0x1;
// enum: Error.
pub const MUM_EVENT_LEVEL_ERR: c_uint = 0x2;
// enum: Fatal.
pub const MUM_EVENT_LEVEL_FATAL: c_uint = 0x3;
pub const MUM_EVENT_DATA_OFST: c_int = 0;
pub const MUM_EVENT_DATA_LEN: c_int = 4;
pub const MUM_EVENT_SENSOR_ID_OFST: c_int = 0;
pub const MUM_EVENT_SENSOR_ID_LBN: c_int = 0;
pub const MUM_EVENT_SENSOR_ID_WIDTH: c_int = 8;
// Enum values, see field(s):
// MC_CMD_SENSOR_INFO/MC_CMD_SENSOR_INFO_OUT/MASK
pub const MUM_EVENT_SENSOR_STATE_OFST: c_int = 0;
pub const MUM_EVENT_SENSOR_STATE_LBN: c_int = 8;
pub const MUM_EVENT_SENSOR_STATE_WIDTH: c_int = 8;
pub const MUM_EVENT_PORT_PHY_READY_OFST: c_int = 0;
pub const MUM_EVENT_PORT_PHY_READY_LBN: c_int = 0;
pub const MUM_EVENT_PORT_PHY_READY_WIDTH: c_int = 1;
pub const MUM_EVENT_PORT_PHY_LINK_UP_OFST: c_int = 0;
pub const MUM_EVENT_PORT_PHY_LINK_UP_LBN: c_int = 1;
pub const MUM_EVENT_PORT_PHY_LINK_UP_WIDTH: c_int = 1;
pub const MUM_EVENT_PORT_PHY_TX_LOL_OFST: c_int = 0;
pub const MUM_EVENT_PORT_PHY_TX_LOL_LBN: c_int = 2;
pub const MUM_EVENT_PORT_PHY_TX_LOL_WIDTH: c_int = 1;
pub const MUM_EVENT_PORT_PHY_RX_LOL_OFST: c_int = 0;
pub const MUM_EVENT_PORT_PHY_RX_LOL_LBN: c_int = 3;
pub const MUM_EVENT_PORT_PHY_RX_LOL_WIDTH: c_int = 1;
pub const MUM_EVENT_PORT_PHY_TX_LOS_OFST: c_int = 0;
pub const MUM_EVENT_PORT_PHY_TX_LOS_LBN: c_int = 4;
pub const MUM_EVENT_PORT_PHY_TX_LOS_WIDTH: c_int = 1;
pub const MUM_EVENT_PORT_PHY_RX_LOS_OFST: c_int = 0;
pub const MUM_EVENT_PORT_PHY_RX_LOS_LBN: c_int = 5;
pub const MUM_EVENT_PORT_PHY_RX_LOS_WIDTH: c_int = 1;
pub const MUM_EVENT_PORT_PHY_TX_FAULT_OFST: c_int = 0;
pub const MUM_EVENT_PORT_PHY_TX_FAULT_LBN: c_int = 6;
pub const MUM_EVENT_PORT_PHY_TX_FAULT_WIDTH: c_int = 1;
pub const MUM_EVENT_DATA_LBN: c_int = 0;
pub const MUM_EVENT_DATA_WIDTH: c_int = 32;
pub const MUM_EVENT_SRC_LBN: c_int = 36;
pub const MUM_EVENT_SRC_WIDTH: c_int = 8;
pub const MUM_EVENT_EV_CODE_LBN: c_int = 60;
pub const MUM_EVENT_EV_CODE_WIDTH: c_int = 4;
pub const MUM_EVENT_CODE_LBN: c_int = 44;
pub const MUM_EVENT_CODE_WIDTH: c_int = 8;
// enum: The MUM was rebooted.
pub const MUM_EVENT_CODE_REBOOT: c_uint = 0x1;
// enum: Bad assert.
pub const MUM_EVENT_CODE_ASSERT: c_uint = 0x2;
// enum: Sensor failure.
pub const MUM_EVENT_CODE_SENSOR: c_uint = 0x3;
// enum: Link fault has been asserted, or has cleared.
pub const MUM_EVENT_CODE_QSFP_LASI_INTERRUPT: c_uint = 0x4;
pub const MUM_EVENT_SENSOR_DATA_OFST: c_int = 0;
pub const MUM_EVENT_SENSOR_DATA_LEN: c_int = 4;
pub const MUM_EVENT_SENSOR_DATA_LBN: c_int = 0;
pub const MUM_EVENT_SENSOR_DATA_WIDTH: c_int = 32;
pub const MUM_EVENT_PORT_PHY_FLAGS_OFST: c_int = 0;
pub const MUM_EVENT_PORT_PHY_FLAGS_LEN: c_int = 4;
pub const MUM_EVENT_PORT_PHY_FLAGS_LBN: c_int = 0;
pub const MUM_EVENT_PORT_PHY_FLAGS_WIDTH: c_int = 32;
pub const MUM_EVENT_PORT_PHY_COPPER_LEN_OFST: c_int = 0;
pub const MUM_EVENT_PORT_PHY_COPPER_LEN_LEN: c_int = 4;
pub const MUM_EVENT_PORT_PHY_COPPER_LEN_LBN: c_int = 0;
pub const MUM_EVENT_PORT_PHY_COPPER_LEN_WIDTH: c_int = 32;
pub const MUM_EVENT_PORT_PHY_CAPS_OFST: c_int = 0;
pub const MUM_EVENT_PORT_PHY_CAPS_LEN: c_int = 4;
pub const MUM_EVENT_PORT_PHY_CAPS_LBN: c_int = 0;
pub const MUM_EVENT_PORT_PHY_CAPS_WIDTH: c_int = 32;
pub const MUM_EVENT_PORT_PHY_TECH_OFST: c_int = 0;
pub const MUM_EVENT_PORT_PHY_TECH_LEN: c_int = 4;
pub const MUM_EVENT_PORT_PHY_STATE_QSFP_MODULE_TECH_UNKNOWN: c_uint = 0x0 /* enum */;
pub const MUM_EVENT_PORT_PHY_STATE_QSFP_MODULE_TECH_OPTICAL: c_uint = 0x1 /* enum */;
pub const MUM_EVENT_PORT_PHY_STATE_QSFP_MODULE_TECH_COPPER_PASSIVE: c_uint = 0x2 /* enum */;
pub const MUM_EVENT_PORT_PHY_STATE_QSFP_MODULE_TECH_COPPER_PASSIVE_EQUALIZED: c_uint = 0x3 /* enum */;
pub const MUM_EVENT_PORT_PHY_STATE_QSFP_MODULE_TECH_COPPER_ACTIVE_LIMITING: c_uint = 0x4 /* enum */;
pub const MUM_EVENT_PORT_PHY_STATE_QSFP_MODULE_TECH_COPPER_ACTIVE_LINEAR: c_uint = 0x5 /* enum */;
pub const MUM_EVENT_PORT_PHY_STATE_QSFP_MODULE_TECH_BASE_T: c_uint = 0x6 /* enum */;
pub const MUM_EVENT_PORT_PHY_STATE_QSFP_MODULE_TECH_LOOPBACK_PASSIVE: c_uint = 0x7 /* enum */;
pub const MUM_EVENT_PORT_PHY_TECH_LBN: c_int = 0;
pub const MUM_EVENT_PORT_PHY_TECH_WIDTH: c_int = 32;
pub const MUM_EVENT_PORT_PHY_SRC_DATA_ID_LBN: c_int = 36;
pub const MUM_EVENT_PORT_PHY_SRC_DATA_ID_WIDTH: c_int = 4;
pub const MUM_EVENT_PORT_PHY_SRC_DATA_ID_FLAGS: c_uint = 0x0 /* enum */;
pub const MUM_EVENT_PORT_PHY_SRC_DATA_ID_COPPER_LEN: c_uint = 0x1 /* enum */;
pub const MUM_EVENT_PORT_PHY_SRC_DATA_ID_CAPS: c_uint = 0x2 /* enum */;
pub const MUM_EVENT_PORT_PHY_SRC_DATA_ID_TECH: c_uint = 0x3 /* enum */;
pub const MUM_EVENT_PORT_PHY_SRC_DATA_ID_MAX: c_uint = 0x4 /* enum */;
pub const MUM_EVENT_PORT_PHY_SRC_PORT_NO_LBN: c_int = 40;
pub const MUM_EVENT_PORT_PHY_SRC_PORT_NO_WIDTH: c_int = 4;
//
// MC_CMD_READ32
// Read multiple 32byte words from MC memory. Note - this command really
// belongs to INSECURE category but is required by shmboot. The command handler
// has additional checks to reject insecure calls.
//
pub const MC_CMD_READ32: c_uint = 0x1;

// MC_CMD_READ32_IN msgrequest
pub const MC_CMD_READ32_IN_LEN: c_int = 8;
pub const MC_CMD_READ32_IN_ADDR_OFST: c_int = 0;
pub const MC_CMD_READ32_IN_ADDR_LEN: c_int = 4;
pub const MC_CMD_READ32_IN_NUMWORDS_OFST: c_int = 4;
pub const MC_CMD_READ32_IN_NUMWORDS_LEN: c_int = 4;
// MC_CMD_READ32_OUT msgresponse
pub const MC_CMD_READ32_OUT_LENMIN: c_int = 4;
pub const MC_CMD_READ32_OUT_LENMAX: c_int = 252;
pub const MC_CMD_READ32_OUT_LENMAX_MCDI2: c_int = 1020;

pub const MC_CMD_READ32_OUT_BUFFER_OFST: c_int = 0;
pub const MC_CMD_READ32_OUT_BUFFER_LEN: c_int = 4;
pub const MC_CMD_READ32_OUT_BUFFER_MINNUM: c_int = 1;
pub const MC_CMD_READ32_OUT_BUFFER_MAXNUM: c_int = 63;
pub const MC_CMD_READ32_OUT_BUFFER_MAXNUM_MCDI2: c_int = 255;
//
// MC_CMD_WRITE32
// Write multiple 32byte words to MC memory.
//
pub const MC_CMD_WRITE32: c_uint = 0x2;

// MC_CMD_WRITE32_IN msgrequest
pub const MC_CMD_WRITE32_IN_LENMIN: c_int = 8;
pub const MC_CMD_WRITE32_IN_LENMAX: c_int = 252;
pub const MC_CMD_WRITE32_IN_LENMAX_MCDI2: c_int = 1020;

pub const MC_CMD_WRITE32_IN_ADDR_OFST: c_int = 0;
pub const MC_CMD_WRITE32_IN_ADDR_LEN: c_int = 4;
pub const MC_CMD_WRITE32_IN_BUFFER_OFST: c_int = 4;
pub const MC_CMD_WRITE32_IN_BUFFER_LEN: c_int = 4;
pub const MC_CMD_WRITE32_IN_BUFFER_MINNUM: c_int = 1;
pub const MC_CMD_WRITE32_IN_BUFFER_MAXNUM: c_int = 62;
pub const MC_CMD_WRITE32_IN_BUFFER_MAXNUM_MCDI2: c_int = 254;
// MC_CMD_WRITE32_OUT msgresponse
pub const MC_CMD_WRITE32_OUT_LEN: c_int = 0;
//
// MC_CMD_COPYCODE
// Copy MC code between two locations and jump. Note - this command really
// belongs to INSECURE category but is required by shmboot. The command handler
// has additional checks to reject insecure calls.
//
pub const MC_CMD_COPYCODE: c_uint = 0x3;

// MC_CMD_COPYCODE_IN msgrequest
pub const MC_CMD_COPYCODE_IN_LEN: c_int = 16;
// Source address
//
// The main image should be entered via a copy of a single word from and to a
// magic address, which controls various aspects of the boot. The magic address
// is a bitfield, with each bit as documented below.
//
pub const MC_CMD_COPYCODE_IN_SRC_ADDR_OFST: c_int = 0;
pub const MC_CMD_COPYCODE_IN_SRC_ADDR_LEN: c_int = 4;
// enum: Deprecated; equivalent to setting BOOT_MAGIC_PRESENT (see below)
pub const MC_CMD_COPYCODE_HUNT_NO_MAGIC_ADDR: c_uint = 0x10000;
// enum: Deprecated; equivalent to setting BOOT_MAGIC_PRESENT and
// BOOT_MAGIC_SATELLITE_CPUS_NOT_LOADED (see below)
//
pub const MC_CMD_COPYCODE_HUNT_NO_DATAPATH_MAGIC_ADDR: c_uint = 0x1d0d0;
// enum: Deprecated; equivalent to setting BOOT_MAGIC_PRESENT,
// BOOT_MAGIC_SATELLITE_CPUS_NOT_LOADED and BOOT_MAGIC_IGNORE_CONFIG (see
// below)
//
pub const MC_CMD_COPYCODE_HUNT_IGNORE_CONFIG_MAGIC_ADDR: c_uint = 0x1badc;
pub const MC_CMD_COPYCODE_IN_BOOT_MAGIC_PRESENT_OFST: c_int = 0;
pub const MC_CMD_COPYCODE_IN_BOOT_MAGIC_PRESENT_LBN: c_int = 17;
pub const MC_CMD_COPYCODE_IN_BOOT_MAGIC_PRESENT_WIDTH: c_int = 1;
pub const MC_CMD_COPYCODE_IN_BOOT_MAGIC_SATELLITE_CPUS_NOT_LOADED_OFST: c_int = 0;
pub const MC_CMD_COPYCODE_IN_BOOT_MAGIC_SATELLITE_CPUS_NOT_LOADED_LBN: c_int = 2;
pub const MC_CMD_COPYCODE_IN_BOOT_MAGIC_SATELLITE_CPUS_NOT_LOADED_WIDTH: c_int = 1;
pub const MC_CMD_COPYCODE_IN_BOOT_MAGIC_IGNORE_CONFIG_OFST: c_int = 0;
pub const MC_CMD_COPYCODE_IN_BOOT_MAGIC_IGNORE_CONFIG_LBN: c_int = 3;
pub const MC_CMD_COPYCODE_IN_BOOT_MAGIC_IGNORE_CONFIG_WIDTH: c_int = 1;
pub const MC_CMD_COPYCODE_IN_BOOT_MAGIC_SKIP_BOOT_ICORE_SYNC_OFST: c_int = 0;
pub const MC_CMD_COPYCODE_IN_BOOT_MAGIC_SKIP_BOOT_ICORE_SYNC_LBN: c_int = 4;
pub const MC_CMD_COPYCODE_IN_BOOT_MAGIC_SKIP_BOOT_ICORE_SYNC_WIDTH: c_int = 1;
pub const MC_CMD_COPYCODE_IN_BOOT_MAGIC_FORCE_STANDALONE_OFST: c_int = 0;
pub const MC_CMD_COPYCODE_IN_BOOT_MAGIC_FORCE_STANDALONE_LBN: c_int = 5;
pub const MC_CMD_COPYCODE_IN_BOOT_MAGIC_FORCE_STANDALONE_WIDTH: c_int = 1;
pub const MC_CMD_COPYCODE_IN_BOOT_MAGIC_DISABLE_XIP_OFST: c_int = 0;
pub const MC_CMD_COPYCODE_IN_BOOT_MAGIC_DISABLE_XIP_LBN: c_int = 6;
pub const MC_CMD_COPYCODE_IN_BOOT_MAGIC_DISABLE_XIP_WIDTH: c_int = 1;
// Destination address
pub const MC_CMD_COPYCODE_IN_DEST_ADDR_OFST: c_int = 4;
pub const MC_CMD_COPYCODE_IN_DEST_ADDR_LEN: c_int = 4;
pub const MC_CMD_COPYCODE_IN_NUMWORDS_OFST: c_int = 8;
pub const MC_CMD_COPYCODE_IN_NUMWORDS_LEN: c_int = 4;
// Address of where to jump after copy.
pub const MC_CMD_COPYCODE_IN_JUMP_OFST: c_int = 12;
pub const MC_CMD_COPYCODE_IN_JUMP_LEN: c_int = 4;
// enum: Control should return to the caller rather than jumping
pub const MC_CMD_COPYCODE_JUMP_NONE: c_uint = 0x1;
// MC_CMD_COPYCODE_OUT msgresponse
pub const MC_CMD_COPYCODE_OUT_LEN: c_int = 0;
//
// MC_CMD_SET_FUNC
// Select function for function-specific commands.
//
pub const MC_CMD_SET_FUNC: c_uint = 0x4;

// MC_CMD_SET_FUNC_IN msgrequest
pub const MC_CMD_SET_FUNC_IN_LEN: c_int = 4;
// Set function
pub const MC_CMD_SET_FUNC_IN_FUNC_OFST: c_int = 0;
pub const MC_CMD_SET_FUNC_IN_FUNC_LEN: c_int = 4;
// MC_CMD_SET_FUNC_OUT msgresponse
pub const MC_CMD_SET_FUNC_OUT_LEN: c_int = 0;
//
// MC_CMD_GET_BOOT_STATUS
// Get the instruction address from which the MC booted.
//
pub const MC_CMD_GET_BOOT_STATUS: c_uint = 0x5;

// MC_CMD_GET_BOOT_STATUS_IN msgrequest
pub const MC_CMD_GET_BOOT_STATUS_IN_LEN: c_int = 0;
// MC_CMD_GET_BOOT_STATUS_OUT msgresponse
pub const MC_CMD_GET_BOOT_STATUS_OUT_LEN: c_int = 8;
// ??
pub const MC_CMD_GET_BOOT_STATUS_OUT_BOOT_OFFSET_OFST: c_int = 0;
pub const MC_CMD_GET_BOOT_STATUS_OUT_BOOT_OFFSET_LEN: c_int = 4;
// enum: indicates that the MC wasn't flash booted
pub const MC_CMD_GET_BOOT_STATUS_OUT_BOOT_OFFSET_NULL: c_uint = 0xdeadbeef;
pub const MC_CMD_GET_BOOT_STATUS_OUT_FLAGS_OFST: c_int = 4;
pub const MC_CMD_GET_BOOT_STATUS_OUT_FLAGS_LEN: c_int = 4;
pub const MC_CMD_GET_BOOT_STATUS_OUT_FLAGS_WATCHDOG_OFST: c_int = 4;
pub const MC_CMD_GET_BOOT_STATUS_OUT_FLAGS_WATCHDOG_LBN: c_int = 0;
pub const MC_CMD_GET_BOOT_STATUS_OUT_FLAGS_WATCHDOG_WIDTH: c_int = 1;
pub const MC_CMD_GET_BOOT_STATUS_OUT_FLAGS_PRIMARY_OFST: c_int = 4;
pub const MC_CMD_GET_BOOT_STATUS_OUT_FLAGS_PRIMARY_LBN: c_int = 1;
pub const MC_CMD_GET_BOOT_STATUS_OUT_FLAGS_PRIMARY_WIDTH: c_int = 1;
pub const MC_CMD_GET_BOOT_STATUS_OUT_FLAGS_BACKUP_OFST: c_int = 4;
pub const MC_CMD_GET_BOOT_STATUS_OUT_FLAGS_BACKUP_LBN: c_int = 2;
pub const MC_CMD_GET_BOOT_STATUS_OUT_FLAGS_BACKUP_WIDTH: c_int = 1;
//
// MC_CMD_GET_ASSERTS
// Get (and optionally clear) the current assertion status. Only
// OUT.GLOBAL_FLAGS is guaranteed to exist in the completion payload. The other
// fields will only be present if OUT.GLOBAL_FLAGS != NO_FAILS
//
pub const MC_CMD_GET_ASSERTS: c_uint = 0x6;

// MC_CMD_GET_ASSERTS_IN msgrequest
pub const MC_CMD_GET_ASSERTS_IN_LEN: c_int = 4;
// Set to clear assertion
pub const MC_CMD_GET_ASSERTS_IN_CLEAR_OFST: c_int = 0;
pub const MC_CMD_GET_ASSERTS_IN_CLEAR_LEN: c_int = 4;
// MC_CMD_GET_ASSERTS_OUT msgresponse
pub const MC_CMD_GET_ASSERTS_OUT_LEN: c_int = 140;
// Assertion status flag.
pub const MC_CMD_GET_ASSERTS_OUT_GLOBAL_FLAGS_OFST: c_int = 0;
pub const MC_CMD_GET_ASSERTS_OUT_GLOBAL_FLAGS_LEN: c_int = 4;
// enum: No assertions have failed.
pub const MC_CMD_GET_ASSERTS_FLAGS_NO_FAILS: c_uint = 0x1;
// enum: A system-level assertion has failed.
pub const MC_CMD_GET_ASSERTS_FLAGS_SYS_FAIL: c_uint = 0x2;
// enum: A thread-level assertion has failed.
pub const MC_CMD_GET_ASSERTS_FLAGS_THR_FAIL: c_uint = 0x3;
// enum: The system was reset by the watchdog.
pub const MC_CMD_GET_ASSERTS_FLAGS_WDOG_FIRED: c_uint = 0x4;
// enum: An illegal address trap stopped the system (huntington and later)
pub const MC_CMD_GET_ASSERTS_FLAGS_ADDR_TRAP: c_uint = 0x5;
// Failing PC value
pub const MC_CMD_GET_ASSERTS_OUT_SAVED_PC_OFFS_OFST: c_int = 4;
pub const MC_CMD_GET_ASSERTS_OUT_SAVED_PC_OFFS_LEN: c_int = 4;
// Saved GP regs
pub const MC_CMD_GET_ASSERTS_OUT_GP_REGS_OFFS_OFST: c_int = 8;
pub const MC_CMD_GET_ASSERTS_OUT_GP_REGS_OFFS_LEN: c_int = 4;
pub const MC_CMD_GET_ASSERTS_OUT_GP_REGS_OFFS_NUM: c_int = 31;
// enum: A magic value hinting that the value in this register at the time of
// the failure has likely been lost.
//
pub const MC_CMD_GET_ASSERTS_REG_NO_DATA: c_uint = 0xda7a1057;
// Failing thread address
pub const MC_CMD_GET_ASSERTS_OUT_THREAD_OFFS_OFST: c_int = 132;
pub const MC_CMD_GET_ASSERTS_OUT_THREAD_OFFS_LEN: c_int = 4;
pub const MC_CMD_GET_ASSERTS_OUT_RESERVED_OFST: c_int = 136;
pub const MC_CMD_GET_ASSERTS_OUT_RESERVED_LEN: c_int = 4;
// MC_CMD_GET_ASSERTS_OUT_V2 msgresponse: Extended response for MicroBlaze CPUs
// found on Riverhead designs
//
pub const MC_CMD_GET_ASSERTS_OUT_V2_LEN: c_int = 240;
// Assertion status flag.
pub const MC_CMD_GET_ASSERTS_OUT_V2_GLOBAL_FLAGS_OFST: c_int = 0;
pub const MC_CMD_GET_ASSERTS_OUT_V2_GLOBAL_FLAGS_LEN: c_int = 4;
// enum: No assertions have failed.
// MC_CMD_GET_ASSERTS_FLAGS_NO_FAILS 0x1
// enum: A system-level assertion has failed.
// MC_CMD_GET_ASSERTS_FLAGS_SYS_FAIL 0x2
// enum: A thread-level assertion has failed.
// MC_CMD_GET_ASSERTS_FLAGS_THR_FAIL 0x3
// enum: The system was reset by the watchdog.
// MC_CMD_GET_ASSERTS_FLAGS_WDOG_FIRED 0x4
// enum: An illegal address trap stopped the system (huntington and later)
// MC_CMD_GET_ASSERTS_FLAGS_ADDR_TRAP 0x5
// Failing PC value
pub const MC_CMD_GET_ASSERTS_OUT_V2_SAVED_PC_OFFS_OFST: c_int = 4;
pub const MC_CMD_GET_ASSERTS_OUT_V2_SAVED_PC_OFFS_LEN: c_int = 4;
// Saved GP regs
pub const MC_CMD_GET_ASSERTS_OUT_V2_GP_REGS_OFFS_OFST: c_int = 8;
pub const MC_CMD_GET_ASSERTS_OUT_V2_GP_REGS_OFFS_LEN: c_int = 4;
pub const MC_CMD_GET_ASSERTS_OUT_V2_GP_REGS_OFFS_NUM: c_int = 31;
// enum: A magic value hinting that the value in this register at the time of
// the failure has likely been lost.
//
// MC_CMD_GET_ASSERTS_REG_NO_DATA 0xda7a1057
// Failing thread address
pub const MC_CMD_GET_ASSERTS_OUT_V2_THREAD_OFFS_OFST: c_int = 132;
pub const MC_CMD_GET_ASSERTS_OUT_V2_THREAD_OFFS_LEN: c_int = 4;
pub const MC_CMD_GET_ASSERTS_OUT_V2_RESERVED_OFST: c_int = 136;
pub const MC_CMD_GET_ASSERTS_OUT_V2_RESERVED_LEN: c_int = 4;
// Saved Special Function Registers
pub const MC_CMD_GET_ASSERTS_OUT_V2_SF_REGS_OFFS_OFST: c_int = 136;
pub const MC_CMD_GET_ASSERTS_OUT_V2_SF_REGS_OFFS_LEN: c_int = 4;
pub const MC_CMD_GET_ASSERTS_OUT_V2_SF_REGS_OFFS_NUM: c_int = 26;
// MC_CMD_GET_ASSERTS_OUT_V3 msgresponse: Extended response with asserted
// firmware version information
//
pub const MC_CMD_GET_ASSERTS_OUT_V3_LEN: c_int = 360;
// Assertion status flag.
pub const MC_CMD_GET_ASSERTS_OUT_V3_GLOBAL_FLAGS_OFST: c_int = 0;
pub const MC_CMD_GET_ASSERTS_OUT_V3_GLOBAL_FLAGS_LEN: c_int = 4;
// enum: No assertions have failed.
// MC_CMD_GET_ASSERTS_FLAGS_NO_FAILS 0x1
// enum: A system-level assertion has failed.
// MC_CMD_GET_ASSERTS_FLAGS_SYS_FAIL 0x2
// enum: A thread-level assertion has failed.
// MC_CMD_GET_ASSERTS_FLAGS_THR_FAIL 0x3
// enum: The system was reset by the watchdog.
// MC_CMD_GET_ASSERTS_FLAGS_WDOG_FIRED 0x4
// enum: An illegal address trap stopped the system (huntington and later)
// MC_CMD_GET_ASSERTS_FLAGS_ADDR_TRAP 0x5
// Failing PC value
pub const MC_CMD_GET_ASSERTS_OUT_V3_SAVED_PC_OFFS_OFST: c_int = 4;
pub const MC_CMD_GET_ASSERTS_OUT_V3_SAVED_PC_OFFS_LEN: c_int = 4;
// Saved GP regs
pub const MC_CMD_GET_ASSERTS_OUT_V3_GP_REGS_OFFS_OFST: c_int = 8;
pub const MC_CMD_GET_ASSERTS_OUT_V3_GP_REGS_OFFS_LEN: c_int = 4;
pub const MC_CMD_GET_ASSERTS_OUT_V3_GP_REGS_OFFS_NUM: c_int = 31;
// enum: A magic value hinting that the value in this register at the time of
// the failure has likely been lost.
//
// MC_CMD_GET_ASSERTS_REG_NO_DATA 0xda7a1057
// Failing thread address
pub const MC_CMD_GET_ASSERTS_OUT_V3_THREAD_OFFS_OFST: c_int = 132;
pub const MC_CMD_GET_ASSERTS_OUT_V3_THREAD_OFFS_LEN: c_int = 4;
pub const MC_CMD_GET_ASSERTS_OUT_V3_RESERVED_OFST: c_int = 136;
pub const MC_CMD_GET_ASSERTS_OUT_V3_RESERVED_LEN: c_int = 4;
// Saved Special Function Registers
pub const MC_CMD_GET_ASSERTS_OUT_V3_SF_REGS_OFFS_OFST: c_int = 136;
pub const MC_CMD_GET_ASSERTS_OUT_V3_SF_REGS_OFFS_LEN: c_int = 4;
pub const MC_CMD_GET_ASSERTS_OUT_V3_SF_REGS_OFFS_NUM: c_int = 26;
// MC firmware unique build ID (as binary SHA-1 value)
pub const MC_CMD_GET_ASSERTS_OUT_V3_MC_FW_BUILD_ID_OFST: c_int = 240;
pub const MC_CMD_GET_ASSERTS_OUT_V3_MC_FW_BUILD_ID_LEN: c_int = 20;
// MC firmware build date (as Unix timestamp)
pub const MC_CMD_GET_ASSERTS_OUT_V3_MC_FW_BUILD_TIMESTAMP_OFST: c_int = 260;
pub const MC_CMD_GET_ASSERTS_OUT_V3_MC_FW_BUILD_TIMESTAMP_LEN: c_int = 8;
pub const MC_CMD_GET_ASSERTS_OUT_V3_MC_FW_BUILD_TIMESTAMP_LO_OFST: c_int = 260;
pub const MC_CMD_GET_ASSERTS_OUT_V3_MC_FW_BUILD_TIMESTAMP_HI_OFST: c_int = 264;
// MC firmware version number
pub const MC_CMD_GET_ASSERTS_OUT_V3_MC_FW_VERSION_OFST: c_int = 268;
pub const MC_CMD_GET_ASSERTS_OUT_V3_MC_FW_VERSION_LEN: c_int = 8;
pub const MC_CMD_GET_ASSERTS_OUT_V3_MC_FW_VERSION_LO_OFST: c_int = 268;
pub const MC_CMD_GET_ASSERTS_OUT_V3_MC_FW_VERSION_HI_OFST: c_int = 272;
// MC firmware security level
pub const MC_CMD_GET_ASSERTS_OUT_V3_MC_FW_SECURITY_LEVEL_OFST: c_int = 276;
pub const MC_CMD_GET_ASSERTS_OUT_V3_MC_FW_SECURITY_LEVEL_LEN: c_int = 4;
// MC firmware extra version info (as null-terminated US-ASCII string)
pub const MC_CMD_GET_ASSERTS_OUT_V3_MC_FW_EXTRA_INFO_OFST: c_int = 280;
pub const MC_CMD_GET_ASSERTS_OUT_V3_MC_FW_EXTRA_INFO_LEN: c_int = 16;
// MC firmware build name (as null-terminated US-ASCII string)
pub const MC_CMD_GET_ASSERTS_OUT_V3_MC_FW_BUILD_NAME_OFST: c_int = 296;
pub const MC_CMD_GET_ASSERTS_OUT_V3_MC_FW_BUILD_NAME_LEN: c_int = 64;
//
// MC_CMD_LOG_CTRL
// Configure the output stream for log events such as link state changes,
// sensor notifications and MCDI completions
//
pub const MC_CMD_LOG_CTRL: c_uint = 0x7;

// MC_CMD_LOG_CTRL_IN msgrequest
pub const MC_CMD_LOG_CTRL_IN_LEN: c_int = 8;
// Log destination
pub const MC_CMD_LOG_CTRL_IN_LOG_DEST_OFST: c_int = 0;
pub const MC_CMD_LOG_CTRL_IN_LOG_DEST_LEN: c_int = 4;
// enum: UART.
pub const MC_CMD_LOG_CTRL_IN_LOG_DEST_UART: c_uint = 0x1;
// enum: Event queue.
pub const MC_CMD_LOG_CTRL_IN_LOG_DEST_EVQ: c_uint = 0x2;
// Legacy argument. Must be zero.
pub const MC_CMD_LOG_CTRL_IN_LOG_DEST_EVQ_OFST: c_int = 4;
pub const MC_CMD_LOG_CTRL_IN_LOG_DEST_EVQ_LEN: c_int = 4;
// MC_CMD_LOG_CTRL_OUT msgresponse
pub const MC_CMD_LOG_CTRL_OUT_LEN: c_int = 0;
//
// MC_CMD_GET_VERSION
// Get version information about adapter components.
//
pub const MC_CMD_GET_VERSION: c_uint = 0x8;

// MC_CMD_GET_VERSION_IN msgrequest
pub const MC_CMD_GET_VERSION_IN_LEN: c_int = 0;
// MC_CMD_GET_VERSION_EXT_IN msgrequest: Asks for the extended version
pub const MC_CMD_GET_VERSION_EXT_IN_LEN: c_int = 4;
// placeholder, set to 0
pub const MC_CMD_GET_VERSION_EXT_IN_EXT_FLAGS_OFST: c_int = 0;
pub const MC_CMD_GET_VERSION_EXT_IN_EXT_FLAGS_LEN: c_int = 4;
// MC_CMD_GET_VERSION_V0_OUT msgresponse: deprecated version format
pub const MC_CMD_GET_VERSION_V0_OUT_LEN: c_int = 4;
pub const MC_CMD_GET_VERSION_OUT_FIRMWARE_OFST: c_int = 0;
pub const MC_CMD_GET_VERSION_OUT_FIRMWARE_LEN: c_int = 4;
// enum: Reserved version number to indicate "any" version.
pub const MC_CMD_GET_VERSION_OUT_FIRMWARE_ANY: c_uint = 0xffffffff;
// enum: Bootrom version value for Siena.
pub const MC_CMD_GET_VERSION_OUT_FIRMWARE_SIENA_BOOTROM: c_uint = 0xb0070000;
// enum: Bootrom version value for Huntington.
pub const MC_CMD_GET_VERSION_OUT_FIRMWARE_HUNT_BOOTROM: c_uint = 0xb0070001;
// enum: Bootrom version value for Medford2.
pub const MC_CMD_GET_VERSION_OUT_FIRMWARE_MEDFORD2_BOOTROM: c_uint = 0xb0070002;
// MC_CMD_GET_VERSION_OUT msgresponse
pub const MC_CMD_GET_VERSION_OUT_LEN: c_int = 32;
// MC_CMD_GET_VERSION_OUT_FIRMWARE_OFST 0
// MC_CMD_GET_VERSION_OUT_FIRMWARE_LEN 4
// Enum values, see field(s):
// MC_CMD_GET_VERSION_V0_OUT/MC_CMD_GET_VERSION_OUT_FIRMWARE
pub const MC_CMD_GET_VERSION_OUT_PCOL_OFST: c_int = 4;
pub const MC_CMD_GET_VERSION_OUT_PCOL_LEN: c_int = 4;
// 128bit mask of functions supported by the current firmware
pub const MC_CMD_GET_VERSION_OUT_SUPPORTED_FUNCS_OFST: c_int = 8;
pub const MC_CMD_GET_VERSION_OUT_SUPPORTED_FUNCS_LEN: c_int = 16;
pub const MC_CMD_GET_VERSION_OUT_VERSION_OFST: c_int = 24;
pub const MC_CMD_GET_VERSION_OUT_VERSION_LEN: c_int = 8;
pub const MC_CMD_GET_VERSION_OUT_VERSION_LO_OFST: c_int = 24;
pub const MC_CMD_GET_VERSION_OUT_VERSION_HI_OFST: c_int = 28;
// MC_CMD_GET_VERSION_EXT_OUT msgresponse
pub const MC_CMD_GET_VERSION_EXT_OUT_LEN: c_int = 48;
// MC_CMD_GET_VERSION_OUT_FIRMWARE_OFST 0
// MC_CMD_GET_VERSION_OUT_FIRMWARE_LEN 4
// Enum values, see field(s):
// MC_CMD_GET_VERSION_V0_OUT/MC_CMD_GET_VERSION_OUT_FIRMWARE
pub const MC_CMD_GET_VERSION_EXT_OUT_PCOL_OFST: c_int = 4;
pub const MC_CMD_GET_VERSION_EXT_OUT_PCOL_LEN: c_int = 4;
// 128bit mask of functions supported by the current firmware
pub const MC_CMD_GET_VERSION_EXT_OUT_SUPPORTED_FUNCS_OFST: c_int = 8;
pub const MC_CMD_GET_VERSION_EXT_OUT_SUPPORTED_FUNCS_LEN: c_int = 16;
pub const MC_CMD_GET_VERSION_EXT_OUT_VERSION_OFST: c_int = 24;
pub const MC_CMD_GET_VERSION_EXT_OUT_VERSION_LEN: c_int = 8;
pub const MC_CMD_GET_VERSION_EXT_OUT_VERSION_LO_OFST: c_int = 24;
pub const MC_CMD_GET_VERSION_EXT_OUT_VERSION_HI_OFST: c_int = 28;
// extra info
pub const MC_CMD_GET_VERSION_EXT_OUT_EXTRA_OFST: c_int = 32;
pub const MC_CMD_GET_VERSION_EXT_OUT_EXTRA_LEN: c_int = 16;
// MC_CMD_GET_VERSION_V2_OUT msgresponse: Extended response providing version
// information for all adapter components. For Riverhead based designs, base MC
// firmware version fields refer to NMC firmware, while CMC firmware data is in
// dedicated CMC fields. Flags indicate which data is present in the response
// (depending on which components exist on a particular adapter)
//
pub const MC_CMD_GET_VERSION_V2_OUT_LEN: c_int = 304;
// MC_CMD_GET_VERSION_OUT_FIRMWARE_OFST 0
// MC_CMD_GET_VERSION_OUT_FIRMWARE_LEN 4
// Enum values, see field(s):
// MC_CMD_GET_VERSION_V0_OUT/MC_CMD_GET_VERSION_OUT_FIRMWARE
pub const MC_CMD_GET_VERSION_V2_OUT_PCOL_OFST: c_int = 4;
pub const MC_CMD_GET_VERSION_V2_OUT_PCOL_LEN: c_int = 4;
// 128bit mask of functions supported by the current firmware
pub const MC_CMD_GET_VERSION_V2_OUT_SUPPORTED_FUNCS_OFST: c_int = 8;
pub const MC_CMD_GET_VERSION_V2_OUT_SUPPORTED_FUNCS_LEN: c_int = 16;
pub const MC_CMD_GET_VERSION_V2_OUT_VERSION_OFST: c_int = 24;
pub const MC_CMD_GET_VERSION_V2_OUT_VERSION_LEN: c_int = 8;
pub const MC_CMD_GET_VERSION_V2_OUT_VERSION_LO_OFST: c_int = 24;
pub const MC_CMD_GET_VERSION_V2_OUT_VERSION_HI_OFST: c_int = 28;
// extra info
pub const MC_CMD_GET_VERSION_V2_OUT_EXTRA_OFST: c_int = 32;
pub const MC_CMD_GET_VERSION_V2_OUT_EXTRA_LEN: c_int = 16;
// Flags indicating which extended fields are valid
pub const MC_CMD_GET_VERSION_V2_OUT_FLAGS_OFST: c_int = 48;
pub const MC_CMD_GET_VERSION_V2_OUT_FLAGS_LEN: c_int = 4;
pub const MC_CMD_GET_VERSION_V2_OUT_MCFW_EXT_INFO_PRESENT_OFST: c_int = 48;
pub const MC_CMD_GET_VERSION_V2_OUT_MCFW_EXT_INFO_PRESENT_LBN: c_int = 0;
pub const MC_CMD_GET_VERSION_V2_OUT_MCFW_EXT_INFO_PRESENT_WIDTH: c_int = 1;
pub const MC_CMD_GET_VERSION_V2_OUT_SUCFW_EXT_INFO_PRESENT_OFST: c_int = 48;
pub const MC_CMD_GET_VERSION_V2_OUT_SUCFW_EXT_INFO_PRESENT_LBN: c_int = 1;
pub const MC_CMD_GET_VERSION_V2_OUT_SUCFW_EXT_INFO_PRESENT_WIDTH: c_int = 1;
pub const MC_CMD_GET_VERSION_V2_OUT_CMC_EXT_INFO_PRESENT_OFST: c_int = 48;
pub const MC_CMD_GET_VERSION_V2_OUT_CMC_EXT_INFO_PRESENT_LBN: c_int = 2;
pub const MC_CMD_GET_VERSION_V2_OUT_CMC_EXT_INFO_PRESENT_WIDTH: c_int = 1;
pub const MC_CMD_GET_VERSION_V2_OUT_FPGA_EXT_INFO_PRESENT_OFST: c_int = 48;
pub const MC_CMD_GET_VERSION_V2_OUT_FPGA_EXT_INFO_PRESENT_LBN: c_int = 3;
pub const MC_CMD_GET_VERSION_V2_OUT_FPGA_EXT_INFO_PRESENT_WIDTH: c_int = 1;
pub const MC_CMD_GET_VERSION_V2_OUT_BOARD_EXT_INFO_PRESENT_OFST: c_int = 48;
pub const MC_CMD_GET_VERSION_V2_OUT_BOARD_EXT_INFO_PRESENT_LBN: c_int = 4;
pub const MC_CMD_GET_VERSION_V2_OUT_BOARD_EXT_INFO_PRESENT_WIDTH: c_int = 1;
// MC firmware unique build ID (as binary SHA-1 value)
pub const MC_CMD_GET_VERSION_V2_OUT_MCFW_BUILD_ID_OFST: c_int = 52;
pub const MC_CMD_GET_VERSION_V2_OUT_MCFW_BUILD_ID_LEN: c_int = 20;
// MC firmware security level
pub const MC_CMD_GET_VERSION_V2_OUT_MCFW_SECURITY_LEVEL_OFST: c_int = 72;
pub const MC_CMD_GET_VERSION_V2_OUT_MCFW_SECURITY_LEVEL_LEN: c_int = 4;
// MC firmware build name (as null-terminated US-ASCII string)
pub const MC_CMD_GET_VERSION_V2_OUT_MCFW_BUILD_NAME_OFST: c_int = 76;
pub const MC_CMD_GET_VERSION_V2_OUT_MCFW_BUILD_NAME_LEN: c_int = 64;
// The SUC firmware version as four numbers - a.b.c.d
pub const MC_CMD_GET_VERSION_V2_OUT_SUCFW_VERSION_OFST: c_int = 140;
pub const MC_CMD_GET_VERSION_V2_OUT_SUCFW_VERSION_LEN: c_int = 4;
pub const MC_CMD_GET_VERSION_V2_OUT_SUCFW_VERSION_NUM: c_int = 4;
// SUC firmware build date (as 64-bit Unix timestamp)
pub const MC_CMD_GET_VERSION_V2_OUT_SUCFW_BUILD_DATE_OFST: c_int = 156;
pub const MC_CMD_GET_VERSION_V2_OUT_SUCFW_BUILD_DATE_LEN: c_int = 8;
pub const MC_CMD_GET_VERSION_V2_OUT_SUCFW_BUILD_DATE_LO_OFST: c_int = 156;
pub const MC_CMD_GET_VERSION_V2_OUT_SUCFW_BUILD_DATE_HI_OFST: c_int = 160;
// The ID of the SUC chip. This is specific to the platform but typically
// indicates family, memory sizes etc. See SF-116728-SW for further details.
//
pub const MC_CMD_GET_VERSION_V2_OUT_SUCFW_CHIP_ID_OFST: c_int = 164;
pub const MC_CMD_GET_VERSION_V2_OUT_SUCFW_CHIP_ID_LEN: c_int = 4;
// The CMC firmware version as four numbers - a.b.c.d
pub const MC_CMD_GET_VERSION_V2_OUT_CMCFW_VERSION_OFST: c_int = 168;
pub const MC_CMD_GET_VERSION_V2_OUT_CMCFW_VERSION_LEN: c_int = 4;
pub const MC_CMD_GET_VERSION_V2_OUT_CMCFW_VERSION_NUM: c_int = 4;
// CMC firmware build date (as 64-bit Unix timestamp)
pub const MC_CMD_GET_VERSION_V2_OUT_CMCFW_BUILD_DATE_OFST: c_int = 184;
pub const MC_CMD_GET_VERSION_V2_OUT_CMCFW_BUILD_DATE_LEN: c_int = 8;
pub const MC_CMD_GET_VERSION_V2_OUT_CMCFW_BUILD_DATE_LO_OFST: c_int = 184;
pub const MC_CMD_GET_VERSION_V2_OUT_CMCFW_BUILD_DATE_HI_OFST: c_int = 188;
// FPGA version as three numbers. On Riverhead based systems this field uses
// the same encoding as hardware version ID registers (MC_FPGA_BUILD_HWRD_REG):
// FPGA_VERSION[0]: x => Image H{x} FPGA_VERSION[1]: Revision letter (0 => A, 1
// => B, ...) FPGA_VERSION[2]: Sub-revision number
//
pub const MC_CMD_GET_VERSION_V2_OUT_FPGA_VERSION_OFST: c_int = 192;
pub const MC_CMD_GET_VERSION_V2_OUT_FPGA_VERSION_LEN: c_int = 4;
pub const MC_CMD_GET_VERSION_V2_OUT_FPGA_VERSION_NUM: c_int = 3;
// Extra FPGA revision information (as null-terminated US-ASCII string)
pub const MC_CMD_GET_VERSION_V2_OUT_FPGA_EXTRA_OFST: c_int = 204;
pub const MC_CMD_GET_VERSION_V2_OUT_FPGA_EXTRA_LEN: c_int = 16;
// Board name / adapter model (as null-terminated US-ASCII string)
pub const MC_CMD_GET_VERSION_V2_OUT_BOARD_NAME_OFST: c_int = 220;
pub const MC_CMD_GET_VERSION_V2_OUT_BOARD_NAME_LEN: c_int = 16;
// Board revision number
pub const MC_CMD_GET_VERSION_V2_OUT_BOARD_REVISION_OFST: c_int = 236;
pub const MC_CMD_GET_VERSION_V2_OUT_BOARD_REVISION_LEN: c_int = 4;
// Board serial number (as null-terminated US-ASCII string)
pub const MC_CMD_GET_VERSION_V2_OUT_BOARD_SERIAL_OFST: c_int = 240;
pub const MC_CMD_GET_VERSION_V2_OUT_BOARD_SERIAL_LEN: c_int = 64;
//
// MC_CMD_PTP
// Perform PTP operation
//
pub const MC_CMD_PTP: c_uint = 0xb;

// MC_CMD_PTP_IN msgrequest
pub const MC_CMD_PTP_IN_LEN: c_int = 1;
// PTP operation code
pub const MC_CMD_PTP_IN_OP_OFST: c_int = 0;
pub const MC_CMD_PTP_IN_OP_LEN: c_int = 1;
// enum: Enable PTP packet timestamping operation.
pub const MC_CMD_PTP_OP_ENABLE: c_uint = 0x1;
// enum: Disable PTP packet timestamping operation.
pub const MC_CMD_PTP_OP_DISABLE: c_uint = 0x2;
// enum: Send a PTP packet. This operation is used on Siena and Huntington.
// From Medford onwards it is not supported: on those platforms PTP transmit
// timestamping is done using the fast path.
//
pub const MC_CMD_PTP_OP_TRANSMIT: c_uint = 0x3;
// enum: Read the current NIC time.
pub const MC_CMD_PTP_OP_READ_NIC_TIME: c_uint = 0x4;
// enum: Get the current PTP status. Note that the clock frequency returned (in
// Hz) is rounded to the nearest MHz (e.g. 666000000 for 666666666).
//
pub const MC_CMD_PTP_OP_STATUS: c_uint = 0x5;
// enum: Adjust the PTP NIC's time.
pub const MC_CMD_PTP_OP_ADJUST: c_uint = 0x6;
// enum: Synchronize host and NIC time.
pub const MC_CMD_PTP_OP_SYNCHRONIZE: c_uint = 0x7;
// enum: Basic manufacturing tests. Siena PTP adapters only.
pub const MC_CMD_PTP_OP_MANFTEST_BASIC: c_uint = 0x8;
// enum: Packet based manufacturing tests. Siena PTP adapters only.
pub const MC_CMD_PTP_OP_MANFTEST_PACKET: c_uint = 0x9;
// enum: Reset some of the PTP related statistics
pub const MC_CMD_PTP_OP_RESET_STATS: c_uint = 0xa;
// enum: Debug operations to MC.
pub const MC_CMD_PTP_OP_DEBUG: c_uint = 0xb;
// enum: Read an FPGA register. Siena PTP adapters only.
pub const MC_CMD_PTP_OP_FPGAREAD: c_uint = 0xc;
// enum: Write an FPGA register. Siena PTP adapters only.
pub const MC_CMD_PTP_OP_FPGAWRITE: c_uint = 0xd;
// enum: Apply an offset to the NIC clock
pub const MC_CMD_PTP_OP_CLOCK_OFFSET_ADJUST: c_uint = 0xe;
// enum: Change the frequency correction applied to the NIC clock
pub const MC_CMD_PTP_OP_CLOCK_FREQ_ADJUST: c_uint = 0xf;
// enum: Set the MC packet filter VLAN tags for received PTP packets.
// Deprecated for Huntington onwards.
//
pub const MC_CMD_PTP_OP_RX_SET_VLAN_FILTER: c_uint = 0x10;
// enum: Set the MC packet filter UUID for received PTP packets. Deprecated for
// Huntington onwards.
//
pub const MC_CMD_PTP_OP_RX_SET_UUID_FILTER: c_uint = 0x11;
// enum: Set the MC packet filter Domain for received PTP packets. Deprecated
// for Huntington onwards.
//
pub const MC_CMD_PTP_OP_RX_SET_DOMAIN_FILTER: c_uint = 0x12;
// enum: Set the clock source. Required for snapper tests on Huntington and
// Medford. Not implemented for Siena or Medford2.
//
pub const MC_CMD_PTP_OP_SET_CLK_SRC: c_uint = 0x13;
// enum: Reset value of Timer Reg. Not implemented.
pub const MC_CMD_PTP_OP_RST_CLK: c_uint = 0x14;
// enum: Enable the forwarding of PPS events to the host
pub const MC_CMD_PTP_OP_PPS_ENABLE: c_uint = 0x15;
// enum: Get the time format used by this NIC for PTP operations
pub const MC_CMD_PTP_OP_GET_TIME_FORMAT: c_uint = 0x16;
// enum: Get the clock attributes. NOTE- extended version of
// MC_CMD_PTP_OP_GET_TIME_FORMAT
//
pub const MC_CMD_PTP_OP_GET_ATTRIBUTES: c_uint = 0x16;
// enum: Get corrections that should be applied to the various different
// timestamps
//
pub const MC_CMD_PTP_OP_GET_TIMESTAMP_CORRECTIONS: c_uint = 0x17;
// enum: Subscribe to receive periodic time events indicating the current NIC
// time
//
pub const MC_CMD_PTP_OP_TIME_EVENT_SUBSCRIBE: c_uint = 0x18;
// enum: Unsubscribe to stop receiving time events
pub const MC_CMD_PTP_OP_TIME_EVENT_UNSUBSCRIBE: c_uint = 0x19;
// enum: PPS based manfacturing tests. Requires PPS output to be looped to PPS
// input on the same NIC. Siena PTP adapters only.
//
pub const MC_CMD_PTP_OP_MANFTEST_PPS: c_uint = 0x1a;
// enum: Set the PTP sync status. Status is used by firmware to report to event
// subscribers.
//
pub const MC_CMD_PTP_OP_SET_SYNC_STATUS: c_uint = 0x1b;
// enum: Above this for future use.
pub const MC_CMD_PTP_OP_MAX: c_uint = 0x1c;
// MC_CMD_PTP_IN_ENABLE msgrequest
pub const MC_CMD_PTP_IN_ENABLE_LEN: c_int = 16;
pub const MC_CMD_PTP_IN_CMD_OFST: c_int = 0;
pub const MC_CMD_PTP_IN_CMD_LEN: c_int = 4;
pub const MC_CMD_PTP_IN_PERIPH_ID_OFST: c_int = 4;
pub const MC_CMD_PTP_IN_PERIPH_ID_LEN: c_int = 4;
// Not used. Events are always sent to function relative queue 0.
pub const MC_CMD_PTP_IN_ENABLE_QUEUE_OFST: c_int = 8;
pub const MC_CMD_PTP_IN_ENABLE_QUEUE_LEN: c_int = 4;
// PTP timestamping mode. Not used from Huntington onwards.
pub const MC_CMD_PTP_IN_ENABLE_MODE_OFST: c_int = 12;
pub const MC_CMD_PTP_IN_ENABLE_MODE_LEN: c_int = 4;
// enum: PTP, version 1
pub const MC_CMD_PTP_MODE_V1: c_uint = 0x0;
// enum: PTP, version 1, with VLAN headers - deprecated
pub const MC_CMD_PTP_MODE_V1_VLAN: c_uint = 0x1;
// enum: PTP, version 2
pub const MC_CMD_PTP_MODE_V2: c_uint = 0x2;
// enum: PTP, version 2, with VLAN headers - deprecated
pub const MC_CMD_PTP_MODE_V2_VLAN: c_uint = 0x3;
// enum: PTP, version 2, with improved UUID filtering
pub const MC_CMD_PTP_MODE_V2_ENHANCED: c_uint = 0x4;
// enum: FCoE (seconds and microseconds)
pub const MC_CMD_PTP_MODE_FCOE: c_uint = 0x5;
// MC_CMD_PTP_IN_DISABLE msgrequest
pub const MC_CMD_PTP_IN_DISABLE_LEN: c_int = 8;
// MC_CMD_PTP_IN_CMD_OFST 0
// MC_CMD_PTP_IN_CMD_LEN 4
// MC_CMD_PTP_IN_PERIPH_ID_OFST 4
// MC_CMD_PTP_IN_PERIPH_ID_LEN 4
// MC_CMD_PTP_IN_TRANSMIT msgrequest
pub const MC_CMD_PTP_IN_TRANSMIT_LENMIN: c_int = 13;
pub const MC_CMD_PTP_IN_TRANSMIT_LENMAX: c_int = 252;
pub const MC_CMD_PTP_IN_TRANSMIT_LENMAX_MCDI2: c_int = 1020;

// MC_CMD_PTP_IN_CMD_OFST 0
// MC_CMD_PTP_IN_CMD_LEN 4
// MC_CMD_PTP_IN_PERIPH_ID_OFST 4
// MC_CMD_PTP_IN_PERIPH_ID_LEN 4
// Transmit packet length
pub const MC_CMD_PTP_IN_TRANSMIT_LENGTH_OFST: c_int = 8;
pub const MC_CMD_PTP_IN_TRANSMIT_LENGTH_LEN: c_int = 4;
// Transmit packet data
pub const MC_CMD_PTP_IN_TRANSMIT_PACKET_OFST: c_int = 12;
pub const MC_CMD_PTP_IN_TRANSMIT_PACKET_LEN: c_int = 1;
pub const MC_CMD_PTP_IN_TRANSMIT_PACKET_MINNUM: c_int = 1;
pub const MC_CMD_PTP_IN_TRANSMIT_PACKET_MAXNUM: c_int = 240;
pub const MC_CMD_PTP_IN_TRANSMIT_PACKET_MAXNUM_MCDI2: c_int = 1008;
// MC_CMD_PTP_IN_READ_NIC_TIME msgrequest
pub const MC_CMD_PTP_IN_READ_NIC_TIME_LEN: c_int = 8;
// MC_CMD_PTP_IN_CMD_OFST 0
// MC_CMD_PTP_IN_CMD_LEN 4
// MC_CMD_PTP_IN_PERIPH_ID_OFST 4
// MC_CMD_PTP_IN_PERIPH_ID_LEN 4
// MC_CMD_PTP_IN_READ_NIC_TIME_V2 msgrequest
pub const MC_CMD_PTP_IN_READ_NIC_TIME_V2_LEN: c_int = 8;
// MC_CMD_PTP_IN_CMD_OFST 0
// MC_CMD_PTP_IN_CMD_LEN 4
// MC_CMD_PTP_IN_PERIPH_ID_OFST 4
// MC_CMD_PTP_IN_PERIPH_ID_LEN 4
// MC_CMD_PTP_IN_STATUS msgrequest
pub const MC_CMD_PTP_IN_STATUS_LEN: c_int = 8;
// MC_CMD_PTP_IN_CMD_OFST 0
// MC_CMD_PTP_IN_CMD_LEN 4
// MC_CMD_PTP_IN_PERIPH_ID_OFST 4
// MC_CMD_PTP_IN_PERIPH_ID_LEN 4
// MC_CMD_PTP_IN_ADJUST msgrequest
pub const MC_CMD_PTP_IN_ADJUST_LEN: c_int = 24;
// MC_CMD_PTP_IN_CMD_OFST 0
// MC_CMD_PTP_IN_CMD_LEN 4
// MC_CMD_PTP_IN_PERIPH_ID_OFST 4
// MC_CMD_PTP_IN_PERIPH_ID_LEN 4
// Frequency adjustment 40 bit fixed point ns
pub const MC_CMD_PTP_IN_ADJUST_FREQ_OFST: c_int = 8;
pub const MC_CMD_PTP_IN_ADJUST_FREQ_LEN: c_int = 8;
pub const MC_CMD_PTP_IN_ADJUST_FREQ_LO_OFST: c_int = 8;
pub const MC_CMD_PTP_IN_ADJUST_FREQ_HI_OFST: c_int = 12;
// enum: Number of fractional bits in frequency adjustment
pub const MC_CMD_PTP_IN_ADJUST_BITS: c_uint = 0x28;
// enum: Number of fractional bits in frequency adjustment when FP44_FREQ_ADJ
// is indicated in the MC_CMD_PTP_OUT_GET_ATTRIBUTES command CAPABILITIES
// field.
//
pub const MC_CMD_PTP_IN_ADJUST_BITS_FP44: c_uint = 0x2c;
// Time adjustment in seconds
pub const MC_CMD_PTP_IN_ADJUST_SECONDS_OFST: c_int = 16;
pub const MC_CMD_PTP_IN_ADJUST_SECONDS_LEN: c_int = 4;
// Time adjustment major value
pub const MC_CMD_PTP_IN_ADJUST_MAJOR_OFST: c_int = 16;
pub const MC_CMD_PTP_IN_ADJUST_MAJOR_LEN: c_int = 4;
// Time adjustment in nanoseconds
pub const MC_CMD_PTP_IN_ADJUST_NANOSECONDS_OFST: c_int = 20;
pub const MC_CMD_PTP_IN_ADJUST_NANOSECONDS_LEN: c_int = 4;
// Time adjustment minor value
pub const MC_CMD_PTP_IN_ADJUST_MINOR_OFST: c_int = 20;
pub const MC_CMD_PTP_IN_ADJUST_MINOR_LEN: c_int = 4;
// MC_CMD_PTP_IN_ADJUST_V2 msgrequest
pub const MC_CMD_PTP_IN_ADJUST_V2_LEN: c_int = 28;
// MC_CMD_PTP_IN_CMD_OFST 0
// MC_CMD_PTP_IN_CMD_LEN 4
// MC_CMD_PTP_IN_PERIPH_ID_OFST 4
// MC_CMD_PTP_IN_PERIPH_ID_LEN 4
// Frequency adjustment 40 bit fixed point ns
pub const MC_CMD_PTP_IN_ADJUST_V2_FREQ_OFST: c_int = 8;
pub const MC_CMD_PTP_IN_ADJUST_V2_FREQ_LEN: c_int = 8;
pub const MC_CMD_PTP_IN_ADJUST_V2_FREQ_LO_OFST: c_int = 8;
pub const MC_CMD_PTP_IN_ADJUST_V2_FREQ_HI_OFST: c_int = 12;
// enum: Number of fractional bits in frequency adjustment
// MC_CMD_PTP_IN_ADJUST_BITS 0x28
// enum: Number of fractional bits in frequency adjustment when FP44_FREQ_ADJ
// is indicated in the MC_CMD_PTP_OUT_GET_ATTRIBUTES command CAPABILITIES
// field.
//
// MC_CMD_PTP_IN_ADJUST_BITS_FP44 0x2c
// Time adjustment in seconds
pub const MC_CMD_PTP_IN_ADJUST_V2_SECONDS_OFST: c_int = 16;
pub const MC_CMD_PTP_IN_ADJUST_V2_SECONDS_LEN: c_int = 4;
// Time adjustment major value
pub const MC_CMD_PTP_IN_ADJUST_V2_MAJOR_OFST: c_int = 16;
pub const MC_CMD_PTP_IN_ADJUST_V2_MAJOR_LEN: c_int = 4;
// Time adjustment in nanoseconds
pub const MC_CMD_PTP_IN_ADJUST_V2_NANOSECONDS_OFST: c_int = 20;
pub const MC_CMD_PTP_IN_ADJUST_V2_NANOSECONDS_LEN: c_int = 4;
// Time adjustment minor value
pub const MC_CMD_PTP_IN_ADJUST_V2_MINOR_OFST: c_int = 20;
pub const MC_CMD_PTP_IN_ADJUST_V2_MINOR_LEN: c_int = 4;
// Upper 32bits of major time offset adjustment
pub const MC_CMD_PTP_IN_ADJUST_V2_MAJOR_HI_OFST: c_int = 24;
pub const MC_CMD_PTP_IN_ADJUST_V2_MAJOR_HI_LEN: c_int = 4;
// MC_CMD_PTP_IN_SYNCHRONIZE msgrequest
pub const MC_CMD_PTP_IN_SYNCHRONIZE_LEN: c_int = 20;
// MC_CMD_PTP_IN_CMD_OFST 0
// MC_CMD_PTP_IN_CMD_LEN 4
// MC_CMD_PTP_IN_PERIPH_ID_OFST 4
// MC_CMD_PTP_IN_PERIPH_ID_LEN 4
// Number of time readings to capture
pub const MC_CMD_PTP_IN_SYNCHRONIZE_NUMTIMESETS_OFST: c_int = 8;
pub const MC_CMD_PTP_IN_SYNCHRONIZE_NUMTIMESETS_LEN: c_int = 4;
// Host address in which to write "synchronization started" indication (64
// bits)
//
pub const MC_CMD_PTP_IN_SYNCHRONIZE_START_ADDR_OFST: c_int = 12;
pub const MC_CMD_PTP_IN_SYNCHRONIZE_START_ADDR_LEN: c_int = 8;
pub const MC_CMD_PTP_IN_SYNCHRONIZE_START_ADDR_LO_OFST: c_int = 12;
pub const MC_CMD_PTP_IN_SYNCHRONIZE_START_ADDR_HI_OFST: c_int = 16;
// MC_CMD_PTP_IN_MANFTEST_BASIC msgrequest
pub const MC_CMD_PTP_IN_MANFTEST_BASIC_LEN: c_int = 8;
// MC_CMD_PTP_IN_CMD_OFST 0
// MC_CMD_PTP_IN_CMD_LEN 4
// MC_CMD_PTP_IN_PERIPH_ID_OFST 4
// MC_CMD_PTP_IN_PERIPH_ID_LEN 4
// MC_CMD_PTP_IN_MANFTEST_PACKET msgrequest
pub const MC_CMD_PTP_IN_MANFTEST_PACKET_LEN: c_int = 12;
// MC_CMD_PTP_IN_CMD_OFST 0
// MC_CMD_PTP_IN_CMD_LEN 4
// MC_CMD_PTP_IN_PERIPH_ID_OFST 4
// MC_CMD_PTP_IN_PERIPH_ID_LEN 4
// Enable or disable packet testing
pub const MC_CMD_PTP_IN_MANFTEST_PACKET_TEST_ENABLE_OFST: c_int = 8;
pub const MC_CMD_PTP_IN_MANFTEST_PACKET_TEST_ENABLE_LEN: c_int = 4;
// MC_CMD_PTP_IN_RESET_STATS msgrequest: Reset PTP statistics
pub const MC_CMD_PTP_IN_RESET_STATS_LEN: c_int = 8;
// MC_CMD_PTP_IN_CMD_OFST 0
// MC_CMD_PTP_IN_CMD_LEN 4
// MC_CMD_PTP_IN_PERIPH_ID_OFST 4
// MC_CMD_PTP_IN_PERIPH_ID_LEN 4
// MC_CMD_PTP_IN_DEBUG msgrequest
pub const MC_CMD_PTP_IN_DEBUG_LEN: c_int = 12;
// MC_CMD_PTP_IN_CMD_OFST 0
// MC_CMD_PTP_IN_CMD_LEN 4
// MC_CMD_PTP_IN_PERIPH_ID_OFST 4
// MC_CMD_PTP_IN_PERIPH_ID_LEN 4
// Debug operations
pub const MC_CMD_PTP_IN_DEBUG_DEBUG_PARAM_OFST: c_int = 8;
pub const MC_CMD_PTP_IN_DEBUG_DEBUG_PARAM_LEN: c_int = 4;
// MC_CMD_PTP_IN_FPGAREAD msgrequest
pub const MC_CMD_PTP_IN_FPGAREAD_LEN: c_int = 16;
// MC_CMD_PTP_IN_CMD_OFST 0
// MC_CMD_PTP_IN_CMD_LEN 4
// MC_CMD_PTP_IN_PERIPH_ID_OFST 4
// MC_CMD_PTP_IN_PERIPH_ID_LEN 4
pub const MC_CMD_PTP_IN_FPGAREAD_ADDR_OFST: c_int = 8;
pub const MC_CMD_PTP_IN_FPGAREAD_ADDR_LEN: c_int = 4;
pub const MC_CMD_PTP_IN_FPGAREAD_NUMBYTES_OFST: c_int = 12;
pub const MC_CMD_PTP_IN_FPGAREAD_NUMBYTES_LEN: c_int = 4;
// MC_CMD_PTP_IN_FPGAWRITE msgrequest
pub const MC_CMD_PTP_IN_FPGAWRITE_LENMIN: c_int = 13;
pub const MC_CMD_PTP_IN_FPGAWRITE_LENMAX: c_int = 252;
pub const MC_CMD_PTP_IN_FPGAWRITE_LENMAX_MCDI2: c_int = 1020;

// MC_CMD_PTP_IN_CMD_OFST 0
// MC_CMD_PTP_IN_CMD_LEN 4
// MC_CMD_PTP_IN_PERIPH_ID_OFST 4
// MC_CMD_PTP_IN_PERIPH_ID_LEN 4
pub const MC_CMD_PTP_IN_FPGAWRITE_ADDR_OFST: c_int = 8;
pub const MC_CMD_PTP_IN_FPGAWRITE_ADDR_LEN: c_int = 4;
pub const MC_CMD_PTP_IN_FPGAWRITE_BUFFER_OFST: c_int = 12;
pub const MC_CMD_PTP_IN_FPGAWRITE_BUFFER_LEN: c_int = 1;
pub const MC_CMD_PTP_IN_FPGAWRITE_BUFFER_MINNUM: c_int = 1;
pub const MC_CMD_PTP_IN_FPGAWRITE_BUFFER_MAXNUM: c_int = 240;
pub const MC_CMD_PTP_IN_FPGAWRITE_BUFFER_MAXNUM_MCDI2: c_int = 1008;
// MC_CMD_PTP_IN_CLOCK_OFFSET_ADJUST msgrequest
pub const MC_CMD_PTP_IN_CLOCK_OFFSET_ADJUST_LEN: c_int = 16;
// MC_CMD_PTP_IN_CMD_OFST 0
// MC_CMD_PTP_IN_CMD_LEN 4
// MC_CMD_PTP_IN_PERIPH_ID_OFST 4
// MC_CMD_PTP_IN_PERIPH_ID_LEN 4
// Time adjustment in seconds
pub const MC_CMD_PTP_IN_CLOCK_OFFSET_ADJUST_SECONDS_OFST: c_int = 8;
pub const MC_CMD_PTP_IN_CLOCK_OFFSET_ADJUST_SECONDS_LEN: c_int = 4;
// Time adjustment major value
pub const MC_CMD_PTP_IN_CLOCK_OFFSET_ADJUST_MAJOR_OFST: c_int = 8;
pub const MC_CMD_PTP_IN_CLOCK_OFFSET_ADJUST_MAJOR_LEN: c_int = 4;
// Time adjustment in nanoseconds
pub const MC_CMD_PTP_IN_CLOCK_OFFSET_ADJUST_NANOSECONDS_OFST: c_int = 12;
pub const MC_CMD_PTP_IN_CLOCK_OFFSET_ADJUST_NANOSECONDS_LEN: c_int = 4;
// Time adjustment minor value
pub const MC_CMD_PTP_IN_CLOCK_OFFSET_ADJUST_MINOR_OFST: c_int = 12;
pub const MC_CMD_PTP_IN_CLOCK_OFFSET_ADJUST_MINOR_LEN: c_int = 4;
// MC_CMD_PTP_IN_CLOCK_OFFSET_ADJUST_V2 msgrequest
pub const MC_CMD_PTP_IN_CLOCK_OFFSET_ADJUST_V2_LEN: c_int = 20;
// MC_CMD_PTP_IN_CMD_OFST 0
// MC_CMD_PTP_IN_CMD_LEN 4
// MC_CMD_PTP_IN_PERIPH_ID_OFST 4
// MC_CMD_PTP_IN_PERIPH_ID_LEN 4
// Time adjustment in seconds
pub const MC_CMD_PTP_IN_CLOCK_OFFSET_ADJUST_V2_SECONDS_OFST: c_int = 8;
pub const MC_CMD_PTP_IN_CLOCK_OFFSET_ADJUST_V2_SECONDS_LEN: c_int = 4;
// Time adjustment major value
pub const MC_CMD_PTP_IN_CLOCK_OFFSET_ADJUST_V2_MAJOR_OFST: c_int = 8;
pub const MC_CMD_PTP_IN_CLOCK_OFFSET_ADJUST_V2_MAJOR_LEN: c_int = 4;
// Time adjustment in nanoseconds
pub const MC_CMD_PTP_IN_CLOCK_OFFSET_ADJUST_V2_NANOSECONDS_OFST: c_int = 12;
pub const MC_CMD_PTP_IN_CLOCK_OFFSET_ADJUST_V2_NANOSECONDS_LEN: c_int = 4;
// Time adjustment minor value
pub const MC_CMD_PTP_IN_CLOCK_OFFSET_ADJUST_V2_MINOR_OFST: c_int = 12;
pub const MC_CMD_PTP_IN_CLOCK_OFFSET_ADJUST_V2_MINOR_LEN: c_int = 4;
// Upper 32bits of major time offset adjustment
pub const MC_CMD_PTP_IN_CLOCK_OFFSET_ADJUST_V2_MAJOR_HI_OFST: c_int = 16;
pub const MC_CMD_PTP_IN_CLOCK_OFFSET_ADJUST_V2_MAJOR_HI_LEN: c_int = 4;
// MC_CMD_PTP_IN_CLOCK_FREQ_ADJUST msgrequest
pub const MC_CMD_PTP_IN_CLOCK_FREQ_ADJUST_LEN: c_int = 16;
// MC_CMD_PTP_IN_CMD_OFST 0
// MC_CMD_PTP_IN_CMD_LEN 4
// MC_CMD_PTP_IN_PERIPH_ID_OFST 4
// MC_CMD_PTP_IN_PERIPH_ID_LEN 4
// Frequency adjustment 40 bit fixed point ns
pub const MC_CMD_PTP_IN_CLOCK_FREQ_ADJUST_FREQ_OFST: c_int = 8;
pub const MC_CMD_PTP_IN_CLOCK_FREQ_ADJUST_FREQ_LEN: c_int = 8;
pub const MC_CMD_PTP_IN_CLOCK_FREQ_ADJUST_FREQ_LO_OFST: c_int = 8;
pub const MC_CMD_PTP_IN_CLOCK_FREQ_ADJUST_FREQ_HI_OFST: c_int = 12;
// Enum values, see field(s):
// MC_CMD_PTP/MC_CMD_PTP_IN_ADJUST/FREQ
// MC_CMD_PTP_IN_RX_SET_VLAN_FILTER msgrequest
pub const MC_CMD_PTP_IN_RX_SET_VLAN_FILTER_LEN: c_int = 24;
// MC_CMD_PTP_IN_CMD_OFST 0
// MC_CMD_PTP_IN_CMD_LEN 4
// MC_CMD_PTP_IN_PERIPH_ID_OFST 4
// MC_CMD_PTP_IN_PERIPH_ID_LEN 4
// Number of VLAN tags, 0 if not VLAN
pub const MC_CMD_PTP_IN_RX_SET_VLAN_FILTER_NUM_VLAN_TAGS_OFST: c_int = 8;
pub const MC_CMD_PTP_IN_RX_SET_VLAN_FILTER_NUM_VLAN_TAGS_LEN: c_int = 4;
// Set of VLAN tags to filter against
pub const MC_CMD_PTP_IN_RX_SET_VLAN_FILTER_VLAN_TAG_OFST: c_int = 12;
pub const MC_CMD_PTP_IN_RX_SET_VLAN_FILTER_VLAN_TAG_LEN: c_int = 4;
pub const MC_CMD_PTP_IN_RX_SET_VLAN_FILTER_VLAN_TAG_NUM: c_int = 3;
// MC_CMD_PTP_IN_RX_SET_UUID_FILTER msgrequest
pub const MC_CMD_PTP_IN_RX_SET_UUID_FILTER_LEN: c_int = 20;
// MC_CMD_PTP_IN_CMD_OFST 0
// MC_CMD_PTP_IN_CMD_LEN 4
// MC_CMD_PTP_IN_PERIPH_ID_OFST 4
// MC_CMD_PTP_IN_PERIPH_ID_LEN 4
// 1 to enable UUID filtering, 0 to disable
pub const MC_CMD_PTP_IN_RX_SET_UUID_FILTER_ENABLE_OFST: c_int = 8;
pub const MC_CMD_PTP_IN_RX_SET_UUID_FILTER_ENABLE_LEN: c_int = 4;
// UUID to filter against
pub const MC_CMD_PTP_IN_RX_SET_UUID_FILTER_UUID_OFST: c_int = 12;
pub const MC_CMD_PTP_IN_RX_SET_UUID_FILTER_UUID_LEN: c_int = 8;
pub const MC_CMD_PTP_IN_RX_SET_UUID_FILTER_UUID_LO_OFST: c_int = 12;
pub const MC_CMD_PTP_IN_RX_SET_UUID_FILTER_UUID_HI_OFST: c_int = 16;
// MC_CMD_PTP_IN_RX_SET_DOMAIN_FILTER msgrequest
pub const MC_CMD_PTP_IN_RX_SET_DOMAIN_FILTER_LEN: c_int = 16;
// MC_CMD_PTP_IN_CMD_OFST 0
// MC_CMD_PTP_IN_CMD_LEN 4
// MC_CMD_PTP_IN_PERIPH_ID_OFST 4
// MC_CMD_PTP_IN_PERIPH_ID_LEN 4
// 1 to enable Domain filtering, 0 to disable
pub const MC_CMD_PTP_IN_RX_SET_DOMAIN_FILTER_ENABLE_OFST: c_int = 8;
pub const MC_CMD_PTP_IN_RX_SET_DOMAIN_FILTER_ENABLE_LEN: c_int = 4;
// Domain number to filter against
pub const MC_CMD_PTP_IN_RX_SET_DOMAIN_FILTER_DOMAIN_OFST: c_int = 12;
pub const MC_CMD_PTP_IN_RX_SET_DOMAIN_FILTER_DOMAIN_LEN: c_int = 4;
// MC_CMD_PTP_IN_SET_CLK_SRC msgrequest
pub const MC_CMD_PTP_IN_SET_CLK_SRC_LEN: c_int = 12;
// MC_CMD_PTP_IN_CMD_OFST 0
// MC_CMD_PTP_IN_CMD_LEN 4
// MC_CMD_PTP_IN_PERIPH_ID_OFST 4
// MC_CMD_PTP_IN_PERIPH_ID_LEN 4
// Set the clock source.
pub const MC_CMD_PTP_IN_SET_CLK_SRC_CLK_OFST: c_int = 8;
pub const MC_CMD_PTP_IN_SET_CLK_SRC_CLK_LEN: c_int = 4;
// enum: Internal.
pub const MC_CMD_PTP_CLK_SRC_INTERNAL: c_uint = 0x0;
// enum: External.
pub const MC_CMD_PTP_CLK_SRC_EXTERNAL: c_uint = 0x1;
// MC_CMD_PTP_IN_RST_CLK msgrequest: Reset value of Timer Reg.
pub const MC_CMD_PTP_IN_RST_CLK_LEN: c_int = 8;
// MC_CMD_PTP_IN_CMD_OFST 0
// MC_CMD_PTP_IN_CMD_LEN 4
// MC_CMD_PTP_IN_PERIPH_ID_OFST 4
// MC_CMD_PTP_IN_PERIPH_ID_LEN 4
// MC_CMD_PTP_IN_PPS_ENABLE msgrequest
pub const MC_CMD_PTP_IN_PPS_ENABLE_LEN: c_int = 12;
// MC_CMD_PTP_IN_CMD_OFST 0
// MC_CMD_PTP_IN_CMD_LEN 4
// Enable or disable
pub const MC_CMD_PTP_IN_PPS_ENABLE_OP_OFST: c_int = 4;
pub const MC_CMD_PTP_IN_PPS_ENABLE_OP_LEN: c_int = 4;
// enum: Enable
pub const MC_CMD_PTP_ENABLE_PPS: c_uint = 0x0;
// enum: Disable
pub const MC_CMD_PTP_DISABLE_PPS: c_uint = 0x1;
// Not used. Events are always sent to function relative queue 0.
pub const MC_CMD_PTP_IN_PPS_ENABLE_QUEUE_ID_OFST: c_int = 8;
pub const MC_CMD_PTP_IN_PPS_ENABLE_QUEUE_ID_LEN: c_int = 4;
// MC_CMD_PTP_IN_GET_TIME_FORMAT msgrequest
pub const MC_CMD_PTP_IN_GET_TIME_FORMAT_LEN: c_int = 8;
// MC_CMD_PTP_IN_CMD_OFST 0
// MC_CMD_PTP_IN_CMD_LEN 4
// MC_CMD_PTP_IN_PERIPH_ID_OFST 4
// MC_CMD_PTP_IN_PERIPH_ID_LEN 4
// MC_CMD_PTP_IN_GET_ATTRIBUTES msgrequest
pub const MC_CMD_PTP_IN_GET_ATTRIBUTES_LEN: c_int = 8;
// MC_CMD_PTP_IN_CMD_OFST 0
// MC_CMD_PTP_IN_CMD_LEN 4
// MC_CMD_PTP_IN_PERIPH_ID_OFST 4
// MC_CMD_PTP_IN_PERIPH_ID_LEN 4
// MC_CMD_PTP_IN_GET_TIMESTAMP_CORRECTIONS msgrequest
pub const MC_CMD_PTP_IN_GET_TIMESTAMP_CORRECTIONS_LEN: c_int = 8;
// MC_CMD_PTP_IN_CMD_OFST 0
// MC_CMD_PTP_IN_CMD_LEN 4
// MC_CMD_PTP_IN_PERIPH_ID_OFST 4
// MC_CMD_PTP_IN_PERIPH_ID_LEN 4
// MC_CMD_PTP_IN_TIME_EVENT_SUBSCRIBE msgrequest
pub const MC_CMD_PTP_IN_TIME_EVENT_SUBSCRIBE_LEN: c_int = 12;
// MC_CMD_PTP_IN_CMD_OFST 0
// MC_CMD_PTP_IN_CMD_LEN 4
// MC_CMD_PTP_IN_PERIPH_ID_OFST 4
// MC_CMD_PTP_IN_PERIPH_ID_LEN 4
// Original field containing queue ID. Now extended to include flags.
pub const MC_CMD_PTP_IN_TIME_EVENT_SUBSCRIBE_QUEUE_OFST: c_int = 8;
pub const MC_CMD_PTP_IN_TIME_EVENT_SUBSCRIBE_QUEUE_LEN: c_int = 4;
pub const MC_CMD_PTP_IN_TIME_EVENT_SUBSCRIBE_QUEUE_ID_OFST: c_int = 8;
pub const MC_CMD_PTP_IN_TIME_EVENT_SUBSCRIBE_QUEUE_ID_LBN: c_int = 0;
pub const MC_CMD_PTP_IN_TIME_EVENT_SUBSCRIBE_QUEUE_ID_WIDTH: c_int = 16;
pub const MC_CMD_PTP_IN_TIME_EVENT_SUBSCRIBE_REPORT_SYNC_STATUS_OFST: c_int = 8;
pub const MC_CMD_PTP_IN_TIME_EVENT_SUBSCRIBE_REPORT_SYNC_STATUS_LBN: c_int = 31;
pub const MC_CMD_PTP_IN_TIME_EVENT_SUBSCRIBE_REPORT_SYNC_STATUS_WIDTH: c_int = 1;
// MC_CMD_PTP_IN_TIME_EVENT_UNSUBSCRIBE msgrequest
pub const MC_CMD_PTP_IN_TIME_EVENT_UNSUBSCRIBE_LEN: c_int = 16;
// MC_CMD_PTP_IN_CMD_OFST 0
// MC_CMD_PTP_IN_CMD_LEN 4
// MC_CMD_PTP_IN_PERIPH_ID_OFST 4
// MC_CMD_PTP_IN_PERIPH_ID_LEN 4
// Unsubscribe options
pub const MC_CMD_PTP_IN_TIME_EVENT_UNSUBSCRIBE_CONTROL_OFST: c_int = 8;
pub const MC_CMD_PTP_IN_TIME_EVENT_UNSUBSCRIBE_CONTROL_LEN: c_int = 4;
// enum: Unsubscribe a single queue
pub const MC_CMD_PTP_IN_TIME_EVENT_UNSUBSCRIBE_SINGLE: c_uint = 0x0;
// enum: Unsubscribe all queues
pub const MC_CMD_PTP_IN_TIME_EVENT_UNSUBSCRIBE_ALL: c_uint = 0x1;
// Event queue ID
pub const MC_CMD_PTP_IN_TIME_EVENT_UNSUBSCRIBE_QUEUE_OFST: c_int = 12;
pub const MC_CMD_PTP_IN_TIME_EVENT_UNSUBSCRIBE_QUEUE_LEN: c_int = 4;
// MC_CMD_PTP_IN_MANFTEST_PPS msgrequest
pub const MC_CMD_PTP_IN_MANFTEST_PPS_LEN: c_int = 12;
// MC_CMD_PTP_IN_CMD_OFST 0
// MC_CMD_PTP_IN_CMD_LEN 4
// MC_CMD_PTP_IN_PERIPH_ID_OFST 4
// MC_CMD_PTP_IN_PERIPH_ID_LEN 4
// 1 to enable PPS test mode, 0 to disable and return result.
pub const MC_CMD_PTP_IN_MANFTEST_PPS_TEST_ENABLE_OFST: c_int = 8;
pub const MC_CMD_PTP_IN_MANFTEST_PPS_TEST_ENABLE_LEN: c_int = 4;
// MC_CMD_PTP_IN_SET_SYNC_STATUS msgrequest
pub const MC_CMD_PTP_IN_SET_SYNC_STATUS_LEN: c_int = 24;
// MC_CMD_PTP_IN_CMD_OFST 0
// MC_CMD_PTP_IN_CMD_LEN 4
// MC_CMD_PTP_IN_PERIPH_ID_OFST 4
// MC_CMD_PTP_IN_PERIPH_ID_LEN 4
// NIC - Host System Clock Synchronization status
pub const MC_CMD_PTP_IN_SET_SYNC_STATUS_STATUS_OFST: c_int = 8;
pub const MC_CMD_PTP_IN_SET_SYNC_STATUS_STATUS_LEN: c_int = 4;
// enum: Host System clock and NIC clock are not in sync
pub const MC_CMD_PTP_IN_SET_SYNC_STATUS_NOT_IN_SYNC: c_uint = 0x0;
// enum: Host System clock and NIC clock are synchronized
pub const MC_CMD_PTP_IN_SET_SYNC_STATUS_IN_SYNC: c_uint = 0x1;
// If synchronized, number of seconds until clocks should be considered to be
// no longer in sync.
//
pub const MC_CMD_PTP_IN_SET_SYNC_STATUS_TIMEOUT_OFST: c_int = 12;
pub const MC_CMD_PTP_IN_SET_SYNC_STATUS_TIMEOUT_LEN: c_int = 4;
pub const MC_CMD_PTP_IN_SET_SYNC_STATUS_RESERVED0_OFST: c_int = 16;
pub const MC_CMD_PTP_IN_SET_SYNC_STATUS_RESERVED0_LEN: c_int = 4;
pub const MC_CMD_PTP_IN_SET_SYNC_STATUS_RESERVED1_OFST: c_int = 20;
pub const MC_CMD_PTP_IN_SET_SYNC_STATUS_RESERVED1_LEN: c_int = 4;
// MC_CMD_PTP_OUT msgresponse
pub const MC_CMD_PTP_OUT_LEN: c_int = 0;
// MC_CMD_PTP_OUT_TRANSMIT msgresponse
pub const MC_CMD_PTP_OUT_TRANSMIT_LEN: c_int = 8;
// Value of seconds timestamp
pub const MC_CMD_PTP_OUT_TRANSMIT_SECONDS_OFST: c_int = 0;
pub const MC_CMD_PTP_OUT_TRANSMIT_SECONDS_LEN: c_int = 4;
// Timestamp major value
pub const MC_CMD_PTP_OUT_TRANSMIT_MAJOR_OFST: c_int = 0;
pub const MC_CMD_PTP_OUT_TRANSMIT_MAJOR_LEN: c_int = 4;
// Value of nanoseconds timestamp
pub const MC_CMD_PTP_OUT_TRANSMIT_NANOSECONDS_OFST: c_int = 4;
pub const MC_CMD_PTP_OUT_TRANSMIT_NANOSECONDS_LEN: c_int = 4;
// Timestamp minor value
pub const MC_CMD_PTP_OUT_TRANSMIT_MINOR_OFST: c_int = 4;
pub const MC_CMD_PTP_OUT_TRANSMIT_MINOR_LEN: c_int = 4;
// MC_CMD_PTP_OUT_TIME_EVENT_SUBSCRIBE msgresponse
pub const MC_CMD_PTP_OUT_TIME_EVENT_SUBSCRIBE_LEN: c_int = 0;
// MC_CMD_PTP_OUT_TIME_EVENT_UNSUBSCRIBE msgresponse
pub const MC_CMD_PTP_OUT_TIME_EVENT_UNSUBSCRIBE_LEN: c_int = 0;
// MC_CMD_PTP_OUT_READ_NIC_TIME msgresponse
pub const MC_CMD_PTP_OUT_READ_NIC_TIME_LEN: c_int = 8;
// Value of seconds timestamp
pub const MC_CMD_PTP_OUT_READ_NIC_TIME_SECONDS_OFST: c_int = 0;
pub const MC_CMD_PTP_OUT_READ_NIC_TIME_SECONDS_LEN: c_int = 4;
// Timestamp major value
pub const MC_CMD_PTP_OUT_READ_NIC_TIME_MAJOR_OFST: c_int = 0;
pub const MC_CMD_PTP_OUT_READ_NIC_TIME_MAJOR_LEN: c_int = 4;
// Value of nanoseconds timestamp
pub const MC_CMD_PTP_OUT_READ_NIC_TIME_NANOSECONDS_OFST: c_int = 4;
pub const MC_CMD_PTP_OUT_READ_NIC_TIME_NANOSECONDS_LEN: c_int = 4;
// Timestamp minor value
pub const MC_CMD_PTP_OUT_READ_NIC_TIME_MINOR_OFST: c_int = 4;
pub const MC_CMD_PTP_OUT_READ_NIC_TIME_MINOR_LEN: c_int = 4;
// MC_CMD_PTP_OUT_READ_NIC_TIME_V2 msgresponse
pub const MC_CMD_PTP_OUT_READ_NIC_TIME_V2_LEN: c_int = 12;
// Value of seconds timestamp
pub const MC_CMD_PTP_OUT_READ_NIC_TIME_V2_SECONDS_OFST: c_int = 0;
pub const MC_CMD_PTP_OUT_READ_NIC_TIME_V2_SECONDS_LEN: c_int = 4;
// Timestamp major value
pub const MC_CMD_PTP_OUT_READ_NIC_TIME_V2_MAJOR_OFST: c_int = 0;
pub const MC_CMD_PTP_OUT_READ_NIC_TIME_V2_MAJOR_LEN: c_int = 4;
// Value of nanoseconds timestamp
pub const MC_CMD_PTP_OUT_READ_NIC_TIME_V2_NANOSECONDS_OFST: c_int = 4;
pub const MC_CMD_PTP_OUT_READ_NIC_TIME_V2_NANOSECONDS_LEN: c_int = 4;
// Timestamp minor value
pub const MC_CMD_PTP_OUT_READ_NIC_TIME_V2_MINOR_OFST: c_int = 4;
pub const MC_CMD_PTP_OUT_READ_NIC_TIME_V2_MINOR_LEN: c_int = 4;
// Upper 32bits of major timestamp value
pub const MC_CMD_PTP_OUT_READ_NIC_TIME_V2_MAJOR_HI_OFST: c_int = 8;
pub const MC_CMD_PTP_OUT_READ_NIC_TIME_V2_MAJOR_HI_LEN: c_int = 4;
// MC_CMD_PTP_OUT_STATUS msgresponse
pub const MC_CMD_PTP_OUT_STATUS_LEN: c_int = 64;
// Frequency of NIC's hardware clock
pub const MC_CMD_PTP_OUT_STATUS_CLOCK_FREQ_OFST: c_int = 0;
pub const MC_CMD_PTP_OUT_STATUS_CLOCK_FREQ_LEN: c_int = 4;
// Number of packets transmitted and timestamped
pub const MC_CMD_PTP_OUT_STATUS_STATS_TX_OFST: c_int = 4;
pub const MC_CMD_PTP_OUT_STATUS_STATS_TX_LEN: c_int = 4;
// Number of packets received and timestamped
pub const MC_CMD_PTP_OUT_STATUS_STATS_RX_OFST: c_int = 8;
pub const MC_CMD_PTP_OUT_STATUS_STATS_RX_LEN: c_int = 4;
// Number of packets timestamped by the FPGA
pub const MC_CMD_PTP_OUT_STATUS_STATS_TS_OFST: c_int = 12;
pub const MC_CMD_PTP_OUT_STATUS_STATS_TS_LEN: c_int = 4;
// Number of packets filter matched
pub const MC_CMD_PTP_OUT_STATUS_STATS_FM_OFST: c_int = 16;
pub const MC_CMD_PTP_OUT_STATUS_STATS_FM_LEN: c_int = 4;
// Number of packets not filter matched
pub const MC_CMD_PTP_OUT_STATUS_STATS_NFM_OFST: c_int = 20;
pub const MC_CMD_PTP_OUT_STATUS_STATS_NFM_LEN: c_int = 4;
// Number of PPS overflows (noise on input?)
pub const MC_CMD_PTP_OUT_STATUS_STATS_PPS_OFLOW_OFST: c_int = 24;
pub const MC_CMD_PTP_OUT_STATUS_STATS_PPS_OFLOW_LEN: c_int = 4;
// Number of PPS bad periods
pub const MC_CMD_PTP_OUT_STATUS_STATS_PPS_BAD_OFST: c_int = 28;
pub const MC_CMD_PTP_OUT_STATUS_STATS_PPS_BAD_LEN: c_int = 4;
// Minimum period of PPS pulse in nanoseconds
pub const MC_CMD_PTP_OUT_STATUS_STATS_PPS_PER_MIN_OFST: c_int = 32;
pub const MC_CMD_PTP_OUT_STATUS_STATS_PPS_PER_MIN_LEN: c_int = 4;
// Maximum period of PPS pulse in nanoseconds
pub const MC_CMD_PTP_OUT_STATUS_STATS_PPS_PER_MAX_OFST: c_int = 36;
pub const MC_CMD_PTP_OUT_STATUS_STATS_PPS_PER_MAX_LEN: c_int = 4;
// Last period of PPS pulse in nanoseconds
pub const MC_CMD_PTP_OUT_STATUS_STATS_PPS_PER_LAST_OFST: c_int = 40;
pub const MC_CMD_PTP_OUT_STATUS_STATS_PPS_PER_LAST_LEN: c_int = 4;
// Mean period of PPS pulse in nanoseconds
pub const MC_CMD_PTP_OUT_STATUS_STATS_PPS_PER_MEAN_OFST: c_int = 44;
pub const MC_CMD_PTP_OUT_STATUS_STATS_PPS_PER_MEAN_LEN: c_int = 4;
// Minimum offset of PPS pulse in nanoseconds (signed)
pub const MC_CMD_PTP_OUT_STATUS_STATS_PPS_OFF_MIN_OFST: c_int = 48;
pub const MC_CMD_PTP_OUT_STATUS_STATS_PPS_OFF_MIN_LEN: c_int = 4;
// Maximum offset of PPS pulse in nanoseconds (signed)
pub const MC_CMD_PTP_OUT_STATUS_STATS_PPS_OFF_MAX_OFST: c_int = 52;
pub const MC_CMD_PTP_OUT_STATUS_STATS_PPS_OFF_MAX_LEN: c_int = 4;
// Last offset of PPS pulse in nanoseconds (signed)
pub const MC_CMD_PTP_OUT_STATUS_STATS_PPS_OFF_LAST_OFST: c_int = 56;
pub const MC_CMD_PTP_OUT_STATUS_STATS_PPS_OFF_LAST_LEN: c_int = 4;
// Mean offset of PPS pulse in nanoseconds (signed)
pub const MC_CMD_PTP_OUT_STATUS_STATS_PPS_OFF_MEAN_OFST: c_int = 60;
pub const MC_CMD_PTP_OUT_STATUS_STATS_PPS_OFF_MEAN_LEN: c_int = 4;
// MC_CMD_PTP_OUT_SYNCHRONIZE msgresponse
pub const MC_CMD_PTP_OUT_SYNCHRONIZE_LENMIN: c_int = 20;
pub const MC_CMD_PTP_OUT_SYNCHRONIZE_LENMAX: c_int = 240;
pub const MC_CMD_PTP_OUT_SYNCHRONIZE_LENMAX_MCDI2: c_int = 1020;

// A set of host and NIC times
pub const MC_CMD_PTP_OUT_SYNCHRONIZE_TIMESET_OFST: c_int = 0;
pub const MC_CMD_PTP_OUT_SYNCHRONIZE_TIMESET_LEN: c_int = 20;
pub const MC_CMD_PTP_OUT_SYNCHRONIZE_TIMESET_MINNUM: c_int = 1;
pub const MC_CMD_PTP_OUT_SYNCHRONIZE_TIMESET_MAXNUM: c_int = 12;
pub const MC_CMD_PTP_OUT_SYNCHRONIZE_TIMESET_MAXNUM_MCDI2: c_int = 51;
// Host time immediately before NIC's hardware clock read
pub const MC_CMD_PTP_OUT_SYNCHRONIZE_HOSTSTART_OFST: c_int = 0;
pub const MC_CMD_PTP_OUT_SYNCHRONIZE_HOSTSTART_LEN: c_int = 4;
// Value of seconds timestamp
pub const MC_CMD_PTP_OUT_SYNCHRONIZE_SECONDS_OFST: c_int = 4;
pub const MC_CMD_PTP_OUT_SYNCHRONIZE_SECONDS_LEN: c_int = 4;
// Timestamp major value
pub const MC_CMD_PTP_OUT_SYNCHRONIZE_MAJOR_OFST: c_int = 4;
pub const MC_CMD_PTP_OUT_SYNCHRONIZE_MAJOR_LEN: c_int = 4;
// Value of nanoseconds timestamp
pub const MC_CMD_PTP_OUT_SYNCHRONIZE_NANOSECONDS_OFST: c_int = 8;
pub const MC_CMD_PTP_OUT_SYNCHRONIZE_NANOSECONDS_LEN: c_int = 4;
// Timestamp minor value
pub const MC_CMD_PTP_OUT_SYNCHRONIZE_MINOR_OFST: c_int = 8;
pub const MC_CMD_PTP_OUT_SYNCHRONIZE_MINOR_LEN: c_int = 4;
// Host time immediately after NIC's hardware clock read
pub const MC_CMD_PTP_OUT_SYNCHRONIZE_HOSTEND_OFST: c_int = 12;
pub const MC_CMD_PTP_OUT_SYNCHRONIZE_HOSTEND_LEN: c_int = 4;
// Number of nanoseconds waited after reading NIC's hardware clock
pub const MC_CMD_PTP_OUT_SYNCHRONIZE_WAITNS_OFST: c_int = 16;
pub const MC_CMD_PTP_OUT_SYNCHRONIZE_WAITNS_LEN: c_int = 4;
// MC_CMD_PTP_OUT_MANFTEST_BASIC msgresponse
pub const MC_CMD_PTP_OUT_MANFTEST_BASIC_LEN: c_int = 8;
// Results of testing
pub const MC_CMD_PTP_OUT_MANFTEST_BASIC_TEST_RESULT_OFST: c_int = 0;
pub const MC_CMD_PTP_OUT_MANFTEST_BASIC_TEST_RESULT_LEN: c_int = 4;
// enum: Successful test
pub const MC_CMD_PTP_MANF_SUCCESS: c_uint = 0x0;
// enum: FPGA load failed
pub const MC_CMD_PTP_MANF_FPGA_LOAD: c_uint = 0x1;
// enum: FPGA version invalid
pub const MC_CMD_PTP_MANF_FPGA_VERSION: c_uint = 0x2;
// enum: FPGA registers incorrect
pub const MC_CMD_PTP_MANF_FPGA_REGISTERS: c_uint = 0x3;
// enum: Oscillator possibly not working?
pub const MC_CMD_PTP_MANF_OSCILLATOR: c_uint = 0x4;
// enum: Timestamps not increasing
pub const MC_CMD_PTP_MANF_TIMESTAMPS: c_uint = 0x5;
// enum: Mismatched packet count
pub const MC_CMD_PTP_MANF_PACKET_COUNT: c_uint = 0x6;
// enum: Mismatched packet count (Siena filter and FPGA)
pub const MC_CMD_PTP_MANF_FILTER_COUNT: c_uint = 0x7;
// enum: Not enough packets to perform timestamp check
pub const MC_CMD_PTP_MANF_PACKET_ENOUGH: c_uint = 0x8;
// enum: Timestamp trigger GPIO not working
pub const MC_CMD_PTP_MANF_GPIO_TRIGGER: c_uint = 0x9;
// enum: Insufficient PPS events to perform checks
pub const MC_CMD_PTP_MANF_PPS_ENOUGH: c_uint = 0xa;
// enum: PPS time event period not sufficiently close to 1s.
pub const MC_CMD_PTP_MANF_PPS_PERIOD: c_uint = 0xb;
// enum: PPS time event nS reading not sufficiently close to zero.
pub const MC_CMD_PTP_MANF_PPS_NS: c_uint = 0xc;
// enum: PTP peripheral registers incorrect
pub const MC_CMD_PTP_MANF_REGISTERS: c_uint = 0xd;
// enum: Failed to read time from PTP peripheral
pub const MC_CMD_PTP_MANF_CLOCK_READ: c_uint = 0xe;
// Presence of external oscillator
pub const MC_CMD_PTP_OUT_MANFTEST_BASIC_TEST_EXTOSC_OFST: c_int = 4;
pub const MC_CMD_PTP_OUT_MANFTEST_BASIC_TEST_EXTOSC_LEN: c_int = 4;
// MC_CMD_PTP_OUT_MANFTEST_PACKET msgresponse
pub const MC_CMD_PTP_OUT_MANFTEST_PACKET_LEN: c_int = 12;
// Results of testing
pub const MC_CMD_PTP_OUT_MANFTEST_PACKET_TEST_RESULT_OFST: c_int = 0;
pub const MC_CMD_PTP_OUT_MANFTEST_PACKET_TEST_RESULT_LEN: c_int = 4;
// Number of packets received by FPGA
pub const MC_CMD_PTP_OUT_MANFTEST_PACKET_TEST_FPGACOUNT_OFST: c_int = 4;
pub const MC_CMD_PTP_OUT_MANFTEST_PACKET_TEST_FPGACOUNT_LEN: c_int = 4;
// Number of packets received by Siena filters
pub const MC_CMD_PTP_OUT_MANFTEST_PACKET_TEST_FILTERCOUNT_OFST: c_int = 8;
pub const MC_CMD_PTP_OUT_MANFTEST_PACKET_TEST_FILTERCOUNT_LEN: c_int = 4;
// MC_CMD_PTP_OUT_FPGAREAD msgresponse
pub const MC_CMD_PTP_OUT_FPGAREAD_LENMIN: c_int = 1;
pub const MC_CMD_PTP_OUT_FPGAREAD_LENMAX: c_int = 252;
pub const MC_CMD_PTP_OUT_FPGAREAD_LENMAX_MCDI2: c_int = 1020;

pub const MC_CMD_PTP_OUT_FPGAREAD_BUFFER_OFST: c_int = 0;
pub const MC_CMD_PTP_OUT_FPGAREAD_BUFFER_LEN: c_int = 1;
pub const MC_CMD_PTP_OUT_FPGAREAD_BUFFER_MINNUM: c_int = 1;
pub const MC_CMD_PTP_OUT_FPGAREAD_BUFFER_MAXNUM: c_int = 252;
pub const MC_CMD_PTP_OUT_FPGAREAD_BUFFER_MAXNUM_MCDI2: c_int = 1020;
// MC_CMD_PTP_OUT_GET_TIME_FORMAT msgresponse
pub const MC_CMD_PTP_OUT_GET_TIME_FORMAT_LEN: c_int = 4;
// Time format required/used by for this NIC. Applies to all PTP MCDI
// operations that pass times between the host and firmware. If this operation
// is not supported (older firmware) a format of seconds and nanoseconds should
// be assumed. Note this enum is deprecated. Do not add to it- use the
// TIME_FORMAT field in MC_CMD_PTP_OUT_GET_ATTRIBUTES instead.
//
pub const MC_CMD_PTP_OUT_GET_TIME_FORMAT_FORMAT_OFST: c_int = 0;
pub const MC_CMD_PTP_OUT_GET_TIME_FORMAT_FORMAT_LEN: c_int = 4;
// enum: Times are in seconds and nanoseconds
pub const MC_CMD_PTP_OUT_GET_TIME_FORMAT_SECONDS_NANOSECONDS: c_uint = 0x0;
// enum: Major register has units of 16 second per tick, minor 8 ns per tick
pub const MC_CMD_PTP_OUT_GET_TIME_FORMAT_16SECONDS_8NANOSECONDS: c_uint = 0x1;
// enum: Major register has units of seconds, minor 2^-27s per tick
pub const MC_CMD_PTP_OUT_GET_TIME_FORMAT_SECONDS_27FRACTION: c_uint = 0x2;
// MC_CMD_PTP_OUT_GET_ATTRIBUTES msgresponse
pub const MC_CMD_PTP_OUT_GET_ATTRIBUTES_LEN: c_int = 24;
// Time format required/used by for this NIC. Applies to all PTP MCDI
// operations that pass times between the host and firmware. If this operation
// is not supported (older firmware) a format of seconds and nanoseconds should
// be assumed.
//
pub const MC_CMD_PTP_OUT_GET_ATTRIBUTES_TIME_FORMAT_OFST: c_int = 0;
pub const MC_CMD_PTP_OUT_GET_ATTRIBUTES_TIME_FORMAT_LEN: c_int = 4;
// enum: Times are in seconds and nanoseconds
pub const MC_CMD_PTP_OUT_GET_ATTRIBUTES_SECONDS_NANOSECONDS: c_uint = 0x0;
// enum: Major register has units of 16 second per tick, minor 8 ns per tick
pub const MC_CMD_PTP_OUT_GET_ATTRIBUTES_16SECONDS_8NANOSECONDS: c_uint = 0x1;
// enum: Major register has units of seconds, minor 2^-27s per tick
pub const MC_CMD_PTP_OUT_GET_ATTRIBUTES_SECONDS_27FRACTION: c_uint = 0x2;
// enum: Major register units are seconds, minor units are quarter nanoseconds
//
pub const MC_CMD_PTP_OUT_GET_ATTRIBUTES_SECONDS_QTR_NANOSECONDS: c_uint = 0x3;
// Minimum acceptable value for a corrected synchronization timeset. When
// comparing host and NIC clock times, the MC returns a set of samples that
// contain the host start and end time, the MC time when the host start was
// detected and the time the MC waited between reading the time and detecting
// the host end. The corrected sync window is the difference between the host
// end and start times minus the time that the MC waited for host end.
//
pub const MC_CMD_PTP_OUT_GET_ATTRIBUTES_SYNC_WINDOW_MIN_OFST: c_int = 4;
pub const MC_CMD_PTP_OUT_GET_ATTRIBUTES_SYNC_WINDOW_MIN_LEN: c_int = 4;
// Various PTP capabilities
pub const MC_CMD_PTP_OUT_GET_ATTRIBUTES_CAPABILITIES_OFST: c_int = 8;
pub const MC_CMD_PTP_OUT_GET_ATTRIBUTES_CAPABILITIES_LEN: c_int = 4;
pub const MC_CMD_PTP_OUT_GET_ATTRIBUTES_REPORT_SYNC_STATUS_OFST: c_int = 8;
pub const MC_CMD_PTP_OUT_GET_ATTRIBUTES_REPORT_SYNC_STATUS_LBN: c_int = 0;
pub const MC_CMD_PTP_OUT_GET_ATTRIBUTES_REPORT_SYNC_STATUS_WIDTH: c_int = 1;
pub const MC_CMD_PTP_OUT_GET_ATTRIBUTES_RX_TSTAMP_OOB_OFST: c_int = 8;
pub const MC_CMD_PTP_OUT_GET_ATTRIBUTES_RX_TSTAMP_OOB_LBN: c_int = 1;
pub const MC_CMD_PTP_OUT_GET_ATTRIBUTES_RX_TSTAMP_OOB_WIDTH: c_int = 1;
pub const MC_CMD_PTP_OUT_GET_ATTRIBUTES_64BIT_SECONDS_OFST: c_int = 8;
pub const MC_CMD_PTP_OUT_GET_ATTRIBUTES_64BIT_SECONDS_LBN: c_int = 2;
pub const MC_CMD_PTP_OUT_GET_ATTRIBUTES_64BIT_SECONDS_WIDTH: c_int = 1;
pub const MC_CMD_PTP_OUT_GET_ATTRIBUTES_FP44_FREQ_ADJ_OFST: c_int = 8;
pub const MC_CMD_PTP_OUT_GET_ATTRIBUTES_FP44_FREQ_ADJ_LBN: c_int = 3;
pub const MC_CMD_PTP_OUT_GET_ATTRIBUTES_FP44_FREQ_ADJ_WIDTH: c_int = 1;
pub const MC_CMD_PTP_OUT_GET_ATTRIBUTES_RESERVED0_OFST: c_int = 12;
pub const MC_CMD_PTP_OUT_GET_ATTRIBUTES_RESERVED0_LEN: c_int = 4;
pub const MC_CMD_PTP_OUT_GET_ATTRIBUTES_RESERVED1_OFST: c_int = 16;
pub const MC_CMD_PTP_OUT_GET_ATTRIBUTES_RESERVED1_LEN: c_int = 4;
pub const MC_CMD_PTP_OUT_GET_ATTRIBUTES_RESERVED2_OFST: c_int = 20;
pub const MC_CMD_PTP_OUT_GET_ATTRIBUTES_RESERVED2_LEN: c_int = 4;
// MC_CMD_PTP_OUT_GET_TIMESTAMP_CORRECTIONS msgresponse
pub const MC_CMD_PTP_OUT_GET_TIMESTAMP_CORRECTIONS_LEN: c_int = 16;
// Uncorrected error on PTP transmit timestamps in NIC clock format
pub const MC_CMD_PTP_OUT_GET_TIMESTAMP_CORRECTIONS_TRANSMIT_OFST: c_int = 0;
pub const MC_CMD_PTP_OUT_GET_TIMESTAMP_CORRECTIONS_TRANSMIT_LEN: c_int = 4;
// Uncorrected error on PTP receive timestamps in NIC clock format
pub const MC_CMD_PTP_OUT_GET_TIMESTAMP_CORRECTIONS_RECEIVE_OFST: c_int = 4;
pub const MC_CMD_PTP_OUT_GET_TIMESTAMP_CORRECTIONS_RECEIVE_LEN: c_int = 4;
// Uncorrected error on PPS output in NIC clock format
pub const MC_CMD_PTP_OUT_GET_TIMESTAMP_CORRECTIONS_PPS_OUT_OFST: c_int = 8;
pub const MC_CMD_PTP_OUT_GET_TIMESTAMP_CORRECTIONS_PPS_OUT_LEN: c_int = 4;
// Uncorrected error on PPS input in NIC clock format
pub const MC_CMD_PTP_OUT_GET_TIMESTAMP_CORRECTIONS_PPS_IN_OFST: c_int = 12;
pub const MC_CMD_PTP_OUT_GET_TIMESTAMP_CORRECTIONS_PPS_IN_LEN: c_int = 4;
// MC_CMD_PTP_OUT_GET_TIMESTAMP_CORRECTIONS_V2 msgresponse
pub const MC_CMD_PTP_OUT_GET_TIMESTAMP_CORRECTIONS_V2_LEN: c_int = 24;
// Uncorrected error on PTP transmit timestamps in NIC clock format
pub const MC_CMD_PTP_OUT_GET_TIMESTAMP_CORRECTIONS_V2_PTP_TX_OFST: c_int = 0;
pub const MC_CMD_PTP_OUT_GET_TIMESTAMP_CORRECTIONS_V2_PTP_TX_LEN: c_int = 4;
// Uncorrected error on PTP receive timestamps in NIC clock format
pub const MC_CMD_PTP_OUT_GET_TIMESTAMP_CORRECTIONS_V2_PTP_RX_OFST: c_int = 4;
pub const MC_CMD_PTP_OUT_GET_TIMESTAMP_CORRECTIONS_V2_PTP_RX_LEN: c_int = 4;
// Uncorrected error on PPS output in NIC clock format
pub const MC_CMD_PTP_OUT_GET_TIMESTAMP_CORRECTIONS_V2_PPS_OUT_OFST: c_int = 8;
pub const MC_CMD_PTP_OUT_GET_TIMESTAMP_CORRECTIONS_V2_PPS_OUT_LEN: c_int = 4;
// Uncorrected error on PPS input in NIC clock format
pub const MC_CMD_PTP_OUT_GET_TIMESTAMP_CORRECTIONS_V2_PPS_IN_OFST: c_int = 12;
pub const MC_CMD_PTP_OUT_GET_TIMESTAMP_CORRECTIONS_V2_PPS_IN_LEN: c_int = 4;
// Uncorrected error on non-PTP transmit timestamps in NIC clock format
pub const MC_CMD_PTP_OUT_GET_TIMESTAMP_CORRECTIONS_V2_GENERAL_TX_OFST: c_int = 16;
pub const MC_CMD_PTP_OUT_GET_TIMESTAMP_CORRECTIONS_V2_GENERAL_TX_LEN: c_int = 4;
// Uncorrected error on non-PTP receive timestamps in NIC clock format
pub const MC_CMD_PTP_OUT_GET_TIMESTAMP_CORRECTIONS_V2_GENERAL_RX_OFST: c_int = 20;
pub const MC_CMD_PTP_OUT_GET_TIMESTAMP_CORRECTIONS_V2_GENERAL_RX_LEN: c_int = 4;
// MC_CMD_PTP_OUT_MANFTEST_PPS msgresponse
pub const MC_CMD_PTP_OUT_MANFTEST_PPS_LEN: c_int = 4;
// Results of testing
pub const MC_CMD_PTP_OUT_MANFTEST_PPS_TEST_RESULT_OFST: c_int = 0;
pub const MC_CMD_PTP_OUT_MANFTEST_PPS_TEST_RESULT_LEN: c_int = 4;
// Enum values, see field(s):
// MC_CMD_PTP_OUT_MANFTEST_BASIC/TEST_RESULT
// MC_CMD_PTP_OUT_SET_SYNC_STATUS msgresponse
pub const MC_CMD_PTP_OUT_SET_SYNC_STATUS_LEN: c_int = 0;
//
// MC_CMD_CSR_READ32
// Read 32bit words from the indirect memory map.
//
pub const MC_CMD_CSR_READ32: c_uint = 0xc;

// MC_CMD_CSR_READ32_IN msgrequest
pub const MC_CMD_CSR_READ32_IN_LEN: c_int = 12;
// Address
pub const MC_CMD_CSR_READ32_IN_ADDR_OFST: c_int = 0;
pub const MC_CMD_CSR_READ32_IN_ADDR_LEN: c_int = 4;
pub const MC_CMD_CSR_READ32_IN_STEP_OFST: c_int = 4;
pub const MC_CMD_CSR_READ32_IN_STEP_LEN: c_int = 4;
pub const MC_CMD_CSR_READ32_IN_NUMWORDS_OFST: c_int = 8;
pub const MC_CMD_CSR_READ32_IN_NUMWORDS_LEN: c_int = 4;
// MC_CMD_CSR_READ32_OUT msgresponse
pub const MC_CMD_CSR_READ32_OUT_LENMIN: c_int = 4;
pub const MC_CMD_CSR_READ32_OUT_LENMAX: c_int = 252;
pub const MC_CMD_CSR_READ32_OUT_LENMAX_MCDI2: c_int = 1020;

// The last dword is the status, not a value read
pub const MC_CMD_CSR_READ32_OUT_BUFFER_OFST: c_int = 0;
pub const MC_CMD_CSR_READ32_OUT_BUFFER_LEN: c_int = 4;
pub const MC_CMD_CSR_READ32_OUT_BUFFER_MINNUM: c_int = 1;
pub const MC_CMD_CSR_READ32_OUT_BUFFER_MAXNUM: c_int = 63;
pub const MC_CMD_CSR_READ32_OUT_BUFFER_MAXNUM_MCDI2: c_int = 255;
//
// MC_CMD_CSR_WRITE32
// Write 32bit dwords to the indirect memory map.
//
pub const MC_CMD_CSR_WRITE32: c_uint = 0xd;

// MC_CMD_CSR_WRITE32_IN msgrequest
pub const MC_CMD_CSR_WRITE32_IN_LENMIN: c_int = 12;
pub const MC_CMD_CSR_WRITE32_IN_LENMAX: c_int = 252;
pub const MC_CMD_CSR_WRITE32_IN_LENMAX_MCDI2: c_int = 1020;

// Address
pub const MC_CMD_CSR_WRITE32_IN_ADDR_OFST: c_int = 0;
pub const MC_CMD_CSR_WRITE32_IN_ADDR_LEN: c_int = 4;
pub const MC_CMD_CSR_WRITE32_IN_STEP_OFST: c_int = 4;
pub const MC_CMD_CSR_WRITE32_IN_STEP_LEN: c_int = 4;
pub const MC_CMD_CSR_WRITE32_IN_BUFFER_OFST: c_int = 8;
pub const MC_CMD_CSR_WRITE32_IN_BUFFER_LEN: c_int = 4;
pub const MC_CMD_CSR_WRITE32_IN_BUFFER_MINNUM: c_int = 1;
pub const MC_CMD_CSR_WRITE32_IN_BUFFER_MAXNUM: c_int = 61;
pub const MC_CMD_CSR_WRITE32_IN_BUFFER_MAXNUM_MCDI2: c_int = 253;
// MC_CMD_CSR_WRITE32_OUT msgresponse
pub const MC_CMD_CSR_WRITE32_OUT_LEN: c_int = 4;
pub const MC_CMD_CSR_WRITE32_OUT_STATUS_OFST: c_int = 0;
pub const MC_CMD_CSR_WRITE32_OUT_STATUS_LEN: c_int = 4;
//
// MC_CMD_HP
// These commands are used for HP related features. They are grouped under one
// MCDI command to avoid creating too many MCDI commands.
//
pub const MC_CMD_HP: c_uint = 0x54;

// MC_CMD_HP_IN msgrequest
pub const MC_CMD_HP_IN_LEN: c_int = 16;
// HP OCSD sub-command. When address is not NULL, request activation of OCSD at
// the specified address with the specified interval.When address is NULL,
// INTERVAL is interpreted as a command: 0: stop OCSD / 1: Report OCSD current
// state / 2: (debug) Show temperature reported by one of the supported
// sensors.
//
pub const MC_CMD_HP_IN_SUBCMD_OFST: c_int = 0;
pub const MC_CMD_HP_IN_SUBCMD_LEN: c_int = 4;
// enum: OCSD (Option Card Sensor Data) sub-command.
pub const MC_CMD_HP_IN_OCSD_SUBCMD: c_uint = 0x0;
// enum: Last known valid HP sub-command.
pub const MC_CMD_HP_IN_LAST_SUBCMD: c_uint = 0x0;
// The address to the array of sensor fields. (Or NULL to use a sub-command.)
//
pub const MC_CMD_HP_IN_OCSD_ADDR_OFST: c_int = 4;
pub const MC_CMD_HP_IN_OCSD_ADDR_LEN: c_int = 8;
pub const MC_CMD_HP_IN_OCSD_ADDR_LO_OFST: c_int = 4;
pub const MC_CMD_HP_IN_OCSD_ADDR_HI_OFST: c_int = 8;
// The requested update interval, in seconds. (Or the sub-command if ADDR is
// NULL.)
//
pub const MC_CMD_HP_IN_OCSD_INTERVAL_OFST: c_int = 12;
pub const MC_CMD_HP_IN_OCSD_INTERVAL_LEN: c_int = 4;
// MC_CMD_HP_OUT msgresponse
pub const MC_CMD_HP_OUT_LEN: c_int = 4;
pub const MC_CMD_HP_OUT_OCSD_STATUS_OFST: c_int = 0;
pub const MC_CMD_HP_OUT_OCSD_STATUS_LEN: c_int = 4;
// enum: OCSD stopped for this card.
pub const MC_CMD_HP_OUT_OCSD_STOPPED: c_uint = 0x1;
// enum: OCSD was successfully started with the address provided.
pub const MC_CMD_HP_OUT_OCSD_STARTED: c_uint = 0x2;
// enum: OCSD was already started for this card.
pub const MC_CMD_HP_OUT_OCSD_ALREADY_STARTED: c_uint = 0x3;
//
// MC_CMD_STACKINFO
// Get stack information.
//
pub const MC_CMD_STACKINFO: c_uint = 0xf;

// MC_CMD_STACKINFO_IN msgrequest
pub const MC_CMD_STACKINFO_IN_LEN: c_int = 0;
// MC_CMD_STACKINFO_OUT msgresponse
pub const MC_CMD_STACKINFO_OUT_LENMIN: c_int = 12;
pub const MC_CMD_STACKINFO_OUT_LENMAX: c_int = 252;
pub const MC_CMD_STACKINFO_OUT_LENMAX_MCDI2: c_int = 1020;

// (thread ptr, stack size, free space) for each thread in system
pub const MC_CMD_STACKINFO_OUT_THREAD_INFO_OFST: c_int = 0;
pub const MC_CMD_STACKINFO_OUT_THREAD_INFO_LEN: c_int = 12;
pub const MC_CMD_STACKINFO_OUT_THREAD_INFO_MINNUM: c_int = 1;
pub const MC_CMD_STACKINFO_OUT_THREAD_INFO_MAXNUM: c_int = 21;
pub const MC_CMD_STACKINFO_OUT_THREAD_INFO_MAXNUM_MCDI2: c_int = 85;
//
// MC_CMD_MDIO_READ
// MDIO register read.
//
pub const MC_CMD_MDIO_READ: c_uint = 0x10;

// MC_CMD_MDIO_READ_IN msgrequest
pub const MC_CMD_MDIO_READ_IN_LEN: c_int = 16;
// Bus number; there are two MDIO buses: one for the internal PHY, and one for
// external devices.
//
pub const MC_CMD_MDIO_READ_IN_BUS_OFST: c_int = 0;
pub const MC_CMD_MDIO_READ_IN_BUS_LEN: c_int = 4;
// enum: Internal.
pub const MC_CMD_MDIO_BUS_INTERNAL: c_uint = 0x0;
// enum: External.
pub const MC_CMD_MDIO_BUS_EXTERNAL: c_uint = 0x1;
// Port address
pub const MC_CMD_MDIO_READ_IN_PRTAD_OFST: c_int = 4;
pub const MC_CMD_MDIO_READ_IN_PRTAD_LEN: c_int = 4;
// Device Address or clause 22.
pub const MC_CMD_MDIO_READ_IN_DEVAD_OFST: c_int = 8;
pub const MC_CMD_MDIO_READ_IN_DEVAD_LEN: c_int = 4;
// enum: By default all the MCDI MDIO operations perform clause45 mode. If you
// want to use clause22 then set DEVAD = MC_CMD_MDIO_CLAUSE22.
//
pub const MC_CMD_MDIO_CLAUSE22: c_uint = 0x20;
// Address
pub const MC_CMD_MDIO_READ_IN_ADDR_OFST: c_int = 12;
pub const MC_CMD_MDIO_READ_IN_ADDR_LEN: c_int = 4;
// MC_CMD_MDIO_READ_OUT msgresponse
pub const MC_CMD_MDIO_READ_OUT_LEN: c_int = 8;
// Value
pub const MC_CMD_MDIO_READ_OUT_VALUE_OFST: c_int = 0;
pub const MC_CMD_MDIO_READ_OUT_VALUE_LEN: c_int = 4;
// Status the MDIO commands return the raw status bits from the MDIO block. A
// "good" transaction should have the DONE bit set and all other bits clear.
//
pub const MC_CMD_MDIO_READ_OUT_STATUS_OFST: c_int = 4;
pub const MC_CMD_MDIO_READ_OUT_STATUS_LEN: c_int = 4;
// enum: Good.
pub const MC_CMD_MDIO_STATUS_GOOD: c_uint = 0x8;
//
// MC_CMD_MDIO_WRITE
// MDIO register write.
//
pub const MC_CMD_MDIO_WRITE: c_uint = 0x11;

// MC_CMD_MDIO_WRITE_IN msgrequest
pub const MC_CMD_MDIO_WRITE_IN_LEN: c_int = 20;
// Bus number; there are two MDIO buses: one for the internal PHY, and one for
// external devices.
//
pub const MC_CMD_MDIO_WRITE_IN_BUS_OFST: c_int = 0;
pub const MC_CMD_MDIO_WRITE_IN_BUS_LEN: c_int = 4;
// enum: Internal.
// MC_CMD_MDIO_BUS_INTERNAL 0x0
// enum: External.
// MC_CMD_MDIO_BUS_EXTERNAL 0x1
// Port address
pub const MC_CMD_MDIO_WRITE_IN_PRTAD_OFST: c_int = 4;
pub const MC_CMD_MDIO_WRITE_IN_PRTAD_LEN: c_int = 4;
// Device Address or clause 22.
pub const MC_CMD_MDIO_WRITE_IN_DEVAD_OFST: c_int = 8;
pub const MC_CMD_MDIO_WRITE_IN_DEVAD_LEN: c_int = 4;
// enum: By default all the MCDI MDIO operations perform clause45 mode. If you
// want to use clause22 then set DEVAD = MC_CMD_MDIO_CLAUSE22.
//
// MC_CMD_MDIO_CLAUSE22 0x20
// Address
pub const MC_CMD_MDIO_WRITE_IN_ADDR_OFST: c_int = 12;
pub const MC_CMD_MDIO_WRITE_IN_ADDR_LEN: c_int = 4;
// Value
pub const MC_CMD_MDIO_WRITE_IN_VALUE_OFST: c_int = 16;
pub const MC_CMD_MDIO_WRITE_IN_VALUE_LEN: c_int = 4;
// MC_CMD_MDIO_WRITE_OUT msgresponse
pub const MC_CMD_MDIO_WRITE_OUT_LEN: c_int = 4;
// Status; the MDIO commands return the raw status bits from the MDIO block. A
// "good" transaction should have the DONE bit set and all other bits clear.
//
pub const MC_CMD_MDIO_WRITE_OUT_STATUS_OFST: c_int = 0;
pub const MC_CMD_MDIO_WRITE_OUT_STATUS_LEN: c_int = 4;
// enum: Good.
// MC_CMD_MDIO_STATUS_GOOD 0x8
//
// MC_CMD_DBI_WRITE
// Write DBI register(s).
//
pub const MC_CMD_DBI_WRITE: c_uint = 0x12;

// MC_CMD_DBI_WRITE_IN msgrequest
pub const MC_CMD_DBI_WRITE_IN_LENMIN: c_int = 12;
pub const MC_CMD_DBI_WRITE_IN_LENMAX: c_int = 252;
pub const MC_CMD_DBI_WRITE_IN_LENMAX_MCDI2: c_int = 1020;

// Each write op consists of an address (offset 0), byte enable/VF/CS2 (offset
// 32) and value (offset 64). See MC_CMD_DBIWROP_TYPEDEF.
//
pub const MC_CMD_DBI_WRITE_IN_DBIWROP_OFST: c_int = 0;
pub const MC_CMD_DBI_WRITE_IN_DBIWROP_LEN: c_int = 12;
pub const MC_CMD_DBI_WRITE_IN_DBIWROP_MINNUM: c_int = 1;
pub const MC_CMD_DBI_WRITE_IN_DBIWROP_MAXNUM: c_int = 21;
pub const MC_CMD_DBI_WRITE_IN_DBIWROP_MAXNUM_MCDI2: c_int = 85;
// MC_CMD_DBI_WRITE_OUT msgresponse
pub const MC_CMD_DBI_WRITE_OUT_LEN: c_int = 0;
// MC_CMD_DBIWROP_TYPEDEF structuredef
pub const MC_CMD_DBIWROP_TYPEDEF_LEN: c_int = 12;
pub const MC_CMD_DBIWROP_TYPEDEF_ADDRESS_OFST: c_int = 0;
pub const MC_CMD_DBIWROP_TYPEDEF_ADDRESS_LEN: c_int = 4;
pub const MC_CMD_DBIWROP_TYPEDEF_ADDRESS_LBN: c_int = 0;
pub const MC_CMD_DBIWROP_TYPEDEF_ADDRESS_WIDTH: c_int = 32;
pub const MC_CMD_DBIWROP_TYPEDEF_PARMS_OFST: c_int = 4;
pub const MC_CMD_DBIWROP_TYPEDEF_PARMS_LEN: c_int = 4;
pub const MC_CMD_DBIWROP_TYPEDEF_VF_NUM_OFST: c_int = 4;
pub const MC_CMD_DBIWROP_TYPEDEF_VF_NUM_LBN: c_int = 16;
pub const MC_CMD_DBIWROP_TYPEDEF_VF_NUM_WIDTH: c_int = 16;
pub const MC_CMD_DBIWROP_TYPEDEF_VF_ACTIVE_OFST: c_int = 4;
pub const MC_CMD_DBIWROP_TYPEDEF_VF_ACTIVE_LBN: c_int = 15;
pub const MC_CMD_DBIWROP_TYPEDEF_VF_ACTIVE_WIDTH: c_int = 1;
pub const MC_CMD_DBIWROP_TYPEDEF_CS2_OFST: c_int = 4;
pub const MC_CMD_DBIWROP_TYPEDEF_CS2_LBN: c_int = 14;
pub const MC_CMD_DBIWROP_TYPEDEF_CS2_WIDTH: c_int = 1;
pub const MC_CMD_DBIWROP_TYPEDEF_PARMS_LBN: c_int = 32;
pub const MC_CMD_DBIWROP_TYPEDEF_PARMS_WIDTH: c_int = 32;
pub const MC_CMD_DBIWROP_TYPEDEF_VALUE_OFST: c_int = 8;
pub const MC_CMD_DBIWROP_TYPEDEF_VALUE_LEN: c_int = 4;
pub const MC_CMD_DBIWROP_TYPEDEF_VALUE_LBN: c_int = 64;
pub const MC_CMD_DBIWROP_TYPEDEF_VALUE_WIDTH: c_int = 32;
//
// MC_CMD_PORT_READ32
// Read a 32-bit register from the indirect port register map. The port to
// access is implied by the Shared memory channel used.
//
pub const MC_CMD_PORT_READ32: c_uint = 0x14;
// MC_CMD_PORT_READ32_IN msgrequest
pub const MC_CMD_PORT_READ32_IN_LEN: c_int = 4;
// Address
pub const MC_CMD_PORT_READ32_IN_ADDR_OFST: c_int = 0;
pub const MC_CMD_PORT_READ32_IN_ADDR_LEN: c_int = 4;
// MC_CMD_PORT_READ32_OUT msgresponse
pub const MC_CMD_PORT_READ32_OUT_LEN: c_int = 8;
// Value
pub const MC_CMD_PORT_READ32_OUT_VALUE_OFST: c_int = 0;
pub const MC_CMD_PORT_READ32_OUT_VALUE_LEN: c_int = 4;
// Status
pub const MC_CMD_PORT_READ32_OUT_STATUS_OFST: c_int = 4;
pub const MC_CMD_PORT_READ32_OUT_STATUS_LEN: c_int = 4;
//
// MC_CMD_PORT_WRITE32
// Write a 32-bit register to the indirect port register map. The port to
// access is implied by the Shared memory channel used.
//
pub const MC_CMD_PORT_WRITE32: c_uint = 0x15;
// MC_CMD_PORT_WRITE32_IN msgrequest
pub const MC_CMD_PORT_WRITE32_IN_LEN: c_int = 8;
// Address
pub const MC_CMD_PORT_WRITE32_IN_ADDR_OFST: c_int = 0;
pub const MC_CMD_PORT_WRITE32_IN_ADDR_LEN: c_int = 4;
// Value
pub const MC_CMD_PORT_WRITE32_IN_VALUE_OFST: c_int = 4;
pub const MC_CMD_PORT_WRITE32_IN_VALUE_LEN: c_int = 4;
// MC_CMD_PORT_WRITE32_OUT msgresponse
pub const MC_CMD_PORT_WRITE32_OUT_LEN: c_int = 4;
// Status
pub const MC_CMD_PORT_WRITE32_OUT_STATUS_OFST: c_int = 0;
pub const MC_CMD_PORT_WRITE32_OUT_STATUS_LEN: c_int = 4;
//
// MC_CMD_PORT_READ128
// Read a 128-bit register from the indirect port register map. The port to
// access is implied by the Shared memory channel used.
//
pub const MC_CMD_PORT_READ128: c_uint = 0x16;
// MC_CMD_PORT_READ128_IN msgrequest
pub const MC_CMD_PORT_READ128_IN_LEN: c_int = 4;
// Address
pub const MC_CMD_PORT_READ128_IN_ADDR_OFST: c_int = 0;
pub const MC_CMD_PORT_READ128_IN_ADDR_LEN: c_int = 4;
// MC_CMD_PORT_READ128_OUT msgresponse
pub const MC_CMD_PORT_READ128_OUT_LEN: c_int = 20;
// Value
pub const MC_CMD_PORT_READ128_OUT_VALUE_OFST: c_int = 0;
pub const MC_CMD_PORT_READ128_OUT_VALUE_LEN: c_int = 16;
// Status
pub const MC_CMD_PORT_READ128_OUT_STATUS_OFST: c_int = 16;
pub const MC_CMD_PORT_READ128_OUT_STATUS_LEN: c_int = 4;
//
// MC_CMD_PORT_WRITE128
// Write a 128-bit register to the indirect port register map. The port to
// access is implied by the Shared memory channel used.
//
pub const MC_CMD_PORT_WRITE128: c_uint = 0x17;
// MC_CMD_PORT_WRITE128_IN msgrequest
pub const MC_CMD_PORT_WRITE128_IN_LEN: c_int = 20;
// Address
pub const MC_CMD_PORT_WRITE128_IN_ADDR_OFST: c_int = 0;
pub const MC_CMD_PORT_WRITE128_IN_ADDR_LEN: c_int = 4;
// Value
pub const MC_CMD_PORT_WRITE128_IN_VALUE_OFST: c_int = 4;
pub const MC_CMD_PORT_WRITE128_IN_VALUE_LEN: c_int = 16;
// MC_CMD_PORT_WRITE128_OUT msgresponse
pub const MC_CMD_PORT_WRITE128_OUT_LEN: c_int = 4;
// Status
pub const MC_CMD_PORT_WRITE128_OUT_STATUS_OFST: c_int = 0;
pub const MC_CMD_PORT_WRITE128_OUT_STATUS_LEN: c_int = 4;
// MC_CMD_CAPABILITIES structuredef
pub const MC_CMD_CAPABILITIES_LEN: c_int = 4;
// Small buf table.
pub const MC_CMD_CAPABILITIES_SMALL_BUF_TBL_LBN: c_int = 0;
pub const MC_CMD_CAPABILITIES_SMALL_BUF_TBL_WIDTH: c_int = 1;
// Turbo mode (for Maranello).
pub const MC_CMD_CAPABILITIES_TURBO_LBN: c_int = 1;
pub const MC_CMD_CAPABILITIES_TURBO_WIDTH: c_int = 1;
// Turbo mode active (for Maranello).
pub const MC_CMD_CAPABILITIES_TURBO_ACTIVE_LBN: c_int = 2;
pub const MC_CMD_CAPABILITIES_TURBO_ACTIVE_WIDTH: c_int = 1;
// PTP offload.
pub const MC_CMD_CAPABILITIES_PTP_LBN: c_int = 3;
pub const MC_CMD_CAPABILITIES_PTP_WIDTH: c_int = 1;
// AOE mode.
pub const MC_CMD_CAPABILITIES_AOE_LBN: c_int = 4;
pub const MC_CMD_CAPABILITIES_AOE_WIDTH: c_int = 1;
// AOE mode active.
pub const MC_CMD_CAPABILITIES_AOE_ACTIVE_LBN: c_int = 5;
pub const MC_CMD_CAPABILITIES_AOE_ACTIVE_WIDTH: c_int = 1;
// AOE mode active.
pub const MC_CMD_CAPABILITIES_FC_ACTIVE_LBN: c_int = 6;
pub const MC_CMD_CAPABILITIES_FC_ACTIVE_WIDTH: c_int = 1;
pub const MC_CMD_CAPABILITIES_RESERVED_LBN: c_int = 7;
pub const MC_CMD_CAPABILITIES_RESERVED_WIDTH: c_int = 25;
//
// MC_CMD_GET_BOARD_CFG
// Returns the MC firmware configuration structure.
//
pub const MC_CMD_GET_BOARD_CFG: c_uint = 0x18;

// MC_CMD_GET_BOARD_CFG_IN msgrequest
pub const MC_CMD_GET_BOARD_CFG_IN_LEN: c_int = 0;
// MC_CMD_GET_BOARD_CFG_OUT msgresponse
pub const MC_CMD_GET_BOARD_CFG_OUT_LENMIN: c_int = 96;
pub const MC_CMD_GET_BOARD_CFG_OUT_LENMAX: c_int = 136;
pub const MC_CMD_GET_BOARD_CFG_OUT_LENMAX_MCDI2: c_int = 136;

pub const MC_CMD_GET_BOARD_CFG_OUT_BOARD_TYPE_OFST: c_int = 0;
pub const MC_CMD_GET_BOARD_CFG_OUT_BOARD_TYPE_LEN: c_int = 4;
pub const MC_CMD_GET_BOARD_CFG_OUT_BOARD_NAME_OFST: c_int = 4;
pub const MC_CMD_GET_BOARD_CFG_OUT_BOARD_NAME_LEN: c_int = 32;
// Capabilities for Siena Port0 (see struct MC_CMD_CAPABILITIES). Unused on
// EF10 and later (use MC_CMD_GET_CAPABILITIES).
//
pub const MC_CMD_GET_BOARD_CFG_OUT_CAPABILITIES_PORT0_OFST: c_int = 36;
pub const MC_CMD_GET_BOARD_CFG_OUT_CAPABILITIES_PORT0_LEN: c_int = 4;
// Capabilities for Siena Port1 (see struct MC_CMD_CAPABILITIES). Unused on
// EF10 and later (use MC_CMD_GET_CAPABILITIES).
//
pub const MC_CMD_GET_BOARD_CFG_OUT_CAPABILITIES_PORT1_OFST: c_int = 40;
pub const MC_CMD_GET_BOARD_CFG_OUT_CAPABILITIES_PORT1_LEN: c_int = 4;
// Base MAC address for Siena Port0. Unused on EF10 and later (use
// MC_CMD_GET_MAC_ADDRESSES).
//
pub const MC_CMD_GET_BOARD_CFG_OUT_MAC_ADDR_BASE_PORT0_OFST: c_int = 44;
pub const MC_CMD_GET_BOARD_CFG_OUT_MAC_ADDR_BASE_PORT0_LEN: c_int = 6;
// Base MAC address for Siena Port1. Unused on EF10 and later (use
// MC_CMD_GET_MAC_ADDRESSES).
//
pub const MC_CMD_GET_BOARD_CFG_OUT_MAC_ADDR_BASE_PORT1_OFST: c_int = 50;
pub const MC_CMD_GET_BOARD_CFG_OUT_MAC_ADDR_BASE_PORT1_LEN: c_int = 6;
// Size of MAC address pool for Siena Port0. Unused on EF10 and later (use
// MC_CMD_GET_MAC_ADDRESSES).
//
pub const MC_CMD_GET_BOARD_CFG_OUT_MAC_COUNT_PORT0_OFST: c_int = 56;
pub const MC_CMD_GET_BOARD_CFG_OUT_MAC_COUNT_PORT0_LEN: c_int = 4;
// Size of MAC address pool for Siena Port1. Unused on EF10 and later (use
// MC_CMD_GET_MAC_ADDRESSES).
//
pub const MC_CMD_GET_BOARD_CFG_OUT_MAC_COUNT_PORT1_OFST: c_int = 60;
pub const MC_CMD_GET_BOARD_CFG_OUT_MAC_COUNT_PORT1_LEN: c_int = 4;
// Increment between addresses in MAC address pool for Siena Port0. Unused on
// EF10 and later (use MC_CMD_GET_MAC_ADDRESSES).
//
pub const MC_CMD_GET_BOARD_CFG_OUT_MAC_STRIDE_PORT0_OFST: c_int = 64;
pub const MC_CMD_GET_BOARD_CFG_OUT_MAC_STRIDE_PORT0_LEN: c_int = 4;
// Increment between addresses in MAC address pool for Siena Port1. Unused on
// EF10 and later (use MC_CMD_GET_MAC_ADDRESSES).
//
pub const MC_CMD_GET_BOARD_CFG_OUT_MAC_STRIDE_PORT1_OFST: c_int = 68;
pub const MC_CMD_GET_BOARD_CFG_OUT_MAC_STRIDE_PORT1_LEN: c_int = 4;
// Siena only. This field contains a 16-bit value for each of the types of
// NVRAM area. The values are defined in the firmware/mc/platform/.c file for a
// specific board type, but otherwise have no meaning to the MC; they are used
// by the driver to manage selection of appropriate firmware updates. Unused on
// EF10 and later (use MC_CMD_NVRAM_METADATA).
//
pub const MC_CMD_GET_BOARD_CFG_OUT_FW_SUBTYPE_LIST_OFST: c_int = 72;
pub const MC_CMD_GET_BOARD_CFG_OUT_FW_SUBTYPE_LIST_LEN: c_int = 2;
pub const MC_CMD_GET_BOARD_CFG_OUT_FW_SUBTYPE_LIST_MINNUM: c_int = 12;
pub const MC_CMD_GET_BOARD_CFG_OUT_FW_SUBTYPE_LIST_MAXNUM: c_int = 32;
pub const MC_CMD_GET_BOARD_CFG_OUT_FW_SUBTYPE_LIST_MAXNUM_MCDI2: c_int = 32;
//
// MC_CMD_DBI_READX
// Read DBI register(s) -- extended functionality
//
pub const MC_CMD_DBI_READX: c_uint = 0x19;

// MC_CMD_DBI_READX_IN msgrequest
pub const MC_CMD_DBI_READX_IN_LENMIN: c_int = 8;
pub const MC_CMD_DBI_READX_IN_LENMAX: c_int = 248;
pub const MC_CMD_DBI_READX_IN_LENMAX_MCDI2: c_int = 1016;

// Each Read op consists of an address (offset 0), VF/CS2)
pub const MC_CMD_DBI_READX_IN_DBIRDOP_OFST: c_int = 0;
pub const MC_CMD_DBI_READX_IN_DBIRDOP_LEN: c_int = 8;
pub const MC_CMD_DBI_READX_IN_DBIRDOP_LO_OFST: c_int = 0;
pub const MC_CMD_DBI_READX_IN_DBIRDOP_HI_OFST: c_int = 4;
pub const MC_CMD_DBI_READX_IN_DBIRDOP_MINNUM: c_int = 1;
pub const MC_CMD_DBI_READX_IN_DBIRDOP_MAXNUM: c_int = 31;
pub const MC_CMD_DBI_READX_IN_DBIRDOP_MAXNUM_MCDI2: c_int = 127;
// MC_CMD_DBI_READX_OUT msgresponse
pub const MC_CMD_DBI_READX_OUT_LENMIN: c_int = 4;
pub const MC_CMD_DBI_READX_OUT_LENMAX: c_int = 252;
pub const MC_CMD_DBI_READX_OUT_LENMAX_MCDI2: c_int = 1020;

// Value
pub const MC_CMD_DBI_READX_OUT_VALUE_OFST: c_int = 0;
pub const MC_CMD_DBI_READX_OUT_VALUE_LEN: c_int = 4;
pub const MC_CMD_DBI_READX_OUT_VALUE_MINNUM: c_int = 1;
pub const MC_CMD_DBI_READX_OUT_VALUE_MAXNUM: c_int = 63;
pub const MC_CMD_DBI_READX_OUT_VALUE_MAXNUM_MCDI2: c_int = 255;
// MC_CMD_DBIRDOP_TYPEDEF structuredef
pub const MC_CMD_DBIRDOP_TYPEDEF_LEN: c_int = 8;
pub const MC_CMD_DBIRDOP_TYPEDEF_ADDRESS_OFST: c_int = 0;
pub const MC_CMD_DBIRDOP_TYPEDEF_ADDRESS_LEN: c_int = 4;
pub const MC_CMD_DBIRDOP_TYPEDEF_ADDRESS_LBN: c_int = 0;
pub const MC_CMD_DBIRDOP_TYPEDEF_ADDRESS_WIDTH: c_int = 32;
pub const MC_CMD_DBIRDOP_TYPEDEF_PARMS_OFST: c_int = 4;
pub const MC_CMD_DBIRDOP_TYPEDEF_PARMS_LEN: c_int = 4;
pub const MC_CMD_DBIRDOP_TYPEDEF_VF_NUM_OFST: c_int = 4;
pub const MC_CMD_DBIRDOP_TYPEDEF_VF_NUM_LBN: c_int = 16;
pub const MC_CMD_DBIRDOP_TYPEDEF_VF_NUM_WIDTH: c_int = 16;
pub const MC_CMD_DBIRDOP_TYPEDEF_VF_ACTIVE_OFST: c_int = 4;
pub const MC_CMD_DBIRDOP_TYPEDEF_VF_ACTIVE_LBN: c_int = 15;
pub const MC_CMD_DBIRDOP_TYPEDEF_VF_ACTIVE_WIDTH: c_int = 1;
pub const MC_CMD_DBIRDOP_TYPEDEF_CS2_OFST: c_int = 4;
pub const MC_CMD_DBIRDOP_TYPEDEF_CS2_LBN: c_int = 14;
pub const MC_CMD_DBIRDOP_TYPEDEF_CS2_WIDTH: c_int = 1;
pub const MC_CMD_DBIRDOP_TYPEDEF_PARMS_LBN: c_int = 32;
pub const MC_CMD_DBIRDOP_TYPEDEF_PARMS_WIDTH: c_int = 32;
//
// MC_CMD_SET_RAND_SEED
// Set the 16byte seed for the MC pseudo-random generator.
//
pub const MC_CMD_SET_RAND_SEED: c_uint = 0x1a;

// MC_CMD_SET_RAND_SEED_IN msgrequest
pub const MC_CMD_SET_RAND_SEED_IN_LEN: c_int = 16;
// Seed value.
pub const MC_CMD_SET_RAND_SEED_IN_SEED_OFST: c_int = 0;
pub const MC_CMD_SET_RAND_SEED_IN_SEED_LEN: c_int = 16;
// MC_CMD_SET_RAND_SEED_OUT msgresponse
pub const MC_CMD_SET_RAND_SEED_OUT_LEN: c_int = 0;
//
// MC_CMD_LTSSM_HIST
// Retrieve the history of the LTSSM, if the build supports it.
//
pub const MC_CMD_LTSSM_HIST: c_uint = 0x1b;
// MC_CMD_LTSSM_HIST_IN msgrequest
pub const MC_CMD_LTSSM_HIST_IN_LEN: c_int = 0;
// MC_CMD_LTSSM_HIST_OUT msgresponse
pub const MC_CMD_LTSSM_HIST_OUT_LENMIN: c_int = 0;
pub const MC_CMD_LTSSM_HIST_OUT_LENMAX: c_int = 252;
pub const MC_CMD_LTSSM_HIST_OUT_LENMAX_MCDI2: c_int = 1020;

// variable number of LTSSM values, as bytes. The history is read-to-clear.
pub const MC_CMD_LTSSM_HIST_OUT_DATA_OFST: c_int = 0;
pub const MC_CMD_LTSSM_HIST_OUT_DATA_LEN: c_int = 4;
pub const MC_CMD_LTSSM_HIST_OUT_DATA_MINNUM: c_int = 0;
pub const MC_CMD_LTSSM_HIST_OUT_DATA_MAXNUM: c_int = 63;
pub const MC_CMD_LTSSM_HIST_OUT_DATA_MAXNUM_MCDI2: c_int = 255;
//
// MC_CMD_DRV_ATTACH
// Inform MCPU that this port is managed on the host (i.e. driver active). For
// Huntington, also request the preferred datapath firmware to use if possible
// (it may not be possible for this request to be fulfilled; the driver must
// issue a subsequent MC_CMD_GET_CAPABILITIES command to determine which
// features are actually available). The FIRMWARE_ID field is ignored by older
// platforms.
//
pub const MC_CMD_DRV_ATTACH: c_uint = 0x1c;

// MC_CMD_DRV_ATTACH_IN msgrequest
pub const MC_CMD_DRV_ATTACH_IN_LEN: c_int = 12;
// new state to set if UPDATE=1
pub const MC_CMD_DRV_ATTACH_IN_NEW_STATE_OFST: c_int = 0;
pub const MC_CMD_DRV_ATTACH_IN_NEW_STATE_LEN: c_int = 4;
pub const MC_CMD_DRV_ATTACH_OFST: c_int = 0;
pub const MC_CMD_DRV_ATTACH_LBN: c_int = 0;
pub const MC_CMD_DRV_ATTACH_WIDTH: c_int = 1;
pub const MC_CMD_DRV_ATTACH_IN_ATTACH_OFST: c_int = 0;
pub const MC_CMD_DRV_ATTACH_IN_ATTACH_LBN: c_int = 0;
pub const MC_CMD_DRV_ATTACH_IN_ATTACH_WIDTH: c_int = 1;
pub const MC_CMD_DRV_PREBOOT_OFST: c_int = 0;
pub const MC_CMD_DRV_PREBOOT_LBN: c_int = 1;
pub const MC_CMD_DRV_PREBOOT_WIDTH: c_int = 1;
pub const MC_CMD_DRV_ATTACH_IN_PREBOOT_OFST: c_int = 0;
pub const MC_CMD_DRV_ATTACH_IN_PREBOOT_LBN: c_int = 1;
pub const MC_CMD_DRV_ATTACH_IN_PREBOOT_WIDTH: c_int = 1;
pub const MC_CMD_DRV_ATTACH_IN_SUBVARIANT_AWARE_OFST: c_int = 0;
pub const MC_CMD_DRV_ATTACH_IN_SUBVARIANT_AWARE_LBN: c_int = 2;
pub const MC_CMD_DRV_ATTACH_IN_SUBVARIANT_AWARE_WIDTH: c_int = 1;
pub const MC_CMD_DRV_ATTACH_IN_WANT_VI_SPREADING_OFST: c_int = 0;
pub const MC_CMD_DRV_ATTACH_IN_WANT_VI_SPREADING_LBN: c_int = 3;
pub const MC_CMD_DRV_ATTACH_IN_WANT_VI_SPREADING_WIDTH: c_int = 1;
pub const MC_CMD_DRV_ATTACH_IN_WANT_V2_LINKCHANGES_OFST: c_int = 0;
pub const MC_CMD_DRV_ATTACH_IN_WANT_V2_LINKCHANGES_LBN: c_int = 4;
pub const MC_CMD_DRV_ATTACH_IN_WANT_V2_LINKCHANGES_WIDTH: c_int = 1;
pub const MC_CMD_DRV_ATTACH_IN_WANT_RX_VI_SPREADING_INHIBIT_OFST: c_int = 0;
pub const MC_CMD_DRV_ATTACH_IN_WANT_RX_VI_SPREADING_INHIBIT_LBN: c_int = 5;
pub const MC_CMD_DRV_ATTACH_IN_WANT_RX_VI_SPREADING_INHIBIT_WIDTH: c_int = 1;
pub const MC_CMD_DRV_ATTACH_IN_WANT_TX_ONLY_SPREADING_OFST: c_int = 0;
pub const MC_CMD_DRV_ATTACH_IN_WANT_TX_ONLY_SPREADING_LBN: c_int = 5;
pub const MC_CMD_DRV_ATTACH_IN_WANT_TX_ONLY_SPREADING_WIDTH: c_int = 1;
// 1 to set new state, or 0 to just report the existing state
pub const MC_CMD_DRV_ATTACH_IN_UPDATE_OFST: c_int = 4;
pub const MC_CMD_DRV_ATTACH_IN_UPDATE_LEN: c_int = 4;
// preferred datapath firmware (for Huntington; ignored for Siena)
pub const MC_CMD_DRV_ATTACH_IN_FIRMWARE_ID_OFST: c_int = 8;
pub const MC_CMD_DRV_ATTACH_IN_FIRMWARE_ID_LEN: c_int = 4;
// enum: Prefer to use full featured firmware
pub const MC_CMD_FW_FULL_FEATURED: c_uint = 0x0;
// enum: Prefer to use firmware with fewer features but lower latency
pub const MC_CMD_FW_LOW_LATENCY: c_uint = 0x1;
// enum: Prefer to use firmware for SolarCapture packed stream mode
pub const MC_CMD_FW_PACKED_STREAM: c_uint = 0x2;
// enum: Prefer to use firmware with fewer features and simpler TX event
// batching but higher TX packet rate
//
pub const MC_CMD_FW_HIGH_TX_RATE: c_uint = 0x3;
// enum: Reserved value
pub const MC_CMD_FW_PACKED_STREAM_HASH_MODE_1: c_uint = 0x4;
// enum: Prefer to use firmware with additional "rules engine" filtering
// support
//
pub const MC_CMD_FW_RULES_ENGINE: c_uint = 0x5;
// enum: Prefer to use firmware with additional DPDK support
pub const MC_CMD_FW_DPDK: c_uint = 0x6;
// enum: Prefer to use "l3xudp" custom datapath firmware (see SF-119495-PD and
// bug69716)
//
pub const MC_CMD_FW_L3XUDP: c_uint = 0x7;
// enum: Requests that the MC keep whatever datapath firmware is currently
// running. It's used for test purposes, where we want to be able to shmboot
// special test firmware variants. This option is only recognised in eftest
// (i.e. non-production) builds.
//
pub const MC_CMD_FW_KEEP_CURRENT_EFTEST_ONLY: c_uint = 0xfffffffe;
// enum: Only this option is allowed for non-admin functions
pub const MC_CMD_FW_DONT_CARE: c_uint = 0xffffffff;
// MC_CMD_DRV_ATTACH_IN_V2 msgrequest: Updated DRV_ATTACH to include driver
// version
//
pub const MC_CMD_DRV_ATTACH_IN_V2_LEN: c_int = 32;
// new state to set if UPDATE=1
pub const MC_CMD_DRV_ATTACH_IN_V2_NEW_STATE_OFST: c_int = 0;
pub const MC_CMD_DRV_ATTACH_IN_V2_NEW_STATE_LEN: c_int = 4;
// MC_CMD_DRV_ATTACH_OFST 0
// MC_CMD_DRV_ATTACH_LBN 0
// MC_CMD_DRV_ATTACH_WIDTH 1
pub const MC_CMD_DRV_ATTACH_IN_V2_ATTACH_OFST: c_int = 0;
pub const MC_CMD_DRV_ATTACH_IN_V2_ATTACH_LBN: c_int = 0;
pub const MC_CMD_DRV_ATTACH_IN_V2_ATTACH_WIDTH: c_int = 1;
// MC_CMD_DRV_PREBOOT_OFST 0
// MC_CMD_DRV_PREBOOT_LBN 1
// MC_CMD_DRV_PREBOOT_WIDTH 1
pub const MC_CMD_DRV_ATTACH_IN_V2_PREBOOT_OFST: c_int = 0;
pub const MC_CMD_DRV_ATTACH_IN_V2_PREBOOT_LBN: c_int = 1;
pub const MC_CMD_DRV_ATTACH_IN_V2_PREBOOT_WIDTH: c_int = 1;
pub const MC_CMD_DRV_ATTACH_IN_V2_SUBVARIANT_AWARE_OFST: c_int = 0;
pub const MC_CMD_DRV_ATTACH_IN_V2_SUBVARIANT_AWARE_LBN: c_int = 2;
pub const MC_CMD_DRV_ATTACH_IN_V2_SUBVARIANT_AWARE_WIDTH: c_int = 1;
pub const MC_CMD_DRV_ATTACH_IN_V2_WANT_VI_SPREADING_OFST: c_int = 0;
pub const MC_CMD_DRV_ATTACH_IN_V2_WANT_VI_SPREADING_LBN: c_int = 3;
pub const MC_CMD_DRV_ATTACH_IN_V2_WANT_VI_SPREADING_WIDTH: c_int = 1;
pub const MC_CMD_DRV_ATTACH_IN_V2_WANT_V2_LINKCHANGES_OFST: c_int = 0;
pub const MC_CMD_DRV_ATTACH_IN_V2_WANT_V2_LINKCHANGES_LBN: c_int = 4;
pub const MC_CMD_DRV_ATTACH_IN_V2_WANT_V2_LINKCHANGES_WIDTH: c_int = 1;
pub const MC_CMD_DRV_ATTACH_IN_V2_WANT_RX_VI_SPREADING_INHIBIT_OFST: c_int = 0;
pub const MC_CMD_DRV_ATTACH_IN_V2_WANT_RX_VI_SPREADING_INHIBIT_LBN: c_int = 5;
pub const MC_CMD_DRV_ATTACH_IN_V2_WANT_RX_VI_SPREADING_INHIBIT_WIDTH: c_int = 1;
pub const MC_CMD_DRV_ATTACH_IN_V2_WANT_TX_ONLY_SPREADING_OFST: c_int = 0;
pub const MC_CMD_DRV_ATTACH_IN_V2_WANT_TX_ONLY_SPREADING_LBN: c_int = 5;
pub const MC_CMD_DRV_ATTACH_IN_V2_WANT_TX_ONLY_SPREADING_WIDTH: c_int = 1;
// 1 to set new state, or 0 to just report the existing state
pub const MC_CMD_DRV_ATTACH_IN_V2_UPDATE_OFST: c_int = 4;
pub const MC_CMD_DRV_ATTACH_IN_V2_UPDATE_LEN: c_int = 4;
// preferred datapath firmware (for Huntington; ignored for Siena)
pub const MC_CMD_DRV_ATTACH_IN_V2_FIRMWARE_ID_OFST: c_int = 8;
pub const MC_CMD_DRV_ATTACH_IN_V2_FIRMWARE_ID_LEN: c_int = 4;
// enum: Prefer to use full featured firmware
// MC_CMD_FW_FULL_FEATURED 0x0
// enum: Prefer to use firmware with fewer features but lower latency
// MC_CMD_FW_LOW_LATENCY 0x1
// enum: Prefer to use firmware for SolarCapture packed stream mode
// MC_CMD_FW_PACKED_STREAM 0x2
// enum: Prefer to use firmware with fewer features and simpler TX event
// batching but higher TX packet rate
//
// MC_CMD_FW_HIGH_TX_RATE 0x3
// enum: Reserved value
// MC_CMD_FW_PACKED_STREAM_HASH_MODE_1 0x4
// enum: Prefer to use firmware with additional "rules engine" filtering
// support
//
// MC_CMD_FW_RULES_ENGINE 0x5
// enum: Prefer to use firmware with additional DPDK support
// MC_CMD_FW_DPDK 0x6
// enum: Prefer to use "l3xudp" custom datapath firmware (see SF-119495-PD and
// bug69716)
//
// MC_CMD_FW_L3XUDP 0x7
// enum: Requests that the MC keep whatever datapath firmware is currently
// running. It's used for test purposes, where we want to be able to shmboot
// special test firmware variants. This option is only recognised in eftest
// (i.e. non-production) builds.
//
// MC_CMD_FW_KEEP_CURRENT_EFTEST_ONLY 0xfffffffe
// enum: Only this option is allowed for non-admin functions
// MC_CMD_FW_DONT_CARE 0xffffffff
// Version of the driver to be reported by management protocols (e.g. NC-SI)
// handled by the NIC. This is a zero-terminated ASCII string.
//
pub const MC_CMD_DRV_ATTACH_IN_V2_DRIVER_VERSION_OFST: c_int = 12;
pub const MC_CMD_DRV_ATTACH_IN_V2_DRIVER_VERSION_LEN: c_int = 20;
// MC_CMD_DRV_ATTACH_OUT msgresponse
pub const MC_CMD_DRV_ATTACH_OUT_LEN: c_int = 4;
// previous or existing state, see the bitmask at NEW_STATE
pub const MC_CMD_DRV_ATTACH_OUT_OLD_STATE_OFST: c_int = 0;
pub const MC_CMD_DRV_ATTACH_OUT_OLD_STATE_LEN: c_int = 4;
// MC_CMD_DRV_ATTACH_EXT_OUT msgresponse
pub const MC_CMD_DRV_ATTACH_EXT_OUT_LEN: c_int = 8;
// previous or existing state, see the bitmask at NEW_STATE
pub const MC_CMD_DRV_ATTACH_EXT_OUT_OLD_STATE_OFST: c_int = 0;
pub const MC_CMD_DRV_ATTACH_EXT_OUT_OLD_STATE_LEN: c_int = 4;
// Flags associated with this function
pub const MC_CMD_DRV_ATTACH_EXT_OUT_FUNC_FLAGS_OFST: c_int = 4;
pub const MC_CMD_DRV_ATTACH_EXT_OUT_FUNC_FLAGS_LEN: c_int = 4;
// enum: Labels the lowest-numbered function visible to the OS
pub const MC_CMD_DRV_ATTACH_EXT_OUT_FLAG_PRIMARY: c_uint = 0x0;
// enum: The function can control the link state of the physical port it is
// bound to.
//
pub const MC_CMD_DRV_ATTACH_EXT_OUT_FLAG_LINKCTRL: c_uint = 0x1;
// enum: The function can perform privileged operations
pub const MC_CMD_DRV_ATTACH_EXT_OUT_FLAG_TRUSTED: c_uint = 0x2;
// enum: The function does not have an active port associated with it. The port
// refers to the Sorrento external FPGA port.
//
pub const MC_CMD_DRV_ATTACH_EXT_OUT_FLAG_NO_ACTIVE_PORT: c_uint = 0x3;
// enum: If set, indicates that VI spreading is currently enabled. Will always
// indicate the current state, regardless of the value in the WANT_VI_SPREADING
// input.
//
pub const MC_CMD_DRV_ATTACH_EXT_OUT_FLAG_VI_SPREADING_ENABLED: c_uint = 0x4;
// enum: Used during development only. Should no longer be used.
pub const MC_CMD_DRV_ATTACH_EXT_OUT_FLAG_RX_VI_SPREADING_INHIBITED: c_uint = 0x5;
// enum: If set, indicates that TX only spreading is enabled. Even-numbered
// TXQs will use one engine, and odd-numbered TXQs will use the other. This
// also has the effect that only even-numbered RXQs will receive traffic.
//
pub const MC_CMD_DRV_ATTACH_EXT_OUT_FLAG_TX_ONLY_VI_SPREADING_ENABLED: c_uint = 0x5;
//
// MC_CMD_SHMUART
// Route UART output to circular buffer in shared memory instead.
//
pub const MC_CMD_SHMUART: c_uint = 0x1f;
// MC_CMD_SHMUART_IN msgrequest
pub const MC_CMD_SHMUART_IN_LEN: c_int = 4;
// ???
pub const MC_CMD_SHMUART_IN_FLAG_OFST: c_int = 0;
pub const MC_CMD_SHMUART_IN_FLAG_LEN: c_int = 4;
// MC_CMD_SHMUART_OUT msgresponse
pub const MC_CMD_SHMUART_OUT_LEN: c_int = 0;
//
// MC_CMD_PORT_RESET
// Generic per-port reset. There is no equivalent for per-board reset. Locks
// required: None; Return code: 0, ETIME. NOTE: This command is deprecated -
// use MC_CMD_ENTITY_RESET instead.
//
pub const MC_CMD_PORT_RESET: c_uint = 0x20;

// MC_CMD_PORT_RESET_IN msgrequest
pub const MC_CMD_PORT_RESET_IN_LEN: c_int = 0;
// MC_CMD_PORT_RESET_OUT msgresponse
pub const MC_CMD_PORT_RESET_OUT_LEN: c_int = 0;
//
// MC_CMD_ENTITY_RESET
// Generic per-resource reset. There is no equivalent for per-board reset.
// Locks required: None; Return code: 0, ETIME. NOTE: This command is an
// extended version of the deprecated MC_CMD_PORT_RESET with added fields.
//
pub const MC_CMD_ENTITY_RESET: c_uint = 0x20;
// MC_CMD_0x20_PRIVILEGE_CTG SRIOV_CTG_GENERAL
// MC_CMD_ENTITY_RESET_IN msgrequest
pub const MC_CMD_ENTITY_RESET_IN_LEN: c_int = 4;
// Optional flags field. Omitting this will perform a "legacy" reset action
// (TBD).
//
pub const MC_CMD_ENTITY_RESET_IN_FLAG_OFST: c_int = 0;
pub const MC_CMD_ENTITY_RESET_IN_FLAG_LEN: c_int = 4;
pub const MC_CMD_ENTITY_RESET_IN_FUNCTION_RESOURCE_RESET_OFST: c_int = 0;
pub const MC_CMD_ENTITY_RESET_IN_FUNCTION_RESOURCE_RESET_LBN: c_int = 0;
pub const MC_CMD_ENTITY_RESET_IN_FUNCTION_RESOURCE_RESET_WIDTH: c_int = 1;
// MC_CMD_ENTITY_RESET_OUT msgresponse
pub const MC_CMD_ENTITY_RESET_OUT_LEN: c_int = 0;
//
// MC_CMD_PCIE_CREDITS
// Read instantaneous and minimum flow control thresholds.
//
pub const MC_CMD_PCIE_CREDITS: c_uint = 0x21;
// MC_CMD_PCIE_CREDITS_IN msgrequest
pub const MC_CMD_PCIE_CREDITS_IN_LEN: c_int = 8;
// poll period. 0 is disabled
pub const MC_CMD_PCIE_CREDITS_IN_POLL_PERIOD_OFST: c_int = 0;
pub const MC_CMD_PCIE_CREDITS_IN_POLL_PERIOD_LEN: c_int = 4;
// wipe statistics
pub const MC_CMD_PCIE_CREDITS_IN_WIPE_OFST: c_int = 4;
pub const MC_CMD_PCIE_CREDITS_IN_WIPE_LEN: c_int = 4;
// MC_CMD_PCIE_CREDITS_OUT msgresponse
pub const MC_CMD_PCIE_CREDITS_OUT_LEN: c_int = 16;
pub const MC_CMD_PCIE_CREDITS_OUT_CURRENT_P_HDR_OFST: c_int = 0;
pub const MC_CMD_PCIE_CREDITS_OUT_CURRENT_P_HDR_LEN: c_int = 2;
pub const MC_CMD_PCIE_CREDITS_OUT_CURRENT_P_DATA_OFST: c_int = 2;
pub const MC_CMD_PCIE_CREDITS_OUT_CURRENT_P_DATA_LEN: c_int = 2;
pub const MC_CMD_PCIE_CREDITS_OUT_CURRENT_NP_HDR_OFST: c_int = 4;
pub const MC_CMD_PCIE_CREDITS_OUT_CURRENT_NP_HDR_LEN: c_int = 2;
pub const MC_CMD_PCIE_CREDITS_OUT_CURRENT_NP_DATA_OFST: c_int = 6;
pub const MC_CMD_PCIE_CREDITS_OUT_CURRENT_NP_DATA_LEN: c_int = 2;
pub const MC_CMD_PCIE_CREDITS_OUT_MINIMUM_P_HDR_OFST: c_int = 8;
pub const MC_CMD_PCIE_CREDITS_OUT_MINIMUM_P_HDR_LEN: c_int = 2;
pub const MC_CMD_PCIE_CREDITS_OUT_MINIMUM_P_DATA_OFST: c_int = 10;
pub const MC_CMD_PCIE_CREDITS_OUT_MINIMUM_P_DATA_LEN: c_int = 2;
pub const MC_CMD_PCIE_CREDITS_OUT_MINIMUM_NP_HDR_OFST: c_int = 12;
pub const MC_CMD_PCIE_CREDITS_OUT_MINIMUM_NP_HDR_LEN: c_int = 2;
pub const MC_CMD_PCIE_CREDITS_OUT_MINIMUM_NP_DATA_OFST: c_int = 14;
pub const MC_CMD_PCIE_CREDITS_OUT_MINIMUM_NP_DATA_LEN: c_int = 2;
//
// MC_CMD_RXD_MONITOR
// Get histogram of RX queue fill level.
//
pub const MC_CMD_RXD_MONITOR: c_uint = 0x22;
// MC_CMD_RXD_MONITOR_IN msgrequest
pub const MC_CMD_RXD_MONITOR_IN_LEN: c_int = 12;
pub const MC_CMD_RXD_MONITOR_IN_QID_OFST: c_int = 0;
pub const MC_CMD_RXD_MONITOR_IN_QID_LEN: c_int = 4;
pub const MC_CMD_RXD_MONITOR_IN_POLL_PERIOD_OFST: c_int = 4;
pub const MC_CMD_RXD_MONITOR_IN_POLL_PERIOD_LEN: c_int = 4;
pub const MC_CMD_RXD_MONITOR_IN_WIPE_OFST: c_int = 8;
pub const MC_CMD_RXD_MONITOR_IN_WIPE_LEN: c_int = 4;
// MC_CMD_RXD_MONITOR_OUT msgresponse
pub const MC_CMD_RXD_MONITOR_OUT_LEN: c_int = 80;
pub const MC_CMD_RXD_MONITOR_OUT_QID_OFST: c_int = 0;
pub const MC_CMD_RXD_MONITOR_OUT_QID_LEN: c_int = 4;
pub const MC_CMD_RXD_MONITOR_OUT_RING_FILL_OFST: c_int = 4;
pub const MC_CMD_RXD_MONITOR_OUT_RING_FILL_LEN: c_int = 4;
pub const MC_CMD_RXD_MONITOR_OUT_CACHE_FILL_OFST: c_int = 8;
pub const MC_CMD_RXD_MONITOR_OUT_CACHE_FILL_LEN: c_int = 4;
pub const MC_CMD_RXD_MONITOR_OUT_RING_LT_1_OFST: c_int = 12;
pub const MC_CMD_RXD_MONITOR_OUT_RING_LT_1_LEN: c_int = 4;
pub const MC_CMD_RXD_MONITOR_OUT_RING_LT_2_OFST: c_int = 16;
pub const MC_CMD_RXD_MONITOR_OUT_RING_LT_2_LEN: c_int = 4;
pub const MC_CMD_RXD_MONITOR_OUT_RING_LT_4_OFST: c_int = 20;
pub const MC_CMD_RXD_MONITOR_OUT_RING_LT_4_LEN: c_int = 4;
pub const MC_CMD_RXD_MONITOR_OUT_RING_LT_8_OFST: c_int = 24;
pub const MC_CMD_RXD_MONITOR_OUT_RING_LT_8_LEN: c_int = 4;
pub const MC_CMD_RXD_MONITOR_OUT_RING_LT_16_OFST: c_int = 28;
pub const MC_CMD_RXD_MONITOR_OUT_RING_LT_16_LEN: c_int = 4;
pub const MC_CMD_RXD_MONITOR_OUT_RING_LT_32_OFST: c_int = 32;
pub const MC_CMD_RXD_MONITOR_OUT_RING_LT_32_LEN: c_int = 4;
pub const MC_CMD_RXD_MONITOR_OUT_RING_LT_64_OFST: c_int = 36;
pub const MC_CMD_RXD_MONITOR_OUT_RING_LT_64_LEN: c_int = 4;
pub const MC_CMD_RXD_MONITOR_OUT_RING_LT_128_OFST: c_int = 40;
pub const MC_CMD_RXD_MONITOR_OUT_RING_LT_128_LEN: c_int = 4;
pub const MC_CMD_RXD_MONITOR_OUT_RING_LT_256_OFST: c_int = 44;
pub const MC_CMD_RXD_MONITOR_OUT_RING_LT_256_LEN: c_int = 4;
pub const MC_CMD_RXD_MONITOR_OUT_RING_GE_256_OFST: c_int = 48;
pub const MC_CMD_RXD_MONITOR_OUT_RING_GE_256_LEN: c_int = 4;
pub const MC_CMD_RXD_MONITOR_OUT_CACHE_LT_1_OFST: c_int = 52;
pub const MC_CMD_RXD_MONITOR_OUT_CACHE_LT_1_LEN: c_int = 4;
pub const MC_CMD_RXD_MONITOR_OUT_CACHE_LT_2_OFST: c_int = 56;
pub const MC_CMD_RXD_MONITOR_OUT_CACHE_LT_2_LEN: c_int = 4;
pub const MC_CMD_RXD_MONITOR_OUT_CACHE_LT_4_OFST: c_int = 60;
pub const MC_CMD_RXD_MONITOR_OUT_CACHE_LT_4_LEN: c_int = 4;
pub const MC_CMD_RXD_MONITOR_OUT_CACHE_LT_8_OFST: c_int = 64;
pub const MC_CMD_RXD_MONITOR_OUT_CACHE_LT_8_LEN: c_int = 4;
pub const MC_CMD_RXD_MONITOR_OUT_CACHE_LT_16_OFST: c_int = 68;
pub const MC_CMD_RXD_MONITOR_OUT_CACHE_LT_16_LEN: c_int = 4;
pub const MC_CMD_RXD_MONITOR_OUT_CACHE_LT_32_OFST: c_int = 72;
pub const MC_CMD_RXD_MONITOR_OUT_CACHE_LT_32_LEN: c_int = 4;
pub const MC_CMD_RXD_MONITOR_OUT_CACHE_GE_32_OFST: c_int = 76;
pub const MC_CMD_RXD_MONITOR_OUT_CACHE_GE_32_LEN: c_int = 4;
//
// MC_CMD_PUTS
// Copy the given ASCII string out onto UART and/or out of the network port.
//
pub const MC_CMD_PUTS: c_uint = 0x23;

// MC_CMD_PUTS_IN msgrequest
pub const MC_CMD_PUTS_IN_LENMIN: c_int = 13;
pub const MC_CMD_PUTS_IN_LENMAX: c_int = 252;
pub const MC_CMD_PUTS_IN_LENMAX_MCDI2: c_int = 1020;

pub const MC_CMD_PUTS_IN_DEST_OFST: c_int = 0;
pub const MC_CMD_PUTS_IN_DEST_LEN: c_int = 4;
pub const MC_CMD_PUTS_IN_UART_OFST: c_int = 0;
pub const MC_CMD_PUTS_IN_UART_LBN: c_int = 0;
pub const MC_CMD_PUTS_IN_UART_WIDTH: c_int = 1;
pub const MC_CMD_PUTS_IN_PORT_OFST: c_int = 0;
pub const MC_CMD_PUTS_IN_PORT_LBN: c_int = 1;
pub const MC_CMD_PUTS_IN_PORT_WIDTH: c_int = 1;
pub const MC_CMD_PUTS_IN_DHOST_OFST: c_int = 4;
pub const MC_CMD_PUTS_IN_DHOST_LEN: c_int = 6;
pub const MC_CMD_PUTS_IN_STRING_OFST: c_int = 12;
pub const MC_CMD_PUTS_IN_STRING_LEN: c_int = 1;
pub const MC_CMD_PUTS_IN_STRING_MINNUM: c_int = 1;
pub const MC_CMD_PUTS_IN_STRING_MAXNUM: c_int = 240;
pub const MC_CMD_PUTS_IN_STRING_MAXNUM_MCDI2: c_int = 1008;
// MC_CMD_PUTS_OUT msgresponse
pub const MC_CMD_PUTS_OUT_LEN: c_int = 0;
//
// MC_CMD_GET_PHY_CFG
// Report PHY configuration. This guarantees to succeed even if the PHY is in a
// 'zombie' state. Locks required: None
//
pub const MC_CMD_GET_PHY_CFG: c_uint = 0x24;

// MC_CMD_GET_PHY_CFG_IN msgrequest
pub const MC_CMD_GET_PHY_CFG_IN_LEN: c_int = 0;
// MC_CMD_GET_PHY_CFG_OUT msgresponse
pub const MC_CMD_GET_PHY_CFG_OUT_LEN: c_int = 72;
// flags
pub const MC_CMD_GET_PHY_CFG_OUT_FLAGS_OFST: c_int = 0;
pub const MC_CMD_GET_PHY_CFG_OUT_FLAGS_LEN: c_int = 4;
pub const MC_CMD_GET_PHY_CFG_OUT_PRESENT_OFST: c_int = 0;
pub const MC_CMD_GET_PHY_CFG_OUT_PRESENT_LBN: c_int = 0;
pub const MC_CMD_GET_PHY_CFG_OUT_PRESENT_WIDTH: c_int = 1;
pub const MC_CMD_GET_PHY_CFG_OUT_BIST_CABLE_SHORT_OFST: c_int = 0;
pub const MC_CMD_GET_PHY_CFG_OUT_BIST_CABLE_SHORT_LBN: c_int = 1;
pub const MC_CMD_GET_PHY_CFG_OUT_BIST_CABLE_SHORT_WIDTH: c_int = 1;
pub const MC_CMD_GET_PHY_CFG_OUT_BIST_CABLE_LONG_OFST: c_int = 0;
pub const MC_CMD_GET_PHY_CFG_OUT_BIST_CABLE_LONG_LBN: c_int = 2;
pub const MC_CMD_GET_PHY_CFG_OUT_BIST_CABLE_LONG_WIDTH: c_int = 1;
pub const MC_CMD_GET_PHY_CFG_OUT_LOWPOWER_OFST: c_int = 0;
pub const MC_CMD_GET_PHY_CFG_OUT_LOWPOWER_LBN: c_int = 3;
pub const MC_CMD_GET_PHY_CFG_OUT_LOWPOWER_WIDTH: c_int = 1;
pub const MC_CMD_GET_PHY_CFG_OUT_POWEROFF_OFST: c_int = 0;
pub const MC_CMD_GET_PHY_CFG_OUT_POWEROFF_LBN: c_int = 4;
pub const MC_CMD_GET_PHY_CFG_OUT_POWEROFF_WIDTH: c_int = 1;
pub const MC_CMD_GET_PHY_CFG_OUT_TXDIS_OFST: c_int = 0;
pub const MC_CMD_GET_PHY_CFG_OUT_TXDIS_LBN: c_int = 5;
pub const MC_CMD_GET_PHY_CFG_OUT_TXDIS_WIDTH: c_int = 1;
pub const MC_CMD_GET_PHY_CFG_OUT_BIST_OFST: c_int = 0;
pub const MC_CMD_GET_PHY_CFG_OUT_BIST_LBN: c_int = 6;
pub const MC_CMD_GET_PHY_CFG_OUT_BIST_WIDTH: c_int = 1;
// ??
pub const MC_CMD_GET_PHY_CFG_OUT_TYPE_OFST: c_int = 4;
pub const MC_CMD_GET_PHY_CFG_OUT_TYPE_LEN: c_int = 4;
// Bitmask of supported capabilities
pub const MC_CMD_GET_PHY_CFG_OUT_SUPPORTED_CAP_OFST: c_int = 8;
pub const MC_CMD_GET_PHY_CFG_OUT_SUPPORTED_CAP_LEN: c_int = 4;
pub const MC_CMD_PHY_CAP_10HDX_OFST: c_int = 8;
pub const MC_CMD_PHY_CAP_10HDX_LBN: c_int = 1;
pub const MC_CMD_PHY_CAP_10HDX_WIDTH: c_int = 1;
pub const MC_CMD_PHY_CAP_10FDX_OFST: c_int = 8;
pub const MC_CMD_PHY_CAP_10FDX_LBN: c_int = 2;
pub const MC_CMD_PHY_CAP_10FDX_WIDTH: c_int = 1;
pub const MC_CMD_PHY_CAP_100HDX_OFST: c_int = 8;
pub const MC_CMD_PHY_CAP_100HDX_LBN: c_int = 3;
pub const MC_CMD_PHY_CAP_100HDX_WIDTH: c_int = 1;
pub const MC_CMD_PHY_CAP_100FDX_OFST: c_int = 8;
pub const MC_CMD_PHY_CAP_100FDX_LBN: c_int = 4;
pub const MC_CMD_PHY_CAP_100FDX_WIDTH: c_int = 1;
pub const MC_CMD_PHY_CAP_1000HDX_OFST: c_int = 8;
pub const MC_CMD_PHY_CAP_1000HDX_LBN: c_int = 5;
pub const MC_CMD_PHY_CAP_1000HDX_WIDTH: c_int = 1;
pub const MC_CMD_PHY_CAP_1000FDX_OFST: c_int = 8;
pub const MC_CMD_PHY_CAP_1000FDX_LBN: c_int = 6;
pub const MC_CMD_PHY_CAP_1000FDX_WIDTH: c_int = 1;
pub const MC_CMD_PHY_CAP_10000FDX_OFST: c_int = 8;
pub const MC_CMD_PHY_CAP_10000FDX_LBN: c_int = 7;
pub const MC_CMD_PHY_CAP_10000FDX_WIDTH: c_int = 1;
pub const MC_CMD_PHY_CAP_PAUSE_OFST: c_int = 8;
pub const MC_CMD_PHY_CAP_PAUSE_LBN: c_int = 8;
pub const MC_CMD_PHY_CAP_PAUSE_WIDTH: c_int = 1;
pub const MC_CMD_PHY_CAP_ASYM_OFST: c_int = 8;
pub const MC_CMD_PHY_CAP_ASYM_LBN: c_int = 9;
pub const MC_CMD_PHY_CAP_ASYM_WIDTH: c_int = 1;
pub const MC_CMD_PHY_CAP_AN_OFST: c_int = 8;
pub const MC_CMD_PHY_CAP_AN_LBN: c_int = 10;
pub const MC_CMD_PHY_CAP_AN_WIDTH: c_int = 1;
pub const MC_CMD_PHY_CAP_40000FDX_OFST: c_int = 8;
pub const MC_CMD_PHY_CAP_40000FDX_LBN: c_int = 11;
pub const MC_CMD_PHY_CAP_40000FDX_WIDTH: c_int = 1;
pub const MC_CMD_PHY_CAP_DDM_OFST: c_int = 8;
pub const MC_CMD_PHY_CAP_DDM_LBN: c_int = 12;
pub const MC_CMD_PHY_CAP_DDM_WIDTH: c_int = 1;
pub const MC_CMD_PHY_CAP_100000FDX_OFST: c_int = 8;
pub const MC_CMD_PHY_CAP_100000FDX_LBN: c_int = 13;
pub const MC_CMD_PHY_CAP_100000FDX_WIDTH: c_int = 1;
pub const MC_CMD_PHY_CAP_25000FDX_OFST: c_int = 8;
pub const MC_CMD_PHY_CAP_25000FDX_LBN: c_int = 14;
pub const MC_CMD_PHY_CAP_25000FDX_WIDTH: c_int = 1;
pub const MC_CMD_PHY_CAP_50000FDX_OFST: c_int = 8;
pub const MC_CMD_PHY_CAP_50000FDX_LBN: c_int = 15;
pub const MC_CMD_PHY_CAP_50000FDX_WIDTH: c_int = 1;
pub const MC_CMD_PHY_CAP_BASER_FEC_OFST: c_int = 8;
pub const MC_CMD_PHY_CAP_BASER_FEC_LBN: c_int = 16;
pub const MC_CMD_PHY_CAP_BASER_FEC_WIDTH: c_int = 1;
pub const MC_CMD_PHY_CAP_BASER_FEC_REQUESTED_OFST: c_int = 8;
pub const MC_CMD_PHY_CAP_BASER_FEC_REQUESTED_LBN: c_int = 17;
pub const MC_CMD_PHY_CAP_BASER_FEC_REQUESTED_WIDTH: c_int = 1;
pub const MC_CMD_PHY_CAP_RS_FEC_OFST: c_int = 8;
pub const MC_CMD_PHY_CAP_RS_FEC_LBN: c_int = 18;
pub const MC_CMD_PHY_CAP_RS_FEC_WIDTH: c_int = 1;
pub const MC_CMD_PHY_CAP_RS_FEC_REQUESTED_OFST: c_int = 8;
pub const MC_CMD_PHY_CAP_RS_FEC_REQUESTED_LBN: c_int = 19;
pub const MC_CMD_PHY_CAP_RS_FEC_REQUESTED_WIDTH: c_int = 1;
pub const MC_CMD_PHY_CAP_25G_BASER_FEC_OFST: c_int = 8;
pub const MC_CMD_PHY_CAP_25G_BASER_FEC_LBN: c_int = 20;
pub const MC_CMD_PHY_CAP_25G_BASER_FEC_WIDTH: c_int = 1;
pub const MC_CMD_PHY_CAP_25G_BASER_FEC_REQUESTED_OFST: c_int = 8;
pub const MC_CMD_PHY_CAP_25G_BASER_FEC_REQUESTED_LBN: c_int = 21;
pub const MC_CMD_PHY_CAP_25G_BASER_FEC_REQUESTED_WIDTH: c_int = 1;
// ??
pub const MC_CMD_GET_PHY_CFG_OUT_CHANNEL_OFST: c_int = 12;
pub const MC_CMD_GET_PHY_CFG_OUT_CHANNEL_LEN: c_int = 4;
// ??
pub const MC_CMD_GET_PHY_CFG_OUT_PRT_OFST: c_int = 16;
pub const MC_CMD_GET_PHY_CFG_OUT_PRT_LEN: c_int = 4;
// ??
pub const MC_CMD_GET_PHY_CFG_OUT_STATS_MASK_OFST: c_int = 20;
pub const MC_CMD_GET_PHY_CFG_OUT_STATS_MASK_LEN: c_int = 4;
// ??
pub const MC_CMD_GET_PHY_CFG_OUT_NAME_OFST: c_int = 24;
pub const MC_CMD_GET_PHY_CFG_OUT_NAME_LEN: c_int = 20;
// ??
pub const MC_CMD_GET_PHY_CFG_OUT_MEDIA_TYPE_OFST: c_int = 44;
pub const MC_CMD_GET_PHY_CFG_OUT_MEDIA_TYPE_LEN: c_int = 4;
// enum: Xaui.
pub const MC_CMD_MEDIA_XAUI: c_uint = 0x1;
// enum: CX4.
pub const MC_CMD_MEDIA_CX4: c_uint = 0x2;
// enum: KX4.
pub const MC_CMD_MEDIA_KX4: c_uint = 0x3;
// enum: XFP Far.
pub const MC_CMD_MEDIA_XFP: c_uint = 0x4;
// enum: SFP+.
pub const MC_CMD_MEDIA_SFP_PLUS: c_uint = 0x5;
// enum: 10GBaseT.
pub const MC_CMD_MEDIA_BASE_T: c_uint = 0x6;
// enum: QSFP+.
pub const MC_CMD_MEDIA_QSFP_PLUS: c_uint = 0x7;
pub const MC_CMD_GET_PHY_CFG_OUT_MMD_MASK_OFST: c_int = 48;
pub const MC_CMD_GET_PHY_CFG_OUT_MMD_MASK_LEN: c_int = 4;
// enum: Native clause 22
pub const MC_CMD_MMD_CLAUSE22: c_uint = 0x0;
pub const MC_CMD_MMD_CLAUSE45_PMAPMD: c_uint = 0x1 /* enum */;
pub const MC_CMD_MMD_CLAUSE45_WIS: c_uint = 0x2 /* enum */;
pub const MC_CMD_MMD_CLAUSE45_PCS: c_uint = 0x3 /* enum */;
pub const MC_CMD_MMD_CLAUSE45_PHYXS: c_uint = 0x4 /* enum */;
pub const MC_CMD_MMD_CLAUSE45_DTEXS: c_uint = 0x5 /* enum */;
pub const MC_CMD_MMD_CLAUSE45_TC: c_uint = 0x6 /* enum */;
pub const MC_CMD_MMD_CLAUSE45_AN: c_uint = 0x7 /* enum */;
// enum: Clause22 proxied over clause45 by PHY.
pub const MC_CMD_MMD_CLAUSE45_C22EXT: c_uint = 0x1d;
pub const MC_CMD_MMD_CLAUSE45_VEND1: c_uint = 0x1e /* enum */;
pub const MC_CMD_MMD_CLAUSE45_VEND2: c_uint = 0x1f /* enum */;
pub const MC_CMD_GET_PHY_CFG_OUT_REVISION_OFST: c_int = 52;
pub const MC_CMD_GET_PHY_CFG_OUT_REVISION_LEN: c_int = 20;
//
// MC_CMD_START_BIST
// Start a BIST test on the PHY. Locks required: PHY_LOCK if doing a PHY BIST
// Return code: 0, EINVAL, EACCES (if PHY_LOCK is not held)
//
pub const MC_CMD_START_BIST: c_uint = 0x25;

// MC_CMD_START_BIST_IN msgrequest
pub const MC_CMD_START_BIST_IN_LEN: c_int = 4;
// Type of test.
pub const MC_CMD_START_BIST_IN_TYPE_OFST: c_int = 0;
pub const MC_CMD_START_BIST_IN_TYPE_LEN: c_int = 4;
// enum: Run the PHY's short cable BIST.
pub const MC_CMD_PHY_BIST_CABLE_SHORT: c_uint = 0x1;
// enum: Run the PHY's long cable BIST.
pub const MC_CMD_PHY_BIST_CABLE_LONG: c_uint = 0x2;
// enum: Run BIST on the currently selected BPX Serdes (XAUI or XFI) .
pub const MC_CMD_BPX_SERDES_BIST: c_uint = 0x3;
// enum: Run the MC loopback tests.
pub const MC_CMD_MC_LOOPBACK_BIST: c_uint = 0x4;
// enum: Run the PHY's standard BIST.
pub const MC_CMD_PHY_BIST: c_uint = 0x5;
// enum: Run MC RAM test.
pub const MC_CMD_MC_MEM_BIST: c_uint = 0x6;
// enum: Run Port RAM test.
pub const MC_CMD_PORT_MEM_BIST: c_uint = 0x7;
// enum: Run register test.
pub const MC_CMD_REG_BIST: c_uint = 0x8;
// MC_CMD_START_BIST_OUT msgresponse
pub const MC_CMD_START_BIST_OUT_LEN: c_int = 0;
//
// MC_CMD_POLL_BIST
// Poll for BIST completion. Returns a single status code, and optionally some
// PHY specific bist output. The driver should only consume the BIST output
// after validating OUTLEN and MC_CMD_GET_PHY_CFG.TYPE. If a driver can't
// successfully parse the BIST output, it should still respect the pass/Fail in
// OUT.RESULT. Locks required: PHY_LOCK if doing a PHY BIST. Return code: 0,
// EACCES (if PHY_LOCK is not held).
//
pub const MC_CMD_POLL_BIST: c_uint = 0x26;

// MC_CMD_POLL_BIST_IN msgrequest
pub const MC_CMD_POLL_BIST_IN_LEN: c_int = 0;
// MC_CMD_POLL_BIST_OUT msgresponse
pub const MC_CMD_POLL_BIST_OUT_LEN: c_int = 8;
// result
pub const MC_CMD_POLL_BIST_OUT_RESULT_OFST: c_int = 0;
pub const MC_CMD_POLL_BIST_OUT_RESULT_LEN: c_int = 4;
// enum: Running.
pub const MC_CMD_POLL_BIST_RUNNING: c_uint = 0x1;
// enum: Passed.
pub const MC_CMD_POLL_BIST_PASSED: c_uint = 0x2;
// enum: Failed.
pub const MC_CMD_POLL_BIST_FAILED: c_uint = 0x3;
// enum: Timed-out.
pub const MC_CMD_POLL_BIST_TIMEOUT: c_uint = 0x4;
pub const MC_CMD_POLL_BIST_OUT_PRIVATE_OFST: c_int = 4;
pub const MC_CMD_POLL_BIST_OUT_PRIVATE_LEN: c_int = 4;
// MC_CMD_POLL_BIST_OUT_SFT9001 msgresponse
pub const MC_CMD_POLL_BIST_OUT_SFT9001_LEN: c_int = 36;
// result
// MC_CMD_POLL_BIST_OUT_RESULT_OFST 0
// MC_CMD_POLL_BIST_OUT_RESULT_LEN 4
// Enum values, see field(s):
// MC_CMD_POLL_BIST_OUT/MC_CMD_POLL_BIST_OUT_RESULT
pub const MC_CMD_POLL_BIST_OUT_SFT9001_CABLE_LENGTH_A_OFST: c_int = 4;
pub const MC_CMD_POLL_BIST_OUT_SFT9001_CABLE_LENGTH_A_LEN: c_int = 4;
pub const MC_CMD_POLL_BIST_OUT_SFT9001_CABLE_LENGTH_B_OFST: c_int = 8;
pub const MC_CMD_POLL_BIST_OUT_SFT9001_CABLE_LENGTH_B_LEN: c_int = 4;
pub const MC_CMD_POLL_BIST_OUT_SFT9001_CABLE_LENGTH_C_OFST: c_int = 12;
pub const MC_CMD_POLL_BIST_OUT_SFT9001_CABLE_LENGTH_C_LEN: c_int = 4;
pub const MC_CMD_POLL_BIST_OUT_SFT9001_CABLE_LENGTH_D_OFST: c_int = 16;
pub const MC_CMD_POLL_BIST_OUT_SFT9001_CABLE_LENGTH_D_LEN: c_int = 4;
// Status of each channel A
pub const MC_CMD_POLL_BIST_OUT_SFT9001_CABLE_STATUS_A_OFST: c_int = 20;
pub const MC_CMD_POLL_BIST_OUT_SFT9001_CABLE_STATUS_A_LEN: c_int = 4;
// enum: Ok.
pub const MC_CMD_POLL_BIST_SFT9001_PAIR_OK: c_uint = 0x1;
// enum: Open.
pub const MC_CMD_POLL_BIST_SFT9001_PAIR_OPEN: c_uint = 0x2;
// enum: Intra-pair short.
pub const MC_CMD_POLL_BIST_SFT9001_INTRA_PAIR_SHORT: c_uint = 0x3;
// enum: Inter-pair short.
pub const MC_CMD_POLL_BIST_SFT9001_INTER_PAIR_SHORT: c_uint = 0x4;
// enum: Busy.
pub const MC_CMD_POLL_BIST_SFT9001_PAIR_BUSY: c_uint = 0x9;
// Status of each channel B
pub const MC_CMD_POLL_BIST_OUT_SFT9001_CABLE_STATUS_B_OFST: c_int = 24;
pub const MC_CMD_POLL_BIST_OUT_SFT9001_CABLE_STATUS_B_LEN: c_int = 4;
// Enum values, see field(s):
// CABLE_STATUS_A
// Status of each channel C
pub const MC_CMD_POLL_BIST_OUT_SFT9001_CABLE_STATUS_C_OFST: c_int = 28;
pub const MC_CMD_POLL_BIST_OUT_SFT9001_CABLE_STATUS_C_LEN: c_int = 4;
// Enum values, see field(s):
// CABLE_STATUS_A
// Status of each channel D
pub const MC_CMD_POLL_BIST_OUT_SFT9001_CABLE_STATUS_D_OFST: c_int = 32;
pub const MC_CMD_POLL_BIST_OUT_SFT9001_CABLE_STATUS_D_LEN: c_int = 4;
// Enum values, see field(s):
// CABLE_STATUS_A
// MC_CMD_POLL_BIST_OUT_MRSFP msgresponse
pub const MC_CMD_POLL_BIST_OUT_MRSFP_LEN: c_int = 8;
// result
// MC_CMD_POLL_BIST_OUT_RESULT_OFST 0
// MC_CMD_POLL_BIST_OUT_RESULT_LEN 4
// Enum values, see field(s):
// MC_CMD_POLL_BIST_OUT/MC_CMD_POLL_BIST_OUT_RESULT
pub const MC_CMD_POLL_BIST_OUT_MRSFP_TEST_OFST: c_int = 4;
pub const MC_CMD_POLL_BIST_OUT_MRSFP_TEST_LEN: c_int = 4;
// enum: Complete.
pub const MC_CMD_POLL_BIST_MRSFP_TEST_COMPLETE: c_uint = 0x0;
// enum: Bus switch off I2C write.
pub const MC_CMD_POLL_BIST_MRSFP_TEST_BUS_SWITCH_OFF_I2C_WRITE: c_uint = 0x1;
// enum: Bus switch off I2C no access IO exp.
pub const MC_CMD_POLL_BIST_MRSFP_TEST_BUS_SWITCH_OFF_I2C_NO_ACCESS_IO_EXP: c_uint = 0x2;
// enum: Bus switch off I2C no access module.
pub const MC_CMD_POLL_BIST_MRSFP_TEST_BUS_SWITCH_OFF_I2C_NO_ACCESS_MODULE: c_uint = 0x3;
// enum: IO exp I2C configure.
pub const MC_CMD_POLL_BIST_MRSFP_TEST_IO_EXP_I2C_CONFIGURE: c_uint = 0x4;
// enum: Bus switch I2C no cross talk.
pub const MC_CMD_POLL_BIST_MRSFP_TEST_BUS_SWITCH_I2C_NO_CROSSTALK: c_uint = 0x5;
// enum: Module presence.
pub const MC_CMD_POLL_BIST_MRSFP_TEST_MODULE_PRESENCE: c_uint = 0x6;
// enum: Module ID I2C access.
pub const MC_CMD_POLL_BIST_MRSFP_TEST_MODULE_ID_I2C_ACCESS: c_uint = 0x7;
// enum: Module ID sane value.
pub const MC_CMD_POLL_BIST_MRSFP_TEST_MODULE_ID_SANE_VALUE: c_uint = 0x8;
// MC_CMD_POLL_BIST_OUT_MEM msgresponse
pub const MC_CMD_POLL_BIST_OUT_MEM_LEN: c_int = 36;
// result
// MC_CMD_POLL_BIST_OUT_RESULT_OFST 0
// MC_CMD_POLL_BIST_OUT_RESULT_LEN 4
// Enum values, see field(s):
// MC_CMD_POLL_BIST_OUT/MC_CMD_POLL_BIST_OUT_RESULT
pub const MC_CMD_POLL_BIST_OUT_MEM_TEST_OFST: c_int = 4;
pub const MC_CMD_POLL_BIST_OUT_MEM_TEST_LEN: c_int = 4;
// enum: Test has completed.
pub const MC_CMD_POLL_BIST_MEM_COMPLETE: c_uint = 0x0;
// enum: RAM test - walk ones.
pub const MC_CMD_POLL_BIST_MEM_MEM_WALK_ONES: c_uint = 0x1;
// enum: RAM test - walk zeros.
pub const MC_CMD_POLL_BIST_MEM_MEM_WALK_ZEROS: c_uint = 0x2;
// enum: RAM test - walking inversions zeros/ones.
pub const MC_CMD_POLL_BIST_MEM_MEM_INV_ZERO_ONE: c_uint = 0x3;
// enum: RAM test - walking inversions checkerboard.
pub const MC_CMD_POLL_BIST_MEM_MEM_INV_CHKBOARD: c_uint = 0x4;
// enum: Register test - set / clear individual bits.
pub const MC_CMD_POLL_BIST_MEM_REG: c_uint = 0x5;
// enum: ECC error detected.
pub const MC_CMD_POLL_BIST_MEM_ECC: c_uint = 0x6;
// Failure address, only valid if result is POLL_BIST_FAILED
pub const MC_CMD_POLL_BIST_OUT_MEM_ADDR_OFST: c_int = 8;
pub const MC_CMD_POLL_BIST_OUT_MEM_ADDR_LEN: c_int = 4;
// Bus or address space to which the failure address corresponds
pub const MC_CMD_POLL_BIST_OUT_MEM_BUS_OFST: c_int = 12;
pub const MC_CMD_POLL_BIST_OUT_MEM_BUS_LEN: c_int = 4;
// enum: MC MIPS bus.
pub const MC_CMD_POLL_BIST_MEM_BUS_MC: c_uint = 0x0;
// enum: CSR IREG bus.
pub const MC_CMD_POLL_BIST_MEM_BUS_CSR: c_uint = 0x1;
// enum: RX0 DPCPU bus.
pub const MC_CMD_POLL_BIST_MEM_BUS_DPCPU_RX: c_uint = 0x2;
// enum: TX0 DPCPU bus.
pub const MC_CMD_POLL_BIST_MEM_BUS_DPCPU_TX0: c_uint = 0x3;
// enum: TX1 DPCPU bus.
pub const MC_CMD_POLL_BIST_MEM_BUS_DPCPU_TX1: c_uint = 0x4;
// enum: RX0 DICPU bus.
pub const MC_CMD_POLL_BIST_MEM_BUS_DICPU_RX: c_uint = 0x5;
// enum: TX DICPU bus.
pub const MC_CMD_POLL_BIST_MEM_BUS_DICPU_TX: c_uint = 0x6;
// enum: RX1 DPCPU bus.
pub const MC_CMD_POLL_BIST_MEM_BUS_DPCPU_RX1: c_uint = 0x7;
// enum: RX1 DICPU bus.
pub const MC_CMD_POLL_BIST_MEM_BUS_DICPU_RX1: c_uint = 0x8;
// Pattern written to RAM / register
pub const MC_CMD_POLL_BIST_OUT_MEM_EXPECT_OFST: c_int = 16;
pub const MC_CMD_POLL_BIST_OUT_MEM_EXPECT_LEN: c_int = 4;
// Actual value read from RAM / register
pub const MC_CMD_POLL_BIST_OUT_MEM_ACTUAL_OFST: c_int = 20;
pub const MC_CMD_POLL_BIST_OUT_MEM_ACTUAL_LEN: c_int = 4;
// ECC error mask
pub const MC_CMD_POLL_BIST_OUT_MEM_ECC_OFST: c_int = 24;
pub const MC_CMD_POLL_BIST_OUT_MEM_ECC_LEN: c_int = 4;
// ECC parity error mask
pub const MC_CMD_POLL_BIST_OUT_MEM_ECC_PARITY_OFST: c_int = 28;
pub const MC_CMD_POLL_BIST_OUT_MEM_ECC_PARITY_LEN: c_int = 4;
// ECC fatal error mask
pub const MC_CMD_POLL_BIST_OUT_MEM_ECC_FATAL_OFST: c_int = 32;
pub const MC_CMD_POLL_BIST_OUT_MEM_ECC_FATAL_LEN: c_int = 4;
//
// MC_CMD_FLUSH_RX_QUEUES
// Flush receive queue(s). If SRIOV is enabled (via MC_CMD_SRIOV), then RXQ
// flushes should be initiated via this MCDI operation, rather than via
// directly writing FLUSH_CMD.
//
// The flush is completed (either done/fail) asynchronously (after this command
// returns). The driver must still wait for flush done/failure events as usual.
//
pub const MC_CMD_FLUSH_RX_QUEUES: c_uint = 0x27;
// MC_CMD_FLUSH_RX_QUEUES_IN msgrequest
pub const MC_CMD_FLUSH_RX_QUEUES_IN_LENMIN: c_int = 4;
pub const MC_CMD_FLUSH_RX_QUEUES_IN_LENMAX: c_int = 252;
pub const MC_CMD_FLUSH_RX_QUEUES_IN_LENMAX_MCDI2: c_int = 1020;

pub const MC_CMD_FLUSH_RX_QUEUES_IN_QID_OFST_OFST: c_int = 0;
pub const MC_CMD_FLUSH_RX_QUEUES_IN_QID_OFST_LEN: c_int = 4;
pub const MC_CMD_FLUSH_RX_QUEUES_IN_QID_OFST_MINNUM: c_int = 1;
pub const MC_CMD_FLUSH_RX_QUEUES_IN_QID_OFST_MAXNUM: c_int = 63;
pub const MC_CMD_FLUSH_RX_QUEUES_IN_QID_OFST_MAXNUM_MCDI2: c_int = 255;
// MC_CMD_FLUSH_RX_QUEUES_OUT msgresponse
pub const MC_CMD_FLUSH_RX_QUEUES_OUT_LEN: c_int = 0;
//
// MC_CMD_GET_LOOPBACK_MODES
// Returns a bitmask of loopback modes available at each speed.
//
pub const MC_CMD_GET_LOOPBACK_MODES: c_uint = 0x28;

// MC_CMD_GET_LOOPBACK_MODES_IN msgrequest
pub const MC_CMD_GET_LOOPBACK_MODES_IN_LEN: c_int = 0;
// MC_CMD_GET_LOOPBACK_MODES_OUT msgresponse
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_LEN: c_int = 40;
// Supported loopbacks.
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_100M_OFST: c_int = 0;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_100M_LEN: c_int = 8;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_100M_LO_OFST: c_int = 0;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_100M_HI_OFST: c_int = 4;
// enum: None.
pub const MC_CMD_LOOPBACK_NONE: c_uint = 0x0;
// enum: Data.
pub const MC_CMD_LOOPBACK_DATA: c_uint = 0x1;
// enum: GMAC.
pub const MC_CMD_LOOPBACK_GMAC: c_uint = 0x2;
// enum: XGMII.
pub const MC_CMD_LOOPBACK_XGMII: c_uint = 0x3;
// enum: XGXS.
pub const MC_CMD_LOOPBACK_XGXS: c_uint = 0x4;
// enum: XAUI.
pub const MC_CMD_LOOPBACK_XAUI: c_uint = 0x5;
// enum: GMII.
pub const MC_CMD_LOOPBACK_GMII: c_uint = 0x6;
// enum: SGMII.
pub const MC_CMD_LOOPBACK_SGMII: c_uint = 0x7;
// enum: XGBR.
pub const MC_CMD_LOOPBACK_XGBR: c_uint = 0x8;
// enum: XFI.
pub const MC_CMD_LOOPBACK_XFI: c_uint = 0x9;
// enum: XAUI Far.
pub const MC_CMD_LOOPBACK_XAUI_FAR: c_uint = 0xa;
// enum: GMII Far.
pub const MC_CMD_LOOPBACK_GMII_FAR: c_uint = 0xb;
// enum: SGMII Far.
pub const MC_CMD_LOOPBACK_SGMII_FAR: c_uint = 0xc;
// enum: XFI Far.
pub const MC_CMD_LOOPBACK_XFI_FAR: c_uint = 0xd;
// enum: GPhy.
pub const MC_CMD_LOOPBACK_GPHY: c_uint = 0xe;
// enum: PhyXS.
pub const MC_CMD_LOOPBACK_PHYXS: c_uint = 0xf;
// enum: PCS.
pub const MC_CMD_LOOPBACK_PCS: c_uint = 0x10;
// enum: PMA-PMD.
pub const MC_CMD_LOOPBACK_PMAPMD: c_uint = 0x11;
// enum: Cross-Port.
pub const MC_CMD_LOOPBACK_XPORT: c_uint = 0x12;
// enum: XGMII-Wireside.
pub const MC_CMD_LOOPBACK_XGMII_WS: c_uint = 0x13;
// enum: XAUI Wireside.
pub const MC_CMD_LOOPBACK_XAUI_WS: c_uint = 0x14;
// enum: XAUI Wireside Far.
pub const MC_CMD_LOOPBACK_XAUI_WS_FAR: c_uint = 0x15;
// enum: XAUI Wireside near.
pub const MC_CMD_LOOPBACK_XAUI_WS_NEAR: c_uint = 0x16;
// enum: GMII Wireside.
pub const MC_CMD_LOOPBACK_GMII_WS: c_uint = 0x17;
// enum: XFI Wireside.
pub const MC_CMD_LOOPBACK_XFI_WS: c_uint = 0x18;
// enum: XFI Wireside Far.
pub const MC_CMD_LOOPBACK_XFI_WS_FAR: c_uint = 0x19;
// enum: PhyXS Wireside.
pub const MC_CMD_LOOPBACK_PHYXS_WS: c_uint = 0x1a;
// enum: PMA lanes MAC-Serdes.
pub const MC_CMD_LOOPBACK_PMA_INT: c_uint = 0x1b;
// enum: KR Serdes Parallel (Encoder).
pub const MC_CMD_LOOPBACK_SD_NEAR: c_uint = 0x1c;
// enum: KR Serdes Serial.
pub const MC_CMD_LOOPBACK_SD_FAR: c_uint = 0x1d;
// enum: PMA lanes MAC-Serdes Wireside.
pub const MC_CMD_LOOPBACK_PMA_INT_WS: c_uint = 0x1e;
// enum: KR Serdes Parallel Wireside (Full PCS).
pub const MC_CMD_LOOPBACK_SD_FEP2_WS: c_uint = 0x1f;
// enum: KR Serdes Parallel Wireside (Sym Aligner to TX).
pub const MC_CMD_LOOPBACK_SD_FEP1_5_WS: c_uint = 0x20;
// enum: KR Serdes Parallel Wireside (Deserializer to Serializer).
pub const MC_CMD_LOOPBACK_SD_FEP_WS: c_uint = 0x21;
// enum: KR Serdes Serial Wireside.
pub const MC_CMD_LOOPBACK_SD_FES_WS: c_uint = 0x22;
// enum: Near side of AOE Siena side port
pub const MC_CMD_LOOPBACK_AOE_INT_NEAR: c_uint = 0x23;
// enum: Medford Wireside datapath loopback
pub const MC_CMD_LOOPBACK_DATA_WS: c_uint = 0x24;
// enum: Force link up without setting up any physical loopback (snapper use
// only)
//
pub const MC_CMD_LOOPBACK_FORCE_EXT_LINK: c_uint = 0x25;
// Supported loopbacks.
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_1G_OFST: c_int = 8;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_1G_LEN: c_int = 8;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_1G_LO_OFST: c_int = 8;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_1G_HI_OFST: c_int = 12;
// Enum values, see field(s):
// 100M
// Supported loopbacks.
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_10G_OFST: c_int = 16;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_10G_LEN: c_int = 8;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_10G_LO_OFST: c_int = 16;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_10G_HI_OFST: c_int = 20;
// Enum values, see field(s):
// 100M
// Supported loopbacks.
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_SUGGESTED_OFST: c_int = 24;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_SUGGESTED_LEN: c_int = 8;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_SUGGESTED_LO_OFST: c_int = 24;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_SUGGESTED_HI_OFST: c_int = 28;
// Enum values, see field(s):
// 100M
// Supported loopbacks.
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_40G_OFST: c_int = 32;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_40G_LEN: c_int = 8;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_40G_LO_OFST: c_int = 32;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_40G_HI_OFST: c_int = 36;
// Enum values, see field(s):
// 100M
// MC_CMD_GET_LOOPBACK_MODES_OUT_V2 msgresponse: Supported loopback modes for
// newer NICs with 25G/50G/100G support
//
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_V2_LEN: c_int = 64;
// Supported loopbacks.
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_V2_100M_OFST: c_int = 0;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_V2_100M_LEN: c_int = 8;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_V2_100M_LO_OFST: c_int = 0;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_V2_100M_HI_OFST: c_int = 4;
// enum: None.
// MC_CMD_LOOPBACK_NONE 0x0
// enum: Data.
// MC_CMD_LOOPBACK_DATA 0x1
// enum: GMAC.
// MC_CMD_LOOPBACK_GMAC 0x2
// enum: XGMII.
// MC_CMD_LOOPBACK_XGMII 0x3
// enum: XGXS.
// MC_CMD_LOOPBACK_XGXS 0x4
// enum: XAUI.
// MC_CMD_LOOPBACK_XAUI 0x5
// enum: GMII.
// MC_CMD_LOOPBACK_GMII 0x6
// enum: SGMII.
// MC_CMD_LOOPBACK_SGMII 0x7
// enum: XGBR.
// MC_CMD_LOOPBACK_XGBR 0x8
// enum: XFI.
// MC_CMD_LOOPBACK_XFI 0x9
// enum: XAUI Far.
// MC_CMD_LOOPBACK_XAUI_FAR 0xa
// enum: GMII Far.
// MC_CMD_LOOPBACK_GMII_FAR 0xb
// enum: SGMII Far.
// MC_CMD_LOOPBACK_SGMII_FAR 0xc
// enum: XFI Far.
// MC_CMD_LOOPBACK_XFI_FAR 0xd
// enum: GPhy.
// MC_CMD_LOOPBACK_GPHY 0xe
// enum: PhyXS.
// MC_CMD_LOOPBACK_PHYXS 0xf
// enum: PCS.
// MC_CMD_LOOPBACK_PCS 0x10
// enum: PMA-PMD.
// MC_CMD_LOOPBACK_PMAPMD 0x11
// enum: Cross-Port.
// MC_CMD_LOOPBACK_XPORT 0x12
// enum: XGMII-Wireside.
// MC_CMD_LOOPBACK_XGMII_WS 0x13
// enum: XAUI Wireside.
// MC_CMD_LOOPBACK_XAUI_WS 0x14
// enum: XAUI Wireside Far.
// MC_CMD_LOOPBACK_XAUI_WS_FAR 0x15
// enum: XAUI Wireside near.
// MC_CMD_LOOPBACK_XAUI_WS_NEAR 0x16
// enum: GMII Wireside.
// MC_CMD_LOOPBACK_GMII_WS 0x17
// enum: XFI Wireside.
// MC_CMD_LOOPBACK_XFI_WS 0x18
// enum: XFI Wireside Far.
// MC_CMD_LOOPBACK_XFI_WS_FAR 0x19
// enum: PhyXS Wireside.
// MC_CMD_LOOPBACK_PHYXS_WS 0x1a
// enum: PMA lanes MAC-Serdes.
// MC_CMD_LOOPBACK_PMA_INT 0x1b
// enum: KR Serdes Parallel (Encoder).
// MC_CMD_LOOPBACK_SD_NEAR 0x1c
// enum: KR Serdes Serial.
// MC_CMD_LOOPBACK_SD_FAR 0x1d
// enum: PMA lanes MAC-Serdes Wireside.
// MC_CMD_LOOPBACK_PMA_INT_WS 0x1e
// enum: KR Serdes Parallel Wireside (Full PCS).
// MC_CMD_LOOPBACK_SD_FEP2_WS 0x1f
// enum: KR Serdes Parallel Wireside (Sym Aligner to TX).
// MC_CMD_LOOPBACK_SD_FEP1_5_WS 0x20
// enum: KR Serdes Parallel Wireside (Deserializer to Serializer).
// MC_CMD_LOOPBACK_SD_FEP_WS 0x21
// enum: KR Serdes Serial Wireside.
// MC_CMD_LOOPBACK_SD_FES_WS 0x22
// enum: Near side of AOE Siena side port
// MC_CMD_LOOPBACK_AOE_INT_NEAR 0x23
// enum: Medford Wireside datapath loopback
// MC_CMD_LOOPBACK_DATA_WS 0x24
// enum: Force link up without setting up any physical loopback (snapper use
// only)
//
// MC_CMD_LOOPBACK_FORCE_EXT_LINK 0x25
// Supported loopbacks.
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_V2_1G_OFST: c_int = 8;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_V2_1G_LEN: c_int = 8;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_V2_1G_LO_OFST: c_int = 8;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_V2_1G_HI_OFST: c_int = 12;
// Enum values, see field(s):
// 100M
// Supported loopbacks.
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_V2_10G_OFST: c_int = 16;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_V2_10G_LEN: c_int = 8;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_V2_10G_LO_OFST: c_int = 16;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_V2_10G_HI_OFST: c_int = 20;
// Enum values, see field(s):
// 100M
// Supported loopbacks.
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_V2_SUGGESTED_OFST: c_int = 24;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_V2_SUGGESTED_LEN: c_int = 8;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_V2_SUGGESTED_LO_OFST: c_int = 24;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_V2_SUGGESTED_HI_OFST: c_int = 28;
// Enum values, see field(s):
// 100M
// Supported loopbacks.
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_V2_40G_OFST: c_int = 32;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_V2_40G_LEN: c_int = 8;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_V2_40G_LO_OFST: c_int = 32;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_V2_40G_HI_OFST: c_int = 36;
// Enum values, see field(s):
// 100M
// Supported 25G loopbacks.
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_V2_25G_OFST: c_int = 40;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_V2_25G_LEN: c_int = 8;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_V2_25G_LO_OFST: c_int = 40;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_V2_25G_HI_OFST: c_int = 44;
// Enum values, see field(s):
// 100M
// Supported 50 loopbacks.
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_V2_50G_OFST: c_int = 48;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_V2_50G_LEN: c_int = 8;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_V2_50G_LO_OFST: c_int = 48;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_V2_50G_HI_OFST: c_int = 52;
// Enum values, see field(s):
// 100M
// Supported 100G loopbacks.
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_V2_100G_OFST: c_int = 56;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_V2_100G_LEN: c_int = 8;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_V2_100G_LO_OFST: c_int = 56;
pub const MC_CMD_GET_LOOPBACK_MODES_OUT_V2_100G_HI_OFST: c_int = 60;
// Enum values, see field(s):
// 100M
// AN_TYPE structuredef: Auto-negotiation types defined in IEEE802.3
pub const AN_TYPE_LEN: c_int = 4;
pub const AN_TYPE_TYPE_OFST: c_int = 0;
pub const AN_TYPE_TYPE_LEN: c_int = 4;
// enum: None, AN disabled or not supported
pub const MC_CMD_AN_NONE: c_uint = 0x0;
// enum: Clause 28 - BASE-T
pub const MC_CMD_AN_CLAUSE28: c_uint = 0x1;
// enum: Clause 37 - BASE-X
pub const MC_CMD_AN_CLAUSE37: c_uint = 0x2;
// enum: Clause 73 - BASE-R startup protocol for backplane and copper cable
// assemblies. Includes Clause 72/Clause 92 link-training.
//
pub const MC_CMD_AN_CLAUSE73: c_uint = 0x3;
pub const AN_TYPE_TYPE_LBN: c_int = 0;
pub const AN_TYPE_TYPE_WIDTH: c_int = 32;
// FEC_TYPE structuredef: Forward error correction types defined in IEEE802.3
//
pub const FEC_TYPE_LEN: c_int = 4;
pub const FEC_TYPE_TYPE_OFST: c_int = 0;
pub const FEC_TYPE_TYPE_LEN: c_int = 4;
// enum: No FEC
pub const MC_CMD_FEC_NONE: c_uint = 0x0;
// enum: Clause 74 BASE-R FEC (a.k.a Firecode)
pub const MC_CMD_FEC_BASER: c_uint = 0x1;
// enum: Clause 91/Clause 108 Reed-Solomon FEC
pub const MC_CMD_FEC_RS: c_uint = 0x2;
pub const FEC_TYPE_TYPE_LBN: c_int = 0;
pub const FEC_TYPE_TYPE_WIDTH: c_int = 32;
//
// MC_CMD_GET_LINK
// Read the unified MAC/PHY link state. Locks required: None Return code: 0,
// ETIME.
//
pub const MC_CMD_GET_LINK: c_uint = 0x29;

// MC_CMD_GET_LINK_IN msgrequest
pub const MC_CMD_GET_LINK_IN_LEN: c_int = 0;
// MC_CMD_GET_LINK_OUT msgresponse
pub const MC_CMD_GET_LINK_OUT_LEN: c_int = 28;
// Near-side advertised capabilities. Refer to
// MC_CMD_GET_PHY_CFG_OUT/SUPPORTED_CAP for bit definitions.
//
pub const MC_CMD_GET_LINK_OUT_CAP_OFST: c_int = 0;
pub const MC_CMD_GET_LINK_OUT_CAP_LEN: c_int = 4;
// Link-partner advertised capabilities. Refer to
// MC_CMD_GET_PHY_CFG_OUT/SUPPORTED_CAP for bit definitions.
//
pub const MC_CMD_GET_LINK_OUT_LP_CAP_OFST: c_int = 4;
pub const MC_CMD_GET_LINK_OUT_LP_CAP_LEN: c_int = 4;
// Autonegotiated speed in mbit/s. The link may still be down even if this
// reads non-zero.
//
pub const MC_CMD_GET_LINK_OUT_LINK_SPEED_OFST: c_int = 8;
pub const MC_CMD_GET_LINK_OUT_LINK_SPEED_LEN: c_int = 4;
// Current loopback setting.
pub const MC_CMD_GET_LINK_OUT_LOOPBACK_MODE_OFST: c_int = 12;
pub const MC_CMD_GET_LINK_OUT_LOOPBACK_MODE_LEN: c_int = 4;
// Enum values, see field(s):
// MC_CMD_GET_LOOPBACK_MODES/MC_CMD_GET_LOOPBACK_MODES_OUT/100M
pub const MC_CMD_GET_LINK_OUT_FLAGS_OFST: c_int = 16;
pub const MC_CMD_GET_LINK_OUT_FLAGS_LEN: c_int = 4;
pub const MC_CMD_GET_LINK_OUT_LINK_UP_OFST: c_int = 16;
pub const MC_CMD_GET_LINK_OUT_LINK_UP_LBN: c_int = 0;
pub const MC_CMD_GET_LINK_OUT_LINK_UP_WIDTH: c_int = 1;
pub const MC_CMD_GET_LINK_OUT_FULL_DUPLEX_OFST: c_int = 16;
pub const MC_CMD_GET_LINK_OUT_FULL_DUPLEX_LBN: c_int = 1;
pub const MC_CMD_GET_LINK_OUT_FULL_DUPLEX_WIDTH: c_int = 1;
pub const MC_CMD_GET_LINK_OUT_BPX_LINK_OFST: c_int = 16;
pub const MC_CMD_GET_LINK_OUT_BPX_LINK_LBN: c_int = 2;
pub const MC_CMD_GET_LINK_OUT_BPX_LINK_WIDTH: c_int = 1;
pub const MC_CMD_GET_LINK_OUT_PHY_LINK_OFST: c_int = 16;
pub const MC_CMD_GET_LINK_OUT_PHY_LINK_LBN: c_int = 3;
pub const MC_CMD_GET_LINK_OUT_PHY_LINK_WIDTH: c_int = 1;
pub const MC_CMD_GET_LINK_OUT_LINK_FAULT_RX_OFST: c_int = 16;
pub const MC_CMD_GET_LINK_OUT_LINK_FAULT_RX_LBN: c_int = 6;
pub const MC_CMD_GET_LINK_OUT_LINK_FAULT_RX_WIDTH: c_int = 1;
pub const MC_CMD_GET_LINK_OUT_LINK_FAULT_TX_OFST: c_int = 16;
pub const MC_CMD_GET_LINK_OUT_LINK_FAULT_TX_LBN: c_int = 7;
pub const MC_CMD_GET_LINK_OUT_LINK_FAULT_TX_WIDTH: c_int = 1;
pub const MC_CMD_GET_LINK_OUT_MODULE_UP_VALID_OFST: c_int = 16;
pub const MC_CMD_GET_LINK_OUT_MODULE_UP_VALID_LBN: c_int = 8;
pub const MC_CMD_GET_LINK_OUT_MODULE_UP_VALID_WIDTH: c_int = 1;
pub const MC_CMD_GET_LINK_OUT_MODULE_UP_OFST: c_int = 16;
pub const MC_CMD_GET_LINK_OUT_MODULE_UP_LBN: c_int = 9;
pub const MC_CMD_GET_LINK_OUT_MODULE_UP_WIDTH: c_int = 1;
// This returns the negotiated flow control value.
pub const MC_CMD_GET_LINK_OUT_FCNTL_OFST: c_int = 20;
pub const MC_CMD_GET_LINK_OUT_FCNTL_LEN: c_int = 4;
// Enum values, see field(s):
// MC_CMD_SET_MAC/MC_CMD_SET_MAC_IN/FCNTL
pub const MC_CMD_GET_LINK_OUT_MAC_FAULT_OFST: c_int = 24;
pub const MC_CMD_GET_LINK_OUT_MAC_FAULT_LEN: c_int = 4;
pub const MC_CMD_MAC_FAULT_XGMII_LOCAL_OFST: c_int = 24;
pub const MC_CMD_MAC_FAULT_XGMII_LOCAL_LBN: c_int = 0;
pub const MC_CMD_MAC_FAULT_XGMII_LOCAL_WIDTH: c_int = 1;
pub const MC_CMD_MAC_FAULT_XGMII_REMOTE_OFST: c_int = 24;
pub const MC_CMD_MAC_FAULT_XGMII_REMOTE_LBN: c_int = 1;
pub const MC_CMD_MAC_FAULT_XGMII_REMOTE_WIDTH: c_int = 1;
pub const MC_CMD_MAC_FAULT_SGMII_REMOTE_OFST: c_int = 24;
pub const MC_CMD_MAC_FAULT_SGMII_REMOTE_LBN: c_int = 2;
pub const MC_CMD_MAC_FAULT_SGMII_REMOTE_WIDTH: c_int = 1;
pub const MC_CMD_MAC_FAULT_PENDING_RECONFIG_OFST: c_int = 24;
pub const MC_CMD_MAC_FAULT_PENDING_RECONFIG_LBN: c_int = 3;
pub const MC_CMD_MAC_FAULT_PENDING_RECONFIG_WIDTH: c_int = 1;
// MC_CMD_GET_LINK_OUT_V2 msgresponse: Extended link state information
pub const MC_CMD_GET_LINK_OUT_V2_LEN: c_int = 44;
// Near-side advertised capabilities. Refer to
// MC_CMD_GET_PHY_CFG_OUT/SUPPORTED_CAP for bit definitions.
//
pub const MC_CMD_GET_LINK_OUT_V2_CAP_OFST: c_int = 0;
pub const MC_CMD_GET_LINK_OUT_V2_CAP_LEN: c_int = 4;
// Link-partner advertised capabilities. Refer to
// MC_CMD_GET_PHY_CFG_OUT/SUPPORTED_CAP for bit definitions.
//
pub const MC_CMD_GET_LINK_OUT_V2_LP_CAP_OFST: c_int = 4;
pub const MC_CMD_GET_LINK_OUT_V2_LP_CAP_LEN: c_int = 4;
// Autonegotiated speed in mbit/s. The link may still be down even if this
// reads non-zero.
//
pub const MC_CMD_GET_LINK_OUT_V2_LINK_SPEED_OFST: c_int = 8;
pub const MC_CMD_GET_LINK_OUT_V2_LINK_SPEED_LEN: c_int = 4;
// Current loopback setting.
pub const MC_CMD_GET_LINK_OUT_V2_LOOPBACK_MODE_OFST: c_int = 12;
pub const MC_CMD_GET_LINK_OUT_V2_LOOPBACK_MODE_LEN: c_int = 4;
// Enum values, see field(s):
// MC_CMD_GET_LOOPBACK_MODES/MC_CMD_GET_LOOPBACK_MODES_OUT/100M
pub const MC_CMD_GET_LINK_OUT_V2_FLAGS_OFST: c_int = 16;
pub const MC_CMD_GET_LINK_OUT_V2_FLAGS_LEN: c_int = 4;
pub const MC_CMD_GET_LINK_OUT_V2_LINK_UP_OFST: c_int = 16;
pub const MC_CMD_GET_LINK_OUT_V2_LINK_UP_LBN: c_int = 0;
pub const MC_CMD_GET_LINK_OUT_V2_LINK_UP_WIDTH: c_int = 1;
pub const MC_CMD_GET_LINK_OUT_V2_FULL_DUPLEX_OFST: c_int = 16;
pub const MC_CMD_GET_LINK_OUT_V2_FULL_DUPLEX_LBN: c_int = 1;
pub const MC_CMD_GET_LINK_OUT_V2_FULL_DUPLEX_WIDTH: c_int = 1;
pub const MC_CMD_GET_LINK_OUT_V2_BPX_LINK_OFST: c_int = 16;
pub const MC_CMD_GET_LINK_OUT_V2_BPX_LINK_LBN: c_int = 2;
pub const MC_CMD_GET_LINK_OUT_V2_BPX_LINK_WIDTH: c_int = 1;
pub const MC_CMD_GET_LINK_OUT_V2_PHY_LINK_OFST: c_int = 16;
pub const MC_CMD_GET_LINK_OUT_V2_PHY_LINK_LBN: c_int = 3;
pub const MC_CMD_GET_LINK_OUT_V2_PHY_LINK_WIDTH: c_int = 1;
pub const MC_CMD_GET_LINK_OUT_V2_LINK_FAULT_RX_OFST: c_int = 16;
pub const MC_CMD_GET_LINK_OUT_V2_LINK_FAULT_RX_LBN: c_int = 6;
pub const MC_CMD_GET_LINK_OUT_V2_LINK_FAULT_RX_WIDTH: c_int = 1;
pub const MC_CMD_GET_LINK_OUT_V2_LINK_FAULT_TX_OFST: c_int = 16;
pub const MC_CMD_GET_LINK_OUT_V2_LINK_FAULT_TX_LBN: c_int = 7;
pub const MC_CMD_GET_LINK_OUT_V2_LINK_FAULT_TX_WIDTH: c_int = 1;
pub const MC_CMD_GET_LINK_OUT_V2_MODULE_UP_VALID_OFST: c_int = 16;
pub const MC_CMD_GET_LINK_OUT_V2_MODULE_UP_VALID_LBN: c_int = 8;
pub const MC_CMD_GET_LINK_OUT_V2_MODULE_UP_VALID_WIDTH: c_int = 1;
pub const MC_CMD_GET_LINK_OUT_V2_MODULE_UP_OFST: c_int = 16;
pub const MC_CMD_GET_LINK_OUT_V2_MODULE_UP_LBN: c_int = 9;
pub const MC_CMD_GET_LINK_OUT_V2_MODULE_UP_WIDTH: c_int = 1;
// This returns the negotiated flow control value.
pub const MC_CMD_GET_LINK_OUT_V2_FCNTL_OFST: c_int = 20;
pub const MC_CMD_GET_LINK_OUT_V2_FCNTL_LEN: c_int = 4;
// Enum values, see field(s):
// MC_CMD_SET_MAC/MC_CMD_SET_MAC_IN/FCNTL
pub const MC_CMD_GET_LINK_OUT_V2_MAC_FAULT_OFST: c_int = 24;
pub const MC_CMD_GET_LINK_OUT_V2_MAC_FAULT_LEN: c_int = 4;
// MC_CMD_MAC_FAULT_XGMII_LOCAL_OFST 24
// MC_CMD_MAC_FAULT_XGMII_LOCAL_LBN 0
// MC_CMD_MAC_FAULT_XGMII_LOCAL_WIDTH 1
// MC_CMD_MAC_FAULT_XGMII_REMOTE_OFST 24
// MC_CMD_MAC_FAULT_XGMII_REMOTE_LBN 1
// MC_CMD_MAC_FAULT_XGMII_REMOTE_WIDTH 1
// MC_CMD_MAC_FAULT_SGMII_REMOTE_OFST 24
// MC_CMD_MAC_FAULT_SGMII_REMOTE_LBN 2
// MC_CMD_MAC_FAULT_SGMII_REMOTE_WIDTH 1
// MC_CMD_MAC_FAULT_PENDING_RECONFIG_OFST 24
// MC_CMD_MAC_FAULT_PENDING_RECONFIG_LBN 3
// MC_CMD_MAC_FAULT_PENDING_RECONFIG_WIDTH 1
// True local device capabilities (taking into account currently used PMD/MDI,
// e.g. plugged-in module). In general, subset of
// MC_CMD_GET_PHY_CFG_OUT/SUPPORTED_CAP, but may include extra _FEC_REQUEST
// bits, if the PMD requires FEC. 0 if unknown (e.g. module unplugged). Equal
// to SUPPORTED_CAP for non-pluggable PMDs. Refer to
// MC_CMD_GET_PHY_CFG_OUT/SUPPORTED_CAP for bit definitions.
//
pub const MC_CMD_GET_LINK_OUT_V2_LD_CAP_OFST: c_int = 28;
pub const MC_CMD_GET_LINK_OUT_V2_LD_CAP_LEN: c_int = 4;
// Auto-negotiation type used on the link
pub const MC_CMD_GET_LINK_OUT_V2_AN_TYPE_OFST: c_int = 32;
pub const MC_CMD_GET_LINK_OUT_V2_AN_TYPE_LEN: c_int = 4;
// Enum values, see field(s):
// AN_TYPE/TYPE
// Forward error correction used on the link
pub const MC_CMD_GET_LINK_OUT_V2_FEC_TYPE_OFST: c_int = 36;
pub const MC_CMD_GET_LINK_OUT_V2_FEC_TYPE_LEN: c_int = 4;
// Enum values, see field(s):
// FEC_TYPE/TYPE
pub const MC_CMD_GET_LINK_OUT_V2_EXT_FLAGS_OFST: c_int = 40;
pub const MC_CMD_GET_LINK_OUT_V2_EXT_FLAGS_LEN: c_int = 4;
pub const MC_CMD_GET_LINK_OUT_V2_PMD_MDI_CONNECTED_OFST: c_int = 40;
pub const MC_CMD_GET_LINK_OUT_V2_PMD_MDI_CONNECTED_LBN: c_int = 0;
pub const MC_CMD_GET_LINK_OUT_V2_PMD_MDI_CONNECTED_WIDTH: c_int = 1;
pub const MC_CMD_GET_LINK_OUT_V2_PMD_READY_OFST: c_int = 40;
pub const MC_CMD_GET_LINK_OUT_V2_PMD_READY_LBN: c_int = 1;
pub const MC_CMD_GET_LINK_OUT_V2_PMD_READY_WIDTH: c_int = 1;
pub const MC_CMD_GET_LINK_OUT_V2_PMD_LINK_UP_OFST: c_int = 40;
pub const MC_CMD_GET_LINK_OUT_V2_PMD_LINK_UP_LBN: c_int = 2;
pub const MC_CMD_GET_LINK_OUT_V2_PMD_LINK_UP_WIDTH: c_int = 1;
pub const MC_CMD_GET_LINK_OUT_V2_PMA_LINK_UP_OFST: c_int = 40;
pub const MC_CMD_GET_LINK_OUT_V2_PMA_LINK_UP_LBN: c_int = 3;
pub const MC_CMD_GET_LINK_OUT_V2_PMA_LINK_UP_WIDTH: c_int = 1;
pub const MC_CMD_GET_LINK_OUT_V2_PCS_LOCK_OFST: c_int = 40;
pub const MC_CMD_GET_LINK_OUT_V2_PCS_LOCK_LBN: c_int = 4;
pub const MC_CMD_GET_LINK_OUT_V2_PCS_LOCK_WIDTH: c_int = 1;
pub const MC_CMD_GET_LINK_OUT_V2_ALIGN_LOCK_OFST: c_int = 40;
pub const MC_CMD_GET_LINK_OUT_V2_ALIGN_LOCK_LBN: c_int = 5;
pub const MC_CMD_GET_LINK_OUT_V2_ALIGN_LOCK_WIDTH: c_int = 1;
pub const MC_CMD_GET_LINK_OUT_V2_HI_BER_OFST: c_int = 40;
pub const MC_CMD_GET_LINK_OUT_V2_HI_BER_LBN: c_int = 6;
pub const MC_CMD_GET_LINK_OUT_V2_HI_BER_WIDTH: c_int = 1;
pub const MC_CMD_GET_LINK_OUT_V2_FEC_LOCK_OFST: c_int = 40;
pub const MC_CMD_GET_LINK_OUT_V2_FEC_LOCK_LBN: c_int = 7;
pub const MC_CMD_GET_LINK_OUT_V2_FEC_LOCK_WIDTH: c_int = 1;
pub const MC_CMD_GET_LINK_OUT_V2_AN_DONE_OFST: c_int = 40;
pub const MC_CMD_GET_LINK_OUT_V2_AN_DONE_LBN: c_int = 8;
pub const MC_CMD_GET_LINK_OUT_V2_AN_DONE_WIDTH: c_int = 1;
pub const MC_CMD_GET_LINK_OUT_V2_PORT_SHUTDOWN_OFST: c_int = 40;
pub const MC_CMD_GET_LINK_OUT_V2_PORT_SHUTDOWN_LBN: c_int = 9;
pub const MC_CMD_GET_LINK_OUT_V2_PORT_SHUTDOWN_WIDTH: c_int = 1;
//
// MC_CMD_SET_LINK
// Write the unified MAC/PHY link configuration. Locks required: None. Return
// code: 0, EINVAL, ETIME, EAGAIN
//
pub const MC_CMD_SET_LINK: c_uint = 0x2a;

// MC_CMD_SET_LINK_IN msgrequest
pub const MC_CMD_SET_LINK_IN_LEN: c_int = 16;
// Near-side advertised capabilities. Refer to
// MC_CMD_GET_PHY_CFG_OUT/SUPPORTED_CAP for bit definitions.
//
pub const MC_CMD_SET_LINK_IN_CAP_OFST: c_int = 0;
pub const MC_CMD_SET_LINK_IN_CAP_LEN: c_int = 4;
// Flags
pub const MC_CMD_SET_LINK_IN_FLAGS_OFST: c_int = 4;
pub const MC_CMD_SET_LINK_IN_FLAGS_LEN: c_int = 4;
pub const MC_CMD_SET_LINK_IN_LOWPOWER_OFST: c_int = 4;
pub const MC_CMD_SET_LINK_IN_LOWPOWER_LBN: c_int = 0;
pub const MC_CMD_SET_LINK_IN_LOWPOWER_WIDTH: c_int = 1;
pub const MC_CMD_SET_LINK_IN_POWEROFF_OFST: c_int = 4;
pub const MC_CMD_SET_LINK_IN_POWEROFF_LBN: c_int = 1;
pub const MC_CMD_SET_LINK_IN_POWEROFF_WIDTH: c_int = 1;
pub const MC_CMD_SET_LINK_IN_TXDIS_OFST: c_int = 4;
pub const MC_CMD_SET_LINK_IN_TXDIS_LBN: c_int = 2;
pub const MC_CMD_SET_LINK_IN_TXDIS_WIDTH: c_int = 1;
pub const MC_CMD_SET_LINK_IN_LINKDOWN_OFST: c_int = 4;
pub const MC_CMD_SET_LINK_IN_LINKDOWN_LBN: c_int = 3;
pub const MC_CMD_SET_LINK_IN_LINKDOWN_WIDTH: c_int = 1;
// Loopback mode.
pub const MC_CMD_SET_LINK_IN_LOOPBACK_MODE_OFST: c_int = 8;
pub const MC_CMD_SET_LINK_IN_LOOPBACK_MODE_LEN: c_int = 4;
// Enum values, see field(s):
// MC_CMD_GET_LOOPBACK_MODES/MC_CMD_GET_LOOPBACK_MODES_OUT/100M
// A loopback speed of "0" is supported, and means (choose any available
// speed).
//
pub const MC_CMD_SET_LINK_IN_LOOPBACK_SPEED_OFST: c_int = 12;
pub const MC_CMD_SET_LINK_IN_LOOPBACK_SPEED_LEN: c_int = 4;
// MC_CMD_SET_LINK_IN_V2 msgrequest: Updated SET_LINK to include sequence
// number to ensure this SET_LINK command corresponds to the latest
// MODULECHANGE event.
//
pub const MC_CMD_SET_LINK_IN_V2_LEN: c_int = 17;
// Near-side advertised capabilities. Refer to
// MC_CMD_GET_PHY_CFG_OUT/SUPPORTED_CAP for bit definitions.
//
pub const MC_CMD_SET_LINK_IN_V2_CAP_OFST: c_int = 0;
pub const MC_CMD_SET_LINK_IN_V2_CAP_LEN: c_int = 4;
// Flags
pub const MC_CMD_SET_LINK_IN_V2_FLAGS_OFST: c_int = 4;
pub const MC_CMD_SET_LINK_IN_V2_FLAGS_LEN: c_int = 4;
pub const MC_CMD_SET_LINK_IN_V2_LOWPOWER_OFST: c_int = 4;
pub const MC_CMD_SET_LINK_IN_V2_LOWPOWER_LBN: c_int = 0;
pub const MC_CMD_SET_LINK_IN_V2_LOWPOWER_WIDTH: c_int = 1;
pub const MC_CMD_SET_LINK_IN_V2_POWEROFF_OFST: c_int = 4;
pub const MC_CMD_SET_LINK_IN_V2_POWEROFF_LBN: c_int = 1;
pub const MC_CMD_SET_LINK_IN_V2_POWEROFF_WIDTH: c_int = 1;
pub const MC_CMD_SET_LINK_IN_V2_TXDIS_OFST: c_int = 4;
pub const MC_CMD_SET_LINK_IN_V2_TXDIS_LBN: c_int = 2;
pub const MC_CMD_SET_LINK_IN_V2_TXDIS_WIDTH: c_int = 1;
pub const MC_CMD_SET_LINK_IN_V2_LINKDOWN_OFST: c_int = 4;
pub const MC_CMD_SET_LINK_IN_V2_LINKDOWN_LBN: c_int = 3;
pub const MC_CMD_SET_LINK_IN_V2_LINKDOWN_WIDTH: c_int = 1;
// Loopback mode.
pub const MC_CMD_SET_LINK_IN_V2_LOOPBACK_MODE_OFST: c_int = 8;
pub const MC_CMD_SET_LINK_IN_V2_LOOPBACK_MODE_LEN: c_int = 4;
// Enum values, see field(s):
// MC_CMD_GET_LOOPBACK_MODES/MC_CMD_GET_LOOPBACK_MODES_OUT/100M
// A loopback speed of "0" is supported, and means (choose any available
// speed).
//
pub const MC_CMD_SET_LINK_IN_V2_LOOPBACK_SPEED_OFST: c_int = 12;
pub const MC_CMD_SET_LINK_IN_V2_LOOPBACK_SPEED_LEN: c_int = 4;
pub const MC_CMD_SET_LINK_IN_V2_MODULE_SEQ_OFST: c_int = 16;
pub const MC_CMD_SET_LINK_IN_V2_MODULE_SEQ_LEN: c_int = 1;
pub const MC_CMD_SET_LINK_IN_V2_MODULE_SEQ_NUMBER_OFST: c_int = 16;
pub const MC_CMD_SET_LINK_IN_V2_MODULE_SEQ_NUMBER_LBN: c_int = 0;
pub const MC_CMD_SET_LINK_IN_V2_MODULE_SEQ_NUMBER_WIDTH: c_int = 7;
pub const MC_CMD_SET_LINK_IN_V2_MODULE_SEQ_IGNORE_OFST: c_int = 16;
pub const MC_CMD_SET_LINK_IN_V2_MODULE_SEQ_IGNORE_LBN: c_int = 7;
pub const MC_CMD_SET_LINK_IN_V2_MODULE_SEQ_IGNORE_WIDTH: c_int = 1;
// MC_CMD_SET_LINK_OUT msgresponse
pub const MC_CMD_SET_LINK_OUT_LEN: c_int = 0;
//
// MC_CMD_SET_ID_LED
// Set identification LED state. Locks required: None. Return code: 0, EINVAL
//
pub const MC_CMD_SET_ID_LED: c_uint = 0x2b;

// MC_CMD_SET_ID_LED_IN msgrequest
pub const MC_CMD_SET_ID_LED_IN_LEN: c_int = 4;
// Set LED state.
pub const MC_CMD_SET_ID_LED_IN_STATE_OFST: c_int = 0;
pub const MC_CMD_SET_ID_LED_IN_STATE_LEN: c_int = 4;
pub const MC_CMD_LED_OFF: c_uint = 0x0 /* enum */;
pub const MC_CMD_LED_ON: c_uint = 0x1 /* enum */;
pub const MC_CMD_LED_DEFAULT: c_uint = 0x2 /* enum */;
// MC_CMD_SET_ID_LED_OUT msgresponse
pub const MC_CMD_SET_ID_LED_OUT_LEN: c_int = 0;
//
// MC_CMD_SET_MAC
// Set MAC configuration. Locks required: None. Return code: 0, EINVAL
//
pub const MC_CMD_SET_MAC: c_uint = 0x2c;

// MC_CMD_SET_MAC_IN msgrequest
pub const MC_CMD_SET_MAC_IN_LEN: c_int = 28;
// The MTU is the MTU programmed directly into the XMAC/GMAC (inclusive of
// EtherII, VLAN, bug16011 padding).
//
pub const MC_CMD_SET_MAC_IN_MTU_OFST: c_int = 0;
pub const MC_CMD_SET_MAC_IN_MTU_LEN: c_int = 4;
pub const MC_CMD_SET_MAC_IN_DRAIN_OFST: c_int = 4;
pub const MC_CMD_SET_MAC_IN_DRAIN_LEN: c_int = 4;
pub const MC_CMD_SET_MAC_IN_ADDR_OFST: c_int = 8;
pub const MC_CMD_SET_MAC_IN_ADDR_LEN: c_int = 8;
pub const MC_CMD_SET_MAC_IN_ADDR_LO_OFST: c_int = 8;
pub const MC_CMD_SET_MAC_IN_ADDR_HI_OFST: c_int = 12;
pub const MC_CMD_SET_MAC_IN_REJECT_OFST: c_int = 16;
pub const MC_CMD_SET_MAC_IN_REJECT_LEN: c_int = 4;
pub const MC_CMD_SET_MAC_IN_REJECT_UNCST_OFST: c_int = 16;
pub const MC_CMD_SET_MAC_IN_REJECT_UNCST_LBN: c_int = 0;
pub const MC_CMD_SET_MAC_IN_REJECT_UNCST_WIDTH: c_int = 1;
pub const MC_CMD_SET_MAC_IN_REJECT_BRDCST_OFST: c_int = 16;
pub const MC_CMD_SET_MAC_IN_REJECT_BRDCST_LBN: c_int = 1;
pub const MC_CMD_SET_MAC_IN_REJECT_BRDCST_WIDTH: c_int = 1;
pub const MC_CMD_SET_MAC_IN_FCNTL_OFST: c_int = 20;
pub const MC_CMD_SET_MAC_IN_FCNTL_LEN: c_int = 4;
// enum: Flow control is off.
pub const MC_CMD_FCNTL_OFF: c_uint = 0x0;
// enum: Respond to flow control.
pub const MC_CMD_FCNTL_RESPOND: c_uint = 0x1;
// enum: Respond to and Issue flow control.
pub const MC_CMD_FCNTL_BIDIR: c_uint = 0x2;
// enum: Auto neg flow control.
pub const MC_CMD_FCNTL_AUTO: c_uint = 0x3;
// enum: Priority flow control (eftest builds only).
pub const MC_CMD_FCNTL_QBB: c_uint = 0x4;
// enum: Issue flow control.
pub const MC_CMD_FCNTL_GENERATE: c_uint = 0x5;
pub const MC_CMD_SET_MAC_IN_FLAGS_OFST: c_int = 24;
pub const MC_CMD_SET_MAC_IN_FLAGS_LEN: c_int = 4;
pub const MC_CMD_SET_MAC_IN_FLAG_INCLUDE_FCS_OFST: c_int = 24;
pub const MC_CMD_SET_MAC_IN_FLAG_INCLUDE_FCS_LBN: c_int = 0;
pub const MC_CMD_SET_MAC_IN_FLAG_INCLUDE_FCS_WIDTH: c_int = 1;
// MC_CMD_SET_MAC_EXT_IN msgrequest
pub const MC_CMD_SET_MAC_EXT_IN_LEN: c_int = 32;
// The MTU is the MTU programmed directly into the XMAC/GMAC (inclusive of
// EtherII, VLAN, bug16011 padding).
//
pub const MC_CMD_SET_MAC_EXT_IN_MTU_OFST: c_int = 0;
pub const MC_CMD_SET_MAC_EXT_IN_MTU_LEN: c_int = 4;
pub const MC_CMD_SET_MAC_EXT_IN_DRAIN_OFST: c_int = 4;
pub const MC_CMD_SET_MAC_EXT_IN_DRAIN_LEN: c_int = 4;
pub const MC_CMD_SET_MAC_EXT_IN_ADDR_OFST: c_int = 8;
pub const MC_CMD_SET_MAC_EXT_IN_ADDR_LEN: c_int = 8;
pub const MC_CMD_SET_MAC_EXT_IN_ADDR_LO_OFST: c_int = 8;
pub const MC_CMD_SET_MAC_EXT_IN_ADDR_HI_OFST: c_int = 12;
pub const MC_CMD_SET_MAC_EXT_IN_REJECT_OFST: c_int = 16;
pub const MC_CMD_SET_MAC_EXT_IN_REJECT_LEN: c_int = 4;
pub const MC_CMD_SET_MAC_EXT_IN_REJECT_UNCST_OFST: c_int = 16;
pub const MC_CMD_SET_MAC_EXT_IN_REJECT_UNCST_LBN: c_int = 0;
pub const MC_CMD_SET_MAC_EXT_IN_REJECT_UNCST_WIDTH: c_int = 1;
pub const MC_CMD_SET_MAC_EXT_IN_REJECT_BRDCST_OFST: c_int = 16;
pub const MC_CMD_SET_MAC_EXT_IN_REJECT_BRDCST_LBN: c_int = 1;
pub const MC_CMD_SET_MAC_EXT_IN_REJECT_BRDCST_WIDTH: c_int = 1;
pub const MC_CMD_SET_MAC_EXT_IN_FCNTL_OFST: c_int = 20;
pub const MC_CMD_SET_MAC_EXT_IN_FCNTL_LEN: c_int = 4;
// enum: Flow control is off.
// MC_CMD_FCNTL_OFF 0x0
// enum: Respond to flow control.
// MC_CMD_FCNTL_RESPOND 0x1
// enum: Respond to and Issue flow control.
// MC_CMD_FCNTL_BIDIR 0x2
// enum: Auto neg flow control.
// MC_CMD_FCNTL_AUTO 0x3
// enum: Priority flow control (eftest builds only).
// MC_CMD_FCNTL_QBB 0x4
// enum: Issue flow control.
// MC_CMD_FCNTL_GENERATE 0x5
pub const MC_CMD_SET_MAC_EXT_IN_FLAGS_OFST: c_int = 24;
pub const MC_CMD_SET_MAC_EXT_IN_FLAGS_LEN: c_int = 4;
pub const MC_CMD_SET_MAC_EXT_IN_FLAG_INCLUDE_FCS_OFST: c_int = 24;
pub const MC_CMD_SET_MAC_EXT_IN_FLAG_INCLUDE_FCS_LBN: c_int = 0;
pub const MC_CMD_SET_MAC_EXT_IN_FLAG_INCLUDE_FCS_WIDTH: c_int = 1;
// Select which parameters to configure. A parameter will only be modified if
// the corresponding control flag is set. If SET_MAC_ENHANCED is not set in
// capabilities then this field is ignored (and all flags are assumed to be
// set).
//
pub const MC_CMD_SET_MAC_EXT_IN_CONTROL_OFST: c_int = 28;
pub const MC_CMD_SET_MAC_EXT_IN_CONTROL_LEN: c_int = 4;
pub const MC_CMD_SET_MAC_EXT_IN_CFG_MTU_OFST: c_int = 28;
pub const MC_CMD_SET_MAC_EXT_IN_CFG_MTU_LBN: c_int = 0;
pub const MC_CMD_SET_MAC_EXT_IN_CFG_MTU_WIDTH: c_int = 1;
pub const MC_CMD_SET_MAC_EXT_IN_CFG_DRAIN_OFST: c_int = 28;
pub const MC_CMD_SET_MAC_EXT_IN_CFG_DRAIN_LBN: c_int = 1;
pub const MC_CMD_SET_MAC_EXT_IN_CFG_DRAIN_WIDTH: c_int = 1;
pub const MC_CMD_SET_MAC_EXT_IN_CFG_REJECT_OFST: c_int = 28;
pub const MC_CMD_SET_MAC_EXT_IN_CFG_REJECT_LBN: c_int = 2;
pub const MC_CMD_SET_MAC_EXT_IN_CFG_REJECT_WIDTH: c_int = 1;
pub const MC_CMD_SET_MAC_EXT_IN_CFG_FCNTL_OFST: c_int = 28;
pub const MC_CMD_SET_MAC_EXT_IN_CFG_FCNTL_LBN: c_int = 3;
pub const MC_CMD_SET_MAC_EXT_IN_CFG_FCNTL_WIDTH: c_int = 1;
pub const MC_CMD_SET_MAC_EXT_IN_CFG_FCS_OFST: c_int = 28;
pub const MC_CMD_SET_MAC_EXT_IN_CFG_FCS_LBN: c_int = 4;
pub const MC_CMD_SET_MAC_EXT_IN_CFG_FCS_WIDTH: c_int = 1;
// MC_CMD_SET_MAC_OUT msgresponse
pub const MC_CMD_SET_MAC_OUT_LEN: c_int = 0;
// MC_CMD_SET_MAC_V2_OUT msgresponse
pub const MC_CMD_SET_MAC_V2_OUT_LEN: c_int = 4;
// MTU as configured after processing the request. See comment at
// MC_CMD_SET_MAC_IN/MTU. To query MTU without doing any changes, set CONTROL
// to 0.
//
pub const MC_CMD_SET_MAC_V2_OUT_MTU_OFST: c_int = 0;
pub const MC_CMD_SET_MAC_V2_OUT_MTU_LEN: c_int = 4;
//
// MC_CMD_PHY_STATS
// Get generic PHY statistics. This call returns the statistics for a generic
// PHY in a sparse array (indexed by the enumerate). Each value is represented
// by a 32bit number. If the DMA_ADDR is 0, then no DMA is performed, and the
// statistics may be read from the message response. If DMA_ADDR != 0, then the
// statistics are dmad to that (page-aligned location). Locks required: None.
// Returns: 0, ETIME
//
pub const MC_CMD_PHY_STATS: c_uint = 0x2d;

// MC_CMD_PHY_STATS_IN msgrequest
pub const MC_CMD_PHY_STATS_IN_LEN: c_int = 8;
// ???
pub const MC_CMD_PHY_STATS_IN_DMA_ADDR_OFST: c_int = 0;
pub const MC_CMD_PHY_STATS_IN_DMA_ADDR_LEN: c_int = 8;
pub const MC_CMD_PHY_STATS_IN_DMA_ADDR_LO_OFST: c_int = 0;
pub const MC_CMD_PHY_STATS_IN_DMA_ADDR_HI_OFST: c_int = 4;
// MC_CMD_PHY_STATS_OUT_DMA msgresponse
pub const MC_CMD_PHY_STATS_OUT_DMA_LEN: c_int = 0;
// MC_CMD_PHY_STATS_OUT_NO_DMA msgresponse

pub const MC_CMD_PHY_STATS_OUT_NO_DMA_STATISTICS_OFST: c_int = 0;
pub const MC_CMD_PHY_STATS_OUT_NO_DMA_STATISTICS_LEN: c_int = 4;

// enum: OUI.
pub const MC_CMD_OUI: c_uint = 0x0;
// enum: PMA-PMD Link Up.
pub const MC_CMD_PMA_PMD_LINK_UP: c_uint = 0x1;
// enum: PMA-PMD RX Fault.
pub const MC_CMD_PMA_PMD_RX_FAULT: c_uint = 0x2;
// enum: PMA-PMD TX Fault.
pub const MC_CMD_PMA_PMD_TX_FAULT: c_uint = 0x3;
// enum: PMA-PMD Signal
pub const MC_CMD_PMA_PMD_SIGNAL: c_uint = 0x4;
// enum: PMA-PMD SNR A.
pub const MC_CMD_PMA_PMD_SNR_A: c_uint = 0x5;
// enum: PMA-PMD SNR B.
pub const MC_CMD_PMA_PMD_SNR_B: c_uint = 0x6;
// enum: PMA-PMD SNR C.
pub const MC_CMD_PMA_PMD_SNR_C: c_uint = 0x7;
// enum: PMA-PMD SNR D.
pub const MC_CMD_PMA_PMD_SNR_D: c_uint = 0x8;
// enum: PCS Link Up.
pub const MC_CMD_PCS_LINK_UP: c_uint = 0x9;
// enum: PCS RX Fault.
pub const MC_CMD_PCS_RX_FAULT: c_uint = 0xa;
// enum: PCS TX Fault.
pub const MC_CMD_PCS_TX_FAULT: c_uint = 0xb;
// enum: PCS BER.
pub const MC_CMD_PCS_BER: c_uint = 0xc;
// enum: PCS Block Errors.
pub const MC_CMD_PCS_BLOCK_ERRORS: c_uint = 0xd;
// enum: PhyXS Link Up.
pub const MC_CMD_PHYXS_LINK_UP: c_uint = 0xe;
// enum: PhyXS RX Fault.
pub const MC_CMD_PHYXS_RX_FAULT: c_uint = 0xf;
// enum: PhyXS TX Fault.
pub const MC_CMD_PHYXS_TX_FAULT: c_uint = 0x10;
// enum: PhyXS Align.
pub const MC_CMD_PHYXS_ALIGN: c_uint = 0x11;
// enum: PhyXS Sync.
pub const MC_CMD_PHYXS_SYNC: c_uint = 0x12;
// enum: AN link-up.
pub const MC_CMD_AN_LINK_UP: c_uint = 0x13;
// enum: AN Complete.
pub const MC_CMD_AN_COMPLETE: c_uint = 0x14;
// enum: AN 10GBaseT Status.
pub const MC_CMD_AN_10GBT_STATUS: c_uint = 0x15;
// enum: Clause 22 Link-Up.
pub const MC_CMD_CL22_LINK_UP: c_uint = 0x16;
// enum: (Last entry)
pub const MC_CMD_PHY_NSTATS: c_uint = 0x17;
//
// MC_CMD_MAC_STATS
// Get generic MAC statistics. This call returns unified statistics maintained
// by the MC as it switches between the GMAC and XMAC. The MC will write out
// all supported stats. The driver should zero initialise the buffer to
// guarantee consistent results. If the DMA_ADDR is 0, then no DMA is
// performed, and the statistics may be read from the message response. If
// DMA_ADDR != 0, then the statistics are dmad to that (page-aligned location).
// Locks required: None. The PERIODIC_CLEAR option is not used and now has no
// effect. Returns: 0, ETIME
//
pub const MC_CMD_MAC_STATS: c_uint = 0x2e;

// MC_CMD_MAC_STATS_IN msgrequest
pub const MC_CMD_MAC_STATS_IN_LEN: c_int = 20;
// ???
pub const MC_CMD_MAC_STATS_IN_DMA_ADDR_OFST: c_int = 0;
pub const MC_CMD_MAC_STATS_IN_DMA_ADDR_LEN: c_int = 8;
pub const MC_CMD_MAC_STATS_IN_DMA_ADDR_LO_OFST: c_int = 0;
pub const MC_CMD_MAC_STATS_IN_DMA_ADDR_HI_OFST: c_int = 4;
pub const MC_CMD_MAC_STATS_IN_CMD_OFST: c_int = 8;
pub const MC_CMD_MAC_STATS_IN_CMD_LEN: c_int = 4;
pub const MC_CMD_MAC_STATS_IN_DMA_OFST: c_int = 8;
pub const MC_CMD_MAC_STATS_IN_DMA_LBN: c_int = 0;
pub const MC_CMD_MAC_STATS_IN_DMA_WIDTH: c_int = 1;
pub const MC_CMD_MAC_STATS_IN_CLEAR_OFST: c_int = 8;
pub const MC_CMD_MAC_STATS_IN_CLEAR_LBN: c_int = 1;
pub const MC_CMD_MAC_STATS_IN_CLEAR_WIDTH: c_int = 1;
pub const MC_CMD_MAC_STATS_IN_PERIODIC_CHANGE_OFST: c_int = 8;
pub const MC_CMD_MAC_STATS_IN_PERIODIC_CHANGE_LBN: c_int = 2;
pub const MC_CMD_MAC_STATS_IN_PERIODIC_CHANGE_WIDTH: c_int = 1;
pub const MC_CMD_MAC_STATS_IN_PERIODIC_ENABLE_OFST: c_int = 8;
pub const MC_CMD_MAC_STATS_IN_PERIODIC_ENABLE_LBN: c_int = 3;
pub const MC_CMD_MAC_STATS_IN_PERIODIC_ENABLE_WIDTH: c_int = 1;
pub const MC_CMD_MAC_STATS_IN_PERIODIC_CLEAR_OFST: c_int = 8;
pub const MC_CMD_MAC_STATS_IN_PERIODIC_CLEAR_LBN: c_int = 4;
pub const MC_CMD_MAC_STATS_IN_PERIODIC_CLEAR_WIDTH: c_int = 1;
pub const MC_CMD_MAC_STATS_IN_PERIODIC_NOEVENT_OFST: c_int = 8;
pub const MC_CMD_MAC_STATS_IN_PERIODIC_NOEVENT_LBN: c_int = 5;
pub const MC_CMD_MAC_STATS_IN_PERIODIC_NOEVENT_WIDTH: c_int = 1;
pub const MC_CMD_MAC_STATS_IN_PERIOD_MS_OFST: c_int = 8;
pub const MC_CMD_MAC_STATS_IN_PERIOD_MS_LBN: c_int = 16;
pub const MC_CMD_MAC_STATS_IN_PERIOD_MS_WIDTH: c_int = 16;
// DMA length. Should be set to MAC_STATS_NUM_STATS * sizeof(uint64_t), as
// returned by MC_CMD_GET_CAPABILITIES_V4_OUT. For legacy firmware not
// supporting MC_CMD_GET_CAPABILITIES_V4_OUT, DMA_LEN should be set to
// MC_CMD_MAC_NSTATS * sizeof(uint64_t)
//
pub const MC_CMD_MAC_STATS_IN_DMA_LEN_OFST: c_int = 12;
pub const MC_CMD_MAC_STATS_IN_DMA_LEN_LEN: c_int = 4;
// port id so vadapter stats can be provided
pub const MC_CMD_MAC_STATS_IN_PORT_ID_OFST: c_int = 16;
pub const MC_CMD_MAC_STATS_IN_PORT_ID_LEN: c_int = 4;
// MC_CMD_MAC_STATS_OUT_DMA msgresponse
pub const MC_CMD_MAC_STATS_OUT_DMA_LEN: c_int = 0;
// MC_CMD_MAC_STATS_OUT_NO_DMA msgresponse

pub const MC_CMD_MAC_STATS_OUT_NO_DMA_STATISTICS_OFST: c_int = 0;
pub const MC_CMD_MAC_STATS_OUT_NO_DMA_STATISTICS_LEN: c_int = 8;
pub const MC_CMD_MAC_STATS_OUT_NO_DMA_STATISTICS_LO_OFST: c_int = 0;
pub const MC_CMD_MAC_STATS_OUT_NO_DMA_STATISTICS_HI_OFST: c_int = 4;

pub const MC_CMD_MAC_GENERATION_START: c_uint = 0x0 /* enum */;
pub const MC_CMD_MAC_DMABUF_START: c_uint = 0x1 /* enum */;
pub const MC_CMD_MAC_TX_PKTS: c_uint = 0x1 /* enum */;
pub const MC_CMD_MAC_TX_PAUSE_PKTS: c_uint = 0x2 /* enum */;
pub const MC_CMD_MAC_TX_CONTROL_PKTS: c_uint = 0x3 /* enum */;
pub const MC_CMD_MAC_TX_UNICAST_PKTS: c_uint = 0x4 /* enum */;
pub const MC_CMD_MAC_TX_MULTICAST_PKTS: c_uint = 0x5 /* enum */;
pub const MC_CMD_MAC_TX_BROADCAST_PKTS: c_uint = 0x6 /* enum */;
pub const MC_CMD_MAC_TX_BYTES: c_uint = 0x7 /* enum */;
pub const MC_CMD_MAC_TX_BAD_BYTES: c_uint = 0x8 /* enum */;
pub const MC_CMD_MAC_TX_LT64_PKTS: c_uint = 0x9 /* enum */;
pub const MC_CMD_MAC_TX_64_PKTS: c_uint = 0xa /* enum */;
pub const MC_CMD_MAC_TX_65_TO_127_PKTS: c_uint = 0xb /* enum */;
pub const MC_CMD_MAC_TX_128_TO_255_PKTS: c_uint = 0xc /* enum */;
pub const MC_CMD_MAC_TX_256_TO_511_PKTS: c_uint = 0xd /* enum */;
pub const MC_CMD_MAC_TX_512_TO_1023_PKTS: c_uint = 0xe /* enum */;
pub const MC_CMD_MAC_TX_1024_TO_15XX_PKTS: c_uint = 0xf /* enum */;
pub const MC_CMD_MAC_TX_15XX_TO_JUMBO_PKTS: c_uint = 0x10 /* enum */;
pub const MC_CMD_MAC_TX_GTJUMBO_PKTS: c_uint = 0x11 /* enum */;
pub const MC_CMD_MAC_TX_BAD_FCS_PKTS: c_uint = 0x12 /* enum */;
pub const MC_CMD_MAC_TX_SINGLE_COLLISION_PKTS: c_uint = 0x13 /* enum */;
pub const MC_CMD_MAC_TX_MULTIPLE_COLLISION_PKTS: c_uint = 0x14 /* enum */;
pub const MC_CMD_MAC_TX_EXCESSIVE_COLLISION_PKTS: c_uint = 0x15 /* enum */;
pub const MC_CMD_MAC_TX_LATE_COLLISION_PKTS: c_uint = 0x16 /* enum */;
pub const MC_CMD_MAC_TX_DEFERRED_PKTS: c_uint = 0x17 /* enum */;
pub const MC_CMD_MAC_TX_EXCESSIVE_DEFERRED_PKTS: c_uint = 0x18 /* enum */;
pub const MC_CMD_MAC_TX_NON_TCPUDP_PKTS: c_uint = 0x19 /* enum */;
pub const MC_CMD_MAC_TX_MAC_SRC_ERR_PKTS: c_uint = 0x1a /* enum */;
pub const MC_CMD_MAC_TX_IP_SRC_ERR_PKTS: c_uint = 0x1b /* enum */;
pub const MC_CMD_MAC_RX_PKTS: c_uint = 0x1c /* enum */;
pub const MC_CMD_MAC_RX_PAUSE_PKTS: c_uint = 0x1d /* enum */;
pub const MC_CMD_MAC_RX_GOOD_PKTS: c_uint = 0x1e /* enum */;
pub const MC_CMD_MAC_RX_CONTROL_PKTS: c_uint = 0x1f /* enum */;
pub const MC_CMD_MAC_RX_UNICAST_PKTS: c_uint = 0x20 /* enum */;
pub const MC_CMD_MAC_RX_MULTICAST_PKTS: c_uint = 0x21 /* enum */;
pub const MC_CMD_MAC_RX_BROADCAST_PKTS: c_uint = 0x22 /* enum */;
pub const MC_CMD_MAC_RX_BYTES: c_uint = 0x23 /* enum */;
pub const MC_CMD_MAC_RX_BAD_BYTES: c_uint = 0x24 /* enum */;
pub const MC_CMD_MAC_RX_64_PKTS: c_uint = 0x25 /* enum */;
pub const MC_CMD_MAC_RX_65_TO_127_PKTS: c_uint = 0x26 /* enum */;
pub const MC_CMD_MAC_RX_128_TO_255_PKTS: c_uint = 0x27 /* enum */;
pub const MC_CMD_MAC_RX_256_TO_511_PKTS: c_uint = 0x28 /* enum */;
pub const MC_CMD_MAC_RX_512_TO_1023_PKTS: c_uint = 0x29 /* enum */;
pub const MC_CMD_MAC_RX_1024_TO_15XX_PKTS: c_uint = 0x2a /* enum */;
pub const MC_CMD_MAC_RX_15XX_TO_JUMBO_PKTS: c_uint = 0x2b /* enum */;
pub const MC_CMD_MAC_RX_GTJUMBO_PKTS: c_uint = 0x2c /* enum */;
pub const MC_CMD_MAC_RX_UNDERSIZE_PKTS: c_uint = 0x2d /* enum */;
pub const MC_CMD_MAC_RX_BAD_FCS_PKTS: c_uint = 0x2e /* enum */;
pub const MC_CMD_MAC_RX_OVERFLOW_PKTS: c_uint = 0x2f /* enum */;
pub const MC_CMD_MAC_RX_FALSE_CARRIER_PKTS: c_uint = 0x30 /* enum */;
pub const MC_CMD_MAC_RX_SYMBOL_ERROR_PKTS: c_uint = 0x31 /* enum */;
pub const MC_CMD_MAC_RX_ALIGN_ERROR_PKTS: c_uint = 0x32 /* enum */;
pub const MC_CMD_MAC_RX_LENGTH_ERROR_PKTS: c_uint = 0x33 /* enum */;
pub const MC_CMD_MAC_RX_INTERNAL_ERROR_PKTS: c_uint = 0x34 /* enum */;
pub const MC_CMD_MAC_RX_JABBER_PKTS: c_uint = 0x35 /* enum */;
pub const MC_CMD_MAC_RX_NODESC_DROPS: c_uint = 0x36 /* enum */;
pub const MC_CMD_MAC_RX_LANES01_CHAR_ERR: c_uint = 0x37 /* enum */;
pub const MC_CMD_MAC_RX_LANES23_CHAR_ERR: c_uint = 0x38 /* enum */;
pub const MC_CMD_MAC_RX_LANES01_DISP_ERR: c_uint = 0x39 /* enum */;
pub const MC_CMD_MAC_RX_LANES23_DISP_ERR: c_uint = 0x3a /* enum */;
pub const MC_CMD_MAC_RX_MATCH_FAULT: c_uint = 0x3b /* enum */;
// enum: PM trunc_bb_overflow counter. Valid for EF10 with PM_AND_RXDP_COUNTERS
// capability only.
//
pub const MC_CMD_MAC_PM_TRUNC_BB_OVERFLOW: c_uint = 0x3c;
// enum: PM discard_bb_overflow counter. Valid for EF10 with
// PM_AND_RXDP_COUNTERS capability only.
//
pub const MC_CMD_MAC_PM_DISCARD_BB_OVERFLOW: c_uint = 0x3d;
// enum: PM trunc_vfifo_full counter. Valid for EF10 with PM_AND_RXDP_COUNTERS
// capability only.
//
pub const MC_CMD_MAC_PM_TRUNC_VFIFO_FULL: c_uint = 0x3e;
// enum: PM discard_vfifo_full counter. Valid for EF10 with
// PM_AND_RXDP_COUNTERS capability only.
//
pub const MC_CMD_MAC_PM_DISCARD_VFIFO_FULL: c_uint = 0x3f;
// enum: PM trunc_qbb counter. Valid for EF10 with PM_AND_RXDP_COUNTERS
// capability only.
//
pub const MC_CMD_MAC_PM_TRUNC_QBB: c_uint = 0x40;
// enum: PM discard_qbb counter. Valid for EF10 with PM_AND_RXDP_COUNTERS
// capability only.
//
pub const MC_CMD_MAC_PM_DISCARD_QBB: c_uint = 0x41;
// enum: PM discard_mapping counter. Valid for EF10 with PM_AND_RXDP_COUNTERS
// capability only.
//
pub const MC_CMD_MAC_PM_DISCARD_MAPPING: c_uint = 0x42;
// enum: RXDP counter: Number of packets dropped due to the queue being
// disabled. Valid for EF10 with PM_AND_RXDP_COUNTERS capability only.
//
pub const MC_CMD_MAC_RXDP_Q_DISABLED_PKTS: c_uint = 0x43;
// enum: RXDP counter: Number of packets dropped by the DICPU. Valid for EF10
// with PM_AND_RXDP_COUNTERS capability only.
//
pub const MC_CMD_MAC_RXDP_DI_DROPPED_PKTS: c_uint = 0x45;
// enum: RXDP counter: Number of non-host packets. Valid for EF10 with
// PM_AND_RXDP_COUNTERS capability only.
//
pub const MC_CMD_MAC_RXDP_STREAMING_PKTS: c_uint = 0x46;
// enum: RXDP counter: Number of times an hlb descriptor fetch was performed.
// Valid for EF10 with PM_AND_RXDP_COUNTERS capability only.
//
pub const MC_CMD_MAC_RXDP_HLB_FETCH_CONDITIONS: c_uint = 0x47;
// enum: RXDP counter: Number of times the DPCPU waited for an existing
// descriptor fetch. Valid for EF10 with PM_AND_RXDP_COUNTERS capability only.
//
pub const MC_CMD_MAC_RXDP_HLB_WAIT_CONDITIONS: c_uint = 0x48;
pub const MC_CMD_MAC_VADAPTER_RX_DMABUF_START: c_uint = 0x4c /* enum */;
pub const MC_CMD_MAC_VADAPTER_RX_UNICAST_PACKETS: c_uint = 0x4c /* enum */;
pub const MC_CMD_MAC_VADAPTER_RX_UNICAST_BYTES: c_uint = 0x4d /* enum */;
pub const MC_CMD_MAC_VADAPTER_RX_MULTICAST_PACKETS: c_uint = 0x4e /* enum */;
pub const MC_CMD_MAC_VADAPTER_RX_MULTICAST_BYTES: c_uint = 0x4f /* enum */;
pub const MC_CMD_MAC_VADAPTER_RX_BROADCAST_PACKETS: c_uint = 0x50 /* enum */;
pub const MC_CMD_MAC_VADAPTER_RX_BROADCAST_BYTES: c_uint = 0x51 /* enum */;
pub const MC_CMD_MAC_VADAPTER_RX_BAD_PACKETS: c_uint = 0x52 /* enum */;
pub const MC_CMD_MAC_VADAPTER_RX_BAD_BYTES: c_uint = 0x53 /* enum */;
pub const MC_CMD_MAC_VADAPTER_RX_OVERFLOW: c_uint = 0x54 /* enum */;
pub const MC_CMD_MAC_VADAPTER_TX_DMABUF_START: c_uint = 0x57 /* enum */;
pub const MC_CMD_MAC_VADAPTER_TX_UNICAST_PACKETS: c_uint = 0x57 /* enum */;
pub const MC_CMD_MAC_VADAPTER_TX_UNICAST_BYTES: c_uint = 0x58 /* enum */;
pub const MC_CMD_MAC_VADAPTER_TX_MULTICAST_PACKETS: c_uint = 0x59 /* enum */;
pub const MC_CMD_MAC_VADAPTER_TX_MULTICAST_BYTES: c_uint = 0x5a /* enum */;
pub const MC_CMD_MAC_VADAPTER_TX_BROADCAST_PACKETS: c_uint = 0x5b /* enum */;
pub const MC_CMD_MAC_VADAPTER_TX_BROADCAST_BYTES: c_uint = 0x5c /* enum */;
pub const MC_CMD_MAC_VADAPTER_TX_BAD_PACKETS: c_uint = 0x5d /* enum */;
pub const MC_CMD_MAC_VADAPTER_TX_BAD_BYTES: c_uint = 0x5e /* enum */;
pub const MC_CMD_MAC_VADAPTER_TX_OVERFLOW: c_uint = 0x5f /* enum */;
// enum: Start of GMAC stats buffer space, for Siena only.
pub const MC_CMD_GMAC_DMABUF_START: c_uint = 0x40;
// enum: End of GMAC stats buffer space, for Siena only.
pub const MC_CMD_GMAC_DMABUF_END: c_uint = 0x5f;
// enum: GENERATION_END value, used together with GENERATION_START to verify
// consistency of DMAd data. For legacy firmware / drivers without extended
// stats (more precisely, when DMA_LEN == MC_CMD_MAC_NSTATS
// sizeof(uint64_t)), this entry holds the GENERATION_END value. Otherwise,
// this value is invalid/ reserved and GENERATION_END is written as the last
// 64-bit word of the DMA buffer (at DMA_LEN - sizeof(uint64_t)). Note that
// this is consistent with the legacy behaviour, in the sense that entry 96 is
// the last 64-bit word in the buffer when DMA_LEN == MC_CMD_MAC_NSTATS
// sizeof(uint64_t). See SF-109306-TC, Section 9.2 for details.
//
pub const MC_CMD_MAC_GENERATION_END: c_uint = 0x60;
pub const MC_CMD_MAC_NSTATS: c_uint = 0x61 /* enum */;
// MC_CMD_MAC_STATS_V2_OUT_DMA msgresponse
pub const MC_CMD_MAC_STATS_V2_OUT_DMA_LEN: c_int = 0;
// MC_CMD_MAC_STATS_V2_OUT_NO_DMA msgresponse

pub const MC_CMD_MAC_STATS_V2_OUT_NO_DMA_STATISTICS_OFST: c_int = 0;
pub const MC_CMD_MAC_STATS_V2_OUT_NO_DMA_STATISTICS_LEN: c_int = 8;
pub const MC_CMD_MAC_STATS_V2_OUT_NO_DMA_STATISTICS_LO_OFST: c_int = 0;
pub const MC_CMD_MAC_STATS_V2_OUT_NO_DMA_STATISTICS_HI_OFST: c_int = 4;

// enum: Start of FEC stats buffer space, Medford2 and up
pub const MC_CMD_MAC_FEC_DMABUF_START: c_uint = 0x61;
// enum: Number of uncorrected FEC codewords on link (RS-FEC only for Medford2)
//
pub const MC_CMD_MAC_FEC_UNCORRECTED_ERRORS: c_uint = 0x61;
// enum: Number of corrected FEC codewords on link (RS-FEC only for Medford2)
//
pub const MC_CMD_MAC_FEC_CORRECTED_ERRORS: c_uint = 0x62;
// enum: Number of corrected 10-bit symbol errors, lane 0 (RS-FEC only)
pub const MC_CMD_MAC_FEC_CORRECTED_SYMBOLS_LANE0: c_uint = 0x63;
// enum: Number of corrected 10-bit symbol errors, lane 1 (RS-FEC only)
pub const MC_CMD_MAC_FEC_CORRECTED_SYMBOLS_LANE1: c_uint = 0x64;
// enum: Number of corrected 10-bit symbol errors, lane 2 (RS-FEC only)
pub const MC_CMD_MAC_FEC_CORRECTED_SYMBOLS_LANE2: c_uint = 0x65;
// enum: Number of corrected 10-bit symbol errors, lane 3 (RS-FEC only)
pub const MC_CMD_MAC_FEC_CORRECTED_SYMBOLS_LANE3: c_uint = 0x66;
// enum: This includes the space at offset 103 which is the final
// GENERATION_END in a MAC_STATS_V2 response and otherwise unused.
//
pub const MC_CMD_MAC_NSTATS_V2: c_uint = 0x68;
// Other enum values, see field(s):
// MC_CMD_MAC_STATS_OUT_NO_DMA/STATISTICS
// MC_CMD_MAC_STATS_V3_OUT_DMA msgresponse
pub const MC_CMD_MAC_STATS_V3_OUT_DMA_LEN: c_int = 0;
// MC_CMD_MAC_STATS_V3_OUT_NO_DMA msgresponse

pub const MC_CMD_MAC_STATS_V3_OUT_NO_DMA_STATISTICS_OFST: c_int = 0;
pub const MC_CMD_MAC_STATS_V3_OUT_NO_DMA_STATISTICS_LEN: c_int = 8;
pub const MC_CMD_MAC_STATS_V3_OUT_NO_DMA_STATISTICS_LO_OFST: c_int = 0;
pub const MC_CMD_MAC_STATS_V3_OUT_NO_DMA_STATISTICS_HI_OFST: c_int = 4;

// enum: Start of CTPIO stats buffer space, Medford2 and up
pub const MC_CMD_MAC_CTPIO_DMABUF_START: c_uint = 0x68;
// enum: Number of CTPIO fallbacks because a DMA packet was in progress on the
// target VI
//
pub const MC_CMD_MAC_CTPIO_VI_BUSY_FALLBACK: c_uint = 0x68;
// enum: Number of times a CTPIO send wrote beyond frame end (informational
// only)
//
pub const MC_CMD_MAC_CTPIO_LONG_WRITE_SUCCESS: c_uint = 0x69;
// enum: Number of CTPIO failures because the TX doorbell was written before
// the end of the frame data
//
pub const MC_CMD_MAC_CTPIO_MISSING_DBELL_FAIL: c_uint = 0x6a;
// enum: Number of CTPIO failures because the internal FIFO overflowed
pub const MC_CMD_MAC_CTPIO_OVERFLOW_FAIL: c_uint = 0x6b;
// enum: Number of CTPIO failures because the host did not deliver data fast
// enough to avoid MAC underflow
//
pub const MC_CMD_MAC_CTPIO_UNDERFLOW_FAIL: c_uint = 0x6c;
// enum: Number of CTPIO failures because the host did not deliver all the
// frame data within the timeout
//
pub const MC_CMD_MAC_CTPIO_TIMEOUT_FAIL: c_uint = 0x6d;
// enum: Number of CTPIO failures because the frame data arrived out of order
// or with gaps
//
pub const MC_CMD_MAC_CTPIO_NONCONTIG_WR_FAIL: c_uint = 0x6e;
// enum: Number of CTPIO failures because the host started a new frame before
// completing the previous one
//
pub const MC_CMD_MAC_CTPIO_FRM_CLOBBER_FAIL: c_uint = 0x6f;
// enum: Number of CTPIO failures because a write was not a multiple of 32 bits
// or not 32-bit aligned
//
pub const MC_CMD_MAC_CTPIO_INVALID_WR_FAIL: c_uint = 0x70;
// enum: Number of CTPIO fallbacks because another VI on the same port was
// sending a CTPIO frame
//
pub const MC_CMD_MAC_CTPIO_VI_CLOBBER_FALLBACK: c_uint = 0x71;
// enum: Number of CTPIO fallbacks because target VI did not have CTPIO enabled
//
pub const MC_CMD_MAC_CTPIO_UNQUALIFIED_FALLBACK: c_uint = 0x72;
// enum: Number of CTPIO fallbacks because length in header was less than 29
// bytes
//
pub const MC_CMD_MAC_CTPIO_RUNT_FALLBACK: c_uint = 0x73;
// enum: Total number of successful CTPIO sends on this port
pub const MC_CMD_MAC_CTPIO_SUCCESS: c_uint = 0x74;
// enum: Total number of CTPIO fallbacks on this port
pub const MC_CMD_MAC_CTPIO_FALLBACK: c_uint = 0x75;
// enum: Total number of CTPIO poisoned frames on this port, whether erased or
// not
//
pub const MC_CMD_MAC_CTPIO_POISON: c_uint = 0x76;
// enum: Total number of CTPIO erased frames on this port
pub const MC_CMD_MAC_CTPIO_ERASE: c_uint = 0x77;
// enum: This includes the space at offset 120 which is the final
// GENERATION_END in a MAC_STATS_V3 response and otherwise unused.
//
pub const MC_CMD_MAC_NSTATS_V3: c_uint = 0x79;
// Other enum values, see field(s):
// MC_CMD_MAC_STATS_V2_OUT_NO_DMA/STATISTICS
// MC_CMD_MAC_STATS_V4_OUT_DMA msgresponse
pub const MC_CMD_MAC_STATS_V4_OUT_DMA_LEN: c_int = 0;
// MC_CMD_MAC_STATS_V4_OUT_NO_DMA msgresponse

pub const MC_CMD_MAC_STATS_V4_OUT_NO_DMA_STATISTICS_OFST: c_int = 0;
pub const MC_CMD_MAC_STATS_V4_OUT_NO_DMA_STATISTICS_LEN: c_int = 8;
pub const MC_CMD_MAC_STATS_V4_OUT_NO_DMA_STATISTICS_LO_OFST: c_int = 0;
pub const MC_CMD_MAC_STATS_V4_OUT_NO_DMA_STATISTICS_HI_OFST: c_int = 4;

// enum: Start of V4 stats buffer space
pub const MC_CMD_MAC_V4_DMABUF_START: c_uint = 0x79;
// enum: RXDP counter: Number of packets truncated because scattering was
// disabled.
//
pub const MC_CMD_MAC_RXDP_SCATTER_DISABLED_TRUNC: c_uint = 0x79;
// enum: RXDP counter: Number of times the RXDP head of line blocked waiting
// for descriptors. Will be zero unless RXDP_HLB_IDLE capability is set.
//
pub const MC_CMD_MAC_RXDP_HLB_IDLE: c_uint = 0x7a;
// enum: RXDP counter: Number of times the RXDP timed out while head of line
// blocking. Will be zero unless RXDP_HLB_IDLE capability is set.
//
pub const MC_CMD_MAC_RXDP_HLB_TIMEOUT: c_uint = 0x7b;
// enum: This includes the space at offset 124 which is the final
// GENERATION_END in a MAC_STATS_V4 response and otherwise unused.
//
pub const MC_CMD_MAC_NSTATS_V4: c_uint = 0x7d;
// Other enum values, see field(s):
// MC_CMD_MAC_STATS_V3_OUT_NO_DMA/STATISTICS
//
// MC_CMD_SRIOV
// to be documented
//
pub const MC_CMD_SRIOV: c_uint = 0x30;
// MC_CMD_SRIOV_IN msgrequest
pub const MC_CMD_SRIOV_IN_LEN: c_int = 12;
pub const MC_CMD_SRIOV_IN_ENABLE_OFST: c_int = 0;
pub const MC_CMD_SRIOV_IN_ENABLE_LEN: c_int = 4;
pub const MC_CMD_SRIOV_IN_VI_BASE_OFST: c_int = 4;
pub const MC_CMD_SRIOV_IN_VI_BASE_LEN: c_int = 4;
pub const MC_CMD_SRIOV_IN_VF_COUNT_OFST: c_int = 8;
pub const MC_CMD_SRIOV_IN_VF_COUNT_LEN: c_int = 4;
// MC_CMD_SRIOV_OUT msgresponse
pub const MC_CMD_SRIOV_OUT_LEN: c_int = 8;
pub const MC_CMD_SRIOV_OUT_VI_SCALE_OFST: c_int = 0;
pub const MC_CMD_SRIOV_OUT_VI_SCALE_LEN: c_int = 4;
pub const MC_CMD_SRIOV_OUT_VF_TOTAL_OFST: c_int = 4;
pub const MC_CMD_SRIOV_OUT_VF_TOTAL_LEN: c_int = 4;
// MC_CMD_MEMCPY_RECORD_TYPEDEF structuredef
pub const MC_CMD_MEMCPY_RECORD_TYPEDEF_LEN: c_int = 32;
// this is only used for the first record
pub const MC_CMD_MEMCPY_RECORD_TYPEDEF_NUM_RECORDS_OFST: c_int = 0;
pub const MC_CMD_MEMCPY_RECORD_TYPEDEF_NUM_RECORDS_LEN: c_int = 4;
pub const MC_CMD_MEMCPY_RECORD_TYPEDEF_NUM_RECORDS_LBN: c_int = 0;
pub const MC_CMD_MEMCPY_RECORD_TYPEDEF_NUM_RECORDS_WIDTH: c_int = 32;
pub const MC_CMD_MEMCPY_RECORD_TYPEDEF_TO_RID_OFST: c_int = 4;
pub const MC_CMD_MEMCPY_RECORD_TYPEDEF_TO_RID_LEN: c_int = 4;
pub const MC_CMD_MEMCPY_RECORD_TYPEDEF_TO_RID_LBN: c_int = 32;
pub const MC_CMD_MEMCPY_RECORD_TYPEDEF_TO_RID_WIDTH: c_int = 32;
pub const MC_CMD_MEMCPY_RECORD_TYPEDEF_TO_ADDR_OFST: c_int = 8;
pub const MC_CMD_MEMCPY_RECORD_TYPEDEF_TO_ADDR_LEN: c_int = 8;
pub const MC_CMD_MEMCPY_RECORD_TYPEDEF_TO_ADDR_LO_OFST: c_int = 8;
pub const MC_CMD_MEMCPY_RECORD_TYPEDEF_TO_ADDR_HI_OFST: c_int = 12;
pub const MC_CMD_MEMCPY_RECORD_TYPEDEF_TO_ADDR_LBN: c_int = 64;
pub const MC_CMD_MEMCPY_RECORD_TYPEDEF_TO_ADDR_WIDTH: c_int = 64;
pub const MC_CMD_MEMCPY_RECORD_TYPEDEF_FROM_RID_OFST: c_int = 16;
pub const MC_CMD_MEMCPY_RECORD_TYPEDEF_FROM_RID_LEN: c_int = 4;
pub const MC_CMD_MEMCPY_RECORD_TYPEDEF_RID_INLINE: c_uint = 0x100 /* enum */;
pub const MC_CMD_MEMCPY_RECORD_TYPEDEF_FROM_RID_LBN: c_int = 128;
pub const MC_CMD_MEMCPY_RECORD_TYPEDEF_FROM_RID_WIDTH: c_int = 32;
pub const MC_CMD_MEMCPY_RECORD_TYPEDEF_FROM_ADDR_OFST: c_int = 20;
pub const MC_CMD_MEMCPY_RECORD_TYPEDEF_FROM_ADDR_LEN: c_int = 8;
pub const MC_CMD_MEMCPY_RECORD_TYPEDEF_FROM_ADDR_LO_OFST: c_int = 20;
pub const MC_CMD_MEMCPY_RECORD_TYPEDEF_FROM_ADDR_HI_OFST: c_int = 24;
pub const MC_CMD_MEMCPY_RECORD_TYPEDEF_FROM_ADDR_LBN: c_int = 160;
pub const MC_CMD_MEMCPY_RECORD_TYPEDEF_FROM_ADDR_WIDTH: c_int = 64;
pub const MC_CMD_MEMCPY_RECORD_TYPEDEF_LENGTH_OFST: c_int = 28;
pub const MC_CMD_MEMCPY_RECORD_TYPEDEF_LENGTH_LEN: c_int = 4;
pub const MC_CMD_MEMCPY_RECORD_TYPEDEF_LENGTH_LBN: c_int = 224;
pub const MC_CMD_MEMCPY_RECORD_TYPEDEF_LENGTH_WIDTH: c_int = 32;
//
// MC_CMD_MEMCPY
// DMA write data into (Rid,Addr), either by dma reading (Rid,Addr), or by data
// embedded directly in the command.
//
// A common pattern is for a client to use generation counts to signal a dma
// update of a datastructure. To facilitate this, this MCDI operation can
// contain multiple requests which are executed in strict order. Requests take
// the form of duplicating the entire MCDI request continuously (including the
// requests record, which is ignored in all but the first structure)
//
// The source data can either come from a DMA from the host, or it can be
// embedded within the request directly, thereby eliminating a DMA read. To
// indicate this, the client sets FROM_RID=%RID_INLINE, ADDR_HI=0, and
// ADDR_LO=offset, and inserts the data at %offset from the start of the
// payload. It's the callers responsibility to ensure that the embedded data
// doesn't overlap the records.
//
// Returns: 0, EINVAL (invalid RID)
//
pub const MC_CMD_MEMCPY: c_uint = 0x31;
// MC_CMD_MEMCPY_IN msgrequest
pub const MC_CMD_MEMCPY_IN_LENMIN: c_int = 32;
pub const MC_CMD_MEMCPY_IN_LENMAX: c_int = 224;
pub const MC_CMD_MEMCPY_IN_LENMAX_MCDI2: c_int = 992;

// see MC_CMD_MEMCPY_RECORD_TYPEDEF
pub const MC_CMD_MEMCPY_IN_RECORD_OFST: c_int = 0;
pub const MC_CMD_MEMCPY_IN_RECORD_LEN: c_int = 32;
pub const MC_CMD_MEMCPY_IN_RECORD_MINNUM: c_int = 1;
pub const MC_CMD_MEMCPY_IN_RECORD_MAXNUM: c_int = 7;
pub const MC_CMD_MEMCPY_IN_RECORD_MAXNUM_MCDI2: c_int = 31;
// MC_CMD_MEMCPY_OUT msgresponse
pub const MC_CMD_MEMCPY_OUT_LEN: c_int = 0;
//
// MC_CMD_WOL_FILTER_SET
// Set a WoL filter.
//
pub const MC_CMD_WOL_FILTER_SET: c_uint = 0x32;

// MC_CMD_WOL_FILTER_SET_IN msgrequest
pub const MC_CMD_WOL_FILTER_SET_IN_LEN: c_int = 192;
pub const MC_CMD_WOL_FILTER_SET_IN_FILTER_MODE_OFST: c_int = 0;
pub const MC_CMD_WOL_FILTER_SET_IN_FILTER_MODE_LEN: c_int = 4;
pub const MC_CMD_FILTER_MODE_SIMPLE: c_uint = 0x0 /* enum */;
pub const MC_CMD_FILTER_MODE_STRUCTURED: c_uint = 0xffffffff /* enum */;
// A type value of 1 is unused.
pub const MC_CMD_WOL_FILTER_SET_IN_WOL_TYPE_OFST: c_int = 4;
pub const MC_CMD_WOL_FILTER_SET_IN_WOL_TYPE_LEN: c_int = 4;
// enum: Magic
pub const MC_CMD_WOL_TYPE_MAGIC: c_uint = 0x0;
// enum: MS Windows Magic
pub const MC_CMD_WOL_TYPE_WIN_MAGIC: c_uint = 0x2;
// enum: IPv4 Syn
pub const MC_CMD_WOL_TYPE_IPV4_SYN: c_uint = 0x3;
// enum: IPv6 Syn
pub const MC_CMD_WOL_TYPE_IPV6_SYN: c_uint = 0x4;
// enum: Bitmap
pub const MC_CMD_WOL_TYPE_BITMAP: c_uint = 0x5;
// enum: Link
pub const MC_CMD_WOL_TYPE_LINK: c_uint = 0x6;
// enum: (Above this for future use)
pub const MC_CMD_WOL_TYPE_MAX: c_uint = 0x7;
pub const MC_CMD_WOL_FILTER_SET_IN_DATA_OFST: c_int = 8;
pub const MC_CMD_WOL_FILTER_SET_IN_DATA_LEN: c_int = 4;
pub const MC_CMD_WOL_FILTER_SET_IN_DATA_NUM: c_int = 46;
// MC_CMD_WOL_FILTER_SET_IN_MAGIC msgrequest
pub const MC_CMD_WOL_FILTER_SET_IN_MAGIC_LEN: c_int = 16;
// MC_CMD_WOL_FILTER_SET_IN_FILTER_MODE_OFST 0
// MC_CMD_WOL_FILTER_SET_IN_FILTER_MODE_LEN 4
// MC_CMD_WOL_FILTER_SET_IN_WOL_TYPE_OFST 4
// MC_CMD_WOL_FILTER_SET_IN_WOL_TYPE_LEN 4
pub const MC_CMD_WOL_FILTER_SET_IN_MAGIC_MAC_OFST: c_int = 8;
pub const MC_CMD_WOL_FILTER_SET_IN_MAGIC_MAC_LEN: c_int = 8;
pub const MC_CMD_WOL_FILTER_SET_IN_MAGIC_MAC_LO_OFST: c_int = 8;
pub const MC_CMD_WOL_FILTER_SET_IN_MAGIC_MAC_HI_OFST: c_int = 12;
// MC_CMD_WOL_FILTER_SET_IN_IPV4_SYN msgrequest
pub const MC_CMD_WOL_FILTER_SET_IN_IPV4_SYN_LEN: c_int = 20;
// MC_CMD_WOL_FILTER_SET_IN_FILTER_MODE_OFST 0
// MC_CMD_WOL_FILTER_SET_IN_FILTER_MODE_LEN 4
// MC_CMD_WOL_FILTER_SET_IN_WOL_TYPE_OFST 4
// MC_CMD_WOL_FILTER_SET_IN_WOL_TYPE_LEN 4
pub const MC_CMD_WOL_FILTER_SET_IN_IPV4_SYN_SRC_IP_OFST: c_int = 8;
pub const MC_CMD_WOL_FILTER_SET_IN_IPV4_SYN_SRC_IP_LEN: c_int = 4;
pub const MC_CMD_WOL_FILTER_SET_IN_IPV4_SYN_DST_IP_OFST: c_int = 12;
pub const MC_CMD_WOL_FILTER_SET_IN_IPV4_SYN_DST_IP_LEN: c_int = 4;
pub const MC_CMD_WOL_FILTER_SET_IN_IPV4_SYN_SRC_PORT_OFST: c_int = 16;
pub const MC_CMD_WOL_FILTER_SET_IN_IPV4_SYN_SRC_PORT_LEN: c_int = 2;
pub const MC_CMD_WOL_FILTER_SET_IN_IPV4_SYN_DST_PORT_OFST: c_int = 18;
pub const MC_CMD_WOL_FILTER_SET_IN_IPV4_SYN_DST_PORT_LEN: c_int = 2;
// MC_CMD_WOL_FILTER_SET_IN_IPV6_SYN msgrequest
pub const MC_CMD_WOL_FILTER_SET_IN_IPV6_SYN_LEN: c_int = 44;
// MC_CMD_WOL_FILTER_SET_IN_FILTER_MODE_OFST 0
// MC_CMD_WOL_FILTER_SET_IN_FILTER_MODE_LEN 4
// MC_CMD_WOL_FILTER_SET_IN_WOL_TYPE_OFST 4
// MC_CMD_WOL_FILTER_SET_IN_WOL_TYPE_LEN 4
pub const MC_CMD_WOL_FILTER_SET_IN_IPV6_SYN_SRC_IP_OFST: c_int = 8;
pub const MC_CMD_WOL_FILTER_SET_IN_IPV6_SYN_SRC_IP_LEN: c_int = 16;
pub const MC_CMD_WOL_FILTER_SET_IN_IPV6_SYN_DST_IP_OFST: c_int = 24;
pub const MC_CMD_WOL_FILTER_SET_IN_IPV6_SYN_DST_IP_LEN: c_int = 16;
pub const MC_CMD_WOL_FILTER_SET_IN_IPV6_SYN_SRC_PORT_OFST: c_int = 40;
pub const MC_CMD_WOL_FILTER_SET_IN_IPV6_SYN_SRC_PORT_LEN: c_int = 2;
pub const MC_CMD_WOL_FILTER_SET_IN_IPV6_SYN_DST_PORT_OFST: c_int = 42;
pub const MC_CMD_WOL_FILTER_SET_IN_IPV6_SYN_DST_PORT_LEN: c_int = 2;
// MC_CMD_WOL_FILTER_SET_IN_BITMAP msgrequest
pub const MC_CMD_WOL_FILTER_SET_IN_BITMAP_LEN: c_int = 187;
// MC_CMD_WOL_FILTER_SET_IN_FILTER_MODE_OFST 0
// MC_CMD_WOL_FILTER_SET_IN_FILTER_MODE_LEN 4
// MC_CMD_WOL_FILTER_SET_IN_WOL_TYPE_OFST 4
// MC_CMD_WOL_FILTER_SET_IN_WOL_TYPE_LEN 4
pub const MC_CMD_WOL_FILTER_SET_IN_BITMAP_MASK_OFST: c_int = 8;
pub const MC_CMD_WOL_FILTER_SET_IN_BITMAP_MASK_LEN: c_int = 48;
pub const MC_CMD_WOL_FILTER_SET_IN_BITMAP_BITMAP_OFST: c_int = 56;
pub const MC_CMD_WOL_FILTER_SET_IN_BITMAP_BITMAP_LEN: c_int = 128;
pub const MC_CMD_WOL_FILTER_SET_IN_BITMAP_LEN_OFST: c_int = 184;
pub const MC_CMD_WOL_FILTER_SET_IN_BITMAP_LEN_LEN: c_int = 1;
pub const MC_CMD_WOL_FILTER_SET_IN_BITMAP_LAYER3_OFST: c_int = 185;
pub const MC_CMD_WOL_FILTER_SET_IN_BITMAP_LAYER3_LEN: c_int = 1;
pub const MC_CMD_WOL_FILTER_SET_IN_BITMAP_LAYER4_OFST: c_int = 186;
pub const MC_CMD_WOL_FILTER_SET_IN_BITMAP_LAYER4_LEN: c_int = 1;
// MC_CMD_WOL_FILTER_SET_IN_LINK msgrequest
pub const MC_CMD_WOL_FILTER_SET_IN_LINK_LEN: c_int = 12;
// MC_CMD_WOL_FILTER_SET_IN_FILTER_MODE_OFST 0
// MC_CMD_WOL_FILTER_SET_IN_FILTER_MODE_LEN 4
// MC_CMD_WOL_FILTER_SET_IN_WOL_TYPE_OFST 4
// MC_CMD_WOL_FILTER_SET_IN_WOL_TYPE_LEN 4
pub const MC_CMD_WOL_FILTER_SET_IN_LINK_MASK_OFST: c_int = 8;
pub const MC_CMD_WOL_FILTER_SET_IN_LINK_MASK_LEN: c_int = 4;
pub const MC_CMD_WOL_FILTER_SET_IN_LINK_UP_OFST: c_int = 8;
pub const MC_CMD_WOL_FILTER_SET_IN_LINK_UP_LBN: c_int = 0;
pub const MC_CMD_WOL_FILTER_SET_IN_LINK_UP_WIDTH: c_int = 1;
pub const MC_CMD_WOL_FILTER_SET_IN_LINK_DOWN_OFST: c_int = 8;
pub const MC_CMD_WOL_FILTER_SET_IN_LINK_DOWN_LBN: c_int = 1;
pub const MC_CMD_WOL_FILTER_SET_IN_LINK_DOWN_WIDTH: c_int = 1;
// MC_CMD_WOL_FILTER_SET_OUT msgresponse
pub const MC_CMD_WOL_FILTER_SET_OUT_LEN: c_int = 4;
pub const MC_CMD_WOL_FILTER_SET_OUT_FILTER_ID_OFST: c_int = 0;
pub const MC_CMD_WOL_FILTER_SET_OUT_FILTER_ID_LEN: c_int = 4;
//
// MC_CMD_WOL_FILTER_REMOVE
// Remove a WoL filter. Locks required: None. Returns: 0, EINVAL, ENOSYS
//
pub const MC_CMD_WOL_FILTER_REMOVE: c_uint = 0x33;

// MC_CMD_WOL_FILTER_REMOVE_IN msgrequest
pub const MC_CMD_WOL_FILTER_REMOVE_IN_LEN: c_int = 4;
pub const MC_CMD_WOL_FILTER_REMOVE_IN_FILTER_ID_OFST: c_int = 0;
pub const MC_CMD_WOL_FILTER_REMOVE_IN_FILTER_ID_LEN: c_int = 4;
// MC_CMD_WOL_FILTER_REMOVE_OUT msgresponse
pub const MC_CMD_WOL_FILTER_REMOVE_OUT_LEN: c_int = 0;
//
// MC_CMD_WOL_FILTER_RESET
// Reset (i.e. remove all) WoL filters. Locks required: None. Returns: 0,
// ENOSYS
//
pub const MC_CMD_WOL_FILTER_RESET: c_uint = 0x34;

// MC_CMD_WOL_FILTER_RESET_IN msgrequest
pub const MC_CMD_WOL_FILTER_RESET_IN_LEN: c_int = 4;
pub const MC_CMD_WOL_FILTER_RESET_IN_MASK_OFST: c_int = 0;
pub const MC_CMD_WOL_FILTER_RESET_IN_MASK_LEN: c_int = 4;
pub const MC_CMD_WOL_FILTER_RESET_IN_WAKE_FILTERS: c_uint = 0x1 /* enum */;
pub const MC_CMD_WOL_FILTER_RESET_IN_LIGHTSOUT_OFFLOADS: c_uint = 0x2 /* enum */;
// MC_CMD_WOL_FILTER_RESET_OUT msgresponse
pub const MC_CMD_WOL_FILTER_RESET_OUT_LEN: c_int = 0;
//
// MC_CMD_SET_MCAST_HASH
// Set the MCAST hash value without otherwise reconfiguring the MAC
//
pub const MC_CMD_SET_MCAST_HASH: c_uint = 0x35;
// MC_CMD_SET_MCAST_HASH_IN msgrequest
pub const MC_CMD_SET_MCAST_HASH_IN_LEN: c_int = 32;
pub const MC_CMD_SET_MCAST_HASH_IN_HASH0_OFST: c_int = 0;
pub const MC_CMD_SET_MCAST_HASH_IN_HASH0_LEN: c_int = 16;
pub const MC_CMD_SET_MCAST_HASH_IN_HASH1_OFST: c_int = 16;
pub const MC_CMD_SET_MCAST_HASH_IN_HASH1_LEN: c_int = 16;
// MC_CMD_SET_MCAST_HASH_OUT msgresponse
pub const MC_CMD_SET_MCAST_HASH_OUT_LEN: c_int = 0;
//
// MC_CMD_NVRAM_TYPES
// Return bitfield indicating available types of virtual NVRAM partitions.
// Locks required: none. Returns: 0
//
pub const MC_CMD_NVRAM_TYPES: c_uint = 0x36;

// MC_CMD_NVRAM_TYPES_IN msgrequest
pub const MC_CMD_NVRAM_TYPES_IN_LEN: c_int = 0;
// MC_CMD_NVRAM_TYPES_OUT msgresponse
pub const MC_CMD_NVRAM_TYPES_OUT_LEN: c_int = 4;
// Bit mask of supported types.
pub const MC_CMD_NVRAM_TYPES_OUT_TYPES_OFST: c_int = 0;
pub const MC_CMD_NVRAM_TYPES_OUT_TYPES_LEN: c_int = 4;
// enum: Disabled callisto.
pub const MC_CMD_NVRAM_TYPE_DISABLED_CALLISTO: c_uint = 0x0;
// enum: MC firmware.
pub const MC_CMD_NVRAM_TYPE_MC_FW: c_uint = 0x1;
// enum: MC backup firmware.
pub const MC_CMD_NVRAM_TYPE_MC_FW_BACKUP: c_uint = 0x2;
// enum: Static configuration Port0.
pub const MC_CMD_NVRAM_TYPE_STATIC_CFG_PORT0: c_uint = 0x3;
// enum: Static configuration Port1.
pub const MC_CMD_NVRAM_TYPE_STATIC_CFG_PORT1: c_uint = 0x4;
// enum: Dynamic configuration Port0.
pub const MC_CMD_NVRAM_TYPE_DYNAMIC_CFG_PORT0: c_uint = 0x5;
// enum: Dynamic configuration Port1.
pub const MC_CMD_NVRAM_TYPE_DYNAMIC_CFG_PORT1: c_uint = 0x6;
// enum: Expansion Rom.
pub const MC_CMD_NVRAM_TYPE_EXP_ROM: c_uint = 0x7;
// enum: Expansion Rom Configuration Port0.
pub const MC_CMD_NVRAM_TYPE_EXP_ROM_CFG_PORT0: c_uint = 0x8;
// enum: Expansion Rom Configuration Port1.
pub const MC_CMD_NVRAM_TYPE_EXP_ROM_CFG_PORT1: c_uint = 0x9;
// enum: Phy Configuration Port0.
pub const MC_CMD_NVRAM_TYPE_PHY_PORT0: c_uint = 0xa;
// enum: Phy Configuration Port1.
pub const MC_CMD_NVRAM_TYPE_PHY_PORT1: c_uint = 0xb;
// enum: Log.
pub const MC_CMD_NVRAM_TYPE_LOG: c_uint = 0xc;
// enum: FPGA image.
pub const MC_CMD_NVRAM_TYPE_FPGA: c_uint = 0xd;
// enum: FPGA backup image
pub const MC_CMD_NVRAM_TYPE_FPGA_BACKUP: c_uint = 0xe;
// enum: FC firmware.
pub const MC_CMD_NVRAM_TYPE_FC_FW: c_uint = 0xf;
// enum: FC backup firmware.
pub const MC_CMD_NVRAM_TYPE_FC_FW_BACKUP: c_uint = 0x10;
// enum: CPLD image.
pub const MC_CMD_NVRAM_TYPE_CPLD: c_uint = 0x11;
// enum: Licensing information.
pub const MC_CMD_NVRAM_TYPE_LICENSE: c_uint = 0x12;
// enum: FC Log.
pub const MC_CMD_NVRAM_TYPE_FC_LOG: c_uint = 0x13;
// enum: Additional flash on FPGA.
pub const MC_CMD_NVRAM_TYPE_FC_EXTRA: c_uint = 0x14;
//
// MC_CMD_NVRAM_INFO
// Read info about a virtual NVRAM partition. Locks required: none. Returns: 0,
// EINVAL (bad type).
//
pub const MC_CMD_NVRAM_INFO: c_uint = 0x37;

// MC_CMD_NVRAM_INFO_IN msgrequest
pub const MC_CMD_NVRAM_INFO_IN_LEN: c_int = 4;
pub const MC_CMD_NVRAM_INFO_IN_TYPE_OFST: c_int = 0;
pub const MC_CMD_NVRAM_INFO_IN_TYPE_LEN: c_int = 4;
// Enum values, see field(s):
// MC_CMD_NVRAM_TYPES/MC_CMD_NVRAM_TYPES_OUT/TYPES
// MC_CMD_NVRAM_INFO_OUT msgresponse
pub const MC_CMD_NVRAM_INFO_OUT_LEN: c_int = 24;
pub const MC_CMD_NVRAM_INFO_OUT_TYPE_OFST: c_int = 0;
pub const MC_CMD_NVRAM_INFO_OUT_TYPE_LEN: c_int = 4;
// Enum values, see field(s):
// MC_CMD_NVRAM_TYPES/MC_CMD_NVRAM_TYPES_OUT/TYPES
pub const MC_CMD_NVRAM_INFO_OUT_SIZE_OFST: c_int = 4;
pub const MC_CMD_NVRAM_INFO_OUT_SIZE_LEN: c_int = 4;
pub const MC_CMD_NVRAM_INFO_OUT_ERASESIZE_OFST: c_int = 8;
pub const MC_CMD_NVRAM_INFO_OUT_ERASESIZE_LEN: c_int = 4;
pub const MC_CMD_NVRAM_INFO_OUT_FLAGS_OFST: c_int = 12;
pub const MC_CMD_NVRAM_INFO_OUT_FLAGS_LEN: c_int = 4;
pub const MC_CMD_NVRAM_INFO_OUT_PROTECTED_OFST: c_int = 12;
pub const MC_CMD_NVRAM_INFO_OUT_PROTECTED_LBN: c_int = 0;
pub const MC_CMD_NVRAM_INFO_OUT_PROTECTED_WIDTH: c_int = 1;
pub const MC_CMD_NVRAM_INFO_OUT_TLV_OFST: c_int = 12;
pub const MC_CMD_NVRAM_INFO_OUT_TLV_LBN: c_int = 1;
pub const MC_CMD_NVRAM_INFO_OUT_TLV_WIDTH: c_int = 1;
pub const MC_CMD_NVRAM_INFO_OUT_READ_ONLY_IF_TSA_BOUND_OFST: c_int = 12;
pub const MC_CMD_NVRAM_INFO_OUT_READ_ONLY_IF_TSA_BOUND_LBN: c_int = 2;
pub const MC_CMD_NVRAM_INFO_OUT_READ_ONLY_IF_TSA_BOUND_WIDTH: c_int = 1;
pub const MC_CMD_NVRAM_INFO_OUT_CRC_OFST: c_int = 12;
pub const MC_CMD_NVRAM_INFO_OUT_CRC_LBN: c_int = 3;
pub const MC_CMD_NVRAM_INFO_OUT_CRC_WIDTH: c_int = 1;
pub const MC_CMD_NVRAM_INFO_OUT_READ_ONLY_OFST: c_int = 12;
pub const MC_CMD_NVRAM_INFO_OUT_READ_ONLY_LBN: c_int = 5;
pub const MC_CMD_NVRAM_INFO_OUT_READ_ONLY_WIDTH: c_int = 1;
pub const MC_CMD_NVRAM_INFO_OUT_CMAC_OFST: c_int = 12;
pub const MC_CMD_NVRAM_INFO_OUT_CMAC_LBN: c_int = 6;
pub const MC_CMD_NVRAM_INFO_OUT_CMAC_WIDTH: c_int = 1;
pub const MC_CMD_NVRAM_INFO_OUT_A_B_OFST: c_int = 12;
pub const MC_CMD_NVRAM_INFO_OUT_A_B_LBN: c_int = 7;
pub const MC_CMD_NVRAM_INFO_OUT_A_B_WIDTH: c_int = 1;
pub const MC_CMD_NVRAM_INFO_OUT_PHYSDEV_OFST: c_int = 16;
pub const MC_CMD_NVRAM_INFO_OUT_PHYSDEV_LEN: c_int = 4;
pub const MC_CMD_NVRAM_INFO_OUT_PHYSADDR_OFST: c_int = 20;
pub const MC_CMD_NVRAM_INFO_OUT_PHYSADDR_LEN: c_int = 4;
// MC_CMD_NVRAM_INFO_V2_OUT msgresponse
pub const MC_CMD_NVRAM_INFO_V2_OUT_LEN: c_int = 28;
pub const MC_CMD_NVRAM_INFO_V2_OUT_TYPE_OFST: c_int = 0;
pub const MC_CMD_NVRAM_INFO_V2_OUT_TYPE_LEN: c_int = 4;
// Enum values, see field(s):
// MC_CMD_NVRAM_TYPES/MC_CMD_NVRAM_TYPES_OUT/TYPES
pub const MC_CMD_NVRAM_INFO_V2_OUT_SIZE_OFST: c_int = 4;
pub const MC_CMD_NVRAM_INFO_V2_OUT_SIZE_LEN: c_int = 4;
pub const MC_CMD_NVRAM_INFO_V2_OUT_ERASESIZE_OFST: c_int = 8;
pub const MC_CMD_NVRAM_INFO_V2_OUT_ERASESIZE_LEN: c_int = 4;
pub const MC_CMD_NVRAM_INFO_V2_OUT_FLAGS_OFST: c_int = 12;
pub const MC_CMD_NVRAM_INFO_V2_OUT_FLAGS_LEN: c_int = 4;
pub const MC_CMD_NVRAM_INFO_V2_OUT_PROTECTED_OFST: c_int = 12;
pub const MC_CMD_NVRAM_INFO_V2_OUT_PROTECTED_LBN: c_int = 0;
pub const MC_CMD_NVRAM_INFO_V2_OUT_PROTECTED_WIDTH: c_int = 1;
pub const MC_CMD_NVRAM_INFO_V2_OUT_TLV_OFST: c_int = 12;
pub const MC_CMD_NVRAM_INFO_V2_OUT_TLV_LBN: c_int = 1;
pub const MC_CMD_NVRAM_INFO_V2_OUT_TLV_WIDTH: c_int = 1;
pub const MC_CMD_NVRAM_INFO_V2_OUT_READ_ONLY_IF_TSA_BOUND_OFST: c_int = 12;
pub const MC_CMD_NVRAM_INFO_V2_OUT_READ_ONLY_IF_TSA_BOUND_LBN: c_int = 2;
pub const MC_CMD_NVRAM_INFO_V2_OUT_READ_ONLY_IF_TSA_BOUND_WIDTH: c_int = 1;
pub const MC_CMD_NVRAM_INFO_V2_OUT_READ_ONLY_OFST: c_int = 12;
pub const MC_CMD_NVRAM_INFO_V2_OUT_READ_ONLY_LBN: c_int = 5;
pub const MC_CMD_NVRAM_INFO_V2_OUT_READ_ONLY_WIDTH: c_int = 1;
pub const MC_CMD_NVRAM_INFO_V2_OUT_A_B_OFST: c_int = 12;
pub const MC_CMD_NVRAM_INFO_V2_OUT_A_B_LBN: c_int = 7;
pub const MC_CMD_NVRAM_INFO_V2_OUT_A_B_WIDTH: c_int = 1;
pub const MC_CMD_NVRAM_INFO_V2_OUT_PHYSDEV_OFST: c_int = 16;
pub const MC_CMD_NVRAM_INFO_V2_OUT_PHYSDEV_LEN: c_int = 4;
pub const MC_CMD_NVRAM_INFO_V2_OUT_PHYSADDR_OFST: c_int = 20;
pub const MC_CMD_NVRAM_INFO_V2_OUT_PHYSADDR_LEN: c_int = 4;
// Writes must be multiples of this size. Added to support the MUM on Sorrento.
//
pub const MC_CMD_NVRAM_INFO_V2_OUT_WRITESIZE_OFST: c_int = 24;
pub const MC_CMD_NVRAM_INFO_V2_OUT_WRITESIZE_LEN: c_int = 4;
//
// MC_CMD_NVRAM_UPDATE_START
// Start a group of update operations on a virtual NVRAM partition. Locks
// required: PHY_LOCK if type==*PHY*. Returns: 0, EINVAL (bad type), EACCES (if
// PHY_LOCK required and not held). In an adapter bound to a TSA controller,
// MC_CMD_NVRAM_UPDATE_START can only be used on a subset of partition types
// i.e. static config, dynamic config and expansion ROM config. Attempting to
// perform this operation on a restricted partition will return the error
// EPERM.
//
pub const MC_CMD_NVRAM_UPDATE_START: c_uint = 0x38;

// MC_CMD_NVRAM_UPDATE_START_IN msgrequest: Legacy NVRAM_UPDATE_START request.
// Use NVRAM_UPDATE_START_V2_IN in new code
//
pub const MC_CMD_NVRAM_UPDATE_START_IN_LEN: c_int = 4;
pub const MC_CMD_NVRAM_UPDATE_START_IN_TYPE_OFST: c_int = 0;
pub const MC_CMD_NVRAM_UPDATE_START_IN_TYPE_LEN: c_int = 4;
// Enum values, see field(s):
// MC_CMD_NVRAM_TYPES/MC_CMD_NVRAM_TYPES_OUT/TYPES
// MC_CMD_NVRAM_UPDATE_START_V2_IN msgrequest: Extended NVRAM_UPDATE_START
// request with additional flags indicating version of command in use. See
// MC_CMD_NVRAM_UPDATE_FINISH_V2_OUT for details of extended functionality. Use
// paired up with NVRAM_UPDATE_FINISH_V2_IN.
//
pub const MC_CMD_NVRAM_UPDATE_START_V2_IN_LEN: c_int = 8;
pub const MC_CMD_NVRAM_UPDATE_START_V2_IN_TYPE_OFST: c_int = 0;
pub const MC_CMD_NVRAM_UPDATE_START_V2_IN_TYPE_LEN: c_int = 4;
// Enum values, see field(s):
// MC_CMD_NVRAM_TYPES/MC_CMD_NVRAM_TYPES_OUT/TYPES
pub const MC_CMD_NVRAM_UPDATE_START_V2_IN_FLAGS_OFST: c_int = 4;
pub const MC_CMD_NVRAM_UPDATE_START_V2_IN_FLAGS_LEN: c_int = 4;
pub const MC_CMD_NVRAM_UPDATE_START_V2_IN_FLAG_REPORT_VERIFY_RESULT_OFST: c_int = 4;
pub const MC_CMD_NVRAM_UPDATE_START_V2_IN_FLAG_REPORT_VERIFY_RESULT_LBN: c_int = 0;
pub const MC_CMD_NVRAM_UPDATE_START_V2_IN_FLAG_REPORT_VERIFY_RESULT_WIDTH: c_int = 1;
// MC_CMD_NVRAM_UPDATE_START_OUT msgresponse
pub const MC_CMD_NVRAM_UPDATE_START_OUT_LEN: c_int = 0;
//
// MC_CMD_NVRAM_READ
// Read data from a virtual NVRAM partition. Locks required: PHY_LOCK if
// type==*PHY*. Returns: 0, EINVAL (bad type/offset/length), EACCES (if
// PHY_LOCK required and not held)
//
pub const MC_CMD_NVRAM_READ: c_uint = 0x39;

// MC_CMD_NVRAM_READ_IN msgrequest
pub const MC_CMD_NVRAM_READ_IN_LEN: c_int = 12;
pub const MC_CMD_NVRAM_READ_IN_TYPE_OFST: c_int = 0;
pub const MC_CMD_NVRAM_READ_IN_TYPE_LEN: c_int = 4;
// Enum values, see field(s):
// MC_CMD_NVRAM_TYPES/MC_CMD_NVRAM_TYPES_OUT/TYPES
pub const MC_CMD_NVRAM_READ_IN_OFFSET_OFST: c_int = 4;
pub const MC_CMD_NVRAM_READ_IN_OFFSET_LEN: c_int = 4;
// amount to read in bytes
pub const MC_CMD_NVRAM_READ_IN_LENGTH_OFST: c_int = 8;
pub const MC_CMD_NVRAM_READ_IN_LENGTH_LEN: c_int = 4;
// MC_CMD_NVRAM_READ_IN_V2 msgrequest
pub const MC_CMD_NVRAM_READ_IN_V2_LEN: c_int = 16;
pub const MC_CMD_NVRAM_READ_IN_V2_TYPE_OFST: c_int = 0;
pub const MC_CMD_NVRAM_READ_IN_V2_TYPE_LEN: c_int = 4;
// Enum values, see field(s):
// MC_CMD_NVRAM_TYPES/MC_CMD_NVRAM_TYPES_OUT/TYPES
pub const MC_CMD_NVRAM_READ_IN_V2_OFFSET_OFST: c_int = 4;
pub const MC_CMD_NVRAM_READ_IN_V2_OFFSET_LEN: c_int = 4;
// amount to read in bytes
pub const MC_CMD_NVRAM_READ_IN_V2_LENGTH_OFST: c_int = 8;
pub const MC_CMD_NVRAM_READ_IN_V2_LENGTH_LEN: c_int = 4;
// Optional control info. If a partition is stored with an A/B versioning
// scheme (i.e. in more than one physical partition in NVRAM) the host can set
// this to control which underlying physical partition is used to read data
// from. This allows it to perform a read-modify-write-verify with the write
// lock continuously held by calling NVRAM_UPDATE_START, reading the old
// contents using MODE=TARGET_CURRENT, overwriting the old partition and then
// verifying by reading with MODE=TARGET_BACKUP.
//
pub const MC_CMD_NVRAM_READ_IN_V2_MODE_OFST: c_int = 12;
pub const MC_CMD_NVRAM_READ_IN_V2_MODE_LEN: c_int = 4;
// enum: Same as omitting MODE: caller sees data in current partition unless it
// holds the write lock in which case it sees data in the partition it is
// updating.
//
pub const MC_CMD_NVRAM_READ_IN_V2_DEFAULT: c_uint = 0x0;
// enum: Read from the current partition of an A/B pair, even if holding the
// write lock.
//
pub const MC_CMD_NVRAM_READ_IN_V2_TARGET_CURRENT: c_uint = 0x1;
// enum: Read from the non-current (i.e. to be updated) partition of an A/B
// pair
//
pub const MC_CMD_NVRAM_READ_IN_V2_TARGET_BACKUP: c_uint = 0x2;
// MC_CMD_NVRAM_READ_OUT msgresponse
pub const MC_CMD_NVRAM_READ_OUT_LENMIN: c_int = 1;
pub const MC_CMD_NVRAM_READ_OUT_LENMAX: c_int = 252;
pub const MC_CMD_NVRAM_READ_OUT_LENMAX_MCDI2: c_int = 1020;

pub const MC_CMD_NVRAM_READ_OUT_READ_BUFFER_OFST: c_int = 0;
pub const MC_CMD_NVRAM_READ_OUT_READ_BUFFER_LEN: c_int = 1;
pub const MC_CMD_NVRAM_READ_OUT_READ_BUFFER_MINNUM: c_int = 1;
pub const MC_CMD_NVRAM_READ_OUT_READ_BUFFER_MAXNUM: c_int = 252;
pub const MC_CMD_NVRAM_READ_OUT_READ_BUFFER_MAXNUM_MCDI2: c_int = 1020;
//
// MC_CMD_NVRAM_WRITE
// Write data to a virtual NVRAM partition. Locks required: PHY_LOCK if
// type==*PHY*. Returns: 0, EINVAL (bad type/offset/length), EACCES (if
// PHY_LOCK required and not held)
//
pub const MC_CMD_NVRAM_WRITE: c_uint = 0x3a;

// MC_CMD_NVRAM_WRITE_IN msgrequest
pub const MC_CMD_NVRAM_WRITE_IN_LENMIN: c_int = 13;
pub const MC_CMD_NVRAM_WRITE_IN_LENMAX: c_int = 252;
pub const MC_CMD_NVRAM_WRITE_IN_LENMAX_MCDI2: c_int = 1020;

pub const MC_CMD_NVRAM_WRITE_IN_TYPE_OFST: c_int = 0;
pub const MC_CMD_NVRAM_WRITE_IN_TYPE_LEN: c_int = 4;
// Enum values, see field(s):
// MC_CMD_NVRAM_TYPES/MC_CMD_NVRAM_TYPES_OUT/TYPES
pub const MC_CMD_NVRAM_WRITE_IN_OFFSET_OFST: c_int = 4;
pub const MC_CMD_NVRAM_WRITE_IN_OFFSET_LEN: c_int = 4;
pub const MC_CMD_NVRAM_WRITE_IN_LENGTH_OFST: c_int = 8;
pub const MC_CMD_NVRAM_WRITE_IN_LENGTH_LEN: c_int = 4;
pub const MC_CMD_NVRAM_WRITE_IN_WRITE_BUFFER_OFST: c_int = 12;
pub const MC_CMD_NVRAM_WRITE_IN_WRITE_BUFFER_LEN: c_int = 1;
pub const MC_CMD_NVRAM_WRITE_IN_WRITE_BUFFER_MINNUM: c_int = 1;
pub const MC_CMD_NVRAM_WRITE_IN_WRITE_BUFFER_MAXNUM: c_int = 240;
pub const MC_CMD_NVRAM_WRITE_IN_WRITE_BUFFER_MAXNUM_MCDI2: c_int = 1008;
// MC_CMD_NVRAM_WRITE_OUT msgresponse
pub const MC_CMD_NVRAM_WRITE_OUT_LEN: c_int = 0;
//
// MC_CMD_NVRAM_ERASE
// Erase sector(s) from a virtual NVRAM partition. Locks required: PHY_LOCK if
// type==*PHY*. Returns: 0, EINVAL (bad type/offset/length), EACCES (if
// PHY_LOCK required and not held)
//
pub const MC_CMD_NVRAM_ERASE: c_uint = 0x3b;

// MC_CMD_NVRAM_ERASE_IN msgrequest
pub const MC_CMD_NVRAM_ERASE_IN_LEN: c_int = 12;
pub const MC_CMD_NVRAM_ERASE_IN_TYPE_OFST: c_int = 0;
pub const MC_CMD_NVRAM_ERASE_IN_TYPE_LEN: c_int = 4;
// Enum values, see field(s):
// MC_CMD_NVRAM_TYPES/MC_CMD_NVRAM_TYPES_OUT/TYPES
pub const MC_CMD_NVRAM_ERASE_IN_OFFSET_OFST: c_int = 4;
pub const MC_CMD_NVRAM_ERASE_IN_OFFSET_LEN: c_int = 4;
pub const MC_CMD_NVRAM_ERASE_IN_LENGTH_OFST: c_int = 8;
pub const MC_CMD_NVRAM_ERASE_IN_LENGTH_LEN: c_int = 4;
// MC_CMD_NVRAM_ERASE_OUT msgresponse
pub const MC_CMD_NVRAM_ERASE_OUT_LEN: c_int = 0;
//
// MC_CMD_NVRAM_UPDATE_FINISH
// Finish a group of update operations on a virtual NVRAM partition. Locks
// required: PHY_LOCK if type==*PHY*. Returns: 0, EINVAL (bad type/offset
// length), EACCES (if PHY_LOCK required and not held). In an adapter bound to
// a TSA controller, MC_CMD_NVRAM_UPDATE_FINISH can only be used on a subset of
// partition types i.e. static config, dynamic config and expansion ROM config.
// Attempting to perform this operation on a restricted partition will return
// the error EPERM.
//
pub const MC_CMD_NVRAM_UPDATE_FINISH: c_uint = 0x3c;

// MC_CMD_NVRAM_UPDATE_FINISH_IN msgrequest: Legacy NVRAM_UPDATE_FINISH
// request. Use NVRAM_UPDATE_FINISH_V2_IN in new code
//
pub const MC_CMD_NVRAM_UPDATE_FINISH_IN_LEN: c_int = 8;
pub const MC_CMD_NVRAM_UPDATE_FINISH_IN_TYPE_OFST: c_int = 0;
pub const MC_CMD_NVRAM_UPDATE_FINISH_IN_TYPE_LEN: c_int = 4;
// Enum values, see field(s):
// MC_CMD_NVRAM_TYPES/MC_CMD_NVRAM_TYPES_OUT/TYPES
pub const MC_CMD_NVRAM_UPDATE_FINISH_IN_REBOOT_OFST: c_int = 4;
pub const MC_CMD_NVRAM_UPDATE_FINISH_IN_REBOOT_LEN: c_int = 4;
// MC_CMD_NVRAM_UPDATE_FINISH_V2_IN msgrequest: Extended NVRAM_UPDATE_FINISH
// request with additional flags indicating version of NVRAM_UPDATE commands in
// use. See MC_CMD_NVRAM_UPDATE_FINISH_V2_OUT for details of extended
// functionality. Use paired up with NVRAM_UPDATE_START_V2_IN.
//
pub const MC_CMD_NVRAM_UPDATE_FINISH_V2_IN_LEN: c_int = 12;
pub const MC_CMD_NVRAM_UPDATE_FINISH_V2_IN_TYPE_OFST: c_int = 0;
pub const MC_CMD_NVRAM_UPDATE_FINISH_V2_IN_TYPE_LEN: c_int = 4;
// Enum values, see field(s):
// MC_CMD_NVRAM_TYPES/MC_CMD_NVRAM_TYPES_OUT/TYPES
pub const MC_CMD_NVRAM_UPDATE_FINISH_V2_IN_REBOOT_OFST: c_int = 4;
pub const MC_CMD_NVRAM_UPDATE_FINISH_V2_IN_REBOOT_LEN: c_int = 4;
pub const MC_CMD_NVRAM_UPDATE_FINISH_V2_IN_FLAGS_OFST: c_int = 8;
pub const MC_CMD_NVRAM_UPDATE_FINISH_V2_IN_FLAGS_LEN: c_int = 4;
pub const MC_CMD_NVRAM_UPDATE_FINISH_V2_IN_FLAG_REPORT_VERIFY_RESULT_OFST: c_int = 8;
pub const MC_CMD_NVRAM_UPDATE_FINISH_V2_IN_FLAG_REPORT_VERIFY_RESULT_LBN: c_int = 0;
pub const MC_CMD_NVRAM_UPDATE_FINISH_V2_IN_FLAG_REPORT_VERIFY_RESULT_WIDTH: c_int = 1;
pub const MC_CMD_NVRAM_UPDATE_FINISH_V2_IN_FLAG_RUN_IN_BACKGROUND_OFST: c_int = 8;
pub const MC_CMD_NVRAM_UPDATE_FINISH_V2_IN_FLAG_RUN_IN_BACKGROUND_LBN: c_int = 1;
pub const MC_CMD_NVRAM_UPDATE_FINISH_V2_IN_FLAG_RUN_IN_BACKGROUND_WIDTH: c_int = 1;
pub const MC_CMD_NVRAM_UPDATE_FINISH_V2_IN_FLAG_POLL_VERIFY_RESULT_OFST: c_int = 8;
pub const MC_CMD_NVRAM_UPDATE_FINISH_V2_IN_FLAG_POLL_VERIFY_RESULT_LBN: c_int = 2;
pub const MC_CMD_NVRAM_UPDATE_FINISH_V2_IN_FLAG_POLL_VERIFY_RESULT_WIDTH: c_int = 1;
// MC_CMD_NVRAM_UPDATE_FINISH_OUT msgresponse: Legacy NVRAM_UPDATE_FINISH
// response. Use NVRAM_UPDATE_FINISH_V2_OUT in new code
//
pub const MC_CMD_NVRAM_UPDATE_FINISH_OUT_LEN: c_int = 0;
// MC_CMD_NVRAM_UPDATE_FINISH_V2_OUT msgresponse:
//
// Extended NVRAM_UPDATE_FINISH response that communicates the result of secure
// firmware validation where applicable back to the host.
//
// Medford only: For signed firmware images, such as those for medford, the MC
// firmware verifies the signature before marking the firmware image as valid.
// This process takes a few seconds to complete. So is likely to take more than
// the MCDI timeout. Hence signature verification is initiated when
// MC_CMD_NVRAM_UPDATE_FINISH_V2_IN is received by the firmware, however, the
// MCDI command is run in a background MCDI processing thread. This response
// payload includes the results of the signature verification. Note that the
// per-partition nvram lock in firmware is only released after the verification
// has completed.
//
pub const MC_CMD_NVRAM_UPDATE_FINISH_V2_OUT_LEN: c_int = 4;
// Result of nvram update completion processing. Result codes that indicate an
// internal build failure and therefore not expected to be seen by customers in
// the field are marked with a prefix 'Internal-error'.
//
pub const MC_CMD_NVRAM_UPDATE_FINISH_V2_OUT_RESULT_CODE_OFST: c_int = 0;
pub const MC_CMD_NVRAM_UPDATE_FINISH_V2_OUT_RESULT_CODE_LEN: c_int = 4;
// enum: Invalid return code; only non-zero values are defined. Defined as
// unknown for backwards compatibility with NVRAM_UPDATE_FINISH_OUT.
//
pub const MC_CMD_NVRAM_VERIFY_RC_UNKNOWN: c_uint = 0x0;
// enum: Verify succeeded without any errors.
pub const MC_CMD_NVRAM_VERIFY_RC_SUCCESS: c_uint = 0x1;
// enum: CMS format verification failed due to an internal error.
pub const MC_CMD_NVRAM_VERIFY_RC_CMS_CHECK_FAILED: c_uint = 0x2;
// enum: Invalid CMS format in image metadata.
pub const MC_CMD_NVRAM_VERIFY_RC_INVALID_CMS_FORMAT: c_uint = 0x3;
// enum: Message digest verification failed due to an internal error.
pub const MC_CMD_NVRAM_VERIFY_RC_MESSAGE_DIGEST_CHECK_FAILED: c_uint = 0x4;
// enum: Error in message digest calculated over the reflash-header, payload
// and reflash-trailer.
//
pub const MC_CMD_NVRAM_VERIFY_RC_BAD_MESSAGE_DIGEST: c_uint = 0x5;
// enum: Signature verification failed due to an internal error.
pub const MC_CMD_NVRAM_VERIFY_RC_SIGNATURE_CHECK_FAILED: c_uint = 0x6;
// enum: There are no valid signatures in the image.
pub const MC_CMD_NVRAM_VERIFY_RC_NO_VALID_SIGNATURES: c_uint = 0x7;
// enum: Trusted approvers verification failed due to an internal error.
pub const MC_CMD_NVRAM_VERIFY_RC_TRUSTED_APPROVERS_CHECK_FAILED: c_uint = 0x8;
// enum: The Trusted approver's list is empty.
pub const MC_CMD_NVRAM_VERIFY_RC_NO_TRUSTED_APPROVERS: c_uint = 0x9;
// enum: Signature chain verification failed due to an internal error.
pub const MC_CMD_NVRAM_VERIFY_RC_SIGNATURE_CHAIN_CHECK_FAILED: c_uint = 0xa;
// enum: The signers of the signatures in the image are not listed in the
// Trusted approver's list.
//
pub const MC_CMD_NVRAM_VERIFY_RC_NO_SIGNATURE_MATCH: c_uint = 0xb;
// enum: The image contains a test-signed certificate, but the adapter accepts
// only production signed images.
//
pub const MC_CMD_NVRAM_VERIFY_RC_REJECT_TEST_SIGNED: c_uint = 0xc;
// enum: The image has a lower security level than the current firmware.
pub const MC_CMD_NVRAM_VERIFY_RC_SECURITY_LEVEL_DOWNGRADE: c_uint = 0xd;
// enum: Internal-error. The signed image is missing the 'contents' section,
// where the 'contents' section holds the actual image payload to be applied.
//
pub const MC_CMD_NVRAM_VERIFY_RC_CONTENT_NOT_FOUND: c_uint = 0xe;
// enum: Internal-error. The bundle header is invalid.
pub const MC_CMD_NVRAM_VERIFY_RC_BUNDLE_CONTENT_HEADER_INVALID: c_uint = 0xf;
// enum: Internal-error. The bundle does not have a valid reflash image layout.
//
pub const MC_CMD_NVRAM_VERIFY_RC_BUNDLE_REFLASH_IMAGE_INVALID: c_uint = 0x10;
// enum: Internal-error. The bundle has an inconsistent layout of components or
// incorrect checksum.
//
pub const MC_CMD_NVRAM_VERIFY_RC_BUNDLE_IMAGE_LAYOUT_INVALID: c_uint = 0x11;
// enum: Internal-error. The bundle manifest is inconsistent with components in
// the bundle.
//
pub const MC_CMD_NVRAM_VERIFY_RC_BUNDLE_MANIFEST_INVALID: c_uint = 0x12;
// enum: Internal-error. The number of components in a bundle do not match the
// number of components advertised by the bundle manifest.
//
pub const MC_CMD_NVRAM_VERIFY_RC_BUNDLE_MANIFEST_NUM_COMPONENTS_MISMATCH: c_uint = 0x13;
// enum: Internal-error. The bundle contains too many components for the MC
// firmware to process
//
pub const MC_CMD_NVRAM_VERIFY_RC_BUNDLE_MANIFEST_TOO_MANY_COMPONENTS: c_uint = 0x14;
// enum: Internal-error. The bundle manifest has an invalid/inconsistent
// component.
//
pub const MC_CMD_NVRAM_VERIFY_RC_BUNDLE_MANIFEST_COMPONENT_INVALID: c_uint = 0x15;
// enum: Internal-error. The hash of a component does not match the hash stored
// in the bundle manifest.
//
pub const MC_CMD_NVRAM_VERIFY_RC_BUNDLE_MANIFEST_COMPONENT_HASH_MISMATCH: c_uint = 0x16;
// enum: Internal-error. Component hash calculation failed.
pub const MC_CMD_NVRAM_VERIFY_RC_BUNDLE_MANIFEST_COMPONENT_HASH_FAILED: c_uint = 0x17;
// enum: Internal-error. The component does not have a valid reflash image
// layout.
//
pub const MC_CMD_NVRAM_VERIFY_RC_BUNDLE_COMPONENT_REFLASH_IMAGE_INVALID: c_uint = 0x18;
// enum: The bundle processing code failed to copy a component to its target
// partition.
//
pub const MC_CMD_NVRAM_VERIFY_RC_BUNDLE_COMPONENT_COPY_FAILED: c_uint = 0x19;
// enum: The update operation is in-progress.
pub const MC_CMD_NVRAM_VERIFY_RC_PENDING: c_uint = 0x1a;
//
// MC_CMD_REBOOT
// Reboot the MC.
//
// The AFTER_ASSERTION flag is intended to be used when the driver notices an
// assertion failure (at which point it is expected to perform a complete tear
// down and reinitialise), to allow both ports to reset the MC once in an
// atomic fashion.
//
// Production mc firmwares are generally compiled with REBOOT_ON_ASSERT=1,
// which means that they will automatically reboot out of the assertion
// handler, so this is in practise an optional operation. It is still
// recommended that drivers execute this to support custom firmwares with
// REBOOT_ON_ASSERT=0.
//
// Locks required: NONE Returns: Nothing. You get back a response with ERR=1,
// DATALEN=0
//
pub const MC_CMD_REBOOT: c_uint = 0x3d;

// MC_CMD_REBOOT_IN msgrequest
pub const MC_CMD_REBOOT_IN_LEN: c_int = 4;
pub const MC_CMD_REBOOT_IN_FLAGS_OFST: c_int = 0;
pub const MC_CMD_REBOOT_IN_FLAGS_LEN: c_int = 4;
pub const MC_CMD_REBOOT_FLAGS_AFTER_ASSERTION: c_uint = 0x1 /* enum */;
// MC_CMD_REBOOT_OUT msgresponse
pub const MC_CMD_REBOOT_OUT_LEN: c_int = 0;
//
// MC_CMD_SCHEDINFO
// Request scheduler info. Locks required: NONE. Returns: An array of
// (timeslice,maximum overrun), one for each thread, in ascending order of
// thread address.
//
pub const MC_CMD_SCHEDINFO: c_uint = 0x3e;

// MC_CMD_SCHEDINFO_IN msgrequest
pub const MC_CMD_SCHEDINFO_IN_LEN: c_int = 0;
// MC_CMD_SCHEDINFO_OUT msgresponse
pub const MC_CMD_SCHEDINFO_OUT_LENMIN: c_int = 4;
pub const MC_CMD_SCHEDINFO_OUT_LENMAX: c_int = 252;
pub const MC_CMD_SCHEDINFO_OUT_LENMAX_MCDI2: c_int = 1020;

pub const MC_CMD_SCHEDINFO_OUT_DATA_OFST: c_int = 0;
pub const MC_CMD_SCHEDINFO_OUT_DATA_LEN: c_int = 4;
pub const MC_CMD_SCHEDINFO_OUT_DATA_MINNUM: c_int = 1;
pub const MC_CMD_SCHEDINFO_OUT_DATA_MAXNUM: c_int = 63;
pub const MC_CMD_SCHEDINFO_OUT_DATA_MAXNUM_MCDI2: c_int = 255;
//
// MC_CMD_REBOOT_MODE
// Set the mode for the next MC reboot. Locks required: NONE. Sets the reboot
// mode to the specified value. Returns the old mode.
//
pub const MC_CMD_REBOOT_MODE: c_uint = 0x3f;

// MC_CMD_REBOOT_MODE_IN msgrequest
pub const MC_CMD_REBOOT_MODE_IN_LEN: c_int = 4;
pub const MC_CMD_REBOOT_MODE_IN_VALUE_OFST: c_int = 0;
pub const MC_CMD_REBOOT_MODE_IN_VALUE_LEN: c_int = 4;
// enum: Normal.
pub const MC_CMD_REBOOT_MODE_NORMAL: c_uint = 0x0;
// enum: Power-on Reset.
pub const MC_CMD_REBOOT_MODE_POR: c_uint = 0x2;
// enum: Snapper.
pub const MC_CMD_REBOOT_MODE_SNAPPER: c_uint = 0x3;
// enum: snapper fake POR
pub const MC_CMD_REBOOT_MODE_SNAPPER_POR: c_uint = 0x4;
pub const MC_CMD_REBOOT_MODE_IN_FAKE_OFST: c_int = 0;
pub const MC_CMD_REBOOT_MODE_IN_FAKE_LBN: c_int = 7;
pub const MC_CMD_REBOOT_MODE_IN_FAKE_WIDTH: c_int = 1;
// MC_CMD_REBOOT_MODE_OUT msgresponse
pub const MC_CMD_REBOOT_MODE_OUT_LEN: c_int = 4;
pub const MC_CMD_REBOOT_MODE_OUT_VALUE_OFST: c_int = 0;
pub const MC_CMD_REBOOT_MODE_OUT_VALUE_LEN: c_int = 4;
//
// MC_CMD_SENSOR_INFO
// Returns information about every available sensor.
//
// Each sensor has a single (16bit) value, and a corresponding state. The
// mapping between value and state is nominally determined by the MC, but may
// be implemented using up to 2 ranges per sensor.
//
// This call returns a mask (32bit) of the sensors that are supported by this
// platform, then an array of sensor information structures, in order of sensor
// type (but without gaps for unimplemented sensors). Each structure defines
// the ranges for the corresponding sensor. An unused range is indicated by
// equal limit values. If one range is used, a value outside that range results
// in STATE_FATAL. If two ranges are used, a value outside the second range
// results in STATE_FATAL while a value outside the first and inside the second
// range results in STATE_WARNING.
//
// Sensor masks and sensor information arrays are organised into pages. For
// backward compatibility, older host software can only use sensors in page 0.
// Bit 32 in the sensor mask was previously unused, and is no reserved for use
// as the next page flag.
//
// If the request does not contain a PAGE value then firmware will only return
// page 0 of sensor information, with bit 31 in the sensor mask cleared.
//
// If the request contains a PAGE value then firmware responds with the sensor
// mask and sensor information array for that page of sensors. In this case bit
// 31 in the mask is set if another page exists.
//
// Locks required: None Returns: 0
//
pub const MC_CMD_SENSOR_INFO: c_uint = 0x41;

// MC_CMD_SENSOR_INFO_IN msgrequest
pub const MC_CMD_SENSOR_INFO_IN_LEN: c_int = 0;
// MC_CMD_SENSOR_INFO_EXT_IN msgrequest
pub const MC_CMD_SENSOR_INFO_EXT_IN_LEN: c_int = 4;
// Which page of sensors to report.
//
// Page 0 contains sensors 0 to 30 (sensor 31 is the next page bit).
//
// Page 1 contains sensors 32 to 62 (sensor 63 is the next page bit). etc.
//
pub const MC_CMD_SENSOR_INFO_EXT_IN_PAGE_OFST: c_int = 0;
pub const MC_CMD_SENSOR_INFO_EXT_IN_PAGE_LEN: c_int = 4;
// MC_CMD_SENSOR_INFO_EXT_IN_V2 msgrequest
pub const MC_CMD_SENSOR_INFO_EXT_IN_V2_LEN: c_int = 8;
// Which page of sensors to report.
//
// Page 0 contains sensors 0 to 30 (sensor 31 is the next page bit).
//
// Page 1 contains sensors 32 to 62 (sensor 63 is the next page bit). etc.
//
pub const MC_CMD_SENSOR_INFO_EXT_IN_V2_PAGE_OFST: c_int = 0;
pub const MC_CMD_SENSOR_INFO_EXT_IN_V2_PAGE_LEN: c_int = 4;
// Flags controlling information retrieved
pub const MC_CMD_SENSOR_INFO_EXT_IN_V2_FLAGS_OFST: c_int = 4;
pub const MC_CMD_SENSOR_INFO_EXT_IN_V2_FLAGS_LEN: c_int = 4;
pub const MC_CMD_SENSOR_INFO_EXT_IN_V2_ENGINEERING_OFST: c_int = 4;
pub const MC_CMD_SENSOR_INFO_EXT_IN_V2_ENGINEERING_LBN: c_int = 0;
pub const MC_CMD_SENSOR_INFO_EXT_IN_V2_ENGINEERING_WIDTH: c_int = 1;
// MC_CMD_SENSOR_INFO_OUT msgresponse
pub const MC_CMD_SENSOR_INFO_OUT_LENMIN: c_int = 4;
pub const MC_CMD_SENSOR_INFO_OUT_LENMAX: c_int = 252;
pub const MC_CMD_SENSOR_INFO_OUT_LENMAX_MCDI2: c_int = 1020;

pub const MC_CMD_SENSOR_INFO_OUT_MASK_OFST: c_int = 0;
pub const MC_CMD_SENSOR_INFO_OUT_MASK_LEN: c_int = 4;
// enum: Controller temperature: degC
pub const MC_CMD_SENSOR_CONTROLLER_TEMP: c_uint = 0x0;
// enum: Phy common temperature: degC
pub const MC_CMD_SENSOR_PHY_COMMON_TEMP: c_uint = 0x1;
// enum: Controller cooling: bool
pub const MC_CMD_SENSOR_CONTROLLER_COOLING: c_uint = 0x2;
// enum: Phy 0 temperature: degC
pub const MC_CMD_SENSOR_PHY0_TEMP: c_uint = 0x3;
// enum: Phy 0 cooling: bool
pub const MC_CMD_SENSOR_PHY0_COOLING: c_uint = 0x4;
// enum: Phy 1 temperature: degC
pub const MC_CMD_SENSOR_PHY1_TEMP: c_uint = 0x5;
// enum: Phy 1 cooling: bool
pub const MC_CMD_SENSOR_PHY1_COOLING: c_uint = 0x6;
// enum: 1.0v power: mV
pub const MC_CMD_SENSOR_IN_1V0: c_uint = 0x7;
// enum: 1.2v power: mV
pub const MC_CMD_SENSOR_IN_1V2: c_uint = 0x8;
// enum: 1.8v power: mV
pub const MC_CMD_SENSOR_IN_1V8: c_uint = 0x9;
// enum: 2.5v power: mV
pub const MC_CMD_SENSOR_IN_2V5: c_uint = 0xa;
// enum: 3.3v power: mV
pub const MC_CMD_SENSOR_IN_3V3: c_uint = 0xb;
// enum: 12v power: mV
pub const MC_CMD_SENSOR_IN_12V0: c_uint = 0xc;
// enum: 1.2v analogue power: mV
pub const MC_CMD_SENSOR_IN_1V2A: c_uint = 0xd;
// enum: reference voltage: mV
pub const MC_CMD_SENSOR_IN_VREF: c_uint = 0xe;
// enum: AOE FPGA power: mV
pub const MC_CMD_SENSOR_OUT_VAOE: c_uint = 0xf;
// enum: AOE FPGA temperature: degC
pub const MC_CMD_SENSOR_AOE_TEMP: c_uint = 0x10;
// enum: AOE FPGA PSU temperature: degC
pub const MC_CMD_SENSOR_PSU_AOE_TEMP: c_uint = 0x11;
// enum: AOE PSU temperature: degC
pub const MC_CMD_SENSOR_PSU_TEMP: c_uint = 0x12;
// enum: Fan 0 speed: RPM
pub const MC_CMD_SENSOR_FAN_0: c_uint = 0x13;
// enum: Fan 1 speed: RPM
pub const MC_CMD_SENSOR_FAN_1: c_uint = 0x14;
// enum: Fan 2 speed: RPM
pub const MC_CMD_SENSOR_FAN_2: c_uint = 0x15;
// enum: Fan 3 speed: RPM
pub const MC_CMD_SENSOR_FAN_3: c_uint = 0x16;
// enum: Fan 4 speed: RPM
pub const MC_CMD_SENSOR_FAN_4: c_uint = 0x17;
// enum: AOE FPGA input power: mV
pub const MC_CMD_SENSOR_IN_VAOE: c_uint = 0x18;
// enum: AOE FPGA current: mA
pub const MC_CMD_SENSOR_OUT_IAOE: c_uint = 0x19;
// enum: AOE FPGA input current: mA
pub const MC_CMD_SENSOR_IN_IAOE: c_uint = 0x1a;
// enum: NIC power consumption: W
pub const MC_CMD_SENSOR_NIC_POWER: c_uint = 0x1b;
// enum: 0.9v power voltage: mV
pub const MC_CMD_SENSOR_IN_0V9: c_uint = 0x1c;
// enum: 0.9v power current: mA
pub const MC_CMD_SENSOR_IN_I0V9: c_uint = 0x1d;
// enum: 1.2v power current: mA
pub const MC_CMD_SENSOR_IN_I1V2: c_uint = 0x1e;
// enum: Not a sensor: reserved for the next page flag
pub const MC_CMD_SENSOR_PAGE0_NEXT: c_uint = 0x1f;
// enum: 0.9v power voltage (at ADC): mV
pub const MC_CMD_SENSOR_IN_0V9_ADC: c_uint = 0x20;
// enum: Controller temperature 2: degC
pub const MC_CMD_SENSOR_CONTROLLER_2_TEMP: c_uint = 0x21;
// enum: Voltage regulator internal temperature: degC
pub const MC_CMD_SENSOR_VREG_INTERNAL_TEMP: c_uint = 0x22;
// enum: 0.9V voltage regulator temperature: degC
pub const MC_CMD_SENSOR_VREG_0V9_TEMP: c_uint = 0x23;
// enum: 1.2V voltage regulator temperature: degC
pub const MC_CMD_SENSOR_VREG_1V2_TEMP: c_uint = 0x24;
// enum: controller internal temperature sensor voltage (internal ADC): mV
pub const MC_CMD_SENSOR_CONTROLLER_VPTAT: c_uint = 0x25;
// enum: controller internal temperature (internal ADC): degC
pub const MC_CMD_SENSOR_CONTROLLER_INTERNAL_TEMP: c_uint = 0x26;
// enum: controller internal temperature sensor voltage (external ADC): mV
pub const MC_CMD_SENSOR_CONTROLLER_VPTAT_EXTADC: c_uint = 0x27;
// enum: controller internal temperature (external ADC): degC
pub const MC_CMD_SENSOR_CONTROLLER_INTERNAL_TEMP_EXTADC: c_uint = 0x28;
// enum: ambient temperature: degC
pub const MC_CMD_SENSOR_AMBIENT_TEMP: c_uint = 0x29;
// enum: air flow: bool
pub const MC_CMD_SENSOR_AIRFLOW: c_uint = 0x2a;
// enum: voltage between VSS08D and VSS08D at CSR: mV
pub const MC_CMD_SENSOR_VDD08D_VSS08D_CSR: c_uint = 0x2b;
// enum: voltage between VSS08D and VSS08D at CSR (external ADC): mV
pub const MC_CMD_SENSOR_VDD08D_VSS08D_CSR_EXTADC: c_uint = 0x2c;
// enum: Hotpoint temperature: degC
pub const MC_CMD_SENSOR_HOTPOINT_TEMP: c_uint = 0x2d;
// enum: Port 0 PHY power switch over-current: bool
pub const MC_CMD_SENSOR_PHY_POWER_PORT0: c_uint = 0x2e;
// enum: Port 1 PHY power switch over-current: bool
pub const MC_CMD_SENSOR_PHY_POWER_PORT1: c_uint = 0x2f;
// enum: Mop-up microcontroller reference voltage: mV
pub const MC_CMD_SENSOR_MUM_VCC: c_uint = 0x30;
// enum: 0.9v power phase A voltage: mV
pub const MC_CMD_SENSOR_IN_0V9_A: c_uint = 0x31;
// enum: 0.9v power phase A current: mA
pub const MC_CMD_SENSOR_IN_I0V9_A: c_uint = 0x32;
// enum: 0.9V voltage regulator phase A temperature: degC
pub const MC_CMD_SENSOR_VREG_0V9_A_TEMP: c_uint = 0x33;
// enum: 0.9v power phase B voltage: mV
pub const MC_CMD_SENSOR_IN_0V9_B: c_uint = 0x34;
// enum: 0.9v power phase B current: mA
pub const MC_CMD_SENSOR_IN_I0V9_B: c_uint = 0x35;
// enum: 0.9V voltage regulator phase B temperature: degC
pub const MC_CMD_SENSOR_VREG_0V9_B_TEMP: c_uint = 0x36;
// enum: CCOM AVREG 1v2 supply (interval ADC): mV
pub const MC_CMD_SENSOR_CCOM_AVREG_1V2_SUPPLY: c_uint = 0x37;
// enum: CCOM AVREG 1v2 supply (external ADC): mV
pub const MC_CMD_SENSOR_CCOM_AVREG_1V2_SUPPLY_EXTADC: c_uint = 0x38;
// enum: CCOM AVREG 1v8 supply (interval ADC): mV
pub const MC_CMD_SENSOR_CCOM_AVREG_1V8_SUPPLY: c_uint = 0x39;
// enum: CCOM AVREG 1v8 supply (external ADC): mV
pub const MC_CMD_SENSOR_CCOM_AVREG_1V8_SUPPLY_EXTADC: c_uint = 0x3a;
// enum: CCOM RTS temperature: degC
pub const MC_CMD_SENSOR_CONTROLLER_RTS: c_uint = 0x3b;
// enum: Not a sensor: reserved for the next page flag
pub const MC_CMD_SENSOR_PAGE1_NEXT: c_uint = 0x3f;
// enum: controller internal temperature sensor voltage on master core
// (internal ADC): mV
//
pub const MC_CMD_SENSOR_CONTROLLER_MASTER_VPTAT: c_uint = 0x40;
// enum: controller internal temperature on master core (internal ADC): degC
pub const MC_CMD_SENSOR_CONTROLLER_MASTER_INTERNAL_TEMP: c_uint = 0x41;
// enum: controller internal temperature sensor voltage on master core
// (external ADC): mV
//
pub const MC_CMD_SENSOR_CONTROLLER_MASTER_VPTAT_EXTADC: c_uint = 0x42;
// enum: controller internal temperature on master core (external ADC): degC
pub const MC_CMD_SENSOR_CONTROLLER_MASTER_INTERNAL_TEMP_EXTADC: c_uint = 0x43;
// enum: controller internal temperature on slave core sensor voltage (internal
// ADC): mV
//
pub const MC_CMD_SENSOR_CONTROLLER_SLAVE_VPTAT: c_uint = 0x44;
// enum: controller internal temperature on slave core (internal ADC): degC
pub const MC_CMD_SENSOR_CONTROLLER_SLAVE_INTERNAL_TEMP: c_uint = 0x45;
// enum: controller internal temperature on slave core sensor voltage (external
// ADC): mV
//
pub const MC_CMD_SENSOR_CONTROLLER_SLAVE_VPTAT_EXTADC: c_uint = 0x46;
// enum: controller internal temperature on slave core (external ADC): degC
pub const MC_CMD_SENSOR_CONTROLLER_SLAVE_INTERNAL_TEMP_EXTADC: c_uint = 0x47;
// enum: Voltage supplied to the SODIMMs from their power supply: mV
pub const MC_CMD_SENSOR_SODIMM_VOUT: c_uint = 0x49;
// enum: Temperature of SODIMM 0 (if installed): degC
pub const MC_CMD_SENSOR_SODIMM_0_TEMP: c_uint = 0x4a;
// enum: Temperature of SODIMM 1 (if installed): degC
pub const MC_CMD_SENSOR_SODIMM_1_TEMP: c_uint = 0x4b;
// enum: Voltage supplied to the QSFP #0 from their power supply: mV
pub const MC_CMD_SENSOR_PHY0_VCC: c_uint = 0x4c;
// enum: Voltage supplied to the QSFP #1 from their power supply: mV
pub const MC_CMD_SENSOR_PHY1_VCC: c_uint = 0x4d;
// enum: Controller die temperature (TDIODE): degC
pub const MC_CMD_SENSOR_CONTROLLER_TDIODE_TEMP: c_uint = 0x4e;
// enum: Board temperature (front): degC
pub const MC_CMD_SENSOR_BOARD_FRONT_TEMP: c_uint = 0x4f;
// enum: Board temperature (back): degC
pub const MC_CMD_SENSOR_BOARD_BACK_TEMP: c_uint = 0x50;
// enum: 1.8v power current: mA
pub const MC_CMD_SENSOR_IN_I1V8: c_uint = 0x51;
// enum: 2.5v power current: mA
pub const MC_CMD_SENSOR_IN_I2V5: c_uint = 0x52;
// enum: 3.3v power current: mA
pub const MC_CMD_SENSOR_IN_I3V3: c_uint = 0x53;
// enum: 12v power current: mA
pub const MC_CMD_SENSOR_IN_I12V0: c_uint = 0x54;
// enum: 1.3v power: mV
pub const MC_CMD_SENSOR_IN_1V3: c_uint = 0x55;
// enum: 1.3v power current: mA
pub const MC_CMD_SENSOR_IN_I1V3: c_uint = 0x56;
// enum: Engineering sensor 1
pub const MC_CMD_SENSOR_ENGINEERING_1: c_uint = 0x57;
// enum: Engineering sensor 2
pub const MC_CMD_SENSOR_ENGINEERING_2: c_uint = 0x58;
// enum: Engineering sensor 3
pub const MC_CMD_SENSOR_ENGINEERING_3: c_uint = 0x59;
// enum: Engineering sensor 4
pub const MC_CMD_SENSOR_ENGINEERING_4: c_uint = 0x5a;
// enum: Engineering sensor 5
pub const MC_CMD_SENSOR_ENGINEERING_5: c_uint = 0x5b;
// enum: Engineering sensor 6
pub const MC_CMD_SENSOR_ENGINEERING_6: c_uint = 0x5c;
// enum: Engineering sensor 7
pub const MC_CMD_SENSOR_ENGINEERING_7: c_uint = 0x5d;
// enum: Engineering sensor 8
pub const MC_CMD_SENSOR_ENGINEERING_8: c_uint = 0x5e;
// enum: Not a sensor: reserved for the next page flag
pub const MC_CMD_SENSOR_PAGE2_NEXT: c_uint = 0x5f;
// MC_CMD_SENSOR_INFO_ENTRY_TYPEDEF
pub const MC_CMD_SENSOR_ENTRY_OFST: c_int = 4;
pub const MC_CMD_SENSOR_ENTRY_LEN: c_int = 8;
pub const MC_CMD_SENSOR_ENTRY_LO_OFST: c_int = 4;
pub const MC_CMD_SENSOR_ENTRY_HI_OFST: c_int = 8;
pub const MC_CMD_SENSOR_ENTRY_MINNUM: c_int = 0;
pub const MC_CMD_SENSOR_ENTRY_MAXNUM: c_int = 31;
pub const MC_CMD_SENSOR_ENTRY_MAXNUM_MCDI2: c_int = 127;
// MC_CMD_SENSOR_INFO_EXT_OUT msgresponse
pub const MC_CMD_SENSOR_INFO_EXT_OUT_LENMIN: c_int = 4;
pub const MC_CMD_SENSOR_INFO_EXT_OUT_LENMAX: c_int = 252;
pub const MC_CMD_SENSOR_INFO_EXT_OUT_LENMAX_MCDI2: c_int = 1020;

pub const MC_CMD_SENSOR_INFO_EXT_OUT_MASK_OFST: c_int = 0;
pub const MC_CMD_SENSOR_INFO_EXT_OUT_MASK_LEN: c_int = 4;
// Enum values, see field(s):
// MC_CMD_SENSOR_INFO_OUT
pub const MC_CMD_SENSOR_INFO_EXT_OUT_NEXT_PAGE_OFST: c_int = 0;
pub const MC_CMD_SENSOR_INFO_EXT_OUT_NEXT_PAGE_LBN: c_int = 31;
pub const MC_CMD_SENSOR_INFO_EXT_OUT_NEXT_PAGE_WIDTH: c_int = 1;
// MC_CMD_SENSOR_INFO_ENTRY_TYPEDEF
// MC_CMD_SENSOR_ENTRY_OFST 4
// MC_CMD_SENSOR_ENTRY_LEN 8
// MC_CMD_SENSOR_ENTRY_LO_OFST 4
// MC_CMD_SENSOR_ENTRY_HI_OFST 8
// MC_CMD_SENSOR_ENTRY_MINNUM 0
// MC_CMD_SENSOR_ENTRY_MAXNUM 31
// MC_CMD_SENSOR_ENTRY_MAXNUM_MCDI2 127
// MC_CMD_SENSOR_INFO_ENTRY_TYPEDEF structuredef
pub const MC_CMD_SENSOR_INFO_ENTRY_TYPEDEF_LEN: c_int = 8;
pub const MC_CMD_SENSOR_INFO_ENTRY_TYPEDEF_MIN1_OFST: c_int = 0;
pub const MC_CMD_SENSOR_INFO_ENTRY_TYPEDEF_MIN1_LEN: c_int = 2;
pub const MC_CMD_SENSOR_INFO_ENTRY_TYPEDEF_MIN1_LBN: c_int = 0;
pub const MC_CMD_SENSOR_INFO_ENTRY_TYPEDEF_MIN1_WIDTH: c_int = 16;
pub const MC_CMD_SENSOR_INFO_ENTRY_TYPEDEF_MAX1_OFST: c_int = 2;
pub const MC_CMD_SENSOR_INFO_ENTRY_TYPEDEF_MAX1_LEN: c_int = 2;
pub const MC_CMD_SENSOR_INFO_ENTRY_TYPEDEF_MAX1_LBN: c_int = 16;
pub const MC_CMD_SENSOR_INFO_ENTRY_TYPEDEF_MAX1_WIDTH: c_int = 16;
pub const MC_CMD_SENSOR_INFO_ENTRY_TYPEDEF_MIN2_OFST: c_int = 4;
pub const MC_CMD_SENSOR_INFO_ENTRY_TYPEDEF_MIN2_LEN: c_int = 2;
pub const MC_CMD_SENSOR_INFO_ENTRY_TYPEDEF_MIN2_LBN: c_int = 32;
pub const MC_CMD_SENSOR_INFO_ENTRY_TYPEDEF_MIN2_WIDTH: c_int = 16;
pub const MC_CMD_SENSOR_INFO_ENTRY_TYPEDEF_MAX2_OFST: c_int = 6;
pub const MC_CMD_SENSOR_INFO_ENTRY_TYPEDEF_MAX2_LEN: c_int = 2;
pub const MC_CMD_SENSOR_INFO_ENTRY_TYPEDEF_MAX2_LBN: c_int = 48;
pub const MC_CMD_SENSOR_INFO_ENTRY_TYPEDEF_MAX2_WIDTH: c_int = 16;
//
// MC_CMD_READ_SENSORS
// Returns the current reading from each sensor. DMAs an array of sensor
// readings, in order of sensor type (but without gaps for unimplemented
// sensors), into host memory. Each array element is a
// MC_CMD_SENSOR_VALUE_ENTRY_TYPEDEF dword.
//
// If the request does not contain the LENGTH field then only sensors 0 to 30
// are reported, to avoid DMA buffer overflow in older host software. If the
// sensor reading require more space than the LENGTH allows, then return
// EINVAL.
//
// The MC will send a SENSOREVT event every time any sensor changes state. The
// driver is responsible for ensuring that it doesn't miss any events. The
// board will function normally if all sensors are in STATE_OK or
// STATE_WARNING. Otherwise the board should not be expected to function.
//
pub const MC_CMD_READ_SENSORS: c_uint = 0x42;

// MC_CMD_READ_SENSORS_IN msgrequest
pub const MC_CMD_READ_SENSORS_IN_LEN: c_int = 8;
// DMA address of host buffer for sensor readings (must be 4Kbyte aligned).
//
// If the address is 0xffffffffffffffff send the readings in the response (used
// by cmdclient).
//
pub const MC_CMD_READ_SENSORS_IN_DMA_ADDR_OFST: c_int = 0;
pub const MC_CMD_READ_SENSORS_IN_DMA_ADDR_LEN: c_int = 8;
pub const MC_CMD_READ_SENSORS_IN_DMA_ADDR_LO_OFST: c_int = 0;
pub const MC_CMD_READ_SENSORS_IN_DMA_ADDR_HI_OFST: c_int = 4;
// MC_CMD_READ_SENSORS_EXT_IN msgrequest
pub const MC_CMD_READ_SENSORS_EXT_IN_LEN: c_int = 12;
// DMA address of host buffer for sensor readings (must be 4Kbyte aligned).
//
// If the address is 0xffffffffffffffff send the readings in the response (used
// by cmdclient).
//
pub const MC_CMD_READ_SENSORS_EXT_IN_DMA_ADDR_OFST: c_int = 0;
pub const MC_CMD_READ_SENSORS_EXT_IN_DMA_ADDR_LEN: c_int = 8;
pub const MC_CMD_READ_SENSORS_EXT_IN_DMA_ADDR_LO_OFST: c_int = 0;
pub const MC_CMD_READ_SENSORS_EXT_IN_DMA_ADDR_HI_OFST: c_int = 4;
// Size in bytes of host buffer.
pub const MC_CMD_READ_SENSORS_EXT_IN_LENGTH_OFST: c_int = 8;
pub const MC_CMD_READ_SENSORS_EXT_IN_LENGTH_LEN: c_int = 4;
// MC_CMD_READ_SENSORS_EXT_IN_V2 msgrequest
pub const MC_CMD_READ_SENSORS_EXT_IN_V2_LEN: c_int = 16;
// DMA address of host buffer for sensor readings (must be 4Kbyte aligned).
//
// If the address is 0xffffffffffffffff send the readings in the response (used
// by cmdclient).
//
pub const MC_CMD_READ_SENSORS_EXT_IN_V2_DMA_ADDR_OFST: c_int = 0;
pub const MC_CMD_READ_SENSORS_EXT_IN_V2_DMA_ADDR_LEN: c_int = 8;
pub const MC_CMD_READ_SENSORS_EXT_IN_V2_DMA_ADDR_LO_OFST: c_int = 0;
pub const MC_CMD_READ_SENSORS_EXT_IN_V2_DMA_ADDR_HI_OFST: c_int = 4;
// Size in bytes of host buffer.
pub const MC_CMD_READ_SENSORS_EXT_IN_V2_LENGTH_OFST: c_int = 8;
pub const MC_CMD_READ_SENSORS_EXT_IN_V2_LENGTH_LEN: c_int = 4;
// Flags controlling information retrieved
pub const MC_CMD_READ_SENSORS_EXT_IN_V2_FLAGS_OFST: c_int = 12;
pub const MC_CMD_READ_SENSORS_EXT_IN_V2_FLAGS_LEN: c_int = 4;
pub const MC_CMD_READ_SENSORS_EXT_IN_V2_ENGINEERING_OFST: c_int = 12;
pub const MC_CMD_READ_SENSORS_EXT_IN_V2_ENGINEERING_LBN: c_int = 0;
pub const MC_CMD_READ_SENSORS_EXT_IN_V2_ENGINEERING_WIDTH: c_int = 1;
// MC_CMD_READ_SENSORS_OUT msgresponse
pub const MC_CMD_READ_SENSORS_OUT_LEN: c_int = 0;
// MC_CMD_READ_SENSORS_EXT_OUT msgresponse
pub const MC_CMD_READ_SENSORS_EXT_OUT_LEN: c_int = 0;
// MC_CMD_SENSOR_VALUE_ENTRY_TYPEDEF structuredef
pub const MC_CMD_SENSOR_VALUE_ENTRY_TYPEDEF_LEN: c_int = 4;
pub const MC_CMD_SENSOR_VALUE_ENTRY_TYPEDEF_VALUE_OFST: c_int = 0;
pub const MC_CMD_SENSOR_VALUE_ENTRY_TYPEDEF_VALUE_LEN: c_int = 2;
pub const MC_CMD_SENSOR_VALUE_ENTRY_TYPEDEF_VALUE_LBN: c_int = 0;
pub const MC_CMD_SENSOR_VALUE_ENTRY_TYPEDEF_VALUE_WIDTH: c_int = 16;
pub const MC_CMD_SENSOR_VALUE_ENTRY_TYPEDEF_STATE_OFST: c_int = 2;
pub const MC_CMD_SENSOR_VALUE_ENTRY_TYPEDEF_STATE_LEN: c_int = 1;
// enum: Ok.
pub const MC_CMD_SENSOR_STATE_OK: c_uint = 0x0;
// enum: Breached warning threshold.
pub const MC_CMD_SENSOR_STATE_WARNING: c_uint = 0x1;
// enum: Breached fatal threshold.
pub const MC_CMD_SENSOR_STATE_FATAL: c_uint = 0x2;
// enum: Fault with sensor.
pub const MC_CMD_SENSOR_STATE_BROKEN: c_uint = 0x3;
// enum: Sensor is working but does not currently have a reading.
pub const MC_CMD_SENSOR_STATE_NO_READING: c_uint = 0x4;
// enum: Sensor initialisation failed.
pub const MC_CMD_SENSOR_STATE_INIT_FAILED: c_uint = 0x5;
pub const MC_CMD_SENSOR_VALUE_ENTRY_TYPEDEF_STATE_LBN: c_int = 16;
pub const MC_CMD_SENSOR_VALUE_ENTRY_TYPEDEF_STATE_WIDTH: c_int = 8;
pub const MC_CMD_SENSOR_VALUE_ENTRY_TYPEDEF_TYPE_OFST: c_int = 3;
pub const MC_CMD_SENSOR_VALUE_ENTRY_TYPEDEF_TYPE_LEN: c_int = 1;
// Enum values, see field(s):
// MC_CMD_SENSOR_INFO/MC_CMD_SENSOR_INFO_OUT/MASK
pub const MC_CMD_SENSOR_VALUE_ENTRY_TYPEDEF_TYPE_LBN: c_int = 24;
pub const MC_CMD_SENSOR_VALUE_ENTRY_TYPEDEF_TYPE_WIDTH: c_int = 8;
//
// MC_CMD_GET_PHY_STATE
// Report current state of PHY. A 'zombie' PHY is a PHY that has failed to boot
// (e.g. due to missing or corrupted firmware). Locks required: None. Return
// code: 0
//
pub const MC_CMD_GET_PHY_STATE: c_uint = 0x43;

// MC_CMD_GET_PHY_STATE_IN msgrequest
pub const MC_CMD_GET_PHY_STATE_IN_LEN: c_int = 0;
// MC_CMD_GET_PHY_STATE_OUT msgresponse
pub const MC_CMD_GET_PHY_STATE_OUT_LEN: c_int = 4;
pub const MC_CMD_GET_PHY_STATE_OUT_STATE_OFST: c_int = 0;
pub const MC_CMD_GET_PHY_STATE_OUT_STATE_LEN: c_int = 4;
// enum: Ok.
pub const MC_CMD_PHY_STATE_OK: c_uint = 0x1;
// enum: Faulty.
pub const MC_CMD_PHY_STATE_ZOMBIE: c_uint = 0x2;
//
// MC_CMD_SETUP_8021QBB
// 802.1Qbb control. 8 Tx queues that map to priorities 0 - 7. Use all 1s to
// disable 802.Qbb for a given priority.
//
pub const MC_CMD_SETUP_8021QBB: c_uint = 0x44;
// MC_CMD_SETUP_8021QBB_IN msgrequest
pub const MC_CMD_SETUP_8021QBB_IN_LEN: c_int = 32;
pub const MC_CMD_SETUP_8021QBB_IN_TXQS_OFST: c_int = 0;
pub const MC_CMD_SETUP_8021QBB_IN_TXQS_LEN: c_int = 32;
// MC_CMD_SETUP_8021QBB_OUT msgresponse
pub const MC_CMD_SETUP_8021QBB_OUT_LEN: c_int = 0;
//
// MC_CMD_WOL_FILTER_GET
// Retrieve ID of any WoL filters. Locks required: None. Returns: 0, ENOSYS
//
pub const MC_CMD_WOL_FILTER_GET: c_uint = 0x45;

// MC_CMD_WOL_FILTER_GET_IN msgrequest
pub const MC_CMD_WOL_FILTER_GET_IN_LEN: c_int = 0;
// MC_CMD_WOL_FILTER_GET_OUT msgresponse
pub const MC_CMD_WOL_FILTER_GET_OUT_LEN: c_int = 4;
pub const MC_CMD_WOL_FILTER_GET_OUT_FILTER_ID_OFST: c_int = 0;
pub const MC_CMD_WOL_FILTER_GET_OUT_FILTER_ID_LEN: c_int = 4;
//
// MC_CMD_ADD_LIGHTSOUT_OFFLOAD
// Add a protocol offload to NIC for lights-out state. Locks required: None.
// Returns: 0, ENOSYS
//
pub const MC_CMD_ADD_LIGHTSOUT_OFFLOAD: c_uint = 0x46;

// MC_CMD_ADD_LIGHTSOUT_OFFLOAD_IN msgrequest
pub const MC_CMD_ADD_LIGHTSOUT_OFFLOAD_IN_LENMIN: c_int = 8;
pub const MC_CMD_ADD_LIGHTSOUT_OFFLOAD_IN_LENMAX: c_int = 252;
pub const MC_CMD_ADD_LIGHTSOUT_OFFLOAD_IN_LENMAX_MCDI2: c_int = 1020;

pub const MC_CMD_ADD_LIGHTSOUT_OFFLOAD_IN_PROTOCOL_OFST: c_int = 0;
pub const MC_CMD_ADD_LIGHTSOUT_OFFLOAD_IN_PROTOCOL_LEN: c_int = 4;
pub const MC_CMD_LIGHTSOUT_OFFLOAD_PROTOCOL_ARP: c_uint = 0x1 /* enum */;
pub const MC_CMD_LIGHTSOUT_OFFLOAD_PROTOCOL_NS: c_uint = 0x2 /* enum */;
pub const MC_CMD_ADD_LIGHTSOUT_OFFLOAD_IN_DATA_OFST: c_int = 4;
pub const MC_CMD_ADD_LIGHTSOUT_OFFLOAD_IN_DATA_LEN: c_int = 4;
pub const MC_CMD_ADD_LIGHTSOUT_OFFLOAD_IN_DATA_MINNUM: c_int = 1;
pub const MC_CMD_ADD_LIGHTSOUT_OFFLOAD_IN_DATA_MAXNUM: c_int = 62;
pub const MC_CMD_ADD_LIGHTSOUT_OFFLOAD_IN_DATA_MAXNUM_MCDI2: c_int = 254;
// MC_CMD_ADD_LIGHTSOUT_OFFLOAD_IN_ARP msgrequest
pub const MC_CMD_ADD_LIGHTSOUT_OFFLOAD_IN_ARP_LEN: c_int = 14;
// MC_CMD_ADD_LIGHTSOUT_OFFLOAD_IN_PROTOCOL_OFST 0
// MC_CMD_ADD_LIGHTSOUT_OFFLOAD_IN_PROTOCOL_LEN 4
pub const MC_CMD_ADD_LIGHTSOUT_OFFLOAD_IN_ARP_MAC_OFST: c_int = 4;
pub const MC_CMD_ADD_LIGHTSOUT_OFFLOAD_IN_ARP_MAC_LEN: c_int = 6;
pub const MC_CMD_ADD_LIGHTSOUT_OFFLOAD_IN_ARP_IP_OFST: c_int = 10;
pub const MC_CMD_ADD_LIGHTSOUT_OFFLOAD_IN_ARP_IP_LEN: c_int = 4;
// MC_CMD_ADD_LIGHTSOUT_OFFLOAD_IN_NS msgrequest
pub const MC_CMD_ADD_LIGHTSOUT_OFFLOAD_IN_NS_LEN: c_int = 42;
// MC_CMD_ADD_LIGHTSOUT_OFFLOAD_IN_PROTOCOL_OFST 0
// MC_CMD_ADD_LIGHTSOUT_OFFLOAD_IN_PROTOCOL_LEN 4
pub const MC_CMD_ADD_LIGHTSOUT_OFFLOAD_IN_NS_MAC_OFST: c_int = 4;
pub const MC_CMD_ADD_LIGHTSOUT_OFFLOAD_IN_NS_MAC_LEN: c_int = 6;
pub const MC_CMD_ADD_LIGHTSOUT_OFFLOAD_IN_NS_SNIPV6_OFST: c_int = 10;
pub const MC_CMD_ADD_LIGHTSOUT_OFFLOAD_IN_NS_SNIPV6_LEN: c_int = 16;
pub const MC_CMD_ADD_LIGHTSOUT_OFFLOAD_IN_NS_IPV6_OFST: c_int = 26;
pub const MC_CMD_ADD_LIGHTSOUT_OFFLOAD_IN_NS_IPV6_LEN: c_int = 16;
// MC_CMD_ADD_LIGHTSOUT_OFFLOAD_OUT msgresponse
pub const MC_CMD_ADD_LIGHTSOUT_OFFLOAD_OUT_LEN: c_int = 4;
pub const MC_CMD_ADD_LIGHTSOUT_OFFLOAD_OUT_FILTER_ID_OFST: c_int = 0;
pub const MC_CMD_ADD_LIGHTSOUT_OFFLOAD_OUT_FILTER_ID_LEN: c_int = 4;
//
// MC_CMD_REMOVE_LIGHTSOUT_OFFLOAD
// Remove a protocol offload from NIC for lights-out state. Locks required:
// None. Returns: 0, ENOSYS
//
pub const MC_CMD_REMOVE_LIGHTSOUT_OFFLOAD: c_uint = 0x47;

// MC_CMD_REMOVE_LIGHTSOUT_OFFLOAD_IN msgrequest
pub const MC_CMD_REMOVE_LIGHTSOUT_OFFLOAD_IN_LEN: c_int = 8;
pub const MC_CMD_REMOVE_LIGHTSOUT_OFFLOAD_IN_PROTOCOL_OFST: c_int = 0;
pub const MC_CMD_REMOVE_LIGHTSOUT_OFFLOAD_IN_PROTOCOL_LEN: c_int = 4;
pub const MC_CMD_REMOVE_LIGHTSOUT_OFFLOAD_IN_FILTER_ID_OFST: c_int = 4;
pub const MC_CMD_REMOVE_LIGHTSOUT_OFFLOAD_IN_FILTER_ID_LEN: c_int = 4;
// MC_CMD_REMOVE_LIGHTSOUT_OFFLOAD_OUT msgresponse
pub const MC_CMD_REMOVE_LIGHTSOUT_OFFLOAD_OUT_LEN: c_int = 0;
//
// MC_CMD_MAC_RESET_RESTORE
// Restore MAC after block reset. Locks required: None. Returns: 0.
//
pub const MC_CMD_MAC_RESET_RESTORE: c_uint = 0x48;
// MC_CMD_MAC_RESET_RESTORE_IN msgrequest
pub const MC_CMD_MAC_RESET_RESTORE_IN_LEN: c_int = 0;
// MC_CMD_MAC_RESET_RESTORE_OUT msgresponse
pub const MC_CMD_MAC_RESET_RESTORE_OUT_LEN: c_int = 0;
//
// MC_CMD_TESTASSERT
// Deliberately trigger an assert-detonation in the firmware for testing
// purposes (i.e. to allow tests that the driver copes gracefully). Locks
// required: None Returns: 0
//
pub const MC_CMD_TESTASSERT: c_uint = 0x49;

// MC_CMD_TESTASSERT_IN msgrequest
pub const MC_CMD_TESTASSERT_IN_LEN: c_int = 0;
// MC_CMD_TESTASSERT_OUT msgresponse
pub const MC_CMD_TESTASSERT_OUT_LEN: c_int = 0;
// MC_CMD_TESTASSERT_V2_IN msgrequest
pub const MC_CMD_TESTASSERT_V2_IN_LEN: c_int = 4;
// How to provoke the assertion
pub const MC_CMD_TESTASSERT_V2_IN_TYPE_OFST: c_int = 0;
pub const MC_CMD_TESTASSERT_V2_IN_TYPE_LEN: c_int = 4;
// enum: Assert using the FAIL_ASSERTION_WITH_USEFUL_VALUES macro. Unless
// you're testing firmware, this is what you want.
//
pub const MC_CMD_TESTASSERT_V2_IN_FAIL_ASSERTION_WITH_USEFUL_VALUES: c_uint = 0x0;
// enum: Assert using assert(0);
pub const MC_CMD_TESTASSERT_V2_IN_ASSERT_FALSE: c_uint = 0x1;
// enum: Deliberately trigger a watchdog
pub const MC_CMD_TESTASSERT_V2_IN_WATCHDOG: c_uint = 0x2;
// enum: Deliberately trigger a trap by loading from an invalid address
pub const MC_CMD_TESTASSERT_V2_IN_LOAD_TRAP: c_uint = 0x3;
// enum: Deliberately trigger a trap by storing to an invalid address
pub const MC_CMD_TESTASSERT_V2_IN_STORE_TRAP: c_uint = 0x4;
// enum: Jump to an invalid address
pub const MC_CMD_TESTASSERT_V2_IN_JUMP_TRAP: c_uint = 0x5;
// MC_CMD_TESTASSERT_V2_OUT msgresponse
pub const MC_CMD_TESTASSERT_V2_OUT_LEN: c_int = 0;
//
// MC_CMD_WORKAROUND
// Enable/Disable a given workaround. The mcfw will return EINVAL if it doesn't
// understand the given workaround number - which should not be treated as a
// hard error by client code. This op does not imply any semantics about each
// workaround, that's between the driver and the mcfw on a per-workaround
// basis. Locks required: None. Returns: 0, EINVAL .
//
pub const MC_CMD_WORKAROUND: c_uint = 0x4a;

// MC_CMD_WORKAROUND_IN msgrequest
pub const MC_CMD_WORKAROUND_IN_LEN: c_int = 8;
// The enums here must correspond with those in MC_CMD_GET_WORKAROUND.
pub const MC_CMD_WORKAROUND_IN_TYPE_OFST: c_int = 0;
pub const MC_CMD_WORKAROUND_IN_TYPE_LEN: c_int = 4;
// enum: Bug 17230 work around.
pub const MC_CMD_WORKAROUND_BUG17230: c_uint = 0x1;
// enum: Bug 35388 work around (unsafe EVQ writes).
pub const MC_CMD_WORKAROUND_BUG35388: c_uint = 0x2;
// enum: Bug35017 workaround (A64 tables must be identity map)
pub const MC_CMD_WORKAROUND_BUG35017: c_uint = 0x3;
// enum: Bug 41750 present (MC_CMD_TRIGGER_INTERRUPT won't work)
pub const MC_CMD_WORKAROUND_BUG41750: c_uint = 0x4;
// enum: Bug 42008 present (Interrupts can overtake associated events). Caution
// - before adding code that queries this workaround, remember that there's
// released Monza firmware that doesn't understand MC_CMD_WORKAROUND_BUG42008,
// and will hence (incorrectly) report that the bug doesn't exist.
//
pub const MC_CMD_WORKAROUND_BUG42008: c_uint = 0x5;
// enum: Bug 26807 features present in firmware (multicast filter chaining)
// This feature cannot be turned on/off while there are any filters already
// present. The behaviour in such case depends on the acting client's privilege
// level. If the client has the admin privilege, then all functions that have
// filters installed will be FLRed and the FLR_DONE flag will be set. Otherwise
// the command will fail with MC_CMD_ERR_FILTERS_PRESENT.
//
pub const MC_CMD_WORKAROUND_BUG26807: c_uint = 0x6;
// enum: Bug 61265 work around (broken EVQ TMR writes).
pub const MC_CMD_WORKAROUND_BUG61265: c_uint = 0x7;
// 0 = disable the workaround indicated by TYPE; any non-zero value = enable
// the workaround
//
pub const MC_CMD_WORKAROUND_IN_ENABLED_OFST: c_int = 4;
pub const MC_CMD_WORKAROUND_IN_ENABLED_LEN: c_int = 4;
// MC_CMD_WORKAROUND_OUT msgresponse
pub const MC_CMD_WORKAROUND_OUT_LEN: c_int = 0;
// MC_CMD_WORKAROUND_EXT_OUT msgresponse: This response format will be used
// when (TYPE == MC_CMD_WORKAROUND_BUG26807)
//
pub const MC_CMD_WORKAROUND_EXT_OUT_LEN: c_int = 4;
pub const MC_CMD_WORKAROUND_EXT_OUT_FLAGS_OFST: c_int = 0;
pub const MC_CMD_WORKAROUND_EXT_OUT_FLAGS_LEN: c_int = 4;
pub const MC_CMD_WORKAROUND_EXT_OUT_FLR_DONE_OFST: c_int = 0;
pub const MC_CMD_WORKAROUND_EXT_OUT_FLR_DONE_LBN: c_int = 0;
pub const MC_CMD_WORKAROUND_EXT_OUT_FLR_DONE_WIDTH: c_int = 1;
//
// MC_CMD_GET_PHY_MEDIA_INFO
// Read media-specific data from PHY (e.g. SFP/SFP+ module ID information for
// SFP+ PHYs). The 'media type' can be found via GET_PHY_CFG
// (GET_PHY_CFG_OUT_MEDIA_TYPE); the valid 'page number' input values, and the
// output data, are interpreted on a per-type basis. For SFP+: PAGE=0 or 1
// returns a 128-byte block read from module I2C address 0xA0 offset 0 or 0x80.
// Anything else: currently undefined. Locks required: None. Return code: 0.
//
pub const MC_CMD_GET_PHY_MEDIA_INFO: c_uint = 0x4b;

// MC_CMD_GET_PHY_MEDIA_INFO_IN msgrequest
pub const MC_CMD_GET_PHY_MEDIA_INFO_IN_LEN: c_int = 4;
pub const MC_CMD_GET_PHY_MEDIA_INFO_IN_PAGE_OFST: c_int = 0;
pub const MC_CMD_GET_PHY_MEDIA_INFO_IN_PAGE_LEN: c_int = 4;
// MC_CMD_GET_PHY_MEDIA_INFO_OUT msgresponse
pub const MC_CMD_GET_PHY_MEDIA_INFO_OUT_LENMIN: c_int = 5;
pub const MC_CMD_GET_PHY_MEDIA_INFO_OUT_LENMAX: c_int = 252;
pub const MC_CMD_GET_PHY_MEDIA_INFO_OUT_LENMAX_MCDI2: c_int = 1020;

// in bytes
pub const MC_CMD_GET_PHY_MEDIA_INFO_OUT_DATALEN_OFST: c_int = 0;
pub const MC_CMD_GET_PHY_MEDIA_INFO_OUT_DATALEN_LEN: c_int = 4;
pub const MC_CMD_GET_PHY_MEDIA_INFO_OUT_DATA_OFST: c_int = 4;
pub const MC_CMD_GET_PHY_MEDIA_INFO_OUT_DATA_LEN: c_int = 1;
pub const MC_CMD_GET_PHY_MEDIA_INFO_OUT_DATA_MINNUM: c_int = 1;
pub const MC_CMD_GET_PHY_MEDIA_INFO_OUT_DATA_MAXNUM: c_int = 248;
pub const MC_CMD_GET_PHY_MEDIA_INFO_OUT_DATA_MAXNUM_MCDI2: c_int = 1016;
//
// MC_CMD_NVRAM_TEST
// Test a particular NVRAM partition for valid contents (where "valid" depends
// on the type of partition).
//
pub const MC_CMD_NVRAM_TEST: c_uint = 0x4c;

// MC_CMD_NVRAM_TEST_IN msgrequest
pub const MC_CMD_NVRAM_TEST_IN_LEN: c_int = 4;
pub const MC_CMD_NVRAM_TEST_IN_TYPE_OFST: c_int = 0;
pub const MC_CMD_NVRAM_TEST_IN_TYPE_LEN: c_int = 4;
// Enum values, see field(s):
// MC_CMD_NVRAM_TYPES/MC_CMD_NVRAM_TYPES_OUT/TYPES
// MC_CMD_NVRAM_TEST_OUT msgresponse
pub const MC_CMD_NVRAM_TEST_OUT_LEN: c_int = 4;
pub const MC_CMD_NVRAM_TEST_OUT_RESULT_OFST: c_int = 0;
pub const MC_CMD_NVRAM_TEST_OUT_RESULT_LEN: c_int = 4;
// enum: Passed.
pub const MC_CMD_NVRAM_TEST_PASS: c_uint = 0x0;
// enum: Failed.
pub const MC_CMD_NVRAM_TEST_FAIL: c_uint = 0x1;
// enum: Not supported.
pub const MC_CMD_NVRAM_TEST_NOTSUPP: c_uint = 0x2;
//
// MC_CMD_MRSFP_TWEAK
// Read status and/or set parameters for the 'mrsfp' driver in mr_rusty builds.
// I2C I/O expander bits are always read; if equaliser parameters are supplied,
// they are configured first. Locks required: None. Return code: 0, EINVAL.
//
pub const MC_CMD_MRSFP_TWEAK: c_uint = 0x4d;
// MC_CMD_MRSFP_TWEAK_IN_EQ_CONFIG msgrequest
pub const MC_CMD_MRSFP_TWEAK_IN_EQ_CONFIG_LEN: c_int = 16;
// 0-6 low->high de-emph.
pub const MC_CMD_MRSFP_TWEAK_IN_EQ_CONFIG_TXEQ_LEVEL_OFST: c_int = 0;
pub const MC_CMD_MRSFP_TWEAK_IN_EQ_CONFIG_TXEQ_LEVEL_LEN: c_int = 4;
// 0-8 low->high ref.V
pub const MC_CMD_MRSFP_TWEAK_IN_EQ_CONFIG_TXEQ_DT_CFG_OFST: c_int = 4;
pub const MC_CMD_MRSFP_TWEAK_IN_EQ_CONFIG_TXEQ_DT_CFG_LEN: c_int = 4;
// 0-8 0-8 low->high boost
pub const MC_CMD_MRSFP_TWEAK_IN_EQ_CONFIG_RXEQ_BOOST_OFST: c_int = 8;
pub const MC_CMD_MRSFP_TWEAK_IN_EQ_CONFIG_RXEQ_BOOST_LEN: c_int = 4;
// 0-8 low->high ref.V
pub const MC_CMD_MRSFP_TWEAK_IN_EQ_CONFIG_RXEQ_DT_CFG_OFST: c_int = 12;
pub const MC_CMD_MRSFP_TWEAK_IN_EQ_CONFIG_RXEQ_DT_CFG_LEN: c_int = 4;
// MC_CMD_MRSFP_TWEAK_IN_READ_ONLY msgrequest
pub const MC_CMD_MRSFP_TWEAK_IN_READ_ONLY_LEN: c_int = 0;
// MC_CMD_MRSFP_TWEAK_OUT msgresponse
pub const MC_CMD_MRSFP_TWEAK_OUT_LEN: c_int = 12;
// input bits
pub const MC_CMD_MRSFP_TWEAK_OUT_IOEXP_INPUTS_OFST: c_int = 0;
pub const MC_CMD_MRSFP_TWEAK_OUT_IOEXP_INPUTS_LEN: c_int = 4;
// output bits
pub const MC_CMD_MRSFP_TWEAK_OUT_IOEXP_OUTPUTS_OFST: c_int = 4;
pub const MC_CMD_MRSFP_TWEAK_OUT_IOEXP_OUTPUTS_LEN: c_int = 4;
// direction
pub const MC_CMD_MRSFP_TWEAK_OUT_IOEXP_DIRECTION_OFST: c_int = 8;
pub const MC_CMD_MRSFP_TWEAK_OUT_IOEXP_DIRECTION_LEN: c_int = 4;
// enum: Out.
pub const MC_CMD_MRSFP_TWEAK_OUT_IOEXP_DIRECTION_OUT: c_uint = 0x0;
// enum: In.
pub const MC_CMD_MRSFP_TWEAK_OUT_IOEXP_DIRECTION_IN: c_uint = 0x1;
//
// MC_CMD_SENSOR_SET_LIMS
// Adjusts the sensor limits. This is a warranty-voiding operation. Returns:
// ENOENT if the sensor specified does not exist, EINVAL if the limits are out
// of range.
//
pub const MC_CMD_SENSOR_SET_LIMS: c_uint = 0x4e;

// MC_CMD_SENSOR_SET_LIMS_IN msgrequest
pub const MC_CMD_SENSOR_SET_LIMS_IN_LEN: c_int = 20;
pub const MC_CMD_SENSOR_SET_LIMS_IN_SENSOR_OFST: c_int = 0;
pub const MC_CMD_SENSOR_SET_LIMS_IN_SENSOR_LEN: c_int = 4;
// Enum values, see field(s):
// MC_CMD_SENSOR_INFO/MC_CMD_SENSOR_INFO_OUT/MASK
// interpretation is sensor-specific.
pub const MC_CMD_SENSOR_SET_LIMS_IN_LOW0_OFST: c_int = 4;
pub const MC_CMD_SENSOR_SET_LIMS_IN_LOW0_LEN: c_int = 4;
// interpretation is sensor-specific.
pub const MC_CMD_SENSOR_SET_LIMS_IN_HI0_OFST: c_int = 8;
pub const MC_CMD_SENSOR_SET_LIMS_IN_HI0_LEN: c_int = 4;
// interpretation is sensor-specific.
pub const MC_CMD_SENSOR_SET_LIMS_IN_LOW1_OFST: c_int = 12;
pub const MC_CMD_SENSOR_SET_LIMS_IN_LOW1_LEN: c_int = 4;
// interpretation is sensor-specific.
pub const MC_CMD_SENSOR_SET_LIMS_IN_HI1_OFST: c_int = 16;
pub const MC_CMD_SENSOR_SET_LIMS_IN_HI1_LEN: c_int = 4;
// MC_CMD_SENSOR_SET_LIMS_OUT msgresponse
pub const MC_CMD_SENSOR_SET_LIMS_OUT_LEN: c_int = 0;
//
// MC_CMD_GET_RESOURCE_LIMITS
//
pub const MC_CMD_GET_RESOURCE_LIMITS: c_uint = 0x4f;
// MC_CMD_GET_RESOURCE_LIMITS_IN msgrequest
pub const MC_CMD_GET_RESOURCE_LIMITS_IN_LEN: c_int = 0;
// MC_CMD_GET_RESOURCE_LIMITS_OUT msgresponse
pub const MC_CMD_GET_RESOURCE_LIMITS_OUT_LEN: c_int = 16;
pub const MC_CMD_GET_RESOURCE_LIMITS_OUT_BUFTBL_OFST: c_int = 0;
pub const MC_CMD_GET_RESOURCE_LIMITS_OUT_BUFTBL_LEN: c_int = 4;
pub const MC_CMD_GET_RESOURCE_LIMITS_OUT_EVQ_OFST: c_int = 4;
pub const MC_CMD_GET_RESOURCE_LIMITS_OUT_EVQ_LEN: c_int = 4;
pub const MC_CMD_GET_RESOURCE_LIMITS_OUT_RXQ_OFST: c_int = 8;
pub const MC_CMD_GET_RESOURCE_LIMITS_OUT_RXQ_LEN: c_int = 4;
pub const MC_CMD_GET_RESOURCE_LIMITS_OUT_TXQ_OFST: c_int = 12;
pub const MC_CMD_GET_RESOURCE_LIMITS_OUT_TXQ_LEN: c_int = 4;
//
// MC_CMD_NVRAM_PARTITIONS
// Reads the list of available virtual NVRAM partition types. Locks required:
// none. Returns: 0, EINVAL (bad type).
//
pub const MC_CMD_NVRAM_PARTITIONS: c_uint = 0x51;

// MC_CMD_NVRAM_PARTITIONS_IN msgrequest
pub const MC_CMD_NVRAM_PARTITIONS_IN_LEN: c_int = 0;
// MC_CMD_NVRAM_PARTITIONS_OUT msgresponse
pub const MC_CMD_NVRAM_PARTITIONS_OUT_LENMIN: c_int = 4;
pub const MC_CMD_NVRAM_PARTITIONS_OUT_LENMAX: c_int = 252;
pub const MC_CMD_NVRAM_PARTITIONS_OUT_LENMAX_MCDI2: c_int = 1020;

// total number of partitions
pub const MC_CMD_NVRAM_PARTITIONS_OUT_NUM_PARTITIONS_OFST: c_int = 0;
pub const MC_CMD_NVRAM_PARTITIONS_OUT_NUM_PARTITIONS_LEN: c_int = 4;
// type ID code for each of NUM_PARTITIONS partitions
pub const MC_CMD_NVRAM_PARTITIONS_OUT_TYPE_ID_OFST: c_int = 4;
pub const MC_CMD_NVRAM_PARTITIONS_OUT_TYPE_ID_LEN: c_int = 4;
pub const MC_CMD_NVRAM_PARTITIONS_OUT_TYPE_ID_MINNUM: c_int = 0;
pub const MC_CMD_NVRAM_PARTITIONS_OUT_TYPE_ID_MAXNUM: c_int = 62;
pub const MC_CMD_NVRAM_PARTITIONS_OUT_TYPE_ID_MAXNUM_MCDI2: c_int = 254;
//
// MC_CMD_NVRAM_METADATA
// Reads soft metadata for a virtual NVRAM partition type. Locks required:
// none. Returns: 0, EINVAL (bad type).
//
pub const MC_CMD_NVRAM_METADATA: c_uint = 0x52;

// MC_CMD_NVRAM_METADATA_IN msgrequest
pub const MC_CMD_NVRAM_METADATA_IN_LEN: c_int = 4;
// Partition type ID code
pub const MC_CMD_NVRAM_METADATA_IN_TYPE_OFST: c_int = 0;
pub const MC_CMD_NVRAM_METADATA_IN_TYPE_LEN: c_int = 4;
// MC_CMD_NVRAM_METADATA_OUT msgresponse
pub const MC_CMD_NVRAM_METADATA_OUT_LENMIN: c_int = 20;
pub const MC_CMD_NVRAM_METADATA_OUT_LENMAX: c_int = 252;
pub const MC_CMD_NVRAM_METADATA_OUT_LENMAX_MCDI2: c_int = 1020;

// Partition type ID code
pub const MC_CMD_NVRAM_METADATA_OUT_TYPE_OFST: c_int = 0;
pub const MC_CMD_NVRAM_METADATA_OUT_TYPE_LEN: c_int = 4;
pub const MC_CMD_NVRAM_METADATA_OUT_FLAGS_OFST: c_int = 4;
pub const MC_CMD_NVRAM_METADATA_OUT_FLAGS_LEN: c_int = 4;
pub const MC_CMD_NVRAM_METADATA_OUT_SUBTYPE_VALID_OFST: c_int = 4;
pub const MC_CMD_NVRAM_METADATA_OUT_SUBTYPE_VALID_LBN: c_int = 0;
pub const MC_CMD_NVRAM_METADATA_OUT_SUBTYPE_VALID_WIDTH: c_int = 1;
pub const MC_CMD_NVRAM_METADATA_OUT_VERSION_VALID_OFST: c_int = 4;
pub const MC_CMD_NVRAM_METADATA_OUT_VERSION_VALID_LBN: c_int = 1;
pub const MC_CMD_NVRAM_METADATA_OUT_VERSION_VALID_WIDTH: c_int = 1;
pub const MC_CMD_NVRAM_METADATA_OUT_DESCRIPTION_VALID_OFST: c_int = 4;
pub const MC_CMD_NVRAM_METADATA_OUT_DESCRIPTION_VALID_LBN: c_int = 2;
pub const MC_CMD_NVRAM_METADATA_OUT_DESCRIPTION_VALID_WIDTH: c_int = 1;
// Subtype ID code for content of this partition
pub const MC_CMD_NVRAM_METADATA_OUT_SUBTYPE_OFST: c_int = 8;
pub const MC_CMD_NVRAM_METADATA_OUT_SUBTYPE_LEN: c_int = 4;
// 1st component of W.X.Y.Z version number for content of this partition
pub const MC_CMD_NVRAM_METADATA_OUT_VERSION_W_OFST: c_int = 12;
pub const MC_CMD_NVRAM_METADATA_OUT_VERSION_W_LEN: c_int = 2;
// 2nd component of W.X.Y.Z version number for content of this partition
pub const MC_CMD_NVRAM_METADATA_OUT_VERSION_X_OFST: c_int = 14;
pub const MC_CMD_NVRAM_METADATA_OUT_VERSION_X_LEN: c_int = 2;
// 3rd component of W.X.Y.Z version number for content of this partition
pub const MC_CMD_NVRAM_METADATA_OUT_VERSION_Y_OFST: c_int = 16;
pub const MC_CMD_NVRAM_METADATA_OUT_VERSION_Y_LEN: c_int = 2;
// 4th component of W.X.Y.Z version number for content of this partition
pub const MC_CMD_NVRAM_METADATA_OUT_VERSION_Z_OFST: c_int = 18;
pub const MC_CMD_NVRAM_METADATA_OUT_VERSION_Z_LEN: c_int = 2;
// Zero-terminated string describing the content of this partition
pub const MC_CMD_NVRAM_METADATA_OUT_DESCRIPTION_OFST: c_int = 20;
pub const MC_CMD_NVRAM_METADATA_OUT_DESCRIPTION_LEN: c_int = 1;
pub const MC_CMD_NVRAM_METADATA_OUT_DESCRIPTION_MINNUM: c_int = 0;
pub const MC_CMD_NVRAM_METADATA_OUT_DESCRIPTION_MAXNUM: c_int = 232;
pub const MC_CMD_NVRAM_METADATA_OUT_DESCRIPTION_MAXNUM_MCDI2: c_int = 1000;
//
// MC_CMD_GET_MAC_ADDRESSES
// Returns the base MAC, count and stride for the requesting function
//
pub const MC_CMD_GET_MAC_ADDRESSES: c_uint = 0x55;

// MC_CMD_GET_MAC_ADDRESSES_IN msgrequest
pub const MC_CMD_GET_MAC_ADDRESSES_IN_LEN: c_int = 0;
// MC_CMD_GET_MAC_ADDRESSES_OUT msgresponse
pub const MC_CMD_GET_MAC_ADDRESSES_OUT_LEN: c_int = 16;
// Base MAC address
pub const MC_CMD_GET_MAC_ADDRESSES_OUT_MAC_ADDR_BASE_OFST: c_int = 0;
pub const MC_CMD_GET_MAC_ADDRESSES_OUT_MAC_ADDR_BASE_LEN: c_int = 6;
// Padding
pub const MC_CMD_GET_MAC_ADDRESSES_OUT_RESERVED_OFST: c_int = 6;
pub const MC_CMD_GET_MAC_ADDRESSES_OUT_RESERVED_LEN: c_int = 2;
// Number of allocated MAC addresses
pub const MC_CMD_GET_MAC_ADDRESSES_OUT_MAC_COUNT_OFST: c_int = 8;
pub const MC_CMD_GET_MAC_ADDRESSES_OUT_MAC_COUNT_LEN: c_int = 4;
// Spacing of allocated MAC addresses
pub const MC_CMD_GET_MAC_ADDRESSES_OUT_MAC_STRIDE_OFST: c_int = 12;
pub const MC_CMD_GET_MAC_ADDRESSES_OUT_MAC_STRIDE_LEN: c_int = 4;
//
// MC_CMD_CLP
// Perform a CLP related operation, see SF-110495-PS for details of CLP
// processing. This command has been extended to accomodate the requirements of
// different manufacturers which are to be found in SF-119187-TC, SF-119186-TC,
// SF-120509-TC and SF-117282-PS.
//
pub const MC_CMD_CLP: c_uint = 0x56;

// MC_CMD_CLP_IN msgrequest
pub const MC_CMD_CLP_IN_LEN: c_int = 4;
// Sub operation
pub const MC_CMD_CLP_IN_OP_OFST: c_int = 0;
pub const MC_CMD_CLP_IN_OP_LEN: c_int = 4;
// enum: Return to factory default settings
pub const MC_CMD_CLP_OP_DEFAULT: c_uint = 0x1;
// enum: Set MAC address
pub const MC_CMD_CLP_OP_SET_MAC: c_uint = 0x2;
// enum: Get MAC address
pub const MC_CMD_CLP_OP_GET_MAC: c_uint = 0x3;
// enum: Set UEFI/GPXE boot mode
pub const MC_CMD_CLP_OP_SET_BOOT: c_uint = 0x4;
// enum: Get UEFI/GPXE boot mode
pub const MC_CMD_CLP_OP_GET_BOOT: c_uint = 0x5;
// MC_CMD_CLP_OUT msgresponse
pub const MC_CMD_CLP_OUT_LEN: c_int = 0;
// MC_CMD_CLP_IN_DEFAULT msgrequest
pub const MC_CMD_CLP_IN_DEFAULT_LEN: c_int = 4;
// MC_CMD_CLP_IN_OP_OFST 0
// MC_CMD_CLP_IN_OP_LEN 4
// MC_CMD_CLP_OUT_DEFAULT msgresponse
pub const MC_CMD_CLP_OUT_DEFAULT_LEN: c_int = 0;
// MC_CMD_CLP_IN_SET_MAC msgrequest
pub const MC_CMD_CLP_IN_SET_MAC_LEN: c_int = 12;
// MC_CMD_CLP_IN_OP_OFST 0
// MC_CMD_CLP_IN_OP_LEN 4
// The MAC address assigned to port. A zero MAC address of 00:00:00:00:00:00
// restores the permanent (factory-programmed) MAC address associated with the
// port. A non-zero MAC address persists until a PCIe reset or a power cycle.
//
pub const MC_CMD_CLP_IN_SET_MAC_ADDR_OFST: c_int = 4;
pub const MC_CMD_CLP_IN_SET_MAC_ADDR_LEN: c_int = 6;
// Padding
pub const MC_CMD_CLP_IN_SET_MAC_RESERVED_OFST: c_int = 10;
pub const MC_CMD_CLP_IN_SET_MAC_RESERVED_LEN: c_int = 2;
// MC_CMD_CLP_OUT_SET_MAC msgresponse
pub const MC_CMD_CLP_OUT_SET_MAC_LEN: c_int = 0;
// MC_CMD_CLP_IN_SET_MAC_V2 msgrequest
pub const MC_CMD_CLP_IN_SET_MAC_V2_LEN: c_int = 16;
// MC_CMD_CLP_IN_OP_OFST 0
// MC_CMD_CLP_IN_OP_LEN 4
// The MAC address assigned to port. A zero MAC address of 00:00:00:00:00:00
// restores the permanent (factory-programmed) MAC address associated with the
// port. A non-zero MAC address persists until a PCIe reset or a power cycle.
//
pub const MC_CMD_CLP_IN_SET_MAC_V2_ADDR_OFST: c_int = 4;
pub const MC_CMD_CLP_IN_SET_MAC_V2_ADDR_LEN: c_int = 6;
// Padding
pub const MC_CMD_CLP_IN_SET_MAC_V2_RESERVED_OFST: c_int = 10;
pub const MC_CMD_CLP_IN_SET_MAC_V2_RESERVED_LEN: c_int = 2;
pub const MC_CMD_CLP_IN_SET_MAC_V2_FLAGS_OFST: c_int = 12;
pub const MC_CMD_CLP_IN_SET_MAC_V2_FLAGS_LEN: c_int = 4;
pub const MC_CMD_CLP_IN_SET_MAC_V2_VIRTUAL_OFST: c_int = 12;
pub const MC_CMD_CLP_IN_SET_MAC_V2_VIRTUAL_LBN: c_int = 0;
pub const MC_CMD_CLP_IN_SET_MAC_V2_VIRTUAL_WIDTH: c_int = 1;
// MC_CMD_CLP_IN_GET_MAC msgrequest
pub const MC_CMD_CLP_IN_GET_MAC_LEN: c_int = 4;
// MC_CMD_CLP_IN_OP_OFST 0
// MC_CMD_CLP_IN_OP_LEN 4
// MC_CMD_CLP_IN_GET_MAC_V2 msgrequest
pub const MC_CMD_CLP_IN_GET_MAC_V2_LEN: c_int = 8;
// MC_CMD_CLP_IN_OP_OFST 0
// MC_CMD_CLP_IN_OP_LEN 4
pub const MC_CMD_CLP_IN_GET_MAC_V2_FLAGS_OFST: c_int = 4;
pub const MC_CMD_CLP_IN_GET_MAC_V2_FLAGS_LEN: c_int = 4;
pub const MC_CMD_CLP_IN_GET_MAC_V2_PERMANENT_OFST: c_int = 4;
pub const MC_CMD_CLP_IN_GET_MAC_V2_PERMANENT_LBN: c_int = 0;
pub const MC_CMD_CLP_IN_GET_MAC_V2_PERMANENT_WIDTH: c_int = 1;
// MC_CMD_CLP_OUT_GET_MAC msgresponse
pub const MC_CMD_CLP_OUT_GET_MAC_LEN: c_int = 8;
// MAC address assigned to port
pub const MC_CMD_CLP_OUT_GET_MAC_ADDR_OFST: c_int = 0;
pub const MC_CMD_CLP_OUT_GET_MAC_ADDR_LEN: c_int = 6;
// Padding
pub const MC_CMD_CLP_OUT_GET_MAC_RESERVED_OFST: c_int = 6;
pub const MC_CMD_CLP_OUT_GET_MAC_RESERVED_LEN: c_int = 2;
// MC_CMD_CLP_IN_SET_BOOT msgrequest
pub const MC_CMD_CLP_IN_SET_BOOT_LEN: c_int = 5;
// MC_CMD_CLP_IN_OP_OFST 0
// MC_CMD_CLP_IN_OP_LEN 4
// Boot flag
pub const MC_CMD_CLP_IN_SET_BOOT_FLAG_OFST: c_int = 4;
pub const MC_CMD_CLP_IN_SET_BOOT_FLAG_LEN: c_int = 1;
// MC_CMD_CLP_OUT_SET_BOOT msgresponse
pub const MC_CMD_CLP_OUT_SET_BOOT_LEN: c_int = 0;
// MC_CMD_CLP_IN_GET_BOOT msgrequest
pub const MC_CMD_CLP_IN_GET_BOOT_LEN: c_int = 4;
// MC_CMD_CLP_IN_OP_OFST 0
// MC_CMD_CLP_IN_OP_LEN 4
// MC_CMD_CLP_OUT_GET_BOOT msgresponse
pub const MC_CMD_CLP_OUT_GET_BOOT_LEN: c_int = 4;
// Boot flag
pub const MC_CMD_CLP_OUT_GET_BOOT_FLAG_OFST: c_int = 0;
pub const MC_CMD_CLP_OUT_GET_BOOT_FLAG_LEN: c_int = 1;
// Padding
pub const MC_CMD_CLP_OUT_GET_BOOT_RESERVED_OFST: c_int = 1;
pub const MC_CMD_CLP_OUT_GET_BOOT_RESERVED_LEN: c_int = 3;
//
// MC_CMD_MUM
// Perform a MUM operation
//
pub const MC_CMD_MUM: c_uint = 0x57;

// MC_CMD_MUM_IN msgrequest
pub const MC_CMD_MUM_IN_LEN: c_int = 4;
pub const MC_CMD_MUM_IN_OP_HDR_OFST: c_int = 0;
pub const MC_CMD_MUM_IN_OP_HDR_LEN: c_int = 4;
pub const MC_CMD_MUM_IN_OP_OFST: c_int = 0;
pub const MC_CMD_MUM_IN_OP_LBN: c_int = 0;
pub const MC_CMD_MUM_IN_OP_WIDTH: c_int = 8;
// enum: NULL MCDI command to MUM
pub const MC_CMD_MUM_OP_NULL: c_uint = 0x1;
// enum: Get MUM version
pub const MC_CMD_MUM_OP_GET_VERSION: c_uint = 0x2;
// enum: Issue raw I2C command to MUM
pub const MC_CMD_MUM_OP_RAW_CMD: c_uint = 0x3;
// enum: Read from registers on devices connected to MUM.
pub const MC_CMD_MUM_OP_READ: c_uint = 0x4;
// enum: Write to registers on devices connected to MUM.
pub const MC_CMD_MUM_OP_WRITE: c_uint = 0x5;
// enum: Control UART logging.
pub const MC_CMD_MUM_OP_LOG: c_uint = 0x6;
// enum: Operations on MUM GPIO lines
pub const MC_CMD_MUM_OP_GPIO: c_uint = 0x7;
// enum: Get sensor readings from MUM
pub const MC_CMD_MUM_OP_READ_SENSORS: c_uint = 0x8;
// enum: Initiate clock programming on the MUM
pub const MC_CMD_MUM_OP_PROGRAM_CLOCKS: c_uint = 0x9;
// enum: Initiate FPGA load from flash on the MUM
pub const MC_CMD_MUM_OP_FPGA_LOAD: c_uint = 0xa;
// enum: Request sensor reading from MUM ADC resulting from earlier request via
// MUM ATB
//
pub const MC_CMD_MUM_OP_READ_ATB_SENSOR: c_uint = 0xb;
// enum: Send commands relating to the QSFP ports via the MUM for PHY
// operations
//
pub const MC_CMD_MUM_OP_QSFP: c_uint = 0xc;
// enum: Request discrete and SODIMM DDR info (type, size, speed grade, voltage
// level) from MUM
//
pub const MC_CMD_MUM_OP_READ_DDR_INFO: c_uint = 0xd;
// MC_CMD_MUM_IN_NULL msgrequest
pub const MC_CMD_MUM_IN_NULL_LEN: c_int = 4;
// MUM cmd header
pub const MC_CMD_MUM_IN_CMD_OFST: c_int = 0;
pub const MC_CMD_MUM_IN_CMD_LEN: c_int = 4;
// MC_CMD_MUM_IN_GET_VERSION msgrequest
pub const MC_CMD_MUM_IN_GET_VERSION_LEN: c_int = 4;
// MUM cmd header
// MC_CMD_MUM_IN_CMD_OFST 0
// MC_CMD_MUM_IN_CMD_LEN 4
// MC_CMD_MUM_IN_READ msgrequest
pub const MC_CMD_MUM_IN_READ_LEN: c_int = 16;
// MUM cmd header
// MC_CMD_MUM_IN_CMD_OFST 0
// MC_CMD_MUM_IN_CMD_LEN 4
// ID of (device connected to MUM) to read from registers of
pub const MC_CMD_MUM_IN_READ_DEVICE_OFST: c_int = 4;
pub const MC_CMD_MUM_IN_READ_DEVICE_LEN: c_int = 4;
// enum: Hittite HMC1035 clock generator on Sorrento board
pub const MC_CMD_MUM_DEV_HITTITE: c_uint = 0x1;
// enum: Hittite HMC1035 clock generator for NIC-side on Sorrento board
pub const MC_CMD_MUM_DEV_HITTITE_NIC: c_uint = 0x2;
// 32-bit address to read from
pub const MC_CMD_MUM_IN_READ_ADDR_OFST: c_int = 8;
pub const MC_CMD_MUM_IN_READ_ADDR_LEN: c_int = 4;
// Number of words to read.
pub const MC_CMD_MUM_IN_READ_NUMWORDS_OFST: c_int = 12;
pub const MC_CMD_MUM_IN_READ_NUMWORDS_LEN: c_int = 4;
// MC_CMD_MUM_IN_WRITE msgrequest
pub const MC_CMD_MUM_IN_WRITE_LENMIN: c_int = 16;
pub const MC_CMD_MUM_IN_WRITE_LENMAX: c_int = 252;
pub const MC_CMD_MUM_IN_WRITE_LENMAX_MCDI2: c_int = 1020;

// MUM cmd header
// MC_CMD_MUM_IN_CMD_OFST 0
// MC_CMD_MUM_IN_CMD_LEN 4
// ID of (device connected to MUM) to write to registers of
pub const MC_CMD_MUM_IN_WRITE_DEVICE_OFST: c_int = 4;
pub const MC_CMD_MUM_IN_WRITE_DEVICE_LEN: c_int = 4;
// enum: Hittite HMC1035 clock generator on Sorrento board
// MC_CMD_MUM_DEV_HITTITE 0x1
// 32-bit address to write to
pub const MC_CMD_MUM_IN_WRITE_ADDR_OFST: c_int = 8;
pub const MC_CMD_MUM_IN_WRITE_ADDR_LEN: c_int = 4;
// Words to write
pub const MC_CMD_MUM_IN_WRITE_BUFFER_OFST: c_int = 12;
pub const MC_CMD_MUM_IN_WRITE_BUFFER_LEN: c_int = 4;
pub const MC_CMD_MUM_IN_WRITE_BUFFER_MINNUM: c_int = 1;
pub const MC_CMD_MUM_IN_WRITE_BUFFER_MAXNUM: c_int = 60;
pub const MC_CMD_MUM_IN_WRITE_BUFFER_MAXNUM_MCDI2: c_int = 252;
// MC_CMD_MUM_IN_RAW_CMD msgrequest
pub const MC_CMD_MUM_IN_RAW_CMD_LENMIN: c_int = 17;
pub const MC_CMD_MUM_IN_RAW_CMD_LENMAX: c_int = 252;
pub const MC_CMD_MUM_IN_RAW_CMD_LENMAX_MCDI2: c_int = 1020;

// MUM cmd header
// MC_CMD_MUM_IN_CMD_OFST 0
// MC_CMD_MUM_IN_CMD_LEN 4
// MUM I2C cmd code
pub const MC_CMD_MUM_IN_RAW_CMD_CMD_CODE_OFST: c_int = 4;
pub const MC_CMD_MUM_IN_RAW_CMD_CMD_CODE_LEN: c_int = 4;
// Number of bytes to write
pub const MC_CMD_MUM_IN_RAW_CMD_NUM_WRITE_OFST: c_int = 8;
pub const MC_CMD_MUM_IN_RAW_CMD_NUM_WRITE_LEN: c_int = 4;
// Number of bytes to read
pub const MC_CMD_MUM_IN_RAW_CMD_NUM_READ_OFST: c_int = 12;
pub const MC_CMD_MUM_IN_RAW_CMD_NUM_READ_LEN: c_int = 4;
// Bytes to write
pub const MC_CMD_MUM_IN_RAW_CMD_WRITE_DATA_OFST: c_int = 16;
pub const MC_CMD_MUM_IN_RAW_CMD_WRITE_DATA_LEN: c_int = 1;
pub const MC_CMD_MUM_IN_RAW_CMD_WRITE_DATA_MINNUM: c_int = 1;
pub const MC_CMD_MUM_IN_RAW_CMD_WRITE_DATA_MAXNUM: c_int = 236;
pub const MC_CMD_MUM_IN_RAW_CMD_WRITE_DATA_MAXNUM_MCDI2: c_int = 1004;
// MC_CMD_MUM_IN_LOG msgrequest
pub const MC_CMD_MUM_IN_LOG_LEN: c_int = 8;
// MUM cmd header
// MC_CMD_MUM_IN_CMD_OFST 0
// MC_CMD_MUM_IN_CMD_LEN 4
pub const MC_CMD_MUM_IN_LOG_OP_OFST: c_int = 4;
pub const MC_CMD_MUM_IN_LOG_OP_LEN: c_int = 4;
pub const MC_CMD_MUM_IN_LOG_OP_UART: c_uint = 0x1 /* enum */;
// MC_CMD_MUM_IN_LOG_OP_UART msgrequest
pub const MC_CMD_MUM_IN_LOG_OP_UART_LEN: c_int = 12;
// MC_CMD_MUM_IN_CMD_OFST 0
// MC_CMD_MUM_IN_CMD_LEN 4
// MC_CMD_MUM_IN_LOG_OP_OFST 4
// MC_CMD_MUM_IN_LOG_OP_LEN 4
// Enable/disable debug output to UART
pub const MC_CMD_MUM_IN_LOG_OP_UART_ENABLE_OFST: c_int = 8;
pub const MC_CMD_MUM_IN_LOG_OP_UART_ENABLE_LEN: c_int = 4;
// MC_CMD_MUM_IN_GPIO msgrequest
pub const MC_CMD_MUM_IN_GPIO_LEN: c_int = 8;
// MUM cmd header
// MC_CMD_MUM_IN_CMD_OFST 0
// MC_CMD_MUM_IN_CMD_LEN 4
pub const MC_CMD_MUM_IN_GPIO_HDR_OFST: c_int = 4;
pub const MC_CMD_MUM_IN_GPIO_HDR_LEN: c_int = 4;
pub const MC_CMD_MUM_IN_GPIO_OPCODE_OFST: c_int = 4;
pub const MC_CMD_MUM_IN_GPIO_OPCODE_LBN: c_int = 0;
pub const MC_CMD_MUM_IN_GPIO_OPCODE_WIDTH: c_int = 8;
pub const MC_CMD_MUM_IN_GPIO_IN_READ: c_uint = 0x0 /* enum */;
pub const MC_CMD_MUM_IN_GPIO_OUT_WRITE: c_uint = 0x1 /* enum */;
pub const MC_CMD_MUM_IN_GPIO_OUT_READ: c_uint = 0x2 /* enum */;
pub const MC_CMD_MUM_IN_GPIO_OUT_ENABLE_WRITE: c_uint = 0x3 /* enum */;
pub const MC_CMD_MUM_IN_GPIO_OUT_ENABLE_READ: c_uint = 0x4 /* enum */;
pub const MC_CMD_MUM_IN_GPIO_OP: c_uint = 0x5 /* enum */;
// MC_CMD_MUM_IN_GPIO_IN_READ msgrequest
pub const MC_CMD_MUM_IN_GPIO_IN_READ_LEN: c_int = 8;
// MC_CMD_MUM_IN_CMD_OFST 0
// MC_CMD_MUM_IN_CMD_LEN 4
pub const MC_CMD_MUM_IN_GPIO_IN_READ_HDR_OFST: c_int = 4;
pub const MC_CMD_MUM_IN_GPIO_IN_READ_HDR_LEN: c_int = 4;
// MC_CMD_MUM_IN_GPIO_OUT_WRITE msgrequest
pub const MC_CMD_MUM_IN_GPIO_OUT_WRITE_LEN: c_int = 16;
// MC_CMD_MUM_IN_CMD_OFST 0
// MC_CMD_MUM_IN_CMD_LEN 4
pub const MC_CMD_MUM_IN_GPIO_OUT_WRITE_HDR_OFST: c_int = 4;
pub const MC_CMD_MUM_IN_GPIO_OUT_WRITE_HDR_LEN: c_int = 4;
// The first 32-bit word to be written to the GPIO OUT register.
pub const MC_CMD_MUM_IN_GPIO_OUT_WRITE_GPIOMASK1_OFST: c_int = 8;
pub const MC_CMD_MUM_IN_GPIO_OUT_WRITE_GPIOMASK1_LEN: c_int = 4;
// The second 32-bit word to be written to the GPIO OUT register.
pub const MC_CMD_MUM_IN_GPIO_OUT_WRITE_GPIOMASK2_OFST: c_int = 12;
pub const MC_CMD_MUM_IN_GPIO_OUT_WRITE_GPIOMASK2_LEN: c_int = 4;
// MC_CMD_MUM_IN_GPIO_OUT_READ msgrequest
pub const MC_CMD_MUM_IN_GPIO_OUT_READ_LEN: c_int = 8;
// MC_CMD_MUM_IN_CMD_OFST 0
// MC_CMD_MUM_IN_CMD_LEN 4
pub const MC_CMD_MUM_IN_GPIO_OUT_READ_HDR_OFST: c_int = 4;
pub const MC_CMD_MUM_IN_GPIO_OUT_READ_HDR_LEN: c_int = 4;
// MC_CMD_MUM_IN_GPIO_OUT_ENABLE_WRITE msgrequest
pub const MC_CMD_MUM_IN_GPIO_OUT_ENABLE_WRITE_LEN: c_int = 16;
// MC_CMD_MUM_IN_CMD_OFST 0
// MC_CMD_MUM_IN_CMD_LEN 4
pub const MC_CMD_MUM_IN_GPIO_OUT_ENABLE_WRITE_HDR_OFST: c_int = 4;
pub const MC_CMD_MUM_IN_GPIO_OUT_ENABLE_WRITE_HDR_LEN: c_int = 4;
// The first 32-bit word to be written to the GPIO OUT ENABLE register.
pub const MC_CMD_MUM_IN_GPIO_OUT_ENABLE_WRITE_GPIOMASK1_OFST: c_int = 8;
pub const MC_CMD_MUM_IN_GPIO_OUT_ENABLE_WRITE_GPIOMASK1_LEN: c_int = 4;
// The second 32-bit word to be written to the GPIO OUT ENABLE register.
pub const MC_CMD_MUM_IN_GPIO_OUT_ENABLE_WRITE_GPIOMASK2_OFST: c_int = 12;
pub const MC_CMD_MUM_IN_GPIO_OUT_ENABLE_WRITE_GPIOMASK2_LEN: c_int = 4;
// MC_CMD_MUM_IN_GPIO_OUT_ENABLE_READ msgrequest
pub const MC_CMD_MUM_IN_GPIO_OUT_ENABLE_READ_LEN: c_int = 8;
// MC_CMD_MUM_IN_CMD_OFST 0
// MC_CMD_MUM_IN_CMD_LEN 4
pub const MC_CMD_MUM_IN_GPIO_OUT_ENABLE_READ_HDR_OFST: c_int = 4;
pub const MC_CMD_MUM_IN_GPIO_OUT_ENABLE_READ_HDR_LEN: c_int = 4;
// MC_CMD_MUM_IN_GPIO_OP msgrequest
pub const MC_CMD_MUM_IN_GPIO_OP_LEN: c_int = 8;
// MC_CMD_MUM_IN_CMD_OFST 0
// MC_CMD_MUM_IN_CMD_LEN 4
pub const MC_CMD_MUM_IN_GPIO_OP_HDR_OFST: c_int = 4;
pub const MC_CMD_MUM_IN_GPIO_OP_HDR_LEN: c_int = 4;
pub const MC_CMD_MUM_IN_GPIO_OP_BITWISE_OP_OFST: c_int = 4;
pub const MC_CMD_MUM_IN_GPIO_OP_BITWISE_OP_LBN: c_int = 8;
pub const MC_CMD_MUM_IN_GPIO_OP_BITWISE_OP_WIDTH: c_int = 8;
pub const MC_CMD_MUM_IN_GPIO_OP_OUT_READ: c_uint = 0x0 /* enum */;
pub const MC_CMD_MUM_IN_GPIO_OP_OUT_WRITE: c_uint = 0x1 /* enum */;
pub const MC_CMD_MUM_IN_GPIO_OP_OUT_CONFIG: c_uint = 0x2 /* enum */;
pub const MC_CMD_MUM_IN_GPIO_OP_OUT_ENABLE: c_uint = 0x3 /* enum */;
pub const MC_CMD_MUM_IN_GPIO_OP_GPIO_NUMBER_OFST: c_int = 4;
pub const MC_CMD_MUM_IN_GPIO_OP_GPIO_NUMBER_LBN: c_int = 16;
pub const MC_CMD_MUM_IN_GPIO_OP_GPIO_NUMBER_WIDTH: c_int = 8;
// MC_CMD_MUM_IN_GPIO_OP_OUT_READ msgrequest
pub const MC_CMD_MUM_IN_GPIO_OP_OUT_READ_LEN: c_int = 8;
// MC_CMD_MUM_IN_CMD_OFST 0
// MC_CMD_MUM_IN_CMD_LEN 4
pub const MC_CMD_MUM_IN_GPIO_OP_OUT_READ_HDR_OFST: c_int = 4;
pub const MC_CMD_MUM_IN_GPIO_OP_OUT_READ_HDR_LEN: c_int = 4;
// MC_CMD_MUM_IN_GPIO_OP_OUT_WRITE msgrequest
pub const MC_CMD_MUM_IN_GPIO_OP_OUT_WRITE_LEN: c_int = 8;
// MC_CMD_MUM_IN_CMD_OFST 0
// MC_CMD_MUM_IN_CMD_LEN 4
pub const MC_CMD_MUM_IN_GPIO_OP_OUT_WRITE_HDR_OFST: c_int = 4;
pub const MC_CMD_MUM_IN_GPIO_OP_OUT_WRITE_HDR_LEN: c_int = 4;
pub const MC_CMD_MUM_IN_GPIO_OP_OUT_WRITE_WRITEBIT_OFST: c_int = 4;
pub const MC_CMD_MUM_IN_GPIO_OP_OUT_WRITE_WRITEBIT_LBN: c_int = 24;
pub const MC_CMD_MUM_IN_GPIO_OP_OUT_WRITE_WRITEBIT_WIDTH: c_int = 8;
// MC_CMD_MUM_IN_GPIO_OP_OUT_CONFIG msgrequest
pub const MC_CMD_MUM_IN_GPIO_OP_OUT_CONFIG_LEN: c_int = 8;
// MC_CMD_MUM_IN_CMD_OFST 0
// MC_CMD_MUM_IN_CMD_LEN 4
pub const MC_CMD_MUM_IN_GPIO_OP_OUT_CONFIG_HDR_OFST: c_int = 4;
pub const MC_CMD_MUM_IN_GPIO_OP_OUT_CONFIG_HDR_LEN: c_int = 4;
pub const MC_CMD_MUM_IN_GPIO_OP_OUT_CONFIG_CFG_OFST: c_int = 4;
pub const MC_CMD_MUM_IN_GPIO_OP_OUT_CONFIG_CFG_LBN: c_int = 24;
pub const MC_CMD_MUM_IN_GPIO_OP_OUT_CONFIG_CFG_WIDTH: c_int = 8;
// MC_CMD_MUM_IN_GPIO_OP_OUT_ENABLE msgrequest
pub const MC_CMD_MUM_IN_GPIO_OP_OUT_ENABLE_LEN: c_int = 8;
// MC_CMD_MUM_IN_CMD_OFST 0
// MC_CMD_MUM_IN_CMD_LEN 4
pub const MC_CMD_MUM_IN_GPIO_OP_OUT_ENABLE_HDR_OFST: c_int = 4;
pub const MC_CMD_MUM_IN_GPIO_OP_OUT_ENABLE_HDR_LEN: c_int = 4;
pub const MC_CMD_MUM_IN_GPIO_OP_OUT_ENABLE_ENABLEBIT_OFST: c_int = 4;
pub const MC_CMD_MUM_IN_GPIO_OP_OUT_ENABLE_ENABLEBIT_LBN: c_int = 24;
pub const MC_CMD_MUM_IN_GPIO_OP_OUT_ENABLE_ENABLEBIT_WIDTH: c_int = 8;
// MC_CMD_MUM_IN_READ_SENSORS msgrequest
pub const MC_CMD_MUM_IN_READ_SENSORS_LEN: c_int = 8;
// MUM cmd header
// MC_CMD_MUM_IN_CMD_OFST 0
// MC_CMD_MUM_IN_CMD_LEN 4
pub const MC_CMD_MUM_IN_READ_SENSORS_PARAMS_OFST: c_int = 4;
pub const MC_CMD_MUM_IN_READ_SENSORS_PARAMS_LEN: c_int = 4;
pub const MC_CMD_MUM_IN_READ_SENSORS_SENSOR_ID_OFST: c_int = 4;
pub const MC_CMD_MUM_IN_READ_SENSORS_SENSOR_ID_LBN: c_int = 0;
pub const MC_CMD_MUM_IN_READ_SENSORS_SENSOR_ID_WIDTH: c_int = 8;
pub const MC_CMD_MUM_IN_READ_SENSORS_NUM_SENSORS_OFST: c_int = 4;
pub const MC_CMD_MUM_IN_READ_SENSORS_NUM_SENSORS_LBN: c_int = 8;
pub const MC_CMD_MUM_IN_READ_SENSORS_NUM_SENSORS_WIDTH: c_int = 8;
// MC_CMD_MUM_IN_PROGRAM_CLOCKS msgrequest
pub const MC_CMD_MUM_IN_PROGRAM_CLOCKS_LEN: c_int = 12;
// MUM cmd header
// MC_CMD_MUM_IN_CMD_OFST 0
// MC_CMD_MUM_IN_CMD_LEN 4
// Bit-mask of clocks to be programmed
pub const MC_CMD_MUM_IN_PROGRAM_CLOCKS_MASK_OFST: c_int = 4;
pub const MC_CMD_MUM_IN_PROGRAM_CLOCKS_MASK_LEN: c_int = 4;
pub const MC_CMD_MUM_CLOCK_ID_FPGA: c_uint = 0x0 /* enum */;
pub const MC_CMD_MUM_CLOCK_ID_DDR: c_uint = 0x1 /* enum */;
pub const MC_CMD_MUM_CLOCK_ID_NIC: c_uint = 0x2 /* enum */;
// Control flags for clock programming
pub const MC_CMD_MUM_IN_PROGRAM_CLOCKS_FLAGS_OFST: c_int = 8;
pub const MC_CMD_MUM_IN_PROGRAM_CLOCKS_FLAGS_LEN: c_int = 4;
pub const MC_CMD_MUM_IN_PROGRAM_CLOCKS_OVERCLOCK_110_OFST: c_int = 8;
pub const MC_CMD_MUM_IN_PROGRAM_CLOCKS_OVERCLOCK_110_LBN: c_int = 0;
pub const MC_CMD_MUM_IN_PROGRAM_CLOCKS_OVERCLOCK_110_WIDTH: c_int = 1;
pub const MC_CMD_MUM_IN_PROGRAM_CLOCKS_CLOCK_NIC_FROM_FPGA_OFST: c_int = 8;
pub const MC_CMD_MUM_IN_PROGRAM_CLOCKS_CLOCK_NIC_FROM_FPGA_LBN: c_int = 1;
pub const MC_CMD_MUM_IN_PROGRAM_CLOCKS_CLOCK_NIC_FROM_FPGA_WIDTH: c_int = 1;
pub const MC_CMD_MUM_IN_PROGRAM_CLOCKS_CLOCK_REF_FROM_XO_OFST: c_int = 8;
pub const MC_CMD_MUM_IN_PROGRAM_CLOCKS_CLOCK_REF_FROM_XO_LBN: c_int = 2;
pub const MC_CMD_MUM_IN_PROGRAM_CLOCKS_CLOCK_REF_FROM_XO_WIDTH: c_int = 1;
// MC_CMD_MUM_IN_FPGA_LOAD msgrequest
pub const MC_CMD_MUM_IN_FPGA_LOAD_LEN: c_int = 8;
// MUM cmd header
// MC_CMD_MUM_IN_CMD_OFST 0
// MC_CMD_MUM_IN_CMD_LEN 4
// Enable/Disable FPGA config from flash
pub const MC_CMD_MUM_IN_FPGA_LOAD_ENABLE_OFST: c_int = 4;
pub const MC_CMD_MUM_IN_FPGA_LOAD_ENABLE_LEN: c_int = 4;
// MC_CMD_MUM_IN_READ_ATB_SENSOR msgrequest
pub const MC_CMD_MUM_IN_READ_ATB_SENSOR_LEN: c_int = 4;
// MUM cmd header
// MC_CMD_MUM_IN_CMD_OFST 0
// MC_CMD_MUM_IN_CMD_LEN 4
// MC_CMD_MUM_IN_QSFP msgrequest
pub const MC_CMD_MUM_IN_QSFP_LEN: c_int = 12;
// MUM cmd header
// MC_CMD_MUM_IN_CMD_OFST 0
// MC_CMD_MUM_IN_CMD_LEN 4
pub const MC_CMD_MUM_IN_QSFP_HDR_OFST: c_int = 4;
pub const MC_CMD_MUM_IN_QSFP_HDR_LEN: c_int = 4;
pub const MC_CMD_MUM_IN_QSFP_OPCODE_OFST: c_int = 4;
pub const MC_CMD_MUM_IN_QSFP_OPCODE_LBN: c_int = 0;
pub const MC_CMD_MUM_IN_QSFP_OPCODE_WIDTH: c_int = 4;
pub const MC_CMD_MUM_IN_QSFP_INIT: c_uint = 0x0 /* enum */;
pub const MC_CMD_MUM_IN_QSFP_RECONFIGURE: c_uint = 0x1 /* enum */;
pub const MC_CMD_MUM_IN_QSFP_GET_SUPPORTED_CAP: c_uint = 0x2 /* enum */;
pub const MC_CMD_MUM_IN_QSFP_GET_MEDIA_INFO: c_uint = 0x3 /* enum */;
pub const MC_CMD_MUM_IN_QSFP_FILL_STATS: c_uint = 0x4 /* enum */;
pub const MC_CMD_MUM_IN_QSFP_POLL_BIST: c_uint = 0x5 /* enum */;
pub const MC_CMD_MUM_IN_QSFP_IDX_OFST: c_int = 8;
pub const MC_CMD_MUM_IN_QSFP_IDX_LEN: c_int = 4;
// MC_CMD_MUM_IN_QSFP_INIT msgrequest
pub const MC_CMD_MUM_IN_QSFP_INIT_LEN: c_int = 16;
// MC_CMD_MUM_IN_CMD_OFST 0
// MC_CMD_MUM_IN_CMD_LEN 4
pub const MC_CMD_MUM_IN_QSFP_INIT_HDR_OFST: c_int = 4;
pub const MC_CMD_MUM_IN_QSFP_INIT_HDR_LEN: c_int = 4;
pub const MC_CMD_MUM_IN_QSFP_INIT_IDX_OFST: c_int = 8;
pub const MC_CMD_MUM_IN_QSFP_INIT_IDX_LEN: c_int = 4;
pub const MC_CMD_MUM_IN_QSFP_INIT_CAGE_OFST: c_int = 12;
pub const MC_CMD_MUM_IN_QSFP_INIT_CAGE_LEN: c_int = 4;
// MC_CMD_MUM_IN_QSFP_RECONFIGURE msgrequest
pub const MC_CMD_MUM_IN_QSFP_RECONFIGURE_LEN: c_int = 24;
// MC_CMD_MUM_IN_CMD_OFST 0
// MC_CMD_MUM_IN_CMD_LEN 4
pub const MC_CMD_MUM_IN_QSFP_RECONFIGURE_HDR_OFST: c_int = 4;
pub const MC_CMD_MUM_IN_QSFP_RECONFIGURE_HDR_LEN: c_int = 4;
pub const MC_CMD_MUM_IN_QSFP_RECONFIGURE_IDX_OFST: c_int = 8;
pub const MC_CMD_MUM_IN_QSFP_RECONFIGURE_IDX_LEN: c_int = 4;
pub const MC_CMD_MUM_IN_QSFP_RECONFIGURE_TX_DISABLE_OFST: c_int = 12;
pub const MC_CMD_MUM_IN_QSFP_RECONFIGURE_TX_DISABLE_LEN: c_int = 4;
pub const MC_CMD_MUM_IN_QSFP_RECONFIGURE_PORT_LANES_OFST: c_int = 16;
pub const MC_CMD_MUM_IN_QSFP_RECONFIGURE_PORT_LANES_LEN: c_int = 4;
pub const MC_CMD_MUM_IN_QSFP_RECONFIGURE_PORT_LINK_SPEED_OFST: c_int = 20;
pub const MC_CMD_MUM_IN_QSFP_RECONFIGURE_PORT_LINK_SPEED_LEN: c_int = 4;
// MC_CMD_MUM_IN_QSFP_GET_SUPPORTED_CAP msgrequest
pub const MC_CMD_MUM_IN_QSFP_GET_SUPPORTED_CAP_LEN: c_int = 12;
// MC_CMD_MUM_IN_CMD_OFST 0
// MC_CMD_MUM_IN_CMD_LEN 4
pub const MC_CMD_MUM_IN_QSFP_GET_SUPPORTED_CAP_HDR_OFST: c_int = 4;
pub const MC_CMD_MUM_IN_QSFP_GET_SUPPORTED_CAP_HDR_LEN: c_int = 4;
pub const MC_CMD_MUM_IN_QSFP_GET_SUPPORTED_CAP_IDX_OFST: c_int = 8;
pub const MC_CMD_MUM_IN_QSFP_GET_SUPPORTED_CAP_IDX_LEN: c_int = 4;
// MC_CMD_MUM_IN_QSFP_GET_MEDIA_INFO msgrequest
pub const MC_CMD_MUM_IN_QSFP_GET_MEDIA_INFO_LEN: c_int = 16;
// MC_CMD_MUM_IN_CMD_OFST 0
// MC_CMD_MUM_IN_CMD_LEN 4
pub const MC_CMD_MUM_IN_QSFP_GET_MEDIA_INFO_HDR_OFST: c_int = 4;
pub const MC_CMD_MUM_IN_QSFP_GET_MEDIA_INFO_HDR_LEN: c_int = 4;
pub const MC_CMD_MUM_IN_QSFP_GET_MEDIA_INFO_IDX_OFST: c_int = 8;
pub const MC_CMD_MUM_IN_QSFP_GET_MEDIA_INFO_IDX_LEN: c_int = 4;
pub const MC_CMD_MUM_IN_QSFP_GET_MEDIA_INFO_PAGE_OFST: c_int = 12;
pub const MC_CMD_MUM_IN_QSFP_GET_MEDIA_INFO_PAGE_LEN: c_int = 4;
// MC_CMD_MUM_IN_QSFP_FILL_STATS msgrequest
pub const MC_CMD_MUM_IN_QSFP_FILL_STATS_LEN: c_int = 12;
// MC_CMD_MUM_IN_CMD_OFST 0
// MC_CMD_MUM_IN_CMD_LEN 4
pub const MC_CMD_MUM_IN_QSFP_FILL_STATS_HDR_OFST: c_int = 4;
pub const MC_CMD_MUM_IN_QSFP_FILL_STATS_HDR_LEN: c_int = 4;
pub const MC_CMD_MUM_IN_QSFP_FILL_STATS_IDX_OFST: c_int = 8;
pub const MC_CMD_MUM_IN_QSFP_FILL_STATS_IDX_LEN: c_int = 4;
// MC_CMD_MUM_IN_QSFP_POLL_BIST msgrequest
pub const MC_CMD_MUM_IN_QSFP_POLL_BIST_LEN: c_int = 12;
// MC_CMD_MUM_IN_CMD_OFST 0
// MC_CMD_MUM_IN_CMD_LEN 4
pub const MC_CMD_MUM_IN_QSFP_POLL_BIST_HDR_OFST: c_int = 4;
pub const MC_CMD_MUM_IN_QSFP_POLL_BIST_HDR_LEN: c_int = 4;
pub const MC_CMD_MUM_IN_QSFP_POLL_BIST_IDX_OFST: c_int = 8;
pub const MC_CMD_MUM_IN_QSFP_POLL_BIST_IDX_LEN: c_int = 4;
// MC_CMD_MUM_IN_READ_DDR_INFO msgrequest
pub const MC_CMD_MUM_IN_READ_DDR_INFO_LEN: c_int = 4;
// MUM cmd header
// MC_CMD_MUM_IN_CMD_OFST 0
// MC_CMD_MUM_IN_CMD_LEN 4
// MC_CMD_MUM_OUT msgresponse
pub const MC_CMD_MUM_OUT_LEN: c_int = 0;
// MC_CMD_MUM_OUT_NULL msgresponse
pub const MC_CMD_MUM_OUT_NULL_LEN: c_int = 0;
// MC_CMD_MUM_OUT_GET_VERSION msgresponse
pub const MC_CMD_MUM_OUT_GET_VERSION_LEN: c_int = 12;
pub const MC_CMD_MUM_OUT_GET_VERSION_FIRMWARE_OFST: c_int = 0;
pub const MC_CMD_MUM_OUT_GET_VERSION_FIRMWARE_LEN: c_int = 4;
pub const MC_CMD_MUM_OUT_GET_VERSION_VERSION_OFST: c_int = 4;
pub const MC_CMD_MUM_OUT_GET_VERSION_VERSION_LEN: c_int = 8;
pub const MC_CMD_MUM_OUT_GET_VERSION_VERSION_LO_OFST: c_int = 4;
pub const MC_CMD_MUM_OUT_GET_VERSION_VERSION_HI_OFST: c_int = 8;
// MC_CMD_MUM_OUT_RAW_CMD msgresponse
pub const MC_CMD_MUM_OUT_RAW_CMD_LENMIN: c_int = 1;
pub const MC_CMD_MUM_OUT_RAW_CMD_LENMAX: c_int = 252;
pub const MC_CMD_MUM_OUT_RAW_CMD_LENMAX_MCDI2: c_int = 1020;

// returned data
pub const MC_CMD_MUM_OUT_RAW_CMD_DATA_OFST: c_int = 0;
pub const MC_CMD_MUM_OUT_RAW_CMD_DATA_LEN: c_int = 1;
pub const MC_CMD_MUM_OUT_RAW_CMD_DATA_MINNUM: c_int = 1;
pub const MC_CMD_MUM_OUT_RAW_CMD_DATA_MAXNUM: c_int = 252;
pub const MC_CMD_MUM_OUT_RAW_CMD_DATA_MAXNUM_MCDI2: c_int = 1020;
// MC_CMD_MUM_OUT_READ msgresponse
pub const MC_CMD_MUM_OUT_READ_LENMIN: c_int = 4;
pub const MC_CMD_MUM_OUT_READ_LENMAX: c_int = 252;
pub const MC_CMD_MUM_OUT_READ_LENMAX_MCDI2: c_int = 1020;

pub const MC_CMD_MUM_OUT_READ_BUFFER_OFST: c_int = 0;
pub const MC_CMD_MUM_OUT_READ_BUFFER_LEN: c_int = 4;
pub const MC_CMD_MUM_OUT_READ_BUFFER_MINNUM: c_int = 1;
pub const MC_CMD_MUM_OUT_READ_BUFFER_MAXNUM: c_int = 63;
pub const MC_CMD_MUM_OUT_READ_BUFFER_MAXNUM_MCDI2: c_int = 255;
// MC_CMD_MUM_OUT_WRITE msgresponse
pub const MC_CMD_MUM_OUT_WRITE_LEN: c_int = 0;
// MC_CMD_MUM_OUT_LOG msgresponse
pub const MC_CMD_MUM_OUT_LOG_LEN: c_int = 0;
// MC_CMD_MUM_OUT_LOG_OP_UART msgresponse
pub const MC_CMD_MUM_OUT_LOG_OP_UART_LEN: c_int = 0;
// MC_CMD_MUM_OUT_GPIO_IN_READ msgresponse
pub const MC_CMD_MUM_OUT_GPIO_IN_READ_LEN: c_int = 8;
// The first 32-bit word read from the GPIO IN register.
pub const MC_CMD_MUM_OUT_GPIO_IN_READ_GPIOMASK1_OFST: c_int = 0;
pub const MC_CMD_MUM_OUT_GPIO_IN_READ_GPIOMASK1_LEN: c_int = 4;
// The second 32-bit word read from the GPIO IN register.
pub const MC_CMD_MUM_OUT_GPIO_IN_READ_GPIOMASK2_OFST: c_int = 4;
pub const MC_CMD_MUM_OUT_GPIO_IN_READ_GPIOMASK2_LEN: c_int = 4;
// MC_CMD_MUM_OUT_GPIO_OUT_WRITE msgresponse
pub const MC_CMD_MUM_OUT_GPIO_OUT_WRITE_LEN: c_int = 0;
// MC_CMD_MUM_OUT_GPIO_OUT_READ msgresponse
pub const MC_CMD_MUM_OUT_GPIO_OUT_READ_LEN: c_int = 8;
// The first 32-bit word read from the GPIO OUT register.
pub const MC_CMD_MUM_OUT_GPIO_OUT_READ_GPIOMASK1_OFST: c_int = 0;
pub const MC_CMD_MUM_OUT_GPIO_OUT_READ_GPIOMASK1_LEN: c_int = 4;
// The second 32-bit word read from the GPIO OUT register.
pub const MC_CMD_MUM_OUT_GPIO_OUT_READ_GPIOMASK2_OFST: c_int = 4;
pub const MC_CMD_MUM_OUT_GPIO_OUT_READ_GPIOMASK2_LEN: c_int = 4;
// MC_CMD_MUM_OUT_GPIO_OUT_ENABLE_WRITE msgresponse
pub const MC_CMD_MUM_OUT_GPIO_OUT_ENABLE_WRITE_LEN: c_int = 0;
// MC_CMD_MUM_OUT_GPIO_OUT_ENABLE_READ msgresponse
pub const MC_CMD_MUM_OUT_GPIO_OUT_ENABLE_READ_LEN: c_int = 8;
pub const MC_CMD_MUM_OUT_GPIO_OUT_ENABLE_READ_GPIOMASK1_OFST: c_int = 0;
pub const MC_CMD_MUM_OUT_GPIO_OUT_ENABLE_READ_GPIOMASK1_LEN: c_int = 4;
pub const MC_CMD_MUM_OUT_GPIO_OUT_ENABLE_READ_GPIOMASK2_OFST: c_int = 4;
pub const MC_CMD_MUM_OUT_GPIO_OUT_ENABLE_READ_GPIOMASK2_LEN: c_int = 4;
// MC_CMD_MUM_OUT_GPIO_OP_OUT_READ msgresponse
pub const MC_CMD_MUM_OUT_GPIO_OP_OUT_READ_LEN: c_int = 4;
pub const MC_CMD_MUM_OUT_GPIO_OP_OUT_READ_BIT_READ_OFST: c_int = 0;
pub const MC_CMD_MUM_OUT_GPIO_OP_OUT_READ_BIT_READ_LEN: c_int = 4;
// MC_CMD_MUM_OUT_GPIO_OP_OUT_WRITE msgresponse
pub const MC_CMD_MUM_OUT_GPIO_OP_OUT_WRITE_LEN: c_int = 0;
// MC_CMD_MUM_OUT_GPIO_OP_OUT_CONFIG msgresponse
pub const MC_CMD_MUM_OUT_GPIO_OP_OUT_CONFIG_LEN: c_int = 0;
// MC_CMD_MUM_OUT_GPIO_OP_OUT_ENABLE msgresponse
pub const MC_CMD_MUM_OUT_GPIO_OP_OUT_ENABLE_LEN: c_int = 0;
// MC_CMD_MUM_OUT_READ_SENSORS msgresponse
pub const MC_CMD_MUM_OUT_READ_SENSORS_LENMIN: c_int = 4;
pub const MC_CMD_MUM_OUT_READ_SENSORS_LENMAX: c_int = 252;
pub const MC_CMD_MUM_OUT_READ_SENSORS_LENMAX_MCDI2: c_int = 1020;

pub const MC_CMD_MUM_OUT_READ_SENSORS_DATA_OFST: c_int = 0;
pub const MC_CMD_MUM_OUT_READ_SENSORS_DATA_LEN: c_int = 4;
pub const MC_CMD_MUM_OUT_READ_SENSORS_DATA_MINNUM: c_int = 1;
pub const MC_CMD_MUM_OUT_READ_SENSORS_DATA_MAXNUM: c_int = 63;
pub const MC_CMD_MUM_OUT_READ_SENSORS_DATA_MAXNUM_MCDI2: c_int = 255;
pub const MC_CMD_MUM_OUT_READ_SENSORS_READING_OFST: c_int = 0;
pub const MC_CMD_MUM_OUT_READ_SENSORS_READING_LBN: c_int = 0;
pub const MC_CMD_MUM_OUT_READ_SENSORS_READING_WIDTH: c_int = 16;
pub const MC_CMD_MUM_OUT_READ_SENSORS_STATE_OFST: c_int = 0;
pub const MC_CMD_MUM_OUT_READ_SENSORS_STATE_LBN: c_int = 16;
pub const MC_CMD_MUM_OUT_READ_SENSORS_STATE_WIDTH: c_int = 8;
pub const MC_CMD_MUM_OUT_READ_SENSORS_TYPE_OFST: c_int = 0;
pub const MC_CMD_MUM_OUT_READ_SENSORS_TYPE_LBN: c_int = 24;
pub const MC_CMD_MUM_OUT_READ_SENSORS_TYPE_WIDTH: c_int = 8;
// MC_CMD_MUM_OUT_PROGRAM_CLOCKS msgresponse
pub const MC_CMD_MUM_OUT_PROGRAM_CLOCKS_LEN: c_int = 4;
pub const MC_CMD_MUM_OUT_PROGRAM_CLOCKS_OK_MASK_OFST: c_int = 0;
pub const MC_CMD_MUM_OUT_PROGRAM_CLOCKS_OK_MASK_LEN: c_int = 4;
// MC_CMD_MUM_OUT_FPGA_LOAD msgresponse
pub const MC_CMD_MUM_OUT_FPGA_LOAD_LEN: c_int = 0;
// MC_CMD_MUM_OUT_READ_ATB_SENSOR msgresponse
pub const MC_CMD_MUM_OUT_READ_ATB_SENSOR_LEN: c_int = 4;
pub const MC_CMD_MUM_OUT_READ_ATB_SENSOR_RESULT_OFST: c_int = 0;
pub const MC_CMD_MUM_OUT_READ_ATB_SENSOR_RESULT_LEN: c_int = 4;
// MC_CMD_MUM_OUT_QSFP_INIT msgresponse
pub const MC_CMD_MUM_OUT_QSFP_INIT_LEN: c_int = 0;
// MC_CMD_MUM_OUT_QSFP_RECONFIGURE msgresponse
pub const MC_CMD_MUM_OUT_QSFP_RECONFIGURE_LEN: c_int = 8;
pub const MC_CMD_MUM_OUT_QSFP_RECONFIGURE_PORT_PHY_LP_CAP_OFST: c_int = 0;
pub const MC_CMD_MUM_OUT_QSFP_RECONFIGURE_PORT_PHY_LP_CAP_LEN: c_int = 4;
pub const MC_CMD_MUM_OUT_QSFP_RECONFIGURE_PORT_PHY_FLAGS_OFST: c_int = 4;
pub const MC_CMD_MUM_OUT_QSFP_RECONFIGURE_PORT_PHY_FLAGS_LEN: c_int = 4;
pub const MC_CMD_MUM_OUT_QSFP_RECONFIGURE_PORT_PHY_READY_OFST: c_int = 4;
pub const MC_CMD_MUM_OUT_QSFP_RECONFIGURE_PORT_PHY_READY_LBN: c_int = 0;
pub const MC_CMD_MUM_OUT_QSFP_RECONFIGURE_PORT_PHY_READY_WIDTH: c_int = 1;
pub const MC_CMD_MUM_OUT_QSFP_RECONFIGURE_PORT_PHY_LINK_UP_OFST: c_int = 4;
pub const MC_CMD_MUM_OUT_QSFP_RECONFIGURE_PORT_PHY_LINK_UP_LBN: c_int = 1;
pub const MC_CMD_MUM_OUT_QSFP_RECONFIGURE_PORT_PHY_LINK_UP_WIDTH: c_int = 1;
// MC_CMD_MUM_OUT_QSFP_GET_SUPPORTED_CAP msgresponse
pub const MC_CMD_MUM_OUT_QSFP_GET_SUPPORTED_CAP_LEN: c_int = 4;
pub const MC_CMD_MUM_OUT_QSFP_GET_SUPPORTED_CAP_PORT_PHY_LP_CAP_OFST: c_int = 0;
pub const MC_CMD_MUM_OUT_QSFP_GET_SUPPORTED_CAP_PORT_PHY_LP_CAP_LEN: c_int = 4;
// MC_CMD_MUM_OUT_QSFP_GET_MEDIA_INFO msgresponse
pub const MC_CMD_MUM_OUT_QSFP_GET_MEDIA_INFO_LENMIN: c_int = 5;
pub const MC_CMD_MUM_OUT_QSFP_GET_MEDIA_INFO_LENMAX: c_int = 252;
pub const MC_CMD_MUM_OUT_QSFP_GET_MEDIA_INFO_LENMAX_MCDI2: c_int = 1020;

// in bytes
pub const MC_CMD_MUM_OUT_QSFP_GET_MEDIA_INFO_DATALEN_OFST: c_int = 0;
pub const MC_CMD_MUM_OUT_QSFP_GET_MEDIA_INFO_DATALEN_LEN: c_int = 4;
pub const MC_CMD_MUM_OUT_QSFP_GET_MEDIA_INFO_DATA_OFST: c_int = 4;
pub const MC_CMD_MUM_OUT_QSFP_GET_MEDIA_INFO_DATA_LEN: c_int = 1;
pub const MC_CMD_MUM_OUT_QSFP_GET_MEDIA_INFO_DATA_MINNUM: c_int = 1;
pub const MC_CMD_MUM_OUT_QSFP_GET_MEDIA_INFO_DATA_MAXNUM: c_int = 248;
pub const MC_CMD_MUM_OUT_QSFP_GET_MEDIA_INFO_DATA_MAXNUM_MCDI2: c_int = 1016;
// MC_CMD_MUM_OUT_QSFP_FILL_STATS msgresponse
pub const MC_CMD_MUM_OUT_QSFP_FILL_STATS_LEN: c_int = 8;
pub const MC_CMD_MUM_OUT_QSFP_FILL_STATS_PORT_PHY_STATS_PMA_PMD_LINK_UP_OFST: c_int = 0;
pub const MC_CMD_MUM_OUT_QSFP_FILL_STATS_PORT_PHY_STATS_PMA_PMD_LINK_UP_LEN: c_int = 4;
pub const MC_CMD_MUM_OUT_QSFP_FILL_STATS_PORT_PHY_STATS_PCS_LINK_UP_OFST: c_int = 4;
pub const MC_CMD_MUM_OUT_QSFP_FILL_STATS_PORT_PHY_STATS_PCS_LINK_UP_LEN: c_int = 4;
// MC_CMD_MUM_OUT_QSFP_POLL_BIST msgresponse
pub const MC_CMD_MUM_OUT_QSFP_POLL_BIST_LEN: c_int = 4;
pub const MC_CMD_MUM_OUT_QSFP_POLL_BIST_TEST_OFST: c_int = 0;
pub const MC_CMD_MUM_OUT_QSFP_POLL_BIST_TEST_LEN: c_int = 4;
// MC_CMD_MUM_OUT_READ_DDR_INFO msgresponse
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_LENMIN: c_int = 24;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_LENMAX: c_int = 248;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_LENMAX_MCDI2: c_int = 1016;

// Discrete (soldered) DDR resistor strap info
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_DISCRETE_DDR_INFO_OFST: c_int = 0;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_DISCRETE_DDR_INFO_LEN: c_int = 4;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_VRATIO_OFST: c_int = 0;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_VRATIO_LBN: c_int = 0;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_VRATIO_WIDTH: c_int = 16;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_RESERVED1_OFST: c_int = 0;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_RESERVED1_LBN: c_int = 16;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_RESERVED1_WIDTH: c_int = 16;
// Number of SODIMM info records
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_NUM_RECORDS_OFST: c_int = 4;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_NUM_RECORDS_LEN: c_int = 4;
// Array of SODIMM info records
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_SODIMM_INFO_RECORD_OFST: c_int = 8;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_SODIMM_INFO_RECORD_LEN: c_int = 8;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_SODIMM_INFO_RECORD_LO_OFST: c_int = 8;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_SODIMM_INFO_RECORD_HI_OFST: c_int = 12;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_SODIMM_INFO_RECORD_MINNUM: c_int = 2;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_SODIMM_INFO_RECORD_MAXNUM: c_int = 30;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_SODIMM_INFO_RECORD_MAXNUM_MCDI2: c_int = 126;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_BANK_ID_OFST: c_int = 8;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_BANK_ID_LBN: c_int = 0;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_BANK_ID_WIDTH: c_int = 8;
// enum: SODIMM bank 1 (Top SODIMM for Sorrento)
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_BANK1: c_uint = 0x0;
// enum: SODIMM bank 2 (Bottom SODDIMM for Sorrento)
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_BANK2: c_uint = 0x1;
// enum: Total number of SODIMM banks
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_NUM_BANKS: c_uint = 0x2;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_TYPE_OFST: c_int = 8;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_TYPE_LBN: c_int = 8;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_TYPE_WIDTH: c_int = 8;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_RANK_OFST: c_int = 8;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_RANK_LBN: c_int = 16;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_RANK_WIDTH: c_int = 4;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_VOLTAGE_OFST: c_int = 8;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_VOLTAGE_LBN: c_int = 20;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_VOLTAGE_WIDTH: c_int = 4;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_NOT_POWERED: c_uint = 0x0 /* enum */;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_1V25: c_uint = 0x1 /* enum */;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_1V35: c_uint = 0x2 /* enum */;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_1V5: c_uint = 0x3 /* enum */;
// enum: Values 5-15 are reserved for future usage
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_1V8: c_uint = 0x4;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_SIZE_OFST: c_int = 8;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_SIZE_LBN: c_int = 24;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_SIZE_WIDTH: c_int = 8;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_SPEED_OFST: c_int = 8;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_SPEED_LBN: c_int = 32;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_SPEED_WIDTH: c_int = 16;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_STATE_OFST: c_int = 8;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_STATE_LBN: c_int = 48;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_STATE_WIDTH: c_int = 4;
// enum: No module present
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_ABSENT: c_uint = 0x0;
// enum: Module present supported and powered on
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_PRESENT_POWERED: c_uint = 0x1;
// enum: Module present but bad type
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_PRESENT_BAD_TYPE: c_uint = 0x2;
// enum: Module present but incompatible voltage
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_PRESENT_BAD_VOLTAGE: c_uint = 0x3;
// enum: Module present but unknown SPD
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_PRESENT_BAD_SPD: c_uint = 0x4;
// enum: Module present but slot cannot support it
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_PRESENT_BAD_SLOT: c_uint = 0x5;
// enum: Modules may or may not be present, but cannot establish contact by I2C
//
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_NOT_REACHABLE: c_uint = 0x6;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_RESERVED2_OFST: c_int = 8;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_RESERVED2_LBN: c_int = 52;
pub const MC_CMD_MUM_OUT_READ_DDR_INFO_RESERVED2_WIDTH: c_int = 12;
// MC_CMD_DYNAMIC_SENSORS_LIMITS structuredef: Set of sensor limits. This
// should match the equivalent structure in the sensor_query SPHINX service.
//
pub const MC_CMD_DYNAMIC_SENSORS_LIMITS_LEN: c_int = 24;
// A value below this will trigger a warning event.
pub const MC_CMD_DYNAMIC_SENSORS_LIMITS_LO_WARNING_OFST: c_int = 0;
pub const MC_CMD_DYNAMIC_SENSORS_LIMITS_LO_WARNING_LEN: c_int = 4;
pub const MC_CMD_DYNAMIC_SENSORS_LIMITS_LO_WARNING_LBN: c_int = 0;
pub const MC_CMD_DYNAMIC_SENSORS_LIMITS_LO_WARNING_WIDTH: c_int = 32;
// A value below this will trigger a critical event.
pub const MC_CMD_DYNAMIC_SENSORS_LIMITS_LO_CRITICAL_OFST: c_int = 4;
pub const MC_CMD_DYNAMIC_SENSORS_LIMITS_LO_CRITICAL_LEN: c_int = 4;
pub const MC_CMD_DYNAMIC_SENSORS_LIMITS_LO_CRITICAL_LBN: c_int = 32;
pub const MC_CMD_DYNAMIC_SENSORS_LIMITS_LO_CRITICAL_WIDTH: c_int = 32;
// A value below this will shut down the card.
pub const MC_CMD_DYNAMIC_SENSORS_LIMITS_LO_FATAL_OFST: c_int = 8;
pub const MC_CMD_DYNAMIC_SENSORS_LIMITS_LO_FATAL_LEN: c_int = 4;
pub const MC_CMD_DYNAMIC_SENSORS_LIMITS_LO_FATAL_LBN: c_int = 64;
pub const MC_CMD_DYNAMIC_SENSORS_LIMITS_LO_FATAL_WIDTH: c_int = 32;
// A value above this will trigger a warning event.
pub const MC_CMD_DYNAMIC_SENSORS_LIMITS_HI_WARNING_OFST: c_int = 12;
pub const MC_CMD_DYNAMIC_SENSORS_LIMITS_HI_WARNING_LEN: c_int = 4;
pub const MC_CMD_DYNAMIC_SENSORS_LIMITS_HI_WARNING_LBN: c_int = 96;
pub const MC_CMD_DYNAMIC_SENSORS_LIMITS_HI_WARNING_WIDTH: c_int = 32;
// A value above this will trigger a critical event.
pub const MC_CMD_DYNAMIC_SENSORS_LIMITS_HI_CRITICAL_OFST: c_int = 16;
pub const MC_CMD_DYNAMIC_SENSORS_LIMITS_HI_CRITICAL_LEN: c_int = 4;
pub const MC_CMD_DYNAMIC_SENSORS_LIMITS_HI_CRITICAL_LBN: c_int = 128;
pub const MC_CMD_DYNAMIC_SENSORS_LIMITS_HI_CRITICAL_WIDTH: c_int = 32;
// A value above this will shut down the card.
pub const MC_CMD_DYNAMIC_SENSORS_LIMITS_HI_FATAL_OFST: c_int = 20;
pub const MC_CMD_DYNAMIC_SENSORS_LIMITS_HI_FATAL_LEN: c_int = 4;
pub const MC_CMD_DYNAMIC_SENSORS_LIMITS_HI_FATAL_LBN: c_int = 160;
pub const MC_CMD_DYNAMIC_SENSORS_LIMITS_HI_FATAL_WIDTH: c_int = 32;
// MC_CMD_DYNAMIC_SENSORS_DESCRIPTION structuredef: Description of a sensor.
// This should match the equivalent structure in the sensor_query SPHINX
// service.
//
pub const MC_CMD_DYNAMIC_SENSORS_DESCRIPTION_LEN: c_int = 64;
// The handle used to identify the sensor in calls to
// MC_CMD_DYNAMIC_SENSORS_GET_VALUES
//
pub const MC_CMD_DYNAMIC_SENSORS_DESCRIPTION_HANDLE_OFST: c_int = 0;
pub const MC_CMD_DYNAMIC_SENSORS_DESCRIPTION_HANDLE_LEN: c_int = 4;
pub const MC_CMD_DYNAMIC_SENSORS_DESCRIPTION_HANDLE_LBN: c_int = 0;
pub const MC_CMD_DYNAMIC_SENSORS_DESCRIPTION_HANDLE_WIDTH: c_int = 32;
// A human-readable name for the sensor (zero terminated string, max 32 bytes)
//
pub const MC_CMD_DYNAMIC_SENSORS_DESCRIPTION_NAME_OFST: c_int = 4;
pub const MC_CMD_DYNAMIC_SENSORS_DESCRIPTION_NAME_LEN: c_int = 32;
pub const MC_CMD_DYNAMIC_SENSORS_DESCRIPTION_NAME_LBN: c_int = 32;
pub const MC_CMD_DYNAMIC_SENSORS_DESCRIPTION_NAME_WIDTH: c_int = 256;
// The type of the sensor device, and by implication the unit of that the
// values will be reported in
//
pub const MC_CMD_DYNAMIC_SENSORS_DESCRIPTION_TYPE_OFST: c_int = 36;
pub const MC_CMD_DYNAMIC_SENSORS_DESCRIPTION_TYPE_LEN: c_int = 4;
// enum: A voltage sensor. Unit is mV
pub const MC_CMD_DYNAMIC_SENSORS_DESCRIPTION_VOLTAGE: c_uint = 0x0;
// enum: A current sensor. Unit is mA
pub const MC_CMD_DYNAMIC_SENSORS_DESCRIPTION_CURRENT: c_uint = 0x1;
// enum: A power sensor. Unit is mW
pub const MC_CMD_DYNAMIC_SENSORS_DESCRIPTION_POWER: c_uint = 0x2;
// enum: A temperature sensor. Unit is Celsius
pub const MC_CMD_DYNAMIC_SENSORS_DESCRIPTION_TEMPERATURE: c_uint = 0x3;
// enum: A cooling fan sensor. Unit is RPM
pub const MC_CMD_DYNAMIC_SENSORS_DESCRIPTION_FAN: c_uint = 0x4;
pub const MC_CMD_DYNAMIC_SENSORS_DESCRIPTION_TYPE_LBN: c_int = 288;
pub const MC_CMD_DYNAMIC_SENSORS_DESCRIPTION_TYPE_WIDTH: c_int = 32;
// A single MC_CMD_DYNAMIC_SENSORS_LIMITS structure
pub const MC_CMD_DYNAMIC_SENSORS_DESCRIPTION_LIMITS_OFST: c_int = 40;
pub const MC_CMD_DYNAMIC_SENSORS_DESCRIPTION_LIMITS_LEN: c_int = 24;
pub const MC_CMD_DYNAMIC_SENSORS_DESCRIPTION_LIMITS_LBN: c_int = 320;
pub const MC_CMD_DYNAMIC_SENSORS_DESCRIPTION_LIMITS_WIDTH: c_int = 192;
// MC_CMD_DYNAMIC_SENSORS_READING structuredef: State and value of a sensor.
// This should match the equivalent structure in the sensor_query SPHINX
// service.
//
pub const MC_CMD_DYNAMIC_SENSORS_READING_LEN: c_int = 12;
// The handle used to identify the sensor
pub const MC_CMD_DYNAMIC_SENSORS_READING_HANDLE_OFST: c_int = 0;
pub const MC_CMD_DYNAMIC_SENSORS_READING_HANDLE_LEN: c_int = 4;
pub const MC_CMD_DYNAMIC_SENSORS_READING_HANDLE_LBN: c_int = 0;
pub const MC_CMD_DYNAMIC_SENSORS_READING_HANDLE_WIDTH: c_int = 32;
// The current value of the sensor
pub const MC_CMD_DYNAMIC_SENSORS_READING_VALUE_OFST: c_int = 4;
pub const MC_CMD_DYNAMIC_SENSORS_READING_VALUE_LEN: c_int = 4;
pub const MC_CMD_DYNAMIC_SENSORS_READING_VALUE_LBN: c_int = 32;
pub const MC_CMD_DYNAMIC_SENSORS_READING_VALUE_WIDTH: c_int = 32;
// The sensor's condition, e.g. good, broken or removed
pub const MC_CMD_DYNAMIC_SENSORS_READING_STATE_OFST: c_int = 8;
pub const MC_CMD_DYNAMIC_SENSORS_READING_STATE_LEN: c_int = 4;
// enum: Sensor working normally within limits
pub const MC_CMD_DYNAMIC_SENSORS_READING_OK: c_uint = 0x0;
// enum: Warning threshold breached
pub const MC_CMD_DYNAMIC_SENSORS_READING_WARNING: c_uint = 0x1;
// enum: Critical threshold breached
pub const MC_CMD_DYNAMIC_SENSORS_READING_CRITICAL: c_uint = 0x2;
// enum: Fatal threshold breached
pub const MC_CMD_DYNAMIC_SENSORS_READING_FATAL: c_uint = 0x3;
// enum: Sensor not working
pub const MC_CMD_DYNAMIC_SENSORS_READING_BROKEN: c_uint = 0x4;
// enum: Sensor working but no reading available
pub const MC_CMD_DYNAMIC_SENSORS_READING_NO_READING: c_uint = 0x5;
// enum: Sensor initialization failed
pub const MC_CMD_DYNAMIC_SENSORS_READING_INIT_FAILED: c_uint = 0x6;
pub const MC_CMD_DYNAMIC_SENSORS_READING_STATE_LBN: c_int = 64;
pub const MC_CMD_DYNAMIC_SENSORS_READING_STATE_WIDTH: c_int = 32;
//
// MC_CMD_DYNAMIC_SENSORS_LIST
// Return a complete list of handles for sensors currently managed by the MC,
// and a generation count for this version of the sensor table. On systems
// advertising the DYNAMIC_SENSORS capability bit, this replaces the
// MC_CMD_READ_SENSORS command. On multi-MC systems this may include sensors
// added by the NMC.
//
// Sensor handles are persistent for the lifetime of the sensor and are used to
// identify sensors in MC_CMD_DYNAMIC_SENSORS_GET_DESCRIPTIONS and
// MC_CMD_DYNAMIC_SENSORS_GET_VALUES.
//
// The generation count is maintained by the MC, is persistent across reboots
// and will be incremented each time the sensor table is modified. When the
// table is modified, a CODE_DYNAMIC_SENSORS_CHANGE event will be generated
// containing the new generation count. The driver should compare this against
// the current generation count, and if it is different, call
// MC_CMD_DYNAMIC_SENSORS_LIST again to update it's copy of the sensor table.
//
// The sensor count is provided to allow a future path to supporting more than
// MC_CMD_DYNAMIC_SENSORS_GET_READINGS_IN_HANDLES_MAXNUM_MCDI2 sensors, i.e.
// the maximum number that will fit in a single response. As this is a fairly
// large number (253) it is not anticipated that this will be needed in the
// near future, so can currently be ignored.
//
// On Riverhead this command is implemented as a wrapper for `list` in the
// sensor_query SPHINX service.
//
pub const MC_CMD_DYNAMIC_SENSORS_LIST: c_uint = 0x66;

// MC_CMD_DYNAMIC_SENSORS_LIST_IN msgrequest
pub const MC_CMD_DYNAMIC_SENSORS_LIST_IN_LEN: c_int = 0;
// MC_CMD_DYNAMIC_SENSORS_LIST_OUT msgresponse
pub const MC_CMD_DYNAMIC_SENSORS_LIST_OUT_LENMIN: c_int = 8;
pub const MC_CMD_DYNAMIC_SENSORS_LIST_OUT_LENMAX: c_int = 252;
pub const MC_CMD_DYNAMIC_SENSORS_LIST_OUT_LENMAX_MCDI2: c_int = 1020;

// Generation count, which will be updated each time a sensor is added to or
// removed from the MC sensor table.
//
pub const MC_CMD_DYNAMIC_SENSORS_LIST_OUT_GENERATION_OFST: c_int = 0;
pub const MC_CMD_DYNAMIC_SENSORS_LIST_OUT_GENERATION_LEN: c_int = 4;
// Number of sensors managed by the MC. Note that in principle, this can be
// larger than the size of the HANDLES array.
//
pub const MC_CMD_DYNAMIC_SENSORS_LIST_OUT_COUNT_OFST: c_int = 4;
pub const MC_CMD_DYNAMIC_SENSORS_LIST_OUT_COUNT_LEN: c_int = 4;
// Array of sensor handles
pub const MC_CMD_DYNAMIC_SENSORS_LIST_OUT_HANDLES_OFST: c_int = 8;
pub const MC_CMD_DYNAMIC_SENSORS_LIST_OUT_HANDLES_LEN: c_int = 4;
pub const MC_CMD_DYNAMIC_SENSORS_LIST_OUT_HANDLES_MINNUM: c_int = 0;
pub const MC_CMD_DYNAMIC_SENSORS_LIST_OUT_HANDLES_MAXNUM: c_int = 61;
pub const MC_CMD_DYNAMIC_SENSORS_LIST_OUT_HANDLES_MAXNUM_MCDI2: c_int = 253;
//
// MC_CMD_DYNAMIC_SENSORS_GET_DESCRIPTIONS
// Get descriptions for a set of sensors, specified as an array of sensor
// handles as returned by MC_CMD_DYNAMIC_SENSORS_LIST
//
// Any handles which do not correspond to a sensor currently managed by the MC
// will be dropped from the response. This may happen when a sensor table
// update is in progress, and effectively means the set of usable sensors is
// the intersection between the sets of sensors known to the driver and the MC.
//
// On Riverhead this command is implemented as a wrapper for
// `get_descriptions` in the sensor_query SPHINX service.
//
pub const MC_CMD_DYNAMIC_SENSORS_GET_DESCRIPTIONS: c_uint = 0x67;

// MC_CMD_DYNAMIC_SENSORS_GET_DESCRIPTIONS_IN msgrequest
pub const MC_CMD_DYNAMIC_SENSORS_GET_DESCRIPTIONS_IN_LENMIN: c_int = 0;
pub const MC_CMD_DYNAMIC_SENSORS_GET_DESCRIPTIONS_IN_LENMAX: c_int = 252;
pub const MC_CMD_DYNAMIC_SENSORS_GET_DESCRIPTIONS_IN_LENMAX_MCDI2: c_int = 1020;

// Array of sensor handles
pub const MC_CMD_DYNAMIC_SENSORS_GET_DESCRIPTIONS_IN_HANDLES_OFST: c_int = 0;
pub const MC_CMD_DYNAMIC_SENSORS_GET_DESCRIPTIONS_IN_HANDLES_LEN: c_int = 4;
pub const MC_CMD_DYNAMIC_SENSORS_GET_DESCRIPTIONS_IN_HANDLES_MINNUM: c_int = 0;
pub const MC_CMD_DYNAMIC_SENSORS_GET_DESCRIPTIONS_IN_HANDLES_MAXNUM: c_int = 63;
pub const MC_CMD_DYNAMIC_SENSORS_GET_DESCRIPTIONS_IN_HANDLES_MAXNUM_MCDI2: c_int = 255;
// MC_CMD_DYNAMIC_SENSORS_GET_DESCRIPTIONS_OUT msgresponse
pub const MC_CMD_DYNAMIC_SENSORS_GET_DESCRIPTIONS_OUT_LENMIN: c_int = 0;
pub const MC_CMD_DYNAMIC_SENSORS_GET_DESCRIPTIONS_OUT_LENMAX: c_int = 192;
pub const MC_CMD_DYNAMIC_SENSORS_GET_DESCRIPTIONS_OUT_LENMAX_MCDI2: c_int = 960;

// Array of MC_CMD_DYNAMIC_SENSORS_DESCRIPTION structures
pub const MC_CMD_DYNAMIC_SENSORS_GET_DESCRIPTIONS_OUT_SENSORS_OFST: c_int = 0;
pub const MC_CMD_DYNAMIC_SENSORS_GET_DESCRIPTIONS_OUT_SENSORS_LEN: c_int = 64;
pub const MC_CMD_DYNAMIC_SENSORS_GET_DESCRIPTIONS_OUT_SENSORS_MINNUM: c_int = 0;
pub const MC_CMD_DYNAMIC_SENSORS_GET_DESCRIPTIONS_OUT_SENSORS_MAXNUM: c_int = 3;
pub const MC_CMD_DYNAMIC_SENSORS_GET_DESCRIPTIONS_OUT_SENSORS_MAXNUM_MCDI2: c_int = 15;
//
// MC_CMD_DYNAMIC_SENSORS_GET_READINGS
// Read the state and value for a set of sensors, specified as an array of
// sensor handles as returned by MC_CMD_DYNAMIC_SENSORS_LIST.
//
// In the case of a broken sensor, then the state of the response's
// MC_CMD_DYNAMIC_SENSORS_VALUE entry will be set to BROKEN, and any value
// provided should be treated as erroneous.
//
// Any handles which do not correspond to a sensor currently managed by the MC
// will be dropped from the response. This may happen when a sensor table
// update is in progress, and effectively means the set of usable sensors is
// the intersection between the sets of sensors known to the driver and the MC.
//
// On Riverhead this command is implemented as a wrapper for `get_readings`
// in the sensor_query SPHINX service.
//
pub const MC_CMD_DYNAMIC_SENSORS_GET_READINGS: c_uint = 0x68;

// MC_CMD_DYNAMIC_SENSORS_GET_READINGS_IN msgrequest
pub const MC_CMD_DYNAMIC_SENSORS_GET_READINGS_IN_LENMIN: c_int = 0;
pub const MC_CMD_DYNAMIC_SENSORS_GET_READINGS_IN_LENMAX: c_int = 252;
pub const MC_CMD_DYNAMIC_SENSORS_GET_READINGS_IN_LENMAX_MCDI2: c_int = 1020;

// Array of sensor handles
pub const MC_CMD_DYNAMIC_SENSORS_GET_READINGS_IN_HANDLES_OFST: c_int = 0;
pub const MC_CMD_DYNAMIC_SENSORS_GET_READINGS_IN_HANDLES_LEN: c_int = 4;
pub const MC_CMD_DYNAMIC_SENSORS_GET_READINGS_IN_HANDLES_MINNUM: c_int = 0;
pub const MC_CMD_DYNAMIC_SENSORS_GET_READINGS_IN_HANDLES_MAXNUM: c_int = 63;
pub const MC_CMD_DYNAMIC_SENSORS_GET_READINGS_IN_HANDLES_MAXNUM_MCDI2: c_int = 255;
// MC_CMD_DYNAMIC_SENSORS_GET_READINGS_OUT msgresponse
pub const MC_CMD_DYNAMIC_SENSORS_GET_READINGS_OUT_LENMIN: c_int = 0;
pub const MC_CMD_DYNAMIC_SENSORS_GET_READINGS_OUT_LENMAX: c_int = 252;
pub const MC_CMD_DYNAMIC_SENSORS_GET_READINGS_OUT_LENMAX_MCDI2: c_int = 1020;

// Array of MC_CMD_DYNAMIC_SENSORS_READING structures
pub const MC_CMD_DYNAMIC_SENSORS_GET_READINGS_OUT_VALUES_OFST: c_int = 0;
pub const MC_CMD_DYNAMIC_SENSORS_GET_READINGS_OUT_VALUES_LEN: c_int = 12;
pub const MC_CMD_DYNAMIC_SENSORS_GET_READINGS_OUT_VALUES_MINNUM: c_int = 0;
pub const MC_CMD_DYNAMIC_SENSORS_GET_READINGS_OUT_VALUES_MAXNUM: c_int = 21;
pub const MC_CMD_DYNAMIC_SENSORS_GET_READINGS_OUT_VALUES_MAXNUM_MCDI2: c_int = 85;
//
// MC_CMD_EVENT_CTRL
// Configure which categories of unsolicited events the driver expects to
// receive (Riverhead).
//
pub const MC_CMD_EVENT_CTRL: c_uint = 0x69;

// MC_CMD_EVENT_CTRL_IN msgrequest
pub const MC_CMD_EVENT_CTRL_IN_LENMIN: c_int = 0;
pub const MC_CMD_EVENT_CTRL_IN_LENMAX: c_int = 252;
pub const MC_CMD_EVENT_CTRL_IN_LENMAX_MCDI2: c_int = 1020;

// Array of event categories for which the driver wishes to receive events.
pub const MC_CMD_EVENT_CTRL_IN_EVENT_TYPE_OFST: c_int = 0;
pub const MC_CMD_EVENT_CTRL_IN_EVENT_TYPE_LEN: c_int = 4;
pub const MC_CMD_EVENT_CTRL_IN_EVENT_TYPE_MINNUM: c_int = 0;
pub const MC_CMD_EVENT_CTRL_IN_EVENT_TYPE_MAXNUM: c_int = 63;
pub const MC_CMD_EVENT_CTRL_IN_EVENT_TYPE_MAXNUM_MCDI2: c_int = 255;
// enum: Driver wishes to receive LINKCHANGE events.
pub const MC_CMD_EVENT_CTRL_IN_MCDI_EVENT_CODE_LINKCHANGE: c_uint = 0x0;
// enum: Driver wishes to receive SENSOR_CHANGE and SENSOR_STATE_CHANGE events.
//
pub const MC_CMD_EVENT_CTRL_IN_MCDI_EVENT_CODE_SENSOREVT: c_uint = 0x1;
// enum: Driver wishes to receive receive errors.
pub const MC_CMD_EVENT_CTRL_IN_MCDI_EVENT_CODE_RX_ERR: c_uint = 0x2;
// enum: Driver wishes to receive transmit errors.
pub const MC_CMD_EVENT_CTRL_IN_MCDI_EVENT_CODE_TX_ERR: c_uint = 0x3;
// enum: Driver wishes to receive firmware alerts.
pub const MC_CMD_EVENT_CTRL_IN_MCDI_EVENT_CODE_FWALERT: c_uint = 0x4;
// enum: Driver wishes to receive reboot events.
pub const MC_CMD_EVENT_CTRL_IN_MCDI_EVENT_CODE_MC_REBOOT: c_uint = 0x5;
// MC_CMD_EVENT_CTRL_OUT msgrequest
pub const MC_CMD_EVENT_CTRL_OUT_LEN: c_int = 0;
// EVB_PORT_ID structuredef
pub const EVB_PORT_ID_LEN: c_int = 4;
pub const EVB_PORT_ID_PORT_ID_OFST: c_int = 0;
pub const EVB_PORT_ID_PORT_ID_LEN: c_int = 4;
// enum: An invalid port handle.
pub const EVB_PORT_ID_NULL: c_uint = 0x0;
// enum: The port assigned to this function..
pub const EVB_PORT_ID_ASSIGNED: c_uint = 0x1000000;
// enum: External network port 0
pub const EVB_PORT_ID_MAC0: c_uint = 0x2000000;
// enum: External network port 1
pub const EVB_PORT_ID_MAC1: c_uint = 0x2000001;
// enum: External network port 2
pub const EVB_PORT_ID_MAC2: c_uint = 0x2000002;
// enum: External network port 3
pub const EVB_PORT_ID_MAC3: c_uint = 0x2000003;
pub const EVB_PORT_ID_PORT_ID_LBN: c_int = 0;
pub const EVB_PORT_ID_PORT_ID_WIDTH: c_int = 32;
// EVB_VLAN_TAG structuredef
pub const EVB_VLAN_TAG_LEN: c_int = 2;
// The VLAN tag value
pub const EVB_VLAN_TAG_VLAN_ID_LBN: c_int = 0;
pub const EVB_VLAN_TAG_VLAN_ID_WIDTH: c_int = 12;
pub const EVB_VLAN_TAG_MODE_LBN: c_int = 12;
pub const EVB_VLAN_TAG_MODE_WIDTH: c_int = 4;
// enum: Insert the VLAN.
pub const EVB_VLAN_TAG_INSERT: c_uint = 0x0;
// enum: Replace the VLAN if already present.
pub const EVB_VLAN_TAG_REPLACE: c_uint = 0x1;
// BUFTBL_ENTRY structuredef
pub const BUFTBL_ENTRY_LEN: c_int = 12;
// the owner ID
pub const BUFTBL_ENTRY_OID_OFST: c_int = 0;
pub const BUFTBL_ENTRY_OID_LEN: c_int = 2;
pub const BUFTBL_ENTRY_OID_LBN: c_int = 0;
pub const BUFTBL_ENTRY_OID_WIDTH: c_int = 16;
// the page parameter as one of ESE_DZ_SMC_PAGE_SIZE_
pub const BUFTBL_ENTRY_PGSZ_OFST: c_int = 2;
pub const BUFTBL_ENTRY_PGSZ_LEN: c_int = 2;
pub const BUFTBL_ENTRY_PGSZ_LBN: c_int = 16;
pub const BUFTBL_ENTRY_PGSZ_WIDTH: c_int = 16;
// the raw 64-bit address field from the SMC, not adjusted for page size
pub const BUFTBL_ENTRY_RAWADDR_OFST: c_int = 4;
pub const BUFTBL_ENTRY_RAWADDR_LEN: c_int = 8;
pub const BUFTBL_ENTRY_RAWADDR_LO_OFST: c_int = 4;
pub const BUFTBL_ENTRY_RAWADDR_HI_OFST: c_int = 8;
pub const BUFTBL_ENTRY_RAWADDR_LBN: c_int = 32;
pub const BUFTBL_ENTRY_RAWADDR_WIDTH: c_int = 64;
// NVRAM_PARTITION_TYPE structuredef
pub const NVRAM_PARTITION_TYPE_LEN: c_int = 2;
pub const NVRAM_PARTITION_TYPE_ID_OFST: c_int = 0;
pub const NVRAM_PARTITION_TYPE_ID_LEN: c_int = 2;
// enum: Primary MC firmware partition
pub const NVRAM_PARTITION_TYPE_MC_FIRMWARE: c_uint = 0x100;
// enum: Secondary MC firmware partition
pub const NVRAM_PARTITION_TYPE_MC_FIRMWARE_BACKUP: c_uint = 0x200;
// enum: Expansion ROM partition
pub const NVRAM_PARTITION_TYPE_EXPANSION_ROM: c_uint = 0x300;
// enum: Static configuration TLV partition
pub const NVRAM_PARTITION_TYPE_STATIC_CONFIG: c_uint = 0x400;
// enum: Dynamic configuration TLV partition
pub const NVRAM_PARTITION_TYPE_DYNAMIC_CONFIG: c_uint = 0x500;
// enum: Expansion ROM configuration data for port 0
pub const NVRAM_PARTITION_TYPE_EXPROM_CONFIG_PORT0: c_uint = 0x600;
// enum: Synonym for EXPROM_CONFIG_PORT0 as used in pmap files
pub const NVRAM_PARTITION_TYPE_EXPROM_CONFIG: c_uint = 0x600;
// enum: Expansion ROM configuration data for port 1
pub const NVRAM_PARTITION_TYPE_EXPROM_CONFIG_PORT1: c_uint = 0x601;
// enum: Expansion ROM configuration data for port 2
pub const NVRAM_PARTITION_TYPE_EXPROM_CONFIG_PORT2: c_uint = 0x602;
// enum: Expansion ROM configuration data for port 3
pub const NVRAM_PARTITION_TYPE_EXPROM_CONFIG_PORT3: c_uint = 0x603;
// enum: Non-volatile log output partition
pub const NVRAM_PARTITION_TYPE_LOG: c_uint = 0x700;
// enum: Non-volatile log output of second core on dual-core device
pub const NVRAM_PARTITION_TYPE_LOG_SLAVE: c_uint = 0x701;
// enum: Device state dump output partition
pub const NVRAM_PARTITION_TYPE_DUMP: c_uint = 0x800;
// enum: Application license key storage partition
pub const NVRAM_PARTITION_TYPE_LICENSE: c_uint = 0x900;
// enum: Start of range used for PHY partitions (low 8 bits are the PHY ID)
pub const NVRAM_PARTITION_TYPE_PHY_MIN: c_uint = 0xa00;
// enum: End of range used for PHY partitions (low 8 bits are the PHY ID)
pub const NVRAM_PARTITION_TYPE_PHY_MAX: c_uint = 0xaff;
// enum: Primary FPGA partition
pub const NVRAM_PARTITION_TYPE_FPGA: c_uint = 0xb00;
// enum: Secondary FPGA partition
pub const NVRAM_PARTITION_TYPE_FPGA_BACKUP: c_uint = 0xb01;
// enum: FC firmware partition
pub const NVRAM_PARTITION_TYPE_FC_FIRMWARE: c_uint = 0xb02;
// enum: FC License partition
pub const NVRAM_PARTITION_TYPE_FC_LICENSE: c_uint = 0xb03;
// enum: Non-volatile log output partition for FC
pub const NVRAM_PARTITION_TYPE_FC_LOG: c_uint = 0xb04;
// enum: MUM firmware partition
pub const NVRAM_PARTITION_TYPE_MUM_FIRMWARE: c_uint = 0xc00;
// enum: SUC firmware partition (this is intentionally an alias of
// MUM_FIRMWARE)
//
pub const NVRAM_PARTITION_TYPE_SUC_FIRMWARE: c_uint = 0xc00;
// enum: MUM Non-volatile log output partition.
pub const NVRAM_PARTITION_TYPE_MUM_LOG: c_uint = 0xc01;
// enum: MUM Application table partition.
pub const NVRAM_PARTITION_TYPE_MUM_APPTABLE: c_uint = 0xc02;
// enum: MUM boot rom partition.
pub const NVRAM_PARTITION_TYPE_MUM_BOOT_ROM: c_uint = 0xc03;
// enum: MUM production signatures & calibration rom partition.
pub const NVRAM_PARTITION_TYPE_MUM_PROD_ROM: c_uint = 0xc04;
// enum: MUM user signatures & calibration rom partition.
pub const NVRAM_PARTITION_TYPE_MUM_USER_ROM: c_uint = 0xc05;
// enum: MUM fuses and lockbits partition.
pub const NVRAM_PARTITION_TYPE_MUM_FUSELOCK: c_uint = 0xc06;
// enum: UEFI expansion ROM if separate from PXE
pub const NVRAM_PARTITION_TYPE_EXPANSION_UEFI: c_uint = 0xd00;
// enum: Used by the expansion ROM for logging
pub const NVRAM_PARTITION_TYPE_PXE_LOG: c_uint = 0x1000;
// enum: Used for XIP code of shmbooted images
pub const NVRAM_PARTITION_TYPE_XIP_SCRATCH: c_uint = 0x1100;
// enum: Spare partition 2
pub const NVRAM_PARTITION_TYPE_SPARE_2: c_uint = 0x1200;
// enum: Manufacturing partition. Used during manufacture to pass information
// between XJTAG and Manftest.
//
pub const NVRAM_PARTITION_TYPE_MANUFACTURING: c_uint = 0x1300;
// enum: Spare partition 4
pub const NVRAM_PARTITION_TYPE_SPARE_4: c_uint = 0x1400;
// enum: Spare partition 5
pub const NVRAM_PARTITION_TYPE_SPARE_5: c_uint = 0x1500;
// enum: Partition for reporting MC status. See mc_flash_layout.h
// medford_mc_status_hdr_t for layout on Medford.
//
pub const NVRAM_PARTITION_TYPE_STATUS: c_uint = 0x1600;
// enum: Spare partition 13
pub const NVRAM_PARTITION_TYPE_SPARE_13: c_uint = 0x1700;
// enum: Spare partition 14
pub const NVRAM_PARTITION_TYPE_SPARE_14: c_uint = 0x1800;
// enum: Spare partition 15
pub const NVRAM_PARTITION_TYPE_SPARE_15: c_uint = 0x1900;
// enum: Spare partition 16
pub const NVRAM_PARTITION_TYPE_SPARE_16: c_uint = 0x1a00;
// enum: Factory defaults for dynamic configuration
pub const NVRAM_PARTITION_TYPE_DYNCONFIG_DEFAULTS: c_uint = 0x1b00;
// enum: Factory defaults for expansion ROM configuration
pub const NVRAM_PARTITION_TYPE_ROMCONFIG_DEFAULTS: c_uint = 0x1c00;
// enum: Field Replaceable Unit inventory information for use on IPMI
// platforms. See SF-119124-PS. The STATIC_CONFIG partition may contain a
// subset of the information stored in this partition.
//
pub const NVRAM_PARTITION_TYPE_FRU_INFORMATION: c_uint = 0x1d00;
// enum: Bundle image partition
pub const NVRAM_PARTITION_TYPE_BUNDLE: c_uint = 0x1e00;
// enum: Bundle metadata partition that holds additional information related to
// a bundle update in TLV format
//
pub const NVRAM_PARTITION_TYPE_BUNDLE_METADATA: c_uint = 0x1e01;
// enum: Bundle update non-volatile log output partition
pub const NVRAM_PARTITION_TYPE_BUNDLE_LOG: c_uint = 0x1e02;
// enum: Partition for Solarflare gPXE bootrom installed via Bundle update.
pub const NVRAM_PARTITION_TYPE_EXPANSION_ROM_INTERNAL: c_uint = 0x1e03;
// enum: Start of reserved value range (firmware may use for any purpose)
pub const NVRAM_PARTITION_TYPE_RESERVED_VALUES_MIN: c_uint = 0xff00;
// enum: End of reserved value range (firmware may use for any purpose)
pub const NVRAM_PARTITION_TYPE_RESERVED_VALUES_MAX: c_uint = 0xfffd;
// enum: Recovery partition map (provided if real map is missing or corrupt)
pub const NVRAM_PARTITION_TYPE_RECOVERY_MAP: c_uint = 0xfffe;
// enum: Partition map (real map as stored in flash)
pub const NVRAM_PARTITION_TYPE_PARTITION_MAP: c_uint = 0xffff;
pub const NVRAM_PARTITION_TYPE_ID_LBN: c_int = 0;
pub const NVRAM_PARTITION_TYPE_ID_WIDTH: c_int = 16;
// LICENSED_APP_ID structuredef
pub const LICENSED_APP_ID_LEN: c_int = 4;
pub const LICENSED_APP_ID_ID_OFST: c_int = 0;
pub const LICENSED_APP_ID_ID_LEN: c_int = 4;
// enum: OpenOnload
pub const LICENSED_APP_ID_ONLOAD: c_uint = 0x1;
// enum: PTP timestamping
pub const LICENSED_APP_ID_PTP: c_uint = 0x2;
// enum: SolarCapture Pro
pub const LICENSED_APP_ID_SOLARCAPTURE_PRO: c_uint = 0x4;
// enum: SolarSecure filter engine
pub const LICENSED_APP_ID_SOLARSECURE: c_uint = 0x8;
// enum: Performance monitor
pub const LICENSED_APP_ID_PERF_MONITOR: c_uint = 0x10;
// enum: SolarCapture Live
pub const LICENSED_APP_ID_SOLARCAPTURE_LIVE: c_uint = 0x20;
// enum: Capture SolarSystem
pub const LICENSED_APP_ID_CAPTURE_SOLARSYSTEM: c_uint = 0x40;
// enum: Network Access Control
pub const LICENSED_APP_ID_NETWORK_ACCESS_CONTROL: c_uint = 0x80;
// enum: TCP Direct
pub const LICENSED_APP_ID_TCP_DIRECT: c_uint = 0x100;
// enum: Low Latency
pub const LICENSED_APP_ID_LOW_LATENCY: c_uint = 0x200;
// enum: SolarCapture Tap
pub const LICENSED_APP_ID_SOLARCAPTURE_TAP: c_uint = 0x400;
// enum: Capture SolarSystem 40G
pub const LICENSED_APP_ID_CAPTURE_SOLARSYSTEM_40G: c_uint = 0x800;
// enum: Capture SolarSystem 1G
pub const LICENSED_APP_ID_CAPTURE_SOLARSYSTEM_1G: c_uint = 0x1000;
// enum: ScaleOut Onload
pub const LICENSED_APP_ID_SCALEOUT_ONLOAD: c_uint = 0x2000;
// enum: SCS Network Analytics Dashboard
pub const LICENSED_APP_ID_DSHBRD: c_uint = 0x4000;
// enum: SolarCapture Trading Analytics
pub const LICENSED_APP_ID_SCATRD: c_uint = 0x8000;
pub const LICENSED_APP_ID_ID_LBN: c_int = 0;
pub const LICENSED_APP_ID_ID_WIDTH: c_int = 32;
// LICENSED_FEATURES structuredef
pub const LICENSED_FEATURES_LEN: c_int = 8;
// Bitmask of licensed firmware features
pub const LICENSED_FEATURES_MASK_OFST: c_int = 0;
pub const LICENSED_FEATURES_MASK_LEN: c_int = 8;
pub const LICENSED_FEATURES_MASK_LO_OFST: c_int = 0;
pub const LICENSED_FEATURES_MASK_HI_OFST: c_int = 4;
pub const LICENSED_FEATURES_RX_CUT_THROUGH_OFST: c_int = 0;
pub const LICENSED_FEATURES_RX_CUT_THROUGH_LBN: c_int = 0;
pub const LICENSED_FEATURES_RX_CUT_THROUGH_WIDTH: c_int = 1;
pub const LICENSED_FEATURES_PIO_OFST: c_int = 0;
pub const LICENSED_FEATURES_PIO_LBN: c_int = 1;
pub const LICENSED_FEATURES_PIO_WIDTH: c_int = 1;
pub const LICENSED_FEATURES_EVQ_TIMER_OFST: c_int = 0;
pub const LICENSED_FEATURES_EVQ_TIMER_LBN: c_int = 2;
pub const LICENSED_FEATURES_EVQ_TIMER_WIDTH: c_int = 1;
pub const LICENSED_FEATURES_CLOCK_OFST: c_int = 0;
pub const LICENSED_FEATURES_CLOCK_LBN: c_int = 3;
pub const LICENSED_FEATURES_CLOCK_WIDTH: c_int = 1;
pub const LICENSED_FEATURES_RX_TIMESTAMPS_OFST: c_int = 0;
pub const LICENSED_FEATURES_RX_TIMESTAMPS_LBN: c_int = 4;
pub const LICENSED_FEATURES_RX_TIMESTAMPS_WIDTH: c_int = 1;
pub const LICENSED_FEATURES_TX_TIMESTAMPS_OFST: c_int = 0;
pub const LICENSED_FEATURES_TX_TIMESTAMPS_LBN: c_int = 5;
pub const LICENSED_FEATURES_TX_TIMESTAMPS_WIDTH: c_int = 1;
pub const LICENSED_FEATURES_RX_SNIFF_OFST: c_int = 0;
pub const LICENSED_FEATURES_RX_SNIFF_LBN: c_int = 6;
pub const LICENSED_FEATURES_RX_SNIFF_WIDTH: c_int = 1;
pub const LICENSED_FEATURES_TX_SNIFF_OFST: c_int = 0;
pub const LICENSED_FEATURES_TX_SNIFF_LBN: c_int = 7;
pub const LICENSED_FEATURES_TX_SNIFF_WIDTH: c_int = 1;
pub const LICENSED_FEATURES_PROXY_FILTER_OPS_OFST: c_int = 0;
pub const LICENSED_FEATURES_PROXY_FILTER_OPS_LBN: c_int = 8;
pub const LICENSED_FEATURES_PROXY_FILTER_OPS_WIDTH: c_int = 1;
pub const LICENSED_FEATURES_EVENT_CUT_THROUGH_OFST: c_int = 0;
pub const LICENSED_FEATURES_EVENT_CUT_THROUGH_LBN: c_int = 9;
pub const LICENSED_FEATURES_EVENT_CUT_THROUGH_WIDTH: c_int = 1;
pub const LICENSED_FEATURES_MASK_LBN: c_int = 0;
pub const LICENSED_FEATURES_MASK_WIDTH: c_int = 64;
// LICENSED_V3_APPS structuredef
pub const LICENSED_V3_APPS_LEN: c_int = 8;
// Bitmask of licensed applications
pub const LICENSED_V3_APPS_MASK_OFST: c_int = 0;
pub const LICENSED_V3_APPS_MASK_LEN: c_int = 8;
pub const LICENSED_V3_APPS_MASK_LO_OFST: c_int = 0;
pub const LICENSED_V3_APPS_MASK_HI_OFST: c_int = 4;
pub const LICENSED_V3_APPS_ONLOAD_OFST: c_int = 0;
pub const LICENSED_V3_APPS_ONLOAD_LBN: c_int = 0;
pub const LICENSED_V3_APPS_ONLOAD_WIDTH: c_int = 1;
pub const LICENSED_V3_APPS_PTP_OFST: c_int = 0;
pub const LICENSED_V3_APPS_PTP_LBN: c_int = 1;
pub const LICENSED_V3_APPS_PTP_WIDTH: c_int = 1;
pub const LICENSED_V3_APPS_SOLARCAPTURE_PRO_OFST: c_int = 0;
pub const LICENSED_V3_APPS_SOLARCAPTURE_PRO_LBN: c_int = 2;
pub const LICENSED_V3_APPS_SOLARCAPTURE_PRO_WIDTH: c_int = 1;
pub const LICENSED_V3_APPS_SOLARSECURE_OFST: c_int = 0;
pub const LICENSED_V3_APPS_SOLARSECURE_LBN: c_int = 3;
pub const LICENSED_V3_APPS_SOLARSECURE_WIDTH: c_int = 1;
pub const LICENSED_V3_APPS_PERF_MONITOR_OFST: c_int = 0;
pub const LICENSED_V3_APPS_PERF_MONITOR_LBN: c_int = 4;
pub const LICENSED_V3_APPS_PERF_MONITOR_WIDTH: c_int = 1;
pub const LICENSED_V3_APPS_SOLARCAPTURE_LIVE_OFST: c_int = 0;
pub const LICENSED_V3_APPS_SOLARCAPTURE_LIVE_LBN: c_int = 5;
pub const LICENSED_V3_APPS_SOLARCAPTURE_LIVE_WIDTH: c_int = 1;
pub const LICENSED_V3_APPS_CAPTURE_SOLARSYSTEM_OFST: c_int = 0;
pub const LICENSED_V3_APPS_CAPTURE_SOLARSYSTEM_LBN: c_int = 6;
pub const LICENSED_V3_APPS_CAPTURE_SOLARSYSTEM_WIDTH: c_int = 1;
pub const LICENSED_V3_APPS_NETWORK_ACCESS_CONTROL_OFST: c_int = 0;
pub const LICENSED_V3_APPS_NETWORK_ACCESS_CONTROL_LBN: c_int = 7;
pub const LICENSED_V3_APPS_NETWORK_ACCESS_CONTROL_WIDTH: c_int = 1;
pub const LICENSED_V3_APPS_TCP_DIRECT_OFST: c_int = 0;
pub const LICENSED_V3_APPS_TCP_DIRECT_LBN: c_int = 8;
pub const LICENSED_V3_APPS_TCP_DIRECT_WIDTH: c_int = 1;
pub const LICENSED_V3_APPS_LOW_LATENCY_OFST: c_int = 0;
pub const LICENSED_V3_APPS_LOW_LATENCY_LBN: c_int = 9;
pub const LICENSED_V3_APPS_LOW_LATENCY_WIDTH: c_int = 1;
pub const LICENSED_V3_APPS_SOLARCAPTURE_TAP_OFST: c_int = 0;
pub const LICENSED_V3_APPS_SOLARCAPTURE_TAP_LBN: c_int = 10;
pub const LICENSED_V3_APPS_SOLARCAPTURE_TAP_WIDTH: c_int = 1;
pub const LICENSED_V3_APPS_CAPTURE_SOLARSYSTEM_40G_OFST: c_int = 0;
pub const LICENSED_V3_APPS_CAPTURE_SOLARSYSTEM_40G_LBN: c_int = 11;
pub const LICENSED_V3_APPS_CAPTURE_SOLARSYSTEM_40G_WIDTH: c_int = 1;
pub const LICENSED_V3_APPS_CAPTURE_SOLARSYSTEM_1G_OFST: c_int = 0;
pub const LICENSED_V3_APPS_CAPTURE_SOLARSYSTEM_1G_LBN: c_int = 12;
pub const LICENSED_V3_APPS_CAPTURE_SOLARSYSTEM_1G_WIDTH: c_int = 1;
pub const LICENSED_V3_APPS_SCALEOUT_ONLOAD_OFST: c_int = 0;
pub const LICENSED_V3_APPS_SCALEOUT_ONLOAD_LBN: c_int = 13;
pub const LICENSED_V3_APPS_SCALEOUT_ONLOAD_WIDTH: c_int = 1;
pub const LICENSED_V3_APPS_DSHBRD_OFST: c_int = 0;
pub const LICENSED_V3_APPS_DSHBRD_LBN: c_int = 14;
pub const LICENSED_V3_APPS_DSHBRD_WIDTH: c_int = 1;
pub const LICENSED_V3_APPS_SCATRD_OFST: c_int = 0;
pub const LICENSED_V3_APPS_SCATRD_LBN: c_int = 15;
pub const LICENSED_V3_APPS_SCATRD_WIDTH: c_int = 1;
pub const LICENSED_V3_APPS_MASK_LBN: c_int = 0;
pub const LICENSED_V3_APPS_MASK_WIDTH: c_int = 64;
// LICENSED_V3_FEATURES structuredef
pub const LICENSED_V3_FEATURES_LEN: c_int = 8;
// Bitmask of licensed firmware features
pub const LICENSED_V3_FEATURES_MASK_OFST: c_int = 0;
pub const LICENSED_V3_FEATURES_MASK_LEN: c_int = 8;
pub const LICENSED_V3_FEATURES_MASK_LO_OFST: c_int = 0;
pub const LICENSED_V3_FEATURES_MASK_HI_OFST: c_int = 4;
pub const LICENSED_V3_FEATURES_RX_CUT_THROUGH_OFST: c_int = 0;
pub const LICENSED_V3_FEATURES_RX_CUT_THROUGH_LBN: c_int = 0;
pub const LICENSED_V3_FEATURES_RX_CUT_THROUGH_WIDTH: c_int = 1;
pub const LICENSED_V3_FEATURES_PIO_OFST: c_int = 0;
pub const LICENSED_V3_FEATURES_PIO_LBN: c_int = 1;
pub const LICENSED_V3_FEATURES_PIO_WIDTH: c_int = 1;
pub const LICENSED_V3_FEATURES_EVQ_TIMER_OFST: c_int = 0;
pub const LICENSED_V3_FEATURES_EVQ_TIMER_LBN: c_int = 2;
pub const LICENSED_V3_FEATURES_EVQ_TIMER_WIDTH: c_int = 1;
pub const LICENSED_V3_FEATURES_CLOCK_OFST: c_int = 0;
pub const LICENSED_V3_FEATURES_CLOCK_LBN: c_int = 3;
pub const LICENSED_V3_FEATURES_CLOCK_WIDTH: c_int = 1;
pub const LICENSED_V3_FEATURES_RX_TIMESTAMPS_OFST: c_int = 0;
pub const LICENSED_V3_FEATURES_RX_TIMESTAMPS_LBN: c_int = 4;
pub const LICENSED_V3_FEATURES_RX_TIMESTAMPS_WIDTH: c_int = 1;
pub const LICENSED_V3_FEATURES_TX_TIMESTAMPS_OFST: c_int = 0;
pub const LICENSED_V3_FEATURES_TX_TIMESTAMPS_LBN: c_int = 5;
pub const LICENSED_V3_FEATURES_TX_TIMESTAMPS_WIDTH: c_int = 1;
pub const LICENSED_V3_FEATURES_RX_SNIFF_OFST: c_int = 0;
pub const LICENSED_V3_FEATURES_RX_SNIFF_LBN: c_int = 6;
pub const LICENSED_V3_FEATURES_RX_SNIFF_WIDTH: c_int = 1;
pub const LICENSED_V3_FEATURES_TX_SNIFF_OFST: c_int = 0;
pub const LICENSED_V3_FEATURES_TX_SNIFF_LBN: c_int = 7;
pub const LICENSED_V3_FEATURES_TX_SNIFF_WIDTH: c_int = 1;
pub const LICENSED_V3_FEATURES_PROXY_FILTER_OPS_OFST: c_int = 0;
pub const LICENSED_V3_FEATURES_PROXY_FILTER_OPS_LBN: c_int = 8;
pub const LICENSED_V3_FEATURES_PROXY_FILTER_OPS_WIDTH: c_int = 1;
pub const LICENSED_V3_FEATURES_EVENT_CUT_THROUGH_OFST: c_int = 0;
pub const LICENSED_V3_FEATURES_EVENT_CUT_THROUGH_LBN: c_int = 9;
pub const LICENSED_V3_FEATURES_EVENT_CUT_THROUGH_WIDTH: c_int = 1;
pub const LICENSED_V3_FEATURES_MASK_LBN: c_int = 0;
pub const LICENSED_V3_FEATURES_MASK_WIDTH: c_int = 64;
// TX_TIMESTAMP_EVENT structuredef
pub const TX_TIMESTAMP_EVENT_LEN: c_int = 6;
// lower 16 bits of timestamp data
pub const TX_TIMESTAMP_EVENT_TSTAMP_DATA_LO_OFST: c_int = 0;
pub const TX_TIMESTAMP_EVENT_TSTAMP_DATA_LO_LEN: c_int = 2;
pub const TX_TIMESTAMP_EVENT_TSTAMP_DATA_LO_LBN: c_int = 0;
pub const TX_TIMESTAMP_EVENT_TSTAMP_DATA_LO_WIDTH: c_int = 16;
// Type of TX event, ordinary TX completion, low or high part of TX timestamp
//
pub const TX_TIMESTAMP_EVENT_TX_EV_TYPE_OFST: c_int = 3;
pub const TX_TIMESTAMP_EVENT_TX_EV_TYPE_LEN: c_int = 1;
// enum: This is a TX completion event, not a timestamp
pub const TX_TIMESTAMP_EVENT_TX_EV_COMPLETION: c_uint = 0x0;
// enum: This is a TX completion event for a CTPIO transmit. The event format
// is the same as for TX_EV_COMPLETION.
//
pub const TX_TIMESTAMP_EVENT_TX_EV_CTPIO_COMPLETION: c_uint = 0x11;
// enum: This is the low part of a TX timestamp for a CTPIO transmission. The
// event format is the same as for TX_EV_TSTAMP_LO
//
pub const TX_TIMESTAMP_EVENT_TX_EV_CTPIO_TS_LO: c_uint = 0x12;
// enum: This is the high part of a TX timestamp for a CTPIO transmission. The
// event format is the same as for TX_EV_TSTAMP_HI
//
pub const TX_TIMESTAMP_EVENT_TX_EV_CTPIO_TS_HI: c_uint = 0x13;
// enum: This is the low part of a TX timestamp event
pub const TX_TIMESTAMP_EVENT_TX_EV_TSTAMP_LO: c_uint = 0x51;
// enum: This is the high part of a TX timestamp event
pub const TX_TIMESTAMP_EVENT_TX_EV_TSTAMP_HI: c_uint = 0x52;
pub const TX_TIMESTAMP_EVENT_TX_EV_TYPE_LBN: c_int = 24;
pub const TX_TIMESTAMP_EVENT_TX_EV_TYPE_WIDTH: c_int = 8;
// upper 16 bits of timestamp data
pub const TX_TIMESTAMP_EVENT_TSTAMP_DATA_HI_OFST: c_int = 4;
pub const TX_TIMESTAMP_EVENT_TSTAMP_DATA_HI_LEN: c_int = 2;
pub const TX_TIMESTAMP_EVENT_TSTAMP_DATA_HI_LBN: c_int = 32;
pub const TX_TIMESTAMP_EVENT_TSTAMP_DATA_HI_WIDTH: c_int = 16;
// RSS_MODE structuredef
pub const RSS_MODE_LEN: c_int = 1;
// The RSS mode for a particular packet type is a value from 0 - 15 which can
// be considered as 4 bits selecting which fields are included in the hash. (A
// value 0 effectively disables RSS spreading for the packet type.) The YAML
// generation tools require this structure to be a whole number of bytes wide,
// but only 4 bits are relevant.
//
pub const RSS_MODE_HASH_SELECTOR_OFST: c_int = 0;
pub const RSS_MODE_HASH_SELECTOR_LEN: c_int = 1;
pub const RSS_MODE_HASH_SRC_ADDR_OFST: c_int = 0;
pub const RSS_MODE_HASH_SRC_ADDR_LBN: c_int = 0;
pub const RSS_MODE_HASH_SRC_ADDR_WIDTH: c_int = 1;
pub const RSS_MODE_HASH_DST_ADDR_OFST: c_int = 0;
pub const RSS_MODE_HASH_DST_ADDR_LBN: c_int = 1;
pub const RSS_MODE_HASH_DST_ADDR_WIDTH: c_int = 1;
pub const RSS_MODE_HASH_SRC_PORT_OFST: c_int = 0;
pub const RSS_MODE_HASH_SRC_PORT_LBN: c_int = 2;
pub const RSS_MODE_HASH_SRC_PORT_WIDTH: c_int = 1;
pub const RSS_MODE_HASH_DST_PORT_OFST: c_int = 0;
pub const RSS_MODE_HASH_DST_PORT_LBN: c_int = 3;
pub const RSS_MODE_HASH_DST_PORT_WIDTH: c_int = 1;
pub const RSS_MODE_HASH_SELECTOR_LBN: c_int = 0;
pub const RSS_MODE_HASH_SELECTOR_WIDTH: c_int = 8;
// CTPIO_STATS_MAP structuredef
pub const CTPIO_STATS_MAP_LEN: c_int = 4;
// The (function relative) VI number
pub const CTPIO_STATS_MAP_VI_OFST: c_int = 0;
pub const CTPIO_STATS_MAP_VI_LEN: c_int = 2;
pub const CTPIO_STATS_MAP_VI_LBN: c_int = 0;
pub const CTPIO_STATS_MAP_VI_WIDTH: c_int = 16;
// The target bucket for the VI
pub const CTPIO_STATS_MAP_BUCKET_OFST: c_int = 2;
pub const CTPIO_STATS_MAP_BUCKET_LEN: c_int = 2;
pub const CTPIO_STATS_MAP_BUCKET_LBN: c_int = 16;
pub const CTPIO_STATS_MAP_BUCKET_WIDTH: c_int = 16;
//
// MC_CMD_READ_REGS
// Get a dump of the MCPU registers
//
pub const MC_CMD_READ_REGS: c_uint = 0x50;

// MC_CMD_READ_REGS_IN msgrequest
pub const MC_CMD_READ_REGS_IN_LEN: c_int = 0;
// MC_CMD_READ_REGS_OUT msgresponse
pub const MC_CMD_READ_REGS_OUT_LEN: c_int = 308;
// Whether the corresponding register entry contains a valid value
pub const MC_CMD_READ_REGS_OUT_MASK_OFST: c_int = 0;
pub const MC_CMD_READ_REGS_OUT_MASK_LEN: c_int = 16;
// Same order as MIPS GDB (r0-r31, sr, lo, hi, bad, cause, 32 x float, fsr,
// fir, fp)
//
pub const MC_CMD_READ_REGS_OUT_REGS_OFST: c_int = 16;
pub const MC_CMD_READ_REGS_OUT_REGS_LEN: c_int = 4;
pub const MC_CMD_READ_REGS_OUT_REGS_NUM: c_int = 73;
//
// MC_CMD_INIT_EVQ
// Set up an event queue according to the supplied parameters. The IN arguments
// end with an address for each 4k of host memory required to back the EVQ.
//
pub const MC_CMD_INIT_EVQ: c_uint = 0x80;

// MC_CMD_INIT_EVQ_IN msgrequest
pub const MC_CMD_INIT_EVQ_IN_LENMIN: c_int = 44;
pub const MC_CMD_INIT_EVQ_IN_LENMAX: c_int = 548;
pub const MC_CMD_INIT_EVQ_IN_LENMAX_MCDI2: c_int = 548;

// Size, in entries
pub const MC_CMD_INIT_EVQ_IN_SIZE_OFST: c_int = 0;
pub const MC_CMD_INIT_EVQ_IN_SIZE_LEN: c_int = 4;
// Desired instance. Must be set to a specific instance, which is a function
// local queue index.
//
pub const MC_CMD_INIT_EVQ_IN_INSTANCE_OFST: c_int = 4;
pub const MC_CMD_INIT_EVQ_IN_INSTANCE_LEN: c_int = 4;
// The initial timer value. The load value is ignored if the timer mode is DIS.
//
pub const MC_CMD_INIT_EVQ_IN_TMR_LOAD_OFST: c_int = 8;
pub const MC_CMD_INIT_EVQ_IN_TMR_LOAD_LEN: c_int = 4;
// The reload value is ignored in one-shot modes
pub const MC_CMD_INIT_EVQ_IN_TMR_RELOAD_OFST: c_int = 12;
pub const MC_CMD_INIT_EVQ_IN_TMR_RELOAD_LEN: c_int = 4;
// tbd
pub const MC_CMD_INIT_EVQ_IN_FLAGS_OFST: c_int = 16;
pub const MC_CMD_INIT_EVQ_IN_FLAGS_LEN: c_int = 4;
pub const MC_CMD_INIT_EVQ_IN_FLAG_INTERRUPTING_OFST: c_int = 16;
pub const MC_CMD_INIT_EVQ_IN_FLAG_INTERRUPTING_LBN: c_int = 0;
pub const MC_CMD_INIT_EVQ_IN_FLAG_INTERRUPTING_WIDTH: c_int = 1;
pub const MC_CMD_INIT_EVQ_IN_FLAG_RPTR_DOS_OFST: c_int = 16;
pub const MC_CMD_INIT_EVQ_IN_FLAG_RPTR_DOS_LBN: c_int = 1;
pub const MC_CMD_INIT_EVQ_IN_FLAG_RPTR_DOS_WIDTH: c_int = 1;
pub const MC_CMD_INIT_EVQ_IN_FLAG_INT_ARMD_OFST: c_int = 16;
pub const MC_CMD_INIT_EVQ_IN_FLAG_INT_ARMD_LBN: c_int = 2;
pub const MC_CMD_INIT_EVQ_IN_FLAG_INT_ARMD_WIDTH: c_int = 1;
pub const MC_CMD_INIT_EVQ_IN_FLAG_CUT_THRU_OFST: c_int = 16;
pub const MC_CMD_INIT_EVQ_IN_FLAG_CUT_THRU_LBN: c_int = 3;
pub const MC_CMD_INIT_EVQ_IN_FLAG_CUT_THRU_WIDTH: c_int = 1;
pub const MC_CMD_INIT_EVQ_IN_FLAG_RX_MERGE_OFST: c_int = 16;
pub const MC_CMD_INIT_EVQ_IN_FLAG_RX_MERGE_LBN: c_int = 4;
pub const MC_CMD_INIT_EVQ_IN_FLAG_RX_MERGE_WIDTH: c_int = 1;
pub const MC_CMD_INIT_EVQ_IN_FLAG_TX_MERGE_OFST: c_int = 16;
pub const MC_CMD_INIT_EVQ_IN_FLAG_TX_MERGE_LBN: c_int = 5;
pub const MC_CMD_INIT_EVQ_IN_FLAG_TX_MERGE_WIDTH: c_int = 1;
pub const MC_CMD_INIT_EVQ_IN_FLAG_USE_TIMER_OFST: c_int = 16;
pub const MC_CMD_INIT_EVQ_IN_FLAG_USE_TIMER_LBN: c_int = 6;
pub const MC_CMD_INIT_EVQ_IN_FLAG_USE_TIMER_WIDTH: c_int = 1;
pub const MC_CMD_INIT_EVQ_IN_TMR_MODE_OFST: c_int = 20;
pub const MC_CMD_INIT_EVQ_IN_TMR_MODE_LEN: c_int = 4;
// enum: Disabled
pub const MC_CMD_INIT_EVQ_IN_TMR_MODE_DIS: c_uint = 0x0;
// enum: Immediate
pub const MC_CMD_INIT_EVQ_IN_TMR_IMMED_START: c_uint = 0x1;
// enum: Triggered
pub const MC_CMD_INIT_EVQ_IN_TMR_TRIG_START: c_uint = 0x2;
// enum: Hold-off
pub const MC_CMD_INIT_EVQ_IN_TMR_INT_HLDOFF: c_uint = 0x3;
// Target EVQ for wakeups if in wakeup mode.
pub const MC_CMD_INIT_EVQ_IN_TARGET_EVQ_OFST: c_int = 24;
pub const MC_CMD_INIT_EVQ_IN_TARGET_EVQ_LEN: c_int = 4;
// Target interrupt if in interrupting mode (note union with target EVQ). Use
// MC_CMD_RESOURCE_INSTANCE_ANY unless a specific one required for test
// purposes.
//
pub const MC_CMD_INIT_EVQ_IN_IRQ_NUM_OFST: c_int = 24;
pub const MC_CMD_INIT_EVQ_IN_IRQ_NUM_LEN: c_int = 4;
// Event Counter Mode.
pub const MC_CMD_INIT_EVQ_IN_COUNT_MODE_OFST: c_int = 28;
pub const MC_CMD_INIT_EVQ_IN_COUNT_MODE_LEN: c_int = 4;
// enum: Disabled
pub const MC_CMD_INIT_EVQ_IN_COUNT_MODE_DIS: c_uint = 0x0;
// enum: Disabled
pub const MC_CMD_INIT_EVQ_IN_COUNT_MODE_RX: c_uint = 0x1;
// enum: Disabled
pub const MC_CMD_INIT_EVQ_IN_COUNT_MODE_TX: c_uint = 0x2;
// enum: Disabled
pub const MC_CMD_INIT_EVQ_IN_COUNT_MODE_RXTX: c_uint = 0x3;
// Event queue packet count threshold.
pub const MC_CMD_INIT_EVQ_IN_COUNT_THRSHLD_OFST: c_int = 32;
pub const MC_CMD_INIT_EVQ_IN_COUNT_THRSHLD_LEN: c_int = 4;
// 64-bit address of 4k of 4k-aligned host memory buffer
pub const MC_CMD_INIT_EVQ_IN_DMA_ADDR_OFST: c_int = 36;
pub const MC_CMD_INIT_EVQ_IN_DMA_ADDR_LEN: c_int = 8;
pub const MC_CMD_INIT_EVQ_IN_DMA_ADDR_LO_OFST: c_int = 36;
pub const MC_CMD_INIT_EVQ_IN_DMA_ADDR_HI_OFST: c_int = 40;
pub const MC_CMD_INIT_EVQ_IN_DMA_ADDR_MINNUM: c_int = 1;
pub const MC_CMD_INIT_EVQ_IN_DMA_ADDR_MAXNUM: c_int = 64;
pub const MC_CMD_INIT_EVQ_IN_DMA_ADDR_MAXNUM_MCDI2: c_int = 64;
// MC_CMD_INIT_EVQ_OUT msgresponse
pub const MC_CMD_INIT_EVQ_OUT_LEN: c_int = 4;
// Only valid if INTRFLAG was true
pub const MC_CMD_INIT_EVQ_OUT_IRQ_OFST: c_int = 0;
pub const MC_CMD_INIT_EVQ_OUT_IRQ_LEN: c_int = 4;
// MC_CMD_INIT_EVQ_V2_IN msgrequest
pub const MC_CMD_INIT_EVQ_V2_IN_LENMIN: c_int = 44;
pub const MC_CMD_INIT_EVQ_V2_IN_LENMAX: c_int = 548;
pub const MC_CMD_INIT_EVQ_V2_IN_LENMAX_MCDI2: c_int = 548;

// Size, in entries
pub const MC_CMD_INIT_EVQ_V2_IN_SIZE_OFST: c_int = 0;
pub const MC_CMD_INIT_EVQ_V2_IN_SIZE_LEN: c_int = 4;
// Desired instance. Must be set to a specific instance, which is a function
// local queue index.
//
pub const MC_CMD_INIT_EVQ_V2_IN_INSTANCE_OFST: c_int = 4;
pub const MC_CMD_INIT_EVQ_V2_IN_INSTANCE_LEN: c_int = 4;
// The initial timer value. The load value is ignored if the timer mode is DIS.
//
pub const MC_CMD_INIT_EVQ_V2_IN_TMR_LOAD_OFST: c_int = 8;
pub const MC_CMD_INIT_EVQ_V2_IN_TMR_LOAD_LEN: c_int = 4;
// The reload value is ignored in one-shot modes
pub const MC_CMD_INIT_EVQ_V2_IN_TMR_RELOAD_OFST: c_int = 12;
pub const MC_CMD_INIT_EVQ_V2_IN_TMR_RELOAD_LEN: c_int = 4;
// tbd
pub const MC_CMD_INIT_EVQ_V2_IN_FLAGS_OFST: c_int = 16;
pub const MC_CMD_INIT_EVQ_V2_IN_FLAGS_LEN: c_int = 4;
pub const MC_CMD_INIT_EVQ_V2_IN_FLAG_INTERRUPTING_OFST: c_int = 16;
pub const MC_CMD_INIT_EVQ_V2_IN_FLAG_INTERRUPTING_LBN: c_int = 0;
pub const MC_CMD_INIT_EVQ_V2_IN_FLAG_INTERRUPTING_WIDTH: c_int = 1;
pub const MC_CMD_INIT_EVQ_V2_IN_FLAG_RPTR_DOS_OFST: c_int = 16;
pub const MC_CMD_INIT_EVQ_V2_IN_FLAG_RPTR_DOS_LBN: c_int = 1;
pub const MC_CMD_INIT_EVQ_V2_IN_FLAG_RPTR_DOS_WIDTH: c_int = 1;
pub const MC_CMD_INIT_EVQ_V2_IN_FLAG_INT_ARMD_OFST: c_int = 16;
pub const MC_CMD_INIT_EVQ_V2_IN_FLAG_INT_ARMD_LBN: c_int = 2;
pub const MC_CMD_INIT_EVQ_V2_IN_FLAG_INT_ARMD_WIDTH: c_int = 1;
pub const MC_CMD_INIT_EVQ_V2_IN_FLAG_CUT_THRU_OFST: c_int = 16;
pub const MC_CMD_INIT_EVQ_V2_IN_FLAG_CUT_THRU_LBN: c_int = 3;
pub const MC_CMD_INIT_EVQ_V2_IN_FLAG_CUT_THRU_WIDTH: c_int = 1;
pub const MC_CMD_INIT_EVQ_V2_IN_FLAG_RX_MERGE_OFST: c_int = 16;
pub const MC_CMD_INIT_EVQ_V2_IN_FLAG_RX_MERGE_LBN: c_int = 4;
pub const MC_CMD_INIT_EVQ_V2_IN_FLAG_RX_MERGE_WIDTH: c_int = 1;
pub const MC_CMD_INIT_EVQ_V2_IN_FLAG_TX_MERGE_OFST: c_int = 16;
pub const MC_CMD_INIT_EVQ_V2_IN_FLAG_TX_MERGE_LBN: c_int = 5;
pub const MC_CMD_INIT_EVQ_V2_IN_FLAG_TX_MERGE_WIDTH: c_int = 1;
pub const MC_CMD_INIT_EVQ_V2_IN_FLAG_USE_TIMER_OFST: c_int = 16;
pub const MC_CMD_INIT_EVQ_V2_IN_FLAG_USE_TIMER_LBN: c_int = 6;
pub const MC_CMD_INIT_EVQ_V2_IN_FLAG_USE_TIMER_WIDTH: c_int = 1;
pub const MC_CMD_INIT_EVQ_V2_IN_FLAG_TYPE_OFST: c_int = 16;
pub const MC_CMD_INIT_EVQ_V2_IN_FLAG_TYPE_LBN: c_int = 7;
pub const MC_CMD_INIT_EVQ_V2_IN_FLAG_TYPE_WIDTH: c_int = 4;
// enum: All initialisation flags specified by host.
pub const MC_CMD_INIT_EVQ_V2_IN_FLAG_TYPE_MANUAL: c_uint = 0x0;
// enum: MEDFORD only. Certain initialisation flags specified by host may be
// over-ridden by firmware based on licenses and firmware variant in order to
// provide the lowest latency achievable. See
// MC_CMD_INIT_EVQ_V2/MC_CMD_INIT_EVQ_V2_OUT/FLAGS for list of affected flags.
//
pub const MC_CMD_INIT_EVQ_V2_IN_FLAG_TYPE_LOW_LATENCY: c_uint = 0x1;
// enum: MEDFORD only. Certain initialisation flags specified by host may be
// over-ridden by firmware based on licenses and firmware variant in order to
// provide the best throughput achievable. See
// MC_CMD_INIT_EVQ_V2/MC_CMD_INIT_EVQ_V2_OUT/FLAGS for list of affected flags.
//
pub const MC_CMD_INIT_EVQ_V2_IN_FLAG_TYPE_THROUGHPUT: c_uint = 0x2;
// enum: MEDFORD only. Certain initialisation flags may be over-ridden by
// firmware based on licenses and firmware variant. See
// MC_CMD_INIT_EVQ_V2/MC_CMD_INIT_EVQ_V2_OUT/FLAGS for list of affected flags.
//
pub const MC_CMD_INIT_EVQ_V2_IN_FLAG_TYPE_AUTO: c_uint = 0x3;
pub const MC_CMD_INIT_EVQ_V2_IN_FLAG_EXT_WIDTH_OFST: c_int = 16;
pub const MC_CMD_INIT_EVQ_V2_IN_FLAG_EXT_WIDTH_LBN: c_int = 11;
pub const MC_CMD_INIT_EVQ_V2_IN_FLAG_EXT_WIDTH_WIDTH: c_int = 1;
pub const MC_CMD_INIT_EVQ_V2_IN_TMR_MODE_OFST: c_int = 20;
pub const MC_CMD_INIT_EVQ_V2_IN_TMR_MODE_LEN: c_int = 4;
// enum: Disabled
pub const MC_CMD_INIT_EVQ_V2_IN_TMR_MODE_DIS: c_uint = 0x0;
// enum: Immediate
pub const MC_CMD_INIT_EVQ_V2_IN_TMR_IMMED_START: c_uint = 0x1;
// enum: Triggered
pub const MC_CMD_INIT_EVQ_V2_IN_TMR_TRIG_START: c_uint = 0x2;
// enum: Hold-off
pub const MC_CMD_INIT_EVQ_V2_IN_TMR_INT_HLDOFF: c_uint = 0x3;
// Target EVQ for wakeups if in wakeup mode.
pub const MC_CMD_INIT_EVQ_V2_IN_TARGET_EVQ_OFST: c_int = 24;
pub const MC_CMD_INIT_EVQ_V2_IN_TARGET_EVQ_LEN: c_int = 4;
// Target interrupt if in interrupting mode (note union with target EVQ). Use
// MC_CMD_RESOURCE_INSTANCE_ANY unless a specific one required for test
// purposes.
//
pub const MC_CMD_INIT_EVQ_V2_IN_IRQ_NUM_OFST: c_int = 24;
pub const MC_CMD_INIT_EVQ_V2_IN_IRQ_NUM_LEN: c_int = 4;
// Event Counter Mode.
pub const MC_CMD_INIT_EVQ_V2_IN_COUNT_MODE_OFST: c_int = 28;
pub const MC_CMD_INIT_EVQ_V2_IN_COUNT_MODE_LEN: c_int = 4;
// enum: Disabled
pub const MC_CMD_INIT_EVQ_V2_IN_COUNT_MODE_DIS: c_uint = 0x0;
// enum: Disabled
pub const MC_CMD_INIT_EVQ_V2_IN_COUNT_MODE_RX: c_uint = 0x1;
// enum: Disabled
pub const MC_CMD_INIT_EVQ_V2_IN_COUNT_MODE_TX: c_uint = 0x2;
// enum: Disabled
pub const MC_CMD_INIT_EVQ_V2_IN_COUNT_MODE_RXTX: c_uint = 0x3;
// Event queue packet count threshold.
pub const MC_CMD_INIT_EVQ_V2_IN_COUNT_THRSHLD_OFST: c_int = 32;
pub const MC_CMD_INIT_EVQ_V2_IN_COUNT_THRSHLD_LEN: c_int = 4;
// 64-bit address of 4k of 4k-aligned host memory buffer
pub const MC_CMD_INIT_EVQ_V2_IN_DMA_ADDR_OFST: c_int = 36;
pub const MC_CMD_INIT_EVQ_V2_IN_DMA_ADDR_LEN: c_int = 8;
pub const MC_CMD_INIT_EVQ_V2_IN_DMA_ADDR_LO_OFST: c_int = 36;
pub const MC_CMD_INIT_EVQ_V2_IN_DMA_ADDR_HI_OFST: c_int = 40;
pub const MC_CMD_INIT_EVQ_V2_IN_DMA_ADDR_MINNUM: c_int = 1;
pub const MC_CMD_INIT_EVQ_V2_IN_DMA_ADDR_MAXNUM: c_int = 64;
pub const MC_CMD_INIT_EVQ_V2_IN_DMA_ADDR_MAXNUM_MCDI2: c_int = 64;
// MC_CMD_INIT_EVQ_V2_OUT msgresponse
pub const MC_CMD_INIT_EVQ_V2_OUT_LEN: c_int = 8;
// Only valid if INTRFLAG was true
pub const MC_CMD_INIT_EVQ_V2_OUT_IRQ_OFST: c_int = 0;
pub const MC_CMD_INIT_EVQ_V2_OUT_IRQ_LEN: c_int = 4;
// Actual configuration applied on the card
pub const MC_CMD_INIT_EVQ_V2_OUT_FLAGS_OFST: c_int = 4;
pub const MC_CMD_INIT_EVQ_V2_OUT_FLAGS_LEN: c_int = 4;
pub const MC_CMD_INIT_EVQ_V2_OUT_FLAG_CUT_THRU_OFST: c_int = 4;
pub const MC_CMD_INIT_EVQ_V2_OUT_FLAG_CUT_THRU_LBN: c_int = 0;
pub const MC_CMD_INIT_EVQ_V2_OUT_FLAG_CUT_THRU_WIDTH: c_int = 1;
pub const MC_CMD_INIT_EVQ_V2_OUT_FLAG_RX_MERGE_OFST: c_int = 4;
pub const MC_CMD_INIT_EVQ_V2_OUT_FLAG_RX_MERGE_LBN: c_int = 1;
pub const MC_CMD_INIT_EVQ_V2_OUT_FLAG_RX_MERGE_WIDTH: c_int = 1;
pub const MC_CMD_INIT_EVQ_V2_OUT_FLAG_TX_MERGE_OFST: c_int = 4;
pub const MC_CMD_INIT_EVQ_V2_OUT_FLAG_TX_MERGE_LBN: c_int = 2;
pub const MC_CMD_INIT_EVQ_V2_OUT_FLAG_TX_MERGE_WIDTH: c_int = 1;
pub const MC_CMD_INIT_EVQ_V2_OUT_FLAG_RXQ_FORCE_EV_MERGING_OFST: c_int = 4;
pub const MC_CMD_INIT_EVQ_V2_OUT_FLAG_RXQ_FORCE_EV_MERGING_LBN: c_int = 3;
pub const MC_CMD_INIT_EVQ_V2_OUT_FLAG_RXQ_FORCE_EV_MERGING_WIDTH: c_int = 1;
// QUEUE_CRC_MODE structuredef
pub const QUEUE_CRC_MODE_LEN: c_int = 1;
pub const QUEUE_CRC_MODE_MODE_LBN: c_int = 0;
pub const QUEUE_CRC_MODE_MODE_WIDTH: c_int = 4;
// enum: No CRC.
pub const QUEUE_CRC_MODE_NONE: c_uint = 0x0;
// enum: CRC Fiber channel over ethernet.
pub const QUEUE_CRC_MODE_FCOE: c_uint = 0x1;
// enum: CRC (digest) iSCSI header only.
pub const QUEUE_CRC_MODE_ISCSI_HDR: c_uint = 0x2;
// enum: CRC (digest) iSCSI header and payload.
pub const QUEUE_CRC_MODE_ISCSI: c_uint = 0x3;
// enum: CRC Fiber channel over IP over ethernet.
pub const QUEUE_CRC_MODE_FCOIPOE: c_uint = 0x4;
// enum: CRC MPA.
pub const QUEUE_CRC_MODE_MPA: c_uint = 0x5;
pub const QUEUE_CRC_MODE_SPARE_LBN: c_int = 4;
pub const QUEUE_CRC_MODE_SPARE_WIDTH: c_int = 4;
//
// MC_CMD_INIT_RXQ
// set up a receive queue according to the supplied parameters. The IN
// arguments end with an address for each 4k of host memory required to back
// the RXQ.
//
pub const MC_CMD_INIT_RXQ: c_uint = 0x81;

// MC_CMD_INIT_RXQ_IN msgrequest: Legacy RXQ_INIT request. Use extended version
// in new code.
//
pub const MC_CMD_INIT_RXQ_IN_LENMIN: c_int = 36;
pub const MC_CMD_INIT_RXQ_IN_LENMAX: c_int = 252;
pub const MC_CMD_INIT_RXQ_IN_LENMAX_MCDI2: c_int = 1020;

// Size, in entries
pub const MC_CMD_INIT_RXQ_IN_SIZE_OFST: c_int = 0;
pub const MC_CMD_INIT_RXQ_IN_SIZE_LEN: c_int = 4;
// The EVQ to send events to. This is an index originally specified to INIT_EVQ
//
pub const MC_CMD_INIT_RXQ_IN_TARGET_EVQ_OFST: c_int = 4;
pub const MC_CMD_INIT_RXQ_IN_TARGET_EVQ_LEN: c_int = 4;
// The value to put in the event data. Check hardware spec. for valid range.
pub const MC_CMD_INIT_RXQ_IN_LABEL_OFST: c_int = 8;
pub const MC_CMD_INIT_RXQ_IN_LABEL_LEN: c_int = 4;
// Desired instance. Must be set to a specific instance, which is a function
// local queue index.
//
pub const MC_CMD_INIT_RXQ_IN_INSTANCE_OFST: c_int = 12;
pub const MC_CMD_INIT_RXQ_IN_INSTANCE_LEN: c_int = 4;
// There will be more flags here.
pub const MC_CMD_INIT_RXQ_IN_FLAGS_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_IN_FLAGS_LEN: c_int = 4;
pub const MC_CMD_INIT_RXQ_IN_FLAG_BUFF_MODE_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_IN_FLAG_BUFF_MODE_LBN: c_int = 0;
pub const MC_CMD_INIT_RXQ_IN_FLAG_BUFF_MODE_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_IN_FLAG_HDR_SPLIT_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_IN_FLAG_HDR_SPLIT_LBN: c_int = 1;
pub const MC_CMD_INIT_RXQ_IN_FLAG_HDR_SPLIT_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_IN_FLAG_TIMESTAMP_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_IN_FLAG_TIMESTAMP_LBN: c_int = 2;
pub const MC_CMD_INIT_RXQ_IN_FLAG_TIMESTAMP_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_IN_CRC_MODE_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_IN_CRC_MODE_LBN: c_int = 3;
pub const MC_CMD_INIT_RXQ_IN_CRC_MODE_WIDTH: c_int = 4;
pub const MC_CMD_INIT_RXQ_IN_FLAG_CHAIN_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_IN_FLAG_CHAIN_LBN: c_int = 7;
pub const MC_CMD_INIT_RXQ_IN_FLAG_CHAIN_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_IN_FLAG_PREFIX_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_IN_FLAG_PREFIX_LBN: c_int = 8;
pub const MC_CMD_INIT_RXQ_IN_FLAG_PREFIX_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_IN_FLAG_DISABLE_SCATTER_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_IN_FLAG_DISABLE_SCATTER_LBN: c_int = 9;
pub const MC_CMD_INIT_RXQ_IN_FLAG_DISABLE_SCATTER_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_IN_UNUSED_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_IN_UNUSED_LBN: c_int = 10;
pub const MC_CMD_INIT_RXQ_IN_UNUSED_WIDTH: c_int = 1;
// Owner ID to use if in buffer mode (zero if physical)
pub const MC_CMD_INIT_RXQ_IN_OWNER_ID_OFST: c_int = 20;
pub const MC_CMD_INIT_RXQ_IN_OWNER_ID_LEN: c_int = 4;
// The port ID associated with the v-adaptor which should contain this DMAQ.
pub const MC_CMD_INIT_RXQ_IN_PORT_ID_OFST: c_int = 24;
pub const MC_CMD_INIT_RXQ_IN_PORT_ID_LEN: c_int = 4;
// 64-bit address of 4k of 4k-aligned host memory buffer
pub const MC_CMD_INIT_RXQ_IN_DMA_ADDR_OFST: c_int = 28;
pub const MC_CMD_INIT_RXQ_IN_DMA_ADDR_LEN: c_int = 8;
pub const MC_CMD_INIT_RXQ_IN_DMA_ADDR_LO_OFST: c_int = 28;
pub const MC_CMD_INIT_RXQ_IN_DMA_ADDR_HI_OFST: c_int = 32;
pub const MC_CMD_INIT_RXQ_IN_DMA_ADDR_MINNUM: c_int = 1;
pub const MC_CMD_INIT_RXQ_IN_DMA_ADDR_MAXNUM: c_int = 28;
pub const MC_CMD_INIT_RXQ_IN_DMA_ADDR_MAXNUM_MCDI2: c_int = 124;
// MC_CMD_INIT_RXQ_EXT_IN msgrequest: Extended RXQ_INIT with additional mode
// flags
//
pub const MC_CMD_INIT_RXQ_EXT_IN_LEN: c_int = 544;
// Size, in entries
pub const MC_CMD_INIT_RXQ_EXT_IN_SIZE_OFST: c_int = 0;
pub const MC_CMD_INIT_RXQ_EXT_IN_SIZE_LEN: c_int = 4;
// The EVQ to send events to. This is an index originally specified to
// INIT_EVQ. If DMA_MODE == PACKED_STREAM this must be equal to INSTANCE.
//
pub const MC_CMD_INIT_RXQ_EXT_IN_TARGET_EVQ_OFST: c_int = 4;
pub const MC_CMD_INIT_RXQ_EXT_IN_TARGET_EVQ_LEN: c_int = 4;
// The value to put in the event data. Check hardware spec. for valid range.
// This field is ignored if DMA_MODE == EQUAL_STRIDE_SUPER_BUFFER or DMA_MODE
// == PACKED_STREAM.
//
pub const MC_CMD_INIT_RXQ_EXT_IN_LABEL_OFST: c_int = 8;
pub const MC_CMD_INIT_RXQ_EXT_IN_LABEL_LEN: c_int = 4;
// Desired instance. Must be set to a specific instance, which is a function
// local queue index.
//
pub const MC_CMD_INIT_RXQ_EXT_IN_INSTANCE_OFST: c_int = 12;
pub const MC_CMD_INIT_RXQ_EXT_IN_INSTANCE_LEN: c_int = 4;
// There will be more flags here.
pub const MC_CMD_INIT_RXQ_EXT_IN_FLAGS_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_EXT_IN_FLAGS_LEN: c_int = 4;
pub const MC_CMD_INIT_RXQ_EXT_IN_FLAG_BUFF_MODE_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_EXT_IN_FLAG_BUFF_MODE_LBN: c_int = 0;
pub const MC_CMD_INIT_RXQ_EXT_IN_FLAG_BUFF_MODE_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_EXT_IN_FLAG_HDR_SPLIT_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_EXT_IN_FLAG_HDR_SPLIT_LBN: c_int = 1;
pub const MC_CMD_INIT_RXQ_EXT_IN_FLAG_HDR_SPLIT_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_EXT_IN_FLAG_TIMESTAMP_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_EXT_IN_FLAG_TIMESTAMP_LBN: c_int = 2;
pub const MC_CMD_INIT_RXQ_EXT_IN_FLAG_TIMESTAMP_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_EXT_IN_CRC_MODE_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_EXT_IN_CRC_MODE_LBN: c_int = 3;
pub const MC_CMD_INIT_RXQ_EXT_IN_CRC_MODE_WIDTH: c_int = 4;
pub const MC_CMD_INIT_RXQ_EXT_IN_FLAG_CHAIN_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_EXT_IN_FLAG_CHAIN_LBN: c_int = 7;
pub const MC_CMD_INIT_RXQ_EXT_IN_FLAG_CHAIN_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_EXT_IN_FLAG_PREFIX_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_EXT_IN_FLAG_PREFIX_LBN: c_int = 8;
pub const MC_CMD_INIT_RXQ_EXT_IN_FLAG_PREFIX_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_EXT_IN_FLAG_DISABLE_SCATTER_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_EXT_IN_FLAG_DISABLE_SCATTER_LBN: c_int = 9;
pub const MC_CMD_INIT_RXQ_EXT_IN_FLAG_DISABLE_SCATTER_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_EXT_IN_DMA_MODE_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_EXT_IN_DMA_MODE_LBN: c_int = 10;
pub const MC_CMD_INIT_RXQ_EXT_IN_DMA_MODE_WIDTH: c_int = 4;
// enum: One packet per descriptor (for normal networking)
pub const MC_CMD_INIT_RXQ_EXT_IN_SINGLE_PACKET: c_uint = 0x0;
// enum: Pack multiple packets into large descriptors (for SolarCapture)
pub const MC_CMD_INIT_RXQ_EXT_IN_PACKED_STREAM: c_uint = 0x1;
// enum: Pack multiple packets into large descriptors using the format designed
// to maximise packet rate. This mode uses 1 "bucket" per descriptor with
// multiple fixed-size packet buffers within each bucket. For a full
// description see SF-119419-TC. This mode is only supported by "dpdk" datapath
// firmware.
//
pub const MC_CMD_INIT_RXQ_EXT_IN_EQUAL_STRIDE_SUPER_BUFFER: c_uint = 0x2;
// enum: Deprecated name for EQUAL_STRIDE_SUPER_BUFFER.
pub const MC_CMD_INIT_RXQ_EXT_IN_EQUAL_STRIDE_PACKED_STREAM: c_uint = 0x2;
pub const MC_CMD_INIT_RXQ_EXT_IN_FLAG_SNAPSHOT_MODE_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_EXT_IN_FLAG_SNAPSHOT_MODE_LBN: c_int = 14;
pub const MC_CMD_INIT_RXQ_EXT_IN_FLAG_SNAPSHOT_MODE_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_EXT_IN_PACKED_STREAM_BUFF_SIZE_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_EXT_IN_PACKED_STREAM_BUFF_SIZE_LBN: c_int = 15;
pub const MC_CMD_INIT_RXQ_EXT_IN_PACKED_STREAM_BUFF_SIZE_WIDTH: c_int = 3;
pub const MC_CMD_INIT_RXQ_EXT_IN_PS_BUFF_1M: c_uint = 0x0 /* enum */;
pub const MC_CMD_INIT_RXQ_EXT_IN_PS_BUFF_512K: c_uint = 0x1 /* enum */;
pub const MC_CMD_INIT_RXQ_EXT_IN_PS_BUFF_256K: c_uint = 0x2 /* enum */;
pub const MC_CMD_INIT_RXQ_EXT_IN_PS_BUFF_128K: c_uint = 0x3 /* enum */;
pub const MC_CMD_INIT_RXQ_EXT_IN_PS_BUFF_64K: c_uint = 0x4 /* enum */;
pub const MC_CMD_INIT_RXQ_EXT_IN_FLAG_WANT_OUTER_CLASSES_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_EXT_IN_FLAG_WANT_OUTER_CLASSES_LBN: c_int = 18;
pub const MC_CMD_INIT_RXQ_EXT_IN_FLAG_WANT_OUTER_CLASSES_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_EXT_IN_FLAG_FORCE_EV_MERGING_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_EXT_IN_FLAG_FORCE_EV_MERGING_LBN: c_int = 19;
pub const MC_CMD_INIT_RXQ_EXT_IN_FLAG_FORCE_EV_MERGING_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_EXT_IN_FLAG_NO_CONT_EV_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_EXT_IN_FLAG_NO_CONT_EV_LBN: c_int = 20;
pub const MC_CMD_INIT_RXQ_EXT_IN_FLAG_NO_CONT_EV_WIDTH: c_int = 1;
// Owner ID to use if in buffer mode (zero if physical)
pub const MC_CMD_INIT_RXQ_EXT_IN_OWNER_ID_OFST: c_int = 20;
pub const MC_CMD_INIT_RXQ_EXT_IN_OWNER_ID_LEN: c_int = 4;
// The port ID associated with the v-adaptor which should contain this DMAQ.
pub const MC_CMD_INIT_RXQ_EXT_IN_PORT_ID_OFST: c_int = 24;
pub const MC_CMD_INIT_RXQ_EXT_IN_PORT_ID_LEN: c_int = 4;
// 64-bit address of 4k of 4k-aligned host memory buffer
pub const MC_CMD_INIT_RXQ_EXT_IN_DMA_ADDR_OFST: c_int = 28;
pub const MC_CMD_INIT_RXQ_EXT_IN_DMA_ADDR_LEN: c_int = 8;
pub const MC_CMD_INIT_RXQ_EXT_IN_DMA_ADDR_LO_OFST: c_int = 28;
pub const MC_CMD_INIT_RXQ_EXT_IN_DMA_ADDR_HI_OFST: c_int = 32;
pub const MC_CMD_INIT_RXQ_EXT_IN_DMA_ADDR_NUM: c_int = 64;
// Maximum length of packet to receive, if SNAPSHOT_MODE flag is set
pub const MC_CMD_INIT_RXQ_EXT_IN_SNAPSHOT_LENGTH_OFST: c_int = 540;
pub const MC_CMD_INIT_RXQ_EXT_IN_SNAPSHOT_LENGTH_LEN: c_int = 4;
// MC_CMD_INIT_RXQ_V3_IN msgrequest
pub const MC_CMD_INIT_RXQ_V3_IN_LEN: c_int = 560;
// Size, in entries
pub const MC_CMD_INIT_RXQ_V3_IN_SIZE_OFST: c_int = 0;
pub const MC_CMD_INIT_RXQ_V3_IN_SIZE_LEN: c_int = 4;
// The EVQ to send events to. This is an index originally specified to
// INIT_EVQ. If DMA_MODE == PACKED_STREAM this must be equal to INSTANCE.
//
pub const MC_CMD_INIT_RXQ_V3_IN_TARGET_EVQ_OFST: c_int = 4;
pub const MC_CMD_INIT_RXQ_V3_IN_TARGET_EVQ_LEN: c_int = 4;
// The value to put in the event data. Check hardware spec. for valid range.
// This field is ignored if DMA_MODE == EQUAL_STRIDE_SUPER_BUFFER or DMA_MODE
// == PACKED_STREAM.
//
pub const MC_CMD_INIT_RXQ_V3_IN_LABEL_OFST: c_int = 8;
pub const MC_CMD_INIT_RXQ_V3_IN_LABEL_LEN: c_int = 4;
// Desired instance. Must be set to a specific instance, which is a function
// local queue index.
//
pub const MC_CMD_INIT_RXQ_V3_IN_INSTANCE_OFST: c_int = 12;
pub const MC_CMD_INIT_RXQ_V3_IN_INSTANCE_LEN: c_int = 4;
// There will be more flags here.
pub const MC_CMD_INIT_RXQ_V3_IN_FLAGS_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V3_IN_FLAGS_LEN: c_int = 4;
pub const MC_CMD_INIT_RXQ_V3_IN_FLAG_BUFF_MODE_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V3_IN_FLAG_BUFF_MODE_LBN: c_int = 0;
pub const MC_CMD_INIT_RXQ_V3_IN_FLAG_BUFF_MODE_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_V3_IN_FLAG_HDR_SPLIT_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V3_IN_FLAG_HDR_SPLIT_LBN: c_int = 1;
pub const MC_CMD_INIT_RXQ_V3_IN_FLAG_HDR_SPLIT_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_V3_IN_FLAG_TIMESTAMP_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V3_IN_FLAG_TIMESTAMP_LBN: c_int = 2;
pub const MC_CMD_INIT_RXQ_V3_IN_FLAG_TIMESTAMP_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_V3_IN_CRC_MODE_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V3_IN_CRC_MODE_LBN: c_int = 3;
pub const MC_CMD_INIT_RXQ_V3_IN_CRC_MODE_WIDTH: c_int = 4;
pub const MC_CMD_INIT_RXQ_V3_IN_FLAG_CHAIN_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V3_IN_FLAG_CHAIN_LBN: c_int = 7;
pub const MC_CMD_INIT_RXQ_V3_IN_FLAG_CHAIN_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_V3_IN_FLAG_PREFIX_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V3_IN_FLAG_PREFIX_LBN: c_int = 8;
pub const MC_CMD_INIT_RXQ_V3_IN_FLAG_PREFIX_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_V3_IN_FLAG_DISABLE_SCATTER_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V3_IN_FLAG_DISABLE_SCATTER_LBN: c_int = 9;
pub const MC_CMD_INIT_RXQ_V3_IN_FLAG_DISABLE_SCATTER_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_V3_IN_DMA_MODE_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V3_IN_DMA_MODE_LBN: c_int = 10;
pub const MC_CMD_INIT_RXQ_V3_IN_DMA_MODE_WIDTH: c_int = 4;
// enum: One packet per descriptor (for normal networking)
pub const MC_CMD_INIT_RXQ_V3_IN_SINGLE_PACKET: c_uint = 0x0;
// enum: Pack multiple packets into large descriptors (for SolarCapture)
pub const MC_CMD_INIT_RXQ_V3_IN_PACKED_STREAM: c_uint = 0x1;
// enum: Pack multiple packets into large descriptors using the format designed
// to maximise packet rate. This mode uses 1 "bucket" per descriptor with
// multiple fixed-size packet buffers within each bucket. For a full
// description see SF-119419-TC. This mode is only supported by "dpdk" datapath
// firmware.
//
pub const MC_CMD_INIT_RXQ_V3_IN_EQUAL_STRIDE_SUPER_BUFFER: c_uint = 0x2;
// enum: Deprecated name for EQUAL_STRIDE_SUPER_BUFFER.
pub const MC_CMD_INIT_RXQ_V3_IN_EQUAL_STRIDE_PACKED_STREAM: c_uint = 0x2;
pub const MC_CMD_INIT_RXQ_V3_IN_FLAG_SNAPSHOT_MODE_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V3_IN_FLAG_SNAPSHOT_MODE_LBN: c_int = 14;
pub const MC_CMD_INIT_RXQ_V3_IN_FLAG_SNAPSHOT_MODE_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_V3_IN_PACKED_STREAM_BUFF_SIZE_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V3_IN_PACKED_STREAM_BUFF_SIZE_LBN: c_int = 15;
pub const MC_CMD_INIT_RXQ_V3_IN_PACKED_STREAM_BUFF_SIZE_WIDTH: c_int = 3;
pub const MC_CMD_INIT_RXQ_V3_IN_PS_BUFF_1M: c_uint = 0x0 /* enum */;
pub const MC_CMD_INIT_RXQ_V3_IN_PS_BUFF_512K: c_uint = 0x1 /* enum */;
pub const MC_CMD_INIT_RXQ_V3_IN_PS_BUFF_256K: c_uint = 0x2 /* enum */;
pub const MC_CMD_INIT_RXQ_V3_IN_PS_BUFF_128K: c_uint = 0x3 /* enum */;
pub const MC_CMD_INIT_RXQ_V3_IN_PS_BUFF_64K: c_uint = 0x4 /* enum */;
pub const MC_CMD_INIT_RXQ_V3_IN_FLAG_WANT_OUTER_CLASSES_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V3_IN_FLAG_WANT_OUTER_CLASSES_LBN: c_int = 18;
pub const MC_CMD_INIT_RXQ_V3_IN_FLAG_WANT_OUTER_CLASSES_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_V3_IN_FLAG_FORCE_EV_MERGING_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V3_IN_FLAG_FORCE_EV_MERGING_LBN: c_int = 19;
pub const MC_CMD_INIT_RXQ_V3_IN_FLAG_FORCE_EV_MERGING_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_V3_IN_FLAG_NO_CONT_EV_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V3_IN_FLAG_NO_CONT_EV_LBN: c_int = 20;
pub const MC_CMD_INIT_RXQ_V3_IN_FLAG_NO_CONT_EV_WIDTH: c_int = 1;
// Owner ID to use if in buffer mode (zero if physical)
pub const MC_CMD_INIT_RXQ_V3_IN_OWNER_ID_OFST: c_int = 20;
pub const MC_CMD_INIT_RXQ_V3_IN_OWNER_ID_LEN: c_int = 4;
// The port ID associated with the v-adaptor which should contain this DMAQ.
pub const MC_CMD_INIT_RXQ_V3_IN_PORT_ID_OFST: c_int = 24;
pub const MC_CMD_INIT_RXQ_V3_IN_PORT_ID_LEN: c_int = 4;
// 64-bit address of 4k of 4k-aligned host memory buffer
pub const MC_CMD_INIT_RXQ_V3_IN_DMA_ADDR_OFST: c_int = 28;
pub const MC_CMD_INIT_RXQ_V3_IN_DMA_ADDR_LEN: c_int = 8;
pub const MC_CMD_INIT_RXQ_V3_IN_DMA_ADDR_LO_OFST: c_int = 28;
pub const MC_CMD_INIT_RXQ_V3_IN_DMA_ADDR_HI_OFST: c_int = 32;
pub const MC_CMD_INIT_RXQ_V3_IN_DMA_ADDR_NUM: c_int = 64;
// Maximum length of packet to receive, if SNAPSHOT_MODE flag is set
pub const MC_CMD_INIT_RXQ_V3_IN_SNAPSHOT_LENGTH_OFST: c_int = 540;
pub const MC_CMD_INIT_RXQ_V3_IN_SNAPSHOT_LENGTH_LEN: c_int = 4;
// The number of packet buffers that will be contained within each
// EQUAL_STRIDE_SUPER_BUFFER format bucket supplied by the driver. This field
// is ignored unless DMA_MODE == EQUAL_STRIDE_SUPER_BUFFER.
//
pub const MC_CMD_INIT_RXQ_V3_IN_ES_PACKET_BUFFERS_PER_BUCKET_OFST: c_int = 544;
pub const MC_CMD_INIT_RXQ_V3_IN_ES_PACKET_BUFFERS_PER_BUCKET_LEN: c_int = 4;
// The length in bytes of the area in each packet buffer that can be written to
// by the adapter. This is used to store the packet prefix and the packet
// payload. This length does not include any end padding added by the driver.
// This field is ignored unless DMA_MODE == EQUAL_STRIDE_SUPER_BUFFER.
//
pub const MC_CMD_INIT_RXQ_V3_IN_ES_MAX_DMA_LEN_OFST: c_int = 548;
pub const MC_CMD_INIT_RXQ_V3_IN_ES_MAX_DMA_LEN_LEN: c_int = 4;
// The length in bytes of a single packet buffer within a
// EQUAL_STRIDE_SUPER_BUFFER format bucket. This field is ignored unless
// DMA_MODE == EQUAL_STRIDE_SUPER_BUFFER.
//
pub const MC_CMD_INIT_RXQ_V3_IN_ES_PACKET_STRIDE_OFST: c_int = 552;
pub const MC_CMD_INIT_RXQ_V3_IN_ES_PACKET_STRIDE_LEN: c_int = 4;
// The maximum time in nanoseconds that the datapath will be backpressured if
// there are no RX descriptors available. If the timeout is reached and there
// are still no descriptors then the packet will be dropped. A timeout of 0
// means the datapath will never be blocked. This field is ignored unless
// DMA_MODE == EQUAL_STRIDE_SUPER_BUFFER.
//
pub const MC_CMD_INIT_RXQ_V3_IN_ES_HEAD_OF_LINE_BLOCK_TIMEOUT_OFST: c_int = 556;
pub const MC_CMD_INIT_RXQ_V3_IN_ES_HEAD_OF_LINE_BLOCK_TIMEOUT_LEN: c_int = 4;
// MC_CMD_INIT_RXQ_V4_IN msgrequest: INIT_RXQ request with new field required
// for systems with a QDMA (currently, Riverhead)
//
pub const MC_CMD_INIT_RXQ_V4_IN_LEN: c_int = 564;
// Size, in entries
pub const MC_CMD_INIT_RXQ_V4_IN_SIZE_OFST: c_int = 0;
pub const MC_CMD_INIT_RXQ_V4_IN_SIZE_LEN: c_int = 4;
// The EVQ to send events to. This is an index originally specified to
// INIT_EVQ. If DMA_MODE == PACKED_STREAM this must be equal to INSTANCE.
//
pub const MC_CMD_INIT_RXQ_V4_IN_TARGET_EVQ_OFST: c_int = 4;
pub const MC_CMD_INIT_RXQ_V4_IN_TARGET_EVQ_LEN: c_int = 4;
// The value to put in the event data. Check hardware spec. for valid range.
// This field is ignored if DMA_MODE == EQUAL_STRIDE_SUPER_BUFFER or DMA_MODE
// == PACKED_STREAM.
//
pub const MC_CMD_INIT_RXQ_V4_IN_LABEL_OFST: c_int = 8;
pub const MC_CMD_INIT_RXQ_V4_IN_LABEL_LEN: c_int = 4;
// Desired instance. Must be set to a specific instance, which is a function
// local queue index.
//
pub const MC_CMD_INIT_RXQ_V4_IN_INSTANCE_OFST: c_int = 12;
pub const MC_CMD_INIT_RXQ_V4_IN_INSTANCE_LEN: c_int = 4;
// There will be more flags here.
pub const MC_CMD_INIT_RXQ_V4_IN_FLAGS_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V4_IN_FLAGS_LEN: c_int = 4;
pub const MC_CMD_INIT_RXQ_V4_IN_FLAG_BUFF_MODE_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V4_IN_FLAG_BUFF_MODE_LBN: c_int = 0;
pub const MC_CMD_INIT_RXQ_V4_IN_FLAG_BUFF_MODE_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_V4_IN_FLAG_HDR_SPLIT_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V4_IN_FLAG_HDR_SPLIT_LBN: c_int = 1;
pub const MC_CMD_INIT_RXQ_V4_IN_FLAG_HDR_SPLIT_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_V4_IN_FLAG_TIMESTAMP_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V4_IN_FLAG_TIMESTAMP_LBN: c_int = 2;
pub const MC_CMD_INIT_RXQ_V4_IN_FLAG_TIMESTAMP_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_V4_IN_CRC_MODE_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V4_IN_CRC_MODE_LBN: c_int = 3;
pub const MC_CMD_INIT_RXQ_V4_IN_CRC_MODE_WIDTH: c_int = 4;
pub const MC_CMD_INIT_RXQ_V4_IN_FLAG_CHAIN_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V4_IN_FLAG_CHAIN_LBN: c_int = 7;
pub const MC_CMD_INIT_RXQ_V4_IN_FLAG_CHAIN_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_V4_IN_FLAG_PREFIX_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V4_IN_FLAG_PREFIX_LBN: c_int = 8;
pub const MC_CMD_INIT_RXQ_V4_IN_FLAG_PREFIX_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_V4_IN_FLAG_DISABLE_SCATTER_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V4_IN_FLAG_DISABLE_SCATTER_LBN: c_int = 9;
pub const MC_CMD_INIT_RXQ_V4_IN_FLAG_DISABLE_SCATTER_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_V4_IN_DMA_MODE_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V4_IN_DMA_MODE_LBN: c_int = 10;
pub const MC_CMD_INIT_RXQ_V4_IN_DMA_MODE_WIDTH: c_int = 4;
// enum: One packet per descriptor (for normal networking)
pub const MC_CMD_INIT_RXQ_V4_IN_SINGLE_PACKET: c_uint = 0x0;
// enum: Pack multiple packets into large descriptors (for SolarCapture)
pub const MC_CMD_INIT_RXQ_V4_IN_PACKED_STREAM: c_uint = 0x1;
// enum: Pack multiple packets into large descriptors using the format designed
// to maximise packet rate. This mode uses 1 "bucket" per descriptor with
// multiple fixed-size packet buffers within each bucket. For a full
// description see SF-119419-TC. This mode is only supported by "dpdk" datapath
// firmware.
//
pub const MC_CMD_INIT_RXQ_V4_IN_EQUAL_STRIDE_SUPER_BUFFER: c_uint = 0x2;
// enum: Deprecated name for EQUAL_STRIDE_SUPER_BUFFER.
pub const MC_CMD_INIT_RXQ_V4_IN_EQUAL_STRIDE_PACKED_STREAM: c_uint = 0x2;
pub const MC_CMD_INIT_RXQ_V4_IN_FLAG_SNAPSHOT_MODE_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V4_IN_FLAG_SNAPSHOT_MODE_LBN: c_int = 14;
pub const MC_CMD_INIT_RXQ_V4_IN_FLAG_SNAPSHOT_MODE_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_V4_IN_PACKED_STREAM_BUFF_SIZE_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V4_IN_PACKED_STREAM_BUFF_SIZE_LBN: c_int = 15;
pub const MC_CMD_INIT_RXQ_V4_IN_PACKED_STREAM_BUFF_SIZE_WIDTH: c_int = 3;
pub const MC_CMD_INIT_RXQ_V4_IN_PS_BUFF_1M: c_uint = 0x0 /* enum */;
pub const MC_CMD_INIT_RXQ_V4_IN_PS_BUFF_512K: c_uint = 0x1 /* enum */;
pub const MC_CMD_INIT_RXQ_V4_IN_PS_BUFF_256K: c_uint = 0x2 /* enum */;
pub const MC_CMD_INIT_RXQ_V4_IN_PS_BUFF_128K: c_uint = 0x3 /* enum */;
pub const MC_CMD_INIT_RXQ_V4_IN_PS_BUFF_64K: c_uint = 0x4 /* enum */;
pub const MC_CMD_INIT_RXQ_V4_IN_FLAG_WANT_OUTER_CLASSES_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V4_IN_FLAG_WANT_OUTER_CLASSES_LBN: c_int = 18;
pub const MC_CMD_INIT_RXQ_V4_IN_FLAG_WANT_OUTER_CLASSES_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_V4_IN_FLAG_FORCE_EV_MERGING_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V4_IN_FLAG_FORCE_EV_MERGING_LBN: c_int = 19;
pub const MC_CMD_INIT_RXQ_V4_IN_FLAG_FORCE_EV_MERGING_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_V4_IN_FLAG_NO_CONT_EV_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V4_IN_FLAG_NO_CONT_EV_LBN: c_int = 20;
pub const MC_CMD_INIT_RXQ_V4_IN_FLAG_NO_CONT_EV_WIDTH: c_int = 1;
// Owner ID to use if in buffer mode (zero if physical)
pub const MC_CMD_INIT_RXQ_V4_IN_OWNER_ID_OFST: c_int = 20;
pub const MC_CMD_INIT_RXQ_V4_IN_OWNER_ID_LEN: c_int = 4;
// The port ID associated with the v-adaptor which should contain this DMAQ.
pub const MC_CMD_INIT_RXQ_V4_IN_PORT_ID_OFST: c_int = 24;
pub const MC_CMD_INIT_RXQ_V4_IN_PORT_ID_LEN: c_int = 4;
// 64-bit address of 4k of 4k-aligned host memory buffer
pub const MC_CMD_INIT_RXQ_V4_IN_DMA_ADDR_OFST: c_int = 28;
pub const MC_CMD_INIT_RXQ_V4_IN_DMA_ADDR_LEN: c_int = 8;
pub const MC_CMD_INIT_RXQ_V4_IN_DMA_ADDR_LO_OFST: c_int = 28;
pub const MC_CMD_INIT_RXQ_V4_IN_DMA_ADDR_HI_OFST: c_int = 32;
pub const MC_CMD_INIT_RXQ_V4_IN_DMA_ADDR_NUM: c_int = 64;
// Maximum length of packet to receive, if SNAPSHOT_MODE flag is set
pub const MC_CMD_INIT_RXQ_V4_IN_SNAPSHOT_LENGTH_OFST: c_int = 540;
pub const MC_CMD_INIT_RXQ_V4_IN_SNAPSHOT_LENGTH_LEN: c_int = 4;
// The number of packet buffers that will be contained within each
// EQUAL_STRIDE_SUPER_BUFFER format bucket supplied by the driver. This field
// is ignored unless DMA_MODE == EQUAL_STRIDE_SUPER_BUFFER.
//
pub const MC_CMD_INIT_RXQ_V4_IN_ES_PACKET_BUFFERS_PER_BUCKET_OFST: c_int = 544;
pub const MC_CMD_INIT_RXQ_V4_IN_ES_PACKET_BUFFERS_PER_BUCKET_LEN: c_int = 4;
// The length in bytes of the area in each packet buffer that can be written to
// by the adapter. This is used to store the packet prefix and the packet
// payload. This length does not include any end padding added by the driver.
// This field is ignored unless DMA_MODE == EQUAL_STRIDE_SUPER_BUFFER.
//
pub const MC_CMD_INIT_RXQ_V4_IN_ES_MAX_DMA_LEN_OFST: c_int = 548;
pub const MC_CMD_INIT_RXQ_V4_IN_ES_MAX_DMA_LEN_LEN: c_int = 4;
// The length in bytes of a single packet buffer within a
// EQUAL_STRIDE_SUPER_BUFFER format bucket. This field is ignored unless
// DMA_MODE == EQUAL_STRIDE_SUPER_BUFFER.
//
pub const MC_CMD_INIT_RXQ_V4_IN_ES_PACKET_STRIDE_OFST: c_int = 552;
pub const MC_CMD_INIT_RXQ_V4_IN_ES_PACKET_STRIDE_LEN: c_int = 4;
// The maximum time in nanoseconds that the datapath will be backpressured if
// there are no RX descriptors available. If the timeout is reached and there
// are still no descriptors then the packet will be dropped. A timeout of 0
// means the datapath will never be blocked. This field is ignored unless
// DMA_MODE == EQUAL_STRIDE_SUPER_BUFFER.
//
pub const MC_CMD_INIT_RXQ_V4_IN_ES_HEAD_OF_LINE_BLOCK_TIMEOUT_OFST: c_int = 556;
pub const MC_CMD_INIT_RXQ_V4_IN_ES_HEAD_OF_LINE_BLOCK_TIMEOUT_LEN: c_int = 4;
// V4 message data
pub const MC_CMD_INIT_RXQ_V4_IN_V4_DATA_OFST: c_int = 560;
pub const MC_CMD_INIT_RXQ_V4_IN_V4_DATA_LEN: c_int = 4;
// Size in bytes of buffers attached to descriptors posted to this queue. Set
// to zero if using this message on non-QDMA based platforms. Currently in
// Riverhead there is a global limit of eight different buffer sizes across all
// active queues. A 2KB and 4KB buffer is guaranteed to be available, but a
// request for a different buffer size will fail if there are already eight
// other buffer sizes in use. In future Riverhead this limit will go away and
// any size will be accepted.
//
pub const MC_CMD_INIT_RXQ_V4_IN_BUFFER_SIZE_BYTES_OFST: c_int = 560;
pub const MC_CMD_INIT_RXQ_V4_IN_BUFFER_SIZE_BYTES_LEN: c_int = 4;
// MC_CMD_INIT_RXQ_V5_IN msgrequest: INIT_RXQ request with ability to request a
// different RX packet prefix
//
pub const MC_CMD_INIT_RXQ_V5_IN_LEN: c_int = 568;
// Size, in entries
pub const MC_CMD_INIT_RXQ_V5_IN_SIZE_OFST: c_int = 0;
pub const MC_CMD_INIT_RXQ_V5_IN_SIZE_LEN: c_int = 4;
// The EVQ to send events to. This is an index originally specified to
// INIT_EVQ. If DMA_MODE == PACKED_STREAM this must be equal to INSTANCE.
//
pub const MC_CMD_INIT_RXQ_V5_IN_TARGET_EVQ_OFST: c_int = 4;
pub const MC_CMD_INIT_RXQ_V5_IN_TARGET_EVQ_LEN: c_int = 4;
// The value to put in the event data. Check hardware spec. for valid range.
// This field is ignored if DMA_MODE == EQUAL_STRIDE_SUPER_BUFFER or DMA_MODE
// == PACKED_STREAM.
//
pub const MC_CMD_INIT_RXQ_V5_IN_LABEL_OFST: c_int = 8;
pub const MC_CMD_INIT_RXQ_V5_IN_LABEL_LEN: c_int = 4;
// Desired instance. Must be set to a specific instance, which is a function
// local queue index.
//
pub const MC_CMD_INIT_RXQ_V5_IN_INSTANCE_OFST: c_int = 12;
pub const MC_CMD_INIT_RXQ_V5_IN_INSTANCE_LEN: c_int = 4;
// There will be more flags here.
pub const MC_CMD_INIT_RXQ_V5_IN_FLAGS_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V5_IN_FLAGS_LEN: c_int = 4;
pub const MC_CMD_INIT_RXQ_V5_IN_FLAG_BUFF_MODE_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V5_IN_FLAG_BUFF_MODE_LBN: c_int = 0;
pub const MC_CMD_INIT_RXQ_V5_IN_FLAG_BUFF_MODE_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_V5_IN_FLAG_HDR_SPLIT_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V5_IN_FLAG_HDR_SPLIT_LBN: c_int = 1;
pub const MC_CMD_INIT_RXQ_V5_IN_FLAG_HDR_SPLIT_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_V5_IN_FLAG_TIMESTAMP_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V5_IN_FLAG_TIMESTAMP_LBN: c_int = 2;
pub const MC_CMD_INIT_RXQ_V5_IN_FLAG_TIMESTAMP_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_V5_IN_CRC_MODE_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V5_IN_CRC_MODE_LBN: c_int = 3;
pub const MC_CMD_INIT_RXQ_V5_IN_CRC_MODE_WIDTH: c_int = 4;
pub const MC_CMD_INIT_RXQ_V5_IN_FLAG_CHAIN_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V5_IN_FLAG_CHAIN_LBN: c_int = 7;
pub const MC_CMD_INIT_RXQ_V5_IN_FLAG_CHAIN_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_V5_IN_FLAG_PREFIX_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V5_IN_FLAG_PREFIX_LBN: c_int = 8;
pub const MC_CMD_INIT_RXQ_V5_IN_FLAG_PREFIX_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_V5_IN_FLAG_DISABLE_SCATTER_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V5_IN_FLAG_DISABLE_SCATTER_LBN: c_int = 9;
pub const MC_CMD_INIT_RXQ_V5_IN_FLAG_DISABLE_SCATTER_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_V5_IN_DMA_MODE_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V5_IN_DMA_MODE_LBN: c_int = 10;
pub const MC_CMD_INIT_RXQ_V5_IN_DMA_MODE_WIDTH: c_int = 4;
// enum: One packet per descriptor (for normal networking)
pub const MC_CMD_INIT_RXQ_V5_IN_SINGLE_PACKET: c_uint = 0x0;
// enum: Pack multiple packets into large descriptors (for SolarCapture)
pub const MC_CMD_INIT_RXQ_V5_IN_PACKED_STREAM: c_uint = 0x1;
// enum: Pack multiple packets into large descriptors using the format designed
// to maximise packet rate. This mode uses 1 "bucket" per descriptor with
// multiple fixed-size packet buffers within each bucket. For a full
// description see SF-119419-TC. This mode is only supported by "dpdk" datapath
// firmware.
//
pub const MC_CMD_INIT_RXQ_V5_IN_EQUAL_STRIDE_SUPER_BUFFER: c_uint = 0x2;
// enum: Deprecated name for EQUAL_STRIDE_SUPER_BUFFER.
pub const MC_CMD_INIT_RXQ_V5_IN_EQUAL_STRIDE_PACKED_STREAM: c_uint = 0x2;
pub const MC_CMD_INIT_RXQ_V5_IN_FLAG_SNAPSHOT_MODE_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V5_IN_FLAG_SNAPSHOT_MODE_LBN: c_int = 14;
pub const MC_CMD_INIT_RXQ_V5_IN_FLAG_SNAPSHOT_MODE_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_V5_IN_PACKED_STREAM_BUFF_SIZE_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V5_IN_PACKED_STREAM_BUFF_SIZE_LBN: c_int = 15;
pub const MC_CMD_INIT_RXQ_V5_IN_PACKED_STREAM_BUFF_SIZE_WIDTH: c_int = 3;
pub const MC_CMD_INIT_RXQ_V5_IN_PS_BUFF_1M: c_uint = 0x0 /* enum */;
pub const MC_CMD_INIT_RXQ_V5_IN_PS_BUFF_512K: c_uint = 0x1 /* enum */;
pub const MC_CMD_INIT_RXQ_V5_IN_PS_BUFF_256K: c_uint = 0x2 /* enum */;
pub const MC_CMD_INIT_RXQ_V5_IN_PS_BUFF_128K: c_uint = 0x3 /* enum */;
pub const MC_CMD_INIT_RXQ_V5_IN_PS_BUFF_64K: c_uint = 0x4 /* enum */;
pub const MC_CMD_INIT_RXQ_V5_IN_FLAG_WANT_OUTER_CLASSES_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V5_IN_FLAG_WANT_OUTER_CLASSES_LBN: c_int = 18;
pub const MC_CMD_INIT_RXQ_V5_IN_FLAG_WANT_OUTER_CLASSES_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_V5_IN_FLAG_FORCE_EV_MERGING_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V5_IN_FLAG_FORCE_EV_MERGING_LBN: c_int = 19;
pub const MC_CMD_INIT_RXQ_V5_IN_FLAG_FORCE_EV_MERGING_WIDTH: c_int = 1;
pub const MC_CMD_INIT_RXQ_V5_IN_FLAG_NO_CONT_EV_OFST: c_int = 16;
pub const MC_CMD_INIT_RXQ_V5_IN_FLAG_NO_CONT_EV_LBN: c_int = 20;
pub const MC_CMD_INIT_RXQ_V5_IN_FLAG_NO_CONT_EV_WIDTH: c_int = 1;
// Owner ID to use if in buffer mode (zero if physical)
pub const MC_CMD_INIT_RXQ_V5_IN_OWNER_ID_OFST: c_int = 20;
pub const MC_CMD_INIT_RXQ_V5_IN_OWNER_ID_LEN: c_int = 4;
// The port ID associated with the v-adaptor which should contain this DMAQ.
pub const MC_CMD_INIT_RXQ_V5_IN_PORT_ID_OFST: c_int = 24;
pub const MC_CMD_INIT_RXQ_V5_IN_PORT_ID_LEN: c_int = 4;
// 64-bit address of 4k of 4k-aligned host memory buffer
pub const MC_CMD_INIT_RXQ_V5_IN_DMA_ADDR_OFST: c_int = 28;
pub const MC_CMD_INIT_RXQ_V5_IN_DMA_ADDR_LEN: c_int = 8;
pub const MC_CMD_INIT_RXQ_V5_IN_DMA_ADDR_LO_OFST: c_int = 28;
pub const MC_CMD_INIT_RXQ_V5_IN_DMA_ADDR_HI_OFST: c_int = 32;
pub const MC_CMD_INIT_RXQ_V5_IN_DMA_ADDR_NUM: c_int = 64;
// Maximum length of packet to receive, if SNAPSHOT_MODE flag is set
pub const MC_CMD_INIT_RXQ_V5_IN_SNAPSHOT_LENGTH_OFST: c_int = 540;
pub const MC_CMD_INIT_RXQ_V5_IN_SNAPSHOT_LENGTH_LEN: c_int = 4;
// The number of packet buffers that will be contained within each
// EQUAL_STRIDE_SUPER_BUFFER format bucket supplied by the driver. This field
// is ignored unless DMA_MODE == EQUAL_STRIDE_SUPER_BUFFER.
//
pub const MC_CMD_INIT_RXQ_V5_IN_ES_PACKET_BUFFERS_PER_BUCKET_OFST: c_int = 544;
pub const MC_CMD_INIT_RXQ_V5_IN_ES_PACKET_BUFFERS_PER_BUCKET_LEN: c_int = 4;
// The length in bytes of the area in each packet buffer that can be written to
// by the adapter. This is used to store the packet prefix and the packet
// payload. This length does not include any end padding added by the driver.
// This field is ignored unless DMA_MODE == EQUAL_STRIDE_SUPER_BUFFER.
//
pub const MC_CMD_INIT_RXQ_V5_IN_ES_MAX_DMA_LEN_OFST: c_int = 548;
pub const MC_CMD_INIT_RXQ_V5_IN_ES_MAX_DMA_LEN_LEN: c_int = 4;
// The length in bytes of a single packet buffer within a
// EQUAL_STRIDE_SUPER_BUFFER format bucket. This field is ignored unless
// DMA_MODE == EQUAL_STRIDE_SUPER_BUFFER.
//
pub const MC_CMD_INIT_RXQ_V5_IN_ES_PACKET_STRIDE_OFST: c_int = 552;
pub const MC_CMD_INIT_RXQ_V5_IN_ES_PACKET_STRIDE_LEN: c_int = 4;
// The maximum time in nanoseconds that the datapath will be backpressured if
// there are no RX descriptors available. If the timeout is reached and there
// are still no descriptors then the packet will be dropped. A timeout of 0
// means the datapath will never be blocked. This field is ignored unless
// DMA_MODE == EQUAL_STRIDE_SUPER_BUFFER.
//
pub const MC_CMD_INIT_RXQ_V5_IN_ES_HEAD_OF_LINE_BLOCK_TIMEOUT_OFST: c_int = 556;
pub const MC_CMD_INIT_RXQ_V5_IN_ES_HEAD_OF_LINE_BLOCK_TIMEOUT_LEN: c_int = 4;
// V4 message data
pub const MC_CMD_INIT_RXQ_V5_IN_V4_DATA_OFST: c_int = 560;
pub const MC_CMD_INIT_RXQ_V5_IN_V4_DATA_LEN: c_int = 4;
// Size in bytes of buffers attached to descriptors posted to this queue. Set
// to zero if using this message on non-QDMA based platforms. Currently in
// Riverhead there is a global limit of eight different buffer sizes across all
// active queues. A 2KB and 4KB buffer is guaranteed to be available, but a
// request for a different buffer size will fail if there are already eight
// other buffer sizes in use. In future Riverhead this limit will go away and
// any size will be accepted.
//
pub const MC_CMD_INIT_RXQ_V5_IN_BUFFER_SIZE_BYTES_OFST: c_int = 560;
pub const MC_CMD_INIT_RXQ_V5_IN_BUFFER_SIZE_BYTES_LEN: c_int = 4;
// Prefix id for the RX prefix format to use on packets delivered this queue.
// Zero is always a valid prefix id and means the default prefix format
// documented for the platform. Other prefix ids can be obtained by calling
// MC_CMD_GET_RX_PREFIX_ID with a requested set of prefix fields.
//
pub const MC_CMD_INIT_RXQ_V5_IN_RX_PREFIX_ID_OFST: c_int = 564;
pub const MC_CMD_INIT_RXQ_V5_IN_RX_PREFIX_ID_LEN: c_int = 4;
// MC_CMD_INIT_RXQ_OUT msgresponse
pub const MC_CMD_INIT_RXQ_OUT_LEN: c_int = 0;
// MC_CMD_INIT_RXQ_EXT_OUT msgresponse
pub const MC_CMD_INIT_RXQ_EXT_OUT_LEN: c_int = 0;
// MC_CMD_INIT_RXQ_V3_OUT msgresponse
pub const MC_CMD_INIT_RXQ_V3_OUT_LEN: c_int = 0;
// MC_CMD_INIT_RXQ_V4_OUT msgresponse
pub const MC_CMD_INIT_RXQ_V4_OUT_LEN: c_int = 0;
// MC_CMD_INIT_RXQ_V5_OUT msgresponse
pub const MC_CMD_INIT_RXQ_V5_OUT_LEN: c_int = 0;
//
// MC_CMD_INIT_TXQ
//
pub const MC_CMD_INIT_TXQ: c_uint = 0x82;

// MC_CMD_INIT_TXQ_IN msgrequest: Legacy INIT_TXQ request. Use extended version
// in new code.
//
pub const MC_CMD_INIT_TXQ_IN_LENMIN: c_int = 36;
pub const MC_CMD_INIT_TXQ_IN_LENMAX: c_int = 252;
pub const MC_CMD_INIT_TXQ_IN_LENMAX_MCDI2: c_int = 1020;

// Size, in entries
pub const MC_CMD_INIT_TXQ_IN_SIZE_OFST: c_int = 0;
pub const MC_CMD_INIT_TXQ_IN_SIZE_LEN: c_int = 4;
// The EVQ to send events to. This is an index originally specified to
// INIT_EVQ.
//
pub const MC_CMD_INIT_TXQ_IN_TARGET_EVQ_OFST: c_int = 4;
pub const MC_CMD_INIT_TXQ_IN_TARGET_EVQ_LEN: c_int = 4;
// The value to put in the event data. Check hardware spec. for valid range.
pub const MC_CMD_INIT_TXQ_IN_LABEL_OFST: c_int = 8;
pub const MC_CMD_INIT_TXQ_IN_LABEL_LEN: c_int = 4;
// Desired instance. Must be set to a specific instance, which is a function
// local queue index.
//
pub const MC_CMD_INIT_TXQ_IN_INSTANCE_OFST: c_int = 12;
pub const MC_CMD_INIT_TXQ_IN_INSTANCE_LEN: c_int = 4;
// There will be more flags here.
pub const MC_CMD_INIT_TXQ_IN_FLAGS_OFST: c_int = 16;
pub const MC_CMD_INIT_TXQ_IN_FLAGS_LEN: c_int = 4;
pub const MC_CMD_INIT_TXQ_IN_FLAG_BUFF_MODE_OFST: c_int = 16;
pub const MC_CMD_INIT_TXQ_IN_FLAG_BUFF_MODE_LBN: c_int = 0;
pub const MC_CMD_INIT_TXQ_IN_FLAG_BUFF_MODE_WIDTH: c_int = 1;
pub const MC_CMD_INIT_TXQ_IN_FLAG_IP_CSUM_DIS_OFST: c_int = 16;
pub const MC_CMD_INIT_TXQ_IN_FLAG_IP_CSUM_DIS_LBN: c_int = 1;
pub const MC_CMD_INIT_TXQ_IN_FLAG_IP_CSUM_DIS_WIDTH: c_int = 1;
pub const MC_CMD_INIT_TXQ_IN_FLAG_TCP_CSUM_DIS_OFST: c_int = 16;
pub const MC_CMD_INIT_TXQ_IN_FLAG_TCP_CSUM_DIS_LBN: c_int = 2;
pub const MC_CMD_INIT_TXQ_IN_FLAG_TCP_CSUM_DIS_WIDTH: c_int = 1;
pub const MC_CMD_INIT_TXQ_IN_FLAG_TCP_UDP_ONLY_OFST: c_int = 16;
pub const MC_CMD_INIT_TXQ_IN_FLAG_TCP_UDP_ONLY_LBN: c_int = 3;
pub const MC_CMD_INIT_TXQ_IN_FLAG_TCP_UDP_ONLY_WIDTH: c_int = 1;
pub const MC_CMD_INIT_TXQ_IN_CRC_MODE_OFST: c_int = 16;
pub const MC_CMD_INIT_TXQ_IN_CRC_MODE_LBN: c_int = 4;
pub const MC_CMD_INIT_TXQ_IN_CRC_MODE_WIDTH: c_int = 4;
pub const MC_CMD_INIT_TXQ_IN_FLAG_TIMESTAMP_OFST: c_int = 16;
pub const MC_CMD_INIT_TXQ_IN_FLAG_TIMESTAMP_LBN: c_int = 8;
pub const MC_CMD_INIT_TXQ_IN_FLAG_TIMESTAMP_WIDTH: c_int = 1;
pub const MC_CMD_INIT_TXQ_IN_FLAG_PACER_BYPASS_OFST: c_int = 16;
pub const MC_CMD_INIT_TXQ_IN_FLAG_PACER_BYPASS_LBN: c_int = 9;
pub const MC_CMD_INIT_TXQ_IN_FLAG_PACER_BYPASS_WIDTH: c_int = 1;
pub const MC_CMD_INIT_TXQ_IN_FLAG_INNER_IP_CSUM_EN_OFST: c_int = 16;
pub const MC_CMD_INIT_TXQ_IN_FLAG_INNER_IP_CSUM_EN_LBN: c_int = 10;
pub const MC_CMD_INIT_TXQ_IN_FLAG_INNER_IP_CSUM_EN_WIDTH: c_int = 1;
pub const MC_CMD_INIT_TXQ_IN_FLAG_INNER_TCP_CSUM_EN_OFST: c_int = 16;
pub const MC_CMD_INIT_TXQ_IN_FLAG_INNER_TCP_CSUM_EN_LBN: c_int = 11;
pub const MC_CMD_INIT_TXQ_IN_FLAG_INNER_TCP_CSUM_EN_WIDTH: c_int = 1;
// Owner ID to use if in buffer mode (zero if physical)
pub const MC_CMD_INIT_TXQ_IN_OWNER_ID_OFST: c_int = 20;
pub const MC_CMD_INIT_TXQ_IN_OWNER_ID_LEN: c_int = 4;
// The port ID associated with the v-adaptor which should contain this DMAQ.
pub const MC_CMD_INIT_TXQ_IN_PORT_ID_OFST: c_int = 24;
pub const MC_CMD_INIT_TXQ_IN_PORT_ID_LEN: c_int = 4;
// 64-bit address of 4k of 4k-aligned host memory buffer
pub const MC_CMD_INIT_TXQ_IN_DMA_ADDR_OFST: c_int = 28;
pub const MC_CMD_INIT_TXQ_IN_DMA_ADDR_LEN: c_int = 8;
pub const MC_CMD_INIT_TXQ_IN_DMA_ADDR_LO_OFST: c_int = 28;
pub const MC_CMD_INIT_TXQ_IN_DMA_ADDR_HI_OFST: c_int = 32;
pub const MC_CMD_INIT_TXQ_IN_DMA_ADDR_MINNUM: c_int = 1;
pub const MC_CMD_INIT_TXQ_IN_DMA_ADDR_MAXNUM: c_int = 28;
pub const MC_CMD_INIT_TXQ_IN_DMA_ADDR_MAXNUM_MCDI2: c_int = 124;
// MC_CMD_INIT_TXQ_EXT_IN msgrequest: Extended INIT_TXQ with additional mode
// flags
//
pub const MC_CMD_INIT_TXQ_EXT_IN_LEN: c_int = 544;
// Size, in entries
pub const MC_CMD_INIT_TXQ_EXT_IN_SIZE_OFST: c_int = 0;
pub const MC_CMD_INIT_TXQ_EXT_IN_SIZE_LEN: c_int = 4;
// The EVQ to send events to. This is an index originally specified to
// INIT_EVQ.
//
pub const MC_CMD_INIT_TXQ_EXT_IN_TARGET_EVQ_OFST: c_int = 4;
pub const MC_CMD_INIT_TXQ_EXT_IN_TARGET_EVQ_LEN: c_int = 4;
// The value to put in the event data. Check hardware spec. for valid range.
pub const MC_CMD_INIT_TXQ_EXT_IN_LABEL_OFST: c_int = 8;
pub const MC_CMD_INIT_TXQ_EXT_IN_LABEL_LEN: c_int = 4;
// Desired instance. Must be set to a specific instance, which is a function
// local queue index.
//
pub const MC_CMD_INIT_TXQ_EXT_IN_INSTANCE_OFST: c_int = 12;
pub const MC_CMD_INIT_TXQ_EXT_IN_INSTANCE_LEN: c_int = 4;
// There will be more flags here.
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAGS_OFST: c_int = 16;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAGS_LEN: c_int = 4;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_BUFF_MODE_OFST: c_int = 16;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_BUFF_MODE_LBN: c_int = 0;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_BUFF_MODE_WIDTH: c_int = 1;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_IP_CSUM_DIS_OFST: c_int = 16;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_IP_CSUM_DIS_LBN: c_int = 1;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_IP_CSUM_DIS_WIDTH: c_int = 1;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_TCP_CSUM_DIS_OFST: c_int = 16;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_TCP_CSUM_DIS_LBN: c_int = 2;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_TCP_CSUM_DIS_WIDTH: c_int = 1;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_TCP_UDP_ONLY_OFST: c_int = 16;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_TCP_UDP_ONLY_LBN: c_int = 3;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_TCP_UDP_ONLY_WIDTH: c_int = 1;
pub const MC_CMD_INIT_TXQ_EXT_IN_CRC_MODE_OFST: c_int = 16;
pub const MC_CMD_INIT_TXQ_EXT_IN_CRC_MODE_LBN: c_int = 4;
pub const MC_CMD_INIT_TXQ_EXT_IN_CRC_MODE_WIDTH: c_int = 4;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_TIMESTAMP_OFST: c_int = 16;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_TIMESTAMP_LBN: c_int = 8;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_TIMESTAMP_WIDTH: c_int = 1;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_PACER_BYPASS_OFST: c_int = 16;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_PACER_BYPASS_LBN: c_int = 9;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_PACER_BYPASS_WIDTH: c_int = 1;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_INNER_IP_CSUM_EN_OFST: c_int = 16;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_INNER_IP_CSUM_EN_LBN: c_int = 10;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_INNER_IP_CSUM_EN_WIDTH: c_int = 1;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_INNER_TCP_CSUM_EN_OFST: c_int = 16;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_INNER_TCP_CSUM_EN_LBN: c_int = 11;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_INNER_TCP_CSUM_EN_WIDTH: c_int = 1;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_TSOV2_EN_OFST: c_int = 16;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_TSOV2_EN_LBN: c_int = 12;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_TSOV2_EN_WIDTH: c_int = 1;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_CTPIO_OFST: c_int = 16;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_CTPIO_LBN: c_int = 13;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_CTPIO_WIDTH: c_int = 1;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_CTPIO_UTHRESH_OFST: c_int = 16;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_CTPIO_UTHRESH_LBN: c_int = 14;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_CTPIO_UTHRESH_WIDTH: c_int = 1;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_M2M_D2C_OFST: c_int = 16;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_M2M_D2C_LBN: c_int = 15;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_M2M_D2C_WIDTH: c_int = 1;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_DESC_PROXY_OFST: c_int = 16;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_DESC_PROXY_LBN: c_int = 16;
pub const MC_CMD_INIT_TXQ_EXT_IN_FLAG_DESC_PROXY_WIDTH: c_int = 1;
// Owner ID to use if in buffer mode (zero if physical)
pub const MC_CMD_INIT_TXQ_EXT_IN_OWNER_ID_OFST: c_int = 20;
pub const MC_CMD_INIT_TXQ_EXT_IN_OWNER_ID_LEN: c_int = 4;
// The port ID associated with the v-adaptor which should contain this DMAQ.
pub const MC_CMD_INIT_TXQ_EXT_IN_PORT_ID_OFST: c_int = 24;
pub const MC_CMD_INIT_TXQ_EXT_IN_PORT_ID_LEN: c_int = 4;
// 64-bit address of 4k of 4k-aligned host memory buffer
pub const MC_CMD_INIT_TXQ_EXT_IN_DMA_ADDR_OFST: c_int = 28;
pub const MC_CMD_INIT_TXQ_EXT_IN_DMA_ADDR_LEN: c_int = 8;
pub const MC_CMD_INIT_TXQ_EXT_IN_DMA_ADDR_LO_OFST: c_int = 28;
pub const MC_CMD_INIT_TXQ_EXT_IN_DMA_ADDR_HI_OFST: c_int = 32;
pub const MC_CMD_INIT_TXQ_EXT_IN_DMA_ADDR_MINNUM: c_int = 1;
pub const MC_CMD_INIT_TXQ_EXT_IN_DMA_ADDR_MAXNUM: c_int = 64;
pub const MC_CMD_INIT_TXQ_EXT_IN_DMA_ADDR_MAXNUM_MCDI2: c_int = 64;
// Flags related to Qbb flow control mode.
pub const MC_CMD_INIT_TXQ_EXT_IN_QBB_FLAGS_OFST: c_int = 540;
pub const MC_CMD_INIT_TXQ_EXT_IN_QBB_FLAGS_LEN: c_int = 4;
pub const MC_CMD_INIT_TXQ_EXT_IN_QBB_ENABLE_OFST: c_int = 540;
pub const MC_CMD_INIT_TXQ_EXT_IN_QBB_ENABLE_LBN: c_int = 0;
pub const MC_CMD_INIT_TXQ_EXT_IN_QBB_ENABLE_WIDTH: c_int = 1;
pub const MC_CMD_INIT_TXQ_EXT_IN_QBB_PRIORITY_OFST: c_int = 540;
pub const MC_CMD_INIT_TXQ_EXT_IN_QBB_PRIORITY_LBN: c_int = 1;
pub const MC_CMD_INIT_TXQ_EXT_IN_QBB_PRIORITY_WIDTH: c_int = 3;
// MC_CMD_INIT_TXQ_OUT msgresponse
pub const MC_CMD_INIT_TXQ_OUT_LEN: c_int = 0;
//
// MC_CMD_FINI_EVQ
// Teardown an EVQ.
//
// All DMAQs or EVQs that point to the EVQ to tear down must be torn down first
// or the operation will fail with EBUSY
//
pub const MC_CMD_FINI_EVQ: c_uint = 0x83;

// MC_CMD_FINI_EVQ_IN msgrequest
pub const MC_CMD_FINI_EVQ_IN_LEN: c_int = 4;
// Instance of EVQ to destroy. Should be the same instance as that previously
// passed to INIT_EVQ
//
pub const MC_CMD_FINI_EVQ_IN_INSTANCE_OFST: c_int = 0;
pub const MC_CMD_FINI_EVQ_IN_INSTANCE_LEN: c_int = 4;
// MC_CMD_FINI_EVQ_OUT msgresponse
pub const MC_CMD_FINI_EVQ_OUT_LEN: c_int = 0;
//
// MC_CMD_FINI_RXQ
// Teardown a RXQ.
//
pub const MC_CMD_FINI_RXQ: c_uint = 0x84;

// MC_CMD_FINI_RXQ_IN msgrequest
pub const MC_CMD_FINI_RXQ_IN_LEN: c_int = 4;
// Instance of RXQ to destroy
pub const MC_CMD_FINI_RXQ_IN_INSTANCE_OFST: c_int = 0;
pub const MC_CMD_FINI_RXQ_IN_INSTANCE_LEN: c_int = 4;
// MC_CMD_FINI_RXQ_OUT msgresponse
pub const MC_CMD_FINI_RXQ_OUT_LEN: c_int = 0;
//
// MC_CMD_FINI_TXQ
// Teardown a TXQ.
//
pub const MC_CMD_FINI_TXQ: c_uint = 0x85;

// MC_CMD_FINI_TXQ_IN msgrequest
pub const MC_CMD_FINI_TXQ_IN_LEN: c_int = 4;
// Instance of TXQ to destroy
pub const MC_CMD_FINI_TXQ_IN_INSTANCE_OFST: c_int = 0;
pub const MC_CMD_FINI_TXQ_IN_INSTANCE_LEN: c_int = 4;
// MC_CMD_FINI_TXQ_OUT msgresponse
pub const MC_CMD_FINI_TXQ_OUT_LEN: c_int = 0;
//
// MC_CMD_DRIVER_EVENT
// Generate an event on an EVQ belonging to the function issuing the command.
//
pub const MC_CMD_DRIVER_EVENT: c_uint = 0x86;

// MC_CMD_DRIVER_EVENT_IN msgrequest
pub const MC_CMD_DRIVER_EVENT_IN_LEN: c_int = 12;
// Handle of target EVQ
pub const MC_CMD_DRIVER_EVENT_IN_EVQ_OFST: c_int = 0;
pub const MC_CMD_DRIVER_EVENT_IN_EVQ_LEN: c_int = 4;
// Bits 0 - 63 of event
pub const MC_CMD_DRIVER_EVENT_IN_DATA_OFST: c_int = 4;
pub const MC_CMD_DRIVER_EVENT_IN_DATA_LEN: c_int = 8;
pub const MC_CMD_DRIVER_EVENT_IN_DATA_LO_OFST: c_int = 4;
pub const MC_CMD_DRIVER_EVENT_IN_DATA_HI_OFST: c_int = 8;
// MC_CMD_DRIVER_EVENT_OUT msgresponse
pub const MC_CMD_DRIVER_EVENT_OUT_LEN: c_int = 0;
//
// MC_CMD_ALLOC_BUFTBL_CHUNK
// Allocate a set of buffer table entries using the specified owner ID. This
// operation allocates the required buffer table entries (and fails if it
// cannot do so). The buffer table entries will initially be zeroed.
//
pub const MC_CMD_ALLOC_BUFTBL_CHUNK: c_uint = 0x87;

// MC_CMD_ALLOC_BUFTBL_CHUNK_IN msgrequest
pub const MC_CMD_ALLOC_BUFTBL_CHUNK_IN_LEN: c_int = 8;
// Owner ID to use
pub const MC_CMD_ALLOC_BUFTBL_CHUNK_IN_OWNER_OFST: c_int = 0;
pub const MC_CMD_ALLOC_BUFTBL_CHUNK_IN_OWNER_LEN: c_int = 4;
// Size of buffer table pages to use, in bytes (note that only a few values are
// legal on any specific hardware).
//
pub const MC_CMD_ALLOC_BUFTBL_CHUNK_IN_PAGE_SIZE_OFST: c_int = 4;
pub const MC_CMD_ALLOC_BUFTBL_CHUNK_IN_PAGE_SIZE_LEN: c_int = 4;
// MC_CMD_ALLOC_BUFTBL_CHUNK_OUT msgresponse
pub const MC_CMD_ALLOC_BUFTBL_CHUNK_OUT_LEN: c_int = 12;
pub const MC_CMD_ALLOC_BUFTBL_CHUNK_OUT_HANDLE_OFST: c_int = 0;
pub const MC_CMD_ALLOC_BUFTBL_CHUNK_OUT_HANDLE_LEN: c_int = 4;
pub const MC_CMD_ALLOC_BUFTBL_CHUNK_OUT_NUMENTRIES_OFST: c_int = 4;
pub const MC_CMD_ALLOC_BUFTBL_CHUNK_OUT_NUMENTRIES_LEN: c_int = 4;
// Buffer table IDs for use in DMA descriptors.
pub const MC_CMD_ALLOC_BUFTBL_CHUNK_OUT_ID_OFST: c_int = 8;
pub const MC_CMD_ALLOC_BUFTBL_CHUNK_OUT_ID_LEN: c_int = 4;
//
// MC_CMD_PROGRAM_BUFTBL_ENTRIES
// Reprogram a set of buffer table entries in the specified chunk.
//
pub const MC_CMD_PROGRAM_BUFTBL_ENTRIES: c_uint = 0x88;

// MC_CMD_PROGRAM_BUFTBL_ENTRIES_IN msgrequest
pub const MC_CMD_PROGRAM_BUFTBL_ENTRIES_IN_LENMIN: c_int = 20;
pub const MC_CMD_PROGRAM_BUFTBL_ENTRIES_IN_LENMAX: c_int = 268;
pub const MC_CMD_PROGRAM_BUFTBL_ENTRIES_IN_LENMAX_MCDI2: c_int = 268;

pub const MC_CMD_PROGRAM_BUFTBL_ENTRIES_IN_HANDLE_OFST: c_int = 0;
pub const MC_CMD_PROGRAM_BUFTBL_ENTRIES_IN_HANDLE_LEN: c_int = 4;
// ID
pub const MC_CMD_PROGRAM_BUFTBL_ENTRIES_IN_FIRSTID_OFST: c_int = 4;
pub const MC_CMD_PROGRAM_BUFTBL_ENTRIES_IN_FIRSTID_LEN: c_int = 4;
// Num entries
pub const MC_CMD_PROGRAM_BUFTBL_ENTRIES_IN_NUMENTRIES_OFST: c_int = 8;
pub const MC_CMD_PROGRAM_BUFTBL_ENTRIES_IN_NUMENTRIES_LEN: c_int = 4;
// Buffer table entry address
pub const MC_CMD_PROGRAM_BUFTBL_ENTRIES_IN_ENTRY_OFST: c_int = 12;
pub const MC_CMD_PROGRAM_BUFTBL_ENTRIES_IN_ENTRY_LEN: c_int = 8;
pub const MC_CMD_PROGRAM_BUFTBL_ENTRIES_IN_ENTRY_LO_OFST: c_int = 12;
pub const MC_CMD_PROGRAM_BUFTBL_ENTRIES_IN_ENTRY_HI_OFST: c_int = 16;
pub const MC_CMD_PROGRAM_BUFTBL_ENTRIES_IN_ENTRY_MINNUM: c_int = 1;
pub const MC_CMD_PROGRAM_BUFTBL_ENTRIES_IN_ENTRY_MAXNUM: c_int = 32;
pub const MC_CMD_PROGRAM_BUFTBL_ENTRIES_IN_ENTRY_MAXNUM_MCDI2: c_int = 32;
// MC_CMD_PROGRAM_BUFTBL_ENTRIES_OUT msgresponse
pub const MC_CMD_PROGRAM_BUFTBL_ENTRIES_OUT_LEN: c_int = 0;
//
// MC_CMD_FREE_BUFTBL_CHUNK
//
pub const MC_CMD_FREE_BUFTBL_CHUNK: c_uint = 0x89;

// MC_CMD_FREE_BUFTBL_CHUNK_IN msgrequest
pub const MC_CMD_FREE_BUFTBL_CHUNK_IN_LEN: c_int = 4;
pub const MC_CMD_FREE_BUFTBL_CHUNK_IN_HANDLE_OFST: c_int = 0;
pub const MC_CMD_FREE_BUFTBL_CHUNK_IN_HANDLE_LEN: c_int = 4;
// MC_CMD_FREE_BUFTBL_CHUNK_OUT msgresponse
pub const MC_CMD_FREE_BUFTBL_CHUNK_OUT_LEN: c_int = 0;
//
// MC_CMD_FILTER_OP
// Multiplexed MCDI call for filter operations
//
pub const MC_CMD_FILTER_OP: c_uint = 0x8a;

// MC_CMD_FILTER_OP_IN msgrequest
pub const MC_CMD_FILTER_OP_IN_LEN: c_int = 108;
// identifies the type of operation requested
pub const MC_CMD_FILTER_OP_IN_OP_OFST: c_int = 0;
pub const MC_CMD_FILTER_OP_IN_OP_LEN: c_int = 4;
// enum: single-recipient filter insert
pub const MC_CMD_FILTER_OP_IN_OP_INSERT: c_uint = 0x0;
// enum: single-recipient filter remove
pub const MC_CMD_FILTER_OP_IN_OP_REMOVE: c_uint = 0x1;
// enum: multi-recipient filter subscribe
pub const MC_CMD_FILTER_OP_IN_OP_SUBSCRIBE: c_uint = 0x2;
// enum: multi-recipient filter unsubscribe
pub const MC_CMD_FILTER_OP_IN_OP_UNSUBSCRIBE: c_uint = 0x3;
// enum: replace one recipient with another (warning - the filter handle may
// change)
//
pub const MC_CMD_FILTER_OP_IN_OP_REPLACE: c_uint = 0x4;
// filter handle (for remove / unsubscribe operations)
pub const MC_CMD_FILTER_OP_IN_HANDLE_OFST: c_int = 4;
pub const MC_CMD_FILTER_OP_IN_HANDLE_LEN: c_int = 8;
pub const MC_CMD_FILTER_OP_IN_HANDLE_LO_OFST: c_int = 4;
pub const MC_CMD_FILTER_OP_IN_HANDLE_HI_OFST: c_int = 8;
// The port ID associated with the v-adaptor which should contain this filter.
//
pub const MC_CMD_FILTER_OP_IN_PORT_ID_OFST: c_int = 12;
pub const MC_CMD_FILTER_OP_IN_PORT_ID_LEN: c_int = 4;
// fields to include in match criteria
pub const MC_CMD_FILTER_OP_IN_MATCH_FIELDS_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_IN_MATCH_FIELDS_LEN: c_int = 4;
pub const MC_CMD_FILTER_OP_IN_MATCH_SRC_IP_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_IN_MATCH_SRC_IP_LBN: c_int = 0;
pub const MC_CMD_FILTER_OP_IN_MATCH_SRC_IP_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_IN_MATCH_DST_IP_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_IN_MATCH_DST_IP_LBN: c_int = 1;
pub const MC_CMD_FILTER_OP_IN_MATCH_DST_IP_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_IN_MATCH_SRC_MAC_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_IN_MATCH_SRC_MAC_LBN: c_int = 2;
pub const MC_CMD_FILTER_OP_IN_MATCH_SRC_MAC_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_IN_MATCH_SRC_PORT_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_IN_MATCH_SRC_PORT_LBN: c_int = 3;
pub const MC_CMD_FILTER_OP_IN_MATCH_SRC_PORT_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_IN_MATCH_DST_MAC_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_IN_MATCH_DST_MAC_LBN: c_int = 4;
pub const MC_CMD_FILTER_OP_IN_MATCH_DST_MAC_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_IN_MATCH_DST_PORT_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_IN_MATCH_DST_PORT_LBN: c_int = 5;
pub const MC_CMD_FILTER_OP_IN_MATCH_DST_PORT_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_IN_MATCH_ETHER_TYPE_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_IN_MATCH_ETHER_TYPE_LBN: c_int = 6;
pub const MC_CMD_FILTER_OP_IN_MATCH_ETHER_TYPE_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_IN_MATCH_INNER_VLAN_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_IN_MATCH_INNER_VLAN_LBN: c_int = 7;
pub const MC_CMD_FILTER_OP_IN_MATCH_INNER_VLAN_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_IN_MATCH_OUTER_VLAN_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_IN_MATCH_OUTER_VLAN_LBN: c_int = 8;
pub const MC_CMD_FILTER_OP_IN_MATCH_OUTER_VLAN_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_IN_MATCH_IP_PROTO_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_IN_MATCH_IP_PROTO_LBN: c_int = 9;
pub const MC_CMD_FILTER_OP_IN_MATCH_IP_PROTO_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_IN_MATCH_FWDEF0_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_IN_MATCH_FWDEF0_LBN: c_int = 10;
pub const MC_CMD_FILTER_OP_IN_MATCH_FWDEF0_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_IN_MATCH_FWDEF1_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_IN_MATCH_FWDEF1_LBN: c_int = 11;
pub const MC_CMD_FILTER_OP_IN_MATCH_FWDEF1_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_IN_MATCH_UNKNOWN_MCAST_DST_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_IN_MATCH_UNKNOWN_MCAST_DST_LBN: c_int = 30;
pub const MC_CMD_FILTER_OP_IN_MATCH_UNKNOWN_MCAST_DST_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_IN_MATCH_UNKNOWN_UCAST_DST_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_IN_MATCH_UNKNOWN_UCAST_DST_LBN: c_int = 31;
pub const MC_CMD_FILTER_OP_IN_MATCH_UNKNOWN_UCAST_DST_WIDTH: c_int = 1;
// receive destination
pub const MC_CMD_FILTER_OP_IN_RX_DEST_OFST: c_int = 20;
pub const MC_CMD_FILTER_OP_IN_RX_DEST_LEN: c_int = 4;
// enum: drop packets
pub const MC_CMD_FILTER_OP_IN_RX_DEST_DROP: c_uint = 0x0;
// enum: receive to host
pub const MC_CMD_FILTER_OP_IN_RX_DEST_HOST: c_uint = 0x1;
// enum: receive to MC
pub const MC_CMD_FILTER_OP_IN_RX_DEST_MC: c_uint = 0x2;
// enum: loop back to TXDP 0
pub const MC_CMD_FILTER_OP_IN_RX_DEST_TX0: c_uint = 0x3;
// enum: loop back to TXDP 1
pub const MC_CMD_FILTER_OP_IN_RX_DEST_TX1: c_uint = 0x4;
// receive queue handle (for multiple queue modes, this is the base queue)
pub const MC_CMD_FILTER_OP_IN_RX_QUEUE_OFST: c_int = 24;
pub const MC_CMD_FILTER_OP_IN_RX_QUEUE_LEN: c_int = 4;
// receive mode
pub const MC_CMD_FILTER_OP_IN_RX_MODE_OFST: c_int = 28;
pub const MC_CMD_FILTER_OP_IN_RX_MODE_LEN: c_int = 4;
// enum: receive to just the specified queue
pub const MC_CMD_FILTER_OP_IN_RX_MODE_SIMPLE: c_uint = 0x0;
// enum: receive to multiple queues using RSS context
pub const MC_CMD_FILTER_OP_IN_RX_MODE_RSS: c_uint = 0x1;
// enum: receive to multiple queues using .1p mapping
pub const MC_CMD_FILTER_OP_IN_RX_MODE_DOT1P_MAPPING: c_uint = 0x2;
// enum: install a filter entry that will never match; for test purposes only
//
pub const MC_CMD_FILTER_OP_IN_RX_MODE_TEST_NEVER_MATCH: c_uint = 0x80000000;
// RSS context (for RX_MODE_RSS) or .1p mapping handle (for
// RX_MODE_DOT1P_MAPPING), as returned by MC_CMD_RSS_CONTEXT_ALLOC or
// MC_CMD_DOT1P_MAPPING_ALLOC.
//
pub const MC_CMD_FILTER_OP_IN_RX_CONTEXT_OFST: c_int = 32;
pub const MC_CMD_FILTER_OP_IN_RX_CONTEXT_LEN: c_int = 4;
// transmit domain (reserved; set to 0)
pub const MC_CMD_FILTER_OP_IN_TX_DOMAIN_OFST: c_int = 36;
pub const MC_CMD_FILTER_OP_IN_TX_DOMAIN_LEN: c_int = 4;
// transmit destination (either set the MAC and/or PM bits for explicit
// control, or set this field to TX_DEST_DEFAULT for sensible default
// behaviour)
//
pub const MC_CMD_FILTER_OP_IN_TX_DEST_OFST: c_int = 40;
pub const MC_CMD_FILTER_OP_IN_TX_DEST_LEN: c_int = 4;
// enum: request default behaviour (based on filter type)
pub const MC_CMD_FILTER_OP_IN_TX_DEST_DEFAULT: c_uint = 0xffffffff;
pub const MC_CMD_FILTER_OP_IN_TX_DEST_MAC_OFST: c_int = 40;
pub const MC_CMD_FILTER_OP_IN_TX_DEST_MAC_LBN: c_int = 0;
pub const MC_CMD_FILTER_OP_IN_TX_DEST_MAC_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_IN_TX_DEST_PM_OFST: c_int = 40;
pub const MC_CMD_FILTER_OP_IN_TX_DEST_PM_LBN: c_int = 1;
pub const MC_CMD_FILTER_OP_IN_TX_DEST_PM_WIDTH: c_int = 1;
// source MAC address to match (as bytes in network order)
pub const MC_CMD_FILTER_OP_IN_SRC_MAC_OFST: c_int = 44;
pub const MC_CMD_FILTER_OP_IN_SRC_MAC_LEN: c_int = 6;
// source port to match (as bytes in network order)
pub const MC_CMD_FILTER_OP_IN_SRC_PORT_OFST: c_int = 50;
pub const MC_CMD_FILTER_OP_IN_SRC_PORT_LEN: c_int = 2;
// destination MAC address to match (as bytes in network order)
pub const MC_CMD_FILTER_OP_IN_DST_MAC_OFST: c_int = 52;
pub const MC_CMD_FILTER_OP_IN_DST_MAC_LEN: c_int = 6;
// destination port to match (as bytes in network order)
pub const MC_CMD_FILTER_OP_IN_DST_PORT_OFST: c_int = 58;
pub const MC_CMD_FILTER_OP_IN_DST_PORT_LEN: c_int = 2;
// Ethernet type to match (as bytes in network order)
pub const MC_CMD_FILTER_OP_IN_ETHER_TYPE_OFST: c_int = 60;
pub const MC_CMD_FILTER_OP_IN_ETHER_TYPE_LEN: c_int = 2;
// Inner VLAN tag to match (as bytes in network order)
pub const MC_CMD_FILTER_OP_IN_INNER_VLAN_OFST: c_int = 62;
pub const MC_CMD_FILTER_OP_IN_INNER_VLAN_LEN: c_int = 2;
// Outer VLAN tag to match (as bytes in network order)
pub const MC_CMD_FILTER_OP_IN_OUTER_VLAN_OFST: c_int = 64;
pub const MC_CMD_FILTER_OP_IN_OUTER_VLAN_LEN: c_int = 2;
// IP protocol to match (in low byte; set high byte to 0)
pub const MC_CMD_FILTER_OP_IN_IP_PROTO_OFST: c_int = 66;
pub const MC_CMD_FILTER_OP_IN_IP_PROTO_LEN: c_int = 2;
// Firmware defined register 0 to match (reserved; set to 0)
pub const MC_CMD_FILTER_OP_IN_FWDEF0_OFST: c_int = 68;
pub const MC_CMD_FILTER_OP_IN_FWDEF0_LEN: c_int = 4;
// Firmware defined register 1 to match (reserved; set to 0)
pub const MC_CMD_FILTER_OP_IN_FWDEF1_OFST: c_int = 72;
pub const MC_CMD_FILTER_OP_IN_FWDEF1_LEN: c_int = 4;
// source IP address to match (as bytes in network order; set last 12 bytes to
// 0 for IPv4 address)
//
pub const MC_CMD_FILTER_OP_IN_SRC_IP_OFST: c_int = 76;
pub const MC_CMD_FILTER_OP_IN_SRC_IP_LEN: c_int = 16;
// destination IP address to match (as bytes in network order; set last 12
// bytes to 0 for IPv4 address)
//
pub const MC_CMD_FILTER_OP_IN_DST_IP_OFST: c_int = 92;
pub const MC_CMD_FILTER_OP_IN_DST_IP_LEN: c_int = 16;
// MC_CMD_FILTER_OP_EXT_IN msgrequest: Extension to MC_CMD_FILTER_OP_IN to
// include handling of VXLAN/NVGRE encapsulated frame filtering (which is
// supported on Medford only).
//
pub const MC_CMD_FILTER_OP_EXT_IN_LEN: c_int = 172;
// identifies the type of operation requested
pub const MC_CMD_FILTER_OP_EXT_IN_OP_OFST: c_int = 0;
pub const MC_CMD_FILTER_OP_EXT_IN_OP_LEN: c_int = 4;
// Enum values, see field(s):
// MC_CMD_FILTER_OP_IN/OP
// filter handle (for remove / unsubscribe operations)
pub const MC_CMD_FILTER_OP_EXT_IN_HANDLE_OFST: c_int = 4;
pub const MC_CMD_FILTER_OP_EXT_IN_HANDLE_LEN: c_int = 8;
pub const MC_CMD_FILTER_OP_EXT_IN_HANDLE_LO_OFST: c_int = 4;
pub const MC_CMD_FILTER_OP_EXT_IN_HANDLE_HI_OFST: c_int = 8;
// The port ID associated with the v-adaptor which should contain this filter.
//
pub const MC_CMD_FILTER_OP_EXT_IN_PORT_ID_OFST: c_int = 12;
pub const MC_CMD_FILTER_OP_EXT_IN_PORT_ID_LEN: c_int = 4;
// fields to include in match criteria
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_FIELDS_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_FIELDS_LEN: c_int = 4;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_SRC_IP_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_SRC_IP_LBN: c_int = 0;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_SRC_IP_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_DST_IP_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_DST_IP_LBN: c_int = 1;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_DST_IP_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_SRC_MAC_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_SRC_MAC_LBN: c_int = 2;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_SRC_MAC_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_SRC_PORT_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_SRC_PORT_LBN: c_int = 3;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_SRC_PORT_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_DST_MAC_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_DST_MAC_LBN: c_int = 4;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_DST_MAC_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_DST_PORT_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_DST_PORT_LBN: c_int = 5;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_DST_PORT_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_ETHER_TYPE_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_ETHER_TYPE_LBN: c_int = 6;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_ETHER_TYPE_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_INNER_VLAN_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_INNER_VLAN_LBN: c_int = 7;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_INNER_VLAN_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_OUTER_VLAN_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_OUTER_VLAN_LBN: c_int = 8;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_OUTER_VLAN_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IP_PROTO_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IP_PROTO_LBN: c_int = 9;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IP_PROTO_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_FWDEF0_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_FWDEF0_LBN: c_int = 10;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_FWDEF0_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_VNI_OR_VSID_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_VNI_OR_VSID_LBN: c_int = 11;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_VNI_OR_VSID_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_SRC_IP_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_SRC_IP_LBN: c_int = 12;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_SRC_IP_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_DST_IP_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_DST_IP_LBN: c_int = 13;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_DST_IP_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_SRC_MAC_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_SRC_MAC_LBN: c_int = 14;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_SRC_MAC_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_SRC_PORT_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_SRC_PORT_LBN: c_int = 15;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_SRC_PORT_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_DST_MAC_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_DST_MAC_LBN: c_int = 16;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_DST_MAC_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_DST_PORT_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_DST_PORT_LBN: c_int = 17;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_DST_PORT_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_ETHER_TYPE_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_ETHER_TYPE_LBN: c_int = 18;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_ETHER_TYPE_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_INNER_VLAN_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_INNER_VLAN_LBN: c_int = 19;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_INNER_VLAN_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_OUTER_VLAN_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_OUTER_VLAN_LBN: c_int = 20;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_OUTER_VLAN_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_IP_PROTO_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_IP_PROTO_LBN: c_int = 21;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_IP_PROTO_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_FWDEF0_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_FWDEF0_LBN: c_int = 22;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_FWDEF0_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_FWDEF1_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_FWDEF1_LBN: c_int = 23;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_FWDEF1_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_UNKNOWN_MCAST_DST_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_UNKNOWN_MCAST_DST_LBN: c_int = 24;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_UNKNOWN_MCAST_DST_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_UNKNOWN_UCAST_DST_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_UNKNOWN_UCAST_DST_LBN: c_int = 25;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_IFRM_UNKNOWN_UCAST_DST_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_UNKNOWN_MCAST_DST_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_UNKNOWN_MCAST_DST_LBN: c_int = 30;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_UNKNOWN_MCAST_DST_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_UNKNOWN_UCAST_DST_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_UNKNOWN_UCAST_DST_LBN: c_int = 31;
pub const MC_CMD_FILTER_OP_EXT_IN_MATCH_UNKNOWN_UCAST_DST_WIDTH: c_int = 1;
// receive destination
pub const MC_CMD_FILTER_OP_EXT_IN_RX_DEST_OFST: c_int = 20;
pub const MC_CMD_FILTER_OP_EXT_IN_RX_DEST_LEN: c_int = 4;
// enum: drop packets
pub const MC_CMD_FILTER_OP_EXT_IN_RX_DEST_DROP: c_uint = 0x0;
// enum: receive to host
pub const MC_CMD_FILTER_OP_EXT_IN_RX_DEST_HOST: c_uint = 0x1;
// enum: receive to MC
pub const MC_CMD_FILTER_OP_EXT_IN_RX_DEST_MC: c_uint = 0x2;
// enum: loop back to TXDP 0
pub const MC_CMD_FILTER_OP_EXT_IN_RX_DEST_TX0: c_uint = 0x3;
// enum: loop back to TXDP 1
pub const MC_CMD_FILTER_OP_EXT_IN_RX_DEST_TX1: c_uint = 0x4;
// receive queue handle (for multiple queue modes, this is the base queue)
pub const MC_CMD_FILTER_OP_EXT_IN_RX_QUEUE_OFST: c_int = 24;
pub const MC_CMD_FILTER_OP_EXT_IN_RX_QUEUE_LEN: c_int = 4;
// receive mode
pub const MC_CMD_FILTER_OP_EXT_IN_RX_MODE_OFST: c_int = 28;
pub const MC_CMD_FILTER_OP_EXT_IN_RX_MODE_LEN: c_int = 4;
// enum: receive to just the specified queue
pub const MC_CMD_FILTER_OP_EXT_IN_RX_MODE_SIMPLE: c_uint = 0x0;
// enum: receive to multiple queues using RSS context
pub const MC_CMD_FILTER_OP_EXT_IN_RX_MODE_RSS: c_uint = 0x1;
// enum: receive to multiple queues using .1p mapping
pub const MC_CMD_FILTER_OP_EXT_IN_RX_MODE_DOT1P_MAPPING: c_uint = 0x2;
// enum: install a filter entry that will never match; for test purposes only
//
pub const MC_CMD_FILTER_OP_EXT_IN_RX_MODE_TEST_NEVER_MATCH: c_uint = 0x80000000;
// RSS context (for RX_MODE_RSS) or .1p mapping handle (for
// RX_MODE_DOT1P_MAPPING), as returned by MC_CMD_RSS_CONTEXT_ALLOC or
// MC_CMD_DOT1P_MAPPING_ALLOC.
//
pub const MC_CMD_FILTER_OP_EXT_IN_RX_CONTEXT_OFST: c_int = 32;
pub const MC_CMD_FILTER_OP_EXT_IN_RX_CONTEXT_LEN: c_int = 4;
// transmit domain (reserved; set to 0)
pub const MC_CMD_FILTER_OP_EXT_IN_TX_DOMAIN_OFST: c_int = 36;
pub const MC_CMD_FILTER_OP_EXT_IN_TX_DOMAIN_LEN: c_int = 4;
// transmit destination (either set the MAC and/or PM bits for explicit
// control, or set this field to TX_DEST_DEFAULT for sensible default
// behaviour)
//
pub const MC_CMD_FILTER_OP_EXT_IN_TX_DEST_OFST: c_int = 40;
pub const MC_CMD_FILTER_OP_EXT_IN_TX_DEST_LEN: c_int = 4;
// enum: request default behaviour (based on filter type)
pub const MC_CMD_FILTER_OP_EXT_IN_TX_DEST_DEFAULT: c_uint = 0xffffffff;
pub const MC_CMD_FILTER_OP_EXT_IN_TX_DEST_MAC_OFST: c_int = 40;
pub const MC_CMD_FILTER_OP_EXT_IN_TX_DEST_MAC_LBN: c_int = 0;
pub const MC_CMD_FILTER_OP_EXT_IN_TX_DEST_MAC_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_EXT_IN_TX_DEST_PM_OFST: c_int = 40;
pub const MC_CMD_FILTER_OP_EXT_IN_TX_DEST_PM_LBN: c_int = 1;
pub const MC_CMD_FILTER_OP_EXT_IN_TX_DEST_PM_WIDTH: c_int = 1;
// source MAC address to match (as bytes in network order)
pub const MC_CMD_FILTER_OP_EXT_IN_SRC_MAC_OFST: c_int = 44;
pub const MC_CMD_FILTER_OP_EXT_IN_SRC_MAC_LEN: c_int = 6;
// source port to match (as bytes in network order)
pub const MC_CMD_FILTER_OP_EXT_IN_SRC_PORT_OFST: c_int = 50;
pub const MC_CMD_FILTER_OP_EXT_IN_SRC_PORT_LEN: c_int = 2;
// destination MAC address to match (as bytes in network order)
pub const MC_CMD_FILTER_OP_EXT_IN_DST_MAC_OFST: c_int = 52;
pub const MC_CMD_FILTER_OP_EXT_IN_DST_MAC_LEN: c_int = 6;
// destination port to match (as bytes in network order)
pub const MC_CMD_FILTER_OP_EXT_IN_DST_PORT_OFST: c_int = 58;
pub const MC_CMD_FILTER_OP_EXT_IN_DST_PORT_LEN: c_int = 2;
// Ethernet type to match (as bytes in network order)
pub const MC_CMD_FILTER_OP_EXT_IN_ETHER_TYPE_OFST: c_int = 60;
pub const MC_CMD_FILTER_OP_EXT_IN_ETHER_TYPE_LEN: c_int = 2;
// Inner VLAN tag to match (as bytes in network order)
pub const MC_CMD_FILTER_OP_EXT_IN_INNER_VLAN_OFST: c_int = 62;
pub const MC_CMD_FILTER_OP_EXT_IN_INNER_VLAN_LEN: c_int = 2;
// Outer VLAN tag to match (as bytes in network order)
pub const MC_CMD_FILTER_OP_EXT_IN_OUTER_VLAN_OFST: c_int = 64;
pub const MC_CMD_FILTER_OP_EXT_IN_OUTER_VLAN_LEN: c_int = 2;
// IP protocol to match (in low byte; set high byte to 0)
pub const MC_CMD_FILTER_OP_EXT_IN_IP_PROTO_OFST: c_int = 66;
pub const MC_CMD_FILTER_OP_EXT_IN_IP_PROTO_LEN: c_int = 2;
// Firmware defined register 0 to match (reserved; set to 0)
pub const MC_CMD_FILTER_OP_EXT_IN_FWDEF0_OFST: c_int = 68;
pub const MC_CMD_FILTER_OP_EXT_IN_FWDEF0_LEN: c_int = 4;
// VNI (for VXLAN/Geneve, when IP protocol is UDP) or VSID (for NVGRE, when IP
// protocol is GRE) to match (as bytes in network order; set last byte to 0 for
// VXLAN/NVGRE, or 1 for Geneve)
//
pub const MC_CMD_FILTER_OP_EXT_IN_VNI_OR_VSID_OFST: c_int = 72;
pub const MC_CMD_FILTER_OP_EXT_IN_VNI_OR_VSID_LEN: c_int = 4;
pub const MC_CMD_FILTER_OP_EXT_IN_VNI_VALUE_OFST: c_int = 72;
pub const MC_CMD_FILTER_OP_EXT_IN_VNI_VALUE_LBN: c_int = 0;
pub const MC_CMD_FILTER_OP_EXT_IN_VNI_VALUE_WIDTH: c_int = 24;
pub const MC_CMD_FILTER_OP_EXT_IN_VNI_TYPE_OFST: c_int = 72;
pub const MC_CMD_FILTER_OP_EXT_IN_VNI_TYPE_LBN: c_int = 24;
pub const MC_CMD_FILTER_OP_EXT_IN_VNI_TYPE_WIDTH: c_int = 8;
// enum: Match VXLAN traffic with this VNI
pub const MC_CMD_FILTER_OP_EXT_IN_VNI_TYPE_VXLAN: c_uint = 0x0;
// enum: Match Geneve traffic with this VNI
pub const MC_CMD_FILTER_OP_EXT_IN_VNI_TYPE_GENEVE: c_uint = 0x1;
// enum: Reserved for experimental development use
pub const MC_CMD_FILTER_OP_EXT_IN_VNI_TYPE_EXPERIMENTAL: c_uint = 0xfe;
pub const MC_CMD_FILTER_OP_EXT_IN_VSID_VALUE_OFST: c_int = 72;
pub const MC_CMD_FILTER_OP_EXT_IN_VSID_VALUE_LBN: c_int = 0;
pub const MC_CMD_FILTER_OP_EXT_IN_VSID_VALUE_WIDTH: c_int = 24;
pub const MC_CMD_FILTER_OP_EXT_IN_VSID_TYPE_OFST: c_int = 72;
pub const MC_CMD_FILTER_OP_EXT_IN_VSID_TYPE_LBN: c_int = 24;
pub const MC_CMD_FILTER_OP_EXT_IN_VSID_TYPE_WIDTH: c_int = 8;
// enum: Match NVGRE traffic with this VSID
pub const MC_CMD_FILTER_OP_EXT_IN_VSID_TYPE_NVGRE: c_uint = 0x0;
// source IP address to match (as bytes in network order; set last 12 bytes to
// 0 for IPv4 address)
//
pub const MC_CMD_FILTER_OP_EXT_IN_SRC_IP_OFST: c_int = 76;
pub const MC_CMD_FILTER_OP_EXT_IN_SRC_IP_LEN: c_int = 16;
// destination IP address to match (as bytes in network order; set last 12
// bytes to 0 for IPv4 address)
//
pub const MC_CMD_FILTER_OP_EXT_IN_DST_IP_OFST: c_int = 92;
pub const MC_CMD_FILTER_OP_EXT_IN_DST_IP_LEN: c_int = 16;
// VXLAN/NVGRE inner frame source MAC address to match (as bytes in network
// order)
//
pub const MC_CMD_FILTER_OP_EXT_IN_IFRM_SRC_MAC_OFST: c_int = 108;
pub const MC_CMD_FILTER_OP_EXT_IN_IFRM_SRC_MAC_LEN: c_int = 6;
// VXLAN/NVGRE inner frame source port to match (as bytes in network order)
pub const MC_CMD_FILTER_OP_EXT_IN_IFRM_SRC_PORT_OFST: c_int = 114;
pub const MC_CMD_FILTER_OP_EXT_IN_IFRM_SRC_PORT_LEN: c_int = 2;
// VXLAN/NVGRE inner frame destination MAC address to match (as bytes in
// network order)
//
pub const MC_CMD_FILTER_OP_EXT_IN_IFRM_DST_MAC_OFST: c_int = 116;
pub const MC_CMD_FILTER_OP_EXT_IN_IFRM_DST_MAC_LEN: c_int = 6;
// VXLAN/NVGRE inner frame destination port to match (as bytes in network
// order)
//
pub const MC_CMD_FILTER_OP_EXT_IN_IFRM_DST_PORT_OFST: c_int = 122;
pub const MC_CMD_FILTER_OP_EXT_IN_IFRM_DST_PORT_LEN: c_int = 2;
// VXLAN/NVGRE inner frame Ethernet type to match (as bytes in network order)
//
pub const MC_CMD_FILTER_OP_EXT_IN_IFRM_ETHER_TYPE_OFST: c_int = 124;
pub const MC_CMD_FILTER_OP_EXT_IN_IFRM_ETHER_TYPE_LEN: c_int = 2;
// VXLAN/NVGRE inner frame Inner VLAN tag to match (as bytes in network order)
//
pub const MC_CMD_FILTER_OP_EXT_IN_IFRM_INNER_VLAN_OFST: c_int = 126;
pub const MC_CMD_FILTER_OP_EXT_IN_IFRM_INNER_VLAN_LEN: c_int = 2;
// VXLAN/NVGRE inner frame Outer VLAN tag to match (as bytes in network order)
//
pub const MC_CMD_FILTER_OP_EXT_IN_IFRM_OUTER_VLAN_OFST: c_int = 128;
pub const MC_CMD_FILTER_OP_EXT_IN_IFRM_OUTER_VLAN_LEN: c_int = 2;
// VXLAN/NVGRE inner frame IP protocol to match (in low byte; set high byte to
// 0)
//
pub const MC_CMD_FILTER_OP_EXT_IN_IFRM_IP_PROTO_OFST: c_int = 130;
pub const MC_CMD_FILTER_OP_EXT_IN_IFRM_IP_PROTO_LEN: c_int = 2;
// VXLAN/NVGRE inner frame Firmware defined register 0 to match (reserved; set
// to 0)
//
pub const MC_CMD_FILTER_OP_EXT_IN_IFRM_FWDEF0_OFST: c_int = 132;
pub const MC_CMD_FILTER_OP_EXT_IN_IFRM_FWDEF0_LEN: c_int = 4;
// VXLAN/NVGRE inner frame Firmware defined register 1 to match (reserved; set
// to 0)
//
pub const MC_CMD_FILTER_OP_EXT_IN_IFRM_FWDEF1_OFST: c_int = 136;
pub const MC_CMD_FILTER_OP_EXT_IN_IFRM_FWDEF1_LEN: c_int = 4;
// VXLAN/NVGRE inner frame source IP address to match (as bytes in network
// order; set last 12 bytes to 0 for IPv4 address)
//
pub const MC_CMD_FILTER_OP_EXT_IN_IFRM_SRC_IP_OFST: c_int = 140;
pub const MC_CMD_FILTER_OP_EXT_IN_IFRM_SRC_IP_LEN: c_int = 16;
// VXLAN/NVGRE inner frame destination IP address to match (as bytes in network
// order; set last 12 bytes to 0 for IPv4 address)
//
pub const MC_CMD_FILTER_OP_EXT_IN_IFRM_DST_IP_OFST: c_int = 156;
pub const MC_CMD_FILTER_OP_EXT_IN_IFRM_DST_IP_LEN: c_int = 16;
// MC_CMD_FILTER_OP_V3_IN msgrequest: FILTER_OP extension to support additional
// filter actions for Intel's DPDK (Data Plane Development Kit, dpdk.org) via
// its rte_flow API. This extension is only useful with the sfc_efx driver
// included as part of DPDK, used in conjunction with the dpdk datapath
// firmware variant.
//
pub const MC_CMD_FILTER_OP_V3_IN_LEN: c_int = 180;
// identifies the type of operation requested
pub const MC_CMD_FILTER_OP_V3_IN_OP_OFST: c_int = 0;
pub const MC_CMD_FILTER_OP_V3_IN_OP_LEN: c_int = 4;
// Enum values, see field(s):
// MC_CMD_FILTER_OP_IN/OP
// filter handle (for remove / unsubscribe operations)
pub const MC_CMD_FILTER_OP_V3_IN_HANDLE_OFST: c_int = 4;
pub const MC_CMD_FILTER_OP_V3_IN_HANDLE_LEN: c_int = 8;
pub const MC_CMD_FILTER_OP_V3_IN_HANDLE_LO_OFST: c_int = 4;
pub const MC_CMD_FILTER_OP_V3_IN_HANDLE_HI_OFST: c_int = 8;
// The port ID associated with the v-adaptor which should contain this filter.
//
pub const MC_CMD_FILTER_OP_V3_IN_PORT_ID_OFST: c_int = 12;
pub const MC_CMD_FILTER_OP_V3_IN_PORT_ID_LEN: c_int = 4;
// fields to include in match criteria
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_FIELDS_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_FIELDS_LEN: c_int = 4;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_SRC_IP_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_SRC_IP_LBN: c_int = 0;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_SRC_IP_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_DST_IP_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_DST_IP_LBN: c_int = 1;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_DST_IP_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_SRC_MAC_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_SRC_MAC_LBN: c_int = 2;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_SRC_MAC_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_SRC_PORT_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_SRC_PORT_LBN: c_int = 3;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_SRC_PORT_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_DST_MAC_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_DST_MAC_LBN: c_int = 4;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_DST_MAC_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_DST_PORT_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_DST_PORT_LBN: c_int = 5;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_DST_PORT_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_ETHER_TYPE_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_ETHER_TYPE_LBN: c_int = 6;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_ETHER_TYPE_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_INNER_VLAN_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_INNER_VLAN_LBN: c_int = 7;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_INNER_VLAN_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_OUTER_VLAN_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_OUTER_VLAN_LBN: c_int = 8;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_OUTER_VLAN_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IP_PROTO_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IP_PROTO_LBN: c_int = 9;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IP_PROTO_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_FWDEF0_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_FWDEF0_LBN: c_int = 10;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_FWDEF0_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_VNI_OR_VSID_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_VNI_OR_VSID_LBN: c_int = 11;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_VNI_OR_VSID_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_SRC_IP_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_SRC_IP_LBN: c_int = 12;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_SRC_IP_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_DST_IP_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_DST_IP_LBN: c_int = 13;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_DST_IP_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_SRC_MAC_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_SRC_MAC_LBN: c_int = 14;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_SRC_MAC_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_SRC_PORT_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_SRC_PORT_LBN: c_int = 15;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_SRC_PORT_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_DST_MAC_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_DST_MAC_LBN: c_int = 16;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_DST_MAC_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_DST_PORT_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_DST_PORT_LBN: c_int = 17;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_DST_PORT_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_ETHER_TYPE_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_ETHER_TYPE_LBN: c_int = 18;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_ETHER_TYPE_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_INNER_VLAN_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_INNER_VLAN_LBN: c_int = 19;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_INNER_VLAN_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_OUTER_VLAN_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_OUTER_VLAN_LBN: c_int = 20;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_OUTER_VLAN_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_IP_PROTO_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_IP_PROTO_LBN: c_int = 21;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_IP_PROTO_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_FWDEF0_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_FWDEF0_LBN: c_int = 22;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_FWDEF0_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_FWDEF1_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_FWDEF1_LBN: c_int = 23;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_FWDEF1_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_UNKNOWN_MCAST_DST_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_UNKNOWN_MCAST_DST_LBN: c_int = 24;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_UNKNOWN_MCAST_DST_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_UNKNOWN_UCAST_DST_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_UNKNOWN_UCAST_DST_LBN: c_int = 25;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_IFRM_UNKNOWN_UCAST_DST_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_UNKNOWN_MCAST_DST_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_UNKNOWN_MCAST_DST_LBN: c_int = 30;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_UNKNOWN_MCAST_DST_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_UNKNOWN_UCAST_DST_OFST: c_int = 16;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_UNKNOWN_UCAST_DST_LBN: c_int = 31;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_UNKNOWN_UCAST_DST_WIDTH: c_int = 1;
// receive destination
pub const MC_CMD_FILTER_OP_V3_IN_RX_DEST_OFST: c_int = 20;
pub const MC_CMD_FILTER_OP_V3_IN_RX_DEST_LEN: c_int = 4;
// enum: drop packets
pub const MC_CMD_FILTER_OP_V3_IN_RX_DEST_DROP: c_uint = 0x0;
// enum: receive to host
pub const MC_CMD_FILTER_OP_V3_IN_RX_DEST_HOST: c_uint = 0x1;
// enum: receive to MC
pub const MC_CMD_FILTER_OP_V3_IN_RX_DEST_MC: c_uint = 0x2;
// enum: loop back to TXDP 0
pub const MC_CMD_FILTER_OP_V3_IN_RX_DEST_TX0: c_uint = 0x3;
// enum: loop back to TXDP 1
pub const MC_CMD_FILTER_OP_V3_IN_RX_DEST_TX1: c_uint = 0x4;
// receive queue handle (for multiple queue modes, this is the base queue)
pub const MC_CMD_FILTER_OP_V3_IN_RX_QUEUE_OFST: c_int = 24;
pub const MC_CMD_FILTER_OP_V3_IN_RX_QUEUE_LEN: c_int = 4;
// receive mode
pub const MC_CMD_FILTER_OP_V3_IN_RX_MODE_OFST: c_int = 28;
pub const MC_CMD_FILTER_OP_V3_IN_RX_MODE_LEN: c_int = 4;
// enum: receive to just the specified queue
pub const MC_CMD_FILTER_OP_V3_IN_RX_MODE_SIMPLE: c_uint = 0x0;
// enum: receive to multiple queues using RSS context
pub const MC_CMD_FILTER_OP_V3_IN_RX_MODE_RSS: c_uint = 0x1;
// enum: receive to multiple queues using .1p mapping
pub const MC_CMD_FILTER_OP_V3_IN_RX_MODE_DOT1P_MAPPING: c_uint = 0x2;
// enum: install a filter entry that will never match; for test purposes only
//
pub const MC_CMD_FILTER_OP_V3_IN_RX_MODE_TEST_NEVER_MATCH: c_uint = 0x80000000;
// RSS context (for RX_MODE_RSS) or .1p mapping handle (for
// RX_MODE_DOT1P_MAPPING), as returned by MC_CMD_RSS_CONTEXT_ALLOC or
// MC_CMD_DOT1P_MAPPING_ALLOC.
//
pub const MC_CMD_FILTER_OP_V3_IN_RX_CONTEXT_OFST: c_int = 32;
pub const MC_CMD_FILTER_OP_V3_IN_RX_CONTEXT_LEN: c_int = 4;
// transmit domain (reserved; set to 0)
pub const MC_CMD_FILTER_OP_V3_IN_TX_DOMAIN_OFST: c_int = 36;
pub const MC_CMD_FILTER_OP_V3_IN_TX_DOMAIN_LEN: c_int = 4;
// transmit destination (either set the MAC and/or PM bits for explicit
// control, or set this field to TX_DEST_DEFAULT for sensible default
// behaviour)
//
pub const MC_CMD_FILTER_OP_V3_IN_TX_DEST_OFST: c_int = 40;
pub const MC_CMD_FILTER_OP_V3_IN_TX_DEST_LEN: c_int = 4;
// enum: request default behaviour (based on filter type)
pub const MC_CMD_FILTER_OP_V3_IN_TX_DEST_DEFAULT: c_uint = 0xffffffff;
pub const MC_CMD_FILTER_OP_V3_IN_TX_DEST_MAC_OFST: c_int = 40;
pub const MC_CMD_FILTER_OP_V3_IN_TX_DEST_MAC_LBN: c_int = 0;
pub const MC_CMD_FILTER_OP_V3_IN_TX_DEST_MAC_WIDTH: c_int = 1;
pub const MC_CMD_FILTER_OP_V3_IN_TX_DEST_PM_OFST: c_int = 40;
pub const MC_CMD_FILTER_OP_V3_IN_TX_DEST_PM_LBN: c_int = 1;
pub const MC_CMD_FILTER_OP_V3_IN_TX_DEST_PM_WIDTH: c_int = 1;
// source MAC address to match (as bytes in network order)
pub const MC_CMD_FILTER_OP_V3_IN_SRC_MAC_OFST: c_int = 44;
pub const MC_CMD_FILTER_OP_V3_IN_SRC_MAC_LEN: c_int = 6;
// source port to match (as bytes in network order)
pub const MC_CMD_FILTER_OP_V3_IN_SRC_PORT_OFST: c_int = 50;
pub const MC_CMD_FILTER_OP_V3_IN_SRC_PORT_LEN: c_int = 2;
// destination MAC address to match (as bytes in network order)
pub const MC_CMD_FILTER_OP_V3_IN_DST_MAC_OFST: c_int = 52;
pub const MC_CMD_FILTER_OP_V3_IN_DST_MAC_LEN: c_int = 6;
// destination port to match (as bytes in network order)
pub const MC_CMD_FILTER_OP_V3_IN_DST_PORT_OFST: c_int = 58;
pub const MC_CMD_FILTER_OP_V3_IN_DST_PORT_LEN: c_int = 2;
// Ethernet type to match (as bytes in network order)
pub const MC_CMD_FILTER_OP_V3_IN_ETHER_TYPE_OFST: c_int = 60;
pub const MC_CMD_FILTER_OP_V3_IN_ETHER_TYPE_LEN: c_int = 2;
// Inner VLAN tag to match (as bytes in network order)
pub const MC_CMD_FILTER_OP_V3_IN_INNER_VLAN_OFST: c_int = 62;
pub const MC_CMD_FILTER_OP_V3_IN_INNER_VLAN_LEN: c_int = 2;
// Outer VLAN tag to match (as bytes in network order)
pub const MC_CMD_FILTER_OP_V3_IN_OUTER_VLAN_OFST: c_int = 64;
pub const MC_CMD_FILTER_OP_V3_IN_OUTER_VLAN_LEN: c_int = 2;
// IP protocol to match (in low byte; set high byte to 0)
pub const MC_CMD_FILTER_OP_V3_IN_IP_PROTO_OFST: c_int = 66;
pub const MC_CMD_FILTER_OP_V3_IN_IP_PROTO_LEN: c_int = 2;
// Firmware defined register 0 to match (reserved; set to 0)
pub const MC_CMD_FILTER_OP_V3_IN_FWDEF0_OFST: c_int = 68;
pub const MC_CMD_FILTER_OP_V3_IN_FWDEF0_LEN: c_int = 4;
// VNI (for VXLAN/Geneve, when IP protocol is UDP) or VSID (for NVGRE, when IP
// protocol is GRE) to match (as bytes in network order; set last byte to 0 for
// VXLAN/NVGRE, or 1 for Geneve)
//
pub const MC_CMD_FILTER_OP_V3_IN_VNI_OR_VSID_OFST: c_int = 72;
pub const MC_CMD_FILTER_OP_V3_IN_VNI_OR_VSID_LEN: c_int = 4;
pub const MC_CMD_FILTER_OP_V3_IN_VNI_VALUE_OFST: c_int = 72;
pub const MC_CMD_FILTER_OP_V3_IN_VNI_VALUE_LBN: c_int = 0;
pub const MC_CMD_FILTER_OP_V3_IN_VNI_VALUE_WIDTH: c_int = 24;
pub const MC_CMD_FILTER_OP_V3_IN_VNI_TYPE_OFST: c_int = 72;
pub const MC_CMD_FILTER_OP_V3_IN_VNI_TYPE_LBN: c_int = 24;
pub const MC_CMD_FILTER_OP_V3_IN_VNI_TYPE_WIDTH: c_int = 8;
// enum: Match VXLAN traffic with this VNI
pub const MC_CMD_FILTER_OP_V3_IN_VNI_TYPE_VXLAN: c_uint = 0x0;
// enum: Match Geneve traffic with this VNI
pub const MC_CMD_FILTER_OP_V3_IN_VNI_TYPE_GENEVE: c_uint = 0x1;
// enum: Reserved for experimental development use
pub const MC_CMD_FILTER_OP_V3_IN_VNI_TYPE_EXPERIMENTAL: c_uint = 0xfe;
pub const MC_CMD_FILTER_OP_V3_IN_VSID_VALUE_OFST: c_int = 72;
pub const MC_CMD_FILTER_OP_V3_IN_VSID_VALUE_LBN: c_int = 0;
pub const MC_CMD_FILTER_OP_V3_IN_VSID_VALUE_WIDTH: c_int = 24;
pub const MC_CMD_FILTER_OP_V3_IN_VSID_TYPE_OFST: c_int = 72;
pub const MC_CMD_FILTER_OP_V3_IN_VSID_TYPE_LBN: c_int = 24;
pub const MC_CMD_FILTER_OP_V3_IN_VSID_TYPE_WIDTH: c_int = 8;
// enum: Match NVGRE traffic with this VSID
pub const MC_CMD_FILTER_OP_V3_IN_VSID_TYPE_NVGRE: c_uint = 0x0;
// source IP address to match (as bytes in network order; set last 12 bytes to
// 0 for IPv4 address)
//
pub const MC_CMD_FILTER_OP_V3_IN_SRC_IP_OFST: c_int = 76;
pub const MC_CMD_FILTER_OP_V3_IN_SRC_IP_LEN: c_int = 16;
// destination IP address to match (as bytes in network order; set last 12
// bytes to 0 for IPv4 address)
//
pub const MC_CMD_FILTER_OP_V3_IN_DST_IP_OFST: c_int = 92;
pub const MC_CMD_FILTER_OP_V3_IN_DST_IP_LEN: c_int = 16;
// VXLAN/NVGRE inner frame source MAC address to match (as bytes in network
// order)
//
pub const MC_CMD_FILTER_OP_V3_IN_IFRM_SRC_MAC_OFST: c_int = 108;
pub const MC_CMD_FILTER_OP_V3_IN_IFRM_SRC_MAC_LEN: c_int = 6;
// VXLAN/NVGRE inner frame source port to match (as bytes in network order)
pub const MC_CMD_FILTER_OP_V3_IN_IFRM_SRC_PORT_OFST: c_int = 114;
pub const MC_CMD_FILTER_OP_V3_IN_IFRM_SRC_PORT_LEN: c_int = 2;
// VXLAN/NVGRE inner frame destination MAC address to match (as bytes in
// network order)
//
pub const MC_CMD_FILTER_OP_V3_IN_IFRM_DST_MAC_OFST: c_int = 116;
pub const MC_CMD_FILTER_OP_V3_IN_IFRM_DST_MAC_LEN: c_int = 6;
// VXLAN/NVGRE inner frame destination port to match (as bytes in network
// order)
//
pub const MC_CMD_FILTER_OP_V3_IN_IFRM_DST_PORT_OFST: c_int = 122;
pub const MC_CMD_FILTER_OP_V3_IN_IFRM_DST_PORT_LEN: c_int = 2;
// VXLAN/NVGRE inner frame Ethernet type to match (as bytes in network order)
//
pub const MC_CMD_FILTER_OP_V3_IN_IFRM_ETHER_TYPE_OFST: c_int = 124;
pub const MC_CMD_FILTER_OP_V3_IN_IFRM_ETHER_TYPE_LEN: c_int = 2;
// VXLAN/NVGRE inner frame Inner VLAN tag to match (as bytes in network order)
//
pub const MC_CMD_FILTER_OP_V3_IN_IFRM_INNER_VLAN_OFST: c_int = 126;
pub const MC_CMD_FILTER_OP_V3_IN_IFRM_INNER_VLAN_LEN: c_int = 2;
// VXLAN/NVGRE inner frame Outer VLAN tag to match (as bytes in network order)
//
pub const MC_CMD_FILTER_OP_V3_IN_IFRM_OUTER_VLAN_OFST: c_int = 128;
pub const MC_CMD_FILTER_OP_V3_IN_IFRM_OUTER_VLAN_LEN: c_int = 2;
// VXLAN/NVGRE inner frame IP protocol to match (in low byte; set high byte to
// 0)
//
pub const MC_CMD_FILTER_OP_V3_IN_IFRM_IP_PROTO_OFST: c_int = 130;
pub const MC_CMD_FILTER_OP_V3_IN_IFRM_IP_PROTO_LEN: c_int = 2;
// VXLAN/NVGRE inner frame Firmware defined register 0 to match (reserved; set
// to 0)
//
pub const MC_CMD_FILTER_OP_V3_IN_IFRM_FWDEF0_OFST: c_int = 132;
pub const MC_CMD_FILTER_OP_V3_IN_IFRM_FWDEF0_LEN: c_int = 4;
// VXLAN/NVGRE inner frame Firmware defined register 1 to match (reserved; set
// to 0)
//
pub const MC_CMD_FILTER_OP_V3_IN_IFRM_FWDEF1_OFST: c_int = 136;
pub const MC_CMD_FILTER_OP_V3_IN_IFRM_FWDEF1_LEN: c_int = 4;
// VXLAN/NVGRE inner frame source IP address to match (as bytes in network
// order; set last 12 bytes to 0 for IPv4 address)
//
pub const MC_CMD_FILTER_OP_V3_IN_IFRM_SRC_IP_OFST: c_int = 140;
pub const MC_CMD_FILTER_OP_V3_IN_IFRM_SRC_IP_LEN: c_int = 16;
// VXLAN/NVGRE inner frame destination IP address to match (as bytes in network
// order; set last 12 bytes to 0 for IPv4 address)
//
pub const MC_CMD_FILTER_OP_V3_IN_IFRM_DST_IP_OFST: c_int = 156;
pub const MC_CMD_FILTER_OP_V3_IN_IFRM_DST_IP_LEN: c_int = 16;
// Set an action for all packets matching this filter. The DPDK driver and dpdk
// f/w variant use their own specific delivery structures, which are documented
// in the DPDK Firmware Driver Interface (SF-119419-TC). Requesting anything
// other than MATCH_ACTION_NONE when the NIC is running another f/w variant
// will cause the filter insertion to fail with ENOTSUP.
//
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_ACTION_OFST: c_int = 172;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_ACTION_LEN: c_int = 4;
// enum: do nothing extra
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_ACTION_NONE: c_uint = 0x0;
// enum: Set the match flag in the packet prefix for packets matching the
// filter (only with dpdk firmware, otherwise fails with ENOTSUP). Used to
// support the DPDK rte_flow "FLAG" action.
//
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_ACTION_FLAG: c_uint = 0x1;
// enum: Insert MATCH_MARK_VALUE into the packet prefix for packets matching
// the filter (only with dpdk firmware, otherwise fails with ENOTSUP). Used to
// support the DPDK rte_flow "MARK" action.
//
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_ACTION_MARK: c_uint = 0x2;
// the mark value for MATCH_ACTION_MARK. Requesting a value larger than the
// maximum (obtained from MC_CMD_GET_CAPABILITIES_V5/FILTER_ACTION_MARK_MAX)
// will cause the filter insertion to fail with EINVAL.
//
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_MARK_VALUE_OFST: c_int = 176;
pub const MC_CMD_FILTER_OP_V3_IN_MATCH_MARK_VALUE_LEN: c_int = 4;
// MC_CMD_FILTER_OP_OUT msgresponse
pub const MC_CMD_FILTER_OP_OUT_LEN: c_int = 12;
// identifies the type of operation requested
pub const MC_CMD_FILTER_OP_OUT_OP_OFST: c_int = 0;
pub const MC_CMD_FILTER_OP_OUT_OP_LEN: c_int = 4;
// Enum values, see field(s):
// MC_CMD_FILTER_OP_IN/OP
// Returned filter handle (for insert / subscribe operations). Note that these
// handles should be considered opaque to the host, although a value of
// 0xFFFFFFFF_FFFFFFFF is guaranteed never to be a valid handle.
//
pub const MC_CMD_FILTER_OP_OUT_HANDLE_OFST: c_int = 4;
pub const MC_CMD_FILTER_OP_OUT_HANDLE_LEN: c_int = 8;
pub const MC_CMD_FILTER_OP_OUT_HANDLE_LO_OFST: c_int = 4;
pub const MC_CMD_FILTER_OP_OUT_HANDLE_HI_OFST: c_int = 8;
// enum: guaranteed invalid filter handle (low 32 bits)
pub const MC_CMD_FILTER_OP_OUT_HANDLE_LO_INVALID: c_uint = 0xffffffff;
// enum: guaranteed invalid filter handle (high 32 bits)
pub const MC_CMD_FILTER_OP_OUT_HANDLE_HI_INVALID: c_uint = 0xffffffff;
// MC_CMD_FILTER_OP_EXT_OUT msgresponse
pub const MC_CMD_FILTER_OP_EXT_OUT_LEN: c_int = 12;
// identifies the type of operation requested
pub const MC_CMD_FILTER_OP_EXT_OUT_OP_OFST: c_int = 0;
pub const MC_CMD_FILTER_OP_EXT_OUT_OP_LEN: c_int = 4;
// Enum values, see field(s):
// MC_CMD_FILTER_OP_EXT_IN/OP
// Returned filter handle (for insert / subscribe operations). Note that these
// handles should be considered opaque to the host, although a value of
// 0xFFFFFFFF_FFFFFFFF is guaranteed never to be a valid handle.
//
pub const MC_CMD_FILTER_OP_EXT_OUT_HANDLE_OFST: c_int = 4;
pub const MC_CMD_FILTER_OP_EXT_OUT_HANDLE_LEN: c_int = 8;
pub const MC_CMD_FILTER_OP_EXT_OUT_HANDLE_LO_OFST: c_int = 4;
pub const MC_CMD_FILTER_OP_EXT_OUT_HANDLE_HI_OFST: c_int = 8;
// Enum values, see field(s):
// MC_CMD_FILTER_OP_OUT/HANDLE
//
// MC_CMD_GET_PARSER_DISP_INFO
// Get information related to the parser-dispatcher subsystem
//
pub const MC_CMD_GET_PARSER_DISP_INFO: c_uint = 0xe4;

// MC_CMD_GET_PARSER_DISP_INFO_IN msgrequest
pub const MC_CMD_GET_PARSER_DISP_INFO_IN_LEN: c_int = 4;
// identifies the type of operation requested
pub const MC_CMD_GET_PARSER_DISP_INFO_IN_OP_OFST: c_int = 0;
pub const MC_CMD_GET_PARSER_DISP_INFO_IN_OP_LEN: c_int = 4;
// enum: read the list of supported RX filter matches
pub const MC_CMD_GET_PARSER_DISP_INFO_IN_OP_GET_SUPPORTED_RX_MATCHES: c_uint = 0x1;
// enum: read flags indicating restrictions on filter insertion for the calling
// client
//
pub const MC_CMD_GET_PARSER_DISP_INFO_IN_OP_GET_RESTRICTIONS: c_uint = 0x2;
// enum: read properties relating to security rules (Medford-only; for use by
// SolarSecure apps, not directly by drivers. See SF-114946-SW.)
//
pub const MC_CMD_GET_PARSER_DISP_INFO_IN_OP_GET_SECURITY_RULE_INFO: c_uint = 0x3;
// enum: read the list of supported RX filter matches for VXLAN/NVGRE
// encapsulated frames, which follow a different match sequence to normal
// frames (Medford only)
//
pub const MC_CMD_GET_PARSER_DISP_INFO_IN_OP_GET_SUPPORTED_ENCAP_RX_MATCHES: c_uint = 0x4;
// enum: read the list of supported matches for the encapsulation detection
// rules inserted by MC_CMD_VNIC_ENCAP_RULE_ADD. (ef100 and later)
//
pub const MC_CMD_GET_PARSER_DISP_INFO_IN_OP_GET_SUPPORTED_VNIC_ENCAP_MATCHES: c_uint = 0x5;
// MC_CMD_GET_PARSER_DISP_INFO_OUT msgresponse
pub const MC_CMD_GET_PARSER_DISP_INFO_OUT_LENMIN: c_int = 8;
pub const MC_CMD_GET_PARSER_DISP_INFO_OUT_LENMAX: c_int = 252;
pub const MC_CMD_GET_PARSER_DISP_INFO_OUT_LENMAX_MCDI2: c_int = 1020;

// identifies the type of operation requested
pub const MC_CMD_GET_PARSER_DISP_INFO_OUT_OP_OFST: c_int = 0;
pub const MC_CMD_GET_PARSER_DISP_INFO_OUT_OP_LEN: c_int = 4;
// Enum values, see field(s):
// MC_CMD_GET_PARSER_DISP_INFO_IN/OP
// number of supported match types
pub const MC_CMD_GET_PARSER_DISP_INFO_OUT_NUM_SUPPORTED_MATCHES_OFST: c_int = 4;
pub const MC_CMD_GET_PARSER_DISP_INFO_OUT_NUM_SUPPORTED_MATCHES_LEN: c_int = 4;
// array of supported match types (valid MATCH_FIELDS values for
// MC_CMD_FILTER_OP) sorted in decreasing priority order
//
pub const MC_CMD_GET_PARSER_DISP_INFO_OUT_SUPPORTED_MATCHES_OFST: c_int = 8;
pub const MC_CMD_GET_PARSER_DISP_INFO_OUT_SUPPORTED_MATCHES_LEN: c_int = 4;
pub const MC_CMD_GET_PARSER_DISP_INFO_OUT_SUPPORTED_MATCHES_MINNUM: c_int = 0;
pub const MC_CMD_GET_PARSER_DISP_INFO_OUT_SUPPORTED_MATCHES_MAXNUM: c_int = 61;
pub const MC_CMD_GET_PARSER_DISP_INFO_OUT_SUPPORTED_MATCHES_MAXNUM_MCDI2: c_int = 253;
// MC_CMD_GET_PARSER_DISP_RESTRICTIONS_OUT msgresponse
pub const MC_CMD_GET_PARSER_DISP_RESTRICTIONS_OUT_LEN: c_int = 8;
// identifies the type of operation requested
pub const MC_CMD_GET_PARSER_DISP_RESTRICTIONS_OUT_OP_OFST: c_int = 0;
pub const MC_CMD_GET_PARSER_DISP_RESTRICTIONS_OUT_OP_LEN: c_int = 4;
// Enum values, see field(s):
// MC_CMD_GET_PARSER_DISP_INFO_IN/OP
// bitfield of filter insertion restrictions
pub const MC_CMD_GET_PARSER_DISP_RESTRICTIONS_OUT_RESTRICTION_FLAGS_OFST: c_int = 4;
pub const MC_CMD_GET_PARSER_DISP_RESTRICTIONS_OUT_RESTRICTION_FLAGS_LEN: c_int = 4;
pub const MC_CMD_GET_PARSER_DISP_RESTRICTIONS_OUT_DST_IP_MCAST_ONLY_OFST: c_int = 4;
pub const MC_CMD_GET_PARSER_DISP_RESTRICTIONS_OUT_DST_IP_MCAST_ONLY_LBN: c_int = 0;
pub const MC_CMD_GET_PARSER_DISP_RESTRICTIONS_OUT_DST_IP_MCAST_ONLY_WIDTH: c_int = 1;
// MC_CMD_GET_PARSER_DISP_VNIC_ENCAP_MATCHES_OUT msgresponse: This response is
// returned if a MC_CMD_GET_PARSER_DISP_INFO_IN request is sent with OP value
// OP_GET_SUPPORTED_VNIC_ENCAP_MATCHES. It contains information about the
// supported match types that can be used in the encapsulation detection rules
// inserted by MC_CMD_VNIC_ENCAP_RULE_ADD.
//
pub const MC_CMD_GET_PARSER_DISP_VNIC_ENCAP_MATCHES_OUT_LENMIN: c_int = 8;
pub const MC_CMD_GET_PARSER_DISP_VNIC_ENCAP_MATCHES_OUT_LENMAX: c_int = 252;
pub const MC_CMD_GET_PARSER_DISP_VNIC_ENCAP_MATCHES_OUT_LENMAX_MCDI2: c_int = 1020;

// The op code OP_GET_SUPPORTED_VNIC_ENCAP_MATCHES is returned.
pub const MC_CMD_GET_PARSER_DISP_VNIC_ENCAP_MATCHES_OUT_OP_OFST: c_int = 0;
pub const MC_CMD_GET_PARSER_DISP_VNIC_ENCAP_MATCHES_OUT_OP_LEN: c_int = 4;
// Enum values, see field(s):
// MC_CMD_GET_PARSER_DISP_INFO_IN/OP
// number of supported match types
pub const MC_CMD_GET_PARSER_DISP_VNIC_ENCAP_MATCHES_OUT_NUM_SUPPORTED_MATCHES_OFST: c_int = 4;
pub const MC_CMD_GET_PARSER_DISP_VNIC_ENCAP_MATCHES_OUT_NUM_SUPPORTED_MATCHES_LEN: c_int = 4;
// array of supported match types (valid MATCH_FLAGS values for
// MC_CMD_VNIC_ENCAP_RULE_ADD) sorted in decreasing priority order
//
pub const MC_CMD_GET_PARSER_DISP_VNIC_ENCAP_MATCHES_OUT_SUPPORTED_MATCHES_OFST: c_int = 8;
pub const MC_CMD_GET_PARSER_DISP_VNIC_ENCAP_MATCHES_OUT_SUPPORTED_MATCHES_LEN: c_int = 4;
pub const MC_CMD_GET_PARSER_DISP_VNIC_ENCAP_MATCHES_OUT_SUPPORTED_MATCHES_MINNUM: c_int = 0;
pub const MC_CMD_GET_PARSER_DISP_VNIC_ENCAP_MATCHES_OUT_SUPPORTED_MATCHES_MAXNUM: c_int = 61;
pub const MC_CMD_GET_PARSER_DISP_VNIC_ENCAP_MATCHES_OUT_SUPPORTED_MATCHES_MAXNUM_MCDI2: c_int = 253;
//
// MC_CMD_GET_PORT_ASSIGNMENT
// Get port assignment for current PCI function.
//
pub const MC_CMD_GET_PORT_ASSIGNMENT: c_uint = 0xb8;

// MC_CMD_GET_PORT_ASSIGNMENT_IN msgrequest
pub const MC_CMD_GET_PORT_ASSIGNMENT_IN_LEN: c_int = 0;
// MC_CMD_GET_PORT_ASSIGNMENT_OUT msgresponse
pub const MC_CMD_GET_PORT_ASSIGNMENT_OUT_LEN: c_int = 4;
// Identifies the port assignment for this function.
pub const MC_CMD_GET_PORT_ASSIGNMENT_OUT_PORT_OFST: c_int = 0;
pub const MC_CMD_GET_PORT_ASSIGNMENT_OUT_PORT_LEN: c_int = 4;
//
// MC_CMD_SET_PORT_ASSIGNMENT
// Set port assignment for current PCI function.
//
pub const MC_CMD_SET_PORT_ASSIGNMENT: c_uint = 0xb9;

// MC_CMD_SET_PORT_ASSIGNMENT_IN msgrequest
pub const MC_CMD_SET_PORT_ASSIGNMENT_IN_LEN: c_int = 4;
// Identifies the port assignment for this function.
pub const MC_CMD_SET_PORT_ASSIGNMENT_IN_PORT_OFST: c_int = 0;
pub const MC_CMD_SET_PORT_ASSIGNMENT_IN_PORT_LEN: c_int = 4;
// MC_CMD_SET_PORT_ASSIGNMENT_OUT msgresponse
pub const MC_CMD_SET_PORT_ASSIGNMENT_OUT_LEN: c_int = 0;
//
// MC_CMD_ALLOC_VIS
// Allocate VIs for current PCI function.
//
pub const MC_CMD_ALLOC_VIS: c_uint = 0x8b;

// MC_CMD_ALLOC_VIS_IN msgrequest
pub const MC_CMD_ALLOC_VIS_IN_LEN: c_int = 8;
// The minimum number of VIs that is acceptable
pub const MC_CMD_ALLOC_VIS_IN_MIN_VI_COUNT_OFST: c_int = 0;
pub const MC_CMD_ALLOC_VIS_IN_MIN_VI_COUNT_LEN: c_int = 4;
// The maximum number of VIs that would be useful
pub const MC_CMD_ALLOC_VIS_IN_MAX_VI_COUNT_OFST: c_int = 4;
pub const MC_CMD_ALLOC_VIS_IN_MAX_VI_COUNT_LEN: c_int = 4;
// MC_CMD_ALLOC_VIS_OUT msgresponse: Huntington-compatible VI_ALLOC request.
// Use extended version in new code.
//
pub const MC_CMD_ALLOC_VIS_OUT_LEN: c_int = 8;
// The number of VIs allocated on this function
pub const MC_CMD_ALLOC_VIS_OUT_VI_COUNT_OFST: c_int = 0;
pub const MC_CMD_ALLOC_VIS_OUT_VI_COUNT_LEN: c_int = 4;
// The base absolute VI number allocated to this function. Required to
// correctly interpret wakeup events.
//
pub const MC_CMD_ALLOC_VIS_OUT_VI_BASE_OFST: c_int = 4;
pub const MC_CMD_ALLOC_VIS_OUT_VI_BASE_LEN: c_int = 4;
// MC_CMD_ALLOC_VIS_EXT_OUT msgresponse
pub const MC_CMD_ALLOC_VIS_EXT_OUT_LEN: c_int = 12;
// The number of VIs allocated on this function
pub const MC_CMD_ALLOC_VIS_EXT_OUT_VI_COUNT_OFST: c_int = 0;
pub const MC_CMD_ALLOC_VIS_EXT_OUT_VI_COUNT_LEN: c_int = 4;
// The base absolute VI number allocated to this function. Required to
// correctly interpret wakeup events.
//
pub const MC_CMD_ALLOC_VIS_EXT_OUT_VI_BASE_OFST: c_int = 4;
pub const MC_CMD_ALLOC_VIS_EXT_OUT_VI_BASE_LEN: c_int = 4;
// Function's port vi_shift value (always 0 on Huntington)
pub const MC_CMD_ALLOC_VIS_EXT_OUT_VI_SHIFT_OFST: c_int = 8;
pub const MC_CMD_ALLOC_VIS_EXT_OUT_VI_SHIFT_LEN: c_int = 4;
//
// MC_CMD_FREE_VIS
// Free VIs for current PCI function. Any linked PIO buffers will be unlinked,
// but not freed.
//
pub const MC_CMD_FREE_VIS: c_uint = 0x8c;

// MC_CMD_FREE_VIS_IN msgrequest
pub const MC_CMD_FREE_VIS_IN_LEN: c_int = 0;
// MC_CMD_FREE_VIS_OUT msgresponse
pub const MC_CMD_FREE_VIS_OUT_LEN: c_int = 0;
//
// MC_CMD_GET_SRIOV_CFG
// Get SRIOV config for this PF.
//
pub const MC_CMD_GET_SRIOV_CFG: c_uint = 0xba;

// MC_CMD_GET_SRIOV_CFG_IN msgrequest
pub const MC_CMD_GET_SRIOV_CFG_IN_LEN: c_int = 0;
// MC_CMD_GET_SRIOV_CFG_OUT msgresponse
pub const MC_CMD_GET_SRIOV_CFG_OUT_LEN: c_int = 20;
// Number of VFs currently enabled.
pub const MC_CMD_GET_SRIOV_CFG_OUT_VF_CURRENT_OFST: c_int = 0;
pub const MC_CMD_GET_SRIOV_CFG_OUT_VF_CURRENT_LEN: c_int = 4;
// Max number of VFs before sriov stride and offset may need to be changed.
pub const MC_CMD_GET_SRIOV_CFG_OUT_VF_MAX_OFST: c_int = 4;
pub const MC_CMD_GET_SRIOV_CFG_OUT_VF_MAX_LEN: c_int = 4;
pub const MC_CMD_GET_SRIOV_CFG_OUT_FLAGS_OFST: c_int = 8;
pub const MC_CMD_GET_SRIOV_CFG_OUT_FLAGS_LEN: c_int = 4;
pub const MC_CMD_GET_SRIOV_CFG_OUT_VF_ENABLED_OFST: c_int = 8;
pub const MC_CMD_GET_SRIOV_CFG_OUT_VF_ENABLED_LBN: c_int = 0;
pub const MC_CMD_GET_SRIOV_CFG_OUT_VF_ENABLED_WIDTH: c_int = 1;
// RID offset of first VF from PF.
pub const MC_CMD_GET_SRIOV_CFG_OUT_VF_OFFSET_OFST: c_int = 12;
pub const MC_CMD_GET_SRIOV_CFG_OUT_VF_OFFSET_LEN: c_int = 4;
// RID offset of each subsequent VF from the previous.
pub const MC_CMD_GET_SRIOV_CFG_OUT_VF_STRIDE_OFST: c_int = 16;
pub const MC_CMD_GET_SRIOV_CFG_OUT_VF_STRIDE_LEN: c_int = 4;
//
// MC_CMD_SET_SRIOV_CFG
// Set SRIOV config for this PF.
//
pub const MC_CMD_SET_SRIOV_CFG: c_uint = 0xbb;

// MC_CMD_SET_SRIOV_CFG_IN msgrequest
pub const MC_CMD_SET_SRIOV_CFG_IN_LEN: c_int = 20;
// Number of VFs currently enabled.
pub const MC_CMD_SET_SRIOV_CFG_IN_VF_CURRENT_OFST: c_int = 0;
pub const MC_CMD_SET_SRIOV_CFG_IN_VF_CURRENT_LEN: c_int = 4;
// Max number of VFs before sriov stride and offset may need to be changed.
pub const MC_CMD_SET_SRIOV_CFG_IN_VF_MAX_OFST: c_int = 4;
pub const MC_CMD_SET_SRIOV_CFG_IN_VF_MAX_LEN: c_int = 4;
pub const MC_CMD_SET_SRIOV_CFG_IN_FLAGS_OFST: c_int = 8;
pub const MC_CMD_SET_SRIOV_CFG_IN_FLAGS_LEN: c_int = 4;
pub const MC_CMD_SET_SRIOV_CFG_IN_VF_ENABLED_OFST: c_int = 8;
pub const MC_CMD_SET_SRIOV_CFG_IN_VF_ENABLED_LBN: c_int = 0;
pub const MC_CMD_SET_SRIOV_CFG_IN_VF_ENABLED_WIDTH: c_int = 1;
// RID offset of first VF from PF, or 0 for no change, or
// MC_CMD_RESOURCE_INSTANCE_ANY to allow the system to allocate an offset.
//
pub const MC_CMD_SET_SRIOV_CFG_IN_VF_OFFSET_OFST: c_int = 12;
pub const MC_CMD_SET_SRIOV_CFG_IN_VF_OFFSET_LEN: c_int = 4;
// RID offset of each subsequent VF from the previous, 0 for no change, or
// MC_CMD_RESOURCE_INSTANCE_ANY to allow the system to allocate a stride.
//
pub const MC_CMD_SET_SRIOV_CFG_IN_VF_STRIDE_OFST: c_int = 16;
pub const MC_CMD_SET_SRIOV_CFG_IN_VF_STRIDE_LEN: c_int = 4;
// MC_CMD_SET_SRIOV_CFG_OUT msgresponse
pub const MC_CMD_SET_SRIOV_CFG_OUT_LEN: c_int = 0;
//
// MC_CMD_GET_VI_ALLOC_INFO
// Get information about number of VI's and base VI number allocated to this
// function.
//
pub const MC_CMD_GET_VI_ALLOC_INFO: c_uint = 0x8d;

// MC_CMD_GET_VI_ALLOC_INFO_IN msgrequest
pub const MC_CMD_GET_VI_ALLOC_INFO_IN_LEN: c_int = 0;
// MC_CMD_GET_VI_ALLOC_INFO_OUT msgresponse
pub const MC_CMD_GET_VI_ALLOC_INFO_OUT_LEN: c_int = 12;
// The number of VIs allocated on this function
pub const MC_CMD_GET_VI_ALLOC_INFO_OUT_VI_COUNT_OFST: c_int = 0;
pub const MC_CMD_GET_VI_ALLOC_INFO_OUT_VI_COUNT_LEN: c_int = 4;
// The base absolute VI number allocated to this function. Required to
// correctly interpret wakeup events.
//
pub const MC_CMD_GET_VI_ALLOC_INFO_OUT_VI_BASE_OFST: c_int = 4;
pub const MC_CMD_GET_VI_ALLOC_INFO_OUT_VI_BASE_LEN: c_int = 4;
// Function's port vi_shift value (always 0 on Huntington)
pub const MC_CMD_GET_VI_ALLOC_INFO_OUT_VI_SHIFT_OFST: c_int = 8;
pub const MC_CMD_GET_VI_ALLOC_INFO_OUT_VI_SHIFT_LEN: c_int = 4;
//
// MC_CMD_DUMP_VI_STATE
// For CmdClient use. Dump pertinent information on a specific absolute VI.
//
pub const MC_CMD_DUMP_VI_STATE: c_uint = 0x8e;

// MC_CMD_DUMP_VI_STATE_IN msgrequest
pub const MC_CMD_DUMP_VI_STATE_IN_LEN: c_int = 4;
// The VI number to query.
pub const MC_CMD_DUMP_VI_STATE_IN_VI_NUMBER_OFST: c_int = 0;
pub const MC_CMD_DUMP_VI_STATE_IN_VI_NUMBER_LEN: c_int = 4;
// MC_CMD_DUMP_VI_STATE_OUT msgresponse
pub const MC_CMD_DUMP_VI_STATE_OUT_LEN: c_int = 96;
// The PF part of the function owning this VI.
pub const MC_CMD_DUMP_VI_STATE_OUT_OWNER_PF_OFST: c_int = 0;
pub const MC_CMD_DUMP_VI_STATE_OUT_OWNER_PF_LEN: c_int = 2;
// The VF part of the function owning this VI.
pub const MC_CMD_DUMP_VI_STATE_OUT_OWNER_VF_OFST: c_int = 2;
pub const MC_CMD_DUMP_VI_STATE_OUT_OWNER_VF_LEN: c_int = 2;
// Base of VIs allocated to this function.
pub const MC_CMD_DUMP_VI_STATE_OUT_FUNC_VI_BASE_OFST: c_int = 4;
pub const MC_CMD_DUMP_VI_STATE_OUT_FUNC_VI_BASE_LEN: c_int = 2;
// Count of VIs allocated to the owner function.
pub const MC_CMD_DUMP_VI_STATE_OUT_FUNC_VI_COUNT_OFST: c_int = 6;
pub const MC_CMD_DUMP_VI_STATE_OUT_FUNC_VI_COUNT_LEN: c_int = 2;
// Base interrupt vector allocated to this function.
pub const MC_CMD_DUMP_VI_STATE_OUT_FUNC_VECTOR_BASE_OFST: c_int = 8;
pub const MC_CMD_DUMP_VI_STATE_OUT_FUNC_VECTOR_BASE_LEN: c_int = 2;
// Number of interrupt vectors allocated to this function.
pub const MC_CMD_DUMP_VI_STATE_OUT_FUNC_VECTOR_COUNT_OFST: c_int = 10;
pub const MC_CMD_DUMP_VI_STATE_OUT_FUNC_VECTOR_COUNT_LEN: c_int = 2;
// Raw evq ptr table data.
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_EVQ_PTR_RAW_OFST: c_int = 12;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_EVQ_PTR_RAW_LEN: c_int = 8;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_EVQ_PTR_RAW_LO_OFST: c_int = 12;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_EVQ_PTR_RAW_HI_OFST: c_int = 16;
// Raw evq timer table data.
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_EV_TIMER_RAW_OFST: c_int = 20;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_EV_TIMER_RAW_LEN: c_int = 8;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_EV_TIMER_RAW_LO_OFST: c_int = 20;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_EV_TIMER_RAW_HI_OFST: c_int = 24;
// Combined metadata field.
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_EV_META_OFST: c_int = 28;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_EV_META_LEN: c_int = 4;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_EV_META_BUFS_BASE_OFST: c_int = 28;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_EV_META_BUFS_BASE_LBN: c_int = 0;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_EV_META_BUFS_BASE_WIDTH: c_int = 16;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_EV_META_BUFS_NPAGES_OFST: c_int = 28;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_EV_META_BUFS_NPAGES_LBN: c_int = 16;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_EV_META_BUFS_NPAGES_WIDTH: c_int = 8;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_EV_META_WKUP_REF_OFST: c_int = 28;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_EV_META_WKUP_REF_LBN: c_int = 24;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_EV_META_WKUP_REF_WIDTH: c_int = 8;
// TXDPCPU raw table data for queue.
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_TX_RAW_TBL_0_OFST: c_int = 32;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_TX_RAW_TBL_0_LEN: c_int = 8;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_TX_RAW_TBL_0_LO_OFST: c_int = 32;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_TX_RAW_TBL_0_HI_OFST: c_int = 36;
// TXDPCPU raw table data for queue.
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_TX_RAW_TBL_1_OFST: c_int = 40;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_TX_RAW_TBL_1_LEN: c_int = 8;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_TX_RAW_TBL_1_LO_OFST: c_int = 40;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_TX_RAW_TBL_1_HI_OFST: c_int = 44;
// TXDPCPU raw table data for queue.
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_TX_RAW_TBL_2_OFST: c_int = 48;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_TX_RAW_TBL_2_LEN: c_int = 8;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_TX_RAW_TBL_2_LO_OFST: c_int = 48;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_TX_RAW_TBL_2_HI_OFST: c_int = 52;
// Combined metadata field.
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_TX_META_OFST: c_int = 56;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_TX_META_LEN: c_int = 8;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_TX_META_LO_OFST: c_int = 56;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_TX_META_HI_OFST: c_int = 60;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_TX_META_BUFS_BASE_OFST: c_int = 56;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_TX_META_BUFS_BASE_LBN: c_int = 0;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_TX_META_BUFS_BASE_WIDTH: c_int = 16;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_TX_META_BUFS_NPAGES_OFST: c_int = 56;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_TX_META_BUFS_NPAGES_LBN: c_int = 16;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_TX_META_BUFS_NPAGES_WIDTH: c_int = 8;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_TX_META_QSTATE_OFST: c_int = 56;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_TX_META_QSTATE_LBN: c_int = 24;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_TX_META_QSTATE_WIDTH: c_int = 8;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_TX_META_WAITCOUNT_OFST: c_int = 56;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_TX_META_WAITCOUNT_LBN: c_int = 32;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_TX_META_WAITCOUNT_WIDTH: c_int = 8;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_PADDING_OFST: c_int = 56;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_PADDING_LBN: c_int = 40;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_PADDING_WIDTH: c_int = 24;
// RXDPCPU raw table data for queue.
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_RX_RAW_TBL_0_OFST: c_int = 64;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_RX_RAW_TBL_0_LEN: c_int = 8;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_RX_RAW_TBL_0_LO_OFST: c_int = 64;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_RX_RAW_TBL_0_HI_OFST: c_int = 68;
// RXDPCPU raw table data for queue.
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_RX_RAW_TBL_1_OFST: c_int = 72;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_RX_RAW_TBL_1_LEN: c_int = 8;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_RX_RAW_TBL_1_LO_OFST: c_int = 72;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_RX_RAW_TBL_1_HI_OFST: c_int = 76;
// Reserved, currently 0.
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_RX_RAW_TBL_2_OFST: c_int = 80;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_RX_RAW_TBL_2_LEN: c_int = 8;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_RX_RAW_TBL_2_LO_OFST: c_int = 80;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_RX_RAW_TBL_2_HI_OFST: c_int = 84;
// Combined metadata field.
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_RX_META_OFST: c_int = 88;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_RX_META_LEN: c_int = 8;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_RX_META_LO_OFST: c_int = 88;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_RX_META_HI_OFST: c_int = 92;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_RX_META_BUFS_BASE_OFST: c_int = 88;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_RX_META_BUFS_BASE_LBN: c_int = 0;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_RX_META_BUFS_BASE_WIDTH: c_int = 16;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_RX_META_BUFS_NPAGES_OFST: c_int = 88;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_RX_META_BUFS_NPAGES_LBN: c_int = 16;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_RX_META_BUFS_NPAGES_WIDTH: c_int = 8;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_RX_META_QSTATE_OFST: c_int = 88;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_RX_META_QSTATE_LBN: c_int = 24;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_RX_META_QSTATE_WIDTH: c_int = 8;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_RX_META_WAITCOUNT_OFST: c_int = 88;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_RX_META_WAITCOUNT_LBN: c_int = 32;
pub const MC_CMD_DUMP_VI_STATE_OUT_VI_RX_META_WAITCOUNT_WIDTH: c_int = 8;
//
// MC_CMD_ALLOC_PIOBUF
// Allocate a push I/O buffer for later use with a tx queue.
//
pub const MC_CMD_ALLOC_PIOBUF: c_uint = 0x8f;

// MC_CMD_ALLOC_PIOBUF_IN msgrequest
pub const MC_CMD_ALLOC_PIOBUF_IN_LEN: c_int = 0;
// MC_CMD_ALLOC_PIOBUF_OUT msgresponse
pub const MC_CMD_ALLOC_PIOBUF_OUT_LEN: c_int = 4;
// Handle for allocated push I/O buffer.
pub const MC_CMD_ALLOC_PIOBUF_OUT_PIOBUF_HANDLE_OFST: c_int = 0;
pub const MC_CMD_ALLOC_PIOBUF_OUT_PIOBUF_HANDLE_LEN: c_int = 4;
//
// MC_CMD_FREE_PIOBUF
// Free a push I/O buffer.
//
pub const MC_CMD_FREE_PIOBUF: c_uint = 0x90;

// MC_CMD_FREE_PIOBUF_IN msgrequest
pub const MC_CMD_FREE_PIOBUF_IN_LEN: c_int = 4;
// Handle for allocated push I/O buffer.
pub const MC_CMD_FREE_PIOBUF_IN_PIOBUF_HANDLE_OFST: c_int = 0;
pub const MC_CMD_FREE_PIOBUF_IN_PIOBUF_HANDLE_LEN: c_int = 4;
// MC_CMD_FREE_PIOBUF_OUT msgresponse
pub const MC_CMD_FREE_PIOBUF_OUT_LEN: c_int = 0;
//
// MC_CMD_GET_CAPABILITIES
// Get device capabilities.
//
// This is supplementary to the MC_CMD_GET_BOARD_CFG command, and intended to
// reference inherent device capabilities as opposed to current NVRAM config.
//
pub const MC_CMD_GET_CAPABILITIES: c_uint = 0xbe;

// MC_CMD_GET_CAPABILITIES_IN msgrequest
pub const MC_CMD_GET_CAPABILITIES_IN_LEN: c_int = 0;
// MC_CMD_GET_CAPABILITIES_OUT msgresponse
pub const MC_CMD_GET_CAPABILITIES_OUT_LEN: c_int = 20;
// First word of flags.
pub const MC_CMD_GET_CAPABILITIES_OUT_FLAGS1_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_OUT_FLAGS1_LEN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_OUT_VPORT_RECONFIGURE_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_OUT_VPORT_RECONFIGURE_LBN: c_int = 3;
pub const MC_CMD_GET_CAPABILITIES_OUT_VPORT_RECONFIGURE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_OUT_TX_STRIPING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_OUT_TX_STRIPING_LBN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_OUT_TX_STRIPING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_OUT_VADAPTOR_QUERY_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_OUT_VADAPTOR_QUERY_LBN: c_int = 5;
pub const MC_CMD_GET_CAPABILITIES_OUT_VADAPTOR_QUERY_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_OUT_EVB_PORT_VLAN_RESTRICT_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_OUT_EVB_PORT_VLAN_RESTRICT_LBN: c_int = 6;
pub const MC_CMD_GET_CAPABILITIES_OUT_EVB_PORT_VLAN_RESTRICT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_OUT_DRV_ATTACH_PREBOOT_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_OUT_DRV_ATTACH_PREBOOT_LBN: c_int = 7;
pub const MC_CMD_GET_CAPABILITIES_OUT_DRV_ATTACH_PREBOOT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_OUT_RX_FORCE_EVENT_MERGING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_OUT_RX_FORCE_EVENT_MERGING_LBN: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_OUT_RX_FORCE_EVENT_MERGING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_OUT_SET_MAC_ENHANCED_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_OUT_SET_MAC_ENHANCED_LBN: c_int = 9;
pub const MC_CMD_GET_CAPABILITIES_OUT_SET_MAC_ENHANCED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_OUT_UNKNOWN_UCAST_DST_FILTER_ALWAYS_MULTI_RECIPIENT_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_OUT_UNKNOWN_UCAST_DST_FILTER_ALWAYS_MULTI_RECIPIENT_LBN: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_OUT_UNKNOWN_UCAST_DST_FILTER_ALWAYS_MULTI_RECIPIENT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_OUT_VADAPTOR_PERMIT_SET_MAC_WHEN_FILTERS_INSTALLED_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_OUT_VADAPTOR_PERMIT_SET_MAC_WHEN_FILTERS_INSTALLED_LBN: c_int = 11;
pub const MC_CMD_GET_CAPABILITIES_OUT_VADAPTOR_PERMIT_SET_MAC_WHEN_FILTERS_INSTALLED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_OUT_TX_MAC_SECURITY_FILTERING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_OUT_TX_MAC_SECURITY_FILTERING_LBN: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_OUT_TX_MAC_SECURITY_FILTERING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_OUT_ADDITIONAL_RSS_MODES_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_OUT_ADDITIONAL_RSS_MODES_LBN: c_int = 13;
pub const MC_CMD_GET_CAPABILITIES_OUT_ADDITIONAL_RSS_MODES_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_OUT_QBB_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_OUT_QBB_LBN: c_int = 14;
pub const MC_CMD_GET_CAPABILITIES_OUT_QBB_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_OUT_RX_PACKED_STREAM_VAR_BUFFERS_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_OUT_RX_PACKED_STREAM_VAR_BUFFERS_LBN: c_int = 15;
pub const MC_CMD_GET_CAPABILITIES_OUT_RX_PACKED_STREAM_VAR_BUFFERS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_OUT_RX_RSS_LIMITED_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_OUT_RX_RSS_LIMITED_LBN: c_int = 16;
pub const MC_CMD_GET_CAPABILITIES_OUT_RX_RSS_LIMITED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_OUT_RX_PACKED_STREAM_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_OUT_RX_PACKED_STREAM_LBN: c_int = 17;
pub const MC_CMD_GET_CAPABILITIES_OUT_RX_PACKED_STREAM_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_OUT_RX_INCLUDE_FCS_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_OUT_RX_INCLUDE_FCS_LBN: c_int = 18;
pub const MC_CMD_GET_CAPABILITIES_OUT_RX_INCLUDE_FCS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_OUT_TX_VLAN_INSERTION_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_OUT_TX_VLAN_INSERTION_LBN: c_int = 19;
pub const MC_CMD_GET_CAPABILITIES_OUT_TX_VLAN_INSERTION_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_OUT_RX_VLAN_STRIPPING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_OUT_RX_VLAN_STRIPPING_LBN: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_OUT_RX_VLAN_STRIPPING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_OUT_TX_TSO_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_OUT_TX_TSO_LBN: c_int = 21;
pub const MC_CMD_GET_CAPABILITIES_OUT_TX_TSO_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_OUT_RX_PREFIX_LEN_0_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_OUT_RX_PREFIX_LEN_0_LBN: c_int = 22;
pub const MC_CMD_GET_CAPABILITIES_OUT_RX_PREFIX_LEN_0_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_OUT_RX_PREFIX_LEN_14_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_OUT_RX_PREFIX_LEN_14_LBN: c_int = 23;
pub const MC_CMD_GET_CAPABILITIES_OUT_RX_PREFIX_LEN_14_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_OUT_RX_TIMESTAMP_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_OUT_RX_TIMESTAMP_LBN: c_int = 24;
pub const MC_CMD_GET_CAPABILITIES_OUT_RX_TIMESTAMP_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_OUT_RX_BATCHING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_OUT_RX_BATCHING_LBN: c_int = 25;
pub const MC_CMD_GET_CAPABILITIES_OUT_RX_BATCHING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_OUT_MCAST_FILTER_CHAINING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_OUT_MCAST_FILTER_CHAINING_LBN: c_int = 26;
pub const MC_CMD_GET_CAPABILITIES_OUT_MCAST_FILTER_CHAINING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_OUT_PM_AND_RXDP_COUNTERS_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_OUT_PM_AND_RXDP_COUNTERS_LBN: c_int = 27;
pub const MC_CMD_GET_CAPABILITIES_OUT_PM_AND_RXDP_COUNTERS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_OUT_RX_DISABLE_SCATTER_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_OUT_RX_DISABLE_SCATTER_LBN: c_int = 28;
pub const MC_CMD_GET_CAPABILITIES_OUT_RX_DISABLE_SCATTER_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_OUT_TX_MCAST_UDP_LOOPBACK_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_OUT_TX_MCAST_UDP_LOOPBACK_LBN: c_int = 29;
pub const MC_CMD_GET_CAPABILITIES_OUT_TX_MCAST_UDP_LOOPBACK_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_OUT_EVB_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_OUT_EVB_LBN: c_int = 30;
pub const MC_CMD_GET_CAPABILITIES_OUT_EVB_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_OUT_VXLAN_NVGRE_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_OUT_VXLAN_NVGRE_LBN: c_int = 31;
pub const MC_CMD_GET_CAPABILITIES_OUT_VXLAN_NVGRE_WIDTH: c_int = 1;
// RxDPCPU firmware id.
pub const MC_CMD_GET_CAPABILITIES_OUT_RX_DPCPU_FW_ID_OFST: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_OUT_RX_DPCPU_FW_ID_LEN: c_int = 2;
// enum: Standard RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_OUT_RXDP: c_uint = 0x0;
// enum: Low latency RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_OUT_RXDP_LOW_LATENCY: c_uint = 0x1;
// enum: Packed stream RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_OUT_RXDP_PACKED_STREAM: c_uint = 0x2;
// enum: Rules engine RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_OUT_RXDP_RULES_ENGINE: c_uint = 0x5;
// enum: DPDK RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_OUT_RXDP_DPDK: c_uint = 0x6;
// enum: BIST RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_OUT_RXDP_BIST: c_uint = 0x10a;
// enum: RXDP Test firmware image 1
pub const MC_CMD_GET_CAPABILITIES_OUT_RXDP_TEST_FW_TO_MC_CUT_THROUGH: c_uint = 0x101;
// enum: RXDP Test firmware image 2
pub const MC_CMD_GET_CAPABILITIES_OUT_RXDP_TEST_FW_TO_MC_STORE_FORWARD: c_uint = 0x102;
// enum: RXDP Test firmware image 3
pub const MC_CMD_GET_CAPABILITIES_OUT_RXDP_TEST_FW_TO_MC_STORE_FORWARD_FIRST: c_uint = 0x103;
// enum: RXDP Test firmware image 4
pub const MC_CMD_GET_CAPABILITIES_OUT_RXDP_TEST_EVERY_EVENT_BATCHABLE: c_uint = 0x104;
// enum: RXDP Test firmware image 5
pub const MC_CMD_GET_CAPABILITIES_OUT_RXDP_TEST_BACKPRESSURE: c_uint = 0x105;
// enum: RXDP Test firmware image 6
pub const MC_CMD_GET_CAPABILITIES_OUT_RXDP_TEST_FW_PACKET_EDITS: c_uint = 0x106;
// enum: RXDP Test firmware image 7
pub const MC_CMD_GET_CAPABILITIES_OUT_RXDP_TEST_FW_RX_HDR_SPLIT: c_uint = 0x107;
// enum: RXDP Test firmware image 8
pub const MC_CMD_GET_CAPABILITIES_OUT_RXDP_TEST_FW_DISABLE_DL: c_uint = 0x108;
// enum: RXDP Test firmware image 9
pub const MC_CMD_GET_CAPABILITIES_OUT_RXDP_TEST_FW_DOORBELL_DELAY: c_uint = 0x10b;
// enum: RXDP Test firmware image 10
pub const MC_CMD_GET_CAPABILITIES_OUT_RXDP_TEST_FW_SLOW: c_uint = 0x10c;
// TxDPCPU firmware id.
pub const MC_CMD_GET_CAPABILITIES_OUT_TX_DPCPU_FW_ID_OFST: c_int = 6;
pub const MC_CMD_GET_CAPABILITIES_OUT_TX_DPCPU_FW_ID_LEN: c_int = 2;
// enum: Standard TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_OUT_TXDP: c_uint = 0x0;
// enum: Low latency TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_OUT_TXDP_LOW_LATENCY: c_uint = 0x1;
// enum: High packet rate TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_OUT_TXDP_HIGH_PACKET_RATE: c_uint = 0x3;
// enum: Rules engine TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_OUT_TXDP_RULES_ENGINE: c_uint = 0x5;
// enum: DPDK TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_OUT_TXDP_DPDK: c_uint = 0x6;
// enum: BIST TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_OUT_TXDP_BIST: c_uint = 0x12d;
// enum: TXDP Test firmware image 1
pub const MC_CMD_GET_CAPABILITIES_OUT_TXDP_TEST_FW_TSO_EDIT: c_uint = 0x101;
// enum: TXDP Test firmware image 2
pub const MC_CMD_GET_CAPABILITIES_OUT_TXDP_TEST_FW_PACKET_EDITS: c_uint = 0x102;
// enum: TXDP CSR bus test firmware
pub const MC_CMD_GET_CAPABILITIES_OUT_TXDP_TEST_FW_CSR: c_uint = 0x103;
pub const MC_CMD_GET_CAPABILITIES_OUT_RXPD_FW_VERSION_OFST: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_OUT_RXPD_FW_VERSION_LEN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_OUT_RXPD_FW_VERSION_REV_OFST: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_OUT_RXPD_FW_VERSION_REV_LBN: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_OUT_RXPD_FW_VERSION_REV_WIDTH: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_OUT_RXPD_FW_VERSION_TYPE_OFST: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_OUT_RXPD_FW_VERSION_TYPE_LBN: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_OUT_RXPD_FW_VERSION_TYPE_WIDTH: c_int = 4;
// enum: reserved value - do not use (may indicate alternative interpretation
// of REV field in future)
//
pub const MC_CMD_GET_CAPABILITIES_OUT_RXPD_FW_TYPE_RESERVED: c_uint = 0x0;
// enum: Trivial RX PD firmware for early Huntington development (Huntington
// development only)
//
pub const MC_CMD_GET_CAPABILITIES_OUT_RXPD_FW_TYPE_FIRST_PKT: c_uint = 0x1;
// enum: RX PD firmware for telemetry prototyping (Medford2 development only)
//
pub const MC_CMD_GET_CAPABILITIES_OUT_RXPD_FW_TYPE_TESTFW_TELEMETRY: c_uint = 0x1;
// enum: RX PD firmware with approximately Siena-compatible behaviour
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_OUT_RXPD_FW_TYPE_SIENA_COMPAT: c_uint = 0x2;
// enum: Full featured RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_OUT_RXPD_FW_TYPE_FULL_FEATURED: c_uint = 0x3;
// enum: (deprecated original name for the FULL_FEATURED variant)
pub const MC_CMD_GET_CAPABILITIES_OUT_RXPD_FW_TYPE_VSWITCH: c_uint = 0x3;
// enum: siena_compat variant RX PD firmware using PM rather than MAC
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_OUT_RXPD_FW_TYPE_SIENA_COMPAT_PM: c_uint = 0x4;
// enum: Low latency RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_OUT_RXPD_FW_TYPE_LOW_LATENCY: c_uint = 0x5;
// enum: Packed stream RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_OUT_RXPD_FW_TYPE_PACKED_STREAM: c_uint = 0x6;
// enum: RX PD firmware handling layer 2 only for high packet rate performance
// tests (Medford development only)
//
pub const MC_CMD_GET_CAPABILITIES_OUT_RXPD_FW_TYPE_LAYER2_PERF: c_uint = 0x7;
// enum: Rules engine RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_OUT_RXPD_FW_TYPE_RULES_ENGINE: c_uint = 0x8;
// enum: Custom firmware variant (see SF-119495-PD and bug69716)
pub const MC_CMD_GET_CAPABILITIES_OUT_RXPD_FW_TYPE_L3XUDP: c_uint = 0x9;
// enum: DPDK RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_OUT_RXPD_FW_TYPE_DPDK: c_uint = 0xa;
// enum: RX PD firmware for GUE parsing prototype (Medford development only)
pub const MC_CMD_GET_CAPABILITIES_OUT_RXPD_FW_TYPE_TESTFW_GUE_PROTOTYPE: c_uint = 0xe;
// enum: RX PD firmware parsing but not filtering network overlay tunnel
// encapsulations (Medford development only)
//
pub const MC_CMD_GET_CAPABILITIES_OUT_RXPD_FW_TYPE_TESTFW_ENCAP_PARSING_ONLY: c_uint = 0xf;
pub const MC_CMD_GET_CAPABILITIES_OUT_TXPD_FW_VERSION_OFST: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_OUT_TXPD_FW_VERSION_LEN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_OUT_TXPD_FW_VERSION_REV_OFST: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_OUT_TXPD_FW_VERSION_REV_LBN: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_OUT_TXPD_FW_VERSION_REV_WIDTH: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_OUT_TXPD_FW_VERSION_TYPE_OFST: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_OUT_TXPD_FW_VERSION_TYPE_LBN: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_OUT_TXPD_FW_VERSION_TYPE_WIDTH: c_int = 4;
// enum: reserved value - do not use (may indicate alternative interpretation
// of REV field in future)
//
pub const MC_CMD_GET_CAPABILITIES_OUT_TXPD_FW_TYPE_RESERVED: c_uint = 0x0;
// enum: Trivial TX PD firmware for early Huntington development (Huntington
// development only)
//
pub const MC_CMD_GET_CAPABILITIES_OUT_TXPD_FW_TYPE_FIRST_PKT: c_uint = 0x1;
// enum: TX PD firmware for telemetry prototyping (Medford2 development only)
//
pub const MC_CMD_GET_CAPABILITIES_OUT_TXPD_FW_TYPE_TESTFW_TELEMETRY: c_uint = 0x1;
// enum: TX PD firmware with approximately Siena-compatible behaviour
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_OUT_TXPD_FW_TYPE_SIENA_COMPAT: c_uint = 0x2;
// enum: Full featured TX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_OUT_TXPD_FW_TYPE_FULL_FEATURED: c_uint = 0x3;
// enum: (deprecated original name for the FULL_FEATURED variant)
pub const MC_CMD_GET_CAPABILITIES_OUT_TXPD_FW_TYPE_VSWITCH: c_uint = 0x3;
// enum: siena_compat variant TX PD firmware using PM rather than MAC
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_OUT_TXPD_FW_TYPE_SIENA_COMPAT_PM: c_uint = 0x4;
pub const MC_CMD_GET_CAPABILITIES_OUT_TXPD_FW_TYPE_LOW_LATENCY: c_uint = 0x5 /* enum */;
// enum: TX PD firmware handling layer 2 only for high packet rate performance
// tests (Medford development only)
//
pub const MC_CMD_GET_CAPABILITIES_OUT_TXPD_FW_TYPE_LAYER2_PERF: c_uint = 0x7;
// enum: Rules engine TX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_OUT_TXPD_FW_TYPE_RULES_ENGINE: c_uint = 0x8;
// enum: Custom firmware variant (see SF-119495-PD and bug69716)
pub const MC_CMD_GET_CAPABILITIES_OUT_TXPD_FW_TYPE_L3XUDP: c_uint = 0x9;
// enum: DPDK TX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_OUT_TXPD_FW_TYPE_DPDK: c_uint = 0xa;
// enum: RX PD firmware for GUE parsing prototype (Medford development only)
pub const MC_CMD_GET_CAPABILITIES_OUT_TXPD_FW_TYPE_TESTFW_GUE_PROTOTYPE: c_uint = 0xe;
// Hardware capabilities of NIC
pub const MC_CMD_GET_CAPABILITIES_OUT_HW_CAPABILITIES_OFST: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_OUT_HW_CAPABILITIES_LEN: c_int = 4;
// Licensed capabilities
pub const MC_CMD_GET_CAPABILITIES_OUT_LICENSE_CAPABILITIES_OFST: c_int = 16;
pub const MC_CMD_GET_CAPABILITIES_OUT_LICENSE_CAPABILITIES_LEN: c_int = 4;
// MC_CMD_GET_CAPABILITIES_V2_IN msgrequest
pub const MC_CMD_GET_CAPABILITIES_V2_IN_LEN: c_int = 0;
// MC_CMD_GET_CAPABILITIES_V2_OUT msgresponse
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_LEN: c_int = 72;
// First word of flags.
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_FLAGS1_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_FLAGS1_LEN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_VPORT_RECONFIGURE_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_VPORT_RECONFIGURE_LBN: c_int = 3;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_VPORT_RECONFIGURE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_STRIPING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_STRIPING_LBN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_STRIPING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_VADAPTOR_QUERY_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_VADAPTOR_QUERY_LBN: c_int = 5;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_VADAPTOR_QUERY_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_EVB_PORT_VLAN_RESTRICT_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_EVB_PORT_VLAN_RESTRICT_LBN: c_int = 6;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_EVB_PORT_VLAN_RESTRICT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_DRV_ATTACH_PREBOOT_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_DRV_ATTACH_PREBOOT_LBN: c_int = 7;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_DRV_ATTACH_PREBOOT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_FORCE_EVENT_MERGING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_FORCE_EVENT_MERGING_LBN: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_FORCE_EVENT_MERGING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_SET_MAC_ENHANCED_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_SET_MAC_ENHANCED_LBN: c_int = 9;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_SET_MAC_ENHANCED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_UNKNOWN_UCAST_DST_FILTER_ALWAYS_MULTI_RECIPIENT_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_UNKNOWN_UCAST_DST_FILTER_ALWAYS_MULTI_RECIPIENT_LBN: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_UNKNOWN_UCAST_DST_FILTER_ALWAYS_MULTI_RECIPIENT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_VADAPTOR_PERMIT_SET_MAC_WHEN_FILTERS_INSTALLED_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_VADAPTOR_PERMIT_SET_MAC_WHEN_FILTERS_INSTALLED_LBN: c_int = 11;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_VADAPTOR_PERMIT_SET_MAC_WHEN_FILTERS_INSTALLED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_MAC_SECURITY_FILTERING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_MAC_SECURITY_FILTERING_LBN: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_MAC_SECURITY_FILTERING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_ADDITIONAL_RSS_MODES_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_ADDITIONAL_RSS_MODES_LBN: c_int = 13;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_ADDITIONAL_RSS_MODES_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_QBB_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_QBB_LBN: c_int = 14;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_QBB_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_PACKED_STREAM_VAR_BUFFERS_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_PACKED_STREAM_VAR_BUFFERS_LBN: c_int = 15;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_PACKED_STREAM_VAR_BUFFERS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_RSS_LIMITED_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_RSS_LIMITED_LBN: c_int = 16;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_RSS_LIMITED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_PACKED_STREAM_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_PACKED_STREAM_LBN: c_int = 17;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_PACKED_STREAM_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_INCLUDE_FCS_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_INCLUDE_FCS_LBN: c_int = 18;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_INCLUDE_FCS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_VLAN_INSERTION_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_VLAN_INSERTION_LBN: c_int = 19;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_VLAN_INSERTION_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_VLAN_STRIPPING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_VLAN_STRIPPING_LBN: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_VLAN_STRIPPING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_TSO_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_TSO_LBN: c_int = 21;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_TSO_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_PREFIX_LEN_0_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_PREFIX_LEN_0_LBN: c_int = 22;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_PREFIX_LEN_0_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_PREFIX_LEN_14_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_PREFIX_LEN_14_LBN: c_int = 23;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_PREFIX_LEN_14_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_TIMESTAMP_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_TIMESTAMP_LBN: c_int = 24;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_TIMESTAMP_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_BATCHING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_BATCHING_LBN: c_int = 25;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_BATCHING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_MCAST_FILTER_CHAINING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_MCAST_FILTER_CHAINING_LBN: c_int = 26;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_MCAST_FILTER_CHAINING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_PM_AND_RXDP_COUNTERS_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_PM_AND_RXDP_COUNTERS_LBN: c_int = 27;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_PM_AND_RXDP_COUNTERS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_DISABLE_SCATTER_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_DISABLE_SCATTER_LBN: c_int = 28;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_DISABLE_SCATTER_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_MCAST_UDP_LOOPBACK_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_MCAST_UDP_LOOPBACK_LBN: c_int = 29;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_MCAST_UDP_LOOPBACK_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_EVB_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_EVB_LBN: c_int = 30;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_EVB_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_VXLAN_NVGRE_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_VXLAN_NVGRE_LBN: c_int = 31;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_VXLAN_NVGRE_WIDTH: c_int = 1;
// RxDPCPU firmware id.
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_DPCPU_FW_ID_OFST: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_DPCPU_FW_ID_LEN: c_int = 2;
// enum: Standard RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXDP: c_uint = 0x0;
// enum: Low latency RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXDP_LOW_LATENCY: c_uint = 0x1;
// enum: Packed stream RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXDP_PACKED_STREAM: c_uint = 0x2;
// enum: Rules engine RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXDP_RULES_ENGINE: c_uint = 0x5;
// enum: DPDK RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXDP_DPDK: c_uint = 0x6;
// enum: BIST RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXDP_BIST: c_uint = 0x10a;
// enum: RXDP Test firmware image 1
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXDP_TEST_FW_TO_MC_CUT_THROUGH: c_uint = 0x101;
// enum: RXDP Test firmware image 2
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXDP_TEST_FW_TO_MC_STORE_FORWARD: c_uint = 0x102;
// enum: RXDP Test firmware image 3
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXDP_TEST_FW_TO_MC_STORE_FORWARD_FIRST: c_uint = 0x103;
// enum: RXDP Test firmware image 4
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXDP_TEST_EVERY_EVENT_BATCHABLE: c_uint = 0x104;
// enum: RXDP Test firmware image 5
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXDP_TEST_BACKPRESSURE: c_uint = 0x105;
// enum: RXDP Test firmware image 6
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXDP_TEST_FW_PACKET_EDITS: c_uint = 0x106;
// enum: RXDP Test firmware image 7
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXDP_TEST_FW_RX_HDR_SPLIT: c_uint = 0x107;
// enum: RXDP Test firmware image 8
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXDP_TEST_FW_DISABLE_DL: c_uint = 0x108;
// enum: RXDP Test firmware image 9
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXDP_TEST_FW_DOORBELL_DELAY: c_uint = 0x10b;
// enum: RXDP Test firmware image 10
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXDP_TEST_FW_SLOW: c_uint = 0x10c;
// TxDPCPU firmware id.
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_DPCPU_FW_ID_OFST: c_int = 6;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_DPCPU_FW_ID_LEN: c_int = 2;
// enum: Standard TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TXDP: c_uint = 0x0;
// enum: Low latency TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TXDP_LOW_LATENCY: c_uint = 0x1;
// enum: High packet rate TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TXDP_HIGH_PACKET_RATE: c_uint = 0x3;
// enum: Rules engine TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TXDP_RULES_ENGINE: c_uint = 0x5;
// enum: DPDK TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TXDP_DPDK: c_uint = 0x6;
// enum: BIST TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TXDP_BIST: c_uint = 0x12d;
// enum: TXDP Test firmware image 1
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TXDP_TEST_FW_TSO_EDIT: c_uint = 0x101;
// enum: TXDP Test firmware image 2
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TXDP_TEST_FW_PACKET_EDITS: c_uint = 0x102;
// enum: TXDP CSR bus test firmware
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TXDP_TEST_FW_CSR: c_uint = 0x103;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXPD_FW_VERSION_OFST: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXPD_FW_VERSION_LEN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXPD_FW_VERSION_REV_OFST: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXPD_FW_VERSION_REV_LBN: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXPD_FW_VERSION_REV_WIDTH: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXPD_FW_VERSION_TYPE_OFST: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXPD_FW_VERSION_TYPE_LBN: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXPD_FW_VERSION_TYPE_WIDTH: c_int = 4;
// enum: reserved value - do not use (may indicate alternative interpretation
// of REV field in future)
//
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXPD_FW_TYPE_RESERVED: c_uint = 0x0;
// enum: Trivial RX PD firmware for early Huntington development (Huntington
// development only)
//
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXPD_FW_TYPE_FIRST_PKT: c_uint = 0x1;
// enum: RX PD firmware for telemetry prototyping (Medford2 development only)
//
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXPD_FW_TYPE_TESTFW_TELEMETRY: c_uint = 0x1;
// enum: RX PD firmware with approximately Siena-compatible behaviour
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXPD_FW_TYPE_SIENA_COMPAT: c_uint = 0x2;
// enum: Full featured RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXPD_FW_TYPE_FULL_FEATURED: c_uint = 0x3;
// enum: (deprecated original name for the FULL_FEATURED variant)
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXPD_FW_TYPE_VSWITCH: c_uint = 0x3;
// enum: siena_compat variant RX PD firmware using PM rather than MAC
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXPD_FW_TYPE_SIENA_COMPAT_PM: c_uint = 0x4;
// enum: Low latency RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXPD_FW_TYPE_LOW_LATENCY: c_uint = 0x5;
// enum: Packed stream RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXPD_FW_TYPE_PACKED_STREAM: c_uint = 0x6;
// enum: RX PD firmware handling layer 2 only for high packet rate performance
// tests (Medford development only)
//
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXPD_FW_TYPE_LAYER2_PERF: c_uint = 0x7;
// enum: Rules engine RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXPD_FW_TYPE_RULES_ENGINE: c_uint = 0x8;
// enum: Custom firmware variant (see SF-119495-PD and bug69716)
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXPD_FW_TYPE_L3XUDP: c_uint = 0x9;
// enum: DPDK RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXPD_FW_TYPE_DPDK: c_uint = 0xa;
// enum: RX PD firmware for GUE parsing prototype (Medford development only)
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXPD_FW_TYPE_TESTFW_GUE_PROTOTYPE: c_uint = 0xe;
// enum: RX PD firmware parsing but not filtering network overlay tunnel
// encapsulations (Medford development only)
//
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXPD_FW_TYPE_TESTFW_ENCAP_PARSING_ONLY: c_uint = 0xf;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TXPD_FW_VERSION_OFST: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TXPD_FW_VERSION_LEN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TXPD_FW_VERSION_REV_OFST: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TXPD_FW_VERSION_REV_LBN: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TXPD_FW_VERSION_REV_WIDTH: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TXPD_FW_VERSION_TYPE_OFST: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TXPD_FW_VERSION_TYPE_LBN: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TXPD_FW_VERSION_TYPE_WIDTH: c_int = 4;
// enum: reserved value - do not use (may indicate alternative interpretation
// of REV field in future)
//
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TXPD_FW_TYPE_RESERVED: c_uint = 0x0;
// enum: Trivial TX PD firmware for early Huntington development (Huntington
// development only)
//
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TXPD_FW_TYPE_FIRST_PKT: c_uint = 0x1;
// enum: TX PD firmware for telemetry prototyping (Medford2 development only)
//
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TXPD_FW_TYPE_TESTFW_TELEMETRY: c_uint = 0x1;
// enum: TX PD firmware with approximately Siena-compatible behaviour
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TXPD_FW_TYPE_SIENA_COMPAT: c_uint = 0x2;
// enum: Full featured TX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TXPD_FW_TYPE_FULL_FEATURED: c_uint = 0x3;
// enum: (deprecated original name for the FULL_FEATURED variant)
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TXPD_FW_TYPE_VSWITCH: c_uint = 0x3;
// enum: siena_compat variant TX PD firmware using PM rather than MAC
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TXPD_FW_TYPE_SIENA_COMPAT_PM: c_uint = 0x4;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TXPD_FW_TYPE_LOW_LATENCY: c_uint = 0x5 /* enum */;
// enum: TX PD firmware handling layer 2 only for high packet rate performance
// tests (Medford development only)
//
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TXPD_FW_TYPE_LAYER2_PERF: c_uint = 0x7;
// enum: Rules engine TX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TXPD_FW_TYPE_RULES_ENGINE: c_uint = 0x8;
// enum: Custom firmware variant (see SF-119495-PD and bug69716)
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TXPD_FW_TYPE_L3XUDP: c_uint = 0x9;
// enum: DPDK TX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TXPD_FW_TYPE_DPDK: c_uint = 0xa;
// enum: RX PD firmware for GUE parsing prototype (Medford development only)
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TXPD_FW_TYPE_TESTFW_GUE_PROTOTYPE: c_uint = 0xe;
// Hardware capabilities of NIC
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_HW_CAPABILITIES_OFST: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_HW_CAPABILITIES_LEN: c_int = 4;
// Licensed capabilities
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_LICENSE_CAPABILITIES_OFST: c_int = 16;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_LICENSE_CAPABILITIES_LEN: c_int = 4;
// Second word of flags. Not present on older firmware (check the length).
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_FLAGS2_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_FLAGS2_LEN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_TSO_V2_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_TSO_V2_LBN: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_TSO_V2_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_TSO_V2_ENCAP_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_TSO_V2_ENCAP_LBN: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_TSO_V2_ENCAP_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_EVQ_TIMER_CTRL_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_EVQ_TIMER_CTRL_LBN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_EVQ_TIMER_CTRL_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_EVENT_CUT_THROUGH_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_EVENT_CUT_THROUGH_LBN: c_int = 3;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_EVENT_CUT_THROUGH_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_CUT_THROUGH_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_CUT_THROUGH_LBN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_CUT_THROUGH_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_VFIFO_ULL_MODE_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_VFIFO_ULL_MODE_LBN: c_int = 5;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_VFIFO_ULL_MODE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_MAC_STATS_40G_TX_SIZE_BINS_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_MAC_STATS_40G_TX_SIZE_BINS_LBN: c_int = 6;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_MAC_STATS_40G_TX_SIZE_BINS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_INIT_EVQ_TYPE_SUPPORTED_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_INIT_EVQ_TYPE_SUPPORTED_LBN: c_int = 7;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_INIT_EVQ_TYPE_SUPPORTED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_INIT_EVQ_V2_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_INIT_EVQ_V2_LBN: c_int = 7;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_INIT_EVQ_V2_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_MAC_TIMESTAMPING_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_MAC_TIMESTAMPING_LBN: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_MAC_TIMESTAMPING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_TIMESTAMP_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_TIMESTAMP_LBN: c_int = 9;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_TIMESTAMP_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_SNIFF_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_SNIFF_LBN: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_SNIFF_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_SNIFF_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_SNIFF_LBN: c_int = 11;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_SNIFF_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_NVRAM_UPDATE_REPORT_VERIFY_RESULT_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_NVRAM_UPDATE_REPORT_VERIFY_RESULT_LBN: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_NVRAM_UPDATE_REPORT_VERIFY_RESULT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_MCDI_BACKGROUND_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_MCDI_BACKGROUND_LBN: c_int = 13;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_MCDI_BACKGROUND_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_MCDI_DB_RETURN_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_MCDI_DB_RETURN_LBN: c_int = 14;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_MCDI_DB_RETURN_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_CTPIO_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_CTPIO_LBN: c_int = 15;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_CTPIO_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TSA_SUPPORT_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TSA_SUPPORT_LBN: c_int = 16;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TSA_SUPPORT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TSA_BOUND_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TSA_BOUND_LBN: c_int = 17;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TSA_BOUND_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_SF_ADAPTER_AUTHENTICATION_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_SF_ADAPTER_AUTHENTICATION_LBN: c_int = 18;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_SF_ADAPTER_AUTHENTICATION_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_FILTER_ACTION_FLAG_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_FILTER_ACTION_FLAG_LBN: c_int = 19;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_FILTER_ACTION_FLAG_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_FILTER_ACTION_MARK_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_FILTER_ACTION_MARK_LBN: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_FILTER_ACTION_MARK_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_EQUAL_STRIDE_SUPER_BUFFER_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_EQUAL_STRIDE_SUPER_BUFFER_LBN: c_int = 21;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_EQUAL_STRIDE_SUPER_BUFFER_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_EQUAL_STRIDE_PACKED_STREAM_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_EQUAL_STRIDE_PACKED_STREAM_LBN: c_int = 21;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_EQUAL_STRIDE_PACKED_STREAM_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_L3XUDP_SUPPORT_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_L3XUDP_SUPPORT_LBN: c_int = 22;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_L3XUDP_SUPPORT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_FW_SUBVARIANT_NO_TX_CSUM_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_FW_SUBVARIANT_NO_TX_CSUM_LBN: c_int = 23;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_FW_SUBVARIANT_NO_TX_CSUM_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_VI_SPREADING_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_VI_SPREADING_LBN: c_int = 24;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_VI_SPREADING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXDP_HLB_IDLE_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXDP_HLB_IDLE_LBN: c_int = 25;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RXDP_HLB_IDLE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_INIT_RXQ_NO_CONT_EV_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_INIT_RXQ_NO_CONT_EV_LBN: c_int = 26;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_INIT_RXQ_NO_CONT_EV_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_INIT_RXQ_WITH_BUFFER_SIZE_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_INIT_RXQ_WITH_BUFFER_SIZE_LBN: c_int = 27;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_INIT_RXQ_WITH_BUFFER_SIZE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_BUNDLE_UPDATE_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_BUNDLE_UPDATE_LBN: c_int = 28;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_BUNDLE_UPDATE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_TSO_V3_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_TSO_V3_LBN: c_int = 29;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_TSO_V3_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_DYNAMIC_SENSORS_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_DYNAMIC_SENSORS_LBN: c_int = 30;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_DYNAMIC_SENSORS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_NVRAM_UPDATE_POLL_VERIFY_RESULT_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_NVRAM_UPDATE_POLL_VERIFY_RESULT_LBN: c_int = 31;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_NVRAM_UPDATE_POLL_VERIFY_RESULT_WIDTH: c_int = 1;
// Number of FATSOv2 contexts per datapath supported by this NIC (when
// TX_TSO_V2 == 1). Not present on older firmware (check the length).
//
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_TSO_V2_N_CONTEXTS_OFST: c_int = 24;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_TSO_V2_N_CONTEXTS_LEN: c_int = 2;
// One byte per PF containing the number of the external port assigned to this
// PF, indexed by PF number. Special values indicate that a PF is either not
// present or not assigned.
//
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_PFS_TO_PORTS_ASSIGNMENT_OFST: c_int = 26;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_PFS_TO_PORTS_ASSIGNMENT_LEN: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_PFS_TO_PORTS_ASSIGNMENT_NUM: c_int = 16;
// enum: The caller is not permitted to access information on this PF.
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_ACCESS_NOT_PERMITTED: c_uint = 0xff;
// enum: PF does not exist.
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_PF_NOT_PRESENT: c_uint = 0xfe;
// enum: PF does exist but is not assigned to any external port.
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_PF_NOT_ASSIGNED: c_uint = 0xfd;
// enum: This value indicates that PF is assigned, but it cannot be expressed
// in this field. It is intended for a possible future situation where a more
// complex scheme of PFs to ports mapping is being used. The future driver
// should look for a new field supporting the new scheme. The current/old
// driver should treat this value as PF_NOT_ASSIGNED.
//
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_INCOMPATIBLE_ASSIGNMENT: c_uint = 0xfc;
// One byte per PF containing the number of its VFs, indexed by PF number. A
// special value indicates that a PF is not present.
//
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_NUM_VFS_PER_PF_OFST: c_int = 42;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_NUM_VFS_PER_PF_LEN: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_NUM_VFS_PER_PF_NUM: c_int = 16;
// enum: The caller is not permitted to access information on this PF.
// MC_CMD_GET_CAPABILITIES_V2_OUT_ACCESS_NOT_PERMITTED 0xff
// enum: PF does not exist.
// MC_CMD_GET_CAPABILITIES_V2_OUT_PF_NOT_PRESENT 0xfe
// Number of VIs available for each external port
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_NUM_VIS_PER_PORT_OFST: c_int = 58;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_NUM_VIS_PER_PORT_LEN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_NUM_VIS_PER_PORT_NUM: c_int = 4;
// Size of RX descriptor cache expressed as binary logarithm The actual size
// equals (2 ^ RX_DESC_CACHE_SIZE)
//
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_DESC_CACHE_SIZE_OFST: c_int = 66;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_RX_DESC_CACHE_SIZE_LEN: c_int = 1;
// Size of TX descriptor cache expressed as binary logarithm The actual size
// equals (2 ^ TX_DESC_CACHE_SIZE)
//
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_DESC_CACHE_SIZE_OFST: c_int = 67;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_TX_DESC_CACHE_SIZE_LEN: c_int = 1;
// Total number of available PIO buffers
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_NUM_PIO_BUFFS_OFST: c_int = 68;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_NUM_PIO_BUFFS_LEN: c_int = 2;
// Size of a single PIO buffer
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_SIZE_PIO_BUFF_OFST: c_int = 70;
pub const MC_CMD_GET_CAPABILITIES_V2_OUT_SIZE_PIO_BUFF_LEN: c_int = 2;
// MC_CMD_GET_CAPABILITIES_V3_OUT msgresponse
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_LEN: c_int = 76;
// First word of flags.
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_FLAGS1_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_FLAGS1_LEN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_VPORT_RECONFIGURE_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_VPORT_RECONFIGURE_LBN: c_int = 3;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_VPORT_RECONFIGURE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_STRIPING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_STRIPING_LBN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_STRIPING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_VADAPTOR_QUERY_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_VADAPTOR_QUERY_LBN: c_int = 5;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_VADAPTOR_QUERY_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_EVB_PORT_VLAN_RESTRICT_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_EVB_PORT_VLAN_RESTRICT_LBN: c_int = 6;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_EVB_PORT_VLAN_RESTRICT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_DRV_ATTACH_PREBOOT_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_DRV_ATTACH_PREBOOT_LBN: c_int = 7;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_DRV_ATTACH_PREBOOT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_FORCE_EVENT_MERGING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_FORCE_EVENT_MERGING_LBN: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_FORCE_EVENT_MERGING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_SET_MAC_ENHANCED_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_SET_MAC_ENHANCED_LBN: c_int = 9;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_SET_MAC_ENHANCED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_UNKNOWN_UCAST_DST_FILTER_ALWAYS_MULTI_RECIPIENT_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_UNKNOWN_UCAST_DST_FILTER_ALWAYS_MULTI_RECIPIENT_LBN: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_UNKNOWN_UCAST_DST_FILTER_ALWAYS_MULTI_RECIPIENT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_VADAPTOR_PERMIT_SET_MAC_WHEN_FILTERS_INSTALLED_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_VADAPTOR_PERMIT_SET_MAC_WHEN_FILTERS_INSTALLED_LBN: c_int = 11;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_VADAPTOR_PERMIT_SET_MAC_WHEN_FILTERS_INSTALLED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_MAC_SECURITY_FILTERING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_MAC_SECURITY_FILTERING_LBN: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_MAC_SECURITY_FILTERING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_ADDITIONAL_RSS_MODES_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_ADDITIONAL_RSS_MODES_LBN: c_int = 13;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_ADDITIONAL_RSS_MODES_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_QBB_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_QBB_LBN: c_int = 14;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_QBB_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_PACKED_STREAM_VAR_BUFFERS_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_PACKED_STREAM_VAR_BUFFERS_LBN: c_int = 15;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_PACKED_STREAM_VAR_BUFFERS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_RSS_LIMITED_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_RSS_LIMITED_LBN: c_int = 16;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_RSS_LIMITED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_PACKED_STREAM_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_PACKED_STREAM_LBN: c_int = 17;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_PACKED_STREAM_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_INCLUDE_FCS_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_INCLUDE_FCS_LBN: c_int = 18;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_INCLUDE_FCS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_VLAN_INSERTION_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_VLAN_INSERTION_LBN: c_int = 19;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_VLAN_INSERTION_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_VLAN_STRIPPING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_VLAN_STRIPPING_LBN: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_VLAN_STRIPPING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_TSO_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_TSO_LBN: c_int = 21;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_TSO_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_PREFIX_LEN_0_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_PREFIX_LEN_0_LBN: c_int = 22;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_PREFIX_LEN_0_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_PREFIX_LEN_14_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_PREFIX_LEN_14_LBN: c_int = 23;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_PREFIX_LEN_14_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_TIMESTAMP_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_TIMESTAMP_LBN: c_int = 24;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_TIMESTAMP_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_BATCHING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_BATCHING_LBN: c_int = 25;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_BATCHING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_MCAST_FILTER_CHAINING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_MCAST_FILTER_CHAINING_LBN: c_int = 26;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_MCAST_FILTER_CHAINING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_PM_AND_RXDP_COUNTERS_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_PM_AND_RXDP_COUNTERS_LBN: c_int = 27;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_PM_AND_RXDP_COUNTERS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_DISABLE_SCATTER_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_DISABLE_SCATTER_LBN: c_int = 28;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_DISABLE_SCATTER_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_MCAST_UDP_LOOPBACK_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_MCAST_UDP_LOOPBACK_LBN: c_int = 29;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_MCAST_UDP_LOOPBACK_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_EVB_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_EVB_LBN: c_int = 30;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_EVB_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_VXLAN_NVGRE_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_VXLAN_NVGRE_LBN: c_int = 31;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_VXLAN_NVGRE_WIDTH: c_int = 1;
// RxDPCPU firmware id.
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_DPCPU_FW_ID_OFST: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_DPCPU_FW_ID_LEN: c_int = 2;
// enum: Standard RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXDP: c_uint = 0x0;
// enum: Low latency RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXDP_LOW_LATENCY: c_uint = 0x1;
// enum: Packed stream RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXDP_PACKED_STREAM: c_uint = 0x2;
// enum: Rules engine RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXDP_RULES_ENGINE: c_uint = 0x5;
// enum: DPDK RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXDP_DPDK: c_uint = 0x6;
// enum: BIST RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXDP_BIST: c_uint = 0x10a;
// enum: RXDP Test firmware image 1
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXDP_TEST_FW_TO_MC_CUT_THROUGH: c_uint = 0x101;
// enum: RXDP Test firmware image 2
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXDP_TEST_FW_TO_MC_STORE_FORWARD: c_uint = 0x102;
// enum: RXDP Test firmware image 3
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXDP_TEST_FW_TO_MC_STORE_FORWARD_FIRST: c_uint = 0x103;
// enum: RXDP Test firmware image 4
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXDP_TEST_EVERY_EVENT_BATCHABLE: c_uint = 0x104;
// enum: RXDP Test firmware image 5
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXDP_TEST_BACKPRESSURE: c_uint = 0x105;
// enum: RXDP Test firmware image 6
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXDP_TEST_FW_PACKET_EDITS: c_uint = 0x106;
// enum: RXDP Test firmware image 7
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXDP_TEST_FW_RX_HDR_SPLIT: c_uint = 0x107;
// enum: RXDP Test firmware image 8
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXDP_TEST_FW_DISABLE_DL: c_uint = 0x108;
// enum: RXDP Test firmware image 9
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXDP_TEST_FW_DOORBELL_DELAY: c_uint = 0x10b;
// enum: RXDP Test firmware image 10
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXDP_TEST_FW_SLOW: c_uint = 0x10c;
// TxDPCPU firmware id.
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_DPCPU_FW_ID_OFST: c_int = 6;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_DPCPU_FW_ID_LEN: c_int = 2;
// enum: Standard TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TXDP: c_uint = 0x0;
// enum: Low latency TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TXDP_LOW_LATENCY: c_uint = 0x1;
// enum: High packet rate TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TXDP_HIGH_PACKET_RATE: c_uint = 0x3;
// enum: Rules engine TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TXDP_RULES_ENGINE: c_uint = 0x5;
// enum: DPDK TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TXDP_DPDK: c_uint = 0x6;
// enum: BIST TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TXDP_BIST: c_uint = 0x12d;
// enum: TXDP Test firmware image 1
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TXDP_TEST_FW_TSO_EDIT: c_uint = 0x101;
// enum: TXDP Test firmware image 2
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TXDP_TEST_FW_PACKET_EDITS: c_uint = 0x102;
// enum: TXDP CSR bus test firmware
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TXDP_TEST_FW_CSR: c_uint = 0x103;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXPD_FW_VERSION_OFST: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXPD_FW_VERSION_LEN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXPD_FW_VERSION_REV_OFST: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXPD_FW_VERSION_REV_LBN: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXPD_FW_VERSION_REV_WIDTH: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXPD_FW_VERSION_TYPE_OFST: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXPD_FW_VERSION_TYPE_LBN: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXPD_FW_VERSION_TYPE_WIDTH: c_int = 4;
// enum: reserved value - do not use (may indicate alternative interpretation
// of REV field in future)
//
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXPD_FW_TYPE_RESERVED: c_uint = 0x0;
// enum: Trivial RX PD firmware for early Huntington development (Huntington
// development only)
//
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXPD_FW_TYPE_FIRST_PKT: c_uint = 0x1;
// enum: RX PD firmware for telemetry prototyping (Medford2 development only)
//
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXPD_FW_TYPE_TESTFW_TELEMETRY: c_uint = 0x1;
// enum: RX PD firmware with approximately Siena-compatible behaviour
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXPD_FW_TYPE_SIENA_COMPAT: c_uint = 0x2;
// enum: Full featured RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXPD_FW_TYPE_FULL_FEATURED: c_uint = 0x3;
// enum: (deprecated original name for the FULL_FEATURED variant)
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXPD_FW_TYPE_VSWITCH: c_uint = 0x3;
// enum: siena_compat variant RX PD firmware using PM rather than MAC
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXPD_FW_TYPE_SIENA_COMPAT_PM: c_uint = 0x4;
// enum: Low latency RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXPD_FW_TYPE_LOW_LATENCY: c_uint = 0x5;
// enum: Packed stream RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXPD_FW_TYPE_PACKED_STREAM: c_uint = 0x6;
// enum: RX PD firmware handling layer 2 only for high packet rate performance
// tests (Medford development only)
//
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXPD_FW_TYPE_LAYER2_PERF: c_uint = 0x7;
// enum: Rules engine RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXPD_FW_TYPE_RULES_ENGINE: c_uint = 0x8;
// enum: Custom firmware variant (see SF-119495-PD and bug69716)
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXPD_FW_TYPE_L3XUDP: c_uint = 0x9;
// enum: DPDK RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXPD_FW_TYPE_DPDK: c_uint = 0xa;
// enum: RX PD firmware for GUE parsing prototype (Medford development only)
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXPD_FW_TYPE_TESTFW_GUE_PROTOTYPE: c_uint = 0xe;
// enum: RX PD firmware parsing but not filtering network overlay tunnel
// encapsulations (Medford development only)
//
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXPD_FW_TYPE_TESTFW_ENCAP_PARSING_ONLY: c_uint = 0xf;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TXPD_FW_VERSION_OFST: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TXPD_FW_VERSION_LEN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TXPD_FW_VERSION_REV_OFST: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TXPD_FW_VERSION_REV_LBN: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TXPD_FW_VERSION_REV_WIDTH: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TXPD_FW_VERSION_TYPE_OFST: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TXPD_FW_VERSION_TYPE_LBN: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TXPD_FW_VERSION_TYPE_WIDTH: c_int = 4;
// enum: reserved value - do not use (may indicate alternative interpretation
// of REV field in future)
//
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TXPD_FW_TYPE_RESERVED: c_uint = 0x0;
// enum: Trivial TX PD firmware for early Huntington development (Huntington
// development only)
//
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TXPD_FW_TYPE_FIRST_PKT: c_uint = 0x1;
// enum: TX PD firmware for telemetry prototyping (Medford2 development only)
//
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TXPD_FW_TYPE_TESTFW_TELEMETRY: c_uint = 0x1;
// enum: TX PD firmware with approximately Siena-compatible behaviour
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TXPD_FW_TYPE_SIENA_COMPAT: c_uint = 0x2;
// enum: Full featured TX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TXPD_FW_TYPE_FULL_FEATURED: c_uint = 0x3;
// enum: (deprecated original name for the FULL_FEATURED variant)
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TXPD_FW_TYPE_VSWITCH: c_uint = 0x3;
// enum: siena_compat variant TX PD firmware using PM rather than MAC
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TXPD_FW_TYPE_SIENA_COMPAT_PM: c_uint = 0x4;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TXPD_FW_TYPE_LOW_LATENCY: c_uint = 0x5 /* enum */;
// enum: TX PD firmware handling layer 2 only for high packet rate performance
// tests (Medford development only)
//
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TXPD_FW_TYPE_LAYER2_PERF: c_uint = 0x7;
// enum: Rules engine TX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TXPD_FW_TYPE_RULES_ENGINE: c_uint = 0x8;
// enum: Custom firmware variant (see SF-119495-PD and bug69716)
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TXPD_FW_TYPE_L3XUDP: c_uint = 0x9;
// enum: DPDK TX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TXPD_FW_TYPE_DPDK: c_uint = 0xa;
// enum: RX PD firmware for GUE parsing prototype (Medford development only)
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TXPD_FW_TYPE_TESTFW_GUE_PROTOTYPE: c_uint = 0xe;
// Hardware capabilities of NIC
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_HW_CAPABILITIES_OFST: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_HW_CAPABILITIES_LEN: c_int = 4;
// Licensed capabilities
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_LICENSE_CAPABILITIES_OFST: c_int = 16;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_LICENSE_CAPABILITIES_LEN: c_int = 4;
// Second word of flags. Not present on older firmware (check the length).
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_FLAGS2_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_FLAGS2_LEN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_TSO_V2_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_TSO_V2_LBN: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_TSO_V2_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_TSO_V2_ENCAP_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_TSO_V2_ENCAP_LBN: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_TSO_V2_ENCAP_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_EVQ_TIMER_CTRL_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_EVQ_TIMER_CTRL_LBN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_EVQ_TIMER_CTRL_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_EVENT_CUT_THROUGH_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_EVENT_CUT_THROUGH_LBN: c_int = 3;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_EVENT_CUT_THROUGH_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_CUT_THROUGH_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_CUT_THROUGH_LBN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_CUT_THROUGH_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_VFIFO_ULL_MODE_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_VFIFO_ULL_MODE_LBN: c_int = 5;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_VFIFO_ULL_MODE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_MAC_STATS_40G_TX_SIZE_BINS_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_MAC_STATS_40G_TX_SIZE_BINS_LBN: c_int = 6;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_MAC_STATS_40G_TX_SIZE_BINS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_INIT_EVQ_TYPE_SUPPORTED_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_INIT_EVQ_TYPE_SUPPORTED_LBN: c_int = 7;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_INIT_EVQ_TYPE_SUPPORTED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_INIT_EVQ_V2_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_INIT_EVQ_V2_LBN: c_int = 7;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_INIT_EVQ_V2_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_MAC_TIMESTAMPING_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_MAC_TIMESTAMPING_LBN: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_MAC_TIMESTAMPING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_TIMESTAMP_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_TIMESTAMP_LBN: c_int = 9;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_TIMESTAMP_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_SNIFF_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_SNIFF_LBN: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_SNIFF_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_SNIFF_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_SNIFF_LBN: c_int = 11;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_SNIFF_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_NVRAM_UPDATE_REPORT_VERIFY_RESULT_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_NVRAM_UPDATE_REPORT_VERIFY_RESULT_LBN: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_NVRAM_UPDATE_REPORT_VERIFY_RESULT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_MCDI_BACKGROUND_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_MCDI_BACKGROUND_LBN: c_int = 13;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_MCDI_BACKGROUND_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_MCDI_DB_RETURN_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_MCDI_DB_RETURN_LBN: c_int = 14;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_MCDI_DB_RETURN_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_CTPIO_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_CTPIO_LBN: c_int = 15;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_CTPIO_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TSA_SUPPORT_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TSA_SUPPORT_LBN: c_int = 16;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TSA_SUPPORT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TSA_BOUND_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TSA_BOUND_LBN: c_int = 17;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TSA_BOUND_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_SF_ADAPTER_AUTHENTICATION_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_SF_ADAPTER_AUTHENTICATION_LBN: c_int = 18;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_SF_ADAPTER_AUTHENTICATION_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_FILTER_ACTION_FLAG_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_FILTER_ACTION_FLAG_LBN: c_int = 19;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_FILTER_ACTION_FLAG_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_FILTER_ACTION_MARK_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_FILTER_ACTION_MARK_LBN: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_FILTER_ACTION_MARK_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_EQUAL_STRIDE_SUPER_BUFFER_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_EQUAL_STRIDE_SUPER_BUFFER_LBN: c_int = 21;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_EQUAL_STRIDE_SUPER_BUFFER_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_EQUAL_STRIDE_PACKED_STREAM_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_EQUAL_STRIDE_PACKED_STREAM_LBN: c_int = 21;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_EQUAL_STRIDE_PACKED_STREAM_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_L3XUDP_SUPPORT_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_L3XUDP_SUPPORT_LBN: c_int = 22;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_L3XUDP_SUPPORT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_FW_SUBVARIANT_NO_TX_CSUM_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_FW_SUBVARIANT_NO_TX_CSUM_LBN: c_int = 23;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_FW_SUBVARIANT_NO_TX_CSUM_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_VI_SPREADING_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_VI_SPREADING_LBN: c_int = 24;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_VI_SPREADING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXDP_HLB_IDLE_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXDP_HLB_IDLE_LBN: c_int = 25;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RXDP_HLB_IDLE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_INIT_RXQ_NO_CONT_EV_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_INIT_RXQ_NO_CONT_EV_LBN: c_int = 26;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_INIT_RXQ_NO_CONT_EV_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_INIT_RXQ_WITH_BUFFER_SIZE_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_INIT_RXQ_WITH_BUFFER_SIZE_LBN: c_int = 27;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_INIT_RXQ_WITH_BUFFER_SIZE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_BUNDLE_UPDATE_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_BUNDLE_UPDATE_LBN: c_int = 28;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_BUNDLE_UPDATE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_TSO_V3_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_TSO_V3_LBN: c_int = 29;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_TSO_V3_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_DYNAMIC_SENSORS_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_DYNAMIC_SENSORS_LBN: c_int = 30;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_DYNAMIC_SENSORS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_NVRAM_UPDATE_POLL_VERIFY_RESULT_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_NVRAM_UPDATE_POLL_VERIFY_RESULT_LBN: c_int = 31;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_NVRAM_UPDATE_POLL_VERIFY_RESULT_WIDTH: c_int = 1;
// Number of FATSOv2 contexts per datapath supported by this NIC (when
// TX_TSO_V2 == 1). Not present on older firmware (check the length).
//
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_TSO_V2_N_CONTEXTS_OFST: c_int = 24;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_TSO_V2_N_CONTEXTS_LEN: c_int = 2;
// One byte per PF containing the number of the external port assigned to this
// PF, indexed by PF number. Special values indicate that a PF is either not
// present or not assigned.
//
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_PFS_TO_PORTS_ASSIGNMENT_OFST: c_int = 26;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_PFS_TO_PORTS_ASSIGNMENT_LEN: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_PFS_TO_PORTS_ASSIGNMENT_NUM: c_int = 16;
// enum: The caller is not permitted to access information on this PF.
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_ACCESS_NOT_PERMITTED: c_uint = 0xff;
// enum: PF does not exist.
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_PF_NOT_PRESENT: c_uint = 0xfe;
// enum: PF does exist but is not assigned to any external port.
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_PF_NOT_ASSIGNED: c_uint = 0xfd;
// enum: This value indicates that PF is assigned, but it cannot be expressed
// in this field. It is intended for a possible future situation where a more
// complex scheme of PFs to ports mapping is being used. The future driver
// should look for a new field supporting the new scheme. The current/old
// driver should treat this value as PF_NOT_ASSIGNED.
//
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_INCOMPATIBLE_ASSIGNMENT: c_uint = 0xfc;
// One byte per PF containing the number of its VFs, indexed by PF number. A
// special value indicates that a PF is not present.
//
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_NUM_VFS_PER_PF_OFST: c_int = 42;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_NUM_VFS_PER_PF_LEN: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_NUM_VFS_PER_PF_NUM: c_int = 16;
// enum: The caller is not permitted to access information on this PF.
// MC_CMD_GET_CAPABILITIES_V3_OUT_ACCESS_NOT_PERMITTED 0xff
// enum: PF does not exist.
// MC_CMD_GET_CAPABILITIES_V3_OUT_PF_NOT_PRESENT 0xfe
// Number of VIs available for each external port
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_NUM_VIS_PER_PORT_OFST: c_int = 58;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_NUM_VIS_PER_PORT_LEN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_NUM_VIS_PER_PORT_NUM: c_int = 4;
// Size of RX descriptor cache expressed as binary logarithm The actual size
// equals (2 ^ RX_DESC_CACHE_SIZE)
//
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_DESC_CACHE_SIZE_OFST: c_int = 66;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_RX_DESC_CACHE_SIZE_LEN: c_int = 1;
// Size of TX descriptor cache expressed as binary logarithm The actual size
// equals (2 ^ TX_DESC_CACHE_SIZE)
//
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_DESC_CACHE_SIZE_OFST: c_int = 67;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_TX_DESC_CACHE_SIZE_LEN: c_int = 1;
// Total number of available PIO buffers
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_NUM_PIO_BUFFS_OFST: c_int = 68;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_NUM_PIO_BUFFS_LEN: c_int = 2;
// Size of a single PIO buffer
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_SIZE_PIO_BUFF_OFST: c_int = 70;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_SIZE_PIO_BUFF_LEN: c_int = 2;
// On chips later than Medford the amount of address space assigned to each VI
// is configurable. This is a global setting that the driver must query to
// discover the VI to address mapping. Cut-through PIO (CTPIO) is not available
// with 8k VI windows.
//
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_VI_WINDOW_MODE_OFST: c_int = 72;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_VI_WINDOW_MODE_LEN: c_int = 1;
// enum: Each VI occupies 8k as on Huntington and Medford. PIO is at offset 4k.
// CTPIO is not mapped.
//
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_VI_WINDOW_MODE_8K: c_uint = 0x0;
// enum: Each VI occupies 16k. PIO is at offset 4k. CTPIO is at offset 12k.
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_VI_WINDOW_MODE_16K: c_uint = 0x1;
// enum: Each VI occupies 64k. PIO is at offset 4k. CTPIO is at offset 12k.
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_VI_WINDOW_MODE_64K: c_uint = 0x2;
// Number of vFIFOs per adapter that can be used for VFIFO Stuffing
// (SF-115995-SW) in the present configuration of firmware and port mode.
//
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_VFIFO_STUFFING_NUM_VFIFOS_OFST: c_int = 73;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_VFIFO_STUFFING_NUM_VFIFOS_LEN: c_int = 1;
// Number of buffers per adapter that can be used for VFIFO Stuffing
// (SF-115995-SW) in the present configuration of firmware and port mode.
//
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_VFIFO_STUFFING_NUM_CP_BUFFERS_OFST: c_int = 74;
pub const MC_CMD_GET_CAPABILITIES_V3_OUT_VFIFO_STUFFING_NUM_CP_BUFFERS_LEN: c_int = 2;
// MC_CMD_GET_CAPABILITIES_V4_OUT msgresponse
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_LEN: c_int = 78;
// First word of flags.
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_FLAGS1_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_FLAGS1_LEN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_VPORT_RECONFIGURE_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_VPORT_RECONFIGURE_LBN: c_int = 3;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_VPORT_RECONFIGURE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_STRIPING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_STRIPING_LBN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_STRIPING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_VADAPTOR_QUERY_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_VADAPTOR_QUERY_LBN: c_int = 5;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_VADAPTOR_QUERY_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_EVB_PORT_VLAN_RESTRICT_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_EVB_PORT_VLAN_RESTRICT_LBN: c_int = 6;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_EVB_PORT_VLAN_RESTRICT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_DRV_ATTACH_PREBOOT_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_DRV_ATTACH_PREBOOT_LBN: c_int = 7;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_DRV_ATTACH_PREBOOT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_FORCE_EVENT_MERGING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_FORCE_EVENT_MERGING_LBN: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_FORCE_EVENT_MERGING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_SET_MAC_ENHANCED_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_SET_MAC_ENHANCED_LBN: c_int = 9;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_SET_MAC_ENHANCED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_UNKNOWN_UCAST_DST_FILTER_ALWAYS_MULTI_RECIPIENT_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_UNKNOWN_UCAST_DST_FILTER_ALWAYS_MULTI_RECIPIENT_LBN: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_UNKNOWN_UCAST_DST_FILTER_ALWAYS_MULTI_RECIPIENT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_VADAPTOR_PERMIT_SET_MAC_WHEN_FILTERS_INSTALLED_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_VADAPTOR_PERMIT_SET_MAC_WHEN_FILTERS_INSTALLED_LBN: c_int = 11;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_VADAPTOR_PERMIT_SET_MAC_WHEN_FILTERS_INSTALLED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_MAC_SECURITY_FILTERING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_MAC_SECURITY_FILTERING_LBN: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_MAC_SECURITY_FILTERING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_ADDITIONAL_RSS_MODES_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_ADDITIONAL_RSS_MODES_LBN: c_int = 13;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_ADDITIONAL_RSS_MODES_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_QBB_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_QBB_LBN: c_int = 14;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_QBB_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_PACKED_STREAM_VAR_BUFFERS_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_PACKED_STREAM_VAR_BUFFERS_LBN: c_int = 15;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_PACKED_STREAM_VAR_BUFFERS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_RSS_LIMITED_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_RSS_LIMITED_LBN: c_int = 16;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_RSS_LIMITED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_PACKED_STREAM_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_PACKED_STREAM_LBN: c_int = 17;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_PACKED_STREAM_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_INCLUDE_FCS_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_INCLUDE_FCS_LBN: c_int = 18;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_INCLUDE_FCS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_VLAN_INSERTION_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_VLAN_INSERTION_LBN: c_int = 19;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_VLAN_INSERTION_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_VLAN_STRIPPING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_VLAN_STRIPPING_LBN: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_VLAN_STRIPPING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_TSO_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_TSO_LBN: c_int = 21;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_TSO_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_PREFIX_LEN_0_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_PREFIX_LEN_0_LBN: c_int = 22;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_PREFIX_LEN_0_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_PREFIX_LEN_14_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_PREFIX_LEN_14_LBN: c_int = 23;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_PREFIX_LEN_14_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_TIMESTAMP_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_TIMESTAMP_LBN: c_int = 24;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_TIMESTAMP_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_BATCHING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_BATCHING_LBN: c_int = 25;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_BATCHING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_MCAST_FILTER_CHAINING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_MCAST_FILTER_CHAINING_LBN: c_int = 26;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_MCAST_FILTER_CHAINING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_PM_AND_RXDP_COUNTERS_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_PM_AND_RXDP_COUNTERS_LBN: c_int = 27;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_PM_AND_RXDP_COUNTERS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_DISABLE_SCATTER_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_DISABLE_SCATTER_LBN: c_int = 28;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_DISABLE_SCATTER_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_MCAST_UDP_LOOPBACK_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_MCAST_UDP_LOOPBACK_LBN: c_int = 29;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_MCAST_UDP_LOOPBACK_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_EVB_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_EVB_LBN: c_int = 30;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_EVB_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_VXLAN_NVGRE_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_VXLAN_NVGRE_LBN: c_int = 31;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_VXLAN_NVGRE_WIDTH: c_int = 1;
// RxDPCPU firmware id.
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_DPCPU_FW_ID_OFST: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_DPCPU_FW_ID_LEN: c_int = 2;
// enum: Standard RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXDP: c_uint = 0x0;
// enum: Low latency RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXDP_LOW_LATENCY: c_uint = 0x1;
// enum: Packed stream RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXDP_PACKED_STREAM: c_uint = 0x2;
// enum: Rules engine RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXDP_RULES_ENGINE: c_uint = 0x5;
// enum: DPDK RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXDP_DPDK: c_uint = 0x6;
// enum: BIST RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXDP_BIST: c_uint = 0x10a;
// enum: RXDP Test firmware image 1
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXDP_TEST_FW_TO_MC_CUT_THROUGH: c_uint = 0x101;
// enum: RXDP Test firmware image 2
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXDP_TEST_FW_TO_MC_STORE_FORWARD: c_uint = 0x102;
// enum: RXDP Test firmware image 3
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXDP_TEST_FW_TO_MC_STORE_FORWARD_FIRST: c_uint = 0x103;
// enum: RXDP Test firmware image 4
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXDP_TEST_EVERY_EVENT_BATCHABLE: c_uint = 0x104;
// enum: RXDP Test firmware image 5
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXDP_TEST_BACKPRESSURE: c_uint = 0x105;
// enum: RXDP Test firmware image 6
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXDP_TEST_FW_PACKET_EDITS: c_uint = 0x106;
// enum: RXDP Test firmware image 7
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXDP_TEST_FW_RX_HDR_SPLIT: c_uint = 0x107;
// enum: RXDP Test firmware image 8
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXDP_TEST_FW_DISABLE_DL: c_uint = 0x108;
// enum: RXDP Test firmware image 9
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXDP_TEST_FW_DOORBELL_DELAY: c_uint = 0x10b;
// enum: RXDP Test firmware image 10
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXDP_TEST_FW_SLOW: c_uint = 0x10c;
// TxDPCPU firmware id.
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_DPCPU_FW_ID_OFST: c_int = 6;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_DPCPU_FW_ID_LEN: c_int = 2;
// enum: Standard TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TXDP: c_uint = 0x0;
// enum: Low latency TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TXDP_LOW_LATENCY: c_uint = 0x1;
// enum: High packet rate TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TXDP_HIGH_PACKET_RATE: c_uint = 0x3;
// enum: Rules engine TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TXDP_RULES_ENGINE: c_uint = 0x5;
// enum: DPDK TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TXDP_DPDK: c_uint = 0x6;
// enum: BIST TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TXDP_BIST: c_uint = 0x12d;
// enum: TXDP Test firmware image 1
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TXDP_TEST_FW_TSO_EDIT: c_uint = 0x101;
// enum: TXDP Test firmware image 2
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TXDP_TEST_FW_PACKET_EDITS: c_uint = 0x102;
// enum: TXDP CSR bus test firmware
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TXDP_TEST_FW_CSR: c_uint = 0x103;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXPD_FW_VERSION_OFST: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXPD_FW_VERSION_LEN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXPD_FW_VERSION_REV_OFST: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXPD_FW_VERSION_REV_LBN: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXPD_FW_VERSION_REV_WIDTH: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXPD_FW_VERSION_TYPE_OFST: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXPD_FW_VERSION_TYPE_LBN: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXPD_FW_VERSION_TYPE_WIDTH: c_int = 4;
// enum: reserved value - do not use (may indicate alternative interpretation
// of REV field in future)
//
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXPD_FW_TYPE_RESERVED: c_uint = 0x0;
// enum: Trivial RX PD firmware for early Huntington development (Huntington
// development only)
//
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXPD_FW_TYPE_FIRST_PKT: c_uint = 0x1;
// enum: RX PD firmware for telemetry prototyping (Medford2 development only)
//
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXPD_FW_TYPE_TESTFW_TELEMETRY: c_uint = 0x1;
// enum: RX PD firmware with approximately Siena-compatible behaviour
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXPD_FW_TYPE_SIENA_COMPAT: c_uint = 0x2;
// enum: Full featured RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXPD_FW_TYPE_FULL_FEATURED: c_uint = 0x3;
// enum: (deprecated original name for the FULL_FEATURED variant)
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXPD_FW_TYPE_VSWITCH: c_uint = 0x3;
// enum: siena_compat variant RX PD firmware using PM rather than MAC
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXPD_FW_TYPE_SIENA_COMPAT_PM: c_uint = 0x4;
// enum: Low latency RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXPD_FW_TYPE_LOW_LATENCY: c_uint = 0x5;
// enum: Packed stream RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXPD_FW_TYPE_PACKED_STREAM: c_uint = 0x6;
// enum: RX PD firmware handling layer 2 only for high packet rate performance
// tests (Medford development only)
//
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXPD_FW_TYPE_LAYER2_PERF: c_uint = 0x7;
// enum: Rules engine RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXPD_FW_TYPE_RULES_ENGINE: c_uint = 0x8;
// enum: Custom firmware variant (see SF-119495-PD and bug69716)
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXPD_FW_TYPE_L3XUDP: c_uint = 0x9;
// enum: DPDK RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXPD_FW_TYPE_DPDK: c_uint = 0xa;
// enum: RX PD firmware for GUE parsing prototype (Medford development only)
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXPD_FW_TYPE_TESTFW_GUE_PROTOTYPE: c_uint = 0xe;
// enum: RX PD firmware parsing but not filtering network overlay tunnel
// encapsulations (Medford development only)
//
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXPD_FW_TYPE_TESTFW_ENCAP_PARSING_ONLY: c_uint = 0xf;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TXPD_FW_VERSION_OFST: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TXPD_FW_VERSION_LEN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TXPD_FW_VERSION_REV_OFST: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TXPD_FW_VERSION_REV_LBN: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TXPD_FW_VERSION_REV_WIDTH: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TXPD_FW_VERSION_TYPE_OFST: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TXPD_FW_VERSION_TYPE_LBN: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TXPD_FW_VERSION_TYPE_WIDTH: c_int = 4;
// enum: reserved value - do not use (may indicate alternative interpretation
// of REV field in future)
//
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TXPD_FW_TYPE_RESERVED: c_uint = 0x0;
// enum: Trivial TX PD firmware for early Huntington development (Huntington
// development only)
//
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TXPD_FW_TYPE_FIRST_PKT: c_uint = 0x1;
// enum: TX PD firmware for telemetry prototyping (Medford2 development only)
//
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TXPD_FW_TYPE_TESTFW_TELEMETRY: c_uint = 0x1;
// enum: TX PD firmware with approximately Siena-compatible behaviour
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TXPD_FW_TYPE_SIENA_COMPAT: c_uint = 0x2;
// enum: Full featured TX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TXPD_FW_TYPE_FULL_FEATURED: c_uint = 0x3;
// enum: (deprecated original name for the FULL_FEATURED variant)
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TXPD_FW_TYPE_VSWITCH: c_uint = 0x3;
// enum: siena_compat variant TX PD firmware using PM rather than MAC
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TXPD_FW_TYPE_SIENA_COMPAT_PM: c_uint = 0x4;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TXPD_FW_TYPE_LOW_LATENCY: c_uint = 0x5 /* enum */;
// enum: TX PD firmware handling layer 2 only for high packet rate performance
// tests (Medford development only)
//
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TXPD_FW_TYPE_LAYER2_PERF: c_uint = 0x7;
// enum: Rules engine TX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TXPD_FW_TYPE_RULES_ENGINE: c_uint = 0x8;
// enum: Custom firmware variant (see SF-119495-PD and bug69716)
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TXPD_FW_TYPE_L3XUDP: c_uint = 0x9;
// enum: DPDK TX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TXPD_FW_TYPE_DPDK: c_uint = 0xa;
// enum: RX PD firmware for GUE parsing prototype (Medford development only)
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TXPD_FW_TYPE_TESTFW_GUE_PROTOTYPE: c_uint = 0xe;
// Hardware capabilities of NIC
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_HW_CAPABILITIES_OFST: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_HW_CAPABILITIES_LEN: c_int = 4;
// Licensed capabilities
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_LICENSE_CAPABILITIES_OFST: c_int = 16;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_LICENSE_CAPABILITIES_LEN: c_int = 4;
// Second word of flags. Not present on older firmware (check the length).
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_FLAGS2_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_FLAGS2_LEN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_TSO_V2_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_TSO_V2_LBN: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_TSO_V2_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_TSO_V2_ENCAP_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_TSO_V2_ENCAP_LBN: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_TSO_V2_ENCAP_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_EVQ_TIMER_CTRL_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_EVQ_TIMER_CTRL_LBN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_EVQ_TIMER_CTRL_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_EVENT_CUT_THROUGH_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_EVENT_CUT_THROUGH_LBN: c_int = 3;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_EVENT_CUT_THROUGH_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_CUT_THROUGH_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_CUT_THROUGH_LBN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_CUT_THROUGH_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_VFIFO_ULL_MODE_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_VFIFO_ULL_MODE_LBN: c_int = 5;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_VFIFO_ULL_MODE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_MAC_STATS_40G_TX_SIZE_BINS_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_MAC_STATS_40G_TX_SIZE_BINS_LBN: c_int = 6;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_MAC_STATS_40G_TX_SIZE_BINS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_INIT_EVQ_TYPE_SUPPORTED_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_INIT_EVQ_TYPE_SUPPORTED_LBN: c_int = 7;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_INIT_EVQ_TYPE_SUPPORTED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_INIT_EVQ_V2_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_INIT_EVQ_V2_LBN: c_int = 7;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_INIT_EVQ_V2_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_MAC_TIMESTAMPING_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_MAC_TIMESTAMPING_LBN: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_MAC_TIMESTAMPING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_TIMESTAMP_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_TIMESTAMP_LBN: c_int = 9;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_TIMESTAMP_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_SNIFF_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_SNIFF_LBN: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_SNIFF_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_SNIFF_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_SNIFF_LBN: c_int = 11;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_SNIFF_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_NVRAM_UPDATE_REPORT_VERIFY_RESULT_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_NVRAM_UPDATE_REPORT_VERIFY_RESULT_LBN: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_NVRAM_UPDATE_REPORT_VERIFY_RESULT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_MCDI_BACKGROUND_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_MCDI_BACKGROUND_LBN: c_int = 13;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_MCDI_BACKGROUND_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_MCDI_DB_RETURN_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_MCDI_DB_RETURN_LBN: c_int = 14;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_MCDI_DB_RETURN_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_CTPIO_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_CTPIO_LBN: c_int = 15;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_CTPIO_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TSA_SUPPORT_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TSA_SUPPORT_LBN: c_int = 16;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TSA_SUPPORT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TSA_BOUND_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TSA_BOUND_LBN: c_int = 17;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TSA_BOUND_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_SF_ADAPTER_AUTHENTICATION_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_SF_ADAPTER_AUTHENTICATION_LBN: c_int = 18;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_SF_ADAPTER_AUTHENTICATION_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_FILTER_ACTION_FLAG_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_FILTER_ACTION_FLAG_LBN: c_int = 19;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_FILTER_ACTION_FLAG_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_FILTER_ACTION_MARK_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_FILTER_ACTION_MARK_LBN: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_FILTER_ACTION_MARK_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_EQUAL_STRIDE_SUPER_BUFFER_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_EQUAL_STRIDE_SUPER_BUFFER_LBN: c_int = 21;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_EQUAL_STRIDE_SUPER_BUFFER_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_EQUAL_STRIDE_PACKED_STREAM_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_EQUAL_STRIDE_PACKED_STREAM_LBN: c_int = 21;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_EQUAL_STRIDE_PACKED_STREAM_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_L3XUDP_SUPPORT_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_L3XUDP_SUPPORT_LBN: c_int = 22;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_L3XUDP_SUPPORT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_FW_SUBVARIANT_NO_TX_CSUM_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_FW_SUBVARIANT_NO_TX_CSUM_LBN: c_int = 23;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_FW_SUBVARIANT_NO_TX_CSUM_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_VI_SPREADING_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_VI_SPREADING_LBN: c_int = 24;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_VI_SPREADING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXDP_HLB_IDLE_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXDP_HLB_IDLE_LBN: c_int = 25;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RXDP_HLB_IDLE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_INIT_RXQ_NO_CONT_EV_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_INIT_RXQ_NO_CONT_EV_LBN: c_int = 26;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_INIT_RXQ_NO_CONT_EV_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_INIT_RXQ_WITH_BUFFER_SIZE_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_INIT_RXQ_WITH_BUFFER_SIZE_LBN: c_int = 27;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_INIT_RXQ_WITH_BUFFER_SIZE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_BUNDLE_UPDATE_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_BUNDLE_UPDATE_LBN: c_int = 28;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_BUNDLE_UPDATE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_TSO_V3_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_TSO_V3_LBN: c_int = 29;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_TSO_V3_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_DYNAMIC_SENSORS_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_DYNAMIC_SENSORS_LBN: c_int = 30;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_DYNAMIC_SENSORS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_NVRAM_UPDATE_POLL_VERIFY_RESULT_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_NVRAM_UPDATE_POLL_VERIFY_RESULT_LBN: c_int = 31;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_NVRAM_UPDATE_POLL_VERIFY_RESULT_WIDTH: c_int = 1;
// Number of FATSOv2 contexts per datapath supported by this NIC (when
// TX_TSO_V2 == 1). Not present on older firmware (check the length).
//
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_TSO_V2_N_CONTEXTS_OFST: c_int = 24;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_TSO_V2_N_CONTEXTS_LEN: c_int = 2;
// One byte per PF containing the number of the external port assigned to this
// PF, indexed by PF number. Special values indicate that a PF is either not
// present or not assigned.
//
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_PFS_TO_PORTS_ASSIGNMENT_OFST: c_int = 26;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_PFS_TO_PORTS_ASSIGNMENT_LEN: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_PFS_TO_PORTS_ASSIGNMENT_NUM: c_int = 16;
// enum: The caller is not permitted to access information on this PF.
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_ACCESS_NOT_PERMITTED: c_uint = 0xff;
// enum: PF does not exist.
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_PF_NOT_PRESENT: c_uint = 0xfe;
// enum: PF does exist but is not assigned to any external port.
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_PF_NOT_ASSIGNED: c_uint = 0xfd;
// enum: This value indicates that PF is assigned, but it cannot be expressed
// in this field. It is intended for a possible future situation where a more
// complex scheme of PFs to ports mapping is being used. The future driver
// should look for a new field supporting the new scheme. The current/old
// driver should treat this value as PF_NOT_ASSIGNED.
//
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_INCOMPATIBLE_ASSIGNMENT: c_uint = 0xfc;
// One byte per PF containing the number of its VFs, indexed by PF number. A
// special value indicates that a PF is not present.
//
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_NUM_VFS_PER_PF_OFST: c_int = 42;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_NUM_VFS_PER_PF_LEN: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_NUM_VFS_PER_PF_NUM: c_int = 16;
// enum: The caller is not permitted to access information on this PF.
// MC_CMD_GET_CAPABILITIES_V4_OUT_ACCESS_NOT_PERMITTED 0xff
// enum: PF does not exist.
// MC_CMD_GET_CAPABILITIES_V4_OUT_PF_NOT_PRESENT 0xfe
// Number of VIs available for each external port
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_NUM_VIS_PER_PORT_OFST: c_int = 58;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_NUM_VIS_PER_PORT_LEN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_NUM_VIS_PER_PORT_NUM: c_int = 4;
// Size of RX descriptor cache expressed as binary logarithm The actual size
// equals (2 ^ RX_DESC_CACHE_SIZE)
//
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_DESC_CACHE_SIZE_OFST: c_int = 66;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_RX_DESC_CACHE_SIZE_LEN: c_int = 1;
// Size of TX descriptor cache expressed as binary logarithm The actual size
// equals (2 ^ TX_DESC_CACHE_SIZE)
//
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_DESC_CACHE_SIZE_OFST: c_int = 67;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_TX_DESC_CACHE_SIZE_LEN: c_int = 1;
// Total number of available PIO buffers
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_NUM_PIO_BUFFS_OFST: c_int = 68;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_NUM_PIO_BUFFS_LEN: c_int = 2;
// Size of a single PIO buffer
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_SIZE_PIO_BUFF_OFST: c_int = 70;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_SIZE_PIO_BUFF_LEN: c_int = 2;
// On chips later than Medford the amount of address space assigned to each VI
// is configurable. This is a global setting that the driver must query to
// discover the VI to address mapping. Cut-through PIO (CTPIO) is not available
// with 8k VI windows.
//
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_VI_WINDOW_MODE_OFST: c_int = 72;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_VI_WINDOW_MODE_LEN: c_int = 1;
// enum: Each VI occupies 8k as on Huntington and Medford. PIO is at offset 4k.
// CTPIO is not mapped.
//
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_VI_WINDOW_MODE_8K: c_uint = 0x0;
// enum: Each VI occupies 16k. PIO is at offset 4k. CTPIO is at offset 12k.
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_VI_WINDOW_MODE_16K: c_uint = 0x1;
// enum: Each VI occupies 64k. PIO is at offset 4k. CTPIO is at offset 12k.
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_VI_WINDOW_MODE_64K: c_uint = 0x2;
// Number of vFIFOs per adapter that can be used for VFIFO Stuffing
// (SF-115995-SW) in the present configuration of firmware and port mode.
//
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_VFIFO_STUFFING_NUM_VFIFOS_OFST: c_int = 73;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_VFIFO_STUFFING_NUM_VFIFOS_LEN: c_int = 1;
// Number of buffers per adapter that can be used for VFIFO Stuffing
// (SF-115995-SW) in the present configuration of firmware and port mode.
//
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_VFIFO_STUFFING_NUM_CP_BUFFERS_OFST: c_int = 74;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_VFIFO_STUFFING_NUM_CP_BUFFERS_LEN: c_int = 2;
// Entry count in the MAC stats array, including the final GENERATION_END
// entry. For MAC stats DMA, drivers should allocate a buffer large enough to
// hold at least this many 64-bit stats values, if they wish to receive all
// available stats. If the buffer is shorter than MAC_STATS_NUM_STATS * 8, the
// stats array returned will be truncated.
//
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_MAC_STATS_NUM_STATS_OFST: c_int = 76;
pub const MC_CMD_GET_CAPABILITIES_V4_OUT_MAC_STATS_NUM_STATS_LEN: c_int = 2;
// MC_CMD_GET_CAPABILITIES_V5_OUT msgresponse
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_LEN: c_int = 84;
// First word of flags.
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_FLAGS1_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_FLAGS1_LEN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_VPORT_RECONFIGURE_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_VPORT_RECONFIGURE_LBN: c_int = 3;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_VPORT_RECONFIGURE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_STRIPING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_STRIPING_LBN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_STRIPING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_VADAPTOR_QUERY_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_VADAPTOR_QUERY_LBN: c_int = 5;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_VADAPTOR_QUERY_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_EVB_PORT_VLAN_RESTRICT_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_EVB_PORT_VLAN_RESTRICT_LBN: c_int = 6;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_EVB_PORT_VLAN_RESTRICT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_DRV_ATTACH_PREBOOT_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_DRV_ATTACH_PREBOOT_LBN: c_int = 7;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_DRV_ATTACH_PREBOOT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_FORCE_EVENT_MERGING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_FORCE_EVENT_MERGING_LBN: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_FORCE_EVENT_MERGING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_SET_MAC_ENHANCED_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_SET_MAC_ENHANCED_LBN: c_int = 9;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_SET_MAC_ENHANCED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_UNKNOWN_UCAST_DST_FILTER_ALWAYS_MULTI_RECIPIENT_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_UNKNOWN_UCAST_DST_FILTER_ALWAYS_MULTI_RECIPIENT_LBN: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_UNKNOWN_UCAST_DST_FILTER_ALWAYS_MULTI_RECIPIENT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_VADAPTOR_PERMIT_SET_MAC_WHEN_FILTERS_INSTALLED_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_VADAPTOR_PERMIT_SET_MAC_WHEN_FILTERS_INSTALLED_LBN: c_int = 11;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_VADAPTOR_PERMIT_SET_MAC_WHEN_FILTERS_INSTALLED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_MAC_SECURITY_FILTERING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_MAC_SECURITY_FILTERING_LBN: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_MAC_SECURITY_FILTERING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_ADDITIONAL_RSS_MODES_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_ADDITIONAL_RSS_MODES_LBN: c_int = 13;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_ADDITIONAL_RSS_MODES_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_QBB_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_QBB_LBN: c_int = 14;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_QBB_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_PACKED_STREAM_VAR_BUFFERS_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_PACKED_STREAM_VAR_BUFFERS_LBN: c_int = 15;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_PACKED_STREAM_VAR_BUFFERS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_RSS_LIMITED_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_RSS_LIMITED_LBN: c_int = 16;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_RSS_LIMITED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_PACKED_STREAM_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_PACKED_STREAM_LBN: c_int = 17;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_PACKED_STREAM_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_INCLUDE_FCS_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_INCLUDE_FCS_LBN: c_int = 18;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_INCLUDE_FCS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_VLAN_INSERTION_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_VLAN_INSERTION_LBN: c_int = 19;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_VLAN_INSERTION_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_VLAN_STRIPPING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_VLAN_STRIPPING_LBN: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_VLAN_STRIPPING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_TSO_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_TSO_LBN: c_int = 21;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_TSO_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_PREFIX_LEN_0_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_PREFIX_LEN_0_LBN: c_int = 22;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_PREFIX_LEN_0_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_PREFIX_LEN_14_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_PREFIX_LEN_14_LBN: c_int = 23;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_PREFIX_LEN_14_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_TIMESTAMP_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_TIMESTAMP_LBN: c_int = 24;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_TIMESTAMP_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_BATCHING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_BATCHING_LBN: c_int = 25;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_BATCHING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_MCAST_FILTER_CHAINING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_MCAST_FILTER_CHAINING_LBN: c_int = 26;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_MCAST_FILTER_CHAINING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_PM_AND_RXDP_COUNTERS_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_PM_AND_RXDP_COUNTERS_LBN: c_int = 27;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_PM_AND_RXDP_COUNTERS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_DISABLE_SCATTER_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_DISABLE_SCATTER_LBN: c_int = 28;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_DISABLE_SCATTER_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_MCAST_UDP_LOOPBACK_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_MCAST_UDP_LOOPBACK_LBN: c_int = 29;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_MCAST_UDP_LOOPBACK_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_EVB_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_EVB_LBN: c_int = 30;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_EVB_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_VXLAN_NVGRE_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_VXLAN_NVGRE_LBN: c_int = 31;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_VXLAN_NVGRE_WIDTH: c_int = 1;
// RxDPCPU firmware id.
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_DPCPU_FW_ID_OFST: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_DPCPU_FW_ID_LEN: c_int = 2;
// enum: Standard RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXDP: c_uint = 0x0;
// enum: Low latency RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXDP_LOW_LATENCY: c_uint = 0x1;
// enum: Packed stream RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXDP_PACKED_STREAM: c_uint = 0x2;
// enum: Rules engine RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXDP_RULES_ENGINE: c_uint = 0x5;
// enum: DPDK RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXDP_DPDK: c_uint = 0x6;
// enum: BIST RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXDP_BIST: c_uint = 0x10a;
// enum: RXDP Test firmware image 1
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXDP_TEST_FW_TO_MC_CUT_THROUGH: c_uint = 0x101;
// enum: RXDP Test firmware image 2
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXDP_TEST_FW_TO_MC_STORE_FORWARD: c_uint = 0x102;
// enum: RXDP Test firmware image 3
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXDP_TEST_FW_TO_MC_STORE_FORWARD_FIRST: c_uint = 0x103;
// enum: RXDP Test firmware image 4
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXDP_TEST_EVERY_EVENT_BATCHABLE: c_uint = 0x104;
// enum: RXDP Test firmware image 5
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXDP_TEST_BACKPRESSURE: c_uint = 0x105;
// enum: RXDP Test firmware image 6
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXDP_TEST_FW_PACKET_EDITS: c_uint = 0x106;
// enum: RXDP Test firmware image 7
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXDP_TEST_FW_RX_HDR_SPLIT: c_uint = 0x107;
// enum: RXDP Test firmware image 8
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXDP_TEST_FW_DISABLE_DL: c_uint = 0x108;
// enum: RXDP Test firmware image 9
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXDP_TEST_FW_DOORBELL_DELAY: c_uint = 0x10b;
// enum: RXDP Test firmware image 10
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXDP_TEST_FW_SLOW: c_uint = 0x10c;
// TxDPCPU firmware id.
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_DPCPU_FW_ID_OFST: c_int = 6;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_DPCPU_FW_ID_LEN: c_int = 2;
// enum: Standard TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TXDP: c_uint = 0x0;
// enum: Low latency TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TXDP_LOW_LATENCY: c_uint = 0x1;
// enum: High packet rate TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TXDP_HIGH_PACKET_RATE: c_uint = 0x3;
// enum: Rules engine TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TXDP_RULES_ENGINE: c_uint = 0x5;
// enum: DPDK TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TXDP_DPDK: c_uint = 0x6;
// enum: BIST TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TXDP_BIST: c_uint = 0x12d;
// enum: TXDP Test firmware image 1
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TXDP_TEST_FW_TSO_EDIT: c_uint = 0x101;
// enum: TXDP Test firmware image 2
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TXDP_TEST_FW_PACKET_EDITS: c_uint = 0x102;
// enum: TXDP CSR bus test firmware
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TXDP_TEST_FW_CSR: c_uint = 0x103;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXPD_FW_VERSION_OFST: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXPD_FW_VERSION_LEN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXPD_FW_VERSION_REV_OFST: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXPD_FW_VERSION_REV_LBN: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXPD_FW_VERSION_REV_WIDTH: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXPD_FW_VERSION_TYPE_OFST: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXPD_FW_VERSION_TYPE_LBN: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXPD_FW_VERSION_TYPE_WIDTH: c_int = 4;
// enum: reserved value - do not use (may indicate alternative interpretation
// of REV field in future)
//
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXPD_FW_TYPE_RESERVED: c_uint = 0x0;
// enum: Trivial RX PD firmware for early Huntington development (Huntington
// development only)
//
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXPD_FW_TYPE_FIRST_PKT: c_uint = 0x1;
// enum: RX PD firmware for telemetry prototyping (Medford2 development only)
//
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXPD_FW_TYPE_TESTFW_TELEMETRY: c_uint = 0x1;
// enum: RX PD firmware with approximately Siena-compatible behaviour
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXPD_FW_TYPE_SIENA_COMPAT: c_uint = 0x2;
// enum: Full featured RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXPD_FW_TYPE_FULL_FEATURED: c_uint = 0x3;
// enum: (deprecated original name for the FULL_FEATURED variant)
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXPD_FW_TYPE_VSWITCH: c_uint = 0x3;
// enum: siena_compat variant RX PD firmware using PM rather than MAC
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXPD_FW_TYPE_SIENA_COMPAT_PM: c_uint = 0x4;
// enum: Low latency RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXPD_FW_TYPE_LOW_LATENCY: c_uint = 0x5;
// enum: Packed stream RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXPD_FW_TYPE_PACKED_STREAM: c_uint = 0x6;
// enum: RX PD firmware handling layer 2 only for high packet rate performance
// tests (Medford development only)
//
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXPD_FW_TYPE_LAYER2_PERF: c_uint = 0x7;
// enum: Rules engine RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXPD_FW_TYPE_RULES_ENGINE: c_uint = 0x8;
// enum: Custom firmware variant (see SF-119495-PD and bug69716)
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXPD_FW_TYPE_L3XUDP: c_uint = 0x9;
// enum: DPDK RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXPD_FW_TYPE_DPDK: c_uint = 0xa;
// enum: RX PD firmware for GUE parsing prototype (Medford development only)
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXPD_FW_TYPE_TESTFW_GUE_PROTOTYPE: c_uint = 0xe;
// enum: RX PD firmware parsing but not filtering network overlay tunnel
// encapsulations (Medford development only)
//
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXPD_FW_TYPE_TESTFW_ENCAP_PARSING_ONLY: c_uint = 0xf;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TXPD_FW_VERSION_OFST: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TXPD_FW_VERSION_LEN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TXPD_FW_VERSION_REV_OFST: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TXPD_FW_VERSION_REV_LBN: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TXPD_FW_VERSION_REV_WIDTH: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TXPD_FW_VERSION_TYPE_OFST: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TXPD_FW_VERSION_TYPE_LBN: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TXPD_FW_VERSION_TYPE_WIDTH: c_int = 4;
// enum: reserved value - do not use (may indicate alternative interpretation
// of REV field in future)
//
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TXPD_FW_TYPE_RESERVED: c_uint = 0x0;
// enum: Trivial TX PD firmware for early Huntington development (Huntington
// development only)
//
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TXPD_FW_TYPE_FIRST_PKT: c_uint = 0x1;
// enum: TX PD firmware for telemetry prototyping (Medford2 development only)
//
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TXPD_FW_TYPE_TESTFW_TELEMETRY: c_uint = 0x1;
// enum: TX PD firmware with approximately Siena-compatible behaviour
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TXPD_FW_TYPE_SIENA_COMPAT: c_uint = 0x2;
// enum: Full featured TX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TXPD_FW_TYPE_FULL_FEATURED: c_uint = 0x3;
// enum: (deprecated original name for the FULL_FEATURED variant)
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TXPD_FW_TYPE_VSWITCH: c_uint = 0x3;
// enum: siena_compat variant TX PD firmware using PM rather than MAC
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TXPD_FW_TYPE_SIENA_COMPAT_PM: c_uint = 0x4;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TXPD_FW_TYPE_LOW_LATENCY: c_uint = 0x5 /* enum */;
// enum: TX PD firmware handling layer 2 only for high packet rate performance
// tests (Medford development only)
//
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TXPD_FW_TYPE_LAYER2_PERF: c_uint = 0x7;
// enum: Rules engine TX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TXPD_FW_TYPE_RULES_ENGINE: c_uint = 0x8;
// enum: Custom firmware variant (see SF-119495-PD and bug69716)
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TXPD_FW_TYPE_L3XUDP: c_uint = 0x9;
// enum: DPDK TX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TXPD_FW_TYPE_DPDK: c_uint = 0xa;
// enum: RX PD firmware for GUE parsing prototype (Medford development only)
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TXPD_FW_TYPE_TESTFW_GUE_PROTOTYPE: c_uint = 0xe;
// Hardware capabilities of NIC
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_HW_CAPABILITIES_OFST: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_HW_CAPABILITIES_LEN: c_int = 4;
// Licensed capabilities
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_LICENSE_CAPABILITIES_OFST: c_int = 16;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_LICENSE_CAPABILITIES_LEN: c_int = 4;
// Second word of flags. Not present on older firmware (check the length).
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_FLAGS2_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_FLAGS2_LEN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_TSO_V2_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_TSO_V2_LBN: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_TSO_V2_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_TSO_V2_ENCAP_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_TSO_V2_ENCAP_LBN: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_TSO_V2_ENCAP_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_EVQ_TIMER_CTRL_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_EVQ_TIMER_CTRL_LBN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_EVQ_TIMER_CTRL_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_EVENT_CUT_THROUGH_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_EVENT_CUT_THROUGH_LBN: c_int = 3;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_EVENT_CUT_THROUGH_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_CUT_THROUGH_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_CUT_THROUGH_LBN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_CUT_THROUGH_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_VFIFO_ULL_MODE_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_VFIFO_ULL_MODE_LBN: c_int = 5;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_VFIFO_ULL_MODE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_MAC_STATS_40G_TX_SIZE_BINS_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_MAC_STATS_40G_TX_SIZE_BINS_LBN: c_int = 6;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_MAC_STATS_40G_TX_SIZE_BINS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_INIT_EVQ_TYPE_SUPPORTED_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_INIT_EVQ_TYPE_SUPPORTED_LBN: c_int = 7;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_INIT_EVQ_TYPE_SUPPORTED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_INIT_EVQ_V2_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_INIT_EVQ_V2_LBN: c_int = 7;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_INIT_EVQ_V2_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_MAC_TIMESTAMPING_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_MAC_TIMESTAMPING_LBN: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_MAC_TIMESTAMPING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_TIMESTAMP_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_TIMESTAMP_LBN: c_int = 9;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_TIMESTAMP_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_SNIFF_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_SNIFF_LBN: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_SNIFF_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_SNIFF_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_SNIFF_LBN: c_int = 11;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_SNIFF_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_NVRAM_UPDATE_REPORT_VERIFY_RESULT_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_NVRAM_UPDATE_REPORT_VERIFY_RESULT_LBN: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_NVRAM_UPDATE_REPORT_VERIFY_RESULT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_MCDI_BACKGROUND_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_MCDI_BACKGROUND_LBN: c_int = 13;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_MCDI_BACKGROUND_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_MCDI_DB_RETURN_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_MCDI_DB_RETURN_LBN: c_int = 14;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_MCDI_DB_RETURN_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_CTPIO_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_CTPIO_LBN: c_int = 15;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_CTPIO_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TSA_SUPPORT_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TSA_SUPPORT_LBN: c_int = 16;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TSA_SUPPORT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TSA_BOUND_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TSA_BOUND_LBN: c_int = 17;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TSA_BOUND_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_SF_ADAPTER_AUTHENTICATION_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_SF_ADAPTER_AUTHENTICATION_LBN: c_int = 18;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_SF_ADAPTER_AUTHENTICATION_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_FILTER_ACTION_FLAG_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_FILTER_ACTION_FLAG_LBN: c_int = 19;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_FILTER_ACTION_FLAG_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_FILTER_ACTION_MARK_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_FILTER_ACTION_MARK_LBN: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_FILTER_ACTION_MARK_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_EQUAL_STRIDE_SUPER_BUFFER_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_EQUAL_STRIDE_SUPER_BUFFER_LBN: c_int = 21;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_EQUAL_STRIDE_SUPER_BUFFER_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_EQUAL_STRIDE_PACKED_STREAM_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_EQUAL_STRIDE_PACKED_STREAM_LBN: c_int = 21;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_EQUAL_STRIDE_PACKED_STREAM_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_L3XUDP_SUPPORT_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_L3XUDP_SUPPORT_LBN: c_int = 22;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_L3XUDP_SUPPORT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_FW_SUBVARIANT_NO_TX_CSUM_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_FW_SUBVARIANT_NO_TX_CSUM_LBN: c_int = 23;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_FW_SUBVARIANT_NO_TX_CSUM_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_VI_SPREADING_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_VI_SPREADING_LBN: c_int = 24;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_VI_SPREADING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXDP_HLB_IDLE_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXDP_HLB_IDLE_LBN: c_int = 25;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RXDP_HLB_IDLE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_INIT_RXQ_NO_CONT_EV_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_INIT_RXQ_NO_CONT_EV_LBN: c_int = 26;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_INIT_RXQ_NO_CONT_EV_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_INIT_RXQ_WITH_BUFFER_SIZE_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_INIT_RXQ_WITH_BUFFER_SIZE_LBN: c_int = 27;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_INIT_RXQ_WITH_BUFFER_SIZE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_BUNDLE_UPDATE_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_BUNDLE_UPDATE_LBN: c_int = 28;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_BUNDLE_UPDATE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_TSO_V3_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_TSO_V3_LBN: c_int = 29;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_TSO_V3_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_DYNAMIC_SENSORS_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_DYNAMIC_SENSORS_LBN: c_int = 30;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_DYNAMIC_SENSORS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_NVRAM_UPDATE_POLL_VERIFY_RESULT_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_NVRAM_UPDATE_POLL_VERIFY_RESULT_LBN: c_int = 31;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_NVRAM_UPDATE_POLL_VERIFY_RESULT_WIDTH: c_int = 1;
// Number of FATSOv2 contexts per datapath supported by this NIC (when
// TX_TSO_V2 == 1). Not present on older firmware (check the length).
//
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_TSO_V2_N_CONTEXTS_OFST: c_int = 24;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_TSO_V2_N_CONTEXTS_LEN: c_int = 2;
// One byte per PF containing the number of the external port assigned to this
// PF, indexed by PF number. Special values indicate that a PF is either not
// present or not assigned.
//
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_PFS_TO_PORTS_ASSIGNMENT_OFST: c_int = 26;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_PFS_TO_PORTS_ASSIGNMENT_LEN: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_PFS_TO_PORTS_ASSIGNMENT_NUM: c_int = 16;
// enum: The caller is not permitted to access information on this PF.
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_ACCESS_NOT_PERMITTED: c_uint = 0xff;
// enum: PF does not exist.
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_PF_NOT_PRESENT: c_uint = 0xfe;
// enum: PF does exist but is not assigned to any external port.
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_PF_NOT_ASSIGNED: c_uint = 0xfd;
// enum: This value indicates that PF is assigned, but it cannot be expressed
// in this field. It is intended for a possible future situation where a more
// complex scheme of PFs to ports mapping is being used. The future driver
// should look for a new field supporting the new scheme. The current/old
// driver should treat this value as PF_NOT_ASSIGNED.
//
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_INCOMPATIBLE_ASSIGNMENT: c_uint = 0xfc;
// One byte per PF containing the number of its VFs, indexed by PF number. A
// special value indicates that a PF is not present.
//
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_NUM_VFS_PER_PF_OFST: c_int = 42;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_NUM_VFS_PER_PF_LEN: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_NUM_VFS_PER_PF_NUM: c_int = 16;
// enum: The caller is not permitted to access information on this PF.
// MC_CMD_GET_CAPABILITIES_V5_OUT_ACCESS_NOT_PERMITTED 0xff
// enum: PF does not exist.
// MC_CMD_GET_CAPABILITIES_V5_OUT_PF_NOT_PRESENT 0xfe
// Number of VIs available for each external port
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_NUM_VIS_PER_PORT_OFST: c_int = 58;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_NUM_VIS_PER_PORT_LEN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_NUM_VIS_PER_PORT_NUM: c_int = 4;
// Size of RX descriptor cache expressed as binary logarithm The actual size
// equals (2 ^ RX_DESC_CACHE_SIZE)
//
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_DESC_CACHE_SIZE_OFST: c_int = 66;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_RX_DESC_CACHE_SIZE_LEN: c_int = 1;
// Size of TX descriptor cache expressed as binary logarithm The actual size
// equals (2 ^ TX_DESC_CACHE_SIZE)
//
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_DESC_CACHE_SIZE_OFST: c_int = 67;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_TX_DESC_CACHE_SIZE_LEN: c_int = 1;
// Total number of available PIO buffers
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_NUM_PIO_BUFFS_OFST: c_int = 68;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_NUM_PIO_BUFFS_LEN: c_int = 2;
// Size of a single PIO buffer
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_SIZE_PIO_BUFF_OFST: c_int = 70;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_SIZE_PIO_BUFF_LEN: c_int = 2;
// On chips later than Medford the amount of address space assigned to each VI
// is configurable. This is a global setting that the driver must query to
// discover the VI to address mapping. Cut-through PIO (CTPIO) is not available
// with 8k VI windows.
//
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_VI_WINDOW_MODE_OFST: c_int = 72;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_VI_WINDOW_MODE_LEN: c_int = 1;
// enum: Each VI occupies 8k as on Huntington and Medford. PIO is at offset 4k.
// CTPIO is not mapped.
//
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_VI_WINDOW_MODE_8K: c_uint = 0x0;
// enum: Each VI occupies 16k. PIO is at offset 4k. CTPIO is at offset 12k.
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_VI_WINDOW_MODE_16K: c_uint = 0x1;
// enum: Each VI occupies 64k. PIO is at offset 4k. CTPIO is at offset 12k.
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_VI_WINDOW_MODE_64K: c_uint = 0x2;
// Number of vFIFOs per adapter that can be used for VFIFO Stuffing
// (SF-115995-SW) in the present configuration of firmware and port mode.
//
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_VFIFO_STUFFING_NUM_VFIFOS_OFST: c_int = 73;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_VFIFO_STUFFING_NUM_VFIFOS_LEN: c_int = 1;
// Number of buffers per adapter that can be used for VFIFO Stuffing
// (SF-115995-SW) in the present configuration of firmware and port mode.
//
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_VFIFO_STUFFING_NUM_CP_BUFFERS_OFST: c_int = 74;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_VFIFO_STUFFING_NUM_CP_BUFFERS_LEN: c_int = 2;
// Entry count in the MAC stats array, including the final GENERATION_END
// entry. For MAC stats DMA, drivers should allocate a buffer large enough to
// hold at least this many 64-bit stats values, if they wish to receive all
// available stats. If the buffer is shorter than MAC_STATS_NUM_STATS * 8, the
// stats array returned will be truncated.
//
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_MAC_STATS_NUM_STATS_OFST: c_int = 76;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_MAC_STATS_NUM_STATS_LEN: c_int = 2;
// Maximum supported value for MC_CMD_FILTER_OP_V3/MATCH_MARK_VALUE. This field
// will only be non-zero if MC_CMD_GET_CAPABILITIES/FILTER_ACTION_MARK is set.
//
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_FILTER_ACTION_MARK_MAX_OFST: c_int = 80;
pub const MC_CMD_GET_CAPABILITIES_V5_OUT_FILTER_ACTION_MARK_MAX_LEN: c_int = 4;
// MC_CMD_GET_CAPABILITIES_V6_OUT msgresponse
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_LEN: c_int = 148;
// First word of flags.
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_FLAGS1_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_FLAGS1_LEN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_VPORT_RECONFIGURE_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_VPORT_RECONFIGURE_LBN: c_int = 3;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_VPORT_RECONFIGURE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_STRIPING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_STRIPING_LBN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_STRIPING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_VADAPTOR_QUERY_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_VADAPTOR_QUERY_LBN: c_int = 5;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_VADAPTOR_QUERY_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_EVB_PORT_VLAN_RESTRICT_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_EVB_PORT_VLAN_RESTRICT_LBN: c_int = 6;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_EVB_PORT_VLAN_RESTRICT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_DRV_ATTACH_PREBOOT_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_DRV_ATTACH_PREBOOT_LBN: c_int = 7;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_DRV_ATTACH_PREBOOT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_FORCE_EVENT_MERGING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_FORCE_EVENT_MERGING_LBN: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_FORCE_EVENT_MERGING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_SET_MAC_ENHANCED_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_SET_MAC_ENHANCED_LBN: c_int = 9;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_SET_MAC_ENHANCED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_UNKNOWN_UCAST_DST_FILTER_ALWAYS_MULTI_RECIPIENT_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_UNKNOWN_UCAST_DST_FILTER_ALWAYS_MULTI_RECIPIENT_LBN: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_UNKNOWN_UCAST_DST_FILTER_ALWAYS_MULTI_RECIPIENT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_VADAPTOR_PERMIT_SET_MAC_WHEN_FILTERS_INSTALLED_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_VADAPTOR_PERMIT_SET_MAC_WHEN_FILTERS_INSTALLED_LBN: c_int = 11;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_VADAPTOR_PERMIT_SET_MAC_WHEN_FILTERS_INSTALLED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_MAC_SECURITY_FILTERING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_MAC_SECURITY_FILTERING_LBN: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_MAC_SECURITY_FILTERING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_ADDITIONAL_RSS_MODES_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_ADDITIONAL_RSS_MODES_LBN: c_int = 13;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_ADDITIONAL_RSS_MODES_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_QBB_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_QBB_LBN: c_int = 14;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_QBB_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_PACKED_STREAM_VAR_BUFFERS_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_PACKED_STREAM_VAR_BUFFERS_LBN: c_int = 15;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_PACKED_STREAM_VAR_BUFFERS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_RSS_LIMITED_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_RSS_LIMITED_LBN: c_int = 16;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_RSS_LIMITED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_PACKED_STREAM_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_PACKED_STREAM_LBN: c_int = 17;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_PACKED_STREAM_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_INCLUDE_FCS_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_INCLUDE_FCS_LBN: c_int = 18;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_INCLUDE_FCS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_VLAN_INSERTION_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_VLAN_INSERTION_LBN: c_int = 19;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_VLAN_INSERTION_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_VLAN_STRIPPING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_VLAN_STRIPPING_LBN: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_VLAN_STRIPPING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_TSO_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_TSO_LBN: c_int = 21;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_TSO_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_PREFIX_LEN_0_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_PREFIX_LEN_0_LBN: c_int = 22;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_PREFIX_LEN_0_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_PREFIX_LEN_14_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_PREFIX_LEN_14_LBN: c_int = 23;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_PREFIX_LEN_14_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_TIMESTAMP_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_TIMESTAMP_LBN: c_int = 24;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_TIMESTAMP_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_BATCHING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_BATCHING_LBN: c_int = 25;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_BATCHING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_MCAST_FILTER_CHAINING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_MCAST_FILTER_CHAINING_LBN: c_int = 26;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_MCAST_FILTER_CHAINING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_PM_AND_RXDP_COUNTERS_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_PM_AND_RXDP_COUNTERS_LBN: c_int = 27;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_PM_AND_RXDP_COUNTERS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_DISABLE_SCATTER_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_DISABLE_SCATTER_LBN: c_int = 28;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_DISABLE_SCATTER_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_MCAST_UDP_LOOPBACK_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_MCAST_UDP_LOOPBACK_LBN: c_int = 29;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_MCAST_UDP_LOOPBACK_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_EVB_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_EVB_LBN: c_int = 30;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_EVB_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_VXLAN_NVGRE_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_VXLAN_NVGRE_LBN: c_int = 31;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_VXLAN_NVGRE_WIDTH: c_int = 1;
// RxDPCPU firmware id.
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_DPCPU_FW_ID_OFST: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_DPCPU_FW_ID_LEN: c_int = 2;
// enum: Standard RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXDP: c_uint = 0x0;
// enum: Low latency RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXDP_LOW_LATENCY: c_uint = 0x1;
// enum: Packed stream RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXDP_PACKED_STREAM: c_uint = 0x2;
// enum: Rules engine RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXDP_RULES_ENGINE: c_uint = 0x5;
// enum: DPDK RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXDP_DPDK: c_uint = 0x6;
// enum: BIST RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXDP_BIST: c_uint = 0x10a;
// enum: RXDP Test firmware image 1
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXDP_TEST_FW_TO_MC_CUT_THROUGH: c_uint = 0x101;
// enum: RXDP Test firmware image 2
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXDP_TEST_FW_TO_MC_STORE_FORWARD: c_uint = 0x102;
// enum: RXDP Test firmware image 3
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXDP_TEST_FW_TO_MC_STORE_FORWARD_FIRST: c_uint = 0x103;
// enum: RXDP Test firmware image 4
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXDP_TEST_EVERY_EVENT_BATCHABLE: c_uint = 0x104;
// enum: RXDP Test firmware image 5
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXDP_TEST_BACKPRESSURE: c_uint = 0x105;
// enum: RXDP Test firmware image 6
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXDP_TEST_FW_PACKET_EDITS: c_uint = 0x106;
// enum: RXDP Test firmware image 7
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXDP_TEST_FW_RX_HDR_SPLIT: c_uint = 0x107;
// enum: RXDP Test firmware image 8
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXDP_TEST_FW_DISABLE_DL: c_uint = 0x108;
// enum: RXDP Test firmware image 9
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXDP_TEST_FW_DOORBELL_DELAY: c_uint = 0x10b;
// enum: RXDP Test firmware image 10
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXDP_TEST_FW_SLOW: c_uint = 0x10c;
// TxDPCPU firmware id.
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_DPCPU_FW_ID_OFST: c_int = 6;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_DPCPU_FW_ID_LEN: c_int = 2;
// enum: Standard TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TXDP: c_uint = 0x0;
// enum: Low latency TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TXDP_LOW_LATENCY: c_uint = 0x1;
// enum: High packet rate TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TXDP_HIGH_PACKET_RATE: c_uint = 0x3;
// enum: Rules engine TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TXDP_RULES_ENGINE: c_uint = 0x5;
// enum: DPDK TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TXDP_DPDK: c_uint = 0x6;
// enum: BIST TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TXDP_BIST: c_uint = 0x12d;
// enum: TXDP Test firmware image 1
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TXDP_TEST_FW_TSO_EDIT: c_uint = 0x101;
// enum: TXDP Test firmware image 2
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TXDP_TEST_FW_PACKET_EDITS: c_uint = 0x102;
// enum: TXDP CSR bus test firmware
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TXDP_TEST_FW_CSR: c_uint = 0x103;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXPD_FW_VERSION_OFST: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXPD_FW_VERSION_LEN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXPD_FW_VERSION_REV_OFST: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXPD_FW_VERSION_REV_LBN: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXPD_FW_VERSION_REV_WIDTH: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXPD_FW_VERSION_TYPE_OFST: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXPD_FW_VERSION_TYPE_LBN: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXPD_FW_VERSION_TYPE_WIDTH: c_int = 4;
// enum: reserved value - do not use (may indicate alternative interpretation
// of REV field in future)
//
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXPD_FW_TYPE_RESERVED: c_uint = 0x0;
// enum: Trivial RX PD firmware for early Huntington development (Huntington
// development only)
//
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXPD_FW_TYPE_FIRST_PKT: c_uint = 0x1;
// enum: RX PD firmware for telemetry prototyping (Medford2 development only)
//
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXPD_FW_TYPE_TESTFW_TELEMETRY: c_uint = 0x1;
// enum: RX PD firmware with approximately Siena-compatible behaviour
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXPD_FW_TYPE_SIENA_COMPAT: c_uint = 0x2;
// enum: Full featured RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXPD_FW_TYPE_FULL_FEATURED: c_uint = 0x3;
// enum: (deprecated original name for the FULL_FEATURED variant)
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXPD_FW_TYPE_VSWITCH: c_uint = 0x3;
// enum: siena_compat variant RX PD firmware using PM rather than MAC
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXPD_FW_TYPE_SIENA_COMPAT_PM: c_uint = 0x4;
// enum: Low latency RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXPD_FW_TYPE_LOW_LATENCY: c_uint = 0x5;
// enum: Packed stream RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXPD_FW_TYPE_PACKED_STREAM: c_uint = 0x6;
// enum: RX PD firmware handling layer 2 only for high packet rate performance
// tests (Medford development only)
//
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXPD_FW_TYPE_LAYER2_PERF: c_uint = 0x7;
// enum: Rules engine RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXPD_FW_TYPE_RULES_ENGINE: c_uint = 0x8;
// enum: Custom firmware variant (see SF-119495-PD and bug69716)
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXPD_FW_TYPE_L3XUDP: c_uint = 0x9;
// enum: DPDK RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXPD_FW_TYPE_DPDK: c_uint = 0xa;
// enum: RX PD firmware for GUE parsing prototype (Medford development only)
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXPD_FW_TYPE_TESTFW_GUE_PROTOTYPE: c_uint = 0xe;
// enum: RX PD firmware parsing but not filtering network overlay tunnel
// encapsulations (Medford development only)
//
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXPD_FW_TYPE_TESTFW_ENCAP_PARSING_ONLY: c_uint = 0xf;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TXPD_FW_VERSION_OFST: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TXPD_FW_VERSION_LEN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TXPD_FW_VERSION_REV_OFST: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TXPD_FW_VERSION_REV_LBN: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TXPD_FW_VERSION_REV_WIDTH: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TXPD_FW_VERSION_TYPE_OFST: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TXPD_FW_VERSION_TYPE_LBN: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TXPD_FW_VERSION_TYPE_WIDTH: c_int = 4;
// enum: reserved value - do not use (may indicate alternative interpretation
// of REV field in future)
//
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TXPD_FW_TYPE_RESERVED: c_uint = 0x0;
// enum: Trivial TX PD firmware for early Huntington development (Huntington
// development only)
//
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TXPD_FW_TYPE_FIRST_PKT: c_uint = 0x1;
// enum: TX PD firmware for telemetry prototyping (Medford2 development only)
//
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TXPD_FW_TYPE_TESTFW_TELEMETRY: c_uint = 0x1;
// enum: TX PD firmware with approximately Siena-compatible behaviour
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TXPD_FW_TYPE_SIENA_COMPAT: c_uint = 0x2;
// enum: Full featured TX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TXPD_FW_TYPE_FULL_FEATURED: c_uint = 0x3;
// enum: (deprecated original name for the FULL_FEATURED variant)
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TXPD_FW_TYPE_VSWITCH: c_uint = 0x3;
// enum: siena_compat variant TX PD firmware using PM rather than MAC
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TXPD_FW_TYPE_SIENA_COMPAT_PM: c_uint = 0x4;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TXPD_FW_TYPE_LOW_LATENCY: c_uint = 0x5 /* enum */;
// enum: TX PD firmware handling layer 2 only for high packet rate performance
// tests (Medford development only)
//
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TXPD_FW_TYPE_LAYER2_PERF: c_uint = 0x7;
// enum: Rules engine TX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TXPD_FW_TYPE_RULES_ENGINE: c_uint = 0x8;
// enum: Custom firmware variant (see SF-119495-PD and bug69716)
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TXPD_FW_TYPE_L3XUDP: c_uint = 0x9;
// enum: DPDK TX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TXPD_FW_TYPE_DPDK: c_uint = 0xa;
// enum: RX PD firmware for GUE parsing prototype (Medford development only)
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TXPD_FW_TYPE_TESTFW_GUE_PROTOTYPE: c_uint = 0xe;
// Hardware capabilities of NIC
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_HW_CAPABILITIES_OFST: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_HW_CAPABILITIES_LEN: c_int = 4;
// Licensed capabilities
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_LICENSE_CAPABILITIES_OFST: c_int = 16;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_LICENSE_CAPABILITIES_LEN: c_int = 4;
// Second word of flags. Not present on older firmware (check the length).
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_FLAGS2_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_FLAGS2_LEN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_TSO_V2_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_TSO_V2_LBN: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_TSO_V2_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_TSO_V2_ENCAP_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_TSO_V2_ENCAP_LBN: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_TSO_V2_ENCAP_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_EVQ_TIMER_CTRL_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_EVQ_TIMER_CTRL_LBN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_EVQ_TIMER_CTRL_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_EVENT_CUT_THROUGH_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_EVENT_CUT_THROUGH_LBN: c_int = 3;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_EVENT_CUT_THROUGH_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_CUT_THROUGH_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_CUT_THROUGH_LBN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_CUT_THROUGH_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_VFIFO_ULL_MODE_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_VFIFO_ULL_MODE_LBN: c_int = 5;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_VFIFO_ULL_MODE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_MAC_STATS_40G_TX_SIZE_BINS_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_MAC_STATS_40G_TX_SIZE_BINS_LBN: c_int = 6;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_MAC_STATS_40G_TX_SIZE_BINS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_INIT_EVQ_TYPE_SUPPORTED_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_INIT_EVQ_TYPE_SUPPORTED_LBN: c_int = 7;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_INIT_EVQ_TYPE_SUPPORTED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_INIT_EVQ_V2_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_INIT_EVQ_V2_LBN: c_int = 7;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_INIT_EVQ_V2_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_MAC_TIMESTAMPING_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_MAC_TIMESTAMPING_LBN: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_MAC_TIMESTAMPING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_TIMESTAMP_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_TIMESTAMP_LBN: c_int = 9;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_TIMESTAMP_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_SNIFF_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_SNIFF_LBN: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_SNIFF_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_SNIFF_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_SNIFF_LBN: c_int = 11;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_SNIFF_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_NVRAM_UPDATE_REPORT_VERIFY_RESULT_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_NVRAM_UPDATE_REPORT_VERIFY_RESULT_LBN: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_NVRAM_UPDATE_REPORT_VERIFY_RESULT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_MCDI_BACKGROUND_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_MCDI_BACKGROUND_LBN: c_int = 13;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_MCDI_BACKGROUND_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_MCDI_DB_RETURN_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_MCDI_DB_RETURN_LBN: c_int = 14;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_MCDI_DB_RETURN_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_CTPIO_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_CTPIO_LBN: c_int = 15;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_CTPIO_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TSA_SUPPORT_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TSA_SUPPORT_LBN: c_int = 16;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TSA_SUPPORT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TSA_BOUND_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TSA_BOUND_LBN: c_int = 17;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TSA_BOUND_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_SF_ADAPTER_AUTHENTICATION_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_SF_ADAPTER_AUTHENTICATION_LBN: c_int = 18;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_SF_ADAPTER_AUTHENTICATION_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_FILTER_ACTION_FLAG_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_FILTER_ACTION_FLAG_LBN: c_int = 19;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_FILTER_ACTION_FLAG_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_FILTER_ACTION_MARK_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_FILTER_ACTION_MARK_LBN: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_FILTER_ACTION_MARK_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_EQUAL_STRIDE_SUPER_BUFFER_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_EQUAL_STRIDE_SUPER_BUFFER_LBN: c_int = 21;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_EQUAL_STRIDE_SUPER_BUFFER_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_EQUAL_STRIDE_PACKED_STREAM_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_EQUAL_STRIDE_PACKED_STREAM_LBN: c_int = 21;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_EQUAL_STRIDE_PACKED_STREAM_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_L3XUDP_SUPPORT_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_L3XUDP_SUPPORT_LBN: c_int = 22;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_L3XUDP_SUPPORT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_FW_SUBVARIANT_NO_TX_CSUM_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_FW_SUBVARIANT_NO_TX_CSUM_LBN: c_int = 23;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_FW_SUBVARIANT_NO_TX_CSUM_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_VI_SPREADING_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_VI_SPREADING_LBN: c_int = 24;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_VI_SPREADING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXDP_HLB_IDLE_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXDP_HLB_IDLE_LBN: c_int = 25;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RXDP_HLB_IDLE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_INIT_RXQ_NO_CONT_EV_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_INIT_RXQ_NO_CONT_EV_LBN: c_int = 26;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_INIT_RXQ_NO_CONT_EV_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_INIT_RXQ_WITH_BUFFER_SIZE_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_INIT_RXQ_WITH_BUFFER_SIZE_LBN: c_int = 27;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_INIT_RXQ_WITH_BUFFER_SIZE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_BUNDLE_UPDATE_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_BUNDLE_UPDATE_LBN: c_int = 28;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_BUNDLE_UPDATE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_TSO_V3_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_TSO_V3_LBN: c_int = 29;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_TSO_V3_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_DYNAMIC_SENSORS_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_DYNAMIC_SENSORS_LBN: c_int = 30;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_DYNAMIC_SENSORS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_NVRAM_UPDATE_POLL_VERIFY_RESULT_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_NVRAM_UPDATE_POLL_VERIFY_RESULT_LBN: c_int = 31;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_NVRAM_UPDATE_POLL_VERIFY_RESULT_WIDTH: c_int = 1;
// Number of FATSOv2 contexts per datapath supported by this NIC (when
// TX_TSO_V2 == 1). Not present on older firmware (check the length).
//
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_TSO_V2_N_CONTEXTS_OFST: c_int = 24;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_TSO_V2_N_CONTEXTS_LEN: c_int = 2;
// One byte per PF containing the number of the external port assigned to this
// PF, indexed by PF number. Special values indicate that a PF is either not
// present or not assigned.
//
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_PFS_TO_PORTS_ASSIGNMENT_OFST: c_int = 26;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_PFS_TO_PORTS_ASSIGNMENT_LEN: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_PFS_TO_PORTS_ASSIGNMENT_NUM: c_int = 16;
// enum: The caller is not permitted to access information on this PF.
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_ACCESS_NOT_PERMITTED: c_uint = 0xff;
// enum: PF does not exist.
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_PF_NOT_PRESENT: c_uint = 0xfe;
// enum: PF does exist but is not assigned to any external port.
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_PF_NOT_ASSIGNED: c_uint = 0xfd;
// enum: This value indicates that PF is assigned, but it cannot be expressed
// in this field. It is intended for a possible future situation where a more
// complex scheme of PFs to ports mapping is being used. The future driver
// should look for a new field supporting the new scheme. The current/old
// driver should treat this value as PF_NOT_ASSIGNED.
//
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_INCOMPATIBLE_ASSIGNMENT: c_uint = 0xfc;
// One byte per PF containing the number of its VFs, indexed by PF number. A
// special value indicates that a PF is not present.
//
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_NUM_VFS_PER_PF_OFST: c_int = 42;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_NUM_VFS_PER_PF_LEN: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_NUM_VFS_PER_PF_NUM: c_int = 16;
// enum: The caller is not permitted to access information on this PF.
// MC_CMD_GET_CAPABILITIES_V6_OUT_ACCESS_NOT_PERMITTED 0xff
// enum: PF does not exist.
// MC_CMD_GET_CAPABILITIES_V6_OUT_PF_NOT_PRESENT 0xfe
// Number of VIs available for each external port
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_NUM_VIS_PER_PORT_OFST: c_int = 58;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_NUM_VIS_PER_PORT_LEN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_NUM_VIS_PER_PORT_NUM: c_int = 4;
// Size of RX descriptor cache expressed as binary logarithm The actual size
// equals (2 ^ RX_DESC_CACHE_SIZE)
//
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_DESC_CACHE_SIZE_OFST: c_int = 66;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_RX_DESC_CACHE_SIZE_LEN: c_int = 1;
// Size of TX descriptor cache expressed as binary logarithm The actual size
// equals (2 ^ TX_DESC_CACHE_SIZE)
//
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_DESC_CACHE_SIZE_OFST: c_int = 67;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_TX_DESC_CACHE_SIZE_LEN: c_int = 1;
// Total number of available PIO buffers
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_NUM_PIO_BUFFS_OFST: c_int = 68;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_NUM_PIO_BUFFS_LEN: c_int = 2;
// Size of a single PIO buffer
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_SIZE_PIO_BUFF_OFST: c_int = 70;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_SIZE_PIO_BUFF_LEN: c_int = 2;
// On chips later than Medford the amount of address space assigned to each VI
// is configurable. This is a global setting that the driver must query to
// discover the VI to address mapping. Cut-through PIO (CTPIO) is not available
// with 8k VI windows.
//
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_VI_WINDOW_MODE_OFST: c_int = 72;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_VI_WINDOW_MODE_LEN: c_int = 1;
// enum: Each VI occupies 8k as on Huntington and Medford. PIO is at offset 4k.
// CTPIO is not mapped.
//
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_VI_WINDOW_MODE_8K: c_uint = 0x0;
// enum: Each VI occupies 16k. PIO is at offset 4k. CTPIO is at offset 12k.
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_VI_WINDOW_MODE_16K: c_uint = 0x1;
// enum: Each VI occupies 64k. PIO is at offset 4k. CTPIO is at offset 12k.
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_VI_WINDOW_MODE_64K: c_uint = 0x2;
// Number of vFIFOs per adapter that can be used for VFIFO Stuffing
// (SF-115995-SW) in the present configuration of firmware and port mode.
//
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_VFIFO_STUFFING_NUM_VFIFOS_OFST: c_int = 73;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_VFIFO_STUFFING_NUM_VFIFOS_LEN: c_int = 1;
// Number of buffers per adapter that can be used for VFIFO Stuffing
// (SF-115995-SW) in the present configuration of firmware and port mode.
//
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_VFIFO_STUFFING_NUM_CP_BUFFERS_OFST: c_int = 74;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_VFIFO_STUFFING_NUM_CP_BUFFERS_LEN: c_int = 2;
// Entry count in the MAC stats array, including the final GENERATION_END
// entry. For MAC stats DMA, drivers should allocate a buffer large enough to
// hold at least this many 64-bit stats values, if they wish to receive all
// available stats. If the buffer is shorter than MAC_STATS_NUM_STATS * 8, the
// stats array returned will be truncated.
//
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_MAC_STATS_NUM_STATS_OFST: c_int = 76;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_MAC_STATS_NUM_STATS_LEN: c_int = 2;
// Maximum supported value for MC_CMD_FILTER_OP_V3/MATCH_MARK_VALUE. This field
// will only be non-zero if MC_CMD_GET_CAPABILITIES/FILTER_ACTION_MARK is set.
//
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_FILTER_ACTION_MARK_MAX_OFST: c_int = 80;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_FILTER_ACTION_MARK_MAX_LEN: c_int = 4;
// On devices where the INIT_RXQ_WITH_BUFFER_SIZE flag (in
// GET_CAPABILITIES_OUT_V2) is set, drivers have to specify a buffer size when
// they create an RX queue. Due to hardware limitations, only a small number of
// different buffer sizes may be available concurrently. Nonzero entries in
// this array are the sizes of buffers which the system guarantees will be
// available for use. If the list is empty, there are no limitations on
// concurrent buffer sizes.
//
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_GUARANTEED_RX_BUFFER_SIZES_OFST: c_int = 84;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_GUARANTEED_RX_BUFFER_SIZES_LEN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V6_OUT_GUARANTEED_RX_BUFFER_SIZES_NUM: c_int = 16;
// MC_CMD_GET_CAPABILITIES_V7_OUT msgresponse
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_LEN: c_int = 152;
// First word of flags.
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_FLAGS1_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_FLAGS1_LEN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_VPORT_RECONFIGURE_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_VPORT_RECONFIGURE_LBN: c_int = 3;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_VPORT_RECONFIGURE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_STRIPING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_STRIPING_LBN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_STRIPING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_VADAPTOR_QUERY_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_VADAPTOR_QUERY_LBN: c_int = 5;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_VADAPTOR_QUERY_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_EVB_PORT_VLAN_RESTRICT_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_EVB_PORT_VLAN_RESTRICT_LBN: c_int = 6;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_EVB_PORT_VLAN_RESTRICT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_DRV_ATTACH_PREBOOT_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_DRV_ATTACH_PREBOOT_LBN: c_int = 7;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_DRV_ATTACH_PREBOOT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_FORCE_EVENT_MERGING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_FORCE_EVENT_MERGING_LBN: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_FORCE_EVENT_MERGING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_SET_MAC_ENHANCED_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_SET_MAC_ENHANCED_LBN: c_int = 9;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_SET_MAC_ENHANCED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_UNKNOWN_UCAST_DST_FILTER_ALWAYS_MULTI_RECIPIENT_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_UNKNOWN_UCAST_DST_FILTER_ALWAYS_MULTI_RECIPIENT_LBN: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_UNKNOWN_UCAST_DST_FILTER_ALWAYS_MULTI_RECIPIENT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_VADAPTOR_PERMIT_SET_MAC_WHEN_FILTERS_INSTALLED_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_VADAPTOR_PERMIT_SET_MAC_WHEN_FILTERS_INSTALLED_LBN: c_int = 11;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_VADAPTOR_PERMIT_SET_MAC_WHEN_FILTERS_INSTALLED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_MAC_SECURITY_FILTERING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_MAC_SECURITY_FILTERING_LBN: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_MAC_SECURITY_FILTERING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_ADDITIONAL_RSS_MODES_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_ADDITIONAL_RSS_MODES_LBN: c_int = 13;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_ADDITIONAL_RSS_MODES_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_QBB_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_QBB_LBN: c_int = 14;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_QBB_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_PACKED_STREAM_VAR_BUFFERS_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_PACKED_STREAM_VAR_BUFFERS_LBN: c_int = 15;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_PACKED_STREAM_VAR_BUFFERS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_RSS_LIMITED_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_RSS_LIMITED_LBN: c_int = 16;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_RSS_LIMITED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_PACKED_STREAM_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_PACKED_STREAM_LBN: c_int = 17;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_PACKED_STREAM_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_INCLUDE_FCS_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_INCLUDE_FCS_LBN: c_int = 18;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_INCLUDE_FCS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_VLAN_INSERTION_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_VLAN_INSERTION_LBN: c_int = 19;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_VLAN_INSERTION_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_VLAN_STRIPPING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_VLAN_STRIPPING_LBN: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_VLAN_STRIPPING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_TSO_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_TSO_LBN: c_int = 21;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_TSO_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_PREFIX_LEN_0_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_PREFIX_LEN_0_LBN: c_int = 22;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_PREFIX_LEN_0_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_PREFIX_LEN_14_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_PREFIX_LEN_14_LBN: c_int = 23;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_PREFIX_LEN_14_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_TIMESTAMP_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_TIMESTAMP_LBN: c_int = 24;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_TIMESTAMP_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_BATCHING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_BATCHING_LBN: c_int = 25;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_BATCHING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_MCAST_FILTER_CHAINING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_MCAST_FILTER_CHAINING_LBN: c_int = 26;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_MCAST_FILTER_CHAINING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_PM_AND_RXDP_COUNTERS_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_PM_AND_RXDP_COUNTERS_LBN: c_int = 27;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_PM_AND_RXDP_COUNTERS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_DISABLE_SCATTER_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_DISABLE_SCATTER_LBN: c_int = 28;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_DISABLE_SCATTER_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_MCAST_UDP_LOOPBACK_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_MCAST_UDP_LOOPBACK_LBN: c_int = 29;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_MCAST_UDP_LOOPBACK_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_EVB_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_EVB_LBN: c_int = 30;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_EVB_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_VXLAN_NVGRE_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_VXLAN_NVGRE_LBN: c_int = 31;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_VXLAN_NVGRE_WIDTH: c_int = 1;
// RxDPCPU firmware id.
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_DPCPU_FW_ID_OFST: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_DPCPU_FW_ID_LEN: c_int = 2;
// enum: Standard RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXDP: c_uint = 0x0;
// enum: Low latency RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXDP_LOW_LATENCY: c_uint = 0x1;
// enum: Packed stream RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXDP_PACKED_STREAM: c_uint = 0x2;
// enum: Rules engine RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXDP_RULES_ENGINE: c_uint = 0x5;
// enum: DPDK RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXDP_DPDK: c_uint = 0x6;
// enum: BIST RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXDP_BIST: c_uint = 0x10a;
// enum: RXDP Test firmware image 1
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXDP_TEST_FW_TO_MC_CUT_THROUGH: c_uint = 0x101;
// enum: RXDP Test firmware image 2
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXDP_TEST_FW_TO_MC_STORE_FORWARD: c_uint = 0x102;
// enum: RXDP Test firmware image 3
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXDP_TEST_FW_TO_MC_STORE_FORWARD_FIRST: c_uint = 0x103;
// enum: RXDP Test firmware image 4
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXDP_TEST_EVERY_EVENT_BATCHABLE: c_uint = 0x104;
// enum: RXDP Test firmware image 5
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXDP_TEST_BACKPRESSURE: c_uint = 0x105;
// enum: RXDP Test firmware image 6
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXDP_TEST_FW_PACKET_EDITS: c_uint = 0x106;
// enum: RXDP Test firmware image 7
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXDP_TEST_FW_RX_HDR_SPLIT: c_uint = 0x107;
// enum: RXDP Test firmware image 8
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXDP_TEST_FW_DISABLE_DL: c_uint = 0x108;
// enum: RXDP Test firmware image 9
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXDP_TEST_FW_DOORBELL_DELAY: c_uint = 0x10b;
// enum: RXDP Test firmware image 10
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXDP_TEST_FW_SLOW: c_uint = 0x10c;
// TxDPCPU firmware id.
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_DPCPU_FW_ID_OFST: c_int = 6;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_DPCPU_FW_ID_LEN: c_int = 2;
// enum: Standard TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TXDP: c_uint = 0x0;
// enum: Low latency TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TXDP_LOW_LATENCY: c_uint = 0x1;
// enum: High packet rate TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TXDP_HIGH_PACKET_RATE: c_uint = 0x3;
// enum: Rules engine TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TXDP_RULES_ENGINE: c_uint = 0x5;
// enum: DPDK TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TXDP_DPDK: c_uint = 0x6;
// enum: BIST TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TXDP_BIST: c_uint = 0x12d;
// enum: TXDP Test firmware image 1
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TXDP_TEST_FW_TSO_EDIT: c_uint = 0x101;
// enum: TXDP Test firmware image 2
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TXDP_TEST_FW_PACKET_EDITS: c_uint = 0x102;
// enum: TXDP CSR bus test firmware
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TXDP_TEST_FW_CSR: c_uint = 0x103;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXPD_FW_VERSION_OFST: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXPD_FW_VERSION_LEN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXPD_FW_VERSION_REV_OFST: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXPD_FW_VERSION_REV_LBN: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXPD_FW_VERSION_REV_WIDTH: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXPD_FW_VERSION_TYPE_OFST: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXPD_FW_VERSION_TYPE_LBN: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXPD_FW_VERSION_TYPE_WIDTH: c_int = 4;
// enum: reserved value - do not use (may indicate alternative interpretation
// of REV field in future)
//
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXPD_FW_TYPE_RESERVED: c_uint = 0x0;
// enum: Trivial RX PD firmware for early Huntington development (Huntington
// development only)
//
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXPD_FW_TYPE_FIRST_PKT: c_uint = 0x1;
// enum: RX PD firmware for telemetry prototyping (Medford2 development only)
//
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXPD_FW_TYPE_TESTFW_TELEMETRY: c_uint = 0x1;
// enum: RX PD firmware with approximately Siena-compatible behaviour
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXPD_FW_TYPE_SIENA_COMPAT: c_uint = 0x2;
// enum: Full featured RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXPD_FW_TYPE_FULL_FEATURED: c_uint = 0x3;
// enum: (deprecated original name for the FULL_FEATURED variant)
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXPD_FW_TYPE_VSWITCH: c_uint = 0x3;
// enum: siena_compat variant RX PD firmware using PM rather than MAC
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXPD_FW_TYPE_SIENA_COMPAT_PM: c_uint = 0x4;
// enum: Low latency RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXPD_FW_TYPE_LOW_LATENCY: c_uint = 0x5;
// enum: Packed stream RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXPD_FW_TYPE_PACKED_STREAM: c_uint = 0x6;
// enum: RX PD firmware handling layer 2 only for high packet rate performance
// tests (Medford development only)
//
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXPD_FW_TYPE_LAYER2_PERF: c_uint = 0x7;
// enum: Rules engine RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXPD_FW_TYPE_RULES_ENGINE: c_uint = 0x8;
// enum: Custom firmware variant (see SF-119495-PD and bug69716)
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXPD_FW_TYPE_L3XUDP: c_uint = 0x9;
// enum: DPDK RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXPD_FW_TYPE_DPDK: c_uint = 0xa;
// enum: RX PD firmware for GUE parsing prototype (Medford development only)
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXPD_FW_TYPE_TESTFW_GUE_PROTOTYPE: c_uint = 0xe;
// enum: RX PD firmware parsing but not filtering network overlay tunnel
// encapsulations (Medford development only)
//
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXPD_FW_TYPE_TESTFW_ENCAP_PARSING_ONLY: c_uint = 0xf;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TXPD_FW_VERSION_OFST: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TXPD_FW_VERSION_LEN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TXPD_FW_VERSION_REV_OFST: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TXPD_FW_VERSION_REV_LBN: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TXPD_FW_VERSION_REV_WIDTH: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TXPD_FW_VERSION_TYPE_OFST: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TXPD_FW_VERSION_TYPE_LBN: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TXPD_FW_VERSION_TYPE_WIDTH: c_int = 4;
// enum: reserved value - do not use (may indicate alternative interpretation
// of REV field in future)
//
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TXPD_FW_TYPE_RESERVED: c_uint = 0x0;
// enum: Trivial TX PD firmware for early Huntington development (Huntington
// development only)
//
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TXPD_FW_TYPE_FIRST_PKT: c_uint = 0x1;
// enum: TX PD firmware for telemetry prototyping (Medford2 development only)
//
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TXPD_FW_TYPE_TESTFW_TELEMETRY: c_uint = 0x1;
// enum: TX PD firmware with approximately Siena-compatible behaviour
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TXPD_FW_TYPE_SIENA_COMPAT: c_uint = 0x2;
// enum: Full featured TX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TXPD_FW_TYPE_FULL_FEATURED: c_uint = 0x3;
// enum: (deprecated original name for the FULL_FEATURED variant)
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TXPD_FW_TYPE_VSWITCH: c_uint = 0x3;
// enum: siena_compat variant TX PD firmware using PM rather than MAC
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TXPD_FW_TYPE_SIENA_COMPAT_PM: c_uint = 0x4;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TXPD_FW_TYPE_LOW_LATENCY: c_uint = 0x5 /* enum */;
// enum: TX PD firmware handling layer 2 only for high packet rate performance
// tests (Medford development only)
//
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TXPD_FW_TYPE_LAYER2_PERF: c_uint = 0x7;
// enum: Rules engine TX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TXPD_FW_TYPE_RULES_ENGINE: c_uint = 0x8;
// enum: Custom firmware variant (see SF-119495-PD and bug69716)
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TXPD_FW_TYPE_L3XUDP: c_uint = 0x9;
// enum: DPDK TX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TXPD_FW_TYPE_DPDK: c_uint = 0xa;
// enum: RX PD firmware for GUE parsing prototype (Medford development only)
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TXPD_FW_TYPE_TESTFW_GUE_PROTOTYPE: c_uint = 0xe;
// Hardware capabilities of NIC
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_HW_CAPABILITIES_OFST: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_HW_CAPABILITIES_LEN: c_int = 4;
// Licensed capabilities
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_LICENSE_CAPABILITIES_OFST: c_int = 16;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_LICENSE_CAPABILITIES_LEN: c_int = 4;
// Second word of flags. Not present on older firmware (check the length).
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_FLAGS2_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_FLAGS2_LEN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_TSO_V2_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_TSO_V2_LBN: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_TSO_V2_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_TSO_V2_ENCAP_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_TSO_V2_ENCAP_LBN: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_TSO_V2_ENCAP_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_EVQ_TIMER_CTRL_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_EVQ_TIMER_CTRL_LBN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_EVQ_TIMER_CTRL_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_EVENT_CUT_THROUGH_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_EVENT_CUT_THROUGH_LBN: c_int = 3;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_EVENT_CUT_THROUGH_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_CUT_THROUGH_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_CUT_THROUGH_LBN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_CUT_THROUGH_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_VFIFO_ULL_MODE_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_VFIFO_ULL_MODE_LBN: c_int = 5;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_VFIFO_ULL_MODE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_MAC_STATS_40G_TX_SIZE_BINS_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_MAC_STATS_40G_TX_SIZE_BINS_LBN: c_int = 6;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_MAC_STATS_40G_TX_SIZE_BINS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_INIT_EVQ_TYPE_SUPPORTED_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_INIT_EVQ_TYPE_SUPPORTED_LBN: c_int = 7;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_INIT_EVQ_TYPE_SUPPORTED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_INIT_EVQ_V2_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_INIT_EVQ_V2_LBN: c_int = 7;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_INIT_EVQ_V2_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_MAC_TIMESTAMPING_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_MAC_TIMESTAMPING_LBN: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_MAC_TIMESTAMPING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_TIMESTAMP_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_TIMESTAMP_LBN: c_int = 9;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_TIMESTAMP_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_SNIFF_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_SNIFF_LBN: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_SNIFF_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_SNIFF_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_SNIFF_LBN: c_int = 11;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_SNIFF_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_NVRAM_UPDATE_REPORT_VERIFY_RESULT_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_NVRAM_UPDATE_REPORT_VERIFY_RESULT_LBN: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_NVRAM_UPDATE_REPORT_VERIFY_RESULT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_MCDI_BACKGROUND_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_MCDI_BACKGROUND_LBN: c_int = 13;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_MCDI_BACKGROUND_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_MCDI_DB_RETURN_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_MCDI_DB_RETURN_LBN: c_int = 14;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_MCDI_DB_RETURN_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_CTPIO_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_CTPIO_LBN: c_int = 15;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_CTPIO_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TSA_SUPPORT_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TSA_SUPPORT_LBN: c_int = 16;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TSA_SUPPORT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TSA_BOUND_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TSA_BOUND_LBN: c_int = 17;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TSA_BOUND_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_SF_ADAPTER_AUTHENTICATION_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_SF_ADAPTER_AUTHENTICATION_LBN: c_int = 18;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_SF_ADAPTER_AUTHENTICATION_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_FILTER_ACTION_FLAG_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_FILTER_ACTION_FLAG_LBN: c_int = 19;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_FILTER_ACTION_FLAG_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_FILTER_ACTION_MARK_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_FILTER_ACTION_MARK_LBN: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_FILTER_ACTION_MARK_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_EQUAL_STRIDE_SUPER_BUFFER_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_EQUAL_STRIDE_SUPER_BUFFER_LBN: c_int = 21;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_EQUAL_STRIDE_SUPER_BUFFER_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_EQUAL_STRIDE_PACKED_STREAM_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_EQUAL_STRIDE_PACKED_STREAM_LBN: c_int = 21;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_EQUAL_STRIDE_PACKED_STREAM_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_L3XUDP_SUPPORT_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_L3XUDP_SUPPORT_LBN: c_int = 22;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_L3XUDP_SUPPORT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_FW_SUBVARIANT_NO_TX_CSUM_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_FW_SUBVARIANT_NO_TX_CSUM_LBN: c_int = 23;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_FW_SUBVARIANT_NO_TX_CSUM_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_VI_SPREADING_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_VI_SPREADING_LBN: c_int = 24;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_VI_SPREADING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXDP_HLB_IDLE_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXDP_HLB_IDLE_LBN: c_int = 25;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RXDP_HLB_IDLE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_INIT_RXQ_NO_CONT_EV_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_INIT_RXQ_NO_CONT_EV_LBN: c_int = 26;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_INIT_RXQ_NO_CONT_EV_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_INIT_RXQ_WITH_BUFFER_SIZE_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_INIT_RXQ_WITH_BUFFER_SIZE_LBN: c_int = 27;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_INIT_RXQ_WITH_BUFFER_SIZE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_BUNDLE_UPDATE_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_BUNDLE_UPDATE_LBN: c_int = 28;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_BUNDLE_UPDATE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_TSO_V3_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_TSO_V3_LBN: c_int = 29;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_TSO_V3_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_DYNAMIC_SENSORS_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_DYNAMIC_SENSORS_LBN: c_int = 30;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_DYNAMIC_SENSORS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_NVRAM_UPDATE_POLL_VERIFY_RESULT_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_NVRAM_UPDATE_POLL_VERIFY_RESULT_LBN: c_int = 31;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_NVRAM_UPDATE_POLL_VERIFY_RESULT_WIDTH: c_int = 1;
// Number of FATSOv2 contexts per datapath supported by this NIC (when
// TX_TSO_V2 == 1). Not present on older firmware (check the length).
//
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_TSO_V2_N_CONTEXTS_OFST: c_int = 24;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_TSO_V2_N_CONTEXTS_LEN: c_int = 2;
// One byte per PF containing the number of the external port assigned to this
// PF, indexed by PF number. Special values indicate that a PF is either not
// present or not assigned.
//
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_PFS_TO_PORTS_ASSIGNMENT_OFST: c_int = 26;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_PFS_TO_PORTS_ASSIGNMENT_LEN: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_PFS_TO_PORTS_ASSIGNMENT_NUM: c_int = 16;
// enum: The caller is not permitted to access information on this PF.
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_ACCESS_NOT_PERMITTED: c_uint = 0xff;
// enum: PF does not exist.
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_PF_NOT_PRESENT: c_uint = 0xfe;
// enum: PF does exist but is not assigned to any external port.
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_PF_NOT_ASSIGNED: c_uint = 0xfd;
// enum: This value indicates that PF is assigned, but it cannot be expressed
// in this field. It is intended for a possible future situation where a more
// complex scheme of PFs to ports mapping is being used. The future driver
// should look for a new field supporting the new scheme. The current/old
// driver should treat this value as PF_NOT_ASSIGNED.
//
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_INCOMPATIBLE_ASSIGNMENT: c_uint = 0xfc;
// One byte per PF containing the number of its VFs, indexed by PF number. A
// special value indicates that a PF is not present.
//
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_NUM_VFS_PER_PF_OFST: c_int = 42;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_NUM_VFS_PER_PF_LEN: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_NUM_VFS_PER_PF_NUM: c_int = 16;
// enum: The caller is not permitted to access information on this PF.
// MC_CMD_GET_CAPABILITIES_V7_OUT_ACCESS_NOT_PERMITTED 0xff
// enum: PF does not exist.
// MC_CMD_GET_CAPABILITIES_V7_OUT_PF_NOT_PRESENT 0xfe
// Number of VIs available for each external port
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_NUM_VIS_PER_PORT_OFST: c_int = 58;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_NUM_VIS_PER_PORT_LEN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_NUM_VIS_PER_PORT_NUM: c_int = 4;
// Size of RX descriptor cache expressed as binary logarithm The actual size
// equals (2 ^ RX_DESC_CACHE_SIZE)
//
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_DESC_CACHE_SIZE_OFST: c_int = 66;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_DESC_CACHE_SIZE_LEN: c_int = 1;
// Size of TX descriptor cache expressed as binary logarithm The actual size
// equals (2 ^ TX_DESC_CACHE_SIZE)
//
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_DESC_CACHE_SIZE_OFST: c_int = 67;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_TX_DESC_CACHE_SIZE_LEN: c_int = 1;
// Total number of available PIO buffers
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_NUM_PIO_BUFFS_OFST: c_int = 68;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_NUM_PIO_BUFFS_LEN: c_int = 2;
// Size of a single PIO buffer
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_SIZE_PIO_BUFF_OFST: c_int = 70;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_SIZE_PIO_BUFF_LEN: c_int = 2;
// On chips later than Medford the amount of address space assigned to each VI
// is configurable. This is a global setting that the driver must query to
// discover the VI to address mapping. Cut-through PIO (CTPIO) is not available
// with 8k VI windows.
//
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_VI_WINDOW_MODE_OFST: c_int = 72;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_VI_WINDOW_MODE_LEN: c_int = 1;
// enum: Each VI occupies 8k as on Huntington and Medford. PIO is at offset 4k.
// CTPIO is not mapped.
//
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_VI_WINDOW_MODE_8K: c_uint = 0x0;
// enum: Each VI occupies 16k. PIO is at offset 4k. CTPIO is at offset 12k.
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_VI_WINDOW_MODE_16K: c_uint = 0x1;
// enum: Each VI occupies 64k. PIO is at offset 4k. CTPIO is at offset 12k.
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_VI_WINDOW_MODE_64K: c_uint = 0x2;
// Number of vFIFOs per adapter that can be used for VFIFO Stuffing
// (SF-115995-SW) in the present configuration of firmware and port mode.
//
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_VFIFO_STUFFING_NUM_VFIFOS_OFST: c_int = 73;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_VFIFO_STUFFING_NUM_VFIFOS_LEN: c_int = 1;
// Number of buffers per adapter that can be used for VFIFO Stuffing
// (SF-115995-SW) in the present configuration of firmware and port mode.
//
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_VFIFO_STUFFING_NUM_CP_BUFFERS_OFST: c_int = 74;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_VFIFO_STUFFING_NUM_CP_BUFFERS_LEN: c_int = 2;
// Entry count in the MAC stats array, including the final GENERATION_END
// entry. For MAC stats DMA, drivers should allocate a buffer large enough to
// hold at least this many 64-bit stats values, if they wish to receive all
// available stats. If the buffer is shorter than MAC_STATS_NUM_STATS * 8, the
// stats array returned will be truncated.
//
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_MAC_STATS_NUM_STATS_OFST: c_int = 76;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_MAC_STATS_NUM_STATS_LEN: c_int = 2;
// Maximum supported value for MC_CMD_FILTER_OP_V3/MATCH_MARK_VALUE. This field
// will only be non-zero if MC_CMD_GET_CAPABILITIES/FILTER_ACTION_MARK is set.
//
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_FILTER_ACTION_MARK_MAX_OFST: c_int = 80;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_FILTER_ACTION_MARK_MAX_LEN: c_int = 4;
// On devices where the INIT_RXQ_WITH_BUFFER_SIZE flag (in
// GET_CAPABILITIES_OUT_V2) is set, drivers have to specify a buffer size when
// they create an RX queue. Due to hardware limitations, only a small number of
// different buffer sizes may be available concurrently. Nonzero entries in
// this array are the sizes of buffers which the system guarantees will be
// available for use. If the list is empty, there are no limitations on
// concurrent buffer sizes.
//
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_GUARANTEED_RX_BUFFER_SIZES_OFST: c_int = 84;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_GUARANTEED_RX_BUFFER_SIZES_LEN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_GUARANTEED_RX_BUFFER_SIZES_NUM: c_int = 16;
// Third word of flags. Not present on older firmware (check the length).
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_FLAGS3_OFST: c_int = 148;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_FLAGS3_LEN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_WOL_ETHERWAKE_OFST: c_int = 148;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_WOL_ETHERWAKE_LBN: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_WOL_ETHERWAKE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RSS_EVEN_SPREADING_OFST: c_int = 148;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RSS_EVEN_SPREADING_LBN: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RSS_EVEN_SPREADING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RSS_SELECTABLE_TABLE_SIZE_OFST: c_int = 148;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RSS_SELECTABLE_TABLE_SIZE_LBN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RSS_SELECTABLE_TABLE_SIZE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_MAE_SUPPORTED_OFST: c_int = 148;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_MAE_SUPPORTED_LBN: c_int = 3;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_MAE_SUPPORTED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_VDPA_SUPPORTED_OFST: c_int = 148;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_VDPA_SUPPORTED_LBN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_VDPA_SUPPORTED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_VLAN_STRIPPING_PER_ENCAP_RULE_OFST: c_int = 148;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_VLAN_STRIPPING_PER_ENCAP_RULE_LBN: c_int = 5;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_RX_VLAN_STRIPPING_PER_ENCAP_RULE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_EXTENDED_WIDTH_EVQS_SUPPORTED_OFST: c_int = 148;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_EXTENDED_WIDTH_EVQS_SUPPORTED_LBN: c_int = 6;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_EXTENDED_WIDTH_EVQS_SUPPORTED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_UNSOL_EV_CREDIT_SUPPORTED_OFST: c_int = 148;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_UNSOL_EV_CREDIT_SUPPORTED_LBN: c_int = 7;
pub const MC_CMD_GET_CAPABILITIES_V7_OUT_UNSOL_EV_CREDIT_SUPPORTED_WIDTH: c_int = 1;
// MC_CMD_GET_CAPABILITIES_V8_OUT msgresponse
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_LEN: c_int = 160;
// First word of flags.
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_FLAGS1_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_FLAGS1_LEN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_VPORT_RECONFIGURE_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_VPORT_RECONFIGURE_LBN: c_int = 3;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_VPORT_RECONFIGURE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_STRIPING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_STRIPING_LBN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_STRIPING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_VADAPTOR_QUERY_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_VADAPTOR_QUERY_LBN: c_int = 5;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_VADAPTOR_QUERY_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_EVB_PORT_VLAN_RESTRICT_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_EVB_PORT_VLAN_RESTRICT_LBN: c_int = 6;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_EVB_PORT_VLAN_RESTRICT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_DRV_ATTACH_PREBOOT_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_DRV_ATTACH_PREBOOT_LBN: c_int = 7;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_DRV_ATTACH_PREBOOT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_FORCE_EVENT_MERGING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_FORCE_EVENT_MERGING_LBN: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_FORCE_EVENT_MERGING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_SET_MAC_ENHANCED_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_SET_MAC_ENHANCED_LBN: c_int = 9;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_SET_MAC_ENHANCED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_UNKNOWN_UCAST_DST_FILTER_ALWAYS_MULTI_RECIPIENT_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_UNKNOWN_UCAST_DST_FILTER_ALWAYS_MULTI_RECIPIENT_LBN: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_UNKNOWN_UCAST_DST_FILTER_ALWAYS_MULTI_RECIPIENT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_VADAPTOR_PERMIT_SET_MAC_WHEN_FILTERS_INSTALLED_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_VADAPTOR_PERMIT_SET_MAC_WHEN_FILTERS_INSTALLED_LBN: c_int = 11;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_VADAPTOR_PERMIT_SET_MAC_WHEN_FILTERS_INSTALLED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_MAC_SECURITY_FILTERING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_MAC_SECURITY_FILTERING_LBN: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_MAC_SECURITY_FILTERING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_ADDITIONAL_RSS_MODES_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_ADDITIONAL_RSS_MODES_LBN: c_int = 13;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_ADDITIONAL_RSS_MODES_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_QBB_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_QBB_LBN: c_int = 14;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_QBB_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_PACKED_STREAM_VAR_BUFFERS_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_PACKED_STREAM_VAR_BUFFERS_LBN: c_int = 15;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_PACKED_STREAM_VAR_BUFFERS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_RSS_LIMITED_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_RSS_LIMITED_LBN: c_int = 16;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_RSS_LIMITED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_PACKED_STREAM_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_PACKED_STREAM_LBN: c_int = 17;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_PACKED_STREAM_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_INCLUDE_FCS_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_INCLUDE_FCS_LBN: c_int = 18;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_INCLUDE_FCS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_VLAN_INSERTION_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_VLAN_INSERTION_LBN: c_int = 19;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_VLAN_INSERTION_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_VLAN_STRIPPING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_VLAN_STRIPPING_LBN: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_VLAN_STRIPPING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_TSO_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_TSO_LBN: c_int = 21;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_TSO_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_PREFIX_LEN_0_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_PREFIX_LEN_0_LBN: c_int = 22;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_PREFIX_LEN_0_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_PREFIX_LEN_14_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_PREFIX_LEN_14_LBN: c_int = 23;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_PREFIX_LEN_14_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_TIMESTAMP_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_TIMESTAMP_LBN: c_int = 24;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_TIMESTAMP_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_BATCHING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_BATCHING_LBN: c_int = 25;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_BATCHING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_MCAST_FILTER_CHAINING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_MCAST_FILTER_CHAINING_LBN: c_int = 26;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_MCAST_FILTER_CHAINING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_PM_AND_RXDP_COUNTERS_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_PM_AND_RXDP_COUNTERS_LBN: c_int = 27;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_PM_AND_RXDP_COUNTERS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_DISABLE_SCATTER_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_DISABLE_SCATTER_LBN: c_int = 28;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_DISABLE_SCATTER_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_MCAST_UDP_LOOPBACK_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_MCAST_UDP_LOOPBACK_LBN: c_int = 29;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_MCAST_UDP_LOOPBACK_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_EVB_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_EVB_LBN: c_int = 30;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_EVB_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_VXLAN_NVGRE_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_VXLAN_NVGRE_LBN: c_int = 31;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_VXLAN_NVGRE_WIDTH: c_int = 1;
// RxDPCPU firmware id.
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_DPCPU_FW_ID_OFST: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_DPCPU_FW_ID_LEN: c_int = 2;
// enum: Standard RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXDP: c_uint = 0x0;
// enum: Low latency RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXDP_LOW_LATENCY: c_uint = 0x1;
// enum: Packed stream RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXDP_PACKED_STREAM: c_uint = 0x2;
// enum: Rules engine RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXDP_RULES_ENGINE: c_uint = 0x5;
// enum: DPDK RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXDP_DPDK: c_uint = 0x6;
// enum: BIST RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXDP_BIST: c_uint = 0x10a;
// enum: RXDP Test firmware image 1
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXDP_TEST_FW_TO_MC_CUT_THROUGH: c_uint = 0x101;
// enum: RXDP Test firmware image 2
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXDP_TEST_FW_TO_MC_STORE_FORWARD: c_uint = 0x102;
// enum: RXDP Test firmware image 3
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXDP_TEST_FW_TO_MC_STORE_FORWARD_FIRST: c_uint = 0x103;
// enum: RXDP Test firmware image 4
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXDP_TEST_EVERY_EVENT_BATCHABLE: c_uint = 0x104;
// enum: RXDP Test firmware image 5
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXDP_TEST_BACKPRESSURE: c_uint = 0x105;
// enum: RXDP Test firmware image 6
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXDP_TEST_FW_PACKET_EDITS: c_uint = 0x106;
// enum: RXDP Test firmware image 7
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXDP_TEST_FW_RX_HDR_SPLIT: c_uint = 0x107;
// enum: RXDP Test firmware image 8
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXDP_TEST_FW_DISABLE_DL: c_uint = 0x108;
// enum: RXDP Test firmware image 9
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXDP_TEST_FW_DOORBELL_DELAY: c_uint = 0x10b;
// enum: RXDP Test firmware image 10
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXDP_TEST_FW_SLOW: c_uint = 0x10c;
// TxDPCPU firmware id.
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_DPCPU_FW_ID_OFST: c_int = 6;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_DPCPU_FW_ID_LEN: c_int = 2;
// enum: Standard TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TXDP: c_uint = 0x0;
// enum: Low latency TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TXDP_LOW_LATENCY: c_uint = 0x1;
// enum: High packet rate TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TXDP_HIGH_PACKET_RATE: c_uint = 0x3;
// enum: Rules engine TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TXDP_RULES_ENGINE: c_uint = 0x5;
// enum: DPDK TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TXDP_DPDK: c_uint = 0x6;
// enum: BIST TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TXDP_BIST: c_uint = 0x12d;
// enum: TXDP Test firmware image 1
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TXDP_TEST_FW_TSO_EDIT: c_uint = 0x101;
// enum: TXDP Test firmware image 2
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TXDP_TEST_FW_PACKET_EDITS: c_uint = 0x102;
// enum: TXDP CSR bus test firmware
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TXDP_TEST_FW_CSR: c_uint = 0x103;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXPD_FW_VERSION_OFST: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXPD_FW_VERSION_LEN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXPD_FW_VERSION_REV_OFST: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXPD_FW_VERSION_REV_LBN: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXPD_FW_VERSION_REV_WIDTH: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXPD_FW_VERSION_TYPE_OFST: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXPD_FW_VERSION_TYPE_LBN: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXPD_FW_VERSION_TYPE_WIDTH: c_int = 4;
// enum: reserved value - do not use (may indicate alternative interpretation
// of REV field in future)
//
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXPD_FW_TYPE_RESERVED: c_uint = 0x0;
// enum: Trivial RX PD firmware for early Huntington development (Huntington
// development only)
//
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXPD_FW_TYPE_FIRST_PKT: c_uint = 0x1;
// enum: RX PD firmware for telemetry prototyping (Medford2 development only)
//
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXPD_FW_TYPE_TESTFW_TELEMETRY: c_uint = 0x1;
// enum: RX PD firmware with approximately Siena-compatible behaviour
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXPD_FW_TYPE_SIENA_COMPAT: c_uint = 0x2;
// enum: Full featured RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXPD_FW_TYPE_FULL_FEATURED: c_uint = 0x3;
// enum: (deprecated original name for the FULL_FEATURED variant)
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXPD_FW_TYPE_VSWITCH: c_uint = 0x3;
// enum: siena_compat variant RX PD firmware using PM rather than MAC
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXPD_FW_TYPE_SIENA_COMPAT_PM: c_uint = 0x4;
// enum: Low latency RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXPD_FW_TYPE_LOW_LATENCY: c_uint = 0x5;
// enum: Packed stream RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXPD_FW_TYPE_PACKED_STREAM: c_uint = 0x6;
// enum: RX PD firmware handling layer 2 only for high packet rate performance
// tests (Medford development only)
//
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXPD_FW_TYPE_LAYER2_PERF: c_uint = 0x7;
// enum: Rules engine RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXPD_FW_TYPE_RULES_ENGINE: c_uint = 0x8;
// enum: Custom firmware variant (see SF-119495-PD and bug69716)
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXPD_FW_TYPE_L3XUDP: c_uint = 0x9;
// enum: DPDK RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXPD_FW_TYPE_DPDK: c_uint = 0xa;
// enum: RX PD firmware for GUE parsing prototype (Medford development only)
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXPD_FW_TYPE_TESTFW_GUE_PROTOTYPE: c_uint = 0xe;
// enum: RX PD firmware parsing but not filtering network overlay tunnel
// encapsulations (Medford development only)
//
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXPD_FW_TYPE_TESTFW_ENCAP_PARSING_ONLY: c_uint = 0xf;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TXPD_FW_VERSION_OFST: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TXPD_FW_VERSION_LEN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TXPD_FW_VERSION_REV_OFST: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TXPD_FW_VERSION_REV_LBN: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TXPD_FW_VERSION_REV_WIDTH: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TXPD_FW_VERSION_TYPE_OFST: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TXPD_FW_VERSION_TYPE_LBN: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TXPD_FW_VERSION_TYPE_WIDTH: c_int = 4;
// enum: reserved value - do not use (may indicate alternative interpretation
// of REV field in future)
//
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TXPD_FW_TYPE_RESERVED: c_uint = 0x0;
// enum: Trivial TX PD firmware for early Huntington development (Huntington
// development only)
//
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TXPD_FW_TYPE_FIRST_PKT: c_uint = 0x1;
// enum: TX PD firmware for telemetry prototyping (Medford2 development only)
//
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TXPD_FW_TYPE_TESTFW_TELEMETRY: c_uint = 0x1;
// enum: TX PD firmware with approximately Siena-compatible behaviour
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TXPD_FW_TYPE_SIENA_COMPAT: c_uint = 0x2;
// enum: Full featured TX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TXPD_FW_TYPE_FULL_FEATURED: c_uint = 0x3;
// enum: (deprecated original name for the FULL_FEATURED variant)
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TXPD_FW_TYPE_VSWITCH: c_uint = 0x3;
// enum: siena_compat variant TX PD firmware using PM rather than MAC
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TXPD_FW_TYPE_SIENA_COMPAT_PM: c_uint = 0x4;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TXPD_FW_TYPE_LOW_LATENCY: c_uint = 0x5 /* enum */;
// enum: TX PD firmware handling layer 2 only for high packet rate performance
// tests (Medford development only)
//
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TXPD_FW_TYPE_LAYER2_PERF: c_uint = 0x7;
// enum: Rules engine TX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TXPD_FW_TYPE_RULES_ENGINE: c_uint = 0x8;
// enum: Custom firmware variant (see SF-119495-PD and bug69716)
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TXPD_FW_TYPE_L3XUDP: c_uint = 0x9;
// enum: DPDK TX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TXPD_FW_TYPE_DPDK: c_uint = 0xa;
// enum: RX PD firmware for GUE parsing prototype (Medford development only)
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TXPD_FW_TYPE_TESTFW_GUE_PROTOTYPE: c_uint = 0xe;
// Hardware capabilities of NIC
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_HW_CAPABILITIES_OFST: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_HW_CAPABILITIES_LEN: c_int = 4;
// Licensed capabilities
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_LICENSE_CAPABILITIES_OFST: c_int = 16;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_LICENSE_CAPABILITIES_LEN: c_int = 4;
// Second word of flags. Not present on older firmware (check the length).
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_FLAGS2_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_FLAGS2_LEN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_TSO_V2_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_TSO_V2_LBN: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_TSO_V2_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_TSO_V2_ENCAP_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_TSO_V2_ENCAP_LBN: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_TSO_V2_ENCAP_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_EVQ_TIMER_CTRL_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_EVQ_TIMER_CTRL_LBN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_EVQ_TIMER_CTRL_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_EVENT_CUT_THROUGH_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_EVENT_CUT_THROUGH_LBN: c_int = 3;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_EVENT_CUT_THROUGH_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_CUT_THROUGH_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_CUT_THROUGH_LBN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_CUT_THROUGH_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_VFIFO_ULL_MODE_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_VFIFO_ULL_MODE_LBN: c_int = 5;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_VFIFO_ULL_MODE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_MAC_STATS_40G_TX_SIZE_BINS_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_MAC_STATS_40G_TX_SIZE_BINS_LBN: c_int = 6;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_MAC_STATS_40G_TX_SIZE_BINS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_INIT_EVQ_TYPE_SUPPORTED_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_INIT_EVQ_TYPE_SUPPORTED_LBN: c_int = 7;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_INIT_EVQ_TYPE_SUPPORTED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_INIT_EVQ_V2_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_INIT_EVQ_V2_LBN: c_int = 7;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_INIT_EVQ_V2_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_MAC_TIMESTAMPING_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_MAC_TIMESTAMPING_LBN: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_MAC_TIMESTAMPING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_TIMESTAMP_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_TIMESTAMP_LBN: c_int = 9;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_TIMESTAMP_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_SNIFF_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_SNIFF_LBN: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_SNIFF_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_SNIFF_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_SNIFF_LBN: c_int = 11;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_SNIFF_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_NVRAM_UPDATE_REPORT_VERIFY_RESULT_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_NVRAM_UPDATE_REPORT_VERIFY_RESULT_LBN: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_NVRAM_UPDATE_REPORT_VERIFY_RESULT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_MCDI_BACKGROUND_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_MCDI_BACKGROUND_LBN: c_int = 13;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_MCDI_BACKGROUND_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_MCDI_DB_RETURN_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_MCDI_DB_RETURN_LBN: c_int = 14;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_MCDI_DB_RETURN_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_CTPIO_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_CTPIO_LBN: c_int = 15;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_CTPIO_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TSA_SUPPORT_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TSA_SUPPORT_LBN: c_int = 16;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TSA_SUPPORT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TSA_BOUND_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TSA_BOUND_LBN: c_int = 17;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TSA_BOUND_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_SF_ADAPTER_AUTHENTICATION_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_SF_ADAPTER_AUTHENTICATION_LBN: c_int = 18;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_SF_ADAPTER_AUTHENTICATION_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_FILTER_ACTION_FLAG_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_FILTER_ACTION_FLAG_LBN: c_int = 19;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_FILTER_ACTION_FLAG_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_FILTER_ACTION_MARK_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_FILTER_ACTION_MARK_LBN: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_FILTER_ACTION_MARK_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_EQUAL_STRIDE_SUPER_BUFFER_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_EQUAL_STRIDE_SUPER_BUFFER_LBN: c_int = 21;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_EQUAL_STRIDE_SUPER_BUFFER_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_EQUAL_STRIDE_PACKED_STREAM_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_EQUAL_STRIDE_PACKED_STREAM_LBN: c_int = 21;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_EQUAL_STRIDE_PACKED_STREAM_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_L3XUDP_SUPPORT_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_L3XUDP_SUPPORT_LBN: c_int = 22;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_L3XUDP_SUPPORT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_FW_SUBVARIANT_NO_TX_CSUM_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_FW_SUBVARIANT_NO_TX_CSUM_LBN: c_int = 23;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_FW_SUBVARIANT_NO_TX_CSUM_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_VI_SPREADING_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_VI_SPREADING_LBN: c_int = 24;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_VI_SPREADING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXDP_HLB_IDLE_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXDP_HLB_IDLE_LBN: c_int = 25;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RXDP_HLB_IDLE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_INIT_RXQ_NO_CONT_EV_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_INIT_RXQ_NO_CONT_EV_LBN: c_int = 26;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_INIT_RXQ_NO_CONT_EV_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_INIT_RXQ_WITH_BUFFER_SIZE_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_INIT_RXQ_WITH_BUFFER_SIZE_LBN: c_int = 27;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_INIT_RXQ_WITH_BUFFER_SIZE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_BUNDLE_UPDATE_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_BUNDLE_UPDATE_LBN: c_int = 28;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_BUNDLE_UPDATE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_TSO_V3_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_TSO_V3_LBN: c_int = 29;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_TSO_V3_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_DYNAMIC_SENSORS_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_DYNAMIC_SENSORS_LBN: c_int = 30;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_DYNAMIC_SENSORS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_NVRAM_UPDATE_POLL_VERIFY_RESULT_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_NVRAM_UPDATE_POLL_VERIFY_RESULT_LBN: c_int = 31;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_NVRAM_UPDATE_POLL_VERIFY_RESULT_WIDTH: c_int = 1;
// Number of FATSOv2 contexts per datapath supported by this NIC (when
// TX_TSO_V2 == 1). Not present on older firmware (check the length).
//
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_TSO_V2_N_CONTEXTS_OFST: c_int = 24;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_TSO_V2_N_CONTEXTS_LEN: c_int = 2;
// One byte per PF containing the number of the external port assigned to this
// PF, indexed by PF number. Special values indicate that a PF is either not
// present or not assigned.
//
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_PFS_TO_PORTS_ASSIGNMENT_OFST: c_int = 26;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_PFS_TO_PORTS_ASSIGNMENT_LEN: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_PFS_TO_PORTS_ASSIGNMENT_NUM: c_int = 16;
// enum: The caller is not permitted to access information on this PF.
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_ACCESS_NOT_PERMITTED: c_uint = 0xff;
// enum: PF does not exist.
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_PF_NOT_PRESENT: c_uint = 0xfe;
// enum: PF does exist but is not assigned to any external port.
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_PF_NOT_ASSIGNED: c_uint = 0xfd;
// enum: This value indicates that PF is assigned, but it cannot be expressed
// in this field. It is intended for a possible future situation where a more
// complex scheme of PFs to ports mapping is being used. The future driver
// should look for a new field supporting the new scheme. The current/old
// driver should treat this value as PF_NOT_ASSIGNED.
//
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_INCOMPATIBLE_ASSIGNMENT: c_uint = 0xfc;
// One byte per PF containing the number of its VFs, indexed by PF number. A
// special value indicates that a PF is not present.
//
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_NUM_VFS_PER_PF_OFST: c_int = 42;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_NUM_VFS_PER_PF_LEN: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_NUM_VFS_PER_PF_NUM: c_int = 16;
// enum: The caller is not permitted to access information on this PF.
// MC_CMD_GET_CAPABILITIES_V8_OUT_ACCESS_NOT_PERMITTED 0xff
// enum: PF does not exist.
// MC_CMD_GET_CAPABILITIES_V8_OUT_PF_NOT_PRESENT 0xfe
// Number of VIs available for each external port
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_NUM_VIS_PER_PORT_OFST: c_int = 58;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_NUM_VIS_PER_PORT_LEN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_NUM_VIS_PER_PORT_NUM: c_int = 4;
// Size of RX descriptor cache expressed as binary logarithm The actual size
// equals (2 ^ RX_DESC_CACHE_SIZE)
//
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_DESC_CACHE_SIZE_OFST: c_int = 66;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_DESC_CACHE_SIZE_LEN: c_int = 1;
// Size of TX descriptor cache expressed as binary logarithm The actual size
// equals (2 ^ TX_DESC_CACHE_SIZE)
//
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_DESC_CACHE_SIZE_OFST: c_int = 67;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TX_DESC_CACHE_SIZE_LEN: c_int = 1;
// Total number of available PIO buffers
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_NUM_PIO_BUFFS_OFST: c_int = 68;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_NUM_PIO_BUFFS_LEN: c_int = 2;
// Size of a single PIO buffer
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_SIZE_PIO_BUFF_OFST: c_int = 70;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_SIZE_PIO_BUFF_LEN: c_int = 2;
// On chips later than Medford the amount of address space assigned to each VI
// is configurable. This is a global setting that the driver must query to
// discover the VI to address mapping. Cut-through PIO (CTPIO) is not available
// with 8k VI windows.
//
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_VI_WINDOW_MODE_OFST: c_int = 72;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_VI_WINDOW_MODE_LEN: c_int = 1;
// enum: Each VI occupies 8k as on Huntington and Medford. PIO is at offset 4k.
// CTPIO is not mapped.
//
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_VI_WINDOW_MODE_8K: c_uint = 0x0;
// enum: Each VI occupies 16k. PIO is at offset 4k. CTPIO is at offset 12k.
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_VI_WINDOW_MODE_16K: c_uint = 0x1;
// enum: Each VI occupies 64k. PIO is at offset 4k. CTPIO is at offset 12k.
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_VI_WINDOW_MODE_64K: c_uint = 0x2;
// Number of vFIFOs per adapter that can be used for VFIFO Stuffing
// (SF-115995-SW) in the present configuration of firmware and port mode.
//
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_VFIFO_STUFFING_NUM_VFIFOS_OFST: c_int = 73;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_VFIFO_STUFFING_NUM_VFIFOS_LEN: c_int = 1;
// Number of buffers per adapter that can be used for VFIFO Stuffing
// (SF-115995-SW) in the present configuration of firmware and port mode.
//
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_VFIFO_STUFFING_NUM_CP_BUFFERS_OFST: c_int = 74;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_VFIFO_STUFFING_NUM_CP_BUFFERS_LEN: c_int = 2;
// Entry count in the MAC stats array, including the final GENERATION_END
// entry. For MAC stats DMA, drivers should allocate a buffer large enough to
// hold at least this many 64-bit stats values, if they wish to receive all
// available stats. If the buffer is shorter than MAC_STATS_NUM_STATS * 8, the
// stats array returned will be truncated.
//
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_MAC_STATS_NUM_STATS_OFST: c_int = 76;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_MAC_STATS_NUM_STATS_LEN: c_int = 2;
// Maximum supported value for MC_CMD_FILTER_OP_V3/MATCH_MARK_VALUE. This field
// will only be non-zero if MC_CMD_GET_CAPABILITIES/FILTER_ACTION_MARK is set.
//
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_FILTER_ACTION_MARK_MAX_OFST: c_int = 80;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_FILTER_ACTION_MARK_MAX_LEN: c_int = 4;
// On devices where the INIT_RXQ_WITH_BUFFER_SIZE flag (in
// GET_CAPABILITIES_OUT_V2) is set, drivers have to specify a buffer size when
// they create an RX queue. Due to hardware limitations, only a small number of
// different buffer sizes may be available concurrently. Nonzero entries in
// this array are the sizes of buffers which the system guarantees will be
// available for use. If the list is empty, there are no limitations on
// concurrent buffer sizes.
//
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_GUARANTEED_RX_BUFFER_SIZES_OFST: c_int = 84;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_GUARANTEED_RX_BUFFER_SIZES_LEN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_GUARANTEED_RX_BUFFER_SIZES_NUM: c_int = 16;
// Third word of flags. Not present on older firmware (check the length).
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_FLAGS3_OFST: c_int = 148;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_FLAGS3_LEN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_WOL_ETHERWAKE_OFST: c_int = 148;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_WOL_ETHERWAKE_LBN: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_WOL_ETHERWAKE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RSS_EVEN_SPREADING_OFST: c_int = 148;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RSS_EVEN_SPREADING_LBN: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RSS_EVEN_SPREADING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RSS_SELECTABLE_TABLE_SIZE_OFST: c_int = 148;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RSS_SELECTABLE_TABLE_SIZE_LBN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RSS_SELECTABLE_TABLE_SIZE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_MAE_SUPPORTED_OFST: c_int = 148;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_MAE_SUPPORTED_LBN: c_int = 3;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_MAE_SUPPORTED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_VDPA_SUPPORTED_OFST: c_int = 148;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_VDPA_SUPPORTED_LBN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_VDPA_SUPPORTED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_VLAN_STRIPPING_PER_ENCAP_RULE_OFST: c_int = 148;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_VLAN_STRIPPING_PER_ENCAP_RULE_LBN: c_int = 5;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_RX_VLAN_STRIPPING_PER_ENCAP_RULE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_EXTENDED_WIDTH_EVQS_SUPPORTED_OFST: c_int = 148;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_EXTENDED_WIDTH_EVQS_SUPPORTED_LBN: c_int = 6;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_EXTENDED_WIDTH_EVQS_SUPPORTED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_UNSOL_EV_CREDIT_SUPPORTED_OFST: c_int = 148;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_UNSOL_EV_CREDIT_SUPPORTED_LBN: c_int = 7;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_UNSOL_EV_CREDIT_SUPPORTED_WIDTH: c_int = 1;
// These bits are reserved for communicating test-specific capabilities to
// host-side test software. All production drivers should treat this field as
// opaque.
//
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TEST_RESERVED_OFST: c_int = 152;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TEST_RESERVED_LEN: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TEST_RESERVED_LO_OFST: c_int = 152;
pub const MC_CMD_GET_CAPABILITIES_V8_OUT_TEST_RESERVED_HI_OFST: c_int = 156;
// MC_CMD_GET_CAPABILITIES_V9_OUT msgresponse
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_LEN: c_int = 184;
// First word of flags.
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_FLAGS1_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_FLAGS1_LEN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_VPORT_RECONFIGURE_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_VPORT_RECONFIGURE_LBN: c_int = 3;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_VPORT_RECONFIGURE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_STRIPING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_STRIPING_LBN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_STRIPING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_VADAPTOR_QUERY_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_VADAPTOR_QUERY_LBN: c_int = 5;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_VADAPTOR_QUERY_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_EVB_PORT_VLAN_RESTRICT_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_EVB_PORT_VLAN_RESTRICT_LBN: c_int = 6;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_EVB_PORT_VLAN_RESTRICT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_DRV_ATTACH_PREBOOT_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_DRV_ATTACH_PREBOOT_LBN: c_int = 7;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_DRV_ATTACH_PREBOOT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_FORCE_EVENT_MERGING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_FORCE_EVENT_MERGING_LBN: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_FORCE_EVENT_MERGING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_SET_MAC_ENHANCED_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_SET_MAC_ENHANCED_LBN: c_int = 9;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_SET_MAC_ENHANCED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_UNKNOWN_UCAST_DST_FILTER_ALWAYS_MULTI_RECIPIENT_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_UNKNOWN_UCAST_DST_FILTER_ALWAYS_MULTI_RECIPIENT_LBN: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_UNKNOWN_UCAST_DST_FILTER_ALWAYS_MULTI_RECIPIENT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_VADAPTOR_PERMIT_SET_MAC_WHEN_FILTERS_INSTALLED_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_VADAPTOR_PERMIT_SET_MAC_WHEN_FILTERS_INSTALLED_LBN: c_int = 11;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_VADAPTOR_PERMIT_SET_MAC_WHEN_FILTERS_INSTALLED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_MAC_SECURITY_FILTERING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_MAC_SECURITY_FILTERING_LBN: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_MAC_SECURITY_FILTERING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_ADDITIONAL_RSS_MODES_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_ADDITIONAL_RSS_MODES_LBN: c_int = 13;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_ADDITIONAL_RSS_MODES_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_QBB_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_QBB_LBN: c_int = 14;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_QBB_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_PACKED_STREAM_VAR_BUFFERS_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_PACKED_STREAM_VAR_BUFFERS_LBN: c_int = 15;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_PACKED_STREAM_VAR_BUFFERS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_RSS_LIMITED_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_RSS_LIMITED_LBN: c_int = 16;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_RSS_LIMITED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_PACKED_STREAM_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_PACKED_STREAM_LBN: c_int = 17;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_PACKED_STREAM_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_INCLUDE_FCS_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_INCLUDE_FCS_LBN: c_int = 18;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_INCLUDE_FCS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_VLAN_INSERTION_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_VLAN_INSERTION_LBN: c_int = 19;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_VLAN_INSERTION_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_VLAN_STRIPPING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_VLAN_STRIPPING_LBN: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_VLAN_STRIPPING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_TSO_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_TSO_LBN: c_int = 21;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_TSO_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_PREFIX_LEN_0_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_PREFIX_LEN_0_LBN: c_int = 22;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_PREFIX_LEN_0_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_PREFIX_LEN_14_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_PREFIX_LEN_14_LBN: c_int = 23;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_PREFIX_LEN_14_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_TIMESTAMP_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_TIMESTAMP_LBN: c_int = 24;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_TIMESTAMP_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_BATCHING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_BATCHING_LBN: c_int = 25;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_BATCHING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_MCAST_FILTER_CHAINING_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_MCAST_FILTER_CHAINING_LBN: c_int = 26;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_MCAST_FILTER_CHAINING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_PM_AND_RXDP_COUNTERS_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_PM_AND_RXDP_COUNTERS_LBN: c_int = 27;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_PM_AND_RXDP_COUNTERS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_DISABLE_SCATTER_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_DISABLE_SCATTER_LBN: c_int = 28;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_DISABLE_SCATTER_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_MCAST_UDP_LOOPBACK_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_MCAST_UDP_LOOPBACK_LBN: c_int = 29;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_MCAST_UDP_LOOPBACK_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_EVB_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_EVB_LBN: c_int = 30;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_EVB_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_VXLAN_NVGRE_OFST: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_VXLAN_NVGRE_LBN: c_int = 31;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_VXLAN_NVGRE_WIDTH: c_int = 1;
// RxDPCPU firmware id.
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_DPCPU_FW_ID_OFST: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_DPCPU_FW_ID_LEN: c_int = 2;
// enum: Standard RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXDP: c_uint = 0x0;
// enum: Low latency RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXDP_LOW_LATENCY: c_uint = 0x1;
// enum: Packed stream RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXDP_PACKED_STREAM: c_uint = 0x2;
// enum: Rules engine RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXDP_RULES_ENGINE: c_uint = 0x5;
// enum: DPDK RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXDP_DPDK: c_uint = 0x6;
// enum: BIST RXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXDP_BIST: c_uint = 0x10a;
// enum: RXDP Test firmware image 1
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXDP_TEST_FW_TO_MC_CUT_THROUGH: c_uint = 0x101;
// enum: RXDP Test firmware image 2
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXDP_TEST_FW_TO_MC_STORE_FORWARD: c_uint = 0x102;
// enum: RXDP Test firmware image 3
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXDP_TEST_FW_TO_MC_STORE_FORWARD_FIRST: c_uint = 0x103;
// enum: RXDP Test firmware image 4
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXDP_TEST_EVERY_EVENT_BATCHABLE: c_uint = 0x104;
// enum: RXDP Test firmware image 5
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXDP_TEST_BACKPRESSURE: c_uint = 0x105;
// enum: RXDP Test firmware image 6
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXDP_TEST_FW_PACKET_EDITS: c_uint = 0x106;
// enum: RXDP Test firmware image 7
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXDP_TEST_FW_RX_HDR_SPLIT: c_uint = 0x107;
// enum: RXDP Test firmware image 8
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXDP_TEST_FW_DISABLE_DL: c_uint = 0x108;
// enum: RXDP Test firmware image 9
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXDP_TEST_FW_DOORBELL_DELAY: c_uint = 0x10b;
// enum: RXDP Test firmware image 10
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXDP_TEST_FW_SLOW: c_uint = 0x10c;
// TxDPCPU firmware id.
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_DPCPU_FW_ID_OFST: c_int = 6;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_DPCPU_FW_ID_LEN: c_int = 2;
// enum: Standard TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TXDP: c_uint = 0x0;
// enum: Low latency TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TXDP_LOW_LATENCY: c_uint = 0x1;
// enum: High packet rate TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TXDP_HIGH_PACKET_RATE: c_uint = 0x3;
// enum: Rules engine TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TXDP_RULES_ENGINE: c_uint = 0x5;
// enum: DPDK TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TXDP_DPDK: c_uint = 0x6;
// enum: BIST TXDP firmware
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TXDP_BIST: c_uint = 0x12d;
// enum: TXDP Test firmware image 1
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TXDP_TEST_FW_TSO_EDIT: c_uint = 0x101;
// enum: TXDP Test firmware image 2
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TXDP_TEST_FW_PACKET_EDITS: c_uint = 0x102;
// enum: TXDP CSR bus test firmware
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TXDP_TEST_FW_CSR: c_uint = 0x103;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXPD_FW_VERSION_OFST: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXPD_FW_VERSION_LEN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXPD_FW_VERSION_REV_OFST: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXPD_FW_VERSION_REV_LBN: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXPD_FW_VERSION_REV_WIDTH: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXPD_FW_VERSION_TYPE_OFST: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXPD_FW_VERSION_TYPE_LBN: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXPD_FW_VERSION_TYPE_WIDTH: c_int = 4;
// enum: reserved value - do not use (may indicate alternative interpretation
// of REV field in future)
//
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXPD_FW_TYPE_RESERVED: c_uint = 0x0;
// enum: Trivial RX PD firmware for early Huntington development (Huntington
// development only)
//
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXPD_FW_TYPE_FIRST_PKT: c_uint = 0x1;
// enum: RX PD firmware for telemetry prototyping (Medford2 development only)
//
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXPD_FW_TYPE_TESTFW_TELEMETRY: c_uint = 0x1;
// enum: RX PD firmware with approximately Siena-compatible behaviour
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXPD_FW_TYPE_SIENA_COMPAT: c_uint = 0x2;
// enum: Full featured RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXPD_FW_TYPE_FULL_FEATURED: c_uint = 0x3;
// enum: (deprecated original name for the FULL_FEATURED variant)
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXPD_FW_TYPE_VSWITCH: c_uint = 0x3;
// enum: siena_compat variant RX PD firmware using PM rather than MAC
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXPD_FW_TYPE_SIENA_COMPAT_PM: c_uint = 0x4;
// enum: Low latency RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXPD_FW_TYPE_LOW_LATENCY: c_uint = 0x5;
// enum: Packed stream RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXPD_FW_TYPE_PACKED_STREAM: c_uint = 0x6;
// enum: RX PD firmware handling layer 2 only for high packet rate performance
// tests (Medford development only)
//
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXPD_FW_TYPE_LAYER2_PERF: c_uint = 0x7;
// enum: Rules engine RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXPD_FW_TYPE_RULES_ENGINE: c_uint = 0x8;
// enum: Custom firmware variant (see SF-119495-PD and bug69716)
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXPD_FW_TYPE_L3XUDP: c_uint = 0x9;
// enum: DPDK RX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXPD_FW_TYPE_DPDK: c_uint = 0xa;
// enum: RX PD firmware for GUE parsing prototype (Medford development only)
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXPD_FW_TYPE_TESTFW_GUE_PROTOTYPE: c_uint = 0xe;
// enum: RX PD firmware parsing but not filtering network overlay tunnel
// encapsulations (Medford development only)
//
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXPD_FW_TYPE_TESTFW_ENCAP_PARSING_ONLY: c_uint = 0xf;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TXPD_FW_VERSION_OFST: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TXPD_FW_VERSION_LEN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TXPD_FW_VERSION_REV_OFST: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TXPD_FW_VERSION_REV_LBN: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TXPD_FW_VERSION_REV_WIDTH: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TXPD_FW_VERSION_TYPE_OFST: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TXPD_FW_VERSION_TYPE_LBN: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TXPD_FW_VERSION_TYPE_WIDTH: c_int = 4;
// enum: reserved value - do not use (may indicate alternative interpretation
// of REV field in future)
//
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TXPD_FW_TYPE_RESERVED: c_uint = 0x0;
// enum: Trivial TX PD firmware for early Huntington development (Huntington
// development only)
//
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TXPD_FW_TYPE_FIRST_PKT: c_uint = 0x1;
// enum: TX PD firmware for telemetry prototyping (Medford2 development only)
//
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TXPD_FW_TYPE_TESTFW_TELEMETRY: c_uint = 0x1;
// enum: TX PD firmware with approximately Siena-compatible behaviour
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TXPD_FW_TYPE_SIENA_COMPAT: c_uint = 0x2;
// enum: Full featured TX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TXPD_FW_TYPE_FULL_FEATURED: c_uint = 0x3;
// enum: (deprecated original name for the FULL_FEATURED variant)
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TXPD_FW_TYPE_VSWITCH: c_uint = 0x3;
// enum: siena_compat variant TX PD firmware using PM rather than MAC
// (Huntington development only)
//
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TXPD_FW_TYPE_SIENA_COMPAT_PM: c_uint = 0x4;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TXPD_FW_TYPE_LOW_LATENCY: c_uint = 0x5 /* enum */;
// enum: TX PD firmware handling layer 2 only for high packet rate performance
// tests (Medford development only)
//
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TXPD_FW_TYPE_LAYER2_PERF: c_uint = 0x7;
// enum: Rules engine TX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TXPD_FW_TYPE_RULES_ENGINE: c_uint = 0x8;
// enum: Custom firmware variant (see SF-119495-PD and bug69716)
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TXPD_FW_TYPE_L3XUDP: c_uint = 0x9;
// enum: DPDK TX PD production firmware
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TXPD_FW_TYPE_DPDK: c_uint = 0xa;
// enum: RX PD firmware for GUE parsing prototype (Medford development only)
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TXPD_FW_TYPE_TESTFW_GUE_PROTOTYPE: c_uint = 0xe;
// Hardware capabilities of NIC
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_HW_CAPABILITIES_OFST: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_HW_CAPABILITIES_LEN: c_int = 4;
// Licensed capabilities
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_LICENSE_CAPABILITIES_OFST: c_int = 16;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_LICENSE_CAPABILITIES_LEN: c_int = 4;
// Second word of flags. Not present on older firmware (check the length).
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_FLAGS2_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_FLAGS2_LEN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_TSO_V2_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_TSO_V2_LBN: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_TSO_V2_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_TSO_V2_ENCAP_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_TSO_V2_ENCAP_LBN: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_TSO_V2_ENCAP_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_EVQ_TIMER_CTRL_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_EVQ_TIMER_CTRL_LBN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_EVQ_TIMER_CTRL_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_EVENT_CUT_THROUGH_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_EVENT_CUT_THROUGH_LBN: c_int = 3;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_EVENT_CUT_THROUGH_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_CUT_THROUGH_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_CUT_THROUGH_LBN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_CUT_THROUGH_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_VFIFO_ULL_MODE_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_VFIFO_ULL_MODE_LBN: c_int = 5;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_VFIFO_ULL_MODE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_MAC_STATS_40G_TX_SIZE_BINS_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_MAC_STATS_40G_TX_SIZE_BINS_LBN: c_int = 6;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_MAC_STATS_40G_TX_SIZE_BINS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_INIT_EVQ_TYPE_SUPPORTED_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_INIT_EVQ_TYPE_SUPPORTED_LBN: c_int = 7;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_INIT_EVQ_TYPE_SUPPORTED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_INIT_EVQ_V2_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_INIT_EVQ_V2_LBN: c_int = 7;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_INIT_EVQ_V2_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_MAC_TIMESTAMPING_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_MAC_TIMESTAMPING_LBN: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_MAC_TIMESTAMPING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_TIMESTAMP_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_TIMESTAMP_LBN: c_int = 9;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_TIMESTAMP_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_SNIFF_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_SNIFF_LBN: c_int = 10;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_SNIFF_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_SNIFF_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_SNIFF_LBN: c_int = 11;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_SNIFF_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_NVRAM_UPDATE_REPORT_VERIFY_RESULT_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_NVRAM_UPDATE_REPORT_VERIFY_RESULT_LBN: c_int = 12;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_NVRAM_UPDATE_REPORT_VERIFY_RESULT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_MCDI_BACKGROUND_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_MCDI_BACKGROUND_LBN: c_int = 13;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_MCDI_BACKGROUND_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_MCDI_DB_RETURN_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_MCDI_DB_RETURN_LBN: c_int = 14;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_MCDI_DB_RETURN_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_CTPIO_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_CTPIO_LBN: c_int = 15;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_CTPIO_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TSA_SUPPORT_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TSA_SUPPORT_LBN: c_int = 16;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TSA_SUPPORT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TSA_BOUND_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TSA_BOUND_LBN: c_int = 17;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TSA_BOUND_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_SF_ADAPTER_AUTHENTICATION_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_SF_ADAPTER_AUTHENTICATION_LBN: c_int = 18;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_SF_ADAPTER_AUTHENTICATION_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_FILTER_ACTION_FLAG_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_FILTER_ACTION_FLAG_LBN: c_int = 19;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_FILTER_ACTION_FLAG_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_FILTER_ACTION_MARK_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_FILTER_ACTION_MARK_LBN: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_FILTER_ACTION_MARK_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_EQUAL_STRIDE_SUPER_BUFFER_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_EQUAL_STRIDE_SUPER_BUFFER_LBN: c_int = 21;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_EQUAL_STRIDE_SUPER_BUFFER_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_EQUAL_STRIDE_PACKED_STREAM_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_EQUAL_STRIDE_PACKED_STREAM_LBN: c_int = 21;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_EQUAL_STRIDE_PACKED_STREAM_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_L3XUDP_SUPPORT_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_L3XUDP_SUPPORT_LBN: c_int = 22;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_L3XUDP_SUPPORT_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_FW_SUBVARIANT_NO_TX_CSUM_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_FW_SUBVARIANT_NO_TX_CSUM_LBN: c_int = 23;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_FW_SUBVARIANT_NO_TX_CSUM_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_VI_SPREADING_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_VI_SPREADING_LBN: c_int = 24;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_VI_SPREADING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXDP_HLB_IDLE_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXDP_HLB_IDLE_LBN: c_int = 25;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RXDP_HLB_IDLE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_INIT_RXQ_NO_CONT_EV_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_INIT_RXQ_NO_CONT_EV_LBN: c_int = 26;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_INIT_RXQ_NO_CONT_EV_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_INIT_RXQ_WITH_BUFFER_SIZE_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_INIT_RXQ_WITH_BUFFER_SIZE_LBN: c_int = 27;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_INIT_RXQ_WITH_BUFFER_SIZE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_BUNDLE_UPDATE_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_BUNDLE_UPDATE_LBN: c_int = 28;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_BUNDLE_UPDATE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_TSO_V3_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_TSO_V3_LBN: c_int = 29;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_TSO_V3_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_DYNAMIC_SENSORS_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_DYNAMIC_SENSORS_LBN: c_int = 30;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_DYNAMIC_SENSORS_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_NVRAM_UPDATE_POLL_VERIFY_RESULT_OFST: c_int = 20;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_NVRAM_UPDATE_POLL_VERIFY_RESULT_LBN: c_int = 31;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_NVRAM_UPDATE_POLL_VERIFY_RESULT_WIDTH: c_int = 1;
// Number of FATSOv2 contexts per datapath supported by this NIC (when
// TX_TSO_V2 == 1). Not present on older firmware (check the length).
//
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_TSO_V2_N_CONTEXTS_OFST: c_int = 24;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_TSO_V2_N_CONTEXTS_LEN: c_int = 2;
// One byte per PF containing the number of the external port assigned to this
// PF, indexed by PF number. Special values indicate that a PF is either not
// present or not assigned.
//
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_PFS_TO_PORTS_ASSIGNMENT_OFST: c_int = 26;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_PFS_TO_PORTS_ASSIGNMENT_LEN: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_PFS_TO_PORTS_ASSIGNMENT_NUM: c_int = 16;
// enum: The caller is not permitted to access information on this PF.
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_ACCESS_NOT_PERMITTED: c_uint = 0xff;
// enum: PF does not exist.
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_PF_NOT_PRESENT: c_uint = 0xfe;
// enum: PF does exist but is not assigned to any external port.
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_PF_NOT_ASSIGNED: c_uint = 0xfd;
// enum: This value indicates that PF is assigned, but it cannot be expressed
// in this field. It is intended for a possible future situation where a more
// complex scheme of PFs to ports mapping is being used. The future driver
// should look for a new field supporting the new scheme. The current/old
// driver should treat this value as PF_NOT_ASSIGNED.
//
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_INCOMPATIBLE_ASSIGNMENT: c_uint = 0xfc;
// One byte per PF containing the number of its VFs, indexed by PF number. A
// special value indicates that a PF is not present.
//
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_NUM_VFS_PER_PF_OFST: c_int = 42;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_NUM_VFS_PER_PF_LEN: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_NUM_VFS_PER_PF_NUM: c_int = 16;
// enum: The caller is not permitted to access information on this PF.
// MC_CMD_GET_CAPABILITIES_V9_OUT_ACCESS_NOT_PERMITTED 0xff
// enum: PF does not exist.
// MC_CMD_GET_CAPABILITIES_V9_OUT_PF_NOT_PRESENT 0xfe
// Number of VIs available for each external port
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_NUM_VIS_PER_PORT_OFST: c_int = 58;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_NUM_VIS_PER_PORT_LEN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_NUM_VIS_PER_PORT_NUM: c_int = 4;
// Size of RX descriptor cache expressed as binary logarithm The actual size
// equals (2 ^ RX_DESC_CACHE_SIZE)
//
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_DESC_CACHE_SIZE_OFST: c_int = 66;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_DESC_CACHE_SIZE_LEN: c_int = 1;
// Size of TX descriptor cache expressed as binary logarithm The actual size
// equals (2 ^ TX_DESC_CACHE_SIZE)
//
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_DESC_CACHE_SIZE_OFST: c_int = 67;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TX_DESC_CACHE_SIZE_LEN: c_int = 1;
// Total number of available PIO buffers
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_NUM_PIO_BUFFS_OFST: c_int = 68;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_NUM_PIO_BUFFS_LEN: c_int = 2;
// Size of a single PIO buffer
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_SIZE_PIO_BUFF_OFST: c_int = 70;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_SIZE_PIO_BUFF_LEN: c_int = 2;
// On chips later than Medford the amount of address space assigned to each VI
// is configurable. This is a global setting that the driver must query to
// discover the VI to address mapping. Cut-through PIO (CTPIO) is not available
// with 8k VI windows.
//
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_VI_WINDOW_MODE_OFST: c_int = 72;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_VI_WINDOW_MODE_LEN: c_int = 1;
// enum: Each VI occupies 8k as on Huntington and Medford. PIO is at offset 4k.
// CTPIO is not mapped.
//
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_VI_WINDOW_MODE_8K: c_uint = 0x0;
// enum: Each VI occupies 16k. PIO is at offset 4k. CTPIO is at offset 12k.
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_VI_WINDOW_MODE_16K: c_uint = 0x1;
// enum: Each VI occupies 64k. PIO is at offset 4k. CTPIO is at offset 12k.
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_VI_WINDOW_MODE_64K: c_uint = 0x2;
// Number of vFIFOs per adapter that can be used for VFIFO Stuffing
// (SF-115995-SW) in the present configuration of firmware and port mode.
//
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_VFIFO_STUFFING_NUM_VFIFOS_OFST: c_int = 73;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_VFIFO_STUFFING_NUM_VFIFOS_LEN: c_int = 1;
// Number of buffers per adapter that can be used for VFIFO Stuffing
// (SF-115995-SW) in the present configuration of firmware and port mode.
//
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_VFIFO_STUFFING_NUM_CP_BUFFERS_OFST: c_int = 74;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_VFIFO_STUFFING_NUM_CP_BUFFERS_LEN: c_int = 2;
// Entry count in the MAC stats array, including the final GENERATION_END
// entry. For MAC stats DMA, drivers should allocate a buffer large enough to
// hold at least this many 64-bit stats values, if they wish to receive all
// available stats. If the buffer is shorter than MAC_STATS_NUM_STATS * 8, the
// stats array returned will be truncated.
//
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_MAC_STATS_NUM_STATS_OFST: c_int = 76;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_MAC_STATS_NUM_STATS_LEN: c_int = 2;
// Maximum supported value for MC_CMD_FILTER_OP_V3/MATCH_MARK_VALUE. This field
// will only be non-zero if MC_CMD_GET_CAPABILITIES/FILTER_ACTION_MARK is set.
//
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_FILTER_ACTION_MARK_MAX_OFST: c_int = 80;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_FILTER_ACTION_MARK_MAX_LEN: c_int = 4;
// On devices where the INIT_RXQ_WITH_BUFFER_SIZE flag (in
// GET_CAPABILITIES_OUT_V2) is set, drivers have to specify a buffer size when
// they create an RX queue. Due to hardware limitations, only a small number of
// different buffer sizes may be available concurrently. Nonzero entries in
// this array are the sizes of buffers which the system guarantees will be
// available for use. If the list is empty, there are no limitations on
// concurrent buffer sizes.
//
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_GUARANTEED_RX_BUFFER_SIZES_OFST: c_int = 84;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_GUARANTEED_RX_BUFFER_SIZES_LEN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_GUARANTEED_RX_BUFFER_SIZES_NUM: c_int = 16;
// Third word of flags. Not present on older firmware (check the length).
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_FLAGS3_OFST: c_int = 148;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_FLAGS3_LEN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_WOL_ETHERWAKE_OFST: c_int = 148;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_WOL_ETHERWAKE_LBN: c_int = 0;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_WOL_ETHERWAKE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RSS_EVEN_SPREADING_OFST: c_int = 148;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RSS_EVEN_SPREADING_LBN: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RSS_EVEN_SPREADING_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RSS_SELECTABLE_TABLE_SIZE_OFST: c_int = 148;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RSS_SELECTABLE_TABLE_SIZE_LBN: c_int = 2;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RSS_SELECTABLE_TABLE_SIZE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_MAE_SUPPORTED_OFST: c_int = 148;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_MAE_SUPPORTED_LBN: c_int = 3;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_MAE_SUPPORTED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_VDPA_SUPPORTED_OFST: c_int = 148;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_VDPA_SUPPORTED_LBN: c_int = 4;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_VDPA_SUPPORTED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_VLAN_STRIPPING_PER_ENCAP_RULE_OFST: c_int = 148;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_VLAN_STRIPPING_PER_ENCAP_RULE_LBN: c_int = 5;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RX_VLAN_STRIPPING_PER_ENCAP_RULE_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_EXTENDED_WIDTH_EVQS_SUPPORTED_OFST: c_int = 148;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_EXTENDED_WIDTH_EVQS_SUPPORTED_LBN: c_int = 6;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_EXTENDED_WIDTH_EVQS_SUPPORTED_WIDTH: c_int = 1;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_UNSOL_EV_CREDIT_SUPPORTED_OFST: c_int = 148;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_UNSOL_EV_CREDIT_SUPPORTED_LBN: c_int = 7;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_UNSOL_EV_CREDIT_SUPPORTED_WIDTH: c_int = 1;
// These bits are reserved for communicating test-specific capabilities to
// host-side test software. All production drivers should treat this field as
// opaque.
//
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TEST_RESERVED_OFST: c_int = 152;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TEST_RESERVED_LEN: c_int = 8;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TEST_RESERVED_LO_OFST: c_int = 152;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_TEST_RESERVED_HI_OFST: c_int = 156;
// The minimum size (in table entries) of indirection table to be allocated
// from the pool for an RSS context. Note that the table size used must be a
// power of 2.
//
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RSS_MIN_INDIRECTION_TABLE_SIZE_OFST: c_int = 160;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RSS_MIN_INDIRECTION_TABLE_SIZE_LEN: c_int = 4;
// The maximum size (in table entries) of indirection table to be allocated
// from the pool for an RSS context. Note that the table size used must be a
// power of 2.
//
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RSS_MAX_INDIRECTION_TABLE_SIZE_OFST: c_int = 164;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RSS_MAX_INDIRECTION_TABLE_SIZE_LEN: c_int = 4;
// The maximum number of queues that can be used by an RSS context in exclusive
// mode. In exclusive mode the context has a configurable indirection table and
// a configurable RSS key.
//
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RSS_MAX_INDIRECTION_QUEUES_OFST: c_int = 168;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RSS_MAX_INDIRECTION_QUEUES_LEN: c_int = 4;
// The maximum number of queues that can be used by an RSS context in even-
// spreading mode. In even-spreading mode the context has no indirection table
// but it does have a configurable RSS key.
//
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RSS_MAX_EVEN_SPREADING_QUEUES_OFST: c_int = 172;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RSS_MAX_EVEN_SPREADING_QUEUES_LEN: c_int = 4;
// The total number of RSS contexts supported. Note that the number of
// available contexts using indirection tables is also limited by the
// availability of indirection table space allocated from a common pool.
//
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RSS_NUM_CONTEXTS_OFST: c_int = 176;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RSS_NUM_CONTEXTS_LEN: c_int = 4;
// The total amount of indirection table space that can be shared between RSS
// contexts.
//
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RSS_TABLE_POOL_SIZE_OFST: c_int = 180;
pub const MC_CMD_GET_CAPABILITIES_V9_OUT_RSS_TABLE_POOL_SIZE_LEN: c_int = 4;
//
// MC_CMD_V2_EXTN
// Encapsulation for a v2 extended command
//
pub const MC_CMD_V2_EXTN: c_uint = 0x7f;
// MC_CMD_V2_EXTN_IN msgrequest
pub const MC_CMD_V2_EXTN_IN_LEN: c_int = 4;
// the extended command number
pub const MC_CMD_V2_EXTN_IN_EXTENDED_CMD_LBN: c_int = 0;
pub const MC_CMD_V2_EXTN_IN_EXTENDED_CMD_WIDTH: c_int = 15;
pub const MC_CMD_V2_EXTN_IN_UNUSED_LBN: c_int = 15;
pub const MC_CMD_V2_EXTN_IN_UNUSED_WIDTH: c_int = 1;
// the actual length of the encapsulated command (which is not in the v1
// header)
//
pub const MC_CMD_V2_EXTN_IN_ACTUAL_LEN_LBN: c_int = 16;
pub const MC_CMD_V2_EXTN_IN_ACTUAL_LEN_WIDTH: c_int = 10;
pub const MC_CMD_V2_EXTN_IN_UNUSED2_LBN: c_int = 26;
pub const MC_CMD_V2_EXTN_IN_UNUSED2_WIDTH: c_int = 2;
// Type of command/response
pub const MC_CMD_V2_EXTN_IN_MESSAGE_TYPE_LBN: c_int = 28;
pub const MC_CMD_V2_EXTN_IN_MESSAGE_TYPE_WIDTH: c_int = 4;
// enum: MCDI command directed to or response originating from the MC.
pub const MC_CMD_V2_EXTN_IN_MCDI_MESSAGE_TYPE_MC: c_uint = 0x0;
// enum: MCDI command directed to a TSA controller. MCDI responses of this type
// are not defined.
//
pub const MC_CMD_V2_EXTN_IN_MCDI_MESSAGE_TYPE_TSA: c_uint = 0x1;
//
// MC_CMD_LINK_PIOBUF
// Link a push I/O buffer to a TxQ
//
pub const MC_CMD_LINK_PIOBUF: c_uint = 0x92;

// MC_CMD_LINK_PIOBUF_IN msgrequest
pub const MC_CMD_LINK_PIOBUF_IN_LEN: c_int = 8;
// Handle for allocated push I/O buffer.
pub const MC_CMD_LINK_PIOBUF_IN_PIOBUF_HANDLE_OFST: c_int = 0;
pub const MC_CMD_LINK_PIOBUF_IN_PIOBUF_HANDLE_LEN: c_int = 4;
// Function Local Instance (VI) number.
pub const MC_CMD_LINK_PIOBUF_IN_TXQ_INSTANCE_OFST: c_int = 4;
pub const MC_CMD_LINK_PIOBUF_IN_TXQ_INSTANCE_LEN: c_int = 4;
// MC_CMD_LINK_PIOBUF_OUT msgresponse
pub const MC_CMD_LINK_PIOBUF_OUT_LEN: c_int = 0;
//
// MC_CMD_UNLINK_PIOBUF
// Unlink a push I/O buffer from a TxQ
//
pub const MC_CMD_UNLINK_PIOBUF: c_uint = 0x93;

// MC_CMD_UNLINK_PIOBUF_IN msgrequest
pub const MC_CMD_UNLINK_PIOBUF_IN_LEN: c_int = 4;
// Function Local Instance (VI) number.
pub const MC_CMD_UNLINK_PIOBUF_IN_TXQ_INSTANCE_OFST: c_int = 0;
pub const MC_CMD_UNLINK_PIOBUF_IN_TXQ_INSTANCE_LEN: c_int = 4;
// MC_CMD_UNLINK_PIOBUF_OUT msgresponse
pub const MC_CMD_UNLINK_PIOBUF_OUT_LEN: c_int = 0;
//
// MC_CMD_VSWITCH_ALLOC
// allocate and initialise a v-switch.
//
pub const MC_CMD_VSWITCH_ALLOC: c_uint = 0x94;

// MC_CMD_VSWITCH_ALLOC_IN msgrequest
pub const MC_CMD_VSWITCH_ALLOC_IN_LEN: c_int = 16;
// The port to connect to the v-switch's upstream port.
pub const MC_CMD_VSWITCH_ALLOC_IN_UPSTREAM_PORT_ID_OFST: c_int = 0;
pub const MC_CMD_VSWITCH_ALLOC_IN_UPSTREAM_PORT_ID_LEN: c_int = 4;
// The type of v-switch to create.
pub const MC_CMD_VSWITCH_ALLOC_IN_TYPE_OFST: c_int = 4;
pub const MC_CMD_VSWITCH_ALLOC_IN_TYPE_LEN: c_int = 4;
// enum: VLAN
pub const MC_CMD_VSWITCH_ALLOC_IN_VSWITCH_TYPE_VLAN: c_uint = 0x1;
// enum: VEB
pub const MC_CMD_VSWITCH_ALLOC_IN_VSWITCH_TYPE_VEB: c_uint = 0x2;
// enum: VEPA (obsolete)
pub const MC_CMD_VSWITCH_ALLOC_IN_VSWITCH_TYPE_VEPA: c_uint = 0x3;
// enum: MUX
pub const MC_CMD_VSWITCH_ALLOC_IN_VSWITCH_TYPE_MUX: c_uint = 0x4;
// enum: Snapper specific; semantics TBD
pub const MC_CMD_VSWITCH_ALLOC_IN_VSWITCH_TYPE_TEST: c_uint = 0x5;
// Flags controlling v-port creation
pub const MC_CMD_VSWITCH_ALLOC_IN_FLAGS_OFST: c_int = 8;
pub const MC_CMD_VSWITCH_ALLOC_IN_FLAGS_LEN: c_int = 4;
pub const MC_CMD_VSWITCH_ALLOC_IN_FLAG_AUTO_PORT_OFST: c_int = 8;
pub const MC_CMD_VSWITCH_ALLOC_IN_FLAG_AUTO_PORT_LBN: c_int = 0;
pub const MC_CMD_VSWITCH_ALLOC_IN_FLAG_AUTO_PORT_WIDTH: c_int = 1;
// The number of VLAN tags to allow for attached v-ports. For VLAN aggregators,
// this must be one or greated, and the attached v-ports must have exactly this
// number of tags. For other v-switch types, this must be zero of greater, and
// is an upper limit on the number of VLAN tags for attached v-ports. An error
// will be returned if existing configuration means we can't support attached
// v-ports with this number of tags.
//
pub const MC_CMD_VSWITCH_ALLOC_IN_NUM_VLAN_TAGS_OFST: c_int = 12;
pub const MC_CMD_VSWITCH_ALLOC_IN_NUM_VLAN_TAGS_LEN: c_int = 4;
// MC_CMD_VSWITCH_ALLOC_OUT msgresponse
pub const MC_CMD_VSWITCH_ALLOC_OUT_LEN: c_int = 0;
//
// MC_CMD_VSWITCH_FREE
// de-allocate a v-switch.
//
pub const MC_CMD_VSWITCH_FREE: c_uint = 0x95;

// MC_CMD_VSWITCH_FREE_IN msgrequest
pub const MC_CMD_VSWITCH_FREE_IN_LEN: c_int = 4;
// The port to which the v-switch is connected.
pub const MC_CMD_VSWITCH_FREE_IN_UPSTREAM_PORT_ID_OFST: c_int = 0;
pub const MC_CMD_VSWITCH_FREE_IN_UPSTREAM_PORT_ID_LEN: c_int = 4;
// MC_CMD_VSWITCH_FREE_OUT msgresponse
pub const MC_CMD_VSWITCH_FREE_OUT_LEN: c_int = 0;
//
// MC_CMD_VSWITCH_QUERY
// read some config of v-switch. For now this command is an empty placeholder.
// It may be used to check if a v-switch is connected to a given EVB port (if
// not, then the command returns ENOENT).
//
pub const MC_CMD_VSWITCH_QUERY: c_uint = 0x63;

// MC_CMD_VSWITCH_QUERY_IN msgrequest
pub const MC_CMD_VSWITCH_QUERY_IN_LEN: c_int = 4;
// The port to which the v-switch is connected.
pub const MC_CMD_VSWITCH_QUERY_IN_UPSTREAM_PORT_ID_OFST: c_int = 0;
pub const MC_CMD_VSWITCH_QUERY_IN_UPSTREAM_PORT_ID_LEN: c_int = 4;
// MC_CMD_VSWITCH_QUERY_OUT msgresponse
pub const MC_CMD_VSWITCH_QUERY_OUT_LEN: c_int = 0;
//
// MC_CMD_VPORT_ALLOC
// allocate a v-port.
//
pub const MC_CMD_VPORT_ALLOC: c_uint = 0x96;

// MC_CMD_VPORT_ALLOC_IN msgrequest
pub const MC_CMD_VPORT_ALLOC_IN_LEN: c_int = 20;
// The port to which the v-switch is connected.
pub const MC_CMD_VPORT_ALLOC_IN_UPSTREAM_PORT_ID_OFST: c_int = 0;
pub const MC_CMD_VPORT_ALLOC_IN_UPSTREAM_PORT_ID_LEN: c_int = 4;
// The type of the new v-port.
pub const MC_CMD_VPORT_ALLOC_IN_TYPE_OFST: c_int = 4;
pub const MC_CMD_VPORT_ALLOC_IN_TYPE_LEN: c_int = 4;
// enum: VLAN (obsolete)
pub const MC_CMD_VPORT_ALLOC_IN_VPORT_TYPE_VLAN: c_uint = 0x1;
// enum: VEB (obsolete)
pub const MC_CMD_VPORT_ALLOC_IN_VPORT_TYPE_VEB: c_uint = 0x2;
// enum: VEPA (obsolete)
pub const MC_CMD_VPORT_ALLOC_IN_VPORT_TYPE_VEPA: c_uint = 0x3;
// enum: A normal v-port receives packets which match a specified MAC and/or
// VLAN.
//
pub const MC_CMD_VPORT_ALLOC_IN_VPORT_TYPE_NORMAL: c_uint = 0x4;
// enum: An expansion v-port packets traffic which don't match any other
// v-port.
//
pub const MC_CMD_VPORT_ALLOC_IN_VPORT_TYPE_EXPANSION: c_uint = 0x5;
// enum: An test v-port receives packets which match any filters installed by
// its downstream components.
//
pub const MC_CMD_VPORT_ALLOC_IN_VPORT_TYPE_TEST: c_uint = 0x6;
// Flags controlling v-port creation
pub const MC_CMD_VPORT_ALLOC_IN_FLAGS_OFST: c_int = 8;
pub const MC_CMD_VPORT_ALLOC_IN_FLAGS_LEN: c_int = 4;
pub const MC_CMD_VPORT_ALLOC_IN_FLAG_AUTO_PORT_OFST: c_int = 8;
pub const MC_CMD_VPORT_ALLOC_IN_FLAG_AUTO_PORT_LBN: c_int = 0;
pub const MC_CMD_VPORT_ALLOC_IN_FLAG_AUTO_PORT_WIDTH: c_int = 1;
pub const MC_CMD_VPORT_ALLOC_IN_FLAG_VLAN_RESTRICT_OFST: c_int = 8;
pub const MC_CMD_VPORT_ALLOC_IN_FLAG_VLAN_RESTRICT_LBN: c_int = 1;
pub const MC_CMD_VPORT_ALLOC_IN_FLAG_VLAN_RESTRICT_WIDTH: c_int = 1;
// The number of VLAN tags to insert/remove. An error will be returned if
// incompatible with the number of VLAN tags specified for the upstream
// v-switch.
//
pub const MC_CMD_VPORT_ALLOC_IN_NUM_VLAN_TAGS_OFST: c_int = 12;
pub const MC_CMD_VPORT_ALLOC_IN_NUM_VLAN_TAGS_LEN: c_int = 4;
// The actual VLAN tags to insert/remove
pub const MC_CMD_VPORT_ALLOC_IN_VLAN_TAGS_OFST: c_int = 16;
pub const MC_CMD_VPORT_ALLOC_IN_VLAN_TAGS_LEN: c_int = 4;
pub const MC_CMD_VPORT_ALLOC_IN_VLAN_TAG_0_OFST: c_int = 16;
pub const MC_CMD_VPORT_ALLOC_IN_VLAN_TAG_0_LBN: c_int = 0;
pub const MC_CMD_VPORT_ALLOC_IN_VLAN_TAG_0_WIDTH: c_int = 16;
pub const MC_CMD_VPORT_ALLOC_IN_VLAN_TAG_1_OFST: c_int = 16;
pub const MC_CMD_VPORT_ALLOC_IN_VLAN_TAG_1_LBN: c_int = 16;
pub const MC_CMD_VPORT_ALLOC_IN_VLAN_TAG_1_WIDTH: c_int = 16;
// MC_CMD_VPORT_ALLOC_OUT msgresponse
pub const MC_CMD_VPORT_ALLOC_OUT_LEN: c_int = 4;
// The handle of the new v-port
pub const MC_CMD_VPORT_ALLOC_OUT_VPORT_ID_OFST: c_int = 0;
pub const MC_CMD_VPORT_ALLOC_OUT_VPORT_ID_LEN: c_int = 4;
//
// MC_CMD_VPORT_FREE
// de-allocate a v-port.
//
pub const MC_CMD_VPORT_FREE: c_uint = 0x97;

// MC_CMD_VPORT_FREE_IN msgrequest
pub const MC_CMD_VPORT_FREE_IN_LEN: c_int = 4;
// The handle of the v-port
pub const MC_CMD_VPORT_FREE_IN_VPORT_ID_OFST: c_int = 0;
pub const MC_CMD_VPORT_FREE_IN_VPORT_ID_LEN: c_int = 4;
// MC_CMD_VPORT_FREE_OUT msgresponse
pub const MC_CMD_VPORT_FREE_OUT_LEN: c_int = 0;
//
// MC_CMD_VADAPTOR_ALLOC
// allocate a v-adaptor.
//
pub const MC_CMD_VADAPTOR_ALLOC: c_uint = 0x98;

// MC_CMD_VADAPTOR_ALLOC_IN msgrequest
pub const MC_CMD_VADAPTOR_ALLOC_IN_LEN: c_int = 30;
// The port to connect to the v-adaptor's port.
pub const MC_CMD_VADAPTOR_ALLOC_IN_UPSTREAM_PORT_ID_OFST: c_int = 0;
pub const MC_CMD_VADAPTOR_ALLOC_IN_UPSTREAM_PORT_ID_LEN: c_int = 4;
// Flags controlling v-adaptor creation
pub const MC_CMD_VADAPTOR_ALLOC_IN_FLAGS_OFST: c_int = 8;
pub const MC_CMD_VADAPTOR_ALLOC_IN_FLAGS_LEN: c_int = 4;
pub const MC_CMD_VADAPTOR_ALLOC_IN_FLAG_AUTO_VADAPTOR_OFST: c_int = 8;
pub const MC_CMD_VADAPTOR_ALLOC_IN_FLAG_AUTO_VADAPTOR_LBN: c_int = 0;
pub const MC_CMD_VADAPTOR_ALLOC_IN_FLAG_AUTO_VADAPTOR_WIDTH: c_int = 1;
pub const MC_CMD_VADAPTOR_ALLOC_IN_FLAG_PERMIT_SET_MAC_WHEN_FILTERS_INSTALLED_OFST: c_int = 8;
pub const MC_CMD_VADAPTOR_ALLOC_IN_FLAG_PERMIT_SET_MAC_WHEN_FILTERS_INSTALLED_LBN: c_int = 1;
pub const MC_CMD_VADAPTOR_ALLOC_IN_FLAG_PERMIT_SET_MAC_WHEN_FILTERS_INSTALLED_WIDTH: c_int = 1;
// The number of VLAN tags to strip on receive
pub const MC_CMD_VADAPTOR_ALLOC_IN_NUM_VLANS_OFST: c_int = 12;
pub const MC_CMD_VADAPTOR_ALLOC_IN_NUM_VLANS_LEN: c_int = 4;
// The number of VLAN tags to transparently insert/remove.
pub const MC_CMD_VADAPTOR_ALLOC_IN_NUM_VLAN_TAGS_OFST: c_int = 16;
pub const MC_CMD_VADAPTOR_ALLOC_IN_NUM_VLAN_TAGS_LEN: c_int = 4;
// The actual VLAN tags to insert/remove
pub const MC_CMD_VADAPTOR_ALLOC_IN_VLAN_TAGS_OFST: c_int = 20;
pub const MC_CMD_VADAPTOR_ALLOC_IN_VLAN_TAGS_LEN: c_int = 4;
pub const MC_CMD_VADAPTOR_ALLOC_IN_VLAN_TAG_0_OFST: c_int = 20;
pub const MC_CMD_VADAPTOR_ALLOC_IN_VLAN_TAG_0_LBN: c_int = 0;
pub const MC_CMD_VADAPTOR_ALLOC_IN_VLAN_TAG_0_WIDTH: c_int = 16;
pub const MC_CMD_VADAPTOR_ALLOC_IN_VLAN_TAG_1_OFST: c_int = 20;
pub const MC_CMD_VADAPTOR_ALLOC_IN_VLAN_TAG_1_LBN: c_int = 16;
pub const MC_CMD_VADAPTOR_ALLOC_IN_VLAN_TAG_1_WIDTH: c_int = 16;
// The MAC address to assign to this v-adaptor
pub const MC_CMD_VADAPTOR_ALLOC_IN_MACADDR_OFST: c_int = 24;
pub const MC_CMD_VADAPTOR_ALLOC_IN_MACADDR_LEN: c_int = 6;
// enum: Derive the MAC address from the upstream port
pub const MC_CMD_VADAPTOR_ALLOC_IN_AUTO_MAC: c_uint = 0x0;
// MC_CMD_VADAPTOR_ALLOC_OUT msgresponse
pub const MC_CMD_VADAPTOR_ALLOC_OUT_LEN: c_int = 0;
//
// MC_CMD_VADAPTOR_FREE
// de-allocate a v-adaptor.
//
pub const MC_CMD_VADAPTOR_FREE: c_uint = 0x99;

// MC_CMD_VADAPTOR_FREE_IN msgrequest
pub const MC_CMD_VADAPTOR_FREE_IN_LEN: c_int = 4;
// The port to which the v-adaptor is connected.
pub const MC_CMD_VADAPTOR_FREE_IN_UPSTREAM_PORT_ID_OFST: c_int = 0;
pub const MC_CMD_VADAPTOR_FREE_IN_UPSTREAM_PORT_ID_LEN: c_int = 4;
// MC_CMD_VADAPTOR_FREE_OUT msgresponse
pub const MC_CMD_VADAPTOR_FREE_OUT_LEN: c_int = 0;
//
// MC_CMD_VADAPTOR_SET_MAC
// assign a new MAC address to a v-adaptor.
//
pub const MC_CMD_VADAPTOR_SET_MAC: c_uint = 0x5d;

// MC_CMD_VADAPTOR_SET_MAC_IN msgrequest
pub const MC_CMD_VADAPTOR_SET_MAC_IN_LEN: c_int = 10;
// The port to which the v-adaptor is connected.
pub const MC_CMD_VADAPTOR_SET_MAC_IN_UPSTREAM_PORT_ID_OFST: c_int = 0;
pub const MC_CMD_VADAPTOR_SET_MAC_IN_UPSTREAM_PORT_ID_LEN: c_int = 4;
// The new MAC address to assign to this v-adaptor
pub const MC_CMD_VADAPTOR_SET_MAC_IN_MACADDR_OFST: c_int = 4;
pub const MC_CMD_VADAPTOR_SET_MAC_IN_MACADDR_LEN: c_int = 6;
// MC_CMD_VADAPTOR_SET_MAC_OUT msgresponse
pub const MC_CMD_VADAPTOR_SET_MAC_OUT_LEN: c_int = 0;
//
// MC_CMD_VADAPTOR_GET_MAC
// read the MAC address assigned to a v-adaptor.
//
pub const MC_CMD_VADAPTOR_GET_MAC: c_uint = 0x5e;

// MC_CMD_VADAPTOR_GET_MAC_IN msgrequest
pub const MC_CMD_VADAPTOR_GET_MAC_IN_LEN: c_int = 4;
// The port to which the v-adaptor is connected.
pub const MC_CMD_VADAPTOR_GET_MAC_IN_UPSTREAM_PORT_ID_OFST: c_int = 0;
pub const MC_CMD_VADAPTOR_GET_MAC_IN_UPSTREAM_PORT_ID_LEN: c_int = 4;
// MC_CMD_VADAPTOR_GET_MAC_OUT msgresponse
pub const MC_CMD_VADAPTOR_GET_MAC_OUT_LEN: c_int = 6;
// The MAC address assigned to this v-adaptor
pub const MC_CMD_VADAPTOR_GET_MAC_OUT_MACADDR_OFST: c_int = 0;
pub const MC_CMD_VADAPTOR_GET_MAC_OUT_MACADDR_LEN: c_int = 6;
//
// MC_CMD_VADAPTOR_QUERY
// read some config of v-adaptor.
//
pub const MC_CMD_VADAPTOR_QUERY: c_uint = 0x61;

// MC_CMD_VADAPTOR_QUERY_IN msgrequest
pub const MC_CMD_VADAPTOR_QUERY_IN_LEN: c_int = 4;
// The port to which the v-adaptor is connected.
pub const MC_CMD_VADAPTOR_QUERY_IN_UPSTREAM_PORT_ID_OFST: c_int = 0;
pub const MC_CMD_VADAPTOR_QUERY_IN_UPSTREAM_PORT_ID_LEN: c_int = 4;
// MC_CMD_VADAPTOR_QUERY_OUT msgresponse
pub const MC_CMD_VADAPTOR_QUERY_OUT_LEN: c_int = 12;
// The EVB port flags as defined at MC_CMD_VPORT_ALLOC.
pub const MC_CMD_VADAPTOR_QUERY_OUT_PORT_FLAGS_OFST: c_int = 0;
pub const MC_CMD_VADAPTOR_QUERY_OUT_PORT_FLAGS_LEN: c_int = 4;
// The v-adaptor flags as defined at MC_CMD_VADAPTOR_ALLOC.
pub const MC_CMD_VADAPTOR_QUERY_OUT_VADAPTOR_FLAGS_OFST: c_int = 4;
pub const MC_CMD_VADAPTOR_QUERY_OUT_VADAPTOR_FLAGS_LEN: c_int = 4;
// The number of VLAN tags that may still be added
pub const MC_CMD_VADAPTOR_QUERY_OUT_NUM_AVAILABLE_VLAN_TAGS_OFST: c_int = 8;
pub const MC_CMD_VADAPTOR_QUERY_OUT_NUM_AVAILABLE_VLAN_TAGS_LEN: c_int = 4;
//
// MC_CMD_EVB_PORT_ASSIGN
// assign a port to a PCI function.
//
pub const MC_CMD_EVB_PORT_ASSIGN: c_uint = 0x9a;

// MC_CMD_EVB_PORT_ASSIGN_IN msgrequest
pub const MC_CMD_EVB_PORT_ASSIGN_IN_LEN: c_int = 8;
// The port to assign.
pub const MC_CMD_EVB_PORT_ASSIGN_IN_PORT_ID_OFST: c_int = 0;
pub const MC_CMD_EVB_PORT_ASSIGN_IN_PORT_ID_LEN: c_int = 4;
// The target function to modify.
pub const MC_CMD_EVB_PORT_ASSIGN_IN_FUNCTION_OFST: c_int = 4;
pub const MC_CMD_EVB_PORT_ASSIGN_IN_FUNCTION_LEN: c_int = 4;
pub const MC_CMD_EVB_PORT_ASSIGN_IN_PF_OFST: c_int = 4;
pub const MC_CMD_EVB_PORT_ASSIGN_IN_PF_LBN: c_int = 0;
pub const MC_CMD_EVB_PORT_ASSIGN_IN_PF_WIDTH: c_int = 16;
pub const MC_CMD_EVB_PORT_ASSIGN_IN_VF_OFST: c_int = 4;
pub const MC_CMD_EVB_PORT_ASSIGN_IN_VF_LBN: c_int = 16;
pub const MC_CMD_EVB_PORT_ASSIGN_IN_VF_WIDTH: c_int = 16;
// MC_CMD_EVB_PORT_ASSIGN_OUT msgresponse
pub const MC_CMD_EVB_PORT_ASSIGN_OUT_LEN: c_int = 0;
//
// MC_CMD_RDWR_A64_REGIONS
// Assign the 64 bit region addresses.
//
pub const MC_CMD_RDWR_A64_REGIONS: c_uint = 0x9b;

// MC_CMD_RDWR_A64_REGIONS_IN msgrequest
pub const MC_CMD_RDWR_A64_REGIONS_IN_LEN: c_int = 17;
pub const MC_CMD_RDWR_A64_REGIONS_IN_REGION0_OFST: c_int = 0;
pub const MC_CMD_RDWR_A64_REGIONS_IN_REGION0_LEN: c_int = 4;
pub const MC_CMD_RDWR_A64_REGIONS_IN_REGION1_OFST: c_int = 4;
pub const MC_CMD_RDWR_A64_REGIONS_IN_REGION1_LEN: c_int = 4;
pub const MC_CMD_RDWR_A64_REGIONS_IN_REGION2_OFST: c_int = 8;
pub const MC_CMD_RDWR_A64_REGIONS_IN_REGION2_LEN: c_int = 4;
pub const MC_CMD_RDWR_A64_REGIONS_IN_REGION3_OFST: c_int = 12;
pub const MC_CMD_RDWR_A64_REGIONS_IN_REGION3_LEN: c_int = 4;
// Write enable bits 0-3, set to write, clear to read.
pub const MC_CMD_RDWR_A64_REGIONS_IN_WRITE_MASK_LBN: c_int = 128;
pub const MC_CMD_RDWR_A64_REGIONS_IN_WRITE_MASK_WIDTH: c_int = 4;
pub const MC_CMD_RDWR_A64_REGIONS_IN_WRITE_MASK_BYTE_OFST: c_int = 16;
pub const MC_CMD_RDWR_A64_REGIONS_IN_WRITE_MASK_BYTE_LEN: c_int = 1;
// MC_CMD_RDWR_A64_REGIONS_OUT msgresponse: This data always included
// regardless of state of write bits in the request.
//
pub const MC_CMD_RDWR_A64_REGIONS_OUT_LEN: c_int = 16;
pub const MC_CMD_RDWR_A64_REGIONS_OUT_REGION0_OFST: c_int = 0;
pub const MC_CMD_RDWR_A64_REGIONS_OUT_REGION0_LEN: c_int = 4;
pub const MC_CMD_RDWR_A64_REGIONS_OUT_REGION1_OFST: c_int = 4;
pub const MC_CMD_RDWR_A64_REGIONS_OUT_REGION1_LEN: c_int = 4;
pub const MC_CMD_RDWR_A64_REGIONS_OUT_REGION2_OFST: c_int = 8;
pub const MC_CMD_RDWR_A64_REGIONS_OUT_REGION2_LEN: c_int = 4;
pub const MC_CMD_RDWR_A64_REGIONS_OUT_REGION3_OFST: c_int = 12;
pub const MC_CMD_RDWR_A64_REGIONS_OUT_REGION3_LEN: c_int = 4;
//
// MC_CMD_ONLOAD_STACK_ALLOC
// Allocate an Onload stack ID.
//
pub const MC_CMD_ONLOAD_STACK_ALLOC: c_uint = 0x9c;

// MC_CMD_ONLOAD_STACK_ALLOC_IN msgrequest
pub const MC_CMD_ONLOAD_STACK_ALLOC_IN_LEN: c_int = 4;
// The handle of the owning upstream port
pub const MC_CMD_ONLOAD_STACK_ALLOC_IN_UPSTREAM_PORT_ID_OFST: c_int = 0;
pub const MC_CMD_ONLOAD_STACK_ALLOC_IN_UPSTREAM_PORT_ID_LEN: c_int = 4;
// MC_CMD_ONLOAD_STACK_ALLOC_OUT msgresponse
pub const MC_CMD_ONLOAD_STACK_ALLOC_OUT_LEN: c_int = 4;
// The handle of the new Onload stack
pub const MC_CMD_ONLOAD_STACK_ALLOC_OUT_ONLOAD_STACK_ID_OFST: c_int = 0;
pub const MC_CMD_ONLOAD_STACK_ALLOC_OUT_ONLOAD_STACK_ID_LEN: c_int = 4;
//
// MC_CMD_ONLOAD_STACK_FREE
// Free an Onload stack ID.
//
pub const MC_CMD_ONLOAD_STACK_FREE: c_uint = 0x9d;

// MC_CMD_ONLOAD_STACK_FREE_IN msgrequest
pub const MC_CMD_ONLOAD_STACK_FREE_IN_LEN: c_int = 4;
// The handle of the Onload stack
pub const MC_CMD_ONLOAD_STACK_FREE_IN_ONLOAD_STACK_ID_OFST: c_int = 0;
pub const MC_CMD_ONLOAD_STACK_FREE_IN_ONLOAD_STACK_ID_LEN: c_int = 4;
// MC_CMD_ONLOAD_STACK_FREE_OUT msgresponse
pub const MC_CMD_ONLOAD_STACK_FREE_OUT_LEN: c_int = 0;
//
// MC_CMD_RSS_CONTEXT_ALLOC
// Allocate an RSS context.
//
pub const MC_CMD_RSS_CONTEXT_ALLOC: c_uint = 0x9e;

// MC_CMD_RSS_CONTEXT_ALLOC_IN msgrequest
pub const MC_CMD_RSS_CONTEXT_ALLOC_IN_LEN: c_int = 12;
// The handle of the owning upstream port
pub const MC_CMD_RSS_CONTEXT_ALLOC_IN_UPSTREAM_PORT_ID_OFST: c_int = 0;
pub const MC_CMD_RSS_CONTEXT_ALLOC_IN_UPSTREAM_PORT_ID_LEN: c_int = 4;
// The type of context to allocate
pub const MC_CMD_RSS_CONTEXT_ALLOC_IN_TYPE_OFST: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_ALLOC_IN_TYPE_LEN: c_int = 4;
// enum: Allocate a context for exclusive use. The key and indirection table
// must be explicitly configured.
//
pub const MC_CMD_RSS_CONTEXT_ALLOC_IN_TYPE_EXCLUSIVE: c_uint = 0x0;
// enum: Allocate a context for shared use; this will spread across a range of
// queues, but the key and indirection table are pre-configured and may not be
// changed. For this mode, NUM_QUEUES must 2, 4, 8, 16, 32 or 64.
//
pub const MC_CMD_RSS_CONTEXT_ALLOC_IN_TYPE_SHARED: c_uint = 0x1;
// enum: Allocate a context to spread evenly across an arbitrary number of
// queues. No indirection table space is allocated for this context. (EF100 and
// later)
//
pub const MC_CMD_RSS_CONTEXT_ALLOC_IN_TYPE_EVEN_SPREADING: c_uint = 0x2;
// Number of queues spanned by this context. For exclusive contexts this must
// be in the range 1 to RSS_MAX_INDIRECTION_QUEUES, where
// RSS_MAX_INDIRECTION_QUEUES is queried from MC_CMD_GET_CAPABILITIES_V9 or if
// V9 is not supported then RSS_MAX_INDIRECTION_QUEUES is 64. Valid entries in
// the indirection table will be in the range 0 to NUM_QUEUES-1. For even-
// spreading contexts this must be in the range 1 to
// RSS_MAX_EVEN_SPREADING_QUEUES as queried from MC_CMD_GET_CAPABILITIES. Note
// that specifying NUM_QUEUES = 1 will not perform any spreading but may still
// be useful as a way of obtaining the Toeplitz hash.
//
pub const MC_CMD_RSS_CONTEXT_ALLOC_IN_NUM_QUEUES_OFST: c_int = 8;
pub const MC_CMD_RSS_CONTEXT_ALLOC_IN_NUM_QUEUES_LEN: c_int = 4;
// MC_CMD_RSS_CONTEXT_ALLOC_V2_IN msgrequest
pub const MC_CMD_RSS_CONTEXT_ALLOC_V2_IN_LEN: c_int = 16;
// The handle of the owning upstream port
pub const MC_CMD_RSS_CONTEXT_ALLOC_V2_IN_UPSTREAM_PORT_ID_OFST: c_int = 0;
pub const MC_CMD_RSS_CONTEXT_ALLOC_V2_IN_UPSTREAM_PORT_ID_LEN: c_int = 4;
// The type of context to allocate
pub const MC_CMD_RSS_CONTEXT_ALLOC_V2_IN_TYPE_OFST: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_ALLOC_V2_IN_TYPE_LEN: c_int = 4;
// enum: Allocate a context for exclusive use. The key and indirection table
// must be explicitly configured.
//
pub const MC_CMD_RSS_CONTEXT_ALLOC_V2_IN_TYPE_EXCLUSIVE: c_uint = 0x0;
// enum: Allocate a context for shared use; this will spread across a range of
// queues, but the key and indirection table are pre-configured and may not be
// changed. For this mode, NUM_QUEUES must 2, 4, 8, 16, 32 or 64.
//
pub const MC_CMD_RSS_CONTEXT_ALLOC_V2_IN_TYPE_SHARED: c_uint = 0x1;
// enum: Allocate a context to spread evenly across an arbitrary number of
// queues. No indirection table space is allocated for this context. (EF100 and
// later)
//
pub const MC_CMD_RSS_CONTEXT_ALLOC_V2_IN_TYPE_EVEN_SPREADING: c_uint = 0x2;
// Number of queues spanned by this context. For exclusive contexts this must
// be in the range 1 to RSS_MAX_INDIRECTION_QUEUES, where
// RSS_MAX_INDIRECTION_QUEUES is queried from MC_CMD_GET_CAPABILITIES_V9 or if
// V9 is not supported then RSS_MAX_INDIRECTION_QUEUES is 64. Valid entries in
// the indirection table will be in the range 0 to NUM_QUEUES-1. For even-
// spreading contexts this must be in the range 1 to
// RSS_MAX_EVEN_SPREADING_QUEUES as queried from MC_CMD_GET_CAPABILITIES. Note
// that specifying NUM_QUEUES = 1 will not perform any spreading but may still
// be useful as a way of obtaining the Toeplitz hash.
//
pub const MC_CMD_RSS_CONTEXT_ALLOC_V2_IN_NUM_QUEUES_OFST: c_int = 8;
pub const MC_CMD_RSS_CONTEXT_ALLOC_V2_IN_NUM_QUEUES_LEN: c_int = 4;
// Size of indirection table to be allocated to this context from the pool.
// Must be a power of 2. The minimum and maximum table size can be queried
// using MC_CMD_GET_CAPABILITIES_V9. If there is not enough space remaining in
// the common pool to allocate the requested table size, due to allocating
// table space to other RSS contexts, then the command will fail with
// MC_CMD_ERR_ENOSPC.
//
pub const MC_CMD_RSS_CONTEXT_ALLOC_V2_IN_INDIRECTION_TABLE_SIZE_OFST: c_int = 12;
pub const MC_CMD_RSS_CONTEXT_ALLOC_V2_IN_INDIRECTION_TABLE_SIZE_LEN: c_int = 4;
// MC_CMD_RSS_CONTEXT_ALLOC_OUT msgresponse
pub const MC_CMD_RSS_CONTEXT_ALLOC_OUT_LEN: c_int = 4;
// The handle of the new RSS context. This should be considered opaque to the
// host, although a value of 0xFFFFFFFF is guaranteed never to be a valid
// handle.
//
pub const MC_CMD_RSS_CONTEXT_ALLOC_OUT_RSS_CONTEXT_ID_OFST: c_int = 0;
pub const MC_CMD_RSS_CONTEXT_ALLOC_OUT_RSS_CONTEXT_ID_LEN: c_int = 4;
// enum: guaranteed invalid RSS context handle value
pub const MC_CMD_RSS_CONTEXT_ALLOC_OUT_RSS_CONTEXT_ID_INVALID: c_uint = 0xffffffff;
//
// MC_CMD_RSS_CONTEXT_FREE
// Free an RSS context.
//
pub const MC_CMD_RSS_CONTEXT_FREE: c_uint = 0x9f;

// MC_CMD_RSS_CONTEXT_FREE_IN msgrequest
pub const MC_CMD_RSS_CONTEXT_FREE_IN_LEN: c_int = 4;
// The handle of the RSS context
pub const MC_CMD_RSS_CONTEXT_FREE_IN_RSS_CONTEXT_ID_OFST: c_int = 0;
pub const MC_CMD_RSS_CONTEXT_FREE_IN_RSS_CONTEXT_ID_LEN: c_int = 4;
// MC_CMD_RSS_CONTEXT_FREE_OUT msgresponse
pub const MC_CMD_RSS_CONTEXT_FREE_OUT_LEN: c_int = 0;
//
// MC_CMD_RSS_CONTEXT_SET_KEY
// Set the Toeplitz hash key for an RSS context.
//
pub const MC_CMD_RSS_CONTEXT_SET_KEY: c_uint = 0xa0;

// MC_CMD_RSS_CONTEXT_SET_KEY_IN msgrequest
pub const MC_CMD_RSS_CONTEXT_SET_KEY_IN_LEN: c_int = 44;
// The handle of the RSS context
pub const MC_CMD_RSS_CONTEXT_SET_KEY_IN_RSS_CONTEXT_ID_OFST: c_int = 0;
pub const MC_CMD_RSS_CONTEXT_SET_KEY_IN_RSS_CONTEXT_ID_LEN: c_int = 4;
// The 40-byte Toeplitz hash key (TBD endianness issues?)
pub const MC_CMD_RSS_CONTEXT_SET_KEY_IN_TOEPLITZ_KEY_OFST: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_SET_KEY_IN_TOEPLITZ_KEY_LEN: c_int = 40;
// MC_CMD_RSS_CONTEXT_SET_KEY_OUT msgresponse
pub const MC_CMD_RSS_CONTEXT_SET_KEY_OUT_LEN: c_int = 0;
//
// MC_CMD_RSS_CONTEXT_GET_KEY
// Get the Toeplitz hash key for an RSS context.
//
pub const MC_CMD_RSS_CONTEXT_GET_KEY: c_uint = 0xa1;

// MC_CMD_RSS_CONTEXT_GET_KEY_IN msgrequest
pub const MC_CMD_RSS_CONTEXT_GET_KEY_IN_LEN: c_int = 4;
// The handle of the RSS context
pub const MC_CMD_RSS_CONTEXT_GET_KEY_IN_RSS_CONTEXT_ID_OFST: c_int = 0;
pub const MC_CMD_RSS_CONTEXT_GET_KEY_IN_RSS_CONTEXT_ID_LEN: c_int = 4;
// MC_CMD_RSS_CONTEXT_GET_KEY_OUT msgresponse
pub const MC_CMD_RSS_CONTEXT_GET_KEY_OUT_LEN: c_int = 44;
// The 40-byte Toeplitz hash key (TBD endianness issues?)
pub const MC_CMD_RSS_CONTEXT_GET_KEY_OUT_TOEPLITZ_KEY_OFST: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_GET_KEY_OUT_TOEPLITZ_KEY_LEN: c_int = 40;
//
// MC_CMD_RSS_CONTEXT_SET_TABLE
// Set the indirection table for an RSS context. This command should only be
// used with indirection tables containing 128 entries, which is the default
// when the RSS context is allocated without specifying a table size.
//
pub const MC_CMD_RSS_CONTEXT_SET_TABLE: c_uint = 0xa2;

// MC_CMD_RSS_CONTEXT_SET_TABLE_IN msgrequest
pub const MC_CMD_RSS_CONTEXT_SET_TABLE_IN_LEN: c_int = 132;
// The handle of the RSS context
pub const MC_CMD_RSS_CONTEXT_SET_TABLE_IN_RSS_CONTEXT_ID_OFST: c_int = 0;
pub const MC_CMD_RSS_CONTEXT_SET_TABLE_IN_RSS_CONTEXT_ID_LEN: c_int = 4;
// The 128-byte indirection table (1 byte per entry)
pub const MC_CMD_RSS_CONTEXT_SET_TABLE_IN_INDIRECTION_TABLE_OFST: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_SET_TABLE_IN_INDIRECTION_TABLE_LEN: c_int = 128;
// MC_CMD_RSS_CONTEXT_SET_TABLE_OUT msgresponse
pub const MC_CMD_RSS_CONTEXT_SET_TABLE_OUT_LEN: c_int = 0;
//
// MC_CMD_RSS_CONTEXT_GET_TABLE
// Get the indirection table for an RSS context. This command should only be
// used with indirection tables containing 128 entries, which is the default
// when the RSS context is allocated without specifying a table size.
//
pub const MC_CMD_RSS_CONTEXT_GET_TABLE: c_uint = 0xa3;

// MC_CMD_RSS_CONTEXT_GET_TABLE_IN msgrequest
pub const MC_CMD_RSS_CONTEXT_GET_TABLE_IN_LEN: c_int = 4;
// The handle of the RSS context
pub const MC_CMD_RSS_CONTEXT_GET_TABLE_IN_RSS_CONTEXT_ID_OFST: c_int = 0;
pub const MC_CMD_RSS_CONTEXT_GET_TABLE_IN_RSS_CONTEXT_ID_LEN: c_int = 4;
// MC_CMD_RSS_CONTEXT_GET_TABLE_OUT msgresponse
pub const MC_CMD_RSS_CONTEXT_GET_TABLE_OUT_LEN: c_int = 132;
// The 128-byte indirection table (1 byte per entry)
pub const MC_CMD_RSS_CONTEXT_GET_TABLE_OUT_INDIRECTION_TABLE_OFST: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_GET_TABLE_OUT_INDIRECTION_TABLE_LEN: c_int = 128;
//
// MC_CMD_RSS_CONTEXT_WRITE_TABLE
// Write a portion of a selectable-size indirection table for an RSS context.
// This command must be used instead of MC_CMD_RSS_CONTEXT_SET_TABLE if the
// RSS_SELECTABLE_TABLE_SIZE bit is set in MC_CMD_GET_CAPABILITIES.
//
pub const MC_CMD_RSS_CONTEXT_WRITE_TABLE: c_uint = 0x13e;

// MC_CMD_RSS_CONTEXT_WRITE_TABLE_IN msgrequest
pub const MC_CMD_RSS_CONTEXT_WRITE_TABLE_IN_LENMIN: c_int = 8;
pub const MC_CMD_RSS_CONTEXT_WRITE_TABLE_IN_LENMAX: c_int = 252;
pub const MC_CMD_RSS_CONTEXT_WRITE_TABLE_IN_LENMAX_MCDI2: c_int = 1020;

// The handle of the RSS context
pub const MC_CMD_RSS_CONTEXT_WRITE_TABLE_IN_RSS_CONTEXT_ID_OFST: c_int = 0;
pub const MC_CMD_RSS_CONTEXT_WRITE_TABLE_IN_RSS_CONTEXT_ID_LEN: c_int = 4;
// An array of index-value pairs to be written to the table. Structure is
// MC_CMD_RSS_CONTEXT_WRITE_TABLE_ENTRY.
//
pub const MC_CMD_RSS_CONTEXT_WRITE_TABLE_IN_ENTRIES_OFST: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_WRITE_TABLE_IN_ENTRIES_LEN: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_WRITE_TABLE_IN_ENTRIES_MINNUM: c_int = 1;
pub const MC_CMD_RSS_CONTEXT_WRITE_TABLE_IN_ENTRIES_MAXNUM: c_int = 62;
pub const MC_CMD_RSS_CONTEXT_WRITE_TABLE_IN_ENTRIES_MAXNUM_MCDI2: c_int = 254;
// MC_CMD_RSS_CONTEXT_WRITE_TABLE_OUT msgresponse
pub const MC_CMD_RSS_CONTEXT_WRITE_TABLE_OUT_LEN: c_int = 0;
// MC_CMD_RSS_CONTEXT_WRITE_TABLE_ENTRY structuredef
pub const MC_CMD_RSS_CONTEXT_WRITE_TABLE_ENTRY_LEN: c_int = 4;
// The index of the table entry to be written.
pub const MC_CMD_RSS_CONTEXT_WRITE_TABLE_ENTRY_INDEX_OFST: c_int = 0;
pub const MC_CMD_RSS_CONTEXT_WRITE_TABLE_ENTRY_INDEX_LEN: c_int = 2;
pub const MC_CMD_RSS_CONTEXT_WRITE_TABLE_ENTRY_INDEX_LBN: c_int = 0;
pub const MC_CMD_RSS_CONTEXT_WRITE_TABLE_ENTRY_INDEX_WIDTH: c_int = 16;
// The value to write into the table entry.
pub const MC_CMD_RSS_CONTEXT_WRITE_TABLE_ENTRY_VALUE_OFST: c_int = 2;
pub const MC_CMD_RSS_CONTEXT_WRITE_TABLE_ENTRY_VALUE_LEN: c_int = 2;
pub const MC_CMD_RSS_CONTEXT_WRITE_TABLE_ENTRY_VALUE_LBN: c_int = 16;
pub const MC_CMD_RSS_CONTEXT_WRITE_TABLE_ENTRY_VALUE_WIDTH: c_int = 16;
//
// MC_CMD_RSS_CONTEXT_READ_TABLE
// Read a portion of a selectable-size indirection table for an RSS context.
// This command must be used instead of MC_CMD_RSS_CONTEXT_GET_TABLE if the
// RSS_SELECTABLE_TABLE_SIZE bit is set in MC_CMD_GET_CAPABILITIES.
//
pub const MC_CMD_RSS_CONTEXT_READ_TABLE: c_uint = 0x13f;

// MC_CMD_RSS_CONTEXT_READ_TABLE_IN msgrequest
pub const MC_CMD_RSS_CONTEXT_READ_TABLE_IN_LENMIN: c_int = 6;
pub const MC_CMD_RSS_CONTEXT_READ_TABLE_IN_LENMAX: c_int = 252;
pub const MC_CMD_RSS_CONTEXT_READ_TABLE_IN_LENMAX_MCDI2: c_int = 1020;

// The handle of the RSS context
pub const MC_CMD_RSS_CONTEXT_READ_TABLE_IN_RSS_CONTEXT_ID_OFST: c_int = 0;
pub const MC_CMD_RSS_CONTEXT_READ_TABLE_IN_RSS_CONTEXT_ID_LEN: c_int = 4;
// An array containing the indices of the entries to be read.
pub const MC_CMD_RSS_CONTEXT_READ_TABLE_IN_INDICES_OFST: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_READ_TABLE_IN_INDICES_LEN: c_int = 2;
pub const MC_CMD_RSS_CONTEXT_READ_TABLE_IN_INDICES_MINNUM: c_int = 1;
pub const MC_CMD_RSS_CONTEXT_READ_TABLE_IN_INDICES_MAXNUM: c_int = 124;
pub const MC_CMD_RSS_CONTEXT_READ_TABLE_IN_INDICES_MAXNUM_MCDI2: c_int = 508;
// MC_CMD_RSS_CONTEXT_READ_TABLE_OUT msgresponse
pub const MC_CMD_RSS_CONTEXT_READ_TABLE_OUT_LENMIN: c_int = 2;
pub const MC_CMD_RSS_CONTEXT_READ_TABLE_OUT_LENMAX: c_int = 252;
pub const MC_CMD_RSS_CONTEXT_READ_TABLE_OUT_LENMAX_MCDI2: c_int = 1020;

// A buffer containing the requested entries read from the table.
pub const MC_CMD_RSS_CONTEXT_READ_TABLE_OUT_DATA_OFST: c_int = 0;
pub const MC_CMD_RSS_CONTEXT_READ_TABLE_OUT_DATA_LEN: c_int = 2;
pub const MC_CMD_RSS_CONTEXT_READ_TABLE_OUT_DATA_MINNUM: c_int = 1;
pub const MC_CMD_RSS_CONTEXT_READ_TABLE_OUT_DATA_MAXNUM: c_int = 126;
pub const MC_CMD_RSS_CONTEXT_READ_TABLE_OUT_DATA_MAXNUM_MCDI2: c_int = 510;
//
// MC_CMD_RSS_CONTEXT_SET_FLAGS
// Set various control flags for an RSS context.
//
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS: c_uint = 0xe1;

// MC_CMD_RSS_CONTEXT_SET_FLAGS_IN msgrequest
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_LEN: c_int = 8;
// The handle of the RSS context
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_RSS_CONTEXT_ID_OFST: c_int = 0;
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_RSS_CONTEXT_ID_LEN: c_int = 4;
// Hash control flags. The _EN bits are always supported, but new modes are
// available when ADDITIONAL_RSS_MODES is reported by MC_CMD_GET_CAPABILITIES:
// in this case, the MODE fields may be set to non-zero values, and will take
// effect regardless of the settings of the _EN flags. See the RSS_MODE
// structure for the meaning of the mode bits. Drivers must check the
// capability before trying to set any _MODE fields, as older firmware will
// reject any attempt to set the FLAGS field to a value > 0xff with EINVAL. In
// the case where all the _MODE flags are zero, the _EN flags take effect,
// providing backward compatibility for existing drivers. (Setting all _MODE
// *and* all _EN flags to zero is valid, to disable RSS spreading for that
// particular packet type.)
//
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_FLAGS_OFST: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_FLAGS_LEN: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_TOEPLITZ_IPV4_EN_OFST: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_TOEPLITZ_IPV4_EN_LBN: c_int = 0;
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_TOEPLITZ_IPV4_EN_WIDTH: c_int = 1;
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_TOEPLITZ_TCPV4_EN_OFST: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_TOEPLITZ_TCPV4_EN_LBN: c_int = 1;
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_TOEPLITZ_TCPV4_EN_WIDTH: c_int = 1;
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_TOEPLITZ_IPV6_EN_OFST: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_TOEPLITZ_IPV6_EN_LBN: c_int = 2;
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_TOEPLITZ_IPV6_EN_WIDTH: c_int = 1;
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_TOEPLITZ_TCPV6_EN_OFST: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_TOEPLITZ_TCPV6_EN_LBN: c_int = 3;
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_TOEPLITZ_TCPV6_EN_WIDTH: c_int = 1;
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_RESERVED_OFST: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_RESERVED_LBN: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_RESERVED_WIDTH: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_TCP_IPV4_RSS_MODE_OFST: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_TCP_IPV4_RSS_MODE_LBN: c_int = 8;
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_TCP_IPV4_RSS_MODE_WIDTH: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_UDP_IPV4_RSS_MODE_OFST: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_UDP_IPV4_RSS_MODE_LBN: c_int = 12;
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_UDP_IPV4_RSS_MODE_WIDTH: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_OTHER_IPV4_RSS_MODE_OFST: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_OTHER_IPV4_RSS_MODE_LBN: c_int = 16;
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_OTHER_IPV4_RSS_MODE_WIDTH: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_TCP_IPV6_RSS_MODE_OFST: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_TCP_IPV6_RSS_MODE_LBN: c_int = 20;
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_TCP_IPV6_RSS_MODE_WIDTH: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_UDP_IPV6_RSS_MODE_OFST: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_UDP_IPV6_RSS_MODE_LBN: c_int = 24;
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_UDP_IPV6_RSS_MODE_WIDTH: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_OTHER_IPV6_RSS_MODE_OFST: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_OTHER_IPV6_RSS_MODE_LBN: c_int = 28;
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_IN_OTHER_IPV6_RSS_MODE_WIDTH: c_int = 4;
// MC_CMD_RSS_CONTEXT_SET_FLAGS_OUT msgresponse
pub const MC_CMD_RSS_CONTEXT_SET_FLAGS_OUT_LEN: c_int = 0;
//
// MC_CMD_RSS_CONTEXT_GET_FLAGS
// Get various control flags for an RSS context.
//
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS: c_uint = 0xe2;

// MC_CMD_RSS_CONTEXT_GET_FLAGS_IN msgrequest
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_IN_LEN: c_int = 4;
// The handle of the RSS context
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_IN_RSS_CONTEXT_ID_OFST: c_int = 0;
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_IN_RSS_CONTEXT_ID_LEN: c_int = 4;
// MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT msgresponse
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_LEN: c_int = 8;
// Hash control flags. If all _MODE bits are zero (which will always be true
// for older firmware which does not report the ADDITIONAL_RSS_MODES
// capability), the _EN bits report the state. If any _MODE bits are non-zero
// (which will only be true when the firmware reports ADDITIONAL_RSS_MODES)
// then the _EN bits should be disregarded, although the _MODE flags are
// guaranteed to be consistent with the _EN flags for a freshly-allocated RSS
// context and in the case where the _EN flags were used in the SET. This
// provides backward compatibility: old drivers will not be attempting to
// derive any meaning from the _MODE bits (and can never set them to any value
// not representable by the _EN bits); new drivers can always determine the
// mode by looking only at the _MODE bits; the value returned by a GET can
// always be used for a SET regardless of old/new driver vs. old/new firmware.
//
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_FLAGS_OFST: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_FLAGS_LEN: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_TOEPLITZ_IPV4_EN_OFST: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_TOEPLITZ_IPV4_EN_LBN: c_int = 0;
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_TOEPLITZ_IPV4_EN_WIDTH: c_int = 1;
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_TOEPLITZ_TCPV4_EN_OFST: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_TOEPLITZ_TCPV4_EN_LBN: c_int = 1;
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_TOEPLITZ_TCPV4_EN_WIDTH: c_int = 1;
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_TOEPLITZ_IPV6_EN_OFST: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_TOEPLITZ_IPV6_EN_LBN: c_int = 2;
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_TOEPLITZ_IPV6_EN_WIDTH: c_int = 1;
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_TOEPLITZ_TCPV6_EN_OFST: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_TOEPLITZ_TCPV6_EN_LBN: c_int = 3;
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_TOEPLITZ_TCPV6_EN_WIDTH: c_int = 1;
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_RESERVED_OFST: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_RESERVED_LBN: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_RESERVED_WIDTH: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_TCP_IPV4_RSS_MODE_OFST: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_TCP_IPV4_RSS_MODE_LBN: c_int = 8;
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_TCP_IPV4_RSS_MODE_WIDTH: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_UDP_IPV4_RSS_MODE_OFST: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_UDP_IPV4_RSS_MODE_LBN: c_int = 12;
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_UDP_IPV4_RSS_MODE_WIDTH: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_OTHER_IPV4_RSS_MODE_OFST: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_OTHER_IPV4_RSS_MODE_LBN: c_int = 16;
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_OTHER_IPV4_RSS_MODE_WIDTH: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_TCP_IPV6_RSS_MODE_OFST: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_TCP_IPV6_RSS_MODE_LBN: c_int = 20;
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_TCP_IPV6_RSS_MODE_WIDTH: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_UDP_IPV6_RSS_MODE_OFST: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_UDP_IPV6_RSS_MODE_LBN: c_int = 24;
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_UDP_IPV6_RSS_MODE_WIDTH: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_OTHER_IPV6_RSS_MODE_OFST: c_int = 4;
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_OTHER_IPV6_RSS_MODE_LBN: c_int = 28;
pub const MC_CMD_RSS_CONTEXT_GET_FLAGS_OUT_OTHER_IPV6_RSS_MODE_WIDTH: c_int = 4;
//
// MC_CMD_VPORT_ADD_MAC_ADDRESS
// Add a MAC address to a v-port
//
pub const MC_CMD_VPORT_ADD_MAC_ADDRESS: c_uint = 0xa8;

// MC_CMD_VPORT_ADD_MAC_ADDRESS_IN msgrequest
pub const MC_CMD_VPORT_ADD_MAC_ADDRESS_IN_LEN: c_int = 10;
// The handle of the v-port
pub const MC_CMD_VPORT_ADD_MAC_ADDRESS_IN_VPORT_ID_OFST: c_int = 0;
pub const MC_CMD_VPORT_ADD_MAC_ADDRESS_IN_VPORT_ID_LEN: c_int = 4;
// MAC address to add
pub const MC_CMD_VPORT_ADD_MAC_ADDRESS_IN_MACADDR_OFST: c_int = 4;
pub const MC_CMD_VPORT_ADD_MAC_ADDRESS_IN_MACADDR_LEN: c_int = 6;
// MC_CMD_VPORT_ADD_MAC_ADDRESS_OUT msgresponse
pub const MC_CMD_VPORT_ADD_MAC_ADDRESS_OUT_LEN: c_int = 0;
//
// MC_CMD_VPORT_DEL_MAC_ADDRESS
// Delete a MAC address from a v-port
//
pub const MC_CMD_VPORT_DEL_MAC_ADDRESS: c_uint = 0xa9;

// MC_CMD_VPORT_DEL_MAC_ADDRESS_IN msgrequest
pub const MC_CMD_VPORT_DEL_MAC_ADDRESS_IN_LEN: c_int = 10;
// The handle of the v-port
pub const MC_CMD_VPORT_DEL_MAC_ADDRESS_IN_VPORT_ID_OFST: c_int = 0;
pub const MC_CMD_VPORT_DEL_MAC_ADDRESS_IN_VPORT_ID_LEN: c_int = 4;
// MAC address to add
pub const MC_CMD_VPORT_DEL_MAC_ADDRESS_IN_MACADDR_OFST: c_int = 4;
pub const MC_CMD_VPORT_DEL_MAC_ADDRESS_IN_MACADDR_LEN: c_int = 6;
// MC_CMD_VPORT_DEL_MAC_ADDRESS_OUT msgresponse
pub const MC_CMD_VPORT_DEL_MAC_ADDRESS_OUT_LEN: c_int = 0;
//
// MC_CMD_VPORT_GET_MAC_ADDRESSES
// Delete a MAC address from a v-port
//
pub const MC_CMD_VPORT_GET_MAC_ADDRESSES: c_uint = 0xaa;

// MC_CMD_VPORT_GET_MAC_ADDRESSES_IN msgrequest
pub const MC_CMD_VPORT_GET_MAC_ADDRESSES_IN_LEN: c_int = 4;
// The handle of the v-port
pub const MC_CMD_VPORT_GET_MAC_ADDRESSES_IN_VPORT_ID_OFST: c_int = 0;
pub const MC_CMD_VPORT_GET_MAC_ADDRESSES_IN_VPORT_ID_LEN: c_int = 4;
// MC_CMD_VPORT_GET_MAC_ADDRESSES_OUT msgresponse
pub const MC_CMD_VPORT_GET_MAC_ADDRESSES_OUT_LENMIN: c_int = 4;
pub const MC_CMD_VPORT_GET_MAC_ADDRESSES_OUT_LENMAX: c_int = 250;
pub const MC_CMD_VPORT_GET_MAC_ADDRESSES_OUT_LENMAX_MCDI2: c_int = 1018;

// The number of MAC addresses returned
pub const MC_CMD_VPORT_GET_MAC_ADDRESSES_OUT_MACADDR_COUNT_OFST: c_int = 0;
pub const MC_CMD_VPORT_GET_MAC_ADDRESSES_OUT_MACADDR_COUNT_LEN: c_int = 4;
// Array of MAC addresses
pub const MC_CMD_VPORT_GET_MAC_ADDRESSES_OUT_MACADDR_OFST: c_int = 4;
pub const MC_CMD_VPORT_GET_MAC_ADDRESSES_OUT_MACADDR_LEN: c_int = 6;
pub const MC_CMD_VPORT_GET_MAC_ADDRESSES_OUT_MACADDR_MINNUM: c_int = 0;
pub const MC_CMD_VPORT_GET_MAC_ADDRESSES_OUT_MACADDR_MAXNUM: c_int = 41;
pub const MC_CMD_VPORT_GET_MAC_ADDRESSES_OUT_MACADDR_MAXNUM_MCDI2: c_int = 169;
//
// MC_CMD_VPORT_RECONFIGURE
// Replace VLAN tags and/or MAC addresses of an existing v-port. If the v-port
// has already been passed to another function (v-port's user), then that
// function will be reset before applying the changes.
//
pub const MC_CMD_VPORT_RECONFIGURE: c_uint = 0xeb;

// MC_CMD_VPORT_RECONFIGURE_IN msgrequest
pub const MC_CMD_VPORT_RECONFIGURE_IN_LEN: c_int = 44;
// The handle of the v-port
pub const MC_CMD_VPORT_RECONFIGURE_IN_VPORT_ID_OFST: c_int = 0;
pub const MC_CMD_VPORT_RECONFIGURE_IN_VPORT_ID_LEN: c_int = 4;
// Flags requesting what should be changed.
pub const MC_CMD_VPORT_RECONFIGURE_IN_FLAGS_OFST: c_int = 4;
pub const MC_CMD_VPORT_RECONFIGURE_IN_FLAGS_LEN: c_int = 4;
pub const MC_CMD_VPORT_RECONFIGURE_IN_REPLACE_VLAN_TAGS_OFST: c_int = 4;
pub const MC_CMD_VPORT_RECONFIGURE_IN_REPLACE_VLAN_TAGS_LBN: c_int = 0;
pub const MC_CMD_VPORT_RECONFIGURE_IN_REPLACE_VLAN_TAGS_WIDTH: c_int = 1;
pub const MC_CMD_VPORT_RECONFIGURE_IN_REPLACE_MACADDRS_OFST: c_int = 4;
pub const MC_CMD_VPORT_RECONFIGURE_IN_REPLACE_MACADDRS_LBN: c_int = 1;
pub const MC_CMD_VPORT_RECONFIGURE_IN_REPLACE_MACADDRS_WIDTH: c_int = 1;
// The number of VLAN tags to insert/remove. An error will be returned if
// incompatible with the number of VLAN tags specified for the upstream
// v-switch.
//
pub const MC_CMD_VPORT_RECONFIGURE_IN_NUM_VLAN_TAGS_OFST: c_int = 8;
pub const MC_CMD_VPORT_RECONFIGURE_IN_NUM_VLAN_TAGS_LEN: c_int = 4;
// The actual VLAN tags to insert/remove
pub const MC_CMD_VPORT_RECONFIGURE_IN_VLAN_TAGS_OFST: c_int = 12;
pub const MC_CMD_VPORT_RECONFIGURE_IN_VLAN_TAGS_LEN: c_int = 4;
pub const MC_CMD_VPORT_RECONFIGURE_IN_VLAN_TAG_0_OFST: c_int = 12;
pub const MC_CMD_VPORT_RECONFIGURE_IN_VLAN_TAG_0_LBN: c_int = 0;
pub const MC_CMD_VPORT_RECONFIGURE_IN_VLAN_TAG_0_WIDTH: c_int = 16;
pub const MC_CMD_VPORT_RECONFIGURE_IN_VLAN_TAG_1_OFST: c_int = 12;
pub const MC_CMD_VPORT_RECONFIGURE_IN_VLAN_TAG_1_LBN: c_int = 16;
pub const MC_CMD_VPORT_RECONFIGURE_IN_VLAN_TAG_1_WIDTH: c_int = 16;
// The number of MAC addresses to add
pub const MC_CMD_VPORT_RECONFIGURE_IN_NUM_MACADDRS_OFST: c_int = 16;
pub const MC_CMD_VPORT_RECONFIGURE_IN_NUM_MACADDRS_LEN: c_int = 4;
// MAC addresses to add
pub const MC_CMD_VPORT_RECONFIGURE_IN_MACADDRS_OFST: c_int = 20;
pub const MC_CMD_VPORT_RECONFIGURE_IN_MACADDRS_LEN: c_int = 6;
pub const MC_CMD_VPORT_RECONFIGURE_IN_MACADDRS_NUM: c_int = 4;
// MC_CMD_VPORT_RECONFIGURE_OUT msgresponse
pub const MC_CMD_VPORT_RECONFIGURE_OUT_LEN: c_int = 4;
pub const MC_CMD_VPORT_RECONFIGURE_OUT_FLAGS_OFST: c_int = 0;
pub const MC_CMD_VPORT_RECONFIGURE_OUT_FLAGS_LEN: c_int = 4;
pub const MC_CMD_VPORT_RECONFIGURE_OUT_RESET_DONE_OFST: c_int = 0;
pub const MC_CMD_VPORT_RECONFIGURE_OUT_RESET_DONE_LBN: c_int = 0;
pub const MC_CMD_VPORT_RECONFIGURE_OUT_RESET_DONE_WIDTH: c_int = 1;
//
// MC_CMD_EVB_PORT_QUERY
// read some config of v-port.
//
pub const MC_CMD_EVB_PORT_QUERY: c_uint = 0x62;

// MC_CMD_EVB_PORT_QUERY_IN msgrequest
pub const MC_CMD_EVB_PORT_QUERY_IN_LEN: c_int = 4;
// The handle of the v-port
pub const MC_CMD_EVB_PORT_QUERY_IN_PORT_ID_OFST: c_int = 0;
pub const MC_CMD_EVB_PORT_QUERY_IN_PORT_ID_LEN: c_int = 4;
// MC_CMD_EVB_PORT_QUERY_OUT msgresponse
pub const MC_CMD_EVB_PORT_QUERY_OUT_LEN: c_int = 8;
// The EVB port flags as defined at MC_CMD_VPORT_ALLOC.
pub const MC_CMD_EVB_PORT_QUERY_OUT_PORT_FLAGS_OFST: c_int = 0;
pub const MC_CMD_EVB_PORT_QUERY_OUT_PORT_FLAGS_LEN: c_int = 4;
// The number of VLAN tags that may be used on a v-adaptor connected to this
// EVB port.
//
pub const MC_CMD_EVB_PORT_QUERY_OUT_NUM_AVAILABLE_VLAN_TAGS_OFST: c_int = 4;
pub const MC_CMD_EVB_PORT_QUERY_OUT_NUM_AVAILABLE_VLAN_TAGS_LEN: c_int = 4;
//
// MC_CMD_GET_CLOCK
// Return the system and PDCPU clock frequencies.
//
pub const MC_CMD_GET_CLOCK: c_uint = 0xac;

// MC_CMD_GET_CLOCK_IN msgrequest
pub const MC_CMD_GET_CLOCK_IN_LEN: c_int = 0;
// MC_CMD_GET_CLOCK_OUT msgresponse
pub const MC_CMD_GET_CLOCK_OUT_LEN: c_int = 8;
// System frequency, MHz
pub const MC_CMD_GET_CLOCK_OUT_SYS_FREQ_OFST: c_int = 0;
pub const MC_CMD_GET_CLOCK_OUT_SYS_FREQ_LEN: c_int = 4;
// DPCPU frequency, MHz
pub const MC_CMD_GET_CLOCK_OUT_DPCPU_FREQ_OFST: c_int = 4;
pub const MC_CMD_GET_CLOCK_OUT_DPCPU_FREQ_LEN: c_int = 4;
//
// MC_CMD_TRIGGER_INTERRUPT
// Trigger an interrupt by prodding the BIU.
//
pub const MC_CMD_TRIGGER_INTERRUPT: c_uint = 0xe3;

// MC_CMD_TRIGGER_INTERRUPT_IN msgrequest
pub const MC_CMD_TRIGGER_INTERRUPT_IN_LEN: c_int = 4;
// Interrupt level relative to base for function.
pub const MC_CMD_TRIGGER_INTERRUPT_IN_INTR_LEVEL_OFST: c_int = 0;
pub const MC_CMD_TRIGGER_INTERRUPT_IN_INTR_LEVEL_LEN: c_int = 4;
// MC_CMD_TRIGGER_INTERRUPT_OUT msgresponse
pub const MC_CMD_TRIGGER_INTERRUPT_OUT_LEN: c_int = 0;
//
// MC_CMD_SHMBOOT_OP
// Special operations to support (for now) shmboot.
//
pub const MC_CMD_SHMBOOT_OP: c_uint = 0xe6;

// MC_CMD_SHMBOOT_OP_IN msgrequest
pub const MC_CMD_SHMBOOT_OP_IN_LEN: c_int = 4;
// Identifies the operation to perform
pub const MC_CMD_SHMBOOT_OP_IN_SHMBOOT_OP_OFST: c_int = 0;
pub const MC_CMD_SHMBOOT_OP_IN_SHMBOOT_OP_LEN: c_int = 4;
// enum: Copy slave_data section to the slave core. (Greenport only)
pub const MC_CMD_SHMBOOT_OP_IN_PUSH_SLAVE_DATA: c_uint = 0x0;
// MC_CMD_SHMBOOT_OP_OUT msgresponse
pub const MC_CMD_SHMBOOT_OP_OUT_LEN: c_int = 0;
//
// MC_CMD_SET_PSU
// Adjusts power supply parameters. This is a warranty-voiding operation.
// Returns: ENOENT if the parameter or rail specified does not exist, EINVAL if
// the parameter is out of range.
//
pub const MC_CMD_SET_PSU: c_uint = 0xea;

// MC_CMD_SET_PSU_IN msgrequest
pub const MC_CMD_SET_PSU_IN_LEN: c_int = 12;
pub const MC_CMD_SET_PSU_IN_PARAM_OFST: c_int = 0;
pub const MC_CMD_SET_PSU_IN_PARAM_LEN: c_int = 4;
pub const MC_CMD_SET_PSU_IN_PARAM_SUPPLY_VOLTAGE: c_uint = 0x0 /* enum */;
pub const MC_CMD_SET_PSU_IN_RAIL_OFST: c_int = 4;
pub const MC_CMD_SET_PSU_IN_RAIL_LEN: c_int = 4;
pub const MC_CMD_SET_PSU_IN_RAIL_0V9: c_uint = 0x0 /* enum */;
pub const MC_CMD_SET_PSU_IN_RAIL_1V2: c_uint = 0x1 /* enum */;
// desired value, eg voltage in mV
pub const MC_CMD_SET_PSU_IN_VALUE_OFST: c_int = 8;
pub const MC_CMD_SET_PSU_IN_VALUE_LEN: c_int = 4;
// MC_CMD_SET_PSU_OUT msgresponse
pub const MC_CMD_SET_PSU_OUT_LEN: c_int = 0;
//
// MC_CMD_GET_FUNCTION_INFO
// Get function information. PF and VF number.
//
pub const MC_CMD_GET_FUNCTION_INFO: c_uint = 0xec;

// MC_CMD_GET_FUNCTION_INFO_IN msgrequest
pub const MC_CMD_GET_FUNCTION_INFO_IN_LEN: c_int = 0;
// MC_CMD_GET_FUNCTION_INFO_OUT msgresponse
pub const MC_CMD_GET_FUNCTION_INFO_OUT_LEN: c_int = 8;
pub const MC_CMD_GET_FUNCTION_INFO_OUT_PF_OFST: c_int = 0;
pub const MC_CMD_GET_FUNCTION_INFO_OUT_PF_LEN: c_int = 4;
pub const MC_CMD_GET_FUNCTION_INFO_OUT_VF_OFST: c_int = 4;
pub const MC_CMD_GET_FUNCTION_INFO_OUT_VF_LEN: c_int = 4;
//
// MC_CMD_ENABLE_OFFLINE_BIST
// Enters offline BIST mode. All queues are torn down, chip enters quiescent
// mode, calling function gets exclusive MCDI ownership. The only way out is
// reboot.
//
pub const MC_CMD_ENABLE_OFFLINE_BIST: c_uint = 0xed;

// MC_CMD_ENABLE_OFFLINE_BIST_IN msgrequest
pub const MC_CMD_ENABLE_OFFLINE_BIST_IN_LEN: c_int = 0;
// MC_CMD_ENABLE_OFFLINE_BIST_OUT msgresponse
pub const MC_CMD_ENABLE_OFFLINE_BIST_OUT_LEN: c_int = 0;
//
// MC_CMD_READ_FUSES
// Read data programmed into the device One-Time-Programmable (OTP) Fuses
//
pub const MC_CMD_READ_FUSES: c_uint = 0xf0;

// MC_CMD_READ_FUSES_IN msgrequest
pub const MC_CMD_READ_FUSES_IN_LEN: c_int = 8;
// Offset in OTP to read
pub const MC_CMD_READ_FUSES_IN_OFFSET_OFST: c_int = 0;
pub const MC_CMD_READ_FUSES_IN_OFFSET_LEN: c_int = 4;
// Length of data to read in bytes
pub const MC_CMD_READ_FUSES_IN_LENGTH_OFST: c_int = 4;
pub const MC_CMD_READ_FUSES_IN_LENGTH_LEN: c_int = 4;
// MC_CMD_READ_FUSES_OUT msgresponse
pub const MC_CMD_READ_FUSES_OUT_LENMIN: c_int = 4;
pub const MC_CMD_READ_FUSES_OUT_LENMAX: c_int = 252;
pub const MC_CMD_READ_FUSES_OUT_LENMAX_MCDI2: c_int = 1020;

// Length of returned OTP data in bytes
pub const MC_CMD_READ_FUSES_OUT_LENGTH_OFST: c_int = 0;
pub const MC_CMD_READ_FUSES_OUT_LENGTH_LEN: c_int = 4;
// Returned data
pub const MC_CMD_READ_FUSES_OUT_DATA_OFST: c_int = 4;
pub const MC_CMD_READ_FUSES_OUT_DATA_LEN: c_int = 1;
pub const MC_CMD_READ_FUSES_OUT_DATA_MINNUM: c_int = 0;
pub const MC_CMD_READ_FUSES_OUT_DATA_MAXNUM: c_int = 248;
pub const MC_CMD_READ_FUSES_OUT_DATA_MAXNUM_MCDI2: c_int = 1016;
//
// MC_CMD_LICENSING
// Operations on the NVRAM_PARTITION_TYPE_LICENSE application license partition
// - not used for V3 licensing
//
pub const MC_CMD_LICENSING: c_uint = 0xf3;

// MC_CMD_LICENSING_IN msgrequest
pub const MC_CMD_LICENSING_IN_LEN: c_int = 4;
// identifies the type of operation requested
pub const MC_CMD_LICENSING_IN_OP_OFST: c_int = 0;
pub const MC_CMD_LICENSING_IN_OP_LEN: c_int = 4;
// enum: re-read and apply licenses after a license key partition update; note
// that this operation returns a zero-length response
//
pub const MC_CMD_LICENSING_IN_OP_UPDATE_LICENSE: c_uint = 0x0;
// enum: report counts of installed licenses
pub const MC_CMD_LICENSING_IN_OP_GET_KEY_STATS: c_uint = 0x1;
// MC_CMD_LICENSING_OUT msgresponse
pub const MC_CMD_LICENSING_OUT_LEN: c_int = 28;
// count of application keys which are valid
pub const MC_CMD_LICENSING_OUT_VALID_APP_KEYS_OFST: c_int = 0;
pub const MC_CMD_LICENSING_OUT_VALID_APP_KEYS_LEN: c_int = 4;
// sum of UNVERIFIABLE_APP_KEYS + WRONG_NODE_APP_KEYS (for compatibility with
// MC_CMD_FC_OP_LICENSE)
//
pub const MC_CMD_LICENSING_OUT_INVALID_APP_KEYS_OFST: c_int = 4;
pub const MC_CMD_LICENSING_OUT_INVALID_APP_KEYS_LEN: c_int = 4;
// count of application keys which are invalid due to being blacklisted
pub const MC_CMD_LICENSING_OUT_BLACKLISTED_APP_KEYS_OFST: c_int = 8;
pub const MC_CMD_LICENSING_OUT_BLACKLISTED_APP_KEYS_LEN: c_int = 4;
// count of application keys which are invalid due to being unverifiable
pub const MC_CMD_LICENSING_OUT_UNVERIFIABLE_APP_KEYS_OFST: c_int = 12;
pub const MC_CMD_LICENSING_OUT_UNVERIFIABLE_APP_KEYS_LEN: c_int = 4;
// count of application keys which are invalid due to being for the wrong node
//
pub const MC_CMD_LICENSING_OUT_WRONG_NODE_APP_KEYS_OFST: c_int = 16;
pub const MC_CMD_LICENSING_OUT_WRONG_NODE_APP_KEYS_LEN: c_int = 4;
// licensing state (for diagnostics; the exact meaning of the bits in this
// field are private to the firmware)
//
pub const MC_CMD_LICENSING_OUT_LICENSING_STATE_OFST: c_int = 20;
pub const MC_CMD_LICENSING_OUT_LICENSING_STATE_LEN: c_int = 4;
// licensing subsystem self-test report (for manftest)
pub const MC_CMD_LICENSING_OUT_LICENSING_SELF_TEST_OFST: c_int = 24;
pub const MC_CMD_LICENSING_OUT_LICENSING_SELF_TEST_LEN: c_int = 4;
// enum: licensing subsystem self-test failed
pub const MC_CMD_LICENSING_OUT_SELF_TEST_FAIL: c_uint = 0x0;
// enum: licensing subsystem self-test passed
pub const MC_CMD_LICENSING_OUT_SELF_TEST_PASS: c_uint = 0x1;
//
// MC_CMD_LICENSING_V3
// Operations on the NVRAM_PARTITION_TYPE_LICENSE application license partition
// - V3 licensing (Medford)
//
pub const MC_CMD_LICENSING_V3: c_uint = 0xd0;

// MC_CMD_LICENSING_V3_IN msgrequest
pub const MC_CMD_LICENSING_V3_IN_LEN: c_int = 4;
// identifies the type of operation requested
pub const MC_CMD_LICENSING_V3_IN_OP_OFST: c_int = 0;
pub const MC_CMD_LICENSING_V3_IN_OP_LEN: c_int = 4;
// enum: re-read and apply licenses after a license key partition update; note
// that this operation returns a zero-length response
//
pub const MC_CMD_LICENSING_V3_IN_OP_UPDATE_LICENSE: c_uint = 0x0;
// enum: report counts of installed licenses Returns EAGAIN if license
// processing (updating) has been started but not yet completed.
//
pub const MC_CMD_LICENSING_V3_IN_OP_REPORT_LICENSE: c_uint = 0x1;
// MC_CMD_LICENSING_V3_OUT msgresponse
pub const MC_CMD_LICENSING_V3_OUT_LEN: c_int = 88;
// count of keys which are valid
pub const MC_CMD_LICENSING_V3_OUT_VALID_KEYS_OFST: c_int = 0;
pub const MC_CMD_LICENSING_V3_OUT_VALID_KEYS_LEN: c_int = 4;
// sum of UNVERIFIABLE_KEYS + WRONG_NODE_KEYS (for compatibility with
// MC_CMD_FC_OP_LICENSE)
//
pub const MC_CMD_LICENSING_V3_OUT_INVALID_KEYS_OFST: c_int = 4;
pub const MC_CMD_LICENSING_V3_OUT_INVALID_KEYS_LEN: c_int = 4;
// count of keys which are invalid due to being unverifiable
pub const MC_CMD_LICENSING_V3_OUT_UNVERIFIABLE_KEYS_OFST: c_int = 8;
pub const MC_CMD_LICENSING_V3_OUT_UNVERIFIABLE_KEYS_LEN: c_int = 4;
// count of keys which are invalid due to being for the wrong node
pub const MC_CMD_LICENSING_V3_OUT_WRONG_NODE_KEYS_OFST: c_int = 12;
pub const MC_CMD_LICENSING_V3_OUT_WRONG_NODE_KEYS_LEN: c_int = 4;
// licensing state (for diagnostics; the exact meaning of the bits in this
// field are private to the firmware)
//
pub const MC_CMD_LICENSING_V3_OUT_LICENSING_STATE_OFST: c_int = 16;
pub const MC_CMD_LICENSING_V3_OUT_LICENSING_STATE_LEN: c_int = 4;
// licensing subsystem self-test report (for manftest)
pub const MC_CMD_LICENSING_V3_OUT_LICENSING_SELF_TEST_OFST: c_int = 20;
pub const MC_CMD_LICENSING_V3_OUT_LICENSING_SELF_TEST_LEN: c_int = 4;
// enum: licensing subsystem self-test failed
pub const MC_CMD_LICENSING_V3_OUT_SELF_TEST_FAIL: c_uint = 0x0;
// enum: licensing subsystem self-test passed
pub const MC_CMD_LICENSING_V3_OUT_SELF_TEST_PASS: c_uint = 0x1;
// bitmask of licensed applications
pub const MC_CMD_LICENSING_V3_OUT_LICENSED_APPS_OFST: c_int = 24;
pub const MC_CMD_LICENSING_V3_OUT_LICENSED_APPS_LEN: c_int = 8;
pub const MC_CMD_LICENSING_V3_OUT_LICENSED_APPS_LO_OFST: c_int = 24;
pub const MC_CMD_LICENSING_V3_OUT_LICENSED_APPS_HI_OFST: c_int = 28;
// reserved for future use
pub const MC_CMD_LICENSING_V3_OUT_RESERVED_0_OFST: c_int = 32;
pub const MC_CMD_LICENSING_V3_OUT_RESERVED_0_LEN: c_int = 24;
// bitmask of licensed features
pub const MC_CMD_LICENSING_V3_OUT_LICENSED_FEATURES_OFST: c_int = 56;
pub const MC_CMD_LICENSING_V3_OUT_LICENSED_FEATURES_LEN: c_int = 8;
pub const MC_CMD_LICENSING_V3_OUT_LICENSED_FEATURES_LO_OFST: c_int = 56;
pub const MC_CMD_LICENSING_V3_OUT_LICENSED_FEATURES_HI_OFST: c_int = 60;
// reserved for future use
pub const MC_CMD_LICENSING_V3_OUT_RESERVED_1_OFST: c_int = 64;
pub const MC_CMD_LICENSING_V3_OUT_RESERVED_1_LEN: c_int = 24;
//
// MC_CMD_LICENSING_GET_ID_V3
// Get ID and type from the NVRAM_PARTITION_TYPE_LICENSE application license
// partition - V3 licensing (Medford)
//
pub const MC_CMD_LICENSING_GET_ID_V3: c_uint = 0xd1;

// MC_CMD_LICENSING_GET_ID_V3_IN msgrequest
pub const MC_CMD_LICENSING_GET_ID_V3_IN_LEN: c_int = 0;
// MC_CMD_LICENSING_GET_ID_V3_OUT msgresponse
pub const MC_CMD_LICENSING_GET_ID_V3_OUT_LENMIN: c_int = 8;
pub const MC_CMD_LICENSING_GET_ID_V3_OUT_LENMAX: c_int = 252;
pub const MC_CMD_LICENSING_GET_ID_V3_OUT_LENMAX_MCDI2: c_int = 1020;

// type of license (eg 3)
pub const MC_CMD_LICENSING_GET_ID_V3_OUT_LICENSE_TYPE_OFST: c_int = 0;
pub const MC_CMD_LICENSING_GET_ID_V3_OUT_LICENSE_TYPE_LEN: c_int = 4;
// length of the license ID (in bytes)
pub const MC_CMD_LICENSING_GET_ID_V3_OUT_LICENSE_ID_LENGTH_OFST: c_int = 4;
pub const MC_CMD_LICENSING_GET_ID_V3_OUT_LICENSE_ID_LENGTH_LEN: c_int = 4;
// the unique license ID of the adapter
pub const MC_CMD_LICENSING_GET_ID_V3_OUT_LICENSE_ID_OFST: c_int = 8;
pub const MC_CMD_LICENSING_GET_ID_V3_OUT_LICENSE_ID_LEN: c_int = 1;
pub const MC_CMD_LICENSING_GET_ID_V3_OUT_LICENSE_ID_MINNUM: c_int = 0;
pub const MC_CMD_LICENSING_GET_ID_V3_OUT_LICENSE_ID_MAXNUM: c_int = 244;
pub const MC_CMD_LICENSING_GET_ID_V3_OUT_LICENSE_ID_MAXNUM_MCDI2: c_int = 1012;
//
// MC_CMD_GET_LICENSED_APP_STATE
// Query the state of an individual licensed application. (Note that the actual
// state may be invalidated by the MC_CMD_LICENSING OP_UPDATE_LICENSE operation
// or a reboot of the MC.) Not used for V3 licensing
//
pub const MC_CMD_GET_LICENSED_APP_STATE: c_uint = 0xf5;

// MC_CMD_GET_LICENSED_APP_STATE_IN msgrequest
pub const MC_CMD_GET_LICENSED_APP_STATE_IN_LEN: c_int = 4;
// application ID to query (LICENSED_APP_ID_xxx)
pub const MC_CMD_GET_LICENSED_APP_STATE_IN_APP_ID_OFST: c_int = 0;
pub const MC_CMD_GET_LICENSED_APP_STATE_IN_APP_ID_LEN: c_int = 4;
// MC_CMD_GET_LICENSED_APP_STATE_OUT msgresponse
pub const MC_CMD_GET_LICENSED_APP_STATE_OUT_LEN: c_int = 4;
// state of this application
pub const MC_CMD_GET_LICENSED_APP_STATE_OUT_STATE_OFST: c_int = 0;
pub const MC_CMD_GET_LICENSED_APP_STATE_OUT_STATE_LEN: c_int = 4;
// enum: no (or invalid) license is present for the application
pub const MC_CMD_GET_LICENSED_APP_STATE_OUT_NOT_LICENSED: c_uint = 0x0;
// enum: a valid license is present for the application
pub const MC_CMD_GET_LICENSED_APP_STATE_OUT_LICENSED: c_uint = 0x1;
//
// MC_CMD_GET_LICENSED_V3_APP_STATE
// Query the state of an individual licensed application. (Note that the actual
// state may be invalidated by the MC_CMD_LICENSING_V3 OP_UPDATE_LICENSE
// operation or a reboot of the MC.) Used for V3 licensing (Medford)
//
pub const MC_CMD_GET_LICENSED_V3_APP_STATE: c_uint = 0xd2;

// MC_CMD_GET_LICENSED_V3_APP_STATE_IN msgrequest
pub const MC_CMD_GET_LICENSED_V3_APP_STATE_IN_LEN: c_int = 8;
// application ID to query (LICENSED_V3_APPS_xxx) expressed as a single bit
// mask
//
pub const MC_CMD_GET_LICENSED_V3_APP_STATE_IN_APP_ID_OFST: c_int = 0;
pub const MC_CMD_GET_LICENSED_V3_APP_STATE_IN_APP_ID_LEN: c_int = 8;
pub const MC_CMD_GET_LICENSED_V3_APP_STATE_IN_APP_ID_LO_OFST: c_int = 0;
pub const MC_CMD_GET_LICENSED_V3_APP_STATE_IN_APP_ID_HI_OFST: c_int = 4;
// MC_CMD_GET_LICENSED_V3_APP_STATE_OUT msgresponse
pub const MC_CMD_GET_LICENSED_V3_APP_STATE_OUT_LEN: c_int = 4;
// state of this application
pub const MC_CMD_GET_LICENSED_V3_APP_STATE_OUT_STATE_OFST: c_int = 0;
pub const MC_CMD_GET_LICENSED_V3_APP_STATE_OUT_STATE_LEN: c_int = 4;
// enum: no (or invalid) license is present for the application
pub const MC_CMD_GET_LICENSED_V3_APP_STATE_OUT_NOT_LICENSED: c_uint = 0x0;
// enum: a valid license is present for the application
pub const MC_CMD_GET_LICENSED_V3_APP_STATE_OUT_LICENSED: c_uint = 0x1;
//
// MC_CMD_GET_LICENSED_V3_FEATURE_STATES
// Query the state of an one or more licensed features. (Note that the actual
// state may be invalidated by the MC_CMD_LICENSING_V3 OP_UPDATE_LICENSE
// operation or a reboot of the MC.) Used for V3 licensing (Medford)
//
pub const MC_CMD_GET_LICENSED_V3_FEATURE_STATES: c_uint = 0xd3;

// MC_CMD_GET_LICENSED_V3_FEATURE_STATES_IN msgrequest
pub const MC_CMD_GET_LICENSED_V3_FEATURE_STATES_IN_LEN: c_int = 8;
// features to query (LICENSED_V3_FEATURES_xxx) expressed as a mask with one or
// more bits set
//
pub const MC_CMD_GET_LICENSED_V3_FEATURE_STATES_IN_FEATURES_OFST: c_int = 0;
pub const MC_CMD_GET_LICENSED_V3_FEATURE_STATES_IN_FEATURES_LEN: c_int = 8;
pub const MC_CMD_GET_LICENSED_V3_FEATURE_STATES_IN_FEATURES_LO_OFST: c_int = 0;
pub const MC_CMD_GET_LICENSED_V3_FEATURE_STATES_IN_FEATURES_HI_OFST: c_int = 4;
// MC_CMD_GET_LICENSED_V3_FEATURE_STATES_OUT msgresponse
pub const MC_CMD_GET_LICENSED_V3_FEATURE_STATES_OUT_LEN: c_int = 8;
// states of these features - bit set for licensed, clear for not licensed
pub const MC_CMD_GET_LICENSED_V3_FEATURE_STATES_OUT_STATES_OFST: c_int = 0;
pub const MC_CMD_GET_LICENSED_V3_FEATURE_STATES_OUT_STATES_LEN: c_int = 8;
pub const MC_CMD_GET_LICENSED_V3_FEATURE_STATES_OUT_STATES_LO_OFST: c_int = 0;
pub const MC_CMD_GET_LICENSED_V3_FEATURE_STATES_OUT_STATES_HI_OFST: c_int = 4;
//
// MC_CMD_LICENSED_APP_OP
// Perform an action for an individual licensed application - not used for V3
// licensing.
//
pub const MC_CMD_LICENSED_APP_OP: c_uint = 0xf6;

// MC_CMD_LICENSED_APP_OP_IN msgrequest
pub const MC_CMD_LICENSED_APP_OP_IN_LENMIN: c_int = 8;
pub const MC_CMD_LICENSED_APP_OP_IN_LENMAX: c_int = 252;
pub const MC_CMD_LICENSED_APP_OP_IN_LENMAX_MCDI2: c_int = 1020;

// application ID
pub const MC_CMD_LICENSED_APP_OP_IN_APP_ID_OFST: c_int = 0;
pub const MC_CMD_LICENSED_APP_OP_IN_APP_ID_LEN: c_int = 4;
// the type of operation requested
pub const MC_CMD_LICENSED_APP_OP_IN_OP_OFST: c_int = 4;
pub const MC_CMD_LICENSED_APP_OP_IN_OP_LEN: c_int = 4;
// enum: validate application
pub const MC_CMD_LICENSED_APP_OP_IN_OP_VALIDATE: c_uint = 0x0;
// enum: mask application
pub const MC_CMD_LICENSED_APP_OP_IN_OP_MASK: c_uint = 0x1;
// arguments specific to this particular operation
pub const MC_CMD_LICENSED_APP_OP_IN_ARGS_OFST: c_int = 8;
pub const MC_CMD_LICENSED_APP_OP_IN_ARGS_LEN: c_int = 4;
pub const MC_CMD_LICENSED_APP_OP_IN_ARGS_MINNUM: c_int = 0;
pub const MC_CMD_LICENSED_APP_OP_IN_ARGS_MAXNUM: c_int = 61;
pub const MC_CMD_LICENSED_APP_OP_IN_ARGS_MAXNUM_MCDI2: c_int = 253;
// MC_CMD_LICENSED_APP_OP_OUT msgresponse
pub const MC_CMD_LICENSED_APP_OP_OUT_LENMIN: c_int = 0;
pub const MC_CMD_LICENSED_APP_OP_OUT_LENMAX: c_int = 252;
pub const MC_CMD_LICENSED_APP_OP_OUT_LENMAX_MCDI2: c_int = 1020;

// result specific to this particular operation
pub const MC_CMD_LICENSED_APP_OP_OUT_RESULT_OFST: c_int = 0;
pub const MC_CMD_LICENSED_APP_OP_OUT_RESULT_LEN: c_int = 4;
pub const MC_CMD_LICENSED_APP_OP_OUT_RESULT_MINNUM: c_int = 0;
pub const MC_CMD_LICENSED_APP_OP_OUT_RESULT_MAXNUM: c_int = 63;
pub const MC_CMD_LICENSED_APP_OP_OUT_RESULT_MAXNUM_MCDI2: c_int = 255;
// MC_CMD_LICENSED_APP_OP_VALIDATE_IN msgrequest
pub const MC_CMD_LICENSED_APP_OP_VALIDATE_IN_LEN: c_int = 72;
// application ID
pub const MC_CMD_LICENSED_APP_OP_VALIDATE_IN_APP_ID_OFST: c_int = 0;
pub const MC_CMD_LICENSED_APP_OP_VALIDATE_IN_APP_ID_LEN: c_int = 4;
// the type of operation requested
pub const MC_CMD_LICENSED_APP_OP_VALIDATE_IN_OP_OFST: c_int = 4;
pub const MC_CMD_LICENSED_APP_OP_VALIDATE_IN_OP_LEN: c_int = 4;
// validation challenge
pub const MC_CMD_LICENSED_APP_OP_VALIDATE_IN_CHALLENGE_OFST: c_int = 8;
pub const MC_CMD_LICENSED_APP_OP_VALIDATE_IN_CHALLENGE_LEN: c_int = 64;
// MC_CMD_LICENSED_APP_OP_VALIDATE_OUT msgresponse
pub const MC_CMD_LICENSED_APP_OP_VALIDATE_OUT_LEN: c_int = 68;
// feature expiry (time_t)
pub const MC_CMD_LICENSED_APP_OP_VALIDATE_OUT_EXPIRY_OFST: c_int = 0;
pub const MC_CMD_LICENSED_APP_OP_VALIDATE_OUT_EXPIRY_LEN: c_int = 4;
// validation response
pub const MC_CMD_LICENSED_APP_OP_VALIDATE_OUT_RESPONSE_OFST: c_int = 4;
pub const MC_CMD_LICENSED_APP_OP_VALIDATE_OUT_RESPONSE_LEN: c_int = 64;
// MC_CMD_LICENSED_APP_OP_MASK_IN msgrequest
pub const MC_CMD_LICENSED_APP_OP_MASK_IN_LEN: c_int = 12;
// application ID
pub const MC_CMD_LICENSED_APP_OP_MASK_IN_APP_ID_OFST: c_int = 0;
pub const MC_CMD_LICENSED_APP_OP_MASK_IN_APP_ID_LEN: c_int = 4;
// the type of operation requested
pub const MC_CMD_LICENSED_APP_OP_MASK_IN_OP_OFST: c_int = 4;
pub const MC_CMD_LICENSED_APP_OP_MASK_IN_OP_LEN: c_int = 4;
// flag
pub const MC_CMD_LICENSED_APP_OP_MASK_IN_FLAG_OFST: c_int = 8;
pub const MC_CMD_LICENSED_APP_OP_MASK_IN_FLAG_LEN: c_int = 4;
// MC_CMD_LICENSED_APP_OP_MASK_OUT msgresponse
pub const MC_CMD_LICENSED_APP_OP_MASK_OUT_LEN: c_int = 0;
//
// MC_CMD_LICENSED_V3_VALIDATE_APP
// Perform validation for an individual licensed application - V3 licensing
// (Medford)
//
pub const MC_CMD_LICENSED_V3_VALIDATE_APP: c_uint = 0xd4;

// MC_CMD_LICENSED_V3_VALIDATE_APP_IN msgrequest
pub const MC_CMD_LICENSED_V3_VALIDATE_APP_IN_LEN: c_int = 56;
// challenge for validation (384 bits)
pub const MC_CMD_LICENSED_V3_VALIDATE_APP_IN_CHALLENGE_OFST: c_int = 0;
pub const MC_CMD_LICENSED_V3_VALIDATE_APP_IN_CHALLENGE_LEN: c_int = 48;
// application ID expressed as a single bit mask
pub const MC_CMD_LICENSED_V3_VALIDATE_APP_IN_APP_ID_OFST: c_int = 48;
pub const MC_CMD_LICENSED_V3_VALIDATE_APP_IN_APP_ID_LEN: c_int = 8;
pub const MC_CMD_LICENSED_V3_VALIDATE_APP_IN_APP_ID_LO_OFST: c_int = 48;
pub const MC_CMD_LICENSED_V3_VALIDATE_APP_IN_APP_ID_HI_OFST: c_int = 52;
// MC_CMD_LICENSED_V3_VALIDATE_APP_OUT msgresponse
pub const MC_CMD_LICENSED_V3_VALIDATE_APP_OUT_LEN: c_int = 116;
// validation response to challenge in the form of ECDSA signature consisting
// of two 384-bit integers, r and s, in big-endian order. The signature signs a
// SHA-384 digest of a message constructed from the concatenation of the input
// message and the remaining fields of this output message, e.g. challenge[48
// bytes] ... expiry_time[4 bytes] ...
//
pub const MC_CMD_LICENSED_V3_VALIDATE_APP_OUT_RESPONSE_OFST: c_int = 0;
pub const MC_CMD_LICENSED_V3_VALIDATE_APP_OUT_RESPONSE_LEN: c_int = 96;
// application expiry time
pub const MC_CMD_LICENSED_V3_VALIDATE_APP_OUT_EXPIRY_TIME_OFST: c_int = 96;
pub const MC_CMD_LICENSED_V3_VALIDATE_APP_OUT_EXPIRY_TIME_LEN: c_int = 4;
// application expiry units
pub const MC_CMD_LICENSED_V3_VALIDATE_APP_OUT_EXPIRY_UNITS_OFST: c_int = 100;
pub const MC_CMD_LICENSED_V3_VALIDATE_APP_OUT_EXPIRY_UNITS_LEN: c_int = 4;
// enum: expiry units are accounting units
pub const MC_CMD_LICENSED_V3_VALIDATE_APP_OUT_EXPIRY_UNIT_ACC: c_uint = 0x0;
// enum: expiry units are calendar days
pub const MC_CMD_LICENSED_V3_VALIDATE_APP_OUT_EXPIRY_UNIT_DAYS: c_uint = 0x1;
// base MAC address of the NIC stored in NVRAM (note that this is a constant
// value for a given NIC regardless which function is calling, effectively this
// is PF0 base MAC address)
//
pub const MC_CMD_LICENSED_V3_VALIDATE_APP_OUT_BASE_MACADDR_OFST: c_int = 104;
pub const MC_CMD_LICENSED_V3_VALIDATE_APP_OUT_BASE_MACADDR_LEN: c_int = 6;
// MAC address of v-adaptor associated with the client. If no such v-adapator
// exists, then the field is filled with 0xFF.
//
pub const MC_CMD_LICENSED_V3_VALIDATE_APP_OUT_VADAPTOR_MACADDR_OFST: c_int = 110;
pub const MC_CMD_LICENSED_V3_VALIDATE_APP_OUT_VADAPTOR_MACADDR_LEN: c_int = 6;
//
// MC_CMD_LICENSED_V3_MASK_FEATURES
// Mask features - V3 licensing (Medford)
//
pub const MC_CMD_LICENSED_V3_MASK_FEATURES: c_uint = 0xd5;

// MC_CMD_LICENSED_V3_MASK_FEATURES_IN msgrequest
pub const MC_CMD_LICENSED_V3_MASK_FEATURES_IN_LEN: c_int = 12;
// mask to be applied to features to be changed
pub const MC_CMD_LICENSED_V3_MASK_FEATURES_IN_MASK_OFST: c_int = 0;
pub const MC_CMD_LICENSED_V3_MASK_FEATURES_IN_MASK_LEN: c_int = 8;
pub const MC_CMD_LICENSED_V3_MASK_FEATURES_IN_MASK_LO_OFST: c_int = 0;
pub const MC_CMD_LICENSED_V3_MASK_FEATURES_IN_MASK_HI_OFST: c_int = 4;
// whether to turn on or turn off the masked features
pub const MC_CMD_LICENSED_V3_MASK_FEATURES_IN_FLAG_OFST: c_int = 8;
pub const MC_CMD_LICENSED_V3_MASK_FEATURES_IN_FLAG_LEN: c_int = 4;
// enum: turn the features off
pub const MC_CMD_LICENSED_V3_MASK_FEATURES_IN_OFF: c_uint = 0x0;
// enum: turn the features back on
pub const MC_CMD_LICENSED_V3_MASK_FEATURES_IN_ON: c_uint = 0x1;
// MC_CMD_LICENSED_V3_MASK_FEATURES_OUT msgresponse
pub const MC_CMD_LICENSED_V3_MASK_FEATURES_OUT_LEN: c_int = 0;
//
// MC_CMD_LICENSING_V3_TEMPORARY
// Perform operations to support installation of a single temporary license in
// the adapter, in addition to those found in the licensing partition. See
// SF-116124-SW for an overview of how this could be used. The license is
// stored in MC persistent data and so will survive a MC reboot, but will be
// erased when the adapter is power cycled
//
pub const MC_CMD_LICENSING_V3_TEMPORARY: c_uint = 0xd6;

// MC_CMD_LICENSING_V3_TEMPORARY_IN msgrequest
pub const MC_CMD_LICENSING_V3_TEMPORARY_IN_LEN: c_int = 4;
// operation code
pub const MC_CMD_LICENSING_V3_TEMPORARY_IN_OP_OFST: c_int = 0;
pub const MC_CMD_LICENSING_V3_TEMPORARY_IN_OP_LEN: c_int = 4;
// enum: install a new license, overwriting any existing temporary license.
// This is an asynchronous operation owing to the time taken to validate an
// ECDSA license
//
pub const MC_CMD_LICENSING_V3_TEMPORARY_SET: c_uint = 0x0;
// enum: clear the license immediately rather than waiting for the next power
// cycle
//
pub const MC_CMD_LICENSING_V3_TEMPORARY_CLEAR: c_uint = 0x1;
// enum: get the status of the asynchronous MC_CMD_LICENSING_V3_TEMPORARY_SET
// operation
//
pub const MC_CMD_LICENSING_V3_TEMPORARY_STATUS: c_uint = 0x2;
// MC_CMD_LICENSING_V3_TEMPORARY_IN_SET msgrequest
pub const MC_CMD_LICENSING_V3_TEMPORARY_IN_SET_LEN: c_int = 164;
pub const MC_CMD_LICENSING_V3_TEMPORARY_IN_SET_OP_OFST: c_int = 0;
pub const MC_CMD_LICENSING_V3_TEMPORARY_IN_SET_OP_LEN: c_int = 4;
// ECDSA license and signature
pub const MC_CMD_LICENSING_V3_TEMPORARY_IN_SET_LICENSE_OFST: c_int = 4;
pub const MC_CMD_LICENSING_V3_TEMPORARY_IN_SET_LICENSE_LEN: c_int = 160;
// MC_CMD_LICENSING_V3_TEMPORARY_IN_CLEAR msgrequest
pub const MC_CMD_LICENSING_V3_TEMPORARY_IN_CLEAR_LEN: c_int = 4;
pub const MC_CMD_LICENSING_V3_TEMPORARY_IN_CLEAR_OP_OFST: c_int = 0;
pub const MC_CMD_LICENSING_V3_TEMPORARY_IN_CLEAR_OP_LEN: c_int = 4;
// MC_CMD_LICENSING_V3_TEMPORARY_IN_STATUS msgrequest
pub const MC_CMD_LICENSING_V3_TEMPORARY_IN_STATUS_LEN: c_int = 4;
pub const MC_CMD_LICENSING_V3_TEMPORARY_IN_STATUS_OP_OFST: c_int = 0;
pub const MC_CMD_LICENSING_V3_TEMPORARY_IN_STATUS_OP_LEN: c_int = 4;
// MC_CMD_LICENSING_V3_TEMPORARY_OUT_STATUS msgresponse
pub const MC_CMD_LICENSING_V3_TEMPORARY_OUT_STATUS_LEN: c_int = 12;
// status code
pub const MC_CMD_LICENSING_V3_TEMPORARY_OUT_STATUS_STATUS_OFST: c_int = 0;
pub const MC_CMD_LICENSING_V3_TEMPORARY_OUT_STATUS_STATUS_LEN: c_int = 4;
// enum: finished validating and installing license
pub const MC_CMD_LICENSING_V3_TEMPORARY_STATUS_OK: c_uint = 0x0;
// enum: license validation and installation in progress
pub const MC_CMD_LICENSING_V3_TEMPORARY_STATUS_IN_PROGRESS: c_uint = 0x1;
// enum: licensing error. More specific error messages are not provided to
// avoid exposing details of the licensing system to the client
//
pub const MC_CMD_LICENSING_V3_TEMPORARY_STATUS_ERROR: c_uint = 0x2;
// bitmask of licensed features
pub const MC_CMD_LICENSING_V3_TEMPORARY_OUT_STATUS_LICENSED_FEATURES_OFST: c_int = 4;
pub const MC_CMD_LICENSING_V3_TEMPORARY_OUT_STATUS_LICENSED_FEATURES_LEN: c_int = 8;
pub const MC_CMD_LICENSING_V3_TEMPORARY_OUT_STATUS_LICENSED_FEATURES_LO_OFST: c_int = 4;
pub const MC_CMD_LICENSING_V3_TEMPORARY_OUT_STATUS_LICENSED_FEATURES_HI_OFST: c_int = 8;
//
// MC_CMD_SET_PARSER_DISP_CONFIG
// Change configuration related to the parser-dispatcher subsystem.
//
pub const MC_CMD_SET_PARSER_DISP_CONFIG: c_uint = 0xf9;

// MC_CMD_SET_PARSER_DISP_CONFIG_IN msgrequest
pub const MC_CMD_SET_PARSER_DISP_CONFIG_IN_LENMIN: c_int = 12;
pub const MC_CMD_SET_PARSER_DISP_CONFIG_IN_LENMAX: c_int = 252;
pub const MC_CMD_SET_PARSER_DISP_CONFIG_IN_LENMAX_MCDI2: c_int = 1020;

// the type of configuration setting to change
pub const MC_CMD_SET_PARSER_DISP_CONFIG_IN_TYPE_OFST: c_int = 0;
pub const MC_CMD_SET_PARSER_DISP_CONFIG_IN_TYPE_LEN: c_int = 4;
// enum: Per-TXQ enable for multicast UDP destination lookup for possible
// internal loopback. (ENTITY is a queue handle, VALUE is a single boolean.)
//
pub const MC_CMD_SET_PARSER_DISP_CONFIG_IN_TXQ_MCAST_UDP_DST_LOOKUP_EN: c_uint = 0x0;
// enum: Per-v-adaptor enable for suppression of self-transmissions on the
// internal loopback path. (ENTITY is an EVB_PORT_ID, VALUE is a single
// boolean.)
//
pub const MC_CMD_SET_PARSER_DISP_CONFIG_IN_VADAPTOR_SUPPRESS_SELF_TX: c_uint = 0x1;
// handle for the entity to update: queue handle, EVB port ID, etc. depending
// on the type of configuration setting being changed
//
pub const MC_CMD_SET_PARSER_DISP_CONFIG_IN_ENTITY_OFST: c_int = 4;
pub const MC_CMD_SET_PARSER_DISP_CONFIG_IN_ENTITY_LEN: c_int = 4;
// new value: the details depend on the type of configuration setting being
// changed
//
pub const MC_CMD_SET_PARSER_DISP_CONFIG_IN_VALUE_OFST: c_int = 8;
pub const MC_CMD_SET_PARSER_DISP_CONFIG_IN_VALUE_LEN: c_int = 4;
pub const MC_CMD_SET_PARSER_DISP_CONFIG_IN_VALUE_MINNUM: c_int = 1;
pub const MC_CMD_SET_PARSER_DISP_CONFIG_IN_VALUE_MAXNUM: c_int = 61;
pub const MC_CMD_SET_PARSER_DISP_CONFIG_IN_VALUE_MAXNUM_MCDI2: c_int = 253;
// MC_CMD_SET_PARSER_DISP_CONFIG_OUT msgresponse
pub const MC_CMD_SET_PARSER_DISP_CONFIG_OUT_LEN: c_int = 0;
//
// MC_CMD_GET_PARSER_DISP_CONFIG
// Read configuration related to the parser-dispatcher subsystem.
//
pub const MC_CMD_GET_PARSER_DISP_CONFIG: c_uint = 0xfa;

// MC_CMD_GET_PARSER_DISP_CONFIG_IN msgrequest
pub const MC_CMD_GET_PARSER_DISP_CONFIG_IN_LEN: c_int = 8;
// the type of configuration setting to read
pub const MC_CMD_GET_PARSER_DISP_CONFIG_IN_TYPE_OFST: c_int = 0;
pub const MC_CMD_GET_PARSER_DISP_CONFIG_IN_TYPE_LEN: c_int = 4;
// Enum values, see field(s):
// MC_CMD_SET_PARSER_DISP_CONFIG/MC_CMD_SET_PARSER_DISP_CONFIG_IN/TYPE
// handle for the entity to query: queue handle, EVB port ID, etc. depending on
// the type of configuration setting being read
//
pub const MC_CMD_GET_PARSER_DISP_CONFIG_IN_ENTITY_OFST: c_int = 4;
pub const MC_CMD_GET_PARSER_DISP_CONFIG_IN_ENTITY_LEN: c_int = 4;
// MC_CMD_GET_PARSER_DISP_CONFIG_OUT msgresponse
pub const MC_CMD_GET_PARSER_DISP_CONFIG_OUT_LENMIN: c_int = 4;
pub const MC_CMD_GET_PARSER_DISP_CONFIG_OUT_LENMAX: c_int = 252;
pub const MC_CMD_GET_PARSER_DISP_CONFIG_OUT_LENMAX_MCDI2: c_int = 1020;

// current value: the details depend on the type of configuration setting being
// read
//
pub const MC_CMD_GET_PARSER_DISP_CONFIG_OUT_VALUE_OFST: c_int = 0;
pub const MC_CMD_GET_PARSER_DISP_CONFIG_OUT_VALUE_LEN: c_int = 4;
pub const MC_CMD_GET_PARSER_DISP_CONFIG_OUT_VALUE_MINNUM: c_int = 1;
pub const MC_CMD_GET_PARSER_DISP_CONFIG_OUT_VALUE_MAXNUM: c_int = 63;
pub const MC_CMD_GET_PARSER_DISP_CONFIG_OUT_VALUE_MAXNUM_MCDI2: c_int = 255;
//
// MC_CMD_GET_PORT_MODES
// Find out about available port modes
//
pub const MC_CMD_GET_PORT_MODES: c_uint = 0xff;

// MC_CMD_GET_PORT_MODES_IN msgrequest
pub const MC_CMD_GET_PORT_MODES_IN_LEN: c_int = 0;
// MC_CMD_GET_PORT_MODES_OUT msgresponse
pub const MC_CMD_GET_PORT_MODES_OUT_LEN: c_int = 12;
// Bitmask of port modes available on the board (indexed by TLV_PORT_MODE_*)
// that are supported for customer use in production firmware.
//
pub const MC_CMD_GET_PORT_MODES_OUT_MODES_OFST: c_int = 0;
pub const MC_CMD_GET_PORT_MODES_OUT_MODES_LEN: c_int = 4;
// Default (canonical) board mode
pub const MC_CMD_GET_PORT_MODES_OUT_DEFAULT_MODE_OFST: c_int = 4;
pub const MC_CMD_GET_PORT_MODES_OUT_DEFAULT_MODE_LEN: c_int = 4;
// Current board mode
pub const MC_CMD_GET_PORT_MODES_OUT_CURRENT_MODE_OFST: c_int = 8;
pub const MC_CMD_GET_PORT_MODES_OUT_CURRENT_MODE_LEN: c_int = 4;
// MC_CMD_GET_PORT_MODES_OUT_V2 msgresponse
pub const MC_CMD_GET_PORT_MODES_OUT_V2_LEN: c_int = 16;
// Bitmask of port modes available on the board (indexed by TLV_PORT_MODE_*)
// that are supported for customer use in production firmware.
//
pub const MC_CMD_GET_PORT_MODES_OUT_V2_MODES_OFST: c_int = 0;
pub const MC_CMD_GET_PORT_MODES_OUT_V2_MODES_LEN: c_int = 4;
// Default (canonical) board mode
pub const MC_CMD_GET_PORT_MODES_OUT_V2_DEFAULT_MODE_OFST: c_int = 4;
pub const MC_CMD_GET_PORT_MODES_OUT_V2_DEFAULT_MODE_LEN: c_int = 4;
// Current board mode
pub const MC_CMD_GET_PORT_MODES_OUT_V2_CURRENT_MODE_OFST: c_int = 8;
pub const MC_CMD_GET_PORT_MODES_OUT_V2_CURRENT_MODE_LEN: c_int = 4;
// Bitmask of engineering port modes available on the board (indexed by
// TLV_PORT_MODE_*). A superset of MC_CMD_GET_PORT_MODES_OUT/MODES that
// contains all modes implemented in firmware for a particular board. Modes
// listed in MODES are considered production modes and should be exposed in
// userland tools. Modes listed in ENGINEERING_MODES, but not in MODES
// should be considered hidden (not to be exposed in userland tools) and for
// engineering use only. There are no other semantic differences and any mode
// listed in either MODES or ENGINEERING_MODES can be set on the board.
//
pub const MC_CMD_GET_PORT_MODES_OUT_V2_ENGINEERING_MODES_OFST: c_int = 12;
pub const MC_CMD_GET_PORT_MODES_OUT_V2_ENGINEERING_MODES_LEN: c_int = 4;
//
// MC_CMD_OVERRIDE_PORT_MODE
// Override flash config port mode for subsequent MC reboot(s). Override data
// is stored in the presistent data section of DMEM and activated on next MC
// warm reboot. A cold reboot resets the override. It is assumed that a
// sufficient number of PFs are available and that port mapping is valid for
// the new port mode, as the override does not affect PF configuration.
//
pub const MC_CMD_OVERRIDE_PORT_MODE: c_uint = 0x137;

// MC_CMD_OVERRIDE_PORT_MODE_IN msgrequest
pub const MC_CMD_OVERRIDE_PORT_MODE_IN_LEN: c_int = 8;
pub const MC_CMD_OVERRIDE_PORT_MODE_IN_FLAGS_OFST: c_int = 0;
pub const MC_CMD_OVERRIDE_PORT_MODE_IN_FLAGS_LEN: c_int = 4;
pub const MC_CMD_OVERRIDE_PORT_MODE_IN_ENABLE_OFST: c_int = 0;
pub const MC_CMD_OVERRIDE_PORT_MODE_IN_ENABLE_LBN: c_int = 0;
pub const MC_CMD_OVERRIDE_PORT_MODE_IN_ENABLE_WIDTH: c_int = 1;
// New mode (TLV_PORT_MODE_*) to set, if override enabled
pub const MC_CMD_OVERRIDE_PORT_MODE_IN_MODE_OFST: c_int = 4;
pub const MC_CMD_OVERRIDE_PORT_MODE_IN_MODE_LEN: c_int = 4;
// MC_CMD_OVERRIDE_PORT_MODE_OUT msgresponse
pub const MC_CMD_OVERRIDE_PORT_MODE_OUT_LEN: c_int = 0;
//
// MC_CMD_GET_WORKAROUNDS
// Read the list of all implemented and all currently enabled workarounds. The
// enums here must correspond with those in MC_CMD_WORKAROUND.
//
pub const MC_CMD_GET_WORKAROUNDS: c_uint = 0x59;

// MC_CMD_GET_WORKAROUNDS_OUT msgresponse
pub const MC_CMD_GET_WORKAROUNDS_OUT_LEN: c_int = 8;
// Each workaround is represented by a single bit according to the enums below.
//
pub const MC_CMD_GET_WORKAROUNDS_OUT_IMPLEMENTED_OFST: c_int = 0;
pub const MC_CMD_GET_WORKAROUNDS_OUT_IMPLEMENTED_LEN: c_int = 4;
pub const MC_CMD_GET_WORKAROUNDS_OUT_ENABLED_OFST: c_int = 4;
pub const MC_CMD_GET_WORKAROUNDS_OUT_ENABLED_LEN: c_int = 4;
// enum: Bug 17230 work around.
pub const MC_CMD_GET_WORKAROUNDS_OUT_BUG17230: c_uint = 0x2;
// enum: Bug 35388 work around (unsafe EVQ writes).
pub const MC_CMD_GET_WORKAROUNDS_OUT_BUG35388: c_uint = 0x4;
// enum: Bug35017 workaround (A64 tables must be identity map)
pub const MC_CMD_GET_WORKAROUNDS_OUT_BUG35017: c_uint = 0x8;
// enum: Bug 41750 present (MC_CMD_TRIGGER_INTERRUPT won't work)
pub const MC_CMD_GET_WORKAROUNDS_OUT_BUG41750: c_uint = 0x10;
// enum: Bug 42008 present (Interrupts can overtake associated events). Caution
// - before adding code that queries this workaround, remember that there's
// released Monza firmware that doesn't understand MC_CMD_WORKAROUND_BUG42008,
// and will hence (incorrectly) report that the bug doesn't exist.
//
pub const MC_CMD_GET_WORKAROUNDS_OUT_BUG42008: c_uint = 0x20;
// enum: Bug 26807 features present in firmware (multicast filter chaining)
pub const MC_CMD_GET_WORKAROUNDS_OUT_BUG26807: c_uint = 0x40;
// enum: Bug 61265 work around (broken EVQ TMR writes).
pub const MC_CMD_GET_WORKAROUNDS_OUT_BUG61265: c_uint = 0x80;
//
// MC_CMD_PRIVILEGE_MASK
// Read/set privileges of an arbitrary PCIe function
//
pub const MC_CMD_PRIVILEGE_MASK: c_uint = 0x5a;

// MC_CMD_PRIVILEGE_MASK_IN msgrequest
pub const MC_CMD_PRIVILEGE_MASK_IN_LEN: c_int = 8;
// The target function to have its mask read or set e.g. PF 0 = 0xFFFF0000, VF
// 1,3 = 0x00030001
//
pub const MC_CMD_PRIVILEGE_MASK_IN_FUNCTION_OFST: c_int = 0;
pub const MC_CMD_PRIVILEGE_MASK_IN_FUNCTION_LEN: c_int = 4;
pub const MC_CMD_PRIVILEGE_MASK_IN_FUNCTION_PF_OFST: c_int = 0;
pub const MC_CMD_PRIVILEGE_MASK_IN_FUNCTION_PF_LBN: c_int = 0;
pub const MC_CMD_PRIVILEGE_MASK_IN_FUNCTION_PF_WIDTH: c_int = 16;
pub const MC_CMD_PRIVILEGE_MASK_IN_FUNCTION_VF_OFST: c_int = 0;
pub const MC_CMD_PRIVILEGE_MASK_IN_FUNCTION_VF_LBN: c_int = 16;
pub const MC_CMD_PRIVILEGE_MASK_IN_FUNCTION_VF_WIDTH: c_int = 16;
pub const MC_CMD_PRIVILEGE_MASK_IN_VF_NULL: c_uint = 0xffff /* enum */;
// New privilege mask to be set. The mask will only be changed if the MSB is
// set to 1.
//
pub const MC_CMD_PRIVILEGE_MASK_IN_NEW_MASK_OFST: c_int = 4;
pub const MC_CMD_PRIVILEGE_MASK_IN_NEW_MASK_LEN: c_int = 4;
pub const MC_CMD_PRIVILEGE_MASK_IN_GRP_ADMIN: c_uint = 0x1 /* enum */;
pub const MC_CMD_PRIVILEGE_MASK_IN_GRP_LINK: c_uint = 0x2 /* enum */;
pub const MC_CMD_PRIVILEGE_MASK_IN_GRP_ONLOAD: c_uint = 0x4 /* enum */;
pub const MC_CMD_PRIVILEGE_MASK_IN_GRP_PTP: c_uint = 0x8 /* enum */;
pub const MC_CMD_PRIVILEGE_MASK_IN_GRP_INSECURE_FILTERS: c_uint = 0x10 /* enum */;
// enum: Deprecated. Equivalent to MAC_SPOOFING_TX combined with CHANGE_MAC.
pub const MC_CMD_PRIVILEGE_MASK_IN_GRP_MAC_SPOOFING: c_uint = 0x20;
pub const MC_CMD_PRIVILEGE_MASK_IN_GRP_UNICAST: c_uint = 0x40 /* enum */;
pub const MC_CMD_PRIVILEGE_MASK_IN_GRP_MULTICAST: c_uint = 0x80 /* enum */;
pub const MC_CMD_PRIVILEGE_MASK_IN_GRP_BROADCAST: c_uint = 0x100 /* enum */;
pub const MC_CMD_PRIVILEGE_MASK_IN_GRP_ALL_MULTICAST: c_uint = 0x200 /* enum */;
pub const MC_CMD_PRIVILEGE_MASK_IN_GRP_PROMISCUOUS: c_uint = 0x400 /* enum */;
// enum: Allows to set the TX packets' source MAC address to any arbitrary MAC
// adress.
//
pub const MC_CMD_PRIVILEGE_MASK_IN_GRP_MAC_SPOOFING_TX: c_uint = 0x800;
// enum: Privilege that allows a Function to change the MAC address configured
// in its associated vAdapter/vPort.
//
pub const MC_CMD_PRIVILEGE_MASK_IN_GRP_CHANGE_MAC: c_uint = 0x1000;
// enum: Privilege that allows a Function to install filters that specify VLANs
// that are not in the permit list for the associated vPort. This privilege is
// primarily to support ESX where vPorts are created that restrict traffic to
// only a set of permitted VLANs. See the vPort flag FLAG_VLAN_RESTRICT.
//
pub const MC_CMD_PRIVILEGE_MASK_IN_GRP_UNRESTRICTED_VLAN: c_uint = 0x2000;
// enum: Privilege for insecure commands. Commands that belong to this group
// are not permitted on secure adapters regardless of the privilege mask.
//
pub const MC_CMD_PRIVILEGE_MASK_IN_GRP_INSECURE: c_uint = 0x4000;
// enum: Trusted Server Adapter (TSA) / ServerLock. Privilege for
// administrator-level operations that are not allowed from the local host once
// an adapter has Bound to a remote ServerLock Controller (see doxbox
// SF-117064-DG for background).
//
pub const MC_CMD_PRIVILEGE_MASK_IN_GRP_ADMIN_TSA_UNBOUND: c_uint = 0x8000;
// enum: Set this bit to indicate that a new privilege mask is to be set,
// otherwise the command will only read the existing mask.
//
pub const MC_CMD_PRIVILEGE_MASK_IN_DO_CHANGE: c_uint = 0x80000000;
// MC_CMD_PRIVILEGE_MASK_OUT msgresponse
pub const MC_CMD_PRIVILEGE_MASK_OUT_LEN: c_int = 4;
// For an admin function, always all the privileges are reported.
pub const MC_CMD_PRIVILEGE_MASK_OUT_OLD_MASK_OFST: c_int = 0;
pub const MC_CMD_PRIVILEGE_MASK_OUT_OLD_MASK_LEN: c_int = 4;
//
// MC_CMD_LINK_STATE_MODE
// Read/set link state mode of a VF
//
pub const MC_CMD_LINK_STATE_MODE: c_uint = 0x5c;

// MC_CMD_LINK_STATE_MODE_IN msgrequest
pub const MC_CMD_LINK_STATE_MODE_IN_LEN: c_int = 8;
// The target function to have its link state mode read or set, must be a VF
// e.g. VF 1,3 = 0x00030001
//
pub const MC_CMD_LINK_STATE_MODE_IN_FUNCTION_OFST: c_int = 0;
pub const MC_CMD_LINK_STATE_MODE_IN_FUNCTION_LEN: c_int = 4;
pub const MC_CMD_LINK_STATE_MODE_IN_FUNCTION_PF_OFST: c_int = 0;
pub const MC_CMD_LINK_STATE_MODE_IN_FUNCTION_PF_LBN: c_int = 0;
pub const MC_CMD_LINK_STATE_MODE_IN_FUNCTION_PF_WIDTH: c_int = 16;
pub const MC_CMD_LINK_STATE_MODE_IN_FUNCTION_VF_OFST: c_int = 0;
pub const MC_CMD_LINK_STATE_MODE_IN_FUNCTION_VF_LBN: c_int = 16;
pub const MC_CMD_LINK_STATE_MODE_IN_FUNCTION_VF_WIDTH: c_int = 16;
// New link state mode to be set
pub const MC_CMD_LINK_STATE_MODE_IN_NEW_MODE_OFST: c_int = 4;
pub const MC_CMD_LINK_STATE_MODE_IN_NEW_MODE_LEN: c_int = 4;
pub const MC_CMD_LINK_STATE_MODE_IN_LINK_STATE_AUTO: c_uint = 0x0 /* enum */;
pub const MC_CMD_LINK_STATE_MODE_IN_LINK_STATE_UP: c_uint = 0x1 /* enum */;
pub const MC_CMD_LINK_STATE_MODE_IN_LINK_STATE_DOWN: c_uint = 0x2 /* enum */;
// enum: Use this value to just read the existing setting without modifying it.
//
pub const MC_CMD_LINK_STATE_MODE_IN_DO_NOT_CHANGE: c_uint = 0xffffffff;
// MC_CMD_LINK_STATE_MODE_OUT msgresponse
pub const MC_CMD_LINK_STATE_MODE_OUT_LEN: c_int = 4;
pub const MC_CMD_LINK_STATE_MODE_OUT_OLD_MODE_OFST: c_int = 0;
pub const MC_CMD_LINK_STATE_MODE_OUT_OLD_MODE_LEN: c_int = 4;
//
// MC_CMD_FUSE_DIAGS
// Additional fuse diagnostics
//
pub const MC_CMD_FUSE_DIAGS: c_uint = 0x102;

// MC_CMD_FUSE_DIAGS_IN msgrequest
pub const MC_CMD_FUSE_DIAGS_IN_LEN: c_int = 0;
// MC_CMD_FUSE_DIAGS_OUT msgresponse
pub const MC_CMD_FUSE_DIAGS_OUT_LEN: c_int = 48;
// Total number of mismatched bits between pairs in area 0
pub const MC_CMD_FUSE_DIAGS_OUT_AREA0_MISMATCH_BITS_OFST: c_int = 0;
pub const MC_CMD_FUSE_DIAGS_OUT_AREA0_MISMATCH_BITS_LEN: c_int = 4;
// Total number of unexpectedly clear (set in B but not A) bits in area 0
pub const MC_CMD_FUSE_DIAGS_OUT_AREA0_PAIR_A_BAD_BITS_OFST: c_int = 4;
pub const MC_CMD_FUSE_DIAGS_OUT_AREA0_PAIR_A_BAD_BITS_LEN: c_int = 4;
// Total number of unexpectedly clear (set in A but not B) bits in area 0
pub const MC_CMD_FUSE_DIAGS_OUT_AREA0_PAIR_B_BAD_BITS_OFST: c_int = 8;
pub const MC_CMD_FUSE_DIAGS_OUT_AREA0_PAIR_B_BAD_BITS_LEN: c_int = 4;
// Checksum of data after logical OR of pairs in area 0
pub const MC_CMD_FUSE_DIAGS_OUT_AREA0_CHECKSUM_OFST: c_int = 12;
pub const MC_CMD_FUSE_DIAGS_OUT_AREA0_CHECKSUM_LEN: c_int = 4;
// Total number of mismatched bits between pairs in area 1
pub const MC_CMD_FUSE_DIAGS_OUT_AREA1_MISMATCH_BITS_OFST: c_int = 16;
pub const MC_CMD_FUSE_DIAGS_OUT_AREA1_MISMATCH_BITS_LEN: c_int = 4;
// Total number of unexpectedly clear (set in B but not A) bits in area 1
pub const MC_CMD_FUSE_DIAGS_OUT_AREA1_PAIR_A_BAD_BITS_OFST: c_int = 20;
pub const MC_CMD_FUSE_DIAGS_OUT_AREA1_PAIR_A_BAD_BITS_LEN: c_int = 4;
// Total number of unexpectedly clear (set in A but not B) bits in area 1
pub const MC_CMD_FUSE_DIAGS_OUT_AREA1_PAIR_B_BAD_BITS_OFST: c_int = 24;
pub const MC_CMD_FUSE_DIAGS_OUT_AREA1_PAIR_B_BAD_BITS_LEN: c_int = 4;
// Checksum of data after logical OR of pairs in area 1
pub const MC_CMD_FUSE_DIAGS_OUT_AREA1_CHECKSUM_OFST: c_int = 28;
pub const MC_CMD_FUSE_DIAGS_OUT_AREA1_CHECKSUM_LEN: c_int = 4;
// Total number of mismatched bits between pairs in area 2
pub const MC_CMD_FUSE_DIAGS_OUT_AREA2_MISMATCH_BITS_OFST: c_int = 32;
pub const MC_CMD_FUSE_DIAGS_OUT_AREA2_MISMATCH_BITS_LEN: c_int = 4;
// Total number of unexpectedly clear (set in B but not A) bits in area 2
pub const MC_CMD_FUSE_DIAGS_OUT_AREA2_PAIR_A_BAD_BITS_OFST: c_int = 36;
pub const MC_CMD_FUSE_DIAGS_OUT_AREA2_PAIR_A_BAD_BITS_LEN: c_int = 4;
// Total number of unexpectedly clear (set in A but not B) bits in area 2
pub const MC_CMD_FUSE_DIAGS_OUT_AREA2_PAIR_B_BAD_BITS_OFST: c_int = 40;
pub const MC_CMD_FUSE_DIAGS_OUT_AREA2_PAIR_B_BAD_BITS_LEN: c_int = 4;
// Checksum of data after logical OR of pairs in area 2
pub const MC_CMD_FUSE_DIAGS_OUT_AREA2_CHECKSUM_OFST: c_int = 44;
pub const MC_CMD_FUSE_DIAGS_OUT_AREA2_CHECKSUM_LEN: c_int = 4;
//
// MC_CMD_PRIVILEGE_MODIFY
// Modify the privileges of a set of PCIe functions. Note that this operation
// only effects non-admin functions unless the admin privilege itself is
// included in one of the masks provided.
//
pub const MC_CMD_PRIVILEGE_MODIFY: c_uint = 0x60;

// MC_CMD_PRIVILEGE_MODIFY_IN msgrequest
pub const MC_CMD_PRIVILEGE_MODIFY_IN_LEN: c_int = 16;
// The groups of functions to have their privilege masks modified.
pub const MC_CMD_PRIVILEGE_MODIFY_IN_FN_GROUP_OFST: c_int = 0;
pub const MC_CMD_PRIVILEGE_MODIFY_IN_FN_GROUP_LEN: c_int = 4;
pub const MC_CMD_PRIVILEGE_MODIFY_IN_NONE: c_uint = 0x0 /* enum */;
pub const MC_CMD_PRIVILEGE_MODIFY_IN_ALL: c_uint = 0x1 /* enum */;
pub const MC_CMD_PRIVILEGE_MODIFY_IN_PFS_ONLY: c_uint = 0x2 /* enum */;
pub const MC_CMD_PRIVILEGE_MODIFY_IN_VFS_ONLY: c_uint = 0x3 /* enum */;
pub const MC_CMD_PRIVILEGE_MODIFY_IN_VFS_OF_PF: c_uint = 0x4 /* enum */;
pub const MC_CMD_PRIVILEGE_MODIFY_IN_ONE: c_uint = 0x5 /* enum */;
// For VFS_OF_PF specify the PF, for ONE specify the target function
pub const MC_CMD_PRIVILEGE_MODIFY_IN_FUNCTION_OFST: c_int = 4;
pub const MC_CMD_PRIVILEGE_MODIFY_IN_FUNCTION_LEN: c_int = 4;
pub const MC_CMD_PRIVILEGE_MODIFY_IN_FUNCTION_PF_OFST: c_int = 4;
pub const MC_CMD_PRIVILEGE_MODIFY_IN_FUNCTION_PF_LBN: c_int = 0;
pub const MC_CMD_PRIVILEGE_MODIFY_IN_FUNCTION_PF_WIDTH: c_int = 16;
pub const MC_CMD_PRIVILEGE_MODIFY_IN_FUNCTION_VF_OFST: c_int = 4;
pub const MC_CMD_PRIVILEGE_MODIFY_IN_FUNCTION_VF_LBN: c_int = 16;
pub const MC_CMD_PRIVILEGE_MODIFY_IN_FUNCTION_VF_WIDTH: c_int = 16;
// Privileges to be added to the target functions. For privilege definitions
// refer to the command MC_CMD_PRIVILEGE_MASK
//
pub const MC_CMD_PRIVILEGE_MODIFY_IN_ADD_MASK_OFST: c_int = 8;
pub const MC_CMD_PRIVILEGE_MODIFY_IN_ADD_MASK_LEN: c_int = 4;
// Privileges to be removed from the target functions. For privilege
// definitions refer to the command MC_CMD_PRIVILEGE_MASK
//
pub const MC_CMD_PRIVILEGE_MODIFY_IN_REMOVE_MASK_OFST: c_int = 12;
pub const MC_CMD_PRIVILEGE_MODIFY_IN_REMOVE_MASK_LEN: c_int = 4;
// MC_CMD_PRIVILEGE_MODIFY_OUT msgresponse
pub const MC_CMD_PRIVILEGE_MODIFY_OUT_LEN: c_int = 0;
// TUNNEL_ENCAP_UDP_PORT_ENTRY structuredef
pub const TUNNEL_ENCAP_UDP_PORT_ENTRY_LEN: c_int = 4;
// UDP port (the standard ports are named below but any port may be used)
pub const TUNNEL_ENCAP_UDP_PORT_ENTRY_UDP_PORT_OFST: c_int = 0;
pub const TUNNEL_ENCAP_UDP_PORT_ENTRY_UDP_PORT_LEN: c_int = 2;
// enum: the IANA allocated UDP port for VXLAN
pub const TUNNEL_ENCAP_UDP_PORT_ENTRY_IANA_VXLAN_UDP_PORT: c_uint = 0x12b5;
// enum: the IANA allocated UDP port for Geneve
pub const TUNNEL_ENCAP_UDP_PORT_ENTRY_IANA_GENEVE_UDP_PORT: c_uint = 0x17c1;
pub const TUNNEL_ENCAP_UDP_PORT_ENTRY_UDP_PORT_LBN: c_int = 0;
pub const TUNNEL_ENCAP_UDP_PORT_ENTRY_UDP_PORT_WIDTH: c_int = 16;
// tunnel encapsulation protocol (only those named below are supported)
pub const TUNNEL_ENCAP_UDP_PORT_ENTRY_PROTOCOL_OFST: c_int = 2;
pub const TUNNEL_ENCAP_UDP_PORT_ENTRY_PROTOCOL_LEN: c_int = 2;
// enum: This port will be used for VXLAN on both IPv4 and IPv6
pub const TUNNEL_ENCAP_UDP_PORT_ENTRY_VXLAN: c_uint = 0x0;
// enum: This port will be used for Geneve on both IPv4 and IPv6
pub const TUNNEL_ENCAP_UDP_PORT_ENTRY_GENEVE: c_uint = 0x1;
pub const TUNNEL_ENCAP_UDP_PORT_ENTRY_PROTOCOL_LBN: c_int = 16;
pub const TUNNEL_ENCAP_UDP_PORT_ENTRY_PROTOCOL_WIDTH: c_int = 16;
//
// MC_CMD_SET_TUNNEL_ENCAP_UDP_PORTS
// Configure UDP ports for tunnel encapsulation hardware acceleration. The
// parser-dispatcher will attempt to parse traffic on these ports as tunnel
// encapsulation PDUs and filter them using the tunnel encapsulation filter
// chain rather than the standard filter chain. Note that this command can
// cause all functions to see a reset. (Available on Medford only.)
//
pub const MC_CMD_SET_TUNNEL_ENCAP_UDP_PORTS: c_uint = 0x117;

// MC_CMD_SET_TUNNEL_ENCAP_UDP_PORTS_IN msgrequest
pub const MC_CMD_SET_TUNNEL_ENCAP_UDP_PORTS_IN_LENMIN: c_int = 4;
pub const MC_CMD_SET_TUNNEL_ENCAP_UDP_PORTS_IN_LENMAX: c_int = 68;
pub const MC_CMD_SET_TUNNEL_ENCAP_UDP_PORTS_IN_LENMAX_MCDI2: c_int = 68;

// Flags
pub const MC_CMD_SET_TUNNEL_ENCAP_UDP_PORTS_IN_FLAGS_OFST: c_int = 0;
pub const MC_CMD_SET_TUNNEL_ENCAP_UDP_PORTS_IN_FLAGS_LEN: c_int = 2;
pub const MC_CMD_SET_TUNNEL_ENCAP_UDP_PORTS_IN_UNLOADING_OFST: c_int = 0;
pub const MC_CMD_SET_TUNNEL_ENCAP_UDP_PORTS_IN_UNLOADING_LBN: c_int = 0;
pub const MC_CMD_SET_TUNNEL_ENCAP_UDP_PORTS_IN_UNLOADING_WIDTH: c_int = 1;
// The number of entries in the ENTRIES array
pub const MC_CMD_SET_TUNNEL_ENCAP_UDP_PORTS_IN_NUM_ENTRIES_OFST: c_int = 2;
pub const MC_CMD_SET_TUNNEL_ENCAP_UDP_PORTS_IN_NUM_ENTRIES_LEN: c_int = 2;
// Entries defining the UDP port to protocol mapping, each laid out as a
// TUNNEL_ENCAP_UDP_PORT_ENTRY
//
pub const MC_CMD_SET_TUNNEL_ENCAP_UDP_PORTS_IN_ENTRIES_OFST: c_int = 4;
pub const MC_CMD_SET_TUNNEL_ENCAP_UDP_PORTS_IN_ENTRIES_LEN: c_int = 4;
pub const MC_CMD_SET_TUNNEL_ENCAP_UDP_PORTS_IN_ENTRIES_MINNUM: c_int = 0;
pub const MC_CMD_SET_TUNNEL_ENCAP_UDP_PORTS_IN_ENTRIES_MAXNUM: c_int = 16;
pub const MC_CMD_SET_TUNNEL_ENCAP_UDP_PORTS_IN_ENTRIES_MAXNUM_MCDI2: c_int = 16;
// MC_CMD_SET_TUNNEL_ENCAP_UDP_PORTS_OUT msgresponse
pub const MC_CMD_SET_TUNNEL_ENCAP_UDP_PORTS_OUT_LEN: c_int = 2;
// Flags
pub const MC_CMD_SET_TUNNEL_ENCAP_UDP_PORTS_OUT_FLAGS_OFST: c_int = 0;
pub const MC_CMD_SET_TUNNEL_ENCAP_UDP_PORTS_OUT_FLAGS_LEN: c_int = 2;
pub const MC_CMD_SET_TUNNEL_ENCAP_UDP_PORTS_OUT_RESETTING_OFST: c_int = 0;
pub const MC_CMD_SET_TUNNEL_ENCAP_UDP_PORTS_OUT_RESETTING_LBN: c_int = 0;
pub const MC_CMD_SET_TUNNEL_ENCAP_UDP_PORTS_OUT_RESETTING_WIDTH: c_int = 1;
//
// MC_CMD_VNIC_ENCAP_RULE_ADD
// Add a rule for detecting encapsulations in the VNIC stage. Currently this only affects checksum validation in VNIC RX - on TX the send descriptor explicitly specifies encapsulation. These rules are per-VNIC, i.e. only apply to the current driver. If a rule matches, then the packet is considered to have the corresponding encapsulation type, and the inner packet is parsed. It is up to the driver to ensure that overlapping rules are not inserted. (If a packet would match multiple rules, a random one of them will be used.) A rule with the exact same match criteria may not be inserted twice (EALREADY). Only a limited number MATCH_FLAGS values are supported, use MC_CMD_GET_PARSER_DISP_INFO with OP OP_GET_SUPPORTED_VNIC_ENCAP_RULE_MATCHES to get a list of supported combinations. Each driver may only have a limited set of active rules - returns ENOSPC if the caller's table is full.
//
pub const MC_CMD_VNIC_ENCAP_RULE_ADD: c_uint = 0x16d;

// MC_CMD_VNIC_ENCAP_RULE_ADD_IN msgrequest
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_LEN: c_int = 36;
// Set to MAE_MPORT_SELECTOR_ASSIGNED. In the future this may be relaxed.
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_MPORT_SELECTOR_OFST: c_int = 0;
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_MPORT_SELECTOR_LEN: c_int = 4;
// Any non-zero bits other than the ones named below or an unsupported
// combination will cause the NIC to return EOPNOTSUPP. In the future more
// flags may be added.
//
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_MATCH_FLAGS_OFST: c_int = 4;
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_MATCH_FLAGS_LEN: c_int = 4;
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_MATCH_ETHER_TYPE_OFST: c_int = 4;
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_MATCH_ETHER_TYPE_LBN: c_int = 0;
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_MATCH_ETHER_TYPE_WIDTH: c_int = 1;
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_MATCH_OUTER_VLAN_OFST: c_int = 4;
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_MATCH_OUTER_VLAN_LBN: c_int = 1;
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_MATCH_OUTER_VLAN_WIDTH: c_int = 1;
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_MATCH_DST_IP_OFST: c_int = 4;
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_MATCH_DST_IP_LBN: c_int = 2;
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_MATCH_DST_IP_WIDTH: c_int = 1;
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_MATCH_IP_PROTO_OFST: c_int = 4;
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_MATCH_IP_PROTO_LBN: c_int = 3;
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_MATCH_IP_PROTO_WIDTH: c_int = 1;
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_MATCH_DST_PORT_OFST: c_int = 4;
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_MATCH_DST_PORT_LBN: c_int = 4;
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_MATCH_DST_PORT_WIDTH: c_int = 1;
// Only if MATCH_ETHER_TYPE is set. Ethertype value as bytes in network order.
// Currently only IPv4 (0x0800) and IPv6 (0x86DD) ethertypes may be used.
//
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_ETHER_TYPE_OFST: c_int = 8;
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_ETHER_TYPE_LEN: c_int = 2;
// Only if MATCH_OUTER_VLAN is set. VID value as bytes in network order.
// (Deprecated)
//
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_OUTER_VLAN_LBN: c_int = 80;
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_OUTER_VLAN_WIDTH: c_int = 12;
// Only if MATCH_OUTER_VLAN is set. Aligned wrapper for OUTER_VLAN_VID.
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_OUTER_VLAN_WORD_OFST: c_int = 10;
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_OUTER_VLAN_WORD_LEN: c_int = 2;
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_OUTER_VLAN_VID_OFST: c_int = 10;
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_OUTER_VLAN_VID_LBN: c_int = 0;
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_OUTER_VLAN_VID_WIDTH: c_int = 12;
// Only if MATCH_DST_IP is set. IP address as bytes in network order. In the
// case of IPv4, the IP should be in the first 4 bytes and all other bytes
// should be zero.
//
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_DST_IP_OFST: c_int = 12;
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_DST_IP_LEN: c_int = 16;
// Only if MATCH_IP_PROTO is set. Currently only UDP proto (17) may be used.
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_IP_PROTO_OFST: c_int = 28;
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_IP_PROTO_LEN: c_int = 1;
// Actions that should be applied to packets match the rule.
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_ACTION_FLAGS_OFST: c_int = 29;
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_ACTION_FLAGS_LEN: c_int = 1;
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_STRIP_OUTER_VLAN_OFST: c_int = 29;
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_STRIP_OUTER_VLAN_LBN: c_int = 0;
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_STRIP_OUTER_VLAN_WIDTH: c_int = 1;
// Only if MATCH_DST_PORT is set. Port number as bytes in network order.
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_DST_PORT_OFST: c_int = 30;
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_DST_PORT_LEN: c_int = 2;
// Resulting encapsulation type, as per MAE_MCDI_ENCAP_TYPE enumeration.
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_ENCAP_TYPE_OFST: c_int = 32;
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_IN_ENCAP_TYPE_LEN: c_int = 4;
// MC_CMD_VNIC_ENCAP_RULE_ADD_OUT msgresponse
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_OUT_LEN: c_int = 4;
// Handle to inserted rule. Used for removing the rule.
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_OUT_HANDLE_OFST: c_int = 0;
pub const MC_CMD_VNIC_ENCAP_RULE_ADD_OUT_HANDLE_LEN: c_int = 4;
//
// MC_CMD_VNIC_ENCAP_RULE_REMOVE
// Remove a VNIC encapsulation rule. Packets which would have previously matched the rule will then be considered as unencapsulated. Returns EALREADY if the input HANDLE doesn't correspond to an existing rule.
//
pub const MC_CMD_VNIC_ENCAP_RULE_REMOVE: c_uint = 0x16e;

// MC_CMD_VNIC_ENCAP_RULE_REMOVE_IN msgrequest
pub const MC_CMD_VNIC_ENCAP_RULE_REMOVE_IN_LEN: c_int = 4;
// Handle which was returned by MC_CMD_VNIC_ENCAP_RULE_ADD.
pub const MC_CMD_VNIC_ENCAP_RULE_REMOVE_IN_HANDLE_OFST: c_int = 0;
pub const MC_CMD_VNIC_ENCAP_RULE_REMOVE_IN_HANDLE_LEN: c_int = 4;
// MC_CMD_VNIC_ENCAP_RULE_REMOVE_OUT msgresponse
pub const MC_CMD_VNIC_ENCAP_RULE_REMOVE_OUT_LEN: c_int = 0;
// FUNCTION_PERSONALITY structuredef: The meanings of the personalities are
// defined in SF-120734-TC with more information in SF-122717-TC.
//
pub const FUNCTION_PERSONALITY_LEN: c_int = 4;
pub const FUNCTION_PERSONALITY_ID_OFST: c_int = 0;
pub const FUNCTION_PERSONALITY_ID_LEN: c_int = 4;
// enum: Function has no assigned personality
pub const FUNCTION_PERSONALITY_NULL: c_uint = 0x0;
// enum: Function has an EF100-style function control window and VI windows
// with both EF100 and vDPA doorbells.
//
pub const FUNCTION_PERSONALITY_EF100: c_uint = 0x1;
// enum: Function has virtio net device configuration registers and doorbells
// for virtio queue pairs.
//
pub const FUNCTION_PERSONALITY_VIRTIO_NET: c_uint = 0x2;
// enum: Function has virtio block device configuration registers and a
// doorbell for a single virtqueue.
//
pub const FUNCTION_PERSONALITY_VIRTIO_BLK: c_uint = 0x3;
// enum: Function is a Xilinx acceleration device - management function
pub const FUNCTION_PERSONALITY_ACCEL_MGMT: c_uint = 0x4;
// enum: Function is a Xilinx acceleration device - user function
pub const FUNCTION_PERSONALITY_ACCEL_USR: c_uint = 0x5;
pub const FUNCTION_PERSONALITY_ID_LBN: c_int = 0;
pub const FUNCTION_PERSONALITY_ID_WIDTH: c_int = 32;
// PCIE_FUNCTION structuredef: Structure representing a PCIe function ID
// (interface/PF/VF tuple)
//
pub const PCIE_FUNCTION_LEN: c_int = 8;
// PCIe PF function number
pub const PCIE_FUNCTION_PF_OFST: c_int = 0;
pub const PCIE_FUNCTION_PF_LEN: c_int = 2;
// enum: Wildcard value representing any available function (e.g in resource
// allocation requests)
//
pub const PCIE_FUNCTION_PF_ANY: c_uint = 0xfffe;
// enum: Value representing invalid (null) function
pub const PCIE_FUNCTION_PF_NULL: c_uint = 0xffff;
pub const PCIE_FUNCTION_PF_LBN: c_int = 0;
pub const PCIE_FUNCTION_PF_WIDTH: c_int = 16;
// PCIe VF Function number (PF relative)
pub const PCIE_FUNCTION_VF_OFST: c_int = 2;
pub const PCIE_FUNCTION_VF_LEN: c_int = 2;
// enum: Wildcard value representing any available function (e.g in resource
// allocation requests)
//
pub const PCIE_FUNCTION_VF_ANY: c_uint = 0xfffe;
// enum: Function is a PF (when PF != PF_NULL) or invalid function (when PF ==
// PF_NULL)
//
pub const PCIE_FUNCTION_VF_NULL: c_uint = 0xffff;
pub const PCIE_FUNCTION_VF_LBN: c_int = 16;
pub const PCIE_FUNCTION_VF_WIDTH: c_int = 16;
// PCIe interface of the function
pub const PCIE_FUNCTION_INTF_OFST: c_int = 4;
pub const PCIE_FUNCTION_INTF_LEN: c_int = 4;
// enum: Host PCIe interface
pub const PCIE_FUNCTION_INTF_HOST: c_uint = 0x0;
// enum: Application Processor interface
pub const PCIE_FUNCTION_INTF_AP: c_uint = 0x1;
pub const PCIE_FUNCTION_INTF_LBN: c_int = 32;
pub const PCIE_FUNCTION_INTF_WIDTH: c_int = 32;
