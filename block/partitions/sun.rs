//! Automatically rewritten from C to Rust
//! Source: block/partitions/sun.c
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
// fs/partitions/sun.c
//
// Code extracted from drivers/block/genhd.c
//
// Copyright (C) 1991-1998  Linus Torvalds
// Re-organised Feb 1998 Russell King
//

pub const SUN_LABEL_MAGIC: c_uint = 0xDABE;
pub const SUN_VTOC_SANITY: c_uint = 0x600DDEEE;
    enum {
    SUN_WHOLE_DISK = 5,
    LINUX_RAID_PARTITION = 0xfd,	/* autodetect RAID partition */
    };
#[no_mangle]
pub unsafe extern "C" fn sun_partition(state: *mut parsed_partitions) -> c_int {
    int sun_partition(struct parsed_partitions *state)
    {
    int i;
    __be16 csum;
    let mut slot: c_int = 1;
    __be16 *ush;
    Sector sect;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun_disklabel {
    pub /: *mut *mut unsigned char info[128]; / Informative text string,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun_vtoc {
    pub /: *mut *mut __be32 version; / Layout version,
    pub /: *mut *mut char volume[8]; / Volume name,
    pub /: *mut *mut __be16 nparts; / Number of partitions,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun_info {
    pub id: __be16,
    pub flags: __be16,
    pub infos: [}; 8],
    pub /: *mut *mut __be16 padding; / Alignment padding,
    pub /: *mut *mut __be32 bootinfo[3]; / Info needed by mboot,
    pub /: *mut *mut __be32 sanity; / To verify vtoc sanity,
    pub /: *mut *mut __be32 reserved[10]; / Free space,
    pub /: *mut *mut __be32 timestamp[8]; / Partition timestamp,
    pub vtoc: },
    pub /: *mut *mut __be32 write_reinstruct; / sectors to skip, writes,
    pub /: *mut *mut __be32 read_reinstruct; / sectors to skip, reads,
    pub /: *mut *mut unsigned char spare[148]; / Padding,
    pub /: *mut *mut __be16 rspeed; / Disk rotational speed,
    pub /: *mut *mut __be16 pcylcount; / Physical cylinder count,
    pub /: *mut *mut __be16 sparecyl; / extra sects per cylinder,
    pub /: *mut *mut __be16 obs1; / gap1,
    pub /: *mut *mut __be16 obs2; / gap2,
    pub /: *mut *mut __be16 ilfact; / Interleave factor,
    pub /: *mut *mut __be16 ncyl; / Data cylinder count,
    pub /: *mut *mut __be16 nacyl; / Alt. cylinder count,
    pub /: *mut *mut __be16 ntrks; / Tracks per cylinder,
    pub /: *mut *mut __be16 nsect; / Sectors per track,
    pub /: *mut *mut __be16 obs3; / bhead - Label head offset,
    pub /: *mut *mut __be16 obs4; / ppart - Physical Partition,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun_partition {
    pub start_cylinder: __be32,
    pub num_sectors: __be32,
    pub partitions: [}; 8],
    pub /: *mut *mut __be16 magic; / Magic number,
    pub /: *mut *mut __be16 csum; / Label xor'd checksum,
    pub label: *mut *mut },
    pub p: *mut sun_partition,
    pub spc: c_ulong,
    pub use_vtoc: c_int,
    pub nparts: c_int,
    pub &sect): label = read_part_sector(state, 0,,
    if (!label)
    pub -1: return,
    pub label->partitions: p =,
    if (be16_to_cpu(label.magic) != SUN_LABEL_MAGIC) {
    pub 0: return,
    }
// Look at the checksum
    pub 1: *mut *mut ush = ((__be16 ) (label+1)) -,
    pub label);): *mut *mut for (csum = 0; ush >= ((__be16 ),
    pub ush--: *mut csum ^=,
    if (csum) {
    printk("Dev %s Sun disklabel: Csum bad, label corrupted\n",
    pub 0: return,
    }
// Check to see if we can use the VTOC table
    use_vtoc = ((be32_to_cpu(label.vtoc.sanity) == SUN_VTOC_SANITY) &&
    (be32_to_cpu(label.vtoc.version) == 1) &&
    pub 8)): (be16_to_cpu(label->vtoc.nparts) <=,
// Use 8 partition entries if not specified in validated VTOC
    pub 8: nparts = (use_vtoc) ? be16_to_cpu(label->vtoc.nparts) :,
//
// So that old Linux-Sun partitions continue to work,
// alow the VTOC to be used under the additional condition ...
//
    use_vtoc = use_vtoc || !(label.vtoc.sanity ||
    pub label->vtoc.nparts): label->vtoc.version ||,
    pub be16_to_cpu(label->nsect): *mut *mut spc = be16_to_cpu(label->ntrks),
    pub {: for (i = 0; i < nparts; i++, p++),
    pub st_sector: c_ulong,
    pub num_sectors: c_uint,
    pub spc: *mut *mut st_sector = be32_to_cpu(p->start_cylinder),
    pub be32_to_cpu(p->num_sectors): num_sectors =,
    if (num_sectors) {
    pub num_sectors): put_partition(state, slot, st_sector,,
    pub 0: state->parts[slot].flags =,
    if (use_vtoc) {
    if (be16_to_cpu(label.vtoc.infos[i].id) == LINUX_RAID_PARTITION)
    pub ADDPART_FLAG_RAID: state->parts[slot].flags |=,
#[no_mangle]
pub unsafe extern "C" fn if(SUN_WHOLE_DISK: be16_to_cpu(label->vtoc.infos[i].id) ==) -> else {
    else if (be16_to_cpu(label.vtoc.infos[i].id) == SUN_WHOLE_DISK)
    pub ADDPART_FLAG_WHOLEDISK: state->parts[slot].flags |=,
    }
    }
    }
    pub "\n"): seq_buf_puts(&state->pp_buf,,
    pub 1: return,
    }
