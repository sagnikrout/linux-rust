//! Automatically rewritten from C to Rust
//! Source: samples/bpf/fds_example.c
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


pub const BPF_M_UNSPEC: c_int = 0;
pub const BPF_M_MAP: c_int = 1;
pub const BPF_M_PROG: c_int = 2;
    char bpf_log_buf[BPF_LOG_BUF_SIZE];
#[no_mangle]
unsafe extern "C" fn usage() {
    static void usage(void)
    {
    printf("Usage: fds_example [...]\n");
    printf("       -F <file>   File to pin/get object\n");
    printf("       -P          |- pin object\n");
    printf("       -G          `- get object\n");
    printf("       -m          eBPF map mode\n");
    printf("       -k <key>    |- map key\n");
    printf("       -v <value>  `- map value\n");
    printf("       -p          eBPF prog mode\n");
    printf("       -o <object> `- object file\n");
    printf("       -h          Display this help.\n");
    }
#[no_mangle]
unsafe extern "C" fn bpf_prog_create(object: *const c_char) -> c_int {
    static int bpf_prog_create(const char *object)
    {
    static struct bpf_insn insns[] = {
    BPF_MOV64_IMM(BPF_REG_0, 1),
    BPF_EXIT_INSN(),
    };
    let mut insns_cnt: usize = ARRAY_SIZE(insns);
    struct bpf_object *obj;
    int err;
    if (object) {
    obj = bpf_object__open_file(object, core::ptr::null_mut());
    assert(!libbpf_get_error(obj));
    err = bpf_object__load(obj);
    assert(!err);
    return bpf_program__fd(bpf_object__next_program(obj, core::ptr::null_mut()));
    } else {
    LIBBPF_OPTS(bpf_prog_load_opts, opts,
    .log_buf = bpf_log_buf,
    .log_size = BPF_LOG_BUF_SIZE,
    );
    return bpf_prog_load(BPF_PROG_TYPE_SOCKET_FILTER, core::ptr::null_mut(), "GPL",
    insns, insns_cnt, &opts);
    }
    }
    static int bpf_do_map(const char *file, uint32_t flags, uint32_t key,
    uint32_t value)
    {
    int fd, ret;
    if (flags & BPF_F_PIN) {
    fd = bpf_map_create(BPF_MAP_TYPE_ARRAY, core::ptr::null_mut(), sizeof(uint32_t),
    sizeof(uint32_t), 1024, core::ptr::null_mut());
    printf("bpf: map fd:%d (%s)\n", fd, strerror(errno));
    assert(fd > 0);
    ret = bpf_obj_pin(fd, file);
    printf("bpf: pin ret:(%d,%s)\n", ret, strerror(errno));
    assert(ret == 0);
    } else {
    fd = bpf_obj_get(file);
    printf("bpf: get fd:%d (%s)\n", fd, strerror(errno));
    assert(fd > 0);
    }
    if ((flags & BPF_F_KEY_VAL) == BPF_F_KEY_VAL) {
    ret = bpf_map_update_elem(fd, &key, &value, 0);
    printf("bpf: fd:%d u.(%u:%u) ret:(%d,%s)\n", fd, key, value,
    ret, strerror(errno));
    assert(ret == 0);
    } else if (flags & BPF_F_KEY) {
    ret = bpf_map_lookup_elem(fd, &key, &value);
    printf("bpf: fd:%d l.(%u):%u ret:(%d,%s)\n", fd, key, value,
    ret, strerror(errno));
    assert(ret == 0);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bpf_do_prog(file: *const c_char, flags: u32, object: *const c_char) -> c_int {
    static int bpf_do_prog(const char *file, uint32_t flags, const char *object)
    {
    int fd, sock, ret;
    if (flags & BPF_F_PIN) {
    fd = bpf_prog_create(object);
    printf("bpf: prog fd:%d (%s)\n", fd, strerror(errno));
    assert(fd > 0);
    ret = bpf_obj_pin(fd, file);
    printf("bpf: pin ret:(%d,%s)\n", ret, strerror(errno));
    assert(ret == 0);
    } else {
    fd = bpf_obj_get(file);
    printf("bpf: get fd:%d (%s)\n", fd, strerror(errno));
    assert(fd > 0);
    }
    sock = open_raw_sock("lo");
    assert(sock > 0);
    ret = setsockopt(sock, SOL_SOCKET, SO_ATTACH_BPF, &fd, sizeof(fd));
    printf("bpf: sock:%d <- fd:%d attached ret:(%d,%s)\n", sock, fd,
    ret, strerror(errno));
    assert(ret == 0);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    const char *file = core::ptr::null_mut(), *object = core::ptr::null_mut();
    let mut key: u32 = 0, value = 0, flags = 0;
    int opt, mode = BPF_M_UNSPEC;
    while ((opt = getopt(argc, argv, "F:PGmk:v:po:")) != -1) {
    switch (opt) {
// General args
    case 'F':
    file = optarg;
    break;
    case 'P':
    flags |= BPF_F_PIN;
    break;
    case 'G':
    flags |= BPF_F_GET;
    break;
// Map-related args
    case 'm':
    mode = BPF_M_MAP;
    break;
    case 'k':
    key = strtoul(optarg, core::ptr::null_mut(), 0);
    flags |= BPF_F_KEY;
    break;
    case 'v':
    value = strtoul(optarg, core::ptr::null_mut(), 0);
    flags |= BPF_F_VAL;
    break;
// Prog-related args
    case 'p':
    mode = BPF_M_PROG;
    break;
    case 'o':
    object = optarg;
    break;
    default:
    goto out;
    }
    }
    if (!(flags & BPF_F_PIN_GET) || !file)
    goto out;
    switch (mode) {
    case BPF_M_MAP:
    return bpf_do_map(file, flags, key, value);
    case BPF_M_PROG:
    return bpf_do_prog(file, flags, object);
    }
    out:
    usage();
    return -1;
    }
