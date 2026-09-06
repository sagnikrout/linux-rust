//! Automatically rewritten from C to Rust
//! Source: drivers/accel/amdxdna/aie2_error.c
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
// Copyright (C) 2023-2024, Advanced Micro Devices, Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct async_event {
    pub ndev: *mut amdxdna_dev_hdl,
    pub resp: async_event_msg_resp,
    pub wq: *mut workqueue_struct,
    pub work: work_struct,
    pub buf: *mut u8,
    pub addr: dma_addr_t,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct async_events {
    pub wq: *mut workqueue_struct,
    pub buf: *mut u8,
    pub addr: dma_addr_t,
    pub size: u32,
    pub event_cnt: u32,
    pub __counted_by(event_cnt): async_event event[],
}

//
// Below enum, struct and lookup tables are porting from XAIE util header file.
//
// Below data is defined by AIE device and it is used for decode error message
// from the device.
//
    enum aie_module_type {
    AIE_MEM_MOD = 0,
    AIE_CORE_MOD,
    AIE_PL_MOD,
    AIE_UNKNOWN_MOD,
    };
    enum aie_error_category {
    AIE_ERROR_SATURATION = 0,
    AIE_ERROR_FP,
    AIE_ERROR_STREAM,
    AIE_ERROR_ACCESS,
    AIE_ERROR_BUS,
    AIE_ERROR_INSTRUCTION,
    AIE_ERROR_ECC,
    AIE_ERROR_LOCK,
    AIE_ERROR_DMA,
    AIE_ERROR_MEM_PARITY,
// Unknown is not from XAIE, added for better category
    AIE_ERROR_UNKNOWN,
    };
// Don't pack, unless XAIE side changed
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie_error {
    pub row: __u8,
    pub col: __u8,
    pub mod_type: __u32,
    pub event_id: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie_err_info {
    pub err_cnt: u32,
    pub ret_code: u32,
    pub rsvd: u32,
    pub __counted_by(err_cnt): aie_error payload[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie_event_category {
    pub event_id: u8,
    pub category: enum aie_error_category,
}

    static const struct aie_event_category aie_ml_mem_event_cat[] = {
    EVENT_CATEGORY(88U,  AIE_ERROR_ECC),
    EVENT_CATEGORY(90U,  AIE_ERROR_ECC),
    EVENT_CATEGORY(91U,  AIE_ERROR_MEM_PARITY),
    EVENT_CATEGORY(92U,  AIE_ERROR_MEM_PARITY),
    EVENT_CATEGORY(93U,  AIE_ERROR_MEM_PARITY),
    EVENT_CATEGORY(94U,  AIE_ERROR_MEM_PARITY),
    EVENT_CATEGORY(95U,  AIE_ERROR_MEM_PARITY),
    EVENT_CATEGORY(96U,  AIE_ERROR_MEM_PARITY),
    EVENT_CATEGORY(97U,  AIE_ERROR_DMA),
    EVENT_CATEGORY(98U,  AIE_ERROR_DMA),
    EVENT_CATEGORY(99U,  AIE_ERROR_DMA),
    EVENT_CATEGORY(100U, AIE_ERROR_DMA),
    EVENT_CATEGORY(101U, AIE_ERROR_LOCK),
    };
    static const struct aie_event_category aie_ml_core_event_cat[] = {
    EVENT_CATEGORY(55U, AIE_ERROR_ACCESS),
    EVENT_CATEGORY(56U, AIE_ERROR_STREAM),
    EVENT_CATEGORY(57U, AIE_ERROR_STREAM),
    EVENT_CATEGORY(58U, AIE_ERROR_BUS),
    EVENT_CATEGORY(59U, AIE_ERROR_INSTRUCTION),
    EVENT_CATEGORY(60U, AIE_ERROR_ACCESS),
    EVENT_CATEGORY(62U, AIE_ERROR_ECC),
    EVENT_CATEGORY(64U, AIE_ERROR_ECC),
    EVENT_CATEGORY(65U, AIE_ERROR_ACCESS),
    EVENT_CATEGORY(66U, AIE_ERROR_ACCESS),
    EVENT_CATEGORY(67U, AIE_ERROR_LOCK),
    EVENT_CATEGORY(70U, AIE_ERROR_INSTRUCTION),
    EVENT_CATEGORY(71U, AIE_ERROR_STREAM),
    EVENT_CATEGORY(72U, AIE_ERROR_BUS),
    };
    static const struct aie_event_category aie_ml_mem_tile_event_cat[] = {
    EVENT_CATEGORY(130U, AIE_ERROR_ECC),
    EVENT_CATEGORY(132U, AIE_ERROR_ECC),
    EVENT_CATEGORY(133U, AIE_ERROR_DMA),
    EVENT_CATEGORY(134U, AIE_ERROR_DMA),
    EVENT_CATEGORY(135U, AIE_ERROR_STREAM),
    EVENT_CATEGORY(136U, AIE_ERROR_STREAM),
    EVENT_CATEGORY(137U, AIE_ERROR_STREAM),
    EVENT_CATEGORY(138U, AIE_ERROR_BUS),
    EVENT_CATEGORY(139U, AIE_ERROR_LOCK),
    };
    static const struct aie_event_category aie_ml_shim_tile_event_cat[] = {
    EVENT_CATEGORY(64U, AIE_ERROR_BUS),
    EVENT_CATEGORY(65U, AIE_ERROR_STREAM),
    EVENT_CATEGORY(66U, AIE_ERROR_STREAM),
    EVENT_CATEGORY(67U, AIE_ERROR_BUS),
    EVENT_CATEGORY(68U, AIE_ERROR_BUS),
    EVENT_CATEGORY(69U, AIE_ERROR_BUS),
    EVENT_CATEGORY(70U, AIE_ERROR_BUS),
    EVENT_CATEGORY(71U, AIE_ERROR_BUS),
    EVENT_CATEGORY(72U, AIE_ERROR_DMA),
    EVENT_CATEGORY(73U, AIE_ERROR_DMA),
    EVENT_CATEGORY(74U, AIE_ERROR_LOCK),
    };
    static const enum amdxdna_error_num aie_cat_err_num_map[] = {
    [AIE_ERROR_SATURATION] = AMDXDNA_ERROR_NUM_AIE_SATURATION,
    [AIE_ERROR_FP] = AMDXDNA_ERROR_NUM_AIE_FP,
    [AIE_ERROR_STREAM] = AMDXDNA_ERROR_NUM_AIE_STREAM,
    [AIE_ERROR_ACCESS] = AMDXDNA_ERROR_NUM_AIE_ACCESS,
    [AIE_ERROR_BUS] = AMDXDNA_ERROR_NUM_AIE_BUS,
    [AIE_ERROR_INSTRUCTION] = AMDXDNA_ERROR_NUM_AIE_INSTRUCTION,
    [AIE_ERROR_ECC] = AMDXDNA_ERROR_NUM_AIE_ECC,
    [AIE_ERROR_LOCK] = AMDXDNA_ERROR_NUM_AIE_LOCK,
    [AIE_ERROR_DMA] = AMDXDNA_ERROR_NUM_AIE_DMA,
    [AIE_ERROR_MEM_PARITY] = AMDXDNA_ERROR_NUM_AIE_MEM_PARITY,
    [AIE_ERROR_UNKNOWN] = AMDXDNA_ERROR_NUM_UNKNOWN,
    };
    static_assert(ARRAY_SIZE(aie_cat_err_num_map) == AIE_ERROR_UNKNOWN + 1);
    static const enum amdxdna_error_module aie_err_mod_map[] = {
    [AIE_MEM_MOD] = AMDXDNA_ERROR_MODULE_AIE_MEMORY,
    [AIE_CORE_MOD] = AMDXDNA_ERROR_MODULE_AIE_CORE,
    [AIE_PL_MOD] = AMDXDNA_ERROR_MODULE_AIE_PL,
    [AIE_UNKNOWN_MOD] = AMDXDNA_ERROR_MODULE_UNKNOWN,
    };
    static_assert(ARRAY_SIZE(aie_err_mod_map) == AIE_UNKNOWN_MOD + 1);
    static enum aie_error_category
    aie_get_error_category(u8 row, u8 event_id, enum aie_module_type mod_type)
    {
    const struct aie_event_category *lut;
    int num_entry;
    int i;
    switch (mod_type) {
    case AIE_PL_MOD:
    lut = aie_ml_shim_tile_event_cat;
    num_entry = ARRAY_SIZE(aie_ml_shim_tile_event_cat);
    break;
    case AIE_CORE_MOD:
    lut = aie_ml_core_event_cat;
    num_entry = ARRAY_SIZE(aie_ml_core_event_cat);
    break;
    case AIE_MEM_MOD:
    if (row == 1) {
    lut = aie_ml_mem_tile_event_cat;
    num_entry = ARRAY_SIZE(aie_ml_mem_tile_event_cat);
    } else {
    lut = aie_ml_mem_event_cat;
    num_entry = ARRAY_SIZE(aie_ml_mem_event_cat);
    }
    break;
    default:
    return AIE_ERROR_UNKNOWN;
    }
    for (i = 0; i < num_entry; i++) {
    if (event_id != lut[i].event_id)
    continue;
    if (lut[i].category > AIE_ERROR_UNKNOWN)
    return AIE_ERROR_UNKNOWN;
    return lut[i].category;
    }
    return AIE_ERROR_UNKNOWN;
    }
#[no_mangle]
unsafe extern "C" fn aie2_update_last_async_error(ndev: *mut amdxdna_dev_hdl, err_info: *mut c_void, num_err: u32) {
    static void aie2_update_last_async_error(struct amdxdna_dev_hdl *ndev, void *err_info, u32 num_err)
    {
    struct aie_error *errs = err_info;
    enum amdxdna_error_module err_mod;
    enum aie_error_category aie_err;
    enum amdxdna_error_num err_num;
    struct aie_error *last_err;
    last_err = &errs[num_err - 1];
    if (last_err.mod_type >= AIE_UNKNOWN_MOD) {
    err_num = aie_cat_err_num_map[AIE_ERROR_UNKNOWN];
    err_mod = aie_err_mod_map[AIE_UNKNOWN_MOD];
    } else {
    aie_err = aie_get_error_category(last_err.row,
    last_err.event_id,
    last_err.mod_type);
    err_num = aie_cat_err_num_map[aie_err];
    err_mod = aie_err_mod_map[last_err.mod_type];
    }
    ndev.last_async_err.err_code = AMDXDNA_ERROR_ENCODE(err_num, err_mod);
    ndev.last_async_err.ts_us = ktime_to_us(ktime_get_real());
    ndev.last_async_err.ex_err_code = AMDXDNA_EXTRA_ERR_ENCODE(last_err.row, last_err.col);
    }
#[no_mangle]
unsafe extern "C" fn aie2_error_backtrack(ndev: *mut amdxdna_dev_hdl, err_info: *mut c_void, num_err: u32) -> u32 {
    static u32 aie2_error_backtrack(struct amdxdna_dev_hdl *ndev, void *err_info, u32 num_err)
    {
    struct aie_error *errs = err_info;
    u32 err_col = 0; /* assume that AIE has less than 32 columns */
    int i;
// Get err column bitmap
    for (i = 0; i < num_err; i++) {
    struct aie_error *err = &errs[i];
    enum aie_error_category cat;
    cat = aie_get_error_category(err.row, err.event_id, err.mod_type);
    XDNA_ERR(ndev.aie.xdna, "Row: %d, Col: %d, module %d, event ID %d, category %d",
    err.row, err.col, err.mod_type,
    err.event_id, cat);
    if (err.col >= 32) {
    XDNA_WARN(ndev.aie.xdna, "Invalid column number");
    break;
    }
    err_col |= (1 << err.col);
    }
    return err_col;
    }
#[no_mangle]
unsafe extern "C" fn aie2_error_async_cb(handle: *mut c_void, data: *mut void __iomem, size: usize) -> c_int {
    static int aie2_error_async_cb(void *handle, void __iomem *data, size_t size)
    {
    struct async_event *e = handle;
    if (data) {
    e.resp.type = readl(data + offsetof(struct async_event_msg_resp, type));
    wmb(); /* Update status in the end, so that no lock for here */
    e.resp.status = readl(data + offsetof(struct async_event_msg_resp, status));
    }
    queue_work(e.wq, &e.work);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aie2_error_event_send(e: *mut async_event) -> c_int {
    static int aie2_error_event_send(struct async_event *e)
    {
    drm_clflush_virt_range(e.buf, e.size); /* device can access */
    return aie2_register_asyn_event_msg(e.ndev, e.addr, e.size, e,
    aie2_error_async_cb);
    }
#[no_mangle]
unsafe extern "C" fn aie2_error_worker(err_work: *mut work_struct) {
    static void aie2_error_worker(struct work_struct *err_work)
    {
    struct aie_err_info *info;
    struct amdxdna_dev *xdna;
    struct async_event *e;
    u32 max_err;
    u32 err_col;
    e = container_of(err_work, struct async_event, work);
    xdna = e.ndev.aie.xdna;
    if (e.resp.status == MAX_AIE2_STATUS_CODE)
    return;
    e.resp.status = MAX_AIE2_STATUS_CODE;
    print_hex_dump_debug("AIE error: ", DUMP_PREFIX_OFFSET, 16, 4,
    e.buf, 0x100, false);
    info = (struct aie_err_info *)e.buf;
    XDNA_DBG(xdna, "Error count %d return code %d", info.err_cnt, info.ret_code);
    max_err = (e.size - sizeof(*info)) / sizeof(struct aie_error);
    if (unlikely(info.err_cnt > max_err)) {
    WARN_ONCE(1, "Error count too large %d\n", info.err_cnt);
    return;
    }
    err_col = aie2_error_backtrack(e.ndev, info.payload, info.err_cnt);
    if (!err_col) {
    XDNA_WARN(xdna, "Did not get error column");
    return;
    }
    mutex_lock(&xdna.dev_lock);
    aie2_update_last_async_error(e.ndev, info.payload, info.err_cnt);
// Re-sent this event to firmware
    if (aie2_error_event_send(e))
    XDNA_WARN(xdna, "Unable to register async event");
    mutex_unlock(&xdna.dev_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn aie2_error_async_events_free(ndev: *mut amdxdna_dev_hdl) {
    void aie2_error_async_events_free(struct amdxdna_dev_hdl *ndev)
    {
    struct amdxdna_dev *xdna = ndev.aie.xdna;
    struct async_events *events;
    events = ndev.async_events;
    mutex_unlock(&xdna.dev_lock);
    destroy_workqueue(events.wq);
    mutex_lock(&xdna.dev_lock);
    amdxdna_free_msg_buffer(xdna, events.size, events.buf, events.addr);
    kfree(events);
    }
#[no_mangle]
pub unsafe extern "C" fn aie2_error_async_events_alloc(ndev: *mut amdxdna_dev_hdl) -> c_int {
    int aie2_error_async_events_alloc(struct amdxdna_dev_hdl *ndev)
    {
    struct amdxdna_dev *xdna = ndev.aie.xdna;
    let mut total_col: u32 = ndev.total_col;
    let mut total_size: u32 = ASYNC_BUF_SIZE * total_col;
    struct async_events *events;
    int i, ret;
    events = kzalloc_flex(*events, event, total_col);
    if (!events)
    return -ENOMEM;
    events.buf = amdxdna_alloc_msg_buffer(xdna, &total_size, &events.addr);
    if (IS_ERR(events.buf)) {
    ret = PTR_ERR(events.buf);
    goto free_events;
    }
    events.size = total_size;
    events.event_cnt = total_col;
    events.wq = alloc_ordered_workqueue("async_wq", 0);
    if (!events.wq) {
    ret = -ENOMEM;
    goto free_buf;
    }
    for (i = 0; i < events.event_cnt; i++) {
    struct async_event *e = &events.event[i];
    let mut offset: u32 = i * ASYNC_BUF_SIZE;
    e.ndev = ndev;
    e.wq = events.wq;
    e.buf = &events.buf[offset];
    e.addr = events.addr + offset;
    e.size = ASYNC_BUF_SIZE;
    e.resp.status = MAX_AIE2_STATUS_CODE;
    INIT_WORK(&e.work, aie2_error_worker);
    ret = aie2_error_event_send(e);
    if (ret)
    goto free_wq;
    }
    ndev.async_events = events;
    XDNA_DBG(xdna, "Async event count %d, buf total size 0x%x",
    events.event_cnt, events.size);
    return 0;
    free_wq:
    destroy_workqueue(events.wq);
    free_buf:
    amdxdna_free_msg_buffer(xdna, events.size, events.buf, events.addr);
    free_events:
    kfree(events);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn aie2_get_array_async_error(ndev: *mut amdxdna_dev_hdl, args: *mut amdxdna_drm_get_array) -> c_int {
    int aie2_get_array_async_error(struct amdxdna_dev_hdl *ndev, struct amdxdna_drm_get_array *args)
    {
    struct amdxdna_dev *xdna = ndev.aie.xdna;
    drm_WARN_ON(&xdna.ddev, !mutex_is_locked(&xdna.dev_lock));
    if (!args.num_element)
    return -EINVAL;
    args.num_element = 1;
    args.element_size = min(args.element_size, sizeof(ndev.last_async_err));
    if (copy_to_user(u64_to_user_ptr(args.buffer),
    &ndev.last_async_err, args.element_size))
    return -EFAULT;
    return 0;
    }
