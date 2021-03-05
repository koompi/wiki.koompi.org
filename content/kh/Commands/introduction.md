## A Brief Overview 
Most of the operating system, we are using can be communicated throughout graphical interfaces. As the graphical user interface, the command line is the other way to operate the system. Therefore, Linux and open source operating systems interact with the system throughout the command line.

## The Command Line Interface
The command line can be the default interface for a computer, and it uses programs like terminal or Konsole, a baserunning software, to provide a command line interface. 

Below is an example of the terminal on KOOMPI OS:

![Image](/public/Images/username.png)

**[ue-koompi]**: The user’s portion represents the hostname of the computer.

**[ue]**: The hostname portion represents the computer’s name.

**Notes:** Each command will be prepended by a **“$”**, but you must not type it. Your program will do it for you.

## What’s in it for you?
While it may seem difficult at first, being able to use the command line will empower you as a computer user. Once you have learned the basics of using command lines, you will be able to simplify and speed up your work on the system. Moreover, you will gain the following knowledge and skills over the computer:
Communicate and work with your computer via command lines 
Take advantage of a lot of power in their raw, and power form.
Diagnose issues with your own software and others’ software.

You will surely have to use command lines to install software, and programmers to compile and run code and even perform other operation of the system as an administrator.

---
## Primary Commands
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

## Refreshing mirrors command
Sometimes it can happen that your system can’t find the updates or packages. In that case, there’s a problem with the mirrors you’re trying to connect to. You can simply refresh the mirrors with this command.
```
 pi -Syyu
```

---
---