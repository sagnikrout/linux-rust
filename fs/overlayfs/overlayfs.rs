//! Automatically rewritten from C Header to Rust Module
//! Source: fs/overlayfs/overlayfs.h
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
// Copyright (C) 2011 Novell Inc.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ovl_path_type {
    __OVL_PATH_UPPER	= (1 << 0),
    __OVL_PATH_MERGE	= (1 << 1),
    __OVL_PATH_ORIGIN	= (1 << 2),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ovl_xattr {
    OVL_XATTR_OPAQUE,
    OVL_XATTR_REDIRECT,
    OVL_XATTR_ORIGIN,
    OVL_XATTR_IMPURE,
    OVL_XATTR_NLINK,
    OVL_XATTR_UPPER,
    OVL_XATTR_UUID,
    OVL_XATTR_METACOPY,
    OVL_XATTR_PROTATTR,
    OVL_XATTR_XWHITEOUT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ovl_inode_flag {
// Pure upper dir that may contain non pure upper entries
    OVL_IMPURE,
// Non-merge dir that may contain whiteout entries
    OVL_WHITEOUTS,
    OVL_INDEX,
    OVL_UPPERDATA,
// Inode number will remain constant over copy up.
    OVL_CONST_INO,
    OVL_HAS_DIGEST,
    OVL_VERIFIED_DIGEST,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ovl_entry_flag {
    OVL_E_UPPER_ALIAS,
    OVL_E_OPAQUE,
    OVL_E_CONNECTED,
// Lower stack may contain xwhiteout entries
    OVL_E_XWHITEOUTS,
}

//
// The tuple (fh,uuid) is a universal unique identifier for a copy up origin,
// where:
// origin.fh	- exported file handle of the lower file
// origin.uuid	- uuid of the lower filesystem
//
pub const OVL_FH_VERSION: c_int = 0;
pub const OVL_FH_MAGIC: c_uint = 0xfb;
// CPU byte order required for fid decoding:

// Is the real inode encoded in fid an upper inode?

pub const OVL_FH_FLAG_CPU_ENDIAN: c_int = 0;

// The type used to be returned by overlay exportfs for misaligned fid
pub const OVL_FILEID_V0: c_uint = 0xfb;
// The type returned by overlay exportfs for 32bit aligned fid
pub const OVL_FILEID_V1: c_uint = 0xf8;
// On-disk format for "origin" file handle
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ovl_fb {
    pub /: *mut *mut u8 version; / 0,
    pub /: *mut *mut u8 magic; / 0xfb,
    pub /: *mut *mut u8 len; / size of this header + size of fid,
    pub /: *mut *mut *mut u8 flags; / OVL_FH_FLAG_,
    pub /: *mut *mut u8 type; / fid_type of fid,
    pub /: *mut *mut uuid_t uuid; / uuid of filesystem,
    pub /: *mut *mut u32 fid[]; / file identifier should be 32bit aligned in-memory,
    pub __packed: },
// In-memory and on-wire format for overlay file handle
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ovl_fh {
    pub /: *mut *mut u8 padding[3]; / make sure fb.fid is 32bit aligned,
    pub fb: ovl_fb,
    pub buf): DECLARE_FLEX_ARRAY(u8,,
}

// On-disk format for "metacopy" xattr (if non-zero size)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ovl_metacopy {
    pub /: *mut *mut u8 version; / 0,
    pub /: *mut *mut u8 len; / size of this header + used digest bytes,
    pub flags: u8,
    pub /: *mut *mut *mut u8 digest_algo; / FS_VERITY_HASH_ALG_ constant, 0 for no digest,
    pub /: *mut *mut u8 digest[FS_VERITY_MAX_DIGEST_SIZE]; / Only the used part on disk,
    pub __packed: },

    pub 0: return,
    pub OVL_METACOPY_MIN_SIZE: return (int)metacopy->len -,
// No atime modification on underlying
    pub ovl_xattr_table: [*const *const extern char; ][2],
    pub ovl_xattr_table: [return; ox][ofs->config.userxattr],
//
// When changing ownership of an upper object map the intended ownership
// according to the upper layer's idmapping. When an upper mount idmaps files
// that are stored on-disk as owned by id 1001 to id 1000 this means stat on
// this object will report it as being owned by id 1000 when calling stat via
// the upper mount.
// In order to change ownership of an object so stat reports id 1000 when
// called on an idmapped upper mount the value written to disk - i.e., the
// value stored in ia_*id - must 1001. The mount mapping helper will thus take
// care to map 1000 to 1001.
// The mnt idmapping helpers are nops if the upper layer isn't idmapped.
//
    pub NULL): return notify_change(ovl_upper_mnt_idmap(ofs), upperdentry, attr,,
    pub NULL): int err = vfs_rmdir(ovl_upper_mnt_idmap(ofs), dir, dentry,,
    pub err): pr_debug("rmdir(%pd2) = %i\n", dentry,,
    pub err: return,
    pub NULL): int err = vfs_unlink(ovl_upper_mnt_idmap(ofs), dir, dentry,,
    pub err): pr_debug("unlink(%pd2) = %i\n", dentry,,
    pub err: return,
    pub NULL): new_dentry,,
    pub err): pr_debug("link(%pd2, %pd2) = %i\n", old_dentry, new_dentry,,
    pub err: return,
    pub NULL): int err = vfs_create(ovl_upper_mnt_idmap(ofs), dentry, mode,,
    pub err): pr_debug("create(%pd2, 0%o) = %i\n", dentry, mode,,
    pub err: return,
    pub ret: *mut dentry,
    pub NULL): ret = vfs_mkdir(ovl_upper_mnt_idmap(ofs), dir, dentry, mode,,
    pub PTR_ERR_OR_ZERO(ret)): pr_debug("mkdir(%pd2, 0%o) = %i\n", dentry, mode,,
    pub ret: return,
    pub NULL): int err = vfs_mknod(ovl_upper_mnt_idmap(ofs), dir, dentry, mode, dev,,
    pub err): pr_debug("mknod(%pd2, 0%o, 0%o) = %i\n", dentry, mode, dev,,
    pub err: return,
    pub NULL): int err = vfs_symlink(ovl_upper_mnt_idmap(ofs), dir, dentry, oldname,,
    pub err): pr_debug("symlink(\"%s\", %pd2) = %i\n", oldname, dentry,,
    pub err: return,
    pub len: int err,,
    pub path->mnt->mnt_sb): WARN_ON(path->dentry->d_sb !=,
    pub size): name, value,,
    pub 0: len = (value && err > 0) ? err :,
    pub err): path->dentry, name, min(len, 48), value, size,,
    pub err: return,
}

extern "C" {
    pub fn ovl_do_getxattr(_arg: &upperpath, _arg: ovl_xattr(ofs, _arg: ox), _arg: value, _arg: size) -> return;
}
extern "C" {
    pub fn ovl_do_getxattr(_arg: path, _arg: ovl_xattr(ofs, _arg: ox), _arg: value, _arg: size) -> return;
}
// Use vfs_setxattr(), not __vfs_setxattr(): it idmaps the security.capability rootid.
extern "C" {
    pub fn ovl_do_setxattr(_arg: ofs, _arg: dentry, _arg: ovl_xattr(ofs, _arg: ox), _arg: value, _arg: size, _arg: 0) -> return;
}
extern "C" {
    pub fn ovl_do_removexattr(_arg: ofs, _arg: dentry, _arg: ovl_xattr(ofs, _arg: ox)) -> return;
}
extern "C" {
    pub fn vfs_set_acl(_arg: ovl_upper_mnt_idmap(ofs), _arg: dentry, _arg: acl_name, _arg: acl) -> return;
}
extern "C" {
    pub fn vfs_remove_acl(_arg: ovl_upper_mnt_idmap(ofs), _arg: dentry, _arg: acl_name) -> return;
}
extern "C" {
    pub fn ovl_do_rename_rd(_arg: &rd) -> return;
}
// util.c
extern "C" {
    pub fn ovl_get_write_access(dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn ovl_put_write_access(dentry: *mut dentry);
}
extern "C" {
    pub fn ovl_start_write(dentry: *mut dentry);
}
extern "C" {
    pub fn ovl_end_write(dentry: *mut dentry);
}
extern "C" {
    pub fn ovl_want_write(dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn ovl_drop_write(dentry: *mut dentry);
}

extern "C" {
    pub fn ovl_can_decode_fh(sb: *mut super_block) -> c_int;
}
extern "C" {
    pub fn ovl_index_all(sb: *mut super_block) -> bool;
}
extern "C" {
    pub fn ovl_verify_lower(sb: *mut super_block) -> bool;
}
extern "C" {
    pub fn ovl_stack_cpy(dst: *mut ovl_path, src: *mut ovl_path, n: c_uint);
}
extern "C" {
    pub fn ovl_stack_put(stack: *mut ovl_path, n: c_uint);
}
extern "C" {
    pub fn ovl_stack_free(stack: *mut ovl_path, n: c_uint);
}
extern "C" {
    pub fn ovl_free_entry(oe: *mut ovl_entry);
}
extern "C" {
    pub fn ovl_dentry_remote(dentry: *mut dentry) -> bool;
}
extern "C" {
    pub fn ovl_dentry_update_reval(dentry: *mut dentry, realdentry: *mut dentry);
}
extern "C" {
    pub fn ovl_dentry_weird(dentry: *mut dentry) -> bool;
}
extern "C" {
    pub fn sb_has_encoding(IS_CASEFOLDED(d_inode(dentry): dentry->d_sb) &&) -> return;
}
extern "C" {
    pub fn ovl_path_type(dentry: *mut dentry) -> ovl_path_type;
}
extern "C" {
    pub fn ovl_path_upper(dentry: *mut dentry, path: *mut path);
}
extern "C" {
    pub fn ovl_path_lower(dentry: *mut dentry, path: *mut path);
}
extern "C" {
    pub fn ovl_path_lowerdata(dentry: *mut dentry, path: *mut path);
}
extern "C" {
    pub fn ovl_path_real(dentry: *mut dentry, path: *mut path) -> ovl_path_type;
}
extern "C" {
    pub fn ovl_path_realdata(dentry: *mut dentry, path: *mut path) -> ovl_path_type;
}
extern "C" {
    pub fn ovl_dentry_set_lowerdata(dentry: *mut dentry, datapath: *mut ovl_path) -> c_int;
}
extern "C" {
    pub fn ovl_set_dir_cache(inode: *mut inode, cache: *mut ovl_dir_cache);
}
extern "C" {
    pub fn ovl_dentry_set_flag(flag: c_ulong, dentry: *mut dentry);
}
extern "C" {
    pub fn ovl_dentry_clear_flag(flag: c_ulong, dentry: *mut dentry);
}
extern "C" {
    pub fn ovl_dentry_test_flag(flag: c_ulong, dentry: *mut dentry) -> bool;
}
extern "C" {
    pub fn ovl_dentry_is_opaque(dentry: *mut dentry) -> bool;
}
extern "C" {
    pub fn ovl_dentry_is_whiteout(dentry: *mut dentry) -> bool;
}
extern "C" {
    pub fn ovl_dentry_set_opaque(dentry: *mut dentry);
}
extern "C" {
    pub fn ovl_dentry_has_xwhiteouts(dentry: *mut dentry) -> bool;
}
extern "C" {
    pub fn ovl_dentry_set_xwhiteouts(dentry: *mut dentry);
}
extern "C" {
    pub fn ovl_dentry_has_upper_alias(dentry: *mut dentry) -> bool;
}
extern "C" {
    pub fn ovl_dentry_set_upper_alias(dentry: *mut dentry);
}
extern "C" {
    pub fn ovl_dentry_needs_data_copy_up(dentry: *mut dentry, flags: c_int) -> bool;
}
extern "C" {
    pub fn ovl_dentry_needs_data_copy_up_locked(dentry: *mut dentry, flags: c_int) -> bool;
}
extern "C" {
    pub fn ovl_has_upperdata(inode: *mut inode) -> bool;
}
extern "C" {
    pub fn ovl_set_upperdata(inode: *mut inode);
}
extern "C" {
    pub fn ovl_dentry_set_redirect(dentry: *mut dentry, redirect: *const c_char);
}
extern "C" {
    pub fn ovl_inode_update(inode: *mut inode, upperdentry: *mut dentry);
}
extern "C" {
    pub fn ovl_dir_modified(dentry: *mut dentry, impurity: bool);
}
extern "C" {
    pub fn ovl_inode_version_get(inode: *mut inode) -> u64;
}
extern "C" {
    pub fn ovl_is_whiteout(dentry: *mut dentry) -> bool;
}
extern "C" {
    pub fn ovl_path_is_whiteout(ofs: *mut ovl_fs, path: *const path) -> bool;
}
extern "C" {
    pub fn ovl_copy_up_start(dentry: *mut dentry, flags: c_int) -> c_int;
}
extern "C" {
    pub fn ovl_copy_up_end(dentry: *mut dentry);
}
extern "C" {
    pub fn ovl_already_copied_up(dentry: *mut dentry, flags: c_int) -> bool;
}
extern "C" {
    pub fn ovl_path_check_origin_xattr(ofs: *mut ovl_fs, path: *const path) -> bool;
}
extern "C" {
    pub fn ovl_path_check_xwhiteout_xattr(ofs: *mut ovl_fs, path: *const path) -> bool;
}
extern "C" {
    pub fn ovl_path_is_whiteout(_arg: ofs, _arg: &upperpath) -> return;
}
extern "C" {
    pub fn ovl_path_check_origin_xattr(_arg: ofs, _arg: &upperpath) -> return;
}
extern "C" {
    pub fn ovl_set_impure(dentry: *mut dentry, upperdentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn ovl_inuse_trylock(dentry: *mut dentry) -> bool;
}
extern "C" {
    pub fn ovl_inuse_unlock(dentry: *mut dentry);
}
extern "C" {
    pub fn ovl_is_inuse(dentry: *mut dentry) -> bool;
}
extern "C" {
    pub fn ovl_need_index(dentry: *mut dentry) -> bool;
}
extern "C" {
    pub fn ovl_nlink_start(dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn ovl_nlink_end(dentry: *mut dentry);
}
extern "C" {
    pub fn ovl_is_metacopy_dentry(dentry: *mut dentry) -> bool;
}
extern "C" {
    pub fn ovl_ensure_verity_loaded(path: *const path) -> c_int;
}
extern "C" {
    pub fn ovl_sync_status(ofs: *mut ovl_fs) -> c_int;
}
extern "C" {
    pub fn test_bit(_arg: flag, _arg: &OVL_I(inode)->flags) -> return;
}
extern "C" {
    pub fn ovl_get_dir_xattr_val(_arg: ofs, _arg: path, _arg: OVL_XATTR_OPAQUE) -> return;
}
//
// With xino=auto, we do best effort to keep all inodes on same st_dev and
// d_ino consistent with st_ino.
// With xino=on, we do the same effort but we warn if we failed.
//
// To avoid regressions in existing setups with overlay lower offline changes,
// we allow lower changes only if none of the new features are used.
//
// All layers on same fs?
// All overlay inodes have same st_dev?
extern "C" {
    pub fn mutex_lock_interruptible(_arg: &OVL_I(inode)->lock) -> return;
}
// namei.c
extern "C" {
    pub fn ovl_check_fb_len(fb: *mut ovl_fb, fb_len: c_int) -> c_int;
}
extern "C" {
    pub fn ovl_check_fb_len(_arg: &fh->fb, OVL_FH_WIRE_OFFSET: fh_len -) -> return;
}
extern "C" {
    pub fn ovl_verify_index(ofs: *mut ovl_fs, index: *mut dentry) -> c_int;
}
extern "C" {
    pub fn ovl_get_index_name_fh(fh: *const ovl_fh, name: *mut qstr) -> c_int;
}
extern "C" {
    pub fn ovl_verify_lowerdata(dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn ovl_lower_positive(dentry: *mut dentry) -> bool;
}
extern "C" {
    pub fn ovl_verify_set_fh(_arg: ofs, _arg: upper, _arg: OVL_XATTR_ORIGIN, _arg: fh, _arg: false, _arg: set) -> return;
}
// readdir.c
extern "C" {
    pub fn ovl_check_empty_dir(dentry: *mut dentry, list: *mut list_head) -> c_int;
}
extern "C" {
    pub fn ovl_cache_free(list: *mut list_head);
}
extern "C" {
    pub fn ovl_dir_cache_free(inode: *mut inode);
}
extern "C" {
    pub fn ovl_check_d_type_supported(realpath: *const path) -> c_int;
}
extern "C" {
    pub fn ovl_indexdir_cleanup(ofs: *mut ovl_fs) -> c_int;
}
//
// Can we iterate real dir directly?
//
// Non-merge dir may contain whiteouts from a time it was a merge upper, before
// lower dir was removed under it and possibly before it was rotated from upper
// to lower layer.
//
// inode.c
extern "C" {
    pub fn ovl_set_nlink_upper(dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn ovl_set_nlink_lower(dentry: *mut dentry) -> c_int;
}

extern "C" {
    pub fn do_ovl_get_acl(_arg: &nop_mnt_idmap, _arg: inode, _arg: type, _arg: rcu, _arg: true) -> return;
}
extern "C" {
    pub fn do_ovl_get_acl(_arg: idmap, _arg: d_inode(dentry), _arg: type, _arg: false, _arg: false) -> return;
}

extern "C" {
    pub fn ovl_is_private_xattr(sb: *mut super_block, name: *const c_char) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ovl_inode_params {
    pub newinode: *mut inode,
    pub upperdentry: *mut dentry,
    pub oe: *mut ovl_entry,
    pub index: bool,
    pub redirect: *mut c_char,
    pub lowerdata_redirect: *mut c_char,
}

extern "C" {
    pub fn ovl_lookup_trap_inode(sb: *mut super_block, dir: *mut dentry) -> bool;
}
extern "C" {
    pub fn ovl_copyattr(to: *mut inode);
}
// vfs fileattr flags read from overlay.protattr xattr to ovl inode

// vfs fileattr flags copied from real to ovl inode

// vfs inode flags copied from real to ovl inode

//
// fileattr flags copied from lower to upper inode on copy up.
// We cannot copy up immutable/append-only flags, because that would prevent
// linking temp inode to upper dir, so we store them in xattr instead.
//

extern "C" {
    pub fn ovl_check_protattr(inode: *mut inode, upper: *mut dentry);
}
// dir.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ovl_cattr {
    pub rdev: dev_t,
    pub mode: umode_t,
    pub link: *const c_char,
    pub hardlink: *mut dentry,
}

extern "C" {
    pub fn ovl_cleanup(ofs: *mut ovl_fs, workdir: *mut dentry, dentry: *mut dentry) -> c_int;
}
pub const OVL_TEMPNAME_SIZE: c_int = 20;
extern "C" {
    pub fn ovl_tempname(name[OVL_TEMPNAME_SIZE]: c_char);
}
// file.c
extern "C" {
    pub fn ovl_real_fileattr_get(realpath: *const path, fa: *mut file_kattr) -> c_int;
}
extern "C" {
    pub fn ovl_real_fileattr_set(realpath: *const path, fa: *mut file_kattr) -> c_int;
}
extern "C" {
    pub fn ovl_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> c_int;
}
extern "C" {
    pub fn ovl_file_free(of: *mut ovl_file);
}
// copy_up.c
extern "C" {
    pub fn ovl_copy_up(dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn ovl_copy_up_with_data(dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn ovl_maybe_copy_up(dentry: *mut dentry, flags: c_int) -> c_int;
}
extern "C" {
    pub fn ovl_copy_xattr(sb: *mut super_block, path: *const path, new: *mut dentry) -> c_int;
}
extern "C" {
    pub fn ovl_set_attr(ofs: *mut ovl_fs, upper: *mut dentry, stat: *mut kstat) -> c_int;
}
// export.c
// super.c
extern "C" {
    pub fn ovl_fill_super(sb: *mut super_block, fc: *mut fs_context) -> c_int;
}
// Will this overlay be forced to mount/remount ro?
// xattr.c
extern "C" {
    pub fn ovl_listxattr(dentry: *mut dentry, list: *mut c_char, size: usize) -> isize;
}
