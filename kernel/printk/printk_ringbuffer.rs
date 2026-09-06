//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/printk/printk_ringbuffer.h
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
// Meta information about each stored message.
//
// All fields are set by the printk code except for @seq, which is
// set by the ringbuffer code.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct printk_info {
    pub /: *mut *mut u64 seq; / sequence number,
    pub /: *mut *mut u64 ts_nsec; / timestamp in nanoseconds,
    pub /: *mut *mut u16 text_len; / length of text message,
    pub /: *mut *mut u8 facility; / syslog facility,
    pub /: *mut *mut u8 flags:5; / internal record flags,
    pub /: *mut *mut u8 level:3; / syslog level,
    pub /: *mut *mut u32 caller_id; / thread id or processor id,

    pub /: *mut *mut u32 caller_id2; / caller_id complement,
// name of the task that generated the message
    pub comm: [c_char; TASK_COMM_LEN],
    pub dev_info: dev_printk_info,
}

//
// A structure providing the buffers, used by writers and readers.
//
// Writers:
// Using prb_rec_init_wr(), a writer sets @text_buf_size before calling
// prb_reserve(). On success, prb_reserve() sets @info and @text_buf to
// buffers reserved for that writer.
//
// Readers:
// Using prb_rec_init_rd(), a reader sets all fields before calling
// prb_read_valid(). Note that the reader provides the @info and @text_buf,
// buffers. On success, the struct pointed to by @info will be filled and
// the char array pointed to by @text_buf will be filled with text data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct printk_record {
    pub info: *mut printk_info,
    pub text_buf: *mut c_char,
    pub text_buf_size: c_uint,
}

// Specifies the logical position and span of a data block.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prb_data_blk_lpos {
    pub begin: c_ulong,
    pub next: c_ulong,
}

//
// A descriptor: the complete meta-data for a record.
//
// @state_var: A bitwise combination of descriptor ID and descriptor state.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prb_desc {
    pub state_var: atomic_long_t,
    pub text_blk_lpos: prb_data_blk_lpos,
}

// A ringbuffer of "ID + data" elements.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prb_data_ring {
    pub size_bits: c_uint,
    pub data: *mut c_char,
    pub head_lpos: atomic_long_t,
    pub tail_lpos: atomic_long_t,
}

// A ringbuffer of "struct prb_desc" elements.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prb_desc_ring {
    pub count_bits: c_uint,
    pub descs: *mut prb_desc,
    pub infos: *mut printk_info,
    pub head_id: atomic_long_t,
    pub tail_id: atomic_long_t,
    pub last_finalized_seq: atomic_long_t,
}

//
// The high level structure representing the printk ringbuffer.
//
// @fail: Count of failed prb_reserve() calls where not even a data-less
// record was created.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct printk_ringbuffer {
    pub desc_ring: prb_desc_ring,
    pub text_data_ring: prb_data_ring,
    pub fail: atomic_long_t,
}

//
// Used by writers as a reserve/commit handle.
//
// @rb:         Ringbuffer where the entry is reserved.
// @irqflags:   Saved irq flags to restore on entry commit.
// @id:         ID of the reserved descriptor.
// @text_space: Total occupied buffer space in the text data ring, including
// ID, alignment padding, and wrapping data blocks.
//
// This structure is an opaque handle for writers. Its contents are only
// to be used by the ringbuffer implementation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prb_reserved_entry {
    pub rb: *mut printk_ringbuffer,
    pub irqflags: c_ulong,
    pub id: c_ulong,
    pub text_space: c_uint,
}

// The possible responses of a descriptor state-query.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum desc_state {
    desc_miss	=  -1,	/* ID mismatch (pseudo state) */
    desc_reserved	= 0x0,	/* reserved, in use by writer */
    desc_committed	= 0x1,	/* committed by writer, could get reopened */
    desc_finalized	= 0x2,	/* committed, no further modification allowed */
    desc_reusable	= 0x3,	/* free, not yet used by any writer */
}

//
// Special data block logical position values (for fields of
// @prb_desc.text_blk_lpos).
//
// - Bit0 is used to identify if the record has no data block. (Implemented in
// the LPOS_DATALESS() macro.)
//
// - Bit1 specifies the reason for not having a data block.
//
// These special values could never be real lpos values because of the
// meta data and alignment padding of data blocks. (See to_blk_size() for
// details.)
//
pub const FAILED_LPOS: c_uint = 0x1;
pub const EMPTY_LINE_LPOS: c_uint = 0x3;

//
// Descriptor Bootstrap
//
// The descriptor array is minimally initialized to allow immediate usage
// by readers and writers. The requirements that the descriptor array
// initialization must satisfy:
//
// Req1
// The tail must point to an existing (committed or reusable) descriptor.
// This is required by the implementation of prb_first_seq().
//
// Req2
// Readers must see that the ringbuffer is initially empty.
//
// Req3
// The first record reserved by a writer is assigned sequence number 0.
//
// To satisfy Req1, the tail initially points to a descriptor that is
// minimally initialized (having no data block, i.e. data-less with the
// data block's lpos @begin and @next values set to FAILED_LPOS).
//
// To satisfy Req2, the initial tail descriptor is initialized to the
// reusable state. Readers recognize reusable descriptors as existing
// records, but skip over them.
//
// To satisfy Req3, the last descriptor in the array is used as the initial
// head (and tail) descriptor. This allows the first record reserved by a
// writer (head + 1) to be the first descriptor in the array. (Only the first
// descriptor in the array could have a valid sequence number of 0.)
//
// The first time a descriptor is reserved, it is assigned a sequence number
// with the value of the array index. A "first time reserved" descriptor can
// be recognized because it has a sequence number of 0 but does not have an
// index of 0. (Only the first descriptor in the array could have a valid
// sequence number of 0.) After the first reservation, all future reservations
// (recycling) simply involve incrementing the sequence number by the array
// count.
//
// Hack #1
// Only the first descriptor in the array is allowed to have the sequence
// number 0. In this case it is not possible to recognize if it is being
// reserved the first time (set to index value) or has been reserved
// previously (increment by the array count). This is handled by _always_
// incrementing the sequence number by the array count when reserving the
// first descriptor in the array. In order to satisfy Req3, the sequence
// number of the first descriptor in the array is initialized to minus
// the array count. Then, upon the first reservation, it is incremented
// to 0, thus satisfying Req3.
//
// Hack #2
// prb_first_seq() can be called at any time by readers to retrieve the
// sequence number of the tail descriptor. However, due to Req2 and Req3,
// initially there are no records to report the sequence number of
// (sequence numbers are u64 and there is nothing less than 0). To handle
// this, the sequence number of the initial tail descriptor is initialized
// to 0. Technically this is incorrect, because there is no record with
// sequence number 0 (yet) and the tail descriptor is not the first
// descriptor in the array. But it allows prb_read_valid() to correctly
// report the existence of a record for _any_ given sequence number at all
// times. Bootstrapping is complete when the tail is pushed the first
// time, thus finally pointing to the first descriptor reserved by a
// writer, which has the assigned sequence number 0.
//
// Initiating Logical Value Overflows
//
// Both logical position (lpos) and ID values can be mapped to array indexes
// but may experience overflows during the lifetime of the system. To ensure
// that printk_ringbuffer can handle the overflows for these types, initial
// values are chosen that map to the correct initial array indexes, but will
// result in overflows soon.
//
// BLK0_LPOS
// The initial @head_lpos and @tail_lpos for data rings. It is at index
// 0 and the lpos value is such that it will overflow on the first wrap.
//
// DESC0_ID
// The initial @head_id and @tail_id for the desc ring. It is at the last
// index of the descriptor array (see Req3 above) and the ID value is such
// that it will overflow on the second wrap.
//

//
// Define a ringbuffer with an external text data buffer. The same as
// DEFINE_PRINTKRB() but requires specifying an external buffer for the
// text data.
//
// Note: The specified external buffer must be of the size:
// 2 ^ (descbits + avgtextbits)
//

// the initial head and tail */								\
// reusable */									\
// no associated data block */							\
// this will be the first record reserved by a writer */				\
// will be incremented to 0 on the first reservation */				\
// the initial head and tail */								\
// reports the first seq value during the bootstrap phase */			\
//
// DEFINE_PRINTKRB() - Define a ringbuffer.
//
// @name:        The name of the ringbuffer variable.
// @descbits:    The number of descriptors as a power-of-2 value.
// @avgtextbits: The average text data size per record as a power-of-2 value.
//
// This is a macro for defining a ringbuffer and all internal structures
// such that it is ready for immediate use. See _DEFINE_PRINTKRB() for a
// variant where the text data buffer can be specified externally.
//

// Writer Interface
//
// prb_rec_init_wr() - Initialize a buffer for writing records.
//
// @r:             The record to initialize.
// @text_buf_size: The needed text buffer size.
//
extern "C" {
    pub fn prb_commit(e: *mut prb_reserved_entry);
}
extern "C" {
    pub fn prb_final_commit(e: *mut prb_reserved_entry);
}
extern "C" {
    pub fn prb_record_text_space(e: *mut prb_reserved_entry) -> c_uint;
}
// Reader Interface
//
// prb_rec_init_rd() - Initialize a buffer for reading records.
//
// @r:             The record to initialize.
// @info:          A buffer to store record meta-data.
// @text_buf:      A buffer to store text data.
// @text_buf_size: The size of @text_buf.
//
// Initialize all the fields that a reader is interested in. All arguments
// (except @r) are optional. Only record data for arguments that are
// non-NULL or non-zero will be read.
//
// prb_for_each_record() - Iterate over the records of a ringbuffer.
//
// @from: The sequence number to begin with.
// @rb:   The ringbuffer to iterate over.
// @s:    A u64 to store the sequence number on each iteration.
// @r:    A printk_record to store the record on each iteration.
//
// This is a macro for conveniently iterating over a ringbuffer.
// Note that @s may not be the sequence number of the record on each
// iteration. For the sequence number, @r->info->seq should be checked.
//
// Context: Any context.
//

//
// prb_for_each_info() - Iterate over the meta data of a ringbuffer.
//
// @from: The sequence number to begin with.
// @rb:   The ringbuffer to iterate over.
// @s:    A u64 to store the sequence number on each iteration.
// @i:    A printk_info to store the record meta data on each iteration.
// @lc:   An unsigned int to store the text line count of each record.
//
// This is a macro for conveniently iterating over a ringbuffer.
// Note that @s may not be the sequence number of the record on each
// iteration. For the sequence number, @i->seq should be checked.
//
// Context: Any context.
//

extern "C" {
    pub fn prb_first_seq(rb: *mut printk_ringbuffer) -> u64;
}
extern "C" {
    pub fn prb_first_valid_seq(rb: *mut printk_ringbuffer) -> u64;
}
extern "C" {
    pub fn prb_next_seq(rb: *mut printk_ringbuffer) -> u64;
}
extern "C" {
    pub fn prb_next_reserve_seq(rb: *mut printk_ringbuffer) -> u64;
}

//
// The provided sequence is only the lower 32 bits of the ringbuffer
// sequence. It needs to be expanded to 64bit. Get the first sequence
// number from the ringbuffer and fold it.
//
// Having a 32bit representation in the console is sufficient.
// If a console ever gets more than 2^31 records behind
// the ringbuffer then this is the least of the problems.
//
// Also the access to the ring buffer is always safe.
//

