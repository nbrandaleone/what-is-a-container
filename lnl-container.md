footer: June 12, 2020
slidenumbers: true

# What is a container?

*Lunch and Learn series* - **remote edition**

Nick Brandaleone
AWS Specialist SA - Containers

--- 
## Ground Rules

- Ask questions
- Try to be on _mute_ when not asking questions
- I will be moving quickly
- The content is on Github.  You can go at your own pace
- I will be using Cloud-9 for a demo

--- 
## Agenda

| Lesson                       | Time | 
-------------------------------| :------: | 
Why are containers so popular? | 5 minutes |
What is the magic? | 5 minutes |
Demo, using BASH | 20 minutes |
Firecracker demo | 5 minutes |

---
# Why so popular?

- Code portability issue is solved
- Faster start-up time makes them preferrable to VMs
- Greater hardware efficiency makes them cheaper
- Isolation provides security (**not perfect though**)
- Docker tooling is **easy to use**

---
# What is the magic?

## Container do not *really* exist

- Namespaces
- cgroups + Linux capabilities
- COW or layered filesystem

**Linux** Kernel tricks - Windows should use .Net Core

---
# Namespaces

*MNT*: It allows a process to have its own filesystem. 
*PID*: The pid namespace gives a process own view of the processes in the system. 
*NET*: Isolated network stack. 
*UTS*: System’s hostname and domain name. 
*USER*: The user namespace maps the uids a process sees to a different set of uids. 
*IPC*: message queues and shared memory.

---
# Cgroups

Where namespaces isolate a process, *cgroups* enforce fair resource sharing between processes.

For example:
- how much memory a process can use
- how many children can be spawned

---
# Layered Filesystem

- Layered Filesystems are how we can efficiently move whole machine images around.
- Also known as *tarballs*...

- Storage Drivers:
    - overlay2
    - aufs (older version)

---
# Vocabulary

## Container = A standardized unit of software

- *images*
A filesystem consisting of all code/libraries/dependencies required for your code to run.  Smaller is better (speed/security), but the image could be an entire Linux distro. Packaged in tarfiles, usually in layers. Stored in repos like DockerHub or ECR.

- *container*
A process running in your custom filesystem

---
## [fit] Demo time! In BASH!!

---
## Thank you

**Stay Safe and Sane**

---
# Resources:

---
# Videos:

-
-
-

---
