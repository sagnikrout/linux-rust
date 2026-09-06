//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/serial/io_usbvend.h
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
// USBVEND.H		Vendor-specific USB definitions
//
// NOTE: This must be kept in sync with the Edgeport firmware and
// must be kept backward-compatible with older firmware.
//
// Copyright (C) 1998 Inside Out Networks, Inc.
//

//
// D e f i n e s   /   T y p e d e f s
//
// Definitions of USB product IDs
//
pub const USB_VENDOR_ID_ION: c_uint = 0x1608		// Our VID;
pub const USB_VENDOR_ID_TI: c_uint = 0x0451		// TI VID;
pub const USB_VENDOR_ID_AXIOHM: c_uint = 0x05D9		/* Axiohm VID */;
//
// Definitions of USB product IDs (PID)
// We break the USB-defined PID into an OEM Id field (upper 6 bits)
// and a Device Id (bottom 10 bits). The Device Id defines what
// device this actually is regardless of what the OEM wants to
// call it.
//
// ION-device OEM IDs

// ION-device Device IDs
// Product IDs - assigned to match middle digit of serial number (No longer true)
pub const ION_DEVICE_ID_80251_NETCHIP: c_uint = 0x020	// This bit is set in the PID if this edgeport hardware$;
// is based on the 80251+Netchip.
pub const ION_DEVICE_ID_GENERATION_1: c_uint = 0x00	// Value for 930 based edgeports;
pub const ION_DEVICE_ID_GENERATION_2: c_uint = 0x01	// Value for 80251+Netchip.;
pub const ION_DEVICE_ID_GENERATION_3: c_uint = 0x02	// Value for Texas Instruments TUSB5052 chip;
pub const ION_DEVICE_ID_GENERATION_4: c_uint = 0x03	// Watchport Family of products;
pub const ION_GENERATION_MASK: c_uint = 0x03;
pub const ION_DEVICE_ID_HUB_MASK: c_uint = 0x0080	// This bit in the PID designates a HUB device;
// for example 8C would be a 421 4 port hub
// and 8D would be a 2 port embedded hub
pub const EDGEPORT_DEVICE_ID_MASK: c_uint = 0x0ff	// Not including OEM or GENERATION fields;
pub const ION_DEVICE_ID_UNCONFIGURED_EDGE_DEVICE: c_uint = 0x000	// In manufacturing only;
pub const ION_DEVICE_ID_EDGEPORT_4: c_uint = 0x001	// Edgeport/4 RS232;
pub const ION_DEVICE_ID_EDGEPORT_8R: c_uint = 0x002	// Edgeport with RJ45 no Ring;
pub const ION_DEVICE_ID_RAPIDPORT_4: c_uint = 0x003	// Rapidport/4;
pub const ION_DEVICE_ID_EDGEPORT_4T: c_uint = 0x004	// Edgeport/4 RS232 for Telxon (aka "Fleetport");
pub const ION_DEVICE_ID_EDGEPORT_2: c_uint = 0x005	// Edgeport/2 RS232;
pub const ION_DEVICE_ID_EDGEPORT_4I: c_uint = 0x006	// Edgeport/4 RS422;
pub const ION_DEVICE_ID_EDGEPORT_2I: c_uint = 0x007	// Edgeport/2 RS422/RS485;
pub const ION_DEVICE_ID_EDGEPORT_8RR: c_uint = 0x008	// Edgeport with RJ45 with Data and RTS/CTS only;
// ION_DEVICE_ID_EDGEPORT_8_HANDBUILT	0x009	// Hand-built Edgeport/8 (Placeholder, used in middle digit of serial number only!)
// ION_DEVICE_ID_MULTIMODEM_4X56		0x00A	// MultiTech version of RP/4 (Placeholder, used in middle digit of serial number only!)
pub const ION_DEVICE_ID_EDGEPORT_PARALLEL_PORT: c_uint = 0x00B	// Edgeport/(4)21 Parallel port (USS720);
pub const ION_DEVICE_ID_EDGEPORT_421: c_uint = 0x00C	// Edgeport/421 Hub+RS232+Parallel;
pub const ION_DEVICE_ID_EDGEPORT_21: c_uint = 0x00D	// Edgeport/21  RS232+Parallel;
pub const ION_DEVICE_ID_EDGEPORT_8_DUAL_CPU: c_uint = 0x00E	// Half of an Edgeport/8 (the kind with 2 EP/4s on 1 PCB);
pub const ION_DEVICE_ID_EDGEPORT_8: c_uint = 0x00F	// Edgeport/8 (single-CPU);
pub const ION_DEVICE_ID_EDGEPORT_2_DIN: c_uint = 0x010	// Edgeport/2 RS232 with Apple DIN connector;
pub const ION_DEVICE_ID_EDGEPORT_4_DIN: c_uint = 0x011	// Edgeport/4 RS232 with Apple DIN connector;
pub const ION_DEVICE_ID_EDGEPORT_16_DUAL_CPU: c_uint = 0x012	// Half of an Edgeport/16 (the kind with 2 EP/8s);
pub const ION_DEVICE_ID_EDGEPORT_COMPATIBLE: c_uint = 0x013	// Edgeport Compatible, for NCR, Axiohm etc. testing;
pub const ION_DEVICE_ID_EDGEPORT_8I: c_uint = 0x014	// Edgeport/8 RS422 (single-CPU);
pub const ION_DEVICE_ID_EDGEPORT_1: c_uint = 0x015	// Edgeport/1 RS232;
pub const ION_DEVICE_ID_EPOS44: c_uint = 0x016	// Half of an EPOS/44 (TIUMP BASED);
pub const ION_DEVICE_ID_EDGEPORT_42: c_uint = 0x017	// Edgeport/42;
pub const ION_DEVICE_ID_EDGEPORT_412_8: c_uint = 0x018	// Edgeport/412 8 port part;
pub const ION_DEVICE_ID_EDGEPORT_412_4: c_uint = 0x019	// Edgeport/412	4 port part;
pub const ION_DEVICE_ID_EDGEPORT_22I: c_uint = 0x01A	// Edgeport/22I is an Edgeport/4 with ports 1&2 RS422 and ports 3&4 RS232;
// Compact Form factor TI based devices  2c, 21c, 22c, 221c
pub const ION_DEVICE_ID_EDGEPORT_2C: c_uint = 0x01B	// Edgeport/2c is a TI based Edgeport/2 - Small I2c;
pub const ION_DEVICE_ID_EDGEPORT_221C: c_uint = 0x01C	// Edgeport/221c is a TI based Edgeport/2 with lucent chip and;
// 2 external hub ports - Large I2C
pub const ION_DEVICE_ID_EDGEPORT_22C: c_uint = 0x01D	// Edgeport/22c is a TI based Edgeport/2 with;
// 2 external hub ports - Large I2C
pub const ION_DEVICE_ID_EDGEPORT_21C: c_uint = 0x01E	// Edgeport/21c is a TI based Edgeport/2 with lucent chip;
// Small I2C
//
// DANGER DANGER The 0x20 bit was used to indicate a 8251/netchip GEN 2 device.
// Since the MAC, Linux, and Optimal drivers still used the old code
// I suggest that you skip the 0x20 bit when creating new PIDs
//
// Generation 3 devices -- 3410 based edgport/1 (256 byte I2C)
pub const ION_DEVICE_ID_TI3410_EDGEPORT_1: c_uint = 0x040	// Edgeport/1 RS232;
pub const ION_DEVICE_ID_TI3410_EDGEPORT_1I: c_uint = 0x041	// Edgeport/1i- RS422 model;
// Ti based software switchable RS232/RS422/RS485 devices
pub const ION_DEVICE_ID_EDGEPORT_4S: c_uint = 0x042	// Edgeport/4s - software switchable model;
pub const ION_DEVICE_ID_EDGEPORT_8S: c_uint = 0x043	// Edgeport/8s - software switchable model;
// Usb to Ethernet dongle
pub const ION_DEVICE_ID_EDGEPORT_E: c_uint = 0x0E0	// Edgeport/E Usb to Ethernet;
// Edgeport TI based devices
pub const ION_DEVICE_ID_TI_EDGEPORT_4: c_uint = 0x0201	// Edgeport/4 RS232;
pub const ION_DEVICE_ID_TI_EDGEPORT_2: c_uint = 0x0205	// Edgeport/2 RS232;
pub const ION_DEVICE_ID_TI_EDGEPORT_4I: c_uint = 0x0206	// Edgeport/4i RS422;
pub const ION_DEVICE_ID_TI_EDGEPORT_2I: c_uint = 0x0207	// Edgeport/2i RS422/RS485;
pub const ION_DEVICE_ID_TI_EDGEPORT_421: c_uint = 0x020C	// Edgeport/421 4 hub 2 RS232 + Parallel (lucent on a different hub port);
pub const ION_DEVICE_ID_TI_EDGEPORT_21: c_uint = 0x020D	// Edgeport/21 2 RS232 + Parallel (lucent on a different hub port);
pub const ION_DEVICE_ID_TI_EDGEPORT_416: c_uint = 0x0212  // Edgeport/416;
pub const ION_DEVICE_ID_TI_EDGEPORT_1: c_uint = 0x0215	// Edgeport/1 RS232;
pub const ION_DEVICE_ID_TI_EDGEPORT_42: c_uint = 0x0217	// Edgeport/42 4 hub 2 RS232;
pub const ION_DEVICE_ID_TI_EDGEPORT_22I: c_uint = 0x021A	// Edgeport/22I is an Edgeport/4 with ports 1&2 RS422 and ports 3&4 RS232;
pub const ION_DEVICE_ID_TI_EDGEPORT_2C: c_uint = 0x021B	// Edgeport/2c RS232;
pub const ION_DEVICE_ID_TI_EDGEPORT_221C: c_uint = 0x021C	// Edgeport/221c is a TI based Edgeport/2 with lucent chip and;
// 2 external hub ports - Large I2C
pub const ION_DEVICE_ID_TI_EDGEPORT_22C: c_uint = 0x021D	// Edgeport/22c is a TI based Edgeport/2 with;
// 2 external hub ports - Large I2C
pub const ION_DEVICE_ID_TI_EDGEPORT_21C: c_uint = 0x021E	// Edgeport/21c is a TI based Edgeport/2 with lucent chip;
// Generation 3 devices -- 3410 based edgport/1 (256 byte I2C)
pub const ION_DEVICE_ID_TI_TI3410_EDGEPORT_1: c_uint = 0x0240	// Edgeport/1 RS232;
pub const ION_DEVICE_ID_TI_TI3410_EDGEPORT_1I: c_uint = 0x0241	// Edgeport/1i- RS422 model;
// Ti based software switchable RS232/RS422/RS485 devices
pub const ION_DEVICE_ID_TI_EDGEPORT_4S: c_uint = 0x0242	// Edgeport/4s - software switchable model;
pub const ION_DEVICE_ID_TI_EDGEPORT_8S: c_uint = 0x0243	// Edgeport/8s - software switchable model;
pub const ION_DEVICE_ID_TI_EDGEPORT_8: c_uint = 0x0244	// Edgeport/8 (single-CPU);
pub const ION_DEVICE_ID_TI_EDGEPORT_416B: c_uint = 0x0247	// Edgeport/416;
//
// Generation 4 devices
//
// Watchport based on 3410 both 1-wire and binary products (16K I2C)
pub const ION_DEVICE_ID_WP_UNSERIALIZED: c_uint = 0x300	// Watchport based on 3410 both 1-wire and binary products;
pub const ION_DEVICE_ID_WP_PROXIMITY: c_uint = 0x301	// Watchport/P Discontinued;
pub const ION_DEVICE_ID_WP_MOTION: c_uint = 0x302	// Watchport/M;
pub const ION_DEVICE_ID_WP_MOISTURE: c_uint = 0x303	// Watchport/W;
pub const ION_DEVICE_ID_WP_TEMPERATURE: c_uint = 0x304	// Watchport/T;
pub const ION_DEVICE_ID_WP_HUMIDITY: c_uint = 0x305	// Watchport/H;
pub const ION_DEVICE_ID_WP_POWER: c_uint = 0x306	// Watchport;
pub const ION_DEVICE_ID_WP_LIGHT: c_uint = 0x307	// Watchport;
pub const ION_DEVICE_ID_WP_RADIATION: c_uint = 0x308	// Watchport;
pub const ION_DEVICE_ID_WP_ACCELERATION: c_uint = 0x309	// Watchport/A;
pub const ION_DEVICE_ID_WP_DISTANCE: c_uint = 0x30A	// Watchport/D Discontinued;
pub const ION_DEVICE_ID_WP_PROX_DIST: c_uint = 0x30B	// Watchport/D uses distance sensor;
// Default to /P function
pub const ION_DEVICE_ID_PLUS_PWR_HP4CD: c_uint = 0x30C	// 5052 Plus Power HubPort/4CD+ (for Dell);
pub const ION_DEVICE_ID_PLUS_PWR_HP4C: c_uint = 0x30D	// 5052 Plus Power HubPort/4C+;
pub const ION_DEVICE_ID_PLUS_PWR_PCI: c_uint = 0x30E	// 3410 Plus Power PCI Host Controller 4 port;
//
// Definitions for AXIOHM USB product IDs
//
pub const USB_VENDOR_ID_AXIOHM: c_uint = 0x05D9	// Axiohm VID;
pub const AXIOHM_DEVICE_ID_MASK: c_uint = 0xffff;
pub const AXIOHM_DEVICE_ID_EPIC_A758: c_uint = 0xA758;
pub const AXIOHM_DEVICE_ID_EPIC_A794: c_uint = 0xA794;
pub const AXIOHM_DEVICE_ID_EPIC_A225: c_uint = 0xA225;
//
// Definitions for NCR USB product IDs
//
pub const USB_VENDOR_ID_NCR: c_uint = 0x0404	// NCR VID;
pub const NCR_DEVICE_ID_MASK: c_uint = 0xffff;
pub const NCR_DEVICE_ID_EPIC_0202: c_uint = 0x0202;
pub const NCR_DEVICE_ID_EPIC_0203: c_uint = 0x0203;
pub const NCR_DEVICE_ID_EPIC_0310: c_uint = 0x0310;
pub const NCR_DEVICE_ID_EPIC_0311: c_uint = 0x0311;
pub const NCR_DEVICE_ID_EPIC_0312: c_uint = 0x0312;
//
// Definitions for SYMBOL USB product IDs
//
pub const USB_VENDOR_ID_SYMBOL: c_uint = 0x05E0	// Symbol VID;
pub const SYMBOL_DEVICE_ID_MASK: c_uint = 0xffff;
pub const SYMBOL_DEVICE_ID_KEYFOB: c_uint = 0x0700;
//
// Definitions for other product IDs
pub const ION_DEVICE_ID_BLACKBOX_IC135A: c_uint = 0x0801	// OEM device (rebranded Edgeport/4);
pub const ION_DEVICE_ID_MT4X56USB: c_uint = 0x1403	// OEM device;
pub const ION_DEVICE_ID_E5805A: c_uint = 0x1A01  // OEM device (rebranded Edgeport/4);

//
// Definitions of parameters for download code. Note that these are
// specific to a given version of download code and must change if the
// corresponding download code changes.
//
// TxCredits value below which driver won't bother sending (to prevent too many small writes).
// Send only if above 25%

// Note that many units were shipped with MPS=16, we
// force an upgrade to this value).

//
// Definitions of I/O Networks vendor-specific requests
// for default endpoint
//
// bmRequestType = 01000000	Set vendor-specific, to device
// bmRequestType = 11000000	Get vendor-specific, to device
//
// These are the definitions for the bRequest field for the
// above bmRequestTypes.
//
// For the read/write Edgeport memory commands, the parameters
// are as follows:
// wValue = 16-bit address
// wIndex = unused (though we could put segment 00: or FF: here)
// wLength = # bytes to read/write (max 64)
//

// unused				2	// Unused, available

// code by jumping to address in wIndex:wValue
// 8	// Unused, available

// (wValue != 0: Enable; wValue = 0: Disable)

pub const USB_REQUEST_ION_DIS_INT_TIMER: c_uint = 0x80	// Sent to Axiohm to enable/ disable;
// interrupt token timer
// wValue = 1, enable (default)
// wValue = 0, disable
//
// Define parameter values for our vendor-specific commands
//
// Edgeport Compatibility Descriptor
//
// This descriptor is only returned by Edgeport-compatible devices
// supporting the EPiC spec. True ION devices do not return this
// descriptor, but instead return STALL on receipt of the
// GET_EPIC_DESC command. The driver interprets a STALL to mean that
// this is a "real" Edgeport.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edge_compatibility_bits {
// This __u32 defines which Vendor-specific commands/functionality
// the device supports on the default EP0 pipe.
    pub ION_ENABLE_SUSPEND: __u32 VendEnableSuspend : 1; // 0001 Set if device supports,
    pub 0: __u32 VendUnused : 31; // Available for future expansion, must be,
// This __u32 defines which IOSP commands are supported over the
// bulk pipe EP1.
// xxxx Set if device supports:
    pub 1): __u32 IOSPOpen : 1; // 0001 OPEN / OPEN_RSP (Currently must be,
    pub CLOSE: __u32 IOSPClose : 1; // 0002,
    pub CHASE_RSP: __u32 IOSPChase : 1; // 0004 CHASE /,
    pub SET_RX_FLOW: __u32 IOSPSetRxFlow : 1; // 0008,
    pub SET_TX_FLOW: __u32 IOSPSetTxFlow : 1; // 0010,
    pub SET_XON_CHAR/SET_XOFF_CHAR: __u32 IOSPSetXChar : 1; // 0020,
    pub RX_CHECK_REQ/RX_CHECK_RSP: __u32 IOSPRxCheck : 1; // 0040,
    pub SET_BREAK/CLEAR_BREAK: __u32 IOSPSetClrBreak : 1; // 0080,
    pub DTR/RTS): __u32 IOSPWriteMCR : 1; // 0100 MCR register writes (set/clr,
    pub (wordlen/stop/parity): __u32 IOSPWriteLCR : 1; // 0200 LCR register writes,
    pub register): __u32 IOSPSetBaudRate : 1; // 0400 setting Baud rate (writes to LCR.80h and DLL/DLM,
    pub RxButesAvailable: __u32 IOSPDisableIntPipe : 1; // 0800 Do not use the interrupt pipe for TxCredits or,
    pub Fifo): __u32 IOSPRxDataAvail : 1; // 1000 Return status of RX Fifo (Data available in,
    pub hardware: __u32 IOSPTxPurge : 1; // 2000 Purge TXBuffer and/or Fifo in Edgeport,
    pub 0: __u32 IOSPUnused : 18; // Available for future expansion, must be,
// This __u32 defines which 'general' features are supported
    pub Edgeport: __u32 TrueEdgeport : 1; // 0001 Set if device is a 'real',
// (Used only by driver, NEVER set by an EPiC device)
    pub 0: __u32 GenUnused : 31; // Available for future expansion, must be,
}

pub const EDGE_COMPATIBILITY_MASK0: c_uint = 0x0001;
pub const EDGE_COMPATIBILITY_MASK1: c_uint = 0x3FFF;
pub const EDGE_COMPATIBILITY_MASK2: c_uint = 0x0001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edge_compatibility_descriptor {
    pub spec): __u8 Length; // Descriptor Length (per USB,
    pub type): __u8 DescType; // Descriptor Type (per USB spec, =DEVICE,
    pub supported: __u8 EpicVer; // Version of EPiC spec,
// (Currently must be 1)
    pub supported: __u8 NumPorts; // Number of serial ports,
    pub filename: __u8 iDownloadFile; // Index of string containing download code,
// 0=no download, FF=download compiled into driver.
    pub 0: __u8 Unused[3]; // Available for future expansion, must be,
// (Currently must be 0).
    pub xx.: __u8 MajorVersion; // Firmware version:,
    pub yy.: __u8 MinorVersion; //,
    pub format): __le16 BuildNumber; // zzzz (LE,
// The following structure contains __u32s, with each bit
// specifying whether the EPiC device supports the given
// command or functionality.
    pub Supports: edge_compatibility_bits,
}

// Values for iDownloadFile

pub const EDGE_DOWNLOAD_FILE_INTERNAL: c_uint = 0xFF	// Download the file compiled into driver (930 version);
pub const EDGE_DOWNLOAD_FILE_I930: c_uint = 0xFF	// Download the file compiled into driver (930 version);
pub const EDGE_DOWNLOAD_FILE_80251: c_uint = 0xFE	// Download the file compiled into driver (80251 version);
//
// Special addresses for READ/WRITE_RAM/ROM
//
// Version 1 (original) format of DeviceParams
pub const EDGE_MANUF_DESC_ADDR_V1: c_uint = 0x00FF7F00;

// Version 2 format of DeviceParams. This format is longer (3C0h)
// and starts lower in memory, at the uppermost 1K in ROM.
pub const EDGE_MANUF_DESC_ADDR: c_uint = 0x00FF7C00;

// Boot params descriptor
pub const EDGE_BOOT_DESC_ADDR: c_uint = 0x00FF7FC0;

// Define the max block size that may be read or written
// in a read/write RAM/ROM command.

//
// Notes for the following two ION vendor-specific param descriptors:
//
// 1.	These have a standard USB descriptor header so they look like a
// normal descriptor.
// 2.	Any strings in the structures are in USB-defined string
// descriptor format, so that they may be separately retrieved,
// if necessary, with a minimum of work on the 930. This also
// requires them to be in UNICODE format, which, for English at
// least, simply means extending each __u8 into a __u16.
// 3.	For all fields, 00 means 'uninitialized'.
// 4.	All unused areas should be set to 00 for future expansion.
//
// This structure is ver 2 format. It contains ALL USB descriptors as
// well as the configuration parameters that were in the original V1
// structure. It is NOT modified when new boot code is downloaded; rather,
// these values are set or modified by manufacturing. It is located at
// xC00-xFBF (length 3C0h) in the ROM.
// This structure is a superset of the v1 structure and is arranged so
// that all of the v1 fields remain at the same address. We are just
// adding more room to the front of the structure to hold the descriptors.
//
// The actual contents of this structure are defined in a 930 assembly
// file, converted to a binary image, and then written by the serialization
// program. The C definition of this structure just defines a dummy
// area for general USB descriptors and the descriptor tables (the root
// descriptor starts at xC00). At the bottom of the structure are the
// fields inherited from the v1 structure.
pub const MAX_SERIALNUMBER_LEN: c_int = 12;
pub const MAX_ASSEMBLYNUMBER_LEN: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edge_manuf_descriptor {
    pub placeholder): __u16 RootDescTable[0x10]; // C00 Root of descriptor tables (just a,
    pub placeholder): __u8 DescriptorArea[0x2E0]; // C20 Descriptors go here, up to 2E0h (just a,
// Start of v1-compatible section
    pub ): __u8 Length; // F00 Desc length for what follows, per USB (= C0h,
    pub type): __u8 DescType; // F01 Desc type, per USB (=DEVICE,
    pub 2): __u8 DescVer; // F02 Desc version/format (currently,
    pub RootDescTable: __u8 NumRootDescEntries; // F03 # entries in,
    pub K: __u8 RomSize; // F04 Size of ROM/E2PROM in,
    pub K: __u8 RamSize; // F05 Size of external RAM in,
    pub visible): __u8 CpuRev; // F06 CPU revision level (chg only if s/w,
    pub visible): __u8 BoardRev; // F07 PCB revision level (chg only if s/w,
    pub ports: __u8 NumPorts; // F08 Number of,
    pub compiler,: __u8 DescDate[3]; // F09 MM/DD/YY when descriptor template was,
// so host can track changes to USB-only descriptors.
    pub len: __u8 SerNumLength; // F0C USB string descriptor,
    pub type): __u8 SerNumDescType; // F0D USB descriptor type (=STRING,
    pub Number: __le16 SerialNumber[MAX_SERIALNUMBER_LEN]; // F0E "01-01-000100" Unicode Serial,
    pub len: __u8 AssemblyNumLength; // F26 USB string descriptor,
    pub type): __u8 AssemblyNumDescType; // F27 USB descriptor type (=STRING,
    pub number: __le16 AssemblyNumber[MAX_ASSEMBLYNUMBER_LEN]; // F28 "350-1000-01-A " assembly,
    pub len: __u8 OemAssyNumLength; // F44 USB string descriptor,
    pub type): __u8 OemAssyNumDescType; // F45 USB descriptor type (=STRING,
    pub number: __le16 OemAssyNumber[MAX_ASSEMBLYNUMBER_LEN]; // F46 "xxxxxxxxxxxxxx" OEM assembly,
    pub len: __u8 ManufDateLength; // F62 USB string descriptor,
    pub type): __u8 ManufDateDescType; // F63 USB descriptor type (=STRING,
    pub date: __le16 ManufDate[6]; // F64 "MMDDYY" manufacturing,
    pub --: __u8 Reserved3[0x4D]; // F70 -- unused, set to 0,
    pub Type: __u8 UartType; // FBD Uart,
    pub DevDesc.PID: __u8 IonPid; // FBE Product ID, == LSB of USB,
// (Note: Edgeport/4s before 11/98 will have
// 00 here instead of 01)
    pub use: __u8 IonConfig; // FBF Config byte for ION manufacturing,
// FBF end of structure, total len = 3C0h
}

// Uart Types
// Note: Since this field was added only recently, all Edgeport/4 units
// shipped before 11/98 will have 00 in this field. Therefore,
// both 00 and 01 values mean '654.

//
// Note: The CpuRev and BoardRev values do not conform to manufacturing
// revisions; they are to be incremented only when the CPU or hardware
// changes in a software-visible way, such that the 930 software or
// the host driver needs to handle the hardware differently.
//
// Values of bottom 5 bits of CpuRev & BoardRev for
// Implementation 0 (ie, 930-based)

pub const MANUF_CPU_80251: c_uint = 0x20	// Intel 80251;

pub const MANUF_BOARD_REV_GENERATION_2: c_uint = 0x20	// Second generaiton edgeport;
// Values of bottom 5 bits of CpuRev & BoardRev for
// Implementation 1 (ie, 251+Netchip-based)

pub const MANUF_ION_CONFIG_DIAG_NO_LOOP: c_uint = 0x20	// As below but no ext loopback test;
pub const MANUF_ION_CONFIG_DIAG: c_uint = 0x40	// 930 based device: 1=Run h/w diags, 0=norm;
// TIUMP Device    : 1=IONSERIAL needs to run Final Test
pub const MANUF_ION_CONFIG_MASTER: c_uint = 0x80	// 930 based device:  1=Master mode, 0=Normal;
// TIUMP Device    :  1=First device on a multi TIUMP Device
//
// This structure describes parameters for the boot code, and
// is programmed along with new boot code. These are values
// which are specific to a given build of the boot code. It
// is exactly 64 bytes long and is fixed at address FF:xFC0
// - FF:xFFF. Note that the 930-mandated UCONFIG bytes are
// included in this structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edge_boot_descriptor {
    pub 40h): __u8 Length; // C0 Desc length, per USB (=,
    pub type): __u8 DescType; // C1 Desc type, per USB (= DEVICE,
    pub version/format: __u8 DescVer; // C2 Desc,
    pub --: __u8 Reserved1; // C3 -- unused, set to 0,
    pub FF:(len-1): __le16 BootCodeLength; // C4 Boot code goes from FF:0000 to,
// (LE format)
    pub xx.: __u8 MajorVersion; // C6 Firmware version:,
    pub yy.: __u8 MinorVersion; // C7,
    pub format): __le16 BuildNumber; // C8 zzzz (LE,
    pub table: __u16 EnumRootDescTable; // CA Root of ROM-based descriptor,
    pub types: __u8 NumDescTypes; // CC Number of supported descriptor,
    pub Packing: __u8 Reserved4; // CD Fix Compiler,
    pub format): __le16 Capabilities; // CE-CF Capabilities flags (LE,
    pub --: __u8 Reserved2[0x28]; // D0 -- unused, set to 0,
    pub 0: __u8 UConfig0; // F8 930-defined CPU configuration byte,
    pub 1: __u8 UConfig1; // F9 930-defined CPU configuration byte,
    pub --: __u8 Reserved3[6]; // FA -- unused, set to 0,
// FF end of structure, total len = 80
}

// Capabilities flags
pub const BOOT_CAP_RESET_CMD: c_uint = 0x0001	// If set, boot correctly supports ION_RESET_DEVICE;
//
// Chip definitions in I2C
pub const UMP5152: c_uint = 0x52;
pub const UMP3410: c_uint = 0x10;
//
// TI I2C Format Definitions
//
pub const I2C_DESC_TYPE_INFO_BASIC: c_uint = 0x01;
pub const I2C_DESC_TYPE_FIRMWARE_BASIC: c_uint = 0x02;
pub const I2C_DESC_TYPE_DEVICE: c_uint = 0x03;
pub const I2C_DESC_TYPE_CONFIG: c_uint = 0x04;
pub const I2C_DESC_TYPE_STRING: c_uint = 0x05;
pub const I2C_DESC_TYPE_FIRMWARE_AUTO: c_uint = 0x07	// for 3410 download;
pub const I2C_DESC_TYPE_CONFIG_KLUDGE: c_uint = 0x14	// for 3410;
pub const I2C_DESC_TYPE_WATCHPORT_VERSION: c_uint = 0x15	// firmware version number for watchport;
pub const I2C_DESC_TYPE_WATCHPORT_CALIBRATION_DATA: c_uint = 0x16	// Watchport Calibration Data;
pub const I2C_DESC_TYPE_FIRMWARE_BLANK: c_uint = 0xf2;
// Special section defined by ION

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_i2c_desc {
    pub descriptor: __u8 Type; // Type of,
    pub header: __le16 Size; // Size of data only not including,
    pub only): __u8 CheckSum; // Checksum (8 bit sum of data,
    pub here: __u8 Data[]; // Data starts,
    pub __attribute__((packed)): },
// for 5152 devices only (type 2 record)
// for 3410 the version is stored in the WATCHPORT_FIRMWARE_VERSION descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_i2c_firmware_rec {
    pub number: __u8 Ver_Major; // Firmware Major version,
    pub number: __u8 Ver_Minor; // Firmware Minor version,
    pub here: __u8 Data[]; // Download starts,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct watchport_firmware_version {
// Added 2 bytes for version number
    pub Watchport): __u8 Version_Major; // Download Version (for,
    pub Version_Minor: __u8,
    pub __attribute__((packed)): },
// Structure of header of download image in fw_down.h
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_i2c_image_header {
    pub Length: __le16,
    pub CheckSum: __u8,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_basic_descriptor {
    pub powered: __u8 Power; // Self,
// bit 7: 1 - power switching supported
// 0 - power switching not supported
//
// bit 0: 1 - self powered
// 0 - bus powered
//
    pub HUB: __u16 HubVid; // VID,
    pub HUB: __u16 HubPid; // PID,
    pub Edgeport: __u16 DevPid; // PID,
    pub good: __u8 HubTime; // Time for power on to power,
    pub 100ma: __u8 HubCurrent; // HUB Current =,
    pub __attribute__((packed)): },
// CPU / Board Rev Definitions

pub const TI_I2C_SIZE_MASK: c_uint = 0x1f  // 5 bits;

pub const TI_MANUF_VERSION_0: c_int = 0;
// IonConig2 flags
pub const TI_CONFIG2_RS232: c_uint = 0x01;
pub const TI_CONFIG2_RS422: c_uint = 0x02;
pub const TI_CONFIG2_RS485: c_uint = 0x04;
pub const TI_CONFIG2_SWITCHABLE: c_uint = 0x08;
pub const TI_CONFIG2_WATCHPORT: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edge_ti_manuf_descriptor {
    pub use: __u8 IonConfig; // Config byte for ION manufacturing,
    pub Expansion: __u8 IonConfig2; //,
    pub Version: __u8 Version; //,
    pub (0x0F): __u8 CpuRev_BoardRev; // CPU revision level (0xF0) and Board Rev Level,
    pub UMP: __u8 NumPorts; // Number of ports for this,
    pub ports: __u8 NumVirtualPorts; // Number of Virtual,
    pub Hub: __u8 HubConfig1; // Used to configure the,
    pub Hub: __u8 HubConfig2; // Used to configure the,
    pub UMPs): __u8 TotalPorts; // Total Number of Com Ports for the entire device (All,
    pub Reserved: __u8 Reserved; //,
    pub __attribute__((packed)): },
