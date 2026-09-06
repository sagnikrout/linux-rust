//! Automatically rewritten from C to Rust
//! Source: security/keys/big_key.c
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
// Large capacity key type
//
// Copyright (C) 2017-2020 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
// Copyright (C) 2013 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// Layout of key payload words.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct big_key_payload {
    pub data: *mut u8,
    pub path: path,
    pub length: usize,
}

    (struct big_key_payload *)((payload).data)
//
// If the data is under this limit, there's no point creating a shm file to
// hold it as the permanently resident metadata for the shmem fs will be at
// least as large as the data.
//

//
// big_key defined keys take an arbitrary string as the description and an
// arbitrary blob of data as the payload
//
    struct key_type key_type_big_key = {
    .name			= "big_key",
    .preparse		= big_key_preparse,
    .free_preparse		= big_key_free_preparse,
    .instantiate		= generic_key_instantiate,
    .revoke			= big_key_revoke,
    .destroy		= big_key_destroy,
    .describe		= big_key_describe,
    .read			= big_key_read,
    .update			= big_key_update,
    };
//
// Preparse a big key
//
#[no_mangle]
pub unsafe extern "C" fn big_key_preparse(prep: *mut key_preparsed_payload) -> c_int {
    int big_key_preparse(struct key_preparsed_payload *prep)
    {
    struct big_key_payload *payload = to_big_key_payload(prep.payload);
    struct file *file;
    u8 *buf, *enckey;
    ssize_t written;
    let mut datalen: usize = prep.datalen;
    let mut enclen: usize = datalen + CHACHA20POLY1305_AUTHTAG_SIZE;
    int ret;
    BUILD_BUG_ON(sizeof(*payload) != sizeof(prep.payload.data));
    if (datalen == 0 || datalen > 1024 * 1024 || !prep.data)
    return -EINVAL;
// Set an arbitrary quota
    prep.quotalen = 16;
    payload.length = datalen;
    if (datalen > BIG_KEY_FILE_THRESHOLD) {
// Create a shmem file to store the data in.  This will permit the data
// to be swapped out if needed.
//
// File content is stored encrypted with randomly generated key.
// Since the key is random for each file, we can set the nonce
// to zero, provided we never define a ->update() call.
//
    let mut pos: loff_t = 0;
    buf = kvmalloc(enclen, GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
// generate random key
    enckey = kmalloc(CHACHA20POLY1305_KEY_SIZE, GFP_KERNEL);
    if (!enckey) {
    ret = -ENOMEM;
    goto error;
    }
    ret = get_random_bytes_wait(enckey, CHACHA20POLY1305_KEY_SIZE);
    if (unlikely(ret))
    goto err_enckey;
// encrypt data
    chacha20poly1305_encrypt(buf, prep.data, datalen, core::ptr::null_mut(), 0,
    0, enckey);
// save aligned data to file
    file = shmem_kernel_file_setup("", enclen, EMPTY_VMA_FLAGS);
    if (IS_ERR(file)) {
    ret = PTR_ERR(file);
    goto err_enckey;
    }
    written = kernel_write(file, buf, enclen, &pos);
    if (written != enclen) {
    ret = written;
    if (written >= 0)
    ret = -EIO;
    goto err_fput;
    }
// Pin the mount and dentry to the key so that we can open it again
// later
//
    payload.data = enckey;
    payload.path = file.f_path;
    path_get(&payload.path);
    fput(file);
    kvfree_sensitive(buf, enclen);
    } else {
// Just store the data in a buffer
    void *data = kmalloc(datalen, GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    payload.data = data;
    memcpy(data, prep.data, prep.datalen);
    }
    return 0;
    err_fput:
    fput(file);
    err_enckey:
    kfree_sensitive(enckey);
    error:
    kvfree_sensitive(buf, enclen);
    return ret;
    }
//
// Clear preparsement.
//
#[no_mangle]
pub unsafe extern "C" fn big_key_free_preparse(prep: *mut key_preparsed_payload) {
    void big_key_free_preparse(struct key_preparsed_payload *prep)
    {
    struct big_key_payload *payload = to_big_key_payload(prep.payload);
    if (prep.datalen > BIG_KEY_FILE_THRESHOLD)
    path_put(&payload.path);
    kfree_sensitive(payload.data);
    }
//
// dispose of the links from a revoked keyring
// - called with the key sem write-locked
//
#[no_mangle]
pub unsafe extern "C" fn big_key_revoke(key: *mut key) {
    void big_key_revoke(struct key *key)
    {
    struct big_key_payload *payload = to_big_key_payload(key.payload);
// clear the quota
    key_payload_reserve(key, 0);
    if (key_is_positive(key) && payload.length > BIG_KEY_FILE_THRESHOLD)
    vfs_truncate(&payload.path, 0);
    }
//
// dispose of the data dangling from the corpse of a big_key key
//
#[no_mangle]
pub unsafe extern "C" fn big_key_destroy(key: *mut key) {
    void big_key_destroy(struct key *key)
    {
    struct big_key_payload *payload = to_big_key_payload(key.payload);
    if (payload.length > BIG_KEY_FILE_THRESHOLD) {
    path_put(&payload.path);
    payload.path.mnt = core::ptr::null_mut();
    payload.path.dentry = core::ptr::null_mut();
    }
    kfree_sensitive(payload.data);
    payload.data = core::ptr::null_mut();
    }
//
// Update a big key
//
#[no_mangle]
pub unsafe extern "C" fn big_key_update(key: *mut key, prep: *mut key_preparsed_payload) -> c_int {
    int big_key_update(struct key *key, struct key_preparsed_payload *prep)
    {
    int ret;
    ret = key_payload_reserve(key, prep.datalen);
    if (ret < 0)
    return ret;
    if (key_is_positive(key))
    big_key_destroy(key);
    return generic_key_instantiate(key, prep);
    }
//
// describe the big_key key
//
#[no_mangle]
pub unsafe extern "C" fn big_key_describe(key: *const key, m: *mut seq_file) {
    void big_key_describe(const struct key *key, struct seq_file *m)
    {
    struct big_key_payload *payload = to_big_key_payload(key.payload);
    seq_puts(m, key.description);
    if (key_is_positive(key))
    seq_printf(m, ": %zu [%s]",
    payload.length,
    payload.length > BIG_KEY_FILE_THRESHOLD ? "file" : "buff");
    }
//
// read the key data
// - the key's semaphore is read-locked
//
#[no_mangle]
pub unsafe extern "C" fn big_key_read(key: *const key, buffer: *mut c_char, buflen: usize) -> c_long {
    long big_key_read(const struct key *key, char *buffer, size_t buflen)
    {
    struct big_key_payload *payload = to_big_key_payload(key.payload);
    let mut datalen: usize = payload.length;
    long ret;
    if (!buffer || buflen < datalen)
    return datalen;
    if (datalen > BIG_KEY_FILE_THRESHOLD) {
    struct file *file;
    u8 *buf, *enckey = payload.data;
    let mut enclen: usize = datalen + CHACHA20POLY1305_AUTHTAG_SIZE;
    let mut pos: loff_t = 0;
    buf = kvmalloc(enclen, GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
    file = dentry_open(&payload.path, O_RDONLY, current_cred());
    if (IS_ERR(file)) {
    ret = PTR_ERR(file);
    goto error;
    }
// read file to kernel and decrypt
    ret = kernel_read(file, buf, enclen, &pos);
    if (ret != enclen) {
    if (ret >= 0)
    ret = -EIO;
    goto err_fput;
    }
    ret = chacha20poly1305_decrypt(buf, buf, enclen, core::ptr::null_mut(), 0, 0,
    enckey) ? 0 : -EBADMSG;
    if (unlikely(ret))
    goto err_fput;
    ret = datalen;
// copy out decrypted data
    memcpy(buffer, buf, datalen);
    err_fput:
    fput(file);
    error:
    kvfree_sensitive(buf, enclen);
    } else {
    ret = datalen;
    memcpy(buffer, payload.data, datalen);
    }
    return ret;
    }
//
// Register key type
//
#[no_mangle]
unsafe extern "C" fn big_key_init() -> int __init {
    static int __init big_key_init(void)
    {
    return register_key_type(&key_type_big_key);
    }
    late_initcall(big_key_init);
