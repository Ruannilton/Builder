#[derive(Debug,Clone)]
pub struct NewArgs<'a> {
    pub name: &'a str,
    pub conf: bool,
    pub p_type: Option<&'a str>,
}

#[derive(Debug,Clone)]
pub struct OpenArgs<'a> {
    pub name: &'a str,
    pub version: Option<&'a str>,
}

#[derive(Debug,Clone)]
pub struct BuildArgs<'a> {
    pub name: Option<&'a str>,
    pub platform: Option<Vec<&'a str>>,
    pub archtecture: Option<Vec<&'a str>>,
    pub version: Option<&'a str>,
    pub release: bool,
    pub verbose: bool,
}

#[derive(Debug,Clone)]
pub struct ShowArgs<'a> {
    pub name: &'a str,
    pub version: Option<&'a str>,
}

pub struct RmArgs<'a> {
    pub name: &'a str,
    pub recursive: bool,
    pub version: Option<&'a str>,
    pub force: bool,
}

#[derive(Debug,Clone)]
pub struct NvArgs<'a> {
    pub name: &'a str,
    pub u_type: &'a str,
    pub from: Option<&'a str>,
    pub to: Option<&'a str>,
}

#[derive(Debug,Clone)]
pub struct ListArgs<'a> {
    pub ptype: &'a str,
    pub show_versions: bool,
    pub show_deps: bool,
}
