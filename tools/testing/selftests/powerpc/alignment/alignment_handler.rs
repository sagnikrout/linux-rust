//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/alignment/alignment_handler.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Test the powerpc alignment handler on POWER8/POWER9
//
// Copyright (C) 2017 IBM Corporation (Michael Neuling, Andrew Donnellan)
//
// This selftest exercises the powerpc alignment fault handler.
//
// We create two sets of source and destination buffers, one in regular memory,
// the other cache-inhibited (by default we use /dev/fb0 for this, but an
// alterative path for cache-inhibited memory may be provided, e.g. memtrace).
//
// We initialise the source buffers, then use whichever set of load/store
// instructions is under test to copy bytes from the source buffers to the
// destination buffers. For the regular buffers, these instructions will
// execute normally. For the cache-inhibited buffers, these instructions
// will trap and cause an alignment fault, and the alignment fault handler
// will emulate the particular instruction under test. We then compare the
// destination buffers to ensure that the native and emulated cases give the
// same result.
//
// TODO:
// - Any FIXMEs below
// - Test VSX regs < 32 and > 32
// - Test all loads and stores
// - Check update forms do update register
// - Test alignment faults over page boundary
//
// Some old binutils may not support all the instructions.
//

    int bufsize;
    int debug;
    int testing;
    volatile int gotsig;
    bool prefixes_enabled;
    char *cipath = "/dev/fb0";
    long cioffset;
#[no_mangle]
pub unsafe extern "C" fn sighandler(sig: c_int, info: *mut siginfo_t, ctx: *mut c_void) {
    void sighandler(int sig, siginfo_t *info, void *ctx)
    {
    ucontext_t *ucp = ctx;
    if (!testing) {
    signal(sig, SIG_DFL);
    kill(0, sig);
    }
    gotsig = sig;

    if (prefixes_enabled) {
    let mut inst: u32 = *(u32 *)ucp.uc_mcontext.gp_regs[PT_NIP];
    ucp.uc_mcontext.gp_regs[PT_NIP] += ((inst >> 26 == 1) ? 8 : 4);
    } else {
    ucp.uc_mcontext.gp_regs[PT_NIP] += 4;
    }

    ucp.uc_mcontext.uc_regs.gregs[PT_NIP] += 4;

    }

    void test_##name(char *s, char *d)			\
    {							\
    asm volatile(					\

    :: "r"(s), "r"(d), "r"(0)		\
    : "memory", "vs0", "vs32", "r31");	\
    }							\
    rc |= do_test(#name, test_##name)

    void test_##name(char *s, char *d)			\
    {							\
    asm volatile(					\
    ld_op(ld_reg, %0, 0, 0)			\
    st_op(st_reg, %1, 0, 0)			\
    :: "r"(s), "r"(d), "r"(0)		\
    : "memory", "vs0", "vs32", "r31");	\
    }							\
    rc |= do_test(#name, test_##name)

// FIXME: Unimplemented tests:
// STORE_DFORM_TEST(stq)   /* FIXME: need two registers for quad
// STORE_DFORM_TEST(stswi) /* FIXME: string instruction
// STORE_XFORM_TEST(stwat) /* AMO can't emulate or run on CI
// STORE_XFORM_TEST(stdat) /* ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
// preload byte by byte
#[no_mangle]
pub unsafe extern "C" fn preload_data(dst: *mut c_void, offset: c_int, width: c_int) {
    void preload_data(void *dst, int offset, int width)
    {
    char *c = dst;
    int i;
    c += offset;
    for (i = 0 ; i < width ; i++)
    c[i] = i;
    }
    int test_memcpy(void *dst, void *src, int size, int offset,
    void (*test_func)(char *, char *))
    {
    char *s, *d;
    s = src;
    s += offset;
    d = dst;
    d += offset;
    assert(size == 16);
    gotsig = 0;
    testing = 1;
    test_func(s, d); /* run the actual test */
    testing = 0;
    if (gotsig) {
    if (debug)
    printf("  Got signal %i\n", gotsig);
    return 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dumpdata(s1: *mut c_char, s2: *mut c_char, n: c_int, test_name: *mut c_char) {
    void dumpdata(char *s1, char *s2, int n, char *test_name)
    {
    int i;
    printf("  %s: unexpected result:\n", test_name);
    printf("    mem:");
    for (i = 0; i < n; i++)
    printf(" %02x", s1[i]);
    printf("\n");
    printf("    ci: ");
    for (i = 0; i < n; i++)
    printf(" %02x", s2[i]);
    printf("\n");
    }
#[no_mangle]
pub unsafe extern "C" fn test_memcmp(s1: *mut c_void, s2: *mut c_void, n: c_int, offset: c_int, test_name: *mut c_char) -> c_int {
    int test_memcmp(void *s1, void *s2, int n, int offset, char *test_name)
    {
    char *s1c, *s2c;
    s1c = s1;
    s1c += offset;
    s2c = s2;
    s2c += offset;
    if (memcmp(s1c, s2c, n)) {
    if (debug) {
    printf("\n  Compare failed. Offset:%i length:%i\n",
    offset, n);
    dumpdata(s1c, s2c, n, test_name);
    }
    return 1;
    }
    return 0;
    }
//
// Do two memcpy tests using the same instructions. One cachable
// memory and the other doesn't.
//
#[no_mangle]
pub unsafe extern "C" fn do_test(test_name: *mut c_char, : *mut *mut void (test_func)(char, ): *mut c_char) -> c_int {
    int do_test(char *test_name, void (*test_func)(char *, char *))
    {
    int offset, width, fd, rc, r;
    void *mem0, *mem1, *ci0, *ci1;
    printf("\tDoing %s:\t", test_name);
    fd = open(cipath, O_RDWR);
    if (fd < 0) {
    printf("\n");
    perror("Can't open ci file now?");
    return 1;
    }
    ci0 = mmap(core::ptr::null_mut(), bufsize, PROT_WRITE | PROT_READ, MAP_SHARED,
    fd, cioffset);
    ci1 = mmap(core::ptr::null_mut(), bufsize, PROT_WRITE | PROT_READ, MAP_SHARED,
    fd, cioffset + bufsize);
    if ((ci0 == MAP_FAILED) || (ci1 == MAP_FAILED)) {
    printf("\n");
    perror("mmap failed");
    SKIP_IF(1);
    }
    rc = posix_memalign(&mem0, bufsize, bufsize);
    if (rc) {
    printf("\n");
    return rc;
    }
    rc = posix_memalign(&mem1, bufsize, bufsize);
    if (rc) {
    printf("\n");
    free(mem0);
    return rc;
    }
    rc = 0;
//
// offset = 0 is aligned but tests the workaround for the P9N
// DD2.1 vector CI load issue (see 5080332c2c89 "powerpc/64s:
// Add workaround for P9 vector CI load issue")
//
    for (offset = 0; offset < 16; offset++) {
    width = 16; /* vsx == 16 bytes */
    r = 0;
// load pattern into memory byte by byte
    preload_data(ci0, offset, width);
    preload_data(mem0, offset, width); // FIXME: remove??
    memcpy(ci0, mem0, bufsize);
    memcpy(ci1, mem1, bufsize); /* initialise output to the same */
// sanity check
    test_memcmp(mem0, ci0, width, offset, test_name);
    r |= test_memcpy(ci1,  ci0,  width, offset, test_func);
    r |= test_memcpy(mem1, mem0, width, offset, test_func);
    if (r && !debug) {
    printf("FAILED: Got signal");
    rc = 1;
    break;
    }
    r |= test_memcmp(mem1, ci1, width, offset, test_name);
    if (r && !debug) {
    printf("FAILED: Wrong Data");
    rc = 1;
    break;
    }
    }
    if (rc == 0)
    printf("PASSED");
    printf("\n");
    munmap(ci0, bufsize);
    munmap(ci1, bufsize);
    free(mem0);
    free(mem1);
    close(fd);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn can_open_cifile() -> bool {
    static bool can_open_cifile(void)
    {
    int fd;
    fd = open(cipath, O_RDWR);
    if (fd < 0)
    return false;
    close(fd);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn test_alignment_handler_vsx_206() -> c_int {
    int test_alignment_handler_vsx_206(void)
    {
    let mut rc: c_int = 0;
    SKIP_IF(!can_open_cifile());
    SKIP_IF(!have_hwcap(PPC_FEATURE_ARCH_2_06));
    printf("VSX: 2.06B\n");
    LOAD_VSX_XFORM_TEST(lxvd2x);
    LOAD_VSX_XFORM_TEST(lxvw4x);
    LOAD_VSX_XFORM_TEST(lxsdx);
    LOAD_VSX_XFORM_TEST(lxvdsx);
    STORE_VSX_XFORM_TEST(stxvd2x);
    STORE_VSX_XFORM_TEST(stxvw4x);
    STORE_VSX_XFORM_TEST(stxsdx);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn test_alignment_handler_vsx_207() -> c_int {
    int test_alignment_handler_vsx_207(void)
    {
    let mut rc: c_int = 0;
    SKIP_IF(!can_open_cifile());
    SKIP_IF(!have_hwcap2(PPC_FEATURE2_ARCH_2_07));
    printf("VSX: 2.07B\n");
    LOAD_VSX_XFORM_TEST(lxsspx);
    LOAD_VSX_XFORM_TEST(lxsiwax);
    LOAD_VSX_XFORM_TEST(lxsiwzx);
    STORE_VSX_XFORM_TEST(stxsspx);
    STORE_VSX_XFORM_TEST(stxsiwx);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn test_alignment_handler_vsx_300() -> c_int {
    int test_alignment_handler_vsx_300(void)
    {
    let mut rc: c_int = 0;
    SKIP_IF(!can_open_cifile());
    SKIP_IF(!have_hwcap2(PPC_FEATURE2_ARCH_3_00));
    printf("VSX: 3.00B\n");
    LOAD_VMX_DFORM_TEST(lxsd);
    LOAD_VSX_XFORM_TEST(lxsibzx);
    LOAD_VSX_XFORM_TEST(lxsihzx);
    LOAD_VMX_DFORM_TEST(lxssp);
    LOAD_VSX_DFORM_TEST(lxv);
    LOAD_VSX_XFORM_TEST(lxvb16x);
    LOAD_VSX_XFORM_TEST(lxvh8x);
    LOAD_VSX_XFORM_TEST(lxvx);
    LOAD_VSX_XFORM_TEST(lxvwsx);
    LOAD_VSX_XFORM_TEST(lxvl);
    LOAD_VSX_XFORM_TEST(lxvll);
    STORE_VMX_DFORM_TEST(stxsd);
    STORE_VSX_XFORM_TEST(stxsibx);
    STORE_VSX_XFORM_TEST(stxsihx);
    STORE_VMX_DFORM_TEST(stxssp);
    STORE_VSX_DFORM_TEST(stxv);
    STORE_VSX_XFORM_TEST(stxvb16x);
    STORE_VSX_XFORM_TEST(stxvh8x);
    STORE_VSX_XFORM_TEST(stxvx);
    STORE_VSX_XFORM_TEST(stxvl);
    STORE_VSX_XFORM_TEST(stxvll);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn test_alignment_handler_vsx_prefix() -> c_int {
    int test_alignment_handler_vsx_prefix(void)
    {
    let mut rc: c_int = 0;
    SKIP_IF(!can_open_cifile());
    SKIP_IF(!have_hwcap2(PPC_FEATURE2_ARCH_3_1));
    printf("VSX: PREFIX\n");
    LOAD_VSX_8LS_PREFIX_TEST(PLXSD, 0);
    LOAD_VSX_8LS_PREFIX_TEST(PLXSSP, 0);
    LOAD_VSX_8LS_PREFIX_TEST(PLXV0, 0);
    LOAD_VSX_8LS_PREFIX_TEST(PLXV1, 1);
    STORE_VSX_8LS_PREFIX_TEST(PSTXSD, 0);
    STORE_VSX_8LS_PREFIX_TEST(PSTXSSP, 0);
    STORE_VSX_8LS_PREFIX_TEST(PSTXV0, 0);
    STORE_VSX_8LS_PREFIX_TEST(PSTXV1, 1);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn test_alignment_handler_integer() -> c_int {
    int test_alignment_handler_integer(void)
    {
    let mut rc: c_int = 0;
    SKIP_IF(!can_open_cifile());
    printf("Integer\n");
    LOAD_DFORM_TEST(lbz);
    LOAD_DFORM_TEST(lbzu);
    LOAD_XFORM_TEST(lbzx);
    LOAD_XFORM_TEST(lbzux);
    LOAD_DFORM_TEST(lhz);
    LOAD_DFORM_TEST(lhzu);
    LOAD_XFORM_TEST(lhzx);
    LOAD_XFORM_TEST(lhzux);
    LOAD_DFORM_TEST(lha);
    LOAD_DFORM_TEST(lhau);
    LOAD_XFORM_TEST(lhax);
    LOAD_XFORM_TEST(lhaux);
    LOAD_XFORM_TEST(lhbrx);
    LOAD_DFORM_TEST(lwz);
    LOAD_DFORM_TEST(lwzu);
    LOAD_XFORM_TEST(lwzx);
    LOAD_XFORM_TEST(lwzux);
    LOAD_DFORM_TEST(lwa);
    LOAD_XFORM_TEST(lwax);
    LOAD_XFORM_TEST(lwaux);
    LOAD_XFORM_TEST(lwbrx);
    LOAD_DFORM_TEST(ld);
    LOAD_DFORM_TEST(ldu);
    LOAD_XFORM_TEST(ldx);
    LOAD_XFORM_TEST(ldux);
    STORE_DFORM_TEST(stb);
    STORE_XFORM_TEST(stbx);
    STORE_DFORM_TEST(stbu);
    STORE_XFORM_TEST(stbux);
    STORE_DFORM_TEST(sth);
    STORE_XFORM_TEST(sthx);
    STORE_DFORM_TEST(sthu);
    STORE_XFORM_TEST(sthux);
    STORE_XFORM_TEST(sthbrx);
    STORE_DFORM_TEST(stw);
    STORE_XFORM_TEST(stwx);
    STORE_DFORM_TEST(stwu);
    STORE_XFORM_TEST(stwux);
    STORE_XFORM_TEST(stwbrx);
    STORE_DFORM_TEST(std);
    STORE_XFORM_TEST(stdx);
    STORE_DFORM_TEST(stdu);
    STORE_XFORM_TEST(stdux);

    LOAD_DFORM_TEST(lmw);
    STORE_DFORM_TEST(stmw);

    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn test_alignment_handler_integer_206() -> c_int {
    int test_alignment_handler_integer_206(void)
    {
    let mut rc: c_int = 0;
    SKIP_IF(!can_open_cifile());
    SKIP_IF(!have_hwcap(PPC_FEATURE_ARCH_2_06));
    printf("Integer: 2.06\n");
    LOAD_XFORM_TEST(ldbrx);
    STORE_XFORM_TEST(stdbrx);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn test_alignment_handler_integer_prefix() -> c_int {
    int test_alignment_handler_integer_prefix(void)
    {
    let mut rc: c_int = 0;
    SKIP_IF(!can_open_cifile());
    SKIP_IF(!have_hwcap2(PPC_FEATURE2_ARCH_3_1));
    printf("Integer: PREFIX\n");
    LOAD_MLS_PREFIX_TEST(PLBZ);
    LOAD_MLS_PREFIX_TEST(PLHZ);
    LOAD_MLS_PREFIX_TEST(PLHA);
    LOAD_MLS_PREFIX_TEST(PLWZ);
    LOAD_8LS_PREFIX_TEST(PLWA);
    LOAD_8LS_PREFIX_TEST(PLD);
    STORE_MLS_PREFIX_TEST(PSTB);
    STORE_MLS_PREFIX_TEST(PSTH);
    STORE_MLS_PREFIX_TEST(PSTW);
    STORE_8LS_PREFIX_TEST(PSTD);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn test_alignment_handler_vmx() -> c_int {
    int test_alignment_handler_vmx(void)
    {
    let mut rc: c_int = 0;
    SKIP_IF(!can_open_cifile());
    SKIP_IF(!have_hwcap(PPC_FEATURE_HAS_ALTIVEC));
    printf("VMX\n");
    LOAD_VMX_XFORM_TEST(lvx);
//
// FIXME: These loads only load part of the register, so our
// testing method doesn't work. Also they don't take alignment
// faults, so it's kinda pointless anyway
//
    LOAD_VMX_XFORM_TEST(lvebx)
    LOAD_VMX_XFORM_TEST(lvehx)
    LOAD_VMX_XFORM_TEST(lvewx)
    LOAD_VMX_XFORM_TEST(lvxl)
//
    STORE_VMX_XFORM_TEST(stvx);
    STORE_VMX_XFORM_TEST(stvebx);
    STORE_VMX_XFORM_TEST(stvehx);
    STORE_VMX_XFORM_TEST(stvewx);
    STORE_VMX_XFORM_TEST(stvxl);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn test_alignment_handler_fp() -> c_int {
    int test_alignment_handler_fp(void)
    {
    let mut rc: c_int = 0;
    SKIP_IF(!can_open_cifile());
    printf("Floating point\n");
    LOAD_FLOAT_DFORM_TEST(lfd);
    LOAD_FLOAT_XFORM_TEST(lfdx);
    LOAD_FLOAT_DFORM_TEST(lfdu);
    LOAD_FLOAT_XFORM_TEST(lfdux);
    LOAD_FLOAT_DFORM_TEST(lfs);
    LOAD_FLOAT_XFORM_TEST(lfsx);
    LOAD_FLOAT_DFORM_TEST(lfsu);
    LOAD_FLOAT_XFORM_TEST(lfsux);
    STORE_FLOAT_DFORM_TEST(stfd);
    STORE_FLOAT_XFORM_TEST(stfdx);
    STORE_FLOAT_DFORM_TEST(stfdu);
    STORE_FLOAT_XFORM_TEST(stfdux);
    STORE_FLOAT_DFORM_TEST(stfs);
    STORE_FLOAT_XFORM_TEST(stfsx);
    STORE_FLOAT_DFORM_TEST(stfsu);
    STORE_FLOAT_XFORM_TEST(stfsux);
    STORE_FLOAT_XFORM_TEST(stfiwx);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn test_alignment_handler_fp_205() -> c_int {
    int test_alignment_handler_fp_205(void)
    {
    let mut rc: c_int = 0;
    SKIP_IF(!can_open_cifile());
    SKIP_IF(!have_hwcap(PPC_FEATURE_ARCH_2_05));
    printf("Floating point: 2.05\n");
    LOAD_FLOAT_DFORM_TEST(lfdp);
    LOAD_FLOAT_XFORM_TEST(lfdpx);
    LOAD_FLOAT_XFORM_TEST(lfiwax);
    STORE_FLOAT_DFORM_TEST(stfdp);
    STORE_FLOAT_XFORM_TEST(stfdpx);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn test_alignment_handler_fp_206() -> c_int {
    int test_alignment_handler_fp_206(void)
    {
    let mut rc: c_int = 0;
    SKIP_IF(!can_open_cifile());
    SKIP_IF(!have_hwcap(PPC_FEATURE_ARCH_2_06));
    printf("Floating point: 2.06\n");
    LOAD_FLOAT_XFORM_TEST(lfiwzx);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn test_alignment_handler_fp_prefix() -> c_int {
    int test_alignment_handler_fp_prefix(void)
    {
    let mut rc: c_int = 0;
    SKIP_IF(!can_open_cifile());
    SKIP_IF(!have_hwcap2(PPC_FEATURE2_ARCH_3_1));
    printf("Floating point: PREFIX\n");
    LOAD_FLOAT_DFORM_TEST(lfs);
    LOAD_FLOAT_MLS_PREFIX_TEST(PLFS);
    LOAD_FLOAT_MLS_PREFIX_TEST(PLFD);
    STORE_FLOAT_MLS_PREFIX_TEST(PSTFS);
    STORE_FLOAT_MLS_PREFIX_TEST(PSTFD);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn usage(prog: *mut c_char) {
    void usage(char *prog)
    {
    printf("Usage: %s [options] [path [offset]]\n", prog);
    printf("  -d	Enable debug error output\n");
    printf("\n");
    printf("This test requires a POWER8, POWER9 or POWER10 CPU ");
    printf("and either a usable framebuffer at /dev/fb0 or ");
    printf("the path to usable cache inhibited memory and optional ");
    printf("offset to be provided\n");
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    struct sigaction sa;
    let mut rc: c_int = 0;
    let mut option: c_int = 0;
    while ((option = getopt(argc, argv, "d")) != -1) {
    switch (option) {
    case 'd':
    debug++;
    break;
    default:
    usage(argv[0]);
    exit(1);
    }
    }
    argc -= optind;
    argv += optind;
    if (argc > 0)
    cipath = argv[0];
    if (argc > 1)
    cioffset = strtol(argv[1], 0, 0x10);
    bufsize = getpagesize();
    sa.sa_sigaction = sighandler;
    sigemptyset(&sa.sa_mask);
    sa.sa_flags = SA_SIGINFO;
    if (sigaction(SIGSEGV, &sa, core::ptr::null_mut()) == -1
    || sigaction(SIGBUS, &sa, core::ptr::null_mut()) == -1
    || sigaction(SIGILL, &sa, core::ptr::null_mut()) == -1) {
    perror("sigaction");
    exit(1);
    }
    prefixes_enabled = have_hwcap2(PPC_FEATURE2_ARCH_3_1);
    rc |= test_harness(test_alignment_handler_vsx_206,
    "test_alignment_handler_vsx_206");
    rc |= test_harness(test_alignment_handler_vsx_207,
    "test_alignment_handler_vsx_207");
    rc |= test_harness(test_alignment_handler_vsx_300,
    "test_alignment_handler_vsx_300");
    rc |= test_harness(test_alignment_handler_vsx_prefix,
    "test_alignment_handler_vsx_prefix");
    rc |= test_harness(test_alignment_handler_integer,
    "test_alignment_handler_integer");
    rc |= test_harness(test_alignment_handler_integer_206,
    "test_alignment_handler_integer_206");
    rc |= test_harness(test_alignment_handler_integer_prefix,
    "test_alignment_handler_integer_prefix");
    rc |= test_harness(test_alignment_handler_vmx,
    "test_alignment_handler_vmx");
    rc |= test_harness(test_alignment_handler_fp,
    "test_alignment_handler_fp");
    rc |= test_harness(test_alignment_handler_fp_205,
    "test_alignment_handler_fp_205");
    rc |= test_harness(test_alignment_handler_fp_206,
    "test_alignment_handler_fp_206");
    rc |= test_harness(test_alignment_handler_fp_prefix,
    "test_alignment_handler_fp_prefix");
    return rc;
    }
