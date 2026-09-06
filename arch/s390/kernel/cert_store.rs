//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/cert_store.c
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
// DIAG 0x320 support and certificate store handling
//
// Copyright IBM Corp. 2023
// Author(s):	Anastasia Eskova <anastasia.eskova@ibm.com>
//

pub const DIAG_MAX_RETRIES: c_int = 10;
pub const VCE_FLAGS_VALID_MASK: c_uint = 0x80;
pub const ISM_LEN_DWORDS: c_int = 4;
pub const VCSSB_LEN_BYTES: c_int = 128;
pub const VCSSB_LEN_NO_CERTS: c_int = 4;
pub const VCB_LEN_NO_CERTS: c_int = 64;
pub const VC_NAME_LEN_BYTES: c_int = 64;

    static debug_info_t *cert_store_dbf;
    static debug_info_t *cert_store_hexdump;

    debug_sprintf_event(cert_store_dbf, 3, fmt "\n", ## __VA_ARGS__)
    enum diag320_subcode {
    DIAG320_SUBCODES	= 0,
    DIAG320_STORAGE		= 1,
    DIAG320_CERT_BLOCK	= 2,
    };
    enum diag320_rc {
    DIAG320_RC_OK		= 0x0001,
    DIAG320_RC_CS_NOMATCH	= 0x0306,
    };
// Verification Certificates Store Support Block (VCSSB).
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcssb {
    pub vcssb_length: u32,
    pub pad_0x04: [u8; 3],
    pub version: u8,
    pub pad_0x08: [u8; 8],
    pub cs_token: u32,
    pub pad_0x14: [u8; 12],
    pub total_vc_index_count: u16,
    pub max_vc_index_count: u16,
    pub pad_0x24: [u8; 28],
    pub max_vce_length: u32,
    pub max_vcxe_length: u32,
    pub pad_0x48: [u8; 8],
    pub max_single_vcb_length: u32,
    pub total_vcb_length: u32,
    pub max_single_vcxb_length: u32,
    pub total_vcxb_length: u32,
    pub pad_0x60: [u8; 32],
    pub __aligned(8): } __packed,
// Verification Certificate Entry (VCE) Header.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vce_header {
    pub vce_length: u32,
    pub flags: u8,
    pub key_type: u8,
    pub vc_index: u16,
    pub /: *mut *mut u8 vc_name[VC_NAME_LEN_BYTES]; / EBCDIC,
    pub vc_format: u8,
    pub pad_0x49: u8,
    pub key_id_length: u16,
    pub pad_0x4c: u8,
    pub vc_hash_type: u8,
    pub vc_hash_length: u16,
    pub pad_0x50: [u8; 4],
    pub vc_length: u32,
    pub pad_0x58: [u8; 8],
    pub vc_hash_offset: u16,
    pub vc_offset: u16,
    pub pad_0x64: [u8; 28],
    pub __aligned(4): } __packed,
// Verification Certificate Block (VCB) Header.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcb_header {
    pub vcb_input_length: u32,
    pub pad_0x04: [u8; 4],
    pub first_vc_index: u16,
    pub last_vc_index: u16,
    pub pad_0x0c: u32,
    pub cs_token: u32,
    pub pad_0x14: [u8; 12],
    pub vcb_output_length: u32,
    pub pad_0x24: [u8; 3],
    pub version: u8,
    pub stored_vc_count: u16,
    pub remaining_vc_count: u16,
    pub pad_0x2c: [u8; 20],
    pub __aligned(4): } __packed,
// Verification Certificate Block (VCB).
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcb {
    pub vcb_hdr: vcb_header,
    pub vcb_buf: [u8; ],
    pub __aligned(4): } __packed,
// Verification Certificate Entry (VCE).
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vce {
    pub vce_hdr: vce_header,
    pub cert_data_buf: [u8; ],
    pub __aligned(4): } __packed,
#[no_mangle]
unsafe extern "C" fn cert_store_key_describe(key: *const key, m: *mut seq_file) {
    static void cert_store_key_describe(const struct key *key, struct seq_file *m)
    {
    pub 1]: char ascii[VC_NAME_LEN_BYTES +,
//
// First 64 bytes of the key description is key name in EBCDIC CP 500.
// Convert it to ASCII for displaying in /proc/keys.
//
    pub key->description): strscpy(ascii,,
    pub VC_NAME_LEN_BYTES): EBCASC_500(ascii,,
    pub ascii): seq_puts(m,,
    pub &key->description[VC_NAME_LEN_BYTES]): seq_puts(m,,
    if (key_is_positive(key))
    pub key->datalen): seq_printf(m, ": %u",,
    }
//
// Certificate store key type takes over properties of
// user key but cannot be updated.
//
    static struct key_type key_type_cert_store_key = {
    .name		= CERT_STORE_KEY_TYPE_NAME,
    .preparse	= user_preparse,
    .free_preparse	= user_free_preparse,
    .instantiate	= generic_key_instantiate,
    .revoke		= user_revoke,
    .destroy	= user_destroy,
    .describe	= cert_store_key_describe,
    .read		= user_read,
}

// Logging functions.
#[no_mangle]
unsafe extern "C" fn pr_dbf_vcb(b: *const vcb) {
    static void pr_dbf_vcb(const struct vcb *b)
    {
    pr_dbf_msg("VCB Header:");
    pr_dbf_msg("vcb_input_length: %d", b.vcb_hdr.vcb_input_length);
    pr_dbf_msg("first_vc_index: %d", b.vcb_hdr.first_vc_index);
    pr_dbf_msg("last_vc_index: %d", b.vcb_hdr.last_vc_index);
    pr_dbf_msg("cs_token: %d", b.vcb_hdr.cs_token);
    pr_dbf_msg("vcb_output_length: %d", b.vcb_hdr.vcb_output_length);
    pr_dbf_msg("version: %d", b.vcb_hdr.version);
    pr_dbf_msg("stored_vc_count: %d", b.vcb_hdr.stored_vc_count);
    pr_dbf_msg("remaining_vc_count: %d", b.vcb_hdr.remaining_vc_count);
    }
#[no_mangle]
unsafe extern "C" fn pr_dbf_vce(e: *const vce) {
    static void pr_dbf_vce(const struct vce *e)
    {
    unsigned char vc_name[VC_NAME_LEN_BYTES + 1];
    char log_string[VC_NAME_LEN_BYTES + 40];
    pr_dbf_msg("VCE Header:");
    pr_dbf_msg("vce_hdr.vce_length: %d", e.vce_hdr.vce_length);
    pr_dbf_msg("vce_hdr.flags: %d", e.vce_hdr.flags);
    pr_dbf_msg("vce_hdr.key_type: %d", e.vce_hdr.key_type);
    pr_dbf_msg("vce_hdr.vc_index: %d", e.vce_hdr.vc_index);
    pr_dbf_msg("vce_hdr.vc_format: %d", e.vce_hdr.vc_format);
    pr_dbf_msg("vce_hdr.key_id_length: %d", e.vce_hdr.key_id_length);
    pr_dbf_msg("vce_hdr.vc_hash_type: %d", e.vce_hdr.vc_hash_type);
    pr_dbf_msg("vce_hdr.vc_hash_length: %d", e.vce_hdr.vc_hash_length);
    pr_dbf_msg("vce_hdr.vc_hash_offset: %d", e.vce_hdr.vc_hash_offset);
    pr_dbf_msg("vce_hdr.vc_length: %d", e.vce_hdr.vc_length);
    pr_dbf_msg("vce_hdr.vc_offset: %d", e.vce_hdr.vc_offset);
// Certificate name in ASCII.
    memcpy(vc_name, e.vce_hdr.vc_name, VC_NAME_LEN_BYTES);
    EBCASC_500(vc_name, VC_NAME_LEN_BYTES);
    vc_name[VC_NAME_LEN_BYTES] = '\0';
    snprintf(log_string, sizeof(log_string),
    "index: %d vce_hdr.vc_name (ASCII): %s",
    e.vce_hdr.vc_index, vc_name);
    debug_text_event(cert_store_hexdump, 3, log_string);
// Certificate data.
    debug_text_event(cert_store_hexdump, 3, "VCE: Certificate data start");
    debug_event(cert_store_hexdump, 3, (u8 *)e.cert_data_buf, 128);
    debug_text_event(cert_store_hexdump, 3, "VCE: Certificate data end");
    debug_event(cert_store_hexdump, 3,
    (u8 *)e.cert_data_buf + e.vce_hdr.vce_length - 128, 128);
    }
#[no_mangle]
unsafe extern "C" fn pr_dbf_vcssb(s: *const vcssb) {
    static void pr_dbf_vcssb(const struct vcssb *s)
    {
    debug_text_event(cert_store_hexdump, 3, "DIAG320 Subcode1");
    debug_event(cert_store_hexdump, 3, (u8 *)s, VCSSB_LEN_BYTES);
    pr_dbf_msg("VCSSB:");
    pr_dbf_msg("vcssb_length: %u", s.vcssb_length);
    pr_dbf_msg("version: %u", s.version);
    pr_dbf_msg("cs_token: %u", s.cs_token);
    pr_dbf_msg("total_vc_index_count: %u", s.total_vc_index_count);
    pr_dbf_msg("max_vc_index_count: %u", s.max_vc_index_count);
    pr_dbf_msg("max_vce_length: %u", s.max_vce_length);
    pr_dbf_msg("max_vcxe_length: %u", s.max_vce_length);
    pr_dbf_msg("max_single_vcb_length: %u", s.max_single_vcb_length);
    pr_dbf_msg("total_vcb_length: %u", s.total_vcb_length);
    pr_dbf_msg("max_single_vcxb_length: %u", s.max_single_vcxb_length);
    pr_dbf_msg("total_vcxb_length: %u", s.total_vcxb_length);
    }
#[no_mangle]
unsafe extern "C" fn __diag320(subcode: c_ulong, addr: *mut c_void) -> c_int {
    static int __diag320(unsigned long subcode, void *addr)
    {
    let mut rp: union register_pair = { .even = (unsigned long)addr, };
    asm_inline volatile(
    "	diag	%[rp],%[subcode],0x320\n"
    "0:	nopr	%%r7\n"
    EX_TABLE(0b, 0b)
    : [rp] "+d" (rp.pair)
    : [subcode] "d" (subcode)
    : "cc", "memory");
    return rp.odd;
    }
#[no_mangle]
unsafe extern "C" fn diag320(subcode: c_ulong, addr: *mut c_void) -> c_int {
    static int diag320(unsigned long subcode, void *addr)
    {
    diag_stat_inc(DIAG_STAT_X320);
    return __diag320(subcode, addr);
    }
//
// Calculate SHA256 hash of the VCE certificate and compare it to hash stored in
// VCE. Return -EINVAL if hashes don't match.
//
#[no_mangle]
unsafe extern "C" fn check_certificate_hash(vce: *const vce) -> c_int {
    static int check_certificate_hash(const struct vce *vce)
    {
    u8 hash[SHA256_DIGEST_SIZE];
    u16 vc_hash_length;
    u8 *vce_hash;
    vce_hash = (u8 *)vce + vce.vce_hdr.vc_hash_offset;
    vc_hash_length = vce.vce_hdr.vc_hash_length;
    sha256((u8 *)vce + vce.vce_hdr.vc_offset, vce.vce_hdr.vc_length, hash);
    if (memcmp(vce_hash, hash, vc_hash_length) == 0)
    return 0;
    pr_dbf_msg("SHA256 hash of received certificate does not match");
    debug_text_event(cert_store_hexdump, 3, "VCE hash:");
    debug_event(cert_store_hexdump, 3, vce_hash, SHA256_DIGEST_SIZE);
    debug_text_event(cert_store_hexdump, 3, "Calculated hash:");
    debug_event(cert_store_hexdump, 3, hash, SHA256_DIGEST_SIZE);
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn check_certificate_valid(vce: *const vce) -> c_int {
    static int check_certificate_valid(const struct vce *vce)
    {
    if (!(vce.vce_hdr.flags & VCE_FLAGS_VALID_MASK)) {
    pr_dbf_msg("Certificate entry is invalid");
    return -EINVAL;
    }
    if (vce.vce_hdr.vc_format != 1) {
    pr_dbf_msg("Certificate format is not supported");
    return -EINVAL;
    }
    if (vce.vce_hdr.vc_hash_type != 1) {
    pr_dbf_msg("Hash type is not supported");
    return -EINVAL;
    }
    return check_certificate_hash(vce);
    }
    static struct key *get_user_session_keyring(void)
    {
    key_ref_t us_keyring_ref;
    us_keyring_ref = lookup_user_key(KEY_SPEC_USER_SESSION_KEYRING,
    KEY_LOOKUP_CREATE, KEY_NEED_LINK);
    if (IS_ERR(us_keyring_ref)) {
    pr_dbf_msg("Couldn't get user session keyring: %ld",
    PTR_ERR(us_keyring_ref));
    return ERR_PTR(-ENOKEY);
    }
    key_ref_put(us_keyring_ref);
    return key_ref_to_ptr(us_keyring_ref);
    }
// Invalidate all keys from cert_store keyring.
#[no_mangle]
unsafe extern "C" fn invalidate_keyring_keys(keyring: *mut key) -> c_int {
    static int invalidate_keyring_keys(struct key *keyring)
    {
    unsigned long num_keys, key_index;
    size_t keyring_payload_len;
    key_serial_t *key_array;
    struct key *current_key;
    int rc;
    keyring_payload_len = key_type_keyring.read(keyring, core::ptr::null_mut(), 0);
    num_keys = keyring_payload_len / sizeof(key_serial_t);
    key_array = kzalloc_objs(key_serial_t, num_keys);
    if (!key_array)
    return -ENOMEM;
    rc = key_type_keyring.read(keyring, (char *)key_array, keyring_payload_len);
    if (rc != keyring_payload_len) {
    pr_dbf_msg("Couldn't read keyring payload");
    goto out;
    }
    for (key_index = 0; key_index < num_keys; key_index++) {
    current_key = key_lookup(key_array[key_index]);
    pr_dbf_msg("Invalidating key %08x", current_key.serial);
    key_invalidate(current_key);
    key_put(current_key);
    rc = key_unlink(keyring, current_key);
    if (rc) {
    pr_dbf_msg("Couldn't unlink key %08x: %d", current_key.serial, rc);
    break;
    }
    }
    out:
    kfree(key_array);
    return rc;
    }
    static struct key *find_cs_keyring(void)
    {
    key_ref_t cs_keyring_ref;
    struct key *cs_keyring;
    cs_keyring_ref = keyring_search(make_key_ref(get_user_session_keyring(), true),
    &key_type_keyring, CERT_STORE_KEYRING_NAME,
    false);
    if (!IS_ERR(cs_keyring_ref)) {
    cs_keyring = key_ref_to_ptr(cs_keyring_ref);
    key_ref_put(cs_keyring_ref);
    goto found;
    }
// Search default locations: thread, process, session keyrings
    cs_keyring = request_key(&key_type_keyring, CERT_STORE_KEYRING_NAME, core::ptr::null_mut());
    if (IS_ERR(cs_keyring))
    return core::ptr::null_mut();
    key_put(cs_keyring);
    found:
    return cs_keyring;
    }
#[no_mangle]
unsafe extern "C" fn cleanup_cs_keys() {
    static void cleanup_cs_keys(void)
    {
    struct key *cs_keyring;
    cs_keyring = find_cs_keyring();
    if (!cs_keyring)
    return;
    pr_dbf_msg("Found cert_store keyring. Purging...");
//
// Remove cert_store_key_type in case invalidation
// of old cert_store keys failed (= severe error).
//
    if (invalidate_keyring_keys(cs_keyring))
    unregister_key_type(&key_type_cert_store_key);
    keyring_clear(cs_keyring);
    key_invalidate(cs_keyring);
    key_put(cs_keyring);
    key_unlink(get_user_session_keyring(), cs_keyring);
    }
    static struct key *create_cs_keyring(void)
    {
    static struct key *cs_keyring;
// Cleanup previous cs_keyring and all associated keys if any.
    cleanup_cs_keys();
    cs_keyring = keyring_alloc(CERT_STORE_KEYRING_NAME, GLOBAL_ROOT_UID,
    GLOBAL_ROOT_GID, current_cred(),
    (KEY_POS_ALL & ~KEY_POS_SETATTR) | KEY_USR_VIEW | KEY_USR_READ,
    KEY_ALLOC_NOT_IN_QUOTA | KEY_ALLOC_SET_KEEP,
    core::ptr::null_mut(), get_user_session_keyring());
    if (IS_ERR(cs_keyring)) {
    pr_dbf_msg("Can't allocate cert_store keyring");
    return core::ptr::null_mut();
    }
    pr_dbf_msg("Successfully allocated cert_store keyring: %08x", cs_keyring.serial);
//
// In case a previous clean-up ran into an
// error and unregistered key type.
//
    register_key_type(&key_type_cert_store_key);
    return cs_keyring;
    }
//
// Allocate memory and create key description in format
// [key name in EBCDIC]:[VCE index]:[CS token].
// Return a pointer to key description or NULL if memory
// allocation failed. Memory should be freed by caller.
//
    static char *get_key_description(struct vcssb *vcssb, const struct vce *vce)
    {
    size_t len, name_len;
    u32 cs_token;
    char *desc;
    cs_token = vcssb.cs_token;
// Description string contains "%64s:%05u:%010u\0".
    name_len = sizeof(vce.vce_hdr.vc_name);
    len = name_len + 1 + 5 + 1 + 10 + 1;
    desc = kmalloc(len, GFP_KERNEL);
    if (!desc)
    return core::ptr::null_mut();
    memcpy(desc, vce.vce_hdr.vc_name, name_len);
    snprintf(desc + name_len, len - name_len, ":%05u:%010u",
    vce.vce_hdr.vc_index, cs_token);
    return desc;
    }
//
// Create a key of type "cert_store_key" using the data from VCE for key
// payload and key description. Link the key to "cert_store" keyring.
//
    static int create_key_from_vce(struct vcssb *vcssb, struct vce *vce,
    struct key *keyring)
    {
    key_ref_t newkey;
    char *desc;
    int rc;
    desc = get_key_description(vcssb, vce);
    if (!desc)
    return -ENOMEM;
    newkey = key_create_or_update(
    make_key_ref(keyring, true), CERT_STORE_KEY_TYPE_NAME,
    desc, (u8 *)vce + vce.vce_hdr.vc_offset,
    vce.vce_hdr.vc_length,
    (KEY_POS_ALL & ~KEY_POS_SETATTR)  | KEY_USR_VIEW | KEY_USR_READ,
    KEY_ALLOC_NOT_IN_QUOTA);
    rc = PTR_ERR_OR_ZERO(newkey);
    if (rc) {
    pr_dbf_msg("Couldn't create a key from Certificate Entry (%d)", rc);
    rc = -ENOKEY;
    goto out;
    }
    key_ref_put(newkey);
    out:
    kfree(desc);
    return rc;
    }
// Get Verification Certificate Storage Size block with DIAG320 subcode2.
#[no_mangle]
unsafe extern "C" fn get_vcssb(vcssb: *mut vcssb) -> c_int {
    static int get_vcssb(struct vcssb *vcssb)
    {
    int diag320_rc;
    memset(vcssb, 0, sizeof(*vcssb));
    vcssb.vcssb_length = VCSSB_LEN_BYTES;
    diag320_rc = diag320(DIAG320_STORAGE, vcssb);
    pr_dbf_vcssb(vcssb);
    if (diag320_rc != DIAG320_RC_OK) {
    pr_dbf_msg("Diag 320 Subcode 1 returned bad RC: %04x", diag320_rc);
    return -EIO;
    }
    if (vcssb.vcssb_length == VCSSB_LEN_NO_CERTS) {
    pr_dbf_msg("No certificates available for current configuration");
    return -ENOKEY;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_4k_mult_vcb_size(vcssb: *mut vcssb) -> u32 {
    static u32 get_4k_mult_vcb_size(struct vcssb *vcssb)
    {
    return round_up(vcssb.max_single_vcb_length, PAGE_SIZE);
    }
// Fill input fields of single-entry VCB that will be read by LPAR.
#[no_mangle]
unsafe extern "C" fn fill_vcb_input(vcssb: *mut vcssb, vcb: *mut vcb, index: u16) {
    static void fill_vcb_input(struct vcssb *vcssb, struct vcb *vcb, u16 index)
    {
    memset(vcb, 0, sizeof(*vcb));
    vcb.vcb_hdr.vcb_input_length = get_4k_mult_vcb_size(vcssb);
    vcb.vcb_hdr.cs_token = vcssb.cs_token;
// Request single entry.
    vcb.vcb_hdr.first_vc_index = index;
    vcb.vcb_hdr.last_vc_index = index;
    }
#[no_mangle]
unsafe extern "C" fn extract_vce_from_sevcb(vcb: *mut vcb, vce: *mut vce) {
    static void extract_vce_from_sevcb(struct vcb *vcb, struct vce *vce)
    {
    struct vce *extracted_vce;
    extracted_vce = (struct vce *)vcb.vcb_buf;
    memcpy(vce, vcb.vcb_buf, extracted_vce.vce_hdr.vce_length);
    pr_dbf_vce(vce);
    }
#[no_mangle]
unsafe extern "C" fn get_sevcb(vcssb: *mut vcssb, index: u16, vcb: *mut vcb) -> c_int {
    static int get_sevcb(struct vcssb *vcssb, u16 index, struct vcb *vcb)
    {
    int rc, diag320_rc;
    fill_vcb_input(vcssb, vcb, index);
    diag320_rc = diag320(DIAG320_CERT_BLOCK, vcb);
    pr_dbf_msg("Diag 320 Subcode2 RC %2x", diag320_rc);
    pr_dbf_vcb(vcb);
    switch (diag320_rc) {
    case DIAG320_RC_OK:
    rc = 0;
    if (vcb.vcb_hdr.vcb_output_length == VCB_LEN_NO_CERTS) {
    pr_dbf_msg("No certificate entry for index %u", index);
    rc = -ENOKEY;
    } else if (vcb.vcb_hdr.remaining_vc_count != 0) {
// Retry on insufficient space.
    pr_dbf_msg("Couldn't get all requested certificates");
    rc = -EAGAIN;
    }
    break;
    case DIAG320_RC_CS_NOMATCH:
    pr_dbf_msg("Certificate Store token mismatch");
    rc = -EAGAIN;
    break;
    default:
    pr_dbf_msg("Diag 320 Subcode2 returned bad rc (0x%4x)", diag320_rc);
    rc = -EINVAL;
    break;
    }
    return rc;
    }
//
// Allocate memory for single-entry VCB, get VCB via DIAG320 subcode 2 call,
// extract VCE and create a key from its' certificate.
//
    static int create_key_from_sevcb(struct vcssb *vcssb, u16 index,
    struct key *keyring)
    {
    struct vcb *vcb;
    struct vce *vce;
    int rc;
    rc = -ENOMEM;
    vcb = vmalloc(get_4k_mult_vcb_size(vcssb));
    vce = vmalloc(vcssb.max_single_vcb_length - sizeof(vcb.vcb_hdr));
    if (!vcb || !vce)
    goto out;
    rc = get_sevcb(vcssb, index, vcb);
    if (rc)
    goto out;
    extract_vce_from_sevcb(vcb, vce);
    rc = check_certificate_valid(vce);
    if (rc)
    goto out;
    rc = create_key_from_vce(vcssb, vce, keyring);
    if (rc)
    goto out;
    pr_dbf_msg("Successfully created key from Certificate Entry %d", index);
    out:
    vfree(vce);
    vfree(vcb);
    return rc;
    }
//
// Request a single-entry VCB for each VCE available for the partition.
// Create a key from it and link it to cert_store keyring. If no keys
// could be created (i.e. VCEs were invalid) return -ENOKEY.
//
#[no_mangle]
unsafe extern "C" fn add_certificates_to_keyring(vcssb: *mut vcssb, keyring: *mut key) -> c_int {
    static int add_certificates_to_keyring(struct vcssb *vcssb, struct key *keyring)
    {
    int rc, index, count, added;
    count = 0;
    added = 0;
// Certificate Store entries indices start with 1 and have no gaps.
    for (index = 1; index < vcssb.total_vc_index_count + 1; index++) {
    pr_dbf_msg("Creating key from VCE %u", index);
    rc = create_key_from_sevcb(vcssb, index, keyring);
    count++;
    if (rc == -EAGAIN)
    return rc;
    if (rc)
    pr_dbf_msg("Creating key from VCE %u failed (%d)", index, rc);
    else
    added++;
    }
    if (added == 0) {
    pr_dbf_msg("Processed %d entries. No keys created", count);
    return -ENOKEY;
    }
    pr_info("Added %d of %d keys to cert_store keyring", added, count);
//
// Do not allow to link more keys to certificate store keyring after all
// the VCEs were processed.
//
    rc = keyring_restrict(make_key_ref(keyring, true), core::ptr::null_mut(), core::ptr::null_mut());
    if (rc)
    pr_dbf_msg("Failed to set restriction to cert_store keyring (%d)", rc);
    return 0;
    }
//
// Check which DIAG320 subcodes are installed.
// Return -ENOENT if subcodes 1 or 2 are not available.
//
#[no_mangle]
unsafe extern "C" fn query_diag320_subcodes() -> c_int {
    static int query_diag320_subcodes(void)
    {
    unsigned long ism[ISM_LEN_DWORDS];
    int rc;
    rc = diag320(0, ism);
    if (rc != DIAG320_RC_OK) {
    pr_dbf_msg("DIAG320 subcode query returned %04x", rc);
    return -ENOENT;
    }
    debug_text_event(cert_store_hexdump, 3, "DIAG320 Subcode 0");
    debug_event(cert_store_hexdump, 3, ism, sizeof(ism));
    if (!test_bit_inv(1, ism) || !test_bit_inv(2, ism)) {
    pr_dbf_msg("Not all required DIAG320 subcodes are installed");
    return -ENOENT;
    }
    return 0;
    }
//
// Check if Certificate Store is supported by the firmware and DIAG320 subcodes
// 1 and 2 are installed. Create cert_store keyring and link all certificates
// available for the current partition to it as "cert_store_key" type
// keys. On refresh or error invalidate cert_store keyring and destroy
// all keys of "cert_store_key" type.
//
#[no_mangle]
unsafe extern "C" fn fill_cs_keyring() -> c_int {
    static int fill_cs_keyring(void)
    {
    struct key *cs_keyring;
    struct vcssb *vcssb;
    int rc;
    rc = -ENOMEM;
    vcssb = kmalloc(VCSSB_LEN_BYTES, GFP_KERNEL);
    if (!vcssb)
    goto cleanup_keys;
    rc = -ENOENT;
    if (!sclp.has_diag320) {
    pr_dbf_msg("Certificate Store is not supported");
    goto cleanup_keys;
    }
    rc = query_diag320_subcodes();
    if (rc)
    goto cleanup_keys;
    rc = get_vcssb(vcssb);
    if (rc)
    goto cleanup_keys;
    rc = -ENOMEM;
    cs_keyring = create_cs_keyring();
    if (!cs_keyring)
    goto cleanup_keys;
    rc = add_certificates_to_keyring(vcssb, cs_keyring);
    if (rc)
    goto cleanup_cs_keyring;
    goto out;
    cleanup_cs_keyring:
    key_put(cs_keyring);
    cleanup_keys:
    cleanup_cs_keys();
    out:
    kfree(vcssb);
    return rc;
    }
    static DEFINE_MUTEX(cs_refresh_lock);
    let mut cs_status_val: static int = -1;
    static ssize_t cs_status_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    if (cs_status_val == -1)
    return sysfs_emit(buf, "uninitialized\n");
#[no_mangle]
pub unsafe extern "C" fn if(0: cs_status_val ==) -> else {
    else if (cs_status_val == 0)
    return sysfs_emit(buf, "ok\n");
    return sysfs_emit(buf, "failed (%d)\n", cs_status_val);
    }
    let mut cs_status_attr: static struct kobj_attribute = __ATTR_RO(cs_status);
    static ssize_t refresh_store(struct kobject *kobj, struct kobj_attribute *attr,
    const char *buf, size_t count)
    {
    int rc, retries;
    pr_dbf_msg("Refresh certificate store information requested");
    rc = mutex_lock_interruptible(&cs_refresh_lock);
    if (rc)
    return rc;
    for (retries = 0; retries < DIAG_MAX_RETRIES; retries++) {
// Request certificates from certificate store.
    rc = fill_cs_keyring();
    if (rc)
    pr_dbf_msg("Failed to refresh certificate store information (%d)", rc);
    if (rc != -EAGAIN)
    break;
    }
    cs_status_val = rc;
    mutex_unlock(&cs_refresh_lock);
    return rc ?: count;
    }
    let mut refresh_attr: static struct kobj_attribute = __ATTR_WO(refresh);
    static const struct attribute *cert_store_attrs[] __initconst = {
    &cs_status_attr.attr,
    &refresh_attr.attr,
    core::ptr::null_mut(),
    };
    static struct kobject *cert_store_kobj;
#[no_mangle]
unsafe extern "C" fn cert_store_init() -> int __init {
    static int __init cert_store_init(void)
    {
    let mut rc: c_int = -ENOMEM;
    cert_store_dbf = debug_register("cert_store_msg", 10, 1, 64);
    if (!cert_store_dbf)
    goto cleanup_dbf;
    cert_store_hexdump = debug_register("cert_store_hexdump", 3, 1, 128);
    if (!cert_store_hexdump)
    goto cleanup_dbf;
    debug_register_view(cert_store_hexdump, &debug_hex_ascii_view);
    debug_register_view(cert_store_dbf, &debug_sprintf_view);
// Create directory /sys/firmware/cert_store.
    cert_store_kobj = kobject_create_and_add("cert_store", firmware_kobj);
    if (!cert_store_kobj)
    goto cleanup_dbf;
    rc = sysfs_create_files(cert_store_kobj, cert_store_attrs);
    if (rc)
    goto cleanup_kobj;
    register_key_type(&key_type_cert_store_key);
    return rc;
    cleanup_kobj:
    kobject_put(cert_store_kobj);
    cleanup_dbf:
    debug_unregister(cert_store_dbf);
    debug_unregister(cert_store_hexdump);
    return rc;
    }
    device_initcall(cert_store_init);
