# Builder
### *PROJECT UNDER DEVELOPMENT*
<br>

A tool for package managing and building in C projects

**Features:**
- Install packages from local folder, github or web server
- Generate project dependency tree
- Choose either add pacakge to project as source, pre-compiled or dynamic lib
- Check compatibility across project dependencies and target platforms
- Use custom build scripts


**Project Structure:**<br>
proj_name
- proj.toml
- header
  - proj_name.h
  - *.h
- source
  - *.c
- build
  - release
  - debug
- assets
  
**Builder Env:**
- config.json
- libs
  - lib_name
    - lib_name_1.0.0 
    - lib_name_1.0.1
  - projects
    - project_name
      - project_name_2.0.1
      - project_name_3.0.5
      - ...
      - log.json

**Proj.toml:**
<br>The project config file is in toml format

```
name = 'my_app'
version = '1.0.0'
authors = ['Ruan Azevedo']
proj_type = 'app'
desc = 'An example of how to make builder config files'

[[platform]]
name = 'all'
arch = ["x86","x64"]
```
The file above means an project designed to be an executable.<br>
On platform section we especified that this project can be built for any platform (like windows, mac, linux , etc...) with x86 or x64 archtecture.<br>

The platform configurations are cumulative:
  ```
  [[platform]]
  name="all"
  arch=["x86","x64"]
  [platform.dependencies]
  glfw = "3.12"
  cglm = "1.0"

  [[platform]]
  name="linux"
  arch=["x64"]
  [platform.dependencies]
  vulkan = "1.2"

  [[platform]]
  name="windows"
  arch=["x86","x64"]
  [platform.dependencies]
  directx = "1.0"
  ```
  For linux x64 platform the dependencies will be: glfw, cglm, vulkan<br>
  For windows platform: glfw, cglm, directx<br>
  In case of conflict between versions of the libraries, the last one specified will be taken
  
# Commands

  ## new
  **Desciption:** Creates a new peoject

  **Paramters:**
  - *name*: The name of the project
  - *c*, *conf*: Define if project metadata should use default configurations 
  - *t*, *type*: Define the projects type
  
  ## open
  **Description:** Open a project in the text editor

  **Paramters:**
  - *name*: The project name
  - *v*, *version*: The project version, if not especified the last version will be open

  ## show
  **Description:** Show the details of an project

  **Parameters:**
  - *name*: The project´s name
  - *v*, *version*: The project version, if not especified all versions will be shaw
  
  ## build
  **Description:** Builds the project

  **Paramters:**
  - *name*: The project name
  - *p*, *platform*: Platform version
  - *a*, *arch*: Archtecture

  ## rm 
  **Description:** Delete a project

  **Paramters:**
  - *name*: The name of the project 
  - *r*, *recursive*: Remove its dependencies too
  - *v*, *version*: The project version, if not especified all versions will be removed
  - *f*, *force*: Force deletion

  ## nv
  **Description:** Creates a new version of an project

  **Paramters:**
  - *name*: The name of the project
  - *t*, *type*: Type of update (major, minor, patch)
  - *f*, *from*: Version from update
  - *to*: Label of new version

  ## list
  **Description:** Show projects and libraries

  **Paramters:**
  - *v*, *versions*: Show project versions
  - *d*, *dependencies*: Show project dependencies
  - *t*, *type*: Filter Library(lib) or Project(project) or both(all)

  <!-- ## update
  **Description:** Install a new version of an project

  **Paramters:**
  - *name*: The name of the project  -->
  



