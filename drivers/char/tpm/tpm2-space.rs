//! Automatically rewritten from C to Rust
//! Source: drivers/char/tpm/tpm2-space.c
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
// Copyright (C) 2016 Intel Corporation
//
// Authors:
// Jarkko Sakkinen <jarkko.sakkinen@linux.intel.com>
//
// Maintained by: <tpmdd-devel@lists.sourceforge.net>
//
// This file contains TPM2 protocol implementations of the commands
// used by the kernel internally.
//

#[no_mangle]
unsafe extern "C" fn tpm2_flush_sessions(chip: *mut tpm_chip, space: *mut tpm_space) {
    static void tpm2_flush_sessions(struct tpm_chip *chip, struct tpm_space *space)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(space.session_tbl); i++) {
    if (space.session_tbl[i])
    tpm2_flush_context(chip, space.session_tbl[i]);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn tpm2_init_space(space: *mut tpm_space, buf_size: c_uint) -> c_int {
    int tpm2_init_space(struct tpm_space *space, unsigned int buf_size)
    {
    space.context_buf = kzalloc(buf_size, GFP_KERNEL);
    if (!space.context_buf)
    return -ENOMEM;
    space.session_buf = kzalloc(buf_size, GFP_KERNEL);
    if (space.session_buf == core::ptr::null_mut()) {
    kfree(space.context_buf);
// Prevent caller getting a dangling pointer.
    space.context_buf = core::ptr::null_mut();
    return -ENOMEM;
    }
    space.buf_size = buf_size;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn tpm2_del_space(chip: *mut tpm_chip, space: *mut tpm_space) {
    void tpm2_del_space(struct tpm_chip *chip, struct tpm_space *space)
    {
    if (tpm_try_get_ops(chip) == 0) {
    tpm2_flush_sessions(chip, space);
    tpm_put_ops(chip);
    }
    kfree(space.context_buf);
    kfree(space.session_buf);
    }
    int tpm2_load_context(struct tpm_chip *chip, u8 *buf,
    unsigned int *offset, u32 *handle)
    {
    struct tpm2_context *ctx;
    unsigned int body_size;
    int rc;
    struct tpm_buf *tbuf __free(kfree) = kzalloc(TPM_BUFSIZE, GFP_KERNEL);
    if (!tbuf)
    return -ENOMEM;
    tpm_buf_init(tbuf, TPM_BUFSIZE);
    tpm_buf_reset(tbuf, TPM2_ST_NO_SESSIONS, TPM2_CC_CONTEXT_LOAD);
    ctx = (struct tpm2_context *)&buf[*offset];
    body_size = sizeof(*ctx) + be16_to_cpu(ctx.blob_size);
    tpm_buf_append(tbuf, &buf[*offset], body_size);
    rc = tpm_transmit_cmd(chip, tbuf, 4, core::ptr::null_mut());
    if (rc < 0) {
    dev_warn(&chip.dev, "%s: failed with a system error %d\n",
    __func__, rc);
    return -EFAULT;
    } else if (tpm2_rc_value(rc) == TPM2_RC_HANDLE ||
    rc == TPM2_RC_REFERENCE_H0) {
//
// TPM_RC_HANDLE means that the session context can't
// be loaded because of an internal counter mismatch
// that makes the TPM think there might have been a
// replay.  This might happen if the context was saved
// and loaded outside the space.
//
// TPM_RC_REFERENCE_H0 means the session has been
// flushed outside the space
//
// handle = 0;
    return -ENOENT;
    } else if (tpm2_rc_value(rc) == TPM2_RC_INTEGRITY) {
    return -EINVAL;
    } else if (rc > 0) {
    dev_warn(&chip.dev, "%s: failed with a TPM error 0x%04X\n",
    __func__, rc);
    return -EFAULT;
    }
// handle = be32_to_cpup((__be32 *)&tbuf->data[TPM_HEADER_SIZE]);
// offset += body_size;
    return 0;
    }
    int tpm2_save_context(struct tpm_chip *chip, u32 handle, u8 *buf,
    unsigned int buf_size, unsigned int *offset)
    {
    unsigned int body_size;
    int rc;
    struct tpm_buf *tbuf __free(kfree) = kzalloc(TPM_BUFSIZE, GFP_KERNEL);
    if (!tbuf)
    return -ENOMEM;
    tpm_buf_init(tbuf, TPM_BUFSIZE);
    tpm_buf_reset(tbuf, TPM2_ST_NO_SESSIONS, TPM2_CC_CONTEXT_SAVE);
    tpm_buf_append_u32(tbuf, handle);
    rc = tpm_transmit_cmd(chip, tbuf, 0, core::ptr::null_mut());
    if (rc < 0) {
    dev_warn(&chip.dev, "%s: failed with a system error %d\n",
    __func__, rc);
    return -EFAULT;
    } else if (tpm2_rc_value(rc) == TPM2_RC_REFERENCE_H0) {
    return -ENOENT;
    } else if (rc) {
    dev_warn(&chip.dev, "%s: failed with a TPM error 0x%04X\n",
    __func__, rc);
    return -EFAULT;
    }
    body_size = tpm_buf_length(tbuf) - TPM_HEADER_SIZE;
    if ((*offset + body_size) > buf_size) {
    dev_warn(&chip.dev, "%s: out of backing storage\n", __func__);
    return -ENOMEM;
    }
    memcpy(&buf[*offset], &tbuf.data[TPM_HEADER_SIZE], body_size);
// offset += body_size;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn tpm2_flush_space(chip: *mut tpm_chip) {
    void tpm2_flush_space(struct tpm_chip *chip)
    {
    struct tpm_space *space = &chip.work_space;
    int i;
    if (!space)
    return;
    for (i = 0; i < ARRAY_SIZE(space.context_tbl); i++)
    if (space.context_tbl[i] && ~space.context_tbl[i])
    tpm2_flush_context(chip, space.context_tbl[i]);
    tpm2_flush_sessions(chip, space);
    }
#[no_mangle]
unsafe extern "C" fn tpm2_load_space(chip: *mut tpm_chip) -> c_int {
    static int tpm2_load_space(struct tpm_chip *chip)
    {
    struct tpm_space *space = &chip.work_space;
    unsigned int offset;
    int i;
    int rc;
    for (i = 0, offset = 0; i < ARRAY_SIZE(space.context_tbl); i++) {
    if (!space.context_tbl[i])
    continue;
// sanity check, should never happen
    if (~space.context_tbl[i]) {
    dev_err(&chip.dev, "context table is inconsistent");
    return -EFAULT;
    }
    rc = tpm2_load_context(chip, space.context_buf, &offset,
    &space.context_tbl[i]);
    if (rc)
    return rc;
    }
    for (i = 0, offset = 0; i < ARRAY_SIZE(space.session_tbl); i++) {
    u32 handle;
    if (!space.session_tbl[i])
    continue;
    rc = tpm2_load_context(chip, space.session_buf,
    &offset, &handle);
    if (rc == -ENOENT) {
// load failed, just forget session
    space.session_tbl[i] = 0;
    } else if (rc) {
    tpm2_flush_space(chip);
    return rc;
    }
    if (handle != space.session_tbl[i]) {
    dev_warn(&chip.dev, "session restored to wrong handle\n");
    tpm2_flush_space(chip);
    return -EFAULT;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tpm2_map_to_phandle(space: *mut tpm_space, handle: *mut c_void) -> bool {
    static bool tpm2_map_to_phandle(struct tpm_space *space, void *handle)
    {
    let mut vhandle: u32 = be32_to_cpup((__be32 *)handle);
    u32 phandle;
    int i;
    i = 0xFFFFFF - (vhandle & 0xFFFFFF);
    if (i >= ARRAY_SIZE(space.context_tbl) || !space.context_tbl[i])
    return false;
    phandle = space.context_tbl[i];
// ((__be32 *)handle) = cpu_to_be32(phandle);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn tpm2_map_command(chip: *mut tpm_chip, cc: u32, cmd: *mut u8) -> c_int {
    static int tpm2_map_command(struct tpm_chip *chip, u32 cc, u8 *cmd)
    {
    struct tpm_space *space = &chip.work_space;
    unsigned int nr_handles;
    u32 attrs;
    __be32 *handle;
    int i;
    i = tpm2_find_cc(chip, cc);
    if (i < 0)
    return -EINVAL;
    attrs = chip.cc_attrs_tbl[i];
    nr_handles = (attrs >> TPM2_CC_ATTR_CHANDLES) & GENMASK(2, 0);
    handle = (__be32 *)&cmd[TPM_HEADER_SIZE];
    for (i = 0; i < nr_handles; i++, handle++) {
    if ((be32_to_cpu(*handle) & 0xFF000000) == TPM2_HT_TRANSIENT) {
    if (!tpm2_map_to_phandle(space, handle))
    return -EINVAL;
    }
    }
    return 0;
    }
    static int tpm_find_and_validate_cc(struct tpm_chip *chip,
    struct tpm_space *space,
    const void *cmd, size_t len)
    {
    const struct tpm_header *header = (const void *)cmd;
    int i;
    u32 cc;
    u32 attrs;
    unsigned int nr_handles;
    if (len < TPM_HEADER_SIZE || !chip.nr_commands)
    return -EINVAL;
    cc = be32_to_cpu(header.ordinal);
    i = tpm2_find_cc(chip, cc);
    if (i < 0) {
    dev_dbg(&chip.dev, "0x%04X is an invalid command\n",
    cc);
    return -EOPNOTSUPP;
    }
    attrs = chip.cc_attrs_tbl[i];
    nr_handles =
    4 * ((attrs >> TPM2_CC_ATTR_CHANDLES) & GENMASK(2, 0));
    if (len < TPM_HEADER_SIZE + 4 * nr_handles)
    goto err_len;
    return cc;
    err_len:
    dev_dbg(&chip.dev, "%s: insufficient command length %zu", __func__,
    len);
    return -EINVAL;
    }
    int tpm2_prepare_space(struct tpm_chip *chip, struct tpm_space *space, u8 *cmd,
    size_t cmdsiz)
    {
    int rc;
    int cc;
    if (!space)
    return 0;
    cc = tpm_find_and_validate_cc(chip, space, cmd, cmdsiz);
    if (cc < 0)
    return cc;
    memcpy(&chip.work_space.context_tbl, &space.context_tbl,
    sizeof(space.context_tbl));
    memcpy(&chip.work_space.session_tbl, &space.session_tbl,
    sizeof(space.session_tbl));
    memcpy(chip.work_space.context_buf, space.context_buf,
    space.buf_size);
    memcpy(chip.work_space.session_buf, space.session_buf,
    space.buf_size);
    rc = tpm2_load_space(chip);
    if (rc) {
    tpm2_flush_space(chip);
    return rc;
    }
    rc = tpm2_map_command(chip, cc, cmd);
    if (rc) {
    tpm2_flush_space(chip);
    return rc;
    }
    chip.last_cc = cc;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tpm2_add_session(chip: *mut tpm_chip, handle: u32) -> bool {
    static bool tpm2_add_session(struct tpm_chip *chip, u32 handle)
    {
    struct tpm_space *space = &chip.work_space;
    int i;
    for (i = 0; i < ARRAY_SIZE(space.session_tbl); i++)
    if (space.session_tbl[i] == 0)
    break;
    if (i == ARRAY_SIZE(space.session_tbl))
    return false;
    space.session_tbl[i] = handle;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn tpm2_map_to_vhandle(space: *mut tpm_space, phandle: u32, alloc: bool) -> u32 {
    static u32 tpm2_map_to_vhandle(struct tpm_space *space, u32 phandle, bool alloc)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(space.context_tbl); i++) {
    if (alloc) {
    if (!space.context_tbl[i]) {
    space.context_tbl[i] = phandle;
    break;
    }
    } else if (space.context_tbl[i] == phandle)
    break;
    }
    if (i == ARRAY_SIZE(space.context_tbl))
    return 0;
    return TPM2_HT_TRANSIENT | (0xFFFFFF - i);
    }
    static int tpm2_map_response_header(struct tpm_chip *chip, u32 cc, u8 *rsp,
    size_t len)
    {
    struct tpm_space *space = &chip.work_space;
    struct tpm_header *header = (struct tpm_header *)rsp;
    u32 phandle;
    u32 phandle_type;
    u32 vhandle;
    u32 attrs;
    int i;
    if (be32_to_cpu(header.return_code) != TPM2_RC_SUCCESS)
    return 0;
    i = tpm2_find_cc(chip, cc);
// sanity check, should never happen
    if (i < 0)
    return -EFAULT;
    attrs = chip.cc_attrs_tbl[i];
    if (!((attrs >> TPM2_CC_ATTR_RHANDLE) & 1))
    return 0;
    phandle = be32_to_cpup((__be32 *)&rsp[TPM_HEADER_SIZE]);
    phandle_type = phandle & 0xFF000000;
    switch (phandle_type) {
    case TPM2_HT_TRANSIENT:
    vhandle = tpm2_map_to_vhandle(space, phandle, true);
    if (!vhandle)
    goto out_no_slots;
// (__be32 *)&rsp[TPM_HEADER_SIZE] = cpu_to_be32(vhandle);
    break;
    case TPM2_HT_HMAC_SESSION:
    case TPM2_HT_POLICY_SESSION:
    if (!tpm2_add_session(chip, phandle))
    goto out_no_slots;
    break;
    default:
    dev_err(&chip.dev, "%s: unknown handle 0x%08X\n",
    __func__, phandle);
    break;
    }
    return 0;
    out_no_slots:
    tpm2_flush_context(chip, phandle);
    dev_warn(&chip.dev, "%s: out of slots for 0x%08X\n", __func__,
    phandle);
    return -ENOMEM;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpm2_cap_handles {
    pub more_data: u8,
    pub capability: __be32,
    pub count: __be32,
    pub handles: [__be32; ],
    pub __packed: },
    static int tpm2_map_response_body(struct tpm_chip *chip, u32 cc, u8 *rsp,
    size_t len)
    {
    pub &chip->work_space: *mut *mut tpm_space space =,
    pub )rsp: *mut *mut tpm_header header = (tpm_header,
    pub data: *mut tpm2_cap_handles,
    pub phandle: u32,
    pub phandle_type: u32,
    pub vhandle: u32,
    pub i: c_int,
    pub j: c_int,
    if (cc != TPM2_CC_GET_CAPABILITY ||
    be32_to_cpu(header.return_code) != TPM2_RC_SUCCESS) {
    pub 0: return,
    }
    if (len < TPM_HEADER_SIZE + 9)
    pub -EFAULT: return,
    pub )&rsp[TPM_HEADER_SIZE]: *mut data = (void,
    if (be32_to_cpu(data.capability) != TPM2_CAP_HANDLES)
    pub 0: return,
    if (be32_to_cpu(data.count) > (UINT_MAX - TPM_HEADER_SIZE - 9) / 4)
    pub -EFAULT: return,
    if (len != TPM_HEADER_SIZE + 9 + 4 * be32_to_cpu(data.count))
    pub -EFAULT: return,
    pub {: for (i = 0, j = 0; i < be32_to_cpu(data->count); i++),
    pub )&data->handles[i]): *mut phandle = be32_to_cpup((__be32,
    pub 0xFF000000: phandle_type = phandle &,
    switch (phandle_type) {
    case TPM2_HT_TRANSIENT:
    pub false): vhandle = tpm2_map_to_vhandle(space, phandle,,
    if (!vhandle)
    pub cpu_to_be32(vhandle): data->handles[j] =,
    default:
    pub cpu_to_be32(phandle): data->handles[j] =,
    }
    }
    pub j): *mut *mut header->length = cpu_to_be32(TPM_HEADER_SIZE + 9 + 4,
    pub cpu_to_be32(j): data->count =,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn tpm2_save_space(chip: *mut tpm_chip) -> c_int {
    static int tpm2_save_space(struct tpm_chip *chip)
    {
    pub &chip->work_space: *mut *mut tpm_space space =,
    pub offset: c_uint,
    pub i: c_int,
    pub rc: c_int,
    pub {: for (i = 0, offset = 0; i < ARRAY_SIZE(space->context_tbl); i++),
    if (!(space.context_tbl[i] && ~space.context_tbl[i]))
    rc = tpm2_save_context(chip, space.context_tbl[i],
    space.context_buf, space.buf_size,
    if (rc == -ENOENT) {
    pub 0: space->context_tbl[i] =,
    } else if (rc)
    pub rc: return,
    pub space->context_tbl[i]): tpm2_flush_context(chip,,
    pub ~0: space->context_tbl[i] =,
    }
    pub {: for (i = 0, offset = 0; i < ARRAY_SIZE(space->session_tbl); i++),
    if (!space.session_tbl[i])
    rc = tpm2_save_context(chip, space.session_tbl[i],
    space.session_buf, space.buf_size,
    if (rc == -ENOENT) {
// handle error saving session, just forget it
    pub 0: space->session_tbl[i] =,
    } else if (rc < 0) {
    pub rc: return,
    }
    }
    pub 0: return,
    }
    int tpm2_commit_space(struct tpm_chip *chip, struct tpm_space *space,
    void *buf, size_t *bufsiz)
    {
    pub buf: *mut *mut tpm_header header =,
    pub rc: c_int,
    if (!space)
    pub 0: return,
    pub bufsiz): *mut rc = tpm2_map_response_header(chip, chip->last_cc, buf,,
    if (rc) {
    pub out: goto,
    }
    pub bufsiz): *mut rc = tpm2_map_response_body(chip, chip->last_cc, buf,,
    if (rc) {
    pub out: goto,
    }
    pub tpm2_save_space(chip): rc =,
    if (rc) {
    pub out: goto,
    }
// bufsiz = be32_to_cpu(header->length);
    memcpy(&space.context_tbl, &chip.work_space.context_tbl,
    memcpy(&space.session_tbl, &chip.work_space.session_tbl,
    memcpy(space.context_buf, chip.work_space.context_buf,
    memcpy(space.session_buf, chip.work_space.session_buf,
    pub 0: return,
    out:
    pub rc): dev_err(&chip->dev, "%s: error %d\n", __func__,,
    pub rc: return,
    }
//
// Put the reference to the main device.
//
#[no_mangle]
unsafe extern "C" fn tpm_devs_release(dev: *mut device) {
    static void tpm_devs_release(struct device *dev)
    {
    pub devs): *mut *mut tpm_chip chip = container_of(dev, tpm_chip,,
// release the master device reference
    }
//
// Remove the device file for exposed TPM spaces and release the device
// reference. This may also release the reference to the master device.
//
#[no_mangle]
pub unsafe extern "C" fn tpm_devs_remove(chip: *mut tpm_chip) {
    void tpm_devs_remove(struct tpm_chip *chip)
    {
    pub &chip->devs): cdev_device_del(&chip->cdevs,,
    }
//
// Add a device file to expose TPM spaces. Also take a reference to the
// main device.
//
#[no_mangle]
pub unsafe extern "C" fn tpm_devs_add(chip: *mut tpm_chip) -> c_int {
    int tpm_devs_add(struct tpm_chip *chip)
    {
    pub rc: c_int,
    pub chip->dev.parent: chip->devs.parent =,
    pub &tpmrm_class: chip->devs.class =,
//
// Get extra reference on main device to hold on behalf of devs.
// This holds the chip structure while cdevs is in use. The
// corresponding put is in the tpm_devs_release.
//
    pub tpm_devs_release: chip->devs.release =,
    pub TPM_NUM_DEVICES): chip->devs.devt = MKDEV(MAJOR(tpm_devt), chip->dev_num +,
    pub &tpmrm_fops): cdev_init(&chip->cdevs,,
    pub THIS_MODULE: chip->cdevs.owner =,
    pub chip->dev_num): rc = dev_set_name(&chip->devs, "tpmrm%d",,
    if (rc)
    pub err_put_devs: goto,
    pub &chip->devs): rc = cdev_device_add(&chip->cdevs,,
    if (rc) {
    dev_err(&chip.devs,
    "unable to cdev_device_add() %s, major %d, minor %d, err=%d\n",
    dev_name(&chip.devs), MAJOR(chip.devs.devt),
    pub rc): MINOR(chip->devs.devt),,
    pub err_put_devs: goto,
    }
    pub 0: return,
    err_put_devs:
    pub rc: return,
    }
