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

use rand::Rng;
use std::thread;
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
use utf8_slice;    //  utf8_slice::slice("holla中国人नमस्ते", 4, 10);   // urf8 方式的切片
use std::env::set_var;
use log;
use tracing::{info, instrument};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

// use env_logger::{Builder, Target};


// 在 所有的 使用到 静态 可变的 变量的地方 都要  使用 unsafe{}    太麻烦
// static  mut  Input_Shell_Path: String = String::new(); 


// 默认的密码 
 const  Encropty_DefaultKey: &str = "zukgit12"; 
 const Cur_Bat_Name: &str  = "rrust_rule_apply_A1";
 
 
lazy_static! {
    static ref VEC: Vec<u8> = vec![0x18u8, 0x11u8];

    static ref MAP: HashMap<u32, String> = {
        let mut map = HashMap::new();
        map.insert(18, "hury".to_owned());
        map
    };
	
	// \rustlib\src\rust\library\alloc\src\string.rs
	static ref Input_Shell_Path_String: String = {
		    let mut Input_Shell_Item: String = String::new(); 
			let mut arg_index = 0 ;
           for arg in std::env::args() {
			   
			   if arg_index == 1{
				   Input_Shell_Item = String::from(arg.as_str());
				   break;
			   }
	
		  arg_index = arg_index + 1;
        }
		Input_Shell_Item
    };
	
	
		static ref Zbin_Path_String: String = {
		    let mut Zbin_Path_String_Item: String = env::var("USERPROFILE").unwrap() + "/Desktop/zbin/";
		   Zbin_Path_String_Item
    };
	
	static ref ZDesktop_Path_String: String = {
		    let mut ZDesktop_Path_String_Item: String = env::var("USERPROFILE").unwrap() + "/Desktop/";
		   ZDesktop_Path_String_Item
    };
	

	
//  类型格式 type=A1_rust_rule::Input_Param_Vec   //   需要转为  to_vec() 
//          InputParam_StingVec_type  =  A1_rust_rule::InputParam_StingVec 
// InputParam_StingVec.to_vec()_type  =  alloc::vec::Vec<alloc::string::String>

//  \rustlib\src\rust\library\alloc\src\vec\mod.rs
pub static ref InputParam_StingVec: Vec<String> ={
        let mut param_vec: Vec<String> = Vec::new();
		    let args = std::env::args();
			let mut arg_index = 0 ;
         for arg in args {
		// C:\Users\zhuzj5\Desktop  type=alloc::string::String
		
		param_vec.push(arg);

		arg_index = arg_index + 1;
        }
		param_vec
    };

}


fn get_var_type<T>( _ : &T   ) -> &str {    
 std::any::type_name::<T>()
}


fn get_var_size<T>( _ : &T   ) -> usize {    
 std::mem::size_of::<T>()
}

fn get_thread_info( ) -> String {    
 let  thread_info =  format!("{:?}", thread::current());
 return thread_info ;
}

#[derive(Debug)]
enum OS_TYPE {
    Windows,
    Linux,
    MacOS,
}


fn getSystem_OS_EnumType() -> OS_TYPE {  // 
	
	let mut os_type_enum  = OS_TYPE::Windows;
	// \rustlib\src\rust\library\core\src\str\mod.rs
	let mut os_name: String = env::var("OS").unwrap();
	
	os_name.make_ascii_lowercase();  // 返回 空   对 自身 进行 修改 
	 if !os_name.contains("window") {
		 if os_name.contains("mac") {
			 os_type_enum = OS_TYPE::MacOS;
		 } else {
			os_type_enum = OS_TYPE::Linux;
		 }
	 }
	os_type_enum
}



pub trait Draw {
    fn simple_desc(&self) -> String; // 使用的简单描述 中文的该 rule的使用情况 默认会在 ruleTip 被调用

	
    fn init_with_input_list_params(&self,paramList:Vec<&str>) -> bool;  // 初始化输入参数
	
    fn apply_all_file_rule_operation(&self
	,shell_file_list:Vec<&std::fs::File>            
	,all_real_file_list:Vec<&std::fs::File>
	,all_dir_file_list:Vec<&std::fs::File> 
	,real_file_type_map:HashMap<String, Vec<&std::fs::File>> )   -> bool   ;   // 实际的规则应用 
  
		
		
}




fn show_args_info(param_vec: Vec<String> ){
	println!("════════════ {} begin ════════════ ", function_name!());
	let mut param_index = 0 ;
	let param_len = param_vec.len();
	    println!("param_vec_type={}", get_var_type(&param_vec));
	    for param in param_vec {
        println!("param[{}][{}]___{}  type={}", param_index ,param_len ,  param,get_var_type(&param));
        param_index = param_index + 1;
    }

}



fn show_vars_info( ){
	println!("════════════ {} begin ════════════ ", function_name!());

	println!("Input_Shell_Path_String={} ", Input_Shell_Path_String.as_str());
	println!("Zbin_Path_String={} ", Zbin_Path_String.as_str());
	println!("ZDesktop_Path_String={} ", ZDesktop_Path_String.as_str());
	println!("getSystem_Batch_EndType()={} ", getSystem_Batch_EndType());
	println!("getSystem_OS_EnumType()={:?} ", getSystem_OS_EnumType());
		

}

fn getSystem_Batch_EndType() -> String {
	
	let mut batch_name : &str = ".bat"; 
	// \rustlib\src\rust\library\core\src\str\mod.rs
	let mut os_name: String = env::var("OS").unwrap();
	
	os_name.make_ascii_lowercase();  // 返回 空   对 自身 进行 修改 
	 if !os_name.contains("window") {
		 batch_name = ".sh";
	 }
	 let batname_string = String::from(batch_name);
	batname_string
}






fn main() {
		// 只有注册 subscriber 后， 才能在控制台上看到日志输出
    tracing_subscriber::registry().with(fmt::layer()).init();
	show_args_info(InputParam_StingVec.to_vec());
	show_vars_info();

  //  show_system_info();
}

/*
fn main2() {
	    
	// set_var("RUST_LOG", "debug");    // env_logger 的 使用 
	// env_logger::init();// 注意，env_logger 必须尽可能早的初始化
	// info!("main_end_info  log test");
	// debug!("this is a debug {}", "message");
	// error!("this is printed by default");
	
	
	
	// 只有注册 subscriber 后， 才能在控制台上看到日志输出
    tracing_subscriber::registry().with(fmt::layer()).init();
	

    show_add_args_info(&Input_Param_Vec);
    show_system_info();
    time_parser();
	
	let str_test_1 = "hello";
	let str_test_2 = "rust";


    let str_test_3 = format!("{}❥{}!", str_test_1, str_test_2);
	
	println!("str_test_3 = {}",str_test_3);
	
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
	
	info!("Hello from tracing  main_end_info ");

let int_max = 100;
let int_min = 1;
 let random_int_A = get_random_int(int_min,int_max);
 let random_int_B = get_random_int(int_min,int_max);
// let random_int = rand::thread_rng().gen_range(1,101);

    println!("A random_int_A = {}   random_int_B={}", random_int_A ,random_int_B);
}

*/

// 产生 指定范围的 随机数 
 fn get_random_int(min: i32 , max: i32 ) -> i32 {
 	let top : i32 =  max + 1;
   let random_number : i32 = rand::thread_rng().gen_range(min,top)  ;
   return random_number;
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

#[instrument]
fn main_end_info() {
    println!("════════════ Main 函数运行结束 ════════════");
    let fmt = "%Y年%m月%d日 %H:%M:%S";
    let now = Local::now().format(fmt);
    println!("{}", now);
	info!("in main_end_info func");
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
