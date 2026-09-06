//! Automatically rewritten from C to Rust
//! Source: drivers/char/tpm/tpm-sysfs.c
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
// Copyright (C) 2004 IBM Corporation
// Authors:
// Leendert van Doorn <leendert@watson.ibm.com>
// Dave Safford <safford@watson.ibm.com>
// Reiner Sailer <sailer@watson.ibm.com>
// Kylene Hall <kjhall@us.ibm.com>
//
// Copyright (C) 2013 Obsidian Research Corp
// Jason Gunthorpe <jgunthorpe@obsidianresearch.com>
//
// sysfs filesystem inspection interface to the TPM
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpm_readpubek_out {
    pub algorithm: [u8; 4],
    pub encscheme: [u8; 2],
    pub sigscheme: [u8; 2],
    pub paramsize: __be32,
    pub parameters: [u8; 12],
    pub keysize: __be32,
    pub modulus: [u8; 256],
    pub checksum: [u8; 20],
    pub __packed: },

pub const TPM_ORD_READPUBEK: c_int = 124;
    static ssize_t pubek_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    pub out: *mut tpm_readpubek_out,
    pub i: c_int,
    pub buf: *mut *mut char str =,
    pub to_tpm_chip(dev): *mut *mut tpm_chip chip =,
    pub anti_replay: [c_char; 20],
    pub NULL: *mut *mut tpm_buf tpm_buf __free(kfree) =,
    pub sizeof(anti_replay)): memset(&anti_replay, 0,,
    if (tpm_try_get_ops(chip))
    pub 0: return,
    pub GFP_KERNEL): tpm_buf = kzalloc(TPM_BUFSIZE,,
    if (!tpm_buf) {
    pub 0: return,
    }
    pub TPM_BUFSIZE): tpm_buf_init(tpm_buf,,
    pub TPM_ORD_READPUBEK): tpm_buf_reset(tpm_buf, TPM_TAG_RQU_COMMAND,,
    pub sizeof(anti_replay)): tpm_buf_append(tpm_buf, anti_replay,,
    if (tpm_transmit_cmd(chip, tpm_buf, READ_PUBEK_RESULT_MIN_BODY_SIZE,
    "attempting to read the PUBEK")) {
    pub 0: return,
    }
    pub )&tpm_buf->data[10]: *mut out = (struct tpm_readpubek_out,
    str +=
    sprintf(str,
    "Algorithm: %4ph\n"
    "Encscheme: %2ph\n"
    "Sigscheme: %2ph\n"
    "Parameters: %12ph\n"
    "Modulus length: %d\n"
    "Modulus:\n",
    out.algorithm,
    out.encscheme,
    out.sigscheme,
    out.parameters,
    pub 16): for (i = 0; i < 256; i +=,
    pub &out->modulus[i]): str += sprintf(str, "%16ph\n",,
    pub buf: return str -,
    }
    pub DEVICE_ATTR_RO(pubek): static,
    static ssize_t pcrs_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    pub cap: cap_t,
    pub digest: [u8; TPM_DIGEST_SIZE],
    pub num_pcrs: u32 i, j,,
    pub buf: *mut *mut char str =,
    pub to_tpm_chip(dev): *mut *mut tpm_chip chip =,
    if (tpm_try_get_ops(chip))
    pub 0: return,
    if (tpm1_getcap(chip, TPM_CAP_PROP_PCR, &cap,
    "attempting to determine the number of PCRS",
    sizeof(cap.num_pcrs))) {
    pub 0: return,
    }
    pub be32_to_cpu(cap.num_pcrs): num_pcrs =,
    pub {: for (i = 0; i < num_pcrs; i++),
    if (tpm1_pcr_read(chip, i, digest)) {
    pub buf: str =,
    }
    pub i): str += sprintf(str, "PCR-%02d: ",,
    pub j++): for (j = 0; j < TPM_DIGEST_SIZE;,
    pub digest[j]): str += sprintf(str, "%02X ",,
    pub "\n"): str += sprintf(str,,
    }
    pub buf: return str -,
    }
    pub DEVICE_ATTR_RO(pcrs): static,
    static ssize_t enabled_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    pub to_tpm_chip(dev): *mut *mut tpm_chip chip =,
    pub 0: ssize_t rc =,
    pub cap: cap_t,
    if (tpm_try_get_ops(chip))
    pub 0: return,
    if (tpm1_getcap(chip, TPM_CAP_FLAG_PERM, &cap,
    "attempting to determine the permanent enabled state",
    sizeof(cap.perm_flags)))
    pub out_ops: goto,
    pub !cap.perm_flags.disable): rc = sprintf(buf, "%d\n",,
    out_ops:
    pub rc: return,
    }
    pub DEVICE_ATTR_RO(enabled): static,
    static ssize_t active_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    pub to_tpm_chip(dev): *mut *mut tpm_chip chip =,
    pub 0: ssize_t rc =,
    pub cap: cap_t,
    if (tpm_try_get_ops(chip))
    pub 0: return,
    if (tpm1_getcap(chip, TPM_CAP_FLAG_PERM, &cap,
    "attempting to determine the permanent active state",
    sizeof(cap.perm_flags)))
    pub out_ops: goto,
    pub !cap.perm_flags.deactivated): rc = sprintf(buf, "%d\n",,
    out_ops:
    pub rc: return,
    }
    pub DEVICE_ATTR_RO(active): static,
    static ssize_t owned_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    pub to_tpm_chip(dev): *mut *mut tpm_chip chip =,
    pub 0: ssize_t rc =,
    pub cap: cap_t,
    if (tpm_try_get_ops(chip))
    pub 0: return,
    if (tpm1_getcap(to_tpm_chip(dev), TPM_CAP_PROP_OWNER, &cap,
    "attempting to determine the owner state",
    sizeof(cap.owned)))
    pub out_ops: goto,
    pub cap.owned): rc = sprintf(buf, "%d\n",,
    out_ops:
    pub rc: return,
    }
    pub DEVICE_ATTR_RO(owned): static,
    static ssize_t temp_deactivated_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    pub to_tpm_chip(dev): *mut *mut tpm_chip chip =,
    pub 0: ssize_t rc =,
    pub cap: cap_t,
    if (tpm_try_get_ops(chip))
    pub 0: return,
    if (tpm1_getcap(to_tpm_chip(dev), TPM_CAP_FLAG_VOL, &cap,
    "attempting to determine the temporary state",
    sizeof(cap.stclear_flags)))
    pub out_ops: goto,
    pub cap.stclear_flags.deactivated): rc = sprintf(buf, "%d\n",,
    out_ops:
    pub rc: return,
    }
    pub DEVICE_ATTR_RO(temp_deactivated): static,
    static ssize_t caps_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    pub to_tpm_chip(dev): *mut *mut tpm_chip chip =,
    pub version: *mut tpm1_version,
    pub 0: ssize_t rc =,
    pub buf: *mut *mut char str =,
    pub cap: cap_t,
    if (tpm_try_get_ops(chip))
    pub 0: return,
    if (tpm1_getcap(chip, TPM_CAP_PROP_MANUFACTURER, &cap,
    "attempting to determine the manufacturer",
    sizeof(cap.manufacturer_id)))
    pub out_ops: goto,
    str += sprintf(str, "Manufacturer: 0x%x\n",
// TPM 1.2
    if (!tpm1_getcap(chip, TPM_CAP_VERSION_1_2, &cap,
    "attempting to determine the 1.2 version",
    sizeof(cap.version2))) {
    pub &cap.version2.version: version =,
    pub out_print: goto,
    }
// TPM 1.1
    if (tpm1_getcap(chip, TPM_CAP_VERSION_1_1, &cap,
    "attempting to determine the 1.1 version",
    sizeof(cap.version1))) {
    pub out_ops: goto,
    }
    pub &cap.version1: version =,
    out_print:
    str += sprintf(str,
    "TCG version: %d.%d\nFirmware version: %d.%d\n",
    version.major, version.minor,
    pub version->rev_minor): version->rev_major,,
    pub buf: rc = str -,
    out_ops:
    pub rc: return,
    }
    pub DEVICE_ATTR_RO(caps): static,
    static ssize_t cancel_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    pub to_tpm_chip(dev): *mut *mut tpm_chip chip =,
    if (tpm_try_get_ops(chip))
    pub 0: return,
    pub count: return,
    }
    pub DEVICE_ATTR_WO(cancel): static,
    static ssize_t durations_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    pub to_tpm_chip(dev): *mut *mut tpm_chip chip =,
    if (chip.duration[TPM_LONG] == 0)
    pub 0: return,
    return sprintf(buf, "%d %d %d [%s]\n",
    jiffies_to_usecs(chip.duration[TPM_SHORT]),
    jiffies_to_usecs(chip.duration[TPM_MEDIUM]),
    jiffies_to_usecs(chip.duration[TPM_LONG]),
    chip.duration_adjusted
    pub "original"): ? "adjusted" :,
    }
    pub DEVICE_ATTR_RO(durations): static,
    static ssize_t timeouts_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    pub to_tpm_chip(dev): *mut *mut tpm_chip chip =,
    return sprintf(buf, "%d %d %d %d [%s]\n",
    jiffies_to_usecs(chip.timeout_a),
    jiffies_to_usecs(chip.timeout_b),
    jiffies_to_usecs(chip.timeout_c),
    jiffies_to_usecs(chip.timeout_d),
    chip.timeout_adjusted
    pub "original"): ? "adjusted" :,
    }
    pub DEVICE_ATTR_RO(timeouts): static,
    static ssize_t tpm_version_major_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    pub to_tpm_chip(dev): *mut *mut tpm_chip chip =,
    return sprintf(buf, "%s\n", chip.flags & TPM_CHIP_FLAG_TPM2
    pub "1"): ? "2" :,
    }
    pub DEVICE_ATTR_RO(tpm_version_major): static,

    static ssize_t null_name_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    pub to_tpm_chip(dev): *mut *mut tpm_chip chip =,
    pub TPM2_NAME_SIZE: int size =,
    pub size): bin2hex(buf, chip->null_key_name,,
    pub 2: *mut *mut size =,
    pub '\n': buf[size++] =,
    pub size: return,
    }
    pub DEVICE_ATTR_RO(null_name): static,

    static struct attribute *tpm1_dev_attrs[] = {
    &dev_attr_pubek.attr,
    &dev_attr_pcrs.attr,
    &dev_attr_enabled.attr,
    &dev_attr_active.attr,
    &dev_attr_owned.attr,
    &dev_attr_temp_deactivated.attr,
    &dev_attr_caps.attr,
    &dev_attr_cancel.attr,
    &dev_attr_durations.attr,
    &dev_attr_timeouts.attr,
    &dev_attr_tpm_version_major.attr,
    core::ptr::null_mut(),
}

    static struct attribute *tpm2_dev_attrs[] = {
    &dev_attr_tpm_version_major.attr,

    &dev_attr_null_name.attr,

    core::ptr::null_mut()
    };
    static const struct attribute_group tpm1_dev_group = {
    .attrs = tpm1_dev_attrs,
    };
    static const struct attribute_group tpm2_dev_group = {
    .attrs = tpm2_dev_attrs,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpm_pcr_attr {
    pub alg_id: c_int,
    pub pcr: c_int,
    pub attr: device_attribute,
}

    static ssize_t pcr_value_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct tpm_pcr_attr *ha = to_tpm_pcr_attr(attr);
    struct tpm_chip *chip = to_tpm_chip(dev);
    struct tpm_digest digest;
    int i;
    let mut digest_size: c_int = 0;
    int rc;
    char *str = buf;
    for (i = 0; i < chip.nr_allocated_banks; i++)
    if (ha.alg_id == chip.allocated_banks[i].alg_id)
    digest_size = chip.allocated_banks[i].digest_size;
// should never happen
    if (!digest_size)
    return -EINVAL;
    digest.alg_id = ha.alg_id;
    rc = tpm_pcr_read(chip, ha.pcr, &digest);
    if (rc)
    return rc;
    for (i = 0; i < digest_size; i++)
    str += sprintf(str, "%02X", digest.digest[i]);
    str += sprintf(str, "\n");
    return str - buf;
    }
//
// The following set of defines represents all the magic to build
// the per hash attribute groups for displaying each bank of PCRs.
// The only slight problem with this approach is that every PCR is
// hard coded to be present, so you don't know if an PCR is missing
// until a cat of the file returns -EINVAL
//
// Also note you must ignore checkpatch warnings in this macro
// code. This is deep macro magic that checkpatch.pl doesn't
// understand.
//
// Note, this must match TPM2_PLATFORM_PCR which is fixed at 24.

    F(_alg, _hash, 0)	    \
    F(_alg, _hash, 1)	    \
    F(_alg, _hash, 2)	    \
    F(_alg, _hash, 3)	    \
    F(_alg, _hash, 4)	    \
    F(_alg, _hash, 5)	    \
    F(_alg, _hash, 6)	    \
    F(_alg, _hash, 7)	    \
    F(_alg, _hash, 8)	    \
    F(_alg, _hash, 9)	    \
    F(_alg, _hash, 10)	    \
    F(_alg, _hash, 11)	    \
    F(_alg, _hash, 12)	    \
    F(_alg, _hash, 13)	    \
    F(_alg, _hash, 14)	    \
    F(_alg, _hash, 15)	    \
    F(_alg, _hash, 16)	    \
    F(_alg, _hash, 17)	    \
    F(_alg, _hash, 18)	    \
    F(_alg, _hash, 19)	    \
    F(_alg, _hash, 20)	    \
    F(_alg, _hash, 21)	    \
    F(_alg, _hash, 22)	    \
    F(_alg, _hash, 23)
// ignore checkpatch warning about trailing ; in macro.

    static struct tpm_pcr_attr dev_attr_pcr_##_hash##_##_pcr = {	\
    .alg_id = _alg,					   \
    .pcr = _pcr,					   \
    .attr = {					   \
    .attr = {				   \
    .name = __stringify(_pcr),	   \
    .mode = 0444			   \
    },					   \
    .show = pcr_value_show			   \
    }						   \
    };

    _TPM_HELPER(_alg, _hash, PCR_ATTR)
// ignore checkpatch warning about trailing , in macro.

    &dev_attr_pcr_##_hash##_##_pcr.attr.attr,

    static struct attribute *pcr_group_attrs_##_hash[] = { \
    _TPM_HELPER(_alg, _hash, PCR_ATTR_VAL)	       \
    core::ptr::null_mut()					       \
    }

    static struct attribute_group pcr_group_##_hash = { \
    .name = "pcr-" __stringify(_hash),	    \
    .attrs = pcr_group_attrs_##_hash	    \
    }

    PCR_ATTRS(_alg, _hash)		   \
    PCR_ATTR_GROUP_ARRAY(_alg, _hash); \
    PCR_ATTR_GROUP(_alg, _hash)
//
// End of macro structure to build an attribute group containing 24
// PCR value files for each supported hash algorithm
//
// The next set of macros implements the cleverness for each hash to
// build a static attribute group called pcr_group_<hash> which can be
// added to chip->groups[].
//
// The first argument is the TPM algorithm id and the second is the
// hash used as both the suffix and the group name.  Note: the group
// name is a directory in the top level tpm class with the name
// pcr-<hash>, so it must not clash with any other names already
// in the sysfs directory.
//
    PCR_ATTR_BUILD(TPM_ALG_SHA1, sha1);
    PCR_ATTR_BUILD(TPM_ALG_SHA256, sha256);
    PCR_ATTR_BUILD(TPM_ALG_SHA384, sha384);
    PCR_ATTR_BUILD(TPM_ALG_SHA512, sha512);
    PCR_ATTR_BUILD(TPM_ALG_SM3_256, sm3);
#[no_mangle]
pub unsafe extern "C" fn tpm_sysfs_add_device(chip: *mut tpm_chip) {
    void tpm_sysfs_add_device(struct tpm_chip *chip)
    {
    int i;
    WARN_ON(chip.groups_cnt != 0);
    if (tpm_is_firmware_upgrade(chip))
    return;
    if (chip.flags & TPM_CHIP_FLAG_TPM2)
    chip.groups[chip.groups_cnt++] = &tpm2_dev_group;
    else
    chip.groups[chip.groups_cnt++] = &tpm1_dev_group;
// add one group for each bank hash
    for (i = 0; i < chip.nr_allocated_banks; i++) {
    switch (chip.allocated_banks[i].alg_id) {
    case TPM_ALG_SHA1:
    chip.groups[chip.groups_cnt++] = &pcr_group_sha1;
    break;
    case TPM_ALG_SHA256:
    chip.groups[chip.groups_cnt++] = &pcr_group_sha256;
    break;
    case TPM_ALG_SHA384:
    chip.groups[chip.groups_cnt++] = &pcr_group_sha384;
    break;
    case TPM_ALG_SHA512:
    chip.groups[chip.groups_cnt++] = &pcr_group_sha512;
    break;
    case TPM_ALG_SM3_256:
    chip.groups[chip.groups_cnt++] = &pcr_group_sm3;
    break;
    default:
//
// If triggers, send a patch to add both a
// PCR_ATTR_BUILD() macro above for the
// missing algorithm as well as an additional
// case in this switch statement.
//
    dev_err(&chip.dev,
    "TPM with unsupported bank algorithm 0x%04x",
    chip.allocated_banks[i].alg_id);
    break;
    }
    }
//
// This will only trigger if someone has added an additional
// hash to the tpm_algorithms enum without incrementing
// TPM_MAX_HASHES.
//
    WARN_ON(chip.groups_cnt > TPM_MAX_HASHES + 1);
    }
