# What is a container? - lab

> Although they are called containers, it might be more accurate 
> to use the term “containerized processes”. A container is still a 
> Linux process, running on the host machine - it just has a limited 
> view of that host machine, and it only has access to a subtree 
> of the file system and perhaps to a limited set of resources 
> restricted by cgroups. Because it’s really just a process, 
> it exists within the context of the host operating system, 
> and it shares the host’s kernel. 
> -- Liz Rice

Pull down code and presentation. I assume that you are running this on a modern Linux host. I am using AWS `Cloud-9` with Ubuntu.

```bash
git clone https://github.com/nbrandaleone/what-is-a-container.git
```

# Demo - using BASH

## Let's create a filesystem to be used by our "container"
I will provide a filesystem based upon frapsoft/fish Fish Docker container, which has been flattened into a single tarball.

``` bash
$ wget bit.ly/fish-container -O fish.tar
$ mkdir rootfs; cd rootfs
$ tar -xf ../fish.tar
```

Poke around in your new filesystem.  It is based upon busybox, so it is not very large.

I like to create a "YOU-ARE-HERE" file to mark this fs.

``` bash
# While in the new rootfs
$ touch YOU-ARE-HERE
```

## Show how `lsns` works

``` bash
# Have 2 terminals open.
# in terminal 1
$ sleep 500 &

# grab PID
# In terminal 2
$ sudo lsns
$ sudo lsns -p <PID>
```

## Examine the `proc` filesystem directly, for information on namespaces and process attributes.

The `proc` filesystem is virtual, in that it does not exist on disk - only in memory.
``` bash
$ ls -l /proc/self/ns
$ readlink /proc/$$/ns/pid
```

## Let's use `chroot` to move us into the filesystem

``` bash
# In terminal 1, outside of chroot
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

## Create a new NS, "unsharing" UTS
``` bash
sudo unshare --uts /bin/sh
# change hostname. Verify it has changed
```

Now, let's `unshare` the **pid** namespace and `chroot` into our new filesystem. Finally, change the hostname and run the *fish* shell. 
``` bash
sudo unshare --pid --fork --mount-proc chroot "$PWD" \
/bin/sh -c "/bin/mount -t proc none /proc && \
hostname containers-fun-times && /usr/bin/fish"
```

## Joining processes into a combined namespace
A powerful aspect of namespaces is their composability; processes may choose to separate some namespaces but share others. For instance it may be useful for two programs to have isolated PID namespaces, but share a network namespace (e.g. Kubernetes pods). This brings us to the setns syscall and the nsentercommand line tool.

Let's find the shell running in a chroot from our last example.

``` bash
# From the host, not the chroot.
ps aux | grep /bin/sh | grep root
...
root <PID>
```
Grab the PID of the running shell (inside the container). For example, say it is 28840.

``` bash
sudo nsenter --pid=/proc/29840/ns/pid \
    unshare -f --mount-proc=$PWD/rootfs/proc \
    chroot rootfs /bin/sh
```

Having entered the namespace successfully, when we run ps in the second shell (PID 5) we see the first shell (PID 1).

# Let's investigate cgroups
The kernel exposes cgroups through the /sys/fs/cgroup directory.

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

echo ${cgroup_id}
cd /sys/fs/cgroup/memory/${cgroup_id}
```

Examine limits

Another way of setting cgroup limits...
``` bash
# echo "100M" > /sys/fs/cgroup/demo/memory/limit_in_bytes
# echo "0" > /sys/fs/cgroup/demo/memory.swappiness
```

## Let's create a memory eating program
```bash
# For AL1. yum install glibc-static -y
cd ~/environment/what-is-a-container/src/c
gcc -static -o munch munch.c
cp munch ~/environment/rootfs/munch
```

Now - let's create our container with the `cgroups`.
You may have to `export cgroup_id=<NAME>` from one terminal to the other.

```bash
sudo cgexec -g "memory,pids:${cgroup_id}" \
    unshare -fmuipn --mount-proc \
    chroot "$PWD" \
    /bin/sh -c "/bin/mount -t proc proc /proc && hostname container-fun-times && /usr/bin/fish"
```

Inside of the *container*, run `munch`
``` bash
# Inside container
./munch
```

## Let's stop a fork bomb!

```bash
cd ~/environment/what-is-a-container/src/c
gcc -static fork-bomb.c -o fb
cp fb ~/environment/rootfs/
```

### From inside our container
``` bash
./fb
```

### From host terminal
```bash
ps aux | grep fb | wc -l
```

The output should be 8.  Ctr-C the program.

## Now, let's see if we can turn our mini-filesystem into an OCI-compatible container

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


## Clean up the CPU/memory cgroups.
Namespaces will clean themselves up if there are no more processes in them.

```bash
sudo cgdelete "memory,pids:${cgroup_id}"
```

# Move to firecrack demo if there is time.
There is a separate `ignite` directory, and README file.
