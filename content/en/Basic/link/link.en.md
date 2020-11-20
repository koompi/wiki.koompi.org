---
title: "Link/Unlink"
date: 2018-12-29T11:02:05+06:00
lastmod: 2020-01-05T10:42:26+06:00
draft: false
# search related keywords
keywords: ["induct", "instate"]
---
To make links between files you need to use ln command. A symbolic link (also known as a soft link or symlink) consists of a special type of file that serves as a reference to another file or directory. To create a symbolic link, use the `-s` *`(--symbolic)`* option.

There are two types of links in Linux/UNIX systems:

- **Hard links:** You can think of a `hard link` as an additional name for an existing file. Hard links are associating two or more file names with the same **inode**.

- **Soft links:** is something like a shortcut in Windows. It is an indirect pointer to a file or directory. Unlike a hard link, a symbolic link can point to a `file` or a `directory` on a **different** `filesystem` or `partition`.

{{< notice note >}}
Hard links cannot be created for directories and files on a different filesystem or partition.
{{< /notice >}}

---
---
## Syntax
The ln command syntax for creating symbolic links is as follows:
```
ln -s [OPTIONS] FILE LINK
```
## Create Hard Link
That's pretty straightforward - all you have to do is to use the ln command in the following way:
```
ln [file] [hard-link-to-file]
```
Sample:
```
ln test.txt test_hard_link.txt
```
## Create Symbolic Link
Soft links are created with the ln command. For example, the following would create a soft link named link to a file named file, both in the current directory
```
ln -s <File> <Link>
```
To verify new soft link run:
```
ls -l file link
```
Output:
```
-rw-r--r--  1 veryv  wheel  0 Mar  7 22:01 file
lrwxr-xr-x  1 veryv  wheel  5 Mar  7 22:01 link -> file1
```
From the above outputs, it is clear that a symbolic link named ‘link’ contains the name of the file named ‘file’ to which it is linked.

## Overwrite/Remove Existing Destination
You can make ln override this behavior by using the `-f` command-line option.
```
ln -f <File> <Link>
```
You can use the `-i` command-line option if you want to make all this deletion process interactive.
```
ln -i <File> <Link>
```
## Delete Linked
There are some other things that you should consider when you need to delete links or the files to which they point.

{{< notice note >}}
Let's delete every hard link first by command `rm` . Remember that every directory entry that points to an inode is simply a hard link.
{{< /notice >}}

By the way, you can use `unlink` to delete linked, too.
```
unlink file
```
{{< notice info >}}
we recommend you guy to use `unlink` for deleting the link between files.
{{< /notice >}}
{{< notice info >}}
For more option or flags, using --help to show.
{{< /notice >}}

---
---