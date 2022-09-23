// 第一个 rust程序
#[macro_use]
extern crate chrono;

extern crate lazy_static;
extern crate num_integer;
extern crate num_traits;
extern crate stdext;
extern crate time;
extern crate walkdir;
extern crate winapi;

use chrono::prelude::*;
use chrono::Duration;
use chrono::Local;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use stdext::function_name;
use walkdir::WalkDir;

lazy_static! {
    static ref VEC: Vec<u8> = vec![0x18u8, 0x11u8];
    static ref MAP: HashMap<u32, String> = {
        let mut map = HashMap::new();
        map.insert(18, "hury".to_owned());
        map
    };
}

struct RootRule {
    pub rule_index: i32,      // 规则序号
    pub opperation_type: i32, // 规则执行类型
}

impl RootRule {
    pub fn ruleTip(&self) {
        println!(
            "RootRule  rule_index={} opperation_type={} ",
            self.rule_index, self.opperation_type
        );
    }

    pub fn initParamsWithInputList(&self, mParamVec: Vec<u8>) {
        println!(
            "RootRule  rule_index={} opperation_type={} ",
            self.rule_index, self.opperation_type
        );

        let mut param_index = 0;
        for paramItem in mParamVec {
            println!("param[{}]={}", param_index, paramItem);
            param_index += 1;
        }
    }

    pub fn simple_desc(&self) -> String {
        let rule_index_str = self.rule_index.to_string();
        let result_desc = rule_index_str + &"_simple_desc";
        return result_desc;
    }
}

struct RealRule {
    root_rule: RootRule,

    shellPath: String,
    paramVec: Vec<String>,
    allFileVec: Vec<File>,
    allFileMap: HashMap<String, Vec<File>>,
    curFileVec: Vec<File>,
    curDirFileVec: Vec<File>,
}

impl RealRule {
    pub fn applyOperationRule(
        &self,
        mShellPath: String,
        mParamVec: Vec<String>,
        mAllFileVec: Vec<File>,
        mAllFileMap: HashMap<String, Vec<File>>,
        mCurFileVec: Vec<File>,
        mCurDirFileVec: Vec<File>,
    ) {
        println!(
            "RootRule  rule_index={} opperation_type={} ",
            self.root_rule.rule_index, self.root_rule.opperation_type
        );
    }
}

fn main() {
    show_args_info();
    show_system_info();
    time_parser();

    let zbin_dir_strpath = env::var("USERPROFILE").unwrap() + "/Desktop/zbin/";
    let zbin_temp_txt_file =
        env::var("USERPROFILE").unwrap() + "/Desktop/zbin/" + "I9_Temp_Text.txt";

    let zbin_temp_txt_file_A = zbin_temp_txt_file.clone();
    read_file_line_by_line(&zbin_temp_txt_file_A);

    let file_content_list = read_to_list(&zbin_temp_txt_file_A).unwrap();
    println!("file_content_list.len() = {}", file_content_list.len());

    let mut read_line_index = 0;
    for line_str in file_content_list {
        println!("line[{}]= {}", read_line_index, line_str);
        read_line_index += 1;
    }

    write_file_str(&zbin_temp_txt_file_A, "Hello-世界!");
    append_write_file_str(&zbin_temp_txt_file_A, "追加的数据!");
    show_dir_allsub_file_info(&zbin_dir_strpath);
    show_dir_sub_file_info(&zbin_dir_strpath);

    // 两个 String相加  ,  其中 一个 需要加 地址符 &   ,   一个 String   一个 &str
    let time_stamp_yyyymmdd = getYYYYMMdd();
    let time_stamp_long = getTimeLong64().to_string();
    let dir_path: String =
        String::from("D:/1A/") + &time_stamp_yyyymmdd + "/" + &time_stamp_long + "/";
    create_dir_file(&dir_path);

    let time_temp_file_name: String = String::from("") + &time_stamp_yyyymmdd + ".txt";
    let time_temp_file_path: String = String::from(&zbin_dir_strpath) + &time_temp_file_name;
    copyfile(&zbin_temp_txt_file, &time_temp_file_path); // 文件的复制

    println!("zbin_dir_strpath = {}   ", zbin_dir_strpath);

    println!("zbin_temp_txt_file={} ", zbin_temp_txt_file);

    main_end_info();
}

fn time_parser() {
    println!("════════════ {} begin ════════════ ", function_name!());
    let fmt = "%Y年%m月%d日 %H:%M:%S";
    let now = Local::now().format(fmt);
    println!("{}", now);

    let mut parse = Local
        .datetime_from_str("2022年3月19日 13:30:59", fmt)
        .unwrap();
    println!("{:?}", parse);
    println!(
        "{}-{}-{} {}:{}:{}",
        parse.year(),
        parse.month(),
        parse.day(),
        parse.hour(),
        parse.minute(),
        parse.second()
    );
    println!("{}", parse.date());
    parse = Local.ymd(2012, 12, 12).and_hms(12, 12, 12);
    println!("{}", parse);
    parse = parse + Duration::days(2);
    println!("{}", parse);
    parse = parse + Duration::hours(2);
    println!("{}", parse);
    parse = parse + Duration::minutes(2);
    println!("{}", parse);
    parse = parse + Duration::seconds(2);
    println!("{}", parse);
    println!("════════════ {} end ════════════ ", function_name!());
}

fn show_args_info() {
    println!("════════════ {} begin ════════════ ", function_name!());
    let mut args_index = 0;
    let args = std::env::args();
    for arg in args {
        println!("args_index[{}]{}", args_index, arg);
        args_index += 1;
    }
    println!("════════════ {} end ════════════ ", function_name!());
}

fn show_system_info() {
    println!("════════════ {} begin ════════════ ", function_name!());

    for (k, v) in env::vars() {
        println!("{}: {}", k, v);
    }
    println!("════════════  重要参数 important_system_info begin ════════════ ");

    println!("PATH: {}", env::var("PATH").unwrap());
    println!();
    println!("USERNAME: {}", env::var("USERNAME").unwrap());
    println!();
    println!("OS: {}", env::var("OS").unwrap());
    println!();
    println!("HOMEPATH: {}", env::var("HOMEPATH").unwrap());
    println!();
    println!("USERPROFILE: {}", env::var("USERPROFILE").unwrap());
    println!();
    println!("════════════ {} end ════════════ ", function_name!());
}

// 主函数 结束时 输出的信息
fn main_end_info() {
    println!("════════════ Main 函数运行结束 ════════════");
    let fmt = "%Y年%m月%d日 %H:%M:%S";
    let now = Local::now().format(fmt);
    println!("{}", now);
}

fn append_write_file_str(filePath: &str, mContent: &str) -> std::io::Result<()> {
    println!("════════════ {} begin ════════════ ", function_name!());
    println!("file_path={}", filePath);
    println!("mRaw_Conent={}", mContent);
    println!();

    let path = Path::new(filePath);
    let path_display = path.display();
    let path_display = path.display();
    // 文件是否存在
    let file_exist_flag = path.exists();
    let is_file_flag = path.is_file();
    let is_dir_flag = path.is_dir();

    println!(
        "path={} file_exist_flag={}  is_file_flag={}  is_dir_flag={}   ",
        path_display, file_exist_flag, is_file_flag, is_dir_flag
    );

    let mut target_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filePath)
        .unwrap();

    // 两个字符串的相加  如果 都是 &str 那么必须有一个转为 String::from("xxx")，
    target_file
        .write((String::from("\n") + mContent + "\npath=" + filePath).as_bytes())
        .unwrap();

    println!();
    println!("file_path={}", filePath);
    println!("mRaw_Conent={}", mContent);
    println!("════════════ {} end ════════════ ", function_name!());
    Ok(())
}

fn copyfile(srcFile: &str, dstFile: &str) -> std::io::Result<()> {
    println!("════════════ {} begin ════════════ ", function_name!());
    println!("srcFile={}", srcFile);
    println!("dstFile={}", dstFile);
    println!();

    let copy_length_flag = fs::copy(srcFile, dstFile)?;
    println!("copy_length_flag={}", copy_length_flag);

    println!();
    println!("srcFile={}", srcFile);
    println!("dstFile={}", dstFile);
    println!("════════════ {} end ════════════ ", function_name!());
    Ok(())
}

//

//读取文件以行为元素保存为列表
fn read_to_list(path: &str) -> Option<Vec<String>> {
    println!("════════════ {} begin ════════════ ", function_name!());
    println!("path={}", path);
    println!();
    let model_filename: String = std::fs::read_to_string(path).unwrap();

    let result: Vec<String> = model_filename
        .lines()
        .into_iter()
        .map(move |ch| ch.to_string())
        .collect();
    println!();
    println!("path={}", path);
    println!("════════════ {} end ════════════ ", function_name!());
    Some(result)
}

fn show_dir_sub_file_info(dirFilePath: &str) -> std::io::Result<()> {
    println!("════════════ {} begin ════════════ ", function_name!());
    println!("dirFilePath={}", dirFilePath);
    println!();

    let mut file_index = 0;

    let mut dir_index = 0;

    let mut file_dir_index = 0;

    let path_ReadDir = fs::read_dir(dirFilePath).unwrap();

    for mCurFile in path_ReadDir {
        if mCurFile.as_ref().unwrap().path().is_file() {
            println!(
                "File[{}]={}",
                file_index,
                mCurFile.unwrap().path().display().to_string()
            );
            file_index += 1;
        } else {
            println!(
                "Dir[{}]={}",
                dir_index,
                mCurFile.unwrap().path().display().to_string()
            );
            dir_index += 1;
        }

        file_dir_index += 1;
    }

    println!(
        "file[{}] dir[{}] all[{}]",
        file_index, dir_index, file_dir_index
    );

    println!();
    println!("dirFilePath={}", dirFilePath);
    println!("════════════ {} end ════════════ ", function_name!());
    Ok(())
}

fn show_dir_allsub_file_info(dirFilePath: &str) -> std::io::Result<()> {
    println!("════════════ {} begin ════════════ ", function_name!());
    println!("dirFilePath={}", dirFilePath);
    println!();

    let path = Path::new(dirFilePath);
    //  path.display()   是 绝对路径
    let path_display = path.display();
    // 文件是否存在
    let file_exist_flag = path.exists();
    let is_file_flag = path.is_file();
    let is_dir_flag = path.is_dir();
    println!(
        "path={} file_exist_flag={}  is_file_flag={}  is_dir_flag={}   ",
        path_display, file_exist_flag, is_file_flag, is_dir_flag
    );

    let mut file_index = 0;

    let mut dir_index = 0;

    let mut file_dir_index = 0;

    for e in WalkDir::new(dirFilePath).into_iter().filter_map(|e| e.ok()) {
        if e.metadata().unwrap().is_file() {
            println!("file[{}]={}", file_index, e.path().display());
            file_index += 1;
        }

        if e.metadata().unwrap().is_dir() {
            println!("dir[{}]={}", dir_index, e.path().display());
            dir_index += 1;
        }

        file_dir_index += 1;
    }

    println!(
        "file[{}] dir[{}] all[{}]",
        file_index, dir_index, file_dir_index
    );

    println!();
    println!("dirFilePath={}", dirFilePath);
    println!("════════════ {} end ════════════ ", function_name!());
    Ok(())
}

fn create_dir_file(dirFilePath: &str) -> std::io::Result<()> {
    println!("════════════ {} begin ════════════ ", function_name!());
    println!("dirFilePath={}", dirFilePath);
    println!();

    fs::create_dir_all(dirFilePath)?;

    println!();
    println!("dirFilePath={}", dirFilePath);
    println!("════════════ {} end ════════════ ", function_name!());
    Ok(())
}

fn write_file_str(filePath: &str, mContent: &str) -> std::io::Result<()> {
    println!("════════════ {} begin ════════════ ", function_name!());
    println!("file_path={}", filePath);
    println!("mRaw_Conent={}", mContent);
    println!();

    let path = Path::new(filePath);
    //  path.display()   是 绝对路径
    let path_display = path.display();
    // 文件是否存在
    let file_exist_flag = path.exists();
    let is_file_flag = path.is_file();
    let is_dir_flag = path.is_dir();

    println!(
        "path={} file_exist_flag={}  is_file_flag={}  is_dir_flag={}   ",
        path_display, file_exist_flag, is_file_flag, is_dir_flag
    );

    let mut targetfile = match File::create(&path) {
        Err(why) => panic!("couldn't create path_display={}: why={}", path_display, why),
        Ok(file) => file,
    };

    match targetfile.write_all(mContent.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", path_display, why),
        Ok(_) => println!("successfully wrote 【{}】 to {}", mContent, path_display),
    }

    println!();
    println!("file_path={}", filePath);
    println!("mRaw_Conent={}", mContent);
    println!("════════════ {} end ════════════ ", function_name!());

    Ok(())
}

fn getYYYYMMdd_Chinese() -> String {
    let fmt = "%Y年%m月%d日_%H时%M分%S秒";
    let now = Local::now().format(fmt);
    println!("{}", now);
    return now.to_string();
}

fn getYYYYMMdd() -> String {
    let fmt = "%Y_%m_%d_%H%M%S";
    let now = Local::now().format(fmt);
    println!("{}", now);
    return now.to_string();
}

fn getTimeLong64() -> i64 {
    return Local::now().timestamp();
}

fn read_file_line_by_line(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("════════════ {} begin ════════════ ", function_name!());
    println!("file_path={}", filepath);
    println!("");

    let file = File::open(filepath)?;
    let reader = std::io::BufReader::new(file);
    let mut line_index = 0;
    for line in reader.lines() {
        println!("line[{}]:{}", line_index, line?);
        line_index += 1;
    }

    println!("");
    println!("file_path={}", filepath);
    println!("════════════ {} end ════════════ ", function_name!());
    Ok(())
}

// &str    -> String--| String::from(s) or s.to_string() or s.to_owned()
// &str    -> &[u8]---| s.as_bytes()
// &str    -> Vec<u8>-| s.as_bytes().to_vec() or s.as_bytes().to_owned()
// String  -> &str----| &s if possible* else s.as_str()
// String  -> &[u8]---| s.as_bytes()
// String  -> Vec<u8>-| s.into_bytes()
// &[u8]   -> &str----| s.to_vec() or s.to_owned()
// &[u8]   -> String--| std::str::from_utf8(s).unwrap(), but don't**
// &[u8]   -> Vec<u8>-| String::from_utf8(s).unwrap(), but don't**
// Vec<u8> -> &str----| &s if possible* else s.as_slice()
// Vec<u8> -> String--| std::str::from_utf8(&s).unwrap(), but don't**
// Vec<u8> -> &[u8]---| String::from_utf8(s).unwrap(), but don't**
