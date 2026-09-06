//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/hp/hp-bioscfg/spmobj-attributes.c
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
// Functions corresponding to secure platform management object type
// attributes under BIOS PASSWORD for use with hp-bioscfg driver
//
// Copyright (c) 2022 HP Development Company, L.P.
//

    static const char * const spm_state_types[] = {
    "not provisioned",
    "provisioned",
    "provisioning in progress",
    };
    static const char * const spm_mechanism_types[] = {
    "not provisioned",
    "signing-key",
    "endorsement-key",
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct secureplatform_provisioning_data {
    pub state: u8,
    pub version: [u8; 2],
    pub reserved1: u8,
    pub features: u32,
    pub nonce: u32,
    pub reserved2: [u8; 28],
    pub sk_mod: [u8; MAX_KEY_MOD_SIZE],
    pub kek_mod: [u8; MAX_KEY_MOD_SIZE],
}

//
// hp_calculate_security_buffer() - determines size of security buffer
// for authentication scheme
//
// @authentication: the authentication content
//
// Currently only supported type is Admin password
//
#[no_mangle]
pub unsafe extern "C" fn hp_calculate_security_buffer(authentication: *const c_char) -> usize {
    size_t hp_calculate_security_buffer(const char *authentication)
    {
    size_t size, authlen;
    if (!authentication)
    return sizeof(u16) * 2;
    authlen = strlen(authentication);
    if (!authlen)
    return sizeof(u16) * 2;
    size = sizeof(u16) + authlen * sizeof(u16);
    if (!strstarts(authentication, BEAM_PREFIX))
    size += strlen(UTF_PREFIX) * sizeof(u16);
    return size;
    }
//
// hp_populate_security_buffer() - builds a security buffer for
// authentication scheme
//
// @authbuf: the security buffer
// @authentication: the authentication content
//
// Currently only supported type is PLAIN TEXT
//
#[no_mangle]
pub unsafe extern "C" fn hp_populate_security_buffer(authbuf: *mut u16, authentication: *const c_char) -> c_int {
    int hp_populate_security_buffer(u16 *authbuf, const char *authentication)
    {
    u16 *auth = authbuf;
    char *strprefix = core::ptr::null_mut();
    let mut ret: c_int = 0;
    if (strstarts(authentication, BEAM_PREFIX)) {
//
// BEAM_PREFIX is append to authbuf when a signature
// is provided and Sure Admin is enabled in BIOS
//
// BEAM_PREFIX found, convert part to unicode
    auth = hp_ascii_to_utf16_unicode(auth, authentication);
    if (!auth)
    return -EINVAL;
    } else {
//
// UTF-16 prefix is append to the * authbuf when a BIOS
// admin password is configured in BIOS
//
// append UTF_PREFIX to part and then convert it to unicode
    strprefix = kasprintf(GFP_KERNEL, "%s%s", UTF_PREFIX,
    authentication);
    if (!strprefix)
    return -ENOMEM;
    auth = hp_ascii_to_utf16_unicode(auth, strprefix);
    kfree(strprefix);
    if (!auth) {
    ret = -EINVAL;
    goto out_buffer;
    }
    }
    out_buffer:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn update_spm_state() -> isize {
    static ssize_t update_spm_state(void)
    {
    struct secureplatform_provisioning_data data;
    int ret;
    ret = hp_wmi_perform_query(HPWMI_SECUREPLATFORM_GET_STATE,
    HPWMI_SECUREPLATFORM, &data, 0,
    sizeof(data));
    if (ret < 0)
    return ret;
    bioscfg_drv.spm_data.mechanism = data.state;
    if (bioscfg_drv.spm_data.mechanism)
    bioscfg_drv.spm_data.is_enabled = 1;
    return 0;
    }
    static ssize_t statusbin(struct kobject *kobj,
    struct kobj_attribute *attr,
    struct secureplatform_provisioning_data *buf)
    {
    int ret = hp_wmi_perform_query(HPWMI_SECUREPLATFORM_GET_STATE,
    HPWMI_SECUREPLATFORM, buf, 0,
    sizeof(*buf));
    if (ret < 0)
    return ret;
    return sizeof(struct secureplatform_provisioning_data);
    }
//
// status_show - Reads SPM status
//
    static ssize_t status_show(struct kobject *kobj, struct kobj_attribute
// attr, char *buf)
    {
    int ret, i;
    let mut len: c_int = 0;
    struct secureplatform_provisioning_data data;
    ret = statusbin(kobj, attr, &data);
    if (ret < 0)
    return ret;
//
// 'status' is a read-only file that returns ASCII text in
// JSON format reporting the status information.
//
// "State": "not provisioned | provisioned | provisioning in progress ",
// "Version": " Major. Minor ",
// "Nonce": <16-bit unsigned number display in base 10>,
// "FeaturesInUse": <16-bit unsigned number display in base 10>,
// "EndorsementKeyMod": "<256 bytes in base64>",
// "SigningKeyMod": "<256 bytes in base64>"
//
    len += sysfs_emit_at(buf, len, "{\n");
    len += sysfs_emit_at(buf, len, "\t\"State\": \"%s\",\n",
    spm_state_types[data.state]);
    len += sysfs_emit_at(buf, len, "\t\"Version\": \"%d.%d\"",
    data.version[0], data.version[1]);
//
// state == 0 means secure platform management
// feature is not configured in BIOS.
//
    if (data.state == 0) {
    len += sysfs_emit_at(buf, len, "\n");
    goto status_exit;
    } else {
    len += sysfs_emit_at(buf, len, ",\n");
    }
    len += sysfs_emit_at(buf, len, "\t\"Nonce\": %d,\n", data.nonce);
    len += sysfs_emit_at(buf, len, "\t\"FeaturesInUse\": %d,\n", data.features);
    len += sysfs_emit_at(buf, len, "\t\"EndorsementKeyMod\": \"");
    for (i = 255; i >= 0; i--)
    len += sysfs_emit_at(buf, len, " %u", data.kek_mod[i]);
    len += sysfs_emit_at(buf, len, " \",\n");
    len += sysfs_emit_at(buf, len, "\t\"SigningKeyMod\": \"");
    for (i = 255; i >= 0; i--)
    len += sysfs_emit_at(buf, len, " %u", data.sk_mod[i]);
// Return buf contents
    len += sysfs_emit_at(buf, len, " \"\n");
    status_exit:
    len += sysfs_emit_at(buf, len, "}\n");
    return len;
    }
    let mut password_spm_status: static struct kobj_attribute = __ATTR_RO(status);
    ATTRIBUTE_SPM_N_PROPERTY_SHOW(is_enabled, spm);
    let mut password_spm_is_key_enabled: static struct kobj_attribute = __ATTR_RO(is_enabled);
    static ssize_t key_mechanism_show(struct kobject *kobj, struct kobj_attribute *attr,
    char *buf)
    {
    return sysfs_emit(buf, "%s\n",
    spm_mechanism_types[bioscfg_drv.spm_data.mechanism]);
    }
    let mut password_spm_key_mechanism: static struct kobj_attribute = __ATTR_RO(key_mechanism);
    static ssize_t sk_store(struct kobject *kobj,
    struct kobj_attribute *attr,
    const char *buf, size_t count)
    {
    int ret;
    int length;
    length = count;
    if (buf[length - 1] == '\n')
    length--;
// allocate space and copy current signing key
    bioscfg_drv.spm_data.signing_key = kmemdup(buf, length, GFP_KERNEL);
    if (!bioscfg_drv.spm_data.signing_key)
    return -ENOMEM;
// submit signing key payload
    ret = hp_wmi_perform_query(HPWMI_SECUREPLATFORM_SET_SK,
    HPWMI_SECUREPLATFORM,
    (void *)bioscfg_drv.spm_data.signing_key,
    length, 0);
    if (!ret) {
    bioscfg_drv.spm_data.mechanism = SIGNING_KEY;
    hp_set_reboot_and_signal_event();
    }
    kfree(bioscfg_drv.spm_data.signing_key);
    bioscfg_drv.spm_data.signing_key = core::ptr::null_mut();
    return ret ? ret : count;
    }
    let mut password_spm_signing_key: static struct kobj_attribute = __ATTR_WO(sk);
    static ssize_t kek_store(struct kobject *kobj,
    struct kobj_attribute *attr,
    const char *buf, size_t count)
    {
    int ret;
    int length;
    length = count;
    if (buf[length - 1] == '\n')
    length--;
// allocate space and copy current signing key
    bioscfg_drv.spm_data.endorsement_key = kmemdup(buf, length, GFP_KERNEL);
    if (!bioscfg_drv.spm_data.endorsement_key) {
    ret = -ENOMEM;
    goto exit_kek;
    }
    ret = hp_wmi_perform_query(HPWMI_SECUREPLATFORM_SET_KEK,
    HPWMI_SECUREPLATFORM,
    (void *)bioscfg_drv.spm_data.endorsement_key,
    length, 0);
    if (!ret) {
    bioscfg_drv.spm_data.mechanism = ENDORSEMENT_KEY;
    hp_set_reboot_and_signal_event();
    }
    exit_kek:
    kfree(bioscfg_drv.spm_data.endorsement_key);
    bioscfg_drv.spm_data.endorsement_key = core::ptr::null_mut();
    return ret ? ret : count;
    }
    let mut password_spm_endorsement_key: static struct kobj_attribute = __ATTR_WO(kek);
    static ssize_t role_show(struct kobject *kobj, struct kobj_attribute *attr,
    char *buf)
    {
    return sysfs_emit(buf, "%s\n", BIOS_SPM);
    }
    let mut password_spm_role: static struct kobj_attribute = __ATTR_RO(role);
    static ssize_t auth_token_store(struct kobject *kobj,
    struct kobj_attribute *attr,
    const char *buf, size_t count)
    {
    let mut ret: c_int = 0;
    int length;
    length = count;
    if (buf[length - 1] == '\n')
    length--;
// allocate space and copy current auth token
    bioscfg_drv.spm_data.auth_token = kmemdup(buf, length, GFP_KERNEL);
    if (!bioscfg_drv.spm_data.auth_token) {
    ret = -ENOMEM;
    goto exit_token;
    }
    return count;
    exit_token:
    kfree(bioscfg_drv.spm_data.auth_token);
    bioscfg_drv.spm_data.auth_token = core::ptr::null_mut();
    return ret;
    }
    let mut password_spm_auth_token: static struct kobj_attribute = __ATTR_WO(auth_token);
    static struct attribute *secure_platform_attrs[] = {
    &password_spm_is_key_enabled.attr,
    &password_spm_signing_key.attr,
    &password_spm_endorsement_key.attr,
    &password_spm_key_mechanism.attr,
    &password_spm_status.attr,
    &password_spm_role.attr,
    &password_spm_auth_token.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group secure_platform_attr_group = {
    .attrs = secure_platform_attrs,
    };
#[no_mangle]
pub unsafe extern "C" fn hp_exit_secure_platform_attributes() {
    void hp_exit_secure_platform_attributes(void)
    {
// remove secure platform sysfs entry and free key data
    kfree(bioscfg_drv.spm_data.endorsement_key);
    bioscfg_drv.spm_data.endorsement_key = core::ptr::null_mut();
    kfree(bioscfg_drv.spm_data.signing_key);
    bioscfg_drv.spm_data.signing_key = core::ptr::null_mut();
    kfree(bioscfg_drv.spm_data.auth_token);
    bioscfg_drv.spm_data.auth_token = core::ptr::null_mut();
    if (bioscfg_drv.spm_data.attr_name_kobj)
    sysfs_remove_group(bioscfg_drv.spm_data.attr_name_kobj,
    &secure_platform_attr_group);
    }
#[no_mangle]
pub unsafe extern "C" fn hp_populate_secure_platform_data(attr_name_kobj: *mut kobject) -> c_int {
    int hp_populate_secure_platform_data(struct kobject *attr_name_kobj)
    {
// Populate data for Secure Platform Management
    bioscfg_drv.spm_data.attr_name_kobj = attr_name_kobj;
    strscpy(bioscfg_drv.spm_data.attribute_name, SPM_STR);
    bioscfg_drv.spm_data.is_enabled = 0;
    bioscfg_drv.spm_data.mechanism = 0;
    bioscfg_drv.pending_reboot = false;
    update_spm_state();
    bioscfg_drv.spm_data.endorsement_key = core::ptr::null_mut();
    bioscfg_drv.spm_data.signing_key = core::ptr::null_mut();
    bioscfg_drv.spm_data.auth_token = core::ptr::null_mut();
    return sysfs_create_group(attr_name_kobj, &secure_platform_attr_group);
    }
