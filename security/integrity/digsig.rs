//! Automatically rewritten from C to Rust
//! Source: security/integrity/digsig.c
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
// Copyright (C) 2011 Intel Corporation
//
// Author:
// Dmitry Kasatkin <dmitry.kasatkin@intel.com>
//

    static struct key *keyring[INTEGRITY_KEYRING_MAX];
    static const char * const keyring_name[INTEGRITY_KEYRING_MAX] = {

    "_evm",
    "_ima",

    ".evm",
    ".ima",

    ".platform",
    ".machine",
    };

    static struct key *integrity_keyring_from_id(const unsigned int id)
    {
    if (id >= INTEGRITY_KEYRING_MAX)
    return ERR_PTR(-EINVAL);
    if (!keyring[id]) {
    keyring[id] =
    request_key(&key_type_keyring, keyring_name[id], core::ptr::null_mut());
    if (IS_ERR(keyring[id])) {
    let mut err: c_int = PTR_ERR(keyring[id]);
    pr_err("no %s keyring: %d\n", keyring_name[id], err);
    keyring[id] = core::ptr::null_mut();
    return ERR_PTR(err);
    }
    }
    return keyring[id];
    }
    int integrity_digsig_verify(const unsigned int id, const char *sig, int siglen,
    const char *digest, int digestlen, u8 algo)
    {
    struct key *keyring;
    if (siglen < 2)
    return -EINVAL;
    keyring = integrity_keyring_from_id(id);
    if (IS_ERR(keyring))
    return PTR_ERR(keyring);
    switch (sig[1]) {
    case 1:
// v1 API expect signature without xattr type
    return digsig_verify(keyring, sig + 1, siglen - 1, digest,
    digestlen);
    case 2: /* regular file data hash based signature */
    return asymmetric_verify(keyring, sig, siglen, digest,
    digestlen);
    case 3: /* struct ima_file_id data based signature */
    return asymmetric_verify_v3(keyring, sig, siglen, digest,
    digestlen, algo);
    }
    return -EOPNOTSUPP;
    }
#[no_mangle]
pub unsafe extern "C" fn integrity_modsig_verify(id: c_uint, modsig: *const modsig) -> c_int {
    int integrity_modsig_verify(const unsigned int id, const struct modsig *modsig)
    {
    struct key *keyring;
    keyring = integrity_keyring_from_id(id);
    if (IS_ERR(keyring))
    return PTR_ERR(keyring);
    return ima_modsig_verify(keyring, modsig);
    }
    static int __init __integrity_init_keyring(const unsigned int id,
    key_perm_t perm,
    struct key_restriction *restriction)
    {
    const struct cred *cred = current_cred();
    let mut err: c_int = 0;
    keyring[id] = keyring_alloc(keyring_name[id], KUIDT_INIT(0),
    KGIDT_INIT(0), cred, perm,
    KEY_ALLOC_NOT_IN_QUOTA, restriction, core::ptr::null_mut());
    if (IS_ERR(keyring[id])) {
    err = PTR_ERR(keyring[id]);
    pr_info("Can't allocate %s keyring (%d)\n",
    keyring_name[id], err);
    keyring[id] = core::ptr::null_mut();
    } else {
    if (id == INTEGRITY_KEYRING_PLATFORM)
    set_platform_trusted_keys(keyring[id]);
    if (id == INTEGRITY_KEYRING_MACHINE && imputed_trust_enabled())
    set_machine_trusted_keys(keyring[id]);
    if (id == INTEGRITY_KEYRING_IMA)
    load_module_cert(keyring[id]);
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn integrity_init_keyring(id: c_uint) -> int __init {
    int __init integrity_init_keyring(const unsigned int id)
    {
    struct key_restriction *restriction;
    key_perm_t perm;
    int ret;
    perm = (KEY_POS_ALL & ~KEY_POS_SETATTR) | KEY_USR_VIEW
    | KEY_USR_READ | KEY_USR_SEARCH;
    if (id == INTEGRITY_KEYRING_PLATFORM ||
    (id == INTEGRITY_KEYRING_MACHINE &&
    !IS_ENABLED(CONFIG_INTEGRITY_CA_MACHINE_KEYRING))) {
    restriction = core::ptr::null_mut();
    goto out;
    }
    if (!IS_ENABLED(CONFIG_INTEGRITY_TRUSTED_KEYRING))
    return 0;
    restriction = kzalloc_obj(struct key_restriction);
    if (!restriction)
    return -ENOMEM;
    if (id == INTEGRITY_KEYRING_MACHINE)
    restriction.check = restrict_link_by_ca;
    else
    restriction.check = restrict_link_to_ima;
//
// MOK keys can only be added through a read-only runtime services
// UEFI variable during boot. No additional keys shall be allowed to
// load into the machine keyring following init from userspace.
//
    if (id != INTEGRITY_KEYRING_MACHINE)
    perm |= KEY_USR_WRITE;
    out:
    ret = __integrity_init_keyring(id, perm, restriction);
    if (ret)
    kfree(restriction);
    return ret;
    }
    static int __init integrity_add_key(const unsigned int id, const void *data,
    off_t size, key_perm_t perm)
    {
    key_ref_t key;
    let mut rc: c_int = 0;
    if (!keyring[id])
    return -EINVAL;
    key = key_create_or_update(make_key_ref(keyring[id], 1), "asymmetric",
    core::ptr::null_mut(), data, size, perm,
    KEY_ALLOC_NOT_IN_QUOTA);
    if (IS_ERR(key)) {
    rc = PTR_ERR(key);
    if (id != INTEGRITY_KEYRING_MACHINE)
    pr_err("Problem loading X.509 certificate %d\n", rc);
    } else {
    pr_notice("Loaded X.509 cert '%s'\n",
    key_ref_to_ptr(key).description);
    key_ref_put(key);
    }
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn integrity_load_x509(id: c_uint, path: *const c_char) -> int __init {
    int __init integrity_load_x509(const unsigned int id, const char *path)
    {
    void *data = core::ptr::null_mut();
    size_t size;
    int rc;
    key_perm_t perm;
    rc = kernel_read_file_from_path(path, 0, &data, INT_MAX, core::ptr::null_mut(),
    READING_X509_CERTIFICATE);
    if (rc < 0) {
    pr_err("Unable to open file: %s (%d)", path, rc);
    return rc;
    }
    size = rc;
    perm = (KEY_POS_ALL & ~KEY_POS_SETATTR) | KEY_USR_VIEW | KEY_USR_READ;
    pr_info("Loading X.509 certificate: %s\n", path);
    rc = integrity_add_key(id, (const void *)data, size, perm);
    vfree(data);
    return rc;
    }
    int __init integrity_load_cert(const unsigned int id, const char *source,
    const void *data, size_t len, key_perm_t perm)
    {
    if (!data)
    return -EINVAL;
    pr_info("Loading X.509 certificate: %s\n", source);
    return integrity_add_key(id, data, len, perm);
    }
