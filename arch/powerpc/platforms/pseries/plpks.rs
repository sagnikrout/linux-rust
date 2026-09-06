//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/pseries/plpks.c
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
// POWER LPAR Platform KeyStore(PLPKS)
// Copyright (C) 2022 IBM Corporation
// Author: Nayna Jain <nayna@linux.ibm.com>
//
// Provides access to variables stored in Power LPAR Platform KeyStore(PLPKS).
//

//
// To 4K align the {input, output} buffers to the {UN}WRAP H_CALLs
//
pub const PLPKS_WRAPPING_BUF_ALIGN: c_int = 4096;
//
// To ensure the output buffer's length is at least 1024 bytes greater
// than the input buffer's length during the WRAP H_CALL
//
pub const PLPKS_WRAPPING_BUF_DIFF: c_int = 1024;
pub const PLPKS_WRAP_INTERFACE_BIT: c_int = 3;
pub const PLPKS_WRAPPING_KEY_LENGTH: c_int = 32;

    BIT_ULL(63 - (be_bit))

    GENMASK_ULL(63 - (be_bit_hi), 63 - (be_bit_lo))

    FIELD_PREP(WRAPFLAG_BE_GENMASK(be_bit_hi, be_bit_lo), (val))

    static u8 *ospassword;
    static u16 ospasswordlength;
// Retrieved with H_PKS_GET_CONFIG
    static u8 version;
    static u16 objoverhead;
    static u16 maxpwsize;
    static u16 maxobjsize;
    static s16 maxobjlabelsize;
    static u32 totalsize;
    static u32 usedspace;
    static u32 supportedpolicies;
    static u32 maxlargeobjectsize;
    static u64 signedupdatealgorithms;
    static u64 wrappingfeatures;
    static bool wrapsupport;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct plpks_auth {
    pub version: u8,
    pub consumer: u8,
    pub rsvd0: __be64,
    pub rsvd1: __be32,
    pub passwordlength: __be16,
    pub password: [u8; ],
    pub __aligned(16): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct label_attr {
    pub prefix: [u8; 8],
    pub version: u8,
    pub os: u8,
    pub length: u8,
    pub reserved: [u8; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct label {
    pub attr: label_attr,
    pub name: [u8; PLPKS_MAX_NAME_SIZE],
    pub size: usize,
}

#[no_mangle]
unsafe extern "C" fn pseries_status_to_err(rc: c_int) -> c_int {
    static int pseries_status_to_err(int rc)
    {
    int err;
    switch (rc) {
    case H_SUCCESS:
    err = 0;
    break;
    case H_FUNCTION:
    err = -ENXIO;
    break;
    case H_PARAMETER:
    case H_P2:
    case H_P3:
    case H_P4:
    case H_P5:
    case H_P6:
    err = -EINVAL;
    break;
    case H_NOT_FOUND:
    err = -ENOENT;
    break;
    case H_BUSY:
    case H_LONG_BUSY_ORDER_1_MSEC:
    case H_LONG_BUSY_ORDER_10_MSEC:
    case H_LONG_BUSY_ORDER_100_MSEC:
    case H_LONG_BUSY_ORDER_1_SEC:
    case H_LONG_BUSY_ORDER_10_SEC:
    case H_LONG_BUSY_ORDER_100_SEC:
    err = -EBUSY;
    break;
    case H_AUTHORITY:
    err = -EPERM;
    break;
    case H_NO_MEM:
    err = -ENOMEM;
    break;
    case H_RESOURCE:
    err = -EEXIST;
    break;
    case H_TOO_BIG:
    err = -EFBIG;
    break;
    case H_STATE:
    err = -EIO;
    break;
    case H_R_STATE:
    err = -EIO;
    break;
    case H_IN_USE:
    err = -EEXIST;
    break;
    case H_ABORTED:
    err = -EIO;
    break;
    default:
    err = -EINVAL;
    }
    pr_debug("Converted hypervisor code %d to Linux %d\n", rc, err);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn plpks_gen_password() -> c_int {
    static int plpks_gen_password(void)
    {
    unsigned long retbuf[PLPAR_HCALL_BUFSIZE] = { 0 };
    u8 *password, consumer = PLPKS_OS_OWNER;
    int rc;
// If we booted from kexec, we could be reusing an existing password already
    if (ospassword) {
    pr_debug("Password of length %u already in use\n", ospasswordlength);
    return 0;
    }
// The password must not cross a page boundary, so we align to the next power of 2
    password = kzalloc(roundup_pow_of_two(maxpwsize), GFP_KERNEL);
    if (!password)
    return -ENOMEM;
    rc = plpar_hcall(H_PKS_GEN_PASSWORD, retbuf, consumer, 0,
    virt_to_phys(password), maxpwsize);
    if (!rc) {
    ospasswordlength = maxpwsize;
    ospassword = kzalloc(maxpwsize, GFP_KERNEL);
    if (!ospassword) {
    kfree_sensitive(password);
    return -ENOMEM;
    }
    memcpy(ospassword, password, ospasswordlength);
    } else {
    if (rc == H_IN_USE) {
    pr_warn("Password already set - authenticated operations will fail\n");
    rc = 0;
    } else {
    goto out;
    }
    }
    out:
    kfree_sensitive(password);
    return pseries_status_to_err(rc);
    }
    static struct plpks_auth *construct_auth(u8 consumer)
    {
    struct plpks_auth *auth;
    if (consumer > PLPKS_OS_OWNER)
    return ERR_PTR(-EINVAL);
// The auth structure must not cross a page boundary and must be
// 16 byte aligned. We align to the next largest power of 2
    auth = kzalloc(roundup_pow_of_two(struct_size(auth, password, maxpwsize)), GFP_KERNEL);
    if (!auth)
    return ERR_PTR(-ENOMEM);
    auth.version = 1;
    auth.consumer = consumer;
    if (consumer == PLPKS_FW_OWNER || consumer == PLPKS_BOOTLOADER_OWNER)
    return auth;
    memcpy(auth.password, ospassword, ospasswordlength);
    auth.passwordlength = cpu_to_be16(ospasswordlength);
    return auth;
    }
//
// Label is combination of label attributes + name.
// Label attributes are used internally by kernel and not exposed to the user.
//
    static struct label *construct_label(char *component, u8 varos, u8 *name,
    u16 namelen)
    {
    struct label *label;
    let mut slen: usize = 0;
    if (!name || namelen > PLPKS_MAX_NAME_SIZE)
    return ERR_PTR(-EINVAL);
// Support NULL component for signed updates
    if (component) {
    slen = strlen(component);
    if (slen > sizeof(label.attr.prefix))
    return ERR_PTR(-EINVAL);
    }
// The label structure must not cross a page boundary, so we align to the next power of 2
    label = kzalloc(roundup_pow_of_two(sizeof(*label)), GFP_KERNEL);
    if (!label)
    return ERR_PTR(-ENOMEM);
    if (component)
    memcpy(&label.attr.prefix, component, slen);
    label.attr.version = PLPKS_LABEL_VERSION;
    label.attr.os = varos;
    label.attr.length = PLPKS_MAX_LABEL_ATTR_SIZE;
    memcpy(&label.name, name, namelen);
    label.size = sizeof(struct label_attr) + namelen;
    return label;
    }
#[no_mangle]
unsafe extern "C" fn _plpks_get_config() -> c_int {
    static int _plpks_get_config(void)
    {
    unsigned long retbuf[PLPAR_HCALL_BUFSIZE] = { 0 };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct config {
    pub version: u8,
    pub flags: u8,
    pub rsvd0: __be16,
    pub objoverhead: __be16,
    pub maxpwsize: __be16,
    pub maxobjlabelsize: __be16,
    pub maxobjsize: __be16,
    pub totalsize: __be32,
    pub usedspace: __be32,
    pub supportedpolicies: __be32,
    pub maxlargeobjectsize: __be32,
    pub signedupdatealgorithms: __be64,
    pub wrappingfeatures: __be64,
    pub rsvd1: [u8; 476],
    pub config: *mut *mut } __packed,
    pub size: usize,
    pub 0: int rc =,
    pub sizeof(*config): *mut size =,
// Config struct must not cross a page boundary. So long as the struct
// size is a power of 2, this should be fine as alignment is guaranteed
    pub GFP_KERNEL): config = kzalloc(size,,
    if (!config) {
    pub -ENOMEM: rc =,
    pub err: goto,
    }
    pub size): rc = plpar_hcall(H_PKS_GET_CONFIG, retbuf, virt_to_phys(config),,
    if (rc != H_SUCCESS) {
    pub pseries_status_to_err(rc): rc =,
    pub err: goto,
    }
    pub config->version: version =,
    pub be16_to_cpu(config->objoverhead): objoverhead =,
    pub be16_to_cpu(config->maxpwsize): maxpwsize =,
    pub be16_to_cpu(config->maxobjsize): maxobjsize =,
    pub be16_to_cpu(config->maxobjlabelsize): maxobjlabelsize =,
    pub be32_to_cpu(config->totalsize): totalsize =,
    pub be32_to_cpu(config->usedspace): usedspace =,
    pub be32_to_cpu(config->supportedpolicies): supportedpolicies =,
    pub be32_to_cpu(config->maxlargeobjectsize): maxlargeobjectsize =,
    pub be64_to_cpu(config->signedupdatealgorithms): signedupdatealgorithms =,
    pub be64_to_cpu(config->wrappingfeatures): wrappingfeatures =,
    pub PPC_BIT8(PLPKS_WRAP_INTERFACE_BIT): wrapsupport = config->flags &,
// Validate that the numbers we get back match the requirements of the spec
    if (maxpwsize < 32) {
    pub maxpwsize): pr_err("Invalid Max Password Size received from hypervisor (%d < 32)\n",,
    pub -EIO: rc =,
    pub err: goto,
    }
    if (maxobjlabelsize < 255) {
    pr_err("Invalid Max Object Label Size received from hypervisor (%d < 255)\n",
    pub -EIO: rc =,
    pub err: goto,
    }
    if (totalsize < 4096) {
    pub totalsize): pr_err("Invalid Total Size received from hypervisor (%d < 4096)\n",,
    pub -EIO: rc =,
    pub err: goto,
    }
    if (version >= 3 && maxlargeobjectsize >= 65536 && maxobjsize != 0xFFFF) {
    pub maxobjsize): pr_err("Invalid Max Object Size (0x%x != 0xFFFF)\n",,
    pub -EIO: rc =,
    pub err: goto,
    }
    err:
    pub rc: return,
    }
//
// plpks_get_version() - Get the version of the PLPKS config structure.
//
// Successful execution of the H_PKS_GET_CONFIG HCALL during initialization
// reads the PLPKS config structure version and saves it in a file local static
// version variable.
//
// Returns: On success the saved PLPKS config structure version is returned, 0
// if not.
//
#[no_mangle]
pub unsafe extern "C" fn plpks_get_version() -> u8 {
    u8 plpks_get_version(void)
    {
    pub version: return,
    }
//
// plpks_get_objoverhead() - Get the hypervisor storage overhead per object.
//
// Successful execution of the H_PKS_GET_CONFIG HCALL during initialization
// reads the per object hypervisor storage overhead in bytes into the local
// static objoverhead variable, excluding the size of the object or the label.
// This value can be treated as valid only when the PLPKS config structure
// version >= 2.
//
// Returns: If PLPKS config structure version >= 2 then the storage overhead is
// returned, 0 otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn plpks_get_objoverhead() -> u16 {
    u16 plpks_get_objoverhead(void)
    {
    pub objoverhead: return,
    }
//
// plpks_get_maxpwsize() - Get the maximum password size.
//
// Successful execution of the H_PKS_GET_CONFIG HCALL during initialization
// reads the maximum password size and checks if it is 32 bytes at the least
// before storing it in the local static maxpwsize variable.
//
// Returns: On success the maximum password size is returned, 0 if not.
//
#[no_mangle]
pub unsafe extern "C" fn plpks_get_maxpwsize() -> u16 {
    u16 plpks_get_maxpwsize(void)
    {
    pub maxpwsize: return,
    }
//
// plpks_get_maxobjectsize() - Get the maximum object size supported by the
// PLPKS.
//
// Successful execution of the H_PKS_GET_CONFIG HCALL during initialization
// reads the maximum object size into the file local static maxobjsize variable.
//
// Returns: On success the maximum object size is returned, 0 if not.
//
#[no_mangle]
pub unsafe extern "C" fn plpks_get_maxobjectsize() -> u16 {
    u16 plpks_get_maxobjectsize(void)
    {
    pub maxobjsize: return,
    }
//
// plpks_get_maxobjectlabelsize() - Get the maximum object label size supported
// by the PLPKS.
//
// Successful execution of the H_PKS_GET_CONFIG HCALL during initialization
// reads the maximum object label size into the local static maxobjlabelsize
// variable.
//
// Returns: On success the maximum object label size is returned, 0 if not.
//
#[no_mangle]
pub unsafe extern "C" fn plpks_get_maxobjectlabelsize() -> u16 {
    u16 plpks_get_maxobjectlabelsize(void)
    {
    pub maxobjlabelsize: return,
    }
//
// plpks_get_totalsize() - Get the total size of the PLPKS that is configured.
//
// Successful execution of the H_PKS_GET_CONFIG HCALL during initialization
// reads the total size of the PLPKS that is configured for the LPAR into the
// file local static totalsize variable.
//
// Returns: On success the total size of the PLPKS configured is returned, 0 if
// not.
//
#[no_mangle]
pub unsafe extern "C" fn plpks_get_totalsize() -> u32 {
    u32 plpks_get_totalsize(void)
    {
    pub totalsize: return,
    }
//
// plpks_get_usedspace() - Get the used space from the total size of the PLPKS.
//
// Invoke the H_PKS_GET_CONFIG HCALL to refresh the latest value for the used
// space as this keeps changing with the creation and removal of objects in the
// PLPKS.
//
// Returns: On success the used space is returned, 0 if not.
//
#[no_mangle]
pub unsafe extern "C" fn plpks_get_usedspace() -> u32 {
    u32 plpks_get_usedspace(void)
    {
    pub _plpks_get_config(): int rc =,
    if (rc) {
    pub rc): pr_err("Couldn't get config, rc: %d\n",,
    pub 0: return,
    }
    pub usedspace: return,
    }
//
// plpks_get_supportedpolicies() - Get a bitmask of the policies supported by
// the hypervisor.
//
// Successful execution of the H_PKS_GET_CONFIG HCALL during initialization
// reads a bitmask of the policies supported by the hypervisor into the file
// local static supportedpolicies variable.
//
// Returns: On success the bitmask of the policies supported by the hypervisor
// are returned, 0 if not.
//
#[no_mangle]
pub unsafe extern "C" fn plpks_get_supportedpolicies() -> u32 {
    u32 plpks_get_supportedpolicies(void)
    {
    pub supportedpolicies: return,
    }
//
// plpks_get_maxlargeobjectsize() - Get the maximum object size supported for
// PLPKS config structure version >= 3
//
// Successful execution of the H_PKS_GET_CONFIG HCALL during initialization
// reads the maximum object size into the local static maxlargeobjectsize
// variable for PLPKS config structure version >= 3. This was introduced
// starting with PLPKS config structure version 3 to allow for objects of
// size >= 64K.
//
// Returns: If PLPKS config structure version >= 3 then the new maximum object
// size is returned, 0 if not.
//
#[no_mangle]
pub unsafe extern "C" fn plpks_get_maxlargeobjectsize() -> u32 {
    u32 plpks_get_maxlargeobjectsize(void)
    {
    pub maxlargeobjectsize: return,
    }
//
// plpks_get_signedupdatealgorithms() - Get a bitmask of the signature
// algorithms supported for signed updates.
//
// Successful execution of the H_PKS_GET_CONFIG HCALL during initialization
// reads a bitmask of the signature algorithms supported for signed updates into
// the file local static signedupdatealgorithms variable. This is valid only
// when the PLPKS config structure version >= 3.
//
// Returns: On success the bitmask of the signature algorithms supported for
// signed updates is returned, 0 if not.
//
#[no_mangle]
pub unsafe extern "C" fn plpks_get_signedupdatealgorithms() -> u64 {
    u64 plpks_get_signedupdatealgorithms(void)
    {
    pub signedupdatealgorithms: return,
    }
//
// plpks_get_wrappingfeatures() - Returns a bitmask of the wrapping features
// supported by the hypervisor.
//
// Successful execution of the H_PKS_GET_CONFIG HCALL during initialization
// reads a bitmask of the wrapping features supported by the hypervisor into the
// file local static wrappingfeatures variable. This is valid only when the
// PLPKS config structure version >= 3.
//
// Return:
// bitmask of the wrapping features supported by the hypervisor
//
#[no_mangle]
pub unsafe extern "C" fn plpks_get_wrappingfeatures() -> u64 {
    u64 plpks_get_wrappingfeatures(void)
    {
    pub wrappingfeatures: return,
    }
//
// plpks_get_passwordlen() - Get the length of the PLPKS password in bytes.
//
// The H_PKS_GEN_PASSWORD HCALL makes the hypervisor generate a random password
// for the specified consumer, apply that password to the PLPKS and return it to
// the caller. In this process, the password length for the OS consumer is
// stored in the local static ospasswordlength variable.
//
// Returns: On success the password length for the OS consumer in bytes is
// returned, 0 if not.
//
#[no_mangle]
pub unsafe extern "C" fn plpks_get_passwordlen() -> u16 {
    u16 plpks_get_passwordlen(void)
    {
    pub ospasswordlength: return,
    }
//
// plpks_is_available() - Get the PLPKS availability status for the LPAR.
//
// The availability of PLPKS is inferred based upon the successful execution of
// the H_PKS_GET_CONFIG HCALL provided the firmware supports this feature. The
// H_PKS_GET_CONFIG HCALL reads the configuration and status information related
// to the PLPKS. The configuration structure provides a version number to inform
// the caller of the supported features.
//
// Returns: true is returned if PLPKS is available, false if not.
//
#[no_mangle]
pub unsafe extern "C" fn plpks_is_available() -> bool {
    bool plpks_is_available(void)
    {
    pub rc: c_int,
    if (!firmware_has_feature(FW_FEATURE_PLPKS))
    pub false: return,
    pub _plpks_get_config(): rc =,
    if (rc)
    pub false: return,
    pub true: return,
    }
    static int plpks_confirm_object_flushed(struct label *label,
    struct plpks_auth *auth)
    {
    pub }: unsigned long retbuf[PLPAR_HCALL_BUFSIZE] = { 0,
    pub true: bool timed_out =,
    pub 0: u64 timeout =,
    pub status: u8,
    pub rc: c_int,
    do {
    rc = plpar_hcall(H_PKS_CONFIRM_OBJECT_FLUSHED, retbuf,
    virt_to_phys(auth), virt_to_phys(label),
    pub retbuf: [status =; 0],
    if (rc) {
    pub false: timed_out =,
    if (rc == H_NOT_FOUND && status == 1)
    pub 0: rc =,
    }
    if (!rc && status == 1) {
    pub false: timed_out =,
    }
    pub PLPKS_FLUSH_SLEEP: timeout = timeout +,
    pub PLPKS_MAX_TIMEOUT): } while (timeout <,
    if (timed_out)
    pub -ETIMEDOUT: return,
    pub pseries_status_to_err(rc): return,
    }
//
// plpks_signed_update_var() - Update the specified authenticated variable.
// @var: authenticated variable to be updated
// @flags: signed update request operation flags
//
// The H_PKS_SIGNED_UPDATE HCALL performs a signed update to an object in the
// PLPKS. The object must have the signed update policy flag set.
//
// Possible reasons for the returned errno values:
//
// -ENXIO	if PLPKS is not supported
// -EIO		if PLPKS access is blocked due to the LPAR's state
// if PLPKS modification is blocked due to the LPAR's state
// if an error occurred while processing the request
// -EINVAL	if invalid authorization parameter
// if invalid object label parameter
// if invalid object label len parameter
// if invalid or unsupported policy declaration
// if invalid signed update flags
// if invalid input data parameter
// if invalid input data len parameter
// if invalid continue token parameter
// -EPERM	if access is denied
// -ENOMEM	if there is inadequate memory to perform the operation
// -EBUSY	if unable to handle the request or long running operation
// initiated, retry later
//
// Returns: On success 0 is returned, a negative errno if not.
//
#[no_mangle]
pub unsafe extern "C" fn plpks_signed_update_var(var: *mut plpks_var, flags: u64) -> c_int {
    int plpks_signed_update_var(struct plpks_var *var, u64 flags)
    {
    pub {0}: unsigned long retbuf[PLPAR_HCALL9_BUFSIZE] =,
    pub rc: c_int,
    pub label: *mut label,
    pub auth: *mut plpks_auth,
    pub 0: u64 continuetoken =,
    pub 0: u64 timeout =,
    if (!var.data || var.datalen <= 0 || var.namelen > PLPKS_MAX_NAME_SIZE)
    pub -EINVAL: return,
    if (!(var.policy & PLPKS_SIGNEDUPDATE))
    pub -EINVAL: return,
    if (var.policy & PLPKS_WRAPPINGKEY)
    pub -EINVAL: return,
// Signed updates need the component to be NULL.
    if (var.component)
    pub -EINVAL: return,
    pub construct_auth(PLPKS_OS_OWNER): auth =,
    if (IS_ERR(auth))
    pub PTR_ERR(auth): return,
    pub var->namelen): label = construct_label(var->component, var->os, var->name,,
    if (IS_ERR(label)) {
    pub PTR_ERR(label): rc =,
    pub out: goto,
    }
    do {
    rc = plpar_hcall9(H_PKS_SIGNED_UPDATE, retbuf,
    virt_to_phys(auth), virt_to_phys(label),
    label.size, var.policy, flags,
    virt_to_phys(var.data), var.datalen,
    pub retbuf: [continuetoken =; 0],
    if (pseries_status_to_err(rc) == -EBUSY) {
    pub 1000: *mut *mut int delay_us = get_longbusy_msecs(rc),
    pub delay_us: timeout +=,
    }
    pub pseries_status_to_err(rc): rc =,
    pub PLPKS_MAX_TIMEOUT): } while (rc == -EBUSY && timeout <,
    if (!rc)
    pub auth): rc = plpks_confirm_object_flushed(label,,
    out:
    pub rc: return,
    }
//
// plpks_write_var() - Write the specified variable and its data to PLPKS.
// @var: variable to be written into the PLPKS
//
// The H_PKS_WRITE_OBJECT HCALL writes an object into the PLPKS. The caller must
// provide a valid component type for the variable, and the signed update policy
// flag must not be set.
//
// Possible reasons for the returned errno values:
//
// -ENXIO	if PLPKS is not supported
// -EIO		if PLPKS access is blocked due to the LPAR's state
// if PLPKS modification is blocked due to the LPAR's state
// if an error occurred while processing the request
// -EINVAL	if invalid authorization parameter
// if invalid object label parameter
// if invalid object label len parameter
// if invalid or unsupported policy declaration
// if invalid input data parameter
// if invalid input data len parameter
// -EPERM	if access is denied
// -ENOMEM	if unable to store the requested object in the space available
// -EBUSY	if unable to handle the request
// -EEXIST	if the object label already exists
//
// Returns: On success 0 is returned, a negative errno if not.
//
#[no_mangle]
pub unsafe extern "C" fn plpks_write_var(var: plpks_var) -> c_int {
    int plpks_write_var(struct plpks_var var)
    {
    pub }: unsigned long retbuf[PLPAR_HCALL_BUFSIZE] = { 0,
    pub auth: *mut plpks_auth,
    pub label: *mut label,
    pub rc: c_int,
    if (!var.component || !var.data || var.datalen <= 0 ||
    var.namelen > PLPKS_MAX_NAME_SIZE || var.datalen > PLPKS_MAX_DATA_SIZE)
    pub -EINVAL: return,
    if (var.policy & PLPKS_SIGNEDUPDATE)
    pub -EINVAL: return,
    if (var.policy & PLPKS_WRAPPINGKEY)
    pub -EINVAL: return,
    pub construct_auth(PLPKS_OS_OWNER): auth =,
    if (IS_ERR(auth))
    pub PTR_ERR(auth): return,
    pub var.namelen): label = construct_label(var.component, var.os, var.name,,
    if (IS_ERR(label)) {
    pub PTR_ERR(label): rc =,
    pub out: goto,
    }
    rc = plpar_hcall(H_PKS_WRITE_OBJECT, retbuf, virt_to_phys(auth),
    virt_to_phys(label), label.size, var.policy,
    pub var.datalen): virt_to_phys(var.data),,
    if (!rc)
    pub auth): rc = plpks_confirm_object_flushed(label,,
    pub pseries_status_to_err(rc): rc =,
    out:
    pub rc: return,
    }
//
// plpks_remove_var() - Remove the specified variable and its data from PLPKS.
// @component: metadata prefix in the object label metadata structure
// @varos: metadata OS flags in the object label metadata structure
// @vname: object label for the object that needs to be removed
//
// The H_PKS_REMOVE_OBJECT HCALL removes an object from the PLPKS. The removal
// is independent of the policy bits that are set.
//
// Possible reasons for the returned errno values:
//
// -ENXIO	if PLPKS is not supported
// -EIO		if PLPKS access is blocked due to the LPAR's state
// if PLPKS modification is blocked due to the LPAR's state
// if an error occurred while processing the request
// -EINVAL	if invalid authorization parameter
// if invalid object label parameter
// if invalid object label len parameter
// -EPERM	if access is denied
// -ENOENT	if the requested object was not found
// -EBUSY	if unable to handle the request
//
// Returns: On success 0 is returned, a negative errno if not.
//
#[no_mangle]
pub unsafe extern "C" fn plpks_remove_var(component: *mut c_char, varos: u8, vname: plpks_var_name) -> c_int {
    int plpks_remove_var(char *component, u8 varos, struct plpks_var_name vname)
    {
    pub }: unsigned long retbuf[PLPAR_HCALL_BUFSIZE] = { 0,
    pub auth: *mut plpks_auth,
    pub label: *mut label,
    pub rc: c_int,
    if (vname.namelen > PLPKS_MAX_NAME_SIZE)
    pub -EINVAL: return,
    pub construct_auth(PLPKS_OS_OWNER): auth =,
    if (IS_ERR(auth))
    pub PTR_ERR(auth): return,
    pub vname.namelen): label = construct_label(component, varos, vname.name,,
    if (IS_ERR(label)) {
    pub PTR_ERR(label): rc =,
    pub out: goto,
    }
    rc = plpar_hcall(H_PKS_REMOVE_OBJECT, retbuf, virt_to_phys(auth),
    pub label->size): virt_to_phys(label),,
    if (!rc)
    pub auth): rc = plpks_confirm_object_flushed(label,,
    pub pseries_status_to_err(rc): rc =,
    out:
    pub rc: return,
    }
#[no_mangle]
unsafe extern "C" fn plpks_read_var(consumer: u8, var: *mut plpks_var) -> c_int {
    static int plpks_read_var(u8 consumer, struct plpks_var *var)
    {
    pub }: unsigned long retbuf[PLPAR_HCALL_BUFSIZE] = { 0,
    pub auth: *mut plpks_auth,
    pub NULL: *mut *mut label label =,
    pub output: *mut u8,
    pub rc: c_int,
    if (var.namelen > PLPKS_MAX_NAME_SIZE)
    pub -EINVAL: return,
    if (var.policy & PLPKS_WRAPPINGKEY)
    pub -EINVAL: return,
    pub construct_auth(consumer): auth =,
    if (IS_ERR(auth))
    pub PTR_ERR(auth): return,
    if (consumer == PLPKS_OS_OWNER) {
    label = construct_label(var.component, var.os, var.name,
    if (IS_ERR(label)) {
    pub PTR_ERR(label): rc =,
    pub out_free_auth: goto,
    }
    }
    pub GFP_KERNEL): output = kzalloc(maxobjsize,,
    if (!output) {
    pub -ENOMEM: rc =,
    pub out_free_label: goto,
    }
    if (consumer == PLPKS_OS_OWNER)
    rc = plpar_hcall(H_PKS_READ_OBJECT, retbuf, virt_to_phys(auth),
    virt_to_phys(label), label.size, virt_to_phys(output),
    else
    rc = plpar_hcall(H_PKS_READ_OBJECT, retbuf, virt_to_phys(auth),
    virt_to_phys(var.name), var.namelen, virt_to_phys(output),
    if (rc != H_SUCCESS) {
    pub pseries_status_to_err(rc): rc =,
    pub out_free_output: goto,
    }
    if (!var.data || var.datalen > retbuf[0])
    pub retbuf: [var->datalen =; 0],
    pub retbuf: [var->policy =; 1],
    if (var.data)
    pub var->datalen): memcpy(var->data, output,,
    pub 0: rc =,
    out_free_output:
    out_free_label:
    out_free_auth:
    pub rc: return,
    }
//
// plpks_wrapping_is_supported() - Get the H_PKS_WRAP_OBJECT interface
// availability status for the LPAR.
//
// Successful execution of the H_PKS_GET_CONFIG HCALL during initialization
// sets bit 3 of the flags variable in the PLPKS config structure if the
// H_PKS_WRAP_OBJECT interface is supported.
//
// Returns: true if the H_PKS_WRAP_OBJECT interface is supported, false if not.
//
#[no_mangle]
pub unsafe extern "C" fn plpks_wrapping_is_supported() -> bool {
    bool plpks_wrapping_is_supported(void)
    {
    pub wrapsupport: return,
    }
//
// plpks_gen_wrapping_key() - Generate a new random key with the 'wrapping key'
// policy set.
//
// The H_PKS_GEN_KEY HCALL makes the hypervisor generate a new random key and
// store the key in a PLPKS object with the provided object label. With the
// 'wrapping key' policy set, only the label to the newly generated random key
// would be visible to the user.
//
// Possible reasons for the returned errno values:
//
// -ENXIO	if PLPKS is not supported
// -EIO		if PLPKS access is blocked due to the LPAR's state
// if PLPKS modification is blocked due to the LPAR's state
// if an error occurred while processing the request
// -EINVAL	if invalid authorization parameter
// if invalid object label parameter
// if invalid object label len parameter
// if invalid or unsupported policy declaration
// if invalid output buffer parameter
// if invalid output buffer length parameter
// -EPERM	if access is denied
// -ENOMEM	if there is inadequate memory to perform this operation
// -EBUSY	if unable to handle the request
// -EEXIST	if the object label already exists
//
// Returns: On success 0 is returned, a negative errno if not.
//
#[no_mangle]
pub unsafe extern "C" fn plpks_gen_wrapping_key() -> c_int {
    int plpks_gen_wrapping_key(void)
    {
    pub }: unsigned long retbuf[PLPAR_HCALL_BUFSIZE] = { 0,
    pub auth: *mut plpks_auth,
    pub label: *mut label,
    pub 0: int rc = 0, pseries_status =,
    struct plpks_var var = {
    .name = PLPKS_WRAPKEY_NAME,
    .namelen = strlen(var.name),
    .policy = PLPKS_WRAPPINGKEY,
    .os = PLPKS_VAR_LINUX,
    .component = PLPKS_WRAPKEY_COMPONENT
}

    auth = construct_auth(PLPKS_OS_OWNER);
    if (IS_ERR(auth))
    return PTR_ERR(auth);
    label = construct_label(var.component, var.os, var.name, var.namelen);
    if (IS_ERR(label)) {
    rc = PTR_ERR(label);
    goto out;
    }
    rc = plpar_hcall(H_PKS_GEN_KEY, retbuf,
    virt_to_phys(auth), virt_to_phys(label),
    label.size, var.policy,
    core::ptr::null_mut(), PLPKS_WRAPPING_KEY_LENGTH);
    if (!rc)
    rc = plpks_confirm_object_flushed(label, auth);
    pseries_status = rc;
    rc = pseries_status_to_err(rc);
    if (rc && rc != -EEXIST) {
    pr_err("H_PKS_GEN_KEY failed. pseries_status=%d, rc=%d",
    pseries_status, rc);
    } else {
    rc = 0;
    }
    kfree(label);
    out:
    kfree(auth);
    return rc;
    }
    EXPORT_SYMBOL_GPL(plpks_gen_wrapping_key);
//
// plpks_wrap_object() - Wrap an object using the default wrapping key stored in
// the PLPKS.
// @input_buf: buffer containing the data to be wrapped
// @input_len: length of the input buffer
// @wrap_flags: object wrapping flags
// @output_buf: buffer to store the wrapped data
// @output_len: length of the output buffer
//
// The H_PKS_WRAP_OBJECT HCALL wraps an object using a wrapping key stored in
// the PLPKS and returns the wrapped object to the caller. The caller provides a
// label to the wrapping key with the 'wrapping key' policy set that must have
// been previously created with the H_PKS_GEN_KEY HCALL. The provided object is
// then encrypted with the wrapping key and additional metadata and the result
// is returned to the user. The metadata includes the wrapping algorithm and the
// wrapping key name so those parameters are not required during unwrap.
//
// Possible reasons for the returned errno values:
//
// -ENXIO	if PLPKS is not supported
// -EIO		if PLPKS access is blocked due to the LPAR's state
// if PLPKS modification is blocked due to the LPAR's state
// if an error occurred while processing the request
// -EINVAL	if invalid authorization parameter
// if invalid wrapping key label parameter
// if invalid wrapping key label length parameter
// if invalid or unsupported object wrapping flags
// if invalid input buffer parameter
// if invalid input buffer length parameter
// if invalid output buffer parameter
// if invalid output buffer length parameter
// if invalid continue token parameter
// if the wrapping key is not compatible with the wrapping
// algorithm
// -EPERM	if access is denied
// -ENOENT	if the requested wrapping key was not found
// -EBUSY	if unable to handle the request or long running operation
// initiated, retry later.
//
// Returns: On success 0 is returned, a negative errno if not.
//
    int plpks_wrap_object(u8 **input_buf, u32 input_len, u16 wrap_flags,
    u8 **output_buf, u32 *output_len)
    {
    unsigned long retbuf[PLPAR_HCALL9_BUFSIZE] = { 0 };
    struct plpks_auth *auth;
    struct label *label;
    let mut continuetoken: u64 = 0;
    let mut objwrapflags: u64 = 0;
    let mut rc: c_int = 0, pseries_status = 0;
    let mut sb_audit_or_enforce_bit: bool = wrap_flags & BIT(0);
    let mut sb_enforce_bit: bool = wrap_flags & BIT(1);
    struct plpks_var var = {
    .name = PLPKS_WRAPKEY_NAME,
    .namelen = strlen(var.name),
    .os = PLPKS_VAR_LINUX,
    .component = PLPKS_WRAPKEY_COMPONENT
    };
    auth = construct_auth(PLPKS_OS_OWNER);
    if (IS_ERR(auth))
    return PTR_ERR(auth);
    label = construct_label(var.component, var.os, var.name, var.namelen);
    if (IS_ERR(label)) {
    rc = PTR_ERR(label);
    goto out;
    }
// Set the consumer password requirement bit. A must have.
    objwrapflags |= WRAPFLAG_BE_BIT_SET(3);
// Set the wrapping algorithm bit. Just one algorithm option for now
    objwrapflags |= WRAPFLAG_BE_FIELD_PREP(60, 63, 0x1);
    if (sb_audit_or_enforce_bit & sb_enforce_bit) {
    pr_err("Cannot set both audit/enforce and enforce bits.");
    rc = -EINVAL;
    goto out_free_label;
    } else if (sb_audit_or_enforce_bit) {
    objwrapflags |= WRAPFLAG_BE_BIT_SET(1);
    } else if (sb_enforce_bit) {
    objwrapflags |= WRAPFLAG_BE_BIT_SET(2);
    }
// output_len = input_len + PLPKS_WRAPPING_BUF_DIFF;
// output_buf = kzalloc(ALIGN(*output_len, PLPKS_WRAPPING_BUF_ALIGN),
    GFP_KERNEL);
    if (!(*output_buf)) {
    pr_err("Output buffer allocation failed. Returning -ENOMEM.");
    rc = -ENOMEM;
    goto out_free_label;
    }
    do {
    rc = plpar_hcall9(H_PKS_WRAP_OBJECT, retbuf,
    virt_to_phys(auth), virt_to_phys(label),
    label.size, objwrapflags,
    virt_to_phys(*input_buf), input_len,
    virt_to_phys(*output_buf), *output_len,
    continuetoken);
    continuetoken = retbuf[0];
    pseries_status = rc;
    rc = pseries_status_to_err(rc);
    } while (rc == -EBUSY);
    if (rc) {
    pr_err("H_PKS_WRAP_OBJECT failed. pseries_status=%d, rc=%d",
    pseries_status, rc);
    kfree(*output_buf);
// output_buf = NULL;
    } else {
// output_len = retbuf[1];
    }
    out_free_label:
    kfree(label);
    out:
    kfree(auth);
    return rc;
    }
    EXPORT_SYMBOL_GPL(plpks_wrap_object);
//
// plpks_unwrap_object() - Unwrap an object using the default wrapping key
// stored in the PLPKS.
// @input_buf: buffer containing the data to be unwrapped
// @input_len: length of the input buffer
// @output_buf: buffer to store the unwrapped data
// @output_len: length of the output buffer
//
// The H_PKS_UNWRAP_OBJECT HCALL unwraps an object that was previously wrapped
// using the H_PKS_WRAP_OBJECT HCALL.
//
// Possible reasons for the returned errno values:
//
// -ENXIO	if PLPKS is not supported
// -EIO		if PLPKS access is blocked due to the LPAR's state
// if PLPKS modification is blocked due to the LPAR's state
// if an error occurred while processing the request
// -EINVAL	if invalid authorization parameter
// if invalid or unsupported object unwrapping flags
// if invalid input buffer parameter
// if invalid input buffer length parameter
// if invalid output buffer parameter
// if invalid output buffer length parameter
// if invalid continue token parameter
// if the wrapping key is not compatible with the wrapping
// algorithm
// if the wrapped object's format is not supported
// if the wrapped object is invalid
// -EPERM	if access is denied
// -ENOENT	if the wrapping key for the provided object was not found
// -EBUSY	if unable to handle the request or long running operation
// initiated, retry later.
//
// Returns: On success 0 is returned, a negative errno if not.
//
    int plpks_unwrap_object(u8 **input_buf, u32 input_len, u8 **output_buf,
    u32 *output_len)
    {
    unsigned long retbuf[PLPAR_HCALL9_BUFSIZE] = { 0 };
    struct plpks_auth *auth;
    let mut continuetoken: u64 = 0;
    let mut objwrapflags: u64 = 0;
    let mut rc: c_int = 0, pseries_status = 0;
    auth = construct_auth(PLPKS_OS_OWNER);
    if (IS_ERR(auth))
    return PTR_ERR(auth);
// output_len = input_len - PLPKS_WRAPPING_BUF_DIFF;
// output_buf = kzalloc(ALIGN(*output_len, PLPKS_WRAPPING_BUF_ALIGN),
    GFP_KERNEL);
    if (!(*output_buf)) {
    pr_err("Output buffer allocation failed. Returning -ENOMEM.");
    rc = -ENOMEM;
    goto out;
    }
    do {
    rc = plpar_hcall9(H_PKS_UNWRAP_OBJECT, retbuf,
    virt_to_phys(auth), objwrapflags,
    virt_to_phys(*input_buf), input_len,
    virt_to_phys(*output_buf), *output_len,
    continuetoken);
    continuetoken = retbuf[0];
    pseries_status = rc;
    rc = pseries_status_to_err(rc);
    } while (rc == -EBUSY);
    if (rc) {
    pr_err("H_PKS_UNWRAP_OBJECT failed. pseries_status=%d, rc=%d",
    pseries_status, rc);
    kfree(*output_buf);
// output_buf = NULL;
    } else {
// output_len = retbuf[1];
    }
    out:
    kfree(auth);
    return rc;
    }
    EXPORT_SYMBOL_GPL(plpks_unwrap_object);
//
// plpks_read_os_var() - Fetch the data for the specified variable that is owned
// by the OS consumer.
// @var: variable to be read from the PLPKS
//
// The consumer or the owner of the object is the os kernel. The
// H_PKS_READ_OBJECT HCALL reads an object from the PLPKS. The caller must
// allocate the buffer var->data and specify the length for this buffer in
// var->datalen. If no buffer is provided, var->datalen will be populated with
// the requested object's size.
//
// Possible reasons for the returned errno values:
//
// -ENXIO	if PLPKS is not supported
// -EIO		if PLPKS access is blocked due to the LPAR's state
// if an error occurred while processing the request
// -EINVAL	if invalid authorization parameter
// if invalid object label parameter
// if invalid object label len parameter
// if invalid output data parameter
// if invalid output data len parameter
// -EPERM	if access is denied
// -ENOENT	if the requested object was not found
// -EFBIG	if the requested object couldn't be
// stored in the buffer provided
// -EBUSY	if unable to handle the request
//
// Returns: On success 0 is returned, a negative errno if not.
//
#[no_mangle]
pub unsafe extern "C" fn plpks_read_os_var(var: *mut plpks_var) -> c_int {
    int plpks_read_os_var(struct plpks_var *var)
    {
    return plpks_read_var(PLPKS_OS_OWNER, var);
    }
//
// plpks_read_fw_var() - Fetch the data for the specified variable that is
// owned by the firmware consumer.
// @var: variable to be read from the PLPKS
//
// The consumer or the owner of the object is the firmware. The
// H_PKS_READ_OBJECT HCALL reads an object from the PLPKS. The caller must
// allocate the buffer var->data and specify the length for this buffer in
// var->datalen. If no buffer is provided, var->datalen will be populated with
// the requested object's size.
//
// Possible reasons for the returned errno values:
//
// -ENXIO	if PLPKS is not supported
// -EIO		if PLPKS access is blocked due to the LPAR's state
// if an error occurred while processing the request
// -EINVAL	if invalid authorization parameter
// if invalid object label parameter
// if invalid object label len parameter
// if invalid output data parameter
// if invalid output data len parameter
// -EPERM	if access is denied
// -ENOENT	if the requested object was not found
// -EFBIG	if the requested object couldn't be
// stored in the buffer provided
// -EBUSY	if unable to handle the request
//
// Returns: On success 0 is returned, a negative errno if not.
//
#[no_mangle]
pub unsafe extern "C" fn plpks_read_fw_var(var: *mut plpks_var) -> c_int {
    int plpks_read_fw_var(struct plpks_var *var)
    {
    return plpks_read_var(PLPKS_FW_OWNER, var);
    }
//
// plpks_read_bootloader_var() - Fetch the data for the specified variable
// owned by the bootloader consumer.
// @var: variable to be read from the PLPKS
//
// The consumer or the owner of the object is the bootloader. The
// H_PKS_READ_OBJECT HCALL reads an object from the PLPKS. The caller must
// allocate the buffer var->data and specify the length for this buffer in
// var->datalen. If no buffer is provided, var->datalen will be populated with
// the requested object's size.
//
// Possible reasons for the returned errno values:
//
// -ENXIO	if PLPKS is not supported
// -EIO		if PLPKS access is blocked due to the LPAR's state
// if an error occurred while processing the request
// -EINVAL	if invalid authorization parameter
// if invalid object label parameter
// if invalid object label len parameter
// if invalid output data parameter
// if invalid output data len parameter
// -EPERM	if access is denied
// -ENOENT	if the requested object was not found
// -EFBIG	if the requested object couldn't be
// stored in the buffer provided
// -EBUSY	if unable to handle the request
//
// Returns: On success 0 is returned, a negative errno if not.
//
#[no_mangle]
pub unsafe extern "C" fn plpks_read_bootloader_var(var: *mut plpks_var) -> c_int {
    int plpks_read_bootloader_var(struct plpks_var *var)
    {
    return plpks_read_var(PLPKS_BOOTLOADER_OWNER, var);
    }
//
// plpks_populate_fdt(): Populates the FDT with the PLPKS password to prepare
// for kexec.
// @fdt: pointer to the device tree blob
//
// Upon confirming the existence of the chosen node, invoke fdt_setprop to
// populate the device tree with the PLPKS password in order to prepare for
// kexec.
//
// Returns: On success 0 is returned, a negative value if not.
//
#[no_mangle]
pub unsafe extern "C" fn plpks_populate_fdt(fdt: *mut c_void) -> c_int {
    int plpks_populate_fdt(void *fdt)
    {
    let mut chosen_offset: c_int = fdt_path_offset(fdt, "/chosen");
    if (chosen_offset < 0) {
    pr_err("Can't find chosen node: %s\n",
    fdt_strerror(chosen_offset));
    return chosen_offset;
    }
    return fdt_setprop(fdt, chosen_offset, "ibm,plpks-pw", ospassword, ospasswordlength);
    }
//
// plpks_early_init_devtree() - Retrieves and clears the PLPKS password from the
// DT in early init.
//
// Once a password is registered with the hypervisor it cannot be cleared
// without rebooting the LPAR, so to keep using the PLPKS across kexec boots we
// need to recover the previous password from the FDT.
//
// There are a few challenges here.  We don't want the password to be visible to
// users, so we need to clear it from the FDT.  This has to be done in early
// boot. Clearing it from the FDT would make the FDT's checksum invalid, so we
// have to manually cause the checksum to be recalculated.
//
#[no_mangle]
pub unsafe extern "C" fn plpks_early_init_devtree() -> void __init {
    void __init plpks_early_init_devtree(void)
    {
    void *fdt = initial_boot_params;
    let mut chosen_node: c_int = fdt_path_offset(fdt, "/chosen");
    const u8 *password;
    int len;
    if (chosen_node < 0)
    return;
    password = fdt_getprop(fdt, chosen_node, "ibm,plpks-pw", &len);
    if (len <= 0) {
    pr_debug("Couldn't find ibm,plpks-pw node.\n");
    return;
    }
    ospassword = memblock_alloc_raw(len, SMP_CACHE_BYTES);
    if (!ospassword) {
    pr_err("Error allocating memory for password.\n");
    goto out;
    }
    memcpy(ospassword, password, len);
    ospasswordlength = (u16)len;
    out:
    fdt_nop_property(fdt, chosen_node, "ibm,plpks-pw");
// Since we've cleared the password, we must update the FDT checksum
    early_init_dt_verify(fdt, __pa(fdt));
    }
#[no_mangle]
unsafe extern "C" fn pseries_plpks_init() -> __init int {
    static __init int pseries_plpks_init(void)
    {
    int rc;
    if (!firmware_has_feature(FW_FEATURE_PLPKS))
    return -ENODEV;
    rc = _plpks_get_config();
    if (rc) {
    pr_err("POWER LPAR Platform KeyStore is not supported or enabled\n");
    return rc;
    }
    rc = plpks_gen_password();
    if (rc)
    pr_err("Failed setting POWER LPAR Platform KeyStore Password\n");
    else
    pr_info("POWER LPAR Platform KeyStore initialized successfully\n");
    return rc;
    }
    machine_arch_initcall(pseries, pseries_plpks_init);
