// Nick Brandaleone - June 2020
// Container from scratch, in Rust.
//
// Shamelessly stolen from:
// https://github.com/gs0510/containerust

use nix::sched::{setns, CloneFlags};
use nix::unistd::{chroot, sethostname};
use std::env;
use std::fs::{self, DirBuilder};
use std::os::unix::fs::DirBuilderExt;
use std::path::{Path, PathBuf};
use std::{process, process::Command};
use std::{thread, time};

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn run(string: &str, string1: &str) {
    println!("{}", string);
    println!("running as pid {}", process::id());
    Command::new("/proc/self/exe")
        .arg("child")
        .arg(string1)
        .spawn()
        .expect("blah");

    setns(-1, CloneFlags::CLONE_NEWUTS);
    setns(-1, CloneFlags::CLONE_NEWPID);

    // let mut cf = CloneFlags::empty();
    //sethostname("gargi")?;
}

// fn run1() {
//     let mut stack = [0; 0x200];
//     let p = nix::sched::clone(Box::new(Command::new("/proc/self/exe")
//         .arg("child")
//         .arg("/bin")
//         .spawn()), &mut stack, nix::sched::CloneFlags::CLONE_NEWUTS, Some(0) );
// }

fn cgroups() {
    let mut cgroups = PathBuf::from("/sys/fs/cgroup/");
    assert!(cgroups.exists(), "cgroups must exists");
    cgroups.push("pids");
    assert!(cgroups.exists(), "cgroup/pids must exists");
    cgroups.push("test_group");

    if !cgroups.exists() {
        eprintln!("creating test_group");
        DirBuilder::new()
            .mode(0o777)
            .create(&cgroups)
            .expect("failed to create pids/test_group");
    } else {
        eprintln!("test_group exists?");
    }
    let pid_max = cgroups.join("pids.max");
    fs::write(pid_max, "20".as_bytes());
    eprintln!("wrote test_group/pids.max ??");
}

fn child(string: &str) -> Result<()> {
    cgroups();

    sethostname("gawwrgi")?;
    let root_dir = Path::new("bind_new_root");
    //cleanup_root_dir(root_dir)?;
    // setup_root_dir(root_dir)?;
    do_things(root_dir)?;
    // Ok(_) => cleanup_root_dir(root_dir)?,
    // Err(e) => {
    //     match cleanup_root_dir(root_dir) {
    //         Ok(_) => (),
    //         Err(e_inner) => {
    //             eprintln!("error in cleanup while handling error: {}", e);
    //             return Err(e_inner);
    //         }
    //     };
    //     return Err(e);
    // }
    // };
    Ok(())
}

fn do_things(root_dir: &Path) -> Result<()> {
    chroot(root_dir)?;
    env::set_var("PATH", "/bin");
    env::set_current_dir("/").expect("chdir failed");

    // let new_root = Path::new("/");
    // env::set_current_dir(&new_root).expect("failed to set `new_root`");

    let cur_dir = env::current_dir();
    eprintln!("cur_dir: {:?}", &cur_dir);

    // let output = Command::new("pwd").output().unwrap();
    // eprintln!(
    //     "output succes {}, stdout {}",
    //     output.status.success(),
    //     unsafe { String::from_utf8_unchecked(output.stdout) }
    // );

    for (key, value) in env::vars() {
        eprintln!("{}: {}", key, value);
    }
    Command::new("which")
        .arg("ls")
        .spawn()
        .expect("which failed");
    Command::new("ls")
        .spawn()
        .expect("ls command failed to start");
    Ok(())
}

fn setup_root_dir(root_path: &Path) -> Result<()> {
    assert!(
        !root_path.exists(),
        "setup_root_dir called with existing path"
    );
    if !root_path.exists() {
        fs::create_dir(&root_path)?;
        fs::create_dir(root_path.join("bin"))?;
        fs::copy("/bin/ls", root_path.join("bin").join("ls"))?;
    }
    Ok(())
}

fn cleanup_root_dir(root_dir: &Path) -> Result<()> {
    assert!(root_dir.exists(), "cleanup expects directory to exist");

    fs::remove_dir_all(root_dir)?;
    Ok(())
}

fn main() -> Result<()> {
    let argv: Vec<_> = env::args().collect();
    println!("{}", argv[1]);
    if argv[1] == "run" {
        run(&argv[2], &argv[3]);
    } else if argv[1] == "child" {
        child(&argv[2])?;
    } else {
        panic!("No run specified")
    }
    Ok(())
}
