Since KOOMPI and Open-source operating systems are closely related to command line. And here are some essential commands that you need to know.
#### pi or pacman command
The **pi** that is **Pacman's shortcut** is one of our system's primary most efficient commands. It is a powerful tool at the center of the system, that allows you to maintain, expand, and update the system.

```
pi [options] 
```

Since you installed the operating system, **pi command** has always been installed, but We can also install **pacman or pi packages** on the system with **pix**. Here is the command:
```
 pix i pi
```

#### pix or pi(extra) command
As you can see that in KOOMPI OS, pi is our primary system's primary command. However, Our team has worked on another influence command that can help users a lot more than pi is Pix. 

Pix is the pacman extra which we consider it as the extra command that help improving in installing the applications and packages that pi can not do.

To install it you can run command belows:
```
curl -s https://repo.koompi.org/script/pix.sh -o pix && chmod +x pix && sudo mv pix /usr/bin
```
***<p style="color:red;"> **Notify**: pix command has been put into system now, you don't have to install it this way anymore.</p>***


#### Updating commands
As I have mentioned above that the system always receives the latest packages, as long as you do the updating command.

The **mandatory step** is to upgrade your operating system before downloading any new applications.
```
pi -Syu
```
**For Database updating:**
```
pi -Syy
```
**For pix command updating:**
```
pix u
```
## Refreshing mirrors
Sometimes it can happen that your system can’t find the updates or packages. In that case, there’s a problem with the mirrors you’re trying to connect to. You can simply refresh the mirrors with this command.
```
 pi -Syyu
```
