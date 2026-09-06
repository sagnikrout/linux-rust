//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/msdos_fs.h
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
// The MS-DOS filesystem constants/structures
//

// directory limit

// attribute bits that are copied "as is"

// bits that are used by the Windows 95/Windows NT extended FAT

pub const DELETED_FLAG: c_uint = 0xe5	/* marks file as deleted when in name[0] */;

// start of data cluster's entry (number of reserved clusters)
pub const FAT_START_ENT: c_int = 2;
// maximum number of clusters
pub const MAX_FAT12: c_uint = 0xFF4;
pub const MAX_FAT16: c_uint = 0xFFF4;
pub const MAX_FAT32: c_uint = 0x0FFFFFF6;
// bad cluster mark
pub const BAD_FAT12: c_uint = 0xFF7;
pub const BAD_FAT16: c_uint = 0xFFF7;
pub const BAD_FAT32: c_uint = 0x0FFFFFF7;
// standard EOF
pub const EOF_FAT12: c_uint = 0xFFF;
pub const EOF_FAT16: c_uint = 0xFFFF;
pub const EOF_FAT32: c_uint = 0x0FFFFFFF;

pub const FAT_FSINFO_SIG1: c_uint = 0x41615252;
pub const FAT_FSINFO_SIG2: c_uint = 0x61417272;

pub const FAT_STATE_DIRTY: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __fat_dirent {
    pub d_ino: c_long,
    pub d_off: __kernel_off_t,
    pub d_reclen: c_ushort,
    pub /: *mut *mut char d_name[256]; / We must not include limits.h!,
}

//
// ioctl commands
//

// <linux/videotext.h> has used 0x72 ('r') in collision, so skip a few

// Android kernel has used 0x12, so we use 0x13

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fat_boot_sector {
    pub /: *mut *mut __u8 ignored[3]; / Boot strap short or near jump,
    pub case: *mut *mut __u8 system_id[8]; / Name - can be used to special,
    pub /: *mut *mut __u8 sector_size[2]; / bytes per logical sector,
    pub /: *mut *mut __u8 sec_per_clus; / sectors/cluster,
    pub /: *mut *mut __le16 reserved; / reserved sectors,
    pub /: *mut *mut __u8 fats; / number of FATs,
    pub /: *mut *mut __u8 dir_entries[2]; / root directory entries,
    pub /: *mut *mut __u8 sectors[2]; / number of sectors,
    pub /: *mut *mut __u8 media; / media code,
    pub /: *mut *mut __le16 fat_length; / sectors/FAT,
    pub /: *mut *mut __le16 secs_track; / sectors per track,
    pub /: *mut *mut __le16 heads; / number of heads,
    pub /: *mut *mut __le32 hidden; / hidden sectors (unused),
    pub /: *mut *mut __le32 total_sect; / number of sectors (if sectors == 0),
// Extended BPB Fields for FAT16
    pub /: *mut *mut __u8 drive_number; / Physical drive number,
    pub used: *mut *mut __u8 state; / undocumented, but,
    pub /: *mut *mut __u8 signature; / extended boot signature,
    pub /: *mut *mut __u8 vol_id[4]; / volume ID,
    pub /: *mut *mut __u8 vol_label[MSDOS_NAME]; / volume label,
    pub /: *mut *mut __u8 fs_type[8]; / file system type,
// other fields are not added here
    pub fat16: },
// only used by FAT32
    pub /: *mut *mut __le32 length; / sectors/FAT,
    pub mirroring,: *mut *mut __le16 flags; / bit 8: fat,
    pub filesystem: *mut *mut __u8 version[2]; / major, minor,
    pub in: *mut *mut __le32 root_cluster; / first cluster,
    pub /: *mut *mut __le16 info_sector; / filesystem info sector,
    pub /: *mut *mut __le16 backup_boot; / backup boot sector,
    pub /: *mut *mut __le16 reserved2[6]; / Unused,
// Extended BPB Fields for FAT32
    pub /: *mut *mut __u8 drive_number; / Physical drive number,
    pub used: *mut *mut __u8 state; / undocumented, but,
    pub /: *mut *mut __u8 signature; / extended boot signature,
    pub /: *mut *mut __u8 vol_id[4]; / volume ID,
    pub /: *mut *mut __u8 vol_label[MSDOS_NAME]; / volume label,
    pub /: *mut *mut __u8 fs_type[8]; / file system type,
// other fields are not added here
    pub fat32: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fat_boot_fsinfo {
    pub /: *mut *mut __le32 signature1; / 0x41615252L,
    pub /: *mut *mut __le32 reserved1[120]; / Nothing as far as I can tell,
    pub /: *mut *mut __le32 signature2; / 0x61417272L,
    pub /: *mut *mut __le32 free_clusters; / Free cluster count. -1 if unknown,
    pub /: *mut *mut __le32 next_cluster; / Most recently allocated cluster,
    pub reserved2: [__le32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msdos_dir_entry {
    pub /: *mut *mut __u8 name[MSDOS_NAME];/ name and extension,
    pub /: *mut *mut __u8 attr; / attribute bits,
    pub /: *mut *mut __u8 lcase; / Case for base and extension,
    pub /: *mut *mut __u8 ctime_cs; / Creation time, centiseconds (0-199),
    pub /: *mut *mut __le16 ctime; / Creation time,
    pub /: *mut *mut __le16 cdate; / Creation date,
    pub /: *mut *mut __le16 adate; / Last access date,
    pub /: *mut *mut __le16 starthi; / High 16 bits of cluster in FAT32,
    pub /: *mut *mut __le16 time,date,start;/ time, date and first cluster,
    pub /: *mut *mut __le32 size; / file size (in bytes),
}

// Up to 13 characters of the name
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msdos_dir_slot {
    pub /: *mut *mut __u8 id; / sequence number for slot,
    pub /: *mut *mut __u8 name0_4[10]; / first 5 characters in name,
    pub /: *mut *mut __u8 attr; / attribute byte,
    pub /: *mut *mut __u8 reserved; / always 0,
    pub /: *mut *mut __u8 alias_checksum; / checksum for 8.3 alias,
    pub /: *mut *mut __u8 name5_10[12]; / 6 more characters in name,
    pub /: *mut *mut __le16 start; / starting cluster number, 0 in long slots,
    pub /: *mut *mut __u8 name11_12[4]; / last 2 characters in name,
}
