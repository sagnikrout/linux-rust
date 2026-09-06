//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/iso_fs.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

//
// The isofs filesystem constants/structures
//
// This part borrowed from the bsd386 isofs

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iso_volume_descriptor {
    pub /: *mut *mut __u8 type[ISODCL(1,1)]; / 711,
    pub id: [c_char; ISODCL(2,6)],
    pub version: [__u8; ISODCL(7,7)],
    pub data: [__u8; ISODCL(8,2048)],
}

// volume descriptor types
pub const ISO_VD_PRIMARY: c_int = 1;
pub const ISO_VD_SUPPLEMENTARY: c_int = 2;
pub const ISO_VD_END: c_int = 255;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iso_primary_descriptor {
    pub /: *mut *mut __u8 type [ISODCL ( 1, 1)]; / 711,
    pub 6)]: char id [ISODCL ( 2,,
    pub /: *mut *mut __u8 version [ISODCL ( 7, 7)]; / 711,
    pub 8)]: __u8 unused1 [ISODCL ( 8,,
    pub /: *mut *mut char system_id [ISODCL ( 9, 40)]; / achars,
    pub /: *mut *mut char volume_id [ISODCL ( 41, 72)]; / dchars,
    pub 80)]: __u8 unused2 [ISODCL ( 73,,
    pub /: *mut *mut __u8 volume_space_size [ISODCL ( 81, 88)]; / 733,
    pub 120)]: __u8 unused3 [ISODCL ( 89,,
    pub /: *mut *mut __u8 volume_set_size [ISODCL (121, 124)]; / 723,
    pub /: *mut *mut __u8 volume_sequence_number [ISODCL (125, 128)]; / 723,
    pub /: *mut *mut __u8 logical_block_size [ISODCL (129, 132)]; / 723,
    pub /: *mut *mut __u8 path_table_size [ISODCL (133, 140)]; / 733,
    pub /: *mut *mut __u8 type_l_path_table [ISODCL (141, 144)]; / 731,
    pub /: *mut *mut __u8 opt_type_l_path_table [ISODCL (145, 148)]; / 731,
    pub /: *mut *mut __u8 type_m_path_table [ISODCL (149, 152)]; / 732,
    pub /: *mut *mut __u8 opt_type_m_path_table [ISODCL (153, 156)]; / 732,
    pub /: *mut *mut __u8 root_directory_record [ISODCL (157, 190)]; / 9.1,
    pub /: *mut *mut char volume_set_id [ISODCL (191, 318)]; / dchars,
    pub /: *mut *mut char publisher_id [ISODCL (319, 446)]; / achars,
    pub /: *mut *mut char preparer_id [ISODCL (447, 574)]; / achars,
    pub /: *mut *mut char application_id [ISODCL (575, 702)]; / achars,
    pub /: *mut *mut char copyright_file_id [ISODCL (703, 739)]; / 7.5 dchars,
    pub /: *mut *mut char abstract_file_id [ISODCL (740, 776)]; / 7.5 dchars,
    pub /: *mut *mut char bibliographic_file_id [ISODCL (777, 813)]; / 7.5 dchars,
    pub /: *mut *mut __u8 creation_date [ISODCL (814, 830)]; / 8.4.26.1,
    pub /: *mut *mut __u8 modification_date [ISODCL (831, 847)]; / 8.4.26.1,
    pub /: *mut *mut __u8 expiration_date [ISODCL (848, 864)]; / 8.4.26.1,
    pub /: *mut *mut __u8 effective_date [ISODCL (865, 881)]; / 8.4.26.1,
    pub /: *mut *mut __u8 file_structure_version [ISODCL (882, 882)]; / 711,
    pub 883)]: __u8 unused4 [ISODCL (883,,
    pub 1395)]: __u8 application_data [ISODCL (884,,
    pub 2048)]: __u8 unused5 [ISODCL (1396,,
}

// Almost the same as the primary descriptor but two fields are specified
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iso_supplementary_descriptor {
    pub /: *mut *mut __u8 type [ISODCL ( 1, 1)]; / 711,
    pub 6)]: char id [ISODCL ( 2,,
    pub /: *mut *mut __u8 version [ISODCL ( 7, 7)]; / 711,
    pub /: *mut *mut __u8 flags [ISODCL ( 8, 8)]; / 853,
    pub /: *mut *mut char system_id [ISODCL ( 9, 40)]; / achars,
    pub /: *mut *mut char volume_id [ISODCL ( 41, 72)]; / dchars,
    pub 80)]: __u8 unused2 [ISODCL ( 73,,
    pub /: *mut *mut __u8 volume_space_size [ISODCL ( 81, 88)]; / 733,
    pub /: *mut *mut __u8 escape [ISODCL ( 89, 120)]; / 856,
    pub /: *mut *mut __u8 volume_set_size [ISODCL (121, 124)]; / 723,
    pub /: *mut *mut __u8 volume_sequence_number [ISODCL (125, 128)]; / 723,
    pub /: *mut *mut __u8 logical_block_size [ISODCL (129, 132)]; / 723,
    pub /: *mut *mut __u8 path_table_size [ISODCL (133, 140)]; / 733,
    pub /: *mut *mut __u8 type_l_path_table [ISODCL (141, 144)]; / 731,
    pub /: *mut *mut __u8 opt_type_l_path_table [ISODCL (145, 148)]; / 731,
    pub /: *mut *mut __u8 type_m_path_table [ISODCL (149, 152)]; / 732,
    pub /: *mut *mut __u8 opt_type_m_path_table [ISODCL (153, 156)]; / 732,
    pub /: *mut *mut __u8 root_directory_record [ISODCL (157, 190)]; / 9.1,
    pub /: *mut *mut char volume_set_id [ISODCL (191, 318)]; / dchars,
    pub /: *mut *mut char publisher_id [ISODCL (319, 446)]; / achars,
    pub /: *mut *mut char preparer_id [ISODCL (447, 574)]; / achars,
    pub /: *mut *mut char application_id [ISODCL (575, 702)]; / achars,
    pub /: *mut *mut char copyright_file_id [ISODCL (703, 739)]; / 7.5 dchars,
    pub /: *mut *mut char abstract_file_id [ISODCL (740, 776)]; / 7.5 dchars,
    pub /: *mut *mut char bibliographic_file_id [ISODCL (777, 813)]; / 7.5 dchars,
    pub /: *mut *mut __u8 creation_date [ISODCL (814, 830)]; / 8.4.26.1,
    pub /: *mut *mut __u8 modification_date [ISODCL (831, 847)]; / 8.4.26.1,
    pub /: *mut *mut __u8 expiration_date [ISODCL (848, 864)]; / 8.4.26.1,
    pub /: *mut *mut __u8 effective_date [ISODCL (865, 881)]; / 8.4.26.1,
    pub /: *mut *mut __u8 file_structure_version [ISODCL (882, 882)]; / 711,
    pub 883)]: __u8 unused4 [ISODCL (883,,
    pub 1395)]: __u8 application_data [ISODCL (884,,
    pub 2048)]: __u8 unused5 [ISODCL (1396,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hs_volume_descriptor {
    pub /: *mut *mut __u8 foo [ISODCL ( 1, 8)]; / 733,
    pub /: *mut *mut __u8 type [ISODCL ( 9, 9)]; / 711,
    pub 14)]: char id [ISODCL ( 10,,
    pub /: *mut *mut __u8 version [ISODCL ( 15, 15)]; / 711,
    pub data: [__u8; ISODCL(16,2048)],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hs_primary_descriptor {
    pub /: *mut *mut __u8 foo [ISODCL ( 1, 8)]; / 733,
    pub /: *mut *mut __u8 type [ISODCL ( 9, 9)]; / 711,
    pub 14)]: __u8 id [ISODCL ( 10,,
    pub /: *mut *mut __u8 version [ISODCL ( 15, 15)]; / 711,
    pub /: *mut *mut __u8 unused1 [ISODCL ( 16, 16)]; / 711,
    pub /: *mut *mut char system_id [ISODCL ( 17, 48)]; / achars,
    pub /: *mut *mut char volume_id [ISODCL ( 49, 80)]; / dchars,
    pub /: *mut *mut __u8 unused2 [ISODCL ( 81, 88)]; / 733,
    pub /: *mut *mut __u8 volume_space_size [ISODCL ( 89, 96)]; / 733,
    pub /: *mut *mut __u8 unused3 [ISODCL ( 97, 128)]; / 733,
    pub /: *mut *mut __u8 volume_set_size [ISODCL (129, 132)]; / 723,
    pub /: *mut *mut __u8 volume_sequence_number [ISODCL (133, 136)]; / 723,
    pub /: *mut *mut __u8 logical_block_size [ISODCL (137, 140)]; / 723,
    pub /: *mut *mut __u8 path_table_size [ISODCL (141, 148)]; / 733,
    pub /: *mut *mut __u8 type_l_path_table [ISODCL (149, 152)]; / 731,
    pub /: *mut *mut __u8 unused4 [ISODCL (153, 180)]; / 733,
    pub /: *mut *mut __u8 root_directory_record [ISODCL (181, 214)]; / 9.1,
}

// We use this to help us look up the parent inode numbers.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iso_path_table {
    pub /: *mut *mut __u8 name_len[2]; / 721,
    pub /: *mut *mut __u8 extent[4]; / 731,
    pub /: *mut *mut __u8 parent[2]; / 721,
    pub name: [c_char; ],
    pub __attribute__((packed)): },
// high sierra is identical to iso, except that the date is only 6 bytes, and
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iso_directory_record {
    pub /: *mut *mut __u8 length [ISODCL (1, 1)]; / 711,
    pub /: *mut *mut __u8 ext_attr_length [ISODCL (2, 2)]; / 711,
    pub /: *mut *mut __u8 extent [ISODCL (3, 10)]; / 733,
    pub /: *mut *mut __u8 size [ISODCL (11, 18)]; / 733,
    pub /: *mut *mut __u8 date [ISODCL (19, 25)]; / 7 by 711,
    pub 26)]: __u8 flags [ISODCL (26,,
    pub /: *mut *mut __u8 file_unit_size [ISODCL (27, 27)]; / 711,
    pub /: *mut *mut __u8 interleave [ISODCL (28, 28)]; / 711,
    pub /: *mut *mut __u8 volume_sequence_number [ISODCL (29, 32)]; / 723,
    pub /: *mut *mut __u8 name_len [ISODCL (33, 33)]; / 711,
    pub []: char name,
    pub __attribute__((packed)): },
pub const ISOFS_BLOCK_BITS: c_int = 11;
pub const ISOFS_BLOCK_SIZE: c_int = 2048;

