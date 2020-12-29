## Applications Management
All installed applications are in the menu list (KOOMPI icon) that is located at the top left of the desktop environment. 

### How to pin applications 

You can pin any application by right-clicking on the particular app and choose from three options: 
- pin onto the desktop by clicking **"Add to Desktop"** 
- pin on the top panel by clicking **"Add to Panel (Widget)"**
- pin to Plank/Dock by running the app and then right clicking on the app's icon that now appears on the dock. You will see an option **"Pin Launcher"**. 

### How to unpin applications 

You can unpin applications by the following methods: 
- For apps that are located on the desktop, simply click on the app's icon and click the delete button on the keyboard 
- For apps on the top panel, right-click on the top panel and choose **"Edit Panel"**. Navigate your cursor onto the app icon and click **"Remove"**. 
- For apps that are on the Plank/Dock, right click on the app's icon and click **"Unpin Launcher"**. 

### How to know more about an application

Simply right-click on the application, click on "Property", and an information box will pop up. Listed in the box are the size, location, date created, modified access, Checksums and usage permission. 

### How to install and remove an application 

Basically, there are two options in order to install and remove your applications from your system. 
- The first option, you can use **Discover**, default installed store on your system.

![Image](/public/Images/Discover1000.png)

-  The second option, you can use **konsole**, a base platform using command-lines to do all the work. 

**Installing  Command**

```
yes | pi -S <application package>
```

**Example**
![Image](/public/Images/InstallingKosole1000.png)

**Removing Command**

```
yes | pi -R <application package>
```

**Example**

![Image](/public/Images/InstallingKosole1000.png)

