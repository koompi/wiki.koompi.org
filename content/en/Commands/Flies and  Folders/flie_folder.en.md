---
title: "Files/Folders"
date: 2019-11-29T11:02:05+06:00
lastmod: 2020-01-05T10:42:26+06:00
draft: false
# search related keywords
keywords: ["induct", "instate"]
---

This document is a guide for information related to **Files and Folders**. All the Command lines will be provided here with the explanation plus the example.

For more details of the instruction, see [KOOMPI OS website](www.koompi.org) articles or the various [program section](www.koompi.org/applications/), both linked from this guide.

---
---

## Create File
Knowing how to create a new file is an **important skill** for anyone using open-source regularly. You can create a new file either from the `command line` or from the `desktop file manager`.

The `touch` command allows us to update the **timestamps on existing files and directories** as well as creating, empty files. The only easiest and most memorable way to create new, empty files is by using the `touch command`.


To create a new file simply run the touch command followed by the name of the file you want to create:
```
 touch Files_Name.txt
```

You can also create multiple files at once by specifying the file names separated by space.
```
 touch File_Name1.txt File_Name2.txt File_Name3.txt
```

{{< notice info >}}
`.txt` stand for the text file. You can also use other extensions for the kind of files you want to use.
{{< /notice >}}

Besides `Touch`, `echo` can also use to create file too. The `echo` command prints the **strings** that are passed as arguments to the standard output, which can be *redirected* to a file.
```
 echo "some line" >> File_Name.txt
```
For Example:
```
 echo $null >> sample.txt
```
{{< notice note >}}
`$null` here is standing for no. It means there will be no arguments redirected into the file.
{{< /notice >}}
## Deleting File
Deleting File is one of the frequently done operation. Run `rm` command with the name of the file you want to remove:
```
 rm File_Name
```
Here is an example
```
 rm sample.txt 
```

## Display File Content
Cat(concatenate) command is very frequently used in open-source. It reads data from the file and gives their content as output. It helps us to create, view, concatenate files. So let us see some frequently used cat commands.
```
 cat File_Name
```
## Edit Text File
There are many ways to edit files through command line such as vi, emacs, pico, ed, and nano. Among those only Nano is regularly using.
```
 nano File_Name
```
{{< notice note >}}
Sometimes, we need to add `sudo`; which stand for **superuser do**, in front of it to edit.
{{< /notice >}}
```
 sudo nano File_Name
```
## Copy File or Folder
A basic example of the cp command to copy files (keep the original file and make a duplicate of it) might look like:
```
 cp File_Name NewFile_Name
```
If you want to copy other file from one directory to other directory, you can use the command below:
```
cp -R <source_folder> <destination_folder>
```
### Options
`-i`  for interactive, asks you to confirm if an existing file (perhaps a version of joe_expenses already exists in the cashflow directory) should be overwritten in the copying process.

`-r` for recursive, to copy all the subdirectories and files in a given directory and preserve the tree structure.

`-v` for verbose, shows files being copied one by one.

## Move/Rename file or folder
For instance, to instantly seek out and move all of the folder. Your command-line instruction would simply be:
```
 mv Folder_Name NewFolder_Name
```
## Create Folder
we can create directories from command line using the command `mkdir`. Syntax of this command is explained below.
```
 mkdir Folder_Name
```
For example, to create a folder named ‘newfolder‘ the command is:
```
 mkdir NewFolder
```
## Remove Folder
Removing folder is like removing the file, too but it has a little bit different. You have to add `-r` after `rm command`.
```
 rm -r Folder_Name
```
## List Folder Contents
Listing Command (ls) allows us to see all the contents in the directory we are in.
```
 ls
```
### Options
| Options  | Descriptions| 
|:-------------|:-------------|
| **ls -a**|      `list all files including hidden file starting with '.'`|
| **ls --color** |      `colored list [=always/never/auto]`|
| **ls -d** |       `list directories - with ' */'`|
| **ls -F** |       `add one char of */=>@| to enteries`|
| **ls -i**|       `list file's inode index number`|
| **ls -l** |      `list with long format - show permissions`|
| **ls -la** |   `list long format including hidden files` |
| **ls -lh** |   `list long format with readable file size` |
| **ls -ls** |   `list with long format with file size` |
| **ls -r** |   `list in reverse order` |
| **ls -R** |   `list recursively directory tree` |

For more info use `--help` after the command like this `ls --help``.
## Change Folder or Directory
`cd` is the command for changing the folder/directory of the command line. Here its syntax:
```
 cd Folder_Name
```
Changing to home directory (determined by $HOME environment variable):
```
 cd
```
Also change to home directory:
```
 cd .
```
Change to parent directory:
```
 cd ..
```
Change to subdirectory Documents:
```
 cd Documents
```
Change to directory with absolute path
```
 cd path1/path2/path3
```
Change to directory name with white space -*path Name*
```
 cd path\ Name 
```
{{< notice tip >}}
`\` here is stand for white space !!!
{{< /notice >}}
## Show Current Folder
To show the directory you are currently in:
```
 pwd
```
**Output :**
```
 /home/koompi
```
## Create Physical Link to File or Folder
To make links between files you need to use `ln` command. A symbolic link (also known as a soft link or symlink) consists of a special type of file that serves as a reference to another file or directory.
```
ln [FileName] [LinkName]
```
## Find Phrase Within File
The grep command is used to search text. It searches the given file for lines containing a match to the given strings or words. It is one of the most useful commands.

Below is some standard grep command explained with examples to get you started with grep. Search any line that contains the word in filename.
```
 grep 'Strings' FileName
```
Perform a case-insensitive search for the word:
```
 grep -i 'Strings' FileName
```
Looking for all files in the current directory and in all of its subdirectories.
```
 grep -R 'Strings'
```
Searching and displaying the total number of times that the string appears in FileName.
```
 grep -c 'Strings' FileName
```
Searching by paths:
```
 grep 'Strings' path/path/path
```

## Mount Filesystem
You can mount the filesystem with the syntax below:
```
 mount /dev/[device] [path]
```
## Unmount Filesystem
You can also unmount filesystem with command below.
```
 unmount [path]
```
## Make File Executable
Change the rights to a file so that it can run as a program.
```
 chmod +x filename
```
## List Trash Files
As you have already known what is `ls`  command, you should know the path of the trash file.
```
 ls -l files ~/.local/share/Trash/files
```
## Empty Trash
Above are the paths of trash file, we can empty it with `rm -r` command:
```
 rm -r ~/.locale/share/Trash
```

---
---



