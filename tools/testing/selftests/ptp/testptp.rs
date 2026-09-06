//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/ptp/testptp.c
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
// PTP 1588 clock support - User space test program
//
// Copyright (C) 2010 OMICRON electronics GmbH
//
// Macro flag: #define _GNU_SOURCE

pub const ADJ_SETOFFSET: c_uint = 0x0100;

// clock_adjtime is not available in GLIBC < 2.14

#[no_mangle]
unsafe extern "C" fn clock_adjtime(id: clockid_t, tx: *mut timex) -> c_int {
    static int clock_adjtime(clockid_t id, struct timex *tx)
    {
    return syscall(__NR_clock_adjtime, id, tx);
    }

#[no_mangle]
unsafe extern "C" fn show_flag_test(rq_index: c_int, flags: c_uint, err: c_int) {
    static void show_flag_test(int rq_index, unsigned int flags, int err)
    {
    printf("PTP_EXTTS_REQUEST%c flags 0x%08x : (%d) %s\n",
    rq_index ? '1' + rq_index : ' ',
    flags, err, strerror(errno));
// sigh, uClibc ...
    errno = 0;
    }
#[no_mangle]
unsafe extern "C" fn do_flag_test(fd: c_int, index: c_uint) {
    static void do_flag_test(int fd, unsigned int index)
    {
    struct ptp_extts_request extts_request;
    unsigned long request[2] = {
    PTP_EXTTS_REQUEST,
    PTP_EXTTS_REQUEST2,
    };
    unsigned int enable_flags[5] = {
    PTP_ENABLE_FEATURE,
    PTP_ENABLE_FEATURE | PTP_RISING_EDGE,
    PTP_ENABLE_FEATURE | PTP_FALLING_EDGE,
    PTP_ENABLE_FEATURE | PTP_RISING_EDGE | PTP_FALLING_EDGE,
    PTP_ENABLE_FEATURE | (PTP_EXTTS_VALID_FLAGS + 1),
    };
    int err, i, j;
    memset(&extts_request, 0, sizeof(extts_request));
    extts_request.index = index;
    for (i = 0; i < 2; i++) {
    for (j = 0; j < 5; j++) {
    extts_request.flags = enable_flags[j];
    err = ioctl(fd, request[i], &extts_request);
    show_flag_test(i, extts_request.flags, err);
    extts_request.flags = 0;
    err = ioctl(fd, request[i], &extts_request);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn get_clockid(fd: c_int) -> clockid_t {
    static clockid_t get_clockid(int fd)
    {
pub const CLOCKFD: c_int = 3;
    return (((unsigned int) ~fd) << 3) | CLOCKFD;
    }
#[no_mangle]
unsafe extern "C" fn ppb_to_scaled_ppm(ppb: c_int) -> c_long {
    static long ppb_to_scaled_ppm(int ppb)
    {
//
// The 'freq' field in the 'struct timex' is in parts per
// million, but with a 16 bit binary fractional field.
// Instead of calculating either one of
//
// scaled_ppm = (ppb / 1000) << 16  [1]
// scaled_ppm = (ppb << 16) / 1000  [2]
//
// we simply use double precision math, in order to avoid the
// truncation in [1] and the possible overflow in [2].
//
    return (long) (ppb * 65.536);
    }
#[no_mangle]
unsafe extern "C" fn pctns(t: *mut ptp_clock_time) -> i64 {
    static int64_t pctns(struct ptp_clock_time *t)
    {
    return t.sec * NSEC_PER_SEC + t.nsec;
    }
#[no_mangle]
unsafe extern "C" fn usage(progname: *mut c_char) {
    static void usage(char *progname)
    {
    fprintf(stderr,
    "usage: %s [options]\n"
    " -c         query the ptp clock's capabilities\n"
    " -d name    device to open\n"
    " -e val     read 'val' external time stamp events\n"
    " -E val     enable rising (1), falling (2), or both (3) edges\n"
    " -f val     adjust the ptp clock frequency by 'val' ppb\n"
    " -F chan    Enable single channel mask and keep device open for debugfs verification.\n"
    " -g         get the ptp clock time\n"
    " -h         prints this message\n"
    " -i val     index for event/trigger\n"
    " -k val     measure the time offset between system and phc clock\n"
    "            for 'val' times (Maximum 25)\n"
    " -l         list the current pin configuration\n"
    " -L pin,val configure pin index 'pin' with function 'val'\n"
    "            the channel index is taken from the '-i' option\n"
    "            'val' specifies the auxiliary function:\n"
    "            0 - none\n"
    "            1 - external time stamp\n"
    "            2 - periodic output\n"
    " -n val     shift the ptp clock time by 'val' nanoseconds\n"
    " -o val     phase offset (in nanoseconds) to be provided to the PHC servo\n"
    " -p val     enable output with a period of 'val' nanoseconds\n"
    " -H val     set output phase to 'val' nanoseconds (requires -p)\n"
    " -w val     set output pulse width to 'val' nanoseconds (requires -p)\n"
    " -P val     enable or disable (val=1|0) the system clock PPS\n"
    " -r         open the ptp clock in readonly mode\n"
    " -s         set the ptp clock time from the system time\n"
    " -S         set the system time from the ptp clock time\n"
    " -t val     shift the ptp clock time by 'val' seconds\n"
    " -T val     set the ptp clock time to 'val' seconds\n"
    " -x val     get an extended ptp clock time with the desired number of samples (up to %d)\n"
    " -X         get a ptp clock cross timestamp\n"
    " -y val     pre/post tstamp timebase to use {realtime|monotonic|monotonic-raw}\n"
    " -z         test combinations of rising/falling external time stamp flags\n",
    progname, PTP_MAX_SAMPLES);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    struct ptp_clock_caps caps;
    struct ptp_extts_event event;
    struct ptp_extts_request extts_request;
    struct ptp_perout_request perout_request;
    struct ptp_pin_desc desc;
    struct timespec ts;
    struct timex tx;
    struct ptp_clock_time *pct;
    struct ptp_sys_offset *sysoff;
    struct ptp_sys_offset_extended *soe;
    struct ptp_sys_offset_precise *xts;
    char *progname;
    unsigned int i;
    int c, cnt, fd;
    char *device = DEVICE;
    clockid_t clkid;
    let mut adjfreq: c_int = 0x7fffffff;
    let mut adjtime: c_int = 0;
    let mut adjns: c_int = 0;
    let mut adjphase: c_int = 0;
    let mut capabilities: c_int = 0;
    let mut extts: c_int = 0;
    let mut edge: c_int = 0;
    let mut flagtest: c_int = 0;
    let mut gettime: c_int = 0;
    let mut index: c_int = 0;
    let mut list_pins: c_int = 0;
    let mut pct_offset: c_int = 0;
    let mut getextended: c_int = 0;
    let mut getcross: c_int = 0;
    let mut n_samples: c_int = 0;
    let mut pin_index: c_int = -1, pin_func;
    let mut pps: c_int = -1;
    let mut seconds: c_int = 0;
    let mut readonly: c_int = 0;
    let mut settime: c_int = 0;
    let mut channel: c_int = -1;
    let mut ext_clockid: clockid_t = CLOCK_REALTIME;
    int64_t t1, t2, tp;
    int64_t interval, offset;
    let mut perout_phase: i64 = -1;
    let mut pulsewidth: i64 = -1;
    let mut perout: i64 = -1;
    progname = strrchr(argv[0], '/');
    progname = progname ? 1+progname : argv[0];
    while (EOF != (c = getopt(argc, argv, "cd:e:E:f:F:ghH:i:k:lL:n:o:p:P:rsSt:T:w:x:Xy:z"))) {
    switch (c) {
    case 'c':
    capabilities = 1;
    break;
    case 'd':
    device = optarg;
    break;
    case 'e':
    extts = atoi(optarg);
    break;
    case 'E':
    edge = atoi(optarg);
    edge = (edge & 1 ? PTP_RISING_EDGE : 0) |
    (edge & 2 ? PTP_FALLING_EDGE : 0);
    break;
    case 'f':
    adjfreq = atoi(optarg);
    break;
    case 'F':
    channel = atoi(optarg);
    break;
    case 'g':
    gettime = 1;
    break;
    case 'H':
    perout_phase = atoll(optarg);
    break;
    case 'i':
    index = atoi(optarg);
    break;
    case 'k':
    pct_offset = 1;
    n_samples = atoi(optarg);
    break;
    case 'l':
    list_pins = 1;
    break;
    case 'L':
    cnt = sscanf(optarg, "%d,%d", &pin_index, &pin_func);
    if (cnt != 2) {
    usage(progname);
    return -1;
    }
    break;
    case 'n':
    adjns = atoi(optarg);
    break;
    case 'o':
    adjphase = atoi(optarg);
    break;
    case 'p':
    perout = atoll(optarg);
    break;
    case 'P':
    pps = atoi(optarg);
    break;
    case 'r':
    readonly = 1;
    break;
    case 's':
    settime = 1;
    break;
    case 'S':
    settime = 2;
    break;
    case 't':
    adjtime = atoi(optarg);
    break;
    case 'T':
    settime = 3;
    seconds = atoi(optarg);
    break;
    case 'w':
    pulsewidth = atoi(optarg);
    break;
    case 'x':
    getextended = atoi(optarg);
    if (getextended < 1 || getextended > PTP_MAX_SAMPLES) {
    fprintf(stderr,
    "number of extended timestamp samples must be between 1 and %d; was asked for %d\n",
    PTP_MAX_SAMPLES, getextended);
    return -1;
    }
    break;
    case 'X':
    getcross = 1;
    break;
    case 'y':
    if (!strcasecmp(optarg, "realtime"))
    ext_clockid = CLOCK_REALTIME;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcasecmp(optarg, _arg: "monotonic")) -> else {
    else if (!strcasecmp(optarg, "monotonic"))
    ext_clockid = CLOCK_MONOTONIC;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcasecmp(optarg, _arg: "monotonic-raw")) -> else {
    else if (!strcasecmp(optarg, "monotonic-raw"))
    ext_clockid = CLOCK_MONOTONIC_RAW;
    else {
    fprintf(stderr,
    "type needs to be realtime, monotonic or monotonic-raw; was given %s\n",
    optarg);
    return -1;
    }
    break;
    case 'z':
    flagtest = 1;
    break;
    case 'h':
    usage(progname);
    return 0;
    case '?':
    default:
    usage(progname);
    return -1;
    }
    }
    fd = open(device, readonly ? O_RDONLY : O_RDWR);
    if (fd < 0) {
    fprintf(stderr, "opening %s: %s\n", device, strerror(errno));
    return -1;
    }
    clkid = get_clockid(fd);
    if (CLOCK_INVALID == clkid) {
    fprintf(stderr, "failed to read clock id\n");
    return -1;
    }
    if (capabilities) {
    if (ioctl(fd, PTP_CLOCK_GETCAPS, &caps)) {
    perror("PTP_CLOCK_GETCAPS");
    } else {
    printf("capabilities:\n"
    "  %d maximum frequency adjustment (ppb)\n"
    "  %d programmable alarms\n"
    "  %d external time stamp channels\n"
    "  %d programmable periodic signals\n"
    "  %d pulse per second\n"
    "  %d programmable pins\n"
    "  %d cross timestamping\n"
    "  %d adjust_phase\n"
    "  %d maximum phase adjustment (ns)\n",
    caps.max_adj,
    caps.n_alarm,
    caps.n_ext_ts,
    caps.n_per_out,
    caps.pps,
    caps.n_pins,
    caps.cross_timestamping,
    caps.adjust_phase,
    caps.max_phase_adj);
    }
    }
    if (0x7fffffff != adjfreq) {
    memset(&tx, 0, sizeof(tx));
    tx.modes = ADJ_FREQUENCY;
    tx.freq = ppb_to_scaled_ppm(adjfreq);
    if (clock_adjtime(clkid, &tx)) {
    perror("clock_adjtime");
    } else {
    puts("frequency adjustment okay");
    }
    }
    if (adjtime || adjns) {
    memset(&tx, 0, sizeof(tx));
    tx.modes = ADJ_SETOFFSET | ADJ_NANO;
    tx.time.tv_sec = adjtime;
    tx.time.tv_usec = adjns;
    while (tx.time.tv_usec < 0) {
    tx.time.tv_sec  -= 1;
    tx.time.tv_usec += NSEC_PER_SEC;
    }
    if (clock_adjtime(clkid, &tx) < 0) {
    perror("clock_adjtime");
    } else {
    puts("time shift okay");
    }
    }
    if (adjphase) {
    memset(&tx, 0, sizeof(tx));
    tx.modes = ADJ_OFFSET | ADJ_NANO;
    tx.offset = adjphase;
    if (clock_adjtime(clkid, &tx) < 0) {
    perror("clock_adjtime");
    } else {
    puts("phase adjustment okay");
    }
    }
    if (gettime) {
    if (clock_gettime(clkid, &ts)) {
    perror("clock_gettime");
    } else {
    printf("clock time: %ld.%09ld or %s",
    ts.tv_sec, ts.tv_nsec, ctime(&ts.tv_sec));
    }
    }
    if (settime == 1) {
    clock_gettime(CLOCK_REALTIME, &ts);
    if (clock_settime(clkid, &ts)) {
    perror("clock_settime");
    } else {
    puts("set time okay");
    }
    }
    if (settime == 2) {
    clock_gettime(clkid, &ts);
    if (clock_settime(CLOCK_REALTIME, &ts)) {
    perror("clock_settime");
    } else {
    puts("set time okay");
    }
    }
    if (settime == 3) {
    ts.tv_sec = seconds;
    ts.tv_nsec = 0;
    if (clock_settime(clkid, &ts)) {
    perror("clock_settime");
    } else {
    puts("set time okay");
    }
    }
    if (pin_index >= 0) {
    memset(&desc, 0, sizeof(desc));
    desc.index = pin_index;
    desc.func = pin_func;
    desc.chan = index;
    if (ioctl(fd, PTP_PIN_SETFUNC, &desc)) {
    perror("PTP_PIN_SETFUNC");
    } else {
    puts("set pin function okay");
    }
    }
    if (extts) {
    if (!readonly) {
    memset(&extts_request, 0, sizeof(extts_request));
    extts_request.index = index;
    extts_request.flags = PTP_ENABLE_FEATURE | edge;
    if (ioctl(fd, PTP_EXTTS_REQUEST, &extts_request)) {
    perror("PTP_EXTTS_REQUEST");
    extts = 0;
    } else {
    puts("external time stamp request okay");
    }
    }
    for (; extts; extts--) {
    cnt = read(fd, &event, sizeof(event));
    if (cnt != sizeof(event)) {
    perror("read");
    break;
    }
    printf("event index %u at %lld.%09u\n", event.index,
    event.t.sec, event.t.nsec);
    fflush(stdout);
    }
    if (!readonly) {
// Disable the feature again.
    extts_request.flags = 0;
    if (ioctl(fd, PTP_EXTTS_REQUEST, &extts_request)) {
    perror("PTP_EXTTS_REQUEST");
    }
    }
    }
    if (flagtest) {
    do_flag_test(fd, index);
    }
    if (list_pins) {
    let mut n_pins: c_int = 0;
    if (ioctl(fd, PTP_CLOCK_GETCAPS, &caps)) {
    perror("PTP_CLOCK_GETCAPS");
    } else {
    n_pins = caps.n_pins;
    }
    for (i = 0; i < n_pins; i++) {
    desc.index = i;
    if (ioctl(fd, PTP_PIN_GETFUNC, &desc)) {
    perror("PTP_PIN_GETFUNC");
    break;
    }
    printf("name %s index %u func %u chan %u\n",
    desc.name, desc.index, desc.func, desc.chan);
    }
    }
    if (pulsewidth >= 0 && perout < 0) {
    puts("-w can only be specified together with -p");
    return -1;
    }
    if (perout_phase >= 0 && perout < 0) {
    puts("-H can only be specified together with -p");
    return -1;
    }
    if (perout >= 0) {
    if (clock_gettime(clkid, &ts)) {
    perror("clock_gettime");
    return -1;
    }
    memset(&perout_request, 0, sizeof(perout_request));
    perout_request.index = index;
    perout_request.period.sec = perout / NSEC_PER_SEC;
    perout_request.period.nsec = perout % NSEC_PER_SEC;
    perout_request.flags = 0;
    if (pulsewidth >= 0) {
    perout_request.flags |= PTP_PEROUT_DUTY_CYCLE;
    perout_request.on.sec = pulsewidth / NSEC_PER_SEC;
    perout_request.on.nsec = pulsewidth % NSEC_PER_SEC;
    }
    if (perout_phase >= 0) {
    perout_request.flags |= PTP_PEROUT_PHASE;
    perout_request.phase.sec = perout_phase / NSEC_PER_SEC;
    perout_request.phase.nsec = perout_phase % NSEC_PER_SEC;
    } else {
    perout_request.start.sec = ts.tv_sec + 2;
    perout_request.start.nsec = 0;
    }
    if (ioctl(fd, PTP_PEROUT_REQUEST2, &perout_request)) {
    perror("PTP_PEROUT_REQUEST");
    } else {
    puts("periodic output request okay");
    }
    }
    if (pps != -1) {
    let mut enable: c_int = pps ? 1 : 0;
    if (ioctl(fd, PTP_ENABLE_PPS, enable)) {
    perror("PTP_ENABLE_PPS");
    } else {
    puts("pps for system time request okay");
    }
    }
    if (pct_offset) {
    if (n_samples <= 0 || n_samples > 25) {
    puts("n_samples should be between 1 and 25");
    usage(progname);
    return -1;
    }
    sysoff = calloc(1, sizeof(*sysoff));
    if (!sysoff) {
    perror("calloc");
    return -1;
    }
    sysoff.n_samples = n_samples;
    if (ioctl(fd, PTP_SYS_OFFSET, sysoff))
    perror("PTP_SYS_OFFSET");
    else
    puts("system and phc clock time offset request okay");
    pct = &sysoff.ts[0];
    for (i = 0; i < sysoff.n_samples; i++) {
    t1 = pctns(pct+2*i);
    tp = pctns(pct+2*i+1);
    t2 = pctns(pct+2*i+2);
    interval = t2 - t1;
    offset = (t2 + t1) / 2 - tp;
    printf("system time: %lld.%09u\n",
    (pct+2*i).sec, (pct+2*i).nsec);
    printf("phc    time: %lld.%09u\n",
    (pct+2*i+1).sec, (pct+2*i+1).nsec);
    printf("system time: %lld.%09u\n",
    (pct+2*i+2).sec, (pct+2*i+2).nsec);
    printf("system/phc clock time offset is %" PRId64 " ns\n"
    "system     clock time delay  is %" PRId64 " ns\n",
    offset, interval);
    }
    free(sysoff);
    }
    if (getextended) {
    soe = calloc(1, sizeof(*soe));
    if (!soe) {
    perror("calloc");
    return -1;
    }
    soe.n_samples = getextended;
    soe.clockid = ext_clockid;
    if (ioctl(fd, PTP_SYS_OFFSET_EXTENDED, soe)) {
    perror("PTP_SYS_OFFSET_EXTENDED");
    } else {
    printf("extended timestamp request returned %d samples\n",
    getextended);
    for (i = 0; i < getextended; i++) {
    switch (ext_clockid) {
    case CLOCK_REALTIME:
    printf("sample #%2d: real time before: %lld.%09u\n",
    i, soe.ts[i][0].sec,
    soe.ts[i][0].nsec);
    break;
    case CLOCK_MONOTONIC:
    printf("sample #%2d: monotonic time before: %lld.%09u\n",
    i, soe.ts[i][0].sec,
    soe.ts[i][0].nsec);
    break;
    case CLOCK_MONOTONIC_RAW:
    printf("sample #%2d: monotonic-raw time before: %lld.%09u\n",
    i, soe.ts[i][0].sec,
    soe.ts[i][0].nsec);
    break;
    default:
    break;
    }
    printf("            phc time: %lld.%09u\n",
    soe.ts[i][1].sec, soe.ts[i][1].nsec);
    switch (ext_clockid) {
    case CLOCK_REALTIME:
    printf("            real time after: %lld.%09u\n",
    soe.ts[i][2].sec,
    soe.ts[i][2].nsec);
    break;
    case CLOCK_MONOTONIC:
    printf("            monotonic time after: %lld.%09u\n",
    soe.ts[i][2].sec,
    soe.ts[i][2].nsec);
    break;
    case CLOCK_MONOTONIC_RAW:
    printf("            monotonic-raw time after: %lld.%09u\n",
    soe.ts[i][2].sec,
    soe.ts[i][2].nsec);
    break;
    default:
    break;
    }
    }
    }
    free(soe);
    }
    if (getcross) {
    xts = calloc(1, sizeof(*xts));
    if (!xts) {
    perror("calloc");
    return -1;
    }
    if (ioctl(fd, PTP_SYS_OFFSET_PRECISE, xts)) {
    perror("PTP_SYS_OFFSET_PRECISE");
    } else {
    puts("system and phc crosstimestamping request okay");
    printf("device time: %lld.%09u\n",
    xts.device.sec, xts.device.nsec);
    printf("system time: %lld.%09u\n",
    xts.sys_realtime.sec, xts.sys_realtime.nsec);
    printf("monoraw time: %lld.%09u\n",
    xts.sys_monoraw.sec, xts.sys_monoraw.nsec);
    }
    free(xts);
    }
    if (channel >= 0) {
    if (ioctl(fd, PTP_MASK_CLEAR_ALL)) {
    perror("PTP_MASK_CLEAR_ALL");
    } else if (ioctl(fd, PTP_MASK_EN_SINGLE, (unsigned int *)&channel)) {
    perror("PTP_MASK_EN_SINGLE");
    } else {
    printf("Channel %d exclusively enabled. Check on debugfs.\n", channel);
    printf("Press any key to continue\n.");
    getchar();
    }
    }
    close(fd);
    return 0;
    }
