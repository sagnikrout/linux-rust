//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/drm_edid_load.c
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
//
    drm_edid_load.c: use a built-in EDID data set or load it via the firmware
    interface
    Copyright (C) 2012 Carsten Emde <C.Emde@osadl.org>
//

    static char edid_firmware[PATH_MAX];
    module_param_string(edid_firmware, edid_firmware, sizeof(edid_firmware), 0644);
    MODULE_PARM_DESC(edid_firmware,
    "Do not probe monitor, use specified EDID blob from /lib/firmware instead.");
    static const struct drm_edid *edid_load(struct drm_connector *connector, const char *name)
    {
    const struct firmware *fw = core::ptr::null_mut();
    const struct drm_edid *drm_edid;
    int err;
    err = request_firmware(&fw, name, connector.dev.dev);
    if (err) {
    drm_err(connector.dev,
    "[CONNECTOR:%d:%s] Requesting EDID firmware \"%s\" failed (err=%d)\n",
    connector.base.id, connector.name,
    name, err);
    return ERR_PTR(err);
    }
    drm_dbg_kms(connector.dev, "[CONNECTOR:%d:%s] Loaded external firmware EDID \"%s\"\n",
    connector.base.id, connector.name, name);
    drm_edid = drm_edid_alloc(fw.data, fw.size);
    if (!drm_edid_valid(drm_edid)) {
    drm_err(connector.dev, "Invalid firmware EDID \"%s\"\n", name);
    drm_edid_free(drm_edid);
    drm_edid = ERR_PTR(-EINVAL);
    }
    release_firmware(fw);
    return drm_edid;
    }
    const struct drm_edid *drm_edid_load_firmware(struct drm_connector *connector)
    {
    char *edidname, *last, *colon, *fwstr, *edidstr, *fallback = core::ptr::null_mut();
    const struct drm_edid *drm_edid;
    if (edid_firmware[0] == '\0')
    return ERR_PTR(-ENOENT);
//
// If there are multiple edid files specified and separated
// by commas, search through the list looking for one that
// matches the connector.
//
// If there's one or more that doesn't specify a connector, keep
// the last one found one as a fallback.
//
    fwstr = kstrdup(edid_firmware, GFP_KERNEL);
    if (!fwstr)
    return ERR_PTR(-ENOMEM);
    edidstr = fwstr;
    while ((edidname = strsep(&edidstr, ","))) {
    colon = strchr(edidname, ':');
    if (colon != core::ptr::null_mut()) {
    if (strncmp(connector.name, edidname, colon - edidname))
    continue;
    edidname = colon + 1;
    break;
    }
    if (*edidname != '\0') /* corner case: multiple ',' */
    fallback = edidname;
    }
    if (!edidname) {
    if (!fallback) {
    kfree(fwstr);
    return ERR_PTR(-ENOENT);
    }
    edidname = fallback;
    }
    last = edidname + strlen(edidname) - 1;
    if (*last == '\n')
// last = '\0';
    drm_edid = edid_load(connector, edidname);
    kfree(fwstr);
    return drm_edid;
    }
