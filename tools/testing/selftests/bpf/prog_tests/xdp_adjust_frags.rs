//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/xdp_adjust_frags.c
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

#[no_mangle]
unsafe extern "C" fn test_xdp_update_frags() {
    static void test_xdp_update_frags(void)
    {
    const char *file = "./test_xdp_update_frags.bpf.o";
    int err, prog_fd, max_skb_frags, buf_size, num;
    struct bpf_program *prog;
    struct bpf_object *obj;
    __u32 *offset;
    __u8 *buf;
    FILE *f;
    LIBBPF_OPTS(bpf_test_run_opts, topts);
    obj = bpf_object__open(file);
    if (libbpf_get_error(obj))
    return;
    prog = bpf_object__next_program(obj, core::ptr::null_mut());
    if (bpf_object__load(obj))
    return;
    prog_fd = bpf_program__fd(prog);
    buf = malloc(128);
    if (!ASSERT_OK_PTR(buf, "alloc buf 128b"))
    goto out;
    memset(buf, 0, 128);
    offset = (__u32 *)buf;
// offset = 16;
    buf[*offset] = 0xaa;		/* marker at offset 16 (head) */
    buf[*offset + 15] = 0xaa;	/* marker at offset 31 (head) */
    topts.data_in = buf;
    topts.data_out = buf;
    topts.data_size_in = 128;
    topts.data_size_out = 128;
    err = bpf_prog_test_run_opts(prog_fd, &topts);
// test_xdp_update_frags: buf[16,31]: 0xaa -> 0xbb
    ASSERT_OK(err, "xdp_update_frag");
    ASSERT_EQ(topts.retval, XDP_PASS, "xdp_update_frag retval");
    ASSERT_EQ(buf[16], 0xbb, "xdp_update_frag buf[16]");
    ASSERT_EQ(buf[31], 0xbb, "xdp_update_frag buf[31]");
    free(buf);
    buf = malloc(9000);
    if (!ASSERT_OK_PTR(buf, "alloc buf 9Kb"))
    goto out;
    memset(buf, 0, 9000);
    offset = (__u32 *)buf;
// offset = 5000;
    buf[*offset] = 0xaa;		/* marker at offset 5000 (frag0) */
    buf[*offset + 15] = 0xaa;	/* marker at offset 5015 (frag0) */
    topts.data_in = buf;
    topts.data_out = buf;
    topts.data_size_in = 9000;
    topts.data_size_out = 9000;
    err = bpf_prog_test_run_opts(prog_fd, &topts);
// test_xdp_update_frags: buf[5000,5015]: 0xaa -> 0xbb
    ASSERT_OK(err, "xdp_update_frag");
    ASSERT_EQ(topts.retval, XDP_PASS, "xdp_update_frag retval");
    ASSERT_EQ(buf[5000], 0xbb, "xdp_update_frag buf[5000]");
    ASSERT_EQ(buf[5015], 0xbb, "xdp_update_frag buf[5015]");
    memset(buf, 0, 9000);
    offset = (__u32 *)buf;
// offset = 3510;
    buf[*offset] = 0xaa;		/* marker at offset 3510 (head) */
    buf[*offset + 15] = 0xaa;	/* marker at offset 3525 (frag0) */
    err = bpf_prog_test_run_opts(prog_fd, &topts);
// test_xdp_update_frags: buf[3510,3525]: 0xaa -> 0xbb
    ASSERT_OK(err, "xdp_update_frag");
    ASSERT_EQ(topts.retval, XDP_PASS, "xdp_update_frag retval");
    ASSERT_EQ(buf[3510], 0xbb, "xdp_update_frag buf[3510]");
    ASSERT_EQ(buf[3525], 0xbb, "xdp_update_frag buf[3525]");
    memset(buf, 0, 9000);
    offset = (__u32 *)buf;
// offset = 7606;
    buf[*offset] = 0xaa;		/* marker at offset 7606 (frag0) */
    buf[*offset + 15] = 0xaa;	/* marker at offset 7621 (frag1) */
    err = bpf_prog_test_run_opts(prog_fd, &topts);
// test_xdp_update_frags: buf[7606,7621]: 0xaa -> 0xbb
    ASSERT_OK(err, "xdp_update_frag");
    ASSERT_EQ(topts.retval, XDP_PASS, "xdp_update_frag retval");
    ASSERT_EQ(buf[7606], 0xbb, "xdp_update_frag buf[7606]");
    ASSERT_EQ(buf[7621], 0xbb, "xdp_update_frag buf[7621]");
    free(buf);
// test_xdp_update_frags: unsupported buffer size
    f = fopen("/proc/sys/net/core/max_skb_frags", "r");
    if (!ASSERT_OK_PTR(f, "max_skb_frag file pointer"))
    goto out;
    num = fscanf(f, "%d", &max_skb_frags);
    fclose(f);
    if (!ASSERT_EQ(num, 1, "max_skb_frags read failed"))
    goto out;
// xdp_buff linear area size is always set to 4096 in the
// bpf_prog_test_run_xdp routine.
//
    buf_size = 4096 + (max_skb_frags + 1) * sysconf(_SC_PAGE_SIZE);
    buf = malloc(buf_size);
    if (!ASSERT_OK_PTR(buf, "alloc buf"))
    goto out;
    memset(buf, 0, buf_size);
    offset = (__u32 *)buf;
// offset = 16;
    buf[*offset] = 0xaa;
    buf[*offset + 15] = 0xaa;
    topts.data_in = buf;
    topts.data_out = buf;
    topts.data_size_in = buf_size;
    topts.data_size_out = buf_size;
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_EQ(err, -ENOMEM,
    "unsupported buf size, possible non-default /proc/sys/net/core/max_skb_flags?");
    free(buf);
    out:
    bpf_object__close(obj);
    }
#[no_mangle]
pub unsafe extern "C" fn test_xdp_adjust_frags() {
    void test_xdp_adjust_frags(void)
    {
    if (test__start_subtest("xdp_adjust_frags"))
    test_xdp_update_frags();
    }
