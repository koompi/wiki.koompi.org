## Applications Management
All installed applications are in the menu list (KOOMPI icon) that is located at the top left of the desktop environment. 

![Image](/public/Images/Appsmenu1000.png)

### How to pin applications 

You can pin any application by right-clicking on the particular app and choose from three options: 
- pin onto the desktop by clicking **"Add to Desktop"** 
- pin on the top panel by clicking **"Add to Panel (Widget)"**
- pin to [Plank/Dock](/en/KOOMPI_OS/Desktop_environment.md) by running the app and then right clicking on the app's icon that now appears on the dock. You will see an option **"Pin Launcher"**. 

### How to unpin applications 

You can unpin applications by the following methods: 
- For apps that are located on the desktop, simply click on the app's icon and click the delete button on the keyboard. 
- For apps on the top panel, right-click on the top panel and choose **"Edit Panel"**. Navigate your cursor onto the app icon and click **"Remove"**. 
- For apps that are on the Plank/Dock, right click on the app's icon and click **"Unpin Launcher"**. 

### How to know more about an application

Simply right-click on the application, click on "Property", and an information box will pop up. Listed in the box is the size, location, date created, modified access, Checksums and usage permission. 

### How to install and remove an application 

Basically, there are two option in order to install and remove your applications from your system. 
- The first option, you can use **Discover**, default installed store on your system.

![Image](/public/Images/Discover1000.png)

-  The second option, you can use **konsole**, a base platform using command-lines to do all the work. 

**Installing  Command**

```
yes | pi -S application_package
```

**Example**
![Image](/public/Images/InstallingKosole1000.png)

**Removing Command**

```
yes | pi -R application_package
```

**Example**

![Image](/public/Images/InstallingKosole1000.png)

### How to update the application
On the discover, it will notify you that your applications need an update. Some of the  applications can be updated by it **(discover)**. However, we suggest you to update through command line like example has been showed below.

```
yes | pi -Syu application_package
```

**Examples**

**On Discover**

![Image](/public/Images/updatediscover1000.png)

**On Konsole**

![Image](/public/Images/updatekonsole1000.png)

[Visit here for application package's keynames](/en/Applications/Office.md)