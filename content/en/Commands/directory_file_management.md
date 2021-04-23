## Directory/File Management
#### Create a new file
**touch** command allows you to create a blank file.
```
touch {filename.extension}
```
Example: For creating file *HTML* named **Web**.
```
touch Web.html
```
#### Create a new folder
we can create folders from the command line using the command **mkdir**.
 
```
mkdir {folder_name}
```
You can create multiple folders at the same time, too.
```
mkdir {foldername1 foldername2 foldername3 ...}
```
#### Delete a file/folder
Deleting a folder or folder is a common operation. Run **rm** for this operation.
```
rm  {file/folder_name}
```
Some files or folders can not be deleted; However, if you still want to delete them, you can use option **-rf**.
 
````
rm -rf {file/folder_name}
````
Another command for delete the empty folder is **rmdir**.
```
rmdir {folder_name}
```
#### Copy a file/folder
A basic example of the copy command to copy files (keep the original file and make a duplicate of it) might look like this:
````
cp {old_file_name} {new_file_name}
````
If you want to copy a folder and all its subfolders, you can use the command below:
```
cp -r {old_dir} {new_dir}
```
#### Move/Rename file/folder
For instance, to instantly seek out and move all of the file/folder. Your command-line instruction would simply be:
```
mv {old_file/folder_path} {new_file/folder_path}
```
Moreover, you can move a file or folder to a new location and rename its name.
```
mv {old_file_name} {new_file_name}
```
#### Search a file/folder
**locate** is a command used to locate files anywhere in the system.
 
```
locate {file_name}
```
 
**find** command can be used for searching a file or folder inside a directory only.
````
find {dir_path} {file_name}
````
#### Zip/Unzip a file/folder
**tar** is a command for archive multiple files into a tarball.
```
tar {folder_name}
```
To compress your files or folders, you can use zip archive:
```
zip {file/folder_name}
```
To extract the zipped file, please use **upzip** command:
```
unzip {zipped_files/folders_name}
```
 
----
----
