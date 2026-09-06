//! Automatically rewritten from C to Rust
//! Source: fs/ecryptfs/debug.c
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
// eCryptfs: Linux filesystem encryption layer
// Functions only useful for debugging.
//
// Copyright (C) 2006 International Business Machines Corp.
// Author(s): Michael A. Halcrow <mahalcro@us.ibm.com>
//

//
// ecryptfs_dump_auth_tok - debug function to print auth toks
//
// This function will print the contents of an ecryptfs authentication
// token.
//
#[no_mangle]
pub unsafe extern "C" fn ecryptfs_dump_auth_tok(auth_tok: *mut ecryptfs_auth_tok) {
    void ecryptfs_dump_auth_tok(struct ecryptfs_auth_tok *auth_tok)
    {
    char salt[ECRYPTFS_SALT_SIZE * 2 + 1];
    char sig[ECRYPTFS_SIG_SIZE_HEX + 1];
    ecryptfs_printk(KERN_DEBUG, "Auth tok at mem loc [%p]:\n",
    auth_tok);
    if (auth_tok.flags & ECRYPTFS_PRIVATE_KEY) {
    ecryptfs_printk(KERN_DEBUG, " * private key type\n");
    } else {
    ecryptfs_printk(KERN_DEBUG, " * passphrase type\n");
    ecryptfs_to_hex(salt, auth_tok.token.password.salt,
    ECRYPTFS_SALT_SIZE);
    ecryptfs_printk(KERN_DEBUG, " * salt = [%s]\n", salt);
    if (auth_tok.token.password.flags &
    ECRYPTFS_PERSISTENT_PASSWORD) {
    ecryptfs_printk(KERN_DEBUG, " * persistent\n");
    }
    strscpy(sig, auth_tok.token.password.signature);
    ecryptfs_printk(KERN_DEBUG, " * signature = [%s]\n", sig);
    }
    ecryptfs_printk(KERN_DEBUG, " * session_key.flags = [0x%x]\n",
    auth_tok.session_key.flags);
    if (auth_tok.session_key.flags
    & ECRYPTFS_USERSPACE_SHOULD_TRY_TO_DECRYPT)
    ecryptfs_printk(KERN_DEBUG,
    " * Userspace decrypt request set\n");
    if (auth_tok.session_key.flags
    & ECRYPTFS_USERSPACE_SHOULD_TRY_TO_ENCRYPT)
    ecryptfs_printk(KERN_DEBUG,
    " * Userspace encrypt request set\n");
    if (auth_tok.session_key.flags & ECRYPTFS_CONTAINS_DECRYPTED_KEY) {
    ecryptfs_printk(KERN_DEBUG, " * Contains decrypted key\n");
    ecryptfs_printk(KERN_DEBUG,
    " * session_key.decrypted_key_size = [0x%x]\n",
    auth_tok.session_key.decrypted_key_size);
    ecryptfs_printk(KERN_DEBUG, " * Decrypted session key "
    "dump:\n");
    if (ecryptfs_verbosity > 0)
    ecryptfs_dump_hex(auth_tok.session_key.decrypted_key,
    ECRYPTFS_DEFAULT_KEY_BYTES);
    }
    if (auth_tok.session_key.flags & ECRYPTFS_CONTAINS_ENCRYPTED_KEY) {
    ecryptfs_printk(KERN_DEBUG, " * Contains encrypted key\n");
    ecryptfs_printk(KERN_DEBUG,
    " * session_key.encrypted_key_size = [0x%x]\n",
    auth_tok.session_key.encrypted_key_size);
    ecryptfs_printk(KERN_DEBUG, " * Encrypted session key "
    "dump:\n");
    if (ecryptfs_verbosity > 0)
    ecryptfs_dump_hex(auth_tok.session_key.encrypted_key,
    auth_tok.session_key.
    encrypted_key_size);
    }
    }
//
// ecryptfs_dump_hex - debug hex printer
// @data: string of bytes to be printed
// @bytes: number of bytes to print
//
// Dump hexadecimal representation of char array
//
#[no_mangle]
pub unsafe extern "C" fn ecryptfs_dump_hex(data: *mut c_char, bytes: c_int) {
    void ecryptfs_dump_hex(char *data, int bytes)
    {
    if (ecryptfs_verbosity < 1)
    return;
    print_hex_dump(KERN_DEBUG, "ecryptfs: ", DUMP_PREFIX_OFFSET, 16, 1,
    data, bytes, false);
    }
