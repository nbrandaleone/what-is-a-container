# Firecracker demo
## https://github.com/weaveworks/ignite/blob/master/docs/usage.md

# Login into the i3.metal instance
ssh -i ~/aws-key.pem ubuntu@ec2-18-223-3-253.us-east-2.compute.amazonaws.com

Turn into root
```bash
$ sudo su
```

## Look at `ignite` tool.
```bash
ignite ps
ignite kernels
ignite images
```

## Start a micro-VM
```bash
ignite run weaveworks/ignite-ubuntu \
  --name my-vm \
  --cpus 2 \
  --memory 1GB \
  --size 6GB \
  --ssh \
  --interactive
```

Login: root/root
# Inside the VM you can check that the kernel version is different, and the IP address came from the container
# Also the memory is limited to what you specify, as well as the vCPUs
> uname -a
> ip addr
> free -m
> cat /proc/cpuinfo

## Stop and Delete the VM
Ignite VMs can be stopped three ways:

By running: # ignite stop my-vm
By running: # ignite kill my-vm
By issuing the reboot command inside the VM

``` bash
# ignite rm my-vm
# ignite rm -f my-vm
```

### Removing other resources
To remove an image, run:

``` bash
# ignite rmi weaveworks/ignite-ubuntu
```

And to remove a kernel, run:

```
# ignite rmk weaveworks/ignite-kernel:4.19.47
```

# 4000 Firecarcker VM demo
## https://github.com/firecracker-microvm/firecracker-demo

Git clone `firecracker-demo` onto host (alread done).
Go into proper directory, and become root.
``` shell
cd firecracker-demo
```

Have 2 terminals open.  In terminal 1:
``` bash
python3 microvm-tiles.py
```

In terminal 2:
``` bash
./parallel-start-many.sh 0 4000 6
```

Each microVM has a workload (iperf client) and will run it in a loop with a random sleep between iterations.

Looking at the heatmap you should see six 'snakes' advancing which are the microVMs that have just been powered up and are doing their first iteration of the workload. Once that's done, the random sleep will lead to random lighting of the heatmap.

## Verify 4000 VM
``` bash
ps | grep firecracker | wc -l
```

Results:
``` bash
# ./parallel-start-many.sh 0 4000 6
Start @ Wed Jun 10 16:54:25 UTC 2020.
Done @ Wed Jun 10 16:55:43 UTC 2020.
Started 4000 microVMs in 77926 milliseconds.
MicroVM mutation rate was 51.94805194805194805194 microVMs per second.
```

That is about 1 VM starting every 20 ms.

## Clean up
``` bash
killall firecracker
```
