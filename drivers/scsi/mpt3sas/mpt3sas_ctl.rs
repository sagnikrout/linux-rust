//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/mpt3sas/mpt3sas_ctl.h
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
// Management Module Support for MPT (Message Passing Technology) based
// controllers
//
// This code is based on drivers/scsi/mpt3sas/mpt3sas_ctl.h
// Copyright (C) 2012-2014  LSI Corporation
// Copyright (C) 2013-2014 Avago Technologies
// (mailto: MPT-FusionLinux.pdl@avagotech.com)
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version 2
// of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// NO WARRANTY
// THE PROGRAM IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, EITHER EXPRESS OR IMPLIED INCLUDING, WITHOUT
// LIMITATION, ANY WARRANTIES OR CONDITIONS OF TITLE, NON-INFRINGEMENT,
// MERCHANTABILITY OR FITNESS FOR A PARTICULAR PURPOSE. Each Recipient is
// solely responsible for determining the appropriateness of using and
// distributing the Program and assumes all risks associated with its
// exercise of rights under this Agreement, including but not limited to
// the risks and costs of program errors, damage to or loss of data,
// programs or equipment, and unavailability or interruption of operations.
// DISCLAIMER OF LIABILITY
// NEITHER RECIPIENT NOR ANY CONTRIBUTORS SHALL HAVE ANY LIABILITY FOR ANY
// DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING WITHOUT LIMITATION LOST PROFITS), HOWEVER CAUSED AND
// ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR
// TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OR DISTRIBUTION OF THE PROGRAM OR THE EXERCISE OF ANY RIGHTS GRANTED
// HEREUNDER, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGES
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301,
// USA.
//

// Macro flag: #define MPT3SAS_CTL_H_INCLUDED

//
// IOCTL opcodes
//

// diag buffer support

// Trace Buffer default UniqueId

// UID not found

//
// struct mpt3_ioctl_header - main header structure
// @ioc_number -  IOC unit number
// @port_number - IOC port number
// @max_data_size - maximum number bytes to transfer on read
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt3_ioctl_header {
    pub ioc_number: u32,
    pub port_number: u32,
    pub max_data_size: u32,
}

//
// struct mpt3_ioctl_diag_reset - diagnostic reset
// @hdr - generic header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt3_ioctl_diag_reset {
    pub hdr: mpt3_ioctl_header,
}

//
// struct mpt3_ioctl_pci_info - pci device info
// @device - pci device id
// @function - pci function id
// @bus - pci bus id
// @segment_id - pci segment id
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt3_ioctl_pci_info {
    pub device:5: u32,
    pub function:3: u32,
    pub bus:24: u32,
    pub bits: },
    pub word: u32,
    pub u: },
    pub segment_id: u32,
}

// Bits set for mpt3_ioctl_iocinfo.driver_cap
pub const MPT3_IOCTL_IOCINFO_DRIVER_CAP_MCTP_PASSTHRU: c_uint = 0x1;
//
// struct mpt3_ioctl_iocinfo - generic controller info
// @hdr - generic header
// @adapter_type - type of adapter (spi, fc, sas)
// @port_number - port number
// @pci_id - PCI Id
// @hw_rev - hardware revision
// @sub_system_device - PCI subsystem Device ID
// @sub_system_vendor - PCI subsystem Vendor ID
// @rsvd0 - reserved
// @firmware_version - firmware version
// @bios_version - BIOS version
// @driver_version - driver version - 32 ASCII characters
// @rsvd1 - reserved
// @scsi_id - scsi id of adapter 0
// @driver_capability - driver capabilities
// @rsvd2 - reserved
// @pci_information - pci info (2nd revision)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt3_ioctl_iocinfo {
    pub hdr: mpt3_ioctl_header,
    pub adapter_type: u32,
    pub port_number: u32,
    pub pci_id: u32,
    pub hw_rev: u32,
    pub subsystem_device: u32,
    pub subsystem_vendor: u32,
    pub rsvd0: u32,
    pub firmware_version: u32,
    pub bios_version: u32,
    pub driver_version: [u8; MPT2_IOCTL_VERSION_LENGTH],
    pub rsvd1: u8,
    pub scsi_id: u8,
    pub driver_capability: u8,
    pub rsvd2: u8,
    pub pci_information: mpt3_ioctl_pci_info,
}

// number of event log entries

//
// struct mpt3_ioctl_eventquery - query event count and type
// @hdr - generic header
// @event_entries - number of events returned by get_event_report
// @rsvd - reserved
// @event_types - type of events currently being captured
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt3_ioctl_eventquery {
    pub hdr: mpt3_ioctl_header,
    pub event_entries: u16,
    pub rsvd: u16,
    pub event_types: [u32; MPI2_EVENT_NOTIFY_EVENTMASK_WORDS],
}

//
// struct mpt3_ioctl_eventenable - enable/disable event capturing
// @hdr - generic header
// @event_types - toggle off/on type of events to be captured
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt3_ioctl_eventenable {
    pub hdr: mpt3_ioctl_header,
    pub event_types: [u32; 4],
}

//
// struct MPT3_IOCTL_EVENTS -
// @event - the event that was reported
// @context - unique value for each event assigned by driver
// @data - event data returned in fw reply message
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPT3_IOCTL_EVENTS {
    pub event: u32,
    pub context: u32,
    pub data: [u8; MPT3_EVENT_DATA_SIZE],
}

//
// struct mpt3_ioctl_eventreport - returing event log
// @hdr - generic header
// @event_data - (see struct MPT3_IOCTL_EVENTS)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt3_ioctl_eventreport {
    pub hdr: mpt3_ioctl_header,
    pub event_data: [MPT3_IOCTL_EVENTS; 1],
}

//
// struct mpt3_ioctl_command - generic mpt firmware passthru ioctl
// @hdr - generic header
// @timeout - command timeout in seconds. (if zero then use driver default
// value).
// @reply_frame_buf_ptr - reply location
// @data_in_buf_ptr - destination for read
// @data_out_buf_ptr - data source for write
// @sense_data_ptr - sense data location
// @max_reply_bytes - maximum number of reply bytes to be sent to app.
// @data_in_size - number bytes for data transfer in (read)
// @data_out_size - number bytes for data transfer out (write)
// @max_sense_bytes - maximum number of bytes for auto sense buffers
// @data_sge_offset - offset in words from the start of the request message to
// the first SGL
// @mf[1];
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt3_ioctl_command {
    pub hdr: mpt3_ioctl_header,
    pub timeout: u32,
    pub reply_frame_buf_ptr: *mut void __user,
    pub data_in_buf_ptr: *mut void __user,
    pub data_out_buf_ptr: *mut void __user,
    pub sense_data_ptr: *mut void __user,
    pub max_reply_bytes: u32,
    pub data_in_size: u32,
    pub data_out_size: u32,
    pub max_sense_bytes: u32,
    pub data_sge_offset: u32,
    pub mf: [u8; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt3_ioctl_command32 {
    pub hdr: mpt3_ioctl_header,
    pub timeout: u32,
    pub reply_frame_buf_ptr: u32,
    pub data_in_buf_ptr: u32,
    pub data_out_buf_ptr: u32,
    pub sense_data_ptr: u32,
    pub max_reply_bytes: u32,
    pub data_in_size: u32,
    pub data_out_size: u32,
    pub max_sense_bytes: u32,
    pub data_sge_offset: u32,
    pub mf: [u8; 1],
}

//
// struct mpt3_ioctl_btdh_mapping - mapping info
// @hdr - generic header
// @id - target device identification number
// @bus - SCSI bus number that the target device exists on
// @handle - device handle for the target device
// @rsvd - reserved
//
// To obtain a bus/id the application sets
// handle to valid handle, and bus/id to 0xFFFF.
//
// To obtain the device handle the application sets
// bus/id valid value, and the handle to 0xFFFF.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt3_ioctl_btdh_mapping {
    pub hdr: mpt3_ioctl_header,
    pub id: u32,
    pub bus: u32,
    pub handle: u16,
    pub rsvd: u16,
}

// application flags for mpt3_diag_register, mpt3_diag_query

// flags for mpt3_diag_read_buffer

pub const MPT3_PRODUCT_SPECIFIC_DWORDS: c_int = 23;
//
// struct mpt3_diag_register - application register with driver
// @hdr - generic header
// @reserved -
// @buffer_type - specifies either TRACE, SNAPSHOT, or EXTENDED
// @application_flags - misc flags
// @diagnostic_flags - specifies flags affecting command processing
// @product_specific - product specific information
// @requested_buffer_size - buffers size in bytes
// @unique_id - tag specified by application that is used to signal ownership
// of the buffer.
//
// This will allow the driver to setup any required buffers that will be
// needed by firmware to communicate with the driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt3_diag_register {
    pub hdr: mpt3_ioctl_header,
    pub reserved: u8,
    pub buffer_type: u8,
    pub application_flags: u16,
    pub diagnostic_flags: u32,
    pub product_specific: [u32; MPT3_PRODUCT_SPECIFIC_DWORDS],
    pub requested_buffer_size: u32,
    pub unique_id: u32,
}

//
// struct mpt3_diag_unregister - application unregister with driver
// @hdr - generic header
// @unique_id - tag uniquely identifies the buffer to be unregistered
//
// This will allow the driver to cleanup any memory allocated for diag
// messages and to free up any resources.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt3_diag_unregister {
    pub hdr: mpt3_ioctl_header,
    pub unique_id: u32,
}

//
// struct mpt3_diag_query - query relevant info associated with diag buffers
// @hdr - generic header
// @reserved -
// @buffer_type - specifies either TRACE, SNAPSHOT, or EXTENDED
// @application_flags - misc flags
// @diagnostic_flags - specifies flags affecting command processing
// @product_specific - product specific information
// @total_buffer_size - diag buffer size in bytes
// @driver_added_buffer_size - size of extra space appended to end of buffer
// @unique_id - unique id associated with this buffer.
//
// The application will send only buffer_type and unique_id.  Driver will
// inspect unique_id first, if valid, fill in all the info.  If unique_id is
// 0x00, the driver will return info specified by Buffer Type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt3_diag_query {
    pub hdr: mpt3_ioctl_header,
    pub reserved: u8,
    pub buffer_type: u8,
    pub application_flags: u16,
    pub diagnostic_flags: u32,
    pub product_specific: [u32; MPT3_PRODUCT_SPECIFIC_DWORDS],
    pub total_buffer_size: u32,
    pub driver_added_buffer_size: u32,
    pub unique_id: u32,
}

//
// struct mpt3_diag_release -  request to send Diag Release Message to firmware
// @hdr - generic header
// @unique_id - tag uniquely identifies the buffer to be released
//
// This allows ownership of the specified buffer to returned to the driver,
// allowing an application to read the buffer without fear that firmware is
// overwriting information in the buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt3_diag_release {
    pub hdr: mpt3_ioctl_header,
    pub unique_id: u32,
}

//
// struct mpt3_diag_read_buffer - request for copy of the diag buffer
// @hdr - generic header
// @status -
// @reserved -
// @flags - misc flags
// @starting_offset - starting offset within drivers buffer where to start
// reading data at into the specified application buffer
// @bytes_to_read - number of bytes to copy from the drivers buffer into the
// application buffer starting at starting_offset.
// @unique_id - unique id associated with this buffer.
// @diagnostic_data - data payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt3_diag_read_buffer {
    pub hdr: mpt3_ioctl_header,
    pub status: u8,
    pub reserved: u8,
    pub flags: u16,
    pub starting_offset: u32,
    pub bytes_to_read: u32,
    pub unique_id: u32,
    pub diagnostic_data: [u32; 1],
}

//
// struct mpt3_addnl_diag_query - diagnostic buffer release reason
// @hdr - generic header
// @unique_id - unique id associated with this buffer.
// @rel_query - release query.
// @reserved2
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt3_addnl_diag_query {
    pub hdr: mpt3_ioctl_header,
    pub unique_id: u32,
    pub rel_query: htb_rel_query,
    pub reserved2: [u32; 2],
}

//
// struct mpt3_enable_diag_sbr_reload - enable sbr reload
// @hdr - generic header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt3_enable_diag_sbr_reload {
    pub hdr: mpt3_ioctl_header,
}

//
// struct mpt3_passthru_command - generic mpt firmware passthru command
// @dev_index - device index
// @timeout - command timeout in seconds. (if zero then use driver default
// value).
// @reply_frame_buf_ptr - MPI reply location
// @data_in_buf_ptr - destination for read
// @data_out_buf_ptr - data source for write
// @max_reply_bytes - maximum number of reply bytes to be sent to app.
// @data_in_size - number bytes for data transfer in (read)
// @data_out_size - number bytes for data transfer out (write)
// @mpi_request - request frame
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt3_passthru_command {
    pub dev_index: u8,
    pub timeout: u32,
    pub reply_frame_buf_ptr: *mut c_void,
    pub data_in_buf_ptr: *mut c_void,
    pub data_out_buf_ptr: *mut c_void,
    pub max_reply_bytes: u32,
    pub data_in_size: u32,
    pub data_out_size: u32,
    pub mpi_request: *mut Mpi26MctpPassthroughRequest_t,
}

//
// mpt3sas_get_device_count - Retrieve the count of MCTP passthrough
// capable devices managed by the driver.
//
// Returns number of devices that support MCTP passthrough.
//
extern "C" {
    pub fn mpt3sas_get_device_count() -> c_int;
}
//
// mpt3sas_send_passthru_cmd - Send an MPI MCTP passthrough command to
// firmware
// @command: The MPI MCTP passthrough command to send to firmware
//
// Returns 0 on success, anything else is error .
//
extern "C" {
    pub fn mpt3sas_send_mctp_passthru_req(command: *mut mpt3_passthru_command) -> c_int;
}
