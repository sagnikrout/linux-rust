//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fs/super_types.h
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

// Possible states of 'frozen' field

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sb_writers {
    pub /: *mut *mut unsigned short frozen; / Is sb frozen?,
    pub /: *mut *mut int freeze_kcount; / How many kernel freeze requests?,
    pub /: *mut *mut int freeze_ucount; / How many userspace freeze requests?,
    pub /: *const *const *const void freeze_owner; / Owner of the freeze,
    pub rw_sem: [percpu_rw_semaphore; SB_FREEZE_LEVELS],
}

//
// enum freeze_holder - holder of the freeze
// @FREEZE_HOLDER_KERNEL: kernel wants to freeze or thaw filesystem
// @FREEZE_HOLDER_USERSPACE: userspace wants to freeze or thaw filesystem
// @FREEZE_MAY_NEST: whether nesting freeze and thaw requests is allowed
// @FREEZE_EXCL: a freeze that can only be undone by the owner
//
// Indicate who the owner of the freeze or thaw request is and whether
// the freeze needs to be exclusive or can nest.
// Without @FREEZE_MAY_NEST, multiple freeze and thaw requests from the
// same holder aren't allowed. It is however allowed to hold a single
// @FREEZE_HOLDER_USERSPACE and a single @FREEZE_HOLDER_KERNEL freeze at
// the same time. This is relied upon by some filesystems during online
// repair or similar.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum freeze_holder {
    FREEZE_HOLDER_KERNEL	= (1U << 0),
    FREEZE_HOLDER_USERSPACE	= (1U << 1),
    FREEZE_MAY_NEST		= (1U << 2),
    FREEZE_EXCL		= (1U << 3),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct super_operations {
    pub sb): *mut *mut *mut inode (alloc_inode)(super_block,
    pub inode): *mut *mut void (destroy_inode)(struct inode,
    pub inode): *mut *mut void (free_inode)(struct inode,
    pub flags): *mut *mut *mut void (dirty_inode)(struct inode inode, int,
    pub wbc): *mut *mut *mut int (write_inode)(struct inode inode, struct writeback_control,
    pub wbc): *mut writeback_control,
    pub inode): *mut *mut int (drop_inode)(struct inode,
    pub inode): *mut *mut void (evict_inode)(struct inode,
    pub sb): *mut *mut void (put_super)(struct super_block,
    pub wait): *mut *mut *mut int (sync_fs)(struct super_block sb, int,
    pub owner): *const c_void,
    pub sb): *mut *mut int (freeze_fs)(struct super_block,
    pub owner): *const c_void,
    pub sb): *mut *mut int (unfreeze_fs)(struct super_block,
    pub kstatfs): *mut *mut *mut int (statfs)(struct dentry dentry, struct kstatfs,
    pub sb): *mut *mut void (umount_begin)(struct super_block,
    pub dentry): *mut *mut *mut int (show_options)(struct seq_file seq, struct dentry,
    pub dentry): *mut *mut *mut int (show_devname)(struct seq_file seq, struct dentry,
    pub dentry): *mut *mut *mut int (show_path)(struct seq_file seq, struct dentry,
    pub dentry): *mut *mut *mut int (show_stats)(struct seq_file seq, struct dentry,

    pub off): size_t len, loff_t,
    pub off): *const *const char data, size_t len, loff_t,
    pub inode): *mut *mut *mut *mut dquot __rcu (get_dquots)(inode,

    pub sc): *mut shrink_control,
    pub sc): *mut shrink_control,
//
// If a filesystem can support graceful removal of a device and
// continue read-write operations, implement this callback.
//
// Return 0 if the filesystem can continue read-write.
// Non-zero return value or no such callback means the fs will be shutdown
// as usual.
//
    pub bdev): *mut *mut *mut int (remove_bdev)(struct super_block sb, struct block_device,
    pub sb): *mut *mut void (shutdown)(struct super_block,
// Report a filesystem error
    pub event): *const *const void (report_error)(struct fserror_event,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct super_block {
    pub /: *mut *mut list_head s_list; / Keep this first,
    pub /: *mut *mut dev_t s_dev; / search index; _not_ kdev_t,
    pub /: *mut *mut *mut super_dev s_super_dev; / sget_fc()'s device table claim,
    pub s_blocksize_bits: c_uchar,
    pub s_blocksize: c_ulong,
    pub /: *mut *mut loff_t s_maxbytes; / Max file size,
    pub s_type: *mut file_system_type,
    pub s_op: *const super_operations,
    pub dq_op: *const dquot_operations,
    pub s_qcop: *const quotactl_ops,
    pub s_export_op: *const export_operations,
    pub s_flags: c_ulong,
    pub /: *mut *mut *mut unsigned long s_iflags; / internal SB_I_ flags,
    pub s_magic: c_ulong,
    pub s_root: *mut dentry,
    pub s_umount: rw_semaphore,
    pub s_passive: refcount_t,
    pub s_active: core::sync::atomic::AtomicI32,

    pub s_security: *mut c_void,

    pub s_xattr: *const *const xattr_handler,

    pub s_cop: *const fscrypt_operations,
    pub /: *mut *mut *mut fscrypt_keyring s_master_keys; / master crypto keys in use,

    pub s_vop: *const fsverity_operations,

    pub s_encoding: *mut unicode_map,
    pub s_encoding_flags: __u16,

    pub /: *mut *mut hlist_head s_roots; / alternate root dentries for NFS,
    pub s_roots_lock: spinlock_t,
    pub /: *mut *mut *mut mount s_mounts; / list of mounts; _not_ for fs use,
    pub /: *mut *mut *mut block_device s_bdev; / can go away once we use an accessor for @s_bdev_file,
    pub s_bdev_file: *mut file,
    pub s_bdi: *mut backing_dev_info,
    pub s_mtd: *mut mtd_info,
    pub s_instances: hlist_node,
    pub /: *mut *mut unsigned int s_quota_types; / Bitmask of supported quota types,
    pub /: *mut *mut quota_info s_dquot; / Diskquota specific options,
    pub s_writers: sb_writers,
//
// Keep s_fs_info, s_time_gran, s_fsnotify_mask, and
// s_fsnotify_info together for cache efficiency. They are frequently
// accessed and rarely modified.
//
    pub /: *mut *mut *mut void s_fs_info; / Filesystem private info,
// Granularity of c/m/atime in ns (cannot be worse than a second)
    pub s_time_gran: u32,
// Time limits for c/m/atime in seconds
    pub s_time_min: time64_t,
    pub s_time_max: time64_t,

    pub s_fsnotify_mask: u32,
    pub s_fsnotify_info: *mut fsnotify_sb_info,

//
// q: why are s_id and s_sysfs_name not the same? both are human
// readable strings that identify the filesystem
// a: s_id is allowed to change at runtime; it's used in log messages,
// and we want to when a device starts out as single device (s_id is dev
// name) but then a device is hot added and we have to switch to
// identifying it by UUID
// but s_sysfs_name is a handle for programmatic access, and can't
// change at runtime
//
    pub /: *mut *mut char s_id[32]; / Informational name,
    pub /: *mut *mut uuid_t s_uuid; / UUID,
    pub /: *mut *mut u8 s_uuid_len; / Default 16, possibly smaller for weird filesystems,
// if set, fs shows up under sysfs at /sys/fs/$FSTYP/s_sysfs_name
    pub 1]: char s_sysfs_name[UUID_STRING_LEN +,
    pub s_max_links: c_uint,
    pub /: *mut *mut unsigned int s_d_flags; / default d_flags for dentries,
//
// The next field is for VFS *only*. No filesystems have any business
// even looking at it. You had been warned.
//
    pub /: *mut *mut mutex s_vfs_rename_mutex; / Kludge,
//
// Filesystem subtype.  If non-empty the filesystem type field
// in /proc/mounts will be "type.subtype"
//
    pub s_subtype: *const c_char,
    pub /: *const *const *const dentry_operations __s_d_op; / default d_op for dentries,
    pub /: *mut *mut *mut shrinker s_shrink; / per-sb shrinker handle,
// Number of inodes with nlink == 0 but still referenced
    pub s_remove_count: atomic_long_t,
// Read-only state of the superblock is being changed
    pub s_readonly_remount: c_int,
// per-sb errseq_t for reporting writeback errors via syncfs
    pub s_wb_err: errseq_t,
// AIO completions deferred from interrupt context
    pub s_dio_done_wq: *mut workqueue_struct,
    pub s_pins: hlist_head,
//
// Owning user namespace and default context in which to
// interpret filesystem uids, gids, quotas, device nodes,
// xattrs and security labels.
//
    pub s_user_ns: *mut user_namespace,
//
// The list_lru structure is essentially just a pointer to a table
// of per-node lru lists, each of which has its own spinlock.
// There is no need to put them into separate cachelines.
//
    pub s_dentry_lru: list_lru,
    pub s_inode_lru: list_lru,
    pub rcu: rcu_head,
    pub destroy_work: work_struct,
    pub /: *mut *mut mutex s_sync_lock; / sync serialisation lock,
//
// Indicates how deep in a filesystem stack this SB is
//
    pub s_stack_depth: c_int,
// s_inode_list_lock protects s_inodes
    pub ____cacheline_aligned_in_smp: spinlock_t s_inode_list_lock,
    pub /: *mut *mut list_head s_inodes; / all inodes,
    pub s_inode_wblist_lock: spinlock_t,
    pub /: *mut *mut list_head s_inodes_wb; / writeback inodes,
    pub s_min_writeback_pages: c_long,
// number of fserrors that are being sent to fsnotify/filesystems
    pub s_pending_errors: refcount_t,

//
// Number of in-flight inode wb switches for this sb.  Drained by
// cgroup_writeback_umount() before tear-down.
//
    pub s_isw_nr_in_flight: core::sync::atomic::AtomicI32,

    pub __randomize_layout: },
//
// sb->s_flags.  Note that these mirror the equivalent MS_* flags where
// represented in both.
//

// These sb flags are internal to the kernel

// These flags relate to encoding and casefolding

// sb->s_iflags
pub const SB_I_CGROUPWB: c_uint = 0x00000001	/* cgroup-aware writeback enabled */;
pub const SB_I_NOEXEC: c_uint = 0x00000002	/* Ignore executables on this fs */;
pub const SB_I_NODEV: c_uint = 0x00000004	/* Ignore devices on this fs */;
pub const SB_I_STABLE_WRITES: c_uint = 0x00000008	/* don't modify blks until WB is done */;
// sb->s_iflags to limit user namespace mounts
pub const SB_I_RESTRICTED_VARIANT: c_uint = 0x00000010;
pub const SB_I_IMA_UNVERIFIABLE_SIGNATURE: c_uint = 0x00000020;
pub const SB_I_UNTRUSTED_MOUNTER: c_uint = 0x00000040;
pub const SB_I_EVM_HMAC_UNSUPPORTED: c_uint = 0x00000080;
pub const SB_I_SKIP_SYNC: c_uint = 0x00000100	/* Skip superblock at global sync */;
pub const SB_I_PERSB_BDI: c_uint = 0x00000200	/* has a per-sb bdi */;
pub const SB_I_TS_EXPIRY_WARNED: c_uint = 0x00000400 /* warned about timestamp range expiry */;
pub const SB_I_RETIRED: c_uint = 0x00000800	/* superblock shouldn't be reused */;
pub const SB_I_NOUMASK: c_uint = 0x00001000	/* VFS does not apply umask */;
pub const SB_I_NOIDMAP: c_uint = 0x00002000	/* No idmapped mounts on this superblock */;
pub const SB_I_ALLOW_HSM: c_uint = 0x00004000	/* Allow HSM events on this superblock */;
pub const SB_I_NO_DATA_INTEGRITY: c_uint = 0x00008000 /* fs cannot guarantee data persistence on sync */;
