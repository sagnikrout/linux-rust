//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mtd/partitions.h
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
// MTD partitioning layer definitions
//
// (C) 2000 Nicolas Pitre <nico@fluxnic.net>
//
// This code is GPL
//

//
// Partition definition structure:
//
// An array of struct partition is passed along with a MTD object to
// mtd_device_register() to create them.
//
// For each partition, these fields are available:
// name: string that will be used to label the partition's MTD device.
// types: some partitions can be containers using specific format to describe
// embedded subpartitions / volumes. E.g. many home routers use "firmware"
// partition that contains at least kernel and rootfs. In such case an
// extra parser is needed that will detect these dynamic partitions and
// report them to the MTD subsystem. If set this property stores an array
// of parser names to use when looking for subpartitions.
// size: the partition size; if defined as MTDPART_SIZ_FULL, the partition
// will extend to the end of the master MTD device.
// offset: absolute starting position within the master MTD device; if
// defined as MTDPART_OFS_APPEND, the partition will start where the
// previous one ended; if MTDPART_OFS_NXTBLK, at the next erase block;
// if MTDPART_OFS_RETAIN, consume as much as possible, leaving size
// after the end of partition.
// mask_flags: contains flags that have to be masked (removed) from the
// master MTD flag set for the corresponding MTD partition.
// For example, to force a read-only partition, simply adding
// MTD_WRITEABLE to the mask_flags will do the trick.
// add_flags: contains flags to add to the parent flags
//
// Note: writeable partitions require their size and offset be
// erasesize aligned (e.g. use MTDPART_OFS_NEXTBLK).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtd_partition {
    pub /: *const *const *const char name; / identifier string,
    pub /: *const *const *const *const char types; / names of parsers to use if any,
    pub /: *mut *mut uint64_t size; / partition size,
    pub /: *mut *mut uint64_t offset; / offset within the master MTD space,
    pub /: *mut *mut uint32_t mask_flags; / master MTD flags to mask out for this partition,
    pub /: *mut *mut uint32_t add_flags; / flags to add to the partition,
    pub of_node: *mut device_node,
}

//
// struct mtd_part_parser_data - used to pass data to MTD partition parsers.
// @origin: for RedBoot, start address of MTD device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtd_part_parser_data {
    pub origin: c_ulong,
}

//
// Functions dealing with the various ways of partitioning the space
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtd_part_parser {
    pub list: list_head,
    pub owner: *mut module,
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub ): *mut mtd_part_parser_data,
    pub nr_parts): *const *const *const void (cleanup)(struct mtd_partition pparts, int,
}

// Container for passing around a set of parsed partitions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtd_partitions {
    pub parts: *const mtd_partition,
    pub nr_parts: c_int,
    pub parser: *const mtd_part_parser,
}

extern "C" {
    pub fn deregister_mtd_parser(parser: *mut mtd_part_parser);
}
//
// module_mtd_part_parser() - Helper macro for MTD partition parsers that don't
// do anything special in module init/exit. Each driver may only use this macro
// once, and calling it replaces module_init() and module_exit().
//

extern "C" {
    pub fn mtd_del_partition(master: *mut mtd_info, partno: c_int) -> c_int;
}
extern "C" {
    pub fn mtd_get_device_size(mtd: *const mtd_info) -> u64;
}
