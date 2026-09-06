//! Automatically rewritten from C Header to Rust Module
//! Source: security/tomoyo/common.h
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
// security/tomoyo/common.h
//
// Header file for TOMOYO.
//
// Copyright (C) 2005-2011  NTT DATA CORPORATION
//

// Constants definitions.
//
// TOMOYO uses this hash only when appending a string into the string
// table. Frequency of appending strings is very low. So we don't need
// large (e.g. 64k) hash size. 256 will be sufficient.
//
pub const TOMOYO_HASH_BITS: c_int = 8;

//
// TOMOYO checks only SOCK_STREAM, SOCK_DGRAM, SOCK_RAW, SOCK_SEQPACKET.
// Therefore, we don't need SOCK_MAX.
//
pub const TOMOYO_SOCK_MAX: c_int = 6;
pub const TOMOYO_EXEC_TMPSIZE: c_int = 4096;
// Garbage collector is trying to kfree() this element.

// Profile number is an integer between 0 and 255.
pub const TOMOYO_MAX_PROFILES: c_int = 256;
// Group number is an integer between 0 and 255.
pub const TOMOYO_MAX_ACL_GROUPS: c_int = 256;
// Index numbers for "struct tomoyo_condition".
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tomoyo_conditions_index {
    TOMOYO_TASK_UID,             /* current_uid()   */
    TOMOYO_TASK_EUID,            /* current_euid()  */
    TOMOYO_TASK_SUID,            /* current_suid()  */
    TOMOYO_TASK_FSUID,           /* current_fsuid() */
    TOMOYO_TASK_GID,             /* current_gid()   */
    TOMOYO_TASK_EGID,            /* current_egid()  */
    TOMOYO_TASK_SGID,            /* current_sgid()  */
    TOMOYO_TASK_FSGID,           /* current_fsgid() */
    TOMOYO_TASK_PID,             /* sys_getpid()   */
    TOMOYO_TASK_PPID,            /* sys_getppid()  */
    TOMOYO_EXEC_ARGC,            /* "struct linux_binprm *"->argc */
    TOMOYO_EXEC_ENVC,            /* "struct linux_binprm *"->envc */
    TOMOYO_TYPE_IS_SOCKET,       /* S_IFSOCK */
    TOMOYO_TYPE_IS_SYMLINK,      /* S_IFLNK */
    TOMOYO_TYPE_IS_FILE,         /* S_IFREG */
    TOMOYO_TYPE_IS_BLOCK_DEV,    /* S_IFBLK */
    TOMOYO_TYPE_IS_DIRECTORY,    /* S_IFDIR */
    TOMOYO_TYPE_IS_CHAR_DEV,     /* S_IFCHR */
    TOMOYO_TYPE_IS_FIFO,         /* S_IFIFO */
    TOMOYO_MODE_SETUID,          /* S_ISUID */
    TOMOYO_MODE_SETGID,          /* S_ISGID */
    TOMOYO_MODE_STICKY,          /* S_ISVTX */
    TOMOYO_MODE_OWNER_READ,      /* S_IRUSR */
    TOMOYO_MODE_OWNER_WRITE,     /* S_IWUSR */
    TOMOYO_MODE_OWNER_EXECUTE,   /* S_IXUSR */
    TOMOYO_MODE_GROUP_READ,      /* S_IRGRP */
    TOMOYO_MODE_GROUP_WRITE,     /* S_IWGRP */
    TOMOYO_MODE_GROUP_EXECUTE,   /* S_IXGRP */
    TOMOYO_MODE_OTHERS_READ,     /* S_IROTH */
    TOMOYO_MODE_OTHERS_WRITE,    /* S_IWOTH */
    TOMOYO_MODE_OTHERS_EXECUTE,  /* S_IXOTH */
    TOMOYO_EXEC_REALPATH,
    TOMOYO_SYMLINK_TARGET,
    TOMOYO_PATH1_UID,
    TOMOYO_PATH1_GID,
    TOMOYO_PATH1_INO,
    TOMOYO_PATH1_MAJOR,
    TOMOYO_PATH1_MINOR,
    TOMOYO_PATH1_PERM,
    TOMOYO_PATH1_TYPE,
    TOMOYO_PATH1_DEV_MAJOR,
    TOMOYO_PATH1_DEV_MINOR,
    TOMOYO_PATH2_UID,
    TOMOYO_PATH2_GID,
    TOMOYO_PATH2_INO,
    TOMOYO_PATH2_MAJOR,
    TOMOYO_PATH2_MINOR,
    TOMOYO_PATH2_PERM,
    TOMOYO_PATH2_TYPE,
    TOMOYO_PATH2_DEV_MAJOR,
    TOMOYO_PATH2_DEV_MINOR,
    TOMOYO_PATH1_PARENT_UID,
    TOMOYO_PATH1_PARENT_GID,
    TOMOYO_PATH1_PARENT_INO,
    TOMOYO_PATH1_PARENT_PERM,
    TOMOYO_PATH2_PARENT_UID,
    TOMOYO_PATH2_PARENT_GID,
    TOMOYO_PATH2_PARENT_INO,
    TOMOYO_PATH2_PARENT_PERM,
    TOMOYO_MAX_CONDITION_KEYWORD,
    TOMOYO_NUMBER_UNION,
    TOMOYO_NAME_UNION,
    TOMOYO_ARGV_ENTRY,
    TOMOYO_ENVP_ENTRY,
}

// Index numbers for stat().
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tomoyo_path_stat_index {
// Do not change this order.
    TOMOYO_PATH1,
    TOMOYO_PATH1_PARENT,
    TOMOYO_PATH2,
    TOMOYO_PATH2_PARENT,
    TOMOYO_MAX_PATH_STAT
}

// Index numbers for operation mode.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tomoyo_mode_index {
    TOMOYO_CONFIG_DISABLED,
    TOMOYO_CONFIG_LEARNING,
    TOMOYO_CONFIG_PERMISSIVE,
    TOMOYO_CONFIG_ENFORCING,
    TOMOYO_CONFIG_MAX_MODE,
    TOMOYO_CONFIG_WANT_REJECT_LOG =  64,
    TOMOYO_CONFIG_WANT_GRANT_LOG  = 128,
    TOMOYO_CONFIG_USE_DEFAULT     = 255,
}

// Index numbers for entry type.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tomoyo_policy_id {
    TOMOYO_ID_GROUP,
    TOMOYO_ID_ADDRESS_GROUP,
    TOMOYO_ID_PATH_GROUP,
    TOMOYO_ID_NUMBER_GROUP,
    TOMOYO_ID_TRANSITION_CONTROL,
    TOMOYO_ID_AGGREGATOR,
    TOMOYO_ID_MANAGER,
    TOMOYO_ID_CONDITION,
    TOMOYO_ID_NAME,
    TOMOYO_ID_ACL,
    TOMOYO_ID_DOMAIN,
    TOMOYO_MAX_POLICY
}

// Index numbers for domain's attributes.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tomoyo_domain_info_flags_index {
// Quota warnning flag.
    TOMOYO_DIF_QUOTA_WARNED,
//
// This domain was unable to create a new domain at
// tomoyo_find_next_domain() because the name of the domain to be
// created was too long or it could not allocate memory.
// More than one process continued execve() without domain transition.
//
    TOMOYO_DIF_TRANSITION_FAILED,
    TOMOYO_MAX_DOMAIN_INFO_FLAGS
}

// Index numbers for audit type.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tomoyo_grant_log {
// Follow profile's configuration.
    TOMOYO_GRANTLOG_AUTO,
// Do not generate grant log.
    TOMOYO_GRANTLOG_NO,
// Generate grant_log.
    TOMOYO_GRANTLOG_YES,
}

// Index numbers for group entries.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tomoyo_group_id {
    TOMOYO_PATH_GROUP,
    TOMOYO_NUMBER_GROUP,
    TOMOYO_ADDRESS_GROUP,
    TOMOYO_MAX_GROUP
}

// Index numbers for type of numeric values.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tomoyo_value_type {
    TOMOYO_VALUE_TYPE_INVALID,
    TOMOYO_VALUE_TYPE_DECIMAL,
    TOMOYO_VALUE_TYPE_OCTAL,
    TOMOYO_VALUE_TYPE_HEXADECIMAL,
}

// Index numbers for domain transition control keywords.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tomoyo_transition_type {
// Do not change this order,
    TOMOYO_TRANSITION_CONTROL_NO_RESET,
    TOMOYO_TRANSITION_CONTROL_RESET,
    TOMOYO_TRANSITION_CONTROL_NO_INITIALIZE,
    TOMOYO_TRANSITION_CONTROL_INITIALIZE,
    TOMOYO_TRANSITION_CONTROL_NO_KEEP,
    TOMOYO_TRANSITION_CONTROL_KEEP,
    TOMOYO_MAX_TRANSITION_TYPE
}

// Index numbers for Access Controls.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tomoyo_acl_entry_type_index {
    TOMOYO_TYPE_PATH_ACL,
    TOMOYO_TYPE_PATH2_ACL,
    TOMOYO_TYPE_PATH_NUMBER_ACL,
    TOMOYO_TYPE_MKDEV_ACL,
    TOMOYO_TYPE_MOUNT_ACL,
    TOMOYO_TYPE_INET_ACL,
    TOMOYO_TYPE_UNIX_ACL,
    TOMOYO_TYPE_ENV_ACL,
    TOMOYO_TYPE_MANUAL_TASK_ACL,
}

// Index numbers for access controls with one pathname.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tomoyo_path_acl_index {
    TOMOYO_TYPE_EXECUTE,
    TOMOYO_TYPE_READ,
    TOMOYO_TYPE_WRITE,
    TOMOYO_TYPE_APPEND,
    TOMOYO_TYPE_UNLINK,
    TOMOYO_TYPE_GETATTR,
    TOMOYO_TYPE_RMDIR,
    TOMOYO_TYPE_TRUNCATE,
    TOMOYO_TYPE_SYMLINK,
    TOMOYO_TYPE_CHROOT,
    TOMOYO_TYPE_UMOUNT,
    TOMOYO_MAX_PATH_OPERATION
}

// Index numbers for /sys/kernel/security/tomoyo/stat interface.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tomoyo_memory_stat_type {
    TOMOYO_MEMORY_POLICY,
    TOMOYO_MEMORY_AUDIT,
    TOMOYO_MEMORY_QUERY,
    TOMOYO_MAX_MEMORY_STAT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tomoyo_mkdev_acl_index {
    TOMOYO_TYPE_MKBLOCK,
    TOMOYO_TYPE_MKCHAR,
    TOMOYO_MAX_MKDEV_OPERATION
}

// Index numbers for socket operations.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tomoyo_network_acl_index {
    TOMOYO_NETWORK_BIND,    /* bind() operation. */
    TOMOYO_NETWORK_LISTEN,  /* listen() operation. */
    TOMOYO_NETWORK_CONNECT, /* connect() operation. */
    TOMOYO_NETWORK_SEND,    /* send() operation. */
    TOMOYO_MAX_NETWORK_OPERATION
}

// Index numbers for access controls with two pathnames.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tomoyo_path2_acl_index {
    TOMOYO_TYPE_LINK,
    TOMOYO_TYPE_RENAME,
    TOMOYO_TYPE_PIVOT_ROOT,
    TOMOYO_MAX_PATH2_OPERATION
}

// Index numbers for access controls with one pathname and one number.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tomoyo_path_number_acl_index {
    TOMOYO_TYPE_CREATE,
    TOMOYO_TYPE_MKDIR,
    TOMOYO_TYPE_MKFIFO,
    TOMOYO_TYPE_MKSOCK,
    TOMOYO_TYPE_IOCTL,
    TOMOYO_TYPE_CHMOD,
    TOMOYO_TYPE_CHOWN,
    TOMOYO_TYPE_CHGRP,
    TOMOYO_MAX_PATH_NUMBER_OPERATION
}

// Index numbers for /sys/kernel/security/tomoyo/ interfaces.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tomoyo_securityfs_interface_index {
    TOMOYO_DOMAINPOLICY,
    TOMOYO_EXCEPTIONPOLICY,
    TOMOYO_PROCESS_STATUS,
    TOMOYO_STAT,
    TOMOYO_AUDIT,
    TOMOYO_VERSION,
    TOMOYO_PROFILE,
    TOMOYO_QUERY,
    TOMOYO_MANAGER
}

// Index numbers for special mount operations.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tomoyo_special_mount {
    TOMOYO_MOUNT_BIND,            /* mount --bind /source /dest   */
    TOMOYO_MOUNT_MOVE,            /* mount --move /old /new       */
    TOMOYO_MOUNT_REMOUNT,         /* mount -o remount /dir        */
    TOMOYO_MOUNT_MAKE_UNBINDABLE, /* mount --make-unbindable /dir */
    TOMOYO_MOUNT_MAKE_PRIVATE,    /* mount --make-private /dir    */
    TOMOYO_MOUNT_MAKE_SLAVE,      /* mount --make-slave /dir      */
    TOMOYO_MOUNT_MAKE_SHARED,     /* mount --make-shared /dir     */
    TOMOYO_MAX_SPECIAL_MOUNT
}

// Index numbers for functionality.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tomoyo_mac_index {
    TOMOYO_MAC_FILE_EXECUTE,
    TOMOYO_MAC_FILE_OPEN,
    TOMOYO_MAC_FILE_CREATE,
    TOMOYO_MAC_FILE_UNLINK,
    TOMOYO_MAC_FILE_GETATTR,
    TOMOYO_MAC_FILE_MKDIR,
    TOMOYO_MAC_FILE_RMDIR,
    TOMOYO_MAC_FILE_MKFIFO,
    TOMOYO_MAC_FILE_MKSOCK,
    TOMOYO_MAC_FILE_TRUNCATE,
    TOMOYO_MAC_FILE_SYMLINK,
    TOMOYO_MAC_FILE_MKBLOCK,
    TOMOYO_MAC_FILE_MKCHAR,
    TOMOYO_MAC_FILE_LINK,
    TOMOYO_MAC_FILE_RENAME,
    TOMOYO_MAC_FILE_CHMOD,
    TOMOYO_MAC_FILE_CHOWN,
    TOMOYO_MAC_FILE_CHGRP,
    TOMOYO_MAC_FILE_IOCTL,
    TOMOYO_MAC_FILE_CHROOT,
    TOMOYO_MAC_FILE_MOUNT,
    TOMOYO_MAC_FILE_UMOUNT,
    TOMOYO_MAC_FILE_PIVOT_ROOT,
    TOMOYO_MAC_NETWORK_INET_STREAM_BIND,
    TOMOYO_MAC_NETWORK_INET_STREAM_LISTEN,
    TOMOYO_MAC_NETWORK_INET_STREAM_CONNECT,
    TOMOYO_MAC_NETWORK_INET_DGRAM_BIND,
    TOMOYO_MAC_NETWORK_INET_DGRAM_SEND,
    TOMOYO_MAC_NETWORK_INET_RAW_BIND,
    TOMOYO_MAC_NETWORK_INET_RAW_SEND,
    TOMOYO_MAC_NETWORK_UNIX_STREAM_BIND,
    TOMOYO_MAC_NETWORK_UNIX_STREAM_LISTEN,
    TOMOYO_MAC_NETWORK_UNIX_STREAM_CONNECT,
    TOMOYO_MAC_NETWORK_UNIX_DGRAM_BIND,
    TOMOYO_MAC_NETWORK_UNIX_DGRAM_SEND,
    TOMOYO_MAC_NETWORK_UNIX_SEQPACKET_BIND,
    TOMOYO_MAC_NETWORK_UNIX_SEQPACKET_LISTEN,
    TOMOYO_MAC_NETWORK_UNIX_SEQPACKET_CONNECT,
    TOMOYO_MAC_ENVIRON,
    TOMOYO_MAX_MAC_INDEX
}

// Index numbers for category of functionality.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tomoyo_mac_category_index {
    TOMOYO_MAC_CATEGORY_FILE,
    TOMOYO_MAC_CATEGORY_NETWORK,
    TOMOYO_MAC_CATEGORY_MISC,
    TOMOYO_MAX_MAC_CATEGORY_INDEX
}

//
// Retry this request. Returned by tomoyo_supervisor() if policy violation has
// occurred in enforcing mode and the userspace daemon decided to retry.
//
// We must choose a positive value in order to distinguish "granted" (which is
// 0) and "rejected" (which is a negative value) and "retry".
//
pub const TOMOYO_RETRY_REQUEST: c_int = 1;
// Index numbers for /sys/kernel/security/tomoyo/stat interface.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tomoyo_policy_stat_type {
// Do not change this order.
    TOMOYO_STAT_POLICY_UPDATES,
    TOMOYO_STAT_POLICY_LEARNING,   /* == TOMOYO_CONFIG_LEARNING */
    TOMOYO_STAT_POLICY_PERMISSIVE, /* == TOMOYO_CONFIG_PERMISSIVE */
    TOMOYO_STAT_POLICY_ENFORCING,  /* == TOMOYO_CONFIG_ENFORCING */
    TOMOYO_MAX_POLICY_STAT
}

// Index numbers for profile's PREFERENCE values.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tomoyo_pref_index {
    TOMOYO_PREF_MAX_AUDIT_LOG,
    TOMOYO_PREF_MAX_LEARNING_ENTRY,
    TOMOYO_MAX_PREF
}

// Structure definitions.
// Common header for holding ACL entries.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_acl_head {
    pub list: list_head,
    pub /: *mut *mut s8 is_deleted; / true or false or TOMOYO_GC_IN_PROGRESS,
    pub __packed: },
// Common header for shared entries.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_shared_acl_head {
    pub list: list_head,
    pub users: core::sync::atomic::AtomicI32,
    pub __packed: },
    pub tomoyo_policy_namespace: struct,
// Structure for request info.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_request_info {
//
// For holding parameters specific to operations which deal files.
// NULL if not dealing files.
//
    pub obj: *mut tomoyo_obj_info,
//
// For holding parameters specific to execve() request.
// NULL if not dealing execve().
//
    pub ee: *mut tomoyo_execve,
    pub domain: *mut tomoyo_domain_info,
// For holding parameters.
    pub filename: *const tomoyo_path_info,
// For using wildcards at tomoyo_find_next_domain().
    pub matched_path: *const tomoyo_path_info,
// One of values in "enum tomoyo_path_acl_index".
    pub operation: u8,
    pub path: },
    pub filename1: *const tomoyo_path_info,
    pub filename2: *const tomoyo_path_info,
// One of values in "enum tomoyo_path2_acl_index".
    pub operation: u8,
    pub path2: },
    pub filename: *const tomoyo_path_info,
    pub mode: c_uint,
    pub major: c_uint,
    pub minor: c_uint,
// One of values in "enum tomoyo_mkdev_acl_index".
    pub operation: u8,
    pub mkdev: },
    pub filename: *const tomoyo_path_info,
    pub number: c_ulong,
//
// One of values in
// "enum tomoyo_path_number_acl_index".
//
    pub operation: u8,
    pub path_number: },
    pub name: *const tomoyo_path_info,
    pub environ: },
    pub address: *const __be32,
    pub port: u16,
// One of values smaller than TOMOYO_SOCK_MAX.
    pub protocol: u8,
// One of values in "enum tomoyo_network_acl_index".
    pub operation: u8,
    pub is_ipv6: bool,
    pub inet_network: },
    pub address: *const tomoyo_path_info,
// One of values smaller than TOMOYO_SOCK_MAX.
    pub protocol: u8,
// One of values in "enum tomoyo_network_acl_index".
    pub operation: u8,
    pub unix_network: },
    pub type: *const tomoyo_path_info,
    pub dir: *const tomoyo_path_info,
    pub dev: *const tomoyo_path_info,
    pub flags: c_ulong,
    pub need_dev: c_int,
    pub mount: },
    pub domainname: *const tomoyo_path_info,
    pub task: },
    pub param: },
    pub matched_acl: *mut tomoyo_acl_info,
    pub param_type: u8,
    pub granted: bool,
    pub retry: u8,
    pub profile: u8,
    pub /: *mut *mut u8 mode; / One of tomoyo_mode_index .,
    pub type: u8,
}

// Structure for holding a token.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_path_info {
    pub name: *const c_char,
    pub /: *mut *mut u32 hash; / = full_name_hash(name, strlen(name)),
    pub /: *mut *mut u16 const_len; / = tomoyo_const_part_length(name),
    pub /: *mut *mut bool is_dir; / = tomoyo_strendswith(name, "/"),
    pub /: *mut *mut bool is_patterned; / = tomoyo_path_contains_pattern(name),
}

// Structure for holding string data.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_name {
    pub head: tomoyo_shared_acl_head,
    pub entry: tomoyo_path_info,
}

// Structure for holding a word.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_name_union {
// Either @filename or @group is NULL.
    pub filename: *const tomoyo_path_info,
    pub group: *mut tomoyo_group,
}

// Structure for holding a number.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_number_union {
    pub values: [c_ulong; 2],
    pub /: *mut *mut *mut tomoyo_group group; / Maybe NULL.,
// One of values in "enum tomoyo_value_type".
    pub value_type: [u8; 2],
}

// Structure for holding an IP address.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_ipaddr_union {
    pub /: *mut *mut in6_addr ip[2]; / Big endian.,
    pub /: *mut *mut *mut tomoyo_group group; / Pointer to address group.,
    pub /: *mut *mut bool is_ipv6; / Valid only if @group == NULL.,
}

// Structure for "path_group"/"number_group"/"address_group" directive.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_group {
    pub head: tomoyo_shared_acl_head,
    pub group_name: *const tomoyo_path_info,
    pub member_list: list_head,
}

// Structure for "path_group" directive.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_path_group {
    pub head: tomoyo_acl_head,
    pub member_name: *const tomoyo_path_info,
}

// Structure for "number_group" directive.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_number_group {
    pub head: tomoyo_acl_head,
    pub number: tomoyo_number_union,
}

// Structure for "address_group" directive.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_address_group {
    pub head: tomoyo_acl_head,
// Structure for holding an IP address.
    pub address: tomoyo_ipaddr_union,
}

// Subset of "struct stat". Used by conditional ACL and audit logs.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_mini_stat {
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub ino: u64,
    pub mode: umode_t,
    pub dev: dev_t,
    pub rdev: dev_t,
}

// Structure for dumping argv[] and envp[] of "struct linux_binprm".
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_page_dump {
    pub /: *mut *mut *mut page page; / Previously dumped page.,
    pub /: *mut *mut *mut char data; / Contents of "page". Size is PAGE_SIZE.,
}

// Structure for attribute checks in addition to pathname checks.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_obj_info {
//
// True if tomoyo_get_attributes() was already called, false otherwise.
//
    pub validate_done: bool,
// True if @stat[] is valid.
    pub stat_valid: [bool; TOMOYO_MAX_PATH_STAT],
// First pathname. Initialized with { NULL, NULL } if no path.
    pub path1: path,
// Second pathname. Initialized with { NULL, NULL } if no path.
    pub path2: path,
//
// Information on @path1, @path1's parent directory, @path2, @path2's
// parent directory.
//
    pub stat: [tomoyo_mini_stat; TOMOYO_MAX_PATH_STAT],
//
// Content of symbolic link to be created. NULL for operations other
// than symlink().
//
    pub symlink_target: *mut tomoyo_path_info,
}

// Structure for argv[].
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_argv {
    pub index: c_ulong,
    pub value: *const tomoyo_path_info,
    pub is_not: bool,
}

// Structure for envp[].
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_envp {
    pub name: *const tomoyo_path_info,
    pub value: *const tomoyo_path_info,
    pub is_not: bool,
}

// Structure for execve() operation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_execve {
    pub r: tomoyo_request_info,
    pub obj: tomoyo_obj_info,
    pub bprm: *mut linux_binprm,
    pub transition: *const tomoyo_path_info,
// For dumping argv[] and envp[].
    pub dump: tomoyo_page_dump,
// For temporary use.
    pub /: *mut *mut *mut char tmp; / Size is TOMOYO_EXEC_TMPSIZE bytes,
}

// Structure for entries which follows "struct tomoyo_condition".
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_condition_element {
//
// Left hand operand. A "struct tomoyo_argv" for TOMOYO_ARGV_ENTRY, a
// "struct tomoyo_envp" for TOMOYO_ENVP_ENTRY is attached to the tail
// of the array of this struct.
//
    pub left: u8,
//
// Right hand operand. A "struct tomoyo_number_union" for
// TOMOYO_NUMBER_UNION, a "struct tomoyo_name_union" for
// TOMOYO_NAME_UNION is attached to the tail of the array of this
// struct.
//
    pub right: u8,
// Equation operator. True if equals or overlaps, false otherwise.
    pub equals: bool,
}

// Structure for optional arguments.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_condition {
    pub head: tomoyo_shared_acl_head,
    pub /: *mut *mut u32 size; / Memory size allocated for this entry.,
    pub /: *mut *mut u16 condc; / Number of conditions in this struct.,
    pub /: *mut *mut u16 numbers_count; / Number of "struct tomoyo_number_union values".,
    pub /: *mut *mut u16 names_count; / Number of "struct tomoyo_name_union names".,
    pub /: *mut *mut u16 argc; / Number of "struct tomoyo_argv".,
    pub /: *mut *mut u16 envc; / Number of "struct tomoyo_envp".,
    pub /: *mut *mut u8 grant_log; / One of values in "enum tomoyo_grant_log".,
    pub /: *const *const *const tomoyo_path_info transit; / Maybe NULL.,
//
// struct tomoyo_condition_element condition[condc];
// struct tomoyo_number_union values[numbers_count];
// struct tomoyo_name_union names[names_count];
// struct tomoyo_argv argv[argc];
// struct tomoyo_envp envp[envc];
//
}

// Common header for individual entries.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_acl_info {
    pub list: list_head,
    pub /: *mut *mut *mut tomoyo_condition cond; / Maybe NULL.,
    pub /: *mut *mut s8 is_deleted; / true or false or TOMOYO_GC_IN_PROGRESS,
    pub /: *mut *mut u8 type; / One of values in "enum tomoyo_acl_entry_type_index".,
    pub __packed: },
// Structure for domain information.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_domain_info {
    pub list: list_head,
    pub acl_info_list: list_head,
// Name of this domain. Never NULL.
    pub domainname: *const tomoyo_path_info,
// Namespace for this domain. Never NULL.
    pub ns: *mut tomoyo_policy_namespace,
// Group numbers to use.
    pub BITS_PER_LONG]: unsigned long group[TOMOYO_MAX_ACL_GROUPS /,
    pub /: *mut *mut u8 profile; / Profile number to use.,
    pub /: *mut *mut bool is_deleted; / Delete flag.,
    pub flags: [bool; TOMOYO_MAX_DOMAIN_INFO_FLAGS],
    pub /: *mut *mut atomic_t users; / Number of referring tasks.,
}

//
// Structure for "task manual_domain_transition" directive.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_task_acl {
    pub /: *mut *mut tomoyo_acl_info head; / type = TOMOYO_TYPE_MANUAL_TASK_ACL,
// Pointer to domainname.
    pub domainname: *const tomoyo_path_info,
}

//
// Structure for "file execute", "file read", "file write", "file append",
// "file unlink", "file getattr", "file rmdir", "file truncate",
// "file symlink", "file chroot" and "file unmount" directive.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_path_acl {
    pub /: *mut *mut tomoyo_acl_info head; / type = TOMOYO_TYPE_PATH_ACL,
    pub /: *mut *mut u16 perm; / Bitmask of values in "enum tomoyo_path_acl_index".,
    pub name: tomoyo_name_union,
}

//
// Structure for "file create", "file mkdir", "file mkfifo", "file mksock",
// "file ioctl", "file chmod", "file chown" and "file chgrp" directive.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_path_number_acl {
    pub /: *mut *mut tomoyo_acl_info head; / type = TOMOYO_TYPE_PATH_NUMBER_ACL,
// Bitmask of values in "enum tomoyo_path_number_acl_index".
    pub perm: u8,
    pub name: tomoyo_name_union,
    pub number: tomoyo_number_union,
}

// Structure for "file mkblock" and "file mkchar" directive.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_mkdev_acl {
    pub /: *mut *mut tomoyo_acl_info head; / type = TOMOYO_TYPE_MKDEV_ACL,
    pub /: *mut *mut u8 perm; / Bitmask of values in "enum tomoyo_mkdev_acl_index".,
    pub name: tomoyo_name_union,
    pub mode: tomoyo_number_union,
    pub major: tomoyo_number_union,
    pub minor: tomoyo_number_union,
}

//
// Structure for "file rename", "file link" and "file pivot_root" directive.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_path2_acl {
    pub /: *mut *mut tomoyo_acl_info head; / type = TOMOYO_TYPE_PATH2_ACL,
    pub /: *mut *mut u8 perm; / Bitmask of values in "enum tomoyo_path2_acl_index".,
    pub name1: tomoyo_name_union,
    pub name2: tomoyo_name_union,
}

// Structure for "file mount" directive.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_mount_acl {
    pub /: *mut *mut tomoyo_acl_info head; / type = TOMOYO_TYPE_MOUNT_ACL,
    pub dev_name: tomoyo_name_union,
    pub dir_name: tomoyo_name_union,
    pub fs_type: tomoyo_name_union,
    pub flags: tomoyo_number_union,
}

// Structure for "misc env" directive in domain policy.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_env_acl {
    pub /: *mut *mut tomoyo_acl_info head; / type = TOMOYO_TYPE_ENV_ACL,
    pub /: *const *const *const tomoyo_path_info env; / environment variable,
}

// Structure for "network inet" directive.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_inet_acl {
    pub /: *mut *mut tomoyo_acl_info head; / type = TOMOYO_TYPE_INET_ACL,
    pub protocol: u8,
    pub /: *mut *mut u8 perm; / Bitmask of values in "enum tomoyo_network_acl_index",
    pub address: tomoyo_ipaddr_union,
    pub port: tomoyo_number_union,
}

// Structure for "network unix" directive.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_unix_acl {
    pub /: *mut *mut tomoyo_acl_info head; / type = TOMOYO_TYPE_UNIX_ACL,
    pub protocol: u8,
    pub /: *mut *mut u8 perm; / Bitmask of values in "enum tomoyo_network_acl_index",
    pub name: tomoyo_name_union,
}

// Structure for holding a line from /sys/kernel/security/tomoyo/ interface.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_acl_param {
    pub data: *mut c_char,
    pub list: *mut list_head,
    pub ns: *mut tomoyo_policy_namespace,
    pub is_delete: bool,
}

pub const TOMOYO_MAX_IO_READ_QUEUE: c_int = 64;
//
// Structure for reading/writing policy via /sys/kernel/security/tomoyo
// interfaces.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_io_buffer {
    pub head): *mut *mut void (read)(struct tomoyo_io_buffer,
    pub head): *mut *mut int (write)(struct tomoyo_io_buffer,
    pub wait): *mut *mut *mut __poll_t (poll)(struct file file, poll_table,
// Exclusive lock for this structure.
    pub io_sem: mutex,
    pub read_user_buf: *mut char __user,
    pub read_user_buf_avail: usize,
    pub ns: *mut list_head,
    pub domain: *mut list_head,
    pub group: *mut list_head,
    pub acl: *mut list_head,
    pub avail: usize,
    pub step: c_uint,
    pub query_index: c_uint,
    pub index: u16,
    pub cond_index: u16,
    pub acl_group_index: u8,
    pub cond_step: u8,
    pub bit: u8,
    pub w_pos: u8,
    pub eof: bool,
    pub print_this_domain_only: bool,
    pub print_transition_related_only: bool,
    pub print_cond_part: bool,
    pub w: [*const c_char; TOMOYO_MAX_IO_READ_QUEUE],
    pub r: },
    pub ns: *mut tomoyo_policy_namespace,
// The position currently writing to.
    pub domain: *mut tomoyo_domain_info,
// Bytes available for writing.
    pub avail: usize,
    pub is_delete: bool,
    pub w: },
// Buffer for reading.
    pub __guarded_by(&io_sem): *mut *mut char read_buf,
// Size of read buffer.
    pub __guarded_by(&io_sem): size_t readbuf_size,
// Buffer for writing.
    pub __guarded_by(&io_sem): *mut *mut char write_buf,
// Size of write buffer.
    pub __guarded_by(&io_sem): size_t writebuf_size,
// Type of this interface.
    pub type: tomoyo_securityfs_interface_index,
// Users counter protected by tomoyo_io_buffer_list_lock.
    pub users: u8,
// List for telling GC not to kfree() elements.
    pub list: list_head,
}

//
// Structure for "initialize_domain"/"no_initialize_domain"/"keep_domain"
// "no_keep_domain" keyword.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_transition_control {
    pub head: tomoyo_acl_head,
    pub /: *mut *mut u8 type; / One of values in "enum tomoyo_transition_type".,
// True if the domainname is tomoyo_get_last_name().
    pub is_last_name: bool,
    pub /: *const *const *const tomoyo_path_info domainname; / Maybe NULL,
    pub /: *const *const *const tomoyo_path_info program; / Maybe NULL,
}

// Structure for "aggregator" keyword.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_aggregator {
    pub head: tomoyo_acl_head,
    pub original_name: *const tomoyo_path_info,
    pub aggregated_name: *const tomoyo_path_info,
}

// Structure for policy manager.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_manager {
    pub head: tomoyo_acl_head,
// A path to program or a domainname.
    pub manager: *const tomoyo_path_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_preference {
    pub learning_max_entry: c_uint,
    pub enforcing_verbose: bool,
    pub learning_verbose: bool,
    pub permissive_verbose: bool,
}

// Structure for /sys/kernel/security/tomnoyo/profile interface.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_profile {
    pub comment: *const tomoyo_path_info,
    pub learning: *mut tomoyo_preference,
    pub permissive: *mut tomoyo_preference,
    pub enforcing: *mut tomoyo_preference,
    pub preference: tomoyo_preference,
    pub default_config: u8,
    pub TOMOYO_MAX_MAC_CATEGORY_INDEX]: u8 config[TOMOYO_MAX_MAC_INDEX +,
    pub pref: [c_uint; TOMOYO_MAX_PREF],
}

// Structure for representing YYYY/MM/DD hh/mm/ss.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_time {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub min: u8,
    pub sec: u8,
}

// Structure for policy namespace.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_policy_namespace {
// Profile table. Memory is allocated as needed.
    pub profile_ptr: [*mut tomoyo_profile; TOMOYO_MAX_PROFILES],
// List of "struct tomoyo_group".
    pub group_list: [list_head; TOMOYO_MAX_GROUP],
// List of policy.
    pub policy_list: [list_head; TOMOYO_MAX_POLICY],
// The global ACL referred by "use_group" keyword.
    pub acl_group: [list_head; TOMOYO_MAX_ACL_GROUPS],
// List for connecting to tomoyo_namespace_list list.
    pub namespace_list: list_head,
// Profile version. Currently only 20150505 is defined.
    pub profile_version: c_uint,
// Name of this namespace (e.g. "<kernel>", "</usr/sbin/httpd>" ).
    pub name: *const c_char,
}

// Structure for "struct task_struct"->security.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tomoyo_task {
    pub domain_info: *mut tomoyo_domain_info,
    pub old_domain_info: *mut tomoyo_domain_info,
}

// External variable definitions.
// Function prototypes.
extern "C" {
    pub fn tomoyo_interface_init() -> c_int;
}
extern "C" {
    pub fn tomoyo_correct_domain(domainname: *const c_uchar) -> bool;
}
extern "C" {
    pub fn tomoyo_correct_path(filename: *const c_char) -> bool;
}
extern "C" {
    pub fn tomoyo_correct_word(string: *const c_char) -> bool;
}
extern "C" {
    pub fn tomoyo_domain_def(buffer: *const c_uchar) -> bool;
}
extern "C" {
    pub fn tomoyo_domain_quota_is_ok(r: *mut tomoyo_request_info) -> bool;
}
extern "C" {
    pub fn tomoyo_memory_ok(ptr: *mut c_void) -> bool;
}
extern "C" {
    pub fn tomoyo_permstr(string: *const c_char, keyword: *const c_char) -> bool;
}
extern "C" {
    pub fn tomoyo_str_starts(src: *mut c_char, find: *const c_char) -> bool;
}
extern "C" {
    pub fn tomoyo_close_control(head: *mut tomoyo_io_buffer);
}
extern "C" {
    pub fn tomoyo_env_perm(r: *mut tomoyo_request_info, __must_hold_shared(&tomoyo_ss: *const *const char env)) -> c_int;
}
extern "C" {
    pub fn tomoyo_find_next_domain(__must_hold_shared(&tomoyo_ss: *mut *mut linux_binprm bprm)) -> c_int;
}
extern "C" {
    pub fn tomoyo_open_control(type: u8, file: *mut file) -> c_int;
}
extern "C" {
    pub fn tomoyo_poll_control(file: *mut file, wait: *mut poll_table) -> __poll_t;
}
extern "C" {
    pub fn tomoyo_poll_log(file: *mut file, wait: *mut poll_table) -> __poll_t;
}
extern "C" {
    pub fn tomoyo_socket_listen_permission(sock: *mut socket) -> c_int;
}
extern "C" {
    pub fn tomoyo_write_aggregator(param: *mut tomoyo_acl_param) -> c_int;
}
extern "C" {
    pub fn tomoyo_write_file(param: *mut tomoyo_acl_param) -> c_int;
}
extern "C" {
    pub fn tomoyo_write_group(param: *mut tomoyo_acl_param, type: u8) -> c_int;
}
extern "C" {
    pub fn tomoyo_write_misc(param: *mut tomoyo_acl_param) -> c_int;
}
extern "C" {
    pub fn tomoyo_write_inet_network(param: *mut tomoyo_acl_param) -> c_int;
}
extern "C" {
    pub fn tomoyo_write_unix_network(param: *mut tomoyo_acl_param) -> c_int;
}
extern "C" {
    pub fn tomoyo_parse_ulong(result: *mut c_ulong, str: *mut c_char) -> u8;
}
extern "C" {
    pub fn tomoyo_load_builtin_policy() -> void __init;
}
extern "C" {
    pub fn tomoyo_mm_init() -> void __init;
}
extern "C" {
    pub fn tomoyo_check_profile();
}
extern "C" {
    pub fn tomoyo_convert_time(time: time64_t, stamp: *mut tomoyo_time);
}
extern "C" {
    pub fn tomoyo_del_condition(element: *mut list_head);
}
extern "C" {
    pub fn tomoyo_fill_path_info(ptr: *mut tomoyo_path_info);
}
extern "C" {
    pub fn tomoyo_get_attributes(obj: *mut tomoyo_obj_info);
}
extern "C" {
    pub fn tomoyo_init_policy_namespace(ns: *mut tomoyo_policy_namespace);
}
extern "C" {
    pub fn tomoyo_load_policy(filename: *const c_char);
}
extern "C" {
    pub fn tomoyo_normalize_line(buffer: *mut c_uchar);
}
extern "C" {
    pub fn tomoyo_notify_gc(head: *mut tomoyo_io_buffer, is_register: bool);
}
extern "C" {
    pub fn tomoyo_put_name_union(ptr: *mut tomoyo_name_union);
}
extern "C" {
    pub fn tomoyo_put_number_union(ptr: *mut tomoyo_number_union);
}
extern "C" {
    pub fn tomoyo_read_log(__must_hold(&head->io_sem: *mut *mut tomoyo_io_buffer head));
}
extern "C" {
    pub fn tomoyo_update_stat(index: u8);
}
extern "C" {
    pub fn tomoyo_warn_oom(function: *const c_char);
}
// Inlined functions.
//
// tomoyo_read_lock - Take lock for protecting policy.
//
// Returns index number for tomoyo_read_unlock().
//
extern "C" {
    pub fn srcu_read_lock(_arg: &tomoyo_ss) -> return;
}
//
// tomoyo_read_unlock - Release lock for protecting policy.
//
// @idx: Index number returned by tomoyo_read_lock().
//
// Returns nothing.
//
// tomoyo_sys_getppid - Copy of getppid().
//
// Returns parent process's PID.
//
// Alpha does not have getppid() defined. To be able to build this module on
// Alpha, I have to copy getppid() from kernel/timer.c.
//
// tomoyo_sys_getpid - Copy of getpid().
//
// Returns current thread's PID.
//
// Alpha does not have getpid() defined. To be able to build this module on
// Alpha, I have to copy getpid() from kernel/timer.c.
//
extern "C" {
    pub fn task_tgid_vnr(_arg: current) -> return;
}
//
// tomoyo_pathcmp - strcmp() for "struct tomoyo_path_info" structure.
//
// @a: Pointer to "struct tomoyo_path_info".
// @b: Pointer to "struct tomoyo_path_info".
//
// Returns true if @a == @b, false otherwise.
//
// tomoyo_put_name - Drop reference on "struct tomoyo_name".
//
// @name: Pointer to "struct tomoyo_path_info". Maybe NULL.
//
// Returns nothing.
//
// tomoyo_put_condition - Drop reference on "struct tomoyo_condition".
//
// @cond: Pointer to "struct tomoyo_condition". Maybe NULL.
//
// Returns nothing.
//
// tomoyo_put_group - Drop reference on "struct tomoyo_group".
//
// @group: Pointer to "struct tomoyo_group". Maybe NULL.
//
// Returns nothing.
//
// tomoyo_task - Get "struct tomoyo_task" for specified thread.
//
// @task - Pointer to "struct task_struct".
//
// Returns pointer to "struct tomoyo_task" for specified thread.
//
// tomoyo_same_name_union - Check for duplicated "struct tomoyo_name_union" entry.
//
// @a: Pointer to "struct tomoyo_name_union".
// @b: Pointer to "struct tomoyo_name_union".
//
// Returns true if @a == @b, false otherwise.
//
// tomoyo_same_number_union - Check for duplicated "struct tomoyo_number_union" entry.
//
// @a: Pointer to "struct tomoyo_number_union".
// @b: Pointer to "struct tomoyo_number_union".
//
// Returns true if @a == @b, false otherwise.
//
// tomoyo_same_ipaddr_union - Check for duplicated "struct tomoyo_ipaddr_union" entry.
//
// @a: Pointer to "struct tomoyo_ipaddr_union".
// @b: Pointer to "struct tomoyo_ipaddr_union".
//
// Returns true if @a == @b, false otherwise.
//
// tomoyo_current_namespace - Get "struct tomoyo_policy_namespace" for current thread.
//
// Returns pointer to "struct tomoyo_policy_namespace" for current thread.
//
// list_for_each_cookie - iterate over a list with cookie.
// @pos:        the &struct list_head to use as a loop cursor.
// @head:       the head for your list.
//

