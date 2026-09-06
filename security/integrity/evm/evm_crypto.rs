//! Automatically rewritten from C to Rust
//! Source: security/integrity/evm/evm_crypto.c
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
// Copyright (C) 2005-2010 IBM Corporation
//
// Authors:
// Mimi Zohar <zohar@us.ibm.com>
// Kylene Hall <kjhall@us.ibm.com>
//
// File: evm_crypto.c
// Using root's kernel master key (kmk), calculate the HMAC
//

pub const MAX_KEY_SIZE: c_int = 128;
    static unsigned char evmkey[MAX_KEY_SIZE];
    let mut evmkey_len: static int = MAX_KEY_SIZE;
    static struct crypto_shash *hmac_tfm;
    static struct crypto_shash *evm_tfm[HASH_ALGO__LAST];
    static DEFINE_MUTEX(mutex);
pub const EVM_SET_KEY_BUSY: c_int = 0;
    static unsigned long evm_set_key_flags;
    static const char evm_hmac[] = "hmac(sha1)";
//
// evm_set_key() - set EVM HMAC key from the kernel
// @key: pointer to a buffer with the key data
// @keylen: length of the key data
//
// This function allows setting the EVM HMAC key from the kernel
// without using the "encrypted" key subsystem keys. It can be used
// by the crypto HW kernel module which has its own way of managing
// keys.
//
// key length should be between 32 and 128 bytes long
//
#[no_mangle]
pub unsafe extern "C" fn evm_set_key(key: *mut c_void, keylen: usize) -> c_int {
    int evm_set_key(void *key, size_t keylen)
    {
    int rc;
    rc = -EBUSY;
    if (test_and_set_bit(EVM_SET_KEY_BUSY, &evm_set_key_flags))
    goto busy;
    rc = -EINVAL;
    if (keylen > MAX_KEY_SIZE)
    goto inval;
    memcpy(evmkey, key, keylen);
    evm_initialized |= EVM_INIT_HMAC;
    pr_info("key initialized\n");
    return 0;
    inval:
    clear_bit(EVM_SET_KEY_BUSY, &evm_set_key_flags);
    busy:
    pr_err("key initialization failed\n");
    return rc;
    }
    EXPORT_SYMBOL_GPL(evm_set_key);
    static struct shash_desc *init_desc(char type, uint8_t hash_algo)
    {
    long rc;
    const char *algo;
    struct crypto_shash **tfm, *tmp_tfm;
    struct shash_desc *desc;
    if (type == EVM_XATTR_HMAC) {
    if (!(evm_initialized & EVM_INIT_HMAC)) {
    pr_err_once("HMAC key is not set\n");
    return ERR_PTR(-ENOKEY);
    }
    tfm = &hmac_tfm;
    algo = evm_hmac;
    } else {
    if (hash_algo >= HASH_ALGO__LAST)
    return ERR_PTR(-EINVAL);
    tfm = &evm_tfm[hash_algo];
    algo = hash_algo_name[hash_algo];
    }
    if (*tfm)
    goto alloc;
    mutex_lock(&mutex);
    if (*tfm)
    goto unlock;
    tmp_tfm = crypto_alloc_shash(algo, 0, CRYPTO_NOLOAD);
    if (IS_ERR(tmp_tfm)) {
    pr_err("Can not allocate %s (reason: %ld)\n", algo,
    PTR_ERR(tmp_tfm));
    mutex_unlock(&mutex);
    return ERR_CAST(tmp_tfm);
    }
    if (type == EVM_XATTR_HMAC) {
    rc = crypto_shash_setkey(tmp_tfm, evmkey, evmkey_len);
    if (rc) {
    crypto_free_shash(tmp_tfm);
    mutex_unlock(&mutex);
    return ERR_PTR(rc);
    }
    }
// tfm = tmp_tfm;
    unlock:
    mutex_unlock(&mutex);
    alloc:
    desc = kmalloc(sizeof(*desc) + crypto_shash_descsize(*tfm),
    GFP_KERNEL);
    if (!desc)
    return ERR_PTR(-ENOMEM);
    desc.tfm = *tfm;
    rc = crypto_shash_init(desc);
    if (rc) {
    kfree(desc);
    return ERR_PTR(rc);
    }
    return desc;
    }
// Protect against 'cutting & pasting' security.evm xattr, include inode
// specific info.
//
// (Additional directory/file metadata needs to be added for more complete
// protection.)
//
    static void hmac_add_misc(struct shash_desc *desc, struct inode *inode,
    char type, char *digest)
    {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct h_misc {
//
// Although inode->i_ino is now u64, this field remains
// unsigned long to allow existing HMAC and signatures from
// 32-bit hosts to continue working when i_ino hasn't changed
// and fits in a u32.
//
    pub ino: c_ulong,
    pub generation: __u32,
    pub uid: uid_t,
    pub gid: gid_t,
    pub mode: umode_t,
    pub hmac_misc: },
    pub sizeof(hmac_misc)): memset(&hmac_misc, 0,,
// Don't include the inode or generation number in portable
// signatures
//
    if (type != EVM_XATTR_PORTABLE_DIGSIG) {
    pub inode->i_ino: hmac_misc.ino =,
    pub inode->i_generation: hmac_misc.generation =,
    }
// The hmac uid and gid must be encoded in the initial user
// namespace (not the filesystems user namespace) as encoding
// them in the filesystems user namespace allows an attack
// where first they are written in an unprivileged fuse mount
// of a filesystem and then the system is tricked to mount the
// filesystem for real on next boot and trust it because
// everything is signed.
//
    pub inode->i_uid): hmac_misc.uid = from_kuid(&init_user_ns,,
    pub inode->i_gid): hmac_misc.gid = from_kgid(&init_user_ns,,
    pub inode->i_mode: hmac_misc.mode =,
    pub sizeof(hmac_misc)): *const *const crypto_shash_update(desc, (u8 )&hmac_misc,,
    if ((evm_hmac_attrs & EVM_ATTR_FSUUID) &&
    type != EVM_XATTR_PORTABLE_DIGSIG)
    pub UUID_SIZE): *mut *mut crypto_shash_update(desc, (u8 )&inode->i_sb->s_uuid,,
    pub digest): crypto_shash_final(desc,,
    pr_debug("hmac_misc: (%zu) [%*phN]\n", sizeof(struct h_misc),
    pub &hmac_misc): (int)sizeof(struct h_misc),,
    }
//
// Dump large security xattr values as a continuous ascii hexadecimal string.
// (pr_debug is limited to 64 bytes.)
//
    static void dump_security_xattr_l(const char *prefix, const void *src,
    size_t count)
    {

    pub p: *mut *mut char asciihex,,
    pub GFP_KERNEL): *mut *mut p = asciihex = kmalloc(count  2 + 1,,
    if (!asciihex)
    pub count): p = bin2hex(p, src,,
// p = 0;
    pub asciihex): *mut *mut *mut pr_debug("%s: (%zu) %.s\n", prefix, count, (int)count  2,,

    }
    static void dump_security_xattr(const char *name, const char *value,
    size_t value_len)
    {
    if (value_len < 64)
    pr_debug("%s: (%zu) [%*phN]\n", name, value_len,
    pub value): (int)value_len,,
    else
    pub value_len): dump_security_xattr_l(name, value,,
    }
//
// Calculate the HMAC value across the set of protected security xattrs.
//
// Instead of retrieving the requested xattr, for performance, calculate
// the hmac using the requested xattr value. Don't alloc/free memory for
// each xattr, but attempt to re-use the previously allocated memory.
//
    static int evm_calc_hmac_or_hash(struct dentry *dentry,
    const char *req_xattr_name,
    const char *req_xattr_value,
    size_t req_xattr_value_len,
    uint8_t type, struct evm_digest *data,
    struct evm_iint_cache *iint)
    {
    pub D_REAL_METADATA)): *mut *mut inode inode = d_inode(d_real(dentry,,
    pub xattr: *mut xattr_list,
    pub desc: *mut shash_desc,
    pub 0: size_t xattr_size =,
    pub NULL: *mut *mut char xattr_value =,
    pub error: c_int,
    pub user_space_size: int size,,
    pub false: bool ima_present =,
    pub 0: u64 i_version =,
    if (!(inode.i_opflags & IOP_XATTR) ||
    inode.i_sb.s_user_ns != &init_user_ns)
    pub -EOPNOTSUPP: return,
    pub data->hdr.algo): desc = init_desc(type,,
    if (IS_ERR(desc))
    pub PTR_ERR(desc): return,
    pub crypto_shash_digestsize(desc->tfm): data->hdr.length =,
    pub -ENODATA: error =,
    list_for_each_entry_lockless(xattr, &evm_config_xattrnames, list) {
    pub false: bool is_ima =,
    if (strcmp(xattr.name, XATTR_NAME_IMA) == 0)
    pub true: is_ima =,
//
// Skip non-enabled xattrs for locally calculated
// signatures/HMACs.
//
    if (type != EVM_XATTR_PORTABLE_DIGSIG && !xattr.enabled)
    if ((req_xattr_name && req_xattr_value)
    && !strcmp(xattr.name, req_xattr_name)) {
    pub 0: error =,
    crypto_shash_update(desc, (const u8 *)req_xattr_value,
    if (is_ima)
    pub true: ima_present =,
    dump_security_xattr(req_xattr_name,
    req_xattr_value,
    }
    size = vfs_getxattr_alloc(&nop_mnt_idmap, dentry, xattr.name,
    pub GFP_NOFS): &xattr_value, xattr_size,,
    if (size == -ENOMEM) {
    pub -ENOMEM: error =,
    pub out: goto,
    }
    if (size < 0)
    user_space_size = vfs_getxattr(&nop_mnt_idmap, dentry,
    pub 0): xattr->name, NULL,,
    if (user_space_size != size)
    pr_debug("file %s: xattr %s size mismatch (kernel: %d, user: %d)\n",
    dentry.d_name.name, xattr.name, size,
    pub 0: error =,
    pub size: xattr_size =,
    pub xattr_size): *const *const crypto_shash_update(desc, (u8 )xattr_value,,
    if (is_ima)
    pub true: ima_present =,
    pub xattr_size): dump_security_xattr(xattr->name, xattr_value,,
    }
    pub data->digest): hmac_add_misc(desc, inode, type,,
    if (inode != d_backing_inode(dentry) && iint) {
    if (IS_I_VERSION(inode))
    pub inode_query_iversion(inode): i_version =,
    integrity_inode_attrs_store(&iint.metadata_inode, i_version,
    }
// Portable EVM signatures must include an IMA hash
    if (type == EVM_XATTR_PORTABLE_DIGSIG && !ima_present)
    pub -EPERM: error =,
    out:
    pub error: return,
    }
    int evm_calc_hmac(struct dentry *dentry, const char *req_xattr_name,
    const char *req_xattr_value, size_t req_xattr_value_len,
    struct evm_digest *data, struct evm_iint_cache *iint)
    {
    return evm_calc_hmac_or_hash(dentry, req_xattr_name, req_xattr_value,
    req_xattr_value_len, EVM_XATTR_HMAC, data,
    }
    int evm_calc_hash(struct dentry *dentry, const char *req_xattr_name,
    const char *req_xattr_value, size_t req_xattr_value_len,
    char type, struct evm_digest *data, struct evm_iint_cache *iint)
    {
    return evm_calc_hmac_or_hash(dentry, req_xattr_name, req_xattr_value,
    pub iint): req_xattr_value_len, type, data,,
    }
#[no_mangle]
unsafe extern "C" fn evm_is_immutable(dentry: *mut dentry, inode: *mut inode) -> c_int {
    static int evm_is_immutable(struct dentry *dentry, struct inode *inode)
    {
    pub NULL: *const *const evm_ima_xattr_data xattr_data =,
    pub iint: *mut evm_iint_cache,
    pub 0: int rc =,
    pub evm_iint_inode(inode): iint =,
    if (iint && (iint.flags & EVM_IMMUTABLE_DIGSIG))
    pub 1: return,
// Do this the hard way
    rc = vfs_getxattr_alloc(&nop_mnt_idmap, dentry, XATTR_NAME_EVM,
    pub GFP_NOFS): *mut *mut *mut (char )&xattr_data, 0,,
    if (rc <= 0) {
    if (rc == -ENODATA)
    pub 0: rc =,
    pub out: goto,
    }
    if (xattr_data.type == EVM_XATTR_PORTABLE_DIGSIG)
    pub 1: rc =,
    else
    pub 0: rc =,
    out:
    pub rc: return,
    }
//
// Calculate the hmac and update security.evm xattr
//
// Expects to be called with i_mutex locked.
//
    int evm_update_evmxattr(struct dentry *dentry, const char *xattr_name,
    const char *xattr_value, size_t xattr_value_len)
    {
    pub d_backing_inode(dentry): *mut *mut inode inode =,
    pub evm_iint_inode(inode): *mut *mut evm_iint_cache iint =,
    pub data: evm_digest,
    pub 0: int rc =,
//
// Don't permit any transformation of the EVM xattr if the signature
// is of an immutable type
//
    pub inode): rc = evm_is_immutable(dentry,,
    if (rc < 0)
    pub rc: return,
    if (rc)
    pub -EPERM: return,
    pub HASH_ALGO_SHA1: data.hdr.algo =,
    rc = evm_calc_hmac(dentry, xattr_name, xattr_value,
    pub iint): xattr_value_len, &data,,
    if (rc == 0) {
    pub EVM_XATTR_HMAC: data.hdr.xattr.sha1.type =,
    rc = __vfs_setxattr_noperm(&nop_mnt_idmap, dentry,
    XATTR_NAME_EVM,
    &data.hdr.xattr.data[1],
    pub 0): SHA1_DIGEST_SIZE + 1,,
    } else if (rc == -ENODATA && (inode.i_opflags & IOP_XATTR)) {
    pub XATTR_NAME_EVM): rc = __vfs_removexattr(&nop_mnt_idmap, dentry,,
    }
    pub rc: return,
    }
    int evm_init_hmac(struct inode *inode, const struct xattr *xattrs,
    char *hmac_val)
    {
    pub desc: *mut shash_desc,
    pub xattr: *const xattr,
    pub xattr_entry: *mut xattr_list,
    pub HASH_ALGO_SHA1): desc = init_desc(EVM_XATTR_HMAC,,
    if (IS_ERR(desc)) {
    pub failed\n"): pr_info("init_desc,
    pub PTR_ERR(desc): return,
    }
    list_for_each_entry_lockless(xattr_entry, &evm_config_xattrnames,
    list) {
    pub {: for (xattr = xattrs; xattr->name; xattr++),
    if (strcmp(xattr_entry.name +
    XATTR_SECURITY_PREFIX_LEN, xattr.name) != 0)
    crypto_shash_update(desc, xattr.value,
    }
    }
    pub hmac_val): hmac_add_misc(desc, inode, EVM_XATTR_HMAC,,
    pub 0: return,
    }
//
// Get the key from the TPM for the SHA1-HMAC
//
#[no_mangle]
pub unsafe extern "C" fn evm_init_key() -> c_int {
    int evm_init_key(void)
    {
    pub evm_key: *mut key,
    pub ekp: *mut encrypted_key_payload,
    pub rc: c_int,
    pub NULL): evm_key = request_key(&key_type_encrypted, EVMKEY,,
    if (IS_ERR(evm_key))
    pub -ENOENT: return,
    pub evm_key->payload.data[0]: ekp =,
    pub ekp->decrypted_datalen): rc = evm_set_key(ekp->decrypted_data,,
// burn the original key contents
    pub ekp->decrypted_datalen): memset(ekp->decrypted_data, 0,,
    pub rc: return,
    }
