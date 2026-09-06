//! Automatically rewritten from C to Rust
//! Source: drivers/nvme/common/keyring.c
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
//
// Copyright (c) 2023 Hannes Reinecke, SUSE Labs
//

    static struct key *nvme_keyring;
#[no_mangle]
pub unsafe extern "C" fn nvme_keyring_id() -> key_serial_t {
    key_serial_t nvme_keyring_id(void)
    {
    return nvme_keyring.serial;
    }
    EXPORT_SYMBOL_GPL(nvme_keyring_id);
#[no_mangle]
unsafe extern "C" fn nvme_tls_psk_revoked(psk: *mut key) -> bool {
    static bool nvme_tls_psk_revoked(struct key *psk)
    {
    return test_bit(KEY_FLAG_REVOKED, &psk.flags) ||
    test_bit(KEY_FLAG_INVALIDATED, &psk.flags);
    }
    struct key *nvme_tls_key_lookup(key_serial_t key_id)
    {
    struct key *key = key_lookup(key_id);
    if (IS_ERR(key)) {
    pr_err("key id %08x not found\n", key_id);
    return key;
    }
    if (nvme_tls_psk_revoked(key)) {
    pr_err("key id %08x revoked\n", key_id);
    return ERR_PTR(-EKEYREVOKED);
    }
    return key;
    }
    EXPORT_SYMBOL_GPL(nvme_tls_key_lookup);
#[no_mangle]
unsafe extern "C" fn nvme_tls_psk_describe(key: *const key, m: *mut seq_file) {
    static void nvme_tls_psk_describe(const struct key *key, struct seq_file *m)
    {
    seq_puts(m, key.description);
    seq_printf(m, ": %u", key.datalen);
    }
    static bool nvme_tls_psk_match(const struct key *key,
    const struct key_match_data *match_data)
    {
    const char *match_id;
    size_t match_len;
    if (!key.description) {
    pr_debug("%s: no key description\n", __func__);
    return false;
    }
    if (!match_data.raw_data) {
    pr_debug("%s: no match data\n", __func__);
    return false;
    }
    match_id = match_data.raw_data;
    match_len = strlen(match_id);
    pr_debug("%s: match '%s' '%s' len %zd\n",
    __func__, match_id, key.description, match_len);
    return !memcmp(key.description, match_id, match_len);
    }
#[no_mangle]
unsafe extern "C" fn nvme_tls_psk_match_preparse(match_data: *mut key_match_data) -> c_int {
    static int nvme_tls_psk_match_preparse(struct key_match_data *match_data)
    {
    match_data.lookup_type = KEYRING_SEARCH_LOOKUP_ITERATE;
    match_data.cmp = nvme_tls_psk_match;
    return 0;
    }
    static struct key_type nvme_tls_psk_key_type = {
    .name           = "psk",
    .flags          = KEY_TYPE_NET_DOMAIN,
    .preparse       = user_preparse,
    .free_preparse  = user_free_preparse,
    .match_preparse = nvme_tls_psk_match_preparse,
    .instantiate    = generic_key_instantiate,
    .revoke         = user_revoke,
    .destroy        = user_destroy,
    .describe       = nvme_tls_psk_describe,
    .read           = user_read,
    };
    static struct key *nvme_tls_psk_lookup(struct key *keyring,
    const char *hostnqn, const char *subnqn,
    u8 hmac, u8 psk_ver, bool generated)
    {
    char *identity;
    let mut identity_len: usize = (NVMF_NQN_SIZE) * 2 + 11;
    key_ref_t keyref;
    key_serial_t keyring_id;
    identity = kzalloc(identity_len, GFP_KERNEL);
    if (!identity)
    return ERR_PTR(-ENOMEM);
    snprintf(identity, identity_len, "NVMe%u%c%02u %s %s",
    psk_ver, generated ? 'G' : 'R', hmac, hostnqn, subnqn);
    if (!keyring)
    keyring = nvme_keyring;
    keyring_id = key_serial(keyring);
    pr_debug("keyring %x lookup tls psk '%s'\n",
    keyring_id, identity);
    keyref = keyring_search(make_key_ref(keyring, true),
    &nvme_tls_psk_key_type,
    identity, false);
    if (IS_ERR(keyref)) {
    pr_debug("lookup tls psk '%s' failed, error %ld\n",
    identity, PTR_ERR(keyref));
    kfree(identity);
    return ERR_PTR(-ENOKEY);
    }
    kfree(identity);
    return key_ref_to_ptr(keyref);
    }
//
// nvme_tls_psk_refresh - Refresh TLS PSK
// @keyring: Keyring holding the TLS PSK
// @hostnqn: Host NQN to use
// @subnqn: Subsystem NQN to use
// @hmac_id: Hash function identifier
// @data: TLS PSK key material
// @data_len: Length of @data
// @digest: TLS PSK digest
//
// Refresh a generated version 1 TLS PSK with the identity generated
// from @hmac_id, @hostnqn, @subnqn, and @digest in the keyring given
// by @keyring.
//
// Returns the updated key success or an error pointer otherwise.
//
    struct key *nvme_tls_psk_refresh(struct key *keyring,
    const char *hostnqn, const char *subnqn, u8 hmac_id,
    u8 *data, size_t data_len, const char *digest)
    {
    key_perm_t keyperm =
    KEY_POS_SEARCH | KEY_POS_VIEW | KEY_POS_READ |
    KEY_POS_WRITE | KEY_POS_LINK | KEY_POS_SETATTR |
    KEY_USR_SEARCH | KEY_USR_VIEW | KEY_USR_READ;
    char *identity;
    key_ref_t keyref;
    key_serial_t keyring_id;
    struct key *key;
    if (!hostnqn || !subnqn || !data || !data_len)
    return ERR_PTR(-EINVAL);
    identity = kasprintf(GFP_KERNEL, "NVMe1G%02d %s %s %s",
    hmac_id, hostnqn, subnqn, digest);
    if (!identity)
    return ERR_PTR(-ENOMEM);
    if (!keyring)
    keyring = nvme_keyring;
    keyring_id = key_serial(keyring);
    pr_debug("keyring %x refresh tls psk '%s'\n",
    keyring_id, identity);
    keyref = key_create_or_update(make_key_ref(keyring, true),
    "psk", identity, data, data_len,
    keyperm, KEY_ALLOC_NOT_IN_QUOTA |
    KEY_ALLOC_BUILT_IN |
    KEY_ALLOC_BYPASS_RESTRICTION);
    if (IS_ERR(keyref)) {
    pr_debug("refresh tls psk '%s' failed, error %ld\n",
    identity, PTR_ERR(keyref));
    kfree(identity);
    return ERR_PTR(-ENOKEY);
    }
    kfree(identity);
//
// Set the default timeout to 1 hour
// as suggested in TP8018.
//
    key = key_ref_to_ptr(keyref);
    key_set_timeout(key, 3600);
    return key;
    }
    EXPORT_SYMBOL_GPL(nvme_tls_psk_refresh);
//
// NVMe PSK priority list
//
// 'Retained' PSKs (ie 'generated == false') should be preferred to 'generated'
// PSKs, PSKs with hash (psk_ver 1) should be preferred to PSKs without hash
// (psk_ver 0), and SHA-384 should be preferred to SHA-256.
//
    static struct nvme_tls_psk_priority_list {
    bool generated;
    u8 psk_ver;
    enum nvme_tcp_tls_cipher cipher;
    } nvme_tls_psk_prio[] = {
    { .generated = false,
    .psk_ver = 1,
    .cipher = NVME_TCP_TLS_CIPHER_SHA384, },
    { .generated = false,
    .psk_ver = 1,
    .cipher = NVME_TCP_TLS_CIPHER_SHA256, },
    { .generated = false,
    .psk_ver = 0,
    .cipher = NVME_TCP_TLS_CIPHER_SHA384, },
    { .generated = false,
    .psk_ver = 0,
    .cipher = NVME_TCP_TLS_CIPHER_SHA256, },
    { .generated = true,
    .psk_ver = 1,
    .cipher = NVME_TCP_TLS_CIPHER_SHA384, },
    { .generated = true,
    .psk_ver = 1,
    .cipher = NVME_TCP_TLS_CIPHER_SHA256, },
    { .generated = true,
    .psk_ver = 0,
    .cipher = NVME_TCP_TLS_CIPHER_SHA384, },
    { .generated = true,
    .psk_ver = 0,
    .cipher = NVME_TCP_TLS_CIPHER_SHA256, },
    };
//
// nvme_tls_psk_default - Return the preferred PSK to use for TLS ClientHello
//
    key_serial_t nvme_tls_psk_default(struct key *keyring,
    const char *hostnqn, const char *subnqn)
    {
    struct key *tls_key;
    key_serial_t tls_key_id;
    int prio;
    for (prio = 0; prio < ARRAY_SIZE(nvme_tls_psk_prio); prio++) {
    let mut generated: bool = nvme_tls_psk_prio[prio].generated;
    let mut ver: u8 = nvme_tls_psk_prio[prio].psk_ver;
    let mut cipher: enum nvme_tcp_tls_cipher = nvme_tls_psk_prio[prio].cipher;
    tls_key = nvme_tls_psk_lookup(keyring, hostnqn, subnqn,
    cipher, ver, generated);
    if (!IS_ERR(tls_key)) {
    tls_key_id = tls_key.serial;
    key_put(tls_key);
    return tls_key_id;
    }
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(nvme_tls_psk_default);
#[no_mangle]
unsafe extern "C" fn nvme_keyring_init() -> int __init {
    static int __init nvme_keyring_init(void)
    {
    int err;
    nvme_keyring = keyring_alloc(".nvme",
    GLOBAL_ROOT_UID, GLOBAL_ROOT_GID,
    current_cred(),
    (KEY_POS_ALL & ~KEY_POS_SETATTR) |
    (KEY_USR_ALL & ~KEY_USR_SETATTR),
    KEY_ALLOC_NOT_IN_QUOTA, core::ptr::null_mut(), core::ptr::null_mut());
    if (IS_ERR(nvme_keyring))
    return PTR_ERR(nvme_keyring);
    err = register_key_type(&nvme_tls_psk_key_type);
    if (err) {
    key_put(nvme_keyring);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nvme_keyring_exit() -> void __exit {
    static void __exit nvme_keyring_exit(void)
    {
    unregister_key_type(&nvme_tls_psk_key_type);
    key_revoke(nvme_keyring);
    key_put(nvme_keyring);
    }
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Hannes Reinecke <hare@suse.de>");
    MODULE_DESCRIPTION("NVMe Keyring implementation");
    module_init(nvme_keyring_init);
    module_exit(nvme_keyring_exit);
