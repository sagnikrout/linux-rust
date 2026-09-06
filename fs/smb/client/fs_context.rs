//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/client/fs_context.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2020, Microsoft Corporation.
//
// Author(s): Steve French <stfrench@microsoft.com>
// David Howells <dhowells@redhat.com>
//

// Log errors in fs_context (new mount api) but also in dmesg (old style)

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smb_version {
    Smb_1 = 1,
    Smb_20,
    Smb_21,
    Smb_30,
    Smb_302,
    Smb_311,
    Smb_3any,
    Smb_default,
    Smb_version_err
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cifs_reparse_parm {
    Opt_reparse_default,
    Opt_reparse_none,
    Opt_reparse_nfs,
    Opt_reparse_wsl,
    Opt_reparse_err
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cifs_symlink_parm {
    Opt_symlink_default,
    Opt_symlink_none,
    Opt_symlink_native,
    Opt_symlink_unix,
    Opt_symlink_mfsymlinks,
    Opt_symlink_sfu,
    Opt_symlink_nfs,
    Opt_symlink_wsl,
    Opt_symlink_err
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cifs_sec_param {
    Opt_sec_krb5,
    Opt_sec_krb5i,
    Opt_sec_krb5p,
    Opt_sec_ntlmsspi,
    Opt_sec_ntlmssp,
    Opt_sec_ntlmv2,
    Opt_sec_ntlmv2i,
    Opt_sec_none,

    Opt_sec_err
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cifs_upcall_target_param {
    Opt_upcall_target_mount,
    Opt_upcall_target_application,
    Opt_upcall_target_err
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cifs_param {
// Mount options that take no arguments
    Opt_user_xattr,
    Opt_forceuid,
    Opt_forcegid,
    Opt_noblocksend,
    Opt_noautotune,
    Opt_lease,
    Opt_nosparse,
    Opt_hard,
    Opt_soft,
    Opt_perm,
    Opt_nodelete,
    Opt_mapposix,
    Opt_mapchars,
    Opt_nomapchars,
    Opt_sfu,
    Opt_nodfs,
    Opt_posixpaths,
    Opt_unix,
    Opt_nocase,
    Opt_brl,
    Opt_handlecache,
    Opt_forcemandatorylock,
    Opt_setuidfromacl,
    Opt_setuids,
    Opt_dynperm,
    Opt_intr,
    Opt_strictsync,
    Opt_serverino,
    Opt_rwpidforward,
    Opt_cifsacl,
    Opt_acl,
    Opt_locallease,
    Opt_sign,
    Opt_ignore_signature,
    Opt_seal,
    Opt_noac,
    Opt_fsc,
    Opt_mfsymlinks,
    Opt_multiuser,
    Opt_sloppy,
    Opt_nosharesock,
    Opt_persistent,
    Opt_resilient,
    Opt_tcp_nodelay,
    Opt_domainauto,
    Opt_rdma,
    Opt_modesid,
    Opt_rootfs,
    Opt_multichannel,
    Opt_compress,
    Opt_witness,
    Opt_is_upcall_target_mount,
    Opt_is_upcall_target_application,
    Opt_unicode,

// Mount options which take numeric value
    Opt_backupuid,
    Opt_backupgid,
    Opt_uid,
    Opt_cruid,
    Opt_gid,
    Opt_port,
    Opt_file_mode,
    Opt_dirmode,
    Opt_min_enc_offload,
    Opt_retrans,
    Opt_blocksize,
    Opt_rasize,
    Opt_rsize,
    Opt_wsize,
    Opt_actimeo,
    Opt_acdirmax,
    Opt_acregmax,
    Opt_closetimeo,
    Opt_echo_interval,
    Opt_max_credits,
    Opt_max_cached_dirs,
    Opt_snapshot,
    Opt_max_channels,
    Opt_handletimeout,

// Mount options which take string value
    Opt_source,
    Opt_user,
    Opt_pass,
    Opt_pass2,
    Opt_ip,
    Opt_domain,
    Opt_srcaddr,
    Opt_iocharset,
    Opt_netbiosname,
    Opt_servern,
    Opt_nbsessinit,
    Opt_ver,
    Opt_vers,
    Opt_sec,
    Opt_cache,
    Opt_reparse,
    Opt_upcalltarget,
    Opt_nativesocket,
    Opt_symlink,
    Opt_symlinkroot,

// Mount options to be ignored
    Opt_ignore,

    Opt_err
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb3_fs_context {
    pub forceuid_specified: bool,
    pub forcegid_specified: bool,
    pub uid_specified: bool,
    pub cruid_specified: bool,
    pub gid_specified: bool,
    pub sloppy: bool,
    pub got_ip: bool,
    pub got_version: bool,
    pub got_rsize: bool,
    pub got_wsize: bool,
    pub got_bsize: bool,
    pub port: c_ushort,
    pub username: *mut c_char,
    pub password: *mut c_char,
    pub password2: *mut c_char,
    pub domainname: *mut c_char,
    pub source: *mut c_char,
    pub server_hostname: *mut c_char,
    pub UNC: *mut c_char,
    pub nodename: *mut c_char,
    pub workstation_name: [c_char; CIFS_MAX_WORKSTATION_LEN],
    pub /: *mut *mut *mut char iocharset; / local code page for mapping to and from Unicode,
    pub /: *mut *mut char source_rfc1001_name[RFC1001_NAME_LEN_WITH_NULL]; / clnt nb name,
    pub /: *mut *mut char target_rfc1001_name[RFC1001_NAME_LEN_WITH_NULL]; / srvr nb name,
    pub rfc1001_sessinit: c_int,
    pub cred_uid: kuid_t,
    pub linux_uid: kuid_t,
    pub linux_gid: kgid_t,
    pub backupuid: kuid_t,
    pub backupgid: kgid_t,
    pub file_mode: umode_t,
    pub dir_mode: umode_t,
    pub /: *mut *mut securityEnum sectype; / sectype requested via mnt opts,
    pub /: *mut *mut upcall_target_upcall_target; / where to upcall for mount,
    pub /: *mut *mut bool sign; / was signing requested via mnt opts?,
    pub ignore_signature:1: bool,
    pub retry:1: bool,
    pub intr:1: bool,
    pub setuids:1: bool,
    pub setuidfromacl:1: bool,
    pub override_uid:1: bool,
    pub override_gid:1: bool,
    pub dynperm:1: bool,
    pub noperm:1: bool,
    pub nodelete:1: bool,
    pub mode_ace:1: bool,
    pub /: *mut *mut bool no_psx_acl:1; / set if posix acl support should be disabled,
    pub cifs_acl:1: bool,
    pub /: *mut *mut bool backupuid_specified; / mount option backupuid is specified,
    pub /: *mut *mut bool backupgid_specified; / mount option backupgid is specified,
    pub disabled*/: *mut *mut bool no_xattr:1; / set if xattr (EA) support should be,
    pub /: *mut *mut bool server_ino:1; / use inode numbers from server ie UniqueId,
    pub direct_io:1: bool,
    pub /: *mut *mut bool strict_io:1; / strict cache behavior,
    pub cache_ro:1: bool,
    pub cache_rw:1: bool,
    pub /: *mut *mut bool remap:1; / set to remap seven reserved chars in filenames,
    pub /: *mut *mut bool sfu_remap:1; / remap seven reserved chars ala SFU,
    pub /: *mut *mut bool posix_paths:1; / unset to not ask for posix pathnames.,
    pub no_linux_ext:1: bool,
    pub linux_ext:1: bool,
    pub sfu_emul:1: bool,
    pub /: *mut *mut bool nullauth:1; / attempt to authenticate with null user,
    pub /: *mut *mut bool nocase:1; / request case insensitive filenames,
    pub /: *mut *mut bool nobrl:1; / disable sending byte range locks to srv,
    pub /: *mut *mut bool nohandlecache:1; / disable caching dir handles if srvr probs,
    pub /: *mut *mut bool mand_lock:1; / send mandatory not posix byte range lock reqs,
    pub /: *mut *mut bool seal:1; / request transport encryption on share,
    pub /: *mut *mut bool nodfs:1; / Do not request DFS, even if available,
    pub /: *mut *mut bool local_lease:1; / check leases only on local system, not remote,
    pub noblocksnd:1: bool,
    pub noautotune:1: bool,
    pub /: *mut *mut bool nostrictsync:1; / do not force expensive SMBflush on every sync,
    pub /: *mut *mut bool no_lease:1; / disable requesting leases,
    pub /: *mut *mut bool no_sparse:1; / do not attempt to set files sparse,
    pub /: *mut *mut bool fsc:1; / enable fscache,
    pub /: *mut *mut bool mfsymlinks:1; / use Minshall+French Symlinks,
    pub multiuser:1: bool,
    pub /: *mut *mut bool rwpidforward:1; / pid forward for read/write operations,
    pub nosharesock:1: bool,
    pub persistent:1: bool,
    pub nopersistent:1: bool,
    pub /: *mut *mut bool resilient:1; / noresilient not required since not fored for CA,
    pub domainauto:1: bool,
    pub rdma:1: bool,
    pub multichannel:1: bool,
    pub /: *mut *mut bool multichannel_specified:1; / true if user specified multichannel or nomultichannel,
    pub /: *mut *mut bool max_channels_specified:1; / true if user specified max_channels,
    pub use_client_guid:1: bool,
// reuse existing guid for multichannel
    pub client_guid: [u8; SMB2_CLIENT_GUID_SIZE],
// User-specified original r/wsize value
    pub vol_rsize: c_uint,
    pub vol_wsize: c_uint,
    pub bsize: c_uint,
    pub rasize: c_uint,
    pub rsize: c_uint,
    pub wsize: c_uint,
    pub min_offload: c_uint,
    pub retrans: c_uint,
    pub sockopt_tcp_nodelay:1: bool,
// attribute cache timeout for files and directories in jiffies
    pub acregmax: c_ulong,
    pub acdirmax: c_ulong,
// timeout for deferred close of files in jiffies
    pub closetimeo: c_ulong,
    pub ops: *mut smb_version_operations,
    pub vals: *mut smb_version_values,
    pub prepath: *mut c_char,
    pub /: *mut *mut sockaddr_storage dstaddr; / destination address,
    pub /: *mut *mut sockaddr_storage srcaddr; / allow binding to a local IP,
    pub /: *mut *mut *mut nls_table local_nls; / This is a copy of the pointer in cifs_sb,
    pub /: *mut *mut unsigned int echo_interval; / echo interval in secs,
    pub /: *mut *mut __u64 snapshot_time; / needed for timewarp tokens,
    pub /: *mut *mut __u32 handle_timeout; / persistent and durable handle timeout in ms,
    pub /: *mut *mut unsigned int max_credits; / smb3 max_credits 10 < credits < 60000,
    pub max_channels: c_uint,
    pub max_cached_dirs: c_uint,
    pub /: *mut *mut bool compress; / enable SMB2 messages (READ/WRITE) de/compression,
    pub /: *mut *mut bool rootfs:1; / if it's a SMB root file system,
    pub /: *mut *mut bool witness:1; / use witness protocol,
    pub unicode: c_int,
    pub leaf_fullpath: *mut c_char,
    pub dfs_root_ses: *mut cifs_ses,
    pub /: *mut *mut bool dfs_automount:1; / set for dfs automount only,
    pub reparse_type: cifs_reparse_type,
    pub symlink_type: cifs_symlink_type,
    pub nonativesocket:1: bool,
    pub /: *mut *mut bool dfs_conn:1; / set for dfs mounts,
    pub dns_dom: *mut c_char,
    pub /: *mut *mut *mut char symlinkroot; / top level directory for native SMB symlinks in absolute format,
}

extern "C" {
    pub fn smb3_init_fs_context(fc: *mut fs_context) -> c_int;
}
extern "C" {
    pub fn smb3_cleanup_fs_context_contents(ctx: *mut smb3_fs_context);
}
extern "C" {
    pub fn smb3_cleanup_fs_context(ctx: *mut smb3_fs_context);
}
extern "C" {
    pub fn smb3_update_mnt_flags(cifs_sb: *mut cifs_sb_info) -> c_uint;
}
//
// max deferred close timeout (jiffies) - 2^30
//

pub const MAX_CACHED_FIDS: c_int = 16;
