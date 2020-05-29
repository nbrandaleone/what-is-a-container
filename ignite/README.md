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
Examine the kernel version

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

# ignite rmi weaveworks/ignite-ubuntu
And to remove a kernel, run:

# ignite rmk weaveworks/ignite-kernel:4.19.47

