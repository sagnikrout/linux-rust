//! Automatically rewritten from C to Rust
//! Source: tools/laptop/dslm/dslm.c
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
// dslm.c
// Simple Disk Sleep Monitor
// by Bartek Kania
// Licensed under the GPL
//

// Macro flag: #define D(x)

    let mut endit: c_int = 0;
// Check if the disk is in powersave-mode
// Most of the code is stolen from hdparm.
// 1 = active, 0 = standby/sleep, -1 = unknown
#[no_mangle]
unsafe extern "C" fn check_powermode(fd: c_int) -> c_int {
    static int check_powermode(int fd)
    {
    unsigned char args[4] = {WIN_CHECKPOWERMODE1,0,0,0};
    int state;
    if (ioctl(fd, HDIO_DRIVE_CMD, &args)
    && (args[0] = WIN_CHECKPOWERMODE2) /* try again with 0x98 */
    && ioctl(fd, HDIO_DRIVE_CMD, &args)) {
    if (errno != EIO || args[0] != 0 || args[1] != 0) {
    state = -1; /* "unknown"; */
    } else
    state = 0; /* "sleeping"; */
    } else {
    state = (args[2] == 255) ? 1 : 0;
    }
    D(printf(" drive state is:  %d\n", state));
    return state;
    }
    static char *state_name(int i)
    {
    if (i == -1) return "unknown";
    if (i == 0) return "sleeping";
    if (i == 1) return "active";
    return "internal error";
    }
    static char *myctime(time_t time)
    {
    char *ts = ctime(&time);
    ts[strlen(ts) - 1] = 0;
    return ts;
    }
#[no_mangle]
unsafe extern "C" fn measure(fd: c_int) {
    static void measure(int fd)
    {
    time_t start_time;
    int last_state;
    time_t last_time;
    int curr_state;
    let mut curr_time: time_t = 0;
    time_t time_diff;
    let mut active_time: time_t = 0;
    let mut sleep_time: time_t = 0;
    let mut unknown_time: time_t = 0;
    let mut total_time: time_t = 0;
    let mut changes: c_int = 0;
    float tmp;
    printf("Starting measurements\n");
    last_state = check_powermode(fd);
    start_time = last_time = time(0);
    printf("  System is in state %s\n\n", state_name(last_state));
    while(!endit) {
    sleep(1);
    curr_state = check_powermode(fd);
    if (curr_state != last_state || endit) {
    changes++;
    curr_time = time(0);
    time_diff = curr_time - last_time;
    if (last_state == 1) active_time += time_diff;
    else if (last_state == 0) sleep_time += time_diff;
    else unknown_time += time_diff;
    last_state = curr_state;
    last_time = curr_time;
    printf("%s: State-change to %s\n", myctime(curr_time),
    state_name(curr_state));
    }
    }
    changes--; /* Compensate for SIGINT */
    total_time = time(0) - start_time;
    printf("\nTotal running time:  %lus\n", curr_time - start_time);
    printf(" State changed %d times\n", changes);
    tmp = (float)sleep_time / (float)total_time * 100;
    printf(" Time in sleep state:   %lus (%.2f%%)\n", sleep_time, tmp);
    tmp = (float)active_time / (float)total_time * 100;
    printf(" Time in active state:  %lus (%.2f%%)\n", active_time, tmp);
    tmp = (float)unknown_time / (float)total_time * 100;
    printf(" Time in unknown state: %lus (%.2f%%)\n", unknown_time, tmp);
    }
#[no_mangle]
unsafe extern "C" fn ender(s: c_int) {
    static void ender(int s)
    {
    endit = 1;
    }
#[no_mangle]
unsafe extern "C" fn usage() {
    static void usage(void)
    {
    puts("usage: dslm [-w <time>] <disk>");
    exit(0);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    int fd;
    char *disk = 0;
    let mut settle_time: c_int = 60;
// Parse the simple command-line
    if (argc == 2)
    disk = argv[1];
#[no_mangle]
pub unsafe extern "C" fn if(4: argc ==) -> else {
    settle_time = atoi(argv[2]);
    disk = argv[3];
    } else
    usage();
    if (!(fd = open(disk, O_RDONLY|O_NONBLOCK))) {
    printf("Can't open %s, because: %s\n", disk, strerror(errno));
    exit(-1);
    }
    if (settle_time) {
    printf("Waiting %d seconds for the system to settle down to "
    "'normal'\n", settle_time);
    sleep(settle_time);
    } else
    puts("Not waiting for system to settle down");
    signal(SIGINT, ender);
    measure(fd);
    close(fd);
    return 0;
    }
