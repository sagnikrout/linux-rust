//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kernfs.h
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
// kernfs.h - pseudo filesystem decoupled from vfs locking
//

//
// NR_KERNFS_LOCK_BITS determines size (NR_KERNFS_LOCKS) of hash
// table of locks.
// Having a small hash table would impact scalability, since
// more and more kernfs_node objects will end up using same lock
// and having a very large hash table would waste memory.
//
// At the moment size of hash table of locks is being set based on
// the number of CPUs as follows:
//
// NR_CPU      NR_KERNFS_LOCK_BITS      NR_KERNFS_LOCKS
// 1                  1                       2
// 2-3                 2                       4
// 4-7                 4                       16
// 8-15                6                       64
// 16-31               8                       256
// 32 and more         10                      1024
//
// The above relation between NR_CPU and number of locks is based
// on some internal experimentation which involved booting qemu
// with different values of smp, performing some sysfs operations
// on all CPUs and observing how increase in number of locks impacts
// completion time of these sysfs operations on each CPU.
//

pub const NR_KERNFS_LOCK_BITS: c_int = 1;

//
// There's one kernfs_open_file for each open file and one kernfs_open_node
// for each kernfs_node with one or more open files.
//
// filp->private_data points to seq_file whose ->private points to
// kernfs_open_file.
//
// kernfs_open_files are chained at kernfs_open_node->files, which is
// protected by kernfs_global_locks.node_mutex[i].
//
// To reduce possible contention in sysfs access, arising due to single
// locks, use an array of locks (e.g. node_mutex) and use kernfs_node
// object address as hash keys to get the index of these locks.
//
// Hashed mutexes are safe to use here because operations using these don't
// rely on global exclusion.
//
// The hashed mutex array protects per-node data: the kernfs_open_node for
// open file management, and kernfs_node xattr operations (necessary because
// multiple superblocks with different namespaces can share the same
// kernfs_node, making per-inode locking insufficient).
//
// In future we intend to replace other global locks with hashed ones as well.
// kernfs_global_locks acts as a holder for all such hash tables.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernfs_global_locks {
    pub node_mutex: [mutex; NR_KERNFS_LOCKS],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kernfs_node_type {
    KERNFS_DIR		= 0x0001,
    KERNFS_FILE		= 0x0002,
    KERNFS_LINK		= 0x0004,
}

pub const KERNFS_TYPE_MASK: c_uint = 0x000f;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kernfs_node_flag {
    KERNFS_ACTIVATED	= 0x0010,
    KERNFS_NS		= 0x0020,
    KERNFS_HAS_SEQ_SHOW	= 0x0040,
    KERNFS_HAS_MMAP		= 0x0080,
    KERNFS_LOCKDEP		= 0x0100,
    KERNFS_HIDDEN		= 0x0200,
    KERNFS_SUICIDAL		= 0x0400,
    KERNFS_SUICIDED		= 0x0800,
    KERNFS_EMPTY_DIR	= 0x1000,
    KERNFS_HAS_RELEASE	= 0x2000,
    KERNFS_REMOVING		= 0x4000,
}

// @flags for kernfs_create_root()
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kernfs_root_flag {
//
// kernfs_nodes are created in the deactivated state and invisible.
// They require explicit kernfs_activate() to become visible.  This
// can be used to make related nodes become visible atomically
// after all nodes are created successfully.
//
    KERNFS_ROOT_CREATE_DEACTIVATED		= 0x0001,

//
// For regular files, if the opener has CAP_DAC_OVERRIDE, open(2)
// succeeds regardless of the RW permissions.  sysfs had an extra
// layer of enforcement where open(2) fails with -EACCES regardless
// of CAP_DAC_OVERRIDE if the permission doesn't have the
// respective read or write access at all (none of S_IRUGO or
// S_IWUGO) or the respective operation isn't implemented.  The
// following flag enables that behavior.
//
    KERNFS_ROOT_EXTRA_OPEN_PERM_CHECK	= 0x0002,

//
// The filesystem supports exportfs operation, so userspace can use
// fhandle to access nodes of the fs.
//
    KERNFS_ROOT_SUPPORT_EXPORTOP		= 0x0004,

//
// Support user xattrs to be written to nodes rooted at this root.
//
    KERNFS_ROOT_SUPPORT_USER_XATTR		= 0x0008,

//
// Renames must not change the parent node.
//
    KERNFS_ROOT_INVARIANT_PARENT		= 0x0010,
}

// type-specific structures for kernfs_node union members
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernfs_elem_dir {
    pub subdirs: c_ulong,
// children rbtree starts here and goes through kn->rb
    pub children: rb_root,
//
// The kernfs hierarchy this directory belongs to.  This fits
// better directly in kernfs_node but is here to save space.
//
    pub root: *mut kernfs_root,
//
// Monotonic revision counter, used to identify if a directory
// node has changed during negative dentry revalidation.
//
    pub rev: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernfs_elem_symlink {
    pub target_kn: *mut kernfs_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernfs_elem_attr {
    pub ops: *const kernfs_ops,
    pub open: *mut kernfs_open_node __rcu,
    pub size: loff_t,
    pub /: *mut *mut *mut kernfs_node notify_next; / for kernfs_notify(),
}

//
// kernfs_node - the building block of kernfs hierarchy.  Each and every
// kernfs node is represented by single kernfs_node.  Most fields are
// private to kernfs and shouldn't be accessed directly by kernfs users.
//
// As long as count reference is held, the kernfs_node itself is
// accessible.  Dereferencing elem or any other outer entity requires
// active reference.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernfs_node {
    pub count: core::sync::atomic::AtomicI32,
    pub active: core::sync::atomic::AtomicI32,

    pub dep_map: lockdep_map,

//
// Use kernfs_get_parent() and kernfs_name/path() instead of
// accessing the following two fields directly.  If the node is
// never moved to a different parent, it is safe to access the
// parent directly.
//
    pub __parent: *mut kernfs_node __rcu,
    pub name: *const char __rcu,
    pub rb: rb_node,
    pub /: *const *const *const ns_common ns; / namespace tag,
    pub /: *mut *mut unsigned int hash; / ns + name hash,
    pub flags: c_ushort,
    pub mode: umode_t,
    pub dir: kernfs_elem_dir,
    pub symlink: kernfs_elem_symlink,
    pub attr: kernfs_elem_attr,
}

//
// 64bit unique ID.  On 64bit ino setups, id is the ino.  On 32bit,
// the low 32bits are ino and upper generation.
//
// kernfs_syscall_ops may be specified on kernfs_create_root() to support
// syscalls.  These optional callbacks are invoked on the matching syscalls
// and can perform any kernfs operations which don't necessarily have to be
// the exact operation requested.  An active reference is held for each
// kernfs_node parameter.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernfs_syscall_ops {
    pub root): *mut *mut *mut int (show_options)(struct seq_file sf, struct kernfs_root,
    pub mode): umode_t,
    pub kn): *mut *mut int (rmdir)(struct kernfs_node,
    pub new_name): *const c_char,
    pub root): *mut kernfs_root,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernfs_open_file {
// published fields
    pub kn: *mut kernfs_node,
    pub file: *mut file,
    pub seq_file: *mut seq_file,
    pub priv: *mut c_void,
// private fields, do not use outside kernfs proper
    pub mutex: mutex,
    pub prealloc_mutex: mutex,
    pub event: c_int,
    pub list: list_head,
    pub prealloc_buf: *mut c_char,
    pub atomic_write_len: usize,
    pub mmapped:1: bool,
    pub released:1: bool,
    pub vm_ops: *const vm_operations_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernfs_ops {
//
// Optional open/release methods.  Both are called with
// @of->seq_file populated.
//
    pub of): *mut *mut int (open)(struct kernfs_open_file,
    pub of): *mut *mut void (release)(struct kernfs_open_file,
//
// Read is handled by either seq_file or raw_read().
//
// If seq_show() is present, seq_file path is active.  Other seq
// operations are optional and if not implemented, the behavior is
// equivalent to single_open().  @sf->private points to the
// associated kernfs_open_file.
//
// read() is bounced through kernel buffer and a read larger than
// PAGE_SIZE results in partial operation of PAGE_SIZE.
//
    pub v): *mut *mut *mut int (seq_show)(struct seq_file sf, void,
    pub ppos): *mut *mut *mut *mut void (seq_start)(struct seq_file sf, loff_t,
    pub ppos): *mut *mut *mut *mut *mut void (seq_next)(struct seq_file sf, void v, loff_t,
    pub v): *mut *mut *mut void (seq_stop)(struct seq_file sf, void,
    pub off): loff_t,
//
// write() is bounced through kernel buffer.  If atomic_write_len
// is not set, a write larger than PAGE_SIZE results in partial
// operations of PAGE_SIZE chunks.  If atomic_write_len is set,
// writes upto the specified size are executed atomically but
// larger ones are rejected with -E2BIG.
//
    pub atomic_write_len: usize,
//
// "prealloc" causes a buffer to be allocated at open for
// all read/write requests.  As ->seq_show uses seq_read()
// which does its own allocation, it is incompatible with
// ->prealloc.  Provide ->read and ->write with ->prealloc.
//
    pub prealloc: bool,
    pub off): loff_t,
    pub pt): *mut poll_table_struct,
    pub vma): *mut *mut *mut int (mmap)(struct kernfs_open_file of, struct vm_area_struct,
    pub whence): *mut *mut *mut loff_t (llseek)(struct kernfs_open_file of, loff_t offset, int,
}

//
// The kernfs superblock creation/mount parameter context.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernfs_fs_context {
    pub /: *mut *mut *mut kernfs_root root; / Root of the hierarchy being mounted,
    pub /: *mut *mut *mut ns_common ns_tag; / Namespace tag of the mount (or NULL),
    pub /: *mut *mut unsigned long magic; / File system specific magic number,
// The following are set/used by kernfs_mount()
    pub /: *mut *mut bool new_sb_created; / Set to T if we allocated a new sb,
}

// id is ino if ino_t is 64bit; otherwise, low 32bits
// gen is fixed at 1 if ino_t is 64bit; otherwise, high 32bits
extern "C" {
    pub fn kernfs_id_ino(_arg: kn->id) -> return;
}
extern "C" {
    pub fn kernfs_id_gen(_arg: kn->id) -> return;
}
//
// kernfs_enable_ns - enable namespace under a directory
// @kn: directory of interest, should be empty
//
// This is to be called right after @kn is created to enable namespace
// under it.  All children of @kn must have non-NULL namespace tags and
// only the ones which match the super_block's tag will be visible.
//
// kernfs_ns_enabled - test whether namespace is enabled
// @kn: the node to test
//
// Test whether namespace filtering is enabled for the children of @ns.
//
extern "C" {
    pub fn kernfs_name(kn: *mut kernfs_node, buf: *mut c_char, buflen: usize) -> c_int;
}
extern "C" {
    pub fn pr_cont_kernfs_name(kn: *mut kernfs_node);
}
extern "C" {
    pub fn pr_cont_kernfs_path(kn: *mut kernfs_node);
}
extern "C" {
    pub fn kernfs_get(kn: *mut kernfs_node);
}
extern "C" {
    pub fn kernfs_put(kn: *mut kernfs_node);
}
extern "C" {
    pub fn kernfs_destroy_root(root: *mut kernfs_root);
}
extern "C" {
    pub fn kernfs_root_flags(kn: *mut kernfs_node) -> c_uint;
}
extern "C" {
    pub fn kernfs_activate(kn: *mut kernfs_node);
}
extern "C" {
    pub fn kernfs_show(kn: *mut kernfs_node, show: bool);
}
extern "C" {
    pub fn kernfs_remove(kn: *mut kernfs_node);
}
extern "C" {
    pub fn kernfs_break_active_protection(kn: *mut kernfs_node);
}
extern "C" {
    pub fn kernfs_unbreak_active_protection(kn: *mut kernfs_node);
}
extern "C" {
    pub fn kernfs_remove_self(kn: *mut kernfs_node) -> bool;
}
extern "C" {
    pub fn kernfs_setattr(kn: *mut kernfs_node, iattr: *const iattr) -> c_int;
}
extern "C" {
    pub fn kernfs_notify(kn: *mut kernfs_node);
}
extern "C" {
    pub fn kernfs_get_tree(fc: *mut fs_context) -> c_int;
}
extern "C" {
    pub fn kernfs_free_fs_context(fc: *mut fs_context);
}
extern "C" {
    pub fn kernfs_kill_sb(sb: *mut super_block);
}
extern "C" {
    pub fn kernfs_init();
}

//
// kernfs_path - build full path of a given node
// @kn: kernfs_node of interest
// @buf: buffer to copy @kn's name into
// @buflen: size of @buf
//
// If @kn is NULL result will be "(null)".
//
// Returns the length of the full path.  If the full length is equal to or
// greater than @buflen, @buf contains the truncated path with the trailing
// '\0'.  On error, -errno is returned.
//
extern "C" {
    pub fn kernfs_path_from_node(_arg: kn, _arg: NULL, _arg: buf, _arg: buflen) -> return;
}
extern "C" {
    pub fn kernfs_find_and_get_ns(_arg: kn, _arg: name, _arg: NULL) -> return;
}
extern "C" {
    pub fn kernfs_walk_and_get_ns(_arg: kn, _arg: path, _arg: NULL) -> return;
}
extern "C" {
    pub fn kernfs_remove_by_name_ns(_arg: parent, _arg: name, _arg: NULL) -> return;
}
extern "C" {
    pub fn kernfs_rename_ns(_arg: kn, _arg: new_parent, _arg: new_name, _arg: NULL) -> return;
}
