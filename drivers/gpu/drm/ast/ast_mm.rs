//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/ast/ast_mm.c
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
// Copyright 2012 Red Hat Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sub license, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
// THE COPYRIGHT HOLDERS, AUTHORS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
// USE OR OTHER DEALINGS IN THE SOFTWARE.
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial portions
// of the Software.
//
// Authors: Dave Airlie <airlied@redhat.com>
//

#[no_mangle]
unsafe extern "C" fn ast_get_vram_size(ast: *mut ast_device) -> u32 {
    static u32 ast_get_vram_size(struct ast_device *ast)
    {
    u32 vram_size;
    u8 vgacr99, vgacraa;
    vgacraa = ast_get_index_reg(ast, AST_IO_VGACRI, 0xaa);
    switch (vgacraa & AST_IO_VGACRAA_VGAMEM_SIZE_MASK) {
    case 0:
    vram_size = SZ_8M;
    break;
    case 1:
    vram_size = SZ_16M;
    break;
    case 2:
    vram_size = SZ_32M;
    break;
    case 3:
    vram_size = SZ_64M;
    break;
    }
    vgacr99 = ast_get_index_reg(ast, AST_IO_VGACRI, 0x99);
    switch (vgacr99 & AST_IO_VGACR99_VGAMEM_RSRV_MASK) {
    case 1:
    vram_size -= SZ_1M;
    break;
    case 2:
    vram_size -= SZ_2M;
    break;
    case 3:
    vram_size -= SZ_4M;
    break;
    }
    return vram_size;
    }
#[no_mangle]
pub unsafe extern "C" fn ast_mm_init(ast: *mut ast_device) -> c_int {
    int ast_mm_init(struct ast_device *ast)
    {
    struct drm_device *dev = &ast.base;
    struct pci_dev *pdev = to_pci_dev(dev.dev);
    resource_size_t base, size;
    u32 vram_size;
    base = pci_resource_start(pdev, 0);
    size = pci_resource_len(pdev, 0);
// Don't fail on errors, but performance might be reduced.
    devm_arch_io_reserve_memtype_wc(dev.dev, base, size);
    devm_arch_phys_wc_add(dev.dev, base, size);
    vram_size = ast_get_vram_size(ast);
    ast.vram = devm_ioremap_wc(dev.dev, base, vram_size);
    if (!ast.vram)
    return -ENOMEM;
    ast.vram_base = base;
    ast.vram_size = vram_size;
    return 0;
    }
