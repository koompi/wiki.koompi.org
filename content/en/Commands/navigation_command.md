### Navigation Commands
#### Show current directory
**pwd** is a command uses for find out a path of current directory (folder) you are in. The command will return an absolute full path, which is basically a path of directories that starts with a forward slash **(/)**.

```
pwd
```
An example of an absolute path:
```
/home/username.
```

#### Change direvctory
To navigate through LINUX files and directories, you can use **cd** command. It require either full path or name of the directory, depending  on the current directory you are in.

```
cd directory/diretory/
```

Example:
```
cd /home/username/Documents
```
This command above will lead you to **Documents directory.**

There are some shortcuts to help you navigate quickly:

- **cd ..** (with two dots): to move one directory up
- **cd .** (with one dot):   Back to home directory
- **cd-** (with a hyphen):   to move to your previous directory

## List Folder Contents
Listing Command (ls) allows you to see all the contents in the directory you are in.
```
ls
```
Example:
````
  Desktop     Downloads     Music       Pictures   
  projects     Documents     Videos
````


There are variations you can use with the ls command:

- **ls -R**        list all the files in the sub-directories as well
- **ls -a**        show the hidden files
-  **ls -a**       list all files including hidden file starting with '.'
- **ls --color**   colored list [=always/never/auto]
- **ls -d**        list directories - with ' */'
- **ls -F**        add one char of */=>@ to enteries
- **ls -i**        list file's inode index number`
- **ls -l**        list with long format - show permissions
- **ls -la**       list long format including hidden files
- **ls -lh**       list long format with readable file size
- **ls -ls**       list with long format with file size
- **ls -r**        list in reverse order

---
---