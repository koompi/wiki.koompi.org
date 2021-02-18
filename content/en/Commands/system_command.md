This section will only cover the commands related to **system**.

#### SuperUser Do command
SuperUser Do, enables to perform tasks that require administrative or root permissions.
```
sudo {command}
```
You can ***run multiple commands*** at the same time using this technique.
```
command1; command2; command3 
```
You can also run a command after a command, too.
```
command1 && command2 && command3
```

#### Create user command
Since most open-sources is a multi-user system, this means more than one person can interact with the same system at the same time. useradd is used to create a new user, while **passwd** is adding a password to that user’s account. To add a user:
```
useradd {username}
```

#### Set password command
You type the following command to set your own password:
```
passwd {password}
```
output:
```
[koompi@koompi-pc ~]$ passwd
Changing password for koompi.
Current password: 
New password: 
Retype new password: 
passwd: password updated successfully
```

The user is first prompted for his/her old password if one is present. This password is then encrypted and compared against the stored password. The user has only one chance to enter the **correct password**.

**The superuser** is permitted to bypass this step so that forgotten passwords may be changed. **A new password** is tested for complexity. As a general guideline, passwords should consist of 10 to 20 characters including one or more** from each of the following sets:

1.  **Lower case alphabetic**
1.  **Upper case alphabetic**
1.  **Digits 0 to 9**
1.  **Punctuation marks/special characters**


For more details about commands password, [Click here](/en/Commands/password.md).


#### Delete user command
You can delete the [user](/en/Commands/users_groups.md) account type:
```
userdel {existing_username}
```

#### Set permission command (files/directories)
chmod command is another command, used to change the read, write, and execute permissions of files and directories. As this command is rather complicated, you can read the full tutorial in order to execute it properly.

```
chmod {options} {filename}
```
**options**:
- ***u***: user
- ***g***: group
- ***o***: other
- ***r***: read
- ***w***: write
- ***x***: execute

#### Transfer ownership command (files)
All files are owned by specifie user. The **chown** command enables you to change  or transfer the ownership of  a file to the specified username.

```
chown {username} {file_name}
```
#### Print information command (system)
The uname command, short for Unix Name, will print detailed information about your system.
```
uname
```
#### hostname command (Network)
If you want to know the name of your host/network simply type **hostname**. Adding a *-i* to the end will display the IP address of your network.
```
hostname -i
```

#### Download command (files)
You can even download files from the internet with the help of the wget command. To do so, simply type wget followed by the download link.
```
wget {link}
```
#### Display information command (Processes)
***ps command*** displays information about the currently running processes, including their process identification numbers (PIDs).
```
ps
```
Example:
```
      PID TTY          TIME CMD
  28076 pts/1    00:00:00 bash
  28089 pts/1    00:00:00 ps

```
#### Terminate manually command (Applications)
If you have an unresponsive program, you can terminate it manually by using the **kill command**. It will send a certain signal to the misbehaving app and instructs the app to terminate itself.

There is a total of ***sixty-four*** signals that you can use, but people usually only use **two signals**:

- **SIGTERM** — requests a program to stop running and gives it some time to save all of its progress. If you don’t specify the signal when entering the kill command, this signal will be used.
- **SIGKILL** — forces programs to stop immediately. Unsaved progress will be lost.

Besides knowing the signals, you also need to know the **process identification number (PID)** of the program you want to kill. If you don’t know the PID, simply run the command **ps** ux.

After knowing what signal you want to use and the PID of the program, enter the following syntax:
```
kill {pid}
```

#### Print information command (disk)

Use **df** command to get a report on the system’s disk space usage, shown in percentage and KBs. 
```
df
```
If you want to display disk space statistic
```
df -h
```
If you want to see the report in megabytes, type df -m.
```
df -m 
```

#### du command (disk)
Use **du** command to display disk usage.
````
du -h
````
#### free command (disk)
**free** is command for display free memory.
````
free -m
````
#### history command (terminal)
When you’ve been using open source for a certain period of time, you’ll quickly notice that you can run hundreds of commands every day. As such, running **history** command is particularly useful if you want to review the commands you’ve entered before.

```
history
```

#### clear command (terminal)
Use the **clear** command to clean out the terminal if it is getting cluttered with too many past commands.
````
clear
````

---
---