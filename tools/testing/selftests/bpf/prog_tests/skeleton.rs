//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/skeleton.c
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
// Copyright (c) 2019 Facebook

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s {
    pub a: c_int,
    pub b: c_longlong,
    pub __attribute__((packed)): },

#[no_mangle]
pub unsafe extern "C" fn test_skeleton() {
    void test_skeleton(void)
    {
    pub err: int duration = 0,,
    pub skel: *mut *mut test_skeleton,
    pub bss: *mut test_skeleton__bss,
    pub data: *mut test_skeleton__data,
    pub data_dyn: *mut test_skeleton__data_dyn,
    pub rodata: *mut test_skeleton__rodata,
    pub rodata_dyn: *mut test_skeleton__rodata_dyn,
    pub kcfg: *mut test_skeleton__kconfig,
    pub elf_bytes: *const c_void,
    pub 0: size_t elf_bytes_sz =,
    pub m: *mut c_void,
    pub fd: int i,,
    pub test_skeleton__open(): skel =,
    if (CHECK(!skel, "skel_open", "failed to open skeleton\n"))
    if (CHECK(skel.kconfig, "skel_kconfig", "kconfig is mmaped()!\n"))
    pub cleanup: goto,
    pub skel->bss: bss =,
    pub skel->data: data =,
    pub skel->data_dyn: data_dyn =,
    pub skel->rodata: rodata =,
    pub skel->rodata_dyn: rodata_dyn =,
    pub "rodata_dyn_name"): ASSERT_STREQ(bpf_map__name(skel->maps.rodata_dyn), ".rodata.dyn",,
    pub "data_dyn_name"): ASSERT_STREQ(bpf_map__name(skel->maps.data_dyn), ".data.dyn",,
// validate values are pre-initialized correctly
    pub -1): CHECK(data->in1 != -1, "in1", "got %d != exp %d\n", data->in1,,
    pub -1): CHECK(data->out1 != -1, "out1", "got %d != exp %d\n", data->out1,,
    pub -1LL): CHECK(data->in2 != -1, "in2", "got %lld != exp %lld\n", data->in2,,
    pub -1LL): CHECK(data->out2 != -1, "out2", "got %lld != exp %lld\n", data->out2,,
    pub 0): CHECK(bss->in3 != 0, "in3", "got %d != exp %d\n", bss->in3,,
    pub 0): CHECK(bss->out3 != 0, "out3", "got %d != exp %d\n", bss->out3,,
    pub 0LL): CHECK(bss->in4 != 0, "in4", "got %lld != exp %lld\n", bss->in4,,
    pub 0LL): CHECK(bss->out4 != 0, "out4", "got %lld != exp %lld\n", bss->out4,,
    pub 0): CHECK(rodata->in.in6 != 0, "in6", "got %d != exp %d\n", rodata->in.in6,,
    pub 0): CHECK(bss->out6 != 0, "out6", "got %d != exp %d\n", bss->out6,,
    pub "in_dynarr_sz"): ASSERT_EQ(rodata_dyn->in_dynarr_sz, 0,,
    pub i++): for (i = 0; i < 4;,
    pub "in_dynarr"): ASSERT_EQ(rodata_dyn->in_dynarr[i], -(i + 1),,
    pub i++): for (i = 0; i < 4;,
    pub "out_dynarr"): ASSERT_EQ(data_dyn->out_dynarr[i], i + 1,,
// validate we can pre-setup global variables, even in .bss
    pub 10: data->in1 =,
    pub 11: data->in2 =,
    pub 12: bss->in3 =,
    pub 13: bss->in4 =,
    pub 14: rodata->in.in6 =,
    pub 4: rodata_dyn->in_dynarr_sz =,
    pub i++): for (i = 0; i < 4;,
    pub 10: rodata_dyn->in_dynarr[i] = i +,
    pub test_skeleton__load(skel): err =,
    if (CHECK(err, "skel_load", "failed to load skeleton: %d\n", err))
    pub cleanup: goto,
// validate pre-setup values are still there
    pub 10): CHECK(data->in1 != 10, "in1", "got %d != exp %d\n", data->in1,,
    pub 11LL): CHECK(data->in2 != 11, "in2", "got %lld != exp %lld\n", data->in2,,
    pub 12): CHECK(bss->in3 != 12, "in3", "got %d != exp %d\n", bss->in3,,
    pub 13LL): CHECK(bss->in4 != 13, "in4", "got %lld != exp %lld\n", bss->in4,,
    pub 14): CHECK(rodata->in.in6 != 14, "in6", "got %d != exp %d\n", rodata->in.in6,,
    pub "in_dynarr_sz"): ASSERT_EQ(rodata_dyn->in_dynarr_sz, 4,,
    pub i++): for (i = 0; i < 4;,
    pub "in_dynarr"): ASSERT_EQ(rodata_dyn->in_dynarr[i], i + 10,,
// now set new values and attach to get them into outX variables
    pub 1: data->in1 =,
    pub 2: data->in2 =,
    pub 3: bss->in3 =,
    pub 4: bss->in4 =,
    pub 5: bss->in5.a =,
    pub 6: bss->in5.b =,
    pub skel->kconfig: kcfg =,
    pub 123: skel->data_read_mostly->read_mostly_var =,
    pub test_skeleton__attach(skel): err =,
    if (CHECK(err, "skel_attach", "skeleton attach failed: %d\n", err))
    pub cleanup: goto,
// trigger tracepoint
    pub 1): CHECK(data->out1 != 1, "res1", "got %d != exp %d\n", data->out1,,
    pub 2): CHECK(data->out2 != 2, "res2", "got %lld != exp %d\n", data->out2,,
    pub 3): CHECK(bss->out3 != 3, "res3", "got %d != exp %d\n", (int)bss->out3,,
    pub 4): CHECK(bss->out4 != 4, "res4", "got %lld != exp %d\n", bss->out4,,
    pub 5): CHECK(bss->out5.a != 5, "res5", "got %d != exp %d\n", bss->out5.a,,
    pub 6): CHECK(bss->out5.b != 6, "res6", "got %lld != exp %d\n", bss->out5.b,,
    pub 14): CHECK(bss->out6 != 14, "res7", "got %d != exp %d\n", bss->out6,,
    CHECK(bss.bpf_syscall != kcfg.CONFIG_BPF_SYSCALL, "ext1",
    pub kcfg->CONFIG_BPF_SYSCALL): "got %d != exp %d\n", bss->bpf_syscall,,
    CHECK(bss.kern_ver != kcfg.LINUX_KERNEL_VERSION, "ext2",
    pub kcfg->LINUX_KERNEL_VERSION): "got %d != exp %d\n", bss->kern_ver,,
    pub i++): for (i = 0; i < 4;,
    pub "out_dynarr"): ASSERT_EQ(data_dyn->out_dynarr[i], i + 10,,
    pub "out_mostly_var"): ASSERT_EQ(skel->bss->out_mostly_var, 123,,
    pub "huge_arr"): ASSERT_EQ(bss->huge_arr[ARRAY_SIZE(bss->huge_arr) - 1], 123,,
    pub bpf_map__fd(skel->maps.data_non_mmapable): fd =,
    pub 0): m = mmap(NULL, getpagesize(), PROT_READ, MAP_SHARED, fd,,
    if (!ASSERT_EQ(m, MAP_FAILED, "unexpected_mmap_success"))
    pub getpagesize()): munmap(m,,
    pub "non_mmap_flags"): ASSERT_EQ(bpf_map__map_flags(skel->maps.data_non_mmapable), 0,,
    pub test_skeleton__elf_bytes(&elf_bytes_sz): elf_bytes =,
    pub "elf_bytes"): ASSERT_OK_PTR(elf_bytes,,
    pub "elf_bytes_sz"): ASSERT_GE(elf_bytes_sz, 0,,
    cleanup:
    }
