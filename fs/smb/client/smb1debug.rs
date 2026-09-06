//! Automatically rewritten from C to Rust
//! Source: fs/smb/client/smb1debug.c
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
// Copyright (C) International Business Machines  Corp., 2000,2005
//
// Modified by Steve French (sfrench@us.ibm.com)
//

#[no_mangle]
pub unsafe extern "C" fn cifs_dump_detail(buf: *mut c_void, buf_len: usize, server: *mut TCP_Server_Info) {
    void cifs_dump_detail(void *buf, size_t buf_len, struct TCP_Server_Info *server)
    {

    struct smb_hdr *smb = buf;
    cifs_dbg(VFS, "Cmd: %d Err: 0x%x Flags: 0x%x Flgs2: 0x%x Mid: %d Pid: %d Wct: %d\n",
    smb.Command, smb.Status.CifsError, smb.Flags,
    smb.Flags2, smb.Mid, smb.Pid, smb.WordCount);
    if (!server.ops.check_message(buf, buf_len, server.total_read, server)) {
    cifs_dbg(VFS, "smb buf %p len %u\n", smb,
    server.ops.calc_smb_size(smb));
    }

    }
