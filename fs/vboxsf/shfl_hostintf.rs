//! Automatically rewritten from C Header to Rust Module
//! Source: fs/vboxsf/shfl_hostintf.h
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


// SPDX-License-Identifier: MIT
//
// VirtualBox Shared Folders: host interface definition.
//
// Copyright (C) 2006-2018 Oracle Corporation
//

// The max in/out buffer size for a FN_READ or FN_WRITE call

//
// Structures shared between guest and the service
// can be relocated and use offsets to point to variable
// length parts.
//
// Shared folders protocol works with handles.
// Before doing any action on a file system object,
// one have to obtain the object handle via a SHFL_FN_CREATE
// request. A handle must be closed with SHFL_FN_CLOSE.
//
// Note function number 10 is not used!
// Root handles for a mapping are of type u32, Root handles are unique.

// Shared folders handle for an opened object are of type u64.

// Hardcoded maximum length (in chars) of a shared folder name.

// Hardcoded maximum number of shared folder mapping available to the guest.

// Shared folder string buffer structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shfl_string {
// Allocated size of the string member in bytes.
    pub size: u16,
// Length of string without trailing nul in bytes.
    pub length: u16,
// UTF-8 or UTF-16 string. Nul terminated.
    pub legacy_padding: [u8; 2],
    pub utf8): DECLARE_FLEX_ARRAY(u8,,
    pub utf16): DECLARE_FLEX_ARRAY(u16,,
    pub string: },
}

// The size of shfl_string w/o the string part.
pub const SHFLSTRING_HEADER_SIZE: c_int = 4;
// Calculate size of the string.
// Set user id on execution (S_ISUID).

// Set group id on execution (S_ISGID).

// Sticky bit (S_ISVTX / S_ISTXT).

// Owner readable (S_IRUSR).

// Owner writable (S_IWUSR).

// Owner executable (S_IXUSR).

// Group readable (S_IRGRP).

// Group writable (S_IWGRP).

// Group executable (S_IXGRP).

// Other readable (S_IROTH).

// Other writable (S_IWOTH).

// Other executable (S_IXOTH).

// Named pipe (fifo) (S_IFIFO).

// Character device (S_IFCHR).

// Directory (S_IFDIR).

// Block device (S_IFBLK).

// Regular file (S_IFREG).

// Symbolic link (S_IFLNK).

// Socket (S_IFSOCK).

// Whiteout (S_IFWHT).

// Type mask (S_IFMT).

// Checks the mode flags indicate a directory (S_ISDIR).

// Checks the mode flags indicate a symbolic link (S_ISLNK).

// The available additional information in a shfl_fsobjattr object.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum shfl_fsobjattr_add {
// No additional information is available / requested.
    SHFLFSOBJATTRADD_NOTHING = 1,
//
// The additional unix attributes (shfl_fsobjattr::u::unix_attr) are
// available / requested.
//
    SHFLFSOBJATTRADD_UNIX,
//
// The additional extended attribute size (shfl_fsobjattr::u::size) is
// available / requested.
//
    SHFLFSOBJATTRADD_EASIZE,
//
// The last valid item (inclusive).
// The valid range is SHFLFSOBJATTRADD_NOTHING thru
// SHFLFSOBJATTRADD_LAST.
//
    SHFLFSOBJATTRADD_LAST = SHFLFSOBJATTRADD_EASIZE,

// The usual 32-bit hack.
    SHFLFSOBJATTRADD_32BIT_SIZE_HACK = 0x7fffffff
}

//
// Additional unix Attributes, these are available when
// shfl_fsobjattr.additional == SHFLFSOBJATTRADD_UNIX.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shfl_fsobjattr_unix {
//
// The user owning the filesystem object (st_uid).
// This field is ~0U if not supported.
//
    pub uid: u32,
//
// The group the filesystem object is assigned (st_gid).
// This field is ~0U if not supported.
//
    pub gid: u32,
//
// Number of hard links to this filesystem object (st_nlink).
// This field is 1 if the filesystem doesn't support hardlinking or
// the information isn't available.
//
    pub hardlinks: u32,
//
// The device number of the device which this filesystem object resides
// on (st_dev). This field is 0 if this information is not available.
//
    pub inode_id_device: u32,
//
// The unique identifier (within the filesystem) of this filesystem
// object (st_ino). Together with inode_id_device, this field can be
// used as a OS wide unique id, when both their values are not 0.
// This field is 0 if the information is not available.
//
    pub inode_id: u64,
//
// User flags (st_flags).
// This field is 0 if this information is not available.
//
    pub flags: u32,
//
// The current generation number (st_gen).
// This field is 0 if this information is not available.
//
    pub generation_id: u32,
//
// The device number of a char. or block device type object (st_rdev).
// This field is 0 if the file isn't a char. or block device or when
// the OS doesn't use the major+minor device idenfication scheme.
//
    pub device: u32,
    pub __packed: },
// Extended attribute size.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shfl_fsobjattr_easize {
// Size of EAs.
    pub cb: i64,
    pub __packed: },
// Shared folder filesystem object attributes.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shfl_fsobjattr {
// Mode flags (st_mode). SHFL_UNIX_*, SHFL_TYPE_*, and SHFL_DOS_*.
    pub mode: u32,
// The additional attributes available.
    pub additional: shfl_fsobjattr_add,
//
// Additional attributes.
//
// Unless explicitly specified to an API, the API can provide additional
// data as it is provided by the underlying OS.
//
    pub unix_attr: shfl_fsobjattr_unix,
    pub size: shfl_fsobjattr_easize,
    pub u: } __packed,
    pub __packed: },
    pub 44): VMMDEV_ASSERT_SIZE(shfl_fsobjattr,,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shfl_timespec {
    pub ns_relative_to_unix_epoch: i64,
}

// Filesystem object information structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shfl_fsobjinfo {
//
// Logical size (st_size).
// For normal files this is the size of the file.
// For symbolic links, this is the length of the path name contained
// in the symbolic link.
// For other objects this fields needs to be specified.
//
    pub size: i64,
// Disk allocation size (st_blocks * DEV_BSIZE).
    pub allocated: i64,
// Time of last access (st_atime).
    pub access_time: shfl_timespec,
// Time of last data modification (st_mtime).
    pub modification_time: shfl_timespec,
//
// Time of last status change (st_ctime).
// If not available this is set to modification_time.
//
    pub change_time: shfl_timespec,
//
// Time of file birth (st_birthtime).
// If not available this is set to change_time.
//
    pub birth_time: shfl_timespec,
// Attributes.
    pub attr: shfl_fsobjattr,
    pub __packed: },
    pub 92): VMMDEV_ASSERT_SIZE(shfl_fsobjinfo,,
//
// result of an open/create request.
// Along with handle value the result code
// identifies what has happened while
// trying to open the object.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum shfl_create_result {
    SHFL_NO_RESULT,
// Specified path does not exist.
    SHFL_PATH_NOT_FOUND,
// Path to file exists, but the last component does not.
    SHFL_FILE_NOT_FOUND,
// File already exists and either has been opened or not.
    SHFL_FILE_EXISTS,
// New file was created.
    SHFL_FILE_CREATED,
// Existing file was replaced or overwritten.
    SHFL_FILE_REPLACED
}

// No flags. Initialization value.

//
// Only lookup the object, do not return a handle. When this is set all other
// flags are ignored.
//

//
// Open parent directory of specified object.
// Useful for the corresponding Windows FSD flag
// and for opening paths like \\dir\\*.* to search the 'dir'.
//

// Create/open a directory.

//
// Open/create action to do if object exists
// and if the object does not exists.
// REPLACE file means atomically DELETE and CREATE.
// OVERWRITE file means truncating the file to 0 and
// setting new size.
// When opening an existing directory REPLACE and OVERWRITE
// actions are considered invalid, and cause returning
// FILE_EXISTS with NIL handle.
//

// What to do if object exists.

// What to do if object does not exist.

// Read/write requested access for the object.

// No access requested.

// Read access requested.

// Write access requested.

// Read/Write access requested.

// Requested share access for the object.

// Allow any access.

// Do not allow read.

// Do not allow write.

// Do not allow access.

// Requested access to attributes of the object.

// No access requested.

// Read access requested.

// Write access requested.

// Read/Write access requested.

//
// The file is opened in append mode.
// Ignored if SHFL_CF_ACCESS_WRITE is not set.
//

// Create parameters buffer struct for SHFL_FN_CREATE call
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shfl_createparms {
// Returned handle of opened object.
    pub handle: u64,
// Returned result of the operation
    pub result: shfl_create_result,
// SHFL_CF_*
    pub create_flags: u32,
//
// Attributes of object to create and
// returned actual attributes of opened/created object.
//
    pub info: shfl_fsobjinfo,
    pub __packed: },
// Shared Folder directory information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shfl_dirinfo {
// Full information about the object.
    pub info: shfl_fsobjinfo,
//
// The length of the short field (number of UTF16 chars).
// It is 16-bit for reasons of alignment.
//
    pub short_name_len: u16,
//
// The short name for 8.3 compatibility.
// Empty string if not available.
//
    pub short_name: [u16; 14],
    pub name: shfl_string,
}

// Shared folder filesystem properties.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shfl_fsproperties {
//
// The maximum size of a filesystem object name.
// This does not include the '\\0'.
//
    pub max_component_len: u32,
//
// True if the filesystem is remote.
// False if the filesystem is local.
//
    pub remote: bool,
//
// True if the filesystem is case sensitive.
// False if the filesystem is case insensitive.
//
    pub case_sensitive: bool,
//
// True if the filesystem is mounted read only.
// False if the filesystem is mounted read write.
//
    pub read_only: bool,
//
// True if the filesystem can encode unicode object names.
// False if it can't.
//
    pub supports_unicode: bool,
//
// True if the filesystem is compresses.
// False if it isn't or we don't know.
//
    pub compressed: bool,
//
// True if the filesystem compresses of individual files.
// False if it doesn't or we don't know.
//
    pub file_compression: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shfl_volinfo {
    pub total_allocation_bytes: i64,
    pub available_allocation_bytes: i64,
    pub bytes_per_allocation_unit: u32,
    pub bytes_per_sector: u32,
    pub serial: u32,
    pub properties: shfl_fsproperties,
}

// SHFL_FN_MAP_FOLDER Parameters structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shfl_map_folder {
//
// pointer, in:
// Points to struct shfl_string buffer.
//
    pub path: vmmdev_hgcm_function_parameter,
//
// pointer, out: SHFLROOT (u32)
// Root handle of the mapping which name is queried.
//
    pub root: vmmdev_hgcm_function_parameter,
//
// pointer, in: UTF16
// Path delimiter
//
    pub delimiter: vmmdev_hgcm_function_parameter,
//
// pointer, in: SHFLROOT (u32)
// Case senstive flag
//
    pub case_sensitive: vmmdev_hgcm_function_parameter,
}

// Number of parameters

// SHFL_FN_UNMAP_FOLDER Parameters structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shfl_unmap_folder {
//
// pointer, in: SHFLROOT (u32)
// Root handle of the mapping which name is queried.
//
    pub root: vmmdev_hgcm_function_parameter,
}

// Number of parameters

// SHFL_FN_CREATE Parameters structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shfl_create {
//
// pointer, in: SHFLROOT (u32)
// Root handle of the mapping which name is queried.
//
    pub root: vmmdev_hgcm_function_parameter,
//
// pointer, in:
// Points to struct shfl_string buffer.
//
    pub path: vmmdev_hgcm_function_parameter,
//
// pointer, in/out:
// Points to struct shfl_createparms buffer.
//
    pub parms: vmmdev_hgcm_function_parameter,
}

// Number of parameters

// SHFL_FN_CLOSE Parameters structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shfl_close {
//
// pointer, in: SHFLROOT (u32)
// Root handle of the mapping which name is queried.
//
    pub root: vmmdev_hgcm_function_parameter,
//
// value64, in:
// SHFLHANDLE (u64) of object to close.
//
    pub handle: vmmdev_hgcm_function_parameter,
}

// Number of parameters

// SHFL_FN_READ Parameters structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shfl_read {
//
// pointer, in: SHFLROOT (u32)
// Root handle of the mapping which name is queried.
//
    pub root: vmmdev_hgcm_function_parameter,
//
// value64, in:
// SHFLHANDLE (u64) of object to read from.
//
    pub handle: vmmdev_hgcm_function_parameter,
//
// value64, in:
// Offset to read from.
//
    pub offset: vmmdev_hgcm_function_parameter,
//
// value64, in/out:
// Bytes to read/How many were read.
//
    pub cb: vmmdev_hgcm_function_parameter,
//
// pointer, out:
// Buffer to place data to.
//
    pub buffer: vmmdev_hgcm_function_parameter,
}

// Number of parameters

// SHFL_FN_WRITE Parameters structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shfl_write {
//
// pointer, in: SHFLROOT (u32)
// Root handle of the mapping which name is queried.
//
    pub root: vmmdev_hgcm_function_parameter,
//
// value64, in:
// SHFLHANDLE (u64) of object to write to.
//
    pub handle: vmmdev_hgcm_function_parameter,
//
// value64, in:
// Offset to write to.
//
    pub offset: vmmdev_hgcm_function_parameter,
//
// value64, in/out:
// Bytes to write/How many were written.
//
    pub cb: vmmdev_hgcm_function_parameter,
//
// pointer, in:
// Data to write.
//
    pub buffer: vmmdev_hgcm_function_parameter,
}

// Number of parameters

//
// SHFL_FN_LIST
// Listing information includes variable length RTDIRENTRY[EX] structures.
//
pub const SHFL_LIST_NONE: c_int = 0;
pub const SHFL_LIST_RETURN_ONE: c_int = 1;
// SHFL_FN_LIST Parameters structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shfl_list {
//
// pointer, in: SHFLROOT (u32)
// Root handle of the mapping which name is queried.
//
    pub root: vmmdev_hgcm_function_parameter,
//
// value64, in:
// SHFLHANDLE (u64) of object to be listed.
//
    pub handle: vmmdev_hgcm_function_parameter,
//
// value32, in:
// List flags SHFL_LIST_*.
//
    pub flags: vmmdev_hgcm_function_parameter,
//
// value32, in/out:
// Bytes to be used for listing information/How many bytes were used.
//
    pub cb: vmmdev_hgcm_function_parameter,
//
// pointer, in/optional
// Points to struct shfl_string buffer that specifies a search path.
//
    pub path: vmmdev_hgcm_function_parameter,
//
// pointer, out:
// Buffer to place listing information to. (struct shfl_dirinfo)
//
    pub buffer: vmmdev_hgcm_function_parameter,
//
// value32, in/out:
// Indicates a key where the listing must be resumed.
// in: 0 means start from begin of object.
// out: 0 means listing completed.
//
    pub resume_point: vmmdev_hgcm_function_parameter,
//
// pointer, out:
// Number of files returned
//
    pub file_count: vmmdev_hgcm_function_parameter,
}

// Number of parameters

// SHFL_FN_READLINK Parameters structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shfl_readLink {
//
// pointer, in: SHFLROOT (u32)
// Root handle of the mapping which name is queried.
//
    pub root: vmmdev_hgcm_function_parameter,
//
// pointer, in:
// Points to struct shfl_string buffer.
//
    pub path: vmmdev_hgcm_function_parameter,
//
// pointer, out:
// Buffer to place data to.
//
    pub buffer: vmmdev_hgcm_function_parameter,
}

// Number of parameters

// SHFL_FN_INFORMATION
// Mask of Set/Get bit.

// Get information

// Set information

// Get name of the object.

// Set size of object (extend/trucate); only applies to file objects

// Get/Set file object info.

// Get volume information.

// SHFL_FN_INFORMATION Parameters structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shfl_information {
//
// pointer, in: SHFLROOT (u32)
// Root handle of the mapping which name is queried.
//
    pub root: vmmdev_hgcm_function_parameter,
//
// value64, in:
// SHFLHANDLE (u64) of object to be listed.
//
    pub handle: vmmdev_hgcm_function_parameter,
//
// value32, in:
// SHFL_INFO_
//
    pub flags: vmmdev_hgcm_function_parameter,
//
// value32, in/out:
// Bytes to be used for information/How many bytes were used.
//
    pub cb: vmmdev_hgcm_function_parameter,
//
// pointer, in/out:
// Information to be set/get (shfl_fsobjinfo or shfl_string). Do not
// forget to set the shfl_fsobjinfo::attr::additional for a get
// operation as well.
//
    pub info: vmmdev_hgcm_function_parameter,
}

// Number of parameters

// SHFL_FN_REMOVE

// SHFL_FN_REMOVE Parameters structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shfl_remove {
//
// pointer, in: SHFLROOT (u32)
// Root handle of the mapping which name is queried.
//
    pub root: vmmdev_hgcm_function_parameter,
//
// pointer, in:
// Points to struct shfl_string buffer.
//
    pub path: vmmdev_hgcm_function_parameter,
//
// value32, in:
// remove flags (file/directory)
//
    pub flags: vmmdev_hgcm_function_parameter,
}

// SHFL_FN_RENAME

// SHFL_FN_RENAME Parameters structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shfl_rename {
//
// pointer, in: SHFLROOT (u32)
// Root handle of the mapping which name is queried.
//
    pub root: vmmdev_hgcm_function_parameter,
//
// pointer, in:
// Points to struct shfl_string src.
//
    pub src: vmmdev_hgcm_function_parameter,
//
// pointer, in:
// Points to struct shfl_string dest.
//
    pub dest: vmmdev_hgcm_function_parameter,
//
// value32, in:
// rename flags (file/directory)
//
    pub flags: vmmdev_hgcm_function_parameter,
}

// SHFL_FN_SYMLINK Parameters structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shfl_symlink {
//
// pointer, in: SHFLROOT (u32)
// Root handle of the mapping which name is queried.
//
    pub root: vmmdev_hgcm_function_parameter,
//
// pointer, in:
// Points to struct shfl_string of path for the new symlink.
//
    pub new_path: vmmdev_hgcm_function_parameter,
//
// pointer, in:
// Points to struct shfl_string of destination for symlink.
//
    pub old_path: vmmdev_hgcm_function_parameter,
//
// pointer, out:
// Information about created symlink.
//
    pub info: vmmdev_hgcm_function_parameter,
}

