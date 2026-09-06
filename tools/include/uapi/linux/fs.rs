//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/uapi/linux/fs.h
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
// This file has definitions for some important file table structures
// and constants and structures used by various generic file system
// ioctl's.  Please do not make any changes in this file before
// sending patches for review to linux-fsdevel@vger.kernel.org and
// linux-api@vger.kernel.org.
//

// Use of MS_* flags within the kernel is restricted to core mount(2) code.

//
// It's silly to have NR_OPEN bigger than NR_FILE, but you can change
// the file limit at runtime and only root can increase the per-process
// nr_file rlimit, so it's safe to set up a ridiculously high absolute
// upper limit on files-per-process.
//
// Some programs (notably those using select()) may have to be
// recompiled to take full advantage of the new limits..
//
// Fixed constants first:

pub const BLOCK_SIZE_BITS: c_int = 10;

// flags for integrity meta

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_clone_range {
    pub src_fd: __s64,
    pub src_offset: __u64,
    pub src_length: __u64,
    pub dest_offset: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fstrim_range {
    pub start: __u64,
    pub len: __u64,
    pub minlen: __u64,
}

//
// We include a length field because some filesystems (vfat) have an identifier
// that we do want to expose as a UUID, but doesn't have the standard length.
//
// We use a fixed size buffer beacuse this interface will, by fiat, never
// support "UUIDs" longer than 16 bytes; we don't want to force all downstream
// users to have to deal with that.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsuuid2 {
    pub len: __u8,
    pub uuid: [__u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_sysfs_path {
    pub len: __u8,
    pub name: [__u8; 128],
}

// extent-same (dedupe) ioctls; these MUST match the btrfs ioctl definitions
pub const FILE_DEDUPE_RANGE_SAME: c_int = 0;
pub const FILE_DEDUPE_RANGE_DIFFERS: c_int = 1;
// from struct btrfs_ioctl_file_extent_same_info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_dedupe_range_info {
    pub /: *mut *mut __s64 dest_fd; / in - destination file,
    pub /: *mut *mut __u64 dest_offset; / in - start of extent in destination,
    pub able: *mut *mut __u64 bytes_deduped; / out - total # of bytes we were,
// to dedupe from this file.
// status of this dedupe operation:
// < 0 for error
// == FILE_DEDUPE_RANGE_SAME if dedupe succeeds
// == FILE_DEDUPE_RANGE_DIFFERS if data differs
//
    pub /: *mut *mut __s32 status; / out - see above description,
    pub /: *mut *mut __u32 reserved; / must be zero,
}

// from struct btrfs_ioctl_file_extent_same_args
#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_dedupe_range {
    pub /: *mut *mut __u64 src_offset; / in - start of extent in source,
    pub /: *mut *mut __u64 src_length; / in - length of extent,
    pub /: *mut *mut __u16 dest_count; / in - total elements in info array,
    pub /: *mut *mut __u16 reserved1; / must be zero,
    pub /: *mut *mut __u32 reserved2; / must be zero,
    pub info: [file_dedupe_range_info; ],
}

// And dynamically-tunable limits and defaults:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct files_stat_struct {
    pub /: *mut *mut unsigned long nr_files; / read only,
    pub /: *mut *mut unsigned long nr_free_files; / read only,
    pub /: *mut *mut unsigned long max_files; / tunable,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inodes_stat_t {
    pub nr_inodes: c_long,
    pub nr_unused: c_long,
    pub /: *mut *mut long dummy[5]; / padding for sysctl ABI compatibility,
}

//
// Structure for FS_IOC_FSGETXATTR[A] and FS_IOC_FSSETXATTR.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsxattr {
    pub /: *mut *mut __u32 fsx_xflags; / xflags field value (get/set),
    pub (get/set)*/: *mut *mut __u32 fsx_extsize; / extsize field value,
    pub /: *mut *mut __u32 fsx_nextents; / nextents field value (get),
    pub /: *mut *mut __u32 fsx_projid; / project identifier (get/set),
    pub (get/set)*/: *mut *mut __u32 fsx_cowextsize; / CoW extsize field value,
    pub fsx_pad: [c_uchar; 8],
}

//
// Flags for the fsx_xflags field
//
pub const FS_XFLAG_REALTIME: c_uint = 0x00000001	/* data in realtime volume */;
pub const FS_XFLAG_PREALLOC: c_uint = 0x00000002	/* preallocated file extents */;
pub const FS_XFLAG_IMMUTABLE: c_uint = 0x00000008	/* file cannot be modified */;
pub const FS_XFLAG_APPEND: c_uint = 0x00000010	/* all writes append */;
pub const FS_XFLAG_SYNC: c_uint = 0x00000020	/* all writes synchronous */;
pub const FS_XFLAG_NOATIME: c_uint = 0x00000040	/* do not update access time */;
pub const FS_XFLAG_NODUMP: c_uint = 0x00000080	/* do not include in backups */;
pub const FS_XFLAG_RTINHERIT: c_uint = 0x00000100	/* create with rt bit set */;
pub const FS_XFLAG_PROJINHERIT: c_uint = 0x00000200	/* create with parents projid */;
pub const FS_XFLAG_NOSYMLINKS: c_uint = 0x00000400	/* disallow symlink creation */;
pub const FS_XFLAG_EXTSIZE: c_uint = 0x00000800	/* extent size allocator hint */;
pub const FS_XFLAG_EXTSZINHERIT: c_uint = 0x00001000	/* inherit inode extent size */;
pub const FS_XFLAG_NODEFRAG: c_uint = 0x00002000	/* do not defragment */;
pub const FS_XFLAG_FILESTREAM: c_uint = 0x00004000	/* use filestream allocator */;
pub const FS_XFLAG_DAX: c_uint = 0x00008000	/* use DAX for IO */;
pub const FS_XFLAG_COWEXTSIZE: c_uint = 0x00010000	/* CoW extent size allocator hint */;
pub const FS_XFLAG_HASATTR: c_uint = 0x80000000	/* no DIFLAG for this	*/;
// the read-only stuff doesn't really belong here, but any other place is

// Some people are morons.  Do not use sizeof!

// This was here just to show that the number is taken -

// A jump here: 108-111 have been used for various private purposes.

//
// A jump here: 130-136 are reserved for zoned block devices
// (see uapi/linux/blkzoned.h)
//

// Returns the external filesystem UUID, the same one blkid returns

//
// Returns the path component under /sys/fs/ that refers to this filesystem;
// also /sys/kernel/debug/ for filesystems with debugfs exports
//

//
// Inode flags (FS_IOC_GETFLAGS / FS_IOC_SETFLAGS)
//
// Note: for historical reasons, these flags were originally used and
// defined for use by ext2/ext3, and then other file systems started
// using these flags so they wouldn't need to write their own version
// of chattr/lsattr (which was shipped as part of e2fsprogs).  You
// should think twice before trying to use these flags in new
// contexts, or trying to assign these flags, since they are used both
// as the UAPI and the on-disk encoding for ext2/3/4.  Also, we are
// almost out of 32-bit flags.  :-)
//
// We have recently hoisted FS_IOC_FSGETXATTR / FS_IOC_FSSETXATTR from
// XFS to the generic FS level interface.  This uses a structure that
// has padding and hence has more room to grow, so it may be more
// appropriate for many new use cases.
//
// Please do not change these flags or interfaces before checking with
// linux-fsdevel@vger.kernel.org and linux-api@vger.kernel.org.
//
pub const FS_SECRM_FL: c_uint = 0x00000001 /* Secure deletion */;
pub const FS_UNRM_FL: c_uint = 0x00000002 /* Undelete */;
pub const FS_COMPR_FL: c_uint = 0x00000004 /* Compress file */;
pub const FS_SYNC_FL: c_uint = 0x00000008 /* Synchronous updates */;
pub const FS_IMMUTABLE_FL: c_uint = 0x00000010 /* Immutable file */;
pub const FS_APPEND_FL: c_uint = 0x00000020 /* writes to file may only append */;
pub const FS_NODUMP_FL: c_uint = 0x00000040 /* do not dump file */;
pub const FS_NOATIME_FL: c_uint = 0x00000080 /* do not update atime */;
// Reserved for compression usage...
pub const FS_DIRTY_FL: c_uint = 0x00000100;
pub const FS_COMPRBLK_FL: c_uint = 0x00000200 /* One or more compressed clusters */;
pub const FS_NOCOMP_FL: c_uint = 0x00000400 /* Don't compress */;
// End compression flags --- maybe not all used
pub const FS_ENCRYPT_FL: c_uint = 0x00000800 /* Encrypted file */;
pub const FS_BTREE_FL: c_uint = 0x00001000 /* btree format dir */;
pub const FS_INDEX_FL: c_uint = 0x00001000 /* hash-indexed directory */;
pub const FS_IMAGIC_FL: c_uint = 0x00002000 /* AFS directory */;
pub const FS_JOURNAL_DATA_FL: c_uint = 0x00004000 /* Reserved for ext3 */;
pub const FS_NOTAIL_FL: c_uint = 0x00008000 /* file tail should not be merged */;
pub const FS_DIRSYNC_FL: c_uint = 0x00010000 /* dirsync behaviour (directories only) */;
pub const FS_TOPDIR_FL: c_uint = 0x00020000 /* Top of directory hierarchies*/;
pub const FS_HUGE_FILE_FL: c_uint = 0x00040000 /* Reserved for ext4 */;
pub const FS_EXTENT_FL: c_uint = 0x00080000 /* Extents */;
pub const FS_VERITY_FL: c_uint = 0x00100000 /* Verity protected inode */;
pub const FS_EA_INODE_FL: c_uint = 0x00200000 /* Inode used for large EA */;
pub const FS_EOFBLOCKS_FL: c_uint = 0x00400000 /* Reserved for ext4 */;
pub const FS_NOCOW_FL: c_uint = 0x00800000 /* Do not cow file */;
pub const FS_DAX_FL: c_uint = 0x02000000 /* Inode is DAX */;
pub const FS_INLINE_DATA_FL: c_uint = 0x10000000 /* Reserved for ext4 */;
pub const FS_PROJINHERIT_FL: c_uint = 0x20000000 /* Create with parents projid */;
pub const FS_CASEFOLD_FL: c_uint = 0x40000000 /* Folder is case insensitive */;
pub const FS_RESERVED_FL: c_uint = 0x80000000 /* reserved for ext2 lib */;
pub const FS_FL_USER_VISIBLE: c_uint = 0x0003DFFF /* User visible flags */;
pub const FS_FL_USER_MODIFIABLE: c_uint = 0x000380FF /* User modifiable flags */;
pub const SYNC_FILE_RANGE_WAIT_BEFORE: c_int = 1;
pub const SYNC_FILE_RANGE_WRITE: c_int = 2;
pub const SYNC_FILE_RANGE_WAIT_AFTER: c_int = 4;

//
// Flags for preadv2/pwritev2:
//
pub type __kernel_rwf_t = int ;
// high priority request, poll if possible

// per-IO O_DSYNC

// per-IO O_SYNC

// per-IO, return -EAGAIN if operation would block

// per-IO O_APPEND

// per-IO negation of O_APPEND

// Atomic Write

// buffered IO that drops the cache after reading or writing data

// mask of flags supported by the kernel

// Pagemap ioctl

// Bitmasks provided in pm_scan_args masks and reported in page_region.categories.

//
// struct page_region - Page region with flags
// @start:	Start of the region
// @end:	End of the region (exclusive)
// @categories:	PAGE_IS_* category bitmask for the region
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct page_region {
    pub start: __u64,
    pub end: __u64,
    pub categories: __u64,
}

// Flags for PAGEMAP_SCAN ioctl

//
// struct pm_scan_arg - Pagemap ioctl argument
// @size:		Size of the structure
// @flags:		Flags for the IOCTL
// @start:		Starting address of the region
// @end:		Ending address of the region
// @walk_end		Address where the scan stopped (written by kernel).
// walk_end == end (address tags cleared) informs that the scan completed on entire range.
// @vec:		Address of page_region struct array for output
// @vec_len:		Length of the page_region struct array
// @max_pages:		Optional limit for number of returned pages (0 = disabled)
// @category_inverted:	PAGE_IS_* categories which values match if 0 instead of 1
// @category_mask:	Skip pages for which any category doesn't match
// @category_anyof_mask: Skip pages for which no category matches
// @return_mask:	PAGE_IS_* categories that are to be reported in `page_region`s returned
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm_scan_arg {
    pub size: __u64,
    pub flags: __u64,
    pub start: __u64,
    pub end: __u64,
    pub walk_end: __u64,
    pub vec: __u64,
    pub vec_len: __u64,
    pub max_pages: __u64,
    pub category_inverted: __u64,
    pub category_mask: __u64,
    pub category_anyof_mask: __u64,
    pub return_mask: __u64,
}

// /proc/<pid>/maps ioctl

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum procmap_query_flags {
//
// VMA permission flags.
//
// Can be used as part of procmap_query.query_flags field to look up
// only VMAs satisfying specified subset of permissions. E.g., specifying
// PROCMAP_QUERY_VMA_READABLE only will return both readable and read/write VMAs,
// while having PROCMAP_QUERY_VMA_READABLE | PROCMAP_QUERY_VMA_WRITABLE will only
// return read/write VMAs, though both executable/non-executable and
// private/shared will be ignored.
//
// PROCMAP_QUERY_VMA_* flags are also returned in procmap_query.vma_flags
// field to specify actual VMA permissions.
//
    PROCMAP_QUERY_VMA_READABLE		= 0x01,
    PROCMAP_QUERY_VMA_WRITABLE		= 0x02,
    PROCMAP_QUERY_VMA_EXECUTABLE		= 0x04,
    PROCMAP_QUERY_VMA_SHARED		= 0x08,
//
// Query modifier flags.
//
// By default VMA that covers provided address is returned, or -ENOENT
// is returned. With PROCMAP_QUERY_COVERING_OR_NEXT_VMA flag set, closest
// VMA with vma_start > addr will be returned if no covering VMA is
// found.
//
// PROCMAP_QUERY_FILE_BACKED_VMA instructs query to consider only VMAs that
// have file backing. Can be combined with PROCMAP_QUERY_COVERING_OR_NEXT_VMA
// to iterate all VMAs with file backing.
//
    PROCMAP_QUERY_COVERING_OR_NEXT_VMA	= 0x10,
    PROCMAP_QUERY_FILE_BACKED_VMA		= 0x20,
}

//
// Input/output argument structured passed into ioctl() call. It can be used
// to query a set of VMAs (Virtual Memory Areas) of a process.
//
// Each field can be one of three kinds, marked in a short comment to the
// right of the field:
// - "in", input argument, user has to provide this value, kernel doesn't modify it;
// - "out", output argument, kernel sets this field with VMA data;
// - "in/out", input and output argument; user provides initial value (used
// to specify maximum allowable buffer size), and kernel sets it to actual
// amount of data written (or zero, if there is no data).
//
// If matching VMA is found (according to criterias specified by
// query_addr/query_flags, all the out fields are filled out, and ioctl()
// returns 0. If there is no matching VMA, -ENOENT will be returned.
// In case of any other error, negative error code other than -ENOENT is
// returned.
//
// Most of the data is similar to the one returned as text in /proc/<pid>/maps
// file, but procmap_query provides more querying flexibility. There are no
// consistency guarantees between subsequent ioctl() calls, but data returned
// for matched VMA is self-consistent.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct procmap_query {
// Query struct size, for backwards/forward compatibility
    pub size: __u64,
//
// Query flags, a combination of enum procmap_query_flags values.
// Defines query filtering and behavior, see enum procmap_query_flags.
//
// Input argument, provided by user. Kernel doesn't modify it.
//
    pub /: *mut *mut __u64 query_flags; / in,
//
// Query address. By default, VMA that covers this address will
// be looked up. PROCMAP_QUERY_* flags above modify this default
// behavior further.
//
// Input argument, provided by user. Kernel doesn't modify it.
//
    pub /: *mut *mut __u64 query_addr; / in,
// VMA starting (inclusive) and ending (exclusive) address, if VMA is found.
    pub /: *mut *mut __u64 vma_start; / out,
    pub /: *mut *mut __u64 vma_end; / out,
// VMA permissions flags. A combination of PROCMAP_QUERY_VMA_* flags.
    pub /: *mut *mut __u64 vma_flags; / out,
// VMA backing page size granularity.
    pub /: *mut *mut __u64 vma_page_size; / out,
//
// VMA file offset. If VMA has file backing, this specifies offset
// within the file that VMA's start address corresponds to.
// Is set to zero if VMA has no backing file.
//
    pub /: *mut *mut __u64 vma_offset; / out,
// Backing file's inode number, or zero, if VMA has no backing file.
    pub /: *mut *mut __u64 inode; / out,
// Backing file's device major/minor number, or zero, if VMA has no backing file.
    pub /: *mut *mut __u32 dev_major; / out,
    pub /: *mut *mut __u32 dev_minor; / out,
//
// If set to non-zero value, signals the request to return VMA name
// (i.e., VMA's backing file's absolute path, with " (deleted)" suffix
// appended, if file was unlinked from FS) for matched VMA. VMA name
// can also be some special name (e.g., "[heap]", "[stack]") or could
// be even user-supplied with prctl(PR_SET_VMA, PR_SET_VMA_ANON_NAME).
//
// Kernel will set this field to zero, if VMA has no associated name.
// Otherwise kernel will return actual amount of bytes filled in
// user-supplied buffer (see vma_name_addr field below), including the
// terminating zero.
//
// If VMA name is longer that user-supplied maximum buffer size,
// -E2BIG error is returned.
//
// If this field is set to non-zero value, vma_name_addr should point
// to valid user space memory buffer of at least vma_name_size bytes.
// If set to zero, vma_name_addr should be set to zero as well
//
    pub /: *mut *mut __u32 vma_name_size; / in/out,
//
// If set to non-zero value, signals the request to extract and return
// VMA's backing file's build ID, if the backing file is an ELF file
// and it contains embedded build ID.
//
// Kernel will set this field to zero, if VMA has no backing file,
// backing file is not an ELF file, or ELF file has no build ID
// embedded.
//
// Build ID is a binary value (not a string). Kernel will set
// build_id_size field to exact number of bytes used for build ID.
// If build ID is requested and present, but needs more bytes than
// user-supplied maximum buffer size (see build_id_addr field below),
// -E2BIG error will be returned.
//
// If this field is set to non-zero value, build_id_addr should point
// to valid user space memory buffer of at least build_id_size bytes.
// If set to zero, build_id_addr should be set to zero as well
//
    pub /: *mut *mut __u32 build_id_size; / in/out,
//
// User-supplied address of a buffer of at least vma_name_size bytes
// for kernel to fill with matched VMA's name (see vma_name_size field
// description above for details).
//
// Should be set to zero if VMA name should not be returned.
//
    pub /: *mut *mut __u64 vma_name_addr; / in,
//
// User-supplied address of a buffer of at least build_id_size bytes
// for kernel to fill with matched VMA's ELF build ID, if available
// (see build_id_size field description above for details).
//
// Should be set to zero if build ID should not be returned.
//
    pub /: *mut *mut __u64 build_id_addr; / in,
}
