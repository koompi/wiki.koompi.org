The **id** command writes to standard output a message containing the system identifications (ID) for a specified user. The system IDs are numbers which **identify users** and **user groups** to the system. The **id** command writes the following information, when applicable:

- User name and real user ID
- Name of the user's group and real group ID
- Name of user's supplementary groups and supplementary group IDs


***<p style="color:red;"> **Note:** Supplementary group information is written only for systems supporting multiple-user groups and only if the specified user belongs to a supplementary group.  </p>***



The **id** command also writes effective user and group IDs, but only for the user that invoked the **id** command. **(If the User parameter is specified with the id command, the effective IDs are assumed to be identical to real IDs.)** If the effective and real IDs for the invoking user are different, the id command writes the following effective ID information, when applicable:

- Effective user name and effective user ID
- Name of the effective user's group and effective group ID



***<p style="color:red;"> **Info:** The **id** command will fail if the specified user does not exist or if the command cannot read the user or group information. </p>***


The id command can be altered with the many flags, To check the flags of it run.
```
id --help
```
#### Display current user
To display all system identifications for the current user, enter: 
```
id 
```
Output for the id command is displayed in the following format: 
```
uid=1001(koompi) gid=1001(koompi) groups=1001(koompi),90(network),98(power),985(users),988(storage),993(input),995(audio),998(wheel)
```
#### Display group id
To display all group ID numbers for the current user, enter:
```
id -G
```
Output is displayed in the following format: 
```
1001 90 98 985 988 993 995 998
```
#### Display group name
```
id -Gn
```
Output is displayed in the following format: 
```
koompi network power users storage input audio wheel
```
To display the real group name for the current user, enter: 
```
id -gnr
```
Output from the format:
```
koompi
```
#### Create user
The **useradd** command creates a new user account using the values specified on the command line plus the default value from the system. Depending on command-line options, the useradd command will update system files and may also create the new user at home directory and copy initial files.

```
useradd {USER_NAME}
```

***<p style="color:red;"> **Note:** Similarly, if the username already exists in an external user database, useradd will deny the user account creation request. </p>***


*<p style="color:red;"> **Info:** Usernames may only be up to 32 characters long.</p>*

#### Delete user
The **userdel**  command modifies the system account files, deleting all entries that refer to the user name `LOGIN`.


***<p style="color:red;"> **Note:** The named user must exist.</p>***


```
userdel {EXISTING_USER_NAME}
```
#### Rename user
The **usermod** command modifies the system account files to reflect the changes that are specified on the command line.
```
usermod -l {NEW_USER_NAME} {OLD_USER_NAME}
```

***<p style="color:red;"> **Info:** you must make certain that the named user is not executing any processes when this command is being executed. **usermod** checks only on open-source platform. </p>***


You can add an existing user to the group by
```
usermod -aG {GROUP_NAME} {USER_NAME}
```

#### Grant "sudo" privileges to existing user
Here is the command for setting privilege to a user:
```
sudo usermod -a -G sudo {USER}
```
- **-a** --- append the user to the supplemental GROUPS mentioned by the -G option without removing the user from other groups
- **-G** --- new list of supplementary GROUPS 

#### Showing group
To display entries group from the system:
```
groups
``` 

#### Creating group
The **groupadd** command creates a new group account using the values specified on the command line plus the default values from the system. The new group will be entered into the system files as needed.
```
groupadd [NAME]
```

***<p style="color:red;"> **Note:** Similarly, if the **NAME** already exists in an external user database, groupadd will deny the group account creation request. </p>***

#### Delete group
The **groupdel** command modifies the system account files, deleting all entries that refer to GROUP. 

```
groupdel {EXISTING_NAME}
```

**Note:** **Group Name** must be existed. 


#### Rename group
You can rename the group by groupmod.
```
groupmod -n {NEW_GROUP_NAME} {OLD_GROUP_NAME}
```

- **-n** --- is the flag for changing the name 
 

***<p style="color:red;"> **Note:** Use **--help** behind any commands for getting other options(flags).  </p>***

---
---