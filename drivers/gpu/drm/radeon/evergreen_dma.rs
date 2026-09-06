//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/radeon/evergreen_dma.c
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


//
// Copyright 2010 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: Alex Deucher
//

//
// evergreen_dma_fence_ring_emit - emit a fence on the DMA ring
//
// @rdev: radeon_device pointer
// @fence: radeon fence object
//
// Add a DMA fence packet to the ring to write
// the fence seq number and DMA trap packet to generate
// an interrupt if needed (evergreen-SI).
//
    void evergreen_dma_fence_ring_emit(struct radeon_device *rdev,
    struct radeon_fence *fence)
    {
    struct radeon_ring *ring = &rdev.ring[fence.ring];
    let mut addr: u64 = rdev.fence_drv[fence.ring].gpu_addr;
// write the fence
    radeon_ring_write(ring, DMA_PACKET(DMA_PACKET_FENCE, 0, 0));
    radeon_ring_write(ring, addr & 0xfffffffc);
    radeon_ring_write(ring, (upper_32_bits(addr) & 0xff));
    radeon_ring_write(ring, fence.seq);
// generate an interrupt
    radeon_ring_write(ring, DMA_PACKET(DMA_PACKET_TRAP, 0, 0));
// flush HDP
    radeon_ring_write(ring, DMA_PACKET(DMA_PACKET_SRBM_WRITE, 0, 0));
    radeon_ring_write(ring, (0xf << 16) | (HDP_MEM_COHERENCY_FLUSH_CNTL >> 2));
    radeon_ring_write(ring, 1);
    }
//
// evergreen_dma_ring_ib_execute - schedule an IB on the DMA engine
//
// @rdev: radeon_device pointer
// @ib: IB object to schedule
//
// Schedule an IB in the DMA ring (evergreen).
//
    void evergreen_dma_ring_ib_execute(struct radeon_device *rdev,
    struct radeon_ib *ib)
    {
    struct radeon_ring *ring = &rdev.ring[ib.ring];
    if (rdev.wb.enabled) {
    let mut next_rptr: u32 = ring.wptr + 4;
    while ((next_rptr & 7) != 5)
    next_rptr++;
    next_rptr += 3;
    radeon_ring_write(ring, DMA_PACKET(DMA_PACKET_WRITE, 0, 1));
    radeon_ring_write(ring, ring.next_rptr_gpu_addr & 0xfffffffc);
    radeon_ring_write(ring, upper_32_bits(ring.next_rptr_gpu_addr) & 0xff);
    radeon_ring_write(ring, next_rptr);
    }
// The indirect buffer packet must end on an 8 DW boundary in the DMA ring.
// Pad as necessary with NOPs.
//
    while ((ring.wptr & 7) != 5)
    radeon_ring_write(ring, DMA_PACKET(DMA_PACKET_NOP, 0, 0));
    radeon_ring_write(ring, DMA_PACKET(DMA_PACKET_INDIRECT_BUFFER, 0, 0));
    radeon_ring_write(ring, (ib.gpu_addr & 0xFFFFFFE0));
    radeon_ring_write(ring, (ib.length_dw << 12) | (upper_32_bits(ib.gpu_addr) & 0xFF));
    }
//
// evergreen_copy_dma - copy pages using the DMA engine
//
// @rdev: radeon_device pointer
// @src_offset: src GPU address
// @dst_offset: dst GPU address
// @num_gpu_pages: number of GPU pages to xfer
// @resv: reservation object with embedded fence
//
// Copy GPU paging using the DMA engine (evergreen-cayman).
// Used by the radeon ttm implementation to move pages if
// registered as the asic copy callback.
//
    struct radeon_fence *evergreen_copy_dma(struct radeon_device *rdev,
    uint64_t src_offset,
    uint64_t dst_offset,
    unsigned num_gpu_pages,
    struct dma_resv *resv)
    {
    struct radeon_fence *fence;
    struct radeon_sync sync;
    let mut ring_index: c_int = rdev.asic.copy.dma_ring_index;
    struct radeon_ring *ring = &rdev.ring[ring_index];
    u32 size_in_dw, cur_size_in_dw;
    int i, num_loops;
    let mut r: c_int = 0;
    radeon_sync_create(&sync);
    size_in_dw = (num_gpu_pages << RADEON_GPU_PAGE_SHIFT) / 4;
    num_loops = DIV_ROUND_UP(size_in_dw, 0xfffff);
    r = radeon_ring_lock(rdev, ring, num_loops * 5 + 11);
    if (r) {
    DRM_ERROR("radeon: moving bo (%d).\n", r);
    radeon_sync_free(rdev, &sync, core::ptr::null_mut());
    return ERR_PTR(r);
    }
    radeon_sync_resv(rdev, &sync, resv, false);
    radeon_sync_rings(rdev, &sync, ring.idx);
    for (i = 0; i < num_loops; i++) {
    cur_size_in_dw = size_in_dw;
    if (cur_size_in_dw > 0xFFFFF)
    cur_size_in_dw = 0xFFFFF;
    size_in_dw -= cur_size_in_dw;
    radeon_ring_write(ring, DMA_PACKET(DMA_PACKET_COPY, 0, cur_size_in_dw));
    radeon_ring_write(ring, dst_offset & 0xfffffffc);
    radeon_ring_write(ring, src_offset & 0xfffffffc);
    radeon_ring_write(ring, upper_32_bits(dst_offset) & 0xff);
    radeon_ring_write(ring, upper_32_bits(src_offset) & 0xff);
    src_offset += cur_size_in_dw * 4;
    dst_offset += cur_size_in_dw * 4;
    }
    r = radeon_fence_emit(rdev, &fence, ring.idx);
    if (r) {
    radeon_ring_unlock_undo(rdev, ring);
    radeon_sync_free(rdev, &sync, core::ptr::null_mut());
    return ERR_PTR(r);
    }
    radeon_ring_unlock_commit(rdev, ring, false);
    radeon_sync_free(rdev, &sync, fence);
    return fence;
    }
//
// evergreen_dma_is_lockup - Check if the DMA engine is locked up
//
// @rdev: radeon_device pointer
// @ring: radeon_ring structure holding ring information
//
// Check if the async DMA engine is locked up.
// Returns true if the engine appears to be locked up, false if not.
//
#[no_mangle]
pub unsafe extern "C" fn evergreen_dma_is_lockup(rdev: *mut radeon_device, ring: *mut radeon_ring) -> bool {
    bool evergreen_dma_is_lockup(struct radeon_device *rdev, struct radeon_ring *ring)
    {
    let mut reset_mask: u32 = evergreen_gpu_check_soft_reset(rdev);
    if (!(reset_mask & RADEON_RESET_DMA)) {
    radeon_ring_lockup_update(rdev, ring);
    return false;
    }
    return radeon_ring_test_lockup(rdev, ring);
    }
