### CALCULATOR

This is a simple calculator project that perform simple arithmetic operations such as
addition, substraction, multiplication, division and some other operations.

### CONTENTS :

An image was built using a docker file which  will be use to run and execute the script
in a docker container(an isolated evironment)

### ABOUT IMG:

 - The image was built using the rust:latest as base image  
 - The docker file was built using multistage for resource and storage management

 - NB:
  For advance storage managemnt, a workflow was implemented using the slim in the  github actions to automate
  the process where one could  simply pull the image and excute the script from the image

 - The workflow automates the process of pushing the image to the ghcr(**github container registry**) so when
   any update is made and pushed to the main-repos the image will also be updated  to date


### How to execute the script :

provided you have docker installed

step1:

simply pull the image using the command bellow

 ``` docker pull ghcr.io/lele-maxwell/my-cal-image:slim ```

step 2:

after pulling the image use the command bellow to see the installed image and size

 docker images

step 3:

after the image is pulled a container is ran using the image where the script will
be exevuted  in the container directly

image name = ghcr.io/lele-maxwell/my-cal-image:slim

 
 ``` docker run -it --name   container-name   hcr.io/lele-maxwell/my-cal-image:slim ```

From here follow the steps promted to do your operation   

