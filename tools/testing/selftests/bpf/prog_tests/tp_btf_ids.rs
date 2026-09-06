//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/tp_btf_ids.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_ids_info {
    pub obj_id: __u32,
    pub raw_id: __u32,
    pub tp_id: __u32,
}

    static const char *btf_ids_path(char *buf, size_t sz)
    {
    if (access(TRACEFS "/trace", F_OK) == 0)
    snprintf(buf, sz, "%s/%s", TRACEFS, EVENT_SUBPATH);
    else
    snprintf(buf, sz, "%s/%s", DEBUGFS_TRACING, EVENT_SUBPATH);
    return buf;
    }
#[no_mangle]
unsafe extern "C" fn read_btf_ids(info: *mut btf_ids_info) -> c_int {
    static int read_btf_ids(struct btf_ids_info *info)
    {
    char path[256], buf[256];
    int fd, n;
    fd = open(btf_ids_path(path, sizeof(path)), O_RDONLY);
    if (fd < 0)
    return -errno;
    n = read(fd, buf, sizeof(buf) - 1);
    close(fd);
    if (n <= 0)
    return -EIO;
    buf[n] = '\0';
    if (sscanf(buf,
    "btf_obj_id: %u\nraw_btf_id: %u\ntp_btf_id: %u\n",
    &info.obj_id, &info.raw_id, &info.tp_id) != 3)
    return -EINVAL;
    return 0;
    }
    static const char *param_name(struct btf *btf, const struct btf_param *p)
    {
    return btf__name_by_offset(btf, p.name_off);
    }
    static const char *member_name(struct btf *btf, const struct btf_member *m)
    {
    return btf__name_by_offset(btf, m.name_off);
    }
#[no_mangle]
pub unsafe extern "C" fn test_tp_btf_ids() {
    void test_tp_btf_ids(void)
    {
    const struct btf_type *proto_t, *rec_t;
    const struct btf_param *params;
    const struct btf_member *members;
    struct btf_ids_info info;
    struct btf *vmlinux_btf, *btf;
    const char *name;
    int err;
    if (!env.has_testmod) {
    test__skip();
    return;
    }
    err = read_btf_ids(&info);
    if (!ASSERT_OK(err, "read btf_ids"))
    return;
    ASSERT_GT(info.obj_id, 0, "obj_id non-zero");
    ASSERT_GT(info.raw_id, 0, "raw_id non-zero");
    ASSERT_GT(info.tp_id, 0, "tp_id non-zero");
    vmlinux_btf = btf__load_vmlinux_btf();
    if (!ASSERT_OK_PTR(vmlinux_btf, "load vmlinux BTF"))
    return;
// Module BTF is split BTF; load with vmlinux as base.
    btf = btf__load_from_kernel_by_id_split(info.obj_id, vmlinux_btf);
    if (!ASSERT_OK_PTR(btf, "load module BTF")) {
    btf__free(vmlinux_btf);
    return;
    }
//
// raw_btf_id should be the FUNC_PROTO of __bpf_trace_<call>:
// void *__data, struct task_struct *task,
// struct bpf_testmod_test_read_ctx *ctx
//
    proto_t = btf__type_by_id(btf, info.raw_id);
    if (!ASSERT_OK_PTR(proto_t, "raw type_by_id"))
    goto out;
    if (!ASSERT_TRUE(btf_is_func_proto(proto_t), "raw is FUNC_PROTO"))
    goto out;
    if (!ASSERT_EQ(btf_vlen(proto_t), 3, "func_proto arg count"))
    goto out;
    params = btf_params(proto_t);
    ASSERT_STREQ(param_name(btf, &params[0]), "__data", "arg0 name");
    ASSERT_STREQ(param_name(btf, &params[1]), "task", "arg1 name");
    ASSERT_STREQ(param_name(btf, &params[2]), "ctx", "arg2 name");
//
// tp_btf_id should be STRUCT trace_event_raw_<call> with the
// fields declared by TP_STRUCT__entry plus the common header.
//
    rec_t = btf__type_by_id(btf, info.tp_id);
    if (!ASSERT_OK_PTR(rec_t, "tp type_by_id"))
    goto out;
    if (!ASSERT_TRUE(btf_is_struct(rec_t), "tp is STRUCT"))
    goto out;
    name = btf__name_by_offset(btf, rec_t.name_off);
    ASSERT_STREQ(name, "trace_event_raw_bpf_testmod_test_read",
    "tp struct name");
    if (!ASSERT_GE(btf_vlen(rec_t), 5, "tp struct field count"))
    goto out;
    members = btf_members(rec_t);
    ASSERT_STREQ(member_name(btf, &members[0]), "ent", "field0 name");
    ASSERT_STREQ(member_name(btf, &members[1]), "pid", "field1 name");
    ASSERT_STREQ(member_name(btf, &members[2]), "comm", "field2 name");
    ASSERT_STREQ(member_name(btf, &members[3]), "off", "field3 name");
    ASSERT_STREQ(member_name(btf, &members[4]), "len", "field4 name");
    out:
    btf__free(btf);
    btf__free(vmlinux_btf);
    }
