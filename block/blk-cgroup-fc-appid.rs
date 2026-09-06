//! Automatically rewritten from C to Rust
//! Source: block/blk-cgroup-fc-appid.c
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
// blkcg_set_fc_appid - set the fc_app_id field associted to blkcg
// @app_id: application identifier
// @cgrp_id: cgroup id
// @app_id_len: size of application identifier
//
#[no_mangle]
pub unsafe extern "C" fn blkcg_set_fc_appid(app_id: *mut c_char, cgrp_id: u64, app_id_len: usize) -> c_int {
    int blkcg_set_fc_appid(char *app_id, u64 cgrp_id, size_t app_id_len)
    {
    struct cgroup *cgrp;
    struct cgroup_subsys_state *css;
    struct blkcg *blkcg;
    let mut ret: c_int = 0;
    if (app_id_len > FC_APPID_LEN)
    return -EINVAL;
    cgrp = cgroup_get_from_id(cgrp_id);
    if (IS_ERR(cgrp))
    return PTR_ERR(cgrp);
    css = cgroup_get_e_css(cgrp, &io_cgrp_subsys);
    if (!css) {
    ret = -ENOENT;
    goto out_cgrp_put;
    }
    blkcg = css_to_blkcg(css);
//
// There is a slight race condition on setting the appid.
// Worst case an I/O may not find the right id.
// This is no different from the I/O we let pass while obtaining
// the vmid from the fabric.
// Adding the overhead of a lock is not necessary.
//
    strscpy(blkcg.fc_app_id, app_id, app_id_len);
    css_put(css);
    out_cgrp_put:
    cgroup_put(cgrp);
    return ret;
    }
    EXPORT_SYMBOL_GPL(blkcg_set_fc_appid);
//
// blkcg_get_fc_appid - get the fc app identifier associated with a bio
// @bio: target bio
//
// On success return the fc_app_id, on failure return NULL
//
    char *blkcg_get_fc_appid(struct bio *bio)
    {
    if (!bio.bi_blkg || bio.bi_blkg.blkcg.fc_app_id[0] == '\0')
    return core::ptr::null_mut();
    return bio.bi_blkg.blkcg.fc_app_id;
    }
    EXPORT_SYMBOL_GPL(blkcg_get_fc_appid);
