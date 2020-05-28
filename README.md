# What is a container lab

pull down code and presentation.

```bash
$ git clone ...
```

# Demo - using BASH
I assume that one is using a Cloud-9 instance.

## 1. Let's create a filesystem to be used by our "container"
I will provide a filesystem based upon frapsoft/fish Fish Docker container, which has been flattened into a single tarball.

``` bash
$ wget bit.ly/fish-container -O fish.tar
$ mkdir container-root; cd container-root
$ tar -xf ../fish.tar
```

Poke around in your new filesystem.  It is based upon busybox, so it is not very large.

I like to create a "YOU-ARE-HERE" file to mark this fs.

## 3. Let's use `chroot` to move us into the filesystem

``` bash
# have two terminals open
# in terminal 1, outside of chroot
$ top
```

We can see the `top` from the other terminal
``` bash
$ sudo chroot rootfs /bin/sh
# mount -t proc proc /proc
# ps aux | grep top
```

Better yet, our new shell is running as root, so...

``` bash
# kill <top process id>
```

So much for containment.

# This why namespaces are important

## 2. Create a new NS, "unsharing" UTS
``` bash
sudo unshare --uts /bin/sh -c '/usr/bin/fish'
# change hostname. Verify it has changed
```

Notice that /proc/[pid]/ns, has links to unique ids.

``` bash
sudo unshare --pid --fork --mount-proc chroot "$PWD" \
/bin/sh -c "/bin/mount -t proc none /proc && \
hostname containers-fun-times && /usr/bin/fish"
```

# Let's investigate cgroups
The kernel exposes cgroups through the /sys/fs/cgroup directory.
However, in Cloud-9 which leverages Amazon Linux 1, the cgroup location is: /cgroup.
Also, cgroups are not enabled by default in AL1.

# Only required in AL1
```bash
# mount
...
```

### Set cgroup limits
``` bash
cgroup_id="cgroup_$(shuf -i 1000-2000 -n 1)"
cgcreate -g "cpu,cpuacct,memory,pids:$cgroup_id"
cgset -r cpu.shares=128 "$cgroup_id"
cgset -r memory.limit_in_bytes="100M" "$cgroup_id"
cgset -r pids.max "$cgroup_id"
```

```bash
# yum -y install libcgroup
# mkdir -p /cgroup/memory/demo

# echo "100M" > /cgroup/memory/demo/memory.limit_in_bytes
# echo "0" > /cgroup/memory/demo/memory.swappiness
```

## Let's create a memory eating program
```bash
# yum install glibc-static -y
$ gcc -static munch.c
$ mv a.out rootfs/
```

Now - let's create our container with the `cgroups`.

```bash
cgexec -g "memory:demo" \
    unshare -fmuipn --mount-proc \
    chroot "$PWD" \
    /bin/sh -c "/bin/mount -t proc proc /proc && hostname container-fun-times && /usr/bin/fish"
```

## Let's stop a fork bomb!

### put the cgroup into place
``` bash
# mkdir -p /cgroup/pids
# mount -t cgroup -o pids none /cgroup/pids
# echo 10 > /cgroup/pids/demo/pids.max
```

### create the bomb
```bash
$ gcc -static fork-bomb.c -o fb
$ mv fb rootfs/
```

# Create our new container
``` bash
$ sudo cgexec -g "pids,memory:demo" unshare -fmuipn --mount-proc \
chroot "$PWD" /bin/sh -c "/bin/mount -t proc proc /proc && \
hostname container-fun-times && /usr/bin/fish"
```

### From host terminal
```bash
$ ps aux | grep forkbomb | wc -l
```

The output should be 98.  Ctr-C the program.

# Move to firecrack demo if there is time.

Clean up the CPU/memory cgroups.
Namespaces will clean themselves up if there are no more processes in them.

```bash
# cgdelete memory,pids:demo
```
