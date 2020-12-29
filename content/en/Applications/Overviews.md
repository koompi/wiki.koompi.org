## Overview
KOOMPI encourages users to use command-lines in **Konsole**, *a baserunning software,* to install, update, and remove because we believe that learning software from the foundation adds more to users’ understanding of technology. On this operating system, pi (pacman) is one of our system’s primary commands. It is a powerful tool at the centre of the system that allows you to maintain, expand, and update the system. There is also another way to get applications onto your computer.
**Discover**, an applications stores for updating, installing, and removing applications. 

***Note:*** *Before, You install any applications, you have to make sure your system is already up to date. If you haven't done it, you must do it.*

***<p style="color:red;">Beware: All operations can only be run on Konsole (command lines).</p>***

##### Updating System
In order to update your system, you have to run this command below in the konsole:
```
pi -Syu
```
##### Installing Application
In order to install the applications in KOOMPI OS, You must know the keyword's names of each apps. And here is the form of the command for installing the applications:
```
pi -S Package_Name
```
**Example:**
To install app **brave:**
```
pi -S brave
```
##### Removing Application
If you wish to remove any applications from your PC, you will need to use this command:
```
 pi -R Package_Name
```
**Example:**
To remove app **brave:**
```
pi -R brave
```
##### Automatic Usage
For automatic command to accept all the requiring requests have been given:
```
yes | Command to install or remove
```
Down here are the examples for you:

**Install Application Example:**
   ```
   yes | pi -S brave
   ```

**Remove Application Example:**
  ```
  yes | pi -Rn brave
  ```

***Info:*** *This auto command can do on both install and remove command.*

***Note:*** *This auto command is working on only some apps.*


