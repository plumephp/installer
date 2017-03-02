use std::collections::HashMap;
use std::path::Path;
use std::env;
use std::process::Command;
use std::process::Stdio;
use std::env::consts::OS;

fn get_args() -> HashMap<String, String> {
    let mut n = 0;
    let mut args = HashMap::new();
    
    for arg in env::args() {
        match n {
            1 => {
                args.insert("new".to_string(), arg);
            },
            2 => {
                args.insert("project".to_string(), arg);
            },
            3 => {
                args.insert("version".to_string(), arg);
            },
            _ => {}
        }
        n = n + 1;
    }

    args
}

fn check_args() -> bool {
    let args = get_args();

    if args.len() < 2 {
        return false;
    }

    true
}

pub fn print_usage() {
    println!("Usage:");
    println!("\tplume new project version\t创建一个plume项目, project: 项目名称，version: plume 版本");
}


pub fn get_current_path() -> String {
    env::current_dir().ok().unwrap().to_str().unwrap().to_string()
}

pub fn project_exists() -> bool {
    
    let args = get_args();
    let current_path = get_current_path();
    let project_name = args.get("project").unwrap();

    let project_path = current_path + &"/" + project_name;

    Path::new(project_path.as_str()).is_dir()
}

pub fn run() {

    if check_args() == false {
        print_usage();
        return;
    }

    let args = get_args();
    let current_path = get_current_path();
    let project_name = args.get("project").unwrap();



    /*
     * 检测是否包含new
     */
    if args.get("new").unwrap() != "new" {
        print_usage();
        return;
    }


    /*
     * 检测版本
     */
    let mut version = "1";
    if let Some(_version) = args.get("version") {
        version = _version;
    }

    if version != "1" && version !="2" {
        version = "1";
    }

    /*
     * 检测项目目录是否已经存在
     * 如果存在，则报错
     */
    if project_exists() {
        println!("目录 {} 已经存在", &project_name);
        return;
    }

    //call_git(current_path.clone(), project_name.clone());
    let git_success = call_git(current_path.clone(), project_name.clone(), version.to_string().clone());

    if git_success {
        call_composer(current_path.clone(), project_name.clone());
    }

}

fn call_git(current_path: String, project: String, version: String) -> bool
{
    
    let mut git_command = Command::new("git");

    let mut branch = "plume1";

    if version == "2" {
        branch = "plume2"
    }

    println!("正在创建......");

    let git = git_command
        .current_dir(&current_path)
        .arg("clone")
        .arg("-b")
        .arg(branch)
        .arg("https://github.com/zhangbaitong/plume-skeleton")
        .arg(&project)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut path = current_path;

    if OS == "windows" {
        path = path + &"\\".to_string() + &project;
    } else {
        path = path + &"/".to_string() + &project;
    }

    if let Ok(mut child) = git.spawn() {
        child.wait().expect("创建项目失败！");

        if Path::new(path.as_str()).is_dir() == false {
            println!("项目创建失败！");
            return false;
        }

        return true;
    } else {
        println!("您的系统没有安装git，或者没有把git加入到PATH !");
        return false;
    }

    
}

fn call_composer(current_path: String, project: String)
{
    let mut composer_str = "composer";

    if OS == "windows" {
        composer_str = "composer.bat";
    }

    let mut path = current_path;

    if OS == "windows" {
        path = path + &"\\".to_string() + &project;
    } else {
        path = path + &"/".to_string() + &project;
    }

    let mut composer_command = Command::new(composer_str);

    println!("正在安装依赖......");

    let composer = composer_command
        .current_dir(&path)
        .arg("install")
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn();

    if let Ok(child) = composer {
        child.wait_with_output().expect("安装依赖失败！");
        println!("安装完成！");
        return;
    } else {
        println!("您的系统没有安装composer，或者没有把composer加入到PATH；在您安装composer后，可以进入项目目录后执行 composer install 安装依赖！");
    }

}