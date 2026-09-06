//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/pmu/memx.c
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


// SPDX-License-Identifier: MIT

// Macro flag: #define __NVKM_PMU_MEMX_H__

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_memx {
    pub pmu: *mut nvkm_pmu,
    pub base: u32,
    pub size: u32,
    struct {
    pub mthd: u32,
    pub size: u32,
    pub data: [u32; 64],
    pub c: },
}

    static void
    memx_out(struct nvkm_memx *memx)
    {
    struct nvkm_device *device = memx.pmu.subdev.device;
    int i;
    if (memx.c.mthd) {
    nvkm_wr32(device, 0x10a1c4, (memx.c.size << 16) | memx.c.mthd);
    for (i = 0; i < memx.c.size; i++)
    nvkm_wr32(device, 0x10a1c4, memx.c.data[i]);
    memx.c.mthd = 0;
    memx.c.size = 0;
    }
    }
    static void
    memx_cmd(struct nvkm_memx *memx, u32 mthd, u32 size, u32 data[])
    {
    if ((memx.c.size + size >= ARRAY_SIZE(memx.c.data)) ||
    (memx.c.mthd && memx.c.mthd != mthd))
    memx_out(memx);
    memcpy(&memx.c.data[memx.c.size], data, size * sizeof(data[0]));
    memx.c.size += size;
    memx.c.mthd  = mthd;
    }
    int
    nvkm_memx_init(struct nvkm_pmu *pmu, struct nvkm_memx **pmemx)
    {
    struct nvkm_device *device = pmu.subdev.device;
    struct nvkm_memx *memx;
    u32 reply[2];
    int ret;
    ret = nvkm_pmu_send(pmu, reply, PROC_MEMX, MEMX_MSG_INFO,
    MEMX_INFO_DATA, 0);
    if (ret)
    return ret;
    memx = *pmemx = kzalloc_obj(*memx);
    if (!memx)
    return -ENOMEM;
    memx.pmu = pmu;
    memx.base = reply[0];
    memx.size = reply[1];
// acquire data segment access
    do {
    nvkm_wr32(device, 0x10a580, 0x00000003);
    } while (nvkm_rd32(device, 0x10a580) != 0x00000003);
    nvkm_wr32(device, 0x10a1c0, 0x01000000 | memx.base);
    return 0;
    }
    int
    nvkm_memx_fini(struct nvkm_memx **pmemx, bool exec)
    {
    struct nvkm_memx *memx = *pmemx;
    struct nvkm_pmu *pmu = memx.pmu;
    struct nvkm_subdev *subdev = &pmu.subdev;
    struct nvkm_device *device = subdev.device;
    u32 finish, reply[2];
// flush the cache...
    memx_out(memx);
// release data segment access
    finish = nvkm_rd32(device, 0x10a1c0) & 0x00ffffff;
    nvkm_wr32(device, 0x10a580, 0x00000000);
// call MEMX process to execute the script, and wait for reply
    if (exec) {
    nvkm_pmu_send(pmu, reply, PROC_MEMX, MEMX_MSG_EXEC,
    memx.base, finish);
    nvkm_debug(subdev, "Exec took %uns, PMU_IN %08x\n",
    reply[0], reply[1]);
    }
    kfree(memx);
    return 0;
    }
    void
    nvkm_memx_wr32(struct nvkm_memx *memx, u32 addr, u32 data)
    {
    nvkm_debug(&memx.pmu.subdev, "R[%06x] = %08x\n", addr, data);
    memx_cmd(memx, MEMX_WR32, 2, (u32[]){ addr, data });
    }
    void
    nvkm_memx_wait(struct nvkm_memx *memx,
    u32 addr, u32 mask, u32 data, u32 nsec)
    {
    nvkm_debug(&memx.pmu.subdev, "R[%06x] & %08x == %08x, %d us\n",
    addr, mask, data, nsec);
    memx_cmd(memx, MEMX_WAIT, 4, (u32[]){ addr, mask, data, nsec });
    memx_out(memx); /* fuc can't handle multiple */
    }
    void
    nvkm_memx_nsec(struct nvkm_memx *memx, u32 nsec)
    {
    nvkm_debug(&memx.pmu.subdev, "    DELAY = %d ns\n", nsec);
    memx_cmd(memx, MEMX_DELAY, 1, (u32[]){ nsec });
    memx_out(memx); /* fuc can't handle multiple */
    }
    void
    nvkm_memx_wait_vblank(struct nvkm_memx *memx)
    {
    struct nvkm_subdev *subdev = &memx.pmu.subdev;
    struct nvkm_device *device = subdev.device;
    u32 heads, x, y, px = 0;
    int i, head_sync;
    if (device.chipset < 0xd0) {
    heads = nvkm_rd32(device, 0x610050);
    for (i = 0; i < 2; i++) {
// Heuristic: sync to head with biggest resolution
    if (heads & (2 << (i << 3))) {
    x = nvkm_rd32(device, 0x610b40 + (0x540 * i));
    y = (x & 0xffff0000) >> 16;
    x &= 0x0000ffff;
    if ((x * y) > px) {
    px = (x * y);
    head_sync = i;
    }
    }
    }
    }
    if (px == 0) {
    nvkm_debug(subdev, "WAIT VBLANK !NO ACTIVE HEAD\n");
    return;
    }
    nvkm_debug(subdev, "WAIT VBLANK HEAD%d\n", head_sync);
    memx_cmd(memx, MEMX_VBLANK, 1, (u32[]){ head_sync });
    memx_out(memx); /* fuc can't handle multiple */
    }
    void
    nvkm_memx_train(struct nvkm_memx *memx)
    {
    nvkm_debug(&memx.pmu.subdev, "   MEM TRAIN\n");
    memx_cmd(memx, MEMX_TRAIN, 0, core::ptr::null_mut());
    }
    int
    nvkm_memx_train_result(struct nvkm_pmu *pmu, u32 *res, int rsize)
    {
    struct nvkm_device *device = pmu.subdev.device;
    u32 reply[2], base, size, i;
    int ret;
    ret = nvkm_pmu_send(pmu, reply, PROC_MEMX, MEMX_MSG_INFO,
    MEMX_INFO_TRAIN, 0);
    if (ret)
    return ret;
    base = reply[0];
    size = reply[1] >> 2;
    if (size > rsize)
    return -ENOMEM;
// read the packet
    nvkm_wr32(device, 0x10a1c0, 0x02000000 | base);
    for (i = 0; i < size; i++)
    res[i] = nvkm_rd32(device, 0x10a1c4);
    return 0;
    }
    void
    nvkm_memx_block(struct nvkm_memx *memx)
    {
    nvkm_debug(&memx.pmu.subdev, "   HOST BLOCKED\n");
    memx_cmd(memx, MEMX_ENTER, 0, core::ptr::null_mut());
    }
    void
    nvkm_memx_unblock(struct nvkm_memx *memx)
    {
    nvkm_debug(&memx.pmu.subdev, "   HOST UNBLOCKED\n");
    memx_cmd(memx, MEMX_LEAVE, 0, core::ptr::null_mut());
    }
