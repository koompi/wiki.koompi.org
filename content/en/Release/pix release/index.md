---
title: "Pix Release"
date: 2018-12-29T11:02:05+06:00
lastmod: 2020-01-05T10:42:26+06:00
draft: false
# search related keywords
keywords: ["induct", "instate"]
---
Pix sounds familiar to pi but it is quite different because it stands for pacman extended or extra which is supported by many more applications which `Pi` can’t do. So, Our team has been trying to create this function in case users want to install some apps which our OS officially support yet.
{{< notice info >}}
After you installed `pix` or you are in `pix version`, you still can use `pi` as **normal**.
{{< /notice >}}


In this document, The details about what `pix` will be provided to all users. First thing first, As pix is just released we have to install it by following the steps below:

Here is the command to install **pix** source:
```
curl -s https://repo.koompi.org/script/pix.sh -o pix && chmod +x pix && sudo mv pix /usr/bin
```
{{< notice note >}}
Please make sure you write it down correctly in Konsole. If you do not know anything about the Konsole [Click here](https://www.koompi.org/details/#konsole-details).
{{< /notice >}}

{{< notice tip >}}
We are recommending you to copy and paste, which makes the command is in correction always.
{{< /notice >}}

If you have already installed the sources of `pix`, you can update it by the command below:
```
pix update 
```

Here is the command that is providing, In case your pi command has been error or unknown, you can get it back with pix command, too:
```
pix -i pi
```

## What pix can do?
Pix is making the impossible possible. You can see all the command by run:
```
pix
```

## Check Available Apps
In pix version, you can check all the Applications that allow to be downloaded with flag `l`(Stand for listing).
```
pix -l
```
Output for current State:

````
NO             APPLICATION           INSTALL                      REMOVE     

1              adobe-acrobat-xi      pix -i adobe-acrobat-xi      pix -r adobe-acrobat-xi     
2              adobe-photoshop-cc    pix -i adobe-photoshop-cc    pix -r adobe-photoshop-cc   
3              asc-timetables        pix -i asc-timetables        pix -r asc-timetables       
4              chinese-support       pix -i chinese-support       pix -r chinese-support      
5              flutter               pix -i flutter               pix -r flutter              
6              ios-support           pix -i ios-support           pix -r ios-support          
7              khmer-typing          pix -i khmer-typing          pix -r khmer-typing         
8              koompi-academy        pix -i koompi-academy        pix -r koompi-academy       
9              koompi-themes         pix -i koompi-themes         pix -r koompi-themes        
10             master-pdf-editor5    pix -i master-pdf-editor5    pix -r master-pdf-editor5   
11             ms-office-2013        pix -i ms-office-2013        pix -r ms-office-2013       
12             pi                    pix -i pi                    pix -r pi  
````
## Update PIX
As you can see it has already been told `how to update` **after you installed PIX Source**, but here again the command:
```
pix u
```
{{< notice note >}}
If your pix had not already up to date, it would have not shown like below.
{{< /notice >}}
```
[ok] No Problems Found.

Dependencies Found:     2
Dependencies Missing:   0

No app updates available.
```
## Install/Remove 
In the version of **PIX**, you can install and remove by put `i` and `r` after `pix`. 


{{% tabs %}}
  {{% tab "Install" %}}
   ```
   pix i <App_Name>
   ```
  {{% /tab %}}

  {{% tab "Remove" %}}
  ```
  pix r <App_NAME>
  ```
  {{% /tab %}}
{{% tabs %}}
Sample for install/remove Microsoft Office 2013: 
```
pix i ms-office-2013 
```
```
pix r ms-office-2013 
```

---
---