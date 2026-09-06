//! Automatically rewritten from C to Rust
//! Source: drivers/char/tpm/tpmrm-dev.c
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
// Copyright (C) 2017 James.Bottomley@HansenPartnership.com
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpmrm_priv {
    pub priv: file_priv,
    pub space: tpm_space,
}

#[no_mangle]
unsafe extern "C" fn tpmrm_open(inode: *mut inode, file: *mut file) -> c_int {
    static int tpmrm_open(struct inode *inode, struct file *file)
    {
    struct tpm_chip *chip;
    struct tpmrm_priv *priv;
    int rc;
    chip = container_of(inode.i_cdev, struct tpm_chip, cdevs);
    priv = kzalloc_obj(*priv);
    if (priv == core::ptr::null_mut())
    return -ENOMEM;
    rc = tpm2_init_space(&priv.space, TPM2_SPACE_BUFFER_SIZE);
    if (rc) {
    kfree(priv);
    return -ENOMEM;
    }
    tpm_common_open(file, chip, &priv.priv, &priv.space);
    return nonseekable_open(inode, file);
    }
#[no_mangle]
unsafe extern "C" fn tpmrm_release(inode: *mut inode, file: *mut file) -> c_int {
    static int tpmrm_release(struct inode *inode, struct file *file)
    {
    struct file_priv *fpriv = file.private_data;
    struct tpmrm_priv *priv = container_of(fpriv, struct tpmrm_priv, priv);
    tpm_common_release(file, fpriv);
    tpm2_del_space(fpriv.chip, &priv.space);
    kfree(priv);
    return 0;
    }
    const struct file_operations tpmrm_fops = {
    .owner = THIS_MODULE,
    .open = tpmrm_open,
    .read = tpm_common_read,
    .write = tpm_common_write,
    .poll = tpm_common_poll,
    .release = tpmrm_release,
    };
