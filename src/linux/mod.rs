/// arm-features.h
pub static ARM_FEATURES_H: &str = include_str!("arm-features.h");

// Fix glibc undefined symbol fcntl64 error

// fcntl.map
pub static FCNTL_MAP: &str = r#"
GLIBC_2.2.5 {
    fcntl;
};
"#;

// fnctl.h shim
pub static FCNTL_H: &str = r#"
#ifdef __ASSEMBLER__
.symver fcntl64, fcntl@GLIBC_2.2.5
#else
__asm__(".symver fcntl64, fcntl@GLIBC_2.2.5");
#endif
"#;

pub static MUSL_WEAK_SYMBOLS_MAPPING_SCRIPT: &str = r#"
PROVIDE (__fxstat64 = __fxstat);
PROVIDE (__fxstatat64 = __fxstatat);
PROVIDE (__lxstat64 = __lxstat);
PROVIDE (__xstat64 = __xstat);
PROVIDE (aio_cancel64 = aio_cancel);
PROVIDE (aio_error64 = aio_error);
PROVIDE (aio_fsync64 = aio_fsync);
PROVIDE (aio_read64 = aio_read);
PROVIDE (aio_return64 = aio_return);
PROVIDE (aio_suspend64 = aio_suspend);
PROVIDE (aio_write64 = aio_write);
PROVIDE (aiocb64 = aiocb);
PROVIDE (alphasort64 = alphasort);
PROVIDE (blkcnt64_t = blkcnt_t);
PROVIDE (creat64 = creat);
PROVIDE (dirent64 = dirent);
PROVIDE (F_GETLK64 = F_GETLK);
PROVIDE (F_SETLK64 = F_SETLK);
PROVIDE (F_SETLKW64 = F_SETLKW);
PROVIDE (fallocate64 = fallocate);
PROVIDE (fgetpos64 = fgetpos);
PROVIDE (flock64 = flock);
PROVIDE (fopen64 = fopen);
PROVIDE (freopen64 = freopen);
PROVIDE (fsblkcnt64_t = fsblkcnt_t);
PROVIDE (fseeko64 = fseeko);
PROVIDE (fsetpos64 = fsetpos);
PROVIDE (fsfilcnt64_t = fsfilcnt_t);
PROVIDE (fstat64 = fstat);
PROVIDE (fstatat64 = fstatat);
PROVIDE (fstatfs64 = fstatfs);
PROVIDE (fstatvfs64 = fstatvfs);
PROVIDE (ftello64 = ftello);
PROVIDE (ftruncate64 = ftruncate);
PROVIDE (ftw64 = ftw);
PROVIDE (getdents64 = getdents);
PROVIDE (getrlimit64 = getrlimit);
PROVIDE (glob64 = glob);
PROVIDE (glob64_t = glob_t);
PROVIDE (globfree64 = globfree);
PROVIDE (ino64_t = ino_t);
PROVIDE (lio_listio64 = lio_listio);
PROVIDE (lockf64 = lockf);
PROVIDE (lseek64 = __lseek);
PROVIDE (lseek64 = lseek);
PROVIDE (lstat64 = lstat);
PROVIDE (mkostemp64 = mkostemp);
PROVIDE (mkostemps64 = __mkostemps);
PROVIDE (mkostemps64 = mkostemps);
PROVIDE (mkstemp64 = mkstemp);
PROVIDE (mkstemps64 = mkstemps);
PROVIDE (mmap64 = mmap);
PROVIDE (nftw64 = nftw);
PROVIDE (off64_t = off_t);
PROVIDE (open64 = open);
PROVIDE (openat64 = openat);
PROVIDE (posix_fadvise64 = posix_fadvise);
PROVIDE (posix_fallocate64 = posix_fallocate);
PROVIDE (pread64 = pread);
PROVIDE (preadv64 = preadv);
PROVIDE (prlimit64 = prlimit);
PROVIDE (PT_GETREGS64 = PTRACE_GETREGS64);
PROVIDE (PT_SETREGS64 = PTRACE_SETREGS64);
PROVIDE (pwrite64 = pwrite);
PROVIDE (pwritev64 = pwritev);
PROVIDE (readdir64 = readdir);
PROVIDE (readdir64_r = readdir_r);
PROVIDE (rlimit64 = rlimit);
PROVIDE (scandir64 = scandir);
PROVIDE (sendfile64 = sendfile);
PROVIDE (setrlimit64 = setrlimit);
PROVIDE (stat64 = stat);
PROVIDE (statfs64 = statfs);
PROVIDE (statvfs64 = statvfs);
PROVIDE (SYS_clock_adjtime64 = SYS_clock_adjtime);
PROVIDE (SYS_clock_gettime64 = SYS_clock_gettime);
PROVIDE (SYS_clock_nanosleep_time64 = SYS_clock_nanosleep);
PROVIDE (SYS_clock_settime64 = SYS_clock_settime);
PROVIDE (SYS_futex_time64 = SYS_futex);
PROVIDE (SYS_getrusage_time64 = SYS_getrusage);
PROVIDE (SYS_mq_timedreceive_time64 = SYS_mq_timedreceive);
PROVIDE (SYS_mq_timedsend_time64 = SYS_mq_timedsend);
PROVIDE (SYS_ppoll_time64 = SYS_ppoll);
PROVIDE (SYS_recvmmsg_time64 = SYS_recvmmsg);
PROVIDE (SYS_rt_sigtimedwait_time64 = SYS_rt_sigtimedwait);
PROVIDE (SYS_sched_rr_get_interval_time64 = SYS_sched_rr_get_interval);
PROVIDE (SYS_semtimedop_time64 = SYS_semtimedop);
PROVIDE (SYS_timer_gettime64 = SYS_timer_gettime);
PROVIDE (SYS_timer_settime64 = SYS_timer_settime);
PROVIDE (SYS_timerfd_gettime64 = SYS_timerfd_gettime);
PROVIDE (SYS_timerfd_settime64 = SYS_timerfd_settime);
PROVIDE (SYS_utimensat_time64 = SYS_utimensat);
PROVIDE (tmpfile64 = tmpfile);
PROVIDE (truncate64 = truncate);
PROVIDE (versionsort64 = versionsort);
"#;
