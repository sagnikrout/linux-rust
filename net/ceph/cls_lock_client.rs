//! Automatically rewritten from C to Rust
//! Source: net/ceph/cls_lock_client.c
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
// ceph_cls_lock - grab rados lock for object
// @osdc: OSD client instance
// @oid: object to lock
// @oloc: object to lock
// @lock_name: the name of the lock
// @type: lock type (CEPH_CLS_LOCK_EXCLUSIVE or CEPH_CLS_LOCK_SHARED)
// @cookie: user-defined identifier for this instance of the lock
// @tag: user-defined tag
// @desc: user-defined lock description
// @flags: lock flags
//
// All operations on the same lock should use the same tag.
//
    int ceph_cls_lock(struct ceph_osd_client *osdc,
    struct ceph_object_id *oid,
    struct ceph_object_locator *oloc,
    char *lock_name, u8 type, char *cookie,
    char *tag, char *desc, u8 flags)
    {
    int lock_op_buf_size;
    let mut name_len: c_int = strlen(lock_name);
    let mut cookie_len: c_int = strlen(cookie);
    let mut tag_len: c_int = strlen(tag);
    let mut desc_len: c_int = strlen(desc);
    void *p, *end;
    struct page *lock_op_page;
    struct timespec64 mtime;
    int ret;
    lock_op_buf_size = name_len + sizeof(__le32) +
    cookie_len + sizeof(__le32) +
    tag_len + sizeof(__le32) +
    desc_len + sizeof(__le32) +
    sizeof(struct ceph_timespec) +
// flag and type
    sizeof(u8) + sizeof(u8) +
    CEPH_ENCODING_START_BLK_LEN;
    if (lock_op_buf_size > PAGE_SIZE)
    return -E2BIG;
    lock_op_page = alloc_page(GFP_NOIO);
    if (!lock_op_page)
    return -ENOMEM;
    p = page_address(lock_op_page);
    end = p + lock_op_buf_size;
// encode cls_lock_lock_op struct
    ceph_start_encoding(&p, 1, 1,
    lock_op_buf_size - CEPH_ENCODING_START_BLK_LEN);
    ceph_encode_string(&p, end, lock_name, name_len);
    ceph_encode_8(&p, type);
    ceph_encode_string(&p, end, cookie, cookie_len);
    ceph_encode_string(&p, end, tag, tag_len);
    ceph_encode_string(&p, end, desc, desc_len);
// only support infinite duration
    memset(&mtime, 0, sizeof(mtime));
    ceph_encode_timespec64(p, &mtime);
    p += sizeof(struct ceph_timespec);
    ceph_encode_8(&p, flags);
    dout("%s lock_name %s type %d cookie %s tag %s desc %s flags 0x%x\n",
    __func__, lock_name, type, cookie, tag, desc, flags);
    ret = ceph_osdc_call(osdc, oid, oloc, "lock", "lock",
    CEPH_OSD_FLAG_WRITE, lock_op_page,
    lock_op_buf_size, core::ptr::null_mut(), core::ptr::null_mut());
    dout("%s: status %d\n", __func__, ret);
    __free_page(lock_op_page);
    return ret;
    }
    EXPORT_SYMBOL(ceph_cls_lock);
//
// ceph_cls_unlock - release rados lock for object
// @osdc: OSD client instance
// @oid: object to lock
// @oloc: object to lock
// @lock_name: the name of the lock
// @cookie: user-defined identifier for this instance of the lock
//
    int ceph_cls_unlock(struct ceph_osd_client *osdc,
    struct ceph_object_id *oid,
    struct ceph_object_locator *oloc,
    char *lock_name, char *cookie)
    {
    int unlock_op_buf_size;
    let mut name_len: c_int = strlen(lock_name);
    let mut cookie_len: c_int = strlen(cookie);
    void *p, *end;
    struct page *unlock_op_page;
    int ret;
    unlock_op_buf_size = name_len + sizeof(__le32) +
    cookie_len + sizeof(__le32) +
    CEPH_ENCODING_START_BLK_LEN;
    if (unlock_op_buf_size > PAGE_SIZE)
    return -E2BIG;
    unlock_op_page = alloc_page(GFP_NOIO);
    if (!unlock_op_page)
    return -ENOMEM;
    p = page_address(unlock_op_page);
    end = p + unlock_op_buf_size;
// encode cls_lock_unlock_op struct
    ceph_start_encoding(&p, 1, 1,
    unlock_op_buf_size - CEPH_ENCODING_START_BLK_LEN);
    ceph_encode_string(&p, end, lock_name, name_len);
    ceph_encode_string(&p, end, cookie, cookie_len);
    dout("%s lock_name %s cookie %s\n", __func__, lock_name, cookie);
    ret = ceph_osdc_call(osdc, oid, oloc, "lock", "unlock",
    CEPH_OSD_FLAG_WRITE, unlock_op_page,
    unlock_op_buf_size, core::ptr::null_mut(), core::ptr::null_mut());
    dout("%s: status %d\n", __func__, ret);
    __free_page(unlock_op_page);
    return ret;
    }
    EXPORT_SYMBOL(ceph_cls_unlock);
//
// ceph_cls_break_lock - release rados lock for object for specified client
// @osdc: OSD client instance
// @oid: object to lock
// @oloc: object to lock
// @lock_name: the name of the lock
// @cookie: user-defined identifier for this instance of the lock
// @locker: current lock owner
//
    int ceph_cls_break_lock(struct ceph_osd_client *osdc,
    struct ceph_object_id *oid,
    struct ceph_object_locator *oloc,
    char *lock_name, char *cookie,
    struct ceph_entity_name *locker)
    {
    int break_op_buf_size;
    let mut name_len: c_int = strlen(lock_name);
    let mut cookie_len: c_int = strlen(cookie);
    struct page *break_op_page;
    void *p, *end;
    int ret;
    break_op_buf_size = name_len + sizeof(__le32) +
    cookie_len + sizeof(__le32) +
    sizeof(u8) + sizeof(__le64) +
    CEPH_ENCODING_START_BLK_LEN;
    if (break_op_buf_size > PAGE_SIZE)
    return -E2BIG;
    break_op_page = alloc_page(GFP_NOIO);
    if (!break_op_page)
    return -ENOMEM;
    p = page_address(break_op_page);
    end = p + break_op_buf_size;
// encode cls_lock_break_op struct
    ceph_start_encoding(&p, 1, 1,
    break_op_buf_size - CEPH_ENCODING_START_BLK_LEN);
    ceph_encode_string(&p, end, lock_name, name_len);
    ceph_encode_copy(&p, locker, sizeof(*locker));
    ceph_encode_string(&p, end, cookie, cookie_len);
    dout("%s lock_name %s cookie %s locker %s%llu\n", __func__, lock_name,
    cookie, ENTITY_NAME(*locker));
    ret = ceph_osdc_call(osdc, oid, oloc, "lock", "break_lock",
    CEPH_OSD_FLAG_WRITE, break_op_page,
    break_op_buf_size, core::ptr::null_mut(), core::ptr::null_mut());
    dout("%s: status %d\n", __func__, ret);
    __free_page(break_op_page);
    return ret;
    }
    EXPORT_SYMBOL(ceph_cls_break_lock);
    int ceph_cls_set_cookie(struct ceph_osd_client *osdc,
    struct ceph_object_id *oid,
    struct ceph_object_locator *oloc,
    char *lock_name, u8 type, char *old_cookie,
    char *tag, char *new_cookie)
    {
    int cookie_op_buf_size;
    let mut name_len: c_int = strlen(lock_name);
    let mut old_cookie_len: c_int = strlen(old_cookie);
    let mut tag_len: c_int = strlen(tag);
    let mut new_cookie_len: c_int = strlen(new_cookie);
    void *p, *end;
    struct page *cookie_op_page;
    int ret;
    cookie_op_buf_size = name_len + sizeof(__le32) +
    old_cookie_len + sizeof(__le32) +
    tag_len + sizeof(__le32) +
    new_cookie_len + sizeof(__le32) +
    sizeof(u8) + CEPH_ENCODING_START_BLK_LEN;
    if (cookie_op_buf_size > PAGE_SIZE)
    return -E2BIG;
    cookie_op_page = alloc_page(GFP_NOIO);
    if (!cookie_op_page)
    return -ENOMEM;
    p = page_address(cookie_op_page);
    end = p + cookie_op_buf_size;
// encode cls_lock_set_cookie_op struct
    ceph_start_encoding(&p, 1, 1,
    cookie_op_buf_size - CEPH_ENCODING_START_BLK_LEN);
    ceph_encode_string(&p, end, lock_name, name_len);
    ceph_encode_8(&p, type);
    ceph_encode_string(&p, end, old_cookie, old_cookie_len);
    ceph_encode_string(&p, end, tag, tag_len);
    ceph_encode_string(&p, end, new_cookie, new_cookie_len);
    dout("%s lock_name %s type %d old_cookie %s tag %s new_cookie %s\n",
    __func__, lock_name, type, old_cookie, tag, new_cookie);
    ret = ceph_osdc_call(osdc, oid, oloc, "lock", "set_cookie",
    CEPH_OSD_FLAG_WRITE, cookie_op_page,
    cookie_op_buf_size, core::ptr::null_mut(), core::ptr::null_mut());
    dout("%s: status %d\n", __func__, ret);
    __free_page(cookie_op_page);
    return ret;
    }
    EXPORT_SYMBOL(ceph_cls_set_cookie);
#[no_mangle]
pub unsafe extern "C" fn ceph_free_lockers(lockers: *mut ceph_locker, num_lockers: u32) {
    void ceph_free_lockers(struct ceph_locker *lockers, u32 num_lockers)
    {
    int i;
    for (i = 0; i < num_lockers; i++)
    kfree(lockers[i].id.cookie);
    kfree(lockers);
    }
    EXPORT_SYMBOL(ceph_free_lockers);
#[no_mangle]
unsafe extern "C" fn decode_locker(p: *mut c_void, end: *mut c_void, locker: *mut ceph_locker) -> c_int {
    static int decode_locker(void **p, void *end, struct ceph_locker *locker)
    {
    u8 struct_v;
    u32 len;
    char *s;
    int ret;
    ret = ceph_start_decoding(p, end, 1, "locker_id_t", &struct_v, &len);
    if (ret)
    return ret;
    ceph_decode_copy_safe(p, end, &locker.id.name,
    sizeof(locker.id.name), bad);
    s = ceph_extract_encoded_string(p, end, core::ptr::null_mut(), GFP_NOIO);
    if (IS_ERR(s))
    return PTR_ERR(s);
    locker.id.cookie = s;
    ret = ceph_start_decoding(p, end, 1, "locker_info_t", &struct_v, &len);
    if (ret)
    return ret;
// skip expiration
    ceph_decode_skip_n(p, end, sizeof(struct ceph_timespec), bad);
    ret = ceph_decode_entity_addr(p, end, &locker.info.addr);
    if (ret)
    return ret;
// skip description
    ceph_decode_skip_string(p, end, bad);
    dout("%s %s%llu cookie %s addr %s\n", __func__,
    ENTITY_NAME(locker.id.name), locker.id.cookie,
    ceph_pr_addr(&locker.info.addr));
    return 0;
    bad:
    return -EINVAL;
    }
    static int decode_lockers(void **p, void *end, u8 *type, char **tag,
    struct ceph_locker **lockers, u32 *num_lockers)
    {
    u8 struct_v;
    u32 struct_len;
    char *s;
    int i;
    int ret;
    ret = ceph_start_decoding(p, end, 1, "cls_lock_get_info_reply",
    &struct_v, &struct_len);
    if (ret)
    return ret;
    ceph_decode_32_safe(p, end, *num_lockers, err_inval);
// lockers = kzalloc_objs(**lockers, *num_lockers, GFP_NOIO);
    if (!*lockers)
    return -ENOMEM;
    for (i = 0; i < *num_lockers; i++) {
    ret = decode_locker(p, end, *lockers + i);
    if (ret)
    goto err_free_lockers;
    }
    ret = -EINVAL;
    ceph_decode_8_safe(p, end, *type, err_free_lockers);
    s = ceph_extract_encoded_string(p, end, core::ptr::null_mut(), GFP_NOIO);
    if (IS_ERR(s)) {
    ret = PTR_ERR(s);
    goto err_free_lockers;
    }
// tag = s;
    return 0;
    err_inval:
    return -EINVAL;
    err_free_lockers:
    ceph_free_lockers(*lockers, *num_lockers);
    return ret;
    }
//
// On success, the caller is responsible for:
//
// kfree(tag);
// ceph_free_lockers(lockers, num_lockers);
//
    int ceph_cls_lock_info(struct ceph_osd_client *osdc,
    struct ceph_object_id *oid,
    struct ceph_object_locator *oloc,
    char *lock_name, u8 *type, char **tag,
    struct ceph_locker **lockers, u32 *num_lockers)
    {
    int get_info_op_buf_size;
    let mut name_len: c_int = strlen(lock_name);
    struct page *get_info_op_page, *reply_page;
    let mut reply_len: usize = PAGE_SIZE;
    void *p, *end;
    int ret;
    get_info_op_buf_size = name_len + sizeof(__le32) +
    CEPH_ENCODING_START_BLK_LEN;
    if (get_info_op_buf_size > PAGE_SIZE)
    return -E2BIG;
    get_info_op_page = alloc_page(GFP_NOIO);
    if (!get_info_op_page)
    return -ENOMEM;
    reply_page = alloc_page(GFP_NOIO);
    if (!reply_page) {
    __free_page(get_info_op_page);
    return -ENOMEM;
    }
    p = page_address(get_info_op_page);
    end = p + get_info_op_buf_size;
// encode cls_lock_get_info_op struct
    ceph_start_encoding(&p, 1, 1,
    get_info_op_buf_size - CEPH_ENCODING_START_BLK_LEN);
    ceph_encode_string(&p, end, lock_name, name_len);
    dout("%s lock_name %s\n", __func__, lock_name);
    ret = ceph_osdc_call(osdc, oid, oloc, "lock", "get_info",
    CEPH_OSD_FLAG_READ, get_info_op_page,
    get_info_op_buf_size, &reply_page, &reply_len);
    dout("%s: status %d\n", __func__, ret);
    if (ret >= 0) {
    p = page_address(reply_page);
    end = p + reply_len;
    ret = decode_lockers(&p, end, type, tag, lockers, num_lockers);
    }
    __free_page(get_info_op_page);
    __free_page(reply_page);
    return ret;
    }
    EXPORT_SYMBOL(ceph_cls_lock_info);
    int ceph_cls_assert_locked(struct ceph_osd_request *req, int which,
    char *lock_name, u8 type, char *cookie, char *tag)
    {
    int assert_op_buf_size;
    let mut name_len: c_int = strlen(lock_name);
    let mut cookie_len: c_int = strlen(cookie);
    let mut tag_len: c_int = strlen(tag);
    struct page **pages;
    void *p, *end;
    int ret;
    assert_op_buf_size = name_len + sizeof(__le32) +
    cookie_len + sizeof(__le32) +
    tag_len + sizeof(__le32) +
    sizeof(u8) + CEPH_ENCODING_START_BLK_LEN;
    if (assert_op_buf_size > PAGE_SIZE)
    return -E2BIG;
    ret = osd_req_op_cls_init(req, which, "lock", "assert_locked");
    if (ret)
    return ret;
    pages = ceph_alloc_page_vector(1, GFP_NOIO);
    if (IS_ERR(pages))
    return PTR_ERR(pages);
    p = page_address(pages[0]);
    end = p + assert_op_buf_size;
// encode cls_lock_assert_op struct
    ceph_start_encoding(&p, 1, 1,
    assert_op_buf_size - CEPH_ENCODING_START_BLK_LEN);
    ceph_encode_string(&p, end, lock_name, name_len);
    ceph_encode_8(&p, type);
    ceph_encode_string(&p, end, cookie, cookie_len);
    ceph_encode_string(&p, end, tag, tag_len);
    WARN_ON(p != end);
    osd_req_op_cls_request_data_pages(req, which, pages, assert_op_buf_size,
    0, false, true);
    return 0;
    }
    EXPORT_SYMBOL(ceph_cls_assert_locked);
