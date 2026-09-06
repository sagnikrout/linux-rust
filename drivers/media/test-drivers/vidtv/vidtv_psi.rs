//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/test-drivers/vidtv/vidtv_psi.h
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
// This file contains the logic to work with MPEG Program-Specific Information.
// These are defined both in ISO/IEC 13818-1 (systems) and ETSI EN 300 468.
// PSI is carried in the form of table structures, and although each table might
// technically be broken into one or more sections, we do not do this here,
// hence 'table' and 'section' are interchangeable for vidtv.
//
// Copyright (C) 2020 Daniel W. S. Almeida
//

//
// all section lengths start immediately after the 'section_length' field
// see ISO/IEC 13818-1 : 2000 and ETSI EN 300 468 V 1.10.1 for
// reference
//
pub const PAT_LEN_UNTIL_LAST_SECTION_NUMBER: c_int = 5;
pub const PMT_LEN_UNTIL_PROGRAM_INFO_LENGTH: c_int = 9;
pub const SDT_LEN_UNTIL_RESERVED_FOR_FUTURE_USE: c_int = 8;
pub const NIT_LEN_UNTIL_NETWORK_DESCRIPTOR_LEN: c_int = 7;
pub const EIT_LEN_UNTIL_LAST_TABLE_ID: c_int = 11;
pub const MAX_SECTION_LEN: c_int = 1021;

pub const VIDTV_SDT_PID: c_uint = 0x0011 /* mandated by the specs */;
pub const VIDTV_NIT_PID: c_uint = 0x0010 /* mandated by the specs */;
pub const VIDTV_EIT_PID: c_uint = 0x0012 /*mandated by the specs */;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vidtv_psi_descriptors {
    REGISTRATION_DESCRIPTOR	= 0x05, /* See ISO/IEC 13818-1 section 2.6.8 */
    NETWORK_NAME_DESCRIPTOR = 0x40, /* See ETSI EN 300 468 section 6.2.27 */
    SERVICE_LIST_DESCRIPTOR = 0x41, /* See ETSI EN 300 468 section 6.2.35 */
    SERVICE_DESCRIPTOR = 0x48, /* See ETSI EN 300 468 section 6.2.33 */
    SHORT_EVENT_DESCRIPTOR = 0x4d, /* See ETSI EN 300 468 section 6.2.37 */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vidtv_psi_stream_types {
    STREAM_PRIVATE_DATA = 0x06, /* see ISO/IEC 13818-1 2000 p. 48 */
}

//
// struct vidtv_psi_desc - A generic PSI descriptor type.
// The descriptor length is an 8-bit field specifying the total number of bytes of the data portion
// of the descriptor following the byte defining the value of this field.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_psi_desc {
    pub next: *mut vidtv_psi_desc,
    pub type: u8,
    pub length: u8,
    pub data: [u8; ],
    pub __packed: },
//
// struct vidtv_psi_desc_service - Service descriptor.
// See ETSI EN 300 468 section 6.2.33.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_psi_desc_service {
    pub next: *mut vidtv_psi_desc,
    pub type: u8,
    pub length: u8,
    pub service_type: u8,
    pub provider_name_len: u8,
    pub provider_name: *mut c_char,
    pub service_name_len: u8,
    pub service_name: *mut c_char,
    pub __packed: },
//
// struct vidtv_psi_desc_registration - A registration descriptor.
// See ISO/IEC 13818-1 section 2.6.8
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_psi_desc_registration {
    pub next: *mut vidtv_psi_desc,
    pub type: u8,
    pub length: u8,
//
// The format_identifier is a 32-bit value obtained from a Registration
// Authority as designated by ISO/IEC JTC 1/SC 29.
//
    pub format_id: __be32,
//
// The meaning of additional_identification_info bytes, if any, are
// defined by the assignee of that format_identifier, and once defined
// they shall not change.
//
    pub additional_identification_info: [u8; ],
    pub __packed: },
//
// struct vidtv_psi_desc_network_name - A network name descriptor
// see ETSI EN 300 468 v1.15.1 section 6.2.27
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_psi_desc_network_name {
    pub next: *mut vidtv_psi_desc,
    pub type: u8,
    pub length: u8,
    pub network_name: *mut c_char,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_psi_desc_service_list_entry {
    pub service_id: __be16,
    pub service_type: u8,
    pub next: *mut vidtv_psi_desc_service_list_entry,
    pub __packed: },
//
// struct vidtv_psi_desc_service_list - A service list descriptor
// see ETSI EN 300 468 v1.15.1 section 6.2.35
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_psi_desc_service_list {
    pub next: *mut vidtv_psi_desc,
    pub type: u8,
    pub length: u8,
    pub service_list: *mut vidtv_psi_desc_service_list_entry,
    pub __packed: },
//
// struct vidtv_psi_desc_short_event - A short event descriptor
// see ETSI EN 300 468 v1.15.1 section 6.2.37
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_psi_desc_short_event {
    pub next: *mut vidtv_psi_desc,
    pub type: u8,
    pub length: u8,
    pub iso_language_code: *mut c_char,
    pub event_name_len: u8,
    pub event_name: *mut c_char,
    pub text_len: u8,
    pub text: *mut c_char,
    pub __packed: },
// vidtv_psi_short_event_desc_init(struct vidtv_psi_desc *head,
    pub text): *mut c_char,
//
// struct vidtv_psi_table_header - A header that is present for all PSI tables.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_psi_table_header {
    pub table_id: u8,
    pub /: *mut *mut __be16 bitfield; / syntax: 1, zero: 1, one: 2, section_length: 13,
    pub /: *mut *mut __be16 id; / TS ID,
    pub current_next:1: u8,
    pub version:5: u8,
    pub one2:2: u8,
    pub /: *mut *mut u8 section_id; / section_number,
    pub /: *mut *mut u8 last_section; / last_section_number,
    pub __packed: },
//
// struct vidtv_psi_table_pat_program - A single program in the PAT
// See ISO/IEC 13818-1 : 2000 p.43
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_psi_table_pat_program {
    pub service_id: __be16,
    pub /: *mut *mut __be16 bitfield; / reserved: 3, program_map_pid/network_pid: 13,
    pub next: *mut vidtv_psi_table_pat_program,
    pub __packed: },
//
// struct vidtv_psi_table_pat - The Program Allocation Table (PAT)
// See ISO/IEC 13818-1 : 2000 p.43
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_psi_table_pat {
    pub header: vidtv_psi_table_header,
    pub num_pat: u16,
    pub num_pmt: u16,
    pub program: *mut vidtv_psi_table_pat_program,
    pub __packed: },
//
// struct vidtv_psi_table_sdt_service - Represents a service in the SDT.
// see ETSI EN 300 468 v1.15.1 section 5.2.3.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_psi_table_sdt_service {
    pub service_id: __be16,
    pub EIT_present_following:1: u8,
    pub EIT_schedule:1: u8,
    pub reserved:6: u8,
    pub /: *mut *mut __be16 bitfield; / running_status: 3, free_ca:1, desc_loop_len:12,
    pub descriptor: *mut vidtv_psi_desc,
    pub next: *mut vidtv_psi_table_sdt_service,
    pub __packed: },
//
// struct vidtv_psi_table_sdt - Represents the Service Description Table
// see ETSI EN 300 468 v1.15.1 section 5.2.3.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_psi_table_sdt {
    pub header: vidtv_psi_table_header,
    pub /: *mut *mut __be16 network_id; / original_network_id,
    pub reserved: u8,
    pub service: *mut vidtv_psi_table_sdt_service,
    pub __packed: },
//
// enum service_running_status - Status of a SDT service.
// see ETSI EN 300 468 v1.15.1 section 5.2.3 table 6.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum service_running_status {
    RUNNING = 0x4,
}

//
// enum service_type - The type of a SDT service.
// see ETSI EN 300 468 v1.15.1 section 6.2.33, table 81.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum service_type {
// see ETSI EN 300 468 v1.15.1 p. 77
    DIGITAL_TELEVISION_SERVICE = 0x1,
    DIGITAL_RADIO_SOUND_SERVICE = 0X2,
}

//
// struct vidtv_psi_table_pmt_stream - A single stream in the PMT.
// See ISO/IEC 13818-1 : 2000 p.46.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_psi_table_pmt_stream {
    pub type: u8,
    pub /: *mut *mut __be16 bitfield; / reserved: 3, elementary_pid: 13,
    pub /: *mut *mut __be16 bitfield2; /reserved: 4, zero: 2, desc_length: 10,
    pub descriptor: *mut vidtv_psi_desc,
    pub next: *mut vidtv_psi_table_pmt_stream,
    pub __packed: },
//
// struct vidtv_psi_table_pmt - The Program Map Table (PMT).
// See ISO/IEC 13818-1 : 2000 p.46.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_psi_table_pmt {
    pub header: vidtv_psi_table_header,
    pub /: *mut *mut __be16 bitfield; / reserved:3, pcr_pid: 13,
    pub /: *mut *mut __be16 bitfield2; / reserved: 4, zero: 2, desc_len: 10,
    pub descriptor: *mut vidtv_psi_desc,
    pub stream: *mut vidtv_psi_table_pmt_stream,
    pub __packed: },
//
// struct psi_write_args - Arguments for the PSI packetizer.
// @dest_buf: The buffer to write into.
// @from: PSI data to be copied.
// @len: How much to write.
// @dest_offset: where to start writing in the dest_buffer.
// @pid: TS packet ID.
// @new_psi_section: Set when starting a table section.
// @continuity_counter: Incremented on every new packet.
// @is_crc: Set when writing the CRC at the end.
// @dest_buf_sz: The size of the dest_buffer
// @crc: a pointer to store the crc for this chunk
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psi_write_args {
    pub dest_buf: *mut c_void,
    pub from: *mut c_void,
    pub len: usize,
    pub dest_offset: u32,
    pub pid: u16,
    pub new_psi_section: bool,
    pub continuity_counter: *mut u8,
    pub is_crc: bool,
    pub dest_buf_sz: u32,
    pub crc: *mut u32,
}

//
// struct desc_write_args - Arguments in order to write a descriptor.
// @dest_buf: The buffer to write into.
// @dest_offset: where to start writing in the dest_buffer.
// @desc: A pointer to the descriptor
// @pid: TS packet ID.
// @continuity_counter: Incremented on every new packet.
// @dest_buf_sz: The size of the dest_buffer
// @crc: a pointer to store the crc for this chunk
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct desc_write_args {
    pub dest_buf: *mut c_void,
    pub dest_offset: u32,
    pub desc: *mut vidtv_psi_desc,
    pub pid: u16,
    pub continuity_counter: *mut u8,
    pub dest_buf_sz: u32,
    pub crc: *mut u32,
}

//
// struct crc32_write_args - Arguments in order to write the CRC at the end of
// the PSI tables.
// @dest_buf: The buffer to write into.
// @dest_offset: where to start writing in the dest_buffer.
// @crc: the CRC value to write
// @pid: TS packet ID.
// @continuity_counter: Incremented on every new packet.
// @dest_buf_sz: The size of the dest_buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crc32_write_args {
    pub dest_buf: *mut c_void,
    pub dest_offset: u32,
    pub crc: __be32,
    pub pid: u16,
    pub continuity_counter: *mut u8,
    pub dest_buf_sz: u32,
}

//
// struct header_write_args - Arguments in order to write the common table
// header
// @dest_buf: The buffer to write into.
// @dest_offset: where to start writing in the dest_buffer.
// @h: a pointer to the header.
// @pid: TS packet ID.
// @continuity_counter: Incremented on every new packet.
// @dest_buf_sz: The size of the dest_buffer
// @crc: a pointer to store the crc for this chunk
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct header_write_args {
    pub dest_buf: *mut c_void,
    pub dest_offset: u32,
    pub h: *mut vidtv_psi_table_header,
    pub pid: u16,
    pub continuity_counter: *mut u8,
    pub dest_buf_sz: u32,
    pub crc: *mut u32,
}

// vidtv_psi_registration_desc_init(struct vidtv_psi_desc *head,
// vidtv_psi_network_name_desc_init(struct vidtv_psi_desc *head, char *network_name);
// vidtv_psi_service_list_desc_init(struct vidtv_psi_desc *head,
// vidtv_psi_pat_program_init(struct vidtv_psi_table_pat_program *head,
//
// vidtv_psi_sdt_service_assign - Assigns the service loop to the SDT.
// @sdt: The SDT to assign to.
// @service: The service loop (one or more services)
//
// This will free the previous service loop in the table.
// This will assign ownership of the service loop to the table, i.e. the table
// will free this service loop when a call to its destroy function is made.
//
// vidtv_psi_desc_assign - Assigns a descriptor loop at some point
// @to: Where to assign this descriptor loop to
// @desc: The descriptor loop that will be assigned.
//
// This will free the loop in 'to', if any.
//
// vidtv_pmt_desc_assign - Assigns a descriptor loop at some point in a PMT section.
// @pmt: The PMT section that will contain the descriptor loop
// @to: Where in the PMT to assign this descriptor loop to
// @desc: The descriptor loop that will be assigned.
//
// This will free the loop in 'to', if any.
// This will assign ownership of the loop to the table, i.e. the table
// will free this loop when a call to its destroy function is made.
//
// vidtv_sdt_desc_assign - Assigns a descriptor loop at some point in a SDT.
// @sdt: The SDT that will contain the descriptor loop
// @to: Where in the PMT to assign this descriptor loop to
// @desc: The descriptor loop that will be assigned.
//
// This will free the loop in 'to', if any.
// This will assign ownership of the loop to the table, i.e. the table
// will free this loop when a call to its destroy function is made.
//
// vidtv_psi_pat_program_assign - Assigns the program loop to the PAT.
// @pat: The PAT to assign to.
// @p: The program loop (one or more programs)
//
// This will free the previous program loop in the table.
// This will assign ownership of the program loop to the table, i.e. the table
// will free this program loop when a call to its destroy function is made.
//
// vidtv_psi_pmt_stream_assign - Assigns the stream loop to the PAT.
// @pmt: The PMT to assign to.
// @s: The stream loop (one or more streams)
//
// This will free the previous stream loop in the table.
// This will assign ownership of the stream loop to the table, i.e. the table
// will free this stream loop when a call to its destroy function is made.
//
// vidtv_psi_pmt_create_sec_for_each_pat_entry - Create a PMT section for each
// program found in the PAT
// @pat: The PAT to look for programs.
// @pcr_pid: packet ID for the PCR to be used for the program described in this
// PMT section
//
// vidtv_psi_pmt_get_pid - Get the TS PID for a PMT section.
// @section: The PMT section whose PID we want to retrieve.
// @pat: The PAT table to look into.
//
// Returns: the TS PID for 'section'
//
// vidtv_psi_pat_table_update_sec_len - Recompute and update the PAT section length.
// @pat: The PAT whose length is to be updated.
//
// This will traverse the table and accumulate the length of its components,
// which is then used to replace the 'section_length' field.
//
// If section_length > MAX_SECTION_LEN, the operation fails.
//
extern "C" {
    pub fn vidtv_psi_pat_table_update_sec_len(pat: *mut vidtv_psi_table_pat);
}
//
// vidtv_psi_pmt_table_update_sec_len - Recompute and update the PMT section length.
// @pmt: The PMT whose length is to be updated.
//
// This will traverse the table and accumulate the length of its components,
// which is then used to replace the 'section_length' field.
//
// If section_length > MAX_SECTION_LEN, the operation fails.
//
extern "C" {
    pub fn vidtv_psi_pmt_table_update_sec_len(pmt: *mut vidtv_psi_table_pmt);
}
//
// vidtv_psi_sdt_table_update_sec_len - Recompute and update the SDT section length.
// @sdt: The SDT whose length is to be updated.
//
// This will traverse the table and accumulate the length of its components,
// which is then used to replace the 'section_length' field.
//
// If section_length > MAX_SECTION_LEN, the operation fails.
//
extern "C" {
    pub fn vidtv_psi_sdt_table_update_sec_len(sdt: *mut vidtv_psi_table_sdt);
}
//
// struct vidtv_psi_pat_write_args - Arguments for writing a PAT table
// @buf: The destination buffer.
// @offset: The offset into the destination buffer.
// @pat: A pointer to the PAT.
// @buf_sz: The size of the destination buffer.
// @continuity_counter: A pointer to the CC. Incremented on every new packet.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_psi_pat_write_args {
    pub buf: *mut c_char,
    pub offset: u32,
    pub pat: *mut vidtv_psi_table_pat,
    pub buf_sz: u32,
    pub continuity_counter: *mut u8,
}

//
// vidtv_psi_pat_write_into - Write PAT as MPEG-TS packets into a buffer.
// @args: An instance of struct vidtv_psi_pat_write_args
//
// This function writes the MPEG TS packets for a PAT table into a buffer.
// Calling code will usually generate the PAT via a call to its init function
// and thus is responsible for freeing it.
//
// Return: The number of bytes written into the buffer. This is NOT
// equal to the size of the PAT, since more space is needed for TS headers during TS
// encapsulation.
//
extern "C" {
    pub fn vidtv_psi_pat_write_into(args: *mut vidtv_psi_pat_write_args) -> u32;
}
//
// struct vidtv_psi_sdt_write_args - Arguments for writing a SDT table
// @buf: The destination buffer.
// @offset: The offset into the destination buffer.
// @sdt: A pointer to the SDT.
// @buf_sz: The size of the destination buffer.
// @continuity_counter: A pointer to the CC. Incremented on every new packet.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_psi_sdt_write_args {
    pub buf: *mut c_char,
    pub offset: u32,
    pub sdt: *mut vidtv_psi_table_sdt,
    pub buf_sz: u32,
    pub continuity_counter: *mut u8,
}

//
// vidtv_psi_sdt_write_into - Write SDT as MPEG-TS packets into a buffer.
// @args: an instance of struct vidtv_psi_sdt_write_args
//
// This function writes the MPEG TS packets for a SDT table into a buffer.
// Calling code will usually generate the SDT via a call to its init function
// and thus is responsible for freeing it.
//
// Return: The number of bytes written into the buffer. This is NOT
// equal to the size of the SDT, since more space is needed for TS headers during TS
// encapsulation.
//
extern "C" {
    pub fn vidtv_psi_sdt_write_into(args: *mut vidtv_psi_sdt_write_args) -> u32;
}
//
// struct vidtv_psi_pmt_write_args - Arguments for writing a PMT section
// @buf: The destination buffer.
// @offset: The offset into the destination buffer.
// @pmt: A pointer to the PMT.
// @pid: Program ID
// @buf_sz: The size of the destination buffer.
// @continuity_counter: A pointer to the CC. Incremented on every new packet.
// @pcr_pid: The TS PID used for the PSI packets. All channels will share the
// same PCR.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_psi_pmt_write_args {
    pub buf: *mut c_char,
    pub offset: u32,
    pub pmt: *mut vidtv_psi_table_pmt,
    pub pid: u16,
    pub buf_sz: u32,
    pub continuity_counter: *mut u8,
    pub pcr_pid: u16,
}

//
// vidtv_psi_pmt_write_into - Write PMT as MPEG-TS packets into a buffer.
// @args: an instance of struct vidtv_psi_pmt_write_args
//
// This function writes the MPEG TS packets for a PMT section into a buffer.
// Calling code will usually generate the PMT section via a call to its init function
// and thus is responsible for freeing it.
//
// Return: The number of bytes written into the buffer. This is NOT
// equal to the size of the PMT section, since more space is needed for TS headers
// during TS encapsulation.
//
extern "C" {
    pub fn vidtv_psi_pmt_write_into(args: *mut vidtv_psi_pmt_write_args) -> u32;
}
//
// vidtv_psi_find_pmt_sec - Finds the PMT section for 'program_num'
// @pmt_sections: The sections to look into.
// @nsections: The number of sections.
// @program_num: The 'program_num' from PAT pointing to a PMT section.
//
// Return: A pointer to the PMT, if found, or NULL.
//
extern "C" {
    pub fn vidtv_psi_get_pat_program_pid(p: *mut vidtv_psi_table_pat_program) -> u16;
}
extern "C" {
    pub fn vidtv_psi_pmt_stream_get_elem_pid(s: *mut vidtv_psi_table_pmt_stream) -> u16;
}
//
// struct vidtv_psi_table_transport - A entry in the TS loop for the NIT and/or other tables.
// See ETSI 300 468 section 5.2.1
// @transport_id: The TS ID being described
// @network_id: The network_id that contains the TS ID
// @bitfield: Contains the descriptor loop length
// @descriptor: A descriptor loop
// @next: Pointer to the next entry
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_psi_table_transport {
    pub transport_id: __be16,
    pub network_id: __be16,
    pub /: *mut *mut __be16 bitfield; / desc_len: 12, reserved: 4,
    pub descriptor: *mut vidtv_psi_desc,
    pub next: *mut vidtv_psi_table_transport,
    pub __packed: },
//
// struct vidtv_psi_table_nit - A Network Information Table (NIT). See ETSI 300
// 468 section 5.2.1
// @header: A PSI table header
// @bitfield: Contains the network descriptor length
// @descriptor: A descriptor loop describing the network
// @bitfield2: Contains the transport stream loop length
// @transport: The transport stream loop
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_psi_table_nit {
    pub header: vidtv_psi_table_header,
    pub /: *mut *mut __be16 bitfield; / network_desc_len: 12, reserved:4,
    pub descriptor: *mut vidtv_psi_desc,
    pub /: *mut *mut __be16 bitfield2; / ts_loop_len: 12, reserved: 4,
    pub transport: *mut vidtv_psi_table_transport,
    pub __packed: },
// vidtv_psi_nit_table_init(u16 network_id,
    pub service_list): *mut vidtv_psi_desc_service_list_entry,
//
// struct vidtv_psi_nit_write_args - Arguments for writing a NIT section
// @buf: The destination buffer.
// @offset: The offset into the destination buffer.
// @nit: A pointer to the NIT
// @buf_sz: The size of the destination buffer.
// @continuity_counter: A pointer to the CC. Incremented on every new packet.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_psi_nit_write_args {
    pub buf: *mut c_char,
    pub offset: u32,
    pub nit: *mut vidtv_psi_table_nit,
    pub buf_sz: u32,
    pub continuity_counter: *mut u8,
}

//
// vidtv_psi_nit_write_into - Write NIT as MPEG-TS packets into a buffer.
// @args: an instance of struct vidtv_psi_nit_write_args
//
// This function writes the MPEG TS packets for a NIT table into a buffer.
// Calling code will usually generate the NIT via a call to its init function
// and thus is responsible for freeing it.
//
// Return: The number of bytes written into the buffer. This is NOT
// equal to the size of the NIT, since more space is needed for TS headers during TS
// encapsulation.
//
extern "C" {
    pub fn vidtv_psi_nit_write_into(args: *mut vidtv_psi_nit_write_args) -> u32;
}
extern "C" {
    pub fn vidtv_psi_nit_table_destroy(nit: *mut vidtv_psi_table_nit);
}
//
// struct vidtv_psi_desc_short_event - A short event descriptor
// see ETSI EN 300 468 v1.15.1 section 6.2.37
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_psi_table_eit_event {
    pub event_id: __be16,
    pub start_time: [u8; 5],
    pub duration: [u8; 3],
    pub /: *mut *mut __be16 bitfield; / desc_length: 12, free_CA_mode: 1, running_status: 1,
    pub descriptor: *mut vidtv_psi_desc,
    pub next: *mut vidtv_psi_table_eit_event,
    pub __packed: },
//
// struct vidtv_psi_table_eit - A Event Information Table (EIT)
// See ETSI 300 468 section 5.2.4
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_psi_table_eit {
    pub header: vidtv_psi_table_header,
    pub transport_id: __be16,
    pub network_id: __be16,
    pub last_segment: u8,
    pub last_table_id: u8,
    pub event: *mut vidtv_psi_table_eit_event,
    pub __packed: },
// vidtv_psi_eit_table_init(u16 network_id,
    pub service_id): __be16,
//
// struct vidtv_psi_eit_write_args - Arguments for writing an EIT section
// @buf: The destination buffer.
// @offset: The offset into the destination buffer.
// @eit: A pointer to the EIT
// @buf_sz: The size of the destination buffer.
// @continuity_counter: A pointer to the CC. Incremented on every new packet.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_psi_eit_write_args {
    pub buf: *mut c_char,
    pub offset: u32,
    pub eit: *mut vidtv_psi_table_eit,
    pub buf_sz: u32,
    pub continuity_counter: *mut u8,
}

//
// vidtv_psi_eit_write_into - Write EIT as MPEG-TS packets into a buffer.
// @args: an instance of struct vidtv_psi_nit_write_args
//
// This function writes the MPEG TS packets for a EIT table into a buffer.
// Calling code will usually generate the EIT via a call to its init function
// and thus is responsible for freeing it.
//
// Return: The number of bytes written into the buffer. This is NOT
// equal to the size of the EIT, since more space is needed for TS headers during TS
// encapsulation.
//
extern "C" {
    pub fn vidtv_psi_eit_write_into(args: *mut vidtv_psi_eit_write_args) -> u32;
}
extern "C" {
    pub fn vidtv_psi_eit_table_destroy(eit: *mut vidtv_psi_table_eit);
}
//
// vidtv_psi_eit_table_update_sec_len - Recompute and update the EIT section length.
// @eit: The EIT whose length is to be updated.
//
// This will traverse the table and accumulate the length of its components,
// which is then used to replace the 'section_length' field.
//
// If section_length > EIT_MAX_SECTION_LEN, the operation fails.
//
extern "C" {
    pub fn vidtv_psi_eit_table_update_sec_len(eit: *mut vidtv_psi_table_eit);
}
//
// vidtv_psi_eit_event_assign - Assigns the event loop to the EIT.
// @eit: The EIT to assign to.
// @e: The event loop
//
// This will free the previous event loop in the table.
// This will assign ownership of the stream loop to the table, i.e. the table
// will free this stream loop when a call to its destroy function is made.
//
// vidtv_psi_eit_event_init(struct vidtv_psi_table_eit_event *head, u16 event_id);
extern "C" {
    pub fn vidtv_psi_eit_event_destroy(e: *mut vidtv_psi_table_eit_event);
}
