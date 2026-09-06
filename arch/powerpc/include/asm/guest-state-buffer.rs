//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/guest-state-buffer.h
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
// Interface based on include/net/netlink.h
//

//
// Guest State Buffer Constants
//
// Element without a value and any length
pub const KVMPPC_GSID_BLANK: c_uint = 0x0000;
// Size required for the L0's internal VCPU representation
pub const KVMPPC_GSID_HOST_STATE_SIZE: c_uint = 0x0001;
// Minimum size for the H_GUEST_RUN_VCPU output buffer
pub const KVMPPC_GSID_RUN_OUTPUT_MIN_SIZE: c_uint = 0x0002;
// "Logical" PVR value as defined in the PAPR
pub const KVMPPC_GSID_LOGICAL_PVR: c_uint = 0x0003;
// L0 relative timebase offset
pub const KVMPPC_GSID_TB_OFFSET: c_uint = 0x0004;
// Partition Scoped Page Table Info
pub const KVMPPC_GSID_PARTITION_TABLE: c_uint = 0x0005;
// Process Table Info
pub const KVMPPC_GSID_PROCESS_TABLE: c_uint = 0x0006;
// Guest Management Heap Size
pub const KVMPPC_GSID_L0_GUEST_HEAP: c_uint = 0x0800;
// Guest Management Heap Max Size
pub const KVMPPC_GSID_L0_GUEST_HEAP_MAX: c_uint = 0x0801;
// Guest Pagetable Size
pub const KVMPPC_GSID_L0_GUEST_PGTABLE_SIZE: c_uint = 0x0802;
// Guest Pagetable Max Size
pub const KVMPPC_GSID_L0_GUEST_PGTABLE_SIZE_MAX: c_uint = 0x0803;
// Guest Pagetable Reclaim in bytes
pub const KVMPPC_GSID_L0_GUEST_PGTABLE_RECLAIM: c_uint = 0x0804;
// H_GUEST_RUN_VCPU input buffer Info
pub const KVMPPC_GSID_RUN_INPUT: c_uint = 0x0C00;
// H_GUEST_RUN_VCPU output buffer Info
pub const KVMPPC_GSID_RUN_OUTPUT: c_uint = 0x0C01;
pub const KVMPPC_GSID_VPA: c_uint = 0x0C02;

pub const KVMPPC_GSID_HDEC_EXPIRY_TB: c_uint = 0x1020;
pub const KVMPPC_GSID_NIA: c_uint = 0x1021;
pub const KVMPPC_GSID_MSR: c_uint = 0x1022;
pub const KVMPPC_GSID_LR: c_uint = 0x1023;
pub const KVMPPC_GSID_XER: c_uint = 0x1024;
pub const KVMPPC_GSID_CTR: c_uint = 0x1025;
pub const KVMPPC_GSID_CFAR: c_uint = 0x1026;
pub const KVMPPC_GSID_SRR0: c_uint = 0x1027;
pub const KVMPPC_GSID_SRR1: c_uint = 0x1028;
pub const KVMPPC_GSID_DAR: c_uint = 0x1029;
pub const KVMPPC_GSID_DEC_EXPIRY_TB: c_uint = 0x102A;
pub const KVMPPC_GSID_VTB: c_uint = 0x102B;
pub const KVMPPC_GSID_LPCR: c_uint = 0x102C;
pub const KVMPPC_GSID_HFSCR: c_uint = 0x102D;
pub const KVMPPC_GSID_FSCR: c_uint = 0x102E;
pub const KVMPPC_GSID_FPSCR: c_uint = 0x102F;
pub const KVMPPC_GSID_DAWR0: c_uint = 0x1030;
pub const KVMPPC_GSID_DAWR1: c_uint = 0x1031;
pub const KVMPPC_GSID_CIABR: c_uint = 0x1032;
pub const KVMPPC_GSID_PURR: c_uint = 0x1033;
pub const KVMPPC_GSID_SPURR: c_uint = 0x1034;
pub const KVMPPC_GSID_IC: c_uint = 0x1035;
pub const KVMPPC_GSID_SPRG0: c_uint = 0x1036;
pub const KVMPPC_GSID_SPRG1: c_uint = 0x1037;
pub const KVMPPC_GSID_SPRG2: c_uint = 0x1038;
pub const KVMPPC_GSID_SPRG3: c_uint = 0x1039;
pub const KVMPPC_GSID_PPR: c_uint = 0x103A;

pub const KVMPPC_GSID_MMCRA: c_uint = 0x103F;

pub const KVMPPC_GSID_BESCR: c_uint = 0x1043;
pub const KVMPPC_GSID_EBBHR: c_uint = 0x1044;
pub const KVMPPC_GSID_EBBRR: c_uint = 0x1045;
pub const KVMPPC_GSID_AMR: c_uint = 0x1046;
pub const KVMPPC_GSID_IAMR: c_uint = 0x1047;
pub const KVMPPC_GSID_AMOR: c_uint = 0x1048;
pub const KVMPPC_GSID_UAMOR: c_uint = 0x1049;
pub const KVMPPC_GSID_SDAR: c_uint = 0x104A;
pub const KVMPPC_GSID_SIAR: c_uint = 0x104B;
pub const KVMPPC_GSID_DSCR: c_uint = 0x104C;
pub const KVMPPC_GSID_TAR: c_uint = 0x104D;
pub const KVMPPC_GSID_DEXCR: c_uint = 0x104E;
pub const KVMPPC_GSID_HDEXCR: c_uint = 0x104F;
pub const KVMPPC_GSID_HASHKEYR: c_uint = 0x1050;
pub const KVMPPC_GSID_HASHPKEYR: c_uint = 0x1051;
pub const KVMPPC_GSID_CTRL: c_uint = 0x1052;
pub const KVMPPC_GSID_DPDES: c_uint = 0x1053;
pub const KVMPPC_GSID_CR: c_uint = 0x2000;
pub const KVMPPC_GSID_PIDR: c_uint = 0x2001;
pub const KVMPPC_GSID_DSISR: c_uint = 0x2002;
pub const KVMPPC_GSID_VSCR: c_uint = 0x2003;
pub const KVMPPC_GSID_VRSAVE: c_uint = 0x2004;
pub const KVMPPC_GSID_DAWRX0: c_uint = 0x2005;
pub const KVMPPC_GSID_DAWRX1: c_uint = 0x2006;

pub const KVMPPC_GSID_WORT: c_uint = 0x200D;
pub const KVMPPC_GSID_PSPB: c_uint = 0x200E;

pub const KVMPPC_GSID_HDAR: c_uint = 0xF000;
pub const KVMPPC_GSID_HDSISR: c_uint = 0xF001;
pub const KVMPPC_GSID_HEIR: c_uint = 0xF002;
pub const KVMPPC_GSID_ASDR: c_uint = 0xF003;

//
// Ranges of guest state buffer elements
//
// Types of guest state buffer elements
//
// Flags for guest state elements
//
// struct kvmppc_gs_part_table - deserialized partition table information
// element
// @address: start of the partition table
// @ea_bits: number of bits in the effective address
// @gpd_size: root page directory size
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_gs_part_table {
    pub address: u64,
    pub ea_bits: u64,
    pub gpd_size: u64,
}

//
// struct kvmppc_gs_proc_table - deserialized process table information element
// @address: start of the process table
// @gpd_size: process table size
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_gs_proc_table {
    pub address: u64,
    pub gpd_size: u64,
}

//
// struct kvmppc_gs_buff_info - deserialized meta guest state buffer information
// @address: start of the guest state buffer
// @size: size of the guest state buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_gs_buff_info {
    pub address: u64,
    pub size: u64,
}

//
// struct kvmppc_gs_header - serialized guest state buffer header
// @nelem: count of guest state elements in the buffer
// @data: start of the stream of elements in the buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_gs_header {
    pub nelems: __be32,
    pub data: [c_char; ],
    pub __packed: },
//
// struct kvmppc_gs_elem - serialized guest state buffer element
// @iden: Guest State ID
// @len: length of data
// @data: the guest state buffer element's value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_gs_elem {
    pub iden: __be16,
    pub len: __be16,
    pub data: [c_char; ],
    pub __packed: },
//
// struct kvmppc_gs_buff - a guest state buffer with metadata.
// @capacity: total length of the buffer
// @len: current length of the elements and header
// @guest_id: guest id associated with the buffer
// @vcpu_id: vcpu_id associated with the buffer
// @hdr: the serialised guest state buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_gs_buff {
    pub capacity: usize,
    pub len: usize,
    pub guest_id: c_ulong,
    pub vcpu_id: c_ulong,
    pub hdr: *mut kvmppc_gs_header,
}

//
// struct kvmppc_gs_bitmap - a bitmap for element ids
// @bitmap: a bitmap large enough for all Guest State IDs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_gs_bitmap {
// private:
    pub KVMPPC_GSE_IDEN_COUNT): DECLARE_BITMAP(bitmap,,
}

//
// struct kvmppc_gs_parser - a map of element ids to locations in a buffer
// @iterator: bitmap used for iterating
// @gses: contains the pointers to elements
//
// A guest state parser is used for deserialising a guest state buffer.
// Given a buffer, it then allows looking up guest state elements using
// a guest state id.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_gs_parser {
// private:
    pub iterator: kvmppc_gs_bitmap,
    pub gses: [*mut kvmppc_gs_elem; KVMPPC_GSE_IDEN_COUNT],
}

//
// struct kvmppc_gs_msg_ops - guest state message behavior
// @get_size: maximum size required for the message data
// @fill_info: serializes to the guest state buffer format
// @refresh_info: dserializes from the guest state buffer format
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_gs_msg_ops {
    pub gsm): *mut *mut size_t (get_size)(struct kvmppc_gs_msg,
    pub gsm): *mut *mut *mut int (fill_info)(struct kvmppc_gs_buff gsb, struct kvmppc_gs_msg,
    pub gsb): *mut kvmppc_gs_buff,
}

//
// struct kvmppc_gs_msg - a guest state message
// @bitmap: the guest state ids that should be included
// @ops: modify message behavior for reading and writing to buffers
// @flags: host wide, guest wide or thread wide
// @data: location where buffer data will be written to or from.
//
// A guest state message is allows flexibility in sending in receiving data
// in a guest state buffer format.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_gs_msg {
    pub bitmap: kvmppc_gs_bitmap,
    pub ops: *mut kvmppc_gs_msg_ops,
    pub flags: c_ulong,
    pub data: *mut c_void,
}

//
// Guest State IDs
//
extern "C" {
    pub fn kvmppc_gsid_size(iden: u16) -> u16;
}
extern "C" {
    pub fn kvmppc_gsid_flags(iden: u16) -> c_ulong;
}
extern "C" {
    pub fn kvmppc_gsid_mask(iden: u16) -> u64;
}
//
// Guest State Buffers
//
extern "C" {
    pub fn kvmppc_gsb_free(gsb: *mut kvmppc_gs_buff);
}
extern "C" {
    pub fn kvmppc_gsb_send(gsb: *mut kvmppc_gs_buff, flags: c_ulong) -> c_int;
}
extern "C" {
    pub fn kvmppc_gsb_recv(gsb: *mut kvmppc_gs_buff, flags: c_ulong) -> c_int;
}
//
// kvmppc_gsb_header() - the header of a guest state buffer
// @gsb: guest state buffer
//
// Returns a pointer to the buffer header.
//
// kvmppc_gsb_data() - the elements of a guest state buffer
// @gsb: guest state buffer
//
// Returns a pointer to the first element of the buffer data.
//
// kvmppc_gsb_len() - the current length of a guest state buffer
// @gsb: guest state buffer
//
// Returns the length including the header of a buffer.
//
// kvmppc_gsb_capacity() - the capacity of a guest state buffer
// @gsb: guest state buffer
//
// Returns the capacity of a buffer.
//
// kvmppc_gsb_paddress() - the physical address of buffer
// @gsb: guest state buffer
//
// Returns the physical address of the buffer.
//
extern "C" {
    pub fn __pa(_arg: kvmppc_gsb_header(gsb)) -> return;
}
//
// kvmppc_gsb_nelems() - the number of elements in a buffer
// @gsb: guest state buffer
//
// Returns the number of elements in a buffer
//
extern "C" {
    pub fn be32_to_cpu(_arg: kvmppc_gsb_header(gsb)->nelems) -> return;
}
//
// kvmppc_gsb_reset() - empty a guest state buffer
// @gsb: guest state buffer
//
// Reset the number of elements and length of buffer to empty.
//
// kvmppc_gsb_data_len() - the length of a buffer excluding the header
// @gsb: guest state buffer
//
// Returns the length of a buffer excluding the header
//
// kvmppc_gsb_data_cap() - the capacity of a buffer excluding the header
// @gsb: guest state buffer
//
// Returns the capacity of a buffer excluding the header
//
// kvmppc_gsb_for_each_elem - iterate over the elements in a buffer
// @i: loop counter
// @pos: set to current element
// @gsb: guest state buffer
// @rem: initialized to buffer capacity, holds bytes currently remaining in
// stream
//

//
// Guest State Elements
//
// kvmppc_gse_iden() - guest state ID of element
// @gse: guest state element
//
// Return the guest state ID in host endianness.
//
extern "C" {
    pub fn be16_to_cpu(_arg: gse->iden) -> return;
}
//
// kvmppc_gse_len() - length of guest state element data
// @gse: guest state element
//
// Returns the length of guest state element data
//
extern "C" {
    pub fn be16_to_cpu(_arg: gse->len) -> return;
}
//
// kvmppc_gse_total_len() - total length of guest state element
// @gse: guest state element
//
// Returns the length of the data plus the ID and size header.
//
extern "C" {
    pub fn be16_to_cpu(sizeof(*gse: *mut gse->len) +) -> return;
}
//
// kvmppc_gse_total_size() - space needed for a given data length
// @size: data length
//
// Returns size plus the space needed for the ID and size header.
//
// kvmppc_gse_data() - pointer to data of a guest state element
// @gse: guest state element
//
// Returns a pointer to the beginning of guest state element data.
//
// kvmppc_gse_ok() - checks space exists for guest state element
// @gse: guest state element
// @remaining: bytes of space remaining
//
// Returns true if the guest state element can fit in remaining space.
//
// kvmppc_gse_next() - iterate to the next guest state element in a stream
// @gse: stream of guest state elements
// @remaining: length of the guest element stream
//
// Returns the next guest state element in a stream of elements. The length of
// the stream is updated in remaining.
//
// remaining -= len;
//
// kvmppc_gse_for_each_elem - iterate over a stream of guest state elements
// @i: loop counter
// @max: number of elements
// @pos: set to current element
// @head: head of elements
// @len: length of the stream
// @rem: initialized to len, holds bytes currently remaining elements
//

extern "C" {
    pub fn kvmppc_gse_parse(gsp: *mut kvmppc_gs_parser, gsb: *mut kvmppc_gs_buff) -> c_int;
}
//
// kvmppc_gse_put_be32() - add a be32 guest state element to a buffer
// @gsb: guest state buffer to add element to
// @iden: guest state ID
// @val: big endian value
//
extern "C" {
    pub fn __kvmppc_gse_put(_arg: gsb, _arg: iden, _arg: sizeof(__be32), _arg: &tmp) -> return;
}
//
// kvmppc_gse_put_u32() - add a host endian 32bit int guest state element to a
// buffer
// @gsb: guest state buffer to add element to
// @iden: guest state ID
// @val: host endian value
//
extern "C" {
    pub fn kvmppc_gse_put_be32(_arg: gsb, _arg: iden, _arg: tmp) -> return;
}
//
// kvmppc_gse_put_be64() - add a be64 guest state element to a buffer
// @gsb: guest state buffer to add element to
// @iden: guest state ID
// @val: big endian value
//
extern "C" {
    pub fn __kvmppc_gse_put(_arg: gsb, _arg: iden, _arg: sizeof(__be64), _arg: &tmp) -> return;
}
//
// kvmppc_gse_put_u64() - add a host endian 64bit guest state element to a
// buffer
// @gsb: guest state buffer to add element to
// @iden: guest state ID
// @val: host endian value
//
extern "C" {
    pub fn kvmppc_gse_put_be64(_arg: gsb, _arg: iden, _arg: tmp) -> return;
}
//
// __kvmppc_gse_put_reg() - add a register type guest state element to a buffer
// @gsb: guest state buffer to add element to
// @iden: guest state ID
// @val: host endian value
//
// Adds a register type guest state element. Uses the guest state ID for
// determining the length of the guest element. If the guest state ID has
// bits that can not be set they will be cleared.
//
extern "C" {
    pub fn kvmppc_gse_put_u64(_arg: gsb, _arg: iden, _arg: val) -> return;
}
extern "C" {
    pub fn kvmppc_gse_put_u32(_arg: gsb, _arg: iden, _arg: tmp) -> return;
}
//
// kvmppc_gse_put_vector128() - add a vector guest state element to a buffer
// @gsb: guest state buffer to add element to
// @iden: guest state ID
// @val: 16 byte vector value
//

extern "C" {
    pub fn __kvmppc_gse_put(_arg: gsb, _arg: iden, _arg: sizeof(tmp), _arg: &tmp) -> return;
}
//
// kvmppc_gse_put_part_table() - add a partition table guest state element to a
// buffer
// @gsb: guest state buffer to add element to
// @iden: guest state ID
// @val: partition table value
//
// kvmppc_gse_put_proc_table() - add a process table guest state element to a
// buffer
// @gsb: guest state buffer to add element to
// @iden: guest state ID
// @val: process table value
//
// kvmppc_gse_put_buff_info() - adds a GSB description guest state element to a
// buffer
// @gsb: guest state buffer to add element to
// @iden: guest state ID
// @val: guest state buffer description value
//
extern "C" {
    pub fn __kvmppc_gse_put(_arg: gsb, _arg: iden, _arg: sizeof(tmp), _arg: &tmp) -> return;
}
//
// kvmppc_gse_get_be32() - return the data of a be32 element
// @gse: guest state element
//
// kvmppc_gse_get_u32() - return the data of a be32 element in host endianness
// @gse: guest state element
//
extern "C" {
    pub fn be32_to_cpu(_arg: kvmppc_gse_get_be32(gse)) -> return;
}
//
// kvmppc_gse_get_be64() - return the data of a be64 element
// @gse: guest state element
//
// kvmppc_gse_get_u64() - return the data of a be64 element in host endianness
// @gse: guest state element
//
extern "C" {
    pub fn be64_to_cpu(_arg: kvmppc_gse_get_be64(gse)) -> return;
}
//
// kvmppc_gse_get_vector128() - return the data of a vector element
// @gse: guest state element
//
// v = u.v;

// v = u.v;
//
// Guest State Bitmap
//
extern "C" {
    pub fn kvmppc_gsbm_test(gsbm: *mut kvmppc_gs_bitmap, iden: u16) -> bool;
}
extern "C" {
    pub fn kvmppc_gsbm_set(gsbm: *mut kvmppc_gs_bitmap, iden: u16);
}
extern "C" {
    pub fn kvmppc_gsbm_clear(gsbm: *mut kvmppc_gs_bitmap, iden: u16);
}
extern "C" {
    pub fn kvmppc_gsbm_next(gsbm: *mut kvmppc_gs_bitmap, prev: u16) -> u16;
}
//
// kvmppc_gsbm_zero - zero the entire bitmap
// @gsbm: guest state buffer bitmap
//
// kvmppc_gsbm_fill - fill the entire bitmap
// @gsbm: guest state buffer bitmap
//
// kvmppc_gsbm_for_each - iterate the present guest state IDs
// @gsbm: guest state buffer bitmap
// @iden: current guest state ID
//

//
// Guest State Parser
//
// kvmppc_gsp_for_each - iterate the <guest state IDs, guest state element>
// pairs
// @gsp: guest state buffer bitmap
// @iden: current guest state ID
// @gse: guest state element
//

//
// Guest State Message
//
// kvmppc_gsm_for_each - iterate the guest state IDs included in a guest state
// message
// @gsp: guest state buffer bitmap
// @iden: current guest state ID
// @gse: guest state element
//

extern "C" {
    pub fn kvmppc_gsm_free(gsm: *mut kvmppc_gs_msg);
}
extern "C" {
    pub fn kvmppc_gsm_size(gsm: *mut kvmppc_gs_msg) -> usize;
}
extern "C" {
    pub fn kvmppc_gsm_fill_info(gsm: *mut kvmppc_gs_msg, gsb: *mut kvmppc_gs_buff) -> c_int;
}
//
// kvmppc_gsm_include - indicate a guest state ID should be included when
// serializing
// @gsm: guest state message
// @iden: guest state ID
//
// kvmppc_gsm_includes - check if a guest state ID will be included when
// serializing
// @gsm: guest state message
// @iden: guest state ID
//
extern "C" {
    pub fn kvmppc_gsbm_test(_arg: &gsm->bitmap, _arg: iden) -> return;
}
//
// kvmppc_gsm_includes - indicate all guest state IDs should be included when
// serializing
// @gsm: guest state message
// @iden: guest state ID
//
// kvmppc_gsm_include - clear the guest state IDs that should be included when
// serializing
// @gsm: guest state message
//
// kvmppc_gsb_receive_data - flexibly update values from a guest state buffer
// @gsb: guest state buffer
// @gsm: guest state message
//
// Requests updated values for the guest state values included in the guest
// state message. The guest state message will then deserialize the guest state
// buffer.
//
// kvmppc_gsb_recv - receive a single guest state ID
// @gsb: guest state buffer
// @gsm: guest state message
// @iden: guest state identity
//
// kvmppc_gsb_send_data - flexibly send values from a guest state buffer
// @gsb: guest state buffer
// @gsm: guest state message
//
// Sends the guest state values included in the guest state message.
//
// kvmppc_gsb_recv - send a single guest state ID
// @gsb: guest state buffer
// @gsm: guest state message
// @iden: guest state identity
//
