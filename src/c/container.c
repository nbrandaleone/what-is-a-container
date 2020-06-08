/* Nick Brandaleone - June 2020

   The program takes a command to execute, in a "containerized" environment.
   $ sudo ./main sh

   This container does not set-up networking, but shows how namespaces work.
   The hostname has been set to "container01".
*/

#define _GNU_SOURCE
#include <stdlib.h>
#include <stdio.h>
#include <sched.h>
#include <signal.h>
#include <errno.h>
#include <string.h>
#include <unistd.h>
#include <sys/mount.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <sys/syscall.h>
#include <sys/stat.h>
#include <unistd.h>
#include <time.h>

const char *hostname = "container01";

/* wrapper for pivot root syscall */
int pivot_root(char *a,char *b)
{
	/* This program must executed one directory above rootfs */
	if (mount("rootfs","rootfs","bind",MS_BIND | MS_REC,"")<0){
		printf("error mount: %s\n",strerror(errno));
	}
/* Skipping this section since destination directory already created
 * It might  be better to create check.
	if (mkdir(b,0755) <0){
		printf("error mkdir %s\n",strerror(errno));
	}
 */
	printf("pivot setup ok\n");

	return syscall(SYS_pivot_root,a,b);
}

int child_exec(void *arg)
{
	int err =0;
	char **commands = (char **)arg;
		
	printf("In child...%s\n", commands[0]);
  
  /*
   * It is preferred to pivot_root over chroot,
   * since pivot_root is more secure.
   */
	if (pivot_root("./rootfs","./rootfs/.old")<0){
		printf("error pivot: %s\n",strerror(errno));
	}
	if (mount("proc", "/proc", "proc",0, NULL) <0)
		printf("error proc mount: %s\n",strerror(errno));

	chdir("/");
	if( umount2("/.old",MNT_DETACH)<0)
		printf("error unmount old: %s\n",strerror(errno));

	//set new hostname
	sethostname(hostname,strlen(hostname));	
	
	if (execvp(commands[0], commands) != 0) {
		fprintf(stderr, "failed to execvp arguments %s\n",
				strerror(errno));
		exit(-1);
	}
	return 0;
}


int main(int argc, char *argv[])
{
	int err =0;
	char c_stack[1024*1024];	
	char **args = &argv[1];

	unsigned int flags =  SIGCHLD | CLONE_NEWNS | CLONE_NEWPID | CLONE_NEWUTS;
	printf("Starting...\n");
	pid_t pid = clone(child_exec,c_stack, flags ,args);
	if(pid<0)
		fprintf(stderr, "clone failed %s\n", strerror(errno));

  // lets wait on our child process here before we, the parent, exits
  if (waitpid(pid, NULL, 0) == -1) {
      fprintf(stderr, "failed to wait pid %d\n", pid);
      exit(EXIT_FAILURE);
  }
  exit(EXIT_SUCCESS);
}
