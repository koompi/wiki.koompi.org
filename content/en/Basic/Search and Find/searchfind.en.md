---
title: "Searching/History"
date: 2018-12-29T11:02:05+06:00
lastmod: 2020-01-05T10:42:26+06:00
draft: false
# search related keywords
keywords: ["induct", "instate"]
---
In this tutorial, we'll look at the `commands` and how you can quickly use it to **locate** files in your filesystem. Finding files is a powerful utility capable of locating files anywhere on your system including mounted drives and removable storage, processing regular expressions, and even running other commands on those files. Fortunately, only a few simple options are needed to provide most users with all the capabilities they need.

----
----

The `find` command is a command-line utility for walking a file hierarchy. It can be used to find files and directories and perform subsequent operations on them. It supports searching by file, folder, name, creation date, modification date, owner and permissions. 



### Find File By Name
As with most open-source commands, you have several available options. And we are attempting to find a file by name, we’ll use one of two options:

+ **name** – case sensitive
+ **iname** – case insensitive

Remember, Open-source is very particular about the case, so if you’re looking for a file named .txt, the following command will return no results.
```
sudo find / -name filename.extension
```
Sample:
```
sudo find / -name text.txt
```
If, however, you were to alter the command by using the -iname option, the find command would locate your file, regardless of case. So the new command looks like:
```
sudo find / -iname filename.extension
```
Sample:
```
sudo find / -iname text.txt
```

### Find By Type
What if you’re not so concerned with locating a file by name but would rather locate all files of a certain type? Some of the more common file descriptors are:

|    Options| Description         | 
|:----------:|:--------------------|
| **f**     |      `Regular File` |
| **d**     |      `Directory`    |
| **l**     |      `Symbolic Link`|
| **c**     |      `Character Devices`|
| **b**     |      `Block Devices`|

Now, suppose you want to locate all `block devices` (a file that refers to a device) on your system. With the help of the `-type` option, we can do that like so:
```
sudo find / -type b
```

We can use the same option to help us look for configuration files. Say, for instance, you want to locate all regular files that end in the `.conf` extension. This command would look something like:
````
sudo find / -type  f -name "*.conf"
````
### Find Modified Files Since Last 60 minutes
Do you know you can modify your files by for the last 60 minutes or any minutes.
```
find / -mmin -60
```
### Find Change Files Since Last 60 minutes
You can also find the changed files that you have been changed for any minutes, too.
```
find / -cmin -60
```
### Find All Files Which Are Accessed 7 days
Besides finding all the files that have been changed by minutes you can look for the files has been accessed for days.
```
find / -atim 7
```
{{< notice info >}}
All the numeric number here, you can put with any other number. 
{{< /notice >}}

### Finding Files By Size
We can use the find command to locate files of a certain size. Say, for instance, you want to go large and locate files that are over **1000MB**. The find command can be issued, with the help of the `-size` option, like so:
```
find / -size +1000M
```
You can also find all files which are greater than or less than, too. For example, `( Greater than 10MB Less than 100MB )`
```
find / -size +10M -size -100M
```
With the output from the command, you can comb through the directory structure and free up space or troubleshoot to find out what is mysteriously filling up your drive.

You can search with the following size descriptions:

|    Options| Description         | 
|:----------:|:--------------------|
| **c**     |      `Bytes` |
| **k**     |      `Kilobytes`    |
| **M**     |      `Megabytes`|
| **G**     |      `Gigabytes`|
| **b**     |      `512-byte blocks`|

----

The `which` command locates the executable file associated with a given command. It returns the pathnames of the files (or links) which would be executed in the current environment, had the filename (or filenames) been given as a command (or commands) in a strictly POSIX-conformant shell. It does this by searching the paths in the PATH environment variable for executable files matching the names of the arguments.


The syntax for `which` command is as follows:
```
which [OPTIONS] FILE_NAME
```
### Locate File With Which
For example, to find the full path of the **ping** command, you would type the following:
```
which ping
```
You can also provide more than one arguments to the which command:
```
which ping uptime
```
The output will include full paths to both ping and uptime executables:
```
/usr/bin/ping
/usr/bin/uptime
```
The search is done from left to right, and if more than one matches are found in the directories listed in the `PATH` path variable, which will print only the first one. To print all matches, use the `-a` option:
```
which -a touch
```
{{< notice info >}}
Usually one of the executables is only a symlink to the other one, but in some cases, you may have two versions of the same command installed in different locations or different commands using the same name.
{{< /notice >}}

{{< notice note >}}
The `which` command is used to locate a command by searching the command executable in the directories specified by the environmental variable `PATH`.
{{< /notice >}}

----
### Showing History Of Used Commands
Since all of our services are currently running on Open-Source. There is a very useful command to show you all of the last commands that have been recently used. The command is simply called `history`, but can also be accessed by looking at your `.bash_history` in your home folder. 

```
history
```
{{< notice note >}}
By default, the history command will show you the last five hundred commands you have entered.
{{< /notice >}}

### Showing History Of Used Command Less
If you wish to view the history one page at a time, you can use the command below.
```
history | less
```
To view just the last ten commands, you can use the following:
```
history | tail
```
To view the last 25 commands, just use the following:
```
history 25
```
Another way to search history is with the following command. This command will show us all the history of commands has used on the object.
```
history | grep <OPTION> <FILE_NAME>
```
For example: I want to see how many commands have been used on this file `text.txt`
```
history | grep text.txt
```
Here is the output all the commands have been used on it.
```
  394  touch text.txt
  397  find / -iname text.txt
  398  sudo find / -iname text.txt
  408  locate text.txt
  431  history | grep text.txt
```

----