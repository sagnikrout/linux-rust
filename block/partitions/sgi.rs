//! Automatically rewritten from C to Rust
//! Source: block/partitions/sgi.c
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
// fs/partitions/sgi.c
//
// Code extracted from drivers/block/genhd.c
//

pub const SGI_LABEL_MAGIC: c_uint = 0x0be5a941;
    enum {
    LINUX_RAID_PARTITION = 0xfd,	/* autodetect RAID partition */
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgi_disklabel {
    pub /: *mut *mut __be32 magic_mushroom; / Big fat spliff...,
    pub /: *mut *mut __be16 root_part_num; / Root partition number,
    pub /: *mut *mut __be16 swap_part_num; / Swap partition number,
    pub /: *mut *mut s8 boot_file[16]; / Name of boot file for ARCS,
    pub /: *mut *mut u8 _unused0[48]; / Device parameter useless crapola..,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgi_volume {
    pub /: *mut *mut s8 name[8]; / Name of volume,
    pub /: *mut *mut __be32 block_num; / Logical block number,
    pub /: *mut *mut __be32 num_bytes; / How big, in bytes,
    pub volume: [}; 15],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgi_partition {
    pub /: *mut *mut __be32 num_blocks; / Size in logical blocks,
    pub /: *mut *mut __be32 first_block; / First logical block,
    pub /: *mut *mut __be32 type; / Type of this partition,
    pub partitions: [}; 16],
    pub /: *mut *mut __be32 csum; / Disk label checksum,
    pub /: *mut *mut __be32 _unused1; / Padding,
}

#[no_mangle]
pub unsafe extern "C" fn sgi_partition(state: *mut parsed_partitions) -> c_int {
    int sgi_partition(struct parsed_partitions *state)
    {
    int i, csum;
    __be32 magic;
    let mut slot: c_int = 1;
    unsigned int start, blocks;
    __be32 *ui, cs;
    Sector sect;
    struct sgi_disklabel *label;
    struct sgi_partition *p;
    label = read_part_sector(state, 0, &sect);
    if (!label)
    return -1;
    p = &label.partitions[0];
    magic = label.magic_mushroom;
    if(be32_to_cpu(magic) != SGI_LABEL_MAGIC) {
    put_dev_sector(sect);
    return 0;
    }
    ui = ((__be32 *) (label + 1)) - 1;
    for(csum = 0; ui >= ((__be32 *) label);) {
    cs = *ui--;
    csum += be32_to_cpu(cs);
    }
    if(csum) {
    printk(KERN_WARNING "Dev %s SGI disklabel: csum bad, label corrupted\n",
    state.disk.disk_name);
    put_dev_sector(sect);
    return 0;
    }
// All SGI disk labels have 16 partitions, disks under Linux only
// have 15 minor's.  Luckily there are always a few zero length
// partitions which we don't care about so we never overflow the
// current_minor.
//
    for(i = 0; i < 16; i++, p++) {
    blocks = be32_to_cpu(p.num_blocks);
    start  = be32_to_cpu(p.first_block);
    if (blocks) {
    put_partition(state, slot, start, blocks);
    if (be32_to_cpu(p.type) == LINUX_RAID_PARTITION)
    state.parts[slot].flags = ADDPART_FLAG_RAID;
    }
    slot++;
    }
    seq_buf_puts(&state.pp_buf, "\n");
    put_dev_sector(sect);
    return 1;
    }
