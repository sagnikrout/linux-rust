//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/message/fusion/mptctl.h
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


//
// linux/drivers/message/fusion/mptioctl.h
// Fusion MPT misc device (ioctl) driver.
// For use with PCI chip/adapter(s):
// LSIFC9xx/LSI409xx Fibre Channel
// running LSI Fusion MPT (Message Passing Technology) firmware.
//
// Copyright (c) 1999-2008 LSI Corporation
// (mailto:DL-MPTFusionLinux@lsi.com)
//
// =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
//

// Macro flag: #define MPTCTL_H_INCLUDED
// =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
//

pub const MPT_PRODUCT_LENGTH: c_int = 12;
//
// Generic MPT Control IOCTLs and structures
//

//
// SPARC PLATFORM REMARKS:
// IOCTL data structures that contain pointers
// will have different sizes in the driver and applications
// (as the app. will not use 8-byte pointers).
// Apps should use MPTFWDOWNLOAD and MPTCOMMAND.
// The driver will convert data from
// mpt_fw_xfer32 (mpt_ioctl_command32) to mpt_fw_xfer (mpt_ioctl_command)
// internally.
//
// If data structures change size, must handle as in IOCGETINFO.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt_fw_xfer {
    pub /: *mut *mut unsigned int iocnum; / IOC unit number,
    pub fwlen: c_uint,
    pub /: *mut *mut *mut void __user bufp; / Pointer to firmware buffer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt_fw_xfer32 {
    pub iocnum: c_uint,
    pub fwlen: c_uint,
    pub bufp: u32,
}

//
// IOCTL header structure.
// iocnum - must be defined.
// port - must be defined for all IOCTL commands other than MPTIOCINFO
// maxDataSize - ignored on MPTCOMMAND commands
// - ignored on MPTFWREPLACE commands
// - on query commands, reports the maximum number of bytes to be returned
// to the host driver (count includes the header).
// That is, set to sizeof(struct mpt_ioctl_iocinfo) for fixed sized commands.
// Set to sizeof(struct mpt_ioctl_targetinfo) + datasize for variable
// sized commands. (MPTTARGETINFO, MPTEVENTREPORT)
//
// Issue a diagnostic reset
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt_ioctl_diag_reset {
    pub hdr: mpt_ioctl_header,
}

//
// PCI bus/device/function information structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt_ioctl_pci_info {
    pub 5: unsigned int deviceNumber :,
    pub 3: unsigned int functionNumber :,
    pub 24: unsigned int busNumber :,
    pub bits: },
    pub asUlong: c_uint,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt_ioctl_pci_info2 {
    pub 5: unsigned int deviceNumber :,
    pub 3: unsigned int functionNumber :,
    pub 24: unsigned int busNumber :,
    pub bits: },
    pub asUlong: c_uint,
    pub u: },
    pub segmentID: c_int,
}

//
// Adapter Information Page
// Read only.
// Data starts at offset 0xC
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt_ioctl_iocinfo {
    pub hdr: mpt_ioctl_header,
    pub /: *mut *mut int adapterType; / SCSI or FCP,
    pub /: *mut *mut int port; / port number,
    pub /: *mut *mut int pciId; / PCI Id.,
    pub /: *mut *mut int hwRev; / hardware revision,
    pub /: *mut *mut int subSystemDevice; / PCI subsystem Device ID,
    pub /: *mut *mut int subSystemVendor; / PCI subsystem Vendor ID,
    pub /: *mut *mut int numDevices; / number of devices,
    pub /: *mut *mut int FWVersion; / FW Version (integer),
    pub /: *mut *mut int BIOSVersion; / BIOS Version (integer),
    pub /: *mut *mut char driverVersion[MPT_IOCTL_VERSION_LENGTH]; / Driver Version (string),
    pub busChangeEvent: c_char,
    pub hostId: c_char,
    pub rsvd: [c_char; 2],
    pub /: *mut *mut mpt_ioctl_pci_info2 pciInfo; / Added Rev 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt_ioctl_iocinfo_rev1 {
    pub hdr: mpt_ioctl_header,
    pub /: *mut *mut int adapterType; / SCSI or FCP,
    pub /: *mut *mut int port; / port number,
    pub /: *mut *mut int pciId; / PCI Id.,
    pub /: *mut *mut int hwRev; / hardware revision,
    pub /: *mut *mut int subSystemDevice; / PCI subsystem Device ID,
    pub /: *mut *mut int subSystemVendor; / PCI subsystem Vendor ID,
    pub /: *mut *mut int numDevices; / number of devices,
    pub /: *mut *mut int FWVersion; / FW Version (integer),
    pub /: *mut *mut int BIOSVersion; / BIOS Version (integer),
    pub /: *mut *mut char driverVersion[MPT_IOCTL_VERSION_LENGTH]; / Driver Version (string),
    pub busChangeEvent: c_char,
    pub hostId: c_char,
    pub rsvd: [c_char; 2],
    pub /: *mut *mut mpt_ioctl_pci_info pciInfo; / Added Rev 1,
}

// Original structure, must always accept these
// IOCTLs. 4 byte pads can occur based on arch with
// above structure. Wish to re-align, but cannot.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt_ioctl_iocinfo_rev0 {
    pub hdr: mpt_ioctl_header,
    pub /: *mut *mut int adapterType; / SCSI or FCP,
    pub /: *mut *mut int port; / port number,
    pub /: *mut *mut int pciId; / PCI Id.,
    pub /: *mut *mut int hwRev; / hardware revision,
    pub /: *mut *mut int subSystemDevice; / PCI subsystem Device ID,
    pub /: *mut *mut int subSystemVendor; / PCI subsystem Vendor ID,
    pub /: *mut *mut int numDevices; / number of devices,
    pub /: *mut *mut int FWVersion; / FW Version (integer),
    pub /: *mut *mut int BIOSVersion; / BIOS Version (integer),
    pub /: *mut *mut char driverVersion[MPT_IOCTL_VERSION_LENGTH]; / Driver Version (string),
    pub busChangeEvent: c_char,
    pub hostId: c_char,
    pub rsvd: [c_char; 2],
}

//
// Device Information Page
// Report the number of, and ids of, all targets
// on this IOC.  The ids array is a packed structure
// of the known targetInfo.
// bits 31-24: reserved
// 23-16: LUN
// 15- 8: Bus Number
// 7- 0: Target ID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt_ioctl_targetinfo {
    pub hdr: mpt_ioctl_header,
    pub /: *mut *mut int numDevices; / Num targets on this ioc,
    pub targetInfo: [c_int; 1],
}

//
// Event reporting IOCTL's.  These IOCTL's will
// use the following defines:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt_ioctl_eventquery {
    pub hdr: mpt_ioctl_header,
    pub eventEntries: c_ushort,
    pub reserved: c_ushort,
    pub eventTypes: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt_ioctl_eventenable {
    pub hdr: mpt_ioctl_header,
    pub eventTypes: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt_ioctl_eventreport {
    pub hdr: mpt_ioctl_header,
    pub eventData: [MPT_IOCTL_EVENTS; 1],
}

pub const MPT_MAX_NAME: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt_ioctl_test {
    pub hdr: mpt_ioctl_header,
    pub name: [u8; MPT_MAX_NAME],
    pub chip_type: c_int,
    pub [MPT_PRODUCT_LENGTH]: u8 product,
}

// Replace the FW image cached in host driver memory
// newImageSize - image size in bytes
// newImage - first byte of the new image
//
// General MPT Pass through data strucutre
//
// iocnum
// timeout - in seconds, command timeout. If 0, set by driver to
// default value.
// replyFrameBufPtr - reply location
// dataInBufPtr - destination for read
// dataOutBufPtr - data source for write
// senseDataPtr - sense data location
// maxReplyBytes - maximum number of reply bytes to be sent to app.
// dataInSize - num bytes for data transfer in (read)
// dataOutSize - num bytes for data transfer out (write)
// dataSgeOffset - offset in words from the start of the request message
// to the first SGL
// MF[1];
//
// Remark:  Some config pages have bi-directional transfer,
// both a read and a write. The basic structure allows for
// a bidirectional set up. Normal messages will have one or
// both of these buffers NULL.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt_ioctl_command {
    pub hdr: mpt_ioctl_header,
    pub /: *mut *mut int timeout; / optional (seconds),
    pub replyFrameBufPtr: *mut char __user,
    pub dataInBufPtr: *mut char __user,
    pub dataOutBufPtr: *mut char __user,
    pub senseDataPtr: *mut char __user,
    pub maxReplyBytes: c_int,
    pub dataInSize: c_int,
    pub dataOutSize: c_int,
    pub maxSenseBytes: c_int,
    pub dataSgeOffset: c_int,
    pub MF: [c_char; 1],
}

//
// SPARC PLATFORM: See earlier remark.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt_ioctl_command32 {
    pub hdr: mpt_ioctl_header,
    pub timeout: c_int,
    pub replyFrameBufPtr: u32,
    pub dataInBufPtr: u32,
    pub dataOutBufPtr: u32,
    pub senseDataPtr: u32,
    pub maxReplyBytes: c_int,
    pub dataInSize: c_int,
    pub dataOutSize: c_int,
    pub maxSenseBytes: c_int,
    pub dataSgeOffset: c_int,
    pub MF: [c_char; 1],
}

// =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=

//
// Header:
// iocnum 	required (input)
// host 	ignored
// channe	ignored
// id		ignored
// lun		ignored
//
// replace ulongs with uints, need to preserve backwards
// compatibility.
//
// Header:
// iocnum 	required (input)
// host 	required
// channel	required	(bus number)
// id		required
// lun		ignored
//
// All error values between 0 and 0xFFFF in size.
//
pub const HP_STATUS_OTHER: c_int = 1;
pub const HP_STATUS_OK: c_int = 2;
pub const HP_STATUS_FAILED: c_int = 3;
pub const HP_BUS_WIDTH_UNK: c_int = 1;
pub const HP_BUS_WIDTH_8: c_int = 2;
pub const HP_BUS_WIDTH_16: c_int = 3;
pub const HP_BUS_WIDTH_32: c_int = 4;
pub const HP_DEV_SPEED_ASYNC: c_int = 2;
pub const HP_DEV_SPEED_FAST: c_int = 3;
pub const HP_DEV_SPEED_ULTRA: c_int = 4;
pub const HP_DEV_SPEED_ULTRA2: c_int = 5;
pub const HP_DEV_SPEED_ULTRA160: c_int = 6;
pub const HP_DEV_SPEED_SCSI1: c_int = 7;
pub const HP_DEV_SPEED_ULTRA320: c_int = 8;
// =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
