//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/encodings.h
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
// Copyright 2023 Red Hat
//

//
// An in-memory representation of a version number for versioned structures on disk.
//
// A version number consists of two portions, a major version and a minor version. Any format
// change which does not require an explicit upgrade step from the previous version should
// increment the minor version. Any format change which either requires an explicit upgrade step,
// or is wholly incompatible (i.e. can not be upgraded to), should increment the major version, and
// set the minor version to 0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct version_number {
    pub major_version: u32,
    pub minor_version: u32,
}

//
// A packed, machine-independent, on-disk representation of a version_number. Both fields are
// stored in little-endian byte order.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct packed_version_number {
    pub major_version: __le32,
    pub minor_version: __le32,
    pub __packed: },
// The registry of component ids for use in headers
pub const VDO_SUPER_BLOCK: c_int = 0;
pub const VDO_LAYOUT: c_int = 1;
pub const VDO_RECOVERY_JOURNAL: c_int = 2;
pub const VDO_SLAB_DEPOT: c_int = 3;
pub const VDO_BLOCK_MAP: c_int = 4;
pub const VDO_GEOMETRY_BLOCK: c_int = 5;
// The header for versioned data stored on disk.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct header {
    pub /: *mut *mut u32 id; / The component this is a header for,
    pub /: *mut *mut version_number version; / The version of the data format,
    pub /: *mut *mut size_t size; / The size of the data following this header,
}

// A packed, machine-independent, on-disk representation of a component header.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct packed_header {
    pub id: __le32,
    pub version: packed_version_number,
    pub size: __le64,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct index_config {
    pub mem: u32,
    pub unused: u32,
    pub sparse: bool,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum volume_region_id {
    VDO_INDEX_REGION = 0,
    VDO_DATA_REGION = 1,
    VDO_VOLUME_REGION_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct volume_region {
// The ID of the region
    pub id: volume_region_id,
//
// The absolute starting offset on the device. The region continues until the next region
// begins.
//
    pub start_block: physical_block_number_t,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct volume_geometry {
// For backwards compatibility
    pub unused: u32,
// The nonce of this volume
    pub nonce: nonce_t,
// The uuid of this volume
    pub uuid: uuid_t,
// The block offset to be applied to bios
    pub bio_offset: block_count_t,
// The regions in ID order
    pub regions: [volume_region; VDO_VOLUME_REGION_COUNT],
// The index config
    pub index_config: index_config,
    pub __packed: },
// This volume geometry struct is used for sizing only
#[repr(C)]
#[derive(Copy, Clone)]
pub struct volume_geometry_4_0 {
// For backwards compatibility
    pub unused: u32,
// The nonce of this volume
    pub nonce: nonce_t,
// The uuid of this volume
    pub uuid: uuid_t,
// The regions in ID order
    pub regions: [volume_region; VDO_VOLUME_REGION_COUNT],
// The index config
    pub index_config: index_config,
    pub __packed: },
    pub 1]: extern u8 VDO_GEOMETRY_MAGIC_NUMBER[VDO_GEOMETRY_MAGIC_NUMBER_SIZE +,
//
// DOC: Block map entries
//
// The entry for each logical block in the block map is encoded into five bytes, which saves space
// in both the on-disk and in-memory layouts. It consists of the 36 low-order bits of a
// physical_block_number_t (addressing 256 terabytes with a 4KB block size) and a 4-bit encoding of
// a block_mapping_state.
//
// Of the 8 high bits of the 5-byte structure:
//
// Bits 7..4: The four highest bits of the 36-bit physical block number
// Bits 3..0: The 4-bit block_mapping_state
//
// The following 4 bytes are the low order bytes of the physical block number, in little-endian
// order.
//
// Conversion functions to and from a data location are provided.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct block_map_entry {

    pub 4: unsigned mapping_state :,
    pub 4: unsigned pbn_high_nibble :,

    pub 4: unsigned pbn_high_nibble :,
    pub 4: unsigned mapping_state :,

    pub pbn_low_word: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct block_map_page_header {
    pub nonce: __le64,
    pub pbn: __le64,
// May be non-zero on disk
    pub unused_long_word: [u8; 8],
// Whether this page has been written twice to disk
    pub initialized: bool,
// Always zero on disk
    pub unused_byte1: u8,
// May be non-zero on disk
    pub unused_byte2: u8,
    pub unused_byte3: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct block_map_page {
    pub version: packed_version_number,
    pub header: block_map_page_header,
    pub entries: [block_map_entry; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum block_map_page_validity {
    VDO_BLOCK_MAP_PAGE_VALID,
    VDO_BLOCK_MAP_PAGE_INVALID,
// Valid page found in the wrong location on disk
    VDO_BLOCK_MAP_PAGE_BAD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct block_map_state_2_0 {
    pub flat_page_origin: physical_block_number_t,
    pub flat_page_count: block_count_t,
    pub root_origin: physical_block_number_t,
    pub root_count: block_count_t,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct boundary {
    pub levels: [page_number_t; VDO_BLOCK_MAP_TREE_HEIGHT],
}

// The state of the recovery journal as encoded in the VDO super block.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct recovery_journal_state_7_0 {
// Sequence number to start the journal
    pub journal_start: sequence_number_t,
// Number of logical blocks used by VDO
    pub logical_blocks_used: block_count_t,
// Number of block map pages allocated
    pub block_map_data_blocks: block_count_t,
    pub __packed: },
    pub VDO_RECOVERY_JOURNAL_HEADER_7_0: extern struct header,
pub type journal_entry_count_t = u16;
//
// A recovery journal entry stores three physical locations: a data location that is the value of a
// single mapping in the block map tree, and the two locations of the block map pages and slots
// that are acquiring and releasing a reference to the location. The journal entry also stores an
// operation code that says whether the mapping is for a logical block or for the block map tree
// itself.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct recovery_journal_entry {
    pub slot: block_map_slot,
    pub mapping: data_location,
    pub unmapping: data_location,
    pub operation: journal_operation,
}

// The packed, on-disk representation of a recovery journal entry.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct packed_recovery_journal_entry {
//
// In little-endian bit order:
// Bits 15..12: The four highest bits of the 36-bit physical block number of the block map
// tree page
// Bits 11..2: The 10-bit block map page slot number
// Bit 1..0: The journal_operation of the entry (this actually only requires 1 bit, but
// it is convenient to keep the extra bit as part of this field.
//

    pub 2: unsigned operation :,
    pub 6: unsigned slot_low :,
    pub 4: unsigned slot_high :,
    pub 4: unsigned pbn_high_nibble :,

    pub 6: unsigned slot_low :,
    pub 2: unsigned operation :,
    pub 4: unsigned pbn_high_nibble :,
    pub 4: unsigned slot_high :,

//
// Bits 47..16: The 32 low-order bits of the block map page PBN, in little-endian byte
// order
//
    pub pbn_low_word: __le32,
//
// Bits 87..48: The five-byte block map entry encoding the location that will be stored in
// the block map page slot
//
    pub mapping: block_map_entry,
//
// Bits 127..88: The five-byte block map entry encoding the location that was stored in the
// block map page slot
//
    pub unmapping: block_map_entry,
    pub __packed: },
// The packed, on-disk representation of an old format recovery journal entry.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct packed_recovery_journal_entry_1 {
//
// In little-endian bit order:
// Bits 15..12: The four highest bits of the 36-bit physical block number of the block map
// tree page
// Bits 11..2: The 10-bit block map page slot number
// Bits 1..0: The 2-bit journal_operation of the entry
//

    pub 2: unsigned operation :,
    pub 6: unsigned slot_low :,
    pub 4: unsigned slot_high :,
    pub 4: unsigned pbn_high_nibble :,

    pub 6: unsigned slot_low :,
    pub 2: unsigned operation :,
    pub 4: unsigned pbn_high_nibble :,
    pub 4: unsigned slot_high :,

//
// Bits 47..16: The 32 low-order bits of the block map page PBN, in little-endian byte
// order
//
    pub pbn_low_word: __le32,
//
// Bits 87..48: The five-byte block map entry encoding the location that was or will be
// stored in the block map page slot
//
    pub block_map_entry: block_map_entry,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum journal_operation_1 {
    VDO_JOURNAL_DATA_DECREMENT = 0,
    VDO_JOURNAL_DATA_INCREMENT = 1,
    VDO_JOURNAL_BLOCK_MAP_DECREMENT = 2,
    VDO_JOURNAL_BLOCK_MAP_INCREMENT = 3,
    } __packed;

    struct recovery_block_header {
    sequence_number_t block_map_head; /* Block map head sequence number */
    sequence_number_t slab_journal_head; /* Slab journal head seq. number */
    sequence_number_t sequence_number; /* Sequence number for this block */
    nonce_t nonce; /* A given VDO instance's nonce */
    block_count_t logical_blocks_used; /* Logical blocks in use */
    block_count_t block_map_data_blocks; /* Allocated block map pages */
    journal_entry_count_t entry_count; /* Number of entries written */
    u8 check_byte; /* The protection check byte */
    u8 recovery_count; /* Number of recoveries completed */
    enum vdo_metadata_type metadata_type; /* Metadata type */
}

//
// The packed, on-disk representation of a recovery journal block header. All fields are kept in
// little-endian byte order.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct packed_journal_header {
// Block map head 64-bit sequence number
    pub block_map_head: __le64,
// Slab journal head 64-bit sequence number
    pub slab_journal_head: __le64,
// The 64-bit sequence number for this block
    pub sequence_number: __le64,
// A given VDO instance's 64-bit nonce
    pub nonce: __le64,
// 8-bit metadata type (should always be one for the recovery journal)
    pub metadata_type: u8,
// 16-bit count of the entries encoded in the block
    pub entry_count: __le16,
// 64-bit count of the logical blocks used when this block was opened
    pub logical_blocks_used: __le64,
// 64-bit count of the block map blocks used when this block was opened
    pub block_map_data_blocks: __le64,
// The protection check byte
    pub check_byte: u8,
// The number of recoveries completed
    pub recovery_count: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct packed_journal_sector {
// The protection check byte
    pub check_byte: u8,
// The number of recoveries completed
    pub recovery_count: u8,
// The number of entries in this sector
    pub entry_count: u8,
// Journal entries for this sector
    pub entries: [packed_recovery_journal_entry; ],
    pub __packed: },
// The number of entries in each sector (except the last) when filled
// The number of entries in a v1 recovery journal block.
// The number of entries in each v1 sector (except the last) when filled
// The number of entries in the last sector when a block is full
}

// A type representing a reference count of a block.
pub type vdo_refcount_t = u8;
// The absolute position of an entry in a recovery journal or slab journal.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct journal_point {
    pub sequence_number: sequence_number_t,
    pub entry_count: journal_entry_count_t,
}

// A packed, platform-independent encoding of a struct journal_point.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct packed_journal_point {
//
// The packed representation is the little-endian 64-bit representation of the low-order 48
// bits of the sequence number, shifted up 16 bits, or'ed with the 16-bit entry count.
//
// Very long-term, the top 16 bits of the sequence number may not always be zero, as this
// encoding assumes--see BZ 1523240.
//
    pub encoded_point: __le64,
    pub __packed: },
// Special vdo_refcount_t values.
pub const EMPTY_REFERENCE_COUNT: c_int = 0;
}

// The format of each sector of a reference_block on disk.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct packed_reference_sector {
    pub commit_point: packed_journal_point,
    pub counts: [vdo_refcount_t; COUNTS_PER_SECTOR],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct packed_reference_block {
    pub sectors: [packed_reference_sector; VDO_SECTORS_PER_BLOCK],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slab_depot_state_2_0 {
    pub slab_config: slab_config,
    pub first_block: physical_block_number_t,
    pub last_block: physical_block_number_t,
    pub zone_count: zone_count_t,
    pub __packed: },
    pub VDO_SLAB_DEPOT_HEADER_2_0: extern struct header,
//
// vdo_slab journal blocks may have one of two formats, depending upon whether or not any of the
// entries in the block are block map increments. Since the steady state for a VDO is that all of
// the necessary block map pages will be allocated, most slab journal blocks will have only data
// entries. Such blocks can hold more entries, hence the two formats.
//
// A single slab journal entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slab_journal_entry {
    pub sbn: slab_block_number,
    pub operation: journal_operation,
    pub increment: bool,
}

// A single slab journal entry in its on-disk form

// The unpacked representation of the header of a slab journal block
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slab_journal_block_header {
// Sequence number for head of journal
    pub head: sequence_number_t,
// Sequence number for this block
    pub sequence_number: sequence_number_t,
// The nonce for a given VDO instance
    pub nonce: nonce_t,
// Recovery journal point for last entry
    pub recovery_point: journal_point,
// Metadata type
    pub metadata_type: vdo_metadata_type,
// Whether this block contains block map increments
    pub has_block_map_increments: bool,
// The number of entries in the block
    pub entry_count: journal_entry_count_t,
}

//
// The packed, on-disk representation of a slab journal block header. All fields are kept in
// little-endian byte order.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct packed_slab_journal_block_header {
// 64-bit sequence number for head of journal
    pub head: __le64,
// 64-bit sequence number for this block
    pub sequence_number: __le64,
// Recovery journal point for the last entry, packed into 64 bits
    pub recovery_point: packed_journal_point,
// The 64-bit nonce for a given VDO instance
    pub nonce: __le64,
// 8-bit metadata type (should always be two, for the slab journal)
    pub metadata_type: u8,
// Whether this block contains block map increments
    pub has_block_map_increments: bool,
// 16-bit count of the entries encoded in the block
    pub entry_count: __le16,
    pub __packed: },
}

// The payload of a slab journal block which has block map increments
#[repr(C)]
#[derive(Copy, Clone)]
pub struct full_slab_journal_entries {
// The entries themselves
    pub entries: [packed_slab_journal_entry; VDO_SLAB_JOURNAL_FULL_ENTRIES_PER_BLOCK],
// The bit map indicating which entries are block map increments
    pub entry_types: [u8; VDO_SLAB_JOURNAL_ENTRY_TYPES_SIZE],
    pub __packed: },
// Entries which include block map increments
    pub full_entries: full_slab_journal_entries,
// Entries which are only data updates
    pub entries: [packed_slab_journal_entry; VDO_SLAB_JOURNAL_ENTRIES_PER_BLOCK],
// Ensure the payload fills to the end of the block
    pub space: [u8; VDO_SLAB_JOURNAL_PAYLOAD_SIZE],
    pub slab_journal_payload: } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct packed_slab_journal_block {
    pub header: packed_slab_journal_block_header,
    pub payload: slab_journal_payload,
    pub __packed: },
// The offset of a slab journal tail block.
pub type tail_block_offset_t = u8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slab_summary_entry {
// Bits 7..0: The offset of the tail block within the slab journal
    pub tail_block_offset: tail_block_offset_t,

// Bits 13..8: A hint about the fullness of the slab
    pub 6: unsigned int fullness_hint :,
// Bit 14: Whether the ref_counts must be loaded from the layer
    pub 1: unsigned int load_ref_counts :,
// Bit 15: The believed cleanliness of this slab
    pub 1: unsigned int is_dirty :,

// Bit 15: The believed cleanliness of this slab
    pub 1: unsigned int is_dirty :,
// Bit 14: Whether the ref_counts must be loaded from the layer
    pub 1: unsigned int load_ref_counts :,
// Bits 13..8: A hint about the fullness of the slab
    pub 6: unsigned int fullness_hint :,

    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct layout {
    pub start: physical_block_number_t,
    pub size: block_count_t,
    pub first_free: physical_block_number_t,
    pub last_free: physical_block_number_t,
    pub num_partitions: usize,
    pub head: *mut partition,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct partition {
    pub /: *mut *mut partition_id id; / The id of this partition,
    pub /: *mut *mut physical_block_number_t offset; / The offset into the layout of this partition,
    pub /: *mut *mut block_count_t count; / The number of blocks in the partition,
    pub /: *mut *mut *mut partition next; / A pointer to the next partition in the layout,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct layout_3_0 {
    pub first_free: physical_block_number_t,
    pub last_free: physical_block_number_t,
    pub partition_count: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct partition_3_0 {
    pub id: partition_id,
    pub offset: physical_block_number_t,
    pub /: *mut *mut physical_block_number_t base; / unused but retained for backwards compatibility,
    pub count: block_count_t,
    pub __packed: },
//
// The configuration of the VDO service.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdo_config {
    pub /: *mut *mut block_count_t logical_blocks; / number of logical blocks,
    pub /: *mut *mut block_count_t physical_blocks; / number of physical blocks,
    pub /: *mut *mut block_count_t slab_size; / number of blocks in a slab,
    pub /: *mut *mut block_count_t recovery_journal_size; / number of recovery journal blocks,
    pub /: *mut *mut block_count_t slab_journal_blocks; / number of slab journal blocks,
}

// The maximum logical space is 4 petabytes, which is 1 terablock.

// The maximum physical space is 256 terabytes, which is 64 gigablocks.

// This is the structure that captures the vdo fields saved as a super block component.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdo_component {
    pub state: vdo_state,
    pub complete_recoveries: u64,
    pub read_only_recoveries: u64,
    pub config: vdo_config,
    pub nonce: nonce_t,
}

//
// A packed, machine-independent, on-disk representation of the vdo_config in the VDO component
// data in the super block.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct packed_vdo_config {
    pub logical_blocks: __le64,
    pub physical_blocks: __le64,
    pub slab_size: __le64,
    pub recovery_journal_size: __le64,
    pub slab_journal_blocks: __le64,
    pub __packed: },
//
// A packed, machine-independent, on-disk representation of version 41.0 of the VDO component data
// in the super block.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct packed_vdo_component_41_0 {
    pub state: __le32,
    pub complete_recoveries: __le64,
    pub read_only_recoveries: __le64,
    pub config: packed_vdo_config,
    pub nonce: __le64,
    pub __packed: },
//
// The version of the on-disk format of a VDO volume. This should be incremented any time the
// on-disk representation of any VDO structure changes. Changes which require only online upgrade
// steps should increment the minor version. Changes which require an offline upgrade or which can
// not be upgraded to at all should increment the major version and set the minor version to 0.
//
    pub VDO_VOLUME_VERSION_67_0: extern struct version_number,
}

// The entirety of the component data encoded in the VDO super block.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdo_component_states {
// For backwards compatibility
    pub unused: u32,
// The VDO volume version
    pub volume_version: version_number,
// Components
    pub vdo: vdo_component,
    pub block_map: block_map_state_2_0,
    pub recovery_journal: recovery_journal_state_7_0,
    pub slab_depot: slab_depot_state_2_0,
// Our partitioning of the underlying storage
    pub layout: layout,
}

//
// vdo_are_same_version() - Check whether two version numbers are the same.
// @version_a: The first version.
// @version_b: The second version.
//
// Return: true if the two versions are the same.
//
// vdo_pack_version_number() - Convert a version_number to its packed on-disk representation.
// @version: The version number to convert.
//
// Return: the platform-independent representation of the version
//
// vdo_unpack_version_number() - Convert a packed_version_number to its native in-memory
// representation.
// @version: The version number to convert.
//
// Return: The platform-independent representation of the version.
//
// vdo_pack_header() - Convert a component header to its packed on-disk representation.
// @header: The header to convert.
//
// Return: the platform-independent representation of the header
//
// vdo_unpack_header() - Convert a packed_header to its native in-memory representation.
// @header: The header to convert.
//
// Return: The platform-independent representation of the version.
//
// vdo_get_index_region_start() - Get the start of the index region from a geometry.
// @geometry: The geometry.
//
// Return: The start of the index region.
//
// vdo_get_data_region_start() - Get the start of the data region from a geometry.
// @geometry: The geometry.
//
// Return: The start of the data region.
//
// vdo_get_index_region_size() - Get the size of the index region from a geometry.
// @geometry: The geometry.
//
// Return: The size of the index region.
//
extern "C" {
    pub fn vdo_is_mapped_location(_arg: location) -> return;
}
extern "C" {
    pub fn __le64_to_cpu(_arg: page->header.pbn) -> return;
}
extern "C" {
    pub fn DIV_ROUND_UP(_arg: entries, _arg: VDO_BLOCK_MAP_ENTRIES_PER_PAGE) -> return;
}
//
// vdo_pack_recovery_journal_entry() - Return the packed, on-disk representation of a recovery
// journal entry.
// @entry: The journal entry to pack.
//
// Return: The packed representation of the journal entry.
//
// vdo_unpack_recovery_journal_entry() - Unpack the on-disk representation of a recovery journal
// entry.
// @entry: The recovery journal entry to unpack.
//
// Return: The unpacked entry.
//
extern "C" {
    pub fn vdo_get_journal_operation_name(operation: journal_operation) -> *const char  __must_check;
}
//
// vdo_is_valid_recovery_journal_sector() - Determine whether the header of the given sector could
// describe a valid sector for the given journal block
// header.
// @header: The unpacked block header to compare against.
// @sector: The packed sector to check.
// @sector_number: The number of the sector being checked.
//
// Return: true if the sector matches the block header.
//
// vdo_compute_recovery_journal_block_number() - Compute the physical block number of the recovery
// journal block which would have a given sequence
// number.
// @journal_size: The size of the journal.
// @sequence_number: The sequence number.
//
// Return: The pbn of the journal block which would the specified sequence number.
//
// Since journal size is a power of two, the block number modulus can just be extracted
// from the low-order bits of the sequence.
//
// vdo_get_journal_block_sector() - Find the recovery journal sector from the block header and
// sector number.
// @header: The header of the recovery journal block.
// @sector_number: The index of the sector (1-based).
//
// Return: A packed recovery journal sector.
//
// vdo_pack_recovery_block_header() - Generate the packed representation of a recovery block
// header.
// @header: The header containing the values to encode.
// @packed: The header into which to pack the values.
//
// packed = (struct packed_journal_header) {
//
// vdo_unpack_recovery_block_header() - Decode the packed representation of a recovery block
// header.
// @packed: The packed header to decode.
//
// Return: The unpacked header.
//
// vdo_compute_slab_count() - Compute the number of slabs a depot with given parameters would have.
// @first_block: PBN of the first data block.
// @last_block: PBN of the last data block.
// @slab_size_shift: Exponent for the number of blocks per slab.
//
// Return: The number of slabs.
//
// vdo_get_saved_reference_count_size() - Get the number of blocks required to save a reference
// counts state covering the specified number of data
// blocks.
// @block_count: The number of physical data blocks that can be referenced.
//
// Return: The number of blocks required to save reference counts with the given block count.
//
extern "C" {
    pub fn DIV_ROUND_UP(_arg: block_count, _arg: COUNTS_PER_BLOCK) -> return;
}
//
// vdo_get_slab_journal_start_block() - Get the physical block number of the start of the slab
// journal relative to the start block allocator partition.
// @slab_config: The slab configuration of the VDO.
// @origin: The first block of the slab.
//
// vdo_advance_journal_point() - Move the given journal point forward by one entry.
// @point: The journal point to adjust.
// @entries_per_block: The number of entries in one full block.
//
// vdo_before_journal_point() - Check whether the first point precedes the second point.
// @first: The first journal point.
// @second: The second journal point.
//
// Return: true if the first point precedes the second point.
//
// vdo_pack_journal_point() - Encode the journal location represented by a
// journal_point into a packed_journal_point.
// @unpacked: The unpacked input point.
// @packed: The packed output point.
//
// vdo_unpack_journal_point() - Decode the journal location represented by a packed_journal_point
// into a journal_point.
// @packed: The packed input point.
// @unpacked: The unpacked output point.
//
// vdo_pack_slab_journal_block_header() - Generate the packed representation of a slab block
// header.
// @header: The header containing the values to encode.
// @packed: The header into which to pack the values.
//
// vdo_unpack_slab_journal_block_header() - Decode the packed representation of a slab block
// header.
// @packed: The packed header to decode.
// @header: The header into which to unpack the values.
//
// header = (struct slab_journal_block_header) {
//
// vdo_pack_slab_journal_entry() - Generate the packed encoding of a slab journal entry.
// @packed: The entry into which to pack the values.
// @sbn: The slab block number of the entry to encode.
// @is_increment: The increment flag.
//
// vdo_unpack_slab_journal_entry() - Decode the packed representation of a slab journal entry.
// @packed: The packed entry to decode.
//
// Return: The decoded slab journal entry.
//
// vdo_get_slab_summary_hint_shift() - Compute the shift for slab summary hints.
// @slab_size_shift: Exponent for the number of blocks per slab.
//
// Return: The hint shift.
//
extern "C" {
    pub fn vdo_uninitialize_layout(layout: *mut layout);
}
extern "C" {
    pub fn vdo_destroy_component_states(states: *mut vdo_component_states);
}
extern "C" {
    pub fn vdo_encode_super_block(buffer: *mut u8, states: *mut vdo_component_states);
}
extern "C" {
    pub fn vdo_decode_super_block(buffer: *mut u8) -> int __must_check;
}
// We start with 0L and postcondition with ~0L to match our historical usage in userspace.
