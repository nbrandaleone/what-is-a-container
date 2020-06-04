# What is a container? - lab

pull down code and presentation.
```bash
git clone https://github.com/nbrandaleone/what-is-a-container.git
```

# Demo - using BASH

## 1. Let's create a filesystem to be used by our "container"
I will provide a filesystem based upon frapsoft/fish Fish Docker container, which has been flattened into a single tarball.

``` bash
$ wget bit.ly/fish-container -O fish.tar
$ mkdir rootfs; cd rootfs
$ tar -xf ../fish.tar
```

Poke around in your new filesystem.  It is based upon busybox, so it is not very large.

I like to create a "YOU-ARE-HERE" file to mark this fs.

## Show how `lsns` works

``` bash
# have 2 terminals open
# in terminal 1
$ sleep 500 &

# grab PID
# In terminal 2
$ sudo lsns
$ sudo lsns -p <PID>
```

## Let's use `chroot` to move us into the filesystem

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
sudo unshare --uts /bin/sh
# change hostname. Verify it has changed
```

``` bash
sudo unshare --pid --fork --mount-proc chroot "$PWD" \
/bin/sh -c "/bin/mount -t proc none /proc && \
hostname containers-fun-times && /usr/bin/fish"
```

# Let's investigate cgroups
The kernel exposes cgroups through the /sys/fs/cgroup directory.
However, in Cloud-9 which leverages Amazon Linux 1, the cgroup location is: /cgroup.
Also, cgroups are not enabled by default in AL1.

## Install cgroup-tools

``` bash
sudo apt install cgroup-tools
```

### Set cgroup limits
``` bash
cgroup_id="cgroup_$(shuf -i 1000-2000 -n 1)"
sudo cgcreate -g "cpu,cpuacct,memory,pids:$cgroup_id"
sudo cgset -r cpu.shares=128 "$cgroup_id"
sudo cgset -r memory.limit_in_bytes="100M" "$cgroup_id"
sudo cgset -r memory.swappiness=0 ${cgroup_id}
sudo cgset -r pids.max=10 "$cgroup_id"

echo ${$cgroup_id}
cd /sys/fs/cgroup/memory/${cgroup_id}
```

Examine limits

# Another way of setting cgroup limits...
# echo "100M" > /sys/fs/cgroup/demo/memory.limit_in_bytes
# echo "0" > /sys/fs/cgroup/demo/memory.swappiness
```

## Let's create a memory eating program
```bash
# For AL1. yum install glibc-static -y
cd ~/environment/what-is-a-container/src/c
gcc -static -o munch munch.c
mv munch ~/environment/rootfsmunch
```

Now - let's create our container with the `cgroups`.

```bash
sudo cgexec -g "memory:${cgroup_id}" \
    unshare -fmuipn --mount-proc \
    chroot "$PWD" \
    /bin/sh -c "/bin/mount -t proc proc /proc && hostname container-fun-times && /usr/bin/fish"
```

Inside of the *container*, run `munch`
``` bash
# Inside container
$ ./munch
```

## Let's stop a fork bomb!

```bash
$ cd what-is-a-container/src/c
$ gcc -static fork-bomb.c -o fb
$ cp fb ~/environment/rootfs/
```

# Create our new container
``` bash
sudo cgexec -g "pids,memory:${cgroup_id}" unshare -fmuipn --mount-proc \
chroot "$PWD" /bin/sh -c "/bin/mount -t proc proc /proc && \
hostname container-fun-times && /usr/bin/fish"
```

### From host terminal
```bash
$ ps aux | grep fb | wc -l
```

The output should be 8.  Ctr-C the program.

## Now, let's see if we can turn our mini-filesystem
   into an OCI-compatible container

``` bash
# In directory, one above rootfs
runc spec
sudo runc run test

# From other terminal
sudo runc list
sudo runc state test
sudo runc ps test
sudo runc kill test KILL
```
 
Look around from other terminal session
``` basg=h
ps axfo pid,ppid,command | grep runc
pstree <pid>
```

Notice that /proc/[pid]/ns, has links to unique ids.

# Move to firecrack demo if there is time.

Clean up the CPU/memory cgroups.
Namespaces will clean themselves up if there are no more processes in them.

```bash
sudo cgdelete "memory,pids:${cgroup_id}"
```

# Future
```bash
sudo yum install skopeo -y
skopeo copy docker://busybox:latest oci:busybox:latest
runc rootfs
```

