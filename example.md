# Builder

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
  - internal
- source
- build
  - release
  - debug
- assets
  
**Builder Env:**
- configs.json
- lib_table.json
- project_table.json
- 
- builder
  - libs
    - lib_name
      - lib_version
          <br>...files
  - projects
    - project_name
      - projec_version
          <br>... files

**Notas:**
- As configurações de plataforma são acumulativas, assim:
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
  Para plataforma linux x64 as dependências serão: glfw,cglm,vulkan<br>
  Para plataforma windows: glfw,cglm,directx<br>
  Caso haja conflito entre versões das bibliotecas, a última especificada será levada em conta
  
# Commands

  ## new
  **Desciption:** Creates a new peoject

  **Paramters:**
  - *name*: The name of the project
  - *c*, *conf*: Define if project metadata should use default configurations 

  ## open
  **Description:** Open a project in the text editor

  **Paramters:**
  - *name*: The project name
  - *v*, *version*: The project version, if not especified the last version will be open

  ## build
  **Description:** Builds the project

  **Paramters:**
  - *name*: The project name
  - *p*, *platform*: Platform version
  - *a*, *arch*: Archtecture

  ## show
  **Description:** Show the details of an project

  **Paramters:**
  - *name*: The project name
  - *l*, *level*: Complete detailing
  - *v*, *version*: The project version, if not especified all versions will be shaw

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
  - *f*, *from*: Version from update
  - *t*, *to*: Label of new version

  ## list
  **Description:** Show all projects

  ## update
  **Description:** Install a new version of an project

  **Paramters:**
  - *name*: The name of the project 
  



