// Author: Nick Brandaleone
// June 2020

// docker		  run image <cmd> <params>
// go run main.go run       <cmd> <params>

// Inspired by Liz Rice's series of talks "Containers From Scratch"
// https://www.youtube.com/watch?v=Utf-A4rODH8      Container Camp 2016
// https://www.youtube.com/watch?v=8fi7uSYlOdc      GOTO 2018
// https://www.youtube.com/watch?v=_TsSmSu57Zo      Container Camp 2018

// sudo go run what-is-a-container/src/go/container-in-go.go run /usr/bin/fish

package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"os/exec"
	"path/filepath"
	"strconv"
	"syscall"
)

func main() {
	switch os.Args[1] {
	case "run":
		parent()
	case "child":
		child()
	default:
		panic("Arguments should be either run [commands...]")
	}
}

func parent() {
	fmt.Printf("Running %v as %d\n", os.Args[2:], os.Getpid())

	cmd := exec.Command("/proc/self/exe", append([]string{"child"}, os.Args[2:]...)...)
	cmd.Stdin = os.Stdin
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr

  // Create namespaces with clone()
  // Utilize User ns function to map UID/GID to non-root.
  // Allows for non-root to execute run program
	cmd.SysProcAttr = &syscall.SysProcAttr{
		Cloneflags:   syscall.CLONE_NEWUTS | syscall.CLONE_NEWPID | syscall.CLONE_NEWNS,
		Unshareflags: syscall.CLONE_NEWNS,
    Credential: &syscall.Credential{Uid: 0, Gid: 0},
    UidMappings: []syscall.SysProcIDMap{
      {ContainerID: 0, HostID: os.Getuid(), Size: 1},
    },
    GidMappings: []syscall.SysProcIDMap{
      {ContainerID: 0, HostID: os.Getgid(), Size: 1},
    },
	}

	must(cmd.Run())
}

func child() {
	fmt.Printf("Running %v as %d\n", os.Args[2:], os.Getpid())

	cg()

	must(syscall.Sethostname([]byte("container")))
	must(syscall.Chroot("/home/ubuntu/environment/rootfs"))
	must(syscall.Chdir("/"))
	must(syscall.Mount("proc", "proc", "proc", 0, ""))

	//  Syscall PivotRoot is more secure and gives same effect
	//	must(syscall.Mount("rootfs", "rootfs", "", syscall.MS_BIND, ""))
	//	must(os.MkdirAll("rootfs/oldrootfs", 0700))
	//	must(syscall.PivotRoot("rootfs", "rootfs/oldrootfs"))
	//	must(os.Chdir("/"))

	cmd := exec.Command(os.Args[2], os.Args[3:]...)
	cmd.Stdin = os.Stdin
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr

	must(cmd.Run())

	must(syscall.Unmount("/proc", 0))
}

func cg() {
	cgroups := "/sys/fs/cgroup/"
	pids := filepath.Join(cgroups, "pids")
	err := os.Mkdir(filepath.Join(pids, "demogo"), 0755)
	if err != nil && !os.IsExist(err) {
		panic(err)
	}

	must(ioutil.WriteFile(filepath.Join(pids, "demogo/pids.max"), []byte("20"), 0700))
	// Remove the new cgroup in place after the container exists
	must(ioutil.WriteFile(filepath.Join(pids, "demogo/notify_on_release"), []byte("1"), 0700))
	must(ioutil.WriteFile(filepath.Join(pids, "demogo/cgroup.procs"), []byte(strconv.Itoa(os.Getpid())), 0700))
}

// Generic error catch function
func must(err error) {
	if err != nil {
		panic(err)
	}
}
